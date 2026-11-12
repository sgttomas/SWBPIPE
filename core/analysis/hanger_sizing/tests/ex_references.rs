//! Sizing on the exact pressure route (EX) against the frozen exact-route
//! reference EXS-A and EXS-B (`validation/references/hanger_sizing/ex_sizing.json`,
//! DEL-04-11 REQ-002 to REQ-005 and REQ-010). The generator reads no product code.
//!
//! Each reference model is built as a product document on EX from its physical
//! description (`model_builder::exs_request`): a load/reference-state document whose
//! cases carry the reference's ledger, temperatures, pressure regions and moved
//! non-candidate supports. Every run is compared in both solver modes:
//! - W, every force-solve position, every load, variation, span, required travel,
//!   envelope and the verification pass at 1e-9 relative, with an absolute floor;
//! - outcomes, selections, first failed tests, failure counts, failed tests and the
//!   standing exactly.
//!
//! Floors, as `derived_solves.rs` chooses its own: a position gets 1e-9 of the
//! largest expected position of its group (one solve's candidates), never below
//! 1e-12 m; a force gets 1e-9 of its candidate's expected hot-load basis W,
//! never below 1e-12 N; a ratio gets 1e-12. A closed-form zero (u_C, the
//! departure) is judged by its floor.
//!
//! A null verification in the reference (an unfitted candidate) supplies no
//! expectation, so nothing is asserted about it.
//!
//! The ledger is observed, not assumed: a force document whose added forces are
//! left out of the ledger solves to the reference's failure signature, and a force
//! document that keeps a removed candidate's support state is refused by the solve.

#[path = "common/model_builder.rs"]
mod model_builder;

use model_builder::*;
use open_pipe_stress_hanger_sizing::derived::*;
use open_pipe_stress_hanger_sizing::*;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use serde_json::{json, Value};

const REFERENCE: &str = include_str!("../../../../validation/references/hanger_sizing/ex_sizing.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive];

fn reference() -> Value {
    let document: Value = serde_json::from_str(REFERENCE).unwrap();
    assert_eq!(document["schema"], "swbpipe.hanger_sizing_reference.ex_sizing/1");
    document
}

fn num(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("number expected, got {value}"))
}

/// |a − e| ≤ 1e-9·max(|a|, |e|) + floor.
fn near(actual: f64, expected: f64, floor: f64, what: &str) {
    assert!((actual - expected).abs() <= 1e-9 * actual.abs().max(expected.abs()) + floor,
        "{what}: {actual} against {expected} (difference {})", actual - expected);
}

/// The position floor of a group of expected positions.
fn position_floor<'a>(values: impl Iterator<Item = &'a Value>) -> f64 {
    1e-9 * values.map(|v| num(v).abs()).fold(1e-3, f64::max)
}

const RATIO_FLOOR: f64 = 1e-12;

fn dof(token: &Value) -> Option<Dof> {
    match token.as_str()? {
        "UX" => Some(Dof::Ux),
        "UY" => Some(Dof::Uy),
        "UZ" => Some(Dof::Uz),
        other => panic!("unknown DOF {other}"),
    }
}

fn records(model: &Value) -> Vec<Record> {
    model["catalogue"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| Record {
            hanger_id: r["hanger_id"].as_str().unwrap().to_string(),
            hanger_type: match r["hanger_type"].as_str().unwrap() {
                "variable_spring_hanger" => HangerType::VariableSpring,
                "constant_effort_support" => HangerType::ConstantEffort,
                other => panic!("{other}"),
            },
            stiffness_n_per_m: r["stiffness_n_per_m"].as_f64(),
            stiffness_dof: dof(&r["stiffness_dof"]),
            working_load_range_n: r["working_load_range_n"].as_array().map(|p| (num(&p[0]), num(&p[1]))),
            travel_range_m: r["travel_range_m"].as_f64(),
        })
        .collect()
}

