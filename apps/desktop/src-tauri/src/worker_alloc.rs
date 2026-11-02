//! The process-wide allocator and the solve worker's heap cap (owner decision
//! 2026-10-12, "(c) Capped worker process"; DEL-04-07 memory-budget decision).
//!
//! In the desktop app the allocator is the system allocator plus one relaxed
//! load per call. A solve worker process ([`crate::solve_worker`]) enables
//! counting: every live allocation is counted by [`resident_envelope`] of its
//! size, a bound on the memory it can make resident, and a reallocation counts
//! its new size before the old one is released. When an allocation would push
//! the counted total above the heap cap (the grant less the worker's process
//! reserve), the allocating thread takes an allocation-free spinlock, re-checks
//! and performs a growth exchange with the parent over the raw control fds:
//! `GROW {requested_grant, phys_footprint}` answered by `GRNT {grant}` or
//! `DENY`. A refusal, or a need above the per-invocation budget, ends the
//! process with a final frame and `_exit(WORKER_CAP_EXIT)` before the
//! allocation is made: nothing above the cap is ever allocated, nothing
//! unwinds and nothing allocates on that path.
//!
//! What the count covers: the requested heap. Allocator metadata, freed blocks
//! the allocator keeps cached, the binary image, dyld and thread stacks are
//! outside it; the worker's process reserve covers them, and the parent
//! checks the measured physical footprint against the grant at every growth
//! exchange, so the reserve is enforced rather than assumed.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU64, Ordering};

/// Requests up to this size come from the allocator's fixed size classes.
pub(crate) const SIZE_CLASS_LIMIT: u64 = 32 * 1024;
/// Larger requests are page-granular (16 KiB pages on Apple silicon).
pub(crate) const LARGE_PAGE: u64 = 16 * 1024;

/// A bound on the memory one allocation of `size` bytes can make resident:
/// max(16, next_power_of_two(size)) for size-class requests (s <= 32 KiB) and
/// round_up(size, 16 KiB) for page-granular ones; 0 for 0. The size-class
/// half is [`crate::ingress::allocation_envelope`] (checked against this
/// host's `malloc_size` by `allocation_envelope_covers_this_hosts_malloc_size`);
/// the page-granular half is the resident-pages fact established there by
/// `envelope_covers_the_resident_pages_of_page_granular_blocks` (a
/// page-granular block is page-aligned, so touching s bytes makes at most
/// round_up(s, P) resident). Saturates instead of overflowing.
pub(crate) const fn resident_envelope(size: u64) -> u64 {
    if size == 0 {
        0
    } else if size <= SIZE_CLASS_LIMIT {
        let p = size.next_power_of_two();
        if p < 16 {
            16
        } else {
            p
        }
    } else {
        size.div_ceil(LARGE_PAGE).saturating_mul(LARGE_PAGE)
    }
}

/// Worker exit code after a cap refusal (a final refusal frame precedes it).
pub(crate) const WORKER_CAP_EXIT: i32 = 86;
/// Worker exit code after a broken control channel or protocol violation.
pub(crate) const WORKER_PROTOCOL_EXIT: i32 = 87;

/// Control frames: a 4-byte tag and three little-endian u64 fields (a, b, c;
/// unused fields are 0); result and error frames are followed by `a` body bytes.
pub(crate) const FRAME_BYTES: usize = 28;
/// Child -> parent: `a` = requested grant (the doubling step), `b` = the
/// minimal grant it needs (counted heap + reserve), `c` = its phys_footprint.
pub(crate) const TAG_GROW: [u8; 4] = *b"GROW";
/// Child -> parent: the need exceeds the budget; `a` = needed grant, `b` = maximum grant.
pub(crate) const TAG_CAP_EXCEEDED: [u8; 4] = *b"CAPX";
/// Child -> parent: the child acknowledges a refused growth; `a` = requested, `b` = grant.
pub(crate) const TAG_REFUSED: [u8; 4] = *b"RFSD";
/// Child -> parent, directly before `RSLT`: `a` = the solve attestation's
/// status (1 attested, 0 not), `b` = `c` = 0, then 64 bytes (`d_M ‖ d_S`,
/// [`crate::solve_attestation`]) when the status is 1 and none when it is 0.
pub(crate) const TAG_ATTESTATION: [u8; 4] = *b"ATST";
/// Child -> parent: `a` = result length R, `b` = the child's lifetime-maximum
/// phys_footprint, then R bytes.
pub(crate) const TAG_RESULT: [u8; 4] = *b"RSLT";
/// Child -> parent: `a` = error length, then that many UTF-8 bytes.
pub(crate) const TAG_ERROR: [u8; 4] = *b"FAIL";
/// Parent -> child: `a` = the new grant (at least the minimal need).
pub(crate) const TAG_GRANTED: [u8; 4] = *b"GRNT";
/// Parent -> child: the growth was refused.
pub(crate) const TAG_DENIED: [u8; 4] = *b"DENY";

