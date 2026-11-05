//! The numerical-integrity checks of a modal solve (DEL-04-09 Design §3.4,
//! checks 1 to 5) and the thresholds they are judged against.
//!
//! **Thresholds (DEC-026): none is pinned.** [`ModalIntegrityParameters`]
//! carries every threshold and algorithm parameter the dense solve and the
//! checks need as a required field. It has no `Default`, no constructor with
//! values and no production instance anywhere in the crate: a caller states
//! each value, and only the tests supply one (a labelled `TEST_LOCAL_…` set).
//! Each value is validated, finite and in its domain, and refused by name
//! otherwise (`MODAL_INTEGRITY_PARAMETER_INVALID`).
//!
//! **The checks** are numerical-integrity checks with stated premises, not
//! certified enclosures (§3.4, D-n). They run on the re-formed pencil:
//! - check 1: K and M re-formed element by element from their leaves in
//!   double-double (`modal_eigen::reformation`), from operands built here out
//!   of the `BuiltModel` (frames with the section re-derived from OD and
//!   wall, realized arcs, connectors and their tie rods, springs, the moduli
//!   actually used with the W-15 stiffness scales, and the component bodies),
//!   and the formation difference on the assembled matrices;
//! - check 2: per computed mode, the residual backward error η, the condition
//!   κ_λ and the first-order estimate e_λ on the re-formed pencil, for every
//!   computed pair (reported, band-recovered and above-band alike), kept in
//!   full; a reported mode with η > τ_r is demoted to Sensitive;
//! - check 3: the M̂-orthogonality of the published shapes; a breach of τ_o
//!   demotes the result to Sensitive;
//! - check 4 (the Sturm count) is in `eigen`; check 5 (clusters, δ_c) is the
//!   kernel's one definition, applied in `eigen`.
//!
//! A kernel refusal is mapped to the governed `MODAL_*` code
//! ([`modal_refusal_code`]); a published value that overflows or is NaN
//! refuses, and one that is nonzero but below the normal range is published
//! as 0 with its flag, never discarded.
use super::mass::{MemberKind, ModalMass};
use super::prepare::Prepared;
use super::weight_basis::IdentityTolerance;
use crate::{diag, stable_suffix, Diagnostic, PreviewPipe};
use open_pipe_stress_modal_eigen::admission::PencilRangeUnsupported;
use open_pipe_stress_modal_eigen::orthogonality::{m_orthogonality_reformed, MOrthogonality, OrthogonalityError};
use open_pipe_stress_modal_eigen::reformation::mass::ComponentBody;
use open_pipe_stress_modal_eigen::reformation::section::{FormedSection, Insulation, MassBasis, PipeSection};
use open_pipe_stress_modal_eigen::reformation::{
    formation_difference, residual_estimate, ArcOperand, ElementOperand, FormationDifference, FormationError, Moduli,
    ReformationError, ReformationFailure, ReformedPencil, ResidualError, ResidualEstimate, SpringOperand,
    StraightOperand,
};
use open_pipe_stress_modal_eigen::sturm::{SturmParameters, MAX_GAP_POSITION_LEVEL};
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;
use std::collections::{HashMap, HashSet};

