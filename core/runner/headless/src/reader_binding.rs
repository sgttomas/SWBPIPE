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

/// RT-A4 §10.4: the recorded binding `{id, sha256}` of a recorded-run read. With it, a
/// reader runs as `reader_verdict::*_recorded` (a registry variant of the statement's
/// identity is read under its own tables; any other binding but the current one is
/// refused by name); without it, as the current reader. Only the recorded-run read,
/// compare and re-save paths supply it (Python's `validate_analysis_run_v0_3(...,
/// recorded_read=True)`).
const RECORDED_BINDING: &str = "recorded_binding";

fn retained(input: &Map<String, Value>) -> Value {
    match member(input, RECORDED_BINDING) {
        Some(binding) => rv::retained_recorded(&input["source"], binding, member(input, "invocation")),
        None => rp::outcome_json(&rp::validate(&input["source"], member(input, "invocation"))),
    }
}

fn retained_transport(input: &Map<String, Value>) -> Value {
    match member(input, RECORDED_BINDING) {
        Some(binding) => rv::retained_transport_recorded(&input["source"], binding),
        None => rp::outcome_json(&rp::validate_transport_metadata(&input["source"])),
    }
}

fn dispatch(input: &Map<String, Value>) -> Value {
    match member(input, RECORDED_BINDING) {
        Some(binding) => rv::dispatch_recorded(&input["source"], binding, member(input, "model"), member(input, "invocation")),
        None => rv::dispatch(&input["source"], member(input, "model"), member(input, "invocation")),
    }
}

fn row_semantics(input: &Map<String, Value>) -> Value {
    match (member(input, RECORDED_BINDING), member(input, "source")) {
        (Some(binding), Some(source)) => rv::row_semantics_recorded(&input["row"], source, binding),
        (Some(_), None) => refused(rp_binding_needs_source()),
        (None, source) => rv::row_semantics(&input["row"], source),
    }
}
/// A recorded binding binds a statement's table: without the statement it names none.
fn rp_binding_needs_source() -> &'static str {
    open_pipe_stress_result_export::recorded_tables::SOURCE_RECORDED_BINDING_UNKNOWN
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

/// DEL-04-09's Rust modal reader (`open_pipe_stress_modal_result`), over a
/// modal result document and, optionally, the request it binds:
/// `{"ok", "verdict"}`. `verdict` is the reader's own `modal_verdict_json`
/// (closure, binding, every relation's standing, health, decision, validation
/// status, limit and `public_admission`). `ok` is true when the reader did not
/// reject the document (`decision` is not `rejected`): the document is readable
/// and its checked relations do not contradict it. It is not numerical
/// eligibility and not admission: a `consistent_with_scoped_unsupported` or
/// `consistent_with_unresolved` decision, a request-less read whose binding and
/// model-derived mass are not checked, and a refused document's
/// `refusal_consistent` are all `ok`, and every verdict states public
/// engineering-result admission `disabled`. The document reaches the reader as
/// text, so that its checked-profile closure applies; `read` refuses an input
/// outside that profile as unusable first, so that no duplicate member is
/// collapsed before the reader sees it.
fn modal_result(input: &Map<String, Value>) -> Value {
    use open_pipe_stress_modal_result::{read_modal_result_text, ModalSupplied};
    let text = serde_json::to_string(&input["source"]).expect("a JSON value serializes");
    let verdict = read_modal_result_text(&text, ModalSupplied { request: member(input, "request") });
    json!({"ok": verdict.decision != "rejected", "verdict": verdict.modal_verdict_json()})
}

/// The reader whose input must be inside the checked canonical profile.
const MODAL_RESULT_READER: &str = "modal-result";

/// A base reader's validation, by its `reader_verdict` name.
macro_rules! base {
    ($name:literal, $optional:expr) => {
        Reader {
            name: $name,
            required_members: &["source"],
            optional_members: $optional,
            read: |input| match member(input, RECORDED_BINDING) {
                Some(binding) => rv::validate_recorded($name, &input["source"], binding, member(input, "model"), member(input, "invocation")),
                None => rv::validate($name, &input["source"], member(input, "model"), member(input, "invocation")),
            },
        }
    };
}
/// A base reader's transport checks, `<reader>-transport`.
macro_rules! transport {
    ($reader:literal, $optional:expr) => {
        Reader {
            name: concat!($reader, "-transport"),
            required_members: &["source"],
            optional_members: $optional,
            read: |input| match member(input, RECORDED_BINDING) {
                Some(binding) => rv::transport_recorded($reader, &input["source"], binding),
                None => rv::transport($reader, &input["source"]),
            },
        }
    };
}

