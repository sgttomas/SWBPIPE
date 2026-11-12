//! Publication, slice 1d-α: the
//! headless pipeline's documents and refusals, the reader's standings over the
//! committed corpus (`fixtures/results/hanger_sizing_*.json`), the schema's
//! fixed members against the crate's, and the existing readers' actual handling
//! of the document. All values are invented.
//!
//! The corpus inputs are formed by the builders below. With
//! `HANGER_SIZING_WRITE_FIXTURES=1` the generator test rewrites the inputs, the
//! documents and the self-consistent forgery from this build; otherwise it checks
//! the committed inputs are the builders'. The corpus file's expected standings
//! are written by hand and are not generated.

#[path = "common/model_builder.rs"]
mod model_builder;

use model_builder::*;
use open_pipe_stress_hanger_sizing::derived::{self, SizingRun};
use open_pipe_stress_hanger_sizing::publish::{
    self, parse_sizing_request, size_hangers, sized_document, Published, BASIS_STATEMENT, LIMITATIONS, LOAD_CATEGORIES,
    REFUSAL_CODES, STATE_CODES,
};
use open_pipe_stress_hanger_sizing::reader::{self, Status, Supplied};
use open_pipe_stress_hanger_sizing::{library, size, verify, verification_standing, Positions};
use serde_json::{json, Value};
use std::path::PathBuf;

const STEEL_E: f64 = 2.0e11;
const ALPHA: f64 = 1.2e-5;

fn project() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn fixture_path(name: &str) -> PathBuf {
    project().join("fixtures/results").join(name)
}

fn read_text(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn read_json(name: &str) -> Value {
    serde_json::from_str(&read_text(name)).unwrap()
}

fn pretty(value: &Value) -> String {
    format!("{}\n", serde_json::to_string_pretty(value).unwrap())
}

fn writing() -> bool {
    std::env::var("HANGER_SIZING_WRITE_FIXTURES").is_ok_and(|v| v == "1")
}

// --- the invented inputs ------------------------------------------------------

fn provenance() -> Value {
    json!({
        "source_name": "Invented non-engineering fixture",
        "source_location": "invented-local:test-only",
        "source_license": "unknown",
        "contributor": "Test author",
        "contributor_certification": "Invented values, no engineering authority",
        "redistribution_status": "private_only",
        "review_status": "pending"
    })
}

fn quantity(magnitude: f64, unit: &str, dimension: &str) -> Value {
    json!({"magnitude": magnitude, "unit": unit, "dimension": dimension, "provenance": provenance()})
}

fn record(id: &str, hanger: Value) -> Value {
    json!({"hanger_id": id, "name": format!("Invented {id}"), "provenance": provenance(), "hanger": hanger})
}

fn variable(id: &str, dof: &str, k: (f64, &str), range: Option<(f64, f64, &str)>, travel: (f64, &str)) -> Value {
    let mut hanger = json!({
        "hanger_type": "variable_spring_hanger",
        "source_reference": format!("invented-local:{id}"),
        "stiffness": {"dof": dof, "value": quantity(k.0, k.1, "force_per_length")},
        "installed_load": quantity(1000.0, "N", "force"),
        "travel_range": quantity(travel.0, travel.1, "length"),
    });
    if let Some((min, max, unit)) = range {
        hanger["working_load_range"] = json!({"minimum": quantity(min, unit, "force"), "maximum": quantity(max, unit, "force")});
    }
    record(id, hanger)
}

fn constant(id: &str, range: (f64, f64, &str), travel: (f64, &str)) -> Value {
    record(id, json!({
        "hanger_type": "constant_effort_support",
        "source_reference": format!("invented-local:{id}"),
        "constant_load": quantity(4000.0, "N", "force"),
        "travel_range": quantity(travel.0, travel.1, "length"),
        "working_load_range": {"minimum": quantity(range.0, range.2, "force"), "maximum": quantity(range.1, range.2, "force")},
    }))
}

/// One invented library for every corpus input, partly in lbf, lbf/in, mm and
/// in, so that the readers' unit conversion is exercised.
fn library_document() -> Value {
    json!({
        "schema_version": "1.1.0",
        "hanger_library": {
            "library_id": "invented.hangers.sizing",
            "name": "Invented hanger sizing fixture only",
            "library_scope": "private_user_library",
            "privacy_class": "private_user_supplied",
            "provenance": provenance()
        },
        "hanger_records": [
            variable("VS-200-L", "UZ", (200000.0, "N/m"), Some((4000.0, 6000.0, "N")), (50.0, "mm")),
            variable("VS-200-S", "UZ", (200000.0, "N/m"), Some((4800.0, 5500.0, "N")), (50.0, "mm")),
            variable("VS-570-W", "UZ", (570.0, "lbf/in"), Some((1012.0, 2025.0, "lbf")), (2.4, "in")),
            variable("VS-500-T", "UZ", (500000.0, "N/m"), Some((1000.0, 20000.0, "N")), (2.0, "mm")),
            variable("VS-200-UY", "UY", (200000.0, "N/m"), Some((4000.0, 6000.0, "N")), (50.0, "mm")),
            variable("VS-NORANGE", "UZ", (200000.0, "N/m"), None, (50.0, "mm")),
            constant("CE-1", (3000.0, 4000.0, "N"), (100.0, "mm")),
            constant("CE-2", (4000.0, 5000.0, "N"), (0.1, "m")),
            constant("CE-3", (1012.0, 1124.0, "lbf"), (0.04, "in")),
        ]
    })
}

fn member<'a>(id: &'a str, from: &'a str, to: &'a str) -> Member<'a> {
    Member { id, from, to, outside_diameter_m: 0.2, wall_m: 0.01, elastic_modulus_pa: STEEL_E, expansion_per_k: ALPHA }
}

