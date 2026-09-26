//! preview-physics-1: the ordinary route's repaired and withheld quantities.
//!
//! The case loop keeps producing its historical rows plus a side record. Once
//! every case has solved and no case was source-selected, `render` replaces the
//! retired rows, sets the per-case headline inputs and edits the diagnostics.
//! Everything downstream (headlines, the high-displacement review, the
//! modifier count, combinations and the finite check) is then built once from
//! the rendered rows. Contract: T0R S1_INTERFACE.md.

use super::*;
use open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum;

pub(crate) const ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-1";
pub(crate) const RETIRED_KINDS: [&str; 3] = [
    "reaction_resultant",
    "open_formula_stress_summary",
    "component_user_stress_multiplier_review",
];
pub(crate) const RETIRED_CODES: [&str; 2] = [
    "COMPONENT_STRESS_MULTIPLIER_APPLIED",
    "COMBINATION_STRESS_SUMMARY_SKIPPED",
];
pub(crate) const INTENSIFIED_KIND: &str = "component_equal_factor_intensified_bending_stress_v1";
const ARC_BASIS: &str = "nominal_straight_beam_formula_on_arc_resultants";
const ARC_COORDINATE: &str = "arc_chord_frame";
const MAXIMUM_SIGN: &str = "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim";
const SUBTRACTION_SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; a signed difference of solved states, not an equilibrium state";
const WITHHELD_ATTRIBUTION: &str = "SUPPORT_ACTION_ATTRIBUTION_WITHHELD";
const WITHHELD_CONSTANT_EFFORT: &str = "CONSTANT_EFFORT_NOT_CONSUMED";
const SUPPORT_COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];

/// The linear state kinds admitted to combination algebra (table
/// `combination_policy.linear_state_kinds`).
const LINEAR_STATE_KINDS: [&str; 20] = [
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
];
const ARC_STRESS_KINDS: [&str; 6] = [
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
];
const END_FORCE_KINDS: [&str; 6] = [
    "element_local_axial_force",
    "element_local_shear_force_y",
    "element_local_shear_force_z",
    "element_local_torsional_moment",
    "element_local_bending_moment_y",
    "element_local_bending_moment_z",
];

pub(crate) const LIMITATIONS: [&str; 7] = [
    "Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.",
    "Nonzero pressure is refused on this route, and so is legacy imposed_displacement.",
    "On straight members the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No transverse shear, equivalent stress or code stress.",
    "On arcs: signed tangent-frame resultants and nominal stress components. Endpoint force rows are in the chord frame. No maximum, and no stress headline for a model containing an arc.",
    "Intensified measures are i*hypot(My,Mz)/Z with the user's scalar SIF at member ends adjacent to markers and branches, using member Z. They are not code stresses and are never combined. A flexibility factor never multiplies stress.",
    "Signed support actions for rigid restraints, springs, consuming constant effort and attributable nonlinear supports. Ambiguous attribution is withheld.",
    "Headlines cover all load cases. Mechanics combinations are withheld for nonlinear supports, constant effort with sum of factors other than 1, or mixed moduli. Subtraction is a labelled difference. No combination maxima and no code compliance.",
];

pub(crate) fn formulation_basis() -> FormulationBasis {
    FormulationBasis {
        profile_id: "product_preview_mechanics_v1".to_string(),
        limitations: LIMITATIONS.iter().map(|s| s.to_string()).collect(),
    }
}

/// A blocked envelope has no rows. Diagnostics pushed before the block keep
/// their meaning, but retired codes go and `result:` refs that can no longer
/// resolve are stripped, so the envelope still satisfies S1 §9 (R1 SF-1).
pub(crate) fn sanitize_blocked_diagnostics(diagnostics: &mut Vec<Diagnostic>) {
    // Retired codes and non-blocking diagnostics that name a (now absent) row
    // are dropped. A blocking diagnostic always stays, so the blocking reason
    // is never lost; only its unresolvable `result:` refs are stripped.
    diagnostics.retain(|d| {
        !RETIRED_CODES.contains(&d.code.as_str())
            && (d.severity == "blocking" || !d.affected_refs.iter().any(|r| r.starts_with("result:")))
    });
    for d in diagnostics.iter_mut() {
        d.affected_refs.retain(|r| !r.starts_with("result:"));
    }
}

