//! B4/D1 (b): the non-composite source-blocks-1 identity is retired. A stored
//! source-blocks-1 statement is refused by name; the composite identities keep
//! the shared source-block machinery and their ordinary cases (B7b: the
//! current pressure-source-1 statement on the 0.9.0/v3 corpus).
use open_pipe_stress_result_export::{reader_verdict, semantic_contract};
use serde_json::{json, Value};

fn read(file: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/product_preview/pressure_source/physics_source")
        .join(file);
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn a_stored_source_blocks_1_statement_is_refused_by_name() {
    for mode in ["dense_scrutiny", "sparse_interactive"] {
        let mut stored = read(&format!("n05-{mode}.raw.json"));
        stored["producer"]["semantic_contract_id"] = json!(semantic_contract::RETIRED_SOURCE_BLOCKS_ID);
        stored["formulation_basis"]["profile_id"] = json!("product_preview_mechanics_v1");
        stored.as_object_mut().unwrap().remove("contract_evidence");
        let refusal = semantic_contract::for_source(&stored).unwrap_err();
        assert!(refusal.starts_with(semantic_contract::SOURCE_BLOCKS_1_RETIRED), "{mode}: {refusal}");
        assert!(semantic_contract::for_source_metadata(&stored).unwrap_err().starts_with(semantic_contract::SOURCE_BLOCKS_1_RETIRED));
        let dispatched = reader_verdict::dispatch(&stored, None, None);
        assert_eq!(dispatched["route"], "unsupported", "{mode}");
        assert_eq!(dispatched["error"]["code"], semantic_contract::SOURCE_BLOCKS_1_RETIRED, "{mode}");
        assert_eq!(semantic_contract::numerical_use_standing(&stored, &[]), "unsupported");
        assert!(!reader_verdict::ROUTES.contains(&"source_blocks"));
        assert!(!reader_verdict::READERS.contains(&"source-blocks"));
    }
}

#[test]
fn pressure_source_ordinary_cases_stay_admitted() {
    for mode in ["dense_scrutiny", "sparse_interactive"] {
        let source = read(&format!("mixed-{mode}.raw.json"));
        let request = read("mixed.request.json");
        assert!(source["source_block_recovery"]["body"]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["outcome"] == "qualified"
                && c["selected_method"] != "retained_source_blocks_exact_v1"));
        assert_eq!(semantic_contract::standing_reason(&source), None);
        let bases = request["model"]["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| json!({"ref_type":"load_case","ref_id":case["id"]}))
            .collect::<Vec<_>>();
        let context = json!({"request":request,"solver_mode":mode});
        assert_eq!(
            semantic_contract::numerical_use_standing_with_context(&source, &bases, Some(&context)),
            "numerically_eligible",
            "{mode}"
        );
        for row in source["results"].as_array().unwrap() {
            assert_eq!(semantic_contract::rule_binding_refusal(&source, row), None, "{mode}");
        }
    }
}
