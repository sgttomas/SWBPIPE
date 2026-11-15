//! The inverse-norm evidence carrier's kernel types (DEL-04-01; the carrier
//! design, revision 4, §1).
//!
//! A record states one fact: whether the inverse-norm premise behind the
//! formation check's Λ·σ bound was verified for its evaluation's matrix and
//! Q. Records have no public constructor. In this milestone no production
//! path mints one (no verifier exists yet), so every evaluation reports
//! `VerificationProgress::VerifierNotRun`; the certificate brings minting.
//! The test-only feature `test-synthetic-records` lets a consumer's tests
//! build records to exercise the carriage; no product build enables it.

use super::{FormationCheck, MemberBoundInputs};
use std::sync::Arc;

/// Which evaluation a record belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BoundStage {
    FormationCheck,
    ConfirmationStageA,
    ConfirmationInfluence,
    ConfirmationStageB,
}

/// A stage and its subject (the coupled support's id for an influence evaluation).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StageKey {
    pub stage: BoundStage,
    pub subject: Option<String>,
}

impl StageKey {
    pub fn new(stage: BoundStage, subject: Option<String>) -> Self {
        Self { stage, subject }
    }
}

/// The expected evaluation's identity, created by the confirmation plan before
/// the evaluation runs. A private `Arc<()>` compared by `Arc::ptr_eq`: no
/// global, no counter, no lookup. Never serialized.
#[derive(Debug, Clone)]
pub struct EvaluationBinding {
    key: StageKey,
    identity: Arc<()>,
}

impl EvaluationBinding {
    /// A fresh identity. Public: it creates no verification.
    pub fn for_stage(key: StageKey) -> Self {
        Self { key, identity: Arc::new(()) }
    }
    pub fn key(&self) -> &StageKey {
        &self.key
    }
    pub fn same_as(&self, other: &EvaluationBinding) -> bool {
        Arc::ptr_eq(&self.identity, &other.identity)
    }
}

/// The two Lemma-based methods (the report lane's agreed vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundMethod {
    LemmaEShiftedFactorization,
    LemmaDComparisonMatrix,
}

/// Lemma E's shift schedule T.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShiftStep {
    Sigma1,
    Sigma2,
}

/// Why a block was not verified (the report lane's agreed vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NotVerifiedReason {
    LemmaENonpositivePivot,
    LemmaESigmaNonpositive,
    LemmaDNonpositivePivot,
    LemmaDContractionNotBelowOne,
    ArithmeticNonfinite,
    BoundExceedsExistingQuantity,
}

/// A verified record's summary. Private fields; getters only.
#[derive(Debug, Clone, PartialEq)]
pub struct VerifiedSummary {
    bound_bits: u64,
    block_count: u32,
    largest_block: u32,
    blocks_e_sigma1: u32,
    blocks_e_sigma2: u32,
    blocks_d: u32,
}

impl VerifiedSummary {
    pub fn bound_bits(&self) -> u64 {
        self.bound_bits
    }
    pub fn block_count(&self) -> u32 {
        self.block_count
    }
    pub fn largest_block(&self) -> u32 {
        self.largest_block
    }
    /// Fixed order: E at σ₁, E at σ₂, D.
    pub fn blocks_by_method(&self) -> [(BoundMethod, Option<ShiftStep>, u32); 3] {
        [
            (BoundMethod::LemmaEShiftedFactorization, Some(ShiftStep::Sigma1), self.blocks_e_sigma1),
            (BoundMethod::LemmaEShiftedFactorization, Some(ShiftStep::Sigma2), self.blocks_e_sigma2),
            (BoundMethod::LemmaDComparisonMatrix, None, self.blocks_d),
        ]
    }
}

/// One method's attempt on the blocks it did not verify: reasons in enum order.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodAttempt {
    method: BoundMethod,
    reasons: Vec<(NotVerifiedReason, u32)>,
}

impl MethodAttempt {
    pub fn method(&self) -> BoundMethod {
        self.method
    }
    pub fn reasons(&self) -> &[(NotVerifiedReason, u32)] {
        &self.reasons
    }
}

