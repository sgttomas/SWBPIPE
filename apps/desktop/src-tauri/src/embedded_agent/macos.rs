//! Explicit-start macOS supplier custody. No initializer, worker reuse or threads.
//! Parent crash termination and absence of escaped descendants are NOT guaranteed.
use super::{loopback::Loopback, sandbox};
use crate::memory_pool::{Lease, LeasePurpose};
use sha2::{Digest, Sha256};
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{symlink, DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub(crate) const STARTUP_BYTES: u64 = 4 * 1024 * 1024;
const PIN_BYTES: u64 = 241_555_024;
const PIN_SHA: &str = "112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b";
const CAPABILITY_ENV: &str = "SWBPIPE_AGENT_RELAY_CAPABILITY";
const CONFIG_BODY: &str = r#"model_provider = "swbpipe_relay"
model_context_window = 8192
model_reasoning_effort = "low"
sandbox_mode = "read-only"
approval_policy = "never"
cli_auth_credentials_store = "file"
web_search = "disabled"
[model_providers.swbpipe_relay]
name = "SWBPIPE provider"
wire_api = "responses"
requires_openai_auth = false
env_key = "SWBPIPE_AGENT_RELAY_CAPABILITY"
[analytics]
enabled = false
[features]
shell_tool = false
shell_snapshot = false
unified_exec = false
code_mode = false
code_mode_host = false
daemon_auto_start = false
apps = false
plugins = false
multi_agent = false
memories = false
browser_use = false
browser_use_external = false
computer_use = false
image_generation = false
view_image = false
skill_search = false
"#;

/// JSON string escaping is a subset of TOML basic-string escaping for the
/// accepted, control-free model identifier. No raw model text is interpolated.
fn supplier_config(model: &str, relay: &Loopback) -> Result<String, Error> {
    if model.is_empty() || model.len() > 1024 || model.chars().any(char::is_control) {
        return Err(Error::InvalidConfiguration);
    }
    let model = serde_json::to_string(model).map_err(|_| Error::InvalidConfiguration)?;
    let base_url = format!("base_url = \"http://127.0.0.1:{}/v1\"\n", relay.port());
    Ok(format!(
        "model = {model}\n{}",
        CONFIG_BODY.replace(
            "name = \"SWBPIPE provider\"\n",
            &format!("name = \"SWBPIPE provider\"\n{base_url}")
        )
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Error {
    InvalidConfiguration,
    StartupReservation,
    InvalidPath,
    StaleScope,
    Prepare,
    PinMismatch,
    Spawn,
}
// The provider credential never enters this module. The child receives only
// the per-start loopback capability held by the private relay scope.
pub(crate) struct LaunchConfig {
    pub binary: PathBuf,
    pub state_dir: PathBuf,
    pub model: String,
    pub relay: Arc<Loopback>,
}

struct Runtime {
    root: PathBuf,
    marker: PathBuf,
    scope: String,
    // Retain both loopback endpoints through original supplier-group cleanup.
    relay: Arc<Loopback>,
    _lease: Lease,
}
pub(crate) struct Prepared {
    runtime: Option<Runtime>,
}
pub(crate) struct Launched {
    runtime: Option<Runtime>,
    child: Option<Child>,
    outcome: Option<CleanupOutcome>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CleanupOutcome {
    /// Original group had no live members, leader reaped, owned files removed.
    /// Does not establish absence of descendants that escaped the group.
    GroupStopped,
    /// No more signals will be attempted; marker and runtime remain for refusal.
    Unknown,
}
fn prepare_error<T>(_: T) -> Error {
    Error::Prepare
}
fn random_scope() -> Result<String, Error> {
    let mut bytes = [0u8; 24];
    getrandom::fill(&mut bytes).map_err(prepare_error)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}
fn private_dir(path: &Path) -> Result<(), Error> {
    DirBuilder::new()
        .mode(0o700)
        .create(path)
        .map_err(prepare_error)
}
fn readonly_file(path: &Path, data: &[u8]) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(prepare_error)?;
    file.write_all(data).map_err(prepare_error)?;
    file.sync_all().map_err(prepare_error)?;
    file.set_permissions(fs::Permissions::from_mode(0o400))
        .map_err(prepare_error)
}
fn copy_pin(source: &Path, destination: &Path) -> Result<(), Error> {
    let mut input = File::open(source).map_err(prepare_error)?;
    if !input.metadata().map_err(prepare_error)?.is_file() {
        return Err(Error::PinMismatch);
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(destination)
        .map_err(prepare_error)?;
    let mut hash = Sha256::new();
    let mut count = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = input.read(&mut buffer).map_err(prepare_error)?;
        if n == 0 {
            break;
        }
        count = count.checked_add(n as u64).ok_or(Error::PinMismatch)?;
        if count > PIN_BYTES {
            return Err(Error::PinMismatch);
        }
        hash.update(&buffer[..n]);
        output.write_all(&buffer[..n]).map_err(prepare_error)?;
    }
    if count != PIN_BYTES || format!("{:x}", hash.finalize()) != PIN_SHA {
        return Err(Error::PinMismatch);
    }
    output.sync_all().map_err(prepare_error)?;
    output
        .set_permissions(fs::Permissions::from_mode(0o500))
        .map_err(prepare_error)
}
fn claim_marker(state: &Path, scope: &str) -> Result<PathBuf, Error> {
    // Caller owns this directory. Reject symlinks and group/other access rather
    // than changing permissions on someone else's persistent state.
    let meta = fs::symlink_metadata(state).map_err(prepare_error)?;
    if !meta.is_dir()
        || meta.permissions().mode() & 0o077 != 0
        || fs::canonicalize(state).map_err(prepare_error)? != state
    {
        return Err(Error::InvalidPath);
    }
    let marker = state.join("embedded-agent.scope");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&marker)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Error::StaleScope
            } else {
                Error::Prepare
            }
        })?;
    file.write_all(scope.as_bytes()).map_err(prepare_error)?;
    file.sync_all().map_err(prepare_error)?;
    File::open(state)
        .and_then(|f| f.sync_all())
        .map_err(prepare_error)?;
    Ok(marker)
}
impl Runtime {
    fn remove(self) -> bool {
        // std remove_dir_all does not follow symlinks; never traverse child paths
        // by canonicalizing them. Same-uid hostile parent mutation is out of scope.
        if fs::remove_dir_all(&self.root).is_err() {
            return false;
        }
        let own_marker = fs::symlink_metadata(&self.marker)
            .map(|m| m.is_file() && m.len() == self.scope.len() as u64)
            .unwrap_or(false)
            && fs::read(&self.marker)
                .map(|s| s == self.scope.as_bytes())
                .unwrap_or(false);
        own_marker && fs::remove_file(&self.marker).is_ok()
    }
}
pub(crate) fn prepare(config: LaunchConfig, lease: Lease) -> Result<Prepared, Error> {
    // Before random/path/profile/config/copy/hash allocations.
    if lease.bytes() < STARTUP_BYTES || lease.purpose() != LeasePurpose::Ingress {
        return Err(Error::StartupReservation);
    }
    let supplier_config = supplier_config(&config.model, &config.relay)?;
    let scope = random_scope()?;
    let marker = claim_marker(&config.state_dir, &scope)?;
    // Fixed OS temp avoids inherited TMPDIR and the real home entirely.
    let root = Path::new("/private/tmp").join(format!("swbpipe-agent-{scope}"));
    if let Err(e) = private_dir(&root) {
        let _ = fs::remove_file(&marker);
        return Err(e);
    }
    let runtime = Runtime {
        root,
        marker,
        scope,
        relay: config.relay,
        _lease: lease,
    };
    let result = (|| {
        let root = &runtime.root;
        for path in [
            root.join("home"),
            root.join("work"),
            root.join("home/codex"),
            root.join("home/tmp"),
            root.join("home/cache"),
            root.join("home/config"),
            root.join("home/data"),
        ] {
            private_dir(&path)?;
        }
        copy_pin(&config.binary, &root.join("codex"))?;
        readonly_file(&root.join("config.toml"), supplier_config.as_bytes())?;
        symlink(
            root.join("config.toml"),
            root.join("home/codex/config.toml"),
        )
        .map_err(prepare_error)?;
        let profile = sandbox::profile(
            root,
            &root.join("home"),
            &root.join("work"),
            &root.join("codex"),
            &runtime.relay,
        )
        .map_err(|_| Error::InvalidPath)?;
        readonly_file(&root.join("sandbox.sb"), profile.as_bytes())?;
        Ok(())
    })();
    if let Err(e) = result {
        runtime.remove();
        return Err(e);
    }
    Ok(Prepared {
        runtime: Some(runtime),
    })
}
impl Prepared {
    pub(crate) fn spawn(self) -> Result<Launched, Error> {
        self.spawn_inner(false)
    }
    fn spawn_inner(mut self, diagnostic_stderr: bool) -> Result<Launched, Error> {
        let runtime = self.runtime.as_ref().ok_or(Error::Spawn)?;
        let root = &runtime.root;
        let result = {
            let mut command = Command::new("/usr/bin/sandbox-exec");
            command
                .arg("-f")
                .arg(root.join("sandbox.sb"))
                .arg(root.join("codex"))
                .args(["app-server", "--listen", "stdio://"])
                .current_dir(root.join("work"))
                .env_clear()
                .env("HOME", root.join("home"))
                .env("CODEX_HOME", root.join("home/codex"))
                .env("TMPDIR", root.join("home/tmp"))
                .env("XDG_CACHE_HOME", root.join("home/cache"))
                .env("XDG_CONFIG_HOME", root.join("home/config"))
                .env("XDG_DATA_HOME", root.join("home/data"))
                .env("PATH", "/usr/bin:/bin")
                .env("LANG", "en_US.UTF-8")
                .env(CAPABILITY_ENV, runtime.relay.capability())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(if diagnostic_stderr {
                    Stdio::piped()
                } else {
                    Stdio::null()
                })
                .process_group(0);
            command.spawn()
        };
        let child = result.map_err(|_| Error::Spawn)?;
        Ok(Launched {
            runtime: self.runtime.take(),
            child: Some(child),
            outcome: None,
        })
    }
}
impl Drop for Prepared {
    fn drop(&mut self) {
        if let Some(runtime) = self.runtime.take() {
            runtime.remove();
        }
    }
}

