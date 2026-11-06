//! The dense modal solve on the modal pencil (DEL-04-09 Design §3.2, §1.4,
//! §1.5 and §3.4 checks 4 and 5): `modal_eigen::dense::solve_dense_pencil`
//! on (K_ff, M_ff), the finite-mode census, the reported set with its cluster
//! completion, the Sturm completeness count at the final shift, and the
//! normalization and participation of the reported modes.
//!
//! **Massless DOFs.** The kernel condenses only the structurally massless
//! (coordinate, structural-zero) free DOFs, and establishes the finite-mode
//! count only under its premises: the equilibrated remaining mass block
//! passes Cholesky with every pivot at least τ_m, and every condensed μ is
//! classified finite above both hard floors (τ_μ·μ_max and κ_μ·δ_q), with
//! the band below κ_μ·δ_μ recovered only under the τ_f and τ_e screens. A
//! general or non-coordinate mass nullspace leaves the census
//! [`FiniteCensus::Unresolved`]. Nothing here truncates a mass rank, rescues
//! a rejected μ or drops a mode by a tolerance.
//!
//! **The reported set** is `clusters::reported_set` under the request's bound
//! (count and/or inclusive cut-off, whichever stops first), extended to
//! complete the cluster at the boundary and disclosed by count. A set is
//! completed only in one of the forms of [`ReportedEndpoint`]: below a
//! completed computed cluster (the final shift lies in the gap above it),
//! empty below the lowest computed mode, or the whole finite spectrum under
//! an **established** census. When the bound needs the whole-spectrum
//! endpoint and the census is unresolved, the top computed cluster may be
//! partial, so the request is refused (`MODAL_FINITE_COUNT_UNRESOLVED`) and
//! no set is formed, even internally.
//!
//! **Completeness** is the Sturm count at the final shift, under the
//! conditioning and separation premises and the growth limit
//! (`sturm::count_in_gap`, or `count_above_spectrum` for the whole
//! spectrum); it must equal the modes reported. A shift that cannot be
//! placed refuses by name, and so does a disagreeing count. These are
//! numerical-integrity checks under stated premises, not bounds.
use super::integrity::{
    modal_range_refusal, pencil_range_text, ModalIntegrityParameters, MODAL_EIGENSOLVE_UNAVAILABLE,
    MODAL_FINITE_COUNT_UNRESOLVED, MODAL_NO_FINITE_MODES, MODAL_STURM_COUNT_MISMATCH, MODAL_STURM_GROWTH_LIMIT,
    MODAL_STURM_SHIFT_ILL_CONDITIONED, MODAL_STURM_SHIFT_UNSEPARATED, MODAL_RESOURCE_ARITHMETIC_OVERFLOW,
    MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED, MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED,
};
use super::request::RequestInputs;
use super::stiffness::ModalPencil;
use crate::{diag, Diagnostic};
use open_pipe_stress_modal_eigen::clusters::{reported_set, ExtractionBound, ReportedSet, StoppingBound};
use open_pipe_stress_modal_eigen::dense::{solve_dense_pencil_with_allotments, DenseAllotments, DenseEigenSolution, DenseError, DenseOptions};
use open_pipe_stress_modal_eigen::modal_quantities::{normalize_mode, ModalBasis, ModalQuantityError, NormalizedMode};
use open_pipe_stress_modal_eigen::sturm::{
    count_above_spectrum, count_in_gap, Gap, GapLower, Neighbour, ShiftPencil, SturmEvidence, SturmOutcome, SturmRefusal,
};

/// The kernel's dense options from the validated parameters.
pub(crate) fn modal_dense_options(params: &ModalIntegrityParameters) -> DenseOptions {
    DenseOptions {
        infinite_mu_relative_threshold: params.infinite_mu_relative_threshold,
        mass_pivot_relative_threshold: params.mass_pivot_relative_threshold,
        resolution_floor_constant: params.resolution_floor_constant,
        resolution_floor_multiple: params.resolution_floor_multiple,
        formed_backward_error_multiple: params.formed_backward_error_multiple,
        screen_error_estimate_threshold: params.screen_error_estimate_threshold,
        max_ql_iterations_per_eigenvalue: params.max_ql_iterations_per_eigenvalue,
    }
}

fn eigensolve_refusal(what: String) -> Vec<Diagnostic> {
    vec![diag("diagnostic:modal:eigensolve", MODAL_EIGENSOLVE_UNAVAILABLE, "blocking", what, vec!["model".into()])]
}

/// The dense eigensolve, its refusals mapped by name.
pub(crate) fn solve_modal_pencil(pencil: &ModalPencil, params: &ModalIntegrityParameters) -> Result<DenseEigenSolution, Vec<Diagnostic>> {
    solve_modal_pencil_within(pencil, params, &DenseAllotments::default())
}

