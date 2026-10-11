//! J1 preparation: the captured source context and its publication check.
//! Full v3 publication/readback is exercised after shared caller/composite fan-in.
use super::*;

fn request(load_state: bool, v3: bool) -> Value {
    let mut raw: Value = serde_json::from_str(if load_state {
        include_str!("../../../../fixtures/product_preview/load_reference_source/n05.request.json")
    } else {
        include_str!("../../../../fixtures/product_preview/physics_source/n05.request.json")
    })
    .unwrap();
    if v3 {
        raw["model"]["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3"});
    }
    raw
}

#[test]
fn captured_composite_context_keeps_v2_names_and_unifies_v3_model_versions() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for (load_state, v3, id, policy, domain, profile) in [
            (
                false,
                false,
                COMPOSITE_SEMANTIC_ID,
                "PHYSICS-SOURCE-1",
                "physics_source_case_evidence_v1",
                "exact_straight_pressure_v2",
            ),
            (
                true,
                false,
                LOAD_REFERENCE_SOURCE_SEMANTIC_ID,
                "LOAD-REFERENCE-SOURCE-1",
                "load_reference_source_case_evidence_v1",
                "resolved_straight_load_state_source_v1",
            ),
            (
                false,
                true,
                "openpipestress.result_semantics/0.3.0/pressure-source-1",
                "PRESSURE-SOURCE-1",
                "pressure_source_case_evidence_v1",
                "exact_pressure_v3",
            ),
            (
                true,
                true,
                "openpipestress.result_semantics/0.3.0/pressure-source-1",
                "PRESSURE-SOURCE-1",
                "pressure_source_case_evidence_v1",
                "exact_pressure_v3",
            ),
        ] {
            let (parsed, captured) =
                CapturedInvocation::parse(request(load_state, v3), mode).unwrap();
            let context = captured.composite_context().unwrap();
            assert_eq!(
                context,
                CompositeSourceContext::for_model(&parsed.model).unwrap()
            );
            assert_eq!(
                (
                    context.semantic_id(),
                    context.policy_name(),
                    context.case_evidence_domain(),
                    context.profile_id()
                ),
                (id, policy, domain, profile)
            );
            assert_eq!(
                context.profile_mode(),
                if v3 {
                    "exact_pressure_v3"
                } else {
                    "exact_straight_pressure_v2"
                }
            );
        }
    }
}

#[test]
fn publication_label_is_checked_against_the_captured_policy() {
    let v2 = CapturedInvocation::parse(request(false, false), PreviewSolverMode::SparseInteractive)
        .unwrap()
        .1;
    let v3 = CapturedInvocation::parse(request(false, true), PreviewSolverMode::SparseInteractive)
        .unwrap()
        .1;
    for capture in [&v2, &v3] {
        let context = capture.composite_context().unwrap();
        let good = json!({"producer":{"semantic_contract_id":context.semantic_id()}});
        assert_eq!(
            validate_publication_identity(&good, context.receipt_policy()),
            Ok(())
        );
        let other = if context == CompositeSourceContext::PressureSource {
            COMPOSITE_SEMANTIC_ID
        } else {
            exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID
        };
        let bad_label = json!({"producer":{"semantic_contract_id":other}});
        assert_eq!(
            validate_publication_identity(&bad_label, context.receipt_policy()),
            Err(bad(
                "publication must have source identity and omit receipt"
            ))
        );
        let with_receipt = json!({"producer":{"semantic_contract_id":context.semantic_id()},"source_block_recovery":null});
        assert!(validate_publication_identity(&with_receipt, context.receipt_policy()).is_err());
    }
}

#[test]
fn captured_context_does_not_infer_a_contract_from_a_publication_claim() {
    let mut raw = request(false, true);
    raw["producer"] = json!({"semantic_contract_id":COMPOSITE_SEMANTIC_ID});
    let (_, captured) = CapturedInvocation::parse(raw, PreviewSolverMode::DenseScrutiny).unwrap();
    assert_eq!(
        captured.composite_context().unwrap(),
        CompositeSourceContext::PressureSource
    );
    for contract in [
        Value::Null,
        json!({"version":"3.0.0","mode":"exact_straight_pressure_v2"}),
        json!({"version":"2.0.0","mode":"exact_pressure_v3"}),
    ] {
        let mut raw = request(false, true);
        raw["model"]["pressure_contract"] = contract;
        let (_, captured) =
            CapturedInvocation::parse(raw, PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(
            captured.composite_context(),
            Err(bad("composite source requires an actual exact contract"))
        );
    }
    let mut preview = request(false, false);
    preview["model"]["schema_version"] = json!("0.1.0");
    preview["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    let (_, captured) =
        CapturedInvocation::parse(preview, PreviewSolverMode::DenseScrutiny).unwrap();
    assert!(captured.composite_context().is_err());
    // The ordinary preview receipt remains its separate policy.
    assert_eq!(ReceiptPolicy::SourceBlocks.semantic_id(), SEMANTIC_ID);
}

#[test]
fn pressure_case_context_uses_one_domain_without_changing_its_payload_shape() {
    for load_state in [false, true] {
        let (_, captured) = CapturedInvocation::parse(
            request(load_state, true),
            PreviewSolverMode::SparseInteractive,
        )
        .unwrap();
        let context = captured.composite_context().unwrap();
        let proof = if load_state {
            json!({"exact_case":{"profile_mode":"exact_pressure_v3"},"pressure":[],"load_reference_state":{"load_case_id":"case"}})
        } else {
            json!({"exact_case":{"profile_mode":"exact_pressure_v3"},"pressure":[]})
        };
        let independently_named =
            checked(&json!({"domain":"pressure_source_case_evidence_v1","payload":proof})).unwrap();
        assert_eq!(
            hash(context.case_evidence_domain(), &proof).unwrap(),
            format!("{:x}", Sha256::digest(independently_named.as_bytes()))
        );
        assert_ne!(
            hash(context.case_evidence_domain(), &proof).unwrap(),
            hash("physics_source_case_evidence_v1", &proof).unwrap()
        );
    }
}

#[test]
fn formulation_helpers_preserve_held_callers_and_name_the_actual_publication() {
    for (text, connectors) in [
        (
            include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"),
            false,
        ),
        (
            include_str!("../../../../fixtures/results/pressure_v3_connector_reader_corpus.json"),
            true,
        ),
    ] {
        let corpus: Value = serde_json::from_str(text).unwrap();
        for case in corpus["cases"].as_array().unwrap() {
            let load_state = case["document_schema_version"] == "0.4.0";
            let held = exact_admission::pressure_v3_formulation_basis(load_state, connectors);
            assert_eq!(
                serialized(&held).unwrap(),
                case["envelope"]["formulation_basis"]
            );
            for selected in [false, true] {
                let joined = exact_admission::pressure_v3_formulation_basis_with_source(
                    load_state, connectors, selected,
                );
                assert_eq!(joined.profile_id, held.profile_id);
                assert_eq!(joined.limitations.len(), held.limitations.len());
                assert_eq!(&joined.limitations[1..7], &held.limitations[1..7]);
                assert_eq!(&joined.limitations[8..], &held.limitations[8..]);
                assert!(!joined.limitations[7].contains("not joined"));
                if selected {
                    assert!(joined.limitations[0].contains("under pressure-source-1 semantics"));
                    assert!(joined.limitations[7].contains("selected retained-source response"));
                } else {
                    assert_eq!(joined.limitations[0], held.limitations[0]);
                    assert!(
                        joined.limitations[7].contains("publishes ordinary structural recovery")
                    );
                }
            }
        }
    }
}
