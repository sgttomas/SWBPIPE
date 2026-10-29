//! One-shot native/webview transport core. No live command/start registration.
//! A notice contains only requestRef, never execution input. An authenticated
//! native claim returns the original admitted input once. The future command
//! adapter derives Origin from the invoking webview, not fields in its body.
//! Domain lookup/Evidence remains with actual queue/job owners. This transport
//! neither executes a tool nor upgrades a webview assertion into domain proof.

use super::{
    dispatcher::{Dispatcher, Evidence, History, Origin, PreparedCompletion, Refusal},
    protocol::Event,
};
use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
use serde::Serialize;
use serde_json::Value;
use std::{io, sync::Arc};

/// Requested GENERAL allocations, separately from driver/dispatcher reservations.
/// 64 entries, reference/turn strings and containers. No argument Value clones.
pub const STATE_BYTES: u64 = 128 * 1024;
/// Raw claim buffer, serialization/response transients and bounded metadata.
pub const CLAIM_BYTES: u64 = 512 * 1024;
pub const MAX_CLAIM_BYTES: usize = 64 * 1024;
const MAX_ENTRIES: usize = 64;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notice<'a> {
    pub request_ref: &'a str,
}

/// Keep alive through serialization/native response handoff. Framework-owned
/// transport copies after handoff have the existing command-ingress residual.
pub struct Claim {
    raw: String,
    _backing: Lease,
}
impl Claim {
    pub fn raw(&self) -> &str {
        &self.raw
    }
}
impl std::fmt::Debug for Claim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Claim")
    }
}
struct Entry {
    reference: String,
    turn: String,
    offered: Option<Claim>,
    claimed: bool,
    available: bool,
}

