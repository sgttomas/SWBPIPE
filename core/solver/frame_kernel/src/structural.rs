//! M03-INTEGRITY-v1: operational checks of represented passive equations.
//! No formal inertia certificate or guaranteed forward accuracy is claimed.
pub mod exact_boundary;
mod formation_check;
mod retained;

pub use formation_check::{
    CurvedFormation, FormationCheck, FormationCheckReason, FormationSource, FORMATION_CRITERION,
    FORMATION_FACTOR, FORMATION_PRECISION,
};

use crate::exact_sum::{ExactAccumulator, SumError};
use crate::load_ledger::{AssembledForce, ForceTerm, ForceTermKind};
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
#[derive(Debug)]
pub struct SymmetryEvidence<'a> {
    pub absolute_roundoff: &'a [Vec<f64>],
    pub operation_counts: &'a [Vec<usize>],
    pub basis: &'a str,
}
#[derive(Debug)]
pub struct StructuralSystem<'a> {
    pub stiffness: &'a [Vec<f64>],
    pub force: &'a [f64],
    pub free_dofs: &'a [usize],
    pub prescribed: &'a [(usize, f64)],
    pub contributions: Option<&'a [StiffnessContribution]>,
    pub symmetry: Option<SymmetryEvidence<'a>>,
}
/// Typed structural system (S11-K): today's `StructuralSystem` layout plus the
/// ledger-built `AssembledForce` its `force` slice is taken from. The kernel's
/// right-hand side (KS1), refinement residual (KS3) and load-fidelity audit
/// read the ledger terms from here. A plain vector is refused at compile time:
/// ```compile_fail
/// use open_pipe_stress_frame_kernel::structural::StructuralSystem;
/// let k = vec![vec![1.0]];
/// let force: Vec<f64> = vec![1.0];
/// let _ = StructuralSystem::assembled(&k, &force, &[0], &[], None, None);
/// ```
/// ```
/// use open_pipe_stress_frame_kernel::load_ledger::LoadLedger;
/// use open_pipe_stress_frame_kernel::structural::{solve_assembled_structural_dense, StructuralSystem};
/// let mut ledger = LoadLedger::new();
/// ledger.push("load:a", 0, 1.0);
/// let force = ledger.finish(1).unwrap();
/// let k = vec![vec![1.0]];
/// let system = StructuralSystem::assembled(&k, &force, &[0], &[], None, None);
/// assert_eq!(solve_assembled_structural_dense(&system).unwrap().displacements, vec![1.0]);
/// ```
#[derive(Debug)]
pub struct AssembledStructuralSystem<'a> {
    system: StructuralSystem<'a>,
    force: &'a AssembledForce,
}
impl<'a> StructuralSystem<'a> {
    /// The typed constructor: the force vector is the ledger's.
    pub fn assembled(
        stiffness: &'a [Vec<f64>],
        force: &'a AssembledForce,
        free_dofs: &'a [usize],
        prescribed: &'a [(usize, f64)],
        contributions: Option<&'a [StiffnessContribution]>,
        symmetry: Option<SymmetryEvidence<'a>>,
    ) -> AssembledStructuralSystem<'a> {
        AssembledStructuralSystem {
            system: StructuralSystem {
                stiffness,
                force: force.values(),
                free_dofs,
                prescribed,
                contributions,
                symmetry,
            },
            force,
        }
    }
}
/// A structural system with its formation source (K-D5, D1 §4.3.1): the
/// primitives of every stiffness contribution, from which the D-5 formation
/// check re-forms the intended system. Only the linear route builds one
/// (`solve_with_formation_check` in the structural adapter); a plain
/// `StructuralSystem` carries none, so every other caller is unchanged.
#[derive(Debug)]
pub struct FormationCheckedSystem<'a> {
    system: StructuralSystem<'a>,
    /// The ledger force of a typed system (S11-K); `None` for a legacy vector.
    force: Option<&'a AssembledForce>,
    source: &'a FormationSource,
}
impl<'a> StructuralSystem<'a> {
    /// Attaches the formation source the D-5 check re-forms from.
    pub fn with_formation_source(self, source: &'a FormationSource) -> FormationCheckedSystem<'a> {
        FormationCheckedSystem {
            system: self,
            force: None,
            source,
        }
    }
}
impl<'a> AssembledStructuralSystem<'a> {
    /// Typed sibling of `StructuralSystem::with_formation_source`: KS1, KS3,
    /// the load audit and the check's ρ use the ledger terms.
    pub fn with_formation_source(self, source: &'a FormationSource) -> FormationCheckedSystem<'a> {
        FormationCheckedSystem {
            system: self.system,
            force: Some(self.force),
            source,
        }
    }
}
impl<'a> FormationCheckedSystem<'a> {
    pub fn system(&self) -> &StructuralSystem<'a> {
        &self.system
    }
    pub fn source(&self) -> &'a FormationSource {
        self.source
    }
    fn binding(&self) -> ForceBinding<'a> {
        match self.force {
            Some(force) => ForceBinding::Assembled(force),
            None => ForceBinding::Legacy,
        }
    }
}
impl<'a> AssembledStructuralSystem<'a> {
    pub fn system(&self) -> &StructuralSystem<'a> {
        &self.system
    }
    pub fn force(&self) -> &'a AssembledForce {
        self.force
    }
}
/// Where the kernel takes its load terms from.
#[derive(Debug, Clone, Copy)]
enum ForceBinding<'s> {
    /// Legacy `&[f64]`: each DOF's value is its only term; no load audit.
    /// KS1/KS3 are exact on rows coupled to a nonzero prescribed value.
    Legacy,
    /// The named, unchanged binary64 path (ROOT option (c)): KS1 and KS3 fold
    /// in binary64 on every row, as before S11-K; no load audit.
    Binary64,
    /// Ledger force: KS1/KS3 and the load audit use its terms.
    Assembled(&'s AssembledForce),
    /// Legacy vector solved as given; the load audit compares it with these
    /// identified terms (the C3-detect entry point).
    AuditTerms(&'s [ForceTerm]),
}
impl<'s> ForceBinding<'s> {
    fn binary64(self) -> bool {
        matches!(self, Self::Binary64)
    }
    fn assembled(self) -> Option<&'s AssembledForce> {
        match self {
            Self::Assembled(force) => Some(force),
            _ => None,
        }
    }
    fn audit_terms(self) -> Option<&'s [ForceTerm]> {
        match self {
            Self::Legacy | Self::Binary64 => None,
            Self::Assembled(force) => Some(force.terms()),
            Self::AuditTerms(terms) => Some(terms),
        }
    }
}
/// One row flagged by the load-fidelity audit (S11 sections 5.1 and 6).
#[derive(Debug, Clone, PartialEq)]
pub struct LoadFidelityRow {
    pub global_dof: usize,
    /// True for a prescribed (restrained) row, where the residual is the reaction.
    pub restrained: bool,
    /// Bits of the correctly rounded exact net of the identified terms.
    pub exact_net_bits: u64,
    /// Bits of the force value the solve used.
    pub actual_bits: u64,
    pub guarded_ratio: f64,
    pub target: f64,
    /// The audit's own per-row operation count m_i.
    pub operation_count: usize,
    /// Largest row amplification d_i/|f_i| for which the guard is complete at
    /// 1e-9: 1e-9 / (64 * gamma(m_i)).
    pub completeness_limit: f64,
    pub sources: Vec<String>,
    /// RV1-N5: `Some(reason)` when the row's audit arithmetic left the radix
    /// range. The row is flagged (Sensitive, never a refusal), with an
    /// infinite ratio; `exact_net_bits` is NaN's when the net is out of range.
    pub unaudited: Option<&'static str>,
}
/// Present only when a row is flagged; never part of `StructuralReport`, so the
/// Debug-published report is byte-unchanged when nothing is flagged.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadFidelityReport {
    pub rows: Vec<LoadFidelityRow>,
    /// RV1-N5: set when the audit as a whole could not run (an invalid
    /// identified term). The case is then Sensitive, never a refusal.
    pub audit_error: Option<String>,
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
    pub accumulated_expansion: Vec<f64>,
    pub difference_expansion: Vec<f64>,
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
    pub assembly_load_perturbation_estimate: f64,
    pub intended_residual_rows: Vec<ResidualRow>,
    pub symmetry_projection_performed: bool,
    pub maximum_scaled_skew: f64,
    pub symmetry_basis: Option<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StructuralSolution {
    pub displacements: Vec<f64>,
    pub report: StructuralReport,
    /// Load-fidelity audit findings (S11); `None` when nothing is flagged or
    /// when no identified force terms were supplied.
    pub load_fidelity: Option<LoadFidelityReport>,
    /// The D-5 formation check's record (K-D5); `Some` only when it demoted
    /// the case from Passed to Sensitive. Never part of `StructuralReport`.
    pub formation_check: Option<FormationCheck>,
}
#[derive(Debug)]
pub struct PreparedSystem<'s> {
    source: &'s StructuralSystem<'s>,
    force_binding: ForceBinding<'s>,
    /// K-D5: present only for a `FormationCheckedSystem`.
    formation: Option<&'s FormationSource>,
    matrix: Vec<Vec<f64>>,
    rhs: Vec<f64>,
    scale_exponents: Vec<i32>,
    assembly_relative_perturbation_estimate: f64,
    assembly_load_perturbation_estimate: f64,
    contribution_rounding: Vec<ContributionRounding>,
    symmetry_projection_performed: bool,
    maximum_scaled_skew: f64,
    symmetry_basis: Option<String>,
}
impl PreparedSystem<'_> {
    pub fn matrix(&self) -> &[Vec<f64>] {
        &self.matrix
    }
    pub fn free_dofs(&self) -> &[usize] {
        self.source.free_dofs
    }
    pub fn scale_exponents(&self) -> &[i32] {
        &self.scale_exponents
    }
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
fn validate_prepared(prepared: &PreparedSystem<'_>) -> Result<(), StructuralError> {
    validate(prepared.source)?;
    let n = prepared.source.free_dofs.len();
    if prepared.matrix.len() != n
        || prepared.rhs.len() != n
        || prepared.scale_exponents.len() != n
        || prepared
            .matrix
            .iter()
            .any(|r| r.len() != n || r.iter().any(|v| !v.is_finite()))
        || prepared.rhs.iter().any(|v| !v.is_finite())
    {
        return Err(StructuralError::InvalidInput(
            "prepared shape/map/finiteness",
        ));
    }
    for i in 0..n {
        for j in 0..i {
            if prepared.matrix[i][j] != prepared.matrix[j][i] {
                return Err(StructuralError::InvalidInput("prepared symmetry"));
            }
        }
    }
    Ok(())
}
/// Exact nonoverlapping expansion of represented inputs within checked IEEE range.
/// Unsupported overflow/underflow is an error, never an erased low tail.
#[derive(Debug, Clone, Default)]
pub(crate) struct Expansion {
    pub(crate) terms: Vec<f64>,
    pub(crate) operations: usize,
}
impl Expansion {
    pub(crate) fn add(&mut self, value: f64) -> Result<(), StructuralError> {
        if !value.is_finite() {
            return Err(StructuralError::Range("expansion input"));
        }
        let mut q = value;
        let mut next = Vec::new();
        for &e in &self.terms {
            let sum = q + e;
            let z = sum - q;
            let low = (q - (sum - z)) + (e - z);
            if !sum.is_finite() || !z.is_finite() || !low.is_finite() {
                return Err(StructuralError::Range("expansion addition"));
            }
            if low != 0.0 {
                next.push(low);
            }
            q = sum;
            self.operations += 6;
        }
        if q != 0.0 {
            next.push(q);
        }
        self.terms = next;
        Ok(())
    }
    pub(crate) fn add_product(
        &mut self,
        a: f64,
        b: f64,
        exponent: i32,
    ) -> Result<(), StructuralError> {
        if a == 0.0 || b == 0.0 {
            return Ok(());
        }
        if !a.is_finite() || !b.is_finite() {
            return Err(StructuralError::Range("expansion product input"));
        }
        let ea = binary_exponent(a);
        let eb = binary_exponent(b);
        let ma = exact_radix(a, -ea)?;
        let mb = exact_radix(b, -eb)?;
        let hi = ma * mb;
        let lo = ma.mul_add(mb, -hi);
        self.operations += 2;
        self.add(exact_radix(lo, ea + eb + exponent)?)?;
        self.add(exact_radix(hi, ea + eb + exponent)?)?;
        Ok(())
    }
    pub(crate) fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
    /// The expansion's exact value, correctly rounded once (+0.0 for zero).
    pub(crate) fn round(&self) -> Result<f64, StructuralError> {
        crate::exact_sum::exact_rounded_sum(self.terms.iter().copied()).map_err(sum_range)
    }
    pub(crate) fn exact_scalar(&self) -> Result<Option<f64>, StructuralError> {
        let value = self.round()?;
        let mut difference = self.clone();
        difference.add(-value)?;
        Ok(if difference.is_zero() {
            Some(value)
        } else {
            None
        })
    }
}
fn sum_range(error: SumError) -> StructuralError {
    match error {
        SumError::NonFinite => StructuralError::Range("exact sum input"),
        SumError::AccumulatorOverflow => StructuralError::Range("exact sum accumulator"),
        SumError::NonRepresentable => StructuralError::Range("exact sum outside binary64 range"),
    }
}
pub(crate) fn exact_radix(mut value: f64, mut exponent: i32) -> Result<f64, StructuralError> {
    if !value.is_finite() {
        return Err(StructuralError::Range("exact radix input"));
    }
    while exponent != 0 && value != 0.0 {
        let step = exponent.clamp(-512, 512);
        let next = value * 2.0_f64.powi(step);
        if !next.is_finite() || next == 0.0 || next * 2.0_f64.powi(-step) != value {
            return Err(StructuralError::Range("exact radix loses represented bits"));
        }
        value = next;
        exponent -= step;
    }
    Ok(value)
}
fn contribution_sums(
    system: &StructuralSystem<'_>,
) -> Result<Option<Vec<Vec<Expansion>>>, StructuralError> {
    let Some(entries) = system.contributions else {
        return Ok(None);
    };
    let n = system.force.len();
    let mut sums = vec![vec![Expansion::default(); n]; n];
    for entry in entries {
        if entry.row >= n || entry.col >= n || !entry.value.is_finite() {
            return Err(StructuralError::InvalidInput("contribution"));
        }
        let sum = &mut sums[entry.row][entry.col];
        let old = sum.round()?;
        sum.add(entry.value)?;
        if entry.row == entry.col && entry.value > 0.0 && old != 0.0 && sum.round()? == old {
            return Err(unresolved(
                "positive diagonal contribution absorbed by assembly; stabilization unresolved",
                Some(entry.row),
            ));
        }
    }
    Ok(Some(sums))
}
fn contribution_differences(
    system: &StructuralSystem<'_>,
    sums: &[Vec<Expansion>],
) -> Result<Vec<Vec<Expansion>>, StructuralError> {
    let mut differences = sums.to_vec();
    for (i, row) in differences.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            value.add(-system.stiffness[i][j])?;
        }
    }
    Ok(differences)
}
fn audit_contributions(
    system: &StructuralSystem<'_>,
    exponents: &[i32],
    rhs: &[f64],
) -> Result<(f64, f64, Vec<ContributionRounding>), StructuralError> {
    let Some(sums) = contribution_sums(system)? else {
        return Ok((0.0, 0.0, Vec::new()));
    };
    let differences = contribution_differences(system, &sums)?;
    let mut rounding = Vec::new();
    for (i, row) in differences.iter().enumerate() {
        for (j, difference) in row.iter().enumerate() {
            if !difference.is_zero() {
                let terms = &sums[i][j].terms;
                rounding.push(ContributionRounding {
                    row: i,
                    col: j,
                    accumulated_high: terms.last().copied().unwrap_or(0.0),
                    accumulated_low: terms.iter().rev().skip(1).copied().sum(),
                    stored_difference_high: difference.terms.last().copied().unwrap_or(0.0),
                    stored_difference_low: difference.terms.iter().rev().skip(1).copied().sum(),
                    accumulated_expansion: terms.clone(),
                    difference_expansion: difference.terms.clone(),
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
            for &part in &differences[i][j].terms {
                delta_column = checked_value(
                    delta_column + exact_radix(part, exponents[r] + exponents[c])?.abs(),
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
    let mut delta_rhs_norm = 0.0;
    let mut rhs_norm = 0.0;
    for (r, &i) in system.free_dofs.iter().enumerate() {
        let mut delta = Expansion::default();
        for &(j, value) in system.prescribed {
            for &part in &differences[i][j].terms {
                delta.add_product(-part, value, exponents[r])?;
            }
        }
        delta_rhs_norm =
            checked_value(delta_rhs_norm + delta.terms.iter().map(|v| v.abs()).sum::<f64>())?;
        rhs_norm = checked_value(rhs_norm + rhs[r].abs())?;
    }
    if delta_rhs_norm != 0.0 && rhs_norm == 0.0 {
        return Err(unresolved(
            "contribution-preserved prescribed coupling changes zero reduced load",
            None,
        ));
    }
    Ok((
        if matrix_norm == 0.0 {
            0.0
        } else {
            perturbation_norm / matrix_norm
        },
        if rhs_norm == 0.0 {
            0.0
        } else {
            delta_rhs_norm / rhs_norm
        },
        rounding,
    ))
}
/// Componentwise action of the complete intended contributions, including K_fc*u_c.
/// Exact expansion numerator; denominator and guard use their separately recorded
/// operation count. This does not establish forward accuracy of a soft mode.
fn audit_intended_action(
    system: &StructuralSystem<'_>,
    u: &[f64],
) -> Result<Vec<ResidualRow>, StructuralError> {
    let Some(sums) = contribution_sums(system)? else {
        return Ok(Vec::new());
    };
    let mut rows = Vec::new();
    for &i in system.free_dofs {
        let mut exponent = if system.force[i] == 0.0 {
            i32::MIN
        } else {
            binary_exponent(system.force[i])
        };
        for (j, coefficient) in sums[i].iter().enumerate() {
            if u[j] != 0.0 {
                for &part in &coefficient.terms {
                    exponent = exponent.max(binary_exponent(part) + binary_exponent(u[j]) + 1);
                }
            }
        }
        if exponent == i32::MIN {
            exponent = 0;
        }
        let mut residual = Expansion::default();
        residual.add(exact_radix(-system.force[i], -exponent)?)?;
        let mut denominator = exact_radix(system.force[i].abs(), -exponent)?;
        let mut denominator_operations = 0;
        for (j, coefficient) in sums[i].iter().enumerate() {
            for &part in &coefficient.terms {
                residual.add_product(part, u[j], -exponent)?;
                // Positive sum over exact expansion components is an explicit estimate
                // of |coalesced intended K|*|u|, not original unconsolidated contribution magnitudes.
                denominator = checked_value(
                    denominator + normalized_product(part.abs(), u[j].abs(), exponent)?,
                )?;
                denominator_operations += 2;
            }
        }
        let operations = (residual.operations + denominator_operations + 2).max(2);
        let g = gamma(operations);
        // Zero witness (S11 D-S11-1): an exact-zero intended-action residual keeps
        // the -0.0 that committed diagnostics publish (245 `normalized_residual:
        // -0.0` values in 31 fixture files, e.g. n05-dense_scrutiny.raw.json).
        // It is only rendered in Debug diagnostics and gated through abs(); it
        // never feeds a bit-equality check or a receipt. Every other zero is +0.0.
        let r = if residual.is_zero() {
            -0.0
        } else {
            residual.round()?
        };
        let allowance = g * denominator / (1.0 - g);
        let ratio = if denominator == 0.0 {
            if residual.is_zero() {
                0.0
            } else {
                f64::INFINITY
            }
        } else {
            (r.abs() / denominator + g / (1.0 - g)) * (1.0 + g) * (1.0 + 4.0 * f64::EPSILON)
        };
        let target = 64.0 * g;
        rows.push(ResidualRow {global_dof:i,residual:physical_residual_record(r,exponent)?,denominator:physical_residual_record(denominator,exponent)?,row_scale_exponent:exponent,normalized_residual:r,normalized_denominator:denominator,normalized_evaluation_allowance:allowance,normalization_basis:"exact contribution expansions and full intended free action including prescribed DOFs",operation_count:operations,evaluation_allowance:physical_residual_record(allowance,exponent)?,guarded_ratio:ratio,target,passed:ratio<=target});
    }
    Ok(rows)
}
fn term_exponent(term: &ForceTerm) -> Option<i32> {
    match term.kind {
        ForceTermKind::Term(x) if x != 0.0 => Some(binary_exponent(x)),
        ForceTermKind::Product(a, b) if a != 0.0 && b != 0.0 => {
            Some(binary_exponent(a) + binary_exponent(b) + 1)
        }
        _ => None,
    }
}
fn subtract_term(
    residual: &mut Expansion,
    term: &ForceTerm,
    exponent: i32,
) -> Result<(), StructuralError> {
    match term.kind {
        ForceTermKind::Term(x) => residual.add(exact_radix(-x, -exponent)?),
        ForceTermKind::Product(a, b) => residual.add_product(-a, b, -exponent),
    }
}
/// Load-fidelity audit (S11 section 5.1): M03's intended-action predicate with
/// the load term replaced by the exact per-DOF sum of the identified terms, on
/// free AND restrained rows.
/// - Free row: `|sum_j K_ij u_j - f_i^exact| / d_i`.
/// - Restrained row (the residual is the reaction): the published reaction's
///   load term against the exact one, `|f_i - f_i^exact| / d_i`.
///
/// `d_i = |f_i^exact| + sum_j |K_ij| |u_j|`; a row is flagged when the guarded
/// ratio exceeds `64 * gamma(m_i)`, m_i being this row's own operation count.
/// K is the exact intended stiffness (contribution expansions) when supplied,
/// otherwise the represented matrix. Only flagged rows are returned.
pub fn audit_load_fidelity(
    system: &StructuralSystem<'_>,
    u: &[f64],
    force_terms: &[ForceTerm],
) -> Result<LoadFidelityReport, StructuralError> {
    validate(system)?;
    let n = system.force.len();
    if u.len() != n || u.iter().any(|x| !x.is_finite()) {
        return Err(StructuralError::InvalidInput("displacement vector"));
    }
    let mut by_dof: Vec<Vec<&ForceTerm>> = vec![Vec::new(); n];
    for term in force_terms {
        if term.dof >= n || term.source.is_empty() {
            return Err(StructuralError::InvalidInput("force term"));
        }
        by_dof[term.dof].push(term);
    }
    let sums = contribution_sums(system)?;
    let mut restrained = vec![false; n];
    for &(i, _) in system.prescribed {
        restrained[i] = true;
    }
    let mut rows = Vec::new();
    for i in 0..n {
        let terms = &by_dof[i];
        let row = match audit_load_row(system, u, sums.as_deref(), &restrained, i, terms) {
            Ok(row) => row,
            // RV1-N5 (S11 section 6): an auditable row's arithmetic that
            // leaves the radix range (for example terms more than about
            // 2^1000 apart, (1e80, -1e80, 1e-300)) never turns the solve into
            // an error. The row is reported unaudited, which makes the case
            // Sensitive; the solve's values are unchanged.
            Err(StructuralError::Range(reason)) => {
                Some(unaudited_row(system, &restrained, i, terms, reason))
            }
            Err(error) => return Err(error),
        };
        rows.extend(row);
    }
    Ok(LoadFidelityReport {
        rows,
        audit_error: None,
    })
}
fn row_sources(terms: &[&ForceTerm]) -> Vec<String> {
    let mut sources: Vec<String> = terms.iter().map(|t| t.source.clone()).collect();
    sources.sort();
    sources.dedup();
    sources
}
/// RV1-N5: a row whose audit arithmetic leaves the radix range. It is flagged
/// (Sensitive, never a refusal) with an infinite ratio and its reason.
fn unaudited_row(
    system: &StructuralSystem<'_>,
    restrained: &[bool],
    i: usize,
    terms: &[&ForceTerm],
    reason: &'static str,
) -> LoadFidelityRow {
    let mut net = ExactAccumulator::new();
    let exact_net = terms
        .iter()
        .try_for_each(|term| term.accumulate(&mut net, false))
        .and_then(|()| net.round())
        .unwrap_or(f64::NAN);
    LoadFidelityRow {
        global_dof: i,
        restrained: restrained[i],
        exact_net_bits: exact_net.to_bits(),
        actual_bits: system.force[i].to_bits(),
        guarded_ratio: f64::INFINITY,
        target: 0.0,
        operation_count: 0,
        completeness_limit: 0.0,
        sources: row_sources(terms),
        unaudited: Some(reason),
    }
}
/// One row of `audit_load_fidelity`: `Some` when flagged.
fn audit_load_row(
    system: &StructuralSystem<'_>,
    u: &[f64],
    sums: Option<&[Vec<Expansion>]>,
    restrained: &[bool],
    i: usize,
    terms: &[&ForceTerm],
) -> Result<Option<LoadFidelityRow>, StructuralError> {
    let mut net = ExactAccumulator::new();
    for term in terms {
        term.accumulate(&mut net, false).map_err(sum_range)?;
    }
    let exact_net = net.round().map_err(sum_range)?;
    let parts: Vec<(f64, f64)> = match sums {
        Some(sums) => sums[i]
            .iter()
            .zip(u)
            .flat_map(|(coefficient, &x)| coefficient.terms.iter().map(move |&k| (k, x)))
            .collect(),
        None => system.stiffness[i]
            .iter()
            .zip(u)
            .filter(|(&k, _)| k != 0.0)
            .map(|(&k, &x)| (k, x))
            .collect(),
    };
    let mut exponent = i32::MIN;
    for term in terms {
        if let Some(e) = term_exponent(term) {
            exponent = exponent.max(e);
        }
    }
    if restrained[i] && system.force[i] != 0.0 {
        exponent = exponent.max(binary_exponent(system.force[i]));
    }
    for &(k, x) in &parts {
        if k != 0.0 && x != 0.0 {
            exponent = exponent.max(binary_exponent(k) + binary_exponent(x) + 1);
        }
    }
    if exponent == i32::MIN {
        exponent = 0;
    }
    let mut residual = Expansion::default();
    if restrained[i] {
        residual.add(exact_radix(system.force[i], -exponent)?)?;
    } else {
        for &(k, x) in &parts {
            residual.add_product(k, x, -exponent)?;
        }
    }
    for term in terms {
        subtract_term(&mut residual, term, exponent)?;
    }
    let mut denominator = exact_radix(exact_net.abs(), -exponent)?;
    let mut denominator_operations = 0;
    for &(k, x) in &parts {
        denominator = checked_value(denominator + normalized_product(k.abs(), x.abs(), exponent)?)?;
        denominator_operations += 2;
    }
    let operations = (residual.operations + denominator_operations + 2).max(2);
    let g = gamma(operations);
    let r = residual.round()?;
    let ratio = if denominator == 0.0 {
        if residual.is_zero() {
            0.0
        } else {
            f64::INFINITY
        }
    } else {
        (r.abs() / denominator + g / (1.0 - g)) * (1.0 + g) * (1.0 + 4.0 * f64::EPSILON)
    };
    let target = 64.0 * g;
    Ok((ratio > target).then(|| LoadFidelityRow {
        global_dof: i,
        restrained: restrained[i],
        exact_net_bits: exact_net.to_bits(),
        actual_bits: system.force[i].to_bits(),
        guarded_ratio: ratio,
        target,
        operation_count: operations,
        completeness_limit: 1e-9 / target,
        sources: row_sources(terms),
        unaudited: None,
    }))
}
/// Nearby radix diagonal equilibration: A = T K_ff T, b = T f_reduced, u_f=T y.
/// Ideal work-conjugate length/energy scaling cancels in total diagonal equilibration.
pub fn prepare_structural<'s>(
    system: &'s StructuralSystem<'s>,
) -> Result<PreparedSystem<'s>, StructuralError> {
    prepare_bound(system, ForceBinding::Legacy)
}
/// Named, unchanged binary64 variant of `prepare_structural` for the nonlinear
/// active-set loop's closed-gap prescribed solves (ROOT option (c)): the
/// right-hand side and the refinement residual fold in binary64 as before
/// S11-K. Linear callers use `prepare_structural` (exact KS1/KS3).
pub fn prepare_structural_binary64<'s>(
    system: &'s StructuralSystem<'s>,
) -> Result<PreparedSystem<'s>, StructuralError> {
    prepare_bound(system, ForceBinding::Binary64)
}
/// K-D5 sibling of `prepare_structural` (or of `prepare_assembled_structural`
/// for a typed system): the same preparation, and the completion runs the
/// D-5 formation check before publishing Passed.
pub fn prepare_formation_checked_structural<'s>(
    system: &'s FormationCheckedSystem<'s>,
) -> Result<PreparedSystem<'s>, StructuralError> {
    let mut prepared = prepare_bound(&system.system, system.binding())?;
    prepared.formation = Some(system.source);
    Ok(prepared)
}
/// K-D5 sibling of `prepare_structural_with_force_terms` (the check's ρ then
/// uses the identified terms). A typed system keeps its own ledger terms.
pub fn prepare_formation_checked_structural_with_force_terms<'s>(
    system: &'s FormationCheckedSystem<'s>,
    force_terms: &'s [ForceTerm],
) -> Result<PreparedSystem<'s>, StructuralError> {
    let binding = match system.force {
        Some(force) => ForceBinding::Assembled(force),
        None => ForceBinding::AuditTerms(force_terms),
    };
    let mut prepared = prepare_bound(&system.system, binding)?;
    prepared.formation = Some(system.source);
    Ok(prepared)
}
/// Typed sibling of `prepare_structural`: KS1 sums the ledger terms exactly.
pub fn prepare_assembled_structural<'s>(
    system: &'s AssembledStructuralSystem<'s>,
) -> Result<PreparedSystem<'s>, StructuralError> {
    prepare_bound(&system.system, ForceBinding::Assembled(system.force))
}
/// The C3-detect entry point: the legacy vector is solved as given, and the
/// load-fidelity audit compares it with the identified terms.
pub fn prepare_structural_with_force_terms<'s>(
    system: &'s StructuralSystem<'s>,
    force_terms: &'s [ForceTerm],
) -> Result<PreparedSystem<'s>, StructuralError> {
    prepare_bound(system, ForceBinding::AuditTerms(force_terms))
}
fn prescribed_coupled(system: &StructuralSystem<'_>, row: usize) -> bool {
    system
        .prescribed
        .iter()
        .any(|&(j, u)| system.stiffness[row][j] != 0.0 && u != 0.0)
}
/// KS1: `f_i - sum_c K_ic * g_c`, one exact sum scaled by `2^exponent` before
/// its single rounding (S11 R3-4), with `radix_scale`'s range semantics.
fn exact_scaled_rhs(
    system: &StructuralSystem<'_>,
    binding: ForceBinding<'_>,
    row: usize,
    exponent: i32,
) -> Result<f64, StructuralError> {
    let mut accumulator = ExactAccumulator::new();
    match binding.assembled() {
        Some(force) => force
            .accumulate_dof(row, &mut accumulator, false)
            .map_err(sum_range)?,
        None => accumulator.add(system.force[row]).map_err(sum_range)?,
    }
    for &(j, u) in system.prescribed {
        accumulator
            .add_product(-system.stiffness[row][j], u)
            .map_err(sum_range)?;
    }
    if accumulator.round().is_err() {
        return Err(StructuralError::Range("nonfinite radix input"));
    }
    let value = accumulator
        .round_scaled(exponent)
        .map_err(|_| StructuralError::Range("radix scaling loses normal range"))?;
    if value.is_subnormal() || (value == 0.0 && !accumulator.is_zero()) {
        return Err(StructuralError::Range("radix scaling loses normal range"));
    }
    Ok(value)
}
fn prepare_bound<'s>(
    system: &'s StructuralSystem<'s>,
    force_binding: ForceBinding<'s>,
) -> Result<PreparedSystem<'s>, StructuralError> {
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
        if !force_binding.binary64()
            && (force_binding.assembled().is_some() || prescribed_coupled(system, i))
        {
            rhs[r] = exact_scaled_rhs(system, force_binding, i, exponents[r])?;
        } else {
            // No nonzero prescribed product: today's evaluation, bit for bit.
            let mut b = system.force[i];
            for &(j, u) in system.prescribed {
                b = checked_value(b - checked_product(system.stiffness[i][j], u)?)?;
            }
            rhs[r] = radix_scale(b, exponents[r])?;
        }
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
    let (perturbation, load_perturbation, rounding) =
        audit_contributions(system, &exponents, &rhs)?;
    Ok(PreparedSystem {
        source: system,
        force_binding,
        formation: None,
        matrix: a,
        rhs,
        scale_exponents: exponents,
        assembly_relative_perturbation_estimate: perturbation,
        assembly_load_perturbation_estimate: load_perturbation,
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
/// This public evaluation is today's binary64 measurement, byte for byte
/// (its callers are the nonlinear loop's product-equilibrium observation and
/// the gap scrutiny). The solve's own residual and refinement use the exact
/// KS3 numerator on prescribed-coupled rows (see `finish_structural`), and so
/// does `evaluate_assembled_original_residual`.
pub fn evaluate_original_residual(
    system: &StructuralSystem<'_>,
    u: &[f64],
) -> Result<Vec<ResidualRow>, StructuralError> {
    evaluate_original_residual_bound(system, ForceBinding::Binary64, u)
}
/// Typed sibling of `evaluate_original_residual` (KS3 reads the ledger terms).
pub fn evaluate_assembled_original_residual(
    system: &AssembledStructuralSystem<'_>,
    u: &[f64],
) -> Result<Vec<ResidualRow>, StructuralError> {
    evaluate_original_residual_bound(&system.system, ForceBinding::Assembled(system.force), u)
}
fn evaluate_original_residual_bound(
    system: &StructuralSystem<'_>,
    binding: ForceBinding<'_>,
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
        // KS3: on a row coupled to a nonzero prescribed value the numerator is
        // one exact sum (ledger terms negated, plus K_ij * u_j over the full
        // row), scaled before its single rounding. Other rows: today's code.
        let exact_numerator = !binding.binary64() && prescribed_coupled(system, i);
        let mut numerator = ExactAccumulator::new();
        if exact_numerator {
            match binding.assembled() {
                Some(force) => force
                    .accumulate_dof(i, &mut numerator, true)
                    .map_err(sum_range)?,
                None => numerator.add(-system.force[i]).map_err(sum_range)?,
            }
        }
        for (&k, &x) in system.stiffness[i].iter().zip(u) {
            if k == 0.0 {
                continue;
            }
            count += 1;
            let p = normalized_product(k, x, row_exponent)?;
            if exact_numerator {
                numerator.add_product(k, x).map_err(sum_range)?;
            } else {
                r = checked_value(r + p)?;
            }
            d = checked_value(d + p.abs())?;
        }
        if exact_numerator {
            r = checked_value(numerator.round_scaled(-row_exponent).map_err(sum_range)?)?;
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
fn finish_checked_factor<F>(
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
    let amplification = (prepared.assembly_relative_perturbation_estimate
        + prepared.assembly_load_perturbation_estimate)
        / rcond;
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
        let residual = evaluate_original_residual_bound(system, prepared.force_binding, &u)?;
        let worst = residual
            .iter()
            .map(|r| r.guarded_ratio / r.target)
            .fold(0.0, f64::max);
        if residual.iter().all(|r| r.passed) {
            let intended = audit_intended_action(system, &u)?;
            if intended.iter().any(|r| !r.passed) {
                return Err(unresolved(
                    "contribution-preserved intended free action failed",
                    None,
                ));
            }
            // S11 section 6: a detected load loss is Sensitive, never a refusal.
            let load_fidelity = match prepared.force_binding.audit_terms() {
                // RV1-N5: an audit failure is never propagated; it is a
                // Sensitive report (section 6).
                Some(terms) => match audit_load_fidelity(system, &u, terms) {
                    Ok(report) => (!report.rows.is_empty()).then_some(report),
                    Err(error) => Some(LoadFidelityReport {
                        rows: Vec::new(),
                        audit_error: Some(format!("{error:?}")),
                    }),
                },
                None => None,
            };
            let ordinary_sensitive = rcond < f64::EPSILON.sqrt() || load_fidelity.is_some();
            // K-D5 (D1 §4.3.1): a case that would publish Passed is demoted to
            // Sensitive when the formation check's estimate exceeds the
            // criterion or the check cannot re-form it. Values never change.
            let formation_check = match prepared.formation {
                Some(source) if !ordinary_sensitive => formation_check::check(
                    system,
                    source,
                    prepared.force_binding.audit_terms(),
                    &prepared.scale_exponents,
                    &u,
                    &solve,
                ),
                _ => None,
            };
            return Ok(StructuralSolution {
                displacements: u,
                load_fidelity: load_fidelity.clone(),
                formation_check: formation_check.clone(),
                report: StructuralReport {
                    policy: POLICY,
                    quality: if ordinary_sensitive || formation_check.is_some() {
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
                    assembly_load_perturbation_estimate: prepared
                        .assembly_load_perturbation_estimate,
                    intended_residual_rows: intended,
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
struct CholeskyFactor {
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
fn cholesky(
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
/// Positive factors cannot be constructed or relabelled by callers. Their source
/// is the same immutable prepared object used by completion.
pub struct PositiveFactor<'p, 's> {
    prepared: &'p PreparedSystem<'s>,
    backend: PositiveBackend,
    pivots: Vec<PivotEvidence>,
}
enum PositiveBackend {
    Dense(CholeskyFactor),
    Profile(ProfileFactor),
}
impl PositiveFactor<'_, '_> {
    pub fn pivots(&self) -> &[PivotEvidence] {
        &self.pivots
    }
    fn solve(&self, rhs: &[f64]) -> Result<Vec<f64>, StructuralError> {
        match &self.backend {
            PositiveBackend::Dense(f) => f.solve(rhs),
            PositiveBackend::Profile(f) => f.solve(rhs),
        }
    }
}
pub fn factor_structural_cholesky<'p, 's>(
    prepared: &'p PreparedSystem<'s>,
) -> Result<PositiveFactor<'p, 's>, StructuralError> {
    validate_prepared(prepared)?;
    let (factor, pivots) = cholesky(prepared, prepared.source.free_dofs)?;
    Ok(PositiveFactor {
        prepared,
        backend: PositiveBackend::Dense(factor),
        pivots,
    })
}
/// The sole public completion interface accepts checked, source-bound factors only.
/// ```compile_fail
/// use open_pipe_stress_frame_kernel::structural::finish_structural;
/// // An LU closure and caller-authored evidence cannot complete a structural solve.
/// let _ = finish_structural(&|rhs: &[f64]| Ok::<_, ()>(rhs.to_vec()));
/// ```
/// There is no public closure, label or user-created pivot-vector acceptance seam.
pub fn finish_structural(
    factor: &PositiveFactor<'_, '_>,
) -> Result<StructuralSolution, StructuralError> {
    let label = match factor.backend {
        PositiveBackend::Dense(_) => "positive dense Cholesky",
        PositiveBackend::Profile(_) => "positive skyline LDL",
    };
    finish_checked_factor(
        factor.prepared.source,
        factor.prepared,
        factor.pivots.clone(),
        label,
        |rhs| factor.solve(rhs),
    )
}
pub fn solve_structural_dense(
    system: &StructuralSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    solve_prepared_dense(prepare_structural(system)?)
}
/// Named, unchanged binary64 variant of `solve_structural_dense` for the
/// nonlinear active-set loop (ROOT option (c)).
pub fn solve_structural_dense_binary64(
    system: &StructuralSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    solve_prepared_dense(prepare_structural_binary64(system)?)
}
/// K-D5 sibling of `solve_structural_dense` (dense Cholesky, with the D-5
/// formation check before Passed).
pub fn solve_formation_checked_structural_dense(
    system: &FormationCheckedSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    solve_prepared_dense(prepare_formation_checked_structural(system)?)
}
/// Typed sibling of `solve_structural_dense`.
pub fn solve_assembled_structural_dense(
    system: &AssembledStructuralSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    solve_prepared_dense(prepare_assembled_structural(system)?)
}
/// C3-detect sibling of `solve_structural_dense` (see
/// `prepare_structural_with_force_terms`).
pub fn solve_structural_dense_with_force_terms(
    system: &StructuralSystem<'_>,
    force_terms: &[ForceTerm],
) -> Result<StructuralSolution, StructuralError> {
    solve_prepared_dense(prepare_structural_with_force_terms(system, force_terms)?)
}
fn solve_prepared_dense(
    prepared: PreparedSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    let factor = match factor_structural_cholesky(&prepared) {
        Ok(factor) => factor,
        Err(error) => return Err(negative_pair_witness(&prepared)?.unwrap_or(error)),
    };
    finish_structural(&factor)
}

struct ProfileFactor {
    first: Vec<usize>,
    rows: Vec<Vec<f64>>,
    order: Vec<usize>,
}
impl ProfileFactor {
    fn get(&self, i: usize, j: usize) -> f64 {
        if j < self.first[i] {
            0.0
        } else {
            self.rows[i][j - self.first[i]]
        }
    }
    fn set(&mut self, i: usize, j: usize, value: f64) {
        self.rows[i][j - self.first[i]] = value;
    }
    fn solve(&self, rhs: &[f64]) -> Result<Vec<f64>, StructuralError> {
        let n = self.order.len();
        if rhs.len() != n || rhs.iter().any(|v| !v.is_finite()) {
            return Err(StructuralError::InvalidInput("profile RHS"));
        }
        let mut x: Vec<f64> = self.order.iter().map(|&i| rhs[i]).collect();
        for i in 0..n {
            for j in self.first[i]..i {
                x[i] = checked_value(x[i] - checked_product(self.get(i, j), x[j])?)?;
            }
        }
        for i in 0..n {
            x[i] = checked_quotient(x[i], self.get(i, i))?;
        }
        for i in (0..n).rev() {
            let v = x[i];
            for j in self.first[i]..i {
                x[j] = checked_value(x[j] - checked_product(self.get(i, j), v)?)?;
            }
        }
        let mut result = vec![0.0; n];
        for (i, &original) in self.order.iter().enumerate() {
            result[original] = x[i];
        }
        Ok(result)
    }
}
/// Build and factor the supplied skyline of the PREPARED matrix. Caller supplies
/// only storage/ordering, never coefficients, factors or evidence of positivity.
/// This performs one skyline LDL; it does not add a dense factorization.
pub fn factor_structural_profile<'p, 's>(
    prepared: &'p PreparedSystem<'s>,
    order: &[usize],
    first: &[usize],
) -> Result<PositiveFactor<'p, 's>, StructuralError> {
    validate_prepared(prepared)?;
    let n = prepared.matrix.len();
    if order.len() != n || first.len() != n {
        return Err(StructuralError::InvalidInput("profile dimensions"));
    }
    let mut seen = vec![false; n];
    for &i in order {
        if i >= n || seen[i] {
            return Err(StructuralError::InvalidInput("profile permutation"));
        }
        seen[i] = true;
    }
    let mut rows = Vec::new();
    for i in 0..n {
        if first[i] > i {
            return Err(StructuralError::InvalidInput("profile first column"));
        }
        for j in 0..first[i] {
            if prepared.matrix[order[i]][order[j]] != 0.0 {
                return Err(StructuralError::InvalidInput(
                    "profile omits represented coefficient",
                ));
            }
        }
        rows.push(
            (first[i]..=i)
                .map(|j| prepared.matrix[order[i]][order[j]])
                .collect(),
        );
    }
    let mut factor = ProfileFactor {
        first: first.to_vec(),
        rows,
        order: order.to_vec(),
    };
    let mut pivots = Vec::new();
    let mut work = vec![0.0; n];
    for i in 0..n {
        for j in first[i]..i {
            let mut sum = factor.get(i, j);
            for k in first[i].max(first[j])..j {
                sum = checked_value(sum - checked_product(work[k], factor.get(j, k))?)?;
            }
            work[j] = sum;
            factor.set(i, j, checked_quotient(sum, factor.get(j, j))?);
        }
        let mut pivot = factor.get(i, i);
        let mut scale = pivot.abs();
        for k in first[i]..i {
            let term = checked_product(work[k], factor.get(i, k))?;
            pivot = checked_value(pivot - term)?;
            scale = checked_value(scale + term.abs())?;
        }
        pivots.push(screen_pivot(
            i,
            prepared.source.free_dofs[order[i]],
            pivot,
            scale,
            2 * (i - first[i]) + 2,
        )?);
        factor.set(i, i, pivot);
    }
    Ok(PositiveFactor {
        prepared,
        backend: PositiveBackend::Profile(factor),
        pivots,
    })
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
    prepared: &PreparedSystem<'_>,
    direction: &[f64],
) -> Result<Option<StructuralError>, StructuralError> {
    validate_prepared(prepared)?;
    let system = prepared.source;
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
    prepared: &PreparedSystem<'_>,
) -> Result<Option<StructuralError>, StructuralError> {
    validate_prepared(prepared)?;
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
            if let Some(witness) = verify_negative_direction(prepared, &v)? {
                return Ok(Some(witness));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod s11f_tests;
#[cfg(test)]
mod s11k_tests;

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
        assert!(cholesky(&prepared, &[]).is_err());
        prepared.matrix[0].clear();
        assert!(factor_structural_cholesky(&prepared).is_err());
        assert!(estimate_rcond(&[vec![1.0]], &|_| Ok(vec![])).is_err());
    }
    #[test]
    fn krev02_prescribed_contribution_error_is_not_zero_fidelity() {
        let matrix = vec![vec![2.0, -1.0], vec![-1.0, 2.0]];
        let force = [0.0, 0.0];
        let free = [1];
        let bc = [(0, 1.0)];
        for coupling in [-1.0, -2.0] {
            let entries = [
                StiffnessContribution {
                    row: 0,
                    col: 0,
                    value: 2.0,
                },
                StiffnessContribution {
                    row: 1,
                    col: 1,
                    value: 2.0,
                },
                StiffnessContribution {
                    row: 0,
                    col: 1,
                    value: coupling,
                },
                StiffnessContribution {
                    row: 1,
                    col: 0,
                    value: coupling,
                },
            ];
            let mut s = system(&matrix, &force, &free, &bc);
            s.contributions = Some(&entries);
            let prepared = prepare_structural(&s).unwrap();
            if coupling == -1.0 {
                let result = solve_structural_dense(&s).unwrap();
                assert_eq!(result.displacements[0], 1.0);
                assert!((result.displacements[1] - 0.5).abs() / 0.5 <= 1e-9);
                assert_eq!(result.report.assembly_load_perturbation_estimate, 0.0);
            } else {
                assert_eq!(prepared.assembly_load_perturbation_estimate, 1.0);
                assert!(solve_structural_dense(&s).is_err());
                assert!(!audit_intended_action(&s, &[1.0, 0.5]).unwrap()[0].passed);
            }
        }
    }
    #[test]
    fn krev03_offdiagonal_expansion_retains_lost_low_tail() {
        let matrix = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let force = [1.0, 1.0];
        let free = [0, 1];
        let mut entries = vec![
            StiffnessContribution {
                row: 0,
                col: 0,
                value: 2.0,
            },
            StiffnessContribution {
                row: 1,
                col: 1,
                value: 2.0,
            },
        ];
        for (row, col) in [(0, 1), (1, 0)] {
            for value in [1e16, 1.0, 1e-16, -1.0, -1e16] {
                entries.push(StiffnessContribution { row, col, value });
            }
        }
        let mut s = system(&matrix, &force, &free, &[]);
        s.contributions = Some(&entries);
        let prepared = prepare_structural(&s).unwrap();
        assert!(prepared.assembly_relative_perturbation_estimate > 0.0);
        let entry = prepared
            .contribution_rounding
            .iter()
            .find(|e| e.row == 0 && e.col == 1)
            .unwrap();
        assert!(!entry.difference_expansion.is_empty());
        let exact_difference = Expansion {
            terms: entry.difference_expansion.clone(),
            operations: 0,
        };
        assert_eq!(exact_difference.exact_scalar().unwrap(), Some(1e-16));
        assert_eq!(
            contribution_sums(&s).unwrap().unwrap()[0][1]
                .exact_scalar()
                .unwrap(),
            Some(1e-16)
        );
    }
    #[test]
    fn krev04_witness_helpers_reject_internally_corrupted_shape_without_panic() {
        let matrix = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
        let force = [1.0, 1.0];
        let free = [0, 1];
        let s = system(&matrix, &force, &free, &[]);
        let mut prepared = prepare_structural(&s).unwrap();
        prepared.matrix[0].clear();
        assert!(
            std::panic::catch_unwind(|| verify_negative_direction(&prepared, &[1.0, -1.0]))
                .unwrap()
                .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| negative_pair_witness(&prepared))
                .unwrap()
                .is_err()
        );
        let mut prepared = prepare_structural(&s).unwrap();
        prepared.scale_exponents.clear();
        assert!(
            std::panic::catch_unwind(|| verify_negative_direction(&prepared, &[1.0, -1.0]))
                .unwrap()
                .is_err()
        );
        let mut prepared = prepare_structural(&s).unwrap();
        let bad_source = system(&matrix, &force, &[0], &[]);
        prepared.source = &bad_source;
        assert!(
            std::panic::catch_unwind(|| negative_pair_witness(&prepared))
                .unwrap()
                .is_err()
        );
    }
    #[test]
    fn krev05_completion_requires_positive_source_bound_factor() {
        let matrix = vec![vec![1.0, 2.0], vec![2.0, 1.0]];
        let force = [1.0, 1.0];
        let free = [0, 1];
        let s = system(&matrix, &force, &free, &[]);
        let prepared = prepare_structural(&s).unwrap();
        assert!(factor_structural_cholesky(&prepared).is_err());
        assert!(factor_structural_profile(&prepared, &[0, 1], &[0, 0]).is_err());
        // Neither path can produce the opaque value required by finish_structural.
        let positive = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
        let ps = system(&positive, &force, &free, &[]);
        let pp = prepare_structural(&ps).unwrap();
        let factor = factor_structural_cholesky(&pp).unwrap();
        let result = finish_structural(&factor).unwrap();
        assert!((result.displacements[0] - 0.5).abs() < 1e-15);
        assert!((result.displacements[1] - 1.0 / 3.0).abs() < 1e-15);
        assert!(factor_structural_profile(&prepared, &[0, 0], &[0, 0]).is_err());
        assert!(factor_structural_profile(&prepared, &[0, 1], &[0, 1]).is_err());
    }
}
