//! Per-stage workspaces (Agent 0's option (c)): the exact
//! stages reuse one set of accumulators and scratch vectors for every pair
//! or product instead of allocating them per call.
//!
//! The callers evaluate one pair or one mode per call through a shared
//! reference (`residual_estimate(&ReformedPencil, ..)`,
//! `normalize_mode(.., &ModalBasis)`), so the workspace is owned by that
//! value, behind a [`Mutex`], and lives as long as it does: its capacity is
//! retained from the first call until the owner is dropped, and the
//! resource ledger counts it as live over that span. It is not part of the
//! owner's value: a clone starts with an empty workspace, equality ignores
//! it, and the owner's `Debug` omits it.
//!
//! **Reset.** Every accumulator taken from a workspace is
//! [`ExactSum::reset`] before use (limbs, `low`, pending adds and the
//! non-finite part), and every scratch vector is cleared, so a reused
//! workspace gives exactly the fresh result whatever an earlier call (an
//! early refusal included) left in it. A poisoned lock (a panic during an
//! earlier call) is taken over for the same reason.

use std::sync::{Mutex, PoisonError};

use crate::exact::{ExactSum, Term};

/// A workspace held by its owner (module docs).
pub(crate) struct Retained<T>(Mutex<T>);

impl<T: Default> Default for Retained<T> {
    fn default() -> Self {
        Self(Mutex::new(T::default()))
    }
}

impl<T: Default> Clone for Retained<T> {
    /// A clone's workspace starts empty: no capacity is copied.
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<T> PartialEq for Retained<T> {
    /// A workspace is not part of its owner's value.
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> Retained<T> {
    /// Runs `f` on the workspace. Calls do not nest.
    pub(crate) fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        f(&mut guard)
    }
}

/// The re-formed pencil's workspace, shared by the residual stage (T8) and
/// the orthogonality stage (T10), which run one after the other.
#[derive(Default)]
pub(crate) struct ProductWorkspace {
    /// n row sums: K̂φ − λM̂φ (residual) or M̂φ (orthogonality).
    pub(crate) rows: Vec<ExactSum>,
    /// n row sums M̂φ (residual).
    pub(crate) mass_rows: Vec<ExactSum>,
    /// The generalized mass (residual) or one bilinear form (orthogonality).
    pub(crate) sum: ExactSum,
    /// The normalized copy of a sum's limbs while it is read.
    pub(crate) limbs: Vec<i64>,
    /// One row's exact terms (residual).
    pub(crate) terms: Vec<Term>,
}

/// The normalization stage's workspace (T6).
#[derive(Default)]
pub(crate) struct SumWorkspace {
    /// xᵀMy or one excitation product.
    pub(crate) sum: ExactSum,
    /// The normalized copy of the sum's limbs while it is read.
    pub(crate) limbs: Vec<i64>,
    /// One mode's below-normal shape indices, copied out at exact capacity.
    pub(crate) below: Vec<usize>,
}

/// The first `n` accumulators of `rows`, every accumulator reset. The
/// vector grows to `n` once (exact capacity) and keeps its accumulators,
/// with their limb capacity, for the next call.
pub(crate) fn reset_rows(rows: &mut Vec<ExactSum>, n: usize) -> &mut [ExactSum] {
    for row in rows.iter_mut() {
        row.reset();
    }
    if rows.len() < n {
        // Exactly n accumulators are requested (48 B each, no limbs yet).
        rows.reserve_exact(n - rows.len());
        rows.resize_with(n, ExactSum::new);
    }
    &mut rows[..n]
}
