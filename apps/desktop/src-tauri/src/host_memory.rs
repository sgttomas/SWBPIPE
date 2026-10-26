//! The host's memory reader for the aggregate pool (macOS; every other target
//! reports unknown values, which refuse admission).
//!
//! - Capacity: `sysctlbyname("hw.memsize")` (u64; equals `ProcessInfo.physicalMemory`).
//! - Headroom (owner, 2026-10-26: memory macOS can reclaim cheaply also
//!   counts; DEL-04-07 `memory-budget-decision.md`, "Headroom counting"):
//!   `host_statistics64(HOST_VM_INFO64)` must succeed with the full 38-word
//!   count. It gives the free list `free_count - speculative_count`
//!   (free_count includes speculative pages, mach/vm_statistics.h) and the
//!   purgeable pages `purgeable_count` (resident, unwired pages of volatile
//!   purgeable objects); `vm.page_pageable_external_count` gives the
//!   file-backed pages on the pageable queues (wired file pages excluded).
//!   The kernel's own reserve is `vm.vm_page_free_target` pages, subtracted
//!   once from the total, and
//!   `headroom = (free_list + external + purgeable - reserve) * vm_page_size`
//!   (saturating at 0 for the subtractions, checked for the sum and the
//!   product). No page is counted twice: a free page belongs to no object,
//!   speculative pages are kept out of the free list, and purgeable objects
//!   are anonymous, never file-backed. Inactive, anonymous, compressed and
//!   wired pages are never counted as such, nor the pressure percentage.
//!   An unreadable `vm.page_pageable_external_count`, or a total above the
//!   capacity, falls back to the free list alone (the rule before 2026-10-26).
//! - Pressure: `kern.memorystatus_vm_pressure_level`: 1 normal, 2 warning,
//!   4 critical; anything else or a failed read is unknown.
//!
//! These reads need no entitlement.

use crate::memory_pool::{HostMemory, HostSample, PressureLevel};
use std::time::Instant;

#[derive(Debug, Default)]
pub(crate) struct MacHostMemory;

impl HostMemory for MacHostMemory {
    fn sample(&self) -> HostSample {
        #[cfg(target_os = "macos")]
        {
            let taken_at = Instant::now();
            HostSample {
                capacity_bytes: mac::capacity_bytes(),
                headroom_bytes: mac::headroom_bytes(),
                pressure: mac::pressure(),
                taken_at,
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            HostSample { capacity_bytes: None, headroom_bytes: None, pressure: PressureLevel::Unknown, taken_at: Instant::now() }
        }
    }
}

/// The kernel's counters one headroom reading uses, in pages (bytes for the
/// page size and capacity). `pageable_external` is `None` when its read failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct VmCounts {
    pub(crate) free_count: u64,
    pub(crate) speculative_count: u64,
    pub(crate) purgeable_count: u64,
    pub(crate) pageable_external: Option<u64>,
    pub(crate) free_target: u64,
    pub(crate) page_size: u64,
    pub(crate) capacity_bytes: Option<u64>,
}

/// The free list above the kernel's reserve, the rule before 2026-10-26 and
/// the fallback of [`reclaimable_headroom`].
pub(crate) fn free_list_headroom(c: &VmCounts) -> Option<u64> {
    c.free_count.saturating_sub(c.speculative_count).saturating_sub(c.free_target).checked_mul(c.page_size)
}

/// Pure headroom arithmetic over the kernel's counters (see the module docs).
pub(crate) fn reclaimable_headroom(c: &VmCounts) -> Option<u64> {
    let fallback = free_list_headroom(c);
    let Some(external) = c.pageable_external else { return fallback };
    let free_list = c.free_count.saturating_sub(c.speculative_count);
    let counted = free_list.checked_add(external).and_then(|n| n.checked_add(c.purgeable_count));
    let counted_bytes = counted.and_then(|n| n.checked_mul(c.page_size));
    match (counted, counted_bytes, c.capacity_bytes) {
        (Some(pages), Some(bytes), Some(capacity)) if bytes <= capacity => pages.saturating_sub(c.free_target).checked_mul(c.page_size),
        _ => fallback,
    }
}

pub(crate) fn pressure_from_level(level: Option<i32>) -> PressureLevel {
    match level {
        Some(1) => PressureLevel::Normal,
        Some(2) => PressureLevel::Warning,
        Some(4) => PressureLevel::Critical,
        _ => PressureLevel::Unknown,
    }
}

#[cfg(target_os = "macos")]
mod mac {
    use super::{pressure_from_level, reclaimable_headroom, VmCounts};
    use crate::memory_pool::PressureLevel;
    use std::ffi::{c_int, c_void, CStr};
    use std::mem::{size_of, MaybeUninit};
    use std::sync::OnceLock;

    // `host_statistics64` writes HOST_VM_INFO64_COUNT integer_t (4-byte) words
    // into the buffer: the struct must be exactly that size.
    const _: () = assert!(size_of::<libc::vm_statistics64>() == libc::HOST_VM_INFO64_COUNT as usize * 4);

