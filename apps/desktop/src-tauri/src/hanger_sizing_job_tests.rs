//! Hanger sizing's connected path (DEL-04-11 increment 3c part 2): `size_hangers`' ingress and ledger, task 4 in a
//! real worker process (this test binary run as the worker, `solve_worker::test_support::TestLauncher`), and the reply,
//! on the committed `fixtures/results/hanger_sizing_input_two_span.json`; then the reply limit's boundary, admission
//! and ledger refusals, retirement and the dispatch ticket (the delayed-registration race, ticket validation, keyed
//! idempotency, exhaustion and the session reset).

use super::*;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::solve_worker::test_support::{Gate, TestLauncher};
use crate::solve_worker::{WorkerLauncher, PROCESS_RESERVE_BYTES};
use open_pipe_stress_hanger_sizing::desktop::size_and_read;
use open_pipe_stress_hanger_sizing::publish::checked_sha256;
use serde_json::json;

const TWO_SPAN: &str = include_str!("../../../../fixtures/results/hanger_sizing_input_two_span.json");
const TWO_SPAN_REPLY: &str = include_str!("../../../../fixtures/results/hanger_sizing_reply_two_span.json");
const TWO_SPAN_DOCUMENT: &str = include_str!("../../../../fixtures/results/hanger_sizing_result_two_span.json");
const GIB: u64 = 1 << 30;
const MIB: u64 = 1 << 20;

// -- Helpers --------------------------------------------------------------------------------------------------------

fn registry_with(launcher: Arc<dyn WorkerLauncher>) -> SolveJobRegistry {
    SolveJobRegistry::with_hooks(
        MemoryPool::with_wait_tick(ScriptedHost::normal(), Duration::from_millis(5)),
        launcher,
        Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)),
    )
}

fn page(n: usize) -> String {
    format!("{n:032x}")
}

fn id(page_n: usize, sequence: u64) -> RetirementId {
    RetirementId { page: page(page_n), sequence }
}

/// A start's body with its ticket.
fn body(ticket: u64, input: &str) -> Value {
    json!({ "expectedEpoch": ticket, "inputText": input })
}

/// The webview's retirement before a start (`cancel_hanger_sizing` from `page_n`'s `sequence`): its acknowledged
/// epoch, the start's ticket.
fn retire(registry: &SolveJobRegistry, page_n: usize, sequence: u64) -> u64 {
    let retirement = registry.hanger_sizing_runs().retire_keyed(id(page_n, sequence), registry.pool());
    assert!(!retirement.duplicate && !retirement.exhausted, "{retirement:?}");
    retirement.epoch
}

/// A run registered directly at the current epoch (tests of `run` itself).
fn register_now(registry: &SolveJobRegistry) -> Registration {
    let runs = registry.hanger_sizing_runs();
    runs.register(runs.entry_now(), registry.pool()).unwrap()
}

fn raw_body(body: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(body).unwrap())
}

/// `size_hangers` through its executor and blocking parts, as `lib.rs` runs them: the reply text, or the error's
/// serialized form (a plain string, or the typed ingress object).
fn size_via_command(registry: &SolveJobRegistry, body: &Value) -> Result<String, String> {
    size_via_command_waiting(registry, body, START_WAIT)
}

fn size_via_command_waiting(registry: &SolveJobRegistry, body: &Value, wait: Duration) -> Result<String, String> {
    let error = |error: CommandError<String>| serde_json::to_string(&error).unwrap();
    let (raw, copy) = reserve(registry, &raw_body(body), &headers(Some(INGRESS_ARGS_JSON_V1))).map_err(|e| error(CommandError::Ingress(e)))?;
    size_command(registry, raw, copy, wait, |text| text).map_err(error)
}

/// The command error's text: a plain string's content.
fn code_of(error: &str) -> String {
    let text: String = serde_json::from_str(error).unwrap_or_else(|_| error.to_string());
    text.split_once(": ").map(|(code, _)| code.to_string()).unwrap_or(text)
}