/// A not-verified record's summary. Private fields; getters only.
#[derive(Debug, Clone, PartialEq)]
pub struct NotVerifiedSummary {
    block_count: u32,
    largest_block: u32,
    blocks_verified: u32,
    attempts: Vec<MethodAttempt>,
}

impl NotVerifiedSummary {
    pub fn block_count(&self) -> u32 {
        self.block_count
    }
    pub fn largest_block(&self) -> u32 {
        self.largest_block
    }
    pub fn blocks_verified(&self) -> u32 {
        self.blocks_verified
    }
    /// In `BoundMethod` order.
    pub fn attempts(&self) -> &[MethodAttempt] {
        &self.attempts
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InverseNormOutcome {
    Verified(VerifiedSummary),
    NotVerified(NotVerifiedSummary),
}

/// One evaluation's inverse-norm verification. Fields private; no public
/// constructor, `Default`, `From`/`TryFrom` or `Deserialize`. A clone keeps its
/// key and binding.
#[derive(Debug, Clone)]
pub struct InverseNormRecord {
    key: StageKey,
    binding: Option<EvaluationBinding>,
    matrix_digest: u64,
    est_bits: u64,
    estimate_limit_bits: u64,
    inputs_digest: Option<u64>,
    outcome: InverseNormOutcome,
}

/// Equality over the deterministic fields only; the binding is excluded.
/// Equality is not `belongs_to`.
impl PartialEq for InverseNormRecord {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.matrix_digest == other.matrix_digest
            && self.est_bits == other.est_bits
            && self.estimate_limit_bits == other.estimate_limit_bits
            && self.inputs_digest == other.inputs_digest
            && self.outcome == other.outcome
    }
}

impl InverseNormRecord {
    pub fn key(&self) -> &StageKey {
        &self.key
    }
    pub fn matrix_digest(&self) -> u64 {
        self.matrix_digest
    }
    pub fn est_bits(&self) -> u64 {
        self.est_bits
    }
    pub fn estimate_limit_bits(&self) -> u64 {
        self.estimate_limit_bits
    }
    pub fn inputs_digest(&self) -> Option<u64> {
        self.inputs_digest
    }
    pub fn outcome(&self) -> &InverseNormOutcome {
        &self.outcome
    }
    pub fn is_verified(&self) -> bool {
        matches!(self.outcome, InverseNormOutcome::Verified(_))
    }
    /// Exact, in memory: this record was minted under `expected`.
    pub fn belongs_to(&self, expected: &EvaluationBinding) -> bool {
        self.binding.as_ref().is_some_and(|b| b.same_as(expected))
    }
    /// Diagnostic only, never an identity: a verified record's inputs digest
    /// equals the digest of `inputs`. A not-verified record has no inputs and
    /// matches only `None`.
    pub fn fingerprint_matches(&self, inputs: Option<&MemberBoundInputs>) -> bool {
        match (self.inputs_digest, inputs) {
            (Some(digest), Some(inputs)) => digest == inputs_digest(inputs),
            (None, None) => true,
            _ => false,
        }
    }
}

/// Test-only construction (carrier revision 4 §1.1). Enabled only from the
/// dev-dependencies of `nonlinear_integration` and `product_physics`; held by
/// product_physics `tests/test_only_feature_guard.rs` (dependency, forward, default).
#[cfg(any(test, feature = "test-synthetic-records"))]
pub mod synthetic {
    use super::*;

    /// A synthetic verified summary with `blocks` blocks, all verified by E at σ₁.
    pub fn verified(bound_bits: u64, blocks: u32) -> InverseNormOutcome {
        InverseNormOutcome::Verified(VerifiedSummary {
            bound_bits,
            block_count: blocks,
            largest_block: 1,
            blocks_e_sigma1: blocks,
            blocks_e_sigma2: 0,
            blocks_d: 0,
        })
    }

