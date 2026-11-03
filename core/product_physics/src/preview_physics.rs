//! preview-physics-2: the ordinary route's repaired and withheld quantities.
//!
//! The case loop produces the current rows plus a side record that marks where
//! the support and maximum rows go. Once every case has solved and no case was
//! source-selected, `render` places them, sets the per-case headline inputs and
//! edits the diagnostics.
//! Everything downstream (headlines, the high-displacement review, the
//! modifier count, combinations and the finite check) is then built once from
//! the rendered rows. Contract: T0R S1_INTERFACE.md.

use super::*;
use open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum;

pub(crate) const ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
pub(crate) const INTENSIFIED_KIND: &str = "component_equal_factor_intensified_bending_stress_v1";
const ARC_BASIS: &str = "nominal_straight_beam_formula_on_arc_resultants";
const MAXIMUM_SIGN: &str = "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim";
const SUBTRACTION_SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; a signed difference of solved states, not an equilibrium state";
const WITHHELD_ATTRIBUTION: &str = "SUPPORT_ACTION_ATTRIBUTION_WITHHELD";
const WITHHELD_CONSTANT_EFFORT: &str = "CONSTANT_EFFORT_NOT_CONSUMED";
const SUPPORT_COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];

/// The linear state kinds admitted to combination algebra (table
/// `combination_policy.linear_state_kinds`).
const LINEAR_STATE_KINDS: [&str; 22] = [
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
    "element_local_axial_force",
    "element_local_shear_force_y",
    "element_local_shear_force_z",
    "element_local_torsional_moment",
    "element_local_bending_moment_y",
    "element_local_bending_moment_z",
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
    "support_reaction_component_v2",
    "constant_effort_support_applied_load",
    // W-7 (DEL-04-01 Design `global-end-actions.md`): the global end actions.
    "element_global_end_force_v1",
    "element_global_end_moment_v1",
];
const ARC_STRESS_KINDS: [&str; 6] = [
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
];

pub(crate) const LIMITATIONS: [&str; 7] = [
    "Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.",
    "Nonzero pressure is refused on this route.",
    "On every member the section normal-stress extremes are N/A ± hypot(My,Mz)/Z at each station and the transverse shear maximum is the straight-annulus Saint-Venant value, with ν = E/(2G) − 1 assuming an isotropic material. On straight members the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No equivalent stress or code stress.",
    "On arcs: signed tangent-frame resultants, nominal stress components and section extremes; endpoint force rows are node-on-element actions in the tangent frame at their end. The normal-stress maximum is bounded along the arc and enters the stress headline. The transverse shear uses the straight-annulus field without curvature correction.",
    "Intensified measures are i*hypot(My,Mz)/Z with the user's scalar SIF at member ends adjacent to markers and branches, using member Z. They are not code stresses and are never combined. A flexibility factor never multiplies stress. On realized arcs with directional SIFs, intensified bending is hypot(i_ip*Mz, i_op*My)/Z in the tangent frame, with its along-arc maximum.",
    "Signed support actions for rigid restraints, springs, consuming constant effort and attributable nonlinear supports. Ambiguous attribution is withheld.",
    "Headlines cover all load cases. In a model with nonlinear supports a mechanics combination is withheld unless one term is a solved load case with factor exactly 1, at least one other term exists, and every other term is a combination whose fixed-load count κ is defined and exactly 0; it is then formed from rows, with its operands' standing, as a solved state plus derived differences or bounds, not an equilibrium state. With a consuming constant effort a mechanics combination is withheld when its κ is undefined or differs from 1 by more than 64ε, and an absolute sum or SRSS unless every operand's κ is exactly 0. Mechanics combinations, absolute sums and SRSS are also withheld for mixed moduli over the load cases they reach, and every combination is withheld with a withheld operand combination. Absolute sums and SRSS are unsigned. Subtraction is a labelled difference. No combination maxima and no code compliance.",
];

pub(crate) fn formulation_basis() -> FormulationBasis {
    FormulationBasis {
        profile_id: "product_preview_mechanics_v1".to_string(),
        limitations: LIMITATIONS.iter().map(|s| s.to_string()).chain([
            "Member actions, section actions, stresses and support reactions of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision from the formation check's corrected displacement and rounded once; they are not binary64 recomputations from the solve's own displacement, from which they may differ by about 1e-12 of the body's action scale (at most about 6e-9 in the evaluation sets).".to_string(),
            "The recovery is checked against the formation check's re-formed system (stiffness and loads formed at 128-bit precision from the binary64 inputs). Wherever the formation check produced its member bound for a load case, including a case that the member recovery check then reports as sensitive, each published nodal displacement component is the corrected displacement (the solve's displacement plus the check's two correction terms, summed exactly) rounded once to binary64 in SI units and then converted to the published unit as every other row is, and each displacement magnitude is the correctly rounded norm of the node's three published translation components; where there is no such bound, or where any component of the case cannot be formed, every published displacement of the case is the solve's own, and a selected retained source publishes its recovery's nodal values. Support reactions balance the re-formed loads, not the solve's binary64 load vector.".to_string(),
        ]).collect(),
    }
}

/// A blocked envelope has no rows. Diagnostics pushed before the block keep
/// their meaning, but `result:` refs that can no longer resolve are stripped,
/// so the envelope still satisfies S1 §9 (R1 SF-1).
pub(crate) fn sanitize_blocked_diagnostics(diagnostics: &mut Vec<Diagnostic>) {
    // Non-blocking diagnostics that name a (now absent) row are dropped. A
    // blocking diagnostic always stays, so the blocking reason is never lost;
    // only its unresolvable `result:` refs are stripped.
    diagnostics.retain(|d| {
        d.severity == "blocking" || !d.affected_refs.iter().any(|r| r.starts_with("result:"))
    });
    for d in diagnostics.iter_mut() {
        d.affected_refs.retain(|r| !r.starts_with("result:"));
    }
}

pub(crate) fn empty_evidence() -> serde_json::Value {
    serde_json::json!({"preview_cases": [], "combination_gates": []})
}

/// Per-member data the case loop records for rendering.
#[derive(Debug, Clone)]
pub(crate) struct MemberRecord {
    pub pipe_id: String,
    pub arc: bool,
    /// The certified enclosure or its failure: a straight member's from its
    /// statics intervals; a realized arc's along the arc (T4-U4b §2.3,
    /// [`from_arc_maximum`]), set after its station rows. `None` only before.
    pub maximum: Option<Result<CertifiedStressMaximum, String>>,
    /// j-side section-cut resultants at end_i and end_j.
    pub end_resultants: [[f64; 6]; 2],
    pub section_modulus: f64,
    pub certified_end_bending: Option<[open_pipe_stress_frame_kernel::structural::CertifiedRow;2]>,
}

/// Where `render` places a support's signed actions or a member's maximum:
/// the position among the case's rows at which the case loop reached them.
#[derive(Debug, Clone)]
pub(crate) enum Slot {
    Support(String),
    Maximum(String),
}

/// Side record of one solved ordinary case.
#[derive(Debug, Clone, Default)]
pub(crate) struct CaseRecord {
    /// Per model support with a node: the device-law six-vector from the case loop.
    pub support_vectors: Vec<(String, [f64; 6])>,
    pub certified_support_norms: Vec<(String,[f64;2])>,
    pub members: Vec<MemberRecord>,
    /// (position among the case's rows, slot), in the case loop's order.
    pub slots: Vec<(usize, Slot)>,
    /// T4-U4b (§2.6): the directional intensified station and maximum records
    /// of the case's realized arcs, appended to `intensified_measures`.
    pub directional_measures: Vec<serde_json::Value>,
}

/// T4-U4b (§2.3): a certified arc maximum in the straight record's carrier:
/// one span (index 0), ψ*/φ as both fractions, and the global upper bound as
/// the published upper value.
pub(crate) fn from_arc_maximum(m: &open_pipe_stress_frame_kernel::structural::CertifiedArcStressMaximum) -> CertifiedStressMaximum {
    CertifiedStressMaximum {
        span_index: 0,
        local_fraction: m.fraction,
        station: m.fraction,
        value_lower: m.value_lower,
        value_upper: m.upper_bound,
        upper_bound: m.upper_bound,
        certified_gap: m.certified_gap,
        subdivisions: m.subdivisions,
    }
}

/// Model facts that gate `mechanics` combinations.
#[derive(Debug, Clone)]
pub(crate) struct Rendered {
    pub preview_cases: Vec<serde_json::Value>,
    nonlinear: bool,
    consuming_constant_effort: bool,
}

impl Rendered {
    /// The contract evidence; `derivation` is [`combination_derivation`]'s member,
    /// absent for a document with flat combinations only (V-8).
    pub(crate) fn evidence(&self, gates: Vec<serde_json::Value>, derivation: Option<serde_json::Value>) -> serde_json::Value {
        let mut evidence = serde_json::json!({"preview_cases": self.preview_cases, "combination_gates": gates});
        if let (Some(derivation), Some(object)) = (derivation, evidence.as_object_mut()) {
            object.insert(DERIVATION_MEMBER.to_string(), derivation);
        }
        evidence
    }
}

pub(crate) fn identity(parts: &[&str]) -> String {
    exact_source_identity(parts)
}

fn final_id(case_index: usize, case_id: &str, row: &ResultItem) -> String {
    if case_index == 0 || row.kind.ends_with("_v2") {
        row.id.clone()
    } else {
        qualified_load_case_result_id(case_id, &row.id)
    }
}

