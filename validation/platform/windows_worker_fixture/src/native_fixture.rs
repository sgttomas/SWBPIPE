//! Native x64 process/control fixture. This is not a solver or product worker.
//! A controller reserves SAFETY_CAP before spawn and holds it through observed
//! exit. It covers this fixture's single 512 KiB mutator buffer, shared page and
//! tiny allocation, not arbitrary production transition growth.
#[path = "win32.rs"]
mod win32;

use std::{
    alloc::{GlobalAlloc, Layout, System},
    ptr,
    sync::{
        Mutex,
        atomic::{AtomicU32, AtomicUsize, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use win32::{Handle, Raw};

const SAFETY_CAP: usize = 2 * 1024 * 1024;
const BUFFER: usize = 512 * 1024;
const MAP_SIZE: usize = 4096;
const RUNNING: u32 = 1;
const GROW_BLOCKED: u32 = 2;
const PARKED: u32 = 3;
const CANCELLED: u32 = 4;
const FINISHED: u32 = 5;
const START_TIMEOUT: Duration = Duration::from_secs(5);
static SERIAL: Mutex<()> = Mutex::new(());

#[repr(C)]
struct Shared {
    phase: AtomicU32,
    epoch: AtomicU32,
    ack_gate: AtomicU32,
    ack: AtomicU32,
    resume: AtomicU32,
    cancel: AtomicU32,
    progress: AtomicU32,
    resumed: AtomicU32,
    grow: AtomicU32,
    grant: AtomicU32,
    allocated: AtomicU32,
    worker_handle: AtomicUsize,
    retained_job: AtomicUsize,
}
impl Shared {
    fn new() -> Self {
        Self {
            phase: AtomicU32::new(0),
            epoch: AtomicU32::new(0),
            ack_gate: AtomicU32::new(1),
            ack: AtomicU32::new(0),
            resume: AtomicU32::new(0),
            cancel: AtomicU32::new(0),
            progress: AtomicU32::new(0),
            resumed: AtomicU32::new(0),
            grow: AtomicU32::new(0),
            grant: AtomicU32::new(0),
            allocated: AtomicU32::new(0),
            worker_handle: AtomicUsize::new(0),
            retained_job: AtomicUsize::new(0),
        }
    }
}
fn get(name: &str) -> usize {
    std::env::var(format!("NATIVE_FIXTURE_{name}"))
        .expect("fixture handle missing")
        .parse()
        .expect("invalid fixture handle")
}
fn shared(view: &win32::View) -> &Shared {
    unsafe { &*(view.0 as *const Shared) }
}
fn publish(s: &Shared, phase: u32, changed: Raw) {
    s.phase.store(phase, SeqCst);
    win32::signal(changed);
}
fn checkpoint(s: &Shared, cmd: Raw, changed: Raw) -> bool {
    if s.cancel.load(SeqCst) != 0 {
        publish(s, CANCELLED, changed);
        return false;
    }
    let epoch = s.epoch.load(SeqCst);
    if epoch != 0 && epoch > s.resume.load(SeqCst) {
        // A test can hold this gate to model a mutator that has not yet
        // reached its next checkpoint. It is not an acknowledgement.
        if s.ack_gate.load(SeqCst) == 0 {
            return true;
        }
        s.ack.store(epoch, SeqCst);
        publish(s, PARKED, changed);
        loop {
            // Check cancel before resume; event delivery alone is never an ack.
            if s.cancel.load(SeqCst) != 0 {
                publish(s, CANCELLED, changed);
                return false;
            }
            if s.resume.load(SeqCst) >= epoch {
                // A resume published after cancellation cannot revive the
                // mutator even if cancellation arrived after the loop's first
                // check. These SeqCst loads observe that publication order.
                if s.cancel.load(SeqCst) != 0 {
                    publish(s, CANCELLED, changed);
                    return false;
                }
                s.resumed.fetch_add(1, SeqCst);
                publish(s, RUNNING, changed);
                break;
            }
            win32::wait(cmd, 20);
        }
    }
    true
}
fn worker(mode: &str, s: &Shared, cmd: Raw, changed: Raw, channel: Raw) {
    let mut pages = vec![0u8; BUFFER];
    if mode == "grow" {
        // Create the control thread and all state before entering the simulated
        // allocator path. The control thread never acquires the channel mutex.
        let sp = s as *const Shared as usize;
        let cmd_value = cmd as usize;
        let changed_value = changed as usize;
        let control = std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(move || {
                let s = unsafe { &*(sp as *const Shared) };
                let cmd = cmd_value as Raw;
                let changed = changed_value as Raw;
                loop {
                    if s.phase.load(SeqCst) == FINISHED {
                        return;
                    }
                    if s.cancel.load(SeqCst) != 0 {
                        publish(s, CANCELLED, changed);
                        return;
                    }
                    if s.grow.load(SeqCst) != 0 && s.phase.load(SeqCst) == GROW_BLOCKED {
                        let e = s.epoch.load(SeqCst);
                        if e != 0 && e > s.resume.load(SeqCst) {
                            s.ack.store(e, SeqCst);
                            publish(s, PARKED, changed);
                        }
                    }
                    win32::wait(cmd, 10);
                }
            })
            .expect("fixture control thread creation failed");
        assert!(
            win32::wait(channel, 2000),
            "child failed to acquire channel"
        );
        s.grow.store(1, SeqCst);
        publish(s, GROW_BLOCKED, changed);
        // No allocation or channel re-entry occurs in this wait. An actual tiny
        // System allocation happens only after a grant and an unparked state.
        loop {
            if s.cancel.load(SeqCst) != 0 {
                break;
            }
            if s.grant.load(SeqCst) >= 64
                && (s.epoch.load(SeqCst) == 0 || s.resume.load(SeqCst) >= s.epoch.load(SeqCst))
            {
                if s.cancel.load(SeqCst) != 0 {
                    break;
                }
                let layout = Layout::from_size_align(64, 8).unwrap();
                let p = unsafe { System.alloc(layout) };
                assert!(!p.is_null());
                unsafe {
                    ptr::write_volatile(p, 7);
                    System.dealloc(p, layout);
                }
                s.allocated.store(1, SeqCst);
                publish(s, FINISHED, changed);
                break;
            }
            win32::wait(cmd, 10);
        }
        win32::release_mutex(channel);
        win32::signal(cmd);
        control.join().expect("fixture control thread failed");
        return;
    }
    assert!(
        matches!(mode, "checkpoint" | "unchecked"),
        "invalid worker mode"
    );
    publish(s, RUNNING, changed);
    let end = Instant::now() + Duration::from_secs(8);
    while Instant::now() < end {
        // This single mutator touches every 4 KiB slot. The unchecked mode has
        // no pause or cancellation check in the bounded work loop.
        for i in (0..pages.len()).step_by(4096) {
            unsafe {
                ptr::write_volatile(pages.as_mut_ptr().add(i), 1);
            }
        }
        s.progress.fetch_add(1, SeqCst);
        if mode == "checkpoint" && !checkpoint(s, cmd, changed) {
            return;
        }
    }
    publish(s, FINISHED, changed);
}

#[test]
fn child_entry() {
    let Ok(mode) = std::env::var("NATIVE_FIXTURE_MODE") else {
        return;
    };
    let mapping = get("MAPPING") as Raw;
    let cmd = get("CMD") as Raw;
    let changed = get("CHANGED") as Raw;
    let channel = get("CHANNEL") as Raw;
    let view = win32::map(mapping, MAP_SIZE);
    let s = shared(&view);
    if mode == "caretaker" || mode == "caretaker_retained" {
        let inner_job = win32::job();
        let handles = [mapping, cmd, changed, channel];
        let values = [
            ("MAPPING", mapping as usize),
            ("CMD", cmd as usize),
            ("CHANGED", changed as usize),
            ("CHANNEL", channel as usize),
        ];
        let mut child = win32::spawn_suspended(inner_job.0, &handles, "checkpoint", &values);
        // Publish a caretaker-owned source handle, not a handle created in
        // another process. The observer duplicates it under its own guard
        // while this caretaker is alive, so a publication failure cannot leak
        // an unidentified observer-owned handle.
        s.worker_handle.store(child.handle.value(), SeqCst);
        if mode == "caretaker_retained" {
            s.retained_job.store(inner_job.value(), SeqCst);
        }
        child.resume();
        win32::signal(changed);
        // Observer terminates this controller after the worker is parked.
        // An unexpected timeout is a child failure and RAII kills/reaps worker.
        assert!(
            !child.exited(10000),
            "worker exited unexpectedly under caretaker"
        );
        panic!("caretaker timed out without observer");
    }
    worker(&mode, s, cmd, changed, channel);
}

struct Fixture {
    // Explicit logical reservation exists before process creation and is held
    // until the process handle is signaled, including all parked phases.
    lease: usize,
    process: Option<win32::Process>,
    job: Handle,
    mapping: Handle,
    view: win32::View,
    cmd: Handle,
    changed: Handle,
    channel: Handle,
}
impl Fixture {
    fn new(mode: &str) -> Self {
        assert!(std::mem::size_of::<Shared>() <= MAP_SIZE);
        assert!(BUFFER + MAP_SIZE + 64 <= SAFETY_CAP && SAFETY_CAP <= 64 * 1024 * 1024);
        eprintln!("phase=spawn mode={mode} safety_allowance_bytes={SAFETY_CAP}");
        let lease = SAFETY_CAP;
        let (mapping, view) = win32::mapping(MAP_SIZE as u32);
        unsafe {
            ptr::write(view.0 as *mut Shared, Shared::new());
        }
        let cmd = win32::event();
        let changed = win32::event();
        let channel = win32::mutex();
        let job = win32::job();
        let handles = vec![mapping.0, cmd.0, changed.0, channel.0];
        let values = vec![
            ("MAPPING", mapping.value()),
            ("CMD", cmd.value()),
            ("CHANGED", changed.value()),
            ("CHANNEL", channel.value()),
        ];
        let mut process = win32::spawn_suspended(job.0, &handles, mode, &values);
        process.resume();
        Self {
            lease,
            process: Some(process),
            job,
            mapping,
            view,
            cmd,
            changed,
            channel,
        }
    }
    fn s(&self) -> &Shared {
        shared(&self.view)
    }
    fn notify(&self) {
        win32::signal(self.cmd.0);
    }
    fn wait_for(&mut self, what: &str, predicate: impl Fn(&Shared) -> bool) {
        let deadline = Instant::now() + START_TIMEOUT;
        loop {
            if predicate(self.s()) {
                eprintln!("phase=observed condition={what}");
                return;
            }
            assert!(
                !self.process.as_mut().unwrap().exited(0),
                "child exited before {what}"
            );
            let left = deadline.saturating_duration_since(Instant::now());
            assert!(!left.is_zero(), "timeout waiting for {what}");
            win32::wait(
                self.changed.0,
                left.min(Duration::from_millis(50)).as_millis() as u32,
            );
        }
    }
    fn pause(&self, epoch: u32) {
        self.s().epoch.store(epoch, SeqCst);
        self.notify();
    }
    fn resume(&self, epoch: u32) {
        self.s().resume.store(epoch, SeqCst);
        self.notify();
    }
    fn cancel(&self) {
        self.s().cancel.store(1, SeqCst);
        self.notify();
    }
    fn reap_ok(&mut self) {
        let p = self.process.as_mut().unwrap();
        assert!(p.exited(5000), "worker did not exit");
        assert_eq!(p.code(), 0, "worker test failed");
        eprintln!("phase=observed-exit exit=0; releasing_fixture_reservation=true");
        self.lease = 0; // Only after the signaled process and successful test result.
    }
    fn terminate_and_reap(&mut self) {
        self.process.as_mut().unwrap().terminate_and_wait();
        self.lease = 0;
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Some(p) = self.process.as_mut() {
            p.terminate_and_wait();
        }
        // No raw caretaker-owned handle is closed in this process. The
        // outer Job also covers a worker whose handle was not yet published.
        win32::drain_job(self.job.0);
        self.lease = 0;
        // Explicitly keep handles and mapping alive until after process exit.
        let _ = (
            &self.job,
            &self.mapping,
            &self.view,
            &self.cmd,
            &self.changed,
            &self.channel,
        );
    }
}

#[test]
fn native_grow_blocked_control_and_cancellation() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut f = Fixture::new("grow");
    f.wait_for("allocator GROW", |s| {
        s.grow.load(SeqCst) == 1 && s.phase.load(SeqCst) == GROW_BLOCKED
    });
    assert!(
        !win32::wait(f.channel.0, 0),
        "allocator did not hold channel mutex"
    );
    assert_eq!(f.s().allocated.load(SeqCst), 0, "allocation preceded grant");
    f.pause(1);
    f.wait_for("current grow checkpoint", |s| {
        s.ack.load(SeqCst) == 1 && s.phase.load(SeqCst) == PARKED
    });
    assert!(
        !win32::wait(f.channel.0, 0),
        "control acquired or released channel mutex"
    );
    f.s().grant.store(64, SeqCst);
    f.cancel();
    f.resume(1);
    f.reap_ok();
    assert_eq!(
        f.s().allocated.load(SeqCst),
        0,
        "late grant/resume revived a cancelled grow"
    );
    assert_eq!(f.s().phase.load(SeqCst), CANCELLED);
}

#[test]
fn native_cancel_reset_across_worker_phases() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for mode in ["running", "grow", "requested", "parked"] {
        let mut f = Fixture::new(if mode == "grow" {
            "grow"
        } else if mode == "requested" {
            "unchecked"
        } else {
            "checkpoint"
        });
        f.wait_for("worker start", |s| s.phase.load(SeqCst) != 0);
        if mode == "grow" {
            f.wait_for("blocked grow", |s| s.grow.load(SeqCst) == 1);
        }
        if mode == "requested" {
            f.pause(1);
            assert_eq!(f.s().ack.load(SeqCst), 0);
        }
        if mode == "parked" {
            f.pause(1);
            f.wait_for("park", |s| {
                s.ack.load(SeqCst) == 1 && s.phase.load(SeqCst) == PARKED
            });
        }
        f.cancel();
        f.resume(1);
        if mode == "requested" {
            f.terminate_and_reap();
        } else {
            f.reap_ok();
            assert_eq!(f.s().phase.load(SeqCst), CANCELLED, "{mode}");
            assert_eq!(f.s().resumed.load(SeqCst), 0, "{mode}");
        }
    }
}

/// This pool is fixture-only. Each verdict acquires the same lock and consumes
/// one freshly scripted headroom sample; no host-headroom claim is made.
struct ScriptedPool {
    samples: Mutex<Vec<u64>>,
    calls: AtomicU32,
    admitted: AtomicUsize,
}
impl ScriptedPool {
    fn new(samples: Vec<u64>) -> Self {
        Self {
            samples: Mutex::new(samples),
            calls: AtomicU32::new(0),
            admitted: AtomicUsize::new(0),
        }
    }
    fn attempt(&self, preferred: u64, minimal: u64) -> crate::Admission {
        assert!(preferred >= minimal);
        let mut samples = self.samples.lock().unwrap();
        self.calls.fetch_add(1, SeqCst);
        let fresh = if samples.len() > 1 {
            samples.remove(0)
        } else {
            *samples.first().expect("empty sample script")
        };
        // Sampling, preferred/minimal selection and the fixture ledger update
        // share one lock. Samples are injected, never native host headroom.
        let (verdict, bytes) = if fresh >= preferred {
            (crate::Admission::Preferred, preferred)
        } else if fresh >= minimal {
            (crate::Admission::Minimal, minimal)
        } else {
            return crate::Admission::Retry;
        };
        self.admitted.fetch_add(bytes as usize, SeqCst);
        verdict
    }
}

#[test]
fn native_admission_fresh_samples_release_and_absolute_deadline() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut f = Fixture::new("checkpoint");
    f.wait_for("running mutator", |s| s.phase.load(SeqCst) == RUNNING);
    let pool = ScriptedPool::new(vec![0, 64, 128]);
    let release = win32::event();
    let start = Instant::now();
    let deadline = start + Duration::from_secs(2);
    f.pause(1);
    // A stale generation cannot establish quiescence, and sending an event
    // does not itself establish a current-generation ack.
    assert_ne!(f.s().ack.load(SeqCst), 2);
    f.wait_for("first checkpoint", |s| {
        s.ack.load(SeqCst) == 1 && s.phase.load(SeqCst) == PARKED
    });
    assert!(Instant::now() < deadline);
    assert_eq!(pool.attempt(128, 64), crate::Admission::Retry);
    // An independent releaser wakes the waiting supervisor before the next
    // 50 ms tick; the next verdict must take a new sample.
    let timer = win32::event();
    let release_thread = win32::duplicate_local(release.0);
    let releaser = std::thread::Builder::new()
        .stack_size(128 * 1024)
        .spawn(move || {
            win32::wait(timer.0, 10);
            win32::signal(release_thread.0);
            // Keep ownership of both handles until after the signal.
            drop((timer, release_thread));
        })
        .expect("fixture release thread creation failed");
    assert!(win32::wait(release.0, 50));
    // The signal proves its final operation completed. Do not join a parent
    // helper without an external deadline; it touches no handles afterwards.
    drop(releaser);
    assert_eq!(pool.attempt(128, 64), crate::Admission::Minimal);
    f.resume(1);
    f.wait_for("resume", |s| s.resumed.load(SeqCst) == 1);
    // The previous epoch's ack cannot satisfy this generation.
    assert_eq!(f.s().ack.load(SeqCst), 1);
    assert_ne!(f.s().ack.load(SeqCst), 2);
    f.pause(2);
    f.wait_for("second checkpoint", |s| {
        s.ack.load(SeqCst) == 2 && s.phase.load(SeqCst) == PARKED
    });
    assert_eq!(pool.attempt(128, 64), crate::Admission::Preferred);
    assert_eq!(pool.admitted.load(SeqCst), 192);
    assert_eq!(pool.calls.load(SeqCst), 3, "every attempt sampled");
    f.resume(2);
    f.wait_for("second resume", |s| s.resumed.load(SeqCst) == 2);
    f.s().ack_gate.store(0, SeqCst);
    let requested = Instant::now();
    let deadline = requested + Duration::from_secs(2);
    f.pause(3);
    let persistent = ScriptedPool::new(vec![0]);
    let mut attempts = 0;
    let mut gate_opened = false;
    let mut saw_current_ack = false;
    loop {
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        assert_eq!(persistent.attempt(128, 64), crate::Admission::Retry);
        attempts += 1;
        if !gate_opened && requested.elapsed() >= Duration::from_millis(1100) {
            f.s().ack_gate.store(1, SeqCst);
            f.notify();
            gate_opened = true;
        }
        if f.s().ack.load(SeqCst) == 3 && f.s().phase.load(SeqCst) == PARKED {
            saw_current_ack = true;
        }
        let left = deadline.saturating_duration_since(Instant::now());
        win32::wait(
            release.0,
            left.min(Duration::from_millis(50)).as_millis() as u32,
        );
    }
    assert!(
        gate_opened && saw_current_ack,
        "late current acknowledgement was not observed"
    );
    assert!(attempts >= 2 && persistent.calls.load(SeqCst) == attempts);
    assert!(requested.elapsed() >= Duration::from_secs(2));
    assert!(
        requested.elapsed() < Duration::from_secs(3),
        "ack reset or supervisor blocked the deadline"
    );
    f.cancel();
    f.resume(3);
    f.reap_ok();
    assert_eq!(f.s().phase.load(SeqCst), CANCELLED);
}

