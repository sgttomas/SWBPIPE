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

/// Supplied values under which the demo ratio (the row's value over the supplied
/// limit, against a ratio limit of 1) passes, or, when `failing`, is 2 and fails.
fn supplied_for(row: &Value, failing: bool) -> Value {
    let value = row["value"].as_f64().unwrap();
    let limit = if failing { value / 2.0 } else { value.abs() * 2.0 + 1.0 };
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

/// What a parity case binds: an admitted row with a passing or failing ratio, an
/// admitted zero row whose zero is written negative, or a row the binding gate refuses.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Bound {
    Passing,
    Failing,
    NegativeZero,
    Refused,
}

/// The case's result text, row and supplied values for `bound`.
fn bound_case(text: &str, bound: Bound) -> (String, Value, Value, Value) {
    let mut parsed: Value = serde_json::from_str(text).unwrap();
    let refusal = |envelope: &Value, row: &Value| open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(envelope, row);
    let row = match bound {
        Bound::Passing => chosen_row(&parsed),
        // A row holding zero, whose sign alone is flipped: the result keeps its meaning and standing.
        Bound::NegativeZero => chosen_row_where(&parsed, |row| refusal(&parsed, row).is_none() && row["value"].as_f64() == Some(0.0)),
        Bound::Failing => chosen_row_where(&parsed, |row| refusal(&parsed, row).is_none() && row["value"].as_f64() != Some(0.0)),
        Bound::Refused => chosen_row_where(&parsed, |row| refusal(&parsed, row).is_some()),
    };
    if bound == Bound::Refused {
        assert!(refusal(&parsed, &row).is_some());
    }
    if bound != Bound::NegativeZero {
        let supplied = supplied_for(&row, bound == Bound::Failing);
        return (text.to_string(), parsed, row, supplied);
    }
    let supplied = supplied_for(&row, false);
    let held = parsed["results"].as_array_mut().unwrap().iter_mut().find(|candidate| candidate["id"] == row["id"]).unwrap();
    held["value"] = json!(-0.0);
    let row = held.clone();
    let text = serde_json::to_string(&parsed).unwrap();
    assert!(row_value(&text, &row["id"]).is_sign_negative());
    (text, parsed, row, supplied)
}

/// The value of the row `id` in the result `text`.
fn row_value(text: &str, id: &Value) -> f64 {
    let parsed: Value = serde_json::from_str(text).unwrap();
    parsed["results"].as_array().unwrap().iter().find(|row| &row["id"] == id).unwrap()["value"].as_f64().unwrap()
}

/// `value` with each negative zero written as zero: the one difference the retired route made.
fn zero_sign_blind(value: &Value) -> Value {
    match value {
        Value::Number(number) if number.as_f64() == Some(0.0) && number.is_f64() => json!(0.0),
        Value::Array(items) => Value::Array(items.iter().map(zero_sign_blind).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(key, item)| (key.clone(), zero_sign_blind(item))).collect()),
        other => other.clone(),
    }
}

