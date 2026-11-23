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
    run_rule_checks_args(&mut args, &no_store, registry)
        .map(|(text, _reply_lease)| serde_json::from_str(&text).unwrap())
        .map_err(|error| match error {
            CommandError::Command(message) => message,
            other => format!("{other:?}"),
        })
}

fn preview_by_handle(registry: &SolveJobRegistry, args: Value) -> Result<Value, String> {
    preview_within(registry, args, rule_solver_selection::MAX_PREVIEW_REPLY_BYTES)
}

/// The selector preview's argument path, through its admission, under a reply limit of `limit` bytes.
fn preview_within(registry: &SolveJobRegistry, args: Value, limit: u64) -> Result<Value, String> {
    let (mut args, _lease) = admit_json(registry, "preview_rule_solver_rows", ArgsDecoding::Moved, &args).expect("admitted");
    preview_rule_solver_rows_reply_within(&mut args, registry, limit)
        .map(|(text, _reply_lease)| serde_json::from_str(&text).unwrap())
        .map_err(|error| match error {
            CommandError::Command(message) => message,
            other => format!("{other:?}"),
        })
}

fn command_message(error: CommandError) -> String {
    match error {
        CommandError::Command(message) => message,
        other => format!("{other:?}"),
    }
}

fn ingress_body(registry: &SolveJobRegistry, body: Vec<u8>, admit: fn(&SolveJobRegistry, &tauri::ipc::InvokeBody, &tauri::http::HeaderMap) -> Result<(command_ingress::Args, crate::memory_pool::Lease), crate::solve_jobs::SolveJobError>) -> Result<(command_ingress::Args, crate::memory_pool::Lease), String> {
    let headers = crate::command_ingress::test_support::headers(Some(crate::command_ingress::INGRESS_ARGS_JSON_V1));
    admit(registry, &tauri::ipc::InvokeBody::Raw(body), &headers).map_err(|error| format!("{error:?}"))
}

/// The selector preview as its command answers raw bytes: its own admission
/// ([`admit_preview_args`]) and its reply under the production limit.
fn preview_command(registry: &SolveJobRegistry, body: Vec<u8>) -> Result<Value, String> {
    let (mut args, _lease) = ingress_body(registry, body, admit_preview_args)?;
    preview_rule_solver_rows_reply(&mut args, registry).map(|(text, _reply_lease)| serde_json::from_str(&text).unwrap()).map_err(command_message)
}

/// The rule check as its command answers raw bytes: its own admission
/// ([`admit_rule_check_args`]) and its reply under the production limits.
fn run_command(registry: &SolveJobRegistry, args: &Value) -> Result<Value, String> {
    let (mut args, _lease) = ingress_body(registry, serde_json::to_vec(args).unwrap(), admit_rule_check_args)?;
    run_rule_checks_args(&mut args, &no_store, registry).map(|(text, _reply_lease)| serde_json::from_str(&text).unwrap()).map_err(command_message)
}

/// Each outcome of `answer` takes at least its least reply bytes and at most its
/// priced text, and the pack's projection counts exactly the runner's outcomes.
fn outcomes_within_their_price(pack: &Value, answer: &Value, label: &str) {
    let outcomes = answer["checks"].as_array().unwrap();
    if outcomes.is_empty() {
        return;
    }
    let prices = rule_check_prices(pack);
    assert_eq!(rule_check_projection(pack).outcomes, outcomes.len() as u64, "{label}: the projection counts the runner's outcomes");
    let mut each = prices.iter().flat_map(|price| std::iter::repeat(price).take(price.outcomes as usize));
    for outcome in outcomes {
        let price = each.next().unwrap();
        let text = serde_json::to_string(outcome).unwrap().len() as u64 + 1;
        assert!(text >= price.least_text && text <= price.priced_text, "{label}: an outcome of {text} bytes, priced at {} to {}: {outcome}", price.least_text, price.priced_text);
    }
}