/// Residual-determined devices on one node-DOF are ambiguous when there are
/// two or more of them (rigid + nonlinear, nonlinear + nonlinear).
fn ambiguous_supports(
    linear: &[LinearSupport],
    nonlinear: &[NonlinearSupport],
    restrained: &HashSet<usize>,
) -> HashSet<String> {
    let mut devices: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for support in linear {
        for dof in &support.restrained_dofs {
            let global = support.node_index * DOF_PER_NODE + dof_index(*dof);
            if restrained.contains(&global) {
                devices.entry(global).or_default().push(&support.support_id);
            }
        }
    }
    for support in nonlinear {
        let global = support.node_index * DOF_PER_NODE + dof_index(support.dof);
        devices.entry(global).or_default().push(&support.support_id);
    }
    devices
        .values()
        .filter(|ids| ids.len() > 1)
        .flatten()
        .map(|id| id.to_string())
        .collect()
}

enum SupportDisposition {
    Attributed([f64; 6]),
    Withheld(&'static str),
}

fn support_dispositions(
    model: &PreviewModel,
    ambiguous: &HashSet<String>,
    record: &CaseRecord,
) -> Vec<(String, SupportDisposition)> {
    let constant_effort: HashMap<&str, Result<ConstantEffortApplication, ConstantEffortNonConsumption>> =
        constant_effort_solve_dispositions(model)
            .into_iter()
            .map(|(support, disposition)| (support.id.as_str(), disposition))
            .collect();
    model
        .supports
        .iter()
        .map(|support| {
            let disposition = if ambiguous.contains(&support.id) {
                SupportDisposition::Withheld(WITHHELD_ATTRIBUTION)
            } else if let Some(disposition) = constant_effort.get(support.id.as_str()) {
                match disposition {
                    Ok(application) => {
                        let mut vector = [0.0; 6];
                        vector[dof_index(application.dof)] = application.force_newtons;
                        SupportDisposition::Attributed(vector)
                    }
                    Err(_) => SupportDisposition::Withheld(WITHHELD_CONSTANT_EFFORT),
                }
            } else {
                match record.support_vectors.iter().find(|(id, _)| id == &support.id) {
                    Some((_, vector)) => SupportDisposition::Attributed(*vector),
                    // A support without a model node was already refused.
                    None => SupportDisposition::Withheld(WITHHELD_ATTRIBUTION),
                }
            };
            (support.id.clone(), disposition)
        })
        .collect()
}

fn maximum_row(case_id: &str, pipe_id: &str, value: f64, arc: bool) -> ResultItem {
    ResultItem { component_body: None,
        id: format!(
            "result:elastic-maximum:{}:{}:{}:{}",
            case_id.len(),
            case_id,
            pipe_id.len(),
            pipe_id
        ),
        kind: "pipe_elastic_normal_stress_maximum_v2".to_string(),
        value,
        unit: "Pa".to_string(),
        entity_ref: pipe_id.to_string(),
        basis_ref: Some(ResultBasisRef {
            ref_type: "load_case".to_string(),
            ref_id: case_id.to_string(),
        }),
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: "maximum_absolute_normal_stress".to_string(),
            coordinate_system: "pipe_section".to_string(),
            location: "governing_station".to_string(),
            basis: "recovered_from_open_mechanics_stress_components".to_string(),
            sign_convention: if arc { u4b_rows::ARC_MAXIMUM_SIGN } else { MAXIMUM_SIGN }.to_string(),
        }),
    }
}

fn extrema_evidence(pipe_id: &str, result_id: &str, m: &CertifiedStressMaximum, arc: bool) -> serde_json::Value {
    if arc {
        // T4-U4b (§2.3): the arc record's closed fourteen keys and arc tokens.
        return serde_json::json!({"pipe_id":pipe_id,"result_id":result_id,
            "station_fraction":m.station,"span_index":0,"local_fraction":m.local_fraction,
            "value_lower_pa":m.value_lower,"value_upper_pa":m.upper_bound,"global_upper_bound_pa":m.upper_bound,
            "certified_gap_pa":m.certified_gap,"subdivisions":m.subdivisions,
            "approximation":u4b_rows::ARC_APPROXIMATION,"coefficient_basis":u4b_rows::ARC_COEFFICIENT_BASIS,
            "enclosure_scope":u4b_rows::ARC_ENCLOSURE_SCOPE});
    }
    serde_json::json!({"pipe_id":pipe_id,"result_id":result_id,
        "station_fraction":m.station,"span_index":m.span_index,"local_fraction":m.local_fraction,
        "value_lower_pa":m.value_lower,"value_upper_pa":m.value_upper,"global_upper_bound_pa":m.upper_bound,
        "certified_gap_pa":m.certified_gap,"subdivisions":m.subdivisions,
        "approximation":"piecewise_quadratic_straight_section_statics","coefficient_basis":"j_side_section_equilibrium_binary64_enclosures",
        "enclosure_scope":"supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"})
}

/// One intensified-measure association of a component with a pipe end
/// (RT-A3: also the exact route's branch rows, `u4b_rows::append_branch_intensified`).
pub(crate) struct Association<'a> {
    pub(crate) component: &'a PreviewComponent,
    pub(crate) role: &'static str,
    pub(crate) side_label: &'static str,
    pub(crate) sif: f64,
    pub(crate) source_reference: &'a str,
}

pub(crate) fn user_sif(value: Option<&Quantity>) -> Option<f64> {
    value.map(|q| q.value).filter(|v| positive_finite(*v))
}

pub(crate) fn solver_consumption(component: &PreviewComponent) -> &str {
    component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref())
        .unwrap_or("mechanics_geometry_only")
}

fn source_reference(component: &PreviewComponent) -> &str {
    component
        .modifiers
        .as_ref()
        .and_then(|m| m.source_reference.as_deref())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("source_reference_missing")
}

/// The associations of a straight pipe end at `node`, per S1 §4.
pub(crate) fn associations<'a>(model: &'a PreviewModel, node: &str, pipe_id: &str) -> Vec<Association<'a>> {
    let mut found = Vec::new();
    for component in model.components.iter().filter(|c| c.node == node) {
        if is_curved_bend_macro_component(component)
            || solver_consumption(component) != "mechanics_geometry_only"
        {
            continue;
        }
        let modifiers = component.modifiers.as_ref();
        if is_bend_component(component) {
            if let Some(sif) = user_sif(modifiers.and_then(|m| m.sif_user_value.as_ref())) {
                found.push(Association { component, role: "bend", side_label: "bend", sif, source_reference: source_reference(component) });
            }
        } else if is_branch_component(component) {
            let geometry = component.geometry.as_ref();
            let (role, side_label, value) = if geometry.and_then(|g| g.branch_header_pipe_ref.as_deref()) == Some(pipe_id) {
                ("branch_header", "branch header", modifiers.and_then(|m| m.branch_header_sif_user_value.as_ref()))
            } else if geometry.and_then(|g| g.branch_branch_pipe_ref.as_deref()) == Some(pipe_id) {
                ("branch_branch", "branch branch", modifiers.and_then(|m| m.branch_branch_sif_user_value.as_ref()))
            } else {
                continue;
            };
            if let Some(sif) = user_sif(value) {
                found.push(Association { component, role, side_label, sif, source_reference: source_reference(component) });
            }
        }
    }
    found
}

/// Model-level intensification diagnostics (once per model, not per case).
fn intensification_model_diagnostics(model: &PreviewModel, arc_pipes: &HashSet<&str>, diagnostics: &mut Vec<Diagnostic>) {
    for component in &model.components {
        let modifiers = component.modifiers.as_ref();
        if is_curved_bend_macro_component(component) {
            if user_sif(modifiers.and_then(|m| m.sif_user_value.as_ref())).is_some() {
                diagnostics.push(diag(
                    &format!("diagnostic:preview-physics:intensification-not-applied:{}", identity(&[&component.id])),
                    "COMPONENT_STRESS_INTENSIFICATION_NOT_APPLIED",
                    "info",
                    format!("bend component {} is realized as a curved-bend macro-element; no intensified measure is published on the arc or its straight neighbours on this route (arc end values are not the arc's governing station)", component.id),
                    vec![component.id.clone()],
                ));
            }
            continue;
        }
        if solver_consumption(component) != "mechanics_geometry_only" {
            continue;
        }
        if is_bend_component(component) {
            if user_sif(modifiers.and_then(|m| m.sif_user_value.as_ref())).is_none() {
                diagnostics.push(diag(
                    &format!("diagnostic:preview-physics:intensification-sif-absent:{}", identity(&[&component.id, "bend"])),
                    "COMPONENT_INTENSIFICATION_SIF_NOT_SUPPLIED",
                    "info",
                    format!("bend marker {} has no positive user-entered SIF; no intensified measure is published and no default factor is supplied", component.id),
                    vec![component.id.clone()],
                ));
            }
        } else if is_branch_component(component) {
            let geometry = component.geometry.as_ref();
            for (role, pipe_ref, value) in [
                ("branch_header", geometry.and_then(|g| g.branch_header_pipe_ref.as_deref()), modifiers.and_then(|m| m.branch_header_sif_user_value.as_ref())),
                ("branch_branch", geometry.and_then(|g| g.branch_branch_pipe_ref.as_deref()), modifiers.and_then(|m| m.branch_branch_sif_user_value.as_ref())),
            ] {
                if pipe_ref.is_some() && user_sif(value).is_none() {
                    diagnostics.push(diag(
                        &format!("diagnostic:preview-physics:intensification-sif-absent:{}", identity(&[&component.id, role])),
                        "COMPONENT_INTENSIFICATION_SIF_NOT_SUPPLIED",
                        "info",
                        format!("branch {} has no positive user-entered {role} SIF; no intensified measure is published for that side and no default factor is supplied", component.id),
                        vec![component.id.clone()],
                    ));
                }
            }
            let referenced: HashSet<&str> = [
                geometry.and_then(|g| g.branch_header_pipe_ref.as_deref()),
                geometry.and_then(|g| g.branch_branch_pipe_ref.as_deref()),
            ]
            .into_iter()
            .flatten()
            .collect();
            for pipe in model.pipe_segments.iter().filter(|p| {
                (p.from == component.node || p.to == component.node)
                    && !referenced.contains(p.id.as_str())
                    && !arc_pipes.contains(p.id.as_str())
            }) {
                diagnostics.push(diag(
                    &format!("diagnostic:preview-physics:intensified-coverage:{}", identity(&[&component.id, &pipe.id])),
                    "COMPONENT_INTENSIFIED_COVERAGE_INCOMPLETE",
                    "warning",
                    format!("pipe {} ends at branch {} but is neither its header nor its branch reference; no intensified measure is published for that end", pipe.id, component.id),
                    vec![component.id.clone(), pipe.id.clone()],
                ));
            }
        }
    }
}

