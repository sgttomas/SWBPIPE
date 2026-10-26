//! W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md` §6.2),
//! Codex review round 3 of #1639: the headless runner's `export-results` on a
//! request whose report carries quarantined FR-REP-003 records with authored
//! ids (paths, private text) refuses the package, and neither its structured
//! output nor its stderr carries any of those ids. Invented data only.
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

const TEXTS: [&str; 3] = ["/Users/invented-user/private-datasheets/steel-a.pdf", "C:/x.pdf", "Invented Private Client Project"];

const WITNESS: &str = include_str!("../../../../validation/witness/inputs/del1005_export_results_success_input.json");

fn witness() -> Value {
    serde_json::from_str(WITNESS).unwrap()
}

#[test]
fn a_refused_export_names_no_authored_id() {
    let mut input = witness();
    let record = json!({
        "source_name": "Invented material datasheet", "source_location": "invented-datasheet-shelf", "source_license": "invented-license",
        "contributor": "invented contributor", "contributor_certification": "invented certification",
        "redistribution_status": "private_only", "review_status": "quarantined", "privacy_classification": "private_project_data"
    });
    let report = &mut input["export_results"]["report"];
    for (kind, id) in [("material", TEXTS[0]), ("section", TEXTS[1]), ("component", TEXTS[2]), ("support", TEXTS[0])] {
        report["report_sections"]["user_supplied_values"].as_array_mut().unwrap().push(json!({
            "value_id": format!("{kind}-provenance:{kind}:{id}"), "value_category": "report_metadata",
            "source": {"ref_type": kind, "ref_id": format!("{kind}:{id}")}, "provenance": record.clone(),
            "privacy_classification": "private_project_data", "required_for": ["reporting"],
            "review_status": "quarantined", "missing_data_finding": false
        }));
    }
    report["library_references"] = json!([{
        "reference_id": format!("library-ref:{}", TEXTS[1]), "library_kind": "material", "library_id": TEXTS[2],
        "library_version": "not recorded", "record_id": TEXTS[0], "entity_refs": [format!("material:{}", TEXTS[0])], "provenance": record
    }]);
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
    assert_ne!(out.status.code(), Some(0), "{stdout}");
    let output: Value = serde_json::from_str(&stdout).unwrap_or_else(|error| panic!("stdout is not JSON: {error}; {stderr}"));
    assert!(output.get("report_package").map_or(true, Value::is_null), "{stdout}");
    assert!(stdout.contains("HEADLESS_RUNNER_EXPORT_RESULTS_PACKAGE_BLOCKED"), "{stdout}");
    assert!(stdout.contains("REPORT_PROVENANCE_DISPOSITION_BLOCKED"), "{stdout}");
    for private in TEXTS {
        assert!(!stdout.contains(private), "{private} in stdout: {stdout}");
        assert!(!stderr.contains(private), "{private} in stderr: {stderr}");
    }
}

/// The runner's payload refusals never echo a request value: a typed path
/// given as an invalid disposition value in the report, or as a typed request
/// member, is refused by path and kind only.
#[test]
fn a_payload_refusal_echoes_no_value() {
    let typed = TEXTS[0];
    let cases: [(fn(&mut Value, &str), &str); 2] = [
        (
            |input, typed| input["export_results"]["report"]["report_sections"]["provenance_notes"][0]["review_status"] = json!(typed),
            "REPORT-PACKAGE-REPORT-DTO-INVALID: unknown variant at report.report_sections.provenance_notes[0].review_status",
        ),
        (|input, typed| input["export_results"]["source_model_ref"] = json!(typed), "REPORT-PACKAGE-WIRE-INCOMPLETE: invalid type at source_model_ref"),
    ];
    for (edit, expected) in cases {
        let mut input = witness();
        edit(&mut input, typed);
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
        assert_ne!(out.status.code(), Some(0));
        assert!(stdout.contains(expected), "{expected}: {stdout}");
        assert!(!stdout.contains(typed) && !String::from_utf8_lossy(&out.stderr).contains(typed), "{stdout}");
    }
}