/// `pack` with `checks` copies of its first check and `cases` required load cases
/// on its mapping: its own first case, then cases naming load cases the model does
/// not hold (each such basis an incomplete outcome with its finding). The runner
/// makes `checks × cases` outcomes.
fn widened(pack: &Value, checks: usize, cases: usize) -> Value {
    let mut pack = pack.clone();
    let check = pack["check_definitions"][0].clone();
    let check_id = check["check_id"].as_str().unwrap().to_string();
    pack["check_definitions"] = Value::Array((0..checks).map(|at| {
        let mut copy = check.clone();
        copy["check_id"] = json!(format!("{check_id}_{at}"));
        copy
    }).collect());
    let case = pack["required_load_cases"][0].clone();
    let list: Vec<Value> = (0..cases).map(|at| {
        let mut copy = case.clone();
        if at > 0 {
            copy["required_load_case_id"] = json!(format!("widened_case_{at}"));
            copy["basis_ref"] = json!({"ref_type": "load_case", "ref_id": format!("invented:absent-{at}")});
        }
        copy
    }).collect();
    pack["load_case_mappings"][0]["required_load_case_refs"] = Value::Array(list.iter().map(|copy| json!({"ref_type": "required_load_case", "ref_id": copy["required_load_case_id"]})).collect());
    pack["required_load_cases"] = Value::Array(list);
    restamp(pack)
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

/// Supplied values under which the demo ratio (the bound row's value over the
/// supplied limit, against a ratio limit of 1) passes on every load case the pack
/// binds (the limit exceeds every value of the row's unit in `envelope`), or,
/// when `failing`, is 2 for the row and fails.
fn supplied_for(envelope: &Value, row: &Value, failing: bool) -> Value {
    let value = row["value"].as_f64().unwrap();
    let largest = envelope["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|candidate| candidate["unit"] == row["unit"])
        .filter_map(|candidate| candidate["value"].as_f64())
        .fold(0.0f64, |most, x| most.max(x.abs()));
    let limit = if failing { value / 2.0 } else { largest * 2.0 + 1.0 };
    json!([
        {"ref_id": "demo_limit_quantity", "value": limit, "unit": row["unit"], "dimension": "stress"},
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
    chosen_row_where(envelope, |row| open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(envelope, row).is_none())
}

/// A stress-unit row with a value, a basis and a selector that also meets `also`, else any such row.
fn chosen_row_where(envelope: &Value, also: impl Fn(&Value) -> bool) -> Value {
    let rows = envelope["results"].as_array().unwrap();
    let usable = |row: &&Value| {
        row["value"].as_f64().is_some()
            && row["unit"].is_string()
            && row["basis_ref"].is_object()
            && rule_solver_selection::selector_for_row(envelope, row).is_some()
            && also(row)
    };
    rows.iter().filter(usable).find(|row| row["unit"] == "Pa").or_else(|| rows.iter().find(usable)).cloned().expect("a bindable row")
}

/// What a parity case binds: an admitted row with a passing or failing ratio, a
/// row the producer published holding negative zero, or a row the binding gate
/// refuses. The result bytes are the committed ones in every case.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Bound {
    Passing,
    Failing,
    NegativeZero,
    Refused,
}

/// The case's row and supplied values for `bound`, from the committed result.
fn bound_case(text: &str, bound: Bound) -> (Value, Value, Value) {
    let parsed: Value = serde_json::from_str(text).unwrap();
    let refusal = |envelope: &Value, row: &Value| open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(envelope, row);
    let row = match bound {
        Bound::Passing => chosen_row(&parsed),
        // A stress row, which the demo pack's stress-dimension limit binds.
        Bound::NegativeZero => chosen_row_where(&parsed, |row| refusal(&parsed, row).is_none() && is_negative_zero(&row["value"]) && (row["unit"] == "MPa" || row["unit"] == "Pa")),
        Bound::Failing => chosen_row_where(&parsed, |row| refusal(&parsed, row).is_none() && row["value"].as_f64() != Some(0.0)),
        Bound::Refused => chosen_row_where(&parsed, |row| refusal(&parsed, row).is_some()),
    };
    match bound {
        Bound::Refused => assert!(refusal(&parsed, &row).is_some()),
        Bound::NegativeZero => assert!(is_negative_zero(&row["value"]), "{}", row["id"]),
        _ => {}
    }
    let supplied = supplied_for(&parsed, &row, bound == Bound::Failing);
    (parsed, row, supplied)
}

fn is_negative_zero(value: &Value) -> bool {
    value.as_f64().is_some_and(|x| x == 0.0 && x.is_sign_negative())
}

/// `value` with each negative zero written as zero: what the retired route's
/// `JSON.stringify` made of it.
fn zero_sign_blind(value: &Value) -> Value {
    match value {
        Value::Number(number) if number.is_f64() && number.as_f64() == Some(0.0) => json!(0.0),
        Value::Array(items) => Value::Array(items.iter().map(zero_sign_blind).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(key, item)| (key.clone(), zero_sign_blind(item))).collect()),
        other => other.clone(),
    }
}

/// Every place `value` holds a negative zero, as JSON pointers.
fn negative_zeros(value: &Value, at: String, found: &mut Vec<String>) {
    match value {
        Value::Array(items) => items.iter().enumerate().for_each(|(index, item)| negative_zeros(item, format!("{at}/{index}"), found)),
        Value::Object(map) => map.iter().for_each(|(key, item)| negative_zeros(item, format!("{at}/{key}"), found)),
        other if is_negative_zero(other) => found.push(at),
        _ => {}
    }
}

/// The desktop solve's committed result that publishes negative zeros among its
/// row values (preview-physics-2), and the model it was solved for.
const NEGATIVE_ZERO_RESULT: &str = "fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_sparse_interactive.json";
const NEGATIVE_ZERO_MODEL: &str = "fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json";

/// The committed fixtures the parity covers: model, result text and the
/// registered invocation each route sends (none, or the captured one).
/// Each case's label, model, result text, invocation, bound row and the outcome
/// it reaches by both routes (an aggregate status, or a refusal's code).
type ParityCase = (String, Value, String, Option<Value>, Bound, &'static str);

fn parity_cases() -> Vec<ParityCase> {
    let mut cases: Vec<ParityCase> = Vec::new();
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    for mode in ["sparse", "dense"] {
        let label = format!("pressure_connected_mechanics_{mode}");
        let text = project_text(&format!("fixtures/results/{label}.json"));
        cases.push((label.clone(), request["model"].clone(), text.clone(), None, Bound::Passing, "USER_RULE_CHECKED"));
        cases.push((format!("{label} failing"), request["model"].clone(), text, None, Bound::Failing, "USER_RULE_FAILED"));
    }
    // A row the desktop solve published holding negative zero, in its committed bytes. The result
    // is numerically sensitive, so both routes refuse it at standing before any row is bound: no
    // committed result that is numerically eligible publishes a bindable negative zero.
    cases.push(("rf_skew desktop result, negative zero row".to_string(), project_file(NEGATIVE_ZERO_MODEL), project_text(NEGATIVE_ZERO_RESULT), None, Bound::NegativeZero, "RULE_NUMERICAL_NEEDS_RECOMPUTE"));
    for name in ["connected", "pressure"] {
        let request = project_file(&format!("fixtures/product_preview/pressure/load_reference/{name}.request.json"));
        for mode in ["sparse_interactive", "dense_scrutiny"] {
            let label = format!("pressure/load_reference/{name}-{mode}");
            cases.push((label.clone(), request["model"].clone(), project_text(&format!("fixtures/product_preview/{label}.raw.json")), None, Bound::Passing, "USER_RULE_CHECKED"));
        }
    }
    // A component body's row, which the binding gate refuses (its reference section's stress), in
    // an eligible preview-physics-2 result: the run binds the row and reports it incomplete.
    let model = project_file("core/product_physics/tests/fixtures/preview_physics_invented_model.json");
    let label = "preview_physics_invented_sparse";
    cases.push((format!("{label} refused row"), model, project_text(&format!("fixtures/results/{label}.json")), None, Bound::Refused, "RULE_INPUTS_INCOMPLETE"));
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
                // A load/reference-source result is not numerically eligible, invocation or not.
                let with_invocation = if group == "physics_source" { "USER_RULE_CHECKED" } else { "RULE_NUMERICAL_NEEDS_RECOMPUTE" };
                cases.push((label.clone(), request["model"].clone(), text.clone(), Some(invocation), Bound::Passing, with_invocation));
                // Without its registered invocation: refused by both routes alike.
                cases.push((format!("{label} without invocation"), request["model"].clone(), text, None, Bound::Passing, "RULE_NUMERICAL_NEEDS_RECOMPUTE"));
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
/// on the exact held bytes), byte for byte in canonical form, and each case
/// reaches its stated outcome. A published negative zero is read as published by
/// handle and erased to `0` by the retired route's `JSON.stringify`; the one
/// committed result that publishes one in a stress row the pack can name is
/// refused at standing by both routes alike, so no bound sign differs here.
#[test]
fn a_rule_check_by_handle_answers_as_the_envelope_route_did() {
    let (registry, gate) = registry();
    let mut files = std::collections::BTreeSet::new();
    let cases = parity_cases();
    for (label, model, text, invocation, bound, expected) in &cases {
        files.insert(label.split(' ').next().unwrap().to_string());
        let (parsed, row, supplied) = bound_case(text, *bound);
        let pack = pack_for(model, &parsed, &row);
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
        let job = solve_holding(&registry, &gate, text, ResultCustody::Native);
        let mut args = json!({"rulePackDocument": pack.clone(), "jobId": job, "model": model, "suppliedValueBindings": supplied});
        if let Some(invocation) = invocation {
            args["sourceBlockInvocation"] = invocation.clone();
        }
        let by_handle = run_by_handle(&registry, args);
        if let Ok(answer) = &by_handle {
            outcomes_within_their_price(&pack, answer, label);
        }
        let canonical = |outcome: &Result<Value, String>| outcome.as_ref().map(|value| serde_json::to_string(value).unwrap()).map_err(Clone::clone);
        assert_eq!(canonical(&by_handle), canonical(&by_envelope), "{label}");
        let reached = match &by_handle {
            Ok(value) => value["aggregate_status"].as_str().unwrap().to_string(),
            Err(error) => error.split(':').next().unwrap().to_string(),
        };
        assert_eq!(reached, *expected, "{label}: {by_handle:?}");
        if *bound == Bound::Refused {
            let answer = serde_json::to_string(by_handle.as_ref().unwrap()).unwrap();
            assert!(answer.contains(open_pipe_stress_result_export::component_body::BODY_REFERENCE_SECTION), "{label}: {answer}");
        }
        if *bound == Bound::Failing {
            let checks = by_handle.as_ref().unwrap()["checks"].as_array().unwrap();
            assert!(checks.iter().any(|check| check["status"] == "USER_RULE_FAILED"), "{label}");
        }
        registry.dispose_result(&job).unwrap();
    }
    assert_eq!((cases.len(), files.len()), (34, 20), "the case and file counts the description states");
}

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
        assert_eq!(actual["bases"][0]["matched"], json!(1));
        assert_eq!(actual["bases"][0]["rows"], json!([{"id": row["id"], "value": row["value"], "unit": row["unit"], "dimension": row.get("dimension").cloned().unwrap_or(Value::Null), "binding_refusal": null}]));
        assert_eq!(actual["bases"][1], json!({"basis_ref": {"ref_type": "load_case", "ref_id": "case:not-solved"}, "solved": false, "matched": 0, "rows": []}));
        assert_eq!(preview["inputs"][1], json!({"input_id": "broken", "selector_readable": false, "bases": []}));
    }
    let error = preview_by_handle(&registry, json!({"jobId": "job-from-another-session", "queries": queries})).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");
    registry.dispose_result(&fresh).unwrap();
    let error = preview_by_handle(&registry, json!({"jobId": fresh, "queries": queries})).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");
    registry.dispose_result(&restored).unwrap();
}

/// A held result above the web view's 64 MiB full-copy limit is checked by handle. The result is a
/// committed exact result whose diagnostics are repeated (each with its own id) until its text
/// passes the limit: a size witness, not a solve of a larger model. The check by handle gives the
/// same aggregate and outcomes as the uninflated result by the engine's value path.
/// Ignored by default for its size (two parses of about 70 MB); run with `--ignored`.
#[test]
#[ignore = "builds and parses a result above 64 MiB; run on request"]
fn a_result_above_the_web_view_limit_is_checked_by_handle() {
    const WEBVIEW_LIMIT: usize = 64 * 1024 * 1024;
    let (registry, gate) = registry();
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let model = request["model"].clone();
    let mut source = project_file("fixtures/results/pressure_connected_mechanics_sparse.json");
    let row = chosen_row(&source);
    let pack = pack_for(&model, &source, &row);
    let supplied = supplied_for(&source, &row, false);
    let small = run_rule_checks_value(&no_store, pack.clone(), Some(model.clone()), Some(source.clone()), None, Some(supplied.clone()), None, None).unwrap();
    let template = source["diagnostics"][0].clone();
    let each = serde_json::to_string(&template).unwrap().len();
    let diagnostics = source["diagnostics"].as_array_mut().unwrap();
    for copy in 0..(WEBVIEW_LIMIT / each + 1000) {
        let mut entry = template.clone();
        entry["id"] = json!(format!("{}:size-witness-{copy}", template["id"].as_str().unwrap()));
        diagnostics.push(entry);
    }
    let text = serde_json::to_string(&source).unwrap();
    assert!(text.len() > WEBVIEW_LIMIT, "{} bytes", text.len());
    let job = solve_holding(&registry, &gate, &text, ResultCustody::Native);
    let by_handle = run_by_handle(&registry, json!({"rulePackDocument": pack, "jobId": job, "model": model, "suppliedValueBindings": supplied})).unwrap();
    assert_eq!(by_handle["aggregate_status"], small["aggregate_status"]);
    assert_eq!(by_handle["checks"], small["checks"]);
    registry.dispose_result(&job).unwrap();
}

/// Two registries in one process (the tests' case; the app has one) issue
/// distinct handles, and a real handle of one is refused by the other by name,
/// with nothing evaluated: by the rule check and the selector preview with
/// `RULE_CHECK_RESULT_NOT_HELD`, and by the existing readers and disposal with
/// their own refusal. Without its issue tag a registry's first handle is the same
/// text as another's, which is what the tag prevents.
#[test]
fn a_handle_issued_by_another_registry_is_refused_by_name() {
    let (first, first_gate) = registry();
    let first = first.with_issue_tag("a1");
    let (second, second_gate) = registry();
    let second = second.with_issue_tag("b2");
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let row = chosen_row(&parsed);
    let pack = pack_for(&request["model"], &parsed, &row);
    let held_first = solve_holding(&first, &first_gate, &text, ResultCustody::Native);
    let held_second = solve_holding(&second, &second_gate, &text, ResultCustody::Native);
    assert_ne!(held_first, held_second);
    assert_eq!(held_first.rsplit('-').next(), held_second.rsplit('-').next(), "both are their registry's first job");
    let args = |job: &str| json!({"rulePackDocument": pack, "jobId": job, "model": request["model"]});

    let error = run_by_handle(&second, args(&held_first)).unwrap_err();
    assert_eq!(error, format!("RULE_CHECK_RESULT_NOT_HELD: {held_first} is not a result held in this session"));
    let selector = serde_json::to_value(rule_solver_selection::selector_for_row(&parsed, &row).unwrap()).unwrap();
    let queries = json!([{"input_id": "actual", "selector": selector, "bases": [row["basis_ref"]]}]);
    let error = preview_by_handle(&second, json!({"jobId": held_first, "queries": queries})).unwrap_err();
    assert!(error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");
    // The existing handle readers and disposal refuse it with their own refusal.
    assert!(second.status(&held_first).is_err());
    assert!(second.held_result(&held_first).is_err());
    assert!(second.dispose_result(&held_first).is_err());
    // Each registry's own handle still reads, and the other's result is untouched.
    assert!(run_by_handle(&first, args(&held_first)).is_ok());
    assert!(run_by_handle(&second, args(&held_second)).is_ok());
    first.dispose_result(&held_first).unwrap();
    second.dispose_result(&held_second).unwrap();
}

/// The pressure fixture, its demo pack and a one-query preview on its chosen row.
struct Probe {
    model: Value,
    text: String,
    parsed: Value,
    row: Value,
    pack: Value,
    queries: Value,
}

fn probe() -> Probe {
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let row = chosen_row(&parsed);
    let pack = pack_for(&request["model"], &parsed, &row);
    let selector = serde_json::to_value(rule_solver_selection::selector_for_row(&parsed, &row).unwrap()).unwrap();
    let queries = json!([{"input_id": "actual", "selector": selector, "bases": [row["basis_ref"]]}]);
    Probe { model: request["model"].clone(), text, parsed, row, pack, queries }
}

/// `command`'s arguments for `job` in `probe`.
fn command_args(probe: &Probe, command: &str, job: &str) -> Value {
    match command {
        "run_rule_checks" => json!({"rulePackDocument": probe.pack, "jobId": job, "model": probe.model}),
        _ => json!({"jobId": job, "queries": probe.queries}),
    }
}

/// `command` through its argument path on another thread, held at the test gate
/// `stage` (closed); the job is disposed while the command waits there, then the
/// gate opens. Returns the command's answer.
fn disposed_at(registry: &SolveJobRegistry, command: &'static str, stage: &str, job: &str, args: Value) -> Result<Value, String> {
    held_at(registry, command, stage, args, || {
        registry.dispose_result(job).unwrap();
    })
}

/// `command` through its argument path on another thread, held at the test gate
/// `stage` (closed) while `meanwhile` runs, then released. Returns its answer.
fn held_at(registry: &SolveJobRegistry, command: &'static str, stage: &str, args: Value, meanwhile: impl FnOnce()) -> Result<Value, String> {
    let held = crate::native_verdict::tests::gate::hold(registry, stage, false);
    let reader = registry.clone();
    let answer = std::thread::spawn(move || match command {
        "run_rule_checks" => run_by_handle(&reader, args),
        _ => preview_by_handle(&reader, args),
    });
    assert!(held.0.wait_entered(1, std::time::Duration::from_secs(60)), "{command}: the read did not reach {stage}");
    meanwhile();
    held.0.open();
    let outcome = answer.join().unwrap();
    drop(held);
    outcome
}

fn released(job: &str) -> String {
    format!("RULE_CHECK_RESULT_NOT_HELD: {job} was released while it was read; nothing it read is returned")
}

/// DEL-07-07 REQ-015: a result disposed at any point after the rule check or the
/// preview took its share, and before the answer is returned, gives the refusal
/// and nothing read: while its bytes are read, while the preview waits for its
/// reply's lease, while either serializes its answer, and at the final check.
#[test]
fn a_result_disposed_while_it_is_read_is_refused_and_nothing_is_returned() {
    let (registry, gate) = registry();
    let probe = probe();
    for (command, stage) in [
        ("run_rule_checks", "rule-check-read"),
        ("run_rule_checks", "rule-check-serialize"),
        ("run_rule_checks", "rule-check-answer"),
        ("preview_rule_solver_rows", "rule-check-read"),
        ("preview_rule_solver_rows", "rule-check-reply-lease"),
        ("preview_rule_solver_rows", "rule-check-serialize"),
        ("preview_rule_solver_rows", "rule-check-answer"),
    ] {
        let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
        // Undisturbed, the command answers.
        let undisturbed = match command {
            "run_rule_checks" => run_by_handle(&registry, command_args(&probe, command, &job)),
            _ => preview_by_handle(&registry, command_args(&probe, command, &job)),
        };
        assert!(undisturbed.is_ok(), "{command}: {undisturbed:?}");
        let error = disposed_at(&registry, command, stage, &job, command_args(&probe, command, &job)).unwrap_err();
        assert_eq!(error, released(&job), "{command} at {stage}");
    }
}

/// A refusal formed after the share was taken is an answer read from the result:
/// once the result is released it is not returned, and the not-held refusal is.
/// Each early refusal is reached undisturbed, then with the result disposed while
/// its bytes are read: a number outside the checked JSON profile (both commands),
/// a parse lease the memory pool refuses (both), the engine's own refusal of the
/// result (the run), and a held result that is not JSON (the preview, on a
/// restored result, which it reads).
#[test]
fn an_early_refusal_is_not_returned_once_the_result_is_released() {
    let host = crate::memory_pool::test_host::ScriptedHost::normal();
    let gate = Gate::new("rule-check-early");
    let pool = crate::memory_pool::MemoryPool::with_timing(host.clone(), std::time::Duration::from_millis(5), std::time::Duration::from_millis(40), std::time::Duration::from_millis(5));
    pool.set_max_running_workers(4);
    let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)));
    let probe = probe();
    let unsafe_text = {
        let mut source = probe.parsed.clone();
        let other = source["results"].as_array().unwrap().iter().position(|candidate| candidate["id"] != probe.row["id"]).unwrap();
        source["results"][other]["value"] = json!("PLACEHOLDER");
        serde_json::to_string(&source).unwrap().replace("\"PLACEHOLDER\"", "9007199254740992")
    };
    let pressure_source = project_file("fixtures/product_preview/pressure_source/physics_source/n05.request.json");
    let pressure_source_text = project_text("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");
    let pressure_source_parsed: Value = serde_json::from_str(&pressure_source_text).unwrap();
    let pressure_source_pack = pack_for(&pressure_source["model"], &pressure_source_parsed, &chosen_row(&pressure_source_parsed));
    let restored = |text: &str| restored_holding(&registry, text);
    let run = |job: &str| run_by_handle(&registry, command_args(&probe, "run_rule_checks", job));
    let preview = |job: &str| preview_by_handle(&registry, command_args(&probe, "preview_rule_solver_rows", job));

    // Outside the profile.
    for command in ["run_rule_checks", "preview_rule_solver_rows"] {
        let job = solve_holding(&registry, &gate, &unsafe_text, ResultCustody::Native);
        let error = if command == "run_rule_checks" { run(&job) } else { preview(&job) }.unwrap_err();
        assert!(error.starts_with("RULE_NATIVE_INVOCATION_REQUIRED: CHECKED-JSON-NUMBER-OUTSIDE-PROFILE"), "{command}: {error}");
        assert_eq!(disposed_at(&registry, command, "rule-check-read", &job, command_args(&probe, command, &job)).unwrap_err(), released(&job), "{command}");
    }
    // The parse lease refused: once the command is admitted and holds its share, the pool
    // has too little headroom for the parse.
    let scarce = || host.set(|script| script.headroom_bytes = Some(1024));
    let ample = || host.set(|script| script.headroom_bytes = Some(64 * crate::memory_pool::test_host::GIB));
    for command in ["run_rule_checks", "preview_rule_solver_rows"] {
        let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
        let error = held_at(&registry, command, "rule-check-read", command_args(&probe, command, &job), scarce).unwrap_err();
        ample();
        assert!(error.contains("Headroom") || error.contains("headroom") || error.contains("RESOURCE"), "{command}: {error}");
        let error = held_at(&registry, command, "rule-check-read", command_args(&probe, command, &job), || {
            scarce();
            registry.dispose_result(&job).unwrap();
        })
        .unwrap_err();
        ample();
        assert_eq!(error, released(&job), "{command}");
    }
    // The engine's refusal: a pressure-source-1 result without its registered invocation.
    let job = solve_holding(&registry, &gate, &pressure_source_text, ResultCustody::Native);
    let args = json!({"rulePackDocument": pressure_source_pack, "jobId": job, "model": pressure_source["model"]});
    assert!(run_by_handle(&registry, args.clone()).unwrap_err().starts_with("RULE_NUMERICAL_NEEDS_RECOMPUTE"));
    assert_eq!(disposed_at(&registry, "run_rule_checks", "rule-check-read", &job, args).unwrap_err(), released(&job));
    // A held text that is not JSON, read by the preview (a restored result).
    let job = restored("{\"results\": [");
    let error = preview(&job).unwrap_err();
    assert!(!error.starts_with("RULE_CHECK_RESULT_NOT_HELD"), "{error}");
    assert_eq!(disposed_at(&registry, "preview_rule_solver_rows", "rule-check-read", &job, command_args(&probe, "preview_rule_solver_rows", &job)).unwrap_err(), released(&job));
}