/// The readers this command dispatches to, by name.
pub const READERS: &[Reader] = &[
    // `retained_precision::validate`: `{"source": <successor>, "invocation"?: <{request, solver_mode}> | null}`.
    Reader {
        name: "retained-precision",
        required_members: &["source"],
        optional_members: &["invocation", RECORDED_BINDING],
        read: retained,
    },
    // `retained_precision::validate_transport_metadata`: `{"source": <successor>}`.
    Reader {
        name: "retained-precision-transport",
        required_members: &["source"],
        optional_members: &[RECORDED_BINDING],
        read: retained_transport,
    },
    // `reader_verdict::dispatch`: `{"source", "model"?, "invocation"?}`; the route,
    // its table binding, the header refusal and the numerical standing. It passes
    // when the header dispatch admits the source.
    Reader {
        name: "dispatch",
        required_members: &["source"],
        optional_members: &["model", "invocation", RECORDED_BINDING],
        read: dispatch,
    },
    // `reader_verdict::validate`: `{"source", "model"?}` or `{"source", "invocation"?}`.
    // B7b: the current readers only (`reader_verdict::READERS`); the retired v2
    // readers (physics, physics-source, load-reference, load-reference-source)
    // have no entry, and a statement under a retired identity is refused by name
    // at `dispatch`.
    // RT-A4 (review r1 P2-P3): preview-physics admits the model, which re-derives its
    // combination gates and unsigned rows.
    // RT-A4 §10.4: the recorded-run read's `recorded_binding` (preview-physics only).
    base!("preview-physics", &["model", RECORDED_BINDING]),
    base!("pressure-source", &["invocation"]),
    base!("source-recovery", &[]),
    base!("pressure", &["model"]),
    // `reader_verdict::transport`: `{"source": <statement>}`. pressure-2 has no
    // transport reader; its entry refuses by name (SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED).
    transport!("preview-physics", &[RECORDED_BINDING]),
    transport!("pressure-source", &[]),
    transport!("pressure", &[]),
    // `reader_verdict::row_semantics`: `{"row", "source"?}`.
    Reader {
        name: "row-semantics",
        required_members: &["row"],
        optional_members: &["source", RECORDED_BINDING],
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
    // DEL-04-09's modal reader: `{"source": <modal result document>, "request"?: <the
    // modal request it binds> | null}`; `{"ok", "verdict"}` (`modal_result`).
    Reader {
        name: MODAL_RESULT_READER,
        required_members: &["source"],
        optional_members: &["request"],
        read: modal_result,
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
    if entry.name == MODAL_RESULT_READER {
        open_pipe_stress_canonical_json::canonical_json_checked_v1_text(input)
            .map_err(|error| format!("{reader} input is outside the checked canonical profile: {error}"))?;
    }
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
    // RT-A4 §10.4: the command is an outermost entry, so it reads with no recorded table
    // selection left over (`recorded_tables::entry_scope`); a recorded reader selects its own.
    let outcome = open_pipe_stress_result_export::recorded_tables::entry_scope(|| (entry.read)(&members));
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
            // RT-A4 review r1: preview-physics admits the model; it takes no invocation.
            ("preview-physics", r#"{"source":{},"invocation":null}"#),
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
            // RT-A4 §10.4: only the recorded-run read's readers admit a recorded binding.
            ("pressure", r#"{"source":{},"recorded_binding":{}}"#),
            ("pressure-source", r#"{"source":{},"recorded_binding":{}}"#),
            ("source-recovery", r#"{"source":{},"recorded_binding":{}}"#),
            ("pressure-transport", r#"{"source":{},"recorded_binding":{}}"#),
            ("pressure-source-transport", r#"{"source":{},"recorded_binding":{}}"#),
            ("standing", r#"{"source":{},"requested_basis_refs":[],"recorded_binding":{}}"#),
            ("classification-summary", r#"{"source":{},"recorded_binding":{}}"#),
            ("rule-binding", r#"{"envelope":{},"row":{},"recorded_binding":{}}"#),
            // DEL-04-09: the modal reader takes a document and its request only, and an
            // input outside the checked profile (a duplicate member) never reaches it.
            ("modal-result", r#"{"request":{}}"#),
            ("modal-result", r#"{"source":{},"model":{}}"#),
            ("modal-result", r#"{"source":{},"recorded_binding":{}}"#),
            ("modal-result", r#"{"source":{"status":"solved","status":"refused"}}"#),
            ("modal-result", r#"{"source":{},"source":{}}"#),
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
                "row-semantics", "standing", "classification-summary", "rule-binding", "modal-result",
            ]
        );
    }

    /// RT-A4 §10.4: with a recorded binding, each recorded-run reader is its
    /// `reader_verdict::*_recorded` entry; a null binding reads as absent.
    #[test]
    fn a_recorded_binding_runs_the_recorded_entry() {
        let pf: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_2/preview_physics_connected_sparse.json"
        )).unwrap();
        let recorded = json!({"id": sc::PREVIEW_PHYSICS_ID, "sha256": "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2"});
        for (reader, input, outcome) in [
            ("preview-physics", json!({"source": pf, "recorded_binding": recorded}), rv::validate_recorded("preview-physics", &pf, &recorded, None, None)),
            ("preview-physics-transport", json!({"source": pf, "recorded_binding": recorded}), rv::transport_recorded("preview-physics", &pf, &recorded)),
            ("dispatch", json!({"source": pf, "recorded_binding": recorded}), rv::dispatch_recorded(&pf, &recorded, None, None)),
            ("row-semantics", json!({"row": pf["results"][0], "source": pf, "recorded_binding": recorded}), rv::row_semantics_recorded(&pf["results"][0], &pf, &recorded)),
            ("retained-precision", json!({"source": pf, "recorded_binding": recorded}), rv::retained_recorded(&pf, &recorded, None)),
            ("retained-precision-transport", json!({"source": pf, "recorded_binding": recorded}), rv::retained_transport_recorded(&pf, &recorded)),
            ("preview-physics", json!({"source": pf, "recorded_binding": null}), rv::validate("preview-physics", &pf, None, None)),
        ] {
            let (code, output) = read(reader, &input.to_string()).unwrap();
            assert_eq!(output["outcome"], outcome, "{reader}");
            assert_eq!(code, if outcome["ok"] == true { EXIT_PASS } else { EXIT_REFUSED }, "{reader}");
        }
        let (code, _) = read("preview-physics", &json!({"source": pf, "recorded_binding": recorded}).to_string()).unwrap();
        assert_eq!(code, EXIT_PASS);
        let (code, output) = read("preview-physics", &json!({"source": pf}).to_string()).unwrap();
        assert_eq!((code, &output["outcome"]["error"]["code"]), (EXIT_REFUSED, &json!("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS")));
        let (code, output) = read("row-semantics", &json!({"row": pf["results"][0], "recorded_binding": recorded}).to_string()).unwrap();
        assert_eq!((code, &output["outcome"]["error"]["code"]), (EXIT_REFUSED, &json!("SOURCE_RECORDED_BINDING_UNKNOWN")));
    }

    /// DEL-04-09: `modal-result` is the modal reader's own verdict, with and without the
    /// bound request; `ok` is "not rejected", never admission.
    #[test]
    fn the_modal_reader_returns_its_own_verdict() {
        use open_pipe_stress_modal_result::{read_modal_result_text, ModalSupplied};
        let fixture = |name: &str| -> Value {
            serde_json::from_str(match name {
                "modal_cantilever_below_cutoff.json" => include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.json"),
                "modal_cantilever_below_cutoff.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.request.json"),
                "modal_refused_no_bound.json" => include_str!("../../../../fixtures/results/modal_refused_no_bound.json"),
                "modal_refused_no_bound.request.json" => include_str!("../../../../fixtures/results/modal_refused_no_bound.request.json"),
                "modal_arc.request.json" => include_str!("../../../../fixtures/results/modal_arc.request.json"),
                other => panic!("no fixture {other}"),
            })
            .unwrap()
        };
        let own = |doc: &Value, request: Option<&Value>| {
            read_modal_result_text(&serde_json::to_string(doc).unwrap(), ModalSupplied { request }).modal_verdict_json()
        };
        for stem in ["modal_cantilever_below_cutoff", "modal_refused_no_bound"] {
            let doc = fixture(&format!("{stem}.json"));
            let request = fixture(&format!("{stem}.request.json"));
            for (input, supplied) in [
                (json!({"source": doc, "request": request}), Some(&request)),
                (json!({"source": doc}), None),
                (json!({"source": doc, "request": null}), None),
            ] {
                let (code, output) = read("modal-result", &input.to_string()).unwrap();
                let verdict = own(&doc, supplied);
                assert_eq!(output["outcome"]["verdict"], verdict, "{stem}");
                assert_eq!(output["outcome"]["ok"], verdict["decision"] != "rejected", "{stem}");
                assert_eq!(code, EXIT_PASS, "{stem}: {verdict}");
                assert_eq!(verdict["public_admission"]["status"], "disabled", "{stem}");
            }
        }
        // Without the request the binding is not checked and the model-derived mass is scoped:
        // still `ok`, and not fully validated.
        let doc = fixture("modal_cantilever_below_cutoff.json");
        let (_, output) = read("modal-result", &json!({"source": doc}).to_string()).unwrap();
        let verdict = &output["outcome"]["verdict"];
        assert_eq!(verdict["decision"], "consistent_with_scoped_unsupported");
        assert_eq!(verdict["validation"]["status"], "not_fully_validated");
        // Another request than the bound one fails the binding: rejected, exit 1.
        let other = fixture("modal_arc.request.json");
        let (code, output) = read("modal-result", &json!({"source": doc, "request": other}).to_string()).unwrap();
        assert_eq!((code, &output["outcome"]["ok"]), (EXIT_REFUSED, &json!(false)));
        assert_eq!(output["outcome"]["verdict"]["decision"], "rejected");
        // A document that is not a modal result is rejected by closure.
        let (code, output) = read("modal-result", &json!({"source": {}}).to_string()).unwrap();
        assert_eq!((code, &output["outcome"]["verdict"]["decision"]), (EXIT_REFUSED, &json!("rejected")));
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
