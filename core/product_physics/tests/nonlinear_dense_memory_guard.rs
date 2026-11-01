//! §8.30 correction 2 (RT-U4, RT-A2; R-4 design C-6 and §3.6): the nonlinear
//! active-set loop's dense memory guard, at the real provisional limit.
//!
//! The loop is dense in both solver modes. Its leading dense storage is
//! estimated as 88 x n^2 + 16 x n_f^2 bytes (n global DOFs, n_f free), a
//! leading-buffer estimate, not a complete memory bound; the provisional limit
//! is floor(0.9 x 12 GiB) = 11,596,411,699 bytes (NI
//! `DENSE_LOOP_MEMORY_CEILING_BYTES`). A 1,760-member chain anchored at N0 with
//! one one-way support at its tip (1,761 nodes, 10,566 global DOFs, 10,560
//! free) estimates 11,608,568,928 bytes and is refused in both modes, on both
//! public entries, before any n^2 allocation: blocking
//! `NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED` with both values in bytes, no
//! mechanics row, and a peak heap increment below 64 MiB. One node fewer is
//! under the limit (arithmetic only: running it would allocate the dense loop).
//!
//! The counting global allocator is this binary's own. Its cap (512 MiB) is a
//! safety net: without the guard the loop's dense stiffness alone (893 MB)
//! aborts this binary instead of exhausting the host. Deterministic
//! allocation counts, not a memory claim. Invented inputs only.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const CAP_BYTES: usize = 512 << 20;
const BOUND_BYTES: usize = 64 << 20;
const MEMBERS: usize = 1_760;
const CEILING: u128 = 11_596_411_699;
const CODE: &str = "NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
/// The tests share the counters, so they run one at a time.
static SERIAL: Mutex<()> = Mutex::new(());

struct Counting;

impl Counting {
    fn grow(size: usize) {
        let now = CURRENT.fetch_add(size, Ordering::SeqCst) + size;
        if now > CAP_BYTES {
            // The safety net: never exhaust the host.
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

const PROV: &str = "invented_830_nonlinear_guard_input_no_library_data";

/// A chain of `members` unit members along x, anchored at N0, with a one-way
/// +y stop at its tip and a downward tip load.
fn chain_request(members: usize) -> Value {
    let nodes: Vec<Value> = (0..=members)
        .map(|i| json!({"id": format!("N{i}"), "position": {"x": i as f64, "y": 0.0, "z": 0.0}, "provenance": PROV}))
        .collect();
    let pipes: Vec<Value> = (1..=members)
        .map(|i| {
            json!({"id": format!("M{i}"), "from": format!("N{}", i - 1), "to": format!("N{i}"), "material": "mat:NL",
                   "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                   "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
                   "provenance": PROV})
        })
        .collect();
    json!({"model": {
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:830:nonlinear-guard-chain-{members}"),
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": nodes,
        "pipe_segments": pipes,
        "materials": [{"id": "mat:NL", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"},
                       "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": PROV}],
        "supports": [
            {"id": "anchor:N0", "node": "N0", "family": "anchor",
             "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "stop:tip", "node": format!("N{members}"), "family": "nonlinear", "restraints": [],
             "nonlinear": {"behavior": "one_way", "dof": "UY", "initial_state": "active",
                           "active_when": "positive_reaction"},
             "provenance": PROV}],
        "load_cases": [{"id": "case", "label": "tip", "kind": "primitive_user_load",
                        "primitive_loads": [{"id": "load:tip", "category": "concentrated_force",
                            "target": {"type": "node", "node": format!("N{members}")},
                            "direction": "global_y", "magnitude": {"value": -10.0, "unit": "N"},
                            "dimension": "force", "provenance": PROV}],
                        "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

fn peak_increment<T>(f: impl FnOnce() -> T) -> (T, usize) {
    let base = CURRENT.load(Ordering::SeqCst);
    PEAK.store(base, Ordering::SeqCst);
    let value = f();
    (value, PEAK.load(Ordering::SeqCst) - base)
}

fn estimate(nodes: u128, restrained: u128) -> u128 {
    let global = 6 * nodes;
    88 * global * global + 16 * (global - restrained) * (global - restrained)
}

fn assert_refused_by_the_guard(envelope: &MechanicsEnvelope, entry: &str) {
    assert!(envelope.results.is_empty(), "{entry}: no mechanics row");
    assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{entry}");
    let blocking: Vec<_> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .collect();
    assert_eq!(blocking.len(), 1, "{entry}: {:?}", envelope.diagnostics);
    let guard = blocking[0];
    assert_eq!(guard.code, CODE, "{entry}");
    assert_eq!(guard.id, "diagnostic:physics:nonlinear-dense-memory-guard", "{entry}");
    assert_eq!(guard.affected_refs, ["model"], "{entry}");
    assert_eq!(
        guard.message,
        "nonlinear dense loop memory guard: estimated leading dense storage 11608568928 bytes (88 bytes x 10566^2 global DOFs + 16 bytes x 10560^2 free DOFs) exceeds the provisional limit 11596411699 bytes; the model is refused before any n^2 allocation. The estimate counts the loop's leading dense buffers only and is not a complete memory bound; the limit is provisional until the sparse nonlinear loop (R-4 stage P); it applies in sparse_interactive and dense_scrutiny alike",
        "{entry}"
    );
}

#[test]
fn a_nonlinear_model_just_over_the_limit_is_refused_in_both_modes_before_any_dense_allocation() {
    let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let nodes = (MEMBERS + 1) as u128;
    assert_eq!(estimate(nodes, 6), 11_608_568_928);
    assert!(estimate(nodes, 6) > CEILING);
    assert!(estimate(nodes - 1, 6) <= CEILING, "one node fewer is under the limit");
    let request = chain_request(MEMBERS);
    for mode in MODES {
        let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
        let (typed_envelope, typed_peak) =
            peak_increment(|| run_linear_static_preview_with_mode(typed, mode));
        assert_refused_by_the_guard(&typed_envelope, &format!("typed {mode:?}"));
        drop(typed_envelope);
        let (captured_envelope, captured_peak) = peak_increment(|| {
            run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()
        });
        assert_refused_by_the_guard(&captured_envelope, &format!("captured {mode:?}"));
        eprintln!(
            "830-C2: mode={mode:?} members={MEMBERS} typed_peak_increment_bytes={typed_peak} captured_peak_increment_bytes={captured_peak} bound_bytes={BOUND_BYTES}"
        );
        assert!(typed_peak < BOUND_BYTES, "{mode:?}: typed peak {typed_peak} bytes");
        assert!(captured_peak < BOUND_BYTES, "{mode:?}: captured peak {captured_peak} bytes");
    }
}

/// Negative: a nonlinear model under the limit solves as before, with no
/// guard diagnostic.
#[test]
fn a_nonlinear_model_under_the_limit_is_unaffected() {
    let _serial = SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let request = chain_request(8);
    for mode in MODES {
        let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
        assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{mode:?}: {:?}", envelope.diagnostics);
        assert!(!envelope.diagnostics.iter().any(|d| d.code == CODE), "{mode:?}");
        assert!(!envelope.results.is_empty(), "{mode:?}");
    }
}
