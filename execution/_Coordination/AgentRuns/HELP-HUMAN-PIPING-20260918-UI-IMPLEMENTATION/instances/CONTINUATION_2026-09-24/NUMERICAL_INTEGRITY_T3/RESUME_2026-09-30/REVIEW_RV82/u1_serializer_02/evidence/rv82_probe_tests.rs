//! RV82 review probe for U1 grant 2 (disposable copy only; never committed).
use super::retained_product as rp;
use super::retained_receipt::TraceFault as F;
use super::retained_wire as wire;
use super::*;
use serde_json::{json, Value};

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

fn out_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("RV82_OUT").expect("RV82_OUT"))
}
fn raw() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
fn candidate(mode: PreviewSolverMode) -> (rp::PrivatePreparedCandidate, source_receipt::CapturedInvocation) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw(), mode).unwrap();
    let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
    prepared.solve_native().unwrap();
    match prepared.project_candidate() { Ok(c) => (c, capture), Err(e) => panic!("{:?}", e.error) }
}
fn refused(mode: PreviewSolverMode, fault: F) -> (rp::PreparedCandidateRefusal, source_receipt::CapturedInvocation) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw(), mode).unwrap();
    let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
    prepared.test_capture_mut().trace_fault = Some(fault);
    prepared.solve_native().unwrap();
    match prepared.project_candidate() { Err(r) => (r, capture), Ok(_) => panic!("refusal expected") }
}
fn res(r: Result<Value, wire::ReceiptFailure>) -> String {
    match r { Ok(_) => "EMITTED".into(), Err(f) => format!("refused {}:{}", f.check.wire(), f.field_path) }
}

/// S2 on b54caba7ab: every same-mode foreign invocation, through both serializers.
#[test]
fn rv82g2_foreign_invocation() {
    let mode = PreviewSolverMode::SparseInteractive;
    let (cand, own) = candidate(mode);
    let (refusal, _) = refused(mode, F::ValuesCompletion);
    let mut variants: Vec<(&str, Value, PreviewSolverMode)> = Vec::new();
    variants.push(("identical_reparse", raw(), mode));
    // Same Value, keys re-inserted in another order: canonical JSON identical.
    let mut reordered = serde_json::Map::new();
    let r = raw();
    for (k, v) in r.as_object().unwrap().iter().rev() { reordered.insert(k.clone(), v.clone()); }
    variants.push(("key_order_only", Value::Object(reordered), mode));
    let mut v = raw(); v["model"]["load_cases"][0]["label"] = json!("RV82 foreign label"); variants.push(("label", v, mode));
    let mut v = raw(); v["model"]["project"]["id"] = json!("invented:rv82:foreign"); variants.push(("project_id", v, mode));
    let mut v = raw(); v["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(0.0049); variants.push(("load_magnitude", v, mode));
    let mut v = raw(); v["model"]["load_cases"][0]["primitive_loads"][0]["provenance"] = json!("rv82"); variants.push(("load_provenance", v, mode));
    let mut v = raw(); v["model"]["nodes"][0]["rv82_extra"] = json!(true); variants.push(("unknown_member", v, mode));
    variants.push(("other_mode", raw(), PreviewSolverMode::DenseScrutiny));
    let mut lines = Vec::new();
    for (name, other_raw, m) in variants {
        let Ok((_, other)) = source_receipt::CapturedInvocation::parse(other_raw, m) else { lines.push(format!("{name}: parse refused")); continue };
        lines.push(format!("{name}: selected={} unavailable={}", res(wire::serialize_selected(&cand, &other)),
            res(wire::serialize_unavailable(wire::Refused::Candidate(&refusal), &other))));
    }
    lines.push(format!("own: selected={}", res(wire::serialize_selected(&cand, &own))));
    std::fs::write(out_dir().join("g2_foreign_invocation.txt"), lines.join("\n") + "\n").unwrap();
}

/// The translation corpus values (the serializer's own output for each variant),
/// for RV82's independent derivation and schema validation in Python.
#[test]
fn rv82g2_corpus_dump() {
    let entries = wire::corpus::build();
    let v: Vec<Value> = entries.iter().map(|x| json!({"def":x.def,"label":x.label,"value":x.value,
        "failures":x.failures.iter().map(|f| json!({"check":f.check.wire(),"path":f.field_path})).collect::<Vec<_>>()})).collect();
    std::fs::write(out_dir().join("g2_corpus.json"), serde_json::to_string_pretty(&v).unwrap()).unwrap();
}

/// Unavailable representations (not publications): RV82's own invocation of the
/// actual refusal owners, for the D4d spot check, schema and reader runs.
#[test]
fn rv82g2_unavailable_dump() {
    let dir = out_dir();
    let write = |name: &str, s: &Value, mode: PreviewSolverMode| {
        std::fs::write(dir.join(format!("{name}.json")), serde_json::to_string_pretty(&json!({"id":name,"source":s,
            "invocation":{"request":raw(),"solver_mode":mode.as_str()}})).unwrap()).unwrap();
    };
    let mut lines = Vec::new();
    for mode in MODES {
        // Preparation refusal (facts[0] diameter 0, as experiment 02).
        let (request, capture) = source_receipt::CapturedInvocation::parse(raw(), mode).unwrap();
        let mut observer = rp::ProductCapture::prepared_probe();
        let ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        observer.facts[0].diameter = 0.0;
        let failed = match observer.prepare_case(ordinary) { Err(f) => f, Ok(_) => panic!("refusal") };
        match wire::serialize_unavailable(wire::Refused::Preparation(&failed), &capture) {
            Ok(s) => write(&format!("g2u_preparation_{}", mode.as_str()), &s, mode),
            Err(f) => lines.push(format!("preparation {}: {f:?}", mode.as_str())),
        }
        // D38: the prepared source withdrawn before the kernel call.
        let (request, capture) = source_receipt::CapturedInvocation::parse(raw(), mode).unwrap();
        let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
        prepared.test_capture_mut().source = None;
        let _ = prepared.solve_native();
        match wire::serialize_unavailable(wire::Refused::Native(&prepared), &capture) {
            Ok(s) => write(&format!("g2u_d38_{}", mode.as_str()), &s, mode),
            Err(f) => lines.push(format!("d38 {}: {f:?}", mode.as_str())),
        }
        // Candidate refusals after a selected Run.
        for (fault, label) in [(F::Maxima, "abandoned"), (F::ValuesCompletion, "values"), (F::AfterPrelude, "after_prelude"),
            (F::AfterHelper, "after_helper"), (F::AfterEvaluation, "after_evaluation")] {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw(), mode).unwrap();
            let mut prepared = match rp::PreparedCase::prepare_observed(request, mode, &capture) { Ok(p) => p, Err(e) => {
                lines.push(format!("{label} {}: preparation refused {:?}", mode.as_str(), e.capture.error));
                if let Ok(s) = wire::serialize_unavailable(wire::Refused::Preparation(&e), &capture) { write(&format!("g2u_{label}_{}", mode.as_str()), &s, mode); }
                continue } };
            prepared.test_capture_mut().trace_fault = Some(fault);
            if prepared.solve_native().is_err() { lines.push(format!("{label}: native refused")); continue }
            match prepared.project_candidate() {
                Ok(_) => lines.push(format!("{label} {}: no refusal", mode.as_str())),
                Err(r) => match wire::serialize_unavailable(wire::Refused::Candidate(&r), &capture) {
                    Ok(s) => write(&format!("g2u_{label}_{}", mode.as_str()), &s, mode),
                    Err(f) => lines.push(format!("{label} {}: {f:?}", mode.as_str())),
                },
            }
        }
    }
    std::fs::write(dir.join("g2_unavailable_notes.txt"), lines.join("\n") + "\n").unwrap();
}