/// Model-level check of arc end tangents against adjacent straight pipes.
fn tangent_diagnostics(model: &PreviewModel, built: &BuiltModel, diagnostics: &mut Vec<Diagnostic>) {
    let arc_pipes: HashSet<&str> = built.curved_bend_elements.iter().map(|b| b.pipe_id.as_str()).collect();
    for bend in &built.curved_bend_elements {
        let Ok(tangents) = bend.macro_element.end_tangents() else { continue };
        for (end, node_index, tangent) in [("end_i", bend.node_i, tangents[0]), ("end_j", bend.node_j, tangents[1])] {
            let node = &model.nodes[node_index];
            let here = built.nodes[node_index].coordinates;
            // The closest adjacent straight pipe decides, so a tee branch at an
            // arc end does not raise a false warning (R1 N-5).
            let mut closest: Option<(f64, &PreviewPipe)> = None;
            for pipe in model.pipe_segments.iter().filter(|p| {
                p.id != bend.pipe_id && !arc_pipes.contains(p.id.as_str()) && (p.from == node.id || p.to == node.id)
            }) {
                let far_id = if pipe.from == node.id { &pipe.to } else { &pipe.from };
                let Some(far) = node_index_of(model, far_id).map(|i| built.nodes[i].coordinates) else { continue };
                // Travel direction through the arc: into end_i, out of end_j.
                let d: [f64; 3] = if end == "end_i" {
                    std::array::from_fn(|k| here[k] - far[k])
                } else {
                    std::array::from_fn(|k| far[k] - here[k])
                };
                let dot = d[0] * tangent[0] + d[1] * tangent[1] + d[2] * tangent[2];
                let cross = [
                    d[1] * tangent[2] - d[2] * tangent[1],
                    d[2] * tangent[0] - d[0] * tangent[2],
                    d[0] * tangent[1] - d[1] * tangent[0],
                ];
                let angle = norm3(cross[0], cross[1], cross[2]).atan2(dot);
                if angle.is_finite() && closest.is_none_or(|(best, _)| angle < best) {
                    closest = Some((angle, pipe));
                }
            }
            if let Some((angle, pipe)) = closest.filter(|(angle, _)| *angle > DEC_070_CURVED_BEND_ANGLE_MATCH_TOLERANCE) {
                diagnostics.push(diag(
                    &format!("diagnostic:preview-physics:tangent-discontinuity:{}", identity(&[&bend.component_id, &bend.pipe_id, end, &pipe.id])),
                    "CURVED_BEND_TANGENT_DISCONTINUITY",
                    "warning",
                    format!("curved-bend {} {end} tangent differs from the closest adjacent pipe {} by {} rad ({} degrees), above the 1e-6 rad geometric-consistency warning threshold; the arc is built as authored and this warning does not repair its geometry", bend.component_id, pipe.id, scalar_string(angle), scalar_string(angle.to_degrees())),
                    vec![bend.component_id.clone(), bend.pipe_id.clone(), node.id.clone(), pipe.id.clone()],
                ));
            }
        }
    }
}

fn node_index_of(model: &PreviewModel, id: &str) -> Option<usize> {
    model.nodes.iter().position(|node| node.id == id)
}

/// Maximum rows, extrema evidence and coverage of one case (S1 §5.1-§5.2).
struct MemberRender {
    maximum_rows: HashMap<String, ResultItem>,
    extrema: Vec<serde_json::Value>,
    unavailable: Vec<String>,
    outside: Vec<String>,
    /// The case maximum with identity ties, over the available members only.
    case_max: Option<LocatedQuantity>,
}

/// W-15 slice d: `bodies` are the members formed as component bodies; they
/// keep their maxima and extrema but never enter the case maximum.
fn render_members(case_id: &str, members: &[MemberRecord], bodies: &HashSet<&str>, diagnostics: &mut Vec<Diagnostic>) -> MemberRender {
    let mut out = MemberRender { maximum_rows: HashMap::new(), extrema: Vec::new(), unavailable: Vec::new(), outside: Vec::new(), case_max: None };
    for member in members {
        match &member.maximum {
            None => out.outside.push(member.pipe_id.clone()),
            Some(Err(error)) => {
                out.unavailable.push(member.pipe_id.clone());
                diagnostics.push(diag(
                    &format!("diagnostic:preview-physics:maximum-unavailable:{}", identity(&[case_id, &member.pipe_id])),
                    "PREVIEW_STRESS_MAXIMUM_UNAVAILABLE",
                    "warning",
                    format!("signed rows remain available; the circular normal-stress maximum of {} is unavailable in {}: {error}", member.pipe_id, case_id),
                    vec![case_id.to_string(), member.pipe_id.clone()],
                ));
            }
            Some(Ok(maximum)) => {
                let public_upper=maximum.upper_bound;
                let value=maximum.value_lower+0.5*(public_upper-maximum.value_lower);
                let row = maximum_row(case_id, &member.pipe_id, value, member.arc);
                let mut evidence=extrema_evidence(&member.pipe_id,&row.id,maximum,member.arc);
                evidence["value_upper_pa"]=serde_json::json!(maximum.upper_bound);
                out.extrema.push(evidence);
                if !bodies.contains(member.pipe_id.as_str()) && out.case_max.as_ref().is_none_or(|q| value > q.value || (value == q.value && member.pipe_id < q.location_ref)) {
                    out.case_max = Some(LocatedQuantity { value, unit: "Pa".to_string(), location_ref: member.pipe_id.clone(), result_ref: row.id.clone() });
                }
                out.maximum_rows.insert(member.pipe_id.clone(), row);
            }
        }
    }
    out
}

/// Test-only view of each certified maximum's local witness upper beside its
/// global upper before publication keeps only the latter (DEL-04-01
/// kd5-member-relative-bound.md §3.2), so a test can assert its own
/// precondition that they differ. Only `kd5-max:` projects are kept; the
/// preview may run on the reserved-stack thread, hence a process-wide list.
#[cfg(test)]
pub(crate) mod witness_hook {
    use super::MemberRecord;
    type Seen = (String, String, f64, f64);
    static SEEN: std::sync::Mutex<Vec<Seen>> = std::sync::Mutex::new(Vec::new());
    pub(crate) fn observe(project: &str, members: &[MemberRecord]) {
        if !project.starts_with("kd5-max:") {
            return;
        }
        let mut seen = SEEN.lock().unwrap();
        // The straight piecewise witness only (T4-U4b: an arc's maximum has no witness interval).
        for member in members.iter().filter(|m| !m.arc) {
            if let Some(Ok(maximum)) = &member.maximum {
                seen.push((project.to_string(), member.pipe_id.clone(), maximum.value_upper, maximum.upper_bound));
            }
        }
    }
    /// (pipe, witness upper, global upper) observed for `project`, removed.
    pub(crate) fn take(project: &str) -> Vec<(String, f64, f64)> {
        let mut seen = SEEN.lock().unwrap();
        let (mine, rest): (Vec<Seen>, Vec<Seen>) = seen.drain(..).partition(|entry| entry.0 == project);
        *seen = rest;
        mine.into_iter().map(|entry| (entry.1, entry.2, entry.3)).collect()
    }
}

