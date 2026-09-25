//! Resolved load/reference-state case (`openpipestress.load_reference_state/1.0.0`).
//!
//! One resolved case per authored load case is the single source for element
//! stiffness, eigenstrain, prescribed boundary values, the physical-source
//! ledger and the evidence published for that case. Submodules are pure:
//! `material` selects per-member E/nu (derived G) from one actual user material;
//! `thermal` converts explicit thermal/fit definitions into normalized strains;
//! `input` holds the closed authored DTOs; `resolve` joins them per case.
//! Nothing here mutates the model or synthesizes material identities.

pub mod input;
pub(crate) mod material;
pub(crate) mod resolve;
pub(crate) mod temperature;
pub(crate) mod thermal;

use crate::PreviewModel;

/// Model document version selected for the connected load/reference-state route.
pub const LOAD_STATE_MODEL_VERSION: &str = "0.4.0";
/// Closed `analysis_state.contract` identity.
pub const LOAD_REFERENCE_STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
/// Formulation profile identity published in `formulation_basis.profile_id`.
pub const LOAD_STATE_PROFILE_ID: &str = "resolved_straight_load_state_v1";
/// Producer result-semantics identity; its table/hash are separately owned.
pub const LOAD_REFERENCE_SEMANTIC_CONTRACT_ID: &str =
    "openpipestress.result_semantics/0.3.0/load-reference-1";
/// Diagnostic recorded for every case until the retained-source join is verified.
pub(crate) const SOURCE_RECOVERY_NOT_JOINED: &str = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED";

pub(crate) fn is_load_state(model: &PreviewModel) -> bool {
    model.schema_version == LOAD_STATE_MODEL_VERSION
}

#[cfg(test)]
mod tests;
