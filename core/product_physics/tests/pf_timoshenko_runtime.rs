//! PF Timoshenko (DEL-05-03 Design `pf-timoshenko.md` §§2–5) at the public value
//! entry, in both solver modes: each refusal and warning by name, admission
//! following consumption (§2, "Construction boundary"), the named arc disclosure
//! and its bound (§4), the result identity on blocked envelopes, and the
//! geometry-only bend's straight-member rows. The documents are the frozen
//! `validation/references/pf_timoshenko` requests used as templates and changed
//! in memory; no expected value here is a reference row (those are judged in
//! `pf_timoshenko_references.rs`).
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const CANTILEVER: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json");
const ONE_WAY: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-NL-ONEWAY-TIMOSHENKO.request.json");
const ARC_TIP: &str = include_str!("../../../validation/references/pf_timoshenko/requests/PFT-ARC-TIP-TIMOSHENKO.request.json");

const TIMOSHENKO_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
const EULER_BERNOULLI_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const PARTIAL_SPAN: &str = "PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED";
const FACTOR: &str = "PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE";
const AUTHORED_NU: &str = "PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED";
const ARCS: &str = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";

fn template(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// The one-way template, with the solver seed its frozen form lacks (see
/// `pf_timoshenko_references.rs`, "Two request-authoring defects").
fn one_way() -> Value {
    let mut request = template(ONE_WAY);
    for support in request["model"]["supports"].as_array_mut().unwrap() {
        if let Some(nonlinear) = support.get_mut("nonlinear") {
            nonlinear["initial_state"] = json!("inactive");
        }
    }
    request
}

fn euler_bernoulli(request: &Value) -> Value {
    let mut request = request.clone();
    request["model"].as_object_mut().unwrap().remove("model_settings");
    request
}

fn solve(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).expect("admitted at the wire")).unwrap()
}

fn with_code<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == json!(code)).collect()
}

fn solved(envelope: &Value, context: &str) {
    assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{context}: {}", envelope["diagnostics"]);
}

/// Blocked by `code` (one diagnostic per refused member), with no rows, under the identity
/// its document selects; the first of them.
fn blocked_by<'a>(envelope: &'a Value, code: &str, identity: &str, context: &str) -> &'a Value {
    assert_ne!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{context}");
    assert!(envelope["results"].as_array().is_none_or(Vec::is_empty), "{context}: no rows");
    assert_eq!(envelope["producer"]["semantic_contract_id"], json!(identity), "{context}: the blocked envelope's identity");
    let found = with_code(envelope, code);
    assert!(!found.is_empty(), "{context}: {code} in {}", envelope["diagnostics"]);
    assert!(found.iter().all(|d| d["severity"] == json!("blocking")), "{context}");
    found[0]
}

