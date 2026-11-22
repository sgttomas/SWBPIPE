//! RT-E1 milestone 1 (DEL-16-06 Design `embedded-agent-host.md`, integration
//! step 5): the W9-A L frame built from the desktop's blank document by one
//! embedded-agent proposal, then solved on the product route.
//!
//! The fixture holds the agent-form operations a `swbpipe_propose_operations`
//! call carries. The host stamps the fixture's constant intent fields (the stamp
//! `embeddedAgentSession.ts` applies; its vitest binds the two) and the batch
//! goes through the one atomic batch route as `author_type: agent`. The applied
//! document is solved on PF and on EX (`3.0.0/exact_pressure_v3`,
//! Euler-Bernoulli, thermal primitives) in both solver modes, and every frozen
//! W9-A row is compared with `validation/references/w9_benchmarks/references.json`
//! under the W-9 criterion (|obs - exp| <= 1e-9 |exp|; a zero expectation uses
//! 1e-9 times its family's zero scale). The fixture's inputs are checked against
//! the reference inputs, so neither side is a copy the other cannot detect.
//!
//! The blank states shear deformation in straight members as included
//! (Timoshenko), and the reference states none, so the proposal's first step
//! sets the pressure-free selection to Euler-Bernoulli with the same operation
//! the person's control issues; activation on EX carries it into the contract.
//! Without that step PF solves as Timoshenko and misses the frozen references,
//! and EX's Euler-Bernoulli activation is refused as a conflict.
//!
//! This establishes the operation/solver link of the journey. It does not
//! establish the catalog projection, the webview stamp, a native session or a
//! model round trip; their checks are named in the Design. All data are invented.
use open_pipe_stress_operation_applier::{apply_operation_batch, canonical_json, sha256_hex, validate_operation_batch};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("../../../../fixtures/model_operations/agent_w9a_journey.json");
const REFERENCES: &str = include_str!("../../../../validation/references/w9_benchmarks/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const CASE: &str = "case:w9";
const PF_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const PF_TIMOSHENKO_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
const SHEAR_STEP: &str = "op:w9a:0";
const EX_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/pressure-3";

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("fixture parses")
}
fn reference() -> Value {
    let refs: Value = serde_json::from_str(REFERENCES).expect("references parse");
    let id = fixture()["reference"]["case"].clone();
    refs["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).expect("reference case").clone()
}
fn input_f64(text: &Value) -> f64 {
    let s = text.as_str().unwrap_or_else(|| panic!("input text {text}"));
    match s.split_once('/') {
        Some((p, q)) => p.parse::<f64>().unwrap() / q.parse::<f64>().unwrap(),
        None => s.parse().unwrap(),
    }
}
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}

/// The host stamp: the fixture's constant intent fields (the desktop stamp's
/// vitest binds `embeddedAgentSession.ts` to the same record), plus the
/// proposal's queue handle and original call reference.
fn stamped(operation: &Value, host: &Value) -> Value {
    let mut native = operation.clone();
    let object = native.as_object_mut().unwrap();
    let mut fields = host["stamp"].as_object().unwrap().clone();
    fields.insert("queue_id".into(), host["proposalHandle"].clone());
    fields.insert("source".into(), json!({"source_ref": host["requestRef"], "source_channel": "embedded_agent", "source_role": "agent"}));
    for (key, value) in fields {
        assert!(object.insert(key.clone(), value).is_none(), "agent-form operations carry no host field `{key}`");
    }
    assert_eq!(native["author_type"], "agent");
    native
}

fn batch(route: &str) -> (Value, Value) {
    batch_without(route, None)
}

/// The route's batch, leaving out the operation `omit` names.
fn batch_without(route: &str, omit: Option<&str>) -> (Value, Value) {
    let f = fixture();
    let blank = f["blank"]["model"].clone();
    let operations: Vec<Value> = f["routes"][route]["operations"].as_array().unwrap().iter()
        .filter(|o| omit.is_none_or(|id| o["operation_id"] != id))
        .map(|o| stamped(o, &f["host"])).collect();
    let claim = hash(&blank);
    (blank, json!({"batch_id": format!("agent-{}", f["host"]["proposalHandle"].as_str().unwrap()), "operations": operations, "source_model_hash": claim["value"]}))
}

