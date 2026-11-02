//! Dense path (design §3.2): K = R Rᵀ, C = R⁻¹ M R⁻ᵀ, Householder
//! tridiagonalization with accumulated transformations, implicit QL with
//! Wilkinson shifts (the tql2 procedure), λ = 1/μ and φ = R⁻ᵀ z.
//!
//! **Condensation of the massless subspace** (design §3.2 item 5). The
//! structurally massless DOFs (set 0, rows and columns of M exactly zero)
//! are eliminated before the reduction: K is factored with set 0 ordered
//! first, K_perm = R Rᵀ with R = [[R₀₀, 0], [R_m0, R_mm]], so R_mm R_mmᵀ is
//! the condensed stiffness K̂ = K_mm − K_m0·K₀₀⁻¹·K_0m, and the reduction runs
//! on C = R_mm⁻¹ M_mm R_mm⁻ᵀ of order n_m. The shapes are recovered as
//! R⁻ᵀ·[0; z], whose set-0 part is φ₀ = −R₀₀⁻ᵀ R_m0ᵀ φ_m = −K₀₀⁻¹K_0m φ_m. With
//! no massless DOF the order is the identity and the arithmetic is the
//! uncondensed path's.
//!
//! The tridiagonalization and QL steps follow Golub & Van Loan, *Matrix
//! Computations* (4th ed.), §8.3 (Householder tridiagonalization and the
//! implicit symmetric QR step with Wilkinson shift; QL is the same iteration
//! run from the other end), in the procedure arrangement of Bowdler, Martin,
//! Reinsch and Wilkinson's tred2 and tql2 (Wilkinson & Reinsch, *Handbook
//! for Automatic Computation* II, 1971). The code is written for this crate.
//!
//! Every loop runs in a fixed order on one thread, and only correctly rounded
//! binary64 operations are used (`hypot` is formed here from `sqrt`, not taken
//! from the platform library), so a repeated run gives bit-identical output.

use std::error::Error;
use std::fmt;

use crate::admission::{check_dense, PencilMatrix, PencilRangeUnsupported};
use crate::dd::{max_propagating_nan, WideDd};
use crate::exact::ExactSum;
use crate::matrix::SymmetricMatrix;
use crate::norm_estimate::inverse_one_norm_estimate;

/// Live n²-sized binary64 buffers at the dense path's peak phase: the
/// caller's K and M, the Cholesky factor R of K with the massless DOFs
/// ordered first (the condensation forms K̂ inside R, as its trailing
/// block, and adds no buffer), and one work buffer of n_m·n entries that
/// holds in turn M_mm R_mm⁻ᵀ, C, the accumulated Householder
/// transformations and the QL eigenvectors (n_m² of it), and finally the
/// returned mode shapes, recovered in place on the massless DOFs (kept·n).
/// O(n) work vectors are not counted. The equilibrated Cholesky of M_mm
/// (n_m², the finite-mode count) runs after R is released. The Sturm counts of §3.2 item 7 run after the solve, when
/// R and the work buffer are released, and add their own storage (`sturm`
/// module docs, "Memory").
pub const DENSE_PATH_PEAK_N2_BUFFERS: u64 = 4;

/// Bytes per n² entry at the peak phase, for design §3.2's
/// `DENSE_MODAL_BYTES_PER_ENTRY`: [`DENSE_PATH_PEAK_N2_BUFFERS`] × 8.
pub const DENSE_PATH_BYTES_PER_ENTRY: u64 = DENSE_PATH_PEAK_N2_BUFFERS * 8;

/// Peak n²-buffer memory of the dense path, in bytes:
/// [`DENSE_PATH_BYTES_PER_ENTRY`] × n². It applies no budget.
pub fn dense_path_peak_bytes(n: u64) -> u128 {
    DENSE_PATH_BYTES_PER_ENTRY as u128 * n as u128 * n as u128
}

/// Design §3.2's operation estimate 9n³ + s·n³/3 for `s` Sturm
/// factorizations, rounded up to a whole flop. Band-recovery exact products
/// are additional (`dense_screen_product_estimate`); this applies no budget.
pub fn dense_path_flop_estimate(n: u64, sturm_factorizations: u64) -> u128 {
    let cube = n as u128 * n as u128 * n as u128;
    9 * cube + (sturm_factorizations as u128 * cube).div_ceil(3)
}

/// Algorithm parameters of the dense path. There is no default: the
/// production values are fixed and recorded by the design (M1b), not here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DenseOptions {
    /// μ ≤ this × max μ (and every μ ≤ 0) is treated as an infinite
    /// eigenvalue of the pencil, from a singular M, and is counted but not
    /// returned. Must be finite and in [0, 1).
    pub infinite_mu_relative_threshold: f64,
    /// τ_m: the equilibrated mass pivots of the finite-mode count must be at
    /// least this (see [`established_finite_mode_count`]). Must be finite
    /// and in [0, 1).
    pub mass_pivot_relative_threshold: f64,
    /// c_μ of the resolution floor δ_μ = c_μ·n·u·ν(K̃̂)·‖M̃_mm‖_∞ on the
    /// diagonally equilibrated pencil (design §3.2 item 5). Finite and
    /// positive.
    pub resolution_floor_constant: f64,
    /// κ_μ: μ must exceed κ_μ·δ_q as well as τ_μ·μ_max; κ_μ·δ_μ
    /// screens the remaining band. Finite and positive.
    pub resolution_floor_multiple: f64,
    /// c_f: a recovered band pair's equilibrated formed backward error must
    /// be at most τ_f = c_f·n·u. Finite and positive.
    pub formed_backward_error_multiple: f64,
    /// Dimensionless maximum e_λ for a recovered band mode. No default or
    /// production pin; measured under DEC-026.
    pub screen_error_estimate_threshold: f64,
    /// The most implicit QL iterations allowed for any one eigenvalue.
    pub max_ql_iterations_per_eigenvalue: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DenseError {
    DimensionMismatch {
        stiffness: usize,
        mass: usize,
    },
    EmptySystem,
    /// A nonzero formed entry outside the admission range
    /// (`MODAL_PENCIL_RANGE_UNSUPPORTED`).
    PencilRangeUnsupported(PencilRangeUnsupported),
    InvalidOption {
        name: &'static str,
    },
    /// The Cholesky factorization of K met a pivot that is not positive (or
    /// is NaN). `index` is the row of K, in the caller's numbering.
    NonPositivePivot {
        index: usize,
        value: f64,
    },
    /// Implicit QL did not converge for the eigenvalue at tridiagonal position
    /// `index` within the stated iteration limit.
    QlIterationLimit {
        index: usize,
        limit: usize,
    },
    /// An intermediate of the reduction that is not finite: the stiffness
    /// factor, the reduced matrix C or an eigenvalue of C (`stage`, `index`
    /// into its flat list), checked before any classification
    /// (`MODAL_PENCIL_RANGE_UNSUPPORTED`: the admitted pencil's reduction
    /// leaves the binary64 range).
    ReductionOutOfRange {
        stage: &'static str,
        index: usize,
    },
    /// A returned μ or λ that is not a positive normal number, or a mode
    /// shape component that is not finite (`index` into the flat list).
    ResultOutOfRange {
        quantity: &'static str,
        index: usize,
    },
}