// Helpers have no credential environment. Their numeric argument is obtained
// only from the retained actual Child, never from a marker or a caller.
fn utility(program: &str, args: &[&str]) -> Option<(bool, Vec<u8>)> {
    let mut child = Command::new(program)
        .args(args)
        .env_clear()
        .env("LC_ALL", "C")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let deadline = Instant::now() + Duration::from_millis(500);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut bytes = Vec::new();
                child
                    .stdout
                    .take()?
                    .take(65537)
                    .read_to_end(&mut bytes)
                    .ok()?;
                if bytes.len() > 65536 {
                    return None;
                }
                return Some((status.success(), bytes));
            }
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(10)),
            _ => {
                // This is only the utility Child, never the supplier leader.
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}
fn zombie_group(bytes: &[u8], leader: u32) -> Option<bool> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut saw_leader = false;
    let mut all_zombies = true;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let mut fields = line.split_whitespace();
        let pid: u32 = fields.next()?.parse().ok()?;
        let state = fields.next()?;
        if fields.next().is_some() || state.is_empty() {
            return None;
        }
        let zombie = state.starts_with('Z');
        if pid == leader {
            saw_leader = zombie;
        }
        all_zombies &= zombie;
    }
    // Empty census, EOF or successful kill is insufficient. Require the reserved
    // leader itself to be observed as a zombie before the sole original wait.
    Some(saw_leader && all_zombies)
}
fn contains_leader(bytes: &[u8], leader: u32) -> bool {
    std::str::from_utf8(bytes).ok().is_some_and(|text| {
        text.lines().any(|line| {
            let mut fields = line.split_whitespace();
            fields.next().and_then(|pid| pid.parse::<u32>().ok()) == Some(leader)
                && fields.next().is_some_and(|state| !state.is_empty())
                && fields.next().is_none()
        })
    })
}
fn stop_owned_group(child: &mut Child) -> bool {
    let group = child.id().to_string();
    let negative = format!("-{group}");
    // This census is a consistency check, NOT PID authority. Authority is the
    // original private Child, never polled/reaped or exposed to another caller.
    // Its reserved id is held across this check and every following group op.
    if !utility("/bin/ps", &["-o", "pid=,stat=", "-g", &group])
        .is_some_and(|(ok, bytes)| ok && contains_leader(&bytes, child.id()))
    {
        return false;
    }
    // Do NOT call try_wait/wait on child before the last group operation.
    // Darwin can refuse signaling zombie-only groups with EPERM. In either
    // success or failure, the independent census below must prove no live member.
    if utility("/bin/kill", &["-KILL", "--", &negative]).is_none() {
        return false;
    }
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        match utility("/bin/ps", &["-o", "pid=,stat=", "-g", &group]) {
            Some((true, bytes)) if zombie_group(&bytes, child.id()) == Some(true) => {
                // FINAL group observation above. No further signal or census
                // after this reap; the id may immediately be reused.
                return child.wait().is_ok();
            }
            Some((true, bytes)) if zombie_group(&bytes, child.id()) == Some(false) => {}
            _ => return false,
        }
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
impl Launched {
    pub(crate) fn work_dir(&self) -> Option<PathBuf> {
        self.runtime.as_ref().map(|r| r.root.join("work"))
    }
    /// Opaque connection scope only; it confers no session/domain authorization.
    pub(crate) fn connection_scope(&self) -> Option<&str> {
        self.runtime.as_ref().map(|r| r.scope.as_str())
    }
    /// The caller owns the sole writer after this one-time transfer. It must
    /// drop it at close; inherited EOF is not proof of parent-crash cleanup.
    pub(crate) fn take_stdin(&mut self) -> Option<ChildStdin> {
        self.child.as_mut()?.stdin.take()
    }
    pub(crate) fn take_stdout(&mut self) -> Option<ChildStdout> {
        self.child.as_mut()?.stdout.take()
    }
    pub(crate) fn close(&mut self) -> CleanupOutcome {
        if let Some(outcome) = self.outcome {
            return outcome;
        }
        let stopped = if let Some(child) = self.child.as_mut() {
            child.stdin.take();
            stop_owned_group(child)
        } else {
            false
        };
        let outcome = if stopped {
            self.child.take();
            if self.runtime.take().map(Runtime::remove).unwrap_or(false) {
                CleanupOutcome::GroupStopped
            } else {
                CleanupOutcome::Unknown
            }
        } else {
            CleanupOutcome::Unknown
        };
        // Even on Unknown, no delayed retry can signal a potentially stale id.
        self.outcome = Some(outcome);
        outcome
    }
}
impl Drop for Launched {
    fn drop(&mut self) {
        if self.close() == CleanupOutcome::Unknown {
            // Preserve reserved leader and supplied memory lease until parent
            // exit when cleanup is unknown. Marker refuses the next launch.
            if let Some(child) = self.child.take() {
                std::mem::forget(child);
            }
            if let Some(runtime) = self.runtime.take() {
                std::mem::forget(runtime);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use crate::solve_jobs::SolveJobRegistry;
    fn fixture() -> PathBuf {
        let root = Path::new("/private/tmp")
            .join(format!("swbpipe-launcher-test-{}", random_scope().unwrap()));
        private_dir(&root).unwrap();
        root
    }
    #[test]
    fn portable_configuration_excludes_provider_secrets_and_escapes_model() {
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let relay = Loopback::bind(&registry).unwrap();
        let text = supplier_config("invented \"model\" \\ variant", &relay).unwrap();
        assert!(text.starts_with("model = \"invented \\\"model\\\" \\\\ variant\"\n"));
        assert!(text.contains(&format!(
            "base_url = \"http://127.0.0.1:{}/v1\"",
            relay.port()
        )));
        assert!(text.contains("env_key = \"SWBPIPE_AGENT_RELAY_CAPABILITY\""));
        assert!(!text.contains(relay.capability()));
        assert!(!CONFIG_BODY.contains("8000"));
        assert!(!text.contains("Qwen"));
        assert!(!text.contains("OMLX"));
        for bad in ["", "x\ny", "x\0y", &"x".repeat(1025)] {
            assert_eq!(
                supplier_config(bad, &relay),
                Err(Error::InvalidConfiguration)
            );
        }
    }
    #[test]
    fn invalid_configuration_is_refused_before_runtime_or_marker_creation() {
        let state = fixture();
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let relay = Loopback::bind(&registry).unwrap();
        let lease = registry.acquire_interactive_ingress(STARTUP_BYTES).unwrap();
        let result = prepare(
            LaunchConfig {
                binary: state.join("never-opened"),
                state_dir: state.clone(),
                model: "bad\nmodel".into(),
                relay: relay.clone(),
            },
            lease,
        );
        assert!(matches!(result, Err(Error::InvalidConfiguration)));
        assert_eq!(fs::read_dir(&state).unwrap().count(), 0);
        assert_eq!(Arc::strong_count(&relay), 1);
        fs::remove_dir_all(state).unwrap();
    }
    #[test]
    fn stale_marker_refuses_without_numeric_recovery() {
        let root = fixture();
        let marker = claim_marker(&root, "invented-scope").unwrap();
        assert_eq!(claim_marker(&root, "other"), Err(Error::StaleScope));
        assert_eq!(fs::read_to_string(marker).unwrap(), "invented-scope");
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn mismatch_is_refused_before_execution() {
        let root = fixture();
        let source = root.join("invented-source");
        fs::write(&source, b"invented bytes, never executed").unwrap();
        assert_eq!(
            copy_pin(&source, &root.join("copy")),
            Err(Error::PinMismatch)
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn census_requires_reserved_zombie_and_no_live_members() {
        assert_eq!(zombie_group(b"42 Z\n43 Z+\n", 42), Some(true));
        assert_eq!(zombie_group(b"42 Z\n43 S\n", 42), Some(false));
        assert_eq!(zombie_group(b"", 42), Some(false));
        assert_eq!(zombie_group(b"43 Z\n", 42), Some(false));
        assert_eq!(zombie_group(b"42 ? extra\n", 42), None);
        assert_ne!(CleanupOutcome::Unknown, CleanupOutcome::GroupStopped);
    }
    #[test]
    #[ignore = "manager-owned pinned supplier/profile probe"]
    fn pinned_supplier_initializes_under_the_profile() {
        let binary = std::env::var_os("SWBPIPE_CODEX_BINARY").expect("selected binary path");
        let state = fixture();
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let lease = registry.acquire_interactive_ingress(STARTUP_BYTES).unwrap();
        let relay = Loopback::bind(&registry).unwrap();
        let prepared = prepare(
            LaunchConfig {
                binary: binary.into(),
                state_dir: state.clone(),
                model: "invented-native-probe-model".into(),
                relay: relay.clone(),
            },
            lease,
        )
        .unwrap();
        let mut process = prepared.spawn_inner(true).unwrap();
        let mut stdin = process.take_stdin().unwrap();
        let stdout = process.take_stdout().unwrap();
        let stderr = process.child.as_mut().unwrap().stderr.take().unwrap();
        let limits = super::super::catalog::Catalog::load().unwrap().limits();
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let reader = std::thread::spawn(move || {
            let mut frames = super::super::framing::Reader::new(stdout, registry, limits).unwrap();
            let _ = sender.send(frames.next());
        });
        stdin.write_all(b"{\"id\":1,\"method\":\"initialize\",\"params\":{\"clientInfo\":{\"name\":\"swbpipe_native_probe\",\"version\":\"0.1\"},\"capabilities\":{\"experimentalApi\":true}}}\n").unwrap();
        stdin.flush().unwrap();
        let received = receiver.recv_timeout(Duration::from_secs(10));
        let stopped = process.close();
        drop(stdin);
        if stopped == CleanupOutcome::GroupStopped {
            reader.join().unwrap();
        }
        let mut diagnostic = String::new();
        stderr.take(8192).read_to_string(&mut diagnostic).unwrap();
        assert!(!diagnostic.contains(relay.capability()));
        // No provider credential was supplied or read by this probe.
        assert_eq!(stopped, CleanupOutcome::GroupStopped);
        let frame = received
            .expect("supplier frame deadline")
            .expect("admitted frame");
        assert!(
            frame.is_some(),
            "isolated no-credential startup diagnostic: {diagnostic}"
        );
        let frame = frame.unwrap();
        assert_eq!(frame.value["id"], 1);
        assert!(frame.value["result"].is_object());
        fs::remove_dir_all(state).unwrap();
    }
    /// Opt-in native custody qualification; never run by this source assignment.
    #[test]
    #[ignore = "manager-owned native process probe"]
    fn owned_group_stop_preserves_unrelated_group() {
        let mut owned = Command::new("/bin/sleep")
            .arg("30")
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0)
            .spawn()
            .unwrap();
        let mut unrelated = Command::new("/bin/sleep")
            .arg("30")
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0)
            .spawn()
            .unwrap();
        let stopped = stop_owned_group(&mut owned);
        let unrelated_alive = matches!(unrelated.try_wait(), Ok(None));
        let _ = unrelated.kill();
        let _ = unrelated.wait();
        // On failure do not reap the original or make another group operation:
        // retain its id until this test process exits, just as production does.
        if !stopped {
            std::mem::forget(owned);
        }
        assert!(stopped);
        assert!(unrelated_alive);
    }
}