    // libc marks `mach_host_self` deprecated; declare it directly.
    extern "C" {
        fn mach_host_self() -> libc::mach_port_t;
    }

    fn sysctl<T: Copy + Default>(name: &CStr) -> Option<T> {
        let mut value = T::default();
        let mut len = size_of::<T>();
        // SAFETY: `value` is a valid, writable T of `len` bytes; the name is NUL-terminated.
        let rc = unsafe {
            libc::sysctlbyname(name.as_ptr(), &mut value as *mut T as *mut c_void, &mut len, std::ptr::null_mut(), 0)
        };
        (rc == 0 && len == size_of::<T>()).then_some(value)
    }

    pub(super) fn capacity_bytes() -> Option<u64> {
        sysctl::<u64>(c"hw.memsize").filter(|v| *v > 0)
    }

    /// The host port, obtained once and held for the process lifetime.
    fn host_port() -> Option<libc::mach_port_t> {
        static PORT: OnceLock<libc::mach_port_t> = OnceLock::new();
        // SAFETY: `mach_host_self` has no preconditions.
        let port = *PORT.get_or_init(|| unsafe { mach_host_self() });
        (port != 0).then_some(port)
    }

    fn vm_statistics() -> Option<libc::vm_statistics64> {
        let port = host_port()?;
        let mut stats = MaybeUninit::<libc::vm_statistics64>::zeroed();
        let mut count: libc::mach_msg_type_number_t = libc::HOST_VM_INFO64_COUNT;
        // SAFETY: `stats` provides HOST_VM_INFO64_COUNT integer_t words; `count` says so.
        let kr = unsafe {
            libc::host_statistics64(port, libc::HOST_VM_INFO64, stats.as_mut_ptr() as libc::host_info64_t, &mut count)
        };
        if kr != libc::KERN_SUCCESS || count != libc::HOST_VM_INFO64_COUNT {
            return None;
        }
        // SAFETY: the kernel filled the full structure (checked count); zeroed is a valid value anyway.
        Some(unsafe { stats.assume_init() })
    }

    pub(super) fn page_size() -> Option<u64> {
        // SAFETY: `vm_page_size` is a process-constant exported by libSystem.
        let size = unsafe { libc::vm_page_size } as u64;
        (size > 0).then_some(size)
    }

    /// One reading of the counters [`reclaimable_headroom`] uses; `None` when
    /// a counter the free-list rule needs is unreadable (headroom unknown).
    pub(super) fn vm_counts() -> Option<VmCounts> {
        let stats = vm_statistics()?;
        let free_target = u64::try_from(sysctl::<c_int>(c"vm.vm_page_free_target")?).ok()?;
        Some(VmCounts {
            free_count: stats.free_count as u64,
            speculative_count: stats.speculative_count as u64,
            purgeable_count: stats.purgeable_count as u64,
            // A u32 counter (`vm_page_pageable_external_count`); a failed or
            // short read falls back to the free list alone.
            pageable_external: sysctl::<u32>(c"vm.page_pageable_external_count").map(u64::from),
            free_target,
            page_size: page_size()?,
            capacity_bytes: capacity_bytes(),
        })
    }

    pub(super) fn headroom_bytes() -> Option<u64> {
        reclaimable_headroom(&vm_counts()?)
    }

