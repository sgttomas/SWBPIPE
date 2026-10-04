use open_pipe_stress_product_physics::*;
use serde_json::Value;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct CountAlloc;
thread_local! { static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) }; }
fn entered_allocation() {
    let _ = ALLOCATIONS.try_with(|n| {
        if let Some(v) = n.get() {
            n.set(Some(v + 1));
        }
    });
}
unsafe impl GlobalAlloc for CountAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        entered_allocation();
        System.alloc(layout)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        entered_allocation();
        System.alloc_zeroed(layout)
    }
    unsafe fn realloc(&self, p: *mut u8, layout: Layout, n: usize) -> *mut u8 {
        entered_allocation();
        System.realloc(p, layout, n)
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        System.dealloc(p, layout)
    }
}
#[global_allocator]
static ALLOCATOR: CountAlloc = CountAlloc;

fn ordinary() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"
    ))
    .unwrap()
}
/// G6: the registered build (QUALIFICATION.md) reports `Registered`; any other build `Stale`.
const REGISTERED_IDENTITY: &str = "v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0";
fn expected_profile() -> ProfileStatus {
    if option_env!("OPS_RETAINED_BUILD_IDENTITY") == Some(REGISTERED_IDENTITY) {
        ProfileStatus::Registered
    } else {
        ProfileStatus::Stale
    }
}
fn bytes(e: &MechanicsEnvelope) -> Vec<u8> {
    serde_json::to_vec(e).unwrap()
}

#[test]
fn retained_direct_preserves_actual_ordinary_both_modes_and_generic_identity() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for renamed in [false, true] {
            let mut raw = ordinary();
            if renamed {
                raw["model"]["project"]["id"] = Value::String("admission-generic-project".into());
            }
            let expected = run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap();
            let actual = run_linear_static_preview_value_with_retained_direct(raw, mode).unwrap();
            assert_eq!(bytes(actual.envelope()), bytes(&expected));
            let report = actual.admission().unwrap();
            assert_eq!(report.caller, RetainedCaller::Direct);
            assert_eq!(report.profile, expected_profile());
            assert_eq!(report.allowance, AllowanceStatus::Unselected);
            assert!(report.census_complete());
            assert_eq!(report.typed.nodes.length, 2);
            assert!(report
                .required_unknown_terms()
                .contains(&MissingAdmissionTerm::OrdinaryActiveAndSuffix));
            assert!(serde_json::to_value(actual.envelope())
                .unwrap()
                .get("retained_precision")
                .is_none());
        }
    }
}

#[test]
fn exact_and_invalid_ordinary_results_are_preserved() {
    let exact: Value = serde_json::from_str(include_str!(
        "fixtures/exact_pressure_connected_request.json"
    ))
    .unwrap();
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let expected = run_linear_static_preview_value_with_mode(exact.clone(), mode).unwrap();
        let actual =
            run_linear_static_preview_value_with_retained_direct(exact.clone(), mode).unwrap();
        assert_eq!(bytes(actual.envelope()), bytes(&expected));
        // The pressure fixture's legacy route need not select source blocks.
        // Use the maintained explicit-material companion for that positive gate.
        let source: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/physics_source/n05.request.json"
        ))
        .unwrap();
        let selected = run_linear_static_preview_value_with_mode(source.clone(), mode).unwrap();
        assert!(selected.source_block_recovery.is_some());
        let refused = run_linear_static_preview_value_with_retained_direct(source, mode).unwrap();
        assert_eq!(bytes(refused.envelope()), bytes(&selected));
        assert_eq!(refused.admission().unwrap().profile, expected_profile());
        let mut invalid = ordinary();
        invalid["model"]["document_kind"] = Value::String("invalid-kind".into());
        let expected = run_linear_static_preview_value_with_mode(invalid.clone(), mode).unwrap();
        let actual = run_linear_static_preview_value_with_retained_direct(invalid, mode).unwrap();
        // G6 (ROOT): the invalid document's ordinary route returns before attempting the
        // case's solve, so G-C declines W1 and the bytes are the value route's exactly,
        // with no notice, registered or not.
        assert_eq!(bytes(actual.envelope()), bytes(&expected));
        assert!(actual.successor().is_none());
        assert_eq!(actual.admission().unwrap().profile, expected_profile());
    }
    let bad = serde_json::json!({"model": null});
    assert_eq!(
        run_linear_static_preview_value_with_mode(
            bad.clone(),
            PreviewSolverMode::SparseInteractive
        )
        .err(),
        run_linear_static_preview_value_with_retained_direct(
            bad,
            PreviewSolverMode::SparseInteractive
        )
        .err()
    );
}