#[test]
fn native_uncheckpointed_work_fails_quiescence_but_supervisor_reaches_deadline() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut f = Fixture::new("unchecked");
    f.wait_for("unchecked work", |s| {
        s.phase.load(SeqCst) == RUNNING && s.progress.load(SeqCst) > 0
    });
    f.pause(1);
    let started = Instant::now();
    let deadline = started + Duration::from_secs(2);
    let before = f.s().progress.load(SeqCst);
    // Supervisor never blocks on an ack; the event wait is bounded by the
    // original deadline and the tick, including while the worker keeps running.
    let tick = win32::event();
    while Instant::now() < deadline {
        assert_eq!(
            f.s().ack.load(SeqCst),
            0,
            "unchecked work falsely acknowledged quiescence"
        );
        assert_eq!(f.s().phase.load(SeqCst), RUNNING);
        let remaining = deadline.saturating_duration_since(Instant::now());
        win32::wait(
            tick.0,
            remaining.min(Duration::from_millis(50)).as_millis() as u32,
        );
    }
    let after = f.s().progress.load(SeqCst);
    assert!(
        after > before,
        "negative case lacks evidence of ongoing work"
    );
    assert_eq!(f.s().ack.load(SeqCst), 0);
    assert!(started.elapsed() < Duration::from_secs(5));
    f.cancel();
    f.resume(1);
    // Uncheckpointed code does not cooperate; the controller kills and waits.
    eprintln!("phase=missing-quiescence; acknowledged=false; ongoing_progress=true");
    f.terminate_and_reap();
}

