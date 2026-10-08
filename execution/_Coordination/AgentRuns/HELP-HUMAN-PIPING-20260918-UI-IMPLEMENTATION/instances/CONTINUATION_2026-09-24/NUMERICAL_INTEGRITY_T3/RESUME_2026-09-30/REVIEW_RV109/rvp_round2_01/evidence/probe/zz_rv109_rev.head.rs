//! RV109 round-2 probe shim, SP's head (603e238517): the n-case production transaction, stage by
//! stage (c = 1 rows), and the reviewer's own W-C2 re-derivation (prints only).
use crate::retained_product::{AttemptEnd, PreparedCases, ProductCapture};
use open_pipe_stress_frame_kernel::structural::retained_api as k;
use serde_json::{json, Value};
pub(super) fn late_total(o: &ProductCapture) -> Option<usize> { Some(o.late_loads_total) }
pub(super) fn cases_seen(o: &ProductCapture) -> usize { o.cases_seen() }
fn cut(text: String) -> String { text.chars().take(160).collect() }
fn end_name(end: &AttemptEnd) -> String {
    match end {
        AttemptEnd::Preparation => "preparation".into(),
        AttemptEnd::Prepared => "prepared".into(),
        AttemptEnd::Native => "native".into(),
        AttemptEnd::Selected => "selected".into(),
        AttemptEnd::Frozen(_) => "frozen".into(),
        AttemptEnd::Candidate(r) => cut(format!("refused:{:?}", r.error)),
    }
}
fn snapshot(p: &mut PreparedCases, k: usize) -> Value {
    let PreparedCases { capture, attempts, .. } = p;
    let attempt = &attempts[k];
    capture.with_case(attempt.request, |c| {
        let mut costs = Default::default();
        json!(attempt.typed_trace(c, &mut costs).map(|v| format!("{:?}", v.adapter)).map_err(|e| format!("{e:?}")))
    })
}
/// SP's `w1_transaction` after T-5, at c = 1: `prepare_cases(…, 1, &[0])`, `native`, `freeze`,
/// `staged_envelope`, `serialize_cases` (precommit is the caller's).
pub(super) fn stages(o: ProductCapture, e: crate::MechanicsEnvelope, capture: &crate::source_receipt::CapturedInvocation) -> Value {
    let mut line = json!({});
    let mut p = match o.prepare_cases(e, 1, &[0]) {
        Err(f) => {
            line["prep"] = json!(cut(format!("failed:Some({:?})", f.error)));
            line["after_prep"] = json!(f.capture.adapter.counts.get());
            line["snapshot"] = Value::Null;
            return line;
        }
        Ok(p) => p,
    };
    if !p.attempts[0].prepared {
        line["prep"] = json!(cut(format!("failed:{:?}", p.capture.error)));
        line["after_prep"] = json!(p.capture.adapter.counts.get());
        line["snapshot"] = json!(p.attempts[0].trace.adapter.as_ref().map(|s| format!("{s:?}")));
        return line;
    }
    line["prep"] = json!("prepared");
    line["after_prep"] = json!(p.capture.adapter.counts.get());
    p.native();
    line["native"] = json!(match &p.attempts[0].end {
        AttemptEnd::Selected => "ok".to_owned(),
        AttemptEnd::Native => format!("{:?}", p.attempts[0].trace.native_error.as_ref().unwrap()),
        other => format!("unexpected {}", end_name(other)),
    });
    line["after_native"] = json!(p.capture.adapter.counts.get());
    if !matches!(p.attempts[0].end, AttemptEnd::Selected) {
        line["snapshot"] = json!(p.attempts[0].trace.adapter.as_ref().map(|s| format!("{s:?}")));
        return line;
    }
    p.freeze();
    line["after_candidate"] = json!(p.capture.adapter.counts.get());
    line["snapshot"] = snapshot(&mut p, 0);
    if !matches!(p.attempts[0].end, AttemptEnd::Frozen(_)) {
        line["candidate"] = json!(end_name(&p.attempts[0].end));
        return line;
    }
    line["candidate"] = json!("frozen");
    let staged = match p.staged_envelope() {
        Err(fault) => {
            line["staging"] = json!(format!("{fault:?}"));
            return line;
        }
        Ok(staged) => staged,
    };
    line["staging"] = json!("ok");
    line["after_staging"] = json!(p.capture.adapter.counts.get());
    line["staged_summary"] = serde_json::to_value(&staged.summary).unwrap();
    let serialized = crate::retained_wire::serialize_cases(&mut p, &staged, capture);
    line["after_serialize"] = json!(p.capture.adapter.counts.get());
    match serialized {
        Err(f) => { line["serializer"] = json!(format!("{f:?}")); }
        Ok(successor) => { line["serializer"] = json!("ok"); line["successor_value"] = successor; }
    }
    line
}

