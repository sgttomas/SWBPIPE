//! The report's numerical standing (DEL-08-01 REQ-024, REQ-017's standing
//! line, REQ-011's required content): the standing line, the numerical
//! standing by load case, the nonlinear support states and the statements of
//! what the solver's figures rest on.
//!
//! The input carries only the standing record the product reader derives
//! (`result_export::report_standing`), as typed tags and identifiers; every
//! boundary that holds the bound result re-derives it and requires equality.
//! Every word below is composed here. Each tag maps to its reviewed statement
//! through a match with no wildcard arm, so a tag without a statement does not
//! build, and a tag outside the vocabulary refuses the input when it is
//! decoded. Free-text detail is quoted as the solver's detail, escaped by the
//! emission, and never interpreted.
use open_pipe_stress_numerical_vocabulary::{
    CaseHealth, EvidenceStatus, FrictionNormal, HistoricalCause, HistoricalConfirmation, HistoricalIntegrityCode, NoFigureReason, Premise,
    StandingFinding, SupportBehavior, SupportState,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A vocabulary the input's tags are drawn from.
pub trait VocabularyTag: Copy + Sized + 'static {
    const VOCABULARY: &'static str;
    fn as_str(self) -> &'static str;
    fn parse(text: &str) -> Option<Self>;
}

macro_rules! vocabulary_tag {
    ($($t:ty),+) => {$(
        impl VocabularyTag for $t {
            const VOCABULARY: &'static str = <$t>::VOCABULARY;
            fn as_str(self) -> &'static str { <$t>::as_str(self) }
            fn parse(text: &str) -> Option<Self> { <$t>::parse(text) }
        }
    )+};
}
vocabulary_tag!(
    CaseHealth,
    EvidenceStatus,
    FrictionNormal,
    HistoricalCause,
    HistoricalConfirmation,
    HistoricalIntegrityCode,
    NoFigureReason,
    Premise,
    StandingFinding,
    SupportBehavior,
    SupportState
);

/// A tag on the wire: its vocabulary's text, decoded strictly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tag<T: VocabularyTag>(pub T);

impl<T: VocabularyTag> Serialize for Tag<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de, T: VocabularyTag> Deserialize<'de> for Tag<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        T::parse(&text)
            .map(Tag)
            .ok_or_else(|| serde::de::Error::custom(format!("{NUMERICAL_TAG_UNSUPPORTED}: {}", T::VOCABULARY)))
    }
}

/// The refusal of a tag outside the vocabulary its reader supports.
pub const NUMERICAL_TAG_UNSUPPORTED: &str = "NUMERICAL_TAG_UNSUPPORTED";
/// The only standing-record form this renderer reads.
pub const HISTORICAL_RECORD_VERSION: &str = "historical_1";

