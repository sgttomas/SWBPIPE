//! b7-plan R2: the base-route verdicts for callers outside Rust. Cases taken
//! from the desktop's reader tests (numericalResultQuality, loadReferenceReaders,
//! sourceBlockRecovery, physicsSourceRecovery, previewPhysicsEvidence), so the
//! G1, G2 and G5 ports keep the behaviour those tests pinned. B7b re-pointed the
//! exact-route cases at the current identities (pressure-2 and pressure-source-1,
//! with and without the load/reference-state group) and pins the named refusal of
//! the retired v2 identities.
use open_pipe_stress_result_export::reader_verdict as rv;
use open_pipe_stress_result_export::semantic_contract as sc;
use serde_json::{json, Value};

fn read(path: &str) -> Value {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../");
    let text = std::fs::read_to_string(format!("{root}{path}")).unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&text).unwrap()
}
fn findings(standing: &Value) -> Vec<&str> {
    standing["findings"].as_array().unwrap().iter().map(|f| f.as_str().unwrap()).collect()
}
fn route(source: &Value) -> &'static str {
    rv::route(source).map_or(rv::UNSUPPORTED, |r| r.0)
}

// numericalResultQuality.test.ts: the synthetic ordinary source and model. B1
// deleted precision-1, the route these generic checks were first pinned on; the
// statement now carries the preview-physics-2 identity, its profile limitations
// and one empty preview case per numerical case (`with_preview_cases`), which the
// preview reader admits, so the ordinary checks are the subject.
fn ordinary_model() -> Value {
    json!({"load_cases": [{"id": "case:a"}, {"id": "case:b"}]})
}
fn ordinary_source() -> Value {
    let cases: Vec<Value> = ["case:a", "case:b"]
        .iter()
        .map(|id| json!({"basis_ref": {"ref_type": "load_case", "ref_id": id}, "structural_status": "passive_model_basis",
            "solve_quality": "checks_passed", "model_matrix_fidelity": "represented_equations_retained",
            "accuracy_evidence": "not_claimed", "evidence_refs": ["r"]}))
        .collect();
    let mut raw = json!({"schema_version": "0.2.0", "document_kind": "MechanicsResult", "run_id": "test", "model_ref": "test",
        "summary": {"max_displacement": null, "max_open_formula_stress": null},
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE", "professional_acceptance": "NOT_PROVIDED"},
        "results": [{"id": "r", "kind": "global_nodal_rotation_x", "value": -1e-12, "unit": "rad", "entity_ref": "n"}], "diagnostics": [],
        "producer": {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": sc::PREVIEW_PHYSICS_ID},
        "contract_evidence": {"preview_cases": [], "combination_gates": []},
        "formulation_basis": {"profile_id": "product_preview_mechanics_v1",
            "limitations": sc::preview_physics_contract()["supported_profile_limitations"].clone()},
        "numerical_quality": {"value_representation": "finite_binary64", "publication_quantization": "none",
            "integrity_policy": "M03-INTEGRITY-v1", "status": "checks_passed", "cases": cases}});
    with_preview_cases(&mut raw);
    raw
}
/// One empty, complete preview case per numerical case, in its order.
fn with_preview_cases(raw: &mut Value) {
    let cases: Vec<Value> = raw["numerical_quality"]["cases"].as_array().unwrap().iter()
        .map(|c| json!({"load_case_id": c["basis_ref"]["ref_id"], "pipe_stress_extrema": [],
            "stress_maximum_coverage": {"complete": true, "unavailable_pipe_ids": [], "outside_domain_pipe_ids": []},
            "support_attribution": {"attributed_support_ids": [], "withheld": []}, "intensified_measures": []}))
        .collect();
    raw["contract_evidence"]["preview_cases"] = Value::from(cases);
}
fn standing(source: &Value, model: Option<&Value>) -> Value {
    rv::standing(source, model, None)
}

#[test]
fn g1_ordinary_checks_require_complete_requested_cases_and_emitted_ids() {
    let model = ordinary_model();
    let mut raw = ordinary_source();
    assert_eq!(route(&raw), "preview_physics");
    assert_eq!(findings(&standing(&raw, Some(&model))), Vec::<&str>::new());
    assert!(findings(&standing(&raw, None)).contains(&"REQUESTED_NUMERICAL_BASIS_UNAVAILABLE"));
    raw["numerical_quality"]["cases"].as_array_mut().unwrap().pop();
    assert!(findings(&standing(&raw, Some(&model))).contains(&"NUMERICAL_CASE_COVERAGE_INCOMPLETE"));
    raw["numerical_quality"]["cases"] = ordinary_source()["numerical_quality"]["cases"].clone();
    raw["numerical_quality"]["cases"][0]["evidence_refs"] = json!(["invented"]);
    assert!(findings(&standing(&raw, Some(&model))).contains(&"NUMERICAL_CASE_EVIDENCE_INCOMPLETE"));
}

#[test]
fn g1_retired_unassessed_unknown_and_sensitive_are_never_eligible() {
    let model = ordinary_model();
    let mut legacy = ordinary_source();
    legacy["schema_version"] = json!("0.1.0");
    for k in ["producer", "numerical_quality", "formulation_basis", "contract_evidence"] {
        legacy.as_object_mut().unwrap().remove(k);
    }
    // b7-plan E2: the retired producer-less carrier is refused by name.
    assert_eq!(route(&legacy), rv::UNSUPPORTED);
    assert_eq!(rv::dispatch(&legacy, None, None)["error"]["code"], sc::MECHANICS_RESULT_VERSION_RETIRED);
    let s = standing(&legacy, Some(&model));
    assert_eq!((s["status"].as_str(), findings(&s)), (Some("needs_recompute"), vec![sc::MECHANICS_RESULT_VERSION_RETIRED]));
    for status in ["not_assessed", "unresolved", "failed"] {
        let mut c = ordinary_source();
        c["numerical_quality"]["status"] = json!(status);
        assert_eq!(standing(&c, Some(&model))["eligible"], false);
    }
    let mut future = ordinary_source();
    future["producer"]["semantic_contract_id"] = json!(sc::PREVIEW_PHYSICS_ID.replace("preview-physics-2", "pressure-2"));
    assert_eq!(route(&future), rv::UNSUPPORTED);
    assert_eq!(findings(&standing(&future, Some(&model))), [rv::SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED]);
    future.as_object_mut().unwrap().remove("producer");
    assert_eq!(route(&future), rv::UNSUPPORTED);
    let mut sensitive = ordinary_source();
    sensitive["numerical_quality"]["cases"][0]["solve_quality"] = json!("sensitive");
    sensitive["numerical_quality"]["status"] = json!("sensitive");
    sensitive["numerical_quality"]["cases"][0]["accuracy_evidence"] = json!("reference_verified");
    assert_eq!(route(&sensitive), "preview_physics");
    assert_eq!(standing(&sensitive, Some(&model))["eligible"], false);
}

#[test]
fn g1_ambiguous_or_missing_emitted_ids() {
    let model = ordinary_model();
    let mut raw = ordinary_source();
    raw["diagnostics"] = json!([{"id": "r", "code": "test", "severity": "info", "message": "duplicate"}]);
    assert!(findings(&standing(&raw, Some(&model))).contains(&"NUMERICAL_EVIDENCE_ID_AMBIGUOUS"));
}

#[test]
fn g5_present_falsy_precision_fields_on_retired_are_unsupported() {
    let mut clean = ordinary_source();
    clean["schema_version"] = json!("0.1.0");
    for k in ["producer", "numerical_quality", "formulation_basis", "contract_evidence"] {
        clean.as_object_mut().unwrap().remove(k);
    }
    for key in ["producer", "numerical_quality", "formulation_basis"] {
        for value in [json!(null), json!(false), json!(0), json!("")] {
            let mut c = clean.clone();
            c[key] = value;
            assert_eq!(route(&c), rv::UNSUPPORTED, "{key}");
            assert_eq!(standing(&c, Some(&ordinary_model()))["eligible"], false);
        }
    }
}

#[test]
fn g1_general_basis_shapes_do_not_grant_coverage() {
    let model = ordinary_model();
    let mut raw = ordinary_source();
    raw["numerical_quality"]["cases"][0]["basis_ref"]["ref_type"] = json!("custom-basis");
    assert_eq!(route(&raw), "preview_physics");
    assert!(findings(&standing(&raw, Some(&model))).contains(&"NUMERICAL_CASE_COVERAGE_INCOMPLETE"));
    for key in ["ref_type", "ref_id"] {
        for value in [json!(""), json!(null), json!(false), json!(0), json!({}), json!([])] {
            let mut bad = ordinary_source();
            bad["numerical_quality"]["cases"][0]["basis_ref"][key] = value;
            assert_eq!(route(&bad), rv::UNSUPPORTED);
        }
    }
}

// numericalResultQuality.test.ts `aggregateFixture`.
fn aggregate_fixture(statuses: &[&str], claimed: &str) -> (Value, Value) {
    let mut raw = ordinary_source();
    let template = raw["numerical_quality"]["cases"][0].clone();
    raw["numerical_quality"]["status"] = json!(claimed);
    let mut cases = Vec::new();
    for (i, q) in statuses.iter().enumerate() {
        let mut c = template.clone();
        c["basis_ref"] = json!({"ref_type": "load_case", "ref_id": format!("case:{i}")});
        c["solve_quality"] = json!(q);
        match *q {
            "not_assessed" => {
                c["structural_status"] = json!("numerically_unresolved");
                c["model_matrix_fidelity"] = json!("not_assessed");
                c["accuracy_evidence"] = json!("not_claimed");
                c["evidence_refs"] = json!([]);
            }
            "unresolved" | "failed" => {
                let id = format!("diagnostic:case:{i}");
                c["structural_status"] = json!("numerically_unresolved");
                c["model_matrix_fidelity"] = json!("assembly_uncertainty");
                c["accuracy_evidence"] = json!("unresolved");
                c["evidence_refs"] = json!([id]);
                raw["diagnostics"].as_array_mut().unwrap().push(
                    json!({"id": id, "code": "consumer-control", "severity": "info", "message": "Nonpassing case evidence control"}),
                );
            }
            _ => {}
        }
        cases.push(c);
    }
    raw["numerical_quality"]["cases"] = Value::from(cases);
    with_preview_cases(&mut raw);
    let requested = json!({"load_cases": (0..statuses.len()).map(|i| json!({"id": format!("case:{i}")})).collect::<Vec<_>>()});
    (raw, requested)
}

#[test]
fn g1_aggregate_consistency_is_distinct_from_qualification() {
    let honest: [(&[&str], &str); 7] = [
        (&["sensitive", "checks_passed"], "sensitive"),
        (&["not_assessed"], "not_assessed"),
        (&["unresolved"], "unresolved"),
        (&["failed"], "failed"),
        (&["checks_passed", "sensitive", "not_assessed"], "not_assessed"),
        (&["unresolved", "sensitive", "not_assessed"], "unresolved"),
        (&["unresolved", "failed", "sensitive"], "failed"),
    ];
    for (cases, aggregate) in honest {
        let (raw, requested) = aggregate_fixture(cases, aggregate);
        let s = standing(&raw, Some(&requested));
        assert_eq!(s["route"], "preview_physics");
        assert_eq!(s["status"], "needs_recompute");
        assert_eq!(
            findings(&s),
            ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"],
            "{cases:?}"
        );
    }
    let (mut raw, requested) = aggregate_fixture(&["unresolved"], "unresolved");
    raw["numerical_quality"]["cases"][0]["model_matrix_fidelity"] = json!("not_assessed");
    assert_eq!(
        findings(&standing(&raw, Some(&requested))),
        ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"]
    );
    let mismatched: [(&[&str], &str); 6] = [
        (&["unresolved", "sensitive"], "sensitive"),
        (&["failed", "unresolved"], "unresolved"),
        (&["not_assessed"], "checks_passed"),
        (&["checks_passed"], "failed"),
        (&["unresolved"], "failed"),
        (&["checks_passed"], "sensitive"),
    ];
    for (cases, claim) in mismatched {
        let (raw, requested) = aggregate_fixture(cases, claim);
        let s = standing(&raw, Some(&requested));
        assert_eq!(s["eligible"], false);
        assert!(findings(&s).contains(&"NUMERICAL_AGGREGATE_CONTRADICTION"), "{cases:?} {claim}");
    }
    let (raw, requested) = aggregate_fixture(&["checks_passed", "checks_passed"], "checks_passed");
    assert_eq!(findings(&standing(&raw, Some(&requested))), Vec::<&str>::new());
    let (mut raw, _) = aggregate_fixture(&[], "not_assessed");
    let model = ordinary_model();
    assert_eq!(
        findings(&standing(&raw, Some(&model))),
        ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_COVERAGE_INCOMPLETE"]
    );
    raw["numerical_quality"]["status"] = json!("checks_passed");
    assert_eq!(
        findings(&standing(&raw, Some(&model))),
        ["NUMERICAL_CASE_COVERAGE_INCOMPLETE", "NUMERICAL_AGGREGATE_CONTRADICTION"]
    );
}

// loadReferenceReaders.test.ts, re-pointed by B7b at the current exact identities:
// the load/reference-state group is read under pressure-2 and pressure-source-1.
const LR: [&str; 4] = ["connected-sparse_interactive", "connected-dense_scrutiny", "pressure-sparse_interactive", "pressure-dense_scrutiny"];
fn lr_raw(name: &str) -> Value {
    read(&format!("fixtures/product_preview/pressure/load_reference/{name}.raw.json"))
}
fn lr_model(name: &str) -> Value {
    read(&format!("fixtures/product_preview/pressure/load_reference/{}.request.json", name.split('-').next().unwrap()))["model"].clone()
}
fn lrs_raw(name: &str) -> Value {
    read(&format!("fixtures/product_preview/pressure_source/load_reference_source/{name}.raw.json"))
}
fn lrs_request(stem: &str) -> Value {
    read(&format!("fixtures/product_preview/pressure_source/load_reference_source/{stem}.request.json"))
}

#[test]
fn g5_load_state_statements_dispatch_with_their_tables() {
    for name in LR {
        let raw = lr_raw(name);
        let d = rv::dispatch(&raw, Some(&lr_model(name)), None);
        assert_eq!((d["ok"].as_bool(), d["route"].as_str()), (Some(true), Some("pressure")), "{name}");
        assert_eq!(d["binding"]["id"], sc::PRESSURE_ID);
        // The actual model: the published checks_passed cases are eligible.
        assert_eq!(d["standing"]["status"], "integrity_checked", "{name}: {}", d["standing"]);
        assert_eq!(findings(&d["standing"]), Vec::<&str>::new());
        assert_eq!(rv::validate("pressure", &raw, Some(&lr_model(name)), None), json!({"ok": true}));
    }
    for name in ["n05", "n06", "fields", "mixed", "eigen_motion"] {
        for mode in ["sparse_interactive", "dense_scrutiny"] {
            let raw = lrs_raw(&format!("{name}-{mode}"));
            let request = lrs_request(name);
            let d = rv::dispatch(&raw, Some(&request["model"]), None);
            assert_eq!(d["route"], "pressure_source", "{name}-{mode}");
            assert_eq!(d["binding"]["id"], sc::PRESSURE_SOURCE_ID);
            // G4: without the registered invocation the source route needs it.
            assert_eq!(
                d["standing"],
                json!({"route": "pressure_source", "token": d["standing"]["token"], "status": "needs_recompute",
                    "eligible": false, "findings": [rv::SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED]})
            );
            // With it, a valid load/reference-state reading is never eligible (T1).
            let inv = invocation(request.clone(), mode);
            let s = rv::standing(&raw, Some(&request["model"]), Some(&inv));
            assert_eq!(s["eligible"], false, "{name}-{mode}");
            assert!(findings(&s).contains(&rv::LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE), "{name}-{mode}: {s}");
            let v = rv::validate("pressure-source", &raw, None, Some(&inv));
            assert_eq!((v["ok"].as_bool(), v["eligible"].as_bool()), (Some(true), Some(false)), "{name}-{mode}: {v}");
            assert_eq!(rv::transport("pressure-source", &raw), json!({"ok": true}), "{name}-{mode}");
        }
    }
}

#[test]
fn g5_each_exact_identity_binds_its_profile_evidence_and_receipt() {
    let lr = lr_raw(LR[0]);
    let lrs = lrs_raw("n05-sparse_interactive");
    let edits: Vec<(&str, &Value, Box<dyn Fn(&mut Value)>)> = vec![
        ("pressure-2 with the retired joined profile", &lr, Box::new(|s| s["formulation_basis"]["profile_id"] = json!("resolved_straight_load_state_source_v1"))),
        ("pressure-2 with the retired v2 profile", &lr, Box::new(|s| s["formulation_basis"]["profile_id"] = json!("exact_straight_pressure_v2"))),
        ("pressure-2 without contract evidence", &lr, Box::new(|s| {
            s.as_object_mut().unwrap().remove("contract_evidence");
        })),
        ("pressure-2 with null contract evidence", &lr, Box::new(|s| s["contract_evidence"] = json!(null))),
        ("pressure-2 with array contract evidence", &lr, Box::new(|s| s["contract_evidence"] = json!([]))),
        ("pressure-2 with a source-block receipt", &lr, Box::new(|s| s["source_block_recovery"] = lrs_raw("n05-sparse_interactive")["source_block_recovery"].clone())),
        ("pressure-2 with a carrier namespace", &lr, Box::new(|s| s["carrier_evidence"] = json!({}))),
        ("joined with the retired load-state profile", &lrs, Box::new(|s| s["formulation_basis"]["profile_id"] = json!("resolved_straight_load_state_v1"))),
        ("joined without its receipt", &lrs, Box::new(|s| {
            s.as_object_mut().unwrap().remove("source_block_recovery");
        })),
        ("joined with the retired physics-source policy", &lrs, Box::new(|s| s["source_block_recovery"]["body"]["policy"] = json!("PHYSICS-SOURCE-1"))),
        ("joined with an extra receipt key", &lrs, Box::new(|s| s["source_block_recovery"]["body"]["invented"] = json!(1))),
        ("joined relabelled as pressure-2", &lrs, Box::new(|s| {
            s["producer"]["semantic_contract_id"] = json!(sc::PRESSURE_ID);
        })),
        ("joined relabelled as retired load-reference-source-1", &lrs, Box::new(|s| {
            s["producer"]["semantic_contract_id"] = json!(sc::RETIRED_EXACT_V2_IDS[3]);
            s["formulation_basis"]["profile_id"] = json!("resolved_straight_load_state_source_v1");
        })),
        ("pressure-2 relabelled as joined", &lr, Box::new(|s| {
            s["producer"]["semantic_contract_id"] = json!(sc::PRESSURE_SOURCE_ID);
        })),
        ("pressure-2 relabelled as retired load-reference-1", &lr, Box::new(|s| {
            s["producer"]["semantic_contract_id"] = json!(sc::RETIRED_EXACT_V2_IDS[2]);
            s["formulation_basis"]["profile_id"] = json!("resolved_straight_load_state_v1");
        })),
    ];
    for (name, base, edit) in edits {
        let mut s = base.clone();
        edit(&mut s);
        assert_eq!(route(&s), rv::UNSUPPORTED, "{name}");
        let d = rv::dispatch(&s, None, None);
        assert_eq!((d["ok"].as_bool(), d["binding"].is_null()), (Some(false), true), "{name}");
        assert_eq!(d["standing"]["eligible"], false);
    }
    // The current receipt shape: its policy and the closed PRESSURE-SOURCE-1 shape.
    use open_pipe_stress_result_export::physics_source::validate_receipt_shape;
    let receipt = lrs["source_block_recovery"].clone();
    assert!(validate_receipt_shape(&receipt).is_ok());
    let mut policy = receipt.clone();
    policy["body"]["policy"] = json!("PHYSICS-SOURCE-1");
    assert!(validate_receipt_shape(&policy).is_err());
    let mut extra = receipt.clone();
    extra["extra"] = json!(1);
    assert!(validate_receipt_shape(&extra).is_err());
    assert!(validate_receipt_shape(&Value::Null).is_err());
}

#[test]
fn g2_load_state_reader_refusals_and_model_binding_are_findings() {
    let mut raw = lr_raw(LR[0]);
    let model = lr_model(LR[0]);
    raw["contract_evidence"]["load_reference_states"][0]["members"][0]["derived_G_pa"] = json!(1);
    let s = standing(&raw, Some(&model));
    assert_eq!(s["eligible"], false);
    assert!(findings(&s).iter().any(|f| f.starts_with("SOURCE_LOAD_REFERENCE_MEMBER_G_BINDING")), "{s}");
    // G2's model binding runs after the unchanged pressure-2 reader (B7b: on the
    // received statement; the retired reader's projected copy is gone).
    let raw = lr_raw(LR[0]);
    let mut fewer = model.clone();
    fewer["load_cases"].as_array_mut().unwrap().remove(0);
    let s = standing(&raw, Some(&fewer));
    assert_eq!(s["eligible"], false);
    assert!(findings(&s).contains(&"SOURCE_PHYSICS_MODEL_CASE_COVERAGE"), "{s}");
    assert_eq!(
        rv::validate("pressure", &raw, Some(&fewer), None)["error"],
        json!({"code": "SOURCE_PHYSICS_MODEL_CASE_COVERAGE", "detail": null})
    );
    // With no model the reader is today's Rust reader.
    assert_eq!(rv::validate("pressure", &raw, None, None), json!({"ok": true}));
    for mode in ["sparse_interactive", "dense_scrutiny"] {
        let raw = read(&format!("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-{mode}.raw.json"));
        let model = read("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json")["model"].clone();
        assert_eq!(route(&raw), "pressure");
        assert_eq!(rv::validate("pressure", &raw, Some(&model), None), json!({"ok": true}));
        let s = standing(&raw, Some(&model));
        assert_eq!(s["eligible"], false);
        assert!(findings(&s).contains(&"NUMERICAL_INTEGRITY_NOT_QUALIFIED"));
    }
}

#[test]
fn g1_joined_refusal_keeps_its_validation_code() {
    let mut raw = lrs_raw("n05-sparse_interactive");
    raw["source_block_recovery"]["receipt_sha256"] = json!("0".repeat(64));
    let request = lrs_request("n05");
    let inv = invocation(request.clone(), "sparse_interactive");
    assert_eq!(findings(&rv::standing(&raw, Some(&request["model"]), Some(&inv))), ["PHYSICS_SOURCE_RECEIPT_HASH"]);
    assert_eq!(rv::validate("pressure-source", &raw, None, None)["error"]["code"], "PHYSICS_SOURCE_RECEIPT_HASH");
}

#[test]
fn g2_physics_model_binding() {
    let request = read("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
    let model = &request["model"];
    for mode in ["sparse", "dense"] {
        let raw = read(&format!("fixtures/results/pressure_connected_mechanics_{mode}.json"));
        assert_eq!(route(&raw), "pressure");
        assert_eq!(rv::validate("pressure", &raw, Some(model), None), json!({"ok": true}));
        assert_eq!(rv::validate("pressure", &raw, None, None), json!({"ok": true}));
        let code = |m: &Value| rv::validate("pressure", &raw, Some(m), None)["error"]["code"].clone();
        let mut fewer = model.clone();
        fewer["load_cases"].as_array_mut().unwrap().pop();
        assert_eq!(code(&fewer), "SOURCE_PHYSICS_MODEL_CASE_COVERAGE");
        let mut more_pipes = model.clone();
        more_pipes["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "pipe:extra"}));
        assert_eq!(code(&more_pipes), "SOURCE_PHYSICS_MODEL_MEMBER_COVERAGE");
        let mut no_pipes = model.clone();
        no_pipes.as_object_mut().unwrap().remove("pipe_segments");
        assert_eq!(code(&no_pipes), Value::Null);
        let mut other_support = model.clone();
        other_support["supports"] = json!([{"id": "support:other"}]);
        assert_eq!(code(&other_support), "SOURCE_PHYSICS_SUPPORT_BINDING");
        let mut no_supports = model.clone();
        no_supports.as_object_mut().unwrap().remove("supports");
        assert_eq!(code(&no_supports), Value::Null);
        // An unsolved result is not held to the model's cases.
        let mut unsolved = raw.clone();
        unsolved["status"]["mechanics"] = json!("MECHANICS_NOT_SOLVED");
        assert!(rv::physics_model_checks(&unsolved, &fewer).is_ok());
        // Standing carries the reader's refusal as its finding.
        assert!(findings(&standing(&raw, Some(&fewer))).contains(&"SOURCE_PHYSICS_MODEL_CASE_COVERAGE"));
    }
}

// sourceBlockRecovery.test.ts and physicsSourceRecovery.test.ts.
fn invocation(request: Value, mode: &str) -> Value {
    json!({"request": request, "solver_mode": mode})
}

#[test]
fn a_stored_source_blocks_1_statement_reads_unsupported_by_name() {
    // B4/D1 (b): the non-composite identity is retired; its header refuses by name.
    let mut stored = read("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");
    stored["producer"]["semantic_contract_id"] = json!(sc::RETIRED_SOURCE_BLOCKS_ID);
    let d = rv::dispatch(&stored, None, None);
    assert_eq!((d["route"].as_str(), d["error"]["code"].as_str()), (Some(rv::UNSUPPORTED), Some(sc::SOURCE_BLOCKS_1_RETIRED)));
    assert_eq!(rv::validate("source-blocks", &stored, None, None)["error"]["code"], "READER_UNKNOWN");
    assert_eq!(rv::transport("source-blocks", &stored)["error"]["code"], "READER_UNKNOWN");
    assert_eq!(rv::binding("source_blocks"), Value::Null);
}

#[test]
fn g1_genuine_pressure_source_validates_with_its_invocation() {
    for name in ["n05", "n06", "fields", "mixed", "mixed_units", "n05_units", "n05_unicode"] {
        let request = read(&format!("fixtures/product_preview/pressure_source/physics_source/{name}.request.json"));
        for mode in ["dense_scrutiny", "sparse_interactive"] {
            let raw = read(&format!("fixtures/product_preview/pressure_source/physics_source/{name}-{mode}.raw.json"));
            let inv = invocation(request.clone(), mode);
            assert_eq!(route(&raw), "pressure_source", "{name}");
            let v = rv::validate("pressure-source", &raw, None, Some(&inv));
            assert_eq!(v["ok"], true, "{name}-{mode}: {v}");
            let s = rv::standing(&raw, Some(&request["model"]), Some(&inv));
            assert_eq!(s["findings"], v["findings"], "{name}-{mode}");
            assert_eq!(s["eligible"], v["eligible"], "{name}-{mode}");
            assert_eq!(rv::transport("pressure-source", &raw), json!({"ok": true}));
            // G2 through the invocation's model.
            let mut other = request.clone();
            other["model"]["supports"] = json!([{"id": "support:other"}]);
            let has_support_rows = raw["results"].as_array().unwrap().iter().any(|r| {
                r["kind"].as_str().is_some_and(|k| k.starts_with("support_reaction_") && k.ends_with("_v2"))
            });
            if has_support_rows {
                let refused = rv::validate("pressure-source", &raw, None, Some(&invocation(other, mode)));
                assert_eq!(refused["ok"], false, "{name}-{mode}");
            }
        }
    }
}

#[test]
fn g5_source_receipt_presence_on_other_headers_is_unsupported() {
    let ordinary = read("fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json");
    assert_eq!(route(&ordinary), "preview_physics");
    for value in [json!(null), json!(false), json!(0), json!({}), json!("")] {
        let mut s = ordinary.clone();
        s["source_block_recovery"] = value;
        assert_eq!(route(&s), rv::UNSUPPORTED);
        s["schema_version"] = json!("0.1.0");
        for k in ["producer", "numerical_quality", "formulation_basis", "contract_evidence"] {
            s.as_object_mut().unwrap().remove(k);
        }
        assert_eq!(route(&s), rv::UNSUPPORTED);
    }
    let mut unknown = ordinary.clone();
    unknown["producer"]["semantic_contract_id"] = json!(format!("{}/future", sc::RETIRED_SOURCE_BLOCKS_ID));
    assert_eq!(route(&unknown), rv::UNSUPPORTED);
    // A source route's receipt shape is the header's (G5): an extra receipt key.
    let mut composite = read("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");
    composite["source_block_recovery"]["extra"] = json!(1);
    assert_eq!(route(&composite), rv::UNSUPPORTED);
    // A stored precision-1 statement is refused by name (B1).
    let mut precision = ordinary_source();
    precision["producer"]["semantic_contract_id"] = json!(sc::RETIRED_PRECISION_ID);
    precision.as_object_mut().unwrap().remove("contract_evidence");
    assert_eq!(route(&precision), rv::UNSUPPORTED);
    assert_eq!(rv::dispatch(&precision, None, None)["error"]["code"], sc::SOURCE_PRECISION_1_RETIRED);
    assert_eq!(findings(&standing(&precision, None)), [rv::SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED]);
    let mut carrier = ordinary_source();
    carrier["carrier_evidence"] = json!({});
    assert_eq!(route(&carrier), rv::UNSUPPORTED);
}

#[test]
fn preview_physics_standing_and_transport() {
    for mode in ["sparse", "dense"] {
        let raw = read(&format!("fixtures/results/preview_physics_invented_{mode}.json"));
        let model = read("fixtures/product_preview/invented_demo_model.json");
        let d = rv::dispatch(&raw, Some(&model), None);
        assert_eq!((d["route"].as_str(), d["binding"]["id"].as_str()), (Some("preview_physics"), Some(sc::PREVIEW_PHYSICS_ID)));
        // Nonlinear supports: the converged states are confirmed and the cases
        // Passed (DEL-04-01 kd5-nonlinear-active-set.md section 5), so the
        // standing is empty, as on the linear route.
        assert_eq!(findings(&d["standing"]), Vec::<&str>::new(), "{mode}: {}", d["standing"]);
        assert_eq!(rv::validate("preview-physics", &raw, Some(&model), None), json!({"ok": true}));
        assert_eq!(rv::transport("preview-physics", &raw), json!({"ok": true}));
        // A linear-route CHECKS_PASSED preview envelope keeps an empty standing.
        let raw = read(&format!("fixtures/results/preview_physics_connected_{mode}.json"));
        let model = read("fixtures/model_operations/precision_connected_ui_model.json");
        let d = rv::dispatch(&raw, Some(&model), None);
        assert_eq!(findings(&d["standing"]), Vec::<&str>::new(), "{mode}: {}", d["standing"]);
        assert_eq!(rv::validate("preview-physics", &raw, Some(&model), None), json!({"ok": true}));
    }
}

#[test]
fn binding_hashes_are_the_table_files() {
    use sha2::{Digest, Sha256};
    let files = [
        ("pressure_source", "semantic_contract_v0_3_pressure_source_1.json"),
        ("preview_physics", "semantic_contract_v0_3_preview_physics_2.json"),
        ("pressure", "semantic_contract_v0_3_pressure_2.json"),
        ("retained_preview_physics", "semantic_contract_v0_3_preview_physics_retained_1.json"),
        ("retained_physics", "semantic_contract_v0_3_physics_retained_1.json"),
    ];
    // B7b: the five current routes; the retired v2 routes bind nothing.
    assert_eq!(rv::ROUTES.as_slice(), files.iter().map(|f| f.0).collect::<Vec<_>>().as_slice());
    for retired in ["physics", "physics_source", "load_reference", "load_reference_source", "source_blocks"] {
        assert_eq!(rv::binding(retired), Value::Null, "{retired}");
    }
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../fixtures/results/");
    for (route, file) in files {
        let bytes = std::fs::read(format!("{root}{file}")).unwrap();
        let hex: String = Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(rv::binding(route)["sha256"], hex, "{route}");
    }
    assert_eq!(rv::binding("legacy"), Value::Null);
    assert_eq!(rv::binding(rv::UNSUPPORTED), Value::Null);
}

#[test]
fn row_semantics_follow_the_dispatched_table() {
    let raw = lr_raw(LR[0]);
    let row = raw["results"].as_array().unwrap().iter().find(|r| r["kind"].as_str().unwrap().ends_with("_v2")).unwrap();
    let out = rv::row_semantics(row, Some(&raw));
    assert_eq!(out["ok"], true, "{out}");
    assert!(out["signature"].is_object() || out["signature"].is_string() || out["signature"].is_array(), "{out}");
    let mut bad = raw.clone();
    bad["carrier_evidence"] = json!({});
    assert_eq!(rv::row_semantics(row, Some(&bad))["error"]["code"], "SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
    // b7-plan E2: no source names no table (the retired v0_2 table is gone).
    let row = json!({"id": "r", "kind": "global_nodal_rotation_x", "value": 0.0, "unit": "rad", "entity_ref": "n"});
    assert_eq!(rv::row_semantics(&row, None)["error"]["code"], "SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
}

#[test]
fn error_json_splits_code_and_detail() {
    assert_eq!(rv::error_json("SOURCE_X"), json!({"code": "SOURCE_X", "detail": null}));
    assert_eq!(rv::error_json("SOURCE_X: more"), json!({"code": "SOURCE_X", "detail": "SOURCE_X: more"}));
    assert_eq!(rv::error_json("SOURCE_V2_Y"), json!({"code": "SOURCE_V2_Y", "detail": null}));
}

#[test]
fn source_recovery_preserves_the_portable_commitment_check_depth() {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    for (folder, receipt_error) in [
        ("pressure_source/physics_source", "PHYSICS_SOURCE_RECEIPT_HASH"),
        ("pressure_source/load_reference_source", "PHYSICS_SOURCE_RECEIPT_HASH"),
    ] {
        let raw = read(&format!("fixtures/product_preview/{folder}/n05-sparse_interactive.raw.json"));
        assert_eq!(rv::validate("source-recovery", &raw, None, None), json!({"ok": true}));
        let mut bad = raw.clone();
        bad["source_block_recovery"]["receipt_sha256"] = json!("0".repeat(64));
        assert_eq!(rv::validate("source-recovery", &bad, None, None)["error"]["code"], receipt_error);
        let mut bad = raw.clone();
        bad["run_id"] = json!("edited-publication");
        assert_eq!(rv::validate("source-recovery", &bad, None, None)["error"]["code"], "SOURCE_BLOCKS_PUBLICATION_HASH");
        // Rebinding a changed row does not turn this portable check into the raw
        // evidence reader. That separate reader still refuses the changed row.
        let mut changed = raw.clone();
        changed["results"][0]["id"] = json!("unbound-result-row");
        let mut publication = changed.clone();
        publication.as_object_mut().unwrap().remove("source_block_recovery");
        changed["source_block_recovery"]["body"]["publication_sha256"] = json!(domain_hash("source_blocks_publication_v1", &publication).unwrap());
        changed["source_block_recovery"]["receipt_sha256"] = json!(domain_hash("source_blocks_receipt_v1", &changed["source_block_recovery"]["body"]).unwrap());
        assert_eq!(rv::validate("source-recovery", &changed, None, None), json!({"ok": true}));
        assert_eq!(rv::validate("pressure-source", &changed, None, None)["ok"], false);
    }
    assert_eq!(rv::validate("source-recovery", &ordinary_source(), None, None)["error"]["code"], "SOURCE_RECOVERY_METADATA_CONTRADICTION");
}

/// pressure-2 (DEL-04-07 `pressure-contract.md`, "Reading pressure-2"): its
/// own route and table binding, read by `validate_pressure_evidence` with G2's
/// model checks and the ordinary standing, so every caller takes one verdict
/// and one standing. Straight (0.3.0 and 0.4.0), arc and connector corpora.
#[test]
fn pressure_1_dispatches_to_its_reader_and_ordinary_standing() {
    for corpus in ["straight", "arc", "connector"] {
        let corpus = read(&format!("fixtures/results/pressure_v3_{corpus}_reader_corpus.json"));
        for case in corpus["cases"].as_array().unwrap() {
            let (label, raw) = (case["label"].as_str().unwrap(), &case["envelope"]);
            assert_eq!(route(raw), "pressure", "{label}");
            let d = rv::dispatch(raw, None, None);
            assert_eq!((d["ok"].as_bool(), d["route"].as_str()), (Some(true), Some("pressure")), "{label}");
            assert_eq!(d["binding"], json!({"id": sc::PRESSURE_ID, "sha256": rv::binding("pressure")["sha256"]}), "{label}");
            assert_eq!(rv::validate("pressure", raw, None, None), json!({"ok": true}), "{label}");
            // The standing is the reader's verdict plus the ordinary checks of
            // `numerical_quality` against the model's cases.
            let cases: Vec<Value> = raw["numerical_quality"]["cases"].as_array().unwrap().iter()
                .map(|c| json!({"id": c["basis_ref"]["ref_id"]})).collect();
            let model = json!({"load_cases": cases});
            let s = standing(raw, Some(&model));
            let passed = raw["numerical_quality"]["status"] == "checks_passed";
            assert_eq!(findings(&s).is_empty(), passed, "{label}: {s}");
            assert_eq!((s["route"].as_str(), s["eligible"].as_bool()), (Some("pressure"), Some(passed)), "{label}");
            // G2: a model that lacks a solved case is a finding of the reader.
            let short = json!({"load_cases": [{"id": "load:not-solved"}]});
            assert_eq!(
                rv::validate("pressure", raw, Some(&short), None)["error"]["code"],
                "SOURCE_PHYSICS_MODEL_CASE_COVERAGE",
                "{label}"
            );
            assert!(findings(&standing(raw, Some(&short))).contains(&"SOURCE_PHYSICS_MODEL_CASE_COVERAGE"), "{label}");
            // A reader refusal is the standing's finding.
            let mut broken = raw.clone();
            broken["contract_evidence"]["exact_cases"] = json!([]);
            assert_eq!(rv::validate("pressure", &broken, None, None)["ok"], false, "{label}");
            assert_eq!(standing(&broken, Some(&model))["eligible"], false, "{label}");
            // No pressure-2 transport reader: a transported statement is refused by name.
            assert_eq!(rv::transport("pressure", raw)["error"]["code"], rv::PRESSURE_TRANSPORT_UNSUPPORTED, "{label}");
            // The pressure-2 header requires its evidence object.
            let mut bare = raw.clone();
            bare.as_object_mut().unwrap().remove("contract_evidence");
            assert_eq!(route(&bare), rv::UNSUPPORTED, "{label}");
        }
    }
    // The arc corpus carries its documents: their models pass G2.
    let arc = read("fixtures/results/pressure_v3_arc_reader_corpus.json");
    for case in arc["cases"].as_array().unwrap() {
        let model = &case["document"]["model"];
        assert_eq!(rv::validate("pressure", &case["envelope"], Some(model), None), json!({"ok": true}), "{}", case["label"]);
    }
}

/// B7b (A4/A5): the four identities of the retired 2.0.0/exact_straight_pressure_v2
/// contract are refused by name at the header (SOURCE_EXACT_PRESSURE_V2_RETIRED),
/// read as unsupported by the standing and name no table; no v2 reader remains.
/// precision-1 (B1), source-blocks-1 (B4/D1 (b)) and the 0.1.0 mechanics-result
/// carrier (E2) keep their own names.
#[test]
fn retired_identities_and_readers_are_refused_by_name() {
    assert_eq!(rv::READERS, ["preview-physics", "pressure-source", "source-recovery", "pressure"]);
    let base = lr_raw(LR[0]);
    for id in sc::RETIRED_EXACT_V2_IDS {
        let mut s = base.clone();
        s["producer"]["semantic_contract_id"] = json!(id);
        assert_eq!(route(&s), rv::UNSUPPORTED, "{id}");
        let d = rv::dispatch(&s, None, None);
        assert_eq!((d["ok"].as_bool(), d["binding"].is_null()), (Some(false), true), "{id}");
        assert_eq!(d["error"]["code"], sc::SOURCE_EXACT_PRESSURE_V2_RETIRED, "{id}");
        assert!(d["error"]["detail"].as_str().unwrap().contains("re-author the model as 0.7.0"), "{id}");
        assert_eq!(findings(&d["standing"]), [rv::SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED], "{id}");
        assert_eq!(rv::row_semantics(&s["results"][0], Some(&s))["error"]["code"], "SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED", "{id}");
    }
    for reader in ["physics", "physics-source", "load-reference", "load-reference-source", "source-blocks"] {
        assert_eq!(rv::validate(reader, &base, None, None)["error"]["code"], "READER_UNKNOWN", "{reader}");
        assert_eq!(rv::transport(reader, &base)["error"]["code"], "READER_UNKNOWN", "{reader}");
    }
    let named = [
        ("/producer/semantic_contract_id", json!(sc::RETIRED_PRECISION_ID), sc::SOURCE_PRECISION_1_RETIRED),
        ("/producer/semantic_contract_id", json!(sc::RETIRED_SOURCE_BLOCKS_ID), sc::SOURCE_BLOCKS_1_RETIRED),
        ("/schema_version", json!(sc::RETIRED_MECHANICS_RESULT_VERSION), sc::MECHANICS_RESULT_VERSION_RETIRED),
    ];
    for (pointer, value, code) in named {
        let mut s = base.clone();
        *s.pointer_mut(pointer).unwrap() = value;
        assert_eq!(rv::dispatch(&s, None, None)["error"]["code"], code);
    }
}
