//! Unit R: transport an accepted retained reader verdict to its Rust semantic helpers.
//! The desktop registration binds this verdict to the source it validated. This is
//! not a second receipt reader: no receipt check or standing predicate is repeated.
use open_pipe_stress_product_physics::result_export::{retained_precision as rp, semantic_contract as sc};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct WireValidation {
    invocation_bound: bool,
    numerical_eligible: bool,
    publication_sha256: String,
    classifications: Vec<WireClassification>,
}
#[derive(Deserialize)]
struct WireClassification {
    result_id: String,
    basis_ref: Value,
    normalized_bits: String,
    scale_bits: Option<String>,
    bound_bits: Option<String>,
    class: String,
}
fn invalid(detail: impl std::fmt::Display) -> String {
    format!("WASM-ENGINE-INPUT-VALIDATION-INVALID: {detail}")
}
fn bits(text: &str) -> Result<u64, String> {
    if text.len() != 16 || !text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        return Err(invalid("binary64 word must have 16 lowercase hex digits"));
    }
    let word = u64::from_str_radix(text, 16).map_err(invalid)?;
    if !f64::from_bits(word).is_finite() {
        return Err(invalid("binary64 word must be finite"));
    }
    Ok(word)
}
fn validation(wire: WireValidation) -> Result<rp::Validation, String> {
    if wire.publication_sha256.len() != 64
        || !wire.publication_sha256.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        return Err(invalid("publication_sha256 must have 64 lowercase hex digits"));
    }
    let classifications = wire.classifications.into_iter().map(|row| {
        let bound = row.bound_bits.as_deref().map(bits).transpose()?;
        let class = match (row.class.as_str(), bound) {
            ("relative_verified", None) => rp::AccuracyClass::RelativeVerified,
            ("absolute_verified", Some(bound_bits)) => rp::AccuracyClass::AbsoluteVerified { bound_bits },
            ("input_derived", None) => rp::AccuracyClass::InputDerived,
            ("non_quantity", None) => rp::AccuracyClass::NonQuantity,
            ("not_covered", None) => rp::AccuracyClass::NotCovered,
            _ => return Err(invalid("unknown class or contradictory bound_bits")),
        };
        Ok(rp::RowClassification {
            result_id: row.result_id, basis_ref: row.basis_ref,
            normalized_bits: bits(&row.normalized_bits)?,
            scale_bits: row.scale_bits.as_deref().map(bits).transpose()?, class,
        })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(rp::Validation {
        invocation_bound: wire.invocation_bound, numerical_eligible: wire.numerical_eligible,
        publication_sha256: wire.publication_sha256, classifications,
    })
}

/// Standing and per-case classification summary from an already accepted reader
/// verdict, bound by the caller's registration to these source bytes. Invalid
/// verdict wire is an input error; it never supplies an eligible standing.
pub fn retained_precision_semantics_json(
    source_json: &str, validation_json: &str, requested_json: &str,
) -> Result<String, String> {
    let source = super::parse_reader_input("source", source_json)?;
    let wire = super::parse_reader_input("validation", validation_json)?;
    let validation = validation(serde_json::from_value(wire).map_err(invalid)?)?;
    let requested = super::parse_reader_input("requested", requested_json)?;
    let requested = requested.as_array().ok_or_else(|| invalid("requested refs must be an array"))?;
    Ok(json!({
        "standing": sc::retained_standing_from(&validation, &source, requested),
        "classification_summary": sc::classification_summary_from(&validation, &source, requested),
    }).to_string())
}
