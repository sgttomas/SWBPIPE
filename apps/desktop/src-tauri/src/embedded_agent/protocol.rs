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
    /// The runtime reported, inside the active turn, an error it will retry
    /// (true), or something else has since arrived in that turn or the turn
    /// ended (false). Payload-free; the supplier's message is never carried.
    Reconnecting(bool),
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
    // A retrying error is the latest thing received in the active turn.
    reconnecting: bool,
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
/// A server request's identity, whose form (int64 or string) was checked.
fn rpc_id(id: &Value) -> Result<Value, Error> {
    match id {
        Value::Number(_) => Ok(id.clone()),
        Value::String(s) if bounded(s, MAX_ID_BYTES) => Ok(id.clone()),
        _ => Err(Error("invalid request identity")),
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

/// The JSON forms of every frame the host reads from stock 0.160.0's app
/// server, written once, as data, from that release's protocol schema
/// (`codex_app_server_protocol.schemas.json`, SHA-256
/// 7243ba241962af92ca60581f1a81808ebda4212a800f8b205f54703bcfd508c5).
/// One generic checker reads a value against its form before the host
/// decides anything; no `serde` derive reads a wire type. A form names every
/// member the schema gives, whether it is required, and its form; null is
/// admitted only where the form says nullable, a present null is a present
/// member, an empty string only where no minimum length is set, and a member
/// the form does not name is refused. The tag of the item union is open: a
/// refused or unlisted kind passes the form unread and admission refuses it.
pub(super) mod form {
    use serde_json::Value;

    #[derive(Clone, Copy, Debug)]
    pub enum Form {
        Str,
        /// A string of at least this many characters (schema `minLength`).
        StrMin(usize),
        /// An integer of the schema's `format`.
        Int(&'static str),
        Bool,
        /// Any JSON value (schema `true`). Never read for a decision.
        Any,
        /// One word of a closed set (schema `enum`).
        Words(&'static [&'static str]),
        Array(&'static Form),
        Nullable(&'static Form),
        Object(&'static Obj),
        Tagged(&'static Tagged),
        /// Exactly one alternative matches.
        OneOf(&'static [Form]),
    }
    #[derive(Debug)]
    pub struct Obj {
        pub members: &'static [Member],
        /// At least one of these sets of members is present (schema `anyOf`
        /// of `required`), when not empty.
        pub any_required: &'static [&'static [&'static str]],
    }
    #[derive(Debug)]
    pub struct Member {
        pub name: &'static str,
        pub required: bool,
        pub form: Form,
    }
    #[derive(Debug)]
    pub struct Tagged {
        pub tag: &'static str,
        pub variants: &'static [(&'static str, Obj)],
        /// Tag words recognised and ended by admission, their bodies unread.
        pub refused: &'static [&'static str],
        /// Whether any other string tag passes the form, to be refused by
        /// admission as unlisted.
        pub open: bool,
    }
    pub const fn req(name: &'static str, form: Form) -> Member {
        Member { name, required: true, form }
    }
    pub const fn opt(name: &'static str, form: Form) -> Member {
        Member { name, required: false, form }
    }
    pub const fn obj(members: &'static [Member]) -> Obj {
        Obj { members, any_required: &[] }
    }

    /// An integer within the range of the schema's `format`; never a float.
    fn int(v: &Value, format: &str) -> bool {
        match format {
            "int64" => v.as_i64().is_some(),
            "int32" => v.as_i64().is_some_and(|i| i32::try_from(i).is_ok()),
            "uint" => v.as_u64().is_some(),
            "uint32" => v.as_u64().is_some_and(|u| u32::try_from(u).is_ok()),
            "uint16" => v.as_u64().is_some_and(|u| u16::try_from(u).is_ok()),
            _ => false,
        }
    }
    pub fn object(v: &Value, o: &Obj) -> bool {
        object_with(v, o, None)
    }
    /// An object of exactly these members; `tag` is a further member
    /// admitted for a tagged variant.
    fn object_with(v: &Value, o: &Obj, tag: Option<&str>) -> bool {
        let Some(map) = v.as_object() else { return false };
        map.iter().all(|(k, value)| {
            Some(k.as_str()) == tag
                || o.members.iter().any(|m| m.name == k && check(value, &m.form))
        }) && o.members.iter().all(|m| !m.required || map.contains_key(m.name))
            && (o.any_required.is_empty()
                || o.any_required
                    .iter()
                    .any(|set| set.iter().all(|name| map.contains_key(*name))))
    }
    pub fn check(v: &Value, f: &Form) -> bool {
        match f {
            Form::Str => v.is_string(),
            Form::StrMin(min) => v.as_str().is_some_and(|s| s.chars().count() >= *min),
            Form::Int(format) => int(v, format),
            Form::Bool => v.is_boolean(),
            Form::Any => true,
            Form::Words(words) => v.as_str().is_some_and(|s| words.contains(&s)),
            Form::Array(item) => v.as_array().is_some_and(|a| a.iter().all(|e| check(e, item))),
            Form::Nullable(inner) => v.is_null() || check(v, inner),
            Form::Object(o) => object(v, o),
            Form::Tagged(t) => tagged(v, t),
            Form::OneOf(alternatives) => alternatives.iter().filter(|a| check(v, a)).count() == 1,
        }
    }
    fn tagged(v: &Value, t: &Tagged) -> bool {
        let Some(word) = v.as_object().and_then(|m| m.get(t.tag)).and_then(Value::as_str) else {
            return false;
        };
        match t.variants.iter().find(|(w, _)| *w == word) {
            Some((_, o)) => object_with(v, o, Some(t.tag)),
            None => t.refused.contains(&word) || t.open,
        }
    }
}
use form::{obj, opt, req, Form, Obj, Tagged};

// The forms, transcribed from the schema. Names follow its definitions.
const STR: Form = Form::Str;
const NSTR: Form = Form::Nullable(&Form::Str);
const NBOOL: Form = Form::Nullable(&Form::Bool);
const I64: Form = Form::Int("int64");
const NI64: Form = Form::Nullable(&Form::Int("int64"));
const REASONING_EFFORT: Form = Form::StrMin(1);

// JSON-RPC envelope: JSONRPCRequest, JSONRPCNotification, JSONRPCResponse,
// JSONRPCError.
const REQUEST_ID: Form = Form::OneOf(&[Form::Int("int64"), Form::Str]);
/// A server request as the supplier writes one: exactly a method, its
/// identity and its parameters (from the supplier's source at the pinned tag:
/// it writes no trace member, which only the request form it reads from
/// clients carries; rechecked at every move of the pin).
pub(super) const ENVELOPE_REQUEST: Obj = obj(&[req("id", REQUEST_ID), req("method", STR), req("params", Form::Any)]);
/// Every ServerNotification variant of the schema requires its parameters.
pub(super) const ENVELOPE_NOTIFICATION: Obj = obj(&[req("method", STR), req("params", Form::Any)]);
pub(super) const ENVELOPE_RESPONSE: Obj = obj(&[req("id", REQUEST_ID), req("result", Form::Any)]);
const JSONRPC_ERROR_ERROR: Obj = obj(&[req("code", I64), opt("data", Form::Any), req("message", STR)]);
pub(super) const ENVELOPE_ERROR: Obj =
    obj(&[req("error", Form::Object(&JSONRPC_ERROR_ERROR)), req("id", REQUEST_ID)]);

// The tool call request (DynamicToolCallParams).
pub(super) const DYNAMIC_TOOL_CALL_PARAMS: Obj = obj(&[
    req("arguments", Form::Any),
    req("callId", STR),
    opt("namespace", NSTR),
    req("threadId", STR),
    req("tool", STR),
    req("turnId", STR),
]);

// Items (ThreadItem): the five admitted kinds in full.
const IMAGE_DETAIL: Form = Form::Nullable(&Form::Words(&["auto", "low", "high", "original"]));
const BYTE_RANGE: Obj = obj(&[req("end", Form::Int("uint")), req("start", Form::Int("uint"))]);
const TEXT_ELEMENT: Obj = obj(&[req("byteRange", Form::Object(&BYTE_RANGE)), opt("placeholder", NSTR)]);
const USER_INPUT: Tagged = Tagged {
    tag: "type",
    variants: &[
        ("text", obj(&[req("text", STR), opt("text_elements", Form::Array(&Form::Object(&TEXT_ELEMENT)))])),
        (
            "image",
            Obj {
                members: &[opt("detail", IMAGE_DETAIL), opt("fileId", STR), opt("url", STR)],
                any_required: &[&["fileId"], &["url"]],
            },
        ),
        ("localImage", obj(&[opt("detail", IMAGE_DETAIL), req("path", STR)])),
        ("audio", obj(&[req("url", STR)])),
        ("localAudio", obj(&[req("path", STR)])),
        ("skill", obj(&[req("name", STR), req("path", STR)])),
        ("mention", obj(&[req("name", STR), req("path", STR)])),
    ],
    refused: &[],
    open: false,
};
const MEMORY_CITATION_ENTRY: Obj = obj(&[
    req("lineEnd", Form::Int("uint32")),
    req("lineStart", Form::Int("uint32")),
    req("note", STR),
    req("path", STR),
]);
const MEMORY_CITATION: Obj = obj(&[
    req("entries", Form::Array(&Form::Object(&MEMORY_CITATION_ENTRY))),
    req("threadIds", Form::Array(&Form::Str)),
]);
const ASYNC_USER_INPUT_QUESTION: Obj =
    obj(&[opt("options", Form::Nullable(&Form::Array(&Form::Str))), req("title", STR)]);
const DYNAMIC_TOOL_CALL_OUTPUT_CONTENT_ITEM: Tagged = Tagged {
    tag: "type",
    variants: &[
        ("inputText", obj(&[req("text", STR)])),
        ("inputImage", obj(&[req("imageUrl", STR)])),
        ("inputAudio", obj(&[req("audioUrl", STR)])),
    ],
    refused: &[],
    open: false,
};
pub(super) const THREAD_ITEM: Tagged = Tagged {
    tag: "type",
    variants: &[
        (
            "userMessage",
            obj(&[
                opt("clientId", NSTR),
                req("content", Form::Array(&Form::Tagged(&USER_INPUT))),
                req("id", STR),
            ]),
        ),
        (
            "agentMessage",
            obj(&[
                opt("delivery", Form::Nullable(&Form::Words(&["async"]))),
                req("id", STR),
                opt("memoryCitation", Form::Nullable(&Form::Object(&MEMORY_CITATION))),
                opt("phase", Form::Nullable(&Form::Words(&["commentary", "final_answer"]))),
                opt("questions", Form::Nullable(&Form::Array(&Form::Object(&ASYNC_USER_INPUT_QUESTION)))),
                req("text", STR),
            ]),
        ),
        (
            "reasoning",
            obj(&[opt("content", Form::Array(&Form::Str)), req("id", STR), opt("summary", Form::Array(&Form::Str))]),
        ),
        ("contextCompaction", obj(&[req("id", STR)])),
        (
            "dynamicToolCall",
            obj(&[
                req("arguments", Form::Any),
                opt("contentItems", Form::Nullable(&Form::Array(&Form::Tagged(&DYNAMIC_TOOL_CALL_OUTPUT_CONTENT_ITEM)))),
                opt("durationMs", NI64),
                req("id", STR),
                opt("namespace", NSTR),
                req("status", Form::Words(&["inProgress", "completed", "failed"])),
                opt("success", NBOOL),
                req("tool", STR),
            ]),
        ),
    ],
    refused: &[
        "commandExecution", "fileChange", "mcpToolCall", "collabAgentToolCall", "subAgentActivity",
        "webSearch", "imageView", "sleep", "imageGeneration", "hookPrompt", "plan",
        "enteredReviewMode", "exitedReviewMode", "functionCallOutput",
    ],
    open: true,
};

// Turn and thread.
const HTTP_STATUS: Obj = obj(&[opt("httpStatusCode", Form::Nullable(&Form::Int("uint16")))]);
const CODEX_ERROR_INFO: Form = Form::OneOf(&[
    Form::Words(&[
        "contextWindowExceeded", "sessionBudgetExceeded", "usageLimitExceeded", "rateLimitExceeded",
        "flexUnavailable", "serverOverloaded", "cyberPolicy", "misalignmentPolicyViolation",
        "tooManyDenials", "internalServerError", "unauthorized", "badRequest",
        "threadRollbackFailed", "sandboxError", "other",
    ]),
    Form::Object(&obj(&[req("httpConnectionFailed", Form::Object(&HTTP_STATUS))])),
    Form::Object(&obj(&[req("responseStreamConnectionFailed", Form::Object(&HTTP_STATUS))])),
    Form::Object(&obj(&[req("responseStreamDisconnected", Form::Object(&HTTP_STATUS))])),
    Form::Object(&obj(&[req("responseTooManyFailedAttempts", Form::Object(&HTTP_STATUS))])),
    Form::Object(&obj(&[req(
        "activeTurnNotSteerable",
        Form::Object(&obj(&[req("turnKind", Form::Words(&["review", "compact"]))])),
    )])),
]);
const MISALIGNMENT_STEER: Obj = obj(&[req("message", STR)]);
const MISALIGNMENT_ERROR_DETAILS: Obj = obj(&[
    opt("detailedExplanation", NSTR),
    opt("errorType", NSTR),
    opt("steer", Form::Nullable(&Form::Object(&MISALIGNMENT_STEER))),
]);
const TURN_ERROR: Obj = obj(&[
    opt("additionalDetails", NSTR),
    opt("codexErrorInfo", Form::Nullable(&CODEX_ERROR_INFO)),
    req("message", STR),
    opt("misalignment", Form::Nullable(&Form::Object(&MISALIGNMENT_ERROR_DETAILS))),
]);
pub(super) const TURN: Obj = obj(&[
    opt("completedAt", NI64),
    opt("durationMs", NI64),
    opt("error", Form::Nullable(&Form::Object(&TURN_ERROR))),
    req("id", STR),
    req("items", Form::Array(&Form::Tagged(&THREAD_ITEM))),
    opt("itemsView", Form::Words(&["notLoaded", "summary", "full"])),
    opt("startedAt", NI64),
    req("status", Form::Words(&["completed", "interrupted", "failed", "inProgress"])),
]);
const THREAD_ENVIRONMENT: Obj = obj(&[
    req("cwd", STR),
    req("environmentId", STR),
    req("runtimeWorkspaceRoots", Form::Array(&Form::Str)),
]);
/// The schema gives this object no members and does not close it; the host
/// admits only `{}` by its own rule that unknown members are refused.
const THREAD_EXTRA: Obj = obj(&[]);
const GIT_INFO: Obj = obj(&[opt("branch", NSTR), opt("originUrl", NSTR), opt("sha", NSTR)]);
const THREAD_SECTION_APPEARANCE: Obj = obj(&[opt("color", NSTR), opt("icon", NSTR)]);
const THREAD_SECTION: Obj = obj(&[
    opt("appearance", Form::Nullable(&Form::Object(&THREAD_SECTION_APPEARANCE))),
    req("id", STR),
    req("name", STR),
]);
const THREAD_SPAWN: Obj = obj(&[
    opt("agent_nickname", NSTR),
    opt("agent_path", NSTR),
    opt("agent_role", NSTR),
    req("depth", Form::Int("int32")),
    req("parent_thread_id", STR),
]);
const SUB_AGENT_SOURCE: Form = Form::OneOf(&[
    Form::Words(&["review", "compact", "memory_consolidation"]),
    Form::Object(&obj(&[req("thread_spawn", Form::Object(&THREAD_SPAWN))])),
    Form::Object(&obj(&[req("other", STR)])),
]);
const SESSION_SOURCE: Form = Form::OneOf(&[
    Form::Words(&["cli", "vscode", "exec", "appServer", "unknown"]),
    Form::Object(&obj(&[req("custom", STR)])),
    Form::Object(&obj(&[req("subAgent", SUB_AGENT_SOURCE)])),
]);
pub(super) const THREAD_STATUS: Tagged = Tagged {
    tag: "type",
    variants: &[
        ("notLoaded", obj(&[])),
        ("idle", obj(&[])),
        ("systemError", obj(&[])),
        ("active", obj(&[req("activeFlags", Form::Array(&Form::Words(&["waitingOnApproval", "waitingOnUserInput"])))])),
    ],
    refused: &[],
    open: false,
};
pub(super) const THREAD: Obj = obj(&[
    opt("agentNickname", NSTR),
    opt("agentRole", NSTR),
    opt("canAcceptDirectInput", NBOOL),
    req("cliVersion", STR),
    req("createdAt", I64),
    req("cwd", STR),
    opt("daybreakEnabled", NBOOL),
    opt("environments", Form::Nullable(&Form::Array(&Form::Object(&THREAD_ENVIRONMENT)))),
    req("ephemeral", Form::Bool),
    opt("extra", Form::Nullable(&Form::Object(&THREAD_EXTRA))),
    opt("forkedFromId", NSTR),
    opt("gitInfo", Form::Nullable(&Form::Object(&GIT_INFO))),
    opt("historyMode", Form::Words(&["legacy", "paginated"])),
    req("id", STR),
    opt("model", NSTR),
    req("modelProvider", STR),
    opt("name", NSTR),
    opt("originator", NSTR),
    opt("parentThreadId", NSTR),
    opt("path", NSTR),
    req("preview", STR),
    req("projectId", NSTR),
    opt("reasoningEffort", Form::Nullable(&REASONING_EFFORT)),
    opt("recencyAt", NI64),
    opt("section", Form::Nullable(&Form::Object(&THREAD_SECTION))),
    opt("sectionEnteredAt", NI64),
    req("sessionId", STR),
    req("source", SESSION_SOURCE),
    req("status", Form::Tagged(&THREAD_STATUS)),
    opt("threadSource", NSTR),
    req("turns", Form::Array(&Form::Object(&TURN))),
    req("updatedAt", I64),
]);

// Responses to the four requests the host sends.
pub(super) const INITIALIZE_RESPONSE: Obj = obj(&[
    req("codexHome", STR),
    req("platformFamily", STR),
    req("platformOs", STR),
    req("userAgent", STR),
]);
const GRANULAR: Obj = obj(&[
    req("mcp_elicitations", Form::Bool),
    opt("request_permissions", Form::Bool),
    req("rules", Form::Bool),
    req("sandbox_approval", Form::Bool),
    opt("skill_approval", Form::Bool),
]);
const ASK_FOR_APPROVAL: Form = Form::OneOf(&[
    Form::Words(&["untrusted", "on-request", "never"]),
    Form::Object(&obj(&[req("granular", Form::Object(&GRANULAR))])),
]);
const SANDBOX_POLICY: Tagged = Tagged {
    tag: "type",
    variants: &[
        ("dangerFullAccess", obj(&[])),
        ("readOnly", obj(&[opt("networkAccess", Form::Bool)])),
        ("externalSandbox", obj(&[opt("networkAccess", Form::Words(&["restricted", "enabled"]))])),
        (
            "workspaceWrite",
            obj(&[
                opt("excludeSlashTmp", Form::Bool),
                opt("excludeTmpdirEnvVar", Form::Bool),
                opt("networkAccess", Form::Bool),
                opt("writableRoots", Form::Array(&Form::Str)),
            ]),
        ),
    ],
    refused: &[],
    open: false,
};
const ACTIVE_PERMISSION_PROFILE: Obj = obj(&[opt("extends", NSTR), req("id", STR)]);
pub(super) const THREAD_START_RESPONSE: Obj = obj(&[
    opt("activePermissionProfile", Form::Nullable(&Form::Object(&ACTIVE_PERMISSION_PROFILE))),
    req("approvalPolicy", ASK_FOR_APPROVAL),
    req("approvalsReviewer", Form::Words(&["user", "auto_review", "guardian_subagent"])),
    req("cwd", STR),
    opt("disabledPluginIds", Form::Array(&Form::Str)),
    opt("instructionSources", Form::Array(&Form::Str)),
    req("model", STR),
    req("modelProvider", STR),
    opt(
        "multiAgentMode",
        Form::OneOf(&[
            Form::Words(&["explicitRequestOnly", "proactive"]),
            Form::Object(&obj(&[req("custom", STR)])),
        ]),
    ),
    opt("reasoningEffort", Form::Nullable(&REASONING_EFFORT)),
    opt("runtimeWorkspaceRoots", Form::Array(&Form::Str)),
    req("sandbox", Form::Tagged(&SANDBOX_POLICY)),
    opt("serviceTier", NSTR),
    req("thread", Form::Object(&THREAD)),
]);
pub(super) const TURN_START_RESPONSE: Obj = obj(&[req("turn", Form::Object(&TURN))]);
/// The schema gives this object no members and does not close it; the host
/// admits only `{}` by its own rule that unknown members are refused.
pub(super) const TURN_INTERRUPT_RESPONSE: Obj = obj(&[]);

// Parameters of the notifications the host handles or ignores by name.
const TOKEN_USAGE_BREAKDOWN: Obj = obj(&[
    opt("cacheWriteInputTokens", I64),
    req("cachedInputTokens", I64),
    req("inputTokens", I64),
    req("outputTokens", I64),
    req("reasoningOutputTokens", I64),
    req("totalTokens", I64),
]);
const THREAD_TOKEN_USAGE: Obj = obj(&[
    req("last", Form::Object(&TOKEN_USAGE_BREAKDOWN)),
    opt("modelContextWindow", NI64),
    req("total", Form::Object(&TOKEN_USAGE_BREAKDOWN)),
]);
const CREDITS_SNAPSHOT: Obj = obj(&[opt("balance", NSTR), req("hasCredits", Form::Bool), req("unlimited", Form::Bool)]);
const SPEND_CONTROL_LIMIT_SNAPSHOT: Obj = obj(&[
    req("limit", STR),
    req("remainingPercent", Form::Int("int32")),
    req("resetsAt", I64),
    req("used", STR),
]);
const RATE_LIMIT_WINDOW: Obj = obj(&[
    opt("resetsAt", NI64),
    req("usedPercent", Form::Int("int32")),
    opt("windowDurationMins", NI64),
]);
const RATE_LIMIT_SNAPSHOT: Obj = obj(&[
    opt("credits", Form::Nullable(&Form::Object(&CREDITS_SNAPSHOT))),
    opt("individualLimit", Form::Nullable(&Form::Object(&SPEND_CONTROL_LIMIT_SNAPSHOT))),
    opt("limitId", NSTR),
    opt("limitName", NSTR),
    opt("normalModelSlug", NSTR),
    opt(
        "planType",
        Form::Nullable(&Form::Words(&[
            "free", "go", "plus", "pro", "prolite", "promax", "team", "self_serve_business_prolite",
            "self_serve_business_usage_based", "business", "ent26", "enterprise_cbp_automation",
            "enterprise_cbp_usage_based", "enterprise", "edu", "edu_plus", "edu_pro", "unknown",
        ])),
    ),
    opt("primary", Form::Nullable(&Form::Object(&RATE_LIMIT_WINDOW))),
    opt(
        "rateLimitReachedType",
        Form::Nullable(&Form::Words(&[
            "rate_limit_reached", "workspace_owner_credits_depleted", "workspace_member_credits_depleted",
            "workspace_owner_usage_limit_reached", "workspace_member_usage_limit_reached",
        ])),
    ),
    opt("secondary", Form::Nullable(&Form::Object(&RATE_LIMIT_WINDOW))),
    opt("spendControlReached", NBOOL),
]);
const TEXT_POSITION: Obj = obj(&[req("column", Form::Int("uint")), req("line", Form::Int("uint"))]);
const TEXT_RANGE: Obj = obj(&[req("end", Form::Object(&TEXT_POSITION)), req("start", Form::Object(&TEXT_POSITION))]);

/// The parameters' form of each notification method the host handles or
/// ignores by name; a method absent here never reaches a parameters check
/// (it ends the session by its class).
pub(super) const NOTIFICATION_PARAMS: [(&str, Obj); 18] = [
    ("thread/started", obj(&[req("thread", Form::Object(&THREAD))])),
    ("thread/status/changed", obj(&[req("status", Form::Tagged(&THREAD_STATUS)), req("threadId", STR)])),
    ("turn/started", obj(&[req("threadId", STR), req("turn", Form::Object(&TURN))])),
    ("turn/completed", obj(&[req("threadId", STR), req("turn", Form::Object(&TURN))])),
    (
        "item/started",
        obj(&[req("item", Form::Tagged(&THREAD_ITEM)), req("startedAtMs", I64), req("threadId", STR), req("turnId", STR)]),
    ),
    (
        "item/completed",
        obj(&[req("completedAtMs", I64), req("item", Form::Tagged(&THREAD_ITEM)), req("threadId", STR), req("turnId", STR)]),
    ),
    ("item/agentMessage/delta", obj(&[req("delta", STR), req("itemId", STR), req("threadId", STR), req("turnId", STR)])),
    (
        "error",
        obj(&[req("error", Form::Object(&TURN_ERROR)), req("threadId", STR), req("turnId", STR), req("willRetry", Form::Bool)]),
    ),
    (
        "thread/tokenUsage/updated",
        obj(&[req("threadId", STR), req("tokenUsage", Form::Object(&THREAD_TOKEN_USAGE)), req("turnId", STR)]),
    ),
    ("account/rateLimits/updated", obj(&[req("rateLimits", Form::Object(&RATE_LIMIT_SNAPSHOT))])),
    (
        "remoteControl/status/changed",
        obj(&[
            opt("environmentId", NSTR),
            req("installationId", STR),
            req("serverName", STR),
            req("status", Form::Words(&["disabled", "connecting", "connected", "errored"])),
        ]),
    ),
    (
        "item/reasoning/summaryTextDelta",
        obj(&[req("delta", STR), req("itemId", STR), req("summaryIndex", I64), req("threadId", STR), req("turnId", STR)]),
    ),
    (
        "item/reasoning/summaryPartAdded",
        obj(&[req("itemId", STR), req("summaryIndex", I64), req("threadId", STR), req("turnId", STR)]),
    ),
    (
        "item/reasoning/textDelta",
        obj(&[req("contentIndex", I64), req("delta", STR), req("itemId", STR), req("threadId", STR), req("turnId", STR)]),
    ),
    ("thread/compacted", obj(&[req("threadId", STR), req("turnId", STR)])),
    ("warning", obj(&[req("message", STR), opt("threadId", NSTR)])),
    (
        "configWarning",
        obj(&[
            opt("details", NSTR),
            opt("path", NSTR),
            opt("range", Form::Nullable(&Form::Object(&TEXT_RANGE))),
            req("summary", STR),
        ]),
    ),
    ("deprecationNotice", obj(&[opt("details", NSTR), req("summary", STR)])),
];
fn notification_params(method: &str) -> Option<&'static Obj> {
    NOTIFICATION_PARAMS.iter().find(|(m, _)| *m == method).map(|(_, o)| o)
}
fn response_form(kind: Rpc) -> &'static Obj {
    match kind {
        Rpc::Initialize => &INITIALIZE_RESPONSE,
        Rpc::ThreadStart => &THREAD_START_RESPONSE,
        Rpc::TurnStart => &TURN_START_RESPONSE,
        Rpc::Interrupt => &TURN_INTERRUPT_RESPONSE,
    }
}

const UNOFFERED_ACTIVITY: Error = Error("unoffered supplier tool activity");
const UNLISTED_ITEM: Error = Error("unlisted supplier item kind");
const UNADMITTED_NOTIFICATION: Error = Error("unadmitted supplier notification");
const UNADMITTED_REQUEST: Error = Error("unadmitted supplier request");
const MALFORMED: Error = Error("malformed supplier report");
const FINAL_ERROR_OUTSIDE_TURN: Error = Error("final supplier error with no turn active");

fn formed(v: &Value, o: &Obj) -> Result<(), Error> {
    if form::object(v, o) {
        Ok(())
    } else {
        Err(MALFORMED)
    }
}
/// A string member of a value whose form has been checked.
fn text<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
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
    /// A frame or report not of its form, or one out of order or misbound.
    Malformed,
    /// A final error from the runtime with no turn active.
    SupplierError,
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
        } else if error == FINAL_ERROR_OUTSIDE_TURN {
            Self::SupplierError
        } else {
            Self::Malformed
        }
    }
    pub fn class(self) -> &'static str {
        match self {
            Self::ToolActivity => "runtime_tool_activity",
            Self::Unadmitted => "unadmitted_report",
            Self::Malformed => "malformed_report",
            Self::SupplierError => "supplier_error",
        }
    }
}

