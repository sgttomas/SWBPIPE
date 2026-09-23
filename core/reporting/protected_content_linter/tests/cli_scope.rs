use std::fs;
use std::process::Command;

#[test]
fn cli_scans_only_the_explicitly_named_file() {
    let directory = std::env::temp_dir().join(format!("ops-lint-cli-scope-{}", std::process::id()));
    fs::create_dir_all(&directory).expect("create fixture directory");
    let selected = directory.join("selected.txt");
    let neighbor = directory.join("selected.txt.neighbor");
    fs::write(&selected, "OPS_SYNTHETIC_PROTECTED_TABLE\n").expect("write selected fixture");
    fs::write(&neighbor, "OPS_SYNTHETIC_CODE_FORMULA\n").expect("write neighbor fixture");

    let output = Command::new(env!(concat!(
        "CARGO_BIN_EXE_protected_content_lint_",
        "cli"
    )))
    .arg("--provenance-mode")
    .arg("external")
    .arg(&selected)
    .output()
    .expect("run lint CLI");
    let canonical = selected.canonicalize().expect("canonical selected path");
    fs::remove_dir_all(&directory).expect("remove fixture directory");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let payload = String::from_utf8(output.stdout).expect("UTF-8 JSON output");
    assert!(payload.contains("\"target_count\":1,\"scanned_target_count\":1"));
    assert!(payload.contains("\"skipped_private_target_count\":0"));
    assert!(payload.contains("\"finding_count\":1"));
    assert!(payload.contains(&format!("\"path\":\"{}\"", canonical.display())));
    assert!(payload.contains("\"excerpt\":\"OPS_SYNTHETIC_PROTECTED_TABLE\""));
    assert!(!payload.contains("selected.txt.neighbor"));
}

#[cfg(unix)]
#[test]
fn cli_refuses_an_explicit_file_it_cannot_scan_without_rewriting_identity() {
    let directory =
        std::env::temp_dir().join(format!("ops-lint-cli-backslash-{}", std::process::id()));
    fs::create_dir_all(&directory).expect("create fixture directory");
    let selected = directory.join("literal\\backslash.txt");
    fs::write(&selected, "OPS_SYNTHETIC_PROTECTED_TABLE\n").expect("write selected fixture");

    let output = Command::new(env!(concat!(
        "CARGO_BIN_EXE_protected_content_lint_",
        "cli"
    )))
    .arg("--provenance-mode")
    .arg("external")
    .arg(&selected)
    .output()
    .expect("run lint CLI");
    fs::remove_dir_all(&directory).expect("remove fixture directory");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("cannot be represented in lint scope"));
}
