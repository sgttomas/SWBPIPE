//! TEST-ONLY seam for REQ-014's refusal (DEL-04-01 AC-012, VER-007): a
//! deliberately inaccurate refinement backend. While a test holds `Active`,
//! every solve the residual refinement makes (the first iterate and each
//! correction) returns its result plus 2^-20·max(|v|, 1) in every component, so
//! the iterate never reaches REQ-014's limit and the loop's own bound ends it.
//!
//! The module is declared under `#[cfg(test)]` and every call is gated the same
//! way, so a non-test build of the kernel contains neither the module nor any
//! call: an ungated reference would not compile there. No consumer crate, and no
//! product build, can enable it (it has no feature).

use std::cell::Cell;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// While alive, the refinement backend is biased on this thread.
pub(super) struct Active;

impl Active {
    pub(super) fn new() -> Self {
        ACTIVE.with(|a| a.set(true));
        Self
    }
}

impl Drop for Active {
    fn drop(&mut self) {
        ACTIVE.with(|a| a.set(false));
    }
}

/// Biases `y` in place when the seam is active; otherwise leaves it untouched.
pub(super) fn bias(y: &mut [f64]) {
    if ACTIVE.with(|a| a.get()) {
        for v in y.iter_mut() {
            *v += v.abs().max(1.0) * 2f64.powi(-20);
        }
    }
}

/// `bias`, by value.
pub(super) fn biased(mut y: Vec<f64>) -> Vec<f64> {
    bias(&mut y);
    y
}
