//! The pair owner of each case's integrity diagnostic and its inverse-norm
//! evidence (DEL-04-01; carrier revision 4 §3).
//!
//! The invocation owns `diagnostics` and one `InverseNormSink`, passed beside
//! each other. The sink's fields are private to this module, and its only
//! writers are `publish`, `amend` and `clear`. Each entry keeps a 64-bit
//! mirror of its integrity diagnostic as the owner last left it. `reconcile`
//! runs where a diagnostic vector becomes, or is copied into, a document, and
//! compares final contents, so a write that bypassed the owner, whatever its
//! spelling, is detected there. A disagreement withdraws that case's evidence
//! (no record is projected; a typed withdrawal is shown) and never changes a
//! published byte. Limits: detection happens only at checkpoints, and a bypass
//! that leaves the diagnostic's bytes identical is not detected (it publishes
//! nothing false). Under `cfg(test)` a withdrawal also panics unless a test
//! allows it, so every product test that assembles a document guards the pair.

use crate::inverse_norm_evidence::{
    self, CaseEvidence, HeldComparisonEvidence, HeldOutcomeView, IntegrityFailureCode, PriorVerdict, ConfirmationView, EstablishmentView,
    InverseNormEvidence, PublishedBasisView, VerdictMirror, WithdrawalView,
};
use crate::Diagnostic;
use open_pipe_stress_frame_kernel::structural::InverseNormRecord;
use open_pipe_stress_nonlinear_integration::confirmation::StageEvidence;

/// The integrity diagnostic id prefix (`integrity_diagnostic_id`).
const INTEGRITY_PREFIX: &str = "diagnostic:numerical-integrity:";

/// Which check supplied the published rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedBasis {
    None,
    Linear,
    ConfirmedStageB,
}

/// Why `reconcile` withdrew an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Withdrawal {
    /// An integrity diagnostic had no entry.
    MissingEntry,
    /// An entry had no integrity diagnostic.
    OrphanEntry,
    /// A case had more than one integrity diagnostic.
    DuplicateDiagnostic,
    /// The diagnostic is not what the owner last left.
    MirrorMismatch,
    /// The diagnostic's code is outside the closed integrity codes.
    UnknownCode,
}

/// A confirmation's evidence with its establishment.
#[derive(Debug, Clone)]
pub(crate) struct ConfirmationEntry {
    pub evidence: StageEvidence,
    pub established: Option<String>, // None: established; Some(reason): not established
}

#[derive(Debug, Clone)]
struct CaseEntry {
    case_id: String,
    linear: Option<InverseNormRecord>,
    confirmation: Option<ConfirmationEntry>,
    published_basis: PublishedBasis,
    mirror: u64,
    withdrawn: Option<Withdrawal>,
}

/// The invocation's evidence, one entry per case with an integrity diagnostic,
/// in first-write order (the case loop's). Private fields.
#[derive(Debug, Clone, Default)]
pub struct InverseNormSink {
    entries: Vec<CaseEntry>,
    held: Vec<HeldEntry>,
}

/// A held comparison's evidence (carrier revision 4 §6): the held solve's own
/// confirmation, never a case entry. Written once per comparison, in model
/// combination order, by `record_held`.
#[derive(Debug, Clone)]
struct HeldEntry {
    comparison_id: String,
    minuend_id: String,
    target_id: String,
    outcome: HeldOutcomeView,
    confirmation: Option<ConfirmationEntry>,
}

/// What an operation changes in the case's entry, beside the diagnostic.
#[derive(Debug, Clone)]
pub(crate) enum EntryUpdate {
    None,
    Linear(Option<InverseNormRecord>),
    Basis(PublishedBasis),
    ConfirmationAndBasis(ConfirmationEntry, PublishedBasis),
}

impl InverseNormSink {
    fn entry_mut(&mut self, case_id: &str) -> &mut CaseEntry {
        if let Some(index) = self.entries.iter().position(|e| e.case_id == case_id) {
            return &mut self.entries[index];
        }
        self.entries.push(CaseEntry {
            case_id: case_id.to_string(),
            linear: None,
            confirmation: None,
            published_basis: PublishedBasis::None,
            mirror: 0,
            withdrawn: None,
        });
        self.entries.last_mut().expect("just pushed")
    }