#[test]
fn borrowed_census_allocates_nothing_and_retains_spare_capacity_facts() {
    let mut text = String::with_capacity(137);
    text.push_str("short");
    let mut array = Vec::with_capacity(29);
    array.push(Value::String(text));
    let raw = Value::Array(array);
    let mut typed: LinearStaticPreviewRequest = serde_json::from_value(ordinary()).unwrap();
    typed.model.nodes.reserve_exact(91);
    let capacity = typed.model.nodes.capacity();
    ALLOCATIONS.with(|n| n.set(Some(0)));
    let facts = borrowed_value_census(&raw);
    let typed_facts = borrowed_request_census(&typed);
    let allocated = ALLOCATIONS.with(|n| {
        let v = n.get().unwrap();
        n.set(None);
        v
    });
    assert_eq!(allocated, 0);
    assert_eq!(facts.status, CensusStatus::Complete);
    assert_eq!(facts.array_elements, 1);
    assert_eq!(facts.array_capacity_elements, 29);
    assert_eq!(facts.string_bytes, 5);
    assert_eq!(facts.string_capacity_bytes, 137);
    assert_eq!(typed_facts.nodes.capacity, capacity);
    assert!(typed_facts.nodes.capacity > typed_facts.nodes.length);
}

#[test]
fn deep_unknown_raw_field_declines_census_without_restricting_ordinary() {
    let mut raw = ordinary();
    let mut child = Value::Null;
    for _ in 0..70 {
        child = Value::Array(vec![child]);
    }
    raw["unknown_admission_depth"] = child;
    let expected = run_linear_static_preview_value_with_mode(
        raw.clone(),
        PreviewSolverMode::SparseInteractive,
    )
    .unwrap();
    let actual = run_linear_static_preview_value_with_retained_direct(
        raw,
        PreviewSolverMode::SparseInteractive,
    )
    .unwrap();
    assert_eq!(bytes(actual.envelope()), bytes(&expected));
    assert_eq!(
        actual.admission().unwrap().raw.status,
        CensusStatus::DepthLimit
    );
    assert!(!actual.admission().unwrap().census_complete());
}

fn section<'a>(s: &'a str, start: &str, end: &str) -> &'a str {
    let suffix = &s[s.find(start).unwrap()..];
    &suffix[..suffix.find(end).unwrap()]
}
#[test]
fn legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission() {
    let pp = include_str!("../src/lib.rs");
    let old = section(
        pp,
        "pub fn run_linear_static_preview_value_with_mode(",
        "pub struct RetainedPreviewOutput",
    );
    assert!(
        old.contains("run_linear_static_preview_value_dispatch(actual_request, solver_mode, None)")
    );
    let typed = section(
        pp,
        "pub fn run_linear_static_preview_with_mode(",
        "pub fn run_linear_static_preview_value_with_mode(",
    );
    assert!(typed.contains("run_linear_static_preview_captured(request, solver_mode, None,"));
    let dispatch = section(
        pp,
        "fn run_linear_static_preview_value_dispatch(",
        "fn run_linear_static_preview_captured(",
    );
    assert_eq!(
        dispatch
            .matches("run_linear_static_preview_captured(")
            .count(),
        1
    );
    for forbidden in [
        "ProductCapture::",
        "prepare_observed(",
        "project_candidate(",
        "retained_memory::Entry::Direct",
    ] {
        assert!(!dispatch.contains(forbidden));
    }
    let native = include_str!("../../../apps/desktop/src-tauri/src/lib.rs");
    assert!(native.contains("run_linear_static_preview_value_with_mode(request, solver_mode)"));
    assert!(native.contains(
        "model_payload.and_then(|payload| solve_preview_mechanics_with_mode(payload, solver_mode))"
    ));
    assert!(!native.contains("with_retained_direct") && !native.contains("with_retained_headless"));
    let typed: LinearStaticPreviewRequest = serde_json::from_value(ordinary()).unwrap();
    let output = run_linear_static_preview_with_mode(typed, PreviewSolverMode::SparseInteractive);
    assert!(serde_json::to_value(output)
        .unwrap()
        .get("retained_precision")
        .is_none());
}
