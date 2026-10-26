//! B4/D1: the maintained preview controls of the W1 source census through the actual W1
//! private driver. Since B4/D1 (b) the preview route selects no source blocks; the census's
//! baseline is the recorded selection (`FORMERLY_SELECTED`) and it is kept as W1's coverage check.
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;
use open_pipe_stress_result_export::{retained_precision as reader, semantic_contract as sc};

/// The census reads only the D1 domain, which never reads the caller's limit L.
const DOMAIN_ONLY: RetainedByteLimit = RetainedByteLimit::new(0);

struct Control {
    name: String,
    recipe: &'static str,
    raw: Value,
    modes: Vec<PreviewSolverMode>,
}
fn control(name: &str, recipe: &'static str, raw: Value) -> Control {
    Control {
        name: name.into(),
        recipe,
        raw,
        modes: MODES.to_vec(),
    }
}
const INVENTED: &str = "invented_t3_s11g_test_input_not_library_data";
const S11F_INVENTED: &str = "invented_t3_s11f_test_input_not_library_data";
const PROV: &str = "invented_t3_f1b_w2_runtime_input_no_library_data";
const ALL: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const RV17_COEX: &str = "invented_rv17_f1b_review_input_no_library_data";
fn pow2(exponent: i32) -> f64 {
    f64::from_bits(((exponent + 1023) as u64) << 52)
}

fn s11g_nodal(id: &str, node: &str, direction: &str, value: f64) -> Value {
    let moment = direction.starts_with('R');
    json!({"id": id, "category": if moment { "concentrated_moment" } else { "concentrated_force" },
        "target": {"type": "node", "node": node}, "direction": direction,
        "magnitude": {"value": value, "unit": if moment { "N*m" } else { "N" }},
        "dimension": if moment { "moment" } else { "force" }, "provenance": INVENTED})
}

fn s11g_uniform(id: &str, pipe: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": pipe},
        "direction": direction, "magnitude": {"value": value, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": INVENTED})
}

fn s11g_request_of(model: Value) -> Value {
    json!({"model": model, "materials": []})
}

fn n05_tip_noise(case_id: &str) -> Value {
    let w = 1e8_f64;
    json!({"id": case_id, "label": case_id, "kind": "primitive_user_load", "provenance": INVENTED,
    "primitive_loads": [
        s11g_uniform(&format!("{case_id}:w"), "pipe", "global_y", w),
        s11g_nodal(&format!("{case_id}:uy"), "tip", "global_y", -w),
        s11g_nodal(&format!("{case_id}:rz"), "tip", "RZ", w / 3.0 + 0.2),
    ]})
}

fn n05_request(cases: Vec<Value>) -> Value {
    let mut model: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
    ))
    .unwrap();
    model["load_cases"] = json!(cases);
    s11g_request_of(model)
}

fn n05_torques() -> Value {
    json!({"id": "case", "label": "torques", "kind": "primitive_user_load", "provenance": INVENTED,
        "primitive_loads": [s11g_nodal("tip:0", "tip", "RX", 1e8), s11g_nodal("tip:1", "tip", "RX", 0.3), s11g_nodal("tip:2", "tip", "RX", -1e8)]})
}

fn with_basis(mut request: Value, case: &str, point: &str, e: f64, g: f64) -> Value {
    let material = &mut request["model"]["materials"][0];
    if material.get("temperature_points").is_none() {
        material["temperature_points"] = json!([]);
    }
    material["temperature_points"]
        .as_array_mut()
        .unwrap()
        .push(json!({
        "id": point, "temperature": {"value": 20.0, "unit": "degC"},
        "elastic_modulus": {"value": e, "unit": "Pa"}, "shear_modulus": {"value": g, "unit": "Pa"},
        "provenance": INVENTED}));
    for load_case in request["model"]["load_cases"].as_array_mut().unwrap() {
        if load_case["id"] == json!(case) {
            load_case["modulus_basis_ref"] = json!(point);
        }
    }
    request
}

fn c1_case_b(force: f64, moment: f64) -> Value {
    json!({"id": "case-b", "label": "case-b", "kind": "primitive_user_load", "provenance": INVENTED,
        "primitive_loads": [s11g_nodal("b:f", "tip", "global_y", force), s11g_nodal("b:m", "tip", "RZ", moment)]})
}

fn c1_request(force: f64, moment: f64) -> Value {
    with_basis(
        n05_request(vec![n05_torques(), c1_case_b(force, moment)]),
        "case-b",
        "point:soft",
        1.0,
        0.4,
    )
}

fn s11f_nodal(id: &str, node: &str, direction: &str, value: f64) -> Value {
    let moment = direction.starts_with('R');
    json!({"id": id, "category": if moment { "concentrated_moment" } else { "concentrated_force" },
        "target": {"type": "node", "node": node}, "direction": direction,
        "magnitude": {"value": value, "unit": if moment { "N*m" } else { "N" }},
        "dimension": if moment { "moment" } else { "force" }, "provenance": S11F_INVENTED})
}

fn s11f_uniform(id: &str, pipe: &str, direction: &str, value: f64) -> Value {
    json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": pipe},
        "direction": direction, "magnitude": {"value": value, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": S11F_INVENTED})
}

fn n05_with_tip_torques(torques: &[f64]) -> Value {
    let mut model: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
    ))
    .unwrap();
    model["load_cases"][0]["primitive_loads"] = json!(torques
        .iter()
        .enumerate()
        .map(|(i, &t)| s11f_nodal(&format!("tip:{i}"), "tip", "RX", t))
        .collect::<Vec<_>>());
    s11g_request_of(model)
}

fn f1b_nodal(id: &str, node: &str, dof: &str, value: f64) -> Value {
    if dof.starts_with('R') {
        json!({"id": id, "category": "concentrated_moment", "target": {"type": "node", "node": node},
               "direction": dof, "magnitude": {"value": value, "unit": "N*m"}, "dimension": "moment", "provenance": PROV})
    } else {
        json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node},
               "direction": format!("global_{}", dof[1..].to_lowercase()),
               "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": PROV})
    }
}

fn f1b_anchor(node: &str, restraints: &[&str]) -> Value {
    json!({"id": format!("anchor:{node}"), "node": node, "family": "anchor", "restraints": restraints,
           "provenance": PROV})
}

fn f1b_frame(
    id: &str,
    nodes: &[(&str, [f64; 3])],
    members: &[(&str, &str, &str)],
    (od, wall): (f64, f64),
    (e, g): (f64, f64),
    supports: Vec<Value>,
    loads: Vec<Value>,
) -> Value {
    json!({"model": {
        "schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:t3-f1b-w2:{id}"),
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": nodes.iter().map(|(n, p)| json!({"id": n, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})).collect::<Vec<_>>(),
        "pipe_segments": members.iter().map(|(m, a, b)| json!({"id": m, "from": a, "to": b, "material": "mat:W2",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": od, "unit": "m"}, "wall_thickness": {"value": wall, "unit": "m"}},
            "provenance": PROV})).collect::<Vec<_>>(),
        "materials": [{"id": "mat:W2", "elastic_modulus": {"value": e, "unit": "Pa"},
                       "shear_modulus": {"value": g, "unit": "Pa"}, "provenance": PROV}],
        "supports": supports,
        "load_cases": [{"id": "case", "label": id, "kind": "primitive_user_load",
                        "primitive_loads": loads, "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

fn mixed_invocation() -> Value {
    let mut request = f1b_frame(
        "mixed",
        &[("root", [0.0; 3]), ("tip", [2.0, 0.0, 0.0])],
        &[("pipe", "root", "tip")],
        (0.2, 0.01),
        (2.0e11, 8.0e10),
        vec![
            f1b_anchor("root", &["UX", "UY", "UZ", "RY", "RZ"]),
            json!({"id": "soft", "node": "root", "family": "spring", "restraints": ["RX"],
                   "stiffness": {"dof": "RX", "value": {"value": 0.0001, "unit": "N*m/rad"}}, "provenance": PROV}),
        ],
        vec![],
    );
    let model = &mut request["model"];
    model["load_cases"] = json!([
        {"id": "case:A", "label": "A", "kind": "primitive_user_load",
         "primitive_loads": [f1b_nodal("torque:a", "tip", "RX", 1e-8)], "provenance": PROV},
        {"id": "case:B", "label": "B", "kind": "primitive_user_load",
         "modulus_basis_ref": "temperature-point:tiny",
         "primitive_loads": [f1b_nodal("torque:b", "tip", "RX", 1e-8)], "provenance": PROV}
    ]);
    model["materials"][0]["temperature_points"] = json!([{"id": "temperature-point:tiny",
        "elastic_modulus": {"value": 2.0e11 * pow2(-1000) * pow2(-50), "unit": "Pa"},
        "shear_modulus": {"value": 8.0e10, "unit": "Pa"}, "provenance": PROV}]);
    request
}

fn rv17_member(
    project: &str,
    material: &str,
    prov: &str,
    length: f64,
    (od, wall): (f64, f64),
    (e, g): (f64, f64),
    supports: Vec<Value>,
    loads: Vec<Value>,
) -> Value {
    let label = project.rsplit(':').next().unwrap();
    json!({"model": {
        "schema_version": "0.7.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": project,
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": [{"id": "N0", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": prov},
                  {"id": "N1", "position": {"x": length, "y": 0.0, "z": 0.0}, "provenance": prov}],
        "pipe_segments": [{"id": "M1", "from": "N0", "to": "N1", "material": material,
                           "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                           "section": {"outside_diameter": {"value": od, "unit": "m"},
                                       "wall_thickness": {"value": wall, "unit": "m"}},
                           "provenance": prov}],
        "materials": [{"id": material, "elastic_modulus": {"value": e, "unit": "Pa"},
                       "shear_modulus": {"value": g, "unit": "Pa"}, "provenance": prov}],
        "supports": supports,
        "load_cases": [{"id": "case", "label": label, "kind": "primitive_user_load",
                        "primitive_loads": loads, "provenance": prov}],
        "combinations": []}, "materials": []})
}

fn rv17_anchor(node: &str, restraints: &[&str], prov: &str) -> Value {
    json!({"id": format!("anchor:{node}"), "node": node, "family": "anchor", "restraints": restraints,
           "provenance": prov})
}

fn rv17_force(id: &str, dof: &str, value: f64, prov: &str) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": "N1"},
           "direction": format!("global_{}", dof[1..].to_lowercase()),
           "magnitude": {"value": value, "unit": "N"}, "dimension": "force", "provenance": prov})
}

