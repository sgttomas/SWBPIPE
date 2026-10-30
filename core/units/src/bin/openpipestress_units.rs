//! Explicit batch bridge to the existing unit authority. No conversion catalog
//! or formula is defined here; the library remains the sole conversion owner.
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, DIMENSIONS};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{self, Read};
use std::process::ExitCode;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    protocol_version: String,
    quantities: Vec<Quantity>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Quantity {
    id: String,
    value: f64,
    unit: String,
    dimension: String,
}
#[derive(Serialize)]
struct Response {
    protocol_version: &'static str,
    quantities: Vec<Converted>,
}
#[derive(Serialize)]
struct Converted {
    id: String,
    value: f64,
    unit: &'static str,
    dimension: &'static str,
}
fn execute(input: &str) -> Result<Response, String> {
    let request: Request =
        serde_json::from_str(input).map_err(|e| format!("UNITS_REQUEST_INVALID: {e}"))?;
    if request.protocol_version != "1.0.0" {
        return Err("UNITS_PROTOCOL_UNSUPPORTED".into());
    }
    let mut ids = HashSet::new();
    let mut quantities = Vec::with_capacity(request.quantities.len());
    for q in request.quantities {
        if q.id.is_empty() || !ids.insert(q.id.clone()) || !q.value.is_finite() {
            return Err("UNITS_QUANTITY_INVALID".into());
        }
        let dimension = DIMENSIONS
            .iter()
            .copied()
            .find(|d| d.as_str() == q.dimension)
            .ok_or("UNITS_DIMENSION_UNSUPPORTED")?;
        let from = unit_by_symbol(&q.unit, dimension)
            .map_err(|e| format!("UNITS_CONVERSION_REFUSED: {e}"))?;
        let target = canonical_unit(dimension).ok_or("UNITS_CANONICAL_UNIT_UNAVAILABLE")?;
        let value = convert_for_dimension(q.value, dimension, from, target)
            .map_err(|e| format!("UNITS_CONVERSION_REFUSED: {e}"))?;
        if !value.is_finite() {
            return Err("UNITS_CONVERSION_NONFINITE".into());
        }
        quantities.push(Converted {
            id: q.id,
            value,
            unit: target.definition().symbol,
            dimension: dimension.as_str(),
        });
    }
    Ok(Response {
        protocol_version: "1.0.0",
        quantities,
    })
}
fn main() -> ExitCode {
    if std::env::args().len() != 1 {
        eprintln!("usage: openpipestress_units < batch.json");
        return ExitCode::from(2);
    }
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("UNITS_INPUT_UNAVAILABLE: {e}");
        return ExitCode::from(2);
    }
    match execute(&input) {
        Ok(result) => {
            println!(
                "{}",
                serde_json::to_string(&result).expect("finite unit result")
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"protocol_version":"1.0.0","error":error})
            );
            ExitCode::from(1)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversions_are_the_actual_library_result() {
        let response=execute(r#"{"protocol_version":"1.0.0","quantities":[{"id":"E","value":200.0,"unit":"GPa","dimension":"stress"},{"id":"T","value":68.0,"unit":"degF","dimension":"temperature"},{"id":"L","value":1.0,"unit":"in","dimension":"length"}]}"#).unwrap();
        for (q, (v, u, d)) in response.quantities.iter().zip([
            (200., "GPa", open_pipe_stress_units::Dimension::Stress),
            (68., "degF", open_pipe_stress_units::Dimension::Temperature),
            (1., "in", open_pipe_stress_units::Dimension::Length),
        ]) {
            assert_eq!(
                q.value.to_bits(),
                convert_for_dimension(
                    v,
                    d,
                    unit_by_symbol(u, d).unwrap(),
                    canonical_unit(d).unwrap()
                )
                .unwrap()
                .to_bits()
            );
        }
        assert_eq!(response.quantities[0].unit, "Pa");
        assert_eq!(response.quantities[1].unit, "K");
    }
    #[test]
    fn closed_shape_dimension_and_finite_errors_refuse_without_results() {
        for input in [
            r#"{"protocol_version":"2","quantities":[]}"#,
            r#"{"protocol_version":"1.0.0","quantities":[],"unknown":0}"#,
            r#"{"protocol_version":"1.0.0","quantities":[{"id":"E","value":1.0,"unit":"m","dimension":"stress"}]}"#,
            r#"{"protocol_version":"1.0.0","quantities":[{"id":"E","value":1e999,"unit":"Pa","dimension":"stress"}]}"#,
            r#"{"protocol_version":"1.0.0","quantities":[{"id":"E","value":true,"unit":"Pa","dimension":"stress"}]}"#,
            r#"{"protocol_version":"1.0.0","protocol_version":"1.0.0","quantities":[]}"#,
        ] {
            assert!(execute(input).is_err(), "{input}");
        }
    }
    #[test]
    fn same_unit_roundtrip_preserves_library_binary64_behavior() {
        let result=execute(r#"{"protocol_version":"1.0.0","quantities":[{"id":"zero","value":-0.0,"unit":"Pa","dimension":"stress"},{"id":"tiny","value":1.2345678901234567e-7,"unit":"Pa","dimension":"stress"}]}"#).unwrap();
        let encoded = serde_json::to_string(&result).unwrap();
        let decoded: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        for (i, q) in result.quantities.iter().enumerate() {
            assert_eq!(
                decoded["quantities"][i]["value"]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                q.value.to_bits()
            );
        }
    }
}
