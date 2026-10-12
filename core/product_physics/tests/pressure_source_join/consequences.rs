//! Consequences over genuine public producer twins. Resealing is used only to
//! construct named negative reader statements, never to fabricate a positive.
use super::*;
use open_pipe_stress_product_physics::result_export::derivative;

fn matches(path: &[String], pattern: &[&str]) -> bool {
    path.len() == pattern.len()
        && path
            .iter()
            .zip(pattern)
            .all(|(p, q)| *q == "*" || p.as_str() == *q)
}
fn excluded(path: &[String]) -> bool {
    [
        &["producer", "semantic_contract_id"][..],
        &["formulation_basis", "profile_id"],
        &["formulation_basis", "limitations"],
        &["contract_evidence", "exact_cases", "*", "profile_mode"],
        &["contract_evidence", "pressure", "*", "profile_mode"],
        &["contract_evidence", "pressure", "*", "profile_version"],
        &["source_block_recovery", "body", "policy"],
        &["source_block_recovery", "body", "invocation", "value"],
        &["source_block_recovery", "body", "publication_sha256"],
        &["source_block_recovery", "receipt_sha256"],
        &[
            "source_block_recovery",
            "body",
            "cases",
            "*",
            "physical_evidence_sha256",
        ],
        &[
            "source_block_recovery",
            "body",
            "cases",
            "*",
            "source",
            "normalized_source_sha256",
        ],
        &[
            "source_block_recovery",
            "body",
            "cases",
            "*",
            "source",
            "functional_plan_sha256",
        ],
        &[
            "source_block_recovery",
            "body",
            "cases",
            "*",
            "work",
            "charged",
        ],
        &[
            "source_block_recovery",
            "body",
            "invocation_work",
            "charged",
        ],
        &[
            "source_block_recovery",
            "body",
            "invocation_work",
            "publication_charged",
        ],
    ]
    .iter()
    .any(|pattern| matches(path, pattern))
}
fn equal_leaves(left: &Value, right: &Value, path: &mut Vec<String>) {
    if excluded(path) {
        return;
    }
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            assert_eq!(
                a.keys().collect::<Vec<_>>(),
                b.keys().collect::<Vec<_>>(),
                "members at {path:?}"
            );
            for (key, item) in a {
                path.push(key.clone());
                equal_leaves(item, &b[key], path);
                path.pop();
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            assert_eq!(a.len(), b.len(), "array size at {path:?}");
            for (index, (a, b)) in a.iter().zip(b).enumerate() {
                path.push(index.to_string());
                equal_leaves(a, b, path);
                path.pop();
            }
        }
        _ => assert_eq!(left, right, "nonexcluded leaf at {path:?}"),
    }
}
fn encoded_length(request: &Value) -> u64 {
    let text = serde_json::to_string(request).unwrap();
    canonical_json_checked_v1_text(&text)
        .unwrap()
        .len()
        .try_into()
        .unwrap()
}
fn count(value: &Value) -> u64 {
    value.as_array().unwrap().len().try_into().unwrap()
}
fn utf8(value: &Value) -> u64 {
    value.as_str().unwrap().len().try_into().unwrap()
}
fn publication_other_inputs(request: &Value, source: &Value) -> u64 {
    // Independently enumerate the existing reservation's actual inputs. No
    // producer counter, multiplier accessor or finalization helper is called.
    let rows = count(&source["results"]);
    let diagnostics = source["diagnostics"].as_array().unwrap();
    let diagnostic_bytes: u64 = diagnostics
        .iter()
        .map(|d| {
            utf8(&d["id"])
                + utf8(&d["code"])
                + utf8(&d["message"])
                + d["affected_refs"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .map(utf8)
                    .sum::<u64>()
        })
        .sum();
    let model = &request["model"];
    1024 + 32 * rows
        + 32 * diagnostics.len() as u64
        + 12 * diagnostic_bytes
        + count(&model["load_cases"])
            * (32000 * count(&model["pipe_segments"])
                + 64 * count(&model["nodes"]).pow(2)
                + 256 * rows)
}
fn check_publication_accounting(control: &Control) {
    let v2 = &control.legacy.source["source_block_recovery"]["body"];
    let v3 = &control.joined.source["source_block_recovery"]["body"];
    let e2 = encoded_length(&control.original);
    let e3 = encoded_length(&control.invocation["request"]);
    let p2 = publication_other_inputs(&control.original, &control.legacy.source);
    let p3 = publication_other_inputs(&control.invocation["request"], &control.joined.source);
    assert_eq!(
        v2["invocation_work"]["publication_charged"]
            .as_u64()
            .unwrap(),
        24 * e2 + p2
    );
    assert_eq!(
        v3["invocation_work"]["publication_charged"]
            .as_u64()
            .unwrap(),
        24 * e3 + p3
    );
    for body in [v2, v3] {
        let cases: u64 = body["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| {
                c["work"]["charged"].as_u64().unwrap()
                    + c["work"]["reserved_unobserved_failure"].as_u64().unwrap()
            })
            .sum();
        assert_eq!(
            body["invocation_work"]["charged"].as_u64().unwrap(),
            cases
                + body["invocation_work"]["publication_charged"]
                    .as_u64()
                    .unwrap()
        );
    }
    // pressure_source_private_reservation_inputs_confirm_all_twins_complete_expression
    // independently proves equal non-E descriptor/system/row/support shapes and
    // zero remaining-work deltas over these same 24 twins at the actual private
    // reservation entry. Thus F_k's delta is zero here, not an inferred counter.
    // This consuming check has no dependency on opt-in scratch observations.
    let delta_e = i128::from(e3) - i128::from(e2);
    for (a, b) in v2["cases"]
        .as_array()
        .unwrap()
        .iter()
        .zip(v3["cases"].as_array().unwrap())
    {
        let delta = i128::from(b["work"]["charged"].as_u64().unwrap())
            - i128::from(a["work"]["charged"].as_u64().unwrap());
        let expected = if a["selected_method"] == "retained_source_blocks_exact_v1" {
            2 * 12 * delta_e
        } else {
            0
        };
        assert_eq!(
            delta, expected,
            "selected-case exact reservation delta {}",
            a["basis_ref"]["ref_id"]
        );
    }
    let local_delta: i128 = v2["cases"]
        .as_array()
        .unwrap()
        .iter()
        .zip(v3["cases"].as_array().unwrap())
        .map(|(a, b)| {
            i128::from(b["work"]["charged"].as_u64().unwrap())
                - i128::from(a["work"]["charged"].as_u64().unwrap())
        })
        .sum();
    assert_eq!(
        i128::from(v3["invocation_work"]["charged"].as_u64().unwrap())
            - i128::from(v2["invocation_work"]["charged"].as_u64().unwrap()),
        local_delta + 24 * (i128::from(e3) - i128::from(e2)) + i128::from(p3) - i128::from(p2)
    );
}
fn case_payload(source: &Value, index: usize) -> Value {
    let evidence = &source["contract_evidence"];
    let exact = &evidence["exact_cases"][index];
    let pressure: Vec<_> = evidence["pressure"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| p["load_case_id"] == exact["load_case_id"])
        .collect();
    let mut payload = json!({"exact_case":exact,"pressure":pressure});
    if let Some(states) = evidence.get("load_reference_states") {
        payload["load_reference_state"] = states
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["load_case_id"] == exact["load_case_id"])
            .unwrap()
            .clone();
    }
    payload
}
fn reseal_receipt(source: &mut Value) {
    source["source_block_recovery"]["receipt_sha256"] = json!(domain_commitment(
        "source_blocks_receipt_v1",
        &source["source_block_recovery"]["body"]
    ));
}
fn reseal_publication(source: &mut Value) {
    let mut publication = source.clone();
    publication
        .as_object_mut()
        .unwrap()
        .remove("source_block_recovery");
    source["source_block_recovery"]["body"]["publication_sha256"] = json!(domain_commitment(
        "source_blocks_publication_v1",
        &publication
    ));
    reseal_receipt(source);
}
fn reseal_physical(source: &mut Value, domain: &str) {
    let n = source["source_block_recovery"]["body"]["cases"]
        .as_array()
        .unwrap()
        .len();
    for index in 0..n {
        source["source_block_recovery"]["body"]["cases"][index]["physical_evidence_sha256"] =
            json!(domain_commitment(domain, &case_payload(source, index)));
    }
    reseal_receipt(source);
}
fn metadata(source: &Value) -> Value {
    let mut value = serde_json::Map::new();
    for key in [
        "schema_version",
        "producer",
        "numerical_quality",
        "formulation_basis",
        "contract_evidence",
        "source_block_recovery",
    ] {
        value.insert(key.to_owned(), source[key].clone());
    }
    Value::Object(value)
}
fn check_refusal(outcome: &Value, code: &str) {
    assert_eq!(outcome["ok"], false, "{outcome}");
    assert_eq!(outcome["error"]["code"], code, "{outcome}");
}
#[derive(Clone, Copy)]
enum DispatchDisposition<'a> {
    PressureHealthy(bool),
    PressureFinding(&'a str),
    HeaderRefused(&'a str),
    PhysicsV2CapturedV3,
    LoadReferenceV2T1,
}
fn checked_dispatch(
    source: &Value,
    model: &Value,
    invocation: &Value,
    disposition: DispatchDisposition<'_>,
) -> Value {
    let actual = rv::dispatch(source, Some(model), Some(invocation));
    let (route, binding, token, findings, error) = match disposition {
        DispatchDisposition::PressureHealthy(resolved) => (
            "pressure_source",
            json!({"id":"openpipestress.result_semantics/0.3.0/pressure-source-1","sha256":"3039f7ca7d36a82f9d34baea648159d7a780ec9623879ce5aa3c890b10b19a61"}),
            if resolved {
                "needs_recompute"
            } else {
                "numerically_eligible"
            },
            if resolved {
                json!([rv::LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE])
            } else {
                json!([])
            },
            Value::Null,
        ),
        DispatchDisposition::PressureFinding(code) => (
            "pressure_source",
            json!({"id":"openpipestress.result_semantics/0.3.0/pressure-source-1","sha256":"3039f7ca7d36a82f9d34baea648159d7a780ec9623879ce5aa3c890b10b19a61"}),
            "unsupported",
            json!([code]),
            Value::Null,
        ),
        DispatchDisposition::HeaderRefused(code) => (
            "unsupported",
            Value::Null,
            "unsupported",
            json!([code]),
            json!({"code":code,"detail":null}),
        ),
        DispatchDisposition::PhysicsV2CapturedV3 => (
            "physics_source",
            json!({"id":"openpipestress.result_semantics/0.3.0/physics-source-1","sha256":"ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1"}),
            "unsupported",
            json!([sc::V3_READ_AS_V2]),
            Value::Null,
        ),
        DispatchDisposition::LoadReferenceV2T1 => (
            "load_reference_source",
            json!({"id":"openpipestress.result_semantics/0.3.0/load-reference-source-1","sha256":"d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337"}),
            "needs_recompute",
            json!([rv::LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE]),
            Value::Null,
        ),
    };
    let eligible = findings.as_array().unwrap().is_empty();
    assert_eq!(
        actual,
        json!({"ok":error.is_null(),"route":route,"binding":binding,"error":error,
        "standing":{"route":route,"token":token,"status":if eligible {"integrity_checked"} else {"needs_recompute"},"eligible":eligible,"findings":findings}}),
        "explicit governed dispatch disposition"
    );
    actual
}
fn corpus_entry(
    id: String,
    source: &Value,
    invocation: &Value,
    reader: &str,
    raw_code: Option<&str>,
    transport_code: Option<&str>,
    disposition: DispatchDisposition<'_>,
) -> Value {
    let model = &invocation["request"]["model"];
    let validation = rv::validate(reader, source, Some(model), Some(invocation));
    let unbound = rv::validate(reader, source, None, None);
    let transported = metadata(source);
    let transport = rv::transport(reader, &transported);
    match raw_code {
        Some(code) => {
            check_refusal(&validation, code);
            check_refusal(&unbound, code);
        }
        None => {
            assert_eq!(validation["ok"], true, "{validation}");
            assert_eq!(validation["eligible"], model["schema_version"] == "0.3.0");
            assert_eq!(unbound["ok"], true, "{unbound}");
            assert_eq!(unbound["eligible"], false);
        }
    }
    if let Some(code) = transport_code {
        check_refusal(&transport, code);
    } else {
        assert_eq!(transport, json!({"ok":true}));
    }
    json!({"id":id,"reader":reader,"source":source,"invocation":invocation,"transport_source":transported,
        "expected":{"validation":validation,"unbound":unbound,"transport":transport,
        "dispatch":checked_dispatch(source,model,invocation,disposition)}})
}
fn add_control_entries(control: &Control, entries: &mut Vec<Value>) {
    let prefix = format!(
        "{}:{}:{}",
        control.family.name(),
        control.name,
        control.mode.as_str()
    );
    let source = &control.joined.source;
    let invocation = &control.invocation;
    let mut positive = corpus_entry(
        format!("{prefix}:genuine"),
        source,
        invocation,
        "pressure-source",
        None,
        None,
        DispatchDisposition::PressureHealthy(control.family.resolved()),
    );
    positive["raw_fixture"] = json!(format!(
        "fixtures/product_preview/pressure_source/{}/{}-{}.raw.json",
        control.family.name(),
        control.name,
        control.mode.as_str()
    ));
    entries.push(positive);
    entries.push(corpus_entry(
        format!("{prefix}:MJ5-v2-offered-to-v3"),
        &control.legacy.source,
        invocation,
        "pressure-source",
        Some(sc::V2_READ_AS_V3),
        Some(sc::V2_READ_AS_V3),
        if control.family.resolved() {
            DispatchDisposition::LoadReferenceV2T1
        } else {
            DispatchDisposition::PhysicsV2CapturedV3
        },
    ));
    let mut wrong_domain = source.clone();
    reseal_physical(&mut wrong_domain, "physics_source_case_evidence_v1");
    entries.push(corpus_entry(
        format!("{prefix}:MJ2-wrong-domain"),
        &wrong_domain,
        invocation,
        "pressure-source",
        Some("PHYSICS_SOURCE_PHYSICAL_CASE_HASH"),
        Some("PHYSICS_SOURCE_PHYSICAL_CASE_HASH"),
        DispatchDisposition::PressureFinding("PHYSICS_SOURCE_PHYSICAL_CASE_HASH"),
    ));
    let mut stale = source.clone();
    stale["source_block_recovery"]["receipt_sha256"] = json!("0".repeat(64));
    entries.push(corpus_entry(
        format!("{prefix}:MJ4-stale-receipt"),
        &stale,
        invocation,
        "pressure-source",
        Some("PHYSICS_SOURCE_RECEIPT_HASH"),
        Some("PHYSICS_SOURCE_RECEIPT_HASH"),
        DispatchDisposition::PressureFinding("PHYSICS_SOURCE_RECEIPT_HASH"),
    ));
    let mut changed_publication = source.clone();
    changed_publication["source_block_recovery"]["body"]["publication_sha256"] =
        json!("0".repeat(64));
    reseal_receipt(&mut changed_publication);
    entries.push(corpus_entry(
        format!("{prefix}:MJ4-publication-binding"),
        &changed_publication,
        invocation,
        "pressure-source",
        Some("SOURCE_BLOCKS_PUBLICATION_HASH"),
        None,
        DispatchDisposition::PressureFinding("SOURCE_BLOCKS_PUBLICATION_HASH"),
    ));
    entries.push(corpus_entry(
        format!("{prefix}:MJ5-v3-offered-to-v2"),
        source,
        invocation,
        "physics-source",
        Some(sc::V3_READ_AS_V2),
        Some(sc::V3_READ_AS_V2),
        DispatchDisposition::PressureHealthy(control.family.resolved()),
    ));
    let mut relabelled = source.clone();
    relabelled["producer"]["semantic_contract_id"] = json!(sc::PHYSICS_SOURCE_ID);
    relabelled["source_block_recovery"] = json!({});
    entries.push(corpus_entry(
        format!("{prefix}:MJ5-relabelled-shape"),
        &relabelled,
        invocation,
        "pressure-source",
        Some(sc::V2_READ_AS_V3),
        Some(sc::V2_READ_AS_V3),
        DispatchDisposition::HeaderRefused(sc::V3_READ_AS_V2),
    ));
    let mut downgraded = source.clone();
    downgraded["producer"]["semantic_contract_id"] = json!(sc::PRESSURE_ID);
    downgraded["formulation_basis"]["profile_id"] = json!("exact_pressure_v3");
    entries.push(corpus_entry(
        format!("{prefix}:MJ7-downgrade"),
        &downgraded,
        invocation,
        "pressure-source",
        Some("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"),
        Some("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"),
        DispatchDisposition::HeaderRefused("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"),
    ));
    let mut budget = source.clone();
    budget["source_block_recovery"]["body"]["invocation_work"]["charged"] = json!(
        source["source_block_recovery"]["body"]["invocation_work"]["charged"]
            .as_u64()
            .unwrap()
            + 1
    );
    reseal_receipt(&mut budget);
    entries.push(corpus_entry(
        format!("{prefix}:invocation-budget-ledger"),
        &budget,
        invocation,
        "pressure-source",
        Some("SOURCE_BLOCKS_INVOCATION_WORK_LEDGER"),
        None,
        DispatchDisposition::PressureFinding("SOURCE_BLOCKS_INVOCATION_WORK_LEDGER"),
    ));
    let mut captured_v2 = invocation.clone();
    captured_v2["request"]["model"]["pressure_contract"] =
        json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"});
    add_context_entry(
        format!("{prefix}:MJ5-captured-v2-context"),
        source,
        &captured_v2,
        sc::V2_READ_AS_V3,
        entries,
    );
    let mut other_version = invocation.clone();
    other_version["request"]["model"]["schema_version"] = json!(if control.family.resolved() {
        "0.3.0"
    } else {
        "0.4.0"
    });
    add_context_entry(
        format!("{prefix}:received-captured-state-context"),
        source,
        &other_version,
        "PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT",
        entries,
    );
    assert_eq!(
        derivative::derive_document(
            json!({}),
            &invocation["request"]["model"],
            source,
            json!({}),
            None
        )
        .unwrap_err(),
        derivative::PRESSURE_1_EXPORT_NOT_AVAILABLE
    );
    assert_eq!(
        derivative::validate_document(&json!({}), source).unwrap_err(),
        derivative::PRESSURE_1_EXPORT_NOT_AVAILABLE
    );
}
fn add_context_entry(
    id: String,
    source: &Value,
    invocation: &Value,
    code: &str,
    entries: &mut Vec<Value>,
) {
    let model = &invocation["request"]["model"];
    let validation = rv::validate("pressure-source", source, Some(model), Some(invocation));
    check_refusal(&validation, code);
    let unbound = rv::validate("pressure-source", source, None, None);
    assert_eq!(unbound["ok"], true);
    assert_eq!(unbound["eligible"], false);
    let transported = metadata(source);
    let transport = rv::transport("pressure-source", &transported);
    assert_eq!(transport, json!({"ok":true}));
    entries.push(json!({"id":id,"reader":"pressure-source","source":source,"invocation":invocation,
        "transport_source":transported,"expected":{"validation":validation,"unbound":unbound,"transport":transport,
        "dispatch":checked_dispatch(source,model,invocation,DispatchDisposition::PressureFinding(code))}}));
}
fn add_selected_pressure_entries(control: &Control, ordinary: &Control, entries: &mut Vec<Value>) {
    let selected = control.joined.source["source_block_recovery"]["body"]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["selected_method"] == "retained_source_blocks_exact_v1")
        .unwrap();
    for pressure in [-1e-5, 0.0, 1e-5] {
        // Produce the internally coherent signed/zero cap, eigen and applied
        // load fields on an actual ordinary region, not by changing p_pa alone.
        let mut request = ordinary.invocation["request"].clone();
        request["model"]["load_cases"][1]["pressure_regions"][0]["pressure"]["value"] =
            json!(pressure);
        let actual = produce(&request, ordinary.mode);
        let actual_invocation = json!({"request":request,"solver_mode":ordinary.mode.as_str()});
        assert_eq!(
            ps::validate_in(
                &actual.source,
                Some(&actual_invocation),
                ps::SourceContract::PressureV3
            ),
            Ok(true),
            "coherent actual signed/zero region"
        );
        let mut region = actual.source["contract_evidence"]["pressure"][0].clone();
        assert_eq!(region["p_pa"].as_f64().unwrap(), pressure);
        let mut mutated = control.joined.source.clone();
        region["load_case_id"] = selected["basis_ref"]["ref_id"].clone();
        mutated["contract_evidence"]["pressure"]
            .as_array_mut()
            .unwrap()
            .push(region);
        reseal_physical(&mut mutated, "pressure_source_case_evidence_v1");
        reseal_publication(&mut mutated);
        entries.push(corpus_entry(
            format!(
                "{}:{}:{}:MJ3-selected-pressure-{pressure}",
                control.family.name(),
                control.name,
                control.mode.as_str()
            ),
            &mutated,
            &control.invocation,
            "pressure-source",
            Some("PHYSICS_SOURCE_SOURCE_PRESSURE_INVENTORY"),
            Some("PHYSICS_SOURCE_SOURCE_PRESSURE_INVENTORY"),
            DispatchDisposition::PressureFinding("PHYSICS_SOURCE_SOURCE_PRESSURE_INVENTORY"),
        ));
    }
}
fn current_fixture(control: &Control) {
    let folder = project_root().join(format!(
        "fixtures/product_preview/pressure_source/{}",
        control.family.name()
    ));
    let raw = folder.join(format!(
        "{}-{}.raw.json",
        control.name,
        control.mode.as_str()
    ));
    let request = folder.join(format!("{}.request.json", control.name));
    if std::env::var("SWBPIPE_PRESSURE_SOURCE_FIXTURES_WRITE").as_deref() == Ok("1") {
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(&raw, control.joined.bytes.as_bytes()).unwrap();
        std::fs::write(
            &request,
            format!(
                "{}\n",
                serde_json::to_string_pretty(&control.invocation["request"]).unwrap()
            ),
        )
        .unwrap();
    }
    assert_eq!(
        std::fs::read_to_string(raw).unwrap(),
        control.joined.bytes,
        "exact public producer bytes"
    );
    assert_eq!(
        serde_json::from_str::<Value>(&std::fs::read_to_string(request).unwrap()).unwrap(),
        control.invocation["request"]
    );
}
/// MJ9's kill: every nonexcluded leaf over all 24 twins and each work counter
/// as an exact integer. It also keeps the committed v3 twins and the J2 reader
/// corpus current.
#[test]
fn pressure_source_twins_have_only_d5_differences() {
    let mut entries = Vec::new();
    let mut controls = Vec::new();
    for family in [Family::Physics03, Family::LoadReference04] {
        for name in family.names() {
            for mode in [
                PreviewSolverMode::SparseInteractive,
                PreviewSolverMode::DenseScrutiny,
            ] {
                let control = check_control(family, name, mode);
                equal_leaves(
                    &control.legacy.source,
                    &control.joined.source,
                    &mut Vec::new(),
                );
                check_publication_accounting(&control);
                current_fixture(&control);
                controls.push(control);
            }
        }
    }
    assert_eq!(controls.len(), 24);
    for control in &controls {
        add_control_entries(control, &mut entries);
        // This 0.3 consuming kill avoids changing an unrelated resolved-state
        // inventory before testing the selected pressure inventory gate.
        if !control.family.resolved() && control.name == "n06" {
            let mixed = controls
                .iter()
                .find(|c| {
                    !c.family.resolved()
                        && c.name == "mixed"
                        && c.mode.as_str() == control.mode.as_str()
                })
                .unwrap();
            add_selected_pressure_entries(control, mixed, &mut entries);
        }
    }
    let corpus = json!({"artifact":"pressure-source-reader-corpus","schema_version":"1.0.0","entries":entries});
    if let Some(path) = std::env::var_os("SWBPIPE_PRESSURE_SOURCE_CORPUS_FULL_OUT") {
        std::fs::write(
            path,
            format!("{}\n", serde_json::to_string(&corpus).unwrap()),
        )
        .unwrap();
    }
    // Explicit first-generation mode writes only the authorized raw twins and
    // a temporary full document for the shared storage packer. Normal checks
    // compare the generated logical document against the committed strict DAG.
    if std::env::var("SWBPIPE_PRESSURE_SOURCE_FIXTURES_WRITE").as_deref() != Ok("1") {
        assert_eq!(
            read_corpus(),
            &corpus,
            "pressure-source reader corpus is current"
        );
    }
}

