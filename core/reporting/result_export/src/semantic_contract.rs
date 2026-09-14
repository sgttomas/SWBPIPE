//! Exact current producer semantics. No ID substring or unit-only family inference.
use serde_json::Value;
use std::sync::OnceLock;
pub fn contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| serde_json::from_str(include_str!("../../../../fixtures/results/semantic_contract_v0_2.json")).expect("pinned semantic contract"))
}
pub fn signature(row: &Value) -> Result<Option<&'static Value>, String> {
    let kind = row["kind"].as_str().ok_or("SOURCE_KIND_MISSING")?;
    let known: Vec<_> = contract()["rows"].as_array().unwrap().iter().filter(|s| s["kind"] == kind).collect();
    if known.is_empty() { return Ok(None); }
    let units: Vec<_> = known.into_iter().filter(|s| s["unit"] == row["unit"]).collect();
    if units.is_empty() { return Err(format!("SOURCE_UNIT_CONTRADICTION: {kind}")); }
    let component = row["metadata"]["component"].as_str().filter(|s| !s.is_empty());
    let exact = units.iter().find(|s| s["component"].is_null() || s["component"].as_str() == component);
    if let Some(s) = exact { return Ok(Some(s)); }
    if component.is_some() { return Err(format!("SOURCE_COMPONENT_CONTRADICTION: {kind}")); }
    // A missing component cannot identify the variant. This representative is
    // used only to classify the incomplete disclosure, never to emit a target.
    Ok(Some(units[0]))
}
pub fn complete_metadata(row: &Value) -> bool {
    ["component", "coordinate_system", "location", "basis", "sign_convention"].iter().all(|k| row["metadata"][k].as_str().is_some_and(|s| !s.is_empty()))
}
pub fn canonical_metadata(row: &Value) -> Option<Value> {
    if !complete_metadata(row) { return None; }
    let mut projection = serde_json::Map::new();
    for (key, rule) in contract()["canonical_metadata_vocabulary"].as_object().unwrap() {
        let value = &row["metadata"][key];
        if let Some(allowed) = rule["enum"].as_array() { if !allowed.contains(value) { return None; } }
        projection.insert(key.clone(), value.clone());
    }
    Some(Value::Object(projection))
}