impl fmt::Display for DenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch { stiffness, mass } => write!(
                f,
                "stiffness is {stiffness}x{stiffness} but mass is {mass}x{mass}"
            ),
            Self::EmptySystem => write!(f, "the pencil has no degrees of freedom"),
            Self::PencilRangeUnsupported(range) => write!(f, "{range}"),
            Self::InvalidOption { name } => write!(f, "invalid dense option `{name}`"),
            Self::NonPositivePivot { index, value } => write!(
                f,
                "stiffness is not positive definite: Cholesky pivot {index} is {value:e}"
            ),
            Self::QlIterationLimit { index, limit } => write!(
                f,
                "implicit QL did not converge for eigenvalue {index} within {limit} iterations"
            ),
            Self::ReductionOutOfRange { stage, index } => write!(
                f,
                "{}: the dense reduction's {stage} [{index}] is not finite",
                crate::admission::PENCIL_RANGE_UNSUPPORTED
            ),
            Self::ResultOutOfRange { quantity, index } => {
                write!(f, "dense result `{quantity}`[{index}] is out of range")
            }
        }
    }
}

impl Error for DenseError {}

/// The finite eigenpairs of K φ = λ M φ, λ ascending.
#[derive(Debug, Clone, PartialEq)]
pub struct DenseEigenSolution {
    /// Order of the pencil.
    pub dimension: usize,
    /// n_m, the order of the condensed pencil: n minus the structurally
    /// massless DOFs.
    pub condensed_dimension: usize,
    /// λ = 1/μ, ascending.
    pub eigenvalues: Vec<f64>,
    /// μ, the eigenvalues of the condensed C, in the same order
    /// (descending).
    pub mu: Vec<f64>,
    /// Mode shapes over all n DOFs in the caller's numbering, row-major:
    /// mode i is `vectors[i * dimension..(i + 1) * dimension]`, recovered
    /// on the massless DOFs (module docs). Unnormalized: z has unit
    /// Euclidean norm, so φᵀKφ = 1 to rounding.
    pub vectors: Vec<f64>,
    /// The largest condensed μ, against which the relative threshold is
    /// applied (0 when n_m = 0).
    pub mu_max: f64,
    /// ν(K̃̂), the Hager–Higham estimate of ‖K̃̂⁻¹‖₁ for the equilibrated
    /// condensed stiffness K̃̂ = D_m K̂ D_m, D = diag(k_jj)^−½, from the
    /// condensed Cholesky factor (+∞ when a solve overflowed).
    pub condensed_inverse_norm_estimate: f64,
    /// ‖M̃_mm‖_∞ = ‖D_m M_mm D_m‖_∞.
    pub condensed_mass_norm: f64,
    /// δ_μ = c_μ·n·u·ν(K̃̂)·‖M̃_mm‖_∞, the resolution floor of the reduction,
    /// invariant to a diagonal rescaling of the basis.
    pub resolution_floor: f64,
    /// max(hard_classification_threshold, κ_μ·δ_μ). Above it no recovery
    /// is needed; only the band above the hard threshold permits recovery.
    pub classification_threshold: f64,
    /// δ_q = c_μ·n·u·μ_max: the QL resolution observation.
    pub ql_resolution_floor: f64,
    /// max(τ_μ·μ_max, κ_μ·δ_q): a hard verdict, never rescued.
    pub hard_classification_threshold: f64,
    /// τ_f = c_f·n·u.
    pub formed_backward_error_threshold: f64,
    /// Every positive QL μ at or below the broad threshold, descending,
    /// with the hard rejection or band recovery and its decision.
    pub screened: Vec<ScreenedMode>,
    /// Condensed μ ≤ 0: classified infinite, not returned.
    pub nonpositive_mu_count: usize,
    /// Positive condensed μ rejected by either hard floor or by band
    /// recovery: classified infinite, not returned.
    pub below_threshold_mu_count: usize,
    /// Total implicit QL iterations.
    pub ql_iterations: usize,
    /// min over j of R_jj² / K_jj, a Cholesky pivot-ratio observation (not a
    /// condition number).
    pub min_cholesky_pivot_ratio: f64,
    /// Free DOFs whose rows and columns of M are exactly zero (design §3.2
    /// item 5).
    pub structurally_massless_count: usize,
    /// [`established_finite_mode_count`] of M with τ_m: n minus the
    /// structurally massless DOFs, or `None` when the equilibrated remaining
    /// block does not pass.
    pub established_finite_mode_count: Option<usize>,
    /// The finite-mode count is unresolved (`MODAL_FINITE_COUNT_UNRESOLVED`
    /// when the whole finite spectrum is needed) when a condensed μ is
    /// rejected or the independent mass census is not established. No
    /// non-coordinate nullspace is truncated or turned into a resolved count.
    pub finite_count_unresolved: bool,
}

impl DenseEigenSolution {
    pub fn mode_count(&self) -> usize {
        self.eigenvalues.len()
    }

    pub fn vector(&self, mode: usize) -> &[f64] {
        &self.vectors[mode * self.dimension..(mode + 1) * self.dimension]
    }
}

