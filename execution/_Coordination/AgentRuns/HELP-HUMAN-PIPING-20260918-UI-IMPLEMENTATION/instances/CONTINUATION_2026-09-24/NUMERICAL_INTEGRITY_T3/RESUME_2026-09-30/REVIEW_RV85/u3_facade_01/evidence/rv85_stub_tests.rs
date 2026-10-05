//! RV85 disposable stub tests (archive only; decision 7). One sequential test, because
//! the stub's switches are process-global so that they reach the reserved-stack thread.
use super::*;
use retained_memory::rv85_stub as stub;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::atomic::Ordering::SeqCst;

fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn bytes(r: &Result<MechanicsEnvelope, String>) -> String {
    match r { Ok(e) => format!("ok:{}", sha(&serde_json::to_vec(e).unwrap())), Err(s) => format!("err:{s}") }
}
fn cause(o: &RetainedPreviewOutput) -> String {
    match o.retained() {
        None => "none".into(),
        Some(Ok(_)) => "SUCCESSOR".into(),
        Some(Err(f)) => format!("{f:?}"),
    }
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
    let mut gate_checks: Vec<(String, bool)> = Vec::new();
    let m: Value = serde_json::from_str(MILESTONE).unwrap();
    let reset = || {
        stub::ADMIT.store(false, SeqCst); stub::ADMIT_HEADLESS.store(false, SeqCst);
        stub::LATE_REFUSE.store(false, SeqCst); stub::COMPLETE_REFUSE.store(false, SeqCst);
        stub::STACK.store(64 << 20, SeqCst);
    };
    // A. The actual Direct entry under a permit publishes U1's pinned successor bytes.
    reset(); stub::ADMIT.store(true, SeqCst);
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(PINNED) {
        let plain = bytes(&run_linear_static_preview_value_with_mode(m.clone(), mode));
        let (l0, c0) = (stub::LATE_CALLS.load(SeqCst), stub::COMPLETE_CALLS.load(SeqCst));
        let out = direct(&m, mode).unwrap();
        let (l1, c1) = (stub::LATE_CALLS.load(SeqCst), stub::COMPLETE_CALLS.load(SeqCst));
        let env = bytes(&Ok(out.envelope().clone()));
        let succ = match out.retained() { Some(Ok(s)) => s.value().clone(), other => panic!("A {name}: {other:?}") };
        let text = serde_json::to_string_pretty(&json!({"id":format!("u1_milestone_{name}"),"source":succ,
            "invocation":{"request":m,"solver_mode":mode.as_str()}})).unwrap();
        let ok = sha(text.as_bytes()) == file_sha && succ["retained_precision"]["receipt_sha256"] == receipt_sha && env == plain;
        log.push(format!("A {name} pinned_successor={} envelope_equals_plain={} admission_is_none={} late_calls={} complete_calls={} file_sha={} receipt={}",
            sha(text.as_bytes()) == file_sha, env == plain, out.admission().is_none(), l1 - l0, c1 - c0, sha(text.as_bytes()), succ["retained_precision"]["receipt_sha256"]));
        assert!(ok, "A {name}");
        if let Ok(dir) = std::env::var("RV85_STUB_DIR") {
            std::fs::write(format!("{dir}/rv85_stub_successor_{name}.json"), &text).unwrap();
        }
    }
    // B. Every sweep input through the permitted Direct (and, for F-5, Headless) entry
    // keeps exactly the value route's ordinary bytes; record each W1 cause.
    reset(); stub::ADMIT.store(true, SeqCst); stub::ADMIT_HEADLESS.store(true, SeqCst);
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let list = std::fs::read_to_string(std::env::var("RV85_SWEEP_LIST").unwrap()).unwrap();
    let mut inputs: Vec<(String, Value)> = Vec::new();
    for line in list.lines().filter(|l| !l.trim().is_empty()) {
        let (kind, rel) = line.split_once(' ').unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(p.join(rel)).unwrap()).unwrap();
        inputs.push(if kind == "MOD" { (format!("{{model}}:{rel}"), json!({"model": v})) } else { (rel.to_string(), v) });
    }
    let mut mismatches = 0; let mut panics = 0;
    let mut causes = std::collections::BTreeMap::<String, usize>::new();
    for (name, raw) in &inputs {
        for mode in MODES {
            let plain = bytes(&run_linear_static_preview_value_with_mode(raw.clone(), mode));
            for route in ["direct", "headless"] {
                let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
                let rid = format!("rv85-{name}");
                let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let r = if route == "direct" { direct(raw, mode) } else {
                        run_linear_static_preview_value_with_retained_headless(raw.clone(), mode,
                            RetainedHeadlessContext::from_borrowed_roots(raw, &invocation, &rid)) };
                    match r { Ok(o) => (bytes(&Ok(o.envelope().clone())), cause(&o)), Err(e) => (format!("err:{e}"), "err".into()) }
                }));
                match got {
                    Ok((b, c)) => {
                        if b != plain { mismatches += 1; log.push(format!("B MISMATCH {name} {} {route}", mode.as_str())); }
                        let key = format!("{route}:{}", c.split(['(', ' ', '{']).next().unwrap());
                        *causes.entry(key).or_default() += 1;
                        log.push(format!("B {name}\t{}\t{route}\t{c}", mode.as_str()));
                    }
                    Err(_) => { panics += 1; log.push(format!("B PANIC {name} {} {route}", mode.as_str())); }
                }
            }
        }
    }
    log.push(format!("B inputs={} mismatches={mismatches} panics={panics} causes={causes:?}", inputs.len()));
    // C. G-B refused: LateGate, ordinary bytes; is G-C still consulted after G-B refused?
    reset(); stub::ADMIT.store(true, SeqCst); stub::LATE_REFUSE.store(true, SeqCst);
    for mode in MODES {
        let plain = bytes(&run_linear_static_preview_value_with_mode(m.clone(), mode));
        let c0 = stub::COMPLETE_CALLS.load(SeqCst);
        let out = direct(&m, mode).unwrap();
        let eq = bytes(&Ok(out.envelope().clone())) == plain;
        gate_checks.push((format!("C {}", mode.as_str()), eq && cause(&out).starts_with("LateGate")));
        log.push(format!("C {} cause={} equals_plain={} complete_calls_after_late_refusal={}", mode.as_str(), cause(&out),
            eq, stub::COMPLETE_CALLS.load(SeqCst) - c0));
    }
    // D. G-C refused.
    reset(); stub::ADMIT.store(true, SeqCst); stub::COMPLETE_REFUSE.store(true, SeqCst);
    for mode in MODES {
        let plain = bytes(&run_linear_static_preview_value_with_mode(m.clone(), mode));
        let out = direct(&m, mode).unwrap();
        let eq = bytes(&Ok(out.envelope().clone())) == plain;
        gate_checks.push((format!("D {}", mode.as_str()), eq && cause(&out).starts_with("CompleteGate")));
        log.push(format!("D {} cause={} equals_plain={}", mode.as_str(), cause(&out), eq));
    }
    // E. Spawn failure: StackReservation, ordinary route on the caller's thread.
    reset(); stub::ADMIT.store(true, SeqCst); stub::STACK.store(1usize << 62, SeqCst);
    for mode in MODES {
        let plain = bytes(&run_linear_static_preview_value_with_mode(m.clone(), mode));
        let out = direct(&m, mode).unwrap();
        let eq = bytes(&Ok(out.envelope().clone())) == plain;
        gate_checks.push((format!("E {}", mode.as_str()), eq && cause(&out) == "StackReservation"));
        log.push(format!("E {} cause={} equals_plain={} admission_is_none={}", mode.as_str(), cause(&out), eq, out.admission().is_none()));
    }
    // F. ROOT's flag: a thread-local test hook set on the caller does not reach the
    // reserved-stack thread; an envelope-bytes-only assertion would pass vacuously,
    // and the unconsumed flag fires later on the caller's own thread.
    reset(); stub::ADMIT.store(true, SeqCst);
    for (label, arm) in [("withdraw_native", 0), ("corrupt_precommit", 1), ("rebind_precommit", 2)] {
        let mode = PreviewSolverMode::SparseInteractive;
        let plain = bytes(&run_linear_static_preview_value_with_mode(m.clone(), mode));
        match arm { 0 => retained_tests_hooks::withdraw_next_native_source(), 1 => retained_tests_hooks::corrupt_next_precommit(),
            _ => retained_tests_hooks::rebind_next_precommit_invocation() }
        let out = direct(&m, mode).unwrap();
        let through_entry = cause(&out);
        let env_equal = bytes(&Ok(out.envelope().clone())) == plain;
        // The same thread now drives retained_w1 directly (as the committed tests do).
        let (request, capture) = source_receipt::CapturedInvocation::parse(m.clone(), mode).unwrap();
        let mut observer = retained_product::ProductCapture::prepared_probe();
        let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        let (_, later) = retained_w1(observer, ordinary, &capture);
        let later = match later { Ok(_) => "SUCCESSOR".to_string(), Err(f) => format!("{f:?}") };
        log.push(format!("F {label}: through_permitted_entry={through_entry} envelope_equals_plain={env_equal} next_direct_retained_w1_on_caller={later}"));
    }
    reset();
    let text = log.join("\n") + "\n";
    if let Ok(path) = std::env::var("RV85_STUB_OUT") { std::fs::write(path, &text).unwrap(); }
    assert_eq!((mismatches, panics), (0, 0), "{text}");
    assert!(gate_checks.iter().all(|(_, ok)| *ok), "{gate_checks:?}");
}
