use std::path::{Path, PathBuf};

/// Project-level configuration extracted from `foundry.toml`.
///
/// This includes both lint settings and compiler settings needed by the
/// solc runner (solc version, remappings).
#[derive(Debug, Clone)]
pub struct FoundryConfig {
    /// The project root where `foundry.toml` was found.
    pub root: PathBuf,
    /// Solc version from `[profile.default] solc = "0.8.26"`.
    /// `None` means use the system default.
    pub solc_version: Option<String>,
    /// Remappings from `[profile.default] remappings = [...]`.
    /// Empty if not specified (will fall back to `forge remappings`).
    pub remappings: Vec<String>,
}

impl Default for FoundryConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::new(),
            solc_version: None,
            remappings: Vec::new(),
        }
    }
}

/// Load project configuration from the nearest `foundry.toml`.
pub fn load_foundry_config(file_path: &Path) -> FoundryConfig {
    let toml_path = match find_foundry_toml(file_path) {
        Some(p) => p,
        None => return FoundryConfig::default(),
    };
    load_foundry_config_from_toml(&toml_path)
}

/// Load project configuration from a known `foundry.toml` path.
pub fn load_foundry_config_from_toml(toml_path: &Path) -> FoundryConfig {
    let root = toml_path.parent().unwrap_or(Path::new("")).to_path_buf();

    let content = match std::fs::read_to_string(toml_path) {
        Ok(c) => c,
        Err(_) => {
            return FoundryConfig {
                root,
                ..Default::default()
            };
        }
    };

    let table: toml::Table = match content.parse() {
        Ok(t) => t,
        Err(_) => {
            return FoundryConfig {
                root,
                ..Default::default()
            };
        }
    };

    let profile_name = std::env::var("FOUNDRY_PROFILE").unwrap_or_else(|_| "default".to_string());

    let profile = table
        .get("profile")
        .and_then(|p| p.as_table())
        .and_then(|p| p.get(&profile_name))
        .and_then(|p| p.as_table());

    let profile = match profile {
        Some(p) => p,
        None => {
            return FoundryConfig {
                root,
                ..Default::default()
            };
        }
    };

    // Parse solc version: `solc = "0.8.26"` or `solc_version = "0.8.26"`
    let solc_version = profile
        .get("solc")
        .or_else(|| profile.get("solc_version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // Parse remappings: `remappings = ["ds-test/=lib/...", ...]`
    let remappings = profile
        .get("remappings")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();

    FoundryConfig {
        root,
        solc_version,
        remappings,
    }
}

/// Lint-related configuration extracted from `foundry.toml`.
#[derive(Debug, Clone)]
pub struct LintConfig {
    /// The project root where `foundry.toml` was found.
    pub root: PathBuf,
    /// Whether linting is enabled on build (default: true).
    pub lint_on_build: bool,
    /// Compiled glob patterns from the `ignore` list.
    pub ignore_patterns: Vec<glob::Pattern>,
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::new(),
            lint_on_build: true,
            ignore_patterns: Vec::new(),
        }
    }
}

impl LintConfig {
    /// Returns `true` if the given file should be linted.
    ///
    /// A file is skipped when:
    /// - `lint_on_build` is `false`, or
    /// - the file's path (relative to the project root) matches any `ignore` pattern.
    pub fn should_lint(&self, file_path: &Path) -> bool {
        if !self.lint_on_build {
            return false;
        }

        if self.ignore_patterns.is_empty() {
            return true;
        }

        // Build a relative path from the project root so that patterns like
        // "test/**/*" work correctly.
        let relative = file_path.strip_prefix(&self.root).unwrap_or(file_path);

        let rel_str = relative.to_string_lossy();

        for pattern in &self.ignore_patterns {
            if pattern.matches(&rel_str) {
                return false;
            }
        }

        true
    }
}

/// Returns the root of the git repository containing `start`, if any.
///
/// This mirrors foundry's own `find_git_root` behavior: walk up ancestors
/// until a directory containing `.git` is found.
fn find_git_root(start: &Path) -> Option<PathBuf> {
    let start = if start.is_file() {
        start.parent()?
    } else {
        start
    };
    start
        .ancestors()
        .find(|p| p.join(".git").exists())
        .map(Path::to_path_buf)
}

/// Walk up from `start` to find the nearest `foundry.toml`, stopping at the
/// git repository root (consistent with foundry's `find_project_root`).
///
/// See: <https://github.com/foundry-rs/foundry/blob/5389caefb5bfb035c547dffb4fd0f441a37e5371/crates/config/src/utils.rs#L62>
pub fn find_foundry_toml(start: &Path) -> Option<PathBuf> {
    let start_dir = if start.is_file() {
        start.parent()?
    } else {
        start
    };

    let boundary = find_git_root(start_dir);

    start_dir
        .ancestors()
        // Don't look outside of the git repo, matching foundry's behavior.
        .take_while(|p| {
            if let Some(boundary) = &boundary {
                p.starts_with(boundary)
            } else {
                true
            }
        })
        .find(|p| p.join("foundry.toml").is_file())
        .map(|p| p.join("foundry.toml"))
}