/// Solves K φ = λ M φ for K symmetric positive definite and M symmetric
/// positive semidefinite (possibly singular), both dense n×n.
pub fn solve_dense_pencil(
    stiffness: &SymmetricMatrix,
    mass: &SymmetricMatrix,
    options: &DenseOptions,
) -> Result<DenseEigenSolution, DenseError> {
    let n = stiffness.dimension();
    if mass.dimension() != n {
        return Err(DenseError::DimensionMismatch {
            stiffness: n,
            mass: mass.dimension(),
        });
    }
    if n == 0 {
        return Err(DenseError::EmptySystem);
    }
    check_dense(PencilMatrix::Stiffness, stiffness).map_err(DenseError::PencilRangeUnsupported)?;
    check_dense(PencilMatrix::Mass, mass).map_err(DenseError::PencilRangeUnsupported)?;
    let threshold = options.infinite_mu_relative_threshold;
    if !(threshold.is_finite() && (0.0..1.0).contains(&threshold)) {
        return Err(DenseError::InvalidOption {
            name: "infinite_mu_relative_threshold",
        });
    }
    let pivot_threshold = options.mass_pivot_relative_threshold;
    if !(pivot_threshold.is_finite() && (0.0..1.0).contains(&pivot_threshold)) {
        return Err(DenseError::InvalidOption {
            name: "mass_pivot_relative_threshold",
        });
    }
    for (name, value) in [
        (
            "screen_error_estimate_threshold",
            options.screen_error_estimate_threshold,
        ),
        (
            "resolution_floor_constant",
            options.resolution_floor_constant,
        ),
        (
            "resolution_floor_multiple",
            options.resolution_floor_multiple,
        ),
        (
            "formed_backward_error_multiple",
            options.formed_backward_error_multiple,
        ),
    ] {
        if !(value.is_finite() && value > 0.0) {
            return Err(DenseError::InvalidOption { name });
        }
    }

    if options.screen_error_estimate_threshold >= 1.0 {
        return Err(DenseError::InvalidOption {
            name: "screen_error_estimate_threshold",
        });
    }

    // Set 0 (structurally massless) first, then the massive DOFs, each
    // ascending.
    let massless = structurally_massless_dofs(mass);
    let n0 = massless.len();
    let nm = n - n0;
    let massive: Vec<usize> = (0..n)
        .filter(|i| massless.binary_search(i).is_err())
        .collect();
    let order: Vec<usize> = massless.iter().chain(&massive).copied().collect();
    let (r, min_cholesky_pivot_ratio) = cholesky_ordered(stiffness, &order)?;
    check_finite("stiffness factor", &r)?;

    // The work buffer is reserved at n_m·n so the recovered shapes fit in
    // place (`DENSE_PATH_PEAK_N2_BUFFERS`).
    let mut work = Vec::with_capacity(nm * n);
    reduce_mass(&r, n, n0, mass, &massive, &mut work);
    // No NaN may reach the classification, where it would count as μ ≤ 0
    // (review round 4: an admitted, exactly SPD K whose factor's inverse
    // overflows).
    check_finite("reduced matrix", &work)?;
    let mut d = vec![0.0; nm];
    let mut e = vec![0.0; nm];
    let mut ql_iterations = 0;
    if nm > 0 {
        householder_tridiagonalize(&mut work, nm, &mut d, &mut e);
        transpose_in_place(&mut work, nm);
        ql_iterations = implicit_ql(
            &mut d,
            &mut e,
            &mut work,
            nm,
            options.max_ql_iterations_per_eigenvalue,
        )?;
        check_finite("eigenvalues of the reduced matrix", &d)?;
        sort_ascending(&mut d, &mut work, nm);
    }

    // The resolution floor (design §3.2 item 5) on the diagonally
    // equilibrated pencil, D = diag(k_jj)^−½ as in the Sturm counts, a
    // congruence: δ_μ = c_μ·n·u·ν(K̃̂)·‖M̃_mm‖, with K̃̂ = D_m K̂ D_m, so
    // K̃̂⁻¹x = D_m⁻¹ K̂⁻¹ D_m⁻¹ x from solves with the condensed factor R_mm.
    let root: Vec<f64> = massive
        .iter()
        .map(|&j| stiffness.get(j, j).sqrt())
        .collect();
    let (condensed_inverse_norm_estimate, _) = inverse_one_norm_estimate(nm, |x| {
        let mut y: Vec<f64> = x.iter().zip(&root).map(|(v, s)| v * s).collect();
        forward_solve_row(&r, n, n0, &mut y);
        back_solve_block(&r, n, n0, &mut y);
        Some(y.iter().zip(&root).map(|(v, s)| v * s).collect())
    });
    let condensed_mass_norm = massive.iter().enumerate().fold(0.0, |norm, (a, &i)| {
        let row: f64 = massive
            .iter()
            .enumerate()
            .map(|(b, &j)| mass.get(i, j).abs() / root[a] / root[b])
            .sum();
        max_propagating_nan(norm, row)
    });
    let resolution_floor = options.resolution_floor_constant
        * n as f64
        * (f64::EPSILON / 2.0)
        * condensed_inverse_norm_estimate
        * condensed_mass_norm;

    // d is ascending μ; λ ascending is μ descending. Reverse the rows so the
    // positive μ come first; μ ≤ 0 is classified infinite.
    let mu_max = d.last().copied().unwrap_or(0.0);
    let ql_resolution_floor =
        options.resolution_floor_constant * n as f64 * (f64::EPSILON / 2.0) * mu_max;
    let hard_cut = max_propagating_nan(
        threshold * mu_max,
        options.resolution_floor_multiple * ql_resolution_floor,
    );
    let cut = max_propagating_nan(
        hard_cut,
        options.resolution_floor_multiple * resolution_floor,
    );
    for (quantity, value) in [
        ("condensed inverse norm", condensed_inverse_norm_estimate),
        ("condensed mass norm", condensed_mass_norm),
        ("formation resolution floor", resolution_floor),
        ("QL resolution floor", ql_resolution_floor),
        ("classification threshold", cut),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(DenseError::ResultOutOfRange { quantity, index: 0 });
        }
    }
    let nonpositive_mu_count = d.iter().filter(|&&mu| !(mu > 0.0)).count();
    let positive = nm - nonpositive_mu_count;
    d.reverse();
    for i in 0..nm / 2 {
        swap_rows(&mut work, nm, i, nm - 1 - i);
    }
    d.truncate(positive);
    work.truncate(positive * nm);
    // Rows of n_m to rows of n, last first so no row is overwritten before
    // it moves (within the reserved capacity, no reallocation).
    work.resize(positive * n, 0.0);
    for i in (0..positive).rev() {
        work.copy_within(i * nm..(i + 1) * nm, i * n);
    }
    let mut ordered = vec![0.0; n];
    for row in work.chunks_exact_mut(n) {
        // φ in the factor's order is R⁻ᵀ·[0; z] (module docs).
        ordered[..n0].fill(0.0);
        ordered[n0..].copy_from_slice(&row[..nm]);
        back_transform(&r, n, &mut ordered);
        for (k, &dof) in order.iter().enumerate() {
            row[dof] = ordered[k];
        }
    }
    drop(r);
    check_finite("mode shapes", &work)?;

    // Only the band above both hard verdicts may be recovered. The broad
    // formation floor screens it; it never rescues a μ below QL resolution.
    let tau_f = options.formed_backward_error_multiple * n as f64 * (f64::EPSILON / 2.0);
    if !tau_f.is_finite() || !hard_cut.is_finite() || hard_cut < 0.0 {
        return Err(DenseError::ResultOutOfRange {
            quantity: "classification thresholds",
            index: 0,
        });
    }
    let scaling = if d.iter().any(|&mu| mu > hard_cut && mu <= cut) {
        Some(formed_scaling(stiffness, mass))
    } else {
        None
    };
    let mut screened = Vec::new();
    let mut retained_rows = Vec::with_capacity(positive);
    for row in 0..d.len() {
        let mu = d[row];
        if classify_mu(mu, hard_cut, cut) == MuRegion::Direct {
            retained_rows.push(row);
            continue;
        }
        let phi = &work[row * n..(row + 1) * n];
        let recovery = if classify_mu(mu, hard_cut, cut) == MuRegion::HardRejected {
            None
        } else {
            recover_band_mode(stiffness, mass, scaling.as_ref().unwrap(), phi)
        };
        let retained = recovery.as_ref().is_some_and(|r| {
            r.mu > hard_cut && screen_passes(r, tau_f, options.screen_error_estimate_threshold)
        });
        if retained {
            retained_rows.push(row);
            d[row] = recovery.as_ref().unwrap().mu;
        }
        screened.push(ScreenedMode {
            mu,
            eigenvalue: 1.0 / mu,
            hard_rejected: mu <= hard_cut,
            recovery,
            retained,
        });
    }
    let below_threshold_mu_count = screened.iter().filter(|s| !s.retained).count();
    // Keep the retained rows, in order (each moves only to an earlier slot).
    for (slot, &row) in retained_rows.iter().enumerate() {
        if slot != row {
            work.copy_within(row * n..(row + 1) * n, slot * n);
            d[slot] = d[row];
        }
    }
    let kept = retained_rows.len();
    d.truncate(kept);
    work.truncate(kept * n);
    // Recovery changes μ. Sort descending in place, carrying the full rows
    // (the condensed row width may differ), with no new n² buffer.
    for i in 0..kept.saturating_sub(1) {
        let mut next = i;
        for j in i + 1..kept {
            if d[j] > d[next] {
                next = j;
            }
        }
        if next != i {
            d.swap(i, next);
            swap_rows(&mut work, n, i, next);
        }
    }
    // After R is released, so the mass block's factor does not add to the
    // peak n² buffers.
    let established = established_finite_mode_count(mass, pivot_threshold);
    let finite_count_unresolved =
        nonpositive_mu_count + below_threshold_mu_count > 0 || established.is_none();
    let eigenvalues: Vec<f64> = d.iter().map(|&mu| 1.0 / mu).collect();
    validate_outputs(&d, &eigenvalues, &work)?;
    Ok(DenseEigenSolution {
        dimension: n,
        condensed_dimension: nm,
        eigenvalues,
        mu: d,
        vectors: work,
        mu_max,
        condensed_inverse_norm_estimate,
        condensed_mass_norm,
        resolution_floor,
        classification_threshold: cut,
        ql_resolution_floor,
        hard_classification_threshold: hard_cut,
        formed_backward_error_threshold: tau_f,
        screened,
        nonpositive_mu_count,
        below_threshold_mu_count,
        ql_iterations,
        min_cholesky_pivot_ratio,
        structurally_massless_count: n0,
        established_finite_mode_count: established,
        finite_count_unresolved,
    })
}