/// Place the support and maximum rows of every case, set the per-case headline
/// inputs and edit the diagnostics (S1 §3-§8). Pure over the solved cases.
pub(crate) fn render(
    model: &PreviewModel,
    built: &BuiltModel,
    restrained_dofs: &[usize],
    solves: &mut [LoadCaseSolve],
    diagnostics: &mut Vec<Diagnostic>,
) -> Rendered {
    let restrained: HashSet<usize> = restrained_dofs.iter().copied().collect();
    let ambiguous = ambiguous_supports(&built.supports, &built.nonlinear_supports, &restrained);
    let arc_pipes: HashSet<&str> = built.curved_bend_elements.iter().map(|b| b.pipe_id.as_str()).collect();
    let bodies: HashSet<&str> = built.scaled_members.keys().map(String::as_str).collect();
    let mut removed_ids = HashSet::new();
    let mut preview_cases = Vec::new();
    let mut incomplete_pipes = BTreeMap::new();
    let mut withheld_once = HashMap::new();

    for (case_index, solve) in solves.iter_mut().enumerate() {
        let case_id = solve.load_case_id.clone();
        let load_case = model
            .load_cases
            .iter()
            .find(|case| case.id == case_id)
            .expect("solved case belongs to the model");
        let mut record = solve.preview.take().unwrap_or_default();
        let slots = std::mem::take(&mut record.slots);

        // Supports.
        let dispositions = support_dispositions(model, &ambiguous, &record);
        let mut support_rows: HashMap<String, Vec<ResultItem>> = HashMap::new();
        let mut attributed = Vec::new();
        let mut withheld = Vec::new();
        for (support_id, disposition) in &dispositions {
            match disposition {
                SupportDisposition::Attributed(vector) => {
                    let support = model.supports.iter().find(|s| &s.id == support_id).unwrap();
                    let mut rows = Vec::new();
                    append_signed_support_results(&mut rows, load_case, support, *vector);
                    if let Some((_,norms)) = record.certified_support_norms.iter().find(|(id,_)| id == support_id) {
                        rows[6].value = norms[0]; rows[7].value = norms[1];
                    }
                    support_rows.insert(support_id.clone(), rows);
                    attributed.push(serde_json::json!(support_id));
                }
                SupportDisposition::Withheld(reason) => {
                    withheld.push(serde_json::json!({"support_id": support_id, "reason": reason}));
                    withheld_once.insert(support_id.clone(), *reason);
                }
            }
        }
        let withheld_nonlinear: HashSet<&str> = dispositions
            .iter()
            .filter(|(_, d)| matches!(d, SupportDisposition::Withheld(WITHHELD_ATTRIBUTION)))
            .map(|(id, _)| id.as_str())
            .collect();

        // Members.
        #[cfg(test)]
        witness_hook::observe(&model.project.id, &record.members);
        let MemberRender { mut maximum_rows, extrema, unavailable, outside, case_max } =
            render_members(&case_id, &record.members, &bodies, diagnostics);
        let complete = unavailable.is_empty() && outside.is_empty();
        for pipe in unavailable.iter().chain(&outside) {
            incomplete_pipes.insert(pipe.clone(), ());
        }

        // Place the support and maximum rows at their slots; withheld
        // nonlinear reactions are removed.
        let old = std::mem::take(&mut solve.results);
        let mut rows = Vec::with_capacity(old.len());
        let mut slots = slots.into_iter().peekable();
        let mut place = |slot: Slot, rows: &mut Vec<ResultItem>| match slot {
            Slot::Support(id) => rows.extend(support_rows.remove(&id).unwrap_or_default()),
            Slot::Maximum(pipe) => rows.extend(maximum_rows.remove(&pipe)),
        };
        for (index, mut row) in old.into_iter().enumerate() {
            while let Some((_, slot)) = slots.next_if(|(at, _)| *at == index) {
                place(slot, &mut rows);
            }
            if row.kind == "nonlinear_support_final_reaction" && withheld_nonlinear.contains(row.entity_ref.as_str()) {
                removed_ids.insert(final_id(case_index, &case_id, &row));
                continue;
            }
            if arc_pipes.contains(row.entity_ref.as_str()) {
                if let Some(metadata) = row.metadata.as_mut() {
                    if ARC_STRESS_KINDS.contains(&row.kind.as_str()) {
                        metadata.basis = ARC_BASIS.to_string();
                    }
                }
            }
            rows.push(row);
        }
        for (_, slot) in slots {
            place(slot, &mut rows);
        }
        drop(place);
        for (_, remaining) in support_rows.into_iter().collect::<BTreeMap<_, _>>() {
            rows.extend(remaining);
        }
        for (_, remaining) in maximum_rows.into_iter().collect::<BTreeMap<_, _>>() {
            rows.push(remaining);
        }

        // Intensified measures (S2b).
        let mut intensified = Vec::new();
        let mut intensified_evidence = Vec::new();
        for member in record.members.iter().filter(|m| !m.arc) {
            let Some(pipe) = model.pipe_segments.iter().find(|p| p.id == member.pipe_id) else { continue };
            for (end_index, (location, node)) in [("end_i", &pipe.from), ("end_j", &pipe.to)].into_iter().enumerate() {
                for association in associations(model, node, &member.pipe_id) {
                    let resultants = member.end_resultants[end_index];
                    let (my, mz) = (resultants[4], resultants[5]);
                    let value = if let Some(bending) = &member.certified_end_bending {
                        match bending[end_index].divided(member.section_modulus).and_then(|row| row.scaled(association.sif)) {
                            Ok(row) => row.published,
                            Err(error) => { diagnostics.push(diag(&format!("diagnostic:certified-intensified:{}:{location}",member.pipe_id),"ELEMENT_FORCE_RECOVERY_FAILED","blocking",format!("certified intensified stress unavailable: {error:?}"),vec![member.pipe_id.clone(),case_id.clone()])); continue; }
                        }
                    } else { association.sif * (norm2(my, mz) / member.section_modulus) };
                    let endpoint = endpoint_id_location(location);
                    let base_id = format!(
                        "result:intensified-bending:{}:{}:{endpoint}",
                        stable_suffix(&association.component.id),
                        stable_suffix(&member.pipe_id)
                    );
                    let row = ResultItem { component_body: None,
                        id: base_id,
                        kind: INTENSIFIED_KIND.to_string(),
                        value,
                        unit: "Pa".to_string(),
                        entity_ref: association.component.id.clone(),
                        basis_ref: Some(ResultBasisRef { ref_type: "load_case".to_string(), ref_id: case_id.clone() }),
                        source_result_refs: endpoint_bending_refs(&member.pipe_id, location),
                        metadata: Some(ResultMetadata {
                            component: "equal_factor_intensified_bending_stress".to_string(),
                            coordinate_system: "pipe_section".to_string(),
                            location: location.to_string(),
                            basis: "user_sif_times_member_section_bending_stress_v1".to_string(),
                            sign_convention: format!(
                                "nonnegative i*hypot(My,Mz)/Z at the member end; i={} user-entered {} SIF (source: {}); member Z; not a code effective section modulus; not a code stress; no flexibility factor, axial or torsion term; never combined",
                                scalar_string(association.sif),
                                association.side_label,
                                association.source_reference
                            ),
                        }),
                    };
                    let row_id = final_id(case_index, &case_id, &row);
                    intensified_evidence.push(serde_json::json!({
                        "result_id": row_id, "component_id": association.component.id, "pipe_id": member.pipe_id,
                        "location": location, "factor_role": association.role, "sif": association.sif,
                        "sif_source_reference": association.source_reference, "section_modulus_m3": member.section_modulus,
                        "bending_moment_y_n_m": my, "bending_moment_z_n_m": mz,
                    }));
                    diagnostics.push(diag(
                        &format!("diagnostic:preview-physics:intensification:{}", identity(&[&case_id, &association.component.id, &member.pipe_id, location])),
                        "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED",
                        "info",
                        format!("{} {} applies the user-entered {} SIF {} to the member-Z bending stress at {} {location}: i*hypot(My,Mz)/Z; no flexibility factor, axial or torsion term; not a code stress", association.role, association.component.id, association.side_label, scalar_string(association.sif), member.pipe_id),
                        vec![association.component.id.clone(), member.pipe_id.clone(), row_id, association.source_reference.to_string(), case_id.clone()],
                    ));
                    intensified.push(row);
                }
            }
        }
        solve.component_stress_modifier_count = intensified.len();
        rows.extend(intensified);
        intensified_evidence.extend(std::mem::take(&mut record.directional_measures));

        // Per-case headline inputs, with identity ties.
        solve.max_stress = if complete { case_max } else { None };
        solve.max_displacement = rows
            .iter()
            .filter(|row| row.kind == "displacement_magnitude")
            .fold(None, |best: Option<LocatedQuantity>, row| {
                if best.as_ref().is_none_or(|q| row.value > q.value || (row.value == q.value && row.entity_ref < q.location_ref)) {
                    Some(LocatedQuantity { value: row.value, unit: row.unit.clone(), location_ref: row.entity_ref.clone(), result_ref: row.id.clone() })
                } else {
                    best
                }
            });
        solve.results = rows;

        preview_cases.push(serde_json::json!({
            "load_case_id": case_id,
            "pipe_stress_extrema": extrema,
            "stress_maximum_coverage": {"complete": complete, "unavailable_pipe_ids": unavailable, "outside_domain_pipe_ids": outside},
            "support_attribution": {"attributed_support_ids": attributed, "withheld": withheld},
            "intensified_measures": intensified_evidence,
        }));
    }

    // Diagnostics: anything naming a removed row goes.
    diagnostics.retain(|d| !d.affected_refs.iter().any(|r| removed_ids.contains(r)));
    for support in &model.supports {
        match withheld_once.get(&support.id).copied() {
            Some(WITHHELD_ATTRIBUTION) => diagnostics.push(diag(
                &format!("diagnostic:preview-physics:attribution:{}", identity(&[&support.id])),
                WITHHELD_ATTRIBUTION,
                "warning",
                format!("support {} shares a node DOF with another residual-determined device (rigid or nonlinear); the nodal residual cannot be divided between them, so its signed actions are withheld rather than duplicated or zero-filled", support.id),
                vec![support.id.clone(), support.node.clone()],
            )),
            Some(WITHHELD_CONSTANT_EFFORT) => diagnostics.push(diag(
                &format!("diagnostic:preview-physics:constant-effort-not-consumed:{}", identity(&[&support.id])),
                WITHHELD_CONSTANT_EFFORT,
                "info",
                format!("constant-effort support {} is not consumed by the solve, so no signed support action is published for it", support.id),
                vec![support.id.clone()],
            )),
            _ => {}
        }
    }
    if !incomplete_pipes.is_empty() {
        diagnostics.push(diag(
            "diagnostic:preview-physics:stress-headline-withheld",
            "PREVIEW_STRESS_HEADLINE_WITHHELD",
            "warning",
            "stress headline withheld: a stress maximum is not available for every member in every load case",
            incomplete_pipes.into_keys().collect(),
        ));
    }
    if !model.combinations.is_empty() {
        diagnostics.push(diag(
            "diagnostic:preview-physics:headline-scope",
            "PREVIEW_HEADLINE_SCOPE_LOAD_CASES",
            "info",
            "headlines cover the load cases only; combinations are excluded",
            model.combinations.iter().map(|c| c.id.clone()).collect(),
        ));
    }
    tangent_diagnostics(model, built, diagnostics);
    intensification_model_diagnostics(model, &arc_pipes, diagnostics);

    Rendered {
        preview_cases,
        nonlinear: model.supports.iter().any(|s| s.nonlinear.is_some()) || !built.nonlinear_supports.is_empty(),
        consuming_constant_effort: constant_effort_solve_dispositions(model).iter().any(|(_, d)| d.is_ok()),
    }
}