/// The standing record (`result_export::report_standing`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumericalStanding {
    pub record_version: String,
    pub evidence_member: String,
    pub figures: Figures,
    pub standing_findings: Vec<Tag<StandingFinding>>,
    pub counts: Vec<HealthCount>,
    pub cases: Vec<CaseStanding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Figures {
    pub status: Tag<EvidenceStatus>,
    pub reason: Tag<NoFigureReason>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthCount {
    pub health: Tag<CaseHealth>,
    pub cases: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaseStanding {
    pub case_id: String,
    pub health: Tag<CaseHealth>,
    pub integrity_code: Option<Tag<HistoricalIntegrityCode>>,
    pub integrity_diagnostic_id: Option<String>,
    pub causes: Vec<CauseRecord>,
    pub confirmation: ConfirmationRecord,
    pub supports: Vec<SupportRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CauseRecord {
    pub cause: Tag<HistoricalCause>,
    pub diagnostic_id: Option<String>,
    pub locations: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmationRecord {
    pub outcome: Tag<HistoricalConfirmation>,
    pub detail: Option<String>,
    pub status: Option<Tag<EvidenceStatus>>,
    pub premises: Vec<Tag<Premise>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportRecord {
    pub support_id: String,
    pub behavior: Tag<SupportBehavior>,
    pub friction_normal: Option<Tag<FrictionNormal>>,
    pub state: Tag<SupportState>,
    pub state_row_id: Option<String>,
}

// ---- reviewed statements: one per tag, no wildcard arm ---------------------------------

/// NQ-HEALTH's state words (docs/claims_registry.md).
pub fn health_words(health: CaseHealth) -> &'static str {
    match health {
        CaseHealth::ChecksPassed => "checks passed",
        CaseHealth::Sensitive => "sensitive",
        CaseHealth::QualityUnresolved => "quality unresolved",
        CaseHealth::SolveFailed => "solve failed",
        CaseHealth::NotAssessed => "not assessed",
    }
}

/// NQ-HEALTH's canonical sentence, which accompanies the checks-passed state.
pub const NQ_HEALTH_CANONICAL: &str =
    "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right.";

/// Each historical cause's statement.
pub fn cause_statement(cause: HistoricalCause) -> &'static str {
    match cause {
        HistoricalCause::Conditioning => "Sensitive: the stiffness matrix is ill-conditioned, by the solver's estimate of its reciprocal condition.",
        HistoricalCause::LoadFidelity => "Sensitive: some load rows differ from the exact net of their contributions, or could not be audited.",
        HistoricalCause::Formation => {
            "Sensitive: the solver's formation check found a displacement-error estimate, or a load row's formation noise, above its criterion of one part in a billion."
        }
        HistoricalCause::FormationRecovery => {
            "Sensitive: the solver's recovery guard did not pass for member or support values: a group's error figure above one part in a billion of its magnitude, an error figure or comparison it could not form, or a member-relative check not available on this route."
        }
        HistoricalCause::CurvedBendShort => "Sensitive: a realized bend is short relative to its neighbouring elements.",
        HistoricalCause::RecoveryBasisUnqualified => {
            "Quality unresolved: this load case has a gap support. No confirmation covers gap supports, and its member values have no error figure."
        }
        HistoricalCause::Mechanism => "Solve failed: the solver found the model to be a mechanism.",
        HistoricalCause::NegativeEnergy => "Solve failed: the solver found negative strain energy in the model.",
    }
}

/// The product reader's standing findings: each one's statement.
pub fn standing_finding_statement(finding: StandingFinding) -> &'static str {
    match finding {
        StandingFinding::NotQualified => "Not every load case of this result is checks passed, so the product reader does not find it numerically eligible.",
        StandingFinding::CaseEvidenceIncomplete => {
            "At least one load case's record is not a checks-passed record; each load case's numerical health is stated by load case."
        }
        StandingFinding::SourceBlocksCaseUnqualified => "At least one load case of the pressure source's recovery is not qualified.",
        StandingFinding::SourceBlocksPhysicalRowInspectionOnly => {
            "Some physical-quantity rows are given for inspection only: the pressure source's recovery does not qualify them."
        }
        StandingFinding::SourceBlocksEnvelopeUnqualified => {
            "The pressure source's recovery does not qualify the result as a whole, or the model has combinations it does not qualify."
        }
        StandingFinding::LoadReferenceSourceNotEligible => {
            "This result includes load/reference states, which the product reader does not find numerically eligible on this route."
        }
        StandingFinding::RetainedNotEligible => "The retained-precision reading does not find this result numerically eligible.",
    }
}

/// The paragraph that states the reader's standing findings, or none without any.
pub fn standing_findings_paragraph(standing: &NumericalStanding) -> Option<String> {
    if standing.standing_findings.is_empty() {
        return None;
    }
    Some(format!(
        "The product reader's findings on this result's standing: {}",
        standing.standing_findings.iter().map(|f| format!("{} ({})", standing_finding_statement(f.0), f.0.as_str())).collect::<Vec<_>>().join(" ")
    ))
}

/// The integrity code's display: the code itself, as the solver published it.
pub fn integrity_code_text(code: HistoricalIntegrityCode) -> &'static str {
    match code {
        HistoricalIntegrityCode::ChecksPassed
        | HistoricalIntegrityCode::Sensitive
        | HistoricalIntegrityCode::RecoveryBasisUnqualified
        | HistoricalIntegrityCode::AssemblyUnresolved
        | HistoricalIntegrityCode::Unresolved
        | HistoricalIntegrityCode::Failed
        | HistoricalIntegrityCode::PhysicalMechanism
        | HistoricalIntegrityCode::NegativeEnergy => code.as_str(),
    }
}

/// The computed state's words.
pub fn state_words(state: SupportState) -> &'static str {
    match state {
        SupportState::Inactive => "inactive",
        SupportState::Active => "active",
        SupportState::Sticking => "sticking",
        SupportState::Sliding => "sliding",
        SupportState::NotPublished => "not published by the solver",
    }
}