/// [`solve_modal_pencil`] under the kernel's S1 allotments (design §5.3.8):
/// with `DenseAllotments::default()` the kernel runs as `solve_dense_pencil`
/// does. An exceeded allotment or a resource overflow refuses by the
/// kernel's own `MODAL_RESOURCE_*` name, distinct from the numerical ones.
pub(crate) fn solve_modal_pencil_within(
    pencil: &ModalPencil,
    params: &ModalIntegrityParameters,
    allotments: &DenseAllotments,
) -> Result<DenseEigenSolution, Vec<Diagnostic>> {
    solve_dense_pencil_with_allotments(&pencil.stiffness, &pencil.mass, &modal_dense_options(params), allotments).map_err(|error| match error {
        DenseError::QlRotationAllotmentExceeded { .. } | DenseError::BandPairAllotmentExceeded { .. } | DenseError::ResourceArithmeticOverflow { .. } => {
            let code = match error {
                DenseError::QlRotationAllotmentExceeded { .. } => MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED,
                DenseError::BandPairAllotmentExceeded { .. } => MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED,
                _ => MODAL_RESOURCE_ARITHMETIC_OVERFLOW,
            };
            vec![diag(
                "diagnostic:modal:resource:kernel",
                code,
                "blocking",
                format!("{error}; the dense eigensolve is refused before that work, with no partial result (a resource-budget refusal, not a numerical one)"),
                vec!["model".into()],
            )]
        }
        DenseError::PencilRangeUnsupported(range) => vec![modal_range_refusal(
            "pencil-range",
            pencil_range_text(&range, &pencil.labels),
            vec![pencil.labels[range.row].clone(), pencil.labels[range.col].clone()],
        )],
        DenseError::ReductionOutOfRange { .. } | DenseError::ResultOutOfRange { .. } => vec![modal_range_refusal(
            "eigensolve-range",
            format!("{error}; the dense reduction left the binary64 range, and no non-finite value is classified or published"),
            vec!["model".into()],
        )],
        DenseError::NonPositivePivot { index, .. } => eigensolve_refusal(format!(
            "{error} (at {}); K_ff is not symmetric positive definite, so the pencil is refused",
            pencil.labels.get(index).map_or("?", String::as_str)
        )),
        other => eigensolve_refusal(format!("the dense eigensolve refused the pencil: {other}")),
    })
}

/// The finite-mode census (module docs).
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FiniteCensus {
    /// The count is established: n minus the structurally massless DOFs, with
    /// every condensed μ classified finite.
    Established { finite_mode_count: usize, structurally_massless: usize },
    /// Not established; the computed spectrum's top cluster may be partial.
    Unresolved {
        structurally_massless: usize,
        /// Condensed μ ≤ 0 and positive μ rejected by a hard floor or the band
        /// screen: classified infinite, not returned.
        nonpositive_mu: usize,
        rejected_mu: usize,
        /// Whether the equilibrated mass block passed its τ_m Cholesky.
        mass_block_established: bool,
    },
}

impl FiniteCensus {
    pub(crate) fn of_solution(solution: &DenseEigenSolution) -> Self {
        match solution.established_finite_mode_count {
            Some(count) if !solution.finite_count_unresolved && count == solution.mode_count() => Self::Established {
                finite_mode_count: count,
                structurally_massless: solution.structurally_massless_count,
            },
            established => Self::Unresolved {
                structurally_massless: solution.structurally_massless_count,
                nonpositive_mu: solution.nonpositive_mu_count,
                rejected_mu: solution.below_threshold_mu_count,
                mass_block_established: established.is_some(),
            },
        }
    }
}

/// How a completed reported set ends (module docs). There is no form for a
/// partial cluster or an unresolved whole spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReportedEndpoint {
    /// The set ends below a completed computed cluster that starts at
    /// computed mode `next_mode` (0-based); the final shift lies in the gap.
    BelowComputedCluster { next_mode: usize },
    /// The cut-off lies below the lowest computed frequency: nothing is
    /// reported, and the count at a shift between them must be 0.
    BelowLowestMode,
    /// The whole finite spectrum, under an established census.
    WholeFiniteSpectrum { established_finite_mode_count: usize },
}

/// A completed reported set: only [`complete_reported_set`] forms one.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CompletedSet {
    pub(crate) set: ReportedSet,
    pub(crate) endpoint: ReportedEndpoint,
}

