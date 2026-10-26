//! W-15 slice a (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §2):
//! DEC-045 stiffness scaling of a rigid-family component's mapped member.
//!
//! A valve, flange, reducer, rigid or specialty component maps one straight
//! member, `geometry.rigid_pipe_ref`, that ends at its node. When the single
//! eligibility predicate here holds (§2.3), the member is formed with s·E and
//! s·G on its own pipe section (`build_model_for_members`); otherwise it
//! solves as unscaled pipe and the validation diagnostics name the first unmet
//! condition. Formation and diagnostics read the same predicate. The three
//! mapping conflicts block by name. No scale, stiffness or other component
//! value is ever supplied by the software.

use crate::{diag, stable_suffix, Diagnostic, PreviewComponent, PreviewModel, PreviewPipe};
use std::collections::{BTreeMap, HashMap, HashSet};

/// The DEC-045 consumption mode of a rigid-family component.
pub(crate) const GEOMETRY_ONLY: &str = "mechanics_geometry_only";
/// §2.5: from this scale the precision warning is emitted. It is the lowest
/// decade at which the W-15 rigid-link benchmark
/// (`validation/references/w15_rigid_scaling`, s = 1e3 to 1e8 by decade, both
/// routes and solver modes, Timoshenko on EX) has a row outside the DEC-026
/// 1e-9 criterion: at 1e4 every row is within 1e-15, at 1e5 the
/// Euler-Bernoulli rows reach 3.8e-9. A numerical heuristic measured on one
/// benchmark, not an accuracy bound or an engineering limit.
pub(crate) const PRECISION_WARNING_SCALE: f64 = 1.0e5;

pub(crate) const MAPPING_INVALID: &str = "RIGID_COMPONENT_MAPPING_INPUT_INVALID";
pub(crate) const CONSUMPTION_UNDECLARED: &str = "RIGID_COMPONENT_CONSUMPTION_UNDECLARED";
pub(crate) const MAPPING_CONFLICT: &str = "RIGID_COMPONENT_MAPPING_CONFLICT";
pub(crate) const STIFFNESS_MISSING: &str = "RIGID_COMPONENT_STIFFNESS_INPUT_MISSING";
pub(crate) const STIFFNESS_INVALID: &str = "RIGID_COMPONENT_STIFFNESS_INPUT_INVALID";
pub(crate) const SCALING_APPLIED: &str = "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED";
pub(crate) const SCALE_PRECISION: &str = "RIGID_COMPONENT_SCALE_PRECISION";

/// The rigid family: valve, flange, reducer, rigid and specialty.
pub(crate) fn is_rigid_family(component: &PreviewComponent) -> bool {
    matches!(component.kind.as_str(), "valve" | "flange" | "reducer" | "rigid" | "specialty")
}

/// The component's nonblank mapped member reference, if any.
pub(crate) fn mapped_pipe_ref(component: &PreviewComponent) -> Option<&str> {
    component
        .geometry
        .as_ref()
        .and_then(|geometry| geometry.rigid_pipe_ref.as_deref())
        .filter(|value| !value.trim().is_empty())
}

/// The first unmet §2.3 condition, in the order listed there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Unmet {
    /// The component is not rigid-family.
    NotRigidFamily,
    /// `mechanics_interface.solver_consumption` is absent: no mode is assumed.
    ConsumptionUndeclared,
    /// A declared mode other than `mechanics_geometry_only` (including the
    /// honoured `not_solver_consumed`); the existing unsupported-interface
    /// diagnostic is the only one.
    ConsumptionNotGeometryOnly,
    /// The mapped member is absent, unknown or does not end at the node.
    MappingInvalid,
    /// The scale or its source reference is missing.
    ScaleMissing,
    /// The scale is not finite positive.
    ScaleInvalid,
    /// A supplied linear or rotational stiffness is not finite positive.
    OptionalStiffnessInvalid,
}

/// §2.3's conditions after the consumption mode: a valid mapping, a present,
/// finite, positive scale with a nonblank source reference, and every supplied
/// linear or rotational stiffness finite positive. Returns the mapped member
/// and the scale.
fn input_conditions<'a>(
    component: &'a PreviewComponent,
    pipes: &HashMap<&str, &PreviewPipe>,
) -> Result<(&'a str, f64), Unmet> {
    let pipe_ref = mapped_pipe_ref(component).ok_or(Unmet::MappingInvalid)?;
    let pipe = pipes.get(pipe_ref).ok_or(Unmet::MappingInvalid)?;
    if pipe.from != component.node && pipe.to != component.node {
        return Err(Unmet::MappingInvalid);
    }
    let modifiers = component.modifiers.as_ref().ok_or(Unmet::ScaleMissing)?;
    let scale = modifiers.stiffness_scaling_user_value.as_ref().ok_or(Unmet::ScaleMissing)?.value;
    if modifiers.source_reference.as_deref().map_or(true, |value| value.trim().is_empty()) {
        return Err(Unmet::ScaleMissing);
    }
    if !scale.is_finite() || scale <= 0.0 {
        return Err(Unmet::ScaleInvalid);
    }
    let optional_invalid = [
        modifiers.linear_stiffness_user_value.as_ref(),
        modifiers.rotational_stiffness_user_value.as_ref(),
    ]
    .into_iter()
    .flatten()
    .any(|quantity| !quantity.value.is_finite() || quantity.value <= 0.0);
    if optional_invalid {
        return Err(Unmet::OptionalStiffnessInvalid);
    }
    Ok((pipe_ref, scale))
}

/// The single eligibility predicate (§2.3): the mapped member and its scale
/// when every condition holds, otherwise the first unmet condition.
pub(crate) fn eligibility<'a>(
    component: &'a PreviewComponent,
    pipes: &HashMap<&str, &PreviewPipe>,
) -> Result<(&'a str, f64), Unmet> {
    if !is_rigid_family(component) {
        return Err(Unmet::NotRigidFamily);
    }
    match component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref())
    {
        None => return Err(Unmet::ConsumptionUndeclared),
        Some(GEOMETRY_ONLY) => {}
        Some(_) => return Err(Unmet::ConsumptionNotGeometryOnly),
    }
    input_conditions(component, pipes)
}

fn pipe_map(model: &PreviewModel) -> HashMap<&str, &PreviewPipe> {
    model.pipe_segments.iter().map(|pipe| (pipe.id.as_str(), pipe)).collect()
}

