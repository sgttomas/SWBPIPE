//! The confirmation's inverse-norm evidence (DEL-04-01; carrier revision 4 §2):
//! the required plan formed before any evaluation, each planned stage's own
//! progress and admitted record, the confirmation-level publication use, and
//! typed association failures. Bindings are in-memory identities, never
//! serialized; a record is admitted only through `collect`.

use open_pipe_stress_frame_kernel::structural::{
    BoundStage, EvaluationBinding, FormationCheck, FormationEvaluation, FormationParts,
    InverseNormRecord, MemberBoundInputs, StageKey, StructuralError, VerificationProgress,
};

/// One planned stage's progress, set once, from this slot's own call.
#[derive(Debug, Clone, PartialEq)]
pub enum StageProgress {
    /// An earlier step stopped the attempt before this call.
    NotCalled,
    /// This slot's own `evaluate_at` returned `Err` (its text).
    CallError(String),
    /// This slot's own evaluation returned; its progress.
    Evaluated(VerificationProgress),
}

/// Whether a confirmation was attempted at all (the report lane's need 3):
/// `NotRun` only for the explicit not-run branches; every other evidence,
/// including a refusal before a plan formed, is `Attempted`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationRun {
    NotRun,
    Attempted,
}

/// Whether publication used this confirmation (carrier revision 4 §2, R3-F3).
/// Confirmation-level, beside the per-stage progress, which it never replaces.
#[derive(Debug, Clone, PartialEq)]
pub enum PublicationUse {
    /// Publication never read this confirmation (the value at plan formation).
    NotConsulted,
    /// The published rows rest on this confirmation's Stage B.
    UsedForPublishedBasis,
    /// The confirmation ran, then publication discarded it, for a closed reason.
    Discarded { reason: DiscardReason },
}

/// Why publication discarded a confirmation that ran (carrier revision 5):
/// the eligibility filter's two conditions, and the displacement fallback,
/// decided before the confirmed-for-publication decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscardReason {
    /// The case's load-row finding made it ineligible.
    LoadRowFinding,
    /// The case's integrity diagnostic was not checks-passed before the guard.
    PreGuardNotPassed,
    /// The corrected displacement could not be formed; every row is the solve's own.
    DisplacementCorrectionUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssociationKind {
    /// The evaluation's binding is absent or another slot's: nothing adopted.
    ForeignEvaluation,
    /// The evaluation is the slot's own, its record is not: progress adopted, record refused.
    ForeignRecord,
    /// The record belongs, its fingerprint does not match the live inputs.
    FingerprintMismatch,
    /// The slot was already collected: the first collection stands.
    DuplicateOffer,
    /// No such slot in the Known plan, or the plan is NotDetermined.
    UnknownSlot,
}

/// A refused offer. `slot` is `None` for an unknown slot. Never a record.
#[derive(Debug, Clone, PartialEq)]
pub struct AssociationFailure {
    pub slot: Option<usize>,
    pub kind: AssociationKind,
}

/// One planned stage.
#[derive(Debug, Clone)]
pub struct PlannedStage {
    binding: EvaluationBinding,
    progress: StageProgress,
    finding: Option<FormationCheck>,
    record: Option<InverseNormRecord>,
}