/// Two 4 m spans, the interior support B a candidate and a lift-off support at
/// C, so that every solve publishes a nonlinear state. A weight case and two
/// operating cases.
fn two_span_request() -> Value {
    let nodes = [("node:A", [0.0, 0.0, 0.0]), ("node:B", [4.0, 0.0, 0.0]), ("node:C", [8.0, 0.0, 0.0])];
    let members = [member("pipe:AB", "node:A", "node:B"), member("pipe:BC", "node:B", "node:C")];
    let supports = vec![
        support("support:A", "node:A", "guide", &["UX", "UY", "UZ"]),
        torsion_spring("support:A-torsion", "node:A"),
        support("support:B", "node:B", "vertical_support", &["UZ"]),
        support("support:C", "node:C", "guide", &["UY"]),
        lift_off("support:C-lift", "node:C"),
    ];
    let weight = |prefix: &str| vec![line_load(&format!("{prefix}-AB"), "pipe:AB", 1000.0), line_load(&format!("{prefix}-BC"), "pipe:BC", 1000.0)];
    let mut o1 = weight("load:O1");
    o1.push(nodal_fz("load:O1-lift", "node:B", 500.0));
    let mut o2 = weight("load:O2");
    o2.push(nodal_fz("load:O2-push", "node:B", -300.0));
    let cases = vec![case("load:W", weight("load:W")), case("load:O1", o1), case("load:O2", o2)];
    request(&nodes, &members, supports, cases)
}

/// Four 4 m spans with the three interior supports as candidates.
fn four_span_request() -> Value {
    let nodes = [
        ("node:A", [0.0, 0.0, 0.0]),
        ("node:B", [4.0, 0.0, 0.0]),
        ("node:C", [8.0, 0.0, 0.0]),
        ("node:D", [12.0, 0.0, 0.0]),
        ("node:E", [16.0, 0.0, 0.0]),
    ];
    let members = [
        member("pipe:AB", "node:A", "node:B"),
        member("pipe:BC", "node:B", "node:C"),
        member("pipe:CD", "node:C", "node:D"),
        member("pipe:DE", "node:D", "node:E"),
    ];
    let supports = vec![
        support("support:A", "node:A", "guide", &["UX", "UY", "UZ"]),
        torsion_spring("support:A-torsion", "node:A"),
        support("support:B", "node:B", "vertical_support", &["UZ"]),
        support("support:C", "node:C", "vertical_support", &["UZ"]),
        support("support:D", "node:D", "vertical_support", &["UZ"]),
        support("support:E", "node:E", "guide", &["UY", "UZ"]),
    ];
    let weight = |prefix: &str| -> Vec<Value> {
        ["AB", "BC", "CD", "DE"].iter().map(|p| line_load(&format!("{prefix}-{p}"), &format!("pipe:{p}"), 1000.0)).collect()
    };
    let mut o1 = weight("load:O1");
    o1.push(nodal_fz("load:O1-lift", "node:C", 400.0));
    let cases = vec![case("load:W", weight("load:W")), case("load:O1", o1)];
    request(&nodes, &members, supports, cases)
}

fn two_span_input() -> Value {
    json!({
        "request": two_span_request(),
        "sizing": {
            "weight_case": "load:W",
            "weight_support_state": "own_solve",
            "design_operating_case": "load:O1",
            "check_operating_cases": ["load:O2"],
            "candidates": [{"support": "support:B", "family_policy": "variable"}],
            "method": {"load_design": "hot", "variation_reference": "operating_load", "variation_limit": 0.25,
                       "travel_allowance_m": 0.002, "selection_rule": "smallest_capacity"},
            "solver_mode": "dense_scrutiny"
        },
        "library": library_document()
    })
}

