//! Public producer controls for retained-source recovery. Expected values come
//! from the maintained independently derived numerical-integrity references.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::Value;

fn scalar(reference: &Value, key: &str) -> f64 {
    reference[key].as_str().unwrap().parse().unwrap()
}

fn assert_relative(actual: f64, expected: f64) {
    assert!(actual.is_finite());
    if expected == 0.0 {
        assert_eq!(actual, expected);
    } else {
        let error = ((actual - expected) / expected).abs();
        assert!(error <= 1e-9, "actual={actual:e}, expected={expected:e}, relative={error:e}");
    }
}

fn row<'a>(raw: &'a Value, entity: &str, kind: &str, location: Option<&str>) -> &'a Value {
    let matching: Vec<_> = raw["results"].as_array().unwrap().iter().filter(|r| {
        r["entity_ref"] == entity && r["kind"] == kind
            && location.map_or(true, |value| r["metadata"]["location"] == value)
    }).collect();
    assert_eq!(matching.len(), 1, "{entity} {kind} {location:?}");
    matching[0]
}

#[test]
fn retained_source_rotation_member_torque_and_device_action_reach_public_rows() {
    let references: Value = serde_json::from_str(include_str!("../../../validation/benchmarks/numerical_integrity/fixtures.json")).unwrap();
    let original: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap();
    for case in ["N05", "N06"] {
        let reference = &references["N"][case];
        for sign in [1.0, -1.0] {
            for rotated in [false, true] {
                let mut model = original.clone();
                model["supports"][1]["stiffness"]["value"]["value"] = scalar(reference, "spring_RX_Nm_per_rad").into();
                model["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = (sign * scalar(reference, "tip_MX_Nm")).into();
                // Identity changes are unrelated to method eligibility.
                model["project"]["id"] = format!("invented:source-recovery:{case}:{sign}:{rotated}").into();
                model["nodes"][0]["id"] = "independent-root".into();
                model["nodes"][1]["id"] = "independent-tip".into();
                model["pipe_segments"][0]["from"] = "independent-root".into();
                model["pipe_segments"][0]["to"] = "independent-tip".into();
                model["pipe_segments"][0]["id"] = "independent-member".into();
                model["supports"][0]["node"] = "independent-root".into();
                model["supports"][1]["node"] = "independent-root".into();
                model["supports"][1]["id"] = "independent-spring".into();
                model["load_cases"][0]["primitive_loads"][0]["target"]["node"] = "independent-tip".into();
                if rotated {
                    // Proper cyclic signed-permutation rotation: X->Y,Y->Z,Z->X.
                    model["nodes"][1]["position"] = serde_json::json!({"x":0,"y":2,"z":0});
                    model["pipe_segments"][0]["y_reference"] = serde_json::json!({"x":0,"y":0,"z":1});
                    model["supports"][0]["restraints"] = serde_json::json!(["UY","UZ","UX","RZ","RX"]);
                    model["supports"][1]["restraints"] = serde_json::json!(["RY"]);
                    model["supports"][1]["stiffness"]["dof"] = "RY".into();
                    model["load_cases"][0]["primitive_loads"][0]["direction"] = "RY".into();
                }
                for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
                    let request = serde_json::json!({"model":model,"materials":[]});
                    let raw = serde_json::to_value(run_linear_static_preview_value_with_mode(request, mode).unwrap()).unwrap();
                    assert_eq!(raw["status"]["mechanics"], "MECHANICS_SOLVED", "{case}: {:?}", raw["diagnostics"]);
                    assert_eq!(raw["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/source-blocks-1");
                    assert!(raw["source_block_recovery"].is_object());
                    assert_eq!(raw["source_block_recovery"]["body"]["status"], "qualified");
                    if !rotated && sign == 1.0 {
                        if let Ok(folder) = std::env::var("OPS_SOURCE_BLOCK_FIXTURE_DIR") {
                            let folder = std::path::Path::new(&folder); std::fs::create_dir_all(folder).unwrap();
                            let name = format!("{}-{}", case.to_lowercase(), mode.as_str());
                            std::fs::write(folder.join(format!("{name}.request.json")), serde_json::to_vec_pretty(&serde_json::json!({"model":model,"materials":[]})).unwrap()).unwrap();
                            std::fs::write(folder.join(format!("{name}.raw.json")), serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
                        }
                    }
                    if std::env::var_os("OPS_SOURCE_BLOCK_BUDGET_OBSERVATION").is_some() {
                        eprintln!("SOURCE_BLOCK_BUDGET_OBSERVATION {}", serde_json::json!({"case":case,"mode":mode.as_str(),"sign":sign,"rotated":rotated,"work":raw["source_block_recovery"]["body"]["cases"][0]["work"]}));
                    }
                    let rotation = if rotated { "global_nodal_rotation_y" } else { "global_nodal_rotation_x" };
                    assert_relative(row(&raw,"independent-root",rotation,None)["value"].as_f64().unwrap(), sign * scalar(reference,"theta_root_rad"));
                    assert_relative(row(&raw,"independent-tip",rotation,None)["value"].as_f64().unwrap(), sign * scalar(reference,"theta_tip_rad"));
                    let torque = sign * scalar(reference,"tip_MX_Nm");
                    for (location, expected) in [("end_i",-torque),("end_j",torque),("quarter_1",torque),("midspan",torque),("quarter_3",torque)] {
                        let value = row(&raw,"independent-member","element_local_torsional_moment",Some(location));
                        assert_eq!(value["unit"],"N*m");
                        assert_relative(value["value"].as_f64().unwrap(),expected);
                    }
                    let component = if rotated {"My"} else {"Mx"};
                    let actions: Vec<_> = raw["results"].as_array().unwrap().iter().filter(|r| r["kind"]=="support_reaction_component_v2" && r["entity_ref"]=="independent-spring" && r["metadata"]["component"]==component).collect();
                    assert_eq!(actions.len(),1);
                    assert_eq!(actions[0]["unit"],"N*m");
                    assert_relative(actions[0]["value"].as_f64().unwrap(),-torque);
                }
            }
        }
    }
}

#[test]
fn complete_ui_requests_are_authored_before_the_real_source_invocation() {
    let fixture_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/product_preview/source_blocks");
    for stem in ["n05", "n06", "multicase"] {
        for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
            let name = format!("{stem}-{}", mode.as_str());
            let mut request: Value = serde_json::from_slice(&std::fs::read(fixture_root.join(format!("{name}.request.json"))).unwrap()).unwrap();
            let model = &mut request["model"];
            model["project"]["name"] = format!("Invented {stem} source-recovery UI control").into();
            model["project"]["description"] = "Numerical reference inputs with explicitly invented desktop metadata; no external project or library data.".into();
            model["data_boundary"] = serde_json::json!({
                "public_examples_policy":"invented_or_cleared_data_only",
                "protected_source_policy":"no_bundled_protected_owner_or_standards_data",
                "private_data_policy":"no_private_project_data",
                "professional_boundary":"technical_preview_requires_human_engineering_review"
            });
            model["components"] = serde_json::json!([]);
            model["diagnostics"] = serde_json::json!([]);
            for category in ["nodes", "pipe_segments", "supports", "materials", "load_cases"] {
                for item in model[category].as_array_mut().unwrap() {
                    item["label"] = format!("Invented {}", item["id"].as_str().unwrap()).into();
                    if category == "load_cases" { item["status"] = "user_entered".into(); }
                }
            }
            let result = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
            let raw = serde_json::to_value(result).unwrap();
            assert_eq!(raw["source_block_recovery"]["body"]["status"], "qualified");
            let original: Value = serde_json::from_slice(&std::fs::read(fixture_root.join(format!("{name}.raw.json"))).unwrap()).unwrap();
            assert_eq!(raw["results"], original["results"], "authored display metadata must not alter mechanics");
            assert_ne!(raw["source_block_recovery"]["body"]["invocation"], original["source_block_recovery"]["body"]["invocation"]);
            if let Ok(folder) = std::env::var("OPS_SOURCE_BLOCK_UI_FIXTURE_DIR") {
                let folder = std::path::Path::new(&folder); std::fs::create_dir_all(folder).unwrap();
                std::fs::write(folder.join(format!("{name}.request.json")), serde_json::to_vec_pretty(&request).unwrap()).unwrap();
                std::fs::write(folder.join(format!("{name}.raw.json")), serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
            }
        }
    }
}

#[test]
fn actual_source_recovery_scaled_norms_admit_tiny_actions_and_keep_large_json_boundary() {
    let original: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap();
    for force in [1e-150_f64, 1e160_f64] {
        let mut model = original.clone();
        model["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(serde_json::json!({
            "id":"invented:range-axial-force", "category":"concentrated_force", "dimension":"force",
            "direction":"global_x", "magnitude":{"value":force,"unit":"N"},
            "target":{"type":"node","node":"tip"},
            "provenance":"invented_derived_range_control_no_external_project_data"
        }));
        for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
            let result = run_linear_static_preview_value_with_mode(serde_json::json!({"model":model,"materials":[]}), mode);
            if force > 1.0 {
                // The real invocation must first satisfy checked I-JSON. This
                // boundary is not bypassed to manufacture a large happy path.
                assert!(result.unwrap_err().starts_with("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"));
                continue;
            }
            let result = result.unwrap();
            assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{force:e} {mode:?}: {:?}", result.diagnostics);
            assert_eq!(result.source_block_recovery.as_ref().unwrap()["body"]["status"], "qualified");
            let raw = serde_json::to_value(result).unwrap();
            let displacement = row(&raw,"tip","global_nodal_displacement_x",None)["value"].as_f64().unwrap();
            let magnitude = row(&raw,"tip","displacement_magnitude",None)["value"].as_f64().unwrap();
            assert_eq!(magnitude.to_bits(), displacement.abs().to_bits());
            let reaction = row(&raw,"anchor","reaction_resultant",None)["value"].as_f64().unwrap();
            assert_relative(reaction, force);
            assert!((displacement * displacement).is_subnormal());
        }
    }
}

