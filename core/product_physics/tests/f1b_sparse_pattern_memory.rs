//! F1b test C-SPARSE (T3 D1 revision 5a.2 §4.8; ROOT's F1b rulings Q8):
//! sparse interactive mode is pattern-only. A 4,000-member axis-aligned chain
//! (24,006 global DOFs), solved in sparse mode through both public entries,
//! keeps its peak heap increment below 1 GiB, while the dense view of its
//! stiffness alone would need 8 x 24,006^2 = 4,610,304,288 bytes (4.29 times
//! the bound).
//!
//! Size and bound (I13 at A1, pending ROOT's ruling): checkpoint 0 proposed a
//! 1,000-member chain and 64 MiB. Measured here, the sparse path's peak grows
//! linearly, about 206 KB per member (206 MB at 1,000 members, 825 MB at
//! 4,000), so the dense view reaches four times a bound above the peak only
//! from about 2,900 members.
//!
//! The counting global allocator is this binary's own. Its cap (6 GiB, the
//! gate's heap cap) is a safety net: a regression that runs the whole dense
//! path aborts this binary instead of exhausting the host; such an abort is
//! not a counted kill. A regression that only materializes the dense view
//! (4.6 GB) stays under the cap and fails the peak assertion. The counts are
//! deterministic allocation counts, not a memory-growth or timing claim (K6
//! owns those). Invented inputs only.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

const CAP_BYTES: usize = 6 << 30;
const BOUND_BYTES: usize = 1 << 30;
const MEMBERS: usize = 4_000;

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

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

const PROV: &str = "invented_t3_f1b_memory_input_no_library_data";

/// An axis-aligned chain of `members` 1 m members along x, anchored at N0,
/// with a tip load (the RF-LARGE-CHAIN shape, invented section).
fn chain_request(members: usize) -> Value {
    let nodes: Vec<Value> = (0..=members)
        .map(|i| json!({"id": format!("N{i}"), "position": {"x": i as f64, "y": 0.0, "z": 0.0}, "provenance": PROV}))
        .collect();
    let pipes: Vec<Value> = (1..=members)
        .map(|i| {
            json!({"id": format!("M{i}"), "from": format!("N{}", i - 1), "to": format!("N{i}"), "material": "mat:F1B",
                   "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                   "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
                   "provenance": PROV})
        })
        .collect();
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:t3-f1b:chain-{members}"),
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": nodes,
        "pipe_segments": pipes,
        "materials": [{"id": "mat:F1B", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"},
                       "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": PROV}],
        "supports": [{"id": "anchor:N0", "node": "N0", "family": "anchor",
                      "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "load_cases": [{"id": "case", "label": "tip", "kind": "primitive_user_load",
                        "primitive_loads": [{"id": "load:tip", "category": "concentrated_force",
                            "target": {"type": "node", "node": format!("N{members}")},
                            "direction": "global_y", "magnitude": {"value": 10.0, "unit": "N"},
                            "dimension": "force", "provenance": PROV}],
                        "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

/// The peak heap increment of `f` over the heap in use when it starts.
fn peak_increment<T>(f: impl FnOnce() -> T) -> (T, usize) {
    let base = CURRENT.load(Ordering::SeqCst);
    PEAK.store(base, Ordering::SeqCst);
    let value = f();
    (value, PEAK.load(Ordering::SeqCst) - base)
}

fn assert_solved(envelope: &MechanicsEnvelope, entry: &str) {
    assert_eq!(
        envelope.status.mechanics, "MECHANICS_SOLVED",
        "{entry}: {:?}",
        envelope.diagnostics
    );
    let mode_rows: Vec<_> = envelope
        .results
        .iter()
        .filter(|r| r.kind == "linear_solver_mode_basis")
        .collect();
    assert_eq!(mode_rows.len(), 1, "{entry}");
    assert_eq!(mode_rows[0].value, 1.0, "{entry}: sparse_interactive");
}

#[test]
fn f1b_sparse_mode_is_pattern_only_at_four_thousand_members() {
    let dense_view_bytes = 8 * (6 * (MEMBERS + 1)).pow(2);
    assert_eq!(dense_view_bytes, 4_610_304_288);
    assert!(dense_view_bytes >= 4 * BOUND_BYTES);
    let request = chain_request(MEMBERS);
    let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let started = std::time::Instant::now();
    let (typed_envelope, typed_peak) = peak_increment(|| {
        run_linear_static_preview_with_mode(typed, PreviewSolverMode::SparseInteractive)
    });
    assert_solved(&typed_envelope, "typed");
    drop(typed_envelope);
    let (captured_envelope, captured_peak) = peak_increment(|| {
        run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive)
            .unwrap()
    });
    assert_solved(&captured_envelope, "captured");
    eprintln!(
        "C-SPARSE: members={MEMBERS} typed_peak_increment_bytes={typed_peak} captured_peak_increment_bytes={captured_peak} bound_bytes={BOUND_BYTES} dense_view_bytes={dense_view_bytes} wall_seconds={:.1}",
        started.elapsed().as_secs_f64()
    );
    assert!(typed_peak < BOUND_BYTES, "typed peak {typed_peak} bytes");
    assert!(
        captured_peak < BOUND_BYTES,
        "captured peak {captured_peak} bytes"
    );
}