/// A modal integrity parameter that is not finite or not in its domain.
/// (New in I2b-1; the design names no code for it.)
pub(crate) const MODAL_INTEGRITY_PARAMETER_INVALID: &str = "MODAL_INTEGRITY_PARAMETER_INVALID";
/// A nonzero formed or re-formed entry outside the admission range, or a
/// checked or published value that overflows or is NaN (§3.2 item 5).
pub(crate) use open_pipe_stress_modal_eigen::admission::PENCIL_RANGE_UNSUPPORTED as MODAL_PENCIL_RANGE_UNSUPPORTED;
/// An element family or operand the re-formation does not cover (§3.4
/// check 1): the formation check fails closed.
pub(crate) const MODAL_REFORMATION_UNAVAILABLE: &str = "MODAL_REFORMATION_UNAVAILABLE";
/// No free DOF, M_ff = 0, or no finite computed mode (§1.5).
pub(crate) const MODAL_NO_FINITE_MODES: &str = "MODAL_NO_FINITE_MODES";
/// The bound needs the whole-finite-spectrum endpoint and the finite-mode
/// count is not established (§3.2 item 5).
pub(crate) const MODAL_FINITE_COUNT_UNRESOLVED: &str = "MODAL_FINITE_COUNT_UNRESOLVED";
/// No final shift satisfies the separation premise (§3.4 check 4).
pub(crate) const MODAL_STURM_SHIFT_UNSEPARATED: &str = "MODAL_STURM_SHIFT_UNSEPARATED";
/// Every separated shift within the growth limit fails the conditioning
/// premise (§3.4 check 4).
pub(crate) const MODAL_STURM_SHIFT_ILL_CONDITIONED: &str = "MODAL_STURM_SHIFT_ILL_CONDITIONED";
/// Every separated shift exceeds g_max or is refused by the pivot guard
/// (§3.4 check 4).
pub(crate) const MODAL_STURM_GROWTH_LIMIT: &str = "MODAL_STURM_GROWTH_LIMIT";
/// The count at the final shift differs from the modes reported.
pub(crate) const MODAL_STURM_COUNT_MISMATCH: &str = "MODAL_STURM_COUNT_MISMATCH";
/// The dense eigensolve refused the pencil for a reason with no governed
/// name (a non-SPD K_ff after the static screen, the QL iteration limit).
/// (New in I2b-1; the design names no code for it.)
pub(crate) const MODAL_EIGENSOLVE_UNAVAILABLE: &str = "MODAL_EIGENSOLVE_UNAVAILABLE";

/// Every threshold and algorithm parameter of the dense modal solve and its
/// integrity checks (module docs). No field has a default.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ModalIntegrityParameters {
    /// τ_μ: a condensed μ ≤ τ_μ·μ_max is classified infinite (§3.2 item 5).
    /// In (0, 1).
    pub(crate) infinite_mu_relative_threshold: f64,
    /// τ_m: the equilibrated mass pivots of the finite-mode count. In (0, 1).
    pub(crate) mass_pivot_relative_threshold: f64,
    /// c_μ of the resolution floors δ_q and δ_μ. Positive.
    pub(crate) resolution_floor_constant: f64,
    /// κ_μ: the hard floor κ_μ·δ_q and the band edge κ_μ·δ_μ. Positive.
    pub(crate) resolution_floor_multiple: f64,
    /// c_f of τ_f = c_f·n·u, a recovered band pair's formed backward error
    /// screen. Positive.
    pub(crate) formed_backward_error_multiple: f64,
    /// τ_e: a recovered band pair's e_λ limit. In (0, 1).
    pub(crate) screen_error_estimate_threshold: f64,
    /// The implicit QL iteration limit per eigenvalue. At least 1.
    pub(crate) max_ql_iterations_per_eigenvalue: usize,
    /// τ_r: a reported mode with η > τ_r is Sensitive (check 2). In (0, 1).
    pub(crate) residual_threshold: f64,
    /// τ_o: the M-orthogonality tolerance (check 3). In (0, 1).
    pub(crate) orthogonality_threshold: f64,
    /// δ_c: the relative cluster tolerance (check 5). In (0, 1).
    pub(crate) cluster_tolerance: f64,
    /// τ_t: the "no translation" threshold of the normalization (§1.5). In
    /// (0, 1).
    pub(crate) no_translation_threshold: f64,
    /// κ_c: the Sturm conditioning multiple. At least 1.
    pub(crate) sturm_conditioning_multiple: f64,
    /// κ_s: the Sturm separation multiple. At least 1.
    pub(crate) sturm_separation_multiple: f64,
    /// g_max: the Sturm growth limit. At least 1.
    pub(crate) sturm_growth_limit: f64,
    /// s_max: the most candidate shifts per placement. At least 1.
    pub(crate) sturm_max_candidates: usize,
    /// The dyadic log-positions tried in a gap after its geometric mean,
    /// each in (0, 1), of level at most `MAX_GAP_POSITION_LEVEL`.
    pub(crate) sturm_gap_log_positions: Vec<f64>,
    /// The relative offsets of the whole-spectrum shift, each positive and
    /// finite; at least one.
    pub(crate) sturm_whole_spectrum_offsets: Vec<f64>,
    /// The weight-basis identity tolerance (§2.5, D-TOL), relative in (0, 1)
    /// and absolute finite and non-negative.
    pub(crate) identity_tolerance: IdentityTolerance,
}

