//! Pure display conversion facade. Stored engineering quantities are never mutated.
use open_pipe_stress_units::{convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};

/// Convert each item independently through the accepted unit catalog. Unknown
/// dimensions/units and non-finite results remain explicit unavailable entries.
pub fn convert_display_quantities(input: &Value) -> Value {
    let Some(items) = input.get("items").and_then(Value::as_array) else {
        return json!({"items":[],"error":{"code":"DISPLAY-INPUT-INVALID","message":"items must be an array"}});
    };
    let items: Vec<Value> = items.iter().map(|item| {
        let id = item.get("id").cloned().unwrap_or(Value::Null);
        let unavailable = |message: String| json!({"id":id,"status":"unavailable","message":message});
        let Some(record) = item.as_object() else { return unavailable("item must be an object".into()); };
        if record.keys().any(|key| !matches!(key.as_str(), "id"|"value"|"from_unit"|"to_unit"|"dimension_id")) {
            return unavailable("unsupported display quantity field; pressure reference changes are not display conversions".into());
        }
        if !item.get("id").is_some_and(Value::is_string) { return unavailable("id must be a string".into()); }
        let Some(value) = item.get("value").and_then(Value::as_f64).filter(|v| v.is_finite()) else { return unavailable("value must be finite".into()); };
        let Some(dimension_id) = item.get("dimension_id").and_then(Value::as_str) else { return unavailable("dimension_id must be explicit".into()); };
        let dimension = match Dimension::from_schema_value(dimension_id) { Ok(value) => value, Err(error) => return unavailable(error.to_string()) };
        let Some(from_symbol) = item.get("from_unit").and_then(Value::as_str) else { return unavailable("from_unit must be explicit".into()); };
        let Some(to_symbol) = item.get("to_unit").and_then(Value::as_str) else { return unavailable("to_unit must be explicit".into()); };
        let from = match unit_by_symbol(from_symbol, dimension) { Ok(value) => value, Err(error) => return unavailable(error.to_string()) };
        let to = match unit_by_symbol(to_symbol, dimension) { Ok(value) => value, Err(error) => return unavailable(error.to_string()) };
        match convert_for_dimension(value, dimension, from, to) {
            Ok(converted) if converted.is_finite() => json!({"id":id,"status":"converted","value":converted,"unit":to_symbol}),
            Ok(_) => unavailable("display conversion result is not finite".into()),
            Err(error) => unavailable(error.to_string()),
        }
    }).collect();
    json!({"items":items})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_conversion_is_pure_and_distinguishes_absolute_and_interval_temperature() {
        let input = json!({"items":[
            {"id":"length","value":1,"from_unit":"m","to_unit":"mm","dimension_id":"length"},
            {"id":"absolute","value":0,"from_unit":"degC","to_unit":"degF","dimension_id":"temperature"},
            {"id":"interval","value":10,"from_unit":"degC","to_unit":"degF","dimension_id":"temperature_interval"},
            {"id":"wrong","value":1,"from_unit":"m","to_unit":"Pa","dimension_id":"length"},
            {"id":"unknown","value":1,"from_unit":"TBD","to_unit":"mm","dimension_id":"length"}
        ]});
        let original = input.clone();
        let output = convert_display_quantities(&input);
        assert_eq!(input, original);
        assert_eq!(output["items"][0]["value"], 1000.0);
        assert!((output["items"][1]["value"].as_f64().unwrap() - 32.0).abs() < 1e-8);
        assert!((output["items"][2]["value"].as_f64().unwrap() - 18.0).abs() < 1e-8);
        assert_eq!(output["items"][3]["status"], "unavailable");
        assert_eq!(output["items"][4]["status"], "unavailable");
    }
    #[test]
    fn display_conversion_rejects_pressure_reference_payload_and_overflow() {
        let output = convert_display_quantities(&json!({"items":[
            {"id":"pressure","value":1,"from_unit":"Pa","to_unit":"psi","dimension_id":"pressure","reference":"gauge"},
            {"id":"overflow","value":1e308,"from_unit":"m","to_unit":"mm","dimension_id":"length"}
        ]}));
        assert!(output["items"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["status"] == "unavailable"));
    }
}
