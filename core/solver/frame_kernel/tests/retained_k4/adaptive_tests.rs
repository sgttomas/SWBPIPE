//! K4 tests of `retained/adaptive.rs` (survey stage).
use super::*;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;

fn unlimited() -> (CaseLimit, InvocationMeter) {
    (CaseLimit::new(u64::MAX), InvocationMeter::new(u64::MAX))
}

#[test]
fn survey_models() {
    for m in models::models() {
        let source = match std::panic::catch_unwind(|| m.source()) {
            Ok(s) => s,
            Err(_) => {
                println!("{}: source refused", m.name);
                continue;
            }
        };
        let (limit, mut meter) = unlimited();
        let started = std::time::Instant::now();
        let outcome = solve_case(source.clone(), limit, &mut meter);
        let secs = started.elapsed().as_secs_f64();
        match &outcome {
            CaseOutcome::Selected(solve) => {
                let (worst, at, n) = models::compare(&source, &solve.publish().rows, &m.expect);
                let attempts: Vec<String> = solve
                    .evidence()
                    .attempts
                    .iter()
                    .map(|a| format!("{}:{:?}:c{}", a.precision, a.outcome, a.corrections))
                    .collect();
                println!(
                    "{}: selected {} worst {:.3e} at {} ({} compared) {:.2}s {:?} geom {:?}",
                    m.name,
                    solve.selected_precision(),
                    worst,
                    at,
                    n,
                    secs,
                    attempts,
                    solve.evidence().geometry
                );
            }
            CaseOutcome::Refused { refusal, geometry } => {
                println!(
                    "{}: refused {:?} {:?} {:.2}s",
                    m.name, refusal, geometry, secs
                )
            }
            CaseOutcome::Unresolved {
                reason,
                attempts,
                geometry,
            } => {
                let attempts: Vec<String> = attempts
                    .iter()
                    .map(|a| format!("{}:{:?}", a.precision, a.outcome))
                    .collect();
                println!(
                    "{}: unresolved {:?} {:?} {:?} {:.2}s",
                    m.name, reason, attempts, geometry, secs
                )
            }
        }
    }
}

/// Solves `name` at 128 (residual at 192) with every k_q diagonal scaled by
/// 1 + 2^-e (a perturbed residual basis: refinement then converges to the
/// perturbed system at a rate of about κ·2^-e per correction).
fn perturbed_solve(name: &str, e: i64) -> Result<u8, AttemptStop> {
    // (The unperturbed factor at p is kept: only the residual's K changes.)
    let source = models::model(name).source();
    let prep = CasePrep::new(source.clone()).unwrap();
    let group = prepare_group(&source).unwrap();
    let mut shared = build_shared::<4, 4>(128, 192, &source, &group, StageGuard::unlimited())
        .result
        .unwrap();
    let mut c = WideContext::<4>::new(192).unwrap();
    for (r, col, index) in group.structure.entries() {
        if r == col {
            let v = shared.k_q[index];
            let delta = v.mul_pow2(-e).unwrap();
            shared.k_q[index] = c.add(&v, &delta).unwrap();
        }
    }
    solve_case_at(&shared, &prep, &group, StageGuard::unlimited())
        .result
        .map(|solved| solved.corrections)
}

#[test]
fn refinement_stops_after_three_corrections_and_the_attempt_escalates() {
    // N09-B's residual basis perturbed on the diagonal by 2^-e: e = 32 needs
    // exactly three corrections (accepted); e = 28 would need a fourth, so the
    // attempt stops with the residual gate, an escalating stop; e = 120 needs
    // none and e = 80 one.
    assert_eq!(perturbed_solve("N09-B", 120), Ok(0));
    assert_eq!(perturbed_solve("N09-B", 80), Ok(1));
    assert_eq!(perturbed_solve("N09-B", 48), Ok(2));
    assert_eq!(perturbed_solve("N09-B", 32), Ok(3));
    let stop = perturbed_solve("N09-B", 28).unwrap_err();
    assert!(matches!(stop, AttemptStop::ResidualGate { .. }));
    assert!(stop.escalates());
}
