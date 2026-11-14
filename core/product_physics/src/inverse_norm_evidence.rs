//! The public, identity-free projection of a document's inverse-norm evidence
//! (DEL-04-01; carrier revision 4 §5). It is what the report lane's separate
//! evidence member is built from: never part of `contract_evidence`, never a
//! serialization of the in-memory carrier, and with no binding identity.
//!
//! Reader rules (part of this contract):
//! 1. `verified` and `not_verified` are emitted; `not_recorded` is derived by
//!    the reader wherever a required record is absent, and never emitted.
//! 2. A required stage is discharged only by an admitted `verified` record.
//! 2a. Withdrawn is its own state: a case whose `withdrawn` is `Some`, or a
//!    confirmation with an association entry, had evidence the product refused
//!    for disagreement. It is a defect signal, distinct from not recorded and
//!    from not verified, and it discharges nothing.
//! 3. A `not_verified` record is a failure of verification, never a refutation
//!    of the premise, and is shown even where coverage is incomplete.
//! 4. Omitted history is not evidence that an evaluation was not attempted; an
//!    application version never upgrades an old document.
//! 5. A duplicate offer leaves the admitted record; it appears only as an
//!    association entry.
//! 6. An unknown-slot offer appears only in `association`, with `slot: None`.
//! 7. The serialized spelling of the u64 fields is the report lane's choice.
//!
//! In this build no verifier exists: linear cases carry no record, and a
//! confirmation stage that formed its inputs shows `NoVerifierInThisBuild`,
//! meaning verification was not attempted. It is never an attempt.

use open_pipe_stress_frame_kernel::structural::{
    BoundMethod, BoundStage, InverseNormOutcome, InverseNormRecord, NotVerifiedReason, ShiftStep,
    StageKey, VerificationProgress,
};
use open_pipe_stress_nonlinear_integration::confirmation::{
    AssociationKind, ConfirmationRun, DiscardReason, PublicationUse, RequiredPlan, StageEvidence, StageProgress,
};
use serde::Serialize;

/// The format tag of this projection.
pub const INVERSE_NORM_EVIDENCE_FORMAT: &str = "swb-inverse-norm-evidence-v1";

/// One document's evidence: cases in case-loop order; held comparisons in
/// model combination order.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InverseNormEvidence {
    pub format: &'static str,
    pub cases: Vec<CaseEvidence>,
    pub held: Vec<HeldComparisonEvidence>,
}

impl InverseNormEvidence {
    pub fn empty() -> Self {
        Self { format: INVERSE_NORM_EVIDENCE_FORMAT, cases: Vec::new(), held: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CaseEvidence {
    pub case_id: String,
    pub verdict: VerdictMirror,
    pub published_basis: PublishedBasisView,
    pub required: Vec<OutputRequirement>,
    pub linear: Option<RecordView>,
    pub confirmation: Option<ConfirmationView>,
    /// Some: evidence existed and was withdrawn for disagreement (rule 2a).
    pub withdrawn: Option<WithdrawalView>,
}

/// The case's integrity verdict, from its integrity diagnostic's code, drawn
/// from the closed integrity codes. `NotAssessed`: the case has no integrity
/// diagnostic (its numerical quality is unassessed).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictMirror {
    ChecksPassed,
    Sensitive,
    Failed { code: IntegrityFailureCode },
    RecoveryBasisUnqualified,
    NotAssessed,
    /// A blocked document's retained evaluation (NUMERICAL_INTEGRITY_INSPECTION_ONLY):
    /// no published result relies on it; `prior` is its outcome before the block.
    InspectionOnly { prior: PriorVerdict },
}

/// An inspection-only case's outcome before its document was blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorVerdict {
    ChecksPassed,
    Sensitive,
}

/// The closed integrity failure codes (`integrity_failure_code` and the
/// force-scaling refusal).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum IntegrityFailureCode {
    #[serde(rename = "NUMERICAL_INTEGRITY_FAILED")]
    Failed,
    #[serde(rename = "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM")]
    PhysicalMechanism,
    #[serde(rename = "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY")]
    NegativeEnergy,
    #[serde(rename = "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED")]
    AssemblyUnresolved,
    #[serde(rename = "NUMERICAL_INTEGRITY_UNRESOLVED")]
    Unresolved,
}

