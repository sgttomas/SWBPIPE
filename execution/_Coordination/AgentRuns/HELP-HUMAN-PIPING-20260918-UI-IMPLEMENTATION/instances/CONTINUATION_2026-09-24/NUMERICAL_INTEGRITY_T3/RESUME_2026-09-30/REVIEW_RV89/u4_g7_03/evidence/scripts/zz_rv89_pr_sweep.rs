//! RV89 (scratch only): public-surface sweep, base vs candidate. Every input × both
//! modes × five routes; each published byte string by sha256 and length, and every
//! public admission-report field by Debug, field by field.
use open_pipe_stress_product_physics::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;

fn h(bytes: &[u8]) -> String {
    format!("{:x}:{}", Sha256::digest(bytes), bytes.len())
}
fn env_bytes(e: &MechanicsEnvelope) -> String {
    h(&serde_json::to_vec(e).unwrap())
}
fn report(out: &mut String, tag: &str, a: Option<&RetainedAdmissionReport>) {
    match a {
        None => writeln!(out, "{tag}\tadmission\tNone").unwrap(),
        Some(a) => {
            writeln!(out, "{tag}\tcaller\t{:?}", a.caller).unwrap();
            writeln!(out, "{tag}\traw\t{:?}", a.raw).unwrap();
            writeln!(out, "{tag}\ttyped\t{:?}", a.typed).unwrap();
            writeln!(out, "{tag}\tcaptured_digest\t{:?}", a.captured_digest).unwrap();
            writeln!(out, "{tag}\tcaptured_encoded_length\t{:?}", a.captured_encoded_length).unwrap();
            writeln!(out, "{tag}\theadless\t{:?}", a.headless).unwrap();
            writeln!(out, "{tag}\tprofile\t{:?}", a.profile).unwrap();
            writeln!(out, "{tag}\tallowance\t{:?}", a.allowance).unwrap();
            writeln!(out, "{tag}\tcensus_complete\t{:?}", a.census_complete()).unwrap();
            writeln!(out, "{tag}\tunknown_terms\t{:?}", a.required_unknown_terms()).unwrap();
        }
    }
}
fn retained(out: &mut String, tag: &str, r: Result<RetainedPreviewOutput, String>) {
    match r {
        Err(e) => writeln!(out, "{tag}\tERR\t{}", h(e.as_bytes())).unwrap(),
        Ok(o) => {
            writeln!(out, "{tag}\tenvelope\t{}", env_bytes(o.envelope())).unwrap();
            writeln!(out, "{tag}\tsuccessor\t{:?}", o.successor().map(|s| h(&serde_json::to_vec(s).unwrap()))).unwrap();
            report(out, tag, o.admission());
            let (e, a) = o.into_parts();
            writeln!(out, "{tag}\tparts\t{}\t{}", env_bytes(&e), a.is_some()).unwrap();
        }
    }
}

#[test]
#[ignore]
fn rv89_sweep() {
    let dir = std::env::var("RV89_SWEEP_INPUTS").unwrap();
    let dest = std::env::var("RV89_SWEEP_OUT").unwrap();
    let mut names: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
    names.sort();
    let mut out = String::new();
    for name in &names {
        let raw: Value = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/{name}")).unwrap()).unwrap();
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let m = mode.as_str();
            // R1: the ordinary Value entry.
            match run_linear_static_preview_value_with_mode(raw.clone(), mode) {
                Ok(e) => writeln!(out, "{name}\t{m}\tR1\t{}", env_bytes(&e)).unwrap(),
                Err(e) => writeln!(out, "{name}\t{m}\tR1\tERR\t{}", h(e.as_bytes())).unwrap(),
            }
            // R2: the historical typed entry.
            match serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()) {
                Ok(request) => writeln!(out, "{name}\t{m}\tR2\t{}", env_bytes(&run_linear_static_preview_with_mode(request, mode))).unwrap(),
                Err(e) => writeln!(out, "{name}\t{m}\tR2\tDESER\t{}", h(e.to_string().as_bytes())).unwrap(),
            }
            // R3: the retained Direct entry.
            retained(&mut out, &format!("{name}\t{m}\tR3"), run_linear_static_preview_value_with_retained_direct(raw.clone(), mode));
            // R4: the retained Headless entry.
            let invocation = json!({"request": raw.clone(), "solver_mode": m});
            let id = String::from("rv89-request");
            let context = RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &id);
            retained(&mut out, &format!("{name}\t{m}\tR4"), run_linear_static_preview_value_with_retained_headless(raw.clone(), mode, context));
            // R5: the one publication (R-1).
            match run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).map(RetainedPreviewOutput::into_publication) {
                Ok(RetainedPublication::Ordinary(e)) => writeln!(out, "{name}\t{m}\tR5\tOrdinary\t{}", env_bytes(&e)).unwrap(),
                Ok(RetainedPublication::Successor(v)) => writeln!(out, "{name}\t{m}\tR5\tSuccessor\t{}", h(&serde_json::to_vec(&v).unwrap())).unwrap(),
                Err(e) => writeln!(out, "{name}\t{m}\tR5\tERR\t{}", h(e.as_bytes())).unwrap(),
            }
        }
    }
    std::fs::write(&dest, out).unwrap();
    {
        use open_pipe_stress_result_export::source_blocks as sb;
        use std::sync::atomic::Ordering::SeqCst;
        let (c, r) = (sb::RV89_VALIDATE_IN_CALLS.load(SeqCst), sb::RV89_LOOP_ROWS.load(SeqCst));
        println!("RV89_PR_SWEEP_COUNTS validate_in_calls={c} loop_rows={r}");
        assert_eq!((c, r), (0, 0), "validate_in is not reached by any route the sweep drives");
    }
    println!("RV89_SWEEP {} inputs written to {dest}", names.len());
}
