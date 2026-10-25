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
