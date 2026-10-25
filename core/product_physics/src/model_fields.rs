//! Closed model records: an undeclared key on the model root, a load case (with
//! its modulus_basis_temperature and equivalent_static inputs), a primitive load
//! (with its target and magnitude), a combination or a combination term is
//! refused by name before anything is solved, never
//! silently dropped. A misspelled `primitive_loads` would otherwise empty a
//! case, and a misspelled `combinations` would drop every combination. So is one
//! on a node, a pipe segment (with its section and `y_reference`), a section of
//! the library (with its property quantities), a support (with its stiffness,
//! hanger and nonlinear inputs), a component (with its geometry, modifiers,
//! mechanics interface and objective connector), a material of the model or the
//! request (with its temperature points), and every quantity and vector they
//! carry: a misspelled `mill_tolerance` would otherwise solve without it.
//!
//! The records deny unknown fields in their serde derives, so every typed
//! reader refuses. The request boundary first scans the actual request so the
//! refusal names the record (and its id and path) as well as the field. The
//! accepted field sets are read from the serde derives themselves, so the scan
//! and the typed parse cannot disagree.

use crate::{
    ComponentGeometryInput, ComponentMechanicsInterfaceInput, ComponentModifierInput,
    EquivalentStaticGenerationInput, FrictionNormalReactionSourceInput, MaterialRecordFields,
    MaterialTemperaturePointInput, NonlinearSupportInput, PipeSectionInput, PreviewCombination,
    PreviewCombinationTerm, PreviewComponent, PreviewLoadCase, PreviewModel, PreviewNode, PreviewPipe,
    PreviewPrimitiveLoad, PreviewSection, PreviewSupport, Quantity, SeismicGenerationInput,
    SpringHangerInput, SupportStiffnessInput, Vec3, VectorQuantity, WindExposedSpanInput,
    WindGenerationInput,
};
use serde::de::{self, DeserializeOwned, IgnoredAny, Visitor};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Request-boundary refusal code (an `Err` of the value entries).
pub(crate) const UNKNOWN_MODEL_FIELD: &str = "UNKNOWN_MODEL_FIELD";
/// Unknown keys named in one refusal; the rest are counted.
const NAMED_LIMIT: usize = 8;

/// A primitive-load magnitude: exactly `value` and `unit`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClosedQuantity {
    value: f64,
    unit: String,
}

/// An optional closed quantity: absence and a present null stay `None`.
pub(crate) fn closed_optional_quantity<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Quantity>, D::Error> {
    Ok(Option::<ClosedQuantity>::deserialize(deserializer)?.map(|ClosedQuantity { value, unit }| Quantity { value, unit }))
}

pub(crate) fn closed_quantity<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Quantity, D::Error> {
    let ClosedQuantity { value, unit } = ClosedQuantity::deserialize(deserializer)?;
    Ok(Quantity { value, unit })
}

/// An optional closed quantity that, when present, must be an object (as
/// `deserialize_present_object` reads a pipe section's corrosion allowance).
fn closed_present_quantity<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Quantity>, D::Error> {
    Ok(crate::deserialize_present_object::<D, ClosedQuantity>(deserializer)?
        .map(|ClosedQuantity { value, unit }| Quantity { value, unit }))
}

/// A section's property quantities, each closed.
pub(crate) fn closed_quantity_map<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<String, Quantity>, D::Error> {
    Ok(BTreeMap::<String, ClosedQuantity>::deserialize(deserializer)?
        .into_iter()
        .map(|(name, ClosedQuantity { value, unit })| (name, Quantity { value, unit }))
        .collect())
}

/// A position or direction in the global frame: exactly `x`, `y` and `z`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClosedVec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub(crate) fn closed_vec3<'de, D: de::Deserializer<'de>>(deserializer: D) -> Result<Vec3, D::Error> {
    let ClosedVec3 { x, y, z } = ClosedVec3::deserialize(deserializer)?;
    Ok(Vec3 { x, y, z })
}

/// An optional closed vector: absence and a present null stay `None`.
pub(crate) fn closed_optional_vec3<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec3>, D::Error> {
    Ok(Option::<ClosedVec3>::deserialize(deserializer)?.map(|ClosedVec3 { x, y, z }| Vec3 { x, y, z }))
}

/// A vector quantity: exactly `x`, `y`, `z` and `unit`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClosedVectorQuantity {
    x: f64,
    y: f64,
    z: f64,
    unit: String,
}

pub(crate) fn closed_optional_vector_quantity<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<VectorQuantity>, D::Error> {
    Ok(Option::<ClosedVectorQuantity>::deserialize(deserializer)?
        .map(|ClosedVectorQuantity { x, y, z, unit }| VectorQuantity { x, y, z, unit }))
}

