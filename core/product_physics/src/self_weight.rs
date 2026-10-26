//! Pure selected-pipe mass operation drafts. The caller retains and hashes the
//! original document and supplies the operation envelope before atomic apply.
//!
//! W-15 slice b (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §3):
//! a selected member that a rigid-family component maps is given the
//! component's own weight instead of its distributed self-weight, as nodal
//! primitives at the component's node (the force W·ĝ and the moments r × F,
//! r the centre-of-gravity vector in global X/Y/Z from that node, the owner's
//! R10 convention of 2026-10-24).
use super::{
    compute_pipe_mass_per_length, normalize_quantity, normalize_vector_quantity,
    resolve_shared_sections, Diagnostic, PreviewComponent, PreviewModel, Quantity,
};
use crate::rigid_component::{self, is_rigid_family, mapped_pipe_ref, GEOMETRY_ONLY};
use open_pipe_stress_units::Dimension;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelfWeightGravity {
    pub value: f64,
    pub unit: String,
    pub axis: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelfWeightRequest {
    pub case_id: String,
    pub label: String,
    pub pipe_refs: Vec<String>,
    pub gravity: SelfWeightGravity,
    pub provenance: String,
    pub source_model_hash: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct SelfWeightOperationPlan {
    pub source_model_hash: String,
    pub changes: Vec<SelfWeightOperationDraft>,
    pub source_evidence: Vec<Value>,
    pub scope_label: String,
    /// W-15 slice b: one warning per selected member whose mapping rigid-family
    /// component fails a weight condition; that member's distributed
    /// self-weight is generated in its place. Absent when empty.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Diagnostic>,
}
#[derive(Debug, Clone, Serialize)]
pub struct SelfWeightOperationDraft {
    pub object_type: String,
    pub target_ref: String,
    pub operation_kind: String,
    pub change_kind: String,
    pub field_label: String,
    pub field_path: String,
    pub before: String,
    pub after: String,
    pub unit: String,
    pub dimension: String,
    pub source_note: String,
}
fn invalid(d: &mut Vec<Diagnostic>, field: &str, message: &str) {
    d.push(Diagnostic {
        id: format!("diagnostic:self-weight:{field}"),
        code: "SELF_WEIGHT_INPUT_INVALID".into(),
        severity: "blocking".into(),
        message: message.into(),
        source: Some("self_weight".into()),
        affected_refs: vec![field.into()],
    });
}
fn quantity(q: &Quantity) -> Value {
    json!({"value":q.value,"unit":q.unit})
}
fn optional(q: &Option<Quantity>) -> Value {
    q.as_ref().map(quantity).unwrap_or(Value::Null)
}
fn normalize(
    q: &mut Quantity,
    dim: Dimension,
    positive: bool,
    field: &str,
    d: &mut Vec<Diagnostic>,
) {
    if !q.value.is_finite() {
        invalid(d, field, "quantity must be finite");
        return;
    }
    normalize_quantity(
        q,
        dim,
        &format!("diagnostic:self-weight:{field}"),
        vec![field.into()],
        d,
    );
    if !q.value.is_finite() || (positive && q.value <= 0.0) || (!positive && q.value < 0.0) {
        invalid(d, field, "quantity has invalid sign or normalized value");
    }
}
fn provenance_present(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::String(s) => !s.trim().is_empty(),
        Value::Object(m) => !m.is_empty(),
        Value::Array(a) => !a.is_empty(),
        _ => false,
    }
}
fn draft(
    r: &SelfWeightRequest,
    kind: &str,
    path: &str,
    after: Value,
    unit: &str,
    dimension: &str,
    evidence: &Value,
) -> SelfWeightOperationDraft {
    SelfWeightOperationDraft {
        object_type: "Load".into(),
        target_ref: r.case_id.clone(),
        operation_kind: "create".into(),
        change_kind: kind.into(),
        field_label: kind.into(),
        field_path: path.into(),
        before: "not_present".into(),
        after: after.to_string(),
        unit: unit.into(),
        dimension: dimension.into(),
        source_note: evidence.to_string(),
    }
}
/// Generate a complete deterministic draft or blocking diagnostics, without
/// mutating the typed view or claiming to authenticate its canonical hash.
pub fn generate_self_weight_operations(
    model: &PreviewModel,
    request: &SelfWeightRequest,
) -> Result<SelfWeightOperationPlan, Vec<Diagnostic>> {
    generate_operations(model, request, false)
}

fn generate_operations(
    model: &PreviewModel,
    request: &SelfWeightRequest,
    inspection: bool,
) -> Result<SelfWeightOperationPlan, Vec<Diagnostic>> {
    let mut d = Vec::new();
    for (name, value) in [
        ("case_id", &request.case_id),
        ("label", &request.label),
        ("provenance", &request.provenance),
    ] {
        if value.trim().is_empty() {
            invalid(&mut d, name, "explicit nonblank value required");
        }
    }
    if !inspection
        && !request
            .source_model_hash
            .strip_prefix("sha256:")
            .is_some_and(|digest| {
                digest.len() == 64
                    && digest
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            })
    {
        invalid(
            &mut d,
            "source_model_hash",
            "canonical sha256: lowercase SHA256 digest required",
        );
    }
    if !matches!(
        request.gravity.axis.as_str(),
        "global_x" | "global_y" | "global_z"
    ) {
        invalid(&mut d, "gravity.axis", "explicit global axis required");
    }
    let mut gravity = Quantity {
        value: request.gravity.value,
        unit: request.gravity.unit.clone(),
    };
    if !gravity.value.is_finite() || gravity.value == 0.0 {
        invalid(
            &mut d,
            "gravity",
            "finite nonzero signed acceleration required",
        );
    } else {
        normalize_quantity(
            &mut gravity,
            Dimension::Acceleration,
            "diagnostic:self-weight:gravity",
            vec!["gravity".into()],
            &mut d,
        );
        if !gravity.value.is_finite() || gravity.value == 0.0 {
            invalid(
                &mut d,
                "gravity",
                "normalized acceleration must be finite and nonzero",
            );
        }
    }
    let mut used: HashSet<String> = model
        .load_cases
        .iter()
        .map(|c| c.id.clone())
        .chain(
            model
                .load_cases
                .iter()
                .flat_map(|c| c.primitive_loads.iter().map(|l| l.id.clone())),
        )
        .collect();
    if !used.insert(request.case_id.clone()) {
        invalid(
            &mut d,
            "case_id",
            "case identity collides with existing case or primitive load",
        );
    }
    if request.pipe_refs.is_empty() {
        invalid(&mut d, "pipe_refs", "select at least one pipe");
    }
    let mut selected: HashSet<&String> = HashSet::new();
    let mut subset = model.clone();
    subset.pipe_segments.clear();
    for id in &request.pipe_refs {
        if id.trim().is_empty() || !selected.insert(id) {
            invalid(&mut d, "pipe_refs", "blank or duplicate selection");
            continue;
        }
        let matches: Vec<_> = model.pipe_segments.iter().filter(|p| &p.id == id).collect();
        if matches.len() != 1 {
            invalid(&mut d, id, "selected pipe must resolve exactly once");
            continue;
        }
        let pipe = matches[0];
        if !inspection && pipe.provenance.as_ref().is_none_or(|s| s.trim().is_empty()) {
            invalid(&mut d, id, "pipe provenance required");
        }
        {
            let mut positions = Vec::new();
            for node_id in [&pipe.from, &pipe.to] {
                let nodes: Vec<_> = model.nodes.iter().filter(|n| &n.id == node_id).collect();
                if node_id.trim().is_empty() || nodes.len() != 1 {
                    invalid(&mut d, id, "endpoint must resolve exactly once");
                    continue;
                }
                let p = nodes[0].position;
                if ![p.x, p.y, p.z].iter().all(|v| v.is_finite()) {
                    invalid(&mut d, id, "endpoint position must be finite");
                }
                positions.push(p);
            }
            if pipe.from == pipe.to
                || (positions.len() == 2
                    && positions[0].x == positions[1].x
                    && positions[0].y == positions[1].y
                    && positions[0].z == positions[1].z)
            {
                invalid(
                    &mut d,
                    id,
                    "pipe requires distinct endpoints and nonzero geometric span",
                );
            }
        }
        let generated = format!(
            "load:self-weight:{}:{}:{}:{}",
            request.case_id.len(),
            request.case_id,
            id.len(),
            id
        );
        if !used.insert(generated) {
            invalid(&mut d, id, "generated primitive identity collision");
        }
        subset.pipe_segments.push(pipe.clone());
    }
    // W-15 slice b (Design §3): a selected member mapped by a rigid-family
    // component that meets every weight condition carries the component's
    // weight, not its own distributed self-weight; its section is then not a
    // mass input.
    // A selection error (an unresolved member, a bad endpoint, ...) is
    // returned before any component is read, so a mapping to a missing member
    // is a refusal, never a lookup failure.
    if !d.is_empty() {
        return Err(d);
    }
    let mut warnings = Vec::new();
    let bodies = component_weights(model, &selected, &request.gravity.axis, gravity.value, &mut d, &mut warnings);
    for body in bodies.values() {
        for (part, axis, _) in &body.primitives {
            if !used.insert(component_load_id(&request.case_id, &body.component.id, part, axis)) {
                invalid(&mut d, &body.component.id, "generated primitive identity collision");
            }
        }
    }
    subset.pipe_segments.retain(|pipe| !bodies.contains_key(&pipe.id));
    subset.pipe_segments.sort_by(|a, b| a.id.cmp(&b.id));
    resolve_shared_sections(&mut subset, &mut d);
    if !d.is_empty() {
        return Err(d);
    }
    let mut evidence = Vec::new();
    let mut loads = Vec::new();
    // D-5 (DEL-05-03 Design §1): an exact-contract document's mass basis.
    let exact = super::pressure_runtime::is_exact(model);
    for pipe in &mut subset.pipe_segments {
        let original = model
            .pipe_segments
            .iter()
            .find(|p| p.id == pipe.id)
            .expect("unique selected pipe");
        let shared = original.section_ref.as_ref().map(|id| {
            model
                .sections
                .iter()
                .find(|s| &s.id == id)
                .expect("resolved section")
        });
        if !inspection && shared.is_some_and(|s| !provenance_present(&s.provenance)) {
            invalid(&mut d, &pipe.id, "referenced section provenance required");
        }
        let section = &mut pipe.section;
        normalize(
            &mut section.outside_diameter,
            Dimension::Length,
            true,
            "outside_diameter",
            &mut d,
        );
        normalize(
            &mut section.wall_thickness,
            Dimension::Length,
            true,
            "wall_thickness",
            &mut d,
        );
        for (name, q, dim) in [
            (
                "mill_tolerance",
                &mut section.mill_tolerance,
                Dimension::Length,
            ),
            (
                "contents_density",
                &mut section.contents_density,
                Dimension::Density,
            ),
            (
                "insulation_thickness",
                &mut section.insulation_thickness,
                Dimension::Length,
            ),
            (
                "insulation_density",
                &mut section.insulation_density,
                Dimension::Density,
            ),
        ] {
            if let Some(q) = q {
                normalize(q, dim, false, name, &mut d);
            }
        }
        if exact {
            if let Some(q) = &mut section.corrosion_allowance {
                normalize(q, Dimension::Length, false, "corrosion_allowance", &mut d);
            }
        }
        match &mut section.material_density {
            Some(q) => normalize(q, Dimension::Density, true, "material_density", &mut d),
            None => invalid(&mut d, &pipe.id, "explicit material density required"),
        }
        if !d.is_empty() {
            continue;
        }
        let Some(mass) = stable_mass_per_length(pipe, exact, &mut d) else {
            continue;
        };
        let intensity = mass * gravity.value;
        if !mass.is_finite() || mass <= 0.0 || !intensity.is_finite() || intensity == 0.0 {
            invalid(
                &mut d,
                &pipe.id,
                "computed mass or intensity is outside the representable range",
            );
            d.last_mut().unwrap().code = "SELF_WEIGHT_RANGE_INVALID".into();
            continue;
        }
        let s = &original.section;
        let shared_evidence=shared.map(|s|json!({"id":s.id,"name":s.name,"section_type":s.section_type,"provenance":s.provenance,
            "properties":s.properties.iter().map(|(k,v)|(k.clone(),quantity(v))).collect::<serde_json::Map<String,Value>>()}));
        let mut e = json!({"method":GENERATED_METHOD,"gravity":request.gravity,
            "normalized_acceleration_m_per_s2":gravity.value,"request_provenance":request.provenance,
            "pipe_id":pipe.id,"pipe_provenance":original.provenance,"section_ref":original.section_ref,"referenced_section":shared_evidence,
            "mass_inputs":{"outside_diameter":quantity(&s.outside_diameter),"wall_thickness":quantity(&s.wall_thickness),"mill_tolerance":optional(&s.mill_tolerance),
                "material_density":optional(&s.material_density),"contents_density":optional(&s.contents_density),"insulation_thickness":optional(&s.insulation_thickness),"insulation_density":optional(&s.insulation_density)},
            "mass_kg_per_m":mass,"contents_absent":s.contents_density.is_none(),"insulation_absent":s.insulation_thickness.is_none() && s.insulation_density.is_none()});
        if exact {
            e["mass_inputs"]["corrosion_allowance"] = optional(&s.corrosion_allowance);
        }
        e["mass_method"] = json!(if exact { EXACT_MASS_METHOD } else { STABLE_MASS_METHOD });
        e["source_model_hash"] = json!(request.source_model_hash);
        e["normalized_dependencies"] = dependency_projection(&pipe.section, &original.section_ref, exact);
        let id = format!(
            "load:self-weight:{}:{}:{}:{}",
            request.case_id.len(),
            request.case_id,
            pipe.id.len(),
            pipe.id
        );
        let mut payload = json!({"id":id,"category":"distributed_force","target":{"type":"element","pipe":pipe.id},"direction":request.gravity.axis,
            "magnitude":{"value":intensity,"unit":"N/m"},"dimension":"force_per_length"});
        e["generated_payload"] = payload.clone();
        payload["provenance"] = json!(e.to_string());
        loads.push(draft(
            request,
            "create_primitive_load",
            "primitive_loads",
            payload,
            "N/m",
            "force_per_length",
            &e,
        ));
        evidence.push(e);
    }
    if !d.is_empty() {
        return Err(d);
    }
    for body in bodies.values() {
        for (part, axis, value) in &body.primitives {
            let (payload, e) = component_primitive(request, gravity.value, body, part, axis, *value);
            let (unit, dimension) = if *part == "force" { ("N", "force") } else { ("N*m", "moment") };
            loads.push(draft(request, "create_primitive_load", "primitive_loads", payload, unit, dimension, &e));
            evidence.push(e);
        }
    }
    let scope = if bodies.is_empty() { PIPE_MASS_SCOPE } else { COMPONENT_WEIGHT_SCOPE };
    let case = json!({"id":request.case_id,"label":request.label,"kind":"primitive_user_load","status":"draft","provenance":request.provenance,"primitive_loads":[]});
    let mut changes = vec![draft(
        request,
        "create_load_case",
        "load_cases",
        case,
        "none",
        "dimensionless",
        &json!({"scope_label":scope,"source_model_hash":request.source_model_hash,"source_evidence":evidence}),
    )];
    changes.extend(loads);
    Ok(SelfWeightOperationPlan {
        source_model_hash: request.source_model_hash.clone(),
        changes,
        source_evidence: evidence,
        scope_label: scope.into(),
        warnings,
    })
}

/// The scope label of a plan with no component weight: the selected members'
/// own pipe mass only (unchanged since before W-15).
pub const PIPE_MASS_SCOPE: &str = "selected_pipe_mass_only";
/// W-15 slice b: the scope label of a plan in which at least one selected
/// member's distributed self-weight is replaced by its component's weight.
pub const COMPONENT_WEIGHT_SCOPE: &str = "selected_pipe_mass_and_component_weight";
/// W-15 slice b: the evidence method of a component-weight primitive.
pub const COMPONENT_WEIGHT_METHOD: &str = "component_weight_at_center_of_gravity_global_offset/v1";
/// W-15 slice b: the warning naming a rigid-family component whose weight is
/// not generated and the member whose pipe weight is generated in its place.
pub const COMPONENT_WEIGHT_NOT_APPLIED: &str = "SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED";
/// W-15 slice b: the authored category of every component-weight primitive,
/// a node-targeted `weight` force or moment (admitted at nodes by
/// `primitive_loads` since this slice).
pub const COMPONENT_WEIGHT_CATEGORY: &str = "weight";
const AXES: [&str; 3] = ["x", "y", "z"];

/// The deterministic identity of one wrench primitive: the generator's
/// length-prefixed case, then `component`, the length-prefixed component id,
/// the wrench part (`force` or `moment`) and the global axis. The literal
/// `component` cannot be read as a length, so it never meets a member identity.
pub fn component_load_id(case: &str, component: &str, part: &str, axis: &str) -> String {
    format!(
        "load:self-weight:{}:{}:component:{}:{}:{part}:{axis}",
        case.len(),
        case,
        component.len(),
        component
    )
}

fn axis_index(axis: &str) -> Option<usize> {
    match axis {
        "global_x" => Some(0),
        "global_y" => Some(1),
        "global_z" => Some(2),
        _ => None,
    }
}

/// The wrench at the node of a weight W whose centre of gravity is r (global,
/// from the node) under gravity along `axis` with the sign of `acceleration`:
/// F = W·ĝ and M = r × F. With F = f·e_k only, each moment component is one
/// product r_j·f (or its negation), rounded once.
fn wrench(weight: f64, cog: [f64; 3], axis: usize, acceleration: f64) -> (f64, [f64; 3]) {
    let f = if acceleration > 0.0 { weight } else { -weight };
    let [x, y, z] = cog;
    let moment = match axis {
        0 => [0.0, z * f, -(y * f)],
        1 => [-(z * f), 0.0, x * f],
        _ => [y * f, -(x * f), 0.0],
    };
    (f, moment)
}

/// One selected member's component weight (Design §3): the component, its
/// normalized weight (N) and centre of gravity (m), and the wrench primitives
/// (part, axis, signed value): the force along the gravity axis, then each
/// nonzero moment component in x, y, z order.
struct ComponentWeight<'a> {
    component: &'a PreviewComponent,
    pipe: String,
    weight: f64,
    cog: [f64; 3],
    primitives: Vec<(&'static str, &'static str, f64)>,
}

