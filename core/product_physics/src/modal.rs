//! F1 modal analysis, product side (DEL-04-09 Design
//! `execution/PKG-04/DEL-04-09/Work/modal-analysis.md`), increment I2a: the
//! modal request and its refusals (§1.4), the state case's post-lowering
//! binding (Agent 0 decision D-L), the preparation that mirrors the static
//! entry's pre-formation sequence, the mass on the self-weight basis (§2.2 to
//! §2.4) and the weight-basis identity (§2.5); increment I2b-1 the dense
//! path to an in-crate `ModalOutcome`; increment I2b-2 nonlinear supports at
//! the named state case (`state`, on lib.rs's typed `LoadCaseSolve` seam).
//!
//! Nothing here is public or reachable from any entry: there is no product,
//! native, headless, schema or publication entry. It never enters the
//! retained Direct entry or D1. The identity's tolerance is not pinned
//! (D-TOL); only tests supply one.
//!
//! Function names here are chosen so that none is also a call token elsewhere
//! in the priced crate sources (`tools/retained_pricing/base/crate_dirs.txt`):
//! the retained-pricing call graph resolves calls by name, so a shared name
//! (`get`, `check`, `validate`, ...) would add over-approximate edges from
//! retained-reachable code into this module. The request has no hand-written
//! `Deserialize` implementation for the same reason (serde entry points reach
//! every custom one).
#![allow(dead_code)] // No entry consumes this module until I2b; its tests do.

mod context;
mod document;
mod eigen;
mod integrity;
mod mass;
mod outcome;
mod prepare;
mod request;
mod state;
mod stiffness;
mod weight_basis;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_document;
#[cfg(test)]
mod tests_eigen;
#[cfg(test)]
mod tests_state;
#[cfg(test)]
mod observation_tests;
#[cfg(test)]
mod phase_measurand_tests;

/// The request names neither `mode_count` nor `frequency_cutoff` (§1.4).
pub(crate) const MODAL_EXTRACTION_BOUND_REQUIRED: &str = "MODAL_EXTRACTION_BOUND_REQUIRED";
/// A named bound that is not positive: `mode_count` 0, or a `frequency_cutoff`
/// that is not a finite positive frequency of the units catalog.
pub(crate) const MODAL_EXTRACTION_BOUND_INVALID: &str = "MODAL_EXTRACTION_BOUND_INVALID";
/// §1.4 requires a state case (a load/reference-state document, or a model
/// with a nonlinear support) and the request names none.
pub(crate) const MODAL_STATE_CASE_REQUIRED: &str = "MODAL_STATE_CASE_REQUIRED";
/// The state case names no single case, its static admission refuses it, its
/// binding to the lowered model fails, or its same-run solve does not give a
/// converged, qualified and confirmed support state (I2b-2, `state`).
pub(crate) const MODAL_STATE_CASE_UNAVAILABLE: &str = "MODAL_STATE_CASE_UNAVAILABLE";
/// A request modulus basis together with a state case.
pub(crate) const MODAL_MODULUS_BASIS_CONFLICT: &str = "MODAL_MODULUS_BASIS_CONFLICT";
/// A component body enters the mass and the request names no
/// `weight_to_mass_acceleration` (§2.3 item 4).
pub(crate) const MODAL_WEIGHT_ACCELERATION_REQUIRED: &str = "MODAL_WEIGHT_ACCELERATION_REQUIRED";
/// `weight_to_mass_acceleration` is not a finite positive acceleration.
pub(crate) const MODAL_WEIGHT_ACCELERATION_INVALID: &str = "MODAL_WEIGHT_ACCELERATION_INVALID";
/// A current applied component-weight primitive's acceleration magnitude is
/// not a, bit for bit (§2.3 item 5).
pub(crate) const MODAL_WEIGHT_ACCELERATION_MISMATCH: &str = "MODAL_WEIGHT_ACCELERATION_MISMATCH";
/// A component body outside §2.3 item 6's binary64 range premise.
pub(crate) const MODAL_COMPONENT_MASS_OUT_OF_RANGE: &str = "MODAL_COMPONENT_MASS_OUT_OF_RANGE";
/// The weight-basis identity fails (§2.5).
pub(crate) const MODAL_MASS_WEIGHT_BASIS_MISMATCH: &str = "MODAL_MASS_WEIGHT_BASIS_MISMATCH";
/// Timoshenko straight members, on either route, until M4 (§1.2).
pub(crate) const MODAL_TIMOSHENKO_MASS_UNSUPPORTED: &str = "MODAL_TIMOSHENKO_MASS_UNSUPPORTED";
