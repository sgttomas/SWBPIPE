//! The native render's identity boundary (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §3.6 step 0, §5.5): the production command
//! refuses a coherent invented exact-route input before any further work, a
//! non-exact input renders as before, the test-only witness set still reaches
//! the exact render path, each structured marker refuses while free text equal
//! to an exact identity does not, and the render's admitted-set forwarding
//! graph is closed. Every id and digest is invented. Standard claim fence applies
//! (claims taxonomy per DEC-081).

#[path = "../../../../core/reporting/report_package/src/forwarding_graph_scan.rs"]
mod forwarding_graph_scan;

use crate::command_ingress::test_support::{admit_json, registry};
use crate::command_ingress::{respond, ArgsDecoding, CommandError};
use crate::ingress::counting::measure;
use crate::memory_pool::test_host::ScriptedHost;
use crate::report_render_exact::{
    check_render_route_admitted, render_calculation_report_args_in, EXACT_IDENTITIES, EXACT_RENDER_WITNESS_ADMITTED,
    PRODUCTION, RENDER_ROUTE_NOT_ADMITTED,
};
use forwarding_graph_scan::{scan, Arg, Graph, Row};
use open_pipe_stress_report_renderer::exact_statements::exact_statement_table;
use open_pipe_stress_report_renderer::{EXACT_HEADLINES_SUBHEADING, EXACT_LIMITATION_UNREGISTERED, EXACT_ROUTE_SUBHEADING};
use open_pipe_stress_result_export::semantic_contract::{PHYSICS_RETAINED_ID, PRESSURE_ID, PRESSURE_SOURCE_ID};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

const RECEIVED: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const MODEL: &str = "2222222222222222222222222222222222222222222222222222222222222222";
const MANIFEST: &str = "3333333333333333333333333333333333333333333333333333333333333333";
const DERIVATIVE: &str = "4444444444444444444444444444444444444444444444444444444444444444";
const BYTES: &str = "5555555555555555555555555555555555555555555555555555555555555555";

const PRODUCTION_REFUSAL: &str =
    "REPORT-RENDER-ROUTE-NOT-ADMITTED: the rendered report admits no exact-route identity; found openpipestress.result_semantics/0.3.0/pressure-2.";

/// A pressure-free invented render input over the invented calculation-report
/// fixture (as the command's fixture test builds it).
fn pressure_free_input() -> Value {
    let fixture_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/reports/invented/calculation_report_fixture.json");
    let fixture: Value = serde_json::from_str(&std::fs::read_to_string(fixture_path).unwrap()).unwrap();
    let provenance = json!({
        "source_name": "Invented SWBPIPE renderer test",
        "source_location": "apps/desktop/src-tauri/src/report_render_exact_tests.rs",
        "source_license": "project_fixture",
        "contributor": "SWBPIPE",
        "contributor_certification": "Invented non-engineering data only.",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "accepted",
        "privacy_classification": "invented_public_example"
    });
    let disclosure = |status: &str, source: (&str, &str), object: (&str, &str), explanation: &str| {
        json!({
            "status": status,
            "source": {"ref_type": source.0, "ref_id": source.1},
            "affected_object": {"ref_type": object.0, "ref_id": object.1},
            "explanation": explanation,
            "human_review_required": true,
            "human_acceptance_ref": null
        })
    };
    json!({
        "report_title": "Invented Calculation Report",
        "calculation_report": fixture["calculation_report"],
        "report_sections": {
            "report_section_id": "invented-report-sections-001",
            "model_ref": {"ref_type": "model", "ref_id": "invented-model"},
            "run_ref": {"ref_type": "analysis_run", "ref_id": "invented-run"},
            "diagnostics": [],
            "analysis_status_disclosures": [
                disclosure("MECHANICS_SOLVED", ("solver", "invented-solver"), ("model", "invented-model"), "Invented preview mechanics solved."),
                disclosure("HUMAN_REVIEW_REQUIRED", ("report_renderer", "invented-render"), ("report", "invented-report-001"), "Human professional review is required before any reliance."),
            ],
            "provenance_notes": [provenance],
            "user_supplied_values": [],
            "assumptions": [],
            "limitations": [{
                "limitation_id": "invented-limitation-001",
                "source": {"ref_type": "report_renderer", "ref_id": "invented-render"},
                "affected_scope": {"ref_type": "model", "ref_id": "invented-model"},
                "statement": "Invented preview data only; not engineering output.",
                "effect": {
                    "mechanics_solve_qualified": true,
                    "user_rule_check_qualified": false,
                    "report_completeness": "qualified",
                    "human_review_required": true
                },
                "provenance": provenance
            }],
            "unresolved_tbds": [],
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            }
        },
        "result_rows": [{
            "row_id": "invented-row-001",
            "label": "Max bending stress (invented)",
            "case_ref": "load_case:invented-weight",
            "quantity_display": "12.5 MPa",
            "source_ref": "result_envelope:invented-result"
        }]
    })
}

/// `base`'s coherent exact-route form, as the renderer's own unblocked exact
/// test builds it (`report_renderer/tests/common/exact_input.rs`): the
/// `exact_route` block with its admission binding bound to the report's own
/// records, one exact limitation entry per table statement and the profile
/// reference.
fn with_exact_route(mut input: Value) -> Value {
    let run = input["report_sections"]["run_ref"]["ref_id"].as_str().unwrap().to_string();
    let envelope_id = format!("result-envelope:{run}");
    let calc = &mut input["calculation_report"];
    let mut received = calc["audit_manifest_refs"][0].clone();
    received["ref"] = json!({"ref_type": "result_envelope", "ref_id": envelope_id});
    received["checksum"]["payload_ref"] = received["ref"].clone();
    received["checksum"]["value"] = json!(RECEIVED);
    calc["audit_manifest_refs"].as_array_mut().unwrap().push(received);
    calc["model_input_summary"]["model_hash"]["value"] = json!(MODEL);
    let manifest_ref = calc["model_input_summary"]["input_manifest_ref"]["ref_id"].clone();
    input["exact_route"] = json!({
        "semantic_contract_id": PRESSURE_ID,
        "admission": {
            "semantic_contract_id": PRESSURE_ID,
            "run_ref": run,
            "result_envelope_id": envelope_id,
            "received_result_sha256": RECEIVED,
            "model_sha256": MODEL,
            "input_manifest_ref": manifest_ref,
            "input_manifest_sha256": MANIFEST,
            "derivative_sha256": DERIVATIVE,
            "native_job_id": "invented-job-1",
            "received_bytes_sha256": BYTES
        },
        "load_reference_states": null,
        "headlines": [
            {"basis": "nominal_section", "label": "Nominal section (nominal wall)", "value_display": "41354908.75465696 Pa", "location_ref": "pipe:invented-span", "source_ref": "result:invented-nominal-maximum"},
            {"basis": "reduced_section", "label": "Reduced section (wall minus mill tolerance and corrosion allowance)", "value_display": "withheld: a stress maximum on this basis is not available for every member in every load case", "location_ref": null, "source_ref": null}
        ]
    });
    let template = input["report_sections"]["limitations"][0].clone();
    let entry = |id: String, statement: &str| {
        let mut entry = template.clone();
        entry["limitation_id"] = json!(id);
        entry["source"] = json!({"ref_type": "semantic_contract", "ref_id": PRESSURE_ID});
        entry["affected_scope"] = json!({"ref_type": "analysis_run", "ref_id": run});
        entry["statement"] = json!(statement);
        entry
    };
    let mut entries: Vec<Value> = exact_statement_table()
        .unwrap()
        .entries()
        .enumerate()
        .map(|(index, (_, statement))| entry(format!("limitation:{run}:exact:{}", index + 1), statement))
        .collect();
    entries.push(entry(
        format!("limitation:{run}:exact:profile-reference"),
        "The profile limitations of the semantic table openpipestress.result_semantics/0.3.0/pressure-2 (sha256 6666666666666666666666666666666666666666666666666666666666666666) are bound by reference and are not restated in this report.",
    ));
    input["report_sections"]["limitations"].as_array_mut().unwrap().extend(entries);
    input
}

