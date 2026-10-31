//! Explicitly owned native session. Partial startup returns custody, not rollback.
use super::{
    http_relay::{Relay, State},
    loopback::Loopback,
    macos::{self, CleanupOutcome, LaunchConfig, Launched},
    protocol::{Event, ThreadOptions},
    relay_policy::{Provider, Secret},
    session_driver::{self, SessionDriver, StartError as DriverStart},
};
use crate::solve_jobs::SolveJobRegistry;
use serde_json::Value;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
pub(super) const INSTRUCTIONS:&str="Use only the declared SWBPIPE catalog tools. Model changes are proposals for the person's review; never apply or approve them. Preserve rationale and assumptions. Internal results are screening and development evidence, not professional acceptance or code compliance.";
pub(super) struct Paths {
    pub resources: PathBuf,
    pub state: PathBuf,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    Admission,
    Relay,
    Prepare,
    Resource(macos::Error),
    Spawn,
    Observation,
    Transport,
    Protocol,
    Cancelled,
}
impl Error {
    pub fn class(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::Relay => "relay",
            Self::Prepare => "prepare",
            Self::Resource(reason) => match reason {
                macos::Error::InvalidConfiguration => "invalid_launch_configuration",
                macos::Error::StartupReservation => "startup_reservation",
                macos::Error::InvalidPath => "invalid_native_path",
                macos::Error::StaleScope => "stale_runtime_scope",
                macos::Error::Prepare => "private_runtime_prepare",
                macos::Error::Spawn => "supplier_spawn",
                macos::Error::Supplier(reason) => match reason {
                    super::supplier::Error::Admission => "supplier_admission",
                    super::supplier::Error::UnsupportedPlatform => "supplier_platform",
                    super::supplier::Error::InvalidManifest => "supplier_manifest",
                    super::supplier::Error::UnavailableResource => "supplier_resource_unavailable",
                    super::supplier::Error::InvalidResource => "supplier_resource_invalid",
                    super::supplier::Error::PinMismatch => "supplier_pin_mismatch",
                    super::supplier::Error::CopyFailed => "supplier_private_copy",
                },
            },
            Self::Spawn => "spawn",
            Self::Observation => "observation",
            Self::Transport => "transport",
            Self::Protocol => "protocol",
            Self::Cancelled => "cancelled",
        }
    }
}
/// Terminal lifecycle standing is separate from successful cleanup and domain
/// outcomes. Cleanup must never erase its cause or leave a workspace open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum End {
    Requested,
    SupplierExit(Option<i32>),
    Eof,
    Failure(Error),
}
impl End {
    pub fn failure(self) -> Option<&'static str> {
        match self {
            Self::Failure(reason) => Some(reason.class()),
            Self::SupplierExit(status) if status != Some(0) => Some("supplier_exit"),
            _ => None,
        }
    }
    fn driver(terminal: session_driver::Terminal, eof: bool) -> Self {
        match terminal {
            session_driver::Terminal::StopRequested => Self::Requested,
            session_driver::Terminal::Io if eof => Self::Eof,
            session_driver::Terminal::Protocol => Self::Failure(Error::Protocol),
            session_driver::Terminal::Admission => Self::Failure(Error::Admission),
            _ => Self::Failure(Error::Transport),
        }
    }
}
pub(super) enum StartError {
    Refused(Error),
    Partial(Error, Live),
}
pub(super) struct Live {
    driver: Option<SessionDriver>,
    process: Option<Launched>,
    relay: Relay,
    model: String,
    context_window: u32,
    stage: u8,
    stopped: bool,
    eof_only: bool,
    end: Option<End>,
    pub cleanup: Option<CleanupOutcome>,
}
impl Live {
    pub fn start(
        registry: &SolveJobRegistry,
        paths: Paths,
        model: String,
        context_window: u32,
        provider: Provider,
        secret: Option<Secret>,
        token: &str,
        basis: &Value,
        cancel: &Arc<AtomicBool>,
    ) -> Result<Self, StartError> {
        let scope = Loopback::bind(registry).map_err(|_| StartError::Refused(Error::Admission))?;
        let relay = Relay::start(registry, scope, provider, secret)
            .map_err(|_| StartError::Refused(Error::Relay))?;
        let mut live = Self {
            driver: None,
            process: None,
            relay,
            model,
            context_window,
            stage: 0,
            stopped: false,
            eof_only: false,
            end: None,
            cleanup: None,
        };
        let result = (|| {
            let deadline = Instant::now() + Duration::from_secs(5);
            while live.relay.poll().state != State::Ready {
                if cancel.load(Ordering::Acquire) || Instant::now() >= deadline {
                    return Err(Error::Relay);
                }
                std::thread::sleep(Duration::from_millis(2));
            }
            let lease = registry
                .acquire_interactive_ingress(macos::STARTUP_BYTES)
                .map_err(|_| Error::Admission)?;
            let endpoint = live.relay.endpoint().map_err(|_| Error::Relay)?;
            let prepared = macos::prepare(
                LaunchConfig {
                    resource_dir: paths.resources,
                    state_dir: paths.state,
                    model: live.model.clone(),
                    context_window: live.context_window,
                    relay: endpoint,
                },
                lease,
            )
            .map_err(Error::Resource)?;
            if cancel.load(Ordering::Acquire) {
                return Err(Error::Cancelled);
            }
            live.process = Some(prepared.spawn().map_err(|_| Error::Spawn)?);
            let process = live.process.as_mut().unwrap();
            let observed = process.observe_scope();
            if observed.invalid.is_some() || observed.exit_observed {
                return Err(Error::Observation);
            }
            #[cfg(feature = "native-smoke")]
            if let Some(gate) = std::env::var_os("SWBPIPE_NATIVE_SMOKE_AGENT_GATE") {
                use std::io::Read;
                let (sdk, scope) = process.fixture_identity().ok_or(Error::Observation)?;
                eprintln!(
                    "[native-smoke-agent-scope] {}",
                    serde_json::json!({"appPid":std::process::id(),"sdkPid":sdk,"scope":scope})
                );
                // The original SDK Child remains unreaped here, before the
                // protocol gate. The owned controller establishes its rescue
                // anchor, then writes one fixed nonsecret readiness record.
                let deadline = Instant::now() + Duration::from_secs(10);
                loop {
                    let mut bytes = Vec::new();
                    if let Ok(file) = std::fs::File::open(&gate) {
                        if file.take(8).read_to_end(&mut bytes).is_ok() && bytes == b"READY\n" {
                            break;
                        }
                    }
                    if cancel.load(Ordering::Acquire) || Instant::now() >= deadline {
                        return Err(Error::Cancelled);
                    }
                    std::thread::sleep(Duration::from_millis(5));
                }
                let observed = process.observe_scope();
                if observed.invalid.is_some() || observed.exit_observed {
                    return Err(Error::Observation);
                }
            }
            let stdout = process.take_stdout().ok_or(Error::Transport)?;
            let stdin = process.take_stdin().ok_or(Error::Transport)?;
            match SessionDriver::start(stdout, stdin, registry.clone(), token, 16, basis) {
                Ok(driver) => live.driver = Some(driver),
                Err(DriverStart::Partial { custody, .. }) => {
                    live.driver = Some(custody);
                    return Err(Error::Transport);
                }
                Err(_) => return Err(Error::Transport),
            }
            Ok(())
        })();
        match result {
            Ok(()) => Ok(live),
            Err(error) => {
                live.stop();
                Err(StartError::Partial(error, live))
            }
        }
    }
    pub fn driver(&mut self) -> Option<&mut SessionDriver> {
        self.driver.as_mut()
    }
    pub fn terminal(&self) -> Option<End> {
        self.end
    }
    fn fail(&mut self, error: Error) {
        self.end.get_or_insert(End::Failure(error));
        self.stop();
    }
    pub fn ready(&self) -> bool {
        self.stage == 3 && !self.stopped
    }
    pub fn poll(&mut self) -> Result<Option<session_driver::EventBatch>, Error> {
        if let Some(process) = &mut self.process {
            let scope = process.observe_scope();
            if scope.invalid.is_some() {
                self.fail(Error::Observation);
                return Err(Error::Observation);
            }
            if scope.exit_observed && !self.stopped {
                self.end = Some(End::SupplierExit(scope.exit_status_raw));
                self.stop();
            }
        }
        if self.driver.is_none() {
            return Ok(None);
        }
        if self.stage == 0 && !self.stopped {
            match self
                .driver
                .as_mut()
                .unwrap()
                .initialize("swbpipe", env!("CARGO_PKG_VERSION"))
            {
                Ok(_) => self.stage = 1,
                Err(session_driver::Refusal::Busy) => {}
                Err(_) => {
                    self.fail(Error::Protocol);
                    return Err(Error::Protocol);
                }
            }
        }
        if self.stage == 2 && !self.stopped {
            let cwd = self
                .process
                .as_ref()
                .and_then(Launched::work_dir)
                .ok_or(Error::Prepare)?
                .to_str()
                .ok_or(Error::Prepare)?
                .to_owned();
            match self.driver.as_mut().unwrap().start_thread(&ThreadOptions {
                model: self.model.clone(),
                provider: "swbpipe_relay".into(),
                cwd,
                base_instructions: INSTRUCTIONS.into(),
            }) {
                Ok(_) => self.stage = 4,
                Err(session_driver::Refusal::Busy) => {}
                Err(_) => {
                    self.fail(Error::Protocol);
                    return Err(Error::Protocol);
                }
            }
        }
        let observation = match self.driver.as_mut().unwrap().poll() {
            Ok(value) => value,
            Err(session_driver::Refusal::Busy) => return Ok(None),
            Err(_) => {
                self.fail(Error::Transport);
                return Err(Error::Transport);
            }
        };
        if let Some(batch) = &observation.events {
            for event in batch.events() {
                match event {
                    Event::Initialized => self.stage = 2,
                    Event::ThreadStarted { .. } => self.stage = 3,
                    _ => {}
                }
            }
        }
        if let Some(terminal) = observation.terminal {
            if !self.stopped && !self.eof_only {
                self.end = Some(End::driver(
                    terminal,
                    observation.io.reader_io == Some(super::stdio_pump::IoEnd::Eof),
                ));
                self.stop();
            }
        }
        Ok(observation.events)
    }
    #[cfg(any(test, feature = "native-smoke"))]
    pub fn request_eof(&mut self) {
        self.eof_only = true;
        if let Some(driver) = &mut self.driver {
            driver.stop();
        }
    }
    #[cfg(any(test, feature = "native-smoke"))]
    pub fn fixture_identity(&self) -> Option<(u32, &str)> {
        self.process.as_ref().and_then(|p| p.fixture_identity())
    }
    #[cfg(any(test, feature = "native-smoke"))]
    pub fn fixture_scope(&mut self) -> Option<super::process_scope::Status> {
        self.process.as_mut().map(Launched::observe_scope)
    }
    pub fn stop(&mut self) {
        if self.stopped {
            return;
        }
        self.end.get_or_insert(End::Requested);
        self.stopped = true;
        if let Some(driver) = &mut self.driver {
            driver.stop();
        }
        self.cleanup = Some(
            self.process
                .as_mut()
                .map(Launched::close)
                .unwrap_or(CleanupOutcome::GroupStopped),
        );
        self.relay.request_stop();
    }
    pub fn joined(&mut self) -> bool {
        if !self.stopped {
            return false;
        }
        let workers = match self.driver.as_mut() {
            None => true,
            Some(driver) => match driver.poll() {
                Ok(value) => {
                    value.io.reader_completion == super::stdio_pump::Completion::Joined
                        && value.io.writer_completion == super::stdio_pump::Completion::Joined
                }
                Err(_) => false,
            },
        };
        workers && self.relay.poll().joined
    }
}
impl Drop for Live {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn terminal_eof_and_malformed_protocol_keep_distinct_lifecycle_causes() {
        assert_eq!(End::driver(session_driver::Terminal::Io, true), End::Eof);
        assert_eq!(End::Eof.failure(), None);
        assert_eq!(
            End::driver(session_driver::Terminal::Protocol, false).failure(),
            Some("protocol")
        );
        assert_eq!(
            End::driver(session_driver::Terminal::Io, false).failure(),
            Some("transport")
        );
    }
}