fn after(route: &str, change_kind: &str, target: &str) -> Value {
    let f = fixture();
    let op = f["routes"][route]["operations"].as_array().unwrap().iter()
        .find(|o| o["change"]["change_kind"] == change_kind && o["target"]["ref"] == target)
        .unwrap_or_else(|| panic!("{route}: no {change_kind} {target}"));
    serde_json::from_str(op["change"]["after"].as_str().unwrap()).unwrap()
}

#[test]
fn fixture_inputs_are_the_w9a_reference_inputs() {
    let r = reference();
    let (g, s, t) = (&r["inputs"]["geometry"], &r["inputs"]["section"], &r["inputs"]["thermal"]);
    for route in ["PF", "EX"] {
        for (id, xyz) in g["nodes"].as_object().unwrap() {
            let node = after(route, "create_node", id);
            let xyz: Vec<f64> = xyz.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            assert_eq!([node["position"]["x"].as_f64().unwrap(), node["position"]["y"].as_f64().unwrap(), node["position"]["z"].as_f64().unwrap()], xyz[..], "{route} {id}");
        }
        for (id, pipe) in g["pipes"].as_object().unwrap() {
            let p = after(route, "connect_pipe_run", id);
            assert_eq!((&p["from"], &p["to"]), (&pipe["from"], &pipe["to"]), "{route} {id}");
            let y: Vec<f64> = pipe["y_reference"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            assert_eq!([p["y_reference"]["x"].as_f64().unwrap(), p["y_reference"]["y"].as_f64().unwrap(), p["y_reference"]["z"].as_f64().unwrap()], y[..]);
            assert_eq!(p["section"]["outside_diameter"], json!({"value": input_f64(&s["outside_diameter_m"]), "unit": "m"}));
            assert_eq!(p["section"]["wall_thickness"], json!({"value": input_f64(&s["wall_thickness_m"]), "unit": "m"}));
        }
        for (id, support) in g["supports"].as_object().unwrap() {
            let a = after(route, "create_support", id);
            assert_eq!((&a["node"], &a["restraints"], &a["family"]), (&support["node"], &support["restraints"], &json!("anchor")));
        }
        let m = after(route, "create_material", "material:w9");
        assert_eq!(m["elastic_modulus"]["value"].as_f64().unwrap(), input_f64(&s["elastic_modulus_Pa"]));
        assert_eq!(m["thermal_expansion_coefficient"]["value"].as_f64().unwrap(), input_f64(&t["alpha_per_K"]));
        // A kelvin interval and a degree-Celsius interval are the same size.
        assert_eq!(m["thermal_expansion_coefficient"]["unit"], "1/degC");
        if route == "PF" {
            assert_eq!(m["shear_modulus"]["value"].as_f64().unwrap(), input_f64(&s["shear_modulus_Pa_preview_route"]));
        } else {
            assert_eq!(m["poisson_ratio"]["value"].as_f64().unwrap(), input_f64(&s["poisson_ratio_exact_route"]));
        }
        let applied: Vec<&str> = t["applied_to"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        for pipe in &applied {
            let f = fixture();
            let load = f["routes"][route]["operations"].as_array().unwrap().iter()
                .filter(|o| o["change"]["change_kind"] == "create_primitive_load")
                .map(|o| serde_json::from_str::<Value>(o["change"]["after"].as_str().unwrap()).unwrap())
                .find(|l| l["target"]["pipe"] == *pipe).unwrap_or_else(|| panic!("{route}: no thermal load on {pipe}"));
            assert_eq!((&load["category"], &load["magnitude"]["unit"]), (&json!("thermal"), &json!("degC")));
            assert_eq!(load["magnitude"]["value"].as_f64().unwrap(), input_f64(&t["delta_T_K"]));
        }
    }
}

/// The agent's proposal passes preview and applies as one atomic batch from
/// the blank document; the stored operations retain agent attribution.
fn applied(route: &str) -> Value {
    let (blank, batch) = batch(route);
    let claim = hash(&blank);
    let preview = validate_operation_batch(&blank, &batch, Some(&claim));
    assert_eq!(preview["validation"]["batch_validation_status"], "passed", "{route}: {preview:#}");
    assert!(preview["applied_model"].is_null(), "{route}: validation never publishes a document");
    let outcome = apply_operation_batch(&blank, &batch, Some(&claim));
    assert_eq!(outcome["validation"]["batch_validation_status"], "passed", "{route}: {outcome:#}");
    let model = outcome["applied_model"].clone();
    assert!(model.is_object(), "{route}: applied document");
    model
}

#[test]
fn a_proposal_on_a_stale_basis_applies_nothing() {
    let (blank, batch) = batch("PF");
    let mut stale = blank.clone();
    stale["project"]["name"] = json!("changed after the agent read the basis");
    let outcome = apply_operation_batch(&stale, &batch, Some(&hash(&blank)));
    assert_eq!(outcome["validation"]["batch_validation_status"], "blocked");
    assert!(outcome["applied_model"].is_null());
}

fn row(out: &MechanicsEnvelope, kind: &str, entity: &str, component: &str, location: Option<&str>) -> (f64, String) {
    let found: Vec<_> = out.results.iter().filter(|r| {
        r.kind == kind && r.entity_ref == entity
            && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == CASE)
            && r.metadata.as_ref().is_some_and(|m| m.component == component && location.is_none_or(|l| m.location == l))
    }).collect();
    assert_eq!(found.len(), 1, "one row {kind} {entity} {component} {location:?}");
    (found[0].value, found[0].unit.clone())
}

/// The frozen rows `out` misses under the W-9 criterion, and how many were compared.
fn misses(out: &MechanicsEnvelope, reference: &Value, label: &str) -> (usize, Vec<String>) {
    let scales = &reference["zero_scales"];
    let mut count = 0;
    let mut missed = Vec::new();
    for r in reference["rows"].as_array().unwrap() {
        let (entity, component, unit) = (r["entity"].as_str().unwrap(), r["component"].as_str().unwrap(), r["unit"].as_str().unwrap());
        let expected = r["value"].as_f64().unwrap();
        let family = match unit { "N" => "force", "N*m" => "moment", "m" => "translation", "rad" => "rotation", other => panic!("unit {other}") };
        let zero_scale = scales[family]["value"].as_f64().unwrap();
        let (observed, what) = match r["quantity"].as_str().unwrap() {
            "support_reaction" => {
                let (v, u) = row(out, "support_reaction_component_v2", entity, component, None);
                assert_eq!(u, unit);
                (v, format!("{label} {entity} {component}"))
            }
            "nodal_motion" => {
                let axis = &component[1..];
                if component.starts_with('u') {
                    let (v, u) = row(out, &format!("global_nodal_displacement_{axis}"), entity, &format!("nodal_displacement_{axis}"), None);
                    assert_eq!(u, "mm");
                    (v / 1000.0, format!("{label} {entity} {component}"))
                } else {
                    let (v, u) = row(out, &format!("global_nodal_rotation_{axis}"), entity, &format!("nodal_rotation_{axis}"), None);
                    assert_eq!(u, unit);
                    (v, format!("{label} {entity} {component}"))
                }
            }
            "section_action" => {
                let (location, sign) = match r["station"].as_str().unwrap() {
                    "0" => ("end_i", -1.0), "1/4" => ("quarter_1", 1.0), "1/2" => ("midspan", 1.0), "3/4" => ("quarter_3", 1.0), "1" => ("end_j", 1.0),
                    s => panic!("station {s}"),
                };
                let (v, u) = row(out, &format!("element_local_{component}"), entity, component, Some(location));
                assert_eq!(u, unit);
                (sign * v, format!("{label} {entity} {component} {location}"))
            }
            q => panic!("quantity {q}"),
        };
        let tolerance = RELATIVE * if expected == 0.0 { zero_scale.abs() } else { expected.abs() };
        if !(observed.is_finite() && (observed - expected).abs() <= tolerance) {
            missed.push(format!("{what}: observed {observed:.17e} expected {expected:.17e}"));
        }
        count += 1;
    }
    (count, missed)
}

fn compare(out: &MechanicsEnvelope, reference: &Value, label: &str) -> usize {
    let (count, missed) = misses(out, reference, label);
    assert!(missed.is_empty(), "{missed:#?}");
    count
}

fn solve(model: &Value, mode: PreviewSolverMode, label: &str) -> MechanicsEnvelope {
    run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), mode)
        .unwrap_or_else(|e| panic!("{label}: typed boundary rejected the applied document: {e}"))
}

