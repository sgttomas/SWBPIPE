//! The closed numerical vocabulary (DEL-08-01 REQ-024 and AC-016; DEL-05-02
//! Standing). It holds tags only: each vocabulary is a closed enum with its
//! wire text, its complete list and a strict parse. There is no logic here and
//! no dependency, so producer, reader, renderer and desktop crates can all
//! depend on it.
//!
//! Rules every consumer keeps:
//! - A tag crosses a boundary as its wire text. A reader parses it with
//!   `parse`, which accepts exactly the listed texts; anything else is refused
//!   by name and never given a known meaning.
//! - Dynamic identifiers, quantities and detail text travel in separate typed
//!   fields, never inside a tag. Free-text detail is quoted faithfully and
//!   escaped, and never interpreted.
//! - The renderer and the desktop map every tag to its reviewed statement with
//!   a match that has no wildcard arm, so a tag without a statement does not
//!   build. `catalogue` lists every tag for the generated catalogue that a
//!   maintained test compares with the statement table in both directions.
//!
//! Vocabularies whose names begin `historical_` are read by the historical
//! reader (`result_export::report_standing`) from results that carry no typed
//! numerical evidence: they are recovered from a result's published integrity
//! code, solve quality and fixed diagnostic text. They are not the typed
//! producer vocabulary, and they stay supported for results made before typed
//! emission.