pub(crate) fn encode_frame(tag: [u8; 4], a: u64, b: u64, c: u64) -> [u8; FRAME_BYTES] {
    let mut frame = [0u8; FRAME_BYTES];
    frame[..4].copy_from_slice(&tag);
    frame[4..12].copy_from_slice(&a.to_le_bytes());
    frame[12..20].copy_from_slice(&b.to_le_bytes());
    frame[20..].copy_from_slice(&c.to_le_bytes());
    frame
}

pub(crate) fn decode_frame(frame: &[u8; FRAME_BYTES]) -> ([u8; 4], u64, u64, u64) {
    let mut tag = [0u8; 4];
    tag.copy_from_slice(&frame[..4]);
    let field = |at: usize| u64::from_le_bytes(frame[at..at + 8].try_into().unwrap_or([0; 8]));
    (tag, field(4), field(12), field(20))
}

static ENABLED: AtomicBool = AtomicBool::new(false);
/// Live counted bytes (signed: frees of allocations made before counting
/// began subtract from it; those allocations sit inside the reserve).
static COUNTED: AtomicI64 = AtomicI64::new(0);
static PEAK: AtomicI64 = AtomicI64::new(0);
/// u64::MAX until armed: the request is read before the cap applies.
static HEAP_CAP: AtomicU64 = AtomicU64::new(u64::MAX);
static GRANT: AtomicU64 = AtomicU64::new(0);
static MAX_GRANT: AtomicU64 = AtomicU64::new(0);
static RESERVE: AtomicU64 = AtomicU64::new(0);
static CONTROL_IN: AtomicI32 = AtomicI32::new(-1);
static CONTROL_OUT: AtomicI32 = AtomicI32::new(-1);
static CHANNEL: AtomicBool = AtomicBool::new(false);

/// The allocation-free channel lock (never a std Mutex: it is taken inside
/// the allocator).
struct ChannelGuard;

impl ChannelGuard {
    fn take() -> Self {
        while CHANNEL.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            std::hint::spin_loop();
            #[cfg(unix)]
            // SAFETY: sched_yield has no preconditions and does not allocate.
            unsafe {
                libc::sched_yield();
            }
        }
        ChannelGuard
    }
}

impl Drop for ChannelGuard {
    fn drop(&mut self) {
        CHANNEL.store(false, Ordering::Release);
    }
}

#[cfg(unix)]
fn write_all_fd(fd: i32, mut bytes: &[u8]) -> bool {
    while !bytes.is_empty() {
        // SAFETY: `bytes` is a valid readable buffer of its length.
        let n = unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) };
        if n < 0 {
            if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return false;
        }
        if n == 0 {
            return false;
        }
        bytes = &bytes[n as usize..];
    }
    true
}

#[cfg(unix)]
fn read_exact_fd(fd: i32, mut buffer: &mut [u8]) -> bool {
    while !buffer.is_empty() {
        // SAFETY: `buffer` is a valid writable buffer of its length.
        let n = unsafe { libc::read(fd, buffer.as_mut_ptr().cast(), buffer.len()) };
        if n < 0 {
            if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return false;
        }
        if n == 0 {
            return false;
        }
        buffer = &mut buffer[n as usize..];
    }
    true
}

/// Ends the worker without unwinding, flushing or allocating.
fn exit_now(code: i32) -> ! {
    #[cfg(unix)]
    // SAFETY: _exit has no preconditions.
    unsafe {
        libc::_exit(code)
    }
    #[cfg(not(unix))]
    std::process::abort()
}

/// This process's physical footprint (the kernel's `phys_footprint` ledger,
/// as Activity Monitor's Memory column), read without allocating; 0 where
/// unavailable.
pub(crate) fn phys_footprint() -> u64 {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: getpid has no preconditions.
        pid_phys_footprint(unsafe { libc::getpid() })
    }
    #[cfg(not(target_os = "macos"))]
    {
        0
    }
}

