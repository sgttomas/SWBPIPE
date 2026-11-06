//! The C1 native commands' per-command ledger, end to end (DEL-14-01
//! `desktop-states-runs-comparison.md` §3, repair r4; DEL-04-05
//! `c1-native-custody-plan.md` slice 5; `memory_pool::CommandLedger`).
//!
//! Every command is traced at each of its pending admissions
//! (`memory_pool::ledger_probe`): the command's total with the pending delta,
//! the delta, the captured budget, the reservation's name when it is a named
//! one, and the pool's bytes outside the command's committed total. On small
//! invented fixtures:
//!
//! - **Every command byte is counted.** At every admission the pool holds
//!   exactly the command's committed total above what it held before the
//!   command (the held result H and restored handles stay outside it), and
//!   the command's first admission counts its argument lease (the seed).
//! - **The boundary actually crossed.** A first run finds the step at which
//!   the command's total first reaches its peak; a second, identical run
//!   under a budget one byte lower is refused at that step, by name and with
//!   that total, before the pool is asked and before what it guards is
//!   allocated, with nothing written and every lease ended.
//! - **Budget identity, custody, rollback, unwinding.** A budget changed
//!   mid-command applies to later commands; a reopen's held texts leave the
//!   command's total when the handle is established while their pool bytes
//!   stay (the command's total ends at zero; the handle's bytes remain); a
//!   refusal after the writes rolls the transaction back; a refused or
//!   unwinding reply leaves no handle; unwinding releases every lease.
use crate::analysis_run_v03_build as build;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::{ledger_probe, AdmissionClass, CommandLease, CommandLedger, PressureLevel, DEFAULT_INVOCATION_BUDGET_BYTES};
use crate::native_persistence::tests::{body, cases, held, native_record, native_request, native_save, scratch, store_snapshot, text_of, unlimited, Case};
use crate::native_verdict::tests::gated;
use crate::solve_jobs::{ResultCustody, SolveJobRegistry};
use crate::stored_custody::tests::{file_digest, native_open, native_read, stored_case};
use crate::stored_read::{native_lease_log, STORE_READ_MEMORY_UNAVAILABLE};
use crate::stored_save::tests::{in_place, native_resave};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

/// One pending admission of a command.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Step {
    /// The command's total with the pending delta.
    total: u64,
    pending: u64,
    budget: u64,
    /// The pool's bytes above its bytes before the command, less the
    /// command's committed total: 0 while every command byte is counted.
    outside: i128,
    /// The named reservation's `what` (`NativeLeaseMemory`), or `None` for a typed lease.
    named: Option<String>,
}

struct Traced {
    outcome: String,
    steps: Vec<Step>,
    ledger: Option<Arc<CommandLedger>>,
}

/// Clears the probe even when the command unwinds.
struct ClearProbe;
impl Drop for ClearProbe {
    fn drop(&mut self) {
        ledger_probe::clear();
    }
}