#[test]
fn pressure_source_declines_every_present_pressure_region() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let original = Family::Physics03.request("n05");
        let region = Family::Physics03.request("mixed")["model"]["load_cases"][1]
            ["pressure_regions"][0]
            .clone();
        for pressure in [-1e-5, 0.0, 1e-5] {
            let mut request = original.clone();
            request["model"]["pressure_contract"] =
                json!({"version":"3.0.0","mode":"exact_pressure_v3"});
            let mut entered = region.clone();
            entered["pressure"]["value"] = json!(pressure);
            request["model"]["load_cases"][0]["pressure_regions"] = json!([entered]);
            let source = produce(&request, mode).source;
            assert_ne!(
                source["producer"]["semantic_contract_id"],
                ps::PRESSURE_CONTRACT_ID
            );
            assert!(source.get("source_block_recovery").is_none());
            assert!(
                source["diagnostics"].as_array().unwrap().iter().any(|d| {
                    d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
                        && d["message"].as_str().is_some_and(|m| {
                            m.contains("stage: \"source closure\"")
                                && m.contains("helper_stage: SourceClosure")
                                && m.contains("error: Unsupported(\"exact source recovery requires explicitly empty pressure regions\")")
                        })
                }),
                "pressure={pressure}, mode={}, diagnostics={}",
                mode.as_str(),
                source["diagnostics"]
            );
        }
    }
}

