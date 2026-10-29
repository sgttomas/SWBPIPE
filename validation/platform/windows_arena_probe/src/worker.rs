//! Two real child processes; shared event delivery is not an acknowledgement.
use crate::{arena::Arena, ledger::{Failure, Ledger}, win32};
use std::{sync::{atomic::{AtomicU32, Ordering::SeqCst}, Mutex}, time::{Duration, Instant}};
const MAP_SIZE: u32 = 4096;
#[repr(C)]
struct Shared { request: AtomicU32, grant: AtomicU32, cancel: AtomicU32, phase: AtomicU32, page: AtomicU32, committed: AtomicU32 }
impl Shared { fn new() -> Self { Self { request: AtomicU32::new(0), grant: AtomicU32::new(0), cancel: AtomicU32::new(0), phase: AtomicU32::new(0), page: AtomicU32::new(0), committed: AtomicU32::new(0) } } }
fn shared(view: &win32::View) -> &Shared { unsafe { &*view.0.cast::<Shared>() } }
fn handle(name: &str) -> win32::Raw { std::env::var(format!("NATIVE_FIXTURE_{name}")).unwrap().parse::<usize>().unwrap() as win32::Raw }
#[test]
fn child_entry() {
    if std::env::var("NATIVE_FIXTURE_MODE").is_err() { return; }
    let view = win32::map(handle("MAPPING"), MAP_SIZE as usize);
    let s = shared(&view); let command = handle("COMMAND"); let changed = handle("CHANGED");
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut epoch = 0;
    let mut admit = |n: usize| {
        epoch += 1; s.page.store(n as u32, SeqCst); s.request.store(epoch, SeqCst); win32::signal(changed);
        loop {
            if s.cancel.load(SeqCst) != 0 { return Err(Failure::Cancelled); }
            if s.grant.load(SeqCst) == epoch {
                if s.cancel.load(SeqCst) != 0 { return Err(Failure::Cancelled); }
                return Ok(());
            }
            assert!(Instant::now() < deadline, "child grant deadline");
            win32::wait(command, 20);
        }
    };
    let result = (|| { let mut arena = Arena::new(&mut admit)?; arena.alloc(8192, 64, &mut admit, false)?;
        let (_,_,data,meta,_,_) = arena.stats(); s.committed.store((data+meta) as u32, SeqCst); s.phase.store(1, SeqCst); win32::signal(changed);
        let finish_deadline = Instant::now() + Duration::from_secs(8);
        while s.cancel.load(SeqCst) == 0 { assert!(Instant::now() < finish_deadline, "child finish deadline"); win32::wait(command, 20); }
        Ok::<(), Failure>(()) })();
    if result == Err(Failure::Cancelled) { s.phase.store(2, SeqCst); win32::signal(changed); return; }
    result.unwrap();
}
struct Child { process: win32::Process, _map: win32::Handle, view: win32::View, command: win32::Handle, changed: win32::Handle }
impl Child {
    fn start(job: win32::Raw) -> Self {
        let (map, view) = win32::mapping(MAP_SIZE); unsafe { view.0.cast::<Shared>().write(Shared::new()); }
        let command = win32::event(); let changed = win32::event();
        let hs = [map.0, command.0, changed.0];
        let values = [("MAPPING", map.value()), ("COMMAND", command.value()), ("CHANGED", changed.value())];
        let mut process = win32::spawn_suspended(job, &hs, "arena", &values); process.resume();
        Self { process, _map: map, view, command, changed }
    }
}
struct Job(win32::Handle);
impl Drop for Job { fn drop(&mut self) { win32::drain_job(self.0.0); } }
fn until(child: &mut Child, predicate: impl Fn(&Shared) -> bool) {
    let end = Instant::now() + Duration::from_secs(5);
    loop { if predicate(shared(&child.view)) { return; } assert!(!child.process.exited(0), "child exited before phase"); assert!(Instant::now() < end, "child phase deadline"); win32::wait(child.changed.0, 20); }
}
#[test]
fn two_native_workers() {
    let job = Job(win32::job());
    let mut a = Child::start(job.0.0); let mut b = Child::start(job.0.0);
    let ledger = Mutex::new(Ledger::new(1024*1024, 1024*1024));
    until(&mut a, |s| s.request.load(SeqCst) == 1); until(&mut b, |s| s.request.load(SeqCst) == 1);
    let p = shared(&a.view).page.load(SeqCst) as usize;
    assert_eq!(shared(&b.view).page.load(SeqCst) as usize, p);
    { let mut l = ledger.lock().unwrap(); assert_eq!(l.grant(0,p,p), Ok(p)); assert_eq!(l.grant(1,p,p), Ok(p)); l.cancelled[1] = true; }
    // Publish a deliberately late grant after cancellation. It must not commit.
    shared(&b.view).cancel.store(1, SeqCst); shared(&b.view).grant.store(1, SeqCst); win32::signal(b.command.0);
    until(&mut b, |s| s.phase.load(SeqCst) == 2);
    assert_eq!(shared(&b.view).committed.load(SeqCst), 0);
    assert_eq!(ledger.lock().unwrap().held[1], p);
    assert!(b.process.exited(5000)); assert_eq!(b.process.code(), 0); ledger.lock().unwrap().observed_exit(1);
    shared(&a.view).grant.store(1, SeqCst); win32::signal(a.command.0);
    let mut granted = 1;
    loop {
        until(&mut a, |s| s.phase.load(SeqCst) == 1 || s.request.load(SeqCst) > granted);
        let s = shared(&a.view);
        if s.phase.load(SeqCst) == 1 { break; }
        let epoch = s.request.load(SeqCst); assert_eq!(epoch, granted+1);
        assert_eq!(ledger.lock().unwrap().grant(0,p,p), Ok(p));
        granted = epoch; s.grant.store(epoch, SeqCst); win32::signal(a.command.0);
    }
    let committed = shared(&a.view).committed.load(SeqCst) as usize;
    { let mut l = ledger.lock().unwrap(); l.committed(0, committed).unwrap(); assert_eq!(l.held[0], committed); }
    // Cancellation and a termination request do not release a live child's lease.
    shared(&a.view).cancel.store(1, SeqCst); win32::signal(a.command.0);
    assert_eq!(ledger.lock().unwrap().held[0], committed);
    assert!(a.process.exited(5000)); assert_eq!(a.process.code(), 0); ledger.lock().unwrap().observed_exit(0);
    assert_eq!(ledger.lock().unwrap().held, [0,0]);
    println!("two child admissions page={p} committed={committed}, cancellation held until observed exit");
}
