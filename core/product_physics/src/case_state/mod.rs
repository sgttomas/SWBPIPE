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
pub const LOAD_STATE_MODEL_VERSION: &str = crate::MODEL_SCHEMA_VERSION;
/// Closed `analysis_state.contract` identity.
pub const LOAD_REFERENCE_STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
/// Formulation profile identity published in `formulation_basis.profile_id`.
pub const LOAD_STATE_PROFILE_ID: &str = "resolved_straight_load_state_v1";

/// Recorded for every case whose published response is the ordinary route:
/// no retained-source response is joined into that case.
pub(crate) const SOURCE_RECOVERY_NOT_JOINED: &str = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED";

pub(crate) fn is_load_state(model: &PreviewModel) -> bool {
    crate::exact_admission::ExactContract::of(model).is_some() && carries_fields(model)
}

/// The connected group is optional as a whole; any authored key (null included)
/// selects its validation duties, independently of the document version.
pub(crate) fn carries_fields(model: &PreviewModel) -> bool {
    model.reference_configurations.is_authored()
        || model.material_expansion_laws.iter().any(crate::Authored::is_authored)
        || !model.request_material_expansion_laws.is_empty()
        || model.load_cases.iter().any(|case| case.analysis_state.is_authored())
}

#[cfg(test)]
mod tests;