/// The 19 item kinds of stock 0.160.0's ThreadItem schema. Kept here so an
/// observation line names only these.
#[cfg(any(test, feature = "native-smoke"))]
pub(super) const SCHEMA_ITEM_KINDS: [&str; 19] = [
    "userMessage", "hookPrompt", "agentMessage", "functionCallOutput", "plan", "reasoning",
    "commandExecution", "fileChange", "mcpToolCall", "dynamicToolCall", "collabAgentToolCall",
    "subAgentActivity", "webSearch", "imageView", "sleep", "imageGeneration",
    "enteredReviewMode", "exitedReviewMode", "contextCompaction",
];

/// Admission of an item whose form has been checked. Admitted: the agent's
/// message, the user's message, reasoning, context compaction, and a call of
/// a tool the catalog offers with no namespace. The runtime's own tool kinds
/// are tool activity; every other kind is unadmitted. This is an observation
/// tripwire, not proof that the operating system's isolation prevented an
/// action.
fn admit(item: &Value) -> Result<(), Error> {
    match text(item, "type") {
        "userMessage" | "agentMessage" | "reasoning" | "contextCompaction" => Ok(()),
        "dynamicToolCall" => {
            let namespaced = item.get("namespace").is_some_and(|n| !n.is_null());
            if Tool::from_name(text(item, "tool")).is_none() || namespaced {
                Err(UNOFFERED_ACTIVITY)
            } else {
                Ok(())
            }
        }
        "commandExecution" | "fileChange" | "mcpToolCall" | "collabAgentToolCall"
        | "subAgentActivity" | "webSearch" | "imageView" | "sleep" | "imageGeneration" => {
            Err(UNOFFERED_ACTIVITY)
        }
        _ => Err(UNLISTED_ITEM),
    }
}
/// Every item of a checked turn, or of each turn of a checked thread.
fn turn_items(turn: &Value) -> impl Iterator<Item = &Value> {
    turn.get("items").and_then(Value::as_array).into_iter().flatten()
}
fn thread_items(thread: &Value) -> impl Iterator<Item = &Value> {
    thread
        .get("turns")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .flat_map(turn_items)
}
/// The items a checked notification's parameters or a checked response's
/// result carry.
fn carried_items<'a>(at: &'a str, v: &'a Value) -> Box<dyn Iterator<Item = &'a Value> + 'a> {
    match at {
        "item/started" | "item/completed" => Box::new(v.get("item").into_iter()),
        "turn/started" | "turn/completed" | "turn/start-response" => {
            Box::new(v.get("turn").into_iter().flat_map(turn_items))
        }
        "thread/started" | "thread/start-response" => {
            Box::new(v.get("thread").into_iter().flat_map(thread_items))
        }
        _ => Box::new(std::iter::empty()),
    }
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
    ("error", Notice::Handled),
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