impl IntegrityFailureCode {
    /// The closed code of `code`; `None` for any other string.
    pub fn of(code: &str) -> Option<Self> {
        Some(match code {
            "NUMERICAL_INTEGRITY_FAILED" => Self::Failed,
            "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM" => Self::PhysicalMechanism,
            "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY" => Self::NegativeEnergy,
            "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED" => Self::AssemblyUnresolved,
            "NUMERICAL_INTEGRITY_UNRESOLVED" => Self::Unresolved,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishedBasisView {
    None,
    Linear,
    ConfirmedStageB,
}

/// Why a case's evidence was withdrawn (carrier revision 4 §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawalView {
    MissingEntry,
    OrphanEntry,
    DuplicateDiagnostic,
    MirrorMismatch,
    /// The integrity diagnostic's code is outside the closed codes.
    UnknownCode,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OutputRequirement {
    pub output: OutputKind,
    pub stages: Vec<StageRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputKind {
    VerdictAndFigures,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StageRef {
    pub stage: StageTag,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StageTag {
    FormationCheck,
    ConfirmationStageA,
    ConfirmationInfluence,
    ConfirmationStageB,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmationView {
    /// `not_run`: no confirmation was attempted; `attempted`: otherwise,
    /// including a refusal before a plan formed (`plan = not_determined`).
    pub attempt: AttemptState,
    pub plan: PlanView,
    pub established: EstablishmentView,
    pub publication: PublicationView,
    pub association: Vec<AssociationView>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptState {
    NotRun,
    Attempted,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanView {
    Known(Vec<StageView>),
    NotDetermined,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EstablishmentView {
    Established,
    NotEstablished { reason: String },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationView {
    NotConsulted,
    UsedForPublishedBasis,
    Discarded { reason: DiscardTag },
}

/// The closed reasons a confirmation that ran was discarded at publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscardTag {
    LoadRowFinding,
    PreGuardNotPassed,
    DisplacementCorrectionUnavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StageView {
    pub stage: StageRef,
    pub progress: ProgressView,
    pub record: Option<RecordView>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressView {
    NotCalled,
    CallError { text: String },
    StoppedBeforeVerifier { finding: String },
    /// This build contains no inverse-norm verifier; verification was not
    /// attempted. Never rendered as an attempt; removed with the certificate.
    NoVerifierInThisBuild,
    VerifierReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    Verified,
    NotVerified,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RecordView {
    pub stage: StageRef,
    pub status: RecordStatus,
    pub estimate_limit_bits: u64,
    pub est_bits: u64,
    pub matrix_digest: u64,
    pub inputs_digest: Option<u64>,
    pub verified: Option<VerifiedView>,
    pub not_verified: Option<NotVerifiedView>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifiedView {
    pub bound_bits: u64,
    pub block_count: u32,
    pub largest_block: u32,
    /// E at σ₁, E at σ₂, D.
    pub blocks_by_method: Vec<MethodCount>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NotVerifiedView {
    pub block_count: u32,
    pub largest_block: u32,
    pub blocks_verified: u32,
    /// In method order.
    pub attempts: Vec<AttemptView>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MethodTag {
    LemmaEShiftedFactorization,
    LemmaDComparisonMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShiftTag {
    Sigma1,
    Sigma2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonTag {
    LemmaENonpositivePivot,
    LemmaESigmaNonpositive,
    LemmaDNonpositivePivot,
    LemmaDContractionNotBelowOne,
    ArithmeticNonfinite,
    BoundExceedsExistingQuantity,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MethodCount {
    pub method: MethodTag,
    pub shift: Option<ShiftTag>,
    pub blocks: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AttemptView {
    pub method: MethodTag,
    /// In reason order.
    pub reasons: Vec<ReasonCount>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReasonCount {
    pub reason: ReasonTag,
    pub blocks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssociationTag {
    ForeignEvaluation,
    ForeignRecord,
    FingerprintMismatch,
    DuplicateOffer,
    UnknownSlot,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AssociationView {
    pub slot: Option<StageRef>,
    pub kind: AssociationTag,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeldOutcomeView {
    Withheld { code: String },
    Admitted { kind: String },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HeldComparisonEvidence {
    pub comparison_id: String,
    pub minuend_id: String,
    pub target_id: String,
    pub outcome: HeldOutcomeView,
    /// The held solve's own confirmation evidence, where carrier revision 4 §6 keeps it.
    pub confirmation: Option<ConfirmationView>,
}

pub(crate) fn stage_tag(stage: BoundStage) -> StageTag {
    match stage {
        BoundStage::FormationCheck => StageTag::FormationCheck,
        BoundStage::ConfirmationStageA => StageTag::ConfirmationStageA,
        BoundStage::ConfirmationInfluence => StageTag::ConfirmationInfluence,
        BoundStage::ConfirmationStageB => StageTag::ConfirmationStageB,
    }
}

pub(crate) fn stage_ref(key: &StageKey) -> StageRef {
    StageRef { stage: stage_tag(key.stage), subject: key.subject.clone() }
}

fn method_tag(method: BoundMethod) -> MethodTag {
    match method {
        BoundMethod::LemmaEShiftedFactorization => MethodTag::LemmaEShiftedFactorization,
        BoundMethod::LemmaDComparisonMatrix => MethodTag::LemmaDComparisonMatrix,
    }
}

fn reason_tag(reason: NotVerifiedReason) -> ReasonTag {
    match reason {
        NotVerifiedReason::LemmaENonpositivePivot => ReasonTag::LemmaENonpositivePivot,
        NotVerifiedReason::LemmaESigmaNonpositive => ReasonTag::LemmaESigmaNonpositive,
        NotVerifiedReason::LemmaDNonpositivePivot => ReasonTag::LemmaDNonpositivePivot,
        NotVerifiedReason::LemmaDContractionNotBelowOne => ReasonTag::LemmaDContractionNotBelowOne,
        NotVerifiedReason::ArithmeticNonfinite => ReasonTag::ArithmeticNonfinite,
        NotVerifiedReason::BoundExceedsExistingQuantity => ReasonTag::BoundExceedsExistingQuantity,
    }
}

/// A record's deterministic fields; no binding.
pub(crate) fn record_view(record: &InverseNormRecord) -> RecordView {
    let (status, verified, not_verified) = match record.outcome() {
        InverseNormOutcome::Verified(summary) => (
            RecordStatus::Verified,
            Some(VerifiedView {
                bound_bits: summary.bound_bits(),
                block_count: summary.block_count(),
                largest_block: summary.largest_block(),
                blocks_by_method: summary
                    .blocks_by_method()
                    .iter()
                    .map(|&(method, shift, blocks)| MethodCount {
                        method: method_tag(method),
                        shift: shift.map(|s| match s {
                            ShiftStep::Sigma1 => ShiftTag::Sigma1,
                            ShiftStep::Sigma2 => ShiftTag::Sigma2,
                        }),
                        blocks,
                    })
                    .collect(),
            }),
            None,
        ),
        InverseNormOutcome::NotVerified(summary) => (
            RecordStatus::NotVerified,
            None,
            Some(NotVerifiedView {
                block_count: summary.block_count(),
                largest_block: summary.largest_block(),
                blocks_verified: summary.blocks_verified(),
                attempts: summary
                    .attempts()
                    .iter()
                    .map(|attempt| AttemptView {
                        method: method_tag(attempt.method()),
                        reasons: attempt
                            .reasons()
                            .iter()
                            .map(|&(reason, blocks)| ReasonCount { reason: reason_tag(reason), blocks })
                            .collect(),
                    })
                    .collect(),
            }),
        ),
    };
    RecordView {
        stage: stage_ref(record.key()),
        status,
        estimate_limit_bits: record.estimate_limit_bits(),
        est_bits: record.est_bits(),
        matrix_digest: record.matrix_digest(),
        inputs_digest: record.inputs_digest(),
        verified,
        not_verified,
    }
}

fn progress_view(progress: &StageProgress, finding: Option<String>) -> ProgressView {
    match progress {
        StageProgress::NotCalled => ProgressView::NotCalled,
        StageProgress::CallError(text) => ProgressView::CallError { text: text.clone() },
        StageProgress::Evaluated(VerificationProgress::StoppedBeforeVerifier) => {
            ProgressView::StoppedBeforeVerifier { finding: finding.unwrap_or_default() }
        }
        StageProgress::Evaluated(VerificationProgress::VerifierNotRun) => ProgressView::NoVerifierInThisBuild,
        StageProgress::Evaluated(VerificationProgress::VerifierReached) => ProgressView::VerifierReached,
    }
}

fn association_tag(kind: AssociationKind) -> AssociationTag {
    match kind {
        AssociationKind::ForeignEvaluation => AssociationTag::ForeignEvaluation,
        AssociationKind::ForeignRecord => AssociationTag::ForeignRecord,
        AssociationKind::FingerprintMismatch => AssociationTag::FingerprintMismatch,
        AssociationKind::DuplicateOffer => AssociationTag::DuplicateOffer,
        AssociationKind::UnknownSlot => AssociationTag::UnknownSlot,
    }
}

/// A stage's own formation-check finding, spelled as the reasons spell it.
fn render_finding(finding: &open_pipe_stress_frame_kernel::structural::FormationCheck) -> String {
    match &finding.reason {
        open_pipe_stress_frame_kernel::structural::FormationCheckReason::Estimate => "estimate".to_string(),
        open_pipe_stress_frame_kernel::structural::FormationCheckReason::FormationCheckUnavailable { detail } => detail.clone(),
    }
}

/// A confirmation's evidence, with its establishment, as the reader sees it.
pub(crate) fn confirmation_view(evidence: &StageEvidence, established: EstablishmentView) -> ConfirmationView {
    let plan = match evidence.plan() {
        RequiredPlan::NotDetermined => PlanView::NotDetermined,
        RequiredPlan::Known(stages) => PlanView::Known(
            stages
                .iter()
                .map(|stage| StageView {
                    stage: stage_ref(stage.key()),
                    progress: progress_view(stage.progress(), stage.finding().map(render_finding)),
                    record: stage.record().map(record_view),
                })
                .collect(),
        ),
    };
    let association = evidence
        .association_failures()
        .iter()
        .map(|failure| AssociationView {
            slot: failure.slot.and_then(|slot| evidence.stages().get(slot)).map(|stage| stage_ref(stage.key())),
            kind: association_tag(failure.kind),
        })
        .collect();
    let publication = match evidence.publication() {
        PublicationUse::NotConsulted => PublicationView::NotConsulted,
        PublicationUse::UsedForPublishedBasis => PublicationView::UsedForPublishedBasis,
        PublicationUse::Discarded { reason } => PublicationView::Discarded {
            reason: match reason {
                DiscardReason::LoadRowFinding => DiscardTag::LoadRowFinding,
                DiscardReason::PreGuardNotPassed => DiscardTag::PreGuardNotPassed,
                DiscardReason::DisplacementCorrectionUnavailable => DiscardTag::DisplacementCorrectionUnavailable,
            },
        },
    };
    let attempt = match evidence.attempt() {
        ConfirmationRun::NotRun => AttemptState::NotRun,
        ConfirmationRun::Attempted => AttemptState::Attempted,
    };
    ConfirmationView { attempt, plan, established, publication, association }
}

/// The stages a confirmed case's verdict and figures rest on: the
/// confirmation's own Known plan (carrier revision 4 §5).
pub(crate) fn confirmation_requirement(evidence: &StageEvidence) -> Option<OutputRequirement> {
    match evidence.plan() {
        RequiredPlan::Known(stages) => Some(OutputRequirement {
            output: OutputKind::VerdictAndFigures,
            stages: stages.iter().map(|stage| stage_ref(stage.key())).collect(),
        }),
        RequiredPlan::NotDetermined => None,
    }
}

/// A linear case's verdict and figures rest on its formation check.
pub(crate) fn linear_requirement() -> OutputRequirement {
    OutputRequirement {
        output: OutputKind::VerdictAndFigures,
        stages: vec![StageRef { stage: StageTag::FormationCheck, subject: None }],
    }
}