pub struct Bridge {
    dispatcher: Dispatcher,
    registry: SolveJobRegistry,
    entries: Vec<Entry>,
    closed: bool,
    // Last: all owned data dies before backing is released.
    _state: Lease,
}
impl Bridge {
    /// Explicit construction only; the dispatcher was bound to a host-generated
    /// session token and actual admitted webview/workspace/generation.
    pub fn new(registry: SolveJobRegistry, dispatcher: Dispatcher) -> Result<Self, Refusal> {
        let state = registry
            .acquire_interactive_ingress(STATE_BYTES)
            .map_err(|_| Refusal::Admission)?;
        Ok(Self {
            dispatcher,
            registry,
            entries: Vec::with_capacity(MAX_ENTRIES),
            closed: false,
            _state: state,
        })
    }
    pub fn observe(&mut self, event: &Event) -> Result<(), Refusal> {
        self.dispatcher.observe(event)?;
        for entry in &mut self.entries {
            let retired = match event {
                Event::ToolDetached { request_ref, .. } | Event::ToolLost { request_ref, .. } => {
                    entry.reference == request_ref.as_str()
                }
                Event::TurnCompleted { turn_id, .. } => entry.turn == *turn_id,
                _ => false,
            };
            if retired {
                entry.available = false;
                entry.offered = None;
            }
        }
        Ok(())
    }
    /// The caller emits only this borrowed notice to the bound webview. It must
    /// serialize the tiny notice before another mutable bridge operation.
    pub fn offer(
        &mut self,
        event: &Event,
        live_basis: &Value,
        history: History<'_>,
    ) -> Result<Notice<'_>, Refusal> {
        if self.closed {
            return Err(Refusal::Closed);
        }
        // Tombstones are not silently evicted: losing a claim response cannot
        // make the original work claimable again, even after custody transfer.
        if self.entries.len() == MAX_ENTRIES {
            return Err(Refusal::Capacity);
        }
        let backing = self
            .registry
            .acquire_interactive_ingress(CLAIM_BYTES)
            .map_err(|_| Refusal::Admission)?;
        let dispatch = self.dispatcher.admit(event, live_basis, history)?;
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Payload<'a> {
            request_ref: &'a str,
            thread_id: &'a str,
            turn_id: &'a str,
            call_id: &'a str,
            tool: &'a str,
            basis: &'a Value,
            arguments: &'a Value,
        }
        let payload = Payload {
            request_ref: dispatch.identity.request_ref(),
            thread_id: dispatch.identity.thread_id(),
            turn_id: dispatch.identity.turn_id(),
            call_id: dispatch.identity.call_id(),
            tool: dispatch.identity.tool().name(),
            basis: dispatch.identity.basis(),
            arguments: dispatch.arguments,
        };
        let mut writer = Capped(Vec::with_capacity(MAX_CLAIM_BYTES));
        if serde_json::to_writer(&mut writer, &payload).is_err() {
            // Admission already occurred. Refuse/close; never retry or publish a
            // partial argument envelope as executable work.
            self.close();
            return Err(Refusal::Capacity);
        }
        let raw = String::from_utf8(writer.0).map_err(|_| Refusal::InvalidInput)?;
        self.entries.push(Entry {
            reference: dispatch.identity.request_ref().into(),
            turn: dispatch.identity.turn_id().into(),
            offered: Some(Claim {
                raw,
                _backing: backing,
            }),
            claimed: false,
            available: true,
        });
        Ok(Notice {
            request_ref: &self.entries.last().unwrap().reference,
        })
    }
    pub fn claim(
        &mut self,
        origin: Origin<'_>,
        reference: &str,
        live_basis: &Value,
    ) -> Result<Claim, Refusal> {
        self.dispatcher.claimable(origin, reference, live_basis)?;
        let entry = self
            .entries
            .iter_mut()
            .find(|r| r.reference == reference)
            .ok_or(Refusal::Unknown)?;
        if entry.claimed {
            return Err(Refusal::Duplicate);
        }
        if !entry.available {
            return Err(Refusal::Closed);
        }
        let claim = entry.offered.take().ok_or(Refusal::Duplicate)?;
        entry.claimed = true; // Before response serialization/transport can fail.
        Ok(claim)
    }
    /// Command ingress must first admit raw bytes and enforce lexical numeric
    /// identity. Evidence is a separate trusted lookup, never command JSON.
    /// Claimed actual outcomes may arrive after close; unclaimed work cannot.
    pub fn complete(
        &mut self,
        origin: Origin<'_>,
        reference: &str,
        raw: &[u8],
        live_basis: Option<&Value>,
        evidence: Evidence<'_>,
    ) -> Result<PreparedCompletion, Refusal> {
        if !self
            .entries
            .iter()
            .any(|r| r.reference == reference && r.claimed)
        {
            return Err(Refusal::MissingEvidence);
        }
        self.dispatcher
            .record_completion(origin, reference, raw, live_basis, evidence)
    }
    pub fn take_actual(
        &mut self,
        origin: Origin<'_>,
        reference: &str,
    ) -> Result<Arc<super::dispatcher::OutcomeRecord>, Refusal> {
        self.dispatcher.take_actual(origin, reference)
    }
    pub fn close(&mut self) {
        self.closed = true;
        self.dispatcher.close();
        for entry in &mut self.entries {
            entry.available = false;
            entry.offered = None;
        }
    }
}
impl std::fmt::Debug for Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Bridge")
    }
}
struct Capped(Vec<u8>);
impl io::Write for Capped {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > MAX_CLAIM_BYTES - self.0.len() {
            return Err(io::Error::other("claim cap"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        catalog::{Catalog, Tool},
        protocol::{Protocol, ThreadOptions},
    };
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use serde_json::json;
    const TOKEN: &str = "webview-bridge-fixture";
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
    fn setup() -> (Bridge, Protocol, Arc<ScriptedHost>) {
        let host = ScriptedHost::normal();
        let registry = SolveJobRegistry::new(MemoryPool::new(host.clone()));
        let dispatcher = Dispatcher::new(registry.clone(), TOKEN, "main", &basis(2), 16).unwrap();
        let mut b = Bridge::new(registry, dispatcher).unwrap();
        let mut p = Protocol::new(Catalog::load().unwrap(), TOKEN.into(), 16).unwrap();
        p.initialize("test", "1").unwrap();
        p.handle_frame(json!({"id":1,"result":{"codexHome":"/invented","platformFamily":"unix","platformOs":"macos","userAgent":"fixture"}}), &basis(2)).unwrap();
        p.start_thread(ThreadOptions {
            model: "fixture".into(),
            provider: "fixture".into(),
            cwd: "/invented".into(),
            base_instructions: "invented".into(),
        })
        .unwrap();
        for event in p.handle_frame(json!({"id":2,"result":{"thread":{"id":"thread-1"},"approvalPolicy":"never","sandbox":{"type":"readOnly","networkAccess":false}}}), &basis(2)).unwrap().events { b.observe(&event).unwrap(); }
        p.submit_text("invented".into()).unwrap();
        for event in p
            .handle_frame(
                json!({"id":3,"result":{"turn":{"id":"turn-1","items":[],"status":"inProgress"}}}),
                &basis(2),
            )
            .unwrap()
            .events
        {
            b.observe(&event).unwrap();
        }
        (b, p, host)
    }
    fn request(p: &mut Protocol, n: u64) -> Event {
        p.handle_frame(json!({"id":100+n,"method":"item/tool/call","params":{"threadId":"thread-1","turnId":"turn-1","callId":format!("call-{n}"),"tool":Tool::Summary.name(),"arguments":{}}}), &basis(2)).unwrap().events.into_iter().find(|e| matches!(e, Event::ToolRequested { .. })).unwrap()
    }
    fn offer(b: &mut Bridge, p: &mut Protocol, n: u64) -> String {
        b.offer(&request(p, n), &basis(2), History::Live)
            .unwrap()
            .request_ref
            .into()
    }
    fn failure() -> Vec<u8> {
        serde_json::to_vec(&json!({"status":"unavailable","basis":basis(2),"message":"Invented unknown outcome","data":null})).unwrap()
    }
    #[test]
    fn notice_has_no_execution_input_and_claim_returns_original_once() {
        let (mut b, mut p, _) = setup();
        let reference = offer(&mut b, &mut p, 1);
        let notice = serde_json::to_value(Notice {
            request_ref: &reference,
        })
        .unwrap();
        assert_eq!(notice, json!({"requestRef":reference}));
        let claim = b.claim(origin(), &reference, &basis(2)).unwrap();
        let payload: Value = serde_json::from_str(claim.raw()).unwrap();
        assert_eq!(payload["requestRef"], reference);
        assert_eq!(payload["arguments"], json!({}));
        assert_eq!(payload["basis"], basis(2));
        assert_eq!(payload["threadId"], "thread-1");
        assert_eq!(payload["callId"], "call-1");
        assert!(payload.get("rpcId").is_none());
        drop(claim); // A lost response never makes it executable again.
        assert_eq!(
            b.claim(origin(), &reference, &basis(2)).unwrap_err(),
            Refusal::Duplicate
        );
    }
    #[test]
    fn forged_origins_or_references_and_stale_live_basis_do_not_consume_claim() {
        let (mut b, mut p, _) = setup();
        let reference = offer(&mut b, &mut p, 1);
        for wrong in [
            Origin {
                webview: "other",
                ..origin()
            },
            Origin {
                session: "other-session-token",
                ..origin()
            },
            Origin {
                generation: 2,
                ..origin()
            },
            Origin {
                workspace: "other",
                ..origin()
            },
        ] {
            assert_eq!(
                b.claim(wrong, &reference, &basis(2)).unwrap_err(),
                Refusal::WrongOrigin
            );
        }
        assert_eq!(
            b.claim(origin(), "forged", &basis(2)).unwrap_err(),
            Refusal::Unknown
        );
        assert_eq!(
            b.claim(origin(), &reference, &basis(3)).unwrap_err(),
            Refusal::WrongBasis
        );
        assert!(b.claim(origin(), &reference, &basis(2)).is_ok());
    }
    #[test]
    fn unclaimed_completion_cannot_create_an_actual_outcome() {
        let (mut b, mut p, _) = setup();
        let reference = offer(&mut b, &mut p, 1);
        assert_eq!(
            b.complete(origin(), &reference, &failure(), None, Evidence::Failure)
                .unwrap_err(),
            Refusal::MissingEvidence
        );
        b.claim(origin(), &reference, &basis(2)).unwrap();
        assert!(b
            .complete(origin(), &reference, &failure(), None, Evidence::Failure)
            .is_ok());
    }
    #[test]
    fn close_disables_claims_but_preserves_correlated_late_actual_and_tombstone() {
        let (mut b, mut p, _) = setup();
        let claimed = offer(&mut b, &mut p, 1);
        let unclaimed = offer(&mut b, &mut p, 2);
        b.claim(origin(), &claimed, &basis(2)).unwrap();
        b.close();
        assert_eq!(
            b.claim(origin(), &unclaimed, &basis(2)).unwrap_err(),
            Refusal::Closed
        );
        let completion = b
            .complete(origin(), &claimed, &failure(), None, Evidence::Failure)
            .unwrap();
        let actual = b.take_actual(origin(), &claimed).unwrap();
        assert_eq!(actual.raw_outcome(), failure());
        drop(completion);
        assert_eq!(actual.identity().request_ref(), claimed);
        assert_eq!(
            b.complete(origin(), &claimed, &failure(), None, Evidence::Failure)
                .unwrap_err(),
            Refusal::Unknown
        );
        assert_eq!(
            b.complete(origin(), &unclaimed, &failure(), None, Evidence::Failure)
                .unwrap_err(),
            Refusal::MissingEvidence
        );
    }
    #[test]
    fn invalid_completion_keeps_claim_and_real_outcome_reservation() {
        let (mut b, mut p, _) = setup();
        let reference = offer(&mut b, &mut p, 1);
        b.claim(origin(), &reference, &basis(2)).unwrap();
        assert_eq!(
            b.complete(origin(), &reference, b"{", None, Evidence::Failure)
                .unwrap_err(),
            Refusal::InvalidOutcome
        );
        assert!(b
            .complete(origin(), &reference, &failure(), None, Evidence::Failure)
            .is_ok());
        assert_eq!(
            b.complete(origin(), &reference, &failure(), None, Evidence::Failure)
                .unwrap_err(),
            Refusal::Duplicate
        );
    }
    #[test]
    fn terminal_turn_retires_unclaimed_work() {
        let (mut b, mut p, _) = setup();
        let reference = offer(&mut b, &mut p, 1);
        for e in p.handle_frame(json!({"method":"turn/completed","params":{"thread":{"id":"thread-1"},"threadId":"thread-1","turn":{"id":"turn-1","status":"completed","items":[]}}}), &basis(2)).unwrap().events { b.observe(&e).unwrap(); }
        assert_eq!(
            b.claim(origin(), &reference, &basis(2)).unwrap_err(),
            Refusal::Closed
        );
        assert!(b.entries[0].offered.is_none());
    }
    #[test]
    fn memory_refusal_before_offer_does_not_consume_native_request() {
        let (mut b, mut p, host) = setup();
        let e = request(&mut p, 1);
        host.set(|s| s.headroom_bytes = Some(0));
        assert!(matches!(
            b.offer(&e, &basis(2), History::Live),
            Err(Refusal::Admission)
        ));
        host.set(|s| s.headroom_bytes = Some(64 * 1024 * 1024 * 1024));
        assert!(b.offer(&e, &basis(2), History::Live).is_ok());
    }
    #[test]
    fn historical_record_claim_preserves_original_basis_after_live_edit() {
        let (mut b, mut p, _) = setup();
        let mut e = request(&mut p, 1);
        if let Event::ToolRequested {
            tool, arguments, ..
        } = &mut e
        {
            *tool = Tool::ObserveProposal;
            *arguments = json!({"workspaceHandle":"workspace-invented","basis":basis(2),"proposalHandle":"proposal-invented"});
        }
        let reference = b
            .offer(
                &e,
                &basis(3),
                History::Proposal {
                    handle: "proposal-invented",
                    basis: &basis(2),
                },
            )
            .unwrap()
            .request_ref
            .to_owned();
        let claim = b.claim(origin(), &reference, &basis(4)).unwrap();
        let value: Value = serde_json::from_str(claim.raw()).unwrap();
        assert_eq!(value["basis"], basis(2));
        assert_eq!(value["arguments"]["proposalHandle"], "proposal-invented");
    }
    #[test]
    fn taking_outcomes_never_evicts_claim_tombstones_or_reopens_work() {
        let (mut b, mut p, _) = setup();
        for n in 1..=64 {
            let reference = offer(&mut b, &mut p, n);
            b.claim(origin(), &reference, &basis(2)).unwrap();
            b.complete(origin(), &reference, &failure(), None, Evidence::Failure)
                .unwrap();
            b.take_actual(origin(), &reference).unwrap();
            p.complete(&reference, serde_json::from_slice(&failure()).unwrap())
                .unwrap();
            assert_eq!(
                b.claim(origin(), &reference, &basis(2)).unwrap_err(),
                Refusal::Unknown
            );
        }
        let event = request(&mut p, 65);
        assert!(matches!(
            b.offer(&event, &basis(2), History::Live),
            Err(Refusal::Capacity)
        ));
        assert_eq!(b.entries.len(), 64);
    }
    #[test]
    fn debug_never_discloses_payload_and_writer_never_grows_past_cap() {
        let (mut b, mut p, _) = setup();
        let reference = offer(&mut b, &mut p, 1);
        let claim = b.claim(origin(), &reference, &basis(2)).unwrap();
        assert_eq!(format!("{b:?} {claim:?}"), "Bridge Claim");
        let mut writer = Capped(Vec::with_capacity(MAX_CLAIM_BYTES));
        use std::io::Write;
        writer.write_all(&vec![b'x'; MAX_CLAIM_BYTES]).unwrap();
        assert!(writer.write_all(b"x").is_err());
        assert_eq!(writer.0.len(), MAX_CLAIM_BYTES);
    }
}
