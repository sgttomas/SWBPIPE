//! K6's counting, capped global allocator (K6 plan §5; ROOT's K6 ruling N8).
//!
//! It is registered only by the observation binary and by the `k6_alloc`
//! test binary, never by the harness library, so no consumer inherits it.
//!
//! - `CURRENT` is the requested bytes alive now. Two peaks are kept, both
//!   deterministic for a given binary and input:
//!   - `PEAK` (the in-place model): a growing `realloc` requests only the
//!     difference, as the Mac gate's heap-cap probe counts it
//!     (`T3/PLATFORM_CALIBRATION_MAC/gate/heap_cap_appended_to_p1_probe.rs.txt`).
//!     The cap is enforced on this model;
//!   - `PEAK_MOVE` (the move model): during a growing `realloc` the old and
//!     the new block are alive together, as F1b's 24-byte estimate assumes.
//! - `STAGE_PEAK` and `STAGE_PEAK_MOVE` restart at `stage_reset`.
//! - A request that would take `CURRENT` above `CAP` is refused: a fixed
//!   marker is written to fd 2 without allocating, and null is returned, so
//!   Rust's allocation-error path aborts the process (SIGABRT). The marker
//!   separates a heap-cap abort from an `RLIMIT_AS` failure, which prints only
//!   Rust's own line.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static PEAK_MOVE: AtomicUsize = AtomicUsize::new(0);
static STAGE_PEAK: AtomicUsize = AtomicUsize::new(0);
static STAGE_PEAK_MOVE: AtomicUsize = AtomicUsize::new(0);
static CAP: AtomicUsize = AtomicUsize::new(usize::MAX);
static CALLS: AtomicU64 = AtomicU64::new(0);

/// The counting, capped allocator over `System`.
pub struct K6Alloc;

/// Sets the heap cap (requested bytes alive at once).
pub fn set_cap(bytes: usize) {
    CAP.store(bytes, Ordering::SeqCst);
}
pub fn cap() -> usize {
    CAP.load(Ordering::SeqCst)
}
pub fn current() -> usize {
    CURRENT.load(Ordering::SeqCst)
}
pub fn peak() -> usize {
    PEAK.load(Ordering::SeqCst)
}
pub fn peak_move() -> usize {
    PEAK_MOVE.load(Ordering::SeqCst)
}
pub fn stage_peak() -> usize {
    STAGE_PEAK.load(Ordering::SeqCst)
}
pub fn stage_peak_move() -> usize {
    STAGE_PEAK_MOVE.load(Ordering::SeqCst)
}
/// Allocation calls that succeeded (alloc, alloc_zeroed, realloc).
#[allow(dead_code)] // observation binary (stage lines); unused by the k6_alloc test binary
pub fn calls() -> u64 {
    CALLS.load(Ordering::SeqCst)
}
/// Restarts the stage peaks at the current level.
pub fn stage_reset() {
    let now = current();
    STAGE_PEAK.store(now, Ordering::SeqCst);
    STAGE_PEAK_MOVE.store(now, Ordering::SeqCst);
}
/// Restarts every peak at the current level (tests only).
#[allow(dead_code)] // k6_alloc test binary
pub fn reset_peaks() {
    let now = current();
    PEAK.store(now, Ordering::SeqCst);
    PEAK_MOVE.store(now, Ordering::SeqCst);
    stage_reset();
}

fn note_peak(level: usize) {
    PEAK.fetch_max(level, Ordering::SeqCst);
    STAGE_PEAK.fetch_max(level, Ordering::SeqCst);
    note_move(level);
}

fn note_move(level: usize) {
    PEAK_MOVE.fetch_max(level, Ordering::SeqCst);
    STAGE_PEAK_MOVE.fetch_max(level, Ordering::SeqCst);
}

/// Adds `bytes` to `CURRENT` unless that exceeds the cap; returns the new level.
fn reserve(bytes: usize) -> Option<usize> {
    let cap = cap();
    let mut now = CURRENT.load(Ordering::SeqCst);
    loop {
        let next = now.checked_add(bytes)?;
        if next > cap {
            return None;
        }
        match CURRENT.compare_exchange_weak(now, next, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return Some(next),
            Err(seen) => now = seen,
        }
    }
}

fn release(bytes: usize) {
    CURRENT.fetch_sub(bytes, Ordering::SeqCst);
}

/// Formats `value` in decimal into `buf` at `at`; returns the new end.
fn put_usize(buf: &mut [u8], mut at: usize, value: usize) -> usize {
    let mut digits = [0u8; 20];
    let mut n = value;
    let mut len = 0;
    loop {
        digits[len] = b'0' + (n % 10) as u8;
        len += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    for k in (0..len).rev() {
        if at < buf.len() {
            buf[at] = digits[k];
            at += 1;
        }
    }
    at
}

fn put_bytes(buf: &mut [u8], mut at: usize, bytes: &[u8]) -> usize {
    for &b in bytes {
        if at < buf.len() {
            buf[at] = b;
            at += 1;
        }
    }
    at
}

/// Writes `k6_observe: heap cap refused <size> bytes (current <c>, cap <cap>)`
/// to fd 2 from a stack buffer: no allocation.
fn refuse(size: usize) {
    let mut buf = [0u8; 160];
    let mut at = put_bytes(&mut buf, 0, b"k6_observe: heap cap refused ");
    at = put_usize(&mut buf, at, size);
    at = put_bytes(&mut buf, at, b" bytes (current ");
    at = put_usize(&mut buf, at, current());
    at = put_bytes(&mut buf, at, b", cap ");
    at = put_usize(&mut buf, at, cap());
    at = put_bytes(&mut buf, at, b")\n");
    write_stderr(&buf[..at]);
}

#[cfg(unix)]
fn write_stderr(bytes: &[u8]) {
    use std::io::Write;
    use std::os::unix::io::FromRawFd;
    // SAFETY: fd 2 is the process's stderr; the `File` is forgotten, never
    // closed, and writing a byte slice does not allocate.
    let mut file = unsafe { std::fs::File::from_raw_fd(2) };
    let _ = file.write_all(bytes);
    std::mem::forget(file);
}

#[cfg(not(unix))]
fn write_stderr(_bytes: &[u8]) {}

unsafe impl GlobalAlloc for K6Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let Some(level) = reserve(size) else {
            refuse(size);
            return std::ptr::null_mut();
        };
        let ptr = System.alloc(layout);
        if ptr.is_null() {
            release(size);
        } else {
            note_peak(level);
            CALLS.fetch_add(1, Ordering::SeqCst);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let Some(level) = reserve(size) else {
            refuse(size);
            return std::ptr::null_mut();
        };
        let ptr = System.alloc_zeroed(layout);
        if ptr.is_null() {
            release(size);
        } else {
            note_peak(level);
            CALLS.fetch_add(1, Ordering::SeqCst);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        release(layout.size());
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old = layout.size();
        if new_size > old {
            let Some(level) = reserve(new_size - old) else {
                refuse(new_size - old);
                return std::ptr::null_mut();
            };
            let moved = System.realloc(ptr, layout, new_size);
            if moved.is_null() {
                release(new_size - old);
            } else {
                note_peak(level);
                // The move model: the old block is alive beside the new one.
                note_move(level + old);
                CALLS.fetch_add(1, Ordering::SeqCst);
            }
            moved
        } else {
            let moved = System.realloc(ptr, layout, new_size);
            if !moved.is_null() {
                release(old - new_size);
                CALLS.fetch_add(1, Ordering::SeqCst);
            }
            moved
        }
    }
}
