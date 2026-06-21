//! Code-neutral solver diagnostics.
//!
//! This crate turns mechanics-solver findings into deterministic diagnostic
//! records. It does not encode design-code checks, professional approval,
//! protected standards content, or private project data.

use open_pipe_stress_frame_kernel::{
    CanonicalModelReference, FrameKernelError, QuantityUnitMetadata,
};
use open_pipe_stress_linear_supports::{
    FindingCode as SupportFindingCode, LinearSupportError, SupportApplicationError, SupportFinding,
};
use open_pipe_stress_primitive_loads::{
    FindingCode as LoadFindingCode, LoadFinding, PrimitiveLoadError,
};
use open_pipe_stress_sparse_direct::{FactorizationReport, SparseDirectError};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSource {
    ModelValidation,
    MechanicsSolver,
    SolverConfiguration,
    SolverIteration,
}

impl DiagnosticSource {
    pub fn analysis_boundary_source(self) -> &'static str {
        match self {
            Self::ModelValidation => "model_validation",
            Self::MechanicsSolver | Self::SolverConfiguration | Self::SolverIteration => {
                "mechanics_solver"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverDiagnosticClass {
    SolveBlocking,
    AssumptionWarning,
    NonlinearWarning,
    Tbd,
}

impl SolverDiagnosticClass {
    pub fn analysis_boundary_class(self) -> &'static str {
        match self {
            Self::SolveBlocking => "SOLVE_BLOCKING",
            Self::AssumptionWarning => "ASSUMPTION_WARNING",
            Self::NonlinearWarning => "NONLINEAR_WARNING",
            Self::Tbd => "TBD",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticProvenance {
    pub source_name: String,
    pub source_location: String,
    pub source_license: String,
    pub contributor: String,
    pub redistribution_status: String,
    pub review_status: String,
}

impl DiagnosticProvenance {
    pub fn solver_generated(source_location: impl Into<String>) -> Self {
        Self {
            source_name: "OpenPipeStress solver diagnostics".to_string(),
            source_location: source_location.into(),
            source_license: "project-governed".to_string(),
            contributor: "software".to_string(),
            redistribution_status: "public_project_record".to_string(),
            review_status: "TBD".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverStatus {
    ReadyToSolve,
    MechanicsSolved,
    SolvedWithWarnings,
    ModelIncomplete,
    SolveFailed,
}

impl SolverStatus {
    pub fn analysis_boundary_mapping(self) -> AnalysisBoundaryStatusMapping {
        match self {
            Self::ReadyToSolve | Self::ModelIncomplete | Self::SolveFailed => {
                AnalysisBoundaryStatusMapping {
                    mechanics_status: "MODEL_INCOMPLETE",
                    automatic_status: "MODEL_INCOMPLETE",
                    mechanics_authority: "solver_result_only",
                    human_review_required: true,
                    emits_rule_status: false,
                    emits_human_acceptance: false,
                    status_evidence_ref_required: !matches!(self, Self::ReadyToSolve),
                }
            }
            Self::MechanicsSolved | Self::SolvedWithWarnings => AnalysisBoundaryStatusMapping {
                mechanics_status: "MECHANICS_SOLVED",
                automatic_status: "MECHANICS_SOLVED",
                mechanics_authority: "solver_result_only",
                human_review_required: true,
                emits_rule_status: false,
                emits_human_acceptance: false,
                status_evidence_ref_required: true,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalysisBoundaryStatusMapping {
    pub mechanics_status: &'static str,
    pub automatic_status: &'static str,
    pub mechanics_authority: &'static str,
    pub human_review_required: bool,
    pub emits_rule_status: bool,
    pub emits_human_acceptance: bool,
    pub status_evidence_ref_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverDiagnosticCode {
    SingularSystem,
    IllConditionedSystem,
    ConditioningFailure,
    InvalidRestraint,
    InvalidModelTopology,
    InvalidNumericInput,
    NonConvergence,
    NonPositivePivot,
    SparseSolverTbd,
    TolerancePolicyTbd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolverDiagnostic {
    pub code: SolverDiagnosticCode,
    pub class: SolverDiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub source: DiagnosticSource,
    pub message: String,
    pub affected_ref: Option<String>,
    pub canonical_ref: Option<CanonicalModelReference>,
    pub remediation: Option<String>,
    pub provenance: DiagnosticProvenance,
    pub quantity_units: Vec<QuantityUnitMetadata>,
}

impl SolverDiagnostic {
    pub fn new(
        code: SolverDiagnosticCode,
        severity: DiagnosticSeverity,
        source: DiagnosticSource,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            class: diagnostic_class_for(code, severity, source),
            severity,
            source,
            message: message.into(),
            affected_ref: None,
            canonical_ref: None,
            remediation: Some(default_remediation(code, severity).to_string()),
            provenance: DiagnosticProvenance::solver_generated("core/solver/diagnostics"),
            quantity_units: Vec::new(),
        }
    }

    pub fn with_affected_ref(mut self, affected_ref: impl Into<String>) -> Self {
        self.affected_ref = Some(affected_ref.into());
        self
    }

    pub fn with_canonical_ref(mut self, canonical_ref: CanonicalModelReference) -> Self {
        self.canonical_ref = Some(canonical_ref);
        self
    }

    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    pub fn with_provenance(mut self, provenance: DiagnosticProvenance) -> Self {
        self.provenance = provenance;
        self
    }

    pub fn with_quantity_unit(mut self, quantity_unit: QuantityUnitMetadata) -> Self {
        self.quantity_units.push(quantity_unit);
        self
    }

    pub fn analysis_boundary_class(&self) -> &'static str {
        self.class.analysis_boundary_class()
    }

    pub fn analysis_boundary_source(&self) -> &'static str {
        self.source.analysis_boundary_source()
    }

    pub fn is_blocking(&self) -> bool {
        matches!(
            self.severity,
            DiagnosticSeverity::Blocking | DiagnosticSeverity::Failure
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolverDiagnosticReport {
    pub status: SolverStatus,
    pub diagnostics: Vec<SolverDiagnostic>,
}

impl SolverDiagnosticReport {
    pub fn new(status: SolverStatus, diagnostics: Vec<SolverDiagnostic>) -> Self {
        Self {
            status,
            diagnostics,
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.diagnostics.iter().any(SolverDiagnostic::is_blocking)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticsError {
    NonFiniteInput {
        name: &'static str,
        value: f64,
    },
    NegativeInput {
        name: &'static str,
        value: f64,
    },
    InvalidThresholds {
        warning_threshold: f64,
        failure_threshold: f64,
    },
}

impl fmt::Display for DiagnosticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NegativeInput { name, value } => {
                write!(f, "{name} must be nonnegative, got {value}")
            }
            Self::InvalidThresholds {
                warning_threshold,
                failure_threshold,
            } => write!(
                f,
                "conditioning thresholds must satisfy 1.0 <= warning <= failure, got {warning_threshold} and {failure_threshold}"
            ),
        }
    }
}

impl Error for DiagnosticsError {}

pub fn diagnostic_from_frame_error(error: &FrameKernelError) -> SolverDiagnostic {
    match error {
        FrameKernelError::SingularSystem { pivot } => SolverDiagnostic::new(
            SolverDiagnosticCode::SingularSystem,
            DiagnosticSeverity::Failure,
            DiagnosticSource::MechanicsSolver,
            format!("reduced stiffness system is singular at pivot {pivot}"),
        ),
        FrameKernelError::RepeatedRestrainedDof { dof } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("restrained degree of freedom {dof} is repeated"),
        ),
        FrameKernelError::RestrainedDofOutOfRange { dof, total_dofs } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("restrained degree of freedom {dof} is outside total DOF count {total_dofs}"),
        ),
        FrameKernelError::RepeatedPrescribedDof { dof } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("prescribed degree of freedom {dof} is repeated"),
        )
        .with_affected_ref(dof_ref(*dof)),
        FrameKernelError::PrescribedDofOutOfRange { dof, total_dofs } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("prescribed degree of freedom {dof} is outside total DOF count {total_dofs}"),
        )
        .with_affected_ref(dof_ref(*dof)),
        FrameKernelError::InvalidNodeIndex {
            node_index,
            node_count,
        } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("node index {node_index} is outside model node count {node_count}"),
        ),
        FrameKernelError::RepeatedElementNodeIndex { node_index } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("element connects node index {node_index} to itself"),
        ),
        FrameKernelError::DegenerateAxis { detail } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("degenerate local axis definition: {detail}"),
        ),
        FrameKernelError::InvalidOrientation { detail } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("invalid frame orientation: {detail}"),
        ),
        FrameKernelError::NonFiniteInput { name, value } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("{name} must be finite, got {value}"),
        ),
        FrameKernelError::NonPositiveInput { name, value } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("{name} must be positive, got {value}"),
        ),
        FrameKernelError::InvalidMatrixDimensions { rows, cols } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::MechanicsSolver,
            format!("stiffness matrix must be square, got {rows} by {cols}"),
        ),
        FrameKernelError::InvalidVectorLength { expected, actual } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::MechanicsSolver,
            format!("force vector length must be {expected}, got {actual}"),
        ),
    }
}

pub fn report_frame_error(error: &FrameKernelError) -> SolverDiagnosticReport {
    SolverDiagnosticReport::new(
        SolverStatus::SolveFailed,
        vec![diagnostic_from_frame_error(error)],
    )
}

pub fn diagnostic_from_linear_support_error(error: &LinearSupportError) -> SolverDiagnostic {
    SolverDiagnostic::new(
        SolverDiagnosticCode::InvalidNumericInput,
        DiagnosticSeverity::Blocking,
        DiagnosticSource::ModelValidation,
        error.to_string(),
    )
}

pub fn diagnostic_from_support_finding(finding: &SupportFinding) -> SolverDiagnostic {
    SolverDiagnostic::new(
        support_finding_diagnostic_code(finding.code),
        DiagnosticSeverity::Blocking,
        DiagnosticSource::ModelValidation,
        finding.message.clone(),
    )
    .with_affected_ref(finding.support_id.clone())
}

pub fn diagnostics_from_support_application_error(
    error: &SupportApplicationError,
) -> Vec<SolverDiagnostic> {
    match error {
        SupportApplicationError::FrameKernel(error) => vec![diagnostic_from_frame_error(error)],
        SupportApplicationError::BlockingSupportFindings { findings } => findings
            .iter()
            .map(diagnostic_from_support_finding)
            .collect(),
        SupportApplicationError::DofCountNotNodeAligned {
            total_dofs,
            dof_per_node,
        } => vec![SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!(
                "global DOF count {total_dofs} is not divisible by frame node DOF count {dof_per_node}"
            ),
        )],
        SupportApplicationError::SupportDofOutOfRange { dof, total_dofs } => {
            vec![SolverDiagnostic::new(
                SolverDiagnosticCode::InvalidRestraint,
                DiagnosticSeverity::Blocking,
                DiagnosticSource::ModelValidation,
                format!("support degree of freedom {dof} is outside total DOF count {total_dofs}"),
            )
            .with_affected_ref(dof_ref(*dof))]
        }
        SupportApplicationError::PreparedBoundaryMismatch { dof } => {
            vec![SolverDiagnostic::new(
                SolverDiagnosticCode::InvalidRestraint,
                DiagnosticSeverity::Blocking,
                DiagnosticSource::ModelValidation,
                format!("imposed displacement degree of freedom {dof} is not present in the prepared boundary"),
            )
            .with_affected_ref(dof_ref(*dof))]
        }
    }
}

pub fn diagnostic_from_primitive_load_error(error: &PrimitiveLoadError) -> SolverDiagnostic {
    SolverDiagnostic::new(
        SolverDiagnosticCode::InvalidNumericInput,
        DiagnosticSeverity::Blocking,
        DiagnosticSource::ModelValidation,
        error.to_string(),
    )
}

pub fn diagnostic_from_primitive_load_finding(finding: &LoadFinding) -> SolverDiagnostic {
    SolverDiagnostic::new(
        load_finding_diagnostic_code(finding.code),
        DiagnosticSeverity::Blocking,
        DiagnosticSource::ModelValidation,
        finding.message.clone(),
    )
    .with_affected_ref(load_finding_affected_ref(finding))
}

pub fn classify_condition_ratio(
    ratio: f64,
    warning_threshold: f64,
    failure_threshold: f64,
) -> Result<Option<SolverDiagnostic>, DiagnosticsError> {
    validate_nonnegative_finite("condition_ratio", ratio)?;
    validate_nonnegative_finite("warning_threshold", warning_threshold)?;
    validate_nonnegative_finite("failure_threshold", failure_threshold)?;

    if warning_threshold < 1.0 || failure_threshold < 1.0 || warning_threshold > failure_threshold {
        return Err(DiagnosticsError::InvalidThresholds {
            warning_threshold,
            failure_threshold,
        });
    }

    if ratio >= failure_threshold {
        return Ok(Some(SolverDiagnostic::new(
            SolverDiagnosticCode::ConditioningFailure,
            DiagnosticSeverity::Failure,
            DiagnosticSource::MechanicsSolver,
            format!("estimated condition ratio {ratio} meets or exceeds failure threshold {failure_threshold}"),
        )));
    }

    if ratio >= warning_threshold {
        return Ok(Some(SolverDiagnostic::new(
            SolverDiagnosticCode::IllConditionedSystem,
            DiagnosticSeverity::Warning,
            DiagnosticSource::MechanicsSolver,
            format!("estimated condition ratio {ratio} meets or exceeds warning threshold {warning_threshold}"),
        )));
    }

    Ok(None)
}

pub fn convergence_diagnostic(
    iteration_count: usize,
    max_iterations: usize,
    residual_norm: f64,
    tolerance: f64,
) -> Result<Option<SolverDiagnostic>, DiagnosticsError> {
    validate_nonnegative_finite("residual_norm", residual_norm)?;
    validate_nonnegative_finite("tolerance", tolerance)?;

    if residual_norm <= tolerance {
        return Ok(None);
    }

    if iteration_count >= max_iterations {
        return Ok(Some(SolverDiagnostic::new(
            SolverDiagnosticCode::NonConvergence,
            DiagnosticSeverity::Failure,
            DiagnosticSource::SolverIteration,
            format!(
                "solver did not converge after {iteration_count} iterations; residual {residual_norm} exceeds tolerance {tolerance}"
            ),
        )));
    }

    Ok(Some(SolverDiagnostic::new(
        SolverDiagnosticCode::NonConvergence,
        DiagnosticSeverity::Warning,
        DiagnosticSource::SolverIteration,
        format!(
            "solver residual {residual_norm} exceeds tolerance {tolerance} before the iteration limit"
        ),
    )))
}

/// Resolution-status diagnostic for the sparse solver adoption state.
///
/// The sparse solver *selection* is resolved by the human ruling `DEC-023`
/// (D-03 Option C: in-repo skyline LDL^T direct solver,
/// `core/solver/sparse_direct`). `DEC-050` binds an R4 sparse evidence lane;
/// what remains `TBD` is profile-direct assembly and default sparse promotion.
pub fn sparse_solver_tbd_diagnostic() -> SolverDiagnostic {
    SolverDiagnostic::new(
        SolverDiagnosticCode::SparseSolverTbd,
        DiagnosticSeverity::Warning,
        DiagnosticSource::SolverConfiguration,
        "sparse solver strategy is resolved by DEC-023 (in-repo skyline LDLT direct solver, core/solver/sparse_direct); DEC-050 sparse evidence lane is live; profile-direct assembly and default sparse promotion remain TBD",
    )
}

/// Maps an in-repo sparse skyline solver error into a solver diagnostic.
///
/// A singular pivot is reported with both its profile-ordered location and
/// the original (pre-ordering) reduced index so the failure can be traced
/// back to a model degree of freedom.
pub fn diagnostic_from_sparse_error(error: &SparseDirectError) -> SolverDiagnostic {
    match error {
        SparseDirectError::SingularPivot {
            ordered_index,
            original_index,
        } => SolverDiagnostic::new(
            SolverDiagnosticCode::SingularSystem,
            DiagnosticSeverity::Failure,
            DiagnosticSource::MechanicsSolver,
            format!(
                "sparse profile factorization found a singular pivot at ordered index {ordered_index} (original reduced index {original_index})"
            ),
        )
        .with_affected_ref(reduced_dof_ref(*original_index)),
        SparseDirectError::InvalidMatrixDimensions { rows, cols } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::MechanicsSolver,
            format!("sparse stiffness matrix must be square, got {rows} by {cols}"),
        ),
        SparseDirectError::InvalidVectorLength { expected, actual } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::MechanicsSolver,
            format!("sparse solve vector length must be {expected}, got {actual}"),
        ),
        SparseDirectError::NonFiniteInput { name, value } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("{name} must be finite, got {value}"),
        ),
        SparseDirectError::InvalidOrdering { detail } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::MechanicsSolver,
            format!("invalid sparse ordering: {detail}"),
        ),
    }
}

