//! Test-only diagnostic counts of the dense modal kernel's screen (A3 item
//! K2, design §5.3.7): per solve, how many screened pairs were hard-cut,
//! attempted in the band, rejected (by their first failing condition) and
//! retained.
//!
//! The counts are a pure function of one solve's public result and its
//! `screen_error_estimate_threshold` τ_e (not on the solution); nothing is
//! global, and nothing in the kernel, its API or its output changes. Every
//! [`aggregate`] asserts that it reproduces the kernel's own identity: each
//! pair's `retained` flag and `hard_rejected` verdict, and the S1 counts
//! `screened_pair_count`, `hard_rejected_pair_count`,
//! `attempted_band_pair_count` and `below_threshold_mu_count`. A divergence
//! from the kernel's branch order therefore fails the using test.
//!
//! The outcome order is the kernel's (`dense.rs`, the screen loop):
//! `classify_mu` (μ ≤ h hard-cut, never recovered); then
//! `recover_band_mode` (`None`: recovery unavailable); then the retention
//! predicate `r.mu > hard_cut && screen_passes(r, τ_f, τ_e)`, whose
//! conjunction short-circuits left to right, η̃ finite, e_λ finite, η̃ ≥ 0,
//! e_λ ≥ 0, η̃ ≤ τ_f, e_λ ≤ τ_e. Each rejection is named by the first
//! conjunct that fails, tested in that order with the kernel's own
//! comparisons (so NaN falls where the kernel puts it); none is collapsed.
//!
//! Counts are u64 with checked addition; an overflow panics, never wraps.

#![allow(dead_code)]

use open_pipe_stress_modal_eigen::dense::{DenseEigenSolution, DenseOptions, ScreenedMode};

/// The first screen condition a screened pair fails, or `Retained`; in the
/// kernel's evaluation order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScreenOutcome {
    /// QL μ ≤ h (`classify_mu` → HardRejected): never recovered.
    HardCut,
    /// Band, and `recover_band_mode` gave `None`: a non-positive or
    /// non-finite exact energy or mass, a non-normal recovered μ or λ, or a
    /// failed `formed_pair_errors` premise.
    RecoveryUnavailable,
    /// Band, recovered, and `!(r.mu > h)`.
    RecoveredMuNotAboveHardCut,
    /// `!r.backward_error.is_finite()` (η̃).
    BackwardErrorNonFinite,
    /// `!r.error_estimate.is_finite()` (e_λ).
    ErrorEstimateNonFinite,
    /// `!(r.backward_error >= 0.0)`.
    BackwardErrorNegative,
    /// `!(r.error_estimate >= 0.0)`.
    ErrorEstimateNegative,
    /// `!(r.backward_error <= τ_f)`.
    BackwardErrorAboveTauF,
    /// `!(r.error_estimate <= τ_e)`.
    ErrorEstimateAboveTauE,
    /// Every condition holds: the kernel keeps the pair.
    Retained,
}

impl ScreenOutcome {
    pub const ALL: [ScreenOutcome; 10] = [
        ScreenOutcome::HardCut,
        ScreenOutcome::RecoveryUnavailable,
        ScreenOutcome::RecoveredMuNotAboveHardCut,
        ScreenOutcome::BackwardErrorNonFinite,
        ScreenOutcome::ErrorEstimateNonFinite,
        ScreenOutcome::BackwardErrorNegative,
        ScreenOutcome::ErrorEstimateNegative,
        ScreenOutcome::BackwardErrorAboveTauF,
        ScreenOutcome::ErrorEstimateAboveTauE,
        ScreenOutcome::Retained,
    ];

    fn index(self) -> usize {
        Self::ALL.iter().position(|&o| o == self).unwrap()
    }
}

/// Classifies one screened pair as the kernel's screen loop does, given h
/// (`hard_classification_threshold`), τ_f (`formed_backward_error_threshold`)
/// and τ_e. The comparisons are the kernel's, in its order; each is negated
/// as written, not rewritten, so NaN fails where the kernel's would.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn classify(mode: &ScreenedMode, hard_cut: f64, tau_f: f64, tau_e: f64) -> ScreenOutcome {
    use ScreenOutcome::*;
    if mode.hard_rejected {
        return HardCut;
    }
    let Some(r) = &mode.recovery else {
        return RecoveryUnavailable;
    };
    if !(r.mu > hard_cut) {
        RecoveredMuNotAboveHardCut
    } else if !r.backward_error.is_finite() {
        BackwardErrorNonFinite
    } else if !r.error_estimate.is_finite() {
        ErrorEstimateNonFinite
    } else if !(r.backward_error >= 0.0) {
        BackwardErrorNegative
    } else if !(r.error_estimate >= 0.0) {
        ErrorEstimateNegative
    } else if !(r.backward_error <= tau_f) {
        BackwardErrorAboveTauF
    } else if !(r.error_estimate <= tau_e) {
        ErrorEstimateAboveTauE
    } else {
        Retained
    }
}

