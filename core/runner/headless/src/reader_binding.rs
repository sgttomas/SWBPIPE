//! The product result readers behind `openpipestress-runner read <reader>`.
//!
//! JSON bytes in; the Rust reader's outcome out, in the reader's own shape
//! (`retained_precision::outcome_json` and `reader_verdict`'s outcomes, the
//! shapes the desktop's wasm engine returns too): `{"ok":true,…}` or
//! `{"ok":false,"error":{…,"code","detail"}}`, inside this command's envelope.
//! Every product caller outside Rust (Python's analysis-run and stress-neutral
//! paths) takes its verdict from here and never re-derives one. A reader is
//! named on the command line and looked up in [`READERS`].
//!
//! The outcome carries only what the reader returns: flags, the stated
//! publication digest, row ids, basis references, bit patterns, routes, table
//! bindings, standing findings and refusal codes. Hashes bind the supplied
//! statements; they never establish producer origin.

use open_pipe_stress_result_export::reader_verdict as rv;
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::{json, Map, Value};

pub const ARTIFACT: &str = "openpipestress.result_reader_verdict";
pub const SCHEMA_VERSION: &str = "1.0.0";

/// One named reader: the input members it requires and admits, and the
/// function that returns its outcome.
pub struct Reader {
    pub name: &'static str,
    /// Input members the reader requires.
    pub required_members: &'static [&'static str],
    /// Input members it admits besides the required ones.
    pub optional_members: &'static [&'static str],
    read: fn(&Map<String, Value>) -> Value,
}

/// An optional member, `null` read as absent.
fn member<'a>(input: &'a Map<String, Value>, name: &str) -> Option<&'a Value> {
    input.get(name).filter(|value| !value.is_null())
}

fn retained(input: &Map<String, Value>) -> Value {
    rp::outcome_json(&rp::validate(&input["source"], member(input, "invocation")))
}

fn retained_transport(input: &Map<String, Value>) -> Value {
    rp::outcome_json(&rp::validate_transport_metadata(&input["source"]))
}

fn dispatch(input: &Map<String, Value>) -> Value {
    rv::dispatch(&input["source"], member(input, "model"), member(input, "invocation"))
}

fn row_semantics(input: &Map<String, Value>) -> Value {
    rv::row_semantics(&input["row"], member(input, "source"))
}

/// A base reader's validation, by its `reader_verdict` name.
macro_rules! base {
    ($name:literal, $optional:expr) => {
        Reader {
            name: $name,
            required_members: &["source"],
            optional_members: $optional,
            read: |input| rv::validate($name, &input["source"], member(input, "model"), member(input, "invocation")),
        }
    };
}
/// A base reader's transport checks, `<reader>-transport`.
macro_rules! transport {
    ($reader:literal) => {
        Reader {
            name: concat!($reader, "-transport"),
            required_members: &["source"],
            optional_members: &[],
            read: |input| rv::transport($reader, &input["source"]),
        }
    };
}

/// The readers this command dispatches to, by name.
pub const READERS: &[Reader] = &[
    // `retained_precision::validate`: `{"source": <successor>, "invocation"?: <{request, solver_mode}> | null}`.
    Reader {
        name: "retained-precision",
        required_members: &["source"],
        optional_members: &["invocation"],
        read: retained,
    },
    // `retained_precision::validate_transport_metadata`: `{"source": <successor>}`.
    Reader {
        name: "retained-precision-transport",
        required_members: &["source"],
        optional_members: &[],
        read: retained_transport,
    },
    // `reader_verdict::dispatch`: `{"source", "model"?, "invocation"?}`; the route,
    // its table binding, the header refusal and the numerical standing. It passes
    // when the header dispatch admits the source.
    Reader {
        name: "dispatch",
        required_members: &["source"],
        optional_members: &["model", "invocation"],
        read: dispatch,
    },
    // `reader_verdict::validate`: `{"source", "model"?}` or `{"source", "invocation"?}`.
    base!("preview-physics", &[]),
    base!("physics", &["model"]),
    base!("physics-source", &["invocation"]),
    base!("source-blocks", &["invocation"]),
    base!("load-reference", &["model"]),
    base!("load-reference-source", &[]),
    // `reader_verdict::transport`: `{"source": <statement>}`.
    transport!("preview-physics"),
    transport!("physics"),
    transport!("physics-source"),
    transport!("load-reference"),
    transport!("load-reference-source"),
    // `reader_verdict::row_semantics`: `{"row", "source"?}`.
    Reader {
        name: "row-semantics",
        required_members: &["row"],
        optional_members: &["source"],
        read: row_semantics,
    },
];

/// Exit status of a read the reader judged: 0 when it passed (`"ok": true`), 1
/// when it refused.
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
    if let Some(missing) = entry.required_members.iter().find(|key| !members.contains_key(**key)) {
        return Err(format!("{reader} input requires a {missing} member"));
    }
    if let Some(unknown) = members.keys().find(|key| {
        !entry.required_members.contains(&key.as_str()) && !entry.optional_members.contains(&key.as_str())
    }) {
        return Err(format!(
            "{reader} input has an unsupported member {unknown}"
        ));
    }
    let outcome = (entry.read)(&members);
    let code = if outcome["ok"] == true {
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
            "outcome": outcome,
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
            ("dispatch", r#"{"source":{},"extra":1}"#),
            ("physics", r#"{"source":{},"invocation":null}"#),
            ("source-blocks", r#"{"source":{},"model":null}"#),
            ("preview-physics", r#"{"source":{},"model":{}}"#),
            ("load-reference-source", r#"{"source":{},"model":{}}"#),
            ("physics-transport", r#"{"source":{},"model":{}}"#),
            ("source-blocks-transport", r#"{"source":{}}"#),
            ("row-semantics", r#"{"source":{}}"#),
            ("row-semantics", r#"{"row":{},"invocation":null}"#),
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

    #[test]
    fn the_base_readers_return_reader_verdict_outcomes() {
        let source = json!({"schema_version": "0.1.0"});
        for (reader, input, outcome) in [
            ("dispatch", json!({"source": source}), rv::dispatch(&source, None, None)),
            ("dispatch", json!({"source": source, "model": null, "invocation": null}), rv::dispatch(&source, None, None)),
            ("physics", json!({"source": source, "model": {"load_cases": []}}), rv::validate("physics", &source, Some(&json!({"load_cases": []})), None)),
            ("source-blocks", json!({"source": source}), rv::validate("source-blocks", &source, None, None)),
            ("load-reference-transport", json!({"source": source}), rv::transport("load-reference", &source)),
            ("row-semantics", json!({"row": {"kind": "x"}}), rv::row_semantics(&json!({"kind": "x"}), None)),
        ] {
            let (code, output) = read(reader, &input.to_string()).unwrap();
            assert_eq!(output["outcome"], outcome, "{reader}");
            assert_eq!(code, if outcome["ok"] == true { EXIT_PASS } else { EXIT_REFUSED }, "{reader}");
        }
        // A legacy header is admitted by the dispatch; its standing is not eligible.
        let (code, output) = read("dispatch", &json!({"source": source}).to_string()).unwrap();
        assert_eq!((code, output["outcome"]["route"].as_str()), (EXIT_PASS, Some("legacy")));
        assert_eq!(output["outcome"]["standing"]["eligible"], false);
        // Every reader_verdict reader and transport has an entry.
        for name in rv::READERS {
            assert!(READERS.iter().any(|r| r.name == name), "{name}");
        }
    }
}