/// The request's extraction bound for the kernel.
pub(crate) fn modal_extraction_bound(inputs: &RequestInputs) -> Option<ExtractionBound> {
    match (inputs.mode_count, inputs.frequency_cutoff_hz) {
        (Some(n), Some(f)) => Some(ExtractionBound::Both { mode_count: n as usize, frequency_cutoff: f }),
        (Some(n), None) => Some(ExtractionBound::ModeCount(n as usize)),
        (None, Some(f)) => Some(ExtractionBound::FrequencyCutoff(f)),
        (None, None) => None,
    }
}

/// The reported set and its endpoint, or a refusal by name (module docs).
pub(crate) fn complete_reported_set(
    frequencies: &[f64],
    cluster_ids: &[usize],
    bound: ExtractionBound,
    census: &FiniteCensus,
) -> Result<CompletedSet, Vec<Diagnostic>> {
    if frequencies.is_empty() {
        return Err(vec![diag(
            "diagnostic:modal:no-finite-modes",
            MODAL_NO_FINITE_MODES,
            "blocking",
            "the dense solve returned no finite mode (every condensed eigenvalue is classified infinite), so nothing is reported".to_string(),
            vec!["model".into()],
        )]);
    }
    let set = reported_set(frequencies, cluster_ids, bound).map_err(|error| {
        eigensolve_refusal(format!("the reported set cannot be formed over the computed frequencies ({error})"))
    })?;
    let total = frequencies.len();
    let endpoint = if set.count == 0 {
        ReportedEndpoint::BelowLowestMode
    } else if set.count < total {
        ReportedEndpoint::BelowComputedCluster { next_mode: set.count }
    } else {
        match census {
            FiniteCensus::Established { finite_mode_count, .. } if *finite_mode_count == total => {
                ReportedEndpoint::WholeFiniteSpectrum { established_finite_mode_count: *finite_mode_count }
            }
            _ => {
                let why = match set.stopped_by {
                    StoppingBound::SpectrumExhausted => "the bound reaches beyond the computed spectrum",
                    _ => "the bound stops in the computed spectrum's top cluster",
                };
                return Err(vec![diag(
                    "diagnostic:modal:finite-count-unresolved",
                    MODAL_FINITE_COUNT_UNRESOLVED,
                    "blocking",
                    format!("{why}, so the set would end at the whole finite spectrum, but the finite-mode count is not established ({census:?}): a general or non-coordinate mass nullspace, or a condensed eigenvalue classified infinite, leaves the top computed cluster possibly partial, and it is never published as complete. A bound that stops below a completed computed cluster is not affected"),
                    vec!["mode_count".into(), "frequency_cutoff".into()],
                )]);
            }
        }
    };
    Ok(CompletedSet { set, endpoint })
}

/// The completeness count and its evidence.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SturmRecord {
    /// The negative-pivot count at the accepted final shift.
    pub(crate) count: usize,
    /// The modes reported, cluster extensions included.
    pub(crate) reported: usize,
    pub(crate) evidence: SturmEvidence,
}

fn sturm_refusal(code: &str, message: String, refs: Vec<String>) -> Vec<Diagnostic> {
    vec![diag("diagnostic:modal:sturm", code, "blocking", message, refs)]
}

