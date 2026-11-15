//! Named refusals of the spectrum table and its evaluation (DEL-04-10 REQ-001).
use std::fmt;

/// The code of a refusal. Each has a stable name, [`RefusalCode::as_str`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RefusalCode {
    /// The spectrum record breaks a rule of the model document's record (DEL-02-01 §2.4). The
    /// name is the one DEL-02-01's validator uses, and the refusal names the field.
    RecordInvalid,
    /// The damping ratio is a valid record value above what a spectrum solve admits,
    /// 0 ≤ ζ ≤ 0.2 (REQ-001).
    DampingOutOfRange,
    /// The spectra of one request do not all have the same damping ratio (REQ-001).
    DampingMismatch,
    /// The query frequency lies outside the spectrum's tabulated range; no spectrum is
    /// extrapolated (REQ-001, Range).
    FrequencyOutsideRange,
    /// The query frequency is not a positive finite frequency in a catalog frequency unit.
    QueryFrequencyInvalid,
    /// The scale factor is not positive and finite (REQ-001, Directions).
    ScaleFactorInvalid,
    /// An evaluation produced a value that is not finite, such as an ordinate times g or the
    /// scale factor beyond the binary64 range. No value is published in its place.
    EvaluationNonfinite,
}

impl RefusalCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordInvalid => "SPECTRUM_RECORD_INVALID",
            Self::DampingOutOfRange => "SPECTRUM_DAMPING_OUT_OF_RANGE",
            Self::DampingMismatch => "SPECTRUM_DAMPING_MISMATCH",
            Self::FrequencyOutsideRange => "SPECTRUM_FREQUENCY_OUTSIDE_RANGE",
            Self::QueryFrequencyInvalid => "SPECTRUM_QUERY_FREQUENCY_INVALID",
            Self::ScaleFactorInvalid => "SPECTRUM_SCALE_FACTOR_INVALID",
            Self::EvaluationNonfinite => "SPECTRUM_EVALUATION_NONFINITE",
        }
    }
}

impl fmt::Display for RefusalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A named refusal: its code, the object refused (a spectrum's id, or `query`), the field
/// where one applies, and a sentence saying why.
#[derive(Debug, Clone, PartialEq)]
pub struct Refusal {
    pub code: RefusalCode,
    pub object: String,
    pub field: Option<String>,
    pub detail: String,
}

impl Refusal {
    pub(crate) fn new(code: RefusalCode, object: &str, field: Option<&str>, detail: String) -> Self {
        Self { code, object: object.to_string(), field: field.map(str::to_string), detail }
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.field {
            Some(field) => write!(f, "{} ({} {}): {}", self.code, self.object, field, self.detail),
            None => write!(f, "{} ({}): {}", self.code, self.object, self.detail),
        }
    }
}

impl std::error::Error for Refusal {}