    /// The deterministic projection (carrier revision 4 §5): exactly one
    /// `CaseEvidence` per published case, in `cases` order (the document's
    /// `numerical_quality.cases`), including cases with no entry.
    pub(crate) fn projection(&self, diagnostics: &[Diagnostic], cases: &[&str]) -> InverseNormEvidence {
        let mut out = InverseNormEvidence::empty();
        let empty = |case_id: &str| CaseEntry {
            case_id: case_id.to_string(),
            linear: None,
            confirmation: None,
            published_basis: PublishedBasis::None,
            mirror: 0,
            withdrawn: None,
        };
        for &case_id in cases {
            let owned;
            let entry = match self.entries.iter().find(|e| e.case_id == case_id) {
                Some(entry) => entry,
                None => {
                    owned = empty(case_id);
                    &owned
                }
            };
            let verdict = find(diagnostics, case_id).map_or(VerdictMirror::NotAssessed, |d| verdict_of(&d.code, &d.message));
            let withdrawn = entry.withdrawn.map(|w| match w {
                Withdrawal::MissingEntry => WithdrawalView::MissingEntry,
                Withdrawal::OrphanEntry => WithdrawalView::OrphanEntry,
                Withdrawal::DuplicateDiagnostic => WithdrawalView::DuplicateDiagnostic,
                Withdrawal::MirrorMismatch => WithdrawalView::MirrorMismatch,
                Withdrawal::UnknownCode => WithdrawalView::UnknownCode,
            });
            let published_basis = match entry.published_basis {
                PublishedBasis::None => PublishedBasisView::None,
                PublishedBasis::Linear => PublishedBasisView::Linear,
                PublishedBasis::ConfirmedStageB => PublishedBasisView::ConfirmedStageB,
            };
            let mut required = Vec::new();
            match entry.published_basis {
                PublishedBasis::Linear => required.push(inverse_norm_evidence::linear_requirement()),
                PublishedBasis::ConfirmedStageB => {
                    if let Some(requirement) = entry
                        .confirmation
                        .as_ref()
                        .and_then(|c| inverse_norm_evidence::confirmation_requirement(&c.evidence))
                    {
                        required.push(requirement);
                    }
                }
                PublishedBasis::None => {}
            }
            let withheld = withdrawn.is_some();
            out.cases.push(CaseEvidence {
                case_id: entry.case_id.clone(),
                verdict,
                published_basis,
                required,
                linear: if withheld { None } else { entry.linear.as_ref().map(inverse_norm_evidence::record_view) },
                confirmation: if withheld {
                    None
                } else {
                    entry.confirmation.as_ref().map(confirmation_of)
                },
                withdrawn,
            });
        }
        for held in &self.held {
            out.held.push(HeldComparisonEvidence {
                comparison_id: held.comparison_id.clone(),
                minuend_id: held.minuend_id.clone(),
                target_id: held.target_id.clone(),
                outcome: held.outcome.clone(),
                confirmation: held.confirmation.as_ref().map(confirmation_of),
            });
        }
        out
    }
}

pub(crate) fn confirmation_of(entry: &ConfirmationEntry) -> ConfirmationView {
    let established = match &entry.established {
        None => EstablishmentView::Established,
        Some(reason) => EstablishmentView::NotEstablished { reason: reason.clone() },
    };
    inverse_norm_evidence::confirmation_view(&entry.evidence, established)
}

/// The verdict of a closed integrity code. An unknown code is withdrawn at
/// the checkpoint, so its projection never carries evidence; it reads as
/// unresolved here only so that the projection stays total.
fn verdict_of(code: &str, message: &str) -> VerdictMirror {
    if code == INSPECTION_ONLY {
        return match inspection_prior(message) {
            Some(prior) => VerdictMirror::InspectionOnly { prior },
            // Withdrawn at the checkpoint (UnknownCode); total here only.
            None => VerdictMirror::Failed { code: IntegrityFailureCode::Unresolved },
        };
    }
    match code {
        "NUMERICAL_INTEGRITY_CHECKS_PASSED" => VerdictMirror::ChecksPassed,
        "NUMERICAL_INTEGRITY_SENSITIVE" => VerdictMirror::Sensitive,
        "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED" => VerdictMirror::RecoveryBasisUnqualified,
        other => VerdictMirror::Failed {
            code: IntegrityFailureCode::of(other).unwrap_or(IntegrityFailureCode::Unresolved),
        },
    }
}

/// The inspection-only code (the exact-route lane's blocked-document repair).
const INSPECTION_ONLY: &str = "NUMERICAL_INTEGRITY_INSPECTION_ONLY";

/// An inspection-only diagnostic's prior outcome: the closed token at the start
/// of its message, `Inspection only (prior_outcome=<code>)`, where <code> is
/// exactly checks-passed or sensitive. Anything else is `None`. (If the prior
/// moves to a diagnostic field, only this parse moves.)
fn inspection_prior(message: &str) -> Option<PriorVerdict> {
    let rest = message.strip_prefix("Inspection only (prior_outcome=")?;
    match &rest[..rest.find(')')?] {
        "NUMERICAL_INTEGRITY_CHECKS_PASSED" => Some(PriorVerdict::ChecksPassed),
        "NUMERICAL_INTEGRITY_SENSITIVE" => Some(PriorVerdict::Sensitive),
        _ => None,
    }
}

