//! Opt-in native-smoke process mode. Invented protocol data only; no GUI or
//! domain apply. The external controller owns app/anchor Children and rescue.
use super::{
    live_runtime::{Live, Paths, StartError},
    loopback::Loopback,
    macos::CleanupOutcome,
    protocol::Event,
    relay_policy::{Provider, Secret},
    sandbox,
};
use crate::{host_memory::MacHostMemory, memory_pool::MemoryPool, solve_jobs::SolveJobRegistry};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{self, BufRead, Write},
    ops::{Deref, DerefMut},
    os::unix::{
        fs::{DirBuilderExt, PermissionsExt},
        process::ExitStatusExt,
    },
    path::PathBuf,
    process::{Command, Stdio},
    sync::{atomic::AtomicBool, mpsc, Arc},
    time::{Duration, Instant},
};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Config {
    state: PathBuf,
    model: String,
    context_window: u32,
    endpoint: String,
    key: Option<String>,
    mode: String,
    process_probe: PathBuf,
    boundary_probe: PathBuf,
}
fn record(start: Instant, kind: &str, data: Value) -> Result<(), &'static str> {
    let out = json!({"record":kind,"elapsedMs":start.elapsed().as_millis() as u64,"data":data});
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    serde_json::to_writer(&mut lock, &out).map_err(|_| "control_write")?;
    lock.write_all(b"\n").map_err(|_| "control_write")?;
    lock.flush().map_err(|_| "control_write")
}
fn frame(reader: &mut impl BufRead) -> Result<Option<Value>, &'static str> {
    let mut raw = Vec::new();
    loop {
        let buffer = reader.fill_buf().map_err(|_| "control_read")?;
        if buffer.is_empty() {
            return if raw.is_empty() {
                Ok(None)
            } else {
                Err("truncated_control")
            };
        }
        let length = buffer
            .iter()
            .position(|byte| *byte == b'\n')
            .map(|n| n + 1)
            .unwrap_or(buffer.len());
        if raw.len() + length > 16 * 1024 {
            return Err("control_limit");
        }
        let end = buffer[length - 1] == b'\n';
        raw.extend_from_slice(&buffer[..length]);
        reader.consume(length);
        if end {
            break;
        }
    }
    crate::ingress::prescan(&raw).map_err(|_| "control_grammar")?;
    super::numeric::check_admitted_json(&raw).map_err(|_| "control_number")?;
    serde_json::from_slice(&raw)
        .map(Some)
        .map_err(|_| "control_json")
}
fn resources() -> Result<PathBuf, &'static str> {
    let exe = std::env::current_exe().map_err(|_| "app_identity")?;
    let parent = exe.parent().ok_or("app_identity")?;
    if parent.file_name().and_then(|n| n.to_str()) != Some("MacOS") {
        return Err("not_bundle");
    }
    Ok(parent.parent().ok_or("app_identity")?.join("Resources"))
}
fn output(mut child: std::process::Child) -> Result<std::process::Output, &'static str> {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return child.wait_with_output().map_err(|_| "probe_output"),
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(2)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("probe_deadline");
            }
        }
    }
}
fn preflight(
    config: &Config,
    registry: &SolveJobRegistry,
    start: Instant,
) -> Result<(), &'static str> {
    let scope = Loopback::bind(registry).map_err(|_| "probe_listener")?;
    let forbidden = std::net::TcpListener::bind("127.0.0.1:0").map_err(|_| "probe_listener")?;
    let root = config.state.join("preflight");
    fs::DirBuilder::new()
        .mode(0o700)
        .create(&root)
        .map_err(|_| "probe_root")?;
    let home = root.join("home");
    let cwd = root.join("work");
    fs::DirBuilder::new()
        .mode(0o700)
        .create(&home)
        .map_err(|_| "probe_home")?;
    fs::DirBuilder::new()
        .mode(0o700)
        .create(&cwd)
        .map_err(|_| "probe_work")?;
    for (name, source) in [
        ("process", &config.process_probe),
        ("boundary", &config.boundary_probe),
    ] {
        let exe = root.join(name);
        fs::copy(source, &exe).map_err(|_| "probe_copy")?;
        fs::set_permissions(&exe, fs::Permissions::from_mode(0o500)).map_err(|_| "probe_mode")?;
        let profile =
            sandbox::profile(&root, &home, &cwd, &exe, &scope).map_err(|_| "probe_profile")?;
        let sb = root.join(format!("{name}.sb"));
        fs::write(&sb, &profile).map_err(|_| "probe_profile")?;
        let outside = config.state.join("outside");
        if name == "boundary" {
            fs::DirBuilder::new()
                .mode(0o700)
                .create(&outside)
                .map_err(|_| "probe_canary")?;
            for file in ["project", "documents", "keychain"] {
                fs::write(outside.join(file), b"invented outside canary")
                    .map_err(|_| "probe_canary")?;
            }
        }
        let mut command = Command::new("/usr/bin/sandbox-exec");
        command
            .arg("-f")
            .arg(&sb)
            .arg(&exe)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        if name == "process" {
            command.arg("/bin/echo");
        } else {
            command
                .current_dir(&cwd)
                .env("HOME", &home)
                .env("SWBPIPE_PROBE_OUTSIDE", &outside)
                .env("SWBPIPE_PROBE_ALLOWED_PORT", scope.port().to_string())
                .env(
                    "SWBPIPE_PROBE_FORBIDDEN_ENDPOINT",
                    forbidden
                        .local_addr()
                        .map_err(|_| "probe_listener")?
                        .to_string(),
                );
        }
        let result = output(command.spawn().map_err(|_| "probe_spawn")?)?;
        let first_record = result.stdout.contains(&b'\n');
        record(
            start,
            "probe_exit",
            json!({"probe":name,"code":result.status.code(),"signal":result.status.signal(),"firstRecord":first_record,"checks":probe_standing(name,result.status.success(),&result.stdout)}),
        )?;
        if name == "boundary" {
            record(
                start,
                "boundary_checks",
                boundary_check_values(&result.stdout),
            )?;
        }
        if !result.status.success() {
            return Err(if name == "process" {
                "process_denial_failed"
            } else {
                "boundary_denial_failed"
            });
        }
        if name == "process" && result.stdout != b"fork_errno=1 spawn_errno=1 exec_errno=1\n" {
            return Err("process_denial_records");
        }
        if name == "boundary" && !boundary_records(&result.stdout) {
            return Err("boundary_denial_records");
        }
        record(
            start,
            "preflight",
            json!({"probe":name,"passed":true,"profileSha256":format!("{:x}",Sha256::digest(profile.as_bytes()))}),
        )?;
    }
    fs::remove_dir_all(root).map_err(|_| "probe_cleanup")?;
    fs::remove_dir_all(config.state.join("outside")).map_err(|_| "probe_cleanup")?;
    Ok(())
}
// Never exit the sole original owner after unknown cleanup. A retained identity
// lets the controller join its original rescue anchor while this Child remains
// unreaped. It may then kill only its own app and rescue through that anchor.
struct Custody(Live);
impl Deref for Custody {
    type Target = Live;
    fn deref(&self) -> &Live {
        &self.0
    }
}
impl DerefMut for Custody {
    fn deref_mut(&mut self) -> &mut Live {
        &mut self.0
    }
}
impl Drop for Custody {
    fn drop(&mut self) {
        self.0.stop();
        if self.0.cleanup == Some(CleanupOutcome::Unknown) {
            if let Some((sdk, scope)) = self.0.fixture_identity() {
                let _ = record(
                    Instant::now(),
                    "custody_retained",
                    json!({"appPid":std::process::id(),"sdkPid":sdk,"scope":scope,"reason":"cleanup_unknown"}),
                );
                loop {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}
pub(crate) fn run() -> i32 {
    match run_inner() {
        Ok(()) => 0,
        Err(reason) => {
            let _ = record(Instant::now(), "failed", json!({"reason":reason}));
            1
        }
    }
}
fn run_inner() -> Result<(), &'static str> {
    let start = Instant::now();
    let registry = SolveJobRegistry::new(MemoryPool::new(Arc::new(MacHostMemory)));
    let _backing = registry
        .acquire_interactive_ingress(64 * 1024 * 1024)
        .map_err(|_| "fixture_admission")?;
    let mut input = io::BufReader::new(io::stdin());
    let first = frame(&mut input)?.ok_or("missing_config")?;
    let mut config: Config = serde_json::from_value(first).map_err(|_| "config_shape")?;
    if !["preflight", "idle", "active", "pending"].contains(&config.mode.as_str())
        || config.model.len() > 1024
        || !(config.endpoint.starts_with("http://127.0.0.1:")
            || config.endpoint.starts_with("http://[::1]:"))
    {
        return Err("config_scope");
    }
    let meta = fs::symlink_metadata(&config.state).map_err(|_| "state")?;
    if !meta.is_dir()
        || meta.permissions().mode() & 0o077 != 0
        || fs::canonicalize(&config.state).map_err(|_| "state")? != config.state
        || fs::read_dir(&config.state)
            .map_err(|_| "state")?
            .next()
            .is_some()
    {
        return Err("state_not_fresh_private");
    }
    preflight(&config, &registry, start)?;
    if config.mode == "preflight" {
        record(start, "preflight_complete", json!({"passed":true}))?;
        return Ok(());
    }
    let (sender, controls) = mpsc::sync_channel(16);
    let _reader_backing = registry
        .acquire_interactive_ingress(2 * 1024 * 1024)
        .map_err(|_| "control_admission")?;
    let reader = std::thread::Builder::new()
        .name("agent-fixture-control".into())
        .stack_size(512 * 1024)
        .spawn(move || {
            let _reader_backing = _reader_backing;
            loop {
                let value = frame(&mut input);
                let ended = !matches!(value, Ok(Some(_)));
                if sender.send(value).is_err() || ended {
                    break;
                }
            }
        })
        .map_err(|_| "control_thread")?;
    let basis = json!({"workspaceHandle":"invented-workspace","applicationGeneration":1,"modelRevision":1,"modelHash":{"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","payload_ref":"invented-project","value":format!("{:x}",Sha256::digest(b"invented native qualification model")),"hash_status":"computed_local_preview"},"stateId":null,"runId":null});
    let provider = Provider::new(&config.endpoint).map_err(|_| "provider")?;
    let secret = config
        .key
        .take()
        .map(Secret::new)
        .transpose()
        .map_err(|_| "credential")?;
    let cancel = Arc::new(AtomicBool::new(false));
    let mut live = match Live::start(
        &registry,
        Paths {
            resources: resources()?,
            state: config.state.clone(),
        },
        config.model,
        config.context_window,
        provider,
        secret,
        "native-fixture-private-session-token",
        &basis,
        &cancel,
    ) {
        Ok(live) => live,
        Err(StartError::Refused(reason)) => {
            record(
                start,
                "startup_failure",
                json!({"class":reason.class(),"custody":"none"}),
            )?;
            return Err("startup_refused");
        }
        Err(StartError::Partial(reason, live)) => {
            let mut live = Custody(live);
            record(
                start,
                "startup_failure",
                json!({"class":reason.class(),"custody":"retained"}),
            )?;
            live.stop();
            let until = Instant::now() + Duration::from_secs(5);
            while !live.joined() && Instant::now() < until {
                std::thread::sleep(Duration::from_millis(2));
            }
            record(
                start,
                "startup_cleanup",
                json!({"joined":live.joined(),"confirmed":live.cleanup==Some(CleanupOutcome::GroupStopped)}),
            )?;
            return Err("startup_partial");
        }
    };
    let mut live = Custody(live);
    let (pid, scope) = live.fixture_identity().ok_or("supplier_identity")?;
    record(
        start,
        "ready",
        json!({"appPid":std::process::id(),"sdkPid":pid,"scope":scope,"mode":config.mode,"state":"held_startup"}),
    )?;
    let deadline = Instant::now() + Duration::from_secs(180);
    let mut go = false;
    let mut turn_started = false;
    let mut turn_ended = false;
    let mut pending = false;
    let mut eof = false;
    let mut finished = false;
    let mut control_eof = false;
    let mut reported_exit = false;
    while Instant::now() < deadline {
        if go {
            // Drain bounded observed frames before considering an external trigger.
            for _ in 0..32 {
                let batch = match live.poll() {
                    Ok(batch) => batch,
                    Err(reason) => {
                        record(
                            start,
                            "session_failure",
                            json!({"class":reason.class(),"rejection":live.driver().and_then(|d|d.last_rejection())}),
                        )?;
                        return Err("session_poll");
                    }
                };
                let Some(batch) = batch else {
                    break;
                };
                for event in batch.events() {
                    match event {
                        Event::Initialized => {
                            record(start, "initialized", json!({}))?;
                        }
                        Event::ThreadStarted { .. } => {
                            record(start, "thread_ready", json!({}))?;
                        }
                        Event::TurnStarted { .. } => {
                            turn_started = true;
                            record(start, "turn_started", json!({}))?;
                        }
                        Event::TurnCompleted { status, .. } => {
                            turn_ended = true;
                            record(start, "turn_ended", json!({"status":status}))?;
                        }
                        Event::ToolRequested {
                            request_ref, tool, ..
                        } => {
                            if *tool != super::catalog::Tool::Summary {
                                return Err("unexpected_tool");
                            }
                            if config.mode == "pending" {
                                pending = true;
                                record(start, "tool_pending", json!({"tool":tool.name()}))?;
                            } else {
                                let value = json!({"status":"ok","basis":basis,"message":"Invented qualification model; no operation applied.","data":{"workspaceHandle":"invented-workspace","projectId":"project:invented","schemaVersion":"0.8.0","pressureContract":null,"pressureProfileBefore":"{\"schema_version\":\"0.8.0\"}","projectUnits":{"length":"m"},"objectCounts":{"nodes":1},"admittedOperationKinds":["modify/Node/set_field/position"],"limits":{"argumentBytes":16384,"resultBytes":65536,"pageRows":50,"pendingCalls":16,"proposalOperations":16}}});
                                let backing = Arc::new(
                                    registry
                                        .acquire_interactive_ingress(2 * 1024 * 1024)
                                        .map_err(|_| "tool_admission")?,
                                );
                                live.driver()
                                    .ok_or("driver")?
                                    .complete(request_ref.as_str(), value, backing)
                                    .map_err(|_| "tool_complete")?;
                                record(start, "tool_completed", json!({"tool":tool.name()}))?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if let Some(reason) = live.terminal().and_then(super::live_runtime::End::failure) {
            record(
                start,
                "session_failure",
                json!({"class":reason,"rejection":live.driver().and_then(|driver|driver.last_rejection())}),
            )?;
            return Err("session_terminal");
        }
        match controls.try_recv() {
            Ok(Ok(Some(command))) => match command.get("cmd").and_then(Value::as_str) {
                Some("GO") if !go => {
                    go = true;
                }
                Some("TEXT") if live.ready() && !turn_started => {
                    live.driver().ok_or("driver")?.submit_text("Call swbpipe_read_model_summary exactly once for this invented model. Then give a factual answer and explain in at least 300 words why a person must review proposed changes. Do not call another tool.").map_err(|_|"turn_submit")?;
                }
                Some(action @ ("EOF" | "ARM_CRASH")) if !eof => {
                    let scope = live.fixture_scope().ok_or("observer")?;
                    if scope.exit_observed
                        || scope.invalid.is_some()
                        || (config.mode != "idle" && (!turn_started || turn_ended))
                        || (config.mode == "pending" && !pending)
                    {
                        return Err("ambiguous_or_terminal_trigger");
                    }
                    eof = true;
                    record(
                        start,
                        if action == "EOF" {
                            "eof_requested"
                        } else {
                            "crash_armed"
                        },
                        json!({"turnStarted":turn_started,"turnEnded":turn_ended,"toolPending":pending}),
                    )?;
                    if action == "EOF" {
                        live.request_eof();
                    }
                }
                Some("FINISH") => {
                    finished = true;
                    live.stop();
                }
                _ => return Err("control_order"),
            },
            Ok(Ok(None)) | Err(mpsc::TryRecvError::Disconnected) => {
                control_eof = true;
                finished = true;
                live.stop();
            }
            Ok(Err(_)) => return Err("control_input"),
            Err(mpsc::TryRecvError::Empty) => {}
        }
        if let Some(observed) = live.fixture_scope() {
            if observed.exit_observed && !reported_exit {
                reported_exit = true;
                record(
                    start,
                    "supplier_exit",
                    json!({"statusRaw":observed.exit_status_raw,"invalid":observed.invalid.is_some()}),
                )?;
            }
        }
        if finished && control_eof && reader.is_finished() && live.joined() {
            reader.join().map_err(|_| "control_reader_panic")?;
            record(
                start,
                "finished",
                json!({"cleanup":if live.cleanup==Some(CleanupOutcome::GroupStopped){"confirmed"}else{"unknown"}}),
            )?;
            return if live.cleanup == Some(CleanupOutcome::GroupStopped) {
                Ok(())
            } else {
                Err("cleanup_unknown")
            };
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    live.stop();
    Err("fixture_deadline")
}

fn probe_standing(name: &str, success: bool, raw: &[u8]) -> &'static str {
    if !raw.contains(&b'\n') {
        return "not_run";
    }
    let valid = match name {
        "process" => raw == b"fork_errno=1 spawn_errno=1 exec_errno=1\n",
        "boundary" => boundary_records(raw),
        _ => false,
    };
    if success && valid {
        "passed"
    } else {
        "unknown"
    }
}
const BOUNDARY_NAMES: &[&str] = &[
    "private_home",
    "private_cwd",
    "project",
    "documents",
    "keychain",
    "outside_write",
    "symlink_read",
    "provider_endpoint",
    "provider_ipv6_endpoint",
    "unowned_loopback_alias",
    "other_loopback",
    "off_host",
    "syslog",
    "users_read",
    "listener",
    "other_exec",
    "same_exec",
];
fn boundary_check_values(raw: &[u8]) -> Value {
    let text = std::str::from_utf8(raw).unwrap_or("");
    let mut fields = serde_json::Map::new();
    for name in BOUNDARY_NAMES {
        let matches: Vec<_> = text
            .lines()
            .filter_map(|line| line.strip_prefix(&format!("{name}: ")))
            .collect();
        let value = match matches.as_slice() {
            ["true"] => json!(true),
            ["false"] => json!(false),
            [] => json!("not_run"),
            _ => json!("unknown"),
        };
        fields.insert((*name).into(), value);
    }
    Value::Object(fields)
}
fn boundary_records(raw: &[u8]) -> bool {
    let Ok(text) = std::str::from_utf8(raw) else {
        return false;
    };
    let expected = BOUNDARY_NAMES;
    let lines: Vec<_> = text.lines().collect();
    lines.len() == expected.len()
        && expected
            .iter()
            .zip(lines)
            .all(|(name, line)| line == format!("{name}: true"))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absent_truncated_or_unexpected_boundary_records_never_pass() {
        assert_eq!(probe_standing("boundary", true, b""), "not_run");
        assert_eq!(
            probe_standing("boundary", true, b"private_home: true\n"),
            "unknown"
        );
        assert_eq!(
            probe_standing(
                "process",
                true,
                b"fork_errno=1 spawn_errno=1 exec_errno=1\n"
            ),
            "passed"
        );
        assert_eq!(boundary_check_values(b"")["outside_write"], "not_run");
        assert_eq!(
            boundary_check_values(b"outside_write: false\n")["outside_write"],
            false
        );
        assert_eq!(
            boundary_check_values(b"outside_write: false\noutside_write: true\n")["outside_write"],
            "unknown"
        );
        assert!(!boundary_records(b""));
        assert!(!boundary_records(b"private_home: true\n"));
        assert!(!boundary_records(b"private_home: false\n"));
    }
}
