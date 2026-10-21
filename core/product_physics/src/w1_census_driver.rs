//! The shared driver of the W1 coverage census and W1-E's confirmation census
//! (DEL-04-08 Design `w1-coverage-repair.md` §3.2, `w1e-sharper-exact-coverage.md`
//! §6.2): one run of the retained work under the kernel probe's switches, after a
//! fresh observed ordinary run, with its receipt summary, its precommit reader
//! outcomes (native scales, and the gate's final scales substituted) and the probe's
//! records. Included by both census modules; `census-probe` only. Every model is
//! invented. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use crate::*;
use crate::retained_product as rp;
use open_pipe_stress_frame_kernel::structural::retained_api::census_probe as probe;
use open_pipe_stress_result_export::retained_precision as reader;
use serde_json::{json, Value};
use std::io::Write as _;

pub(super) fn field_json(f: &probe::Field) -> Value {
    match f {
        probe::Field::F(x) => if x.is_finite() { json!(x) } else { json!(format!("{x}")) },
        probe::Field::U(x) => json!(x),
        probe::Field::I(x) => json!(x),
        probe::Field::B(x) => json!(x),
        probe::Field::S(x) => json!(x),
        probe::Field::L(v) => Value::Object(v.iter().map(|(k, x)| (k.clone(), field_json(x))).collect()),
    }
}
pub(super) fn record_json(r: &probe::Record) -> Value {
    let mut o = serde_json::Map::new();
    o.insert("kind".into(), json!(r.kind));
    for (k, v) in &r.fields {
        o.insert(k.clone(), field_json(v));
    }
    Value::Object(o)
}

/// The observed ordinary run, as the facade and O4's M3 driver run it.
pub(super) fn observed(raw: &Value, mode: PreviewSolverMode) -> (source_receipt::CapturedInvocation, rp::ProductCapture, MechanicsEnvelope) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).expect("the request parses");
    let route = w1_route(&request.model).unwrap_or(rp::W1Route::Preview);
    let mut observer = rp::ProductCapture::prepared_probe_on(route);
    let mut ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut w1_budget(route), Some(&mut observer));
    ordinary.diagnostics.shrink_to_fit();
    (capture, observer, ordinary)
}

/// A reader outcome as text: "ok" or "<gate> <code>".
pub(super) fn reader_outcome(source: &Value, invocation: &Value) -> String {
    match reader::validate(source, Some(invocation)) {
        Ok(_) => "ok".into(),
        Err(e) => format!("{} {}", e.gate, e.code),
    }
}
pub(super) const GATES: [&str; 13] = ["G0", "G1", "G2", "G3", "G4", "G5", "G5a", "G5b", "G5c", "G6", "G7", "G8", "ok"];
pub(super) fn gate_rank(outcome: &str) -> usize {
    let gate = outcome.split(' ').next().unwrap_or("");
    GATES.iter().position(|g| *g == gate).unwrap_or(0)
}

/// The candidate with every selection's `body_scales` replaced by the gate's final
/// scales from the probe's case records (matched by the case's basis id), and its
/// receipt hash resealed, so the unchanged reader judges the scales at G5b.
pub(super) fn with_final_scales(candidate: &Value, records: &[Value]) -> Option<Value> {
    let mut c = candidate.clone();
    let finals: Vec<&Value> = records.iter().filter(|r| r["kind"] == "gate_case").collect();
    let body = c["retained_precision"]["body"].as_object_mut()?;
    let mut replaced = 0;
    for list in ["cases", "combinations"] {
        let Some(items) = body.get_mut(list).and_then(Value::as_array_mut) else { continue };
        for item in items {
            if item.get("selection").is_none() { continue; }
            let id = item["basis_ref"]["ref_id"].as_str().or_else(|| item["combination_id"].as_str()).or_else(|| item["id"].as_str())?.to_owned();
            let rec = finals.iter().rev().find(|r| r["case_id"] == id.as_str())?;
            let scales = rec["final_scales"].as_object()?;
            let mut out = Vec::new();
            for b in 0..scales.len() {
                let s = &scales[&b.to_string()];
                let hex = |k: &str| json!(format!("{:016x}", s[k].as_u64().unwrap()));
                out.push(json!({"body": b, "translation": hex("translation"), "rotation": hex("rotation"), "force": hex("force"), "moment": hex("moment")}));
            }
            item["selection"]["body_scales"] = json!(out);
            replaced += 1;
        }
    }
    if replaced == 0 {
        return None;
    }
    // Reseal the test copy: the receipt hash binds the body (G1).
    let receipt = crate::retained_wire::domain_hash("retained_precision_receipt_mp_v2", &c["retained_precision"]["body"])?;
    c["retained_precision"]["receipt_sha256"] = json!(receipt);
    Some(c)
}

