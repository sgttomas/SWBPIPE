//! DEL-04-10 response-spectrum analysis: the validated spectrum table and its evaluation (unit
//! S2), and the combinations of supplied signed contributions with their contract (unit S4).
//!
//! ## Unit S2: the spectrum table
//!
//! A person, or a later unit, supplies a model-document spectrum record ([`SpectrumRecord`],
//! DEL-02-01 §2.4: abscissa kind and unit, acceleration or g-multiple ordinates with an explicit
//! g, damping ratio, interpolation on each axis, points), a query frequency with its unit and a
//! scale factor, and gets the pseudo-spectral acceleration or a named [`Refusal`]. Every input is
//! the user's; nothing is defaulted, no spectrum ships, and no table is extrapolated (REQ-001,
//! AX-001).
//!
//! The method, as the DEL-04-10 scope records Agent 0's decisions:
//! 1. a table is interpolated on its authored axis: a period table in period, a frequency table
//!    in frequency;
//! 2. a period query is located exactly: the period T = 1/f of the represented binary64 query
//!    frequency (as the modal result publishes it) is compared as a rational with the represented
//!    binary64 knots and range ends ([`period_query_ordering`]). That comparison selects the
//!    interval and decides in or out of range, and nothing more; it makes neither the modal
//!    frequency nor a knot exact, and the interpolation after it is binary64 arithmetic with the
//!    rounding [`SpectrumTable::evaluate`] states;
//! 3. each published value (the interpolated ordinate, S and s·S) is stated within a conditional
//!    relative error bound of at most [`STATED_ACCURACY`], under IEEE 754 binary64 and the libm
//!    assumption [`LIBM_RELATIVE_ERROR`] (not established by platform documentation or tests), or the evaluation is refused.
//!
//! The table is not a spectrum solve: it forms no modal response or rigid response. A later unit
//! supplies the modal frequencies it is evaluated at.
//!
//! ## Unit S4: the combinations and the signed-contribution contract
//!
//! [`combine`] takes a [`CombinationRequest`]: the method choices, the damping ratio, the modal
//! metadata of each extracted mode (its index, its DEL-04-09 cluster and its frequency), the rows,
//! and the signed per-mode, per-direction contributions and signed rigid responses that a
//! producer supplies. It sums each cluster's contributions with their signs first, applies the
//! modal rule across the periodic clusters, combines the rigid response by the rigid rule, then
//! the directions by the directional rule, and returns a [`Combination`] of unsigned
//! [`Magnitude`]s, or named refusals (CLM-001; REQ-003, REQ-004). Each published magnitude is a
//! binary64 shown by a runtime enclosure within 1e-9 relative of its rule's exact value on the
//! represented operands, or an exact zero proved by exact arithmetic; otherwise the request is
//! refused with `SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED` (Agent 0's route A/E2; the enclosure
//! arithmetic is crate-local, with no external dependency). The types in [`contract`]
//! fix the identities, units, frames and sign tags a later document and its readers build on;
//! [`combination_json`] is the unit's JSON entry and output, which is not the `spectrum-1`
//! publication.
//!
//! Neither unit publishes a spectrum result, and nothing here has standing as an engineering
//! result.
mod bigint;
mod combine;
pub mod combination_json;
pub mod contract;
mod dyadic;
mod exact;
mod record;
mod refusal;
mod request;
mod rules;
mod scaled;
mod table;

pub use combine::combine;
pub use contract::{
    ClusterClass, ClusterIndex, ClusterStatement, ClusterSum, Combination, CombinationRequest, CombinedRow,
    ConnectorComponent, CorrelationStatement, Direction, DirectionalPart, DirectionalRule, DisplacementComponent, End,
    ForceComponent, Frame, Magnitude, MemberFamily, MissingMass, ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity,
    ReactionComponent, RigidContribution, RigidRule, RowIdentity, RowUnit, SignConvention, SignStatus, COMBINATION_STATEMENT,
    QUANTITY_KINDS, TIE_ROD_COMPONENT,
};
pub use exact::period_query_ordering;
pub use rules::{correlation_coefficient, within_ten_percent, PRECISION_LADDER};
pub use record::{Abscissa, AxisScale, Interpolation, OrdinateKind, QuantityInput, RecordUnits, SpectrumRecord};
pub use refusal::{Refusal, RefusalCode};
pub use request::{evaluate_request_json, EvaluationRequest, QueryInput, STATEMENT};
pub use table::{
    common_damping_ratio, Location, Ordinate, OrdinateMeasure, QueryFrequency, ScaleFactor, SpectrumTable,
    LIBM_RELATIVE_ERROR, MAX_SOLVE_DAMPING_RATIO, STATED_ACCURACY,
};
