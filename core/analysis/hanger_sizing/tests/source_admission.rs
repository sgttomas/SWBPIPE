//! The source model's admission is not hidden by projection (DEL-04-11 REQ-003,
//! REQ-009). A derived document keeps one case and no combinations, so what the
//! model's own solve refuses in the complete source must refuse the request with
//! the solve's code before any derived solve.
//!
//! The source is admitted on its neutralised baseline (`derived::neutralised_source`:
//! the whole authored document with only the admitted candidates made rigid) by the
//! product's `source_admission::admit_source_before_assembly`.
//!
//! - Every committed sized input is a complete source that an ordinary solve of the
//!   product admits, with every case and combination it carries.
//! - Neutralisation changes only the candidates' family, restraints, hanger,
//!   stiffness and nonlinear members, and leaves the authored request unchanged.
//! - Both directions, one fault at a time: data sizing replaces on a candidate (a
//!   malformed hanger, an invalid stiffness, an unknown family token) cannot refuse
//!   the request; a candidate the rules forbid (a direction, a nonlinear member, a
//!   reference to it) is refused by the candidate rules and cannot slip through.
//! - Faults in a case sizing does not solve (a duplicated load id, an unresolved
//!   ledger source, a support state naming no support, a ledgered load whose target
//!   node is unknown) refuse the request; the codes and affected ids are the
//!   ordinary solve's of the same neutralised baseline. A stored primitive the case's
//!   ledger excludes stays excluded. On PF: a duplicated load id, and, in a case
//!   sizing does not select, a load on an unknown node or pipe, a load the load
//!   application refuses, and each material-selection fault (both selectors, an
//!   exact point a material lacks, a selected point without G, an unbracketed
//!   interpolation temperature); a valid exact or interpolated selection is
//!   admitted whether its case is selected or not.
//! - The local preparation before global assembly, in a case sizing does not
//!   select: on EX, the pressure region's unknown terminal node, unknown member,
//!   broken chain, interior terminal and repeated terminal; on PF, a selected basis
//!   without the thermal expansion a thermal load needs, and a nonzero imposed
//!   translation at a skew stop's node. Each is the ordinary solve's whole-document
//!   refusal of the same neutralised baseline (blocked, no result), with its code and
//!   affected ids. Controls: the same basis or skew stop without the fault is
//!   admitted and sized; a reused basis still prepares the case that reuses it and
//!   every case after it; warnings on sized inputs stay warnings and are the
//!   ordinary solve's. (A partial extent on a realized arc is the product module's
//!   unit test: no sized input has a curved bend.)
//! - The authored pressure-elongation setting: an empty, duplicated or unknown
//!   selection, a selection on a document without a pressure contract and a
//!   malformed setting each refuse the request through the real sizing entry, and an
//!   ordinary solve of the same source refuses it with the same code.
//! - A valid selection is admitted, and each derived document keeps it only for a
//!   case it selects; unselected cases carry no setting, and other settings are kept.
//!
//! These are admission and document checks. They set no numerical expectation: the
//! numerical evidence with pressure elongation selected is EXS-A's, in
//! `ex_references.rs`, for its pressure-independent vertical quantities only. Where
//! a run is sized here, its candidates are compared with the committed result of the
//! same fixture, to show the fault changed nothing, not as a reference.

#[path = "common/model_builder.rs"]
mod model_builder;

use open_pipe_stress_hanger_sizing::derived::{check_candidates, derived_document, neutralised_source, Role, Site};
use open_pipe_stress_hanger_sizing::publish::{checked_sha256, parse_sizing_request, size_hangers};
use open_pipe_stress_product_physics::source_admission::admit_source_before_assembly;
use open_pipe_stress_hanger_sizing::Dof;
use open_pipe_stress_product_physics::model_settings::{
    MODEL_SETTING_PRESSURE_ELONGATION_CASES_INVALID, MODEL_SETTING_PRESSURE_ELONGATION_CASE_UNKNOWN,
    MODEL_SETTING_PRESSURE_ELONGATION_WITHOUT_PRESSURE_CONTRACT,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::path::PathBuf;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive];

fn results_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results")
}

fn read_json(name: &str) -> Value {
    let path = results_dir().join(name);
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{name}: {e}"))).unwrap()
}

/// The ordinary solve's blocking codes for a complete solve request, or its refusal text.
fn ordinary_blocking(request: &Value, mode: PreviewSolverMode) -> Result<Vec<String>, String> {
    let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)?;
    Ok(envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect())
}

/// Every committed input whose committed result is sized.
fn sized_inputs() -> Vec<(String, Value)> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(results_dir()).unwrap() {
        let name = entry.unwrap().file_name().into_string().unwrap();
        let Some(stem) = name.strip_prefix("hanger_sizing_input_").and_then(|n| n.strip_suffix(".json")) else { continue };
        let result = format!("hanger_sizing_result_{stem}.json");
        if results_dir().join(&result).exists() && read_json(&result)["status"] == "sized" {
            found.push((stem.to_string(), read_json(&name)));
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));
    found
}

