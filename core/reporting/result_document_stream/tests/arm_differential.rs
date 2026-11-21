//! A generated-input differential: sources built from committed fixtures, each unedited or
//! with the smallest edit that reaches an arm of the decisions the streamed producer
//! restates from `derivative.rs`, each derived both ways and compared byte for byte
//! (`common::compare`). A census of the arms reached, read from `derive_document`'s
//! document and the source, must be exactly the reachable arms: (a) each downgrade to
//! disclosed, with and without a class disclosure; (b) every reason code; (c) both evidence
//! kinds and both basis fallbacks; (d) each identity and the refusal of a dimension-absent
//! carrier that declares a dimension; and combined connector rows of a combination.
//! Invented data only.

mod common;

use common::{compare, parse, Input, Outcome};
use open_pipe_stress_result_export::semantic_contract::{canonical_metadata_for, complete_metadata, for_source, signature_for};
use open_pipe_stress_result_export::source_blocks::domain_hash;
use serde_json::{json, Value};
use std::collections::BTreeSet;

const INVENTED: &str = include_str!("../../../../fixtures/results/preview_physics_invented_sparse.json");
const CONNECTED: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");
const TIMOSHENKO: &str = include_str!("../../../../fixtures/results/preview_physics_timoshenko_arc_sparse.json");
const PRESSURE: &str = include_str!("../../../../fixtures/results/pressure_connected_mechanics_sparse.json");
const FRICTION: &str = include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json");
const PRESSURE_SOURCE: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");
const MILESTONE: &str = include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json");
const EXACT: &str = include_str!("../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json");
const COMBINATIONS: &str = include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json");

/// Every arm; those in [`UNREACHED`] are not reachable through the product reader.
const ARMS: &[&str] = &[
    "a: declared disposition kept",
    "a: review downgraded for incomplete metadata",
    "a: review downgraded for incomplete metadata, with a class disclosure",
    "a: quantity downgraded for missing canonical metadata",
    "a: quantity downgraded for missing canonical metadata, with a class disclosure",
    "a: downgraded by a class disclosure alone",
    "b: retained_precision_absolute_verified",
    "b: retained_precision_not_covered",
    "b: review_metadata_incomplete",
    "b: physical_metadata_incomplete",
    "b: unsupported_source_kind",
    "b: basis_annotation_not_quantity",
    "b: diagnostic_relative_ratio_non_governing",
    "b: discrete_evidence_not_ratio",
    "b: diagnostic_evidence_not_physical_quantity",
    "c: evidence kind assembled_load_review",
    "c: evidence kind user_input_review",
    "c: review basis from its metadata (no basis_ref)",
    "c: quantity basis from the run (no basis_ref)",
    "c: target basis the row's own basis_ref",
    "d: pressure-3",
    "d: pressure-source-1",
    "d: preview-physics-2",
    "d: preview-physics-timoshenko-1",
    "d: preview-physics-retained-1",
    "d: physics-retained-1",
    "d: CARRIER_DIMENSION_PRESENT",
    "combined connector row of a combination, exported",
];

/// Arms no source the product reader admits can reach, and why. The census must not reach
/// them; were one reached, this list is wrong and the arm is held by the comparison.
const UNREACHED: &[(&str, &str)] = &[
    ("b: diagnostic_relative_ratio_non_governing", "no resident semantic table has a row of the category diagnostic_relative_ratio"),
    (
        "a: review downgraded for incomplete metadata, with a class disclosure",
        "only a successor's rows carry a class; no committed successor has a review row, and a review row added to a sealed \
         successor is refused by the reader (RETAINED_PRECISION_COVERAGE_MISMATCH, the input below)",
    ),
];

