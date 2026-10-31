//! The Hager–Higham estimate of ‖A⁻¹‖₁ for a symmetric A given by its
//! solves (Hager 1984; Higham 1988, Algorithm 4.1, with its alternative
//! estimate; LAPACK xLACN2 uses the same iteration limit). Used by the Sturm
//! counts' conditioning premise (design §3.4 check 4) and by the dense
//! path's resolution floor (design §3.2 item 5).

/// The estimator's iteration limit.
pub const INVERSE_NORM_MAX_ITERATIONS: usize = 5;

/// ν ≈ ‖A⁻¹‖₁ for A symmetric (A⁻ᵀ = A⁻¹) of order n, with `solve(x)` = A⁻¹x
/// (`None` when it fails). Every vector evaluated gives a lower bound
/// ‖A⁻¹x‖₁/‖x‖₁ and the largest is returned. A failed solve, a non-finite
/// solution or a non-finite norm gives +∞: no NaN is discarded by the
/// maximum. Returns (ν, solves).
pub fn inverse_one_norm_estimate(
    n: usize,
    mut solve: impl FnMut(&[f64]) -> Option<Vec<f64>>,
) -> (f64, usize) {
    if n == 0 {
        return (0.0, 0);
    }
    let mut solves = 0;
    let mut solve = |x: &[f64]| {
        solves += 1;
        solve(x).filter(|y| y.iter().all(|v| v.is_finite()))
    };
    // A non-finite norm (an overflowing sum) ends the estimate as +∞.
    let norm1 = |y: &[f64]| {
        let norm = y.iter().fold(0.0, |acc, x| acc + x.abs());
        norm.is_finite().then_some(norm)
    };
    let signs = |y: &[f64]| -> Vec<f64> {
        y.iter()
            .map(|&x| if x >= 0.0 { 1.0 } else { -1.0 })
            .collect()
    };
    let argmax_abs = |z: &[f64]| {
        let mut best = 0;
        for (i, x) in z.iter().enumerate() {
            if x.abs() > z[best].abs() {
                best = i;
            }
        }
        best
    };

    let estimate = (|| {
        let x = vec![1.0 / n as f64; n];
        let y = solve(&x)?;
        let mut estimate = norm1(&y)?;
        if n == 1 {
            return Some(estimate);
        }
        let mut xi = signs(&y);
        let z = solve(&xi)?;
        let mut j = argmax_abs(&z);
        let mut iteration = 2;
        loop {
            let mut e = vec![0.0; n];
            e[j] = 1.0;
            let y = solve(&e)?;
            let previous = estimate;
            let current = norm1(&y)?;
            estimate = estimate.max(current);
            let next_xi = signs(&y);
            if next_xi == xi || current <= previous {
                break;
            }
            xi = next_xi;
            let z = solve(&xi)?;
            let last = j;
            j = argmax_abs(&z);
            if z[last].abs() == z[j].abs() || iteration >= INVERSE_NORM_MAX_ITERATIONS {
                break;
            }
            iteration += 1;
        }
        let denominator = (n - 1) as f64;
        let alternative: Vec<f64> = (0..n)
            .map(|i| {
                let magnitude = 1.0 + i as f64 / denominator;
                if i % 2 == 0 {
                    magnitude
                } else {
                    -magnitude
                }
            })
            .collect();
        let y = solve(&alternative)?;
        Some(estimate.max(2.0 * norm1(&y)? / (3.0 * n as f64)))
    })();
    (
        estimate
            .filter(|value| value.is_finite())
            .unwrap_or(f64::INFINITY),
        solves,
    )
}