struct DeathWitness {
    caretaker: Fixture,
    worker: Handle,
    retained: Option<Handle>,
}
impl DeathWitness {
    fn new(retained: bool) -> Self {
        let mut caretaker = Fixture::new(if retained {
            "caretaker_retained"
        } else {
            "caretaker"
        });
        caretaker.wait_for("published caretaker process handle and worker start", |s| {
            s.worker_handle.load(SeqCst) != 0
                && s.phase.load(SeqCst) == RUNNING
                && (!retained || s.retained_job.load(SeqCst) != 0)
        });
        let worker = win32::duplicate_from(
            caretaker.process.as_ref().unwrap().handle.0,
            caretaker.s().worker_handle.load(SeqCst) as Raw,
        );
        assert!(
            win32::is_in_job(worker.0, caretaker.job.0),
            "worker escaped outer cleanup Job"
        );
        caretaker.pause(1);
        caretaker.wait_for("caretaker worker parked", |s| {
            s.ack.load(SeqCst) == 1 && s.phase.load(SeqCst) == PARKED
        });
        let retained = if retained {
            let source = caretaker.s().retained_job.load(SeqCst) as Raw;
            Some(win32::duplicate_from(
                caretaker.process.as_ref().unwrap().handle.0,
                source,
            ))
        } else {
            None
        };
        Self {
            caretaker,
            worker,
            retained,
        }
    }
    fn worker_handle(&self) -> Raw {
        self.worker.0
    }
    fn end_caretaker(&mut self) {
        self.caretaker
            .process
            .as_mut()
            .unwrap()
            .terminate_and_wait();
        // Its logical reservation remains held until the worker is observed dead.
    }
}
impl Drop for DeathWitness {
    fn drop(&mut self) {
        // Stop the sole publisher first, so no duplicated handle can appear
        // after cleanup checks. This guard also runs on assertion failure.
        self.caretaker
            .process
            .as_mut()
            .unwrap()
            .terminate_and_wait();
        win32::terminate_and_wait(self.worker.0);
        self.retained.take();
        win32::drain_job(self.caretaker.job.0);
        self.caretaker.lease = 0;
    }
}

