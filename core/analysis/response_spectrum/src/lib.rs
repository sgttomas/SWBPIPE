//! DEL-04-10 response-spectrum analysis, unit S2: the validated spectrum table and its
//! evaluation.
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
//! This unit is not a spectrum solve: it forms no modal response, combination or rigid response,
//! publishes no spectrum result and has no standing as an engineering result. A later unit
//! supplies the modal frequencies it is evaluated at.
mod exact;
mod record;
mod refusal;
mod request;
mod scaled;
mod table;

pub use exact::period_query_ordering;
pub use record::{Abscissa, AxisScale, Interpolation, OrdinateKind, QuantityInput, RecordUnits, SpectrumRecord};
pub use refusal::{Refusal, RefusalCode};
pub use request::{evaluate_request_json, EvaluationRequest, QueryInput, STATEMENT};
pub use table::{
    common_damping_ratio, Location, Ordinate, OrdinateMeasure, QueryFrequency, ScaleFactor, SpectrumTable,
    LIBM_RELATIVE_ERROR, MAX_SOLVE_DAMPING_RATIO, STATED_ACCURACY,
};