/// The committed fixtures the parity covers: model, result text and the
/// registered invocation each route sends (none, or the captured one).
fn parity_cases() -> Vec<(String, Value, String, Option<Value>, Bound)> {
    let mut cases = Vec::new();
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    for mode in ["sparse", "dense"] {
        let label = format!("pressure_connected_mechanics_{mode}");
        let text = project_text(&format!("fixtures/results/{label}.json"));
        cases.push((label.clone(), request["model"].clone(), text.clone(), None, Bound::Passing));
        cases.push((format!("{label} failing"), request["model"].clone(), text.clone(), None, Bound::Failing));
        if mode == "sparse" {
            cases.push((format!("{label} negative zero"), request["model"].clone(), text, None, Bound::NegativeZero));
        }
    }
    for name in ["connected", "pressure"] {
        let request = project_file(&format!("fixtures/product_preview/pressure/load_reference/{name}.request.json"));
        for mode in ["sparse_interactive", "dense_scrutiny"] {
            let label = format!("pressure/load_reference/{name}-{mode}");
            cases.push((label.clone(), request["model"].clone(), project_text(&format!("fixtures/product_preview/{label}.raw.json")), None, Bound::Passing));
        }
    }
    // A component body's row, which the binding gate refuses (its reference section's stress), in
    // an eligible preview-physics-2 result: the run binds the row and reports it incomplete.
    let model = project_file("core/product_physics/tests/fixtures/preview_physics_invented_model.json");
    let label = "preview_physics_invented_sparse";
    cases.push((format!("{label} refused row"), model, project_text(&format!("fixtures/results/{label}.json")), None, Bound::Refused));
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
                cases.push((label.clone(), request["model"].clone(), text.clone(), Some(invocation), Bound::Passing));
                // Without its registered invocation: refused by both routes alike.
                cases.push((format!("{label} without invocation"), request["model"].clone(), text, None, Bound::Passing));
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
/// on the exact held bytes), byte for byte in canonical form. The one exception
/// is a held negative zero, which the retired route rewrote as `0` and the
/// handle route's reader refuses (see the case).
#[test]
fn a_rule_check_by_handle_answers_as_the_envelope_route_did() {
    let (registry, gate) = registry();
    let mut outcomes = std::collections::BTreeSet::new();
    let mut files = std::collections::BTreeSet::new();
    let cases = parity_cases();
    for (label, model, text, invocation, bound) in &cases {
        let (label, model, invocation) = (label.clone(), model.clone(), invocation.clone());
        files.insert(label.split(' ').next().unwrap().to_string());
        let (text, parsed, row, supplied) = bound_case(text, *bound);
        let pack = pack_for(&model, &parsed, &row);
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
        if let Some(invocation) = &invocation {
            args["sourceBlockInvocation"] = invocation.clone();
        }
        let by_handle = run_by_handle(&registry, args);
        let canonical = |outcome: &Result<Value, String>| outcome.as_ref().map(|value| serde_json::to_string(value).unwrap()).map_err(Clone::clone);
        if *bound == Bound::NegativeZero {
            // The one difference. The retired route sent the negative zero as `0` and checked the
            // result; the handle route reads the held bytes, and the result reader's scientific
            // ingress refuses negative-zero input (`core/serialization/canonical_json`), so the
            // held result is unsupported, as every native reader of it finds. The same text with
            // the zero's sign alone restored answers as the envelope route did.
            assert!(by_envelope.is_ok(), "{label}: {by_envelope:?}");
            assert_eq!(by_handle.as_ref().unwrap_err(), "RULE_NUMERICAL_SOURCE_UNSUPPORTED: recompute with a supported producer and semantic contract", "{label}");
            let restored = serde_json::to_string(&zero_sign_blind(&serde_json::from_str::<Value>(&text).unwrap())).unwrap();
            assert!(row_value(&restored, &row["id"]).is_sign_positive());
            let control = solve_holding(&registry, &gate, &restored, ResultCustody::Native);
            let mut args = json!({"rulePackDocument": pack, "jobId": control, "model": model, "suppliedValueBindings": supplied});
            if let Some(invocation) = &invocation {
                args["sourceBlockInvocation"] = invocation.clone();
            }
            assert_eq!(canonical(&run_by_handle(&registry, args)), canonical(&by_envelope), "{label}: control");
            registry.dispose_result(&control).unwrap();
        } else {
            assert_eq!(canonical(&by_handle), canonical(&by_envelope), "{label}");
        }
        if *bound == Bound::Refused {
            let answer = serde_json::to_string(by_handle.as_ref().unwrap()).unwrap();
            assert!(answer.contains(open_pipe_stress_result_export::component_body::BODY_REFERENCE_SECTION), "{label}: {answer}");
        }
        outcomes.insert(match &by_handle {
            Ok(value) => {
                let checks = value["checks"].as_array().unwrap();
                format!("{} {:?}", value["aggregate_status"].as_str().unwrap(), checks.iter().map(|check| check["status"].as_str().unwrap_or("")).collect::<std::collections::BTreeSet<_>>())
            }
            Err(error) => error.split(':').next().unwrap().to_string(),
        });
        registry.dispose_result(&job).unwrap();
    }
    assert_eq!((cases.len(), files.len()), (34, 19), "the case and file counts the description states");
    eprintln!("parity outcomes: {outcomes:#?}");
    // The cases reach a checked outcome and the standing refusals, not only one of them.
    assert!(outcomes.iter().any(|outcome| outcome.starts_with("USER_RULE_CHECKED")), "{outcomes:?}");
    assert!(outcomes.iter().any(|outcome| outcome.starts_with("USER_RULE_FAILED")), "{outcomes:?}");
    assert!(outcomes.iter().any(|outcome| outcome.starts_with("RULE_INPUTS_INCOMPLETE")), "{outcomes:?}");
    assert!(outcomes.iter().any(|outcome| outcome.starts_with("RULE_NUMERICAL_NEEDS_RECOMPUTE") || outcome.starts_with("RULE_NUMERICAL_SOURCE_UNSUPPORTED")), "{outcomes:?}");
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
    let supplied = supplied_for(&row, false);
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

/// DEL-07-07 REQ-015: a result disposed after the rule check or the preview
/// acquired it, and before either answered, gives the refusal and nothing read.
#[test]
fn a_result_disposed_while_it_is_read_is_refused_and_nothing_is_returned() {
    let (registry, gate) = registry();
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let row = chosen_row(&parsed);
    let pack = pack_for(&request["model"], &parsed, &row);
    let selector = serde_json::to_value(rule_solver_selection::selector_for_row(&parsed, &row).unwrap()).unwrap();
    let queries = json!([{"input_id": "actual", "selector": selector, "bases": [row["basis_ref"]]}]);
    for command in ["run_rule_checks", "preview_rule_solver_rows"] {
        let job = solve_holding(&registry, &gate, &text, ResultCustody::Native);
        let held = crate::native_verdict::tests::gate::hold(&registry, "rule-check-answer", false);
        let (reader, reading) = (registry.clone(), job.clone());
        let args = match command {
            "run_rule_checks" => json!({"rulePackDocument": pack, "jobId": job, "model": request["model"]}),
            _ => json!({"jobId": job, "queries": queries}),
        };
        let answer = std::thread::spawn(move || match command {
            "run_rule_checks" => run_by_handle(&reader, args),
            _ => preview_by_handle(&reader, args),
        });
        assert!(held.0.wait_entered(1, std::time::Duration::from_secs(60)), "{command}: the read did not reach its answer");
        registry.dispose_result(&reading).unwrap();
        held.0.open();
        let error = answer.join().unwrap().unwrap_err();
        assert_eq!(error, format!("RULE_CHECK_RESULT_NOT_HELD: {job} was released while it was read; nothing it read is returned"), "{command}");
        drop(held);
    }
}

/// A held result holding an integral number outside the safe integer range,
/// which the web view's checked JSON profile refused at registration, is still
/// refused by the rule check and the preview, by name, with nothing evaluated;
/// the largest safe integer is admitted.
#[test]
fn a_held_result_outside_the_checked_json_profile_is_refused() {
    let (registry, gate) = registry();
    let request = project_file("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let row = chosen_row(&parsed);
    let pack = pack_for(&request["model"], &parsed, &row);
    let selector = serde_json::to_value(rule_solver_selection::selector_for_row(&parsed, &row).unwrap()).unwrap();
    let queries = json!([{"input_id": "actual", "selector": selector, "bases": [row["basis_ref"]]}]);
    let other = parsed["results"].as_array().unwrap().iter().position(|candidate| candidate["id"] != row["id"]).unwrap();
    for (written, refused) in [("9007199254740992", true), ("-9007199254740992", true), ("1.8e19", true), ("9007199254740993.0", true), ("9007199254740991", false), ("1.5e300", true)] {
        let mut source = parsed.clone();
        source["results"][other]["value"] = json!("PLACEHOLDER");
        let held_text = serde_json::to_string(&source).unwrap().replace("\"PLACEHOLDER\"", written);
        let job = solve_holding(&registry, &gate, &held_text, ResultCustody::Native);
        let run = run_by_handle(&registry, json!({"rulePackDocument": pack, "jobId": job, "model": request["model"]}));
        let preview = preview_by_handle(&registry, json!({"jobId": job, "queries": queries}));
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

/// The selector preview's request is bounded before the result is read, and its
/// reply returns at most the row the display shows on each basis and is refused
/// by name above its byte limit.
#[test]
fn the_selector_preview_is_bounded_and_refuses_by_name() {
    let (registry, gate) = registry();
    let text = project_text("fixtures/results/pressure_connected_mechanics_sparse.json");
    let parsed: Value = serde_json::from_str(&text).unwrap();
    let row = chosen_row(&parsed);
    let selector = serde_json::to_value(rule_solver_selection::selector_for_row(&parsed, &row).unwrap()).unwrap();
    let job = solve_holding(&registry, &gate, &text, ResultCustody::Native);
    let invalid = |queries: Value| {
        let error = preview_by_handle(&registry, json!({"jobId": job, "queries": queries})).unwrap_err();
        assert!(error.starts_with("RULE_SOLVER_PREVIEW_REQUEST_INVALID: "), "{error}");
        error
    };
    let query = |input: usize, bases: Vec<Value>| json!({"input_id": format!("input-{input}"), "selector": selector, "bases": bases});
    // Too many queries; too many bases; one input twice; one basis twice.
    invalid(Value::Array((0..=rule_solver_selection::MAX_PREVIEW_QUERIES).map(|input| query(input, vec![row["basis_ref"].clone()])).collect()));
    invalid(json!([query(0, (0..=rule_solver_selection::MAX_PREVIEW_BASES_PER_QUERY).map(|case| json!({"ref_type": "load_case", "ref_id": format!("case:{case}")})).collect())]));
    invalid(json!([query(0, vec![row["basis_ref"].clone()]), query(0, vec![row["basis_ref"].clone()])]));
    let error = invalid(json!([query(0, vec![row["basis_ref"].clone(), row["basis_ref"].clone()])]));
    assert!(error.contains("more than once"), "{error}");
    // At the bounds the request is admitted.
    let most = Value::Array((0..rule_solver_selection::MAX_PREVIEW_QUERIES).map(|input| query(input, vec![row["basis_ref"].clone()])).collect());
    let preview = preview_by_handle(&registry, json!({"jobId": job, "queries": most})).unwrap();
    assert_eq!(preview["inputs"].as_array().unwrap().len(), rule_solver_selection::MAX_PREVIEW_QUERIES);

    // A selector naming more than one row on a basis returns none of them, only
    // that it names more than one; here the result holds the row twice.
    let mut doubled = parsed.clone();
    let mut copy = row.clone();
    copy["id"] = json!(format!("{}:copy", row["id"].as_str().unwrap()));
    doubled["results"].as_array_mut().unwrap().push(copy);
    let ambiguous = solve_holding(&registry, &gate, &serde_json::to_string(&doubled).unwrap(), ResultCustody::Native);
    let preview = preview_by_handle(&registry, json!({"jobId": ambiguous, "queries": [query(0, vec![row["basis_ref"].clone()])]})).unwrap();
    let basis = &preview["inputs"][0]["bases"][0];
    assert_eq!((&basis["matched"], &basis["rows"]), (&json!(2), &json!([])));
    registry.dispose_result(&ambiguous).unwrap();

    // Above the reply limit: refused by name, with the size and the limit.
    let one = json!([query(0, vec![row["basis_ref"].clone()])]);
    let error = preview_within(&registry, json!({"jobId": job, "queries": one}), 64).unwrap_err();
    assert!(error.starts_with("RULE_SOLVER_PREVIEW_RESPONSE_LIMIT: the preview needs a reply of ") && error.contains("more than its 64-byte limit"), "{error}");
    registry.dispose_result(&job).unwrap();
}