/// Load the lint configuration from the nearest `foundry.toml` relative to
/// `file_path`. Returns `LintConfig::default()` when no config is found or
/// the relevant sections are absent.
pub fn load_lint_config(file_path: &Path) -> LintConfig {
    let toml_path = match find_foundry_toml(file_path) {
        Some(p) => p,
        None => return LintConfig::default(),
    };

    let root = toml_path.parent().unwrap_or(Path::new("")).to_path_buf();

    let content = match std::fs::read_to_string(&toml_path) {
        Ok(c) => c,
        Err(_) => {
            return LintConfig {
                root,
                ..Default::default()
            };
        }
    };

    let table: toml::Table = match content.parse() {
        Ok(t) => t,
        Err(_) => {
            return LintConfig {
                root,
                ..Default::default()
            };
        }
    };

    // Determine the active profile (default: "default").
    let profile_name = std::env::var("FOUNDRY_PROFILE").unwrap_or_else(|_| "default".to_string());

    // Look up [profile.<name>.lint]
    let lint_table = table
        .get("profile")
        .and_then(|p| p.as_table())
        .and_then(|p| p.get(&profile_name))
        .and_then(|p| p.as_table())
        .and_then(|p| p.get("lint"))
        .and_then(|l| l.as_table());

    let lint_table = match lint_table {
        Some(t) => t,
        None => {
            return LintConfig {
                root,
                ..Default::default()
            };
        }
    };

    // Parse lint_on_build (default: true)
    let lint_on_build = lint_table
        .get("lint_on_build")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    // Parse ignore patterns
    let ignore_patterns = lint_table
        .get("ignore")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .filter_map(|s| glob::Pattern::new(s).ok())
                .collect()
        })
        .unwrap_or_default();

    LintConfig {
        root,
        lint_on_build,
        ignore_patterns,
    }
}