/// A scripted host that, once armed, reports the next sample the pool takes: with headroom made
/// scarce, that is the pool sampling for a lease it cannot yet grant, so the command is waiting.
/// It also keeps the headroom of the last sample taken.
struct LeaseWaitHost {
    script: Arc<crate::memory_pool::test_host::ScriptedHost>,
    armed: std::sync::Mutex<Option<std::sync::mpsc::Sender<()>>>,
    last_headroom: std::sync::Mutex<Option<u64>>,
}

impl crate::memory_pool::HostMemory for LeaseWaitHost {
    fn sample(&self) -> crate::memory_pool::HostSample {
        let sample = crate::memory_pool::HostMemory::sample(&*self.script);
        *self.last_headroom.lock().unwrap() = sample.headroom_bytes;
        if let Some(waiting) = self.armed.lock().unwrap().take() {
            let _ = waiting.send(());
        }
        sample
    }
}

/// DEL-07-07 REQ-015, while a lease is awaited: the result is disposed while the command waits
/// in the memory pool for its parse lease (both commands) or for the preview's reply lease. The
/// headroom is made scarce at the stage before the lease, so the pool refuses its first attempt
/// and waits; the disposal is made only once the pool has sampled for that lease, and headroom is
/// freed only after the disposal, so the lease cannot be granted before it. Each wait ends in a
/// grant (the last sample saw the freed headroom). Undisturbed, the command then answers;
/// disposed, it gives the not-held refusal and nothing read.
#[test]
fn a_result_disposed_while_its_lease_is_awaited_is_refused() {
    use crate::memory_pool::test_host::{ScriptedHost, GIB};
    let script = ScriptedHost::normal();
    let host = Arc::new(LeaseWaitHost { script: script.clone(), armed: std::sync::Mutex::new(None), last_headroom: std::sync::Mutex::new(None) });
    let gate = Gate::new("rule-check-lease-wait");
    let tick = std::time::Duration::from_millis(5);
    let pool = crate::memory_pool::MemoryPool::with_timing(host.clone(), tick, std::time::Duration::from_secs(30), tick);
    pool.set_max_running_workers(4);
    let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)));
    let probe = probe();
    let scarce = || script.set(|s| s.headroom_bytes = Some(1024));
    let ample = || script.set(|s| s.headroom_bytes = Some(64 * GIB));
    for (command, stage) in [
        ("run_rule_checks", "rule-check-read"),
        ("preview_rule_solver_rows", "rule-check-read"),
        ("preview_rule_solver_rows", "rule-check-reply-lease"),
    ] {
        for dispose in [false, true] {
            let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
            let (waiting, wait_began) = std::sync::mpsc::channel();
            let outcome = std::thread::scope(|scope| {
                let (disposer, held_job, free) = (&registry, &job, &ample);
                let freer = scope.spawn(move || {
                    wait_began.recv_timeout(std::time::Duration::from_secs(60)).unwrap_or_else(|_| panic!("{command} at {stage}: the command did not wait for its lease"));
                    if dispose {
                        disposer.dispose_result(held_job).unwrap();
                    }
                    free();
                });
                let outcome = held_at(&registry, command, stage, command_args(&probe, command, &job), || {
                    scarce();
                    *host.armed.lock().unwrap() = Some(waiting);
                });
                freer.join().unwrap();
                outcome
            });
            assert_eq!(*host.last_headroom.lock().unwrap(), Some(64 * GIB), "{command} at {stage}: the wait ended in a grant");
            if dispose {
                assert_eq!(outcome.unwrap_err(), released(&job), "{command} at {stage}");
            } else {
                outcome.unwrap_or_else(|error| panic!("{command} at {stage}, undisturbed: {error}"));
                registry.dispose_result(&job).unwrap();
            }
        }
    }
}

