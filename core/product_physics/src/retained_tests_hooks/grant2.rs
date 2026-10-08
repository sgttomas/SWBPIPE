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

/// One invocation's counts, shared with the reserved-stack thread, and (RV107 A1-N-1) a copy
/// of the successor its precommit received.
#[derive(Debug, Default)]
pub(crate) struct Tally { runs: AtomicUsize, complete_gates: AtomicUsize, successor: std::sync::Mutex<Option<serde_json::Value>> }
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
    let (value, counts, _) = counted_with_successor(f);
    (value, counts)
}
/// B1 SP (RV107 A1-N-1): `counted`, also returning a copy of the serialized successor that the
/// invocation's precommit received (taken before any precommit fault), if it reached precommit.
/// The copy is shared across the reserved-stack hop with the tally.
pub(crate) fn counted_with_successor<T>(f: impl FnOnce() -> T) -> (T, Counts, Option<serde_json::Value>) {
    let fresh = Arc::new(Tally::default());
    assert!(TALLY.with(|t| t.replace(Some(fresh.clone()))).is_none(), "one tally at a time");
    let value = f();
    TALLY.with(|t| t.borrow_mut().take());
    let successor = fresh.successor.lock().unwrap().take();
    (value, Counts { runs: fresh.runs.load(SeqCst), complete_gates: fresh.complete_gates.load(SeqCst) }, successor)
}
/// At precommit (`lib.rs`, `before_precommit`): the successor's copy, when a tally is installed.
pub(crate) fn capture_successor(successor: &serde_json::Value) {
    if let Some(t) = tally() { *t.successor.lock().unwrap() = Some(successor.clone()); }
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
    a.preparation_of_case = a.preparation_of_case.or(faults.preparation_of_case);
    a.exact_capture |= faults.exact_capture;
    a.section_staging |= faults.section_staging;
    a.operand_preparation_of_case = a.operand_preparation_of_case.or(faults.operand_preparation_of_case);
    a.combination_call = a.combination_call.or(faults.combination_call);
    a.combination_freeze |= faults.combination_freeze;
    a.freeze_of_case = a.freeze_of_case.or(faults.freeze_of_case);
}
/// `armed_names`: this grant's faults.
pub(super) fn names(a: &Armed) -> [(bool, &'static str); 11] {
    [(a.late_gate, "late_gate"), (a.complete_gate, "complete_gate"), (a.preparation, "preparation"), (a.candidate.is_some(), "candidate"),
        (a.preparation_of_case.is_some(), "preparation_of_case"), (a.exact_capture, "exact_capture"), (a.section_staging, "section_staging"),
        (a.operand_preparation_of_case.is_some(), "operand_preparation_of_case"), (a.combination_call.is_some(), "combination_call"),
        (a.combination_freeze, "combination_freeze"), (a.freeze_of_case.is_some(), "freeze_of_case")]
}
/// RV123 S-2: the freeze of case `index` (request index) refuses at its maxima stage (the
/// existing `TraceFault::Maxima`, for that case only), after its selected Run: the case is
/// `unavailable` (`facade_certificate`) with its prepared source, and the other cases go on.
pub(crate) fn fail_freeze_of_case(index: usize) { arm(|a| a.freeze_of_case = Some(index)); }
/// At each case freeze (`PreparedCases::freeze`): whether the armed fault fires for it (consumed).
pub(crate) fn freeze_fault_of_case(request: usize) -> bool {
    consume(|a| if a.freeze_of_case == Some(request) { a.freeze_of_case.take() } else { None }).is_some()
}
/// B2-P hook (PLAN §1.2.4; B2-C §2.6): the operand preparation of `not_required` case `index`
/// (request index) refuses, through the real section preparation (its first member's diameter
/// is 0, as `fail_preparation_of_case` does for a case attempt). Each combination that needs it
/// becomes `retained_unavailable` with `operand_preparation_failure`.
pub(crate) fn fail_operand_preparation(index: usize) { arm(|a| a.operand_preparation_of_case = Some(index)); }
/// At each operand preparation, before C3's preparation stage, on the owner in the capture's fields.
pub(crate) fn before_operand_preparation(capture: &mut crate::retained_product::ProductCapture, owner: usize) {
    if consume(|a| if a.operand_preparation_of_case == Some(owner) { a.operand_preparation_of_case.take() } else { None }).is_some() {
        capture.facts[0].diameter = 0.0;
    }
}
/// B2-P hook: the Call of combination `index` (authored index) refuses before any source, through
/// the kernel's own operand validation (the Call is made with no operand: `no_operands`), so the
/// combination is `retained_unavailable` with that `pre_source_refusal`.
pub(crate) fn fail_combination_call(index: usize) { arm(|a| a.combination_call = Some(index)); }
/// At a combination's Call: whether the armed Call fault fires for it (consumed).
pub(crate) fn combination_call_fault(index: usize) -> bool {
    consume(|a| if a.combination_call == Some(index) { a.combination_call.take() } else { None }).is_some()
}
/// B2-P hook: the next combination freeze refuses at its observables stage (`facade_certificate`).
pub(crate) fn fault_next_combination_freeze() { arm(|a| a.combination_freeze = true); }
/// At a combination's observables stage: whether the armed freeze fault fires (consumed).
pub(crate) fn combination_freeze_fault() -> bool { consume(|a| std::mem::take(&mut a.combination_freeze)) }
/// B3b-P (B3-D P-12): the exact route's material capture refuses: its Ĝ check (retained_product.rs
/// `exact_material_nu`) sees the represented Ĝ one ulp up, so the capture records a typed
/// association error and W1 falls back at preparation (custody).
pub(crate) fn fault_next_exact_capture() { arm(|a| a.exact_capture = true); }
/// At the exact route's Ĝ check: whether the armed exact-capture fault fires here (consumed).
pub(crate) fn exact_capture_fault() -> bool { consume(|a| std::mem::take(&mut a.exact_capture)) }
/// B3b-P (B3-D P-12): an evidence-overlay fault: the next frozen exact case's first section patch
/// names an index past its `pipe_sections`, so staging refuses (`StagingFault("pipe_sections[]")`).
pub(crate) fn break_next_section_overlay() { arm(|a| a.section_staging = true); }
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
/// zero diameter). It is applied at G-C to the case in the capture's own fields: at c ≥ 2 the
/// last requested case (RV109 R3P-6).
pub(crate) fn fail_next_preparation() { arm(|a| a.preparation = true); }
/// Candidate fault: the proof trace faults at this point.
pub(crate) fn fault_next_candidate(fault: crate::retained_receipt::TraceFault) { arm(|a| a.candidate = Some(fault)); }
/// B1 SP (decision 23; RV107 A1-N-6): the preparation of request case `index` fails (the closed
/// annulus helper refuses a zero diameter), on the private driver and the actual entry alike. Its
/// attempt alone fails; the other cases' attempts continue (DESIGN_v2 T-7). `fail_next_preparation`
/// would fail the last requested case instead, the one in the capture's own fields at G-C.
pub(crate) fn fail_preparation_of_case(index: usize) { arm(|a| a.preparation_of_case = Some(index)); }
/// At each product attempt, before its preparation (retained_product.rs, `prepare_attempt`), on the
/// case in the capture's own fields.
pub(crate) fn before_case_preparation(capture: &mut crate::retained_product::ProductCapture, request: usize) {
    if consume(|a| if a.preparation_of_case == Some(request) { a.preparation_of_case.take() } else { None }).is_some() {
        capture.facts[0].diameter = 0.0;
    }
}
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
