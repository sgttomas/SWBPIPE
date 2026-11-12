//! Pure stock 0.160 app-server protocol state. The host must check raw grammar,
//! byte/lease budgets and exact delivered JSON numbers before creating `Value`.
//! This module performs no I/O, authorization, domain operation, or retry.
//! An error poisons the connection: the host must stop dispatch and call
//! `disconnect` to recover unresolved references. It must keep reading while
//! dispatchers work; none of these methods waits for a domain operation.

use serde::Serialize;
use super::catalog::{Catalog, Tool};
use serde_json::{json, Value};
use std::io::{self, Write};

// Additional transport guards, not catalog capability limits. Measured in UTF-8 bytes.
pub const MAX_ID_BYTES: usize = 256;
pub const MAX_TEXT_BYTES: usize = 64 * 1024;
pub const MAX_CONTROL_BYTES: usize = 16 * 1024;
pub const MAX_CALLS_PER_TURN: usize = 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error(&'static str);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for Error {}

/// Allowlisted rejection telemetry. No supplier text, paths, tool names or key.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct RpcRejection {
    pub method: &'static str,
    pub code: Option<i64>,
    pub class: &'static str,
}
fn rejection(kind: Rpc, error: &Value) -> RpcRejection {
    let method = match kind {
        Rpc::Initialize => "initialize",
        Rpc::ThreadStart => "thread/start",
        Rpc::TurnStart => "turn/start",
        Rpc::Interrupt => "turn/interrupt",
    };
    let code = error.get("code").and_then(Value::as_i64);
    let text = error.get("message").and_then(Value::as_str).unwrap_or("");
    let class = if text.contains("dynamic tool input schema") {
        "dynamic_tool_schema_unsupported"
    } else if text.contains("configuration") || text.contains("config load") {
        "configuration_load_error"
    } else if text.contains("tool name") {
        "tool_name_invalid"
    } else {
        match code {
            Some(-32600) => "invalid_request",
            Some(-32601) => "method_not_found",
            Some(-32602) => "invalid_params",
            _ => "request_rejected",
        }
    };
    RpcRejection {
        method,
        code,
        class,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestRef(String);
impl RequestRef {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub enum Event {
    Initialized,
    ThreadStarted {
        thread_id: String,
    },
    TurnStarted {
        turn_id: String,
    },
    TurnCompleted {
        turn_id: String,
        status: String,
    },
    InterruptAcknowledged,
    ThreadStatus {
        status: String,
    },
    MessageDelta {
        item_id: String,
        text: String,
    },
    MessageFinal {
        item_id: String,
        text: String,
    },
    ToolRequested {
        request_ref: RequestRef,
        thread_id: String,
        turn_id: String,
        call_id: String,
        tool: Tool,
        arguments: Value,
    },
    /// The vendor turn ended. The host must still resolve the domain operation;
    /// completion returns `Detached` and emits no response to the vendor.
    ToolDetached {
        request_ref: RequestRef,
        thread_id: String,
        turn_id: String,
        call_id: String,
        tool: Tool,
    },
    /// Connection is gone; this reference is invalid and must not be retried.
    ToolLost {
        request_ref: RequestRef,
        thread_id: String,
        turn_id: String,
        call_id: String,
        tool: Tool,
    },
}
#[derive(Default, Debug)]
pub struct Actions {
    pub outbound: Vec<Value>,
    pub events: Vec<Event>,
}
#[derive(Debug)]
pub enum Completion {
    Delivered(Value),
    Detached,
}

/// All values are trusted host configuration, never model tool arguments.
pub struct ThreadOptions {
    pub model: String,
    pub provider: String,
    pub cwd: String,
    pub base_instructions: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    New,
    Initializing,
    Ready,
    StartingThread,
    ThreadReady,
    StartingTurn,
    InTurn,
    Poisoned,
    Closed,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Rpc {
    Initialize,
    ThreadStart,
    TurnStart,
    Interrupt,
}
struct Pending {
    reference: RequestRef,
    rpc_id: Value,
    thread_id: String,
    turn_id: String,
    call_id: String,
    tool: Tool,
    detached: bool,
}

pub struct Protocol {
    catalog: Catalog,
    token: String,
    proposal_cap: u64,
    phase: Phase,
    next_rpc: u64,
    next_ref: u64,
    outstanding: Vec<(u64, Rpc)>,
    thread: Option<String>,
    turn: Option<String>,
    turn_announced: bool,
    pending: Vec<Pending>,
    seen_rpc: Vec<Value>,
    seen_call: Vec<String>,
    last_rejection: Option<RpcRejection>,
    refusal: Option<Refusal>,
    #[cfg(any(test, feature = "native-smoke"))]
    observed: observed::Observed,
}

fn bounded(s: &str, max: usize) -> bool {
    !s.is_empty() && s.len() <= max && !s.chars().any(char::is_control)
}
fn identity(v: Option<&Value>) -> Result<&str, Error> {
    v.and_then(Value::as_str)
        .filter(|s| bounded(s, MAX_ID_BYTES))
        .ok_or(Error("invalid identity"))
}
fn rpc_id(v: Option<&Value>) -> Result<Value, Error> {
    let v = v.ok_or(Error("missing request identity"))?;
    if v.as_str().is_some_and(|s| bounded(s, MAX_ID_BYTES))
        || v.as_i64().is_some()
        || v.as_u64().is_some()
    {
        Ok(v.clone())
    } else {
        Err(Error("invalid request identity"))
    }
}
// Count serialized bytes without allocating a second copy of model arguments.
struct Counter {
    used: usize,
    max: usize,
}
impl Write for Counter {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.used = self
            .used
            .checked_add(b.len())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "limit"))?;
        if self.used > self.max {
            return Err(io::Error::new(io::ErrorKind::Other, "limit"));
        }
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn within(value: &Value, limit: u64) -> bool {
    let max = usize::try_from(limit).unwrap_or(usize::MAX);
    serde_json::to_writer(Counter { used: 0, max }, value).is_ok()
}
fn turn_id(v: &Value) -> Result<&str, Error> {
    identity(v.pointer("/turn/id"))
}
fn turn_status(v: &Value) -> Result<&str, Error> {
    v.pointer("/turn/status")
        .and_then(Value::as_str)
        .filter(|s| matches!(*s, "inProgress" | "completed" | "interrupted" | "failed"))
        .ok_or(Error("invalid turn status"))
}

// Item kinds this host admits, from stock 0.160.0's ThreadItem schema. Every
// other kind, and an item with no string kind, ends the session: the default is
// refusal, so a kind a later supplier adds is refused until it is listed here.
// - agentMessage: the reply, which the host reads as a message.
// - dynamicToolCall: the report of this host's own offered tools; the call
//   itself reaches the host only as an `item/tool/call` request it validates.
// - userMessage: the record of the person's input that the host sent.
// - reasoning: the model's reasoning text, conversation content only.
// - contextCompaction: the runtime summarising the conversation through the
//   relay when it nears the configured context window; it reaches nothing
//   outside the conversation, and a long session cannot continue without it.
// Refused although named by the schema: hookPrompt (a hook ran), plan (the
// runtime's plan mode), enteredReviewMode and exitedReviewMode (a runtime
// mode the host never requests), functionCallOutput (the output of a function
// call other than the reported dynamicToolCall).
// This is an observation tripwire, not proof that configuration/OS isolation
// prevented an action.
const ADMITTED_ITEM_KINDS: [&str; 5] = [
    "agentMessage",
    "dynamicToolCall",
    "userMessage",
    "reasoning",
    "contextCompaction",
];
// The runtime's own tools. Kept apart from other refused kinds so the reason
// says "the runtime used a tool of its own".
const BUILTIN_TOOL_ITEM_KINDS: [&str; 9] = [
    "commandExecution",
    "fileChange",
    "mcpToolCall",
    "collabAgentToolCall",
    "subAgentActivity",
    "webSearch",
    "imageView",
    "sleep",
    "imageGeneration",
];
const UNOFFERED_ACTIVITY: Error = Error("unoffered supplier tool activity");
const UNLISTED_ITEM: Error = Error("unlisted supplier item kind");
const UNLISTED_RESPONSE_MEMBER: Error = Error("unlisted supplier response member");

/// Why the protocol ended a session: a closed, payload-free class that the
/// panel turns into plain words. Never supplier text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The runtime reported activity of a tool it was not offered.
    ToolActivity,
    /// The runtime reported an item of a kind this host does not list, or a
    /// malformed report of an item.
    UnadmittedItem,
    /// Any other frame or report the host could not accept.
    Malformed,
}
impl Refusal {
    pub fn of(error: Error) -> Self {
        if error == UNOFFERED_ACTIVITY {
            Self::ToolActivity
        } else if error == UNLISTED_ITEM {
            Self::UnadmittedItem
        } else {
            Self::Malformed
        }
    }
    pub fn class(self) -> &'static str {
        match self {
            Self::ToolActivity => "runtime_tool_activity",
            Self::UnadmittedItem => "unadmitted_item",
            Self::Malformed => "malformed_report",
        }
    }
}

// Members that stock 0.160.0's schema defines for a Thread and a Turn, and for
// the result of each request this host sends. Any other member is refused, so
// no alternate list can carry items past the inspection below.
const THREAD_MEMBERS: [&str; 32] = [
    "agentNickname", "agentRole", "canAcceptDirectInput", "cliVersion", "createdAt", "cwd",
    "daybreakEnabled", "environments", "ephemeral", "extra", "forkedFromId", "gitInfo",
    "historyMode", "id", "model", "modelProvider", "name", "originator", "parentThreadId", "path",
    "preview", "projectId", "reasoningEffort", "recencyAt", "section", "sectionEnteredAt",
    "sessionId", "source", "status", "threadSource", "turns", "updatedAt",
];
const TURN_MEMBERS: [&str; 8] = [
    "completedAt", "durationMs", "error", "id", "items", "itemsView", "startedAt", "status",
];
const INITIALIZE_RESULT_MEMBERS: [&str; 4] = ["codexHome", "platformFamily", "platformOs", "userAgent"];
const THREAD_START_RESULT_MEMBERS: [&str; 14] = [
    "activePermissionProfile", "approvalPolicy", "approvalsReviewer", "cwd", "disabledPluginIds",
    "instructionSources", "model", "modelProvider", "multiAgentMode", "reasoningEffort",
    "runtimeWorkspaceRoots", "sandbox", "serviceTier", "thread",
];
const TURN_START_RESULT_MEMBERS: [&str; 1] = ["turn"];
const INTERRUPT_RESULT_MEMBERS: [&str; 0] = [];

fn admit_kind(kind: Option<&str>) -> Result<(), Error> {
    match kind {
        Some(k) if ADMITTED_ITEM_KINDS.contains(&k) => Ok(()),
        Some(k) if BUILTIN_TOOL_ITEM_KINDS.contains(&k) => Err(UNOFFERED_ACTIVITY),
        _ => Err(UNLISTED_ITEM),
    }
}
/// An item must be an object whose `type` is an admitted kind. A
/// `dynamicToolCall` must name a tool the catalog offers (its `tool` is the
/// bare declared name, as in an `item/tool/call` request) with no namespace;
/// its arguments are never read or executed from an item.
fn admit_item(item: Option<&Value>) -> Result<(), Error> {
    let item = item.and_then(Value::as_object).ok_or(UNLISTED_ITEM)?;
    let kind = item.get("type").and_then(Value::as_str);
    admit_kind(kind)?;
    if kind == Some("dynamicToolCall") {
        let tool = item.get("tool").and_then(Value::as_str).ok_or(UNLISTED_ITEM)?;
        if Tool::from_name(tool).is_none() || !item.get("namespace").map_or(true, Value::is_null) {
            return Err(UNOFFERED_ACTIVITY);
        }
    }
    Ok(())
}
fn object_within<'a>(
    value: &'a Value,
    members: &[&str],
    error: Error,
) -> Result<&'a serde_json::Map<String, Value>, Error> {
    let object = value.as_object().ok_or(error)?;
    if object.keys().all(|k| members.contains(&k.as_str())) {
        Ok(object)
    } else {
        Err(error)
    }
}
/// A supplied turn must be an object with only schema members; its `items`,
/// when present, an array of admitted items.
fn admit_turn(turn: &Value) -> Result<(), Error> {
    match object_within(turn, &TURN_MEMBERS, UNLISTED_ITEM)?.get("items") {
        None => Ok(()),
        Some(items) => items
            .as_array()
            .ok_or(UNLISTED_ITEM)?
            .iter()
            .try_for_each(|item| admit_item(Some(item))),
    }
}
/// A supplied thread must be an object with only schema members; its `turns`,
/// when present, an array of admitted turns.
fn admit_thread(thread: &Value) -> Result<(), Error> {
    match object_within(thread, &THREAD_MEMBERS, UNLISTED_ITEM)?.get("turns") {
        None => Ok(()),
        Some(turns) => turns.as_array().ok_or(UNLISTED_ITEM)?.iter().try_for_each(admit_turn),
    }
}
fn admit_turn_items(turn: Option<&Value>) -> Result<(), Error> {
    turn.map_or(Ok(()), admit_turn)
}
fn admit_thread_items(thread: Option<&Value>) -> Result<(), Error> {
    thread.map_or(Ok(()), admit_thread)
}
/// A response's result may hold only the members the pinned schema defines
/// for the request it answers; a thread or turn in it is inspected as above.
fn admit_response(kind: Rpc, result: &Value) -> Result<(), Error> {
    let members: &[&str] = match kind {
        Rpc::Initialize => &INITIALIZE_RESULT_MEMBERS,
        Rpc::ThreadStart => &THREAD_START_RESULT_MEMBERS,
        Rpc::TurnStart => &TURN_START_RESULT_MEMBERS,
        Rpc::Interrupt => &INTERRUPT_RESULT_MEMBERS,
    };
    let result = object_within(result, members, UNLISTED_RESPONSE_MEMBER)?;
    admit_thread_items(result.get("thread"))?;
    admit_turn_items(result.get("turn"))
}
/// Every place a server message can carry an item, or name an item kind in
/// its method (`item/<kind>/...`), is checked before the message is handled.
fn admit_notification_items(method: &str, p: &Value) -> Result<(), Error> {
    match method {
        "item/started" | "item/completed" => admit_item(p.get("item")),
        "turn/started" | "turn/completed" => admit_turn_items(p.get("turn")),
        "thread/started" => admit_thread_items(p.get("thread")),
        _ => match method.strip_prefix("item/") {
            // Deltas and progress keyed by item name its kind in the method.
            Some(rest) => admit_kind(rest.split('/').next()),
            None => Ok(()),
        },
    }
}

