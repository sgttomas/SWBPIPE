//! The in-crate typed modal outcome (DEL-04-09 Design §1.5, §3.4) and the
//! dense entry `run_modal_dense` (increment I2b-1, dense path only; I2b-2
//! adds nonlinear supports at the named state case, `state`).
//!
//! The sequence, each step blocking:
//! 1. the integrity parameters are validated (no default, none pinned);
//! 2. the request is prepared (I2a: its request-level and static-path
//!    refusals, the state case's binding, Timoshenko refused by name); with
//!    nonlinear supports the state case is solved once in this path's mode
//!    (`dense_scrutiny`) and its confirmed converged states linearize them
//!    (I2b-2, `state`), giving the restrained set the pencil is formed on;
//! 3. the mass is formed on the self-weight basis and **the weight-basis
//!    identity runs first** (§2.5; its tolerance is the caller's);
//! 4. K_ff and M_ff on the static partition, after the dense guard and the
//!    static singularity screen (`stiffness`);
//! 5. the dense solve, the census, clusters (δ_c) and the reported set
//!    (`eigen`);
//! 6. check 1 (re-formation and formation difference), check 2 for every
//!    computed mode, the cluster-level e_λ, check 4 (the Sturm count at the
//!    final shift), the normalization and participation of the reported
//!    modes, and check 3 on their published shapes;
//! 7. the standing: a reported mode with η > τ_r is Sensitive, an
//!    orthogonality breach makes the whole set Sensitive, and everything
//!    else that fails refuses.
//!
//! **Not built here**: no public, native, headless, schema or publication
//! entry (O-8), no sparse path (the request carries no `solver_mode`; its
//! closed wire form refuses one as an undeclared member: this entry is the
//! `dense_scrutiny` path, and its state solve records that mode). §2.6's
//! member frequency f_e is computed and reported; the discretization factor
//! c is not adopted (O-5), so no `MODAL_MASS_DISCRETIZATION_COARSE` decision
//! is made.
use super::context::BoundCase;
use super::eigen::{
    complete_reported_set, modal_basis, modal_extraction_bound, modal_sturm_count, normalize_modal_shape, solve_modal_pencil,
    CompletedSet, FiniteCensus, SturmRecord,
};
use super::integrity::{
    modal_orthogonality, modal_reformation_operands, modal_residual, reform_modal_pencil, ModalIntegrityParameters,
};
use super::mass::{assemble_mass, MassSummary, MemberKind};
use super::prepare::prepare_modal;
use super::request::ModalPreviewRequest;
use super::state::{solve_modal_state_case, ModalStateCase};
use super::stiffness::{form_modal_pencil, DenseGuardRecord};
use super::weight_basis::{check_weight_basis, IdentityReport};
use crate::{Diagnostic, PreviewSolverMode};
use open_pipe_stress_modal_eigen::clusters::{cluster_ids, ExtractionBound, ReportedSet};
use open_pipe_stress_modal_eigen::dense::ScreenedMode;
use open_pipe_stress_modal_eigen::modal_quantities::{
    cumulative_effective_mass_fractions, frequency_from_eigenvalue, BelowNormalRange, NormalizationCase,
};
use open_pipe_stress_modal_eigen::orthogonality::MOrthogonality;
use open_pipe_stress_modal_eigen::reformation::{cluster_error_estimates, FormationDifference, ResidualEstimate};

/// The solver mode of this entry, the modal dense path: its state solve runs
/// and records it (Agent 0's D3); no sparse or other mode is substituted.
pub(crate) const MODAL_DENSE_PATH_MODE: PreviewSolverMode = PreviewSolverMode::DenseScrutiny;

/// The standing of a reported mode or of the whole set (§3.4).
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ModalStanding {
    /// Checks 1 to 6 hold within their stated thresholds and premises.
    ChecksPassed,
    /// Published, demoted, with the reasons.
    Sensitive(Vec<String>),
}

/// One computed mode's check-2 evidence, reported or not (nothing is
/// summarized away).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComputedModeEvidence {
    /// 0-based computed index, ascending λ.
    pub(crate) index: usize,
    pub(crate) eigenvalue: f64,
    pub(crate) cluster: usize,
    pub(crate) reported: bool,
    /// Evaluated on the published vector for a reported mode, and on the
    /// computed vector otherwise.
    pub(crate) residual: ResidualEstimate,
    /// Whether its μ was recovered in the band below κ_μ·δ_μ (§3.2 item 5);
    /// the band record itself is in `ModalIntegrity::screened`.
    pub(crate) band_recovered: bool,
}