/// The physical footprint of process `pid` (this process or a child), or 0.
#[cfg(target_os = "macos")]
pub(crate) fn pid_phys_footprint(pid: i32) -> u64 {
    let mut info = std::mem::MaybeUninit::<libc::rusage_info_v0>::zeroed();
    // SAFETY: `info` is a writable rusage_info_v0, the struct RUSAGE_INFO_V0 fills.
    let rc = unsafe { libc::proc_pid_rusage(pid, libc::RUSAGE_INFO_V0, info.as_mut_ptr().cast()) };
    if rc != 0 {
        return 0;
    }
    // SAFETY: zeroed is a valid value and the kernel filled it on success.
    unsafe { info.assume_init() }.ri_phys_footprint
}

/// The lifetime-maximum physical footprint of process `pid` (the kernel's
/// high-water mark, which also covers moments between growth exchanges), or 0.
#[cfg(target_os = "macos")]
pub(crate) fn pid_lifetime_max_footprint(pid: i32) -> u64 {
    let mut info = std::mem::MaybeUninit::<libc::rusage_info_v4>::zeroed();
    // SAFETY: `info` is a writable rusage_info_v4, the struct RUSAGE_INFO_V4 fills.
    let rc = unsafe { libc::proc_pid_rusage(pid, libc::RUSAGE_INFO_V4, info.as_mut_ptr().cast()) };
    if rc != 0 {
        return 0;
    }
    // SAFETY: zeroed is a valid value and the kernel filled it on success.
    unsafe { info.assume_init() }.ri_lifetime_max_phys_footprint
}

/// What process `pid` holds resident and private, bounded above:
/// min(resident size, physical footprint). The resident size includes shared
/// pages (not taken from the free queue for this process) and the footprint
/// includes compressed pages (which occupy no free page until decompressed);
/// each over-states, so the smaller is used (still an upper bound on private
/// resident memory). `None` if the read fails.
#[cfg(target_os = "macos")]
pub(crate) fn pid_realized_bytes(pid: i32) -> Option<u64> {
    let mut info = std::mem::MaybeUninit::<libc::rusage_info_v0>::zeroed();
    // SAFETY: `info` is a writable rusage_info_v0, the struct RUSAGE_INFO_V0 fills.
    let rc = unsafe { libc::proc_pid_rusage(pid, libc::RUSAGE_INFO_V0, info.as_mut_ptr().cast()) };
    if rc != 0 {
        return None;
    }
    // SAFETY: zeroed is a valid value and the kernel filled it on success.
    let info = unsafe { info.assume_init() };
    Some(info.ri_resident_size.min(info.ri_phys_footprint))
}

/// This process's lifetime-maximum physical footprint, or 0 where unavailable.
pub(crate) fn lifetime_max_footprint() -> u64 {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: getpid has no preconditions.
        pid_lifetime_max_footprint(unsafe { libc::getpid() })
    }
    #[cfg(not(target_os = "macos"))]
    {
        0
    }
}

/// Starts counting in a worker process. The cap is not yet armed.
pub(crate) fn enable(control_in: i32, control_out: i32) {
    CONTROL_IN.store(control_in, Ordering::SeqCst);
    CONTROL_OUT.store(control_out, Ordering::SeqCst);
    COUNTED.store(0, Ordering::SeqCst);
    PEAK.store(0, Ordering::SeqCst);
    ENABLED.store(true, Ordering::SeqCst);
}

/// Arms the cap at `grant - reserve`, growing at once if the counted heap
/// (the request already read) is above it.
pub(crate) fn arm(grant: u64, max_grant: u64, reserve: u64) {
    RESERVE.store(reserve, Ordering::SeqCst);
    MAX_GRANT.store(max_grant, Ordering::SeqCst);
    GRANT.store(grant, Ordering::SeqCst);
    HEAP_CAP.store(grant.saturating_sub(reserve), Ordering::SeqCst);
    if counted() > HEAP_CAP.load(Ordering::SeqCst) {
        grow_or_exit();
    }
}

fn counted() -> u64 {
    COUNTED.load(Ordering::Acquire).max(0) as u64
}

#[cfg(test)]
pub(crate) fn peak_counted() -> u64 {
    PEAK.load(Ordering::Relaxed).max(0) as u64
}

#[cfg(test)]
pub(crate) fn current_grant() -> u64 {
    GRANT.load(Ordering::Relaxed)
}

/// Writes one frame (and its body) to the parent under the channel lock.
/// Returns false if the channel is broken.
pub(crate) fn send_frame(tag: [u8; 4], a: u64, b: u64, body: &[u8]) -> bool {
    #[cfg(unix)]
    {
        let _guard = ChannelGuard::take();
        let fd = CONTROL_OUT.load(Ordering::SeqCst);
        write_all_fd(fd, &encode_frame(tag, a, b, 0)) && write_all_fd(fd, body)
    }
    #[cfg(not(unix))]
    {
        let _ = (tag, a, b, body);
        false
    }
}