fn coexistence_cx_f() -> Value {
    let free_ux: Vec<&str> = ALL.iter().copied().filter(|d| *d != "UX").collect();
    rv17_member(
        "invented:rv17-coex:CX-F",
        "mat:C",
        RV17_COEX,
        1.0,
        (0.02, 0.002),
        (579465463.915025, 222871332.2750096),
        vec![
            rv17_anchor("N0", &ALL, RV17_COEX),
            rv17_anchor("N1", &free_ux, RV17_COEX),
        ],
        vec![rv17_force("load:0", "UX", 3.0 * pow2(-1016), RV17_COEX)],
    )
}

fn source_budget_request() -> serde_json::Value {
    let mut model: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
    ))
    .unwrap();
    let mut second = model["load_cases"][0].clone();
    second["id"] = "case:signed-companion".into();
    second["primitive_loads"][0]["id"] = "torque:signed-companion".into();
    second["primitive_loads"][0]["magnitude"]["value"] = (-2e-8).into();
    // The existing summary is normal stress, not an equivalent torsional stress.
    // Add an independent axial action so the second case truly governs it.
    second["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({
            "id":"force:axial-companion","category":"concentrated_force","dimension":"force",
            "direction":"global_x","magnitude":{"value":1000,"unit":"N"},
            "target":{"type":"node","node":"tip"},
            "provenance":"invented_source_recovery_multicase_control_no_external_project_data"
        }));
    model["load_cases"].as_array_mut().unwrap().push(second);
    serde_json::json!({"model":model,"materials":[]})
}

/// The rejected_stress_range requests are a pinned historical capture (0.1.0, bound by their
/// CAPTURE/ORACLE records); B7a retired 0.1.0, so the census re-authors them to the current
/// version, changing nothing else.
fn current_rejected_stress_range(captured: &str) -> Value {
    let mut request: Value = serde_json::from_str(captured).unwrap();
    assert_eq!(request["model"]["schema_version"], serde_json::json!("0.1.0"), "the historical capture");
    request["model"]["schema_version"] = serde_json::json!(crate::MODEL_SCHEMA_VERSION);
    request
}