/// The normalized dependencies of a component-weight primitive: what makes it
/// stale when edited (the mapping, the consumption mode, the node, the weight
/// and the centre of gravity), in SI so that an equivalent unit is no change.
fn component_dependencies(component: &PreviewComponent, pipe: &str, weight: f64, cog: [f64; 3]) -> Value {
    json!({"component_id":component.id,"kind":component.kind,"node":component.node,"rigid_pipe_ref":pipe,
        "solver_consumption":GEOMETRY_ONLY,"weight":{"value":weight,"unit":"N"},
        "center_of_gravity":{"x":cog[0],"y":cog[1],"z":cog[2],"unit":"m"}})
}

/// Decisions 1 and 2 of W-15 slice b. For every selected member mapped by a
/// rigid-family component: a mapping conflict (slice a) blocks; a
/// `not_solver_consumed` component is honoured (pipe weight, no warning);
/// otherwise the component's weight replaces the member's distributed
/// self-weight when its consumption is `mechanics_geometry_only`, its mapping
/// is valid (the member ends at its node), its weight is a finite positive
/// force and its centre of gravity a finite length vector. A component that
/// fails a condition gets one warning naming the first unmet one, and the
/// member keeps its pipe weight. No stiffness scale is required.
fn component_weights<'a>(
    model: &'a PreviewModel,
    selected: &HashSet<&String>,
    axis: &str,
    acceleration: f64,
    d: &mut Vec<Diagnostic>,
    warnings: &mut Vec<Diagnostic>,
) -> BTreeMap<String, ComponentWeight<'a>> {
    let mut bodies = BTreeMap::new();
    if model.components.is_empty() {
        return bodies;
    }
    let conflicted: HashSet<String> = rigid_component::mapping_conflicts(model)
        .into_iter()
        .filter(|(member, _)| selected.contains(member))
        .map(|(member, finding)| {
            d.push(finding);
            member
        })
        .collect();
    let Some(axis) = axis_index(axis) else { return bodies };
    for component in model.components.iter().filter(|c| is_rigid_family(c)) {
        let Some(pipe_ref) = mapped_pipe_ref(component).filter(|p| selected.contains(&p.to_string())) else {
            continue;
        };
        if conflicted.contains(pipe_ref) {
            continue;
        }
        let consumption = component.mechanics_interface.as_ref().and_then(|m| m.solver_consumption.as_deref());
        if consumption == Some("not_solver_consumed") {
            continue;
        }
        let Some(pipe) = model.pipe_segments.iter().find(|p| p.id == pipe_ref) else {
            invalid(d, pipe_ref, "selected pipe must resolve exactly once");
            continue;
        };
        let geometry = component.geometry.as_ref();
        let weight = geometry.and_then(|g| g.weight.as_ref()).and_then(normalized_weight);
        let (cog, cog_underflow) = match geometry.and_then(|g| g.center_of_gravity.as_ref()) {
            Some(v) => match normalized_cog(v) {
                Ok(r) => (r, false),
                Err(underflow) => (None, underflow),
            },
            None => (None, false),
        };
        if cog_underflow {
            // A nonzero entered component that normalizes to zero would
            // silently erase its moments.
            invalid(d, &component.id, "a nonzero centre-of-gravity component normalizes to zero in model length units");
            d.last_mut().unwrap().code = "SELF_WEIGHT_RANGE_INVALID".into();
            continue;
        }
        let unmet = match (consumption, weight, cog) {
            (None, _, _) => Some("it declares no mechanics_interface.solver_consumption (no mode is assumed)".to_string()),
            (Some(mode), _, _) if mode != GEOMETRY_ONLY => Some(format!("its solver_consumption is {mode}, not {GEOMETRY_ONLY}")),
            _ if pipe.from != component.node && pipe.to != component.node => {
                Some(format!("its mapped member {pipe_ref} does not end at its node {}", component.node))
            }
            (_, None, _) => Some("its geometry.weight is missing, not a force, or not finite and positive".to_string()),
            (_, _, None) => Some("its geometry.center_of_gravity is missing, not a length vector, or not finite".to_string()),
            _ => None,
        };
        if let Some(reason) = unmet {
            warnings.push(Diagnostic {
                id: format!("diagnostic:self-weight:component:{}:weight-not-applied", crate::stable_suffix(&component.id)),
                code: COMPONENT_WEIGHT_NOT_APPLIED.into(),
                severity: "warning".into(),
                message: format!("rigid/semi-rigid component {} maps selected member {pipe_ref}, but its weight is not generated: {reason}; the distributed self-weight of member {pipe_ref} is generated in its place", component.id),
                source: Some("self_weight".into()),
                affected_refs: vec![component.id.clone(), pipe_ref.to_string()],
            });
            continue;
        }
        let (weight, cog) = (weight.unwrap(), cog.unwrap());
        let (force, moment) = wrench(weight, cog, axis, acceleration);
        if !force.is_finite() {
            invalid(d, &component.id, "component weight force is outside the representable range");
            d.last_mut().unwrap().code = "SELF_WEIGHT_RANGE_INVALID".into();
            continue;
        }
        let mut primitives = vec![("force", AXES[axis], force)];
        for (i, value) in moment.into_iter().enumerate() {
            // r_j ≠ 0 with a zero or non-finite product is a range failure,
            // never a silently dropped moment.
            let lever = match (axis, i) {
                (0, 1) | (1, 0) => cog[2],
                (0, 2) | (2, 0) => cog[1],
                (1, 2) | (2, 1) => cog[0],
                _ => 0.0,
            };
            if lever != 0.0 && (value == 0.0 || !value.is_finite()) {
                invalid(d, &component.id, "component weight moment is outside the representable range");
                d.last_mut().unwrap().code = "SELF_WEIGHT_RANGE_INVALID".into();
            } else if value != 0.0 {
                primitives.push(("moment", AXES[i], value));
            }
        }
        bodies.insert(pipe_ref.to_string(), ComponentWeight { component, pipe: pipe_ref.to_string(), weight, cog, primitives });
    }
    bodies
}