/// Reads exactly `buffer.len()` bytes from the parent (the request, read
/// before the cap is armed and before any growth exchange can occur).
pub(crate) fn read_request(buffer: &mut [u8]) -> bool {
    #[cfg(unix)]
    {
        read_exact_fd(CONTROL_IN.load(Ordering::SeqCst), buffer)
    }
    #[cfg(not(unix))]
    {
        let _ = buffer;
        false
    }
}

/// Writes a final refusal frame and exits with [`WORKER_CAP_EXIT`]. Must be
/// called holding the channel lock (or before any other thread exists).
fn refuse_and_exit(tag: [u8; 4], a: u64, b: u64) -> ! {
    #[cfg(unix)]
    {
        let _ = write_all_fd(CONTROL_OUT.load(Ordering::SeqCst), &encode_frame(tag, a, b, 0));
    }
    #[cfg(not(unix))]
    let _ = (tag, a, b);
    exit_now(WORKER_CAP_EXIT)
}

/// Exits for a need above the budget before any request is read into memory.
pub(crate) fn refuse_need_above_budget(needed_grant: u64, max_grant: u64) -> ! {
    let _guard = ChannelGuard::take();
    refuse_and_exit(TAG_CAP_EXCEEDED, needed_grant, max_grant)
}

/// A growth exchange's terms for a counted heap of `counted` bytes:
/// (requested, needed) grants, where needed = counted + reserve and requested
/// = min(budget, max(needed, 2 * grant, footprint + reserve)). Saturating and
/// allocation-free, so exact for every budget up to u64::MAX.
#[inline]
pub(crate) fn growth_request(counted: u64, reserve: u64, grant: u64, max_grant: u64, footprint: u64) -> (u64, u64) {
    let needed_grant = counted.saturating_add(reserve);
    let requested = max_grant.min(needed_grant.max(grant.saturating_mul(2)).max(footprint.saturating_add(reserve)));
    (requested, needed_grant)
}

/// The slow path: under the channel lock, grows the grant until the counted
/// heap fits or ends the process. Never allocates.
#[cold]
fn grow_or_exit() {
    let _guard = ChannelGuard::take();
    loop {
        let needed = counted();
        let cap = HEAP_CAP.load(Ordering::Acquire);
        if needed <= cap {
            return;
        }
        let reserve = RESERVE.load(Ordering::Relaxed);
        let max_grant = MAX_GRANT.load(Ordering::Relaxed);
        let grant = GRANT.load(Ordering::Relaxed);
        if needed.saturating_add(reserve) > max_grant {
            refuse_and_exit(TAG_CAP_EXCEEDED, needed.saturating_add(reserve), max_grant);
        }
        // The measured footprint is a growth need too: memory the count does
        // not see (allocator-retained freed blocks, metadata) is still resident.
        let footprint = phys_footprint();
        let (requested, needed_grant) = growth_request(needed, reserve, grant, max_grant, footprint);
        #[cfg(unix)]
        {
            let out = CONTROL_OUT.load(Ordering::Relaxed);
            if !write_all_fd(out, &encode_frame(TAG_GROW, requested, needed_grant, footprint)) {
                exit_now(WORKER_PROTOCOL_EXIT);
            }
            let mut reply = [0u8; FRAME_BYTES];
            if !read_exact_fd(CONTROL_IN.load(Ordering::Relaxed), &mut reply) {
                exit_now(WORKER_PROTOCOL_EXIT);
            }
            match decode_frame(&reply) {
                (TAG_GRANTED, granted, _, _) if granted >= needed_grant && granted <= max_grant => {
                    GRANT.store(granted, Ordering::Release);
                    HEAP_CAP.store(granted.saturating_sub(reserve), Ordering::Release);
                }
                (TAG_DENIED, _, _, _) => refuse_and_exit(TAG_REFUSED, needed_grant, grant),
                _ => exit_now(WORKER_PROTOCOL_EXIT),
            }
        }
        #[cfg(not(unix))]
        exit_now(WORKER_PROTOCOL_EXIT);
    }
}

#[inline]
fn charge(size: usize) {
    let envelope = resident_envelope(size as u64).min(i64::MAX as u64) as i64;
    let total = COUNTED.fetch_add(envelope, Ordering::AcqRel).saturating_add(envelope);
    PEAK.fetch_max(total, Ordering::Relaxed);
    if total > 0 && total as u64 > HEAP_CAP.load(Ordering::Acquire) {
        grow_or_exit();
    }
}

