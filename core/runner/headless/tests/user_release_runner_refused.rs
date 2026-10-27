//! `user_release_runner_refused` (W-18b slice (r2c); DEL-08-01 Design
//! `report-user-release.md` §2.8 refusal 1, §6): the headless runner's
//! `export-results` packages no user-released report. The request is the
//! committed derivation of the first user-release vector
//! (`fixtures/reports/invented/user_release_runner_refusal_request.json`): a
//! coherent, pressure-free, desktop-origin request with the user-released
//! profile and an invented record, which the producer's one admitting entry
//! admits (`report_package/tests/user_release_package.rs`). Given as the
//! runner's payload, with and without `--explicit-local-private-intent`, it
//! is refused, as is each user-release marker alone; the runner exits 1,
//! carries no `report_package` and writes no `--output` file. With the
//! intent its output names the refusal and the marker; without it the
//! runner's existing route control withholds the payload, the diagnostic
//! with it. The runner's
//! `del1005_*` witnesses are unchanged. Standard claim fence applies (claims
//! taxonomy per DEC-081).

use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const SUCCESS_INPUT: &str = "validation/witness/inputs/del1005_export_results_success_input.json";
const REQUEST: &str = "fixtures/reports/invented/user_release_runner_refusal_request.json";
const CODE: &str = "REPORT-USER-RELEASE-RUNNER-NOT-ADMITTED";

fn project(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)
}

fn read(path: &str) -> Value {
    serde_json::from_slice(&std::fs::read(project(path)).unwrap()).unwrap()
}

/// Runs `export-results` on `payload` (as the success witness's
/// `export_results`), with `--output` into a fresh directory; returns the
/// exit code, stdout and whether the output file exists.
fn run(payload: &Value, intent: bool, label: &str) -> (Option<i32>, String, bool) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let dir = std::env::temp_dir().join(format!("swbpipe-user-release-runner-{label}-{intent}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let output = dir.join("must-not-exist.json");
    let mut input = read(SUCCESS_INPUT);
    input["export_results"] = payload.clone();
    let mut args = vec!["export-results".to_string(), "--output".to_string(), output.to_string_lossy().into_owned()];
    if intent {
        args.push("--explicit-local-private-intent".to_string());
    }
    let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
    let out = child.wait_with_output().unwrap();
    let exists = output.exists();
    let _ = std::fs::remove_dir_all(&dir);
    (out.status.code(), String::from_utf8(out.stdout).unwrap(), exists)
}

#[test]
fn user_release_runner_refused() {
    let coherent = read(REQUEST);
    assert_eq!(coherent["export_profile_id"], "desktop_user_released_report_package_1");
    let mut profile_only = coherent.clone();
    profile_only["report"]["report_context"].as_object_mut().unwrap().remove("user_release");
    profile_only["report"]["report_context"]["export_context"] = json!("local_private");
    // A record under the runner profile with the runner's attested origin:
    // the success witness's own runner payload with the record added.
    let mut runner_record = read(SUCCESS_INPUT)["export_results"].clone();
    assert_eq!(runner_record["export_profile_id"], "runner_local_private_report_package_1");
    runner_record["report"]["report_context"]["user_release"] = coherent["report"]["report_context"]["user_release"].clone();
    let mut desktop_context = coherent.clone();
    desktop_context["export_profile_id"] = json!("desktop_local_private_report_package_1");
    desktop_context["report"]["report_context"].as_object_mut().unwrap().remove("user_release");
    for (label, payload, marker) in [
        ("coherent", &coherent, "export_profile_id"),
        ("profile-only", &profile_only, "export_profile_id"),
        ("runner-record", &runner_record, "report.report_context.user_release"),
        ("desktop-context", &desktop_context, "report.report_context.export_context"),
    ] {
        for intent in [true, false] {
            let (code, stdout, exists) = run(payload, intent, label);
            assert_eq!(code, Some(1), "{label} intent {intent}: {stdout}");
            assert!(!exists, "{label} intent {intent}: no output file");
            let output: Value = serde_json::from_str(&stdout).unwrap_or_else(|error| panic!("{label}: {error}: {stdout}"));
            assert!(output.pointer("/payload/report_package").map_or(true, Value::is_null), "{label} intent {intent}: {stdout}");
            assert!(output.get("report_package").map_or(true, Value::is_null), "{label} intent {intent}");
            // The message never carries the member's value.
            assert!(!stdout.contains("user_statement_of_data_rights_only"), "{label}");
            if intent {
                let message = format!(
                    "{CODE}: the headless runner packages no user-released report; only the desktop's route DREP-PACKAGE-USER-RELEASE-010 builds one (marker: {marker})"
                );
                assert!(stdout.contains(&message), "{label}: {stdout}");
            } else {
                // Without the intent the runner's existing route control
                // withholds the whole payload, the refusal's diagnostic with
                // it, and records the missing intent (`redaction_binding`).
                assert!(output["payload"].is_null(), "{label}");
                assert!(stdout.contains("LOCAL_PRIVATE_INTENT_REQUIRED"), "{label}");
            }
        }
    }
}

/// Also asserted by the reach scan's design (DEL-08-01 Design
/// `report-user-release.md` §2.8): no source of the headless runner, its
/// library, binaries or tests, names the producer's admitting entry, its
/// derivation check, its proof type or the desktop's bridge function. The
/// names are assembled here so this file holds none of them.
#[test]
fn no_runner_source_names_the_admitting_entry() {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }
    /// The identifiers of Rust source outside comments and string literals.
    fn identifiers(text: &str) -> std::collections::BTreeSet<String> {
        let chars: Vec<char> = text.chars().collect();
        let (mut out, mut index) = (std::collections::BTreeSet::new(), 0);
        while index < chars.len() {
            match chars[index] {
                '/' if chars.get(index + 1) == Some(&'/') => {
                    while index < chars.len() && chars[index] != '\n' {
                        index += 1;
                    }
                }
                '/' if chars.get(index + 1) == Some(&'*') => {
                    index += 2;
                    while index + 1 < chars.len() && !(chars[index] == '*' && chars[index + 1] == '/') {
                        index += 1;
                    }
                    index += 2;
                }
                // A char literal (`'"'`, `'\\''`); a lifetime is read as identifier text.
                '\'' if chars.get(index + 2) == Some(&'\'') => index += 3,
                '\'' if chars.get(index + 1) == Some(&'\\') && chars.get(index + 3) == Some(&'\'') => index += 4,
                '"' => {
                    index += 1;
                    while index < chars.len() && chars[index] != '"' {
                        index += if chars[index] == '\\' { 2 } else { 1 };
                    }
                    index += 1;
                }
                c if c.is_alphabetic() || c == '_' => {
                    let start = index;
                    while index < chars.len() && (chars[index].is_alphanumeric() || chars[index] == '_') {
                        index += 1;
                    }
                    out.insert(chars[start..index].iter().collect());
                }
                _ => index += 1,
            }
        }
        out
    }
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk(&crate_dir.join("src"), &mut files);
    walk(&crate_dir.join("tests"), &mut files);
    assert!(files.len() > 5, "{files:?}");
    let names = [
        ["assemble", "derived"].join("_"),
        ["check", "derived"].join("_"),
        ["assemble", "user", "released", "request"].join("_"),
        ["Derived", "User", "Release"].concat(),
    ];
    for file in files {
        let found = identifiers(&std::fs::read_to_string(&file).unwrap());
        for name in &names {
            assert!(!found.contains(name), "{} names {name}", file.display());
        }
    }
}