// ---- The reviewer's W-C2 (DESIGN_v2 §1.4), re-derived on the head's API ----

/// W-C2 from DESIGN_v2 §1.4's words, cases in `order` (any of "a", "b", "c"), ids `case-<x>`:
/// A = the milestone's moments on body 0, B = the tip force and torque on body 1, C = A's loads
/// then B's, each load id suffixed `:c` (load ids are model-unique).
pub(super) fn w_c2_order(order: &[&str]) -> Value {
    let mut raw = super::two_body_a();
    let template = raw["model"]["load_cases"][0].clone();
    let a = template["primitive_loads"].clone();
    let b = super::tip_loads();
    let mut c: Vec<Value> = a.as_array().unwrap().iter().chain(b.as_array().unwrap()).cloned().collect();
    for load in &mut c {
        load["id"] = json!(format!("{}:c", load["id"].as_str().unwrap()));
    }
    let make = |x: &str| {
        let mut case = template.clone();
        case["id"] = json!(format!("case-{x}"));
        case["primitive_loads"] = match x { "a" => a.clone(), "b" => b.clone(), "c" => json!(c.clone()),
            // A second copy of A (or C), its load ids suffixed (two selected cases; RV109 round 2).
            "a2" | "c2" => {
                let base = if x == "a2" { a.as_array().unwrap().clone() } else { c.clone() };
                json!(base.into_iter().map(|mut l| { l["id"] = json!(format!("{}:{x}", l["id"].as_str().unwrap())); l }).collect::<Vec<_>>())
            }
            other => panic!("unknown case {other}") };
        case
    };
    raw["model"]["load_cases"] = json!(order.iter().map(|x| make(x)).collect::<Vec<_>>());
    raw
}
/// The one case of `raw` whose id is `id`, as a one-case request (the case keeps its id).
fn one_case_of(raw: &Value, id: &str) -> Value {
    let mut one = raw.clone();
    let case = raw["model"]["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap().clone();
    one["model"]["load_cases"] = json!([case]);
    one
}
/// A kernel outcome, every field the kernel exposes for comparison: the terminal and its reason,
/// and each physical attempt record in full (`Debug`), and the selected precision.
fn outcome_value(outcome: &k::ExecutionOutcome) -> Value {
    match outcome {
        k::ExecutionOutcome::Selected(owner) => json!({"terminal": "selected", "precision": owner.evidence().selected_precision,
            "attempts": owner.evidence().attempts.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()}),
        k::ExecutionOutcome::Refused { refusal, attempts, .. } => json!({"terminal": format!("refused {refusal:?}"),
            "attempts": attempts.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()}),
        k::ExecutionOutcome::Unresolved { reason, attempts, .. } => json!({"terminal": format!("unresolved {reason:?}"),
            "attempts": attempts.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()}),
    }
}
fn observe(raw: &Value, mode: crate::PreviewSolverMode) -> (crate::source_receipt::CapturedInvocation, ProductCapture, crate::MechanicsEnvelope) {
    let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let mut observer = ProductCapture::prepared_probe();
    let mut ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    ordinary.diagnostics.shrink_to_fit();
    (capture, observer, ordinary)
}
fn attempted_of(observer: &ProductCapture, ordinary: &crate::MechanicsEnvelope, ids: &[&str]) -> Vec<usize> {
    crate::case_triggers(&ordinary.numerical_quality, &observer.ordinary, ids).enumerate()
        .filter(|(_, t)| *t == crate::CaseTrigger::Attempted).map(|(i, _)| i).collect()
}
/// The one-case native outcome of `raw` through the head's c = 1 transaction (custody, T-7, T-8).
fn one_case_outcome(raw: &Value, mode: crate::PreviewSolverMode) -> Value {
    let (_, observer, ordinary) = observe(raw, mode);
    let verdict = ordinary.numerical_quality.cases.first().map(|c| format!("{:?}", c.solve_quality));
    let mut p = match observer.prepare_cases(ordinary, 1, &[0]) {
        Ok(p) => p,
        Err(f) => return json!({"verdict": verdict, "custody": format!("{:?}", f.error)}),
    };
    p.native();
    let run = p.capture.native.as_ref().map(|case| outcome_value(&case.outcome));
    json!({"verdict": verdict, "end": end_name(&p.attempts[0].end), "run": run})
}
/// The reviewer's projection for C1 G7 (the accepted reader's own projection, restated): drop the
/// receipt and the method token, restore the base contract id and profile.
fn project(successor: &Value) -> Value {
    let mut p = successor.clone();
    p.as_object_mut().unwrap().remove("retained_precision");
    p["producer"]["semantic_contract_id"] = json!("openpipestress.result_semantics/0.3.0/preview-physics-1");
    p["formulation_basis"]["profile_id"] = json!("product_preview_mechanics_v1");
    for r in p["results"].as_array_mut().unwrap() {
        r.as_object_mut().unwrap().remove("recovery_method");
    }
    p
}
/// The governing row of `kind` over `rows`: greatest value, then smaller case id, then smaller
/// location (the reviewer's restatement of RS/PY/TS's base readers).
fn governing<'a>(rows: &'a [Value], kind: &str, filter: &dyn Fn(&Value) -> bool) -> Option<&'a Value> {
    let mut best: Option<&Value> = None;
    for row in rows.iter().filter(|r| r["kind"] == kind && r["basis_ref"]["ref_type"] == "load_case" && filter(r)) {
        best = match best {
            None => Some(row),
            Some(b) => {
                let (v, bv) = (row["value"].as_f64().unwrap(), b["value"].as_f64().unwrap());
                let key = (row["basis_ref"]["ref_id"].as_str().unwrap(), row["entity_ref"].as_str().unwrap());
                let bkey = (b["basis_ref"]["ref_id"].as_str().unwrap(), b["entity_ref"].as_str().unwrap());
                if v > bv || (v == bv && key < bkey) { Some(row) } else { Some(b) }
            }
        };
    }
    best
}
fn headline_of(row: &Value) -> Value {
    json!({"value": row["value"], "unit": row["unit"], "location_ref": row["entity_ref"], "result_ref": row["id"]})
}
fn base(projected: &Value) -> String {
    match open_pipe_stress_result_export::semantic_contract::for_source(projected) {
        Ok((_, version)) => format!("PASS {version}"),
        Err(e) => cut(e),
    }
}
fn case_index_of(v: &Value) -> Option<u64> { v["index"].as_u64() }