fn endpoint_bending_refs(pipe_id: &str, location: &str) -> Vec<String> {
    let suffix = stable_suffix(pipe_id);
    let id_location = endpoint_id_location(location);
    vec![
        format!("result:stress:{suffix}:{id_location}:bending-normal-y"),
        format!("result:stress:{suffix}:{id_location}:bending-normal-z"),
    ]
}

/// §5.1's codes on this route, whose fixed-load source is a consuming
/// constant-effort support.
pub(crate) const CONSTANT_EFFORT_REQUIRES_SOLVE: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
pub(crate) const CONSTANT_EFFORT_UNSIGNED_OPERAND: &str = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND";
pub(crate) const CONSTANT_EFFORT_OPERAND_MIXED: &str = "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED";
/// §4: an operand combination withheld whole by a gate withholds its dependants.
pub(crate) const OPERAND_WITHHELD: &str = "LOAD_COMBINATION_OPERAND_WITHHELD";
/// §5.1: a combination whose own κ needs more than 4,096 significand bits.
pub(crate) const FIXED_LOAD_COUNT_UNREPRESENTABLE: &str = "COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE";

/// §5.1: the info naming a combination whose own κ is unrepresentable (both routes).
pub(crate) fn fixed_load_count_message(id: &str) -> String {
    format!("combination {id}: its fixed-load count (the number of times it carries each fixed load) needs more than 4,096 significand bits exactly, so it is undefined; a gated use never treats an undefined count as passing")
}

/// §4: the withheld operand combinations of one combination, each with its own
/// code, in operand order (each once).
pub(crate) fn withheld_operands<'t>(targets: &[combination_plan::Target<'t>], withheld: &HashMap<String, &'static str>) -> Vec<(&'t str, &'static str)> {
    let mut out: Vec<(&str, &'static str)> = Vec::new();
    for target in targets {
        if let combination_plan::Target::Combination(id) = target {
            if let Some(code) = withheld.get(*id) {
                if !out.iter().any(|(named, _)| named == id) {
                    out.push((id, code));
                }
            }
        }
    }
    out
}

/// The message of [`OPERAND_WITHHELD`] (both routes).
pub(crate) fn operand_withheld_message(operands: &[(&str, &str)]) -> String {
    let named = operands.iter().map(|(id, code)| format!("{id} ({code})")).collect::<Vec<_>>().join(", ");
    format!("Combination withheld: its operand combination(s) {named} are withheld, so it is withheld whole.")
}

/// The gates (absolute-sum-srss §5), in precedence order: a withheld operand
/// combination; nonlinear supports (`mechanics` only, except decision OQ-1); the
/// fixed-load count of a consuming constant-effort support (`mechanics`,
/// `absolute_sum`, `srss`); and mixed modulus bases over every load case the
/// combination reaches (`mechanics`, `absolute_sum`, `srss`). Subtractions and
/// envelopes are ungated otherwise. A flat combination meets today's gates exactly.
fn gate_reason(
    rendered: &Rendered,
    model: &PreviewModel,
    plan: &combination_plan::Plan<'_>,
    index: usize,
    targets: &[combination_plan::Target<'_>],
    withheld: &HashMap<String, &'static str>,
) -> Option<(&'static str, String)> {
    let combination = &model.combinations[index];
    let operands = withheld_operands(targets, withheld);
    if !operands.is_empty() {
        return Some((OPERAND_WITHHELD, operand_withheld_message(&operands)));
    }
    let gated = |code: &'static str| Some((code, gate_message(code).to_string()));
    let basis = combination.basis.as_str();
    if !matches!(basis, "mechanics" | "absolute_sum" | "srss") {
        return None;
    }
    if basis == "mechanics" && rendered.nonlinear && !plan.solved_state_plus_increments(combination) {
        return gated("NONLINEAR_COMBINATION_REQUIRES_SOLVE");
    }
    // The fixed-load count κ (§5.1); flat, it is the unit factor-sum rule (option C):
    // the exact sum of the binary64 factors, admitted iff |sum - 1| <= 64*epsilon,
    // compared exactly. A non-finite factor (already refused by validation's
    // LOAD_COMBINATION_FACTOR_INVALID) makes κ undefined, which never passes.
    if rendered.consuming_constant_effort {
        let operand_kappa: Vec<_> = targets.iter().map(|target| plan.kappa_of(target)).collect();
        let withholding = if basis == "mechanics" {
            open_pipe_stress_load_case_algebra::mechanics_fixed_load_gate(&plan.kappa[index], operand_kappa.iter())
        } else {
            open_pipe_stress_load_case_algebra::unsigned_fixed_load_gate(operand_kappa.iter())
        };
        if let Some(withholding) = withholding {
            use open_pipe_stress_load_case_algebra::FixedLoadWithholding as W;
            return gated(match withholding {
                W::UnitFactorSum => CONSTANT_EFFORT_REQUIRES_SOLVE,
                W::OperandUndefined => CONSTANT_EFFORT_OPERAND_MIXED,
                W::UnsignedOperand => CONSTANT_EFFORT_UNSIGNED_OPERAND,
            });
        }
    }
    let mut scratch = Vec::new();
    let keys: HashSet<Option<String>> = plan.reached[index]
        .iter()
        .filter_map(|case| model.load_cases.iter().find(|c| c.id == *case))
        .map(|case| modulus_basis_key(case, &mut scratch))
        .collect();
    // Selection conflicts were refused before solving; the scratch list is discarded.
    if keys.len() > 1 {
        return gated("COMBINATION_MODULUS_BASIS_MIXED");
    }
    None
}

fn gate_message(code: &str) -> &'static str {
    match code {
        "NONLINEAR_COMBINATION_REQUIRES_SOLVE" => "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
        CONSTANT_EFFORT_REQUIRES_SOLVE => "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
        CONSTANT_EFFORT_UNSIGNED_OPERAND => "Combination withheld: a constant-effort support's force acts in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.",
        CONSTANT_EFFORT_OPERAND_MIXED => "Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts a constant-effort support's force cannot be decided.",
        _ => "Combination withheld: its load cases were solved on different modulus bases.",
    }
}

fn combination_row(
    combination: &PreviewCombination,
    expression: &CombinationExpression<'_>,
    sign_convention: &str,
    reference: &ResultItem,
    id: String,
    value: f64,
    source_refs: Vec<String>,
) -> ResultItem {
    let mut combined = reference.clone();
    combined.id = id;
    combined.value = value;
    combined.basis_ref = Some(ResultBasisRef { ref_type: "combination".to_string(), ref_id: combination.id.clone() });
    combined.source_result_refs = source_refs;
    if let Some(metadata) = combined.metadata.as_mut() {
        metadata.basis = expression.result_metadata_basis().to_string();
        metadata.sign_convention = sign_convention.to_string();
    }
    combined
}

/// Kinds a combination combines (as the reference row's kind); every other
/// kind is never combined and publishes no combination row.
/// (`displacement_magnitude` combines on both paths: as the norm of combined
/// signed components, or range-selected directly.)
fn combined_kind(kind: &str) -> bool {
    !kind.ends_with("_v2") && (LINEAR_STATE_KINDS.contains(&kind) || kind == "displacement_magnitude")
}

/// §4 (r1 F4): with a combination operand, rows pair by kind, unit, entity and
/// every metadata field except `basis` and `sign_convention`, which describe a
/// row's derivation; a section-basis suffix within `basis` is still compared.
/// A `displacement_magnitude` row is identified by kind, unit and node alone: a load
/// case's carries no metadata and a derived one's states only its derivation.
fn nested_source_identity_matches(reference: &ResultItem, candidate: &ResultItem) -> bool {
    const SECTION: &str = "; section_basis=";
    let section = |basis: &str| basis.split_once(SECTION).map(|(_, s)| s.to_string());
    reference.kind == candidate.kind
        && reference.unit == candidate.unit
        && reference.entity_ref == candidate.entity_ref
        && (reference.kind == "displacement_magnitude" || match (&reference.metadata, &candidate.metadata) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a.component == b.component
                    && a.coordinate_system == b.coordinate_system
                    && a.location == b.location
                    && section(&a.basis) == section(&b.basis)
            }
            _ => false,
        })
}