/// Counts of one solve's screened pairs, by outcome; fresh per call.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScreenOutcomeCounts {
    by_outcome: [u64; ScreenOutcome::ALL.len()],
}

fn checked_sum(values: impl IntoIterator<Item = u64>) -> u64 {
    values.into_iter().fold(0u64, |total, v| {
        total
            .checked_add(v)
            .expect("screen outcome count overflow (never wrapped)")
    })
}

impl ScreenOutcomeCounts {
    /// Counts in [`ScreenOutcome::ALL`] order (for the overflow check).
    pub fn from_array(by_outcome: [u64; ScreenOutcome::ALL.len()]) -> Self {
        Self { by_outcome }
    }

    /// One more pair with `outcome`; panics on overflow, never wraps.
    pub fn record(&mut self, outcome: ScreenOutcome) {
        let count = &mut self.by_outcome[outcome.index()];
        *count = count
            .checked_add(1)
            .expect("screen outcome count overflow (never wrapped)");
    }

    pub fn of(&self, outcome: ScreenOutcome) -> u64 {
        self.by_outcome[outcome.index()]
    }

    /// Every screened pair (hard-cut plus attempted).
    pub fn screened(&self) -> u64 {
        checked_sum(self.by_outcome)
    }

    pub fn hard_cut(&self) -> u64 {
        self.of(ScreenOutcome::HardCut)
    }

    /// Band pairs: every outcome after the hard cut.
    pub fn attempted(&self) -> u64 {
        checked_sum(
            ScreenOutcome::ALL
                .iter()
                .filter(|&&o| o != ScreenOutcome::HardCut)
                .map(|&o| self.of(o)),
        )
    }

    /// Attempted band pairs the screen rejected (any first failure).
    pub fn band_rejected(&self) -> u64 {
        checked_sum(
            ScreenOutcome::ALL
                .iter()
                .filter(|&&o| o != ScreenOutcome::HardCut && o != ScreenOutcome::Retained)
                .map(|&o| self.of(o)),
        )
    }

    /// Every screened pair not kept: hard-cut plus band-rejected.
    pub fn rejected(&self) -> u64 {
        checked_sum([self.hard_cut(), self.band_rejected()])
    }

    pub fn retained(&self) -> u64 {
        self.of(ScreenOutcome::Retained)
    }

    /// The non-zero counts, in outcome order (a compact tripwire form).
    pub fn nonzero(&self) -> Vec<(ScreenOutcome, u64)> {
        ScreenOutcome::ALL
            .iter()
            .map(|&o| (o, self.of(o)))
            .filter(|&(_, c)| c > 0)
            .collect()
    }
}

/// Tallies screened pairs by [`classify`], with no identity check (for
/// constructed records, which no solve produced).
pub fn tally(modes: &[ScreenedMode], hard_cut: f64, tau_f: f64, tau_e: f64) -> ScreenOutcomeCounts {
    let mut counts = ScreenOutcomeCounts::default();
    for mode in modes {
        counts.record(classify(mode, hard_cut, tau_f, tau_e));
    }
    counts
}

/// The counts of one solve, asserting the kernel's identity. `options`
/// must be the ones the solve used (τ_e is not on the solution).
pub fn aggregate(solution: &DenseEigenSolution, options: &DenseOptions) -> ScreenOutcomeCounts {
    let hard_cut = solution.hard_classification_threshold;
    let tau_f = solution.formed_backward_error_threshold;
    let tau_e = options.screen_error_estimate_threshold;
    let counts = tally(&solution.screened, hard_cut, tau_f, tau_e);
    for (i, mode) in solution.screened.iter().enumerate() {
        assert_eq!(
            mode.hard_rejected,
            mode.mu <= hard_cut,
            "pair {i}: hard verdict"
        );
        assert!(
            mode.mu <= solution.classification_threshold,
            "pair {i}: screened μ above the broad threshold"
        );
        if mode.hard_rejected {
            assert!(mode.recovery.is_none(), "pair {i}: hard-cut pair recovered");
        }
        let outcome = classify(mode, hard_cut, tau_f, tau_e);
        assert_eq!(
            outcome == ScreenOutcome::Retained,
            mode.retained,
            "pair {i}: {outcome:?} disagrees with the kernel's retained flag"
        );
    }
    let screened = u64::try_from(solution.screened.len()).unwrap();
    assert_eq!(counts.screened(), screened);
    assert_eq!(counts.screened(), solution.screened_pair_count);
    assert_eq!(counts.hard_cut(), solution.hard_rejected_pair_count);
    assert_eq!(counts.attempted(), solution.attempted_band_pair_count);
    assert_eq!(
        counts.rejected(),
        u64::try_from(solution.below_threshold_mu_count).unwrap()
    );
    assert_eq!(
        checked_sum([counts.rejected(), counts.retained()]),
        counts.screened()
    );
    counts
}