/// A pipe segment's section as the model authors it: `PipeSectionInput`'s
/// fields with their defaults, closed, and its quantities closed.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClosedPipeSection {
    #[serde(deserialize_with = "closed_quantity")]
    outside_diameter: Quantity,
    #[serde(deserialize_with = "closed_quantity")]
    wall_thickness: Quantity,
    #[serde(default, deserialize_with = "closed_optional_quantity")]
    mill_tolerance: Option<Quantity>,
    #[serde(default, deserialize_with = "closed_present_quantity")]
    corrosion_allowance: Option<Quantity>,
    #[serde(default, deserialize_with = "closed_optional_quantity")]
    material_density: Option<Quantity>,
    #[serde(default, deserialize_with = "closed_optional_quantity")]
    contents_density: Option<Quantity>,
    #[serde(default, deserialize_with = "closed_optional_quantity")]
    insulation_thickness: Option<Quantity>,
    #[serde(default, deserialize_with = "closed_optional_quantity")]
    insulation_density: Option<Quantity>,
}

pub(crate) fn closed_pipe_section<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<PipeSectionInput, D::Error> {
    let ClosedPipeSection {
        outside_diameter,
        wall_thickness,
        mill_tolerance,
        corrosion_allowance,
        material_density,
        contents_density,
        insulation_thickness,
        insulation_density,
    } = ClosedPipeSection::deserialize(deserializer)?;
    Ok(PipeSectionInput {
        outside_diameter,
        wall_thickness,
        mill_tolerance,
        corrosion_allowance,
        material_density,
        contents_density,
        insulation_thickness,
        insulation_density,
    })
}

/// The members of objective connector 1.0.0 (`ObjectiveConnector` in
/// `schemas/model.schema.yaml`). The connector is read by key, so its own
/// members are checked here; its member records are the connector decoder's.
const OBJECTIVE_CONNECTOR_FIELDS: &[&str] = &[
    "version",
    "motion_basis",
    "end_i",
    "end_j",
    "connector_axes_global",
    "installed_reference_temperature",
    "temperature_applicability",
    "reference_state",
    "q_ref",
    "stiffness",
    "topology",
    "calibration",
    "hardware",
    "pressure_model",
    "provenance",
];

/// A component's objective connector, kept as authored, with no undeclared
/// member. A value that is not an object is left to the connector decoder.
pub(crate) fn closed_connector<'de, D: de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Value>, D::Error> {
    let connector = Option::<Value>::deserialize(deserializer)?;
    if let Some(Value::Object(members)) = &connector {
        if let Some(key) = members.keys().find(|key| !OBJECTIVE_CONNECTOR_FIELDS.contains(&key.as_str())) {
            return Err(de::Error::unknown_field(key, OBJECTIVE_CONNECTOR_FIELDS));
        }
    }
    Ok(connector)
}

/// The field names a serde-derived struct accepts, read from its derive.
fn declared_fields<T: DeserializeOwned>() -> &'static [&'static str] {
    struct Probe<'a>(&'a mut &'static [&'static str]);
    impl<'de> de::Deserializer<'de> for Probe<'_> {
        type Error = de::value::Error;
        fn deserialize_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
            Err(de::Error::custom("field probe"))
        }
        fn deserialize_struct<V: Visitor<'de>>(
            self,
            _: &'static str,
            fields: &'static [&'static str],
            _: V,
        ) -> Result<V::Value, Self::Error> {
            *self.0 = fields;
            Err(de::Error::custom("field probe"))
        }
        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map enum identifier ignored_any
        }
    }
    let mut fields: &'static [&'static str] = &[];
    let _ = T::deserialize(Probe(&mut fields));
    fields
}

/// `LoadTargetInput` is internally tagged; its variant fields per `type`.
fn target_fields(kind: &str) -> Option<&'static [&'static str]> {
    match kind {
        "node" => Some(&["type", "node"]),
        "element" => Some(&["type", "pipe"]),
        _ => None,
    }
}