/// A held result holding an integral number outside the safe integer range,
/// which the web view's checked JSON profile refused at registration, is still
/// refused by the rule check and the preview, by name, with nothing evaluated;
/// the largest safe integer is admitted.
#[test]
fn a_held_result_outside_the_checked_json_profile_is_refused() {
    let (registry, gate) = registry();
    let probe = probe();
    let other = probe.parsed["results"].as_array().unwrap().iter().position(|candidate| candidate["id"] != probe.row["id"]).unwrap();
    for (written, refused) in [("9007199254740992", true), ("-9007199254740992", true), ("1.8e19", true), ("9007199254740993.0", true), ("9007199254740991", false), ("1.5e300", true)] {
        let mut source = probe.parsed.clone();
        source["results"][other]["value"] = json!("PLACEHOLDER");
        let held_text = serde_json::to_string(&source).unwrap().replace("\"PLACEHOLDER\"", written);
        let job = solve_holding(&registry, &gate, &held_text, ResultCustody::Native);
        let run = run_by_handle(&registry, command_args(&probe, "run_rule_checks", &job));
        let preview = preview_by_handle(&registry, command_args(&probe, "preview_rule_solver_rows", &job));
        for outcome in [run.map(drop), preview.map(drop)] {
            match outcome {
                Err(error) if refused => assert!(error.starts_with("RULE_NATIVE_INVOCATION_REQUIRED: CHECKED-JSON-NUMBER-OUTSIDE-PROFILE"), "{written}: {error}"),
                Ok(()) if !refused => {}
                other => panic!("{written}: {other:?}"),
            }
        }
        registry.dispose_result(&job).unwrap();
    }
}

