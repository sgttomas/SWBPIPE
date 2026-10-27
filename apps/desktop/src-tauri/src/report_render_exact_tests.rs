//! The native render's identity boundary (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §3.6 step 0, §5.1, §5.5). Since the owner
//! opened exact-route report outputs (§5.1; owner, 2026-10-26: "My two
//! decisions are to approve your recommendations in both cases. Proceed
//! accordingly."), the production command admits pressure-2 and
//! pressure-source-1 at step 0 and binds them natively before rendering, so a
//! coherent invented exact-route input refuses at its custody binding; a
//! non-exact input renders as before; every other exact identity and each
//! structured marker without an admitted `exact_route` refuses while free text
//! equal to an exact identity does not; and the render's admitted-set
//! forwarding graph is closed, with no test-only set. Every id and digest is
//! invented. Standard claim fence applies (claims taxonomy per DEC-081).

#[path = "../../../../core/reporting/report_package/src/forwarding_graph_scan.rs"]
mod forwarding_graph_scan;

use crate::command_ingress::test_support::{admit_json, registry};
use crate::command_ingress::{respond, ArgsDecoding, CommandError};
use crate::ingress::counting::measure;
use crate::memory_pool::test_host::ScriptedHost;
use crate::report_render_exact::{check_render_route_admitted, EXACT_IDENTITIES, PRODUCTION, RENDER_ROUTE_NOT_ADMITTED};
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

/// The production step 0's refusal of an exact identity outside its set
/// (physics-retained-1), naming the admitted set (§5.1 item 4).
const RETAINED_REFUSAL: &str = "REPORT-RENDER-ROUTE-NOT-ADMITTED: the rendered report admits only the exact-route identities openpipestress.result_semantics/0.3.0/pressure-2, openpipestress.result_semantics/0.3.0/pressure-source-1; found openpipestress.result_semantics/0.3.0/physics-retained-1.";
/// The same refusal's prefix, for any `found`.
const ADMITS_ONLY: &str = "REPORT-RENDER-ROUTE-NOT-ADMITTED: the rendered report admits only the exact-route identities openpipestress.result_semantics/0.3.0/pressure-2, openpipestress.result_semantics/0.3.0/pressure-source-1; found";

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