/// The support behaviour's words, with the friction normal's source.
pub fn behavior_words(behavior: SupportBehavior, normal: Option<FrictionNormal>) -> &'static str {
    match (behavior, normal) {
        (SupportBehavior::OneWay, _) => "one-way",
        (SupportBehavior::LiftOff, _) => "lift-off",
        (SupportBehavior::Gap, _) => "gap",
        (SupportBehavior::Friction, Some(FrictionNormal::Derived)) => "friction, normal force derived from another support's reaction",
        (SupportBehavior::Friction, Some(FrictionNormal::Explicit) | None) => "friction, normal force entered",
    }
}

/// The premise's display name (EV-FIGURE).
pub fn premise_display_name(premise: Premise) -> &'static str {
    match premise {
        Premise::InverseNormStep => "the solver's inverse-norm step",
    }
}

/// An S-NO-FIGURE reason statement.
pub fn no_figure_reason_statement(reason: NoFigureReason) -> &'static str {
    match reason {
        NoFigureReason::OlderDocument => "this result has no numerical evidence member from which this reader can derive an error figure here",
    }
}

/// S-NO-FIGURE.
pub fn no_figure_statement(reason: NoFigureReason) -> String {
    format!(
        "No error figure: {}. These values are shown for inspection with their load case's numerical health.",
        no_figure_reason_statement(reason)
    )
}

/// The evidence status's display form (EV-FIGURE), with its premise or reason.
pub fn status_form(status: EvidenceStatus, premises: &[Premise], reason: &str) -> String {
    match status {
        EvidenceStatus::Established => "established".to_string(),
        EvidenceStatus::Conditional => {
            format!("conditional on {}", premises.iter().map(|p| premise_display_name(*p)).collect::<Vec<_>>().join(" and "))
        }
        EvidenceStatus::Unavailable => format!("no error figure available: {reason}"),
        EvidenceStatus::NotEstablished => format!("not established: {reason}"),
        EvidenceStatus::PremiseRefuted => {
            format!("premise shown not to hold: {}", premises.iter().map(|p| premise_display_name(*p)).collect::<Vec<_>>().join(" and "))
        }
    }
}

/// The case-level confirmation's statement (the support-state confirmation
/// column of the case table).
pub fn confirmation_statement(outcome: HistoricalConfirmation) -> &'static str {
    match outcome {
        HistoricalConfirmation::SolverReportedEstablished => {
            "Not confirmed: the solver's confirmation reports every support state of this load case established, conditional on the solver's inverse-norm step. This result does not publish the deciding quantities or their intervals."
        }
        HistoricalConfirmation::NotAttemptedReasonNotRecorded => {
            "Not confirmed: no confirmation was attempted for this load case; the result does not record why."
        }
        HistoricalConfirmation::NotEstablishedDetailQuoted => {
            "Not confirmed: the solver's confirmation did not establish the support states of this load case. The result publishes one reason only, quoted below; it does not show that any other support's state was established."
        }
        HistoricalConfirmation::NoConfirmationRecord => "Not confirmed: the result records no support-state confirmation for this load case.",
        HistoricalConfirmation::NotApplicable => "No nonlinear supports.",
    }
}

/// One support's statement in one load case.
pub fn support_statement(support: &SupportRecord, outcome: HistoricalConfirmation) -> String {
    let state = state_words(support.state.0);
    let gap = support.behavior.0 == SupportBehavior::Gap;
    // No confirmation covers gap supports (DEL-04-01 REQ-016): a family fact, true
    // whatever this result records about its load case.
    let tail = match outcome {
        _ if gap => "Not confirmed: no confirmation covers gap supports.",
        HistoricalConfirmation::SolverReportedEstablished => {
            "Not confirmed: the solver's confirmation reports this state established, conditional on the solver's inverse-norm step. This result does not publish the deciding quantity or its interval."
        }
        HistoricalConfirmation::NotAttemptedReasonNotRecorded => {
            "Not confirmed: no confirmation was attempted for this load case; the result does not record why."
        }
        HistoricalConfirmation::NotEstablishedDetailQuoted => "Not confirmed: the solver's confirmation did not establish the support states of this load case.",
        HistoricalConfirmation::NoConfirmationRecord => "Not confirmed: the result records no support-state confirmation for this load case.",
        HistoricalConfirmation::NotApplicable => "Not confirmed.",
    };
    format!("Computed state: {state}. {tail}")
}

