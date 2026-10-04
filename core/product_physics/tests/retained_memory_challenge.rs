//! U4 G5 part 2: the allocation challenge (brief §6). A challenge only, not a proof
//! and not a production guard.
//!
//! This binary's own global allocator records the peak of live requested heap
//! bytes (the f1b_sparse_pattern_memory.rs pattern). Through the public retained
//! Direct entry (no permit exists, so the ordinary span runs: G-A's census and
//! law, the parse, the ordinary run), it measures the peak on the milestone and on
//! a cap-maximal D1 input, in both modes, and challenges the cap-priced profile:
//! each measured peak must stay at or below the profile's in-build W1 phase (the
//! ordinary span, requested plus moving, without R). The bounds below are the
//! generated profile's values; a lib test (`challenge_bounds_are_the_profile`)
//! reads this file and checks they equal the in-build evaluation.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_retained_direct, PreviewSolverMode};
use serde_json::{json, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The profile's in-build W1 phase (requested + moving, without R), sparse and dense, in the
/// pinned record's build (the lib test `challenge_bounds_are_the_profile` checks it there).
const W1_PHASE_BYTES: [u64; 2] = [1_856_156_348, 1_875_866_796];
/// The profile's in-build maximum over every phase (E_mov,max, without R; W3 in both modes),
/// the bound once a permit admits the W1 phases (a registered build, G6).
const MAX_PHASE_BYTES: [u64; 2] = [3_508_669_422, 3_528_379_870];
const CAP_BYTES: usize = 6 << 30;

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
struct Counting;
impl Counting {
    fn grow(size: usize) {
        let now = CURRENT.fetch_add(size, Ordering::SeqCst) + size;
        if now > CAP_BYTES {
            std::process::abort();
        }
        PEAK.fetch_max(now, Ordering::SeqCst);
    }
}
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Self::grow(layout.size());
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Self::grow(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        CURRENT.fetch_sub(layout.size(), Ordering::SeqCst);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            Self::grow(new_size - layout.size());
        } else {
            CURRENT.fetch_sub(layout.size() - new_size, Ordering::SeqCst);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
#[global_allocator]
static GLOBAL: Counting = Counting;

fn milestone() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()
}
/// A large D1 input in the law tests' shape (32 nodes, a 32-member ring, 32 supports:
/// one anchor and 31 scalar springs, 128 nodal loads, 4 + 4 materials with 16
/// temperature points, a 128-byte identifier).
fn cap_maximal() -> Value {
    let p = "invented_t3_g5_cap_maximal_input_no_library_data";
    let nodes: Vec<Value> = (0..32)
        .map(|i| {
            let t = 2.0 * std::f64::consts::PI * i as f64 / 32.0;
            json!({"id": format!("N{i}"), "position": {"x": 10.0 * t.cos(), "y": 10.0 * t.sin(), "z": 0.0}, "provenance": p})
        })
        .collect();
    let pipes: Vec<Value> = (0..32)
        .map(|i| json!({"id": format!("M{i}"), "from": format!("N{i}"), "to": format!("N{}", (i + 1) % 32), "material": "mat:0", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0},
            "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": p}))
        .collect();
    // Node 0 is anchored; every other node has one scalar spring, so the model solves.
    let supports: Vec<Value> = (0..32)
        .map(|i| if i == 0 {
            json!({"id": "S0", "node": "N0", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p})
        } else {
            json!({"id": format!("S{i}"), "node": format!("N{i}"), "family": "spring", "restraints": [],
                "stiffness": {"dof": "UY", "value": {"value": 1.0e6, "unit": "N/m"}}, "provenance": p})
        })
        .collect();
    let loads: Vec<Value> = (0..128)
        .map(|i| json!({"id": format!("L{i}"), "category": "concentrated_force", "target": {"type": "node", "node": format!("N{}", i % 32)},
            "direction": if i % 2 == 0 { "global_y" } else { "rotation_x" }, "magnitude": {"value": 1.0, "unit": if i % 2 == 0 { "N" } else { "N*m" }},
            "dimension": if i % 2 == 0 { "force" } else { "moment" }, "provenance": p}))
        .collect();
    let points: Vec<Value> = (0..16).map(|i| json!({"id": format!("T{i}"), "provenance": p})).collect();
    let materials: Vec<Value> = (0..4)
        .map(|i| json!({"id": format!("mat:{i}"), "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "shear_modulus": {"value": 8.0e10, "unit": "Pa"},
            "temperature_points": points, "provenance": p}))
        .collect();
    let mut project = String::from("project:");
    while project.len() < 128 {
        project.push('x');
    }
    json!({"model": {"schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": project, "units": {"length": "m", "force": "N"}},
        "nodes": nodes, "pipe_segments": pipes, "materials": materials, "supports": supports,
        "load_cases": [{"id": "case", "primitive_loads": loads, "provenance": p}], "combinations": []},
        "materials": materials})
}

#[test]
fn retained_direct_peak_is_within_the_profiles_ordinary_span() {
    for (label, raw) in [("milestone", milestone()), ("cap_maximal", cap_maximal())] {
        for (m, mode) in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny].into_iter().enumerate() {
            let input = raw.clone();
            let base = CURRENT.load(Ordering::SeqCst);
            PEAK.store(base, Ordering::SeqCst);
            let output = run_linear_static_preview_value_with_retained_direct(input, mode).unwrap();
            let blocking: Vec<&str> = output.envelope().diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
            println!("I65_G5_CHALLENGE_RUN {label} {mode:?} mechanics={} rows={} blocking={blocking:?}", output.envelope().status.mechanics,
                output.envelope().results.len());
            let peak = PEAK.load(Ordering::SeqCst) - base;
            // Without a permit the ordinary span runs (the W1 phase bounds it); with one (a
            // registered build) the W1 phases run too, and the whole maximum bounds it.
            let permitted = output.successor().is_some();
            let bound = if permitted { MAX_PHASE_BYTES[m] } else { W1_PHASE_BYTES[m] };
            drop(output);
            println!("I65_G5_CHALLENGE {label} {mode:?} permitted={permitted} peak_bytes={peak} bound_bytes={bound} ratio={:.6}",
                peak as f64 / bound as f64);
            assert!(peak as u64 <= bound, "{label} {mode:?}: measured peak {peak} above the profile's bound {bound}");
        }
    }
}