/// The command body (`render_calculation_report`, `lib.rs`) on a raw args
/// body: the ingress admission, the command's production `*_args` root and
/// the response serialized under the held lease (its JSON text).
fn command_body(args: &Value) -> Result<String, CommandError> {
    let registry = registry(ScriptedHost::normal());
    let (mut args, mut lease) = admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, args).unwrap();
    respond(crate::render_calculation_report_args(&mut args, &registry, &mut lease)).map(response_text)
}

fn response_text(response: tauri::ipc::Response) -> String {
    use tauri::ipc::IpcResponse;
    match response.body().unwrap() {
        tauri::ipc::InvokeResponseBody::Json(text) => text,
        tauri::ipc::InvokeResponseBody::Raw(_) => panic!("raw response"),
    }
}

fn refusal_text(outcome: Result<impl std::fmt::Debug, CommandError>) -> String {
    match outcome {
        Err(CommandError::Command(message)) => message,
        other => panic!("not a command refusal: {other:?}"),
    }
}

fn witness_render(input: &Value) -> Result<Value, CommandError> {
    let registry = registry(ScriptedHost::normal());
    let (mut args, mut lease) =
        admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": input})).unwrap();
    render_calculation_report_args_in(&mut args, &registry, &mut lease, EXACT_RENDER_WITNESS_ADMITTED)
}

/// W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md` §6.2),
/// Codex review round 1 of #1639: the production native command, called
/// directly with a pressure-free input that carries a private FR-REP-003
/// record, renders it only under `local_private`; under `public_report` or
/// with no report context it refuses private record content, and a
/// `quarantined` record refuses in every context. A refused render is
/// export-blocked, so no sink receives it; the record's typed
/// `source_location` path therefore reaches only the local-private document.
#[test]
fn the_native_render_refuses_private_records_outside_local_private() {
    let record = |review: &str| json!({
        "source_name": "Invented material datasheet", "source_location": "/Users/invented-user/datasheets/steel-a.pdf", "source_license": "invented-license",
        "contributor": "invented contributor", "contributor_certification": "invented certification",
        "redistribution_status": "private_only", "review_status": review, "privacy_classification": "private_project_data"
    });
    let context = |export_context: &str| json!({
        "export_context": export_context, "generated_at": null, "software_name": "Invented", "software_version": null,
        "build_release_identifier": null, "solver_name": null, "solver_version": null, "solver_build_ref": null,
        "analysis_run_name": null, "analysis_run_recorded_at": null, "solver_mode": null, "solver_settings_ref": null,
        "solver_settings": [], "model_coordinate_system": null, "result_coordinate_systems": []
    });
    let input = |review: &str, export_context: Option<&str>| {
        let mut input = pressure_free_input();
        input["report_sections"]["provenance_notes"].as_array_mut().unwrap().push(record(review));
        if let Some(export_context) = export_context {
            input["report_context"] = context(export_context);
        }
        json!({"input": input})
    };
    let outcome = |args: &Value| -> Value { serde_json::from_str(&command_body(args).expect("the render answers")).unwrap() };
    let reasons = |rendered: &Value| -> Vec<String> {
        rendered["blocking_reasons"].as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_string()).collect()
    };
    // The invented public input renders without a context, as before.
    assert_eq!(outcome(&json!({"input": pressure_free_input()}))["export_blocked"], json!(false));
    let local = outcome(&input("pending", Some("local_private")));
    assert_eq!(local["export_blocked"], json!(false), "{:?}", reasons(&local));
    assert!(local["html"].as_str().unwrap().contains("Invented material datasheet"));
    // Owner ruling on #1639 ("C1 b, C2 a"): the typed path stays as written locally.
    assert!(local["html"].as_str().unwrap().contains("<td>/Users/invented-user/datasheets/steel-a.pdf</td>"));
    for export_context in [Some("public_report"), None] {
        let refused = outcome(&input("pending", export_context));
        assert_eq!(refused["export_blocked"], json!(true), "{export_context:?}");
        assert!(
            reasons(&refused).iter().any(|reason| reason.starts_with("report provenance: REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE (")),
            "{:?}",
            reasons(&refused)
        );
    }
    for export_context in [Some("local_private"), Some("public_report"), None] {
        let refused = outcome(&input("quarantined", export_context));
        assert_eq!(refused["export_blocked"], json!(true), "{export_context:?}");
        assert!(reasons(&refused).iter().any(|reason| reason.contains("REPORT_PROVENANCE_DISPOSITION_BLOCKED (provenance_notes[1] review_status quarantined;")));
    }
}

/// Codex review round 1 (blocking): a coherent invented exact input, which
/// the renderer itself renders unblocked with "integrity checked", is refused
/// by the production native command at step 0, with no HTML. Since slice (b)
/// the witness set's exact path binds the source natively first, so this
/// invented input, whose job is not in custody, refuses there
/// (`REPORT-EXACT-CUSTODY-RELEASED`); the genuine live-job input renders under
/// the witness set (`exact_render_direct_native_tests`).
#[test]
fn production_native_render_refuses_a_coherent_exact_input() {
    let exact = with_exact_route(pressure_free_input());
    // Coherent: the renderer itself renders it unblocked, with the exact block
    // and the composed standing line.
    let rendered = crate::render_calculation_report_value(exact.clone()).expect("the renderer renders it");
    assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
    let html = rendered["html"].as_str().unwrap();
    assert!(html.contains(EXACT_ROUTE_SUBHEADING) && html.contains("Numerical standing: integrity checked"));
    // Below the gate, the witness set reaches the native source binding, which
    // refuses an invented job.
    let refused = refusal_text(witness_render(&exact));
    assert!(refused.starts_with("REPORT-EXACT-CUSTODY-RELEASED: "), "{refused}");

    // The production root, through the ingress decode.
    let registry = registry(ScriptedHost::normal());
    let (mut args, mut lease) =
        admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": exact})).unwrap();
    assert_eq!(refusal_text(crate::render_calculation_report_args(&mut args, &registry, &mut lease)), PRODUCTION_REFUSAL);
    // The command body on the raw args body: the refusal is the response;
    // no HTML is produced.
    let refused = command_body(&json!({"input": exact}));
    assert_eq!(refusal_text(refused), PRODUCTION_REFUSAL);
    assert_eq!(serde_json::to_value(CommandError::<String>::Command(PRODUCTION_REFUSAL.into())).unwrap(), json!(PRODUCTION_REFUSAL));
}

/// Step 0 runs before the renderer's own decode: an exact input that would
/// not even decode refuses with the route code, not `RENDER-INPUT-INVALID`.
#[test]
fn step_zero_precedes_the_render_input_decode() {
    let broken = json!({"report_title": "broken", "exact_route": {"semantic_contract_id": PRESSURE_ID}});
    assert_eq!(refusal_text(command_body(&json!({"input": broken}))), PRODUCTION_REFUSAL);
    let broken = json!({"report_title": "broken"});
    assert!(refusal_text(command_body(&json!({"input": broken}))).starts_with("RENDER-INPUT-INVALID"));
}

