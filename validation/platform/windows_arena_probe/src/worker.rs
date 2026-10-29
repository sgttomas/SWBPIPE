//! Native children with creation-time Job custody; events are not acknowledgements.
use crate::{arena::Arena, ledger::{Failure, Ledger, Window}, win32};
use std::{sync::{atomic::{AtomicU32, Ordering::SeqCst}, Mutex}, time::{Duration, Instant}};
const MAP_SIZE: u32 = 4096;
const SETUP: u32 = 1;
const READY: u32 = 2;
const GROW: u32 = 3;
const AT_GATE: u32 = 4;
const DONE: u32 = 5;
const GRANT: u32 = 1;
const PHYSICAL: u32 = 2;
const COMMIT: u32 = 3;
const UNKNOWN: u32 = 4;
const CANCELLED: u32 = 5;
const OTHER: u32 = 6;
const OK: u32 = 7;
const TIMEOUT: u32 = 8;
const ADDRESS: u32 = 9;
const LIMIT: u32 = 10;
const OS_COMMIT: u32 = 11;
const OS_DECOMMIT: u32 = 12;
const SYNTHETIC_HEADROOM: u32 = 13;
const STALE: u32 = 14;
const SYNTHETIC_DECOMMIT: u32 = 15;

#[repr(C)]
struct Shared {
    request: AtomicU32, reply: AtomicU32, verdict: AtomicU32, amount: AtomicU32,
    cancel: AtomicU32, phase: AtomicU32, start: AtomicU32, finish: AtomicU32,
    result: AtomicU32, detail: AtomicU32, actual: AtomicU32, authorized: AtomicU32,
    faults: AtomicU32, retries: AtomicU32, old_intact: AtomicU32, elapsed_ms: AtomicU32,
}
impl Shared {
    fn new() -> Self { Self {
        request: AtomicU32::new(0), reply: AtomicU32::new(0), verdict: AtomicU32::new(0), amount: AtomicU32::new(0),
        cancel: AtomicU32::new(0), phase: AtomicU32::new(0), start: AtomicU32::new(0), finish: AtomicU32::new(0),
        result: AtomicU32::new(0), detail: AtomicU32::new(0), actual: AtomicU32::new(0), authorized: AtomicU32::new(0),
        faults: AtomicU32::new(0), retries: AtomicU32::new(0), old_intact: AtomicU32::new(0), elapsed_ms: AtomicU32::new(0),
    } }
}
fn shared(view: &win32::View) -> &Shared { unsafe { &*view.0.cast::<Shared>() } }
fn handle(name: &str) -> win32::Raw { std::env::var(format!("NATIVE_FIXTURE_{name}")).unwrap().parse::<usize>().unwrap() as win32::Raw }
fn outcome(f: Failure) -> u32 {
    match f {
        Failure::PhysicalHeadroom => PHYSICAL, Failure::CommitHeadroom => COMMIT,
        Failure::SyntheticCommitHeadroom => SYNTHETIC_HEADROOM,
        Failure::SyntheticCommitOther => OTHER, Failure::SyntheticDecommit => SYNTHETIC_DECOMMIT,
        Failure::Cancelled => CANCELLED, Failure::Stale => STALE,
        Failure::Unknown => UNKNOWN, Failure::Deadline => TIMEOUT,
        Failure::Address => ADDRESS, Failure::Limit => LIMIT,
        Failure::OsCommit(_) => OS_COMMIT, Failure::OsDecommit(_) => OS_DECOMMIT,
    }
}
fn failure_detail(f: Failure) -> u32 { match f { Failure::OsCommit(code) | Failure::OsDecommit(code) => code, _ => 0 } }
fn check(s: &Shared, w: &Window) -> Result<(), Failure> {
    if s.cancel.load(SeqCst) != 0 { Err(Failure::Cancelled) }
    else if !w.open() { Err(Failure::Deadline) }
    else { Ok(()) }
}
fn request(s: &Shared, command: win32::Raw, changed: win32::Raw, epoch: &mut u32, w: &Window, amount: usize) -> Result<(), Failure> {
    let mut last = None;
    loop {
        if s.cancel.load(SeqCst) != 0 { return Err(Failure::Cancelled); }
        if !w.open() { return Err(last.unwrap_or(Failure::Deadline)); }
        *epoch = epoch.checked_add(1).ok_or(Failure::Limit)?;
        s.amount.store(u32::try_from(amount).map_err(|_| Failure::Limit)?, SeqCst);
        s.request.store(*epoch, SeqCst); win32::signal(changed);
        loop {
            if s.cancel.load(SeqCst) != 0 { return Err(Failure::Cancelled); }
            if !w.open() { return Err(last.unwrap_or(Failure::Deadline)); }
            if s.reply.load(SeqCst) == *epoch {
                match s.verdict.load(SeqCst) {
                    GRANT => { if s.cancel.load(SeqCst) != 0 { return Err(Failure::Cancelled); } return Ok(()); }
                    PHYSICAL => last = Some(Failure::PhysicalHeadroom),
                    COMMIT => last = Some(Failure::CommitHeadroom),
                    _ => return Err(Failure::Unknown),
                }
                break;
            }
            win32::wait(command, 20);
        }
        let reason = last.expect("headroom reply classified");
        if !w.retry(reason) { return Err(reason); }
        s.retries.fetch_add(1, SeqCst);
        win32::wait(command, 10);
    }
}
#[test]
fn child_entry() {
    let Ok(mode) = std::env::var("NATIVE_FIXTURE_MODE") else { return; };
    if mode == "negative" { panic!("injected child failure negative control"); }
    let view = win32::map(handle("MAPPING"), MAP_SIZE as usize);
    let s = shared(&view); let command = handle("COMMAND"); let changed = handle("CHANGED");
    if !matches!(mode.as_str(), "recover" | "nonheadroom" | "commit_timeout" | "cancel_granted" | "blocked" | "persistent" | "normal") {
        s.result.store(UNKNOWN, SeqCst); s.phase.store(DONE, SeqCst); win32::signal(changed); return;
    }
    let mut epoch = 0u32;
    s.phase.store(SETUP, SeqCst);
    let setup = (|| -> Result<(Arena, usize), Failure> {
        let w = Window::new();
        let mut admit = |n| request(s, command, changed, &mut epoch, &w, n);
        let mut guard = || check(s, &w);
        let mut arena = Arena::new(&mut admit, &mut guard)?;
        // A distinct operation receives its own original window.
        let w = Window::new();
        let mut admit = |n| request(s, command, changed, &mut epoch, &w, n);
        let mut guard = || check(s, &w);
        let old = arena.alloc(8192, 64, &mut admit, &mut guard)?;
        unsafe { *arena.byte(old, 0) = 73; }
        Ok((arena, old))
    })();
    let (mut arena, old) = match setup { Ok(v) => v, Err(f) => {
        s.detail.store(failure_detail(f), SeqCst); s.result.store(outcome(f), SeqCst); s.phase.store(DONE, SeqCst); win32::signal(changed); return;
    } };
    s.actual.store(arena.committed_bytes() as u32, SeqCst);
    s.authorized.store(arena.authorized_bytes() as u32, SeqCst);
    s.phase.store(READY, SeqCst); win32::signal(changed);
    let start_deadline = Instant::now() + Duration::from_secs(8);
    while s.start.load(SeqCst) == 0 && s.cancel.load(SeqCst) == 0 {
        assert!(Instant::now() < start_deadline, "child start deadline"); win32::wait(command, 20);
    }
    s.phase.store(GROW, SeqCst);
    let started = Instant::now(); let w = Window::new();
    let mut injected = false;
    let result = loop {
        let mut admit = |n| request(s, command, changed, &mut epoch, &w, n);
        let mut guard = || -> Result<(), Failure> {
            check(s, &w)?;
            if mode == "cancel_granted" && !injected {
                injected = true;
                s.phase.store(AT_GATE, SeqCst); win32::signal(changed);
                while s.cancel.load(SeqCst) == 0 && w.open() { win32::wait(command, 20); }
                return check(s, &w);
            }
            if mode == "commit_timeout" && injected {
                while w.open() && s.cancel.load(SeqCst) == 0 { win32::wait(command, 20); }
                if s.cancel.load(SeqCst) != 0 { return Err(Failure::Cancelled); }
                return Err(Failure::SyntheticCommitHeadroom);
            }
            if !injected && (mode == "recover" || mode == "nonheadroom" || mode == "commit_timeout") {
                injected = true; s.faults.fetch_add(1, SeqCst); win32::signal(changed);
                return Err(if mode == "nonheadroom" { Failure::SyntheticCommitOther } else { Failure::SyntheticCommitHeadroom });
            }
            Ok(())
        };
        match arena.realloc(old, 128*1024, 64, &mut admit, &mut guard) {
            Ok(moved) => { assert_eq!(unsafe { *arena.byte(moved, 0) }, 73); break Ok(()); }
            Err(f) => {
                assert_eq!(unsafe { *arena.byte(old, 0) }, 73);
                s.old_intact.store(1, SeqCst);
                if !w.retry(f) { break Err(f); }
                s.retries.fetch_add(1, SeqCst);
            }
        }
    };
    s.elapsed_ms.store(started.elapsed().as_millis().min(u32::MAX as u128) as u32, SeqCst);
    s.actual.store(arena.committed_bytes() as u32, SeqCst);
    s.authorized.store(arena.authorized_bytes() as u32, SeqCst);
    if let Err(f) = result { s.detail.store(failure_detail(f), SeqCst); }
    s.result.store(match result { Ok(()) => OK, Err(f) => outcome(f) }, SeqCst);
    s.phase.store(DONE, SeqCst); win32::signal(changed);
    let finish_deadline = Instant::now() + Duration::from_secs(8);
    while s.finish.load(SeqCst) == 0 { assert!(Instant::now() < finish_deadline, "child finish deadline"); win32::wait(command, 20); }
}
struct Child { process: win32::Process, _map: win32::Handle, view: win32::View, command: win32::Handle, changed: win32::Handle }
impl Child {
    fn start(job: win32::Raw, mode: &str) -> Self {
        let (map, view) = win32::mapping(MAP_SIZE); unsafe { view.0.cast::<Shared>().write(Shared::new()); }
        let command = win32::event(); let changed = win32::event();
        let hs = [map.0, command.0, changed.0];
        let values = [("MAPPING", map.value()), ("COMMAND", command.value()), ("CHANGED", changed.value())];
        let mut process = win32::spawn_suspended(job, &hs, mode, &values); process.resume();
        Self { process, _map: map, view, command, changed }
    }
}
struct Job(win32::Handle);
impl Drop for Job { fn drop(&mut self) { win32::drain_job(self.0.0); } }
fn until(child: &mut Child, predicate: impl Fn(&Shared) -> bool) {
    let end = Instant::now() + Duration::from_secs(5);
    loop { if predicate(shared(&child.view)) { return; } assert!(!child.process.exited(0), "child exited before phase"); assert!(Instant::now() < end, "child phase deadline"); win32::wait(child.changed.0, 20); }
}
fn reply(child: &Child, ledger: &Mutex<Ledger>, worker: usize, epoch: u32, rejection: Option<u32>) {
    let s = shared(&child.view);
    let verdict = if let Some(code) = rejection { code } else {
        let mut l = ledger.lock().unwrap(); let target = l.held[worker].checked_add(s.amount.load(SeqCst) as usize).unwrap();
        let generation = l.generation[worker];
        match l.grant_to(worker, generation, target, target) { Ok(_) => GRANT, Err(Failure::PhysicalHeadroom) => PHYSICAL, Err(Failure::CommitHeadroom) => COMMIT, Err(_) => UNKNOWN }
    };
    s.verdict.store(verdict, SeqCst); s.reply.store(epoch, SeqCst); win32::signal(child.command.0);
}
fn setup(child: &mut Child, ledger: &Mutex<Ledger>, worker: usize) -> u32 {
    let mut handled = 0;
    loop {
        until(child, |s| s.phase.load(SeqCst) == READY || s.phase.load(SeqCst) == DONE || s.request.load(SeqCst) > handled);
        let s = shared(&child.view); let phase = s.phase.load(SeqCst);
        assert_ne!(phase, DONE, "setup failed with typed result {}", s.result.load(SeqCst));
        if phase == READY { break; }
        handled = s.request.load(SeqCst); reply(child, ledger, worker, handled, None);
    }
    let s = shared(&child.view); let mut l = ledger.lock().unwrap();
    let actual = s.actual.load(SeqCst) as usize; l.record_actual(worker, actual).unwrap();
    assert_eq!(l.held[worker], s.authorized.load(SeqCst) as usize);
    handled
}
fn start_grow(child: &Child) { shared(&child.view).start.store(1, SeqCst); win32::signal(child.command.0); }
fn finish(child: &mut Child, ledger: &Mutex<Ledger>, worker: usize) {
    let s = shared(&child.view); let actual = s.actual.load(SeqCst) as usize; let authorized = s.authorized.load(SeqCst) as usize;
    { let mut l = ledger.lock().unwrap(); l.record_actual(worker, actual).unwrap(); assert!(l.held[worker] >= authorized); }
    s.finish.store(1, SeqCst); win32::signal(child.command.0);
    assert!(child.process.exited(5000)); assert_eq!(child.process.code(), 0);
    ledger.lock().unwrap().observed_exit(worker);
}

