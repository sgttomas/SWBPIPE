//! Named refusals of the spectrum table and its evaluation (DEL-04-10 REQ-001, unit S2), and of
//! the combinations of supplied signed contributions (REQ-001, REQ-003, REQ-004, unit S4).
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
    /// This implemented error argument does not establish 1e-9 relative accuracy for a value
    /// (the interpolated ordinate, S or s·S), under its arithmetic and libm assumptions.
    /// This can be conservative even for an exactly representable answer; it is not proof that
    /// accurate representation is impossible. The evaluation publishes nothing. This is distinct
    /// from a query outside the table and from an invalid record.
    EvaluationAccuracyUnsupported,
    /// A required method choice of a combination (the modal or directional rule, or whether to
    /// apply the missing-mass correction and its rule) is absent; none has a default (REQ-001).
    MethodChoiceMissing,
    /// CQC was chosen with a damping ratio of 0 (REQ-001, Methods).
    CqcDampingUnsupported,
    /// The requested directions are empty or repeat a direction (REQ-001, Directions).
    DirectionInvalid,
    /// The supplied modal metadata is not a modal result's: mode indices not strictly
    /// ascending, a frequency not positive and finite or not ascending, or clusters not
    /// chained from 1 over adjacent modes. The combinations check this structure; they infer
    /// no cluster.
    ModalMetadataInvalid,
    /// A row is empty, repeated, or stated in a unit or frame its kind does not have.
    RowInvalid,
    /// A signed contribution is missing, repeated, not finite, of the wrong length, or for a
    /// mode or direction the request does not have.
    ContributionInvalid,
    /// A contribution was supplied for a mode of a rigid cluster. Extracted rigid modes are
    /// never combined as periodic responses (REQ-004).
    ContributionNotPeriodic,
    /// The rigid frequency is not a positive finite frequency in Hz.
    RigidFrequencyInvalid,
    /// A rigid response is missing, repeated, not finite or of the wrong length for a requested
    /// direction, or supplied without the correction.
    RigidResponseInvalid,
    /// The clusters a rigid response's load left out as periodic are not the clusters the
    /// request classifies as periodic (CLM-001, Rigid response).
    RigidResponsePremiseMismatch,
    /// This implemented error argument does not establish the required relative accuracy (1e-9 of
    /// the exact value on the represented operands) for a combined magnitude that exact arithmetic
    /// does not prove zero: at the precision cap, the CQC radicand's enclosure still reaches zero,
    /// or the binary64 nearest the enclosure's midpoint is not shown within 1e-9 of every point of
    /// the enclosure. It says what the argument could not establish, not that no binary64 lies
    /// within 1e-9 of the exact value. The refusal names the row and the field where it occurred;
    /// no value is published in its place, and no partial result is published.
    CombinationAccuracyUnsupported,
    /// An exact cluster sum, or a combined value's enclosure, lies beyond the binary64 range. No
    /// value is published in its place.
    CombinationNonfinite,
    /// The correction is applied and the extracted modal set does not reach the rigid frequency:
    /// the lowest frequency of its highest completed cluster is below f_r and it is not the whole
    /// finite spectrum (REQ-001, Extraction premise).
    ExtractionBelowRigidFrequency,
    /// The spectrum document's inputs fail one of the document unit's own checks of the request,
    /// the model's spectra, the formation records, the modal basis statement, the census and the
    /// rows (for example a formation record that is not the bound request's spectrum and scale
    /// factor, a roster that is not the census's, a census frame that is not orthonormal, or
    /// clusters that contradict the cluster rule). No solved document is returned.
    DocumentInputInvalid,
    /// An input the document binds or carries, a number or integer it would publish, or the
    /// assembled document is outside the checked canonical profile (too deep for the checked
    /// parse, not finite, or of magnitude 2^53 or more), so its hashes cannot be formed. No solved
    /// document is returned.
    DocumentOutsideCanonicalProfile,
}

impl RefusalCode {
    /// Every code, in declaration order.
    pub const ALL: [RefusalCode; 22] = [
        Self::RecordInvalid,
        Self::DampingOutOfRange,
        Self::DampingMismatch,
        Self::FrequencyOutsideRange,
        Self::QueryFrequencyInvalid,
        Self::ScaleFactorInvalid,
        Self::EvaluationAccuracyUnsupported,
        Self::MethodChoiceMissing,
        Self::CqcDampingUnsupported,
        Self::DirectionInvalid,
        Self::ModalMetadataInvalid,
        Self::RowInvalid,
        Self::ContributionInvalid,
        Self::ContributionNotPeriodic,
        Self::RigidFrequencyInvalid,
        Self::RigidResponseInvalid,
        Self::RigidResponsePremiseMismatch,
        Self::CombinationAccuracyUnsupported,
        Self::CombinationNonfinite,
        Self::ExtractionBelowRigidFrequency,
        Self::DocumentInputInvalid,
        Self::DocumentOutsideCanonicalProfile,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordInvalid => "SPECTRUM_RECORD_INVALID",
            Self::DampingOutOfRange => "SPECTRUM_DAMPING_OUT_OF_RANGE",
            Self::DampingMismatch => "SPECTRUM_DAMPING_MISMATCH",
            Self::FrequencyOutsideRange => "SPECTRUM_FREQUENCY_OUTSIDE_RANGE",
            Self::QueryFrequencyInvalid => "SPECTRUM_QUERY_FREQUENCY_INVALID",
            Self::ScaleFactorInvalid => "SPECTRUM_SCALE_FACTOR_INVALID",
            Self::EvaluationAccuracyUnsupported => "SPECTRUM_EVALUATION_ACCURACY_UNSUPPORTED",
            Self::MethodChoiceMissing => "SPECTRUM_METHOD_CHOICE_MISSING",
            Self::CqcDampingUnsupported => "SPECTRUM_CQC_DAMPING_UNSUPPORTED",
            Self::DirectionInvalid => "SPECTRUM_DIRECTION_INVALID",
            Self::ModalMetadataInvalid => "SPECTRUM_MODAL_METADATA_INVALID",
            Self::RowInvalid => "SPECTRUM_ROW_INVALID",
            Self::ContributionInvalid => "SPECTRUM_CONTRIBUTION_INVALID",
            Self::ContributionNotPeriodic => "SPECTRUM_CONTRIBUTION_NOT_PERIODIC",
            Self::RigidFrequencyInvalid => "SPECTRUM_RIGID_FREQUENCY_INVALID",
            Self::RigidResponseInvalid => "SPECTRUM_RIGID_RESPONSE_INVALID",
            Self::RigidResponsePremiseMismatch => "SPECTRUM_RIGID_RESPONSE_PREMISE_MISMATCH",
            Self::CombinationAccuracyUnsupported => "SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED",
            Self::CombinationNonfinite => "SPECTRUM_COMBINATION_NONFINITE",
            Self::ExtractionBelowRigidFrequency => "SPECTRUM_EXTRACTION_BELOW_RIGID_FREQUENCY",
            Self::DocumentInputInvalid => "SPECTRUM_DOCUMENT_INPUT_INVALID",
            Self::DocumentOutsideCanonicalProfile => "SPECTRUM_DOCUMENT_OUTSIDE_CANONICAL_PROFILE",
        }
    }
}

impl fmt::Display for RefusalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A named refusal: its code, the object refused (a spectrum's id, `query`, `request`, a mode, a
/// direction or a row key), the field where one applies, and a sentence saying why.
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
