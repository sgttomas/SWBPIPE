//! The user-released request derivation (W-18b slice (r2b); DEL-08-01 Design
//! `report-user-release.md` §2.6, §6) against the shared vectors
//! `fixtures/reports/invented/user_release_derivation_vectors.json`, which the
//! desktop's builders write (`apps/desktop/src/features/report/userReleaseDerivation.parity.test.ts`).
//! Each vector's inputs are the model and the producer's bytes (committed
//! fixtures, named by the vector), the solver mode, the input manifest, the
//! solve's transient run record, the software identity and an invented
//! record; the Rust derivation must give the desktop's request exactly. No
//! route calls the derivation yet; nothing here records a user release.
//! Standard claim fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_report_package::bindings::{check_report_result_refs, expected_report_result_rows, BINDING_MISMATCH};
use open_pipe_stress_report_package::user_release::{
    check_matches_derivation, derive, derive_input_manifest, first_difference, SoftwareIdentity, UserReleaseBasis, UserReleaseDerivation,
    BASIS_INVALID, CONTENT_NOT_DERIVED, EXACT_ROUTE_NOT_ADMITTED, USER_RELEASED_CONTEXT, USER_RELEASED_DESKTOP_PROFILE,
};
use open_pipe_stress_result_export::derivative::digest;
use serde_json::{json, Value};

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/user_release_derivation_vectors.json");
const MODEL_FIXTURE: (&str, &str) = (
    "fixtures/model_operations/precision_connected_ui_model.json",
    include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json"),
);
const SOURCE_FIXTURES: [(&str, &str); 2] = [
    ("fixtures/results/preview_physics_connected_sparse.json", include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json")),
    ("fixtures/results/preview_physics_connected_dense.json", include_str!("../../../../fixtures/results/preview_physics_connected_dense.json")),
];

struct Case {
    vector: Value,
    model: Value,
    source: Value,
    software: SoftwareIdentity,
}

impl Case {
    fn basis(&self) -> UserReleaseBasis<'_> {
        UserReleaseBasis {
            model: &self.model,
            source: &self.source,
            solver_mode: self.vector["solver_mode"].as_str().unwrap(),
            run_record: &self.vector["run_record"],
            software: &self.software,
            record: &self.vector["user_release"],
        }
    }

    fn derived(&self) -> UserReleaseDerivation {
        derive(&self.basis()).unwrap_or_else(|error| panic!("{}: {error}", self.vector["name"]))
    }

    /// The desktop's whole request, rebuilt from the vector's members and the
    /// derived result document (bound by the vector's document digest).
    fn expected_request(&self, document: &Value) -> Value {
        let expected = &self.vector["expected"];
        let mut request = expected["top_level"].clone();
        request["report"] = expected["report"].clone();
        request["audit_manifest"] = expected["audit_manifest"].clone();
        request["result_documents"] = json!([document]);
        request["source_results"] = json!([self.source]);
        request
    }
}

fn fixture(path: &str) -> Value {
    let text = if path == MODEL_FIXTURE.0 {
        MODEL_FIXTURE.1
    } else {
        SOURCE_FIXTURES.iter().find(|(name, _)| *name == path).unwrap_or_else(|| panic!("no committed fixture {path}")).1
    };
    serde_json::from_str(text).unwrap()
}

fn cases() -> Vec<Case> {
    let file: Value = serde_json::from_str(VECTORS).unwrap();
    assert_eq!(file["schema_version"], "1.0.0");
    file["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|vector| Case {
            model: fixture(vector["model_fixture"].as_str().unwrap()),
            source: fixture(vector["source_fixture"].as_str().unwrap()),
            software: SoftwareIdentity {
                name: vector["software"]["name"].as_str().unwrap().to_string(),
                version: vector["software"]["version"].as_str().unwrap().to_string(),
            },
            vector: vector.clone(),
        })
        .collect()
}

