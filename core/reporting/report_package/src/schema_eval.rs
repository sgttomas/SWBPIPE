//! JSON Schema 2020-12 evaluation of the DEL-08-04 result-export 0.3.0 schema
//! closure (DEL-08-01 Design `report-package-current-results.md` §2.3 item 1).
//!
//! The evaluator is bounded to the constructs the five embedded resources use.
//! It is not a general JSON Schema implementation:
//! - **Resources.** A closed, explicit catalog: `results.v0.3.schema.yaml` and
//!   its transitive closure, embedded at compile time. Nothing is loaded from
//!   the file system or the network. A reference resolves by its resource's
//!   final path segment against the catalog, and a local (`#…`) reference
//!   against the resource it appears in. An unknown resource or pointer fails
//!   closed.
//! - **Keywords.** The census of the closure:
//!   - core and resource keywords: `$schema`, `$id`, `$defs`, `$ref`;
//!   - annotations: `description`, `title`, `x-rp-encoding`;
//!   - the ignored comment keyword `$comment`;
//!   - evaluated keywords: everything in [`EVALUATED_KEYWORDS`].
//!
//!   Any other keyword in an evaluated schema fails closed
//!   ([`SchemaFailure::Unsupported`]).
//! - **Semantics.**
//!   - Applicability follows the instance type: `properties`, `required` and
//!     `dependentRequired` apply only to objects, `items` and `contains` only
//!     to arrays, and so on. No type assertion is invented.
//!   - `required` holds without `properties`, and `properties` without a
//!     `type`. An object schema without `properties` admits unconstrained
//!     members.
//!   - There is no implicit array cap.
//!   - `const`, `enum` and `uniqueItems` use JSON equality: numbers by value,
//!     booleans distinct from numbers, and objects independent of member order.
//!   - `pattern` supports exactly the closure's two forms,
//!     `^[0-9a-f]{N}$`, with ECMA-262 semantics: `$` matches only at the end
//!     of input. Python's `re.search` also accepts a final line feed there;
//!     that is a recorded oracle divergence (`fixtures/results/schema_eval_vectors.json`).
//!
//! The existing `result_export::source_blocks` shape checker and its consumers
//! are untouched. Standard claim fence applies (claims taxonomy per DEC-081).

use serde_json::{Map, Number, Value};
use std::fmt;
use std::sync::OnceLock;

/// The root resource of the result-export 0.3.0 contract.
pub const RESULTS_V03: &str = "results.v0.3.schema.yaml";

/// The closed resource catalog: the root and its transitive references.
pub const RESULTS_V03_CLOSURE: [(&str, &str); 5] = [
    (RESULTS_V03, include_str!("../../../../schemas/results.v0.3.schema.yaml")),
    (
        "results_pressure_2.v0.3.schema.json",
        include_str!("../../../../schemas/results_pressure_2.v0.3.schema.json"),
    ),
    (
        "physics_source_recovery.schema.json",
        include_str!("../../../../schemas/physics_source_recovery.schema.json"),
    ),
    (
        "retained_precision_mp_v2.schema.json",
        include_str!("../../../../schemas/retained_precision_mp_v2.schema.json"),
    ),
    (
        "load_reference_state.schema.json",
        include_str!("../../../../schemas/load_reference_state.schema.json"),
    ),
];

/// Core and resource keywords: they shape resolution, not instance validity.
pub const CORE_KEYWORDS: [&str; 4] = ["$schema", "$id", "$defs", "$ref"];
/// Annotation keywords: no assertion.
pub const ANNOTATION_KEYWORDS: [&str; 3] = ["description", "title", "x-rp-encoding"];
/// The ignored comment keyword.
pub const COMMENT_KEYWORDS: [&str; 1] = ["$comment"];
/// The evaluated assertion and applicator keywords (`$ref` is evaluated too,
/// and is listed with the core keywords).
pub const EVALUATED_KEYWORDS: [&str; 24] = [
    "additionalProperties",
    "allOf",
    "anyOf",
    "const",
    "contains",
    "dependentRequired",
    "else",
    "enum",
    "exclusiveMinimum",
    "if",
    "items",
    "maxItems",
    "maximum",
    "minItems",
    "minLength",
    "minimum",
    "not",
    "oneOf",
    "pattern",
    "properties",
    "required",
    "then",
    "type",
    "uniqueItems",
];