impl PlannedStage {
    pub fn key(&self) -> &StageKey {
        self.binding.key()
    }
    pub fn progress(&self) -> &StageProgress {
        &self.progress
    }
    pub fn finding(&self) -> Option<&FormationCheck> {
        self.finding.as_ref()
    }
    pub fn record(&self) -> Option<&InverseNormRecord> {
        self.record.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum RequiredPlan {
    Known(Vec<PlannedStage>),
    /// The empty evidence of every exit before a required plan can be formed.
    NotDetermined,
}

/// Why `collect` refused to hand parts back.
#[derive(Debug)]
pub(crate) enum CollectRefusal {
    /// The slot's own call returned this error.
    CallError(StructuralError),
    /// The evaluation is foreign or unbound.
    Foreign,
    /// The slot was already collected.
    Duplicate,
    /// No such slot.
    UnknownSlot,
}

/// The confirmation's evidence. Read-only outside this crate except for the
/// publication use, which publication sets.
#[derive(Debug, Clone)]
pub struct StageEvidence {
    plan: RequiredPlan,
    matrix_digest: Option<u64>,
    b_inputs_digest: Option<u64>,
    publication: PublicationUse,
    association: Vec<AssociationFailure>,
    attempt: ConfirmationRun,
}

impl StageEvidence {
    /// The evidence of an explicitly not-run confirmation. Carries nothing.
    pub fn not_run() -> Self {
        Self { attempt: ConfirmationRun::NotRun, ..Self::not_determined() }
    }
    pub fn attempt(&self) -> ConfirmationRun {
        self.attempt
    }
    /// The evidence of an exit before a plan exists. Carries nothing.
    pub fn not_determined() -> Self {
        Self {
            plan: RequiredPlan::NotDetermined,
            matrix_digest: None,
            b_inputs_digest: None,
            publication: PublicationUse::NotConsulted,
            association: Vec::new(),
            attempt: ConfirmationRun::Attempted,
        }
    }

    /// The plan, before any evaluation: one fresh binding per key, every
    /// stage `NotCalled`. Keys must be legal and unique (debug-checked here;
    /// `verifies_all_required_premises` checks them again).
    pub(crate) fn planned(keys: Vec<StageKey>) -> Self {
        debug_assert!(plan_is_legal(&keys), "confirmation plan: illegal or duplicate keys");
        let stages = keys
            .into_iter()
            .map(|key| PlannedStage {
                binding: EvaluationBinding::for_stage(key),
                progress: StageProgress::NotCalled,
                finding: None,
                record: None,
            })
            .collect();
        Self { plan: RequiredPlan::Known(stages), ..Self::not_determined() }
    }

    /// The attempt's matrix fingerprint, recorded once the attempt exists.
    pub(crate) fn set_matrix_digest(&mut self, digest: u64) {
        self.matrix_digest = Some(digest);
    }

    pub fn plan(&self) -> &RequiredPlan {
        &self.plan
    }
    /// The planned stages; empty when NotDetermined.
    pub fn stages(&self) -> &[PlannedStage] {
        match &self.plan {
            RequiredPlan::Known(stages) => stages,
            RequiredPlan::NotDetermined => &[],
        }
    }
    pub fn matrix_digest(&self) -> Option<u64> {
        self.matrix_digest
    }
    pub fn publication(&self) -> &PublicationUse {
        &self.publication
    }
    pub fn association_failures(&self) -> &[AssociationFailure] {
        &self.association
    }
    /// Publication records whether it used or discarded this confirmation.
    pub fn set_publication(&mut self, use_: PublicationUse) {
        self.publication = use_;
    }

    /// The slot index of `key`, when planned.
    pub(crate) fn slot_of(&self, stage: BoundStage, subject: Option<&str>) -> Option<usize> {
        self.stages().iter().position(|s| s.key().stage == stage && s.key().subject.as_deref() == subject)
    }

    /// Runs the slot's own call under the slot's binding and collects its
    /// result: the only way production code evaluates a planned stage.
    pub(crate) fn evaluate_slot(
        &mut self,
        slot: Option<usize>,
        call: impl FnOnce(&EvaluationBinding) -> Result<FormationEvaluation, StructuralError>,
    ) -> Result<FormationParts, CollectRefusal> {
        let Some(slot) = slot else {
            self.association.push(AssociationFailure { slot: None, kind: AssociationKind::UnknownSlot });
            return Err(CollectRefusal::UnknownSlot);
        };
        let Some(binding) = self.stages().get(slot).map(|s| s.binding.clone()) else {
            self.association.push(AssociationFailure { slot: None, kind: AssociationKind::UnknownSlot });
            return Err(CollectRefusal::UnknownSlot);
        };
        let result = call(&binding);
        self.collect(slot, result)
    }

    /// Collection (carrier revision 4 §2), in order: slot; once; the call's own
    /// error; the whole evaluation's binding; progress; the record; Stage B's
    /// fingerprint. Hands the parts back so the caller keeps the finding and
    /// inputs.
    pub(crate) fn collect(
        &mut self,
        slot: usize,
        result: Result<FormationEvaluation, StructuralError>,
    ) -> Result<FormationParts, CollectRefusal> {
        // 1. Slot.
        let RequiredPlan::Known(stages) = &mut self.plan else {
            self.association.push(AssociationFailure { slot: None, kind: AssociationKind::UnknownSlot });
            return Err(CollectRefusal::UnknownSlot);
        };
        let Some(stage) = stages.get_mut(slot) else {
            self.association.push(AssociationFailure { slot: None, kind: AssociationKind::UnknownSlot });
            return Err(CollectRefusal::UnknownSlot);
        };
        // 2. Once.
        if stage.progress != StageProgress::NotCalled {
            self.association.push(AssociationFailure { slot: Some(slot), kind: AssociationKind::DuplicateOffer });
            return Err(CollectRefusal::Duplicate);
        }
        // 3. The call's own error.
        let evaluation = match result {
            Ok(evaluation) => evaluation,
            Err(error) => {
                stage.progress = StageProgress::CallError(error.to_string());
                return Err(CollectRefusal::CallError(error));
            }
        };
        // 4. The whole evaluation's identity, before anything is read from it.
        if !evaluation.is_bound_to(&stage.binding) {
            self.association.push(AssociationFailure { slot: Some(slot), kind: AssociationKind::ForeignEvaluation });
            return Err(CollectRefusal::Foreign);
        }
        // 5. Progress and finding from the slot's own evaluation.
        stage.progress = StageProgress::Evaluated(evaluation.progress());
        let parts = evaluation.into_parts();
        stage.finding = parts.finding.clone();
        // 6. The record, admitted only if it belongs and its fingerprint matches.
        if let Some(record) = &parts.record {
            if !record.belongs_to(&stage.binding) {
                self.association.push(AssociationFailure { slot: Some(slot), kind: AssociationKind::ForeignRecord });
            } else if !record.fingerprint_matches(parts.inputs.as_ref()) {
                self.association.push(AssociationFailure { slot: Some(slot), kind: AssociationKind::FingerprintMismatch });
            } else {
                stage.record = Some(record.clone());
            }
        }
        // 7. Stage B's inputs fingerprint, before the inputs are handed back.
        if stage.binding.key().stage == BoundStage::ConfirmationStageB {
            if let Some(inputs) = &parts.inputs {
                self.b_inputs_digest = Some(open_pipe_stress_frame_kernel::structural::inputs_digest(inputs));
            }
        }
        Ok(parts)
    }

    /// Premise coverage only (carrier revision 4 §2): the plan is Known,
    /// non-empty, legal and unique; each slot holds an admitted verified
    /// record whose matrix digest is the attempt's; no association failure of
    /// any kind. It does not establish that every evaluation ran to
    /// completion, and never constructs or upgrades a `Confirmation`.
    pub fn verifies_all_required_premises(&self) -> bool {
        let RequiredPlan::Known(stages) = &self.plan else {
            return false;
        };
        let keys: Vec<StageKey> = stages.iter().map(|s| s.key().clone()).collect();
        !stages.is_empty()
            && plan_is_legal(&keys)
            && self.association.is_empty()
            && self.matrix_digest.is_some()
            && stages.iter().all(|s| {
                s.record.as_ref().is_some_and(|r| r.is_verified() && Some(r.matrix_digest()) == self.matrix_digest)
            })
    }

    /// Diagnostic consistency within the ownership path, never an identity:
    /// `current` has the fingerprint taken from Stage B's inputs at collection.
    pub fn binds_stage_b(&self, current: &MemberBoundInputs) -> bool {
        self.b_inputs_digest == Some(open_pipe_stress_frame_kernel::structural::inputs_digest(current))
    }
}

/// Legal keys: optionally A(None) then Influence(Some(id))… with unique ids,
/// then exactly one B(None) last; A and influence appear together or not at all.
fn plan_is_legal(keys: &[StageKey]) -> bool {
    let Some((last, rest)) = keys.split_last() else {
        return false;
    };
    if *last != StageKey::new(BoundStage::ConfirmationStageB, None) {
        return false;
    }
    if rest.is_empty() {
        return true;
    }
    if rest[0] != StageKey::new(BoundStage::ConfirmationStageA, None) || rest.len() < 2 {
        return false;
    }
    let influences = &rest[1..];
    let mut ids = Vec::new();
    for key in influences {
        match (&key.stage, &key.subject) {
            (BoundStage::ConfirmationInfluence, Some(id)) if !ids.contains(&id) => ids.push(id),
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
#[path = "stage_evidence_tests.rs"]
mod tests;
