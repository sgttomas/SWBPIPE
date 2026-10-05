//! RV89 U7 (scratch only; never in maintained code): does the reader's eligibility switch
//! (`IMPLEMENTATION_COMPLETE`, result_export retained_precision.rs:4269, read at :4309)
//! allocate or add text on D1? A counting global allocator counts every allocation made
//! while counting is on (all threads, so the reserved-stack W1 thread is included), around
//! (a) the precommit reader call on the milestone's successor with its invocation, and
//! (b) the whole Direct entry, in both modes. The same probe runs in RV89's copy of
//! `cfda60403f` as committed (flag true) and with only the flag set back to false.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};

static ON: AtomicBool = AtomicBool::new(false);
static N: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
fn note(size: usize) {
    if ON.load(SeqCst) {
        N.fetch_add(1, SeqCst);
        BYTES.fetch_add(size, SeqCst);
    }
}
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
    N.store(0, SeqCst);
    BYTES.store(0, SeqCst);
    ON.store(true, SeqCst);
    let v = f();
    ON.store(false, SeqCst);
    (v, N.load(SeqCst), BYTES.load(SeqCst))
}

use crate::PreviewSolverMode;
use open_pipe_stress_result_export::retained_precision as reader;
use serde_json::{json, Value};

fn milestone() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()
}

#[test]
fn rv89u7_eligibility_switch_allocations() {
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let out = crate::run_linear_static_preview_value_with_retained_direct(milestone(), mode).unwrap();
        let successor = out.successor().expect("the registered build publishes the milestone's successor").clone();
        let invocation = json!({"request": milestone(), "solver_mode": mode.as_str()});
        for round in 0..2 {
            let (v, n, b) = count(|| reader::validate(&successor, Some(&invocation)));
            let v = v.expect("the precommit reader accepts the milestone's successor");
            println!("RV89_U7_READER {mode:?} round={round} invocation_bound={} numerical_eligible={} allocations={n} bytes={b} classifications={}",
                     v.invocation_bound, v.numerical_eligible, v.classifications.len());
            let (v, n, b) = count(|| reader::validate(&successor, None));
            let v = v.unwrap();
            println!("RV89_U7_READER_NO_INVOCATION {mode:?} round={round} numerical_eligible={} allocations={n} bytes={b}", v.numerical_eligible);
        }
        for round in 0..2 {
            let (o, n, b) = count(|| crate::run_linear_static_preview_value_with_retained_direct(milestone(), mode).unwrap());
            let published = o.successor().map(|s| serde_json::to_vec(s).unwrap().len());
            println!("RV89_U7_DIRECT {mode:?} round={round} successor_len={published:?} allocations={n} bytes={b}");
            drop(o);
        }
    }
}
