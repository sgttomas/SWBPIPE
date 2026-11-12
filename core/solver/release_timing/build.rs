//! Embeds the build's identity for `release_timing --supervise`'s metadata: the source commit,
//! its tree, whether the working tree was dirty, the build profile, and the `rustc -Vv` and
//! `cargo -Vv` of the toolchain that built it. Where git or a tool is absent, each value is
//! "unknown" (the dirty flag null). Standard library only: no build dependency.
//!
//! It runs again when HEAD, the checked-out branch's ref, the index or this crate's files change.
//! A change to another crate's files alone (a product crate edited without staging) does not
//! rerun it, so the dirty flag is the tree's state when it last ran: build a carried binary from
//! a clean checkout.

use std::path::{Path, PathBuf};
use std::process::Command;

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

fn main() {
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    for path in ["build.rs", "src", "Cargo.toml", "Cargo.lock"] {
        println!("cargo:rerun-if-changed={path}");
    }
    let git = |args: &[&str]| run("git", args, &dir);
    let mut watched = vec![git(&["rev-parse", "--git-path", "HEAD"]), git(&["rev-parse", "--git-path", "index"])];
    if let Some(reference) = git(&["symbolic-ref", "-q", "HEAD"]) {
        watched.push(git(&["rev-parse", "--git-path", &reference]));
        watched.push(git(&["rev-parse", "--git-path", "packed-refs"]));
    }
    for path in watched.into_iter().flatten() {
        let path = dir.join(path);
        // A path that does not exist would make cargo rerun this script on every build.
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    let commit = git(&["rev-parse", "HEAD"]);
    let tree = git(&["rev-parse", "HEAD^{tree}"]);
    let dirty = git(&["status", "--porcelain"]).map(|status| !status.is_empty());
    let tool = |var: &str, fallback: &str| std::env::var(var).unwrap_or_else(|_| fallback.to_string());
    let rustc = run(&tool("RUSTC", "rustc"), &["-Vv"], &dir);
    let cargo = run(&tool("CARGO", "cargo"), &["-Vv"], &dir);
    let profile = std::env::var("PROFILE").ok();
    let text = |value: Option<String>| quote(value.as_deref().unwrap_or("unknown"));
    let identity = format!(
        "{{\"source_commit\": {}, \"source_tree\": {}, \"source_dirty\": {}, \"build_profile\": {}, \"rustc\": {}, \"cargo\": {}}}\n",
        text(commit),
        text(tree),
        dirty.map_or("null".to_string(), |d| d.to_string()),
        text(profile),
        lines(rustc),
        lines(cargo),
    );
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    std::fs::write(out.join("build_identity.json"), identity).expect("write the build identity");
}