/// Native smoke only: one payload-free line for each distinct item kind,
/// unhandled notification method and unsupported server request method the
/// runtime reports, so an observed session shows the ordinary set. Names are
/// printed only when short and plain; nothing else from a frame is kept.
#[cfg(any(test, feature = "native-smoke"))]
mod observed {
    use super::{admit_item, admit_kind, Value};

    const PREFIX: &str = "[native-smoke-agent-host]";
    const MAX_NAME_BYTES: usize = 64;
    pub const MAX_LINES: usize = 64;

    fn plain(name: &str) -> &str {
        if !name.is_empty()
            && name.len() <= MAX_NAME_BYTES
            && name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'/' || b == b'_' || b == b'-')
        {
            name
        } else {
            "(unprintable)"
        }
    }
    fn kind_name(kind: Option<&Value>) -> &str {
        match kind {
            None => "(missing)",
            Some(Value::String(k)) => plain(k),
            Some(_) => "(non-string)",
        }
    }

    #[derive(Default)]
    pub struct Observed {
        seen: Vec<String>,
        unread: Vec<String>,
        full: bool,
    }
    impl Observed {
        fn line(&mut self, line: String) {
            if self.full || self.seen.contains(&line) {
                return;
            }
            if self.seen.len() >= MAX_LINES {
                self.full = true;
                self.unread.push(format!("{PREFIX} observation limit reached"));
                return;
            }
            self.seen.push(line.clone());
            self.unread.push(line);
        }
        fn kind(&mut self, kind: Option<&Value>, via: &str) {
            let decision = match admit_kind(kind.and_then(Value::as_str)) {
                Ok(()) => "admitted",
                Err(_) => "refused",
            };
            let line = format!("{PREFIX} item kind {} via {} {decision}", kind_name(kind), plain(via));
            self.line(line);
        }
        fn malformed(&mut self, via: &str) {
            let line = format!("{PREFIX} item kind (malformed) via {} refused", plain(via));
            self.line(line);
        }
        fn item(&mut self, item: Option<&Value>, via: &str) {
            match item {
                Some(Value::Object(o)) => {
                    let decision = match admit_item(item) {
                        Ok(()) => "admitted",
                        Err(_) => "refused",
                    };
                    let line = format!(
                        "{PREFIX} item kind {} via {} {decision}",
                        kind_name(o.get("type")),
                        plain(via)
                    );
                    self.line(line);
                }
                Some(_) => self.malformed(via),
                None => self.kind(None, via),
            }
        }
        fn turn(&mut self, turn: Option<&Value>, via: &str) {
            match turn {
                None => {}
                Some(Value::Object(t)) => match t.get("items") {
                    None => {}
                    Some(Value::Array(items)) => items.iter().for_each(|i| self.item(Some(i), via)),
                    Some(_) => self.malformed(via),
                },
                Some(_) => self.malformed(via),
            }
        }
        fn thread(&mut self, thread: Option<&Value>, via: &str) {
            match thread {
                None => {}
                Some(Value::Object(t)) => match t.get("turns") {
                    None => {}
                    Some(Value::Array(turns)) => turns.iter().for_each(|t| self.turn(Some(t), via)),
                    Some(_) => self.malformed(via),
                },
                Some(_) => self.malformed(via),
            }
        }
        pub fn notification_items(&mut self, method: &str, p: &Value) {
            match method {
                "item/started" | "item/completed" => self.item(p.get("item"), method),
                "turn/started" | "turn/completed" => self.turn(p.get("turn"), method),
                "thread/started" => self.thread(p.get("thread"), method),
                _ => {
                    if let Some(rest) = method.strip_prefix("item/") {
                        let kind = rest.split('/').next().map(|k| Value::String(k.to_owned()));
                        self.kind(kind.as_ref(), method);
                    }
                }
            }
        }
        pub fn response_items(&mut self, result: &Value, via: &str) {
            self.thread(result.get("thread"), via);
            self.turn(result.get("turn"), via);
        }
        pub fn ignored_notification(&mut self, method: &str) {
            let line = format!("{PREFIX} notification {} ignored", plain(method));
            self.line(line);
        }
        pub fn unsupported_request(&mut self, method: &str) {
            let line = format!("{PREFIX} server request {} refused", plain(method));
            self.line(line);
        }
        pub fn take(&mut self) -> Vec<String> {
            std::mem::take(&mut self.unread)
        }
    }
}

