//! RV89 U9 G9a (scratch only; never in maintained code): PR1080's receipt-order walk in
//! `source_blocks::validate_in`. (1) Is it reached on D1? Counters on `validate_in` and its
//! row loop across the milestone's Direct entry, both modes. (2) Off D1, where it is reached
//! (`for_source` on a source-blocks-1 document), does the new walk allocate more than the old?
//! A counting global allocator (all threads) around `for_source`, run with the loop as
//! committed and with only the loop header set back to PR1080's parent.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};
static ON: AtomicBool = AtomicBool::new(false);
static N: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
fn note(size: usize) { if ON.load(SeqCst) { N.fetch_add(1, SeqCst); BYTES.fetch_add(size, SeqCst); } }
struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 { note(l.size()); unsafe { System.alloc(l) } }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 { note(l.size()); unsafe { System.alloc_zeroed(l) } }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 { note(n); unsafe { System.realloc(p, l, n) } }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) { unsafe { System.dealloc(p, l) } }
}
#[global_allocator]
static RV89_COUNTING: Counting = Counting;
fn count<T>(f: impl FnOnce() -> T) -> (T, usize, usize) {
    N.store(0, SeqCst); BYTES.store(0, SeqCst); ON.store(true, SeqCst);
    let v = f();
    ON.store(false, SeqCst);
    (v, N.load(SeqCst), BYTES.load(SeqCst))
}
use crate::PreviewSolverMode;
use open_pipe_stress_result_export::{semantic_contract as sc, source_blocks as sb};
use serde_json::Value;
fn calls() -> (usize, usize) { (sb::RV89_VALIDATE_IN_CALLS.load(SeqCst), sb::RV89_INTEGER_CALLS.load(SeqCst)) }

#[test]
fn rv89int_integer_reach_and_allocations() {
    // (1) D1: the milestone's Direct entry, both modes (precommit reader included).
    let milestone: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap();
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let before = calls();
        let out = crate::run_linear_static_preview_value_with_retained_direct(milestone.clone(), mode).unwrap();
        let after = calls();
        println!("RV89_INT_D1 {mode:?} successor={} validate_in_calls={} integer_calls={}", out.successor().is_some(), after.0 - before.0, after.1 - before.1);
        assert!(out.successor().is_some());
        assert_eq!(after, before, "validate_in and integer are not reached on D1");
    }
    // (2) Off D1: for_source on source-blocks-1 documents.
    let docs: [(&str, &str); 4] = [
        ("n05 sparse", include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json")),
        ("n05 dense", include_str!("../../../fixtures/product_preview/source_blocks/n05-dense_scrutiny.raw.json")),
        ("multicase sparse", include_str!("../../../fixtures/product_preview/source_blocks/multicase-sparse_interactive.raw.json")),
        ("multicase dense", include_str!("../../../fixtures/product_preview/source_blocks/multicase-dense_scrutiny.raw.json")),
    ];
    for (label, text) in docs {
        let doc: Value = serde_json::from_str(text).unwrap();
        for round in 0..2 {
            let before = calls();
            let (r, n, b) = count(|| sc::for_source(&doc).map(|(_, v)| v));
            let after = calls();
            println!("RV89_INT_OFFD1 {label} round={round} result={:?} validate_in_calls={} integer_calls={} allocations={n} bytes={b}",
                     r, after.0 - before.0, after.1 - before.1);
        }
    }
}