fn seeded(registry: &SolveJobRegistry, bytes: u64) -> (NativeLeaseMemory<'_>, CommandLease) {
    NativeLeaseMemory::seeded(registry, registry.acquire_interactive_ingress_capped(bytes).unwrap())
}

/// Nothing the run took stays reserved, and no worker is left.
fn assert_released(registry: &SolveJobRegistry) {
    assert_eq!(registry.pool_status().reserved_bytes, 0, "every lease is released");
    assert_eq!(registry.pool().running_workers(), 0, "every worker is reaped");
}

fn worker_alive(pid: i32) -> bool {
    std::process::Command::new("/bin/kill")
        .args(["-0", &pid.to_string()])
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// Waits until the pool records `matches` as its last refusal (a run is in the queue wait).
fn await_queued(registry: &SolveJobRegistry, matches: impl Fn(&ResourceRefusal) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !registry.pool_status().last_refusal.as_ref().is_some_and(&matches) {
        assert!(Instant::now() < deadline, "the run never waited");
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// Cancels the current run if a test's own thread panics inside a scope, so its worker ends and the scope's join
/// returns.
struct CancelOnUnwind<'a>(&'a SolveJobRegistry);

impl Drop for CancelOnUnwind<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.0.hanger_sizing_runs().cancel_current(self.0.pool());
        }
    }
}

// -- The connected path ---------------------------------------------------------------------------------------------

/// `size_hangers` end to end on the committed two-span input: the raw body's ingress and ledger, task 4 in a real
/// worker process, and the reply. The reply is byte for byte the entry's own for the same text; its evidence carries
/// the checked canonical hash of its document and reproduction `not_checked`; everything is released after.
#[test]
fn size_hangers_runs_task_4_in_the_capped_worker_and_replies_the_entrys_document_and_evidence() {
    let registry = registry_with(TestLauncher::solve());
    let ticket = retire(&registry, 1, 1);
    let reply = size_via_command(&registry, &body(ticket, TWO_SPAN)).unwrap();
    assert_eq!(reply, size_and_read(TWO_SPAN.as_bytes()).unwrap().reply(), "the worker's reply is the entry's");
    // The committed reply the webview's tests read (`hangerSizingService.test.ts`) is this path's own, byte for byte.
    assert_eq!(reply, TWO_SPAN_REPLY, "fixtures/results/hanger_sizing_reply_two_span.json is the worker's reply");
    let reply: Value = serde_json::from_str(&reply).unwrap();
    assert_eq!(reply["document"]["status"], "sized");
    assert_eq!(reply["document"], serde_json::from_str::<Value>(TWO_SPAN_DOCUMENT).unwrap(), "the document is the headless one");
    assert_eq!(reply["evidence"]["document_sha256"], checked_sha256(&reply["document"]).unwrap());
    assert_eq!(reply["evidence"]["verdict"]["reproduction"]["status"], "not_checked");
    for standing in ["closure", "binding", "arithmetic_consistent_with_published_positions"] {
        assert_eq!(reply["evidence"]["verdict"][standing]["status"], "passed", "{standing}");
    }
    assert_eq!(registry.hanger_sizing_runs().current_ticket(), None, "the slot is clear once the run has ended");
    assert_released(&registry);
}

/// Task 4's own refusal is the command's error with its code: a non-UTF-8 input text cannot be sent as a JSON string,
/// so the input bound stands for it (a text one byte above the entry's limit, refused before any worker starts).
#[test]
fn task_4s_refusal_keeps_its_code_and_starts_no_worker_above_the_input_limit() {
    let gate = Gate::new("sizing-input-limit");
    let registry = registry_with(gate.launcher());
    let (_memory, args) = seeded(&registry, 3 * MIB);
    let registration = register_now(&registry);
    let input = " ".repeat(MAX_INPUT_BYTES + 1);
    let Err(failure) = run(&registry, args.ledger(), &registration, input, START_WAIT) else { panic!("not refused") };
    let CommandError::Command(text) = failure.into_command_error() else { panic!("not a command error") };
    assert!(text.starts_with("HANGER_SIZING_INPUT_TOO_LARGE: "), "{text}");
    assert_eq!(gate.started_count(), 0, "no worker started");
    assert_eq!(args.ledger().total_bytes(), args.bytes());
    drop(registration);
    drop(args);
    assert_released(&registry);
}

