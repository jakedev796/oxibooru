use std::process::Command;

fn main() {
    let pkg_version = env!("CARGO_PKG_VERSION");

    // Try git describe with tags first (e.g., "v0.7.4-0-gc3167431")
    let git_describe = Command::new("git")
        .args(["describe", "--always", "--dirty", "--long", "--tags"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    // Get short commit hash
    let short_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Check if working tree is dirty
    let dirty = Command::new("git")
        .args(["diff", "--quiet", "HEAD"])
        .status()
        .map(|s| !s.success())
        .unwrap_or(false);

    let dirty_suffix = if dirty { "-dirty" } else { "" };

    // If git describe found a tag, use it; otherwise construct from package version + hash
    let git_version = if git_describe.contains('-') && !git_describe.starts_with(&short_hash) {
        // Has tags — git describe gives something like "v0.7.4-0-gc3167431-dirty"
        git_describe
    } else {
        // No tags — construct "v{pkg_version}+{hash}[-dirty]"
        format!("v{pkg_version}+{short_hash}{dirty_suffix}")
    };

    println!("cargo:rustc-env=GIT_VERSION={git_version}");

    // Embed build date as UTC string
    let build_date = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_DATE={build_date}");

    // Re-run if git HEAD changes
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/refs/");
}
