//! Pure selected-pipe mass operation drafts. The caller retains and hashes the
//! original document and supplies the operation envelope before atomic apply.
use super::{
    compute_pipe_mass_per_length, normalize_quantity, resolve_shared_sections, Diagnostic,
    PreviewModel, Quantity,
};
use open_pipe_stress_units::Dimension;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;

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
    let mut selected = HashSet::new();
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
    subset.pipe_segments.sort_by(|a, b| a.id.cmp(&b.id));
    resolve_shared_sections(&mut subset, &mut d);
    if !d.is_empty() {
        return Err(d);
    }
    let mut evidence = Vec::new();
    let mut loads = Vec::new();
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
        match &mut section.material_density {
            Some(q) => normalize(q, Dimension::Density, true, "material_density", &mut d),
            None => invalid(&mut d, &pipe.id, "explicit material density required"),
        }
        if !d.is_empty() {
            continue;
        }
        let Some(mass) = compute_pipe_mass_per_length(pipe, &request.case_id, &mut d) else {
            continue;
        };
        let intensity = mass * gravity.value;
        if !mass.is_finite() || mass <= 0.0 || !intensity.is_finite() {
            invalid(&mut d, &pipe.id, "computed mass or intensity is invalid");
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
        e["source_model_hash"] = json!(request.source_model_hash);
        e["normalized_dependencies"] = dependency_projection(&pipe.section, &original.section_ref);
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
    let case = json!({"id":request.case_id,"label":request.label,"kind":"primitive_user_load","status":"draft","provenance":request.provenance,"primitive_loads":[]});
    let mut changes = vec![draft(
        request,
        "create_load_case",
        "load_cases",
        case,
        "none",
        "dimensionless",
        &json!({"scope_label":"selected_pipe_mass_only","source_model_hash":request.source_model_hash,"source_evidence":evidence}),
    )];
    changes.extend(loads);
    Ok(SelfWeightOperationPlan {
        source_model_hash: request.source_model_hash.clone(),
        changes,
        source_evidence: evidence,
        scope_label: "selected_pipe_mass_only".into(),
    })
}

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
// Match the documented JSON transport exactly, without a numerical tolerance.
fn transported_number(value: f64) -> Option<f64> {
    serde_json::from_str::<Value>(&json!(value).to_string())
        .ok()?
        .as_f64()
}
fn valid_hash(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn dependency_projection(s: &super::PipeSectionInput, reference: &Option<String>) -> Value {
    json!({"section_ref":reference,
        "outside_diameter":quantity(&s.outside_diameter),
        "wall_thickness":quantity(&s.wall_thickness),
        "mill_tolerance":optional(&s.mill_tolerance),
        "material_density":optional(&s.material_density),
        "contents_density":optional(&s.contents_density),
        "insulation_thickness":optional(&s.insulation_thickness),
        "insulation_density":optional(&s.insulation_density)})
}
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
        || evidence["normalized_acceleration_m_per_s2"].as_f64()
            != transported_number(acceleration.value)
    {
        return Err(fail());
    }
    let reference: Option<String> =
        serde_json::from_value(evidence.get("section_ref").cloned().ok_or_else(fail)?)
            .map_err(|_| fail())?;
    let inputs = evidence["mass_inputs"].as_object().ok_or_else(fail)?;
    for key in [
        "outside_diameter",
        "wall_thickness",
        "mill_tolerance",
        "material_density",
        "contents_density",
        "insulation_thickness",
        "insulation_density",
    ] {
        if !inputs.contains_key(key) {
            return Err(fail());
        }
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
    let dependencies = dependency_projection(&section, &reference);
    let pipe = super::PreviewPipe {
        id: pipe_id.into(),
        from: String::new(),
        to: String::new(),
        section,
        section_ref: reference,
        material: String::new(),
        y_reference: None,
        provenance: None,
    };
    let mass = compute_pipe_mass_per_length(&pipe, case_id, &mut d).ok_or_else(fail)?;
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
        let snapshot = &evidence["generated_payload"];
        let mut expected = generated.clone();
        // Recompute independently from retained source quantities; the snapshot
        // is an exact edit detector, not its own physical-value oracle.
        let q: Quantity =
            serde_json::from_value(snapshot["magnitude"].clone()).map_err(|_| fail())?;
        if q.unit != "N/m" || !q.value.is_finite() || q.value == 0.0 {
            return Err(fail());
        }
        if transported_number(mass) != Some(retained_mass)
            || transported_number(mass * acceleration.value) != Some(q.value)
        {
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
                Some(GENERATED_METHOD | LEGACY_METHOD | MANUAL_OVERRIDE_METHOD)
            );
            let claimed_method_family = method.is_some_and(|method| {
                method == "pipe_mass_per_length_times_explicit_axis_acceleration"
                    || method.starts_with("pipe_mass_per_length_times_explicit_axis_acceleration/")
                    || method == "manual_override_of_generated_self_weight"
                    || method.starts_with("manual_override_of_generated_self_weight/")
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
                super::LoadTargetInput::Node { .. } => false,
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
                    retained_basis(&evidence["original_generation_provenance"], &case.id)?;
                    status.state = AppliedSelfWeightState::ManualOverride;
                    return Ok(());
                }
                let (mut request, old_dependencies, old_payload) =
                    retained_basis(evidence, &case.id)?;
                if let Some(hash) = source_model_hash {
                    if !valid_hash(hash) {
                        return Err(fail());
                    }
                    request.source_model_hash = hash.into();
                }
                // Clear only load identity collisions on a clone. Current source
                // quantities and shared-section cache checks stay authoritative.
                let mut source = model.clone();
                source.load_cases.clear();
                let generated = generate_operations(&source, &request, true)?;
                let current_evidence = &generated.source_evidence[0];
                let payload_changed = primitive_payload(load) != old_payload;
                // Legacy provenance did not retain the exact emitted float. An
                // inconsistent old mass is unverifiable, never auto-refreshable.
                let legacy_uncertain = method == Some(LEGACY_METHOD)
                    && evidence["mass_kg_per_m"].as_f64()
                        != old_dependencies_mass(evidence, &case.id);
                status.state = if payload_changed || legacy_uncertain {
                    AppliedSelfWeightState::Modified
                } else if current_evidence["normalized_dependencies"] != old_dependencies {
                    AppliedSelfWeightState::Stale
                } else {
                    AppliedSelfWeightState::Fresh
                };
                if status.state == AppliedSelfWeightState::Stale && source_model_hash.is_some() {
                    status.replacement = Some(
                        serde_json::from_str(&generated.changes[1].after).map_err(|_| fail())?,
                    );
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
    let (_, dependencies, _) = retained_basis(evidence, case_id).ok()?;
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
            AppliedSelfWeightState::Fresh | AppliedSelfWeightState::Modified
        ));
        assert!(checked[0].replacement.is_none());
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
