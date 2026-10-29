//! Minimal documented x64 Win32 bindings for the fixture. No production bridge.
use std::{
    ffi::{OsStr, c_void},
    mem::{size_of, zeroed},
    os::windows::ffi::OsStrExt,
    ptr,
};

pub type Raw = *mut c_void;
const NULL: Raw = ptr::null_mut();
const INVALID: Raw = -1isize as Raw;
const WAIT_OBJECT_0: u32 = 0;
const WAIT_TIMEOUT: u32 = 258;
const CREATE_SUSPENDED: u32 = 4;
const CREATE_UNICODE_ENVIRONMENT: u32 = 0x400;
const EXTENDED_STARTUPINFO_PRESENT: u32 = 0x80000;
const DUPLICATE_SAME_ACCESS: u32 = 2;
const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x2000;
const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: usize = 0x20002;
const PROC_THREAD_ATTRIBUTE_JOB_LIST: usize = 0x2000d;

#[repr(C)]
struct Security {
    length: u32,
    descriptor: Raw,
    inherit: i32,
}
#[repr(C)]
#[derive(Default)]
struct BasicLimit {
    process_time: i64,
    job_time: i64,
    flags: u32,
    min_working: usize,
    max_working: usize,
    active_limit: u32,
    affinity: usize,
    priority: u32,
    scheduling: u32,
}
#[repr(C)]
#[derive(Default)]
struct JobAccounting {
    total_user: i64,
    total_kernel: i64,
    period_user: i64,
    period_kernel: i64,
    faults: u32,
    total_processes: u32,
    active_processes: u32,
    terminated_processes: u32,
}
#[repr(C)]
#[derive(Default)]
struct IoCounters {
    read_ops: u64,
    write_ops: u64,
    other_ops: u64,
    read_bytes: u64,
    write_bytes: u64,
    other_bytes: u64,
}
#[repr(C)]
#[derive(Default)]
struct ExtendedLimit {
    basic: BasicLimit,
    io: IoCounters,
    process_memory: usize,
    job_memory: usize,
    peak_process: usize,
    peak_job: usize,
}
#[repr(C)]
struct Startup {
    cb: u32,
    reserved: *mut u16,
    desktop: *mut u16,
    title: *mut u16,
    x: u32,
    y: u32,
    xsize: u32,
    ysize: u32,
    xcount: u32,
    ycount: u32,
    fill: u32,
    flags: u32,
    show: u16,
    reserved2_len: u16,
    reserved2: *mut u8,
    stdin: Raw,
    stdout: Raw,
    stderr: Raw,
}
#[repr(C)]
struct StartupEx {
    startup: Startup,
    attributes: Raw,
}
#[repr(C)]
struct ProcessInformation {
    process: Raw,
    thread: Raw,
    pid: u32,
    tid: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetLastError() -> u32;
    fn CloseHandle(h: Raw) -> i32;
    fn GetCurrentProcess() -> Raw;
    fn GetStdHandle(kind: u32) -> Raw;
    fn DuplicateHandle(
        source_process: Raw,
        source: Raw,
        target_process: Raw,
        target: *mut Raw,
        access: u32,
        inherit: i32,
        options: u32,
    ) -> i32;
    fn CreateFileW(
        name: *const u16,
        access: u32,
        share: u32,
        sa: *const Security,
        creation: u32,
        flags: u32,
        template: Raw,
    ) -> Raw;
    fn CreateJobObjectW(sa: *const Security, name: *const u16) -> Raw;
    fn SetInformationJobObject(job: Raw, class: u32, info: *const c_void, len: u32) -> i32;
    fn IsProcessInJob(process: Raw, job: Raw, result: *mut i32) -> i32;
    fn QueryInformationJobObject(
        job: Raw,
        class: u32,
        info: Raw,
        length: u32,
        returned: *mut u32,
    ) -> i32;
    fn TerminateJobObject(job: Raw, code: u32) -> i32;
    fn Sleep(milliseconds: u32);
    fn CreateFileMappingW(
        file: Raw,
        sa: *const Security,
        protect: u32,
        high: u32,
        low: u32,
        name: *const u16,
    ) -> Raw;
    fn MapViewOfFile(mapping: Raw, access: u32, high: u32, low: u32, length: usize) -> Raw;
    fn UnmapViewOfFile(address: *const c_void) -> i32;
    fn CreateEventW(sa: *const Security, manual: i32, initial: i32, name: *const u16) -> Raw;
    fn SetEvent(event: Raw) -> i32;
    fn CreateMutexW(sa: *const Security, initial: i32, name: *const u16) -> Raw;
    fn ReleaseMutex(mutex: Raw) -> i32;
    fn WaitForSingleObject(h: Raw, timeout: u32) -> u32;
    fn InitializeProcThreadAttributeList(
        list: Raw,
        count: u32,
        flags: u32,
        size: *mut usize,
    ) -> i32;
    fn UpdateProcThreadAttribute(
        list: Raw,
        flags: u32,
        attribute: usize,
        value: Raw,
        size: usize,
        previous: Raw,
        returned: *mut usize,
    ) -> i32;
    fn DeleteProcThreadAttributeList(list: Raw);
    fn CreateProcessW(
        app: *const u16,
        command: *mut u16,
        process_sa: Raw,
        thread_sa: Raw,
        inherit: i32,
        flags: u32,
        environment: Raw,
        cwd: *const u16,
        startup: *mut Startup,
        info: *mut ProcessInformation,
    ) -> i32;
    fn ResumeThread(thread: Raw) -> u32;
    fn TerminateProcess(process: Raw, code: u32) -> i32;
    fn GetExitCodeProcess(process: Raw, code: *mut u32) -> i32;
}
fn error(what: &str) -> ! {
    panic!("{what}: Win32 error {}", unsafe { GetLastError() })
}
fn ok(v: i32, what: &str) {
    if v == 0 {
        error(what)
    }
}
fn sa() -> Security {
    Security {
        length: size_of::<Security>() as u32,
        descriptor: NULL,
        inherit: 1,
    }
}