/// The members in a §2.3 mapping conflict, each with its refusal, in model
/// order. Every rigid-family component that maps an existing member counts,
/// whether or not it is eligible.
fn conflicts(model: &PreviewModel) -> Vec<(String, Diagnostic)> {
    let pipes = pipe_map(model);
    let curved = crate::curved_bend_realized_pipe_ids(model);
    let replaced = crate::joint::replaced_span_ids(model);
    let mut by_member: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut order: Vec<&str> = Vec::new();
    let mut found = Vec::new();
    for component in model.components.iter().filter(|component| is_rigid_family(component)) {
        let Some(pipe_ref) = mapped_pipe_ref(component).filter(|pipe_ref| pipes.contains_key(pipe_ref)) else {
            continue;
        };
        let entry = by_member.entry(pipe_ref).or_default();
        if entry.is_empty() {
            order.push(pipe_ref);
        }
        entry.push(component.id.as_str());
        if curved.contains(pipe_ref) {
            found.push((pipe_ref.to_string(), diag(
                &format!("diagnostic:component:{}:rigid-mapping-conflict:curved-bend", stable_suffix(&component.id)),
                MAPPING_CONFLICT,
                "blocking",
                format!("rigid/semi-rigid component {} maps member {pipe_ref}, which is realized as a curved-bend macro-element; a component body is a straight member, so the mapping is refused rather than solved", component.id),
                vec![component.id.clone(), pipe_ref.to_string()],
            )));
        }
        if replaced.contains(pipe_ref) {
            found.push((pipe_ref.to_string(), diag(
                &format!("diagnostic:component:{}:rigid-mapping-conflict:replaced-span", stable_suffix(&component.id)),
                MAPPING_CONFLICT,
                "blocking",
                format!("rigid/semi-rigid component {} maps member {pipe_ref}, a span replaced by an objective connector and excluded from the frame assembly; the mapping is refused rather than solved", component.id),
                vec![component.id.clone(), pipe_ref.to_string()],
            )));
        }
    }
    for pipe_ref in order {
        let components = &by_member[pipe_ref];
        if components.len() > 1 {
            let mut refs = vec![pipe_ref.to_string()];
            refs.extend(components.iter().map(|id| id.to_string()));
            found.push((pipe_ref.to_string(), diag(
                &format!("diagnostic:pipe:{}:rigid-mapping-conflict:shared", stable_suffix(pipe_ref)),
                MAPPING_CONFLICT,
                "blocking",
                format!("member {pipe_ref} is mapped by more than one rigid/semi-rigid component ({}); one member is one component body, so the mapping is refused rather than solved", components.join(", ")),
                refs,
            )));
        }
    }
    found
}

/// The members in a mapping conflict with their refusals, in model order. The
/// W-15 slice b self-weight generator blocks on a selected one (Design §3).
pub(crate) fn mapping_conflicts(model: &PreviewModel) -> Vec<(String, Diagnostic)> {
    conflicts(model)
}

/// The three mapping conflicts (§2.3), blocking by name.
pub(crate) fn append_mapping_conflicts(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    diagnostics.extend(conflicts(model).into_iter().map(|(_, finding)| finding));
}

/// One member formed with an applied scale.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AppliedScale {
    pub component_id: String,
    pub scale: f64,
}

/// Every member whose component is eligible (§2.3), keyed by member ID. A
/// member in a mapping conflict is never scaled: the conflict blocks the run,
/// and a direct build does not choose between two scales.
pub(crate) fn applied_scales(model: &PreviewModel) -> HashMap<String, AppliedScale> {
    let pipes = pipe_map(model);
    let conflicted = conflicts(model).into_iter().map(|(member, _)| member).collect::<HashSet<_>>();
    model
        .components
        .iter()
        .filter_map(|component| {
            let (pipe_ref, scale) = eligibility(component, &pipes).ok()?;
            (!conflicted.contains(pipe_ref)).then(|| {
                (pipe_ref.to_string(), AppliedScale { component_id: component.id.clone(), scale })
            })
        })
        .collect()
}

/// The input diagnostics of one rigid-family component (§2.6): one warning
/// naming the first unmet condition, and nothing for an eligible component
/// (its info and precision warning are those of the formed member,
/// [`append_formed_diagnostics`]). `not_solver_consumed` and every other
/// declared mode get none here (the unsupported-interface warning is the
/// existing one). An absent declaration is named when the component carries a
/// scale; without one, the first unmet input condition is named instead.
pub(crate) fn append_scaling_diagnostics(
    component: &PreviewComponent,
    pipes: &HashMap<&str, &PreviewPipe>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let id = stable_suffix(&component.id);
    let unmet = match eligibility(component, pipes) {
        Ok(_) => return,
        Err(Unmet::NotRigidFamily | Unmet::ConsumptionNotGeometryOnly) => return,
        Err(Unmet::ConsumptionUndeclared) => {
            let has_scale = component
                .modifiers
                .as_ref()
                .is_some_and(|modifiers| modifiers.stiffness_scaling_user_value.is_some());
            if has_scale {
                Unmet::ConsumptionUndeclared
            } else {
                match input_conditions(component, pipes) {
                    Err(unmet) => unmet,
                    // Unreachable: without a scale the inputs are incomplete.
                    Ok(_) => Unmet::ScaleMissing,
                }
            }
        }
        Err(unmet) => unmet,
    };
    let (suffix, code, message) = match unmet {
        Unmet::MappingInvalid => ("rigid-mapping", MAPPING_INVALID,
            "rigid/semi-rigid component pipe mapping must reference an existing generic frame member that terminates at the component node; the member solves as pipe, unscaled"),
        Unmet::ConsumptionUndeclared => ("rigid-consumption", CONSUMPTION_UNDECLARED,
            "rigid/semi-rigid component carries a stiffness scale but declares no mechanics_interface.solver_consumption; no mode is assumed, so the mapped member solves as pipe, unscaled; declare mechanics_geometry_only to apply the scale under DEC-045"),
        Unmet::ScaleMissing => ("rigid-stiffness", STIFFNESS_MISSING,
            "rigid/semi-rigid component requires a user-entered stiffness scale and its source reference for DEC-045 stiffness scaling; the mapped member solves as pipe, unscaled; linear and rotational stiffness are optional"),
        Unmet::ScaleInvalid | Unmet::OptionalStiffnessInvalid => ("rigid-stiffness-invalid", STIFFNESS_INVALID,
            "rigid/semi-rigid user-entered stiffness scale and any supplied linear or rotational stiffness must be finite positive values; the mapped member solves as pipe, unscaled"),
        Unmet::NotRigidFamily | Unmet::ConsumptionNotGeometryOnly => return,
    };
    diagnostics.push(diag(
        &format!("diagnostic:component:{id}:{suffix}"),
        code,
        "warning",
        message,
        vec![component.id.clone()],
    ));
}

