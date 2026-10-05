//! RV85 disposable stub tests for grant 1b (archive only; decision 7). One sequential
//! test: the stub's switches are process-global so that they reach the worker thread.
use super::*;
use retained_memory::rv85_stub as stub;
use open_pipe_stress_result_export::semantic_contract as sc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::atomic::Ordering::SeqCst;

fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn cause(o: &RetainedPreviewOutput) -> String {
    match o.retained() { None => "none".into(), Some(Ok(_)) => "SUCCESSOR".into(), Some(Err(f)) => format!("{f:?}") }
}
fn w1_ran(cause: &str) -> bool {
    ["Preparation", "Native", "Candidate", "Serializer", "Precommit"].iter().any(|c| cause.starts_with(c))
}
/// RV85's own expectation of the R-2 notice (ROOT's ruling + I61's accepted fixed text).
fn notice(case: &str) -> Value {
    json!({"id": format!("diagnostic:retained-precision:{case}:unavailable"), "code": "RETAINED_PRECISION_UNAVAILABLE",
        "severity": "info", "message": "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.",
        "source": "core/product_physics", "affected_refs": [case]})
}
fn expected(plain: &Value, case: Option<&str>) -> Value {
    let mut v = plain.clone();
    if let Some(case) = case { v["diagnostics"].as_array_mut().unwrap().push(notice(case)); }
    v
}
fn direct(raw: &Value, mode: PreviewSolverMode) -> Result<RetainedPreviewOutput, String> {
    run_linear_static_preview_value_with_retained_direct(raw.clone(), mode)
}
const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const PINNED: [(&str, &str, &str); 2] = [
    ("sparse_interactive", "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", "efc1a39bbe83840df6bd0761c932b8020285d3b8c005ba8fd0b45ba10d667494"),
    ("dense_scrutiny", "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5", "3e26499f17caff8f5fc0d46406bbe54acf43e8cb16e761784aa5074413b0ac4a"),
];
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

