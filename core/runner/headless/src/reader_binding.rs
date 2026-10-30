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
use open_pipe_stress_result_export::semantic_contract as sc;
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

fn refused(text: &str) -> Value {
    json!({"ok": false, "error": rv::error_json(text)})
}

/// The caller's requested basis references: an array, or refused.
fn requested_refs(input: &Map<String, Value>) -> Result<&[Value], Value> {
    match member(input, "requested_basis_refs") {
        None => Ok(&[]),
        Some(Value::Array(refs)) => Ok(refs),
        Some(_) => Err(refused("READER_REQUESTED_BASIS_REFS_INVALID")),
    }
}

/// `semantic_contract::numerical_use_standing_with_context` over the caller's
/// requested references, and `standing_reason`.
fn standing(input: &Map<String, Value>) -> Value {
    let refs = match requested_refs(input) {
        Ok(refs) => refs,
        Err(refusal) => return refusal,
    };
    let source = &input["source"];
    json!({"ok": true,
        "token": sc::numerical_use_standing_with_context(source, refs, member(input, "invocation")),
        "reason": sc::standing_reason(source)})
}

/// `semantic_contract::classification_summary`.
fn classification_summary(input: &Map<String, Value>) -> Value {
    match requested_refs(input) {
        Ok(refs) => json!({"ok": true,
            "summary": sc::classification_summary(&input["source"], member(input, "invocation"), refs)}),
        Err(refusal) => refusal,
    }
}

