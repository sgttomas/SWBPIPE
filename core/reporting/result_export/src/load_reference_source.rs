//! Current retained-source load/reference-state pre-pass.
//! Received records are checked directly under the current pressure contract;
//! no retired namespace projection, policy substitution or hash resealing exists.
//! A resolved state remains numerically ineligible: this reader does not
//! independently rederive it from a captured request.
use crate::load_reference::{prepass, Method};
use serde_json::Value;

type Check = Result<(), String>;

/// Current source group's closed received-state checks. Numerical eligibility
/// is handled by the source reader and remains false for this group.
pub(crate) fn validate_pressure_source_evidence(source: &Value, raw: bool) -> Check {
    crate::physics_source::SourceContract::PressureV3.check_received(source)?;
    prepass(source, raw, Method::PressureJoined)
}