fn journey(route: &str, semantics: &str) {
    let model = applied(route);
    let reference = reference();
    for mode in MODES {
        let label = format!("W9-A agent journey {route} {}", mode.as_str());
        let out = solve(&model, mode, &label);
        let blocking: Vec<String> = out.diagnostics.iter().filter(|d| d.severity == "blocking" || d.severity == "failure").map(|d| format!("{}: {}", d.code, d.message)).collect();
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: {blocking:?}");
        assert!(blocking.is_empty(), "{label}: {blocking:?}");
        assert_eq!(out.producer.semantic_contract_id, semantics, "{label}");
        let rows = compare(&out, &reference, &label);
        assert_eq!(rows, reference["rows"].as_array().unwrap().len());
        println!("{label}: {rows} frozen rows within the W-9 criterion");
    }
}

#[test]
fn w9a_built_from_blank_by_one_agent_proposal_solves_on_pf() {
    journey("PF", PF_SEMANTICS);
}

#[test]
fn w9a_built_from_blank_by_one_agent_proposal_solves_on_ex() {
    journey("EX", EX_SEMANTICS);
}

/// The proposal states the reference's beam theory: the blank states
/// Timoshenko, the proposal's first step sets Euler-Bernoulli, and the
/// reference states no shear deformation.
#[test]
fn the_proposal_sets_the_reference_beam_theory_first() {
    let f = fixture();
    assert_eq!(f["blank"]["model"]["model_settings"], json!({"shear_model": "timoshenko_annulus_energy_v1"}));
    assert!(serde_json::from_str::<Value>(REFERENCES).unwrap()["theory"].as_str().unwrap().contains("Euler-Bernoulli space frame, no shear deformation"));
    for route in ["PF", "EX"] {
        let first = &f["routes"][route]["operations"][0];
        assert_eq!(first["operation_id"], SHEAR_STEP, "{route}");
        assert_eq!((&first["change"]["change_kind"], &first["change"]["field_path"], &first["change"]["after"]), (&json!("set_model_setting"), &json!("model_settings.shear_model"), &json!("euler_bernoulli")), "{route}");
    }
    assert_eq!(applied("PF")["model_settings"], json!({"shear_model": "euler_bernoulli"}));
    // Activation carried the selection into the contract and removed it.
    let exact = applied("EX");
    assert!(exact.get("model_settings").is_none(), "{:#}", exact["model_settings"]);
    assert_eq!(exact["pressure_contract"]["shear_model"], "euler_bernoulli");
}