/// W-15 slice d (Design §5): the `section_role` of every `component_body`
/// record.
pub(crate) const SECTION_ROLE: &str = "reference_pipe_section";

/// The stress kinds of the two current tables (pressure-2 and
/// preview-physics-2): every row kind whose physical dimension is stress. A
/// body member's row of one of these kinds carries the record. Pinned to the
/// tables by `w15_rigid_scaling_tests::stress_kinds_are_the_tables_stress_rows`.
pub(crate) const STRESS_KINDS: [&str; 17] = [
    "component_equal_factor_intensified_bending_stress_v1",
    "curved_bend_directional_intensified_bending_maximum_v1",
    "curved_bend_directional_intensified_bending_stress_v1",
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_normal_stress_extreme",
    "element_local_torsional_shear_stress",
    "element_local_transverse_shear_stress_maximum",
    "pipe_axial_membrane_stress_v2",
    "pipe_elastic_normal_stress_maximum_v2",
    "pipe_lame_hoop_stress_v2",
    "pipe_lame_radial_stress_v2",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
    "pipe_toroidal_membrane_hoop_stress_v1",
    "pipe_toroidal_radial_surface_stress_v1",
];

/// The record of one body member.
pub(crate) fn body_record(member: &crate::ScaledMember) -> crate::ComponentBody {
    crate::ComponentBody {
        component_ref: member.component_id.clone(),
        stiffness_scale: member.scale,
        section_role: SECTION_ROLE,
    }
}

fn record_value(member: &crate::ScaledMember) -> serde_json::Value {
    serde_json::to_value(body_record(member)).expect("a finite record serializes")
}

/// W-15 slice d (Design §5): identify every reference-section result of a
/// solved envelope. Each stress row whose entity is a body member (every case,
/// section basis and combination) and each such member's stress extremum
/// carry the record; each stress-maximum coverage entry lists the body
/// members when there are any. Nothing changes without a body, so a result
/// without an applied scale keeps its bytes.
pub(crate) fn annotate_component_bodies(
    envelope: &mut crate::MechanicsEnvelope,
    pipes: &[open_pipe_stress_straight_pipe::StraightPipeElement],
    bodies: &HashMap<String, crate::ScaledMember>,
) {
    if bodies.is_empty() {
        return;
    }
    for row in &mut envelope.results {
        if STRESS_KINDS.contains(&row.kind.as_str()) {
            if let Some(member) = bodies.get(&row.entity_ref) {
                row.component_body = Some(body_record(member));
            }
        }
    }
    let ordered = pipes
        .iter()
        .filter(|pipe| bodies.contains_key(&pipe.element_id))
        .map(|pipe| serde_json::json!(pipe.element_id))
        .collect::<Vec<_>>();
    let Some(evidence) = envelope.contract_evidence.as_mut() else { return };
    for list in ["preview_cases", "exact_cases"] {
        let Some(cases) = evidence.get_mut(list).and_then(serde_json::Value::as_array_mut) else { continue };
        for case in cases {
            if let Some(extrema) = case.get_mut("pipe_stress_extrema").and_then(serde_json::Value::as_array_mut) {
                for extremum in extrema {
                    let pipe = extremum["pipe_id"].as_str().unwrap_or_default().to_string();
                    if let Some(member) = bodies.get(&pipe) {
                        extremum["component_body"] = record_value(member);
                    }
                }
            }
            match case.get_mut("stress_maximum_coverage") {
                Some(serde_json::Value::Array(entries)) => {
                    for entry in entries {
                        entry["component_body_pipe_ids"] = serde_json::json!(ordered);
                    }
                }
                Some(entry @ serde_json::Value::Object(_)) => entry["component_body_pipe_ids"] = serde_json::json!(ordered),
                _ => {}
            }
        }
    }
}