/// Check 4: the count at the final shift (module docs). `vectors[i]` is
/// computed mode i's vector over the free DOFs, `cluster_estimates[i]` its
/// cluster's e_λ, and `cutoff_eigenvalue` λ_c = (2πf_c)² for an empty set.
pub(crate) fn modal_sturm_count(
    pencil: &ModalPencil,
    completed: &CompletedSet,
    eigenvalues: &[f64],
    vectors: &[&[f64]],
    cluster_estimates: &[f64],
    cutoff_eigenvalue: Option<f64>,
    params: &ModalIntegrityParameters,
) -> Result<SturmRecord, Vec<Diagnostic>> {
    let shift_pencil = ShiftPencil::from_entries(pencil.free_dofs.len(), &pencil.stiffness_entries, &pencil.mass_entries)
        .map_err(|error| eigensolve_refusal(format!("the Sturm pencil cannot be formed ({error})")))?;
    let parameters = params.modal_sturm_parameters();
    let neighbour = |i: usize| Neighbour { eigenvalue: eigenvalues[i], relative_error_estimate: cluster_estimates[i], vector: vectors[i] };
    let reported = completed.set.count;
    let (outcome, sides) = match completed.endpoint {
        ReportedEndpoint::BelowComputedCluster { next_mode } => {
            let gap = Gap { lower: GapLower::Mode(neighbour(reported - 1)), upper: neighbour(next_mode) };
            (count_in_gap(&shift_pencil, &gap, &parameters), format!("modes {reported} and {}", next_mode + 1))
        }
        ReportedEndpoint::BelowLowestMode => {
            let Some(bound) = cutoff_eigenvalue else {
                return Err(eigensolve_refusal("an empty reported set without a cut-off".into()));
            };
            let gap = Gap { lower: GapLower::Bound(bound), upper: neighbour(0) };
            (count_in_gap(&shift_pencil, &gap, &parameters), "the cut-off and mode 1".to_string())
        }
        ReportedEndpoint::WholeFiniteSpectrum { established_finite_mode_count } => (
            count_above_spectrum(&shift_pencil, &neighbour(reported - 1), established_finite_mode_count, &parameters),
            format!("mode {reported} (the top of the established finite spectrum) and the shift above it"),
        ),
    };
    let refs = vec![format!("mode:{}", reported.max(1)), "sturm_count".to_string()];
    let outcome = outcome.map_err(|error| {
        sturm_refusal(
            MODAL_STURM_SHIFT_UNSEPARATED,
            format!("no final shift can be placed between {sides}: the Sturm placement refused its inputs ({error})"),
            refs.clone(),
        )
    })?;
    match outcome {
        SturmOutcome::Counted { count, evidence } if count == reported => Ok(SturmRecord { count, reported, evidence }),
        SturmOutcome::Counted { count, evidence } => {
            let sigma = evidence.accepted().map_or(f64::NAN, |c| c.sigma);
            Err(sturm_refusal(
                MODAL_STURM_COUNT_MISMATCH,
                format!("the Sturm count at the final shift σ = {sigma:e} rad^2/s^2 between {sides} is {count}, but {reported} modes would be reported; a count disagreement is never published"),
                refs,
            ))
        }
        SturmOutcome::Refused { reason, evidence } => {
            let (code, why) = match reason {
                SturmRefusal::Unseparated => (MODAL_STURM_SHIFT_UNSEPARATED, "no candidate shift satisfies the separation premise"),
                SturmRefusal::IllConditioned => (MODAL_STURM_SHIFT_ILL_CONDITIONED, "every separated candidate within the growth limit fails the conditioning premise e_b·ν ≤ 1/κ_c"),
                SturmRefusal::GrowthLimit => (MODAL_STURM_GROWTH_LIMIT, "every separated candidate exceeds the growth limit or is refused by the factorization's pivot guard"),
                SturmRefusal::OutOfRange => (MODAL_STURM_GROWTH_LIMIT, "every candidate's shifted matrix left the binary64 normal range in its formation, which is rejected as the pivot guard rejects a shift"),
            };
            let furthest = evidence.candidates.last().map_or_else(String::new, |c| {
                let product = c.factored.as_ref().map_or(f64::NAN, |f| f.conditioning_product);
                let ratio = c.factored.as_ref().and_then(|f| f.separation_ratio).unwrap_or(f64::NAN);
                format!("; last candidate σ = {:e}: {:?}, e_b·ν = {product:e}, separation ratio {ratio:e}", c.sigma, c.decision)
            });
            Err(sturm_refusal(
                code,
                format!("the final shift between {sides} cannot be placed under the premises: {why} ({} candidates, largest growth {:?}){furthest}; the set is refused, never published with an unchecked count", evidence.candidates.len(), evidence.largest_growth()),
                refs,
            ))
        }
    }
}

/// The normalization basis of the published modes (§1.5): M_ff, the free
/// DOFs, p_k and T_k, the bounding-box diagonal ℓ and τ_t.
pub(crate) fn modal_basis<'a>(
    pencil: &'a ModalPencil,
    bounding_box_diagonal: f64,
    params: &ModalIntegrityParameters,
) -> Result<ModalBasis<'a>, Vec<Diagnostic>> {
    ModalBasis::new(
        &pencil.mass,
        &pencil.free_labels,
        [&pencil.excitation[0], &pencil.excitation[1], &pencil.excitation[2]],
        pencil.total_mass,
        bounding_box_diagonal,
        params.no_translation_threshold,
    )
    .map_err(|error| quantity_refusal(None, error))
}

pub(crate) fn quantity_refusal(mode: Option<usize>, error: ModalQuantityError) -> Vec<Diagnostic> {
    let refs = mode.map_or_else(|| vec!["model".into()], |m| vec![format!("mode:{}", m + 1)]);
    vec![modal_range_refusal(
        "quantities",
        format!("the normalization or participation quantities cannot be published ({error}); a value that overflows or is NaN refuses, never a silent zero"),
        refs,
    )]
}

/// One reported mode's normalized shape and participation (§1.5).
pub(crate) fn normalize_modal_shape(basis: &ModalBasis<'_>, mode: usize, vector: &[f64]) -> Result<NormalizedMode, Vec<Diagnostic>> {
    normalize_mode(mode, vector, basis).map_err(|error| quantity_refusal(Some(mode), error))
}