/// One reported mode (§1.5), with its own unit case.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ModalMode {
    /// 1-based, ascending frequency.
    pub(crate) index: usize,
    pub(crate) cluster: usize,
    /// λ = ω², rad²/s².
    pub(crate) eigenvalue: f64,
    pub(crate) circular_frequency: f64,
    pub(crate) frequency: f64,
    pub(crate) period: f64,
    /// Which of the two normalization cases the shape is in: (a) unit
    /// maximum translation, (b) unit maximum rotation.
    pub(crate) normalization: NormalizationCase,
    /// The global DOF whose component is exactly +1.
    pub(crate) reference_dof: usize,
    /// The published shape over all global DOFs, restrained DOFs exactly 0.
    pub(crate) shape: Vec<f64>,
    /// The largest free translational and rotational magnitudes of the
    /// computed vector, before normalization.
    pub(crate) max_translation: f64,
    pub(crate) max_rotation: f64,
    /// kg in case (a), kg·m²·rad⁻² in case (b).
    pub(crate) generalized_mass: f64,
    pub(crate) participation: [f64; 3],
    /// kg.
    pub(crate) effective_mass: [f64; 3],
    /// `None` where undefined (T_k = 0), never 0 in its place.
    pub(crate) effective_mass_fraction: [Option<f64>; 3],
    pub(crate) cumulative_effective_mass_fraction: [Option<f64>; 3],
    /// Values published as 0 because they are nonzero but below the binary64
    /// normal range (shape indices are global DOFs).
    pub(crate) below_normal_range: BelowNormalRange,
    pub(crate) residual: ResidualEstimate,
    /// The cluster-level first-order e_λ (the largest over the cluster).
    pub(crate) cluster_error_estimate: f64,
    pub(crate) standing: ModalStanding,
}

/// A cluster of computed modes (§3.4 check 5).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ModalCluster {
    pub(crate) id: usize,
    /// 0-based computed indices.
    pub(crate) modes: std::ops::Range<usize>,
    pub(crate) error_estimate: f64,
    pub(crate) reported: bool,
}

/// §2.6's member frequency, reported with no threshold decision (O-5).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MemberDiscretization {
    pub(crate) pipe: String,
    pub(crate) length: f64,
    /// f_e = (4.7300²/2π)·√(EI/(m̄L⁴)), Hz, on the formed E·I (W-15 scale
    /// included) and the member's m̄.
    pub(crate) fixed_fixed_frequency: f64,
    /// f_e / f_max, f_max the highest reported frequency; `None` with no
    /// reported mode.
    pub(crate) ratio_to_highest_reported: Option<f64>,
}

/// The integrity evidence (§3.4, §1.5 "per result").
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ModalIntegrity {
    pub(crate) formation: FormationDifference,
    /// Check 2 for every computed mode, in computed order.
    pub(crate) computed: Vec<ComputedModeEvidence>,
    /// The band and hard-floor classifications of the dense solve.
    pub(crate) screened: Vec<ScreenedMode>,
    pub(crate) orthogonality: MOrthogonality,
    pub(crate) sturm: SturmRecord,
    pub(crate) census: FiniteCensus,
    /// The dense solve's resolution observations: μ_max, δ_μ, δ_q, the
    /// thresholds and the smallest Cholesky pivot ratio.
    pub(crate) mu_max: f64,
    pub(crate) resolution_floor: f64,
    pub(crate) ql_resolution_floor: f64,
    pub(crate) hard_classification_threshold: f64,
    pub(crate) classification_threshold: f64,
    pub(crate) formed_backward_error_threshold: f64,
    pub(crate) min_cholesky_pivot_ratio: f64,
    /// The thresholds the standing was judged against (the caller's).
    pub(crate) residual_threshold: f64,
    pub(crate) orthogonality_threshold: f64,
    pub(crate) cluster_tolerance: f64,
}