/// Schema keywords whose value is a map from names to subschemas (or, for
/// `dependentRequired`, to name lists); their keys are not keywords.
pub const MAP_KEYWORDS: [&str; 3] = ["properties", "$defs", "dependentRequired"];

/// Evaluation depth bound: a reference cycle that consumes no instance depth
/// fails closed instead of recursing without end.
const MAX_DEPTH: usize = 256;

/// The instance paths at which one marked schema location was applied (see
/// [`Catalog::validate_marking`]).
struct Marks<'a> {
    target: Option<&'a Value>,
    found: Vec<String>,
}

impl Marks<'_> {
    fn none() -> Self {
        Marks { target: None, found: Vec::new() }
    }
}

/// Why an instance or a schema did not validate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaFailure {
    /// The instance does not satisfy `keyword` at `instance_path`.
    Invalid { instance_path: String, keyword: String },
    /// The schema uses a keyword, pattern or reference this evaluator does not
    /// implement; evaluation fails closed.
    Unsupported { detail: String },
}

impl fmt::Display for SchemaFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid { instance_path, keyword } => {
                write!(f, "instance {instance_path} fails `{keyword}`")
            }
            Self::Unsupported { detail } => write!(f, "schema unsupported: {detail}"),
        }
    }
}

/// A closed set of parsed schema resources.
#[derive(Debug, Clone)]
pub struct Catalog {
    resources: Vec<(String, Value)>,
}

fn final_segment(reference: &str) -> &str {
    reference.rsplit('/').next().unwrap_or(reference)
}

impl Catalog {
    /// Builds a catalog from `(name, schema)` pairs. Names are the resources'
    /// file names; references resolve by their final path segment.
    pub fn new(resources: Vec<(String, Value)>) -> Self {
        Self { resources }
    }

