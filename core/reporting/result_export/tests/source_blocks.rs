//! Synthetic statements plus received producer artifacts. No fresh solve/native witness is performed here.
use open_pipe_stress_result_export::{semantic_contract, source_blocks};
use serde_json::{json, Value};
fn control() -> (Value, Value) {
    let v: Value = serde_json::from_str(include_str!(
        "fixtures/source_blocks_statement_control.json"
    ))
    .unwrap();
    let mut source = v["source"].clone();
    let context = v["context"].clone();
    let raw = source["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| (row["id"].as_str().unwrap().to_owned(), row.clone()))
        .collect::<std::collections::HashMap<_, _>>();
    for case in source["source_block_recovery"]["body"]["cases"]
        .as_array_mut()
        .unwrap()
    {
        for row in case["rows"].as_array_mut().unwrap() {
            match row["recipe_id"].as_str() {
                Some("translation_norm_v1") => {
                    row["recipe_id"] = json!("translation_norm_scaled_v1")
                }
                Some("support_force_norm_v1") => {
                    row["recipe_id"] = json!("support_force_norm_scaled_v1")
                }
                _ => (),
            }
            if row["recipe_id"] == "straight_open_stress_v1" {
                let output = &raw[row["result_id"].as_str().unwrap()];
                let action = match output["kind"].as_str().unwrap() {
                    "element_local_axial_normal_stress" => "element_local_axial_force",
                    "element_local_bending_normal_stress_y" => "element_local_bending_moment_y",
                    "element_local_bending_normal_stress_z" => "element_local_bending_moment_z",
                    "element_local_torsional_shear_stress" => "element_local_torsional_moment",
                    _ => unreachable!(),
                };
                row["input_result_ids"] = json!(row["input_result_ids"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|id| raw[id.as_str().unwrap()]["kind"] == action)
                    .collect::<Vec<_>>());
            }
        }
    }
    seal(&mut source, &context);
    (source, context)
}
fn seal(source: &mut Value, context: &Value) {
    source["source_block_recovery"]["body"]["invocation"]["value"] =
        json!(source_blocks::domain_hash("source_blocks_invocation_v1", context).unwrap());
    let mut publication = source.clone();
    publication
        .as_object_mut()
        .unwrap()
        .remove("source_block_recovery");
    source["source_block_recovery"]["body"]["publication_sha256"] =
        json!(source_blocks::domain_hash("source_blocks_publication_v1", &publication).unwrap());
    source["source_block_recovery"]["receipt_sha256"] = json!(source_blocks::domain_hash(
        "source_blocks_receipt_v1",
        &source["source_block_recovery"]["body"]
    )
    .unwrap());
}
#[test]
fn independent_actual_context_is_required_and_bound() {
    let (source, context) = control();
    let basis = vec![json!({"ref_type":"load_case","ref_id":"synthetic-case"})];
    assert_eq!(source_blocks::validate(&source, None), Ok(false));
    assert_eq!(
        semantic_contract::numerical_use_standing(&source, &basis),
        "needs_recompute"
    );
    assert_eq!(source_blocks::validate(&source, Some(&context)), Ok(true));
    assert_eq!(
        semantic_contract::numerical_use_standing_with_context(&source, &basis, Some(&context)),
        "numerically_eligible"
    );
    let mut changed = context.clone();
    changed["request"]["unknown_but_captured"] = Value::Null;
    assert!(source_blocks::validate(&source, Some(&changed)).is_err());
}
#[test]
fn receipt_presence_never_downgrades_to_precision_or_legacy() {
    let (source, _) = control();
    for value in [
        Value::Null,
        json!(false),
        json!(0),
        json!(""),
        json!({}),
        json!([]),
    ] {
        for contract in [
            semantic_contract::PRECISION_ID,
            "openpipestress.result_semantics/0.3.0/physics-1",
        ] {
            let mut altered = source.clone();
            altered["producer"]["semantic_contract_id"] = json!(contract);
            altered["source_block_recovery"] = value.clone();
            assert!(semantic_contract::for_source(&altered).is_err());
        }
    }
}
#[test]
fn rehashed_projection_work_and_attribution_mutations_reject() {
    let (source, context) = control();
    for (pointer, value) in [
        (
            "/source_block_recovery/body/cases/0/projections/0/value_bits",
            json!("8000000000000000"),
        ),
        (
            "/source_block_recovery/body/cases/0/projections/0/relative_limit",
            json!(1e-8),
        ),
        (
            "/source_block_recovery/body/cases/0/work/charged",
            json!(2000001),
        ),
        (
            "/source_block_recovery/body/cases/0/work/rejected_reservation/amount",
            Value::Null,
        ),
        (
            "/source_block_recovery/body/cases/0/requested_mode",
            json!("sparse_interactive"),
        ),
        (
            "/source_block_recovery/body/cases/0/supports/0/components/0/action_terms/0/source_id",
            json!("unowned"),
        ),
        ("/summary/max_open_formula_stress/value", json!(1.0)),
    ] {
        let mut changed = source.clone();
        *changed.pointer_mut(pointer).unwrap() = value;
        seal(&mut changed, &context);
        assert!(
            source_blocks::validate(&changed, Some(&context)).is_err(),
            "{pointer}"
        );
    }
}
#[test]
fn additive_signatures_leave_precision_table_unchanged() {
    let old = semantic_contract::precision_contract();
    let new = semantic_contract::source_blocks_contract();
    assert_eq!(
        &new["rows"].as_array().unwrap()[..60],
        old["rows"].as_array().unwrap()
    );
    assert_eq!(new["source_signature_count"], 66);
    for row in &new["rows"].as_array().unwrap()[60..] {
        assert_eq!(row["family"], "reaction");
        assert_eq!(row["kind"], "support_reaction_component_v2");
    }
}

fn received_artifacts() -> Vec<(String, Value, Value)> {
    // These frozen producer request/raw pairs are independent received inputs.
    // Do not reseal or regenerate the raw artifact in a positive test.
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/product_preview/source_blocks");
    let mut received = Vec::new();
    for case in ["n05", "n06", "multicase"] {
        for mode in ["dense_scrutiny", "sparse_interactive"] {
            let name = format!("{case}-{mode}");
            let request_text =
                std::fs::read_to_string(directory.join(format!("{name}.request.json"))).unwrap();
            let raw_text =
                std::fs::read_to_string(directory.join(format!("{name}.raw.json"))).unwrap();
            // Parse the received raw JSON directly; canonicalization would erase -0.
            let request: Value = serde_json::from_str(&request_text).unwrap();
            let source: Value = serde_json::from_str(&raw_text).unwrap();
            let context = json!({"request":request,"solver_mode":mode});
            received.push((name, source, context));
        }
    }
    received
}

#[test]
fn received_real_producer_artifacts_qualify_only_against_actual_invocation() {
    let mut negative_zero_rows = 0;
    for (name, source, context) in received_artifacts() {
        let bases = context["request"]["model"]["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| json!({"ref_type":"load_case","ref_id":case["id"]}))
            .collect::<Vec<_>>();
        assert_eq!(
            source_blocks::validate(&source, Some(&context)),
            Ok(true),
            "{name}"
        );
        assert_eq!(
            semantic_contract::numerical_use_standing_with_context(&source, &bases, Some(&context)),
            "numerically_eligible",
            "{name}"
        );
        assert_eq!(
            semantic_contract::numerical_use_standing(&source, &bases),
            "needs_recompute",
            "{name}"
        );
        let mut changed = context.clone();
        changed["request"]["received_context_mutation"] = Value::Null;
        assert!(
            source_blocks::validate(&source, Some(&changed)).is_err(),
            "{name}: changed request"
        );
        let mut changed = context.clone();
        changed["solver_mode"] = json!(if context["solver_mode"] == "dense_scrutiny" {
            "sparse_interactive"
        } else {
            "dense_scrutiny"
        });
        assert!(
            source_blocks::validate(&source, Some(&changed)).is_err(),
            "{name}: changed mode"
        );
        negative_zero_rows += source["results"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| {
                row["value"]
                    .as_f64()
                    .is_some_and(|value| value == 0.0 && value.is_sign_negative())
            })
            .count();
    }
    assert!(
        negative_zero_rows > 0,
        "received signed zero must survive direct raw JSON parsing"
    );
}

#[test]
fn received_invocation_work_aggregates_all_cases_and_rejects_rehashed_tampering() {
    for (name, source, context) in received_artifacts() {
        let body = &source["source_block_recovery"]["body"];
        let cases = body["cases"].as_array().unwrap();
        let case_charged = cases
            .iter()
            .map(|case| {
                case["work"]["charged"].as_u64().unwrap()
                    + case["work"]["reserved_unobserved_failure"]
                        .as_u64()
                        .unwrap()
            })
            .sum::<u64>();
        let work = &body["invocation_work"];
        assert_eq!(
            work["charged"].as_u64().unwrap(),
            case_charged + work["publication_charged"].as_u64().unwrap(),
            "{name}"
        );
        assert!(work["publication_charged"].as_u64().unwrap() > 0, "{name}");
        assert!(cases
            .iter()
            .all(|case| case["work"]["limit"].as_u64().unwrap() <= 4_000_000));
        assert!(
            work["charged"].as_u64().unwrap() <= work["limit"].as_u64().unwrap()
                && work["limit"].as_u64().unwrap() <= 64_000_000
        );
        for (pointer, value) in [
            (
                "/source_block_recovery/body/invocation_work/charged",
                json!(work["charged"].as_u64().unwrap() + 1),
            ),
            (
                "/source_block_recovery/body/invocation_work/publication_charged",
                json!(work["publication_charged"].as_u64().unwrap() + 1),
            ),
            (
                "/source_block_recovery/body/invocation_work/limit",
                json!(64_000_001),
            ),
            (
                "/source_block_recovery/body/cases/0/work/limit",
                json!(4_000_001),
            ),
        ] {
            let mut changed = source.clone();
            *changed.pointer_mut(pointer).unwrap() = value;
            // Negative test only: recomputing checksums must not mask a bad ledger.
            seal(&mut changed, &context);
            assert!(
                source_blocks::validate(&changed, Some(&context)).is_err(),
                "{name}: {pointer}"
            );
        }
    }
}

#[test]
fn stress_containing_case_reserves_both_recipe_layers() {
    let (mut source, context) = control();
    let case = &source["source_block_recovery"]["body"]["cases"][0];
    let norm = case["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["recipe_id"] == "translation_norm_scaled_v1")
        .unwrap();
    let norm_id = norm["result_id"].as_str().unwrap().to_owned();
    let x_id = norm["input_result_ids"][0].as_str().unwrap().to_owned();
    let relative = (1e-9 - 128.0 * f64::EPSILON) / (1.0 + 128.0 * f64::EPSILON);
    for row in source["results"].as_array_mut().unwrap() {
        if row["id"] == x_id || row["id"] == norm_id {
            row["value"] = json!(1.0);
        }
    }
    let projection = source["source_block_recovery"]["body"]["cases"][0]["projections"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["result_id"] == x_id)
        .unwrap();
    projection["value"] = json!(1.0);
    projection["value_bits"] = json!("3ff0000000000000");
    projection["interval"] = json!([1.0 - 1e-12, 1.0 + 1e-12]);
    projection["absolute_error_bound"] = json!(1.1e-12);
    projection["relative_error_bound"] = json!(relative);
    projection["basis"] = json!("outward_interval");
    source["summary"]["max_displacement"]["value"] = json!(1.0);
    source["summary"]["max_displacement"]["result_ref"] = json!(norm_id);
    seal(&mut source, &context);
    assert_eq!(source_blocks::validate(&source, Some(&context)), Ok(true));
    let projection = source["source_block_recovery"]["body"]["cases"][0]["projections"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["result_id"] == x_id)
        .unwrap();
    projection["relative_error_bound"] = json!(f64::from_bits(relative.to_bits() + 1));
    seal(&mut source, &context);
    assert_eq!(
        source_blocks::validate(&source, Some(&context)),
        Err("SOURCE_BLOCKS_STRESS_TOTAL_RELATIVE_BOUND".into())
    );
}

#[test]
fn reserved_unobserved_failure_is_debited_in_the_invocation_ledger() {
    let (mut source, context) = control();
    let case = &mut source["source_block_recovery"]["body"]["cases"][0];
    case["outcome"] = json!("failed");
    case["selected_method"] = Value::Null;
    case["source"] = Value::Null;
    case["projections"] = json!([]);
    case["supports"] = json!([]);
    case["failure"] = json!({"stage":"exact_solve","code":"budget","diagnostic_ref":"synthetic-report","block_order":null});
    for row in case["rows"].as_array_mut().unwrap() {
        row["treatment"] = json!("inspection_only");
        row["projection_id"] = Value::Null;
        row["recipe_id"] = Value::Null;
        row["input_result_ids"] = json!([]);
    }
    case["work"]["reserved_unobserved_failure"] = json!(7);
    source["source_block_recovery"]["body"]["status"] = json!("unavailable");
    let ledger = &mut source["source_block_recovery"]["body"]["invocation_work"];
    ledger["charged"] = json!(ledger["charged"].as_u64().unwrap() + 7);
    seal(&mut source, &context);
    assert_eq!(source_blocks::validate(&source, Some(&context)), Ok(false));
    let ledger = &mut source["source_block_recovery"]["body"]["invocation_work"];
    ledger["charged"] = json!(ledger["charged"].as_u64().unwrap() - 7);
    seal(&mut source, &context);
    assert_eq!(
        source_blocks::validate(&source, Some(&context)),
        Err("SOURCE_BLOCKS_INVOCATION_WORK_LEDGER".into())
    );
}

#[test]
fn rehashed_received_stress_underflow_and_subnormal_rows_are_refused() {
    let (_, source, context) = received_artifacts().into_iter().next().unwrap();
    let index = source["results"]
        .as_array()
        .unwrap()
        .iter()
        .position(|row| {
            row["kind"] == "element_local_torsional_shear_stress"
                && row["value"].as_f64().unwrap() != 0.0
        })
        .unwrap();
    for value in [0.0, 1.675317e-318] {
        let mut changed = source.clone();
        changed["results"][index]["value"] = json!(value);
        seal(&mut changed, &context);
        assert!(matches!(source_blocks::validate(&changed, Some(&context)),
            Err(error) if error=="SOURCE_BLOCKS_STRESS_OUTPUT_RANGE" || error=="SOURCE_BLOCKS_SUMMARY_INPUT_RANGE"));
    }
    let mut changed = source.clone();
    let relative = (1e-9 - 128.0 * f64::EPSILON) / (1.0 + 128.0 * f64::EPSILON);
    let projection = changed["source_block_recovery"]["body"]["cases"][0]["projections"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["basis"] == "outward_interval")
        .unwrap();
    projection["relative_error_bound"] = json!(f64::from_bits(relative.to_bits() + 1));
    seal(&mut changed, &context);
    assert_eq!(
        source_blocks::validate(&changed, Some(&context)),
        Err("SOURCE_BLOCKS_STRESS_TOTAL_RELATIVE_BOUND".into())
    );
}

#[test]
fn actual_pre_repair_producer_stress_range_packets_are_refused_unchanged() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/product_preview/source_blocks/rejected_stress_range");
    for mode in ["dense_scrutiny", "sparse_interactive"] {
        let raw = std::fs::read_to_string(directory.join(format!("{mode}.raw.json"))).unwrap();
        let request =
            std::fs::read_to_string(directory.join(format!("{mode}.request.json"))).unwrap();
        let source: Value = serde_json::from_str(&raw).unwrap();
        let request: Value = serde_json::from_str(&request).unwrap();
        let context = json!({"request":request,"solver_mode":mode});
        assert_eq!(
            source["source_block_recovery"]["body"]["status"],
            "qualified"
        );
        // Actual received producer defect, not a synthesized or resealed packet.
        assert!(
            matches!(source_blocks::validate(&source,Some(&context)),
            Err(error) if error=="SOURCE_BLOCKS_STRESS_OUTPUT_RANGE" || error=="SOURCE_BLOCKS_SUMMARY_INPUT_RANGE"),
            "{mode}"
        );
        let bases = context["request"]["model"]["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| json!({"ref_type":"load_case","ref_id":case["id"]}))
            .collect::<Vec<_>>();
        assert_ne!(
            semantic_contract::numerical_use_standing_with_context(&source, &bases, Some(&context)),
            "numerically_eligible"
        );
    }
}