/// What the front made of a frame: the request, notification or response
/// form it passed, with the class or request kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    ToolCall,
    Notification(Notice),
    Response(Rpc),
    ErrorReply(Rpc),
}

/// Native smoke only: one payload-free line for each distinct item kind,
/// notification method and server request method the runtime reports, read
/// after the frame's form check, so an observed session shows the ordinary
/// set. A kind or method is printed only when it is a name the 0.160.0
/// schema defines; otherwise "unlisted". A frame not of its form gets one
/// fixed line.
#[cfg(any(test, feature = "native-smoke"))]
mod observed {
    use super::{admit, carried_items, Error, Notice, Rpc, Shape, Value, MALFORMED, NOTIFICATIONS, REQUESTS, SCHEMA_ITEM_KINDS};

    const PREFIX: &str = "[native-smoke-agent-host]";
    pub const MAX_LINES: usize = 64;

    fn schema_kind(item: &Value) -> &str {
        let kind = item.get("type").and_then(Value::as_str).unwrap_or("");
        SCHEMA_ITEM_KINDS.iter().find(|k| **k == kind).copied().unwrap_or("unlisted")
    }
    fn schema_method(frame: &Value) -> &str {
        let method = frame.get("method").and_then(Value::as_str).unwrap_or("");
        NOTIFICATIONS
            .iter()
            .chain(REQUESTS.iter())
            .find(|(m, _)| *m == method)
            .map_or("unlisted", |(m, _)| m)
    }
    fn via(kind: Rpc) -> &'static str {
        match kind {
            Rpc::Initialize => "initialize-response",
            Rpc::ThreadStart => "thread/start-response",
            Rpc::TurnStart => "turn/start-response",
            Rpc::Interrupt => "turn/interrupt-response",
        }
    }

    #[derive(Default)]
    pub struct Observed {
        seen: Vec<String>,
        unread: Vec<String>,
        full: bool,
    }
    impl Observed {
        pub fn line(&mut self, line: String) {
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
        fn items(&mut self, at: &str, value: &Value) {
            for item in carried_items(at, value) {
                let decision = if admit(item).is_ok() { "admitted" } else { "refused" };
                let line = format!("{PREFIX} item kind {} via {at} {decision}", schema_kind(item));
                self.line(line);
            }
        }
        /// After the front: only a frame that passed its form is read.
        pub fn frame(&mut self, frame: &Value, checked: &Result<Shape, Error>) {
            let request = frame.get("id").is_some();
            match checked {
                Err(e) if *e == MALFORMED => self.line(format!("{PREFIX} malformed report")),
                Err(_) if frame.get("method").is_some() => {
                    let what = if request { "server request" } else { "notification" };
                    self.line(format!("{PREFIX} {what} {} refused", schema_method(frame)));
                }
                Err(_) | Ok(Shape::ToolCall) | Ok(Shape::ErrorReply(_)) => {}
                Ok(Shape::Notification(class)) => {
                    let method = schema_method(frame);
                    if let Some(params) = frame.get("params") {
                        self.items(method, params);
                    }
                    if *class == Notice::Ignored {
                        self.line(format!("{PREFIX} notification {method} ignored"));
                    }
                }
                Ok(Shape::Response(kind)) => {
                    if let Some(result) = frame.get("result") {
                        self.items(via(*kind), result);
                    }
                }
            }
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
            reconnecting: false,
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
    /// The one front every frame passes before the host decides anything:
    /// the envelope's form, chosen by the members present (a present null is
    /// present), the method's class, and the form of its parameters or of
    /// the result for the request it answers. Pure; `kind_of` names the
    /// outstanding request an identity answers.
    fn check_frame(frame: &Value, kind_of: &dyn Fn(u64) -> Option<Rpc>) -> Result<Shape, Error> {
        let keys = frame.as_object().ok_or(MALFORMED)?;
        if keys.contains_key("method") {
            if keys.contains_key("id") {
                formed(frame, &ENVELOPE_REQUEST)?;
                let method = text(frame, "method");
                if !bounded(method, MAX_ID_BYTES) {
                    return Err(Error("invalid method"));
                }
                match request_class(method) {
                    Notice::Handled => {
                        formed(frame.get("params").unwrap_or(&Value::Null), &DYNAMIC_TOOL_CALL_PARAMS)?;
                        Ok(Shape::ToolCall)
                    }
                    Notice::Ignored => Err(UNADMITTED_REQUEST),
                    Notice::Ends(reason) => Err(reason),
                }
            } else {
                formed(frame, &ENVELOPE_NOTIFICATION)?;
                let method = text(frame, "method");
                if !bounded(method, MAX_ID_BYTES) {
                    return Err(Error("invalid method"));
                }
                match notice(method) {
                    Notice::Ends(reason) => Err(reason),
                    class => {
                        let params = notification_params(method).ok_or(UNADMITTED_NOTIFICATION)?;
                        formed(frame.get("params").unwrap_or(&Value::Null), params)?;
                        Ok(Shape::Notification(class))
                    }
                }
            }
        } else if keys.contains_key("result") {
            formed(frame, &ENVELOPE_RESPONSE)?;
            let kind = Self::answered(frame, kind_of)?;
            formed(&frame["result"], response_form(kind))?;
            Ok(Shape::Response(kind))
        } else {
            formed(frame, &ENVELOPE_ERROR)?;
            let kind = Self::answered(frame, kind_of)?;
            Ok(Shape::ErrorReply(kind))
        }
    }
    fn answered(frame: &Value, kind_of: &dyn Fn(u64) -> Option<Rpc>) -> Result<Rpc, Error> {
        let id = frame["id"].as_u64().ok_or(Error("invalid response identity"))?;
        kind_of(id).ok_or(Error("unexpected response identity"))
    }
    fn handle_inner(&mut self, mut frame: Value, basis: &Value) -> Result<Actions, Error> {
        let mut actions = Actions::default();
        let outstanding = self.outstanding.clone();
        let checked = Self::check_frame(&frame, &|id| {
            outstanding.iter().find(|(n, _)| *n == id).map(|(_, k)| *k)
        });
        #[cfg(any(test, feature = "native-smoke"))]
        self.observed.frame(&frame, &checked);
        let shape = checked?;
        // Anything but a retrying error clears the reconnecting indication.
        let retrying = frame.get("method").and_then(Value::as_str) == Some("error")
            && frame.pointer("/params/willRetry") == Some(&json!(true));
        let clear = self.reconnecting && !retrying;
        let object = frame.as_object_mut().ok_or(MALFORMED)?;
        match shape {
            Shape::ToolCall => {
                let id = rpc_id(object.get("id").unwrap_or(&Value::Null))?;
                let params = object.remove("params").unwrap_or(Value::Null);
                self.tool_call(id, params, basis, &mut actions)?;
            }
            Shape::Notification(_) => {
                let method = object
                    .get("method")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_owned();
                let params = object.remove("params").unwrap_or(Value::Null);
                self.notification(&method, params, &mut actions)?;
            }
            Shape::Response(kind) => {
                let id = object.get("id").and_then(Value::as_u64).ok_or(MALFORMED)?;
                let result = object.remove("result").unwrap_or(Value::Null);
                self.response(kind, &result, &mut actions)?;
                let pos = self.outstanding.iter().position(|(n, _)| *n == id).ok_or(MALFORMED)?;
                self.outstanding.remove(pos);
            }
            Shape::ErrorReply(kind) => {
                let error = object.get("error").cloned().unwrap_or(Value::Null);
                self.last_rejection = Some(rejection(kind, &error));
                return Err(Error("client request failed"));
            }
        }
        if clear {
            self.reconnecting = false;
            actions.events.insert(0, Event::Reconnecting(false));
        }
        Ok(actions)
    }
    fn response(&mut self, kind: Rpc, result: &Value, actions: &mut Actions) -> Result<(), Error> {
        match kind {
            Rpc::Initialize if self.phase == Phase::Initializing => {
                self.phase = Phase::Ready;
                actions
                    .outbound
                    .push(json!({"method":"initialized","params":{}}));
                actions.events.push(Event::Initialized);
            }
            Rpc::ThreadStart if self.phase == Phase::StartingThread => {
                carried_items("thread/start-response", result).try_for_each(admit)?;
                let thread = &result["thread"];
                let id = bounded_identity(text(thread, "id"))?;
                if self.thread.as_ref().is_some_and(|v| v != &id) {
                    return Err(Error("thread response mismatch"));
                }
                let sandbox = &result["sandbox"];
                if result["approvalPolicy"] != json!("never")
                    || text(sandbox, "type") != "readOnly"
                    || sandbox.get("networkAccess").is_some_and(|v| v != &json!(false))
                    || thread["ephemeral"] != json!(true)
                    || thread
                        .get("environments")
                        .is_some_and(|e| !e.is_null() && e.as_array().map_or(true, |a| !a.is_empty()))
                {
                    return Err(Error("unexpected thread policy"));
                }
                self.thread = Some(id.clone());
                self.phase = Phase::ThreadReady;
                actions.events.push(Event::ThreadStarted { thread_id: id });
            }
            Rpc::TurnStart if matches!(self.phase, Phase::StartingTurn | Phase::InTurn) => {
                carried_items("turn/start-response", result).try_for_each(admit)?;
                let turn = &result["turn"];
                let id = bounded_identity(text(turn, "id"))?;
                if text(turn, "status") != "inProgress" || self.turn.as_ref().is_some_and(|v| v != &id) {
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
    /// `p`'s form has been checked for `method`, which is handled or ignored.
    fn notification(&mut self, method: &str, p: Value, actions: &mut Actions) -> Result<(), Error> {
        carried_items(method, &p).try_for_each(admit)?;
        match method {
            "turn/started" => {
                let turn = &p["turn"];
                self.check_thread(text(&p, "threadId"))?;
                if !matches!(self.phase, Phase::StartingTurn | Phase::InTurn)
                    || text(turn, "status") != "inProgress"
                {
                    return Err(Error("unexpected turn start"));
                }
                let id = bounded_identity(text(turn, "id"))?;
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
                let turn = &p["turn"];
                self.check_thread(text(&p, "threadId"))?;
                if self.phase != Phase::InTurn
                    || Some(bounded_identity(text(turn, "id"))?.as_str()) != self.turn.as_deref()
                    || self.outstanding.iter().any(|(_, k)| *k == Rpc::TurnStart)
                {
                    return Err(Error("unsupported turn completion ordering"));
                }
                let status = text(turn, "status");
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
                self.check_turn(text(&p, "threadId"), text(&p, "turnId"))?;
                let item_id = bounded_identity(text(&p, "itemId"))?;
                let delta = text(&p, "delta");
                if delta.len() > MAX_TEXT_BYTES {
                    return Err(Error("message text limit"));
                }
                actions.events.push(Event::MessageDelta { item_id, text: delta.to_owned() });
            }
            // Its item was admitted above; nothing else is handled.
            "item/started" => {}
            "item/completed" => {
                self.check_turn(text(&p, "threadId"), text(&p, "turnId"))?;
                let item = &p["item"];
                if text(item, "type") == "agentMessage" {
                    let item_id = bounded_identity(text(item, "id"))?;
                    let message = text(item, "text");
                    if message.len() > MAX_TEXT_BYTES {
                        return Err(Error("message text limit"));
                    }
                    actions.events.push(Event::MessageFinal { item_id, text: message.to_owned() });
                }
            }
            "thread/status/changed" => {
                let thread_id = text(&p, "threadId");
                if self.phase == Phase::StartingThread && self.thread.is_none() {
                    self.thread = Some(bounded_identity(thread_id)?);
                }
                self.check_thread(thread_id)?;
                actions.events.push(Event::ThreadStatus {
                    status: text(&p["status"], "type").to_owned(),
                });
            }
            // The supplier may send thread/started before its response. It is
            // informational only; response supplies authoritative thread id.
            "thread/started" => {
                if self.phase == Phase::StartingThread {
                    let id = bounded_identity(text(&p["thread"], "id"))?;
                    if self.thread.as_deref().is_some_and(|old| old != id) {
                        return Err(Error("thread mismatch"));
                    }
                    self.thread = Some(id);
                }
            }
            // The runtime's error report, whose form was checked. Inside an
            // active turn it changes no session state: the turn ends by its
            // completion or the person's interrupt. A retrying error is shown
            // as reconnecting until anything else arrives. With no turn
            // active, a final error ends the session; a retrying one changes
            // nothing. The error's turn identifier is not matched, and its
            // message is never carried.
            "error" => {
                // Tied to this session by its thread, as other notifications
                // are; its turn identifier is not matched.
                self.check_thread(text(&p, "threadId"))?;
                let active = matches!(self.phase, Phase::StartingTurn | Phase::InTurn);
                let retrying = p["willRetry"] == json!(true);
                match (active, retrying) {
                    (true, true) => {
                        if !self.reconnecting {
                            self.reconnecting = true;
                            actions.events.push(Event::Reconnecting(true));
                        }
                    }
                    (true, false) | (false, true) => {}
                    (false, false) => return Err(FINAL_ERROR_OUTSIDE_TURN),
                }
            }
            // Ignored by name: its parameters' form was checked and carries
            // nothing the host acts on.
            _ => {}
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
        self.check_turn(text(&p, "threadId"), text(&p, "turnId"))?;
        let call_id = bounded_identity(text(&p, "callId"))?;
        bounded_identity(text(&p, "tool"))?;
        if self.seen_rpc.contains(&id) || self.seen_call.contains(&call_id) {
            return Err(Error("duplicate tool identity"));
        }
        if self.seen_rpc.len() >= MAX_CALLS_PER_TURN || self.seen_call.len() >= MAX_CALLS_PER_TURN {
            return Err(Error("turn identity limit"));
        }
        // The supplier forwards a call only for a tool the host declared at
        // thread start, none of them namespaced; anything else means it is
        // not behaving as pinned, and the session ends.
        let namespaced = p.get("namespace").is_some_and(|n| !n.is_null());
        let tool = match Tool::from_name(text(&p, "tool")) {
            Some(tool) if !namespaced => tool,
            _ => return Err(UNOFFERED_ACTIVITY),
        };
        self.seen_rpc.push(id.clone());
        self.seen_call.push(call_id.clone());
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
            assert_ends(p, reference, frame, if method.is_empty() { Error("invalid method") } else { reason });
        }
    }

    #[test]
    fn ignored_notifications_are_read_whole_and_carry_no_item() {
        let accepted = [
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
        // The supplier writes no trace member on a server request (its
        // source at the pinned tag); one is refused like any unlisted member.
        for trace in [json!(null), json!({}), json!({"traceparent":null})] {
            let (p, reference) = active_with_pending();
            let mut c = call_summary(json!(8), "traced");
            c["trace"] = trace;
            assert_ends(p, reference, c, MALFORMED);
        }
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
            assert_eq!(p.take_observations(), ["[native-smoke-agent-host] malformed report"]);
        }
        let mut p = active();
        p.take_observations();
        assert!(p.handle_frame(item_frame("item/started", json!(marker)), &basis()).is_err());
        assert_eq!(p.take_observations(), ["[native-smoke-agent-host] malformed report"]);
    }

    #[test]
    fn observation_lines_are_bounded_per_session() {
        let mut o = observed::Observed::default();
        for (method, _) in NOTIFICATIONS {
            o.line(format!("[native-smoke-agent-host] notification {method} ignored"));
        }
        let lines = o.take();
        assert_eq!(lines.len(), observed::MAX_LINES + 1);
        assert_eq!(lines.last().unwrap(), "[native-smoke-agent-host] observation limit reached");
        for (method, _) in NOTIFICATIONS {
            o.line(format!("[native-smoke-agent-host] notification {method} refused"));
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
        assert_eq!(count(|c| *c == Notice::Handled), 8);
        assert_eq!(count(|c| *c == Notice::Ignored), 10);
        assert_eq!(count(|c| matches!(c, Notice::Ends(_))), 65);
        assert_eq!(REQUESTS.iter().filter(|(_, c)| *c == Notice::Handled).count(), 1);
    }

    fn error_frame(turn: &str, retry: bool) -> Value {
        notification(
            "error",
            json!({"error":{"message":"supplier text","codexErrorInfo":{"responseStreamDisconnected":{"httpStatusCode":502}}},"threadId":"thread-1","turnId":turn,"willRetry":retry}),
        )
    }

    #[test]
    fn a_malformed_error_ends_the_session() {
        for params in [
            json!({"error":{"message":"m"},"threadId":"thread-1","turnId":"turn-1"}),
            json!({"error":{"message":"m"},"threadId":"thread-1","turnId":"turn-1","willRetry":"no"}),
            json!({"error":{"message":1},"threadId":"thread-1","turnId":"turn-1","willRetry":false}),
            json!({"error":{},"threadId":"thread-1","turnId":"turn-1","willRetry":false}),
            json!({"error":{"message":"m","note":"x"},"threadId":"thread-1","turnId":"turn-1","willRetry":false}),
            json!({"error":{"message":"m","codexErrorInfo":{"invented":{}}},"threadId":"thread-1","turnId":"turn-1","willRetry":true}),
            json!({"error":{"message":"m"},"threadId":"thread-1","turnId":"turn-1","willRetry":true,"item":{"type":"commandExecution"}}),
            json!({"error":"m","threadId":"thread-1","turnId":"turn-1","willRetry":true}),
        ] {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, notification("error", params), MALFORMED);
        }
    }

    #[test]
    fn a_retrying_error_inside_a_turn_changes_no_state_and_shows_reconnecting_until_anything_else() {
        let (mut p, reference) = active_with_pending();
        assert!(matches!(
            p.handle_frame(error_frame("turn-1", true), &basis()).unwrap().events.as_slice(),
            [Event::Reconnecting(true)]
        ));
        // Repeated retries add nothing; the turn and its pending call stand.
        assert!(p.handle_frame(error_frame("turn-1", true), &basis()).unwrap().events.is_empty());
        assert_eq!(p.phase, Phase::InTurn);
        // Anything else in the turn clears the indication first.
        assert!(matches!(
            p.handle_frame(notification("item/agentMessage/delta", json!({"threadId":"thread-1","turnId":"turn-1","itemId":"m","delta":"x"})), &basis())
                .unwrap()
                .events
                .as_slice(),
            [Event::Reconnecting(false), Event::MessageDelta { .. }]
        ));
        assert!(matches!(
            p.handle_frame(error_frame("turn-1", true), &basis()).unwrap().events.as_slice(),
            [Event::Reconnecting(true)]
        ));
        // The turn's end clears it, before the completion.
        assert!(matches!(
            p.handle_frame(notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn("turn-1", "failed")})), &basis())
                .unwrap()
                .events
                .as_slice(),
            [Event::Reconnecting(false), Event::ToolDetached { .. }, Event::TurnCompleted { .. }]
        ));
        assert!(matches!(
            p.complete(reference.as_str(), outcome("unavailable")).unwrap(),
            Completion::Detached
        ));
    }

    #[test]
    fn a_final_error_inside_a_turn_changes_no_state_and_its_completion_ends_the_turn_failed() {
        let (mut p, _reference) = active_with_pending();
        let actions = p.handle_frame(error_frame("turn-1", false), &basis()).unwrap();
        assert!(actions.events.is_empty() && actions.outbound.is_empty());
        assert_eq!(p.phase, Phase::InTurn);
        // The turn still takes calls until its completion.
        assert!(matches!(
            p.handle_frame(call_summary(json!(8), "after-error"), &basis()).unwrap().events.as_slice(),
            [Event::ToolRequested { .. }]
        ));
        let events = p
            .handle_frame(notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn("turn-1", "failed")})), &basis())
            .unwrap()
            .events;
        assert!(matches!(events.last(), Some(Event::TurnCompleted { status, .. }) if status == "failed"));
        assert_eq!(p.phase, Phase::ThreadReady);
    }

    #[test]
    fn a_final_error_with_no_turn_active_ends_the_session_with_its_own_reason() {
        let mut p = ready();
        assert_eq!(p.handle_frame(error_frame("turn-1", false), &basis()).err(), Some(FINAL_ERROR_OUTSIDE_TURN));
        assert_eq!(p.refusal(), Some(Refusal::SupplierError));
        assert_eq!(Refusal::SupplierError.class(), "supplier_error");
        assert!(p.submit_text("next".into()).is_err());
        assert!(p.disconnect().is_empty());
        // After a turn has completed, the same.
        let mut q = active();
        q.handle_frame(notification("turn/completed", json!({"threadId":"thread-1","turn":fixture::turn("turn-1", "completed")})), &basis()).unwrap();
        assert_eq!(q.handle_frame(error_frame("turn-1", false), &basis()).err(), Some(FINAL_ERROR_OUTSIDE_TURN));
    }

    #[test]
    fn a_retrying_error_with_no_turn_active_changes_nothing() {
        let mut p = ready();
        let actions = p.handle_frame(error_frame("turn-1", true), &basis()).unwrap();
        assert!(actions.events.is_empty() && actions.outbound.is_empty());
        assert_eq!(p.phase, Phase::ThreadReady);
        assert!(p.submit_text("next".into()).is_ok());
    }

    #[test]
    fn an_error_is_not_matched_on_its_turn_identifier() {
        for retry in [false, true] {
            let outcomes: Vec<String> = ["turn-1", "another-turn", ""]
                .iter()
                .map(|turn| {
                    let mut p = active();
                    let actions = p.handle_frame(error_frame(turn, retry), &basis()).unwrap();
                    format!("{:?} {:?}", actions.events, p.phase)
                })
                .collect();
            assert!(outcomes.windows(2).all(|w| w[0] == w[1]), "{outcomes:?}");
            let outcomes: Vec<String> = ["turn-1", "another-turn"]
                .iter()
                .map(|turn| format!("{:?}", ready().handle_frame(error_frame(turn, retry), &basis()).err()))
                .collect();
            assert!(outcomes.windows(2).all(|w| w[0] == w[1]), "{outcomes:?}");
        }
    }

    #[test]
    fn an_error_is_tied_to_this_session_by_its_thread() {
        for retry in [false, true] {
            let mut frame = error_frame("turn-1", retry);
            frame["params"]["threadId"] = json!("another-thread");
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, frame.clone(), Error("thread mismatch"));
            let mut q = ready();
            assert_eq!(q.handle_frame(frame, &basis()).err(), Some(Error("thread mismatch")));
        }
    }

    #[test]
    fn round_three_frames_are_refused_by_their_form() {
        let mut final_phase = fixture::agent_message("m", "hello");
        final_phase["phase"] = json!({"final_answer":null});
        let mut dynamic = fixture::dynamic_tool_call("d", Tool::Summary.name());
        dynamic["status"] = json!({"completed":null});
        let mut empty_effort = fixture::thread("thread-1");
        empty_effort["reasoningEffort"] = json!("");
        for frame in [
            item_frame("item/completed", final_phase),
            item_frame("item/completed", dynamic),
            json!({"method":"item/completed","params":[1,fixture::agent_message("m", "hello"),"thread-1","turn-1"]}),
            json!({"method":"item/completed","params":fixture::item_params("item/completed", "thread-1", "turn-1", fixture::agent_message("m", "hello")),"result":null}),
            notification("turn/completed", json!({"threadId":"thread-1","turn":{"id":"turn-1","items":[],"status":{"completed":null}}})),
            notification("thread/started", json!({"thread":empty_effort})),
        ] {
            let (p, reference) = active_with_pending();
            assert_ends(p, reference, frame, MALFORMED);
        }
        // An interrupt reply with params null.
        let (mut p, reference) = active_with_pending();
        p.interrupt().unwrap();
        assert_ends(p, reference, json!({"id":4,"params":null,"result":{}}), MALFORMED);
        // A thread start reply with an empty reasoning effort.
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
        result["reasoningEffort"] = json!("");
        assert_eq!(q.handle_frame(frame(2, result), &basis()).err(), Some(MALFORMED));
    }

    // --- The sweep: every member of every form, every other JSON kind. ---

    /// Whether a form admits a replacement value. Written apart from the
    /// checker, from the forms alone, for the plain values the sweep uses.
    fn admits(f: &Form, r: &Value) -> bool {
        match f {
            Form::Any => true,
            Form::Nullable(inner) => r.is_null() || admits(inner, r),
            Form::Str => r.is_string(),
            Form::StrMin(min) => r.as_str().is_some_and(|s| s.chars().count() >= *min),
            Form::Int(format) => {
                r.is_i64() && !r.is_f64() && (r.as_i64().unwrap_or(-1) >= 0 || matches!(*format, "int64" | "int32"))
            }
            Form::Bool => r.is_boolean(),
            Form::Words(words) => r.as_str().is_some_and(|s| words.contains(&s)),
            Form::Array(_) => r.as_array().is_some_and(Vec::is_empty),
            Form::Object(o) => {
                r.as_object().is_some_and(|m| m.is_empty())
                    && o.members.iter().all(|m| !m.required)
                    && o.any_required.is_empty()
            }
            Form::Tagged(_) => false,
            Form::OneOf(alternatives) => alternatives.iter().filter(|a| admits(a, r)).count() == 1,
        }
    }
    fn replacements() -> Vec<Value> {
        vec![json!(null), json!(true), json!(0), json!(-1), json!(1.5), json!("x"), json!(""), json!([]), json!({})]
    }

    /// Valid values of a form with every member present: one for each
    /// variant or alternative reachable, varying one place at a time.
    fn samples(f: &Form) -> Vec<Value> {
        match f {
            Form::Str => vec![json!("s")],
            Form::StrMin(min) => vec![json!("s".repeat((*min).max(1)))],
            Form::Int(_) => vec![json!(1)],
            Form::Bool => vec![json!(true)],
            Form::Any => vec![json!({"any":[1]})],
            Form::Words(words) => vec![json!(words[0])],
            Form::Array(item) => samples(item).into_iter().map(|v| json!([v])).collect(),
            Form::Nullable(inner) => samples(inner),
            Form::Object(o) => object_samples(o, None),
            Form::Tagged(t) => t
                .variants
                .iter()
                .flat_map(|(word, o)| object_samples(o, Some((t.tag, word))))
                .collect(),
            Form::OneOf(alternatives) => alternatives.iter().flat_map(samples).collect(),
        }
    }
    fn object_samples(o: &Obj, tag: Option<(&str, &str)>) -> Vec<Value> {
        let mut base = serde_json::Map::new();
        if let Some((tag, word)) = tag {
            base.insert(tag.into(), json!(word));
        }
        let per_member: Vec<Vec<Value>> = o.members.iter().map(|m| samples(&m.form)).collect();
        for (m, values) in o.members.iter().zip(&per_member) {
            base.insert(m.name.into(), values[0].clone());
        }
        let mut out = vec![Value::Object(base.clone())];
        for (m, values) in o.members.iter().zip(&per_member) {
            for v in &values[1..] {
                let mut variant = base.clone();
                variant.insert(m.name.into(), v.clone());
                out.push(Value::Object(variant));
            }
        }
        out
    }
    /// Every mutation of `v`, a valid value of `f`, at `v` and below, with
    /// whether the form admits it.
    fn mutations(f: &Form, v: &Value) -> Vec<(Value, bool)> {
        let mut out: Vec<(Value, bool)> = replacements()
            .into_iter()
            .filter(|r| r != v)
            .map(|r| {
                let ok = admits(f, &r);
                (r, ok)
            })
            .collect();
        if let Some(map) = v.as_object() {
            out.push((Value::Array(map.values().cloned().collect()), matches!(f, Form::Any)));
        }
        out.extend(inner_mutations(f, v));
        out
    }
    /// Mutations below `v`, judged by the form that governs each place.
    fn inner_mutations(f: &Form, v: &Value) -> Vec<(Value, bool)> {
        let mut out = Vec::new();
        match f {
            Form::Nullable(inner) => out.extend(inner_mutations(inner, v)),
            Form::Array(item) => {
                for (m, ok) in mutations(item, &v[0]) {
                    out.push((json!([m]), ok));
                }
            }
            Form::Object(o) => out.extend(member_mutations(o, v, None)),
            Form::Tagged(t) => {
                let word = v[t.tag].as_str().unwrap_or("");
                let (_, o) = t.variants.iter().find(|(w, _)| *w == word).expect("variant sample");
                out.extend(member_mutations(o, v, Some(t)));
            }
            Form::OneOf(alternatives) => {
                if let Some(alt) = alternatives.iter().find(|a| form::check(v, a)) {
                    out.extend(inner_mutations(alt, v));
                }
            }
            _ => {}
        }
        out
    }
    fn member_mutations(o: &Obj, v: &Value, tagged: Option<&Tagged>) -> Vec<(Value, bool)> {
        let map = v.as_object().expect("object sample");
        let mut out = Vec::new();
        let with = |key: &str, value: Option<Value>| {
            let mut m = map.clone();
            match value {
                Some(value) => m.insert(key.into(), value),
                None => m.remove(key),
            };
            Value::Object(m)
        };
        if let Some(t) = tagged {
            for r in replacements() {
                let ok = r.as_str().is_some_and(|w| t.open || t.refused.contains(&w) || t.variants.iter().any(|(v, _)| *v == w));
                out.push((with(t.tag, Some(r)), ok));
            }
            out.push((with(t.tag, None), false));
        }
        for m in o.members {
            let Some(current) = map.get(m.name) else { continue };
            let still_any = |gone: &str| {
                o.any_required.is_empty()
                    || o.any_required.iter().any(|set| set.iter().all(|n| *n != gone && map.contains_key(*n)))
            };
            out.push((with(m.name, None), !m.required && still_any(m.name)));
            for (mutated, ok) in mutations(&m.form, current) {
                out.push((with(m.name, Some(mutated)), ok));
            }
        }
        out.push((with("unlistedMember", Some(json!(1))), false));
        out
    }
    /// The roots the front checks, each with a frame builder and the kind of
    /// request its identity answers.
    fn sweep_roots() -> Vec<(String, &'static Obj, Box<dyn Fn(Value) -> Value>)> {
        let mut roots: Vec<(String, &'static Obj, Box<dyn Fn(Value) -> Value>)> = Vec::new();
        for (method, params) in NOTIFICATION_PARAMS.iter() {
            let method = *method;
            roots.push((format!("notification:{method}"), params, Box::new(move |p| json!({"method":method,"params":p}))));
        }
        roots.push(("request:item/tool/call".into(), &DYNAMIC_TOOL_CALL_PARAMS, Box::new(|p| json!({"id":"r","method":"item/tool/call","params":p}))));
        for (name, id, form) in [
            ("response:initialize", 1, &INITIALIZE_RESPONSE),
            ("response:thread/start", 2, &THREAD_START_RESPONSE),
            ("response:turn/start", 3, &TURN_START_RESPONSE),
            ("response:turn/interrupt", 4, &TURN_INTERRUPT_RESPONSE),
        ] {
            roots.push((name.into(), form, Box::new(move |r| json!({"id":id,"result":r}))));
        }
        roots.push(("envelope:error".into(), &JSONRPC_ERROR_ERROR, Box::new(|e| json!({"id":1,"error":e}))));
        roots
    }
    fn sweep_kind(id: u64) -> Option<Rpc> {
        match id {
            1 => Some(Rpc::Initialize),
            2 => Some(Rpc::ThreadStart),
            3 => Some(Rpc::TurnStart),
            4 => Some(Rpc::Interrupt),
            _ => None,
        }
    }

    #[test]
    fn the_sweep_finds_no_leniency_in_the_front() {
        let mut cases = 0usize;
        let mut refused = 0usize;
        let mut failures = Vec::new();
        for (name, form, wrap) in sweep_roots() {
            let root = Form::Object(form);
            for sample in samples(&root) {
                let base = Protocol::check_frame(&wrap(sample.clone()), &sweep_kind);
                if base == Err(MALFORMED) {
                    failures.push(format!("{name}: valid sample refused: {sample}"));
                }
                for (mutated, ok) in mutations(&root, &sample) {
                    cases += 1;
                    let frame = wrap(mutated);
                    let outcome = Protocol::check_frame(&frame, &sweep_kind);
                    // Admitted by the form: never refused as malformed.
                    // Not admitted: refused as malformed.
                    let pass = if ok { outcome != Err(MALFORMED) } else { outcome == Err(MALFORMED) };
                    if !ok {
                        refused += 1;
                    }
                    if !pass {
                        failures.push(format!("{name}: form admits={ok} but {outcome:?}: {frame}"));
                    }
                }
            }
        }
        // Envelope members: each envelope form, its members mutated.
        let envelopes = [
            json!({"method":"warning","params":{"message":"m"}}),
            json!({"id":"r","method":"item/tool/call","params":{"threadId":"t","turnId":"u","callId":"c","tool":"x","arguments":{}}}),
            json!({"id":4,"result":{}}),
            json!({"id":1,"error":{"code":1,"message":"m"}}),
        ];
        for envelope in envelopes {
            let map = envelope.as_object().unwrap().clone();
            let mut variants: Vec<Value> = vec![Value::Array(map.values().cloned().collect())];
            for extra in ["jsonrpc", "trace", "params", "result", "error", "id", "method", "x"] {
                if !map.contains_key(extra) {
                    let mut m = map.clone();
                    m.insert(extra.into(), json!(null));
                    variants.push(Value::Object(m));
                }
            }
            for key in map.keys() {
                let mut m = map.clone();
                m.remove(key);
                variants.push(Value::Object(m));
                for r in replacements() {
                    if key == "id" && (r.is_string() || r.as_i64().is_some())
                        || key == "method" && r.is_string()
                        || map.get(key) == Some(&r)
                    {
                        continue;
                    }
                    let mut m = map.clone();
                    m.insert(key.clone(), r);
                    variants.push(Value::Object(m));
                }
            }
            for frame in variants {
                cases += 1;
                refused += 1;
                if Protocol::check_frame(&frame, &sweep_kind).is_ok() {
                    failures.push(format!("envelope admitted: {frame}"));
                }
            }
        }
        println!("sweep: {cases} cases, {refused} refused by form");
        assert!(failures.is_empty(), "{} of {cases}:\n{}", failures.len(), failures.join("\n"));
        assert!(cases > 1000 && refused > 1000, "{cases} cases, {refused} refused");
    }

    // --- Fidelity: the canonical text of the forms and its hash. ---
    //
    // The canonical text of a form, the rule by which a reviewer recomputes
    // the hash from the schema copy:
    //   str | str(min=N) for minLength | int(FORMAT) with the schema's
    //   format | bool | any for `true` | words(w1,w2,...) for an enum or a
    //   oneOf of string enums, words sorted | array(F) | nullable(F) for a
    //   type list with "null" or an anyOf of F and {"type":"null"} |
    //   object{name:req F;name:opt F;...} members sorted by name, "req" when
    //   in `required`, optionally followed by anyof-required[a|b] for an
    //   anyOf of required sets (members of those sets added as opt; sets
    //   sorted, members within a set joined by ",") |
    //   tagged(TAG){word=object{...};...} for a oneOf whose alternatives are
    //   all objects with a single-word `type`, variants sorted by word, the
    //   tag not among the members, followed by refused[w,...] and open for
    //   ThreadItem only, whose five admitted kinds (agentMessage,
    //   contextCompaction, dynamicToolCall, reasoning, userMessage) are
    //   rendered in full and the other fourteen listed by tag, sorted |
    //   oneof(F|F|...) for any other oneOf or anyOf, alternatives sorted.
    //   `$ref` and a single `allOf` are followed; description, title and
    //   default are ignored.
    // The text is one line per root, `NAME = FORM`, sorted by name, each
    // ending in "\n". The roots: envelope:error (JSONRPCError),
    // envelope:notification (JSONRPCNotification with params required, as
    // every ServerNotification variant requires it), envelope:request
    // (JSONRPCRequest's id, method and params, all required; the supplier
    // writes no trace member), envelope:response (JSONRPCResponse),
    // notification:METHOD (the params of ServerNotification's METHOD variant,
    // for the 18 handled or ignored methods), request:item/tool/call
    // (DynamicToolCallParams), response:initialize (InitializeResponse),
    // response:thread/start, response:turn/start, response:turn/interrupt
    // (v2 ThreadStartResponse, TurnStartResponse, TurnInterruptResponse).
    // Its SHA-256, computed once from the schema copy (SHA-256 below) by
    // `subset_hash_from_schema`:
    const SCHEMA_SUBSET_SHA256: &str = "afe71c99f8b8dc8a278588e66d9cc512d9a2d5b5b84323ee8ee3952ed2d913ac";

    fn canon(f: &Form) -> String {
        match f {
            Form::Str => "str".into(),
            Form::StrMin(min) => format!("str(min={min})"),
            Form::Int(format) => format!("int({format})"),
            Form::Bool => "bool".into(),
            Form::Any => "any".into(),
            Form::Words(words) => {
                let mut w = words.to_vec();
                w.sort_unstable();
                format!("words({})", w.join(","))
            }
            Form::Array(item) => format!("array({})", canon(item)),
            Form::Nullable(inner) => format!("nullable({})", canon(inner)),
            Form::Object(o) => canon_object(o),
            Form::Tagged(t) => {
                let mut variants: Vec<String> =
                    t.variants.iter().map(|(word, o)| format!("{word}={}", canon_object(o))).collect();
                variants.sort();
                let mut text = format!("tagged({}){{{}}}", t.tag, variants.join(";"));
                if !t.refused.is_empty() {
                    let mut r = t.refused.to_vec();
                    r.sort_unstable();
                    text.push_str(&format!("refused[{}]", r.join(",")));
                }
                if t.open {
                    text.push_str("open");
                }
                text
            }
            Form::OneOf(alternatives) => {
                let mut a: Vec<String> = alternatives.iter().map(canon).collect();
                a.sort();
                format!("oneof({})", a.join("|"))
            }
        }
    }
    fn canon_object(o: &Obj) -> String {
        let mut members: Vec<String> = o
            .members
            .iter()
            .map(|m| format!("{}:{} {}", m.name, if m.required { "req" } else { "opt" }, canon(&m.form)))
            .collect();
        members.sort();
        let mut text = format!("object{{{}}}", members.join(";"));
        if !o.any_required.is_empty() {
            let mut sets: Vec<String> = o
                .any_required
                .iter()
                .map(|set| {
                    let mut s = set.to_vec();
                    s.sort_unstable();
                    s.join(",")
                })
                .collect();
            sets.sort();
            text.push_str(&format!("anyof-required[{}]", sets.join("|")));
        }
        text
    }
    fn canonical_text() -> String {
        let mut roots: Vec<(String, String)> = vec![
            ("envelope:error".into(), canon_object(&ENVELOPE_ERROR)),
            ("envelope:notification".into(), canon_object(&ENVELOPE_NOTIFICATION)),
            ("envelope:request".into(), canon_object(&ENVELOPE_REQUEST)),
            ("envelope:response".into(), canon_object(&ENVELOPE_RESPONSE)),
            ("request:item/tool/call".into(), canon_object(&DYNAMIC_TOOL_CALL_PARAMS)),
            ("response:initialize".into(), canon_object(&INITIALIZE_RESPONSE)),
            ("response:thread/start".into(), canon_object(&THREAD_START_RESPONSE)),
            ("response:turn/interrupt".into(), canon_object(&TURN_INTERRUPT_RESPONSE)),
            ("response:turn/start".into(), canon_object(&TURN_START_RESPONSE)),
        ];
        for (method, params) in NOTIFICATION_PARAMS.iter() {
            roots.push((format!("notification:{method}"), canon_object(params)));
        }
        roots.sort();
        roots.iter().map(|(name, form)| format!("{name} = {form}\n")).collect()
    }
    fn sha256_hex(text: &str) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(text.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn the_forms_render_to_the_schema_subset_hash() {
        assert_eq!(sha256_hex(&canonical_text()), SCHEMA_SUBSET_SHA256, "{}", canonical_text());
    }

    /// The development script for the hash, run by hand only, never by a
    /// build or a hosted check:
    ///   SWBPIPE_AGENT_SCHEMA=<path to codex_app_server_protocol.schemas.json> \
    ///   cargo test --lib embedded_agent::protocol::tests::subset_hash_from_schema -- --ignored --nocapture
    /// It renders the schema's covered part by the rule above, prints the
    /// text and its SHA-256, and compares it with the forms' text.
    #[test]
    #[ignore]
    fn subset_hash_from_schema() {
        let path = std::env::var("SWBPIPE_AGENT_SCHEMA").expect("SWBPIPE_AGENT_SCHEMA");
        let bytes = std::fs::read(&path).expect("schema");
        let schema: Value = serde_json::from_slice(&bytes).expect("schema JSON");
        let text = schema_text(&schema);
        println!("schema file sha256 {}", sha256_hex(&String::from_utf8_lossy(&bytes)));
        println!("{text}subset sha256 {}", sha256_hex(&text));
        let ours = canonical_text();
        for (a, b) in text.lines().zip(ours.lines()) {
            if a != b {
                println!("SCHEMA {a}\nFORMS  {b}");
            }
        }
        assert_eq!(text, ours);
    }
    fn resolve<'a>(schema: &'a Value, node: &'a Value) -> &'a Value {
        match node.get("$ref").and_then(Value::as_str) {
            Some(r) => resolve(schema, schema.pointer(&r[1..]).expect("ref")),
            None => node,
        }
    }
    fn is_null_schema(n: &Value) -> bool {
        n.get("type") == Some(&json!("null"))
    }
    fn sch(schema: &Value, node: &Value) -> String {
        if node == &json!(true) {
            return "any".into();
        }
        if let Some(r) = node.get("$ref").and_then(Value::as_str) {
            if r.ends_with("/ThreadItem") {
                return thread_item(schema, resolve(schema, node));
            }
            return sch(schema, resolve(schema, node));
        }
        if let Some(all) = node.get("allOf").and_then(Value::as_array) {
            assert_eq!(all.len(), 1);
            return sch(schema, &all[0]);
        }
        if node.get("properties").is_some() || node.get("type") == Some(&json!("object")) {
            return sch_object(schema, node, None);
        }
        if let Some(any) = node.get("anyOf").and_then(Value::as_array) {
            let rest: Vec<&Value> = any.iter().filter(|a| !is_null_schema(a)).collect();
            if rest.len() == 1 && rest.len() < any.len() {
                return format!("nullable({})", sch(schema, rest[0]));
            }
            let mut alts: Vec<String> = rest.iter().map(|a| sch(schema, a)).collect();
            alts.sort();
            return format!("oneof({})", alts.join("|"));
        }
        if let Some(one) = node.get("oneOf").and_then(Value::as_array) {
            let alts: Vec<&Value> = one.iter().map(|a| resolve(schema, a)).collect();
            if alts.iter().all(|a| a.get("enum").is_some()) {
                let mut w: Vec<&str> = alts.iter().flat_map(|a| a["enum"].as_array().unwrap().iter().map(|x| x.as_str().unwrap())).collect();
                w.sort_unstable();
                return format!("words({})", w.join(","));
            }
            let tagged = alts.iter().all(|a| a.pointer("/properties/type/enum").and_then(Value::as_array).is_some_and(|e| e.len() == 1));
            if tagged {
                let mut variants: Vec<String> = alts
                    .iter()
                    .map(|a| format!("{}={}", a["properties"]["type"]["enum"][0].as_str().unwrap(), sch_object(schema, a, Some("type"))))
                    .collect();
                variants.sort();
                return format!("tagged(type){{{}}}", variants.join(";"));
            }
            let mut a: Vec<String> = alts.iter().map(|a| sch(schema, a)).collect();
            a.sort();
            return format!("oneof({})", a.join("|"));
        }
        if let Some(types) = node.get("type").and_then(Value::as_array) {
            let rest: Vec<&Value> = types.iter().filter(|t| *t != "null").collect();
            assert_eq!(rest.len(), 1, "{node}");
            let mut single = node.clone();
            single["type"] = rest[0].clone();
            let inner = sch(schema, &single);
            return if rest.len() < types.len() { format!("nullable({inner})") } else { inner };
        }
        match node.get("type").and_then(Value::as_str) {
            Some("string") => {
                if let Some(e) = node.get("enum").and_then(Value::as_array) {
                    let mut w: Vec<&str> = e.iter().map(|x| x.as_str().unwrap()).collect();
                    w.sort_unstable();
                    format!("words({})", w.join(","))
                } else if let Some(min) = node.get("minLength").and_then(Value::as_u64) {
                    format!("str(min={min})")
                } else {
                    "str".into()
                }
            }
            Some("integer") => format!("int({})", node["format"].as_str().unwrap_or("?")),
            Some("boolean") => "bool".into(),
            Some("array") => format!("array({})", sch(schema, &node["items"])),
            other => panic!("unrendered schema node {other:?}: {node}"),
        }
    }
    fn sch_object(schema: &Value, node: &Value, tag: Option<&str>) -> String {
        let required: Vec<&str> = node
            .get("required")
            .and_then(Value::as_array)
            .map(|r| r.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        let mut members: Vec<String> = Vec::new();
        if let Some(props) = node.get("properties").and_then(Value::as_object) {
            for (name, prop) in props {
                if Some(name.as_str()) == tag {
                    continue;
                }
                let req = if required.contains(&name.as_str()) { "req" } else { "opt" };
                members.push(format!("{name}:{req} {}", sch(schema, prop)));
            }
        }
        let mut sets: Vec<String> = Vec::new();
        if let Some(any) = node.get("anyOf").and_then(Value::as_array) {
            for alt in any {
                if let Some(props) = alt.get("properties").and_then(Value::as_object) {
                    for (name, prop) in props {
                        members.push(format!("{name}:opt {}", sch(schema, prop)));
                    }
                }
                let mut set: Vec<&str> = alt["required"].as_array().unwrap().iter().filter_map(Value::as_str).collect();
                set.sort_unstable();
                sets.push(set.join(","));
            }
        }
        members.sort();
        let mut text = format!("object{{{}}}", members.join(";"));
        if !sets.is_empty() {
            sets.sort();
            text.push_str(&format!("anyof-required[{}]", sets.join("|")));
        }
        text
    }
    fn thread_item(schema: &Value, node: &Value) -> String {
        const ADMITTED: [&str; 5] = ["agentMessage", "contextCompaction", "dynamicToolCall", "reasoning", "userMessage"];
        let mut variants = Vec::new();
        let mut refused = Vec::new();
        for alt in node["oneOf"].as_array().unwrap() {
            let word = alt["properties"]["type"]["enum"][0].as_str().unwrap();
            if ADMITTED.contains(&word) {
                variants.push(format!("{word}={}", sch_object(schema, alt, Some("type"))));
            } else {
                refused.push(word);
            }
        }
        variants.sort();
        refused.sort_unstable();
        format!("tagged(type){{{}}}refused[{}]open", variants.join(";"), refused.join(","))
    }
    fn schema_text(schema: &Value) -> String {
        let defs = &schema["definitions"];
        let v2 = &defs["v2"];
        let mut request = defs["JSONRPCRequest"].clone();
        request["properties"].as_object_mut().unwrap().remove("trace");
        request["required"] = json!(["id", "method", "params"]);
        let mut notification = defs["JSONRPCNotification"].clone();
        notification["required"] = json!(["method", "params"]);
        let mut roots: Vec<(String, String)> = vec![
            ("envelope:error".into(), sch(schema, &defs["JSONRPCError"])),
            ("envelope:notification".into(), sch(schema, &notification)),
            ("envelope:request".into(), sch(schema, &request)),
            ("envelope:response".into(), sch(schema, &defs["JSONRPCResponse"])),
            ("request:item/tool/call".into(), sch(schema, &defs["DynamicToolCallParams"])),
            ("response:initialize".into(), sch(schema, &defs["InitializeResponse"])),
            ("response:thread/start".into(), sch(schema, &v2["ThreadStartResponse"])),
            ("response:turn/interrupt".into(), sch(schema, &v2["TurnInterruptResponse"])),
            ("response:turn/start".into(), sch(schema, &v2["TurnStartResponse"])),
        ];
        for variant in defs["ServerNotification"]["oneOf"].as_array().unwrap() {
            let method = variant["properties"]["method"]["enum"][0].as_str().unwrap();
            if NOTIFICATION_PARAMS.iter().any(|(m, _)| *m == method) {
                roots.push((format!("notification:{method}"), sch(schema, &variant["properties"]["params"])));
            }
        }
        roots.sort();
        roots.iter().map(|(name, form)| format!("{name} = {form}\n")).collect()
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
