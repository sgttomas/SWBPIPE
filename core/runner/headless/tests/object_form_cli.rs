//! Object form on the headless runner's package path (DEL-08-01 Design
//! `report-package-current-results.md` §2.6; found in Codex review round 3 of
//! #1648): `export-results` on the committed DEL-10-05 witness input with a
//! report provenance record written positionally (a quarantined one) refuses
//! with `REPORT-PACKAGE-REQUEST-POSITIONAL` and writes no package; its stdout
//! and stderr echo no record text. Invented data only.
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

const WITNESS: &str = include_str!("../../../../validation/witness/inputs/del1005_export_results_success_input.json");
const PROVENANCE: [&str; 8] = [
    "source_name", "source_location", "source_license", "contributor", "contributor_certification", "redistribution_status", "review_status",
    "privacy_classification",
];

#[test]
fn a_positional_report_record_refuses_the_runner_package() {
    for pointer in ["/calculation_report/model_input_summary/provenance", "/report_sections/provenance_notes/0"] {
        let mut input: Value = serde_json::from_str(WITNESS).unwrap();
        let record = input["export_results"]["report"].pointer_mut(pointer).unwrap();
        record["review_status"] = json!("quarantined");
        let source_name = record["source_name"].as_str().unwrap().to_string();
        *record = Value::Array(PROVENANCE.iter().map(|name| record[*name].clone()).collect());
        let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
            .args(["export-results", "--explicit-local-private-intent"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        let stdout = String::from_utf8(out.stdout).unwrap();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert_ne!(out.status.code(), Some(0), "{pointer}: {stdout}");
        let output: Value = serde_json::from_str(&stdout).unwrap_or_else(|error| panic!("{pointer}: {error}; {stderr}"));
        assert!(output.pointer("/payload/report_package").map_or(true, Value::is_null), "{pointer}: {stdout}");
        assert!(stdout.contains("REPORT-PACKAGE-REQUEST-POSITIONAL"), "{pointer}: {stdout}");
        assert!(!stdout.contains(&source_name) && !stderr.contains(&source_name), "{pointer}");
    }
}