/// Runs `command` traced; `inject(index, ledger)` runs at each pending
/// admission after it is recorded (on this thread, outside every lock).
///
/// Every pool admission or growth this thread makes while the command runs
/// (outside `inject`) must be the delta the command's ledger reserved just
/// before it: a lease of the command that bypassed its ledger fails here.
fn traced(registry: &SolveJobRegistry, mut inject: impl FnMut(usize, &Arc<CommandLedger>) + 'static, command: impl FnOnce() -> String) -> Traced {
    let baseline = registry.pool().reserved_total();
    let steps = Rc::new(RefCell::new(Vec::new()));
    let ledger_seen: Rc<RefCell<Option<Arc<CommandLedger>>>> = Rc::new(RefCell::new(None));
    let mut logged = native_lease_log::last().0;
    // The delta reserved and not yet admitted by the pool; pool admissions outside the ledger.
    let expected: Rc<RefCell<Option<u64>>> = Rc::new(RefCell::new(None));
    // (steps traced when it was admitted, bytes)
    let bypassed: Rc<RefCell<Vec<(usize, u64)>>> = Rc::new(RefCell::new(Vec::new()));
    let injecting = Rc::new(std::cell::Cell::new(false));
    {
        let (expected, bypassed, injecting, steps) = (expected.clone(), bypassed.clone(), injecting.clone(), steps.clone());
        ledger_probe::set_pool(move |bytes| {
            if injecting.get() {
                return;
            }
            if expected.borrow_mut().take() != Some(bytes) {
                let traced: usize = steps.borrow().len();
                bypassed.borrow_mut().push((traced, bytes));
            }
        });
    }
    {
        let (steps, ledger_seen, registry, expected, injecting) = (steps.clone(), ledger_seen.clone(), registry.clone(), expected.clone(), injecting.clone());
        ledger_probe::set(move |ledger, pending| {
            *expected.borrow_mut() = Some(pending);
            let (count, last) = native_lease_log::last();
            let named = (count > logged).then(|| last.map(|(what, _)| what).unwrap_or_default());
            logged = count;
            let committed = ledger.total_bytes() - pending;
            let outside = i128::from(registry.pool().reserved_total()) - i128::from(baseline) - i128::from(committed);
            steps.borrow_mut().push(Step { total: ledger.total_bytes(), pending, budget: ledger.budget_bytes(), outside, named });
            ledger_seen.borrow_mut().get_or_insert_with(|| Arc::clone(ledger));
            let index = steps.borrow().len() - 1;
            injecting.set(true);
            inject(index, ledger);
            injecting.set(false);
        });
    }
    let clear = ClearProbe;
    let outcome = command();
    drop(clear);
    let steps = steps.borrow().clone();
    let ledger = ledger_seen.borrow_mut().take();
    // The one admission before the ledger exists is the argument lease it is
    // seeded with (its bytes known once a step is traced; a command refused at
    // its first growth traces none).
    let mut bypassed = bypassed.borrow().clone();
    let seed = steps.first().map(|step: &Step| step.total - step.pending);
    if let Some(at) = bypassed.iter().position(|&(traced, bytes)| traced == 0 && seed.is_none_or(|seed| seed == bytes)) {
        bypassed.remove(at);
    }
    assert!(bypassed.is_empty(), "pool admissions outside the command's ledger (steps traced, bytes): {bypassed:?} ({outcome})");
    Traced { outcome, steps, ledger }
}

fn outcome(result: Result<Value, String>) -> String {
    match result {
        Ok(value) => value.to_string(),
        Err(text) => text,
    }
}

/// The first step at the command's peak total (every earlier total is below it).
fn crossing(steps: &[Step]) -> usize {
    let peak = steps.iter().map(|step| step.total).max().expect("the command admitted something");
    steps.iter().position(|step| step.total == peak).unwrap()
}

/// `text` carries the refusal of `step`'s admission over `budget`, its total `required`, in its form.
fn assert_over_budget(text: &str, step: &Step, required: u64, budget: u64) {
    match &step.named {
        Some(what) => {
            let expected = format!("{STORE_READ_MEMORY_UNAVAILABLE}: {what} needs {required} bytes in all, above the per-invocation budget of {budget} bytes; nothing more was read");
            assert!(text.contains(&expected), "expected `{expected}` in: {text}");
        }
        None => {
            let expected = format!("IngressExceedsBudget {{ required_bytes: {required}, budget_bytes: {budget} }}");
            assert!(text.contains(&expected), "expected `{expected}` in: {text}");
        }
    }
}

/// Every command byte is counted at every step: the pool's bytes above the
/// command's start are its committed total, plus (after a reopen
/// established its handle) the handle's bytes, `custody`.
fn assert_every_byte_counted(name: &str, steps: &[Step], custody: u64) {
    for (index, step) in steps.iter().enumerate() {
        assert!(step.outside == 0 || step.outside == i128::from(custody), "{name}: step {index} has pool bytes outside the command's total: {step:?}");
    }
}