/// Maps a completed sparse factorization report into deterministic solver
/// diagnostics. Ordering is fixed: the nonpositive-pivot diagnostic (when
/// present) is emitted first and is currently the only report-derived
/// diagnostic; conditioning classification stays threshold-driven through
/// [`classify_condition_ratio`] because conditioning thresholds remain
/// governed by D-04.
pub fn diagnostics_from_factorization_report(
    report: &FactorizationReport,
) -> Vec<SolverDiagnostic> {
    let mut diagnostics = Vec::new();
    if report.nonpositive_pivot_count > 0 {
        let location_detail = match &report.first_nonpositive_pivot {
            Some(location) => format!(
                "; first at ordered index {} (original reduced index {})",
                location.ordered_index, location.original_index
            ),
            None => String::new(),
        };
        let mut diagnostic = SolverDiagnostic::new(
            SolverDiagnosticCode::NonPositivePivot,
            DiagnosticSeverity::Warning,
            DiagnosticSource::MechanicsSolver,
            format!(
                "sparse profile factorization accepted {} nonpositive pivot(s); the reduced stiffness is not positive definite{location_detail}",
                report.nonpositive_pivot_count
            ),
        );
        if let Some(location) = &report.first_nonpositive_pivot {
            diagnostic = diagnostic.with_affected_ref(reduced_dof_ref(location.original_index));
        }
        diagnostics.push(diagnostic);
    }
    diagnostics
}