/// Variable records all fail V under the small limit: B falls back to a
/// constant record, C (constant) is selected and D (variable) has no fit, so
/// the verification is conditional.
fn four_span_input() -> Value {
    json!({
        "request": four_span_request(),
        "sizing": {
            "weight_case": "load:W",
            "weight_support_state": "own_solve",
            "design_operating_case": "load:O1",
            "check_operating_cases": [],
            "candidates": [
                {"support": "support:B", "family_policy": "variable_else_constant"},
                {"support": "support:C", "family_policy": "constant"},
                {"support": "support:D", "family_policy": "variable"}
            ],
            "method": {"load_design": "cold", "variation_reference": "design_load", "variation_limit": 0.001,
                       "travel_allowance_m": 0.0, "selection_rule": "least_variation"},
            "solver_mode": "sparse_interactive"
        },
        "library": library_document()
    })
}

/// The two-span input naming an unknown candidate: refused by the engine.
fn refused_input() -> Value {
    let mut input = two_span_input();
    input["sizing"]["candidates"] = json!([
        {"support": "support:B", "family_policy": "variable"},
        {"support": "support:missing", "family_policy": "constant"}
    ]);
    input
}

const EX_REFERENCE: &str = include_str!("../../../../validation/references/hanger_sizing/ex_sizing.json");

/// A library of one exact-route reference model's invented catalogue.
fn catalogue_library(model: &Value) -> Value {
    let records: Vec<Value> = model["catalogue"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let id = r["hanger_id"].as_str().unwrap();
            let range = r["working_load_range_n"].as_array().map(|p| (p[0].as_f64().unwrap(), p[1].as_f64().unwrap(), "N"));
            let travel = (r["travel_range_m"].as_f64().unwrap(), "m");
            if r["hanger_type"] == "variable_spring_hanger" {
                variable(id, r["stiffness_dof"].as_str().unwrap(), (r["stiffness_n_per_m"].as_f64().unwrap(), "N/m"), range, travel)
            } else {
                constant(id, range.unwrap(), travel)
            }
        })
        .collect();
    json!({
        "schema_version": "1.1.0",
        "hanger_library": {
            "library_id": format!("invented.hangers.{}", model["id"].as_str().unwrap().to_lowercase()),
            "name": "Invented exact-route hanger sizing fixture only",
            "library_scope": "private_user_library",
            "privacy_class": "private_user_supplied",
            "provenance": provenance()
        },
        "hanger_records": records
    })
}

/// The headless input of one exact-route reference run (`ex_sizing.json`): the
/// model built as a product document on EX (`model_builder::exs_request`), the
/// run's request and the model's catalogue. The model carries no
/// `model_settings`, and no sized value depends on whether pressure elongation acts.
fn ex_input(model_index: usize, run_id: &str, solver_mode: &str) -> Value {
    let reference: Value = serde_json::from_str(EX_REFERENCE).unwrap();
    let model = &reference["models"][model_index];
    let run = model["runs"].as_array().unwrap().iter().find(|r| r["id"] == run_id).unwrap();
    let r = &run["request"];
    json!({
        "request": exs_request(model),
        "sizing": {
            "weight_case": r["weight_case"],
            "weight_support_state": "own_solve",
            "design_operating_case": r["design_operating_case"],
            "check_operating_cases": r["further_operating_cases"],
            "candidates": r["candidates"],
            "method": {"load_design": r["load_design"], "variation_reference": r["variation_reference"],
                       "variation_limit": r["variation_limit"], "travel_allowance_m": r["travel_allowance_m"],
                       "selection_rule": r["selection_rule"]},
            "solver_mode": solver_mode
        },
        "library": catalogue_library(model)
    })
}

/// EXS-A, run A-HOT: a pressurised load/reference-state document with a ledger
/// and moved supports; a variable and a constant selection, the constant one
/// failing its verified travel.
fn ex_pressurised_input() -> Value {
    ex_input(0, "A-HOT", "dense_scrutiny")
}

/// EXS-B, run B-COLD: a free riser on EX with no pressurised region.
fn ex_riser_input() -> Value {
    ex_input(1, "B-COLD", "sparse_interactive")
}