/// The product request of one reference run. The reference's `weight_case_own`
/// is the product's own-solve support state.
fn sizing_request(run: &Value, mode: PreviewSolverMode) -> SizingRequest {
    let r = &run["request"];
    assert_eq!(r["weight_support_state"], "weight_case_own");
    SizingRequest {
        weight_case: r["weight_case"].as_str().unwrap().into(),
        weight_support_state: WeightSupportState::OwnSolve,
        design_operating_case: r["design_operating_case"].as_str().unwrap().into(),
        check_operating_cases: r["further_operating_cases"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().into()).collect(),
        candidates: r["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| RequestCandidate {
                support: c["support"].as_str().unwrap().into(),
                family_policy: match c["family_policy"].as_str().unwrap() {
                    "variable" => FamilyPolicy::Variable,
                    "constant" => FamilyPolicy::Constant,
                    _ => FamilyPolicy::VariableElseConstant,
                },
            })
            .collect(),
        method: Method {
            load_design: if r["load_design"] == "hot" { LoadDesign::Hot } else { LoadDesign::Cold },
            variation_reference: if r["variation_reference"] == "operating_load" {
                VariationReference::OperatingLoad
            } else {
                VariationReference::DesignLoad
            },
            variation_limit: num(&r["variation_limit"]),
            travel_allowance_m: num(&r["travel_allowance_m"]),
            selection_rule: if r["selection_rule"] == "smallest_capacity" {
                SelectionRule::SmallestCapacity
            } else {
                SelectionRule::LeastVariation
            },
        },
        solver_mode: mode,
    }
}

fn outcome_token(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Selected => "selected",
        Outcome::SelectedByFallback => "selected_by_fallback",
        Outcome::NoRecordFits => "no_record_fits",
    }
}

fn counts(map: &Option<std::collections::BTreeMap<Test, usize>>) -> Value {
    match map {
        None => Value::Null,
        Some(map) => Value::Object(map.iter().map(|(t, n)| (t.name().to_string(), json!(n))).collect()),
    }
}

/// The reference's `first_failed_test` for one candidate, by the selection's own
/// function (`evaluate`) over the records of each family the reference tested.
fn first_failed(candidate: &Candidate, records: &[Record], method: &Method, expected: &Value) -> Value {
    Value::Object(
        expected
            .as_object()
            .unwrap()
            .keys()
            .map(|id| {
                let record = records.iter().find(|r| &r.hanger_id == id).unwrap();
                let token = evaluate(candidate, record, method).failed().map_or("fits", Test::name);
                (id.clone(), json!(token))
            })
            .collect(),
    )
}

fn reaction(envelope: &MechanicsEnvelope, support: &str) -> f64 {
    let row = envelope
        .results
        .iter()
        .find(|r| r.kind == "support_reaction_component_v2" && r.entity_ref == support && r.metadata.as_ref().is_some_and(|m| m.component == "Fz"))
        .unwrap_or_else(|| panic!("no Fz reaction at {support}"));
    assert_eq!(row.unit, "N");
    row.value
}

fn displacement(envelope: &MechanicsEnvelope, node: &str) -> f64 {
    let row = envelope.results.iter().find(|r| r.kind == "global_nodal_displacement_z" && r.entity_ref == node).unwrap();
    match row.unit.as_str() {
        "mm" => row.value / 1000.0,
        "m" => row.value,
        other => panic!("unit {other}"),
    }
}

fn solve(doc: Value, mode: PreviewSolverMode, what: &str) -> MechanicsEnvelope {
    let envelope = run_linear_static_preview_value_with_mode(doc, mode).unwrap_or_else(|e| panic!("{what}: refused: {e}"));
    let blocking: Vec<&str> = envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
    assert!(blocking.is_empty(), "{what}: {blocking:?}");
    envelope
}

/// The model's sites, in the order the reference lists its candidates.
fn sites(model: &Value) -> Vec<Site> {
    model["supports"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["candidate"] == true)
        .map(|s| Site { support: s["id"].as_str().unwrap().into(), node: s["node"].as_str().unwrap().into(), dof: Dof::Uz })
        .collect()
}

