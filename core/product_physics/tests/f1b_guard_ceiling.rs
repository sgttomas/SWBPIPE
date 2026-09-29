//! F1b test C-CEILING (T3 D1 revision 5a.2 §4.8 "Resource guard"; ROOT's
//! F1b rulings Q8 and the checkpoint-0 sizes): at the real, provisional 6 GiB
//! ceiling, a 1,365-member axis-aligned chain (1,366 nodes, 8,196 global
//! DOFs, estimate 96 x 8,196^2 = 6,448,743,936 bytes) is refused in dense
//! scrutiny on both public entries, before any n^2 allocation: blocking
//! `SOLVER_SYSTEM_BLOCKED` naming the estimate, no mechanics row, and a peak
//! heap increment below 64 MiB (about 1 % of the estimate). It never reaches
//! 10,000 members.
//!
//! The counting global allocator is this binary's own. Its cap (512 MiB) is a
//! safety net: without the guard the dense view alone (537 MB) would abort
//! this binary instead of exhausting the host; such an abort is not a counted
//! kill (the lowered-ceiling unit test is the guard's behavioural kill).
//! Deterministic allocation counts, not a memory claim. Invented inputs only.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

const CAP_BYTES: usize = 512 << 20;
const BOUND_BYTES: usize = 64 << 20;
const MEMBERS: usize = 1_365;

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

const PROV: &str = "invented_t3_f1b_guard_input_no_library_data";

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
        "project": {"id": format!("invented:t3-f1b:guard-chain-{members}"),
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

fn peak_increment<T>(f: impl FnOnce() -> T) -> (T, usize) {
    let base = CURRENT.load(Ordering::SeqCst);
    PEAK.store(base, Ordering::SeqCst);
    let value = f();
    (value, PEAK.load(Ordering::SeqCst) - base)
}

fn assert_refused_by_the_guard(envelope: &MechanicsEnvelope, entry: &str) {
    assert!(envelope.results.is_empty(), "{entry}: no mechanics row");
    assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{entry}");
    let guard: Vec<_> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.id == "diagnostic:physics:dense-scrutiny-resource-guard")
        .collect();
    assert_eq!(guard.len(), 1, "{entry}: {:?}", envelope.diagnostics);
    assert_eq!(guard[0].code, "SOLVER_SYSTEM_BLOCKED", "{entry}");
    assert_eq!(guard[0].severity, "blocking", "{entry}");
    assert_eq!(
        guard[0].message,
        "dense scrutiny resource guard: estimated dense-path peak 6448743936 bytes (96 bytes x 67174416 dense entries, 8196 global DOFs squared) exceeds the provisional ceiling 6442450944 bytes; the model is refused before any n^2 allocation. The estimate is a stated formula, not a measurement; sparse_interactive does not use it, and no automatic dense fallback exists",
        "{entry}"
    );
}

#[test]
fn f1b_dense_scrutiny_is_refused_just_above_the_real_ceiling_before_any_dense_allocation() {
    let dimension = 6 * (MEMBERS + 1);
    assert_eq!(dimension, 8_196);
    assert!(96 * (dimension as u128).pow(2) > 6 * (1u128 << 30));
    assert!(
        96 * ((dimension - 6) as u128).pow(2) <= 6 * (1u128 << 30),
        "one node fewer passes"
    );
    let request = chain_request(MEMBERS);
    let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let started = std::time::Instant::now();
    let (typed_envelope, typed_peak) = peak_increment(|| {
        run_linear_static_preview_with_mode(typed, PreviewSolverMode::DenseScrutiny)
    });
    assert_refused_by_the_guard(&typed_envelope, "typed");
    drop(typed_envelope);
    let (captured_envelope, captured_peak) = peak_increment(|| {
        run_linear_static_preview_value_with_mode(request, PreviewSolverMode::DenseScrutiny)
            .unwrap()
    });
    assert_refused_by_the_guard(&captured_envelope, "captured");
    eprintln!(
        "C-CEILING: members={MEMBERS} typed_peak_increment_bytes={typed_peak} captured_peak_increment_bytes={captured_peak} bound_bytes={BOUND_BYTES} estimate_bytes=6448743936 wall_seconds={:.1}",
        started.elapsed().as_secs_f64()
    );
    assert!(typed_peak < BOUND_BYTES, "typed peak {typed_peak} bytes");
    assert!(
        captured_peak < BOUND_BYTES,
        "captured peak {captured_peak} bytes"
    );
}