/// The profile walk fails closed: a number with no finite binary64 reading is
/// outside it. (The JSON parser admits no such number today; the walk is checked
/// on values built directly.)
#[test]
fn the_profile_walk_refuses_numbers_it_cannot_read_as_finite() {
    assert!(!outside_checked_json_profile(&json!([0.5, -0.0, 9007199254740991u64, {"a": -9007199254740991i64}])));
    assert!(outside_checked_json_profile(&json!({"a": [9007199254740992u64]})));
    // serde_json builds no non-finite Number; the walk's f64 branch is reached for every float.
    assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
    assert!(outside_checked_json_profile(&json!(1.0e300)));
}

/// A raw body above the preview's argument cap is refused at admission, before
/// any prescan or parse: a body that is not JSON at all is refused the same way.
#[test]
fn the_selector_preview_refuses_an_oversized_body_before_it_is_parsed() {
    let (registry, _gate) = registry();
    let cap = rule_solver_selection::MAX_PREVIEW_ARGS_BYTES;
    assert_eq!(cap, 5_049_241, "the cap the description derives");
    for body in [vec![b'['; cap + 1], {
        let mut body = br#"{"jobId":"j","queries":[],"pad":""#.to_vec();
        body.resize(cap, b'x');
        body.extend_from_slice(br#""}"#);
        body
    }] {
        let error = preview_command(&registry, body.clone()).unwrap_err();
        assert_eq!(error, format!("{:?}", crate::solve_jobs::SolveJobError::InvalidRequest { message: format!("INGRESS-ARGS-TOO-LARGE: `preview_rule_solver_rows` takes at most {cap} bytes of arguments; {} were sent.", body.len()) }));
    }
    // At the cap, the command admits the body to its prescan and parse, and answers past the size check.
    let mut body = br#"{"jobId":"j","queries":[],"pad":""#.to_vec();
    body.resize(cap - 2, b'x');
    body.extend_from_slice(br#""}"#);
    assert_eq!(body.len(), cap);
    let error = preview_command(&registry, body).unwrap_err();
    assert!(!error.contains("INGRESS-ARGS-TOO-LARGE"), "{error}");
}

/// The selector preview's queries are read and bounded before the result is
/// read, every refusal names positions and echoes no identifier, a basis is one
/// by its parsed identity, and the reply is refused as soon as its running
/// length passes the limit, before the rest of it is built.
#[test]
fn the_selector_preview_is_bounded_and_refuses_by_name() {
    let (registry, gate) = registry();
    let probe = probe();
    let row = &probe.row;
    let selector = probe.queries[0]["selector"].clone();
    let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
    let invalid = |queries: Value| {
        let error = preview_by_handle(&registry, json!({"jobId": job, "queries": queries})).unwrap_err();
        assert!(error.starts_with("RULE_SOLVER_PREVIEW_REQUEST_INVALID: "), "{error}");
        assert!(error.len() < 200, "a bounded refusal: {error}");
        error
    };
    let query = |input: usize, bases: Vec<Value>| json!({"input_id": format!("input-{input}"), "selector": selector, "bases": bases});
    let case = |n: usize| json!({"ref_type": "load_case", "ref_id": format!("case:{n}")});
    // Shapes.
    invalid(json!({"not": "an array"}));
    invalid(json!(["not an object"]));
    invalid(json!([{"input_id": 7, "selector": selector, "bases": []}]));
    invalid(json!([{"input_id": "", "selector": selector, "bases": []}]));
    invalid(json!([{"input_id": "a", "selector": selector, "bases": {}}]));
    invalid(json!([query(0, vec![json!({"ref_type": "elsewhere", "ref_id": "x"})])]));
    // Counts: more than 128 queries, or 128 bases in one query, refused; exactly 128 admitted.
    invalid(Value::Array((0..=rule_solver_selection::MAX_PREVIEW_QUERIES).map(|input| query(input, vec![row["basis_ref"].clone()])).collect()));
    invalid(json!([query(0, (0..=rule_solver_selection::MAX_PREVIEW_BASES_PER_QUERY).map(case).collect())]));
    let full = preview_by_handle(&registry, json!({"jobId": job, "queries": [query(0, (0..rule_solver_selection::MAX_PREVIEW_BASES_PER_QUERY).map(case).collect())]})).unwrap();
    assert_eq!(full["inputs"][0]["bases"].as_array().unwrap().len(), rule_solver_selection::MAX_PREVIEW_BASES_PER_QUERY);
    let most = Value::Array((0..rule_solver_selection::MAX_PREVIEW_QUERIES).map(|input| query(input, vec![row["basis_ref"].clone()])).collect());
    assert_eq!(preview_by_handle(&registry, json!({"jobId": job, "queries": most})).unwrap()["inputs"].as_array().unwrap().len(), rule_solver_selection::MAX_PREVIEW_QUERIES);
    // Repeats: one input twice; one basis twice; the extra-key alias of a basis is the same
    // basis by its parsed identity, refused as a repeat; a lone extra key is refused by shape.
    invalid(json!([query(0, vec![row["basis_ref"].clone()]), query(0, vec![row["basis_ref"].clone()])]));
    let twice = invalid(json!([query(0, vec![row["basis_ref"].clone(), row["basis_ref"].clone()])]));
    assert_eq!(twice, "RULE_SOLVER_PREVIEW_REQUEST_INVALID: query 0's basis 1 names a basis the query already names");
    let mut alias = row["basis_ref"].clone();
    alias["note"] = json!("an alias");
    assert_eq!(invalid(json!([query(0, vec![row["basis_ref"].clone(), alias.clone()])])), twice);
    assert_eq!(invalid(json!([query(0, vec![alias])])), "RULE_SOLVER_PREVIEW_REQUEST_INVALID: query 0's basis 0 must carry only ref_type and ref_id");
    // Long identifiers are refused without being echoed.
    let long = "x".repeat(10_000);
    for queries in [
        json!([{"input_id": long, "selector": selector, "bases": []}]),
        json!([{"input_id": "a", "selector": {"result_kind": long, "entity_ref": "e"}, "bases": []}]),
        json!([query(0, vec![json!({"ref_type": "load_case", "ref_id": long})])]),
    ] {
        let error = invalid(queries);
        assert!(!error.contains(&"x".repeat(rule_solver_selection::MAX_PREVIEW_IDENTIFIER_BYTES / 4)), "{error}");
    }
    // A selector naming more than one row on a basis returns none of them, only
    // that it names more than one; here the result holds the row twice.
    let mut doubled = probe.parsed.clone();
    let mut copy = row.clone();
    copy["id"] = json!(format!("{}:copy", row["id"].as_str().unwrap()));
    doubled["results"].as_array_mut().unwrap().push(copy);
    let ambiguous = solve_holding(&registry, &gate, &serde_json::to_string(&doubled).unwrap(), ResultCustody::Native);
    let preview = preview_by_handle(&registry, json!({"jobId": ambiguous, "queries": [query(0, vec![row["basis_ref"].clone()])]})).unwrap();
    let basis = &preview["inputs"][0]["bases"][0];
    assert_eq!((&basis["matched"], &basis["rows"]), (&json!(2), &json!([])));
    registry.dispose_result(&ambiguous).unwrap();

    // The running limit: 128 queries, each binding one row. Unbounded, the count
    // and the write build each row once (256); under a limit a few rows long, the
    // count stops at the limit, having built only the rows before it.
    let rows_built = || rule_solver_selection::PREVIEW_ROWS_BUILT.with(std::cell::Cell::get);
    let reset = || rule_solver_selection::PREVIEW_ROWS_BUILT.with(|count| count.set(0));
    let most = Value::Array((0..rule_solver_selection::MAX_PREVIEW_QUERIES).map(|input| query(input, vec![row["basis_ref"].clone()])).collect());
    reset();
    let answer = preview_within(&registry, json!({"jobId": job, "queries": most}), rule_solver_selection::MAX_PREVIEW_REPLY_BYTES).unwrap();
    assert_eq!(rows_built(), 2 * rule_solver_selection::MAX_PREVIEW_QUERIES);
    let whole = serde_json::to_string(&answer).unwrap().len() as u64;
    reset();
    let limit = whole / 16;
    let error = preview_within(&registry, json!({"jobId": job, "queries": most}), limit).unwrap_err();
    assert_eq!(error, format!("RULE_SOLVER_PREVIEW_RESPONSE_LIMIT: the preview's reply passes its {limit}-byte limit; preview fewer inputs or load cases"));
    assert!(rows_built() <= rule_solver_selection::MAX_PREVIEW_QUERIES / 16 + 1, "{} rows built before the refusal", rows_built());
    // The reply is the exact text the count measured: the limit at its length admits it.
    reset();
    assert_eq!(preview_within(&registry, json!({"jobId": job, "queries": most}), whole).unwrap(), answer);
    registry.dispose_result(&job).unwrap();
}

/// A published negative zero, which the desktop solve writes, is read as it was
/// published: the desktop's committed result holding negative zeros among its
/// row values is admitted unchanged by the result reader and read by handle,
/// reaching its own standing (the parity case on one of its -0.0 rows). A result whose reduced-section row differs from its nominal twin
/// in that one field, -0.0 against 0.0, is refused by the result reader's own
/// twin check ([`TWIN_MISMATCH_READER_CODE`]), by handle; the retired route's
/// `JSON.stringify` erased the sign and so masked the mismatch.
#[test]
fn a_published_negative_zero_is_read_as_published() {
    let (registry, gate) = registry();
    // The unchanged producer result: admitted.
    let model = project_file(NEGATIVE_ZERO_MODEL);
    let text = project_text(NEGATIVE_ZERO_RESULT);
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let mut zeros = Vec::new();
    negative_zeros(&parsed["results"], String::new(), &mut zeros);
    assert!(zeros.len() >= 5, "{zeros:?}");
    let verdict: Value = serde_json::from_str(&open_pipe_stress_operation_applier::reader_dispatch_json(&text, Some(&serde_json::to_string(&model).unwrap()), None).unwrap()).unwrap();
    assert!(verdict["error"].is_null(), "{verdict}");
    // By handle the held bytes are read as published, -0.0 and all: the run reaches the result's
    // standing, which is not numerically eligible (the result is numerically sensitive), and
    // refuses it there exactly as the retired route did on its sign-erased copy.
    let row = chosen_row(&parsed);
    let (pack, supplied) = (pack_for(&model, &parsed, &row), supplied_for(&parsed, &row, false));
    let job = solve_holding(&registry, &gate, &text, ResultCustody::Native);
    let by_handle = run_by_handle(&registry, json!({"rulePackDocument": pack, "jobId": job, "model": model, "suppliedValueBindings": supplied}));
    let by_envelope = run_rule_checks_value(&no_store, pack, Some(model.clone()), Some(as_web_view_sent(&parsed)), None, Some(supplied), None, None);
    assert_eq!(by_handle.unwrap_err(), "RULE_NUMERICAL_NEEDS_RECOMPUTE: complete passing or source-qualified numerical evidence is required for every current load case");
    assert_eq!(by_envelope.unwrap_err(), "RULE_NUMERICAL_NEEDS_RECOMPUTE: complete passing or source-qualified numerical evidence is required for every current load case");
    registry.dispose_result(&job).unwrap();

    // The one-field twin mismatch: a reduced-section row whose nominal twin holds 0.0.
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let mut parsed: Value = serde_json::from_str(&text).unwrap();
    let pack = pack_for(&request["model"], &parsed, &chosen_row(&parsed));
    let reduced = open_pipe_stress_result_export::section_basis::REDUCED;
    let rows = parsed["results"].as_array().unwrap();
    let at = rows
        .iter()
        .position(|row| {
            let id = row["id"].as_str().unwrap_or_default();
            open_pipe_stress_result_export::section_basis::id_basis(id) == Some(reduced)
                && row["value"].as_f64() == Some(0.0)
                && open_pipe_stress_result_export::section_basis::twin_id(id)
                    .and_then(|twin| rows.iter().find(|candidate| candidate["id"] == twin.as_str()))
                    .is_some_and(|twin| twin["value"].as_f64() == Some(0.0) && !is_negative_zero(&twin["value"]))
        })
        .expect("a reduced-section zero row with a zero nominal twin");
    let twin_at = {
        let twin = open_pipe_stress_result_export::section_basis::twin_id(rows[at]["id"].as_str().unwrap()).unwrap();
        rows.iter().position(|candidate| candidate["id"] == twin.as_str()).unwrap()
    };
    let model_text = serde_json::to_string(&request["model"]).unwrap();
    let standing = |text: &str| -> Value {
        let verdict: Value = serde_json::from_str(&open_pipe_stress_operation_applier::reader_dispatch_json(text, Some(&model_text), None).unwrap()).unwrap();
        assert!(verdict["error"].is_null(), "{verdict}");
        verdict["standing"].clone()
    };
    assert_eq!(standing(&text)["eligible"], json!(true), "the committed result");
    parsed["results"][at]["value"] = json!(-0.0);
    let mismatched = serde_json::to_string(&parsed).unwrap();
    let found = standing(&mismatched);
    assert_eq!((&found["eligible"], &found["findings"]), (&json!(false), &json!([TWIN_MISMATCH_READER_CODE])), "{found}");
    // Both twins written -0.0: they agree again, and the reader's finding is gone.
    let mut both = parsed.clone();
    both["results"][twin_at]["value"] = json!(-0.0);
    assert!(!standing(&serde_json::to_string(&both).unwrap())["findings"].as_array().unwrap().contains(&json!(TWIN_MISMATCH_READER_CODE)));
    let job = solve_holding(&registry, &gate, &mismatched, ResultCustody::Native);
    let error = run_by_handle(&registry, json!({"rulePackDocument": pack, "jobId": job, "model": request["model"]})).unwrap_err();
    assert_eq!(error, "RULE_NUMERICAL_SOURCE_UNSUPPORTED: recompute with a supported producer and semantic contract");
    registry.dispose_result(&job).unwrap();
    // The same bytes with the sign erased, as the retired route's `JSON.stringify` erased it: the
    // twins are equal again, and the mismatch is masked.
    let erased = serde_json::to_string(&zero_sign_blind(&parsed)).unwrap();
    assert_eq!(standing(&erased)["eligible"], json!(true), "the sign erased");
}

/// The result reader's standing finding for a reduced-section row that differs from its nominal
/// twin where the two sections coincide: `section_basis::twin_values` requires the twins to be
/// bit-equal there ("a signed zero is not its twin's zero").
const TWIN_MISMATCH_READER_CODE: &str = "SOURCE_PHYSICS_ROW_BASIS_SECTION";

/// One query of the preview, on `row`'s selector and basis.
fn row_query(envelope: &Value, row: &Value) -> Value {
    let selector = rule_solver_selection::selector_for_row(envelope, row).expect("the row has a selector");
    json!({"input_id": "in_0", "selector": serde_json::to_value(&selector).unwrap(), "bases": [row["basis_ref"].clone()]})
}

/// A single basis entry longer than the whole reply limit, through the command: a
/// matched row whose id is one byte longer than the limit. The row's members are
/// written borrowed, in pieces of at most `PREVIEW_WRITE_PIECE_BYTES`, so the count
/// refuses by name within one piece of its limit, the row built once and never
/// copied whole.
#[test]
fn the_selector_preview_refuses_an_oversized_entry_within_a_piece_of_its_limit() {
    let (registry, gate) = registry();
    let probe = probe();
    let mut parsed = probe.parsed.clone();
    let at = parsed["results"].as_array().unwrap().iter().position(|candidate| candidate["id"] == probe.row["id"]).unwrap();
    let limit = rule_solver_selection::MAX_PREVIEW_REPLY_BYTES;
    parsed["results"][at]["id"] = json!("r".repeat(limit as usize + 1));
    let row = parsed["results"][at].clone();
    let job = solve_holding(&registry, &gate, &serde_json::to_string(&parsed).unwrap(), ResultCustody::Native);
    rule_solver_selection::PREVIEW_LARGEST_PIECE.with(|largest| largest.set(0));
    rule_solver_selection::PREVIEW_ROWS_BUILT.with(|count| count.set(0));
    let body = serde_json::to_vec(&json!({"jobId": job, "queries": [row_query(&parsed, &row)]})).unwrap();
    let error = preview_command(&registry, body).unwrap_err();
    assert_eq!(error, format!("RULE_SOLVER_PREVIEW_RESPONSE_LIMIT: the preview's reply passes its {limit}-byte limit; preview fewer inputs or load cases"));
    assert!(rule_solver_selection::PREVIEW_LARGEST_PIECE.with(std::cell::Cell::get) <= rule_solver_selection::PREVIEW_WRITE_PIECE_BYTES);
    assert_eq!(rule_solver_selection::PREVIEW_ROWS_BUILT.with(std::cell::Cell::get), 1);
    registry.dispose_result(&job).unwrap();
}

/// A selector naming many rows on a basis, through the command: the selection
/// stops at the second row, which is enough to say "more than one", and no row is
/// returned.
#[test]
fn an_ambiguous_selection_stops_at_its_second_row() {
    let (registry, gate) = registry();
    let probe = probe();
    let mut parsed = probe.parsed.clone();
    for copy in 0..100 {
        let mut row = probe.row.clone();
        row["id"] = json!(format!("{}:copy-{copy}", probe.row["id"].as_str().unwrap()));
        parsed["results"].as_array_mut().unwrap().push(row);
    }
    let job = solve_holding(&registry, &gate, &serde_json::to_string(&parsed).unwrap(), ResultCustody::Native);
    rule_solver_selection::PREVIEW_MOST_SELECTED.with(|most| most.set(0));
    rule_solver_selection::PREVIEW_ROWS_EXAMINED.with(|count| count.set(0));
    let body = serde_json::to_vec(&json!({"jobId": job, "queries": [row_query(&parsed, &probe.row)]})).unwrap();
    let preview = preview_command(&registry, body).unwrap();
    let basis = &preview["inputs"][0]["bases"][0];
    assert_eq!((&basis["matched"], &basis["rows"]), (&json!(2), &json!([])));
    assert_eq!(rule_solver_selection::PREVIEW_MOST_SELECTED.with(std::cell::Cell::get), 2);
    // The selection stopped at its second match: the copies follow every original
    // candidate of the row's basis, kind and entity, so the second match is the first
    // copy, and no candidate after it is examined (100 more copies match). The reply is
    // emitted twice, counted and then written, so each pass examines that many.
    let originals = probe.parsed["results"].as_array().unwrap().iter()
        .filter(|row| row["basis_ref"] == probe.row["basis_ref"] && row["kind"] == probe.row["kind"] && row["entity_ref"] == probe.row["entity_ref"] && !rule_solver_selection::is_held_operand_row(row))
        .count();
    let examined = rule_solver_selection::PREVIEW_ROWS_EXAMINED.with(std::cell::Cell::get);
    assert_eq!(examined, 2 * (originals + 1), "candidates examined over both passes, of {} with 101 matching", originals + 100);
    registry.dispose_result(&job).unwrap();
}

/// A rule check whose projected outcomes cannot fit its reply, through the
/// command: refused by name from the pack alone, before custody is read (a handle
/// naming nothing gets the same refusal) and before the runner starts.
#[test]
fn a_rule_check_too_large_for_its_reply_is_refused_before_it_runs() {
    let (registry, gate) = registry();
    let probe = probe();
    let pack = widened(&pack_for(&probe.model, &probe.parsed, &probe.row), 200, 250);
    let projection = rule_check_projection(&pack);
    assert_eq!(projection.outcomes, 50_000);
    assert!(projection.least_bytes > MAX_RULE_CHECK_REPLY_BYTES);
    let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
    let started = RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get);
    let expected = format!(
        "RULE_CHECK_RESPONSE_LIMIT: the rule check would return 50000 outcomes, at least {} bytes, past its {MAX_RULE_CHECK_REPLY_BYTES}-byte reply limit; check fewer load cases or checks",
        projection.least_bytes
    );
    for handle in [job.as_str(), "a-handle-naming-nothing"] {
        let error = run_command(&registry, &json!({"rulePackDocument": pack, "jobId": handle, "model": probe.model})).unwrap_err();
        assert_eq!(error, expected, "{handle}");
    }
    assert_eq!(RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get), started, "the runner did not start");
    registry.dispose_result(&job).unwrap();
}

