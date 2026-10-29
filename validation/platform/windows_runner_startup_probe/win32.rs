//! Standalone x64 diagnostic bindings, adapted read-only from windows_arena_probe.
#![allow(dead_code)]
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
    fn CreateEventW(sa: *const Security, manual: i32, initial: i32, name: *const u16) -> Raw;
    fn SetEvent(event: Raw) -> i32;
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

pub fn event() -> Handle {
    Handle::new(
        unsafe { CreateEventW(&sa(), 0, 0, ptr::null()) },
        "CreateEventW",
    )
}
pub fn signal(h: Raw) {
    ok(unsafe { SetEvent(h) }, "SetEvent")
}
pub fn wait(h: Raw, ms: u32) -> bool {
    match unsafe { WaitForSingleObject(h, ms) } {
        WAIT_OBJECT_0 => true,
        WAIT_TIMEOUT => false,
        _ => error("WaitForSingleObject"),
    }
}
pub fn job() -> Handle {
    let h = Handle::new(
        unsafe { CreateJobObjectW(ptr::null(), ptr::null()) },
        "CreateJobObjectW",
    );
    let mut limits = ExtendedLimit::default();
    limits.basic.flags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE | 8 | 0x200;
    limits.basic.active_limit = 2;
    limits.job_memory = 512 * 1024 * 1024; // diagnostic cap only; a hit is failure.
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
        eprintln!("diagnostic cleanup: TerminateJobObject failed");
        std::process::abort();
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while job_active(job) != 0 {
        if std::time::Instant::now() >= deadline {
            eprintln!("diagnostic cleanup: Job still has active processes");
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

/// Creation-time Job assignment; only the named standard handles plus extras inherit.
pub fn spawn(job: Raw, handles: &[Raw], exe: &std::path::Path, args: &[String], streams: Option<[Raw; 3]>) -> Process {
    assert_eq!(size_of::<Startup>(), 104);
    assert_eq!(size_of::<StartupEx>(), 112);
    assert_eq!(size_of::<BasicLimit>(), 64);
    assert_eq!(size_of::<ExtendedLimit>(), 144);
    let nul_name = wide(OsStr::new("NUL"));
    let nul = Handle::new(unsafe { CreateFileW(nul_name.as_ptr(), 0xc0000000, 3, &sa(), 3, 0x80, NULL) }, "NUL");
    let supplied = streams.unwrap_or([nul.0, unsafe { GetStdHandle((-11i32) as u32) }, unsafe { GetStdHandle((-12i32) as u32) }]);
    let mut owners = Vec::new();
    for h in supplied {
        let mut raw = NULL;
        ok(unsafe { DuplicateHandle(GetCurrentProcess(), h, GetCurrentProcess(), &mut raw, 0, 1, DUPLICATE_SAME_ACCESS) }, "standard handle duplicate");
        owners.push(Handle::new(raw, "standard handle"));
    }
    let mut inherited: Vec<Raw> = owners.iter().map(|h| h.0).collect();
    inherited.extend_from_slice(handles);
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
    // Quote by the documented CommandLineToArgvW/MS C runtime backslash rules.
    fn quote(s: &str) -> String {
        let mut out = String::from("\""); let mut slashes = 0;
        for c in s.chars() {
            if c == '\\' { slashes += 1; continue; }
            out.extend(std::iter::repeat_n('\\', if c == '\"' { slashes * 2 + 1 } else { slashes }));
            slashes = 0; out.push(c);
        }
        out.extend(std::iter::repeat_n('\\', slashes * 2)); out.push('\"'); out
    }
    let mut words = vec![quote(exe.to_str().expect("UTF-8 candidate path"))];
    words.extend(args.iter().map(|s| quote(s)));
    let mut command = wide(OsStr::new(&words.join(" ")));
    let app = wide(exe.as_os_str());
    let mut startup: StartupEx = unsafe { zeroed() };
    startup.startup.cb = size_of::<StartupEx>() as u32;
    startup.startup.flags = 0x100; // STARTF_USESTDHANDLES
    startup.startup.stdin = owners[0].0;
    startup.startup.stdout = owners[1].0;
    startup.startup.stderr = owners[2].0;
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
                NULL,
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
        pid: pi.pid,
    };
    assert!(
        is_in_job(process.handle.0, job),
        "child absent from creation-time Job"
    );
    process
}

pub struct Process {
    pub handle: Handle,
    pub pid: u32,
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
                eprintln!("diagnostic cleanup: TerminateProcess failed");
                std::process::abort();
            }
            if !self.exited(5000) {
                eprintln!("diagnostic cleanup: process did not exit");
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
#[repr(C)]
struct Overlapped { internal: usize, internal_high: usize, offset: u64, event: Raw }
impl Default for Overlapped { fn default() -> Self { Self { internal: 0, internal_high: 0, offset: 0, event: NULL } } }
#[link(name="kernel32")]
unsafe extern "system" {
    fn CreateNamedPipeW(name: *const u16, access: u32, mode: u32, instances: u32, output: u32, input: u32, timeout: u32, security: *const Security) -> Raw;
    fn ConnectNamedPipe(pipe: Raw, ov: *mut Overlapped) -> i32;
    fn GetNamedPipeClientProcessId(pipe: Raw, pid: *mut u32) -> i32;
    fn GetOverlappedResult(file: Raw, ov: *mut Overlapped, count: *mut u32, wait: i32) -> i32;
    fn CancelIoEx(file: Raw, ov: *mut Overlapped) -> i32;
    fn WriteFile(file: Raw, bytes: *const u8, length: u32, written: *mut u32, ov: *mut Overlapped) -> i32;
    fn CreatePipe(read: *mut Raw, write: *mut Raw, security: *const Security, size: u32) -> i32;
    fn QueryFullProcessImageNameW(process: Raw, flags: u32, name: *mut u16, size: *mut u32) -> i32;
    fn VirtualQueryEx(process: Raw, address: *const c_void, info: *mut MemoryBasic, size: usize) -> usize;
    fn K32EnumProcessModules(process: Raw, modules: *mut Raw, bytes: u32, needed: *mut u32) -> i32;
    fn K32GetModuleBaseNameW(process: Raw, module: Raw, name: *mut u16, size: u32) -> u32;
    fn K32GetProcessMemoryInfo(process: Raw, counters: *mut MemoryCounters, size: u32) -> i32;
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> Raw;
    fn Thread32First(snapshot: Raw, entry: *mut ThreadEntry) -> i32;
    fn Thread32Next(snapshot: Raw, entry: *mut ThreadEntry) -> i32;
    fn LocalFree(memory: Raw) -> Raw;
    fn IsWow64Process2(process: Raw, process_machine: *mut u16, native_machine: *mut u16) -> i32;
}
#[link(name="advapi32")]
unsafe extern "system" {
    fn ConvertStringSecurityDescriptorToSecurityDescriptorW(text: *const u16, revision: u32, descriptor: *mut Raw, size: *mut u32) -> i32;
}
pub fn native_x64(process: Raw) {
    let (mut p, mut n) = (0,0);
    ok(unsafe { IsWow64Process2(process, &mut p, &mut n) }, "IsWow64Process2");
    assert_eq!((p,n), (0,0x8664), "native x64 required");
}
pub fn native_self() { native_x64(unsafe { GetCurrentProcess() }); }

pub struct InputPipe { pub handle: Handle, pub name: String }
impl InputPipe {
    pub fn new() -> Self {
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let name = format!(r"\\.\pipe\swbpipe-startup-{}-{nonce}", std::process::id());
        let mut descriptor = NULL;
        let sddl = wide(OsStr::new("D:P(A;;GA;;;SY)(A;;GA;;;OW)"));
        ok(unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorW(sddl.as_ptr(), 1, &mut descriptor, ptr::null_mut()) }, "pipe DACL");
        let security = Security { length: size_of::<Security>() as u32, descriptor, inherit: 0 };
        let raw = unsafe { CreateNamedPipeW(wide(OsStr::new(&name)).as_ptr(), 2 | 0x40000000 | 0x80000, 8, 1, 65536, 0, 0, &security) };
        let saved = unsafe { GetLastError() };
        unsafe { LocalFree(descriptor); }
        assert!(raw != INVALID && !raw.is_null(), "CreateNamedPipeW error {saved}");
        Self { handle: Handle(raw), name }
    }
    fn complete(&self, ov: &mut Overlapped, started: i32, bytes: &mut u32) {
        if started == 0 {
            let error = unsafe { GetLastError() };
            assert_eq!(error, 997, "overlapped operation did not pend");
            if !wait(ov.event, 5000) {
                unsafe { CancelIoEx(self.handle.0, ov); }
                // The independent outer observer can kill this case even on cancellation failure.
                if !wait(ov.event, 1000) { std::process::abort(); }
                panic!("pipe phase timeout");
            }
        }
        ok(unsafe { GetOverlappedResult(self.handle.0, ov, bytes, 0) }, "overlapped completion");
    }
    pub fn connected(&self, expected_pid: u32) {
        let event = event(); let mut ov = Overlapped { event: event.0, ..Default::default() };
        let start = unsafe { ConnectNamedPipe(self.handle.0, &mut ov) };
        let already = start == 0 && unsafe { GetLastError() } == 535;
        if !already { self.complete(&mut ov, start, &mut 0); }
        let mut pid = 0;
        ok(unsafe { GetNamedPipeClientProcessId(self.handle.0, &mut pid) }, "pipe client PID");
        assert_eq!(pid, expected_pid, "wrong pipe client");
    }
    pub fn send(&self, bytes: &[u8]) {
        assert!(bytes.len() <= 65536);
        let event = event(); let mut ov = Overlapped { event: event.0, ..Default::default() }; let mut sent = 0;
        let start = unsafe { WriteFile(self.handle.0, bytes.as_ptr(), bytes.len() as u32, ptr::null_mut(), &mut ov) };
        self.complete(&mut ov, start, &mut sent);
        assert_eq!(sent as usize, bytes.len(), "short pipe write");
        // Do not FlushFileBuffers: synchronous flush could block. Closing after the
        // completed write leaves buffered bytes for the client, then broken-pipe EOF.
    }
}
pub fn output_pipe() -> (std::fs::File, Handle) {
    use std::os::windows::io::FromRawHandle;
    let (mut r, mut w) = (NULL,NULL);
    ok(unsafe { CreatePipe(&mut r, &mut w, ptr::null(), 65536) }, "CreatePipe");
    (unsafe { std::fs::File::from_raw_handle(r) }, Handle::new(w, "output writer"))
}
pub fn bind_image(process: Raw, candidate: &std::path::Path) {
    let mut name = vec![0u16; 32768]; let mut n = name.len() as u32;
    ok(unsafe { QueryFullProcessImageNameW(process, 0, name.as_mut_ptr(), &mut n) }, "process image path");
    let path = std::path::PathBuf::from(String::from_utf16(&name[..n as usize]).expect("image UTF16"));
    assert!(std::fs::canonicalize(path).expect("launched image canonicalization").to_string_lossy().eq_ignore_ascii_case(&std::fs::canonicalize(candidate).expect("candidate canonicalization").to_string_lossy()), "launched image mismatch");
    println!("artifact_binding=actual_process_image_match");
}
#[repr(C)]
#[derive(Default)]
struct MemoryCounters { cb: u32, faults: u32, peak_ws: usize, ws: usize, peak_paged: usize, paged: usize, peak_nonpaged: usize, nonpaged: usize, pagefile: usize, peak_pagefile: usize, private_usage: usize }
#[repr(C)]
struct MemoryBasic { base: Raw, allocation: Raw, protection: u32, partition: u16, size: usize, state: u32, protect: u32, kind: u32 }
impl Default for MemoryBasic { fn default() -> Self { Self { base: NULL, allocation: NULL, protection: 0, partition: 0, size: 0, state: 0, protect: 0, kind: 0 } } }
#[repr(C)]
#[derive(Default)]
struct ThreadEntry { size: u32, usage: u32, tid: u32, pid: u32, base: i32, delta: i32, flags: u32 }
fn basename(s: &str) -> &str { s.rsplit(['\\','/']).next().unwrap_or("") }
fn safe_name(s: &str) -> bool { !s.is_empty() && s.len() <= 260 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)) }
pub fn observe(process: &Process, phase: &str) {
    assert!(!wait(process.handle.0, 0), "observation requires live process");
    let mut c = MemoryCounters::default(); c.cb = size_of::<MemoryCounters>() as u32;
    ok(unsafe { K32GetProcessMemoryInfo(process.handle.0, &mut c, size_of::<MemoryCounters>() as u32) }, "GetProcessMemoryInfo");
    println!("sample={phase} private_usage={} ws={} peak_ws={}", c.private_usage,c.ws,c.peak_ws);
    let mut sums = [[0u64; 2];3]; let mut address = 0usize; let mut ended = false;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    for _ in 0..1_000_000 {
        assert!(std::time::Instant::now() < deadline, "VA enumeration deadline");
        let mut info = MemoryBasic::default();
        let count = unsafe { VirtualQueryEx(process.handle.0, address as *const c_void, &mut info, size_of::<MemoryBasic>()) };
        if count == 0 {
            assert_eq!(unsafe { GetLastError() }, 87, "VirtualQueryEx failure"); ended = true; break;
        }
        assert_eq!(count, size_of::<MemoryBasic>());
        assert!(info.size > 0 && info.base as usize >= address, "invalid VA region");
        if let Some(kind) = match info.kind { 0x20000=>Some(0),0x1000000=>Some(1),0x40000=>Some(2),_=>None } {
            if let Some(state) = match info.state {0x1000=>Some(0),0x2000=>Some(1),_=>None} {
                sums[kind][state] = sums[kind][state].checked_add(info.size as u64).expect("VA sum overflow");
            }
        }
        address = (info.base as usize).checked_add(info.size).expect("VA address overflow");
    }
    assert!(ended, "VA enumeration cap");
    for (name, sum) in ["private","image","mapped"].iter().zip(sums) { println!("sample={phase} va_type={name} commit={} reserve={}",sum[0],sum[1]); }
    let snapshot = Handle::new(unsafe { CreateToolhelp32Snapshot(4, 0) }, "thread snapshot");
    let mut t = ThreadEntry { size: size_of::<ThreadEntry>() as u32, ..Default::default() }; let mut threads = 0; let mut seen = 0;
    let mut result = unsafe { Thread32First(snapshot.0, &mut t) };
    while result != 0 { seen += 1; assert!(seen < 100000, "thread enumeration cap"); if t.pid == process.pid { threads += 1; } t.size=size_of::<ThreadEntry>() as u32; result=unsafe { Thread32Next(snapshot.0, &mut t) }; }
    assert_eq!(unsafe { GetLastError() }, 18, "thread enumeration failed"); assert!(threads > 0);
    println!("sample={phase} threads={threads}");
    let mut success = false;
    for _ in 0..3 {
        let mut modules = [NULL; 2048]; let mut needed = 0;
        if unsafe { K32EnumProcessModules(process.handle.0, modules.as_mut_ptr(), size_of::<[Raw;2048]>() as u32, &mut needed) } == 0 { continue; }
        if needed == 0 || needed as usize > size_of::<[Raw;2048]>() || needed as usize % size_of::<Raw>() != 0 { continue; }
        let mut names = Vec::new(); let mut complete = true;
        for module in &modules[..needed as usize / size_of::<Raw>()] {
            let mut name = [0u16; 261]; let n = unsafe { K32GetModuleBaseNameW(process.handle.0,*module,name.as_mut_ptr(),name.len() as u32) } as usize;
            if n == 0 || n >= name.len() { complete=false; break; }
            let s=String::from_utf16(&name[..n]).expect("module UTF16");
            if !safe_name(basename(&s)) { complete=false; break; } names.push(basename(&s).to_string());
        }
        if complete { names.sort(); names.dedup(); for name in names { println!("sample={phase} module={name}"); } success=true; break; }
    }
    assert!(success, "module inventory failed after three bounded attempts");
}