#[test]
fn the_weight_solve_reproduces_the_rigid_reactions_and_the_force_solves_their_positions() {
    for model in reference()["models"].as_array().unwrap() {
        let id = model["id"].as_str().unwrap();
        let doc = exs_request(model);
        let sites = sites(model);
        let expected = &model["expected"];
        for mode in MODES {
            let who = format!("{id} {mode:?}");
            // weight_rigid: every candidate rigid; each support's reaction.
            let rigid = vec![Role::Rigid; sites.len()];
            let envelope = solve(derived_document(&doc, "W", &sites, &rigid), mode, &who);
            let reactions = expected["weight_rigid_reactions_n"].as_object().unwrap();
            let floor = 1e-9 * reactions.values().map(|v| num(v).abs()).fold(0.0, f64::max);
            for (support, value) in reactions {
                near(reaction(&envelope, support), num(value), floor, &format!("{who} weight_rigid {support}"));
            }
            // The force solves: every candidate a force W, entered in the ledger.
            let forces: Vec<Role> = sites.iter().map(|s| Role::Force(num(&expected["hot_load_basis_n"][&s.support]))).collect();
            for (case, positions) in expected["force_solve_positions_m"].as_object().unwrap() {
                let derived = derived_document(&doc, case, &sites, &forces);
                let ledger = derived["model"]["load_cases"][0]["analysis_state"]["load_sources"].as_array().unwrap().clone();
                for added in expected["added_force_sources"][case].as_array().unwrap() {
                    let entry = json!({"source_ref": format!("load:hanger-sizing:{}", added["support"].as_str().unwrap()),
                                       "factor": added["factor"]});
                    assert!(ledger.contains(&entry), "{who} {case}: {entry} not in the ledger");
                }
                let envelope = solve(derived, mode, &format!("{who} {case}"));
                let floor = position_floor(positions.as_object().unwrap().values());
                for site in &sites {
                    near(displacement(&envelope, &site.node), num(&positions[&site.support]), floor, &format!("{who} {case} {}", site.support));
                }
                let reactions = expected["force_solve_reactions_n"][case].as_object().unwrap();
                let floor = 1e-9 * reactions.values().map(|v| num(v).abs()).fold(0.0, f64::max);
                for (support, value) in reactions {
                    near(reaction(&envelope, support), num(value), floor, &format!("{who} {case} reaction {support}"));
                }
            }
        }
    }
}

