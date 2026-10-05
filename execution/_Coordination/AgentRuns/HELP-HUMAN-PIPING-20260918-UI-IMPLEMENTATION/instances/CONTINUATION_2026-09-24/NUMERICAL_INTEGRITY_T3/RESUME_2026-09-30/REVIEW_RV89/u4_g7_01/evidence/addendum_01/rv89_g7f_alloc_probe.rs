//! RV89 G7 final basis (scratch only; never in maintained code): do grant 2's three
//! `#[cfg(test)]` statements allocate in the lib test binary, unarmed or armed? A counting
//! global allocator (this probe only) counts this thread's allocations around each call.
//! Mounted at the crate root of RV89's copy of `7f07a2f7b4`.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static N: Cell<usize> = const { Cell::new(0) };
    static BYTES: Cell<usize> = const { Cell::new(0) };
    static ON: Cell<bool> = const { Cell::new(false) };
}
fn note(size: usize) {
    if ON.try_with(|o| o.get()).unwrap_or(false) {
        let _ = N.try_with(|n| n.set(n.get() + 1));
        let _ = BYTES.try_with(|b| b.set(b.get() + size));
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
    N.with(|n| n.set(0));
    BYTES.with(|b| b.set(0));
    ON.with(|o| o.set(true));
    let v = f();
    ON.with(|o| o.set(false));
    (v, N.with(|n| n.get()), BYTES.with(|b| b.get()))
}

use crate::retained_product::ProductCapture;
use crate::retained_tests_hooks as h;

#[test]
fn rv89g7f_grant2_hooks_allocate_nothing() {
    // A fresh thread, so each hook's thread-locals are first touched inside `count`.
    std::thread::spawn(|| {
        let mut cap = ProductCapture::prepared_probe();
        let mut rows = Vec::new();
        let mut record = |label: &str, (n, b): (usize, usize)| {
            println!("RV89_G7F_ALLOC {label}: allocations={n} bytes={b}");
            rows.push((label.to_string(), n, b));
        };
        // Unarmed, first touches on this thread (TALLY: const RefCell<Option<Arc<_>>>, has drop;
        // ARMED: lazy Cell<Armed>, Copy).
        let ((), n, b) = count(|| h::ordinary_run_entered());
        record("unarmed ordinary_run_entered (first touch of TALLY)", (n, b));
        let ((), n, b) = count(|| h::ordinary_run_entered());
        record("unarmed ordinary_run_entered (again)", (n, b));
        let ((), n, b) = count(|| h::before_late_gate(&cap));
        record("unarmed before_late_gate (first touch of ARMED)", (n, b));
        let ((), n, b) = count(|| h::at_complete_gate(&mut cap));
        record("unarmed at_complete_gate", (n, b));
        let ((), n, b) = count(|| h::at_complete_gate(&mut cap));
        record("unarmed at_complete_gate (again)", (n, b));
        // Armed, one at a time (arming happens outside the count).
        h::fail_next_late_gate();
        let ((), n, b) = count(|| h::before_late_gate(&cap));
        record("armed late_gate: before_late_gate", (n, b));
        h::fail_next_complete_gate();
        let ((), n, b) = count(|| h::at_complete_gate(&mut cap));
        record("armed complete_gate: at_complete_gate", (n, b));
        h::fault_next_candidate(crate::retained_receipt::TraceFault::Maxima);
        let ((), n, b) = count(|| h::at_complete_gate(&mut cap));
        record("armed candidate: at_complete_gate", (n, b));
        assert_eq!(cap.trace_fault, Some(crate::retained_receipt::TraceFault::Maxima), "the armed candidate fault fired");
        // With a live tally (as grant 2's tests run them): the hooks clone the Arc only.
        let (((), n, b), counts) = h::counted(|| count(|| { h::ordinary_run_entered(); h::at_complete_gate(&mut cap); }));
        record("tally active: ordinary_run_entered + at_complete_gate", (n, b));
        assert_eq!((counts.runs, counts.complete_gates), (1, 1));
        h::disarm();
        // Positive control: the counter sees an allocation made inside `count`.
        let (bx, n, b) = count(|| Box::new(7u64));
        record("positive control: Box::new(u64) (first touch)", (n, b));
        assert!(n == 1 && b == 8, "the counter counts");
        drop(bx);
        for (label, n, _) in &rows {
            if !label.contains("first touch") {
                assert_eq!(*n, 0, "{label}");
            }
        }
    }).join().unwrap();
}