/// An invented report context under `export_context` (as the provenance
/// refusal test builds it).
fn report_context(export_context: &str) -> Value {
    json!({
        "export_context": export_context, "generated_at": null, "software_name": "Invented", "software_version": null,
        "build_release_identifier": null, "solver_name": null, "solver_version": null, "solver_build_ref": null,
        "analysis_run_name": null, "analysis_run_recorded_at": null, "solver_mode": null, "solver_settings_ref": null,
        "solver_settings": [], "model_coordinate_system": null, "result_coordinate_systems": []
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
    // Codex review round 2 of #1639: a refusal for the record is metadata-only
    // at the native boundary. No field of the response (html, derived print
    // view, findings, diagnostics) carries the record's text or typed path.
    let withheld = |refused: &Value, label: &str| {
        assert_eq!(refused["export_blocked"], json!(true), "{label}");
        assert_eq!(refused["content_withheld"], json!(true), "{label}");
        assert_eq!(refused["html"], json!(""), "{label}");
        assert_eq!(refused["derived_print_html"], json!(""), "{label}");
        assert_eq!(refused["pre_render_findings"], json!([]), "{label}");
        assert_eq!(refused["post_render_findings"], json!([]), "{label}");
        let text = refused.to_string();
        for private in ["/Users/invented-user/datasheets/steel-a.pdf", "steel-a.pdf", "Invented material datasheet", "invented-license", "invented contributor", "invented certification"] {
            assert!(!text.contains(private), "{label}: {private} in {text}");
        }
    };
    for export_context in [Some("public_report"), None] {
        let refused = outcome(&input("pending", export_context));
        assert!(
            reasons(&refused).iter().any(|reason| reason.starts_with("report provenance: REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE (")),
            "{:?}",
            reasons(&refused)
        );
        withheld(&refused, &format!("{export_context:?}"));
    }
    for export_context in [Some("local_private"), Some("public_report"), None] {
        let refused = outcome(&input("quarantined", export_context));
        assert!(reasons(&refused).iter().any(|reason| reason.contains("REPORT_PROVENANCE_DISPOSITION_BLOCKED (provenance_notes[1] review_status quarantined;")));
        withheld(&refused, &format!("quarantined {export_context:?}"));
    }
    // Other blocked renders keep their diagnostic document: a rule-check run
    // for a pack the report does not reference blocks, and still returns its
    // HTML and print view (behaviour unchanged).
    let mut unreferenced = pressure_free_input();
    unreferenced["user_rule_checks"] = json!({"rule_pack_id": "invented-unreferenced-pack", "grammar_version": "1.0.0", "aggregate_status": "USER_RULE_CHECKED", "checks": []});
    let blocked = outcome(&json!({"input": unreferenced}));
    assert_eq!(blocked["export_blocked"], json!(true), "{:?}", reasons(&blocked));
    assert!(blocked.get("content_withheld").is_none());
    assert_ne!(blocked["html"], json!(""));
    assert_ne!(blocked["derived_print_html"], json!(""));
    // A rendered (not refused) response carries no content_withheld member.
    assert!(local.get("content_withheld").is_none());
}

/// Codex review round 3 of #1639: a refusal for provenance records names
/// records by position only. Authored ids (value ids for each entity kind, an
/// assumption id, library owners) carrying a long path, a short path and
/// private text reach no field of the native response, under public_report,
/// without a context and for a quarantined record under local_private.
#[test]
fn the_native_render_response_names_no_authored_id() {
    let texts = ["/Users/invented-user/private-datasheets/steel-a.pdf", "C:/x.pdf", "Invented Private Client Project"];
    let record = json!({
        "source_name": "Invented material datasheet", "source_location": "invented-datasheet-shelf", "source_license": "invented-license",
        "contributor": "invented contributor", "contributor_certification": "invented certification",
        "redistribution_status": "private_only", "review_status": "quarantined", "privacy_classification": "private_project_data"
    });
    for export_context in [Some("public_report"), None, Some("local_private")] {
        let mut input = pressure_free_input();
        let sections = &mut input["report_sections"];
        for (kind, id) in [("material", texts[0]), ("section", texts[1]), ("component", texts[2]), ("support", texts[0])] {
            sections["user_supplied_values"].as_array_mut().unwrap().push(json!({
                "value_id": format!("{kind}-provenance:{kind}:{id}"), "value_category": "report_metadata",
                "source": {"ref_type": kind, "ref_id": format!("{kind}:{id}")}, "provenance": record.clone(),
                "privacy_classification": "private_project_data", "required_for": ["reporting"],
                "review_status": "quarantined", "missing_data_finding": false
            }));
        }
        sections["assumptions"] = json!([{
            "assumption_id": format!("assumption:{} {}", texts[2], texts[1]), "owner": "invented owner",
            "source": {"ref_type": "model", "ref_id": "invented"}, "affected_scope": {"ref_type": "model", "ref_id": "invented"},
            "statement": "Invented assumption.", "review_status": "quarantined", "expiration_or_re_review_trigger": null,
            "effect": {"mechanics_solve_qualified": true, "user_rule_check_qualified": false, "report_completeness": "qualified", "human_review_required": true},
            "provenance": sections["provenance_notes"][0].clone()
        }]);
        input["library_references"] = json!([{
            "reference_id": format!("library-ref:{}", texts[1]), "library_kind": "material", "library_id": texts[2],
            "library_version": "not recorded", "record_id": texts[0], "entity_refs": [format!("material:{}", texts[0])], "provenance": record.clone()
        }]);
        if let Some(export_context) = export_context {
            input["report_context"] = json!({
                "export_context": export_context, "generated_at": null, "software_name": "Invented", "software_version": null,
                "build_release_identifier": null, "solver_name": null, "solver_version": null, "solver_build_ref": null,
                "analysis_run_name": null, "analysis_run_recorded_at": null, "solver_mode": null, "solver_settings_ref": null,
                "solver_settings": [], "model_coordinate_system": null, "result_coordinate_systems": []
            });
        }
        let text = command_body(&json!({"input": input})).expect("the render answers");
        let response: Value = serde_json::from_str(&text).unwrap();
        assert_eq!((response["export_blocked"].clone(), response["content_withheld"].clone()), (json!(true), json!(true)), "{export_context:?}");
        for private in texts {
            assert!(!text.contains(private), "{export_context:?}: {private} in {text}");
        }
        assert!(text.contains("(assumptions[0] review_status quarantined;"), "{text}");
    }
}

/// Codex review round 3 of #1639 (non-blocking): an input that does not
/// decode is refused with its JSON path and the error kind only. A typed path
/// supplied as an invalid disposition value (review, redistribution, privacy)
/// is not echoed.
#[test]
fn an_undecodable_input_names_its_path_and_kind_not_its_value() {
    let typed = "/Users/invented-user/private-datasheets/steel-a.pdf";
    for (member, path) in [
        ("review_status", "report_sections.provenance_notes[0].review_status"),
        ("redistribution_status", "report_sections.provenance_notes[0].redistribution_status"),
        ("privacy_classification", "report_sections.provenance_notes[0].privacy_classification"),
    ] {
        let mut input = pressure_free_input();
        input["report_sections"]["provenance_notes"][0][member] = json!(typed);
        let message = refusal_text(command_body(&json!({"input": input})));
        assert_eq!(message, format!("RENDER-INPUT-INVALID: unknown variant at {path}"), "{member}");
    }
    let mut input = pressure_free_input();
    input["report_sections"]["user_supplied_values"] = json!([{
        "value_id": "material-provenance:material:a", "value_category": "report_metadata",
        "source": {"ref_type": "material", "ref_id": "material:a"},
        "provenance": {"source_name": "a", "source_location": "b", "source_license": "c", "contributor": "d", "contributor_certification": "e",
            "redistribution_status": "private_only", "review_status": typed, "privacy_classification": "private_project_data"},
        "privacy_classification": "private_project_data", "required_for": ["reporting"], "review_status": "pending", "missing_data_finding": false
    }]);
    let message = refusal_text(command_body(&json!({"input": input})));
    assert_eq!(message, "RENDER-INPUT-INVALID: unknown variant at report_sections.user_supplied_values[0].provenance.review_status");
    let mut input = pressure_free_input();
    input["result_rows"][0]["label"] = json!(7);
    assert_eq!(refusal_text(command_body(&json!({"input": input}))), "RENDER-INPUT-INVALID: invalid type at result_rows");
    assert!(!refusal_text(command_body(&json!({"input": {"report_title": typed}}))).contains(typed));
}

/// Codex review round 1 (blocking), after opening: a coherent invented exact
/// input, which the renderer itself renders unblocked with "integrity
/// checked", passes step 0 in production and is bound to its source natively
/// first; without a report context it refuses before that, and under
/// local_private its invented job is not in custody, so the production
/// command refuses it there (`REPORT-EXACT-CUSTODY-RELEASED`) with no HTML. The
/// genuine live-job input renders (`direct_native`). The same input relabelled
/// physics-retained-1 still refuses at step 0.
#[test]
fn production_native_render_binds_a_coherent_exact_input_before_rendering() {
    let exact = with_exact_route(pressure_free_input());
    // Coherent: the renderer itself renders it unblocked, with the exact block
    // and the composed standing line.
    let rendered = crate::render_calculation_report_value(exact.clone()).expect("the renderer renders it");
    assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
    let html = rendered["html"].as_str().unwrap();
    assert!(html.contains(EXACT_ROUTE_SUBHEADING) && html.contains("Numerical standing: integrity checked"));
    assert_eq!(check_render_route_admitted(&exact, PRODUCTION), Ok(()));
    // Without a report context, the exact render refuses before its binding
    // (Codex review round 1 of #1648).
    let refused = refusal_text(command_body(&json!({"input": exact})));
    assert!(refused.starts_with("REPORT-RENDER-EXACT-CONTEXT-NOT-LOCAL-PRIVATE: "), "{refused}");
    // Under local_private, the production root (through the ingress decode)
    // and the command body on the raw args body: the native source binding
    // refuses the invented job.
    let mut exact = exact;
    exact["report_context"] = report_context("local_private");
    let registry = registry(ScriptedHost::normal());
    let (mut args, mut lease) =
        admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": exact})).unwrap();
    let refused = refusal_text(crate::render_calculation_report_args(&mut args, &registry, &mut lease));
    assert!(refused.starts_with("REPORT-EXACT-CUSTODY-RELEASED: "), "{refused}");
    let refused = refusal_text(command_body(&json!({"input": exact})));
    assert!(refused.starts_with("REPORT-EXACT-CUSTODY-RELEASED: "), "{refused}");
    // physics-retained-1 is outside the admitted set.
    let mut retained = exact.clone();
    retained["exact_route"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    retained["exact_route"]["admission"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    assert_eq!(refusal_text(command_body(&json!({"input": retained}))), RETAINED_REFUSAL);
    assert_eq!(serde_json::to_value(CommandError::<String>::Command(RETAINED_REFUSAL.into())).unwrap(), json!(RETAINED_REFUSAL));
}

/// Step 0 runs before the renderer's own decode: an exact input outside the
/// admitted set that would not even decode refuses with the route code, not
/// `RENDER-INPUT-INVALID`.
#[test]
fn step_zero_precedes_the_render_input_decode() {
    let broken = json!({"report_title": "broken", "exact_route": {"semantic_contract_id": PHYSICS_RETAINED_ID}});
    assert_eq!(refusal_text(command_body(&json!({"input": broken}))), RETAINED_REFUSAL);
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

/// Each structured marker refuses in production unless it is an admitted
/// `exact_route`, through the production root and the raw body: an
/// `exact_route` that names no identity or is not an object, an exact
/// identity outside the admitted set at an identity field
/// (`exact_route.semantic_contract_id`, `exact_route.admission.semantic_contract_id`,
/// a `semantic_contract` limitation source), exact identities at limitation
/// sources without `exact_route`, and exact limitation ids without
/// `exact_route`.
#[test]
fn every_structured_marker_refuses_in_production() {
    let exact = with_exact_route(pressure_free_input());
    let no_identity_text = format!("{ADMITS_ONLY} an exact_route section that names no identity.");
    let found = |id: &str| format!("{ADMITS_ONLY} {id}.");
    let mut no_identity = exact.clone();
    no_identity["exact_route"].as_object_mut().unwrap().remove("semantic_contract_id");
    let mut empty_route = pressure_free_input();
    empty_route["exact_route"] = json!({});
    let mut text_route = pressure_free_input();
    text_route["exact_route"] = json!("invented");
    let mut retained_route = exact.clone();
    retained_route["exact_route"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    retained_route["exact_route"]["admission"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    // An admitted route that names an unadmitted identity at another identity field.
    let mut retained_admission = exact.clone();
    retained_admission["exact_route"]["admission"]["semantic_contract_id"] = json!(PHYSICS_RETAINED_ID);
    let mut retained_source = exact.clone();
    retained_source["report_sections"]["limitations"][1]["source"]["ref_id"] = json!(PHYSICS_RETAINED_ID);
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
        ("an exact_route that names no identity", &no_identity, no_identity_text.clone()),
        ("an empty exact_route", &empty_route, no_identity_text.clone()),
        ("an exact_route that is not an object", &text_route, no_identity_text),
        ("an exact_route naming physics-retained-1", &retained_route, found(PHYSICS_RETAINED_ID)),
        ("an admission naming physics-retained-1", &retained_admission, found(PHYSICS_RETAINED_ID)),
        ("a limitation source naming physics-retained-1", &retained_source, found(PHYSICS_RETAINED_ID)),
        ("exact limitation entries without exact_route", &limitations_only, found(PRESSURE_ID)),
        ("a semantic_contract limitation source", &source_only, found(PRESSURE_SOURCE_ID)),
        (
            "exact limitation ids without exact_route",
            &ids_only,
            format!("{ADMITS_ONLY} the exact-route limitation entry limitation:invented-run:exact:1 without an exact_route section."),
        ),
    ];
    for (case, input, expected) in cases {
        assert_eq!(check_render_route_admitted(input, PRODUCTION).unwrap_err(), expected, "{case}");
        assert_eq!(production_refusal(input), expected, "{case}");
    }
    // The coherent exact input passes step 0 (pressure-2 and pressure-source-1
    // are admitted), and the admitting decision allocates nothing.
    let mut source_route = exact.clone();
    source_route["exact_route"]["semantic_contract_id"] = json!(PRESSURE_SOURCE_ID);
    source_route["exact_route"]["admission"]["semantic_contract_id"] = json!(PRESSURE_SOURCE_ID);
    for (id, input) in [(PRESSURE_ID, &exact), (PRESSURE_SOURCE_ID, &source_route)] {
        let (admitted, measured) = measure(|| check_render_route_admitted(input, PRODUCTION));
        assert!(admitted.is_ok(), "{id}: {admitted:?}");
        assert_eq!(measured.allocations, 0, "{id}: {measured:?}");
    }
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

const GRAPH: Graph = Graph { rows: &ROWS, set_type: "RenderAdmittedSet", set_module: "report_render_exact", scanner_files: &[SCANNER] };

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
            file("fn w() { let wide = RenderAdmittedSet(&[A, B]); }\n"),
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
            "a test-only widened set",
            edit(
                "report_render_exact.rs",
                "pub(crate) const PRODUCTION: RenderAdmittedSet = RenderAdmittedSet(&RENDER_ADMITTED_EXACT_CONTRACTS);\n",
                "pub(crate) const PRODUCTION: RenderAdmittedSet = RenderAdmittedSet(&RENDER_ADMITTED_EXACT_CONTRACTS);\n#[cfg(test)]\npub(crate) const EXACT_RENDER_WITNESS_ADMITTED: RenderAdmittedSet = RenderAdmittedSet(&[A, B]);\n",
            ),
            "report_render_exact.rs:4: a set of type RenderAdmittedSet other than PRODUCTION",
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
// W-11: the native source binding (DEL-08-01 Design
// `exact-route-report-sections.md` §3.6, §5.1, §5.5, §7 "Direct-native
// substitution" and "Pricing"), through the production roots since the owner
// opened exact-route report outputs (2026-10-26): the render through
// `render_calculation_report_args` and the save through `admit_report_package_in`
// to the assembled package. A test registry job is admitted from a witness's
// intake model text, delivers the committed producer bytes through the gated
// test launcher and is taken into custody. The witness requests are the ones
// the desktop's vitest writes
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
    const ROWS_MISMATCH: &str = "REPORT-PACKAGE-REPORT-ROWS-MISMATCH";
    /// The standing line's start, HTML-escaped: without an integrity warning,
    /// and on pressure-source-1 (synthetic n05), whose Diagnostics carry
    /// `NUMERICAL_INTEGRITY_SENSITIVE` (§3.2).
    const PLAIN_STANDING: &str = "Numerical standing: integrity checked (the Rust reader&#39;s standing for the result";
    const SENSITIVE_STANDING: &str = "Numerical standing: integrity warning present in the Diagnostics: NUMERICAL_INTEGRITY_SENSITIVE (see the Diagnostics table under Warnings, Assumptions, And Provenance); integrity checked (the Rust reader&#39;s standing for the result";

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

    /// The production save's admission of `args`: the assembled, unblocked
    /// package, read back by the production reader, with its exact-identity
    /// result member and the custody record its binding checked.
    fn save_admitted(live: &Live, args: ReportPackageSaveRequest, name: &str) -> crate::report_package_save::AdmittedReportPackage {
        let custody = |job: &str| live.registry.custody_binding(job);
        let admitted = admit_report_package_in(args, &software(), &mut no_store, &custody)
            .unwrap_or_else(|error| panic!("{name}: {} {}", error.code, error.message));
        assert!(!admitted.package.export_blocked, "{name}: {:?}", admitted.package.blocking_reasons);
        assert_eq!(admitted.exact_custody.as_ref().map(|(job, _)| job.as_str()), Some(live.job.as_str()), "{name}");
        let read = open_pipe_stress_report_package::read_report_package_container(&admitted.package.container_bytes)
            .unwrap_or_else(|refusal| panic!("{name}: {refusal:?}"));
        assert!(!read.export_blocked, "{name}");
        let member = read.members.iter().find(|member| member.role == open_pipe_stress_report_package::ROLE_RESULT_EXPORT).expect("a result member");
        let document: Value = serde_json::from_slice(&member.bytes).unwrap();
        assert!(EXACT_IDENTITIES[..2].contains(&document["result_envelope"]["producer"]["semantic_contract_id"].as_str().unwrap()), "{name}");
        admitted
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

    fn production_render_on(live: &Live, args: &Value) -> Result<Value, CommandError> {
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, args).unwrap();
        crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease)
    }

    fn genuine_render_args(w: &Witness, job: &str) -> Value {
        let request = bound(&w.request, job);
        render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)))
    }

    #[test]
    fn the_genuine_request_passes_the_native_binding_and_is_admitted() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let request = bound(&w.request, &live.job);
        save_admitted(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)), "straight");
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

    /// Every witness shape (§5.5), through the production roots: with its live
    /// job and genuine texts, the render input renders (HTML and the derived
    /// print view, export gates passed, no numerical-health line), and the
    /// save request passes the native binding and is assembled into an
    /// unblocked package that the production reader reads back.
    #[test]
    fn every_witness_shape_binds_natively_renders_and_saves_through_the_production_roots() {
        for (name, text) in SHAPES {
            let w = witness(text);
            let live = live(&w);
            let rendered = production_render_on(&live, &genuine_render_args(&w, &live.job)).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            assert_eq!(rendered["export_blocked"], json!(false), "{name}: {}", rendered["blocking_reasons"]);
            let html = rendered["html"].as_str().unwrap();
            // Observation 4 (owner, 2026-10-26, option A; §3.2): beside an
            // integrity warning the standing line names it and points to it.
            let standing = if name == "pressure_source_1" { SENSITIVE_STANDING } else { PLAIN_STANDING };
            assert!(html.contains(EXACT_ROUTE_SUBHEADING) && html.contains(standing), "{name}");
            if name == "pressure_source_1" {
                assert!(!html.contains("Numerical standing: integrity checked"), "{name}: never only \"integrity checked\" beside an integrity warning");
            }
            assert!(!html.contains("numerical-health:"), "{name}: an exact report carries no numerical-health line");
            assert!(rendered["derived_print_html"].as_str().is_some_and(|print| print.contains(EXACT_ROUTE_SUBHEADING) && print.contains(standing)), "{name}");
            let admitted = save_admitted(&live, save_args(&bound(&w.request, &live.job), &w, &w.model, Some(&w.result), Some(&w.intake)), name);
            let member = admitted.package.members.iter().find(|member| member.role == open_pipe_stress_report_package::ROLE_REPORT_HTML).unwrap();
            let package_html = String::from_utf8(member.bytes.clone()).unwrap();
            assert!(package_html.contains(standing), "{name}: the package's HTML member");
            // Readability (DEL-08-01 Design `exact-route-report-sections.md` §3.7, §6.3
            // items 1-3), in the native render and the saved package alike: the display
            // rule statement, the headlines in MPa, the run-name note with this run's
            // received-result digest, and the integrity evidence in full text.
            let received = w.request["report"]["exact_route"]["admission"]["received_result_sha256"].as_str().unwrap();
            let note = format!("This run is identified by its received result sha256 {received}.");
            for document in [html, package_html.as_str()] {
                assert!(document.contains(open_pipe_stress_report_renderer::DISPLAY_RULE_STATEMENT), "{name}");
                assert!(document.contains(&note) && document.contains("Run name: run:preview-linear-static-001 is the producer&#39;s fixed run name"), "{name}");
                assert!(document.contains(&format!("<h3>{}</h3>", open_pipe_stress_report_renderer::DIAGNOSTIC_EVIDENCE_SUBHEADING)), "{name}");
                assert!(document.contains("full text: Diagnostic evidence E1"), "{name}");
                assert!(!document.contains(" Pa</td>"), "{name}: no value is displayed in Pa");
            }
            for diagnostic in w.request["report"]["report_sections"]["diagnostics"].as_array().unwrap() {
                let message = open_pipe_stress_report_renderer::escape_html(diagnostic["message"].as_str().unwrap());
                assert!(html.contains(&message) && package_html.contains(&message), "{name}: {} kept verbatim", diagnostic["code"]);
            }
        }
    }

    /// W-11 opening, §5.5 evidence for the owner (test-only write mode): with
    /// `OPENPIPESTRESS_WRITE_EXACT_REPORT_EVIDENCE=<dir>`, each witness shape's
    /// local-private report through the production roots is written outside
    /// the repository: `<shape>.html`, the native render command's canonical
    /// HTML (`render_calculation_report_args`), and `<shape>.pdf`, the PDF
    /// member of the package the native save admits and assembles
    /// (`admit_report_package_in`), with `<shape>.package.html`, that
    /// package's HTML member. Without the variable it writes nothing.
    #[test]
    fn writes_the_exact_report_evidence_when_asked() {
        let Ok(dir) = std::env::var("OPENPIPESTRESS_WRITE_EXACT_REPORT_EVIDENCE") else { return };
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).unwrap();
        for (name, text) in SHAPES {
            let w = witness(text);
            let live = live(&w);
            let rendered = production_render_on(&live, &genuine_render_args(&w, &live.job)).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            assert_eq!(rendered["export_blocked"], json!(false), "{name}: {}", rendered["blocking_reasons"]);
            std::fs::write(dir.join(format!("{name}.html")), rendered["html"].as_str().unwrap()).unwrap();
            let admitted = save_admitted(&live, save_args(&bound(&w.request, &live.job), &w, &w.model, Some(&w.result), Some(&w.intake)), name);
            let member = |role: &str| admitted.package.members.iter().find(|member| member.role == role).unwrap_or_else(|| panic!("{name}: {role}")).bytes.clone();
            std::fs::write(dir.join(format!("{name}.pdf")), member(open_pipe_stress_report_package::ROLE_REPORT_PDF)).unwrap();
            std::fs::write(dir.join(format!("{name}.package.html")), member(open_pipe_stress_report_package::ROLE_REPORT_HTML)).unwrap();
        }
    }

    /// `value` with every privacy classification relabelled `public_metadata`
    /// and every redistribution status `invented_non_engineering_example`, at
    /// every depth (Codex review round 1 of #1648).
    fn relabelled(value: &mut Value) {
        match value {
            Value::Object(map) => {
                for (key, item) in map.iter_mut() {
                    match key.as_str() {
                        "privacy_classification" if item.is_string() => *item = json!("public_metadata"),
                        "redistribution_status" if item.is_string() => *item = json!("invented_non_engineering_example"),
                        _ => relabelled(item),
                    }
                }
            }
            Value::Array(items) => items.iter_mut().for_each(relabelled),
            _ => {}
        }
    }

    /// `report` under `context`: `None` removes the export context, `Some`
    /// sets it.
    fn with_context(report: &mut Value, context: Option<&str>) {
        let report_context = report["report_context"].as_object_mut().expect("a report context");
        match context {
            Some(context) => {
                report_context.insert("export_context".into(), json!(context));
            }
            None => {
                report_context.remove("export_context");
            }
        }
    }

    /// Codex review round 1 of #1648 (blocking): an exact-route render or save
    /// is local private only, natively, whatever the input's provenance flags
    /// say. With a live job and its genuine texts, pressure-2 and
    /// pressure-source-1 inputs whose provenance is relabelled public refuse
    /// under `public_report`, another context, without an export context and
    /// without a report context: the render before its binding (no body, no
    /// print view; the lease does not grow and custody is unchanged), from the
    /// decoded args and from the raw body; the save, the shared producer
    /// (no package bytes) by their context checks.
    #[test]
    fn an_exact_render_or_save_outside_local_private_refuses_whatever_its_provenance_flags() {
        for (name, text) in [("pressure_2_straight", STRAIGHT), ("pressure_source_1", SOURCE_1)] {
            let w = witness(text);
            let live = live(&w);
            let genuine = genuine_render_args(&w, &live.job);
            assert_eq!(genuine["input"]["report_context"]["export_context"], json!("local_private"), "{name}");
            let mut contexts: Vec<(String, Value)> = Vec::new();
            for context in [Some("public_report"), Some("downstream_tool"), None] {
                let mut args = genuine.clone();
                relabelled(&mut args["input"]);
                with_context(&mut args["input"], context);
                contexts.push((format!("{context:?}"), args));
            }
            let mut no_context = genuine.clone();
            relabelled(&mut no_context["input"]);
            no_context["input"].as_object_mut().unwrap().remove("report_context");
            contexts.push(("no report_context".to_string(), no_context));
            for (label, args) in &contexts {
                let before = live.registry.custody_binding(&live.job);
                let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, args).unwrap();
                let admitted_bytes = lease.bytes();
                let outcome = crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease);
                let refused = refusal_text(outcome);
                assert!(refused.starts_with("REPORT-RENDER-EXACT-CONTEXT-NOT-LOCAL-PRIVATE: "), "{name} {label}: {refused}");
                assert!(!refused.contains("public_report") && !refused.contains("downstream_tool"), "{name} {label}: {refused}");
                assert_eq!(lease.bytes(), admitted_bytes, "{name} {label}: the lease does not grow");
                assert_eq!(live.registry.custody_binding(&live.job), before, "{name} {label}");
                drop(lease);
                let body = InvokeBody::Raw(serde_json::to_vec(args).unwrap());
                let (mut decoded, mut lease) =
                    admit_body(&live.registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "render_calculation_report", ArgsDecoding::Moved).unwrap();
                let response = respond(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease));
                assert!(response.is_err(), "{name} {label}: no response body");
                drop(lease);
            }
            // The save and the shared producer: relabelled provenance under
            // another context, or none, refuses with no package.
            for context in [Some("public_report"), None] {
                let mut request = bound(&w.request, &live.job);
                relabelled(&mut request["report"]);
                with_context(&mut request["report"], context);
                let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)));
                assert_eq!(code, open_pipe_stress_report_package::bindings::REPORT_CONTEXT_NOT_LOCAL_PRIVATE, "{name} {context:?}: {message}");
                if context.is_some() {
                    let solver = &request["audit_manifest"]["solver_version"];
                    let text = |key: &str| solver[key].as_str().unwrap().to_string();
                    let (solver_name, solver_version, solver_build_ref) = (text("solver_name"), text("solver_version"), text("solver_build_ref"));
                    let identity = open_pipe_stress_report_package::wire::LinkedSolverIdentity {
                        solver_name: &solver_name,
                        solver_version: &solver_version,
                        solver_build_ref: &solver_build_ref,
                    };
                    let typed: open_pipe_stress_report_package::wire::ReportPackageRequest = serde_json::from_value(request.clone()).unwrap();
                    let error = open_pipe_stress_report_package::wire::assemble_wire_request(typed, &identity).map(drop).expect_err("refused");
                    assert!(error.starts_with(open_pipe_stress_report_package::bindings::REPORT_CONTEXT_NOT_LOCAL_PRIVATE), "{name}: {error}");
                }
            }
            // Control: the genuine local-private input still renders.
            let rendered = production_render_on(&live, &genuine).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            assert_eq!(rendered["export_blocked"], json!(false), "{name}");
            assert!(rendered["html"].as_str().unwrap().contains("LOCAL PRIVATE REPORT"), "{name}");
        }
    }

    /// The renderer input's members in declaration order (its positional form,
    /// as the renderer's decode reads a JSON array).
    const RENDER_MEMBERS: [&str; 8] =
        ["report_title", "calculation_report", "report_sections", "result_rows", "report_context", "user_rule_checks", "exact_route", "library_references"];

    fn positional(input: &Value) -> Value {
        Value::Array(RENDER_MEMBERS.iter().map(|member| input.get(*member).cloned().unwrap_or(Value::Null)).collect())
    }

    /// The object's member values as a JSON array (map order; a struct in
    /// this form generally does not decode, so other checks refuse it).
    fn as_array(value: &Value) -> Value {
        Value::Array(value.as_object().expect("an object").values().cloned().collect())
    }

    /// The object as its struct's positional form: `names` in declaration
    /// order, an absent member as null.
    fn ordered(value: &Value, names: &[&str]) -> Value {
        Value::Array(names.iter().map(|name| value.get(*name).cloned().unwrap_or(Value::Null)).collect())
    }

    const CONTEXT_MEMBERS: [&str; 15] = [
        "export_context", "generated_at", "software_name", "software_version", "build_release_identifier", "solver_name", "solver_version",
        "solver_build_ref", "analysis_run_name", "analysis_run_recorded_at", "solver_mode", "solver_settings_ref", "solver_settings",
        "model_coordinate_system", "result_coordinate_systems",
    ];
    const ROUTE_MEMBERS: [&str; 4] = ["semantic_contract_id", "admission", "load_reference_states", "headlines"];
    const ADMISSION_MEMBERS: [&str; 10] = [
        "semantic_contract_id", "run_ref", "result_envelope_id", "received_result_sha256", "model_sha256", "input_manifest_ref",
        "input_manifest_sha256", "derivative_sha256", "native_job_id", "received_bytes_sha256",
    ];
    const LIMITATION_MEMBERS: [&str; 6] = ["limitation_id", "source", "affected_scope", "statement", "effect", "provenance"];
    const ROW_MEMBERS: [&str; 5] = ["row_id", "label", "case_ref", "quantity_display", "source_ref"];
    const PROVENANCE_MEMBERS: [&str; 8] = [
        "source_name", "source_location", "source_license", "contributor", "contributor_certification", "redistribution_status",
        "review_status", "privacy_classification",
    ];

    /// The provenance-record paths of a witness report (Codex review round 3
    /// of #1648): the calculation report's, its model summary's, a load
    /// case's, a result-export reference's, an audit reference's, a report
    /// section reference's, and the report sections' notes, diagnostics,
    /// user-supplied values and limitations; those present in `report`.
    fn provenance_paths(report: &Value) -> Vec<&'static str> {
        [
            "/calculation_report/provenance",
            "/calculation_report/model_input_summary/provenance",
            "/calculation_report/load_case_summary/0/provenance",
            "/calculation_report/result_export_refs/0/provenance",
            "/calculation_report/audit_manifest_refs/0/provenance",
            "/calculation_report/report_section_refs/0/provenance",
            "/report_sections/provenance_notes/0",
            "/report_sections/diagnostics/0/provenance",
            "/report_sections/user_supplied_values/0/provenance",
            "/report_sections/limitations/1/provenance",
        ]
        .into_iter()
        .filter(|pointer| report.pointer(pointer).is_some_and(Value::is_object))
        .collect()
    }

    /// Codex review round 2 of #1648 (blocking): a positional render input
    /// carried an exact section past step 0, the context check and the source
    /// binding, which read members by key, while the renderer's decode reads
    /// a JSON array by field order. For pressure-2 and pressure-source-1, with
    /// relabelled public provenance and without the source texts, the
    /// positional input (under public_report, without a context and under
    /// local_private) and an object input with a positional report_context,
    /// exact_route, admission, limitation entry or result row each refuse
    /// with `REPORT-RENDER-INPUT-POSITIONAL` before any check, from the
    /// decoded args and the raw body: no body or print view, the lease not
    /// grown, custody unchanged. The genuine object-form input still renders.
    #[test]
    fn a_positional_render_input_refuses_before_any_check() {
        for (name, text) in [("pressure_2_straight", STRAIGHT), ("pressure_source_1", SOURCE_1)] {
            let w = witness(text);
            let live = live(&w);
            let genuine = genuine_render_args(&w, &live.job);
            let mut base = genuine["input"].clone();
            relabelled(&mut base);
            let mut cases: Vec<(&str, Value)> = Vec::new();
            for (label, context) in [("positional, public_report", Some("public_report")), ("positional, no report context", None), ("positional, local_private", Some("local_private"))] {
                let mut input = base.clone();
                match context {
                    Some(context) => with_context(&mut input, Some(context)),
                    None => {
                        input.as_object_mut().unwrap().remove("report_context");
                    }
                }
                let positional = positional(&input);
                // The renderer's own decode accepts the positional form, so
                // the native check is what refuses it.
                assert!(open_pipe_stress_report_renderer::decode_renderable_report_input(positional.clone()).is_ok(), "{name} {label}");
                cases.push((label, positional));
            }
            let nested: [(&str, fn(&mut Value)); 5] = [
                ("positional report_context", |input| input["report_context"] = ordered(&input["report_context"], &CONTEXT_MEMBERS)),
                ("positional exact_route", |input| input["exact_route"] = ordered(&input["exact_route"], &ROUTE_MEMBERS)),
                ("positional admission", |input| input["exact_route"]["admission"] = ordered(&input["exact_route"]["admission"], &ADMISSION_MEMBERS)),
                ("positional limitation entry", |input| input["report_sections"]["limitations"][1] = ordered(&input["report_sections"]["limitations"][1], &LIMITATION_MEMBERS)),
                ("positional result row", |input| input["result_rows"][0] = ordered(&input["result_rows"][0], &ROW_MEMBERS)),
            ];
            for (label, edit) in nested {
                let mut input = base.clone();
                edit(&mut input);
                assert!(open_pipe_stress_report_renderer::decode_renderable_report_input(input.clone()).is_ok(), "{name} {label}: decodes");
                cases.push((label, input));
            }
            for (label, input) in cases {
                // Without the source texts, as Codex submitted it.
                let args = json!({"input": input});
                let before = live.registry.custody_binding(&live.job);
                let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
                let admitted_bytes = lease.bytes();
                let refused = refusal_text(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease));
                assert!(refused.starts_with("REPORT-RENDER-INPUT-POSITIONAL: "), "{name} {label}: {refused}");
                assert_eq!(lease.bytes(), admitted_bytes, "{name} {label}");
                assert_eq!(live.registry.custody_binding(&live.job), before, "{name} {label}");
                drop(lease);
                let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
                let (mut decoded, mut lease) =
                    admit_body(&live.registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "render_calculation_report", ArgsDecoding::Moved).unwrap();
                let refused = refusal_text(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease));
                assert!(refused.starts_with("REPORT-RENDER-INPUT-POSITIONAL: "), "{name} {label} (raw body): {refused}");
                drop(lease);
            }
            let rendered = production_render_on(&live, &genuine).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            assert_eq!(rendered["export_blocked"], json!(false), "{name}");
        }
    }

    /// Codex review rounds 2 and 3 of #1648: the native save refuses a
    /// positional request, or a positional struct in a typed member or the
    /// report, before its exact detection, binding, screen and context
    /// checks, from the decoded args and the raw body, with no package; a
    /// positional untyped member is refused by the keyed checks; for
    /// pressure-2 and pressure-source-1, with genuine custody and texts and
    /// relabelled public provenance.
    #[test]
    fn a_positional_save_request_refuses_before_any_check() {
        use crate::report_package_save::REQUEST_POSITIONAL;
        for (name, text) in [("pressure_2_straight", STRAIGHT), ("pressure_source_1", SOURCE_1)] {
            let w = witness(text);
            let live = live(&w);
            let mut request = bound(&w.request, &live.job);
            relabelled(&mut request["report"]);
            let members = open_pipe_stress_report_package::wire::REQUEST_MEMBERS;
            let top = Value::Array(members.iter().map(|member| request.get(*member).cloned().unwrap_or(Value::Null)).collect());
            assert!(open_pipe_stress_report_package::wire::decode_report_package_request(top.clone()).is_ok(), "{name}: the shared decode accepts it");
            let edits: [(&str, fn(&mut Value)); 4] = [
                ("positional report", |r| r["report"] = ordered(&r["report"], &RENDER_MEMBERS)),
                ("positional report_context", |r| r["report"]["report_context"] = ordered(&r["report"]["report_context"], &CONTEXT_MEMBERS)),
                ("positional exact_route", |r| r["report"]["exact_route"] = ordered(&r["report"]["exact_route"], &ROUTE_MEMBERS)),
                ("positional source_model_ref", |r| r["source_model_ref"] = ordered(&r["source_model_ref"], &["ref_type", "ref_id"])),
            ];
            let mut cases = vec![("positional request", top)];
            for (label, edit) in edits {
                let mut variant = request.clone();
                edit(&mut variant);
                cases.push((label, variant));
            }
            // The untyped members have no struct decoder; a positional result
            // document, envelope or producer is refused by the keyed checks.
            for (label, pointer) in [
                ("positional result document", "/result_documents/0"),
                ("positional result envelope", "/result_documents/0/result_envelope"),
                ("positional producer", "/result_documents/0/result_envelope/producer"),
            ] {
                let mut variant = request.clone();
                let member = variant.pointer_mut(pointer).unwrap();
                *member = as_array(member);
                let (code, message) = save_code(&live, save_args(&variant, &w, &w.model, Some(&w.result), Some(&w.intake)));
                assert_ne!(code, REQUEST_POSITIONAL, "{name} {label}: {message}");
            }
            for (label, variant) in cases {
                let (code, message) = save_code(&live, save_args(&variant, &w, &w.model, Some(&w.result), Some(&w.intake)));
                assert_eq!(code, REQUEST_POSITIONAL, "{name} {label}: {message}");
                // The raw body, as the command admits it.
                let args = save_args(&variant, &w, &w.model, Some(&w.result), Some(&w.intake));
                let mut raw = crate::report_package_save::test_support::args_body(&args);
                raw["exactResultText"] = json!(w.result);
                raw["exactIntakeText"] = json!(w.intake);
                let body = InvokeBody::Raw(serde_json::to_vec(&raw).unwrap());
                let (mut decoded, _lease) =
                    admit_body(&live.registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "save_report_package", ArgsDecoding::Moved).unwrap();
                let parsed = crate::report_package_save::save_report_package_args(&mut decoded).unwrap();
                let custody = |job: &str| live.registry.custody_binding(job);
                let error = admit_report_package_in(parsed, &software(), &mut no_store, &custody).map(drop).expect_err("refused");
                assert_eq!(error.code, REQUEST_POSITIONAL, "{name} {label} (raw body)");
            }
        }
    }

    /// The render's production refusal of `args`, from the decoded args and
    /// from the raw body; both must agree.
    fn render_refusal_both(live: &Live, args: &Value) -> String {
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, args).unwrap();
        let first = refusal_text(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease));
        drop(lease);
        let body = InvokeBody::Raw(serde_json::to_vec(args).unwrap());
        let (mut decoded, mut lease) =
            admit_body(&live.registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "render_calculation_report", ArgsDecoding::Moved).unwrap();
        let second = refusal_text(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease));
        assert_eq!(first, second);
        first
    }

    /// The save's refusal code of `request`, from the decoded args and from
    /// the raw body; both must agree.
    fn save_refusal_both(live: &Live, w: &Witness, request: &Value) -> (String, String) {
        let (code, message) = save_code(live, save_args(request, w, &w.model, Some(&w.result), Some(&w.intake)));
        let args = save_args(request, w, &w.model, Some(&w.result), Some(&w.intake));
        let mut raw = crate::report_package_save::test_support::args_body(&args);
        raw["exactResultText"] = json!(w.result);
        raw["exactIntakeText"] = json!(w.intake);
        let body = InvokeBody::Raw(serde_json::to_vec(&raw).unwrap());
        let (mut decoded, _lease) =
            admit_body(&live.registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "save_report_package", ArgsDecoding::Moved).unwrap();
        let parsed = crate::report_package_save::save_report_package_args(&mut decoded).unwrap();
        let custody = |job: &str| live.registry.custody_binding(job);
        let error = admit_report_package_in(parsed, &software(), &mut no_store, &custody).map(drop).expect_err("refused");
        assert_eq!(error.code, code, "raw body: {}", error.message);
        (code, message)
    }

    /// Codex review round 3 of #1648 (blocking): a provenance record in
    /// positional form hid its disposition from the save's keyed screen while
    /// the renderer's decode read it, so a quarantined, rejected or
    /// protected-suspected record reached an unblocked package. For pressure-2
    /// and pressure-source-1, with genuine custody and texts, each such record
    /// at every provenance depth of the witness report (the calculation
    /// report's, its model summary's, a load case's, a result-export
    /// reference's, an audit and a section reference's, and the report
    /// sections' notes, diagnostics, user-supplied values and limitations),
    /// written positionally, refuses the render and the save before any keyed
    /// check (`REPORT-RENDER-INPUT-POSITIONAL`,
    /// `REPORT-PACKAGE-SAVE-REQUEST-POSITIONAL`), from the decoded args and the
    /// raw body. In object form the save refuses the same record by its screen.
    #[test]
    fn a_positional_provenance_record_refuses_the_render_and_the_save_at_every_depth() {
        use crate::report_package_save::REQUEST_POSITIONAL;
        for (name, text) in [("pressure_2_straight", STRAIGHT), ("pressure_source_1", SOURCE_1)] {
            let w = witness(text);
            let live = live(&w);
            let genuine_args = genuine_render_args(&w, &live.job);
            let genuine_request = bound(&w.request, &live.job);
            let paths = provenance_paths(&genuine_request["report"]);
            assert!(paths.len() >= 6, "{name}: {paths:?}");
            for (member, disposition) in [("review_status", "quarantined"), ("review_status", "rejected"), ("redistribution_status", "protected_suspected")] {
                for pointer in &paths {
                    let label = format!("{name} {pointer} {member}={disposition}");
                    let positional = |report: &mut Value| {
                        let record = report.pointer_mut(pointer).unwrap();
                        record[member] = json!(disposition);
                        *record = ordered(record, &PROVENANCE_MEMBERS);
                    };
                    // The render.
                    let mut args = genuine_args.clone();
                    positional(&mut args["input"]);
                    assert!(open_pipe_stress_report_renderer::decode_renderable_report_input(args["input"].clone()).is_ok(), "{label}: decodes");
                    let refused = render_refusal_both(&live, &args);
                    assert!(refused.starts_with("REPORT-RENDER-INPUT-POSITIONAL: "), "{label}: {refused}");
                    assert!(!refused.contains(disposition), "{label}: {refused}");
                    // The save.
                    let mut request = genuine_request.clone();
                    positional(&mut request["report"]);
                    let (code, message) = save_refusal_both(&live, &w, &request);
                    assert_eq!(code, REQUEST_POSITIONAL, "{label}: {message}");
                    // Object form: the save's screen refuses the record.
                    let mut object = genuine_request.clone();
                    object["report"].pointer_mut(pointer).unwrap()[member] = json!(disposition);
                    let (code, message) = save_code(&live, save_args(&object, &w, &w.model, Some(&w.result), Some(&w.intake)));
                    assert_ne!(code, REQUEST_POSITIONAL, "{label}: {message}");
                }
            }
        }
    }

    /// The legitimate-arrays control (Codex review round 3 of #1648): every
    /// witness shape's genuine render input and save request, all of whose
    /// arrays are collections, passes the object-form comparison (the
    /// pressure-free control is `object_form_tests.rs`).
    #[test]
    fn genuine_inputs_with_every_array_field_pass_the_object_form_check() {
        use open_pipe_stress_report_renderer::object_form::report_positional_struct_path;
        for (name, text) in SHAPES {
            let w = witness(text);
            assert_eq!(report_positional_struct_path(&w.request["report"]), None, "{name}");
            assert_eq!(open_pipe_stress_report_package::wire::request_positional_struct_path(&w.request), None, "{name}");
            assert_eq!(crate::report_render_exact::check_object_form(&w.request["report"]), Ok(()), "{name}");
        }
    }

    /// Codex r1 finding 2: with genuine custody and texts, a fabricated row
    /// (a numerical-health id with a stress statement and invented refs)
    /// refuses at the render's re-derivation, and the save refuses it at the
    /// package's item 6 (`report_package` `a_fabricated_report_row_refuses_at_item_6`).
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
        assert_eq!(refusal_text(production_render_on(&live, &args)), format!("{SECTIONS_MISMATCH}: result_rows"));
        let mut request = bound(&w.request, &live.job);
        request["report"]["result_rows"].as_array_mut().unwrap().push(fabricated);
        let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, ROWS_MISMATCH, "{message}");
    }

    #[test]
    fn the_genuine_render_input_renders_through_the_production_root() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let rendered = production_render_on(&live, &genuine_render_args(&w, &live.job)).expect("the genuine input renders");
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
        let refused = refusal_text(production_render_on(&live, &render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: result:")), "{refused}");
        // The variant keeping the genuine exact_route binding: the save's source
        // equality, and the render's re-derivation.
        let genuine = bound(&w.request, &live.job);
        let mut variant = request.clone();
        variant["report"]["exact_route"] = genuine["report"]["exact_route"].clone();
        let (code, message) = save_code(&live, save_args(&variant, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED, "{message}");
        assert!(message.contains("/request/source_results/0"), "{message}");
        // The substitute's row differs from the genuine row by a relative 1e-6, so at
        // the report's six significant digits (§3.7) both display the same rows; the
        // render refuses it by the rows' full-precision checksums (Codex review
        // round 1 of #1655), not by the displayed rows.
        assert_eq!(variant["report"]["result_rows"], genuine["report"]["result_rows"], "the displayed rows coincide");
        let manifest = canonical_json(&w.manifest);
        let refused = refusal_text(production_render_on(&live, &render_args(&variant["report"], Some(&w.result), Some(&w.intake), Some(&manifest))));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: result_export_refs"));
        // The same variant with the genuine received-result checksum as well (Codex's
        // reproduction): the renderer's admission comparison then passes, and the
        // rows' checksums still refuse it; the save refuses it at the source.
        variant["report"]["calculation_report"]["audit_manifest_refs"][1]["checksum"] = genuine["report"]["calculation_report"]["audit_manifest_refs"][1]["checksum"].clone();
        let refused = refusal_text(production_render_on(&live, &render_args(&variant["report"], Some(&w.result), Some(&w.intake), Some(&manifest))));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: result_export_refs"));
        let (code, message) = save_code(&live, save_args(&variant, &w, &w.model, Some(&w.result), Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED, "{message}");
        // 2. With its own text: step 2.
        let own = serde_json::to_string(&substitute.request["source_results"][0]).unwrap();
        let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&own), Some(&w.intake)));
        assert_eq!(code, SOURCE_NOT_DELIVERED);
        assert!(message.contains(": result:"), "{message}");
        let refused = refusal_text(production_render_on(&live, &render_args(&request["report"], Some(&own), Some(&w.intake), Some(&canonical_json(&w.manifest)))));
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
        let refused = refusal_text(production_render_on(&live, &render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: model:")), "{refused}");
        // With its own text as the intake text: step 2.
        let own = serde_json::to_string(&model).unwrap();
        let (code, message) = save_code(&live, save_args(&request, &w, &model, Some(&w.result), Some(&own)));
        assert_eq!(code, SOURCE_NOT_DELIVERED);
        assert!(message.contains(": model:"), "{message}");
        let refused = refusal_text(production_render_on(&live, &render_args(&request["report"], Some(&w.result), Some(&own), Some(&canonical_json(&w.manifest)))));
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
            let refused = refusal_text(production_render_on(&live, &genuine_render_args(&w, &live.job)));
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
        let refused = refusal_text(production_render_on(&live, &render_args(&request["report"], Some(&lr_raw), Some(&intake), Some(&canonical_json(&w.manifest)))));
        assert_eq!(refused, format!("{STANDING_NOT_ELIGIBLE}: the native reading of the delivered result is not numerically eligible."));
        // The genuine pressure-source-1 witness without load/reference states is eligible:
        // its request passes the binding and is admitted.
        let genuine = live_on(ScriptedHost::normal(), &w.intake, &w.mode, &w.result);
        save_admitted(&genuine, save_args(&bound(&w.request, &genuine.job), &w, &w.model, Some(&w.result), Some(&w.intake)), "pressure_source_1");
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
            let refused = refusal_text(production_render_on(&live, &render_args(&input, Some(&w.result), Some(&w.intake), Some(&manifest))));
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
        let refused = refusal_text(production_render_on(&live, &render_args(&input, Some(&w.result), Some(&w.intake), Some(&manifest))));
        assert!(refused.starts_with(&format!("{SOURCE_NOT_DELIVERED}: model:")), "{refused}");
        // Another input manifest: its digest is not the binding's.
        let mut other = w.manifest.clone();
        other["invented_extra"] = json!(true);
        let refused = refusal_text(production_render_on(&live, &render_args(&genuine, Some(&w.result), Some(&w.intake), Some(&canonical_json(&other)))));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: admission.input_manifest_sha256"));
        // The texts without exact_route (a pressure-free input).
        let refused = refusal_text(production_render_on(&live, &render_args(&pressure_free_input(), Some(&w.result), Some(&w.intake), Some(&manifest))));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: exact_route"));
        let refused = refusal_text(production_render_on(&live, &render_args(&pressure_free_input(), Some(&w.result), None, None)));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: exact_route"));
    }

    /// The rows' full-precision binding on both identities (§3.7; Codex review
    /// round 1 of #1655): the displayed rows are rounded, so the render binds
    /// each row's reference and checksum to the delivered source. A checksum,
    /// a reference or the entry count that is not the delivered source's
    /// refuses natively, with the displayed rows unchanged.
    #[test]
    fn the_render_binds_the_rows_full_precision_checksums_on_both_identities() {
        for (name, text) in [("pressure_2_straight", STRAIGHT), ("pressure_source_1", SOURCE_1)] {
            let w = witness(text);
            let live = live(&w);
            let manifest = canonical_json(&w.manifest);
            let genuine = bound(&w.request, &live.job)["report"].clone();
            let edits: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
                ("a row checksum", Box::new(|input: &mut Value| input["calculation_report"]["result_export_refs"][1]["checksum"]["value"] = json!("0".repeat(64)))),
                ("two row checksums exchanged", Box::new(|input: &mut Value| {
                    let refs = input["calculation_report"]["result_export_refs"].as_array_mut().unwrap();
                    let (first, second) = (refs[0]["checksum"]["value"].clone(), refs[1]["checksum"]["value"].clone());
                    refs[0]["checksum"]["value"] = second;
                    refs[1]["checksum"]["value"] = first;
                })),
                ("a row reference", Box::new(|input: &mut Value| input["calculation_report"]["result_export_refs"][0]["ref"]["ref_id"] = json!("result:invented"))),
                ("a row reference removed", Box::new(|input: &mut Value| { input["calculation_report"]["result_export_refs"].as_array_mut().unwrap().pop(); })),
            ];
            for (label, edit) in edits {
                let mut input = genuine.clone();
                edit(&mut input);
                assert_eq!(input["result_rows"], genuine["result_rows"], "{name}: {label}");
                let refused = refusal_text(production_render_on(&live, &render_args(&input, Some(&w.result), Some(&w.intake), Some(&manifest))));
                assert_eq!(refused, format!("{SECTIONS_MISMATCH}: result_export_refs"), "{name}: {label}");
            }
        }
    }

    /// The display policy and `conversion_performed` on both identities (§3.7;
    /// Codex review round 1 of #1655): the render requires the report's unit
    /// display summary to be the delivered source's, by the package's own
    /// check, so it neither shows MPa values under a false
    /// `conversion_performed` nor drops the display rule statement under
    /// another policy. The save refuses both.
    #[test]
    fn the_render_binds_the_display_policy_and_conversion_on_both_identities() {
        const OLD_POLICY: &str = "display result-row values with their explicit units; no report-time conversion";
        for (name, text) in [("pressure_2_straight", STRAIGHT), ("pressure_source_1", SOURCE_1)] {
            let w = witness(text);
            let live = live(&w);
            let manifest = canonical_json(&w.manifest);
            let genuine = bound(&w.request, &live.job);
            let display = &genuine["report"]["calculation_report"]["model_input_summary"]["unit_display_summary"];
            // Both witnesses display their Pa headlines in MPa.
            assert_eq!(display["conversion_performed"], json!(true), "{name}");
            assert_eq!(display["quantity_display_policy"], json!(open_pipe_stress_report_renderer::QUANTITY_DISPLAY_POLICY), "{name}");
            let edits: Vec<(&str, Box<dyn Fn(&mut Value)>, &str)> = vec![
                ("conversion_performed false", Box::new(|summary: &mut Value| summary["conversion_performed"] = json!(false)), "unit display conversion_performed"),
                (
                    "the earlier policy, conversion_performed false",
                    Box::new(|summary: &mut Value| {
                        summary["quantity_display_policy"] = json!(OLD_POLICY);
                        summary["conversion_performed"] = json!(false);
                    }),
                    "unit display quantity_display_policy",
                ),
                ("the earlier policy", Box::new(|summary: &mut Value| summary["quantity_display_policy"] = json!(OLD_POLICY)), "unit display quantity_display_policy"),
                ("another result unit", Box::new(|summary: &mut Value| summary["result_units"] = json!(["MPa"])), "unit display result_units"),
            ];
            for (label, edit, save_detail) in edits {
                let mut request = genuine.clone();
                edit(&mut request["report"]["calculation_report"]["model_input_summary"]["unit_display_summary"]);
                let refused = refusal_text(production_render_on(&live, &render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&manifest))));
                assert_eq!(refused, format!("{SECTIONS_MISMATCH}: unit_display_summary"), "{name}: {label}");
                let (code, message) = save_code(&live, save_args(&request, &w, &w.model, Some(&w.result), Some(&w.intake)));
                assert_eq!((code.as_str(), message.as_str()), ("REPORT-PACKAGE-BINDING-MISMATCH", format!("REPORT-PACKAGE-BINDING-MISMATCH: {save_detail}").as_str()), "{name}: {label}");
            }
            // The genuine summary renders with the statement and MPa values.
            let rendered = production_render_on(&live, &genuine_render_args(&w, &live.job)).expect("the genuine input renders");
            assert_eq!(rendered["export_blocked"], json!(false), "{name}: {}", rendered["blocking_reasons"]);
            let html = rendered["html"].as_str().unwrap();
            assert!(html.contains(open_pipe_stress_report_renderer::DISPLAY_RULE_STATEMENT) && html.contains(" MPa</"), "{name}");
        }
    }

    /// The Diagnostics bound to the delivered result on both identities
    /// (DEL-08-01 Design `exact-route-report-sections.md` §3.6, §6.3 item 4):
    /// the standing line names every integrity warning in the report's
    /// Diagnostics, so with genuine custody and texts, an edit of the
    /// witness's integrity entry (pressure-source-1's
    /// `NUMERICAL_INTEGRITY_SENSITIVE`, pressure-2's
    /// `NUMERICAL_INTEGRITY_CHECKS_PASSED`) refuses the render (no body or
    /// print view), from the decoded args and the raw body, and the save, by
    /// the package's own check (`bindings::check_report_diagnostics`): its
    /// deletion, a renamed code, its replacement by the other integrity
    /// code and severity, another severity, an edited message or affected
    /// object, a reordering, and a forged integrity entry appended after the
    /// result's. The genuine witnesses render and save.
    #[test]
    fn the_render_and_the_save_bind_the_diagnostics_on_both_identities() {
        const PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
        const SENSITIVE: &str = "NUMERICAL_INTEGRITY_SENSITIVE";
        for (name, text, integrity_code) in [("pressure_2_straight", STRAIGHT, PASSED), ("pressure_source_1", SOURCE_1, SENSITIVE)] {
            let w = witness(text);
            let live = live(&w);
            let genuine = bound(&w.request, &live.job);
            let diagnostics = genuine["report"]["report_sections"]["diagnostics"].as_array().unwrap().clone();
            let at = diagnostics.iter().position(|entry| entry["code"] == integrity_code).unwrap_or_else(|| panic!("{name}: {integrity_code}"));
            let source_count = serde_json::from_str::<Value>(&w.result).unwrap()["diagnostics"].as_array().unwrap().len();
            assert!(at + 1 < source_count, "{name}: a result entry follows the integrity entry");
            let (other_code, other_severity) = if integrity_code == SENSITIVE { (PASSED, "info") } else { (SENSITIVE, "warning") };
            let edits: Vec<(&str, Box<dyn Fn(&mut Vec<Value>)>, String)> = vec![
                ("deleted", Box::new(move |d| { d.remove(at); }), "report diagnostics cover the source diagnostics".to_string()),
                ("renamed", Box::new(move |d| d[at]["code"] = json!("NUMERICAL_CHECK_RENAMED")), format!("report diagnostic {at} code")),
                (
                    "replaced by the other integrity code and severity",
                    Box::new(move |d| {
                        d[at]["code"] = json!(other_code);
                        d[at]["severity"] = json!(other_severity);
                    }),
                    format!("report diagnostic {at} code"),
                ),
                ("another severity", Box::new(move |d| d[at]["severity"] = json!(other_severity)), format!("report diagnostic {at} severity")),
                (
                    "an edited message",
                    Box::new(move |d| {
                        let message = format!("{} (edited)", d[at]["message"].as_str().unwrap());
                        d[at]["message"] = json!(message);
                    }),
                    format!("report diagnostic {at} message"),
                ),
                ("an edited affected object", Box::new(move |d| d[at]["affected_object"]["ref_id"] = json!("case:invented")), format!("report diagnostic {at} affected_object")),
                ("reordered", Box::new(move |d| d.swap(at, at + 1)), format!("report diagnostic {at} code")),
                (
                    "a forged integrity entry appended",
                    Box::new(move |d| {
                        let mut forged = d[at].clone();
                        forged["code"] = json!(SENSITIVE);
                        forged["severity"] = json!("warning");
                        d.push(forged);
                    }),
                    format!("report diagnostic {} is an integrity code the source does not carry", diagnostics.len()),
                ),
            ];
            for (label, edit, save_detail) in edits {
                let mut request = genuine.clone();
                edit(request["report"]["report_sections"]["diagnostics"].as_array_mut().unwrap());
                let args = render_args(&request["report"], Some(&w.result), Some(&w.intake), Some(&canonical_json(&w.manifest)));
                assert_eq!(render_refusal_both(&live, &args), format!("{SECTIONS_MISMATCH}: diagnostics"), "{name}: {label}");
                let (code, message) = save_refusal_both(&live, &w, &request);
                assert_eq!((code.as_str(), message), ("REPORT-PACKAGE-BINDING-MISMATCH", format!("REPORT-PACKAGE-BINDING-MISMATCH: {save_detail}")), "{name}: {label}");
            }
            // The genuine witness renders, with its standing line, and saves.
            let rendered = production_render_on(&live, &genuine_render_args(&w, &live.job)).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            let standing = if integrity_code == SENSITIVE { SENSITIVE_STANDING } else { PLAIN_STANDING };
            assert!(rendered["html"].as_str().unwrap().contains(standing), "{name}");
            assert!(rendered["derived_print_html"].as_str().unwrap().contains(standing), "{name}");
            save_admitted(&live, save_args(&genuine, &w, &w.model, Some(&w.result), Some(&w.intake)), name);
        }
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

    /// The regression check for the revision-4 gate finding, after opening:
    /// the production native render admits a genuine exact input from a live
    /// job, with its genuine texts, only after step 0 and the native source
    /// binding, from the decoded args and from the raw body as the command
    /// does; the lease grows by the priced parse, and custody is unchanged.
    /// An `exact_route` relabelled to another identity refuses: preview-physics-2,
    /// physics-retained-1 and an invented identity at step 0, and
    /// pressure-source-1 at the re-derivation.
    #[test]
    fn the_production_native_render_renders_a_genuine_exact_result() {
        let w = witness(STRAIGHT);
        let live = live(&w);
        let args = genuine_render_args(&w, &live.job);
        let before = live.registry.custody_binding(&live.job);
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
        let admitted_bytes = lease.bytes();
        let rendered = crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease).expect("the genuine input renders");
        assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
        assert!(lease.bytes() > admitted_bytes, "the lease grows by the priced parse");
        assert_eq!(live.registry.custody_binding(&live.job), before, "custody is unchanged");
        drop(lease);
        // From the raw body, as the command does.
        let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
        let (mut decoded, mut lease) =
            admit_body(&live.registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), "render_calculation_report", ArgsDecoding::Moved).unwrap();
        let again = crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease).expect("the genuine input renders");
        assert_eq!(again["html"], rendered["html"]);
        drop(lease);
        for id in [
            open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_ID,
            PHYSICS_RETAINED_ID,
            "openpipestress.result_semantics/0.3.0/invented-1",
        ] {
            let mut args = args.clone();
            args["input"]["exact_route"]["semantic_contract_id"] = json!(id);
            let refused = refusal_text(production_render_on(&live, &args));
            assert!(refused.starts_with(RENDER_ROUTE_NOT_ADMITTED), "{id}: {refused}");
        }
        let mut args = args.clone();
        args["input"]["exact_route"]["semantic_contract_id"] = json!(PRESSURE_SOURCE_ID);
        let refused = refusal_text(production_render_on(&live, &args));
        assert_eq!(refused, format!("{SECTIONS_MISMATCH}: semantic_contract_id"));
        // `RENDER_ADMITTED_EXACT_CONTRACTS` is the two exact identities.
        assert_eq!(crate::report_render_exact::RENDER_ADMITTED_EXACT_CONTRACTS, [PRESSURE_ID, PRESSURE_SOURCE_ID]);
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
        let refused = refusal_text(production_render_on(&live, &args));
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
        let (outcome, measured) = measure(|| crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease));
        match outcome {
            Err(CommandError::Ingress(crate::solve_jobs::SolveJobError::ResourceUnavailable { .. })) => {}
            other => panic!("not a resource refusal: {other:?}"),
        }
        assert_eq!(lease.bytes(), admitted_bytes);
        // Neither text is parsed: the peak is the object-form check's
        // by-reference decode of the input (priced within the ingress lease;
        // DEL-08-01 Design `report-package-current-results.md` §2.6) plus less
        // than a quarter of the result text.
        let args = genuine_render_args(&w, &live.job);
        let (_, check) = measure(|| crate::report_render_exact::check_object_form(&args["input"]));
        assert!(measured.peak_requested < check.peak_requested + (w.result.len() as u64) / 4, "nothing is parsed: {measured:?}, check {check:?}");
    }

    /// The growth is bounded by the per-invocation budget, as the ingress
    /// lease is (Codex review r2 of #1665): with the budget at the admitted
    /// lease, it is refused `ingress_exceeds_budget` and the lease is unchanged.
    #[test]
    fn a_growth_above_the_budget_refuses_with_ingress_exceeds_budget() {
        let w = witness(STRAIGHT);
        let live = live_on(ScriptedHost::normal(), &w.intake, &w.mode, &w.result);
        let (mut decoded, mut lease) = admit_json(&live.registry, "render_calculation_report", ArgsDecoding::Moved, &genuine_render_args(&w, &live.job)).unwrap();
        let admitted_bytes = lease.bytes();
        let growth = exact_render_growth_bytes(&w.result, &w.intake, &canonical_json(&w.manifest)).unwrap();
        let (pool_bytes, _) = live.registry.pool().settings();
        live.registry.configure_pool(pool_bytes, admitted_bytes, false).unwrap();
        match crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease) {
            Err(CommandError::Ingress(crate::solve_jobs::SolveJobError::ResourceUnavailable { refusal, .. })) => assert_eq!(
                refusal,
                crate::memory_pool::ResourceRefusal::IngressExceedsBudget { required_bytes: admitted_bytes + growth, budget_bytes: admitted_bytes }
            ),
            other => panic!("not the budget refusal: {other:?}"),
        }
        assert_eq!(lease.bytes(), admitted_bytes);
    }

    /// The whole-command counting-allocator check of the genuine exact render
    /// (from the raw body through the ingress admission and the production
    /// root) and of the genuine exact save (through the shared producer's
    /// assembly of the admitted package, §5.1 item 8): each priced peak
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
            let response = respond(crate::render_calculation_report_args(&mut decoded, &live.registry, &mut lease)).map(response_text);
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
        let ((blocked, lease_bytes), measured) = measure(|| {
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
            let admitted = admit_report_package_in(request, &software(), &mut no_store, &custody)
                .unwrap_or_else(|error| panic!("the genuine save is admitted: {} {}", error.code, error.message));
            let blocked = admitted.package.export_blocked;
            drop(admitted);
            (blocked, lease.bytes())
        });
        assert!(!blocked, "the assembled package is unblocked");
        assert!(measured.peak_priced <= lease_bytes - envelope, "save: priced peak {} exceeds its lease {}", measured.peak_priced, lease_bytes - envelope);
        eprintln!("exact save: priced peak {} lease {}", measured.peak_priced, lease_bytes - envelope);
    }
}
