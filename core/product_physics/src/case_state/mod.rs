//! Resolved load/reference-state case (`openpipestress.load_reference_state/1.0.0`).
//!
//! One resolved case per authored load case is the single source for element
//! stiffness, eigenstrain, prescribed boundary values, the physical-source
//! ledger and the evidence published for that case. Submodules are pure:
//! `material` selects per-member E/nu (derived G) from one actual user material;
//! `thermal` converts explicit thermal/fit definitions into normalized strains.
//! Neither mutates the model or synthesizes material identities.

pub(crate) mod material;
pub(crate) mod thermal;