pub(crate) struct RecordFields {
    pub(crate) model: &'static [&'static str],
    pub(crate) load_case: &'static [&'static str],
    pub(crate) primitive_load: &'static [&'static str],
    pub(crate) magnitude: &'static [&'static str],
    pub(crate) combination: &'static [&'static str],
    pub(crate) term: &'static [&'static str],
    pub(crate) equivalent_static: &'static [&'static str],
    pub(crate) seismic: &'static [&'static str],
    pub(crate) wind: &'static [&'static str],
    pub(crate) exposed_span: &'static [&'static str],
    pub(crate) node: &'static [&'static str],
    pub(crate) vector: &'static [&'static str],
    pub(crate) pipe: &'static [&'static str],
    pub(crate) pipe_section: &'static [&'static str],
    pub(crate) section: &'static [&'static str],
    pub(crate) support: &'static [&'static str],
    pub(crate) stiffness: &'static [&'static str],
    pub(crate) hanger: &'static [&'static str],
    pub(crate) nonlinear: &'static [&'static str],
    pub(crate) reaction_source: &'static [&'static str],
    pub(crate) component: &'static [&'static str],
    pub(crate) geometry: &'static [&'static str],
    pub(crate) vector_quantity: &'static [&'static str],
    pub(crate) modifiers: &'static [&'static str],
    pub(crate) mechanics_interface: &'static [&'static str],
    pub(crate) material: &'static [&'static str],
    pub(crate) temperature_point: &'static [&'static str],
}

pub(crate) fn record_fields() -> RecordFields {
    RecordFields {
        model: declared_fields::<PreviewModel>(),
        load_case: declared_fields::<PreviewLoadCase>(),
        primitive_load: declared_fields::<PreviewPrimitiveLoad>(),
        magnitude: declared_fields::<ClosedQuantity>(),
        combination: declared_fields::<PreviewCombination>(),
        term: declared_fields::<PreviewCombinationTerm>(),
        equivalent_static: declared_fields::<EquivalentStaticGenerationInput>(),
        seismic: declared_fields::<SeismicGenerationInput>(),
        wind: declared_fields::<WindGenerationInput>(),
        exposed_span: declared_fields::<WindExposedSpanInput>(),
        node: declared_fields::<PreviewNode>(),
        vector: declared_fields::<ClosedVec3>(),
        pipe: declared_fields::<PreviewPipe>(),
        pipe_section: declared_fields::<ClosedPipeSection>(),
        section: declared_fields::<PreviewSection>(),
        support: declared_fields::<PreviewSupport>(),
        stiffness: declared_fields::<SupportStiffnessInput>(),
        hanger: declared_fields::<SpringHangerInput>(),
        nonlinear: declared_fields::<NonlinearSupportInput>(),
        reaction_source: declared_fields::<FrictionNormalReactionSourceInput>(),
        component: declared_fields::<PreviewComponent>(),
        geometry: declared_fields::<ComponentGeometryInput>(),
        vector_quantity: declared_fields::<ClosedVectorQuantity>(),
        modifiers: declared_fields::<ComponentModifierInput>(),
        mechanics_interface: declared_fields::<ComponentMechanicsInterfaceInput>(),
        material: declared_fields::<MaterialRecordFields<IgnoredAny>>(),
        temperature_point: declared_fields::<MaterialTemperaturePointInput>(),
    }
}

/// The quantity members of the nested closed records, each a closed quantity.
const PIPE_SECTION_QUANTITIES: [&str; 8] = [
    "outside_diameter",
    "wall_thickness",
    "mill_tolerance",
    "corrosion_allowance",
    "material_density",
    "contents_density",
    "insulation_thickness",
    "insulation_density",
];
const HANGER_QUANTITIES: [&str; 6] =
    ["installed_load", "cold_load", "hot_load", "constant_load", "travel_range", "movement_limit"];
const NONLINEAR_QUANTITIES: [&str; 3] = ["gap", "friction_coefficient", "normal_reaction"];
const GEOMETRY_QUANTITIES: [&str; 12] = [
    "bend_radius",
    "bend_angle",
    "branch_run_size",
    "branch_header_size",
    "branch_connection_angle",
    "branch_reinforcement_area",
    "rigid_body_length",
    "end_a_size",
    "end_b_size",
    "weight",
    "effective_area",
    "movement_limit",
];
const MODIFIER_QUANTITIES: [&str; 13] = [
    "sif_user_value",
    "branch_header_sif_user_value",
    "branch_branch_sif_user_value",
    "sif_in_plane_user_value",
    "sif_out_of_plane_user_value",
    "flexibility_factor_user_value",
    "stiffness_scaling_user_value",
    "linear_stiffness_user_value",
    "rotational_stiffness_user_value",
    "axial_stiffness_user_value",
    "lateral_stiffness_user_value",
    "angular_stiffness_user_value",
    "torsional_stiffness_user_value",
];
const MATERIAL_QUANTITIES: [&str; 4] =
    ["poisson_ratio", "elastic_modulus", "shear_modulus", "thermal_expansion_coefficient"];
const POINT_QUANTITIES: [&str; 5] =
    ["poisson_ratio", "temperature", "elastic_modulus", "shear_modulus", "thermal_expansion_coefficient"];