/// The modal outcome of one request (module docs).
#[derive(Debug, Clone)]
pub(crate) struct ModalOutcome {
    pub(crate) bound: ExtractionBound,
    /// The stop reason, the count by the bound and the cluster extension.
    pub(crate) reported: ReportedSet,
    pub(crate) completed: CompletedSet,
    pub(crate) modes: Vec<ModalMode>,
    pub(crate) clusters: Vec<ModalCluster>,
    pub(crate) integrity: ModalIntegrity,
    pub(crate) standing: ModalStanding,
    pub(crate) mass_summary: MassSummary,
    /// R_k,fᵀ M_ff R_k,f, kg.
    pub(crate) free_dof_mass: [f64; 3],
    pub(crate) identity: IdentityReport,
    pub(crate) state_case: Option<BoundCase>,
    /// With nonlinear supports: the state case's mode, each support's state
    /// and modal treatment, and the linearized restrained set.
    pub(crate) nonlinear_state: Option<ModalStateCase>,
    pub(crate) exact_route: bool,
    pub(crate) guard: DenseGuardRecord,
    pub(crate) discretization: Vec<MemberDiscretization>,
    pub(crate) disclosures: Vec<String>,
    /// Non-blocking diagnostics of the preparation and the mass.
    pub(crate) diagnostics: Vec<Diagnostic>,
    /// The authoritative payload the I3a result document publishes (Design
    /// §5.3), copied from this run's own formed values: the model's node ids
    /// and their canonical coordinates (m), in node order.
    pub(crate) node_ids: Vec<String>,
    pub(crate) node_coordinates: Vec<[f64; 3]>,
    /// The free DOFs of the pencil, ascending global indices.
    pub(crate) free_dofs: Vec<usize>,
    /// The formed global mass M over all DOFs, each stored nonzero upper
    /// entry (row ≤ col, global indices) as the binary64 value the pencil,
    /// T_k and p_k were formed from, in node-block order.
    pub(crate) mass_upper_entries: Vec<(usize, usize, f64)>,
    /// ℓ, the bounding-box diagonal of every node (m), and τ_t, as the
    /// normalization used them.
    pub(crate) bounding_box_diagonal: f64,
    pub(crate) no_translation_threshold: f64,
    /// The request's a (m/s², normalized), when given.
    pub(crate) weight_to_mass_acceleration: Option<f64>,
}

/// The stored nonzero upper entries of the formed global mass, in node-block
/// order (row ≤ col): the values `form_modal_pencil` reads.
fn modal_mass_upper_entries(mass: &super::mass::ModalMass) -> Vec<(usize, usize, f64)> {
    let mut entries = Vec::new();
    for ((i, j), block) in mass.node_blocks() {
        for (a, line) in block.iter().enumerate() {
            for (b, &value) in line.iter().enumerate() {
                let (row, col) = (6 * i + a, 6 * j + b);
                if row <= col && value != 0.0 {
                    entries.push((row, col, value));
                }
            }
        }
    }
    entries
}

/// f_e of §2.6: 4.730040744862704 is the first fixed-fixed root βL.
const FIXED_FIXED_ROOT: f64 = 4.730040744862704;

fn bounding_box_diagonal(positions: &[[f64; 3]]) -> f64 {
    let mut low = [f64::INFINITY; 3];
    let mut high = [f64::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            low[k] = low[k].min(p[k]);
            high[k] = high[k].max(p[k]);
        }
    }
    let d = [high[0] - low[0], high[1] - low[1], high[2] - low[2]];
    open_pipe_stress_correct_norm::norm3(d[0], d[1], d[2])
}

