//! The exact-route sections' projection and the dormant item 9 (W-11,
//! DEL-08-01 Design `exact-route-report-sections.md` §3.5 and §7, slice (a)),
//! against the shared vectors `fixtures/reports/invented/exact_report_sections_parity.json`,
//! which this test writes from the projection and fails on when stale
//! (`OPENPIPESTRESS_WRITE_EXACT_REPORT_SECTIONS_PARITY=<path>` writes them).
//! The desktop builder is checked against the same vectors in slice (b).
//! Item 0 refuses exact identities on every production entry point, so item 9
//! is exercised here through its public function. The sources are the
//! committed exact corpora; the document and audit bindings are invented.
//! Standard claim fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_report_package::bindings::{DESKTOP_PROFILE, RUNNER_PROFILE};
use open_pipe_stress_report_package::document_form::ADMITTED_SEMANTIC_CONTRACTS;
use open_pipe_stress_report_package::exact_sections::{
    check_exact_sections, check_runner_profile, expected_exact_limitations, expected_exact_route_core,
    expected_exact_route_section, EXACT_HEADLINE_UNREADABLE, EXACT_LIMITATION_UNREGISTERED, EXACT_RUNNER_PROFILE,
    EXACT_SECTIONS_MISMATCH, WITHHELD_HEADLINE,
};
use open_pipe_stress_report_renderer::exact_statements::{exact_statement_for, producer_sha256};
use open_pipe_stress_result_export::derivative::digest;
use open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_ID;
use serde_json::{json, Value};

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/exact_report_sections_parity.json");
const WRITE_ENV: &str = "OPENPIPESTRESS_WRITE_EXACT_REPORT_SECTIONS_PARITY";
const GENERATOR: &str = "core/reporting/report_package/tests/exact_sections_parity.rs::exact_sections_parity_vectors_are_current";
const COVERAGE: &str = include_str!("../../../../fixtures/reports/exact_route_limitations.json");
const OMITTED_SENTENCE: &str =
    "No code compliance or professional acceptance is produced, and actual numerical admission remains separate.";