pub fn tolerance_policy_tbd_diagnostic() -> SolverDiagnostic {
    SolverDiagnostic::new(
        SolverDiagnosticCode::TolerancePolicyTbd,
        DiagnosticSeverity::Warning,
        DiagnosticSource::SolverConfiguration,
        "solver tolerance policy remains TBD and must be accepted before release-quality performance claims",
    )
}

fn validate_nonnegative_finite(name: &'static str, value: f64) -> Result<(), DiagnosticsError> {
    if !value.is_finite() {
        return Err(DiagnosticsError::NonFiniteInput { name, value });
    }
    if value < 0.0 {
        return Err(DiagnosticsError::NegativeInput { name, value });
    }
    Ok(())
}

fn diagnostic_class_for(
    code: SolverDiagnosticCode,
    severity: DiagnosticSeverity,
    source: DiagnosticSource,
) -> SolverDiagnosticClass {
    match code {
        SolverDiagnosticCode::SparseSolverTbd | SolverDiagnosticCode::TolerancePolicyTbd => {
            SolverDiagnosticClass::Tbd
        }
        SolverDiagnosticCode::NonConvergence
            if source == DiagnosticSource::SolverIteration
                && severity == DiagnosticSeverity::Warning =>
        {
            SolverDiagnosticClass::NonlinearWarning
        }
        SolverDiagnosticCode::IllConditionedSystem => SolverDiagnosticClass::AssumptionWarning,
        _ if matches!(
            severity,
            DiagnosticSeverity::Blocking | DiagnosticSeverity::Failure
        ) =>
        {
            SolverDiagnosticClass::SolveBlocking
        }
        _ => SolverDiagnosticClass::AssumptionWarning,
    }
}

