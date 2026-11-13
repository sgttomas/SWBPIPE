//! Embeds the build's identity (`BUILD_IDENTITY` in `src/lib.rs`, through the compiler environment
//! variable `RELEASE_TIMING_BUILD_IDENTITY`): the source commit, its tree,
//! whether the working tree was dirty, the build profile, and the `rustc -Vv` and `cargo -Vv` of
//! the toolchain that built it. Standard library only: no build dependency.
//!
//! It runs on every build (its one `rerun-if-changed` path never exists), so the identity is read
//! from the tree the executable is linked from: an unstaged edit of a linked product crate, which
//! rebuilds that crate and relinks this one, is seen here as a dirty tree.
//!
//! `source_dirty` has three states: `false` only when `git status --porcelain --untracked-files=all`
//! ran and listed nothing (the whole repository, every untracked file included whatever the
//! repository's configuration says, so every linked crate's source is covered) and no index entry
//! carries a flag that hides a file's changes from that status (`git ls-files -v`: assume-unchanged,
//! a lower-case tag, or skip-worktree, `S`); `true` when the status listed anything; and
//! `"unknown"` whenever git cannot establish it (no git, no repository, a repository that is not
//! this crate's, a failed status or listing, no commit, or a clean status with such a flag set),
//! never `false` then. Where git or a tool is absent, each other value is "unknown".
//!
//! Every git call is isolated, so that the identity can only describe the repository this crate
//! sits in: each runs with every inherited `GIT_*` variable removed (an inherited `GIT_DIR` or
//! `GIT_WORK_TREE` would otherwise point it at another repository), with no system or global
//! configuration, without optional locks (it writes nothing to the repository), and bound to an
//! explicit path. The repository's top level must be the crate's own root (three levels above this
//! crate, `core/solver/release_timing`); its git directory is then passed explicitly
//! (`--git-dir`, `--work-tree`) and the top level verified again before any answer is trusted.
//! Anything that cannot be verified gives "unknown".

use std::path::{Path, PathBuf};
use std::process::Command;

/// A path that never exists: cargo reruns a build script whose `rerun-if-changed` path is absent
/// on every build.
const ALWAYS_RERUN: &str = "build-identity-is-read-on-every-build.absent";
/// A home folder that does not exist, so no user configuration is found by a git too old for
/// `GIT_CONFIG_GLOBAL`.
const NO_HOME: &str = "/nonexistent-release-timing-build-home";

fn run(program: &str, args: &[&str], dir: &Path) -> Option<String> {
    let output = Command::new(program).args(args).current_dir(dir).output().ok()?;
    output.status.success().then(|| String::from_utf8_lossy(&output.stdout).trim_end().to_string())
}

/// One isolated git call in `dir`: no inherited `GIT_*` variable, no system or global
/// configuration, no optional locks, no file-system monitor.
fn git(args: &[&str], dir: &Path) -> Option<String> {
    let mut command = Command::new("git");
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("HOME", NO_HOME)
        .env("XDG_CONFIG_HOME", NO_HOME)
        .args(["--no-optional-locks", "-c", "core.fsmonitor=false", "-C"])
        .arg(dir)
        .args(args)
        .current_dir(dir);
    let output = command.output().ok()?;
    output.status.success().then(|| String::from_utf8_lossy(&output.stdout).trim_end().to_string())
}

/// What the repository whose top level is the crate's root says: each value `None` where it cannot
/// be verified.
struct SourceState {
    commit: Option<String>,
    tree: Option<String>,
    /// `git status --porcelain` output.
    status: Option<String>,
    /// Whether an index entry is flagged assume-unchanged or skip-worktree.
    index_flags: Option<bool>,
}

/// Whether `git ls-files -v -z` output lists an entry whose changes `git status` does not show:
/// assume-unchanged (a lower-case tag) or skip-worktree (`S`, or `s` with both).
fn hides_changes(listing: &str) -> bool {
    listing.split('\0').filter_map(|entry| entry.chars().next()).any(|tag| tag.is_ascii_lowercase() || tag == 'S')
}

fn source_state(crate_dir: &Path) -> SourceState {
    let none = SourceState { commit: None, tree: None, status: None, index_flags: None };
    let Some(root) = crate_dir.join("../../..").canonicalize().ok() else { return none };
    let same = |reported: Option<String>| reported.and_then(|p| Path::new(&p).canonicalize().ok()).is_some_and(|p| p == root);
    if !same(git(&["rev-parse", "--show-toplevel"], crate_dir)) {
        return none;
    }
    let Some(git_dir) = git(&["rev-parse", "--absolute-git-dir"], &root) else { return none };
    let root_text = root.to_string_lossy().to_string();
    let bound = |rest: &[&str]| {
        let mut args = vec!["--git-dir", git_dir.as_str(), "--work-tree", root_text.as_str()];
        args.extend_from_slice(rest);
        git(&args, &root)
    };
    if !same(bound(&["rev-parse", "--show-toplevel"])) {
        return none;
    }
    let commit = bound(&["rev-parse", "--verify", "HEAD"]);
    let tree = bound(&["rev-parse", "--verify", "HEAD^{tree}"]);
    let status = bound(&["status", "--porcelain", "--untracked-files=all", "--ignore-submodules=none"]);
    let index_flags = bound(&["ls-files", "-v", "-z"]).map(|listing| hides_changes(&listing));
    SourceState { commit, tree, status, index_flags }
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

/// `source_dirty` as JSON: `true` when the status lists a change, `false` only for a clean status
/// of a tree that has a commit and no index entry flagged to hide its changes, and otherwise
/// `"unknown"`.
fn dirty_flag(state: &SourceState) -> String {
    match (&state.commit, &state.status, state.index_flags) {
        (Some(_), Some(status), _) if !status.is_empty() => "true".to_string(),
        (Some(_), Some(_), Some(false)) => "false".to_string(),
        _ => quote("unknown"),
    }
}

fn main() {
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    assert!(!dir.join(ALWAYS_RERUN).exists(), "{ALWAYS_RERUN} must not exist: it makes the build identity current");
    println!("cargo:rerun-if-changed={ALWAYS_RERUN}");
    let state = source_state(&dir);
    let tool = |var: &str, fallback: &str| std::env::var(var).unwrap_or_else(|_| fallback.to_string());
    let rustc = run(&tool("RUSTC", "rustc"), &["-Vv"], &dir);
    let cargo = run(&tool("CARGO", "cargo"), &["-Vv"], &dir);
    let profile = std::env::var("PROFILE").ok();
    let text = |value: Option<&str>| quote(value.unwrap_or("unknown"));
    // One line of JSON (every string is escaped by `quote`), passed to the compiler as an
    // environment variable that `src/lib.rs` reads with `env!`.
    let identity = format!(
        "{{\"source_commit\": {}, \"source_tree\": {}, \"source_dirty\": {}, \"build_profile\": {}, \"rustc\": {}, \"cargo\": {}}}",
        text(state.commit.as_deref()),
        text(state.tree.as_deref()),
        dirty_flag(&state),
        text(profile.as_deref()),
        lines(rustc),
        lines(cargo),
    );
    println!("cargo:rustc-env=RELEASE_TIMING_BUILD_IDENTITY={identity}");
}
