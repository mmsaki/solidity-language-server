use crate::links;
use crate::utils;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tower_lsp::lsp_types::{Position, Range, TextEdit, Url};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A single file rename: old absolute path → new absolute path.
#[derive(Debug, Clone)]
pub struct FileRename {
    pub old_path: PathBuf,
    pub new_path: PathBuf,
}

/// Diagnostic counters returned alongside edits so the caller can log them.
#[derive(Debug, Default)]
pub struct RenameStats {
    /// Source files whose bytes could not be read.
    pub read_failures: usize,
    /// Source files with no parseable parent directory.
    pub no_parent: usize,
    /// Imports skipped because pathdiff returned None.
    pub pathdiff_failures: usize,
    /// Imports whose new path equals the old path (no-op, correctly skipped).
    pub no_op_skips: usize,
    /// Edits suppressed because Case 1 already covered the range.
    pub dedup_skips: usize,
    /// Duplicate old_path entries detected in the rename list (last wins).
    pub duplicate_renames: usize,
}

/// Result of a rename_imports call: edits + diagnostic stats.
pub struct RenameResult {
    pub edits: HashMap<Url, Vec<TextEdit>>,
    pub stats: RenameStats,
}

// ---------------------------------------------------------------------------
// Folder expansion
// ---------------------------------------------------------------------------

/// Expand rename entries that target folders into per-file renames.
///
/// For each entry, if `old_path` is a directory (or has no `.sol` extension),
/// every source file under it is expanded to a concrete file rename.
/// Uses `Path::strip_prefix` for component-aware matching (won't match
/// `/src2` given `/src`).
pub fn expand_folder_renames(
    params: &[(PathBuf, PathBuf)],
    source_files: &[String],
) -> Vec<FileRename> {
    // Deduplicate by old path; last entry wins.
    let mut dedup: HashMap<PathBuf, PathBuf> = HashMap::new();
    for (old_path, new_path) in params {
        if old_path.is_dir() || !old_path.extension().map_or(false, |e| e == "sol") {
            for sf in source_files {
                let sf_path = Path::new(sf);
                if let Ok(suffix) = sf_path.strip_prefix(old_path) {
                    dedup.insert(sf_path.to_path_buf(), new_path.join(suffix));
                }
            }
        } else {
            dedup.insert(old_path.clone(), new_path.clone());
        }
    }
    dedup
        .into_iter()
        .map(|(old_path, new_path)| FileRename { old_path, new_path })
        .collect()
}

/// Expand folder renames using candidate filesystem paths.
///
/// `candidate_paths` should be the union of discovered project files and files
/// currently present in `text_cache` so folder renames don't miss entries.
pub fn expand_folder_renames_from_paths(
    params: &[(Url, Url)],
    candidate_paths: &[PathBuf],
) -> Vec<(String, String)> {
    // Deduplicate by old URI; last entry wins.
    let mut dedup: HashMap<String, String> = HashMap::new();
    for (old_uri, new_uri) in params {
        let old_path = match old_uri.to_file_path() {
            Ok(p) => p,
            Err(_) => continue,
        };
        let new_path = match new_uri.to_file_path() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if old_path.extension().map_or(false, |e| e == "sol") && !old_path.is_dir() {
            dedup.insert(old_uri.to_string(), new_uri.to_string());
        } else {
            for existing_path in candidate_paths {
                if let Ok(suffix) = existing_path.strip_prefix(&old_path) {
                    let new_file_path = new_path.join(suffix);
                    let Ok(existing_uri) = Url::from_file_path(existing_path) else {
                        continue;
                    };
                    let Ok(new_file_uri) = Url::from_file_path(&new_file_path) else {
                        continue;
                    };
                    dedup.insert(existing_uri.to_string(), new_file_uri.to_string());
                }
            }
        }
    }
    dedup.into_iter().collect()
}

// ---------------------------------------------------------------------------
// Core rename logic
// ---------------------------------------------------------------------------