/// The dense modal entry (module docs). Returns the outcome or every
/// blocking diagnostic of the step that refused.
pub(crate) fn run_modal_dense(req: &ModalPreviewRequest, params: &ModalIntegrityParameters) -> Result<ModalOutcome, Vec<Diagnostic>> {
    let refusals = params.modal_parameter_refusals();
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let prepared = prepare_modal(req)?;
    let nonlinear_state = solve_modal_state_case(&prepared, MODAL_DENSE_PATH_MODE)?;
    let mass = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration)?;
    let identity = check_weight_basis(&prepared, &mass, params.identity_tolerance)?;
    let restrained = nonlinear_state
        .as_ref()
        .map_or(&prepared.boundary.restrained_dofs[..], |state| &state.restrained_dofs[..]);
    let pencil = form_modal_pencil(&prepared, &mass, restrained, params.sturm_max_candidates)?;
    let solution = solve_modal_pencil(&pencil, params)?;
    let census = FiniteCensus::of_solution(&solution);
    let eigenvalues = solution.eigenvalues.clone();
    let total = eigenvalues.len();
    let ids = cluster_ids(&eigenvalues, params.cluster_tolerance)
        .map_err(|error| vec![super::integrity::modal_range_refusal("clusters", format!("the computed eigenvalues cannot be clustered ({error})"), vec!["model".into()])])?;
    let frequencies: Vec<f64> = eigenvalues.iter().map(|&l| frequency_from_eigenvalue(l).0).collect();
    let bound = modal_extraction_bound(&prepared.inputs).expect("a prepared request names its bound");
    let completed = complete_reported_set(&frequencies, &ids, bound, &census)?;
    let reported_count = completed.set.count;

    // Normalization and participation of the reported modes (§1.5).
    let positions: Vec<[f64; 3]> = prepared.built.nodes.iter().map(|n| n.coordinates).collect();
    let diagonal = bounding_box_diagonal(&positions);
    let basis = modal_basis(&pencil, diagonal, params)?;
    let mut normalized = Vec::with_capacity(reported_count);
    for mode in 0..reported_count {
        normalized.push(normalize_modal_shape(&basis, mode, solution.vector(mode))?);
    }

    // Check 1, then check 2 for every computed mode.
    let operands = modal_reformation_operands(&prepared, &mass)?;
    let (reformed, formation) = reform_modal_pencil(
        &operands,
        prepared.exact,
        pencil.free_dofs.len(),
        &pencil.dof_map,
        &pencil.stiffness_entries,
        &pencil.mass_entries,
    )?;
    let vectors: Vec<&[f64]> =
        (0..total).map(|i| if i < reported_count { normalized[i].shape.as_slice() } else { solution.vector(i) }).collect();
    let mut residuals = Vec::with_capacity(total);
    for (i, vector) in vectors.iter().enumerate() {
        residuals.push(modal_residual(&reformed, &formation, i, eigenvalues[i], vector).map_err(|d| vec![d])?);
    }
    let estimates: Vec<f64> = residuals.iter().map(|r| r.error_estimate).collect();
    let per_cluster = cluster_error_estimates(&estimates, &ids).map_err(|error| {
        vec![super::integrity::modal_range_refusal("cluster-estimates", format!("the cluster error estimates cannot be formed ({error:?})"), vec!["model".into()])]
    })?;
    let mode_estimates: Vec<f64> = ids.iter().map(|&id| per_cluster[id]).collect();

    // Check 4 at the final shift.
    let cutoff_eigenvalue = prepared.inputs.frequency_cutoff_hz.map(|f| {
        let omega = 2.0 * std::f64::consts::PI * f;
        omega * omega
    });
    let sturm = modal_sturm_count(&pencil, &completed, &eigenvalues, &vectors, &mode_estimates, cutoff_eigenvalue, params)?;

    // Check 3 on the published shapes.
    let published: Vec<&[f64]> = normalized.iter().map(|m| m.shape.as_slice()).collect();
    let orthogonality = modal_orthogonality(&reformed, &published).map_err(|d| vec![d])?;

    // Standing and the published rows.
    let band: Vec<bool> = {
        let recovered: Vec<f64> = solution.screened.iter().filter(|s| s.retained).filter_map(|s| s.recovery.as_ref().map(|r| r.eigenvalue)).collect();
        eigenvalues.iter().map(|l| recovered.iter().any(|r| r.to_bits() == l.to_bits())).collect()
    };
    let computed: Vec<ComputedModeEvidence> = residuals
        .iter()
        .enumerate()
        .map(|(i, residual)| ComputedModeEvidence {
            index: i,
            eigenvalue: eigenvalues[i],
            cluster: ids[i],
            reported: i < reported_count,
            residual: *residual,
            band_recovered: band[i],
        })
        .collect();
    let cumulative = cumulative_effective_mass_fractions(&normalized);
    let mut set_reasons = Vec::new();
    let mut modes = Vec::with_capacity(reported_count);
    for (i, mode) in normalized.into_iter().enumerate() {
        let (frequency, circular_frequency, period) = frequency_from_eigenvalue(eigenvalues[i]);
        let residual = residuals[i];
        let standing = if residual.backward_error > params.residual_threshold {
            let reason = format!(
                "mode {}: residual backward error η = {:e} exceeds τ_r = {:e}",
                i + 1,
                residual.backward_error,
                params.residual_threshold
            );
            set_reasons.push(reason.clone());
            ModalStanding::Sensitive(vec![reason])
        } else {
            ModalStanding::ChecksPassed
        };
        let mut shape = vec![0.0; pencil.global_dimension];
        for (free, &dof) in pencil.free_dofs.iter().enumerate() {
            shape[dof] = mode.shape[free];
        }
        let mut below = mode.below_normal_range.clone();
        below.shape = below.shape.iter().map(|&free| pencil.free_dofs[free]).collect();
        modes.push(ModalMode {
            index: i + 1,
            cluster: ids[i],
            eigenvalue: eigenvalues[i],
            circular_frequency,
            frequency,
            period,
            normalization: mode.case,
            reference_dof: pencil.free_dofs[mode.reference_dof],
            shape,
            max_translation: mode.max_translation,
            max_rotation: mode.max_rotation,
            generalized_mass: mode.generalized_mass,
            participation: mode.participation,
            effective_mass: mode.effective_mass,
            effective_mass_fraction: mode.effective_mass_fraction,
            cumulative_effective_mass_fraction: cumulative[i],
            below_normal_range: below,
            residual,
            cluster_error_estimate: mode_estimates[i],
            standing,
        });
    }
    if orthogonality.max_ratio > params.orthogonality_threshold {
        set_reasons.push(format!(
            "M-orthogonality deviation {:e} of modes {:?} exceeds τ_o = {:e}",
            orthogonality.max_ratio,
            orthogonality.pair.map(|(a, b)| (a + 1, b + 1)),
            params.orthogonality_threshold
        ));
    }
    let standing = if set_reasons.is_empty() { ModalStanding::ChecksPassed } else { ModalStanding::Sensitive(set_reasons) };

    let clusters: Vec<ModalCluster> = per_cluster
        .iter()
        .enumerate()
        .map(|(id, &error_estimate)| {
            let start = ids.iter().position(|&c| c == id).expect("every cluster id has a member");
            let end = ids.iter().rposition(|&c| c == id).expect("every cluster id has a member") + 1;
            ModalCluster { id, modes: start..end, error_estimate, reported: start < reported_count }
        })
        .collect();

    // §2.6: f_e per straight member, with no threshold decision (O-5).
    let highest = modes.last().map(|m| m.frequency);
    let sections: std::collections::HashMap<&str, &open_pipe_stress_straight_pipe::StraightPipeElement> =
        prepared.built.pipes.iter().map(|p| (p.element_id.as_str(), p)).collect();
    let discretization: Vec<MemberDiscretization> = mass
        .members
        .iter()
        .filter(|m| matches!(m.kind, MemberKind::Straight(_)))
        .filter_map(|m| {
            let pipe = sections.get(m.pipe.as_str())?;
            let (a, b) = (positions[m.node_i], positions[m.node_j]);
            let length = open_pipe_stress_correct_norm::norm3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
            let ei = pipe.section.elastic_modulus * pipe.section.second_moment_y;
            let f_e = FIXED_FIXED_ROOT * FIXED_FIXED_ROOT / (2.0 * std::f64::consts::PI) * (ei / (m.mass_per_length * length.powi(4))).sqrt();
            Some(MemberDiscretization {
                pipe: m.pipe.clone(),
                length,
                fixed_fixed_frequency: f_e,
                ratio_to_highest_reported: highest.map(|f| f_e / f),
            })
        })
        .collect();

    let mut disclosures = modal_disclosures(&prepared, &mass, &pencil, &completed, &census);
    if let Some(state) = &nonlinear_state {
        disclosures.extend(state.disclosures.iter().cloned());
    }
    if let Some(lambda_c) = cutoff_eigenvalue {
        for (i, &lambda) in eigenvalues.iter().enumerate() {
            let e = mode_estimates[i];
            if lambda * (1.0 - e) <= lambda_c && lambda_c <= lambda * (1.0 + e) {
                disclosures.push(format!(
                    "computed mode {} ({}) has a first-order eigenvalue error estimate e_λ = {e:e} that reaches across the cut-off",
                    i + 1,
                    if i < reported_count { "reported" } else { "not reported" }
                ));
            }
        }
    }
    let mut diagnostics = prepared.diagnostics.clone();
    diagnostics.extend(mass.warnings.iter().cloned());
    Ok(ModalOutcome {
        bound,
        reported: completed.set,
        completed,
        modes,
        clusters,
        integrity: ModalIntegrity {
            formation,
            computed,
            screened: solution.screened.clone(),
            orthogonality,
            sturm,
            census,
            mu_max: solution.mu_max,
            resolution_floor: solution.resolution_floor,
            ql_resolution_floor: solution.ql_resolution_floor,
            hard_classification_threshold: solution.hard_classification_threshold,
            classification_threshold: solution.classification_threshold,
            formed_backward_error_threshold: solution.formed_backward_error_threshold,
            min_cholesky_pivot_ratio: solution.min_cholesky_pivot_ratio,
            residual_threshold: params.residual_threshold,
            orthogonality_threshold: params.orthogonality_threshold,
            cluster_tolerance: params.cluster_tolerance,
        },
        standing,
        mass_summary: mass.summary.clone(),
        free_dof_mass: pencil.free_dof_mass,
        identity,
        state_case: prepared.bound_case.clone(),
        nonlinear_state,
        exact_route: prepared.exact,
        guard: pencil.guard.clone(),
        discretization,
        disclosures,
        diagnostics,
        node_ids: prepared.model.nodes.iter().map(|node| node.id.clone()).collect(),
        node_coordinates: positions,
        free_dofs: pencil.free_dofs.clone(),
        mass_upper_entries: modal_mass_upper_entries(&mass),
        bounding_box_diagonal: diagonal,
        no_translation_threshold: params.no_translation_threshold,
        weight_to_mass_acceleration: prepared.inputs.weight_to_mass_acceleration,
    })
}