/// Combination rows for preview-physics-2 (S1 §7; absolute-sum-srss §2-§5).
/// Returns the evidence gates, in evaluation order.
pub(crate) fn append_combination_results(
    model: &PreviewModel,
    rendered: &Rendered,
    rows_by_base_id: &HashMap<String, HashMap<String, ResultItem>>,
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<serde_json::Value> {
    use combination_plan::Target;
    let mut base_ids = rows_by_base_id.keys().cloned().collect::<Vec<_>>();
    base_ids.sort();
    let case_rows: Vec<ResultItem> = results
        .iter()
        .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case"))
        .cloned()
        .collect();
    // §2.7: the never_combined kinds publish no combination row.
    let has_maximum = case_rows.iter().any(|r| matches!(r.kind.as_str(), "pipe_elastic_normal_stress_maximum_v2" | u4b_rows::EXTREME_KIND | u4b_rows::SHEAR_MAX_KIND));
    let has_intensified = case_rows.iter().any(|r| matches!(r.kind.as_str(), INTENSIFIED_KIND | u4b_rows::DIRECTIONAL_KIND | u4b_rows::DIRECTIONAL_MAX_KIND));
    let plan = combination_plan::Plan::new(model);
    let mut gates = Vec::new();
    // §4: rows a combination publishes, kept only while a later one names it: by base
    // id (ordinary rows and magnitudes), and whole (for its support groups).
    let mut published_by_base: HashMap<String, HashMap<String, ResultItem>> = HashMap::new();
    let mut published_rows: HashMap<String, Vec<ResultItem>> = HashMap::new();
    let mut withheld_combinations: HashMap<String, &'static str> = HashMap::new();
    for &index in &plan.order {
        let combination = &model.combinations[index];
        let Some(expression) = CombinationExpression::resolve(combination) else {
            gates.push(serde_json::json!({"combination_id": combination.id, "withheld": false, "reason": null}));
            continue;
        };
        let targets = expression.targets(&plan.resolver);
        if plan.kappa[index].is_unrepresentable() {
            diagnostics.push(diag(
                &format!("diagnostic:preview-physics:combination-fixed-load-count:{}", identity(&[&combination.id])),
                FIXED_LOAD_COUNT_UNREPRESENTABLE,
                "info",
                fixed_load_count_message(&combination.id),
                vec![combination.id.clone()],
            ));
        }
        if let Some((code, message)) = gate_reason(rendered, model, &plan, index, &targets, &withheld_combinations) {
            let mut refs = vec![combination.id.clone()];
            if code == OPERAND_WITHHELD {
                refs.extend(withheld_operands(&targets, &withheld_combinations).iter().map(|(id, _)| id.to_string()));
            }
            diagnostics.push(diag(
                &format!("diagnostic:preview-physics:combination-gate:{}", identity(&[&combination.id])),
                code,
                "warning",
                message,
                refs,
            ));
            gates.push(serde_json::json!({"combination_id": combination.id, "withheld": true, "reason": code}));
            withheld_combinations.insert(combination.id.clone(), code);
            continue;
        }
        gates.push(serde_json::json!({"combination_id": combination.id, "withheld": false, "reason": null}));
        let range = matches!(expression, CombinationExpression::RangeEnvelope { .. });
        let nested = targets.iter().any(|t| matches!(t, Target::Combination(_)));
        // §4: a magnitude row of an unsigned or nested combination states its
        // derivation in its metadata; a flat combination's keeps none (V-8).
        let derived = nested || expression.is_unsigned();
        let sign = match expression {
            CombinationExpression::ResultStateSubtraction { .. } => SUBTRACTION_SIGN.to_string(),
            _ => expression.result_sign_convention(rendered.nonlinear && plan.solved_state_plus_increments(combination)),
        };
        let lookup = |base_id: &str, target: Target<'_>| -> Option<&ResultItem> {
            match target {
                Target::Case(case) => rows_by_base_id.get(base_id)?.get(case),
                Target::Combination(id) => published_by_base.get(base_id)?.get(id),
            }
        };
        let matches = |reference: &ResultItem, source: &ResultItem| {
            if nested {
                nested_source_identity_matches(reference, source)
            } else {
                combination_source_identity_matches(reference, source)
            }
        };
        // This combination's rows and, only when a later combination names it, the base
        // id of each ordinary row (a flat document keeps none).
        let keep = plan.named[index];
        let mut out: Vec<ResultItem> = Vec::new();
        let mut out_bases: Vec<Option<String>> = Vec::new();

        // Every combination row this combination does not publish is named:
        // a missing operand row by LOAD_COMBINATION_SOURCE_RESULT_MISSING, any
        // other withheld row in one bounded LOAD_COMBINATION_ROW_WITHHELD.
        let mut withheld: Vec<String> = Vec::new();

        // Ordinary rows grouped by base id.
        for base_id in &base_ids {
            let source_missing = |diagnostics: &mut Vec<Diagnostic>| {
                diagnostics.push(diag(
                    &format!("diagnostic:preview-physics:combination-source-missing:{}", identity(&[&combination.id, base_id])),
                    "LOAD_COMBINATION_SOURCE_RESULT_MISSING",
                    "warning",
                    format!("combination {} omits {base_id} because a matching source row is missing in an operand case", combination.id),
                    vec![combination.id.clone()],
                ));
            };
            let Some(reference) = lookup(base_id, targets[0]) else {
                // The reference (first) operand lacks a row another operand has:
                // reported exactly as a missing row in any other operand.
                if targets.iter().filter_map(|target| lookup(base_id, *target)).next().is_some_and(|row| combined_kind(&row.kind)) {
                    source_missing(diagnostics);
                }
                continue;
            };
            let kind = reference.kind.as_str();
            if !combined_kind(kind) {
                continue;
            }
            if kind == "displacement_magnitude" && !range {
                let before = out.len();
                append_combined_vector_magnitude(combination, &expression, &sign, derived, &targets, base_id, reference, &lookup, &mut out, &mut withheld, diagnostics);
                if out.len() > before && keep {
                    out_bases.push(Some(base_id.clone()));
                }
                continue;
            }
            let Some(dimension) = algebra_dimension(reference) else {
                withheld.push(format!("{base_id} (no algebra dimension)"));
                continue;
            };
            let mut values = Vec::new();
            let mut refs = Vec::new();
            let mut complete = true;
            for target in &targets {
                match lookup(base_id, *target).filter(|s| matches(reference, s)) {
                    Some(source) => {
                        values.push(source.value);
                        refs.push(source.id.clone());
                    }
                    None => complete = false,
                }
            }
            if !complete {
                source_missing(diagnostics);
                continue;
            }
            match expression.evaluate_values(&values, dimension) {
                Some(value) => {
                    out.push(combination_row(combination, &expression, &sign, reference, qualified_combination_result_id(&combination.id, base_id), value, refs));
                    if keep {
                        out_bases.push(Some(base_id.clone()));
                    }
                }
                None => withheld.push(format!("{base_id} (load-case algebra refused)")),
            }
        }

        // Signed support actions, grouped by (support, component). A support's
        // group publishes whole or not at all: no partial component set.
        for support in &model.supports {
            let find = |target: Target<'_>, component: &str| -> Option<&ResultItem> {
                let hit = |r: &&ResultItem| {
                    r.entity_ref == support.id
                        && r.metadata.as_ref().is_some_and(|m| m.component == component)
                        && r.kind.starts_with("support_reaction_")
                };
                match target {
                    Target::Case(case) => case_rows.iter().find(|r| hit(r) && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case)),
                    Target::Combination(id) => published_rows.get(id)?.iter().find(hit),
                }
            };
            let base = |component: &str| format!("support-action:{}:{}:{component}", support.id.len(), support.id);
            if targets.iter().all(|target| find(*target, "Fx").is_none()) {
                continue;
            }
            let all_sources = |component: &str| -> Option<Vec<&ResultItem>> {
                targets.iter().map(|target| find(*target, component)).collect()
            };
            let group = || -> Result<Vec<ResultItem>, &'static str> {
                let mut rows = Vec::new();
                let mut combined = [0.0; 6];
                for (slot, component) in SUPPORT_COMPONENTS.iter().enumerate() {
                    let sources = all_sources(component).ok_or("source support row missing in an operand case")?;
                    let dimension = if slot < 3 { LoadDimension::Force } else { LoadDimension::Moment };
                    let values: Vec<f64> = sources.iter().map(|s| s.value).collect();
                    let value = expression.evaluate_values(&values, dimension).ok_or("load-case algebra refused")?;
                    combined[slot] = value;
                    rows.push(combination_row(combination, &expression, &sign, sources[0], qualified_combination_result_id(&combination.id, &base(component)), value, sources.iter().map(|s| s.id.clone()).collect()));
                }
                for (component, value) in [
                    ("force_magnitude", norm3(combined[0], combined[1], combined[2])),
                    ("moment_magnitude", norm3(combined[3], combined[4], combined[5])),
                ] {
                    // §3, §6: an unsigned magnitude outside the binary64 range is
                    // refused, never published as an infinity; the group is withheld.
                    if !range && expression.is_unsigned() && !value.is_finite() {
                        return Err("load-case algebra refused");
                    }
                    // A linear magnitude is the norm of the combined components;
                    // its row identity is the first operand row that has it.
                    let sources = if range {
                        all_sources(component)
                    } else {
                        Some(targets.iter().filter_map(|target| find(*target, component)).collect::<Vec<_>>()).filter(|rows| !rows.is_empty())
                    }
                    .ok_or("source support row missing in an operand case")?;
                    let value = if range {
                        let dimension = if component == "force_magnitude" { LoadDimension::Force } else { LoadDimension::Moment };
                        let values: Vec<f64> = sources.iter().map(|s| s.value).collect();
                        expression.evaluate_values(&values, dimension).ok_or("load-case algebra refused")?
                    } else {
                        value
                    };
                    let refs = if range {
                        sources.iter().map(|s| s.id.clone()).collect()
                    } else {
                        SUPPORT_COMPONENTS
                            .iter()
                            .map(|c| qualified_combination_result_id(&combination.id, &base(c)))
                            .collect()
                    };
                    rows.push(combination_row(combination, &expression, &sign, sources[0], qualified_combination_result_id(&combination.id, &base(component)), value, refs));
                }
                Ok(rows)
            };
            match group() {
                Ok(rows) => {
                    if keep {
                        out_bases.extend(rows.iter().map(|_| None));
                    }
                    out.extend(rows);
                }
                Err(reason) => withheld.push(format!("support-action:{}:{} ({reason})", support.id.len(), support.id)),
            }
        }
        if derived {
            // A range-selected magnitude of a nested envelope whose source row is a
            // load case's carries no metadata of its own.
            for row in out.iter_mut().filter(|r| r.kind == "displacement_magnitude" && r.metadata.is_none()) {
                row.metadata = Some(crate::derived_magnitude_metadata(expression.result_metadata_basis(), &sign));
            }
        }
        if !withheld.is_empty() {
            const NAMED: usize = 8;
            let mut named = withheld.iter().take(NAMED).cloned().collect::<Vec<_>>().join(", ");
            if withheld.len() > NAMED {
                named.push_str(&format!(", and {} more", withheld.len() - NAMED));
            }
            diagnostics.push(diag(
                &format!("diagnostic:preview-physics:combination-row-withheld:{}", identity(&[&combination.id])),
                "LOAD_COMBINATION_ROW_WITHHELD",
                "warning",
                format!("combination {} withholds {} combined row(s) that could not be formed: {named}", combination.id, withheld.len()),
                vec![combination.id.clone()],
            ));
        }
        if has_maximum {
            diagnostics.push(diag(
                &format!("diagnostic:preview-physics:combination-maximum:{}", identity(&[&combination.id])),
                "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE",
                "info",
                "stress maxima and section stress extremes are not combined; combine the signed stress component rows instead",
                vec![combination.id.clone()],
            ));
        }
        if has_intensified {
            diagnostics.push(diag(
                &format!("diagnostic:preview-physics:combination-intensified:{}", identity(&[&combination.id])),
                "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE",
                "info",
                "intensified measures are never combined (their ranges and combinations follow the owner's rule in T6)",
                vec![combination.id.clone()],
            ));
        }
        if plan.named[index] {
            for (row, base) in out.iter().zip(&out_bases) {
                if let Some(base) = base {
                    published_by_base.entry(base.clone()).or_default().insert(combination.id.clone(), row.clone());
                }
            }
            published_rows.insert(combination.id.clone(), out.clone());
        }
        results.extend(out);
    }
    gates
}