pub struct Handle(pub Raw);
// Win32 handles are process-wide and can be transferred to another thread;
// ownership stays unique and Drop closes the handle once.
unsafe impl Send for Handle {}
impl Handle {
    pub fn new(raw: Raw, what: &str) -> Self {
        if raw.is_null() || raw == INVALID {
            error(what)
        }
        Self(raw)
    }
    pub fn value(&self) -> usize {
        self.0 as usize
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

pub struct View(pub Raw);
impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile(self.0);
        }
    }
}
pub fn mapping(bytes: u32) -> (Handle, View) {
    let handle = Handle::new(
        unsafe { CreateFileMappingW(INVALID, &sa(), 4, 0, bytes, ptr::null()) },
        "CreateFileMappingW",
    );
    let view = map(handle.0, bytes as usize);
    (handle, view)
}
pub fn map(handle: Raw, bytes: usize) -> View {
    let p = unsafe { MapViewOfFile(handle, 0xF001F, 0, 0, bytes) };
    if p.is_null() {
        error("MapViewOfFile")
    }
    View(p)
}
pub fn event() -> Handle {
    Handle::new(
        unsafe { CreateEventW(&sa(), 0, 0, ptr::null()) },
        "CreateEventW",
    )
}
pub fn mutex() -> Handle {
    Handle::new(
        unsafe { CreateMutexW(&sa(), 0, ptr::null()) },
        "CreateMutexW",
    )
}
pub fn signal(h: Raw) {
    ok(unsafe { SetEvent(h) }, "SetEvent")
}
pub fn release_mutex(h: Raw) {
    ok(unsafe { ReleaseMutex(h) }, "ReleaseMutex")
}
pub fn wait(h: Raw, ms: u32) -> bool {
    match unsafe { WaitForSingleObject(h, ms) } {
        WAIT_OBJECT_0 => true,
        WAIT_TIMEOUT => false,
        _ => error("WaitForSingleObject"),
    }
}
pub fn duplicate_local(source: Raw) -> Handle {
    let mut out = NULL;
    ok(
        unsafe {
            DuplicateHandle(
                GetCurrentProcess(),
                source,
                GetCurrentProcess(),
                &mut out,
                0,
                0,
                DUPLICATE_SAME_ACCESS,
            )
        },
        "DuplicateHandle locally",
    );
    Handle::new(out, "duplicated local handle")
}
pub fn duplicate_from(source_process: Raw, source: Raw) -> Handle {
    let mut out = NULL;
    ok(
        unsafe {
            DuplicateHandle(
                source_process,
                source,
                GetCurrentProcess(),
                &mut out,
                0,
                0,
                DUPLICATE_SAME_ACCESS,
            )
        },
        "DuplicateHandle from caretaker",
    );
    Handle::new(out, "duplicated caretaker handle")
}
pub fn job() -> Handle {
    let h = Handle::new(
        unsafe { CreateJobObjectW(ptr::null(), ptr::null()) },
        "CreateJobObjectW",
    );
    let mut limits = ExtendedLimit::default();
    limits.basic.flags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    ok(
        unsafe {
            SetInformationJobObject(
                h.0,
                9,
                (&limits as *const ExtendedLimit).cast(),
                size_of::<ExtendedLimit>() as u32,
            )
        },
        "SetInformationJobObject",
    );
    h
}
/// External cleanup backstop: terminate any remaining member and observe
/// the Job's active-process count reach zero even when no worker handle was
/// published before a caretaker failure. The loop is bounded.
pub fn job_active(job: Raw) -> u32 {
        let mut a = JobAccounting::default();
        ok(
            unsafe {
                QueryInformationJobObject(
                    job,
                    1,
                    (&mut a as *mut JobAccounting).cast(),
                    size_of::<JobAccounting>() as u32,
                    ptr::null_mut(),
                )
            },
            "QueryInformationJobObject",
        );
        a.active_processes
}
pub fn drain_job(job: Raw) {
    if job_active(job) == 0 {
        return;
    }
    if unsafe { TerminateJobObject(job, 93) } == 0 {
        eprintln!("fixture cleanup: TerminateJobObject failed");
        std::process::abort();
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while job_active(job) != 0 {
        if std::time::Instant::now() >= deadline {
            eprintln!("fixture cleanup: Job still has active processes");
            std::process::abort();
        }
        unsafe {
            Sleep(10);
        }
    }
}
pub fn is_in_job(process: Raw, job: Raw) -> bool {
    let mut result = 0;
    ok(
        unsafe { IsProcessInJob(process, job, &mut result) },
        "IsProcessInJob",
    );
    result != 0
}

struct Attributes {
    storage: Vec<usize>,
    initialized: bool,
}
impl Drop for Attributes {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                DeleteProcThreadAttributeList(self.storage.as_mut_ptr().cast());
            }
        }
    }
}
fn wide(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(Some(0)).collect()
}

