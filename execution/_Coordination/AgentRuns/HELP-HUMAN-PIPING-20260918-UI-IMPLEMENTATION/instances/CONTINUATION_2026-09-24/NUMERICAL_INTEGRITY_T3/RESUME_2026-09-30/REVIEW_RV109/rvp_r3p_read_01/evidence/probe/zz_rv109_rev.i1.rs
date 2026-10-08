//! RV109 probe shim, I1 (262bd687f0): the one-case production preparation.
use crate::retained_product::{PreparedCase, ProductCapture};
pub(super) fn late_total(o: &ProductCapture) -> Option<usize> { Some(o.late_loads_total) }
pub(super) fn cases_seen(_o: &ProductCapture) -> usize { 1 }
pub(super) fn prepare(o: ProductCapture, e: crate::MechanicsEnvelope) -> (String, [u64; 10], Option<PreparedCase>) {
    match o.prepare_case(e) {
        Ok(pc) => ("prepared".into(), pc.capture().adapter.counts.get(), Some(pc)),
        Err(f) => (format!("failed:{:?}", f.capture.error), f.capture.adapter.counts.get(), None),
    }
}