// -- The reply limit ------------------------------------------------------------------------------------------------

/// The limit's boundary on the complete reply, with a scripted worker: a reply of exactly 64 MiB is read and returned
/// whole; one byte more ends the run `HANGER-SIZING-REPLY-TOO-LARGE` on the announced length, before any result lease
/// is admitted or any byte read, with its worker reaped and everything released.
#[test]
fn the_reply_limit_admits_64_mib_and_refuses_one_byte_more_before_reading_it() {
    assert_eq!(REPLY_LIMIT_BYTES, 64 * MIB);
    for (length, admitted) in [(REPLY_LIMIT_BYTES, true), (REPLY_LIMIT_BYTES + 1, false)] {
        let registry = registry_with(TestLauncher::scripted(format!("result:{length}")));
        let (_memory, args) = seeded(&registry, 3 * MIB);
        let registration = register_now(&registry);
        let outcome = run(&registry, args.ledger(), &registration, "{}".to_string(), START_WAIT);
        if admitted {
            let SizingReply { reply, hold } = outcome.unwrap();
            assert_eq!(reply.len() as u64, length);
            assert!(hold.lease_bytes() <= RESULT_LEASE_BYTES);
            assert_eq!(args.ledger().total_bytes(), args.bytes() + hold.lease_bytes(), "the bound shrank to the result lease");
        } else {
            let Err(SizingFailure::ReplyTooLarge { reply_bytes, limit_bytes }) = outcome else { panic!("{outcome:?}") };
            assert_eq!((reply_bytes, limit_bytes), (length, REPLY_LIMIT_BYTES));
            assert_eq!(registry.pool().running_workers(), 0, "the worker was reaped");
            assert_eq!(args.ledger().total_bytes(), args.bytes(), "no result lease is counted");
            let CommandError::Command(text) = SizingFailure::ReplyTooLarge { reply_bytes, limit_bytes }.into_command_error() else { panic!() };
            assert!(text.starts_with(&format!("{REPLY_TOO_LARGE}: the sizing reply of {length} bytes")), "{text}");
        }
        drop(registration);
        drop(args);
        assert_released(&registry);
    }
}

// -- Admission and the ledger ---------------------------------------------------------------------------------------

