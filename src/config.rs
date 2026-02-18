use std::path::{Path, PathBuf};

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

/// Walk up from `start` to find the nearest `foundry.toml`.
pub fn find_foundry_toml(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };

    loop {
        let candidate = current.join("foundry.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        if !current.pop() {
            return None;
        }
    }
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
}
