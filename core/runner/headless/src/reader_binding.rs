//! The product result readers behind `openpipestress-runner read <reader>`.
//!
//! JSON bytes in; the Rust reader's outcome out, in the reader's own shape
//! (`retained_precision::outcome_json`, the shape the desktop's wasm engine
//! returns too): `{"ok":true,"validation":{…}}` or
//! `{"ok":false,"error":{"gate","code","detail"}}`, inside this command's
//! envelope. Every product caller outside Rust (Python's analysis-run and
//! stress-neutral paths) takes its verdict from here and never re-derives one.
//! A reader is named on the command line and looked up in [`READERS`].
//!
//! The outcome carries only what the reader returns: flags, the stated
//! publication digest, row ids, basis references, bit patterns and refusal
//! codes. Hashes bind the supplied statements; they never establish producer
//! origin.

use open_pipe_stress_result_export::retained_precision::{self as rp, Validation, ValidationError};
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

/// Run `reader` over the JSON text `input`: the exit status and
/// `{"artifact","schema_version","reader","outcome"}`. `Err` is an unusable
/// invocation (an unknown reader, input that is not JSON, or an input object of
/// the wrong shape): the reader did not run, and the caller exits 2.
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
    let outcome = (entry.read)(&members);
    let code = if outcome.is_ok() {
        EXIT_PASS
    } else {
        EXIT_REFUSED
    };
    Ok((
        code,
        json!({
            "artifact": ARTIFACT,
            "schema_version": SCHEMA_VERSION,
            "reader": reader,
            "outcome": rp::outcome_json(&outcome),
        }),
    ))
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
    fn the_outcome_is_the_readers_own_json() {
        for (reader, input, outcome) in [
            (
                "retained-precision",
                r#"{"source":{}}"#,
                rp::validate(&json!({}), None),
            ),
            (
                "retained-precision",
                r#"{"source":{},"invocation":null}"#,
                rp::validate(&json!({}), None),
            ),
            (
                "retained-precision-transport",
                r#"{"source":{}}"#,
                rp::validate_transport_metadata(&json!({})),
            ),
        ] {
            let (code, output) = read(reader, input).unwrap();
            assert_eq!(code, EXIT_REFUSED);
            assert_eq!(output["reader"], reader);
            assert_eq!(output["outcome"], rp::outcome_json(&outcome));
            assert_eq!(output["outcome"]["ok"], false);
        }
    }
}