impl Protocol {
    pub fn last_rejection(&self) -> Option<RpcRejection> {
        self.last_rejection
    }
    /// Why a frame ended the session, once one has; a closed class.
    pub fn refusal(&self) -> Option<Refusal> {
        self.refusal
    }
    /// Native smoke only: the observation lines not yet taken. Each is
    /// payload-free and bounded; the caller prints them.
    #[cfg(any(test, feature = "native-smoke"))]
    pub fn take_observations(&mut self) -> Vec<String> {
        self.observed.take()
    }

    /// Token must be host-generated, unpredictable and unique per connection;
    /// it is never supplied by the model. The caller must not reuse it.
    pub fn new(catalog: Catalog, token: String, proposal_cap: u64) -> Result<Self, Error> {
        if token.len() < 16
            || token.len() > 128
            || !token
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || proposal_cap == 0
        {
            return Err(Error("invalid host configuration"));
        }
        Ok(Self {
            catalog,
            token,
            proposal_cap,
            phase: Phase::New,
            next_rpc: 1,
            next_ref: 1,
            outstanding: Vec::new(),
            thread: None,
            turn: None,
            turn_announced: false,
            pending: Vec::new(),
            seen_rpc: Vec::new(),
            seen_call: Vec::new(),
            last_rejection: None,
            refusal: None,
            #[cfg(any(test, feature = "native-smoke"))]
            observed: Default::default(),
        })
    }
    fn request(&mut self, kind: Rpc, method: &'static str, params: Value) -> Result<Value, Error> {
        let id = self.next_rpc;
        self.next_rpc = id
            .checked_add(1)
            .ok_or(Error("client identity exhausted"))?;
        self.outstanding.push((id, kind));
        Ok(json!({"id":id,"method":method,"params":params}))
    }
    pub fn initialize(&mut self, name: &str, version: &str) -> Result<Value, Error> {
        if self.phase != Phase::New || !bounded(name, 128) || !bounded(version, 128) {
            return Err(Error("invalid initialize"));
        }
        let v = self.request(Rpc::Initialize, "initialize", json!({"clientInfo":{"name":name,"version":version},"capabilities":{"experimentalApi":true}}))?;
        self.phase = Phase::Initializing;
        Ok(v)
    }
    pub fn start_thread(&mut self, options: ThreadOptions) -> Result<Value, Error> {
        if self.phase != Phase::Ready
            || !bounded(&options.model, 1024)
            || !bounded(&options.provider, 1024)
            || !bounded(&options.cwd, 4096)
            || options.base_instructions.is_empty()
            || options.base_instructions.len() > MAX_CONTROL_BYTES
            || options.base_instructions.contains('\0')
        {
            return Err(Error("invalid thread options or state"));
        }
        let params = json!({"model":options.model,"modelProvider":options.provider,"cwd":options.cwd,"baseInstructions":options.base_instructions,"ephemeral":true,"sandbox":"read-only","approvalPolicy":"never","environments":[],"dynamicTools":self.catalog.declarations()});
        let v = self.request(Rpc::ThreadStart, "thread/start", params)?;
        self.phase = Phase::StartingThread;
        Ok(v)
    }
    pub fn submit_text(&mut self, text: String) -> Result<Value, Error> {
        if self.phase != Phase::ThreadReady
            || !self.pending.is_empty()
            || !self.outstanding.is_empty()
            || text.is_empty()
            || text.len() > MAX_TEXT_BYTES
            || text.contains('\0')
        {
            return Err(Error("invalid turn input or state"));
        }
        let params = json!({"threadId":self.thread.as_ref().ok_or(Error("missing thread"))?,"input":[{"type":"text","text":text}]});
        let v = self.request(Rpc::TurnStart, "turn/start", params)?;
        self.turn = None;
        self.turn_announced = false;
        self.seen_rpc.clear();
        self.seen_call.clear();
        self.phase = Phase::StartingTurn;
        Ok(v)
    }
    /// This does not cancel or roll back a domain operation.
    pub fn interrupt(&mut self) -> Result<Value, Error> {
        if self.phase != Phase::InTurn || self.outstanding.iter().any(|(_, k)| *k == Rpc::Interrupt)
        {
            return Err(Error("invalid interrupt state"));
        }
        let params = json!({"threadId":self.thread.as_ref().ok_or(Error("missing thread"))?,"turnId":self.turn.as_ref().ok_or(Error("missing turn"))?});
        self.request(Rpc::Interrupt, "turn/interrupt", params)
    }
    /// One admitted frame in, bounded actions out. `error_basis` is trusted
    /// dispatcher state and is used only for a schema-invalid known tool call.
    /// It must make the catalog execution_error output valid for that tool.
    pub fn handle_frame(&mut self, frame: Value, error_basis: &Value) -> Result<Actions, Error> {
        if matches!(self.phase, Phase::Poisoned | Phase::Closed) {
            return Err(Error("connection closed"));
        }
        let result = self.handle_inner(frame, error_basis);
        if let Err(error) = &result {
            self.phase = Phase::Poisoned;
            self.refusal = Some(Refusal::of(*error));
        }
        result
    }
    fn handle_inner(&mut self, mut frame: Value, basis: &Value) -> Result<Actions, Error> {
        let mut actions = Actions::default();
        let object = frame.as_object_mut().ok_or(Error("invalid frame"))?;
        if let Some(method) = object.get("method") {
            let method = method
                .as_str()
                .filter(|s| bounded(s, MAX_ID_BYTES))
                .ok_or(Error("invalid method"))?
                .to_owned();
            if object.contains_key("id") {
                let id = rpc_id(object.get("id"))?;
                if method == "item/tool/call" {
                    let params = object.remove("params").ok_or(Error("missing parameters"))?;
                    self.tool_call(id, params, basis, &mut actions)?;
                } else {
                    #[cfg(any(test, feature = "native-smoke"))]
                    self.observed.unsupported_request(&method);
                    // No native method conveys authority, including approval requests.
                    if self.seen_rpc.contains(&id) {
                        return Err(Error("duplicate server request"));
                    }
                    if self.seen_rpc.len() >= MAX_CALLS_PER_TURN {
                        return Err(Error("server request limit"));
                    }
                    self.seen_rpc.push(id.clone());
                    actions.outbound.push(
                        json!({"id":id,"error":{"code":-32601,"message":"Unsupported request"}}),
                    );
                }
            } else {
                let params = object.remove("params").unwrap_or(Value::Null);
                self.notification(&method, params, &mut actions)?;
            }
        } else {
            let id = object
                .get("id")
                .and_then(Value::as_u64)
                .ok_or(Error("invalid response identity"))?;
            let pos = self
                .outstanding
                .iter()
                .position(|(n, _)| *n == id)
                .ok_or(Error("unexpected response identity"))?;
            if object.contains_key("error") || !object.contains_key("result") {
                self.last_rejection = Some(rejection(
                    self.outstanding[pos].1,
                    object.get("error").unwrap_or(&Value::Null),
                ));
                return Err(Error("client request failed"));
            }
            let result = object.remove("result").ok_or(Error("missing response"))?;
            let kind = self.outstanding[pos].1;
            self.response(kind, &result, &mut actions)?;
            self.outstanding.remove(pos);
        }
        Ok(actions)
    }
    fn response(&mut self, kind: Rpc, result: &Value, actions: &mut Actions) -> Result<(), Error> {
        #[cfg(any(test, feature = "native-smoke"))]
        self.observed.response_items(
            result,
            match kind {
                Rpc::Initialize => "initialize-response",
                Rpc::ThreadStart => "thread/start-response",
                Rpc::TurnStart => "turn/start-response",
                Rpc::Interrupt => "turn/interrupt-response",
            },
        );
        admit_response(kind, result)?;
        match kind {
            Rpc::Initialize if self.phase == Phase::Initializing => {
                if !result.is_object()
                    || ["codexHome", "platformFamily", "platformOs", "userAgent"]
                        .iter()
                        .any(|k| !result.get(k).is_some_and(Value::is_string))
                {
                    return Err(Error("invalid initialize response"));
                }
                self.phase = Phase::Ready;
                actions
                    .outbound
                    .push(json!({"method":"initialized","params":{}}));
                actions.events.push(Event::Initialized);
            }
            Rpc::ThreadStart if self.phase == Phase::StartingThread => {
                let id = identity(result.pointer("/thread/id"))?.to_owned();
                if self.thread.as_ref().is_some_and(|v| v != &id) {
                    return Err(Error("thread response mismatch"));
                }
                if result.get("approvalPolicy").and_then(Value::as_str) != Some("never")
                    || result.pointer("/sandbox/type").and_then(Value::as_str) != Some("readOnly")
                    || result
                        .pointer("/sandbox/networkAccess")
                        .is_some_and(|v| v.as_bool() != Some(false))
                    || result
                        .pointer("/thread/ephemeral")
                        .is_some_and(|v| v.as_bool() != Some(true))
                    || result.pointer("/thread/environments").is_some_and(|v| {
                        !v.is_null() && v.as_array().map_or(true, |a| !a.is_empty())
                    })
                {
                    return Err(Error("unexpected thread policy"));
                }
                self.thread = Some(id.clone());
                self.phase = Phase::ThreadReady;
                actions.events.push(Event::ThreadStarted { thread_id: id });
            }
            Rpc::TurnStart if matches!(self.phase, Phase::StartingTurn | Phase::InTurn) => {
                let id = turn_id(result)?.to_owned();
                if turn_status(result)? != "inProgress"
                    || self.turn.as_ref().is_some_and(|v| v != &id)
                {
                    return Err(Error("turn response mismatch"));
                }
                self.turn = Some(id.clone());
                self.phase = Phase::InTurn;
                if !self.turn_announced {
                    self.turn_announced = true;
                    actions.events.push(Event::TurnStarted { turn_id: id });
                }
            }
            Rpc::Interrupt
                if matches!(self.phase, Phase::InTurn | Phase::ThreadReady)
                    && result.is_object() =>
            {
                actions.events.push(Event::InterruptAcknowledged);
            }
            _ => return Err(Error("response in invalid state")),
        }
        Ok(())
    }
    fn check_thread(&self, params: &Value) -> Result<(), Error> {
        if Some(identity(params.get("threadId"))?) != self.thread.as_deref() {
            return Err(Error("thread mismatch"));
        }
        Ok(())
    }
    fn check_turn(&self, params: &Value) -> Result<(), Error> {
        self.check_thread(params)?;
        if self.phase != Phase::InTurn
            || self.outstanding.iter().any(|(_, k)| *k == Rpc::TurnStart)
            || Some(identity(params.get("turnId"))?) != self.turn.as_deref()
        {
            return Err(Error("turn mismatch or unconfirmed turn"));
        }
        Ok(())
    }
    fn notification(&mut self, method: &str, p: Value, actions: &mut Actions) -> Result<(), Error> {
        #[cfg(any(test, feature = "native-smoke"))]
        self.observed.notification_items(method, &p);
        admit_notification_items(method, &p)?;
        match method {
            "turn/started" => {
                self.check_thread(&p)?;
                if !matches!(self.phase, Phase::StartingTurn | Phase::InTurn)
                    || turn_status(&p)? != "inProgress"
                {
                    return Err(Error("unexpected turn start"));
                }
                let id = turn_id(&p)?.to_owned();
                if self.turn.as_ref().is_some_and(|v| v != &id) {
                    return Err(Error("turn mismatch"));
                }
                self.turn = Some(id.clone());
                self.phase = Phase::InTurn;
                if !self.turn_announced {
                    self.turn_announced = true;
                    actions.events.push(Event::TurnStarted { turn_id: id });
                }
            }
            "turn/completed" => {
                self.check_thread(&p)?;
                if self.phase != Phase::InTurn
                    || Some(turn_id(&p)?) != self.turn.as_deref()
                    || self.outstanding.iter().any(|(_, k)| *k == Rpc::TurnStart)
                {
                    return Err(Error("unsupported turn completion ordering"));
                }
                let status = turn_status(&p)?;
                if status == "inProgress" {
                    return Err(Error("invalid completion status"));
                }
                for pending in &mut self.pending {
                    if !pending.detached {
                        pending.detached = true;
                        actions.events.push(Event::ToolDetached {
                            request_ref: pending.reference.clone(),
                            thread_id: pending.thread_id.clone(),
                            turn_id: pending.turn_id.clone(),
                            call_id: pending.call_id.clone(),
                            tool: pending.tool,
                        });
                    }
                }
                actions.events.push(Event::TurnCompleted {
                    turn_id: self.turn.take().ok_or(Error("missing turn"))?,
                    status: status.to_owned(),
                });
                self.phase = Phase::ThreadReady;
            }
            "item/agentMessage/delta" => {
                self.check_turn(&p)?;
                let item_id = identity(p.get("itemId"))?.to_owned();
                let text = p
                    .get("delta")
                    .and_then(Value::as_str)
                    .filter(|s| s.len() <= MAX_TEXT_BYTES)
                    .ok_or(Error("message text limit"))?
                    .to_owned();
                actions.events.push(Event::MessageDelta { item_id, text });
            }
            // Its item's kind was admitted above; nothing else is handled.
            "item/started" => {}
            "item/completed" => {
                self.check_turn(&p)?;
                let item = p
                    .get("item")
                    .and_then(Value::as_object)
                    .ok_or(Error("invalid item"))?;
                if item.get("type").and_then(Value::as_str) == Some("agentMessage") {
                    let item_id = identity(item.get("id"))?.to_owned();
                    let text = item
                        .get("text")
                        .and_then(Value::as_str)
                        .filter(|s| s.len() <= MAX_TEXT_BYTES)
                        .ok_or(Error("message text limit"))?
                        .to_owned();
                    actions.events.push(Event::MessageFinal { item_id, text });
                }
            }
            "thread/status/changed" => {
                if self.phase == Phase::StartingThread && self.thread.is_none() {
                    self.thread = Some(identity(p.get("threadId"))?.to_owned());
                }
                self.check_thread(&p)?;
                let status = p
                    .pointer("/status/type")
                    .and_then(Value::as_str)
                    .filter(|s| matches!(*s, "notLoaded" | "idle" | "systemError" | "active"))
                    .ok_or(Error("invalid thread status"))?;
                actions.events.push(Event::ThreadStatus {
                    status: status.to_owned(),
                });
            }
            // The supplier may send thread/started before its response. It is
            // informational only; response supplies authoritative thread id.
            "thread/started" if self.phase == Phase::StartingThread => {
                let id = identity(p.pointer("/thread/id"))?;
                if self.thread.as_deref().is_some_and(|old| old != id) {
                    return Err(Error("thread mismatch"));
                }
                self.thread = Some(id.to_owned());
            }
            // Other notifications are informational and carry no authority.
            _ => {
                #[cfg(any(test, feature = "native-smoke"))]
                self.observed.ignored_notification(method);
            }
        }
        Ok(())
    }
    fn tool_call(
        &mut self,
        id: Value,
        mut p: Value,
        basis: &Value,
        actions: &mut Actions,
    ) -> Result<(), Error> {
        self.check_turn(&p)?;
        let call_id = identity(p.get("callId"))?.to_owned();
        let name = identity(p.get("tool"))?;
        if self.seen_rpc.contains(&id) || self.seen_call.contains(&call_id) {
            return Err(Error("duplicate tool identity"));
        }
        if self.seen_rpc.len() >= MAX_CALLS_PER_TURN || self.seen_call.len() >= MAX_CALLS_PER_TURN {
            return Err(Error("turn identity limit"));
        }
        self.seen_rpc.push(id.clone());
        self.seen_call.push(call_id.clone());
        let tool = Tool::from_name(name);
        if tool.is_none() || !p.get("namespace").map_or(true, Value::is_null) {
            actions
                .outbound
                .push(json!({"id":id,"error":{"code":-32601,"message":"Unsupported request"}}));
            return Ok(());
        }
        let tool = tool.ok_or(Error("unsupported tool"))?;
        let args = p
            .as_object_mut()
            .and_then(|o| o.remove("arguments"))
            .unwrap_or(Value::Null);
        let valid = within(&args, self.catalog.limits().argument_bytes)
            && self.catalog.validate_input(tool, &args).is_ok()
            && (tool != Tool::ProposeOperations
                || self
                    .catalog
                    .validate_proposal_context(&args, self.proposal_cap)
                    .is_ok());
        if !valid || self.pending.len() as u64 >= self.catalog.limits().pending_calls {
            let output = json!({"status":"execution_error","basis":basis,"message":"Tool request could not be admitted.","data":null});
            actions.outbound.push(self.tool_response(id, tool, output)?);
            return Ok(());
        }
        let seq = self.next_ref;
        self.next_ref = seq
            .checked_add(1)
            .ok_or(Error("request identity exhausted"))?;
        let reference = RequestRef(format!("{}:{}", self.token, seq));
        let thread_id = self.thread.as_ref().ok_or(Error("missing thread"))?.clone();
        let turn_id = self.turn.as_ref().ok_or(Error("missing turn"))?.clone();
        self.pending.push(Pending {
            reference: reference.clone(),
            rpc_id: id,
            thread_id: thread_id.clone(),
            turn_id: turn_id.clone(),
            call_id: call_id.clone(),
            tool,
            detached: false,
        });
        actions.events.push(Event::ToolRequested {
            request_ref: reference,
            thread_id,
            turn_id,
            call_id,
            tool,
            arguments: args,
        });
        Ok(())
    }
    fn tool_response(&self, id: Value, tool: Tool, output: Value) -> Result<Value, Error> {
        if self.catalog.validate_output(tool, &output).is_err()
            || !within(&output, self.catalog.limits().result_bytes)
        {
            return Err(Error("invalid tool outcome"));
        }
        let success = output.get("status").and_then(Value::as_str) != Some("execution_error");
        let text = serde_json::to_string(&output).map_err(|_| Error("invalid tool outcome"))?;
        Ok(
            json!({"id":id,"result":{"contentItems":[{"type":"inputText","text":text}],"success":success}}),
        )
    }
    /// Resolve only by the opaque reference. Invalid outcomes leave it pending.
    /// Detached completions are validated but never sent to the dead vendor RPC.
    pub fn complete(&mut self, reference: &str, outcome: Value) -> Result<Completion, Error> {
        if matches!(self.phase, Phase::Closed | Phase::Poisoned) {
            return Err(Error("connection closed"));
        }
        let pos = self
            .pending
            .iter()
            .position(|p| p.reference.as_str() == reference)
            .ok_or(Error("unknown request reference"))?;
        let p = &self.pending[pos];
        let response = self.tool_response(p.rpc_id.clone(), p.tool, outcome)?;
        let detached = p.detached;
        self.pending.remove(pos);
        Ok(if detached {
            Completion::Detached
        } else {
            Completion::Delivered(response)
        })
    }
    /// Invalidates all pending references and reports unresolved work once.
    pub fn disconnect(&mut self) -> Vec<Event> {
        self.phase = Phase::Closed;
        self.outstanding.clear();
        self.pending
            .drain(..)
            .map(|p| Event::ToolLost {
                request_ref: p.reference,
                thread_id: p.thread_id,
                turn_id: p.turn_id,
                call_id: p.call_id,
                tool: p.tool,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn safe_rpc_rejection_excludes_arbitrary_supplier_text() {
        let error=serde_json::json!({"code":-32600,"message":"configuration load failed: invented-sensitive-marker"});
        let safe=super::rejection(super::Rpc::ThreadStart,&error);
        assert_eq!(safe.method,"thread/start");assert_eq!(safe.code,Some(-32600));
        assert_eq!(safe.class,"configuration_load_error");
        assert!(!serde_json::to_string(&safe).unwrap().contains("invented-sensitive-marker"));
    }

    use super::*;

    fn basis() -> Value {
        json!({"workspaceHandle":"workspace-1","applicationGeneration":1,"modelRevision":2,"modelHash":null,"stateId":null,"runId":null})
    }
    fn outcome(status: &str) -> Value {
        json!({"status":status,"basis":basis(),"message":"Unavailable","data":null})
    }
    fn frame(id: u64, result: Value) -> Value {
        json!({"id":id,"result":result})
    }
    fn turn(id: &str, status: &str) -> Value {
        json!({"id":id,"items":[],"status":status})
    }
    fn notification(method: &str, params: Value) -> Value {
        json!({"method":method,"params":params})
    }
    fn call(rpc: Value, call_id: &str, tool: &str, args: Value) -> Value {
        json!({"id":rpc,"method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":call_id,"tool":tool,"arguments":args}})
    }
    fn ready() -> Protocol {
        let mut p = Protocol::new(Catalog::load().unwrap(), "0123456789abcdef".into(), 16).unwrap();
        let init = p.initialize("test", "1").unwrap();
        assert_eq!(init["params"]["capabilities"]["experimentalApi"], true);
        let init_reply = json!({"codexHome":"/private","platformFamily":"test","platformOs":"test","userAgent":"test"});
        assert_eq!(
            p.handle_frame(frame(1, init_reply), &basis())
                .unwrap()
                .outbound[0]["method"],
            "initialized"
        );
        let request = p
            .start_thread(ThreadOptions {
                model: "test".into(),
                provider: "test".into(),
                cwd: "/test".into(),
                base_instructions: "test".into(),
            })
            .unwrap();
        assert_eq!(request["params"]["ephemeral"], true);
        assert_eq!(request["params"]["sandbox"], "read-only");
        assert_eq!(request["params"]["approvalPolicy"], "never");
        assert_eq!(request["params"]["environments"], json!([]));
        let declarations = request["params"]["dynamicTools"].as_array().unwrap();
        assert_eq!(declarations.len(), 7);
        for declaration in declarations {
            assert_eq!(declaration.as_object().unwrap().len(), 4);
            assert_eq!(declaration["type"], "function");
        }
        p.handle_frame(frame(2, json!({"thread":{"id":"thread-1"},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}})), &basis()).unwrap();
        p
    }
    fn active() -> Protocol {
        let mut p = ready();
        p.submit_text("hello".into()).unwrap();
        p.handle_frame(
            frame(3, json!({"turn":turn("turn-1", "inProgress")})),
            &basis(),
        )
        .unwrap();
        p
    }
    fn requested(actions: Actions) -> RequestRef {
        match actions.events.into_iter().next().unwrap() {
            Event::ToolRequested { request_ref, .. } => request_ref,
            _ => panic!("expected tool request"),
        }
    }
    fn call_summary(rpc: Value, id: &str) -> Value {
        call(rpc, id, Tool::Summary.name(), json!({}))
    }

    #[test]
    fn unoffered_supplier_tool_activity_closes_with_pending_calls_preserved() {
        for method in ["item/started", "item/completed"] {
            for kind in [
                "commandExecution",
                "fileChange",
                "mcpToolCall",
                "collabAgentToolCall",
                "subAgentActivity",
                "webSearch",
                "imageView",
                "sleep",
                "imageGeneration",
            ] {
                let mut p = active();
                let reference = requested(
                    p.handle_frame(call_summary(json!(7), "pending"), &basis())
                        .unwrap(),
                );
                let result = p.handle_frame(notification(method, json!({"threadId":"thread-1","turnId":"turn-1","item":{"id":"unoffered","type":kind}})), &basis());
                assert_eq!(result.err(), Some(UNOFFERED_ACTIVITY), "{method}: {kind}");
                assert!(
                    matches!(p.disconnect().as_slice(), [Event::ToolLost { request_ref, .. }] if request_ref == &reference)
                );
                assert!(p.disconnect().is_empty());
            }
        }
    }

    fn item_frame(method: &str, item: Value) -> Value {
        notification(
            method,
            json!({"threadId":"thread-1","turnId":"turn-1","item":item}),
        )
    }

    /// The pending call is reported lost once, as for any protocol failure,
    /// and the poisoned protocol accepts nothing further.
    fn assert_ends(mut p: Protocol, reference: RequestRef, frame: Value, reason: Error) {
        let label = frame.to_string();
        assert_eq!(p.handle_frame(frame, &basis()).err(), Some(reason), "{label}");
        assert!(
            p.handle_frame(call_summary(json!(99), "after"), &basis())
                .is_err(),
            "{label}"
        );
        assert!(
            matches!(p.disconnect().as_slice(), [Event::ToolLost { request_ref, .. }] if request_ref == &reference),
            "{label}"
        );
    }
    fn active_with_pending() -> (Protocol, RequestRef) {
        let mut p = active();
        let reference = requested(
            p.handle_frame(call_summary(json!(7), "pending"), &basis())
                .unwrap(),
        );
        (p, reference)
    }

    #[test]
    fn admitted_item_kinds_pass_and_are_handled_as_before() {
        let mut p = active();
        for method in ["item/started", "item/completed"] {
            for kind in [
                "dynamicToolCall",
                "userMessage",
                "reasoning",
                "contextCompaction",
            ] {
                let actions = p
                    .handle_frame(item_frame(method, json!({"id":"item","type":kind,"tool":Tool::Summary.name()})), &basis())
                    .unwrap();
                assert!(actions.events.is_empty() && actions.outbound.is_empty(), "{method}: {kind}");
            }
            let message = p
                .handle_frame(
                    item_frame(method, json!({"id":"message","type":"agentMessage","text":"hi"})),
                    &basis(),
                )
                .unwrap();
            match (method, message.events.as_slice()) {
                ("item/started", []) => {}
                ("item/completed", [Event::MessageFinal { item_id, text }]) => {
                    assert_eq!((item_id.as_str(), text.as_str()), ("message", "hi"))
                }
                other => panic!("unexpected {other:?}"),
            }
        }
        for method in [
            "item/reasoning/textDelta",
            "item/reasoning/summaryTextDelta",
            "item/reasoning/summaryPartAdded",
        ] {
            p.handle_frame(
                notification(method, json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r"})),
                &basis(),
            )
            .unwrap();
        }
        assert!(matches!(
            p.handle_frame(
                notification("item/agentMessage/delta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"m","delta":"x"})),
                &basis()
            )
            .unwrap()
            .events
            .as_slice(),
            [Event::MessageDelta { .. }]
        ));
        let admitted: Vec<Value> = ADMITTED_ITEM_KINDS
            .iter()
            .map(|k| json!({"id":"i","type":k,"text":"t","tool":Tool::Summary.name(),"namespace":null}))
            .collect();
        p.handle_frame(
            notification("turn/started", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"inProgress","items":admitted}})),
            &basis(),
        )
        .unwrap();
        assert!(matches!(
            p.handle_frame(call_summary(json!(7), "valid"), &basis())
                .unwrap()
                .events
                .as_slice(),
            [Event::ToolRequested { .. }]
        ));
        assert!(matches!(
            p.handle_frame(
                notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":admitted}})),
                &basis()
            )
            .unwrap()
            .events
            .as_slice(),
            [Event::ToolDetached { .. }, Event::TurnCompleted { .. }]
        ));
    }