fn case_ids(request: &Value) -> Vec<String> {
    request["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap().to_owned()).collect()
}

fn set_moduli(request: &mut Value, elastic: f64, shear: f64) {
    let material = &mut request["model"]["materials"][0];
    material["elastic_modulus"] = json!({"value": elastic, "unit": "Pa"});
    material["shear_modulus"] = json!({"value": shear, "unit": "Pa"});
}

/// Every published physical row's value by (kind, entity_ref, component, location, load
/// case). A selected basis's `modulus_basis_record` row states the selection itself, so a
/// base-authored comparison document has none; it is left out.
fn values(envelope: &Value) -> BTreeMap<(String, String, String, String, String), f64> {
    let text = |value: &Value| value.as_str().unwrap_or("").to_owned();
    envelope["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["kind"] != json!("modulus_basis_record"))
        .map(|r| {
            let key = (text(&r["kind"]), text(&r["entity_ref"]), text(&r["metadata"]["component"]), text(&r["metadata"]["location"]), text(&r["basis_ref"]["ref_id"]));
            (key, r["value"].as_f64().unwrap())
        })
        .collect()
}

/// The keys whose values differ, or that only one side publishes.
fn differences(left: &BTreeMap<(String, String, String, String, String), f64>, right: &BTreeMap<(String, String, String, String, String), f64>) -> Vec<String> {
    let mut keys: Vec<_> = left.keys().chain(right.keys()).collect();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .filter(|key| left.get(*key).map(|v| v.to_bits()) != right.get(*key).map(|v| v.to_bits()))
        .map(|key| format!("{key:?}: {:?} against {:?}", left.get(key), right.get(key)))
        .collect()
}

/// The rows of `case` only.
fn case_values(envelope: &Value, case: &str) -> BTreeMap<(String, String, String, String, String), f64> {
    values(envelope).into_iter().filter(|(key, _)| key.4 == case).collect()
}

// ---------------------------------------------------------------- refusals

/// §3: a wind exposed span (a partial-span member load) on a Timoshenko straight member is
/// refused by name, naming the member and the load, before any solve. The Euler–Bernoulli
/// twin and a whole-span marking on the Timoshenko document both solve.
#[test]
fn a_partial_span_on_a_timoshenko_member_is_refused_by_name() {
    let base = template(CANTILEVER);
    let pipe = base["model"]["pipe_segments"][0]["id"].as_str().unwrap().to_owned();
    let case = case_ids(&base)[0].clone();
    let wind = |spans: Value, refs: Value| {
        let mut request = base.clone();
        request["model"]["load_cases"][0]["equivalent_static"] = json!({
            "wind": {"pressure": {"value": 480.0, "unit": "Pa"}, "shape_factor": {"value": 0.7, "unit": "1"},
                "direction": "global_y", "exposed_pipe_refs": refs, "exposed_spans": spans},
            "provenance": "invented PF Timoshenko test input"});
        request
    };
    let partial = wind(json!([{"pipe_ref": pipe, "start_fraction": {"value": 0.25, "unit": "1"}, "end_fraction": {"value": 0.75, "unit": "1"}}]), json!([]));
    let whole = wind(json!([]), json!([pipe]));
    for mode in MODES {
        let envelope = solve(&partial, mode);
        let refusal = blocked_by(&envelope, PARTIAL_SPAN, TIMOSHENKO_ID, &format!("{mode:?}"));
        assert_eq!(refusal["affected_refs"], json!([pipe, case]));
        assert_eq!(refusal["message"], json!(format!(
            "load case {case} equivalent_static.wind.exposed_spans[0] is a partial-span member load on timoshenko_annulus_energy_v1 straight member {pipe}; partial-span loads on Timoshenko members are not yet supported, so mark the whole span (exposed_pipe_refs) or select euler_bernoulli")));
        let twin = solve(&euler_bernoulli(&partial), mode);
        solved(&twin, "the Euler-Bernoulli twin");
        assert!(with_code(&twin, PARTIAL_SPAN).is_empty());
        let whole = solve(&whole, mode);
        solved(&whole, "a whole-span marking");
        assert_eq!(whole["producer"]["semantic_contract_id"], json!(TIMOSHENKO_ID));
    }
}

/// §2: ν = E/(2G) − 1 outside the factor's range blocks the solve by name, naming the member,
/// the material, the consuming cases and the basis; there is no default factor. The
/// Euler–Bernoulli twin forms no factor and solves.
#[test]
fn an_out_of_range_shear_factor_blocks_by_name() {
    let mut request = template(CANTILEVER);
    set_moduli(&mut request, 210e9, 60e9);
    let cases = case_ids(&request);
    for mode in MODES {
        let envelope = solve(&request, mode);
        let refusal = blocked_by(&envelope, FACTOR, TIMOSHENKO_ID, &format!("{mode:?}"));
        let mut refs = vec![json!("pipe:AB"), json!("mat:A")];
        refs.extend(cases.iter().map(|c| json!(c)));
        assert_eq!(refusal["affected_refs"], json!(refs));
        let message = refusal["message"].as_str().unwrap();
        assert!(message.starts_with("timoshenko_annulus_energy_v1 member pipe:AB has no energy-matched annular shear factor on the base material basis (no temperature selection) (load cases "), "{message}");
        assert!(message.contains("nu = E/(2G) - 1 = 0.75 from material mat:A E = 210000000000.0 Pa and G = 60000000000.0 Pa"), "{message}");
        assert!(message.ends_with("no default factor or Euler-Bernoulli member is used"), "{message}");
        solved(&solve(&euler_bernoulli(&request), mode), "the Euler-Bernoulli twin");
    }
}

/// §2: an authored `poisson_ratio` that differs from E/(2G) − 1 by more than 1e-6 of the larger
/// magnitude is disclosed (not consumed): one warning per material and basis. Equal values,
/// two zeros and a difference inside the threshold give none; an Euler–Bernoulli document none.
#[test]
fn the_authored_poisson_ratio_warning_follows_its_threshold() {
    let derived = 200e9 / (2.0 * 77e9) - 1.0;
    let authored = |request: &Value, nu: f64| {
        let mut request = request.clone();
        request["model"]["materials"][0]["poisson_ratio"] = json!({"value": nu, "unit": "1"});
        request
    };
    let base = template(CANTILEVER);
    let mut zero = base.clone();
    set_moduli(&mut zero, 160e9, 80e9);
    let cases = case_ids(&base);
    for mode in MODES {
        let warned = solve(&authored(&base, 0.3), mode);
        solved(&warned, "authored 0.3");
        let found = with_code(&warned, AUTHORED_NU);
        assert_eq!(found.len(), 1, "{}", warned["diagnostics"]);
        assert_eq!(found[0]["severity"], json!("warning"));
        let mut refs = vec![json!("mat:A")];
        refs.extend(cases.iter().map(|c| json!(c)));
        assert_eq!(found[0]["affected_refs"], json!(refs));
        assert_eq!(found[0]["message"], json!(format!(
            "material mat:A authors poisson_ratio 0.3, which is not used: its timoshenko_annulus_energy_v1 straight members on the base material basis (no temperature selection) (load cases {}) take nu = E/(2G) - 1 = {derived:?} from E = 200000000000.0 Pa and G = 77000000000.0 Pa",
            cases.join(", "))));
        for (label, request, warns) in [
            ("equal", authored(&base, derived), false),
            ("inside the threshold", authored(&base, derived * (1.0 + 5e-7)), false),
            ("outside the threshold", authored(&base, derived * (1.0 + 2e-6)), true),
            ("two zeros", authored(&zero, 0.0), false),
            ("a zero and a nonzero", authored(&zero, 1e-12), true),
            ("Euler-Bernoulli", euler_bernoulli(&authored(&base, 0.3)), false),
        ] {
            let envelope = solve(&request, mode);
            solved(&envelope, label);
            assert_eq!(with_code(&envelope, AUTHORED_NU).len(), usize::from(warns), "{label} {mode:?}");
        }
    }
}

/// §3: an authored objective connector keeps PF's existing refusal on a Timoshenko document.
#[test]
fn an_objective_connector_keeps_its_refusal_on_a_timoshenko_document() {
    let mut request = template(CANTILEVER);
    request["model"]["components"] = json!([{"id": "component:connector", "kind": "expansion_joint", "node": "node:B",
        "objective_connector": {"version": "1.0.0"}, "provenance": "invented PF Timoshenko test input"}]);
    for mode in MODES {
        let envelope = solve(&request, mode);
        let refusal = blocked_by(&envelope, "PREVIEW_CONTRACT_VERSION_MISMATCH", TIMOSHENKO_ID, &format!("{mode:?}"));
        assert_eq!(refusal["affected_refs"], json!(["component:connector", "objective_connector"]));
    }
}

// ------------------------------------------------- admission follows consumption

/// The one-way template's material with temperature points, and each case's selection.
fn selecting(base: (f64, f64), points: Value, selections: &[Option<Value>]) -> Value {
    let mut request = one_way();
    set_moduli(&mut request, base.0, base.1);
    request["model"]["materials"][0]["temperature_points"] = points;
    let cases = request["model"]["load_cases"].as_array_mut().unwrap();
    assert_eq!(cases.len(), selections.len());
    for (case, selection) in cases.iter_mut().zip(selections) {
        match selection {
            Some(Value::String(point)) => case["modulus_basis_ref"] = json!(point),
            Some(temperature) => case["modulus_basis_temperature"] = temperature.clone(),
            None => {}
        }
    }
    request
}

fn point(id: &str, kelvin: f64, elastic: f64, shear: f64) -> Value {
    json!({"id": id, "temperature": {"value": kelvin, "unit": "K"}, "elastic_modulus": {"value": elastic, "unit": "Pa"},
        "shear_modulus": {"value": shear, "unit": "Pa"}, "thermal_expansion_coefficient": {"value": 1.2e-5, "unit": "1/degC"}})
}

/// The one-way template solved on base moduli (E, G), for comparison.
fn plain(elastic: f64, shear: f64, mode: PreviewSolverMode) -> Value {
    let mut request = one_way();
    set_moduli(&mut request, elastic, shear);
    let envelope = solve(&request, mode);
    solved(&envelope, "the plain one-way document");
    envelope
}

/// §2: a base basis whose ν is out of range (E 210 GPa, G 60 GPa: ν = 0.75) does not block a
/// document whose every case selects a temperature point, by name and by interpolation,
/// with a nonlinear support. The seed is never solved; each case solves on its own basis,
/// with the same row values as a document authoring that basis's moduli as its base.
#[test]
fn an_unconsumed_out_of_range_base_does_not_block_named_or_interpolated_selections() {
    let points = json!([point("point:cold", 300.0, 190e9, 74e9), point("point:hot", 400.0, 210e9, 80e9), point("point:mid", 500.0, 200e9, 77e9)]);
    let named = selecting((210e9, 60e9), points.clone(), &[Some(json!("point:mid")), Some(json!("point:mid"))]);
    // 350 K lies halfway between 300 K and 400 K: E = 200 GPa and G = 77 GPa exactly.
    let interpolated = selecting((210e9, 60e9), points, &[Some(json!({"value": 350.0, "unit": "K"})), Some(json!({"value": 350.0, "unit": "K"}))]);
    assert!(one_way()["model"]["supports"].as_array().unwrap().iter().any(|s| s.get("nonlinear").is_some()), "a nonlinear support");
    for mode in MODES {
        let expected = values(&plain(200e9, 77e9, mode));
        for (label, request) in [("named", &named), ("interpolated", &interpolated)] {
            let envelope = solve(request, mode);
            solved(&envelope, label);
            assert!(with_code(&envelope, FACTOR).is_empty(), "{label}");
            assert_eq!(envelope["producer"]["semantic_contract_id"], json!(TIMOSHENKO_ID));
            assert_eq!(differences(&values(&envelope), &expected), Vec::<String>::new(), "{label} {mode:?}: the rows of the selected basis");
        }
    }
}

/// §2: a consumed base basis whose ν is out of range blocks by name, naming only the cases
/// that consume it.
#[test]
fn a_consumed_out_of_range_base_blocks_naming_its_cases() {
    let points = json!([point("point:hot", 400.0, 200e9, 77e9)]);
    let request = selecting((210e9, 60e9), points, &[None, Some(json!("point:hot"))]);
    let cases = case_ids(&request);
    for mode in MODES {
        let envelope = solve(&request, mode);
        let refusal = blocked_by(&envelope, FACTOR, TIMOSHENKO_ID, &format!("{mode:?}"));
        assert_eq!(refusal["affected_refs"], json!(["pipe:AB", "mat:A", cases[0]]), "only the base's case");
        assert!(refusal["message"].as_str().unwrap().contains(&format!("on the base material basis (no temperature selection) (load cases {}):", cases[0])));
    }
}

/// §2: a document whose cases consume both the base and a selected basis solves each case on
/// its own basis; the authored-ν warning is local to each basis and names its cases.
#[test]
fn the_base_and_a_selected_basis_are_each_consumed_by_their_own_cases() {
    let points = json!([point("point:hot", 400.0, 180e9, 70e9)]);
    let mut request = selecting((200e9, 77e9), points, &[None, Some(json!("point:hot"))]);
    request["model"]["materials"][0]["poisson_ratio"] = json!({"value": 0.3, "unit": "1"});
    let cases = case_ids(&request);
    for mode in MODES {
        let envelope = solve(&request, mode);
        solved(&envelope, "both bases");
        assert_eq!(differences(&case_values(&envelope, &cases[0]), &case_values(&plain(200e9, 77e9, mode), &cases[0])), Vec::<String>::new(), "{mode:?}: the base case");
        assert_eq!(differences(&case_values(&envelope, &cases[1]), &case_values(&plain(180e9, 70e9, mode), &cases[1])), Vec::<String>::new(), "{mode:?}: the selected case");
        let warnings = with_code(&envelope, AUTHORED_NU);
        assert_eq!(warnings.len(), 2, "one per consumed basis");
        assert_eq!(warnings[0]["affected_refs"], json!(["mat:A", cases[0]]));
        assert!(warnings[0]["message"].as_str().unwrap().contains("on the base material basis (no temperature selection)"));
        assert_eq!(warnings[1]["affected_refs"], json!(["mat:A", cases[1]]));
        assert!(warnings[1]["message"].as_str().unwrap().contains(&format!(
            "on the material basis at temperature point point:hot (load cases {}) take nu = E/(2G) - 1 = {:?}", cases[1], 180e9 / (2.0 * 70e9) - 1.0)));
    }
}

/// §2: an out-of-range selected basis blocks by name, naming its basis and only its cases; the
/// Euler–Bernoulli twin solves.
#[test]
fn an_out_of_range_selected_basis_blocks_by_name() {
    let points = json!([point("point:hot", 400.0, 210e9, 60e9)]);
    let request = selecting((200e9, 77e9), points, &[None, Some(json!("point:hot"))]);
    let cases = case_ids(&request);
    for mode in MODES {
        let envelope = solve(&request, mode);
        let refusal = blocked_by(&envelope, FACTOR, TIMOSHENKO_ID, &format!("{mode:?}"));
        assert_eq!(refusal["affected_refs"], json!(["pipe:AB", "mat:A", cases[1]]), "only the selecting case");
        assert!(refusal["message"].as_str().unwrap().contains(&format!("on the material basis at temperature point point:hot (load cases {}): nu = E/(2G) - 1 = 0.75", cases[1])));
        solved(&solve(&euler_bernoulli(&request), mode), "the Euler-Bernoulli twin");
    }
}

// ------------------------------------------------------------- arc disclosure

/// §4: a Timoshenko result with a realized arc names it once; a result without one, and an
/// Euler–Bernoulli result with one, carry no such diagnostic.
#[test]
fn realized_arcs_are_disclosed_once_and_only_on_a_timoshenko_result() {
    let arc = template(ARC_TIP);
    for mode in MODES {
        let envelope = solve(&arc, mode);
        solved(&envelope, "arc");
        let found = with_code(&envelope, ARCS);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0]["severity"], json!("info"));
        assert_eq!(found[0]["affected_refs"], json!(["pipe:BC"]));
        assert_eq!(found[0]["message"], json!("timoshenko_annulus_energy_v1: 2 straight members with shear deformation (Timoshenko); 1 realized arc without shear deformation (Euler-Bernoulli): pipe:BC."));
        let twin = solve(&euler_bernoulli(&arc), mode);
        solved(&twin, "arc twin");
        assert_eq!(twin["producer"]["semantic_contract_id"], json!(EULER_BERNOULLI_ID));
        assert!(with_code(&twin, ARCS).is_empty());
        let straight = solve(&template(CANTILEVER), mode);
        solved(&straight, "no arc");
        assert!(with_code(&straight, ARCS).is_empty());
    }
}