    /// The embedded result-export 0.3.0 closure, parsed once.
    pub fn results_v03() -> &'static Catalog {
        static CATALOG: OnceLock<Catalog> = OnceLock::new();
        CATALOG.get_or_init(|| {
            Catalog::new(
                RESULTS_V03_CLOSURE
                    .iter()
                    .map(|(name, text)| {
                        let value = serde_json::from_str(text)
                            .unwrap_or_else(|error| panic!("embedded schema {name} parses: {error}"));
                        (name.to_string(), value)
                    })
                    .collect(),
            )
        })
    }

    /// The resources, in catalog order.
    pub fn resources(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.resources.iter().map(|(name, value)| (name.as_str(), value))
    }

    fn index(&self, name: &str) -> Option<usize> {
        let name = final_segment(name);
        self.resources.iter().position(|(own, _)| own == name)
    }

    /// Validates `instance` against the schema at `pointer` (an RFC 6901 JSON
    /// pointer, `""` for the whole resource) in resource `resource`.
    pub fn validate_at(&self, resource: &str, pointer: &str, instance: &Value) -> Result<(), SchemaFailure> {
        let index = self.index(resource).ok_or_else(|| SchemaFailure::Unsupported {
            detail: format!("resource {resource} is not in the catalog"),
        })?;
        let schema = self.resources[index].1.pointer(pointer).ok_or_else(|| SchemaFailure::Unsupported {
            detail: format!("pointer {pointer} is not in {resource}"),
        })?;
        let mut path = String::new();
        self.eval(index, schema, instance, &mut path, 0, &mut Marks::none())
    }

    /// Validates `instance` like [`Catalog::validate_at`], and returns the
    /// instance paths (RFC 6901) at which the schema `marked_pointer` of
    /// `marked_resource` was applied by a `$ref` and held, outside any failed
    /// branch. This is how a caller finds the instance locations the schema
    /// itself types as, say, a provenance record, without reading arbitrary
    /// members of the same name elsewhere.
    pub fn validate_marking(
        &self,
        resource: &str,
        instance: &Value,
        marked_resource: &str,
        marked_pointer: &str,
    ) -> Result<Vec<String>, SchemaFailure> {
        let unsupported = |detail: String| SchemaFailure::Unsupported { detail };
        let index = self.index(resource).ok_or_else(|| unsupported(format!("resource {resource} is not in the catalog")))?;
        let marked_index = self
            .index(marked_resource)
            .ok_or_else(|| unsupported(format!("resource {marked_resource} is not in the catalog")))?;
        let target = self.resources[marked_index]
            .1
            .pointer(marked_pointer)
            .ok_or_else(|| unsupported(format!("pointer {marked_pointer} is not in {marked_resource}")))?;
        let mut marks = Marks { target: Some(target), found: Vec::new() };
        let mut path = String::new();
        self.eval(index, &self.resources[index].1, instance, &mut path, 0, &mut marks)?;
        Ok(marks.found)
    }

    /// Validates `instance` against the whole resource `resource`.
    pub fn validate(&self, resource: &str, instance: &Value) -> Result<(), SchemaFailure> {
        self.validate_at(resource, "", instance)
    }

    fn resolve(&self, current: usize, reference: &str) -> Result<(usize, &Value), SchemaFailure> {
        let unsupported = || SchemaFailure::Unsupported { detail: format!("unresolved reference {reference}") };
        let (resource, fragment) = match reference.split_once('#') {
            Some((resource, fragment)) => (resource, fragment),
            None => (reference, ""),
        };
        let index = if resource.is_empty() { current } else { self.index(resource).ok_or_else(unsupported)? };
        let target = self.resources[index].1.pointer(fragment).ok_or_else(unsupported)?;
        Ok((index, target))
    }

    fn eval(
        &self,
        resource: usize,
        schema: &Value,
        instance: &Value,
        path: &mut String,
        depth: usize,
        marks: &mut Marks<'_>,
    ) -> Result<(), SchemaFailure> {
        if depth > MAX_DEPTH {
            return Err(SchemaFailure::Unsupported { detail: "evaluation depth bound reached".to_string() });
        }
        let schema = match schema {
            Value::Bool(true) => return Ok(()),
            Value::Bool(false) => return Err(invalid(path, "false")),
            Value::Object(schema) => schema,
            _ => return Err(SchemaFailure::Unsupported { detail: format!("non-schema value at {path}") }),
        };
        for key in schema.keys() {
            if !is_known_keyword(key) {
                return Err(SchemaFailure::Unsupported { detail: format!("keyword {key}") });
            }
        }
        if let Some(reference) = schema.get("$ref") {
            let reference = reference.as_str().ok_or_else(|| SchemaFailure::Unsupported {
                detail: "non-string $ref".to_string(),
            })?;
            let (target_resource, target) = self.resolve(resource, reference)?;
            self.eval(target_resource, target, instance, path, depth + 1, marks)?;
            if marks.target.is_some_and(|marked| std::ptr::eq(marked, target)) {
                marks.found.push(path.clone());
            }
        }
        if let Some(types) = schema.get("type") {
            if !type_matches(types, instance)? {
                return Err(invalid(path, "type"));
            }
        }
        if let Some(constant) = schema.get("const") {
            if !json_equal(constant, instance) {
                return Err(invalid(path, "const"));
            }
        }
        if let Some(values) = schema.get("enum") {
            let values = values.as_array().ok_or_else(|| unsupported_value("enum"))?;
            if !values.iter().any(|value| json_equal(value, instance)) {
                return Err(invalid(path, "enum"));
            }
        }
        match instance {
            Value::Number(number) => self.eval_number(schema, number, path)?,
            Value::String(text) => eval_string(schema, text, path)?,
            Value::Array(items) => self.eval_array(resource, schema, items, path, depth, marks)?,
            Value::Object(members) => self.eval_object(resource, schema, members, path, depth, marks)?,
            _ => {}
        }
        if let Some(all) = schema.get("allOf") {
            for branch in all.as_array().ok_or_else(|| unsupported_value("allOf"))? {
                self.eval(resource, branch, instance, path, depth + 1, marks)?;
            }
        }
        if let Some(any) = schema.get("anyOf") {
            let branches = any.as_array().ok_or_else(|| unsupported_value("anyOf"))?;
            let mut matched = false;
            for branch in branches {
                if self.passes(resource, branch, instance, path, depth, marks)? {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return Err(invalid(path, "anyOf"));
            }
        }
        if let Some(one) = schema.get("oneOf") {
            let branches = one.as_array().ok_or_else(|| unsupported_value("oneOf"))?;
            let mut matched = 0usize;
            for branch in branches {
                if self.passes(resource, branch, instance, path, depth, marks)? {
                    matched += 1;
                }
            }
            if matched != 1 {
                return Err(invalid(path, "oneOf"));
            }
        }
        if let Some(negated) = schema.get("not") {
            if self.passes(resource, negated, instance, path, depth, marks)? {
                return Err(invalid(path, "not"));
            }
        }
        if let Some(condition) = schema.get("if") {
            if self.passes(resource, condition, instance, path, depth, marks)? {
                if let Some(then) = schema.get("then") {
                    self.eval(resource, then, instance, path, depth + 1, marks)?;
                }
            } else if let Some(otherwise) = schema.get("else") {
                self.eval(resource, otherwise, instance, path, depth + 1, marks)?;
            }
        }
        Ok(())
    }

    /// Whether `instance` satisfies `schema`. An unsupported construct is not a
    /// mismatch: it propagates, so a branch cannot silently pass or fail.
    fn passes(
        &self,
        resource: usize,
        schema: &Value,
        instance: &Value,
        path: &mut String,
        depth: usize,
        marks: &mut Marks<'_>,
    ) -> Result<bool, SchemaFailure> {
        let length = path.len();
        let marked = marks.found.len();
        let outcome = self.eval(resource, schema, instance, path, depth + 1, marks);
        path.truncate(length);
        if outcome.is_err() {
            marks.found.truncate(marked);
        }
        match outcome {
            Ok(()) => Ok(true),
            Err(SchemaFailure::Invalid { .. }) => Ok(false),
            Err(failure @ SchemaFailure::Unsupported { .. }) => Err(failure),
        }
    }

    fn eval_number(&self, schema: &Map<String, Value>, number: &Number, path: &str) -> Result<(), SchemaFailure> {
        let value = number.as_f64().ok_or_else(|| unsupported_value("number"))?;
        let bound = |keyword: &str| -> Result<Option<f64>, SchemaFailure> {
            match schema.get(keyword) {
                None => Ok(None),
                Some(Value::Number(bound)) => Ok(bound.as_f64()),
                Some(_) => Err(unsupported_value(keyword)),
            }
        };
        if let Some(minimum) = bound("minimum")? {
            if value < minimum {
                return Err(invalid(path, "minimum"));
            }
        }
        if let Some(maximum) = bound("maximum")? {
            if value > maximum {
                return Err(invalid(path, "maximum"));
            }
        }
        if let Some(exclusive) = bound("exclusiveMinimum")? {
            if value <= exclusive {
                return Err(invalid(path, "exclusiveMinimum"));
            }
        }
        Ok(())
    }

    fn eval_array(
        &self,
        resource: usize,
        schema: &Map<String, Value>,
        items: &[Value],
        path: &mut String,
        depth: usize,
        marks: &mut Marks<'_>,
    ) -> Result<(), SchemaFailure> {
        if let Some(minimum) = count(schema, "minItems")? {
            if (items.len() as u64) < minimum {
                return Err(invalid(path, "minItems"));
            }
        }
        if let Some(maximum) = count(schema, "maxItems")? {
            if (items.len() as u64) > maximum {
                return Err(invalid(path, "maxItems"));
            }
        }
        match schema.get("uniqueItems") {
            None | Some(Value::Bool(false)) => {}
            Some(Value::Bool(true)) => {
                if has_duplicate(items) {
                    return Err(invalid(path, "uniqueItems"));
                }
            }
            Some(_) => return Err(unsupported_value("uniqueItems")),
        }
        if let Some(item_schema) = schema.get("items") {
            for (index, item) in items.iter().enumerate() {
                let length = path.len();
                path.push_str(&format!("/{index}"));
                self.eval(resource, item_schema, item, path, depth + 1, marks)?;
                path.truncate(length);
            }
        }
        if let Some(contained) = schema.get("contains") {
            let mut found = false;
            for (index, item) in items.iter().enumerate() {
                let length = path.len();
                path.push_str(&format!("/{index}"));
                let passes = self.passes(resource, contained, item, path, depth, marks)?;
                path.truncate(length);
                if passes {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(invalid(path, "contains"));
            }
        }
        Ok(())
    }

    fn eval_object(
        &self,
        resource: usize,
        schema: &Map<String, Value>,
        members: &Map<String, Value>,
        path: &mut String,
        depth: usize,
        marks: &mut Marks<'_>,
    ) -> Result<(), SchemaFailure> {
        if let Some(required) = schema.get("required") {
            for name in required.as_array().ok_or_else(|| unsupported_value("required"))? {
                let name = name.as_str().ok_or_else(|| unsupported_value("required"))?;
                if !members.contains_key(name) {
                    return Err(invalid(path, "required"));
                }
            }
        }
        if let Some(dependencies) = schema.get("dependentRequired") {
            let dependencies = dependencies.as_object().ok_or_else(|| unsupported_value("dependentRequired"))?;
            for (present, names) in dependencies {
                if !members.contains_key(present) {
                    continue;
                }
                for name in names.as_array().ok_or_else(|| unsupported_value("dependentRequired"))? {
                    let name = name.as_str().ok_or_else(|| unsupported_value("dependentRequired"))?;
                    if !members.contains_key(name) {
                        return Err(invalid(path, "dependentRequired"));
                    }
                }
            }
        }
        let properties = match schema.get("properties") {
            None => None,
            Some(Value::Object(properties)) => Some(properties),
            Some(_) => return Err(unsupported_value("properties")),
        };
        if let Some(properties) = properties {
            for (name, property_schema) in properties {
                if let Some(member) = members.get(name) {
                    let length = path.len();
                    path.push('/');
                    path.push_str(&escape_pointer(name));
                    self.eval(resource, property_schema, member, path, depth + 1, marks)?;
                    path.truncate(length);
                }
            }
        }
        if let Some(additional) = schema.get("additionalProperties") {
            for (name, member) in members {
                if properties.is_some_and(|properties| properties.contains_key(name)) {
                    continue;
                }
                let length = path.len();
                path.push('/');
                path.push_str(&escape_pointer(name));
                self.eval(resource, additional, member, path, depth + 1, marks)?;
                path.truncate(length);
            }
        }
        Ok(())
    }
}

fn eval_string(schema: &Map<String, Value>, text: &str, path: &str) -> Result<(), SchemaFailure> {
    if let Some(minimum) = count(schema, "minLength")? {
        if (text.chars().count() as u64) < minimum {
            return Err(invalid(path, "minLength"));
        }
    }
    if let Some(pattern) = schema.get("pattern") {
        let pattern = pattern.as_str().ok_or_else(|| unsupported_value("pattern"))?;
        if !pattern_matches(pattern, text)? {
            return Err(invalid(path, "pattern"));
        }
    }
    Ok(())
}

fn count(schema: &Map<String, Value>, keyword: &str) -> Result<Option<u64>, SchemaFailure> {
    match schema.get(keyword) {
        None => Ok(None),
        Some(Value::Number(number)) => number.as_u64().map(Some).ok_or_else(|| unsupported_value(keyword)),
        Some(_) => Err(unsupported_value(keyword)),
    }
}

/// The closure's two patterns, `^[0-9a-f]{16}$` and `^[0-9a-f]{64}$`, as one
/// form `^[0-9a-f]{N}$`, with ECMA-262 anchors. Any other pattern fails closed.
fn pattern_matches(pattern: &str, text: &str) -> Result<bool, SchemaFailure> {
    let length = pattern
        .strip_prefix("^[0-9a-f]{")
        .and_then(|rest| rest.strip_suffix("}$"))
        .and_then(|digits| digits.parse::<usize>().ok())
        .ok_or_else(|| SchemaFailure::Unsupported { detail: format!("pattern {pattern}") })?;
    Ok(text.len() == length && text.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')))
}

fn type_matches(types: &Value, instance: &Value) -> Result<bool, SchemaFailure> {
    match types {
        Value::String(name) => single_type_matches(name, instance),
        Value::Array(names) => {
            for name in names {
                let name = name.as_str().ok_or_else(|| unsupported_value("type"))?;
                if single_type_matches(name, instance)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        _ => Err(unsupported_value("type")),
    }
}

fn single_type_matches(name: &str, instance: &Value) -> Result<bool, SchemaFailure> {
    Ok(match name {
        "null" => instance.is_null(),
        "boolean" => instance.is_boolean(),
        "object" => instance.is_object(),
        "array" => instance.is_array(),
        "string" => instance.is_string(),
        "number" => instance.is_number(),
        "integer" => match instance {
            Value::Number(number) => {
                number.is_i64()
                    || number.is_u64()
                    || number.as_f64().is_some_and(|value| value.is_finite() && value.fract() == 0.0)
            }
            _ => false,
        },
        _ => return Err(SchemaFailure::Unsupported { detail: format!("type {name}") }),
    })
}

/// JSON equality (JSON Schema 2020-12 §4.2.2): numbers by mathematical value,
/// booleans distinct from numbers, objects independent of member order.
pub fn json_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(left), Value::Bool(right)) => left == right,
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Number(left), Value::Number(right)) => number_equal(left, right),
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len() && left.iter().zip(right).all(|(left, right)| json_equal(left, right))
        }
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left.iter().all(|(key, value)| right.get(key).is_some_and(|other| json_equal(value, other)))
        }
        _ => false,
    }
}