fn parameter_refusal(name: &str, value: String, domain: &str) -> Diagnostic {
    diag(
        &format!("diagnostic:modal:integrity-parameter:{}", stable_suffix(name)),
        MODAL_INTEGRITY_PARAMETER_INVALID,
        "blocking",
        format!("modal integrity parameter `{name}` is {value}; it must be {domain}. No threshold is defaulted or pinned, so nothing is solved"),
        vec![name.to_string()],
    )
}

impl ModalIntegrityParameters {
    /// Every out-of-domain value, by name, in field order; empty when all
    /// are admitted.
    pub(crate) fn modal_parameter_refusals(&self) -> Vec<Diagnostic> {
        let mut found = Vec::new();
        let unit_open = |v: f64| v.is_finite() && v > 0.0 && v < 1.0;
        let positive = |v: f64| v.is_finite() && v > 0.0;
        let at_least_one = |v: f64| v.is_finite() && v >= 1.0;
        for (name, value, ok, domain) in [
            ("infinite_mu_relative_threshold", self.infinite_mu_relative_threshold, unit_open(self.infinite_mu_relative_threshold), "finite, in (0, 1)"),
            ("mass_pivot_relative_threshold", self.mass_pivot_relative_threshold, unit_open(self.mass_pivot_relative_threshold), "finite, in (0, 1)"),
            ("resolution_floor_constant", self.resolution_floor_constant, positive(self.resolution_floor_constant), "finite and positive"),
            ("resolution_floor_multiple", self.resolution_floor_multiple, positive(self.resolution_floor_multiple), "finite and positive"),
            ("formed_backward_error_multiple", self.formed_backward_error_multiple, positive(self.formed_backward_error_multiple), "finite and positive"),
            ("screen_error_estimate_threshold", self.screen_error_estimate_threshold, unit_open(self.screen_error_estimate_threshold), "finite, in (0, 1)"),
            ("residual_threshold", self.residual_threshold, unit_open(self.residual_threshold), "finite, in (0, 1)"),
            ("orthogonality_threshold", self.orthogonality_threshold, unit_open(self.orthogonality_threshold), "finite, in (0, 1)"),
            ("cluster_tolerance", self.cluster_tolerance, unit_open(self.cluster_tolerance), "finite, in (0, 1)"),
            ("no_translation_threshold", self.no_translation_threshold, unit_open(self.no_translation_threshold), "finite, in (0, 1)"),
            ("sturm_conditioning_multiple", self.sturm_conditioning_multiple, at_least_one(self.sturm_conditioning_multiple), "finite and at least 1"),
            ("sturm_separation_multiple", self.sturm_separation_multiple, at_least_one(self.sturm_separation_multiple), "finite and at least 1"),
            ("sturm_growth_limit", self.sturm_growth_limit, at_least_one(self.sturm_growth_limit), "finite and at least 1"),
            ("identity_tolerance.relative", self.identity_tolerance.relative, unit_open(self.identity_tolerance.relative), "finite, in (0, 1)"),
            (
                "identity_tolerance.absolute",
                self.identity_tolerance.absolute,
                self.identity_tolerance.absolute.is_finite() && self.identity_tolerance.absolute >= 0.0,
                "finite and non-negative",
            ),
        ] {
            if !ok {
                found.push(parameter_refusal(name, format!("{value:e}"), domain));
            }
        }
        if self.max_ql_iterations_per_eigenvalue == 0 {
            found.push(parameter_refusal("max_ql_iterations_per_eigenvalue", "0".into(), "at least 1"));
        }
        if self.sturm_max_candidates == 0 {
            found.push(parameter_refusal("sturm_max_candidates", "0".into(), "at least 1"));
        }
        let scale = (1u64 << MAX_GAP_POSITION_LEVEL) as f64;
        // The caller's lists have no length bound, so a refusal names the
        // first offending entry and its index rather than the whole list:
        // the diagnostic text stays bounded (retained pricing, §8.43).
        if let Some((index, t)) = self.sturm_gap_log_positions.iter().enumerate().find(|&(_, &t)| !(t > 0.0 && t < 1.0 && (t * scale).fract() == 0.0)) {
            found.push(parameter_refusal(
                "sturm_gap_log_positions",
                format!("{t:e} at index {index}"),
                &format!("a list of dyadic fractions in (0, 1) of level at most {MAX_GAP_POSITION_LEVEL}"),
            ));
        }
        let bad_offset = self.sturm_whole_spectrum_offsets.iter().enumerate().find(|&(_, &r)| !(r.is_finite() && r > 0.0));
        if self.sturm_whole_spectrum_offsets.is_empty() || bad_offset.is_some() {
            found.push(parameter_refusal(
                "sturm_whole_spectrum_offsets",
                match bad_offset {
                    Some((index, r)) => format!("{r:e} at index {index}"),
                    None => "empty".to_string(),
                },
                "a non-empty list of finite positive offsets",
            ));
        }
        found
    }

