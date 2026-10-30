//! Explicit, unwired host dispatcher. No command, process, thread or domain write.
//! Only the trusted native coordinator may construct Origin/History/Evidence,
//! from the admitted webview and its actual queue/run records, never model JSON.
//! Completion raw bytes require admission here even if command ingress admitted
//! its outer envelope. Record the domain outcome before attempting vendor send.
//! A caller taking a retained outcome becomes its owner; it must preserve actual
//! application evidence independently of this bounded session ledger.

use super::{
    catalog::{Catalog, Tool},
    framing, numeric,
    protocol::Event,
    session_driver::{self, SessionDriver},
};
use crate::{ingress, memory_pool::Lease, solve_jobs::SolveJobRegistry};
use serde_json::Value;
use std::sync::Arc;

/// Catalog expansion, 64 bounded identities/bases, and control bookkeeping.
/// GENERAL requested-allocation reservation; not an RSS/retained-route claim.
pub const STATE_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_RECORDS: usize = 64;
const RECORD_OVERHEAD: u64 = 128 * 1024;
const BASIS_BYTES: usize = 16 * 1024;
/// Reserved before handing out work. The exact prescan price is checked again
/// before parsing; no larger parse is allowed to use this fixed reservation.
pub const COMPLETION_BYTES: u64 = 32 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    Configuration,
    Admission,
    Closed,
    NotTool,
    InvalidInput,
    InvalidOutcome,
    Unknown,
    Duplicate,
    WrongOrigin,
    WrongThread,
    WrongBasis,
    MissingEvidence,
    Capacity,
}

/// Derived by Rust from the actual invoking webview/session, not payload fields.
pub struct Origin<'a> {
    pub session: &'a str,
    pub webview: &'a str,
    pub workspace: &'a str,
    pub generation: u64,
}

/// Trusted record lookup for historical reads. The handle and basis must come
/// from the retained proposal/run record, not from the request being checked.
pub enum History<'a> {
    Live,
    Proposal { handle: &'a str, basis: &'a Value },
    Run { handle: &'a str, basis: &'a Value },
}

/// A proof obligation on the native coordinator, not an authenticity assertion.
/// It must obtain this from the real read/queue/job owner. Scalar model fields
/// or a successful send cannot construct actual application evidence.
pub enum Evidence<'a> {
    Read,
    Proposal {
        handle: &'a str,
        basis: &'a Value,
        status: &'a str,
        outcome_model_hash: Option<&'a Value>,
    },
    Run {
        handle: Option<&'a str>,
        run_id: Option<&'a str>,
        basis: &'a Value,
        /// Catalog standing derived from the actual job owner, not reply JSON.
        status: &'a str,
    },
    Failure,
}

pub struct CallIdentity {
    reference: String,
    thread: String,
    turn: String,
    call: String,
    tool: Tool,
    basis: Value,
    target: Option<String>,
}
impl CallIdentity {
    pub fn request_ref(&self) -> &str {
        &self.reference
    }
    pub fn thread_id(&self) -> &str {
        &self.thread
    }
    pub fn turn_id(&self) -> &str {
        &self.turn
    }
    pub fn call_id(&self) -> &str {
        &self.call
    }
    pub fn tool(&self) -> Tool {
        self.tool
    }
    pub fn basis(&self) -> &Value {
        &self.basis
    }
    pub fn target_handle(&self) -> Option<&str> {
        self.target.as_deref()
    }
    fn owned_copy(&self) -> Self {
        Self {
            reference: self.reference.clone(),
            thread: self.thread.clone(),
            turn: self.turn.clone(),
            call: self.call.clone(),
            tool: self.tool,
            basis: self.basis.clone(),
            target: self.target.clone(),
        }
    }
}
impl std::fmt::Debug for CallIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CallIdentity")
    }
}

/// Borrow only through synchronous event projection. Any receiver retaining a
/// serialized copy must admit/retain its own backing until actual consumption.
pub struct Dispatch<'a> {
    pub identity: &'a CallIdentity,
    pub arguments: &'a Value,
}
impl std::fmt::Debug for Dispatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Dispatch")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VendorState {
    Active,
    Detached,
    Lost,
}
struct Entry {
    identity: CallIdentity,
    vendor: VendorState,
    actual: Option<Arc<OutcomeRecord>>,
    completion_backing: Option<Arc<Lease>>,
    record_backing: Option<Lease>,
}

pub struct OutcomeRecord {
    identity: CallIdentity,
    raw: Vec<u8>,
    outcome_model_hash: Option<Value>,
    // Last: raw bytes/identity are destroyed before reservation release.
    _backing: Lease,
}
impl OutcomeRecord {
    pub fn identity(&self) -> &CallIdentity {
        &self.identity
    }
    pub fn raw_outcome(&self) -> &[u8] {
        &self.raw
    }
    pub fn outcome_model_hash(&self) -> Option<&Value> {
        self.outcome_model_hash.as_ref()
    }
}
impl std::fmt::Debug for OutcomeRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OutcomeRecord")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delivery {
    Queued,
    Detached,
    Lost,
    Unconfirmed(session_driver::Refusal),
}
pub struct PreparedCompletion {
    value: Value,
    backing: Arc<Lease>,
    actual: Arc<OutcomeRecord>,
    vendor: VendorState,
}
impl PreparedCompletion {
    pub fn actual(&self) -> &Arc<OutcomeRecord> {
        &self.actual
    }
    /// One attempt only. The dispatcher already retained actual bytes; every
    /// result below is delivery standing, never an application/undo/cancel act.
    pub fn deliver(self, driver: &mut SessionDriver) -> Delivery {
        if self.vendor == VendorState::Lost {
            return Delivery::Lost;
        }
        match driver.complete(self.actual.identity.request_ref(), self.value, self.backing) {
            Ok(session_driver::Submission::Queued) => Delivery::Queued,
            Ok(session_driver::Submission::Detached) => Delivery::Detached,
            Err(error) => Delivery::Unconfirmed(error),
        }
    }
}
impl std::fmt::Debug for PreparedCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PreparedCompletion")
    }
}

