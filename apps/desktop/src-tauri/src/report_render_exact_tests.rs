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
    let (mut args, _lease) = admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, args).unwrap();
    respond(crate::render_calculation_report_args(&mut args)).map(response_text)
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
    let (mut args, _lease) =
        admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": input})).unwrap();
    render_calculation_report_args_in(&mut args, EXACT_RENDER_WITNESS_ADMITTED)
}

/// Codex review round 1 (blocking): a coherent invented exact input, which
/// the renderer itself renders unblocked with "integrity checked", is refused
/// by the production native command at step 0, with no HTML.
#[test]
fn production_native_render_refuses_a_coherent_exact_input() {
    let exact = with_exact_route(pressure_free_input());
    // Coherent: below the gate, the exact render path renders it unblocked,
    // with the exact block and the composed standing line.
    let rendered = witness_render(&exact).expect("the witness set admits pressure-2");
    assert_eq!(rendered["export_blocked"], json!(false), "{}", rendered["blocking_reasons"]);
    let html = rendered["html"].as_str().unwrap();
    assert!(html.contains(EXACT_ROUTE_SUBHEADING) && html.contains("Numerical standing: integrity checked"));

    // The production root, through the ingress decode.
    let registry = registry(ScriptedHost::normal());
    let (mut args, _lease) =
        admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": exact})).unwrap();
    assert_eq!(refusal_text(crate::render_calculation_report_args(&mut args)), PRODUCTION_REFUSAL);
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
    let (mut args, _lease) =
        admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": input})).unwrap();
    let root = refusal_text(crate::render_calculation_report_args(&mut args));
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
        let (mut args, _lease) =
            admit_json(&registry, "render_calculation_report", ArgsDecoding::Moved, &json!({"input": input})).unwrap();
        assert_eq!(serde_json::to_string(&crate::render_calculation_report_args(&mut args).unwrap()).unwrap(), before);
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