const INPUTS: [(&str, fn() -> Value); 5] = [
    ("hanger_sizing_input_two_span.json", two_span_input),
    ("hanger_sizing_input_four_span.json", four_span_input),
    ("hanger_sizing_input_refused.json", refused_input),
    ("hanger_sizing_input_ex_pressurised.json", ex_pressurised_input),
    ("hanger_sizing_input_ex_riser.json", ex_riser_input),
];

fn run_of(input: &Value) -> SizingRun {
    let request = parse_sizing_request(&input["sizing"], "$.sizing").unwrap();
    derived::run(&input["request"], &library::records(&input["library"]), &request).unwrap_or_else(|r| panic!("{r:?}"))
}

/// The self-consistent forgery: the two-span run with the check case's
/// position, and its verified position, moved by 0.4 mm, every dependent value
/// recomputed through the engine's own arithmetic. The setting (hot design on
/// the design case) and so every derived document are unchanged.
fn forgery(input: &Value) -> Value {
    let request = parse_sizing_request(&input["sizing"], "$.sizing").unwrap();
    let records = library::records(&input["library"]);
    let mut run = run_of(input);
    run.candidates[0].positions.u_operating_m[1] += 4.0e-4;
    run.results = size(&run.candidates, &records, &request.method).unwrap();
    let mut verifications = Vec::new();
    for ((candidate, result), old) in run.candidates.iter().zip(&run.results).zip(&run.verifications) {
        let mut verified: Positions = old.as_ref().unwrap().positions.clone();
        verified.u_operating_m[1] += 4.0e-4;
        verifications.push(verify(candidate, result, &records, &request.method, &verified).unwrap());
    }
    run.verifications = verifications;
    run.standing = verification_standing(&run.results);
    sized_document(&input["request"], &input["sizing"], &input["library"], &request, &run).unwrap()
}

#[test]
fn the_committed_inputs_are_the_builders() {
    for (name, build) in INPUTS {
        let input = build();
        let import = open_pipe_stress_library_import_document::validate_library_import(
            &input["library"],
            open_pipe_stress_library_import_document::LibraryKind::Hanger,
            open_pipe_stress_library_import_document::IntendedVisibility::Private,
        );
        assert!(import.accepted, "{name}: {:?}", import.findings);
        if writing() {
            std::fs::write(fixture_path(name), pretty(&input)).unwrap();
            let document = size_hangers(&pretty(&input)).into_document();
            std::fs::write(fixture_path(&name.replace("_input_", "_result_")), pretty(&document)).unwrap();
        }
        assert_eq!(read_json(name), input, "{name} is not the builder's; rerun with HANGER_SIZING_WRITE_FIXTURES=1");
    }
    let two_span = two_span_input();
    if writing() {
        std::fs::write(fixture_path("hanger_sizing_result_forgery.json"), pretty(&forgery(&two_span))).unwrap();
    }
    // The forgery's positions differ from the bound model's, and every value
    // recomputed with them is the engine's.
    let forged = read_json("hanger_sizing_result_forgery.json");
    let honest = read_json("hanger_sizing_result_two_span.json");
    assert_ne!(forged["result"]["candidates"][0]["operating"][1], honest["result"]["candidates"][0]["operating"][1]);
    assert_eq!(forged["result"]["solves"], honest["result"]["solves"]);
}

