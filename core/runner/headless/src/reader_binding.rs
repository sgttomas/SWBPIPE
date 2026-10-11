//! The product result readers behind `openpipestress-runner read <reader>`.
//!
//! JSON bytes in; the Rust reader's verdict out. Every product caller outside
//! Rust (Python's analysis-run and stress-neutral paths) takes its verdict from
//! here and never re-derives one. A reader is named on the command line and
//! looked up in [`READERS`]; each reads one input object and returns either the
//! reader's `Validation` or its refusal (gate, code and detail).
//!
//! The verdict carries only what the reader returns: flags, the stated
//! publication digest, row ids, basis references, bit patterns and refusal
//! codes. Hashes bind the supplied statements; they never establish producer
//! origin.

use open_pipe_stress_result_export::retained_precision::{
    self as rp, AccuracyClass, Validation, ValidationError,
};
use serde_json::{json, Map, Value};

pub const ARTIFACT: &str = "openpipestress.result_reader_verdict";
pub const SCHEMA_VERSION: &str = "1.0.0";

/// One named reader: the input members it admits and the function it runs.
pub struct Reader {
    pub name: &'static str,
    /// Input members besides the required `source`.
    pub optional_members: &'static [&'static str],
    read: fn(&Map<String, Value>) -> Result<Validation, ValidationError>,
}

fn retained(input: &Map<String, Value>) -> Result<Validation, ValidationError> {
    let invocation = input.get("invocation").filter(|value| !value.is_null());
    rp::validate(&input["source"], invocation)
}

fn retained_transport(input: &Map<String, Value>) -> Result<Validation, ValidationError> {
    rp::validate_transport_metadata(&input["source"])
}

/// The readers this command dispatches to, by name.
pub const READERS: &[Reader] = &[
    // `retained_precision::validate`: `{"source": <successor>, "invocation"?: <{request, solver_mode}> | null}`.
    Reader {
        name: "retained-precision",
        optional_members: &["invocation"],
        read: retained,
    },
    // `retained_precision::validate_transport_metadata`: `{"source": <successor>}`.
    Reader {
        name: "retained-precision-transport",
        optional_members: &[],
        read: retained_transport,
    },
];

/// Exit status of a read the reader judged: 0 when it passed, 1 when it refused.
pub const EXIT_PASS: u8 = 0;
pub const EXIT_REFUSED: u8 = 1;

/// Run `reader` over the JSON text `input`. `Err` is an unusable invocation (an
/// unknown reader, input that is not JSON, or an input object of the wrong
/// shape): the reader did not run, and the caller exits 2.
pub fn read(reader: &str, input: &str) -> Result<(u8, Value), String> {
    let entry = READERS
        .iter()
        .find(|entry| entry.name == reader)
        .ok_or_else(|| format!("unknown reader {reader}"))?;
    let value: Value =
        serde_json::from_str(input).map_err(|error| format!("input is not JSON: {error}"))?;
    let Value::Object(members) = value else {
        return Err("input must be a JSON object".into());
    };
    if !members.contains_key("source") {
        return Err(format!("{reader} input requires a source member"));
    }
    if let Some(unknown) = members
        .keys()
        .find(|key| *key != "source" && !entry.optional_members.contains(&key.as_str()))
    {
        return Err(format!(
            "{reader} input has an unsupported member {unknown}"
        ));
    }
    Ok(match (entry.read)(&members) {
        Ok(validation) => (
            EXIT_PASS,
            json!({
                "artifact": ARTIFACT,
                "schema_version": SCHEMA_VERSION,
                "reader": reader,
                "verdict": "pass",
                "validation": validation_json(&validation),
            }),
        ),
        Err(error) => (
            EXIT_REFUSED,
            json!({
                "artifact": ARTIFACT,
                "schema_version": SCHEMA_VERSION,
                "reader": reader,
                "verdict": "refused",
                "refusal": refusal_json(&error),
            }),
        ),
    })
}

fn bits(value: u64) -> String {
    format!("{value:016x}")
}

/// The `Validation` as JSON: bit patterns as 16 lowercase hex digits, the form
/// the shared corpus states its `expected_classifications` in.
pub fn validation_json(validation: &Validation) -> Value {
    let classifications: Vec<Value> = validation
        .classifications
        .iter()
        .map(|row| {
            let (class, bound_bits) = match row.class {
                AccuracyClass::RelativeVerified => ("relative_verified", None),
                AccuracyClass::AbsoluteVerified { bound_bits } => {
                    ("absolute_verified", Some(bits(bound_bits)))
                }
                AccuracyClass::InputDerived => ("input_derived", None),
                AccuracyClass::NonQuantity => ("non_quantity", None),
                AccuracyClass::NotCovered => ("not_covered", None),
            };
            json!({
                "result_id": row.result_id,
                "basis_ref": row.basis_ref,
                "normalized_bits": bits(row.normalized_bits),
                "scale_bits": row.scale_bits.map(bits),
                "class": class,
                "bound_bits": bound_bits,
            })
        })
        .collect();
    json!({
        "invocation_bound": validation.invocation_bound,
        "numerical_eligible": validation.numerical_eligible,
        "publication_sha256": validation.publication_sha256,
        "classifications": classifications,
    })
}

/// The refusal as JSON: the gate, the bare code and the optional detail.
pub fn refusal_json(error: &ValidationError) -> Value {
    json!({"gate": error.gate, "code": error.code, "detail": error.detail})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unusable_inputs_never_run_a_reader() {
        for (reader, input) in [
            ("retained", r#"{"source":{}}"#),
            ("retained-precision", "not json"),
            ("retained-precision", "[]"),
            ("retained-precision", r#"{"invocation":null}"#),
            ("retained-precision", r#"{"source":{},"extra":1}"#),
            (
                "retained-precision-transport",
                r#"{"source":{},"invocation":null}"#,
            ),
        ] {
            assert!(read(reader, input).is_err(), "{reader} {input}");
        }
    }

    #[test]
    fn a_refusal_carries_the_readers_gate_code_and_detail() {
        let (code, output) = read("retained-precision", r#"{"source":{}}"#).unwrap();
        let error = rp::validate(&json!({}), None).unwrap_err();
        assert_eq!(code, EXIT_REFUSED);
        assert_eq!(output["verdict"], "refused");
        assert_eq!(output["refusal"], refusal_json(&error));
        let (code, output) =
            read("retained-precision", r#"{"source":{},"invocation":null}"#).unwrap();
        assert_eq!(
            (code, &output["refusal"]),
            (EXIT_REFUSED, &refusal_json(&error))
        );
        let (code, output) = read("retained-precision-transport", r#"{"source":{}}"#).unwrap();
        let error = rp::validate_transport_metadata(&json!({})).unwrap_err();
        assert_eq!(
            (code, &output["refusal"]),
            (EXIT_REFUSED, &refusal_json(&error))
        );
    }
}