const SOURCES: [(&str, &str); 6] = [
    ("fixtures/results/pressure_v3_straight_reader_corpus.json", include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json")),
    ("fixtures/results/pressure_v3_arc_reader_corpus.json", include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json")),
    ("fixtures/results/pressure_v3_connector_reader_corpus.json", include_str!("../../../../fixtures/results/pressure_v3_connector_reader_corpus.json")),
    ("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")),
    ("fixtures/product_preview/pressure_source/load_reference_source/fields-sparse_interactive.raw.json", include_str!("../../../../fixtures/product_preview/pressure_source/load_reference_source/fields-sparse_interactive.raw.json")),
    ("fixtures/results/preview_physics_connected_sparse.json", include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json")),
];

/// Invented binding inputs: the document's and the audit's digests and the
/// custody facts the shared producer checks for form only.
const MODEL_SHA256: &str = "a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1";
const DERIVATIVE_SHA256: &str = "b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2";
const BYTES_SHA256: &str = "c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3";
const NATIVE_JOB_ID: &str = "invented-solve-job-1";
const INPUT_MANIFEST_REF: &str = "invented-input-manifest";

fn source_text(path: &str) -> &'static str {
    SOURCES.iter().find(|(name, _)| *name == path).unwrap_or_else(|| panic!("no source {path}")).1
}

/// A vector's source: the named file at the pointer, with its edits applied.
fn resolve(spec: &Value) -> Value {
    let document: Value = serde_json::from_str(source_text(spec["path"].as_str().unwrap())).unwrap();
    let mut source = document.pointer(spec["pointer"].as_str().unwrap()).unwrap().clone();
    for edit in spec["edits"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let pointer = edit["pointer"].as_str().unwrap();
        match edit["op"].as_str().unwrap() {
            "set" => *source.pointer_mut(pointer).unwrap() = edit["value"].clone(),
            "remove" => {
                let (parent, key) = pointer.rsplit_once('/').unwrap();
                source.pointer_mut(parent).unwrap().as_object_mut().unwrap().remove(key).unwrap();
            }
            "append" => source.pointer_mut(pointer).unwrap().as_array_mut().unwrap().push(edit["value"].clone()),
            "replace_text" => {
                let at = source.pointer_mut(pointer).unwrap();
                let text = at.as_str().unwrap();
                let from = edit["from"].as_str().unwrap();
                assert_eq!(text.matches(from).count(), 1, "{from}");
                *at = Value::from(text.replace(from, edit["to"].as_str().unwrap()));
            }
            op => panic!("unknown edit {op}"),
        }
    }
    source
}

fn spec(path: &str, pointer: &str, edits: Value) -> Value {
    json!({"path": path, "pointer": pointer, "edits": edits})
}

/// The invented document fragment item 9 reads, for `source`.
fn invented_document(source: &Value) -> Value {
    let run = source["run_id"].as_str().unwrap();
    json!({"result_envelope": {
        "producer": {"semantic_contract_id": source["producer"]["semantic_contract_id"]},
        "run_ref": {"ref_type": "analysis_run", "ref_id": run},
        "envelope_id": format!("result-envelope:{run}"),
        "reproducibility": {"model_hash": {"value": MODEL_SHA256}, "derivative_hash": {"value": DERIVATIVE_SHA256}}
    }})
}

fn invented_audit() -> audit::AuditManifest {
    audit::AuditManifest {
        manifest_id: INPUT_MANIFEST_REF.to_string(),
        model_hash: None,
        input_manifest_hash: Some(audit::hash_canonical_json(
            audit::PayloadKind::InputManifestJson,
            INPUT_MANIFEST_REF,
            &audit::CanonicalJson::object([(
                "input_manifest_id",
                audit::CanonicalJson::String(INPUT_MANIFEST_REF.to_string()),
            )]),
        )),
        solver_version: audit::SolverVersionStamp {
            solver_name: "invented".to_string(),
            solver_version: "0.0.0".to_string(),
            solver_build_ref: "invented".to_string(),
        },
        unit_system_ref: "invented-si".to_string(),
        rule_pack_refs: vec![],
        assets: vec![],
        professional_boundary: audit::ProfessionalBoundary::project_default(),
    }
}

fn shapes() -> Vec<(&'static str, Value)> {
    let straight = "fixtures/results/pressure_v3_straight_reader_corpus.json";
    let arc = "fixtures/results/pressure_v3_arc_reader_corpus.json";
    let connector = "fixtures/results/pressure_v3_connector_reader_corpus.json";
    vec![
        ("pressure_2_straight", spec(straight, "/cases/0/envelope", json!([]))),
        ("pressure_2_load_reference_states", spec(straight, "/cases/2/envelope", json!([]))),
        ("pressure_2_realized_bend", spec(arc, "/cases/0/envelope", json!([]))),
        ("pressure_2_realized_bend_load_reference_states", spec(arc, "/cases/1/envelope", json!([]))),
        ("pressure_2_realized_bend_connector", spec(arc, "/cases/3/envelope", json!([]))),
        ("pressure_2_connector", spec(connector, "/cases/0/envelope", json!([]))),
        ("pressure_2_connector_load_reference_states", spec(connector, "/cases/1/envelope", json!([]))),
        ("pressure_source_1", spec("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", "", json!([]))),
        ("pressure_source_1_load_reference_states", spec("fixtures/product_preview/pressure_source/load_reference_source/fields-sparse_interactive.raw.json", "", json!([]))),
        (
            "pressure_2_straight_reduced_headline_withheld",
            spec(straight, "/cases/0/envelope", json!([{"op": "set", "pointer": "/summary/max_open_formula_stress_reduced_section", "value": null}])),
        ),
    ]
}

fn refusals() -> Vec<(&'static str, Value, &'static str)> {
    let straight = "fixtures/results/pressure_v3_straight_reader_corpus.json";
    vec![
        (
            "an_invented_unknown_limitation",
            spec(straight, "/cases/0/envelope", json!([{"op": "append", "pointer": "/formulation_basis/limitations", "value": "An invented limitation the producer never published."}])),
            EXACT_LIMITATION_UNREGISTERED,
        ),
        (
            "s9_with_one_word_changed",
            spec(straight, "/cases/0/envelope", json!([{"op": "replace_text", "pointer": "/formulation_basis/limitations/7", "from": "ordinary structural recovery", "to": "ordinary structural response"}])),
            EXACT_LIMITATION_UNREGISTERED,
        ),
        (
            "s9_without_its_last_sentence",
            spec(straight, "/cases/0/envelope", json!([{"op": "replace_text", "pointer": "/formulation_basis/limitations/7", "from": format!(" {OMITTED_SENTENCE}"), "to": ""}])),
            EXACT_LIMITATION_UNREGISTERED,
        ),
        (
            "a_missing_headline_key",
            spec(straight, "/cases/0/envelope", json!([{"op": "remove", "pointer": "/summary/max_open_formula_stress_nominal_section"}])),
            EXACT_HEADLINE_UNREADABLE,
        ),
        (
            "a_malformed_headline",
            spec(straight, "/cases/0/envelope", json!([{"op": "set", "pointer": "/summary/max_open_formula_stress_reduced_section/value", "value": "41"}])),
            EXACT_HEADLINE_UNREADABLE,
        ),
        ("a_pressure_free_source", spec("fixtures/results/preview_physics_connected_sparse.json", "", json!([])), EXACT_SECTIONS_MISMATCH),
    ]
}

/// The projection of one source: the core, the whole section under the
/// invented bindings, and the limitation entries.
fn project(source: &Value) -> Result<Value, String> {
    let core = expected_exact_route_core(source)?;
    let document = invented_document(source);
    let section = expected_exact_route_section(source, &document, &invented_audit(), NATIVE_JOB_ID, BYTES_SHA256)?;
    let limitations = expected_exact_limitations(source, &core.run_ref)?;
    Ok(json!({"exact_route_core": core, "exact_route": section, "limitations": limitations}))
}

fn first_code(error: &str) -> &str {
    error.split(':').next().unwrap()
}

fn vectors_document() -> Value {
    let binding_inputs = json!({
        "document_model_sha256": MODEL_SHA256,
        "document_derivative_sha256": DERIVATIVE_SHA256,
        "audit_input_manifest_ref": INPUT_MANIFEST_REF,
        "audit_input_manifest_sha256": invented_audit().input_manifest_hash.unwrap().value,
        "native_job_id": NATIVE_JOB_ID,
        "received_bytes_sha256": BYTES_SHA256,
    });
    let vectors: Vec<Value> = shapes()
        .into_iter()
        .map(|(name, spec)| {
            let expected = project(&resolve(&spec)).unwrap_or_else(|error| panic!("{name}: {error}"));
            json!({"name": name, "source": spec, "expected": expected})
        })
        .collect();
    let refusals: Vec<Value> = refusals()
        .into_iter()
        .map(|(name, spec, code)| {
            let error = project(&resolve(&spec)).err().unwrap_or_else(|| panic!("{name} projected"));
            assert_eq!(first_code(&error), code, "{name}: {error}");
            json!({"name": name, "source": spec, "code": code})
        })
        .collect();
    json!({
        "schema_version": "1.0.0",
        "description": "Shared parity vectors for the exact-route report sections (DEL-08-01 Design exact-route-report-sections.md sections 2.4 and 3.2-3.5): each vector names a committed source (path, JSON pointer, edits applied in order) and the expected exact_route_core, exact_route (with the invented binding_inputs as the document's, the audit's and the custody values) and exact limitation entries (without the effect and provenance they share with the technical-preview entry). report_package::exact_sections is checked against them by core/reporting/report_package/tests/exact_sections_parity.rs; the desktop builder is checked against them in slice (b). Edits: set, remove, append and replace_text at a JSON pointer. admitted_semantic_contracts is document_form::ADMITTED_SEMANTIC_CONTRACTS, which the desktop's REPORT_PACKAGE_ADMITTED_CONTRACT_IDS must equal. Invented bindings only.",
        "generator": GENERATOR,
        "admitted_semantic_contracts": ADMITTED_SEMANTIC_CONTRACTS,
        "binding_inputs": binding_inputs,
        "vectors": vectors,
        "refusals": refusals
    })
}

#[test]
fn exact_sections_parity_vectors_are_current() {
    let document = vectors_document();
    let text = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    if let Ok(target) = std::env::var(WRITE_ENV) {
        std::fs::write(target, &text).unwrap();
        return;
    }
    assert!(text == VECTORS, "the committed vectors are stale; regenerate them with {WRITE_ENV}=<path>");
}

/// Independent of the generator: each vector's expected texts are the
/// table's statements and the §3 texts, and the sources it reads.
#[test]
fn the_vectors_state_the_table_statements_and_the_section_texts() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    for vector in vectors["vectors"].as_array().unwrap() {
        let name = vector["name"].as_str().unwrap();
        let source = resolve(&vector["source"]);
        let expected = &vector["expected"];
        let run = source["run_id"].as_str().unwrap();
        let published = source["formulation_basis"]["limitations"].as_array().unwrap();
        let entries = expected["limitations"].as_array().unwrap();
        assert_eq!(entries.len(), published.len() + 1, "{name}");
        for (index, (entry, text)) in entries.iter().zip(published).enumerate() {
            assert_eq!(entry["limitation_id"], format!("limitation:{run}:exact:{}", index + 1));
            let statement = entry["statement"].as_str().unwrap();
            let text = text.as_str().unwrap();
            assert_eq!(Some(statement), exact_statement_for(&producer_sha256(text)));
            assert!(statement == text || format!("{statement} {OMITTED_SENTENCE}") == text, "{name} {index}");
        }
        let reference = entries.last().unwrap()["statement"].as_str().unwrap();
        assert!(reference.starts_with("The profile limitations of the semantic table openpipestress.result_semantics/0.3.0/"));
        assert!(reference.ends_with(") are bound by reference and are not restated in this report."));
        let route = &expected["exact_route"];
        assert_eq!(route["admission"]["received_result_sha256"], digest(&source).unwrap());
        assert_eq!(route["admission"]["model_sha256"], MODEL_SHA256);
        assert_eq!(route["semantic_contract_id"], source["producer"]["semantic_contract_id"]);
        let headlines = route["headlines"].as_array().unwrap();
        assert_eq!(headlines[0]["basis"], "nominal_section");
        assert_eq!(headlines[1]["basis"], "reduced_section");
        for (headline, key) in headlines.iter().zip(["max_open_formula_stress_nominal_section", "max_open_formula_stress_reduced_section"]) {
            let summary = &source["summary"][key];
            if summary.is_null() {
                assert_eq!(headline["value_display"], WITHHELD_HEADLINE);
                assert!(headline["location_ref"].is_null() && headline["source_ref"].is_null());
            } else {
                let (number, unit) = headline["value_display"].as_str().unwrap().split_once(' ').unwrap();
                assert_eq!(number.parse::<f64>().unwrap(), summary["value"].as_f64().unwrap());
                assert_eq!(unit, summary["unit"].as_str().unwrap());
                assert_eq!(headline["source_ref"], summary["result_ref"]);
            }
        }
        let states = source.pointer("/contract_evidence/load_reference_states").and_then(Value::as_array).is_some_and(|list| !list.is_empty());
        assert_eq!(route["load_reference_states"].is_string(), states, "{name}");
        if states {
            let line = route["load_reference_states"].as_str().unwrap();
            assert!(line.starts_with("This result includes resolved load/reference states (reference configuration "));
            assert!(line.ends_with(". Their published limitations are listed under Limitations."));
        }
    }
    assert_eq!(vectors["vectors"].as_array().unwrap().len(), 10);
}