#[test]
fn rv85_stub_permitted_path() {
    let mut log: Vec<String> = Vec::new();
    let mut checks: Vec<(String, bool)> = Vec::new();
    let m: Value = serde_json::from_str(MILESTONE).unwrap();
    let mcase = m["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
    let out_dir = std::env::var("RV85_STUB_DIR").ok();
    let reset = || {
        stub::ADMIT.store(false, SeqCst); stub::ADMIT_HEADLESS.store(false, SeqCst);
        stub::LATE_REFUSE.store(false, SeqCst); stub::COMPLETE_REFUSE.store(false, SeqCst); stub::STACK.store(64 << 20, SeqCst);
    };
    let plain_v = |raw: &Value, mode| run_linear_static_preview_value_with_mode(raw.clone(), mode).map(|e| serde_json::to_value(&e).unwrap());
    // A. Success through the actual Direct entry: R-1 names the successor.
    reset(); stub::ADMIT.store(true, SeqCst);
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(PINNED) {
        let plain = plain_v(&m, mode).unwrap();
        let out = direct(&m, mode).unwrap();
        let env_ok = serde_json::to_value(out.envelope()).unwrap() == plain;
        let acc = out.successor().cloned();
        let publ = match out.into_publication() { RetainedPublication::Successor(v) => Some(v), RetainedPublication::Ordinary(_) => None };
        let succ = publ.clone().unwrap_or(Value::Null);
        let text = serde_json::to_string_pretty(&json!({"id":format!("u1_milestone_{name}"),"source":succ,"invocation":{"request":m,"solver_mode":mode.as_str()}})).unwrap();
        let ok = sha(text.as_bytes()) == file_sha && succ["retained_precision"]["receipt_sha256"] == receipt_sha && env_ok && acc == publ;
        checks.push((format!("A {name}"), ok));
        log.push(format!("A {name} publication=Successor:{} pinned_file={} receipt={} envelope_plain={env_ok} successor_accessor_equals_publication={}",
            publ.is_some(), sha(text.as_bytes()) == file_sha, succ["retained_precision"]["receipt_sha256"], acc == publ));
    }
    // B. Every input, Direct and Headless, everything admitted: exact ordinary bytes where no
    // W1 work ran; plain + exactly RV85's notice where it ran; the base readers' verdicts unchanged.
    reset(); stub::ADMIT.store(true, SeqCst); stub::ADMIT_HEADLESS.store(true, SeqCst);
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let list = std::fs::read_to_string(std::env::var("RV85_SWEEP_LIST").unwrap()).unwrap();
    let mut inputs: Vec<(String, Value)> = Vec::new();
    for line in list.lines().filter(|l| !l.trim().is_empty()) {
        let (kind, rel) = line.split_once(' ').unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(p.join(rel)).unwrap()).unwrap();
        inputs.push(if kind == "MOD" { (format!("{{model}}:{rel}"), json!({"model": v})) } else { (rel.to_string(), v) });
    }
    let (mut mismatches, mut panics, mut reader_diffs, mut noticed_count) = (0, 0, 0, 0);
    let mut causes = std::collections::BTreeMap::<String, usize>::new();
    for (name, raw) in &inputs {
        for mode in MODES {
            let plain = plain_v(raw, mode);
            for route in ["direct", "headless"] {
                let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
                let rid = format!("rv85-{name}");
                let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let r = if route == "direct" { direct(raw, mode) } else {
                        run_linear_static_preview_value_with_retained_headless(raw.clone(), mode,
                            RetainedHeadlessContext::from_borrowed_roots(raw, &invocation, &rid)) };
                    r.map(|o| { let c = cause(&o); let succ = o.successor().is_some();
                        let env = serde_json::to_value(o.envelope()).unwrap();
                        let publ = match o.into_publication() { RetainedPublication::Ordinary(e) => Some(serde_json::to_value(&e).unwrap()), RetainedPublication::Successor(_) => None };
                        (c, succ, env, publ) })
                }));
                match (got, &plain) {
                    (Ok(Ok((c, succ, env, publ))), Ok(plain)) => {
                        let case = if w1_ran(&c) { raw["model"]["load_cases"][0]["id"].as_str() } else { None };
                        let want = expected(plain, case);
                        let carrier_ok = if c == "SUCCESSOR" { succ && publ.is_none() && &env == plain } else { !succ && publ.as_ref() == Some(&env) };
                        if env != want || !carrier_ok { mismatches += 1; log.push(format!("B MISMATCH {name} {} {route} {c}", mode.as_str())); }
                        if case.is_some() {
                            noticed_count += 1;
                            let bases: Vec<Value> = raw["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect();
                            let same = sc::for_source(&env) == sc::for_source(plain) && sc::standing_reason(&env) == sc::standing_reason(plain)
                                && sc::numerical_use_standing(&env, &bases) == sc::numerical_use_standing(plain, &bases)
                                && sc::numerical_use_standing_with_context(&env, &bases, Some(&invocation)) == sc::numerical_use_standing_with_context(plain, &bases, Some(&invocation));
                            if !same { reader_diffs += 1; }
                            log.push(format!("B NOTICED {name}\t{}\t{route}\t{c}\tfor_source_ok={} standing={}\treaders_unchanged={same}", mode.as_str(),
                                sc::for_source(&env).is_ok(), sc::numerical_use_standing(&env, &bases)));
                            if let (Some(dir), "direct") = (&out_dir, route) {
                                let stem = name.replace('/', "_");
                                std::fs::write(format!("{dir}/noticed_{stem}_{}.json", mode.as_str()), serde_json::to_vec(&env).unwrap()).unwrap();
                                std::fs::write(format!("{dir}/plain_{stem}_{}.json", mode.as_str()), serde_json::to_vec(plain).unwrap()).unwrap();
                                std::fs::write(format!("{dir}/request_{stem}.json"), serde_json::to_vec(raw).unwrap()).unwrap();
                            }
                        }
                        *causes.entry(format!("{route}:{}", c.split(['(', ' ', '{']).next().unwrap())).or_default() += 1;
                    }
                    (Ok(Err(a)), Err(b)) if &a == b => { *causes.entry(format!("{route}:err_equal")).or_default() += 1; }
                    (Err(_), _) => { panics += 1; log.push(format!("B PANIC {name} {} {route}", mode.as_str())); }
                    _ => { mismatches += 1; log.push(format!("B OUTCOME MISMATCH {name} {} {route}", mode.as_str())); }
                }
            }
        }
    }
    log.push(format!("B inputs={} mismatches={mismatches} panics={panics} noticed={noticed_count} reader_diffs={reader_diffs} causes={causes:?}", inputs.len()));
    checks.push(("B".into(), mismatches == 0 && panics == 0 && reader_diffs == 0));
    // C/D/E. G-B, G-C and stack refusals: no W1 work ran, so exact ordinary bytes, Ordinary.
    for (label, set, want) in [("C", 0, "LateGate"), ("D", 1, "CompleteGate"), ("E", 2, "StackReservation")] {
        reset(); stub::ADMIT.store(true, SeqCst);
        match set { 0 => stub::LATE_REFUSE.store(true, SeqCst), 1 => stub::COMPLETE_REFUSE.store(true, SeqCst), _ => stub::STACK.store(1usize << 62, SeqCst) }
        for mode in MODES {
            let plain = plain_v(&m, mode).unwrap();
            let c0 = stub::COMPLETE_CALLS.load(SeqCst);
            let out = direct(&m, mode).unwrap();
            let c = cause(&out);
            let gc = stub::COMPLETE_CALLS.load(SeqCst) - c0;
            let publ_ok = matches!(out.into_publication(), RetainedPublication::Ordinary(e) if serde_json::to_value(&e).unwrap() == plain);
            checks.push((format!("{label} {}", mode.as_str()), c.starts_with(want) && publ_ok));
            log.push(format!("{label} {} cause={c} Ordinary_plain={publ_ok} complete_calls={gc}", mode.as_str()));
        }
    }
    // F. RV85's grant-1 hazard demonstration, rerun: a hook armed on the caller now fires on
    // the reserved-stack thread, the publication is plain + notice, and nothing leaks back.
    reset(); stub::ADMIT.store(true, SeqCst);
    for (label, arm, want) in [("withdraw_native", 0, "Native"), ("corrupt_precommit", 1, "Precommit { gate: \"G1\""), ("rebind_precommit", 2, "Precommit { gate: \"G8\""),
        ("serializer_encoding", 3, "Serializer")] {
        let mode = PreviewSolverMode::SparseInteractive;
        let plain = plain_v(&m, mode).unwrap();
        match arm { 0 => retained_tests_hooks::withdraw_next_native_source(), 1 => retained_tests_hooks::corrupt_next_precommit(),
            2 => retained_tests_hooks::rebind_next_precommit_invocation(), _ => retained_tests_hooks::fail_next_serializer(retained_wire::ReceiptCheck::Encoding) }
        let out = direct(&m, mode).unwrap();
        let c = cause(&out);
        let publ_ok = matches!(out.into_publication(), RetainedPublication::Ordinary(e) if serde_json::to_value(&e).unwrap() == expected(&plain, Some(&mcase)));
        let left = retained_tests_hooks::armed() == retained_tests_hooks::Armed::default();
        let (request, capture) = source_receipt::CapturedInvocation::parse(m.clone(), mode).unwrap();
        let mut observer = retained_product::ProductCapture::prepared_probe();
        let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        let later = match retained_w1(observer, ordinary, &capture).1 { Ok(_) => "SUCCESSOR".to_string(), Err(f) => format!("{f:?}") };
        checks.push((format!("F {label}"), c.starts_with(want) && publ_ok && left && later == "SUCCESSOR"));
        log.push(format!("F {label}: through_permitted_entry={c} Ordinary_plain_plus_notice={publ_ok} caller_armed_empty={left} next_direct_retained_w1_on_caller={later}"));
    }
    // G. Residual: a fault that cannot fire (G-B refuses first) is taken from the caller and
    // discarded with the worker; afterwards the caller shows nothing armed, so a post-call
    // "none left armed" assertion cannot tell a fired fault from an unfired one.
    reset(); stub::ADMIT.store(true, SeqCst); stub::LATE_REFUSE.store(true, SeqCst);
    retained_tests_hooks::corrupt_next_precommit();
    let out = direct(&m, PreviewSolverMode::SparseInteractive).unwrap();
    log.push(format!("G unfired fault: cause={} caller_armed_empty_after={}", cause(&out), retained_tests_hooks::armed() == retained_tests_hooks::Armed::default()));
    reset();
    let text = log.join("\n") + "\n";
    if let Ok(path) = std::env::var("RV85_STUB_OUT") { std::fs::write(path, &text).unwrap(); }
    assert!(checks.iter().all(|(_, ok)| *ok), "{:?}\n{text}", checks.iter().filter(|(_, ok)| !ok).collect::<Vec<_>>());
}
