//! RV109 probe shim, SP checkpoint R3′ (56c5579f07): the n-case production path at c = 1, and
//! an SP-only multi-case read of T-2, T-6 and T-7 (prints only).
use crate::retained_product::{PreparedCase, ProductCapture};
use serde_json::{json, Value};
pub(super) fn late_total(o: &ProductCapture) -> Option<usize> { Some(o.late_loads_total) }
pub(super) fn cases_seen(o: &ProductCapture) -> usize { o.cases_seen() }
pub(super) fn prepare(o: ProductCapture, e: crate::MechanicsEnvelope) -> (String, [u64; 10], Option<PreparedCase>) {
    match o.prepare_cases(e, 1, &[0]) {
        Err(f) => (format!("failed:Some({:?})", f.error), f.capture.adapter.counts.get(), None),
        Ok(p) => match p.into_single() {
            Ok(Ok(pc)) => ("prepared".into(), pc.capture().adapter.counts.get(), Some(pc)),
            Ok(Err(f)) => (format!("failed:{:?}", f.capture.error), f.capture.adapter.counts.get(), None),
            Err(_) => ("several attempts".into(), [0; 10], None),
        },
    }
}

/// W-C2 built from DESIGN_v2 §1.4's words: A = the milestone's moments on body 0, B = the tip
/// force and torque on body 1, C = A's loads then B's (load ids suffixed, ids are model-unique).
fn w_c2_cases(order: &[&str]) -> Value {
    let mut raw = super::two_body_a();
    let template = raw["model"]["load_cases"][0].clone();
    let a = template["primitive_loads"].clone();
    let b = super::tip_loads();
    let mut c: Vec<Value> = a.as_array().unwrap().iter().chain(b.as_array().unwrap()).cloned().collect();
    for load in &mut c {
        load["id"] = json!(format!("{}:c", load["id"].as_str().unwrap()));
    }
    let make = |id: &str| {
        let mut case = template.clone();
        case["id"] = json!(id);
        case["primitive_loads"] = match id { "A" => a.clone(), "B" => b.clone(), _ => json!(c.clone()) };
        case
    };
    raw["model"]["load_cases"] = json!(order.iter().map(|id| make(id)).collect::<Vec<_>>());
    raw
}

#[test]
#[ignore]
fn zz_rv109_sp_multicase_read() {
    for order in [&["A", "B", "C"][..], &["C", "B", "A"], &["B", "A"], &["A", "C"], &["C", "A", "B"]] {
        for mode in super::MODES {
            let raw = w_c2_cases(order);
            let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let mut observer = ProductCapture::prepared_probe();
            let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let n = order.len();
            let slots: Vec<Value> = (0..observer.cases_seen()).map(|i| observer.with_case(i, |c| json!([c.case_id.clone(),
                c.source.as_ref().map(|s| s.loads().len()), c.error.as_ref().map(|e| format!("{e:?}")), c.prepared_late_calls, c.observation_calls]))).collect();
            let ids: Vec<&str> = order.to_vec();
            let triggers: Vec<String> = crate::case_triggers(&ordinary.numerical_quality, &observer.ordinary, &ids).map(|t| format!("{t:?}")).collect();
            let attempted: Vec<usize> = triggers.iter().enumerate().filter(|(_, t)| *t == "Attempted").map(|(i, _)| i).collect();
            let counts_run = observer.adapter.counts.get();
            let mut line = json!({"order": order, "mode": mode.as_str(), "run_eq_plain": serde_json::to_vec(&ordinary).unwrap() == plain,
                "mechanics": ordinary.status.mechanics, "cases_seen": observer.cases_seen(), "slots": slots, "triggers": triggers,
                "attempted": attempted, "counts_after_run": counts_run});
            match observer.prepare_cases(ordinary, n, &attempted) {
                Err(f) => { line["custody"] = json!(format!("{:?}", f.error)); }
                Ok(mut p) => {
                    line["custody"] = json!("ok");
                    line["attempts"] = json!(p.attempts.iter().map(|a| json!([a.request, a.attempt, a.prepared, a.trace.source_ready, a.trace.adapter.is_some()])).collect::<Vec<_>>());
                    line["slot_errors_after"] = json!((0..n).map(|i| p.capture.with_case(i, |c| c.error.as_ref().map(|e| format!("{e:?}")))).collect::<Vec<_>>());
                    line["ordinary_untouched"] = json!(serde_json::to_vec(&p.ordinary).unwrap() == plain);
                    line["counts_after_prep"] = json!(p.capture.adapter.counts.get());
                }
            }
            println!("RV109_MULTI {line}");
        }
    }
}