/// §4: forty realized arcs: every arc is referenced, the first 32 in code-point order are
/// named, the rest counted, and the message stays within 1,000 Unicode scalar values.
#[test]
fn more_than_32_arcs_are_named_up_to_the_bound_and_counted() {
    let arc = template(ARC_TIP);
    let mut request = arc.clone();
    let model = &mut request["model"];
    for key in ["nodes", "pipe_segments", "supports", "components"] {
        model[key] = json!([]);
    }
    let mut loads = Vec::new();
    // Copies 39 down to 0, so the authored order is not the code-point order.
    for copy in (0..40).rev() {
        let rename = |id: &Value| json!(format!("{}-{copy:02}", id.as_str().unwrap()));
        for node in arc["model"]["nodes"].as_array().unwrap() {
            let mut node = node.clone();
            node["id"] = rename(&node["id"]);
            node["position"]["z"] = json!(10.0 * copy as f64);
            model["nodes"].as_array_mut().unwrap().push(node);
        }
        for pipe in arc["model"]["pipe_segments"].as_array().unwrap() {
            let mut pipe = pipe.clone();
            for field in ["id", "from", "to"] {
                pipe[field] = rename(&pipe[field]);
            }
            model["pipe_segments"].as_array_mut().unwrap().push(pipe);
        }
        for support in arc["model"]["supports"].as_array().unwrap() {
            let mut support = support.clone();
            support["id"] = rename(&support["id"]);
            support["node"] = rename(&support["node"]);
            model["supports"].as_array_mut().unwrap().push(support);
        }
        for component in arc["model"]["components"].as_array().unwrap() {
            let mut component = component.clone();
            component["id"] = rename(&component["id"]);
            component["node"] = rename(&component["node"]);
            component["geometry"]["bend_pipe_ref"] = rename(&component["geometry"]["bend_pipe_ref"]);
            model["components"].as_array_mut().unwrap().push(component);
        }
        for load in arc["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap() {
            let mut load = load.clone();
            load["id"] = rename(&load["id"]);
            load["target"]["node"] = rename(&load["target"]["node"]);
            loads.push(load);
        }
    }
    let first = arc["model"]["load_cases"][0].clone();
    model["load_cases"] = json!([first]);
    model["load_cases"][0]["primitive_loads"] = json!(loads);
    let arcs: Vec<String> = (0..40).map(|copy| format!("pipe:BC-{copy:02}")).collect();
    for mode in MODES {
        let envelope = solve(&request, mode);
        solved(&envelope, "forty arcs");
        let found = with_code(&envelope, ARCS);
        assert_eq!(found.len(), 1, "once per result");
        assert_eq!(found[0]["affected_refs"], json!(arcs), "every arc, in code-point order");
        let message = found[0]["message"].as_str().unwrap();
        assert!(message.chars().count() <= 1000);
        assert_eq!(message, format!(
            "timoshenko_annulus_energy_v1: 80 straight members with shear deformation (Timoshenko); 40 realized arcs without shear deformation (Euler-Bernoulli): {} and 8 more.",
            arcs[..32].join(", ")));
    }
}