#[derive(Debug, PartialEq)]
enum MuRegion {
    HardRejected,
    Band,
    Direct,
}

fn classify_mu(mu: f64, hard: f64, broad: f64) -> MuRegion {
    if mu <= hard {
        MuRegion::HardRejected
    } else if mu <= broad {
        MuRegion::Band
    } else {
        MuRegion::Direct
    }
}

/// Maximum exact-product insertions for p screened pairs: two quadratic
/// forms and two residual products per dense entry, 4pn². This is additional
/// work to the binary64 eigensolve's flop estimate, not interchangeable
/// with binary64 flops. A product has at most three binary64 factors (up to
/// four exact parts); the consuming guard must price these separately.
/// No product entry consumes this kernel yet, so this applies no budget.
pub fn dense_screen_product_estimate(n: u64, screened_pairs: u64) -> u128 {
    4 * screened_pairs as u128 * n as u128 * n as u128
}

/// O(n) diagonal scaling and two norms; no scaled n² matrix is formed.
struct FormedScaling {
    roots: Vec<f64>,
    stiffness_norm: WideDd,
    mass_norm: WideDd,
}

fn formed_scaling(k: &SymmetricMatrix, m: &SymmetricMatrix) -> FormedScaling {
    let n = k.dimension();
    let roots: Vec<f64> = (0..n).map(|i| k.get(i, i).sqrt()).collect();
    let norm = |a: &SymmetricMatrix| {
        let mut largest = WideDd::ZERO;
        for i in 0..n {
            let mut row = WideDd::ZERO;
            for j in 0..n {
                row = row.add(
                    WideDd::from_f64(a.get(i, j).abs())
                        .div(WideDd::from_f64(roots[i]))
                        .div(WideDd::from_f64(roots[j])),
                );
            }
            if !row.is_finite() {
                return row;
            }
            if row.sub(largest).is_positive() {
                largest = row;
            }
        }
        largest
    };
    FormedScaling {
        stiffness_norm: norm(k),
        mass_norm: norm(m),
        roots,
    }
}

/// Recovery of one band pair on the formed, uncondensed pencil. Products
/// entering the Rayleigh quotient and each residual row are summed exactly;
/// norms and ratios use extended-exponent double-double. The mass and energy
/// must be positive before division. Every compared output must be normal
/// (or an exact zero); a failed premise rejects the pair.
fn recover_band_mode(
    k: &SymmetricMatrix,
    m: &SymmetricMatrix,
    scaling: &FormedScaling,
    phi: &[f64],
) -> Option<BandRecovery> {
    let mut energy = ExactSum::new();
    let mut mass = ExactSum::new();
    for i in 0..phi.len() {
        for j in 0..phi.len() {
            energy.add_product(&[phi[i], k.get(i, j), phi[j]]);
            mass.add_product(&[phi[i], m.get(i, j), phi[j]]);
        }
    }
    let energy = energy.to_wide();
    let mass = mass.to_wide();
    if !energy.is_finite() || !energy.is_positive() || !mass.is_finite() || !mass.is_positive() {
        return None;
    }
    let mu = mass.div(energy).to_normal_or_zero()?;
    let eigenvalue = 1.0 / mu;
    if !(mu > 0.0 && eigenvalue.is_normal() && eigenvalue > 0.0) {
        return None;
    }
    let (backward_error, error_estimate) =
        formed_pair_errors(k, m, scaling, eigenvalue, phi, mass)?;
    Some(BandRecovery {
        mu,
        eigenvalue,
        backward_error,
        error_estimate,
    })
}

/// Check-2 form on the formed pencil with D=diag(k_jj)^−½:
/// η̃=‖Dr‖₂/((‖DKD‖∞+λ‖DMD‖∞)‖D⁻¹φ‖₂),
/// e_λ=‖Dr‖₂‖D⁻¹φ‖₂/(λ φᵀMφ). Both are dimensionless estimates,
/// not error bounds. Using a global norm alone cannot detect a lost high
/// mode in a nearly massless direction.
fn formed_pair_errors(
    k: &SymmetricMatrix,
    m: &SymmetricMatrix,
    scaling: &FormedScaling,
    eigenvalue: f64,
    phi: &[f64],
    mass: WideDd,
) -> Option<(f64, f64)> {
    if !mass.is_finite() || !mass.is_positive() {
        return None;
    }
    let mut residual = WideDd::ZERO;
    let mut shape = WideDd::ZERO;
    for i in 0..phi.len() {
        let mut row = ExactSum::new();
        for (j, &x) in phi.iter().enumerate() {
            row.add_product(&[k.get(i, j), x]);
            row.add_product(&[-eigenvalue, m.get(i, j), x]);
        }
        let r = row.to_wide().div(WideDd::from_f64(scaling.roots[i]));
        residual = residual.add(r.mul(r));
        let x = WideDd::from_f64(phi[i]).mul_f64(scaling.roots[i]);
        shape = shape.add(x.mul(x));
    }
    if !shape.is_finite() || !shape.is_positive() {
        return None;
    }
    let residual = residual.sqrt();
    let shape = shape.sqrt();
    let lambda = WideDd::from_f64(eigenvalue);
    let norm = scaling.stiffness_norm.add(scaling.mass_norm.mul(lambda));
    if !norm.is_finite() || !norm.is_positive() {
        return None;
    }
    let eta = residual.div(norm.mul(shape)).to_normal_or_zero()?;
    let estimate = residual
        .mul(shape)
        .div(lambda.mul(mass))
        .to_normal_or_zero()?;
    if eta < 0.0 || estimate < 0.0 {
        return None;
    }
    Some((eta, estimate))
}

fn screen_passes(r: &BandRecovery, tau_f: f64, tau_e: f64) -> bool {
    r.backward_error.is_finite()
        && r.error_estimate.is_finite()
        && r.backward_error >= 0.0
        && r.error_estimate >= 0.0
        && r.backward_error <= tau_f
        && r.error_estimate <= tau_e
}

#[derive(Debug, Clone, PartialEq)]
pub struct BandRecovery {
    /// μ recovered from the exact generalized mass and stiffness energy.
    pub mu: f64,
    /// Exactly the binary64 reciprocal of `mu`.
    pub eigenvalue: f64,
    pub backward_error: f64,
    pub error_estimate: f64,
}

/// A candidate at or below the broad classification threshold. Hard
/// rejections have no recovery. A failed sign/range premise also gives
/// `None`; neither is rescued. Only the band has error estimates.
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenedMode {
    /// Original QL value, before any Rayleigh recovery.
    pub mu: f64,
    pub eigenvalue: f64,
    pub hard_rejected: bool,
    pub recovery: Option<BandRecovery>,
    pub retained: bool,
}