/// Without the step the same journey is not the reference's: PF solves as
/// Timoshenko and misses frozen rows, and EX's Euler-Bernoulli activation
/// conflicts with the blank's Timoshenko selection and applies nothing.
#[test]
fn without_the_euler_bernoulli_step_the_journey_is_timoshenko_and_misses_the_references() {
    let (blank, batch) = batch_without("PF", Some(SHEAR_STEP));
    let claim = hash(&blank);
    let outcome = apply_operation_batch(&blank, &batch, Some(&claim));
    assert_eq!(outcome["validation"]["batch_validation_status"], "passed", "{outcome:#}");
    let model = outcome["applied_model"].clone();
    let reference = reference();
    for mode in MODES {
        let label = format!("W9-A agent journey PF without the step {}", mode.as_str());
        let out = solve(&model, mode, &label);
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}");
        assert_eq!(out.producer.semantic_contract_id, PF_TIMOSHENKO_SEMANTICS, "{label}");
        let (rows, missed) = misses(&out, &reference, &label);
        assert_eq!(rows, reference["rows"].as_array().unwrap().len());
        assert!(!missed.is_empty(), "{label}: Timoshenko met every frozen Euler-Bernoulli row");
        println!("{label}: {} of {rows} frozen rows missed", missed.len());
    }
    let (blank, batch) = batch_without("EX", Some(SHEAR_STEP));
    let refused = apply_operation_batch(&blank, &batch, Some(&hash(&blank)));
    assert!(refused["applied_model"].is_null(), "{refused:#}");
    let codes: Vec<&Value> = refused["operation_outcomes"].as_array().unwrap().iter()
        .flat_map(|step| step["diagnostics"].as_array().unwrap().iter().map(|d| &d["code"])).collect();
    assert!(codes.contains(&&json!("OP-PRESSURE-PROFILE-SHEAR-MODEL-CONFLICT")), "{codes:?}");
}