/// The command's admission refusals: no ingress header, a JSON body (what Tauri's `postMessage` fallback makes of a raw
/// body: a JSON array of numbers, or any JSON value), arguments that are not an object, and a missing input text.
/// Each starts no worker and holds nothing.
#[test]
fn the_command_refuses_each_inadmissible_request_before_any_worker() {
    let gate = Gate::new("sizing-admission");
    let registry = registry_with(gate.launcher());
    let good = body(0, "{}");
    let no_header = reserve(&registry, &raw_body(&good), &headers(None)).unwrap_err();
    assert!(format!("{no_header:?}").contains("INGRESS-HEADER-REQUIRED"), "{no_header:?}");
    let fallback = json!(serde_json::to_vec(&good).unwrap());
    for json_body in [InvokeBody::Json(good.clone()), InvokeBody::Json(fallback)] {
        let refused = reserve(&registry, &json_body, &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap_err();
        assert!(format!("{refused:?}").contains("INGRESS-RAW-BODY-REQUIRED"), "{refused:?}");
    }
    let not_object = size_via_command(&registry, &json!([1])).unwrap_err();
    assert!(not_object.contains("INGRESS-ARGS-NOT-OBJECT"), "{not_object}");
    for input in [json!(null), json!({"request": {}})] {
        let refused = size_via_command(&registry, &json!({"expectedEpoch": 0, "inputText": input})).unwrap_err();
        assert_eq!(code_of(&refused), ARGS_INVALID, "{refused}");
    }
    assert_eq!(gate.started_count(), 0);
    assert_released(&registry);
}

/// While the worker runs, the command's ledger holds its whole captured budget (the argument lease, and the bound
/// for the request copy, the grant up to its maximum and the result lease) and the pool holds no more than the
/// ledger counts; once the worker is reaped the bound is the result lease's alone, and after the response the
/// argument lease's.
#[test]
fn the_argument_grant_and_result_leases_are_counted_in_one_ledger() {
    let gate = Gate::new("sizing-ledger");
    let registry = registry_with(gate.launcher());
    let (_memory, args) = seeded(&registry, 3 * MIB);
    let ledger = Arc::clone(args.ledger());
    let registration = register_now(&registry);
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&registry);
        let running = scope.spawn(|| run(&registry, &ledger, &registration, TWO_SPAN.to_string(), START_WAIT));
        gate.wait_started(1);
        assert_eq!(ledger.total_bytes(), ledger.budget_bytes(), "the bound takes the rest of the captured budget");
        assert!(registry.pool_status().reserved_bytes <= ledger.total_bytes(), "the pool holds no more than the ledger counts");
        gate.release(r#"{"document":{},"evidence":{}}"#);
        let SizingReply { reply, hold } = running.join().unwrap().unwrap();
        assert_eq!(reply, r#"{"document":{},"evidence":{}}"#);
        assert_eq!(ledger.total_bytes(), args.bytes() + hold.lease_bytes(), "the bound shrank to the result lease");
        assert_eq!(registry.pool_status().reserved_bytes, args.bytes() + hold.lease_bytes());
        drop(hold);
        assert_eq!(ledger.total_bytes(), args.bytes());
    });
    drop(registration);
    drop(args);
    assert_released(&registry);
}

/// The admission's exact boundary under the captured budget: the argument lease, the request copy, the largest
/// result lease and the starting grant one byte above it are refused by name (`ingress_exceeds_budget`) before any
/// worker starts, with the ledger as before; at the budget the worker starts. A budget raised after the command was
/// admitted does not extend it.
#[test]
fn a_run_one_byte_above_the_captured_budget_is_refused_before_any_worker_and_one_at_it_starts() {
    let gate = Gate::new("sizing-boundary");
    let argument = 3 * MIB;
    let input = TWO_SPAN.to_string();
    let len = input.len() as u64;
    let start = initial_grant(PROCESS_RESERVE_BYTES, resident_envelope(len)).unwrap();
    let required = argument + allocation_envelope(len).unwrap() + RESULT_LEASE_BYTES + start;
    for (budget, admitted) in [(required - 1, false), (required, true)] {
        let registry = registry_with(gate.launcher());
        registry.configure_pool(12 * GIB, budget, false).unwrap();
        let (_memory, args) = seeded(&registry, argument);
        let ledger = Arc::clone(args.ledger());
        // Raised after admission: the ledger keeps the budget it captured.
        registry.configure_pool(12 * GIB, 4 * GIB, false).unwrap();
        assert_eq!(ledger.budget_bytes(), budget);
        let started = gate.started_count();
        let registration = register_now(&registry);
        std::thread::scope(|scope| {
            let _guard = CancelOnUnwind(&registry);
            let running = scope.spawn(|| run(&registry, &ledger, &registration, input.clone(), START_WAIT));
            if admitted {
                gate.wait_started(started + 1);
                assert_eq!(ledger.total_bytes(), budget);
                assert!(registry.hanger_sizing_runs().cancel_current(registry.pool()));
                assert!(matches!(running.join().unwrap(), Err(SizingFailure::Cancelled)));
            } else {
                let Err(SizingFailure::Memory(SolveJobError::ResourceUnavailable { refusal, .. })) = running.join().unwrap() else { panic!("not refused") };
                assert_eq!(refusal, ResourceRefusal::IngressExceedsBudget { required_bytes: required, budget_bytes: budget });
                assert_eq!(gate.started_count(), started, "no worker started");
            }
        });
        assert_eq!(ledger.total_bytes(), argument);
        drop(registration);
        drop(args);
        assert_released(&registry);
    }
}