fn check_finite(stage: &'static str, values: &[f64]) -> Result<(), DenseError> {
    match values.iter().position(|v| !v.is_finite()) {
        Some(index) => Err(DenseError::ReductionOutOfRange { stage, index }),
        None => Ok(()),
    }
}

/// Every returned μ and λ is a positive normal number and every shape
/// component finite (design §3.2, admission range: no silent zero, no NaN).
fn validate_outputs(mu: &[f64], eigenvalues: &[f64], vectors: &[f64]) -> Result<(), DenseError> {
    for (quantity, list) in [("mu", mu), ("eigenvalues", eigenvalues)] {
        if let Some(index) = list.iter().position(|&x| !(x.is_normal() && x > 0.0)) {
            return Err(DenseError::ResultOutOfRange { quantity, index });
        }
    }
    if let Some(index) = vectors.iter().position(|x| !x.is_finite()) {
        return Err(DenseError::ResultOutOfRange {
            quantity: "vectors",
            index,
        });
    }
    Ok(())
}

/// The structurally massless DOFs of M, ascending: those whose row (and so
/// column) is exactly zero.
pub fn structurally_massless_dofs(mass: &SymmetricMatrix) -> Vec<usize> {
    let n = mass.dimension();
    (0..n)
        .filter(|&i| (0..n).all(|j| mass.get(i, j) == 0.0))
        .collect()
}

/// The finite-mode count established independently of the eigenvalues
/// (design §3.2 item 5): n minus the structurally massless DOFs, when the
/// remaining block of M, equilibrated by the congruence D_M·M·D_M with
/// D_M = diag(m_jj)^−½, factors by Cholesky with every pivot (before its
/// square root) at least `relative_pivot_threshold` τ_m, inclusive, and
/// positive. `None` when the count is not established: a pivot below τ_m,
/// a remaining diagonal that is not positive, or a threshold that is not
/// finite in [0, 1).
///
/// The equilibrated block has unit diagonal exactly, so it is formed with
/// 1 there and m_ij/√m_ii/√m_jj off it. Its pivots are dimensionless and do
/// not depend on units or on the mix of translational and rotational DOFs.
pub fn established_finite_mode_count(
    mass: &SymmetricMatrix,
    relative_pivot_threshold: f64,
) -> Option<usize> {
    if !(relative_pivot_threshold.is_finite() && (0.0..1.0).contains(&relative_pivot_threshold)) {
        return None;
    }
    let massless = structurally_massless_dofs(mass);
    let kept: Vec<usize> = (0..mass.dimension())
        .filter(|i| massless.binary_search(i).is_err())
        .collect();
    let m = kept.len();
    let mut root = Vec::with_capacity(m);
    for &i in &kept {
        let diagonal = mass.get(i, i);
        if !(diagonal > 0.0) {
            return None;
        }
        root.push(diagonal.sqrt());
    }
    let equilibrated = |a: usize, b: usize| {
        if a == b {
            1.0
        } else {
            mass.get(kept[a], kept[b]) / root[a] / root[b]
        }
    };
    let mut r = vec![0.0; m * m];
    for a in 0..m {
        for b in 0..=a {
            let mut s = equilibrated(a, b);
            for p in 0..b {
                s -= r[a * m + p] * r[b * m + p];
            }
            if a == b {
                if !(s >= relative_pivot_threshold && s > 0.0) {
                    return None;
                }
                r[a * m + a] = s.sqrt();
            } else {
                r[a * m + b] = s / r[b * m + b];
            }
        }
    }
    Some(m)
}

/// P K Pᵀ = R Rᵀ with R lower triangular, row by row
/// (Cholesky–Banachiewicz), for the ordering P given by `order` (position
/// i holds DOF `order[i]`). Returns R in an n² row-major buffer (upper part
/// zero) and the smallest pivot ratio R_jj² / K_jj. A pivot error names
/// the DOF in the caller's numbering.
fn cholesky_ordered(k: &SymmetricMatrix, order: &[usize]) -> Result<(Vec<f64>, f64), DenseError> {
    let n = k.dimension();
    let mut r = vec![0.0; n * n];
    let mut min_ratio = f64::INFINITY;
    for i in 0..n {
        let k_ij = |j: usize| k.get(order[i], order[j]);
        for j in 0..=i {
            let mut s = k_ij(j);
            for p in 0..j {
                s -= r[i * n + p] * r[j * n + p];
            }
            if i == j {
                if !(s > 0.0) {
                    return Err(DenseError::NonPositivePivot {
                        index: order[i],
                        value: s,
                    });
                }
                let ratio = s / k_ij(i);
                if ratio < min_ratio {
                    min_ratio = ratio;
                }
                r[i * n + i] = s.sqrt();
            } else {
                r[i * n + j] = s / r[j * n + j];
            }
        }
    }
    Ok((r, min_ratio))
}

/// Solves x R_bbᵀ = b for the row vector x in place, R_bb the trailing
/// diagonal block of R from row and column `offset` (forward substitution
/// over R's contiguous rows).
fn forward_solve_row(r: &[f64], n: usize, offset: usize, row: &mut [f64]) {
    for j in 0..row.len() {
        let base = (offset + j) * n + offset;
        let r_row = &r[base..base + j];
        let mut s = row[j];
        for p in 0..j {
            s -= r_row[p] * row[p];
        }
        row[j] = s / r[base + j];
    }
}

/// Solves R_bbᵀ y = w in place, R_bb the trailing diagonal block of R from
/// `offset` (back substitution by columns of R_bbᵀ, R's contiguous rows).
fn back_solve_block(r: &[f64], n: usize, offset: usize, row: &mut [f64]) {
    for i in (0..row.len()).rev() {
        let base = (offset + i) * n + offset;
        let value = row[i] / r[base + i];
        row[i] = value;
        let r_row = &r[base..base + i];
        for p in 0..i {
            row[p] -= r_row[p] * value;
        }
    }
}

/// Solves Rᵀ φ = z in place (back substitution by columns of Rᵀ, which are
/// R's contiguous rows).
fn back_transform(r: &[f64], n: usize, row: &mut [f64]) {
    for i in (0..n).rev() {
        let value = row[i] / r[i * n + i];
        row[i] = value;
        let r_row = &r[i * n..i * n + i];
        for p in 0..i {
            row[p] -= r_row[p] * value;
        }
    }
}

/// C = R_mm⁻¹ M_mm R_mm⁻ᵀ into `work` (n_m² entries), R_mm the trailing
/// block of R from `offset` and M_mm the rows and columns `massive` of M:
/// rows of M_mm R_mm⁻ᵀ, transpose (giving R_mm⁻¹ M_mm, since M is
/// symmetric), rows again, then symmetrize from the lower triangle.
fn reduce_mass(
    r: &[f64],
    n: usize,
    offset: usize,
    mass: &SymmetricMatrix,
    massive: &[usize],
    work: &mut Vec<f64>,
) {
    let nm = massive.len();
    work.clear();
    for &i in massive {
        work.extend(massive.iter().map(|&j| mass.get(i, j)));
    }
    if nm == 0 {
        return;
    }
    for row in work.chunks_exact_mut(nm) {
        forward_solve_row(r, n, offset, row);
    }
    transpose_in_place(work, nm);
    for row in work.chunks_exact_mut(nm) {
        forward_solve_row(r, n, offset, row);
    }
    for i in 0..nm {
        for j in i + 1..nm {
            work[i * nm + j] = work[j * nm + i];
        }
    }
}