/// The milestone successor files (pinned bytes) for the reader runs.
#[test]
fn rv82g2_milestone_dump() {
    for mode in MODES {
        let (cand, capture) = candidate(mode);
        let s = wire::serialize_selected(&cand, &capture).unwrap();
        let text = serde_json::to_string_pretty(&json!({"id":format!("u1_milestone_{}", mode.as_str()),"source":s,
            "invocation":{"request":raw(),"solver_mode":mode.as_str()}})).unwrap();
        std::fs::write(out_dir().join(format!("u1_milestone_{}.json", mode.as_str())), text).unwrap();
    }
}

/// The accepted Rust reader (dev-dependency bytes = NUM's) on RV82's files.
#[test]
fn rv82g2_rust_reader() {
    use open_pipe_stress_result_export::retained_precision as reader;
    let dir = std::path::PathBuf::from(std::env::var("RV82_READ_DIR").expect("RV82_READ_DIR"));
    let mut names: Vec<_> = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok().map(|e| e.file_name().into_string().unwrap()))
        .filter(|n| n.starts_with("u1_milestone_") && n.ends_with(".json")).collect();
    names.sort();
    let mut lines = Vec::new();
    for n in names {
        let c: Value = serde_json::from_str(&std::fs::read_to_string(dir.join(&n)).unwrap()).unwrap();
        match reader::validate(&c["source"], Some(&c["invocation"])) {
            Ok(v) => {
                lines.push(format!("RS {n} PASS invocation_bound={} eligible={} classes={}", v.invocation_bound, v.numerical_eligible, v.classifications.len()));
                let text: String = v.classifications.iter().map(|x| {
                    let class = match &x.class {
                        reader::AccuracyClass::RelativeVerified => "relative_verified",
                        reader::AccuracyClass::AbsoluteVerified { .. } => "absolute_verified",
                        reader::AccuracyClass::InputDerived => "input_derived",
                        reader::AccuracyClass::NonQuantity => "non_quantity",
                        reader::AccuracyClass::NotCovered => "not_covered",
                    };
                    format!("{}|{:016x}|{}|{class}\n", x.result_id, x.normalized_bits, x.scale_bits.map_or("null".to_string(), |b| format!("{b:016x}")))
                }).collect();
                std::fs::write(dir.join(format!("{n}.rs_classes.txt")), text).unwrap();
            }
            Err(e) => lines.push(format!("RS {n} FIRST {} {}", e.gate, e.code)),
        }
    }
    std::fs::write(dir.join("rs_results.txt"), lines.join("\n") + "\n").unwrap();
}