#[test]
fn every_run_reproduces_the_loads_fit_selection_and_verification() {
    for model in reference()["models"].as_array().unwrap() {
        let doc = exs_request(model);
        let records = records(model);
        for run_ref in model["runs"].as_array().unwrap() {
            for mode in MODES {
                let who = format!("{} {} {mode:?}", model["id"].as_str().unwrap(), run_ref["id"].as_str().unwrap());
                let request = sizing_request(run_ref, mode);
                let operating: Vec<String> = request.operating_cases().iter().map(|c| c.to_string()).collect();
                let run = run(&doc, &records, &request).unwrap_or_else(|r| panic!("{who}: {r:?}"));
                // REQ-011: 2n + 3 ordinary solves.
                assert_eq!(run.states.len(), 2 * operating.len() + 3, "{who}");
                let expected = &run_ref["expected"];
                let standing = match run.standing {
                    VerificationStanding::Unconditional => "unconditional",
                    VerificationStanding::Conditional => "conditional",
                };
                assert_eq!(standing, expected["verification_standing"], "{who}");
                let expected_candidates = expected["candidates"].as_array().unwrap();
                assert_eq!(run.candidates.len(), expected_candidates.len(), "{who}");
                let floor_by_case: Vec<f64> = operating
                    .iter()
                    .map(|case| position_floor(expected_candidates.iter().map(|e| &e["u_operating_m"][case])))
                    .collect();
                let cold_floor = position_floor(expected_candidates.iter().map(|e| &e["u_cold_m"]));
                for (((candidate, result), verification), e) in
                    run.candidates.iter().zip(&run.results).zip(&run.verifications).zip(expected_candidates)
                {
                    let who = format!("{who} {}", candidate.id);
                    assert_eq!(candidate.id, e["support"].as_str().unwrap());
                    assert_eq!(outcome_token(result.outcome), e["outcome"], "{who}");
                    let w = num(&e["w_n"]);
                    let force_floor = (1e-9 * w.abs()).max(1e-12);
                    near(candidate.w_n, w, force_floor, &format!("{who} W"));
                    near(candidate.positions.u_cold_m, num(&e["u_cold_m"]), cold_floor, &format!("{who} u_C"));
                    for (i, case) in operating.iter().enumerate() {
                        near(candidate.positions.u_operating_m[i], num(&e["u_operating_m"][case]), floor_by_case[i], &format!("{who} u_{case}"));
                    }
                    let span_floor = position_floor(e["span_m"].as_array().unwrap().iter());
                    near(result.span.min_m, num(&e["span_m"][0]), span_floor, &format!("{who} span min"));
                    near(result.span.max_m, num(&e["span_m"][1]), span_floor, &format!("{who} span max"));
                    near(result.required_travel_m, num(&e["required_travel_m"]), span_floor, &format!("{who} required travel"));
                    // Failure counts: the reference states the families it tested.
                    let published = json!({"variable": counts(&result.variable_failures), "constant": counts(&result.constant_failures)});
                    for (family, expected_counts) in e["failure_counts"].as_object().unwrap() {
                        assert_eq!(&published[family], expected_counts, "{who} failure counts {family}");
                    }
                    assert_eq!(first_failed(candidate, &records, &request.method, &e["first_failed_test"]), e["first_failed_test"], "{who}");
                    match &result.selected {
                        None => assert!(e["hanger_id"].is_null(), "{who}"),
                        Some(selection) => {
                            assert_eq!(records[selection.record_index()].hanger_id, e["hanger_id"].as_str().unwrap(), "{who}");
                            match selection {
                                Selected::Variable { loads, .. } => {
                                    near(loads.f_cold_n, num(&e["f_cold_n"]), force_floor, &format!("{who} F_cold"));
                                    near(loads.f_cold_condition_n, num(&e["f_cold_condition_n"]), force_floor, &format!("{who} F_C"));
                                    near(loads.envelope_n.0, num(&e["envelope_n"][0]), force_floor, &format!("{who} envelope low"));
                                    near(loads.envelope_n.1, num(&e["envelope_n"][1]), force_floor, &format!("{who} envelope high"));
                                    for (i, case) in operating.iter().enumerate() {
                                        near(loads.f_operating_n[i], num(&e["f_operating_n"][case]), force_floor, &format!("{who} F_{case}"));
                                        near(loads.variation[i], num(&e["variation"][case]), RATIO_FLOOR, &format!("{who} variation {case}"));
                                    }
                                }
                                Selected::Constant { load_n, .. } => {
                                    near(*load_n, num(&e["f_cold_n"]), force_floor, &format!("{who} constant load"));
                                    assert!(e["envelope_n"].is_null() && e["variation"].is_null(), "{who}");
                                }
                            }
                        }
                    }
                    // A null verification in the reference supplies no expectation.
                    let ev = &e["verification"];
                    if ev.is_null() {
                        continue;
                    }
                    let v = verification.as_ref().unwrap_or_else(|| panic!("{who}: no verification"));
                    let vfloor = position_floor(std::iter::once(&ev["u_cold_m"]).chain(ev["u_operating_m"].as_object().unwrap().values()));
                    near(v.positions.u_cold_m, num(&ev["u_cold_m"]), vfloor, &format!("{who} verified u_C"));
                    for (i, case) in operating.iter().enumerate() {
                        near(v.positions.u_operating_m[i], num(&ev["u_operating_m"][case]), vfloor, &format!("{who} verified u_{case}"));
                    }
                    near(v.span.min_m, num(&ev["span_m"][0]), vfloor, &format!("{who} verified span min"));
                    near(v.span.max_m, num(&ev["span_m"][1]), vfloor, &format!("{who} verified span max"));
                    near(v.required_travel_m, num(&ev["required_travel_m"]), vfloor, &format!("{who} verified required travel"));
                    assert_eq!(json!(v.t_pass), ev["t_pass"], "{who}");
                    let failed: Vec<&str> = v.failed_tests.iter().map(|t| t.name()).collect();
                    assert_eq!(json!(failed), ev["failed_tests"], "{who}");
                    match &v.variable {
                        Some(vv) => {
                            assert_eq!(ev["configuration"]["kind"], "spring_plus_force", "{who}");
                            near(vv.f_cold_condition_n, num(&ev["f_cold_condition_n"]), force_floor, &format!("{who} verified F_C"));
                            near(vv.departure_n, num(&ev["departure_n"]), force_floor, &format!("{who} departure"));
                            for (i, case) in operating.iter().enumerate() {
                                near(vv.f_operating_n[i], num(&ev["f_operating_n"][case]), force_floor, &format!("{who} verified F_{case}"));
                                let ratio = vv.variation[i].unwrap_or_else(|| panic!("{who}: verified variation {case} absent"));
                                near(ratio, num(&ev["variation"][case]), RATIO_FLOOR, &format!("{who} verified variation {case}"));
                            }
                        }
                        None => {
                            assert_eq!(ev["configuration"]["kind"], "force", "{who}");
                            assert!(ev["f_operating_n"].is_null() && ev["departure_n"].is_null(), "{who}");
                        }
                    }
                }
                // An unfitted candidate is verified as a force W, and the other candidates' verified values include it.
                for unfitted in expected["unfitted_verified_as_force_w"].as_array().unwrap() {
                    let index = run.candidates.iter().position(|c| c.id == unfitted.as_str().unwrap()).unwrap();
                    assert_eq!(run.results[index].outcome, Outcome::NoRecordFits, "{who}");
                }
            }
        }
    }
}