/// The only process launcher: suspended at creation inside the supplied Job,
/// and inheriting only the explicit handles. No breakaway or assign fallback.
pub fn spawn_suspended(job: Raw, handles: &[Raw], mode: &str, values: &[(&str, usize)]) -> Process {
    let exe = std::env::current_exe().expect("current executable");
    spawn_exact(job, handles, &exe, "worker::child_entry", mode, values, false)
}
/// External case controller: keep its Job/observation capability outside the
/// tested process, including when that process aborts or is force-terminated.
pub fn spawn_case(job: Raw, handles: &[Raw], exe: &std::path::Path, case: &str, values: &[(&str, usize)]) -> Process {
    assert!(!case.is_empty() && case.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b':'));
    spawn_exact(job, handles, exe, case, "case", values, true)
}
fn spawn_exact(job: Raw, handles: &[Raw], exe: &std::path::Path, case: &str, mode: &str, values: &[(&str, usize)], forward_stdout: bool) -> Process {
    assert_eq!(size_of::<Startup>(), 104);
    assert_eq!(size_of::<StartupEx>(), 112);
    assert_eq!(size_of::<BasicLimit>(), 64);
    assert_eq!(size_of::<ExtendedLimit>(), 144);
    // libtest needs valid standard streams. The inherited NUL handle is
    // explicitly allowlisted, and does not expose an ambient parent handle.
    let nul_name = wide(OsStr::new("NUL"));
    let nul = Handle::new(
        unsafe { CreateFileW(nul_name.as_ptr(), 0xc0000000, 3, &sa(), 3, 0x80, NULL) },
        "CreateFileW NUL",
    );
    let mut inherited = handles.to_vec();
    inherited.push(nul.0);
    // Keep child failure diagnostics live without giving it an ambient input
    // or stdout handle (libtest's child summary must not mimic a parent pass).
    let mut stderr_raw = NULL;
    ok(
        unsafe {
            DuplicateHandle(
                GetCurrentProcess(),
                GetStdHandle((-12i32) as u32),
                GetCurrentProcess(),
                &mut stderr_raw,
                0,
                1,
                DUPLICATE_SAME_ACCESS,
            )
        },
        "duplicate stderr",
    );
    let stderr = Handle::new(stderr_raw, "duplicated stderr");
    inherited.push(stderr.0);
    let stdout = if forward_stdout {
        let mut raw = NULL;
        ok(unsafe { DuplicateHandle(GetCurrentProcess(), GetStdHandle((-11i32) as u32), GetCurrentProcess(), &mut raw, 0, 1, DUPLICATE_SAME_ACCESS) }, "duplicate stdout");
        Some(Handle::new(raw, "duplicated stdout"))
    } else { None };
    if let Some(h) = &stdout { inherited.push(h.0); }
    let mut bytes = 0usize;
    unsafe {
        InitializeProcThreadAttributeList(NULL, 2, 0, &mut bytes);
    }
    assert!(bytes > 0, "attribute-list sizing failed");
    let mut a = Attributes {
        storage: vec![0usize; (bytes + size_of::<usize>() - 1) / size_of::<usize>()],
        initialized: false,
    };
    let ap = a.storage.as_mut_ptr().cast();
    ok(
        unsafe { InitializeProcThreadAttributeList(ap, 2, 0, &mut bytes) },
        "InitializeProcThreadAttributeList",
    );
    a.initialized = true;
    ok(
        unsafe {
            UpdateProcThreadAttribute(
                ap,
                0,
                PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
                inherited.as_ptr() as Raw,
                std::mem::size_of_val(inherited.as_slice()),
                NULL,
                ptr::null_mut(),
            )
        },
        "handle-list attribute",
    );
    let jobs = [job];
    ok(
        unsafe {
            UpdateProcThreadAttribute(
                ap,
                0,
                PROC_THREAD_ATTRIBUTE_JOB_LIST,
                jobs.as_ptr() as Raw,
                size_of::<Raw>(),
                NULL,
                ptr::null_mut(),
            )
        },
        "job-list attribute",
    );
    let mut command = wide(OsStr::new(&format!(
        "\"{}\" --exact {} --nocapture --test-threads=1",
        exe.display(), case
    )));
    let app = wide(exe.as_os_str());
    let mut env: Vec<(std::ffi::OsString, std::ffi::OsString)> = std::env::vars_os()
        .filter(|(k, _)| !k.to_string_lossy().starts_with("NATIVE_FIXTURE_"))
        .collect();
    env.push(("NATIVE_FIXTURE_MODE".into(), mode.into()));
    for (k, v) in values {
        env.push((format!("NATIVE_FIXTURE_{k}").into(), v.to_string().into()));
    }
    env.sort_by_key(|(k, _)| k.to_string_lossy().to_ascii_uppercase());
    let mut block = Vec::<u16>::new();
    for (k, v) in env {
        block.extend(k.encode_wide());
        block.push(b'=' as u16);
        block.extend(v.encode_wide());
        block.push(0);
    }
    block.push(0);
    let mut startup: StartupEx = unsafe { zeroed() };
    startup.startup.cb = size_of::<StartupEx>() as u32;
    startup.startup.flags = 0x100; // STARTF_USESTDHANDLES
    startup.startup.stdin = nul.0;
    startup.startup.stdout = stdout.as_ref().map_or(nul.0, |h| h.0);
    startup.startup.stderr = stderr.0;
    startup.attributes = ap;
    let mut pi: ProcessInformation = unsafe { zeroed() };
    ok(
        unsafe {
            CreateProcessW(
                app.as_ptr(),
                command.as_mut_ptr(),
                NULL,
                NULL,
                1,
                CREATE_SUSPENDED | CREATE_UNICODE_ENVIRONMENT | EXTENDED_STARTUPINFO_PRESENT,
                block.as_mut_ptr().cast(),
                ptr::null(),
                &mut startup.startup,
                &mut pi,
            )
        },
        "CreateProcessW with Job list",
    );
    let process = Process {
        handle: Handle::new(pi.process, "process handle"),
        thread: Some(Handle::new(pi.thread, "thread handle")),
        exited: false,
    };
    assert!(
        is_in_job(process.handle.0, job),
        "child absent from creation-time Job"
    );
    process
}