struct Finding {
    field: String,
    record: String,
    path: String,
}

fn id_of(record: &serde_json::Map<String, Value>) -> Option<&str> {
    record.get("id").and_then(Value::as_str)
}

fn check(
    record: &serde_json::Map<String, Value>,
    accepted: &[&str],
    name: impl Fn() -> String,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    for key in record.keys() {
        if !accepted.contains(&key.as_str()) {
            findings.push(Finding { field: key.clone(), record: name(), path: path.to_string() });
        }
    }
}

fn objects<'a>(
    parent: &'a serde_json::Map<String, Value>,
    key: &str,
) -> impl Iterator<Item = (usize, &'a serde_json::Map<String, Value>)> {
    parent
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(index, value)| value.as_object().map(|object| (index, object)))
}

fn named(kind: &str, id: Option<&str>, index: usize) -> String {
    match id {
        Some(id) => format!("{kind} `{id}`"),
        None => format!("{kind} {index}"),
    }
}

/// Where a nested record sits. Its name and path are formed only for a finding
/// (`check_nested`), so the scan of a request without one forms neither.
#[derive(Clone, Copy)]
enum At<'a> {
    /// `{collection}[{index}]`, named `{kind} `{id}``.
    Record { collection: &'static str, kind: &'static str, index: usize, id: Option<&'a str> },
    /// `{parent}.{key}`, named `the {key} of {parent}`.
    Member { parent: &'a At<'a>, key: &'a str },
    /// `{parent}.{key}[{index}]`, named `{kind} `{id}` of {parent}`.
    Entry { parent: &'a At<'a>, key: &'static str, kind: &'static str, index: usize, id: Option<&'a str> },
}

impl<'a> At<'a> {
    fn in_collection(collection: &'static str, kind: &'static str, index: usize, record: &'a serde_json::Map<String, Value>) -> Self {
        At::Record { collection, kind, index, id: id_of(record) }
    }

    fn finding_name(&self) -> String {
        match *self {
            At::Record { kind, index, id, .. } => named(kind, id, index),
            At::Member { parent, key } => format!("the {key} of {}", parent.finding_name()),
            At::Entry { parent, kind, index, id, .. } => format!("{} of {}", named(kind, id, index), parent.finding_name()),
        }
    }

    fn finding_path(&self) -> String {
        match *self {
            At::Record { collection, index, .. } => format!("{collection}[{index}]"),
            At::Member { parent, key } => format!("{}.{key}", parent.finding_path()),
            At::Entry { parent, key, index, .. } => format!("{}.{key}[{index}]", parent.finding_path()),
        }
    }
}

/// `check` for a nested record, which is named and placed only for a finding.
fn check_nested(record: &serde_json::Map<String, Value>, accepted: &[&str], at: &At<'_>, findings: &mut Vec<Finding>) {
    for key in record.keys() {
        if !accepted.contains(&key.as_str()) {
            findings.push(Finding { field: key.clone(), record: at.finding_name(), path: at.finding_path() });
        }
    }
}

/// The member record `key` of `record`, checked when it is an object, with
/// where it sits.
fn nested_member<'a>(
    record: &'a serde_json::Map<String, Value>,
    key: &'a str,
    accepted: &[&str],
    parent: &'a At<'a>,
    findings: &mut Vec<Finding>,
) -> Option<(&'a serde_json::Map<String, Value>, At<'a>)> {
    let object = record.get(key)?.as_object()?;
    let at = At::Member { parent, key };
    check_nested(object, accepted, &at, findings);
    Some((object, at))
}

/// The closed quantities `keys` of `record`.
fn nested_quantities(
    record: &serde_json::Map<String, Value>,
    keys: &[&'static str],
    accepted: &[&str],
    parent: &At<'_>,
    findings: &mut Vec<Finding>,
) {
    for key in keys {
        nested_member(record, key, accepted, parent, findings);
    }
}

/// A support stiffness record and its value.
fn nested_stiffness(record: &serde_json::Map<String, Value>, fields: &RecordFields, parent: &At<'_>, findings: &mut Vec<Finding>) {
    if let Some((stiffness, at)) = nested_member(record, "stiffness", fields.stiffness, parent, findings) {
        nested_member(stiffness, "value", fields.magnitude, &at, findings);
    }
}

/// The model's nested closed records, and the request's materials.
fn nested_records(
    request: &serde_json::Map<String, Value>,
    model: &serde_json::Map<String, Value>,
    fields: &RecordFields,
    findings: &mut Vec<Finding>,
) {
    for (i, node) in objects(model, "nodes") {
        let at = At::in_collection("model.nodes", "node", i, node);
        check_nested(node, fields.node, &at, findings);
        nested_member(node, "position", fields.vector, &at, findings);
    }
    for (i, pipe) in objects(model, "pipe_segments") {
        let at = At::in_collection("model.pipe_segments", "pipe segment", i, pipe);
        check_nested(pipe, fields.pipe, &at, findings);
        nested_member(pipe, "y_reference", fields.vector, &at, findings);
        if let Some((section, at)) = nested_member(pipe, "section", fields.pipe_section, &at, findings) {
            nested_quantities(section, &PIPE_SECTION_QUANTITIES, fields.magnitude, &at, findings);
        }
    }
    for (i, section) in objects(model, "sections") {
        let at = At::in_collection("model.sections", "section", i, section);
        check_nested(section, fields.section, &at, findings);
        if let Some(properties) = section.get("properties").and_then(Value::as_object) {
            let at = At::Member { parent: &at, key: "properties" };
            for (name, quantity) in properties {
                if let Some(quantity) = quantity.as_object() {
                    check_nested(quantity, fields.magnitude, &At::Member { parent: &at, key: name }, findings);
                }
            }
        }
    }
    for (i, support) in objects(model, "supports") {
        let at = At::in_collection("model.supports", "support", i, support);
        check_nested(support, fields.support, &at, findings);
        nested_stiffness(support, fields, &at, findings);
        if let Some((hanger, at)) = nested_member(support, "hanger", fields.hanger, &at, findings) {
            nested_stiffness(hanger, fields, &at, findings);
            nested_quantities(hanger, &HANGER_QUANTITIES, fields.magnitude, &at, findings);
        }
        if let Some((nonlinear, at)) = nested_member(support, "nonlinear", fields.nonlinear, &at, findings) {
            nested_member(nonlinear, "normal_reaction_source", fields.reaction_source, &at, findings);
            nested_quantities(nonlinear, &NONLINEAR_QUANTITIES, fields.magnitude, &at, findings);
        }
    }
    for (i, component) in objects(model, "components") {
        let at = At::in_collection("model.components", "component", i, component);
        check_nested(component, fields.component, &at, findings);
        nested_member(component, "objective_connector", OBJECTIVE_CONNECTOR_FIELDS, &at, findings);
        if let Some((geometry, at)) = nested_member(component, "geometry", fields.geometry, &at, findings) {
            nested_member(geometry, "center_of_gravity", fields.vector_quantity, &at, findings);
            nested_quantities(geometry, &GEOMETRY_QUANTITIES, fields.magnitude, &at, findings);
        }
        if let Some((modifiers, at)) = nested_member(component, "modifiers", fields.modifiers, &at, findings) {
            nested_quantities(modifiers, &MODIFIER_QUANTITIES, fields.magnitude, &at, findings);
        }
        nested_member(component, "mechanics_interface", fields.mechanics_interface, &at, findings);
    }
    for (owner, collection, kind) in [(model, "model.materials", "material"), (request, "materials", "request material")] {
        for (i, material) in objects(owner, "materials") {
            let at = At::in_collection(collection, kind, i, material);
            check_nested(material, fields.material, &at, findings);
            nested_quantities(material, &MATERIAL_QUANTITIES, fields.magnitude, &at, findings);
            for (k, point) in objects(material, "temperature_points") {
                let at = At::Entry { parent: &at, key: "temperature_points", kind: "temperature point", index: k, id: id_of(point) };
                check_nested(point, fields.temperature_point, &at, findings);
                nested_quantities(point, &POINT_QUANTITIES, fields.magnitude, &at, findings);
            }
        }
    }
}

/// The named refusal for undeclared keys on the closed records of the actual
/// request's model, or `None`. Shapes other than unknown keys (a missing or
/// mistyped member, a non-object record) are left to the typed parse.
pub(crate) fn unknown_field_refusal(actual_request: &Value) -> Option<String> {
    let request = actual_request.as_object()?;
    let model = request.get("model")?.as_object()?;
    let fields = record_fields();
    let mut findings = Vec::new();
    check(model, fields.model, || "the model".into(), "model", &mut findings);
    for (i, case) in objects(model, "load_cases") {
        let case_name = named("load case", id_of(case), i);
        let case_path = format!("model.load_cases[{i}]");
        check(case, fields.load_case, || case_name.clone(), &case_path, &mut findings);
        if let Some(temperature) = case.get("modulus_basis_temperature").and_then(Value::as_object) {
            check(temperature, fields.magnitude, || format!("the modulus_basis_temperature of {case_name}"), &format!("{case_path}.modulus_basis_temperature"), &mut findings);
        }
        if let Some(generation) = case.get("equivalent_static").and_then(Value::as_object) {
            let generation_path = format!("{case_path}.equivalent_static");
            check(generation, fields.equivalent_static, || format!("the equivalent_static of {case_name}"), &generation_path, &mut findings);
            if let Some(seismic) = generation.get("seismic").and_then(Value::as_object) {
                check(seismic, fields.seismic, || format!("the seismic input of {case_name}"), &format!("{generation_path}.seismic"), &mut findings);
            }
            if let Some(wind) = generation.get("wind").and_then(Value::as_object) {
                check(wind, fields.wind, || format!("the wind input of {case_name}"), &format!("{generation_path}.wind"), &mut findings);
                for (k, span) in objects(wind, "exposed_spans") {
                    check(span, fields.exposed_span, || format!("exposed span {k} of {case_name}"), &format!("{generation_path}.wind.exposed_spans[{k}]"), &mut findings);
                }
            }
        }
        for (j, load) in objects(case, "primitive_loads") {
            let load_name = format!("{} in {case_name}", named("primitive load", id_of(load), j));
            let load_path = format!("{case_path}.primitive_loads[{j}]");
            check(load, fields.primitive_load, || load_name.clone(), &load_path, &mut findings);
            if let Some(target) = load.get("target").and_then(Value::as_object) {
                if let Some(accepted) =
                    target.get("type").and_then(Value::as_str).and_then(target_fields)
                {
                    check(target, accepted, || format!("the target of {load_name}"), &format!("{load_path}.target"), &mut findings);
                }
            }
            if let Some(magnitude) = load.get("magnitude").and_then(Value::as_object) {
                check(magnitude, fields.magnitude, || format!("the magnitude of {load_name}"), &format!("{load_path}.magnitude"), &mut findings);
            }
        }
    }
    for (i, combination) in objects(model, "combinations") {
        let combination_name = named("combination", id_of(combination), i);
        let combination_path = format!("model.combinations[{i}]");
        check(combination, fields.combination, || combination_name.clone(), &combination_path, &mut findings);
        for (j, term) in objects(combination, "terms") {
            check(term, fields.term, || format!("term {j} of {combination_name}"), &format!("{combination_path}.terms[{j}]"), &mut findings);
        }
    }
    nested_records(request, model, &fields, &mut findings);
    if findings.is_empty() {
        return None;
    }
    let mut named_findings = findings
        .iter()
        .take(NAMED_LIMIT)
        .map(|f| format!("unknown field `{}` in {} ({})", f.field, f.record, f.path))
        .collect::<Vec<_>>()
        .join("; ");
    if findings.len() > NAMED_LIMIT {
        named_findings.push_str(&format!("; and {} more", findings.len() - NAMED_LIMIT));
    }
    Some(format!(
        "{UNKNOWN_MODEL_FIELD}: request: {named_findings}. The model document does not define these keys, so they would be ignored; nothing is solved. Correct or remove each key."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn set(keys: &[&str]) -> BTreeSet<String> {
        keys.iter().map(|k| k.to_string()).collect()
    }

    /// The committed model document schema (`schemas/model.schema.yaml`, JSON
    /// syntax; model document 0.7.0) is the authoritative key list of each
    /// closed record.
    const MODEL_SCHEMA: &str = include_str!("../../../schemas/model.schema.yaml");

    fn schema() -> serde_json::Value {
        serde_json::from_str(MODEL_SCHEMA).expect("model.schema.yaml is JSON")
    }

    fn keys(record: &serde_json::Value, what: &str) -> BTreeSet<String> {
        assert_eq!(record["additionalProperties"], serde_json::json!(false), "{what} is a closed schema record");
        let keys: BTreeSet<String> = record["properties"].as_object().unwrap_or_else(|| panic!("{what} properties")).keys().cloned().collect();
        assert!(!keys.is_empty(), "{what}");
        keys
    }

    fn def(schema: &serde_json::Value, name: &str) -> BTreeSet<String> {
        keys(&schema["$defs"][name], name)
    }

    /// The members model document 0.7.0 adds to the closed records (DEL-02-01
    /// Design `model-document-0.7.0.md` §2, §3). `model_settings` is typed
    /// (`model_settings.rs`); every other one is declared and ignored.
    const ROOT_0_7_0: [&str; 6] =
        ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs", "model_settings"];
    const ANNOTATIONS_0_7_0: [&str; 2] = ["tags", "notes"];

    /// The nested records' members the solve never reads (declared and ignored).
    fn declared_ignored(fields: &RecordFields) -> [(&'static str, &'static [&'static str], &'static [&'static str]); 6] {
        [
            ("node", fields.node, &["label", "tags", "notes"]),
            ("pipe segment", fields.pipe, &["label", "tags", "notes"]),
            ("section", fields.section, &["tags", "notes", "provenance_record"]),
            ("support", fields.support, &["label", "boundary_association", "properties", "tags", "notes", "provenance_record"]),
            ("component", fields.component, &["completeness", "tags", "notes", "provenance_record"]),
            ("material", fields.material, &["label", "tags", "notes", "provenance_record"]),
        ]
    }

    #[test]
    fn the_key_lists_are_model_document_0_7_0s() {
        let schema = schema();
        assert_eq!(schema["properties"]["schema_version"]["enum"], serde_json::json!([crate::MODEL_SCHEMA_VERSION]));
        assert_eq!(crate::MODEL_SCHEMA_VERSION, "0.7.0");
        let fields = record_fields();
        for key in ROOT_0_7_0 {
            assert!(fields.model.contains(&key), "model root declares {key}");
        }
        for key in ANNOTATIONS_0_7_0 {
            assert!(fields.load_case.contains(&key), "load case declares {key}");
            assert!(fields.combination.contains(&key), "combination declares {key}");
            for (record, declared) in [("primitive load", fields.primitive_load), ("combination term", fields.term)] {
                assert!(!declared.contains(&key), "{record} does not declare {key}");
            }
        }
        for (record, declared, ignored) in declared_ignored(&fields) {
            for key in ignored {
                assert!(declared.contains(key), "{record} declares {key}");
            }
        }
    }

    /// The members of a schema record that are `$ref`s to `target`, or to
    /// `target` or null (`anyOf`).
    fn refs(record: &serde_json::Value, target: &str) -> BTreeSet<String> {
        let target = serde_json::json!(format!("#/$defs/{target}"));
        record["properties"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_, member)| {
                member["$ref"] == target
                    || member["anyOf"].as_array().is_some_and(|arms| {
                        arms.len() == 2 && arms.iter().any(|arm| arm["$ref"] == target) && arms.iter().any(|arm| arm["type"] == "null")
                    })
            })
            .map(|(key, _)| key.clone())
            .collect()
    }

    #[test]
    fn nested_records_match_the_committed_schema_key_lists() {
        let schema = schema();
        let d = &schema["$defs"];
        let fields = record_fields();
        for (record, declared, name) in [
            ("node", fields.node, "Node"),
            ("pipe segment", fields.pipe, "PipeSegment"),
            ("pipe section", fields.pipe_section, "PipeSection"),
            ("section", fields.section, "Section"),
            ("support", fields.support, "Support"),
            ("support stiffness", fields.stiffness, "SupportStiffness"),
            ("hanger", fields.hanger, "SpringHanger"),
            ("nonlinear support", fields.nonlinear, "NonlinearSupport"),
            ("component", fields.component, "Component"),
            ("component geometry", fields.geometry, "ComponentGeometry"),
            ("component modifiers", fields.modifiers, "ComponentModifiers"),
            ("mechanics interface", fields.mechanics_interface, "ComponentMechanicsInterface"),
            ("material", fields.material, "Material"),
            ("temperature point", fields.temperature_point, "MaterialTemperaturePoint"),
            ("vector", fields.vector, "Vector3"),
            ("vector quantity", fields.vector_quantity, "VectorQuantity"),
            ("objective connector", OBJECTIVE_CONNECTOR_FIELDS, "ObjectiveConnector"),
        ] {
            assert_eq!(set(declared), def(&schema, name), "{record}");
        }
        assert_eq!(
            set(fields.reaction_source),
            keys(&d["NonlinearSupport"]["properties"]["normal_reaction_source"], "normal reaction source"),
            "normal reaction source"
        );
        // The closed pipe section accepts exactly `PipeSectionInput`'s fields.
        assert_eq!(set(fields.pipe_section), set(declared_fields::<PipeSectionInput>()), "the closed and the evidence section agree");
        // The scan checks every quantity and vector member of the records it walks.
        for (record, scanned, name) in [
            ("pipe section", &PIPE_SECTION_QUANTITIES[..], "PipeSection"),
            ("hanger", &HANGER_QUANTITIES[..], "SpringHanger"),
            ("nonlinear support", &NONLINEAR_QUANTITIES[..], "NonlinearSupport"),
            ("component geometry", &GEOMETRY_QUANTITIES[..], "ComponentGeometry"),
            ("component modifiers", &MODIFIER_QUANTITIES[..], "ComponentModifiers"),
            ("material", &MATERIAL_QUANTITIES[..], "Material"),
            ("temperature point", &POINT_QUANTITIES[..], "MaterialTemperaturePoint"),
        ] {
            assert_eq!(set(scanned), refs(&d[name], "Quantity"), "{record} quantities");
        }
        for name in ["Node", "PipeSegment", "Section", "Support", "SupportStiffness", "Component", "ComponentMechanicsInterface"] {
            let quantities = refs(&d[name], "Quantity");
            let expected: BTreeSet<String> = if name == "SupportStiffness" { set(&["value"]) } else { BTreeSet::new() };
            assert_eq!(quantities, expected, "{name}'s quantities are the ones the scan walks");
        }
        assert_eq!(refs(&d["Node"], "Vector3"), set(&["position"]));
        assert_eq!(refs(&d["PipeSegment"], "Vector3"), set(&["y_reference"]));
        assert_eq!(refs(&d["ComponentGeometry"], "VectorQuantity"), set(&["center_of_gravity"]));
        assert_eq!(refs(&d["SpringHanger"], "SupportStiffness"), set(&["stiffness"]));
        assert_eq!(refs(&d["Support"], "SupportStiffness"), set(&["stiffness"]));
        assert_eq!(d["Section"]["properties"]["properties"]["additionalProperties"]["$ref"], "#/$defs/Quantity");
    }

    #[test]
    fn the_typed_connector_refuses_what_the_scan_names() {
        let connector = |extra: Option<&str>| {
            let mut members: serde_json::Map<String, Value> =
                OBJECTIVE_CONNECTOR_FIELDS.iter().map(|key| (key.to_string(), Value::Null)).collect();
            if let Some(extra) = extra {
                members.insert(extra.into(), Value::Null);
            }
            serde_json::json!({"id": "c", "kind": "expansion_joint", "node": "n", "objective_connector": members})
        };
        assert!(serde_json::from_value::<PreviewComponent>(connector(None)).is_ok());
        let error = serde_json::from_value::<PreviewComponent>(connector(Some("stifness"))).unwrap_err().to_string();
        assert!(error.contains("unknown field `stifness`"), "{error}");
        // A connector that is not an object is the connector decoder's to refuse.
        assert!(serde_json::from_value::<PreviewComponent>(serde_json::json!({"id": "c", "kind": "k", "node": "n", "objective_connector": "text"})).is_ok());
    }

    #[test]
    fn declared_fields_match_the_committed_schema_key_lists() {
        let schema = schema();
        let fields = record_fields();
        // `shear_model` is a rejection-only root member (refused by name), not an accepted key.
        let mut root = set(fields.model);
        assert!(root.remove("shear_model"));
        assert_eq!(root, keys(&schema, "model root"), "model root");
        assert_eq!(set(fields.load_case), def(&schema, "LoadCase"), "load case");
        assert_eq!(set(fields.primitive_load), def(&schema, "LoadRecord"), "load record");
        assert_eq!(set(fields.combination), def(&schema, "Combination"), "combination");
        assert_eq!(set(fields.term), def(&schema, "CombinationTerm"), "combination term");
        assert_eq!(set(fields.magnitude), def(&schema, "Quantity"), "magnitude and solve temperature");
        assert_eq!(set(fields.equivalent_static), def(&schema, "EquivalentStatic"), "equivalent static");
        assert_eq!(set(fields.seismic), def(&schema, "SeismicEquivalentStaticInput"), "seismic input");
        assert_eq!(set(fields.wind), def(&schema, "WindEquivalentStaticInput"), "wind input");
        assert_eq!(set(fields.exposed_span), def(&schema, "WindExposedSpan"), "exposed span");
        // The node and element arms of LoadTarget; its support arm is refused as an unknown
        // variant by the typed target (and, on a pre-load-state model, by name before parsing).
        let arms = schema["$defs"]["LoadTarget"]["oneOf"].as_array().expect("LoadTarget arms");
        for kind in ["node", "element"] {
            let arm = arms.iter().find(|arm| arm["properties"]["type"]["const"] == kind).unwrap_or_else(|| panic!("{kind} arm"));
            assert_eq!(set(target_fields(kind).unwrap()), keys(arm, kind), "{kind} target");
        }
    }

    #[test]
    fn the_typed_target_refuses_what_the_scan_names() {
        for (target, ok) in [
            (serde_json::json!({"type": "node", "node": "n"}), true),
            (serde_json::json!({"type": "element", "pipe": "p"}), true),
            (serde_json::json!({"type": "node", "node": "n", "pipe": "p"}), false),
            (serde_json::json!({"type": "element", "pipe": "p", "nod": "n"}), false),
        ] {
            assert_eq!(serde_json::from_value::<crate::LoadTargetInput>(target.clone()).is_ok(), ok, "{target}");
        }
    }
}