#[test]
fn derivation_vectors_match_the_desktop_builders() {
    let cases = cases();
    assert_eq!(cases.len(), 2, "one vector per desktop solver mode");
    for case in &cases {
        let name = case.vector["name"].as_str().unwrap();
        assert!(!case.vector["reason"].as_str().unwrap().is_empty(), "{name}: every vector states its reason");
        let expected = &case.vector["expected"];
        let derived = case.derived();
        // The input manifest is the solve's.
        assert_eq!(first_difference(&case.vector["input_manifest"], &derived.input_manifest), None, "{name}: input manifest");
        assert_eq!(derived.input_manifest_sha256, expected["input_manifest_sha256"].as_str().unwrap(), "{name}");
        assert_eq!(derived.input_manifest_ref, expected["input_manifest_ref"].as_str().unwrap(), "{name}");
        // The result document is the desktop's Export Result JSON document.
        let document = &derived.request["result_documents"][0];
        assert_eq!(digest(document).unwrap(), expected["result_document_sha256"].as_str().unwrap(), "{name}: result document");
        assert_eq!(
            document["result_envelope"]["reproducibility"]["derivative_hash"]["value"],
            expected["result_document_derivative_sha256"],
            "{name}: derivative hash"
        );
        // The minimal renderer input, the audit manifest and the top-level members, member by member.
        assert_eq!(first_difference(&expected["report"], &derived.request["report"]), None, "{name}: report");
        assert_eq!(first_difference(&expected["audit_manifest"], &derived.request["audit_manifest"]), None, "{name}: audit manifest");
        for (key, value) in expected["top_level"].as_object().unwrap() {
            assert_eq!(first_difference(value, &derived.request[key]), None, "{name}: {key}");
        }
        // The whole request, canonically.
        assert_eq!(digest(&derived.request).unwrap(), expected["request_sha256"].as_str().unwrap(), "{name}: request");
        let request = case.expected_request(document);
        assert_eq!(check_matches_derivation(&derived, &case.vector["input_manifest"], &request), Ok(()), "{name}");
        // Scope R's member contract (§2.1).
        let request = &derived.request;
        assert_eq!(request["export_profile_id"], USER_RELEASED_DESKTOP_PROFILE);
        assert_eq!(request["report"]["report_context"]["export_context"], USER_RELEASED_CONTEXT);
        assert_eq!(request["report"]["report_context"]["user_release"], case.vector["user_release"]);
        assert_eq!(request["report"]["report_context"]["generated_at"], Value::Null);
        assert_eq!(request["state_comparison_handoff_records"], json!([]));
        assert_eq!(request["rule_check_aggregate"], Value::Null);
        assert_eq!(request["audit_manifest"]["rule_pack_refs"], json!([]));
        assert_eq!(request["audit_manifest"]["assets"], json!([]));
        assert_eq!(request["report"]["calculation_report"]["rule_pack_refs"], json!([]));
        assert_eq!(request["report"]["calculation_report"]["model_input_summary"]["persistence_ref"]["ref_id"], "TBD");
        for absent in ["user_rule_checks", "exact_route", "library_references"] {
            assert!(request["report"].get(absent).is_none(), "{name}: {absent} is absent");
        }
        assert_eq!(request["report"]["report_sections"]["provenance_notes"].as_array().unwrap().len(), 1);
        assert_eq!(request["report"]["report_sections"]["user_supplied_values"], json!([]));
        assert_eq!(
            request["report"]["report_sections"]["diagnostics"].as_array().unwrap().len(),
            case.source["diagnostics"].as_array().unwrap().len(),
            "{name}: the solve's diagnostics only"
        );
        assert_eq!(request["report"]["result_rows"], json!(expected_report_result_rows(&case.source).unwrap()));
    }
}

/// Every member pointer of `value` down to `depth` levels, arrays by their
/// first and last element.
fn member_pointers(value: &Value, pointer: &str, depth: usize, out: &mut Vec<String>) {
    if depth == 0 {
        return;
    }
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let at = format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                out.push(at.clone());
                member_pointers(child, &at, depth - 1, out);
            }
        }
        Value::Array(items) => {
            let mut indices = vec![0, items.len().saturating_sub(1)];
            indices.dedup();
            for index in indices.into_iter().filter(|index| *index < items.len()) {
                let at = format!("{pointer}/{index}");
                out.push(at.clone());
                member_pointers(&items[index], &at, depth - 1, out);
            }
        }
        _ => {}
    }
}