    /// The kernel's Sturm parameters, from the validated fields.
    pub(crate) fn modal_sturm_parameters(&self) -> SturmParameters {
        SturmParameters {
            conditioning_multiple: self.sturm_conditioning_multiple,
            separation_multiple: self.sturm_separation_multiple,
            growth_limit: self.sturm_growth_limit,
            max_candidates: self.sturm_max_candidates,
            gap_log_positions: self.sturm_gap_log_positions.clone(),
            whole_spectrum_offsets: self.sturm_whole_spectrum_offsets.clone(),
        }
    }
}

/// The governed code of a kernel range refusal, with its text.
pub(crate) fn modal_range_refusal(id: &str, what: String, refs: Vec<String>) -> Diagnostic {
    diag(&format!("diagnostic:modal:{id}"), MODAL_PENCIL_RANGE_UNSUPPORTED, "blocking", what, refs)
}

/// The code a kernel refusal maps to (module docs): range refusals to
/// `MODAL_PENCIL_RANGE_UNSUPPORTED`, the rest of the re-formation to
/// `MODAL_REFORMATION_UNAVAILABLE`.
pub(crate) fn modal_refusal_code(failure: &ReformationFailure) -> &'static str {
    match failure {
        ReformationFailure::PencilRangeUnsupported(_) => MODAL_PENCIL_RANGE_UNSUPPORTED,
        _ => MODAL_REFORMATION_UNAVAILABLE,
    }
}

/// One re-formation operand with the model entity it came from.
#[derive(Debug, Clone)]
pub(crate) struct NamedOperand {
    pub(crate) subject: String,
    pub(crate) operand: ElementOperand,
}

fn modal_pipe_section(pipe: &PreviewPipe) -> Option<PipeSection> {
    let s = &pipe.section;
    let insulation = match (&s.insulation_thickness, &s.insulation_density) {
        (Some(thickness), Some(density)) => Some(Insulation { thickness: thickness.value, density: density.value }),
        _ => None,
    };
    Some(PipeSection {
        outside_diameter: s.outside_diameter.value,
        wall_thickness: s.wall_thickness.value,
        mill_tolerance: s.mill_tolerance.as_ref().map_or(0.0, |q| q.value),
        corrosion_allowance: s.corrosion_allowance.as_ref().map(|q| q.value),
        material_density: s.material_density.as_ref()?.value,
        contents_density: s.contents_density.as_ref().map(|q| q.value),
        insulation,
    })
}