fn is_closed_diagnostic(diagnostic: &Diagnostic) -> bool {
    if diagnostic.code == INSPECTION_ONLY {
        return inspection_prior(&diagnostic.message).is_some();
    }
    is_closed_code(&diagnostic.code)
}

fn is_closed_code(code: &str) -> bool {
    matches!(
        code,
        "NUMERICAL_INTEGRITY_CHECKS_PASSED" | "NUMERICAL_INTEGRITY_SENSITIVE" | "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"
    ) || IntegrityFailureCode::of(code).is_some()
}

/// The case id of an integrity diagnostic, by its id.
fn case_of(diagnostic: &Diagnostic) -> Option<&str> {
    diagnostic.id.strip_prefix(INTEGRITY_PREFIX)
}

/// FNV-1a-64 of the diagnostic's content: `SWB-IDM-v1`, then id, code,
/// severity, message, source and each affected ref, each as a u64 LE length
/// and its UTF-8 bytes (an absent source is length u64::MAX).
fn mirror_of(diagnostic: &Diagnostic) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut bytes = |b: &[u8]| {
        for &x in b {
            h ^= u64::from(x);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    bytes(b"SWB-IDM-v1");
    for field in [&diagnostic.id, &diagnostic.code, &diagnostic.severity, &diagnostic.message] {
        bytes(&(field.len() as u64).to_le_bytes());
        bytes(field.as_bytes());
    }
    match &diagnostic.source {
        Some(source) => {
            bytes(&(source.len() as u64).to_le_bytes());
            bytes(source.as_bytes());
        }
        None => bytes(&u64::MAX.to_le_bytes()),
    }
    bytes(&(diagnostic.affected_refs.len() as u64).to_le_bytes());
    for r in &diagnostic.affected_refs {
        bytes(&(r.len() as u64).to_le_bytes());
        bytes(r.as_bytes());
    }
    h
}

fn apply(entry: &mut CaseEntry, update: EntryUpdate) {
    match update {
        EntryUpdate::None => {}
        EntryUpdate::Linear(record) => entry.linear = record,
        EntryUpdate::Basis(basis) => entry.published_basis = basis,
        EntryUpdate::ConfirmationAndBasis(confirmation, basis) => {
            entry.confirmation = Some(confirmation);
            entry.published_basis = basis;
        }
    }
}

/// The case's integrity diagnostic, read only.
pub(crate) fn find<'d>(diagnostics: &'d [Diagnostic], case_id: &str) -> Option<&'d Diagnostic> {
    diagnostics.iter().find(|d| case_of(d) == Some(case_id))
}

/// Writes a case's integrity diagnostic: `diagnostic` is the final diagnostic,
/// built (with any internal amendment) before either side is changed. Pushes
/// it, then records the entry update and the mirror.
pub(crate) fn publish(
    diagnostics: &mut Vec<Diagnostic>,
    sink: &mut InverseNormSink,
    case_id: &str,
    diagnostic: Diagnostic,
    update: EntryUpdate,
) {
    debug_assert_eq!(case_of(&diagnostic), Some(case_id), "publish: an integrity diagnostic of this case");
    let mirror = mirror_of(&diagnostic);
    diagnostics.push(diagnostic);
    let entry = sink.entry_mut(case_id);
    apply(entry, update);
    entry.mirror = mirror;
}

/// Amends a case's integrity diagnostic. `edit` changes the diagnostic and
/// returns whether it changed anything; on a no-op (`false`) neither the
/// diagnostic's mirror nor the entry changes, and `amend` returns `false`.
/// With no integrity diagnostic for the case, nothing changes.
pub(crate) fn amend(
    diagnostics: &mut [Diagnostic],
    sink: &mut InverseNormSink,
    case_id: &str,
    edit: impl FnOnce(&mut Diagnostic) -> bool,
    update: EntryUpdate,
) -> bool {
    let Some(diagnostic) = diagnostics.iter_mut().find(|d| case_of(d) == Some(case_id)) else {
        return false;
    };
    if !edit(diagnostic) {
        return false;
    }
    let mirror = mirror_of(diagnostic);
    let entry = sink.entry_mut(case_id);
    apply(entry, update);
    entry.mirror = mirror;
    true
}

