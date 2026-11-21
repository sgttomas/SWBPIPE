//! The rule check by handle: the web view sends the custody handle (`jobId`)
//! of the solved result the native side holds, never a copy of the result.
//! These tests pin, through the command's own admission and argument path
//! (`run_rule_checks_args`), that a handle selects only a result this
//! session holds natively from a fresh solve, that every other handle is
//! refused by name before anything is evaluated, and that a check by handle
//! answers exactly as the retired envelope route answered for the same pack,
//! model and result. Packs, models and values are invented; the results are
//! committed fixtures. Nothing here is a professional or code-compliance claim.

use super::*;
use crate::command_ingress::test_support::admit_json;
use crate::native_replay_tests::registry;
use crate::solve_jobs::test_support::wait_terminal;
use crate::solve_jobs::{HeldResult, ResultCustody, RestoredSource};
use crate::solve_worker::test_support::Gate;
use std::path::Path;

fn project_text(relative: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn project_file(relative: &str) -> Value {
    serde_json::from_str(&project_text(relative)).unwrap()
}

fn no_store() -> Result<PathBuf, String> {
    Err("no store in this test".to_string())
}

/// A completed solve job holding `text` under `custody`.
fn solve_holding(registry: &SolveJobRegistry, gate: &Gate, text: &str, custody: ResultCustody) -> String {
    let started = gate.started_count();
    let receipt = registry.start_with_custody(br#"{"project":{"id":"p"}}"#, None, custody).unwrap();
    gate.wait_started(started + 1);
    gate.release(text);
    let status = wait_terminal(registry, &receipt.job_id);
    assert_eq!(status.state, "completed", "{status:?}");
    receipt.job_id
}

/// A restored saved project run holding `text`, as `stored_custody` reopens one.
fn restored_holding(registry: &SolveJobRegistry, text: &str) -> String {
    let lease = registry.acquire_result_parse(text.len() as u64).unwrap();
    let source = RestoredSource { kind: "saved_project_run", project_id: "p".into(), recording_id: None, result_sha256: "test".into(), record_sha256: None };
    registry.restore(registry.reset_epoch(), source, HeldResult::restored(text.to_string(), lease), None).unwrap()
}

/// The command's argument path, through its admission.
fn run_by_handle(registry: &SolveJobRegistry, args: Value) -> Result<Value, String> {
    let (mut args, _lease) = admit_json(registry, "run_rule_checks", ArgsDecoding::Moved, &args).expect("admitted");
    run_rule_checks_args(&mut args, &no_store, registry).map_err(|error| match error {
        CommandError::Command(message) => message,
        other => format!("{other:?}"),
    })
}

fn preview_by_handle(registry: &SolveJobRegistry, args: Value) -> Result<Value, String> {
    let (mut args, _lease) = admit_json(registry, "preview_rule_solver_rows", ArgsDecoding::Moved, &args).expect("admitted");
    preview_rule_solver_rows_args(&mut args, registry).map_err(|error| match error {
        CommandError::Command(message) => message,
        other => format!("{other:?}"),
    })
}

fn restamp(mut pack: Value) -> Value {
    let computed = rule_pack_document::compute_rule_pack_checksum(&pack).expect("checksum recomputes");
    pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
    pack
}

/// The invented demo pack with a selector naming `row`, its first required load
/// case on the row's basis and its second on another load case of the model
/// (dropped when the model has no other), its units the row's.
fn pack_for(model: &Value, envelope: &Value, row: &Value) -> Value {
    let mut pack = project_file("examples/rule_packs/invented_demo.yaml");
    let selector = rule_solver_selection::selector_for_row(envelope, row).expect("the row has a selector");
    pack["required_inputs"][0]["solver_result_ref"] = serde_json::to_value(&selector).unwrap();
    let unit = row["unit"].clone();
    pack["required_inputs"][0]["quantity_intent"]["unit_ref"] = unit.clone();
    pack["required_inputs"][1]["quantity_intent"]["unit_ref"] = unit;
    pack["required_load_cases"][0]["basis_ref"] = row["basis_ref"].clone();
    let other = model["load_cases"].as_array().unwrap().iter().find(|case| case["id"] != row["basis_ref"]["ref_id"]);
    match other {
        Some(case) => pack["required_load_cases"][1]["basis_ref"] = json!({"ref_type": "load_case", "ref_id": case["id"]}),
        None => {
            pack["required_load_cases"].as_array_mut().unwrap().truncate(1);
            pack["load_case_mappings"][0]["required_load_case_refs"].as_array_mut().unwrap().truncate(1);
        }
    }
    restamp(pack)
}

fn supplied_for(row: &Value) -> Value {
    json!([
        {"ref_id": "demo_limit_quantity", "value": row["value"].as_f64().unwrap().abs() * 2.0 + 1.0, "unit": row["unit"], "dimension": "stress"},
        {"ref_id": "demo_limit_slot", "value": 1.0, "unit": "ratio", "dimension": "dimensionless"}
    ])
}

/// The result as the retired envelope route delivered it: the web view sent
/// `JSON.stringify` of its parsed copy, which writes a negative zero as `0` and
/// an integral binary64 below 1e21 without a fraction, and the native side parsed
/// that text. Every other number round-trips exactly in both.
fn as_web_view_sent(value: &Value) -> Value {
    match value {
        Value::Number(number) => match number.as_f64() {
            Some(x) if number.is_f64() && x == 0.0 => json!(0),
            Some(x) if number.is_f64() && x.fract() == 0.0 && x.abs() < 9.0e18 => json!(x as i64),
            _ => value.clone(),
        },
        Value::Array(items) => Value::Array(items.iter().map(as_web_view_sent).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(key, item)| (key.clone(), as_web_view_sent(item))).collect()),
        other => other.clone(),
    }
}

/// The row a fixture's pack binds: a stress-unit row with a value and a basis
/// that the binding gate admits, else any such row.
fn chosen_row(envelope: &Value) -> Value {
    let rows = envelope["results"].as_array().unwrap();
    let usable = |row: &&Value| {
        row["value"].as_f64().is_some()
            && row["unit"].is_string()
            && row["basis_ref"].is_object()
            && rule_solver_selection::selector_for_row(envelope, row).is_some()
            && open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(envelope, row).is_none()
    };
    rows.iter().filter(usable).find(|row| row["unit"] == "Pa").or_else(|| rows.iter().find(usable)).cloned().expect("a bindable row")
}

/// The committed fixtures the parity covers: model, result text and the
/// registered invocation each route sends (none, or the captured one).
fn parity_cases() -> Vec<(String, Value, String, Option<Value>)> {
    let mut cases = Vec::new();
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    for mode in ["sparse", "dense"] {
        let label = format!("pressure_connected_mechanics_{mode}");
        cases.push((label.clone(), request["model"].clone(), project_text(&format!("fixtures/results/{label}.json")), None));
    }
    for name in ["connected", "pressure"] {
        let request = project_file(&format!("fixtures/product_preview/pressure/load_reference/{name}.request.json"));
        for mode in ["sparse_interactive", "dense_scrutiny"] {
            let label = format!("pressure/load_reference/{name}-{mode}");
            cases.push((label.clone(), request["model"].clone(), project_text(&format!("fixtures/product_preview/{label}.raw.json")), None));
        }
    }
    for (group, names) in [
        ("physics_source", &["fields", "mixed", "n05", "n06"][..]),
        ("load_reference_source", &["fields", "n05"][..]),
    ] {
        for name in names {
            let request = project_file(&format!("fixtures/product_preview/pressure_source/{group}/{name}.request.json"));
            for mode in ["sparse_interactive", "dense_scrutiny"] {
                let label = format!("pressure_source/{group}/{name}-{mode}");
                let text = project_text(&format!("fixtures/product_preview/{label}.raw.json"));
                let invocation = json!({"request": request, "solver_mode": mode});
                cases.push((label.clone(), request["model"].clone(), text.clone(), Some(invocation)));
                // Without its registered invocation: refused by both routes alike.
                cases.push((format!("{label} without invocation"), request["model"].clone(), text, None));
            }
        }
    }
    cases
}

/// Parity, kept while the parity is the evidence that the handle route changed
/// nothing: for every committed fixture case, the same pack, model, supplied
/// values and invocation give the same outcome, bound rows, findings and
/// refusals by the retired envelope route (the engine's value path on the
/// result as the web view sent it) and by handle (the command's argument path
/// on the exact held bytes), byte for byte in canonical form.
#[test]
fn a_rule_check_by_handle_answers_as_the_envelope_route_did() {
    let (registry, gate) = registry();
    let mut outcomes = std::collections::BTreeSet::new();
    for (label, model, text, invocation) in parity_cases() {
        let parsed: Value = serde_json::from_str(&text).unwrap();
        let row = chosen_row(&parsed);
        let pack = pack_for(&model, &parsed, &row);
        let supplied = supplied_for(&row);
        let by_envelope = run_rule_checks_value(
            &no_store,
            pack.clone(),
            Some(model.clone()),
            Some(as_web_view_sent(&parsed)),
            None,
            Some(supplied.clone()),
            None,
            invocation.clone(),
        );
        let job = solve_holding(&registry, &gate, &text, ResultCustody::Native);
        let mut args = json!({"rulePackDocument": pack, "jobId": job, "model": model, "suppliedValueBindings": supplied});
        if let Some(invocation) = invocation {
            args["sourceBlockInvocation"] = invocation;
        }
        let by_handle = run_by_handle(&registry, args);
        let canonical = |outcome: &Result<Value, String>| outcome.as_ref().map(|value| serde_json::to_string(value).unwrap()).map_err(Clone::clone);
        assert_eq!(canonical(&by_handle), canonical(&by_envelope), "{label}");
        outcomes.insert(match &by_handle {
            Ok(value) => value["aggregate_status"].as_str().unwrap().to_string(),
            Err(error) => error.split(':').next().unwrap().to_string(),
        });
        registry.dispose_result(&job).unwrap();
    }
    // The cases reach a checked outcome and the standing refusals, not only one of them.
    assert!(outcomes.contains("USER_RULE_CHECKED"), "{outcomes:?}");
    assert!(outcomes.contains("RULE_NUMERICAL_NEEDS_RECOMPUTE") || outcomes.contains("RULE_NUMERICAL_SOURCE_UNSUPPORTED"), "{outcomes:?}");
}

/// A handle selects only a result this session holds natively from a fresh
/// solve. Every other handle is refused by name, and nothing is evaluated: the
/// refusal is the whole answer.
#[test]
fn a_handle_outside_the_sessions_native_custody_is_refused_by_name() {
    let (registry, gate) = registry();
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let pack = pack_for(&request["model"], &parsed, &chosen_row(&parsed));
    let args = |job: &str| json!({"rulePackDocument": pack, "jobId": job, "model": request["model"]});

    // The held fresh solve runs.
    let fresh = solve_holding(&registry, &gate, &text, ResultCustody::Native);
    assert!(run_by_handle(&registry, args(&fresh)).is_ok());

    // Unknown (never issued here: another session's or invented).
    let error = run_by_handle(&registry, args("job-from-another-session")).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD: job-from-another-session is not a result held in this session"), "{error}");

    // Disposed.
    registry.dispose_result(&fresh).unwrap();
    let error = run_by_handle(&registry, args(&fresh)).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");

    // Held in web view custody, before and after its take.
    let webview = solve_holding(&registry, &gate, &text, ResultCustody::Webview);
    let error = run_by_handle(&registry, args(&webview)).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD") && error.contains("not held natively"), "{error}");
    registry.take_result(&webview).unwrap();
    let error = run_by_handle(&registry, args(&webview)).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");
    registry.dispose_result(&webview).unwrap();

    // A restored saved run: historical standing only, refused as the web view refused it.
    let restored = restored_holding(&registry, &text);
    let error = run_by_handle(&registry, args(&restored)).unwrap_err();
    assert_eq!(error, "RULE_NATIVE_INVOCATION_REQUIRED: reference or saved data is not a fresh supported solve.");

    // A copy of the result is refused by name, with or without a handle.
    let held = solve_holding(&registry, &gate, &text, ResultCustody::Native);
    let mut copied = args(&held);
    copied["solvedEnvelope"] = parsed.clone();
    let error = run_by_handle(&registry, copied).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_SOLVED_ENVELOPE_RETIRED"), "{error}");

    // No handle; and no model (refused before the result is read, as before).
    let error = run_by_handle(&registry, json!({"rulePackDocument": pack, "model": request["model"]})).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_SOLVED_ENVELOPE_REQUIRED"), "{error}");
    let error = run_by_handle(&registry, json!({"rulePackDocument": pack, "jobId": "job-from-another-session"})).unwrap_err();
    assert!(error.starts_with("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE"), "{error}");
    registry.dispose_result(&held).unwrap();
    registry.dispose_result(&restored).unwrap();
}

/// The selector preview reads the held result with the run's own index and
/// row gate: the selector's row on each basis, an unsolved basis, an
/// unreadable selector; it reads a restored run too (it evaluates nothing) and
/// refuses a handle the session does not hold natively.
#[test]
fn the_selector_preview_reads_the_held_rows_by_handle() {
    let (registry, gate) = registry();
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let row = chosen_row(&parsed);
    let selector = serde_json::to_value(rule_solver_selection::selector_for_row(&parsed, &row).unwrap()).unwrap();
    let queries = json!([
        {"input_id": "actual", "selector": selector, "bases": [row["basis_ref"], {"ref_type": "load_case", "ref_id": "case:not-solved"}]},
        {"input_id": "broken", "selector": {"result_kind": ""}, "bases": [row["basis_ref"]]}
    ]);
    let fresh = solve_holding(&registry, &gate, &text, ResultCustody::Native);
    let restored = restored_holding(&registry, &text);
    for job in [&fresh, &restored] {
        let preview = preview_by_handle(&registry, json!({"jobId": job, "queries": queries})).unwrap();
        assert_eq!(preview["job_id"], json!(job));
        assert_eq!(preview["row_count"], json!(parsed["results"].as_array().unwrap().len()));
        let actual = &preview["inputs"][0];
        assert_eq!(actual["selector_readable"], json!(true));
        assert_eq!(actual["bases"][0]["solved"], json!(true));
        assert_eq!(actual["bases"][0]["rows"], json!([{"id": row["id"], "value": row["value"], "unit": row["unit"], "dimension": row.get("dimension").cloned().unwrap_or(Value::Null), "binding_refusal": null}]));
        assert_eq!(actual["bases"][1], json!({"basis_ref": {"ref_type": "load_case", "ref_id": "case:not-solved"}, "solved": false, "rows": []}));
        assert_eq!(preview["inputs"][1], json!({"input_id": "broken", "selector_readable": false, "bases": []}));
    }
    let error = preview_by_handle(&registry, json!({"jobId": "job-from-another-session", "queries": queries})).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");
    registry.dispose_result(&fresh).unwrap();
    let error = preview_by_handle(&registry, json!({"jobId": fresh, "queries": queries})).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");
    registry.dispose_result(&restored).unwrap();
}