fn controls() -> Vec<Control> {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/product_preview/w1_controls");
    let mut cases = Vec::new();
    for folder in ["", "ui/"] {
        for name in ["n05", "n06", "multicase"] {
            for mode in MODES {
                let file = format!("{folder}{name}-{}.request.json", mode.as_str());
                cases.push(Control {
                    name: format!("committed/{file}"),
                    recipe: "fixtures/product_preview/w1_controls/*.request.json",
                    raw: serde_json::from_slice(&std::fs::read(base.join(&file)).unwrap()).unwrap(),
                    modes: vec![mode],
                });
            }
        }
    }
    for mode in MODES {
        let file = format!("rejected_stress_range/{}.request.json", mode.as_str());
        cases.push(Control { name: format!("committed/{file}"), recipe: "tests/source_block_recovery.rs::derived_stress_unit_range_cannot_mint_source_qualification", raw: current_rejected_stress_range(&std::fs::read_to_string(base.join(&file)).unwrap()), modes: vec![mode] });
    }
    let original: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
    ))
    .unwrap();
    let references: Value = serde_json::from_str(include_str!(
        "../../../validation/benchmarks/numerical_integrity/fixtures.json"
    ))
    .unwrap();
    for name in ["N05", "N06"] {
        for sign in [1.0, -1.0] {
            for rotated in [false, true] {
                let mut model = original.clone();
                let scalar = |key: &str| {
                    references["N"][name][key]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap()
                };
                model["supports"][1]["stiffness"]["value"]["value"] =
                    json!(scalar("spring_RX_Nm_per_rad"));
                model["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] =
                    json!(sign * scalar("tip_MX_Nm"));
                model["project"]["id"] =
                    json!(format!("invented:source-recovery:{name}:{sign}:{rotated}"));
                model["nodes"][0]["id"] = json!("independent-root");
                model["nodes"][1]["id"] = json!("independent-tip");
                model["pipe_segments"][0]["from"] = json!("independent-root");
                model["pipe_segments"][0]["to"] = json!("independent-tip");
                model["pipe_segments"][0]["id"] = json!("independent-member");
                model["supports"][0]["node"] = json!("independent-root");
                model["supports"][1]["node"] = json!("independent-root");
                model["supports"][1]["id"] = json!("independent-spring");
                model["load_cases"][0]["primitive_loads"][0]["target"]["node"] =
                    json!("independent-tip");
                if rotated {
                    model["nodes"][1]["position"] = json!({"x":0,"y":2,"z":0});
                    model["pipe_segments"][0]["y_reference"] = json!({"x":0,"y":0,"z":1});
                    model["supports"][0]["restraints"] = json!(["UY", "UZ", "UX", "RZ", "RX"]);
                    model["supports"][1]["restraints"] = json!(["RY"]);
                    model["supports"][1]["stiffness"]["dof"] = json!("RY");
                    model["load_cases"][0]["primitive_loads"][0]["direction"] = json!("RY");
                }
                cases.push(control(&format!("sign_rotation/{name}/{sign}/{rotated}"), "tests/source_block_recovery.rs::retained_source_rotation_member_torque_and_device_action_reach_public_rows", json!({"model":model,"materials":[]})));
            }
        }
    }
    for force in [1e-150_f64, 1e160_f64] {
        let mut model = original.clone();
        model["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({"id":"invented:range-axial-force","category":"concentrated_force","dimension":"force","direction":"global_x","magnitude":{"value":force,"unit":"N"},"target":{"type":"node","node":"tip"},"provenance":"invented_derived_range_control_no_external_project_data"}));
        cases.push(control(&format!("tiny_range/{force:e}"), "tests/source_block_recovery.rs::actual_source_recovery_scaled_norms_admit_tiny_actions_and_keep_large_json_boundary", json!({"model":model,"materials":[]})));
    }
    cases.push(control("F4/cancelling", "src/s11f_tests.rs::f4_retained_source_selects_the_cancelling_case_and_matches_the_exact_net", n05_with_tip_torques(&[1e8,0.3,-1e8])));
    cases.push(control("F4/net", "src/s11f_tests.rs::f4_retained_source_selects_the_cancelling_case_and_matches_the_exact_net", n05_with_tip_torques(&[0.3])));
    let mut f6 = n05_with_tip_torques(&[1e8, 0.3, -1e8]);
    let mut b = f6["model"]["load_cases"][0].clone();
    b["id"] = json!("case-b");
    for load in b["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("b:{}", load["id"].as_str().unwrap()));
    }
    b["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(s11f_uniform("udl:b", "pipe", "global_y", 10.0));
    f6["model"]["load_cases"].as_array_mut().unwrap().push(b);
    cases.push(control(
        "F6/partial",
        "src/s11f_tests.rs::f6_multi_case_pre_0_4_invocation_never_errs_on_cancelling_loads",
        f6,
    ));
    cases.push(control(
        "source_budget/multicase",
        "src/source_budget_tests.rs::request",
        source_budget_request(),
    ));
    cases.push(control(
        "N05/two_case",
        "src/retained_facade_tests.rs::n05_two_cases",
        n05_two_cases(),
    ));
    cases.push(control(
        "F1b/mixed",
        "tests/f1b_w2_runtime.rs::mixed_invocation",
        mixed_invocation(),
    ));
    cases.push(control(
        "CX-F",
        "tests/f1b_w2_runtime.rs::coexistence_cx_f",
        coexistence_cx_f(),
    ));
    cases.push(control(
        "T18",
        "src/s11g_tests.rs::t18_path2_invocation_is_not_refused",
        n05_request(vec![n05_torques(), n05_tip_noise("case-b")]),
    ));
    // T19's construction before K-D5 (G = 0.4 Pa), kept as a control.
    cases.push(control(
        "T19/pre-K-D5",
        "src/s11g_tests.rs::t19_path1_load_row_variant_is_not_refused (before K-D5)",
        with_basis(
            n05_request(vec![n05_torques(), n05_tip_noise("case-b")]),
            "case-b",
            "point:soft",
            1.0,
            0.4,
        ),
    ));
    // K-D5's W1 routing (HELP_HUMAN, 2026-10-12, agreed on T3's behalf): these cases now demote
    // at the solve, and the captured entry attempts retained-source recovery on them. The
    // recipes are the tests' own requests.
    cases.push(control(
        "T19",
        "src/s11g_tests.rs::t19_request",
        crate::s11g_tests::t19_request(),
    ));
    cases.push(control(
        "T1/F1a-UDL-W1e8",
        "src/s11g_tests.rs::t1_udl_w1e8_is_demoted_on_both_entries_and_modes; src/f1a_tests.rs::f1a_non_demoted_cases_carry_no_d5_line",
        crate::s11g_tests::rf_request("RF-CANCEL-UDL-W1e8"),
    ));
    cases.push(control(
        "T2/UDL-W1e80",
        "src/s11g_tests.rs::t2_udl_w1e80_is_demoted_on_the_typed_entry",
        crate::s11g_tests::rf_request("RF-CANCEL-UDL-W1e80"),
    ));
    for delta_t in [69.806, 0.69806] {
        cases.push(control(
            &format!("T6b/{delta_t}"),
            "src/s11g_tests.rs::t6b_floor_never_hides_a_net_defect",
            crate::s11g_tests::db1(delta_t),
        ));
    }
    cases.push(control(
        "F1a/UDL-W1e8-soft-branch",
        "src/f1a_tests.rs::udl_w1e8_with_soft_branch",
        crate::f1a_tests::udl_w1e8_with_soft_branch(),
    ));
    cases.push(control(
        "T20/refused",
        "src/s11g_tests.rs::t20_characterization_rb_prime_residual_c1",
        c1_request(1.0, 1e-7),
    ));
    cases.push(control(
        "T20/control",
        "src/s11g_tests.rs::t20_characterization_rb_prime_residual_c1",
        c1_request(1.0, 0.5),
    ));
    cases
}

/// The B4/D1 census's recorded baseline (run on the B7a head 2f741183b0, before the
/// retirement): every (control, mode) trial whose legacy source-block route selected cases,
/// those cases' basis ids, and the trial's recorded numerical-use standing. 54 trials; the
/// census's other 14 trials selected nothing or were refused. The four group-3 trials'
/// rows (F6/partial, T18, T19/pre-K-D5, T19) record W1's outcome after the coverage
/// repair (`w1-coverage-repair.md` §1.4): their `case-b`, formerly unavailable with an
/// exhausted loaded-maximum search, is selected and the trial is numerically eligible.
/// So 70 cases: the legacy route's 62 and those eight.
const FORMERLY_SELECTED: [(&str, &str, &[&str], &str); 54] = [
    ("committed/n05-sparse_interactive.request.json", "sparse_interactive", &["case"], "numerically_eligible"),
    ("committed/n05-dense_scrutiny.request.json", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("committed/n06-sparse_interactive.request.json", "sparse_interactive", &["case"], "numerically_eligible"),
    ("committed/n06-dense_scrutiny.request.json", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("committed/multicase-sparse_interactive.request.json", "sparse_interactive", &["case", "case:signed-companion"], "numerically_eligible"),
    ("committed/multicase-dense_scrutiny.request.json", "dense_scrutiny", &["case", "case:signed-companion"], "numerically_eligible"),
    ("committed/ui/n05-sparse_interactive.request.json", "sparse_interactive", &["case"], "numerically_eligible"),
    ("committed/ui/n05-dense_scrutiny.request.json", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("committed/ui/n06-sparse_interactive.request.json", "sparse_interactive", &["case"], "numerically_eligible"),
    ("committed/ui/n06-dense_scrutiny.request.json", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("committed/ui/multicase-sparse_interactive.request.json", "sparse_interactive", &["case", "case:signed-companion"], "numerically_eligible"),
    ("committed/ui/multicase-dense_scrutiny.request.json", "dense_scrutiny", &["case", "case:signed-companion"], "numerically_eligible"),
    ("sign_rotation/N05/1/false", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N05/1/false", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("sign_rotation/N05/1/true", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N05/1/true", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("sign_rotation/N05/-1/false", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N05/-1/false", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("sign_rotation/N05/-1/true", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N05/-1/true", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/1/false", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/1/false", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/1/true", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/1/true", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/-1/false", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/-1/false", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/-1/true", "sparse_interactive", &["case"], "numerically_eligible"),
    ("sign_rotation/N06/-1/true", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("tiny_range/1e-150", "sparse_interactive", &["case"], "numerically_eligible"),
    ("tiny_range/1e-150", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("F4/cancelling", "sparse_interactive", &["case"], "numerically_eligible"),
    ("F4/cancelling", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("F4/net", "sparse_interactive", &["case"], "numerically_eligible"),
    ("F4/net", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("F6/partial", "sparse_interactive", &["case", "case-b"], "numerically_eligible"),
    ("F6/partial", "dense_scrutiny", &["case", "case-b"], "numerically_eligible"),
    ("source_budget/multicase", "sparse_interactive", &["case", "case:signed-companion"], "numerically_eligible"),
    ("source_budget/multicase", "dense_scrutiny", &["case", "case:signed-companion"], "numerically_eligible"),
    ("N05/two_case", "sparse_interactive", &["case", "case-2"], "numerically_eligible"),
    ("N05/two_case", "dense_scrutiny", &["case", "case-2"], "numerically_eligible"),
    ("F1b/mixed", "sparse_interactive", &["case:A"], "needs_recompute"),
    ("F1b/mixed", "dense_scrutiny", &["case:A"], "needs_recompute"),
    ("CX-F", "sparse_interactive", &["case"], "numerically_eligible"),
    ("CX-F", "dense_scrutiny", &["case"], "numerically_eligible"),
    ("T18", "sparse_interactive", &["case", "case-b"], "numerically_eligible"),
    ("T18", "dense_scrutiny", &["case", "case-b"], "numerically_eligible"),
    ("T19/pre-K-D5", "sparse_interactive", &["case", "case-b"], "numerically_eligible"),
    ("T19/pre-K-D5", "dense_scrutiny", &["case", "case-b"], "numerically_eligible"),
    ("T19", "sparse_interactive", &["case", "case-b"], "numerically_eligible"),
    ("T19", "dense_scrutiny", &["case", "case-b"], "numerically_eligible"),
    ("T20/refused", "sparse_interactive", &["case"], "needs_recompute"),
    ("T20/refused", "dense_scrutiny", &["case"], "needs_recompute"),
    ("T20/control", "sparse_interactive", &["case"], "needs_recompute"),
    ("T20/control", "dense_scrutiny", &["case"], "needs_recompute"),
];

/// B4/D1 (b) coverage, kept from the census: each formerly selected case publishes a selected W1
/// retained case through the actual private driver, the full Rust reader admits the successor
/// with the invocation, and the trial's standing is the recorded one or better. The ordinary run
/// beside it selects no source blocks and discloses no source-recovery outcome.
#[test]
fn b4_w1_covers_every_formerly_selected_source_block_case() {
    let controls = controls();
    let (mut trials, mut cases) = (0usize, 0usize);
    for (name, mode_name, bases, recorded) in FORMERLY_SELECTED {
        let c = controls.iter().find(|c| c.name == name).unwrap_or_else(|| panic!("control {name}"));
        let mode = MODES.into_iter().find(|m| m.as_str() == mode_name).unwrap();
        assert!(c.modes.contains(&mode), "{name} {mode_name}");
        trials += 1;
        let label = format!("{name} {mode_name}");
        let invocation = json!({"request":c.raw,"solver_mode":mode_name});
        let (request, capture) = source_receipt::CapturedInvocation::parse(c.raw.clone(), mode).unwrap();
        let refs: Vec<Value> = request.model.load_cases.iter()
            .map(|case| json!({"ref_type":"load_case","ref_id":case.id})).collect();
        assert!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY).law().domain.is_none(), "{label}: D1 domain");
        let mut observer = rp::ProductCapture::prepared_probe_on(rp::W1Route::Preview);
        let mut budget = SourceRecoveryBudget::default();
        let mut ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut budget, Some(&mut observer));
        assert!(ordinary.source_block_recovery.is_none(), "{label}: no source blocks");
        assert!(!ordinary.diagnostics.iter().any(|d| d.code.starts_with("SOURCE_BLOCK_RECOVERY")), "{label}");
        assert_eq!(budget.attempts, 0, "{label}: no source attempt");
        ordinary.diagnostics.shrink_to_fit();
        let (_, successor) = retained_w1(observer, ordinary, &capture);
        let successor = successor.unwrap_or_else(|f| panic!("{label}: W1 fell back: {f:?}"));
        let source = successor.value();
        let validation = reader::validate(source, Some(&invocation)).unwrap_or_else(|e| panic!("{label}: reader: {e:?}"));
        assert!(validation.invocation_bound, "{label}");
        if matches!(name, "F6/partial" | "T18" | "T19/pre-K-D5" | "T19") {
            w1_partial_case_cause(name,mode,source);
        }
        for basis in bases {
            cases += 1;
            let case = source["retained_precision"]["body"]["cases"].as_array().unwrap().iter()
                .find(|case| case["basis_ref"]["ref_id"] == json!(basis))
                .unwrap_or_else(|| panic!("{label} {basis}: no receipt case"));
            assert_eq!(case["status"], json!("selected"), "{label} {basis}");
        }
        let standing = sc::numerical_use_standing_with_context(source, &refs, Some(&invocation));
        match recorded {
            "numerically_eligible" => assert_eq!(standing, "numerically_eligible", "{label}"),
            "needs_recompute" => assert!(matches!(standing, "needs_recompute" | "numerically_eligible"), "{label}: {standing}"),
            other => panic!("{label}: recorded standing {other}"),
        }
        eprintln!("B4_W1_COVERAGE {}", json!({"control":name,"mode":mode_name,"cases":bases,"recorded":recorded,"w1_standing":standing}));
    }
    assert_eq!((trials, cases), (54, 70));
}

/// Case 1 (`case-b`, a global-y UDL on the D = 0.2 m section) of F6/partial, T18,
/// T19/pre-K-D5 and T19. Before the W1 coverage repair it was unavailable with "loaded
/// maximum search exhausted" in both modes: the search's stop rule was applied to the
/// width of the represented-Z hull, which subdivision cannot close. The repair drives the
/// admitted lane at the hull's binding endpoint (`w1-coverage-repair.md` §1.2, §1.4), so the
/// search converges and the unchanged gate decides the case. The expectation is that
/// decision, read from the attempt's own certificate check: the case is selected exactly
/// when the gate passed every row. The census (§3.5, C1-a) found the gate passing on all
/// eight trials, and that is asserted too.
fn w1_partial_case_cause(name:&str,mode:PreviewSolverMode,source:&Value) {
    let body=&source["retained_precision"]["body"];
    assert_eq!(body["cases"][0]["status"],"selected","{name} {mode:?}");
    let other=&body["cases"][1];
    assert_eq!(other["basis_ref"]["ref_id"],"case-b","{name} {mode:?}");
    let attempt=body["product_attempts"].as_array().unwrap().iter()
        .find(|a|a["owner_ref"]["index"]==1).unwrap();
    // The search converged: projection completed, and no exhaustion diagnostic exists.
    assert_eq!(attempt["stages"]["projection"],"completed","{name} {mode:?}");
    assert!(attempt["proof"].get("loaded_maximum_diagnostic").is_none_or(Value::is_null),"{name} {mode:?}");
    // The gate decided: the case's status is its certificate check's.
    let gate=&attempt["proof"]["checks"]["certificate"]["kind"];
    let passed=gate=="passed";
    assert_eq!(other["status"],if passed {"selected"} else {"unavailable"},"{name} {mode:?}: the status is the gate's ({gate})");
    assert_eq!(attempt["result"]["kind"],if passed {"ready"} else {"unavailable"},"{name} {mode:?}");
    // Census cross-check (§3.5): the unchanged gate passes this member in every trial and mode.
    assert!(passed,"{name} {mode:?}: the census found the gate passing: {gate}");
    assert!(other.get("reason").is_none_or(Value::is_null),"{name} {mode:?}");
    eprintln!("W1_MEMBER_CENSUS {}",json!({"control":name,"mode":mode.as_str(),
        "status":other["status"],"gate":gate,"result":attempt["result"]}));
}

/// The complete census above covers 54 trials; this subset isolates the other
/// case's outcome so a future change cannot hide it behind case 0's success.
#[test]
fn w1_partial_other_case_statuses_and_causes_are_pinned() {
    let controls=controls();
    for name in ["F6/partial","T18","T19/pre-K-D5","T19"] {
        let c=controls.iter().find(|c|c.name==name).unwrap();
        for mode in MODES {
            let (capture,observer,mut ordinary)=early_observed(&c.raw,mode,true);
            ordinary.diagnostics.shrink_to_fit();
            let (_,successor)=retained_w1(observer,ordinary,&capture);
            let successor=successor.unwrap_or_else(|f|panic!("{name} {mode:?}: {f:?}"));
            w1_partial_case_cause(name,mode,successor.value());
            let invocation=json!({"request":c.raw,"solver_mode":mode.as_str()});
            assert!(reader::validate(successor.value(),Some(&invocation)).unwrap().invocation_bound);
        }
    }
}

/// The committed failed-ordinary successor fixture, the producer's published bytes, read by the
/// Rust reader's and the Python verifier's tests (b7-plan ruling (6): the table pins now follow the
/// reviewed table bytes, so these validate from G0).
const FAILED_ORDINARY_FIXTURE: &str = "fixtures/results/retained_precision_failed_ordinary_cxf.json";

/// The published state (b7-plan ruling (6): the packaged table pins moved to the reviewed table
/// bytes). W1 returns the untouched ordinary owner beside its successor; the successor is the copy
/// precommit received; and the full Rust reader, G0 first, accepts it with the invocation as
/// bound and numerically eligible.
fn published_successor(
    label: &str,
    raw: &Value,
    mode: PreviewSolverMode,
    plain: &MechanicsEnvelope,
    (ordinary, result): (MechanicsEnvelope, Result<RetainedSuccessor, W1Fallback>),
    precommit: Option<Value>,
) -> Value {
    assert_eq!(serde_json::to_vec(&ordinary).unwrap(), serde_json::to_vec(plain).unwrap(),
        "{label}: ordinary bytes beside the successor");
    let s = match result {
        Ok(successor) => successor.value().clone(),
        Err(fallback) => panic!("{label}: W1 fell back: {fallback:?}"),
    };
    assert_eq!(precommit.as_ref(), Some(&s), "{label}: the published successor is precommit's copy");
    let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
    let v = reader::validate(&s, Some(&invocation)).unwrap_or_else(|e| panic!("{label}: reader: {e:?}"));
    assert!(v.invocation_bound && v.numerical_eligible, "{label}: bound and eligible");
    s
}

/// B4/D1 W1 extension (b7-plan "Failed-ordinary successor rulings"): CX-F's ordinary attempt fails
/// genuinely (a range trigger that W2's orchestrator refused with ScaledEvaluation). W1 builds the
/// typed row manifest from the captured maps, freezes the selected retained owner, serializes the
/// successor (rulings (1)-(5)) and publishes it: since ruling (6)'s pin move precommit's G0 accepts
/// the table, and the full reader finds the successor bound and eligible. Until the move this test
/// pinned the held-pin G0 refusal with the blocked fallback. The published successors are the
/// committed fixture, byte for byte.
#[test]
fn b4_failed_ordinary_cxf_publishes_an_eligible_successor() {
    let raw = coexistence_cx_f();
    let mut fixture = Vec::new();
    for mode in MODES {
        let label = mode.as_str();
        let (_, _, plain) = early_observed(&raw, mode, false);
        let (capture, observer, ordinary) = early_observed(&raw, mode, true);
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(), serde_json::to_vec(&plain).unwrap(), "{label}: ordinary bytes");
        assert_eq!(observer.terminal, rp::OrdinaryTerminal::FailedAttempt, "{label}");
        let (returned, _, precommit) =
            retained_tests_hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        let s = published_successor(label, &raw, mode, &plain, returned, precommit);
        let plain = serde_json::to_value(&plain).unwrap();
        // Ruling (1): solved only as a successor; the ordinary envelope stays MODEL_INCOMPLETE.
        assert_ne!(plain["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{label}: the ordinary attempt failed");
        assert_eq!(s["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{label}");
        let rows = s["results"].as_array().unwrap();
        let requested: Vec<&Value> = rows.iter().filter(|r| r["kind"] == json!(rp::REQUESTED_MODE_KIND)).collect();
        assert_eq!(requested.len(), 1, "{label}: ruling (4)");
        assert!(!rows.iter().any(|r| matches!(r["kind"].as_str(),
            Some("linear_solver_mode_basis"))), "{label}: no ordinary observation");
        assert_eq!(requested[0]["value"], json!(mode.mode_code()), "{label}");
        assert_eq!(requested[0]["metadata"]["basis"], json!(rp::requested_mode_basis(mode, 0)), "{label}");
        // Ruling (2): the ordinary tokens kept; the evidence refs add the basis row and the certificate.
        let quality = &s["numerical_quality"]["cases"][0];
        let ordinary_quality = &plain["numerical_quality"]["cases"][0];
        for key in ["structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence"] {
            assert_eq!(quality[key], ordinary_quality[key], "{label}: {key}");
        }
        let mut refs = ordinary_quality["evidence_refs"].as_array().unwrap().clone();
        refs.extend([json!(rp::REQUESTED_MODE_ID), json!("diagnostic:retained-precision:case:selected")]);
        assert_eq!(quality["evidence_refs"], json!(refs), "{label}");
        // Ruling (3): the blocking diagnostic verbatim.
        let blocking = plain["diagnostics"].as_array().unwrap().iter().find(|d| d["severity"] == json!("blocking")).unwrap();
        assert!(s["diagnostics"].as_array().unwrap().contains(blocking), "{label}: blocking diagnostic kept verbatim");
        // Ruling (5): W2's named refusal, translated once.
        let body = &s["retained_precision"]["body"];
        let range = json!({"tag":"evaluation","error":{"tag":"range","detail":"division overflow or underflow"}});
        assert_eq!(body["ordinary_attempts"][0]["w2"], json!({"kind":"failed","trigger":range,
            "failure":{"tag":"refused","refusal":{"reason":{"tag":"scaled_evaluation"},"trigger":range}},
            "diagnostic_ref":"diagnostic:numerical-integrity:case"}), "{label}");
        assert_eq!(body["cases"][0]["status"], json!("selected"), "{label}");
        fixture.push(json!({"solver_mode": label, "request": raw, "successor": s}));
    }
    let fixture = serde_json::to_string(&json!({
        "description": "B4/D1 failed-ordinary W1 successors of the CX-F control, both modes, as the producer serializes them before precommit (b7-plan Failed-ordinary successor rulings). Invented input; standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).",
        "cases": fixture})).unwrap() + "\n";
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(FAILED_ORDINARY_FIXTURE);
    if let Ok(out) = std::env::var("B4_FAILED_ORDINARY_FIXTURE_OUT") {
        std::fs::write(out, &fixture).unwrap();
    } else {
        assert_eq!(std::fs::read_to_string(path).unwrap(), fixture, "the committed fixture is the producer's output");
    }
}

/// The committed N06 failed-ordinary successor fixture (b7-plan "N06 absorbed-spring certificate
/// extension"): two of the twelve N06 census trials in both modes, the producer's published bytes,
/// read from G0 by the Rust reader's and the Python verifier's tests (ruling (6)'s pin move).
const N06_FIXTURE: &str = "fixtures/results/retained_precision_failed_ordinary_n06.json";
const N06_FIXTURE_CONTROLS: [&str; 3] = ["committed/n06-sparse_interactive.request.json",
    "committed/n06-dense_scrutiny.request.json", "sign_rotation/N06/-1/true"];

/// B4/D1: N06's ordinary assembly absorbs its soft rotational spring (GJ/L over the spring is
/// about 2.2e18). Custody and the typed manifest hold and the native Run is selected. DEF-O's
/// annular_source lane fails the existing contraction condition (alpha >= 1 in block 0); under
/// b7-plan's "N06 absorbed-spring certificate extension" that block takes the congruence radius
/// and one further correction, the proof freezes, and W1 publishes the successor, which the full
/// reader finds bound and eligible, as CX-F's. Before the extension this test pinned the alpha
/// refusal (W1Fallback::Candidate), and until ruling (6)'s pin move the held-pin G0 refusal; each
/// expectation moved with the governed change that removed its cause.
#[test]
fn b4_failed_ordinary_n06_freezes_under_the_congruence_extension() {
    let mut fixture = Vec::new();
    let mut trials = 0;
    for c in controls().into_iter().filter(|c| c.name.contains("n06-") || c.name.starts_with("sign_rotation/N06")) {
        for mode in c.modes.clone() {
            trials += 1;
            let label = format!("{} {}", c.name, mode.as_str());
            let (_, observer, ordinary) = early_observed(&c.raw, mode, true);
            assert!(observer.failed_ordinary_case(), "{label}: genuine ordinary failure");
            assert_eq!((observer.ordinary.len(), observer.terminal_calls), (1, 1), "{label}");
            let mut prepared = observer.prepare_cases(ordinary, 1, &[0]).unwrap_or_else(|f| panic!("{label}: {:?}", f.error));
            assert!(prepared.failed_ordinary(), "{label}: the base is the typed manifest");
            prepared.native();
            prepared.freeze();
            match &prepared.attempts[0].end {
                rp::AttemptEnd::Frozen(_) => {}
                rp::AttemptEnd::Candidate(refused) => panic!("{label}: {:?}", refused.error),
                _ => panic!("{label}: expected a frozen certificate"),
            }
            let (_, _, plain_envelope) = early_observed(&c.raw, mode, false);
            let plain = serde_json::to_value(&plain_envelope).unwrap();
            let (capture, observer, ordinary) = early_observed(&c.raw, mode, true);
            assert_eq!(serde_json::to_vec(&ordinary).unwrap(), serde_json::to_vec(&plain_envelope).unwrap(), "{label}: ordinary bytes");
            let (returned, _, precommit) =
                retained_tests_hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            let s = published_successor(&label, &c.raw, mode, &plain_envelope, returned, precommit);
            assert_eq!(s["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{label}");
            assert_eq!(s["retained_precision"]["body"]["cases"][0]["status"], json!("selected"), "{label}");
            let requested: Vec<&Value> = s["results"].as_array().unwrap().iter().filter(|r| r["kind"] == json!(rp::REQUESTED_MODE_KIND)).collect();
            assert_eq!(requested.len(), 1, "{label}: ruling (4)");
            let blocking = plain["diagnostics"].as_array().unwrap().iter().find(|d| d["severity"] == json!("blocking")).unwrap();
            assert!(s["diagnostics"].as_array().unwrap().contains(blocking), "{label}: blocking diagnostic kept verbatim");
            if N06_FIXTURE_CONTROLS.contains(&c.name.as_str()) {
                fixture.push(json!({"control": c.name, "solver_mode": mode.as_str(), "request": c.raw, "successor": s}));
            }
        }
    }
    assert_eq!((trials, fixture.len()), (12, 4));
    let fixture = serde_json::to_string(&json!({
        "description": "B4/D1 failed-ordinary W1 successors of two N06 controls (committed n06 and sign_rotation/N06/-1/true), both modes, certified under b7-plan's N06 absorbed-spring certificate extension, as the producer serializes them before precommit. Invented input; standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).",
        "cases": fixture})).unwrap() + "\n";
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(N06_FIXTURE);
    if let Ok(out) = std::env::var("B4_N06_FIXTURE_OUT") {
        std::fs::write(out, &fixture).unwrap();
    } else {
        assert_eq!(std::fs::read_to_string(path).unwrap(), fixture, "the committed fixture is the producer's output");
    }
}

/// B4/D1 ruling (5): only `ForceScalingFailure::Refused` has a closed translation, with every named
/// reason and its optional inner trigger; every other variant stays an untranslated refusal.
#[test]
fn b4_w2_failed_translation_is_closed_over_the_named_refusal() {
    let range = RangeTrigger::Evaluation(StructuralError::Range("division overflow or underflow"));
    let range_v = json!({"tag":"evaluation","error":{"tag":"range","detail":"division overflow or underflow"}});
    let seed = |failure: ForceScalingFailure| rp::OrdinarySeed { case: "case".into(),
        initial: Some(rp::InitialSeed::StructuralFailure { error: StructuralError::Range("division overflow or underflow"), diagnostic_ref: None }),
        w2: rp::W2Seed::Failed { trigger: range.clone(), failure, diagnostic_ref: "diagnostic:numerical-integrity:case".into() },
        load_row_finding: None, d5_diagnostic_ref: None, recovery_demoted: false };
    for (reason, wire_reason, inner) in [
        (ForceScaleReason::ScaledEvaluation, json!({"tag":"scaled_evaluation"}), Some(range.clone())),
        (ForceScaleReason::SubnormalAtFormation, json!({"tag":"subnormal_at_formation"}), None),
        (ForceScaleReason::InfeasibleWindow { e_min: -1074, e_max: 1023 }, json!({"tag":"infeasible_window","e_min":-1074,"e_max":1023}), Some(range.clone())),
        (ForceScaleReason::PublicationOutsideBinary64 { global_dof: Some(6) }, json!({"tag":"publication_outside_binary64","global_dof":6}), Some(range.clone())),
    ] {
        let inner_v = if inner.is_some() { range_v.clone() } else { Value::Null };
        let ((_, w2, _), failures) = super::retained_wire::test_ordinary_members(&seed(ForceScalingFailure::Refused(ForceScalingRefusal { reason, trigger: inner })));
        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(w2, json!({"kind":"failed","trigger":range_v,"failure":{"tag":"refused","refusal":{"reason":wire_reason,"trigger":inner_v}},
            "diagnostic_ref":"diagnostic:numerical-integrity:case"}));
    }
    for failure in [ForceScalingFailure::NotAdmitted { family: "spring", b: 518 }, ForceScalingFailure::NotEngaged,
        ForceScalingFailure::Publication { quantity: "reaction".into(), b: 4 },
        ForceScalingFailure::Failed(ForceScaledError::Structural(StructuralError::Range("division overflow or underflow")))] {
        let (_, failures) = super::retained_wire::test_ordinary_members(&seed(failure.clone()));
        assert_eq!(failures.first().map(|f| (f.check, f.field_path)), Some((super::retained_wire::ReceiptCheck::Untranslated, "ordinary_attempts[].w2.failure")), "{failure:?}");
    }
}

#[test]
fn b4_w1_n06_cxf_capture_errors() {
    for c in controls().into_iter().filter(|c| {
        c.name == "CX-F"
            || c.name == "committed/n06-sparse_interactive.request.json"
            || c.name == "committed/n06-dense_scrutiny.request.json"
    }) {
        for mode in c.modes {
            let invocation = json!({"request":c.raw,"solver_mode":mode.as_str()});
            let (request, capture) =
                source_receipt::CapturedInvocation::parse(c.raw.clone(), mode).unwrap();
            let domain =
                retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY)
                    .law()
                    .domain;
            let mut observer = rp::ProductCapture::prepared_probe_on(rp::W1Route::Preview);
            let mut budget = SourceRecoveryBudget::default();
            let mut ordinary = run_linear_static_preview_observed(
                request,
                mode,
                Some(&capture),
                &mut budget,
                Some(&mut observer),
            );
            assert!(ordinary.source_block_recovery.is_none());
            let before = json!({"control":c.name,"recipe":c.recipe,"mode":mode.as_str(),
                "domain_refusal":domain.map(|d|format!("{d:?}")),"ordinary_status":ordinary.status.mechanics,
                "ordinary_quality":format!("{:?}",ordinary.numerical_quality),
                "capture_error":observer.error.as_ref().map(|e|format!("{e:?}")),
                "observation_calls":observer.observation_calls,"observations_present":observer.observations.is_some(),
                "late_calls":observer.prepared_late_calls,"source_capture_entries":observer.source_capture_entries,
                "source_present":observer.source.is_some(),"cases_seen":observer.cases_seen(),
                "initial_w2_seeds":format!("{:?}",observer.ordinary),"native_present":observer.native.is_some(),
                "final_calls":observer.final_calls});
            eprintln!("B4_W1_CAPTURE_BEFORE {}", before);
            ordinary.diagnostics.shrink_to_fit();
            let (_, successor) = retained_w1(observer, ordinary, &capture);
            let outcome = match successor {
                Err(error) => json!({"fallback":format!("{error:?}")}),
                Ok(successor) => {
                    json!({"reader":format!("{:?}",reader::validate(successor.value(),Some(&invocation)))})
                }
            };
            eprintln!(
                "B4_W1_CAPTURE_AFTER {}",
                json!({"control":c.name,"mode":mode.as_str(),"w1":outcome})
            );
        }
    }
}

/// The census's per-case causes before precommit: the actual W1 stages, unchanged,
/// stepped one by one so that each lost case names its first refusal. Evidence only.
#[test]
fn b4_w1_census_loss_causes() {
    for c in controls() {
        for mode in c.modes.clone() {
            let Ok((request, capture)) = source_receipt::CapturedInvocation::parse(c.raw.clone(), mode) else {
                eprintln!("B4_W1_CAUSE {}", json!({"control":c.name,"mode":mode.as_str(),"cause":"parse_refused"}));
                continue;
            };
            let mut observer = rp::ProductCapture::prepared_probe_on(rp::W1Route::Preview);
            let mut budget = SourceRecoveryBudget::default();
            let mut ordinary = run_linear_static_preview_observed(request, mode, Some(&capture), &mut budget, Some(&mut observer));
            ordinary.diagnostics.shrink_to_fit();
            let Some(cases) = w1_case_ids(&capture) else {
                eprintln!("B4_W1_CAUSE {}", json!({"control":c.name,"mode":mode.as_str(),"cause":"domain"}));
                continue;
            };
            let triggers: Vec<CaseTrigger> = case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()).collect();
            let attempted = cases.attempted(triggers.iter().copied());
            let slots: Vec<Value> = (0..observer.cases_seen()).map(|i| {
                let (source, terminal) = observer.source_terminal_at(i);
                let error = observer.parked_cases().get(i).map_or_else(|| observer.error.as_ref().map(|e| format!("{e:?}")), |slot| slot.error.as_ref().map(|e| format!("{e:?}")));
                json!({"case":i,"source_ready":source,"terminal":format!("{terminal:?}"),"capture_error":error})
            }).collect();
            let seeds: Vec<String> = observer.ordinary.iter().map(|seed| format!("{:?}/{:?}", seed.initial, seed.w2)).collect();
            let head = json!({"control":c.name,"mode":mode.as_str(),"ordinary_status":ordinary.status.mechanics,
                "triggers":format!("{triggers:?}"),"slots":slots,"seeds":seeds});
            let attempts = match observer.prepare_cases(ordinary, cases.ids().len(), attempted.requests()) {
                Err(failure) => json!({"custody":format!("{:?}", failure.error)}),
                Ok(mut prepared) => {
                    prepared.native();
                    prepared.freeze();
                    let mut rows = Vec::new();
                    for index in 0..prepared.attempts.len() {
                        let request = prepared.attempts[index].request;
                        let end = match &prepared.attempts[index].end {
                            rp::AttemptEnd::Preparation => format!("Preparation({})", prepared.capture.with_case(request, |c| format!("{:?}", c.error))),
                            rp::AttemptEnd::Prepared => "Prepared".into(),
                            rp::AttemptEnd::Native => format!("Native({:?})", prepared.attempts[index].trace.native_error),
                            rp::AttemptEnd::Selected => "Selected".into(),
                            rp::AttemptEnd::Frozen(_) => "Frozen".into(),
                            rp::AttemptEnd::Candidate(refused) => format!("Candidate({:?})", refused.error),
                        };
                        rows.push(json!({"request":request,"end":end}));
                    }
                    json!(rows)
                }
            };
            eprintln!("B4_W1_CAUSE {}", json!({"head":head,"attempts":attempts}));
        }
    }
}

fn early_observed(
    raw: &Value,
    mode: PreviewSolverMode,
    installed: bool,
) -> (
    source_receipt::CapturedInvocation,
    rp::ProductCapture,
    MechanicsEnvelope,
) {
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let route = w1_route(&request.model).unwrap_or(rp::W1Route::Preview);
    let mut observer = rp::ProductCapture::prepared_probe_on(route);
    let mut budget = SourceRecoveryBudget::default();
    let ordinary = run_linear_static_preview_observed(
        request,
        mode,
        Some(&capture),
        &mut budget,
        if installed { Some(&mut observer) } else { None },
    );
    (capture, observer, ordinary)
}

#[test]
fn b4_early_failed_inputs_are_ready_with_genuine_terminal_and_no_rescue() {
    for c in controls().into_iter().filter(|c| {
        c.name == "CX-F"
            || c.name == "committed/n06-sparse_interactive.request.json"
            || c.name == "committed/n06-dense_scrutiny.request.json"
    }) {
        for mode in c.modes {
            let (_, _, plain) = early_observed(&c.raw, mode, false);
            let (capture, observer, ordinary) = early_observed(&c.raw, mode, true);
            assert_eq!(
                serde_json::to_vec(&ordinary).unwrap(),
                serde_json::to_vec(&plain).unwrap()
            );
            assert!(retained_memory::ordinary_solve_attempted(&observer, 1));
            assert_eq!(observer.terminal_calls, 1);
            assert_eq!(observer.terminal, rp::OrdinaryTerminal::FailedAttempt);
            assert!(observer.source.is_some());
            assert!(observer.error.is_none());
            assert_eq!(observer.source_capture_entries, 1);
            assert_eq!(observer.early_source_count(), 1);
            assert_eq!(
                observer.source.as_ref().unwrap().loads().len(),
                c.raw["model"]["load_cases"][0]["primitive_loads"]
                    .as_array()
                    .unwrap()
                    .len()
            );
            for term in observer.source.as_ref().unwrap().loads() {
                let raw = &c.raw["model"]["load_cases"][0]["primitive_loads"][0];
                assert_eq!(term.source_id, raw["id"].as_str().unwrap());
                assert_eq!(
                    term.value.to_bits(),
                    raw["magnitude"]["value"].as_f64().unwrap().to_bits()
                );
            }
            assert_eq!(
                (
                    observer.observation_calls,
                    observer.prepared_late_calls,
                    observer.final_calls
                ),
                (0, 0, 0)
            );
            assert!(observer.native.is_none());
            assert!(observer.observations.is_none());
            eprintln!(
                "B4_EARLY_PREPARATION {}",
                json!({"control":c.name,"mode":mode.as_str(),"terminal":format!("{:?}",observer.terminal),"source_ready":true,"layout":observer.early_source_layout()})
            );
            // Before the failed-ordinary extension custody refused these at Preparation. The
            // governed extension (b7-plan "Failed-ordinary successor rulings") carries them past
            // custody; both CX-F and N06 (under the N06 absorbed-spring certificate extension)
            // freeze; and since ruling (6)'s pin move, which ended the held-pin G0 refusal this
            // pinned, W1 publishes an eligible successor. Early readiness alone created no attempt
            // (above): the successor comes from W1's own stages.
            let label = format!("{} {}", c.name, mode.as_str());
            let (returned, _, precommit) =
                retained_tests_hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            let s = published_successor(&label, &c.raw, mode, &plain, returned, precommit);
            assert_eq!(s["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{label}");
        }
    }
}

#[test]
fn b4_early_normal_and_partial_controls_keep_ordinary_bytes_and_custody() {
    let single: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/w1_controls/n05-sparse_interactive.request.json"
    ))
    .unwrap();
    let partial = controls()
        .into_iter()
        .find(|c| c.name == "F6/partial")
        .unwrap()
        .raw;
    for raw in [single, n05_two_cases(), partial] {
        for mode in MODES {
            let (_, _, plain) = early_observed(&raw, mode, false);
            let (_, observer, ordinary) = early_observed(&raw, mode, true);
            assert_eq!(
                serde_json::to_vec(&ordinary).unwrap(),
                serde_json::to_vec(&plain).unwrap()
            );
            assert_eq!(observer.terminal_calls, 1);
            for case in 0..observer.cases_seen() {
                assert_eq!(
                    observer.source_terminal_at(case).1,
                    rp::OrdinaryTerminal::ObservedCompletion
                );
            }
            let (request, capture) =
                source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let admitted =
                retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY)
                    .law()
                    .domain
                    .is_none();
            if admitted {
                assert_eq!(
                    observer.early_source_count(),
                    raw["model"]["load_cases"].as_array().unwrap().len()
                );
                // Stage 2 retains a nodal declaration plus a separate member-only sidecar.
                for case in 0..observer.cases_seen() {
                    let member_loads = observer.member_loads_basis_at(case).0;
                    let loads = &raw["model"]["load_cases"][case]["primitive_loads"];
                    assert_eq!(member_loads, loads.as_array().unwrap().iter().any(|l| l["target"]["type"] == "element"));
                    assert!(observer.source_terminal_at(case).0);
                    assert_eq!(observer.member_source_at(case).is_some(), member_loads);
                }
                assert!(observer.error.is_none());
            } else {
                assert_eq!(observer.early_source_count(), 0);
            }
        }
    }
}

#[test]
fn b4_early_invalid_unrun_missing_and_repeated_capture_refuse() {
    let raw: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/w1_controls/n05-sparse_interactive.request.json"
    ))
    .unwrap();
    for mode in MODES {
        let mut invalid = raw.clone();
        invalid["model"]["load_cases"][0]["primitive_loads"][0]["target"]["node"] =
            json!("not-a-node");
        let (_, _, plain) = early_observed(&invalid, mode, false);
        let (_, observer, ordinary) = early_observed(&invalid, mode, true);
        assert_eq!(
            serde_json::to_vec(&ordinary).unwrap(),
            serde_json::to_vec(&plain).unwrap()
        );
        assert!(!retained_memory::ordinary_solve_attempted(&observer, 1));
        assert!(observer.source.is_none());
        assert_eq!(observer.terminal, rp::OrdinaryTerminal::MissingOrUnrun);
        let mut unrun = raw.clone();
        unrun["model"]["supports"] = json!([]);
        let (_, observer, _) = early_observed(&unrun, mode, true);
        assert!(!retained_memory::ordinary_solve_attempted(&observer, 1));
        assert_eq!(observer.early_source_count(), 0);
        let (_, mut observer, ordinary) = early_observed(&raw, mode, true);
        observer.observations = None;
        assert!(
            observer.prepare_case(ordinary).is_err(),
            "missing successful observation remains refused"
        );
        let (_, mut observer, _) = early_observed(&raw, mode, true);
        observer.settle_ordinary_terminal();
        assert_eq!(observer.terminal_calls, 2);
        assert!(observer.error.is_some());
        let (_, mut observer, _) = early_observed(&raw, mode, true);
        let mut changed = raw.clone();
        changed["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(2e-8);
        assert!(observer.test_refuse_changed_occurrence(&changed).is_err());
    }
}

#[test]
fn b4_early_failed_terminal_keeps_dn_exclusions() {
    let raw = coexistence_cx_f();
    for error in [
        StructuralError::InvalidInput("invented invalid input".into()),
        StructuralError::Asymmetric {
            row: 0,
            col: 1,
            relative_skew: 1.0,
        },
        StructuralError::Mechanism {
            direction: vec![0.0; 12],
        },
    ] {
        let (capture, mut observer, ordinary) =
            early_observed(&raw, PreviewSolverMode::SparseInteractive, true);
        observer.ordinary[0].initial = Some(rp::InitialSeed::StructuralFailure {
            error,
            diagnostic_ref: None,
        });
        observer.ordinary[0].w2 = rp::W2Seed::NotTriggered;
        let (_, successor) = retained_w1(observer, ordinary, &capture);
        assert_eq!(successor.err(), Some(W1Fallback::NoTriggeredCase));
    }
}

#[test]
fn b4_early_captured_binding_refuses_changed_raw_context_and_normalizes_units() {
    let raw: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/w1_controls/n05-sparse_interactive.request.json"
    ))
    .unwrap();
    for mode in MODES {
        for mutation in 0..3 {
            let (mut request, capture) =
                source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            match mutation {
                0 => {
                    request.model.load_cases[0].primitive_loads[0]
                        .magnitude
                        .value = 2e-8
                }
                1 => request.model.nodes[1].position.x = 3.0,
                _ => {
                    request.model.supports[1]
                        .stiffness
                        .as_mut()
                        .unwrap()
                        .value
                        .value = 0.0002
                }
            }
            let mut observer = rp::ProductCapture::prepared_probe_on(rp::W1Route::Preview);
            let mut budget = SourceRecoveryBudget::default();
            let ordinary = run_linear_static_preview_observed(
                request,
                mode,
                Some(&capture),
                &mut budget,
                Some(&mut observer),
            );
            assert!(observer.source.is_none(), "captured mutation {mutation}");
            assert!(observer.error.is_some());
            assert!(observer.prepare_case(ordinary).is_err());
        }
        let unit_raw: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/pressure_source/physics_source/n05_units.request.json"
        ))
        .unwrap();
        let (_, _, plain) = early_observed(&unit_raw, mode, false);
        let (_, observer, ordinary) = early_observed(&unit_raw, mode, true);
        assert_eq!(
            serde_json::to_vec(&ordinary).unwrap(),
            serde_json::to_vec(&plain).unwrap()
        );
        assert!(
            observer.source.is_some(),
            "maintained exact-route unit control: {:?}",
            observer.error
        );
        // B4/D1 (b): under the one source budget (physics-source-1's 8,000,000) the composite
        // selects n05_units, so the run settles by T-3 (c)'s coexistence before W1 starts; the
        // selected publication carries no solver-mode observation, which the observer records.
        assert!(ordinary.source_block_recovery.is_some(), "{mode:?}: composite selected");
        assert!(observer.error.is_some(), "{mode:?}: the selected run is not W1's");
    }
}


#[test]
fn b4_early_exact_material_binding_uses_authoritative_subnormal_words() {
    let baseline: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/pressure_source/physics_source/n05_units.request.json"
    ))
    .unwrap();
    for (nu, authoritative_bits, direct_bits) in [
        (-0.25, 0x000a_aaaa_aaaa_aaaau64, 0x000a_aaaa_aaaa_aaab),
        (-0.4, 0x000d_5555_5555_5556, 0x000d_5555_5555_5555),
    ] {
        for mode in MODES {
            let mut raw = baseline.clone();
            raw["model"]["materials"][0]["elastic_modulus"] =
                json!({"value": f64::MIN_POSITIVE, "unit": "Pa"});
            raw["model"]["materials"][0]["poisson_ratio"] =
                json!({"value": nu, "unit": "1"});
            let (request, capture) =
                source_receipt::CapturedInvocation::parse(raw, mode).unwrap();
            let mut model = request.model;
            let mut materials = model.materials.clone();
            let mut diagnostics = Vec::new();
            normalize_model_units(&mut model, &mut materials, &mut diagnostics);
            pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
            assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
            assert_eq!(materials[0].elastic_modulus.value.to_bits(), f64::MIN_POSITIVE.to_bits());
            let g = materials[0].shear_modulus.as_ref().unwrap().value;
            assert_eq!(g.to_bits(), authoritative_bits);
            assert_eq!((f64::MIN_POSITIVE / (2.0 * (1.0 + nu))).to_bits(), direct_bits);
            let mut observer = rp::ProductCapture::prepared_probe_on(rp::W1Route::Exact);
            observer.invocation(Some(&capture), mode);
            assert!(observer
                .test_check_captured_structure(&model, &materials, &capture)
                .is_ok(), "unchanged normalized leaves must bind exactly");
            // Structural identity does not widen the independent retained exact-source gate.
            assert!(observer.exact_material_nu(&materials[0]).is_err());
            materials[0].shear_modulus.as_mut().unwrap().value = f64::from_bits(direct_bits);
            assert!(observer
                .test_check_captured_structure(&model, &materials, &capture)
                .is_err(), "one-ulp normalization tamper must refuse");
        }
    }
}