#[test]
fn the_corpus_documents_cover_each_outcome_both_families_and_both_standings() {
    let two = read_json("hanger_sizing_result_two_span.json");
    let four = read_json("hanger_sizing_result_four_span.json");
    let refused = read_json("hanger_sizing_result_refused.json");
    assert_eq!(two["status"], "sized");
    assert_eq!(two["result"]["verification_standing"], "unconditional");
    let c = &two["result"]["candidates"][0];
    assert_eq!((c["outcome"].as_str(), c["selected"]["family"].as_str()), (Some("selected"), Some("variable_spring")));
    assert_eq!(c["selected"]["hanger_id"], "VS-200-L");
    assert_eq!(two["result"]["solves"].as_array().unwrap().len(), 2 * 2 + 3);
    assert!(two["result"]["solves"].as_array().unwrap().iter().all(|s| s["states"]["support:C-lift"] == "active"));
    assert_eq!(four["result"]["verification_standing"], "conditional");
    let outcomes: Vec<(&str, Option<&str>)> = four["result"]["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| (c["outcome"].as_str().unwrap(), c["selected"]["hanger_id"].as_str()))
        .collect();
    assert_eq!(outcomes, [("selected_by_fallback", Some("CE-2")), ("selected", Some("CE-1")), ("no_record_fits", None)]);
    assert_eq!(four["result"]["solves"].as_array().unwrap().len(), 2 + 3);
    assert_eq!(refused["status"], "refused");
    assert_eq!(refused["refusals"][0]["code"], "HANGER_SIZING_CANDIDATE_UNKNOWN");
    assert!(!refused["inputs"]["sizing_request"].is_null());
    for document in [&two, &four, &refused] {
        assert_eq!(document["route"], "pressure_free");
    }
    // The exact route: 2n + 3 solves, a variable and a constant selection, the
    // constant one failing its verified travel; and a riser with no pressurised region.
    let pressurised = read_json("hanger_sizing_result_ex_pressurised.json");
    let riser = read_json("hanger_sizing_result_ex_riser.json");
    for document in [&pressurised, &riser] {
        assert_eq!((document["status"].as_str(), document["route"].as_str()), (Some("sized"), Some("exact_pressure")));
        assert_eq!(document["result"]["solves"].as_array().unwrap().len(), 2 * 2 + 3);
    }
    let selected: Vec<(&str, Option<&str>)> = pressurised["result"]["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| (c["selected"]["family"].as_str().unwrap(), c["selected"]["hanger_id"].as_str()))
        .collect();
    assert_eq!(selected, [("variable_spring", Some("VS-B")), ("constant_effort", Some("CE-1"))]);
    assert_eq!(pressurised["result"]["candidates"][1]["verification"]["failed_tests"], json!(["T"]));
    assert_eq!(riser["result"]["candidates"][0]["selected"]["hanger_id"], "VS-R1");
    for name in ["hanger_sizing_input_ex_pressurised.json", "hanger_sizing_input_ex_riser.json"] {
        let input = read_json(name);
        assert!(input["request"]["model"].get("model_settings").is_none(), "{name}");
        assert_eq!(input["request"]["model"]["pressure_contract"]["mode"], "exact_pressure_v3", "{name}");
    }
}

// --- the corpus ---------------------------------------------------------------

fn pointer_parent<'a>(doc: &'a mut Value, pointer: &str) -> (&'a mut Value, String) {
    let (parent, last) = pointer.rsplit_once('/').unwrap();
    (doc.pointer_mut(parent).unwrap_or_else(|| panic!("no {parent}")), last.replace("~1", "/").replace("~0", "~"))
}

/// The corpus mutations: `replace` (an existing member or element), `add` (an
/// object member) and `remove` (an object member).
pub fn mutate(doc: &mut Value, ops: &[Value]) {
    for op in ops {
        let path = op["path"].as_str().unwrap();
        match op["op"].as_str().unwrap() {
            "replace" => *doc.pointer_mut(path).unwrap_or_else(|| panic!("no {path}")) = op["value"].clone(),
            "add" => {
                let (parent, key) = pointer_parent(doc, path);
                assert!(parent.get(&key).is_none(), "add over an existing member {path}");
                parent[key.as_str()] = op["value"].clone();
            }
            "remove" => {
                let (parent, key) = pointer_parent(doc, path);
                parent.as_object_mut().unwrap().remove(&key).unwrap();
            }
            other => panic!("unknown op {other}"),
        }
    }
}

fn statuses(verdict: &Value) -> Vec<Value> {
    STANDINGS.iter().map(|standing| verdict[*standing]["status"].clone()).collect()
}

const STANDINGS: [&str; 5] = ["closure", "binding", "arithmetic_consistent_with_published_positions", "derivation", "reproduction"];

/// Every committed document and mutation reads with the corpus's expected
/// standings; the Python verifier is held to the same expectations for
/// `closure`, `binding` and arithmetic (`tests/test_hanger_sizing_evidence.py`).
///
/// Reproduction is claimed only for the same engine build on the same platform
/// (§8.3.7), and the committed sized documents were formed on one platform. A
/// genuine sized entry's reproduction is therefore `same_build`: it is not
/// asserted on the committed bytes. Instead the document is published afresh
/// from the committed input in-process, and that document must reproduce and
/// read with the entry's other expectations. Every other expectation (the
/// forgery's and the mutations' failed reproduction included) is asserted on
/// the committed bytes, where the outcome does not depend on the platform.
#[test]
fn the_reader_returns_the_corpus_standings() {
    let corpus = read_json("hanger_sizing_corpus.json");
    let entries = corpus["entries"].as_array().unwrap();
    assert!(entries.len() >= 12);
    let mut same_build = 0;
    for entry in entries {
        let id = entry["id"].as_str().unwrap();
        let mut doc = read_json(entry["document"].as_str().unwrap());
        mutate(&mut doc, entry["mutations"].as_array().unwrap());
        let input_name = entry["input"].as_str().unwrap();
        let input = read_json(input_name);
        let supply = entry["supply"].as_array().unwrap();
        let supplied = Supplied {
            solve_request: supply.contains(&json!("solve_request")).then_some(&input["request"]),
            library: supply.contains(&json!("library")).then_some(&input["library"]),
        };
        let expected = &entry["expected"];
        let mut wanted: Vec<Value> = STANDINGS.iter().map(|standing| expected[*standing].clone()).collect();
        let verdict = reader::read(&serde_json::to_string(&doc).unwrap(), supplied).to_json();
        if expected["reproduction"] == "same_build" {
            same_build += 1;
            assert!(entry["mutations"].as_array().unwrap().is_empty(), "{id}: same_build is for genuine documents");
            // The committed bytes: every standing but reproduction.
            assert_eq!(statuses(&verdict)[..4], wanted[..4], "{id}: {verdict}");
            // A document published afresh by this build from the committed input.
            let fresh = size_hangers(&read_text(input_name));
            assert!(fresh.is_sized(), "{id}");
            let fresh = reader::read(&serde_json::to_string(fresh.document()).unwrap(), supplied).to_json();
            wanted[4] = json!("passed");
            assert_eq!(statuses(&fresh), wanted, "{id}: the fresh document: {fresh}");
            assert_eq!(fresh["binding"]["not_checked"], expected["binding_not_checked"], "{id}");
        } else {
            assert_eq!(statuses(&verdict), wanted, "{id}: {verdict}");
        }
        assert_eq!(verdict["binding"]["not_checked"], expected["binding_not_checked"], "{id}");
    }
    assert_eq!(same_build, 4, "the four genuine sized entries, two on each route");
}

// --- the headless pipeline's refusals -----------------------------------------

fn refused(published: Published) -> Value {
    assert!(!published.is_sized());
    let doc = published.into_document();
    assert_eq!(reader::closure(&doc), Ok(()), "{doc}");
    doc
}

#[test]
fn an_input_outside_the_profile_is_refused_by_kind_and_path_without_its_value() {
    let text = pretty(&two_span_input());
    for (bad, kind, path) in [
        (text.replacen("\"variation_limit\": 0.25", "\"variation_limit\": 1e20", 1), "number_outside_profile", "$.sizing.method.variation_limit"),
        (text.replacen("\"variation_limit\": 0.25", "\"variation_limit\": 0.25, \"variation_limit\": 0.3", 1), "duplicate_key", "$.sizing.method.variation_limit"),
        (text.replacen("\"variation_limit\": 0.25", "\"variation_limit\": 1e999", 1), "number_outside_profile", "$.sizing.method.variation_limit"),
        (text.replacen("\"variation_limit\": 0.25", &format!("\"variation_limit\": {}", "9".repeat(5000)), 1), "number_outside_profile", "$.sizing.method.variation_limit"),
        (format!("{text} x"), "invalid_json", "$"),
    ] {
        let doc = refused(size_hangers(&bad));
        assert_eq!(doc["refusals"], json!([{"code": "HANGER_SIZING_INPUT_NOT_ADMITTED", "names": [kind, path],
            "message": "the input is outside the openpipestress_jcs_ijson_v1 profile; the finding names its kind and path"}]));
        assert_eq!(doc["inputs"], json!({"canonical_profile": "openpipestress_jcs_ijson_v1", "sizing_request": null,
            "sizing_request_sha256": null, "solve_request": {"sha256": null}, "library": {"library_id": null, "sha256": null}}));
        assert!(!doc.to_string().contains("1e20") && !doc.to_string().contains("0.3"));
        // No model was read, so no route is stated.
        assert_eq!(doc["route"], Value::Null);
    }
}

#[test]
fn shape_library_and_request_refusals_name_their_paths_and_keep_the_hashes_formed() {
    let mut input = two_span_input();
    input["extra"] = json!(1);
    input["request"]["settings"] = json!({});
    let doc = refused(size_hangers(&input.to_string()));
    assert_eq!(doc["refusals"][0]["code"], "HANGER_SIZING_INPUT_SHAPE_INVALID");
    assert_eq!(doc["refusals"][0]["names"], json!(["$.extra", "$.request.settings"]));
    assert_eq!(doc["inputs"]["solve_request"]["sha256"].as_str(), publish::checked_sha256(&input["request"]).as_deref());
    assert_eq!(doc["inputs"]["library"]["library_id"], "invented.hangers.sizing");

    let mut input = two_span_input();
    input["library"]["hanger_records"][0]["provenance"] = json!(null);
    input["sizing"]["method"]["variation_limit"] = json!("0.25");
    input["sizing"]["candidates"][0]["note"] = json!("x");
    input["sizing"].as_object_mut().unwrap().remove("solver_mode");
    let doc = refused(size_hangers(&input.to_string()));
    let codes: Vec<&str> = doc["refusals"].as_array().unwrap().iter().map(|r| r["code"].as_str().unwrap()).collect();
    assert_eq!(codes, ["HANGER_SIZING_LIBRARY_NOT_ADMITTED", "HANGER_SIZING_REQUEST_INVALID"]);
    assert!(doc["refusals"][0]["message"].as_str().unwrap().contains("hanger_records[0]"));
    assert_eq!(doc["refusals"][1]["names"], json!(["$.sizing.solver_mode", "$.sizing.candidates[0].note", "$.sizing.method.variation_limit"]));
    assert!(doc["inputs"]["sizing_request"].is_null() && doc["inputs"]["sizing_request_sha256"].is_null());
    assert_eq!(doc["inputs"]["library"]["sha256"].as_str(), publish::checked_sha256(&input["library"]).as_deref());

    // The engine's value checks stay the engine's, with the admitted request
    // carried; a refused document states its request's route.
    let mut input = two_span_input();
    input["sizing"]["method"]["variation_limit"] = json!(-1);
    input["request"]["model"]["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3"});
    let doc = refused(size_hangers(&input.to_string()));
    let codes: Vec<&str> = doc["refusals"].as_array().unwrap().iter().map(|r| r["code"].as_str().unwrap()).collect();
    assert_eq!(codes, ["HANGER_SIZING_VARIATION_LIMIT_INVALID"]);
    assert_eq!(doc["route"], "exact_pressure");
    assert_eq!(doc["inputs"]["sizing_request"], input["sizing"]);
    assert_eq!(doc["inputs"]["sizing_request_sha256"].as_str(), publish::checked_sha256(&input["sizing"]).as_deref());
}

