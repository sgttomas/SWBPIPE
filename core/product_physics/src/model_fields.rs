//! Closed model records: an undeclared key on the model root, a load case (with
//! its modulus_basis_temperature and equivalent_static inputs), a primitive load
//! (with its target and magnitude), a combination or a combination term is
//! refused by name before anything is solved, never
//! silently dropped. A misspelled `primitive_loads` would otherwise empty a
//! case, and a misspelled `combinations` would drop every combination.
//!
//! The records deny unknown fields in their serde derives, so every typed
//! reader refuses. The request boundary first scans the actual request so the
//! refusal names the record (and its id and path) as well as the field. The
//! accepted field sets are read from the serde derives themselves, so the scan
//! and the typed parse cannot disagree.

use crate::{
    EquivalentStaticGenerationInput, PreviewCombination, PreviewCombinationTerm, PreviewLoadCase,
    PreviewModel, PreviewPrimitiveLoad, Quantity, SeismicGenerationInput, WindExposedSpanInput,
    WindGenerationInput,
};
use serde::de::{self, DeserializeOwned, Visitor};
use serde::Deserialize;
use serde_json::Value;

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
    }
}

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

/// The named refusal for undeclared keys on the closed records of the actual
/// request's model, or `None`. Shapes other than unknown keys (a missing or
/// mistyped member, a non-object record) are left to the typed parse.
pub(crate) fn unknown_field_refusal(actual_request: &Value) -> Option<String> {
    let model = actual_request.get("model")?.as_object()?;
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
