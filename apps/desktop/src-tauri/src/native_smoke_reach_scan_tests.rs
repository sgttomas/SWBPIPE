//! Reach scan of the native-smoke result injection (`native_smoke_inject.rs`;
//! RT-U4, DEL-04-05 `Work/criterion-9-measurement.md`): a product build, which
//! never enables the `native-smoke` feature, cannot reach it. Runs in every
//! `cargo test` of this crate, with or without the feature.
//!
//! - Outside the module itself, every source line that names the module, its
//!   entry, its environment variable or the constant holding it is directly
//!   preceded (comments aside) by exactly `#[cfg(feature = "native-smoke")]`.
//!   Comment lines are not code and are skipped.
//! - The module is declared once, in `lib.rs`.
//! - No feature of this crate implies `native-smoke`, and it is not a default.
//!
//! The release binary check (the variable's name absent from a product build)
//! is in DEL-04-05 `Work/criterion-9-measurement.md`, "Desktop measurement".

use std::path::{Path, PathBuf};

const GATE: &str = "#[cfg(feature = \"native-smoke\")]";
const NEEDLES: [&str; 4] = ["SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "native_smoke_inject", "injected_outcome", "INJECT_RESULT_ENV"];
const EXEMPT: [&str; 2] = ["native_smoke_inject.rs", "native_smoke_reach_scan_tests.rs"];

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

fn is_comment(line: &str) -> bool {
    line.trim_start().starts_with("//")
}

/// Every line of `text` naming a needle whose preceding code line is not the gate.
fn ungated(text: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = text.lines().collect();
    let mut found = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if is_comment(line) || !NEEDLES.iter().any(|needle| line.contains(needle)) {
            continue;
        }
        let previous = lines[..index].iter().rev().find(|line| !line.trim().is_empty() && !is_comment(line));
        if previous.map(|line| line.trim()) != Some(GATE) {
            found.push((index + 1, line.trim().to_string()));
        }
    }
    found
}

#[test]
fn the_result_injection_is_named_only_under_the_native_smoke_gate() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    sources(&root.join("src"), &mut files);
    assert!(files.len() > 20, "the scan reads the crate's sources");
    let mut problems = Vec::new();
    let mut declarations = 0;
    for file in &files {
        let name = file.file_name().unwrap().to_string_lossy();
        if EXEMPT.contains(&name.as_ref()) {
            continue;
        }
        let text = std::fs::read_to_string(file).unwrap();
        declarations += text.lines().filter(|line| line.trim() == "mod native_smoke_inject;").count();
        for (line, code) in ungated(&text) {
            problems.push(format!("{}:{line}: {code}", file.strip_prefix(root).unwrap().display()));
        }
    }
    assert!(problems.is_empty(), "named outside the native-smoke gate:\n{}", problems.join("\n"));
    assert_eq!(declarations, 1, "the module is declared once");
    let lib = std::fs::read_to_string(root.join("src/lib.rs")).unwrap();
    assert!(lib.contains(&format!("{GATE}\nmod native_smoke_inject;\n")), "lib.rs declares it under the gate");

    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    let features = manifest.split("[features]").nth(1).and_then(|rest| rest.split("\n[").next()).expect("a [features] table");
    let implying: Vec<&str> = features
        .lines()
        .filter(|line| !line.trim_start().starts_with('#') && line.contains("native-smoke") && line.trim() != "native-smoke = []")
        .collect();
    assert!(implying.is_empty(), "no feature or default implies native-smoke: {implying:?}");
}

#[test]
fn the_scan_finds_an_ungated_name() {
    let gated = format!("{GATE}\nif let Some(o) = crate::native_smoke_inject::injected_outcome(p, b, c) {{}}\n");
    assert!(ungated(&gated).is_empty());
    let commented = format!("{GATE}\n// a note\nmod native_smoke_inject;\n");
    assert!(ungated(&commented).is_empty());
    assert_eq!(ungated("let v = std::env::var(\"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT\");\n").len(), 1);
    assert_eq!(ungated("#[cfg(any(test, feature = \"native-smoke\"))]\nmod native_smoke_inject;\n").len(), 1);
    assert_eq!(ungated(&format!("{GATE}\nfn a() {{}}\nfn b() {{ injected_outcome(); }}\n")).len(), 1);
    // A comment line naming it is not code.
    assert!(ungated("// see native_smoke_inject.rs\n").is_empty());
}