fn reformation_refusal(subject: &str, why: String) -> Diagnostic {
    diag(
        &format!("diagnostic:modal:reformation:{}", stable_suffix(subject)),
        MODAL_REFORMATION_UNAVAILABLE,
        "blocking",
        format!("element {subject} cannot be re-formed from its leaves ({why}); the formation check fails closed, so the modal set is refused"),
        vec![subject.to_string()],
    )
}

/// The re-formation operands of a prepared model (check 1, module docs), in
/// a fixed order: straight frames, realized arcs, component bodies,
/// connectors, their tie rods, then springs.
pub(crate) fn modal_reformation_operands(prepared: &Prepared, mass: &ModalMass) -> Result<Vec<NamedOperand>, Vec<Diagnostic>> {
    let built = &prepared.built;
    let model = &prepared.model;
    let pipes: HashMap<&str, &PreviewPipe> = model.pipe_segments.iter().map(|p| (p.id.as_str(), p)).collect();
    let bend_pipes: HashSet<&str> = built.curved_bend_elements.iter().map(|b| b.pipe_id.as_str()).collect();
    let replaced = crate::joint::replaced_span_ids(model);
    let bodies: HashSet<&str> = mass.bodies.iter().map(|b| b.pipe.as_str()).collect();
    let massive: HashSet<&str> = mass
        .members
        .iter()
        .map(|m| m.pipe.as_str())
        .chain(mass.bodies.iter().map(|b| b.pipe.as_str()))
        .collect();
    let mut operands = Vec::new();
    let mut refusals = Vec::new();
    let section_of = |id: &str, refusals: &mut Vec<Diagnostic>| {
        let section = pipes.get(id).and_then(|pipe| modal_pipe_section(pipe));
        if section.is_none() {
            refusals.push(reformation_refusal(id, "its section or material density is not available".into()));
        }
        section
    };
    for pipe in &built.pipes {
        let id = pipe.element_id.as_str();
        if bend_pipes.contains(id) || replaced.contains(&pipe.element_id) {
            continue;
        }
        let Some(section) = section_of(id, &mut refusals) else { continue };
        let (elastic_modulus, shear_modulus) = built.unscaled_moduli(pipe);
        let carries_mass = !bodies.contains(id);
        if carries_mass && !massive.contains(id) {
            refusals.push(reformation_refusal(id, "the formed mass has no entry for this member".into()));
            continue;
        }
        operands.push(NamedOperand {
            subject: id.to_string(),
            operand: ElementOperand::Straight(StraightOperand {
                node_i: pipe.node_i.index,
                node_j: pipe.node_j.index,
                coordinates_i: pipe.node_i.coordinates,
                coordinates_j: pipe.node_j.coordinates,
                y_reference: pipe.y_reference,
                section,
                moduli: Moduli {
                    elastic_modulus,
                    shear_modulus,
                    stiffness_scale: built.scaled_members.get(id).map(|m| m.scale),
                },
                carries_mass,
                formed_section: Some(FormedSection {
                    area: pipe.section.area,
                    second_moment: pipe.section.second_moment_y,
                    torsion_constant: pipe.section.torsion_constant,
                }),
            }),
        });
    }
    for bend in &built.curved_bend_elements {
        let id = bend.pipe_id.as_str();
        let Some(section) = section_of(id, &mut refusals) else { continue };
        let element = &bend.macro_element;
        let scaled = built.scaled_members.get(id);
        let (elastic_modulus, shear_modulus) =
            scaled.map_or((element.elastic_modulus, element.shear_modulus), |m| (m.elastic_modulus, m.shear_modulus));
        let carries_mass = !bodies.contains(id);
        let arc_member = mass.members.iter().any(|m| m.pipe == id && matches!(m.kind, MemberKind::Arc { .. }));
        if carries_mass && !arc_member {
            refusals.push(reformation_refusal(id, "the formed mass has no half-arc lumps for this bend".into()));
            continue;
        }
        operands.push(NamedOperand {
            subject: id.to_string(),
            operand: ElementOperand::Arc(ArcOperand {
                node_i: bend.node_i,
                node_j: bend.node_j,
                coordinates_i: built.nodes[bend.node_i].coordinates,
                coordinates_j: built.nodes[bend.node_j].coordinates,
                radius: element.radius,
                y_reference: element.y_reference,
                section,
                moduli: Moduli { elastic_modulus, shear_modulus, stiffness_scale: scaled.map(|m| m.scale) },
                in_plane_flexibility_factor: element.in_plane_flexibility_factor,
                out_of_plane_flexibility_factor: element.out_of_plane_flexibility_factor,
                carries_mass,
                formed_section: Some(FormedSection {
                    area: element.area,
                    second_moment: element.second_moment,
                    torsion_constant: element.torsion_constant,
                }),
            }),
        });
    }
    for body in &mass.bodies {
        let Some(acceleration) = mass.acceleration else {
            refusals.push(reformation_refusal(&body.component_id, "a component body without the request's acceleration".into()));
            continue;
        };
        operands.push(NamedOperand {
            subject: body.component_id.clone(),
            operand: ElementOperand::ComponentBody(ComponentBody {
                node: body.node,
                weight: body.weight,
                acceleration,
                center_of_gravity: body.center_of_gravity,
            }),
        });
    }
    for (connector, record) in built.connectors.iter().zip(&built.connector_records) {
        operands.push(NamedOperand { subject: record.component_id.clone(), operand: ElementOperand::Connector(connector.clone()) });
    }
    for (connector, record) in built.connectors.iter().zip(&built.connector_records) {
        for (index, rod) in connector.tie_rods().iter().enumerate() {
            let subject = record.tie_rods.get(index).map_or_else(|| format!("{}:tie-rod:{index}", record.component_id), |r| r.id.clone());
            operands.push(NamedOperand { subject, operand: ElementOperand::TieRod(*rod) });
        }
    }
    for spring in &prepared.boundary.springs {
        // A zero spring adds +0.0 to the formed diagonal and nothing here.
        if spring.stiffness.value == 0.0 {
            continue;
        }
        operands.push(NamedOperand {
            subject: spring.support_id.clone(),
            operand: ElementOperand::Spring(SpringOperand::Axis { dof: spring.node_dof.global_index(), stiffness: spring.stiffness.value }),
        });
    }
    // R-6/R-7 D-1 (a): each skewed spring re-formed as k·n̂n̂ᵀ on its node's
    // translations from the authored direction (the block K-D5 intends).
    for spring in built.skew.iter().flat_map(|basis| basis.springs()).filter(|spring| spring.stiffness != 0.0) {
        operands.push(NamedOperand {
            subject: spring.support_id.clone(),
            operand: ElementOperand::Spring(SpringOperand::RankOne {
                node: spring.node,
                rotational: false,
                direction: spring.direction.authored(),
                stiffness: spring.stiffness,
            }),
        });
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }
    Ok(operands)
}

