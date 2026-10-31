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
    collections::BTreeMap,
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
                let owner = &self.workspaces[i];
                let result = Live::start(
                    &self.registry,
                    runtime_paths,
                    config.model.clone(),
                    config.context_window,
                    provider,
                    config.secret.take(),
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
                validate_actual(&self.registry, meta, &outcome, &proof)?;
                // Native actual, proof and its bounded retained frontend copy.
                let record_backing = self
                    .registry
                    .acquire_interactive_ingress(512 * 1024)
                    .map_err(|_| Error::Admission)?;
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
fn validate_actual(
    registry: &SolveJobRegistry,
    meta: &Meta,
    raw: &str,
    proof: &Proof,
) -> Result<(), Error> {
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
            ..
        } if matches!(meta.tool, Tool::Solve | Tool::ReadResults) && basis == &meta.basis => {
            let state = registry.status(job).map_err(|_| Error::Evidence)?;
            if (status == "completed" && state.state != "completed")
                || (status == "failed" && state.state != "failed")
                || (status == "cancelled" && state.state != "cancelled")
            {
                return Err(Error::Evidence);
            }
        }
        _ => return Err(Error::Evidence),
    }
    Ok(())
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
