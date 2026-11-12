//! Lazy native command owner. Workspace handles are public identity, never
//! execution authority. A distinct private capability authenticates the registered
//! workspace execution producer; completion bytes cannot mint owner evidence.
use super::{
    catalog::{Catalog, Tool},
    dispatcher::{Dispatcher, Evidence, History, Origin},
    keychain_native::NativeKeychain,
    live_runtime::{End, Live, Paths as RuntimePaths, StartError},
    macos::CleanupOutcome,
    numeric,
    protocol::Event,
    provider_store::{self, CredentialChange, Store},
    relay_policy::{Provider, Secret},
    webview_bridge::Bridge,
};
use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    os::unix::fs::{DirBuilderExt, PermissionsExt},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
/// Selected stock ARM pin/profile passed retained-bundle native qualification.
/// Intel remains unavailable until its own matrix passes; no env override.
#[cfg(target_arch = "aarch64")]
pub(crate) const NATIVE_QUALIFIED: bool = true;
#[cfg(not(target_arch = "aarch64"))]
pub(crate) const NATIVE_QUALIFIED: bool = false;
const STATE_BYTES: u64 = 32 * 1024 * 1024;
const THREAD_BYTES: u64 = 2 * 1024 * 1024;
// Escaped text plus 128 bounded item IDs, Value storage and serialization.
const RESPONSE_BYTES: u64 = 1024 * 1024;
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Error {
    Unavailable,
    Authority,
    Input,
    Admission,
    Busy,
    Capacity,
    Workspace,
    Evidence,
    Duplicate,
    Worker,
    Session,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Authority {
    workspace_handle: String,
    application_generation: u64,
    owner_capability: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct BasisDraft {
    model_revision: u64,
    model_hash: Value,
    state_id: Option<String>,
    run_id: Option<String>,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Action {
    Availability,
    Register {
        basis: BasisDraft,
    },
    Basis {
        authority: Authority,
        basis: BasisDraft,
    },
    ProviderSnapshot,
    ProviderSave {
        expected: Option<String>,
        endpoint: String,
        model: String,
        /// Required: no window is assumed for the person's provider.
        #[serde(rename = "contextWindow")]
        context_window: u32,
        #[serde(rename = "credentialMode")]
        credential_mode: String,
        key: Option<String>,
    },
    ProviderClear {
        expected: String,
    },
    ProviderRecover,
    Start {
        authority: Authority,
        #[serde(rename = "providerRevision")]
        provider_revision: String,
    },
    Text {
        authority: Authority,
        text: String,
    },
    Interrupt {
        authority: Authority,
    },
    Close {
        authority: Authority,
    },
    Poll {
        authority: Authority,
    },
    Claim {
        authority: Authority,
        #[serde(rename = "requestRef")]
        request_ref: String,
    },
    PublishActual {
        authority: Authority,
        #[serde(rename = "requestRef")]
        request_ref: String,
        outcome: String,
        proof: Proof,
    },
    Complete {
        authority: Authority,
        #[serde(rename = "requestRef")]
        request_ref: String,
        outcome: String,
    },
    TakeActual {
        authority: Authority,
        #[serde(rename = "requestRef")]
        request_ref: String,
    },
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Proof {
    Read,
    Failure,
    Proposal {
        handle: String,
        basis: Value,
        status: String,
        #[serde(rename = "outcomeModelHash")]
        outcome_model_hash: Option<Value>,
    },
    Run {
        handle: Option<String>,
        #[serde(rename = "runId")]
        run_id: Option<String>,
        basis: Value,
        status: String,
        #[serde(rename = "nativeJobId")]
        native_job_id: Option<String>,
        /// A result read's values' native row ordinals, in page order.
        ordinals: Option<Vec<u64>>,
    },
}
pub(crate) struct Paths {
    pub data: PathBuf,
    pub resources: PathBuf,
}
pub(crate) struct Reply {
    pub value: Result<Value, Error>,
    pub _backing: Lease,
}
struct Request {
    webview: String,
    action: Action,
    paths: Paths,
    reply: tokio::sync::oneshot::Sender<Reply>,
    backing: Lease,
}
struct Worker {
    sender: mpsc::SyncSender<Request>,
    shutdown: Arc<AtomicBool>,
    ready: Arc<AtomicBool>,
    exit: Arc<Mutex<Option<tauri::AppHandle>>>,
    join: Option<JoinHandle<()>>,
}
pub(crate) struct Host {
    registry: SolveJobRegistry,
    worker: Mutex<Option<Worker>>,
}
fn nonce() -> Result<String, Error> {
    let mut bytes = [0; 24];
    getrandom::fill(&mut bytes).map_err(|_| Error::Worker)?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
fn private(path: &std::path::Path) -> Result<PathBuf, Error> {
    if !path.exists() {
        let mut builder = fs::DirBuilder::new();
        builder
            .recursive(true)
            .mode(0o700)
            .create(path)
            .map_err(|_| Error::Unavailable)?;
    }
    let meta = fs::symlink_metadata(path).map_err(|_| Error::Unavailable)?;
    if !meta.is_dir() || meta.permissions().mode() & 0o077 != 0 {
        return Err(Error::Unavailable);
    }
    fs::canonicalize(path).map_err(|_| Error::Unavailable)
}
impl Host {
    pub fn new(registry: SolveJobRegistry) -> Self {
        Self {
            registry,
            worker: Mutex::new(None),
        }
    }
    pub async fn request(
        &self,
        webview: &str,
        action: Action,
        paths: Paths,
        backing: Lease,
    ) -> Result<Reply, Error> {
        let (reply, received) = tokio::sync::oneshot::channel();
        {
            let mut slot = self.worker.lock().map_err(|_| Error::Worker)?;
            if slot.is_none() {
                let state = self
                    .registry
                    .acquire_interactive_ingress(STATE_BYTES)
                    .map_err(|_| Error::Admission)?;
                let stack = self
                    .registry
                    .acquire_interactive_ingress(THREAD_BYTES)
                    .map_err(|_| Error::Admission)?;
                let (sender, receiver) = mpsc::sync_channel(16);
                let registry = self.registry.clone();
                let shutdown = Arc::new(AtomicBool::new(false));
                let ready = Arc::new(AtomicBool::new(false));
                let stop = shutdown.clone();
                let done = ready.clone();
                let exit: Arc<Mutex<Option<tauri::AppHandle>>> = Arc::new(Mutex::new(None));
                let exit_after_cleanup = exit.clone();
                let join = thread::Builder::new()
                    .name("swbpipe-agent-owner".into())
                    .stack_size(1024 * 1024)
                    .spawn(move || {
                        let (_state, _stack) = (state, stack);
                        let mut owner = Owner::new(registry, stop.clone());
                        loop {
                            owner.tick();
                            if stop.load(Ordering::Acquire) {
                                owner.close_live();
                                if owner.closed_clean() {
                                    done.store(true, Ordering::Release);
                                    if let Ok(mut callback) = exit_after_cleanup.lock() {
                                        if let Some(app) = callback.take() {
                                            app.exit(0);
                                        }
                                    }
                                    break;
                                }
                            }
                            match receiver.recv_timeout(Duration::from_millis(10)) {
                                Ok(request) => owner.handle(request),
                                Err(mpsc::RecvTimeoutError::Timeout) => {}
                                Err(_) => {
                                    stop.store(true, Ordering::Release);
                                }
                            }
                        }
                    })
                    .map_err(|_| Error::Worker)?;
                *slot = Some(Worker {
                    sender,
                    shutdown,
                    ready,
                    exit,
                    join: Some(join),
                });
            }
            let worker = slot.as_ref().unwrap();
            if worker.shutdown.load(Ordering::Acquire) {
                return Err(Error::Unavailable);
            }
            worker
                .sender
                .try_send(Request {
                    webview: webview.into(),
                    action,
                    paths,
                    reply,
                    backing,
                })
                .map_err(|error| match error {
                    mpsc::TrySendError::Full(_) => Error::Busy,
                    mpsc::TrySendError::Disconnected(_) => Error::Worker,
                })?;
        }
        received.await.map_err(|_| Error::Worker)
    }
    pub fn shutdown(&self, app: &tauri::AppHandle) -> bool {
        let Ok(mut slot) = self.worker.lock() else {
            return false;
        };
        let Some(worker) = slot.as_mut() else {
            return true;
        };
        if worker.ready.load(Ordering::Acquire) {
            if worker.join.as_ref().is_some_and(JoinHandle::is_finished) {
                if let Some(join) = worker.join.take() {
                    let _ = join.join();
                }
            }
            return true; // native scopes/workers are closed; actor is exiting.
        }
        if let Ok(mut callback) = worker.exit.lock() {
            *callback = Some(app.clone());
        }
        worker.shutdown.store(true, Ordering::Release);
        false
    }
}
impl Drop for Host {
    fn drop(&mut self) {
        if let Ok(slot) = self.worker.get_mut() {
            if let Some(worker) = slot {
                worker.shutdown.store(true, Ordering::Release);
            }
        }
    }
}
struct Meta {
    tool: Tool,
    basis: Value,
    /// The registry's last job number when the call was claimed: a job a solve's
    /// completion names must be newer (made after the claim, which precedes execution).
    jobs_before: u64,
    /// A result read's admitted arguments (its filter, page and run handle); null
    /// for other tools.
    arguments: Value,
}
struct Actual {
    raw: String,
    proof: Proof,
    _backing: Lease,
}
struct Workspace {
    webview: String,
    handle: String,
    generation: u64,
    capability: String,
    token: String,
    basis: Value,
    bridge: Bridge,
    claims: BTreeMap<String, Meta>,
    actual: BTreeMap<String, Actual>,
    /// Solve jobs a completion has named: each answers one call only.
    consumed_jobs: BTreeSet<String>,
    /// Each completed run handle's native job and the SHA-256 of that job's result
    /// bytes, read from the registry when the solve completed: a read through the
    /// handle must name that job, still holding those bytes.
    runs: BTreeMap<String, (String, [u8; 32])>,
    /// Each result page's continuation, recorded when the page was verified: the
    /// cursor names its run handle, the call's filter and where the page ended in
    /// the filtered rows, so the next page starts exactly there.
    cursors: BTreeMap<String, (String, Value, u64)>,
    notices: Vec<String>,
    messages: Vec<Value>,
    turn_active: bool,
    turn_id: Option<String>,
    turn_status: &'static str,
    message_bytes: usize,
    closed: bool,
}
impl Workspace {
    fn origin(&self) -> Origin<'_> {
        Origin {
            session: &self.token,
            webview: &self.webview,
            workspace: &self.handle,
            generation: self.generation,
        }
    }
    fn authority(&self, webview: &str, auth: &Authority) -> bool {
        self.webview == webview
            && self.handle == auth.workspace_handle
            && self.generation == auth.application_generation
            && self.capability == auth.owner_capability
    }
    fn push_message(&mut self, item_id: &str, text: &str, final_message: bool) -> bool {
        if self.messages.len() >= 128 || self.message_bytes + text.len() > 64 * 1024 {
            return false;
        }
        self.message_bytes += text.len();
        self.messages.push(
            json!({"itemId":item_id,"kind":if final_message{"final"}else{"delta"},"text":text}),
        );
        true
    }
    fn history(&self, target: &str) -> Option<History<'_>> {
        for record in self.actual.values() {
            match &record.proof {
                Proof::Proposal { handle, basis, .. } if handle == target => {
                    return Some(History::Proposal { handle, basis })
                }
                Proof::Run {
                    handle: Some(handle),
                    basis,
                    ..
                } if handle == target => return Some(History::Run { handle, basis }),
                _ => {}
            }
        }
        None
    }
}
struct Owner {
    registry: SolveJobRegistry,
    shutdown: Arc<AtomicBool>,
    workspaces: Vec<Workspace>,
    live: Option<Live>,
    live_index: Option<usize>,
    generation: u64,
    stage: &'static str,
    failure: Option<&'static str>,
}
impl Owner {
    fn new(registry: SolveJobRegistry, shutdown: Arc<AtomicBool>) -> Self {
        Self {
            registry,
            shutdown,
            workspaces: Vec::with_capacity(4),
            live: None,
            live_index: None,
            generation: 0,
            stage: "closed",
            failure: None,
        }
    }
    fn close_live(&mut self) {
        if let Some(i) = self.live_index {
            self.workspaces[i].closed = true;
            self.workspaces[i].bridge.close();
        }
        if let Some(live) = &mut self.live {
            live.stop();
            self.stage = "stopping";
        }
    }
    fn retire_terminal(&mut self, index: usize, end: End) {
        let owner = &mut self.workspaces[index];
        owner.closed = true;
        owner.turn_active = false;
        owner.bridge.close();
        owner.notices.clear();
        if let Some(reason) = end.failure() {
            self.failure = Some(reason);
        }
    }
    fn closed_clean(&mut self) -> bool {
        if let (Some(index), Some(end)) =
            (self.live_index, self.live.as_ref().and_then(Live::terminal))
        {
            self.retire_terminal(index, end);
        }
        if let Some(live) = &mut self.live {
            if !live.joined() {
                return false;
            }
            if live.cleanup != Some(CleanupOutcome::GroupStopped) {
                self.stage = "cleanup_unknown";
                return false;
            }
        }
        self.live = None;
        self.live_index = None;
        self.stage = "closed";
        true
    }
    fn tick(&mut self) {
        let Some(index) = self.live_index else {
            return;
        };
        let Some(live) = &mut self.live else {
            return;
        };
        match live.poll() {
            Ok(Some(batch)) => {
                let owner = &mut self.workspaces[index];
                for event in batch.events() {
                    if owner.bridge.observe(event).is_err() {
                        owner.closed = true;
                        live.stop();
                        break;
                    }
                    match event {
                        Event::ToolRequested {
                            tool, arguments, ..
                        } => {
                            let target = match tool {
                                Tool::ObserveProposal => {
                                    arguments.get("proposalHandle").and_then(Value::as_str)
                                }
                                Tool::ReadResults => {
                                    arguments.get("runHandle").and_then(Value::as_str)
                                }
                                _ => None,
                            };
                            // Borrow only the retained immutable history while bridge mutates.
                            let history =
                                target
                                    .and_then(|target| owner.history(target))
                                    .map(|history| match history {
                                        History::Proposal { handle, basis } => {
                                            (true, handle.to_owned(), basis.clone())
                                        }
                                        History::Run { handle, basis } => {
                                            (false, handle.to_owned(), basis.clone())
                                        }
                                        History::Live => unreachable!(),
                                    });
                            let history = match &history {
                                Some((true, h, b)) => History::Proposal {
                                    handle: h,
                                    basis: b,
                                },
                                Some((false, h, b)) => History::Run {
                                    handle: h,
                                    basis: b,
                                },
                                None => History::Live,
                            };
                            match owner.bridge.offer(event, &owner.basis, history) {
                                Ok(notice) => owner.notices.push(notice.request_ref.into()),
                                Err(
                                    super::dispatcher::Refusal::WrongBasis
                                    | super::dispatcher::Refusal::Unknown
                                    | super::dispatcher::Refusal::MissingEvidence,
                                ) => {
                                    // No domain work was admitted. A bounded refusal is a
                                    // protocol response, never an invented actual outcome.
                                    if let Event::ToolRequested { request_ref, .. } = event {
                                        let result = self
                                            .registry
                                            .acquire_interactive_ingress(2 * 1024 * 1024);
                                        if let (Ok(backing), Some(driver)) = (result, live.driver())
                                        {
                                            if driver.complete(request_ref.as_str(),json!({"status":"unavailable","basis":owner.basis,"message":"The requested historical record is not held by this session.","data":null}),Arc::new(backing)).is_ok(){continue;}
                                        }
                                    }
                                    owner.closed = true;
                                    live.stop();
                                    break;
                                }
                                Err(_) => {
                                    owner.closed = true;
                                    live.stop();
                                    break;
                                }
                            }
                        }
                        Event::TurnStarted { turn_id, .. } => {
                            owner.turn_active = true;
                            owner.turn_id = Some(turn_id.clone());
                            owner.turn_status = "in_progress";
                        }
                        Event::TurnCompleted { status, .. } => {
                            owner.turn_active = false;
                            owner.turn_status = match status.as_str() {
                                "completed" => "completed",
                                "interrupted" => "interrupted",
                                _ => "failed",
                            };
                        }
                        Event::MessageDelta { text, item_id }
                        | Event::MessageFinal { text, item_id } => {
                            if !owner.push_message(
                                item_id,
                                text,
                                matches!(event, Event::MessageFinal { .. }),
                            ) {
                                owner.closed = true;
                                self.failure = Some("message_limit");
                                live.stop();
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(None) => {}
            Err(reason) => {
                self.failure = Some(reason.class());
                self.stage = "session_failed";
                self.workspaces[index].closed = true;
                self.workspaces[index].bridge.close();
                live.stop();
            }
        }
        if live.ready() {
            self.stage = "ready";
        }
        if live.cleanup.is_some() {
            self.closed_clean();
        }
    }
    fn workspace(&self, webview: &str, auth: &Authority) -> Result<usize, Error> {
        self.workspaces
            .iter()
            .position(|w| w.authority(webview, auth))
            .ok_or(Error::Authority)
    }
    fn handle(&mut self, request: Request) {
        let Request {
            webview,
            action,
            paths,
            reply,
            backing,
        } = request;
        let response = self.registry.acquire_interactive_ingress(RESPONSE_BYTES);
        let value = if response.is_err() {
            Err(Error::Admission)
        } else {
            self.action(&webview, action, paths, backing)
        };
        if let Ok(_backing) = response {
            let _ = reply.send(Reply { value, _backing });
        }
    }
    fn action(
        &mut self,
        webview: &str,
        action: Action,
        paths: Paths,
        _backing: Lease,
    ) -> Result<Value, Error> {
        match action {
            Action::Availability => Ok(
                json!({"available":NATIVE_QUALIFIED,"reason":if NATIVE_QUALIFIED{"qualified_mac_runtime"}else{"native_qualification_pending"}}),
            ),
            Action::Register { basis } => {
                self.close_live();
                if !self.closed_clean() {
                    return Err(Error::Busy);
                }
                // A closed registration with no claimed work has no possible
                // domain outcome. Reclaim it for recoverable settings retries;
                // retain every context that may still own a late actual.
                self.workspaces.retain(|owner| {
                    !owner.closed || !owner.claims.is_empty() || !owner.actual.is_empty()
                });
                if self.workspaces.len() >= 4 {
                    return Err(Error::Capacity);
                }
                self.generation = self.generation.checked_add(1).ok_or(Error::Capacity)?;
                let handle = nonce()?;
                let capability = nonce()?;
                let token = nonce()?;
                let full = basis_value(&handle, self.generation, basis);
                let dispatcher = Dispatcher::new(self.registry.clone(), &token, webview, &full, 16)
                    .map_err(|_| Error::Input)?;
                let bridge =
                    Bridge::new(self.registry.clone(), dispatcher).map_err(|_| Error::Admission)?;
                let result = json!({"workspaceHandle":handle,"applicationGeneration":self.generation,"ownerCapability":capability,"basis":full});
                self.workspaces.push(Workspace {
                    webview: webview.into(),
                    handle,
                    generation: self.generation,
                    capability,
                    token,
                    basis: full,
                    bridge,
                    claims: BTreeMap::new(),
                    actual: BTreeMap::new(),
                    consumed_jobs: BTreeSet::new(),
                    runs: BTreeMap::new(),
                    cursors: BTreeMap::new(),
                    notices: Vec::with_capacity(64),
                    messages: Vec::new(),
                    turn_active: false,
                    turn_id: None,
                    turn_status: "none",
                    message_bytes: 0,
                    closed: false,
                });
                Ok(result)
            }
            Action::Basis { authority, basis } => {
                let i = self.workspace(webview, &authority)?;
                let owner = &mut self.workspaces[i];
                if owner.closed {
                    return Err(Error::Workspace);
                }
                let next = basis_value(&owner.handle, owner.generation, basis);
                if next["modelRevision"].as_u64() < owner.basis["modelRevision"].as_u64()
                    || (next["modelRevision"] == owner.basis["modelRevision"]
                        && next != owner.basis)
                {
                    return Err(Error::Workspace);
                }
                let cat = Catalog::load().map_err(|_| Error::Input)?;
                cat.validate_output(Tool::Summary,&json!({"status":"unavailable","basis":next,"message":"Workspace update","data":null})).map_err(|_|Error::Input)?;
                owner.basis = next;
                if self.live_index == Some(i) {
                    if let Some(driver) = self.live.as_mut().and_then(Live::driver) {
                        driver
                            .set_error_basis(&owner.basis)
                            .map_err(|_| Error::Session)?;
                    }
                }
                Ok(json!({"updated":true}))
            }
            Action::ProviderSnapshot
            | Action::ProviderSave { .. }
            | Action::ProviderClear { .. }
            | Action::ProviderRecover => {
                if self.live.is_some() {
                    return Err(Error::Busy);
                }
                let root = private(&paths.data.join("embedded-agent/provider"))?;
                let keys = NativeKeychain::new(&self.registry).map_err(|_| Error::Unavailable)?;
                let mut store = Store::open(&root, keys, self.registry.clone())
                    .map_err(|_| Error::Unavailable)?;
                let result = match action {
                    Action::ProviderSnapshot => store.snapshot(),
                    Action::ProviderRecover => store.recover(),
                    Action::ProviderClear { expected } => store.clear(&expected),
                    Action::ProviderSave {
                        expected,
                        endpoint,
                        model,
                        context_window,
                        credential_mode,
                        key,
                    } => {
                        let change = match (credential_mode.as_str(), key) {
                            ("keep", None) => CredentialChange::Keep,
                            ("no_key", None) => CredentialChange::NoKey,
                            ("replace", Some(value)) => CredentialChange::Replace(
                                Secret::new(value).map_err(|_| Error::Input)?,
                            ),
                            _ => return Err(Error::Input),
                        };
                        store.save(expected.as_deref(), &endpoint, &model, context_window, change)
                    }
                    _ => unreachable!(),
                };
                Ok(match result {
                    Ok(snapshot) => json!({"status":"ok","snapshot":snapshot}),
                    Err(error) => {
                        json!({"status":"credential_error","error":error,"message":error.user_message()})
                    }
                })
            }
            Action::Start {
                authority,
                provider_revision,
            } => {
                if !NATIVE_QUALIFIED {
                    return Err(Error::Unavailable);
                }
                let i = self.workspace(webview, &authority)?;
                if self.live.is_some() || self.workspaces[i].closed {
                    return Err(Error::Busy);
                }
                let root = private(&paths.data.join("embedded-agent/provider"))?;
                let keys = NativeKeychain::new(&self.registry).map_err(|_| Error::Unavailable)?;
                let mut store = Store::open(&root, keys, self.registry.clone())
                    .map_err(|_| Error::Unavailable)?;
                let mut config = match store.load(&provider_revision) {
                    Ok(config) => config,
                    Err(error) => {
                        return Ok(
                            json!({"status":"credential_error","error":error,"message":error.user_message()}),
                        )
                    }
                };
                let provider =
                    Provider::new(config.provider.endpoint()).map_err(|_| Error::Input)?;
                let runtime_paths = RuntimePaths {
                    resources: paths.resources,
                    state: private(&paths.data.join("embedded-agent/runtime"))?,
                };
                #[allow(unused_mut)]
                let mut secret = config.secret.take();
                // Native smoke only (RT-E1 D4): a provider saved with no key
                // takes the key the external controller passed on stdin, once.
                #[cfg(feature = "native-smoke")]
                if secret.is_none() {
                    secret = crate::native_smoke::take_provider_secret();
                }
                let owner = &self.workspaces[i];
                let result = Live::start(
                    &self.registry,
                    runtime_paths,
                    config.model.clone(),
                    config.context_window,
                    provider,
                    secret,
                    &owner.token,
                    &owner.basis,
                    &self.shutdown,
                );
                match result {
                    Ok(live) => {
                        self.failure = None;
                        self.live = Some(live);
                        self.live_index = Some(i);
                        self.stage = "starting";
                    }
                    Err(StartError::Partial(reason, live)) => {
                        self.failure = Some(reason.class());
                        self.live = Some(live);
                        self.live_index = Some(i);
                        self.stage = "startup_failed";
                        return Err(Error::Session);
                    }
                    Err(StartError::Refused(reason)) => {
                        self.failure = Some(reason.class());
                        return Err(Error::Session);
                    }
                }
                Ok(json!({"status":self.stage}))
            }
            Action::Text { authority, text } => {
                let i = self.workspace(webview, &authority)?;
                if self.live_index != Some(i) || !self.live.as_ref().is_some_and(Live::ready) {
                    return Err(Error::Busy);
                }
                self.live
                    .as_mut()
                    .unwrap()
                    .driver()
                    .ok_or(Error::Session)?
                    .submit_text(&text)
                    .map_err(|_| Error::Session)?;
                self.workspaces[i].turn_active = true;
                self.workspaces[i].turn_id = None;
                self.workspaces[i].turn_status = "starting";
                Ok(json!({"status":"queued"}))
            }
            Action::Interrupt { authority } => {
                let i = self.workspace(webview, &authority)?;
                if self.live_index != Some(i) {
                    return Err(Error::Workspace);
                }
                self.live
                    .as_mut()
                    .and_then(Live::driver)
                    .ok_or(Error::Session)?
                    .interrupt()
                    .map_err(|_| Error::Session)?;
                Ok(json!({"status":"queued","turnId":self.workspaces[i].turn_id}))
            }
            Action::Close { authority } => {
                let i = self.workspace(webview, &authority)?;
                self.workspaces[i].closed = true;
                self.workspaces[i].bridge.close();
                if self.live_index == Some(i) {
                    self.close_live();
                }
                Ok(json!({"status":"stopping"}))
            }
            Action::Poll { authority } => {
                let i = self.workspace(webview, &authority)?;
                let owner = &mut self.workspaces[i];
                let messages = std::mem::take(&mut owner.messages);
                owner.message_bytes = 0;
                let rejection = self
                    .live
                    .as_mut()
                    .and_then(Live::driver)
                    .and_then(|driver| driver.last_rejection());
                Ok(
                    json!({"status":self.stage,"notices":owner.notices,"messages":messages,"closed":owner.closed,"rejection":rejection,"failure":self.failure,"turnActive":owner.turn_active,"turnId":owner.turn_id,"turnStatus":owner.turn_status}),
                )
            }
            Action::Claim {
                authority,
                request_ref,
            } => {
                let i = self.workspace(webview, &authority)?;
                let owner = &mut self.workspaces[i];
                let origin = Origin {
                    session: &owner.token,
                    webview: &owner.webview,
                    workspace: &owner.handle,
                    generation: owner.generation,
                };
                let claim = owner
                    .bridge
                    .claim(origin, &request_ref, &owner.basis)
                    .map_err(|_| Error::Evidence)?;
                let data: Value = serde_json::from_str(claim.raw()).map_err(|_| Error::Input)?;
                let tool = Tool::from_name(data["tool"].as_str().ok_or(Error::Input)?)
                    .ok_or(Error::Input)?;
                owner.claims.insert(
                    request_ref.clone(),
                    Meta {
                        tool,
                        basis: data["basis"].clone(),
                        jobs_before: self.registry.last_job_number(),
                        arguments: if tool == Tool::ReadResults { data["arguments"].clone() } else { Value::Null },
                    },
                );
                owner.notices.retain(|r| r != &request_ref);
                Ok(json!({"raw":claim.raw()}))
            }
            Action::PublishActual {
                authority,
                request_ref,
                outcome,
                proof,
            } => {
                let i = self.workspace(webview, &authority)?;
                let owner = &mut self.workspaces[i];
                let meta = owner.claims.get(&request_ref).ok_or(Error::Evidence)?;
                if owner.actual.contains_key(&request_ref) {
                    return Err(Error::Duplicate);
                }
                let bound = validate_actual(&self.registry, meta, &outcome, &proof, &owner.consumed_jobs, &owner.runs)?;
                // A run handle is bound once: a completion naming a handle that any recorded
                // outcome already holds is refused before anything changes.
                if let Proof::Run { handle: Some(handle), .. } = &proof {
                    if meta.tool == Tool::Solve && (owner.runs.contains_key(handle) || owner.history(handle).is_some()) {
                        return Err(Error::Evidence);
                    }
                }
                // A result read's values are checked against the native rows of the run's
                // held result (the same rows query the webview reads), before anything changes.
                let cursor = match &proof {
                    Proof::Run { native_job_id: Some(job), handle: Some(handle), ordinals, .. } if meta.tool == Tool::ReadResults => {
                        let next = verify_result_page(&self.registry, meta, job, handle, &outcome, ordinals.as_deref(), &owner.cursors)?;
                        if next.as_ref().is_some_and(|(cursor, _)| owner.cursors.contains_key(cursor)) || (next.is_some() && owner.cursors.len() >= MAX_CURSORS) {
                            return Err(Error::Evidence);
                        }
                        next
                    }
                    _ => None,
                };
                // The call's whole evidence (its requested mode, the agreement of outcome and
                // proof, the evaluated hash and the outcome's shape) is checked by the
                // dispatcher before anything is committed; a refused completion spends no job,
                // binds no handle and leaves no actual that history can read.
                owner
                    .bridge
                    .check_complete(owner.origin(), &request_ref, outcome.as_bytes(), Some(&owner.basis), evidence(&proof))
                    .map_err(|_| Error::Evidence)?;
                // Native actual, proof and its bounded retained frontend copy.
                let record_backing = self
                    .registry
                    .acquire_interactive_ingress(512 * 1024)
                    .map_err(|_| Error::Admission)?;
                // Committed together, only on success: the solve's named job is spent, a
                // completed run keeps its job's result hash, a verified page's continuation
                // is recorded, and the actual is recorded.
                if let Some((cursor, at)) = cursor {
                    owner.cursors.insert(cursor, at);
                }
                if let Some((job, result_sha256)) = bound {
                    owner.consumed_jobs.insert(job.clone());
                    if let (Some(sha), Proof::Run { handle: Some(handle), .. }) = (result_sha256, &proof) {
                        owner.runs.insert(handle.clone(), (job, sha));
                    }
                }
                owner.actual.insert(
                    request_ref,
                    Actual {
                        raw: outcome,
                        proof,
                        _backing: record_backing,
                    },
                );
                Ok(json!({"recorded":true}))
            }
            Action::Complete {
                authority,
                request_ref,
                outcome,
            } => {
                let i = self.workspace(webview, &authority)?;
                let owner = &mut self.workspaces[i];
                let actual = owner.actual.get(&request_ref).ok_or(Error::Evidence)?;
                if actual.raw != outcome {
                    return Err(Error::Evidence);
                }
                let origin = Origin {
                    session: &owner.token,
                    webview: &owner.webview,
                    workspace: &owner.handle,
                    generation: owner.generation,
                };
                let prepared = owner
                    .bridge
                    .complete(
                        origin,
                        &request_ref,
                        outcome.as_bytes(),
                        Some(&owner.basis),
                        evidence(&actual.proof),
                    )
                    .map_err(|_| Error::Evidence)?;
                let delivery = if self.live_index == Some(i) {
                    if let Some(driver) = self.live.as_mut().and_then(Live::driver) {
                        match prepared.deliver(driver) {
                            super::dispatcher::Delivery::Queued => "queued",
                            super::dispatcher::Delivery::Detached => "detached",
                            super::dispatcher::Delivery::Lost => "lost",
                            _ => "unconfirmed",
                        }
                    } else {
                        "lost"
                    }
                } else {
                    "detached"
                };
                Ok(json!({"delivery":delivery}))
            }
            Action::TakeActual {
                authority,
                request_ref,
            } => {
                let i = self.workspace(webview, &authority)?;
                let owner = &mut self.workspaces[i];
                let actual = owner.actual.get(&request_ref).ok_or(Error::Evidence)?;
                let raw = actual.raw.clone();
                let origin = Origin {
                    session: &owner.token,
                    webview: &owner.webview,
                    workspace: &owner.handle,
                    generation: owner.generation,
                };
                // Producer custody exists even when the complete response was
                // lost before vendor delivery. Taking never replays delivery.
                let _ = owner.bridge.take_actual(origin, &request_ref);
                Ok(json!({"raw":raw}))
            }
        }
    }
}
fn basis_value(handle: &str, generation: u64, basis: BasisDraft) -> Value {
    json!({"workspaceHandle":handle,"applicationGeneration":generation,"modelRevision":basis.model_revision,"modelHash":basis.model_hash,"stateId":basis.state_id,"runId":basis.run_id})
}
fn evidence(proof: &Proof) -> Evidence<'_> {
    match proof {
        Proof::Read => Evidence::Read,
        Proof::Failure => Evidence::Failure,
        Proof::Proposal {
            handle,
            basis,
            status,
            outcome_model_hash,
        } => Evidence::Proposal {
            handle,
            basis,
            status,
            outcome_model_hash: outcome_model_hash.as_ref(),
        },
        Proof::Run {
            handle,
            run_id,
            basis,
            status,
            ..
        } => Evidence::Run {
            handle: handle.as_deref(),
            run_id: run_id.as_deref(),
            basis,
            status,
        },
    }
}
/// The published result's `status.mechanics`, read natively from the bytes the
/// registry holds for the job; every other member is skipped unread. `None` when
/// the job holds no result or the bytes do not state exactly one status.
fn held_mechanics_status(registry: &SolveJobRegistry, job: &str) -> Option<String> {
    #[derive(Deserialize)]
    struct Envelope {
        status: Status,
    }
    #[derive(Deserialize)]
    struct Status {
        mechanics: String,
    }
    let held = registry.held_result(job).ok()?;
    serde_json::from_str::<Envelope>(held.text())
        .ok()
        .map(|envelope| envelope.status.mechanics)
}
/// The native binding of a solve job that a completed or rejected outcome names,
/// established from the registry, not from the webview: the job was made after the
/// call was claimed, no other completion has named it, it ran in the mode the
/// outcome states, and its held result is solved (completed) or not (rejected).
/// Returns the job's result hash. It does not bind the job's model to the call's
/// model: the registry does not hold the catalog's model hash.
fn solve_job_binding(
    registry: &SolveJobRegistry,
    meta: &Meta,
    job: &str,
    status: &str,
    value: &Value,
    consumed: &BTreeSet<String>,
) -> Result<[u8; 32], Error> {
    let number = registry.job_number(job).ok_or(Error::Evidence)?;
    if number <= meta.jobs_before || consumed.contains(job) {
        return Err(Error::Evidence);
    }
    let custody = registry.custody_binding(job).ok_or(Error::Evidence)?;
    let mode = value["data"]["solverMode"].as_str().and_then(open_pipe_stress_product_physics::PreviewSolverMode::from_wire);
    if mode != Some(custody.solver_mode) {
        return Err(Error::Evidence);
    }
    let solved = held_mechanics_status(registry, job).ok_or(Error::Evidence)? == "MECHANICS_SOLVED";
    if solved != (status == "completed") {
        return Err(Error::Evidence);
    }
    Ok(custody.result_sha256)
}
/// A result page's extent: it stops before the row whose native row text would take the
/// page's row texts past this many bytes, always holding at least one row. The webview
/// (`RESULT_PAGE_ROW_TEXT_BYTES` in `embeddedAgentSession.ts`) applies the same rule to the
/// same rows, so the page the host requires is determined by the query alone.
const PAGE_ROW_TEXT_BYTES: u64 = 32 * 1024;
/// How many of a native page's rows a result page holds under [`PAGE_ROW_TEXT_BYTES`].
fn page_extent(rows: &[String]) -> usize {
    let (mut used, mut count) = (0u64, 0usize);
    for row in rows {
        let bytes = row.len() as u64;
        if count > 0 && used + bytes > PAGE_ROW_TEXT_BYTES {
            break;
        }
        used += bytes;
        count += 1;
    }
    count
}
/// The most page continuations a workspace records (the webview keeps as many cursors).
const MAX_CURSORS: usize = 64;
/// The text a withheld value carries: the webview's projection writes exactly this.
const WITHHELD_REASON: &str = "The existing numerical view does not permit this binary64 value.";
/// A row's or a submitted value's `value` member as written: its exact JSON text.
#[derive(Deserialize)]
struct RawMember {
    value: Option<Box<serde_json::value::RawValue>>,
}
#[derive(Deserialize)]
struct RawPageData {
    values: Vec<RawMember>,
}
#[derive(Deserialize)]
struct RawPage {
    data: RawPageData,
}
/// The binary64 a JSON number text denotes, as its bits. Both texts compared are
/// parsed by Rust's correctly rounded `f64` parser from their exact JSON text, never
/// through another float: two texts are equal exactly when they denote the same
/// binary64 (`1.0` and `1`, `1e-7` and `0.0000001`), and `-0` is not `0`. `None` for
/// anything but a finite JSON number.
fn number_bits(text: &str) -> Option<u64> {
    let text = text.trim();
    if !text.starts_with(|c: char| c == '-' || c.is_ascii_digit()) {
        return None;
    }
    text.parse::<f64>().ok().filter(|value| value.is_finite()).map(f64::to_bits)
}
/// The native rows query's page for `job` (`result_detail::run_rows`, the command the
/// webview's read calls, with its filtering, paging and byte bound): the total of
/// admitted rows, the page's row ordinals and each row's exact text.
fn native_rows(registry: &SolveJobRegistry, job: &str, filter: Value, offset: u64, limit: u64) -> Result<(u64, Vec<u64>, Vec<String>), Error> {
    let args = json!({"jobId": job, "filter": filter, "offset": offset, "limit": limit}).to_string().into_bytes();
    let copy = registry
        .acquire_interactive_ingress_capped(crate::ingress::allocation_envelope(args.len() as u64).unwrap_or(u64::MAX))
        .map_err(|_| Error::Admission)?;
    let registry = registry.clone();
    let response = open_pipe_stress_result_export::recorded_tables::entry_scope(move || crate::result_detail::run_rows(registry, args, copy))
        .map_err(|_| Error::Evidence)?;
    // The detail response: magic, the header's length, the header, then the rows' texts.
    if response.len() < 16 || &response[..8] != b"SWBDETv1" {
        return Err(Error::Evidence);
    }
    let length = u64::from_le_bytes(response[8..16].try_into().map_err(|_| Error::Evidence)?) as usize;
    let header: Value = serde_json::from_slice(response.get(16..16 + length).ok_or(Error::Evidence)?).map_err(|_| Error::Evidence)?;
    let numbers = |key: &str| -> Result<Vec<u64>, Error> {
        header[key].as_array().ok_or(Error::Evidence)?.iter().map(|n| n.as_u64().ok_or(Error::Evidence)).collect()
    };
    let (ordinals, lengths) = (numbers("ordinals")?, numbers("lengths")?);
    let mut at = 16 + length;
    let mut rows = Vec::with_capacity(lengths.len());
    for len in &lengths {
        let end = at.checked_add(*len as usize).ok_or(Error::Evidence)?;
        rows.push(std::str::from_utf8(response.get(at..end).ok_or(Error::Evidence)?).map_err(|_| Error::Evidence)?.to_owned());
        at = end;
    }
    if at != response.len() || ordinals.len() != rows.len() {
        return Err(Error::Evidence);
    }
    Ok((header["total"].as_u64().ok_or(Error::Evidence)?, ordinals, rows))
}
/// One submitted value against the native row it names: the same id, kind, entity,
/// unit, case and metadata, and either that row's exact number or null marked
/// withheld. Withholding can only remove a number; it never changes one.
fn value_matches_row(value: &Value, written: Option<&serde_json::value::RawValue>, row_text: &str) -> bool {
    let (Ok(row), Ok(raw)) = (serde_json::from_str::<Value>(row_text), serde_json::from_str::<RawMember>(row_text)) else {
        return false;
    };
    let case = match row.get("basis_ref") {
        Some(basis) if !basis.is_null() => json!({"refType": basis["ref_type"], "refId": basis["ref_id"]}),
        _ => Value::Null,
    };
    let member = |key: &str| match row["metadata"].get(key) {
        Some(Value::String(text)) if !text.is_empty() => Value::String(text.clone()),
        _ => Value::Null,
    };
    let same = value["id"] == row["id"]
        && value["kind"] == row["kind"]
        && value["entityRef"] == row["entity_ref"]
        && value["unit"] == row["unit"]
        && value["case"] == case
        && value["component"] == member("component")
        && value["coordinateSystem"] == member("coordinate_system")
        && value["location"] == member("location")
        && value["signConvention"] == member("sign_convention")
        && value["exactDecimal"].is_null();
    same && match value["availability"].as_str() {
        Some("available") => {
            let submitted = written.and_then(|text| number_bits(text.get()));
            let native = raw.value.as_deref().and_then(|text| number_bits(text.get()));
            value["reason"].is_null() && submitted.is_some() && submitted == native
        }
        Some("withheld") => value["value"].is_null() && value["reason"].as_str() == Some(WITHHELD_REASON),
        _ => false,
    }
}
/// A result read's page, checked against the native rows query of the run's held
/// result, bound to the admitted request: the call's filter; its start (0 for no
/// cursor, else where the page that issued the cursor ended, as this host recorded
/// it); its extent (exactly the query's rows at that start and the call's limit, cut
/// by the page's row-text rule); its continuation (present exactly when rows remain,
/// and recorded here at the page's end); and each value against its row. Returns the
/// continuation to record.
fn verify_result_page(
    registry: &SolveJobRegistry,
    meta: &Meta,
    job: &str,
    handle: &str,
    raw: &str,
    ordinals: Option<&[u64]>,
    cursors: &BTreeMap<String, (String, Value, u64)>,
) -> Result<Option<(String, (String, Value, u64))>, Error> {
    let value: Value = serde_json::from_str(raw).map_err(|_| Error::Input)?;
    let args = &meta.arguments;
    if value["status"].as_str() != Some("ok") || args["runHandle"].as_str() != Some(handle) {
        return Err(Error::Evidence);
    }
    let filter = args.get("filter").cloned().unwrap_or(Value::Null);
    let limit = args["page"]["limit"].as_u64().ok_or(Error::Evidence)?;
    let start = match &args["page"]["cursor"] {
        Value::Null => 0,
        Value::String(cursor) => {
            let (bound, bound_filter, at) = cursors.get(cursor).ok_or(Error::Evidence)?;
            if bound != handle || *bound_filter != filter {
                return Err(Error::Evidence);
            }
            *at
        }
        _ => return Err(Error::Evidence),
    };
    // The call's filter as the native query states it (the webview's agentDetailFilter).
    let native_filter = match &filter {
        Value::Null => json!({}),
        filter => {
            let mut native = json!({"kinds": filter["kinds"], "entities": filter["entityRefs"]});
            if let Some(case) = filter.get("case") {
                let (kind, id) = (case["refType"].as_str().ok_or(Error::Evidence)?, case["refId"].as_str().ok_or(Error::Evidence)?);
                native["case"] = json!(format!("{kind}:{id}"));
            }
            native
        }
    };
    let (total, native, rows) = native_rows(registry, job, native_filter, start, limit)?;
    let values = value["data"]["values"].as_array().ok_or(Error::Evidence)?;
    let written: RawPage = serde_json::from_str(raw).map_err(|_| Error::Input)?;
    let ordinals = ordinals.ok_or(Error::Evidence)?;
    // The extent is the query's: exactly the rows it returns at the call's offset and
    // limit (its own byte bound included), cut by the page's row-text rule; no fewer,
    // no more, none other.
    let k = page_extent(&rows);
    if values.len() != k || ordinals.len() != k || written.data.values.len() != k || ordinals != &native[..k] {
        return Err(Error::Evidence);
    }
    for (at, submitted) in values.iter().enumerate() {
        if !value_matches_row(submitted, written.data.values[at].value.as_deref(), &rows[at]) {
            return Err(Error::Evidence);
        }
    }
    let end = start + k as u64;
    match (end < total, &value["data"]["nextCursor"]) {
        (true, Value::String(cursor)) => Ok(Some((cursor.clone(), (handle.to_owned(), filter, end)))),
        (false, Value::Null) => Ok(None),
        _ => Err(Error::Evidence),
    }
}
/// Validates a webview's actual outcome and its proof. For a solve naming a job it
/// returns that job and, for a completed run, the job's result hash, which the
/// caller records (the job is then spent).
#[allow(clippy::type_complexity)]
fn validate_actual(
    registry: &SolveJobRegistry,
    meta: &Meta,
    raw: &str,
    proof: &Proof,
    consumed: &BTreeSet<String>,
    runs: &BTreeMap<String, (String, [u8; 32])>,
) -> Result<Option<(String, Option<[u8; 32]>)>, Error> {
    if raw.len() > 64 * 1024 {
        return Err(Error::Input);
    }
    let bound = crate::ingress::prescan(raw.as_bytes()).map_err(|_| Error::Input)?;
    if super::framing::reservation_bytes(bound).ok_or(Error::Admission)? > 32 * 1024 * 1024 {
        return Err(Error::Admission);
    }
    numeric::check_admitted_json(raw.as_bytes()).map_err(|_| Error::Input)?;
    let value: Value = serde_json::from_str(raw).map_err(|_| Error::Input)?;
    let catalog = Catalog::load().map_err(|_| Error::Input)?;
    catalog
        .validate_output(meta.tool, &value)
        .map_err(|_| Error::Input)?;
    if value["basis"] != meta.basis {
        return Err(Error::Evidence);
    }
    match proof {
        Proof::Read if matches!(meta.tool, Tool::Summary | Tool::Nodes | Tool::ReadRecords) => {}
        Proof::Failure
            if value["data"].is_null()
                && matches!(
                    value["status"].as_str(),
                    Some("unknown" | "unavailable" | "stale_basis" | "execution_error")
                ) => {}
        Proof::Proposal {
            handle,
            basis,
            status,
            ..
        } if matches!(meta.tool, Tool::ProposeOperations | Tool::ObserveProposal)
            && basis == &meta.basis
            && value["status"].as_str() == Some(status)
            && value["data"]["proposalHandle"].as_str() == Some(handle) => {}
        Proof::Run {
            native_job_id: Some(job),
            status,
            basis,
            handle,
            ..
        } if matches!(meta.tool, Tool::Solve | Tool::ReadResults) && basis == &meta.basis => {
            let state = registry.status(job).map_err(|_| Error::Evidence)?;
            // A solve the solver refused ran to its result as a completed job, as a
            // solved one does; which of the two the result is, is read from the
            // result by the webview (the native job does not state it).
            if (matches!(status.as_str(), "completed" | "rejected") && state.state != "completed")
                || (status == "failed" && state.state != "failed")
                || (status == "cancelled" && state.state != "cancelled")
            {
                return Err(Error::Evidence);
            }
            if meta.tool == Tool::Solve && matches!(status.as_str(), "completed" | "rejected") {
                let sha = solve_job_binding(registry, meta, job, status, &value, consumed)?;
                return Ok(Some((job.clone(), (status == "completed").then_some(sha))));
            }
            if meta.tool == Tool::Solve {
                // A failure that names a job is not a completion of that job; it spends it.
                if consumed.contains(job) {
                    return Err(Error::Evidence);
                }
                return Ok(Some((job.clone(), None)));
            }
            // A result read names the job its run handle was bound to, still holding
            // the same result bytes; never another solve's.
            let handle = handle.as_deref().ok_or(Error::Evidence)?;
            let (bound, sha) = runs.get(handle).ok_or(Error::Evidence)?;
            let custody = registry.custody_binding(job).ok_or(Error::Evidence)?;
            if bound != job || custody.result_sha256 != *sha {
                return Err(Error::Evidence);
            }
        }
        // A refused or failed solve has no run and may have no job (a solve gate
        // refused the model before any job started; a start the memory pool refused;
        // an error before the job existed): it names neither, and it remains bound
        // to the call's basis. A completed claim always names its job (above). The
        // dispatcher checks the rest.
        Proof::Run {
            native_job_id: None,
            handle: None,
            run_id: None,
            status,
            basis,
            ordinals: None,
        } if meta.tool == Tool::Solve
            && matches!(status.as_str(), "rejected" | "failed")
            && basis == &meta.basis
            && value["status"].as_str() == Some(status.as_str()) => {}
        _ => return Err(Error::Evidence),
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::super::protocol::{Protocol, ThreadOptions};
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    fn hash() -> Value {
        json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","payload_ref":"invented-project","value":"a".repeat(64),"hash_status":"computed_local_preview"})
    }
    fn draft() -> BasisDraft {
        BasisDraft {
            model_revision: 2,
            model_hash: hash(),
            state_id: None,
            run_id: None,
        }
    }
    fn paths() -> Paths {
        Paths {
            data: "/tmp/invented-unopened-agent-data".into(),
            resources: "/tmp/invented-unopened-agent-resources".into(),
        }
    }
    fn lease(owner: &Owner) -> Lease {
        owner
            .registry
            .acquire_interactive_ingress(32 * 1024 * 1024)
            .unwrap()
    }
    fn registered() -> Owner {
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let mut owner = Owner::new(registry, Arc::new(AtomicBool::new(false)));
        let backing = lease(&owner);
        owner
            .action(
                "main",
                Action::Register { basis: draft() },
                paths(),
                backing,
            )
            .unwrap();
        owner
    }
    fn auth(owner: &Owner) -> Authority {
        let w = &owner.workspaces[0];
        Authority {
            workspace_handle: w.handle.clone(),
            application_generation: w.generation,
            owner_capability: w.capability.clone(),
        }
    }
    fn offered(owner: &mut Owner) -> String {
        let w = &mut owner.workspaces[0];
        let basis = w.basis.clone();
        let mut p = Protocol::new(Catalog::load().unwrap(), w.token.clone(), 16).unwrap();
        p.initialize("test", "1").unwrap();
        p.handle_frame(json!({"id":1,"result":{"codexHome":"/invented","platformFamily":"unix","platformOs":"macos","userAgent":"fixture"}}),&basis).unwrap();
        p.start_thread(ThreadOptions {
            model: "invented".into(),
            provider: "invented".into(),
            cwd: "/invented".into(),
            base_instructions: "invented".into(),
        })
        .unwrap();
        for event in p.handle_frame(json!({"id":2,"result":{"thread":{"id":"thread-1"},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}}}),&basis).unwrap().events{w.bridge.observe(&event).unwrap();}
        p.submit_text("invented".into()).unwrap();
        for event in p
            .handle_frame(
                json!({"id":3,"result":{"turn":{"id":"turn-1","items":[],"status":"inProgress"}}}),
                &basis,
            )
            .unwrap()
            .events
        {
            w.bridge.observe(&event).unwrap();
        }
        let event=p.handle_frame(json!({"id":90,"method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":"call-1","tool":Tool::Summary.name(),"arguments":{}}}),&basis).unwrap().events.into_iter().find(|event|matches!(event,Event::ToolRequested{..})).unwrap();
        w.bridge
            .offer(&event, &basis, History::Live)
            .unwrap()
            .request_ref
            .into()
    }
    fn outcome(owner: &Owner) -> String {
        json!({"status":"unavailable","basis":owner.workspaces[0].basis,"message":"Invented unavailable data","data":null}).to_string()
    }
    #[test]
    fn raw_command_camel_case_fields_use_only_the_existing_serde_forms() {
        let action:Action=serde_json::from_value(json!({"kind":"publish_actual","authority":{"workspaceHandle":"invented","applicationGeneration":1,"ownerCapability":"private"},"requestRef":"request-1","outcome":"{}","proof":{"kind":"run","handle":"handle-1","runId":"run-1","basis":{},"status":"completed","nativeJobId":"job-1"}})).unwrap();
        assert!(
            matches!(action,Action::PublishActual {request_ref,proof:Proof::Run{native_job_id:Some(job),..},..} if request_ref=="request-1" && job=="job-1")
        );
    }
    #[test]
    fn public_handle_cannot_authenticate_the_execution_owner() {
        let owner = registered();
        let mut wrong = auth(&owner);
        wrong.owner_capability = wrong.workspace_handle.clone();
        assert_eq!(owner.workspace("main", &wrong), Err(Error::Authority));
        assert_eq!(
            owner.workspace("other", &auth(&owner)),
            Err(Error::Authority)
        );
    }
    #[test]
    fn completion_cannot_mint_or_change_an_actual_owner_record() {
        let mut owner = registered();
        let reference = offered(&mut owner);
        let a = auth(&owner);
        let backing = lease(&owner);
        owner
            .action(
                "main",
                Action::Claim {
                    authority: a,
                    request_ref: reference.clone(),
                },
                paths(),
                backing,
            )
            .unwrap();
        let raw = outcome(&owner);
        let a = auth(&owner);
        let backing = lease(&owner);
        assert_eq!(
            owner
                .action(
                    "main",
                    Action::Complete {
                        authority: a,
                        request_ref: reference.clone(),
                        outcome: raw.clone()
                    },
                    paths(),
                    backing
                )
                .unwrap_err(),
            Error::Evidence
        );
        let a = auth(&owner);
        let backing = lease(&owner);
        owner
            .action(
                "main",
                Action::PublishActual {
                    authority: a,
                    request_ref: reference.clone(),
                    outcome: raw.clone(),
                    proof: Proof::Failure,
                },
                paths(),
                backing,
            )
            .unwrap();
        let a = auth(&owner);
        let backing = lease(&owner);
        assert_eq!(
            owner
                .action(
                    "main",
                    Action::Complete {
                        authority: a,
                        request_ref: reference.clone(),
                        outcome: raw.replace("Invented unavailable data", "tampered")
                    },
                    paths(),
                    backing
                )
                .unwrap_err(),
            Error::Evidence
        );
        let a = auth(&owner);
        let backing = lease(&owner);
        let completed = owner
            .action(
                "main",
                Action::Complete {
                    authority: a,
                    request_ref: reference.clone(),
                    outcome: raw.clone(),
                },
                paths(),
                backing,
            )
            .unwrap();
        assert_eq!(completed["delivery"], "detached");
        let a = auth(&owner);
        let backing = lease(&owner);
        let actual = owner
            .action(
                "main",
                Action::TakeActual {
                    authority: a,
                    request_ref: reference,
                },
                paths(),
                backing,
            )
            .unwrap();
        assert_eq!(actual["raw"], raw);
    }
    #[test]
    fn closing_does_not_erase_late_claimed_actuals() {
        let mut owner = registered();
        let reference = offered(&mut owner);
        let a = auth(&owner);
        let backing = lease(&owner);
        owner
            .action(
                "main",
                Action::Claim {
                    authority: a,
                    request_ref: reference.clone(),
                },
                paths(),
                backing,
            )
            .unwrap();
        let a = auth(&owner);
        let backing = lease(&owner);
        owner
            .action("main", Action::Close { authority: a }, paths(), backing)
            .unwrap();
        let raw = outcome(&owner);
        let a = auth(&owner);
        let backing = lease(&owner);
        owner
            .action(
                "main",
                Action::PublishActual {
                    authority: a,
                    request_ref: reference.clone(),
                    outcome: raw.clone(),
                    proof: Proof::Failure,
                },
                paths(),
                backing,
            )
            .unwrap();
        let a = auth(&owner);
        let backing = lease(&owner);
        assert_eq!(
            owner
                .action(
                    "main",
                    Action::TakeActual {
                        authority: a,
                        request_ref: reference
                    },
                    paths(),
                    backing
                )
                .unwrap()["raw"],
            raw
        );
    }
    #[test]
    fn terminal_eof_or_protocol_failure_closes_the_workspace_without_losing_late_actual() {
        for (end, failure) in [
            (End::Eof, None),
            (
                End::Failure(super::super::live_runtime::Error::Protocol),
                Some("protocol"),
            ),
        ] {
            let mut owner = registered();
            let reference = offered(&mut owner);
            let authority = auth(&owner);
            let backing = lease(&owner);
            owner
                .action(
                    "main",
                    Action::Claim {
                        authority,
                        request_ref: reference.clone(),
                    },
                    paths(),
                    backing,
                )
                .unwrap();
            owner.retire_terminal(0, end);
            assert!(owner.closed_clean());
            let authority = auth(&owner);
            let backing = lease(&owner);
            let poll = owner
                .action("main", Action::Poll { authority }, paths(), backing)
                .unwrap();
            assert_eq!(poll["status"], "closed");
            assert_eq!(poll["closed"], true);
            assert_eq!(poll["failure"].as_str(), failure);
            let raw = outcome(&owner);
            let authority = auth(&owner);
            let backing = lease(&owner);
            owner
                .action(
                    "main",
                    Action::PublishActual {
                        authority,
                        request_ref: reference.clone(),
                        outcome: raw.clone(),
                        proof: Proof::Failure,
                    },
                    paths(),
                    backing,
                )
                .unwrap();
            let authority = auth(&owner);
            let backing = lease(&owner);
            assert_eq!(
                owner
                    .action(
                        "main",
                        Action::Complete {
                            authority,
                            request_ref: reference.clone(),
                            outcome: raw.clone()
                        },
                        paths(),
                        backing
                    )
                    .unwrap()["delivery"],
                "detached"
            );
            let authority = auth(&owner);
            let backing = lease(&owner);
            assert_eq!(
                owner
                    .action(
                        "main",
                        Action::TakeActual {
                            authority,
                            request_ref: reference
                        },
                        paths(),
                        backing
                    )
                    .unwrap()["raw"],
                raw
            );
        }
    }
    const SOLVED: &str = r#"{"run_id":"run:preview-linear-static-001","status":{"mechanics":"MECHANICS_SOLVED"},"results":[]}"#;
    const BLOCKED: &str = r#"{"run_id":"run:preview-linear-static-blocked","status":{"mechanics":"MODEL_INCOMPLETE"},"results":[],"diagnostics":[{"code":"INVENTED","severity":"blocking","message":"Invented."}]}"#;
    /// A registry whose solve jobs keep their results in native custody, released by a gate.
    fn native_registry() -> (SolveJobRegistry, Arc<crate::solve_worker::test_support::Gate>) {
        let gate = crate::solve_worker::test_support::Gate::new("agent-host");
        let pool = MemoryPool::with_wait_tick(ScriptedHost::normal(), Duration::from_millis(5));
        pool.set_max_running_workers(16);
        let spawn: crate::solve_jobs::SpawnFn = Arc::new(|job| std::thread::Builder::new().spawn(job).map(drop));
        (SolveJobRegistry::with_hooks(pool, gate.launcher(), spawn), gate)
    }
    /// The `started`-th job: run to completion with `result` held natively.
    fn native_job(registry: &SolveJobRegistry, gate: &crate::solve_worker::test_support::Gate, started: usize, mode: Option<&str>, result: &str) -> String {
        let job = registry.start_with_custody(br#"{"project":{"id":"p"}}"#, mode, crate::solve_jobs::ResultCustody::Native).unwrap().job_id;
        gate.wait_started(started);
        gate.release(result);
        assert_eq!(crate::solve_jobs::test_support::wait_terminal(registry, &job).state, "completed");
        job
    }
    fn registered_with(registry: SolveJobRegistry) -> Owner {
        let mut owner = Owner::new(registry, Arc::new(AtomicBool::new(false)));
        let backing = lease(&owner);
        owner.action("main", Action::Register { basis: draft() }, paths(), backing).unwrap();
        owner
    }
    /// The workspace's supplier protocol with a thread and a turn begun.
    fn opened(owner: &mut Owner) -> Protocol {
        let w = &mut owner.workspaces[0];
        let basis = w.basis.clone();
        let mut p = Protocol::new(Catalog::load().unwrap(), w.token.clone(), 16).unwrap();
        p.initialize("test", "1").unwrap();
        p.handle_frame(json!({"id":1,"result":{"codexHome":"/invented","platformFamily":"unix","platformOs":"macos","userAgent":"fixture"}}),&basis).unwrap();
        p.start_thread(ThreadOptions { model: "invented".into(), provider: "invented".into(), cwd: "/invented".into(), base_instructions: "invented".into() }).unwrap();
        for event in p.handle_frame(json!({"id":2,"result":{"thread":{"id":"thread-1"},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}}}),&basis).unwrap().events { w.bridge.observe(&event).unwrap(); }
        p.submit_text("invented".into()).unwrap();
        for event in p.handle_frame(json!({"id":3,"result":{"turn":{"id":"turn-1","items":[],"status":"inProgress"}}}), &basis).unwrap().events { w.bridge.observe(&event).unwrap(); }
        p
    }
    /// A solve call offered to the bridge and claimed by the webview, as the live route does.
    fn claimed_solve(owner: &mut Owner, p: &mut Protocol, n: u64) -> String {
        claimed_solve_in(owner, p, n, None)
    }
    /// [`claimed_solve`] with a requested solver mode.
    fn claimed_solve_in(owner: &mut Owner, p: &mut Protocol, n: u64, mode: Option<&str>) -> String {
        let basis = owner.workspaces[0].basis.clone();
        let mut args = json!({"workspaceHandle":basis["workspaceHandle"],"basis":basis,"claimedModelHash":basis["modelHash"]});
        if let Some(mode) = mode {
            args["solverMode"] = json!(mode);
        }
        let event = p.handle_frame(json!({"id":90+n,"method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":format!("call-{n}"),"tool":Tool::Solve.name(),"arguments":args}}),&basis).unwrap().events.into_iter().find(|event| matches!(event, Event::ToolRequested { .. })).unwrap();
        let reference: String = owner.workspaces[0].bridge.offer(&event, &basis, History::Live).unwrap().request_ref.into();
        let (authority, backing) = (auth(owner), lease(owner));
        owner.action("main", Action::Claim { authority, request_ref: reference.clone() }, paths(), backing).unwrap();
        reference
    }
    fn solve_outcome(basis: &Value, status: &str, mode: &str) -> String {
        solve_outcome_for(basis, status, mode, "run-1")
    }
    fn solve_outcome_for(basis: &Value, status: &str, mode: &str, handle: &str) -> String {
        let run = status == "completed";
        json!({"status":status,"basis":basis,"message":"Invented outcome","data":{"runHandle":run.then_some(handle),"runId":run.then_some("run:preview-linear-static-001"),
            "evaluatedModelHash":hash(),"health":"invented","solverMode":mode,"diagnostics":[],"diagnosticsNotListed":0,"failure":null}}).to_string()
    }
    fn run_proof(basis: &Value, status: &str, job: &str) -> Proof {
        run_proof_for(basis, status, job, "run-1")
    }
    fn run_proof_for(basis: &Value, status: &str, job: &str, handle: &str) -> Proof {
        let run = status == "completed";
        Proof::Run { handle: run.then(|| handle.to_owned()), run_id: run.then(|| "run:preview-linear-static-001".to_owned()), basis: basis.clone(), status: status.into(), native_job_id: Some(job.to_owned()), ordinals: None }
    }
    fn publish(owner: &mut Owner, reference: &str, outcome: String, proof: Proof) -> Result<Value, Error> {
        let (authority, backing) = (auth(owner), lease(owner));
        owner.action("main", Action::PublishActual { authority, request_ref: reference.into(), outcome, proof }, paths(), backing)
    }
    #[test]
    fn a_solve_completion_names_a_fresh_unused_job_in_its_mode_whose_held_result_is_solved() {
        let (registry, gate) = native_registry();
        // Retained and completed, but made before the call was claimed.
        let older = native_job(&registry, &gate, 1, None, SOLVED);
        let mut owner = registered_with(registry.clone());
        let mut p = opened(&mut owner);
        let basis = owner.workspaces[0].basis.clone();
        let a = claimed_solve(&mut owner, &mut p, 1);
        let dense = native_job(&registry, &gate, 2, Some("dense_scrutiny"), SOLVED);
        let blocked = native_job(&registry, &gate, 3, None, BLOCKED);
        let fresh = native_job(&registry, &gate, 4, None, SOLVED);
        let sparse = "sparse_interactive";
        // The reviewer's sequence: another retained completed job, named by a forged completion.
        assert_eq!(publish(&mut owner, &a, solve_outcome(&basis, "completed", sparse), run_proof(&basis, "completed", &older)).unwrap_err(), Error::Evidence);
        // A job that ran in another mode than the outcome states.
        assert_eq!(publish(&mut owner, &a, solve_outcome(&basis, "completed", sparse), run_proof(&basis, "completed", &dense)).unwrap_err(), Error::Evidence);
        // A blocked result named completed, its findings removed: refused from the held bytes.
        assert_eq!(publish(&mut owner, &a, solve_outcome(&basis, "completed", sparse), run_proof(&basis, "completed", &blocked)).unwrap_err(), Error::Evidence);
        // A solved result named rejected.
        assert_eq!(publish(&mut owner, &a, solve_outcome(&basis, "rejected", sparse), run_proof(&basis, "rejected", &fresh)).unwrap_err(), Error::Evidence);
        // An unknown job.
        assert_eq!(publish(&mut owner, &a, solve_outcome(&basis, "completed", sparse), run_proof(&basis, "completed", "backend-solve-job-999")).unwrap_err(), Error::Evidence);
        assert!(owner.workspaces[0].consumed_jobs.is_empty() && owner.workspaces[0].actual.is_empty());
        publish(&mut owner, &a, solve_outcome(&basis, "completed", sparse), run_proof(&basis, "completed", &fresh)).unwrap();
        let held = registry.custody_binding(&fresh).unwrap().result_sha256;
        assert_eq!(owner.workspaces[0].runs.get("run-1"), Some(&(fresh.clone(), held)));
        // The same job named by a second call: used once only.
        let b = claimed_solve(&mut owner, &mut p, 2);
        assert_eq!(publish(&mut owner, &b, solve_outcome(&basis, "completed", sparse), run_proof(&basis, "completed", &fresh)).unwrap_err(), Error::Evidence);
        // The dense job is older than the second call too; a blocked job named rejected is admitted.
        assert_eq!(publish(&mut owner, &b, solve_outcome(&basis, "completed", "dense_scrutiny"), run_proof(&basis, "completed", &dense)).unwrap_err(), Error::Evidence);
        let c = claimed_solve(&mut owner, &mut p, 3);
        // The pool retains four results: release one before the next job.
        registry.dispose_result(&older).unwrap();
        let later_blocked = native_job(&registry, &gate, 5, None, BLOCKED);
        publish(&mut owner, &c, solve_outcome(&basis, "rejected", sparse), run_proof(&basis, "rejected", &later_blocked)).unwrap();
        // Two calls claimed before one job is made: the job is fresh for both, and the
        // second completion naming it is refused because the first spent it.
        let d = claimed_solve(&mut owner, &mut p, 4);
        let e = claimed_solve(&mut owner, &mut p, 5);
        registry.dispose_result(&dense).unwrap();
        let shared = native_job(&registry, &gate, 6, None, BLOCKED);
        publish(&mut owner, &d, solve_outcome(&basis, "rejected", sparse), run_proof(&basis, "rejected", &shared)).unwrap();
        assert_eq!(publish(&mut owner, &e, solve_outcome(&basis, "rejected", sparse), run_proof(&basis, "rejected", &shared)).unwrap_err(), Error::Evidence);
        drop(gate);
    }
    /// A result read through `handle`, admitted from the recorded history as the live
    /// route admits it, claimed by the webview.
    fn claimed_read(owner: &mut Owner, p: &mut Protocol, n: u64, handle: &str) -> String {
        claimed_read_with(owner, p, n, handle, None, None, 5)
    }
    /// [`claimed_read`] with a filter, a cursor and a page limit.
    fn claimed_read_with(owner: &mut Owner, p: &mut Protocol, n: u64, handle: &str, filter: Option<Value>, cursor: Option<&str>, limit: u64) -> String {
        let live = owner.workspaces[0].basis.clone();
        let mut args = json!({"workspaceHandle":live["workspaceHandle"],"basis":live,"runHandle":handle,"page":{"cursor":cursor,"limit":limit}});
        if let Some(filter) = filter {
            args["filter"] = filter;
        }
        let event = p.handle_frame(json!({"id":90+n,"method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":format!("call-{n}"),"tool":Tool::ReadResults.name(),"arguments":args}}),&live).unwrap().events.into_iter().find(|event| matches!(event, Event::ToolRequested { .. })).unwrap();
        let w = &mut owner.workspaces[0];
        let (history_handle, history_basis) = match w.history(handle).expect("a recorded run") {
            History::Run { handle, basis } => (handle.to_owned(), basis.clone()),
            _ => panic!("not a run"),
        };
        let reference: String = w.bridge.offer(&event, &live, History::Run { handle: &history_handle, basis: &history_basis }).unwrap().request_ref.into();
        let (authority, backing) = (auth(owner), lease(owner));
        owner.action("main", Action::Claim { authority, request_ref: reference.clone() }, paths(), backing).unwrap();
        reference
    }
    fn read_proof(basis: &Value, job: &str, handle: &str, ordinals: &[u64]) -> Proof {
        Proof::Run { handle: Some(handle.into()), run_id: Some("run:preview-linear-static-001".into()), basis: basis.clone(), status: "completed".into(), native_job_id: Some(job.into()), ordinals: Some(ordinals.to_vec()) }
    }
    fn read_outcome(basis: &Value, handle: &str) -> String {
        json!({"status":"ok","basis":basis,"message":"Invented page","data":{"runHandle":handle,"runId":"run:preview-linear-static-001","evaluatedModelHash":hash(),"values":[],"nextCursor":null,"standing":{"route":"invented","health":"invented","checks":[]}}}).to_string()
    }
    /// A solved result with five rows: three of load case S, one of combination S, one
    /// of load case S in another unit; numbers written with many digits, a negative zero,
    /// an exponent and none.
    const ROWS: &str = r#"{"run_id":"run:preview-linear-static-001","status":{"mechanics":"MECHANICS_SOLVED"},"results":[
        {"id":"r0","kind":"k","entity_ref":"node:a","unit":"mm","value":4.771112073942932,"basis_ref":{"ref_type":"load_case","ref_id":"S"},"metadata":{"component":"ux","coordinate_system":"global","location":"node","basis":"b","sign_convention":"positive_global"}},
        {"id":"r1","kind":"k","entity_ref":"node:b","unit":"mm","value":-0.0,"basis_ref":{"ref_type":"combination","ref_id":"S"}},
        {"id":"r2","kind":"k","entity_ref":"node:c","unit":"mm","value":1e-7,"basis_ref":{"ref_type":"load_case","ref_id":"S"}},
        {"id":"r3","kind":"k","entity_ref":"node:d","unit":"mm","value":null,"basis_ref":{"ref_type":"load_case","ref_id":"S"}},
        {"id":"r4","kind":"k","entity_ref":"node:e","unit":"kN","value":0.1,"basis_ref":{"ref_type":"load_case","ref_id":"S"}}]}"#;
    /// A value as the webview writes it; `number` is the exact JSON text of its value.
    fn page_value(id: &str, entity: &str, unit: &str, number: &str, case: (&str, &str), metadata: bool) -> String {
        let (availability, reason) = if number == "null" { ("withheld", json!(WITHHELD_REASON)) } else { ("available", Value::Null) };
        let (component, system, location, sign) = if metadata { (json!("ux"), json!("global"), json!("node"), json!("positive_global")) } else { (Value::Null, Value::Null, Value::Null, Value::Null) };
        format!(r#"{{"id":"{id}","kind":"k","entityRef":"{entity}","value":{number},"exactDecimal":null,"unit":"{unit}","availability":"{availability}","reason":{reason},"case":{{"refType":"{}","refId":"{}"}},"component":{component},"coordinateSystem":{system},"location":{location},"signConvention":{sign}}}"#, case.0, case.1)
    }
    fn page_outcome(basis: &Value, values: &[String], next: Option<&str>, run_id: &str) -> String {
        format!(r#"{{"status":"ok","basis":{},"message":"Invented page","data":{{"runHandle":"H","runId":"{run_id}","evaluatedModelHash":{},"values":[{}],"nextCursor":{},"standing":{{"route":"invented","health":"invented","checks":[]}}}}}}"#,
            basis, hash(), values.join(","), next.map_or("null".to_owned(), |c| format!(r#""{c}""#)))
    }
    #[test]
    fn a_result_page_is_the_native_rows_query_page_value_by_value() {
        let (registry, gate) = native_registry();
        let mut owner = registered_with(registry.clone());
        let mut p = opened(&mut owner);
        let basis = owner.workspaces[0].basis.clone();
        let solve = claimed_solve(&mut owner, &mut p, 1);
        let job = native_job(&registry, &gate, 1, None, ROWS);
        publish(&mut owner, &solve, solve_outcome_for(&basis, "completed", "sparse_interactive", "H"), run_proof_for(&basis, "completed", &job, "H")).unwrap();
        let ls = ("load_case", "S");
        let r0 = page_value("r0", "node:a", "mm", "4.771112073942932", ls, true);
        let r2 = page_value("r2", "node:c", "mm", "0.0000001", ls, false); // 1e-7 written another way: the same binary64
        let r3 = page_value("r3", "node:d", "mm", "null", ls, false);
        let r4 = page_value("r4", "node:e", "kN", "0.1", ls, false);
        let filter = json!({"kinds":[],"entityRefs":[],"case":{"refType":"load_case","refId":"S"}});
        let page = |values: &[&String], next: Option<&str>| page_outcome(&basis, &values.iter().map(|v| (*v).clone()).collect::<Vec<_>>(), next, "run:preview-linear-static-001");
        // Page 1 of load case S, two rows: the native page is rows 0 and 2 of 4 admitted.
        let first = claimed_read_with(&mut owner, &mut p, 2, "H", Some(filter.clone()), None, 2);
        let refused = |owner: &mut Owner, outcome: String, ordinals: &[u64]| publish(owner, &first, outcome, read_proof(&basis, &job, "H", ordinals)).unwrap_err();
        let changed = page_value("r0", "node:a", "mm", "4.7711120739429", ls, true);
        assert_eq!(refused(&mut owner, page(&[&changed, &r2], Some("C1")), &[0, 2]), Error::Evidence, "a changed value");
        assert_eq!(refused(&mut owner, page(&[&r2, &r0], Some("C1")), &[0, 2]), Error::Evidence, "values moved to another row's ordinal");
        assert_eq!(refused(&mut owner, page(&[&r0, &r2], Some("C1")), &[0, 1]), Error::Evidence, "an ordinal outside the admitted filter");
        assert_eq!(refused(&mut owner, page(&[&r0, &r3], Some("C1")), &[0, 3]), Error::Evidence, "a gap in the run");
        assert_eq!(refused(&mut owner, page(&[&r0, &r0], Some("C1")), &[0, 0]), Error::Evidence, "a duplicated ordinal");
        let metres = page_value("r0", "node:a", "m", "4.771112073942932", ls, true);
        assert_eq!(refused(&mut owner, page(&[&metres, &r2], Some("C1")), &[0, 2]), Error::Evidence, "a wrong unit");
        assert_eq!(refused(&mut owner, page(&[&r0, &r2], None), &[0, 2]), Error::Evidence, "a last page claimed early");
        assert_eq!(refused(&mut owner, page(&[&r3, &r4], Some("C1")), &[3, 4]), Error::Evidence, "a true page at the wrong offset");
        assert_eq!(refused(&mut owner, page(&[&r0], Some("C1")), &[0]), Error::Evidence, "a page shorter than the query's");
        assert_eq!(refused(&mut owner, page(&[&r0, &r2, &r3], Some("C1")), &[0, 2, 3]), Error::Evidence, "a page longer than the query's");
        let decimal = r0.replace(r#""exactDecimal":null"#, r#""exactDecimal":"4.77""#);
        assert_eq!(refused(&mut owner, page(&[&decimal, &r2], Some("C1")), &[0, 2]), Error::Evidence, "an exact decimal the row does not give");
        // The page passes the host's check but the call refuses it (another run id): nothing is recorded.
        assert_eq!(publish(&mut owner, &first, page_outcome(&basis, &[r0.clone(), r2.clone()], Some("C1"), "run:other"), read_proof(&basis, &job, "H", &[0, 2])).unwrap_err(), Error::Evidence);
        assert!(owner.workspaces[0].cursors.is_empty() && !owner.workspaces[0].actual.contains_key(&first));
        publish(&mut owner, &first, page(&[&r0, &r2], Some("C1")), read_proof(&basis, &job, "H", &[0, 2])).unwrap();
        assert_eq!(owner.workspaces[0].cursors.get("C1"), Some(&("H".to_owned(), filter.clone(), 2)));
        // Page 2 starts where page 1 ended: rows 3 and 4; row 3 holds no number.
        let second = claimed_read_with(&mut owner, &mut p, 3, "H", Some(filter.clone()), Some("C1"), 2);
        assert_eq!(publish(&mut owner, &second, page(&[&r4], None), read_proof(&basis, &job, "H", &[4])).unwrap_err(), Error::Evidence, "a continuation that skips a row");
        let none_held = page_value("r3", "node:d", "mm", "0", ls, false);
        assert_eq!(publish(&mut owner, &second, page(&[&none_held, &r4], None), read_proof(&basis, &job, "H", &[3, 4])).unwrap_err(), Error::Evidence, "a number where the row holds none");
        let forged_reason = r3.replace(WITHHELD_REASON, "The value is 12.5 mm.");
        assert_eq!(publish(&mut owner, &second, page(&[&forged_reason, &r4], None), read_proof(&basis, &job, "H", &[3, 4])).unwrap_err(), Error::Evidence, "a withheld value with another reason");
        // A longer spelling of the same binary64 is refused at admission already: the
        // numeric guard admits no token whose decimal value materialization would change.
        let spelled = page_value("r4", "node:e", "kN", "0.1000000000000000055511151231257827", ls, false);
        assert_eq!(publish(&mut owner, &second, page(&[&r3, &spelled], None), read_proof(&basis, &job, "H", &[3, 4])).unwrap_err(), Error::Input);
        publish(&mut owner, &second, page(&[&r3, &r4], None), read_proof(&basis, &job, "H", &[3, 4])).unwrap();
        // Without a filter: a wrong case, and a negative zero, compared as binary64.
        let all = claimed_read_with(&mut owner, &mut p, 4, "H", None, None, 5);
        let r0_withheld = page_value("r0", "node:a", "mm", "null", ls, true);
        let r1_case = page_value("r1", "node:b", "mm", "-0", ("load_case", "S"), false);
        let r1_zero = page_value("r1", "node:b", "mm", "0", ("combination", "S"), false);
        let r1 = page_value("r1", "node:b", "mm", "-0", ("combination", "S"), false);
        let unfiltered = |values: &[&String]| page_outcome(&basis, &values.iter().map(|v| (*v).clone()).collect::<Vec<_>>(), None, "run:preview-linear-static-001");
        let every = [0, 1, 2, 3, 4];
        assert_eq!(publish(&mut owner, &all, unfiltered(&[&r0_withheld, &r1_case, &r2, &r3, &r4]), read_proof(&basis, &job, "H", &every)).unwrap_err(), Error::Evidence, "a wrong case");
        assert_eq!(publish(&mut owner, &all, unfiltered(&[&r0_withheld, &r1_zero, &r2, &r3, &r4]), read_proof(&basis, &job, "H", &every)).unwrap_err(), Error::Evidence, "zero for a negative zero");
        // A withheld value for a row with a number is accepted (it removes a number).
        publish(&mut owner, &all, unfiltered(&[&r0_withheld, &r1, &r2, &r3, &r4]), read_proof(&basis, &job, "H", &every)).unwrap();
        // Cursors: one the host never recorded, and a recorded one under another filter.
        let forged = claimed_read_with(&mut owner, &mut p, 5, "H", Some(filter.clone()), Some("C9"), 2);
        assert_eq!(publish(&mut owner, &forged, page(&[&r3, &r4], None), read_proof(&basis, &job, "H", &[3, 4])).unwrap_err(), Error::Evidence, "an unrecorded cursor");
        let other_filter = claimed_read_with(&mut owner, &mut p, 6, "H", None, Some("C1"), 2);
        assert_eq!(publish(&mut owner, &other_filter, page(&[&r3, &r4], None), read_proof(&basis, &job, "H", &[3, 4])).unwrap_err(), Error::Evidence, "a cursor under another filter");
        // The query's own extent: rows of about 12 KiB each, limit 4; the page holds the
        // first two (24 KiB; a third would pass 32 KiB of row text), no more and no fewer.
        let pad = "p".repeat(12 * 1024);
        let big = format!(r#"{{"run_id":"run:preview-linear-static-001","status":{{"mechanics":"MECHANICS_SOLVED"}},"results":[{}]}}"#,
            (0..4).map(|i| format!(r#"{{"id":"b{i}","kind":"k","entity_ref":"node:{i}","unit":"mm","value":{i}.5,"note":"{pad}"}}"#)).collect::<Vec<_>>().join(","));
        let big_solve = claimed_solve(&mut owner, &mut p, 8);
        let big_job = native_job(&registry, &gate, 2, None, &big);
        publish(&mut owner, &big_solve, solve_outcome_for(&basis, "completed", "sparse_interactive", "HB"), run_proof_for(&basis, "completed", &big_job, "HB")).unwrap();
        let big_value = |i: usize| format!(r#"{{"id":"b{i}","kind":"k","entityRef":"node:{i}","value":{i}.5,"exactDecimal":null,"unit":"mm","availability":"available","reason":null,"case":null,"component":null,"coordinateSystem":null,"location":null,"signConvention":null}}"#);
        let big_page = |n: usize, next: Option<&str>| page_outcome(&basis, &(0..n).map(big_value).collect::<Vec<_>>(), next, "run:preview-linear-static-001").replace(r#""runHandle":"H""#, r#""runHandle":"HB""#);
        let truncated = claimed_read_with(&mut owner, &mut p, 9, "HB", None, None, 4);
        assert_eq!(publish(&mut owner, &truncated, big_page(3, Some("CB")), read_proof(&basis, &big_job, "HB", &[0, 1, 2])).unwrap_err(), Error::Evidence, "a page past the row-text bound");
        assert_eq!(publish(&mut owner, &truncated, big_page(1, Some("CB")), read_proof(&basis, &big_job, "HB", &[0])).unwrap_err(), Error::Evidence, "a page short of the row-text bound");
        publish(&mut owner, &truncated, big_page(2, Some("CB")), read_proof(&basis, &big_job, "HB", &[0, 1])).unwrap();
        assert_eq!(owner.workspaces[0].cursors.get("CB"), Some(&("HB".to_owned(), Value::Null, 2)));
        // Once the held bytes are gone, a read through the handle is still refused.
        let gone = claimed_read_with(&mut owner, &mut p, 7, "H", Some(filter.clone()), None, 2);
        registry.dispose_result(&job).unwrap();
        assert_eq!(publish(&mut owner, &gone, page(&[&r0, &r2], Some("C3")), read_proof(&basis, &job, "H", &[0, 2])).unwrap_err(), Error::Evidence, "held bytes gone");
        assert!(!owner.workspaces[0].actual.contains_key(&gone) && !owner.workspaces[0].cursors.contains_key("C3"));
        drop(gate);
    }
    #[test]
    fn a_run_handle_is_bound_once_and_a_later_completion_cannot_rebind_it() {
        let (registry, gate) = native_registry();
        let mut owner = registered_with(registry.clone());
        let mut p = opened(&mut owner);
        let basis = owner.workspaces[0].basis.clone();
        // A completes in sparse mode and is given handle H.
        let a = claimed_solve(&mut owner, &mut p, 1);
        let job_a = native_job(&registry, &gate, 1, None, SOLVED);
        publish(&mut owner, &a, solve_outcome_for(&basis, "completed", "sparse_interactive", "H"), run_proof_for(&basis, "completed", &job_a, "H")).unwrap();
        let bound = owner.workspaces[0].runs.get("H").cloned();
        // B completes in dense mode with a fresh, unused, solved job, but submits H again.
        let b = claimed_solve_in(&mut owner, &mut p, 2, Some("dense_scrutiny"));
        let job_b = native_job(&registry, &gate, 2, Some("dense_scrutiny"), SOLVED);
        assert_eq!(publish(&mut owner, &b, solve_outcome_for(&basis, "completed", "dense_scrutiny", "H"), run_proof_for(&basis, "completed", &job_b, "H")).unwrap_err(), Error::Evidence);
        // Nothing changed: H keeps A's job, digest and basis; B's job is unspent; B has no actual.
        assert_eq!(owner.workspaces[0].runs.get("H").cloned(), bound);
        assert!(!owner.workspaces[0].consumed_jobs.contains(&job_b) && !owner.workspaces[0].actual.contains_key(&b));
        // A read through H, admitted from history, returns A's rows and never B's.
        let read = claimed_read(&mut owner, &mut p, 3, "H");
        assert_eq!(publish(&mut owner, &read, read_outcome(&basis, "H"), read_proof(&basis, &job_b, "H", &[])).unwrap_err(), Error::Evidence);
        publish(&mut owner, &read, read_outcome(&basis, "H"), read_proof(&basis, &job_a, "H", &[])).unwrap();
        // B may still complete under its own handle.
        publish(&mut owner, &b, solve_outcome_for(&basis, "completed", "dense_scrutiny", "H2"), run_proof_for(&basis, "completed", &job_b, "H2")).unwrap();
        drop(gate);
    }
    #[test]
    fn a_completion_the_call_refuses_spends_no_job_binds_no_handle_and_leaves_no_history() {
        let (registry, gate) = native_registry();
        let mut owner = registered_with(registry.clone());
        let mut p = opened(&mut owner);
        let basis = owner.workspaces[0].basis.clone();
        // The reviewer's sequence: A requests dense, B requests sparse, then one sparse job J.
        let a = claimed_solve_in(&mut owner, &mut p, 1, Some("dense_scrutiny"));
        let b = claimed_solve_in(&mut owner, &mut p, 2, Some("sparse_interactive"));
        let j = native_job(&registry, &gate, 1, Some("sparse_interactive"), SOLVED);
        // A's outcome names sparse and J: the native facts hold, but the call asked for dense.
        assert_eq!(publish(&mut owner, &a, solve_outcome_for(&basis, "completed", "sparse_interactive", "HA"), run_proof_for(&basis, "completed", &j, "HA")).unwrap_err(), Error::Evidence);
        let w = &owner.workspaces[0];
        assert!(!w.consumed_jobs.contains(&j) && !w.runs.contains_key("HA") && !w.actual.contains_key(&a) && w.history("HA").is_none());
        // B can use J.
        publish(&mut owner, &b, solve_outcome_for(&basis, "completed", "sparse_interactive", "HB"), run_proof_for(&basis, "completed", &j, "HB")).unwrap();
        assert!(owner.workspaces[0].consumed_jobs.contains(&j) && owner.workspaces[0].history("HB").is_some());
        drop(gate);
    }
    #[test]
    fn the_status_reader_reads_only_the_root_status_member_once() {
        let (registry, gate) = native_registry();
        let large = format!(r#"{{"results":[{}],"status":{{"mechanics":"MECHANICS_SOLVED"}}}}"#, vec![r#"{"id":"r","value":1.5,"note":"x"}"#; 60_000].join(","));
        let cases: [(&str, String, Option<&str>); 9] = [
            ("solved", SOLVED.into(), Some("MECHANICS_SOLVED")),
            ("blocked", BLOCKED.into(), Some("MODEL_INCOMPLETE")),
            ("partial", r#"{"status":{"mechanics":"MECHANICS_PARTIAL"}}"#.into(), Some("MECHANICS_PARTIAL")),
            ("status twice", r#"{"status":{"mechanics":"MODEL_INCOMPLETE"},"status":{"mechanics":"MECHANICS_SOLVED"}}"#.into(), None),
            ("mechanics twice", r#"{"status":{"mechanics":"MODEL_INCOMPLETE","mechanics":"MECHANICS_SOLVED"}}"#.into(), None),
            ("nested deeper", r#"{"summary":{"status":{"mechanics":"MECHANICS_SOLVED"}}}"#.into(), None),
            ("missing", r#"{"run_id":"run:preview-linear-static-001","results":[]}"#.into(), None),
            ("not a string", r#"{"status":{"mechanics":true}}"#.into(), None),
            ("large member first", large, Some("MECHANICS_SOLVED")),
        ];
        for (n, (name, text, expected)) in cases.into_iter().enumerate() {
            let job = native_job(&registry, &gate, n + 1, None, &text);
            assert_eq!(held_mechanics_status(&registry, &job).as_deref(), expected, "{name}");
            registry.dispose_result(&job).unwrap();
        }
        drop(gate);
    }
    #[test]
    fn a_result_read_names_its_run_handles_job_still_holding_the_same_result() {
        let (registry, gate) = native_registry();
        let first = native_job(&registry, &gate, 1, None, SOLVED);
        let second = native_job(&registry, &gate, 2, None, SOLVED);
        let basis = json!({"workspaceHandle":"workspace-invented","applicationGeneration":1,"modelRevision":2,"modelHash":hash(),"stateId":null,"runId":null});
        let meta = Meta { tool: Tool::ReadResults, basis: basis.clone(), jobs_before: 0, arguments: Value::Null };
        let consumed = BTreeSet::new();
        let runs = BTreeMap::from([("run-1".to_owned(), (first.clone(), registry.custody_binding(&first).unwrap().result_sha256))]);
        let page = json!({"status":"ok","basis":basis,"message":"Invented page","data":{"runHandle":"run-1","runId":"run:preview-linear-static-001","evaluatedModelHash":hash(),"values":[],"nextCursor":null,"standing":{"route":"invented","health":"invented","checks":[]}}}).to_string();
        let read = |job: &str, handle: &str| validate_actual(&registry, &meta, &page, &Proof::Run { handle: Some(handle.into()), run_id: Some("run:preview-linear-static-001".into()), basis: basis.clone(), status: "completed".into(), native_job_id: Some(job.into()), ordinals: None }, &consumed, &runs);
        // Both jobs carry the producer's one run id; only the handle's own job is read.
        assert_eq!(read(&second, "run-1").unwrap_err(), Error::Evidence);
        assert_eq!(read(&first, "run-2").unwrap_err(), Error::Evidence);
        assert_eq!(read(&first, "run-1").unwrap(), None);
        // A handle bound to other bytes than the job now holds reads nothing.
        let other = BTreeMap::from([("run-1".to_owned(), (first.clone(), [7u8; 32]))]);
        assert_eq!(validate_actual(&registry, &meta, &page, &Proof::Run { handle: Some("run-1".into()), run_id: None, basis: basis.clone(), status: "completed".into(), native_job_id: Some(first.clone()), ordinals: None }, &consumed, &other).unwrap_err(), Error::Evidence);
        // Once that result is no longer held, the handle reads nothing.
        registry.dispose_result(&first).unwrap();
        assert_eq!(read(&first, "run-1").unwrap_err(), Error::Evidence);
        drop(gate);
    }
    #[test]
    fn a_refused_solve_needs_its_completed_job_and_a_failed_one_names_no_run() {
        use crate::solve_jobs::test_support::wait_terminal;
        let (registry, gate) = native_registry();
        let completed = native_job(&registry, &gate, 1, None, BLOCKED);
        let failed = registry.start_with_custody(br#"{"project":{"id":"q"}}"#, None, crate::solve_jobs::ResultCustody::Native).unwrap().job_id;
        gate.wait_started(2);
        gate.release("err:invented worker failure");
        assert_eq!(wait_terminal(&registry, &failed).state, "failed");
        let basis = json!({"workspaceHandle":"workspace-invented","applicationGeneration":1,"modelRevision":2,"modelHash":hash(),"stateId":null,"runId":null});
        let meta = Meta { tool: Tool::Solve, basis: basis.clone(), jobs_before: 0, arguments: Value::Null };
        let (consumed, runs) = (BTreeSet::new(), BTreeMap::new());
        let raw = |status: &str, run: bool, failure: Value| {
            json!({"status":status,"basis":basis,"message":"Invented outcome","data":{"runHandle":run.then_some("run-1"),"runId":run.then_some("native-run-1"),
                "evaluatedModelHash":hash(),"health":"MODEL_INCOMPLETE","solverMode":"sparse_interactive","diagnostics":[],"diagnosticsNotListed":0,"failure":failure}}).to_string()
        };
        let proof = |status: &str, run: bool, job: Option<&str>| Proof::Run {
            handle: run.then(|| "run-1".to_owned()),
            run_id: run.then(|| "native-run-1".to_owned()),
            basis: basis.clone(),
            status: status.into(),
            native_job_id: job.map(str::to_owned),
            ordinals: None,
        };
        let failure = json!({"kind":"job_failed","message":"Invented job failure."});
        let check = |raw: String, proof: Proof| validate_actual(&registry, &meta, &raw, &proof, &consumed, &runs).map(drop);
        // A refusal that names a job names one that ran to completion, never an unknown or
        // failed job; a refusal before any job (a solve gate) names none.
        assert!(check(raw("rejected", false, Value::Null), proof("rejected", false, Some(&completed))).is_ok());
        assert_eq!(check(raw("rejected", false, Value::Null), proof("rejected", false, Some(&failed))), Err(Error::Evidence));
        assert_eq!(check(raw("rejected", false, Value::Null), proof("rejected", false, Some("invented-job"))), Err(Error::Evidence));
        assert!(check(raw("rejected", false, Value::Null), proof("rejected", false, None)).is_ok());
        assert_eq!(check(raw("rejected", true, Value::Null), proof("rejected", true, None)), Err(Error::Evidence));
        assert_eq!(check(raw("failed", false, Value::Null), proof("rejected", false, None)), Err(Error::Evidence));
        // A failure with no job names no run; one with a job must be that failed job.
        assert!(check(raw("failed", false, failure.clone()), proof("failed", false, None)).is_ok());
        assert!(check(raw("failed", false, failure.clone()), proof("failed", false, Some(&failed))).is_ok());
        assert_eq!(check(raw("failed", false, failure.clone()), proof("failed", false, Some(&completed))), Err(Error::Evidence));
        assert_eq!(check(raw("failed", true, failure.clone()), proof("failed", true, None)), Err(Error::Evidence));
        // A completed claim still requires its completed job.
        assert_eq!(check(raw("completed", true, Value::Null), proof("completed", true, None)), Err(Error::Evidence));
        assert_eq!(check(raw("completed", true, Value::Null), proof("completed", true, Some(&failed))), Err(Error::Evidence));
        // Bound to the call's basis.
        let mut other = basis.clone();
        other["modelRevision"] = json!(3);
        let elsewhere = Proof::Run { handle: None, run_id: None, basis: other, status: "failed".into(), native_job_id: None, ordinals: None };
        assert_eq!(check(raw("failed", false, failure), elsewhere), Err(Error::Evidence));
        drop(gate);
    }
    #[test]
    fn empty_deltas_still_hit_the_message_count_bound() {
        let mut owner = registered();
        let workspace = &mut owner.workspaces[0];
        for _ in 0..128 {
            assert!(workspace.push_message("invented-item", "", false));
        }
        assert!(!workspace.push_message("invented-item", "", false));
        assert_eq!(workspace.messages.len(), 128);
        assert_eq!(workspace.message_bytes, 0);
    }
    #[test]
    fn unclaimed_closed_registrations_do_not_exhaust_key_reentry_retries() {
        let mut owner = registered();
        for _ in 0..8 {
            let authority = auth(&owner);
            let backing = lease(&owner);
            owner
                .action("main", Action::Close { authority }, paths(), backing)
                .unwrap();
            let backing = lease(&owner);
            owner
                .action(
                    "main",
                    Action::Register { basis: draft() },
                    paths(),
                    backing,
                )
                .unwrap();
            assert_eq!(owner.workspaces.len(), 1);
        }
    }
    #[test]
    fn basis_revisions_cannot_go_backwards_or_change_under_one_revision() {
        let mut owner = registered();
        let mut changed = draft();
        changed.model_hash["value"] = json!("b".repeat(64));
        let a = auth(&owner);
        let backing = lease(&owner);
        assert_eq!(
            owner
                .action(
                    "main",
                    Action::Basis {
                        authority: a,
                        basis: changed
                    },
                    paths(),
                    backing
                )
                .unwrap_err(),
            Error::Workspace
        );
    }
    #[cfg(target_arch = "aarch64")]
    #[test]
    fn qualified_launch_still_requires_the_private_owner_before_any_resource_lookup() {
        let mut owner = registered();
        let mut authority = auth(&owner);
        authority.owner_capability = "forged-owner".into();
        let backing = lease(&owner);
        assert_eq!(
            owner
                .action(
                    "main",
                    Action::Start {
                        authority,
                        provider_revision: "invented".into()
                    },
                    paths(),
                    backing
                )
                .unwrap_err(),
            Error::Authority
        );
        assert!(owner.live.is_none());
    }
    #[cfg(not(target_arch = "aarch64"))]
    #[test]
    fn the_unqualified_public_launcher_never_reads_resources() {
        let mut owner = registered();
        let a = auth(&owner);
        let backing = lease(&owner);
        assert_eq!(
            owner
                .action(
                    "main",
                    Action::Start {
                        authority: a,
                        provider_revision: "invented".into()
                    },
                    paths(),
                    backing
                )
                .unwrap_err(),
            Error::Unavailable
        );
        assert!(owner.live.is_none());
    }
}