/// The ledger entry is observed: a force document whose added forces are left out
/// of the ledger (a test-only mutation of the derived document) solves to the
/// reference's failure signature; with the entries, to the force-solve positions.
#[test]
fn without_its_ledger_entry_the_added_force_does_not_act() {
    for model in reference()["models"].as_array().unwrap() {
        let id = model["id"].as_str().unwrap();
        let doc = exs_request(model);
        let sites = sites(model);
        let expected = &model["expected"];
        let forces: Vec<Role> = sites.iter().map(|s| Role::Force(num(&expected["hot_load_basis_n"][&s.support]))).collect();
        for (case, signature) in expected["failure_signature_positions_m"].as_object().unwrap() {
            let mut derived = derived_document(&doc, case, &sites, &forces);
            let sources = derived["model"]["load_cases"][0]["analysis_state"]["load_sources"].as_array_mut().unwrap();
            let before = sources.len();
            sources.retain(|s| !s["source_ref"].as_str().unwrap().starts_with("load:hanger-sizing:"));
            assert_eq!(before - sources.len(), sites.len(), "{id} {case}");
            let envelope = solve(derived, PreviewSolverMode::DenseScrutiny, &format!("{id} {case} without the entries"));
            let floor = position_floor(signature.as_object().unwrap().values());
            for site in &sites {
                let u = displacement(&envelope, &site.node);
                near(u, num(&signature[&site.support]), floor, &format!("{id} {case} {} without the entry", site.support));
                let with = num(&expected["force_solve_positions_m"][case][&site.support]);
                assert!((u - with).abs() > 1e-5, "{id} {case}: the failure signature must differ from the expected position");
            }
        }
    }
}

/// A force document that keeps a removed candidate's support state is refused by
/// the solve (`LOAD_STATE_SUPPORT_UNKNOWN`), so the reconciliation is required.
#[test]
fn a_kept_state_of_a_removed_candidate_is_refused_by_the_solve() {
    let model = &reference()["models"][0];
    let doc = exs_request(model);
    let sites = sites(model);
    let forces: Vec<Role> = sites.iter().map(|_| Role::Force(1000.0)).collect();
    let mut derived = derived_document(&doc, "O1", &sites, &forces);
    let states = derived["model"]["load_cases"][0]["analysis_state"]["support_states"].as_array_mut().unwrap();
    states.push(json!({"support_ref": sites[0].support, "participation": {"kind": "active_model_device"}}));
    let refused = match run_linear_static_preview_value_with_mode(derived, PreviewSolverMode::DenseScrutiny) {
        Err(message) => message,
        Ok(envelope) => envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect::<Vec<_>>().join(", "),
    };
    assert!(refused.contains("LOAD_STATE_SUPPORT_UNKNOWN"), "{refused}");
}