pub struct Dispatcher {
    catalog: Catalog,
    registry: SolveJobRegistry,
    session: String,
    webview: String,
    workspace: String,
    generation: u64,
    proposal_cap: u64,
    thread: Option<String>,
    turn: Option<String>,
    highest_reference: u64,
    closed: bool,
    records: Vec<Entry>,
    _state: Lease,
}
impl Dispatcher {
    pub fn new(
        registry: SolveJobRegistry,
        session: &str,
        webview: &str,
        basis: &Value,
        proposal_cap: u64,
    ) -> Result<Self, Refusal> {
        if !(16..=128).contains(&session.len())
            || !session
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
            || !session_driver::control(webview, 160)
            || proposal_cap == 0
        {
            return Err(Refusal::Configuration);
        }
        let state = registry
            .acquire_interactive_ingress(STATE_BYTES)
            .map_err(|_| Refusal::Admission)?;
        let catalog = Catalog::load().map_err(|_| Refusal::Configuration)?;
        valid_basis(&catalog, basis)?;
        if catalog.limits().pending_calls > 16 || catalog.limits().result_bytes > 64 * 1024 {
            return Err(Refusal::Configuration);
        }
        Ok(Self {
            catalog,
            registry,
            session: session.into(),
            webview: webview.into(),
            workspace: basis["workspaceHandle"]
                .as_str()
                .ok_or(Refusal::Configuration)?
                .into(),
            generation: integer(&basis["applicationGeneration"]).ok_or(Refusal::Configuration)?,
            proposal_cap,
            thread: None,
            turn: None,
            highest_reference: 0,
            closed: false,
            records: Vec::with_capacity(MAX_RECORDS),
            _state: state,
        })
    }
    fn origin(&self, origin: &Origin<'_>) -> Result<(), Refusal> {
        if origin.session != self.session
            || origin.webview != self.webview
            || origin.workspace != self.workspace
            || origin.generation != self.generation
        {
            Err(Refusal::WrongOrigin)
        } else {
            Ok(())
        }
    }
    fn scoped_basis(&self, basis: &Value) -> Result<(), Refusal> {
        valid_basis(&self.catalog, basis)?;
        if basis["workspaceHandle"].as_str() != Some(&self.workspace)
            || integer(&basis["applicationGeneration"]) != Some(self.generation)
        {
            Err(Refusal::WrongBasis)
        } else {
            Ok(())
        }
    }
    fn index(&self, reference: &str) -> Result<usize, Refusal> {
        self.records
            .iter()
            .position(|r| r.identity.reference == reference)
            .ok_or(Refusal::Unknown)
    }
    /// Called in the single driver's event order. Detachment/loss never deletes
    /// an actual outcome or fabricates domain cancellation.
    pub fn observe(&mut self, event: &Event) -> Result<(), Refusal> {
        match event {
            Event::ThreadStarted { thread_id } => {
                if self.closed || self.thread.is_some() || !session_driver::control(thread_id, 256)
                {
                    return Err(Refusal::WrongThread);
                }
                self.thread = Some(thread_id.clone());
            }
            Event::TurnStarted { turn_id } => {
                if self.closed
                    || self.thread.is_none()
                    || self.turn.is_some()
                    || !session_driver::control(turn_id, 256)
                {
                    return Err(Refusal::WrongThread);
                }
                self.turn = Some(turn_id.clone());
            }
            Event::TurnCompleted { turn_id, .. } => {
                if self.turn.as_ref() != Some(turn_id) {
                    return Err(Refusal::WrongThread);
                }
                self.turn = None;
                for record in &mut self.records {
                    if record.identity.turn == *turn_id && record.vendor == VendorState::Active {
                        record.vendor = VendorState::Detached;
                    }
                }
            }
            Event::ToolDetached {
                request_ref,
                thread_id,
                turn_id,
                call_id,
                tool,
            }
            | Event::ToolLost {
                request_ref,
                thread_id,
                turn_id,
                call_id,
                tool,
            } => {
                let index = self.index(request_ref.as_str())?;
                let record = &mut self.records[index];
                if record.identity.thread != *thread_id
                    || record.identity.turn != *turn_id
                    || record.identity.call != *call_id
                    || record.identity.tool != *tool
                {
                    return Err(Refusal::WrongThread);
                }
                if matches!(event, Event::ToolLost { .. }) {
                    record.vendor = VendorState::Lost;
                } else if record.vendor == VendorState::Active {
                    record.vendor = VendorState::Detached;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn close(&mut self) {
        self.closed = true;
        self.turn = None;
        for record in &mut self.records {
            record.vendor = VendorState::Lost;
        }
    }
    pub fn admit<'a>(
        &'a mut self,
        event: &'a Event,
        live_basis: &Value,
        history: History<'_>,
    ) -> Result<Dispatch<'a>, Refusal> {
        if self.closed {
            return Err(Refusal::Closed);
        }
        let Event::ToolRequested {
            request_ref,
            thread_id,
            turn_id,
            call_id,
            tool,
            arguments,
        } = event
        else {
            return Err(Refusal::NotTool);
        };
        if self.thread.as_ref() != Some(thread_id)
            || self.turn.as_ref() != Some(turn_id)
            || !session_driver::control(call_id, 256)
        {
            return Err(Refusal::WrongThread);
        }
        let seq = request_ref
            .as_str()
            .strip_prefix(&self.session)
            .and_then(|s| s.strip_prefix(':'))
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or(Refusal::WrongOrigin)?;
        if seq <= self.highest_reference {
            return Err(Refusal::Duplicate);
        }
        if self.records.len() >= MAX_RECORDS
            || self.records.iter().filter(|r| r.actual.is_none()).count()
                >= self.catalog.limits().pending_calls as usize
        {
            return Err(Refusal::Capacity);
        }
        self.scoped_basis(live_basis)?;
        if !session_driver::bounded_value(arguments, self.catalog.limits().argument_bytes as usize)
        {
            return Err(Refusal::InvalidInput);
        }
        self.catalog
            .validate_input(*tool, arguments)
            .map_err(|_| Refusal::InvalidInput)?;
        if *tool == Tool::ProposeOperations {
            self.catalog
                .validate_proposal_context(arguments, self.proposal_cap)
                .map_err(|_| Refusal::InvalidInput)?;
        }
        let (basis, target) = match (tool, history) {
            (Tool::ObserveProposal, History::Proposal { handle, basis })
                if arguments["proposalHandle"].as_str() == Some(handle) =>
            {
                (basis, Some(handle))
            }
            (Tool::ReadResults, History::Run { handle, basis })
                if arguments["runHandle"].as_str() == Some(handle) =>
            {
                (basis, Some(handle))
            }
            (
                Tool::Summary | Tool::Nodes | Tool::ProposeOperations | Tool::Solve,
                History::Live,
            ) => (live_basis, None),
            _ => return Err(Refusal::MissingEvidence),
        };
        self.scoped_basis(basis)?;
        if *tool != Tool::Summary
            && (arguments["workspaceHandle"].as_str() != Some(&self.workspace)
                || !same_basis(&arguments["basis"], basis))
        {
            return Err(Refusal::WrongBasis);
        }
        if matches!(tool, Tool::ProposeOperations | Tool::Solve)
            && arguments["claimedModelHash"] != basis["modelHash"]
        {
            return Err(Refusal::WrongBasis);
        }
        let completion_backing = Arc::new(
            self.registry
                .acquire_interactive_ingress(COMPLETION_BYTES)
                .map_err(|_| Refusal::Admission)?,
        );
        let record_bytes = ingress::allocation_envelope(self.catalog.limits().result_bytes)
            .and_then(|b| b.checked_add(RECORD_OVERHEAD))
            .ok_or(Refusal::Admission)?;
        let record_backing = self
            .registry
            .acquire_interactive_ingress(record_bytes)
            .map_err(|_| Refusal::Admission)?;
        self.records.push(Entry {
            identity: CallIdentity {
                reference: request_ref.as_str().into(),
                thread: thread_id.clone(),
                turn: turn_id.clone(),
                call: call_id.clone(),
                tool: *tool,
                basis: basis.clone(),
                target: target.map(str::to_owned),
            },
            vendor: VendorState::Active,
            actual: None,
            completion_backing: Some(completion_backing),
            record_backing: Some(record_backing),
        });
        self.highest_reference = seq;
        Ok(Dispatch {
            identity: &self.records.last().unwrap().identity,
            arguments,
        })
    }
    /// Native bridge claim gate, immediately before exposing execution input.
    /// The caller derives Origin from the invoking webview/session. Historical
    /// calls retain their admitted record basis; new work rechecks live basis.
    pub(super) fn claimable(
        &self,
        origin: Origin<'_>,
        reference: &str,
        live_basis: &Value,
    ) -> Result<&CallIdentity, Refusal> {
        self.origin(&origin)?;
        let record = &self.records[self.index(reference)?];
        if self.closed || record.vendor != VendorState::Active {
            return Err(Refusal::Closed);
        }
        if record.actual.is_some() {
            return Err(Refusal::Duplicate);
        }
        self.scoped_basis(live_basis)?;
        if matches!(
            record.identity.tool,
            Tool::Summary | Tool::Nodes | Tool::ProposeOperations | Tool::Solve
        ) && !same_basis(&record.identity.basis, live_basis)
        {
            return Err(Refusal::WrongBasis);
        }
        Ok(&record.identity)
    }
    pub fn record_completion(
        &mut self,
        origin: Origin<'_>,
        reference: &str,
        raw: &[u8],
        current_basis: Option<&Value>,
        evidence: Evidence<'_>,
    ) -> Result<PreparedCompletion, Refusal> {
        self.origin(&origin)?;
        let index = self.index(reference)?;
        if self.records[index].actual.is_some() {
            return Err(Refusal::Duplicate);
        }
        if raw.len() > self.catalog.limits().result_bytes as usize {
            return Err(Refusal::InvalidOutcome);
        }
        let bound = ingress::prescan(raw).map_err(|_| Refusal::InvalidOutcome)?;
        let bytes = framing::reservation_bytes(bound).ok_or(Refusal::Admission)?;
        if bytes
            > self.records[index]
                .completion_backing
                .as_ref()
                .ok_or(Refusal::Duplicate)?
                .bytes()
        {
            return Err(Refusal::InvalidOutcome);
        }
        numeric::check_admitted_json(raw).map_err(|_| Refusal::InvalidOutcome)?;
        let value: Value = serde_json::from_slice(raw).map_err(|_| Refusal::InvalidOutcome)?;
        let call = &self.records[index].identity;
        self.catalog
            .validate_output(call.tool, &value)
            .map_err(|_| Refusal::InvalidOutcome)?;
        if !same_basis(&value["basis"], &call.basis) {
            return Err(Refusal::WrongBasis);
        }
        let status = value["status"].as_str().ok_or(Refusal::InvalidOutcome)?;
        let mut actual_hash = None;
        match evidence {
            Evidence::Read if matches!(call.tool, Tool::Summary | Tool::Nodes) => {
                if status == "ok" {
                    let live = current_basis.ok_or(Refusal::WrongBasis)?;
                    self.scoped_basis(live)?;
                    if !same_basis(live, &call.basis) {
                        return Err(Refusal::WrongBasis);
                    }
                    if !value["data"].is_object()
                        || (call.tool == Tool::Summary
                            && value["data"]["workspaceHandle"].as_str() != Some(&self.workspace))
                    {
                        return Err(Refusal::MissingEvidence);
                    }
                }
            }
            Evidence::Proposal {
                handle,
                basis,
                status: actual_status,
                outcome_model_hash,
            } if matches!(call.tool, Tool::ProposeOperations | Tool::ObserveProposal) => {
                if !same_basis(basis, &call.basis)
                    || status != actual_status
                    || value
                        .pointer("/data/proposalHandle")
                        .and_then(Value::as_str)
                        != Some(handle)
                {
                    return Err(Refusal::MissingEvidence);
                }
                if status == "applied" {
                    let hash = outcome_model_hash.ok_or(Refusal::MissingEvidence)?;
                    if !hash.is_object() || !session_driver::bounded_value(hash, 2048) {
                        return Err(Refusal::MissingEvidence);
                    }
                    let mut proof_basis = call.basis.clone();
                    proof_basis["modelHash"] = hash.clone();
                    valid_basis(&self.catalog, &proof_basis)
                        .map_err(|_| Refusal::MissingEvidence)?;
                    actual_hash = Some(hash.clone());
                }
                if call.tool == Tool::ObserveProposal
                    && (call.target.as_deref() != Some(handle)
                        || !same_basis(&value["data"]["proposalBasis"], &call.basis))
                {
                    return Err(Refusal::WrongBasis);
                }
                if call.tool == Tool::ObserveProposal
                    && value["data"]["outcomeModelHash"].as_object().is_some()
                    && outcome_model_hash != Some(&value["data"]["outcomeModelHash"])
                {
                    return Err(Refusal::MissingEvidence);
                }
                if call.tool == Tool::ObserveProposal
                    && status == "applied"
                    && !value["data"]["outcomeModelHash"].is_object()
                {
                    return Err(Refusal::MissingEvidence);
                }
            }
            Evidence::Run {
                handle,
                run_id,
                basis,
                status: actual_status,
            } if matches!(call.tool, Tool::Solve | Tool::ReadResults) => {
                if (call.tool == Tool::Solve && status != actual_status)
                    || (call.tool == Tool::ReadResults
                        && status == "ok"
                        && actual_status != "completed")
                {
                    return Err(Refusal::MissingEvidence);
                }
                if status == "completed" && (handle.is_none() || run_id.is_none()) {
                    return Err(Refusal::MissingEvidence);
                }
                if !value["data"].is_object()
                    || !same_basis(basis, &call.basis)
                    || value.pointer("/data/runHandle").and_then(Value::as_str) != handle
                {
                    return Err(Refusal::MissingEvidence);
                }
                if call.tool == Tool::ReadResults && call.target.as_deref() != handle {
                    return Err(Refusal::MissingEvidence);
                }
                if value.pointer("/data/runId").and_then(Value::as_str) != run_id {
                    return Err(Refusal::MissingEvidence);
                }
                if value["data"]["evaluatedModelHash"] != call.basis["modelHash"] {
                    return Err(Refusal::WrongBasis);
                }
            }
            Evidence::Failure
                if matches!(
                    status,
                    "unknown" | "unavailable" | "stale_basis" | "execution_error"
                ) && value["data"].is_null() => {}
            _ => return Err(Refusal::MissingEvidence),
        }
        let record_backing = self.records[index]
            .record_backing
            .take()
            .ok_or(Refusal::Duplicate)?;
        let actual = Arc::new(OutcomeRecord {
            identity: self.records[index].identity.owned_copy(),
            raw: raw.to_vec(),
            outcome_model_hash: actual_hash,
            _backing: record_backing,
        });
        self.records[index].actual = Some(actual.clone()); // Before any vendor operation.
        Ok(PreparedCompletion {
            value,
            backing: self.records[index]
                .completion_backing
                .take()
                .ok_or(Refusal::Duplicate)?,
            actual,
            vendor: self.records[index].vendor,
        })
    }
    pub fn actual(
        &self,
        origin: Origin<'_>,
        reference: &str,
    ) -> Result<Option<&Arc<OutcomeRecord>>, Refusal> {
        self.origin(&origin)?;
        Ok(self
            .records
            .get(self.index(reference)?)
            .and_then(|r| r.actual.as_ref()))
    }
    /// Explicit custody transfer to the actual outcome owner, never implicit
    /// eviction. The returned Arc retains raw bytes, provenance and reservation.
    pub fn take_actual(
        &mut self,
        origin: Origin<'_>,
        reference: &str,
    ) -> Result<Arc<OutcomeRecord>, Refusal> {
        self.origin(&origin)?;
        let index = self.index(reference)?;
        if self.records[index].actual.is_none() {
            return Err(Refusal::MissingEvidence);
        }
        Ok(self.records.remove(index).actual.unwrap())
    }
}
impl std::fmt::Debug for Dispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Dispatcher")
    }
}

