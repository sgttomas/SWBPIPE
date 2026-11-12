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
    EquivalentStaticGenerationInput, FrictionNormalReactionSourceInput,
    MaterialTemperaturePointInput, NonlinearSupportInput, PipeSectionInput, PreviewCombination,
    PreviewCombinationTerm, PreviewComponent, PreviewLoadCase, PreviewModel, PreviewNode, PreviewPipe,
    PreviewPrimitiveLoad, PreviewSection, PreviewSupport, Quantity, SeismicGenerationInput,
    SpringHangerInput, SupportStiffnessInput, Vec3, VectorQuantity, WindExposedSpanInput,
    WindGenerationInput,
};
use crate::operating_conditions::{
    AddedWeightInput, ConditionExposureInput, ConditionSetInput, ConditionUseInput, ConditionValuesInput,
    OperatingConditionInput, TestConditionInput,
};
use crate::spectra::{SpectrumInput, SpectrumInterpolationInput, SpectrumUnitsInput};
use serde::de::{self, DeserializeOwned, Visitor};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Request-boundary refusal code (an `Err` of the value entries).
pub(crate) const UNKNOWN_MODEL_FIELD: &str = "UNKNOWN_MODEL_FIELD";
/// Request-boundary refusal code for a record that is not an object.
pub(crate) const MODEL_RECORD_NOT_OBJECT: &str = "MODEL_RECORD_NOT_OBJECT";
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
pub(crate) fn closed_present_quantity<'de, D: de::Deserializer<'de>>(
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
        "support" => Some(&["type", "support", "dof"]),
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
    // Model document 0.9.0's records (DEL-02-01 Design `model-document-0.9.0.md` §3.4).
    pub(crate) condition_set: &'static [&'static str],
    pub(crate) condition: &'static [&'static str],
    pub(crate) condition_values: &'static [&'static str],
    pub(crate) added_weight: &'static [&'static str],
    pub(crate) exposure: &'static [&'static str],
    pub(crate) condition_use: &'static [&'static str],
    pub(crate) test_condition: &'static [&'static str],
    pub(crate) spectrum: &'static [&'static str],
    pub(crate) spectrum_units: &'static [&'static str],
    pub(crate) spectrum_interpolation: &'static [&'static str],
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
        pipe_section: declared_fields::<PipeSectionInput>(),
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
        material: MATERIAL_FIELDS,
        temperature_point: declared_fields::<MaterialTemperaturePointInput>(),
        condition_set: declared_fields::<ConditionSetInput>(),
        condition: declared_fields::<OperatingConditionInput>(),
        condition_values: declared_fields::<ConditionValuesInput>(),
        added_weight: declared_fields::<AddedWeightInput>(),
        exposure: declared_fields::<ConditionExposureInput>(),
        condition_use: declared_fields::<ConditionUseInput>(),
        test_condition: declared_fields::<TestConditionInput>(),
        spectrum: declared_fields::<SpectrumInput>(),
        spectrum_units: declared_fields::<SpectrumUnitsInput>(),
        spectrum_interpolation: declared_fields::<SpectrumInterpolationInput>(),
    }
}

/// A material record's keys: `MaterialInput`'s, the material-owned
/// `expansion_laws` and model document 0.7.0's annotations (`lib.rs`
/// `MATERIAL_ANNOTATIONS`), bound to the schema and to `MaterialInput` by the
/// key test. The record is read in its flattened form, so this list is stated.
pub(crate) const MATERIAL_FIELDS: &[&str] = &[
    "constitutive_basis",
    "poisson_ratio",
    "id",
    "elastic_modulus",
    "shear_modulus",
    "thermal_expansion_coefficient",
    "temperature_points",
    "provenance",
    "expansion_laws",
    "label",
    "tags",
    "notes",
    "provenance_record",
];

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
const CONDITION_SET_QUANTITIES: [&str; 1] = ["installation_temperature"];
const CONDITION_QUANTITIES: [&str; 3] = ["temperature", "pressure", "contents_density"];
const CONDITION_VALUES_QUANTITIES: [&str; 2] = ["temperature", "pressure"];
const ADDED_WEIGHT_QUANTITIES: [&str; 1] = ["magnitude"];
const SPECTRUM_QUANTITIES: [&str; 1] = ["g_value"];