/// A mutation of each member of a derived request is refused, naming its
/// pointer (§6): a changed value, and an extra member added only in the
/// request.
#[test]
fn a_mutation_of_each_member_is_refused_naming_its_pointer() {
    let case = &cases()[0];
    let derived = case.derived();
    let input_manifest = &case.vector["input_manifest"];
    let mut pointers = Vec::new();
    member_pointers(&derived.request, "", 5, &mut pointers);
    assert!(pointers.len() > 200, "{}", pointers.len());
    for pointer in &pointers {
        let mut request = derived.request.clone();
        let target = request.pointer_mut(pointer).unwrap();
        *target = match target {
            Value::String(text) => json!(format!("{text} (invented-change-7f3a)")),
            Value::Bool(flag) => json!(!*flag),
            Value::Number(number) => {
                let value = number.as_f64().unwrap();
                json!(value + value.abs().max(1.0))
            }
            Value::Null => json!("invented-change-7f3a"),
            Value::Array(items) => {
                let mut items = items.clone();
                items.push(json!("invented-change-7f3a"));
                Value::Array(items)
            }
            Value::Object(object) => {
                let mut object = object.clone();
                object.insert("invented_marker".to_string(), json!("invented-change-7f3a"));
                Value::Object(object)
            }
        };
        let refusal = check_matches_derivation(&derived, input_manifest, &request).unwrap_err();
        let named = refusal
            .strip_prefix(&format!("{CONTENT_NOT_DERIVED}: the request differs from its derivation at "))
            .unwrap_or_else(|| panic!("{pointer}: {refusal}"));
        assert!(named == pointer || named.starts_with(&format!("{pointer}/")), "{pointer}: named {named}");
        assert!(!refusal.contains("invented-change-7f3a"), "the refusal never quotes a value: {refusal}");
    }
    // The input manifest is compared first.
    let mut manifest = input_manifest.clone();
    manifest["external_assets"] = json!([{"asset_ref": "invented", "media_type": "text/plain", "checksum_sha256": "0".repeat(64)}]);
    let refusal = check_matches_derivation(&derived, &manifest, &json!({})).unwrap_err();
    assert_eq!(refusal, format!("{CONTENT_NOT_DERIVED}: the input manifest differs from its derivation at /external_assets/0"));
    let mut manifest = input_manifest.clone();
    manifest["solver_basis"]["settings"]["sparse_evidence_lane"] = json!(false);
    assert!(check_matches_derivation(&derived, &manifest, &derived.request).unwrap_err().ends_with("/solver_basis/settings/sparse_evidence_lane"));
    // An extra key in a source row's metadata, added only in the request (§6,
    // `user_release_unknown_annotation_refused`'s derivation half).
    let mut request = derived.request.clone();
    request["source_results"][0]["results"][0]["metadata"]["invented_annotation"] = json!("invented marker");
    assert!(check_matches_derivation(&derived, input_manifest, &request)
        .unwrap_err()
        .ends_with("/source_results/0/results/0/metadata/invented_annotation"));
}

