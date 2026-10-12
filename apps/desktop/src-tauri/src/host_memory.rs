//! The host's memory reader for the aggregate pool (macOS; every other target
//! reports unknown values, which refuse admission).
//!
//! - Capacity: `sysctlbyname("hw.memsize")` (u64; equals `ProcessInfo.physicalMemory`).
//! - Headroom (conservative): only the kernel free queue counts, never
//!   speculative, purgeable, inactive, file-backed or compressed pages, nor
//!   the pressure percentage. `host_statistics64(HOST_VM_INFO64)` must
//!   succeed with the full 38-word count; then
//!   `free_list = free_count - speculative_count` (free_count includes
//!   speculative pages, mach/vm_statistics.h), the kernel's own reserve is
//!   `vm.vm_page_free_target` pages, and
//!   `headroom = (free_list - reserve) * vm_page_size` (saturating at 0 for
//!   the subtractions, checked for the product).
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

/// Pure headroom arithmetic over the kernel's counters.
pub(crate) fn conservative_headroom(free_count: u64, speculative_count: u64, free_target: u64, page_size: u64) -> Option<u64> {
    free_count.saturating_sub(speculative_count).saturating_sub(free_target).checked_mul(page_size)
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
    use super::{conservative_headroom, pressure_from_level};
    use crate::memory_pool::PressureLevel;
    use std::ffi::{c_int, c_void, CStr};
    use std::mem::{size_of, MaybeUninit};
    use std::sync::OnceLock;

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

    pub(super) fn headroom_bytes() -> Option<u64> {
        let stats = vm_statistics()?;
        let free_target = u64::try_from(sysctl::<c_int>(c"vm.vm_page_free_target")?).ok()?;
        let free_count = stats.free_count as u64;
        let speculative_count = stats.speculative_count as u64;
        conservative_headroom(free_count, speculative_count, free_target, page_size()?)
    }

    pub(super) fn pressure() -> PressureLevel {
        pressure_from_level(sysctl::<c_int>(c"kern.memorystatus_vm_pressure_level"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headroom_arithmetic_counts_only_the_free_list_above_the_kernel_reserve() {
        assert_eq!(conservative_headroom(1000, 200, 300, 16384), Some(500 * 16384));
        assert_eq!(conservative_headroom(100, 200, 0, 16384), Some(0), "speculative pages never count");
        assert_eq!(conservative_headroom(400, 0, 500, 16384), Some(0), "the reserve saturates at zero");
        assert_eq!(conservative_headroom(u64::MAX, 0, 0, 2), None, "the product is checked");
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
        assert_ne!(sample.pressure, PressureLevel::Unknown, "kern.memorystatus_vm_pressure_level readable");
    }
}