/// `semantic_contract::rule_binding_refusal`.
fn rule_binding(input: &Map<String, Value>) -> Value {
    json!({"ok": true, "refusal": sc::rule_binding_refusal(&input["envelope"], &input["row"])})
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
    // B7b: the current readers only (`reader_verdict::READERS`); the retired v2
    // readers (physics, physics-source, load-reference, load-reference-source)
    // have no entry, and a statement under a retired identity is refused by name
    // at `dispatch`.
    base!("preview-physics", &[]),
    base!("pressure-source", &["invocation"]),
    base!("source-recovery", &[]),
    base!("pressure", &["model"]),
    // `reader_verdict::transport`: `{"source": <statement>}`. pressure-2 has no
    // transport reader; its entry refuses by name (SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED).
    transport!("preview-physics"),
    transport!("pressure-source"),
    transport!("pressure"),
    // `reader_verdict::row_semantics`: `{"row", "source"?}`.
    Reader {
        name: "row-semantics",
        required_members: &["row"],
        optional_members: &["source"],
        read: row_semantics,
    },
    // `semantic_contract::numerical_use_standing_with_context` and `standing_reason`:
    // `{"source", "requested_basis_refs", "invocation"?}`; `{"ok":true,"token","reason"}`.
    Reader {
        name: "standing",
        required_members: &["source", "requested_basis_refs"],
        optional_members: &["invocation"],
        read: standing,
    },
    // `semantic_contract::classification_summary`: `{"source", "invocation"?,
    // "requested_basis_refs"?}`; `{"ok":true,"summary":[…]}`.
    Reader {
        name: "classification-summary",
        required_members: &["source"],
        optional_members: &["invocation", "requested_basis_refs"],
        read: classification_summary,
    },
    // `semantic_contract::rule_binding_refusal`: `{"envelope", "row"}`;
    // `{"ok":true,"refusal":<code>|null}`.
    Reader {
        name: "rule-binding",
        required_members: &["envelope", "row"],
        optional_members: &[],
        read: rule_binding,
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
            ("pressure", r#"{"source":{},"invocation":null}"#),
            ("pressure-source", r#"{"source":{},"model":null}"#),
            ("preview-physics", r#"{"source":{},"model":{}}"#),
            ("source-recovery", r#"{"source":{},"model":{}}"#),
            ("pressure-transport", r#"{"source":{},"model":{}}"#),
            ("pressure-source-transport", r#"{"source":{},"model":{}}"#),
            // B7b: the retired v2 readers have no entry.
            ("physics", r#"{"source":{}}"#),
            ("physics-source", r#"{"source":{}}"#),
            ("load-reference", r#"{"source":{}}"#),
            ("load-reference-source", r#"{"source":{}}"#),
            ("physics-transport", r#"{"source":{}}"#),
            ("physics-source-transport", r#"{"source":{}}"#),
            ("load-reference-transport", r#"{"source":{}}"#),
            ("load-reference-source-transport", r#"{"source":{}}"#),
            ("standing", r#"{"source":{}}"#),
            ("rule-binding", r#"{"row":{}}"#),
            ("classification-summary", r#"{"source":{},"model":{}}"#),
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
            ("pressure", json!({"source": source, "model": {"load_cases": []}}), rv::validate("pressure", &source, Some(&json!({"load_cases": []})), None)),
            ("pressure-source", json!({"source": source}), rv::validate("pressure-source", &source, None, None)),
            ("source-recovery", json!({"source": source}), rv::validate("source-recovery", &source, None, None)),
            ("pressure-source-transport", json!({"source": source}), rv::transport("pressure-source", &source)),
            ("pressure-transport", json!({"source": source}), rv::transport("pressure", &source)),
            ("preview-physics-transport", json!({"source": source}), rv::transport("preview-physics", &source)),
            ("row-semantics", json!({"row": {"kind": "x"}}), rv::row_semantics(&json!({"kind": "x"}), None)),
        ] {
            let (code, output) = read(reader, &input.to_string()).unwrap();
            assert_eq!(output["outcome"], outcome, "{reader}");
            assert_eq!(code, if outcome["ok"] == true { EXIT_PASS } else { EXIT_REFUSED }, "{reader}");
        }
        // b7-plan E2: a retired 0.1.0 header is refused by name; its standing is not eligible.
        let (code, output) = read("dispatch", &json!({"source": source}).to_string()).unwrap();
        assert_eq!((code, output["outcome"]["route"].as_str()), (EXIT_REFUSED, Some("unsupported")));
        assert_eq!(output["outcome"]["error"]["code"], sc::MECHANICS_RESULT_VERSION_RETIRED);
        assert_eq!(output["outcome"]["standing"]["eligible"], false);
        // Every reader_verdict reader has an entry, and the base and transport
        // entries are exactly the current readers (B7b).
        for name in rv::READERS {
            assert!(READERS.iter().any(|r| r.name == name), "{name}");
        }
        let names: Vec<&str> = READERS.iter().map(|r| r.name).collect();
        assert_eq!(
            names,
            [
                "retained-precision", "retained-precision-transport", "dispatch",
                "preview-physics", "pressure-source", "source-recovery", "pressure",
                "preview-physics-transport", "pressure-source-transport", "pressure-transport",
                "row-semantics", "standing", "classification-summary", "rule-binding",
            ]
        );
    }

    #[test]
    fn the_semantic_contract_entries_return_its_values() {
        let source = json!({"schema_version": "0.1.0", "results": [], "diagnostics": []});
        let refs = json!([{"ref_type": "load_case", "ref_id": "lc"}]);
        let (code, output) =
            read("standing", &json!({"source": source, "requested_basis_refs": refs}).to_string()).unwrap();
        assert_eq!(code, EXIT_PASS);
        assert_eq!(
            output["outcome"],
            json!({"ok": true, "token": sc::numerical_use_standing(&source, refs.as_array().unwrap()),
                "reason": sc::standing_reason(&source)})
        );
        let (code, output) =
            read("standing", &json!({"source": source, "requested_basis_refs": {}}).to_string()).unwrap();
        assert_eq!((code, &output["outcome"]["error"]["code"]), (EXIT_REFUSED, &json!("READER_REQUESTED_BASIS_REFS_INVALID")));
        // B4/D1 (b): the retired source-blocks-1 summary refusal is gone; no row of it is refused here.
        let envelope = json!({"producer": {"semantic_contract_id": sc::RETIRED_SOURCE_BLOCKS_ID}, "summary": {}});
        let row = json!({"id": "r", "kind": "open_formula_stress_summary"});
        let (_, output) = read("rule-binding", &json!({"envelope": envelope, "row": row}).to_string()).unwrap();
        assert_eq!(output["outcome"], json!({"ok": true, "refusal": sc::rule_binding_refusal(&envelope, &row)}));
        assert_eq!(output["outcome"]["refusal"], Value::Null);
        let (_, output) = read("classification-summary", &json!({"source": source}).to_string()).unwrap();
        assert_eq!(output["outcome"], json!({"ok": true, "summary": []}));
        // The retired source-blocks readers have no entry.
        assert!(read("source-blocks", &json!({"source": {}}).to_string()).is_err());
        assert!(read("source-blocks-transport", &json!({"source": {}}).to_string()).is_err());
    }
}