#[test]
fn derived_stress_unit_range_cannot_mint_source_qualification() {
    let mut model: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/numerical_sensitive_torsion_model.json")).unwrap();
    model["project"]["id"] = "invented:derived-stress-range".into();
    model["pipe_segments"][0]["section"]["outside_diameter"]["value"] = 2e15.into();
    model["pipe_segments"][0]["section"]["wall_thickness"]["value"] = 1e14.into();
    model["materials"][0]["elastic_modulus"]["value"] = 1e-29.into();
    model["materials"][0]["shear_modulus"]["value"] = 1e-53.into();
    model["load_cases"][0]["primitive_loads"] = serde_json::json!([{
        "id":"invented:tiny-axial-force", "category":"concentrated_force", "dimension":"force",
        "direction":"global_x", "magnitude":{"value":1e-282,"unit":"N"},
        "target":{"type":"node","node":"tip"},
        "provenance":"invented_derived_stress_range_control_no_external_project_data"
    }]);
    let mut improperly_qualified = Vec::new();
    for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
        let request = serde_json::json!({"model":model,"materials":[]});
        let result = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
        let qualified = result.source_block_recovery.as_ref().is_some_and(|r| r["body"]["status"] == "qualified");
        if let Ok(folder) = std::env::var("OPS_SOURCE_BLOCK_RANGE_PROBE_DIR") {
            let folder = std::path::Path::new(&folder); std::fs::create_dir_all(folder).unwrap();
            std::fs::write(folder.join(format!("{}.request.json",mode.as_str())),serde_json::to_vec_pretty(&request).unwrap()).unwrap();
            std::fs::write(folder.join(format!("{}.raw.json",mode.as_str())),serde_json::to_vec_pretty(&result).unwrap()).unwrap();
        }
        if qualified { improperly_qualified.push(mode.as_str()); }
        else {
            assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
            assert!(result.diagnostics.iter().any(|d| d.code == "SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED" && d.message.contains("stress")));
        }
    }
    assert!(improperly_qualified.is_empty(), "subnormal MPa summary has independently measured relative error 1.26945e-6: {improperly_qualified:?}");
}