#[test]
fn values_outside_the_profile_refuse_publication_by_owner_and_member() {
    let input = two_span_input();
    let request = parse_sizing_request(&input["sizing"], "$.sizing").unwrap();
    let publish = |run: &SizingRun| sized_document(&input["request"], &input["sizing"], &input["library"], &request, run);
    let honest = run_of(&input);
    assert!(publish(&honest).is_ok());

    // An integral W beyond 2^53 - 1: published nowhere, and the force derived
    // documents that carry it cannot be hashed.
    let mut run = honest.clone();
    run.candidates[0].w_n = 1.0e20;
    let refusals = publish(&run).unwrap_err();
    let first = &refusals[0];
    assert_eq!((first.code, first.names.as_slice()), ("HANGER_SIZING_RESULT_NOT_PUBLISHABLE", &["support:B".to_string(), "w_n".to_string()][..]));
    let unhashable: Vec<&str> = refusals.iter().filter(|r| r.code == "HANGER_SIZING_DERIVED_DOCUMENT_NOT_HASHABLE").map(|r| r.names[0].as_str()).collect();
    assert_eq!(unhashable, ["S_C", "S_O1", "S_O2"]);

    // A non-finite derived value, and a state code outside the four.
    let mut run = honest.clone();
    if let Some(open_pipe_stress_hanger_sizing::Selected::Variable { loads, .. }) = &mut run.results[0].selected {
        loads.envelope_n.1 = f64::INFINITY;
    }
    run.states[2].states.insert("support:C-lift".into(), "unknown".into());
    let refusals = publish(&run).unwrap_err();
    let named: Vec<(&str, Vec<String>)> = refusals.iter().map(|r| (r.code, r.names.clone())).collect();
    assert_eq!(named, [
        ("HANGER_SIZING_RESULT_NOT_PUBLISHABLE", vec!["support:B".to_string(), "selected.envelope_n[1]".to_string()]),
        ("HANGER_SIZING_RESULT_NOT_PUBLISHABLE", vec!["S_O1".to_string(), "states.support:C-lift".to_string()]),
    ]);
    // The pipeline publishes such refusals as a refused document.
    let doc = publish::refused_document(&Default::default(), &refusals);
    assert_eq!(reader::closure(&doc), Ok(()));
}

