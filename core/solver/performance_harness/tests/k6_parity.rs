//! K6 tests D: §4.8's parity items that are K6's (K6 brief Scope 7), at up to
//! 100 members and on the DEC-053 nine. No new tolerance: the DEC-053 basis is
//! the harness's existing 1e-9 relative criterion (`H/README.md:54`).
//!
//! - D1, bitwise K: the pattern assembly equals the dense assembly entry for
//!   entry (bits), and every unstored entry is +0.0 (bits 0).
//! - D2, outcome-class parity between the modes, asserted in every case.
//! - D3, the DEC-053 basis: max |u_s − u_d| ≤ 1e-9 · max |u_d|, asserted where
//!   both modes are Passed. Where a mode is Sensitive, the delta is recorded
//!   (printed) and not asserted: a Sensitive publication carries no accuracy
//!   claim at that level (ROOT, "K6: rulings at I15's A1 stop", option (a);
//!   §4.8 item 3 read as applying to Passed publications). The three
//!   Sensitive 100-member deltas are recorded in
//!   `T3/IMPLEMENTATION/K6/_run_records/a1/dec053_breach/`.

use open_pipe_stress_frame_kernel::assemble_global_stiffness;
use open_pipe_stress_frame_kernel::structural::{assemble_sparse_stiffness, SparseAssemblyOptions};
use open_pipe_stress_solver_performance_harness::k6::models::{model, sealed_model_ids};
use open_pipe_stress_solver_performance_harness::k6::parity::{bitwise_k, dec053_delta};
use open_pipe_stress_solver_performance_harness::k6::staged::{
    outcome_class, setup, staged_solve, NoObserver,
};
use open_pipe_stress_solver_performance_harness::k6::Mode;

/// The DEC-026/DEC-053 relative criterion (`H/src/lib.rs:44`).
const DEC053_RELATIVE: f64 = 1.0e-9;

fn debug_ids() -> Vec<String> {
    sealed_model_ids()
        .into_iter()
        .filter(|id| {
            id.contains("-n00010-") || id.contains("-n00100-") || id.starts_with("DEC053:")
        })
        .collect()
}

/// D1 on every model at up to 100 members and the nine.
#[test]
fn bitwise_k_equals_dense_assembly() {
    for id in debug_ids() {
        let m = model(&id).unwrap();
        let frames = m.frames().unwrap();
        let sparse = assemble_sparse_stiffness(
            m.node_count(),
            &frames,
            &[],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        )
        .unwrap();
        let dense = assemble_global_stiffness(m.node_count(), &frames).unwrap();
        let result = bitwise_k(&sparse, &dense);
        assert!(result.equal(), "{id}: {result:?}");
        assert_eq!(
            result.stored_entries,
            sparse.pattern().entry_count(),
            "{id}"
        );
    }
}

/// The bitwise-K check detects a changed bit and a stored-where-zero entry.
#[test]
fn bitwise_k_detects_differences() {
    let m = model("RF-LARGE-CHAIN-n00010-AX").unwrap();
    let frames = m.frames().unwrap();
    let sparse = assemble_sparse_stiffness(
        m.node_count(),
        &frames,
        &[],
        &[],
        &[],
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    let mut dense = assemble_global_stiffness(m.node_count(), &frames).unwrap();
    dense[6][6] = f64::from_bits(dense[6][6].to_bits() ^ 1);
    dense[0][65] = -0.0;
    let result = bitwise_k(&sparse, &dense);
    assert_eq!(
        (result.stored_mismatches, result.unstored_nonzero_bits),
        (1, 1)
    );
}

/// D2 and D3 for one model. Returns whether D3 was asserted (both Passed).
fn parity(id: &str) -> bool {
    let m = model(id).unwrap();
    let frames = m.frames().unwrap();
    let setup = setup(&m, &frames, &mut NoObserver).unwrap();
    let sparse = staged_solve(
        &setup,
        m.node_count(),
        &frames,
        Mode::Sparse,
        true,
        &mut NoObserver,
    );
    let dense = staged_solve(
        &setup,
        m.node_count(),
        &frames,
        Mode::Dense,
        true,
        &mut NoObserver,
    );
    let (sparse_class, dense_class) = (
        outcome_class(&sparse.solution),
        outcome_class(&dense.solution),
    );
    assert_eq!(
        sparse_class, dense_class,
        "{id}: outcome-class divergence between the modes"
    );
    let (Ok(s), Ok(d)) = (&sparse.solution, &dense.solution) else {
        return false;
    };
    let delta = dec053_delta(&s.displacements, &d.displacements);
    if sparse_class == "Passed" && dense_class == "Passed" {
        assert!(
            delta.max_abs_delta <= DEC053_RELATIVE * delta.dense_scale,
            "{id}: DEC-053 basis breached on a Passed result: {delta:?}"
        );
        true
    } else {
        eprintln!(
            "{id}: {sparse_class}/{dense_class}; DEC-053 delta recorded, not asserted: \
             max_abs_delta={:e} dense_scale={:e} relative={:e}",
            delta.max_abs_delta,
            delta.dense_scale,
            delta.relative()
        );
        false
    }
}

/// D2 and D3 on the 10-member cases and the nine (all Passed in both modes).
#[test]
fn outcome_parity_and_dec053_basis_small_models() {
    let ids: Vec<String> = debug_ids()
        .into_iter()
        .filter(|id| !id.contains("-n00100-"))
        .collect();
    let asserted = ids.iter().filter(|id| parity(id)).count();
    assert_eq!(
        asserted,
        ids.len(),
        "every small model is Passed in both modes"
    );
}

/// D2 and D3 on the six 100-member cases (the set fixed at A1 from the
/// measured debug time; ROOT's ruling N17). D3 is asserted on CONT's two
/// Passed cases; the four CHAIN and TREE cases are Sensitive in both modes.
#[test]
fn outcome_parity_and_dec053_basis_100_member_models() {
    let asserted = debug_ids()
        .into_iter()
        .filter(|id| id.contains("-n00100-"))
        .filter(|id| parity(id))
        .count();
    assert!(asserted >= 1, "the Passed branch of D3 is exercised");
}