/// L-ERROR-FIGURE-BASIS, its title and its text.
pub const ERROR_FIGURE_BASIS_TITLE: &str = "What the solver's error figures rest on";
pub const ERROR_FIGURE_BASIS: &str = "What the solver's error figures rest on. For each load case whose member and support values carry an error figure, and for each support state the solver's confirmation reports established, the solver's computation takes 1,024 times an estimate of the size of the inverse of the scaled stiffness matrix as an upper limit on that size. The product does not establish that step for any model. On constructed models accepted through the product's ordinary input and solve path, the true size was 256 times the limit used. There, internal displacement-row intervals, including a one-way support's deciding displacement interval, did not contain the exact value: the distance from the interval centre to the exact value was about 22 to 256 times the stated half-width, and a one-way support was published inactive, with its state reported established and the load case reported as checks passed, where the exact solution has it active. In those models the deciding displacements at which a wrong state was published were of order 10\u{207b}\u{00b3}\u{2074} to 10\u{207b}\u{00b3}\u{2076} m, and the load terms that made those states wrong were 36 to 39 decimal orders below the main loads of about 190 to 700 N. How far these observations reach into models in ordinary use is not established. An output that relies on that step is never shown as established. Where its other required evidence is present and its other establishment conditions are met, it is shown as conditional on the step: it states what follows if the step holds for this model, and does not assert that it does. Where other evidence is missing or a state or comparison is inconclusive, its status states that reason.";
/// L-DISPLACEMENT-ESTIMATE.
pub const DISPLACEMENT_ESTIMATE: &str = "Nodal displacements and rotations carry no published error figure. At each free degree of freedom, the solver compares twice the absolute value of its first correction with one part in a billion of the larger of the computed component's magnitude and its body's coupled scale of the same kind. This is an error estimate by its construction, not an established limit on the error. Whether that comparison passed is part of the load case's numerical health.";
/// L-MAX-INTERVAL.
pub const MAX_INTERVAL: &str = "A stress maximum's printed interval covers only the arithmetic of forming the maximum from the recovered values. It does not include the error of the solution and is not an error figure for the maximum.";

/// The count of load cases in `health`.
fn count(standing: &NumericalStanding, health: CaseHealth) -> u64 {
    standing.counts.iter().find(|c| c.health.0 == health).map(|c| c.cases).unwrap_or(0)
}

/// The standing line (REQ-017): the solver's verdict for every load case with
/// its counts, the missing basis of any checks-passed verdict, and where the
/// evidence status of each figure and support state is stated.
pub fn standing_line(standing: &NumericalStanding) -> String {
    let n = standing.cases.len() as u64;
    let passed = count(standing, CaseHealth::ChecksPassed);
    let canonical = if passed > 0 { format!(" {NQ_HEALTH_CANONICAL}") } else { String::new() };
    if n > 0 && passed == n {
        format!(
            "Numerical health, as the solver reports it: checks passed for {}. The solver's checks-passed verdict rests on its inverse-norm step, which the product does not establish; the numerical conditions supporting this verdict have not been established. The evidence status of each figure and support state is stated with it; see Numerical Standing by Load Case and Limitations, What the solver's error figures rest on.{canonical}",
            if n == 1 { "its one load case".to_string() } else { format!("all {n} load cases") }
        )
    } else {
        let failed = count(standing, CaseHealth::SolveFailed);
        let failed_text = if failed > 0 { format!(", {failed} solve failed") } else { String::new() };
        format!(
            "Numerical health, as the solver reports it: {passed} checks passed, {} sensitive, {} quality unresolved, {} not assessed{failed_text}, of {n} {}. Any checks-passed verdict rests on the solver's inverse-norm step, which the product does not establish; the numerical conditions supporting this verdict have not been established. Each value carries its load case's health and the evidence status of its error figure; see Numerical Standing by Load Case and Limitations, What the solver's error figures rest on.{canonical}",
            count(standing, CaseHealth::Sensitive),
            count(standing, CaseHealth::QualityUnresolved),
            count(standing, CaseHealth::NotAssessed),
            if n == 1 { "load case" } else { "load cases" },
        )
    }
}