fn transpose_in_place(a: &mut [f64], n: usize) {
    for i in 0..n {
        for j in i + 1..n {
            a.swap(i * n + j, j * n + i);
        }
    }
}

fn swap_rows(a: &mut [f64], n: usize, i: usize, j: usize) {
    if i == j {
        return;
    }
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    let (head, tail) = a.split_at_mut(hi * n);
    head[lo * n..lo * n + n].swap_with_slice(&mut tail[..n]);
}

/// Two distinct rows as mutable slices, `(row i, row j)`.
fn row_pair(a: &mut [f64], n: usize, i: usize, j: usize) -> (&mut [f64], &mut [f64]) {
    debug_assert!(i < j);
    let (head, tail) = a.split_at_mut(j * n);
    (&mut head[i * n..i * n + n], &mut tail[..n])
}

/// √(a² + b²) without overflow, from correctly rounded operations only.
fn hypot(a: f64, b: f64) -> f64 {
    let (a, b) = (a.abs(), b.abs());
    let (big, small) = if a >= b { (a, b) } else { (b, a) };
    if big == 0.0 {
        return 0.0;
    }
    let ratio = small / big;
    big * (1.0 + ratio * ratio).sqrt()
}

/// Householder reduction of the symmetric matrix in `v` (lower triangle
/// read) to tridiagonal form T = Qᵀ A Q, with Q accumulated into `v`.
/// On return `d` holds T's diagonal, `e[i]` the subdiagonal T(i, i−1)
/// (e[0] = 0) and `v` holds Q, column j the j-th basis vector.
///
/// Rows are reduced from the last upwards. For row i the Householder vector
/// is formed from its scaled leading part (scaling by the 1-norm avoids
/// overflow and underflow); the rank-two update A ← A − u wᵀ − w uᵀ is applied
/// to the leading i×i block; the transformations are then accumulated from
/// the first upwards.
fn householder_tridiagonalize(v: &mut [f64], n: usize, d: &mut [f64], e: &mut [f64]) {
    for j in 0..n {
        d[j] = v[(n - 1) * n + j];
    }
    for i in (1..n).rev() {
        let mut scale = 0.0;
        let mut h = 0.0;
        for k in 0..i {
            scale += d[k].abs();
        }
        if scale == 0.0 {
            e[i] = d[i - 1];
            for j in 0..i {
                d[j] = v[(i - 1) * n + j];
                v[i * n + j] = 0.0;
                v[j * n + i] = 0.0;
            }
        } else {
            for k in 0..i {
                d[k] /= scale;
                h += d[k] * d[k];
            }
            let f = d[i - 1];
            let mut g = h.sqrt();
            if f > 0.0 {
                g = -g;
            }
            e[i] = scale * g;
            h -= f * g;
            d[i - 1] = f - g;
            for j in 0..i {
                e[j] = 0.0;
            }
            // w = A u / h, accumulated over the lower triangle.
            for j in 0..i {
                let f = d[j];
                v[j * n + i] = f;
                let mut g = e[j] + v[j * n + j] * f;
                for k in j + 1..i {
                    g += v[k * n + j] * d[k];
                    e[k] += v[k * n + j] * f;
                }
                e[j] = g;
            }
            let mut f = 0.0;
            for j in 0..i {
                e[j] /= h;
                f += e[j] * d[j];
            }
            let hh = f / (h + h);
            for j in 0..i {
                e[j] -= hh * d[j];
            }
            for j in 0..i {
                let f = d[j];
                let g = e[j];
                for k in j..i {
                    v[k * n + j] -= f * e[k] + g * d[k];
                }
                d[j] = v[(i - 1) * n + j];
                v[i * n + j] = 0.0;
            }
        }
        d[i] = h;
    }
    // Accumulate Q.
    for i in 0..n - 1 {
        v[(n - 1) * n + i] = v[i * n + i];
        v[i * n + i] = 1.0;
        let h = d[i + 1];
        if h != 0.0 {
            for k in 0..=i {
                d[k] = v[k * n + i + 1] / h;
            }
            for j in 0..=i {
                let mut g = 0.0;
                for k in 0..=i {
                    g += v[k * n + i + 1] * v[k * n + j];
                }
                for k in 0..=i {
                    v[k * n + j] -= g * d[k];
                }
            }
        }
        for k in 0..=i {
            v[k * n + i + 1] = 0.0;
        }
    }
    for j in 0..n {
        d[j] = v[(n - 1) * n + j];
        v[(n - 1) * n + j] = 0.0;
    }
    v[(n - 1) * n + n - 1] = 1.0;
    e[0] = 0.0;
}

/// Implicit QL with Wilkinson shifts on the tridiagonal (d, e), applying
/// each Givens rotation to the rows of `z` (row j is the j-th vector, the
/// transpose of the Householder accumulation). Returns the total iteration
/// count.
///
/// For each l, the first negligible subdiagonal at or below l
/// (|e_m| ≤ ε·max_{j≤l}(|d_j| + |e_j|), ε = 2⁻⁵²) bounds the unreduced block
/// l..=m. The shift is the eigenvalue of the leading 2×2 block nearer d_l;
/// it is applied explicitly to the block and accumulated in `shift_total`,
/// and a QL sweep of rotations from m−1 up to l chases the bulge.
fn implicit_ql(
    d: &mut [f64],
    e: &mut [f64],
    z: &mut [f64],
    n: usize,
    limit: usize,
) -> Result<usize, DenseError> {
    for i in 1..n {
        e[i - 1] = e[i];
    }
    e[n - 1] = 0.0;
    let eps = f64::EPSILON;
    let mut shift_total = 0.0;
    let mut tst1: f64 = 0.0;
    let mut total = 0;
    for l in 0..n {
        tst1 = tst1.max(d[l].abs() + e[l].abs());
        let mut m = l;
        while m < n - 1 && e[m].abs() > eps * tst1 {
            m += 1;
        }
        if m > l {
            let mut iterations = 0;
            loop {
                if iterations == limit {
                    return Err(DenseError::QlIterationLimit { index: l, limit });
                }
                iterations += 1;
                total += 1;
                // Shift.
                let g = d[l];
                let mut p = (d[l + 1] - g) / (2.0 * e[l]);
                let mut r = hypot(p, 1.0);
                if p < 0.0 {
                    r = -r;
                }
                d[l] = e[l] / (p + r);
                d[l + 1] = e[l] * (p + r);
                let dl1 = d[l + 1];
                let mut h = g - d[l];
                for value in &mut d[l + 2..n] {
                    *value -= h;
                }
                shift_total += h;
                // QL sweep.
                p = d[m];
                let mut c = 1.0;
                let mut c2 = c;
                let mut c3 = c;
                let el1 = e[l + 1];
                let mut s = 0.0;
                let mut s2 = 0.0;
                for i in (l..m).rev() {
                    c3 = c2;
                    c2 = c;
                    s2 = s;
                    let g = c * e[i];
                    h = c * p;
                    r = hypot(p, e[i]);
                    e[i + 1] = s * r;
                    s = e[i] / r;
                    c = p / r;
                    p = c * d[i] - s * g;
                    d[i + 1] = h + s * (c * g + s * d[i]);
                    let (row_i, row_next) = row_pair(z, n, i, i + 1);
                    for k in 0..n {
                        let h = row_next[k];
                        row_next[k] = s * row_i[k] + c * h;
                        row_i[k] = c * row_i[k] - s * h;
                    }
                }
                p = -s * s2 * c3 * el1 * e[l] / dl1;
                e[l] = s * p;
                d[l] = c * p;
                if e[l].abs() <= eps * tst1 {
                    break;
                }
            }
        }
        d[l] += shift_total;
        e[l] = 0.0;
    }
    Ok(total)
}