    #[test]
    fn unlisted_missing_or_malformed_item_kinds_end_the_session() {
        let kinds = [
            json!("hookPrompt"),
            json!("plan"),
            json!("enteredReviewMode"),
            json!("exitedReviewMode"),
            json!("functionCallOutput"),
            json!("inventedKind"),
            json!(""),
            json!("AgentMessage"),
            json!("agentmessage"),
            json!("DYNAMICTOOLCALL"),
            json!("CommandExecution"),
            json!(" agentMessage"),
            json!(1),
            json!(true),
            json!(null),
            json!(["agentMessage"]),
            json!({"type":"agentMessage"}),
        ];
        for method in ["item/started", "item/completed"] {
            for kind in &kinds {
                let (p, reference) = active_with_pending();
                let frame = item_frame(method, json!({"id":"item","type":kind,"text":"t"}));
                assert_ends(p, reference, frame, UNLISTED_ITEM);
            }
            for item in [json!({"id":"item","text":"t"}), json!("agentMessage"), json!(null)] {
                let (p, reference) = active_with_pending();
                assert_ends(p, reference, item_frame(method, item), UNLISTED_ITEM);
            }
            let (p, reference) = active_with_pending();
            let frame = notification(method, json!({"threadId":"thread-1","turnId":"turn-1"}));
            assert_ends(p, reference, frame, UNLISTED_ITEM);
        }
    }