/// The rows of the numerical standing by load case: load case, numerical
/// health, integrity code, reasons, support-state confirmation, error figures.
pub fn case_rows(standing: &NumericalStanding) -> Vec<Vec<String>> {
    standing
        .cases
        .iter()
        .map(|case| {
            let reasons = if case.causes.is_empty() {
                "None named by the solver.".to_string()
            } else {
                case.causes.iter().map(|c| cause_statement(c.cause.0)).collect::<Vec<_>>().join(" ")
            };
            let mut confirmation = confirmation_statement(case.confirmation.outcome.0).to_string();
            if let Some(detail) = &case.confirmation.detail {
                confirmation.push_str(&format!(" Solver's detail: \u{201c}{detail}\u{201d}"));
            }
            vec![
                case.case_id.clone(),
                format!("Numerical health: {}", health_words(case.health.0)),
                case.integrity_code.map(|c| integrity_code_text(c.0).to_string()).unwrap_or_else(|| "none published".to_string()),
                reasons,
                confirmation,
                no_figure_statement(standing.figures.reason.0),
            ]
        })
        .collect()
}

/// The rows of the nonlinear support states: load case, support, behaviour,
/// statement.
pub fn support_rows(standing: &NumericalStanding) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    for case in &standing.cases {
        for support in &case.supports {
            rows.push(vec![
                case.case_id.clone(),
                support.support_id.clone(),
                behavior_words(support.behavior.0, support.friction_normal.map(|n| n.0)).to_string(),
                support_statement(support, case.confirmation.outcome.0),
            ]);
        }
    }
    rows
}

/// Every reviewed statement keyed by its tag, for the generated statement
/// table (`fixtures/reports/standing_statements.json`): `(vocabulary, tag,
/// statement)`. Templates keep their typed fields in braces.
pub fn statement_table() -> Vec<(&'static str, &'static str, String)> {
    let mut out = Vec::new();
    for h in CaseHealth::ALL {
        out.push((CaseHealth::VOCABULARY, h.as_str(), format!("Numerical health: {}", health_words(*h))));
    }
    for c in HistoricalIntegrityCode::ALL {
        out.push((HistoricalIntegrityCode::VOCABULARY, c.as_str(), integrity_code_text(*c).to_string()));
    }
    for c in HistoricalCause::ALL {
        out.push((HistoricalCause::VOCABULARY, c.as_str(), cause_statement(*c).to_string()));
    }
    for f in StandingFinding::ALL {
        out.push((StandingFinding::VOCABULARY, f.as_str(), standing_finding_statement(*f).to_string()));
    }
    for c in HistoricalConfirmation::ALL {
        out.push((HistoricalConfirmation::VOCABULARY, c.as_str(), confirmation_statement(*c).to_string()));
    }
    for s in SupportState::ALL {
        out.push((SupportState::VOCABULARY, s.as_str(), state_words(*s).to_string()));
    }
    for b in SupportBehavior::ALL {
        let normal = (*b == SupportBehavior::Friction).then_some(FrictionNormal::Explicit);
        out.push((SupportBehavior::VOCABULARY, b.as_str(), behavior_words(*b, normal).to_string()));
    }
    for n in FrictionNormal::ALL {
        out.push((FrictionNormal::VOCABULARY, n.as_str(), behavior_words(SupportBehavior::Friction, Some(*n)).to_string()));
    }
    for s in EvidenceStatus::ALL {
        out.push((EvidenceStatus::VOCABULARY, s.as_str(), status_form(*s, &[Premise::InverseNormStep], "{reason statement}")));
    }
    for p in Premise::ALL {
        out.push((Premise::VOCABULARY, p.as_str(), premise_display_name(*p).to_string()));
    }
    for r in NoFigureReason::ALL {
        out.push((NoFigureReason::VOCABULARY, r.as_str(), no_figure_statement(*r)));
    }
    out
}