/// RT-A4 (absolute-sum-srss §10.3): the evidence member from which a reader without
/// the model re-derives every absolute sum's, SRSS's and nested combination's rows
/// and the gates that turn on κ and decision OQ-1.
pub(crate) const DERIVATION_MEMBER: &str = "combination_derivation";

/// §10.3: the model conditions the gates read (`nonlinear_supports`,
/// `consuming_constant_effort`) and, in gate (evaluation) order, one record
/// `{combination_id, basis, expression}` for every absolute sum, SRSS and
/// combination with a combination operand, withheld or not, and for every
/// combination those reach through their operands, so each record's κ is
/// re-derivable from earlier records. The expression is `combination_operands`'
/// shape (a term names its operand by kind; a subtraction's or envelope's id names
/// the load case when one has it). `None` when no combination is unsigned or nested:
/// a flat document publishes no member, so its bytes are unchanged (V-8).
pub(crate) fn combination_derivation(model: &PreviewModel, rendered: &Rendered, gates: &[serde_json::Value]) -> Option<serde_json::Value> {
    let resolver = combination_plan::Resolver::new(model);
    let ordered: Vec<&PreviewCombination> =
        gates.iter().filter_map(|g| model.combinations.iter().find(|c| g["combination_id"] == c.id.as_str())).collect();
    let operands = |combination: &PreviewCombination| -> Vec<String> {
        CombinationExpression::resolve(combination)
            .map(|e| {
                e.targets(&resolver)
                    .iter()
                    .filter_map(|t| match t {
                        combination_plan::Target::Combination(id) => Some(id.to_string()),
                        combination_plan::Target::Case(_) => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let mut recorded: HashSet<String> = ordered
        .iter()
        .filter(|c| matches!(c.basis.as_str(), "absolute_sum" | "srss") || !operands(c).is_empty())
        .map(|c| c.id.clone())
        .collect();
    if recorded.is_empty() {
        return None;
    }
    // An operand precedes its dependants in the order, so one reverse pass closes the set.
    for combination in ordered.iter().rev() {
        if recorded.contains(&combination.id) {
            recorded.extend(operands(combination));
        }
    }
    let records: Vec<serde_json::Value> = ordered
        .iter()
        .filter(|c| recorded.contains(&c.id))
        .map(|c| serde_json::json!({"combination_id": c.id, "basis": c.basis, "expression": exact_combinations::expression_evidence(c)}))
        .collect();
    Some(serde_json::json!({
        "nonlinear_supports": rendered.nonlinear,
        "consuming_constant_effort": rendered.consuming_constant_effort,
        "combinations": records,
    }))
}

/// Operating-case slice 1a (DEL-05-02 Design `pf-nonlinear-operand-standing.md`):
/// the evidence member stating each operand's standing.
pub(crate) const OPERANDS_MEMBER: &str = "combination_operands";
/// §2.2: some nonlinear support's published final state differs between the operands.
pub(crate) const OPERAND_STATES_DIFFER: &str = "COMBINATION_OPERAND_SUPPORT_STATES_DIFFER";
/// §2.2: some operand's numerical integrity checks did not pass.
pub(crate) const OPERAND_CHECKS_NOT_PASSED: &str = "COMBINATION_OPERAND_CHECKS_NOT_PASSED";
const STATE_CODE_KIND: &str = "nonlinear_support_active_set_state_code";
const DISPLACEMENT_KIND: &str = "nonlinear_support_final_displacement";
const RELATIVE_DISPLACEMENT_KIND: &str = "nonlinear_support_final_relative_displacement";
const STATES_NAMED: usize = 8;

/// §2.2: an operand's reported state of one support: the published state code and,
/// for a sliding friction support, its slip sense, from the relative-displacement row
/// when the case publishes one and otherwise the zero-ground displacement row.
pub(crate) fn reported_state(results: &[ResultItem], case: &str, support: &str) -> &'static str {
    let row = |kind: &str| {
        results.iter().find(|r| r.kind == kind && r.entity_ref == support && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case))
    };
    match row(STATE_CODE_KIND).map(|r| r.value) {
        None => "unreported",
        Some(c) if c == 0.0 => "inactive",
        Some(c) if c == 1.0 => "active",
        Some(c) if c == 2.0 => "sticking",
        Some(c) if c == 3.0 => match row(RELATIVE_DISPLACEMENT_KIND).or_else(|| row(DISPLACEMENT_KIND)).map(|r| r.value) {
            Some(v) if v > 0.0 => "sliding (positive slip)",
            Some(v) if v < 0.0 => "sliding (negative slip)",
            _ => "sliding (slip sense unavailable)",
        },
        Some(_) => "unknown",
    }
}

/// §2.2: equal reported states, with every slip sense available.
pub(crate) fn same_state(states: &[&str]) -> bool {
    states.iter().all(|s| *s == states[0] && *s != "unreported" && !s.contains("unavailable"))
}

/// Slice 1a §2.1: in a model with nonlinear supports, one operand record per
/// admitted combination, in gate (evaluation) order: a subtraction, a range
/// envelope, an absolute sum or SRSS, or a mechanics combination admitted by
/// decision OQ-1 (absolute-sum-srss §5.2); every other mechanics combination is
/// withheld there. The operands' numerical quality and both notices are stated on
/// the published envelope (`state_operand_standing`), so a blocked envelope
/// carries neither. Returns no record for any other model.
pub(crate) fn append_operand_standing(model: &PreviewModel, rendered: &Rendered, gates: &[serde_json::Value]) -> Vec<serde_json::Value> {
    if !rendered.nonlinear {
        return Vec::new();
    }
    let resolver = combination_plan::Resolver::new(model);
    gates
        .iter()
        .filter(|g| g["withheld"] == false)
        .filter_map(|g| model.combinations.iter().find(|c| g["combination_id"] == c.id.as_str()))
        .filter_map(|combination| {
            let expression = CombinationExpression::resolve(combination)?;
            Some(serde_json::json!({
                "combination_id": combination.id,
                "basis": combination.basis,
                "expression": exact_combinations::expression_evidence(combination),
                "operands": expression.targets(&resolver).iter().map(|target| exact_combinations::operand_evidence(model, target)).collect::<Vec<_>>(),
                "standing": exact_combinations::STANDING,
            }))
        })
        .collect()
}

/// §2.2: the support-state notice of one record, from the published state rows.
fn operand_states_diagnostic(id: &str, operands: &[&str], results: &[ResultItem]) -> Option<Diagnostic> {
    let in_case = |r: &&ResultItem, case: &str| r.kind == STATE_CODE_KIND && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case);
    // Supports in order of first appearance over the operands, each operand's
    // state rows in published order.
    let mut supports: Vec<&str> = Vec::new();
    for case in operands {
        for row in results.iter().filter(|r| in_case(r, case)) {
            if !supports.contains(&row.entity_ref.as_str()) {
                supports.push(&row.entity_ref);
            }
        }
    }
    let mut differing: Vec<(&str, Vec<&'static str>)> = Vec::new();
    for support in supports {
        let states: Vec<&'static str> = operands.iter().map(|case| reported_state(results, case, support)).collect();
        if !same_state(&states) {
            differing.push((support, states));
        }
    }
    if differing.is_empty() {
        return None;
    }
    let mut named = differing
        .iter()
        .take(STATES_NAMED)
        .map(|(support, states)| {
            let per_case = operands.iter().zip(states).map(|(case, state)| format!("{case}: {state}")).collect::<Vec<_>>().join(", ");
            format!("{support} ({per_case})")
        })
        .collect::<Vec<_>>()
        .join("; ");
    if differing.len() > STATES_NAMED {
        named.push_str(&format!("; and {} more", differing.len() - STATES_NAMED));
    }
    Some(diag(
        &format!("diagnostic:preview-physics:combination-operand-states:{}", identity(&[id])),
        OPERAND_STATES_DIFFER,
        "info",
        format!(
            "combination {id}: the reported final state of {} nonlinear support(s) differs between its operand cases: {named}. Each operand case is solved independently from the model's authored initial support states, under the static, history-free support model, and keeps its own reported support state; the combination's rows are derived from those case results and are not one solved state.",
            differing.len()
        ),
        std::iter::once(id.to_string()).chain(differing.iter().map(|(s, _)| s.to_string())).collect(),
    ))
}

/// Slice 1a §2.1-§2.2, on the published envelope: each operand's numerical quality
/// (the case's assessed record, verbatim), the support-state and not-passed
/// notices, and the member, placed only when there is a record.
pub(crate) fn state_operand_standing(envelope: &mut MechanicsEnvelope, mut records: Vec<serde_json::Value>) {
    if records.is_empty() {
        return;
    }
    exact_combinations::state_operand_standing(&mut records, &envelope.numerical_quality);
    for record in &records {
        let id = record["combination_id"].as_str().unwrap_or_default();
        // A combination operand states its own standing in its own record (§4).
        let operands: Vec<&serde_json::Value> = record["operands"].as_array().into_iter().flatten().filter(|o| o.get("load_case_id").is_some()).collect();
        let cases: Vec<&str> = operands.iter().map(|o| o["load_case_id"].as_str().unwrap_or_default()).collect();
        if let Some(notice) = operand_states_diagnostic(id, &cases, &envelope.results) {
            envelope.diagnostics.push(notice);
        }
        let failing: Vec<(&str, &str)> = operands
            .iter()
            .map(|o| (o["load_case_id"].as_str().unwrap_or_default(), o["numerical_quality"]["solve_quality"].as_str().unwrap_or("not_assessed")))
            .filter(|(_, quality)| *quality != "checks_passed")
            .collect();
        if failing.is_empty() {
            continue;
        }
        let named = failing.iter().map(|(case, quality)| format!("{case} ({quality})")).collect::<Vec<_>>().join(", ");
        envelope.diagnostics.push(diag(
            &format!("diagnostic:preview-physics:combination-operand-standing:{}", identity(&[id])),
            OPERAND_CHECKS_NOT_PASSED,
            "warning",
            format!("combination {id} uses operand case(s) whose numerical integrity checks did not pass: {named}; its rows are still published, as derived inspection results without their own numerical enclosure; each operand's standing is stated in contract_evidence.combination_operands"),
            std::iter::once(id.to_string()).chain(failing.iter().map(|(case, _)| case.to_string())).collect(),
        ));
    }
    if let Some(evidence) = envelope.contract_evidence.as_mut().and_then(serde_json::Value::as_object_mut) {
        evidence.insert(OPERANDS_MEMBER.to_string(), serde_json::Value::Array(records));
    }
}

#[cfg(test)]
#[path = "silent_drop_unit_tests.rs"]
mod silent_drop_unit_tests;

#[cfg(test)]
mod tests {
    use super::*;

    fn nonlinear(id: &str, node_index: usize, dof: FrameDof) -> NonlinearSupport {
        NonlinearSupport { support_id: id.into(), node_index, dof, behavior: NonlinearSupportBehavior::Friction, gap: None, friction_coefficient: None }
    }

    fn straight(pipe: &str, maximum: Result<CertifiedStressMaximum, String>) -> MemberRecord {
        MemberRecord { pipe_id: pipe.into(), arc: false, maximum: Some(maximum), end_resultants: [[0.0; 6]; 2], section_modulus: 1.0, certified_end_bending:None }
    }
    fn certified(value: f64) -> CertifiedStressMaximum {
        CertifiedStressMaximum { span_index: 0, local_fraction: 0.0, station: 0.0, value_lower: value, value_upper: value, upper_bound: value, certified_gap: 0.0, subdivisions: 0 }
    }

    /// Seam test: an unavailable member makes coverage incomplete; the headline
    /// is then withheld by `render` rather than taken from the available subset.
    #[test]
    fn coverage_reports_unavailable_and_outside_domain_members() {
        let mut diagnostics = Vec::new();
        let members = vec![
            straight("pipe:b", Ok(certified(5.0))),
            straight("pipe:a", Ok(certified(5.0))),
            straight("pipe:c", Err("enclosure failed".into())),
            MemberRecord { pipe_id: "pipe:arc".into(), arc: true, maximum: None, end_resultants: [[0.0; 6]; 2], section_modulus: 1.0, certified_end_bending:None },
        ];
        let out = render_members("case:1", &members, &HashSet::new(), &mut diagnostics);
        assert_eq!(out.unavailable, vec!["pipe:c".to_string()]);
        assert_eq!(out.outside, vec!["pipe:arc".to_string()]);
        assert_eq!(out.extrema.len(), 2);
        // Identity tie: the smaller member id wins on an exactly equal value.
        assert_eq!(out.case_max.as_ref().unwrap().location_ref, "pipe:a");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "PREVIEW_STRESS_MAXIMUM_UNAVAILABLE");
        assert_eq!(diagnostics[0].affected_refs, vec!["case:1".to_string(), "pipe:c".to_string()]);
    }

    /// Both ambiguous shapes are refused before solving today (REF-ATTR and the
    /// nonlinear+nonlinear runtime variant), so the rule is pinned here directly.
    #[test]
    fn only_two_residual_devices_on_one_node_dof_are_ambiguous() {
        let guide = LinearSupport { support_id: "support:guide".into(), family: SupportFamily::Guide, node_index: 1, restrained_dofs: vec![FrameDof::Uy], stiffness: None, imposed_displacement: None };
        let anchor = LinearSupport::anchor("support:a", 0);
        let restrained: HashSet<usize> = (0..6).chain([DOF_PER_NODE + 1]).collect();
        // Rigid + nonlinear on UY at node 1.
        let found = ambiguous_supports(&[anchor.clone(), guide.clone()], &[nonlinear("support:one-way", 1, FrameDof::Uy)], &restrained);
        assert_eq!(found, HashSet::from(["support:guide".to_string(), "support:one-way".to_string()]));
        // Nonlinear + nonlinear on one DOF.
        let found = ambiguous_supports(&[anchor.clone()], &[nonlinear("support:g1", 1, FrameDof::Uz), nonlinear("support:g2", 1, FrameDof::Uz)], &restrained);
        assert_eq!(found, HashSet::from(["support:g1".to_string(), "support:g2".to_string()]));
        // Different DOFs, or a rigid row that is not restrained (a spring DOF), are not ambiguous.
        assert!(ambiguous_supports(&[anchor.clone(), guide.clone()], &[nonlinear("support:one-way", 1, FrameDof::Uz)], &restrained).is_empty());
        let spring_restrained: HashSet<usize> = (0..6).collect();
        assert!(ambiguous_supports(&[anchor, guide], &[nonlinear("support:one-way", 1, FrameDof::Uy)], &spring_restrained).is_empty());
    }

    #[test]
    fn limitations_match_the_frozen_table() {
        let table: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json"
        ))
        .unwrap();
        assert_eq!(table["semantic_contract_id"], ID);
        let frozen: Vec<&str> = table["supported_profile_limitations"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(frozen, formulation_basis().limitations);
        assert!(table.get("retired_source_kinds").is_none());
        let linear: Vec<&str> = table["combination_policy"]["linear_state_kinds"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(linear, LINEAR_STATE_KINDS.to_vec());
        assert!(table["rows"].as_array().unwrap().iter().any(|r| r["kind"] == INTENSIFIED_KIND));
    }

    /// The gate's own refusal of a factor sum it cannot evaluate. Validation refuses a
    /// non-finite factor first (LOAD_COMBINATION_FACTOR_INVALID); the gate still withholds
    /// by name rather than reading a NaN or infinite fold as a unit sum (main's
    /// `(sum - 1).abs() > bound` was false for NaN and for inf against an inf bound).
    #[test]
    fn constant_effort_gate_withholds_a_factor_sum_it_cannot_evaluate() {
        let model: PreviewModel = serde_json::from_value(serde_json::json!({
            "schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
            "project":{"id":"project:t","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
            "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
            "nodes":[],"pipe_segments":[],"supports":[],"components":[],"materials":[],
            "load_cases":[{"id":"case:W1","primitive_loads":[],"provenance":"t"},{"id":"case:W2","primitive_loads":[],"provenance":"t"}],
            "combinations":[]
        }))
        .unwrap();
        let combination = |factors: [f64; 2]| {
            let mut c: PreviewCombination = serde_json::from_value(serde_json::json!({"id":"comb:c","basis":"mechanics",
                "terms":[{"load_case":"case:W1","factor":0.5},{"load_case":"case:W2","factor":0.5}],"provenance":"t"}))
            .unwrap();
            c.terms[0].factor = factors[0];
            c.terms[1].factor = factors[1];
            c
        };
        let rendered = |consuming_constant_effort| Rendered { preview_cases: Vec::new(), nonlinear: false, consuming_constant_effort };
        let ce = Some("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE");
        let gate_reason = |rendered: &Rendered, model: &PreviewModel, combination: &PreviewCombination| {
            let mut model = model.clone();
            model.combinations = vec![combination.clone()];
            let plan = combination_plan::Plan::new(&model);
            let expression = CombinationExpression::resolve(&model.combinations[0]).unwrap();
            let targets = expression.targets(&plan.resolver);
            gate_reason(rendered, &model, &plan, 0, &targets, &HashMap::new()).map(|(code, _)| code)
        };
        for factors in [[f64::NAN, 1.0], [f64::INFINITY, -f64::INFINITY], [0.5, f64::NEG_INFINITY]] {
            assert_eq!(gate_reason(&rendered(true), &model, &combination(factors)), ce, "{factors:?}");
        }
        for factors in [[2f64.powi(53), -2f64.powi(53)], [f64::MAX, f64::MAX]] {
            assert_eq!(gate_reason(&rendered(true), &model, &combination(factors)), ce, "{factors:?}");
            // Without a consuming constant-effort support the factor sum is not a gate.
            assert_eq!(gate_reason(&rendered(false), &model, &combination(factors)), None, "{factors:?}");
        }
        for factors in [[0.5, 0.5], [0.3, 0.7], [1.2, -0.2], [1.0 / 3.0, 2.0 / 3.0]] {
            assert_eq!(gate_reason(&rendered(true), &model, &combination(factors)), None, "{factors:?}");
        }
    }
}
