//! Genuine public-producer fixtures plus explicitly corrupted copies, not native Current.
use open_pipe_stress_result_export::{physics_source, semantic_contract, source_blocks};
use serde_json::{json, Value};
use std::path::PathBuf;
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../")
}
fn read(path: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(root().join(path)).unwrap()).unwrap()
}
fn actual(name: &str, mode: &str) -> (Value, Value) {
    (
        read(&format!(
            "fixtures/product_preview/physics_source/{name}-{mode}.raw.json"
        )),
        json!({"request":read(&format!("fixtures/product_preview/physics_source/{name}.request.json")),"solver_mode":mode}),
    )
}
fn reseal_negative(source: &mut Value, invocation: Option<&Value>) {
    let physical = source["contract_evidence"].clone();
    let body = &mut source["source_block_recovery"]["body"];
    if let Some(invocation) = invocation {
        body["invocation"]["value"] =
            json!(source_blocks::domain_hash("source_blocks_invocation_v1", invocation).unwrap());
    }
    for case in body["cases"].as_array_mut().unwrap() {
        let exact = physical["exact_cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["load_case_id"] == case["basis_ref"]["ref_id"])
            .unwrap();
        let pressure = physical["pressure"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|p| p["load_case_id"] == exact["load_case_id"])
            .collect::<Vec<_>>();
        case["physical_evidence_sha256"] = json!(source_blocks::domain_hash(
            "physics_source_case_evidence_v1",
            &json!({"exact_case":exact,"pressure":pressure})
        )
        .unwrap());
    }
    let publication = Value::Object(
        source
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, _)| k.as_str() != "source_block_recovery")
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    );
    source["source_block_recovery"]["body"]["publication_sha256"] =
        json!(source_blocks::domain_hash("source_blocks_publication_v1", &publication).unwrap());
    source["source_block_recovery"]["receipt_sha256"] = json!(source_blocks::domain_hash(
        "source_blocks_receipt_v1",
        &source["source_block_recovery"]["body"]
    )
    .unwrap());
}
#[test]
fn actual_records_require_original_full_invocation() {
    for name in [
        "n05",
        "n06",
        "mixed",
        "fields",
        "n05_units",
        "mixed_units",
        "n05_unicode",
    ] {
        for mode in ["sparse_interactive", "dense_scrutiny"] {
            let (source, invocation) = actual(name, mode);
            let original = source.clone();
            assert_eq!(
                physics_source::validate(&source, Some(&invocation)),
                Ok(true),
                "{name}/{mode}"
            );
            assert_eq!(physics_source::validate(&source, None), Ok(false));
            assert_eq!(
                physics_source::validate_transport_metadata(
                    &json!({"contract_evidence":source["contract_evidence"],"source_block_recovery":source["source_block_recovery"]})
                ),
                Ok(())
            );
            assert_eq!(source, original);
            let mut changed = invocation.clone();
            changed["request"]["unknown_but_captured"] = json!(false);
            assert!(physics_source::validate(&source, Some(&changed)).is_err());
            changed = invocation.clone();
            changed["solver_mode"] = json!(if mode == "sparse_interactive" {
                "dense_scrutiny"
            } else {
                "sparse_interactive"
            });
            assert!(physics_source::validate(&source, Some(&changed)).is_err());
        }
    }
}
#[test]
fn maximum_signature_requires_explicit_basis_without_claiming_current() {
    let (source, _) = actual("n05", "sparse_interactive");
    let mut row = source["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2")
        .unwrap()
        .clone();
    let table = semantic_contract::physics_source_contract();
    let exact = semantic_contract::signature_in(table, &row)
        .unwrap()
        .unwrap();
    assert_eq!(exact["source_basis"], physics_source::MAX_BASIS);
    row["metadata"]["basis"] = json!("recovered_from_open_mechanics_stress_components");
    let ordinary = semantic_contract::signature_in(table, &row)
        .unwrap()
        .unwrap();
    assert_ne!(exact["signature_id"], ordinary["signature_id"]);
    row["metadata"]["basis"] = json!("invented");
    assert!(semantic_contract::signature_in(table, &row).is_err());
    row["metadata"].as_object_mut().unwrap().remove("basis");
    assert!(semantic_contract::signature_in(table, &row).is_err());
}
#[test]
fn old_entrypoints_and_composite_remain_separate() {
    let (source, invocation) = actual("n05", "sparse_interactive");
    assert!(source_blocks::validate(&source, Some(&invocation)).is_err());
    assert!(semantic_contract::validate_physics_evidence(&source).is_err());
    for path in [
        "fixtures/results/physics_connected_mechanics_sparse.json",
        "fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json",
    ] {
        assert!(physics_source::validate(&read(path), None).is_err());
    }
}
#[test]
fn rehashed_false_statements_remain_invalid() {
    let (original, invocation) = actual("n05", "sparse_interactive");
    for mutation in [
        "maximum_basis",
        "maximum_action",
        "maximum_interval",
        "maximum_functional",
        "maximum_section",
        "maximum_identity",
        "maximum_location",
        "maximum_criterion",
        "stress_operand",
        "stress_recipe",
        "stress_section",
        "stress_row",
        "stress_missing",
        "norm_interval",
        "norm_functional",
        "norm_missing",
        "support_owner",
        "projection_bits",
        "source_member",
        "ordinary_quality",
        "foreign_namespace",
        "case_work",
        "invocation_work",
        "pressure_presence",
        "material_selection",
        "geometry_selection",
    ] {
        let mut source = original.clone();
        let mut context = invocation.clone();
        let maximum = "/contract_evidence/exact_cases/0/pipe_stress_extrema/0";
        let case = "/source_block_recovery/body/cases/0";
        let path = |suffix: &str| format!("{case}{suffix}");
        match mutation {
            "maximum_basis" => source
                .pointer_mut(&format!("{maximum}/basis"))
                .map(|v| *v = json!("recovered_from_open_mechanics_stress_components")),
            "maximum_action" => source
                .pointer_mut(&format!("{maximum}/endpoints/0/actions/0/value"))
                .map(|v| *v = json!(1.0)),
            "maximum_interval" => source
                .pointer_mut(&format!("{maximum}/value_upper_pa"))
                .map(|v| *v = json!(1.0)),
            "maximum_functional" => source
                .pointer_mut(&format!("{maximum}/endpoints/0/functional_indices/0"))
                .map(|v| *v = json!(v.as_u64().unwrap() + 1)),
            "maximum_section" => source
                .pointer_mut(&format!("{maximum}/area_m2"))
                .map(|v| *v = json!(v.as_f64().unwrap() * 2.0)),
            "maximum_identity" => source
                .pointer_mut(&format!("{maximum}/source_identity_sha256"))
                .map(|v| *v = json!("0".repeat(64))),
            "maximum_location" => source
                .pointer_mut(&format!("{maximum}/locations"))
                .map(|v| *v = json!({"kind":"strict_endpoint","endpoint":"i"})),
            "maximum_criterion" => source
                .pointer_mut(&format!("{maximum}/relative_limit"))
                .map(|v| *v = json!(1e-8)),
            "stress_operand" => source
                .pointer_mut(&path("/section_stress_checks/0/action/value"))
                .map(|v| *v = json!(1.0)),
            "stress_recipe" => source
                .pointer_mut(&path("/section_stress_checks/0/recipe_id"))
                .map(|v| *v = json!("straight_open_stress_v1")),
            "stress_section" => source
                .pointer_mut(&path(
                    "/section_stress_checks/0/parameters/torsion_constant_m4",
                ))
                .map(|v| *v = json!(v.as_f64().unwrap() * 2.0)),
            "stress_row" => {
                let id = source
                    .pointer(&path("/section_stress_checks/0/result_id"))
                    .unwrap()
                    .clone();
                source["results"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|r| r["id"] == id)
                    .map(|r| r["value"] = json!(1.0))
            }
            "stress_missing" => source
                .pointer_mut(&path("/section_stress_checks"))
                .map(|v| {
                    v.as_array_mut().unwrap().pop();
                }),
            "norm_interval" => source
                .pointer_mut(&path("/derived_checks/0/interval/1"))
                .map(|v| *v = json!(v.as_f64().unwrap() + 1.0)),
            "norm_functional" => source
                .pointer_mut(&path("/derived_checks/0/functional_indices/0"))
                .map(|v| *v = json!(v.as_u64().unwrap() + 1)),
            "norm_missing" => source.pointer_mut(&path("/derived_checks")).map(|v| {
                v.as_array_mut().unwrap().pop();
            }),
            "support_owner" => source
                .pointer_mut(&path("/supports/0/components/0/action_terms/0/source_id"))
                .map(|v| *v = json!("other")),
            "projection_bits" => source
                .pointer_mut(&path("/projections/0/value_bits"))
                .map(|v| *v = json!("8000000000000000")),
            "source_member" => source
                .pointer_mut(&path("/source/member_ids/0"))
                .map(|v| *v = json!("other")),
            "ordinary_quality" => source
                .pointer_mut("/numerical_quality/cases/0/solve_quality")
                .map(|v| *v = json!("checks_passed")),
            "foreign_namespace" => {
                source["carrier_evidence"] = json!({});
                Some(())
            }
            "case_work" => source
                .pointer_mut(&path("/work/limit"))
                .map(|v| *v = json!(8_000_001)),
            "invocation_work" => source
                .pointer_mut("/source_block_recovery/body/invocation_work/charged")
                .map(|v| *v = json!(v.as_u64().unwrap() + 1)),
            "pressure_presence" => context
                .pointer_mut("/request/model/load_cases/0/pressure_regions")
                .map(|v| *v = json!([{"p_pa":0}])),
            "material_selection" => context
                .pointer_mut("/request/model/materials/0/poisson_ratio/value")
                .map(|v| *v = json!(0.3)),
            "geometry_selection" => context
                .pointer_mut("/request/model/pipe_segments/0/section/wall_thickness/value")
                .map(|v| *v = json!(v.as_f64().unwrap() * 0.5)),
            _ => unreachable!(),
        }
        .expect("mutation addresses actual producer field");
        reseal_negative(&mut source, Some(&context));
        assert!(
            physics_source::validate(&source, Some(&context)).is_err(),
            "{mutation}"
        );
    }
}
#[test]
fn physical_transport_never_reconstructs_raw_rows() {
    for name in [
        "physics_connected_mechanics_sparse",
        "physics_thermal_ui_mechanics_sparse",
    ] {
        let source = read(&format!("fixtures/results/{name}.json"));
        assert_eq!(
            semantic_contract::validate_physics_transport_metadata(
                &json!({"contract_evidence":source["contract_evidence"]})
            ),
            Ok(())
        );
        let mut bad = source["contract_evidence"].clone();
        bad["exact_cases"][0]["pipe_stress_extrema"][0]["coefficient_basis"] = json!("invented");
        assert!(semantic_contract::validate_physics_transport_metadata(
            &json!({"contract_evidence":bad})
        )
        .is_err());
    }
}