/// M07 containment (T0R, ROOT ruling on R1 N-1): a user-stiffness joint with
/// nonzero lateral stiffness over a nonzero length has lateral springs without
/// the rigid-body moment coupling, so it is not in moment equilibrium and every
/// result of the model is suspect. Refuse the solve until T4 repairs it.
pub(crate) fn refuse_unqualified_joint_elements(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    // Only named in-crate historical tests can enter this scope; normal builds
    // have no selector (same custody as the historical pressure premise).
    #[cfg(test)]
    if crate::historical_pressure_reference::active() {
        return;
    }
    for component in model.components.iter().filter(|c| is_expansion_joint_component(c)) {
        if component.mechanics_interface.as_ref().and_then(|i| i.solver_consumption.as_deref())
            != Some("mechanics_geometry_and_user_flexibility")
        {
            continue;
        }
        let lateral = component.modifiers.as_ref().and_then(|m| m.lateral_stiffness_user_value.as_ref()).map(|q| q.value);
        if !lateral.is_some_and(|k| k != 0.0) {
            continue;
        }
        let Some(pipe) = component
            .geometry
            .as_ref()
            .and_then(|g| g.expansion_joint_pipe_ref.as_deref())
            .and_then(|id| model.pipe_segments.iter().find(|p| p.id == id))
        else {
            continue;
        };
        let position = |id: &str| model.nodes.iter().find(|n| n.id == id).map(|n| [n.position.x, n.position.y, n.position.z]);
        let (Some(a), Some(b)) = (position(&pipe.from), position(&pipe.to)) else { continue };
        if a == b {
            continue;
        }
        diagnostics.push(diag(
            &format!("diagnostic:preview-physics:joint-equilibrium:{}", identity(&[&component.id])),
            "JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED",
            "blocking",
            format!("expansion joint {} realizes user lateral stiffness over the length of {} without the rigid-body moment coupling, so the element is not in moment equilibrium and no result of this model can be published; the solve is refused until the joint element is repaired (M07, T4). Axial, angular and torsional joint stiffness alone are unaffected", component.id, pipe.id),
            vec![component.id.clone(), pipe.id.clone()],
        ));
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
    /// `None` on an arc; the certified enclosure or its failure on a straight member.
    pub maximum: Option<Result<CertifiedStressMaximum, String>>,
    /// j-side section-cut resultants at end_i and end_j.
    pub end_resultants: [[f64; 6]; 2],
    pub section_modulus: f64,
}

/// Side record of one solved ordinary case.
#[derive(Debug, Clone, Default)]
pub(crate) struct CaseRecord {
    /// Per model support with a node: the device-law six-vector from the case loop.
    pub support_vectors: Vec<(String, [f64; 6])>,
    pub members: Vec<MemberRecord>,
}

/// Model facts that gate `mechanics` combinations.
#[derive(Debug, Clone)]
pub(crate) struct Rendered {
    pub preview_cases: Vec<serde_json::Value>,
    nonlinear: bool,
    consuming_constant_effort: bool,
}

impl Rendered {
    pub(crate) fn evidence(&self, gates: Vec<serde_json::Value>) -> serde_json::Value {
        serde_json::json!({"preview_cases": self.preview_cases, "combination_gates": gates})
    }
}

fn identity(parts: &[&str]) -> String {
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

fn maximum_row(case_id: &str, pipe_id: &str, value: f64) -> ResultItem {
    ResultItem {
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
            sign_convention: MAXIMUM_SIGN.to_string(),
        }),
    }
}

fn extrema_evidence(pipe_id: &str, result_id: &str, m: &CertifiedStressMaximum) -> serde_json::Value {
    serde_json::json!({"pipe_id":pipe_id,"result_id":result_id,
        "station_fraction":m.station,"span_index":m.span_index,"local_fraction":m.local_fraction,
        "value_lower_pa":m.value_lower,"value_upper_pa":m.value_upper,"global_upper_bound_pa":m.upper_bound,
        "certified_gap_pa":m.certified_gap,"subdivisions":m.subdivisions,
        "approximation":"piecewise_quadratic_straight_section_statics","coefficient_basis":"j_side_section_equilibrium_binary64",
        "enclosure_scope":"supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate"})
}