fn read_corpus() -> &'static Value {
    static CORPUS: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    CORPUS.get_or_init(|| {
        let text = std::fs::read_to_string(
            project_root().join("fixtures/results/pressure_source_reader_corpus.json"),
        )
        .unwrap();
        corpus_storage::expand(serde_json::from_str(&text).unwrap())
    })
}

/// Corpus paths are selectors over the maintained generated vocabulary. Only
/// the matching generated path reaches the read callback; no unchecked input,
/// canonicalization, search or alias fallback reaches filesystem access.
fn read_listed_raw_fixture<T>(
    entry: &Value,
    read: impl FnOnce(std::path::PathBuf) -> T,
) -> Result<Option<T>, &'static str> {
    let Some(value) = entry.get("raw_fixture") else { return Ok(None); };
    let input = value.as_str().ok_or("raw_fixture must be a listed string")?;
    for family in [Family::Physics03, Family::LoadReference04] {
        for name in family.names() {
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let generated = format!(
                    "fixtures/product_preview/pressure_source/{}/{}-{}.raw.json",
                    family.name(), name, mode.as_str()
                );
                if input == generated {
                    return Ok(Some(read(project_root().join(generated))));
                }
            }
        }
    }
    Err("raw_fixture is not a maintained generated path")
}

#[test]
fn pressure_source_raw_fixture_path_guard_admits_exact_generated24() {
    let mut accepted = std::collections::BTreeSet::new();
    for family in [Family::Physics03, Family::LoadReference04] {
        for name in family.names() {
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let generated = format!(
                    "fixtures/product_preview/pressure_source/{}/{}-{}.raw.json",
                    family.name(), name, mode.as_str()
                );
                let calls = std::cell::Cell::new(0);
                let actual = read_listed_raw_fixture(&json!({"raw_fixture":generated}), |path| {
                    calls.set(calls.get()+1); path
                }).unwrap().unwrap();
                assert_eq!(actual, project_root().join(&generated));
                assert_eq!(calls.get(), 1);
                assert!(accepted.insert(generated));
            }
        }
    }
    assert_eq!(accepted.len(), 24);
}