/// The arms one derivation reached.
fn census(source: &Value, document: &Value, arms: &mut BTreeSet<String>) {
    let (table, _) = for_source(source).unwrap();
    let e = &document["result_envelope"];
    let identity = source["producer"]["semantic_contract_id"].as_str().unwrap();
    arms.insert(format!("d: {}", identity.rsplit('/').next().unwrap()));
    let rows = source["results"].as_array().unwrap();
    for (index, row) in rows.iter().enumerate() {
        let account = &e["row_accounting"][index];
        let signature = signature_for(source, table, row).unwrap();
        let declared = signature.as_ref().map(|s| s["canonical_disposition"].as_str().unwrap()).unwrap_or("disclosed");
        let actual = account["disposition"].as_str().unwrap();
        let target = document.pointer(account["target_field_path"].as_str().unwrap()).unwrap();
        let reason = target["reason_code"].as_str().unwrap_or_default();
        let class = reason.starts_with("retained_precision_");
        if actual == "disclosed" {
            arms.insert(format!("b: {reason}"));
        }
        let family = signature.as_ref().map(|s| s["family"].clone()).unwrap_or(Value::Null);
        let mandatory = matches!(family.as_str(), Some("force" | "moment" | "section_property"));
        let review_missing = declared == "exported_review" && !complete_metadata(row);
        let physical_missing = declared == "exported_quantity" && mandatory && canonical_metadata_for(source, table, row).is_none();
        let with_class = if class { ", with a class disclosure" } else { "" };
        if declared == actual {
            arms.insert("a: declared disposition kept".into());
        } else if review_missing {
            arms.insert(format!("a: review downgraded for incomplete metadata{with_class}"));
        } else if physical_missing {
            arms.insert(format!("a: quantity downgraded for missing canonical metadata{with_class}"));
        } else if class {
            arms.insert("a: downgraded by a class disclosure alone".into());
        }
        let own_basis = row.get("basis_ref").filter(|b| !b.is_null()).is_some();
        match actual {
            "exported_review" => {
                arms.insert(format!("c: evidence kind {}", target["evidence_kind"].as_str().unwrap()));
                arms.insert(if own_basis { "c: target basis the row's own basis_ref".into() } else { "c: review basis from its metadata (no basis_ref)".into() });
            }
            "exported_quantity" => {
                if own_basis {
                    arms.insert("c: target basis the row's own basis_ref".into());
                } else {
                    arms.insert("c: quantity basis from the run (no basis_ref)".into());
                }
                if row["basis_ref"]["ref_type"] == "combination" && row["kind"].as_str().unwrap().starts_with("connector_") && target.get("metadata").is_some() {
                    arms.insert("combined connector row of a combination, exported".into());
                }
            }
            _ => {}
        }
    }
}

/// The successor's publication and receipt hashes after an edit (as the result export
/// successor tests reseal an edited statement).
fn reseal(mut raw: Value) -> Value {
    let mut public = raw.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    raw["retained_precision"]["body"]["publication_sha256"] = json!(domain_hash("retained_precision_publication_mp_v2", &public).unwrap());
    let body = raw["retained_precision"]["body"].clone();
    raw["retained_precision"]["receipt_sha256"] = json!(domain_hash("retained_precision_receipt_mp_v2", &body).unwrap());
    raw
}

/// The first row satisfying `which`.
fn first<'v>(source: &'v mut Value, which: impl Fn(&Value) -> bool) -> &'v mut Value {
    source["results"].as_array_mut().unwrap().iter_mut().find(|row| which(row)).expect("a row to edit")
}

fn declared(source: &Value, row: &Value) -> Option<(String, Value)> {
    let (table, _) = for_source(source).ok()?;
    let s = signature_for(source, table, row).ok()??;
    Some((s["canonical_disposition"].as_str()?.to_owned(), s["family"].clone()))
}

/// The row indices whose declared disposition and family satisfy `which`.
fn rows_where(source: &Value, which: impl Fn(&str, &Value) -> bool) -> Vec<usize> {
    source["results"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|(_, row)| declared(source, row).is_some_and(|(d, f)| which(&d, &f)))
        .map(|(index, _)| index)
        .collect()
}

fn successor(text: &str) -> (Value, Value) {
    let fixture = parse(text);
    (fixture["source"].clone(), fixture["invocation"]["request"]["model"].clone())
}