/// A JSON integer's exact value.
fn integer_value(number: &Number) -> Option<i128> {
    number.as_i64().map(i128::from).or_else(|| number.as_u64().map(i128::from))
}

/// A binary64 value's exact integer value, when it is integral and in range.
fn integral_value(value: f64) -> Option<i128> {
    // 2^126: every integral binary64 below it converts to i128 exactly.
    (value.is_finite() && value.fract() == 0.0 && value.abs() < 8.507_059_173_023_462e37).then(|| value as i128)
}

/// Equality by mathematical value, exactly: integers against integers, an
/// integral binary64 exactly against an integer (never by rounding the
/// integer), and binary64 values by value (so -0 equals 0).
fn number_equal(left: &Number, right: &Number) -> bool {
    match (integer_value(left), integer_value(right)) {
        (Some(left), Some(right)) => left == right,
        (Some(integer), None) => right.as_f64().and_then(integral_value) == Some(integer),
        (None, Some(integer)) => left.as_f64().and_then(integral_value) == Some(integer),
        (None, None) => match (left.as_f64(), right.as_f64()) {
            (Some(left), Some(right)) => left == right,
            _ => false,
        },
    }
}

/// A hash consistent with [`json_equal`]: equal values hash equally. Numbers
/// hash by exact value (an integral value as its integer), objects by sorted
/// member names.
fn hash_json(value: &Value, state: &mut impl std::hash::Hasher) {
    use std::hash::Hash;
    match value {
        Value::Null => 0u8.hash(state),
        Value::Bool(flag) => (1u8, *flag).hash(state),
        Value::Number(number) => {
            2u8.hash(state);
            match integer_value(number).or_else(|| number.as_f64().and_then(integral_value)) {
                Some(integer) => (0u8, integer).hash(state),
                None => (1u8, number.as_f64().map(f64::to_bits)).hash(state),
            }
        }
        Value::String(text) => (3u8, text).hash(state),
        Value::Array(items) => {
            (4u8, items.len()).hash(state);
            for item in items {
                hash_json(item, state);
            }
        }
        Value::Object(members) => {
            (5u8, members.len()).hash(state);
            let mut names: Vec<&String> = members.keys().collect();
            names.sort();
            for name in names {
                name.hash(state);
                hash_json(&members[name], state);
            }
        }
    }
}