    #[test]
    fn items_inside_turns_threads_and_item_keyed_methods_are_checked() {
        let turn_with = |status: &str, item: Value| json!({"id":"turn-1","status":status,"items":[{"id":"ok","type":"userMessage"},item]});
        for (item, reason) in [
            (json!({"id":"x","type":"commandExecution"}), UNOFFERED_ACTIVITY),
            (json!({"id":"x","type":"plan"}), UNLISTED_ITEM),
            (json!({"id":"x"}), UNLISTED_ITEM),
        ] {
            for frame in [
                notification("turn/completed", json!({"threadId":"thread-1","turn":turn_with("completed", item.clone())})),
                notification("turn/started", json!({"threadId":"thread-1","turn":turn_with("inProgress", item.clone())})),
                notification("thread/started", json!({"thread":{"id":"thread-1","turns":[turn_with("completed", item.clone())]}})),
            ] {
                let (p, reference) = active_with_pending();
                assert_ends(p, reference, frame, reason);
            }
        }
        // A turn's items or a thread's turns that are not arrays are refused.
        for frame in [
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":{"type":"agentMessage"}}})),
            notification("thread/started", json!({"thread":{"id":"thread-1","turns":{}}})),
        ] {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, frame, UNLISTED_ITEM);
        }
        // Notifications keyed by item name its kind in the method.
        for (method, reason) in [
            ("item/commandExecution/outputDelta", UNOFFERED_ACTIVITY),
            ("item/commandExecution/terminalInteraction", UNOFFERED_ACTIVITY),
            ("item/fileChange/outputDelta", UNOFFERED_ACTIVITY),
            ("item/fileChange/patchUpdated", UNOFFERED_ACTIVITY),
            ("item/mcpToolCall/progress", UNOFFERED_ACTIVITY),
            ("item/plan/delta", UNLISTED_ITEM),
            ("item/autoApprovalReview/started", UNLISTED_ITEM),
            ("item/inventedKind/delta", UNLISTED_ITEM),
            ("item/tool/call", UNLISTED_ITEM),
            ("item/", UNLISTED_ITEM),
        ] {
            let (p, reference) = active_with_pending();
            let frame = notification(method, json!({"threadId":"thread-1","turnId":"turn-1","itemId":"x"}));
            assert_ends(p, reference, frame, reason);
        }
    }

    #[test]
    fn observation_lines_name_each_distinct_kind_method_and_request_once() {
        let mut p = active();
        p.take_observations();
        for _ in 0..2 {
            p.handle_frame(item_frame("item/completed", json!({"id":"u","type":"userMessage","content":[{"type":"text","text":"SECRET"}]})), &basis()).unwrap();
            p.handle_frame(item_frame("item/started", json!({"id":"r","type":"reasoning"})), &basis()).unwrap();
            p.handle_frame(notification("item/reasoning/textDelta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","delta":"SECRET"})), &basis()).unwrap();
            p.handle_frame(notification("hook/started", json!({"threadId":"thread-1","run":"SECRET"})), &basis()).unwrap();
        }
        p.handle_frame(json!({"id":"approval","method":"item/commandExecution/requestApproval","params":{"command":"SECRET"}}), &basis()).unwrap();
        assert_eq!(
            p.take_observations(),
            [
                "[native-smoke-agent-host] item kind userMessage via item/completed admitted",
                "[native-smoke-agent-host] item kind reasoning via item/started admitted",
                "[native-smoke-agent-host] item kind reasoning via item/reasoning/textDelta admitted",
                "[native-smoke-agent-host] notification item/reasoning/textDelta ignored",
                "[native-smoke-agent-host] notification hook/started ignored",
                "[native-smoke-agent-host] server request item/commandExecution/requestApproval refused",
            ]
        );
        assert!(p.take_observations().is_empty());
        // A refused kind is observed before the session ends, so the caller
        // still prints it.
        let mut q = active();
        q.take_observations();
        assert!(q.handle_frame(item_frame("item/started", json!({"id":"h","type":"hookPrompt"})), &basis()).is_err());
        assert_eq!(
            q.take_observations(),
            ["[native-smoke-agent-host] item kind hookPrompt via item/started refused"]
        );
        let mut r = ready();
        r.submit_text("hello".into()).unwrap();
        r.take_observations();
        assert!(r
            .handle_frame(frame(3, json!({"turn":{"id":"turn-1","status":"inProgress","items":[{"type":"agentMessage"},{"type":"plan"}]}})), &basis())
            .is_err());
        assert_eq!(
            r.take_observations(),
            [
                "[native-smoke-agent-host] item kind agentMessage via turn/start-response admitted",
                "[native-smoke-agent-host] item kind plan via turn/start-response refused",
            ]
        );
    }

    #[test]
    fn hostile_kind_or_method_names_put_no_content_on_the_line() {
        let marker = "SECRET";
        let hostile = [
            format!("agentMessage {marker}"),
            format!("{marker}\" via item/started admitted"),
            format!("{marker}é"),
            format!("{marker}{}", "x".repeat(64)),
            format!("{marker}.kind"),
            format!("{marker}\\n"),
        ];
        let allowed = |line: &str| {
            line.starts_with("[native-smoke-agent-host] ")
                && line.len() <= 26 + 64 * 2 + 40
                && line[26..]
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b" /_-()".contains(&b))
                && !line.contains(marker)
        };
        for name in &hostile {
            let mut p = active();
            p.take_observations();
            let _ = p.handle_frame(item_frame("item/completed", json!({"id":"i","type":name,"text":marker})), &basis());
            assert_eq!(
                p.take_observations(),
                ["[native-smoke-agent-host] item kind (unprintable) via item/completed refused"],
                "{name}"
            );
            let mut q = active();
            q.take_observations();
            q.handle_frame(notification(&format!("thread/{name}"), json!({"threadId":"thread-1","x":marker})), &basis()).unwrap();
            q.handle_frame(json!({"id":"rq","method":format!("x/{name}"),"params":{"x":marker}}), &basis()).unwrap();
            let _ = q.handle_frame(notification(&format!("item/{name}/delta"), json!({"threadId":"thread-1"})), &basis());
            let lines = q.take_observations();
            assert_eq!(
                lines,
                [
                    "[native-smoke-agent-host] notification (unprintable) ignored",
                    "[native-smoke-agent-host] server request (unprintable) refused",
                    "[native-smoke-agent-host] item kind (unprintable) via (unprintable) refused",
                ],
                "{name}"
            );
            assert!(lines.iter().all(|l| allowed(l)), "{lines:?}");
        }
        for (kind, label) in [
            (json!({"type":marker}), "(non-string)"),
            (json!([marker]), "(non-string)"),
            (json!(1), "(non-string)"),
        ] {
            let mut p = active();
            p.take_observations();
            let _ = p.handle_frame(item_frame("item/started", json!({"id":"i","type":kind})), &basis());
            let lines = p.take_observations();
            assert_eq!(lines, [format!("[native-smoke-agent-host] item kind {label} via item/started refused")]);
            assert!(lines.iter().all(|l| allowed(l)));
        }
        let mut p = active();
        p.take_observations();
        let _ = p.handle_frame(item_frame("item/started", json!(marker)), &basis());
        assert_eq!(
            p.take_observations(),
            ["[native-smoke-agent-host] item kind (malformed) via item/started refused"]
        );
    }

    #[test]
    fn observation_lines_are_bounded_per_session() {
        let mut p = active();
        p.take_observations();
        for n in 0..(observed::MAX_LINES * 2) {
            p.handle_frame(notification(&format!("invented/m{n}"), json!({})), &basis()).unwrap();
        }
        let lines = p.take_observations();
        assert_eq!(lines.len(), observed::MAX_LINES + 1);
        assert_eq!(lines.last().unwrap(), "[native-smoke-agent-host] observation limit reached");
        p.handle_frame(notification("invented/more", json!({})), &basis()).unwrap();
        assert!(p.take_observations().is_empty());
    }

    #[test]
    fn dynamic_tool_call_items_must_name_an_offered_tool_without_namespace() {
        // Every tool the catalog offers, with no or a null namespace, passes
        // in each place an item can arrive.
        let mut p = active();
        for tool in Tool::ALL {
            for item in [
                json!({"id":"d","type":"dynamicToolCall","tool":tool.name(),"arguments":{"x":"ignored"},"status":"completed"}),
                json!({"id":"d","type":"dynamicToolCall","tool":tool.name(),"namespace":null}),
            ] {
                for method in ["item/started", "item/completed"] {
                    p.handle_frame(item_frame(method, item.clone()), &basis()).unwrap();
                }
                p.handle_frame(notification("turn/started", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"inProgress","items":[item.clone()]}})), &basis()).unwrap();
            }
        }
        assert!(matches!(
            p.handle_frame(call_summary(json!(7), "valid"), &basis()).unwrap().events.as_slice(),
            [Event::ToolRequested { .. }]
        ));
        let cases = [
            (json!({"id":"d","type":"dynamicToolCall"}), UNLISTED_ITEM),
            (json!({"id":"d","type":"dynamicToolCall","tool":null}), UNLISTED_ITEM),
            (json!({"id":"d","type":"dynamicToolCall","tool":1}), UNLISTED_ITEM),
            (json!({"id":"d","type":"dynamicToolCall","tool":["x"]}), UNLISTED_ITEM),
            (json!({"id":"d","type":"dynamicToolCall","tool":"shell"}), UNOFFERED_ACTIVITY),
            (json!({"id":"d","type":"dynamicToolCall","tool":""}), UNOFFERED_ACTIVITY),
            (json!({"id":"d","type":"dynamicToolCall","tool":Tool::Summary.name().to_uppercase()}), UNOFFERED_ACTIVITY),
            (json!({"id":"d","type":"dynamicToolCall","tool":format!("{} ", Tool::Summary.name())}), UNOFFERED_ACTIVITY),
            (json!({"id":"d","type":"dynamicToolCall","tool":Tool::Summary.name(),"namespace":"swbpipe"}), UNOFFERED_ACTIVITY),
            (json!({"id":"d","type":"dynamicToolCall","tool":Tool::Summary.name(),"namespace":""}), UNOFFERED_ACTIVITY),
            (json!({"id":"d","type":"dynamicToolCall","tool":Tool::Summary.name(),"namespace":{}}), UNOFFERED_ACTIVITY),
        ];
        for (item, reason) in cases {
            let frames = [
                item_frame("item/started", item.clone()),
                item_frame("item/completed", item.clone()),
                notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":[item.clone()]}})),
                notification("thread/started", json!({"thread":{"id":"thread-1","turns":[{"id":"old","status":"completed","items":[item.clone()]}]}})),
            ];
            for frame in frames {
                let (p, reference) = active_with_pending();
                assert_ends(p, reference, frame, reason);
            }
        }
    }

    #[test]
    fn non_object_turns_threads_and_lists_end_the_session() {
        let cmd = json!({"id":"x","type":"commandExecution"});
        let frames = [
            notification("thread/started", json!({"thread":{"id":"t","turns":[[{"items":[cmd.clone()]}]]}})),
            notification("thread/started", json!({"thread":{"id":"t","turns":[null]}})),
            notification("thread/started", json!({"thread":{"id":"t","turns":["turn"]}})),
            notification("thread/started", json!({"thread":{"id":"t","turns":null}})),
            notification("thread/started", json!({"thread":[{"turns":[{"items":[cmd.clone()]}]}]})),
            notification("thread/started", json!({"thread":null})),
            notification("thread/started", json!({"thread":{"id":"t","data":[{"items":[cmd.clone()]}]}})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":[[cmd.clone()]]}})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":[null]}})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":null}})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":[],"data":[cmd.clone()]}})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":null})),
            notification("turn/started", json!({"threadId":"thread-1","turn":[{"items":[cmd.clone()]}]})),
        ];
        for frame in frames {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, frame, UNLISTED_ITEM);
        }
    }

    #[test]
    fn responses_hold_only_the_members_the_schema_defines_for_their_request() {
        let cmd = json!([{"items":[{"type":"commandExecution"}]}]);
        // An interrupt response with its correct identity and an alternate list.
        for result in [json!({"data":cmd.clone()}), json!({"turn":{"id":"turn-1","items":[]}}), json!([]), json!(null)] {
            let (mut p, reference) = active_with_pending();
            assert_eq!(p.interrupt().unwrap()["id"], 4);
            assert_ends(p, reference, frame(4, result), UNLISTED_RESPONSE_MEMBER);
        }
        // The interrupt response the schema defines is still acknowledged.
        let (mut p, _reference) = active_with_pending();
        p.interrupt().unwrap();
        assert!(matches!(
            p.handle_frame(frame(4, json!({})), &basis()).unwrap().events.as_slice(),
            [Event::InterruptAcknowledged]
        ));
        // Initialize, thread/start and turn/start responses precede any call,
        // so no call can be pending; each refusal still ends the session.
        let init = |extra: Value| {
            let mut v = json!({"codexHome":"/private","platformFamily":"test","platformOs":"test","userAgent":"test"});
            v.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
            v
        };
        let mut p = Protocol::new(Catalog::load().unwrap(), "0123456789abcdef".into(), 16).unwrap();
        p.initialize("test", "1").unwrap();
        assert_eq!(
            p.handle_frame(frame(1, init(json!({"data":cmd.clone()}))), &basis()).err(),
            Some(UNLISTED_RESPONSE_MEMBER)
        );
        assert_eq!(p.refusal(), Some(Refusal::Malformed));
        assert!(p.handle_frame(frame(1, init(json!({}))), &basis()).is_err());
        assert!(p.disconnect().is_empty());
        let thread_start = |result: Value| {
            let mut q = Protocol::new(Catalog::load().unwrap(), "0123456789abcdef".into(), 16).unwrap();
            q.initialize("test", "1").unwrap();
            q.handle_frame(frame(1, init(json!({}))), &basis()).unwrap();
            q.start_thread(ThreadOptions {
                model: "test".into(),
                provider: "test".into(),
                cwd: "/test".into(),
                base_instructions: "test".into(),
            })
            .unwrap();
            let outcome = q.handle_frame(frame(2, result), &basis()).err();
            (q, outcome)
        };
        for (result, reason) in [
            (json!({"thread":{"id":"thread-1"},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false},"turns":cmd.clone()}), UNLISTED_RESPONSE_MEMBER),
            (json!({"thread":{"id":"thread-1","data":cmd.clone()},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}}), UNLISTED_ITEM),
            (json!({"thread":{"id":"thread-1","turns":[null]},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}}), UNLISTED_ITEM),
        ] {
            let (mut q, outcome) = thread_start(result);
            assert_eq!(outcome, Some(reason));
            assert!(q.submit_text("next".into()).is_err());
            assert!(q.disconnect().is_empty());
        }
        let (_, accepted) = thread_start(json!({"thread":{"id":"thread-1","turns":[]},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false},"model":"m","cwd":"/test"}));
        assert_eq!(accepted, None);
        for (result, reason) in [
            (json!({"turn":turn("turn-1", "inProgress"),"items":cmd.clone()}), UNLISTED_RESPONSE_MEMBER),
            (json!({"turn":{"id":"turn-1","status":"inProgress","items":[],"data":cmd.clone()}}), UNLISTED_ITEM),
        ] {
            let mut r = ready();
            r.submit_text("hello".into()).unwrap();
            assert_eq!(r.handle_frame(frame(3, result), &basis()).err(), Some(reason));
            assert!(r.handle_frame(call_summary(json!(7), "after"), &basis()).is_err());
            assert!(r.disconnect().is_empty());
        }
    }

    #[test]
    fn refusals_carry_a_closed_payload_free_class() {
        for (frame, class) in [
            (item_frame("item/started", json!({"id":"x","type":"webSearch"})), "runtime_tool_activity"),
            (item_frame("item/started", json!({"id":"x","type":"plan"})), "unadmitted_item"),
            (item_frame("item/started", json!({"id":"x","type":"dynamicToolCall","tool":"shell"})), "runtime_tool_activity"),
            (json!({"id":999,"result":{}}), "malformed_report"),
        ] {
            let mut p = active();
            assert_eq!(p.refusal(), None);
            assert!(p.handle_frame(frame, &basis()).is_err());
            assert_eq!(p.refusal().map(Refusal::class), Some(class));
            // A later frame on the closed protocol does not change the cause.
            assert!(p.handle_frame(json!({"id":1000,"result":{}}), &basis()).is_err());
            assert_eq!(p.refusal().map(Refusal::class), Some(class));
        }
    }

    #[test]
    fn items_in_thread_and_turn_responses_are_checked() {
        let unlisted = json!([{"id":"x","type":"hookPrompt"}]);
        let mut p = ready();
        p.submit_text("hello".into()).unwrap();
        assert_eq!(
            p.handle_frame(frame(3, json!({"turn":{"id":"turn-1","status":"inProgress","items":unlisted}})), &basis())
                .err(),
            Some(UNLISTED_ITEM)
        );
        let mut q = Protocol::new(Catalog::load().unwrap(), "0123456789abcdef".into(), 16).unwrap();
        q.initialize("test", "1").unwrap();
        q.handle_frame(frame(1, json!({"codexHome":"/private","platformFamily":"test","platformOs":"test","userAgent":"test"})), &basis()).unwrap();
        q.start_thread(ThreadOptions {
            model: "test".into(),
            provider: "test".into(),
            cwd: "/test".into(),
            base_instructions: "test".into(),
        })
        .unwrap();
        let thread = json!({"id":"thread-1","turns":[{"id":"old","status":"completed","items":[{"id":"x","type":"webSearch"}]}]});
        assert_eq!(
            q.handle_frame(frame(2, json!({"thread":thread,"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}})), &basis())
                .err(),
            Some(UNOFFERED_ACTIVITY)
        );
    }

    #[test]
    fn handshake_declarations_and_early_turn_identity() {
        let mut p = ready();
        p.submit_text("hello".into()).unwrap();
        let started = p
            .handle_frame(
                notification(
                    "turn/started",
                    json!({"threadId":"thread-1","turn":turn("turn-1", "inProgress")}),
                ),
                &basis(),
            )
            .unwrap();
        assert!(matches!(&started.events[..], [Event::TurnStarted { .. }]));
        let response = p
            .handle_frame(
                frame(3, json!({"turn":turn("turn-1", "inProgress")})),
                &basis(),
            )
            .unwrap();
        assert!(response.events.is_empty());
        assert!(matches!(
            p.handle_frame(call_summary(json!(7), "call-1"), &basis())
                .unwrap()
                .events[0],
            Event::ToolRequested { .. }
        ));
        let mut q = ready();
        q.submit_text("hello".into()).unwrap();
        q.handle_frame(
            notification(
                "turn/started",
                json!({"threadId":"thread-1","turn":turn("turn-1", "inProgress")}),
            ),
            &basis(),
        )
        .unwrap();
        assert!(q
            .handle_frame(
                frame(3, json!({"turn":turn("wrong", "inProgress")})),
                &basis()
            )
            .is_err());
    }
    #[test]
    fn early_call_is_refused_until_start_response_confirms_identity() {
        let mut p = ready();
        p.submit_text("hello".into()).unwrap();
        p.handle_frame(
            notification(
                "turn/started",
                json!({"threadId":"thread-1","turn":turn("turn-1", "inProgress")}),
            ),
            &basis(),
        )
        .unwrap();
        assert!(p
            .handle_frame(call_summary(json!(7), "call-1"), &basis())
            .is_err());
    }
    #[test]
    fn interleaved_calls_route_by_original_id_and_reject_replay() {
        let mut p = active();
        let a = requested(
            p.handle_frame(call_summary(json!("rpc-a"), "a"), &basis())
                .unwrap(),
        );
        let b = requested(
            p.handle_frame(call_summary(json!(8), "b"), &basis())
                .unwrap(),
        );
        let rb = match p.complete(b.as_str(), outcome("unavailable")).unwrap() {
            Completion::Delivered(v) => v,
            _ => panic!(),
        };
        let ra = match p.complete(a.as_str(), outcome("execution_error")).unwrap() {
            Completion::Delivered(v) => v,
            _ => panic!(),
        };
        assert_eq!(rb["id"], 8);
        assert_eq!(rb["result"]["success"], true);
        assert_eq!(ra["id"], "rpc-a");
        assert_eq!(ra["result"]["success"], false);
        assert_eq!(ra["result"]["contentItems"][0]["type"], "inputText");
        assert!(p.complete(a.as_str(), outcome("unavailable")).is_err());
        assert!(p.complete("foreign:1", outcome("unavailable")).is_err());
        assert!(p
            .handle_frame(call_summary(json!(9), "a"), &basis())
            .is_err());
    }
    #[test]
    fn invalid_calls_are_non_executable_and_schema_valid() {
        let mut p = active();
        let a = p
            .handle_frame(
                call(json!(9), "bad", Tool::Summary.name(), json!({"extra":true})),
                &basis(),
            )
            .unwrap();
        assert!(a.events.is_empty());
        assert_eq!(a.outbound[0]["result"]["success"], false);
        let value: Value = serde_json::from_str(
            a.outbound[0]["result"]["contentItems"][0]["text"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
        assert!(p.catalog.validate_output(Tool::Summary, &value).is_ok());
        let unknown = p
            .handle_frame(call(json!(10), "unknown", "unknown", json!({})), &basis())
            .unwrap();
        assert!(unknown.events.is_empty());
        assert_eq!(unknown.outbound[0]["error"]["code"], -32601);
        let mut q = active();
        assert!(q
            .handle_frame(
                call(json!(1), "bad", Tool::Summary.name(), json!({"extra":true})),
                &json!({})
            )
            .is_err());
    }

    #[test]
    fn proposal_context_is_checked_before_dispatch() {
        let mut p = active();
        let hash = json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","payload_ref":"model","value":"a".repeat(64),"hash_status":"computed_local_preview"});
        let operation = json!({"operation_id":"op1","operation_kind":"modify",
            "target":{"object_type":"Node","ref":"n1"},
            "change":{"change_id":"c1","change_kind":"set_field","field_label":"x","field_path":"position.x","before":"0","after":"1","unit":"m","dimension":"length","source_note":"source"},
            "rationale":"why"});
        let args = json!({"workspaceHandle":"workspace-1","basis":basis(),"claimedModelHash":hash,"operations":[operation],"assumptions":[{"assumptionId":"a","statement":"reason","affectedOperationIds":["missing"]}]});
        assert!(p
            .catalog
            .validate_input(Tool::ProposeOperations, &args)
            .is_ok());
        assert!(p.catalog.validate_proposal_context(&args, 16).is_err());
        let refused = p
            .handle_frame(
                call(json!(6), "proposal", Tool::ProposeOperations.name(), args),
                &basis(),
            )
            .unwrap();
        assert!(refused.events.is_empty());
        assert_eq!(refused.outbound[0]["result"]["success"], false);
    }
    #[test]
    fn wrong_thread_turn_and_response_fail_closed() {
        let mut p = active();
        let mut c = call_summary(json!(1), "one");
        c["params"]["threadId"] = json!("wrong");
        assert!(p.handle_frame(c, &basis()).is_err());
        let mut q = active();
        let mut c = call_summary(json!(1), "one");
        c["params"]["turnId"] = json!("old");
        assert!(q.handle_frame(c, &basis()).is_err());
        let mut r = ready();
        r.submit_text("hello".into()).unwrap();
        assert!(r
            .handle_frame(
                frame(100, json!({"turn":turn("turn-1", "inProgress")})),
                &basis()
            )
            .is_err());
    }
    #[test]
    fn interrupt_detaches_without_claiming_cancellation() {
        let mut p = active();
        let reference = requested(
            p.handle_frame(call_summary(json!(1), "call"), &basis())
                .unwrap(),
        );
        assert_eq!(p.interrupt().unwrap()["method"], "turn/interrupt");
        let events = p
            .handle_frame(
                notification(
                    "turn/completed",
                    json!({"threadId":"thread-1","turn":turn("turn-1", "interrupted")}),
                ),
                &basis(),
            )
            .unwrap()
            .events;
        assert!(matches!(
            &events[..],
            [Event::ToolDetached { .. }, Event::TurnCompleted { .. }]
        ));
        assert!(matches!(
            p.complete(reference.as_str(), outcome("unavailable"))
                .unwrap(),
            Completion::Detached
        ));
        p.handle_frame(frame(4, json!({})), &basis()).unwrap();
        assert!(p.submit_text("next".into()).is_ok());
    }
    #[test]
    fn disconnect_invalidates_without_retry() {
        let mut p = active();
        let reference = requested(
            p.handle_frame(call_summary(json!(1), "call"), &basis())
                .unwrap(),
        );
        assert!(matches!(&p.disconnect()[..], [Event::ToolLost { .. }]));
        assert!(p.disconnect().is_empty());
        assert!(p
            .complete(reference.as_str(), outcome("unavailable"))
            .is_err());
    }
    #[test]
    fn pending_and_text_limits_are_explicit() {
        let mut p = active();
        for n in 0..p.catalog.limits().pending_calls {
            p.handle_frame(call_summary(json!(n), &format!("call-{n}")), &basis())
                .unwrap();
        }
        let excess = p
            .handle_frame(call_summary(json!(100), "excess"), &basis())
            .unwrap();
        assert!(excess.events.is_empty());
        assert_eq!(excess.outbound[0]["result"]["success"], false);
        let mut q = active();
        assert!(q.handle_frame(notification("item/agentMessage/delta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"message","delta":"x".repeat(MAX_TEXT_BYTES + 1)})), &basis()).is_err());
        let mut r = ready();
        assert!(r.submit_text("x".repeat(MAX_TEXT_BYTES + 1)).is_err());
    }
    #[test]
    fn outcome_validation_preserves_pending_and_historical_status() {
        let mut p = active();
        let reference = requested(
            p.handle_frame(call_summary(json!(1), "call"), &basis())
                .unwrap(),
        );
        assert!(p
            .complete(reference.as_str(), json!({"status":"ok"}))
            .is_err());
        assert!(matches!(
            p.complete(reference.as_str(), outcome("unavailable"))
                .unwrap(),
            Completion::Delivered(_)
        ));
    }

    #[test]
    fn historical_and_nonexecution_statuses_are_not_relabelled() {
        let mut p = active();
        for (n, status) in ["rejected", "withdrawn", "stale_basis", "unknown", "applied"]
            .into_iter()
            .enumerate()
        {
            let args = json!({"workspaceHandle":"workspace-1","basis":basis(),"proposalHandle":"proposal-1"});
            let reference = requested(
                p.handle_frame(
                    call(
                        json!(n),
                        &format!("call-{n}"),
                        Tool::ObserveProposal.name(),
                        args,
                    ),
                    &basis(),
                )
                .unwrap(),
            );
            let response = match p.complete(reference.as_str(), outcome(status)).unwrap() {
                Completion::Delivered(v) => v,
                _ => panic!("unexpected detachment"),
            };
            assert_eq!(response["result"]["success"], true);
            let delivered: Value = serde_json::from_str(
                response["result"]["contentItems"][0]["text"]
                    .as_str()
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(delivered["status"], status);
        }
    }

    #[test]
    fn transport_identity_guards_refuse_excess_and_replays() {
        let mut p = active();
        let reference = requested(
            p.handle_frame(call_summary(json!(1), "first"), &basis())
                .unwrap(),
        );
        p.complete(reference.as_str(), outcome("unavailable"))
            .unwrap();
        assert!(p
            .handle_frame(call_summary(json!(1), "second"), &basis())
            .is_err());
        let mut q = active();
        assert!(q
            .handle_frame(
                call_summary(json!(2), &"x".repeat(MAX_ID_BYTES + 1)),
                &basis()
            )
            .is_err());
        let mut r = active();
        for n in 0..MAX_CALLS_PER_TURN {
            let response = r
                .handle_frame(call_summary(json!(n), &format!("call-{n}")), &basis())
                .unwrap();
            let reference = requested(response);
            r.complete(reference.as_str(), outcome("unavailable"))
                .unwrap();
        }
        assert!(r
            .handle_frame(call_summary(json!(MAX_CALLS_PER_TURN), "excess"), &basis())
            .is_err());
    }
}