/// The production refusal of `input` through the production root (after the
/// ingress decode) and through the command body on the raw args body; both
/// must agree, and the command produces no HTML.
fn production_refusal(input: &Value) -> String {
    let registry = registry(ScriptedHost::normal());
    let (mut args, mut lease) =
        admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": input})).unwrap();
    let root = refusal_text(crate::render_calculation_report_args(&mut args, &registry, &mut lease));
    assert_eq!(refusal_text(command_body(&json!({"input": input}))), root);
    root
}

/// `base`'s exact limitation entries with neither an exact identity nor an
/// exact id: the copied statements stay, as free text.
fn without_exact_limitation_markers(mut input: Value) -> Value {
    for (index, entry) in input["report_sections"]["limitations"].as_array_mut().unwrap().iter_mut().enumerate() {
        entry["limitation_id"] = json!(format!("invented-copied-limitation-{index}"));
        entry["source"] = json!({"ref_type": "report_renderer", "ref_id": "invented-render"});
    }
    input
}

/// Each structured marker refuses in production, through the production root
/// and the raw body: a non-null `exact_route` (even one that names no
/// identity, or is not an object), an exact identity at an identity field
/// (`exact_route.semantic_contract_id`, `exact_route.admission.semantic_contract_id`,
/// a `semantic_contract` limitation source), and exact limitation ids without
/// `exact_route`.
#[test]
fn every_structured_marker_refuses_in_production() {
    let exact = with_exact_route(pressure_free_input());
    let no_identity_text = format!(
        "{RENDER_ROUTE_NOT_ADMITTED}: the rendered report admits no exact-route identity; found an exact_route section that names no identity."
    );
    let found = |id: &str| format!("{RENDER_ROUTE_NOT_ADMITTED}: the rendered report admits no exact-route identity; found {id}.");
    let mut no_identity = exact.clone();
    no_identity["exact_route"].as_object_mut().unwrap().remove("semantic_contract_id");
    let mut empty_route = pressure_free_input();
    empty_route["exact_route"] = json!({});
    let mut text_route = pressure_free_input();
    text_route["exact_route"] = json!("invented");
    let mut retained_route = exact.clone();
    retained_route["exact_route"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    retained_route["exact_route"]["admission"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    // Without `exact_route`, the limitation entries' sources still name pressure-2.
    let mut limitations_only = exact.clone();
    limitations_only.as_object_mut().unwrap().remove("exact_route");
    // A single `semantic_contract` limitation source on a pressure-free input.
    let mut source_only = pressure_free_input();
    source_only["report_sections"]["limitations"][0]["source"] = json!({"ref_type": "semantic_contract", "ref_id": PRESSURE_SOURCE_ID});
    // Exact limitation ids alone, with no exact identity at any identity field.
    let mut ids_only = limitations_only.clone();
    for entry in ids_only["report_sections"]["limitations"].as_array_mut().unwrap() {
        entry["source"] = json!({"ref_type": "report_renderer", "ref_id": "invented-render"});
    }
    let cases = [
        ("the coherent exact input", &exact, PRODUCTION_REFUSAL.to_string()),
        ("an exact_route that names no identity", &no_identity, no_identity_text.clone()),
        ("an empty exact_route", &empty_route, no_identity_text.clone()),
        ("an exact_route that is not an object", &text_route, no_identity_text),
        ("an exact_route naming physics-retained-1", &retained_route, found(PHYSICS_RETAINED_ID)),
        ("exact limitation entries without exact_route", &limitations_only, PRODUCTION_REFUSAL.to_string()),
        ("a semantic_contract limitation source", &source_only, found(PRESSURE_SOURCE_ID)),
        (
            "exact limitation ids without exact_route",
            &ids_only,
            format!("{RENDER_ROUTE_NOT_ADMITTED}: the rendered report admits no exact-route identity; found the exact-route limitation entry limitation:invented-run:exact:1 without an exact_route section."),
        ),
    ];
    for (case, input, expected) in cases {
        assert_eq!(check_render_route_admitted(input, PRODUCTION).unwrap_err(), expected, "{case}");
        assert_eq!(production_refusal(input), expected, "{case}");
    }
    // Below the gate, an admitted route still refuses an unadmitted identity
    // at another identity field, and exact limitations without `exact_route`.
    let witness_refusal = |input: &Value| check_render_route_admitted(input, EXACT_RENDER_WITNESS_ADMITTED).unwrap_err();
    let mut retained_admission = exact.clone();
    retained_admission["exact_route"]["admission"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    assert!(witness_refusal(&retained_admission).ends_with(&format!("found {PHYSICS_RETAINED_ID}.")));
    let mut retained_source = exact.clone();
    retained_source["report_sections"]["limitations"][1]["source"]["ref_id"] = json!(PHYSICS_RETAINED_ID);
    assert!(witness_refusal(&retained_source).ends_with(&format!("found {PHYSICS_RETAINED_ID}.")));
    assert!(witness_refusal(&limitations_only).ends_with(&format!("found {PRESSURE_ID}.")));
    assert!(witness_refusal(&ids_only).ends_with("without an exact_route section."));
    // The admitting decision allocates nothing.
    let (admitted, measured) = measure(|| check_render_route_admitted(&exact, EXACT_RENDER_WITNESS_ADMITTED));
    assert!(admitted.is_ok());
    assert_eq!(measured.allocations, 0, "{measured:?}");
}

/// Codex review round 2 (P2): free text is not route evidence. A pressure-free
/// input whose load-case label and other free-text fields (the report title,
/// a result row label, a limitation statement, a provenance source name, a
/// disclosure explanation, a non-`semantic_contract` reference id) exactly
/// equal an exact identity renders through the production command, from the
/// raw body, byte-identically to the unchanged renderer call. Step 0
/// allocates nothing on it, and an object key equal to an identity is no
/// marker either.
#[test]
fn free_text_equal_to_an_exact_identity_renders_as_before() {
    for id in EXACT_IDENTITIES {
        let mut input = pressure_free_input();
        input["calculation_report"]["load_case_summary"][0]["label"] = json!(id);
        input["report_title"] = json!(id);
        input["result_rows"][0]["label"] = json!(id);
        let sections = &mut input["report_sections"];
        sections["limitations"][0]["statement"] = json!(id);
        sections["limitations"][0]["source"]["ref_id"] = json!(id);
        sections["provenance_notes"][0]["source_name"] = json!(id);
        sections["analysis_status_disclosures"][0]["explanation"] = json!(id);
        let before = serde_json::to_string(&crate::render_calculation_report_value(input.clone()).unwrap()).unwrap();
        assert!(before.contains(id) && !before.contains(EXACT_ROUTE_SUBHEADING), "{id}");
        assert_eq!(command_body(&json!({"input": input})).expect("free text is not route evidence"), before, "{id}");
        let registry = registry(ScriptedHost::normal());
        let (mut args, mut lease) =
            admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": input})).unwrap();
        assert_eq!(serde_json::to_string(&crate::render_calculation_report_args(&mut args, &registry, &mut lease).unwrap()).unwrap(), before);
        let (outcome, measured) = measure(|| check_render_route_admitted(&input, PRODUCTION));
        assert!(outcome.is_ok(), "{id}: {outcome:?}");
        assert_eq!(measured.allocations, 0, "{measured:?}");
        let mut keyed = pressure_free_input();
        keyed["calculation_report"]["template_slots"] = json!({id: 1});
        assert_eq!(check_render_route_admitted(&keyed, PRODUCTION), Ok(()), "{id}");
    }
}

/// The renderer facts the structured rule rests on (Codex review round 2): an
/// input that avoids every marker, here the coherent exact input without
/// `exact_route` and with its limitation entries' ids and sources renamed
/// (their statements copied as free text), passes step 0 and renders with no
/// exact block, standing line or headlines, byte-identically to the unchanged
/// renderer call. Exact limitation entries without `exact_route` block export
/// in the renderer itself, below step 0.
#[test]
fn a_marker_free_input_produces_no_exact_output() {
    let mut copied = with_exact_route(pressure_free_input());
    copied.as_object_mut().unwrap().remove("exact_route");
    let copied = without_exact_limitation_markers(copied);
    let before = serde_json::to_string(&crate::render_calculation_report_value(copied.clone()).unwrap()).unwrap();
    for absent in [EXACT_ROUTE_SUBHEADING, EXACT_HEADLINES_SUBHEADING, "Numerical standing", "integrity checked"] {
        assert!(!before.contains(absent), "{absent}");
    }
    assert_eq!(command_body(&json!({"input": copied})).expect("a marker-free input renders"), before);

    let mut limitations_only = with_exact_route(pressure_free_input());
    limitations_only.as_object_mut().unwrap().remove("exact_route");
    let rendered = crate::render_calculation_report_value(limitations_only).unwrap();
    assert_eq!(rendered["export_blocked"], json!(true));
    let reasons = rendered["blocking_reasons"].as_array().unwrap();
    assert!(
        reasons.iter().any(|reason| reason.as_str().unwrap().contains(EXACT_LIMITATION_UNREGISTERED)),
        "{reasons:?}"
    );
    assert!(!rendered["html"].as_str().unwrap().contains(EXACT_ROUTE_SUBHEADING));
}

/// A non-exact input renders through the production command byte-identically
/// to the unchanged renderer call (the command's body before W-11), with
/// `exact_route` absent or null; step 0 allocates nothing on it.
#[test]
fn a_non_exact_input_renders_as_before() {
    let input = pressure_free_input();
    let before = serde_json::to_string(&crate::render_calculation_report_value(input.clone()).unwrap()).unwrap();
    assert!(before.contains("\"export_blocked\":false"));
    let mut null_route = input.clone();
    null_route["exact_route"] = Value::Null;
    for input in [input.clone(), null_route] {
        let response = command_body(&json!({"input": input})).expect("a pressure-free input renders");
        assert_eq!(response, before);
    }
    // Under the witness set too.
    assert_eq!(serde_json::to_string(&witness_render(&input).unwrap()).unwrap(), before);
    let (outcome, measured) = measure(|| check_render_route_admitted(&input, PRODUCTION));
    assert!(outcome.is_ok());
    assert_eq!(measured.allocations, 0, "{measured:?}");
}

const ROWS: [Row; 4] = [
    ("render_calculation_report_args", "lib.rs", &[("lib.rs", "render_calculation_report", Arg::None)]),
    (
        "render_calculation_report_args_in",
        "report_render_exact.rs",
        &[("lib.rs", "render_calculation_report_args", Arg::Production)],
    ),
    (
        "check_render_route_admitted",
        "report_render_exact.rs",
        &[("report_render_exact.rs", "render_calculation_report_args_in", Arg::Admitted)],
    ),
    (
        "render_calculation_report_value",
        "lib.rs",
        &[("report_render_exact.rs", "render_calculation_report_args_in", Arg::None)],
    ),
];

const SCANNER: &str = "report_render_exact_tests.rs";

const GRAPH: Graph = Graph {
    rows: &ROWS,
    set_type: "RenderAdmittedSet",
    set_module: "report_render_exact",
    witness: "EXACT_RENDER_WITNESS_ADMITTED",
    witness_files: &[SCANNER],
};

/// Every `.rs` file under `dir`, keyed by its path relative to `root`.
fn read_sources(dir: &Path, root: &Path, out: &mut BTreeMap<String, String>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            read_sources(&path, root, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let local = path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            out.insert(local, std::fs::read_to_string(&path).unwrap());
        }
    }
}

#[test]
fn render_admitted_set_forwarding_graph_is_closed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = BTreeMap::new();
    read_sources(&root, &root, &mut files);
    assert!(files.contains_key("report_render_exact.rs") && files.contains_key(SCANNER), "{:?}", files.keys());
    let findings = scan(&GRAPH, &files);
    assert!(findings.is_empty(), "{findings:#?}");
}

/// An invented desktop crate whose render graph is closed.
fn invented_crate() -> BTreeMap<String, String> {
    let file = |text: &str| text.trim_start_matches('\n').to_string();
    BTreeMap::from([
        (
            "lib.rs".to_string(),
            file("
mod report_render_exact;
#[cfg(test)]
mod report_render_exact_tests;
#[tauri::command]
fn render_calculation_report(
    invoke: Request,
) -> R {
    respond(render_calculation_report_args(&mut args))
}
fn render_calculation_report_args(args: &mut Args) -> R {
    report_render_exact::render_calculation_report_args_in(args, report_render_exact::PRODUCTION)
}
fn render_calculation_report_value(input: Value) -> R {
    Ok(input)
}
#[cfg(test)]
mod tests {
    fn t() { render_calculation_report_value(json!({})); }
}
"),
        ),
        (
            "report_render_exact.rs".to_string(),
            file("
pub(crate) struct RenderAdmittedSet(&'static [&'static str]);
pub(crate) const PRODUCTION: RenderAdmittedSet = RenderAdmittedSet(&RENDER_ADMITTED_EXACT_CONTRACTS);
#[cfg(test)]
pub(crate) const EXACT_RENDER_WITNESS_ADMITTED: RenderAdmittedSet = RenderAdmittedSet(&[A, B]);
pub(crate) fn render_calculation_report_args_in(args: &mut Args, admitted: RenderAdmittedSet) -> R {
    let input = args.value(\"input\")?;
    check_render_route_admitted(&input, admitted)?;
    crate::render_calculation_report_value(input)
}
pub(crate) fn check_render_route_admitted(input: &Value, admitted: RenderAdmittedSet) -> Result<(), String> {
    Ok(())
}
"),
        ),
        (
            "report_render_exact_tests.rs".to_string(),
            file("fn w() { render_calculation_report_args_in(&mut a, EXACT_RENDER_WITNESS_ADMITTED) }\n"),
        ),
    ])
}

#[test]
fn the_render_scanner_finds_each_break_of_the_graph() {
    let closed = invented_crate();
    assert!(scan(&GRAPH, &closed).is_empty(), "{:#?}", scan(&GRAPH, &closed));
    let edit = |file: &str, old: &str, new: &str| {
        let mut files = closed.clone();
        let text = files.get_mut(file).unwrap();
        assert_eq!(text.matches(old).count(), 1, "{old}");
        *text = text.replace(old, new);
        files
    };
    let other = |body: &str| {
        edit("lib.rs", "fn render_calculation_report_value(", &format!("fn other(input: Value) -> R {{\n{body}\n}}\nfn render_calculation_report_value("))
    };
    let cases = [
        (
            "a renderer call that skips step 0",
            other("    render_calculation_report_value(input)"),
            "other calls render_calculation_report_value outside the forwarding graph",
        ),
        (
            "an alias of the renderer call",
            other("    let render = crate::render_calculation_report_value;\n    render(input)"),
            "render_calculation_report_value is used other than as a direct call",
        ),
        (
            "an alias of the exact path",
            other("    let render = report_render_exact::render_calculation_report_args_in;\n    render(args, report_render_exact::PRODUCTION)"),
            "render_calculation_report_args_in is used other than as a direct call",
        ),
        (
            "a root passing another argument",
            edit("lib.rs", "args, report_render_exact::PRODUCTION)", "args, admitted)"),
            "the root render_calculation_report_args passes admitted, not report_render_exact::PRODUCTION",
        ),
        (
            "a witness-set use outside the tests",
            edit("lib.rs", "args, report_render_exact::PRODUCTION)", "args, report_render_exact::EXACT_RENDER_WITNESS_ADMITTED)"),
            "EXACT_RENDER_WITNESS_ADMITTED in non-test source",
        ),
        (
            "a construction outside report_render_exact.rs",
            other("    let wide = RenderAdmittedSet(&[A]);"),
            "RenderAdmittedSet is constructed outside report_render_exact.rs",
        ),
        (
            "a public exact path",
            edit("report_render_exact.rs", "pub(crate) fn render_calculation_report_args_in", "pub fn render_calculation_report_args_in"),
            "render_calculation_report_args_in is declared pub fn",
        ),
    ];
    for (case, files, expected) in cases {
        let findings = scan(&GRAPH, &files);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{case}: {findings:#?}");
    }
}

// ---------------------------------------------------------------------------
// W-11 slice (b): the native source binding (DEL-08-01 Design
// `exact-route-report-sections.md` §3.6, §5.5, §7 "Direct-native
// substitution", "The production native render refuses a genuine exact
// result" and "Pricing"). A test registry job is admitted from a witness's
// intake model text, delivers the committed producer bytes through the gated
// test launcher and is taken into custody. The witness requests are the ones
// the desktop's vitest assembles below the output gate
// (`fixtures/reports/invented/exact_report_package_requests/`); their
// `native_job_id` is set to the live job's. Every id is invented.
// ---------------------------------------------------------------------------

mod direct_native {
    use super::*;
    use crate::command_ingress::{admit_body, test_support::headers, text_parse_bytes, INGRESS_ARGS_JSON_V1, REPORT_PACKAGE_WORKING_MEMORY_FACTOR};
    use crate::exact_source_binding::{CUSTODY_RELEASED, SECTIONS_MISMATCH, SOURCE_NOT_DELIVERED, STANDING_NOT_ELIGIBLE};
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::report_package_save::test_support::{no_store, save_request_with, software};
    use crate::report_package_save::{
        admit_report_package_in, parse_lease_bytes, parse_lease_bytes_with_exact, save_admitted_report_package_in,
        ReportPackageSaveRequest, MAX_MODEL_TEXT_BYTES, TOO_LARGE,
    };
    use crate::report_render_exact::exact_render_growth_bytes;
    use crate::solve_jobs::test_support::delivered_in_custody;
    use crate::solve_jobs::SolveJobRegistry;
    use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
    use open_pipe_stress_result_export::derivative::digest;
    use std::sync::Arc;
    use tauri::ipc::InvokeBody;

    const STRAIGHT: &str = include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json");
    const LOAD_REFERENCE: &str =
        include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_load_reference_states.json");
    const CONNECTOR: &str = include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_connector.json");
    const BEND: &str = include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_realized_bend.json");
    const SUBSTITUTE_OVERLAY: &str =
        include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight_substitute.overlay.json");
    const SOURCE_1: &str = include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json");
    /// The five witness shapes of §5.5.
    const SHAPES: [(&str, &str); 5] = [
        ("pressure_2_straight", STRAIGHT),
        ("pressure_2_load_reference_states", LOAD_REFERENCE),
        ("pressure_2_connector", CONNECTOR),
        ("pressure_2_realized_bend", BEND),
        ("pressure_source_1", SOURCE_1),
    ];
    const ROUTE_NOT_ADMITTED: &str = "REPORT-PACKAGE-ROUTE-NOT-ADMITTED";

    fn project(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)
    }

    /// One witness: its request, model, input manifest, the intake text the
    /// job was sent, the delivered bytes (the committed raw file) and the mode.
    struct Witness {
        request: Value,
        model: Value,
        manifest: Value,
        intake: String,
        result: String,
        mode: String,
    }

    /// The substitute: the straight witness with the overlay's leaf
    /// replacements applied to its request (its model, input manifest and
    /// intake text are the straight witness's).
    fn substitute_witness() -> Witness {
        let overlay: Value = serde_json::from_str(SUBSTITUTE_OVERLAY).unwrap();
        assert_eq!(overlay["base"], "pressure_2_straight.json");
        let mut w = witness(STRAIGHT);
        for entry in overlay["replacements"].as_array().unwrap() {
            *w.request.pointer_mut(entry["pointer"].as_str().unwrap()).unwrap() = entry["value"].clone();
        }
        w
    }

    fn witness(text: &str) -> Witness {
        let file: Value = serde_json::from_str(text).unwrap();
        Witness {
            request: file["request"].clone(),
            model: file["model"].clone(),
            manifest: file["input_manifest"].clone(),
            intake: file["exact_intake_text"].as_str().unwrap().to_string(),
            result: std::fs::read_to_string(project(file["raw_path"].as_str().unwrap())).unwrap(),
            mode: file["solver_mode"].as_str().unwrap().to_string(),
        }
    }

    /// A live custody job for `w` on `host`.
    struct Live {
        registry: SolveJobRegistry,
        _gate: Arc<crate::solve_worker::test_support::Gate>,
        job: String,
    }

    fn live_on(host: Arc<ScriptedHost>, intake: &str, mode: &str, result: &str) -> Live {
        let (registry, gate, job) = delivered_in_custody(host, intake.as_bytes(), Some(mode), result);
        Live { registry, _gate: gate, job }
    }

    fn live(w: &Witness) -> Live {
        live_on(ScriptedHost::normal(), &w.intake, &w.mode, &w.result)
    }

    /// `request` bound to the live job (its `native_job_id`).
    fn bound(request: &Value, job: &str) -> Value {
        let mut request = request.clone();
        request["report"]["exact_route"]["admission"]["native_job_id"] = json!(job);
        request
    }

    fn save_args(request: &Value, w: &Witness, model: &Value, result: Option<&str>, intake: Option<&str>) -> ReportPackageSaveRequest {
        let mut args = save_request_with(request, model, &w.manifest);
        args.exact_result_text = result.map(str::to_string);
        args.exact_intake_text = intake.map(str::to_string);
        args
    }

    fn save_code(live: &Live, args: ReportPackageSaveRequest) -> (String, String) {
        let custody = |job: &str| live.registry.custody_binding(job);
        match admit_report_package_in(args, &software(), &mut no_store, &custody) {
            Ok(_) => panic!("admitted"),
            Err(error) => (error.code, error.message),
        }
    }

    fn render_args(input: &Value, result: Option<&str>, intake: Option<&str>, manifest: Option<&str>) -> Value {
        let mut args = json!({"input": input});
        for (key, text) in [("exactResultText", result), ("exactIntakeText", intake), ("inputManifestText", manifest)] {
            if let Some(text) = text {
                args[key] = json!(text);
            }
        }
        args
    }

    fn witness_render_on(live: &Live, args: &Value) -> Result<Value, CommandError> {
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, args).unwrap();
        render_calculation_report_args_in(&mut decoded, &live.registry, &mut lease, EXACT_RENDER_WITNESS_ADMITTED)
    }

    fn production_render_on(live: &Live, args: &Value) -> Result<Value, CommandError> {
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, args).unwrap();
        crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease)
    }

    fn genuine_render_args(w: &Witness, job: &str) -> Value {
        let request = bound(&w.request, job);
        render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)))
    }

    #[test]
    fn the_genuine_request_passes_the_native_binding_and_refuses_at_item_0() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let request = bound(&w.request, &live.job);
        let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, ROUTE_NOT_ADMITTED, "{message}");
        // Without the texts, the binding refuses first.
        let (code, _) = save_code(&live, save_args(&request, &w, &w.model, None, Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED);
        // The committed request's invented job is not in custody.
        let (code, _) = save_code(&live, save_args(&w.request, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, CUSTODY_RELEASED);
        // A pressure-free request carrying exact texts refuses.
        let pressure_free = crate::report_package_save::test_support::desktop_request(&crate::report_package_save::test_support::model());
        let mut args = crate::report_package_save::test_support::save_request(&pressure_free, &crate::report_package_save::test_support::model());
        args.exact_result_text = Some(w.result.clone());
        let (code, message) = save_code(&live, args);
        assert_eq!((code.as_str(), message.contains("exact_route")), (SECTIONS_MISMATCH, true), "{message}");
    }

    /// Every witness shape (§5.5): with its live job and genuine texts, the
    /// render input renders under the witness set (HTML and the derived print
    /// view, export gates passed), and the save request passes the native
    /// binding and refuses at item 0.
    #[test]
    fn every_witness_shape_binds_natively_renders_below_the_gate_and_refuses_at_item_0() {
        for (name, text) in SHAPES {
            let w = witness(text);
            let live = live(&w);
            let rendered = witness_render_on(&live, &genuine_render_args(&w, &live.job)).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            assert_eq!(rendered["export_blocked"], json!(false), "{name}: {}", rendered["blocking_reasons"]);
            let html = rendered["html"].as_str().unwrap();
            assert!(html.contains(EXACT_ROUTE_SUBHEADING) && html.contains("Numerical standing: integrity checked"), "{name}");
            assert!(rendered["derived_print_html"].as_str().is_some_and(|print| print.contains(EXACT_ROUTE_SUBHEADING)), "{name}");
            let refused = refusal_text(production_render_on(&live, &genuine_render_args(&w, &live.job)));
            assert_eq!(refused.split(':').next(), Some(RENDER_ROUTE_NOT_ADMITTED), "{name}");
            let (code, message) = save_code(&live, save_args(&bound(&w.request, &live.job), &w, &w.model, Some(&w.result), Some(&w.intake)));
            assert_eq!(code, ROUTE_NOT_ADMITTED, "{name}: {message}");
        }
    }

    /// Codex r1 finding 2: with genuine custody and texts, a fabricated row
    /// (a numerical-health id with a stress statement and invented refs)
    /// refuses at the render's re-derivation; the save refuses at item 0, and
    /// the package's item 6 refuses the row behind it
    /// (`report_package` `a_fabricated_report_row_refuses_at_item_6`).
    #[test]
    fn a_fabricated_row_refuses_at_the_render_and_the_save() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let fabricated = json!({
            "row_id": "numerical-health:case:invented",
            "label": "Maximum stress 1 Pa on every member (fabricated)",
            "case_ref": "load_case:case:invented",
            "quantity_display": "",
            "source_ref": "diagnostic:invented"
        });
        let mut args = genuine_render_args(&w, &live.job);
        args["input"]["result_rows"].as_array_mut().unwrap().push(fabricated.clone());
        assert_eq!(refusal_text(witness_render_on(&live, &args)), format!("{SECTIONS_MISMATCH}: result_rows"));
        let mut request = bound(&w.request, &live.job);
        request["report"]["result_rows"].as_array_mut().unwrap().push(fabricated);
        let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, ROUTE_NOT_ADMITTED, "{message}");
    }

    #[test]
    fn the_genuine_render_input_renders_under_the_witness_set() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let rendered = witness_render_on(&live, &genuine_render_args(&w, &live.job)).expect("the genuine input renders");
        assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
        let html = rendered["html"].as_str().unwrap();
        assert!(html.contains(EXACT_ROUTE_SUBHEADING) && html.contains("Numerical standing: integrity checked"));
        assert!(html.contains(EXACT_HEADLINES_SUBHEADING) && html.contains("[reduced section]"));
        assert!(live.registry.custody_binding(&live.job).is_some(), "the render keeps no bytes and releases no custody");
    }

    #[test]
    fn a_same_run_substitute_refuses_at_the_native_binding() {
        let w = witness(STRAIGHT);
        let substitute = substitute_witness();
        let live = live(&w);
        let request = bound(&substitute.request, &live.job);
        // 1. With the genuine result text: step 5 (the binding's result digest).
        let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED, "{message}");
        assert!(message.contains(": result:"), "{message}");
        let refused = refusal_text(witness_render_on(&live, &render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: result:")), "{refused}");
        // The variant keeping the genuine exact_route binding: the save's source
        // equality, and the render's re-derivation.
        let genuine = bound(&w.request, &live.job);
        let mut variant = request.clone();
        variant["report"]["exact_route"] = genuine["report"]["exact_route"].clone();
        let (code, message) = save_code(&live, save_args(&variant, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED, "{message}");
        assert!(message.contains("/request/source_results/0"), "{message}");
        let refused = refusal_text(witness_render_on(&live, &render_args(&variant["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)))));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: result_rows"));
        // 2. With its own text: step 2.
        let own = serde_json::to_string(&substitute.request["source_results"][0]).unwrap();
        let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&own), Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED);
        assert!(message.contains(": result:"), "{message}");
        let refused = refusal_text(witness_render_on(&live, &render_args(&request["report"], Some(&own), Some(&w.intake), Some(&canonical_json(&w.manifest)))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: result:")), "{refused}");
    }

    /// The witness model with one node moved (invented edit). The straight
    /// witness's model has no spring, so no support stiffness to change.
    fn substituted_model(model: &Value) -> Value {
        let mut model = model.clone();
        let x = model["nodes"][0]["position"]["x"].as_f64().expect("a node position");
        model["nodes"][0]["position"]["x"] = json!(x + 0.001);
        model
    }

    #[test]
    fn a_substituted_model_refuses_at_the_native_binding() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let model = substituted_model(&w.model);
        let mut request = bound(&w.request, &live.job);
        // The model hashes rebound to the substituted model.
        let model_sha256 = digest(&model).unwrap();
        request["report"]["exact_route"]["admission"]["model_sha256"] = json!(model_sha256);
        request["report"]["calculation_report"]["model_input_summary"]["model_hash"]["value"] = json!(model_sha256);
        request["audit_manifest"]["model_hash"]["value"] = json!(model_sha256);
        // 3. With the genuine intake text: step 5 (model).
        let (code, message) = save_code(&live, save_args(&request, &w, &model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED, "{message}");
        assert!(message.contains(": model:"), "{message}");
        let refused = refusal_text(witness_render_on(&live, &render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: model:")), "{refused}");
        // With its own text as the intake text: step 2.
        let own = serde_json::to_string(&model).unwrap();
        let (code, message) = save_code(&live, save_args(&request, &w, &model, Some(&w.result), Some(&own)));
        assert_eq!(code, SOURCE_NOT_DELIVERED);
        assert!(message.contains(": model:"), "{message}");
        let refused = refusal_text(witness_render_on(&live, &render_args(&request["report"], Some(&w.result), Some(&own), Some(&canonical_json(&w.manifest)))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: model:")), "{refused}");
    }

    #[test]
    fn the_genuine_input_refuses_once_custody_is_released() {
        let w = witness(STRAIGHT);
        for dispose_all in [false, true] {
            let live = live(&w);
            if dispose_all {
                live.registry.dispose_all_results();
            } else {
                live.registry.dispose_result(&live.job).unwrap();
            }
            let request = bound(&w.request, &live.job);
            let (code, _) = save_code(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)));
            assert_eq!(code, CUSTODY_RELEASED);
            let refused = refusal_text(witness_render_on(&live, &genuine_render_args(&w, &live.job)));
            assert!(refused.starts_with(&format!("{CUSTODY_RELEASED}: ")), "{refused}");
        }
    }

    #[test]
    fn a_delivered_pressure_source_1_result_with_load_reference_states_is_not_eligible() {
        let w = witness(SOURCE_1);
        let lr_raw = std::fs::read_to_string(project("fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json")).unwrap();
        let lr_request: Value = serde_json::from_str(&std::fs::read_to_string(project("fixtures/product_preview/pressure_source/load_reference_source/n05.request.json")).unwrap()).unwrap();
        let intake = serde_json::to_string(&lr_request["model"]).unwrap();
        let live = live_on(ScriptedHost::normal(), &intake, "sparse_interactive", &lr_raw);
        let mut request = bound(&w.request, &live.job);
        request["report"]["exact_route"]["admission"]["received_bytes_sha256"] = json!(sha256_hex(&lr_raw));
        let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&lr_raw), Some(&intake)));
        assert_eq!(code, STANDING_NOT_ELIGIBLE, "{message}");
        let refused = refusal_text(witness_render_on(&live, &render_args(&request["report"], Some(&lr_raw), Some(&intake), Some(&canonical_json(&w.manifest)))));
        assert_eq!(refused, format!("{STANDING_NOT_ELIGIBLE}: the native reading of the delivered result is not numerically eligible."));
        // The genuine pressure-source-1 witness without load/reference states is eligible:
        // its request passes the binding and refuses at item 0.
        let genuine = live_on(ScriptedHost::normal(), &w.intake, &w.mode, &w.result);
        let (code, message) = save_code(&genuine, save_args(&bound(&w.request, &genuine.job), &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, ROUTE_NOT_ADMITTED, "{message}");
    }

    #[test]
    fn the_render_re_derives_its_rows_limitations_and_headlines_from_the_delivered_result() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let manifest = canonical_json(&w.manifest);
        let genuine = bound(&w.request, &live.job)["report"].clone();
        let edits: Vec<(&str, Box<dyn Fn(&mut Value)>, String)> = vec![
            ("a row", Box::new(|input: &mut Value| input["result_rows"][0]["quantity_display"] = json!("1 Pa")), format!("{SECTIONS_MISMATCH}: result_rows")),
            (
                "an exact limitation statement",
                Box::new(|input: &mut Value| {
                    let statement = input["report_sections"]["limitations"][2]["statement"].as_str().unwrap().replace("Pressure", "Pressure loading");
                    input["report_sections"]["limitations"][2]["statement"] = json!(statement);
                }),
                String::new(),
            ),
            ("a headline", Box::new(|input: &mut Value| input["exact_route"]["headlines"][0]["value_display"] = json!("1 Pa")), format!("{SECTIONS_MISMATCH}: headlines")),
        ];
        for (label, edit, expected) in edits {
            let mut input = genuine.clone();
            edit(&mut input);
            let refused = refusal_text(witness_render_on(&live, &render_args(&input, Some(&w.result), Some(&w.intake), Some(&manifest))));
            if expected.is_empty() {
                assert!(refused.starts_with(&format!("{SECTIONS_MISMATCH}: limitations: limitation:")), "{label}: {refused}");
            } else {
                assert_eq!(refused, expected, "{label}");
            }
        }
        // `model_sha256` changed: step 5 refuses it before the re-derivation
        // (§3.6 step 5, as design §7 states for this case).
        let mut input = genuine.clone();
        input["exact_route"]["admission"]["model_sha256"] = json!("0".repeat(64));
        let refused = refusal_text(witness_render_on(&live, &render_args(&input, Some(&w.result), Some(&w.intake), Some(&manifest))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: model:")), "{refused}");
        // Another input manifest: its digest is not the binding's.
        let mut other = w.manifest.clone();
        other["invented_extra"] = json!(true);
        let refused = refusal_text(witness_render_on(&live, &render_args(&genuine, Some(&w.result), Some(&w.intake), Some(&canonical_json(&other)))));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: admission.input_manifest_sha256"));
        // The texts without exact_route (a pressure-free input).
        let refused = refusal_text(witness_render_on(&live, &render_args(&pressure_free_input(), Some(&w.result), Some(&w.intake), Some(&manifest))));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: exact_route"));
        let refused = refusal_text(production_render_on(&live, &render_args(&pressure_free_input(), Some(&w.result), None, None)));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: exact_route"));
    }

    #[test]
    fn the_save_refuses_when_custody_ends_during_its_dialog_and_writes_nothing() {
        use crate::report_package_save::test_support::{desktop_request, model, sandbox, save_request};
        let w = witness(STRAIGHT);
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        for release in [false, true] {
            let live = live(&w);
            let digests = live.registry.custody_binding(&live.job).unwrap();
            // A pressure-free admitted package carrying an exact custody record:
            // the post-dialog re-check is the only exact step on this path.
            let mut admitted = crate::report_package_save::admit_report_package(save_request(&desktop_request(&model()), &model()), &software(), &mut no_store).unwrap();
            admitted.exact_custody = Some((live.job.clone(), digests));
            let root = sandbox(if release { "custody-ends" } else { "custody-held" });
            let target = root.join("exact.opsproj");
            let custody = |job: &str| live.registry.custody_binding(job);
            let outcome = save_admitted_report_package_in(admitted, &custody, |_| {
                if release {
                    live.registry.dispose_result(&live.job).unwrap();
                }
                Some(Ok(target.clone()))
            });
            if release {
                let error = outcome.expect_err("custody ended during the dialog");
                assert_eq!(error.code, CUSTODY_RELEASED);
                assert!(!target.exists(), "nothing is written");
            } else {
                assert_eq!(outcome.unwrap().outcome, "saved");
                assert!(target.exists());
            }
            std::fs::remove_dir_all(&root).unwrap();
        }
    }

    /// The regression check for the revision-4 gate finding: the production
    /// native render refuses a genuine exact input from a live job, with its
    /// genuine texts, at step 0, before any parse, lease growth or custody use.
    #[test]
    fn the_production_native_render_refuses_a_genuine_exact_result() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let args = genuine_render_args(&w, &live.job);
        let before = live.registry.custody_binding(&live.job);
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
        let admitted_bytes = lease.bytes();
        let (outcome, measured) = measure(|| crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease));
        assert_eq!(refusal_text(outcome), PRODUCTION_REFUSAL);
        assert_eq!(lease.bytes(), admitted_bytes, "the lease does not grow");
        assert!(measured.peak_requested < (w.result.len() as u64) / 4, "neither text is parsed: {measured:?}");
        assert_eq!(live.registry.custody_binding(&live.job), before, "custody is unchanged");
        drop(lease);
        // From the raw body, as the command does.
        let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
        let (mut decoded, mut lease) =
            admit_body(&live.registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "render_calculation_report", ArgsDecoding::Moved).unwrap();
        assert_eq!(refusal_text(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease)), PRODUCTION_REFUSAL);
        // An exact_route naming another identity refuses likewise; under the
        // witness set, preview-physics-2 and an invented identity still refuse.
        for (id, witness_refuses) in [
            (PRESSURE_SOURCE_ID, false),
            (open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_ID, true),
            ("openpipestress.result_semantics/0.3.0/invented-1", true),
        ] {
            let mut args = args.clone();
            args["input"]["exact_route"]["semantic_contract_id"] = json!(id);
            let refused = refusal_text(production_render_on(&live, &args));
            assert!(refused.starts_with(RENDER_ROUTE_NOT_ADMITTED), "{id}: {refused}");
            if witness_refuses {
                let refused = refusal_text(witness_render_on(&live, &args));
                assert!(refused.starts_with(RENDER_ROUTE_NOT_ADMITTED), "{id}: {refused}");
            }
        }
        // `RENDER_ADMITTED_EXACT_CONTRACTS` is empty.
        assert!(crate::report_render_exact::RENDER_ADMITTED_EXACT_CONTRACTS.is_empty());
    }

    #[test]
    fn the_exact_texts_are_priced_before_any_parse() {
        let w = witness(STRAIGHT);
        let request = canonical_json(&bound(&w.request, "job"));
        let model = canonical_json(&w.model);
        let manifest = canonical_json(&w.manifest);
        let base = parse_lease_bytes(&request, &model, &manifest).unwrap();
        let price = |text: &str| text_parse_bytes(text, ArgsDecoding::Moved).unwrap();
        assert_eq!(
            parse_lease_bytes_with_exact(&request, &model, &manifest, Some(&w.result), Some(&w.intake)).unwrap(),
            base + price(&w.result) + price(&w.intake)
        );
        assert_eq!(parse_lease_bytes_with_exact(&request, &model, &manifest, None, None).unwrap(), base);
        let m_r = crate::ingress::prescan(w.result.as_bytes()).unwrap().materialization_bytes;
        assert_eq!(
            exact_render_growth_bytes(&w.result, &w.intake, &manifest).unwrap(),
            price(&w.result) + price(&w.intake) + price(&manifest) + REPORT_PACKAGE_WORKING_MEMORY_FACTOR * m_r
        );
        // Oversize texts refuse with the save's TOO_LARGE before any parse.
        let over = " ".repeat(MAX_MODEL_TEXT_BYTES as usize + 1);
        assert_eq!(parse_lease_bytes_with_exact(&request, &model, &manifest, None, Some(&over)).unwrap_err().code, TOO_LARGE);
        let live = live(&w);
        let mut args = genuine_render_args(&w, &live.job);
        args["exactIntakeText"] = json!(over);
        let refused = refusal_text(witness_render_on(&live, &args));
        assert!(refused.starts_with(TOO_LARGE), "{}", &refused[..refused.len().min(120)]);
    }

    #[test]
    fn a_refused_growth_refuses_with_its_resource_refusal_and_parses_nothing() {
        let w = witness(STRAIGHT);
        let host = ScriptedHost::normal();
        let live = live_on(host.clone(), &w.intake, &w.mode, &w.result);
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &genuine_render_args(&w, &live.job)).unwrap();
        host.set(|s| s.headroom_bytes = Some(0));
        let admitted_bytes = lease.bytes();
        let (outcome, measured) = measure(|| render_calculation_report_args_in(&mut decoded, &live.registry, &mut lease, EXACT_RENDER_WITNESS_ADMITTED));
        match outcome {
            Err(CommandError::Ingress(crate::solve_jobs::SolveJobError::ResourceUnavailable { .. })) => {}
            other => panic!("not a resource refusal: {other:?}"),
        }
        assert_eq!(lease.bytes(), admitted_bytes);
        assert!(measured.peak_requested < (w.result.len() as u64) / 4, "nothing is parsed: {measured:?}");
    }

    /// The whole-command counting-allocator check of the genuine exact render
    /// (from the raw body through the ingress admission and the witness path)
    /// and of the genuine exact save (to its item-0 refusal): each priced peak
    /// is within its lease, less the buffered body's envelope, which is
    /// allocated before the command runs. W M_r and the save's W M are measured
    /// provisions for these witness inputs, not proven bounds (§3.6).
    #[test]
    fn the_genuine_exact_render_and_save_stay_within_their_leases() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let raw = serde_json::to_vec(&genuine_render_args(&w, &live.job)).unwrap();
        let envelope = crate::ingress::allocation_envelope(raw.len() as u64).unwrap();
        let body = InvokeBody::Raw(raw);
        let headers = headers(Some(INGRESS_ARGS_JSON_V1));
        let ((response, lease_bytes), measured) = measure(|| {
            let (mut decoded, mut lease) = admit_body(&live.registry, &body, &headers, "render_calculation_report", ArgsDecoding::Moved).unwrap();
            let response = respond(render_calculation_report_args_in(&mut decoded, &live.registry, &mut lease, EXACT_RENDER_WITNESS_ADMITTED)).map(response_text);
            drop(decoded);
            (response, lease.bytes())
        });
        let response = response.expect("the genuine input renders");
        assert!(response.contains("\"export_blocked\":false"));
        assert!(measured.peak_priced <= lease_bytes - envelope, "render: priced peak {} exceeds its lease {}", measured.peak_priced, lease_bytes - envelope);
        eprintln!("exact render: text {} priced peak {} lease {}", w.result.len(), measured.peak_priced, lease_bytes - envelope);

        let args = save_args(&bound(&w.request, &live.job), &w, &w.model, Some(&w.result), Some(&w.intake));
        let mut raw_body = crate::report_package_save::test_support::args_body(&args);
        raw_body["exactResultText"] = json!(w.result);
        raw_body["exactIntakeText"] = json!(w.intake);
        let raw = serde_json::to_vec(&raw_body).unwrap();
        let envelope = crate::ingress::allocation_envelope(raw.len() as u64).unwrap();
        let body = InvokeBody::Raw(raw);
        let custody = |job: &str| live.registry.custody_binding(job);
        let ((code, lease_bytes), measured) = measure(|| {
            let (mut decoded, mut lease) = admit_body(&live.registry, &body, &headers, "save_report_package", ArgsDecoding::Moved).unwrap();
            let request = crate::report_package_save::save_report_package_args(&mut decoded).unwrap();
            drop(decoded);
            let growth = parse_lease_bytes_with_exact(
                &request.request_text,
                &request.model_text,
                &request.input_manifest_text,
                request.exact_result_text.as_deref(),
                request.exact_intake_text.as_deref(),
            )
            .unwrap();
            lease.grow_to(lease.bytes() + growth).unwrap();
            let code = admit_report_package_in(request, &software(), &mut no_store, &custody).map(drop).unwrap_err().code;
            (code, lease.bytes())
        });
        assert_eq!(code, ROUTE_NOT_ADMITTED);
        assert!(measured.peak_priced <= lease_bytes - envelope, "save: priced peak {} exceeds its lease {}", measured.peak_priced, lease_bytes - envelope);
        eprintln!("exact save: priced peak {} lease {}", measured.peak_priced, lease_bytes - envelope);
    }
}