/// The entered weight in newtons, when it is a finite positive force.
fn normalized_weight(q: &Quantity) -> Option<f64> {
    let mut q = q.clone();
    let mut scratch = Vec::new();
    normalize_quantity(&mut q, Dimension::Force, "diagnostic:self-weight:component-weight", vec![], &mut scratch);
    (scratch.is_empty() && q.value.is_finite() && q.value > 0.0).then_some(q.value)
}

/// The entered centre of gravity in metres: `Ok(Some(r))` for a finite length
/// vector, `Ok(None)` when it is not one, and `Err(true)` when a nonzero
/// component normalizes to zero (a range failure, not a missing input).
fn normalized_cog(v: &super::VectorQuantity) -> Result<Option<[f64; 3]>, bool> {
    let entered = [v.x, v.y, v.z];
    if !entered.iter().all(|c| c.is_finite()) {
        return Ok(None);
    }
    let mut v = v.clone();
    let mut scratch = Vec::new();
    normalize_vector_quantity(&mut v, Dimension::Length, "diagnostic:self-weight:component-cog", vec![], &mut scratch);
    let r = [v.x, v.y, v.z];
    if !scratch.is_empty() || !r.iter().all(|c| c.is_finite()) {
        return Ok(None);
    }
    if entered.iter().zip(&r).any(|(e, n)| *e != 0.0 && *n == 0.0) {
        return Err(true);
    }
    Ok(Some(r))
}

/// One wrench primitive and its evidence row (decision 4): the method, the
/// component, the omitted member, the inputs as entered and normalized, the
/// whole wrench, the dependency projection and the generated payload.
fn component_primitive(
    request: &SelfWeightRequest,
    acceleration: f64,
    body: &ComponentWeight,
    part: &str,
    axis: &str,
    value: f64,
) -> (Value, Value) {
    let c = body.component;
    let g = c.geometry.as_ref().expect("a component weight has geometry");
    let index = axis_index(&request.gravity.axis).expect("validated gravity axis");
    let (force, moment) = wrench(body.weight, body.cog, index, acceleration);
    let mut force_vector = [0.0; 3];
    force_vector[index] = force;
    // Both parts are `weight` primitives (a nodal force or moment), so the
    // preview classifies them as weight, like the members' distributed loads.
    let (category, direction, unit, dimension) = if part == "force" {
        (COMPONENT_WEIGHT_CATEGORY, request.gravity.axis.clone(), "N", "force")
    } else {
        (COMPONENT_WEIGHT_CATEGORY, format!("rotation_{axis}"), "N*m", "moment")
    };
    let id = component_load_id(&request.case_id, &c.id, part, axis);
    let mut payload = json!({"id":id,"category":category,"target":{"type":"node","node":c.node},"direction":direction,
        "magnitude":{"value":value,"unit":unit},"dimension":dimension});
    let cog_input = g.center_of_gravity.as_ref().map(|v| json!({"x":v.x,"y":v.y,"z":v.z,"unit":v.unit}));
    let mut e = json!({"method":COMPONENT_WEIGHT_METHOD,"gravity":request.gravity,
        "normalized_acceleration_m_per_s2":acceleration,"request_provenance":request.provenance,
        "component_id":c.id,"component_kind":c.kind,"component_node":c.node,"component_provenance":c.provenance,
        "omitted_pipe_id":body.pipe,"omitted_load":"distributed self-weight of the mapped member",
        "center_of_gravity_convention":"global X/Y/Z axes from the component node, model length units (DEL-03-05 R10, 2026-10-24)",
        "weight_input":optional(&g.weight),"center_of_gravity_input":cog_input,
        "weight_N":body.weight,"center_of_gravity_m":body.cog,
        "force_N":force_vector,"moment_N_m":moment,"wrench_part":part,"wrench_axis":axis});
    e["source_model_hash"] = json!(request.source_model_hash);
    e["normalized_dependencies"] = component_dependencies(c, &body.pipe, body.weight, body.cog);
    e["generated_payload"] = payload.clone();
    payload["provenance"] = json!(e.to_string());
    (payload, e)
}

/// Same explicit density participation as legacy generation, with the named
/// source-area successor. Only annulus_geometry owns the circular area algebra.
///
/// `exact` selects the D-5 basis of an exact-contract section (DEL-05-03
/// Design §1; `EXACT_MASS_METHOD`): pipe metal on the nominal wall t and
/// contents on the pressure bore D/2 - t_r, t_r = (t - c_m) - c_a in that
/// binary64 order (`ExactSectionBases::reduced_wall`), the corrosion allowance
/// c_a an explicit operand. Otherwise (`STABLE_MASS_METHOD`, the preview
/// route, unchanged) both use the single wall t - c_m.
fn stable_mass_per_length(pipe: &super::PreviewPipe, exact: bool, d: &mut Vec<Diagnostic>) -> Option<f64> {
    use crate::annulus_geometry::{
        source_bore_area_m2, source_insulation_area_m2, source_wall_area_m2, SourceAreaError,
    };
    fn range(d: &mut Vec<Diagnostic>, field: &str, message: &str) {
        invalid(d, field, message);
        d.last_mut().unwrap().code = "SELF_WEIGHT_RANGE_INVALID".into();
    }
    fn area(
        value: Result<f64, SourceAreaError>,
        field: &str,
        d: &mut Vec<Diagnostic>,
    ) -> Option<f64> {
        match value {
            Ok(value) => Some(value),
            Err(SourceAreaError::NonRepresentableArea) => {
                range(
                    d,
                    field,
                    "positive source area is outside the representable range",
                );
                None
            }
            Err(error) => {
                invalid(d, field, &format!("invalid source area input: {error:?}"));
                None
            }
        }
    }
    fn term(area: f64, density: f64, field: &str, d: &mut Vec<Diagnostic>) -> Option<f64> {
        if !density.is_finite() || density < 0.0 {
            invalid(d, field, "density must be finite and nonnegative");
            return None;
        }
        let value = area * density;
        if !value.is_finite() || (area > 0.0 && density > 0.0 && value == 0.0) {
            range(
                d,
                field,
                "positive component mass is outside the representable range",
            );
            None
        } else {
            Some(value)
        }
    }
    let s = &pipe.section;
    let od = s.outside_diameter.value;
    let mill = s.mill_tolerance.as_ref().map_or(0.0, |q| q.value);
    if !mill.is_finite() || mill < 0.0 {
        invalid(d, &pipe.id, "mill deduction must be finite and nonnegative");
        return None;
    }
    let (metal_wall, bore_wall) = if exact {
        let Some(allowance) = s.corrosion_allowance.as_ref().map(|q| q.value) else {
            invalid(d, &pipe.id, "explicit corrosion allowance required on an exact-contract section");
            return None;
        };
        if !allowance.is_finite() || allowance < 0.0 {
            invalid(d, &pipe.id, "corrosion allowance must be finite and nonnegative");
            return None;
        }
        let nominal = s.wall_thickness.value;
        (nominal, crate::pressure_exact::ExactSectionBases::reduced_wall(nominal, mill, allowance))
    } else {
        let wall = s.wall_thickness.value - mill;
        (wall, wall)
    };
    let Some(density) = s.material_density.as_ref().map(|q| q.value) else {
        invalid(d, &pipe.id, "explicit material density required");
        return None;
    };
    if !density.is_finite() || density <= 0.0 {
        invalid(d, &pipe.id, "material density must be finite and positive");
        return None;
    }
    let metal = area(source_wall_area_m2(od, metal_wall), &pipe.id, d)?;
    let mut mass = term(metal, density, &pipe.id, d)?;
    if let Some(contents) = &s.contents_density {
        let bore = area(source_bore_area_m2(od, bore_wall), &pipe.id, d)?;
        mass += term(bore, contents.value, &pipe.id, d)?;
    }
    match (&s.insulation_thickness, &s.insulation_density) {
        (None, None) => {}
        (Some(thickness), Some(density)) => {
            let insulation = area(source_insulation_area_m2(od, thickness.value), &pipe.id, d)?;
            mass += term(insulation, density.value, &pipe.id, d)?;
        }
        _ => {
            invalid(
                d,
                &pipe.id,
                "insulation mass requires both thickness and density",
            );
            return None;
        }
    }
    if !mass.is_finite() || mass <= 0.0 {
        range(
            d,
            &pipe.id,
            "total positive mass is outside the representable range",
        );
        None
    } else {
        Some(mass)
    }
}