/// Selection sort of d ascending, swapping the rows of z with it. Equal
/// values keep their positions, so the order is a fixed function of d.
fn sort_ascending(d: &mut [f64], z: &mut [f64], n: usize) {
    for i in 0..n.saturating_sub(1) {
        let mut k = i;
        let mut p = d[i];
        for (j, &value) in d.iter().enumerate().skip(i + 1) {
            if value < p {
                k = j;
                p = value;
            }
        }
        if k != i {
            d.swap(i, k);
            swap_rows(z, n, i, k);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> DenseOptions {
        // Test-local values; the production values are the design's (M1b).
        DenseOptions {
            infinite_mu_relative_threshold: 1e-12,
            mass_pivot_relative_threshold: 1e-12,
            resolution_floor_constant: 1.0,
            resolution_floor_multiple: 1.0,
            formed_backward_error_multiple: 64.0,
            screen_error_estimate_threshold: 1e-6,
            max_ql_iterations_per_eigenvalue: 30,
        }
    }

    #[test]
    fn non_positive_pivot_names_its_index() {
        let k = SymmetricMatrix::from_rows(&[vec![1.0, 2.0], vec![2.0, 1.0]]).unwrap();
        let m = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        assert_eq!(
            solve_dense_pencil(&k, &m, &options()),
            Err(DenseError::NonPositivePivot {
                index: 1,
                value: -3.0
            })
        );
        let singular = SymmetricMatrix::from_rows(&[
            vec![4.0, 2.0, 0.0],
            vec![2.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ])
        .unwrap();
        let m3 = SymmetricMatrix::from_diagonal(&[1.0, 1.0, 1.0]).unwrap();
        assert_eq!(
            solve_dense_pencil(&singular, &m3, &options()),
            Err(DenseError::NonPositivePivot {
                index: 1,
                value: 0.0
            })
        );
        let negative = SymmetricMatrix::from_diagonal(&[-1.0]).unwrap();
        let m1 = SymmetricMatrix::from_diagonal(&[1.0]).unwrap();
        assert_eq!(
            solve_dense_pencil(&negative, &m1, &options()),
            Err(DenseError::NonPositivePivot {
                index: 0,
                value: -1.0
            })
        );
    }

    #[test]
    fn iteration_limit_is_a_typed_error() {
        let n = 12;
        let mut rows = vec![vec![0.0; n]; n];
        for i in 0..n {
            rows[i][i] = 2.0;
            if i + 1 < n {
                rows[i][i + 1] = -1.0;
                rows[i + 1][i] = -1.0;
            }
        }
        let k = SymmetricMatrix::from_rows(&rows).unwrap();
        let m = SymmetricMatrix::from_diagonal(&vec![1.0; n]).unwrap();
        let mut opts = options();
        opts.max_ql_iterations_per_eigenvalue = 0;
        assert_eq!(
            solve_dense_pencil(&k, &m, &opts),
            Err(DenseError::QlIterationLimit { index: 0, limit: 0 })
        );
        opts.max_ql_iterations_per_eigenvalue = 30;
        assert!(solve_dense_pencil(&k, &m, &opts).is_ok());
    }

    #[test]
    fn invalid_inputs_are_refused() {
        let k = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        let m = SymmetricMatrix::from_diagonal(&[1.0]).unwrap();
        assert_eq!(
            solve_dense_pencil(&k, &m, &options()),
            Err(DenseError::DimensionMismatch {
                stiffness: 2,
                mass: 1
            })
        );
        let empty = SymmetricMatrix::from_rows(&[]).unwrap();
        assert_eq!(
            solve_dense_pencil(&empty, &empty, &options()),
            Err(DenseError::EmptySystem)
        );
        for bad in [-1e-3, 1.0, f64::NAN] {
            let opts = DenseOptions {
                infinite_mu_relative_threshold: bad,
                ..options()
            };
            assert_eq!(
                solve_dense_pencil(&k, &k, &opts),
                Err(DenseError::InvalidOption {
                    name: "infinite_mu_relative_threshold"
                })
            );
        }
        for bad in [-1e-3, 1.0, f64::INFINITY] {
            let opts = DenseOptions {
                mass_pivot_relative_threshold: bad,
                ..options()
            };
            assert_eq!(
                solve_dense_pencil(&k, &k, &opts),
                Err(DenseError::InvalidOption {
                    name: "mass_pivot_relative_threshold"
                })
            );
        }
    }

    #[test]
    fn identity_stiffness_with_diagonal_mass_is_exact() {
        // R = I and C = M exactly; a diagonal C takes tred2's zero-scale
        // branch (no arithmetic on d) and QL finds every subdiagonal zero, so
        // μ is each mass exactly and λ = 1/μ is one correctly rounded
        // division. Exact equality is therefore the expectation.
        let masses = [3.0, 0.5, 7.0, 1.25, 0.0];
        let k = SymmetricMatrix::from_diagonal(&[1.0; 5]).unwrap();
        let m = SymmetricMatrix::from_diagonal(&masses).unwrap();
        let solution = solve_dense_pencil(&k, &m, &options()).unwrap();
        assert_eq!(solution.mu, vec![7.0, 3.0, 1.25, 0.5]);
        assert_eq!(
            solution.eigenvalues,
            vec![1.0 / 7.0, 1.0 / 3.0, 1.0 / 1.25, 1.0 / 0.5]
        );
        // The massless DOF is condensed out: no μ is classified infinite.
        assert_eq!(solution.structurally_massless_count, 1);
        assert_eq!(solution.condensed_dimension, 4);
        assert_eq!(solution.nonpositive_mu_count, 0);
        assert_eq!(solution.below_threshold_mu_count, 0);
        assert!(!solution.finite_count_unresolved);
        assert_eq!(solution.ql_iterations, 0);
        // Mode i is the unit vector of the DOF carrying that mass, to sign.
        for (mode, dof) in [(0, 2), (1, 0), (2, 3), (3, 1)] {
            let v = solution.vector(mode);
            for (j, &value) in v.iter().enumerate() {
                assert_eq!(value.abs(), if j == dof { 1.0 } else { 0.0 });
            }
        }
    }

    #[test]
    fn outputs_are_validated() {
        assert_eq!(validate_outputs(&[2.0], &[0.5], &[1.0, 0.0]), Ok(()));
        assert_eq!(
            validate_outputs(&[2.0, f64::NAN], &[0.5, 1.0], &[]),
            Err(DenseError::ResultOutOfRange {
                quantity: "mu",
                index: 1
            })
        );
        assert_eq!(
            validate_outputs(&[2.0], &[f64::from_bits(1)], &[]),
            Err(DenseError::ResultOutOfRange {
                quantity: "eigenvalues",
                index: 0
            })
        );
        assert_eq!(
            validate_outputs(&[2.0], &[0.5], &[1.0, f64::INFINITY]),
            Err(DenseError::ResultOutOfRange {
                quantity: "vectors",
                index: 1
            })
        );
    }

    #[test]
    fn memory_and_work_formulas() {
        assert_eq!(DENSE_PATH_BYTES_PER_ENTRY, 32);
        assert_eq!(dense_path_peak_bytes(1000), 32_000_000);
        assert_eq!(dense_path_flop_estimate(10, 0), 9_000);
        // 9·1000 + ⌈2·1000/3⌉ = 9000 + 667.
        assert_eq!(dense_path_flop_estimate(10, 2), 9_667);
        assert_eq!(dense_path_flop_estimate(3, 1), 9 * 27 + 9);
    }

    #[test]
    fn hypot_is_exact_on_pythagorean_triples() {
        assert_eq!(hypot(3.0, 4.0), 5.0);
        assert_eq!(hypot(-5.0, 12.0), 13.0);
        assert_eq!(hypot(0.0, 0.0), 0.0);
        assert_eq!(hypot(1e300, 1e300), 1e300 * 2.0f64.sqrt());
    }
    #[test]
    fn hard_and_band_boundaries_are_inclusive_and_not_rescued() {
        let hard = 2f64.powi(-40);
        let broad = 2f64.powi(-30);
        assert_eq!(
            classify_mu(hard.next_down(), hard, broad),
            MuRegion::HardRejected
        );
        assert_eq!(classify_mu(hard, hard, broad), MuRegion::HardRejected);
        assert_eq!(classify_mu(hard.next_up(), hard, broad), MuRegion::Band);
        assert_eq!(classify_mu(broad.next_down(), hard, broad), MuRegion::Band);
        assert_eq!(classify_mu(broad, hard, broad), MuRegion::Band);
        assert_eq!(classify_mu(broad.next_up(), hard, broad), MuRegion::Direct);
        // Actual pencil: λ=1/μ is exact. Neither an exact residual nor an
        // arbitrarily permissive recovery threshold may rescue a hard μ.
        for value in [
            2f64.powi(-52).next_down(),
            2f64.powi(-52),
            2f64.powi(-52).next_up(),
        ] {
            let k = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
            let m = SymmetricMatrix::from_diagonal(&[1.0, value]).unwrap();
            let o = DenseOptions {
                infinite_mu_relative_threshold: 0.0,
                ..options()
            };
            let solution = solve_dense_pencil(&k, &m, &o).unwrap();
            assert_eq!(
                solution.mode_count(),
                if value <= 2f64.powi(-52) { 1 } else { 2 }
            );
        }
    }

    #[test]
    fn screen_thresholds_are_inclusive_and_finite() {
        let tau_f: f64 = 1e-13;
        let tau_e: f64 = 1e-6; // explicit TEST-LOCAL candidate, never a default
        let r = BandRecovery {
            mu: 1.0,
            eigenvalue: 1.0,
            backward_error: tau_f,
            error_estimate: tau_e,
        };
        assert!(screen_passes(&r, tau_f, tau_e));
        assert!(screen_passes(
            &BandRecovery {
                error_estimate: tau_e.next_down(),
                ..r.clone()
            },
            tau_f,
            tau_e
        ));
        assert!(!screen_passes(
            &BandRecovery {
                error_estimate: tau_e.next_up(),
                ..r.clone()
            },
            tau_f,
            tau_e
        ));
        assert!(screen_passes(
            &BandRecovery {
                backward_error: tau_f.next_down(),
                ..r.clone()
            },
            tau_f,
            tau_e
        ));
        assert!(!screen_passes(
            &BandRecovery {
                backward_error: tau_f.next_up(),
                ..r.clone()
            },
            tau_f,
            tau_e
        ));
        for bad in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(!screen_passes(
                &BandRecovery {
                    error_estimate: bad,
                    ..r.clone()
                },
                tau_f,
                tau_e
            ));
            assert!(!screen_passes(
                &BandRecovery {
                    backward_error: bad,
                    ..r.clone()
                },
                tau_f,
                tau_e
            ));
        }
    }

    #[test]
    fn normwise_blind_pair_is_not_an_accurate_eigenvalue() {
        // Closed form φ=(0,1), K=[[1,1],[1,3]], M=diag(1,2^-100), λ=2^52:
        // r=(1,3-2^-48). The unscaled normwise denominator is 4+2^52,
        // making η_f tiny despite an eigenvalue error of about 1.
        let k = SymmetricMatrix::from_rows(&[vec![1.0, 1.0], vec![1.0, 3.0]]).unwrap();
        let m = SymmetricMatrix::from_diagonal(&[1.0, 2f64.powi(-100)]).unwrap();
        let lambda = 2f64.powi(52);
        let normwise = (1.0 + (3.0 - 2f64.powi(-48)).powi(2)).sqrt() / (4.0 + lambda);
        assert!(normwise < 64.0 * 2.0 * (f64::EPSILON / 2.0));
        let scaling = formed_scaling(&k, &m);
        for scale in [2f64.powi(-300), 1.0, -2f64.powi(300)] {
            let mass = WideDd::from_f64(2f64.powi(-100))
                .mul_f64(scale)
                .mul_f64(scale);
            let (_, error) =
                formed_pair_errors(&k, &m, &scaling, lambda, &[0.0, scale], mass).unwrap();
            // eλ closed form sqrt(3)*sqrt(4-2^-47+2^-96/3)*2^48.
            let expected =
                3f64.sqrt() * (1.0 + (3.0 - 2f64.powi(-48)).powi(2) / 3.0).sqrt() * 2f64.powi(48);
            assert!((error / expected - 1.0).abs() < 4.0 * f64::EPSILON);
            for tau_e in [1e-9, 1e-7, 1e-6, 1e-5, 1e-3] {
                assert!(error > tau_e);
            }
        }
    }

    #[test]
    fn band_recovery_rejects_nonpositive_energy_or_mass_and_invalid_options() {
        let k = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        for mass in [-1.0, 0.0] {
            let m = SymmetricMatrix::from_diagonal(&[1.0, mass]).unwrap();
            assert!(recover_band_mode(&k, &m, &formed_scaling(&k, &m), &[0.0, 1.0]).is_none());
        }
        let m = SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap();
        assert!(recover_band_mode(&k, &m, &formed_scaling(&k, &m), &[0.0, 0.0]).is_none());
        for bad in [0.0, -1.0, f64::NAN, f64::INFINITY, 1.0] {
            let o = DenseOptions {
                screen_error_estimate_threshold: bad,
                ..options()
            };
            assert_eq!(
                solve_dense_pencil(&k, &m, &o),
                Err(DenseError::InvalidOption {
                    name: "screen_error_estimate_threshold"
                })
            );
        }
        assert_eq!(dense_screen_product_estimate(10, 3), 1200);
    }

    #[test]
    fn recovered_errors_respect_vector_normalization_and_diagonal_congruence() {
        for basis in [[1.0, 1.0], [2f64.powi(100), 2f64.powi(-100)]] {
            let k = SymmetricMatrix::from_rows(&[
                vec![2.0 * basis[0] * basis[0], basis[0] * basis[1]],
                vec![basis[0] * basis[1], 2.0 * basis[1] * basis[1]],
            ])
            .unwrap();
            let m = SymmetricMatrix::from_diagonal(&[basis[0] * basis[0], basis[1] * basis[1]])
                .unwrap();
            for amplitude in [2f64.powi(-300), 1.0, -2f64.powi(300)] {
                let phi = [amplitude / basis[0], amplitude / basis[1]];
                let r = recover_band_mode(&k, &m, &formed_scaling(&k, &m), &phi).unwrap();
                assert_eq!(r.eigenvalue, 3.0);
                assert_eq!(r.backward_error, 0.0);
                assert_eq!(r.error_estimate, 0.0);
            }
        }
    }
}