    pub(super) fn pressure() -> PressureLevel {
        pressure_from_level(sysctl::<c_int>(c"kern.memorystatus_vm_pressure_level"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: u64 = 16384;

    fn counts(free: u64, speculative: u64, purgeable: u64, external: Option<u64>, target: u64) -> VmCounts {
        VmCounts {
            free_count: free,
            speculative_count: speculative,
            purgeable_count: purgeable,
            pageable_external: external,
            free_target: target,
            page_size: PAGE,
            capacity_bytes: Some(128 << 30),
        }
    }

    #[test]
    fn free_list_rule_counts_only_the_free_list_above_the_kernel_reserve() {
        assert_eq!(free_list_headroom(&counts(1000, 200, 0, None, 300)), Some(500 * PAGE));
        assert_eq!(free_list_headroom(&counts(100, 200, 0, None, 0)), Some(0), "speculative pages never count");
        assert_eq!(free_list_headroom(&counts(400, 0, 0, None, 500)), Some(0), "the reserve saturates at zero");
        assert_eq!(free_list_headroom(&VmCounts { page_size: 2, ..counts(u64::MAX, 0, 0, None, 0) }), None, "the product is checked");
    }

    #[test]
    fn reclaimable_rule_adds_pageable_file_backed_and_purgeable_pages_once() {
        // free 1000 of which 200 speculative; 300 purgeable; 5000 pageable external; reserve 400.
        assert_eq!(reclaimable_headroom(&counts(1000, 200, 300, Some(5000), 400)), Some((800 + 5000 + 300 - 400) * PAGE));
        // Speculative pages count only through E: raising them moves pages out of
        // the free list and never adds them twice.
        let all_speculative = counts(1000, 1000, 0, Some(1000), 0);
        assert_eq!(reclaimable_headroom(&all_speculative), Some(1000 * PAGE), "a speculative file page counts once, in E");
        // Inactive and anonymous pages are not inputs: only F, E and P move the result.
        // The reserve is subtracted once, from the total, not from the free list alone.
        assert_eq!(reclaimable_headroom(&counts(100, 0, 0, Some(1000), 400)), Some(700 * PAGE));
        assert_eq!(reclaimable_headroom(&counts(0, 0, 0, Some(100), 400)), Some(0), "the reserve saturates at zero");
        // This host on 2026-10-26 (vm.page_free_count, which excludes speculative pages, as F):
        // a free list just above the reserve beside a large file cache.
        assert_eq!(reclaimable_headroom(&counts(8990, 0, 29_276, Some(2_462_954), 4000)),
            Some((8990 + 2_462_954 + 29_276 - 4000) * PAGE));
    }

    #[test]
    fn reclaimable_rule_is_never_below_the_free_list_rule() {
        for free in [0, 3999, 4000, 4001, 100_000] {
            for speculative in [0, 1, 50_000] {
                for purgeable in [0, 7, 30_000] {
                    for external in [None, Some(0), Some(1), Some(2_500_000)] {
                        let c = counts(free, speculative, purgeable, external, 4000);
                        assert!(reclaimable_headroom(&c) >= free_list_headroom(&c), "{c:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn reclaimable_rule_fails_closed_to_the_free_list() {
        let fallback = Some((1000 - 400) * PAGE);
        assert_eq!(reclaimable_headroom(&counts(1000, 0, 300, None, 400)), fallback, "E unreadable: purgeable is not counted either");
        let implausible = VmCounts { capacity_bytes: Some(4000 * PAGE), ..counts(1000, 0, 300, Some(5000), 400) };
        assert_eq!(reclaimable_headroom(&implausible), fallback, "a total above capacity is implausible");
        let unknown_capacity = VmCounts { capacity_bytes: None, ..counts(1000, 0, 300, Some(5000), 400) };
        assert_eq!(reclaimable_headroom(&unknown_capacity), fallback);
        let overflow = counts(1000, 0, u64::MAX, Some(5000), 400);
        assert_eq!(reclaimable_headroom(&overflow), fallback, "an overflowing sum falls back");
        let at_capacity = VmCounts { capacity_bytes: Some(6300 * PAGE), ..counts(1000, 0, 300, Some(5000), 400) };
        assert_eq!(reclaimable_headroom(&at_capacity), Some(5900 * PAGE), "a total equal to capacity is accepted");
    }

    #[test]
    fn pressure_levels_map_to_their_states() {
        assert_eq!(pressure_from_level(Some(1)), PressureLevel::Normal);
        assert_eq!(pressure_from_level(Some(2)), PressureLevel::Warning);
        assert_eq!(pressure_from_level(Some(4)), PressureLevel::Critical);
        assert_eq!(pressure_from_level(Some(3)), PressureLevel::Unknown);
        assert_eq!(pressure_from_level(None), PressureLevel::Unknown);
    }

    /// Reads this host once (no allocation toward host limits).
    #[cfg(target_os = "macos")]
    #[test]
    fn mac_reader_establishes_capacity_headroom_and_pressure() {
        assert_eq!(libc::HOST_VM_INFO64_COUNT, 38);
        let sample = MacHostMemory.sample();
        let capacity = sample.capacity_bytes.expect("hw.memsize readable");
        assert!(capacity >= 1 << 30);
        let headroom = sample.headroom_bytes.expect("host_statistics64 and vm.vm_page_free_target readable");
        assert!(headroom <= capacity);
        let c = mac::vm_counts().expect("counters readable");
        assert!(c.pageable_external.is_some(), "vm.page_pageable_external_count readable");
        assert!(reclaimable_headroom(&c) >= free_list_headroom(&c));
        assert_ne!(sample.pressure, PressureLevel::Unknown, "kern.memorystatus_vm_pressure_level readable");
    }

    /// Prints the headroom both rules see on this host, sampled 20 times over
    /// 10 s (no allocation toward host limits). Run with `--ignored --nocapture`.
    #[cfg(target_os = "macos")]
    #[test]
    #[ignore = "a host report, not a check"]
    fn report_headroom_under_both_rules() {
        let mib = |b: Option<u64>| b.map_or("unknown".to_string(), |b| format!("{:.1} MiB", b as f64 / 1048576.0));
        for i in 0..20 {
            let c = mac::vm_counts().expect("counters readable");
            println!(
                "sample {i:2}: free-list rule {:>12}, reclaimable rule {:>12} (F {} E {:?} P {} T {} pages; pressure {:?})",
                mib(free_list_headroom(&c)),
                mib(reclaimable_headroom(&c)),
                c.free_count - c.speculative_count.min(c.free_count),
                c.pageable_external,
                c.purgeable_count,
                c.free_target,
                mac::pressure(),
            );
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