pub const STABLE_MASS_METHOD: &str = "source_od_effective_wall_areas/v1";
/// T4-U6 (DEL-05-03 Design §1, "D-5 mass"): the mass method of an
/// exact-contract section, metal on the nominal wall and contents on the
/// pressure bore, with the corrosion allowance among its `mass_inputs` and
/// `normalized_dependencies`.
pub const EXACT_MASS_METHOD: &str = "source_od_nominal_wall_pressure_bore_areas/v1";
pub const GENERATED_METHOD: &str = "pipe_mass_per_length_times_explicit_axis_acceleration/v2";
pub const LEGACY_METHOD: &str = "pipe_mass_per_length_times_explicit_axis_acceleration/v1";
pub const MANUAL_OVERRIDE_METHOD: &str = "manual_override_of_generated_self_weight/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AppliedSelfWeightState {
    Fresh,
    Stale,
    Modified,
    ManualOverride,
    Invalid,
}
#[derive(Debug, Clone, Serialize)]
pub struct AppliedSelfWeightStatus {
    pub case_id: String,
    pub primitive_index: usize,
    pub state: AppliedSelfWeightState,
    pub diagnostics: Vec<Diagnostic>,
    pub replacement: Option<Value>,
}
fn valid_hash(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn dependency_projection(s: &super::PipeSectionInput, reference: &Option<String>, exact: bool) -> Value {
    let mut projection = json!({"section_ref":reference,
        "outside_diameter":quantity(&s.outside_diameter),
        "wall_thickness":quantity(&s.wall_thickness),
        "mill_tolerance":optional(&s.mill_tolerance),
        "material_density":optional(&s.material_density),
        "contents_density":optional(&s.contents_density),
        "insulation_thickness":optional(&s.insulation_thickness),
        "insulation_density":optional(&s.insulation_density)});
    // D-5 (EXACT_MASS_METHOD): the allowance moves the contents bore.
    if exact {
        projection["corrosion_allowance"] = optional(&s.corrosion_allowance);
    }
    projection
}
/// The member `normalized_dependencies` carries beside the section's own.
const DEPENDENCY_SECTION_REF: &str = "section_ref";
/// The keys of a generated record's `mass_inputs` (and of a section rebuilt
/// from them): the preview method's, plus the corrosion allowance under
/// `EXACT_MASS_METHOD`.
const MASS_INPUT_KEYS: [&str; 7] = [
    "outside_diameter",
    "wall_thickness",
    "mill_tolerance",
    "material_density",
    "contents_density",
    "insulation_thickness",
    "insulation_density",
];
fn primitive_payload(load: &super::PreviewPrimitiveLoad) -> Value {
    let target = match &load.target {
        super::LoadTargetInput::Node { node } => json!({"type":"node","node":node}),
        super::LoadTargetInput::Element { pipe } => json!({"type":"element","pipe":pipe}),
    };
    json!({"id":load.id,"category":load.category,"target":target,"direction":load.direction,
        "magnitude":quantity(&load.magnitude),"dimension":load.dimension})
}

/// Validate the retained generation basis independently of the current source.
/// This also supplies the conservative v1 dependency and payload reconstruction.
fn retained_basis(
    evidence: &Value,
    case_id: &str,
) -> Result<(SelfWeightRequest, Value, Value), Vec<Diagnostic>> {
    let mut d = Vec::new();
    let fail = || {
        let mut d = Vec::new();
        invalid(
            &mut d,
            case_id,
            "generated self-weight provenance is incomplete or inconsistent",
        );
        d
    };
    let method = evidence["method"].as_str().ok_or_else(fail)?;
    if !matches!(method, GENERATED_METHOD | LEGACY_METHOD) {
        return Err(fail());
    }
    let pipe_id = evidence["pipe_id"]
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(fail)?;
    let provenance = evidence["request_provenance"]
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(fail)?;
    let gravity: SelfWeightGravity =
        serde_json::from_value(evidence["gravity"].clone()).map_err(|_| fail())?;
    if !matches!(gravity.axis.as_str(), "global_x" | "global_y" | "global_z")
        || !gravity.value.is_finite()
        || gravity.value == 0.0
    {
        return Err(fail());
    }
    let mut acceleration = Quantity {
        value: gravity.value,
        unit: gravity.unit.clone(),
    };
    normalize_quantity(
        &mut acceleration,
        Dimension::Acceleration,
        "diagnostic:self-weight:gravity",
        vec![case_id.into()],
        &mut d,
    );
    if !d.is_empty() {
        return Err(d);
    }
    if !acceleration.value.is_finite()
        || acceleration.value == 0.0
        || evidence["normalized_acceleration_m_per_s2"].as_f64() != Some(acceleration.value)
    {
        return Err(fail());
    }
    let reference: Option<String> =
        serde_json::from_value(evidence.get("section_ref").cloned().ok_or_else(fail)?)
            .map_err(|_| fail())?;
    // The generated record's mass basis: the preview method or, on an
    // exact-contract section, D-5's (with the allowance as an input).
    // (A generated record naming neither method fails below, where the
    // preview method was always checked.)
    let exact = method == GENERATED_METHOD && evidence["mass_method"] == EXACT_MASS_METHOD;
    let inputs = evidence["mass_inputs"].as_object().ok_or_else(fail)?;
    if MASS_INPUT_KEYS.iter().any(|key| !inputs.contains_key(*key))
        || (exact && !inputs.contains_key("corrosion_allowance"))
    {
        return Err(fail());
    }
    let mut section: super::PipeSectionInput =
        serde_json::from_value(Value::Object(inputs.clone())).map_err(|_| fail())?;
    if let Some(reference) = &reference {
        let shared = &evidence["referenced_section"];
        if reference.trim().is_empty()
            || shared["id"].as_str() != Some(reference.as_str())
            || shared["section_type"] != "pipe"
        {
            return Err(fail());
        }
        let props = shared["properties"].as_object().ok_or_else(fail)?;
        if props.len() != 2
            || props.get("outside_diameter") != inputs.get("outside_diameter")
            || props.get("wall_thickness") != inputs.get("wall_thickness")
        {
            return Err(fail());
        }
    } else if !evidence["referenced_section"].is_null() {
        return Err(fail());
    }
    normalize(
        &mut section.outside_diameter,
        Dimension::Length,
        true,
        "outside_diameter",
        &mut d,
    );
    normalize(
        &mut section.wall_thickness,
        Dimension::Length,
        true,
        "wall_thickness",
        &mut d,
    );
    for (name, q, dimension) in [
        (
            "mill_tolerance",
            &mut section.mill_tolerance,
            Dimension::Length,
        ),
        (
            "contents_density",
            &mut section.contents_density,
            Dimension::Density,
        ),
        (
            "insulation_thickness",
            &mut section.insulation_thickness,
            Dimension::Length,
        ),
        (
            "insulation_density",
            &mut section.insulation_density,
            Dimension::Density,
        ),
    ] {
        if let Some(q) = q {
            normalize(q, dimension, false, name, &mut d);
        }
    }
    if exact {
        if let Some(q) = &mut section.corrosion_allowance {
            normalize(q, Dimension::Length, false, "corrosion_allowance", &mut d);
        }
    }
    match &mut section.material_density {
        Some(q) => normalize(q, Dimension::Density, true, "material_density", &mut d),
        None => return Err(fail()),
    }
    if !d.is_empty() {
        return Err(d);
    }
    if evidence["contents_absent"] != json!(section.contents_density.is_none())
        || evidence["insulation_absent"]
            != json!(section.insulation_thickness.is_none() && section.insulation_density.is_none())
    {
        return Err(fail());
    }
    let dependencies = dependency_projection(&section, &reference, exact);
    let pipe = super::PreviewPipe {
        id: pipe_id.into(),
        from: String::new(),
        to: String::new(),
        section,
        section_ref: reference,
        material: String::new(),
        y_reference: None,
        provenance: None,
        label: serde::de::IgnoredAny,
        tags: serde::de::IgnoredAny,
        notes: serde::de::IgnoredAny,
    };
    let mass = if method == LEGACY_METHOD {
        compute_pipe_mass_per_length(&pipe, case_id, &mut d)
    } else {
        if !exact && evidence["mass_method"] != STABLE_MASS_METHOD {
            return Err(fail());
        }
        stable_mass_per_length(&pipe, exact, &mut d)
    };
    let Some(mass) = mass else {
        return Err(if d.is_empty() { fail() } else { d });
    };
    if !d.is_empty() {
        return Err(d);
    }
    // v1's decimal transport can make its multiplication ambiguous. Preserve
    // that ambiguity as Modified below rather than adding a rounding tolerance.
    let retained_mass = evidence["mass_kg_per_m"]
        .as_f64()
        .filter(|m| m.is_finite() && *m > 0.0)
        .ok_or_else(fail)?;
    let intensity = retained_mass * acceleration.value;
    if !mass.is_finite() || mass <= 0.0 || !intensity.is_finite() {
        return Err(fail());
    }
    let source_hash = evidence["source_model_hash"].as_str().unwrap_or("");
    let generated = json!({"id":format!("load:self-weight:{}:{}:{}:{}",case_id.len(),case_id,pipe_id.len(),pipe_id),
        "category":"distributed_force","target":{"type":"element","pipe":pipe_id},"direction":gravity.axis,
        "magnitude":{"value":intensity,"unit":"N/m"},"dimension":"force_per_length"});
    let payload = if method == GENERATED_METHOD {
        if !valid_hash(source_hash) || evidence["normalized_dependencies"] != dependencies {
            return Err(fail());
        }
        if let Some(legacy) = evidence.get("legacy_generation_provenance") {
            let legacy: Value =
                serde_json::from_str(legacy.as_str().ok_or_else(fail)?).map_err(|_| fail())?;
            if legacy["method"] != LEGACY_METHOD {
                return Err(fail());
            }
            retained_basis(&legacy, case_id)?;
        }
        let snapshot = &evidence["generated_payload"];
        let mut expected = generated.clone();
        // Recompute independently from retained source quantities; the snapshot
        // is an exact edit detector, not its own physical-value oracle.
        let q: Quantity =
            serde_json::from_value(snapshot["magnitude"].clone()).map_err(|_| fail())?;
        if q.unit != "N/m" || !q.value.is_finite() || q.value == 0.0 {
            return Err(fail());
        }
        if Some(mass) != Some(retained_mass) || Some(mass * acceleration.value) != Some(q.value) {
            return Err(fail());
        }
        expected["magnitude"] = snapshot["magnitude"].clone();
        if snapshot != &expected {
            return Err(fail());
        }
        snapshot.clone()
    } else {
        generated
    };
    Ok((
        SelfWeightRequest {
            case_id: case_id.into(),
            label: "Applied self-weight refresh".into(),
            pipe_refs: vec![pipe_id.into()],
            gravity,
            provenance: provenance.into(),
            source_model_hash: source_hash.into(),
        },
        dependencies,
        payload,
    ))
}

/// The retained basis of any generated record: a component-weight primitive's
/// (W-15 slice b) or a member's distributed self-weight.
fn generation_basis(
    evidence: &Value,
    case_id: &str,
) -> Result<(SelfWeightRequest, Value, Value), Vec<Diagnostic>> {
    if evidence["method"] == COMPONENT_WEIGHT_METHOD {
        component_basis(evidence, case_id)
    } else {
        retained_basis(evidence, case_id)
    }
}

/// W-15 slice b: validate a component-weight record independently of the
/// current source. The wrench is recomputed from the retained weight, centre
/// of gravity and gravity sign; the snapshot is an exact edit detector, not
/// its own oracle.
fn component_basis(
    evidence: &Value,
    case_id: &str,
) -> Result<(SelfWeightRequest, Value, Value), Vec<Diagnostic>> {
    let fail = || {
        let mut d = Vec::new();
        invalid(&mut d, case_id, "generated component-weight provenance is incomplete or inconsistent");
        d
    };
    let text = |key: &str| evidence[key].as_str().filter(|s| !s.trim().is_empty()).ok_or_else(fail);
    let (component, node, pipe, provenance) =
        (text("component_id")?, text("component_node")?, text("omitted_pipe_id")?, text("request_provenance")?);
    let source_hash = evidence["source_model_hash"].as_str().filter(|h| valid_hash(h)).ok_or_else(fail)?;
    let gravity: SelfWeightGravity = serde_json::from_value(evidence["gravity"].clone()).map_err(|_| fail())?;
    let axis = axis_index(&gravity.axis).ok_or_else(fail)?;
    if !gravity.value.is_finite() || gravity.value == 0.0 {
        return Err(fail());
    }
    let mut acceleration = Quantity { value: gravity.value, unit: gravity.unit.clone() };
    let mut d = Vec::new();
    normalize_quantity(&mut acceleration, Dimension::Acceleration, "diagnostic:self-weight:gravity", vec![case_id.into()], &mut d);
    if !d.is_empty() {
        return Err(d);
    }
    if !acceleration.value.is_finite()
        || acceleration.value == 0.0
        || evidence["normalized_acceleration_m_per_s2"].as_f64() != Some(acceleration.value)
    {
        return Err(fail());
    }
    // The historical wrench follows from the retained authored quantities,
    // normalized again here; the recorded SI values, dependencies and whole
    // wrench must agree with it exactly, or the record is not lineage.
    // Both are closed quantities, as on the component they were read from: an
    // undeclared member is not lineage.
    let weight_input = super::model_fields::closed_quantity(evidence["weight_input"].clone()).map_err(|_| fail())?;
    let weight = normalized_weight(&weight_input).ok_or_else(fail)?;
    let cog_input = super::model_fields::closed_optional_vector_quantity(evidence["center_of_gravity_input"].clone())
        .map_err(|_| fail())?
        .ok_or_else(fail)?;
    let cog = normalized_cog(&cog_input).ok().flatten().ok_or_else(fail)?;
    let recorded_cog: [f64; 3] = serde_json::from_value(evidence["center_of_gravity_m"].clone()).map_err(|_| fail())?;
    if evidence["weight_N"].as_f64() != Some(weight) || recorded_cog != cog {
        return Err(fail());
    }
    let dependencies = &evidence["normalized_dependencies"];
    let kind = dependencies["kind"].as_str().ok_or_else(fail)?;
    let expected_dependencies = json!({"component_id":component,"kind":kind,"node":node,"rigid_pipe_ref":pipe,
        "solver_consumption":GEOMETRY_ONLY,"weight":{"value":weight,"unit":"N"},
        "center_of_gravity":{"x":cog[0],"y":cog[1],"z":cog[2],"unit":"m"}});
    if dependencies != &expected_dependencies {
        return Err(fail());
    }
    let (force, moment) = wrench(weight, cog, axis, acceleration.value);
    let mut force_vector = [0.0; 3];
    force_vector[axis] = force;
    let recorded_force: [f64; 3] = serde_json::from_value(evidence["force_N"].clone()).map_err(|_| fail())?;
    let recorded_moment: [f64; 3] = serde_json::from_value(evidence["moment_N_m"].clone()).map_err(|_| fail())?;
    if recorded_force != force_vector || recorded_moment != moment {
        return Err(fail());
    }
    let part = evidence["wrench_part"].as_str().ok_or_else(fail)?;
    let part_axis = evidence["wrench_axis"].as_str().and_then(|a| AXES.iter().position(|x| *x == a)).ok_or_else(fail)?;
    let (category, direction, unit, dimension, value) = match part {
        "force" if part_axis == axis => (COMPONENT_WEIGHT_CATEGORY, gravity.axis.clone(), "N", "force", force),
        "moment" if moment[part_axis] != 0.0 => {
            (COMPONENT_WEIGHT_CATEGORY, format!("rotation_{}", AXES[part_axis]), "N*m", "moment", moment[part_axis])
        }
        _ => return Err(fail()),
    };
    let snapshot = &evidence["generated_payload"];
    let q: Quantity = serde_json::from_value(snapshot["magnitude"].clone()).map_err(|_| fail())?;
    if q.unit != unit || q.value != value {
        return Err(fail());
    }
    let mut expected = json!({"id":component_load_id(case_id, component, part, AXES[part_axis]),"category":category,
        "target":{"type":"node","node":node},"direction":direction,"magnitude":{"value":value,"unit":unit},"dimension":dimension});
    expected["magnitude"] = snapshot["magnitude"].clone();
    if snapshot != &expected {
        return Err(fail());
    }
    Ok((
        SelfWeightRequest {
            case_id: case_id.into(),
            label: "Applied self-weight refresh".into(),
            pipe_refs: vec![pipe.into()],
            gravity,
            provenance: provenance.into(),
            source_model_hash: source_hash.into(),
        },
        dependencies.clone(),
        snapshot.clone(),
    ))
}

/// Inspect on the original, unnormalized view. Only a caller-authenticated hash
/// requests replacement data; absence never manufactures provenance or mutates.
pub fn inspect_applied_self_weight(
    model: &PreviewModel,
    source_model_hash: Option<&str>,
) -> Vec<AppliedSelfWeightStatus> {
    let mut statuses = Vec::new();
    for case in &model.load_cases {
        for (primitive_index, load) in case.primitive_loads.iter().enumerate() {
            let evidence = load
                .provenance
                .as_deref()
                .and_then(|p| serde_json::from_str::<Value>(p).ok());
            let method = evidence.as_ref().and_then(|p| p["method"].as_str());
            let recognized = matches!(
                method,
                Some(GENERATED_METHOD | LEGACY_METHOD | MANUAL_OVERRIDE_METHOD | COMPONENT_WEIGHT_METHOD)
            );
            let claimed_method_family = method.is_some_and(|method| {
                method == "pipe_mass_per_length_times_explicit_axis_acceleration"
                    || method.starts_with("pipe_mass_per_length_times_explicit_axis_acceleration/")
                    || method == "manual_override_of_generated_self_weight"
                    || method.starts_with("manual_override_of_generated_self_weight/")
                    || method.starts_with("component_weight_at_center_of_gravity")
            });
            let deterministic_identity = match &load.target {
                super::LoadTargetInput::Element { pipe } => {
                    load.id
                        == format!(
                            "load:self-weight:{}:{}:{}:{}",
                            case.id.len(),
                            case.id,
                            pipe.len(),
                            pipe
                        )
                }
                // W-15 slice b: a node-targeted load in the generated
                // namespace claims component-weight lineage (allocation-free).
                super::LoadTargetInput::Node { .. } => load.id.starts_with("load:self-weight:"),
            };
            let looks_generated = claimed_method_family || deterministic_identity;
            if !recognized && !looks_generated {
                continue;
            }
            let mut status = AppliedSelfWeightStatus {
                case_id: case.id.clone(),
                primitive_index,
                state: AppliedSelfWeightState::Invalid,
                diagnostics: Vec::new(),
                replacement: None,
            };
            let mut scope_changed = false;
            let result = (|| -> Result<(), Vec<Diagnostic>> {
                let fail = || {
                    let mut d = Vec::new();
                    invalid(&mut d, &load.id, "generated self-weight lineage is invalid; explicit source reconciliation required");
                    d
                };
                let evidence = evidence.as_ref().ok_or_else(fail)?;
                if method == Some(MANUAL_OVERRIDE_METHOD) {
                    if evidence["decision"] != "preserve_modified_generated_load"
                        || !evidence["source_model_hash"]
                            .as_str()
                            .is_some_and(valid_hash)
                    {
                        return Err(fail());
                    }
                    generation_basis(&evidence["original_generation_provenance"], &case.id)?;
                    status.state = AppliedSelfWeightState::ManualOverride;
                    return Ok(());
                }
                let (mut request, old_dependencies, old_payload) =
                    generation_basis(evidence, &case.id)?;
                if let Some(hash) = source_model_hash {
                    if !valid_hash(hash) {
                        return Err(fail());
                    }
                    request.source_model_hash = hash.into();
                }
                let payload_changed = primitive_payload(load) != old_payload;
                // Legacy provenance did not retain the exact emitted float. An
                // inconsistent old mass is unverifiable, never auto-refreshable.
                let legacy_uncertain = method == Some(LEGACY_METHOD)
                    && evidence["mass_kg_per_m"].as_f64()
                        != old_dependencies_mass(evidence, &case.id);
                // A modified primitive can be explicitly detached even when its
                // former source was removed. Ordinary validation still owns the
                // current target and physical payload; historical lineage above
                // must remain valid before preservation can be offered.
                if payload_changed || legacy_uncertain {
                    status.state = AppliedSelfWeightState::Modified;
                    return Ok(());
                }
                // Clear only load identity collisions on a clone. Current source
                // quantities and shared-section cache checks stay authoritative
                // for primitives that retain their generated physical payload.
                let mut source = model.clone();
                source.load_cases.clear();
                let generated = generate_operations(&source, &request, true)?;
                // The regenerated row of this primitive: the member's own row,
                // or the same component wrench part and axis. None when the
                // selection now generates other loads (W-15 slice b: a
                // component weight replaces, or no longer replaces, the
                // member's distributed self-weight, or a moment component
                // appears or vanishes); no single replacement then exists.
                let component = method == Some(COMPONENT_WEIGHT_METHOD);
                let row = generated.source_evidence.iter().position(|row| {
                    if component {
                        row["method"] == COMPONENT_WEIGHT_METHOD
                            && row["component_id"] == evidence["component_id"]
                            && row["wrench_part"] == evidence["wrench_part"]
                            && row["wrench_axis"] == evidence["wrench_axis"]
                    } else {
                        row["method"] == GENERATED_METHOD && row["pipe_id"] == evidence["pipe_id"]
                    }
                });
                let Some(row) = row else {
                    status.state = AppliedSelfWeightState::Stale;
                    scope_changed = true;
                    return Ok(());
                };
                // The complete wrench set: a component's primitives in this
                // case (manual overrides of its primitives hold their slot)
                // must be exactly those the current inputs generate. A moment
                // that appears or vanishes leaves no in-place refresh.
                if component {
                    let wanted = &evidence["component_id"];
                    let key = |e: &Value| (e["wrench_part"].as_str().unwrap_or_default().to_string(), e["wrench_axis"].as_str().unwrap_or_default().to_string());
                    let existing: BTreeSet<_> = case
                        .primitive_loads
                        .iter()
                        .filter_map(|l| serde_json::from_str::<Value>(l.provenance.as_deref()?).ok())
                        .filter_map(|e| {
                            let e = if e["method"] == MANUAL_OVERRIDE_METHOD { e["original_generation_provenance"].clone() } else { e };
                            (e["method"] == COMPONENT_WEIGHT_METHOD && &e["component_id"] == wanted).then(|| key(&e))
                        })
                        .collect();
                    let regenerated: BTreeSet<_> = generated
                        .source_evidence
                        .iter()
                        .filter(|e| e["method"] == COMPONENT_WEIGHT_METHOD && &e["component_id"] == wanted)
                        .map(key)
                        .collect();
                    if existing != regenerated {
                        status.state = AppliedSelfWeightState::Stale;
                        scope_changed = true;
                        return Ok(());
                    }
                }
                let current_evidence = &generated.source_evidence[row];
                // A document moved across the exact contract changes the mass
                // method (D-5) even where the consumed quantities agree.
                status.state = if method == Some(LEGACY_METHOD)
                    || current_evidence["normalized_dependencies"] != old_dependencies
                    || current_evidence["mass_method"] != evidence["mass_method"]
                {
                    AppliedSelfWeightState::Stale
                } else {
                    AppliedSelfWeightState::Fresh
                };
                if status.state == AppliedSelfWeightState::Stale && source_model_hash.is_some() {
                    let mut replacement: Value =
                        serde_json::from_str(&generated.changes[row + 1].after).map_err(|_| fail())?;
                    let mut provenance: Value =
                        serde_json::from_str(replacement["provenance"].as_str().ok_or_else(fail)?)
                            .map_err(|_| fail())?;
                    if method == Some(LEGACY_METHOD) {
                        provenance["legacy_generation_provenance"] =
                            json!(load.provenance.as_deref().ok_or_else(fail)?);
                    } else if let Some(legacy) = evidence.get("legacy_generation_provenance") {
                        provenance["legacy_generation_provenance"] = legacy.clone();
                    }
                    replacement["provenance"] = json!(provenance.to_string());
                    status.replacement = Some(replacement);
                }
                Ok(())
            })();
            if let Err(mut diagnostics) = result {
                for diagnostic in &mut diagnostics {
                    diagnostic.affected_refs.push(case.id.clone());
                    diagnostic.affected_refs.push(load.id.clone());
                }
                status.diagnostics = diagnostics;
            } else if status.state != AppliedSelfWeightState::Fresh {
                let (code, severity, message) = match status.state {
                    AppliedSelfWeightState::Stale if method == Some(LEGACY_METHOD) => ("SELF_WEIGHT_METHOD_REFRESH_REQUIRED", "blocking", "Legacy self-weight requires explicit refresh to the source-area mass method before solving."),
                    AppliedSelfWeightState::Stale if scope_changed => ("SELF_WEIGHT_INPUTS_STALE", "blocking", "Consumed self-weight inputs changed which loads the selection generates (a component weight now replaces, or no longer replaces, a member's distributed self-weight, or a component moment appears or vanishes); generate this self-weight case again before solving."),
                    AppliedSelfWeightState::Stale => ("SELF_WEIGHT_INPUTS_STALE", "blocking", "Consumed self-weight inputs changed; explicitly refresh this generated load before solving."),
                    AppliedSelfWeightState::Modified => ("SELF_WEIGHT_GENERATED_LOAD_MODIFIED", "blocking", "Generated self-weight payload changed or is unverifiable; explicitly preserve it as a manual override before solving."),
                    AppliedSelfWeightState::ManualOverride => ("SELF_WEIGHT_MANUAL_OVERRIDE", "warning", "Explicitly preserved self-weight is a fixed manual load and will not follow mass input changes."),
                    _ => unreachable!(),
                };
                status.diagnostics.push(Diagnostic {
                    id: format!("diagnostic:self-weight:{}:{primitive_index}", case.id),
                    code: code.into(),
                    severity: severity.into(),
                    message: message.into(),
                    source: Some("self_weight".into()),
                    affected_refs: vec![case.id.clone(), load.id.clone()],
                });
            }
            statuses.push(status);
        }
    }
    statuses
}

// Reconstructing legacy mass through the same helper avoids using a second
// physics formula. A mismatch is an explicit-reconciliation requirement.
fn old_dependencies_mass(evidence: &Value, case_id: &str) -> Option<f64> {
    let (_, mut dependencies, _) = retained_basis(evidence, case_id).ok()?;
    // The projection's one member beside the section's own (the closed section refuses any other).
    dependencies.as_object_mut()?.remove(DEPENDENCY_SECTION_REF);
    let section = serde_json::from_value(dependencies).ok()?;
    let pipe = super::PreviewPipe {
        id: String::new(),
        from: String::new(),
        to: String::new(),
        section,
        section_ref: None,
        material: String::new(),
        y_reference: None,
        provenance: None,
        label: serde::de::IgnoredAny,
        tags: serde::de::IgnoredAny,
        notes: serde::de::IgnoredAny,
    };
    compute_pipe_mass_per_length(&pipe, case_id, &mut Vec::new())
}
pub fn validate_applied_self_weight(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for status in inspect_applied_self_weight(model, None) {
        diagnostics.extend(status.diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (PreviewModel, SelfWeightRequest) {
        let model=serde_json::from_value(json!({"schema_version":"invented","document_kind":"preview","project":{"id":"invented"},
            "analysis_status":{"mechanics":"draft","rule_check":"not_run","professional_acceptance":"not_assessed"},
            "nodes":[{"id":"a","position":{"x":0,"y":0,"z":0}},{"id":"b","position":{"x":1,"y":0,"z":0}}],
            "pipe_segments":[{"id":"pipe:é","from":"a","to":"b","material":"invented","provenance":"invented pipe",
                "section":{"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"},"material_density":{"value":1000,"unit":"kg/m^3"}}}],"supports":[]})).unwrap();
        let request = SelfWeightRequest {
            case_id: "case:α".into(),
            label: "invented self mass".into(),
            pipe_refs: vec!["pipe:é".into()],
            gravity: SelfWeightGravity {
                value: -7.0,
                unit: "m/s^2".into(),
                axis: "global_y".into(),
            },
            provenance: "invented explicit input".into(),
            source_model_hash: format!("sha256:{}", "a".repeat(64)),
        };
        (model, request)
    }
    fn plan_value(m: &PreviewModel, r: &SelfWeightRequest) -> Value {
        serde_json::to_value(generate_self_weight_operations(m, r).unwrap()).unwrap()
    }
    fn intensity(m: &PreviewModel, r: &SelfWeightRequest) -> f64 {
        let plan = generate_self_weight_operations(m, r).unwrap();
        serde_json::from_str::<Value>(&plan.changes[1].after).unwrap()["magnitude"]["value"]
            .as_f64()
            .unwrap()
    }
    fn close(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.0),
            "{a} != {b}"
        );
    }
    fn applied() -> (PreviewModel, SelfWeightRequest) {
        let (mut model, request) = fixture();
        let plan = generate_self_weight_operations(&model, &request).unwrap();
        let mut case: Value = serde_json::from_str(&plan.changes[0].after).unwrap();
        case["primitive_loads"] =
            json!([serde_json::from_str::<Value>(&plan.changes[1].after).unwrap()]);
        model.load_cases = serde_json::from_value(json!([case])).unwrap();
        (model, request)
    }
    fn state(model: &PreviewModel) -> AppliedSelfWeightState {
        inspect_applied_self_weight(model, None)[0].state
    }
    #[test]
    fn applied_self_weight_is_pure_and_density_drift_blocks() {
        let (mut model, request) = applied();
        assert_eq!(state(&model), AppliedSelfWeightState::Fresh);
        model.pipe_segments[0]
            .section
            .material_density
            .as_mut()
            .unwrap()
            .value *= 2.0;
        let before = format!("{model:?}");
        let checked = inspect_applied_self_weight(&model, None);
        assert_eq!(checked[0].state, AppliedSelfWeightState::Stale);
        assert_eq!(checked[0].diagnostics[0].code, "SELF_WEIGHT_INPUTS_STALE");
        assert!(checked[0].replacement.is_none());
        assert_eq!(format!("{model:?}"), before);
        let refreshed = inspect_applied_self_weight(&model, Some(&request.source_model_hash));
        let replacement = refreshed[0].replacement.clone().unwrap();
        close(
            replacement["magnitude"]["value"].as_f64().unwrap(),
            2.0 * model.load_cases[0].primitive_loads[0].magnitude.value,
        );
        model.load_cases[0].primitive_loads[0] = serde_json::from_value(replacement).unwrap();
        assert_eq!(state(&model), AppliedSelfWeightState::Fresh);
    }
    #[test]
    fn applied_self_weight_ignores_unconsumed_edits_and_equivalent_units() {
        let (mut model, _) = applied();
        model.nodes[1].position.x = 3.0;
        model.nodes[1].position.y = 2.0;
        model.pipe_segments[0].material = "another stiffness material".into();
        model.pipe_segments[0].provenance = None;
        model.project.units = json!({"length":"mm"});
        model.pipe_segments[0].section.outside_diameter = Quantity {
            value: 100.0,
            unit: "mm".into(),
        };
        model.pipe_segments[0].section.wall_thickness = Quantity {
            value: 10.0,
            unit: "mm".into(),
        };
        let mut unselected = model.pipe_segments[0].clone();
        unselected.id = "unselected".into();
        unselected.section.material_density = None;
        unselected.section_ref = Some("missing".into());
        model.pipe_segments.push(unselected);
        assert_eq!(state(&model), AppliedSelfWeightState::Fresh);
    }
    #[test]
    fn applied_self_weight_checks_each_consumed_input() {
        for mode in 0..7 {
            let (mut model, _) = applied();
            let section = &mut model.pipe_segments[0].section;
            match mode {
                0 => section.outside_diameter.value = 0.11,
                1 => section.wall_thickness.value = 0.011,
                2 => section.material_density.as_mut().unwrap().value = 1100.0,
                3 => {
                    section.mill_tolerance = Some(Quantity {
                        value: 0.001,
                        unit: "m".into(),
                    })
                }
                4 => {
                    section.contents_density = Some(Quantity {
                        value: 10.0,
                        unit: "kg/m^3".into(),
                    })
                }
                _ => {
                    section.insulation_thickness = Some(Quantity {
                        value: 0.01,
                        unit: "m".into(),
                    });
                    section.insulation_density = Some(Quantity {
                        value: if mode == 5 { 10.0 } else { 20.0 },
                        unit: "kg/m^3".into(),
                    });
                }
            }
            assert_eq!(state(&model), AppliedSelfWeightState::Stale, "mode {mode}");
        }
    }
    #[test]
    fn applied_self_weight_invalid_units_and_shared_cache_are_not_stale() {
        let (mut model, _) = applied();
        model.pipe_segments[0]
            .section
            .material_density
            .as_mut()
            .unwrap()
            .unit = "N".into();
        assert_eq!(state(&model), AppliedSelfWeightState::Invalid);
        let (mut model, _) = applied();
        model.pipe_segments[0].section_ref = Some("s".into());
        model.sections = serde_json::from_value(json!([{"id":"s","name":"s","section_type":"pipe","properties":{"outside_diameter":{"value":0.2,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"source"}])).unwrap();
        let status = inspect_applied_self_weight(&model, None);
        assert_eq!(status[0].state, AppliedSelfWeightState::Invalid);
        assert!(status[0]
            .diagnostics
            .iter()
            .any(|d| d.code == "SECTION_REFERENCE_CACHE_STALE"));
        model.pipe_segments[0].section.outside_diameter.value = 0.2;
        assert_eq!(state(&model), AppliedSelfWeightState::Stale);
    }
    #[test]
    fn applied_self_weight_modified_requires_explicit_manual_preservation() {
        let (mut model, request) = applied();
        model.load_cases[0].primitive_loads[0].magnitude.value = -123.0;
        let inspected = inspect_applied_self_weight(&model, Some(&request.source_model_hash));
        assert_eq!(inspected[0].state, AppliedSelfWeightState::Modified);
        assert!(inspected[0].replacement.is_none());
        let load = &mut model.load_cases[0].primitive_loads[0];
        let original: Value = serde_json::from_str(load.provenance.as_ref().unwrap()).unwrap();
        load.provenance = Some(json!({"method":MANUAL_OVERRIDE_METHOD,"original_generation_provenance":original,"decision":"preserve_modified_generated_load","source_model_hash":request.source_model_hash}).to_string());
        model.pipe_segments[0]
            .section
            .material_density
            .as_mut()
            .unwrap()
            .value *= 2.0;
        let checked = inspect_applied_self_weight(&model, None);
        assert_eq!(checked[0].state, AppliedSelfWeightState::ManualOverride);
        assert_eq!(checked[0].diagnostics[0].severity, "warning");
        assert_eq!(
            model.load_cases[0].primitive_loads[0].magnitude.value,
            -123.0
        );
    }
    #[test]
    fn modified_retarget_can_be_preserved_after_original_source_is_removed() {
        let (mut model, request) = applied();
        model.pipe_segments[0].id = "new_manual_target".into();
        let load = &mut model.load_cases[0].primitive_loads[0];
        load.target = super::super::LoadTargetInput::Element {
            pipe: "new_manual_target".into(),
        };
        let original: Value = serde_json::from_str(load.provenance.as_ref().unwrap()).unwrap();
        let before = primitive_payload(load);
        let checked = inspect_applied_self_weight(&model, Some(&request.source_model_hash));
        assert_eq!(checked[0].state, AppliedSelfWeightState::Modified);
        assert!(checked[0].replacement.is_none());
        model.load_cases[0].primitive_loads[0].provenance = Some(json!({
            "method":MANUAL_OVERRIDE_METHOD,"original_generation_provenance":original,
            "decision":"preserve_modified_generated_load","source_model_hash":request.source_model_hash
        }).to_string());
        assert_eq!(state(&model), AppliedSelfWeightState::ManualOverride);
        assert_eq!(
            primitive_payload(&model.load_cases[0].primitive_loads[0]),
            before
        );
        let (mut unmodified, _) = applied();
        unmodified.pipe_segments.clear();
        assert_eq!(state(&unmodified), AppliedSelfWeightState::Invalid);
    }
    #[test]
    fn applied_self_weight_fails_closed_for_malformed_lineage_but_ignores_manual() {
        let (mut model, _) = applied();
        model.load_cases[0].primitive_loads[0].provenance = None;
        assert_eq!(state(&model), AppliedSelfWeightState::Invalid);
        model.load_cases[0].primitive_loads[0].provenance =
            Some(json!({"method":GENERATED_METHOD}).to_string());
        assert_eq!(state(&model), AppliedSelfWeightState::Invalid);
        model.load_cases[0].primitive_loads[0].id = "ordinary_manual".into();
        model.load_cases[0].primitive_loads[0].provenance = Some("human input".into());
        assert!(inspect_applied_self_weight(&model, None).is_empty());
    }
    #[test]
    fn ordinary_manual_mentions_and_near_prefixes_are_not_lineage() {
        let (mut model, _) = applied();
        let load = &mut model.load_cases[0].primitive_loads[0];
        load.id.push_str(":manual");
        for provenance in [
            "human compared pipe_mass_per_length_times_explicit_axis_acceleration/v2".to_string(),
            "human compared manual_override_of_generated_self_weight/v1".to_string(),
            json!({"note":GENERATED_METHOD}).to_string(),
            json!({"method":"human comparison of pipe_mass_per_length_times_explicit_axis_acceleration/v2"}).to_string(),
        ] {
            model.load_cases[0].primitive_loads[0].provenance = Some(provenance);
            assert!(inspect_applied_self_weight(&model, None).is_empty());
        }
        model.load_cases[0].primitive_loads[0].provenance = Some(
            json!({"method":"pipe_mass_per_length_times_explicit_axis_acceleration/v999"})
                .to_string(),
        );
        assert_eq!(state(&model), AppliedSelfWeightState::Invalid);
    }
    #[test]
    fn applied_self_weight_invalid_current_geometry_is_input_invalid() {
        for mode in 0..5 {
            let (mut model, _) = applied();
            model.pipe_segments[0]
                .section
                .material_density
                .as_mut()
                .unwrap()
                .value *= 2.0;
            match mode {
                0 => model.nodes.clear(),
                1 => model.nodes.push(model.nodes[0].clone()),
                2 => model.nodes[1].position.x = f64::NAN,
                3 => model.nodes[1].position = model.nodes[0].position,
                _ => model.pipe_segments[0].to = model.pipe_segments[0].from.clone(),
            }
            let status = inspect_applied_self_weight(&model, None);
            assert_eq!(
                status[0].state,
                AppliedSelfWeightState::Invalid,
                "mode {mode}"
            );
            assert!(status[0]
                .diagnostics
                .iter()
                .any(|d| d.code == "SELF_WEIGHT_INPUT_INVALID"));
        }
    }
    #[test]
    fn applied_self_weight_snapshot_cannot_authenticate_itself() {
        let (mut model, _) = applied();
        let load = &mut model.load_cases[0].primitive_loads[0];
        let mut evidence: Value = serde_json::from_str(load.provenance.as_ref().unwrap()).unwrap();
        load.magnitude.value *= 2.0;
        evidence["generated_payload"]["magnitude"]["value"] = json!(load.magnitude.value);
        evidence["mass_kg_per_m"] = json!(evidence["mass_kg_per_m"].as_f64().unwrap() * 2.0);
        load.provenance = Some(evidence.to_string());
        assert_eq!(state(&model), AppliedSelfWeightState::Invalid);
    }
    #[test]
    fn applied_legacy_without_source_hash_is_inspected_without_fabrication() {
        let (mut model, _) = applied();
        let load = &mut model.load_cases[0].primitive_loads[0];
        let mut evidence: Value = serde_json::from_str(load.provenance.as_ref().unwrap()).unwrap();
        evidence["method"] = json!(LEGACY_METHOD);
        for key in [
            "source_model_hash",
            "generated_payload",
            "normalized_dependencies",
        ] {
            evidence.as_object_mut().unwrap().remove(key);
        }
        load.provenance = Some(evidence.to_string());
        let checked = inspect_applied_self_weight(&model, None);
        assert!(matches!(
            checked[0].state,
            AppliedSelfWeightState::Stale | AppliedSelfWeightState::Modified
        ));
        assert!(checked[0].replacement.is_none());
    }
    #[test]
    fn genuine_v1_requires_method_refresh_and_retains_verbatim_history() {
        let mut model: PreviewModel = serde_json::from_str(include_str!(
            "../../loads/self_weight_wasm/tests/fixtures/applied_self_weight_v1.json"
        ))
        .unwrap();
        let original = model.load_cases[0].primitive_loads[0]
            .provenance
            .clone()
            .unwrap();
        let hash = format!("sha256:{}", "b".repeat(64));
        let checked = inspect_applied_self_weight(&model, Some(&hash));
        assert_eq!(checked[0].state, AppliedSelfWeightState::Stale);
        assert_eq!(
            checked[0].diagnostics[0].code,
            "SELF_WEIGHT_METHOD_REFRESH_REQUIRED"
        );
        let replacement = checked[0].replacement.clone().unwrap();
        let evidence: Value =
            serde_json::from_str(replacement["provenance"].as_str().unwrap()).unwrap();
        assert_eq!(evidence["mass_method"], STABLE_MASS_METHOD);
        assert_eq!(evidence["legacy_generation_provenance"], original);
        model.load_cases[0].primitive_loads[0] = serde_json::from_value(replacement).unwrap();
        assert_eq!(state(&model), AppliedSelfWeightState::Fresh);
        model.pipe_segments[0]
            .section
            .material_density
            .as_mut()
            .unwrap()
            .value *= 2.0;
        let checked = inspect_applied_self_weight(&model, Some(&hash));
        let evidence: Value = serde_json::from_str(
            checked[0].replacement.as_ref().unwrap()["provenance"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(evidence["legacy_generation_provenance"], original);
    }
    #[test]
    fn evidence_mass_inputs_with_an_undeclared_member_is_invalid_not_fresh() {
        // The generated record's mass inputs rebuild a closed section: a member it
        // does not declare (a misspelled input, or an extra key in a quantity) is
        // not ignored.
        let edits: [fn(&mut Value); 3] = [
            |e| e["mass_inputs"]["mill_tolerence"] = json!({"value": 0.001, "unit": "m"}),
            |e| e["mass_inputs"]["outside_diameter"]["extra"] = json!(1),
            |e| e["mass_inputs"]["section_ref"] = json!("section:S"),
        ];
        for edit in edits {
            let (mut model, _) = applied();
            assert_eq!(state(&model), AppliedSelfWeightState::Fresh);
            let load = &mut model.load_cases[0].primitive_loads[0];
            let mut evidence: Value = serde_json::from_str(load.provenance.as_ref().unwrap()).unwrap();
            edit(&mut evidence);
            load.provenance = Some(evidence.to_string());
            assert_eq!(state(&model), AppliedSelfWeightState::Invalid, "{evidence}");
        }
    }
    #[test]
    fn stable_method_tag_is_mandatory_and_not_inferred() {
        for tag in [Value::Null, json!("unknown/v1")] {
            let (mut model, _) = applied();
            let load = &mut model.load_cases[0].primitive_loads[0];
            let mut evidence: Value =
                serde_json::from_str(load.provenance.as_ref().unwrap()).unwrap();
            if tag.is_null() {
                evidence.as_object_mut().unwrap().remove("mass_method");
            } else {
                evidence["mass_method"] = tag;
            }
            load.provenance = Some(evidence.to_string());
            assert_eq!(state(&model), AppliedSelfWeightState::Invalid);
        }
    }
    #[test]
    fn stable_source_areas_preserve_thin_wall_and_insulation() {
        let (mut model, mut request) = fixture();
        request.gravity.value = -1.0;
        let section = &mut model.pipe_segments[0].section;
        section.outside_diameter.value = 1.0;
        section.material_density.as_mut().unwrap().value = 1.0;
        for wall in [1e-9, 1e-12, 2.0_f64.powi(-55)] {
            model.pipe_segments[0].section.wall_thickness.value = wall;
            let q = intensity(&model, &request);
            assert!(q < 0.0);
            assert!((q.abs() / (std::f64::consts::PI * wall * (1.0 - wall)) - 1.0).abs() < 1e-12);
        }
        model.pipe_segments[0].section.wall_thickness.value = 0.25;
        let baseline = intensity(&model, &request);
        let thickness = 2.0_f64.powi(-55);
        model.pipe_segments[0].section.insulation_thickness = Some(Quantity {
            value: thickness,
            unit: "m".into(),
        });
        model.pipe_segments[0].section.insulation_density = Some(Quantity {
            value: 1e15,
            unit: "kg/m^3".into(),
        });
        assert!(intensity(&model, &request) < baseline - 0.08);
    }
    #[test]
    fn stable_mass_ranges_fail_but_actual_optional_zero_is_valid() {
        for mode in 0..5 {
            let (mut model, mut request) = fixture();
            let s = &mut model.pipe_segments[0].section;
            match mode {
                0 => {
                    s.outside_diameter.value = 1e-200;
                    s.wall_thickness.value = 1e-201;
                }
                1 => {
                    s.material_density.as_mut().unwrap().value = f64::from_bits(1);
                }
                2 => {
                    s.material_density.as_mut().unwrap().value = f64::MAX;
                    s.outside_diameter.value = 10.0;
                    s.wall_thickness.value = 1.0;
                }
                3 => {
                    request.gravity.value = f64::from_bits(1);
                    s.material_density.as_mut().unwrap().value = 1.0;
                }
                _ => {
                    request.gravity.value = f64::MAX;
                }
            }
            let diagnostics = generate_self_weight_operations(&model, &request).unwrap_err();
            assert!(
                diagnostics
                    .iter()
                    .any(|d| d.code == "SELF_WEIGHT_RANGE_INVALID"),
                "mode {mode}: {diagnostics:?}"
            );
        }
        let (mut model, request) = fixture();
        let s = &mut model.pipe_segments[0].section;
        s.wall_thickness.value = 0.05;
        s.contents_density = Some(Quantity {
            value: 1000.0,
            unit: "kg/m^3".into(),
        });
        s.insulation_thickness = Some(Quantity {
            value: 0.0,
            unit: "m".into(),
        });
        s.insulation_density = Some(Quantity {
            value: 1000.0,
            unit: "kg/m^3".into(),
        });
        assert!(generate_self_weight_operations(&model, &request).is_ok());
        model.pipe_segments[0].section.wall_thickness.value = 0.06;
        assert!(generate_self_weight_operations(&model, &request)
            .unwrap_err()
            .iter()
            .any(|d| d.code == "SELF_WEIGHT_INPUT_INVALID"));
    }
    #[test]
    fn self_weight_rejects_unknown_request_options() {
        let (_, request) = fixture();
        let mut top = serde_json::to_value(&request).unwrap();
        top["component_weights"] = json!(true);
        assert!(serde_json::from_value::<SelfWeightRequest>(top).is_err());
        let mut gravity = serde_json::to_value(&request).unwrap();
        gravity["gravity"]["default_gravity"] = json!(true);
        assert!(serde_json::from_value::<SelfWeightRequest>(gravity).is_err());
        assert!(serde_json::from_value::<SelfWeightRequest>(
            serde_json::to_value(&request).unwrap()
        )
        .is_ok());
    }

    #[test]
    fn self_weight_signed_axes_shape_and_immutable_inputs() {
        let (m, mut r) = fixture();
        let before = format!("{m:?}");
        for axis in ["global_x", "global_y", "global_z"] {
            for g in [-7.0, 7.0] {
                r.gravity.axis = axis.into();
                r.gravity.value = g;
                close(
                    intensity(&m, &r),
                    std::f64::consts::PI / 4.0 * (0.1_f64.powi(2) - 0.08_f64.powi(2)) * 1000.0 * g,
                );
            }
        }
        let request_before = serde_json::to_value(&r).unwrap();
        let p = generate_self_weight_operations(&m, &r).unwrap();
        assert_eq!(p.source_model_hash, r.source_model_hash);
        assert_eq!(p.scope_label, "selected_pipe_mass_only");
        assert_eq!(p.changes.len(), 2);
        for c in &p.changes {
            assert_eq!(c.object_type, "Load");
            assert_eq!(c.target_ref, r.case_id);
            assert_eq!(c.operation_kind, "create");
            assert_eq!(c.before, "not_present");
        }
        let case: Value = serde_json::from_str(&p.changes[0].after).unwrap();
        assert_eq!(case["status"], "draft");
        assert_eq!(case["primitive_loads"], json!([]));
        let load: Value = serde_json::from_str(&p.changes[1].after).unwrap();
        assert_eq!(load["category"], "distributed_force");
        assert_eq!(load["direction"], r.gravity.axis);
        let e: Value = serde_json::from_str(load["provenance"].as_str().unwrap()).unwrap();
        assert_eq!(e, p.source_evidence[0]);
        assert_eq!(
            serde_json::from_str::<Value>(&p.changes[1].source_note).unwrap(),
            e
        );
        assert_eq!(e["contents_absent"], true);
        assert_eq!(e["insulation_absent"], true);
        assert_eq!(e["mass_inputs"]["contents_density"], Value::Null);
        assert_eq!(format!("{m:?}"), before);
        assert_eq!(serde_json::to_value(&r).unwrap(), request_before);
    }
    #[test]
    fn self_weight_alternate_units_and_all_mass_sources() {
        let (mut m, r) = fixture();
        let baseline = intensity(&m, &r);
        m.pipe_segments[0].section.outside_diameter = Quantity {
            value: 100.0,
            unit: "mm".into(),
        };
        m.pipe_segments[0].section.wall_thickness = Quantity {
            value: 10.0,
            unit: "mm".into(),
        };
        close(intensity(&m, &r), baseline);
        m.pipe_segments[0].section.material_density = Some(Quantity {
            value: 1000.0 * 0.0254_f64.powi(3) / 0.45359237,
            unit: "lb/in^3".into(),
        });
        close(intensity(&m, &r), baseline);
        let s = &mut m.pipe_segments[0].section;
        s.mill_tolerance = Some(Quantity {
            value: 1.0,
            unit: "mm".into(),
        });
        s.contents_density = Some(Quantity {
            value: 400.0,
            unit: "kg/m^3".into(),
        });
        s.insulation_thickness = Some(Quantity {
            value: 10.0,
            unit: "mm".into(),
        });
        s.insulation_density = Some(Quantity {
            value: 50.0,
            unit: "kg/m^3".into(),
        });
        let mass = std::f64::consts::PI / 4.0
            * ((0.1_f64.powi(2) - 0.082_f64.powi(2)) * 1000.0
                + 0.082_f64.powi(2) * 400.0
                + (0.12_f64.powi(2) - 0.1_f64.powi(2)) * 50.0);
        close(intensity(&m, &r), mass * r.gravity.value);
        let p = plan_value(&m, &r);
        assert_eq!(p["source_evidence"][0]["contents_absent"], false);
        assert_eq!(p["source_evidence"][0]["insulation_absent"], false);
    }
    #[test]
    fn self_weight_invalid_quantities_fail_without_partial_plan() {
        let (m, r) = fixture();
        for value in [f64::NAN, f64::INFINITY, -1.0, 0.0] {
            let mut bad = m.clone();
            bad.pipe_segments[0]
                .section
                .material_density
                .as_mut()
                .unwrap()
                .value = value;
            assert!(generate_self_weight_operations(&bad, &r).is_err());
        }
        for unit in ["", "N", "unknown"] {
            let mut bad = m.clone();
            bad.pipe_segments[0]
                .section
                .material_density
                .as_mut()
                .unwrap()
                .unit = unit.into();
            assert!(generate_self_weight_operations(&bad, &r).is_err());
        }
        for mode in 0..9 {
            let mut bad = m.clone();
            let s = &mut bad.pipe_segments[0].section;
            match mode {
                0 => s.material_density = None,
                1 => s.wall_thickness.value = 0.06,
                2 => {
                    s.mill_tolerance = Some(Quantity {
                        value: -1.0,
                        unit: "m".into(),
                    })
                }
                3 => {
                    s.mill_tolerance = Some(Quantity {
                        value: 0.01,
                        unit: "m".into(),
                    })
                }
                4 => {
                    s.insulation_density = Some(Quantity {
                        value: 1.0,
                        unit: "kg/m^3".into(),
                    })
                }
                5 => {
                    s.contents_density = Some(Quantity {
                        value: -1.0,
                        unit: "kg/m^3".into(),
                    })
                }
                6 => s.outside_diameter.value = f64::MAX,
                7 => s.wall_thickness.value = f64::NAN,
                _ => {
                    s.insulation_thickness = Some(Quantity {
                        value: -1.0,
                        unit: "m".into(),
                    })
                }
            };
            assert!(
                generate_self_weight_operations(&bad, &r).is_err(),
                "mode {mode}"
            );
        }
        for mode in 0..7 {
            let mut bad = r.clone();
            match mode {
                0 => bad.gravity.value = 0.0,
                1 => bad.gravity.value = f64::INFINITY,
                2 => bad.gravity.unit = "N".into(),
                3 => bad.gravity.axis = "local_x".into(),
                4 => bad.provenance = " ".into(),
                5 => bad.source_model_hash = "A".repeat(64),
                _ => bad.gravity.value = f64::MAX,
            };
            assert!(generate_self_weight_operations(&m, &bad).is_err());
        }
    }
    #[test]
    fn self_weight_selection_geometry_and_collisions() {
        let (m, r) = fixture();
        for mode in 0..10 {
            let mut bad = m.clone();
            match mode {0=>bad.pipe_segments.push(bad.pipe_segments[0].clone()),1=>bad.nodes.push(bad.nodes[0].clone()),2=>bad.nodes.clear(),3=>bad.nodes[1].position=bad.nodes[0].position,4=>bad.nodes[1].position.x=f64::NAN,5=>bad.pipe_segments[0].to="a".into(),6=>bad.pipe_segments[0].provenance=None,7=>bad.pipe_segments.clear(),8=>bad.load_cases=serde_json::from_value(json!([{"id":r.case_id}])).unwrap(),_=>bad.load_cases=serde_json::from_value(json!([{"id":"other","primitive_loads":[{"id":format!("load:self-weight:{}:{}:{}:{}",r.case_id.len(),r.case_id,r.pipe_refs[0].len(),r.pipe_refs[0]),"category":"distributed_force","target":{"type":"element","pipe":"pipe:é"},"direction":"global_y","magnitude":{"value":1,"unit":"N/m"},"dimension":"force_per_length"}]}])).unwrap()};
            assert!(
                generate_self_weight_operations(&bad, &r).is_err(),
                "mode {mode}"
            );
        }
        for refs in [
            vec![],
            vec![" ".into()],
            vec!["missing".into()],
            vec![r.pipe_refs[0].clone(), r.pipe_refs[0].clone()],
        ] {
            let mut bad = r.clone();
            bad.pipe_refs = refs;
            assert!(generate_self_weight_operations(&m, &bad).is_err());
        }
        let mut two = m.clone();
        let mut pipe = two.pipe_segments[0].clone();
        pipe.id = "pipe_é:6".into();
        two.pipe_segments.push(pipe);
        let mut rr = r.clone();
        rr.pipe_refs.push("pipe_é:6".into());
        let first = plan_value(&two, &rr);
        rr.pipe_refs.reverse();
        assert_eq!(first, plan_value(&two, &rr));
        let a: Value =
            serde_json::from_str(first["changes"][1]["after"].as_str().unwrap()).unwrap();
        let b: Value =
            serde_json::from_str(first["changes"][2]["after"].as_str().unwrap()).unwrap();
        assert_ne!(a["id"], b["id"]);
        assert!(a["id"].as_str().unwrap().contains("pipe:é"));
    }
    #[test]
    fn self_weight_section_resolution_and_unselected_isolation() {
        let (mut m, r) = fixture();
        let baseline = intensity(&m, &r);
        m.pipe_segments[0].section_ref = Some("section".into());
        m.sections=serde_json::from_value(json!([{"id":"section","name":"invented","section_type":"pipe","properties":{"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented section"}])).unwrap();
        close(intensity(&m, &r), baseline);
        assert_eq!(
            plan_value(&m, &r)["source_evidence"][0]["referenced_section"]["provenance"],
            "invented section"
        );
        for mode in 0..6 {
            let mut bad = m.clone();
            match mode {
                0 => bad.sections.clear(),
                1 => bad.sections.push(bad.sections[0].clone()),
                2 => bad.sections[0].section_type = "other".into(),
                3 => bad.pipe_segments[0].section.outside_diameter.value = 0.2,
                4 => {
                    bad.pipe_segments[0].section.outside_diameter = Quantity {
                        value: 100.0,
                        unit: "mm".into(),
                    }
                }
                _ => bad.sections[0].provenance = Value::Null,
            };
            assert!(
                generate_self_weight_operations(&bad, &r).is_err(),
                "mode {mode}"
            );
        }
        let mut broken = m.pipe_segments[0].clone();
        broken.id = "unselected".into();
        broken.section_ref = Some("missing".into());
        broken.from = "missing".into();
        m.pipe_segments.push(broken);
        close(intensity(&m, &r), baseline);
    }
}