#[test]
fn actual_units_authority_rejects_wrong_dimension_and_unregistered_spelling() {
    for (field, unit) in [
        ("elastic_modulus", "N"),
        ("elastic_modulus", "gpa"),
        ("shear_modulus", "K"),
        ("poisson_ratio", "none"),
        ("thermal_expansion_coefficient", "K"),
    ] {
        let (mut source, mut invocation) = actual("n05_units", "sparse_interactive");
        invocation["request"]["model"]["materials"][0][field] =
            json!({"value":if field=="poisson_ratio"{0.25}else{200.0},"unit":unit});
        reseal_negative(&mut source, Some(&invocation));
        assert!(
            physics_source::validate(&source, Some(&invocation)).is_err(),
            "{field}/{unit}"
        );
    }
}

#[test]
fn public_rehashed_copy_cannot_hide_an_invalid_interpolation_pair() {
    for bad_endpoint in [0usize, 1] {
        let (mut source, mut invocation) = actual("mixed_units", "sparse_interactive");
        invocation["request"]["model"]["materials"][0]["temperature_points"][bad_endpoint]
            ["elastic_modulus"] = json!({"value":f64::from_bits(1),"unit":"Pa"});
        invocation["request"]["model"]["materials"][0]["temperature_points"][1 - bad_endpoint]
            ["elastic_modulus"] = json!({"value":4000.0,"unit":"Pa"});
        // Negative false statement only: rounded interpolated E still equals 2000.
        reseal_negative(&mut source, Some(&invocation));
        let stage = if bad_endpoint == 0 {
            "lower_bracket"
        } else {
            "upper_bracket"
        };
        assert_eq!(
            physics_source::validate(&source, Some(&invocation)),
            Err(format!("PHYSICS_SOURCE_ACTUAL_DERIVED_SHEAR_RANGE:{stage}"))
        );
    }
}
