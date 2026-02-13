use std::process::Command;

fn main() {
    // Git commit hash: try git first, fall back to .cargo_vcs_info.json
    // (which cargo embeds automatically when publishing to crates.io)
    let commit = git_commit()
        .or_else(cargo_vcs_commit)
        .unwrap_or_else(|| "unknown".to_string());

    let version = env!("CARGO_PKG_VERSION");
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let long_version = format!("{version}+commit.{commit}.{os}.{arch}");
    println!("cargo:rustc-env=LONG_VERSION={long_version}");

    // Rerun if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
    println!("cargo:rerun-if-changed=.cargo_vcs_info.json");
}

/// Read commit hash from local git repo.
fn git_commit() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Read commit hash from .cargo_vcs_info.json (present in crates.io builds).
fn cargo_vcs_commit() -> Option<String> {
    let content = std::fs::read_to_string(".cargo_vcs_info.json").ok()?;
    // Parse: {"git":{"sha1":"abc123..."}}
    // Avoid pulling in serde for the build script — just find the sha1 value.
    let sha1_key = "\"sha1\"";
    let idx = content.find(sha1_key)?;
    let rest = &content[idx + sha1_key.len()..];
    let start = rest.find('"')? + 1;
    let end = start + rest[start..].find('"')?;
    let full_sha = &rest[start..end];
    // Return short hash (first 7 chars) to match git --short
    Some(full_sha.chars().take(7).collect())
}
