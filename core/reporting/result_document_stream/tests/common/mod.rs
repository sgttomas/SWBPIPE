//! Shared by the tests: the inputs `derive_document` takes, built as the result export
//! tests build them, and the comparison of the streamed producer with `derive_document`.
//! Invented data only.
#![allow(dead_code)]

use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_document_stream::{DocumentSource, Member, Peek, SourceList, SourceObject, WalkStopped};
use open_pipe_stress_result_export::derivative::{self as d, derive_document};
use open_pipe_stress_result_export::json_value_eq;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

/// The document base the result export contract tests derive with.
pub const BASE: &str = include_str!("../../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");

pub fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

pub fn base() -> Value {
    parse(BASE)
}

/// A model that names the source's model (the derivation reads only its id and hashes it).
pub fn model_of(source: &Value) -> Value {
    json!({"project": {"id": source["model_ref"]}})
}

/// An attested headless origin with an authentic producer, as the result export contract
/// tests bind one.
pub fn attested(source: &Value) -> Value {
    let carrier = d::checksum(source, "attested_headless_producer_carrier", d::reference("test_carrier", "document-stream")).unwrap();
    json!({"origin_id":"document-stream-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Committed producer bytes; streamed derivative test only","actual_model_ref":d::reference("model_payload",source["model_ref"].as_str().unwrap_or("")),"mechanics_run_ref":d::reference("mechanics_run",source["run_id"].as_str().unwrap_or("")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null})
}

/// A received-current origin (dimension-absent carrier, no authentic producer), as the
/// desktop binds one.
pub fn received(source: &Value) -> Value {
    let run = source["run_id"].as_str().unwrap_or("");
    let carrier = d::checksum(source, "received_current_dimension_absent_carrier", d::reference("received_current_carrier", run)).unwrap();
    json!({"origin_id":"source-origin:current-received","origin_class":"received_current_dimension_absent","qualification_ref":d::reference("current_manifest","input-manifest:test"),"authentic_producer_available":false,"received_carrier_checksum":carrier,"original_producer_checksum":null,"origin_limit":"Committed producer bytes; streamed derivative test only","actual_model_ref":d::reference("model_payload",source["model_ref"].as_str().unwrap_or("")),"mechanics_run_ref":d::reference("mechanics_run",run),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null})
}

/// The arguments of one derivation.
#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub base: Value,
    pub model: Value,
    pub source: Value,
    pub origin: Value,
    pub request: Option<Value>,
}

impl Input {
    pub fn attested(name: impl Into<String>, source: Value, model: Option<Value>) -> Self {
        Input { name: name.into(), base: base(), model: model.unwrap_or_else(|| model_of(&source)), origin: attested(&source), source, request: None }
    }

    pub fn received(name: impl Into<String>, source: Value, model: Option<Value>) -> Self {
        Input { name: name.into(), base: base(), model: model.unwrap_or_else(|| model_of(&source)), origin: received(&source), source, request: None }
    }

    /// The same input with its origin rebound to its (edited) source.
    pub fn rebound(mut self) -> Self {
        self.origin = if self.origin["authentic_producer_available"] == true { attested(&self.source) } else { received(&self.source) };
        self
    }
}

/// What both derivations gave.
pub enum Outcome {
    Derived(Value),
    Refused(String),
}