    /// A synthetic not-verified summary: no block verified; each method failed
    /// every block for `reason`.
    pub fn not_verified(blocks: u32, reason: NotVerifiedReason) -> InverseNormOutcome {
        InverseNormOutcome::NotVerified(NotVerifiedSummary {
            block_count: blocks,
            largest_block: 1,
            blocks_verified: 0,
            attempts: vec![
                MethodAttempt { method: BoundMethod::LemmaEShiftedFactorization, reasons: vec![(reason, blocks)] },
                MethodAttempt { method: BoundMethod::LemmaDComparisonMatrix, reasons: vec![(reason, blocks)] },
            ],
        })
    }

    /// A record for `key`, minted as if under `binding`. A verified record's
    /// inputs digest is taken from `inputs`; a not-verified record has none.
    pub fn record(
        key: StageKey,
        binding: Option<&EvaluationBinding>,
        outcome: InverseNormOutcome,
        matrix_digest: u64,
        est: f64,
        inputs: Option<&MemberBoundInputs>,
    ) -> InverseNormRecord {
        let verified = matches!(outcome, InverseNormOutcome::Verified(_));
        InverseNormRecord {
            key,
            binding: binding.cloned(),
            matrix_digest,
            est_bits: est.to_bits(),
            estimate_limit_bits: (1024.0 * est).to_bits(),
            inputs_digest: if verified { inputs.map(inputs_digest) } else { None },
            outcome,
        }
    }
}

/// How far one formation-check evaluation got toward the inverse-norm
/// verifier (carrier revision 4 §1.3). Read from the evaluation itself, never
/// from whether a record is present.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationProgress {
    /// An earlier formation step returned a finding; the evaluation's finding says which.
    StoppedBeforeVerifier,
    /// Milestone 1 only: this build contains no inverse-norm verifier, so
    /// verification was not attempted. The certificate removes this value.
    VerifierNotRun,
    /// The verifier ran; the evaluation's record holds Verified or NotVerified.
    VerifierReached,
}

/// One formation-check evaluation's result, with the binding it was requested
/// under. On the confirmation path every returned evaluation carries its
/// binding, including one that stopped before the verifier and so has no
/// record (carrier revision 4 §1.3, R3-F1).
#[derive(Debug, Clone)]
pub struct FormationEvaluation {
    binding: Option<EvaluationBinding>,
    finding: Option<FormationCheck>,
    inputs: Option<MemberBoundInputs>,
    record: Option<InverseNormRecord>,
    progress: VerificationProgress,
}

/// `FormationEvaluation`'s parts, moved out whole.
#[derive(Debug, Clone)]
pub struct FormationParts {
    pub binding: Option<EvaluationBinding>,
    pub finding: Option<FormationCheck>,
    pub inputs: Option<MemberBoundInputs>,
    pub record: Option<InverseNormRecord>,
    pub progress: VerificationProgress,
}

impl FormationEvaluation {
    /// The evaluation of a `check_at` result under `binding`. No verifier exists
    /// in this build, so no record is minted: an evaluation that formed its
    /// inputs reports `VerifierNotRun`, and one that returned a finding
    /// reports `StoppedBeforeVerifier`.
    pub(super) fn from_check(
        binding: Option<EvaluationBinding>,
        (finding, inputs): (Option<FormationCheck>, Option<MemberBoundInputs>),
    ) -> Self {
        let progress = if finding.is_none() && inputs.is_some() {
            VerificationProgress::VerifierNotRun
        } else {
            VerificationProgress::StoppedBeforeVerifier
        };
        Self { binding, finding, inputs, record: None, progress }
    }
    pub fn binding(&self) -> Option<&EvaluationBinding> {
        self.binding.as_ref()
    }
    /// The binding is present and is `expected`.
    pub fn is_bound_to(&self, expected: &EvaluationBinding) -> bool {
        self.binding.as_ref().is_some_and(|b| b.same_as(expected))
    }
    pub fn finding(&self) -> Option<&FormationCheck> {
        self.finding.as_ref()
    }
    pub fn inputs(&self) -> Option<&MemberBoundInputs> {
        self.inputs.as_ref()
    }
    pub fn record(&self) -> Option<&InverseNormRecord> {
        self.record.as_ref()
    }
    pub fn progress(&self) -> VerificationProgress {
        self.progress
    }
    pub fn into_parts(self) -> FormationParts {
        FormationParts {
            binding: self.binding,
            finding: self.finding,
            inputs: self.inputs,
            record: self.record,
            progress: self.progress,
        }
    }
}