#[test]
fn a_refused_document_is_read_by_the_refused_rules() {
    let doc = read_json("hanger_sizing_result_refused.json");
    let input = read_json("hanger_sizing_input_refused.json");
    let verdict = reader::read_document(&doc, Supplied { solve_request: Some(&input["request"]), library: Some(&input["library"]) });
    assert_eq!(verdict.closure.status, Status::Passed);
    assert_eq!(verdict.binding.status, Status::Passed);
    assert_eq!([verdict.arithmetic.status, verdict.derivation.status, verdict.reproduction.status], [Status::NotChecked; 3]);
    // A null hash checks nothing; a supplied input that differs fails.
    let other = read_json("hanger_sizing_input_four_span.json");
    let verdict = reader::read_document(&doc, Supplied { solve_request: Some(&other["request"]), library: None });
    assert_eq!(verdict.binding.status, Status::Failed);
    assert_eq!(verdict.binding_not_checked, ["library"]);
}

// --- the schema's fixed members -----------------------------------------------

#[test]
fn the_schema_states_the_crates_fixed_members() {
    let schema: Value = serde_json::from_str(&std::fs::read_to_string(project().join("schemas/hanger_sizing_result.schema.json")).unwrap()).unwrap();
    let defs = &schema["$defs"];
    let texts = |items: &Value| -> Vec<String> {
        items["prefixItems"].as_array().unwrap().iter().map(|i| i["const"].as_str().unwrap().to_string()).collect()
    };
    assert_eq!(texts(&defs["Limitations"]), LIMITATIONS);
    assert_eq!(texts(&defs["BasisStatement"]), BASIS_STATEMENT);
    let enumeration = |v: &Value| -> Vec<String> { v["enum"].as_array().unwrap().iter().map(|i| i.as_str().unwrap().to_string()).collect() };
    assert_eq!(enumeration(&defs["RefusalCode"]), REFUSAL_CODES);
    assert_eq!(enumeration(&defs["LoadCategory"]), LOAD_CATEGORIES);
    assert_eq!(enumeration(&defs["StateCode"]), STATE_CODES);
    assert_eq!(defs["Identity"]["properties"]["document_kind"]["const"], publish::DOCUMENT_KIND);
    assert_eq!(defs["Identity"]["properties"]["schema_version"]["const"], publish::SCHEMA_VERSION);
    assert_eq!(defs["Identity"]["properties"]["semantic_contract_id"]["const"], publish::SEMANTIC_CONTRACT_ID);
    assert_eq!(enumeration(&defs["Route"]), publish::ROUTES);
}