#[test]
fn every_committed_sized_input_is_a_complete_source_the_ordinary_solve_admits() {
    let inputs = sized_inputs();
    let names: Vec<&str> = inputs.iter().map(|(n, _)| n.as_str()).collect();
    // Both EX sources are among them.
    for ex in ["ex_pressurised", "ex_riser"] {
        assert!(names.contains(&ex), "{names:?}");
    }
    for (name, input) in &inputs {
        let request = &input["request"];
        assert!(request["model"]["load_cases"].as_array().unwrap().len() >= 2, "{name}: not a complete source");
        for mode in MODES {
            let blocking = ordinary_blocking(request, mode).unwrap_or_else(|e| panic!("{name} {mode:?}: refused: {e}"));
            assert!(blocking.is_empty(), "{name} {mode:?}: the complete source is not admitted: {blocking:?}");
        }
    }
}

fn with_setting(mut input: Value, setting: Value) -> Value {
    input["request"]["model"]["model_settings"] = json!({"pressure_elongation": setting});
    input
}

/// The real entry refuses the request naming the source, with the product's code,
/// before any derived solve; an ordinary solve of the same complete source refuses
/// it with that code too.
fn assert_source_refused(input: &Value, code: &str, what: &str) {
    let sized = size_hangers(&serde_json::to_string(input).unwrap());
    let document = sized.document();
    assert!(!sized.is_sized(), "{what}: sized: {document}");
    let refusals = document["refusals"].as_array().unwrap();
    assert_eq!(refusals.len(), 1, "{what}: {refusals:?}");
    assert_eq!(refusals[0]["code"], "HANGER_SIZING_DERIVED_SOLVE_FAILED", "{what}");
    assert_eq!(refusals[0]["names"], json!(["source"]), "{what}");
    assert!(refusals[0]["message"].as_str().unwrap().contains(code), "{what}: {}", refusals[0]["message"]);
    match ordinary_blocking(&input["request"], PreviewSolverMode::DenseScrutiny) {
        Ok(blocking) => assert!(blocking.iter().any(|c| c == code), "{what}: the ordinary solve does not refuse with {code}: {blocking:?}"),
        Err(text) => assert!(text.contains(code), "{what}: {text}"),
    }
}

#[test]
fn an_invalid_pressure_elongation_selection_refuses_the_request_with_the_solves_code() {
    let ex = read_json("hanger_sizing_input_ex_pressurised.json");
    let design = ex["sizing"]["design_operating_case"].as_str().unwrap().to_string();
    assert_source_refused(&with_setting(ex.clone(), json!({"operating_load_cases": []})),
        MODEL_SETTING_PRESSURE_ELONGATION_CASES_INVALID, "empty");
    assert_source_refused(&with_setting(ex.clone(), json!({"operating_load_cases": [design, design]})),
        MODEL_SETTING_PRESSURE_ELONGATION_CASES_INVALID, "duplicated");
    assert_source_refused(&with_setting(ex.clone(), json!({"operating_load_cases": [design, "no-such-case"]})),
        MODEL_SETTING_PRESSURE_ELONGATION_CASE_UNKNOWN, "unknown");
    // A malformed setting is refused by the solve's own request parse.
    let malformed = with_setting(ex.clone(), json!({"operating_load_cases": design}));
    let sized = size_hangers(&serde_json::to_string(&malformed).unwrap());
    assert!(!sized.is_sized());
    assert_eq!(sized.document()["refusals"][0]["names"], json!(["source"]), "{}", sized.document());
    assert!(run_linear_static_preview_value_with_mode(malformed["request"].clone(), PreviewSolverMode::DenseScrutiny).is_err());
    // Without a pressure contract.
    let pf = read_json("hanger_sizing_input_two_span.json");
    assert!(pf["request"]["model"].get("pressure_contract").is_none());
    let case = pf["sizing"]["design_operating_case"].clone();
    assert_source_refused(&with_setting(pf, json!({"operating_load_cases": [case]})),
        MODEL_SETTING_PRESSURE_ELONGATION_WITHOUT_PRESSURE_CONTRACT, "without a pressure contract");
}

#[test]
fn a_valid_selection_is_kept_only_in_the_derived_documents_of_the_cases_it_selects() {
    let ex = read_json("hanger_sizing_input_ex_pressurised.json");
    let cases: Vec<String> =
        ex["request"]["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap().to_string()).collect();
    let sites: Vec<Site> = ex["sizing"]["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            let id = c["support"].as_str().unwrap();
            let support = ex["request"]["model"]["supports"].as_array().unwrap().iter().find(|s| s["id"] == id).unwrap();
            Site { support: id.into(), node: support["node"].as_str().unwrap().into(), dof: Dof::Uz }
        })
        .collect();
    let design = ex["sizing"]["design_operating_case"].as_str().unwrap();
    let check = ex["sizing"]["check_operating_cases"][0].as_str().unwrap();
    let tolerance = json!({"value": 0.002, "unit": "m"});
    for selected in [vec![design], vec![check], vec![design, check]] {
        let mut input = ex.clone();
        input["request"]["model"]["model_settings"] = json!({"node_proximity_tolerance": tolerance,
            "pressure_elongation": {"operating_load_cases": selected}});
        let request = &input["request"];
        for case in &cases {
            for roles in [vec![Role::Rigid; sites.len()], vec![Role::Force(1000.0); sites.len()]] {
                let projected = derived_document(request, case, &sites, &roles);
                let settings = &projected["model"]["model_settings"];
                assert_eq!(settings["node_proximity_tolerance"], tolerance, "{selected:?} {case}: an unrelated setting was not kept");
                if selected.contains(&case.as_str()) {
                    assert_eq!(settings["pressure_elongation"], json!({"operating_load_cases": [case]}), "{selected:?} {case}");
                } else {
                    assert!(settings.get("pressure_elongation").is_none(), "{selected:?} {case}: {settings}");
                }
            }
        }
        // The real entry admits it, runs 2n + 3 solves and binds the complete
        // source as given, the setting included.
        let sized = size_hangers(&serde_json::to_string(&input).unwrap());
        let document = sized.document();
        assert!(sized.is_sized(), "{selected:?}: {document}");
        let n = 1 + ex["sizing"]["check_operating_cases"].as_array().unwrap().len();
        assert_eq!(document["result"]["solves"].as_array().unwrap().len(), 2 * n + 3, "{selected:?}");
        assert_eq!(document["inputs"]["solve_request"]["sha256"].as_str(), checked_sha256(request).as_deref(), "{selected:?}");
        // The stored model is unchanged by the run.
        assert_eq!(input["request"]["model"]["model_settings"]["pressure_elongation"]["operating_load_cases"], json!(selected));
    }
}