// -- Retirement and the dispatch ticket -----------------------------------------------------------------------------

/// A newer request's retirement ends one waiting for its starting grant (its wait ends at once, holding nothing it
/// took), and a cancel ends the newer; a wait that reaches its deadline is refused by the last queue reason as a typed
/// memory refusal.
#[test]
fn a_queued_run_is_retired_by_a_newer_request_or_a_cancel_or_refused_at_its_deadline() {
    let gate = Gate::new("sizing-queue");
    let registry = registry_with(gate.launcher());
    registry.pool().set_max_running_workers(0);
    let first = body(retire(&registry, 1, 1), "{}");
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&registry);
        let began = Instant::now();
        let older = scope.spawn(|| size_via_command(&registry, &first));
        await_queued(&registry, |refusal| matches!(refusal, ResourceRefusal::WorkerLimitReached { .. }));
        let second = body(retire(&registry, 1, 2), "{}");
        assert_eq!(code_of(&older.join().unwrap().unwrap_err()), CANCELLED);
        assert!(began.elapsed() < START_WAIT, "the older wait ended at the retirement");
        let newer = scope.spawn(|| size_via_command(&registry, &second));
        while registry.hanger_sizing_runs().current_ticket().is_none() {
            std::thread::sleep(Duration::from_millis(2));
        }
        let receipt = registry.hanger_sizing_runs().retire_keyed(id(1, 3), registry.pool());
        assert!(receipt.cancelled && !receipt.duplicate);
        assert_eq!(code_of(&newer.join().unwrap().unwrap_err()), CANCELLED);
    });
    assert_eq!(registry.hanger_sizing_runs().current_ticket(), None);
    assert_released(&registry);
    // The deadline: the queue reason stands as the typed refusal.
    let refused = size_via_command_waiting(&registry, &body(retire(&registry, 1, 4), "{}"), Duration::from_millis(30)).unwrap_err();
    assert!(refused.contains("worker_limit_reached"), "{refused}");
    assert_eq!(gate.started_count(), 0);
    assert_released(&registry);
}

/// A newer request's retirement stops the running one: its worker is killed and reaped, its grant released, and it
/// replies `HANGER-SIZING-CANCELLED`; the newer one completes with its own reply.
#[test]
fn a_newer_request_kills_the_running_worker_and_completes() {
    let gate = Gate::new("sizing-supersede");
    let registry = registry_with(gate.launcher());
    let first = body(retire(&registry, 1, 1), "{}");
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&registry);
        let older = scope.spawn(|| size_via_command(&registry, &first));
        gate.wait_started(1);
        let worker = gate.pids()[0];
        let second = body(retire(&registry, 1, 2), "{}");
        assert_eq!(code_of(&older.join().unwrap().unwrap_err()), CANCELLED);
        assert!(!worker_alive(worker), "the retired worker was killed and reaped");
        let newer = scope.spawn(|| size_via_command(&registry, &second));
        gate.wait_started(2);
        gate.release(r#"{"newer":true}"#);
        assert_eq!(newer.join().unwrap().unwrap(), r#"{"newer":true}"#);
    });
    assert_released(&registry);
}

