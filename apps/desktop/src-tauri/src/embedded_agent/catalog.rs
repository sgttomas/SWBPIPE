//! SWBPIPE's embedded capability contract. Callers must admit and reserve
//! resources for untrusted bytes before constructing a `Value`. JSON numeric
//! tokens may round when materialized as binary64; the caller must also check
//! their exact lexical values against integer and boundary constraints before
//! materialization. This module validates the resulting value structure only;
//! session binding and operation semantics belong to the dispatcher and the
//! native operation engine.

use serde_json::{json, Map, Number, Value};
use std::collections::{HashMap, HashSet};

const SOURCE: &str =
    include_str!("../../../../../execution/PKG-16/DEL-16-06/Design/agent-capability-catalog.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tool {
    Summary,
    Nodes,
    ProposeOperations,
    ObserveProposal,
    Solve,
    ReadResults,
}

impl Tool {
    pub const ALL: [Self; 6] = [
        Self::Summary,
        Self::Nodes,
        Self::ProposeOperations,
        Self::ObserveProposal,
        Self::Solve,
        Self::ReadResults,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Summary => "swbpipe_read_model_summary",
            Self::Nodes => "swbpipe_read_nodes",
            Self::ProposeOperations => "swbpipe_propose_operations",
            Self::ObserveProposal => "swbpipe_observe_proposal",
            Self::Solve => "swbpipe_solve",
            Self::ReadResults => "swbpipe_read_results",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tool| tool.name() == name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Effect {
    Read,
    ProposalOnly,
    Solve,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub argument_bytes: u64,
    pub result_bytes: u64,
    pub page_rows: u64,
    pub pending_calls: u64,
    pub proposal_operations: u64,
}

/// Bounded diagnostic: neither catalog data nor caller-supplied values are echoed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error(&'static str);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for Error {}

struct Descriptor {
    description: String,
    effect: Effect,
    input: Value,
    output: Value,
}
pub struct Catalog {
    limits: Limits,
    tools: HashMap<&'static str, Descriptor>,
}

impl Catalog {
    pub fn load() -> Result<Self, Error> {
        Self::from_source(SOURCE)
    }

    fn from_source(source: &str) -> Result<Self, Error> {
        let root: Value =
            serde_json::from_str(source).map_err(|_| Error("invalid catalog JSON"))?;
        if root.get("catalogVersion").and_then(Value::as_str) != Some("swbpipe-catalog/2")
            || root.get("schemaDialect").and_then(Value::as_str)
                != Some("https://json-schema.org/draft/2020-12/schema")
        {
            return Err(Error("unsupported catalog version or dialect"));
        }
        let defs = root
            .get("definitions")
            .and_then(Value::as_object)
            .ok_or(Error("invalid catalog definitions"))?;
        // Check even unreferenced definitions, so later use cannot reveal a silently
        // ignored assertion or a broken/cyclic reference.
        for schema in defs.values() {
            expand(schema, defs, &mut Vec::new())?;
        }
        let limit = root
            .get("limits")
            .and_then(Value::as_object)
            .ok_or(Error("invalid catalog limits"))?;
        let get_limit = |key: &str| -> Result<u64, Error> {
            limit
                .get(key)
                .and_then(Value::as_u64)
                .filter(|n| *n > 0)
                .ok_or(Error("invalid catalog limit"))
        };
        let limits = Limits {
            argument_bytes: get_limit("argumentBytes")?,
            result_bytes: get_limit("resultBytes")?,
            page_rows: get_limit("pageRows")?,
            pending_calls: get_limit("pendingCalls")?,
            proposal_operations: get_limit("proposalOperations")?,
        };
        let entries = root
            .get("tools")
            .and_then(Value::as_array)
            .ok_or(Error("invalid catalog tools"))?;
        let mut tools = HashMap::new();
        for entry in entries {
            let name = entry
                .get("name")
                .and_then(Value::as_str)
                .ok_or(Error("invalid tool name"))?;
            let tool = Tool::from_name(name).ok_or(Error("unsupported tool"))?;
            let effect = match entry.get("effect").and_then(Value::as_str) {
                Some("read") => Effect::Read,
                Some("proposal_only") => Effect::ProposalOnly,
                Some("solve") => Effect::Solve,
                _ => return Err(Error("unsupported tool effect")),
            };
            let expected = match tool {
                Tool::ProposeOperations => Effect::ProposalOnly,
                Tool::Solve => Effect::Solve,
                _ => Effect::Read,
            };
            if effect != expected {
                return Err(Error("unexpected tool effect"));
            }
            let description = entry
                .get("description")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
                .ok_or(Error("invalid tool description"))?
                .to_owned();
            let input = expand(
                entry
                    .get("inputSchema")
                    .ok_or(Error("missing input schema"))?,
                defs,
                &mut Vec::new(),
            )?;
            let output = expand(
                entry
                    .get("outputSchema")
                    .ok_or(Error("missing output schema"))?,
                defs,
                &mut Vec::new(),
            )?;
            if tools
                .insert(
                    tool.name(),
                    Descriptor {
                        description,
                        effect,
                        input,
                        output,
                    },
                )
                .is_some()
            {
                return Err(Error("duplicate tool"));
            }
        }
        if tools.len() != Tool::ALL.len() {
            return Err(Error("missing tool"));
        }
        Ok(Self { limits, tools })
    }

    pub fn limits(&self) -> Limits {
        self.limits
    }
    pub fn effect(&self, tool: Tool) -> Effect {
        self.tools[tool.name()].effect
    }

    /// Standalone stock app-server declarations; outputs remain internal.
    pub fn declarations(&self) -> Vec<Value> {
        Tool::ALL.into_iter().map(|tool| {
            let d = &self.tools[tool.name()];
            json!({"type": "function", "name": tool.name(), "description": d.description, "inputSchema": d.input})
        }).collect()
    }

    pub fn validate_input(&self, tool: Tool, value: &Value) -> Result<(), Error> {
        if matches_schema(&self.tools[tool.name()].input, value) {
            Ok(())
        } else {
            Err(Error("input does not match catalog schema"))
        }
    }
    pub fn validate_output(&self, tool: Tool, value: &Value) -> Result<(), Error> {
        if matches_schema(&self.tools[tool.name()].output, value) {
            Ok(())
        } else {
            Err(Error("output does not match catalog schema"))
        }
    }

    /// After structural validation, enforce host proposal context policy. The
    /// effective cap is host configuration, reported by summary. The catalog's
    /// count is a default, not a schema constant. Bytes remain separately capped
    /// before materialization by the host's actual admission budget.
    pub fn validate_proposal_context(
        &self,
        value: &Value,
        effective_operation_cap: u64,
    ) -> Result<(), Error> {
        self.validate_input(Tool::ProposeOperations, value)?;
        let operations = value["operations"]
            .as_array()
            .ok_or(Error("invalid proposal context"))?;
        if effective_operation_cap == 0 || operations.len() as u64 > effective_operation_cap {
            return Err(Error("proposal operation limit exceeded"));
        }
        let ids: HashSet<&str> = operations
            .iter()
            .filter_map(|op| op["operation_id"].as_str())
            .collect();
        let mut assumption_ids = HashSet::new();
        for assumption in value["assumptions"]
            .as_array()
            .ok_or(Error("invalid proposal context"))?
        {
            let id = assumption["assumptionId"]
                .as_str()
                .ok_or(Error("invalid proposal context"))?;
            if !assumption_ids.insert(id) {
                return Err(Error("duplicate assumption id"));
            }
            for affected in assumption["affectedOperationIds"]
                .as_array()
                .ok_or(Error("invalid proposal context"))?
            {
                if !affected.as_str().is_some_and(|id| ids.contains(id)) {
                    return Err(Error("unknown affected operation"));
                }
            }
        }
        Ok(())
    }
}

// Expand and validate the controlled subset at load, rejecting unrecognized
// keywords (including annotations until deliberately supported). $ref siblings
// are rejected explicitly rather than losing their 2020-12 meaning.
fn expand(
    schema: &Value,
    defs: &Map<String, Value>,
    stack: &mut Vec<String>,
) -> Result<Value, Error> {
    let obj = schema
        .as_object()
        .ok_or(Error("schema must be an object"))?;
    if let Some(reference) = obj.get("$ref") {
        if obj.len() != 1 {
            return Err(Error("reference siblings unsupported"));
        }
        let name = reference
            .as_str()
            .and_then(|s| s.strip_prefix("#/definitions/"))
            .filter(|s| !s.is_empty() && !s.contains('/') && !s.contains('~'))
            .ok_or(Error("unsupported schema reference"))?;
        if stack.iter().any(|s| s == name) {
            return Err(Error("cyclic schema reference"));
        }
        let target = defs.get(name).ok_or(Error("unresolved schema reference"))?;
        stack.push(name.to_owned());
        let result = expand(target, defs, stack);
        stack.pop();
        return result;
    }
    let mut out = Map::new();
    for (key, value) in obj {
        let expanded = match key.as_str() {
            "type" => {
                let valid_type = |v: &Value| {
                    matches!(
                        v.as_str(),
                        Some(
                            "object"
                                | "array"
                                | "string"
                                | "number"
                                | "integer"
                                | "boolean"
                                | "null"
                        )
                    )
                };
                if !(valid_type(value)
                    || value
                        .as_array()
                        .is_some_and(|a| !a.is_empty() && a.iter().all(valid_type)))
                {
                    return Err(Error("unsupported schema type"));
                }
                value.clone()
            }
            "properties" => {
                let props = value
                    .as_object()
                    .ok_or(Error("invalid schema properties"))?;
                let mut result = Map::new();
                for (name, sub) in props {
                    result.insert(name.clone(), expand(sub, defs, stack)?);
                }
                Value::Object(result)
            }
            "additionalProperties" => {
                if value.is_boolean() {
                    value.clone()
                } else {
                    expand(value, defs, stack)?
                }
            }
            "items" => expand(value, defs, stack)?,
            "anyOf" => {
                let choices = value
                    .as_array()
                    .filter(|a| !a.is_empty())
                    .ok_or(Error("invalid schema anyOf"))?;
                Value::Array(
                    choices
                        .iter()
                        .map(|s| expand(s, defs, stack))
                        .collect::<Result<_, _>>()?,
                )
            }
            "required" => {
                let names = value.as_array().ok_or(Error("invalid schema required"))?;
                let mut seen = HashSet::new();
                if !names
                    .iter()
                    .all(|v| v.as_str().is_some_and(|s| seen.insert(s)))
                {
                    return Err(Error("invalid schema required"));
                }
                value.clone()
            }
            "enum" => {
                if !value.as_array().is_some_and(|a| !a.is_empty()) {
                    return Err(Error("invalid schema enum"));
                }
                value.clone()
            }
            "const" => value.clone(),
            "minItems" | "maxItems" | "minLength" | "maxLength" | "maxProperties" => {
                if value.as_u64().is_none() {
                    return Err(Error("invalid schema count"));
                }
                value.clone()
            }
            "minimum" | "maximum" => {
                // The catalog currently has small integral boundaries. Refuse a
                // future boundary that cannot be compared exactly to binary64.
                if !value
                    .as_i64()
                    .is_some_and(|n| (-9_007_199_254_740_992..=9_007_199_254_740_992).contains(&n))
                {
                    return Err(Error("unsupported numeric boundary"));
                }
                value.clone()
            }
            "pattern" => {
                if value.as_str() != Some("^[0-9a-f]{64}$") {
                    return Err(Error("unsupported schema pattern"));
                }
                value.clone()
            }
            "uniqueItems" => {
                if !value.is_boolean() {
                    return Err(Error("invalid uniqueItems"));
                }
                value.clone()
            }
            _ => return Err(Error("unsupported schema keyword")),
        };
        out.insert(key.clone(), expanded);
    }
    Ok(Value::Object(out))
}

fn number_is_integer(n: &Number) -> bool {
    n.is_i64()
        || n.is_u64()
        || n.as_f64()
            .is_some_and(|f| f.is_finite() && f.fract() == 0.0)
}
fn type_matches(kind: &str, value: &Value) -> bool {
    match kind {
        "object" => value.is_object(),
        "array" => value.is_array(),
        "string" => value.is_string(),
        "number" => value.is_number(),
        "integer" => value.as_number().is_some_and(number_is_integer),
        "boolean" => value.is_boolean(),
        "null" => value.is_null(),
        _ => false,
    }
}
// Mathematical equality for JSON numbers, including integer/float values,
// without rounding a large integer through f64.
fn number_equal(a: &Number, b: &Number) -> bool {
    if let (Some(x), Some(y)) = (a.as_i64(), b.as_i64()) {
        return x == y;
    }
    if let (Some(x), Some(y)) = (a.as_u64(), b.as_u64()) {
        return x == y;
    }
    fn float_integer(f: f64, n: &Number) -> bool {
        if !f.is_finite() || f.fract() != 0.0 {
            return false;
        }
        if f >= 0.0 && f < 18_446_744_073_709_551_616.0 {
            n.as_u64().is_some_and(|x| x == f as u64)
        } else if f >= -9_223_372_036_854_775_808.0 && f < 0.0 {
            n.as_i64().is_some_and(|x| x == f as i64)
        } else {
            false
        }
    }
    if a.is_f64() && !b.is_f64() {
        return float_integer(a.as_f64().unwrap(), b);
    }
    if b.is_f64() && !a.is_f64() {
        return float_integer(b.as_f64().unwrap(), a);
    }
    a.is_f64() && b.is_f64() && a.as_f64() == b.as_f64()
}
fn json_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => number_equal(x, y),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| json_equal(a, b))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|w| json_equal(v, w)))
        }
        _ => a == b,
    }
}
fn number_at_least(n: &Number, bound: i64) -> bool {
    if let Some(i) = n.as_i64() {
        i >= bound
    } else if let Some(u) = n.as_u64() {
        bound < 0 || u >= bound as u64
    } else {
        n.as_f64().is_some_and(|f| f >= bound as f64)
    }
}
fn number_at_most(n: &Number, bound: i64) -> bool {
    if let Some(i) = n.as_i64() {
        i <= bound
    } else if let Some(u) = n.as_u64() {
        bound >= 0 && u <= bound as u64
    } else {
        n.as_f64().is_some_and(|f| f <= bound as f64)
    }
}
fn matches_schema(schema: &Value, value: &Value) -> bool {
    let s = schema.as_object().expect("expanded schema is validated");
    if let Some(types) = s.get("type") {
        let valid = if let Some(t) = types.as_str() {
            type_matches(t, value)
        } else {
            types
                .as_array()
                .unwrap()
                .iter()
                .any(|t| type_matches(t.as_str().unwrap(), value))
        };
        if !valid {
            return false;
        }
    }
    if s.get("const").is_some_and(|v| !json_equal(v, value)) {
        return false;
    }
    if s.get("enum")
        .is_some_and(|v| !v.as_array().unwrap().iter().any(|x| json_equal(x, value)))
    {
        return false;
    }
    if s.get("anyOf").is_some_and(|v| {
        !v.as_array()
            .unwrap()
            .iter()
            .any(|x| matches_schema(x, value))
    }) {
        return false;
    }
    if let Some(n) = value.as_number() {
        if s.get("minimum")
            .is_some_and(|v| !number_at_least(n, v.as_i64().unwrap()))
            || s.get("maximum")
                .is_some_and(|v| !number_at_most(n, v.as_i64().unwrap()))
        {
            return false;
        }
    }
    if let Some(text) = value.as_str() {
        let len = text.chars().count() as u64;
        if s.get("minLength")
            .is_some_and(|v| len < v.as_u64().unwrap())
            || s.get("maxLength")
                .is_some_and(|v| len > v.as_u64().unwrap())
        {
            return false;
        }
        if s.contains_key("pattern")
            && !(text.len() == 64
                && text
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
        {
            return false;
        }
    }
    if let Some(array) = value.as_array() {
        let len = array.len() as u64;
        if s.get("minItems").is_some_and(|v| len < v.as_u64().unwrap())
            || s.get("maxItems").is_some_and(|v| len > v.as_u64().unwrap())
        {
            return false;
        }
        if let Some(item) = s.get("items") {
            if !array.iter().all(|v| matches_schema(item, v)) {
                return false;
            }
        }
        if s.get("uniqueItems") == Some(&Value::Bool(true)) {
            for i in 0..array.len() {
                if array[..i].iter().any(|v| json_equal(v, &array[i])) {
                    return false;
                }
            }
        }
    }
    if let Some(object) = value.as_object() {
        if s.get("maxProperties")
            .is_some_and(|v| object.len() as u64 > v.as_u64().unwrap())
        {
            return false;
        }
        if s.get("required").is_some_and(|v| {
            !v.as_array()
                .unwrap()
                .iter()
                .all(|name| object.contains_key(name.as_str().unwrap()))
        }) {
            return false;
        }
        let props = s.get("properties").and_then(Value::as_object);
        for (name, item) in object {
            if let Some(sub) = props.and_then(|p| p.get(name)) {
                if !matches_schema(sub, item) {
                    return false;
                }
            } else if let Some(additional) = s.get("additionalProperties") {
                if additional == &Value::Bool(false)
                    || (additional.is_object() && !matches_schema(additional, item))
                {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash() -> Value {
        json!({"algorithm":"sha256", "canonicalization":"rfc8785_jcs", "payload_scope":"model_payload", "payload_ref":"model", "value":"a".repeat(64), "hash_status":"computed_local_preview"})
    }
    fn basis() -> Value {
        json!({"workspaceHandle":"workspace", "applicationGeneration":1, "modelRevision":0, "modelHash":hash(), "stateId":null, "runId":null})
    }
    fn operation() -> Value {
        json!({"operation_id":"op1", "operation_kind":"modify", "operation_status":"proposed", "author_type":"agent",
            "target":{"object_type":"Node", "ref":"n1"},
            "change":{"change_id":"c1", "change_kind":"set_field", "field_label":"x", "field_path":"position.x", "before":"0", "after":"1", "unit":"m", "dimension":"length", "source_note":"source"},
            "validation":{"schema_validation":"", "constraint_validation":"", "unit_validation":"", "diff_preview_status":"", "application_status":""},
            "audit_boundary":{"mutation_route":"structured_operations_only", "direct_model_mutation_allowed":false, "requires_user_acceptance":true, "mutates_accepted_model_state":false},
            "professional_boundary":{"human_review_required":true, "software_makes_compliance_claim":false, "software_makes_certification_claim":false, "software_makes_sealing_claim":false, "software_makes_approval_claim":false, "software_makes_authentication_claim":false}, "rationale":"why"})
    }
    fn proposal() -> Value {
        json!({"workspaceHandle":"workspace", "basis":basis(), "claimedModelHash":hash(), "operations":[operation()], "assumptions":[]})
    }
    fn assumption(id: &str, refs: Value) -> Value {
        json!({"assumptionId":id, "statement":"reason", "affectedOperationIds":refs})
    }

    #[test]
    fn catalog_projection_and_empty_summary() {
        let catalog = Catalog::load().unwrap();
        let declarations = catalog.declarations();
        assert_eq!(declarations.len(), 6);
        fn no_refs(value: &Value) -> bool {
            match value {
                Value::Object(o) => !o.contains_key("$ref") && o.values().all(no_refs),
                Value::Array(a) => a.iter().all(no_refs),
                _ => true,
            }
        }
        for (declaration, tool) in declarations.iter().zip(Tool::ALL) {
            let o = declaration.as_object().unwrap();
            assert_eq!(o.len(), 4);
            assert_eq!(o["type"], "function");
            assert_eq!(o["name"], tool.name());
            assert!(o["description"].as_str().is_some_and(|s| !s.is_empty()));
            assert!(no_refs(&o["inputSchema"]));
            assert_eq!(Tool::from_name(tool.name()), Some(tool));
        }
        assert_eq!(
            catalog.effect(Tool::ProposeOperations),
            Effect::ProposalOnly
        );
        assert_eq!(catalog.effect(Tool::Solve), Effect::Solve);
        assert!(catalog.validate_input(Tool::Summary, &json!({})).is_ok());
        for bad in [json!({"workspaceHandle":"x"}), json!(null), json!([])] {
            assert!(catalog.validate_input(Tool::Summary, &bad).is_err());
        }
    }

    #[test]
    fn proposal_native_literals_and_structure() {
        let catalog = Catalog::load().unwrap();
        let valid = proposal();
        assert!(catalog
            .validate_input(Tool::ProposeOperations, &valid)
            .is_ok());
        for (path, replacement) in [
            ("/operations/0/operation_kind", json!("create")),
            ("/operations/0/author_type", json!("human")),
            ("/operations/0/target/object_type", json!("Pipe")),
            ("/operations/0/change/field_path", json!("diameter")),
            (
                "/operations/0/audit_boundary/requires_user_acceptance",
                json!(false),
            ),
        ] {
            let mut bad = valid.clone();
            *bad.pointer_mut(path).unwrap() = replacement;
            assert!(catalog
                .validate_input(Tool::ProposeOperations, &bad)
                .is_err());
        }
        let mut bad = valid.clone();
        bad["operations"][0]["invented"] = json!(true);
        assert!(catalog
            .validate_input(Tool::ProposeOperations, &bad)
            .is_err());
        let mut bad = valid;
        bad.as_object_mut().unwrap().remove("assumptions");
        assert!(catalog
            .validate_input(Tool::ProposeOperations, &bad)
            .is_err());
    }

    #[test]
    fn proposal_context_limits_and_membership() {
        let catalog = Catalog::load().unwrap();
        let mut input = proposal();
        input["assumptions"] = json!([assumption("a", json!(["op1"]))]);
        assert!(catalog.validate_proposal_context(&input, 1).is_ok());
        assert!(catalog.validate_proposal_context(&input, 0).is_err());
        input["assumptions"] = json!([
            assumption("a", json!(["op1"])),
            assumption("a", json!(["op1"]))
        ]);
        assert!(catalog.validate_proposal_context(&input, 16).is_err());
        input["assumptions"] = json!([assumption("a", json!(["absent"]))]);
        assert!(catalog.validate_proposal_context(&input, 16).is_err());
        input["assumptions"] = json!([assumption("a", json!(["op1", "op1"]))]);
        assert!(catalog
            .validate_input(Tool::ProposeOperations, &input)
            .is_err());
        input["assumptions"] = json!((0..17)
            .map(|i| assumption(&i.to_string(), json!(["op1"])))
            .collect::<Vec<_>>());
        assert!(catalog
            .validate_input(Tool::ProposeOperations, &input)
            .is_err());
        input["assumptions"] = json!([]);
        input["operations"] = json!(vec![operation(); 17]);
        assert!(catalog
            .validate_input(Tool::ProposeOperations, &input)
            .is_ok());
        assert!(catalog.validate_proposal_context(&input, 16).is_err());
        assert!(catalog.validate_proposal_context(&input, 17).is_ok());
    }

    #[test]
    fn hashes_pages_and_unicode_lengths() {
        let catalog = Catalog::load().unwrap();
        let mut input = json!({"workspaceHandle":"é".repeat(160), "basis":basis(), "page":{"cursor":null, "limit":50}});
        assert!(catalog.validate_input(Tool::Nodes, &input).is_ok());
        input["workspaceHandle"] = json!("é".repeat(161));
        assert!(catalog.validate_input(Tool::Nodes, &input).is_err());
        input["workspaceHandle"] = json!("workspace");
        for limit in [json!(0), json!(51), json!(1.5), json!(u64::MAX)] {
            input["page"]["limit"] = limit;
            assert!(catalog.validate_input(Tool::Nodes, &input).is_err());
        }
        input["page"]["limit"] = json!(1.0);
        assert!(catalog.validate_input(Tool::Nodes, &input).is_ok());
        for bad in ["A".repeat(64), "a".repeat(63), "g".repeat(64)] {
            input["basis"]["modelHash"]["value"] = json!(bad);
            assert!(catalog.validate_input(Tool::Nodes, &input).is_err());
        }
    }

    #[test]
    fn output_nullable_fields_and_bounds() {
        let catalog = Catalog::load().unwrap();
        let mut output =
            json!({"status":"unavailable", "basis":basis(), "message":"unavailable", "data":null});
        assert!(catalog.validate_output(Tool::ReadResults, &output).is_ok());
        output["data"] = json!({"runHandle":"run", "runId":"id", "evaluatedModelHash":hash(), "values":[{"id":"v", "kind":"stress", "entityRef":"n", "value":null, "exactDecimal":null, "unit":"Pa", "availability":"withheld", "reason":null}], "nextCursor":null, "standing":{"route":"r", "health":"h", "checks":[]}});
        assert!(catalog.validate_output(Tool::ReadResults, &output).is_ok());
        output["data"]["values"][0]["value"] = json!("1");
        assert!(catalog.validate_output(Tool::ReadResults, &output).is_err());
        output["data"]["values"] = json!(vec![
            json!({"id":"v", "kind":"stress", "entityRef":"n", "value":null, "exactDecimal":null, "unit":"Pa", "availability":"withheld", "reason":null});
            51
        ]);
        assert!(catalog.validate_output(Tool::ReadResults, &output).is_err());
        let summary = json!({"status":"ok", "basis":basis(), "message":"ok", "data":{"workspaceHandle":"w", "projectUnits":{"length":"m"}, "objectCounts":{}, "admittedOperationKinds":[], "limits":{"argumentBytes":1, "resultBytes":1, "pageRows":1, "pendingCalls":1, "proposalOperations":1}}});
        // Exercise object additionalProperties separately, regardless of other summary fields.
        let d = &catalog.tools[Tool::Summary.name()].output;
        let counts = d
            .pointer("/properties/data/anyOf/0/properties/objectCounts")
            .unwrap();
        assert!(matches_schema(counts, &json!({"nodes":0, "pipes":12})));
        assert!(!matches_schema(counts, &json!({"nodes":-1})));
        assert!(!matches_schema(
            counts,
            &json!((0..33)
                .map(|i| (i.to_string(), json!(0)))
                .collect::<Map<String, Value>>())
        ));
        assert!(catalog.validate_output(Tool::Summary, &summary).is_ok());
    }

    #[test]
    fn reject_unknown_assertions_bad_references_and_cycles() {
        let mut root: Value = serde_json::from_str(SOURCE).unwrap();
        root["definitions"]["summaryInput"]["not"] = json!({});
        assert!(Catalog::from_source(&root.to_string()).is_err());
        root["definitions"]["summaryInput"]
            .as_object_mut()
            .unwrap()
            .remove("not");
        root["definitions"]["summaryInput"]["$ref"] = json!("#/definitions/absent");
        assert!(Catalog::from_source(&root.to_string()).is_err());
        root["definitions"]["summaryInput"] = json!({"$ref":"#/definitions/summaryInput"});
        assert!(Catalog::from_source(&root.to_string()).is_err());
        root["definitions"]["summaryInput"] = json!({"$ref":"#/definitions/absent"});
        assert!(Catalog::from_source(&root.to_string()).is_err());
    }

    #[test]
    fn numeric_equality_does_not_round_large_integers() {
        let schema = json!({"uniqueItems":true});
        assert!(!matches_schema(&schema, &json!([1, 1.0])));
        assert!(matches_schema(
            &schema,
            &json!([9007199254740993_u64, 9007199254740992.0])
        ));
        assert!(number_at_least(json!(u64::MAX).as_number().unwrap(), 0));
        assert!(!number_at_most(json!(u64::MAX).as_number().unwrap(), 50));
    }
}