fn ex_input() -> Value {
    read_json("hanger_sizing_input_ex_pressurised.json")
}

/// The admitted candidates' sites, by the candidate rules on the unmodified source.
fn admitted_sites(input: &Value) -> Vec<Site> {
    let request = parse_sizing_request(&input["sizing"], "$.sizing").unwrap();
    check_candidates(&input["request"], &request).unwrap_or_else(|r| panic!("{r:?}"))
}

fn blocking_refs(diagnostics: &[open_pipe_stress_product_physics::Diagnostic]) -> Vec<(String, Vec<String>)> {
    diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| (d.code.clone(), d.affected_refs.clone())).collect()
}

/// The ordinary solve of a complete request: its blocking codes and affected ids,
/// or its refusal text.
fn ordinary_refs(request: &Value) -> Result<Vec<(String, Vec<String>)>, String> {
    run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::DenseScrutiny).map(|e| blocking_refs(&e.diagnostics))
}

fn sized(input: &Value) -> Value {
    let published = size_hangers(&serde_json::to_string(input).unwrap());
    assert!(published.is_sized(), "{}", published.document());
    published.document().clone()
}

/// A further case `U`, which no sizing role names: a copy of the check case O2 with
/// its own qualified ids, then changed by `change`.
fn with_unselected_case(mut input: Value, change: impl FnOnce(&mut Value)) -> Value {
    let o2 = input["request"]["model"]["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == "O2").unwrap().clone();
    let text = serde_json::to_string(&o2).unwrap().replace("\"O2:", "\"U:");
    let mut case: Value = serde_json::from_str(&text).unwrap();
    case["id"] = json!("U");
    change(&mut case);
    input["request"]["model"]["load_cases"].as_array_mut().unwrap().push(case);
    input
}

#[test]
fn neutralisation_changes_only_the_members_sizing_replaces() {
    let mut input = ex_input();
    // Ignored members on a candidate, so that removing them is observed.
    input["request"]["model"]["supports"][3]["hanger"] = json!({"not_a_member": true});
    let sites = admitted_sites(&input);
    let before = input.clone();
    let baseline = neutralised_source(&input["request"], &sites);
    assert_eq!(input, before, "the authored request is unchanged");
    let (original, neutral) = (&input["request"], &baseline);
    assert_eq!(original["materials"], neutral["materials"]);
    let (om, nm) = (original["model"].as_object().unwrap(), neutral["model"].as_object().unwrap());
    assert_eq!(om.keys().collect::<Vec<_>>(), nm.keys().collect::<Vec<_>>());
    for (key, value) in om {
        if key != "supports" {
            assert_eq!(&nm[key], value, "model.{key}");
        }
    }
    let (os, ns) = (om["supports"].as_array().unwrap(), nm["supports"].as_array().unwrap());
    assert_eq!(os.len(), ns.len());
    for (o, n) in os.iter().zip(ns) {
        let site = sites.iter().find(|s| o["id"] == s.support.as_str());
        let Some(site) = site else {
            assert_eq!(o, n, "a support that is not a candidate is unchanged");
            continue;
        };
        let replaced = ["family", "restraints", "hanger", "stiffness", "nonlinear"];
        for (key, value) in o.as_object().unwrap() {
            if !replaced.contains(&key.as_str()) {
                assert_eq!(&n[key], value, "{} {key}", site.support);
            }
        }
        let mut kept: Vec<&String> = n.as_object().unwrap().keys().filter(|k| !replaced.contains(&k.as_str())).collect();
        let mut authored: Vec<&String> = o.as_object().unwrap().keys().filter(|k| !replaced.contains(&k.as_str())).collect();
        kept.sort();
        authored.sort();
        assert_eq!(kept, authored, "{}: no member added or lost", site.support);
        assert_eq!(n["family"], "guide");
        assert_eq!(n["restraints"], json!(["UZ"]));
        for removed in ["hanger", "stiffness", "nonlinear"] {
            assert!(n.get(removed).is_none(), "{} {removed}", site.support);
        }
    }
}

#[test]
fn data_sizing_replaces_on_a_candidate_cannot_refuse_the_request() {
    let clean = ex_input();
    let committed = read_json("hanger_sizing_result_ex_pressurised.json");
    let sites = admitted_sites(&clean);
    let faults: [(&str, Value); 3] = [
        ("hanger", json!({"not_a_member": true, "hanger_type": 7})),
        ("stiffness", json!({"dof": "UZ", "value": {"value": 1.0, "unit": "not-a-unit"}})),
        ("family", json!("not_a_family")),
    ];
    for (member, value) in faults {
        let mut input = clean.clone();
        let candidate = input["request"]["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "H-3").unwrap();
        candidate[member] = value;
        // Without neutralisation the ordinary solve refuses the source.
        let refused = match ordinary_refs(&input["request"]) {
            Err(_) => true,
            Ok(blocking) => !blocking.is_empty(),
        };
        assert!(refused, "{member}: the fault is not one the ordinary solve refuses");
        assert_eq!(admitted_sites(&input).len(), sites.len(), "{member}: the candidate rules admit the candidate");
        assert_eq!(neutralised_source(&input["request"], &sites), neutralised_source(&clean["request"], &sites), "{member}");
        let document = sized(&input);
        assert_eq!(document["result"]["candidates"], committed["result"]["candidates"], "{member}");
        assert_eq!(document["result"]["solves"], committed["result"]["solves"], "{member}");
    }
}

#[test]
fn a_candidate_the_rules_forbid_cannot_slip_through_neutralisation() {
    let cases: [(&str, Box<dyn Fn(&mut Value)>); 3] = [
        ("HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED", Box::new(|input: &mut Value| {
            input["request"]["model"]["supports"][3]["direction"] = json!({"x": 0.0, "y": 0.0, "z": 1.0});
        })),
        ("HANGER_SIZING_CANDIDATE_FAMILY_UNSUPPORTED", Box::new(|input: &mut Value| {
            input["request"]["model"]["supports"][3]["nonlinear"] = json!({"kind": "lift_off", "dof": "UZ"});
        })),
        ("HANGER_SIZING_CANDIDATE_REFERENCED", Box::new(|input: &mut Value| {
            let referenced = with_unselected_case(input.clone(), |case| {
                case["primitive_loads"].as_array_mut().unwrap().push(json!({
                    "id": "U:to-H-3", "category": "imposed_displacement", "target": {"type": "support", "support": "H-3", "dof": "UZ"},
                    "direction": "UZ", "magnitude": {"value": 0.001, "unit": "m"}, "dimension": "displacement",
                    "provenance": "invented_test_value"}));
            });
            *input = referenced;
        })),
    ];
    for (code, change) in cases {
        let mut input = ex_input();
        assert_eq!(input["request"]["model"]["supports"][3]["id"], "H-3");
        change(&mut input);
        let published = size_hangers(&serde_json::to_string(&input).unwrap());
        assert!(!published.is_sized(), "{code}");
        let refusals = published.document()["refusals"].as_array().unwrap().clone();
        assert!(refusals.iter().any(|r| r["code"] == code && r["names"].as_array().unwrap().contains(&json!("H-3"))), "{code}: {refusals:?}");
        assert!(refusals.iter().all(|r| r["names"] != json!(["source"])), "{code}: judged as a source after neutralisation: {refusals:?}");
    }
}

/// One fault in the unselected case: the request is refused naming the source, and
/// the seam's blocking codes and affected ids are the ordinary solve's of the same
/// neutralised baseline, including `code` naming `object`.
fn assert_refused_like_the_ordinary_solve(input: &Value, code: &str, object: &str) {
    let baseline = neutralised_source(&input["request"], &admitted_sites(input));
    let expected = ordinary_refs(&baseline).unwrap_or_else(|e| panic!("{code}: refused at parse: {e}"));
    assert!(matches!(expected.as_slice(), [(c, refs)] if c == code && refs.iter().any(|r| r == object)),
        "{code}: one fault, but the ordinary solve gives {expected:?}");
    let seam = admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap();
    assert_eq!(blocking_refs(&seam), expected, "{code}");
    let published = size_hangers(&serde_json::to_string(input).unwrap());
    assert!(!published.is_sized(), "{code}: sized");
    let refusals = published.document()["refusals"].as_array().unwrap();
    assert_eq!(refusals.len(), 1, "{code}: {refusals:?}");
    assert_eq!(refusals[0]["names"], json!(["source"]));
    let message = refusals[0]["message"].as_str().unwrap();
    assert!(message.contains(code) && message.contains(object), "{code} {object}: {message}");
}

#[test]
fn a_duplicated_load_id_is_refused_with_the_ordinary_code() {
    // The saved probe's fault: one primitive-load id in two cases.
    let mut input = ex_input();
    let o2 = input["request"]["model"]["load_cases"].as_array_mut().unwrap().iter_mut().find(|c| c["id"] == "O2").unwrap();
    o2["primitive_loads"][0]["id"] = json!("W:F-1");
    o2["analysis_state"]["load_sources"][0]["source_ref"] = json!("W:F-1");
    assert_refused_like_the_ordinary_solve(&input, "DUPLICATE_ID", "W:F-1");
}

#[test]
fn an_unresolved_ledger_source_in_an_unselected_case_is_refused() {
    let input = with_unselected_case(ex_input(), |case| {
        case["analysis_state"]["load_sources"][0]["source_ref"] = json!("U:no-such-source");
    });
    assert_refused_like_the_ordinary_solve(&input, "LOAD_STATE_SOURCE_UNRESOLVED", "U:no-such-source");
}

#[test]
fn a_support_state_naming_no_support_in_an_unselected_case_is_refused() {
    let input = with_unselected_case(ex_input(), |case| {
        let mut extra = case["analysis_state"]["support_states"][0].clone();
        extra["support_ref"] = json!("support:not-in-model");
        extra.as_object_mut().unwrap().remove("boundary_motion");
        case["analysis_state"]["support_states"].as_array_mut().unwrap().push(extra);
    });
    assert_refused_like_the_ordinary_solve(&input, "LOAD_STATE_SUPPORT_UNKNOWN", "support:not-in-model");
}

#[test]
fn a_ledgered_load_on_an_unknown_node_in_an_unselected_case_is_refused() {
    let input = with_unselected_case(ex_input(), |case| {
        case["primitive_loads"][0]["target"]["node"] = json!("N-not-in-model");
    });
    assert_eq!(input["request"]["model"]["load_cases"][3]["analysis_state"]["load_sources"][0]["source_ref"], "U:F-1");
    assert_refused_like_the_ordinary_solve(&input, "LOAD_NODE_UNKNOWN", "U:F-1");
}

#[test]
fn a_primitive_the_ledger_excludes_stays_excluded() {
    // U:F-4 is stored in U and named by no ledger entry; an unknown target on it is
    // never judged, as the ordinary solve never applies it.
    let input = with_unselected_case(ex_input(), |case| {
        let load = case["primitive_loads"].as_array_mut().unwrap().iter_mut().find(|l| l["id"] == "U:F-4").unwrap();
        load["target"]["node"] = json!("N-not-in-model");
        assert!(!case["analysis_state"]["load_sources"].as_array().unwrap().iter().any(|s| s["source_ref"] == "U:F-4"));
    });
    let baseline = neutralised_source(&input["request"], &admitted_sites(&input));
    assert_eq!(ordinary_refs(&baseline), Ok(Vec::new()));
    assert_eq!(blocking_refs(&admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap()), Vec::new());
    let committed = read_json("hanger_sizing_result_ex_pressurised.json");
    assert_eq!(sized(&input)["result"]["candidates"], committed["result"]["candidates"]);
}

/// The pressure-free two-span input with a further case `load:U`, which no sizing
/// role names: a copy of `load:O2` with its own ids, then changed by `change`.
fn pf_with_unselected_case(change: impl FnOnce(&mut Value)) -> Value {
    let mut input = read_json("hanger_sizing_input_two_span.json");
    assert!(input["request"]["model"].get("pressure_contract").is_none());
    let o2 = input["request"]["model"]["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == "load:O2").unwrap().clone();
    let text = serde_json::to_string(&o2).unwrap().replace("\"load:O2", "\"load:U");
    let mut case: Value = serde_json::from_str(&text).unwrap();
    change(&mut case);
    input["request"]["model"]["load_cases"].as_array_mut().unwrap().push(case);
    input
}

#[test]
fn on_pf_isolated_faults_in_the_source_are_refused_like_the_ordinary_solve() {
    // A load id used in two cases.
    let mut input = read_json("hanger_sizing_input_two_span.json");
    let o2 = input["request"]["model"]["load_cases"].as_array_mut().unwrap().iter_mut().find(|c| c["id"] == "load:O2").unwrap();
    o2["primitive_loads"][2]["id"] = json!("load:O1-lift");
    assert_refused_like_the_ordinary_solve(&input, "DUPLICATE_ID", "load:O1-lift");
    // In a case sizing does not select: a load on an unknown node, and one on an unknown pipe.
    let input = pf_with_unselected_case(|case| {
        assert_eq!(case["primitive_loads"][2]["id"], "load:U-push");
        case["primitive_loads"][2]["target"]["node"] = json!("node:not-in-model");
    });
    assert_refused_like_the_ordinary_solve(&input, "LOAD_NODE_UNKNOWN", "load:U-push");
    let input = pf_with_unselected_case(|case| {
        case["primitive_loads"][0]["target"]["pipe"] = json!("pipe:not-in-model");
    });
    assert_refused_like_the_ordinary_solve(&input, "LOAD_PIPE_UNKNOWN", "load:U-AB");
    // Without a fault the further case is admitted and sizing is unchanged.
    let input = pf_with_unselected_case(|_| {});
    let committed = read_json("hanger_sizing_result_two_span.json");
    assert_eq!(sized(&input)["result"]["candidates"], committed["result"]["candidates"]);
}

/// The request is refused naming the source, and the seam's blocking diagnostics
/// are the ordinary solve's of the same neutralised baseline, each with `code`, and
/// one naming `object`. (A selection fault is reported per material, so a document
/// with two materials may give one diagnostic for each.)
fn assert_refused_alike(input: &Value, code: &str, object: &str) {
    let baseline = neutralised_source(&input["request"], &admitted_sites(input));
    let expected = ordinary_refs(&baseline).unwrap_or_else(|e| panic!("{code}: refused at parse: {e}"));
    assert!(!expected.is_empty() && expected.iter().all(|(c, _)| c == code), "{code}: the ordinary solve gives {expected:?}");
    assert!(expected.iter().any(|(_, refs)| refs.iter().any(|r| r == object)), "{code} {object}: {expected:?}");
    let seam = admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap();
    assert_eq!(blocking_refs(&seam), expected, "{code}");
    let published = size_hangers(&serde_json::to_string(input).unwrap());
    assert!(!published.is_sized(), "{code}: sized");
    let refusals = published.document()["refusals"].as_array().unwrap();
    assert_eq!(refusals.len(), 1, "{code}: {refusals:?}");
    assert_eq!(refusals[0]["names"], json!(["source"]));
    let message = refusals[0]["message"].as_str().unwrap();
    assert!(message.contains(code) && message.contains(object), "{code} {object}: {message}");
}

fn quantity(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// A user-entered temperature point with E, G and alpha.
fn point(id: &str, temperature_c: f64, e_pa: f64) -> Value {
    json!({"id": id, "temperature": quantity(temperature_c, "degC"), "elastic_modulus": quantity(e_pa, "Pa"),
           "shear_modulus": quantity(e_pa / 2.6, "Pa"), "thermal_expansion_coefficient": quantity(1.25e-5, "1/degC"),
           "provenance": "invented_test_value"})
}

/// The two-span input with the given points on every material, and the unselected
/// case `load:U` changed by `change`.
fn pf_with_points(points: Value, change: impl FnOnce(&mut Value)) -> Value {
    let mut input = pf_with_unselected_case(change);
    for material in input["request"]["model"]["materials"].as_array_mut().unwrap() {
        material["temperature_points"] = points.clone();
    }
    input
}

#[test]
fn on_pf_a_material_selection_fault_in_an_unselected_case_is_refused_like_the_ordinary_solve() {
    let points = json!([point("point:100", 100.0, 1.95e11), point("point:200", 200.0, 1.9e11)]);
    // Both selectors.
    let input = pf_with_points(points.clone(), |case| {
        case["modulus_basis_ref"] = json!("point:100");
        case["modulus_basis_temperature"] = quantity(150.0, "degC");
    });
    assert_refused_alike(&input, "MODULUS_BASIS_SELECTION_CONFLICT", "load:U");
    // An exact point no material stores.
    let input = pf_with_points(points.clone(), |case| case["modulus_basis_ref"] = json!("point:no-such-point"));
    assert_refused_alike(&input, "MODULUS_BASIS_UNRESOLVED", "point:no-such-point");
    // An exact point one material lacks.
    let mut input = pf_with_points(points.clone(), |case| case["modulus_basis_ref"] = json!("point:200"));
    input["request"]["model"]["materials"][1]["temperature_points"] = json!([point("point:100", 100.0, 1.95e11)]);
    assert_refused_alike(&input, "MODULUS_BASIS_UNRESOLVED", "material:pipe:BC");
    // A selected exact point without its shear modulus.
    let mut without_g = point("point:200", 200.0, 1.9e11);
    without_g.as_object_mut().unwrap().remove("shear_modulus");
    let input = pf_with_points(json!([point("point:100", 100.0, 1.95e11), without_g]), |case| case["modulus_basis_ref"] = json!("point:200"));
    assert_refused_alike(&input, "MODULUS_BASIS_INPUT_MISSING", "point:200");
    // An interpolation temperature outside the stored points.
    let input = pf_with_points(points.clone(), |case| case["modulus_basis_temperature"] = quantity(500.0, "degC"));
    assert_refused_alike(&input, "MODULUS_BASIS_UNRESOLVED", "load:U");
}

#[test]
fn on_pf_a_valid_material_selection_is_admitted_selected_or_not() {
    let points = json!([point("point:100", 100.0, 1.95e11), point("point:200", 200.0, 1.9e11)]);
    let committed = read_json("hanger_sizing_result_two_span.json");
    for (what, selector) in [("exact", json!({"modulus_basis_ref": "point:200"})), ("interpolated", json!({"modulus_basis_temperature": quantity(150.0, "degC")}))] {
        // In the unselected case: admitted, and sizing unchanged.
        let input = pf_with_points(points.clone(), |case| {
            for (key, value) in selector.as_object().unwrap() {
                case[key] = value.clone();
            }
        });
        let baseline = neutralised_source(&input["request"], &admitted_sites(&input));
        assert_eq!(ordinary_refs(&baseline), Ok(Vec::new()), "{what}");
        assert_eq!(blocking_refs(&admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap()), Vec::new(), "{what}");
        assert_eq!(sized(&input)["result"]["candidates"], committed["result"]["candidates"], "{what}");
        // In the selected check case: admitted and sized (its values are the
        // selected basis's; none is asserted here).
        let mut input = input.clone();
        let o2 = input["request"]["model"]["load_cases"].as_array_mut().unwrap().iter_mut().find(|c| c["id"] == "load:O2").unwrap();
        for (key, value) in selector.as_object().unwrap() {
            o2[key] = value.clone();
        }
        let baseline = neutralised_source(&input["request"], &admitted_sites(&input));
        assert_eq!(blocking_refs(&admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap()), Vec::new(), "{what}");
        sized(&input);
    }
}

#[test]
fn on_pf_a_load_application_fault_in_an_unselected_case_is_refused_like_the_ordinary_solve() {
    // A nodal force given the dimension of a moment: the load application refuses it.
    let input = pf_with_unselected_case(|case| {
        assert_eq!(case["primitive_loads"][2]["id"], "load:U-push");
        case["primitive_loads"][2]["dimension"] = json!("moment");
        case["primitive_loads"][2]["magnitude"]["unit"] = json!("N*m");
    });
    assert_refused_like_the_ordinary_solve(&input, "LOAD_INPUT_INVALID", "load:U-push");
}

/// The ordinary solve of the neutralised baseline refuses the whole document: its
/// blocking diagnostics, with no published result.
fn ordinary_document_blocked(input: &Value) -> Vec<(String, Vec<String>)> {
    let baseline = neutralised_source(&input["request"], &admitted_sites(input));
    let envelope = run_linear_static_preview_value_with_mode(baseline, PreviewSolverMode::DenseScrutiny).expect("parsed");
    let blocking = blocking_refs(&envelope.diagnostics);
    assert!(!blocking.is_empty(), "the ordinary solve admits it");
    assert!(envelope.results.is_empty(), "the ordinary solve published results beside {blocking:?}");
    blocking
}

/// A fault the local preparation reaches: equal to the ordinary solve of the
/// neutralised baseline, which refuses the whole document.
fn assert_local_refusal(input: &Value, code: &str, object: &str) {
    let blocking = ordinary_document_blocked(input);
    eprintln!("{code}: ordinary {blocking:?}");
    assert_refused_alike(input, code, object);
}

fn ex_region_change(change: impl FnOnce(&mut Value)) -> Value {
    with_unselected_case(ex_input(), |case| {
        assert_eq!(case["pressure_regions"][0]["id"], "U:PR-1");
        change(&mut case["pressure_regions"][0]);
    })
}

#[test]
fn on_ex_a_pressure_region_fault_in_an_unselected_case_is_refused_like_the_ordinary_solve() {
    let input = ex_region_change(|region| region["terminals"][1]["node_ref"] = json!("N-not-in-model"));
    assert_local_refusal(&input, "PRESSURE_TERMINAL_NODE_UNKNOWN", "U:PR-1");
    let input = ex_region_change(|region| region["member_pipe_ids"].as_array_mut().unwrap().push(json!("pipe:not-in-model")));
    assert_local_refusal(&input, "PRESSURE_REGION_MEMBER_UNKNOWN", "pipe:not-in-model");
    let input = ex_region_change(|region| region["member_pipe_ids"].as_array_mut().unwrap().retain(|m| m != "M3"));
    assert_local_refusal(&input, "PRESSURE_REGION_TOPOLOGY_INVALID", "U:PR-1");
    // A terminal at an interior node of the chain.
    let input = ex_region_change(|region| region["terminals"][1]["node_ref"] = json!("N4"));
    assert_local_refusal(&input, "PRESSURE_REGION_TOPOLOGY_INVALID", "U:PR-1");
    // One end named as both terminals.
    let input = ex_region_change(|region| region["terminals"][1]["node_ref"] = json!("N1"));
    assert_local_refusal(&input, "PRESSURE_TERMINALS_INVALID", "U:PR-1");
}

#[test]
fn on_pf_a_selected_basis_without_thermal_expansion_refuses_a_thermal_load_in_an_unselected_case() {
    let mut without_alpha = point("point:200", 200.0, 1.9e11);
    without_alpha.as_object_mut().unwrap().remove("thermal_expansion_coefficient");
    let points = json!([point("point:100", 100.0, 1.95e11), without_alpha]);
    let input = pf_with_points(points.clone(), |case| {
        case["modulus_basis_ref"] = json!("point:200");
        case["primitive_loads"].as_array_mut().unwrap().push(model_builder::thermal("load:U-t", "pipe:AB", 50.0));
    });
    assert_local_refusal(&input, "THERMAL_EXPANSION_INPUT_MISSING", "load:U");
    // Control: the same basis without a thermal load in the case is admitted.
    let input = pf_with_points(points, |case| case["modulus_basis_ref"] = json!("point:200"));
    let baseline = neutralised_source(&input["request"], &admitted_sites(&input));
    assert_eq!(ordinary_refs(&baseline), Ok(Vec::new()));
    assert_eq!(blocking_refs(&admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap()), Vec::new());
    sized(&input);
}

/// The four-span constant input with a skewed line stop at node D (a support no
/// role names) and a further case `load:U`, a copy of `load:O1`, changed by `change`.
fn skewed_four_span(change: impl FnOnce(&mut Value)) -> Value {
    let mut input = read_json("hanger_sizing_input_four_span_constant.json");
    input["request"]["model"]["supports"].as_array_mut().unwrap().push(json!({
        "id": "support:D-skew", "node": "node:D", "family": "line_stop", "restraints": ["UY"],
        "direction": {"x": 0.0, "y": 0.6, "z": 0.8}, "provenance": "invented_test_value"}));
    let o1 = input["request"]["model"]["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == "load:O1").unwrap().clone();
    let text = serde_json::to_string(&o1).unwrap().replace("\"load:O1", "\"load:U");
    let mut case: Value = serde_json::from_str(&text).unwrap();
    change(&mut case);
    input["request"]["model"]["load_cases"].as_array_mut().unwrap().push(case);
    input
}

#[test]
fn on_pf_a_moved_skew_stop_node_in_an_unselected_case_is_refused_like_the_ordinary_solve() {
    // Control: the skewed stop, with no motion, is admitted and sized.
    let input = skewed_four_span(|_| {});
    let baseline = neutralised_source(&input["request"], &admitted_sites(&input));
    assert_eq!(ordinary_refs(&baseline), Ok(Vec::new()));
    assert_eq!(blocking_refs(&admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap()), Vec::new());
    sized(&input);
    // A nonzero imposed translation of support D at the skew stop's node.
    let input = skewed_four_span(|case| {
        case["primitive_loads"].as_array_mut().unwrap().push(json!({
            "id": "load:U-move-D", "category": "imposed_displacement", "target": {"type": "support", "support": "support:D", "dof": "UZ"},
            "direction": "UZ", "magnitude": {"value": 0.002, "unit": "m"}, "dimension": "displacement", "provenance": "invented_test_value"}));
    });
    assert_local_refusal(&input, "SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED", "load:U");
}

/// The nonblocking outcomes of the ordinary solve stay nonblocking: every warning
/// the source admission reports on a sized input is one the ordinary solve of the
/// same baseline reports, and the run is sized.
#[test]
fn warnings_on_the_source_stay_warnings() {
    for name in ["hanger_sizing_input_ex_pressurised.json", "hanger_sizing_input_two_span.json", "hanger_sizing_input_four_span_constant.json"] {
        let input = read_json(name);
        let baseline = neutralised_source(&input["request"], &admitted_sites(&input));
        let envelope = run_linear_static_preview_value_with_mode(baseline.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
        let ordinary: Vec<(String, Vec<String>)> =
            envelope.diagnostics.iter().filter(|d| d.severity == "warning").map(|d| (d.code.clone(), d.affected_refs.clone())).collect();
        let seam = admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(blocking_refs(&seam), Vec::new(), "{name}");
        let warnings: Vec<(String, Vec<String>)> =
            seam.iter().filter(|d| d.severity == "warning").map(|d| (d.code.clone(), d.affected_refs.clone())).collect();
        for warning in &warnings {
            assert!(ordinary.contains(warning), "{name}: {warning:?} is not an ordinary warning: {ordinary:?}");
        }
        eprintln!("{name}: seam warnings {warnings:?}; ordinary warnings {ordinary:?}");
        sized(&input);
    }
}

/// The two-span input with further cases, none named by a sizing role: one copy of
/// `load:O2` per `(id, change)`, in order, with the given points on every material.
fn pf_with_cases(points: &Value, cases: Vec<(&str, Box<dyn Fn(&mut Value)>)>) -> Value {
    let mut input = read_json("hanger_sizing_input_two_span.json");
    for material in input["request"]["model"]["materials"].as_array_mut().unwrap() {
        material["temperature_points"] = points.clone();
    }
    let o2 = input["request"]["model"]["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == "load:O2").unwrap().clone();
    for (id, change) in cases {
        let text = serde_json::to_string(&o2).unwrap().replace("\"load:O2", &format!("\"{id}"));
        let mut case: Value = serde_json::from_str(&text).unwrap();
        change(&mut case);
        input["request"]["model"]["load_cases"].as_array_mut().unwrap().push(case);
    }
    input
}

/// A basis is resolved once per key and its built model reused; every case-scoped
/// check still runs on each case that reuses it, and every case after it is still
/// prepared. Valid cases stand before and after the repeated-basis case.
#[test]
fn on_pf_a_reused_basis_still_prepares_every_case_that_uses_it_and_every_case_after_it() {
    let mut without_alpha = point("point:200", 200.0, 1.9e11);
    without_alpha.as_object_mut().unwrap().remove("thermal_expansion_coefficient");
    let points = json!([point("point:100", 100.0, 1.95e11), without_alpha]);
    let select = |case: &mut Value| case["modulus_basis_ref"] = json!("point:200");
    let thermal = |case: &mut Value| {
        let id = format!("{}-t", case["id"].as_str().unwrap());
        case["primitive_loads"].as_array_mut().unwrap().push(model_builder::thermal(&id, "pipe:AB", 50.0));
    };
    // All valid: the first use of the basis, a case on the base materials, the
    // basis reused, and a case after it.
    let valid = pf_with_cases(&points, vec![
        ("load:U1", Box::new(select)),
        ("load:U2", Box::new(|_: &mut Value| {})),
        ("load:U3", Box::new(select)),
        ("load:U4", Box::new(|_: &mut Value| {})),
    ]);
    let baseline = neutralised_source(&valid["request"], &admitted_sites(&valid));
    assert_eq!(ordinary_refs(&baseline), Ok(Vec::new()));
    assert_eq!(blocking_refs(&admit_source_before_assembly(baseline, PreviewSolverMode::DenseScrutiny).unwrap()), Vec::new());
    sized(&valid);
    // The reused basis's case carries a thermal load its basis cannot expand.
    let reused_fault = pf_with_cases(&points, vec![
        ("load:U1", Box::new(select)),
        ("load:U2", Box::new(|_: &mut Value| {})),
        ("load:U3", Box::new(move |case: &mut Value| { select(case); thermal(case); })),
        ("load:U4", Box::new(|_: &mut Value| {})),
    ]);
    assert_local_refusal(&reused_fault, "THERMAL_EXPANSION_INPUT_MISSING", "load:U3");
    // A fault in the case after the reused basis.
    let after_fault = pf_with_cases(&points, vec![
        ("load:U1", Box::new(select)),
        ("load:U2", Box::new(|_: &mut Value| {})),
        ("load:U3", Box::new(select)),
        ("load:U4", Box::new(|case: &mut Value| case["primitive_loads"][2]["target"]["node"] = json!("node:not-in-model"))),
    ]);
    assert_refused_like_the_ordinary_solve(&after_fault, "LOAD_NODE_UNKNOWN", "load:U4-push");
}
