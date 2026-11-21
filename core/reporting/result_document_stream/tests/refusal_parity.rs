//! `DocumentSource::new` refuses a faulty input with `derive_document`'s text, and where an
//! input has two faults, with the one `derive_document` meets first. Each input is a
//! committed source, base or origin with the smallest edit that makes the fault. The label
//! beside each case is the refusal `derive_document` gives; the commitment is that both
//! refuse alike. Invented data only.

mod common;

use common::{compare, parse, received, Input, Outcome};
use open_pipe_stress_result_document_stream::DocumentSource;
use open_pipe_stress_result_export::derivative::derive_document;
use serde_json::{json, Value};

const PREVIEW: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");
const PRESSURE: &str = include_str!("../../../../fixtures/results/pressure_connected_mechanics_sparse.json");
const PRESSURE_SOURCE: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");

fn preview() -> Input {
    Input::attested("preview", parse(PREVIEW), None)
}

fn with(name: &str, input: Input, edit: impl FnOnce(&mut Input)) -> (String, Input) {
    let mut input = input;
    input.name = name.to_owned();
    edit(&mut input);
    (name.to_owned(), input)
}

/// Edits the source and rebinds the origin to it, so that only the edit is a fault.
fn source_edit(name: &str, input: Input, edit: impl FnOnce(&mut Value)) -> (String, Input) {
    let mut input = input;
    input.name = name.to_owned();
    edit(&mut input.source);
    (name.to_owned(), input.rebound())
}

fn row(input: &mut Value, index: usize) -> &mut Value {
    &mut input["results"][index]
}