// --- §8.3.8: the existing readers' actual handling ----------------------------

#[test]
fn the_mechanics_route_reader_and_the_result_export_derivative_refuse_the_document() {
    use open_pipe_stress_result_export::{derivative, reader_verdict};
    let model = read_json("hanger_sizing_input_two_span.json")["request"]["model"].clone();
    for name in ["hanger_sizing_result_two_span.json", "hanger_sizing_result_four_span.json", "hanger_sizing_result_refused.json"] {
        let doc = read_json(name);
        // The mechanics route reader's header dispatch (every route reader's and the
        // analysis-run producer's first step) refuses it: no route, no binding, not
        // eligible.
        let dispatch = reader_verdict::dispatch(&doc, Some(&model), None);
        assert_eq!(dispatch["ok"], false, "{name}");
        assert_eq!(dispatch["route"], "unsupported", "{name}");
        assert_eq!(dispatch["binding"], Value::Null, "{name}");
        assert_eq!(dispatch["error"]["code"], "SOURCE_SCHEMA_VERSION_UNSUPPORTED", "{name}");
        assert_eq!(dispatch["standing"]["eligible"], false, "{name}");
        assert_eq!(dispatch["standing"]["findings"][0], "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED", "{name}");
        // Each named route reader refuses it with its own code.
        let codes: Vec<Value> = reader_verdict::READERS
            .iter()
            .map(|reader| {
                let verdict = reader_verdict::validate(reader, &doc, Some(&model), None);
                assert_eq!(verdict["ok"], false, "{name}: {reader}");
                verdict["error"]["code"].clone()
            })
            .collect();
        assert_eq!(codes, [
            "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS",
            "PHYSICS_SOURCE_IDENTITY",
            "SOURCE_RECOVERY_METADATA_CONTRADICTION",
            "SOURCE_PHYSICS_EVIDENCE_SHAPE",
        ], "{name}");
        // The analysis-run and derivative builders' source check.
        assert_eq!(reader_verdict::source_recovery(&doc).unwrap_err(), "SOURCE_RECOVERY_METADATA_CONTRADICTION", "{name}");
        // The result-export derivative refuses to derive from it or validate against it.
        assert_eq!(derivative::derive_document(json!({}), &model, &doc, json!({}), None).unwrap_err(), "SOURCE_SCHEMA_VERSION_UNSUPPORTED", "{name}");
        assert_eq!(derivative::validate_document(&json!({}), &doc).unwrap_err(), "SOURCE_SCHEMA_VERSION_UNSUPPORTED", "{name}");
    }
}
