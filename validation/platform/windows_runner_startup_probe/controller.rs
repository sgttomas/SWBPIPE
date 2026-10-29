//! Finite diagnostic only. No product runner edits or external crate dependencies.
#[cfg(not(all(windows, target_arch="x86_64")))]
compile_error!("hosted native x64 Windows only");
mod pe;
mod win32;
use std::{io::Read, path::{Path}, time::{Duration,Instant}};
const CASES: &[&str] = &["validate_metadata","malformed_json","missing_payload","blocked_timeout"];
const BLOCKED: &[&str] = &["solve_first=BLOCKED_UNVERIFIED_CAPTURE_CAP", "solve_repeat=BLOCKED_UNVERIFIED_CAPTURE_CAP", "ten_live_solve_mutations=UNVERIFIED"];
struct Job(win32::Handle);
impl Drop for Job { fn drop(&mut self) { win32::drain_job(self.0.0); } }
fn string(s: &std::ffi::OsStr) -> String { s.to_str().expect("UTF8 argument").to_string() }
fn main() {
    win32::native_self();
    let a: Vec<_>=std::env::args_os().skip(1).collect();
    assert!(!a.is_empty(), "empty selection");
    match string(&a[0]).as_str() {
        "--inventory" => { assert_eq!(a.len(),1); for case in CASES { println!("{case}"); } }
        "--blocked-inventory" => { assert_eq!(a.len(),1); for item in BLOCKED { println!("{item}"); } }
        "--pe" => {
            assert_eq!(a.len(),2);
            let f=std::fs::File::open(&a[1]).expect("candidate open"); let mut b=Vec::new();
            f.take(256*1024*1024+1).read_to_end(&mut b).expect("candidate read");
            let facts=pe::parse(&b).expect("actual PE rejected");
            println!("pe=PE32+ machine=AMD64 stack_reserve={} stack_commit={} image_size={} headers_size={} file_size={}",facts.stack_reserve,facts.stack_commit,facts.image_size,facts.headers_size,facts.file_size);
            for import in facts.imports { println!("{import}"); }
        }
        "--supervise" => {
            assert_eq!(a.len(),6,"supervise requires case, runner, input, output, verb");
            let case=string(&a[1]); assert!(CASES.contains(&case.as_str()),"unknown selection");
            let job=Job(win32::job()); let ready=win32::event();
            let mut args=vec!["--case".into(),case.clone()];
            args.extend(a[2..].iter().map(|s|string(s)));
            args.push(ready.value().to_string());
            let mut process=win32::spawn(job.0.0,&[ready.0],&std::env::current_exe().expect("controller path"),&args,None);
            process.resume();
            let mut timeout=false;
            if case=="blocked_timeout" {
                assert!(win32::wait(ready.0,10000),"blocked descendant gate absent");
                assert_eq!(win32::job_active(job.0.0),2,"negative control needs case plus actual runner");
                assert!(!process.exited(200),"blocked case unexpectedly exited");
                timeout=true;
            } else if !process.exited(25000) { timeout=true; }
            if timeout { win32::drain_job(job.0.0); }
            assert!(process.exited(5000),"case HANDLE not signaled");
            let code=process.code(); // 259 is never consulted for liveness.
            win32::drain_job(job.0.0);
            assert_eq!(win32::job_active(job.0.0),0);
            println!("phase=JOB_DRAINED case={case} active=0 case_exit_observed=true");
            if case=="blocked_timeout" {
                assert_eq!(code,93,"negative control was not killed by outer Job");
                println!("BLOCKED_DESCENDANT_TIMEOUT_CONFIRMED");
            } else { assert!(!timeout,"outer case timeout"); assert_eq!(code,0,"case failed"); }
        }
        "--case" => {
            assert_eq!(a.len(),7);
            let case=string(&a[1]); assert!(CASES.contains(&case.as_str()));
            let verb=string(&a[5]);
            let expected = if matches!(case.as_str(),"validate_metadata"|"blocked_timeout") { "validate-input" } else { "solve" };
            assert_eq!(verb,expected,"case verb mismatch");
            let ready=string(&a[6]).parse::<usize>().expect("ready handle") as win32::Raw;
            run_case(&case,Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]),&verb,ready);
        }
        _=>panic!("unknown selection"),
    }
}
fn reader(mut file:std::fs::File, budget:std::sync::Arc<std::sync::atomic::AtomicUsize>) -> std::sync::mpsc::Receiver<Result<Vec<u8>, &'static str>> {
    let (tx,rx)=std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let mut out=Vec::new(); let mut buf=[0;8192];
        let result=(|| { loop {
            let n=file.read(&mut buf).map_err(|_|"output pipe read failed")?;
            if n==0 { break; }
            let before=budget.fetch_add(n,std::sync::atomic::Ordering::SeqCst);
            if before.checked_add(n).ok_or("output overflow")?>1024*1024 { return Err("combined stdout/stderr exceeds 1 MiB"); }
            out.extend_from_slice(&buf[..n]);
        } Ok(out) })();
        let _=tx.send(result);
    }); rx
}
fn run_case(case:&str,exe:&Path,input:&Path,output:&Path,verb:&str,ready:win32::Raw) {
    let start=Instant::now();
    let phase=|name:&str| println!("phase={name} case={case} observation_offset_us={}",start.elapsed().as_micros());
    let mut payload=Vec::new();
    std::fs::File::open(input).expect("fixture open").take(65537).read_to_end(&mut payload).expect("fixture read");
    assert!(!payload.is_empty() && payload.len()<=65536,"input limit");
    // Closed, compiled byte inputs: no case can substitute a real solve or user data.
    let expected: &[u8] = match case {
        "validate_metadata"|"blocked_timeout" => include_bytes!("fixtures/validate_metadata.json"),
        "missing_payload" => include_bytes!("fixtures/missing_payload.json"),
        "malformed_json" => b"{",
        _ => panic!("case is outside PARTIAL diagnostic inventory"),
    };
    assert!(payload.as_slice()==expected,"diagnostic input differs from compiled invented fixture");
    let pipe=win32::InputPipe::new(); let job=Job(win32::job());
    let (stdout_read,stdout_write)=win32::output_pipe(); let (stderr_read,stderr_write)=win32::output_pipe();
    // stdin=NUL (owned inside spawn); stdout/stderr=only these explicit writers.
    // No pipe server, Job, process, thread or output reader handle inherits.
    let nul=std::fs::OpenOptions::new().read(true).write(true).open("NUL").expect("NUL input");
    use std::os::windows::io::AsRawHandle;
    let mut args=vec![verb.to_string(),"--input".into(),pipe.name.clone()];
    if case!="blocked_timeout" {
        // Explicit test intent for only the compiled invented diagnostic records.
        // Source flag semantics, privacy warnings and audit records stay unchanged.
        args.push("--explicit-local-private-intent".into());
        println!("test_intent=invented_diagnostic_only case={case}");
    }
    let mut runner=win32::spawn(job.0.0,&[],exe,&args,Some([nul.as_raw_handle(),stdout_write.0,stderr_write.0]));
    drop(stdout_write); drop(stderr_write);
    phase("CREATED_SUSPENDED"); // pre-loader, not settled startup.
    win32::native_x64(runner.handle.0); win32::bind_image(runner.handle.0,exe);
    let budget=std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let out=reader(stdout_read,budget.clone()); let err=reader(stderr_read,budget);
    runner.resume(); phase("RESUMED");
    pipe.connected(runner.pid);
    assert!(!runner.exited(0),"runner exited before payload gate");
    phase("INPUT_CONNECTED_PAYLOAD_WITHHELD");
    win32::signal(ready);
    if case=="blocked_timeout" {
        // Actual unmodified runner remains without bytes/EOF. The external
        // supervisor deliberately kills case+runner after observing this gate.
        loop { std::thread::park(); }
    }
    win32::observe(&runner,"INPUT_CONNECTED_PAYLOAD_WITHHELD");
    pipe.send(&payload); drop(pipe); phase("PAYLOAD_SENT_EOF");
    let deadline=Instant::now()+Duration::from_secs(8);
    let recv=|rx:std::sync::mpsc::Receiver<Result<Vec<u8>,&'static str>>| {
        rx.recv_timeout(deadline.saturating_duration_since(Instant::now())).expect("output EOF timeout").expect("output capture failed")
    };
    let stdout=recv(out); let stderr=recv(err);
    phase("OUTPUT_COMPLETE");
    assert!(runner.exited(3000),"runner process HANDLE not signaled"); phase("EXIT_OBSERVED");
    let expected=match case {"malformed_json"=>2,"missing_payload"=>1,_=>0};
    assert_eq!(runner.code(),expected,"unexpected runner exit code");
    assert!(stderr.is_empty(),"unexpected runner stderr (withheld from log)");
    assert!(!stdout.is_empty(),"empty runner output");
    println!("captured_output_bytes={} case={case}",stdout.len());
    std::fs::write(output,stdout).expect("bounded output write");
    println!("runner_exit_code={} case={case}",runner.code());
    win32::drain_job(job.0.0); assert_eq!(win32::job_active(job.0.0),0);
}