/// `uniqueItems` in near-linear work: items are bucketed by [`hash_json`] and
/// compared with [`json_equal`] only within a bucket.
fn has_duplicate(items: &[Value]) -> bool {
    use std::hash::{BuildHasher, Hasher};
    let seed = std::collections::hash_map::RandomState::new();
    let mut buckets: std::collections::HashMap<u64, Vec<usize>> = std::collections::HashMap::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        let mut hasher = seed.build_hasher();
        hash_json(item, &mut hasher);
        let bucket = buckets.entry(hasher.finish()).or_default();
        if bucket.iter().any(|&earlier| {
            #[cfg(test)]
            UNIQUE_COMPARISONS.with(|count| count.set(count.get() + 1));
            json_equal(&items[earlier], item)
        }) {
            return true;
        }
        bucket.push(index);
    }
    false
}

#[cfg(test)]
thread_local! {
    /// Equality comparisons made by [`has_duplicate`] (work-bound test).
    static UNIQUE_COMPARISONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn is_known_keyword(key: &str) -> bool {
    CORE_KEYWORDS.contains(&key)
        || ANNOTATION_KEYWORDS.contains(&key)
        || COMMENT_KEYWORDS.contains(&key)
        || EVALUATED_KEYWORDS.contains(&key)
}

fn escape_pointer(name: &str) -> String {
    name.replace('~', "~0").replace('/', "~1")
}

fn invalid(path: &str, keyword: &str) -> SchemaFailure {
    SchemaFailure::Invalid {
        instance_path: if path.is_empty() { "/".to_string() } else { path.to_string() },
        keyword: keyword.to_string(),
    }
}

fn unsupported_value(keyword: &str) -> SchemaFailure {
    SchemaFailure::Unsupported { detail: format!("malformed {keyword} value") }
}

/// Every keyword name used anywhere in `schema`, walking subschemas and the
/// map-valued keywords' members (the census, §2.3 item 1).
pub fn keyword_census(schema: &Value, out: &mut std::collections::BTreeSet<String>) {
    fn walk_schema(schema: &Value, out: &mut std::collections::BTreeSet<String>) {
        let Value::Object(schema) = schema else { return };
        for (key, value) in schema {
            out.insert(key.clone());
            match key.as_str() {
                "properties" | "$defs" => {
                    if let Some(map) = value.as_object() {
                        for subschema in map.values() {
                            walk_schema(subschema, out);
                        }
                    }
                }
                "allOf" | "anyOf" | "oneOf" => {
                    if let Some(branches) = value.as_array() {
                        for branch in branches {
                            walk_schema(branch, out);
                        }
                    }
                }
                "items" | "contains" | "not" | "if" | "then" | "else" | "additionalProperties" => {
                    walk_schema(value, out)
                }
                _ => {}
            }
        }
    }
    walk_schema(schema, out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn inline(schema: Value) -> Catalog {
        Catalog::new(vec![("inline.schema.json".to_string(), schema)])
    }

    fn verdict(schema: Value, instance: Value) -> Result<(), SchemaFailure> {
        inline(schema).validate("inline.schema.json", &instance)
    }

    #[test]
    fn unknown_keywords_and_patterns_fail_closed() {
        assert!(matches!(verdict(json!({"minContains": 1}), json!([])), Err(SchemaFailure::Unsupported { .. })));
        assert!(matches!(verdict(json!({"pattern": "^a+$"}), json!("a")), Err(SchemaFailure::Unsupported { .. })));
        assert!(matches!(verdict(json!({"$ref": "absent.json"}), json!(1)), Err(SchemaFailure::Unsupported { .. })));
        // An unsupported construct inside a branch is not read as a mismatch.
        assert!(matches!(
            verdict(json!({"anyOf": [{"minContains": 1}, true]}), json!([])),
            Err(SchemaFailure::Unsupported { .. })
        ));
        assert!(matches!(verdict(json!({"not": {"unevaluatedProperties": false}}), json!({})), Err(SchemaFailure::Unsupported { .. })));
    }

    #[test]
    fn reference_cycles_without_progress_fail_closed() {
        let catalog = inline(json!({"$defs": {"a": {"$ref": "#/$defs/a"}}, "$ref": "#/$defs/a"}));
        assert!(matches!(catalog.validate("inline.schema.json", &json!(1)), Err(SchemaFailure::Unsupported { .. })));
    }

    /// No implicit array cap: a schema-unbounded array longer than the old
    /// shape checker's 16384-item cap is accepted (round-3 review B1).
    #[test]
    fn unbounded_arrays_have_no_implicit_cap() {
        let items: Vec<Value> = (0..20_000).map(|index| json!(index)).collect();
        assert_eq!(verdict(json!({"type": "array", "items": {"type": "number"}}), Value::Array(items)), Ok(()));
    }

    /// `uniqueItems` work is near-linear: 100k distinct strings (the shape of
    /// the unbounded `member_connector_ids`) need almost no equality
    /// comparisons, where pairwise checking needs about 5e9; a duplicate at
    /// the end is still found. Work is counted, not timed.
    #[test]
    fn unique_items_work_is_near_linear() {
        let schema = json!({"type": "array", "uniqueItems": true, "items": {"type": "string"}});
        let mut items: Vec<Value> = (0..100_000).map(|index| json!(format!("connector:{index:06}"))).collect();
        UNIQUE_COMPARISONS.with(|count| count.set(0));
        assert_eq!(verdict(schema.clone(), Value::Array(items.clone())), Ok(()));
        let comparisons = UNIQUE_COMPARISONS.with(|count| count.get());
        assert!(comparisons < 1_000, "{comparisons} comparisons for 100000 distinct items");
        items.push(json!("connector:000042"));
        assert!(matches!(verdict(schema, Value::Array(items)), Err(SchemaFailure::Invalid { keyword, .. }) if keyword == "uniqueItems"));
        // Equal by value across representations: 1 and 1.0, member order.
        assert!(has_duplicate(&[json!(1), json!(1.0)]));
        assert!(has_duplicate(&[json!({"a": 1, "b": 2}), json!({"b": 2.0, "a": 1})]));
        assert!(!has_duplicate(&[json!(9007199254740993u64), json!(9007199254740992.0)]));
    }

    #[test]
    fn number_equality_is_exact() {
        let parse = |text: &str| serde_json::from_str::<Value>(text).unwrap();
        assert!(!json_equal(&parse("9007199254740993"), &parse("9007199254740992.0")));
        assert!(json_equal(&parse("9007199254740992"), &parse("9007199254740992.0")));
        assert!(!json_equal(&parse("18446744073709551615"), &parse("18446744073709551616.0")));
        assert!(json_equal(&parse("-0.0"), &parse("0")));
        assert!(!json_equal(&parse("1.5"), &parse("1")));
    }

    #[test]
    fn marks_skip_failed_branches_and_unmarked_members() {
        let catalog = inline(json!({
            "$defs": {"Record": {"type": "object", "required": ["name"]}},
            "type": "object",
            "properties": {
                "record": {"$ref": "#/$defs/Record"},
                "either": {"anyOf": [{"$ref": "#/$defs/Record", "required": ["never"]}, true]},
                "free": {"type": "object"}
            }
        }));
        let found = catalog
            .validate_marking("inline.schema.json", &json!({"record": {"name": "a"}, "either": {"name": "b"}, "free": {"record": {"name": ""}}}), "inline.schema.json", "/$defs/Record")
            .unwrap();
        assert_eq!(found, ["/record"]);
    }

    #[test]
    fn json_equality_is_by_value() {
        assert!(json_equal(&json!(1), &json!(1.0)));
        assert!(!json_equal(&json!(1), &json!(true)));
        assert!(json_equal(&json!({"a": 1, "b": [2.0]}), &json!({"b": [2], "a": 1.0})));
        assert!(!json_equal(&json!({"a": 1}), &json!({"a": 1, "b": null})));
    }

    #[test]
    fn embedded_closure_parses_and_its_census_is_known() {
        let catalog = Catalog::results_v03();
        let mut names = std::collections::BTreeSet::new();
        for (_, schema) in catalog.resources() {
            keyword_census(schema, &mut names);
        }
        for name in &names {
            assert!(is_known_keyword(name), "{name}");
        }
    }
}