/// R3′ read: the interim one-case continuation (`PreparedCases::into_single`) on a multi-case
/// request with |A| = 1, at the API (custody, T-7, then `into_single`, native and the candidate)
/// and through `retained_w1` on the private driver. With `caps::LOAD_CASES` = 1, `retained_w1`
/// returns `Domain`; a reviewer-only build with `LOAD_CASES` = 3 shows the path after I2. Prints only.
#[test]
#[ignore]
fn zz_rv109_interim_single_attempt() {
    for order in [&["A", "B"][..], &["C", "B"], &["B", "A"], &["B", "C"]] {
        for mode in super::MODES {
            let raw = w_c2_cases(order);
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let ids: Vec<&str> = order.to_vec();
            let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let mut observer = ProductCapture::prepared_probe();
            let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            let attempted: Vec<usize> = crate::case_triggers(&ordinary.numerical_quality, &observer.ordinary, &ids)
                .enumerate().filter(|(_, t)| *t == crate::CaseTrigger::Attempted).map(|(i, _)| i).collect();
            let mut line = json!({"order": order, "mode": mode.as_str(), "attempted": attempted, "load_cases_cap": crate::retained_memory::caps::LOAD_CASES});
            match observer.prepare_cases(ordinary, order.len(), &attempted) {
                Err(f) => { line["custody"] = json!(format!("{:?}", f.error)); }
                Ok(p) => match p.into_single() {
                    Ok(Ok(mut pc)) => {
                        line["single_case_in_fields"] = json!(pc.capture().case_id.clone());
                        line["single_source_loads"] = json!(pc.capture().source.as_ref().map(|s| s.loads().len()));
                        line["single_trace_source_ready"] = json!(pc.trace.source_ready);
                        line["native"] = json!(pc.solve_native().map_err(|e| format!("{e:?}")).err().unwrap_or_else(|| "ok".into()));
                        line["candidate"] = json!(match pc.freeze_candidate() {
                            Ok(_) => "frozen".to_owned(),
                            Err(r) => format!("{:?}", r.error).chars().take(160).collect(),
                        });
                    }
                    Ok(Err(f)) => { line["single"] = json!(format!("preparation failed {:?}", f.capture.error)); }
                    Err(_) => { line["single"] = json!("several attempts"); }
                },
            }
            let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let mut observer = ProductCapture::prepared_probe();
            let mut ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            ordinary.diagnostics.shrink_to_fit();
            let (envelope, retained) = crate::retained_w1(observer, ordinary, &capture);
            let bytes = serde_json::to_vec(&envelope).unwrap();
            line["w1_cause"] = json!(super::cause_of(&retained));
            line["w1_owner_eq_plain"] = json!(bytes == plain);
            line["w1_owner_notices"] = json!(super::notices(&bytes));
            if let Ok(successor) = &retained {
                let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
                line["successor_cases"] = json!(successor.value()["retained_precision"]["body"]["cases"].as_array().map(Vec::len));
                line["reader"] = json!(match open_pipe_stress_result_export::retained_precision::validate(successor.value(), Some(&invocation)) {
                    Ok(_) => "PASS".to_owned(),
                    Err(e) => format!("{} {}", e.gate, e.code),
                });
            }
            println!("RV109_INTERIM {line}");
        }
    }
}