/// `user_release_display_rule` (§6): the vector whose S has rows stored in
/// `Pa` and rows with more than six significant digits. The derived rows show
/// six significant digits and `MPa`, `conversion_performed` is true, and the
/// row checksums are the full-precision digests. A stored value changed in
/// its seventh significant digit leaves the displayed row unchanged, and the
/// derivation still refuses it, by the producer's `check_report_result_refs`
/// over the run record's row checksums.
#[test]
fn user_release_display_rule() {
    let cases = cases();
    let case = cases.iter().find(|case| case.vector["reason"].as_str().unwrap().starts_with("user_release_display_rule")).unwrap();
    let derived = case.derived();
    let report = &derived.request["report"];
    let rows = case.source["results"].as_array().unwrap();
    let pascal = rows.iter().position(|row| row["unit"] == "Pa").expect("a row stored in Pa");
    assert!(report["result_rows"][pascal]["quantity_display"].as_str().unwrap().ends_with(" MPa"));
    assert_eq!(report["calculation_report"]["model_input_summary"]["unit_display_summary"]["conversion_performed"], true);
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(report["calculation_report"]["result_export_refs"][index]["checksum"]["value"], digest(row).unwrap(), "row {index}");
    }
    let significant = |value: f64| {
        let text = format!("{:e}", value.abs());
        text.split('e').next().unwrap().replace('.', "").len()
    };
    // A seventh-significant-digit change of a stored value that keeps its displayed row.
    let seventh_digit_change = |index: usize| -> Option<Value> {
        let value = rows[index]["value"].as_f64().filter(|value| *value != 0.0 && significant(*value) > 6)?;
        let step = 10f64.powi(value.abs().log10().floor() as i32 - 6);
        [value + step, value - step].into_iter().find_map(|changed| {
            let mut source = case.source.clone();
            source["results"][index]["value"] = json!(changed);
            let kept = expected_report_result_rows(&source).unwrap()[index] == expected_report_result_rows(&case.source).unwrap()[index];
            (kept && source["results"][index]["value"] != case.source["results"][index]["value"]).then_some(source)
        })
    };
    let changed: Vec<(usize, Value)> = (0..rows.len()).filter_map(|index| seventh_digit_change(index).map(|source| (index, source))).collect();
    assert!(!changed.is_empty(), "a row with more than six significant digits");
    let mut by_row_checksum = 0;
    for (index, source) in &changed {
        // The producer's row-checksum check refuses the derived report over the changed source.
        let refusal = check_report_result_refs(report, source).unwrap_err();
        assert!(refusal.starts_with(BINDING_MISMATCH) && refusal.contains("report row checksum"), "row {index}: {refusal}");
        // The derivation refuses it: by that check, or earlier, when the changed row is a
        // value the result's own reader binds (a headline's source row).
        let refusal = derive(&UserReleaseBasis { source, ..case.basis() }).unwrap_err();
        if refusal.starts_with(BINDING_MISMATCH) && refusal.contains("report row checksum") {
            by_row_checksum += 1;
        }
    }
    assert!(by_row_checksum > 0, "some seventh-digit change reaches the row-checksum check");
}

/// The derivation's own refusals: an exact-route source, a run record with a
/// rule-check aggregate, a rule-pack exclusion diagnostic or another input
/// manifest, and a solver mode no desktop solve records.
#[test]
fn the_derivation_refuses_a_basis_outside_scope_r() {
    let cases = cases();
    let case = &cases[0];
    for exact in ["openpipestress.result_semantics/0.3.0/pressure-2", "openpipestress.result_semantics/0.3.0/pressure-source-1"] {
        let mut source = case.source.clone();
        source["producer"]["semantic_contract_id"] = json!(exact);
        let refusal = derive(&UserReleaseBasis { source: &source, ..case.basis() }).unwrap_err();
        assert!(refusal.starts_with(EXACT_ROUTE_NOT_ADMITTED), "{refusal}");
    }
    let mut record = case.vector["run_record"].clone();
    let statuses = record["analysis_run"]["analysis_status"].as_array_mut().unwrap();
    for status in statuses.iter_mut() {
        if status == "RULE_INPUTS_INCOMPLETE" {
            *status = json!("USER_RULE_CHECKED");
        }
    }
    let refusal = derive(&UserReleaseBasis { run_record: &record, ..case.basis() }).unwrap_err();
    assert!(refusal.starts_with(BASIS_INVALID) && refusal.contains("rule status"), "{refusal}");
    let mut record = case.vector["run_record"].clone();
    record["analysis_run"]["diagnostics"].as_array_mut().unwrap().push(json!({"source_annotation": {"id": "diagnostic:rule-pack-ref:invented@1", "code": "MODEL-RULE-PACK-REF-UNRESOLVED"}}));
    assert!(derive(&UserReleaseBasis { run_record: &record, ..case.basis() }).unwrap_err().contains("rule-pack reference exclusion"));
    let other = &cases[1];
    assert!(derive(&UserReleaseBasis { run_record: &other.vector["run_record"], ..case.basis() }).unwrap_err().contains("does not name the derived input manifest"));
    assert!(derive(&UserReleaseBasis { solver_mode: "invented_mode", ..case.basis() }).unwrap_err().starts_with(BASIS_INVALID));
    // The manifest is the model's whatever the record: built from M, S's producer and m.
    let manifest = derive_input_manifest(&case.model, &case.source, "dense_scrutiny").unwrap();
    assert_eq!(first_difference(&other.vector["input_manifest"], &manifest), None);
}