/// Test-only: an evaluation with chosen parts, so a consumer's tests can offer
/// foreign, record-free and record-bearing evaluations to a collection.
#[cfg(any(test, feature = "test-synthetic-records"))]
impl FormationEvaluation {
    pub fn synthetic(
        binding: Option<EvaluationBinding>,
        finding: Option<FormationCheck>,
        inputs: Option<MemberBoundInputs>,
        record: Option<InverseNormRecord>,
        progress: VerificationProgress,
    ) -> Self {
        Self { binding, finding, inputs, record, progress }
    }
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a-64 over a byte stream (the carrier's digest streams v1, §1.5).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Fnv64(u64);

impl Fnv64 {
    pub(crate) fn new() -> Self {
        Self(FNV_OFFSET)
    }
    pub(crate) fn bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(FNV_PRIME);
        }
    }
    pub(crate) fn finish(self) -> u64 {
        self.0
    }
}

/// The matrix stream: `SWB-IN-v1`, n (u64 LE), n exponents (i32 LE), then for
/// each row r in original free-position order the logical entries c in 0..=r
/// as f64 bits (u64 LE), implicit zeros as +0.0. `row(r, emit)` must emit the
/// row's stored entries with c <= r in ascending c; the encoder fills the gaps.
pub(crate) fn matrix_digest_rows(
    exponents: &[i32],
    mut row: impl FnMut(usize, &mut dyn FnMut(usize, f64)),
) -> u64 {
    let n = exponents.len();
    let mut h = Fnv64::new();
    h.bytes(b"SWB-IN-v1");
    h.bytes(&(n as u64).to_le_bytes());
    for e in exponents {
        h.bytes(&e.to_le_bytes());
    }
    let zero = 0.0f64.to_bits().to_le_bytes();
    for r in 0..n {
        let mut next = 0usize;
        row(r, &mut |c, v| {
            debug_assert!(c >= next && c <= r, "matrix digest: entries in ascending column order within the lower triangle");
            while next < c {
                h.bytes(&zero);
                next += 1;
            }
            h.bytes(&v.to_bits().to_le_bytes());
            next = c + 1;
        });
        while next <= r {
            h.bytes(&zero);
            next += 1;
        }
    }
    h.finish()
}

/// The inputs stream: `SWB-MBI-v1`; then tag 0x01 w, 0x02 w2, 0x03 exponents,
/// 0x04 lambda_sigma, 0x05 lift_radius, each tag immediately before its
/// payload. A vector is a u64 LE element count then its elements (f64 as
/// `to_bits().to_le_bytes()`, exponents as i32 LE); `lambda_sigma` is exactly
/// `lambda_sigma.to_bits().to_le_bytes()`, with no count.
pub fn inputs_digest(inputs: &MemberBoundInputs) -> u64 {
    fn floats(h: &mut Fnv64, tag: u8, values: &[f64]) {
        h.bytes(&[tag]);
        h.bytes(&(values.len() as u64).to_le_bytes());
        for v in values {
            h.bytes(&v.to_bits().to_le_bytes());
        }
    }
    let mut h = Fnv64::new();
    h.bytes(b"SWB-MBI-v1");
    floats(&mut h, 0x01, &inputs.w);
    floats(&mut h, 0x02, &inputs.w2);
    h.bytes(&[0x03]);
    h.bytes(&(inputs.exponents.len() as u64).to_le_bytes());
    for e in &inputs.exponents {
        h.bytes(&e.to_le_bytes());
    }
    h.bytes(&[0x04]);
    h.bytes(&inputs.lambda_sigma.to_bits().to_le_bytes());
    floats(&mut h, 0x05, &inputs.lift_radius);
    h.finish()
}

#[cfg(test)]
#[path = "inverse_bound_tests.rs"]
mod tests;