/// Declares one closed vocabulary.
macro_rules! vocabulary {
    ($(#[$meta:meta])* $name:ident, $vocabulary:literal, { $($(#[$vmeta:meta])* $variant:ident => $text:literal,)+ }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $name {
            $($(#[$vmeta])* $variant,)+
        }

        impl $name {
            /// This vocabulary's name in the catalogue.
            pub const VOCABULARY: &'static str = $vocabulary;
            /// Every tag, in declaration order.
            pub const ALL: &'static [$name] = &[$($name::$variant,)+];

            /// The tag's wire text.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $text,)+
                }
            }

            /// The tag whose wire text is exactly `text`; `None` for any other text.
            pub fn parse(text: &str) -> Option<Self> {
                match text {
                    $($text => Some($name::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

vocabulary!(
    /// A load case's numerical health, as the solver reports it
    /// (`numerical_quality.cases[].solve_quality`: `checks_passed`,
    /// `sensitive`, `unresolved`, `failed`, `not_assessed`).
    CaseHealth, "case_health", {
        ChecksPassed => "checks_passed",
        Sensitive => "sensitive",
        QualityUnresolved => "quality_unresolved",
        SolveFailed => "solve_failed",
        NotAssessed => "not_assessed",
    }
);

vocabulary!(
    /// The integrity codes the producer publishes on a load case's
    /// `diagnostic:numerical-integrity:<case>` diagnostic
    /// (product_physics `assessed_numerical_quality`).
    HistoricalIntegrityCode, "historical_integrity_code", {
        ChecksPassed => "NUMERICAL_INTEGRITY_CHECKS_PASSED",
        Sensitive => "NUMERICAL_INTEGRITY_SENSITIVE",
        RecoveryBasisUnqualified => "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED",
        AssemblyUnresolved => "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED",
        Unresolved => "NUMERICAL_INTEGRITY_UNRESOLVED",
        Failed => "NUMERICAL_INTEGRITY_FAILED",
        PhysicalMechanism => "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM",
        NegativeEnergy => "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY",
    }
);

vocabulary!(
    /// The causes the historical health reader recovers
    /// (`result_export::historical_health`), each only where its evidence is
    /// present at its fixed position.
    HistoricalCause, "historical_cause", {
        /// The integrity report's reciprocal condition estimate, on a linear
        /// sensitive case.
        Conditioning => "conditioning",
        /// A `LOAD_CONTRIBUTION_ABSORBED` diagnostic naming the case.
        LoadFidelity => "load_fidelity",
        /// The formation check's estimate line, or the S11-G load-row sentence.
        Formation => "formation",
        /// The S11-G recovery-guard sentence, or the member-relative check
        /// being unavailable on the route.
        FormationRecovery => "formation_recovery",
        /// A `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS` diagnostic naming the case.
        CurvedBendShort => "curved_bend_short",
        /// The integrity code `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`.
        RecoveryBasisUnqualified => "recovery_basis_unqualified",
        /// The integrity code `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`.
        Mechanism => "mechanism",
        /// The integrity code `NUMERICAL_INTEGRITY_NEGATIVE_ENERGY`.
        NegativeEnergy => "negative_energy",
    }
);

vocabulary!(
    /// What a result made before typed emission publishes about a case's
    /// support-state confirmation, recovered from the fixed text the producer
    /// appends to the case's integrity diagnostic.
    HistoricalConfirmation, "historical_confirmation", {
        /// The producer's confirmed-state line is present.
        SolverReportedEstablished => "solver_reported_established",
        /// The not-established sentence with the reason "confirmation attempt:
        /// not run". The published text does not say why it was not run.
        NotAttemptedReasonNotRecorded => "not_attempted_reason_not_recorded",
        /// The not-established sentence with any other reason. The reason is
        /// quoted as the solver's detail and given no meaning. Only the first
        /// failing support's reason is published.
        NotEstablishedDetailQuoted => "not_established_detail_quoted",
        /// A case with nonlinear supports whose integrity diagnostic carries
        /// neither sentence.
        NoConfirmationRecord => "no_confirmation_record",
        /// A case of a model with no nonlinear support.
        NotApplicable => "not_applicable",
    }
);

vocabulary!(
    /// The product reader's findings that state a readable result's numerical
    /// standing (DEL-05-02 Standing): `reader_verdict::standing`'s findings
    /// that do not make the result unreadable.
    StandingFinding, "standing_finding", {
        NotQualified => "NUMERICAL_INTEGRITY_NOT_QUALIFIED",
        CaseEvidenceIncomplete => "NUMERICAL_CASE_EVIDENCE_INCOMPLETE",
        SourceBlocksCaseUnqualified => "SOURCE_BLOCKS_CASE_UNQUALIFIED",
        SourceBlocksPhysicalRowInspectionOnly => "SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY",
        SourceBlocksEnvelopeUnqualified => "SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED",
        LoadReferenceSourceNotEligible => "LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1",
        RetainedNotEligible => "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE",
    }
);

vocabulary!(
    /// A nonlinear support's computed state in a load case
    /// (`nonlinear_support_active_set_state_code`: 0 inactive, 1 active,
    /// 2 sticking, 3 sliding).
    SupportState, "support_state", {
        Inactive => "inactive",
        Active => "active",
        Sticking => "sticking",
        Sliding => "sliding",
        /// No state row, or a state code outside the published convention.
        NotPublished => "not_published",
    }
);

vocabulary!(
    /// A nonlinear support's behaviour, as the model enters it (each of the
    /// model schema's accepted spellings reads as one of these).
    SupportBehavior, "support_behavior", {
        OneWay => "one_way",
        LiftOff => "lift_off",
        Gap => "gap",
        Friction => "friction",
    }
);

vocabulary!(
    /// Where a friction support's normal force comes from, as the model enters
    /// it: an explicit value, or the reaction of a named support (derived), which
    /// couples the friction force to that reaction.
    FrictionNormal, "friction_normal", {
        Explicit => "explicit",
        Derived => "derived",
    }
);

vocabulary!(
    /// The evidence status of a figure, a support state or a comparison
    /// (release target exit criterion 18; claims registry EV-FIGURE).
    EvidenceStatus, "evidence_status", {
        Established => "established",
        /// What follows if its named premises hold; it does not assert them.
        Conditional => "conditional",
        /// No error figure is available, with its reason.
        Unavailable => "unavailable",
        /// The required conditions are not established, with the reason.
        NotEstablished => "not_established",
        /// The product establishes that a named premise fails. Emitted by
        /// nothing until the product can establish such a failure.
        PremiseRefuted => "premise_refuted",
    }
);

vocabulary!(
    /// A premise an output may rest on.
    Premise, "premise", {
        /// The solver's step taking 1,024 times an estimate of the inverse
        /// norm of the scaled stiffness matrix as an upper limit on it
        /// (DEL-04-01 OUT-001, the row of REQ-015).
        InverseNormStep => "inverse_norm_step",
    }
);

vocabulary!(
    /// Why a figure is unavailable (statement S-NO-FIGURE).
    NoFigureReason, "no_figure_reason", {
        /// The result has no numerical evidence member from which the reader
        /// can derive an error figure.
        OlderDocument => "older_document",
    }
);

/// One catalogue entry: a vocabulary's name and one of its tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CatalogueEntry {
    pub vocabulary: &'static str,
    pub tag: &'static str,
}

/// Every tag of every vocabulary, vocabulary by vocabulary in declaration
/// order. The generated catalogue (`fixtures/reports/numerical_vocabulary.json`)
/// is written from this list, never by scanning source.
pub fn catalogue() -> Vec<CatalogueEntry> {
    fn of<T: Copy + 'static>(vocabulary: &'static str, all: &'static [T], text: fn(T) -> &'static str) -> impl Iterator<Item = CatalogueEntry> {
        all.iter().map(move |tag| CatalogueEntry { vocabulary, tag: text(*tag) })
    }
    of(CaseHealth::VOCABULARY, CaseHealth::ALL, CaseHealth::as_str)
        .chain(of(HistoricalIntegrityCode::VOCABULARY, HistoricalIntegrityCode::ALL, HistoricalIntegrityCode::as_str))
        .chain(of(HistoricalCause::VOCABULARY, HistoricalCause::ALL, HistoricalCause::as_str))
        .chain(of(HistoricalConfirmation::VOCABULARY, HistoricalConfirmation::ALL, HistoricalConfirmation::as_str))
        .chain(of(StandingFinding::VOCABULARY, StandingFinding::ALL, StandingFinding::as_str))
        .chain(of(SupportState::VOCABULARY, SupportState::ALL, SupportState::as_str))
        .chain(of(SupportBehavior::VOCABULARY, SupportBehavior::ALL, SupportBehavior::as_str))
        .chain(of(FrictionNormal::VOCABULARY, FrictionNormal::ALL, FrictionNormal::as_str))
        .chain(of(EvidenceStatus::VOCABULARY, EvidenceStatus::ALL, EvidenceStatus::as_str))
        .chain(of(Premise::VOCABULARY, Premise::ALL, Premise::as_str))
        .chain(of(NoFigureReason::VOCABULARY, NoFigureReason::ALL, NoFigureReason::as_str))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every tag parses back to itself, and nothing else parses.
    #[test]
    fn every_tag_round_trips_and_only_listed_text_parses() {
        fn check<T: Copy + PartialEq + std::fmt::Debug>(all: &[T], text: fn(T) -> &'static str, parse: fn(&str) -> Option<T>) {
            for tag in all {
                let wire = text(*tag);
                assert_eq!(parse(wire), Some(*tag));
                assert_eq!(parse(&format!(" {wire}")), None);
                assert_eq!(parse(&format!("{wire} ")), None);
                if wire.contains('_') {
                    assert_eq!(parse(&wire.replace('_', "-")), None);
                }
                let flipped: String = wire.chars().map(|c| if c.is_ascii_lowercase() { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() }).collect();
                assert_eq!(parse(&flipped), None);
            }
            assert_eq!(parse(""), None);
        }
        check(CaseHealth::ALL, CaseHealth::as_str, CaseHealth::parse);
        check(HistoricalIntegrityCode::ALL, HistoricalIntegrityCode::as_str, HistoricalIntegrityCode::parse);
        check(HistoricalCause::ALL, HistoricalCause::as_str, HistoricalCause::parse);
        check(HistoricalConfirmation::ALL, HistoricalConfirmation::as_str, HistoricalConfirmation::parse);
        check(StandingFinding::ALL, StandingFinding::as_str, StandingFinding::parse);
        check(SupportState::ALL, SupportState::as_str, SupportState::parse);
        check(SupportBehavior::ALL, SupportBehavior::as_str, SupportBehavior::parse);
        check(FrictionNormal::ALL, FrictionNormal::as_str, FrictionNormal::parse);
        check(EvidenceStatus::ALL, EvidenceStatus::as_str, EvidenceStatus::parse);
        check(Premise::ALL, Premise::as_str, Premise::parse);
        check(NoFigureReason::ALL, NoFigureReason::as_str, NoFigureReason::parse);
        assert_eq!(HistoricalIntegrityCode::parse("NUMERICAL_INTEGRITY_SENSITIVE_ENOUGH"), None);
    }

    /// No two entries of one vocabulary share a tag, and no two vocabularies
    /// share a name.
    #[test]
    fn the_catalogue_is_unique() {
        let entries = catalogue();
        let mut seen = std::collections::BTreeSet::new();
        for entry in &entries {
            assert!(seen.insert((entry.vocabulary, entry.tag)), "{entry:?}");
        }
        let names: std::collections::BTreeSet<_> = entries.iter().map(|e| e.vocabulary).collect();
        assert_eq!(names.len(), 11);
    }
}