/// Load lint config from a known `foundry.toml` path (used when reloading
/// after a file-watch notification).
pub fn load_lint_config_from_toml(toml_path: &Path) -> LintConfig {
    let root = toml_path.parent().unwrap_or(Path::new("")).to_path_buf();

    let content = match std::fs::read_to_string(toml_path) {
        Ok(c) => c,
        Err(_) => {
            return LintConfig {
                root,
                ..Default::default()
            };
        }
    };

    let table: toml::Table = match content.parse() {
        Ok(t) => t,
        Err(_) => {
            return LintConfig {
                root,
                ..Default::default()
            };
        }
    };

    let profile_name = std::env::var("FOUNDRY_PROFILE").unwrap_or_else(|_| "default".to_string());

    let lint_table = table
        .get("profile")
        .and_then(|p| p.as_table())
        .and_then(|p| p.get(&profile_name))
        .and_then(|p| p.as_table())
        .and_then(|p| p.get("lint"))
        .and_then(|l| l.as_table());

    let lint_table = match lint_table {
        Some(t) => t,
        None => {
            return LintConfig {
                root,
                ..Default::default()
            };
        }
    };

    let lint_on_build = lint_table
        .get("lint_on_build")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let ignore_patterns = lint_table
        .get("ignore")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .filter_map(|s| glob::Pattern::new(s).ok())
                .collect()
        })
        .unwrap_or_default();

    LintConfig {
        root,
        lint_on_build,
        ignore_patterns,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_default_config_lints_everything() {
        let config = LintConfig::default();
        assert!(config.should_lint(Path::new("test/MyTest.sol")));
        assert!(config.should_lint(Path::new("src/Token.sol")));
    }

    #[test]
    fn test_lint_on_build_false_skips_all() {
        let config = LintConfig {
            lint_on_build: false,
            ..Default::default()
        };
        assert!(!config.should_lint(Path::new("src/Token.sol")));
    }

    #[test]
    fn test_ignore_pattern_matches() {
        let config = LintConfig {
            root: PathBuf::from("/project"),
            lint_on_build: true,
            ignore_patterns: vec![glob::Pattern::new("test/**/*").unwrap()],
        };
        assert!(!config.should_lint(Path::new("/project/test/MyTest.sol")));
        assert!(config.should_lint(Path::new("/project/src/Token.sol")));
    }

    #[test]
    fn test_multiple_ignore_patterns() {
        let config = LintConfig {
            root: PathBuf::from("/project"),
            lint_on_build: true,
            ignore_patterns: vec![
                glob::Pattern::new("test/**/*").unwrap(),
                glob::Pattern::new("script/**/*").unwrap(),
            ],
        };
        assert!(!config.should_lint(Path::new("/project/test/MyTest.sol")));
        assert!(!config.should_lint(Path::new("/project/script/Deploy.sol")));
        assert!(config.should_lint(Path::new("/project/src/Token.sol")));
    }

    #[test]
    fn test_load_lint_config_from_toml() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("foundry.toml");
        fs::write(
            &toml_path,
            r#"
[profile.default.lint]
ignore = ["test/**/*"]
lint_on_build = true
"#,
        )
        .unwrap();

        let config = load_lint_config_from_toml(&toml_path);
        assert!(config.lint_on_build);
        assert_eq!(config.ignore_patterns.len(), 1);
        assert!(!config.should_lint(&dir.path().join("test/MyTest.sol")));
        assert!(config.should_lint(&dir.path().join("src/Token.sol")));
    }

    #[test]
    fn test_load_lint_config_lint_on_build_false() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("foundry.toml");
        fs::write(
            &toml_path,
            r#"
[profile.default.lint]
lint_on_build = false
"#,
        )
        .unwrap();

        let config = load_lint_config_from_toml(&toml_path);
        assert!(!config.lint_on_build);
        assert!(!config.should_lint(&dir.path().join("src/Token.sol")));
    }

    #[test]
    fn test_load_lint_config_no_lint_section() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("foundry.toml");
        fs::write(
            &toml_path,
            r#"
[profile.default]
src = "src"
"#,
        )
        .unwrap();

        let config = load_lint_config_from_toml(&toml_path);
        assert!(config.lint_on_build);
        assert!(config.ignore_patterns.is_empty());
    }

    #[test]
    fn test_find_foundry_toml() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("foundry.toml");
        fs::write(&toml_path, "[profile.default]").unwrap();

        // Create a nested directory
        let nested = dir.path().join("src");
        fs::create_dir_all(&nested).unwrap();

        let found = find_foundry_toml(&nested);
        assert_eq!(found, Some(toml_path));
    }

    #[test]
    fn test_load_lint_config_walks_ancestors() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("foundry.toml");
        fs::write(
            &toml_path,
            r#"
[profile.default.lint]
ignore = ["test/**/*"]
"#,
        )
        .unwrap();

        let nested_file = dir.path().join("src/Token.sol");
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(&nested_file, "// solidity").unwrap();

        let config = load_lint_config(&nested_file);
        assert_eq!(config.root, dir.path());
        assert_eq!(config.ignore_patterns.len(), 1);
    }

    #[test]
    fn test_find_git_root() {
        let dir = tempfile::tempdir().unwrap();
        // Create a fake .git directory
        fs::create_dir_all(dir.path().join(".git")).unwrap();
        let nested = dir.path().join("sub/deep");
        fs::create_dir_all(&nested).unwrap();

        let root = find_git_root(&nested);
        assert_eq!(root, Some(dir.path().to_path_buf()));
    }

    #[test]
    fn test_find_foundry_toml_stops_at_git_boundary() {
        // Layout:
        //   tmp/
        //     foundry.toml          <-- outside git repo, should NOT be found
        //     repo/
        //       .git/
        //       sub/
        //         [search starts here]
        let dir = tempfile::tempdir().unwrap();

        // foundry.toml outside the git repo
        fs::write(dir.path().join("foundry.toml"), "[profile.default]").unwrap();

        // git repo with no foundry.toml
        let repo = dir.path().join("repo");
        fs::create_dir_all(repo.join(".git")).unwrap();
        fs::create_dir_all(repo.join("sub")).unwrap();

        let found = find_foundry_toml(&repo.join("sub"));
        // Should NOT find the foundry.toml above the .git boundary
        assert_eq!(found, None);
    }

    #[test]
    fn test_find_foundry_toml_within_git_boundary() {
        // Layout:
        //   tmp/
        //     repo/
        //       .git/
        //       foundry.toml        <-- inside git repo, should be found
        //       src/
        //         [search starts here]
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path().join("repo");
        fs::create_dir_all(repo.join(".git")).unwrap();
        fs::create_dir_all(repo.join("src")).unwrap();
        let toml_path = repo.join("foundry.toml");
        fs::write(&toml_path, "[profile.default]").unwrap();

        let found = find_foundry_toml(&repo.join("src"));
        assert_eq!(found, Some(toml_path));
    }

    #[test]
    fn test_find_foundry_toml_no_git_repo_still_walks_up() {
        // When there's no .git directory at all, the search should still
        // walk up (unbounded), matching foundry's behavior.
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("foundry.toml");
        fs::write(&toml_path, "[profile.default]").unwrap();

        let nested = dir.path().join("a/b/c");
        fs::create_dir_all(&nested).unwrap();

        let found = find_foundry_toml(&nested);
        assert_eq!(found, Some(toml_path));
    }
}