#[test]
fn native_parent_death_kills_parked_worker_before_lease_release() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut witness = DeathWitness::new(false);
    assert_eq!(
        witness.caretaker.s().retained_job.load(SeqCst),
        0,
        "observer must not hold inner Job"
    );
    assert_eq!(witness.caretaker.lease, SAFETY_CAP);
    witness.end_caretaker();
    assert!(
        win32::wait(witness.worker_handle(), 5000),
        "last Job handle did not end parked worker"
    );
    assert_eq!(
        witness.caretaker.lease, SAFETY_CAP,
        "lease released before observed exit"
    );
    witness.caretaker.lease = 0;
}

#[test]
fn native_retained_job_handle_requires_external_cleanup() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut witness = DeathWitness::new(true);
    assert_ne!(witness.caretaker.s().retained_job.load(SeqCst), 0);
    witness.end_caretaker();
    assert!(
        !win32::wait(witness.worker_handle(), 200),
        "worker died despite deliberately retained Job handle"
    );
    assert_eq!(witness.caretaker.lease, SAFETY_CAP);
    win32::terminate_and_wait(witness.worker_handle());
    assert!(
        win32::wait(witness.worker_handle(), 0),
        "external cleanup did not observe worker exit"
    );
    witness.caretaker.lease = 0;
}