#[test]
fn pressure_source_raw_fixture_path_guard_refuses_before_read() {
    let valid = "fixtures/product_preview/pressure_source/physics_source/n06-sparse_interactive.raw.json";
    let absolute = project_root().join(valid).to_string_lossy().into_owned();
    let invalid = [
        String::new(), absolute, format!("/{valid}"), format!("../{valid}"),
        format!("./{valid}"), format!("{valid}/"), format!("{valid}\0"),
        valid.replace("fixtures/", "fixtures//"),
        valid.replace("/physics_source/", "/physics_source/./"),
        valid.replace("/physics_source/", "/physics_source/../physics_source/"),
        valid.replace("n06-", "unlisted-"), valid.replace("n06-", "eigen_motion-"),
        valid.replace("physics_source/n06-", "load_reference_source/n05_unicode-"),
        valid.replace("sparse_interactive", "other_mode"),
        valid.replace("pressure_source/", "other_family/"),
        valid.replace("/", "\\"), valid.to_uppercase(),
        "C:\\fixtures\\n06-sparse_interactive.raw.json".into(),
    ];
    let mut refused = 0;
    for value in invalid.into_iter().map(Value::String).chain([
        Value::Null, json!(false), json!(24), json!([]), json!({"path":valid}),
    ]) {
        let calls = std::cell::Cell::new(0);
        let result = read_listed_raw_fixture(&json!({"raw_fixture":value}), |_| {
            calls.set(calls.get()+1);
        });
        assert!(result.is_err(), "unlisted or malformed fixture selector must refuse");
        assert_eq!(calls.get(), 0, "refusal occurs before the actual read boundary");
        refused += 1;
    }
    let calls = std::cell::Cell::new(0);
    assert_eq!(read_listed_raw_fixture(&json!({}), |_| calls.set(calls.get()+1)), Ok(None));
    assert_eq!(calls.get(), 0, "absent optional raw fixture performs no read");
    assert_eq!(refused, 23);
    println!("pressure_source_fixture_path_controls=24allowed,23refused_before_read,1absent_no_read");
}