struct Finding {
    field: String,
    record: String,
    path: String,
}

/// A record of a collection that is not an object, by its path.
struct NonObject {
    path: String,
    is: &'static str,
}

/// One scan's findings: the first `NAMED_LIMIT` formed and kept, the rest only
/// counted, so a request with many findings costs no more than eight.
struct Findings<T> {
    kept: Vec<T>,
    more: usize,
}

impl<T> Default for Findings<T> {
    fn default() -> Self {
        Findings { kept: Vec::new(), more: 0 }
    }
}

impl<T> Findings<T> {
    /// Keeps the finding, formed only while fewer than `NAMED_LIMIT` are kept;
    /// counts it otherwise.
    fn keep_finding(&mut self, finding: impl FnOnce() -> T) {
        if self.kept.len() < NAMED_LIMIT {
            self.kept.push(finding());
        } else {
            self.more += 1;
        }
    }
}

fn id_of(record: &serde_json::Map<String, Value>) -> Option<&str> {
    record.get("id").and_then(Value::as_str)
}

fn check(
    record: &serde_json::Map<String, Value>,
    accepted: &[&str],
    name: impl Fn() -> String,
    path: &str,
    findings: &mut Findings<Finding>,
) {
    for key in record.keys() {
        if !accepted.contains(&key.as_str()) {
            findings.keep_finding(|| Finding { field: key.clone(), record: name(), path: path.to_string() });
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

    /// Named as `named` names a record, with its own sites: they are formed only for a finding.
    fn finding_name(&self) -> String {
        match *self {
            At::Record { kind, id: Some(id), .. } => format!("{kind} `{id}`"),
            At::Record { kind, index, id: None, .. } => format!("{kind} {index}"),
            At::Member { parent, key } => format!("the {key} of {}", parent.finding_name()),
            At::Entry { parent, kind, id: Some(id), .. } => format!("{kind} `{id}` of {}", parent.finding_name()),
            At::Entry { parent, kind, index, id: None, .. } => format!("{kind} {index} of {}", parent.finding_name()),
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
fn check_nested(record: &serde_json::Map<String, Value>, accepted: &[&str], at: &At<'_>, findings: &mut Findings<Finding>) {
    for key in record.keys() {
        if !accepted.contains(&key.as_str()) {
            findings.keep_finding(|| Finding { field: key.clone(), record: at.finding_name(), path: at.finding_path() });
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
    findings: &mut Findings<Finding>,
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
    findings: &mut Findings<Finding>,
) {
    for key in keys {
        nested_member(record, key, accepted, parent, findings);
    }
}

/// A support stiffness record and its value.
fn nested_stiffness(record: &serde_json::Map<String, Value>, fields: &RecordFields, parent: &At<'_>, findings: &mut Findings<Finding>) {
    if let Some((stiffness, at)) = nested_member(record, "stiffness", fields.stiffness, parent, findings) {
        nested_member(stiffness, "value", fields.magnitude, &at, findings);
    }
}

/// The model's nested closed records, and the request's materials.
fn nested_records(
    request: &serde_json::Map<String, Value>,
    model: &serde_json::Map<String, Value>,
    fields: &RecordFields,
    findings: &mut Findings<Finding>,
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
        nested_member(support, "direction", fields.vector, &at, findings);
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
            nested_member(geometry, "eccentric_offset", fields.vector_quantity, &at, findings);
            nested_quantities(geometry, &GEOMETRY_QUANTITIES, fields.magnitude, &at, findings);
        }
        if let Some((modifiers, at)) = nested_member(component, "modifiers", fields.modifiers, &at, findings) {
            nested_quantities(modifiers, &MODIFIER_QUANTITIES, fields.magnitude, &at, findings);
        }
        nested_member(component, "mechanics_interface", fields.mechanics_interface, &at, findings);
    }
    for (i, set) in objects(model, "condition_sets") {
        let at = At::in_collection("model.condition_sets", "condition set", i, set);
        check_nested(set, fields.condition_set, &at, findings);
        nested_quantities(set, &CONDITION_SET_QUANTITIES, fields.magnitude, &at, findings);
        for (k, condition) in objects(set, "conditions") {
            let at = At::Entry { parent: &at, key: "conditions", kind: "condition", index: k, id: condition.get("key").and_then(Value::as_str) };
            check_nested(condition, fields.condition, &at, findings);
            nested_quantities(condition, &CONDITION_QUANTITIES, fields.magnitude, &at, findings);
        }
        for key in ["design", "peak"] {
            if let Some((values, at)) = nested_member(set, key, fields.condition_values, &at, findings) {
                nested_quantities(values, &CONDITION_VALUES_QUANTITIES, fields.magnitude, &at, findings);
            }
        }
        if let Some((weight, at)) = nested_member(set, "added_weight", fields.added_weight, &at, findings) {
            nested_quantities(weight, &ADDED_WEIGHT_QUANTITIES, fields.magnitude, &at, findings);
        }
        nested_member(set, "exposure", fields.exposure, &at, findings);
    }
    for (i, spectrum) in objects(model, "spectra") {
        let at = At::in_collection("model.spectra", "spectrum", i, spectrum);
        check_nested(spectrum, fields.spectrum, &at, findings);
        nested_member(spectrum, "units", fields.spectrum_units, &at, findings);
        nested_member(spectrum, "interpolation", fields.spectrum_interpolation, &at, findings);
        nested_quantities(spectrum, &SPECTRUM_QUANTITIES, fields.magnitude, &at, findings);
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

/// The model's record collections; the request's are its materials.
const RECORD_COLLECTIONS: [&str; 10] = [
    "load_cases", "combinations", "nodes", "pipe_segments", "sections", "supports", "components", "materials",
    "condition_sets", "spectra",
];

/// What a record that is not an object is.
fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Array(_) => "an array (a positional record)",
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Object(_) => "an object",
    }
}

/// Every element of a record collection that is not an object. Serde's
/// derives also read a record in positional (array) form, whose keys the scan
/// cannot name, so such a record is refused by its path.
fn non_object_records(
    request: &serde_json::Map<String, Value>,
    model: &serde_json::Map<String, Value>,
    findings: &mut Findings<NonObject>,
) {
    let request_collections: &[&str] = &["materials"];
    for (owner, prefix, collections) in [(model, "model.", &RECORD_COLLECTIONS[..]), (request, "", request_collections)] {
        for collection in collections {
            for (i, record) in owner.get(*collection).and_then(Value::as_array).into_iter().flatten().enumerate() {
                if !record.is_object() {
                    findings.keep_finding(|| NonObject { path: format!("{prefix}{collection}[{i}]"), is: value_kind(record) });
                }
            }
        }
    }
}

/// A refusal naming the kept findings and counting the rest.
fn listed_refusal<T>(code: &str, findings: &Findings<T>, each: impl Fn(&T) -> String, remedy: &str) -> String {
    let mut listed = findings.kept.iter().map(each).collect::<Vec<_>>().join("; ");
    if findings.more > 0 {
        listed.push_str(&format!("; and {} more", findings.more));
    }
    format!("{code}: request: {listed}. {remedy}")
}

/// The named refusal for a record that is not an object, or for undeclared
/// keys on the closed records of the actual request's model and on its
/// materials, or `None`. Shapes other than unknown keys (a missing or
/// mistyped member, a non-object record) are left to the typed parse.
pub(crate) fn unknown_field_refusal(actual_request: &Value) -> Option<String> {
    let request = actual_request.as_object()?;
    let model = request.get("model")?.as_object()?;
    let mut non_objects = Findings::default();
    non_object_records(request, model, &mut non_objects);
    if !non_objects.kept.is_empty() {
        return Some(listed_refusal(MODEL_RECORD_NOT_OBJECT, &non_objects, |f| format!("{} is {}", f.path, f.is),
            "A model record is an object of named members; a record in another form cannot have its keys checked, so nothing is solved. Write each record as an object."));
    }
    let fields = record_fields();
    let mut findings = Findings::default();
    check(model, fields.model, || "the model".into(), "model", &mut findings);
    for (i, case) in objects(model, "load_cases") {
        let case_name = named("load case", id_of(case), i);
        let case_path = format!("model.load_cases[{i}]");
        check(case, fields.load_case, || case_name.clone(), &case_path, &mut findings);
        if let Some(temperature) = case.get("modulus_basis_temperature").and_then(Value::as_object) {
            check(temperature, fields.magnitude, || format!("the modulus_basis_temperature of {case_name}"), &format!("{case_path}.modulus_basis_temperature"), &mut findings);
        }
        if let Some(using) = case.get("condition_use").and_then(Value::as_object) {
            check(using, fields.condition_use, || format!("the condition_use of {case_name}"), &format!("{case_path}.condition_use"), &mut findings);
        }
        if let Some(test) = case.get("test_condition").and_then(Value::as_object) {
            check(test, fields.test_condition, || format!("the test_condition of {case_name}"), &format!("{case_path}.test_condition"), &mut findings);
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
    if findings.kept.is_empty() {
        return None;
    }
    Some(listed_refusal(UNKNOWN_MODEL_FIELD, &findings, |f| format!("unknown field `{}` in {} ({})", f.field, f.record, f.path),
        "The model document does not define these keys, so they would be ignored; nothing is solved. Correct or remove each key."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MaterialInput;
    use std::collections::BTreeSet;

    fn set(keys: &[&str]) -> BTreeSet<String> {
        keys.iter().map(|k| k.to_string()).collect()
    }

    /// The committed model document schema (`schemas/model.schema.yaml`, JSON
    /// syntax; model document 0.8.0) is the authoritative key list of each
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
        assert_eq!(crate::MODEL_SCHEMA_VERSION, "0.9.0");
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
            // Model document 0.9.0's records.
            ("condition set", fields.condition_set, "ConditionSet"),
            ("operating condition", fields.condition, "OperatingCondition"),
            ("condition values", fields.condition_values, "ConditionValues"),
            ("added weight", fields.added_weight, "AddedWeight"),
            ("condition exposure", fields.exposure, "ConditionExposure"),
            ("condition use", fields.condition_use, "ConditionUse"),
            ("test condition", fields.test_condition, "TestCondition"),
            ("spectrum", fields.spectrum, "Spectrum"),
        ] {
            assert_eq!(set(declared), def(&schema, name), "{record}");
        }
        assert_eq!(set(fields.spectrum_units), keys(&d["Spectrum"]["properties"]["units"], "spectrum units"));
        assert_eq!(set(fields.spectrum_interpolation), keys(&d["Spectrum"]["properties"]["interpolation"], "spectrum interpolation"));
        assert_eq!(
            set(fields.reaction_source),
            keys(&d["NonlinearSupport"]["properties"]["normal_reaction_source"], "normal reaction source"),
            "normal reaction source"
        );
        // The material record's stated keys are `MaterialInput`'s, `expansion_laws` and the annotations.
        let mut material = set(declared_fields::<MaterialInput>());
        material.extend(set(&["expansion_laws"]).into_iter().chain(set(&crate::MATERIAL_ANNOTATIONS)));
        assert_eq!(set(MATERIAL_FIELDS), material, "material record");
        // The scan checks every quantity and vector member of the records it walks.
        for (record, scanned, name) in [
            ("pipe section", &PIPE_SECTION_QUANTITIES[..], "PipeSection"),
            ("hanger", &HANGER_QUANTITIES[..], "SpringHanger"),
            ("nonlinear support", &NONLINEAR_QUANTITIES[..], "NonlinearSupport"),
            ("component geometry", &GEOMETRY_QUANTITIES[..], "ComponentGeometry"),
            ("component modifiers", &MODIFIER_QUANTITIES[..], "ComponentModifiers"),
            ("material", &MATERIAL_QUANTITIES[..], "Material"),
            ("temperature point", &POINT_QUANTITIES[..], "MaterialTemperaturePoint"),
            ("condition set", &CONDITION_SET_QUANTITIES[..], "ConditionSet"),
            ("operating condition", &CONDITION_QUANTITIES[..], "OperatingCondition"),
            ("condition values", &CONDITION_VALUES_QUANTITIES[..], "ConditionValues"),
            ("added weight", &ADDED_WEIGHT_QUANTITIES[..], "AddedWeight"),
            ("spectrum", &SPECTRUM_QUANTITIES[..], "Spectrum"),
        ] {
            assert_eq!(set(scanned), refs(&d[name], "Quantity"), "{record} quantities");
        }
        assert_eq!(refs(&d["Support"], "Vector3"), set(&["direction"]));
        for name in ["Node", "PipeSegment", "Section", "Support", "SupportStiffness", "Component", "ComponentMechanicsInterface"] {
            let quantities = refs(&d[name], "Quantity");
            let expected: BTreeSet<String> = if name == "SupportStiffness" { set(&["value"]) } else { BTreeSet::new() };
            assert_eq!(quantities, expected, "{name}'s quantities are the ones the scan walks");
        }
        assert_eq!(refs(&d["Node"], "Vector3"), set(&["position"]));
        assert_eq!(refs(&d["PipeSegment"], "Vector3"), set(&["y_reference"]));
        assert_eq!(refs(&d["ComponentGeometry"], "VectorQuantity"), set(&["center_of_gravity", "eccentric_offset"]));
        assert_eq!(refs(&d["SpringHanger"], "SupportStiffness"), set(&["stiffness"]));
        assert_eq!(refs(&d["Support"], "SupportStiffness"), set(&["stiffness"]));
        assert_eq!(d["Section"]["properties"]["properties"]["additionalProperties"]["$ref"], "#/$defs/Quantity");
    }

    #[test]
    fn a_scan_keeps_at_most_eight_findings_and_counts_the_rest() {
        // A 16 KiB id and 1,024 unknown keys: eight findings are formed and kept, the rest counted.
        let mut record = serde_json::Map::new();
        record.insert("id".into(), Value::String("n".repeat(16 * 1024)));
        for k in 0..1024 {
            record.insert(format!("extra_{k:04}"), Value::from(k));
        }
        let formed = std::cell::Cell::new(0);
        let mut findings = Findings::default();
        check(&record, &["id"], || { formed.set(formed.get() + 1); "the record".into() }, "model", &mut findings);
        assert_eq!((findings.kept.len(), findings.more, formed.get()), (NAMED_LIMIT, 1016, NAMED_LIMIT));
        let at = At::in_collection("model.nodes", "node", 0, &record);
        let mut nested = Findings::default();
        check_nested(&record, &["id"], &at, &mut nested);
        assert_eq!((nested.kept.len(), nested.more), (NAMED_LIMIT, 1016));
        let refusal = unknown_field_refusal(&serde_json::json!({"model": {"nodes": [record]}})).unwrap();
        assert_eq!(refusal.matches("unknown field `extra_").count(), NAMED_LIMIT, "{}", &refusal[..200]);
        assert!(refusal.contains("; and 1016 more. "), "{}", &refusal[refusal.len() - 200..]);
    }

    #[test]
    fn a_record_that_is_not_an_object_is_refused_by_its_path() {
        let refusal = |request: Value| unknown_field_refusal(&request).unwrap();
        let positional = serde_json::json!({"model": {"nodes": [["n", {"x": 0, "y": 0, "z": 0, "extra": 1}, null, null, null, null]]}});
        assert_eq!(refusal(positional), format!("{MODEL_RECORD_NOT_OBJECT}: request: model.nodes[0] is an array (a positional record). A model record is an object of named members; a record in another form cannot have its keys checked, so nothing is solved. Write each record as an object."));
        let mixed = serde_json::json!({"model": {"load_cases": [null], "supports": [{"id": "s"}, "s"], "nodes": [{"extra": 1}]}, "materials": [7]});
        let text = refusal(mixed);
        for named in ["model.load_cases[0] is null", "model.supports[1] is a string", "materials[0] is a number"] {
            assert!(text.contains(named), "{text}");
        }
        assert!(!text.contains("unknown field"), "the record form is refused first: {text}");
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
        // Every arm of LoadTarget: node, element and (W-16 slice a) support.
        let arms = schema["$defs"]["LoadTarget"]["oneOf"].as_array().expect("LoadTarget arms");
        assert_eq!(arms.len(), 3, "LoadTarget arms");
        for kind in ["node", "element", "support"] {
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
            (serde_json::json!({"type": "support", "support": "s", "dof": "UY"}), true),
            (serde_json::json!({"type": "support", "support": "s"}), false),
            (serde_json::json!({"type": "support", "support": "s", "dof": "UY", "node": "n"}), false),
        ] {
            assert_eq!(serde_json::from_value::<crate::LoadTargetInput>(target.clone()).is_ok(), ok, "{target}");
        }
    }
}