fn default_remediation(code: SolverDiagnosticCode, severity: DiagnosticSeverity) -> &'static str {
    match code {
        SolverDiagnosticCode::SparseSolverTbd => {
            "Complete profile-direct sparse assembly and governed default sparse-solver promotion before external performance reliance."
        }
        SolverDiagnosticCode::NonPositivePivot => {
            "Review restraint and stiffness definitions; a nonpositive pivot means the reduced stiffness system is not positive definite."
        }
        SolverDiagnosticCode::TolerancePolicyTbd => {
            "Bind the solve path to an accepted tolerance policy before release-quality claims."
        }
        SolverDiagnosticCode::NonConvergence => {
            "Review the nonlinear iteration inputs, active-set state, and accepted tolerance policy."
        }
        SolverDiagnosticCode::IllConditionedSystem | SolverDiagnosticCode::ConditioningFailure => {
            "Review model conditioning evidence before relying on mechanics results."
        }
        _ if matches!(
            severity,
            DiagnosticSeverity::Blocking | DiagnosticSeverity::Failure
        ) =>
        {
            "Correct the referenced model or solver input before relying on mechanics results."
        }
        _ => "Review the solver diagnostic before downstream use.",
    }
}

fn support_finding_diagnostic_code(code: SupportFindingCode) -> SolverDiagnosticCode {
    match code {
        SupportFindingCode::MissingSupportDof
        | SupportFindingCode::MissingSupportStiffness
        | SupportFindingCode::MissingImposedDisplacement
        | SupportFindingCode::InvalidSupportDof
        | SupportFindingCode::InvalidSupportDimension
        | SupportFindingCode::RepeatedRestrainedDof
        | SupportFindingCode::NodeOutOfRange => SolverDiagnosticCode::InvalidRestraint,
    }
}