/// The command's first admission counts its argument lease, seeded once.
fn assert_seeded(name: &str, steps: &[Step], args: &Value) {
    let raw = serde_json::to_vec(args).unwrap().len() as u64;
    assert_eq!(steps[0].total - steps[0].pending, crate::ingress::allocation_envelope(raw).unwrap(), "{name}: the argument lease starts the total");
}

/// The boundary: `first` traced under the configured budget, then `second`
/// (an identical command on an identical store) under a budget one byte
/// below `first`'s peak total. `continues`: the command goes on after a
/// refused reservation (the project open keeps the model editable).
fn boundary(registry: &SolveJobRegistry, name: &str, args: &Value, continues: bool, first: impl FnOnce() -> String, second: impl FnOnce() -> String) -> (Traced, Traced) {
    let (pool_bytes, budget) = registry.pool().settings();
    let a = traced(registry, |_, _| {}, first);
    assert!(a.steps.len() > 3, "{name}: {:?} {}", a.steps, a.outcome);
    assert_seeded(name, &a.steps, args);
    assert!(a.steps.iter().all(|step| step.budget == budget.min(DEFAULT_INVOCATION_BUDGET_BYTES)), "{name}");
    assert_eq!(a.ledger.as_ref().unwrap().total_bytes(), 0, "{name}: the command's total ends at zero");
    let at = crossing(&a.steps);
    let peak = a.steps[at].total;
    assert!(a.steps[..at].iter().all(|step| step.total < peak));
    registry.configure_pool(pool_bytes, peak - 1, false).unwrap();
    let before = registry.pool().reserved_total();
    let b = traced(registry, |_, _| {}, second);
    registry.configure_pool(pool_bytes, budget, false).unwrap();
    if continues {
        assert!(b.steps.len() >= at, "{name}");
    } else {
        assert_eq!(b.steps.len(), at, "{name}: refused at the crossing, before its pool admission: {}", b.outcome);
    }
    for (index, (x, y)) in a.steps.iter().zip(&b.steps).take(at).enumerate() {
        assert_eq!((x.total, x.pending, &x.named), (y.total, y.pending, &y.named), "{name}: step {index} differs");
        assert_eq!(y.budget, peak - 1);
    }
    assert_over_budget(&b.outcome, &a.steps[at], peak, peak - 1);
    if let Some(ledger) = &b.ledger {
        assert_eq!(ledger.total_bytes(), 0, "{name}: the refused command's total ends at zero");
    }
    assert_eq!(registry.pool().reserved_total(), before, "{name}: every lease of the refused command ended");
    (a, b)
}

fn first_case() -> Case {
    cases().into_iter().next().unwrap()
}

fn record_args(registry: &SolveJobRegistry, case: &Case, job: &str, path: &Path) -> Value {
    let saved = native_save(registry, path, &case.save_args(job, "create", Value::Null, Value::Null), &unlimited()).unwrap();
    let mut request = native_request(None);
    request["expected_base"] = saved["project_envelope_hash"]["value"].clone();
    case.record_args(job, request)
}

fn preview_args(case: &Case, job: &str) -> Value {
    json!({
        "jobId": job,
        "inputManifest": case.case["input_manifest"],
        "ruleCheckAggregate": case.case["rule_check_aggregate"],
        "rulePackExclusions": case.case.get("rule_pack_exclusions").cloned().unwrap_or(json!([])),
    })
}

fn preview(registry: &SolveJobRegistry, args: &Value) -> String {
    let (raw, lease) = match build::reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1))) {
        Ok(admitted) => admitted,
        Err(error) => return text_of(error),
    };
    match build::run(registry.clone(), raw, lease) {
        Ok(bytes) => String::from_utf8(bytes).unwrap(),
        Err(error) => text_of(error),
    }
}

