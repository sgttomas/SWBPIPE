//! The native render's identity boundary (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §3.6 step 0, §5.5): the production command
//! refuses a coherent invented exact-route input before any further work, a
//! non-exact input renders as before, the test-only witness set still reaches
//! the exact render path, and the render's admitted-set forwarding graph is
//! closed. Every id and digest is invented. Standard claim fence applies
//! (claims taxonomy per DEC-081).

#[path = "../../../../core/reporting/report_package/src/forwarding_graph_scan.rs"]
mod forwarding_graph_scan;

use crate::command_ingress::test_support::{admit_json, registry};
use crate::command_ingress::{respond, ArgsDecoding, CommandError};
use crate::ingress::counting::measure;
use crate::memory_pool::test_host::ScriptedHost;
use crate::report_render_exact::{
    check_render_route_admitted, render_calculation_report_args_in, EXACT_RENDER_WITNESS_ADMITTED, PRODUCTION,
    RENDER_ROUTE_NOT_ADMITTED,
};
use forwarding_graph_scan::{scan, Arg, Graph, Row};
use open_pipe_stress_report_renderer::exact_statements::exact_statement_table;
use open_pipe_stress_report_renderer::EXACT_ROUTE_SUBHEADING;
use open_pipe_stress_result_export::semantic_contract::{PHYSICS_RETAINED_ID, PRESSURE_ID};
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

/// Every exact marker refuses in production: `exact_route` (even without an
/// identity), an exact identity named anywhere, and exact limitation entries
/// without `exact_route`.
#[test]
fn every_exact_marker_refuses_in_production() {
    let exact = with_exact_route(pressure_free_input());
    let refused = |input: &Value| check_render_route_admitted(input, PRODUCTION).unwrap_err();
    let mut no_identity = exact.clone();
    no_identity["exact_route"].as_object_mut().unwrap().remove("semantic_contract_id");
    assert_eq!(
        refused(&no_identity),
        format!("{RENDER_ROUTE_NOT_ADMITTED}: the rendered report admits no exact-route identity; found an exact_route section that names no identity.")
    );
    // Without `exact_route`, the limitation entries still name pressure-2.
    let mut limitations_only = exact.clone();
    limitations_only.as_object_mut().unwrap().remove("exact_route");
    assert_eq!(refused(&limitations_only), PRODUCTION_REFUSAL);
    // Exact limitation ids alone, with no identity anywhere.
    for entry in limitations_only["report_sections"]["limitations"].as_array_mut().unwrap() {
        entry["source"] = json!({"ref_type": "report_renderer", "ref_id": "invented-render"});
        if entry["statement"].as_str().unwrap().contains(PRESSURE_ID) {
            entry["statement"] = json!("Invented statement.");
        }
    }
    assert_eq!(
        refused(&limitations_only),
        format!("{RENDER_ROUTE_NOT_ADMITTED}: the rendered report admits no exact-route identity; found the exact-route limitation entry limitation:invented-run:exact:1 without an exact_route section.")
    );
    // An exact identity named only in some other string, or as a key.
    let mut named = pressure_free_input();
    named["report_title"] = json!(PHYSICS_RETAINED_ID);
    assert!(refused(&named).ends_with(&format!("found {PHYSICS_RETAINED_ID}.")));
    let mut keyed = pressure_free_input();
    keyed["calculation_report"]["template_slots"] = json!({PRESSURE_ID: 1});
    assert_eq!(refused(&keyed), PRODUCTION_REFUSAL);
    // Below the gate, an admitted route still refuses an unadmitted identity
    // it names elsewhere, and exact limitations without `exact_route`.
    let mut retained = exact.clone();
    retained["report_title"] = json!(PHYSICS_RETAINED_ID);
    assert!(check_render_route_admitted(&retained, EXACT_RENDER_WITNESS_ADMITTED)
        .unwrap_err()
        .ends_with(&format!("found {PHYSICS_RETAINED_ID}.")));
    assert!(check_render_route_admitted(&limitations_only, EXACT_RENDER_WITNESS_ADMITTED).is_err());
    // The admitting decision allocates nothing.
    let (admitted, measured) = measure(|| check_render_route_admitted(&exact, EXACT_RENDER_WITNESS_ADMITTED));
    assert!(admitted.is_ok());
    assert_eq!(measured.allocations, 0, "{measured:?}");
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