/// The fixed disclosures of §1.2, §1.3, §1.5 and §3.4 that apply.
fn modal_disclosures(
    prepared: &super::prepare::Prepared,
    mass: &super::mass::ModalMass,
    pencil: &super::stiffness::ModalPencil,
    completed: &CompletedSet,
    census: &FiniteCensus,
) -> Vec<String> {
    let model = &prepared.model;
    let mut out = vec![
        "numerical integrity does not establish physical correctness: the residual is evaluated against a double-double re-formation of the stiffness and mass, the eigenvalue error is a first-order estimate, and the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues; none of these bounds the eigenvalues or modelling, discretization or input error".to_string(),
        "contents are in all three translational masses and excluded from torsional and rotary inertia (D-e)".to_string(),
        "loads, temperatures and pressures do not enter the eigenproblem".to_string(),
    ];
    if model.supports.iter().any(crate::is_constant_effort_support) {
        out.push("constant-effort supports contribute zero stiffness and zero mass".into());
    }
    if !prepared.built.connectors.is_empty() {
        out.push("objective connectors and their tie rods are massless; a connector's replaced span carries no mass".into());
    }
    if !prepared.boundary.imposed_displacements.is_empty() {
        out.push("imposed-displacement DOFs stay rigidly restrained; their motion values do not enter".into());
    }
    if prepared.exact {
        out.push("exact route: pressure stiffening is not modelled; the mass is on the route's own basis (EXACT_MASS_METHOD)".into());
    }
    if !mass.bodies.is_empty() {
        out.push("component bodies are point masses W/a at their centre of gravity; their own rotary inertia is not authored and is zero".into());
    }
    let shortfall: Vec<String> = (0..3)
        .filter(|&k| pencil.free_dof_mass[k] != pencil.total_mass[k])
        .map(|k| format!("{}: total {:e} kg, free-DOF {:e} kg", ["X", "Y", "Z"][k], pencil.total_mass[k], pencil.free_dof_mass[k]))
        .collect();
    if !shortfall.is_empty() {
        out.push(format!("mass at or coupled to restrained DOFs moves with the supports; fractions are relative to the total mass T_k ({})", shortfall.join("; ")));
    }
    if pencil.stiffness_asymmetric_entries > 0 {
        out.push(format!("{} formed stiffness positions differ between triangles in their bits; K_ff takes the lower one", pencil.stiffness_asymmetric_entries));
    }
    if completed.set.extension_count > 0 {
        out.push(format!(
            "{} modes selected by the bound; {} modes added to complete a cluster",
            completed.set.bounded_count, completed.set.extension_count
        ));
    }
    if let FiniteCensus::Unresolved { .. } = census {
        out.push(format!("the finite-mode count is not established ({census:?}); the set ends below a completed computed cluster, and the whole-spectrum endpoint is not used"));
    }
    out
}