/// One intensified-measure association of a component with a pipe end.
struct Association<'a> {
    component: &'a PreviewComponent,
    role: &'static str,
    side_label: &'static str,
    sif: f64,
    source_reference: &'a str,
}

fn user_sif(value: Option<&Quantity>) -> Option<f64> {
    value.map(|q| q.value).filter(|v| positive_finite(*v))
}

fn solver_consumption(component: &PreviewComponent) -> &str {
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
fn associations<'a>(model: &'a PreviewModel, node: &str, pipe_id: &str) -> Vec<Association<'a>> {
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
                let angle = cross[0].hypot(cross[1]).hypot(cross[2]).atan2(dot);
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

fn render_members(case_id: &str, members: &[MemberRecord], diagnostics: &mut Vec<Diagnostic>) -> MemberRender {
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
                let value = maximum.value_lower + 0.5 * (maximum.value_upper - maximum.value_lower);
                let row = maximum_row(case_id, &member.pipe_id, value);
                out.extrema.push(extrema_evidence(&member.pipe_id, &row.id, maximum));
                if out.case_max.as_ref().is_none_or(|q| value > q.value || (value == q.value && member.pipe_id < q.location_ref)) {
                    out.case_max = Some(LocatedQuantity { value, unit: "Pa".to_string(), location_ref: member.pipe_id.clone(), result_ref: row.id.clone() });
                }
                out.maximum_rows.insert(member.pipe_id.clone(), row);
            }
        }
    }
    out
}

/// Replace the retired rows of every case, set the per-case headline inputs
/// and edit the diagnostics (S1 §3-§8). Pure over the solved cases.
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
        let record = solve.preview.take().unwrap_or_default();

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
        let MemberRender { mut maximum_rows, extrema, unavailable, outside, case_max } =
            render_members(&case_id, &record.members, diagnostics);
        let complete = unavailable.is_empty() && outside.is_empty();
        for pipe in unavailable.iter().chain(&outside) {
            incomplete_pipes.insert(pipe.clone(), ());
        }

        // Rebuild the rows: support and maximum rows take the place of the
        // retired rows; withheld nonlinear reactions are removed.
        let old = std::mem::take(&mut solve.results);
        let mut rows = Vec::with_capacity(old.len());
        for mut row in old {
            let retired = RETIRED_KINDS.contains(&row.kind.as_str())
                || (row.kind == "nonlinear_support_final_reaction" && withheld_nonlinear.contains(row.entity_ref.as_str()));
            if retired {
                removed_ids.insert(final_id(case_index, &case_id, &row));
                if row.kind == "reaction_resultant" {
                    rows.extend(support_rows.remove(&row.entity_ref).unwrap_or_default());
                } else if row.kind == "open_formula_stress_summary" {
                    rows.extend(maximum_rows.remove(&row.entity_ref));
                }
                continue;
            }
            if arc_pipes.contains(row.entity_ref.as_str()) {
                if let Some(metadata) = row.metadata.as_mut() {
                    if ARC_STRESS_KINDS.contains(&row.kind.as_str()) {
                        metadata.basis = ARC_BASIS.to_string();
                    } else if END_FORCE_KINDS.contains(&row.kind.as_str())
                        && matches!(metadata.location.as_str(), "end_i" | "end_j")
                        && metadata.basis == SECTION_RESULTANT_BASIS
                        && metadata.coordinate_system == "element_local"
                        && metadata.sign_convention.starts_with("positive value follows the element-local DOF at the")
                    {
                        metadata.coordinate_system = ARC_COORDINATE.to_string();
                    }
                }
            }
            rows.push(row);
        }
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
                    let value = association.sif * (my.hypot(mz) / member.section_modulus);
                    let endpoint = endpoint_id_location(location);
                    let base_id = format!(
                        "result:intensified-bending:{}:{}:{endpoint}",
                        stable_suffix(&association.component.id),
                        stable_suffix(&member.pipe_id)
                    );
                    let row = ResultItem {
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

    // Diagnostics: retired codes and anything naming a removed row go.
    diagnostics.retain(|d| {
        !RETIRED_CODES.contains(&d.code.as_str()) && !d.affected_refs.iter().any(|r| removed_ids.contains(r))
    });
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
            "stress headline withheld: a stress maximum is not available for every member in every load case (arc members have no maximum on this route)",
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

fn gate_reason(
    rendered: &Rendered,
    model: &PreviewModel,
    combination: &PreviewCombination,
) -> Option<&'static str> {
    if combination.basis != "mechanics" {
        return None;
    }
    if rendered.nonlinear {
        return Some("NONLINEAR_COMBINATION_REQUIRES_SOLVE");
    }
    let sum: f64 = combination.terms.iter().map(|t| t.factor).sum();
    let abs_sum: f64 = combination.terms.iter().map(|t| t.factor.abs()).sum();
    if rendered.consuming_constant_effort && (sum - 1.0).abs() > 64.0 * f64::EPSILON * abs_sum.max(1.0) {
        return Some("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE");
    }
    let mut scratch = Vec::new();
    let keys: HashSet<Option<String>> = combination
        .terms
        .iter()
        .filter_map(|term| model.load_cases.iter().find(|c| c.id == term.load_case))
        .map(|case| modulus_basis_key(case, &mut scratch))
        .collect();
    // Selection conflicts were refused before solving; the scratch list is discarded.
    if keys.len() > 1 {
        return Some("COMBINATION_MODULUS_BASIS_MIXED");
    }
    None
}