/// Repair A's race. Page A's start passes its entry with a current ticket and is paused before it registers. The
/// session reset and page B's keyed retirement (B's mount, or its start's own) then move the epoch on, and B's start
/// becomes current with a worker running. A resumes: it is refused `HANGER-SIZING-REQUEST-STALE` at registration,
/// starts no worker, and neither replaces nor cancels B, whose run completes with its own reply.
#[test]
fn a_start_delayed_before_registration_never_supersedes_a_run_started_after_a_later_retirement() {
    let gate = Gate::new("sizing-delayed-registration");
    let registry = registry_with(gate.launcher());
    let ticket_a = retire(&registry, 1, 1);
    let start_a = body(ticket_a, "{}");
    let (paused_tx, paused_rx) = std::sync::mpsc::channel::<()>();
    let (resume_tx, resume_rx) = std::sync::mpsc::channel::<()>();
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&registry);
        let a = scope.spawn(|| {
            BEFORE_REGISTRATION.with(|hook| {
                *hook.borrow_mut() = Some(Box::new(move || {
                    paused_tx.send(()).unwrap();
                    resume_rx.recv().unwrap();
                }))
            });
            size_via_command(&registry, &start_a)
        });
        paused_rx.recv().unwrap();
        // A has passed its entry and is paused before registering. The session reset, then page B's retirement.
        registry.dispose_all_results();
        let ticket_b = retire(&registry, 2, 1);
        assert_eq!(ticket_b, ticket_a + 2, "both retirements moved the epoch on");
        let start_b = body(ticket_b, "{}");
        let b = scope.spawn(|| size_via_command(&registry, &start_b));
        gate.wait_started(1);
        let worker_b = gate.pids()[0];
        assert_eq!(registry.hanger_sizing_runs().current_ticket(), Some(ticket_b));
        resume_tx.send(()).unwrap();
        let refused = a.join().unwrap().unwrap_err();
        assert_eq!(code_of(&refused), REQUEST_STALE, "{refused}");
        assert_eq!(registry.hanger_sizing_runs().current_ticket(), Some(ticket_b), "A replaced nothing");
        assert!(worker_alive(worker_b), "A cancelled nothing");
        assert_eq!(gate.started_count(), 1, "A started no worker");
        gate.release(r#"{"b":true}"#);
        assert_eq!(b.join().unwrap().unwrap(), r#"{"b":true}"#, "B's run completes");
    });
    assert_eq!(gate.started_count(), 1);
    assert_released(&registry);
}

/// The ticket at entry: missing, malformed, future and stale tickets, and a second start under a ticket a run has
/// used, are each refused by name before any worker starts, and none of them touches the current run.
#[test]
fn missing_malformed_future_stale_and_used_tickets_are_refused_before_any_worker() {
    let gate = Gate::new("sizing-tickets");
    let registry = registry_with(gate.launcher());
    retire(&registry, 1, 1);
    let ticket = retire(&registry, 1, 2);
    assert_eq!(ticket, 2);
    let cases: [(Value, &str); 7] = [
        (json!({"inputText": "{}"}), TICKET_INVALID),
        (json!({"expectedEpoch": null, "inputText": "{}"}), TICKET_INVALID),
        (json!({"expectedEpoch": "2", "inputText": "{}"}), TICKET_INVALID),
        (json!({"expectedEpoch": -1, "inputText": "{}"}), TICKET_INVALID),
        (json!({"expectedEpoch": 2.5, "inputText": "{}"}), TICKET_INVALID),
        (body(ticket + 1, "{}"), TICKET_INVALID),
        (body(ticket - 1, "{}"), REQUEST_STALE),
    ];
    for (request, code) in &cases {
        let refused = size_via_command(&registry, request).unwrap_err();
        assert_eq!(code_of(&refused), *code, "{request}: {refused}");
    }
    assert_eq!(gate.started_count(), 0, "no refused ticket started a worker");
    // A start under the current ticket runs; a second delivery of it (or any start under the same ticket) is stale.
    let current = body(ticket, "{}");
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&registry);
        let running = scope.spawn(|| size_via_command(&registry, &current));
        gate.wait_started(1);
        assert_eq!(code_of(&size_via_command(&registry, &current).unwrap_err()), REQUEST_STALE);
        for (request, _) in &cases {
            size_via_command(&registry, request).unwrap_err();
        }
        assert_eq!(registry.hanger_sizing_runs().current_ticket(), Some(ticket), "the current run is untouched");
        assert_eq!(gate.started_count(), 1);
        gate.release("{}");
        assert_eq!(running.join().unwrap().unwrap(), "{}");
    });
    assert_eq!(code_of(&size_via_command(&registry, &current).unwrap_err()), REQUEST_STALE, "a used ticket stays used");
    assert_eq!(gate.started_count(), 1);
    assert_released(&registry);
}