/// Records an entry update that changes no diagnostic (the evidence of a case
/// whose diagnostic is already written). The mirror is unchanged.
pub(crate) fn update_entry(sink: &mut InverseNormSink, case_id: &str, update: EntryUpdate) {
    if let Some(entry) = sink.entries.iter_mut().find(|e| e.case_id == case_id) {
        apply(entry, update);
    }
}

/// Drops every diagnostic and every entry together (`fail_closed_form`).
pub(crate) fn clear(diagnostics: &mut Vec<Diagnostic>, sink: &mut InverseNormSink) {
    diagnostics.clear();
    sink.entries.clear();
    sink.held.clear();
}

/// Edits every integrity diagnostic in place (`blocked_envelope`'s
/// inspection-only conversion). An entry's mirror follows the edit only where
/// it agreed with the diagnostic before it, as the first diagnostic of its
/// case; any other disagreement is left for `reconcile` to withdraw, so the
/// conversion launders no bypass.
pub(crate) fn amend_each(diagnostics: &mut [Diagnostic], sink: &mut InverseNormSink, mut edit: impl FnMut(&mut Diagnostic)) {
    let mut seen: Vec<String> = Vec::new();
    for diagnostic in diagnostics.iter_mut() {
        let Some(case_id) = case_of(diagnostic).map(str::to_string) else { continue };
        let before = mirror_of(diagnostic);
        let agreed = !seen.contains(&case_id) && sink.entries.iter().any(|e| e.case_id == case_id && e.mirror == before);
        seen.push(case_id.clone());
        edit(diagnostic);
        if agreed {
            if let Some(entry) = sink.entries.iter_mut().find(|e| e.case_id == case_id) {
                entry.mirror = mirror_of(diagnostic);
            }
        }
    }
}

/// The position of a case's integrity diagnostic, read only (the stress-free
/// notice is placed after it and moves no mirror).
pub(crate) fn position(diagnostics: &[Diagnostic], case_id: &str) -> Option<usize> {
    diagnostics.iter().position(|d| case_of(d) == Some(case_id))
}

/// Records one held comparison's evidence (carrier revision 4 §6).
pub(crate) fn record_held(
    sink: &mut InverseNormSink,
    comparison_id: &str,
    minuend_id: &str,
    target_id: &str,
    outcome: HeldOutcomeView,
    confirmation: Option<ConfirmationEntry>,
) {
    sink.held.push(HeldEntry {
        comparison_id: comparison_id.to_string(),
        minuend_id: minuend_id.to_string(),
        target_id: target_id.to_string(),
        outcome,
        confirmation,
    });
}

/// The checkpoint: compares every integrity diagnostic with its entry and
/// withdraws each disagreeing entry's evidence. Never changes `diagnostics`.
pub(crate) fn reconcile(diagnostics: &[Diagnostic], sink: &mut InverseNormSink) {
    let mut seen: Vec<&str> = Vec::new();
    let mut withdrawn_any = false;
    for diagnostic in diagnostics {
        let Some(case_id) = case_of(diagnostic) else { continue };
        let duplicate = seen.contains(&case_id);
        seen.push(case_id);
        let present = sink.entries.iter().any(|e| e.case_id == case_id);
        let entry = sink.entry_mut(case_id);
        let reason = if duplicate {
            Some(Withdrawal::DuplicateDiagnostic)
        } else if !present {
            Some(Withdrawal::MissingEntry)
        } else if entry.mirror != mirror_of(diagnostic) {
            Some(Withdrawal::MirrorMismatch)
        } else if !is_closed_diagnostic(diagnostic) {
            Some(Withdrawal::UnknownCode)
        } else {
            None
        };
        if let Some(reason) = reason {
            if entry.withdrawn.is_none() {
                entry.withdrawn = Some(reason);
            }
            withdrawn_any = true;
        }
    }
    for entry in &mut sink.entries {
        if !seen.contains(&entry.case_id.as_str()) {
            if entry.withdrawn.is_none() {
                entry.withdrawn = Some(Withdrawal::OrphanEntry);
            }
            withdrawn_any = true;
        }
    }
    #[cfg(test)]
    if withdrawn_any && !tests::ALLOW_WITHDRAWAL.with(|allow| allow.get()) {
        panic!("integrity pair disagreement at a checkpoint: {:?}", sink.entries.iter().map(|e| (&e.case_id, e.withdrawn)).collect::<Vec<_>>());
    }
    let _ = withdrawn_any;
}

#[cfg(test)]
#[path = "integrity_tests.rs"]
pub(crate) mod tests;

#[cfg(test)]
#[path = "integrity_wiring_tests.rs"]
mod wiring_tests;
