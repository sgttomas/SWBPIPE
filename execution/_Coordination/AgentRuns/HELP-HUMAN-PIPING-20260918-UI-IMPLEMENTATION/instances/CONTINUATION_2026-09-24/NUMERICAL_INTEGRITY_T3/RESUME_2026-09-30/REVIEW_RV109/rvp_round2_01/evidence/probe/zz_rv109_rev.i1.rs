//! RV109 round-2 probe shim, I1 (262bd687f0): the one-case production transaction, stage by stage.
use crate::retained_product::ProductCapture;
use serde_json::{json, Value};
pub(super) fn late_total(o: &ProductCapture) -> Option<usize> { Some(o.late_loads_total) }
pub(super) fn cases_seen(_o: &ProductCapture) -> usize { 1 }
fn cut(text: String) -> String { text.chars().take(160).collect() }
/// I1's `retained_w1` after T-5: `prepare_case`, `solve_native`, `freeze_candidate`, `staged_envelope`,
/// `serialize_frozen` (precommit is the caller's).
pub(super) fn stages(o: ProductCapture, e: crate::MechanicsEnvelope, capture: &crate::source_receipt::CapturedInvocation) -> Value {
    let mut line = json!({});
    let mut pc = match o.prepare_case(e) {
        Err(f) => {
            line["prep"] = json!(cut(format!("failed:{:?}", f.capture.error)));
            line["after_prep"] = json!(f.capture.adapter.counts.get());
            line["snapshot"] = json!(f.trace.adapter.as_ref().map(|s| format!("{s:?}")));
            return line;
        }
        Ok(pc) => pc,
    };
    line["prep"] = json!("prepared");
    line["after_prep"] = json!(pc.capture().adapter.counts.get());
    let native = pc.solve_native().map_err(|e| format!("{e:?}"));
    line["native"] = json!(native.clone().err().unwrap_or_else(|| "ok".into()));
    line["after_native"] = json!(pc.capture().adapter.counts.get());
    if native.is_err() {
        line["snapshot"] = json!(pc.trace.adapter.as_ref().map(|s| format!("{s:?}")));
        return line;
    }
    let frozen = match pc.freeze_candidate() {
        Err(r) => {
            line["candidate"] = json!(cut(format!("refused:{:?}", r.error)));
            line["after_candidate"] = json!(r.capture().adapter.counts.get());
            let mut costs = Default::default();
            line["snapshot"] = json!(r.typed_trace(&mut costs).map(|v| format!("{:?}", v.adapter)).map_err(|e| format!("{e:?}")));
            return line;
        }
        Ok(f) => f,
    };
    line["candidate"] = json!("frozen");
    line["after_candidate"] = json!(frozen.capture().adapter.counts.get());
    {
        let mut costs = Default::default();
        line["snapshot"] = json!(frozen.typed_trace(&mut costs).map(|v| format!("{:?}", v.adapter)).map_err(|e| format!("{e:?}")));
    }
    let staged = match frozen.staged_envelope() {
        Err(fault) => {
            line["staging"] = json!(format!("{fault:?}"));
            return line;
        }
        Ok(staged) => staged,
    };
    line["staging"] = json!("ok");
    line["after_staging"] = json!(frozen.capture().adapter.counts.get());
    line["staged_summary"] = serde_json::to_value(&staged.summary).unwrap();
    let serialized = crate::retained_wire::serialize_frozen(&frozen, &staged, capture);
    line["after_serialize"] = json!(frozen.capture().adapter.counts.get());
    match serialized {
        Err(f) => { line["serializer"] = json!(format!("{f:?}")); }
        Ok(successor) => { line["serializer"] = json!("ok"); line["successor_value"] = successor; }
    }
    line
}