/// A rule check's working memory is leased at its price before the runner
/// starts: refused by the pool, by its own typed refusal, when the host has too
/// little headroom for it; with headroom, the same run reaches the runner.
#[test]
fn a_rule_check_leases_its_priced_work_before_it_runs() {
    let host = crate::memory_pool::test_host::ScriptedHost::normal();
    let gate = Gate::new("rule-check-work");
    let pool = crate::memory_pool::MemoryPool::with_timing(host.clone(), std::time::Duration::from_millis(5), std::time::Duration::from_millis(40), std::time::Duration::from_millis(5));
    pool.set_max_running_workers(4);
    let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)));
    let probe = probe();
    let pack = widened(&pack_for(&probe.model, &probe.parsed, &probe.row), 1000, 2);
    let projection = rule_check_projection(&pack);
    let prices = rule_check_prices(&pack);
    assert_eq!(projection.outcomes, 2000);
    assert_eq!(projection.work_bytes, prices.iter().map(|price| price.outcomes * price.priced_text * RULE_CHECK_OUTCOME_WORK_FACTOR).sum::<u64>());
    assert!(prices.iter().all(|price| price.priced_text > RULE_CHECK_OUTCOME_TEXT_BYTES + 3 * RULE_CHECK_INPUT_TEXT_BYTES), "each outcome is priced with its identifiers");
    let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
    let args = json!({"rulePackDocument": pack, "jobId": job, "model": probe.model});
    let started = RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get);
    host.set(|script| script.headroom_bytes = Some(64 * 1024 * 1024));
    let error = run_command(&registry, &args).unwrap_err();
    assert!(error.contains("Headroom") || error.contains("headroom") || error.contains("RESOURCE"), "{error}");
    assert_eq!(RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get), started, "the runner did not start");
    host.set(|script| script.headroom_bytes = Some(64 * crate::memory_pool::test_host::GIB));
    let _ = run_command(&registry, &args);
    assert_eq!(RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get), started + 1, "with headroom the runner starts");
    registry.dispose_result(&job).unwrap();
}