#[test]
fn pressure_source_corpus_cache_shares_one_strict_expansion() {
    let corpus = read_corpus();
    assert_eq!(corpus["entries"].as_array().unwrap().len(), 270);
    for _ in 0..8 { assert!(std::ptr::eq(corpus, read_corpus())); }
    std::thread::scope(|scope| {
        let readers: Vec<_> = (0..8).map(|_| scope.spawn(read_corpus)).collect();
        for reader in readers { assert!(std::ptr::eq(corpus, reader.join().unwrap())); }
    });
    println!("pressure_source_corpus_cache=one_shared_immutable_value,270entries");
}
fn check_corpus_entry(entry: &Value) {
    let source = &entry["source"];
    let invocation = &entry["invocation"];
    let model = &invocation["request"]["model"];
    let reader = entry["reader"].as_str().unwrap();
    if let Some(text) = read_listed_raw_fixture(entry, |generated_path| {
        std::fs::read_to_string(generated_path).unwrap()
    }).expect("raw_fixture must be in the exact maintained generated vocabulary") {
        assert_eq!(
            serde_json::from_str::<Value>(&text).unwrap(),
            *source,
            "genuine received base {}",
            entry["id"]
        );
    }
    assert_eq!(
        json!({
            "validation":rv::validate(reader,source,Some(model),Some(invocation)),
            "unbound":rv::validate(reader,source,None,None),
            "transport":rv::transport(reader,&entry["transport_source"]),
            "dispatch":rv::dispatch(source,Some(model),Some(invocation)),
        }),
        entry["expected"],
        "{}",
        entry["id"]
    );
}
fn named_corpus_controls(marker: &str) {
    let corpus = read_corpus();
    let entries: Vec<_> = corpus["entries"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry["id"].as_str().unwrap().contains(marker))
        .collect();
    assert!(!entries.is_empty(), "named controls {marker} are present");
    for entry in entries {
        check_corpus_entry(entry);
    }
}
#[test]
fn pressure_source_reader_corpus_native_reads_every_statement_unchanged() {
    let corpus = read_corpus();
    assert_eq!(corpus["artifact"], "pressure-source-reader-corpus");
    assert_eq!(corpus["schema_version"], "1.0.0");
    let entries = corpus["entries"].as_array().unwrap();
    assert_eq!(
        entries
            .iter()
            .filter(|entry| entry.get("raw_fixture").is_some())
            .count(),
        24
    );
    assert_eq!(
        entries
            .iter()
            .map(|entry| entry["id"].as_str().unwrap())
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        entries.len()
    );
    for entry in entries {
        check_corpus_entry(entry);
    }
}
/// MJ2's kill. Genuine n06 v3 twins on 0.3 and 0.4, in both modes, carry case
/// commitments equal to an independent hash of the exact case payload under
/// `pressure_source_case_evidence_v1` (`check_control`), never the v2 domain's.
/// A wrong-domain commitment with resealed outer hashes fails raw and transport.
#[test]
fn pressure_source_case_commitments_use_the_v3_domain() {
    for family in [Family::Physics03, Family::LoadReference04] {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let control = check_control(family, "n06", mode);
            let source = &control.joined.source;
            for (index, case) in source["source_block_recovery"]["body"]["cases"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
            {
                let payload = case_payload(source, index);
                assert_eq!(
                    case["physical_evidence_sha256"],
                    domain_commitment("pressure_source_case_evidence_v1", &payload)
                );
                assert_ne!(
                    case["physical_evidence_sha256"],
                    domain_commitment("physics_source_case_evidence_v1", &payload)
                );
            }
        }
    }
    named_corpus_controls("MJ2-wrong-domain");
}
/// The top-level production functions of a Rust source, comment lines removed.
fn top_level_functions(text: &str) -> Vec<(String, String)> {
    let mut functions: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        let header = ["fn ", "pub fn ", "pub(crate) fn ", "pub(super) fn "]
            .iter()
            .find_map(|prefix| line.strip_prefix(prefix));
        if let Some(rest) = header {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            functions.push((name, String::new()));
        }
        if let Some((_, body)) = functions.last_mut() {
            if !line.trim_start().starts_with("//") {
                body.push_str(line);
                body.push('\n');
            }
        }
    }
    functions
}
/// MJ4's source-qualified guard. Every reader function that dispatches
/// `Method::PressureJoined` or calls the direct v3 load-state check reads the
/// received evidence; none calls the v2 projection/reseal path, and that path's
/// callers remain the v2 joined reader's own (`validate`,
/// `physics_source_projection`). Verifying incoming hashes before a shim does
/// not satisfy this guard.
fn pressure_joined_dispatch_has_no_v2_projection_or_reseal() {
    const READERS: [(&str, &str); 4] = [
        (
            "load_reference_source.rs",
            include_str!("../../../reporting/result_export/src/load_reference_source.rs"),
        ),
        (
            "physics_source.rs",
            include_str!("../../../reporting/result_export/src/physics_source.rs"),
        ),
        (
            "physics_evidence.rs",
            include_str!("../../../reporting/result_export/src/physics_evidence.rs"),
        ),
        (
            "load_reference.rs",
            include_str!("../../../reporting/result_export/src/load_reference.rs"),
        ),
    ];
    let mut direct = std::collections::BTreeSet::new();
    for (file, text) in READERS {
        for (name, body) in top_level_functions(text) {
            if body.contains("Method::PressureJoined")
                || body.contains("validate_pressure_source_evidence(")
            {
                for forbidden in [
                    "project(",
                    "project_load_reference(",
                    "physics_source_projection(",
                    "reseal",
                ] {
                    assert!(
                        !body.contains(forbidden),
                        "{file}:{name} dispatches the direct v3 join but contains {forbidden}"
                    );
                }
                direct.insert(format!("{file}:{name}"));
            }
        }
    }
    for required in [
        "load_reference_source.rs:validate_pressure_source_evidence",
        "physics_source.rs:validate_in",
        "physics_source.rs:validate_transport_metadata_in",
    ] {
        assert!(direct.contains(required), "{required} is a direct dispatch site");
    }
    let projection_callers: std::collections::BTreeSet<_> = top_level_functions(READERS[0].1)
        .into_iter()
        .filter(|(_, body)| {
            let call = body.split_once('\n').map_or("", |(_, rest)| rest);
            call.match_indices("project(").any(|(at, _)| {
                !call[..at].ends_with(|c: char| c.is_alphanumeric() || c == '_' || c == ':')
            })
        })
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        projection_callers,
        ["physics_source_projection".to_owned(), "validate".to_owned()]
            .into_iter()
            .collect(),
        "the v2 projection/reseal path is reached only from the v2 joined reader"
    );
}
/// MJ4's kill. Genuine eigen_motion, mixed and n06 v3 twins pass raw and
/// transport checks with v3 commitments; a stale receipt hash and a coherently
/// resealed changed publication commitment take their named outcomes (corpus);
/// and the source-qualified guard above excludes every v2 projection/reseal
/// call from the direct `Method::PressureJoined` dispatch.
#[test]
fn v3_load_state_join_reads_received_v3_evidence_directly() {
    pressure_joined_dispatch_has_no_v2_projection_or_reseal();
    named_corpus_controls("load_reference_source:");
    named_corpus_controls("MJ4-publication-binding");
}
#[test]
fn source_contract_cross_reads_are_refused_by_name() {
    named_corpus_controls("MJ5-");
    named_corpus_controls("received-captured-state-context");
}
#[test]
fn pressure_source_receipt_cannot_downgrade_to_pressure_1() {
    named_corpus_controls("MJ7-downgrade");
}
#[test]
fn pressure_source_export_is_refused_until_u2() {
    let corpus = read_corpus();
    for entry in corpus["entries"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry.get("raw_fixture").is_some())
    {
        let source = &entry["source"];
        assert_eq!(
            derivative::derive_document(
                json!({}),
                &entry["invocation"]["request"]["model"],
                source,
                json!({}),
                None
            )
            .unwrap_err(),
            derivative::PRESSURE_1_EXPORT_NOT_AVAILABLE
        );
        assert_eq!(
            derivative::validate_document(&json!({}), source).unwrap_err(),
            derivative::PRESSURE_1_EXPORT_NOT_AVAILABLE
        );
    }
}