/// Derives `input` both ways and holds them equal: the same refusal text, or the same
/// canonical bytes, derivative hash, header and lists, with the walk's staged bytes equal to
/// what the walk showed.
pub fn compare(input: &Input) -> Outcome {
    let name = &input.name;
    let expected = derive_document(input.base.clone(), &input.model, &input.source, input.origin.clone(), input.request.as_ref());
    let streamed = DocumentSource::new(input.base.clone(), &input.model, &input.source, input.origin.clone(), input.request.as_ref());
    let (document, stream) = match (expected, streamed) {
        (Err(expected), Err(streamed)) => {
            assert_eq!(streamed, expected, "{name}: the refusals differ");
            return Outcome::Refused(expected);
        }
        (Ok(_), Err(streamed)) => panic!("{name}: derive_document derives, the streamed producer refuses: {streamed}"),
        (Err(expected), Ok(_)) => panic!("{name}: derive_document refuses ({expected}), the streamed producer derives"),
        (Ok(document), Ok(stream)) => (document, stream),
    };
    let text = canonical_json(&document);
    let hash = document["result_envelope"]["reproducibility"]["derivative_hash"]["value"].as_str().unwrap();
    if stream.derivative_sha256() != hash {
        let mut written = Vec::new();
        let _ = stream.write_to(&mut written);
        let strip = |mut v: Value| {
            v["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
            canonical_json(&v)
        };
        let ours = strip(serde_json::from_slice(&written).unwrap());
        let theirs = strip(document.clone());
        let at = ours.bytes().zip(theirs.bytes()).position(|(a, b)| a != b).unwrap_or(ours.len().min(theirs.len()));
        let from = at.saturating_sub(200);
        panic!("{name}: payloads differ at {at} (lengths {} {}):\n ours:   {}\n theirs: {}", ours.len(), theirs.len(), &ours[from..(at + 200).min(ours.len())], &theirs[from..(at + 200).min(theirs.len())]);
    }

    // The second emission, written whole.
    let mut written = Vec::new();
    let emitted = stream.write_to(&mut written).unwrap_or_else(|e| panic!("{name}: {e}"));
    if written != text.as_bytes() {
        let at = written.iter().zip(text.as_bytes()).position(|(a, b)| a != b).unwrap_or(written.len().min(text.len()));
        let from = at.saturating_sub(120);
        panic!(
            "{name}: the streamed bytes differ from derive_document's canonical text at byte {at}:\n streamed: {}\n expected: {}",
            String::from_utf8_lossy(&written[from..(at + 120).min(written.len())]),
            &text[from..(at + 120).min(text.len())]
        );
    }
    assert_eq!(emitted.bytes, text.len() as u64, "{name}");
    assert_eq!(emitted.sha256, format!("{:x}", Sha256::digest(text.as_bytes())), "{name}");
    stream.verify(&mut written.as_slice()).unwrap_or_else(|e| panic!("{name}: verify: {e}"));

    // The walk: a recording visitor's value is what the stage holds, and is derive_document's.
    let mut staged = Vec::new();
    let emission = stream.emission(&mut staged);
    let recorded = record_object(&emission).unwrap_or_else(|_| panic!("{name}: the recording walk stopped"));
    emission.finish().unwrap_or_else(|e| panic!("{name}: {e}"));
    assert!(recorded_is_staged(&recorded, &staged), "{name}: the walk staged other bytes than it showed");
    assert!(json_value_eq(&recorded, &document), "{name}: the walk's value is not derive_document's");

    // The header with each list collected from a walk is derive_document's value.
    let mut whole = stream.header();
    let lists = collect_lists(&stream);
    for (pointer, entries) in lists {
        *whole.pointer_mut(pointer).unwrap() = Value::Array(entries);
    }
    assert_eq!(canonical_json(&whole), text, "{name}: header and walked lists");
    Outcome::Derived(document)
}

/// Whether the value a visitor rebuilt equals the parse of the bytes staged (by value and in
/// canonical text).
pub fn recorded_is_staged(recorded: &Value, staged: &[u8]) -> bool {
    match serde_json::from_slice::<Value>(staged) {
        Ok(parsed) => json_value_eq(recorded, &parsed) && canonical_json(recorded).as_bytes() == staged,
        Err(_) => false,
    }
}

/// The six row lists of an emission, by their pointers, from a walk that keeps only them.
pub fn collect_lists(stream: &DocumentSource) -> Vec<(&'static str, Vec<Value>)> {
    let mut sink = std::io::sink();
    let emission = stream.emission(&mut sink);
    let mut lists = Vec::new();
    let mut path = Vec::new();
    walk_collect(&emission, &mut path, &mut lists).unwrap();
    emission.finish().unwrap();
    lists
}

fn list_pointer(path: &[String]) -> Option<&'static str> {
    let path: Vec<&str> = path.iter().map(String::as_str).collect();
    Some(match path.as_slice() {
        ["result_envelope", "result_sets", "0", "values"] => "/result_envelope/result_sets/0/values",
        ["result_envelope", "review_evidence"] => "/result_envelope/review_evidence",
        ["result_envelope", "row_accounting"] => "/result_envelope/row_accounting",
        ["result_envelope", "row_disclosures"] => "/result_envelope/row_disclosures",
        ["result_envelope", "source_annotations"] => "/result_envelope/source_annotations",
        ["result_envelope", "unit_preservation_witnesses"] => "/result_envelope/unit_preservation_witnesses",
        _ => return None,
    })
}