/// A rule check whose reply passes its limit though its projection fits: the
/// reply is counted as it would be written and refused by name as soon as the
/// count passes the limit, so no more than one write past it is ever counted.
#[test]
fn a_rule_check_reply_past_its_limit_is_refused_as_it_is_counted() {
    let (registry, gate) = registry();
    let probe = probe();
    let pack = widened(&pack_for(&probe.model, &probe.parsed, &probe.row), 20_000, 1);
    let projection = rule_check_projection(&pack);
    assert!(projection.least_bytes <= MAX_RULE_CHECK_REPLY_BYTES, "the projection fits");
    let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
    let supplied = supplied_for(&probe.parsed, &probe.row, false);
    let started = RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get);
    let error = run_command(&registry, &json!({"rulePackDocument": pack, "jobId": job, "model": probe.model, "suppliedValueBindings": supplied})).unwrap_err();
    assert_eq!(error, format!("RULE_CHECK_RESPONSE_LIMIT: the rule check's reply passes its {MAX_RULE_CHECK_REPLY_BYTES}-byte limit; check fewer load cases or checks"));
    assert_eq!(RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get), started + 1, "the runner ran");
    let counted = RULE_CHECK_REPLY_COUNTED.with(std::cell::Cell::get);
    assert!(counted > MAX_RULE_CHECK_REPLY_BYTES && counted <= MAX_RULE_CHECK_REPLY_BYTES + 64 * 1024, "{counted} bytes counted");
    registry.dispose_result(&job).unwrap();
}