#[test]
fn native_grow_grant_only_allocates_after_current_resume() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut f = Fixture::new("grow");
    f.wait_for("blocked grow", |s| s.phase.load(SeqCst) == GROW_BLOCKED);
    f.pause(1);
    f.wait_for("grow parked", |s| {
        s.ack.load(SeqCst) == 1 && s.phase.load(SeqCst) == PARKED
    });
    f.s().grant.store(64, SeqCst);
    f.notify();
    let tick = win32::event();
    win32::wait(tick.0, 50);
    assert_eq!(f.s().allocated.load(SeqCst), 0, "grant bypassed checkpoint");
    f.resume(1);
    f.reap_ok();
    assert_eq!(f.s().allocated.load(SeqCst), 1);
    assert_eq!(f.s().phase.load(SeqCst), FINISHED);
}

/// The real blocked allocator fixture traverses the scripted admission wait;
/// it is not merely exercised beside an unrelated pure protocol model.
#[test]
fn native_grow_waits_for_transient_or_persistent_headroom() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    for persistent in [false, true] {
        let mut f = Fixture::new("grow");
        f.wait_for("allocator before allocation", |s| {
            s.phase.load(SeqCst) == GROW_BLOCKED
        });
        let pool = ScriptedPool::new(if persistent { vec![0] } else { vec![0, 64] });
        let release = win32::event();
        let first = Instant::now();
        let deadline = first + crate::WINDOW;
        f.pause(1);
        let mut admitted = None;
        while Instant::now() < deadline {
            assert_eq!(
                f.s().allocated.load(SeqCst),
                0,
                "allocation escaped the waiting grant"
            );
            if f.s().ack.load(SeqCst) == 1 && f.s().phase.load(SeqCst) == PARKED {
                let verdict = pool.attempt(128, 64);
                if verdict != crate::Admission::Retry {
                    admitted = Some(verdict);
                    break;
                }
                // A release is a wakeup hint, never an admission. The next
                // iteration still samples under the fixture pool lock.
                if !persistent {
                    win32::signal(release.0);
                }
            }
            let left = deadline.saturating_duration_since(Instant::now());
            win32::wait(release.0, left.min(crate::TICK).as_millis() as u32);
        }
        assert!(pool.calls.load(SeqCst) >= 2, "missing fresh attempts");
        if persistent {
            assert!(admitted.is_none());
            assert_eq!(pool.admitted.load(SeqCst), 0);
            assert!(first.elapsed() >= crate::WINDOW && first.elapsed() < Duration::from_secs(3));
            eprintln!("phase=grow-window-expired; absolute_window_ms=2000; allocated=0");
            f.cancel();
            f.resume(1);
            f.reap_ok();
            assert_eq!(f.s().allocated.load(SeqCst), 0);
        } else {
            assert_eq!(admitted, Some(crate::Admission::Minimal));
            assert!(Instant::now() < deadline);
            f.s().grant.store(pool.admitted.load(SeqCst) as u32, SeqCst);
            f.resume(1);
            f.reap_ok();
            assert_eq!(f.s().allocated.load(SeqCst), 1);
            eprintln!("phase=grow-admitted; preferred_bytes=128; minimal_bytes=64");
        }
    }
}