/// The EXS models expressed on the pressure-free route, where they can be: no
/// pressure (by the reference's own argument it changes no vertical quantity),
/// each case's stored forces at the ledger's effective factors as plain primitives,
/// each member's ΔT from the installation temperature as a thermal primitive, and
/// each moved support's motion as an imposed displacement of that support. Every
/// run gives the same selections and, at the same tolerance, the same W, positions
/// and loads.
#[test]
fn the_pressure_free_expression_of_each_model_sizes_alike() {
    for model in reference()["models"].as_array().unwrap() {
        let doc = pressure_free_expression(model);
        let records = records(model);
        for run_ref in model["runs"].as_array().unwrap() {
            let who = format!("{} {} PF", model["id"].as_str().unwrap(), run_ref["id"].as_str().unwrap());
            let request = sizing_request(run_ref, PreviewSolverMode::DenseScrutiny);
            let operating = request.operating_cases().iter().map(|c| c.to_string()).collect::<Vec<_>>();
            let run = run(&doc, &records, &request).unwrap_or_else(|r| panic!("{who}: {r:?}"));
            for ((candidate, result), e) in run.candidates.iter().zip(&run.results).zip(run_ref["expected"]["candidates"].as_array().unwrap()) {
                let who = format!("{who} {}", candidate.id);
                assert_eq!(outcome_token(result.outcome), e["outcome"], "{who}");
                let selected = result.selected.as_ref().map(|s| records[s.record_index()].hanger_id.clone());
                assert_eq!(json!(selected), e["hanger_id"], "{who}");
                near(candidate.w_n, num(&e["w_n"]), 1e-9 * num(&e["w_n"]), &format!("{who} W"));
                let floor = position_floor(e["u_operating_m"].as_object().unwrap().values());
                for (i, case) in operating.iter().enumerate() {
                    near(candidate.positions.u_operating_m[i], num(&e["u_operating_m"][case]), floor, &format!("{who} u_{case}"));
                }
            }
        }
    }
}

fn pressure_free_expression(model: &Value) -> Value {
    let section = &model["section"];
    let t_install = num(&model["installation_temperature_c"]);
    let nodes: Vec<(String, [f64; 3])> = model["nodes"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(id, p)| (id.clone(), [num(&p[0]), num(&p[1]), num(&p[2])]))
        .collect();
    let node_refs: Vec<(&str, [f64; 3])> = nodes.iter().map(|(id, p)| (id.as_str(), *p)).collect();
    let members: Vec<Member> = model["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| Member {
            id: m["id"].as_str().unwrap(),
            from: m["end_nodes"][0].as_str().unwrap(),
            to: m["end_nodes"][1].as_str().unwrap(),
            outside_diameter_m: num(&section["outside_diameter_m"]),
            wall_m: num(&section["wall_thickness_m"]),
            elastic_modulus_pa: num(&section["elastic_modulus_pa"]),
            expansion_per_k: num(&section["thermal_expansion_per_k"]),
        })
        .collect();
    // The EX document's supports, which are PF supports too.
    let supports = exs_request(model)["model"]["supports"].as_array().unwrap().clone();
    let stored = model["stored_loads"].as_array().unwrap();
    let cases: Vec<Value> = model["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|lc| {
            let case_id = lc["id"].as_str().unwrap();
            let mut loads = Vec::new();
            for include in lc["includes"].as_array().unwrap() {
                let Some(load) = stored.iter().find(|l| l["id"] == include["load"] && l["kind"] == "nodal_force") else { continue };
                loads.push(nodal_fz(load["id"].as_str().unwrap(), load["node"].as_str().unwrap(), num(&load["value_n"]) * num(&include["factor"])));
            }
            let delta_t = num(&lc["temperature_c"]) - t_install;
            if delta_t != 0.0 {
                for m in &members {
                    loads.push(thermal(&format!("load:{case_id}-t-{}", m.id), m.id, delta_t));
                }
            }
            for (i, motion) in lc["imposed_displacements"].as_array().unwrap().iter().enumerate() {
                loads.push(json!({"id": format!("load:{case_id}-g{i}"), "category": "imposed_displacement",
                    "target": {"type": "support", "support": motion["support"], "dof": motion["dof"]}, "direction": motion["dof"],
                    "magnitude": {"value": motion["value_m"], "unit": "m"}, "dimension": "displacement", "provenance": "invented_test_value"}));
            }
            case(case_id, loads)
        })
        .collect();
    request(&node_refs, &members, supports, cases)
}
