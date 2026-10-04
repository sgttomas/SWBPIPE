//! U3 grant 2 (I61): the test-only seams the actual Direct entry's permitted path needs,
//! as a child of `retained_tests_hooks` (test builds only; production is unchanged).
//!
//! - **B-1/S-7 and RV85 U4/N2:** a per-invocation tally of ordinary runs entered (the first
//!   statement of `run_linear_static_preview_observed`, which every ordinary run passes) and
//!   of G-C consultations (`permitted_run`, just before `check_complete`). The tally is shared
//!   with the reserved-stack thread: `carry_test_hooks` carries it with the armed faults.
//! - **Faults on the actual entry's own observer and owner:** G-B and G-C refusals by the
//!   actual gates (the observer's capacity record is raised above every bound), and
//!   preparation and candidate (the private driver's triggers). Each is armed on the
//!   caller's thread, carried across the hop, and consumed once by its own seam.
//!   Coexistence needs no seam: the source-block fixtures n05/n06 select exact blocks.
use super::{arm, consume, Armed};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::Arc;

/// One invocation's counts, shared with the reserved-stack thread.
#[derive(Debug, Default)]
pub(crate) struct Tally { runs: AtomicUsize, complete_gates: AtomicUsize }
thread_local! {
    static TALLY: std::cell::RefCell<Option<Arc<Tally>>> = const { std::cell::RefCell::new(None) };
}
fn tally() -> Option<Arc<Tally>> { TALLY.with(|t| t.borrow().clone()) }
/// What `counted` observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Counts { pub(crate) runs: usize, pub(crate) complete_gates: usize }
/// Run `f` on this thread with a fresh tally (the permitted work carries it to the
/// reserved-stack thread). One tally at a time per thread.
pub(crate) fn counted<T>(f: impl FnOnce() -> T) -> (T, Counts) {
    let fresh = Arc::new(Tally::default());
    assert!(TALLY.with(|t| t.replace(Some(fresh.clone()))).is_none(), "one tally at a time");
    let value = f();
    TALLY.with(|t| t.borrow_mut().take());
    (value, Counts { runs: fresh.runs.load(SeqCst), complete_gates: fresh.complete_gates.load(SeqCst) })
}
/// The caller's tally in transit with its faults (`Carried`): shared, not copied.
pub(super) struct TallyCarry(Option<Arc<Tally>>);
impl TallyCarry {
    pub(super) fn take() -> Self { Self(tally()) }
    /// On the worker, with the carried faults.
    pub(super) fn install(&mut self) { TALLY.with(|t| *t.borrow_mut() = self.0.take()); }
    /// On the worker, after the work.
    pub(super) fn hand_back() { TALLY.with(|t| t.borrow_mut().take()); }
}
/// `reclaim_handed_back`: this grant's unfired faults are re-armed like the others.
pub(super) fn merge(a: &mut Armed, faults: &Armed) {
    a.late_gate |= faults.late_gate;
    a.complete_gate |= faults.complete_gate;
    a.preparation |= faults.preparation;
    a.candidate = a.candidate.or(faults.candidate);
}
/// `armed_names`: this grant's faults.
pub(super) fn names(a: &Armed) -> [(bool, &'static str); 4] {
    [(a.late_gate, "late_gate"), (a.complete_gate, "complete_gate"), (a.preparation, "preparation"), (a.candidate.is_some(), "candidate")]
}
/// `run_linear_static_preview_observed`'s first statement: one ordinary run.
pub(crate) fn ordinary_run_entered() {
    if let Some(t) = tally() { t.runs.fetch_add(1, SeqCst); }
}
/// G-B fault: before the actual `check_late`, the observer's capacity record exceeds
/// every gate's bound, so the late gate refuses (`LateObservationBytes`).
pub(crate) fn fail_next_late_gate() { arm(|a| a.late_gate = true); }
/// G-C fault: the same, applied after the ordinary run, so G-B has passed and the
/// complete gate refuses (`ObservationBytes`).
pub(crate) fn fail_next_complete_gate() { arm(|a| a.complete_gate = true); }
/// Preparation fault (the private driver's trigger: the closed annulus helper refuses a
/// zero diameter).
pub(crate) fn fail_next_preparation() { arm(|a| a.preparation = true); }
/// Candidate fault: the proof trace faults at this point.
pub(crate) fn fault_next_candidate(fault: crate::retained_receipt::TraceFault) { arm(|a| a.candidate = Some(fault)); }
fn exceed_every_bound(capture: &crate::retained_product::ProductCapture) {
    use crate::retained_product::AdapterEvent;
    let mut counts = capture.adapter.counts.get();
    counts[AdapterEvent::RustCapacityBytes as usize] = u64::MAX / 2;
    capture.adapter.counts.set(counts);
}
/// At G-B, before `check_late` (retained_product.rs, `prepared_case_source`).
pub(crate) fn before_late_gate(capture: &crate::retained_product::ProductCapture) {
    if consume(|a| std::mem::take(&mut a.late_gate)) {
        exceed_every_bound(capture);
    }
}
/// At G-C, after coexistence and G-B's outcome: counted, then the armed observer faults.
pub(crate) fn at_complete_gate(observer: &mut crate::retained_product::ProductCapture) {
    if let Some(t) = tally() { t.complete_gates.fetch_add(1, SeqCst); }
    if consume(|a| std::mem::take(&mut a.complete_gate)) {
        exceed_every_bound(observer);
    }
    if consume(|a| std::mem::take(&mut a.preparation)) {
        observer.facts[0].diameter = 0.0;
    }
    if let Some(fault) = consume(|a| a.candidate.take()) {
        observer.trace_fault = Some(fault);
    }
}
