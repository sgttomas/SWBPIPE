//! Pins CB's relative plane admissibility. HELP_HUMAN's M31b0 ruling
//! (2026-10-10, DEL-04-01 Design) rests on it: an admitted y_reference has
//! |y_perp| > AXIS_TOLERANCE * |y| with AXIS_TOLERANCE = 1e-12, so |y|/|y_perp|
//! stays below 1e12 for every element K-D5 re-forms. Loosening the tolerance,
//! or making it absolute, reopens that ruling.

use super::{arc_geometry, AXIS_TOLERANCE};

/// A chord along x, and y_reference = (1, t, 0) * scale: |y_perp| / |y| is
/// t / sqrt(1 + t^2), which equals t to well within the margins used here.
fn plane_admitted(t: f64, scale: f64) -> bool {
    arc_geometry([0.3, 0.0, 0.0], 0.3, [scale, t * scale, 0.0]).is_ok()
}

#[test]
fn plane_tolerance_is_relative_1e_minus_12() {
    assert_eq!(AXIS_TOLERANCE, 1.0e-12);
    for scale in [1.0e-6, 1.0, 1.0e6, 1.0e12] {
        // Comfortably above the tolerance: admitted at every scale of y.
        assert!(plane_admitted(4.0e-12, scale), "scale {scale}");
        // Below the tolerance: refused at every scale of y, so the
        // admissible |y|/|y_perp| is bounded by 1e12 whatever |y| is.
        assert!(!plane_admitted(0.25e-12, scale), "scale {scale}");
    }
}