fn gate_message(code: &str) -> &'static str {
    match code {
        "NONLINEAR_COMBINATION_REQUIRES_SOLVE" => "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
        "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE" => "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
        _ => "Combination withheld: its load cases were solved on different modulus bases.",
    }
}

fn combination_row(
    combination: &PreviewCombination,
    expression: &CombinationExpression<'_>,
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
        metadata.sign_convention = match expression {
            CombinationExpression::ResultStateSubtraction { .. } => SUBTRACTION_SIGN.to_string(),
            _ => expression.result_sign_convention(),
        };
    }
    combined
}

fn evaluate_values(
    expression: &CombinationExpression<'_>,
    values: &[(&str, f64)],
    dimension: LoadDimension,
) -> Option<f64> {
    let operands = values
        .iter()
        .map(|(case, value)| {
            AlgebraQuantity::new(*value, dimension).ok().map(|quantity| {
                AlgebraOperand::new(
                    case.to_string(),
                    case.to_string(),
                    quantity,
                    vec![AlgebraAnalysisStatus::MechanicsSolved, AlgebraAnalysisStatus::HumanReviewRequired],
                )
            })
        })
        .collect::<Option<Vec<_>>>()?;
    let by_id: HashMap<&str, &AlgebraOperand> = operands.iter().map(|o| (o.operand_id.as_str(), o)).collect();
    let algebra = expression.evaluate(&by_id);
    if algebra.is_blocked() {
        return None;
    }
    algebra.quantity.map(|q| q.value)
}