#[test]
fn native_recovery() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut child = Child::start(job.0.0, "recover"); let mut handled = setup(&mut child, &ledger, 0);
    start_grow(&child); let mut rejected = 0;
    loop {
        until(&mut child, |s| s.phase.load(SeqCst) == DONE || s.request.load(SeqCst) > handled);
        let s = shared(&child.view); if s.phase.load(SeqCst) == DONE { break; }
        handled = s.request.load(SeqCst);
        if rejected == 0 {
            // A stale grant is not an acknowledgement of this request.
            s.verdict.store(GRANT, SeqCst); s.reply.store(handled-1, SeqCst); win32::signal(child.command.0);
            std::thread::sleep(Duration::from_millis(30));
            assert_ne!(s.reply.load(SeqCst), s.request.load(SeqCst));
            assert_eq!(s.faults.load(SeqCst), 0);
            assert_ne!(s.phase.load(SeqCst), DONE);
        }
        let refusal = match rejected { 0 => Some(PHYSICAL), 1 => Some(COMMIT), _ => None };
        reply(&child, &ledger, 0, handled, refusal); rejected += 1;
    }
    let s = shared(&child.view);
    assert_eq!(s.result.load(SeqCst), OK); assert_eq!(s.faults.load(SeqCst), 1);
    assert!(s.retries.load(SeqCst) >= 3); assert_eq!(s.old_intact.load(SeqCst), 1);
    assert!(s.elapsed_ms.load(SeqCst) < 2000);
    // Equality shows the post-grant injected commit retry did not re-grant
    // the already authorized page, even with partial progress.
    assert_eq!(ledger.lock().unwrap().held[0], s.authorized.load(SeqCst) as usize);
    assert!(s.actual.load(SeqCst) <= s.authorized.load(SeqCst));
    println!("native recovery actual={} held={} retries={} elapsed_ms={}",s.actual.load(SeqCst),s.authorized.load(SeqCst),s.retries.load(SeqCst),s.elapsed_ms.load(SeqCst));
    finish(&mut child, &ledger, 0);
}
#[test]
fn native_persistent_headroom() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut child = Child::start(job.0.0, "persistent"); let mut handled = setup(&mut child, &ledger, 0);
    let held = ledger.lock().unwrap().held[0]; start_grow(&child);
    loop {
        until(&mut child, |s| s.phase.load(SeqCst) == DONE || s.request.load(SeqCst) > handled);
        let s = shared(&child.view); if s.phase.load(SeqCst) == DONE { break; }
        handled = s.request.load(SeqCst); reply(&child, &ledger, 0, handled, Some(COMMIT));
    }
    let s = shared(&child.view); assert_eq!(s.result.load(SeqCst), COMMIT);
    assert!(s.elapsed_ms.load(SeqCst) >= 2000 && s.elapsed_ms.load(SeqCst) < 3000);
    assert!(s.retries.load(SeqCst) > 1); assert_eq!(s.old_intact.load(SeqCst), 1);
    assert_eq!(ledger.lock().unwrap().held[0], held); finish(&mut child, &ledger, 0);
}
#[test]
fn native_nonheadroom_no_retry() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut child = Child::start(job.0.0, "nonheadroom"); let mut handled = setup(&mut child, &ledger, 0); start_grow(&child);
    loop {
        until(&mut child, |s| s.phase.load(SeqCst) == DONE || s.request.load(SeqCst) > handled);
        let s = shared(&child.view); if s.phase.load(SeqCst) == DONE { break; }
        handled = s.request.load(SeqCst); reply(&child, &ledger, 0, handled, None);
    }
    let s = shared(&child.view); assert_eq!(s.result.load(SeqCst), OTHER);
    assert_eq!(s.faults.load(SeqCst), 1); assert_eq!(s.retries.load(SeqCst), 0);
    assert_eq!(s.old_intact.load(SeqCst), 1); assert_eq!(s.request.load(SeqCst), handled);
    finish(&mut child, &ledger, 0);
}
#[test]
fn native_cancel_and_reset() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut child = Child::start(job.0.0, "cancel_granted"); let handled = setup(&mut child, &ledger, 0);
    start_grow(&child); until(&mut child, |s| s.request.load(SeqCst) > handled);
    let epoch = shared(&child.view).request.load(SeqCst); reply(&child, &ledger, 0, epoch, None);
    until(&mut child, |s| s.phase.load(SeqCst) == AT_GATE);
    let held = ledger.lock().unwrap().held[0];
    let actual_before = ledger.lock().unwrap().actual[0];
    { let mut l = ledger.lock().unwrap(); l.reset(0).unwrap(); assert_eq!(l.grant_to(0, 1, held, held), Err(Failure::Stale)); }
    let s = shared(&child.view); s.cancel.store(1, SeqCst); s.verdict.store(GRANT, SeqCst); s.reply.store(epoch, SeqCst); win32::signal(child.command.0);
    until(&mut child, |s| s.phase.load(SeqCst) == DONE);
    let s = shared(&child.view); assert_eq!(s.result.load(SeqCst), CANCELLED); assert_eq!(s.old_intact.load(SeqCst), 1); assert_eq!(s.actual.load(SeqCst) as usize, actual_before);
    assert_eq!(ledger.lock().unwrap().held[0], held); finish(&mut child, &ledger, 0);
}
#[test]
fn two_native_workers() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut a = Child::start(job.0.0, "blocked"); let mut b = Child::start(job.0.0, "blocked");
    let ea = setup(&mut a, &ledger, 0); let eb = setup(&mut b, &ledger, 1);
    let actual_before = ledger.lock().unwrap().actual;
    start_grow(&a); start_grow(&b);
    until(&mut a, |s| s.request.load(SeqCst) > ea); until(&mut b, |s| s.request.load(SeqCst) > eb);
    let ra = shared(&a.view).request.load(SeqCst); let rb = shared(&b.view).request.load(SeqCst);
    // Reserve both under the same lock but hold the replies. Cancellation wins
    // even if a late grant arrives, and neither liability is released yet.
    { let mut l = ledger.lock().unwrap();
        for (i, c) in [(0, &a), (1, &b)] { let target = l.held[i] + shared(&c.view).amount.load(SeqCst) as usize; let gen = l.generation[i]; l.grant_to(i,gen,target,target).unwrap(); l.cancel(i); }
    }
    for (c, epoch) in [(&a,ra),(&b,rb)] { let s=shared(&c.view); s.cancel.store(1, SeqCst); s.verdict.store(GRANT, SeqCst); s.reply.store(epoch, SeqCst); win32::signal(c.command.0); }
    until(&mut a, |s| s.phase.load(SeqCst) == DONE); until(&mut b, |s| s.phase.load(SeqCst) == DONE);
    assert_eq!(shared(&a.view).result.load(SeqCst), CANCELLED); assert_eq!(shared(&b.view).result.load(SeqCst), CANCELLED);
    assert_eq!(shared(&a.view).actual.load(SeqCst) as usize, actual_before[0]); assert_eq!(shared(&b.view).actual.load(SeqCst) as usize, actual_before[1]);
    { let l = ledger.lock().unwrap(); assert!(l.held[0] > 0 && l.held[1] > 0); }
    finish(&mut a, &ledger, 0); assert!(ledger.lock().unwrap().held[1] > 0); finish(&mut b, &ledger, 1);
}
#[test]
fn blocked_child_termination() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut child = Child::start(job.0.0, "blocked"); let handled = setup(&mut child, &ledger, 0);
    start_grow(&child); until(&mut child, |s| s.request.load(SeqCst) > handled);
    let held = { let mut l = ledger.lock().unwrap(); let target = l.held[0] + shared(&child.view).amount.load(SeqCst) as usize; l.grant_to(0,1,target,target).unwrap(); target };
    child.process.terminate_and_wait(); assert_eq!(child.process.code(), 91);
    assert_eq!(ledger.lock().unwrap().held[0], held); ledger.lock().unwrap().observed_exit(0);
}
#[test]
fn child_failure_negative_control() {
    let job = Job(win32::job()); let mut child = Child::start(job.0.0, "negative");
    assert!(child.process.exited(5000), "negative child did not exit"); assert_ne!(child.process.code(), 0, "negative child failure was swallowed");
}
#[test]
fn native_separate_deadline() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut child = Child::start(job.0.0, "normal"); let mut handled = setup(&mut child, &ledger, 0);
    // A prior unrelated operation's window is expired; growth gets its own.
    std::thread::sleep(Duration::from_millis(2100)); start_grow(&child);
    loop {
        until(&mut child, |s| s.phase.load(SeqCst) == DONE || s.request.load(SeqCst) > handled);
        let s = shared(&child.view); if s.phase.load(SeqCst) == DONE { break; }
        handled = s.request.load(SeqCst); reply(&child, &ledger, 0, handled, None);
    }
    assert_eq!(shared(&child.view).result.load(SeqCst), OK);
    finish(&mut child, &ledger, 0);
}
#[test]
fn native_commit_retry_deadline() {
    let job = Job(win32::job()); let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    let mut child = Child::start(job.0.0, "commit_timeout"); let mut handled = setup(&mut child, &ledger, 0); start_grow(&child);
    let mut delayed = false;
    loop {
        until(&mut child, |s| s.phase.load(SeqCst) == DONE || s.request.load(SeqCst) > handled);
        let s = shared(&child.view); if s.phase.load(SeqCst) == DONE { break; }
        handled = s.request.load(SeqCst);
        // Delay the first grant: restarting the window after this grant or
        // its injected commit failure would take over3s and fail the witness.
        if !delayed { std::thread::sleep(Duration::from_millis(1100)); delayed = true; }
        reply(&child, &ledger, 0, handled, None);
    }
    let s = shared(&child.view); assert_eq!(s.result.load(SeqCst), SYNTHETIC_HEADROOM);
    assert_eq!(s.faults.load(SeqCst), 1); assert_eq!(s.retries.load(SeqCst), 1);
    assert_eq!(s.old_intact.load(SeqCst), 1);
    assert!(s.elapsed_ms.load(SeqCst) >= 2000 && s.elapsed_ms.load(SeqCst) < 3000);
    assert_eq!(ledger.lock().unwrap().held[0], s.authorized.load(SeqCst) as usize);
    finish(&mut child, &ledger, 0);
}