/// Check 1: the re-formed pencil and the formation difference against the
/// formed K_ff and M_ff entries.
pub(crate) fn reform_modal_pencil(
    operands: &[NamedOperand],
    exact: bool,
    dimension: usize,
    dof_map: &[Option<usize>],
    coordinates: Option<&[Option<open_pipe_stress_modal_eigen::reformation::Coordinates>]>,
    stiffness_entries: &[SymmetricMatrixEntry],
    mass_entries: &[SymmetricMatrixEntry],
) -> Result<(ReformedPencil, FormationDifference), Vec<Diagnostic>> {
    let basis = if exact { MassBasis::NominalWallPressureBore } else { MassBasis::EffectiveWall };
    let elements: Vec<ElementOperand> = operands.iter().map(|o| o.operand.clone()).collect();
    // R-6/R-7 D-1 (a): with a restraint basis, the re-formed reduced pencil Q̃ᵀK̂Q̃, Q̃ᵀM̂Q̃.
    let reformed = match coordinates {
        Some(coordinates) => ReformedPencil::new_with_coordinates(dimension, dof_map, coordinates, basis, &elements),
        None => ReformedPencil::new(dimension, dof_map, basis, &elements),
    };
    let pencil = reformed.map_err(|ReformationError { element, failure }| {
        let subject = operands.get(element).map_or("model", |o| o.subject.as_str());
        let code = modal_refusal_code(&failure);
        vec![diag(
            &format!("diagnostic:modal:reformation:{}", stable_suffix(subject)),
            code,
            "blocking",
            format!("element {subject} cannot be re-formed from its leaves ({failure:?}); the formation check fails closed, so the modal set is refused"),
            vec![subject.to_string()],
        )]
    })?;
    let difference = formation_difference(&pencil, stiffness_entries, mass_entries).map_err(|error| {
        let code = match error {
            FormationError::PencilRangeUnsupported(_) | FormationError::ResultOutOfRange { .. } => MODAL_PENCIL_RANGE_UNSUPPORTED,
            _ => MODAL_REFORMATION_UNAVAILABLE,
        };
        vec![diag(
            "diagnostic:modal:formation-difference",
            code,
            "blocking",
            format!("the formation difference cannot be measured on the formed pencil ({error:?}), so the modal set is refused"),
            vec!["model".into()],
        )]
    })?;
    Ok((pencil, difference))
}