/// Combination rows for preview-physics-1 (S1 §7). Returns the evidence gates.
pub(crate) fn append_combination_results(
    model: &PreviewModel,
    rendered: &Rendered,
    rows_by_base_id: &HashMap<String, HashMap<String, ResultItem>>,
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<serde_json::Value> {
    let mut base_ids = rows_by_base_id.keys().cloned().collect::<Vec<_>>();
    base_ids.sort();
    let case_rows: Vec<ResultItem> = results
        .iter()
        .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case"))
        .cloned()
        .collect();
    let has_maximum = case_rows.iter().any(|r| r.kind == "pipe_elastic_normal_stress_maximum_v2");
    let has_intensified = case_rows.iter().any(|r| r.kind == INTENSIFIED_KIND);
    let mut gates = Vec::new();
    for combination in &model.combinations {
        let Some(expression) = CombinationExpression::resolve(combination) else {
            gates.push(serde_json::json!({"combination_id": combination.id, "withheld": false, "reason": null}));
            continue;
        };
        if let Some(code) = gate_reason(rendered, model, combination) {
            diagnostics.push(diag(
                &format!("diagnostic:preview-physics:combination-gate:{}", identity(&[&combination.id])),
                code,
                "warning",
                gate_message(code),
                vec![combination.id.clone()],
            ));
            gates.push(serde_json::json!({"combination_id": combination.id, "withheld": true, "reason": code}));
            continue;
        }
        gates.push(serde_json::json!({"combination_id": combination.id, "withheld": false, "reason": null}));
        let range = matches!(expression, CombinationExpression::RangeEnvelope { .. });
        let operands = expression.operand_ids();

        // Ordinary rows grouped by base id.
        for base_id in &base_ids {
            let source_rows = &rows_by_base_id[base_id];
            let Some(reference) = source_rows.get(expression.reference_operand_id()) else { continue };
            let kind = reference.kind.as_str();
            if kind.ends_with("_v2") {
                continue;
            }
            if kind == "displacement_magnitude" && !range {
                append_combined_vector_magnitude(combination, &expression, base_id, reference, rows_by_base_id, &HashMap::new(), results, diagnostics);
                continue;
            }
            if !(LINEAR_STATE_KINDS.contains(&kind) || (range && kind == "displacement_magnitude")) {
                continue;
            }
            let Some(dimension) = algebra_dimension(reference) else { continue };
            let mut values = Vec::new();
            let mut refs = Vec::new();
            let mut complete = true;
            for case in &operands {
                match source_rows.get(*case).filter(|s| combination_source_identity_matches(reference, s)) {
                    Some(source) => {
                        values.push((*case, source.value));
                        refs.push(source.id.clone());
                    }
                    None => complete = false,
                }
            }
            if !complete {
                diagnostics.push(diag(
                    &format!("diagnostic:preview-physics:combination-source-missing:{}", identity(&[&combination.id, base_id])),
                    "LOAD_COMBINATION_SOURCE_RESULT_MISSING",
                    "warning",
                    format!("combination {} omits {base_id} because a matching source row is missing in an operand case", combination.id),
                    vec![combination.id.clone()],
                ));
                continue;
            }
            if let Some(value) = evaluate_values(&expression, &values, dimension) {
                results.push(combination_row(combination, &expression, reference, qualified_combination_result_id(&combination.id, base_id), value, refs));
            }
        }

        // Signed support actions, grouped by (support, component).
        for support in &model.supports {
            let find = |case: &str, component: &str| {
                case_rows.iter().find(|r| {
                    r.entity_ref == support.id
                        && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case)
                        && r.metadata.as_ref().is_some_and(|m| m.component == component)
                        && r.kind.starts_with("support_reaction_")
                })
            };
            if operands.iter().any(|case| find(case, "Fx").is_none()) {
                continue;
            }
            let base = |component: &str| format!("support-action:{}:{}:{component}", support.id.len(), support.id);
            let mut combined = [0.0; 6];
            let mut complete = true;
            for (slot, component) in SUPPORT_COMPONENTS.iter().enumerate() {
                let sources: Vec<&ResultItem> = operands.iter().filter_map(|case| find(case, component)).collect();
                let dimension = if slot < 3 { LoadDimension::Force } else { LoadDimension::Moment };
                let values: Vec<(&str, f64)> = operands.iter().copied().zip(sources.iter().map(|s| s.value)).collect();
                match evaluate_values(&expression, &values, dimension) {
                    Some(value) => {
                        combined[slot] = value;
                        results.push(combination_row(combination, &expression, sources[0], qualified_combination_result_id(&combination.id, &base(component)), value, sources.iter().map(|s| s.id.clone()).collect()));
                    }
                    None => complete = false,
                }
            }
            if !complete {
                continue;
            }
            for (component, value) in [
                ("force_magnitude", combined[0].hypot(combined[1]).hypot(combined[2])),
                ("moment_magnitude", combined[3].hypot(combined[4]).hypot(combined[5])),
            ] {
                let sources: Vec<&ResultItem> = operands.iter().filter_map(|case| find(case, component)).collect();
                let value = if range {
                    let dimension = if component == "force_magnitude" { LoadDimension::Force } else { LoadDimension::Moment };
                    let values: Vec<(&str, f64)> = operands.iter().copied().zip(sources.iter().map(|s| s.value)).collect();
                    match evaluate_values(&expression, &values, dimension) {
                        Some(value) => value,
                        None => continue,
                    }
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
                results.push(combination_row(combination, &expression, sources[0], qualified_combination_result_id(&combination.id, &base(component)), value, refs));
            }
        }
        if has_maximum {
            diagnostics.push(diag(
                &format!("diagnostic:preview-physics:combination-maximum:{}", identity(&[&combination.id])),
                "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE",
                "info",
                "stress maxima are not combined; combine the signed stress component rows instead (combination maxima are T6)",
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
    }
    gates
}

/// SF-E: a pre-0.4 request whose primitive load targets a support is refused
/// with a targeted diagnostic. Nothing is solved from the cleaned clone.
pub(crate) fn imposed_displacement_refusal(
    actual_request: &serde_json::Value,
) -> Option<Result<MechanicsEnvelope, String>> {
    let model = actual_request.get("model")?;
    if model.get("schema_version").and_then(|v| v.as_str()) == Some("0.4.0") {
        return None;
    }
    let mut offending = Vec::new();
    for case in model.get("load_cases")?.as_array()? {
        let case_id = case.get("id").and_then(|v| v.as_str()).unwrap_or("");
        for load in case.get("primitive_loads").and_then(|v| v.as_array()).into_iter().flatten() {
            if load.pointer("/target/type").and_then(|v| v.as_str()) == Some("support") {
                offending.push((
                    case_id.to_string(),
                    load.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    load.pointer("/target/support").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                ));
            }
        }
    }
    if offending.is_empty() {
        return None;
    }
    let mut clone = actual_request.clone();
    if let Some(cases) = clone.pointer_mut("/model/load_cases").and_then(|v| v.as_array_mut()) {
        for case in cases {
            if let Some(loads) = case.get_mut("primitive_loads").and_then(|v| v.as_array_mut()) {
                loads.retain(|load| load.pointer("/target/type").and_then(|v| v.as_str()) != Some("support"));
            }
        }
    }
    let request: LinearStaticPreviewRequest = match serde_json::from_value(clone) {
        Ok(request) => request,
        Err(_) => return None,
    };
    let diagnostics = offending
        .into_iter()
        .map(|(case_id, load_id, support_id)| {
            diag(
                &format!("diagnostic:preview-physics:imposed-displacement:{}", identity(&[&case_id, &load_id])),
                "IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL",
                "blocking",
                format!("load {load_id} in {case_id} imposes a displacement on support {support_id}; this model version has no load-state boundary motion, so the request is refused and nothing is solved (author it in a load-state model)"),
                vec![load_id, case_id, support_id],
            )
        })
        .collect();
    Some(Ok(blocked_envelope(request.model, diagnostics)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nonlinear(id: &str, node_index: usize, dof: FrameDof) -> NonlinearSupport {
        NonlinearSupport { support_id: id.into(), node_index, dof, behavior: NonlinearSupportBehavior::Friction, gap: None, friction_coefficient: None }
    }

    fn straight(pipe: &str, maximum: Result<CertifiedStressMaximum, String>) -> MemberRecord {
        MemberRecord { pipe_id: pipe.into(), arc: false, maximum: Some(maximum), end_resultants: [[0.0; 6]; 2], section_modulus: 1.0 }
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
            MemberRecord { pipe_id: "pipe:arc".into(), arc: true, maximum: None, end_resultants: [[0.0; 6]; 2], section_modulus: 1.0 },
        ];
        let out = render_members("case:1", &members, &mut diagnostics);
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
            "../../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json"
        ))
        .unwrap();
        assert_eq!(table["semantic_contract_id"], ID);
        let frozen: Vec<&str> = table["supported_profile_limitations"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(frozen, LIMITATIONS.to_vec());
        let retired: Vec<&str> = table["retired_source_kinds"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(retired, RETIRED_KINDS.to_vec());
        let linear: Vec<&str> = table["combination_policy"]["linear_state_kinds"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(linear, LINEAR_STATE_KINDS.to_vec());
        assert!(table["rows"].as_array().unwrap().iter().any(|r| r["kind"] == INTENSIFIED_KIND));
    }
}