/// Compute import-path edits needed when one or more files are renamed/moved.
///
/// Handles batch renames correctly: when both an importer and its import target
/// are being moved in the same request (e.g. folder rename), the relative path
/// between them is computed using their **new** locations, so imports that are
/// still valid after the move are left unchanged.
///
/// Uses tree-sitter to find import strings in each source file, making this
/// robust against stale or unavailable solc AST data.
///
/// Handles two cases per renamed file:
/// 1. **Other files import the renamed file** — their import path string must
///    change to reflect the new location of the target.
/// 2. **The renamed file's own relative imports** — if the file moved to a
///    different directory, its relative imports to other files need updating.
pub fn rename_imports(
    source_files: &[String],
    renames: &[FileRename],
    project_root: &Path,
    get_source_bytes: &dyn Fn(&str) -> Option<Vec<u8>>,
) -> RenameResult {
    let mut edits: HashMap<Url, Vec<TextEdit>> = HashMap::new();
    let mut stats = RenameStats::default();

    if renames.is_empty() {
        return RenameResult { edits, stats };
    }

    // Build lookup: old_path → new_path.
    // Detect duplicates (same old_path appears more than once).
    let mut rename_map: HashMap<PathBuf, PathBuf> = HashMap::with_capacity(renames.len());
    for r in renames {
        if rename_map
            .insert(r.old_path.clone(), r.new_path.clone())
            .is_some()
        {
            stats.duplicate_renames += 1;
        }
    }

    // ── Case 1: files that import a renamed file ───────────────────────
    for source_fs_str in source_files {
        let source_path = Path::new(source_fs_str);

        // If this source file is itself being renamed, use its NEW directory
        // for computing replacement import paths.
        let source_new_path = rename_map.get(source_path);

        let effective_source_dir = match source_new_path {
            Some(new_p) => match new_p.parent() {
                Some(d) => d.to_path_buf(),
                None => {
                    stats.no_parent += 1;
                    continue;
                }
            },
            None => match source_path.parent() {
                Some(d) => d.to_path_buf(),
                None => {
                    stats.no_parent += 1;
                    continue;
                }
            },
        };

        let current_source_dir = match source_path.parent() {
            Some(d) => d,
            None => {
                stats.no_parent += 1;
                continue;
            }
        };

        let bytes = match get_source_bytes(source_fs_str) {
            Some(b) => b,
            None => {
                stats.read_failures += 1;
                continue;
            }
        };

        let imports = links::ts_find_imports(&bytes);

        for imp in &imports {
            let resolved = normalize_path(&current_source_dir.join(&imp.path));

            let resolved_target = if rename_map.contains_key(&resolved) {
                Some(resolved)
            } else if !imp.path.starts_with('.') {
                let via_root = normalize_path(&project_root.join(&imp.path));
                if rename_map.contains_key(&via_root) {
                    Some(via_root)
                } else {
                    None
                }
            } else {
                None
            };

            let old_target = match resolved_target {
                Some(t) => t,
                None => continue,
            };

            let new_target = &rename_map[&old_target];

            let new_import_path = if imp.path.starts_with('.') {
                match pathdiff::diff_paths(new_target, &effective_source_dir) {
                    Some(p) => ensure_dot_prefix(&p),
                    None => {
                        stats.pathdiff_failures += 1;
                        continue;
                    }
                }
            } else {
                match pathdiff::diff_paths(new_target, project_root) {
                    Some(p) => normalize_slashes(&p.to_string_lossy()),
                    None => {
                        stats.pathdiff_failures += 1;
                        continue;
                    }
                }
            };

            if new_import_path == imp.path {
                stats.no_op_skips += 1;
                continue;
            }

            let source_uri = match Url::from_file_path(source_fs_str) {
                Ok(u) => u,
                Err(_) => continue,
            };

            edits.entry(source_uri).or_default().push(TextEdit {
                range: range_with_quotes(imp.inner_range),
                new_text: format!("\"{}\"", new_import_path),
            });
        }
    }

    // ── Case 2: renamed files' own relative imports ─────────────────────
    for rename in renames {
        let old_dir = match rename.old_path.parent() {
            Some(d) => d,
            None => {
                stats.no_parent += 1;
                continue;
            }
        };
        let new_dir = match rename.new_path.parent() {
            Some(d) => d,
            None => {
                stats.no_parent += 1;
                continue;
            }
        };

        if old_dir == new_dir {
            continue;
        }

        let old_fs_str = match rename.old_path.to_str() {
            Some(s) => s,
            None => continue,
        };

        let bytes = match get_source_bytes(old_fs_str) {
            Some(b) => b,
            None => {
                stats.read_failures += 1;
                continue;
            }
        };

        let imports = links::ts_find_imports(&bytes);

        let old_uri = match Url::from_file_path(&rename.old_path) {
            Ok(u) => u,
            Err(_) => continue,
        };

        for imp in &imports {
            if !imp.path.starts_with('.') {
                continue;
            }

            let target_fs = normalize_path(&old_dir.join(&imp.path));
            let effective_target = rename_map.get(&target_fs).unwrap_or(&target_fs);

            let new_rel = match pathdiff::diff_paths(effective_target, new_dir) {
                Some(p) => p,
                None => {
                    stats.pathdiff_failures += 1;
                    continue;
                }
            };

            let new_import_str = ensure_dot_prefix(&new_rel);

            if new_import_str == imp.path {
                stats.no_op_skips += 1;
                continue;
            }

            let already_edited = edits.get(&old_uri).map_or(false, |file_edits| {
                let qr = range_with_quotes(imp.inner_range);
                file_edits.iter().any(|e| e.range == qr)
            });
            if already_edited {
                stats.dedup_skips += 1;
                continue;
            }

            edits.entry(old_uri.clone()).or_default().push(TextEdit {
                range: range_with_quotes(imp.inner_range),
                new_text: format!("\"{}\"", new_import_str),
            });
        }
    }

    RenameResult { edits, stats }
}

