//! M03-INTEGRITY-v1: operational checks of represented passive equations.
//! No formal inertia certificate or guaranteed forward accuracy is claimed.
use std::fmt;

pub const POLICY: &str = "M03-INTEGRITY-v1";
pub const CONDITION_ESTIMATOR: &str =
    "Hager-Higham inverse-1-norm iteration with alternating-vector safeguard";
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StiffnessContribution {
    pub row: usize,
    pub col: usize,
    pub value: f64,
}
/// All global DOFs must occur exactly once in free_dofs or prescribed.
/// Contributions, when supplied, contain BOTH triangles, before coalescence.
pub struct SymmetryEvidence<'a> {
    pub absolute_roundoff: &'a [Vec<f64>],
    pub operation_counts: &'a [Vec<usize>],
    pub basis: &'a str,
}
pub struct StructuralSystem<'a> {
    pub stiffness: &'a [Vec<f64>],
    pub force: &'a [f64],
    pub free_dofs: &'a [usize],
    pub prescribed: &'a [(usize, f64)],
    pub contributions: Option<&'a [StiffnessContribution]>,
    pub symmetry: Option<SymmetryEvidence<'a>>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum StructuralError {
    InvalidInput(&'static str),
    Range(&'static str),
    Asymmetric {
        row: usize,
        col: usize,
        relative_skew: f64,
    },
    NumericallyUnresolved {
        reason: &'static str,
        global_dof: Option<usize>,
    },
    NegativeEnergy {
        direction: Vec<f64>,
        energy: f64,
        allowance: f64,
    },
    Mechanism {
        direction: Vec<f64>,
    },
}
impl fmt::Display for StructuralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "structural integrity: {self:?}")
    }
}
impl std::error::Error for StructuralError {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolveQuality {
    Passed,
    Sensitive,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PivotEvidence {
    pub ordered_index: usize,
    pub global_dof: usize,
    pub pivot: f64,
    pub cancellation_scale: f64,
    pub operation_count: usize,
    pub screen: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ResidualRow {
    pub global_dof: usize,
    /// Physical-unit descriptive values can round in the subnormal range. The
    /// normalized fields below are the authoritative arithmetic/gate basis.
    pub residual: f64,
    pub denominator: f64,
    pub row_scale_exponent: i32,
    pub normalized_residual: f64,
    pub normalized_denominator: f64,
    pub normalized_evaluation_allowance: f64,
    pub normalization_basis: &'static str,
    pub operation_count: usize,
    pub evaluation_allowance: f64,
    pub guarded_ratio: f64,
    pub target: f64,
    pub passed: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContributionRounding {
    pub row: usize,
    pub col: usize,
    pub accumulated_high: f64,
    pub accumulated_low: f64,
    pub stored_difference_high: f64,
    pub stored_difference_low: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StructuralReport {
    pub policy: &'static str,
    pub quality: SolveQuality,
    pub factorization: &'static str,
    pub scale_exponents: Vec<i32>,
    pub pivots: Vec<PivotEvidence>,
    pub condition_estimator: &'static str,
    pub reciprocal_condition_estimate: f64,
    pub residual_rows: Vec<ResidualRow>,
    pub refinement_attempts: usize,
    /// Assembly fidelity is unknown when contributions were not supplied.
    pub contribution_audit_performed: bool,
    pub contribution_rounding: Vec<ContributionRounding>,
    pub assembly_relative_perturbation_estimate: f64,
    pub assembly_amplification_estimate: f64,
    pub symmetry_projection_performed: bool,
    pub maximum_scaled_skew: f64,
    pub symmetry_basis: Option<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StructuralSolution {
    pub displacements: Vec<f64>,
    pub report: StructuralReport,
}
#[derive(Debug, Clone)]
pub struct PreparedSystem {
    pub matrix: Vec<Vec<f64>>,
    pub rhs: Vec<f64>,
    pub scale_exponents: Vec<i32>,
    pub assembly_relative_perturbation_estimate: f64,
    pub contribution_rounding: Vec<ContributionRounding>,
    pub symmetry_projection_performed: bool,
    pub maximum_scaled_skew: f64,
    pub symmetry_basis: Option<String>,
}
pub fn gamma(operations: usize) -> f64 {
    let x = operations as f64 * (f64::EPSILON / 2.0);
    x / (1.0 - x)
}
fn unresolved(reason: &'static str, global_dof: Option<usize>) -> StructuralError {
    StructuralError::NumericallyUnresolved { reason, global_dof }
}
/// Power-of-two scale in bounded steps; reject underflow/overflow instead of erasing terms.
pub fn radix_scale(mut value: f64, mut exponent: i32) -> Result<f64, StructuralError> {
    if !value.is_finite() {
        return Err(StructuralError::Range("nonfinite radix input"));
    }
    if value == 0.0 {
        return Ok(value);
    }
    while exponent != 0 {
        let step = exponent.clamp(-512, 512);
        value *= 2.0_f64.powi(step);
        exponent -= step;
        if !value.is_finite() || value == 0.0 || value.is_subnormal() {
            return Err(StructuralError::Range("radix scaling loses normal range"));
        }
    }
    Ok(value)
}
fn binary_exponent(value: f64) -> i32 {
    let bits = value.abs().to_bits();
    let e = ((bits >> 52) & 0x7ff) as i32;
    if e != 0 {
        e - 1023
    } else {
        -1074 + (63 - (bits & ((1_u64 << 52) - 1)).leading_zeros() as i32)
    }
}
/// Normal-arithmetic relative-error screens deliberately reject subnormal results.
pub fn checked_product(a: f64, b: f64) -> Result<f64, StructuralError> {
    let x = a * b;
    if !x.is_finite() || (a != 0.0 && b != 0.0 && (x == 0.0 || x.is_subnormal())) {
        Err(StructuralError::Range("product overflow or underflow"))
    } else {
        Ok(x)
    }
}
pub fn checked_quotient(a: f64, b: f64) -> Result<f64, StructuralError> {
    let result = a / b;
    if !result.is_finite() || (a != 0.0 && (result == 0.0 || result.is_subnormal())) {
        Err(StructuralError::Range("division overflow or underflow"))
    } else {
        Ok(result)
    }
}
pub fn checked_value(x: f64) -> Result<f64, StructuralError> {
    if !x.is_finite() || x.is_subnormal() {
        Err(StructuralError::Range("arithmetic outside normal range"))
    } else {
        Ok(x)
    }
}
fn validate(system: &StructuralSystem<'_>) -> Result<(), StructuralError> {
    let n = system.force.len();
    if system.stiffness.len() != n || system.stiffness.iter().any(|r| r.len() != n) {
        return Err(StructuralError::InvalidInput("matrix dimensions"));
    }
    if system
        .force
        .iter()
        .chain(system.stiffness.iter().flatten())
        .any(|x| !x.is_finite())
    {
        return Err(StructuralError::InvalidInput(
            "nonfinite original equations",
        ));
    }
    if let Some(evidence) = &system.symmetry {
        if evidence.basis.is_empty()
            || evidence.absolute_roundoff.len() != n
            || evidence.operation_counts.len() != n
            || evidence
                .absolute_roundoff
                .iter()
                .any(|r| r.len() != n || r.iter().any(|v| !v.is_finite() || *v < 0.0))
            || evidence.operation_counts.iter().any(|r| r.len() != n)
        {
            return Err(StructuralError::InvalidInput("symmetry provenance"));
        }
    }
    // Audit BOTH triangles of full retained equations, including coupling to prescribed DOFs.
    for i in 0..n {
        for j in 0..i {
            let a = system.stiffness[i][j];
            let b = system.stiffness[j][i];
            if a == b {
                continue;
            }
            let scale = a.abs().max(b.abs());
            let skew = (a / scale - b / scale).abs();
            let allowed = system.symmetry.as_ref().map_or(0.0, |e| {
                e.absolute_roundoff[i][j] / scale + e.absolute_roundoff[j][i] / scale
            });
            if skew > allowed {
                return Err(StructuralError::Asymmetric {
                    row: i,
                    col: j,
                    relative_skew: skew,
                });
            }
        }
    }
    let mut seen = vec![false; n];
    for &i in system.free_dofs {
        if i >= n || seen[i] {
            return Err(StructuralError::InvalidInput("free map"));
        }
        seen[i] = true;
    }
    for &(i, v) in system.prescribed {
        if i >= n || seen[i] || !v.is_finite() {
            return Err(StructuralError::InvalidInput("prescribed map"));
        }
        seen[i] = true;
    }
    if seen.iter().any(|x| !*x) {
        return Err(StructuralError::InvalidInput("incomplete constraint map"));
    }
    Ok(())
}
/// Error-free addition transform in normal arithmetic. Low words expose assembly loss.
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let z = s - a;
    (s, (a - (s - z)) + (b - z))
}
fn audit_contributions(
    system: &StructuralSystem<'_>,
    exponents: &[i32],
) -> Result<(f64, Vec<ContributionRounding>), StructuralError> {
    let Some(entries) = system.contributions else {
        return Ok((0.0, Vec::new()));
    };
    let n = system.force.len();
    let mut sums = vec![vec![(0.0, 0.0); n]; n];
    for entry in entries {
        if entry.row >= n || entry.col >= n || !entry.value.is_finite() {
            return Err(StructuralError::InvalidInput("contribution"));
        }
        let pair = &mut sums[entry.row][entry.col];
        let (s, e) = two_sum(pair.0, entry.value);
        if entry.row == entry.col && entry.value > 0.0 && s == pair.0 {
            return Err(unresolved(
                "positive diagonal contribution absorbed by assembly; stabilization unresolved",
                Some(entry.row),
            ));
        }
        pair.0 = checked_value(s)?;
        pair.1 = checked_value(pair.1 + e)?;
    }
    let mut rounding = Vec::new();
    for (i, row) in sums.iter().enumerate() {
        for (j, &(hi, lo)) in row.iter().enumerate() {
            let (difference, tail) = two_sum(hi, -system.stiffness[i][j]);
            if difference != 0.0 || tail != 0.0 || lo != 0.0 {
                rounding.push(ContributionRounding {
                    row: i,
                    col: j,
                    accumulated_high: hi,
                    accumulated_low: lo,
                    stored_difference_high: difference,
                    stored_difference_low: tail,
                });
            }
        }
    }
    let mut perturbation_norm: f64 = 0.0;
    let mut matrix_norm: f64 = 0.0;
    for (c, &j) in system.free_dofs.iter().enumerate() {
        let mut delta_column = 0.0;
        let mut matrix_column = 0.0;
        for (r, &i) in system.free_dofs.iter().enumerate() {
            let (hi, lo) = sums[i][j];
            let (difference, tail) = two_sum(hi, -system.stiffness[i][j]);
            // Keep low words separate through scaling and absolute norm accumulation.
            for part in [difference, lo, tail] {
                delta_column = checked_value(
                    delta_column + radix_scale(part, exponents[r] + exponents[c])?.abs(),
                )?;
            }
            matrix_column = checked_value(
                matrix_column
                    + radix_scale(system.stiffness[i][j], exponents[r] + exponents[c])?.abs(),
            )?;
        }
        perturbation_norm = perturbation_norm.max(delta_column);
        matrix_norm = matrix_norm.max(matrix_column);
    }
    Ok((
        if matrix_norm == 0.0 {
            0.0
        } else {
            perturbation_norm / matrix_norm
        },
        rounding,
    ))
}
/// Nearby radix diagonal equilibration: A = T K_ff T, b = T f_reduced, u_f=T y.
/// Ideal work-conjugate length/energy scaling cancels in total diagonal equilibration.
pub fn prepare_structural(
    system: &StructuralSystem<'_>,
) -> Result<PreparedSystem, StructuralError> {
    validate(system)?;
    let n = system.free_dofs.len();
    let mut exponents = Vec::with_capacity(n);
    for &i in system.free_dofs {
        let d = system.stiffness[i][i];
        if d < 0.0 {
            let mut direction = vec![0.0; system.force.len()];
            direction[i] = 1.0;
            return Err(StructuralError::NegativeEnergy {
                direction,
                energy: d,
                allowance: 0.0,
            });
        }
        if d == 0.0 {
            return Err(unresolved(
                "zero original diagonal; no physical nullity inferred",
                Some(i),
            ));
        }
        exponents.push(-binary_exponent(d).div_euclid(2));
    }
    let mut a = vec![vec![0.0; n]; n];
    let mut rhs = vec![0.0; n];
    for (r, &i) in system.free_dofs.iter().enumerate() {
        for (c, &j) in system.free_dofs.iter().enumerate() {
            a[r][c] = radix_scale(system.stiffness[i][j], exponents[r] + exponents[c])?;
        }
        let mut b = system.force[i];
        for &(j, u) in system.prescribed {
            b = checked_value(b - checked_product(system.stiffness[i][j], u)?)?;
        }
        rhs[r] = radix_scale(b, exponents[r])?;
    }
    let mut projection = false;
    let mut max_skew: f64 = 0.0;
    if let Some(evidence) = &system.symmetry {
        let full = system.force.len();
        if evidence.basis.is_empty()
            || evidence.absolute_roundoff.len() != full
            || evidence.operation_counts.len() != full
            || evidence
                .absolute_roundoff
                .iter()
                .any(|r| r.len() != full || r.iter().any(|v| !v.is_finite() || *v < 0.0))
            || evidence.operation_counts.iter().any(|r| r.len() != full)
        {
            return Err(StructuralError::InvalidInput("symmetry provenance"));
        }
    }
    for i in 0..n {
        for j in 0..i {
            let skew = checked_value(a[i][j] - a[j][i])?.abs();
            max_skew = max_skew.max(skew);
            if skew == 0.0 {
                continue;
            }
            let allowed = if let Some(e) = &system.symmetry {
                let gi = system.free_dofs[i];
                let gj = system.free_dofs[j];
                checked_value(
                    radix_scale(e.absolute_roundoff[gi][gj], exponents[i] + exponents[j])?
                        + radix_scale(e.absolute_roundoff[gj][gi], exponents[i] + exponents[j])?,
                )?
            } else {
                0.0
            };
            if skew > allowed {
                return Err(StructuralError::Asymmetric {
                    row: system.free_dofs[i],
                    col: system.free_dofs[j],
                    relative_skew: skew / a[i][j].abs().max(a[j][i].abs()),
                });
            }
            let average = a[i][j] / 2.0 + a[j][i] / 2.0;
            a[i][j] = average;
            a[j][i] = average;
            projection = true;
        }
    }
    let (perturbation, rounding) = audit_contributions(system, &exponents)?;
    Ok(PreparedSystem {
        matrix: a,
        rhs,
        scale_exponents: exponents,
        assembly_relative_perturbation_estimate: perturbation,
        contribution_rounding: rounding,
        symmetry_projection_performed: projection,
        maximum_scaled_skew: max_skew,
        symmetry_basis: system.symmetry.as_ref().map(|e| e.basis.to_owned()),
    })
}
/// Product divided by 2^row_exponent, without forming an out-of-range physical product.
/// Mantissas and subsequent radix shifts are exact when the result stays normal;
/// one ordinary multiplication supplies the rounded normalized term.
fn normalized_product(a: f64, b: f64, row_exponent: i32) -> Result<f64, StructuralError> {
    if a == 0.0 || b == 0.0 {
        return Ok(0.0);
    }
    let ea = binary_exponent(a);
    let eb = binary_exponent(b);
    let ma = radix_scale(a, -ea)?;
    let mb = radix_scale(b, -eb)?;
    radix_scale(checked_product(ma, mb)?, ea + eb - row_exponent)
}
/// Descriptive recovery only. A subnormal/zero rounded physical value does not
/// replace the retained normalized value and is never used for correction solves.
fn physical_residual_record(mut value: f64, mut exponent: i32) -> Result<f64, StructuralError> {
    while exponent != 0 {
        let step = exponent.clamp(-512, 512);
        value *= 2.0_f64.powi(step);
        exponent -= step;
        if !value.is_finite() {
            return Err(StructuralError::Range("physical residual record overflow"));
        }
    }
    Ok(value)
}
/// Original GLOBAL equations, independent of factor/order/reduced RHS. Constrained rows are reactions.
pub fn evaluate_original_residual(
    system: &StructuralSystem<'_>,
    u: &[f64],
) -> Result<Vec<ResidualRow>, StructuralError> {
    validate(system)?;
    if u.len() != system.force.len() || u.iter().any(|x| !x.is_finite()) {
        return Err(StructuralError::InvalidInput("displacement vector"));
    }
    if system.prescribed.iter().any(|&(i, v)| u[i] != v) {
        return Err(StructuralError::InvalidInput(
            "prescribed displacement incompatibility",
        ));
    }
    let mut rows = Vec::new();
    for &i in system.free_dofs {
        let mut row_exponent = if system.force[i] == 0.0 {
            i32::MIN
        } else {
            binary_exponent(system.force[i])
        };
        for (&k, &x) in system.stiffness[i].iter().zip(u) {
            if k != 0.0 && x != 0.0 {
                row_exponent = row_exponent.max(binary_exponent(k) + binary_exponent(x) + 1);
            }
        }
        if row_exponent == i32::MIN {
            row_exponent = 0;
        }
        let mut r = -radix_scale(system.force[i], -row_exponent)?;
        let mut d = r.abs();
        let mut count = 0;
        for (&k, &x) in system.stiffness[i].iter().zip(u) {
            if k == 0.0 {
                continue;
            }
            count += 1;
            let p = normalized_product(k, x, row_exponent)?;
            r = checked_value(r + p)?;
            d = checked_value(d + p.abs())?;
        }
        let operations = 2 * count + 2;
        let g = gamma(operations);
        let allowance = g * d / (1.0 - g);
        let guarded = if d == 0.0 {
            if r == 0.0 {
                0.0
            } else {
                f64::INFINITY
            }
        } else {
            (r.abs() / d + g / (1.0 - g)) * (1.0 + g) * (1.0 + 4.0 * f64::EPSILON)
        };
        let target = 64.0 * g;
        rows.push(ResidualRow {
            global_dof: i,
            residual: physical_residual_record(r, row_exponent)?,
            denominator: physical_residual_record(d, row_exponent)?,
            row_scale_exponent: row_exponent,
            normalized_residual: r,
            normalized_denominator: d,
            normalized_evaluation_allowance: allowance,
            normalization_basis:
                "radix mantissas; one multiply/add per term; physical records descriptive",
            operation_count: operations,
            evaluation_allowance: physical_residual_record(allowance, row_exponent)?,
            guarded_ratio: guarded,
            target,
            passed: guarded <= target,
        });
    }
    Ok(rows)
}
pub fn screen_pivot(
    ordered_index: usize,
    global_dof: usize,
    pivot: f64,
    cancellation_scale: f64,
    operation_count: usize,
) -> Result<PivotEvidence, StructuralError> {
    checked_value(pivot)?;
    checked_value(cancellation_scale)?;
    let screen = 64.0 * gamma(operation_count) * cancellation_scale;
    if pivot <= screen {
        return Err(unresolved(
            "nonpositive or cancellation-unresolved structural pivot",
            Some(global_dof),
        ));
    }
    Ok(PivotEvidence {
        ordered_index,
        global_dof,
        pivot,
        cancellation_scale,
        operation_count,
        screen,
    })
}
/// Hager iteration for symmetric inverse with Higham alternating-vector safeguard.
/// Solve failures/range loss propagate; this estimate is not a certified upper bound.
pub fn estimate_rcond<F>(a: &[Vec<f64>], solve: &F) -> Result<f64, StructuralError>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
{
    let n = a.len();
    if a.iter()
        .any(|row| row.len() != n || row.iter().any(|v| !v.is_finite()))
    {
        return Err(StructuralError::InvalidInput("condition matrix"));
    }
    if n == 0 {
        return Ok(1.0);
    }
    let mut norm: f64 = 0.0;
    for j in 0..n {
        let mut sum = 0.0;
        for row in a {
            sum = checked_value(sum + row[j].abs())?;
        }
        norm = norm.max(sum);
    }
    let mut x = vec![1.0 / n as f64; n];
    let mut estimate: f64 = 0.0;
    let mut previous = n;
    for _ in 0..5 {
        let y = solve(&x)?;
        if y.len() != n || y.iter().any(|v| !v.is_finite()) {
            return Err(StructuralError::InvalidInput("condition solve output"));
        }
        let current = checked_value(y.iter().map(|v| v.abs()).sum())?;
        if current <= estimate && estimate != 0.0 {
            break;
        }
        estimate = current;
        let signs: Vec<f64> = y
            .iter()
            .map(|&v| if v >= 0.0 { 1.0 } else { -1.0 })
            .collect();
        let z = solve(&signs)?;
        if z.len() != n || z.iter().any(|v| !v.is_finite()) {
            return Err(StructuralError::InvalidInput(
                "condition transpose solve output",
            ));
        }
        let j = (0..n)
            .max_by(|&i, &j| z[i].abs().total_cmp(&z[j].abs()))
            .unwrap();
        let dot: f64 = z.iter().zip(&x).map(|(a, b)| a * b).sum();
        checked_value(dot)?;
        if z[j].abs() <= dot || j == previous {
            break;
        }
        previous = j;
        x.fill(0.0);
        x[j] = 1.0;
    }
    let alt: Vec<f64> = (0..n)
        .map(|i| {
            if n == 1 {
                1.0
            } else {
                (if i % 2 == 0 { 1.0 } else { -1.0 }) * (1.0 + i as f64 / (n - 1) as f64)
            }
        })
        .collect();
    let y = solve(&alt)?;
    if y.len() != n || y.iter().any(|v| !v.is_finite()) {
        return Err(StructuralError::InvalidInput("condition safeguard output"));
    }
    let alt_est = checked_value(y.iter().map(|v| v.abs()).sum::<f64>() * 2.0 / (3.0 * n as f64))?;
    estimate = estimate.max(alt_est);
    let rcond = 1.0 / checked_product(norm, estimate)?;
    if !rcond.is_finite() || rcond <= f64::EPSILON {
        return Err(unresolved(
            "scaled condition estimate at working-precision boundary",
            None,
        ));
    }
    Ok(rcond.min(1.0))
}
/// Common completion for EVERY structural backend; no finite-LU bypass.
pub fn finish_structural<F>(
    system: &StructuralSystem<'_>,
    prepared: &PreparedSystem,
    pivots: Vec<PivotEvidence>,
    factorization: &'static str,
    solve: F,
) -> Result<StructuralSolution, StructuralError>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
{
    validate(system)?;
    let n = system.free_dofs.len();
    if prepared.matrix.len() != n || prepared.rhs.len() != n || prepared.scale_exponents.len() != n
    {
        return Err(StructuralError::InvalidInput("prepared dimensions"));
    }
    let rcond = estimate_rcond(&prepared.matrix, &solve)?;
    let amplification = prepared.assembly_relative_perturbation_estimate / rcond;
    if !amplification.is_finite() || amplification >= 1.0 {
        return Err(unresolved(
            "assembly perturbation amplification unresolved",
            None,
        ));
    }
    let mut y = solve(&prepared.rhs)?;
    if y.len() != n || y.iter().any(|v| !v.is_finite()) {
        return Err(StructuralError::InvalidInput("structural backend output"));
    }
    let mut attempts = 0;
    let mut prior = f64::INFINITY;
    loop {
        let mut u = vec![0.0; system.force.len()];
        for &(i, v) in system.prescribed {
            u[i] = v;
        }
        for (r, &i) in system.free_dofs.iter().enumerate() {
            u[i] = radix_scale(y[r], prepared.scale_exponents[r])?;
        }
        let residual = evaluate_original_residual(system, &u)?;
        let worst = residual
            .iter()
            .map(|r| r.guarded_ratio / r.target)
            .fold(0.0, f64::max);
        if residual.iter().all(|r| r.passed) {
            return Ok(StructuralSolution {
                displacements: u,
                report: StructuralReport {
                    policy: POLICY,
                    quality: if rcond < f64::EPSILON.sqrt() {
                        SolveQuality::Sensitive
                    } else {
                        SolveQuality::Passed
                    },
                    factorization,
                    scale_exponents: prepared.scale_exponents.clone(),
                    pivots,
                    condition_estimator: CONDITION_ESTIMATOR,
                    reciprocal_condition_estimate: rcond,
                    residual_rows: residual,
                    refinement_attempts: attempts,
                    contribution_audit_performed: system.contributions.is_some(),
                    contribution_rounding: prepared.contribution_rounding.clone(),
                    assembly_relative_perturbation_estimate: prepared
                        .assembly_relative_perturbation_estimate,
                    assembly_amplification_estimate: amplification,
                    symmetry_projection_performed: prepared.symmetry_projection_performed,
                    maximum_scaled_skew: prepared.maximum_scaled_skew,
                    symmetry_basis: prepared.symmetry_basis.clone(),
                },
            });
        }
        if attempts == 3 || worst >= prior {
            return Err(unresolved(
                "original-equation residual failed after bounded refinement",
                None,
            ));
        }
        prior = worst;
        let mut correction = Vec::new();
        for (i, row) in residual.iter().enumerate() {
            correction.push(radix_scale(
                -row.normalized_residual,
                row.row_scale_exponent + prepared.scale_exponents[i],
            )?);
        }
        let delta = solve(&correction)?;
        if delta.len() != n || delta.iter().any(|v| !v.is_finite()) {
            return Err(StructuralError::InvalidInput("refinement backend output"));
        }
        for (v, d) in y.iter_mut().zip(delta) {
            *v = checked_value(*v + d)?;
        }
        attempts += 1;
    }
}
#[derive(Debug, Clone)]
pub struct CholeskyFactor {
    lower: Vec<Vec<f64>>,
}
impl CholeskyFactor {
    pub fn solve(&self, rhs: &[f64]) -> Result<Vec<f64>, StructuralError> {
        let n = self.lower.len();
        if rhs.len() != n {
            return Err(StructuralError::InvalidInput("triangular RHS"));
        }
        let mut x = rhs.to_vec();
        for i in 0..n {
            for j in 0..i {
                x[i] = checked_value(x[i] - checked_product(self.lower[i][j], x[j])?)?;
            }
            x[i] = checked_quotient(x[i], self.lower[i][i])?;
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                x[i] = checked_value(x[i] - checked_product(self.lower[j][i], x[j])?)?;
            }
            x[i] = checked_quotient(x[i], self.lower[i][i])?;
        }
        Ok(x)
    }
}
pub fn factor_structural_cholesky(
    prepared: &PreparedSystem,
    free_dofs: &[usize],
) -> Result<(CholeskyFactor, Vec<PivotEvidence>), StructuralError> {
    let n = prepared.matrix.len();
    if free_dofs.len() != n
        || prepared
            .matrix
            .iter()
            .any(|row| row.len() != n || row.iter().any(|v| !v.is_finite()))
    {
        return Err(StructuralError::InvalidInput("Cholesky matrix/free map"));
    }
    for i in 0..n {
        for j in 0..i {
            if prepared.matrix[i][j] != prepared.matrix[j][i] {
                return Err(StructuralError::InvalidInput(
                    "Cholesky requires audited symmetric matrix",
                ));
            }
        }
    }
    let mut l = vec![vec![0.0; n]; n];
    let mut evidence = Vec::new();
    for i in 0..n {
        for j in 0..=i {
            let mut sum = prepared.matrix[i][j];
            let mut scale = sum.abs();
            for k in 0..j {
                let term = checked_product(l[i][k], l[j][k])?;
                sum = checked_value(sum - term)?;
                scale = checked_value(scale + term.abs())?;
            }
            if i == j {
                evidence.push(screen_pivot(i, free_dofs[i], sum, scale, 2 * j + 2)?);
                l[i][j] = sum.sqrt();
            } else {
                l[i][j] = checked_quotient(sum, l[j][j])?;
            }
        }
    }
    Ok((CholeskyFactor { lower: l }, evidence))
}
pub fn solve_structural_dense(
    system: &StructuralSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    let prepared = prepare_structural(system)?;
    let (factor, pivots) = match factor_structural_cholesky(&prepared, system.free_dofs) {
        Ok(value) => value,
        Err(error) => return Err(negative_pair_witness(system, &prepared)?.unwrap_or(error)),
    };
    finish_structural(
        system,
        &prepared,
        pivots,
        "positive dense Cholesky",
        |rhs| factor.solve(rhs),
    )
}

#[derive(Debug, Clone)]
pub struct TransformationRoundoff {
    pub absolute_roundoff: crate::Matrix12,
    pub operation_counts: [[usize; 12]; 12],
    pub basis: &'static str,
}
/// Estimated normal-arithmetic allowance for existing two-stage T^T (K T), each dot
/// has 12 products and 12 additions. The represented local K and T are the basis;
/// constitutive/orientation formation errors and assembly are NOT included.
pub fn transform_roundoff(
    local: &crate::Matrix12,
    transform: &crate::Matrix12,
) -> Result<TransformationRoundoff, StructuralError> {
    let mut temp = [[0.0; 12]; 12];
    let mut magnitudes = [[0.0; 12]; 12];
    for i in 0..12 {
        for j in 0..12 {
            for k in 0..12 {
                let term = checked_product(local[i][k], transform[k][j])?;
                temp[i][j] = checked_value(temp[i][j] + term)?;
                magnitudes[i][j] = checked_value(magnitudes[i][j] + term.abs())?;
            }
        }
    }
    let g = gamma(24);
    let mut bounds = [[0.0; 12]; 12];
    for i in 0..12 {
        for j in 0..12 {
            let mut inherited = 0.0;
            let mut second = 0.0;
            for k in 0..12 {
                inherited = checked_value(
                    inherited + checked_product(transform[k][i].abs(), magnitudes[k][j])?,
                )?;
                second = checked_value(
                    second + checked_product(transform[k][i].abs(), temp[k][j].abs())?,
                )?;
            }
            // Extra final-operation reserve; operational estimate, not outward-rounded enclosure.
            bounds[i][j] =
                checked_value(g * (inherited / (1.0 - g) + second) * (1.0 + 64.0 * f64::EPSILON))?;
        }
    }
    Ok(TransformationRoundoff {absolute_roundoff:bounds,operation_counts:[[48;12];12],basis:"represented T^T(KT); two sequential 12-product/12-add dots; gamma(24) per stage; normal-range estimate"})
}

/// Validate an independently proposed scaled direction before a negative-energy verdict.
/// A failed pivot by itself never calls this a physical mechanism or exact inertia.
pub fn verify_negative_direction(
    system: &StructuralSystem<'_>,
    prepared: &PreparedSystem,
    direction: &[f64],
) -> Result<Option<StructuralError>, StructuralError> {
    if direction.len() != prepared.matrix.len() || direction.iter().any(|v| !v.is_finite()) {
        return Err(StructuralError::InvalidInput("negative direction"));
    }
    let mut energy = 0.0;
    let mut magnitude = 0.0;
    let mut terms = 0;
    for i in 0..direction.len() {
        for j in 0..direction.len() {
            if prepared.matrix[i][j] == 0.0 || direction[i] == 0.0 || direction[j] == 0.0 {
                continue;
            }
            let original = radix_scale(
                system.stiffness[system.free_dofs[i]][system.free_dofs[j]],
                prepared.scale_exponents[i] + prepared.scale_exponents[j],
            )?;
            let term = checked_product(checked_product(direction[i], original)?, direction[j])?;
            energy = checked_value(energy + term)?;
            magnitude = checked_value(magnitude + term.abs())?;
            terms += 1;
        }
    }
    let allowance = 64.0 * gamma(3 * terms + 2) * magnitude;
    if energy < -allowance {
        let mut mapped = vec![0.0; system.force.len()];
        for (i, &global) in system.free_dofs.iter().enumerate() {
            mapped[global] = radix_scale(direction[i], prepared.scale_exponents[i])?;
        }
        Ok(Some(StructuralError::NegativeEnergy {
            direction: mapped,
            energy,
            allowance,
        }))
    } else {
        Ok(None)
    }
}
/// Limited diagnostic search; failure to find a direction is unresolved, never positive evidence.
pub fn negative_pair_witness(
    system: &StructuralSystem<'_>,
    prepared: &PreparedSystem,
) -> Result<Option<StructuralError>, StructuralError> {
    let n = prepared.matrix.len();
    for i in 0..n {
        for j in 0..i {
            let mut v = vec![0.0; n];
            v[i] = 1.0;
            v[j] = if prepared.matrix[i][j] >= 0.0 {
                -1.0
            } else {
                1.0
            };
            if let Some(witness) = verify_negative_direction(system, prepared, &v)? {
                return Ok(Some(witness));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn system<'a>(
        k: &'a [Vec<f64>],
        f: &'a [f64],
        free: &'a [usize],
        bc: &'a [(usize, f64)],
    ) -> StructuralSystem<'a> {
        StructuralSystem {
            stiffness: k,
            force: f,
            free_dofs: free,
            prescribed: bc,
            contributions: None,
            symmetry: None,
        }
    }
    #[test]
    fn soft_diagonal_and_unit_congruence_are_not_mechanisms() {
        for scale in [1e-300, 1e-20, 1.0, 1e200] {
            let k = vec![vec![2.0 * scale, 0.0], vec![0.0, 3.0 * scale]];
            let f = [4.0 * scale, -9.0 * scale];
            let free = [0, 1];
            let result = solve_structural_dense(&system(&k, &f, &free, &[])).unwrap();
            assert!((result.displacements[0] - 2.0).abs() < 1e-14);
            assert!((result.displacements[1] + 3.0).abs() < 1e-14);
            assert_eq!(result.report.quality, SolveQuality::Passed);
        }
    }
    #[test]
    fn invalid_energy_zero_load_and_skew_are_not_solved() {
        let free = [0, 1];
        let f = [1.0, 1.0];
        let negative = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
        assert!(matches!(
            solve_structural_dense(&system(&negative, &f, &free, &[])),
            Err(StructuralError::NegativeEnergy { .. })
        ));
        let singular = vec![vec![1.0, -1.0], vec![-1.0, 1.0]];
        assert!(matches!(
            solve_structural_dense(&system(&singular, &[0.0, 0.0], &free, &[])),
            Err(StructuralError::NumericallyUnresolved { .. })
        ));
        let skew = vec![vec![2.0, 0.1], vec![0.2, 2.0]];
        assert!(matches!(
            solve_structural_dense(&system(&skew, &f, &free, &[])),
            Err(StructuralError::Asymmetric { .. })
        ));
    }
    #[test]
    fn original_residual_kills_wrong_solution_wrong_matrix_and_missing_load() {
        let k = vec![vec![1000.0]];
        let f = [1000.0];
        let free = [0];
        let original = system(&k, &f, &free, &[]);
        let wrong = evaluate_original_residual(&original, &[1.01]).unwrap();
        assert!((wrong[0].residual - 10.0).abs() < 1e-12);
        assert!(!wrong[0].passed);
        let wrong_k = vec![vec![1000.0 / 1.01]];
        let wrong_result = solve_structural_dense(&system(&wrong_k, &f, &free, &[])).unwrap();
        assert!(
            !evaluate_original_residual(&original, &wrong_result.displacements).unwrap()[0].passed
        );
        assert!(!evaluate_original_residual(&original, &[0.0]).unwrap()[0].passed);
    }
    #[test]
    fn prescribed_and_constrained_load_semantics() {
        let k = vec![
            vec![1.0, -1.0, 0.0],
            vec![-1.0, 2.0, -1.0],
            vec![0.0, -1.0, 1.0],
        ];
        let f = [8.0, 0.0, 9.0];
        let free = [1];
        let bc = [(0, 1e-4), (2, 0.0)];
        let s = system(&k, &f, &free, &bc);
        let solved = solve_structural_dense(&s).unwrap();
        assert!((solved.displacements[1] - 5e-5).abs() < 1e-19);
        assert!(!evaluate_original_residual(&s, &[1e-4, 0.0, 0.0]).unwrap()[0].passed);
        assert!(evaluate_original_residual(&s, &[0.0, 0.0, 0.0]).is_err());
        let all = [(0, 0.0), (1, 0.0), (2, 0.0)];
        let fixed = solve_structural_dense(&system(&k, &f, &[], &all)).unwrap();
        assert!(fixed.report.residual_rows.is_empty());
        assert!(solve_structural_dense(&system(&k, &f, &[1], &[(0, 0.0)])).is_err());
    }
    #[test]
    fn np_b_banded_family_does_not_require_comparison_inverse_certificate() {
        for n in [8, 64, 80] {
            let mut c = vec![vec![0.0; n]; n];
            for i in 0..n {
                c[i][i] = 1.0;
                if i > 0 {
                    c[i][i - 1] = 0.75000001;
                }
                if i > 1 {
                    c[i][i - 2] = 0.75;
                }
            }
            let mut k = vec![vec![0.0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    k[i][j] = (0..n).map(|q| c[i][q] * c[j][q]).sum();
                }
            }
            let expected: Vec<f64> = (0..n)
                .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
                .collect();
            let f: Vec<f64> = k
                .iter()
                .map(|r| r.iter().zip(&expected).map(|(a, b)| a * b).sum())
                .collect();
            let free: Vec<usize> = (0..n).collect();
            let result = solve_structural_dense(&system(&k, &f, &free, &[])).unwrap();
            assert_eq!(result.report.quality, SolveQuality::Passed);
            for (a, b) in result.displacements.iter().zip(expected) {
                assert!((a - b).abs() < 1e-12);
            }
        }
    }
    #[test]
    fn lost_stabilizer_retains_unresolved_instead_of_physical_mechanism() {
        let a = 687800.0 * std::f64::consts::PI;
        let k = 1e-12;
        let matrix = vec![vec![a + k, -a], vec![-a, a]];
        let f = [0.0, 1e-16];
        let free = [0, 1];
        let entries = [
            StiffnessContribution {
                row: 0,
                col: 0,
                value: a,
            },
            StiffnessContribution {
                row: 0,
                col: 0,
                value: k,
            },
            StiffnessContribution {
                row: 0,
                col: 1,
                value: -a,
            },
            StiffnessContribution {
                row: 1,
                col: 0,
                value: -a,
            },
            StiffnessContribution {
                row: 1,
                col: 1,
                value: a,
            },
        ];
        let mut s = system(&matrix, &f, &free, &[]);
        s.contributions = Some(&entries);
        assert!(matches!(
            solve_structural_dense(&s),
            Err(StructuralError::NumericallyUnresolved { .. })
        ));
    }
    #[test]
    fn generic_nonsymmetric_api_is_preserved() {
        let k = vec![vec![1.0, 2.0], vec![0.0, 1.0]];
        assert_eq!(crate::solve_dense(&k, &[3.0, 1.0]).unwrap(), vec![1.0, 1.0]);
        assert!(solve_structural_dense(&system(&k, &[3.0, 1.0], &[0, 1], &[])).is_err());
    }
    #[test]
    fn nonfinite_and_unrepresentable_range_block() {
        let k = vec![vec![f64::NAN]];
        assert!(solve_structural_dense(&system(&k, &[1.0], &[0], &[])).is_err());
        let k = vec![vec![1e-300]];
        assert!(solve_structural_dense(&system(&k, &[1e300], &[0], &[])).is_err());
    }
    #[test]
    fn normalized_residual_retains_soft_physical_cancellation() {
        let matrix = vec![vec![2e-300]];
        let force = [4e-300];
        let free = [0];
        let s = system(&matrix, &force, &free, &[]);
        let rows = evaluate_original_residual(&s, &[1.9999999999999996]).unwrap();
        assert!(rows[0].residual.is_subnormal());
        assert!(rows[0].normalized_residual.is_normal());
        assert!(rows[0].passed);
        assert_eq!(rows[0].operation_count, 4);
        assert!(!evaluate_original_residual(&s, &[2.02]).unwrap()[0].passed);
    }
    #[test]
    fn public_factor_dimensions_are_errors_not_panics() {
        let matrix = vec![vec![2.0]];
        let s = system(&matrix, &[1.0], &[0], &[]);
        let mut prepared = prepare_structural(&s).unwrap();
        assert!(factor_structural_cholesky(&prepared, &[]).is_err());
        prepared.matrix[0].clear();
        assert!(factor_structural_cholesky(&prepared, &[0]).is_err());
        assert!(estimate_rcond(&[vec![1.0]], &|_| Ok(vec![])).is_err());
    }
}
