//! K6 tests E: the staged sequence is SA's entry, bit for bit in `Debug`.
//!
//! In each mode, the staged sequence with the formation source equals
//! `SparseAssemblyEvidence::solve_assembled_with_formation_check(…, true)`,
//! and without it equals `solve_assembled` (K6 brief, Required tests E). The
//! models are RF-LARGE's 10- and 100-member cases and the DEC-053 nine, in
//! both modes (the set fixed at A1 from the measured debug time; ROOT's
//! ruling N17).

use open_pipe_stress_solver_performance_harness::k6::models::{model, sealed_model_ids};
use open_pipe_stress_solver_performance_harness::k6::staged::{
    entry, setup, staged_solve, NoObserver,
};
use open_pipe_stress_solver_performance_harness::k6::Mode;

fn check(id: &str, mode: Mode) {
    let model = model(id).expect("model");
    let frames = model.frames().expect("frames");
    let setup = setup(&model, &frames, &mut NoObserver).expect("setup");
    for with_formation in [true, false] {
        let staged = staged_solve(
            &setup,
            model.node_count(),
            &frames,
            mode,
            with_formation,
            &mut NoObserver,
        );
        let entry = entry(&setup, mode, with_formation, &mut NoObserver);
        assert_eq!(
            format!("{:?}", staged.solution),
            format!("{entry:?}"),
            "{id} {mode:?} formation={with_formation}: the staged sequence differs from SA's entry"
        );
    }
}

fn small_ids() -> Vec<String> {
    sealed_model_ids()
        .into_iter()
        .filter(|id| id.contains("-n00010-") || id.starts_with("DEC053:"))
        .collect()
}

#[test]
fn staged_equals_sa_entries_small_models_both_modes() {
    for id in small_ids() {
        check(&id, Mode::Sparse);
        check(&id, Mode::Dense);
    }
}

#[test]
fn staged_equals_sa_entries_100_member_models_sparse() {
    for id in sealed_model_ids()
        .into_iter()
        .filter(|id| id.contains("-n00100-"))
    {
        check(&id, Mode::Sparse);
    }
}

#[test]
fn staged_equals_sa_entries_100_member_models_dense() {
    for id in sealed_model_ids()
        .into_iter()
        .filter(|id| id.contains("-n00100-"))
    {
        check(&id, Mode::Dense);
    }
}