/// Backward-compatible wrapper: single rename, used by existing tests.
pub fn rename_imports_single(
    source_files: &[String],
    old_uri: &Url,
    new_uri: &Url,
    project_root: &Path,
    get_source_bytes: &dyn Fn(&str) -> Option<Vec<u8>>,
) -> HashMap<Url, Vec<TextEdit>> {
    let old_path = match old_uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return HashMap::new(),
    };
    let new_path = match new_uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return HashMap::new(),
    };
    rename_imports(
        source_files,
        &[FileRename { old_path, new_path }],
        project_root,
        get_source_bytes,
    )
    .edits
}

// ---------------------------------------------------------------------------
// Cache patching
// ---------------------------------------------------------------------------

/// Apply computed edits to in-memory file content.
///
/// Returns the number of files patched (for logging).
pub fn apply_edits_to_cache(
    edits: &HashMap<Url, Vec<TextEdit>>,
    cache: &mut HashMap<String, (i32, String)>,
) -> usize {
    let mut patched = 0;
    for (uri, text_edits) in edits {
        let uri_str = uri.to_string();
        if let Some((version, content)) = cache.get(&uri_str).cloned() {
            let new_content = apply_text_edits(&content, text_edits);
            cache.insert(uri_str, (version, new_content));
            patched += 1;
        }
    }
    patched
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Expand a range to include the surrounding quote characters.
fn range_with_quotes(inner: Range) -> Range {
    Range {
        start: Position {
            line: inner.start.line,
            character: inner.start.character.saturating_sub(1),
        },
        end: Position {
            line: inner.end.line,
            character: inner.end.character + 1,
        },
    }
}

/// Ensure a relative path starts with `./` or `../` for Solidity import convention.
/// Always uses forward slashes regardless of platform.
fn ensure_dot_prefix(rel: &Path) -> String {
    let s = normalize_slashes(&rel.to_string_lossy());
    if s.starts_with("..") || s.starts_with('.') {
        s
    } else {
        format!("./{s}")
    }
}

/// Replace backslashes with forward slashes for Solidity import paths.
/// Solidity uses forward slashes in import strings regardless of platform.
fn normalize_slashes(s: &str) -> String {
    s.replace('\\', "/")
}

/// Apply a set of `TextEdit`s to a source string and return the new content.
///
/// Edits are sorted in reverse document order so that earlier byte offsets
/// remain valid as we splice in replacements from the end.
///
/// Positions are interpreted according to the negotiated encoding
/// (UTF-8 or UTF-16), matching the positions produced by `ts_find_imports`.
pub fn apply_text_edits(source: &str, edits: &[TextEdit]) -> String {
    // Convert to byte ranges on the original source.
    let mut resolved: Vec<(usize, usize, &str)> = edits
        .iter()
        .filter_map(|e| {
            let start = utils::position_to_byte_offset(source, e.range.start);
            let end = utils::position_to_byte_offset(source, e.range.end);
            if start > end {
                tracing::warn!(
                    "apply_text_edits: skipping invalid edit range start={} end={}",
                    start,
                    end
                );
                None
            } else {
                Some((start, end, e.new_text.as_str()))
            }
        })
        .collect();

    // Keep non-overlapping edits in forward order. For overlapping spans,
    // prefer the earliest-starting edit (wider edit when starts are equal).
    resolved.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let mut filtered: Vec<(usize, usize, &str)> = Vec::with_capacity(resolved.len());
    for (start, end, new_text) in resolved {
        if let Some((_, last_end, _)) = filtered.last()
            && start < *last_end
        {
            tracing::warn!(
                "apply_text_edits: skipping overlapping edit range start={} end={} last_end={}",
                start,
                end,
                last_end
            );
            continue;
        }
        filtered.push((start, end, new_text));
    }

    // Apply from back to front so earlier offsets remain valid.
    let mut result = source.to_string();
    for (start, end, new_text) in filtered.into_iter().rev() {
        result.replace_range(start..end, new_text);
    }
    result
}

/// Normalize a path by resolving `.` and `..` components without requiring
/// the file to exist on disk (unlike `std::fs::canonicalize`).
///
/// Guards against excessive `..` that would pop past the root by only
/// popping `Normal` components (never `RootDir` or `Prefix`).
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if matches!(components.last(), Some(std::path::Component::Normal(_))) {
                    components.pop();
                }
            }
            other => components.push(other),
        }
    }
    components.iter().collect()
}