fn load_finding_diagnostic_code(code: LoadFindingCode) -> SolverDiagnosticCode {
    match code {
        LoadFindingCode::MissingLoadMagnitude
        | LoadFindingCode::InvalidLoadDimension
        | LoadFindingCode::InvalidLoadDirection
        | LoadFindingCode::InvalidElementSpan
        | LoadFindingCode::InvalidElementProperties
        | LoadFindingCode::InvalidPhysicalProperty
        | LoadFindingCode::NonFiniteLoadMagnitude
        | LoadFindingCode::NonFiniteAxialEffect => SolverDiagnosticCode::InvalidNumericInput,
        LoadFindingCode::MissingLoadId
        | LoadFindingCode::MissingLoadTarget
        | LoadFindingCode::NodeOutOfRange
        | LoadFindingCode::ElementOutOfRange
        | LoadFindingCode::MissingElementSpan
        | LoadFindingCode::MissingElementProperties
        | LoadFindingCode::MissingPhysicalProperty
        | LoadFindingCode::InvalidElementConnectivity
        | LoadFindingCode::UnsupportedTargetForCategory => {
            SolverDiagnosticCode::InvalidModelTopology
        }
    }
}

fn load_finding_affected_ref(finding: &LoadFinding) -> String {
    if finding.code == LoadFindingCode::MissingLoadId || finding.load_id.trim().is_empty() {
        "load:<missing-id>".to_string()
    } else {
        finding.load_id.clone()
    }
}