pub struct Process {
    pub handle: Handle,
    thread: Option<Handle>,
    exited: bool,
}
impl Process {
    pub fn resume(&mut self) {
        let h = self.thread.take().expect("already resumed");
        if unsafe { ResumeThread(h.0) } == u32::MAX {
            error("ResumeThread")
        }
    }
    pub fn exited(&mut self, ms: u32) -> bool {
        if wait(self.handle.0, ms) {
            self.exited = true;
            true
        } else {
            false
        }
    }
    pub fn code(&self) -> u32 {
        assert!(self.exited);
        let mut c = 0;
        ok(
            unsafe { GetExitCodeProcess(self.handle.0, &mut c) },
            "GetExitCodeProcess",
        );
        c
    }
    pub fn terminate_and_wait(&mut self) {
        if !self.exited(0) {
            if unsafe { TerminateProcess(self.handle.0, 91) } == 0 && !self.exited(0) {
                eprintln!("fixture cleanup: TerminateProcess failed");
                std::process::abort();
            }
            if !self.exited(5000) {
                eprintln!("fixture cleanup: process did not exit");
                std::process::abort();
            }
        }
    }
}
impl Drop for Process {
    fn drop(&mut self) {
        if !self.exited {
            self.terminate_and_wait();
        }
    }
}
pub fn terminate_and_wait(handle: Raw) {
    if !wait(handle, 0) {
        if unsafe { TerminateProcess(handle, 92) } == 0 && !wait(handle, 0) {
            eprintln!("fixture cleanup: external TerminateProcess failed");
            std::process::abort();
        }
        if !wait(handle, 5000) {
            eprintln!("fixture cleanup: observed worker did not exit");
            std::process::abort();
        }
    }
}