// B7a (b) Widen domain, stage 1: per-case basis custody and member-load cases.
#[path = "w1_domain_stage1_tests.rs"]
mod w1_domain_stage1_tests;

// The W1 coverage repair census (DEL-04-08 Design `w1-coverage-repair.md` §3,
// `w1e-sharper-exact-coverage.md` §6.2): opt-in, `--features census-probe` only.
#[cfg(feature = "census-probe")]
#[path = "w1_coverage_census_tests.rs"]
mod w1_coverage_census_tests;

/// An opt-in kernel feature is named only as the frame kernel's `[features]` key
/// (`<feature> = []`) and as `product_physics`' forward of it, once each, never in a
/// dependency table or a `default` list (the `mutation-controls` guard's rule). The
/// registered build identity names no features and `Cargo.lock` records none, so the
/// manifests are where an accidental enablement would show.
fn opt_in_feature_is_named_only_by_its_definition_and_forward(feature: &str) {
    fn manifests(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if path.is_dir() {
                if name.starts_with('.') || ["target", "node_modules", "execution"].contains(&name.as_str()) { continue; }
                manifests(&path, out);
            } else if name == "Cargo.toml" {
                out.push(path);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap();
    let mut found = Vec::new();
    manifests(&root, &mut found);
    let (mut definitions, mut forwards, mut violations) = (0, 0, Vec::new());
    for path in &found {
        let text = std::fs::read_to_string(path).unwrap();
        let mut table = String::new();
        for raw in text.lines() {
            let line = raw.split('#').next().unwrap().trim();
            if line.starts_with('[') { table = line.to_owned(); continue; }
            if !line.contains(feature) { continue; }
            let key = line.split('=').next().unwrap().trim();
            let kernel = path.ends_with("core/solver/frame_kernel/Cargo.toml");
            let pp = path.ends_with("core/product_physics/Cargo.toml");
            if table == "[features]" && kernel && line == format!("{feature} = []") {
                definitions += 1;
            } else if table == "[features]" && pp && key == feature && line == format!(r#"{feature} = ["open_pipe_stress_frame_kernel/{feature}"]"#) {
                forwards += 1;
            } else {
                violations.push(format!("{}: {table} {line}", path.display()));
            }
        }
    }
    assert!(violations.is_empty(), "{feature} named outside its definition and forward: {violations:#?}");
    assert_eq!((definitions, forwards), (1, 1), "the kernel defines {feature} and product_physics forwards it, once each");
}
/// W1 coverage repair §3.2 (revision 8): the census probe's feature.
#[test]
fn census_probe_is_named_only_by_its_definition_and_forward() {
    opt_in_feature_is_named_only_by_its_definition_and_forward("census-probe");
}
/// W1 coverage repair §1.4, option 2: the loaded-maximum capacity hook follows the same
/// rule. Only the genuine-exhaustion control's maintained command passes it.
#[test]
fn loaded_maximum_hooks_are_named_only_by_their_definition_and_forward() {
    opt_in_feature_is_named_only_by_its_definition_and_forward("loaded-maximum-hooks");
}

/// W1 coverage repair §3.2: the census probe is opt-in. No dev-dependency enables it,
/// so the ordinary test build (and the stack-witness build) compiles the kernel without
/// it. A census command, which passes `--features census-probe`, does not run this test.
#[test]
fn census_probe_is_off_in_the_ordinary_test_build() {
    assert!(!open_pipe_stress_frame_kernel::structural::retained_api::CENSUS_PROBE,
        "the frame kernel was compiled with the census probe in an ordinary test build");
}

/// W1 coverage repair §1.4, option 2: the loaded-maximum capacity hook is opt-in. No
/// dev-dependency enables it, so the ordinary test build (and the stack-witness build)
/// compiles the kernel without it and every search runs at its production limits. The
/// genuine-exhaustion control's command, which passes `--features loaded-maximum-hooks`,
/// does not compile this test.
#[cfg(not(feature = "loaded-maximum-hooks"))]
#[test]
fn loaded_maximum_hooks_are_off_in_the_ordinary_test_build() {
    assert!(!open_pipe_stress_frame_kernel::structural::retained_api::LOADED_MAXIMUM_HOOKS,
        "the frame kernel was compiled with the loaded-maximum capacity hook in an ordinary test build");
}

/// T4-U4b S4 (DEL-04-07 Design u4b-u5b-implementation.md §4.1): the failed-ordinary manifest
/// carries each member's 15 derived rows after its circular maximum (66 rows per member, in
/// preview-physics-2's order) as placeholders, and the freeze forms them from the projected rows.
/// CX-F and the two committed N06 controls stay selected, bound and eligible in both modes; every
/// derived row is not covered, recomputed from the successor's own rows, and read by the base
/// reader holding the captured model.
#[test]
fn s4_failed_ordinary_manifest_carries_derived_rows_formed_after_projection() {
    use super::s4_retained_tests as s4;
    let mut inputs = vec![("CX-F".to_string(), coexistence_cx_f(), MODES.to_vec())];
    inputs.extend(controls().into_iter().filter(|c| N06_FIXTURE_CONTROLS.contains(&c.name.as_str())).map(|c| (c.name.clone(), c.raw.clone(), c.modes.clone())));
    assert_eq!(inputs.iter().map(|i| i.2.len()).sum::<usize>(), 6, "CX-F and the committed N06 controls, both modes");
    for (name, raw, modes) in &inputs {
        for mode in modes.iter().copied() {
            let label = format!("{name} {}", mode.as_str());
            let (capture, observer, ordinary) = early_observed(raw, mode, true);
            assert!(observer.failed_ordinary_case(), "{label}: genuine ordinary failure");
            let ((_, retained), _, captured) = retained_tests_hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            let s = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
            assert!(retained.is_ok(), "{label}: {retained:?}");
            assert_eq!(s["retained_precision"]["body"]["cases"][0]["status"], json!("selected"), "{label}");
            let rows = s["results"].as_array().unwrap();
            let members: Vec<&str> = raw["model"]["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].as_str().unwrap()).collect();
            for member in &members {
                let own: Vec<&Value> = rows.iter().filter(|r| r["entity_ref"] == *member).collect();
                // W-7: 12 global end rows (after the maximum, as on a solved case) join the 66.
                assert_eq!(own.len(), 78, "{label}: {member}'s manifest rows");
                let maximum = own.iter().position(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2").unwrap();
                assert!(own[maximum + 1..maximum + 13].iter().all(|r| s4::GLOBAL_END.contains(&r["kind"].as_str().unwrap())),
                    "{label}: {member}'s global end rows follow its maximum");
                assert!(own[maximum + 13..].iter().all(|r| s4::DERIVED.contains(&r["kind"].as_str().unwrap())) && own.len() - maximum - 13 == 15,
                    "{label}: {member}'s derived rows follow its global end rows");
            }
            assert_eq!(s4::derived_rows_follow_own_rows(&label, &s), 15 * members.len());
            s4::base_read_with_model(&label, &s, raw);
            let validation = s4::receipt_and_reader_classify(&label, &s, raw, mode);
            assert!(validation.numerical_eligible, "{label}: bound and eligible");
            s4::tau_tamper_is_refused_by_the_reader(&label, &s, raw, mode);
        }
    }
}

/// S4 round 2 (§2.4, §4.1): a failed-ordinary member whose ν is out of domain (E ≥ 3G; I47's
/// moduli, E = 195 GPa and G = 55 GPa, ν = 0.7727…) stays selected. Its manifest omits the τ rows
/// (61 rows per member) and carries `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN` for the case and
/// member, as the ordinary rows do; σ± are formed after projection and every reader admits it.
#[test]
fn s4_failed_ordinary_out_of_domain_poisson_omits_tau_and_stays_selected() {
    use super::s4_retained_tests as s4;
    let mut raw = coexistence_cx_f();
    raw["model"]["materials"][0]["elastic_modulus"]["value"] = json!(195e9);
    raw["model"]["materials"][0]["shear_modulus"]["value"] = json!(55e9);
    for mode in MODES {
        let label = format!("CX-F E=3.5G {}", mode.as_str());
        let (capture, observer, ordinary) = early_observed(&raw, mode, true);
        assert!(observer.failed_ordinary_case(), "{label}: genuine ordinary failure");
        let ((_, retained), _, captured) = retained_tests_hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        let s = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
        assert!(retained.is_ok(), "{label}: {retained:?}");
        assert_eq!(s["retained_precision"]["body"]["cases"][0]["status"], json!("selected"), "{label}: still selected");
        let rows = s["results"].as_array().unwrap();
        let member = raw["model"]["pipe_segments"][0]["id"].as_str().unwrap();
        let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap();
        // W-7: 61 + the member's 12 global end rows.
        assert_eq!(rows.iter().filter(|r| r["entity_ref"] == member).count(), 73, "{label}: no τ rows");
        assert!(!rows.iter().any(|r| r["kind"] == s4::DERIVED[1]), "{label}: no τ row");
        assert_eq!(rows.iter().filter(|r| r["kind"] == s4::DERIVED[0]).count(), 10, "{label}: σ± at five stations");
        let named: Vec<&Value> = s["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN").collect();
        assert_eq!(named.len(), 1, "{label}: one named warning");
        assert_eq!(named[0]["affected_refs"], json!([case, member]), "{label}");
        assert_eq!(named[0]["severity"], json!("warning"), "{label}");
        assert_eq!(s4::derived_rows_follow_own_rows(&label, &s), 10);
        s4::base_read_with_model(&label, &s, &raw);
        let validation = s4::receipt_and_reader_classify(&label, &s, &raw, mode);
        assert!(validation.numerical_eligible, "{label}: bound and eligible");
    }
}