fn valid_basis(catalog: &Catalog, basis: &Value) -> Result<(), Refusal> {
    if !session_driver::bounded_value(basis, BASIS_BYTES) {
        return Err(Refusal::WrongBasis);
    }
    session_driver::validate_basis(catalog, basis).map_err(|_| Refusal::WrongBasis)
}
fn integer(value: &Value) -> Option<u64> {
    numeric::as_exact_u64(value.as_number()?)
}
fn same_basis(a: &Value, b: &Value) -> bool {
    ["workspaceHandle", "modelHash", "stateId", "runId"]
        .iter()
        .all(|k| a.get(k) == b.get(k))
        && ["applicationGeneration", "modelRevision"]
            .iter()
            .all(|k| integer(&a[k]).is_some() && integer(&a[k]) == integer(&b[k]))
}

#[cfg(test)]
mod tests {
    use super::super::{
        protocol::{Protocol, ThreadOptions},
        stdio_pump,
    };
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use serde_json::json;

    const TOKEN: &str = "dispatcher-fixture-token";
    fn basis(revision: u64) -> Value {
        json!({"workspaceHandle":"workspace-invented","applicationGeneration":1,"modelRevision":revision,"modelHash":null,"stateId":null,"runId":null})
    }
    fn origin() -> Origin<'static> {
        Origin {
            session: TOKEN,
            webview: "main",
            workspace: "workspace-invented",
            generation: 1,
        }
    }
    fn registry() -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
    }
    fn setup() -> (Dispatcher, Protocol) {
        let mut d = Dispatcher::new(registry(), TOKEN, "main", &basis(2), 16).unwrap();
        let mut p = Protocol::new(Catalog::load().unwrap(), TOKEN.into(), 16).unwrap();
        p.initialize("test", "1").unwrap();
        p.handle_frame(json!({"id":1,"result":{"codexHome":"/invented","platformFamily":"unix","platformOs":"macos","userAgent":"fixture"}}),&basis(2)).unwrap();
        p.start_thread(ThreadOptions {
            model: "fixture".into(),
            provider: "fixture".into(),
            cwd: "/invented".into(),
            base_instructions: "invented".into(),
        })
        .unwrap();
        for event in p.handle_frame(json!({"id":2,"result":{"thread":{"id":"thread-1"},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}}}),&basis(2)).unwrap().events { d.observe(&event).unwrap(); }
        p.submit_text("invented".into()).unwrap();
        for event in p
            .handle_frame(
                json!({"id":3,"result":{"turn":{"id":"turn-1","items":[],"status":"inProgress"}}}),
                &basis(2),
            )
            .unwrap()
            .events
        {
            d.observe(&event).unwrap();
        }
        (d, p)
    }
    fn request(p: &mut Protocol, index: u64, tool: Tool, args: Value) -> Event {
        p.handle_frame(json!({"id":100+index,"method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":format!("call-{index}"),"tool":tool.name(),"arguments":args}}),&basis(2)).unwrap().events.into_iter().find(|e|matches!(e,Event::ToolRequested{..})).expect("valid call")
    }
    fn reference(e: &Event) -> &str {
        match e {
            Event::ToolRequested { request_ref, .. } => request_ref.as_str(),
            _ => panic!("call"),
        }
    }
    fn outcome(status: &str, basis: Value, data: Value) -> Vec<u8> {
        serde_json::to_vec(
            &json!({"status":status,"basis":basis,"message":"Invented outcome","data":data}),
        )
        .unwrap()
    }
    fn summary() -> Vec<u8> {
        outcome(
            "ok",
            basis(2),
            json!({"workspaceHandle":"workspace-invented","projectUnits":{},"objectCounts":{},"admittedOperationKinds":[],"limits":{"argumentBytes":65536,"resultBytes":65536,"pageRows":50,"pendingCalls":16,"proposalOperations":16}}),
        )
    }
    fn model_hash() -> Value {
        json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","payload_ref":"invented","value":"0".repeat(64),"hash_status":"computed_local_preview"})
    }

    #[test]
    fn rejects_invalid_config_and_general_admission_failure() {
        assert_eq!(
            Dispatcher::new(registry(), "short", "main", &basis(2), 16).unwrap_err(),
            Refusal::Configuration
        );
        let host = ScriptedHost::normal();
        host.set(|s| s.headroom_bytes = Some(0));
        let r = SolveJobRegistry::new(MemoryPool::new(host));
        assert_eq!(
            Dispatcher::new(r, TOKEN, "main", &basis(2), 16).unwrap_err(),
            Refusal::Admission
        );
    }
    #[test]
    fn captures_original_context_and_never_an_rpc_destination() {
        let (mut d, mut p) = setup();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        let dispatch = d.admit(&e, &basis(2), History::Live).unwrap();
        assert_eq!(dispatch.identity.request_ref(), reference(&e));
        assert_eq!(dispatch.identity.thread_id(), "thread-1");
        assert_eq!(dispatch.identity.turn_id(), "turn-1");
        assert_eq!(dispatch.identity.call_id(), "call-1");
        assert_eq!(dispatch.identity.basis(), &basis(2));
        assert_eq!(dispatch.arguments, &json!({}));
        assert_eq!(format!("{:?}", dispatch), "Dispatch");
    }
    #[test]
    fn wrong_origin_and_unknown_completion_preserve_the_pending_call() {
        let (mut d, mut p) = setup();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        d.admit(&e, &basis(2), History::Live).unwrap();
        let bad = Origin {
            webview: "other",
            ..origin()
        };
        assert_eq!(
            d.record_completion(
                bad,
                reference(&e),
                &summary(),
                Some(&basis(2)),
                Evidence::Read
            )
            .unwrap_err(),
            Refusal::WrongOrigin
        );
        assert_eq!(
            d.record_completion(origin(), "unknown", &[], None, Evidence::Failure)
                .unwrap_err(),
            Refusal::Unknown
        );
        assert!(d.actual(origin(), reference(&e)).unwrap().is_none());
        d.record_completion(
            origin(),
            reference(&e),
            &summary(),
            Some(&basis(2)),
            Evidence::Read,
        )
        .unwrap();
        assert!(d.actual(origin(), reference(&e)).unwrap().is_some());
    }
    #[test]
    fn wrong_thread_or_workspace_does_not_dispatch() {
        let (mut d, mut p) = setup();
        let mut e = request(&mut p, 1, Tool::Summary, json!({}));
        if let Event::ToolRequested { thread_id, .. } = &mut e {
            *thread_id = "other".into();
        }
        assert_eq!(
            d.admit(&e, &basis(2), History::Live).unwrap_err(),
            Refusal::WrongThread
        );
        if let Event::ToolRequested { thread_id, .. } = &mut e {
            *thread_id = "thread-1".into();
        }
        let mut wrong = basis(2);
        wrong["workspaceHandle"] = json!("other");
        assert_eq!(
            d.admit(&e, &wrong, History::Live).unwrap_err(),
            Refusal::WrongBasis
        );
        d.admit(&e, &basis(2), History::Live).unwrap();
    }
    #[test]
    fn live_success_cannot_complete_after_revision_drift() {
        let (mut d, mut p) = setup();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        d.admit(&e, &basis(2), History::Live).unwrap();
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &summary(),
                Some(&basis(3)),
                Evidence::Read
            )
            .unwrap_err(),
            Refusal::WrongBasis
        );
        assert!(d.actual(origin(), reference(&e)).unwrap().is_none());
        d.record_completion(
            origin(),
            reference(&e),
            &outcome("execution_error", basis(2), Value::Null),
            Some(&basis(3)),
            Evidence::Failure,
        )
        .unwrap();
    }
    #[test]
    fn historical_admission_requires_matching_record_handle_and_basis() {
        let (mut d, mut p) = setup();
        let e = request(
            &mut p,
            1,
            Tool::ObserveProposal,
            json!({"workspaceHandle":"workspace-invented","basis":basis(2),"proposalHandle":"proposal-1"}),
        );
        assert_eq!(
            d.admit(&e, &basis(3), History::Live).unwrap_err(),
            Refusal::MissingEvidence
        );
        assert_eq!(
            d.admit(
                &e,
                &basis(3),
                History::Proposal {
                    handle: "other",
                    basis: &basis(2)
                }
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
        assert_eq!(
            d.admit(
                &e,
                &basis(3),
                History::Proposal {
                    handle: "proposal-1",
                    basis: &basis(3)
                }
            )
            .unwrap_err(),
            Refusal::WrongBasis
        );
        assert_eq!(
            d.admit(
                &e,
                &basis(3),
                History::Proposal {
                    handle: "proposal-1",
                    basis: &basis(2)
                }
            )
            .unwrap()
            .identity
            .target_handle(),
            Some("proposal-1")
        );
    }
    #[test]
    fn applied_outcome_survives_detachment_and_failed_vendor_delivery() {
        let (mut d, mut p) = setup();
        let e = request(
            &mut p,
            1,
            Tool::ObserveProposal,
            json!({"workspaceHandle":"workspace-invented","basis":basis(2),"proposalHandle":"proposal-1"}),
        );
        d.admit(
            &e,
            &basis(2),
            History::Proposal {
                handle: "proposal-1",
                basis: &basis(2),
            },
        )
        .unwrap();
        d.observe(&Event::TurnCompleted {
            turn_id: "turn-1".into(),
            status: "completed".into(),
        })
        .unwrap();
        let raw = outcome(
            "applied",
            basis(2),
            json!({"proposalHandle":"proposal-1","proposalBasis":basis(2),"outcomeModelHash":model_hash(),"author_type":"agent","assumptions":[]}),
        );
        let prepared = d
            .record_completion(
                origin(),
                reference(&e),
                &raw,
                Some(&basis(3)),
                Evidence::Proposal {
                    handle: "proposal-1",
                    basis: &basis(2),
                    status: "applied",
                    outcome_model_hash: Some(&model_hash()),
                },
            )
            .unwrap();
        assert_eq!(prepared.vendor, VendorState::Detached);
        let mut driver = SessionDriver::start(
            std::io::Cursor::new(Vec::<u8>::new()),
            std::io::sink(),
            registry(),
            TOKEN,
            16,
            &basis(2),
        )
        .unwrap();
        driver.stop();
        assert!(matches!(
            prepared.deliver(&mut driver),
            Delivery::Unconfirmed(_)
        ));
        let end = std::time::Instant::now() + std::time::Duration::from_secs(2);
        loop {
            assert!(std::time::Instant::now() < end);
            let s = match driver.poll() {
                Ok(observation) => observation.io,
                Err(session_driver::Refusal::Busy) => {
                    std::thread::yield_now();
                    continue;
                }
                Err(other) => panic!("unexpected synthetic worker observation: {other:?}"),
            };
            if s.reader_completion == stdio_pump::Completion::Joined
                && s.writer_completion == stdio_pump::Completion::Joined
            {
                break;
            }
            assert!(std::time::Instant::now() < end);
            std::thread::yield_now();
        }
        assert_eq!(
            d.actual(origin(), reference(&e))
                .unwrap()
                .unwrap()
                .raw_outcome(),
            raw
        );
        assert_eq!(
            d.actual(origin(), reference(&e))
                .unwrap()
                .unwrap()
                .outcome_model_hash(),
            Some(&model_hash())
        );
        let retained = d.take_actual(origin(), reference(&e)).unwrap();
        drop(d);
        assert_eq!(retained.raw_outcome(), raw);
    }
    #[test]
    fn duplicate_completion_and_replay_after_custody_transfer_are_refused() {
        let (mut d, mut p) = setup();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        d.admit(&e, &basis(2), History::Live).unwrap();
        let raw = outcome("execution_error", basis(2), Value::Null);
        let prepared = d
            .record_completion(origin(), reference(&e), &raw, None, Evidence::Failure)
            .unwrap();
        drop(prepared);
        assert_eq!(
            d.record_completion(origin(), reference(&e), &raw, None, Evidence::Failure)
                .unwrap_err(),
            Refusal::Duplicate
        );
        d.take_actual(origin(), reference(&e)).unwrap();
        assert_eq!(
            d.admit(&e, &basis(2), History::Live).unwrap_err(),
            Refusal::Duplicate
        );
    }
    #[test]
    fn closed_session_can_capture_late_actual_outcome_without_reopening() {
        let (mut d, mut p) = setup();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        d.admit(&e, &basis(2), History::Live).unwrap();
        d.close();
        let prepared = d
            .record_completion(
                origin(),
                reference(&e),
                &outcome("execution_error", basis(2), Value::Null),
                None,
                Evidence::Failure,
            )
            .unwrap();
        assert_eq!(prepared.vendor, VendorState::Lost);
        assert!(d.actual(origin(), reference(&e)).unwrap().is_some());
        assert_eq!(
            d.admit(&e, &basis(2), History::Live).unwrap_err(),
            Refusal::Closed
        );
    }
    #[test]
    fn invalid_oversized_and_rounded_numbers_never_create_an_actual_record() {
        let (mut d, mut p) = setup();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        d.admit(&e, &basis(2), History::Live).unwrap();
        let raw = outcome("execution_error", basis(2), Value::Null);
        let rounded = String::from_utf8(raw.clone()).unwrap().replace(
            "\"modelRevision\":2",
            "\"modelRevision\":9007199254740993.0",
        );
        for bytes in [b"{".as_slice(), &vec![b' '; 65537], rounded.as_bytes()] {
            assert_eq!(
                d.record_completion(origin(), reference(&e), bytes, None, Evidence::Failure)
                    .unwrap_err(),
                Refusal::InvalidOutcome
            );
            assert!(d.actual(origin(), reference(&e)).unwrap().is_none());
        }
        d.record_completion(origin(), reference(&e), &raw, None, Evidence::Failure)
            .unwrap();
    }
    #[test]
    fn read_success_needs_actual_data_and_correct_workspace() {
        let (mut d, mut p) = setup();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        d.admit(&e, &basis(2), History::Live).unwrap();
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &outcome("ok", basis(2), Value::Null),
                Some(&basis(2)),
                Evidence::Read
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
        let mut wrong: Value = serde_json::from_slice(&summary()).unwrap();
        wrong["data"]["workspaceHandle"] = json!("other");
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &serde_json::to_vec(&wrong).unwrap(),
                Some(&basis(2)),
                Evidence::Read
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
    }
    #[test]
    fn bounded_pending_calls_do_not_evict_unresolved_domain_work() {
        let (mut d, mut p) = setup();
        let mut first = None;
        for n in 1..=16 {
            let e = request(&mut p, n, Tool::Summary, json!({}));
            if first.is_none() {
                first = Some(reference(&e).to_owned());
            }
            d.admit(&e, &basis(2), History::Live).unwrap();
        }
        p.complete(
            first.as_ref().unwrap(),
            serde_json::from_slice(&outcome("execution_error", basis(2), Value::Null)).unwrap(),
        )
        .unwrap();
        let e = request(&mut p, 17, Tool::Summary, json!({}));
        assert_eq!(
            d.admit(&e, &basis(2), History::Live).unwrap_err(),
            Refusal::Capacity
        );
        assert_eq!(d.records.len(), 16);
    }

    #[test]
    fn admission_reserves_completion_before_work_and_later_pressure_cannot_lose_it() {
        let host = ScriptedHost::normal();
        let registry = SolveJobRegistry::new(MemoryPool::new(host.clone()));
        let (_, mut p) = setup();
        let mut d = Dispatcher::new(registry, TOKEN, "main", &basis(2), 16).unwrap();
        d.observe(&Event::ThreadStarted {
            thread_id: "thread-1".into(),
        })
        .unwrap();
        d.observe(&Event::TurnStarted {
            turn_id: "turn-1".into(),
        })
        .unwrap();
        let e = request(&mut p, 1, Tool::Summary, json!({}));
        d.admit(&e, &basis(2), History::Live).unwrap();
        host.set(|s| s.headroom_bytes = Some(0));
        let raw = outcome("execution_error", basis(2), Value::Null);
        let result = d
            .record_completion(origin(), reference(&e), &raw, None, Evidence::Failure)
            .unwrap();
        assert_eq!(result.actual().raw_outcome(), raw);
        assert_eq!(
            d.actual(origin(), reference(&e))
                .unwrap()
                .unwrap()
                .raw_outcome(),
            raw
        );
    }

    #[test]
    fn a_matching_basis_does_not_substitute_another_proposal_or_applied_hash() {
        let (mut d, mut p) = setup();
        let e = request(
            &mut p,
            1,
            Tool::ObserveProposal,
            json!({"workspaceHandle":"workspace-invented","basis":basis(2),"proposalHandle":"proposal-1"}),
        );
        d.admit(
            &e,
            &basis(2),
            History::Proposal {
                handle: "proposal-1",
                basis: &basis(2),
            },
        )
        .unwrap();
        let raw = outcome(
            "applied",
            basis(2),
            json!({"proposalHandle":"other","proposalBasis":basis(2),"outcomeModelHash":model_hash(),"author_type":"agent","assumptions":[]}),
        );
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &raw,
                None,
                Evidence::Proposal {
                    handle: "other",
                    basis: &basis(2),
                    status: "applied",
                    outcome_model_hash: Some(&model_hash())
                }
            )
            .unwrap_err(),
            Refusal::WrongBasis
        );
        let raw = outcome(
            "applied",
            basis(2),
            json!({"proposalHandle":"proposal-1","proposalBasis":basis(2),"outcomeModelHash":null,"author_type":"agent","assumptions":[]}),
        );
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &raw,
                None,
                Evidence::Proposal {
                    handle: "proposal-1",
                    basis: &basis(2),
                    status: "applied",
                    outcome_model_hash: None
                }
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
        assert!(d.actual(origin(), reference(&e)).unwrap().is_none());
    }

    #[test]
    fn dense_and_deep_completion_parses_are_bounded_before_materialization() {
        let dense = format!("[{}]", vec!["{\"\":0}"; 8000].join(","));
        let deep = format!("{}0{}", "{\"\":".repeat(100), "}".repeat(100));
        for raw in [dense.as_bytes(), deep.as_bytes()] {
            let bound = ingress::prescan(raw).unwrap();
            assert!(framing::reservation_bytes(bound).unwrap() <= COMPLETION_BYTES);
        }
    }

    #[test]
    fn retained_run_result_keeps_its_handle_id_and_evaluated_basis_after_live_edit() {
        let (mut d, mut p) = setup();
        let mut evaluated = basis(2);
        evaluated["modelHash"] = model_hash();
        let mut live = evaluated.clone();
        live["modelRevision"] = json!(3);
        let e = request(
            &mut p,
            1,
            Tool::ReadResults,
            json!({"workspaceHandle":"workspace-invented","basis":evaluated,"runHandle":"run-1","page":{"cursor":null,"limit":1}}),
        );
        d.admit(
            &e,
            &live,
            History::Run {
                handle: "run-1",
                basis: &evaluated,
            },
        )
        .unwrap();
        let raw = outcome(
            "ok",
            evaluated.clone(),
            json!({"runHandle":"run-1","runId":"native-run-1","evaluatedModelHash":model_hash(),"values":[],"nextCursor":null,"standing":{"route":"invented","health":"invented","checks":[]}}),
        );
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &raw,
                Some(&live),
                Evidence::Run {
                    status: "completed",
                    handle: Some("run-2"),
                    run_id: Some("native-run-1"),
                    basis: &evaluated
                }
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &raw,
                Some(&live),
                Evidence::Run {
                    status: "completed",
                    handle: Some("run-1"),
                    run_id: Some("native-run-2"),
                    basis: &evaluated
                }
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &raw,
                Some(&live),
                Evidence::Run {
                    status: "execution_error",
                    handle: Some("run-1"),
                    run_id: Some("native-run-1"),
                    basis: &evaluated
                }
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
        let completion = d
            .record_completion(
                origin(),
                reference(&e),
                &raw,
                Some(&live),
                Evidence::Run {
                    status: "completed",
                    handle: Some("run-1"),
                    run_id: Some("native-run-1"),
                    basis: &evaluated,
                },
            )
            .unwrap();
        assert_eq!(completion.actual().identity().basis(), &evaluated);
    }

    #[test]
    fn completed_solve_requires_real_run_identity_and_matching_evaluated_hash() {
        let (mut d, mut p) = setup();
        let mut evaluated = basis(2);
        evaluated["modelHash"] = model_hash();
        let e = request(
            &mut p,
            1,
            Tool::Solve,
            json!({"workspaceHandle":"workspace-invented","basis":evaluated,"claimedModelHash":model_hash()}),
        );
        d.admit(&e, &evaluated, History::Live).unwrap();
        let absent = outcome(
            "completed",
            evaluated.clone(),
            json!({"runHandle":null,"runId":null,"evaluatedModelHash":model_hash(),"health":"invented","solverMode":"invented"}),
        );
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &absent,
                None,
                Evidence::Run {
                    status: "completed",
                    handle: None,
                    run_id: None,
                    basis: &evaluated
                }
            )
            .unwrap_err(),
            Refusal::MissingEvidence
        );
        let mut changed = model_hash();
        changed["value"] = json!("f".repeat(64));
        let wrong = outcome(
            "completed",
            evaluated.clone(),
            json!({"runHandle":"run-1","runId":"native-run-1","evaluatedModelHash":changed,"health":"invented","solverMode":"invented"}),
        );
        assert_eq!(
            d.record_completion(
                origin(),
                reference(&e),
                &wrong,
                None,
                Evidence::Run {
                    status: "completed",
                    handle: Some("run-1"),
                    run_id: Some("native-run-1"),
                    basis: &evaluated
                }
            )
            .unwrap_err(),
            Refusal::WrongBasis
        );
        let raw = outcome(
            "completed",
            evaluated.clone(),
            json!({"runHandle":"run-1","runId":"native-run-1","evaluatedModelHash":model_hash(),"health":"invented","solverMode":"invented"}),
        );
        d.record_completion(
            origin(),
            reference(&e),
            &raw,
            None,
            Evidence::Run {
                status: "completed",
                handle: Some("run-1"),
                run_id: Some("native-run-1"),
                basis: &evaluated,
            },
        )
        .unwrap();
    }

    #[test]
    fn proposals_need_the_current_hash_revision_and_host_operation_cap() {
        let (mut d, mut p) = setup();
        d.proposal_cap = 1;
        let mut live = basis(2);
        live["modelHash"] = model_hash();
        let op = json!({"operation_id":"op1","operation_kind":"modify","operation_status":"proposed","author_type":"agent","target":{"object_type":"Node","ref":"n1"},
            "change":{"change_id":"c1","change_kind":"set_field","field_label":"x","field_path":"position.x","before":"0","after":"1","unit":"m","dimension":"length","source_note":"invented"},
            "validation":{"schema_validation":"","constraint_validation":"","unit_validation":"","diff_preview_status":"","application_status":""},
            "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
            "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"invented"});
        let input = json!({"workspaceHandle":"workspace-invented","basis":live,"claimedModelHash":model_hash(),"operations":[op.clone()],"assumptions":[]});
        let mut wrong = input.clone();
        wrong["claimedModelHash"]["value"] = json!("f".repeat(64));
        let e = request(&mut p, 1, Tool::ProposeOperations, wrong);
        assert_eq!(
            d.admit(&e, &live, History::Live).unwrap_err(),
            Refusal::WrongBasis
        );
        let e = request(&mut p, 2, Tool::ProposeOperations, input.clone());
        let mut edited = live.clone();
        edited["modelRevision"] = json!(3);
        assert_eq!(
            d.admit(&e, &edited, History::Live).unwrap_err(),
            Refusal::WrongBasis
        );
        let mut over = input.clone();
        let mut second = op;
        second["operation_id"] = json!("op2");
        second["change"]["change_id"] = json!("c2");
        over["operations"].as_array_mut().unwrap().push(second);
        let e = request(&mut p, 3, Tool::ProposeOperations, over);
        assert_eq!(
            d.admit(&e, &live, History::Live).unwrap_err(),
            Refusal::InvalidInput
        );
        let e = request(&mut p, 4, Tool::ProposeOperations, input);
        d.admit(&e, &live, History::Live).unwrap();
    }

    #[test]
    fn unfinished_or_failed_job_cannot_be_reported_completed_or_consume_custody() {
        let (mut d, mut p) = setup();
        let mut evaluated = basis(2);
        evaluated["modelHash"] = model_hash();
        let e = request(
            &mut p,
            1,
            Tool::Solve,
            json!({"workspaceHandle":"workspace-invented","basis":evaluated,"claimedModelHash":model_hash()}),
        );
        d.admit(&e, &evaluated, History::Live).unwrap();
        let index = d.index(reference(&e)).unwrap();
        let reservation = d.records[index]
            .completion_backing
            .as_ref()
            .unwrap()
            .clone();
        let record_bytes = d.records[index].record_backing.as_ref().unwrap().bytes();
        let raw = outcome(
            "completed",
            evaluated.clone(),
            json!({"runHandle":"run-1","runId":"native-run-1","evaluatedModelHash":model_hash(),"health":"invented","solverMode":"invented"}),
        );
        for actual_status in ["unknown", "execution_error", "rejected"] {
            assert_eq!(
                d.record_completion(
                    origin(),
                    reference(&e),
                    &raw,
                    None,
                    Evidence::Run {
                        handle: Some("run-1"),
                        run_id: Some("native-run-1"),
                        basis: &evaluated,
                        status: actual_status
                    }
                )
                .unwrap_err(),
                Refusal::MissingEvidence
            );
            assert!(d.actual(origin(), reference(&e)).unwrap().is_none());
            assert!(Arc::ptr_eq(
                &reservation,
                d.records[index].completion_backing.as_ref().unwrap()
            ));
            assert_eq!(
                d.records[index].record_backing.as_ref().unwrap().bytes(),
                record_bytes
            );
        }
        let truthful = outcome(
            "execution_error",
            evaluated.clone(),
            json!({"runHandle":"run-1","runId":"native-run-1","evaluatedModelHash":model_hash(),"health":"invented failure","solverMode":"invented"}),
        );
        let captured = d
            .record_completion(
                origin(),
                reference(&e),
                &truthful,
                None,
                Evidence::Run {
                    handle: Some("run-1"),
                    run_id: Some("native-run-1"),
                    basis: &evaluated,
                    status: "execution_error",
                },
            )
            .unwrap();
        assert_eq!(captured.actual().raw_outcome(), truthful);
    }
}