#[test]
fn the_fixtures_strings_ending_in_the_omitted_sentence_are_the_two_source_paragraphs() {
    let coverage: Value = serde_json::from_str(COVERAGE).unwrap();
    let ending: Vec<&str> = coverage["strings"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .filter(|text| text.ends_with(OMITTED_SENTENCE))
        .collect();
    let paragraphs: std::collections::BTreeSet<&str> =
        coverage["lists"].as_array().unwrap().iter().map(|list| list["limitations"][7].as_str().unwrap()).collect();
    assert_eq!(ending.len(), 2);
    assert_eq!(ending.iter().copied().collect::<std::collections::BTreeSet<_>>(), paragraphs);
}

/// A coherent item-9 input for `source`: the invented document and audit and
/// the report sections the vectors state.
fn coherent(source: &Value) -> (Value, audit::AuditManifest, Value) {
    let document = invented_document(source);
    let audit = invented_audit();
    let projected = project(source).unwrap();
    let run = source["run_id"].as_str().unwrap();
    let preview = json!({
        "limitation_id": format!("limitation:{run}:technical-preview"),
        "source": {"ref_type": "report_renderer", "ref_id": format!("report:{run}")},
        "affected_scope": {"ref_type": "model", "ref_id": "invented-model"},
        "statement": "Output over invented or user-local data; not validated engineering output.",
        "effect": {"mechanics_solve_qualified": true, "user_rule_check_qualified": false, "report_completeness": "qualified", "human_review_required": true},
        "provenance": {"source_name": "invented session"}
    });
    let mut limitations = vec![preview.clone()];
    for entry in projected["limitations"].as_array().unwrap() {
        let mut entry = entry.clone();
        entry["effect"] = preview["effect"].clone();
        entry["provenance"] = preview["provenance"].clone();
        limitations.push(entry);
    }
    let report = json!({"exact_route": projected["exact_route"], "report_sections": {"limitations": limitations}});
    (document, audit, report)
}

fn straight_source() -> Value {
    resolve(&spec("fixtures/results/pressure_v3_straight_reader_corpus.json", "/cases/0/envelope", json!([])))
}

#[track_caller]
fn refuses(result: Result<(), String>, code: &str, case: &str) {
    match result {
        Ok(()) => panic!("{case}: passed"),
        Err(error) => assert!(error.starts_with(&format!("{code}:")), "{case}: {error}"),
    }
}

#[test]
fn item_9_admits_a_coherent_exact_report_of_every_vector_shape() {
    for (name, spec) in shapes() {
        let source = resolve(&spec);
        let (document, audit, report) = coherent(&source);
        check_exact_sections(&source, &document, &audit, &report).unwrap_or_else(|error| panic!("{name}: {error}"));
    }
}

#[test]
fn item_9_refuses_unregistered_and_changed_limitations() {
    for (name, spec, code) in refusals().into_iter().filter(|(_, _, code)| *code == EXACT_LIMITATION_UNREGISTERED) {
        let source = resolve(&spec);
        // A source and report rebound coherently to the changed source: only
        // the limitation can refuse.
        let (document, audit, mut report) = coherent(&straight_source());
        report["exact_route"]["admission"]["received_result_sha256"] = Value::from(digest(&source).unwrap());
        let error = check_exact_sections(&source, &document, &audit, &report).unwrap_err();
        assert_eq!(
            error,
            format!("{code}: openpipestress.result_semantics/0.3.0/pressure-2 publishes a limitation this report has no reviewed statement for."),
            "{name}"
        );
    }
}

#[test]
fn item_9_refuses_a_same_run_substitute_source() {
    let genuine = straight_source();
    let (document, audit, report) = coherent(&genuine);
    let mut substitute = genuine.clone();
    let row = substitute["results"].as_array_mut().unwrap().iter_mut().find(|row| row["value"].as_f64().is_some_and(|value| value != 0.0)).unwrap();
    row["value"] = Value::from(row["value"].as_f64().unwrap() * 2.0);
    assert_eq!(substitute["run_id"], genuine["run_id"]);
    refuses(check_exact_sections(&substitute, &document, &audit, &report), EXACT_SECTIONS_MISMATCH, "substitute");
    let error = check_exact_sections(&substitute, &document, &audit, &report).unwrap_err();
    assert!(error.ends_with("admission.received_result_sha256"), "{error}");
}

#[test]
fn item_9_refuses_each_binding_field_mutated() {
    let source = straight_source();
    let (document, audit, report) = coherent(&source);
    check_exact_sections(&source, &document, &audit, &report).unwrap();
    let fields = [
        "semantic_contract_id",
        "run_ref",
        "result_envelope_id",
        "received_result_sha256",
        "model_sha256",
        "input_manifest_ref",
        "input_manifest_sha256",
        "derivative_sha256",
        "native_job_id",
        "received_bytes_sha256",
    ];
    for field in fields {
        let mut changed = report.clone();
        let old = changed["exact_route"]["admission"][field].as_str().unwrap().to_string();
        // The custody facts are checked for form only; the others for value.
        changed["exact_route"]["admission"][field] = Value::from(match field {
            "native_job_id" => String::new(),
            "received_bytes_sha256" => old.to_ascii_uppercase(),
            _ if field.ends_with("sha256") => "d4".repeat(32),
            _ => format!("{old}-other"),
        });
        assert_ne!(changed, report, "{field}");
        let error = check_exact_sections(&source, &document, &audit, &changed).unwrap_err();
        assert!(error.starts_with(&format!("{EXACT_SECTIONS_MISMATCH}: admission.")), "{field}: {error}");
        assert!(error.ends_with(field), "{field}: {error}");
    }
    // The other sections: the identity, the load/reference line, a headline,
    // an unknown field, the limitation entries.
    let edits: [(&str, fn(&mut Value)); 8] = [
        ("identity", |r| r["exact_route"]["semantic_contract_id"] = Value::from("openpipestress.result_semantics/0.3.0/pressure-source-1")),
        ("load/reference line", |r| r["exact_route"]["load_reference_states"] = Value::from("invented")),
        ("headline", |r| r["exact_route"]["headlines"][0]["value_display"] = Value::from("1 Pa")),
        ("unknown field", |r| r["exact_route"]["standing"] = Value::from("invented")),
        ("statement", |r| r["report_sections"]["limitations"][2]["statement"] = r["report_sections"]["limitations"][3]["statement"].clone()),
        ("profile reference", |r| {
            r["report_sections"]["limitations"].as_array_mut().unwrap().pop();
        }),
        ("effect", |r| r["report_sections"]["limitations"][1]["effect"]["human_review_required"] = Value::from(false)),
        ("technical preview first", |r| r["report_sections"]["limitations"].as_array_mut().unwrap().swap(0, 1)),
    ];
    for (case, edit) in edits {
        let mut changed = report.clone();
        edit(&mut changed);
        refuses(check_exact_sections(&source, &document, &audit, &changed), EXACT_SECTIONS_MISMATCH, case);
    }
}

#[test]
fn item_9_binds_the_identity_to_the_presence_of_exact_sections() {
    let source = straight_source();
    let (document, audit, report) = coherent(&source);
    // An exact identity requires exact_route.
    let mut without = report.clone();
    without.as_object_mut().unwrap().remove("exact_route");
    refuses(check_exact_sections(&source, &document, &audit, &without), EXACT_SECTIONS_MISMATCH, "absent");
    without["exact_route"] = Value::Null;
    refuses(check_exact_sections(&source, &document, &audit, &without), EXACT_SECTIONS_MISMATCH, "null");
    // A preview-physics-2 document carries neither exact_route nor an exact
    // limitation entry; without them it passes unchanged.
    let preview: Value = serde_json::from_str(source_text("fixtures/results/preview_physics_connected_sparse.json")).unwrap();
    let preview_document = json!({"result_envelope": {"producer": {"semantic_contract_id": PREVIEW_PHYSICS_ID}}});
    refuses(check_exact_sections(&preview, &preview_document, &audit, &report), EXACT_SECTIONS_MISMATCH, "preview with exact_route");
    let mut entries_only = report.clone();
    entries_only.as_object_mut().unwrap().remove("exact_route");
    refuses(check_exact_sections(&preview, &preview_document, &audit, &entries_only), EXACT_SECTIONS_MISMATCH, "preview with exact entries");
    let plain = json!({"report_sections": {"limitations": [report["report_sections"]["limitations"][0]]}});
    check_exact_sections(&preview, &preview_document, &audit, &plain).unwrap();
    let mut null_route = plain.clone();
    null_route["exact_route"] = Value::Null;
    check_exact_sections(&preview, &preview_document, &audit, &null_route).unwrap();
}

#[test]
fn the_runner_profile_refuses_an_exact_identity_whatever_the_report_carries() {
    let source = straight_source();
    let document = invented_document(&source);
    refuses(check_runner_profile(RUNNER_PROFILE, &document), EXACT_RUNNER_PROFILE, "runner, pressure-2");
    let error = check_runner_profile(RUNNER_PROFILE, &document).unwrap_err();
    assert_eq!(error, format!("{EXACT_RUNNER_PROFILE}: the runner profile does not package pressure-2 or pressure-source-1 results; only the desktop builds their sections."));
    let mut source_1 = document.clone();
    source_1["result_envelope"]["producer"]["semantic_contract_id"] = Value::from("openpipestress.result_semantics/0.3.0/pressure-source-1");
    refuses(check_runner_profile(RUNNER_PROFILE, &source_1), EXACT_RUNNER_PROFILE, "runner, pressure-source-1");
    check_runner_profile(DESKTOP_PROFILE, &document).unwrap();
    let preview = json!({"result_envelope": {"producer": {"semantic_contract_id": PREVIEW_PHYSICS_ID}}});
    check_runner_profile(RUNNER_PROFILE, &preview).unwrap();
}