// --------------------------------------------------------- geometry-only bend

/// §1: a geometry-only bend is analysed as its mapped straight frame, so on a Timoshenko
/// document it gives the same rows as its plain straight twin (the bend component removed),
/// and it is not a realized arc.
#[test]
fn a_geometry_only_bend_gives_its_plain_straight_twins_rows() {
    let mut bend = template(ARC_TIP);
    bend["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_only");
    let mut plain = bend.clone();
    plain["model"]["components"] = json!([]);
    for request in [&bend, &plain] {
        assert_eq!(request["model"]["model_settings"]["shear_model"], json!("timoshenko_annulus_energy_v1"));
    }
    for mode in MODES {
        let (bend, plain) = (solve(&bend, mode), solve(&plain, mode));
        solved(&bend, "geometry-only bend");
        solved(&plain, "plain twin");
        assert!(with_code(&bend, ARCS).is_empty(), "not a realized arc");
        let (bend_rows, plain_rows) = (values(&bend), values(&plain));
        for (key, value) in &plain_rows {
            assert_eq!(bend_rows.get(key).map(|v| v.to_bits()), Some(value.to_bits()), "{mode:?} {key:?}");
        }
        let extra: Vec<_> = bend_rows.keys().filter(|key| !plain_rows.contains_key(*key)).collect();
        assert!(extra.iter().all(|key| key.0 == "component_equal_factor_intensified_bending_stress_v1"), "{mode:?}: only the bend's review rows are added: {extra:?}");
    }
}
