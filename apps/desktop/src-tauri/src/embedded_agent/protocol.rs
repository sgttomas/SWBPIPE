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
fn bounded_identity(s: &str) -> Result<String, Error> {
    if bounded(s, MAX_ID_BYTES) {
        Ok(s.to_owned())
    } else {
        Err(Error("invalid identity"))
    }
}
fn rpc_id(id: &wire::RequestId) -> Result<Value, Error> {
    match id {
        wire::RequestId::Int(n) => Ok(json!(n)),
        wire::RequestId::Str(s) if bounded(s, MAX_ID_BYTES) => Ok(json!(s)),
        wire::RequestId::Str(_) => Err(Error("invalid request identity")),
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

/// Closed types for every frame the host reads from stock 0.160.0's app
/// server, from its protocol schema (`codex_app_server_protocol.schemas.json`,
/// SHA-256 7243ba241962af92ca60581f1a81808ebda4212a800f8b205f54703bcfd508c5).
/// Each type names every member the schema gives it, with its type; required
/// members are required, a required nullable member must be present, an
/// optional member that is not nullable refuses null, and any member the
/// schema does not give is refused. Members the schema types as any JSON
/// (`arguments`, `params`, `result`, error `data`) are the only `Value`s.
/// The refused item kinds end the session on their tag; their bodies are
/// skipped unread, which bounds this module to the admitted kinds' types.
#[allow(dead_code)]
pub(super) mod wire {
    use serde::de::{Deserializer, IgnoredAny};
    use serde::Deserialize;
    use serde_json::Value;

    /// A required member whose schema type admits null: present, maybe null.
    fn nullable<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Option<T>, D::Error> {
        Option::<T>::deserialize(d)
    }
    /// An optional member whose schema type does not admit null.
    fn some<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Option<T>, D::Error> {
        T::deserialize(d).map(Some)
    }

    // JSON-RPC envelope (JSONRPCRequest, JSONRPCNotification,
    // JSONRPCResponse, JSONRPCError): the union of their members; which
    // combination arrived is checked by the caller.
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Envelope {
        #[serde(default, deserialize_with = "some")]
        pub id: Option<RequestId>,
        #[serde(default, deserialize_with = "some")]
        pub method: Option<String>,
        #[serde(default)]
        pub params: Option<Value>,
        #[serde(default)]
        pub result: Option<Value>,
        #[serde(default, deserialize_with = "some")]
        pub error: Option<JsonRpcError>,
        #[serde(default, deserialize_with = "some")]
        pub trace: Option<W3cTraceContext>,
    }
    #[derive(Deserialize, Clone, Debug, PartialEq)]
    #[serde(untagged)]
    pub enum RequestId {
        Int(i64),
        Str(String),
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct JsonRpcError {
        pub code: i64,
        pub message: String,
        #[serde(default)]
        pub data: Option<Value>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct W3cTraceContext {
        #[serde(default)]
        pub traceparent: Option<String>,
        #[serde(default)]
        pub tracestate: Option<String>,
    }

    // The tool call request (DynamicToolCallParams).
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct DynamicToolCallParams {
        pub arguments: Value,
        pub call_id: String,
        #[serde(default)]
        pub namespace: Option<String>,
        pub thread_id: String,
        pub tool: String,
        pub turn_id: String,
    }

    // Responses to the four requests the host sends.
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct InitializeResponse {
        pub codex_home: String,
        pub platform_family: String,
        pub platform_os: String,
        pub user_agent: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ThreadStartResponse {
        #[serde(default)]
        pub active_permission_profile: Option<ActivePermissionProfile>,
        pub approval_policy: AskForApproval,
        pub approvals_reviewer: ApprovalsReviewer,
        pub cwd: String,
        #[serde(default, deserialize_with = "some")]
        pub disabled_plugin_ids: Option<Vec<String>>,
        #[serde(default, deserialize_with = "some")]
        pub instruction_sources: Option<Vec<String>>,
        pub model: String,
        pub model_provider: String,
        #[serde(default, deserialize_with = "some")]
        pub multi_agent_mode: Option<MultiAgentMode>,
        #[serde(default)]
        pub reasoning_effort: Option<String>,
        #[serde(default, deserialize_with = "some")]
        pub runtime_workspace_roots: Option<Vec<String>>,
        pub sandbox: SandboxPolicy,
        #[serde(default)]
        pub service_tier: Option<String>,
        pub thread: Thread,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TurnStartResponse {
        pub turn: Turn,
    }
    /// An object the schema gives no members: only `{}` is accepted, not
    /// an array or any member.
    pub struct EmptyObject;
    impl<'de> Deserialize<'de> for EmptyObject {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = EmptyObject;
                fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str("an empty object")
                }
                fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<EmptyObject, A::Error> {
                    match map.next_key::<IgnoredAny>()? {
                        None => Ok(EmptyObject),
                        Some(_) => Err(serde::de::Error::custom("member of an empty object")),
                    }
                }
            }
            d.deserialize_map(Visitor)
        }
    }
    pub type TurnInterruptResponse = EmptyObject;

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ActivePermissionProfile {
        #[serde(default)]
        pub extends: Option<String>,
        pub id: String,
    }
    #[derive(Deserialize, PartialEq)]
    pub enum ApprovalsReviewer {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "auto_review")]
        AutoReview,
        #[serde(rename = "guardian_subagent")]
        GuardianSubagent,
    }
    #[derive(Deserialize, PartialEq)]
    #[serde(untagged)]
    pub enum AskForApproval {
        Mode(ApprovalMode),
        Granular(GranularApproval),
    }
    #[derive(Deserialize, PartialEq)]
    pub enum ApprovalMode {
        #[serde(rename = "untrusted")]
        Untrusted,
        #[serde(rename = "on-request")]
        OnRequest,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Deserialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct GranularApproval {
        pub granular: Granular,
    }
    #[derive(Deserialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Granular {
        pub mcp_elicitations: bool,
        #[serde(default, deserialize_with = "some")]
        pub request_permissions: Option<bool>,
        pub rules: bool,
        pub sandbox_approval: bool,
        #[serde(default, deserialize_with = "some")]
        pub skill_approval: Option<bool>,
    }
    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum MultiAgentMode {
        Mode(MultiAgentModeName),
        Custom(Custom),
    }
    #[derive(Deserialize)]
    pub enum MultiAgentModeName {
        #[serde(rename = "explicitRequestOnly")]
        ExplicitRequestOnly,
        #[serde(rename = "proactive")]
        Proactive,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Custom {
        pub custom: String,
    }
    #[derive(Deserialize)]
    #[serde(tag = "type", deny_unknown_fields)]
    pub enum SandboxPolicy {
        #[serde(rename = "dangerFullAccess")]
        DangerFullAccess {},
        #[serde(rename = "readOnly", rename_all = "camelCase")]
        ReadOnly {
            #[serde(default, deserialize_with = "some")]
            network_access: Option<bool>,
        },
        #[serde(rename = "externalSandbox", rename_all = "camelCase")]
        ExternalSandbox {
            #[serde(default, deserialize_with = "some")]
            network_access: Option<NetworkAccess>,
        },
        #[serde(rename = "workspaceWrite", rename_all = "camelCase")]
        WorkspaceWrite {
            #[serde(default, deserialize_with = "some")]
            exclude_slash_tmp: Option<bool>,
            #[serde(default, deserialize_with = "some")]
            exclude_tmpdir_env_var: Option<bool>,
            #[serde(default, deserialize_with = "some")]
            network_access: Option<bool>,
            #[serde(default, deserialize_with = "some")]
            writable_roots: Option<Vec<String>>,
        },
    }
    #[derive(Deserialize)]
    pub enum NetworkAccess {
        #[serde(rename = "restricted")]
        Restricted,
        #[serde(rename = "enabled")]
        Enabled,
    }

    // Thread and turn.
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct Thread {
        #[serde(default)]
        pub agent_nickname: Option<String>,
        #[serde(default)]
        pub agent_role: Option<String>,
        #[serde(default)]
        pub can_accept_direct_input: Option<bool>,
        pub cli_version: String,
        pub created_at: i64,
        pub cwd: String,
        #[serde(default)]
        pub daybreak_enabled: Option<bool>,
        #[serde(default)]
        pub environments: Option<Vec<ThreadEnvironment>>,
        pub ephemeral: bool,
        #[serde(default)]
        pub extra: Option<ThreadExtra>,
        #[serde(default)]
        pub forked_from_id: Option<String>,
        #[serde(default)]
        pub git_info: Option<GitInfo>,
        #[serde(default, deserialize_with = "some")]
        pub history_mode: Option<ThreadHistoryMode>,
        pub id: String,
        #[serde(default)]
        pub model: Option<String>,
        pub model_provider: String,
        #[serde(default)]
        pub name: Option<String>,
        #[serde(default)]
        pub originator: Option<String>,
        #[serde(default)]
        pub parent_thread_id: Option<String>,
        #[serde(default)]
        pub path: Option<String>,
        pub preview: String,
        #[serde(deserialize_with = "nullable")]
        pub project_id: Option<String>,
        #[serde(default)]
        pub reasoning_effort: Option<String>,
        #[serde(default)]
        pub recency_at: Option<i64>,
        #[serde(default)]
        pub section: Option<ThreadSection>,
        #[serde(default)]
        pub section_entered_at: Option<i64>,
        pub session_id: String,
        pub source: SessionSource,
        pub status: ThreadStatus,
        #[serde(default)]
        pub thread_source: Option<String>,
        pub turns: Vec<Turn>,
        pub updated_at: i64,
    }
    /// The schema gives this object no members, so only `{}` is accepted.
    pub type ThreadExtra = EmptyObject;
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ThreadEnvironment {
        pub cwd: String,
        pub environment_id: String,
        pub runtime_workspace_roots: Vec<String>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct GitInfo {
        #[serde(default)]
        pub branch: Option<String>,
        #[serde(default)]
        pub origin_url: Option<String>,
        #[serde(default)]
        pub sha: Option<String>,
    }
    #[derive(Deserialize)]
    pub enum ThreadHistoryMode {
        #[serde(rename = "legacy")]
        Legacy,
        #[serde(rename = "paginated")]
        Paginated,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ThreadSection {
        #[serde(default)]
        pub appearance: Option<ThreadSectionAppearance>,
        pub id: String,
        pub name: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ThreadSectionAppearance {
        #[serde(default)]
        pub color: Option<String>,
        #[serde(default)]
        pub icon: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum SessionSource {
        Name(SessionSourceName),
        Custom(Custom),
        SubAgent(SubAgentSessionSource),
    }
    #[derive(Deserialize)]
    pub enum SessionSourceName {
        #[serde(rename = "cli")]
        Cli,
        #[serde(rename = "vscode")]
        Vscode,
        #[serde(rename = "exec")]
        Exec,
        #[serde(rename = "appServer")]
        AppServer,
        #[serde(rename = "unknown")]
        Unknown,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct SubAgentSessionSource {
        pub sub_agent: SubAgentSource,
    }
    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum SubAgentSource {
        Name(SubAgentSourceName),
        ThreadSpawn(ThreadSpawnSource),
        Other(OtherSubAgentSource),
    }
    #[derive(Deserialize)]
    pub enum SubAgentSourceName {
        #[serde(rename = "review")]
        Review,
        #[serde(rename = "compact")]
        Compact,
        #[serde(rename = "memory_consolidation")]
        MemoryConsolidation,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ThreadSpawnSource {
        pub thread_spawn: ThreadSpawn,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ThreadSpawn {
        #[serde(default)]
        pub agent_nickname: Option<String>,
        #[serde(default)]
        pub agent_path: Option<String>,
        #[serde(default)]
        pub agent_role: Option<String>,
        pub depth: i32,
        pub parent_thread_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct OtherSubAgentSource {
        pub other: String,
    }
    #[derive(Deserialize)]
    #[serde(tag = "type", deny_unknown_fields)]
    pub enum ThreadStatus {
        #[serde(rename = "notLoaded")]
        NotLoaded {},
        #[serde(rename = "idle")]
        Idle {},
        #[serde(rename = "systemError")]
        SystemError {},
        #[serde(rename = "active", rename_all = "camelCase")]
        Active { active_flags: Vec<ThreadActiveFlag> },
    }
    impl ThreadStatus {
        pub fn name(&self) -> &'static str {
            match self {
                Self::NotLoaded {} => "notLoaded",
                Self::Idle {} => "idle",
                Self::SystemError {} => "systemError",
                Self::Active { .. } => "active",
            }
        }
    }
    #[derive(Deserialize)]
    pub enum ThreadActiveFlag {
        #[serde(rename = "waitingOnApproval")]
        WaitingOnApproval,
        #[serde(rename = "waitingOnUserInput")]
        WaitingOnUserInput,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct Turn {
        #[serde(default)]
        pub completed_at: Option<i64>,
        #[serde(default)]
        pub duration_ms: Option<i64>,
        #[serde(default)]
        pub error: Option<TurnError>,
        pub id: String,
        pub items: Vec<Item>,
        #[serde(default, deserialize_with = "some")]
        pub items_view: Option<TurnItemsView>,
        #[serde(default)]
        pub started_at: Option<i64>,
        pub status: TurnStatus,
    }
    #[derive(Deserialize, Clone, Copy, PartialEq)]
    pub enum TurnStatus {
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "interrupted")]
        Interrupted,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "inProgress")]
        InProgress,
    }
    impl TurnStatus {
        pub fn name(self) -> &'static str {
            match self {
                Self::Completed => "completed",
                Self::Interrupted => "interrupted",
                Self::Failed => "failed",
                Self::InProgress => "inProgress",
            }
        }
    }
    #[derive(Deserialize)]
    pub enum TurnItemsView {
        #[serde(rename = "notLoaded")]
        NotLoaded,
        #[serde(rename = "summary")]
        Summary,
        #[serde(rename = "full")]
        Full,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct TurnError {
        #[serde(default)]
        pub additional_details: Option<String>,
        #[serde(default)]
        pub codex_error_info: Option<CodexErrorInfo>,
        pub message: String,
        #[serde(default)]
        pub misalignment: Option<MisalignmentErrorDetails>,
    }
    #[derive(Deserialize)]
    #[serde(untagged)]
    pub enum CodexErrorInfo {
        Code(CodexErrorCode),
        HttpConnectionFailed(HttpConnectionFailed),
        ResponseStreamConnectionFailed(ResponseStreamConnectionFailed),
        ResponseStreamDisconnected(ResponseStreamDisconnected),
        ResponseTooManyFailedAttempts(ResponseTooManyFailedAttempts),
        ActiveTurnNotSteerable(ActiveTurnNotSteerable),
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum CodexErrorCode {
        ContextWindowExceeded,
        SessionBudgetExceeded,
        UsageLimitExceeded,
        RateLimitExceeded,
        FlexUnavailable,
        ServerOverloaded,
        CyberPolicy,
        MisalignmentPolicyViolation,
        TooManyDenials,
        InternalServerError,
        Unauthorized,
        BadRequest,
        ThreadRollbackFailed,
        SandboxError,
        Other,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct HttpStatus {
        #[serde(default)]
        pub http_status_code: Option<u16>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct HttpConnectionFailed {
        pub http_connection_failed: HttpStatus,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ResponseStreamConnectionFailed {
        pub response_stream_connection_failed: HttpStatus,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ResponseStreamDisconnected {
        pub response_stream_disconnected: HttpStatus,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ResponseTooManyFailedAttempts {
        pub response_too_many_failed_attempts: HttpStatus,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ActiveTurnNotSteerable {
        pub active_turn_not_steerable: NotSteerable,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct NotSteerable {
        pub turn_kind: NonSteerableTurnKind,
    }
    #[derive(Deserialize)]
    pub enum NonSteerableTurnKind {
        #[serde(rename = "review")]
        Review,
        #[serde(rename = "compact")]
        Compact,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct MisalignmentErrorDetails {
        #[serde(default)]
        pub detailed_explanation: Option<String>,
        #[serde(default)]
        pub error_type: Option<String>,
        #[serde(default)]
        pub steer: Option<MisalignmentSteer>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MisalignmentSteer {
        pub message: String,
    }

    /// A thread item. The five admitted kinds are read in full; the other
    /// schema kinds are recognised by their tag and their bodies skipped,
    /// since the host ends the session on them; any other string kind is
    /// `Unlisted`. A missing or non-string kind fails to deserialize.
    #[derive(Deserialize)]
    #[serde(tag = "type", deny_unknown_fields)]
    pub enum Item {
        #[serde(rename = "userMessage", rename_all = "camelCase")]
        UserMessage {
            #[serde(default)]
            client_id: Option<String>,
            content: Vec<UserInput>,
            id: String,
        },
        #[serde(rename = "agentMessage", rename_all = "camelCase")]
        AgentMessage {
            #[serde(default)]
            delivery: Option<AgentMessageDelivery>,
            id: String,
            #[serde(default)]
            memory_citation: Option<MemoryCitation>,
            #[serde(default)]
            phase: Option<MessagePhase>,
            #[serde(default)]
            questions: Option<Vec<AsyncUserInputQuestion>>,
            text: String,
        },
        #[serde(rename = "reasoning")]
        Reasoning {
            #[serde(default)]
            content: Vec<String>,
            id: String,
            #[serde(default)]
            summary: Vec<String>,
        },
        #[serde(rename = "contextCompaction")]
        ContextCompaction { id: String },
        #[serde(rename = "dynamicToolCall", rename_all = "camelCase")]
        DynamicToolCall {
            arguments: Value,
            #[serde(default)]
            content_items: Option<Vec<DynamicToolCallOutputContentItem>>,
            #[serde(default)]
            duration_ms: Option<i64>,
            id: String,
            #[serde(default)]
            namespace: Option<String>,
            status: DynamicToolCallStatus,
            #[serde(default)]
            success: Option<bool>,
            tool: String,
        },
        #[serde(rename = "commandExecution")]
        CommandExecution(IgnoredAny),
        #[serde(rename = "fileChange")]
        FileChange(IgnoredAny),
        #[serde(rename = "mcpToolCall")]
        McpToolCall(IgnoredAny),
        #[serde(rename = "collabAgentToolCall")]
        CollabAgentToolCall(IgnoredAny),
        #[serde(rename = "subAgentActivity")]
        SubAgentActivity(IgnoredAny),
        #[serde(rename = "webSearch")]
        WebSearch(IgnoredAny),
        #[serde(rename = "imageView")]
        ImageView(IgnoredAny),
        #[serde(rename = "sleep")]
        Sleep(IgnoredAny),
        #[serde(rename = "imageGeneration")]
        ImageGeneration(IgnoredAny),
        #[serde(rename = "hookPrompt")]
        HookPrompt(IgnoredAny),
        #[serde(rename = "plan")]
        Plan(IgnoredAny),
        #[serde(rename = "enteredReviewMode")]
        EnteredReviewMode(IgnoredAny),
        #[serde(rename = "exitedReviewMode")]
        ExitedReviewMode(IgnoredAny),
        #[serde(rename = "functionCallOutput")]
        FunctionCallOutput(IgnoredAny),
        #[serde(other)]
        Unlisted,
    }
    #[derive(Deserialize)]
    #[serde(tag = "type", deny_unknown_fields)]
    pub enum UserInput {
        #[serde(rename = "text")]
        Text {
            text: String,
            #[serde(default)]
            text_elements: Vec<TextElement>,
        },
        #[serde(rename = "image", rename_all = "camelCase")]
        Image {
            #[serde(default)]
            detail: Option<ImageDetail>,
            #[serde(default, deserialize_with = "some")]
            url: Option<String>,
            #[serde(default, deserialize_with = "some")]
            file_id: Option<String>,
        },
        #[serde(rename = "localImage")]
        LocalImage {
            #[serde(default)]
            detail: Option<ImageDetail>,
            path: String,
        },
        #[serde(rename = "audio")]
        Audio { url: String },
        #[serde(rename = "localAudio")]
        LocalAudio { path: String },
        #[serde(rename = "skill")]
        Skill { name: String, path: String },
        #[serde(rename = "mention")]
        Mention { name: String, path: String },
    }
    #[derive(Deserialize)]
    pub enum ImageDetail {
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "low")]
        Low,
        #[serde(rename = "high")]
        High,
        #[serde(rename = "original")]
        Original,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct TextElement {
        pub byte_range: ByteRange,
        #[serde(default)]
        pub placeholder: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ByteRange {
        pub end: u64,
        pub start: u64,
    }
    #[derive(Deserialize)]
    pub enum AgentMessageDelivery {
        #[serde(rename = "async")]
        Async,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct MemoryCitation {
        pub entries: Vec<MemoryCitationEntry>,
        pub thread_ids: Vec<String>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct MemoryCitationEntry {
        pub line_end: u32,
        pub line_start: u32,
        pub note: String,
        pub path: String,
    }
    #[derive(Deserialize)]
    pub enum MessagePhase {
        #[serde(rename = "commentary")]
        Commentary,
        #[serde(rename = "final_answer")]
        FinalAnswer,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct AsyncUserInputQuestion {
        #[serde(default)]
        pub options: Option<Vec<String>>,
        pub title: String,
    }
    #[derive(Deserialize)]
    #[serde(tag = "type", deny_unknown_fields)]
    pub enum DynamicToolCallOutputContentItem {
        #[serde(rename = "inputText")]
        InputText { text: String },
        #[serde(rename = "inputImage", rename_all = "camelCase")]
        InputImage { image_url: String },
        #[serde(rename = "inputAudio", rename_all = "camelCase")]
        InputAudio { audio_url: String },
    }
    #[derive(Deserialize)]
    pub enum DynamicToolCallStatus {
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
    }

    // Parameters of the notifications the host handles.
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ThreadStartedParams {
        pub thread: Thread,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ThreadStatusChangedParams {
        pub status: ThreadStatus,
        pub thread_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct TurnParams {
        pub thread_id: String,
        pub turn: Turn,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ItemStartedParams {
        pub item: Item,
        pub started_at_ms: i64,
        pub thread_id: String,
        pub turn_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ItemCompletedParams {
        pub completed_at_ms: i64,
        pub item: Item,
        pub thread_id: String,
        pub turn_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct AgentMessageDeltaParams {
        pub delta: String,
        pub item_id: String,
        pub thread_id: String,
        pub turn_id: String,
    }

    // Parameters of the notifications the host ignores by name. Each is read
    // in full, so no item, turn or thread can ride in one.
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct AccountRateLimitsUpdatedParams {
        pub rate_limits: RateLimitSnapshot,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct RateLimitSnapshot {
        #[serde(default)]
        pub credits: Option<CreditsSnapshot>,
        #[serde(default)]
        pub individual_limit: Option<SpendControlLimitSnapshot>,
        #[serde(default)]
        pub limit_id: Option<String>,
        #[serde(default)]
        pub limit_name: Option<String>,
        #[serde(default)]
        pub normal_model_slug: Option<String>,
        #[serde(default)]
        pub plan_type: Option<PlanType>,
        #[serde(default)]
        pub primary: Option<RateLimitWindow>,
        #[serde(default)]
        pub rate_limit_reached_type: Option<RateLimitReachedType>,
        #[serde(default)]
        pub secondary: Option<RateLimitWindow>,
        #[serde(default)]
        pub spend_control_reached: Option<bool>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct CreditsSnapshot {
        #[serde(default)]
        pub balance: Option<String>,
        pub has_credits: bool,
        pub unlimited: bool,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct SpendControlLimitSnapshot {
        pub limit: String,
        pub remaining_percent: i32,
        pub resets_at: i64,
        pub used: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct RateLimitWindow {
        #[serde(default)]
        pub resets_at: Option<i64>,
        pub used_percent: i32,
        #[serde(default)]
        pub window_duration_mins: Option<i64>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum PlanType {
        Free,
        Go,
        Plus,
        Pro,
        Prolite,
        Promax,
        Team,
        SelfServeBusinessProlite,
        SelfServeBusinessUsageBased,
        Business,
        Ent26,
        EnterpriseCbpAutomation,
        EnterpriseCbpUsageBased,
        Enterprise,
        Edu,
        EduPlus,
        EduPro,
        Unknown,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum RateLimitReachedType {
        RateLimitReached,
        WorkspaceOwnerCreditsDepleted,
        WorkspaceMemberCreditsDepleted,
        WorkspaceOwnerUsageLimitReached,
        WorkspaceMemberUsageLimitReached,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ThreadTokenUsageUpdatedParams {
        pub thread_id: String,
        pub token_usage: ThreadTokenUsage,
        pub turn_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ThreadTokenUsage {
        pub last: TokenUsageBreakdown,
        #[serde(default)]
        pub model_context_window: Option<i64>,
        pub total: TokenUsageBreakdown,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct TokenUsageBreakdown {
        #[serde(default, deserialize_with = "some")]
        pub cache_write_input_tokens: Option<i64>,
        pub cached_input_tokens: i64,
        pub input_tokens: i64,
        pub output_tokens: i64,
        pub reasoning_output_tokens: i64,
        pub total_tokens: i64,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct WarningParams {
        pub message: String,
        #[serde(default)]
        pub thread_id: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct RemoteControlStatusChangedParams {
        #[serde(default)]
        pub environment_id: Option<String>,
        pub installation_id: String,
        pub server_name: String,
        pub status: RemoteControlConnectionStatus,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum RemoteControlConnectionStatus {
        Disabled,
        Connecting,
        Connected,
        Errored,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ReasoningSummaryTextDeltaParams {
        pub delta: String,
        pub item_id: String,
        pub summary_index: i64,
        pub thread_id: String,
        pub turn_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ReasoningSummaryPartAddedParams {
        pub item_id: String,
        pub summary_index: i64,
        pub thread_id: String,
        pub turn_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ReasoningTextDeltaParams {
        pub content_index: i64,
        pub delta: String,
        pub item_id: String,
        pub thread_id: String,
        pub turn_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ContextCompactedParams {
        pub thread_id: String,
        pub turn_id: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ErrorParams {
        pub error: TurnError,
        pub thread_id: String,
        pub turn_id: String,
        pub will_retry: bool,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ConfigWarningParams {
        #[serde(default)]
        pub details: Option<String>,
        #[serde(default)]
        pub path: Option<String>,
        #[serde(default)]
        pub range: Option<TextRange>,
        pub summary: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TextRange {
        pub end: TextPosition,
        pub start: TextPosition,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TextPosition {
        pub column: u64,
        pub line: u64,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct DeprecationNoticeParams {
        #[serde(default)]
        pub details: Option<String>,
        pub summary: String,
    }
}
use wire::Item;

const UNOFFERED_ACTIVITY: Error = Error("unoffered supplier tool activity");
const UNLISTED_ITEM: Error = Error("unlisted supplier item kind");
const UNADMITTED_NOTIFICATION: Error = Error("unadmitted supplier notification");
const UNADMITTED_REQUEST: Error = Error("unadmitted supplier request");
const MALFORMED: Error = Error("malformed supplier report");

/// Read any part of a frame into its closed type; any failure is a
/// malformed report.
fn typed<T: serde::de::DeserializeOwned>(value: Value) -> Result<T, Error> {
    serde_json::from_value(value).map_err(|_| MALFORMED)
}
/// The same, reading from a borrowed value.
fn typed_ref<'a, T: serde::Deserialize<'a>>(value: &'a Value) -> Result<T, Error> {
    T::deserialize(value).map_err(|_| MALFORMED)
}

/// Why the protocol ended a session: a closed, payload-free class that the
/// panel turns into plain words. Never supplier text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Tool activity this host does not admit: the runtime's own tools, a tool
    /// the catalog does not offer, or a namespaced tool.
    ToolActivity,
    /// An item kind, notification or request this host does not admit.
    Unadmitted,
    /// A frame or report the host could not read into its closed type, or one
    /// out of order.
    Malformed,
}
impl Refusal {
    pub fn of(error: Error) -> Self {
        if error == UNOFFERED_ACTIVITY {
            Self::ToolActivity
        } else if error == UNLISTED_ITEM
            || error == UNADMITTED_NOTIFICATION
            || error == UNADMITTED_REQUEST
        {
            Self::Unadmitted
        } else {
            Self::Malformed
        }
    }
    pub fn class(self) -> &'static str {
        match self {
            Self::ToolActivity => "runtime_tool_activity",
            Self::Unadmitted => "unadmitted_report",
            Self::Malformed => "malformed_report",
        }
    }
}

/// The 19 item kinds of stock 0.160.0's ThreadItem schema. Five are admitted;
/// see `admit`. Kept here so an observation line names only these.
#[cfg(any(test, feature = "native-smoke"))]
pub(super) const SCHEMA_ITEM_KINDS: [&str; 19] = [
    "userMessage", "hookPrompt", "agentMessage", "functionCallOutput", "plan", "reasoning",
    "commandExecution", "fileChange", "mcpToolCall", "dynamicToolCall", "collabAgentToolCall",
    "subAgentActivity", "webSearch", "imageView", "sleep", "imageGeneration",
    "enteredReviewMode", "exitedReviewMode", "contextCompaction",
];

/// Admission of a typed item. Admitted: the agent's message, the user's
/// message, reasoning, context compaction, and a call of a tool the catalog
/// offers with no namespace. The runtime's own tool kinds are tool activity;
/// every other kind (hookPrompt, plan, the review modes, functionCallOutput,
/// and any kind the schema does not name) is unadmitted. This is an
/// observation tripwire, not proof that the operating system's isolation
/// prevented an action.
fn admit(item: &Item) -> Result<(), Error> {
    match item {
        Item::UserMessage { content, .. } => {
            // ImageUserInput requires `url` or `fileId` (anyOf).
            if content.iter().any(|input| {
                matches!(input, wire::UserInput::Image { url: None, file_id: None, .. })
            }) {
                return Err(MALFORMED);
            }
            Ok(())
        }
        Item::AgentMessage { .. } | Item::Reasoning { .. } | Item::ContextCompaction { .. } => Ok(()),
        Item::DynamicToolCall { tool, namespace, .. } => {
            if Tool::from_name(tool).is_none() || namespace.is_some() {
                Err(UNOFFERED_ACTIVITY)
            } else {
                Ok(())
            }
        }
        Item::CommandExecution(_)
        | Item::FileChange(_)
        | Item::McpToolCall(_)
        | Item::CollabAgentToolCall(_)
        | Item::SubAgentActivity(_)
        | Item::WebSearch(_)
        | Item::ImageView(_)
        | Item::Sleep(_)
        | Item::ImageGeneration(_) => Err(UNOFFERED_ACTIVITY),
        Item::HookPrompt(_)
        | Item::Plan(_)
        | Item::EnteredReviewMode(_)
        | Item::ExitedReviewMode(_)
        | Item::FunctionCallOutput(_)
        | Item::Unlisted => Err(UNLISTED_ITEM),
    }
}
fn admit_turn(turn: &wire::Turn) -> Result<(), Error> {
    turn.items.iter().try_for_each(admit)
}
fn admit_thread(thread: &wire::Thread) -> Result<(), Error> {
    thread.turns.iter().try_for_each(admit_turn)
}

/// What the host does with a notification method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Notice {
    /// Read into its closed type and acted on.
    Handled,
    /// Read into its closed type, which carries no item, turn or thread, and
    /// otherwise ignored.
    Ignored,
    /// Ends the session with this reason.
    Ends(Error),
}
const TOOL: Notice = Notice::Ends(UNOFFERED_ACTIVITY);
const ENDS: Notice = Notice::Ends(UNADMITTED_NOTIFICATION);

/// Every ServerNotification method of stock 0.160.0's schema, and what the
/// host does with it. This table is the single source; a method absent from
/// it ends the session. Ignored methods are those an ordinary session sends
/// (observed: account/rateLimits/updated, thread/tokenUsage/updated, warning,
/// remoteControl/status/changed) or that belong to an admitted kind's
/// conversation (reasoning deltas, the deprecated thread/compacted, a turn's
/// error, configuration and deprecation notices); none carries an item, turn
/// or thread. Every other method ends the session: the runtime's own tools and
/// processes as tool activity, everything else as unadmitted. The SWBPIPE
/// Agent 0 decided that the guardian review, hook and thread settings methods
/// end it. remoteControl/status/changed is ignored here until pull request
/// 264's rule for it lands.
pub(super) const NOTIFICATIONS: [(&str, Notice); 83] = [
    ("error", Notice::Ignored),
    ("thread/started", Notice::Handled),
    ("thread/status/changed", Notice::Handled),
    ("thread/archived", ENDS),
    ("thread/deleted", ENDS),
    ("thread/unarchived", ENDS),
    ("thread/closed", ENDS),
    ("thread/reverted", ENDS),
    ("skills/changed", ENDS),
    ("thread/name/updated", ENDS),
    ("thread/attachment/updated", ENDS),
    ("thread/goal/updated", ENDS),
    ("thread/goal/cleared", ENDS),
    ("thread/queue/changed", ENDS),
    ("project/changed", ENDS),
    ("thread/project/updated", ENDS),
    ("thread/environment/connected", ENDS),
    ("thread/environment/disconnected", ENDS),
    ("thread/settings/updated", ENDS),
    ("thread/tokenUsage/updated", Notice::Ignored),
    ("turn/started", Notice::Handled),
    ("hook/started", ENDS),
    ("turn/completed", Notice::Handled),
    ("hook/completed", ENDS),
    ("turn/diff/updated", TOOL),
    ("turn/plan/updated", TOOL),
    ("item/started", Notice::Handled),
    ("item/autoApprovalReview/started", ENDS),
    ("item/autoApprovalReview/completed", ENDS),
    ("autoApprovalReview/strictReviewRequired", ENDS),
    ("item/completed", Notice::Handled),
    ("item/agentMessage/delta", Notice::Handled),
    ("item/plan/delta", ENDS),
    ("command/exec/outputDelta", TOOL),
    ("process/outputDelta", TOOL),
    ("process/exited", TOOL),
    ("item/commandExecution/outputDelta", TOOL),
    ("item/commandExecution/terminalInteraction", TOOL),
    ("item/fileChange/outputDelta", TOOL),
    ("item/fileChange/patchUpdated", TOOL),
    ("serverRequest/resolved", ENDS),
    ("item/mcpToolCall/progress", TOOL),
    ("mcpServer/oauthLogin/completed", ENDS),
    ("mcpServer/startupStatus/updated", ENDS),
    ("mcpServer/event/stream/notification", ENDS),
    ("account/updated", ENDS),
    ("account/gatewayOAuth/changed", ENDS),
    ("account/rateLimits/updated", Notice::Ignored),
    ("app/list/updated", ENDS),
    ("remoteControl/status/changed", Notice::Ignored),
    ("externalAgentConfig/import/progress", ENDS),
    ("externalAgentConfig/import/completed", ENDS),
    ("fs/changed", ENDS),
    ("item/reasoning/summaryTextDelta", Notice::Ignored),
    ("item/reasoning/summaryPartAdded", Notice::Ignored),
    ("item/reasoning/textDelta", Notice::Ignored),
    ("thread/compacted", Notice::Ignored),
    ("model/rerouted", ENDS),
    ("model/verification", ENDS),
    ("modelProvider/authRecoveryStarted", ENDS),
    ("modelProvider/authRecoveryCompleted", ENDS),
    ("turn/moderationMetadata", ENDS),
    ("model/safetyBuffering/updated", ENDS),
    ("warning", Notice::Ignored),
    ("guardianWarning", ENDS),
    ("deprecationNotice", Notice::Ignored),
    ("configWarning", Notice::Ignored),
    ("fuzzyFileSearch/sessionUpdated", ENDS),
    ("fuzzyFileSearch/sessionCompleted", ENDS),
    ("thread/realtime/started", ENDS),
    ("thread/realtime/itemAdded", ENDS),
    ("thread/realtime/item/started", ENDS),
    ("thread/realtime/item/transcript/delta", ENDS),
    ("thread/realtime/item/completed", ENDS),
    ("thread/realtime/transcript/delta", ENDS),
    ("thread/realtime/transcript/done", ENDS),
    ("thread/realtime/outputAudio/delta", ENDS),
    ("thread/realtime/sdp", ENDS),
    ("thread/realtime/error", ENDS),
    ("thread/realtime/closed", ENDS),
    ("windows/worldWritableWarning", ENDS),
    ("windowsSandbox/setupCompleted", ENDS),
    ("account/login/completed", ENDS),
];
fn notice(method: &str) -> Notice {
    NOTIFICATIONS
        .iter()
        .find(|(m, _)| *m == method)
        .map_or(ENDS, |(_, class)| *class)
}
/// Read an ignored notification's parameters into their closed type.
fn ignored_params(method: &str, p: &Value) -> Result<(), Error> {
    match method {
        "error" => typed_ref::<wire::ErrorParams>(p).map(drop),
        "thread/tokenUsage/updated" => typed_ref::<wire::ThreadTokenUsageUpdatedParams>(p).map(drop),
        "account/rateLimits/updated" => typed_ref::<wire::AccountRateLimitsUpdatedParams>(p).map(drop),
        "remoteControl/status/changed" => {
            typed_ref::<wire::RemoteControlStatusChangedParams>(p).map(drop)
        }
        "item/reasoning/summaryTextDelta" => {
            typed_ref::<wire::ReasoningSummaryTextDeltaParams>(p).map(drop)
        }
        "item/reasoning/summaryPartAdded" => {
            typed_ref::<wire::ReasoningSummaryPartAddedParams>(p).map(drop)
        }
        "item/reasoning/textDelta" => typed_ref::<wire::ReasoningTextDeltaParams>(p).map(drop),
        "thread/compacted" => typed_ref::<wire::ContextCompactedParams>(p).map(drop),
        "warning" => typed_ref::<wire::WarningParams>(p).map(drop),
        "deprecationNotice" => typed_ref::<wire::DeprecationNoticeParams>(p).map(drop),
        "configWarning" => typed_ref::<wire::ConfigWarningParams>(p).map(drop),
        _ => Err(UNADMITTED_NOTIFICATION),
    }
}

/// Every ServerRequest method of stock 0.160.0's schema. Only the tool call
/// is answered; every other request ends the session, approval requests for
/// the runtime's own tools as tool activity. The supplier runs with approval
/// policy "never", so none of these is expected.
pub(super) const REQUESTS: [(&str, Notice); 11] = [
    ("item/commandExecution/requestApproval", TOOL),
    ("item/fileChange/requestApproval", TOOL),
    ("item/tool/requestUserInput", Notice::Ends(UNADMITTED_REQUEST)),
    ("mcpServer/elicitation/request", Notice::Ends(UNADMITTED_REQUEST)),
    ("item/permissions/requestApproval", TOOL),
    ("item/tool/call", Notice::Handled),
    ("account/chatgptAuthTokens/refresh", Notice::Ends(UNADMITTED_REQUEST)),
    ("attestation/generate", Notice::Ends(UNADMITTED_REQUEST)),
    ("currentTime/read", Notice::Ends(UNADMITTED_REQUEST)),
    ("applyPatchApproval", TOOL),
    ("execCommandApproval", TOOL),
];
fn request_class(method: &str) -> Notice {
    REQUESTS
        .iter()
        .find(|(m, _)| *m == method)
        .map_or(Notice::Ends(UNADMITTED_REQUEST), |(_, class)| *class)
}

/// Native smoke only: one payload-free line for each distinct item kind,
/// notification method and server request method the runtime reports, so an
/// observed session shows the ordinary set. A kind or method is printed only
/// when it is a name the 0.160.0 schema defines; otherwise "unlisted".
#[cfg(any(test, feature = "native-smoke"))]
mod observed {
    use super::{admit, wire, Notice, Value, NOTIFICATIONS, REQUESTS, SCHEMA_ITEM_KINDS};

    const PREFIX: &str = "[native-smoke-agent-host]";
    pub const MAX_LINES: usize = 64;

    fn schema_kind(kind: Option<&Value>) -> &str {
        match kind {
            Some(Value::String(k)) => SCHEMA_ITEM_KINDS
                .iter()
                .find(|known| **known == k.as_str())
                .copied()
                .unwrap_or("unlisted"),
            _ => "unlisted",
        }
    }
    fn schema_method(method: &str) -> &str {
        NOTIFICATIONS
            .iter()
            .chain(REQUESTS.iter())
            .find(|(m, _)| *m == method)
            .map_or("unlisted", |(m, _)| m)
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
        /// `via` is always a schema method name or a fixed response name.
        fn item(&mut self, item: &Value, via: &str) {
            let decision = match <wire::Item as serde::Deserialize>::deserialize(item)
                .map_err(|_| ())
                .and_then(|typed| admit(&typed).map_err(|_| ()))
            {
                Ok(()) => "admitted",
                Err(()) => "refused",
            };
            let line = format!(
                "{PREFIX} item kind {} via {via} {decision}",
                schema_kind(item.get("type"))
            );
            self.line(line);
        }
        fn turn(&mut self, turn: &Value, via: &str) {
            if let Some(items) = turn.get("items").and_then(Value::as_array) {
                items.iter().for_each(|i| self.item(i, via));
            }
        }
        fn thread(&mut self, thread: &Value, via: &str) {
            if let Some(turns) = thread.get("turns").and_then(Value::as_array) {
                turns.iter().for_each(|t| self.turn(t, via));
            }
        }
        pub fn notification(&mut self, method: &str, p: &Value, class: Notice) {
            let via = schema_method(method);
            match method {
                "item/started" | "item/completed" => {
                    if let Some(item) = p.get("item") {
                        self.item(item, via);
                    }
                }
                "turn/started" | "turn/completed" => {
                    if let Some(turn) = p.get("turn") {
                        self.turn(turn, via);
                    }
                }
                "thread/started" => {
                    if let Some(thread) = p.get("thread") {
                        self.thread(thread, via);
                    }
                }
                _ => {}
            }
            let word = match class {
                Notice::Handled => return,
                Notice::Ignored => "ignored",
                Notice::Ends(_) => "refused",
            };
            self.line(format!("{PREFIX} notification {via} {word}"));
        }
        pub fn response(&mut self, result: &Value, via: &'static str) {
            if let Some(thread) = result.get("thread") {
                self.thread(thread, via);
            }
            if let Some(turn) = result.get("turn") {
                self.turn(turn, via);
            }
        }
        pub fn request(&mut self, method: &str) {
            if method == "item/tool/call" {
                return;
            }
            let line = format!("{PREFIX} server request {} refused", schema_method(method));
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
    fn handle_inner(&mut self, frame: Value, basis: &Value) -> Result<Actions, Error> {
        let mut actions = Actions::default();
        let frame: wire::Envelope = typed(frame)?;
        match frame {
            // A server request: JSONRPCRequest.
            wire::Envelope {
                id: Some(id),
                method: Some(method),
                params,
                result: None,
                error: None,
                trace: _,
            } => {
                if !bounded(&method, MAX_ID_BYTES) {
                    return Err(MALFORMED);
                }
                let id = rpc_id(&id)?;
                #[cfg(any(test, feature = "native-smoke"))]
                self.observed.request(&method);
                match request_class(&method) {
                    Notice::Handled => {
                        let params: wire::DynamicToolCallParams =
                            typed(params.ok_or(MALFORMED)?)?;
                        self.tool_call(id, params, basis, &mut actions)?;
                    }
                    // No server request but the tool call conveys anything the
                    // host accepts; each one ends the session.
                    Notice::Ignored => return Err(UNADMITTED_REQUEST),
                    Notice::Ends(reason) => return Err(reason),
                }
            }
            // A notification: JSONRPCNotification.
            wire::Envelope {
                id: None,
                method: Some(method),
                params,
                result: None,
                error: None,
                trace: None,
            } => {
                if !bounded(&method, MAX_ID_BYTES) {
                    return Err(MALFORMED);
                }
                self.notification(&method, params.unwrap_or(Value::Null), &mut actions)?;
            }
            // A response: JSONRPCResponse or JSONRPCError.
            wire::Envelope {
                id: Some(id),
                method: None,
                params: None,
                result,
                error,
                trace: None,
            } => {
                // Exactly one of result and error, as the schema's two forms.
                if result.is_some() == error.is_some() {
                    return Err(MALFORMED);
                }
                let id = match id {
                    wire::RequestId::Int(n) => u64::try_from(n).map_err(|_| MALFORMED)?,
                    wire::RequestId::Str(_) => return Err(Error("invalid response identity")),
                };
                let pos = self
                    .outstanding
                    .iter()
                    .position(|(n, _)| *n == id)
                    .ok_or(Error("unexpected response identity"))?;
                let kind = self.outstanding[pos].1;
                match (result, error) {
                    (Some(result), None) => {
                        self.response(kind, result, &mut actions)?;
                        self.outstanding.remove(pos);
                    }
                    (None, Some(error)) => {
                        let error = json!({"code":error.code,"message":error.message});
                        self.last_rejection = Some(rejection(kind, &error));
                        return Err(Error("client request failed"));
                    }
                    _ => return Err(MALFORMED),
                }
            }
            _ => return Err(MALFORMED),
        }
        Ok(actions)
    }
    fn response(&mut self, kind: Rpc, result: Value, actions: &mut Actions) -> Result<(), Error> {
        #[cfg(any(test, feature = "native-smoke"))]
        self.observed.response(
            &result,
            match kind {
                Rpc::Initialize => "initialize-response",
                Rpc::ThreadStart => "thread/start-response",
                Rpc::TurnStart => "turn/start-response",
                Rpc::Interrupt => "turn/interrupt-response",
            },
        );
        match kind {
            Rpc::Initialize if self.phase == Phase::Initializing => {
                typed::<wire::InitializeResponse>(result)?;
                self.phase = Phase::Ready;
                actions
                    .outbound
                    .push(json!({"method":"initialized","params":{}}));
                actions.events.push(Event::Initialized);
            }
            Rpc::ThreadStart if self.phase == Phase::StartingThread => {
                let result: wire::ThreadStartResponse = typed(result)?;
                admit_thread(&result.thread)?;
                let id = bounded_identity(&result.thread.id)?;
                if self.thread.as_ref().is_some_and(|v| v != &id) {
                    return Err(Error("thread response mismatch"));
                }
                let read_only = matches!(
                    result.sandbox,
                    wire::SandboxPolicy::ReadOnly {
                        network_access: None | Some(false)
                    }
                );
                if result.approval_policy != wire::AskForApproval::Mode(wire::ApprovalMode::Never)
                    || !read_only
                    || !result.thread.ephemeral
                    || result.thread.environments.as_ref().is_some_and(|e| !e.is_empty())
                {
                    return Err(Error("unexpected thread policy"));
                }
                self.thread = Some(id.clone());
                self.phase = Phase::ThreadReady;
                actions.events.push(Event::ThreadStarted { thread_id: id });
            }
            Rpc::TurnStart if matches!(self.phase, Phase::StartingTurn | Phase::InTurn) => {
                let result: wire::TurnStartResponse = typed(result)?;
                admit_turn(&result.turn)?;
                let id = bounded_identity(&result.turn.id)?;
                if result.turn.status != wire::TurnStatus::InProgress
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
            Rpc::Interrupt if matches!(self.phase, Phase::InTurn | Phase::ThreadReady) => {
                typed::<wire::TurnInterruptResponse>(result)?;
                actions.events.push(Event::InterruptAcknowledged);
            }
            _ => return Err(Error("response in invalid state")),
        }
        Ok(())
    }
    fn check_thread(&self, thread_id: &str) -> Result<(), Error> {
        if Some(bounded_identity(thread_id)?.as_str()) != self.thread.as_deref() {
            return Err(Error("thread mismatch"));
        }
        Ok(())
    }
    fn check_turn(&self, thread_id: &str, turn_id: &str) -> Result<(), Error> {
        self.check_thread(thread_id)?;
        if self.phase != Phase::InTurn
            || self.outstanding.iter().any(|(_, k)| *k == Rpc::TurnStart)
            || Some(bounded_identity(turn_id)?.as_str()) != self.turn.as_deref()
        {
            return Err(Error("turn mismatch or unconfirmed turn"));
        }
        Ok(())
    }
    fn notification(&mut self, method: &str, p: Value, actions: &mut Actions) -> Result<(), Error> {
        let class = notice(method);
        #[cfg(any(test, feature = "native-smoke"))]
        self.observed.notification(method, &p, class);
        match class {
            Notice::Ends(reason) => return Err(reason),
            Notice::Ignored => {
                ignored_params(method, &p)?;
            }
            Notice::Handled => {}
        }
        match method {
            "turn/started" => {
                let p: wire::TurnParams = typed(p)?;
                admit_turn(&p.turn)?;
                self.check_thread(&p.thread_id)?;
                if !matches!(self.phase, Phase::StartingTurn | Phase::InTurn)
                    || p.turn.status != wire::TurnStatus::InProgress
                {
                    return Err(Error("unexpected turn start"));
                }
                let id = bounded_identity(&p.turn.id)?;
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
                let p: wire::TurnParams = typed(p)?;
                admit_turn(&p.turn)?;
                self.check_thread(&p.thread_id)?;
                if self.phase != Phase::InTurn
                    || Some(bounded_identity(&p.turn.id)?.as_str()) != self.turn.as_deref()
                    || self.outstanding.iter().any(|(_, k)| *k == Rpc::TurnStart)
                {
                    return Err(Error("unsupported turn completion ordering"));
                }
                if p.turn.status == wire::TurnStatus::InProgress {
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
                    status: p.turn.status.name().to_owned(),
                });
                self.phase = Phase::ThreadReady;
            }
            "item/agentMessage/delta" => {
                let p: wire::AgentMessageDeltaParams = typed(p)?;
                self.check_turn(&p.thread_id, &p.turn_id)?;
                let item_id = bounded_identity(&p.item_id)?;
                if p.delta.len() > MAX_TEXT_BYTES {
                    return Err(Error("message text limit"));
                }
                actions.events.push(Event::MessageDelta { item_id, text: p.delta });
            }
            "item/started" => {
                let p: wire::ItemStartedParams = typed(p)?;
                admit(&p.item)?;
            }
            "item/completed" => {
                let p: wire::ItemCompletedParams = typed(p)?;
                admit(&p.item)?;
                self.check_turn(&p.thread_id, &p.turn_id)?;
                if let Item::AgentMessage { id, text, .. } = p.item {
                    let item_id = bounded_identity(&id)?;
                    if text.len() > MAX_TEXT_BYTES {
                        return Err(Error("message text limit"));
                    }
                    actions.events.push(Event::MessageFinal { item_id, text });
                }
            }
            "thread/status/changed" => {
                let p: wire::ThreadStatusChangedParams = typed(p)?;
                if self.phase == Phase::StartingThread && self.thread.is_none() {
                    self.thread = Some(bounded_identity(&p.thread_id)?);
                }
                self.check_thread(&p.thread_id)?;
                actions.events.push(Event::ThreadStatus {
                    status: p.status.name().to_owned(),
                });
            }
            // The supplier may send thread/started before its response. It is
            // informational only; response supplies authoritative thread id.
            "thread/started" => {
                let p: wire::ThreadStartedParams = typed(p)?;
                admit_thread(&p.thread)?;
                if self.phase == Phase::StartingThread {
                    let id = bounded_identity(&p.thread.id)?;
                    if self.thread.as_deref().is_some_and(|old| old != id) {
                        return Err(Error("thread mismatch"));
                    }
                    self.thread = Some(id);
                }
            }
            // Ignored by name: its parameters were read into their closed
            // type above and carry nothing the host acts on.
            _ => {}
        }
        Ok(())
    }
    fn tool_call(
        &mut self,
        id: Value,
        p: wire::DynamicToolCallParams,
        basis: &Value,
        actions: &mut Actions,
    ) -> Result<(), Error> {
        self.check_turn(&p.thread_id, &p.turn_id)?;
        let call_id = bounded_identity(&p.call_id)?;
        bounded_identity(&p.tool)?;
        if self.seen_rpc.contains(&id) || self.seen_call.contains(&call_id) {
            return Err(Error("duplicate tool identity"));
        }
        if self.seen_rpc.len() >= MAX_CALLS_PER_TURN || self.seen_call.len() >= MAX_CALLS_PER_TURN {
            return Err(Error("turn identity limit"));
        }
        // The supplier forwards a call only for a tool the host declared at
        // thread start, none of them namespaced; anything else means it is
        // not behaving as pinned, and the session ends.
        let tool = match Tool::from_name(&p.tool) {
            Some(tool) if p.namespace.is_none() => tool,
            _ => return Err(UNOFFERED_ACTIVITY),
        };
        self.seen_rpc.push(id.clone());
        self.seen_call.push(call_id.clone());
        let args = p.arguments;
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

/// Schema-complete frames for tests across the embedded agent: each holds
/// every member stock 0.160.0's schema requires, and only members it gives.
#[cfg(test)]
pub(super) mod fixture {
    use serde_json::{json, Value};

    pub fn initialize_result() -> Value {
        json!({"codexHome":"/invented","platformFamily":"unix","platformOs":"macos","userAgent":"fixture"})
    }
    pub fn thread(id: &str) -> Value {
        json!({"cliVersion":"0.160.0","createdAt":1,"cwd":"/invented","ephemeral":true,"id":id,"modelProvider":"fixture","preview":"","projectId":null,"sessionId":"session-invented","source":"appServer","status":{"type":"idle"},"turns":[],"updatedAt":1})
    }
    pub fn thread_with_turns(id: &str, turns: Value) -> Value {
        let mut thread = thread(id);
        thread["turns"] = turns;
        thread
    }
    pub fn thread_start_result(id: &str) -> Value {
        json!({"approvalPolicy":"never","approvalsReviewer":"user","cwd":"/invented","model":"fixture","modelProvider":"fixture","sandbox":{"type":"readOnly","networkAccess":false},"thread":thread(id)})
    }
    pub fn turn(id: &str, status: &str) -> Value {
        json!({"id":id,"items":[],"status":status})
    }
    pub fn turn_with_items(id: &str, status: &str, items: Value) -> Value {
        json!({"id":id,"items":items,"status":status})
    }
    pub fn turn_start_result(id: &str) -> Value {
        json!({"turn":turn(id, "inProgress")})
    }
    pub fn user_message(id: &str) -> Value {
        json!({"type":"userMessage","id":id,"content":[{"type":"text","text":"invented"}]})
    }
    pub fn agent_message(id: &str, text: &str) -> Value {
        json!({"type":"agentMessage","id":id,"text":text})
    }
    pub fn reasoning(id: &str) -> Value {
        json!({"type":"reasoning","id":id,"summary":["invented"]})
    }
    pub fn context_compaction(id: &str) -> Value {
        json!({"type":"contextCompaction","id":id})
    }
    pub fn dynamic_tool_call(id: &str, tool: &str) -> Value {
        json!({"type":"dynamicToolCall","id":id,"tool":tool,"arguments":{},"status":"completed"})
    }
    /// The parameters of item/started or item/completed for an item.
    pub fn item_params(method: &str, thread: &str, turn: &str, item: Value) -> Value {
        let mut params = json!({"threadId":thread,"turnId":turn,"item":item});
        let at = if method == "item/started" { "startedAtMs" } else { "completedAtMs" };
        params[at] = json!(1);
        params
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
        assert_eq!(
            p.handle_frame(frame(1, fixture::initialize_result()), &basis())
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
        p.handle_frame(frame(2, fixture::thread_start_result("thread-1")), &basis()).unwrap();
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

    fn item_frame(method: &str, item: Value) -> Value {
        notification(method, fixture::item_params(method, "thread-1", "turn-1", item))
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
                let (p, reference) = active_with_pending();
                let frame = item_frame(method, json!({"id":"unoffered","type":kind}));
                assert_ends(p, reference, frame, UNOFFERED_ACTIVITY);
            }
        }
    }

    /// The pending call is reported lost exactly once, as for any protocol
    /// failure, and the poisoned protocol accepts nothing further.
    fn assert_ends(mut p: Protocol, reference: RequestRef, frame: Value, reason: Error) {
        let label = frame.to_string();
        assert_eq!(p.handle_frame(frame, &basis()).err(), Some(reason), "{label}");
        assert_eq!(p.refusal(), Some(Refusal::of(reason)), "{label}");
        assert!(
            p.handle_frame(call_summary(json!(99), "after"), &basis())
                .is_err(),
            "{label}"
        );
        assert!(
            matches!(p.disconnect().as_slice(), [Event::ToolLost { request_ref, .. }] if request_ref == &reference),
            "{label}"
        );
        assert!(p.disconnect().is_empty(), "{label}");
    }
    fn active_with_pending() -> (Protocol, RequestRef) {
        let mut p = active();
        let reference = requested(
            p.handle_frame(call_summary(json!(7), "pending"), &basis())
                .unwrap(),
        );
        (p, reference)
    }
    fn admitted_items() -> Vec<Value> {
        vec![
            fixture::user_message("u"),
            fixture::agent_message("m", "hi"),
            fixture::reasoning("r"),
            fixture::context_compaction("c"),
            fixture::dynamic_tool_call("d", Tool::Summary.name()),
        ]
    }

    #[test]
    fn admitted_item_kinds_pass_and_are_handled_as_before() {
        let mut p = active();
        for method in ["item/started", "item/completed"] {
            for item in admitted_items() {
                let message = item["type"] == "agentMessage";
                let actions = p.handle_frame(item_frame(method, item), &basis()).unwrap();
                match (method, message, actions.events.as_slice()) {
                    ("item/completed", true, [Event::MessageFinal { item_id, text }]) => {
                        assert_eq!((item_id.as_str(), text.as_str()), ("m", "hi"))
                    }
                    (_, _, []) if !(method == "item/completed" && message) => {}
                    other => panic!("unexpected {method}: {:?}", other.2),
                }
                assert!(actions.outbound.is_empty());
            }
        }
        // Every item member the schema gives, typed, is accepted.
        let full = json!({"type":"agentMessage","id":"full","text":"t","delivery":"async","phase":"final_answer",
            "memoryCitation":{"entries":[{"lineEnd":2,"lineStart":1,"note":"n","path":"p"}],"threadIds":["t"]},
            "questions":[{"title":"q","options":["a"]}]});
        p.handle_frame(item_frame("item/started", full), &basis()).unwrap();
        let full = json!({"type":"userMessage","id":"u2","clientId":null,"content":[
            {"type":"text","text":"t","text_elements":[{"byteRange":{"start":0,"end":1},"placeholder":null}]},
            {"type":"image","url":"u","detail":"auto"},{"type":"image","fileId":"f"},
            {"type":"localImage","path":"/p"},{"type":"audio","url":"u"},{"type":"localAudio","path":"/p"},
            {"type":"skill","name":"n","path":"/p"},{"type":"mention","name":"n","path":"/p"}]});
        p.handle_frame(item_frame("item/started", full), &basis()).unwrap();
        let full = json!({"type":"dynamicToolCall","id":"d2","tool":Tool::Summary.name(),"namespace":null,"arguments":{"x":1},
            "status":"failed","success":false,"durationMs":3,"contentItems":[{"type":"inputText","text":"t"},{"type":"inputImage","imageUrl":"u"},{"type":"inputAudio","audioUrl":"u"}]});
        p.handle_frame(item_frame("item/completed", full), &basis()).unwrap();
        for (method, params) in [
            ("item/reasoning/textDelta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","delta":"x","contentIndex":0})),
            ("item/reasoning/summaryTextDelta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","delta":"x","summaryIndex":0})),
            ("item/reasoning/summaryPartAdded", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","summaryIndex":0})),
            ("thread/compacted", json!({"threadId":"thread-1","turnId":"turn-1"})),
        ] {
            let actions = p.handle_frame(notification(method, params), &basis()).unwrap();
            assert!(actions.events.is_empty() && actions.outbound.is_empty(), "{method}");
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
        let items = Value::Array(admitted_items());
        p.handle_frame(
            notification("turn/started", json!({"threadId":"thread-1","turn":fixture::turn_with_items("turn-1", "inProgress", items.clone())})),
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
                notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn_with_items("turn-1", "completed", items)})),
                &basis()
            )
            .unwrap()
            .events
            .as_slice(),
            [Event::ToolDetached { .. }, Event::TurnCompleted { .. }]
        ));
    }

    #[test]
    fn unlisted_kinds_are_unadmitted_and_malformed_items_are_malformed() {
        for method in ["item/started", "item/completed"] {
            for kind in [
                "hookPrompt",
                "plan",
                "enteredReviewMode",
                "exitedReviewMode",
                "functionCallOutput",
                "inventedKind",
                "",
                "AgentMessage",
                "agentmessage",
                "DYNAMICTOOLCALL",
                "CommandExecution",
                " agentMessage",
            ] {
                let (p, reference) = active_with_pending();
                let frame = item_frame(method, json!({"id":"item","type":kind,"text":"t"}));
                assert_ends(p, reference, frame, UNLISTED_ITEM);
            }
            let malformed = [
                json!({"id":"item","type":1,"text":"t"}),
                json!({"id":"item","type":true}),
                json!({"id":"item","type":null}),
                json!({"id":"item","type":["agentMessage"]}),
                json!({"id":"item","type":{"type":"agentMessage"}}),
                json!({"id":"item","text":"t"}),
                json!("agentMessage"),
                json!(null),
                // Admitted kinds with a wrong type, a missing required
                // member or an unknown member.
                json!({"id":"u","type":"userMessage","content":null}),
                json!({"id":"u","type":"userMessage"}),
                json!({"id":"u","type":"userMessage","content":[{"type":"image"}]}),
                json!({"id":"u","type":"userMessage","content":[{"type":"text","text":"t","extra":1}]}),
                json!({"type":"contextCompaction","id":null}),
                json!({"type":"contextCompaction"}),
                json!({"type":"contextCompaction","id":"c","items":[{"type":"commandExecution"}]}),
                json!({"type":"agentMessage","id":"m"}),
                json!({"type":"agentMessage","id":"m","text":"t","phase":"other"}),
                json!({"type":"reasoning","id":"r","summary":null}),
                json!({"type":"reasoning","id":"r","content":[1]}),
                json!({"type":"dynamicToolCall","id":"d","tool":Tool::Summary.name(),"status":"completed"}),
                json!({"type":"dynamicToolCall","id":"d","tool":Tool::Summary.name(),"arguments":{},"status":"running"}),
            ];
            for item in malformed {
                let (p, reference) = active_with_pending();
                assert_ends(p, reference, item_frame(method, item), MALFORMED);
            }
            let (p, reference) = active_with_pending();
            let frame = notification(method, json!({"threadId":"thread-1","turnId":"turn-1","startedAtMs":1,"completedAtMs":1}));
            assert_ends(p, reference, frame, MALFORMED);
        }
        // The notification's own required and unknown members.
        for params in [
            json!({"threadId":"thread-1","turnId":"turn-1","item":fixture::user_message("u")}),
            json!({"threadId":"thread-1","turnId":"turn-1","startedAtMs":"1","item":fixture::user_message("u")}),
            json!({"threadId":"thread-1","turnId":"turn-1","startedAtMs":1,"item":fixture::user_message("u"),"items":[{"type":"commandExecution"}]}),
        ] {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, notification("item/started", params), MALFORMED);
        }
    }

    #[test]
    fn items_inside_turns_and_threads_are_checked() {
        let turn_with = |status: &str, item: Value| {
            fixture::turn_with_items("turn-1", status, json!([fixture::user_message("ok"), item]))
        };
        for (item, reason) in [
            (json!({"id":"x","type":"commandExecution"}), UNOFFERED_ACTIVITY),
            (json!({"id":"x","type":"plan"}), UNLISTED_ITEM),
            (json!({"id":"x"}), MALFORMED),
        ] {
            for frame in [
                notification("turn/completed", json!({"threadId":"thread-1","turn":turn_with("completed", item.clone())})),
                notification("turn/started", json!({"threadId":"thread-1","turn":turn_with("inProgress", item.clone())})),
                notification("thread/started", json!({"thread":fixture::thread_with_turns("thread-1", json!([turn_with("completed", item.clone())]))})),
            ] {
                let (p, reference) = active_with_pending();
                assert_ends(p, reference, frame, reason);
            }
        }
    }

    #[test]
    fn non_object_or_incomplete_turns_and_threads_are_malformed() {
        let cmd = json!({"id":"x","type":"commandExecution"});
        let mut thread_without_turns = fixture::thread("thread-1");
        thread_without_turns.as_object_mut().unwrap().remove("turns");
        let mut thread_with_extra = fixture::thread("thread-1");
        thread_with_extra["extra"] = json!({"items":[cmd.clone()]});
        let mut thread_with_data = fixture::thread("thread-1");
        thread_with_data["data"] = json!([{"items":[cmd.clone()]}]);
        let frames = [
            notification("thread/started", json!({"thread":fixture::thread_with_turns("thread-1", json!([[{"items":[cmd.clone()]}]]))})),
            notification("thread/started", json!({"thread":fixture::thread_with_turns("thread-1", json!([null]))})),
            notification("thread/started", json!({"thread":fixture::thread_with_turns("thread-1", json!(["turn"]))})),
            notification("thread/started", json!({"thread":fixture::thread_with_turns("thread-1", json!(null))})),
            notification("thread/started", json!({"thread":thread_without_turns})),
            notification("thread/started", json!({"thread":thread_with_extra})),
            notification("thread/started", json!({"thread":thread_with_data})),
            notification("thread/started", json!({"thread":[{"turns":[{"items":[cmd.clone()]}]}]})),
            notification("thread/started", json!({"thread":null})),
            notification("thread/started", json!({})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn_with_items("turn-1", "completed", json!([[cmd.clone()]]))})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn_with_items("turn-1", "completed", json!([null]))})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn_with_items("turn-1", "completed", json!(null))})),
            // A turn with no items list completes nothing.
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed"}})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":[],"data":[cmd.clone()]}})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":null})),
            notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn("turn-1", "completed"),"thread":{"id":"thread-1"}})),
            notification("turn/started", json!({"threadId":"thread-1","turn":[{"items":[cmd.clone()]}]})),
        ];
        for frame in frames {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, frame, MALFORMED);
        }
    }

    #[test]
    fn dynamic_tool_call_items_must_name_an_offered_tool_without_namespace() {
        let mut p = active();
        for tool in Tool::ALL {
            for item in [
                json!({"id":"d","type":"dynamicToolCall","tool":tool.name(),"arguments":{"x":"ignored"},"status":"completed"}),
                json!({"id":"d","type":"dynamicToolCall","tool":tool.name(),"namespace":null,"arguments":null,"status":"inProgress"}),
            ] {
                for method in ["item/started", "item/completed"] {
                    p.handle_frame(item_frame(method, item.clone()), &basis()).unwrap();
                }
                p.handle_frame(notification("turn/started", json!({"threadId":"thread-1","turn":fixture::turn_with_items("turn-1", "inProgress", json!([item.clone()]))})), &basis()).unwrap();
            }
        }
        assert!(matches!(
            p.handle_frame(call_summary(json!(7), "valid"), &basis()).unwrap().events.as_slice(),
            [Event::ToolRequested { .. }]
        ));
        let call = |extra: Value| {
            let mut item = json!({"id":"d","type":"dynamicToolCall","arguments":{},"status":"completed"});
            item.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
            item
        };
        let cases = [
            (call(json!({})), MALFORMED),
            (call(json!({"tool":null})), MALFORMED),
            (call(json!({"tool":1})), MALFORMED),
            (call(json!({"tool":["x"]})), MALFORMED),
            (call(json!({"tool":"shell"})), UNOFFERED_ACTIVITY),
            (call(json!({"tool":""})), UNOFFERED_ACTIVITY),
            (call(json!({"tool":Tool::Summary.name().to_uppercase()})), UNOFFERED_ACTIVITY),
            (call(json!({"tool":format!("{} ", Tool::Summary.name())})), UNOFFERED_ACTIVITY),
            (call(json!({"tool":Tool::Summary.name(),"namespace":"swbpipe"})), UNOFFERED_ACTIVITY),
            (call(json!({"tool":Tool::Summary.name(),"namespace":""})), UNOFFERED_ACTIVITY),
            (call(json!({"tool":Tool::Summary.name(),"namespace":{}})), MALFORMED),
            (call(json!({"tool":Tool::Summary.name(),"namespace":1})), MALFORMED),
        ];
        for (item, reason) in cases {
            let frames = [
                item_frame("item/started", item.clone()),
                item_frame("item/completed", item.clone()),
                notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn_with_items("turn-1", "completed", json!([item.clone()]))})),
                notification("thread/started", json!({"thread":fixture::thread_with_turns("thread-1", json!([fixture::turn_with_items("old", "completed", json!([item.clone()]))]))})),
            ];
            for frame in frames {
                let (p, reference) = active_with_pending();
                assert_ends(p, reference, frame, reason);
            }
        }
    }

    #[test]
    fn a_tool_call_request_for_an_unoffered_or_namespaced_tool_ends_the_session() {
        let request = |tool: Value, namespace: Option<Value>| {
            let mut c = json!({"id":"r1","method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":"c1","tool":tool,"arguments":{}}});
            if let Some(ns) = namespace {
                c["params"]["namespace"] = ns;
            }
            c
        };
        for (frame, reason) in [
            (request(json!("shell"), None), UNOFFERED_ACTIVITY),
            (request(json!("unknown"), None), UNOFFERED_ACTIVITY),
            (request(json!(Tool::Summary.name()), Some(json!("swbpipe"))), UNOFFERED_ACTIVITY),
            (request(json!(Tool::Summary.name()), Some(json!({}))), MALFORMED),
            (request(json!(1), None), MALFORMED),
            (json!({"id":"r1","method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":"c1","tool":Tool::Summary.name()}}), MALFORMED),
            (json!({"id":"r1","method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":"c1","tool":Tool::Summary.name(),"arguments":{},"extra":1}}), MALFORMED),
            (json!({"id":"r1","method":"item/tool/call"}), MALFORMED),
        ] {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, frame, reason);
        }
        // A null namespace is the absent one.
        let mut p = active();
        let ok = p.handle_frame(request(json!(Tool::Summary.name()), Some(json!(null))), &basis()).unwrap();
        assert!(matches!(ok.events.as_slice(), [Event::ToolRequested { .. }]));
    }

    #[test]
    fn every_server_request_but_the_tool_call_ends_the_session() {
        for (method, reason) in REQUESTS
            .iter()
            .filter(|(m, _)| *m != "item/tool/call")
            .map(|(m, class)| match class {
                Notice::Ends(reason) => (*m, *reason),
                _ => panic!("{m} must end the session"),
            })
            .chain([("remoteControl/enable", UNADMITTED_REQUEST), ("invented/request", UNADMITTED_REQUEST)])
        {
            let (p, reference) = active_with_pending();
            let frame = json!({"id":"server-1","method":method,"params":{}});
            assert_ends(p, reference, frame, reason);
        }
        assert_eq!(request_class("execCommandApproval"), Notice::Ends(UNOFFERED_ACTIVITY));
        assert_eq!(request_class("currentTime/read"), Notice::Ends(UNADMITTED_REQUEST));
    }

    #[test]
    fn every_notification_method_is_classified_and_unlisted_ones_end_the_session() {
        // Each method that ends the session does so with its reason, whatever
        // its parameters.
        for (method, class) in NOTIFICATIONS {
            if let Notice::Ends(reason) = class {
                let (p, reference) = active_with_pending();
                assert_ends(p, reference, notification(method, json!({"threadId":"thread-1"})), reason);
            }
        }
        // The guardian review, hook and settings methods end it (the SWBPIPE
        // Agent 0's decision), as do realtime items and methods not in the
        // schema, including item-keyed ones for an admitted kind.
        for method in [
            "item/autoApprovalReview/started",
            "item/autoApprovalReview/completed",
            "autoApprovalReview/strictReviewRequired",
            "hook/started",
            "hook/completed",
            "thread/settings/updated",
            "thread/realtime/item/started",
            "item/dynamicToolCall/progress",
            "item/agentMessage/started",
            "item/tool/call",
            "item/",
            "",
        ] {
            assert!(matches!(notice(method), Notice::Ends(_)), "{method}");
            let (p, reference) = active_with_pending();
            let reason = match notice(method) {
                Notice::Ends(reason) => reason,
                _ => unreachable!(),
            };
            let frame = if method.is_empty() {
                json!({"method":"","params":{}})
            } else {
                notification(method, json!({"threadId":"thread-1"}))
            };
            assert_ends(p, reference, frame, if method.is_empty() { MALFORMED } else { reason });
        }
    }

    #[test]
    fn ignored_notifications_are_read_whole_and_carry_no_item() {
        let accepted = [
            ("error", json!({"error":{"message":"m","codexErrorInfo":{"httpConnectionFailed":{"httpStatusCode":502}},"additionalDetails":null},"threadId":"thread-1","turnId":"turn-1","willRetry":true})),
            ("error", json!({"error":{"message":"m","codexErrorInfo":"contextWindowExceeded","misalignment":{"steer":{"message":"s"}}},"threadId":"thread-1","turnId":"turn-1","willRetry":false})),
            ("thread/tokenUsage/updated", json!({"threadId":"thread-1","turnId":"turn-1","tokenUsage":{"last":{"cachedInputTokens":0,"inputTokens":1,"outputTokens":1,"reasoningOutputTokens":0,"totalTokens":2},"total":{"cachedInputTokens":0,"inputTokens":1,"outputTokens":1,"reasoningOutputTokens":0,"totalTokens":2,"cacheWriteInputTokens":0},"modelContextWindow":32768}})),
            ("account/rateLimits/updated", json!({"rateLimits":{"primary":{"usedPercent":1,"resetsAt":null},"planType":"unknown","credits":{"hasCredits":false,"unlimited":false,"balance":null}}})),
            ("account/rateLimits/updated", json!({"rateLimits":{}})),
            ("remoteControl/status/changed", json!({"installationId":"i","serverName":"s","status":"disabled","environmentId":null})),
            ("warning", json!({"message":"m"})),
            ("warning", json!({"message":"m","threadId":"thread-1"})),
            ("configWarning", json!({"summary":"s","range":{"start":{"line":1,"column":1},"end":{"line":1,"column":2}}})),
            ("deprecationNotice", json!({"summary":"s","details":null})),
            ("thread/compacted", json!({"threadId":"thread-1","turnId":"turn-1"})),
            ("item/reasoning/textDelta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","delta":"x","contentIndex":0})),
            ("item/reasoning/summaryTextDelta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","delta":"x","summaryIndex":0})),
            ("item/reasoning/summaryPartAdded", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","summaryIndex":0})),
        ];
        let mut p = active();
        for (method, params) in accepted {
            assert_eq!(notice(method), Notice::Ignored, "{method}");
            let actions = p.handle_frame(notification(method, params), &basis()).unwrap();
            assert!(actions.events.is_empty() && actions.outbound.is_empty(), "{method}");
        }
        // An unknown member, a wrong type or a missing member in an ignored
        // notification's parameters is malformed: nothing rides along.
        let refused = [
            ("warning", json!({"message":"m","item":{"type":"commandExecution"}})),
            ("warning", json!({"message":1})),
            ("warning", json!({})),
            ("thread/compacted", json!({"threadId":"thread-1","turnId":"turn-1","items":[{"type":"commandExecution"}]})),
            ("account/rateLimits/updated", json!({"rateLimits":{"turn":{"items":[]}}})),
            ("account/rateLimits/updated", json!({"rateLimits":{"planType":"invented"}})),
            ("error", json!({"error":{"message":"m","codexErrorInfo":{"invented":{}}},"threadId":"thread-1","turnId":"turn-1","willRetry":true})),
            ("remoteControl/status/changed", json!({"installationId":"i","serverName":"s","status":"open"})),
            ("item/reasoning/textDelta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","delta":"x"})),
            ("configWarning", json!({"summary":"s","range":{"start":{"line":-1,"column":1},"end":{"line":1,"column":2}}})),
        ];
        for (method, params) in refused {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, notification(method, params), MALFORMED);
        }
    }

    #[test]
    fn frames_are_closed_envelopes() {
        for frame in [
            json!({"method":"warning","params":{"message":"m"},"jsonrpc":"2.0"}),
            json!({"method":"warning","params":{"message":"m"},"trace":{}}),
            json!({"method":"warning","params":{"message":"m"},"result":{}}),
            json!({"id":7,"result":{},"error":{"code":1,"message":"m"}}),
            json!({"id":"x","method":"item/tool/call","params":{},"result":{}}),
            json!({"method":null,"params":{}}),
            json!({"id":null,"result":{}}),
            json!({"id":1.5,"result":{}}),
            json!([]),
            json!("frame"),
        ] {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, frame, MALFORMED);
        }
        // A request may carry a trace context, as the schema allows.
        let mut p = active();
        let mut c = call_summary(json!(8), "traced");
        c["trace"] = json!({"traceparent":null});
        assert!(matches!(p.handle_frame(c, &basis()).unwrap().events.as_slice(), [Event::ToolRequested { .. }]));
    }

    #[test]
    fn observation_lines_name_each_distinct_kind_method_and_request_once() {
        let mut p = active();
        p.take_observations();
        for _ in 0..2 {
            let mut user = fixture::user_message("u");
            user["content"][0]["text"] = json!("SECRET");
            p.handle_frame(item_frame("item/completed", user), &basis()).unwrap();
            p.handle_frame(item_frame("item/started", fixture::reasoning("r")), &basis()).unwrap();
            p.handle_frame(notification("item/reasoning/textDelta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"r","delta":"SECRET","contentIndex":0})), &basis()).unwrap();
            p.handle_frame(notification("warning", json!({"message":"SECRET"})), &basis()).unwrap();
        }
        assert_eq!(
            p.take_observations(),
            [
                "[native-smoke-agent-host] item kind userMessage via item/completed admitted",
                "[native-smoke-agent-host] item kind reasoning via item/started admitted",
                "[native-smoke-agent-host] notification item/reasoning/textDelta ignored",
                "[native-smoke-agent-host] notification warning ignored",
            ]
        );
        assert!(p.take_observations().is_empty());
        // A refused frame is observed before the session ends, so the caller
        // still prints it.
        for (frame, line) in [
            (item_frame("item/started", json!({"id":"h","type":"hookPrompt"})), "[native-smoke-agent-host] item kind hookPrompt via item/started refused"),
            (notification("hook/started", json!({"threadId":"thread-1"})), "[native-smoke-agent-host] notification hook/started refused"),
            (json!({"id":"a","method":"item/commandExecution/requestApproval","params":{"command":"SECRET"}}), "[native-smoke-agent-host] server request item/commandExecution/requestApproval refused"),
        ] {
            let mut q = active();
            q.take_observations();
            assert!(q.handle_frame(frame, &basis()).is_err());
            assert_eq!(q.take_observations(), [line]);
        }
        let mut r = ready();
        r.submit_text("hello".into()).unwrap();
        r.take_observations();
        let items = json!([fixture::agent_message("m", "t"), {"id":"p","type":"plan"}]);
        assert!(r
            .handle_frame(frame(3, json!({"turn":fixture::turn_with_items("turn-1", "inProgress", items)})), &basis())
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
    fn observation_lines_name_only_what_the_schema_defines() {
        let marker = "SECRET";
        let hostile = [
            format!("agentMessage {marker}"),
            format!("{marker}\" via item/started admitted"),
            format!("{marker}é"),
            format!("{marker}{}", "x".repeat(64)),
            format!("{marker}.kind"),
            format!("{marker}\\n"),
            format!("/private/{marker}"),
            format!("private/{marker}"),
        ];
        let allowed = |line: &str| {
            line.starts_with("[native-smoke-agent-host] ")
                && line[26..]
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b" /_-".contains(&b))
                && !line.contains(marker)
        };
        for name in &hostile {
            let cases = [
                (item_frame("item/completed", json!({"id":"i","type":name,"text":marker})), "[native-smoke-agent-host] item kind unlisted via item/completed refused"),
                (notification(name, json!({"threadId":"thread-1","x":marker})), "[native-smoke-agent-host] notification unlisted refused"),
                (notification(&format!("item/{name}/delta"), json!({"threadId":"thread-1"})), "[native-smoke-agent-host] notification unlisted refused"),
                (json!({"id":"rq","method":format!("x/{name}"),"params":{"x":marker}}), "[native-smoke-agent-host] server request unlisted refused"),
            ];
            for (frame, line) in cases {
                let mut p = active();
                p.take_observations();
                assert!(p.handle_frame(frame, &basis()).is_err(), "{name}");
                let lines = p.take_observations();
                assert_eq!(lines, [line], "{name}");
                assert!(lines.iter().all(|l| allowed(l)), "{lines:?}");
            }
        }
        for kind in [json!({"type":marker}), json!([marker]), json!(1), json!(null)] {
            let mut p = active();
            p.take_observations();
            assert!(p.handle_frame(item_frame("item/started", json!({"id":"i","type":kind})), &basis()).is_err());
            assert_eq!(p.take_observations(), ["[native-smoke-agent-host] item kind unlisted via item/started refused"]);
        }
        let mut p = active();
        p.take_observations();
        assert!(p.handle_frame(item_frame("item/started", json!(marker)), &basis()).is_err());
        assert_eq!(p.take_observations(), ["[native-smoke-agent-host] item kind unlisted via item/started refused"]);
    }

    #[test]
    fn observation_lines_are_bounded_per_session() {
        let mut o = observed::Observed::default();
        for (method, _) in NOTIFICATIONS {
            o.notification(method, &json!({}), Notice::Ignored);
        }
        let lines = o.take();
        assert_eq!(lines.len(), observed::MAX_LINES + 1);
        assert_eq!(lines.last().unwrap(), "[native-smoke-agent-host] observation limit reached");
        for (method, _) in NOTIFICATIONS {
            o.notification(method, &json!({}), Notice::Ends(MALFORMED));
        }
        assert!(o.take().is_empty());
    }

    #[test]
    fn responses_are_read_into_the_type_of_their_request() {
        let cmd = json!([{"items":[{"type":"commandExecution"}]}]);
        // An interrupt response with its correct identity and an alternate list.
        for result in [json!({"data":cmd.clone()}), json!({"turn":fixture::turn("turn-1", "completed")}), json!([]), json!(null)] {
            let (mut p, reference) = active_with_pending();
            assert_eq!(p.interrupt().unwrap()["id"], 4);
            assert_ends(p, reference, frame(4, result), MALFORMED);
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
        for result in [
            {
                let mut v = fixture::initialize_result();
                v["data"] = cmd.clone();
                v
            },
            {
                let mut v = fixture::initialize_result();
                v["codexHome"] = json!(1);
                v
            },
            {
                let mut v = fixture::initialize_result();
                v.as_object_mut().unwrap().remove("userAgent");
                v
            },
        ] {
            let mut p = Protocol::new(Catalog::load().unwrap(), "0123456789abcdef".into(), 16).unwrap();
            p.initialize("test", "1").unwrap();
            assert_eq!(p.handle_frame(frame(1, result), &basis()).err(), Some(MALFORMED));
            assert_eq!(p.refusal(), Some(Refusal::Malformed));
            assert!(p.handle_frame(frame(1, fixture::initialize_result()), &basis()).is_err());
            assert!(p.disconnect().is_empty());
        }
        let thread_start = |result: Value| {
            let mut q = Protocol::new(Catalog::load().unwrap(), "0123456789abcdef".into(), 16).unwrap();
            q.initialize("test", "1").unwrap();
            q.handle_frame(frame(1, fixture::initialize_result()), &basis()).unwrap();
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
        let with = |path: &[&str], value: Value| {
            let mut v = fixture::thread_start_result("thread-1");
            let mut at = &mut v;
            for key in &path[..path.len() - 1] {
                at = &mut at[*key];
            }
            at[path[path.len() - 1]] = value;
            v
        };
        let without = |key: &str| {
            let mut v = fixture::thread_start_result("thread-1");
            v.as_object_mut().unwrap().remove(key);
            v
        };
        for (result, reason) in [
            (with(&["turns"], cmd.clone()), MALFORMED),
            (with(&["model"], json!({"items":[{"id":"c","type":"commandExecution"}]})), MALFORMED),
            (with(&["thread", "data"], cmd.clone()), MALFORMED),
            (with(&["thread", "turns"], json!([null])), MALFORMED),
            (with(&["thread", "extra"], json!({"note":"x"})), MALFORMED),
            (with(&["sandbox", "networkAccess"], json!("no")), MALFORMED),
            (with(&["approvalsReviewer"], json!("invented")), MALFORMED),
            (without("approvalsReviewer"), MALFORMED),
            (without("thread"), MALFORMED),
            (with(&["thread", "turns"], json!([fixture::turn_with_items("old", "completed", json!([{"id":"w","type":"webSearch"}]))])), UNOFFERED_ACTIVITY),
            (with(&["approvalPolicy"], json!("on-request")), Error("unexpected thread policy")),
            (with(&["sandbox"], json!({"type":"workspaceWrite"})), Error("unexpected thread policy")),
            (with(&["sandbox", "networkAccess"], json!(true)), Error("unexpected thread policy")),
            (with(&["thread", "ephemeral"], json!(false)), Error("unexpected thread policy")),
        ] {
            let (mut q, outcome) = thread_start(result);
            assert_eq!(outcome, Some(reason));
            assert!(q.submit_text("next".into()).is_err());
            assert!(q.disconnect().is_empty());
        }
        for result in [
            fixture::thread_start_result("thread-1"),
            with(&["thread", "extra"], json!({})),
            with(&["approvalPolicy"], json!("never")),
            {
                let mut v = with(&["thread", "turns"], json!([]));
                for (k, value) in [
                    ("activePermissionProfile", json!({"id":":read-only","extends":null})),
                    ("disabledPluginIds", json!([])),
                    ("instructionSources", json!([])),
                    ("multiAgentMode", json!("explicitRequestOnly")),
                    ("reasoningEffort", json!("low")),
                    ("runtimeWorkspaceRoots", json!([])),
                    ("serviceTier", json!(null)),
                ] {
                    v[k] = value;
                }
                v["thread"]["environments"] = json!([]);
                v["thread"]["gitInfo"] = json!({"branch":null});
                v["thread"]["status"] = json!({"type":"active","activeFlags":[]});
                v["thread"]["source"] = json!({"subAgent":{"thread_spawn":{"depth":1,"parent_thread_id":"p"}}});
                v
            },
        ] {
            let (_, accepted) = thread_start(result);
            assert_eq!(accepted, None);
        }
        for (result, reason) in [
            (json!({"turn":fixture::turn("turn-1", "inProgress"),"items":cmd.clone()}), MALFORMED),
            (json!({"turn":{"id":"turn-1","status":"inProgress","items":[],"data":cmd.clone()}}), MALFORMED),
            (json!({"turn":{"id":"turn-1","status":"inProgress"}}), MALFORMED),
            (json!({"turn":fixture::turn_with_items("turn-1", "inProgress", json!([{"id":"h","type":"hookPrompt"}]))}), UNLISTED_ITEM),
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
            (item_frame("item/started", json!({"id":"x","type":"plan"})), "unadmitted_report"),
            (item_frame("item/started", fixture::dynamic_tool_call("x", "shell")), "runtime_tool_activity"),
            (notification("hook/started", json!({})), "unadmitted_report"),
            (notification("turn/plan/updated", json!({})), "runtime_tool_activity"),
            (json!({"id":"s","method":"currentTime/read","params":{}}), "unadmitted_report"),
            (json!({"id":"s","method":"execCommandApproval","params":{}}), "runtime_tool_activity"),
            (item_frame("item/started", json!({"id":"x","type":"dynamicToolCall","tool":"shell","namespace":{},"arguments":{},"status":"completed"})), "malformed_report"),
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

    // The schema's own lists, extracted from the 0.160.0 schema copy
    // `codex_app_server_protocol.schemas.json` (SHA-256 below) and embedded
    // here so that no file outside the repository is read at test time.
    const SCHEMA_SHA256: &str = "7243ba241962af92ca60581f1a81808ebda4212a800f8b205f54703bcfd508c5";
    const SCHEMA_NOTIFICATION_METHODS: [&str; 83] = [
        "error", "thread/started", "thread/status/changed", "thread/archived", "thread/deleted",
        "thread/unarchived", "thread/closed", "thread/reverted", "skills/changed",
        "thread/name/updated", "thread/attachment/updated", "thread/goal/updated",
        "thread/goal/cleared", "thread/queue/changed", "project/changed", "thread/project/updated",
        "thread/environment/connected", "thread/environment/disconnected",
        "thread/settings/updated", "thread/tokenUsage/updated", "turn/started", "hook/started",
        "turn/completed", "hook/completed", "turn/diff/updated", "turn/plan/updated",
        "item/started", "item/autoApprovalReview/started", "item/autoApprovalReview/completed",
        "autoApprovalReview/strictReviewRequired", "item/completed", "item/agentMessage/delta",
        "item/plan/delta", "command/exec/outputDelta", "process/outputDelta", "process/exited",
        "item/commandExecution/outputDelta", "item/commandExecution/terminalInteraction",
        "item/fileChange/outputDelta", "item/fileChange/patchUpdated", "serverRequest/resolved",
        "item/mcpToolCall/progress", "mcpServer/oauthLogin/completed",
        "mcpServer/startupStatus/updated", "mcpServer/event/stream/notification",
        "account/updated", "account/gatewayOAuth/changed", "account/rateLimits/updated",
        "app/list/updated", "remoteControl/status/changed", "externalAgentConfig/import/progress",
        "externalAgentConfig/import/completed", "fs/changed", "item/reasoning/summaryTextDelta",
        "item/reasoning/summaryPartAdded", "item/reasoning/textDelta", "thread/compacted",
        "model/rerouted", "model/verification", "modelProvider/authRecoveryStarted",
        "modelProvider/authRecoveryCompleted", "turn/moderationMetadata",
        "model/safetyBuffering/updated", "warning", "guardianWarning", "deprecationNotice",
        "configWarning", "fuzzyFileSearch/sessionUpdated", "fuzzyFileSearch/sessionCompleted",
        "thread/realtime/started", "thread/realtime/itemAdded", "thread/realtime/item/started",
        "thread/realtime/item/transcript/delta", "thread/realtime/item/completed",
        "thread/realtime/transcript/delta", "thread/realtime/transcript/done",
        "thread/realtime/outputAudio/delta", "thread/realtime/sdp", "thread/realtime/error",
        "thread/realtime/closed", "windows/worldWritableWarning", "windowsSandbox/setupCompleted",
        "account/login/completed",
    ];
    const SCHEMA_REQUEST_METHODS: [&str; 11] = [
        "item/commandExecution/requestApproval", "item/fileChange/requestApproval",
        "item/tool/requestUserInput", "mcpServer/elicitation/request",
        "item/permissions/requestApproval", "item/tool/call", "account/chatgptAuthTokens/refresh",
        "attestation/generate", "currentTime/read", "applyPatchApproval", "execCommandApproval",
    ];
    const SCHEMA_THREAD_ITEM_KINDS: [&str; 19] = [
        "userMessage", "hookPrompt", "agentMessage", "functionCallOutput", "plan", "reasoning",
        "commandExecution", "fileChange", "mcpToolCall", "dynamicToolCall", "collabAgentToolCall",
        "subAgentActivity", "webSearch", "imageView", "sleep", "imageGeneration",
        "enteredReviewMode", "exitedReviewMode", "contextCompaction",
    ];

    #[test]
    fn the_tables_match_the_schema_method_and_kind_lists() {
        assert_eq!(SCHEMA_SHA256.len(), 64);
        let sorted = |mut v: Vec<&'static str>| {
            v.sort_unstable();
            v
        };
        assert_eq!(
            sorted(NOTIFICATIONS.iter().map(|(m, _)| *m).collect()),
            sorted(SCHEMA_NOTIFICATION_METHODS.to_vec())
        );
        assert_eq!(
            sorted(REQUESTS.iter().map(|(m, _)| *m).collect()),
            sorted(SCHEMA_REQUEST_METHODS.to_vec())
        );
        assert_eq!(sorted(SCHEMA_ITEM_KINDS.to_vec()), sorted(SCHEMA_THREAD_ITEM_KINDS.to_vec()));
        // The confirmed classification: seven handled, eleven ignored.
        let count = |class: fn(&Notice) -> bool| NOTIFICATIONS.iter().filter(|(_, c)| class(c)).count();
        assert_eq!(count(|c| *c == Notice::Handled), 7);
        assert_eq!(count(|c| *c == Notice::Ignored), 11);
        assert_eq!(count(|c| matches!(c, Notice::Ends(_))), 65);
        assert_eq!(REQUESTS.iter().filter(|(_, c)| *c == Notice::Handled).count(), 1);
    }

    #[test]
    fn items_in_thread_and_turn_responses_are_checked() {
        let unlisted = json!([{"id":"h","type":"hookPrompt"}]);
        let mut p = ready();
        p.submit_text("hello".into()).unwrap();
        assert_eq!(
            p.handle_frame(frame(3, json!({"turn":fixture::turn_with_items("turn-1", "inProgress", unlisted)})), &basis())
                .err(),
            Some(UNLISTED_ITEM)
        );
        let mut q = Protocol::new(Catalog::load().unwrap(), "0123456789abcdef".into(), 16).unwrap();
        q.initialize("test", "1").unwrap();
        q.handle_frame(frame(1, fixture::initialize_result()), &basis()).unwrap();
        q.start_thread(ThreadOptions {
            model: "test".into(),
            provider: "test".into(),
            cwd: "/test".into(),
            base_instructions: "test".into(),
        })
        .unwrap();
        let mut result = fixture::thread_start_result("thread-1");
        result["thread"]["turns"] = json!([fixture::turn_with_items("old", "completed", json!([{"id":"x","type":"webSearch"}]))]);
        assert_eq!(q.handle_frame(frame(2, result), &basis()).err(), Some(UNOFFERED_ACTIVITY));
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
        // A call of a tool the catalog does not offer now ends the session:
        // the supplier forwards only declared tools, so it is not as pinned.
        let unknown = p.handle_frame(call(json!(10), "unknown", "unknown", json!({})), &basis());
        assert_eq!(unknown.err(), Some(UNOFFERED_ACTIVITY));
        assert_eq!(p.refusal(), Some(Refusal::ToolActivity));
        assert!(p.handle_frame(call_summary(json!(11), "after"), &basis()).is_err());
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
