//! Embeds the build's identity (`BUILD_IDENTITY` in `src/lib.rs`, through the compiler environment
//! variable `RELEASE_TIMING_BUILD_IDENTITY`): the source commit, its tree,
//! whether the working tree was dirty, the build profile, and the `rustc -Vv` and `cargo -Vv` of
//! the toolchain that built it. Standard library only: no build dependency.
//!
//! It runs on every build (its one `rerun-if-changed` path never exists), so the identity is read
//! from the tree the executable is linked from: an unstaged edit of a linked product crate, which
//! rebuilds that crate and relinks this one, is seen here as a dirty tree.
//!
//! `source_dirty` has three states: `false` only when `git status --porcelain` ran and listed
//! nothing (the whole repository, untracked files included, so every linked crate's source is
//! covered); `true` when it listed anything; and `"unknown"` whenever git cannot establish it (no
//! git, no repository, a failed status, or no commit), never `false` then. Where git or a tool is
//! absent, each other value is "unknown".

use std::path::{Path, PathBuf};
use std::process::Command;

/// A path that never exists: cargo reruns a build script whose `rerun-if-changed` path is absent
/// on every build.
const ALWAYS_RERUN: &str = "build-identity-is-read-on-every-build.absent";

fn run(program: &str, args: &[&str], dir: &Path) -> Option<String> {
    let output = Command::new(program).args(args).current_dir(dir).output().ok()?;
    output.status.success().then(|| String::from_utf8_lossy(&output.stdout).trim_end().to_string())
}

fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn lines(text: Option<String>) -> String {
    match text {
        Some(text) => format!("[{}]", text.lines().map(quote).collect::<Vec<_>>().join(", ")),
        None => quote("unknown"),
    }
}

/// `source_dirty` as JSON: `true`, `false`, or `"unknown"` (never `false` without a clean status
/// of a tree that has a commit).
fn dirty_flag(commit: Option<&str>, status: Option<&str>) -> String {
    match (commit, status) {
        (Some(_), Some(status)) => (!status.is_empty()).to_string(),
        _ => quote("unknown"),
    }
}

fn main() {
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    assert!(!dir.join(ALWAYS_RERUN).exists(), "{ALWAYS_RERUN} must not exist: it makes the build identity current");
    println!("cargo:rerun-if-changed={ALWAYS_RERUN}");
    let git = |args: &[&str]| run("git", args, &dir);
    let commit = git(&["rev-parse", "--verify", "HEAD"]);
    let tree = git(&["rev-parse", "--verify", "HEAD^{tree}"]);
    let status = git(&["status", "--porcelain"]);
    let tool = |var: &str, fallback: &str| std::env::var(var).unwrap_or_else(|_| fallback.to_string());
    let rustc = run(&tool("RUSTC", "rustc"), &["-Vv"], &dir);
    let cargo = run(&tool("CARGO", "cargo"), &["-Vv"], &dir);
    let profile = std::env::var("PROFILE").ok();
    let text = |value: Option<&str>| quote(value.unwrap_or("unknown"));
    // One line of JSON (every string is escaped by `quote`), passed to the compiler as an
    // environment variable that `src/lib.rs` reads with `env!`.
    let identity = format!(
        "{{\"source_commit\": {}, \"source_tree\": {}, \"source_dirty\": {}, \"build_profile\": {}, \"rustc\": {}, \"cargo\": {}}}",
        text(commit.as_deref()),
        text(tree.as_deref()),
        dirty_flag(commit.as_deref(), status.as_deref()),
        text(profile.as_deref()),
        lines(rustc),
        lines(cargo),
    );
    println!("cargo:rustc-env=RELEASE_TIMING_BUILD_IDENTITY={identity}");
}