/// The receipt's per-case outcome summary.
pub(super) fn cases_summary(source: &Value) -> Value {
    let body = &source["retained_precision"]["body"];
    let attempts = body["product_attempts"].as_array().cloned().unwrap_or_default();
    let cases: Vec<Value> = body["cases"].as_array().cloned().unwrap_or_default().iter().enumerate().map(|(i, c)| {
        let attempt = attempts.iter().find(|a| a["owner_ref"]["index"] == i);
        json!({"case": c["basis_ref"]["ref_id"], "status": c["status"], "reason": c.get("reason"),
            "error": attempt.map(|a| a["result"]["error"].clone()), "case_charge": c["run"]["case_charge"],
            "absolute_verified": c["selection"]["absolute_verified"], "body_scales": c["selection"]["body_scales"],
            "attempt_stages": attempt.map(|a| a["stages"].clone()),
            "proof_numeric": attempt.map(|a| a["proof"]["numeric"].clone())})
    }).collect();
    json!({"cases": cases, "combinations": body.get("combinations"), "work": body.get("work")})
}

/// One run of the retained work with `switches`, after a fresh observed ordinary run.
/// Returns its summary, the candidate precommit received (if any) and the ordinary
/// envelope as a value.
pub(super) fn run_once(raw: &Value, mode: PreviewSolverMode, switches: probe::Switches, hook: &dyn Fn()) -> (Value, Option<Value>, Value) {
    let (capture, observer, ordinary) = observed(raw, mode);
    let ordinary_value = serde_json::to_value(&ordinary).unwrap();
    let ordinary_sha = format!("{:x}", <sha2::Sha256 as sha2::Digest>::digest(serde_json::to_vec(&ordinary).unwrap()));
    probe::drain();
    let ((_, retained), _, candidate) = probe::with(switches, || {
        hook();
        retained_tests_hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture))
    });
    let records: Vec<Value> = probe::drain().iter().map(record_json).collect();
    let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
    let (fallback, published) = match &retained {
        Ok(_) => ("published".to_owned(), true),
        Err(f) => (format!("{f:?}"), false),
    };
    let mut out = json!({"switches": format!("{switches:?}"), "fallback": fallback, "published": published, "ordinary_sha256": ordinary_sha});
    if let Some(c) = &candidate {
        let native = reader_outcome(c, &invocation);
        let finals = with_final_scales(c, &records).map(|f| reader_outcome(&f, &invocation));
        out["reader_native"] = json!(native);
        out["reader_final"] = json!(finals);
        out["summary"] = cases_summary(c);
        out["candidate_sha256"] = json!(format!("{:x}", <sha2::Sha256 as sha2::Digest>::digest(serde_json::to_vec(c).unwrap())));
    }
    out["records"] = json!(records);
    (out, candidate, ordinary_value)
}

/// One JSON line to `W1_CENSUS_OUT/<target>.jsonl`. Each target's file is created
/// (truncated) at its first line in the process and kept open for the target's later lines.
pub(super) fn write_line(target: &str, value: &Value) {
    static FILES: std::sync::Mutex<std::collections::BTreeMap<String, std::fs::File>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
    let Some(dir) = std::env::var_os("W1_CENSUS_OUT") else { return };
    let mut files = FILES.lock().unwrap();
    let f = files.entry(target.to_owned())
        .or_insert_with(|| std::fs::File::create(std::path::Path::new(&dir).join(format!("{target}.jsonl"))).unwrap());
    writeln!(f, "{}", serde_json::to_string(value).unwrap()).unwrap();
}
/// `W1_CENSUS_LIMIT=<n>` (the pilot) runs only the first n inputs of a target.
pub(super) fn limit() -> usize {
    std::env::var("W1_CENSUS_LIMIT").ok().and_then(|v| v.parse().ok()).unwrap_or(usize::MAX)
}