fn walk_collect(object: &dyn SourceObject, path: &mut Vec<String>, lists: &mut Vec<(&'static str, Vec<Value>)>) -> Result<(), WalkStopped> {
    object.walk(&mut |name, member| {
        path.push(name.to_owned());
        let walked = match member {
            Member::Value(_) => Ok(()),
            Member::Object(child) => walk_collect(child, path, lists),
            Member::List(list) => {
                let pointer = list_pointer(path);
                let mut entries = Vec::new();
                let mut index = 0;
                let walked = list.walk(&mut |entry| {
                    let walked = match entry {
                        Member::Value(value) => {
                            entries.push(value.clone());
                            Ok(())
                        }
                        Member::Object(child) => {
                            path.push(index.to_string());
                            let walked = walk_collect(child, path, lists);
                            path.pop();
                            walked
                        }
                        Member::List(_) => panic!("a list in a list"),
                    };
                    index += 1;
                    walked
                });
                if let Some(pointer) = pointer {
                    lists.push((pointer, entries));
                }
                walked
            }
        };
        path.pop();
        walked
    })
}

/// A recording visitor: rebuilds the value of everything a source object shows, checking on
/// the way the promises a rule checks of a source (distinct names, ascending order, the
/// peek/walk pairing, `provenance` held as a value, `None` for a name that is not a member).
pub fn record_object(object: &dyn SourceObject) -> Result<Value, WalkStopped> {
    let names = object.member_names();
    let mut unique = names.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), names.len(), "member names repeat");
    assert!(object.peek("\u{0}not a member").is_none(), "peek answers for a name that is not a member");
    let mut map = Map::new();
    let mut previous: Option<String> = None;
    object.walk(&mut |name, member| {
        if let Some(previous) = &previous {
            assert!(previous.encode_utf16().cmp(name.encode_utf16()).is_lt(), "members out of canonical order: {previous} then {name}");
        }
        previous = Some(name.to_owned());
        assert!(names.iter().any(|n| n == name), "{name} walked but not named");
        let peeked = object.peek(name).unwrap_or_else(|| panic!("{name}: no peek"));
        let value = match member {
            Member::Value(value) => {
                match peeked {
                    Peek::Value(peeked) => assert_eq!(peeked.as_ref(), value, "{name}: peeked and walked values differ"),
                    other => panic!("{name}: walked as a value, peeked as {other:?}"),
                }
                value.clone()
            }
            Member::Object(child) => {
                assert_ne!(name, "provenance", "provenance given by walk");
                assert!(matches!(peeked, Peek::Object), "{name}: walked as an object, peeked otherwise");
                record_object(child)?
            }
            Member::List(list) => {
                assert_ne!(name, "provenance", "provenance given by walk");
                assert!(matches!(peeked, Peek::List), "{name}: walked as a list, peeked otherwise");
                record_list(list)?
            }
        };
        map.insert(name.to_owned(), value);
        Ok(())
    })?;
    assert_eq!(map.len(), names.len(), "not every named member was walked");
    Ok(Value::Object(map))
}

pub fn record_list(list: &dyn SourceList) -> Result<Value, WalkStopped> {
    let mut items = Vec::new();
    list.walk(&mut |entry| {
        items.push(match entry {
            Member::Value(value) => value.clone(),
            Member::Object(child) => record_object(child)?,
            Member::List(inner) => record_list(inner)?,
        });
        Ok(())
    })?;
    Ok(Value::Array(items))
}