/// The session reset (`dispose_all_results`) retires a running sizing beside the model check, in its own slot: the
/// model check's cancel leaves the sizing alone, and the reset moves the epoch on, so a start under the ticket before
/// it is stale. A keyed retirement (the person's cancel or the form's unmount) stops a running worker too.
#[test]
fn the_session_reset_and_a_keyed_retirement_cancel_a_running_sizing() {
    let gate = Gate::new("sizing-reset");
    let registry = registry_with(gate.launcher());
    for (sequence, retirement) in [(1u64, "reset"), (3, "keyed")] {
        let ticket = retire(&registry, 1, sequence);
        let request = body(ticket, "{}");
        std::thread::scope(|scope| {
            let _guard = CancelOnUnwind(&registry);
            let running = scope.spawn(|| size_via_command(&registry, &request));
            gate.wait_started(sequence.div_ceil(2) as usize);
            let worker = gate.pids().into_iter().find(|pid| worker_alive(*pid)).unwrap();
            assert!(!registry.cancel_model_checks(), "the model check's slot holds no sizing");
            assert_eq!(registry.hanger_sizing_runs().current_ticket(), Some(ticket));
            match retirement {
                "reset" => {
                    registry.dispose_all_results();
                }
                _ => assert!(registry.hanger_sizing_runs().retire_keyed(id(1, sequence + 1), registry.pool()).cancelled),
            }
            assert_eq!(registry.hanger_sizing_runs().epoch(), ticket + 1);
            assert_eq!(code_of(&running.join().unwrap().unwrap_err()), CANCELLED);
            assert!(!worker_alive(worker));
        });
        assert_eq!(code_of(&size_via_command(&registry, &request).unwrap_err()), REQUEST_STALE);
        assert_released(&registry);
    }
}

/// Keyed retirement is idempotent: a repeated or older delivery cancels nothing, moves nothing on and says
/// `duplicate` with the current epoch, and never reaches the run in progress; a malformed id retires nothing. The
/// reply is JSON-compatible metadata (Tauri's JSON arguments in, a plain object out).
#[test]
fn a_repeated_or_older_retirement_is_a_duplicate_that_reaches_nothing() {
    let gate = Gate::new("sizing-keyed");
    let registry = registry_with(gate.launcher());
    let runs = registry.hanger_sizing_runs();
    let first = runs.retire_keyed(id(1, 1), registry.pool());
    assert_eq!(first, HangerSizingRetirement { cancelled: false, epoch: 1, retirement_id: id(1, 1), duplicate: false, exhausted: false });
    assert_eq!(serde_json::to_value(&first).unwrap(), json!({"cancelled": false, "epoch": 1, "retirement_id": {"page": page(1), "sequence": 1}}));
    let ticket = retire(&registry, 1, 2);
    let request = body(ticket, "{}");
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&registry);
        let running = scope.spawn(|| size_via_command(&registry, &request));
        gate.wait_started(1);
        for sequence in [2, 2, 1] {
            let again = cancel_command(&registry, Some(json!({"page": page(1), "sequence": sequence}))).unwrap();
            assert_eq!(again, HangerSizingRetirement { cancelled: false, epoch: ticket, retirement_id: id(1, sequence), duplicate: true, exhausted: false });
            assert_eq!(serde_json::to_value(&again).unwrap()["duplicate"], true);
        }
        assert_eq!(runs.current_ticket(), Some(ticket), "a duplicate reaches nothing");
        assert_eq!(runs.epoch(), ticket);
        gate.release("{}");
        assert_eq!(running.join().unwrap().unwrap(), "{}");
    });
    for malformed in [None, Some(json!({"page": "x", "sequence": 1})), Some(json!({"page": page(1), "sequence": 0})), Some(json!({"page": page(1), "sequence": 1, "extra": 1}))] {
        assert!(cancel_command(&registry, malformed).unwrap_err().starts_with(RETIREMENT_ID_INVALID));
    }
    assert_eq!(runs.epoch(), ticket, "a malformed id retired nothing");
    assert_released(&registry);
}