#[test]
fn record_run_is_refused_at_its_actual_cumulative_boundary() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-record");
    let case = first_case();
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let (a, b) = (dir.join("a.sqlite"), dir.join("b.sqlite"));
    let args = record_args(&registry, &case, &job, &a);
    assert_eq!(record_args(&registry, &case, &job, &b), args);
    let held_bytes = registry.pool().reserved_total();
    let untouched = store_snapshot(&b);
    let (traced_a, _) = boundary(&registry, "record run", &args, false, || outcome(native_record(&registry, &a, &args, None)), || outcome(native_record(&registry, &b, &args, None)));
    assert!(traced_a.outcome.contains("recording_id"), "{}", traced_a.outcome);
    assert_every_byte_counted("record run", &traced_a.steps, 0);
    assert_eq!(store_snapshot(&b), untouched, "nothing was written");
    assert_eq!(registry.pool().reserved_total(), held_bytes, "only H remains");
    assert!(registry.held_result(&job).is_ok(), "H is still held");
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn save_and_create_project_is_refused_at_its_actual_cumulative_boundary() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-save");
    let case = first_case();
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let (a, b) = (dir.join("a.sqlite"), dir.join("b.sqlite"));
    let args = case.save_args(&job, "create", Value::Null, Value::Null);
    let held_bytes = registry.pool().reserved_total();
    let (traced_a, _) = boundary(&registry, "save", &args, false, || outcome(native_save(&registry, &a, &args, &unlimited())), || outcome(native_save(&registry, &b, &args, &unlimited())));
    assert!(traced_a.outcome.contains("\"saved\""), "{}", traced_a.outcome);
    assert_every_byte_counted("save", &traced_a.steps, 0);
    assert!(!b.exists() || crate::open_project_in_store(b.clone(), None).unwrap().is_none(), "nothing was saved");
    assert_eq!(registry.pool().reserved_total(), held_bytes);
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Two identical stores with the first case saved, the session reset, and
/// each project reopened; returns each store and its re-save arguments.
fn reopened_pair(registry: &SolveJobRegistry, gate: &crate::solve_worker::test_support::Gate, dir: &Path) -> (Case, [(PathBuf, Value); 2]) {
    let case = first_case();
    let job = held(registry, gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let paths = [dir.join("a.sqlite"), dir.join("b.sqlite")];
    for path in &paths {
        native_save(registry, path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap();
    }
    registry.dispose_all_results();
    let [a, b] = paths;
    let opened_a = native_open(registry, &a, Some(case.project_id())).unwrap();
    let opened_b = native_open(registry, &b, Some(case.project_id())).unwrap();
    assert_eq!(opened_a["stored_run"]["state"], "restored");
    (case, [(a, in_place(&opened_a)), (b, in_place(&opened_b))])
}

#[test]
fn a_restored_runs_resave_is_refused_at_its_actual_cumulative_boundary() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-resave");
    let (_, [(a, args_a), (b, args_b)]) = reopened_pair(&registry, &gate, &dir);
    let handles = registry.pool().reserved_total();
    let untouched = store_snapshot(&b);
    let (traced_a, _) = boundary(&registry, "re-save", &args_a, false, || outcome(native_resave(&registry, &a, &args_a, &unlimited())), || outcome(native_resave(&registry, &b, &args_b, &unlimited())));
    assert!(traced_a.outcome.contains("\"restored\""), "{}", traced_a.outcome);
    assert_every_byte_counted("re-save", &traced_a.steps, 0);
    assert_eq!(store_snapshot(&b), untouched, "nothing was written");
    assert_eq!(registry.pool().reserved_total(), handles, "only the handles' bytes remain");
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The reopen keeps the custody bytes: the command's total ends at zero,
/// the handle's bytes stay in the pool, and from its registration on every
/// step has exactly the handle's bytes outside the command's total.
fn assert_custody_handoff(registry: &SolveJobRegistry, name: &str, traced: &Traced, baseline: u64, handle: &str) {
    let custody = registry.pool().reserved_total() - baseline;
    assert!(custody > 0, "{name}: the handle holds its bytes");
    assert_every_byte_counted(name, &traced.steps, custody);
    let reply = traced.steps.last().unwrap();
    assert_eq!(reply.named.as_deref(), Some("the reply"));
    assert_eq!(reply.outside, i128::from(custody), "{name}: at the reply the handle's bytes are out of the command's total and in the pool");
    assert!(traced.steps.iter().any(|step| step.outside == 0 && step.total > custody), "{name}: before registration they were counted");
    assert_eq!(traced.ledger.as_ref().unwrap().total_bytes(), 0, "{name}: the command's total ends at zero");
    assert_eq!(registry.dispose_result(handle).unwrap().released_bytes, custody);
    assert_eq!(registry.pool().reserved_total(), baseline);
}

#[test]
fn the_project_reopen_is_refused_at_its_boundary_and_hands_its_held_texts_over() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-open");
    let case = first_case();
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let (a, b) = (dir.join("a.sqlite"), dir.join("b.sqlite"));
    for path in [&a, &b] {
        native_save(&registry, path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap();
    }
    registry.dispose_all_results();
    let project = case.project_id().to_string();
    let args = json!({"projectId": project});
    let untouched = file_digest(&b);
    let baseline = registry.pool().reserved_total();
    let (traced_a, traced_b) = boundary(&registry, "open", &args, true, || outcome(native_open(&registry, &a, Some(&project))), || outcome(native_open(&registry, &b, Some(&project))));
    let opened: Value = serde_json::from_str(&traced_a.outcome).unwrap();
    // The refused open's run is refused by name; the second open holds nothing.
    assert_eq!(registry.pool_status().retained_results, 1, "only the first open's handle is held: {}", traced_b.outcome);
    assert_custody_handoff(&registry, "open", &traced_a, baseline, opened["stored_run"]["handle"].as_str().unwrap());
    assert_eq!(file_digest(&b), untouched);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn the_recorded_runs_reopen_is_refused_at_its_boundary_and_hands_its_held_texts_over() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-read");
    let (a, b) = (dir.join("a.sqlite"), dir.join("b.sqlite"));
    let (case, recording) = stored_case(&registry, &gate, &a);
    let (_, again) = stored_case(&registry, &gate, &b);
    assert_eq!(recording, again);
    let project = case.project_id().to_string();
    let args = json!({"projectId": project, "recordingId": recording});
    let untouched = file_digest(&b);
    let baseline = registry.pool().reserved_total();
    let (traced_a, _) = boundary(&registry, "read", &args, false, || outcome(native_read(&registry, &a, &project, &recording)), || outcome(native_read(&registry, &b, &project, &recording)));
    let read: Value = serde_json::from_str(&traced_a.outcome).unwrap();
    assert_eq!(registry.pool_status().retained_results, 1);
    assert_custody_handoff(&registry, "read", &traced_a, baseline, read["restored"]["handle"].as_str().unwrap());
    assert_eq!(file_digest(&b), untouched);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn the_analysis_run_preview_is_refused_at_its_actual_cumulative_boundary() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let case = first_case();
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let args = preview_args(&case, &job);
    let held_bytes = registry.pool().reserved_total();
    let (traced_a, _) = boundary(&registry, "preview", &args, false, || preview(&registry, &args), || preview(&registry, &args));
    assert!(traced_a.outcome.contains("analysis_run"), "{}", traced_a.outcome);
    assert_every_byte_counted("preview", &traced_a.steps, 0);
    assert_eq!(registry.pool().reserved_total(), held_bytes);
    registry.dispose_all_results();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A budget changed while a command runs applies to later commands: raising
/// it does not extend the running command, lowering it does not stop it.
#[test]
fn a_budget_changed_mid_command_applies_to_later_commands() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-budget");
    let case = first_case();
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let [a, raised, lowered] = ["a", "raised", "lowered"].map(|name| dir.join(format!("{name}.sqlite")));
    let args = record_args(&registry, &case, &job, &a);
    for path in [&raised, &lowered] {
        assert_eq!(record_args(&registry, &case, &job, path), args);
    }
    let (pool_bytes, budget) = registry.pool().settings();
    let first = traced(&registry, |_, _| {}, || outcome(native_record(&registry, &a, &args, None)));
    let at = crossing(&first.steps);
    let peak = first.steps[at].total;
    // Admitted under peak - 1, raised to the pool at the first admission: still refused at the crossing.
    registry.configure_pool(pool_bytes, peak - 1, false).unwrap();
    let configure = registry.clone();
    let up = traced(&registry, move |index, _| if index == 0 { configure.configure_pool(pool_bytes, pool_bytes, false).unwrap(); }, || outcome(native_record(&registry, &raised, &args, None)));
    assert_eq!(registry.pool().settings().1, pool_bytes, "the raise applied to the pool");
    assert_eq!(up.steps.len(), at);
    assert!(up.steps.iter().all(|step| step.budget == peak - 1));
    assert_over_budget(&up.outcome, &first.steps[at], peak, peak - 1);
    // Admitted under peak, lowered to 64 KiB at the first admission: recorded.
    registry.configure_pool(pool_bytes, peak, false).unwrap();
    let configure = registry.clone();
    let down = traced(&registry, move |index, _| if index == 0 { configure.configure_pool(pool_bytes, 64 * 1024, false).unwrap(); }, || outcome(native_record(&registry, &lowered, &args, None)));
    assert!(down.outcome.contains("recording_id"), "{}", down.outcome);
    assert!(down.steps.iter().all(|step| step.budget == peak) && down.steps.iter().map(|step| step.total).max() == Some(peak));
    // A later command takes the lowered budget.
    let later = traced(&registry, |_, _| {}, || outcome(native_record(&registry, &raised, &args, None)));
    assert!(!later.outcome.contains("recording_id") && later.outcome.contains("65536"), "refused under the lowered budget: {}", later.outcome);
    registry.configure_pool(pool_bytes, budget, false).unwrap();
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The latest step after `write` whose total exceeds the previous step's,
/// and its predecessor: a ballast admitted at the predecessor crosses the
/// budget exactly at it.
fn post_write_crossing(steps: &[Step], write: &str) -> usize {
    let written = steps.iter().position(|step| step.named.as_deref() == Some(write)).unwrap_or_else(|| panic!("no `{write}` step: {steps:?}"));
    (written..steps.len() - 1).rev().find(|&k| steps[k + 1].total > steps[k].total).unwrap_or_else(|| panic!("no rising step after `{write}`: {steps:?}")) + 1
}

/// Refused over the budget after its rows were written (at `crossed`): the
/// transaction rolls back, nothing is stored, every lease ends. A ballast
/// lease in the command's own ledger, admitted at the step before, makes the
/// crossing.
fn refused_after_the_writes(registry: &SolveJobRegistry, name: &str, write: &str, crossed: &str, first: impl FnOnce() -> String, second: impl FnOnce() -> String) -> Traced {
    let (pool_bytes, budget) = registry.pool().settings();
    let a = traced(registry, |_, _| {}, first);
    let peak = a.steps.iter().map(|step| step.total).max().unwrap();
    let at = post_write_crossing(&a.steps, write);
    let ballast_bytes = peak - a.steps[at].total + 1;
    assert!(a.steps[at].named.as_deref().is_some_and(|what| what.starts_with(crossed)), "{name}: crossing at {:?}", a.steps[at].named);
    registry.configure_pool(pool_bytes, peak, false).unwrap();
    let before = registry.pool().reserved_total();
    let ballast: Rc<RefCell<Option<CommandLease>>> = Rc::new(RefCell::new(None));
    let slot = ballast.clone();
    let b = traced(registry, move |index, ledger| if index + 1 == at { *slot.borrow_mut() = Some(ledger.acquire(ballast_bytes, AdmissionClass::Adaptive).unwrap()); }, second);
    registry.configure_pool(pool_bytes, budget, false).unwrap();
    assert!(ballast.borrow().is_some(), "{name}: the ballast was admitted");
    ballast.borrow_mut().take();
    assert_eq!(b.steps.len(), at, "{name}: refused at step {at} ({:?}): {}", a.steps[at].named, b.outcome);
    assert_over_budget(&b.outcome, &a.steps[at], peak + 1, peak);
    assert_eq!(b.ledger.as_ref().unwrap().total_bytes(), 0, "{name}");
    assert_eq!(registry.pool().reserved_total(), before, "{name}: every lease ended");
    b
}

/// A pool refusal of the command's response, after its rows were written:
/// the delta is undone, the transaction rolls back.
fn response_refused_by_the_pool(registry: &SolveJobRegistry, host: &Arc<ScriptedHost>, name: &str, second: impl FnOnce() -> String) {
    let before = registry.pool().reserved_total();
    let host_in = host.clone();
    let mut logged = native_lease_log::last().0;
    let b = traced(registry, move |_, _| {
        // Only a reservation logged at this step (not a previous command's).
        let (count, last) = native_lease_log::last();
        if count > std::mem::replace(&mut logged, count) && last.is_some_and(|(what, _)| what.starts_with("the response to")) {
            host_in.set(|s| s.pressure = PressureLevel::Warning);
        }
    }, second);
    host.set(|s| s.pressure = PressureLevel::Normal);
    assert!(b.outcome.starts_with(STORE_READ_MEMORY_UNAVAILABLE) && b.outcome.contains("the response to") && b.outcome.contains("pressure_elevated"), "{name}: {}", b.outcome);
    assert_eq!(b.ledger.as_ref().unwrap().total_bytes(), 0, "{name}: the refused delta was undone");
    assert_eq!(registry.pool().reserved_total(), before, "{name}: every lease ended");
}

#[test]
fn record_run_and_save_refused_after_their_writes_roll_back() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-rollback");
    let case = first_case();
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let [a, b, c] = ["a", "b", "c"].map(|name| dir.join(format!("record-{name}.sqlite")));
    let args = record_args(&registry, &case, &job, &a);
    for path in [&b, &c] {
        assert_eq!(record_args(&registry, &case, &job, path), args);
    }
    let (untouched_b, untouched_c) = (store_snapshot(&b), store_snapshot(&c));
    refused_after_the_writes(&registry, "record run", "SQLite's copies of the run's row", "the response to", || outcome(native_record(&registry, &a, &args, None)), || outcome(native_record(&registry, &b, &args, None)));
    assert_eq!(store_snapshot(&b), untouched_b, "record run: rolled back");
    response_refused_by_the_pool(&registry, &host, "record run", || outcome(native_record(&registry, &c, &args, None)));
    assert_eq!(store_snapshot(&c), untouched_c, "record run: rolled back");
    let save = case.save_args(&job, "create", Value::Null, Value::Null);
    let [a, b, c] = ["a", "b", "c"].map(|name| dir.join(format!("save-{name}.sqlite")));
    // Since store schema v14 the save writes no search row, so its only
    // reservation after the project row is the response's, admitted once the
    // write's leases ended: the running total never rises after the write,
    // and no budget refusal can follow it (the pool's refusal of the
    // response still can, below).
    let saved = traced(&registry, |_, _| {}, || outcome(native_save(&registry, &a, &save, &unlimited())));
    let written = saved.steps.iter().position(|step| step.named.as_deref() == Some("SQLite's copies of the project's row")).unwrap();
    assert!(saved.steps[written + 1..].iter().all(|step| step.total < saved.steps[written].total), "save: {:?}", saved.steps);
    assert!(saved.steps.last().and_then(|step| step.named.as_deref()).is_some_and(|what| what.starts_with("the response to")), "save: {:?}", saved.steps);
    assert!(crate::open_project_in_store(a.clone(), None).unwrap().is_some(), "save: written");
    assert!(crate::open_project_in_store(b.clone(), None).unwrap().is_none(), "save: nothing written to the unused store");
    response_refused_by_the_pool(&registry, &host, "save", || outcome(native_save(&registry, &c, &save, &unlimited())));
    assert!(crate::open_project_in_store(c.clone(), None).unwrap().is_none(), "save: rolled back");
    assert!(registry.held_result(&job).is_ok());
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A panic inside Record Run's transaction (at its statement's reservation)
/// unwinds: nothing written, every lease and charge released, H held.
#[test]
fn unwinding_inside_record_run_releases_everything_and_writes_nothing() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-unwind");
    let case = first_case();
    let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let path = dir.join("store.sqlite");
    let args = record_args(&registry, &case, &job, &path);
    let untouched = store_snapshot(&path);
    let before = registry.pool().reserved_total();
    let mut logged = native_lease_log::last().0;
    let unwound = traced(&registry, move |_, _| {
        let (count, last) = native_lease_log::last();
        if count > std::mem::replace(&mut logged, count) && last.is_some_and(|(what, _)| what == "SQLite's copies of the run's row") {
            panic!("injected inside the transaction");
        }
    }, || match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| native_record(&registry, &path, &args, None))) {
        Ok(result) => outcome(result),
        Err(_) => "UNWOUND".into(),
    });
    assert_eq!(unwound.outcome, "UNWOUND");
    assert_eq!(unwound.ledger.as_ref().unwrap().total_bytes(), 0);
    assert_eq!(registry.pool().reserved_total(), before);
    assert_eq!(store_snapshot(&path), untouched);
    assert!(registry.held_result(&job).is_ok());
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// After the handle's custody is established, a reply the pool refuses, a
/// panic at the reply's admission, or a session reset before the reply:
/// no handle is left held, every lease and charge ends.
#[test]
fn a_reopen_whose_reply_fails_after_the_handoff_leaves_no_handle() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("ledger-stranded");
    let path = dir.join("store.sqlite");
    let (case, recording) = stored_case(&registry, &gate, &path);
    let project = case.project_id().to_string();
    let baseline = registry.pool().reserved_total();
    assert_eq!(baseline, 0);
    for ending in ["pool", "panic", "reset"] {
        for command in ["open", "read"] {
            let (host_in, registry_in) = (host.clone(), registry.clone());
            let fired = Rc::new(std::cell::Cell::new(false));
            let fired_in = fired.clone();
            let traced = traced(&registry, move |_, _| {
                // The reply's lease once the handle is established (the
                // record header's text, before it, is also "the reply").
                if native_lease_log::last().1.is_some_and(|(what, _)| what == "the reply") && registry_in.pool_status().retained_results == 1 {
                    fired_in.set(true);
                    match ending {
                        "pool" => host_in.set(|s| s.pressure = PressureLevel::Warning),
                        "panic" => panic!("injected at the reply"),
                        _ => drop(registry_in.dispose_all_results()),
                    }
                }
            }, || {
                let run = || if command == "open" { native_open(&registry, &path, Some(&project)) } else { native_read(&registry, &path, &project, &recording) };
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(run)) {
                    Ok(result) => outcome(result),
                    Err(_) => "UNWOUND".into(),
                }
            });
            host.set(|s| s.pressure = PressureLevel::Normal);
            assert!(fired.get(), "{ending} {command}: injected after the handoff");
            match ending {
                "pool" => assert!(traced.outcome.starts_with(STORE_READ_MEMORY_UNAVAILABLE) && traced.outcome.contains("the reply"), "{command}: {}", traced.outcome),
                "panic" => assert_eq!(traced.outcome, "UNWOUND"),
                _ => assert!(traced.outcome.contains("restored-stored-result-"), "{command}: {}", traced.outcome),
            }
            assert_eq!(registry.pool_status().retained_results, 0, "{ending} {command}: no handle is left");
            assert_eq!(traced.ledger.as_ref().unwrap().total_bytes(), 0, "{ending} {command}");
            assert_eq!(registry.pool().reserved_total(), baseline, "{ending} {command}: every lease ended");
        }
    }
    std::fs::remove_dir_all(&dir).unwrap();
}
