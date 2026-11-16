//! Host facts and process readings with no tool outside the binary: `sysctl`, `uname`,
//! `getloadavg`, `proc_pid_rusage`, `proc_listallpids`, `proc_name` and `proc_listpids`, all
//! through `libc`. The Python runner gets the same facts from `sysctl`, `sw_vers`, `ps` and
//! `pgrep` (`k6_runner.py` `metadata`, `memorystatus_level`, `host_busy`, `ps_rss_kib`,
//! `survivors_of`) and from `ctypes` (`release_runner.py` `read_footprint`).

/// One `proc_pid_rusage(pid, RUSAGE_INFO_V4)` reading of a process: the struct fields the Python
/// `RusageInfoV4` reads, and its resident size (which the Python takes from `ps -o rss=`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reading {
    pub phys_footprint: u64,
    pub lifetime_max_phys_footprint: u64,
    pub resident_size: u64,
}

/// `proc_pid_rusage(pid, RUSAGE_INFO_V4, ...)`; `None` off macOS, on a failed read, or once the
/// process has been reaped. On an exited child not yet reaped (a zombie) the kernel returns the
/// rusage kept at exit: `phys_footprint` 0 and the final lifetime maximum.
pub fn rusage_v4(pid: i32) -> Option<Reading> {
    #[cfg(target_os = "macos")]
    {
        let mut info = std::mem::MaybeUninit::<libc::rusage_info_v4>::zeroed();
        // SAFETY: `info` is a writable rusage_info_v4, the struct RUSAGE_INFO_V4 fills.
        if unsafe { libc::proc_pid_rusage(pid, libc::RUSAGE_INFO_V4, info.as_mut_ptr().cast()) } != 0 {
            return None;
        }
        // SAFETY: zeroed is a valid value and the kernel filled it on success.
        let info = unsafe { info.assume_init() };
        Some(Reading {
            phys_footprint: info.ri_phys_footprint,
            lifetime_max_phys_footprint: info.ri_lifetime_max_phys_footprint,
            resident_size: info.ri_resident_size,
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = pid;
        None
    }
}

/// `sysctlbyname` of a string value.
pub fn sysctl_string(name: &str) -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let name = std::ffi::CString::new(name).ok()?;
        let mut size: libc::size_t = 0;
        // SAFETY: a size query (null buffer) of a NUL-terminated name.
        if unsafe { libc::sysctlbyname(name.as_ptr(), std::ptr::null_mut(), &mut size, std::ptr::null_mut(), 0) } != 0 {
            return None;
        }
        let mut buffer = vec![0u8; size];
        // SAFETY: `buffer` holds `size` writable bytes.
        if unsafe { libc::sysctlbyname(name.as_ptr(), buffer.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0) } != 0 {
            return None;
        }
        buffer.truncate(size);
        while buffer.last() == Some(&0) {
            buffer.pop();
        }
        Some(String::from_utf8_lossy(&buffer).trim().to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = name;
        None
    }
}

/// `sysctlbyname` of an integer value (4 or 8 bytes, as the kernel gives it).
pub fn sysctl_int(name: &str) -> Option<i64> {
    #[cfg(target_os = "macos")]
    {
        let name = std::ffi::CString::new(name).ok()?;
        let mut value = [0u8; 8];
        let mut size: libc::size_t = value.len();
        // SAFETY: `value` holds 8 writable bytes, enough for an int or a 64-bit integer.
        if unsafe { libc::sysctlbyname(name.as_ptr(), value.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0) } != 0 {
            return None;
        }
        match size {
            4 => Some(i64::from(i32::from_ne_bytes(value[..4].try_into().ok()?))),
            8 => Some(i64::from_ne_bytes(value)),
            _ => None,
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = name;
        None
    }
}

/// `kern.memorystatus_level` (`k6.memorystatus_level`: `sysctl -n kern.memorystatus_level`).
pub fn memorystatus_level() -> Option<i64> {
    sysctl_int("kern.memorystatus_level")
}

/// `getloadavg` (`os.getloadavg()`): the 1, 5 and 15 minute load averages.
pub fn load_average() -> [f64; 3] {
    #[cfg(unix)]
    {
        let mut load = [0f64; 3];
        // SAFETY: `load` holds three writable doubles.
        unsafe { libc::getloadavg(load.as_mut_ptr(), 3) };
        load
    }
    #[cfg(not(unix))]
    {
        [0f64; 3]
    }
}

/// `uname`'s system name, release and machine (`platform.system()`, `platform.release()`,
/// `platform.machine()`).
pub fn uname() -> (Option<String>, Option<String>, Option<String>) {
    #[cfg(unix)]
    {
        let mut name = std::mem::MaybeUninit::<libc::utsname>::zeroed();
        // SAFETY: `name` is a writable utsname.
        if unsafe { libc::uname(name.as_mut_ptr()) } != 0 {
            return (None, None, None);
        }
        // SAFETY: zeroed is valid and uname filled it.
        let name = unsafe { name.assume_init() };
        let field = |chars: &[libc::c_char]| {
            let bytes: Vec<u8> = chars.iter().take_while(|c| **c != 0).map(|c| *c as u8).collect();
            Some(String::from_utf8_lossy(&bytes).to_string())
        };
        (field(&name.sysname), field(&name.release), field(&name.machine))
    }
    #[cfg(not(unix))]
    {
        (None, None, None)
    }
}

/// Every process id on the host (`proc_listallpids`).
#[cfg(target_os = "macos")]
fn all_pids() -> Vec<i32> {
    // SAFETY: a count query with a null buffer.
    let count = unsafe { libc::proc_listallpids(std::ptr::null_mut(), 0) };
    if count <= 0 {
        return Vec::new();
    }
    let mut pids = vec![0i32; count as usize + 64];
    let bytes = (pids.len() * std::mem::size_of::<i32>()) as libc::c_int;
    // SAFETY: `pids` holds `bytes` writable bytes.
    let found = unsafe { libc::proc_listallpids(pids.as_mut_ptr().cast(), bytes) };
    pids.truncate(found.max(0) as usize);
    pids.retain(|pid| *pid > 0);
    pids
}

/// A process's name (`proc_name`), the name `pgrep -x` matches.
#[cfg(target_os = "macos")]
fn process_name(pid: i32) -> Option<String> {
    let mut buffer = [0u8; 256];
    // SAFETY: `buffer` holds 256 writable bytes.
    let length = unsafe { libc::proc_name(pid, buffer.as_mut_ptr().cast(), buffer.len() as u32) };
    (length > 0).then(|| String::from_utf8_lossy(&buffer[..length as usize]).to_string())
}

/// A process's argument list joined by spaces, the text `pgrep -f` matches
/// (`sysctl` `KERN_PROCARGS2`); `None` where it cannot be read (another user's process).
#[cfg(target_os = "macos")]
fn process_arguments(pid: i32, buffer: &mut Vec<u8>) -> Option<String> {
    let mut mib = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid];
    let mut size: libc::size_t = buffer.len();
    // SAFETY: `buffer` holds `size` writable bytes; the mib names one process's arguments.
    if unsafe { libc::sysctl(mib.as_mut_ptr(), 3, buffer.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0) } != 0 {
        return None;
    }
    let data = &buffer[..size];
    let argc = i32::from_ne_bytes(data.get(..4)?.try_into().ok()?);
    // After argc: the executable path, NUL padding, then argc NUL-terminated arguments.
    let mut rest = &data[4..];
    let path_end = rest.iter().position(|b| *b == 0)?;
    rest = &rest[path_end..];
    let start = rest.iter().position(|b| *b != 0)?;
    rest = &rest[start..];
    let mut arguments = Vec::new();
    for piece in rest.split(|b| *b == 0).take(argc.max(0) as usize) {
        arguments.push(String::from_utf8_lossy(piece).to_string());
    }
    Some(arguments.join(" "))
}

/// Whether `text` matches the Python's `pgrep -f 'python[0-9.]* .*run_evidence_sweep'` (an
/// unanchored search).
pub fn matches_sweep(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut from = 0;
    while let Some(at) = text[from..].find("python") {
        let mut j = from + at + "python".len();
        while j < bytes.len() && (bytes[j].is_ascii_digit() || bytes[j] == b'.') {
            j += 1;
        }
        if bytes.get(j) == Some(&b' ') && text[j + 1..].contains("run_evidence_sweep") {
            return true;
        }
        from += at + 1;
    }
    false
}

/// `k6.host_busy`: another cargo job (`pgrep -x cargo`) or a DEC-025 sweep
/// (`pgrep -f 'python[0-9.]* .*run_evidence_sweep'`), read from the process table instead of
/// `pgrep`. Like `pgrep`, it leaves out its own process.
pub fn host_busy() -> Vec<String> {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: getpid has no preconditions.
        let me = unsafe { libc::getpid() };
        let pids: Vec<i32> = all_pids().into_iter().filter(|pid| *pid != me).collect();
        let mut busy = Vec::new();
        if pids.iter().any(|pid| process_name(*pid).as_deref() == Some("cargo")) {
            busy.push("cargo".to_string());
        }
        let mut buffer = vec![0u8; sysctl_int("kern.argmax").unwrap_or(1 << 20).clamp(4096, 16 << 20) as usize];
        if pids.iter().any(|pid| process_arguments(*pid, &mut buffer).is_some_and(|args| matches_sweep(&args))) {
            busy.push("sweep".to_string());
        }
        busy
    }
    #[cfg(not(target_os = "macos"))]
    {
        Vec::new()
    }
}

/// The processes in process group `pgid` (`pgrep -g`), as `proc_listpids(PROC_PGRP_ONLY)`.
pub fn group_members(pgid: i32) -> Vec<i32> {
    #[cfg(target_os = "macos")]
    {
        const PROC_PGRP_ONLY: u32 = 2; // <sys/proc_info.h>
        let mut pids = vec![0i32; 256];
        let bytes = (pids.len() * std::mem::size_of::<i32>()) as libc::c_int;
        // SAFETY: `pids` holds `bytes` writable bytes.
        let found = unsafe { libc::proc_listpids(PROC_PGRP_ONLY, pgid as u32, pids.as_mut_ptr().cast(), bytes) };
        let count = (found.max(0) as usize) / std::mem::size_of::<i32>();
        pids.truncate(count);
        pids.retain(|pid| *pid > 0);
        pids
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = pgid;
        Vec::new()
    }
}