/// Exhaustion by the epoch: a retirement at [`EPOCH_MAX`] cannot move it on (checked; it never wraps), cancels the
/// run in progress, and makes the sizing refuse every start `HANGER-SIZING-EXHAUSTED`, a start already entered
/// included; the reset and later retirements move nothing.
#[test]
fn a_retirement_at_the_largest_epoch_exhausts_the_sizing() {
    let gate = Gate::new("sizing-exhausted");
    let registry = registry_with(gate.launcher());
    let runs = registry.hanger_sizing_runs();
    runs.seed_epoch_for_tests(EPOCH_MAX - 1);
    let ticket = retire(&registry, 1, 1);
    assert_eq!(ticket, EPOCH_MAX);
    let request = body(ticket, "{}");
    let entered = runs.enter(Some(json!(ticket))).unwrap();
    std::thread::scope(|scope| {
        let _guard = CancelOnUnwind(&registry);
        let running = scope.spawn(|| size_via_command(&registry, &request));
        gate.wait_started(1);
        let exhausting = runs.retire_keyed(id(1, 2), registry.pool());
        assert_eq!(exhausting, HangerSizingRetirement { cancelled: true, epoch: EPOCH_MAX, retirement_id: id(1, 2), duplicate: false, exhausted: true });
        assert_eq!(code_of(&running.join().unwrap().unwrap_err()), CANCELLED);
    });
    assert!(matches!(runs.register(entered, registry.pool()), Err(SizingFailure::Exhausted)), "an entered start is refused at registration");
    for ticket in [EPOCH_MAX, 0] {
        assert_eq!(code_of(&size_via_command(&registry, &body(ticket, "{}")).unwrap_err()), EXHAUSTED);
    }
    assert!(!runs.cancel_current(registry.pool()));
    assert!(runs.retire_keyed(id(1, 3), registry.pool()).exhausted);
    assert_eq!(runs.epoch(), EPOCH_MAX, "the epoch never wraps");
    assert_eq!(gate.started_count(), 1);
    assert_released(&registry);
}

/// Exhaustion by the page tokens, and no re-admission: the host keeps every applied token, at most
/// [`RETIREMENT_PAGES_MAX`]. A retirement from a new token beyond them is applied once (it cancels and moves the epoch
/// on) and exhausts the sizing instead of evicting a token, so a late replay of the oldest token's retirement is still
/// a duplicate that moves nothing, and every start is refused.
#[test]
fn page_tokens_are_never_evicted_and_saturation_fails_closed() {
    let registry = registry_with(TestLauncher::solve());
    let runs = registry.hanger_sizing_runs();
    for n in 0..RETIREMENT_PAGES_MAX {
        retire(&registry, n, 1);
    }
    assert_eq!(runs.applied_pages(), RETIREMENT_PAGES_MAX);
    let epoch = runs.epoch();
    let saturating = runs.retire_keyed(id(RETIREMENT_PAGES_MAX, 1), registry.pool());
    assert!(saturating.exhausted && !saturating.duplicate, "{saturating:?}");
    assert_eq!(saturating.epoch, epoch + 1, "the saturating retirement is applied once");
    assert_eq!(runs.applied_pages(), RETIREMENT_PAGES_MAX, "no token was evicted to make room");
    // The oldest token's replay is still known: a duplicate, which moves nothing.
    let replay = runs.retire_keyed(id(0, 1), registry.pool());
    assert!(replay.duplicate && replay.exhausted && !replay.cancelled, "{replay:?}");
    // The saturating token was not recorded: its replay is applied as exhaustion is, and moves nothing either.
    assert!(runs.retire_keyed(id(RETIREMENT_PAGES_MAX, 1), registry.pool()).exhausted);
    assert_eq!(runs.epoch(), epoch + 1);
    assert_eq!(code_of(&size_via_command(&registry, &body(epoch + 1, "{}")).unwrap_err()), EXHAUSTED);
    assert_released(&registry);
}