#[inline]
fn uncharge(size: usize) {
    let envelope = resident_envelope(size as u64).min(i64::MAX as u64) as i64;
    COUNTED.fetch_sub(envelope, Ordering::AcqRel);
}

/// The process-wide allocator: [`System`], counted and capped in a worker.
pub(crate) struct CappedAllocator;

/// The counted size of a layout: alignment above the size is served from a
/// class at least that large.
#[inline]
fn counted_size(layout: &Layout) -> usize {
    layout.size().max(layout.align())
}

// SAFETY: every call forwards to `System` with the caller's layout; the
// accounting never touches the returned memory and never allocates.
unsafe impl GlobalAlloc for CappedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let enabled = ENABLED.load(Ordering::Relaxed);
        if enabled {
            charge(counted_size(&layout));
        }
        // SAFETY: forwarded unchanged.
        let ptr = unsafe { System.alloc(layout) };
        if ptr.is_null() && enabled {
            uncharge(counted_size(&layout));
        }
        #[cfg(any(test, feature = "native-smoke"))]
        if !ptr.is_null() {
            crate::ingress::counting::record(layout.size(), 0, true);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let enabled = ENABLED.load(Ordering::Relaxed);
        if enabled {
            charge(counted_size(&layout));
        }
        // SAFETY: forwarded unchanged.
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if ptr.is_null() && enabled {
            uncharge(counted_size(&layout));
        }
        #[cfg(any(test, feature = "native-smoke"))]
        if !ptr.is_null() {
            crate::ingress::counting::record(layout.size(), 0, true);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: forwarded unchanged.
        unsafe { System.dealloc(ptr, layout) };
        if ENABLED.load(Ordering::Relaxed) {
            uncharge(counted_size(&layout));
        }
        #[cfg(any(test, feature = "native-smoke"))]
        crate::ingress::counting::record(0, layout.size(), false);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let enabled = ENABLED.load(Ordering::Relaxed);
        let new_counted = new_size.max(layout.align());
        if enabled {
            // The new block is counted before the old one is released: both
            // are live across a moving reallocation.
            charge(new_counted);
        }
        // SAFETY: forwarded unchanged.
        let new = unsafe { System.realloc(ptr, layout, new_size) };
        if enabled {
            uncharge(if new.is_null() { new_counted } else { counted_size(&layout) });
        }
        #[cfg(any(test, feature = "native-smoke"))]
        if !new.is_null() {
            crate::ingress::counting::record(new_size, layout.size(), true);
        }
        new
    }
}

#[global_allocator]
static GLOBAL: CappedAllocator = CappedAllocator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resident_envelope_prices_size_classes_and_page_granular_blocks() {
        assert_eq!(resident_envelope(0), 0);
        assert_eq!(resident_envelope(1), 16);
        assert_eq!(resident_envelope(17), 32);
        assert_eq!(resident_envelope(32 * 1024), 32 * 1024);
        assert_eq!(resident_envelope(32 * 1024 + 1), 48 * 1024);
        assert_eq!(resident_envelope(64 * 1024 + 1), 80 * 1024);
        assert_eq!(resident_envelope(u64::MAX), u64::MAX, "saturates");
        for size in [1u64, 15, 16, 100, 257, 4097, 32 * 1024] {
            assert_eq!(Some(resident_envelope(size)), crate::ingress::allocation_envelope(size), "{size}");
        }
        // Page-granular: never below the size, never above the power-of-two envelope.
        for size in (32 * 1024 + 1..=1 << 22).step_by(4093) {
            let envelope = resident_envelope(size);
            assert!(envelope >= size && envelope.is_multiple_of(LARGE_PAGE));
            assert!(envelope <= crate::ingress::allocation_envelope(size).unwrap());
        }
    }

    #[test]
    fn frames_round_trip() {
        let frame = encode_frame(TAG_GROW, 1 << 40, 7, u64::MAX);
        assert_eq!(decode_frame(&frame), (TAG_GROW, 1 << 40, 7, u64::MAX));
        assert_eq!(&frame[..4], b"GROW");
    }

    /// The desktop process itself never counts: the cap is only armed in a worker.
    #[test]
    fn the_app_process_is_uncapped() {
        assert!(!ENABLED.load(Ordering::Relaxed));
        assert_eq!(HEAP_CAP.load(Ordering::Relaxed), u64::MAX);
        let block = vec![0u8; 1 << 20];
        assert_eq!(COUNTED.load(Ordering::Relaxed), 0);
        drop(block);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn phys_footprint_reads_this_process() {
        let footprint = phys_footprint();
        assert!(footprint > 0 && footprint < 1 << 40, "{footprint}");
    }
}
