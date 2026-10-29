//! External controller for each libtest case, not an allocator/solver worker.
#[cfg(all(windows, target_arch = "x86_64"))]
#[path = "../win32.rs"]
mod win32;

#[cfg(all(windows, target_arch = "x86_64"))]
fn main() {
    use std::{sync::atomic::{AtomicU32, Ordering::SeqCst}, time::{Duration, Instant}};
    struct OwnedJob(win32::Handle);
    impl Drop for OwnedJob { fn drop(&mut self) { win32::drain_job(self.0.0); } }
    let args: Vec<_> = std::env::args_os().collect();
    assert_eq!(args.len(), 3, "guard requires test executable and exact case");
    let case = args[2].to_str().expect("ASCII case name");
    let negative = case == "worker::cleanup_timeout_entry";
    let job = OwnedJob(win32::job());
    let (mapping, view) = win32::mapping(4096);
    // A fixed control page records that the negative case actually has a
    // running descendant, before the controller intentionally times it out.
    unsafe { view.0.cast::<AtomicU32>().write(AtomicU32::new(0)); }
    let ready = unsafe { &*view.0.cast::<AtomicU32>() };
    let changed = win32::event();
    let values = [("GUARD_MAPPING", mapping.value()), ("GUARD_CHANGED", changed.value())];
    let mut process = win32::spawn_case(job.0.0, &[mapping.0, changed.0], std::path::Path::new(&args[1]), case, &values);
    process.resume();
    eprintln!("phase=external-custody case={case}");
    let code = if negative {
        let start = Instant::now();
        while ready.load(SeqCst) == 0 {
            assert!(!process.exited(0), "cleanup negative control exited before worker ready");
            assert!(start.elapsed() < Duration::from_secs(5), "cleanup negative control setup timeout");
            win32::wait(changed.0, 20);
        }
        assert!(win32::job_active(job.0.0) >= 2, "negative control lacks owned case and arena child");
        assert!(!process.exited(200), "negative control must still be running at timeout");
        win32::drain_job(job.0.0);
        assert!(process.exited(5000), "case exit not observed after external Job drain");
        assert_eq!(process.code(), 93, "negative case not ended by Job cleanup");
        println!("ARENA_TIMEOUT_NEGATIVE_CONFIRMED case={case}");
        0
    } else if process.exited(30_000) {
        process.code()
    } else {
        eprintln!("phase=external-timeout case={case}");
        win32::drain_job(job.0.0);
        assert!(process.exited(5000), "case exit not observed after timeout");
        124
    };
    // Even a failed or exited case may have left arena children. The
    // controller holds independent custody until their active count is zero.
    win32::drain_job(job.0.0);
    assert_eq!(win32::job_active(job.0.0), 0);
    assert!(process.exited(0));
    println!("ARENA_CLEANUP_CONFIRMED case={case} active=0");
    drop(process); drop(job); drop(view); drop(mapping); drop(changed);
    std::process::exit(code as i32);
}

#[cfg(not(all(windows, target_arch = "x86_64")))]
fn main() { eprintln!("native x64 Windows required"); std::process::exit(2); }