/// W-C2 and its reorderings, both modes: the transaction stage by stage with the adapter counts at
/// each boundary, then the serialized successor, checked against the reviewer's own expectations
/// (computed from the transaction's attempts, not from the serializer), the readers, the
/// headline rule and its alternatives, and N-16 against each case's one-case run. Prints
/// `RV109_WC2 {json}` lines; writes the W-C2 successors and their projections when
/// `RV109_WC2_DIR` is set.
#[test]
#[ignore]
fn zz_rv109_wc2_rederive() {
    let dir = std::env::var("RV109_WC2_DIR").ok();
    for order in [&["a", "b", "c"][..], &["c", "b", "a"], &["b", "a", "c"], &["b", "c", "a"], &["a", "c"], &["c", "a"], &["a", "b"],
        &["a", "a2"], &["a", "c", "a2"], &["a2", "b", "a"], &["c", "a", "a2"]] {
        // RV109_ORDERS (optional): only these orders, e.g. "cba,ca" (the mutant re-runs on the I3 merge).
        if let Ok(only) = std::env::var("RV109_ORDERS") {
            if !only.split(',').any(|o| o == order.join("")) {
                continue;
            }
        }
        for mode in super::MODES {
            let raw = w_c2_order(order);
            let ids: Vec<String> = order.iter().map(|x| format!("case-{x}")).collect();
            let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
            let label = format!("{} {}", order.join(""), mode.as_str());
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let (capture, observer, ordinary) = observe(&raw, mode);
            let attempted = attempted_of(&observer, &ordinary, &ids);
            let mut line = json!({"label": label, "input_sha": super::sha(&serde_json::to_vec(&raw).unwrap()), "plain_sha": super::sha(&plain),
                "verdicts": ordinary.numerical_quality.cases.iter().map(|c| json!([c.basis_ref.ref_id, format!("{:?}", c.solve_quality)])).collect::<Vec<_>>(),
                "attempted": attempted, "after_run": observer.adapter.counts.get()});
            let ordinary_summary = serde_json::to_value(&ordinary.summary).unwrap();
            let ordinary_value = serde_json::to_value(&ordinary).unwrap();
            let mut p = match observer.prepare_cases(ordinary, ids.len(), &attempted) {
                Ok(p) => p,
                Err(f) => { line["custody"] = json!(format!("{:?}", f.error)); println!("RV109_WC2 {line}"); continue; }
            };
            line["after_prep"] = json!(p.capture.adapter.counts.get());
            line["attempts_after_prep"] = json!(p.attempts.iter().map(|a| json!([a.request, a.attempt, a.prepared])).collect::<Vec<_>>());
            p.native();
            let after_native = p.capture.adapter.counts.get();
            line["after_native"] = json!(after_native);
            line["ends_after_native"] = json!(p.attempts.iter().map(|a| end_name(&a.end)).collect::<Vec<_>>());
            // Each Run, read from its own slot: its outcome, run id, and owner ordinal in the invocation.
            let runs: Vec<Value> = (0..p.attempts.len()).map(|k| {
                let request = p.attempts[k].request;
                let case = p.capture.with_case(request, |c| c.native.as_ref().map(|case| (case.run, outcome_value(&case.outcome))));
                let owner = case.as_ref().and_then(|(run, _)| p.capture.native_invocation.as_ref().and_then(|inv| inv.runs().get(*run)).map(|r| format!("{:?}", r.owner)));
                json!({"request": request, "run": case.as_ref().map(|(r, _)| *r), "owner": owner, "outcome": case.map(|(_, o)| o)})
            }).collect();
            line["runs"] = json!(runs);
            let native_snapshots: Vec<Value> = p.attempts.iter().map(|a| json!(a.trace.adapter.as_ref().map(|s| s.counts))).collect();
            line["snapshots_after_native"] = json!(native_snapshots);
            p.freeze();
            let after_freeze = p.capture.adapter.counts.get();
            line["after_freeze"] = json!(after_freeze);
            line["ends_after_freeze"] = json!(p.attempts.iter().map(|a| end_name(&a.end)).collect::<Vec<_>>());
            let snaps: Vec<Option<[u64; 10]>> = p.attempts.iter().map(|a| a.trace.adapter.as_ref().map(|s| s.counts)).collect();
            line["snapshots_after_freeze"] = json!(snaps);
            line["selected_bits"] = json!(p.selected_attempts());
            if !p.attempts.iter().any(|a| matches!(a.end, AttemptEnd::Frozen(_))) {
                line["outcome"] = json!("no case selected");
                println!("RV109_WC2 {line}");
                continue;
            }
            let staged = p.staged_envelope().expect("staging");
            line["after_staging"] = json!(p.capture.adapter.counts.get());
            let staged_value = serde_json::to_value(&staged).unwrap();
            let successor = crate::retained_wire::serialize_cases(&mut p, &staged, &capture);
            line["after_serialize"] = json!(p.capture.adapter.counts.get());
            let successor = match successor {
                Ok(s) => s,
                Err(e) => { line["serializer"] = json!(format!("{e:?}")); println!("RV109_WC2 {line}"); continue; }
            };
            line["ordinary_untouched"] = json!(serde_json::to_vec(&p.ordinary).unwrap() == plain);
            line["successor_sha"] = json!(super::sha(&serde_json::to_vec(&successor).unwrap()));
            line["receipt_sha"] = successor["retained_precision"]["receipt_sha256"].clone();
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            line["reader"] = json!(match open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {
                Ok(v) => format!("PASS eligible={}", v.numerical_eligible),
                Err(e) => format!("{} {}", e.gate, e.code),
            });
            // ---- the reviewer's expectations, from the attempts (not from the serializer) ----
            let body = &successor["retained_precision"]["body"];
            let mut checks: Vec<(String, bool)> = Vec::new();
            let mut check = |name: &str, ok: bool| checks.push((name.to_owned(), ok));
            let status_of = |i: usize| -> &'static str {
                match p.attempts.iter().find(|a| a.request == i) {
                    None => "not_required",
                    Some(a) if matches!(a.end, AttemptEnd::Frozen(_)) => "selected",
                    Some(_) => "unavailable",
                }
            };
            let expect_status: Vec<&str> = (0..ids.len()).map(status_of).collect();
            let got_status: Vec<&str> = body["cases"].as_array().unwrap().iter().map(|c| c["status"].as_str().unwrap()).collect();
            check("statuses", got_status == expect_status);
            check("case ids in request order", body["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"]["ref_id"].as_str().unwrap()).collect::<Vec<_>>() == ids);
            check("ordinary_attempts one per case, request order", body["ordinary_attempts"].as_array().unwrap().iter().enumerate()
                .all(|(i, o)| o["case_index"] == json!(i) && o["case_id"] == json!(ids[i])) && body["ordinary_attempts"].as_array().unwrap().len() == ids.len());
            // Batch ordinals: the prepared attempts in request order.
            let batch: Vec<usize> = p.attempts.iter().filter(|a| a.prepared).map(|a| a.request).collect();
            let run_owner_ordinals: Vec<String> = runs.iter().filter_map(|r| r["owner"].as_str().map(str::to_owned)).collect();
            check("kernel owners are batch ordinals", run_owner_ordinals == (0..batch.len()).map(|o| format!("Case({o})")).collect::<Vec<_>>());
            check("calls: one", body["calls"].as_array().unwrap().len() == 1);
            check("calls[0].owner_refs = batch request indices", body["calls"][0]["owner_refs"] == json!(batch.iter().map(|r| json!({"kind":"case","index":r})).collect::<Vec<_>>()));
            check("calls[0].source_refs = 0..n", body["calls"][0]["source_refs"] == json!((0..batch.len()).collect::<Vec<_>>()));
            // Each case's Run: its owner names its own request index.
            for (i, c) in body["cases"].as_array().unwrap().iter().enumerate() {
                if !c["run"].is_null() {
                    check(&format!("cases[{i}].run.origin.owner_ref = {i}"), case_index_of(&c["run"]["origin"]["owner_ref"]) == Some(i as u64));
                }
            }
            // execution_order: every Run in run-id order, named by request index.
            let mut by_run: Vec<(u64, usize)> = body["cases"].as_array().unwrap().iter().enumerate()
                .filter_map(|(i, c)| c["run"]["id"].as_u64().map(|r| (r, i))).collect();
            by_run.sort();
            check("execution_order = Runs in id order by request", body["work"]["execution_order"] == json!(by_run.iter().map(|(_, i)| json!({"kind":"case","index":i})).collect::<Vec<_>>()));
            // sources: registration (= batch) order, owners by request, preparation names the attempt.
            let sources = body["sources"].as_array().unwrap();
            let submitted_with_run: Vec<usize> = batch.clone();
            check("sources: one per submitted case", sources.len() == submitted_with_run.len());
            for (j, s) in sources.iter().enumerate() {
                let request = submitted_with_run[j];
                let attempt = p.attempts.iter().find(|a| a.request == request).unwrap().attempt;
                check(&format!("sources[{j}] index/owner/attempt_ref"), s["index"] == json!(j) && s["owner"]["case_index"] == json!(request)
                    && s["owner"]["case_id"] == json!(ids[request]) && s["preparation"]["attempt_ref"] == json!(attempt));
            }
            // product attempts: start order; owner = request; source/run refs.
            let pa = body["product_attempts"].as_array().unwrap();
            check("product_attempts: one per case in A", pa.len() == p.attempts.len());
            for (k, a) in p.attempts.iter().enumerate() {
                let j = batch.iter().position(|r| *r == a.request);
                check(&format!("product_attempts[{k}] id/owner/ordinary/source"), pa[k]["id"] == json!(k) && pa[k]["owner_ref"] == json!({"kind":"case","index":a.request})
                    && pa[k]["ordinary_attempt_ref"] == json!(a.request) && pa[k]["source_ref"] == json!(j));
                check(&format!("cases[{}].product_attempt_ref = {k}", a.request), body["cases"][a.request]["product_attempt_ref"] == json!(k));
                // T-11's record point: the receipt carries the snapshot the trace took at the attempt's terminal stage.
                check(&format!("product_attempts[{k}].adapter.counts = the trace's snapshot"), pa[k]["adapter"]["counts"] == json!(snaps[k]));
                if matches!(a.end, AttemptEnd::Native) {
                    check(&format!("product_attempts[{k}] (native end): snapshot = counts after the call"), snaps[k] == Some(after_native));
                }
            }
            let not_required: Vec<usize> = (0..ids.len()).filter(|i| expect_status[*i] == "not_required").collect();
            for i in &not_required {
                check(&format!("cases[{i}] not_required: 4 members, null attempt ref"), body["cases"][*i].as_object().unwrap().len() == 4
                    && body["cases"][*i]["product_attempt_ref"].is_null());
            }
            // charged = the call's after = the last Run's after.
            let last_after = by_run.last().map(|(_, i)| body["cases"][*i]["run"]["invocation_after"].clone());
            check("charged = calls[0].invocation_after = last Run's after", body["work"]["charged"] == body["calls"][0]["invocation_after"]
                && Some(body["work"]["charged"].clone()) == last_after);
            check("material basis: one, every case", body["material_bases"].as_array().unwrap().len() == 1
                && body["material_bases"][0]["case_indices"] == json!((0..ids.len()).collect::<Vec<_>>()));
            line["groups"] = body["groups"].clone();
            line["builds"] = json!(body["builds"].as_array().map(|b| b.len()));
            line["builds_detail"] = body["builds"].clone();
            line["run_groups"] = json!(body["cases"].as_array().unwrap().iter().map(|c| c["run"]["origin"].clone()).collect::<Vec<_>>());
            line["charged"] = body["work"]["charged"].clone();
            line["calls"] = body["calls"].clone();
            line["execution_order"] = body["work"]["execution_order"].clone();
            line["pa_counts"] = json!(pa.iter().map(|a| a["adapter"]["counts"].clone()).collect::<Vec<_>>());
            // The last frozen attempt's snapshot against the counts after T-9 (its last proof stage ends T-9's adapter work).
            line["last_frozen_snapshot_eq_after_freeze"] = json!(p.attempts.iter().rposition(|a| matches!(a.end, AttemptEnd::Frozen(_))).map(|k| snaps[k] == Some(after_freeze)));
            line["cases_brief"] = json!(body["cases"].as_array().unwrap().iter().map(|c| json!({"status": c["status"], "reason": c["reason"],
                "terminal": c["run"]["kernel_terminal"], "source_ref": c["source_ref"], "par": c["product_attempt_ref"]})).collect::<Vec<_>>());
            // Staging order: selected diagnostics in request order, then unavailable ones; one each.
            let diag_ids: Vec<String> = successor["diagnostics"].as_array().unwrap().iter().map(|d| d["id"].as_str().unwrap().to_owned())
                .filter(|id| id.starts_with("diagnostic:retained-precision:")).collect();
            let expect_diags: Vec<String> = (0..ids.len()).filter(|i| expect_status[*i] == "selected").map(|i| format!("diagnostic:retained-precision:{}:selected", ids[i]))
                .chain((0..ids.len()).filter(|i| expect_status[*i] == "unavailable").map(|i| format!("diagnostic:retained-precision:{}:unavailable", ids[i]))).collect();
            check("staging: diagnostics selected-then-unavailable, request order", diag_ids == expect_diags);
            let n = successor["diagnostics"].as_array().unwrap().len();
            check("staging: retained diagnostics are the last ones", successor["diagnostics"].as_array().unwrap()[n - diag_ids.len()..].iter()
                .all(|d| d["id"].as_str().unwrap().starts_with("diagnostic:retained-precision:")));
            check("method token on selected cases' rows only", successor["results"].as_array().unwrap().iter().all(|r| r.get("recovery_method").is_some()
                == ids.iter().position(|id| r["basis_ref"]["ref_id"] == *id).is_some_and(|i| expect_status[i] == "selected")));
            // Rows outside the selected cases are the ordinary rows, byte for byte; selected rows differ only in value and the token.
            let ord_rows = ordinary_value["results"].as_array().unwrap();
            let suc_rows = successor["results"].as_array().unwrap();
            check("row count unchanged", ord_rows.len() == suc_rows.len());
            check("unselected rows untouched", ord_rows.iter().zip(suc_rows).all(|(o, s)| {
                let selected = ids.iter().position(|id| o["basis_ref"]["ref_id"] == *id).is_some_and(|i| expect_status[i] == "selected");
                if selected {
                    let mut s2 = s.clone();
                    s2.as_object_mut().unwrap().remove("recovery_method");
                    s2["value"] = o["value"].clone();
                    s2 == *o
                } else { s == o }
            }));
            // ---- the headline rule, against the base readers' G7 ----
            let rows = staged_value["results"].as_array().unwrap();
            let mut heads = json!({});
            for (kind, field) in [("displacement_magnitude", "max_displacement"), ("pipe_elastic_normal_stress_maximum_v2", "max_open_formula_stress")] {
                let best = governing(rows, kind, &|_: &Value| true).map(headline_of);
                let ordinary_case = ord_rows.iter().find(|r| r["id"] == ordinary_summary[field]["result_ref"]).map(|r| r["basis_ref"]["ref_id"].clone());
                let staged_case = rows.iter().find(|r| r["id"] == successor["summary"][field]["result_ref"]).map(|r| r["basis_ref"]["ref_id"].clone());
                check(&format!("{field}: the governing staged row"), best.as_ref() == Some(&successor["summary"][field]));
                // Alternatives: (1) keep the ordinary headline; (2) the first frozen case's alias (its own rows' maximum); (3) the selected rows' maximum.
                let first_frozen = p.attempts.iter().find(|a| matches!(a.end, AttemptEnd::Frozen(_))).map(|a| ids[a.request]);
                let alias = first_frozen.and_then(|id| governing(rows, kind, &|r: &Value| r["basis_ref"]["ref_id"] == id)).map(headline_of);
                let selected_only = governing(rows, kind, &|r: &Value| ids.iter().position(|id| r["basis_ref"]["ref_id"] == *id).is_some_and(|i| expect_status[i] == "selected")).map(headline_of);
                let projected = project(&successor);
                let alt = |headline: Option<Value>| -> String {
                    let mut q = projected.clone();
                    q["summary"][field] = headline.unwrap_or(Value::Null);
                    base(&q)
                };
                heads[field] = json!({"ordinary_case": ordinary_case, "staged_case": staged_case, "ordinary": ordinary_summary[field], "staged": successor["summary"][field],
                    "g7_staged": base(&projected), "g7_keep_ordinary": alt(Some(ordinary_summary[field].clone())), "g7_first_alias": alt(alias),
                    "g7_selected_only": alt(selected_only)});
            }
            line["headlines"] = heads;
            line["g7_projected"] = json!(base(&project(&successor)));
            line["checks_failed"] = json!(checks.iter().filter(|(_, ok)| !ok).map(|(n, _)| n.clone()).collect::<Vec<_>>());
            line["checks_total"] = json!(checks.len());
            // ---- N-16: each case's one-case run ----
            let mut n16 = json!({});
            for (i, id) in ids.iter().enumerate() {
                let one = one_case_outcome(&one_case_of(&raw, id), mode);
                let batch_run = runs.iter().find(|r| r["request"] == json!(i)).map(|r| r["outcome"].clone());
                n16[*id] = json!({"one_case": one, "batch_equal": batch_run.as_ref().map(|b| *b == one["run"])});
            }
            line["n16"] = n16;
            if let Some(dir) = &dir {
                let stem = format!("{dir}/wc2_{}_{}", order.join(""), mode.as_str());
                std::fs::write(format!("{stem}.successor.json"), serde_json::to_vec(&successor).unwrap()).unwrap();
                std::fs::write(format!("{stem}.projected.json"), serde_json::to_vec(&project(&successor)).unwrap()).unwrap();
                std::fs::write(format!("{stem}.ordinary.json"), &plain).unwrap();
            }
            println!("RV109_WC2 {line}");
        }
    }
}

