//! RV89 G7 (scratch only; never in maintained code): is any U6 `is_retained` branch, or the
//! new `preview_physics_retained_contract` static, reached from the D1 path? Mounted as a
//! child of retained_memory in RV89's copy of the integrated tree `ba1faa1c`, whose
//! result_export carries RV89's copy-only counters (`semantic_contract::RV89_*`,
//! `retained_precision::RV89_VALIDATE_CALLS`).
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::PreviewSolverMode;
use open_pipe_stress_result_export::retained_precision as rp;
use open_pipe_stress_result_export::semantic_contract as sc;
use serde_json::{json, Value};
use std::sync::atomic::Ordering::SeqCst;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
fn milestone() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()
}
fn with(f: impl FnOnce(&mut Value)) -> Value {
    let mut r = milestone();
    f(&mut r);
    r
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Counts { validate: usize, calls: usize, retained: usize, statik: usize, forbid: usize, transport: usize }
fn counts() -> Counts {
    Counts {
        validate: rp::RV89_VALIDATE_CALLS.load(SeqCst),
        calls: sc::RV89_IS_RETAINED_CALLS.load(SeqCst),
        retained: sc::RV89_IS_RETAINED_TRUE.load(SeqCst),
        statik: sc::RV89_RETAINED_STATIC_INIT.load(SeqCst),
        forbid: sc::RV89_FORBID_ERR.load(SeqCst),
        transport: rp::RV89_TRANSPORT_CALLS.load(SeqCst),
    }
}
fn delta(a: Counts, b: Counts) -> Counts {
    Counts { validate: b.validate - a.validate, calls: b.calls - a.calls, retained: b.retained - a.retained,
             statik: b.statik - a.statik, forbid: b.forbid - a.forbid, transport: b.transport - a.transport }
}
fn every_string(v: &mut Value, f: &dyn Fn(&str, &str) -> Option<String>, key: &str) {
    match v {
        Value::Object(m) => for (k, x) in m.iter_mut() { every_string(x, f, k) },
        Value::Array(a) => for x in a.iter_mut() { every_string(x, f, key) },
        Value::String(s) => if let Some(n) = f(key, s) { *s = n },
        _ => {}
    }
}

/// Hostile but D1-admissible text: every free text the milestone carries set to the
/// successor's identity, the receipt member name or the W1 method token, and ids renamed to
/// them (references kept consistent). Each variant is checked inside D1 and run through the
/// Direct entry; the counters must show the reader ran (`validate`, `is_retained` calls) and
/// no `is_retained` true, no static initialization, no downgrade refusal.
fn hostile() -> Vec<(&'static str, Value)> {
    let id = rp::CONTRACT_ID;
    let method = rp::METHOD;
    vec![
        ("project id = successor identity", with(|r| r["model"]["project"]["id"] = json!(id))),
        ("every provenance = method token", with(|r| every_string(r, &|k, _| (k == "provenance").then(|| method.to_string()), ""))),
        ("every provenance = successor identity", with(|r| every_string(r, &|k, _| (k == "provenance").then(|| id.to_string()), ""))),
        ("case id = successor identity", with(|r| r["model"]["load_cases"][0]["id"] = json!(id))),
        ("case label = retained_precision", with(|r| r["model"]["load_cases"][0]["label"] = json!("retained_precision"))),
        ("N1 renamed recovery_method", with(|r| every_string(r, &|_, s| (s == "N1").then(|| "recovery_method".to_string()), ""))),
        ("N1 renamed method token", with(|r| every_string(r, &|_, s| (s == "N1").then(|| method.to_string()), ""))),
        ("material renamed successor identity", with(|r| every_string(r, &|_, s| (s == "mat:N").then(|| id.to_string()), ""))),
        ("segment renamed retained_precision", with(|r| every_string(r, &|_, s| (s == "M1").then(|| "retained_precision".to_string()), ""))),
        ("load ids = producer", with(|r| every_string(r, &|k, s| (k == "id" && s.starts_with("load:")).then(|| format!("producer{s}")), ""))),
        ("model.producer member", with(|r| r["model"]["producer"] = json!({"semantic_contract_id": id}))),
        ("request.retained_precision member", with(|r| r["retained_precision"] = json!({"body": {}}))),
        ("model.retained_precision member", with(|r| r["model"]["retained_precision"] = json!({"body": {}}))),
    ]
}

#[test]
fn rv89g7_hostile_d1_text_never_selects_the_retained_branches() {
    let mut inside = 0;
    for (label, raw) in hostile() {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let report = match admit(&capture, &request, Entry::Direct) { Ok((_, r)) => r, Err(r) => r };
            let domain = report.law().domain;
            let before = counts();
            let out = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            let d = delta(before, counts());
            let published = out.successor().is_some();
            println!("RV89_G7_HOSTILE {label} {mode:?} domain={domain:?} successor={published} {d:?}");
            assert_eq!((d.retained, d.statik, d.forbid, d.transport), (0, 0, 0, 0), "{label} {mode:?}");
            if domain.is_none() {
                inside += 1;
                assert!(d.validate >= 1 && d.calls >= 4, "{label} {mode:?}: the precommit reader ran");
                assert!(published, "{label} {mode:?}: in D1, published");
            }
        }
    }
    println!("RV89_G7_HOSTILE_SUMMARY inside_d1_runs={inside}");
}

/// Run last (name order): the totals over every probe run in this process, then a positive
/// control showing the counters fire when an unprojected successor is dispatched.
#[test]
fn rv89zz_counters_and_positive_control() {
    let total = counts();
    println!("RV89_G7_TOTAL_BEFORE_CONTROL {total:?}");
    assert_eq!((total.retained, total.statik, total.forbid, total.transport), (0, 0, 0, 0));
    let out = crate::run_linear_static_preview_value_with_retained_direct(milestone(), PreviewSolverMode::SparseInteractive).unwrap();
    let successor = out.successor().unwrap().clone();
    let before = counts();
    let dispatched = sc::for_source(&successor);
    let d = delta(before, counts());
    println!("RV89_G7_CONTROL for_source(unprojected successor) ok={} {d:?}", dispatched.is_ok());
    assert!(d.retained >= 1 && d.statik == 1 && d.validate >= 1, "the instrumentation fires off the D1 path");
}
