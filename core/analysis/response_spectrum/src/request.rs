//! A JSON evaluation request, the input of the `evaluate_spectrum` example: one spectrum record
//! and the queries to evaluate it at, each with its frequency and scale factor. Every member is
//! required; nothing is defaulted.
use crate::record::{QuantityInput, SpectrumRecord};
use crate::refusal::Refusal;
use crate::table::{Location, OrdinateMeasure, QueryFrequency, ScaleFactor, SpectrumTable};
use serde::Deserialize;
use serde_json::{json, Value};

/// The fixed statement every output document carries.
pub const STATEMENT: &str = "Pseudo-spectral acceleration interpolated from the user-entered spectrum table by its \
stated rule on its authored axis, with no extrapolation. It is an evaluation of the table, not a response-spectrum \
analysis result, and it establishes neither physical correctness nor code compliance.";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationRequest {
    pub spectrum: SpectrumRecord,
    pub queries: Vec<QueryInput>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryInput {
    pub frequency: QuantityInput,
    pub scale_factor: f64,
}

fn refusal_json(refusal: &Refusal) -> Value {
    json!({"code": refusal.code.as_str(), "object": refusal.object, "field": refusal.field, "detail": refusal.detail})
}

fn location_json(location: Location) -> Value {
    match location {
        Location::Knot { index } => json!({"knot": index}),
        Location::Segment { lower } => json!({"segment": [lower, lower + 1]}),
    }
}

/// Evaluates a request. A request that does not decode (malformed JSON, a missing, unknown or
/// `null` member) is an `Err` with the decoder's message; an admitted or refused table, and each
/// query's ordinate or refusal, is the `Ok` document.
pub fn evaluate_request_json(text: &str) -> Result<Value, String> {
    let request: EvaluationRequest = serde_json::from_str(text).map_err(|e| format!("request does not decode: {e}"))?;
    let table = match SpectrumTable::admit(&request.spectrum) {
        Ok(table) => table,
        Err(refusals) => {
            return Ok(json!({
                "statement": STATEMENT,
                "spectrum": request.spectrum.id,
                "admitted": false,
                "refusals": refusals.iter().map(refusal_json).collect::<Vec<_>>(),
            }))
        }
    };
    let measure = match table.ordinate_measure() {
        OrdinateMeasure::MetersPerSecondSquared => "m/s^2",
        OrdinateMeasure::GMultiple => "g multiple",
    };
    let results: Vec<Value> = request
        .queries
        .iter()
        .map(|query| {
            let echo = json!({"frequency": {"value": query.frequency.value, "unit": query.frequency.unit},
                "scale_factor": query.scale_factor});
            let outcome = QueryFrequency::new(query.frequency.value, &query.frequency.unit).and_then(|frequency| {
                let scale = ScaleFactor::new(query.scale_factor)?;
                table.evaluate(frequency, scale).map(|ordinate| (frequency, ordinate))
            });
            match outcome {
                Ok((frequency, ordinate)) => json!({
                    "query": echo,
                    "outcome": "evaluated",
                    "frequency_hz": frequency.hertz(),
                    "location": location_json(ordinate.location),
                    "interpolated_ordinate": {"value": ordinate.interpolated, "measure": measure},
                    "pseudo_acceleration_m_per_s2": ordinate.pseudo_acceleration_m_per_s2,
                    "scaled_pseudo_acceleration_m_per_s2": ordinate.scaled_pseudo_acceleration_m_per_s2,
                }),
                Err(refusal) => json!({"query": echo, "outcome": "refused", "refusal": refusal_json(&refusal)}),
            }
        })
        .collect();
    Ok(json!({
        "statement": STATEMENT,
        "spectrum": table.id(),
        "admitted": true,
        "damping_ratio": table.damping_ratio(),
        "results": results,
    }))
}