/// Check 2 for one computed mode, its refusal mapped by name.
pub(crate) fn modal_residual(
    pencil: &ReformedPencil,
    difference: &FormationDifference,
    mode: usize,
    eigenvalue: f64,
    vector: &[f64],
) -> Result<ResidualEstimate, Diagnostic> {
    residual_estimate(pencil, &difference.scaling, eigenvalue, vector).map_err(|error: ResidualError| {
        modal_range_refusal(
            &format!("residual:mode-{}", mode + 1),
            format!("the residual check of computed mode {} cannot be evaluated ({error:?}); a value that is not finite is never discarded, so the modal set is refused", mode + 1),
            vec![format!("mode:{}", mode + 1)],
        )
    })
}

/// Check 3 on the published shapes, its refusal mapped by name.
pub(crate) fn modal_orthogonality(pencil: &ReformedPencil, shapes: &[&[f64]]) -> Result<MOrthogonality, Diagnostic> {
    m_orthogonality_reformed(pencil, shapes).map_err(|error: OrthogonalityError| {
        let refs = match &error {
            OrthogonalityError::RatioOutOfRange { pair, .. } => vec![format!("mode:{}", pair.0 + 1), format!("mode:{}", pair.1 + 1)],
            OrthogonalityError::NonPositiveGeneralizedMass { mode, .. } | OrthogonalityError::NonFinite { mode, .. } => {
                vec![format!("mode:{}", mode + 1)]
            }
            _ => vec!["model".into()],
        };
        modal_range_refusal(
            "orthogonality",
            format!("the M-orthogonality check of the published shapes cannot be evaluated ({error:?}), so the modal set is refused"),
            refs,
        )
    })
}

/// A kernel range refusal named by its matrix and free position.
pub(crate) fn pencil_range_text(range: &PencilRangeUnsupported, labels: &[String]) -> String {
    let at = |i: usize| labels.get(i).cloned().unwrap_or_else(|| format!("free DOF {i}"));
    format!("{range} (at {} / {})", at(range.row), at(range.col))
}