fn cases() -> Vec<(String, Input, &'static str)> {
    let unsafe_number = json!(9007199254740993u64);
    let mut cases = Vec::new();
    let mut case = |(name, input): (String, Input), code: &'static str| cases.push((name, input, code));

    // derive_document's own header checks, in its order.
    case(source_edit("an unknown identity", preview(), |s| s["producer"]["semantic_contract_id"] = json!("x")), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED");
    case(with("an unsafe model number", preview(), |i| i.model["n"] = unsafe_number.clone()), "UNSAFE_JSON_NUMBER");
    case(with("an unsafe base number", preview(), |i| i.base["n"] = unsafe_number.clone()), "UNSAFE_JSON_NUMBER");
    case(with("an unsafe origin number", preview(), |i| i.origin["n"] = unsafe_number.clone()), "UNSAFE_JSON_NUMBER");
    case(with("another model", preview(), |i| i.model["project"]["id"] = json!("other")), "SOURCE_MODEL_IDENTITY_MISMATCH");
    case(with("a pressure-source receipt on another identity's base", preview(), |i| i.base["result_envelope"]["source_block_recovery"] = json!({})), "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN");
    case(with("no origin id", preview(), |i| i.origin.as_object_mut().unwrap().remove("origin_id").map(drop).unwrap()), "ORIGIN_ID_MISSING");
    case(with("no producer availability", preview(), |i| i.origin["authentic_producer_available"] = json!("yes")), "ORIGIN_AVAILABILITY_MISSING");
    case(with("no carrier scope", preview(), |i| i.origin["received_carrier_checksum"]["payload_scope"] = json!(null)), "ORIGIN_SCOPE_MISSING");
    case(with("another carrier digest", preview(), |i| i.origin["received_carrier_checksum"]["value"] = json!("0".repeat(64))), "SOURCE_CARRIER_HASH_MISMATCH");
    case(
        with("an unavailable producer with a hash", preview(), |i| {
            i.origin["authentic_producer_available"] = json!(false);
        }),
        "UNAVAILABLE_PRODUCER_HASH",
    );
    // After derive_document's row loop: the request's hash.
    case(with("an unsafe request number", preview(), |i| i.request = Some(json!({"request_id": "r", "n": unsafe_number.clone()}))), "UNSAFE_JSON_NUMBER");
    // validate_document's header checks, in its order.
    case(with("a precision receipt on another identity's base", preview(), |i| i.base["result_envelope"]["retained_precision"] = json!({})), "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN");
    case(with("another actual model", preview(), |i| i.origin["actual_model_ref"]["ref_id"] = json!("other")), "ORIGIN_IDENTITY_MISMATCH");
    case(with("another mechanics run", preview(), |i| i.origin["mechanics_run_ref"]["ref_id"] = json!("other")), "ORIGIN_IDENTITY_MISMATCH");
    case(
        with("an invalid carrier scope", preview(), |i| {
            i.origin["received_carrier_checksum"]["payload_scope"] = json!("other_carrier");
            i.origin["original_producer_checksum"] = i.origin["received_carrier_checksum"].clone();
        }),
        "ORIGIN_SCOPE_INVALID",
    );
    case(with("a carrier checksum of another algorithm", preview(), |i| {
        i.origin["received_carrier_checksum"]["algorithm"] = json!("sha512");
        i.origin["original_producer_checksum"] = i.origin["received_carrier_checksum"].clone();
    }), "SOURCE_CARRIER_HASH_MISMATCH");
    case(with("an original producer of other bytes", preview(), |i| i.origin["original_producer_checksum"]["payload_ref"]["ref_id"] = json!("other")), "ORIGINAL_PRODUCER_BINDING_MISMATCH");

    // The source and its rows. The product reader (`for_source`, called first) refuses each
    // of these before derive_document's own row checks; the commitment is that the two
    // refuse alike, so the reader's code is the expected one.
    case(source_edit("not solved", preview(), |s| s["status"]["mechanics"] = json!("MODEL_INCOMPLETE")), "SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE");
    case(source_edit("rows not a list", preview(), |s| s["results"] = json!({})), "SOURCE_PREVIEW_PHYSICS_ARRAY_INVALID");
    case(source_edit("no rows", preview(), |s| s["results"] = json!([])), "SOURCE_PREVIEW_PHYSICS_DANGLING_RESULT_REF");
    case(source_edit("a row without an id", preview(), |s| row(s, 3).as_object_mut().unwrap().remove("id").map(drop).unwrap()), "SOURCE_PREVIEW_PHYSICS_STRING_INVALID");
    case(source_edit("a row with an empty id", preview(), |s| row(s, 3)["id"] = json!("")), "SOURCE_PREVIEW_PHYSICS_STRING_INVALID");
    case(source_edit("a repeated id", preview(), |s| row(s, 3)["id"] = s["results"][2]["id"].clone()), "SOURCE_PREVIEW_PHYSICS_DUPLICATE_SOURCE_ID");
    case(source_edit("a value that is not a number", preview(), |s| row(s, 3)["value"] = json!("1")), "SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID");
    case(source_edit("a row without a unit", preview(), |s| row(s, 3)["unit"] = json!("")), "SOURCE_PREVIEW_PHYSICS_STRING_INVALID");
    case(source_edit("a row without a kind", preview(), |s| row(s, 3)["kind"] = json!(null)), "SOURCE_PREVIEW_PHYSICS_STRING_INVALID");
    case(source_edit("a row without an entity", preview(), |s| row(s, 3).as_object_mut().unwrap().remove("entity_ref").map(drop).unwrap()), "SOURCE_PREVIEW_PHYSICS_STRING_INVALID");
    case(source_edit("a pressure row without an entity", Input::attested("p", parse(PRESSURE), None), |s| row(s, 3).as_object_mut().unwrap().remove("entity_ref").map(drop).unwrap()), "SOURCE_PHYSICS_STRING_INVALID");
    case(source_edit("a pressure-source row without an entity", Input::attested("ps", parse(PRESSURE_SOURCE), None), |s| row(s, 3).as_object_mut().unwrap().remove("entity_ref").map(drop).unwrap()), "SOURCE_PHYSICS_STRING_INVALID");
    case(source_edit("no contract evidence", preview(), |s| s.as_object_mut().unwrap().remove("contract_evidence").map(drop).unwrap()), "SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED");
    case(
        source_edit("a dimension-absent carrier that declares a dimension", Input::received("r", parse(PREVIEW), None), |s| row(s, 3)["dimension"] = json!("force")),
        "CARRIER_DIMENSION_PRESENT",
    );

    // Two faults: the one derive_document meets first.
    case(
        with("another model, and no origin id", preview(), |i| {
            i.model["project"]["id"] = json!("other");
            i.origin.as_object_mut().unwrap().remove("origin_id");
        }),
        "SOURCE_MODEL_IDENTITY_MISMATCH",
    );
    case(
        with("a pressure-source receipt, and no origin id", preview(), |i| {
            i.base["result_envelope"]["source_block_recovery"] = json!({});
            i.origin.as_object_mut().unwrap().remove("origin_id");
        }),
        "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN",
    );
    case(
        with("a pressure-source receipt, and no envelope id", preview(), |i| {
            i.base["result_envelope"]["source_block_recovery"] = json!({});
            i.base["result_envelope"].as_object_mut().unwrap().remove("envelope_id");
        }),
        "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN",
    );
    case(
        with("no origin id, and an invalid carrier scope", preview(), |i| {
            i.origin.as_object_mut().unwrap().remove("origin_id");
            i.origin["received_carrier_checksum"]["payload_scope"] = json!("other_carrier");
        }),
        "ORIGIN_ID_MISSING",
    );
    case(
        with("another carrier digest, and an invalid carrier scope", preview(), |i| {
            i.origin["received_carrier_checksum"]["value"] = json!("0".repeat(64));
            i.origin["received_carrier_checksum"]["payload_scope"] = json!("other_carrier");
        }),
        "SOURCE_CARRIER_HASH_MISMATCH",
    );
    case(
        with("an unsafe request number, and an invalid carrier scope", preview(), |i| {
            i.request = Some(json!({"n": unsafe_number.clone()}));
            i.origin["received_carrier_checksum"]["payload_scope"] = json!("other_carrier");
            i.origin["original_producer_checksum"] = i.origin["received_carrier_checksum"].clone();
        }),
        "UNSAFE_JSON_NUMBER",
    );
    case(
        with("a precision receipt, and another actual model", preview(), |i| {
            i.base["result_envelope"]["retained_precision"] = json!({});
            i.origin["actual_model_ref"]["ref_id"] = json!("other");
        }),
        "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN",
    );
    case(
        with("another actual model, and no mechanics run", preview(), |i| {
            i.source.as_object_mut().unwrap().remove("run_id");
            i.origin = common::attested(&i.source);
            i.origin["actual_model_ref"]["ref_id"] = json!("other");
        }),
        "ORIGIN_IDENTITY_MISMATCH",
    );
    case(
        with("another actual model, and an invalid carrier scope", preview(), |i| {
            i.origin["actual_model_ref"]["ref_id"] = json!("other");
            i.origin["received_carrier_checksum"]["payload_scope"] = json!("other_carrier");
            i.origin["original_producer_checksum"] = i.origin["received_carrier_checksum"].clone();
        }),
        "ORIGIN_IDENTITY_MISMATCH",
    );
    case(
        with("an invalid carrier scope, and an original producer of other bytes", preview(), |i| {
            i.origin["received_carrier_checksum"]["payload_scope"] = json!("other_carrier");
            i.origin["original_producer_checksum"]["payload_ref"]["ref_id"] = json!("other");
        }),
        "ORIGIN_SCOPE_INVALID",
    );
    case(
        with("a row without an entity, and an invalid carrier scope", preview(), |i| {
            row(&mut i.source, 5).as_object_mut().unwrap().remove("entity_ref");
            i.origin = common::attested(&i.source);
            i.origin["received_carrier_checksum"]["payload_scope"] = json!("other_carrier");
            i.origin["original_producer_checksum"] = i.origin["received_carrier_checksum"].clone();
        }),
        "SOURCE_PREVIEW_PHYSICS_STRING_INVALID",
    );
    case(
        source_edit("a repeated id at row 6, and a row without an entity at row 4", preview(), |s| {
            row(s, 6)["id"] = s["results"][5]["id"].clone();
            row(s, 4).as_object_mut().unwrap().remove("entity_ref");
        }),
        "SOURCE_PREVIEW_PHYSICS_DUPLICATE_SOURCE_ID",
    );
    cases
}

#[test]
fn each_faulty_input_is_refused_with_derive_documents_text() {
    let mut texts = Vec::new();
    for (name, input, code) in cases() {
        match compare(&input) {
            Outcome::Refused(text) => {
                if !code.is_empty() {
                    assert_eq!(text.split(':').next().unwrap(), code, "{name}: {text}");
                }
                texts.push(format!("{name}: {text}"));
            }
            Outcome::Derived(_) => panic!("{name}: both derive; the case makes no fault"),
        }
    }
    println!("{}", texts.join("\n"));
}

/// The cases that edit a received origin are rebound to a received origin.
#[test]
fn a_received_origin_is_rebound_as_received() {
    let input = Input::received("r", parse(PREVIEW), None).rebound();
    assert_eq!(input.origin, received(&input.source));
}

/// Where `derive_document` panics on a malformed base, the streamed producer refuses.
#[test]
fn a_malformed_base_is_refused_where_derive_document_panics() {
    let input = preview();
    let cases: [(&str, Value, &str); 4] = [
        ("a base that is not an object", json!([]), "DERIVATIVE_BASE_NOT_OBJECT"),
        ("an envelope that is not an object", json!({"result_envelope": 1}), "RESULT_ENVELOPE_NOT_OBJECT"),
        ("a result set that is not an object", {
            let mut base = input.base.clone();
            base["result_envelope"]["result_sets"] = json!([[]]);
            base
        }, "RESULT_SET_NOT_OBJECT"),
        ("no envelope id", {
            let mut base = input.base.clone();
            base["result_envelope"].as_object_mut().unwrap().remove("envelope_id");
            base
        }, "ENVELOPE_ID_MISSING"),
    ];
    for (name, base, code) in cases {
        let streamed = DocumentSource::new(base.clone(), &input.model, &input.source, input.origin.clone(), None).err();
        assert_eq!(streamed.as_deref(), Some(code), "{name}");
        let (model, source, origin) = (input.model.clone(), input.source.clone(), input.origin.clone());
        let derived = std::panic::catch_unwind(move || derive_document(base, &model, &source, origin, None).map(drop));
        assert!(derived.is_err(), "{name}: derive_document no longer panics; hold the two refusals equal instead");
    }
}
