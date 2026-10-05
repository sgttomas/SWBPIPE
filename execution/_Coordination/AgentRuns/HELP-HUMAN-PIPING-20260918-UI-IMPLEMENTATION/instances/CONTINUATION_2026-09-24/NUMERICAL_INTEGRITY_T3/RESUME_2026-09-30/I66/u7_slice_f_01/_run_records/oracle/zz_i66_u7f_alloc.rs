//! I66 U7 slice F (scratch lanes only, never committed): the allocation evidence for the
//! flag hunk. A counting global allocator measures `retained_precision::validate` on both
//! milestones, with and without the actual invocation, three times each. Run in the base
//! lane (flag false: the conjunction short-circuits at the constant) and in the candidate
//! lane (flag true: the rest of the conjunction is evaluated). Equal counts and bytes show
//! that evaluating the conjunct allocates nothing. Output: I66_U7F_ALLOC_OUT.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::Value;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

struct Counting;
static CALLS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        CALLS.fetch_add(1, SeqCst);
        BYTES.fetch_add(layout.size() as u64, SeqCst);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        CALLS.fetch_add(1, SeqCst);
        BYTES.fetch_add(new_size as u64, SeqCst);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
#[global_allocator]
static GLOBAL: Counting = Counting;

#[test]
#[ignore]
fn zz_i66_u7f_alloc() {
    let mut lines = Vec::new();
    for (mode, text) in [
        ("sparse_interactive", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json")),
        ("dense_scrutiny", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")),
    ] {
        let doc: Value = serde_json::from_str(text).unwrap();
        for with in [true, false] {
            let invocation = with.then_some(&doc["invocation"]);
            for run in 0..3 {
                let (c0, b0) = (CALLS.load(SeqCst), BYTES.load(SeqCst));
                let v = rp::validate(&doc["source"], invocation).unwrap();
                let (c1, b1) = (CALLS.load(SeqCst), BYTES.load(SeqCst));
                let eligible = v.numerical_eligible;
                drop(v);
                lines.push(format!("{mode}\tinvocation={with}\trun={run}\tallocations={}\tbytes={}\tnumerical_eligible={eligible}", c1 - c0, b1 - b0));
            }
        }
    }
    std::fs::write(std::env::var("I66_U7F_ALLOC_OUT").unwrap(), lines.join("\n") + "\n").unwrap();
}
