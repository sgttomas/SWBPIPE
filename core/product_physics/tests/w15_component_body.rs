//! W-15 slice d (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §5, §6
//! "Headline switch"): the reference-section identification of a component
//! body's results, through the producer and the public readers.
//!
//! The case is §2.4's indeterminate pair: two collinear members of equal E·A
//! meeting at a loaded node, each anchored at its far end, the body (a valve's
//! mapped member) twice the pipe's length. At s = 1 the pipe carries 2F/3 and
//! governs; at s = 4 the body carries 2F/3 and its reference-section stress is
//! the larger, so without slice d the headline would move onto the body. Model
//! inputs are invented; the scale is the case's user value, not catalog data.
use open_pipe_stress_product_physics::result_export::{derivative, reader_verdict, semantic_contract};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const PROV: &str = "invented W-15 slice-d test input; not library, catalog or code data";
const PIPE: &str = "pipe:w15-pipe";
const BODY: &str = "pipe:w15-body";
const VALVE: &str = "component:w15-valve";
const CASE: &str = "case:w15";
const MAXIMUM: &str = "pipe_elastic_normal_stress_maximum_v2";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const BODY_REFERENCE_SECTION: &str = "COMPONENT_BODY_REFERENCE_SECTION";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Route {
    Preview,
    Exact,
}
const ROUTES: [Route; 2] = [Route::Preview, Route::Exact];

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn valve(id: &str, node: &str, pipe: &str, s: f64) -> Value {
    json!({"id": id, "kind": "valve", "node": node, "geometry": {"rigid_pipe_ref": pipe},
        "modifiers": {"stiffness_scaling_user_value": q(s, "1"), "source_reference": "invented W-15 scale"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV})
}

/// §2.4's pair A(0) - B(1 m) - C(3 m): `PIPE` A-B and `BODY` B-C, anchored at
/// A and C, a 3 kN axial force at B. `components` are the case's valves.
fn request(route: Route, components: Vec<Value>) -> Value {
    let exact = route == Route::Exact;
    let pipe = |id: &str, from: &str, to: &str| {
        let mut section = json!({"outside_diameter": q(0.12, "m"), "wall_thickness": q(0.01, "m")});
        if exact {
            section["corrosion_allowance"] = q(0.0, "m");
        }
        json!({"id": id, "from": from, "to": to, "section": section, "material": "material:w15",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})
    };
    let node = |id: &str, x: f64| json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV});
    let anchor = |node: &str| json!({"id": format!("support:{node}"), "node": node, "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV});
    let mut case = json!({"id": CASE, "primitive_loads": [{"id": "load:w15-fx", "category": "concentrated_force",
        "target": {"type": "node", "node": "node:b"}, "direction": "global_x", "magnitude": q(3000.0, "N"),
        "dimension": "force", "provenance": PROV}], "provenance": PROV});
    let mut model = json!({"schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:w15-slice-d", "units": {"length": "m"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("node:a", 0.0), node("node:b", 1.0), node("node:c", 3.0)],
        "pipe_segments": [pipe(PIPE, "node:a", "node:b"), pipe(BODY, "node:b", "node:c")],
        "supports": [anchor("node:a"), anchor("node:c")], "components": components, "combinations": []});
    if exact {
        model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
        model["materials"] = json!([{"id": "material:w15", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": q(200e9, "Pa"), "poisson_ratio": q(0.3, "1"), "provenance": PROV}]);
        case["pressure_regions"] = json!([]);
    } else {
        model["materials"] = json!([{"id": "material:w15", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(80e9, "Pa"),
            "thermal_expansion_coefficient": q(1.2e-5, "1/degC"), "provenance": PROV}]);
    }
    model["load_cases"] = json!([case]);
    json!({"model": model, "materials": []})
}

fn solve(wire: &Value, mode: PreviewSolverMode) -> Value {
    let out = run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("the request parses");
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{:#?}", out.diagnostics);
    serde_json::to_value(&out).unwrap()
}

fn reader(route: Route) -> &'static str {
    if route == Route::Preview { "preview-physics" } else { "pressure" }
}

fn table(route: Route) -> &'static Value {
    if route == Route::Preview { semantic_contract::preview_physics_contract() } else { semantic_contract::pressure_rows_contract() }
}

fn stress_kind(route: Route, kind: &str) -> bool {
    table(route)["rows"].as_array().unwrap().iter().any(|r| r["kind"] == kind && r["source_physical_semantic_dimension"] == "stress")
}

/// The public reader's verdict, model-free and with the captured model.
fn verdicts(route: Route, source: &Value, model: &Value) -> [Value; 2] {
    [reader_verdict::validate(reader(route), source, None, None), reader_verdict::validate(reader(route), source, Some(model), None)]
}

fn admitted(route: Route, source: &Value, model: &Value, label: &str) {
    for verdict in verdicts(route, source, model) {
        assert_eq!(verdict["ok"], true, "{label}: {verdict}");
    }
}

/// Refused with `code` by the model-free reader, or (when `model_only`) only by
/// the model-bound one.
fn refused(route: Route, source: &Value, model: &Value, code: &str, model_only: bool, label: &str) {
    let [free, bound] = verdicts(route, source, model);
    if model_only {
        assert_eq!(free["ok"], true, "{label}: model-free {free}");
    } else {
        assert_eq!(free["ok"], false, "{label}: model-free admitted");
        assert!(free.to_string().contains(code), "{label}: expected {code}, got {free}");
    }
    assert_eq!(bound["ok"], false, "{label}: model-bound admitted");
    assert!(bound.to_string().contains(code), "{label}: expected {code}, got {bound}");
}

fn headlines(route: Route, source: &Value) -> Vec<&Value> {
    let summary = &source["summary"];
    match route {
        Route::Preview => vec![&summary["max_open_formula_stress"]],
        Route::Exact => vec![&summary["max_open_formula_stress_nominal_section"], &summary["max_open_formula_stress_reduced_section"]],
    }
}

fn maxima(source: &Value, pipe: &str) -> Vec<f64> {
    source["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == MAXIMUM && r["entity_ref"] == pipe)
        .map(|r| r["value"].as_f64().unwrap())
        .collect()
}

fn cases(source: &Value) -> Vec<&Value> {
    let evidence = &source["contract_evidence"];
    evidence["preview_cases"].as_array().or(evidence["exact_cases"].as_array()).unwrap().iter().collect()
}

fn coverage(case: &Value) -> Vec<&Value> {
    match &case["stress_maximum_coverage"] {
        Value::Array(entries) => entries.iter().collect(),
        entry => vec![entry],
    }
}

fn record(s: f64) -> Value {
    json!({"component_ref": VALVE, "stiffness_scale": s, "section_role": "reference_pipe_section"})
}

/// The governing switch: the headline stays on the pipe at s = 1 and s = 4;
/// every stress row, extremum and maximum of the body carries its record, and
/// nothing else does; every coverage entry lists the body; both readers, with
/// and without the model, admit the result.
#[test]
fn w15_headline_stays_on_the_pipe_while_the_body_carries_its_record() {
    for route in ROUTES {
        for s in [1.0, 4.0] {
            let wire = request(route, vec![valve(VALVE, "node:b", BODY, s)]);
            for mode in MODES {
                let label = format!("{route:?} s={s} {}", mode.as_str());
                let source = solve(&wire, mode);
                admitted(route, &source, &wire["model"], &label);
                let (pipe, body) = (maxima(&source, PIPE), maxima(&source, BODY));
                assert!(!pipe.is_empty() && pipe.len() == body.len(), "{label}");
                // §2.4: s = 1 lets the pipe govern; s = 4 makes the body's
                // reference-section stress the larger one.
                let governs = pipe.iter().zip(&body).all(|(p, b)| if s == 1.0 { p > b } else { b > p });
                assert!(governs, "{label}: pipe {pipe:?} body {body:?}");
                for headline in headlines(route, &source) {
                    assert_eq!(headline["location_ref"], PIPE, "{label}: {headline}");
                }
                let mut tagged = 0;
                for row in source["results"].as_array().unwrap() {
                    let expected = row["entity_ref"] == BODY && stress_kind(route, row["kind"].as_str().unwrap());
                    match row.get("component_body") {
                        Some(r) => {
                            assert!(expected, "{label}: forged on {}", row["id"]);
                            assert_eq!(r, &record(s), "{label}: {}", row["id"]);
                            assert_eq!(r["stiffness_scale"].as_f64().map(f64::to_bits), Some(s.to_bits()));
                            tagged += 1;
                        }
                        None => assert!(!expected, "{label}: missing on {}", row["id"]),
                    }
                }
                assert!(tagged >= body.len() * 6, "{label}: {tagged} body stress rows");
                for case in cases(&source) {
                    for entry in coverage(case) {
                        assert_eq!(entry["component_body_pipe_ids"], json!([BODY]), "{label}");
                        assert_eq!(entry["complete"], true, "{label}: completeness is unchanged");
                    }
                    for extremum in case["pipe_stress_extrema"].as_array().unwrap() {
                        if extremum["pipe_id"] == BODY {
                            assert_eq!(extremum["component_body"], record(s), "{label}");
                        } else {
                            assert!(extremum.get("component_body").is_none(), "{label}");
                        }
                    }
                }
            }
        }
    }
}

/// A rule bound to a body row is refused with `COMPONENT_BODY_REFERENCE_SECTION`
/// (`RULE_INPUTS_INCOMPLETE`); a rule bound to the pipe's row binds.
#[test]
fn w15_a_rule_bound_to_a_body_row_reports_the_reference_section() {
    for route in ROUTES {
        let source = solve(&request(route, vec![valve(VALVE, "node:b", BODY, 4.0)]), MODES[0]);
        let rows = source["results"].as_array().unwrap();
        for row in rows.iter().filter(|r| r["kind"] == MAXIMUM) {
            let expected = (row["entity_ref"] == BODY).then_some(BODY_REFERENCE_SECTION);
            assert_eq!(semantic_contract::rule_binding_refusal(&source, row), expected, "{route:?} {}", row["id"]);
        }
        // A headline binds the row its result_ref names, never a body.
        for headline in headlines(route, &source) {
            let bound = rows.iter().find(|r| r["id"] == headline["result_ref"]).unwrap();
            assert_eq!(semantic_contract::rule_binding_refusal(&source, bound), None, "{route:?}");
        }
    }
}

fn derive(source: &Value, wire: &Value) -> Result<Value, String> {
    let base: Value = serde_json::from_str(include_str!("../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
    let carrier = derivative::checksum(source, "attested_headless_producer_carrier", derivative::reference("test_carrier", "w15-slice-d"))?;
    let origin = json!({"origin_id": "w15-slice-d-export-test", "origin_class": "attested_headless_producer",
        "qualification_ref": derivative::reference("test_fixture", "not-authentication"), "authentic_producer_available": true,
        "received_carrier_checksum": carrier, "original_producer_checksum": carrier,
        "origin_limit": "In-memory producer output; consumer contract test only",
        "actual_model_ref": derivative::reference("model_payload", source["model_ref"].as_str().unwrap_or("")),
        "mechanics_run_ref": derivative::reference("mechanics_run", source["run_id"].as_str().unwrap_or("")),
        "request_model_ref": null, "request_run_ref": null, "request_alias_disclosure": null});
    derivative::derive_document(base, &wire["model"], source, origin, Some(wire))
}

/// Paths of every object in `doc` that carries a `component_body` member.
fn carriers(value: &Value, path: String, out: &mut Vec<(String, Value)>) {
    match value {
        Value::Object(o) => {
            if let Some(r) = o.get("component_body") {
                out.push((path.clone(), r.clone()));
            }
            for (k, v) in o {
                carriers(v, format!("{path}/{k}"), out);
            }
        }
        Value::Array(a) => a.iter().enumerate().for_each(|(i, v)| carriers(v, format!("{path}/{i}"), out)),
        _ => {}
    }
}

/// §5 Exports: the Rust derivative exporter copies the record verbatim onto the
/// body's quantity rows and source annotations and validates it against the
/// source row; a dropped, forged or altered copy is refused.
#[test]
fn w15_the_derivative_export_preserves_and_validates_the_record() {
    for route in ROUTES {
        let wire = request(route, vec![valve(VALVE, "node:b", BODY, 4.0)]);
        let source = solve(&wire, MODES[0]);
        let doc = derive(&source, &wire).unwrap_or_else(|e| panic!("{route:?}: {e}"));
        derivative::validate_document(&doc, &source).unwrap_or_else(|e| panic!("{route:?}: {e}"));
        let mut found = Vec::new();
        carriers(&doc, String::new(), &mut found);
        let source_rows = source["results"].as_array().unwrap().iter().filter(|r| r.get("component_body").is_some()).count();
        assert!(!found.is_empty() && found.iter().all(|(_, r)| r == &record(4.0)), "{route:?}: {found:?}");
        let quantities: Vec<&(String, Value)> = found.iter().filter(|(p, _)| p.starts_with("/result_envelope/result_sets/0/values/")).collect();
        assert!(!quantities.is_empty(), "{route:?}: the body's quantity rows carry the record");
        assert!(found.len() >= source_rows, "{route:?}: {} copies for {source_rows} source rows", found.len());
        // Tampers: drop, alter and forge a copy on a quantity row.
        let pointer = |p: &str| p.to_string();
        let first = pointer(&quantities[0].0);
        let mut dropped = doc.clone();
        dropped.pointer_mut(&first).unwrap().as_object_mut().unwrap().remove("component_body");
        assert!(derivative::validate_document(&dropped, &source).is_err(), "{route:?}: a dropped record is refused");
        let mut altered = doc.clone();
        altered.pointer_mut(&first).unwrap()["component_body"]["stiffness_scale"] = json!(5.0);
        assert!(derivative::validate_document(&altered, &source).is_err(), "{route:?}: an altered record is refused");
        let mut forged = doc.clone();
        let values = forged.pointer_mut("/result_envelope/result_sets/0/values").unwrap().as_array_mut().unwrap();
        let plain = values.iter_mut().find(|v| v.get("component_body").is_none() && v["object_ref"]["ref_id"] == PIPE)
            .unwrap_or_else(|| panic!("{route:?}: a pipe quantity row"));
        plain["component_body"] = record(4.0);
        assert!(derivative::validate_document(&forged, &source).is_err(), "{route:?}: a forged record is refused");
    }
}

fn row_mut<'a>(source: &'a mut Value, pick: impl Fn(&Value) -> bool) -> &'a mut Value {
    source["results"].as_array_mut().unwrap().iter_mut().find(|r| pick(r)).expect("a matching row")
}

/// §5 Row record: missing, forged and inconsistent records, a basis twin's
/// disagreement, a coverage disagreement, a headline on a body and a model
/// mismatch are each refused.
#[test]
fn w15_missing_forged_and_inconsistent_records_are_refused() {
    for route in ROUTES {
        let wire = request(route, vec![valve(VALVE, "node:b", BODY, 4.0)]);
        let model = &wire["model"];
        let source = solve(&wire, MODES[0]);
        let tamper = |label: &str, code: &str, model_only: bool, change: &dyn Fn(&mut Value)| {
            let mut copy = source.clone();
            change(&mut copy);
            refused(route, &copy, model, code, model_only, &format!("{route:?} {label}"));
        };
        tamper("missing on a body stress row", "SOURCE_COMPONENT_BODY_MISSING", false, &|s| {
            row_mut(s, |r| r["entity_ref"] == BODY && r["kind"] == "element_local_axial_normal_stress").as_object_mut().unwrap().remove("component_body");
        });
        tamper("forged on a pipe stress row", "SOURCE_COMPONENT_BODY_FORGED", false, &|s| {
            row_mut(s, |r| r["entity_ref"] == PIPE && r["kind"] == "element_local_axial_normal_stress")["component_body"] = record(4.0);
        });
        tamper("forged on a body force row", "SOURCE_COMPONENT_BODY_FORGED", false, &|s| {
            row_mut(s, |r| r["entity_ref"] == BODY && r["kind"] == "element_local_axial_force")["component_body"] = record(4.0);
        });
        tamper("inconsistent scale", "SOURCE_COMPONENT_BODY_INCONSISTENT", false, &|s| {
            row_mut(s, |r| r["entity_ref"] == BODY && r["kind"] == MAXIMUM)["component_body"]["stiffness_scale"] = json!(4.000000000000001);
        });
        tamper("record shape", "SOURCE_COMPONENT_BODY_RECORD_SHAPE", false, &|s| {
            for row in s["results"].as_array_mut().unwrap().iter_mut().filter(|r| r.get("component_body").is_some()) {
                row["component_body"]["section_role"] = json!("component_body_section");
            }
        });
        tamper("coverage without the body", "SOURCE_COMPONENT_BODY", false, &|s| {
            let evidence = &mut s["contract_evidence"];
            let key = if evidence.get("preview_cases").is_some() { "preview_cases" } else { "exact_cases" };
            match &mut evidence[key][0]["stress_maximum_coverage"] {
                Value::Array(entries) => {
                    entries[0].as_object_mut().unwrap().remove("component_body_pipe_ids");
                }
                entry => {
                    entry.as_object_mut().unwrap().remove("component_body_pipe_ids");
                }
            }
        });
        tamper("extremum without its record", "SOURCE_COMPONENT_BODY_MISSING", false, &|s| {
            let evidence = &mut s["contract_evidence"];
            let key = if evidence.get("preview_cases").is_some() { "preview_cases" } else { "exact_cases" };
            let extremum = evidence[key][0]["pipe_stress_extrema"].as_array_mut().unwrap().iter_mut().find(|e| e["pipe_id"] == BODY).unwrap();
            extremum.as_object_mut().unwrap().remove("component_body");
        });
        tamper("headline on the body", "HEADLINE", false, &|s| {
            let body = s["results"].as_array().unwrap().iter().find(|r| r["entity_ref"] == BODY && r["kind"] == MAXIMUM).unwrap().clone();
            let key = if route == Route::Preview { "max_open_formula_stress".to_string() } else {
                format!("max_open_formula_stress_{}", body["metadata"]["basis"].as_str().unwrap().rsplit('=').next().unwrap())
            };
            s["summary"][key] = json!({"value": body["value"], "unit": body["unit"], "location_ref": BODY, "result_ref": body["id"]});
        });
        if route == Route::Exact {
            tamper("basis twin disagreement", "SOURCE_COMPONENT_BODY_INCONSISTENT", false, &|s| {
                row_mut(s, |r| r["entity_ref"] == BODY && r["kind"] == MAXIMUM && r["metadata"]["basis"].as_str().is_some_and(|b| b.ends_with("reduced_section")))
                    ["component_body"]["component_ref"] = json!("component:w15-other");
            });
        }
        // The model-bound reader derives the bodies from the model.
        let mut other = wire.clone();
        other["model"]["components"][0]["modifiers"]["stiffness_scaling_user_value"]["value"] = json!(5.0);
        refused(route, &source, &other["model"], "SOURCE_COMPONENT_BODY_MODEL_MISMATCH", true, &format!("{route:?} model scale"));
        let mut unscaled = wire.clone();
        unscaled["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
        refused(route, &source, &unscaled["model"], "SOURCE_COMPONENT_BODY_MODEL_MISMATCH", true, &format!("{route:?} model without the body"));
        // And a result without its record against a model that scales it.
        let plain = solve(&unscaled, MODES[0]);
        let mut members = Vec::new();
        carriers(&plain, String::new(), &mut members);
        assert!(members.is_empty(), "{route:?}: an unscaled result carries no record");
        for case in cases(&plain) {
            assert!(coverage(case).iter().all(|e| e.get("component_body_pipe_ids").is_none()), "{route:?}: nor a coverage list");
        }
        admitted(route, &plain, &unscaled["model"], &format!("{route:?} unscaled"));
        refused(route, &plain, model, "SOURCE_COMPONENT_BODY_MODEL_MISMATCH", true, &format!("{route:?} unscaled result, scaled model"));
    }
}

/// §5 Maximum: a case whose every member is a body has a null headline, with
/// every member's maximum and extremum still published and coverage complete.
#[test]
fn w15_an_all_body_case_has_a_null_headline() {
    for route in ROUTES {
        let wire = request(route, vec![valve(VALVE, "node:b", BODY, 4.0), valve("component:w15-valve-a", "node:a", PIPE, 2.0)]);
        for mode in MODES {
            let label = format!("{route:?} {}", mode.as_str());
            let source = solve(&wire, mode);
            admitted(route, &source, &wire["model"], &label);
            for headline in headlines(route, &source) {
                assert!(headline.is_null(), "{label}: {headline}");
            }
            assert!(!maxima(&source, PIPE).is_empty() && !maxima(&source, BODY).is_empty(), "{label}");
            for case in cases(&source) {
                for entry in coverage(case) {
                    assert_eq!(entry["component_body_pipe_ids"], json!([PIPE, BODY]), "{label}");
                    assert_eq!(entry["complete"], true, "{label}");
                }
                assert_eq!(case["pipe_stress_extrema"].as_array().unwrap().iter().filter(|e| e.get("component_body").is_some()).count(),
                    case["pipe_stress_extrema"].as_array().unwrap().len(), "{label}");
            }
            // A headline naming a body is refused.
            let mut copy = source.clone();
            let row = source["results"].as_array().unwrap().iter().find(|r| r["kind"] == MAXIMUM).unwrap();
            let key = match route {
                Route::Preview => "max_open_formula_stress".to_string(),
                Route::Exact => format!("max_open_formula_stress_{}", row["metadata"]["basis"].as_str().unwrap().rsplit('=').next().unwrap()),
            };
            copy["summary"][key] = json!({"value": row["value"], "unit": row["unit"], "location_ref": row["entity_ref"], "result_ref": row["id"]});
            refused(route, &copy, &wire["model"], "HEADLINE", false, &format!("{label} body headline"));
        }
    }
}