/// N-16's one-case rows (DESIGN_v2 §1.4; PLAN_v2 §5's oracle list): A, B and C alone, as one-case
/// requests with their W-C2 ids, through the head's c = 1 transaction and through `retained_w1`.
#[test]
#[ignore]
fn zz_rv109_one_case_rows() {
    let raw = w_c2_order(&["a", "b", "c"]);
    for id in ["case-a", "case-b", "case-c"] {
        for mode in super::MODES {
            let one = one_case_of(&raw, id);
            let outcome = one_case_outcome(&one, mode);
            let (capture, observer, ordinary) = observe(&one, mode);
            let (_, retained) = crate::retained_w1(observer, ordinary, &capture);
            println!("RV109_ONE {}", json!({"case": id, "mode": mode.as_str(), "outcome": outcome, "w1": super::cause_of(&retained)}));
        }
    }
}

/// The I3 front-run's document pins, reproduced: W-C2 through the actual Direct entry (counted) and
/// through `retained_w1`, both modes; the D-U6-5 document (`{"id": "w_c2_<mode>", "source", "invocation"}`,
/// pretty-printed as the committed carrier fixtures are) and its sha256, the receipt sha256 and the
/// published bytes' sha256. Prints `RV109_DOC {json}`.
#[test]
#[ignore]
fn zz_rv109_wc2_documents() {
    let raw = w_c2_order(&["a", "b", "c"]);
    for mode in super::MODES {
        let name = mode.as_str();
        let raw_d = raw.clone();
        let (direct, counts) = super::hooks::counted(move || crate::run_linear_static_preview_value_with_retained_direct(raw_d, mode).unwrap());
        let cause = direct.retained().map_or("None".to_owned(), super::cause_of);
        let published = match direct.into_publication() {
            crate::RetainedPublication::Successor(value) => value,
            crate::RetainedPublication::Ordinary(envelope) => serde_json::to_value(&envelope).unwrap(),
        };
        let document = serde_json::to_string_pretty(&json!({"id": format!("w_c2_{name}"), "source": published,
            "invocation": {"request": raw, "solver_mode": name}})).unwrap();
        let (capture, observer, ordinary) = observe(&raw, mode);
        let (_, retained) = crate::retained_w1(observer, ordinary, &capture);
        let driver = retained.as_ref().ok().map(|s| super::sha(&serde_json::to_vec(s.value()).unwrap()));
        println!("RV109_DOC {}", json!({"mode": name, "cause": cause, "runs": counts.runs, "complete_gates": counts.complete_gates,
            "document_sha": super::sha(document.as_bytes()), "receipt_sha": published["retained_precision"]["receipt_sha256"],
            "published_sha": super::sha(&serde_json::to_vec(&published).unwrap()), "driver_cause": super::cause_of(&retained), "driver_successor_sha": driver,
            "input_sha": super::sha(&serde_json::to_vec(&raw).unwrap())}));
    }
}