/// The generated inputs: committed sources as they are, and with the smallest edits that reach
/// an arm. An edit the product reader refuses is still compared (both must refuse alike).
fn inputs() -> Vec<Input> {
    let mut inputs = Vec::new();
    // Unedited committed sources: the arms they reach as they are.
    inputs.push(Input::attested("invented", parse(INVENTED), None));
    inputs.push(Input::received("invented (received)", parse(INVENTED), None));
    inputs.push(Input::attested("connected", parse(CONNECTED), None));
    inputs.push(Input::attested("timoshenko arc", parse(TIMOSHENKO), None));
    inputs.push(Input::attested("pressure connected", parse(PRESSURE), None));
    inputs.push(Input::attested("pressure friction sensitive", parse(FRICTION), None));
    inputs.push(Input::attested("pressure source n05", parse(PRESSURE_SOURCE), None));
    let (milestone, milestone_model) = successor(MILESTONE);
    inputs.push(Input::received("milestone successor", milestone.clone(), Some(milestone_model.clone())));
    let (exact, exact_model) = successor(EXACT);
    inputs.push(Input::received("exact successor", exact.clone(), Some(exact_model.clone())));
    let corpus = parse(COMBINATIONS);
    for index in [0, 2, 3] {
        let case = &corpus["cases"][index];
        inputs.push(Input::attested(format!("combination case {index}"), case["envelope"].clone(), Some(case["model"].clone())));
    }

    // Edits.
    let edit = |name: &str, text: &str, received: bool, change: &dyn Fn(&mut Value)| {
        let mut source = parse(text);
        change(&mut source);
        if received { Input::received(name, source, None) } else { Input::attested(name, source, None) }
    };
    let review = |s: &Value| rows_where(s, |d, _| d == "exported_review")[0];
    let force = |s: &Value| rows_where(s, |d, f| d == "exported_quantity" && f == "force")[0];
    let quantity = |s: &Value| rows_where(s, |d, _| d == "exported_quantity")[0];
    inputs.push(edit("a review row without a sign convention", INVENTED, false, &|s| {
        let i = review(s);
        s["results"][i]["metadata"].as_object_mut().unwrap().remove("sign_convention");
    }));
    inputs.push(edit("a force row without a sign convention", CONNECTED, false, &|s| {
        let i = force(s);
        s["results"][i]["metadata"].as_object_mut().unwrap().remove("sign_convention");
    }));
    inputs.push(edit("a pressure force row without a sign convention", PRESSURE, false, &|s| {
        let i = force(s);
        s["results"][i]["metadata"].as_object_mut().unwrap().remove("sign_convention");
    }));
    inputs.push(edit("a row of an unknown kind", CONNECTED, false, &|s| {
        first(s, |_| true)["kind"] = json!("document_stream_unknown_kind_v1");
    }));
    inputs.push(edit("a pressure row of an unknown kind", PRESSURE, false, &|s| {
        first(s, |_| true)["kind"] = json!("document_stream_unknown_kind_v1");
    }));
    inputs.push(edit("a quantity row without a basis", CONNECTED, false, &|s| {
        let i = quantity(s);
        s["results"][i].as_object_mut().unwrap().remove("basis_ref");
    }));
    inputs.push(edit("a quantity row with a null basis", CONNECTED, false, &|s| {
        let i = quantity(s);
        s["results"][i]["basis_ref"] = Value::Null;
    }));
    inputs.push(edit("a review row with its own basis", INVENTED, false, &|s| {
        let i = review(s);
        let basis = s["results"][i]["metadata"]["basis"].as_str().unwrap().to_owned();
        s["results"][i]["basis_ref"] = json!({"ref_type": "load_case", "ref_id": basis});
    }));
    inputs.push(edit("a dimension-absent carrier that declares a dimension", CONNECTED, true, &|s| {
        first(s, |_| true)["dimension"] = json!("force");
    }));
    // Successor rows downgraded for their metadata, with the publication resealed. The
    // receipt's row shape requires the five metadata members as text, so the edit empties
    // one rather than removing it.
    for (name, source, model) in [("milestone", &milestone, &milestone_model), ("exact", &exact, &exact_model)] {
        let mut source = source.clone();
        for i in rows_where(&source, |d, f| d == "exported_quantity" && (f == "force" || f == "moment")) {
            if !source["results"][i]["kind"].as_str().unwrap().contains("global_end") {
                source["results"][i]["metadata"]["sign_convention"] = json!("");
            }
        }
        inputs.push(Input::received(format!("{name} successor element force and moment rows with an empty sign convention"), reseal(source), Some(model.clone())));
    }
    // No committed successor has a review row: one appended to a sealed successor, resealed.
    let invented = parse(INVENTED);
    let review_row = invented["results"][review(&invented)].clone();
    let mut source = milestone.clone();
    let mut appended = review_row.clone();
    appended["id"] = json!("result:document-stream:appended-review");
    appended["metadata"]["sign_convention"] = json!("");
    source["results"].as_array_mut().unwrap().push(appended);
    inputs.push(Input::received("milestone successor with an appended review row", reseal(source), Some(milestone_model.clone())));
    inputs
}

#[test]
fn every_reachable_arm_is_reached_and_derived_alike() {
    let mut arms = BTreeSet::new();
    let mut refused = Vec::new();
    for input in inputs() {
        match compare(&input) {
            Outcome::Derived(document) => census(&input.source, &document, &mut arms),
            Outcome::Refused(text) => {
                if text == "CARRIER_DIMENSION_PRESENT" {
                    arms.insert("d: CARRIER_DIMENSION_PRESENT".to_string());
                }
                refused.push(format!("{}: {text}", input.name));
            }
        }
    }
    println!("refused alike:\n  {}", refused.join("\n  "));
    println!("reached:\n  {}", arms.iter().cloned().collect::<Vec<_>>().join("\n  "));
    for arm in &arms {
        assert!(ARMS.contains(&arm.as_str()), "an arm the census does not list: {arm}");
    }
    let missing: Vec<&str> = ARMS.iter().copied().filter(|arm| !arms.contains(*arm) && !UNREACHED.iter().any(|(u, _)| u == arm)).collect();
    let reached_unreachable: Vec<&str> = UNREACHED.iter().map(|(arm, _)| *arm).filter(|arm| arms.contains(*arm)).collect();
    assert!(missing.is_empty(), "arms not reached: {missing:?}");
    assert!(reached_unreachable.is_empty(), "arms listed unreachable were reached: {reached_unreachable:?}");
}