/// A check whose identifiers are long is priced from their written lengths: a
/// 128 KiB check id on 128 bases writes at least 16 MiB of ids, so the run is
/// refused by name from the pack alone, before custody is read or the runner starts.
#[test]
fn a_rule_check_is_priced_from_its_identifiers_lengths() {
    let (registry, gate) = registry();
    let probe = probe();
    let mut pack = widened(&pack_for(&probe.model, &probe.parsed, &probe.row), 1, 128);
    let long = "c".repeat(128 * 1024);
    pack["check_definitions"][0]["check_id"] = json!(long);
    let projection = rule_check_projection(&pack);
    assert_eq!(projection.outcomes, 128);
    assert_eq!(projection.least_bytes, 128 * (RULE_CHECK_OUTCOME_LEAST_BYTES + 128 * 1024));
    assert!(128 * RULE_CHECK_OUTCOME_LEAST_BYTES <= MAX_RULE_CHECK_REPLY_BYTES, "without the id's length the run would be admitted");
    let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
    let started = RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get);
    let error = run_command(&registry, &json!({"rulePackDocument": pack, "jobId": job, "model": probe.model})).unwrap_err();
    assert_eq!(error, format!(
        "RULE_CHECK_RESPONSE_LIMIT: the rule check would return 128 outcomes, at least {} bytes, past its {MAX_RULE_CHECK_REPLY_BYTES}-byte reply limit; check fewer load cases or checks",
        projection.least_bytes
    ));
    assert_eq!(RULE_CHECK_RUNS_STARTED.with(std::cell::Cell::get), started, "the runner did not start");
    registry.dispose_result(&job).unwrap();
}

/// Checks sharing a category are priced from the category's declarations once,
/// never by forming each check's basis list: 1,000 checks on 1,000 shared cases
/// count one basis list and are refused by name with their million outcomes.
#[test]
fn checks_sharing_a_category_are_priced_without_forming_their_bases() {
    let probe = probe();
    let pack = widened(&pack_for(&probe.model, &probe.parsed, &probe.row), 1000, 1000);
    RULE_CHECK_CATEGORIES_PRICED.with(|count| count.set(0));
    let projection = rule_check_projection(&pack);
    assert_eq!(RULE_CHECK_CATEGORIES_PRICED.with(std::cell::Cell::get), 1, "one category's list is counted once");
    assert_eq!(projection.outcomes, 1_000_000);
    assert!(projection.least_bytes > MAX_RULE_CHECK_REPLY_BYTES);
}

/// A solver input whose selector names an absent entity by a 128 KiB entity_ref,
/// on one solved basis: the missing-row finding echoes the whole identifier into
/// the outcome, and the outcome stays within its price, which counts the input's
/// whole definition, selector strings included.
#[test]
fn a_solver_inputs_selector_strings_are_priced_where_a_finding_echoes_them() {
    let (registry, gate) = registry();
    let probe = probe();
    let mut pack = widened(&probe.pack, 1, 1);
    let absent = format!("pipe:absent-{}", "e".repeat(128 * 1024));
    pack["required_inputs"][0]["solver_result_ref"]["entity_ref"] = json!(absent);
    let pack = restamp(pack);
    let job = solve_holding(&registry, &gate, &probe.text, ResultCustody::Native);
    let answer = run_command(&registry, &json!({"rulePackDocument": pack, "jobId": job, "model": probe.model})).unwrap();
    let outcomes = answer["checks"].as_array().unwrap();
    assert_eq!(outcomes.len(), 1);
    assert!(serde_json::to_string(&outcomes[0]).unwrap().contains(&absent), "a finding echoes the absent entity");
    outcomes_within_their_price(&pack, &answer, "a 128 KiB absent entity");
    registry.dispose_result(&job).unwrap();
}