/// The applied-scale diagnostics (§2.5, §2.6) of the members actually formed
/// with a scale, in the model's component order: the info naming the member
/// and the scale, and from [`PRECISION_WARNING_SCALE`] the precision warning.
/// Called only for a published (solved) result, so a refused or blocked model
/// never states that a scale was applied.
pub(crate) fn append_formed_diagnostics(
    model: &PreviewModel,
    formed: &HashMap<String, crate::ScaledMember>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for component in &model.components {
        let Some((pipe_ref, member)) = formed.iter().find(|(_, m)| m.component_id == component.id) else {
            continue;
        };
        let (id, scale) = (stable_suffix(&component.id), member.scale);
        diagnostics.push(diag(
            &format!("diagnostic:component:{id}:rigid-scaling-applied"),
            SCALING_APPLIED,
            "info",
            format!("rigid/semi-rigid component {} applies its user-entered stiffness scale {scale} to mapped member {pipe_ref} under DEC-045 mechanics_geometry_only: the member is solved with the scaled moduli s*E and s*G on its pipe section; its published stress rows are those of the mapped pipe section, a reference section under the body's forces, not a stress of the component body; linear and rotational stiffness values, where supplied, are recorded and not applied; no protected or default component value is supplied", component.id),
            vec![component.id.clone(), pipe_ref.to_string()],
        ));
        if scale >= PRECISION_WARNING_SCALE {
            diagnostics.push(diag(
                &format!("diagnostic:component:{id}:rigid-scale-precision"),
                SCALE_PRECISION,
                "warning",
                format!("rigid/semi-rigid component {} scales member {pipe_ref} by {scale}, at least {PRECISION_WARNING_SCALE:e}: the scaled member's forces may carry reduced precision; this is a numerical heuristic measured on the W-15 rigid-link benchmark, not an accuracy bound or an engineering limit, and the pivot, formation and recovery screens remain the qualification", component.id),
                vec![component.id.clone(), pipe_ref.to_string()],
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// W-15 slice c (DEL-03-05 Design `eccentric-reducer-offset.md` §3.2-§3.6):
// a reducer's declared eccentric offset, checked against the node geometry,
// which governs. The offset never places, moves or re-forms anything.

pub(crate) const ECCENTRIC_INVALID: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_INVALID";
pub(crate) const ECCENTRIC_BELOW_PRECISION: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_BELOW_PRECISION";
pub(crate) const ECCENTRIC_NOT_TRANSVERSE: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_NOT_TRANSVERSE";
pub(crate) const ECCENTRIC_MISMATCH: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_MISMATCH";
pub(crate) const ECCENTRIC_REALIZED: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED";
pub(crate) const ECCENTRIC_AXIS_ALLOWANCE_USED: &str = "RIGID_COMPONENT_ECCENTRIC_OFFSET_AXIS_ALLOWANCE_USED";

/// §3.3 (HELP_HUMAN, 2026-10-26, under the owner's delegation): the declared
/// coordinate-rounding allowance q. A fixed constant of the product, not a
/// user setting: coordinates and offset components are taken to be entered
/// rounded to the millimetre, so each lies within q/2 of its true value.
pub(crate) const COORDINATE_ROUNDING_ALLOWANCE_M: f64 = 1.0e-3;
/// §3.2: ρ_base, the base transversality ratio.
pub(crate) const TRANSVERSE_RATIO_BASE: f64 = 1.0e-3;
/// §3.3: θ_base, the base agreement angle of the two ends' run directions.
pub(crate) const AXIS_ANGLE_BASE_RAD: f64 = 1.0e-3;

type V3 = [f64; 3];

fn v_sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn v_add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn v_scale(k: f64, a: V3) -> V3 {
    [k * a[0], k * a[1], k * a[2]]
}
fn v_dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn v_cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn v_norm(a: V3) -> f64 {
    v_dot(a, a).sqrt()
}
fn v_neg(a: V3) -> V3 {
    [-a[0], -a[1], -a[2]]
}

/// The component's declared offset, when present.
pub(crate) fn eccentric_offset(component: &PreviewComponent) -> Option<&crate::VectorQuantity> {
    component.geometry.as_ref().and_then(|geometry| geometry.eccentric_offset.as_ref())
}

/// R-c1 (§3.6; the one place the convention lives), owner ruling of 2026-10-26,
/// "R-c1 A, 0.8.0 a2": the origin end N is the component's node and the far end
/// F is the mapped member's other end, whichever way the member runs. `None`
/// when the member does not end at the component's node.
pub(crate) fn eccentric_offset_ends<'a>(component: &'a PreviewComponent, member: &'a PreviewPipe) -> Option<(&'a str, &'a str)> {
    if member.from == component.node {
        Some((member.from.as_str(), member.to.as_str()))
    } else if member.to == component.node {
        Some((member.to.as_str(), member.from.as_str()))
    } else {
        None
    }
}

/// One adjoining member's evidence at an end (§3.3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Evidence {
    /// A run direction (undirected unit vector) with its uncertainty β (rad).
    Run { direction: V3, beta: f64 },
    /// Not run evidence: another reducer's body, an unrealized bend member, an
    /// expansion-joint member or a connector span.
    Unsupported(&'static str),
    /// Run-class, but no direction can be resolved within the coordinate-rounding
    /// allowance; the reason names why (a short span, a reachable semicircle, an
    /// ill-conditioned bend plane).
    Unresolvable(&'static str),
}

/// An end's status (§3.3).
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EndStatus {
    None,
    Direction { direction: V3, beta: f64 },
    Ambiguous(Vec<String>),
    Unsupported { member: String, reason: &'static str },
    Unresolvable { member: String, reason: &'static str },
}

/// β of a straight member of length L: asin(√3·q/L), or `None` when √3·q ≥ L.
pub(crate) fn straight_beta(length: f64) -> Option<f64> {
    let ratio = 3f64.sqrt() * COORDINATE_ROUNDING_ALLOWANCE_M / length;
    (length.is_finite() && ratio < 1.0).then(|| ratio.asin())
}

/// The named reasons a run-class member gives no direction (§3.3).
pub(crate) const SHORT_SPAN: &str = "a member shorter than the resolvable length sqrt(3)*q";
pub(crate) const NEAR_SEMICIRCLE: &str = "a bend whose coordinate rounding admits a semicircular arc (c + sqrt(3)*q >= 2R), where its end tangent is unresolved";
pub(crate) const ILL_CONDITIONED_PLANE: &str = "a bend whose plane reference is too nearly parallel to its chord for the coordinate rounding to resolve the bend plane";
pub(crate) const TOTAL_UNCERTAINTY: &str = "a bend whose total end-tangent uncertainty within the coordinate rounding reaches or exceeds pi/2";

/// The tangent of a realized arc at `at` and its finite uncertainty bound β_arc
/// (§3.3, revised after Codex numerical scrutiny r1). The tangent is
/// cos h·ĉ + s·sin h·n̂ with chord ĉ, in-plane normal n̂ (the `y_reference` less its
/// chord part), half-angle h = asin(c/(2R)), s = +1 at the arc's `from` node and
/// −1 at its `to` node (the arc bows toward +n̂).
///
/// Each endpoint lies within (√3/2)·q of its true position, so the true chord
/// differs from the entered one by at most ε = √3·q. Over that whole region:
/// - chord direction: angle ≤ α_c = asin(ε/c), so ‖ĉ′ − ĉ‖ ≤ d_c = 2·sin(α_c/2);
/// - chord length within c ± ε: the half-angle changes by at most
///   Δh = max(asin((c + ε)/(2R)) − h, h − asin(max(0, c − ε)/(2R))), defined
///   only while c + ε < 2R (otherwise a semicircle is admissible: refused);
/// - plane normal: with u = y/‖y‖ and sin φ = ‖u⊥‖, ‖u⊥′ − u⊥‖ ≤
///   d_n = d_c + (1 − cos α_c) + sin φ·sin α_c, so the normal turns by at most
///   α_n = asin(d_n / sin φ), defined only while d_n < sin φ (otherwise the bend
///   plane is unresolved: refused);
/// - the frame (ĉ, n̂) → (ĉ′, n̂′) at a fixed half-angle moves the tangent by at
///   most 2·asin(√(sin²(α_c/2) + sin²(α_n/2))).
///
/// β_arc = Δh + 2·asin(√(sin²(α_c/2) + sin²(α_n/2))), and it must be < π/2.
pub(crate) fn arc_evidence(from: V3, to: V3, at_from: bool, radius: f64, y_reference: V3) -> Evidence {
    let chord = v_sub(to, from);
    let c = v_norm(chord);
    let eps = 3f64.sqrt() * COORDINATE_ROUNDING_ALLOWANCE_M;
    if !(c.is_finite() && c > 0.0 && radius.is_finite() && radius > 0.0 && 2.0 * radius > c) {
        return Evidence::Unresolvable(NEAR_SEMICIRCLE);
    }
    if !(eps < c) {
        return Evidence::Unresolvable(SHORT_SPAN);
    }
    if !(c + eps < 2.0 * radius) {
        return Evidence::Unresolvable(NEAR_SEMICIRCLE);
    }
    let chat = v_scale(1.0 / c, chord);
    let y_norm = v_norm(y_reference);
    let axial = v_dot(y_reference, chat);
    let y_perp = v_sub(y_reference, v_scale(axial, chat));
    let y_perp_norm = v_norm(y_perp);
    let sin_phi = y_perp_norm / y_norm;
    let alpha_c = (eps / c).asin();
    let d_c = 2.0 * (0.5 * alpha_c).sin();
    let d_n = d_c + (1.0 - alpha_c.cos()) + sin_phi * alpha_c.sin();
    if !(sin_phi.is_finite() && d_n < sin_phi) {
        return Evidence::Unresolvable(ILL_CONDITIONED_PLANE);
    }
    let nhat = v_scale(1.0 / y_perp_norm, y_perp);
    let half = (c / (2.0 * radius)).asin();
    let side = if at_from { 1.0 } else { -1.0 };
    let direction = v_add(v_scale(half.cos(), chat), v_scale(side * half.sin(), nhat));
    let delta_h = (((c + eps) / (2.0 * radius)).asin() - half).max(half - ((c - eps).max(0.0) / (2.0 * radius)).asin());
    let alpha_n = (d_n / sin_phi).asin();
    let frame = 2.0 * ((0.5 * alpha_c).sin().powi(2) + (0.5 * alpha_n).sin().powi(2)).sqrt().min(1.0).asin();
    let beta = delta_h + frame;
    if !(beta.is_finite() && beta < std::f64::consts::FRAC_PI_2) {
        return Evidence::Unresolvable(TOTAL_UNCERTAINTY);
    }
    Evidence::Run { direction, beta }
}

/// The run axis decided from the two ends (§3.3): one governing undirected
/// direction u with its β_run, or, on an exact tie of β after the agreement
/// check, both ends' own (direction, β), each judged separately (J3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RunAxis {
    One(V3, f64),
    Tied((V3, f64), (V3, f64)),
}

/// The two ends' statuses decided into a run axis (§3.3), or the refusal reason
/// and its detail.
pub(crate) fn decide_axis(n: &EndStatus, f: &EndStatus) -> Result<RunAxis, (&'static str, String)> {
    for (end, status) in [("N", n), ("F", f)] {
        if let EndStatus::Ambiguous(candidates) = status {
            return Err(("ambiguous_run", format!("end {end} adjoins more than one member ({})", candidates.join(", "))));
        }
    }
    for (end, status) in [("N", n), ("F", f)] {
        if let EndStatus::Unsupported { member, reason } = status {
            return Err(("unsupported_run_evidence", format!("end {end} adjoins {member}, {reason}, which is not run evidence")));
        }
    }
    for (end, status) in [("N", n), ("F", f)] {
        if let EndStatus::Unresolvable { member, reason } = status {
            return Err(("unresolvable_run_evidence", format!("end {end} adjoins {member}, {reason}, so no run direction is resolved within the coordinate-rounding allowance")));
        }
    }
    match (n, f) {
        (EndStatus::Direction { direction: dn, beta: bn }, EndStatus::Direction { direction: df, beta: bf }) => {
            let along = v_dot(*dn, *df);
            let theta = v_norm(v_cross(*dn, *df)).atan2(along.abs());
            let bound = AXIS_ANGLE_BASE_RAD + bn + bf;
            if !(theta <= bound) {
                return Err(("run_directions_disagree", format!("the run directions at the two ends are {theta:e} rad apart, more than the bound {bound:e} rad")));
            }
            // J2 (HELP_HUMAN, 2026-10-26, under the owner's delegation): the
            // better-resolved end governs, so weak evidence cannot tilt the axis.
            // J3 (same): on an exact tie no consensus is formed; each end's own
            // (direction, β) is judged (`judge_run`). Both are symmetric in the
            // two ends.
            if bn < bf {
                return Ok(RunAxis::One(*dn, *bn));
            }
            if bf < bn {
                return Ok(RunAxis::One(*df, *bf));
            }
            Ok(RunAxis::Tied((*dn, *bn), (*df, *bf)))
        }
        (EndStatus::Direction { direction, beta }, EndStatus::None) | (EndStatus::None, EndStatus::Direction { direction, beta }) => Ok(RunAxis::One(*direction, *beta)),
        _ => Err(("standalone_body", "neither end adjoins a member that gives a run direction".to_string())),
    }
}

/// The measured quantities of an accepted or refused offset, in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct OffsetMeasure {
    pub axis: V3,
    pub axial_length: f64,
    pub transverse: V3,
    pub offset_axial: f64,
    pub allowance: f64,
    pub residual: f64,
    pub beta_run: f64,
    /// The placement's run-axis allowance ‖D‖·sin β_run (rule B, §3.3): the
    /// residual is accepted up to τ plus this.
    pub axis_allowance: f64,
}

/// The outcome of one reducer's check.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OffsetOutcome {
    Realized(OffsetMeasure),
    Invalid(&'static str, String),
    BelowPrecision(OffsetMeasure),
    NotTransverse(OffsetMeasure),
    Mismatch(OffsetMeasure),
}

/// §3.2 steps 2-4 for given ends, axis evidence, offset and tolerance. Pure:
/// swapping N and F together with negating e (a sign-only convention change)
/// gives bit-identical measures.
pub(crate) fn judge_offset(x_n: V3, x_f: V3, axis: (V3, f64), offset: V3, tolerance: f64) -> OffsetOutcome {
    let measure = offset_measure(x_n, x_f, axis, offset);
    let OffsetMeasure { axial_length, offset_axial, allowance, residual, .. } = measure;
    let e_norm = v_norm(offset);
    if !(axial_length > 0.0) {
        return OffsetOutcome::Invalid("no_axial_length", no_axial_length_detail(axial_length));
    }
    let zero = offset.iter().all(|v| *v == 0.0);
    if !zero {
        if !(allowance < e_norm) {
            return OffsetOutcome::BelowPrecision(measure);
        }
        if !(offset_axial <= allowance) {
            return OffsetOutcome::NotTransverse(measure);
        }
    }
    // Rule B (HELP_HUMAN, 2026-10-26, under the owner's delegation): the
    // placement carries the run axis's own uncertainty, as the transversality
    // allowance does. A residual above τ but within τ + ‖D‖·sin β_run is
    // realized with the `..._AXIS_ALLOWANCE_USED` warning.
    if !(residual <= tolerance + measure.axis_allowance) {
        return OffsetOutcome::Mismatch(measure);
    }
    OffsetOutcome::Realized(measure)
}

fn no_axial_length_detail(axial_length: f64) -> String {
    format!("the body has no length along the run axis (D·t = {axial_length:e} m)")
}

/// The measured quantities for one (direction, β) against the body D = x_f − x_n:
/// the axis t oriented by D, and the offset's quantities along and across it.
fn offset_measure(x_n: V3, x_f: V3, axis: (V3, f64), offset: V3) -> OffsetMeasure {
    let d = v_sub(x_f, x_n);
    let (u, beta_run) = axis;
    let t = if v_dot(u, d) < 0.0 { v_neg(u) } else { u };
    let axial_length = v_dot(d, t);
    let transverse = v_sub(d, v_scale(axial_length, t));
    let e_norm = v_norm(offset);
    let offset_axial = v_dot(offset, t).abs();
    let allowance = TRANSVERSE_RATIO_BASE * e_norm + 0.5 * 3f64.sqrt() * COORDINATE_ROUNDING_ALLOWANCE_M + e_norm * beta_run.sin();
    let residual = v_norm(v_sub(transverse, offset));
    let axis_allowance = v_norm(d) * beta_run.sin();
    OffsetMeasure { axis: t, axial_length, transverse, offset_axial, allowance, residual, beta_run, axis_allowance }
}

/// The offset judged against the decided run axis (§3.3). On an exact tie
/// (J3, HELP_HUMAN, 2026-10-26, under the owner's delegation) each end's own
/// (direction, β) is judged by `judge_offset`; the reducer is refused if either
/// judgment refuses, and the reported judgment is chosen by this precedence:
/// a refusal before an acceptance; then the larger residual ‖D⊥ − e‖; then the
/// earlier refusal in `judge_offset`'s order (no axial length, below precision,
/// not transverse, mismatch); then the larger axial offset share, axial length
/// and allowance; and last the axis and transverse vectors each multiplied by
/// the sign of D's first nonzero component, compared component by component.
/// Every key is unchanged when the ends' names are swapped (D, t, D⊥ and, under
/// the other convention, e negate exactly), so the outcome does not depend on
/// which end is N. Signed zeros are the one exception IEEE negation leaves (a
/// D·t of −0 at one naming is +0 at the other), so on this path every key and
/// every reported quantity is canonicalised to +0 (x + 0.0). Keys that compare
/// equal throughout then give literally identical outcomes.
pub(crate) fn judge_run(x_n: V3, x_f: V3, axis: RunAxis, offset: V3, tolerance: f64) -> OffsetOutcome {
    match axis {
        RunAxis::One(u, beta) => judge_offset(x_n, x_f, (u, beta), offset, tolerance),
        RunAxis::Tied(end_n, end_f) => {
            let d = v_sub(x_f, x_n);
            let sign = d.iter().find(|v| **v != 0.0).map_or(1.0, |v| v.signum());
            let key = |axis: (V3, f64)| {
                let m = offset_measure(x_n, x_f, axis, offset);
                let outcome = judge_offset(x_n, x_f, axis, offset, tolerance);
                let rank = match outcome {
                    OffsetOutcome::Realized(_) => 0.0,
                    OffsetOutcome::Mismatch(_) => 1.0,
                    OffsetOutcome::NotTransverse(_) => 2.0,
                    OffsetOutcome::BelowPrecision(_) => 3.0,
                    OffsetOutcome::Invalid(..) => 4.0,
                };
                let refused = if rank > 0.0 { 1.0 } else { 0.0 };
                let (t, p) = (v_scale(sign, m.axis), v_scale(sign, m.transverse));
                let k = [refused, m.residual, rank, m.offset_axial, m.axial_length, m.allowance, t[0], t[1], t[2], p[0], p[1], p[2]].map(|v| v + 0.0);
                (k, canonical_outcome(outcome, &m))
            };
            let ((k_n, o_n), (k_f, o_f)) = (key(end_n), key(end_f));
            let later = k_n.iter().zip(&k_f).map(|(a, b)| b.total_cmp(a)).find(|o| o.is_ne());
            if later == Some(std::cmp::Ordering::Greater) { o_f } else { o_n }
        }
    }
}

/// The tie path's outcome with every signed zero in its measure, and in the
/// `no_axial_length` detail, made +0 (J3 naming invariance).
fn canonical_outcome(outcome: OffsetOutcome, measure: &OffsetMeasure) -> OffsetOutcome {
    let z = |v: f64| v + 0.0;
    let m = OffsetMeasure {
        axis: measure.axis.map(z),
        axial_length: z(measure.axial_length),
        transverse: measure.transverse.map(z),
        offset_axial: z(measure.offset_axial),
        allowance: z(measure.allowance),
        residual: z(measure.residual),
        beta_run: z(measure.beta_run),
        axis_allowance: z(measure.axis_allowance),
    };
    match outcome {
        OffsetOutcome::Realized(_) => OffsetOutcome::Realized(m),
        OffsetOutcome::BelowPrecision(_) => OffsetOutcome::BelowPrecision(m),
        OffsetOutcome::NotTransverse(_) => OffsetOutcome::NotTransverse(m),
        OffsetOutcome::Mismatch(_) => OffsetOutcome::Mismatch(m),
        OffsetOutcome::Invalid("no_axial_length", _) => OffsetOutcome::Invalid("no_axial_length", no_axial_length_detail(m.axial_length)),
        invalid @ OffsetOutcome::Invalid(..) => invalid,
    }
}

/// How a member is mapped, for run evidence (§3.3).
#[derive(Clone, Copy)]
enum Mapping<'a> {
    ReducerBody,
    ConcentricBody,
    Bend(&'a PreviewComponent),
    ExpansionJoint,
}

struct Topology<'a> {
    positions: HashMap<&'a str, V3>,
    pipes: HashMap<&'a str, &'a PreviewPipe>,
    at_node: HashMap<&'a str, Vec<&'a PreviewPipe>>,
    mapping: HashMap<&'a str, Mapping<'a>>,
    realized: HashSet<&'a str>,
    replaced: HashSet<String>,
}

impl<'a> Topology<'a> {
    fn of(model: &'a PreviewModel) -> Self {
        let positions = model.nodes.iter().map(|node| (node.id.as_str(), [node.position.x, node.position.y, node.position.z])).collect();
        let pipes = pipe_map(model);
        let mut at_node: HashMap<&str, Vec<&PreviewPipe>> = HashMap::new();
        for pipe in &model.pipe_segments {
            at_node.entry(pipe.from.as_str()).or_default().push(pipe);
            if pipe.to != pipe.from {
                at_node.entry(pipe.to.as_str()).or_default().push(pipe);
            }
        }
        let mut mapping: HashMap<&str, Mapping> = HashMap::new();
        for component in &model.components {
            let Some(geometry) = component.geometry.as_ref() else { continue };
            let refs = [
                (geometry.rigid_pipe_ref.as_deref(), if component.kind == "reducer" { Mapping::ReducerBody } else { Mapping::ConcentricBody }),
                (geometry.bend_pipe_ref.as_deref(), Mapping::Bend(component)),
                (geometry.expansion_joint_pipe_ref.as_deref(), Mapping::ExpansionJoint),
            ];
            for (pipe_ref, kind) in refs {
                let Some(pipe_ref) = pipe_ref.filter(|value| !value.trim().is_empty()) else { continue };
                let applies = match kind {
                    Mapping::ReducerBody | Mapping::ConcentricBody => is_rigid_family(component),
                    Mapping::Bend(_) => crate::is_bend_component(component),
                    Mapping::ExpansionJoint => component.kind == "expansion_joint",
                };
                if !applies {
                    continue;
                }
                // A reducer body outranks every other mapping of the member.
                let entry = mapping.entry(pipe_ref).or_insert(kind);
                if matches!(kind, Mapping::ReducerBody) {
                    *entry = kind;
                }
            }
        }
        Topology { positions, pipes, at_node, mapping, realized: crate::curved_bend_realized_pipe_ids(model), replaced: crate::joint::replaced_span_ids(model) }
    }

    fn evidence(&self, member: &PreviewPipe, at: &str) -> Evidence {
        if self.replaced.contains(&member.id) {
            return Evidence::Unsupported("a span replaced by an objective connector");
        }
        let (Some(&from), Some(&to)) = (self.positions.get(member.from.as_str()), self.positions.get(member.to.as_str())) else {
            return Evidence::Unresolvable("a member whose end nodes are not both present");
        };
        match self.mapping.get(member.id.as_str()) {
            Some(Mapping::ReducerBody) => Evidence::Unsupported("the body of another reducer"),
            Some(Mapping::ExpansionJoint) => Evidence::Unsupported("an expansion-joint member"),
            Some(Mapping::Bend(component)) => {
                let radius = component.geometry.as_ref().and_then(|g| g.bend_radius.as_ref()).map(|q| q.value);
                match (self.realized.contains(member.id.as_str()), radius, member.y_reference) {
                    (true, Some(radius), Some(y)) => arc_evidence(from, to, member.from == at, radius, [y.x, y.y, y.z]),
                    _ => Evidence::Unsupported("a member mapped by a bend component and not realized as an arc"),
                }
            }
            Some(Mapping::ConcentricBody) | None => {
                let chord = v_sub(to, from);
                let length = v_norm(chord);
                match straight_beta(length) {
                    Some(beta) if length > 0.0 => Evidence::Run { direction: v_scale(1.0 / length, chord), beta },
                    _ => Evidence::Unresolvable(SHORT_SPAN),
                }
            }
        }
    }

    fn end_status(&self, body: &PreviewPipe, at: &str) -> EndStatus {
        let adjoining: Vec<&PreviewPipe> = self
            .at_node
            .get(at)
            .map(|pipes| pipes.iter().copied().filter(|pipe| pipe.id != body.id).collect())
            .unwrap_or_default();
        match adjoining.as_slice() {
            [] => EndStatus::None,
            [member] => match self.evidence(member, at) {
                Evidence::Run { direction, beta } => EndStatus::Direction { direction, beta },
                Evidence::Unsupported(reason) => EndStatus::Unsupported { member: member.id.clone(), reason },
                Evidence::Unresolvable(reason) => EndStatus::Unresolvable { member: member.id.clone(), reason },
            },
            many => EndStatus::Ambiguous(many.iter().map(|pipe| pipe.id.clone()).collect()),
        }
    }
}

/// Whether the reducer's offset is read: present, and the mode is not the
/// honoured `not_solver_consumed` (§3.2, P6).
fn offset_is_read(component: &PreviewComponent) -> bool {
    eccentric_offset(component).is_some()
        && component.mechanics_interface.as_ref().and_then(|i| i.solver_consumption.as_deref()) != Some("not_solver_consumed")
}

/// Every reducer whose offset is read, with its outcome, in model order.
fn judge_all<'a>(model: &'a PreviewModel, tolerance: f64) -> Vec<(&'a PreviewComponent, OffsetOutcome)> {
    let topology = Topology::of(model);
    let mut out = Vec::new();
    for component in model.components.iter().filter(|c| c.kind == "reducer" && offset_is_read(c)) {
        let offset = eccentric_offset(component).expect("filtered on presence");
        let e = [offset.x, offset.y, offset.z];
        let outcome = if !e.iter().all(|v| v.is_finite()) {
            OffsetOutcome::Invalid("not_finite", "a component of the offset is not finite after conversion".to_string())
        } else {
            match mapped_pipe_ref(component).and_then(|r| topology.pipes.get(r).copied()).and_then(|member| {
                let (n, f) = eccentric_offset_ends(component, member)?;
                Some((member, n, f, *topology.positions.get(n)?, *topology.positions.get(f)?))
            }) {
                None => OffsetOutcome::Invalid("mapping_invalid", "the offset needs a mapped body member that exists and ends at the component's node".to_string()),
                Some((member, n, f, x_n, x_f)) => match decide_axis(&topology.end_status(member, n), &topology.end_status(member, f)) {
                    Err((reason, detail)) => OffsetOutcome::Invalid(reason, detail),
                    Ok(axis) => judge_run(x_n, x_f, axis, e, tolerance),
                },
            }
        };
        out.push((component, outcome));
    }
    out
}

fn vec_text(v: V3) -> String {
    format!("({:e}, {:e}, {:e}) m", v[0], v[1], v[2])
}

/// §3.2-§3.4: the refusals of declared offsets. The first pass is
/// allocation-free and returns when no component carries the slot; a
/// non-reducer carrying it is refused before any reducer processing.
pub(crate) fn append_eccentric_offset_refusals(model: &PreviewModel, tolerance: f64, diagnostics: &mut Vec<Diagnostic>) {
    if !model.components.iter().any(|component| eccentric_offset(component).is_some()) {
        return;
    }
    for component in model.components.iter().filter(|c| eccentric_offset(c).is_some() && c.kind != "reducer") {
        diagnostics.push(diag(
            &format!("diagnostic:component:{}:eccentric-offset-invalid", stable_suffix(&component.id)),
            ECCENTRIC_INVALID,
            "blocking",
            format!("component {} (kind {}) carries geometry.eccentric_offset, a reducer slot (reason not_reducer); the offset is refused rather than left unread", component.id, component.kind),
            vec![component.id.clone()],
        ));
    }
    for (component, outcome) in judge_all(model, tolerance) {
        let id = stable_suffix(&component.id);
        let refs = vec![component.id.clone()];
        let (suffix, code, message) = match outcome {
            OffsetOutcome::Realized(_) => continue,
            OffsetOutcome::Invalid(reason, detail) => ("eccentric-offset-invalid", ECCENTRIC_INVALID,
                format!("reducer {} eccentric offset cannot be checked (reason {reason}): {detail}; the node geometry governs, and the declared offset is refused rather than left unchecked", component.id)),
            OffsetOutcome::BelowPrecision(m) => ("eccentric-offset-below-precision", ECCENTRIC_BELOW_PRECISION,
                format!("reducer {} declares a nonzero eccentric offset of magnitude {:e} m, not above its transversality allowance {:e} m (coordinate-rounding allowance q = {COORDINATE_ROUNDING_ALLOWANCE_M:e} m, run-axis uncertainty {:e} rad); declare a zero offset or correct the geometry", component.id, v_norm_of(component), m.allowance, m.beta_run)),
            OffsetOutcome::NotTransverse(m) => ("eccentric-offset-not-transverse", ECCENTRIC_NOT_TRANSVERSE,
                format!("reducer {} eccentric offset has an axial component {:e} m along the run axis {}, above its allowance {:e} m; an eccentric offset is transverse to the run", component.id, m.offset_axial, vec_text(m.axis), m.allowance)),
            OffsetOutcome::Mismatch(m) => ("eccentric-offset-mismatch", ECCENTRIC_MISMATCH,
                format!("reducer {} declares eccentric offset {} but its end nodes are separated transversely by {}: residual {:e} m exceeds its placement allowance {:e} m, the node-proximity tolerance {tolerance:e} m plus the run-axis allowance {:e} m (body length times the sine of the run-axis uncertainty {:e} rad); the node geometry governs, so correct the nodes or the declared offset", component.id, vec_text(offset_v3(component)), vec_text(m.transverse), m.residual, tolerance + m.axis_allowance, m.axis_allowance, m.beta_run)),
        };
        diagnostics.push(diag(&format!("diagnostic:component:{id}:{suffix}"), code, "blocking", message, refs));
    }
}

fn offset_v3(component: &PreviewComponent) -> V3 {
    eccentric_offset(component).map(|o| [o.x, o.y, o.z]).unwrap_or([0.0; 3])
}

fn v_norm_of(component: &PreviewComponent) -> f64 {
    v_norm(offset_v3(component))
}

/// §3.4: the realized offset of each checked reducer, on a published result
/// only (called past the last blocked return).
pub(crate) fn append_eccentric_offset_realized(model: &PreviewModel, tolerance: f64, diagnostics: &mut Vec<Diagnostic>) {
    if !model.components.iter().any(|component| eccentric_offset(component).is_some()) {
        return;
    }
    for (component, outcome) in judge_all(model, tolerance) {
        let OffsetOutcome::Realized(m) = outcome else { continue };
        let length = (m.axial_length * m.axial_length + v_dot(m.transverse, m.transverse)).sqrt();
        let inclination = v_norm(m.transverse).atan2(m.axial_length);
        diagnostics.push(diag(
            &format!("diagnostic:component:{}:eccentric-offset-realized", stable_suffix(&component.id)),
            ECCENTRIC_REALIZED,
            "info",
            format!("reducer {} declares eccentric offset {} and its end nodes realize a transverse separation {}, which governs the solve; the eccentricity is carried by the body member between the two centerlines (length {length:e} m, inclined {inclination:e} rad to the run axis); no rigid offset link is added; the member's stress rows are those of the mapped pipe section along the chord, a reference section, whether or not a stiffness scale is applied", component.id, vec_text(offset_v3(component)), vec_text(m.transverse)),
            vec![component.id.clone()],
        ));
        // Rule B's warning band (§3.3): accepted only within the run-axis allowance.
        if !(m.residual <= tolerance) {
            diagnostics.push(diag(
                &format!("diagnostic:component:{}:eccentric-offset-axis-allowance-used", stable_suffix(&component.id)),
                ECCENTRIC_AXIS_ALLOWANCE_USED,
                "warning",
                format!("reducer {} declares eccentric offset {} and its end nodes realize a transverse separation {}: residual {:e} m exceeds the node-proximity tolerance {tolerance:e} m and is accepted only within the run-axis allowance {:e} m (body length times the sine of the run-axis uncertainty {:e} rad from the coordinate-rounding allowance); the node geometry was used for the solve, not the declared offset", component.id, vec_text(offset_v3(component)), vec_text(m.transverse), m.residual, m.axis_allowance, m.beta_run),
                vec![component.id.clone()],
            ));
        }
    }
}