fn dof_ref(dof: usize) -> String {
    format!("dof:{dof}")
}

fn reduced_dof_ref(reduced_dof: usize) -> String {
    format!("reduced-dof:{reduced_dof}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_linear_supports::{
        FindingCode as SupportFindingCode, SupportApplicationError, SupportFinding,
    };
    use open_pipe_stress_primitive_loads::{FindingCode as LoadFindingCode, LoadFinding};
    use open_pipe_stress_sparse_direct::PivotLocation;

    #[test]
    fn maps_singular_frame_error_to_failure_diagnostic() {
        let report = report_frame_error(&FrameKernelError::SingularSystem { pivot: 3 });

        assert_eq!(report.status, SolverStatus::SolveFailed);
        assert!(report.is_blocked());
        assert_eq!(
            report.diagnostics[0].code,
            SolverDiagnosticCode::SingularSystem
        );
        assert_eq!(report.diagnostics[0].severity, DiagnosticSeverity::Failure);
        assert_eq!(
            report.diagnostics[0].analysis_boundary_class(),
            "SOLVE_BLOCKING"
        );
        assert_eq!(
            report.diagnostics[0].analysis_boundary_source(),
            "mechanics_solver"
        );
        assert!(report.diagnostics[0].remediation.is_some());
        assert_eq!(
            report.diagnostics[0].provenance.source_location,
            "core/solver/diagnostics"
        );
    }

    #[test]
    fn maps_invalid_restraint_to_blocking_model_diagnostic() {
        let diagnostic =
            diagnostic_from_frame_error(&FrameKernelError::RepeatedRestrainedDof { dof: 2 });

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
    }

    #[test]
    fn maps_invalid_orientation_to_blocking_topology_diagnostic() {
        let diagnostic = diagnostic_from_frame_error(&FrameKernelError::InvalidOrientation {
            detail: "local axes are not orthonormal",
        });

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidModelTopology);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
        assert_eq!(diagnostic.analysis_boundary_class(), "SOLVE_BLOCKING");
        assert_eq!(diagnostic.analysis_boundary_source(), "model_validation");
        assert!(diagnostic
            .message
            .contains("local axes are not orthonormal"));
    }

    #[test]
    fn maps_prescribed_frame_dof_errors_to_blocking_restraint_diagnostics() {
        let repeated =
            diagnostic_from_frame_error(&FrameKernelError::RepeatedPrescribedDof { dof: 4 });
        let out_of_range =
            diagnostic_from_frame_error(&FrameKernelError::PrescribedDofOutOfRange {
                dof: 12,
                total_dofs: 12,
            });

        assert_eq!(repeated.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(repeated.severity, DiagnosticSeverity::Blocking);
        assert_eq!(repeated.source, DiagnosticSource::ModelValidation);
        assert_eq!(repeated.affected_ref.as_deref(), Some("dof:4"));
        assert_eq!(out_of_range.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(out_of_range.severity, DiagnosticSeverity::Blocking);
        assert_eq!(out_of_range.source, DiagnosticSource::ModelValidation);
        assert_eq!(out_of_range.affected_ref.as_deref(), Some("dof:12"));
    }

    #[test]
    fn maps_support_finding_to_blocking_model_validation_diagnostic() {
        let finding = SupportFinding {
            code: SupportFindingCode::MissingSupportStiffness,
            support_id: "spring-1".to_string(),
            message: "spring support requires explicit stiffness".to_string(),
        };

        let diagnostic = diagnostic_from_support_finding(&finding);

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("spring-1"));
        assert_eq!(diagnostic.analysis_boundary_class(), "SOLVE_BLOCKING");
        assert_eq!(diagnostic.analysis_boundary_source(), "model_validation");
    }

    #[test]
    fn maps_support_application_error_to_dof_blocking_diagnostic() {
        let diagnostics = diagnostics_from_support_application_error(
            &SupportApplicationError::SupportDofOutOfRange {
                dof: 18,
                total_dofs: 18,
            },
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostics[0].source, DiagnosticSource::ModelValidation);
        assert_eq!(diagnostics[0].affected_ref.as_deref(), Some("dof:18"));
    }

    #[test]
    fn maps_primitive_load_finding_to_blocking_model_validation_diagnostic() {
        let finding = LoadFinding {
            code: LoadFindingCode::MissingLoadMagnitude,
            load_id: "weight-1".to_string(),
            message: "primitive load requires an explicit magnitude".to_string(),
        };

        let diagnostic = diagnostic_from_primitive_load_finding(&finding);

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidNumericInput);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("weight-1"));
    }

    #[test]
    fn maps_missing_load_id_to_topology_diagnostic_with_stable_reference() {
        let finding = LoadFinding {
            code: LoadFindingCode::MissingLoadId,
            load_id: String::new(),
            message: "primitive load requires a stable load ID".to_string(),
        };

        let diagnostic = diagnostic_from_primitive_load_finding(&finding);

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidModelTopology);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
        assert_eq!(
            diagnostic.affected_ref.as_deref(),
            Some("load:<missing-id>")
        );
        assert_eq!(diagnostic.analysis_boundary_class(), "SOLVE_BLOCKING");
    }

    #[test]
    fn maps_axial_effect_missing_property_findings_to_topology_diagnostics() {
        let finding = LoadFinding {
            code: LoadFindingCode::MissingPhysicalProperty,
            load_id: "thermal-axial".to_string(),
            message: "thermal_expansion_coefficient is required".to_string(),
        };

        let diagnostic = diagnostic_from_primitive_load_finding(&finding);

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidModelTopology);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("thermal-axial"));
    }

    #[test]
    fn maps_axial_effect_invalid_property_findings_to_numeric_diagnostics() {
        let finding = LoadFinding {
            code: LoadFindingCode::NonFiniteAxialEffect,
            load_id: "thermal-overflow".to_string(),
            message: "computed axial force must be finite".to_string(),
        };

        let diagnostic = diagnostic_from_primitive_load_finding(&finding);

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidNumericInput);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("thermal-overflow"));
    }

    #[test]
    fn maps_repeated_element_node_to_topology_diagnostic() {
        let diagnostic = diagnostic_from_frame_error(&FrameKernelError::RepeatedElementNodeIndex {
            node_index: 4,
        });

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidModelTopology);
        assert!(diagnostic.message.contains("node index 4"));
    }

    #[test]
    fn condition_ratio_below_warning_returns_none() {
        let diagnostic = classify_condition_ratio(10.0, 100.0, 1000.0).unwrap();

        assert!(diagnostic.is_none());
    }

    #[test]
    fn condition_ratio_warning_is_nonblocking() {
        let diagnostic = classify_condition_ratio(150.0, 100.0, 1000.0)
            .unwrap()
            .unwrap();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::IllConditionedSystem);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Warning);
        assert_eq!(diagnostic.analysis_boundary_class(), "ASSUMPTION_WARNING");
        assert!(!diagnostic.is_blocking());
    }

    #[test]
    fn condition_ratio_failure_is_blocking() {
        let diagnostic = classify_condition_ratio(1200.0, 100.0, 1000.0)
            .unwrap()
            .unwrap();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::ConditioningFailure);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Failure);
        assert!(diagnostic.is_blocking());
    }

    #[test]
    fn condition_ratio_rejects_non_finite_input() {
        let error = classify_condition_ratio(f64::INFINITY, 100.0, 1000.0).unwrap_err();

        assert_eq!(
            error,
            DiagnosticsError::NonFiniteInput {
                name: "condition_ratio",
                value: f64::INFINITY
            }
        );
    }

    #[test]
    fn nonconvergence_after_iteration_limit_is_failure() {
        let diagnostic = convergence_diagnostic(20, 20, 1.0e-3, 1.0e-6)
            .unwrap()
            .unwrap();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::NonConvergence);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Failure);
    }

    #[test]
    fn converged_residual_returns_no_diagnostic() {
        let diagnostic = convergence_diagnostic(5, 20, 1.0e-8, 1.0e-6).unwrap();

        assert!(diagnostic.is_none());
    }

    #[test]
    fn sparse_solver_tbd_is_warning_not_failure() {
        let diagnostic = sparse_solver_tbd_diagnostic();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::SparseSolverTbd);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Warning);
        assert_eq!(diagnostic.analysis_boundary_class(), "TBD");
        assert!(!diagnostic.is_blocking());
    }

    #[test]
    fn sparse_solver_status_reflects_dec023_dec050_resolution_and_remaining_tbd() {
        let diagnostic = sparse_solver_tbd_diagnostic();

        assert!(diagnostic.message.contains("DEC-023"));
        assert!(diagnostic.message.contains("DEC-050"));
        assert!(diagnostic.message.contains("core/solver/sparse_direct"));
        assert!(diagnostic
            .message
            .contains("default sparse promotion remain TBD"));
        assert!(diagnostic
            .remediation
            .as_deref()
            .unwrap()
            .contains("profile-direct sparse assembly"));
    }

    #[test]
    fn maps_sparse_singular_pivot_to_failure_with_both_locations() {
        let diagnostic = diagnostic_from_sparse_error(&SparseDirectError::SingularPivot {
            ordered_index: 7,
            original_index: 3,
        });

        assert_eq!(diagnostic.code, SolverDiagnosticCode::SingularSystem);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Failure);
        assert_eq!(diagnostic.source, DiagnosticSource::MechanicsSolver);
        assert!(diagnostic.message.contains("ordered index 7"));
        assert!(diagnostic.message.contains("original reduced index 3"));
        assert_eq!(diagnostic.affected_ref.as_deref(), Some("reduced-dof:3"));
        assert!(diagnostic.is_blocking());
    }

    #[test]
    fn maps_sparse_invalid_inputs_to_blocking_numeric_diagnostics() {
        let dimensions =
            diagnostic_from_sparse_error(&SparseDirectError::InvalidMatrixDimensions {
                rows: 4,
                cols: 2,
            });
        assert_eq!(dimensions.code, SolverDiagnosticCode::InvalidNumericInput);
        assert_eq!(dimensions.severity, DiagnosticSeverity::Blocking);

        let length = diagnostic_from_sparse_error(&SparseDirectError::InvalidVectorLength {
            expected: 6,
            actual: 5,
        });
        assert_eq!(length.code, SolverDiagnosticCode::InvalidNumericInput);
        assert_eq!(length.severity, DiagnosticSeverity::Blocking);

        let ordering = diagnostic_from_sparse_error(&SparseDirectError::InvalidOrdering {
            detail: "ordering repeats an index",
        });
        assert_eq!(ordering.code, SolverDiagnosticCode::InvalidNumericInput);
        assert!(ordering.message.contains("ordering repeats an index"));

        let non_finite = diagnostic_from_sparse_error(&SparseDirectError::NonFiniteInput {
            name: "matrix entry",
            value: f64::INFINITY,
        });
        assert_eq!(non_finite.code, SolverDiagnosticCode::InvalidNumericInput);
        assert_eq!(non_finite.source, DiagnosticSource::ModelValidation);
    }

    #[test]
    fn nonpositive_pivot_report_yields_located_warning_diagnostic() {
        let report = FactorizationReport {
            dimension: 5,
            profile_entry_count: 9,
            max_half_bandwidth: 1,
            min_abs_pivot: Some(0.5),
            max_abs_pivot: Some(4.0),
            pivot_condition_ratio_estimate: Some(8.0),
            nonpositive_pivot_count: 2,
            first_nonpositive_pivot: Some(PivotLocation {
                ordered_index: 2,
                original_index: 4,
            }),
        };

        let diagnostics = diagnostics_from_factorization_report(&report);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, SolverDiagnosticCode::NonPositivePivot);
        assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(
            diagnostics[0].analysis_boundary_class(),
            "ASSUMPTION_WARNING"
        );
        assert!(!diagnostics[0].is_blocking());
        assert!(diagnostics[0].message.contains("2 nonpositive pivot(s)"));
        assert!(diagnostics[0].message.contains("ordered index 2"));
        assert!(diagnostics[0].message.contains("original reduced index 4"));
        assert_eq!(
            diagnostics[0].affected_ref.as_deref(),
            Some("reduced-dof:4")
        );
    }

    #[test]
    fn positive_definite_factorization_report_yields_no_diagnostics() {
        let report = FactorizationReport {
            dimension: 3,
            profile_entry_count: 5,
            max_half_bandwidth: 1,
            min_abs_pivot: Some(3.75),
            max_abs_pivot: Some(4.0),
            pivot_condition_ratio_estimate: Some(4.0 / 3.75),
            nonpositive_pivot_count: 0,
            first_nonpositive_pivot: None,
        };

        let diagnostics = diagnostics_from_factorization_report(&report);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn solver_status_maps_to_analysis_boundary_without_human_authority() {
        let solved = SolverStatus::SolvedWithWarnings.analysis_boundary_mapping();
        assert_eq!(solved.mechanics_status, "MECHANICS_SOLVED");
        assert_eq!(solved.automatic_status, "MECHANICS_SOLVED");
        assert_eq!(solved.mechanics_authority, "solver_result_only");
        assert!(solved.human_review_required);
        assert!(!solved.emits_rule_status);
        assert!(!solved.emits_human_acceptance);

        let failed = SolverStatus::SolveFailed.analysis_boundary_mapping();
        assert_eq!(failed.mechanics_status, "MODEL_INCOMPLETE");
        assert_eq!(failed.automatic_status, "MODEL_INCOMPLETE");
        assert!(failed.status_evidence_ref_required);
        assert!(!failed.emits_human_acceptance);
    }
}
