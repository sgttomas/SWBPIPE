//! Consequences over genuine public producer outputs. Resealing is used only to
//! construct named negative reader statements, never to fabricate a positive.
//!
//! B7b deleted the v2 contract, so there is no v2/v3 twin left to compare:
//! D5's closed twin exclusions lapsed with v2. The consuming checks now hold
//! each current control to its exact work arithmetic and its committed bytes,
//! and the retired v2 identities and contract are refusal controls.
use super::*;
use open_pipe_stress_product_physics::result_export::derivative;

/// The pressure-source table binding, pinned independently of the reader
/// constant (bytes changed at B7b's resident current table, a43583c625, and
/// at T4-U6's per-basis stress signatures).
const PRESSURE_SOURCE_TABLE_SHA256: &str =
    "609a4e92ac19de0f5a1dd067dab64b4abeb7b3b70511e2095455f1d000d28e2c";

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
/// D5/MJ9's public half, as exact integers: composite publication reserves
/// `24E + P` from its actual encoded captured request `E` and other inputs `P`,
/// and total invocation work is the case work (charged plus reserved
/// unobserved failure) plus publication. The selected-case finalization
/// expression `2(12E + B_k)` is checked at its private site by
/// `pressure_source_private_reservation_inputs_confirm_all_twins_complete_expression`.
fn check_publication_accounting(control: &Control) {
    let body = &control.joined.source["source_block_recovery"]["body"];
    let e = encoded_length(&control.invocation["request"]);
    let p = publication_other_inputs(&control.invocation["request"], &control.joined.source);
    assert_eq!(
        body["invocation_work"]["publication_charged"]
            .as_u64()
            .unwrap(),
        24 * e + p
    );
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
/// b7-plan MJ8 (D6 lapsed with T4-U2's export, #1314): the genuine joined
/// source exports through the derivative, which carries its received v3
/// evidence and receipt whole and refuses a changed, resealed receipt.
fn check_export_binds_the_received_receipt(source: &Value, model: &Value, label: &str) {
    let base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let carrier = derivative::checksum(source, "attested_headless_producer_carrier", derivative::reference("test_carrier", "genuine-pressure-source-producer-output")).unwrap();
    let origin = json!({"origin_id":"pressure-source-export-consequence","origin_class":"attested_headless_producer","qualification_ref":derivative::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Genuine producer twin; consumer consequence test only","actual_model_ref":derivative::reference("model_payload",source["model_ref"].as_str().unwrap_or("")),"mechanics_run_ref":derivative::reference("mechanics_run",source["run_id"].as_str().unwrap_or("")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let doc = derivative::derive_document(base, model, source, origin, None).unwrap_or_else(|e| panic!("{label}: {e}"));
    derivative::validate_document(&doc, source).unwrap_or_else(|e| panic!("{label}: {e}"));
    let envelope = &doc["result_envelope"];
    assert_eq!(envelope["semantic_contract_ref"]["ref_id"], sc::PRESSURE_SOURCE_ID, "{label}");
    assert_eq!(envelope["source_block_recovery"], source["source_block_recovery"], "{label}");
    assert_eq!(envelope["contract_evidence"], source["contract_evidence"], "{label}");
    let mut changed = doc.clone();
    changed["result_envelope"]["source_block_recovery"]["body"]["invocation_work"]["charged"] = json!(
        source["source_block_recovery"]["body"]["invocation_work"]["charged"].as_u64().unwrap() + 1
    );
    reseal_receipt(&mut changed["result_envelope"]);
    assert_eq!(
        derivative::validate_document(&changed, source).unwrap_err(),
        "SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH",
        "{label}"
    );
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
/// Authored presence of the optional load/reference-state group, as the
/// reader keys it (never the version).
fn carries_group(model: &Value) -> bool {
    model.get("reference_configurations").is_some()
        || model["materials"]
            .as_array()
            .is_some_and(|m| m.iter().any(|m| m.get("expansion_laws").is_some()))
        || model["load_cases"]
            .as_array()
            .is_some_and(|c| c.iter().any(|c| c.get("analysis_state").is_some()))
}
#[derive(Clone, Copy)]
enum DispatchDisposition<'a> {
    PressureHealthy(bool),
    PressureFinding(&'a str),
    /// A pressure-source statement whose reading finds `code` and whose
    /// standing token is `needs_recompute` (G2's model-check findings on a
    /// load/reference-state statement, which is never eligible).
    PressureFindingNeedsRecompute(&'a str),
    HeaderRefused(&'a str),
    /// A retired v2 identity, refused by name at header dispatch.
    HeaderRetiredV2,
    /// A current-identity statement whose receipt the header's shape check
    /// refuses: unsupported, under the generic finding.
    HeaderShapeRefused(&'a str),
}
const RETIRED_V2_DETAIL: &str = "SOURCE_EXACT_PRESSURE_V2_RETIRED: 2.0.0/exact_straight_pressure_v2 results are no longer read; re-author the model as 0.9.0 with the current exact pressure contract and re-run the analysis";
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
            json!({"id":"openpipestress.result_semantics/0.3.0/pressure-source-1","sha256":PRESSURE_SOURCE_TABLE_SHA256}),
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
            json!({"id":"openpipestress.result_semantics/0.3.0/pressure-source-1","sha256":PRESSURE_SOURCE_TABLE_SHA256}),
            "unsupported",
            json!([code]),
            Value::Null,
        ),
        DispatchDisposition::PressureFindingNeedsRecompute(code) => (
            "pressure_source",
            json!({"id":"openpipestress.result_semantics/0.3.0/pressure-source-1","sha256":PRESSURE_SOURCE_TABLE_SHA256}),
            "needs_recompute",
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
        DispatchDisposition::HeaderRetiredV2 => (
            "unsupported",
            Value::Null,
            "unsupported",
            json!(["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"]),
            json!({"code":"SOURCE_EXACT_PRESSURE_V2_RETIRED","detail":RETIRED_V2_DETAIL}),
        ),
        DispatchDisposition::HeaderShapeRefused(code) => (
            "unsupported",
            Value::Null,
            "unsupported",
            json!(["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"]),
            json!({"code":code,"detail":null}),
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
            assert_eq!(validation["eligible"], !carries_group(model));
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
/// The family's retired v2 composite identity (received text only).
fn retired_identity(family: Family) -> &'static str {
    if family.resolved() {
        "openpipestress.result_semantics/0.3.0/load-reference-source-1"
    } else {
        "openpipestress.result_semantics/0.3.0/physics-source-1"
    }
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
    // MJ5 (B7b): a genuine statement whose header names the family's retired
    // v2 identity is refused by name at header dispatch, and by the
    // caller-selected current reader on its identity, raw and transported.
    let mut retired = source.clone();
    retired["producer"]["semantic_contract_id"] = json!(retired_identity(control.family));
    entries.push(corpus_entry(
        format!("{prefix}:MJ5-retired-v2-identity"),
        &retired,
        invocation,
        "pressure-source",
        Some("PHYSICS_SOURCE_IDENTITY"),
        Some("PHYSICS_SOURCE_IDENTITY"),
        DispatchDisposition::HeaderRetiredV2,
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
    // MJ5 (B7b): the reader is fixed by its caller. The genuine source offered
    // to the ordinary pressure-2 reader is refused by that reader's own
    // foreign-method check (the receipt), never read as pressure-2.
    entries.push(corpus_entry(
        format!("{prefix}:MJ5-pressure-source-offered-to-pressure-1"),
        source,
        invocation,
        "pressure",
        Some(if control.family.resolved() {
            "SOURCE_LOAD_REFERENCE_FOREIGN_METHOD_EVIDENCE"
        } else {
            "SOURCE_PHYSICS_FOREIGN_METHOD_EVIDENCE"
        }),
        Some(rv::PRESSURE_TRANSPORT_UNSUPPORTED),
        DispatchDisposition::PressureHealthy(control.family.resolved()),
    ));
    let mut relabelled = source.clone();
    relabelled["producer"]["semantic_contract_id"] = json!(retired_identity(control.family));
    relabelled["source_block_recovery"] = json!({});
    entries.push(corpus_entry(
        format!("{prefix}:MJ5-relabelled-shape"),
        &relabelled,
        invocation,
        "pressure-source",
        Some("PHYSICS_SOURCE_IDENTITY"),
        Some("PHYSICS_SOURCE_IDENTITY"),
        DispatchDisposition::HeaderRetiredV2,
    ));
    let mut downgraded = source.clone();
    downgraded["producer"]["semantic_contract_id"] = json!(sc::PRESSURE_ID);
    downgraded["formulation_basis"]["profile_id"] = json!("exact_pressure_v3");
    entries.push(corpus_entry(
        format!("{prefix}:MJ7-downgrade"),
        &downgraded,
        invocation,
        "pressure-source",
        Some("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"),
        Some("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"),
        DispatchDisposition::HeaderRefused("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"),
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
    // MJ5: the expected contract comes from the captured invocation. A
    // captured retired v2 contract is refused by name before any received
    // label or profile is read.
    let mut captured_v2 = invocation.clone();
    captured_v2["request"]["model"]["pressure_contract"] =
        json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"});
    add_context_entry(
        format!("{prefix}:MJ5-captured-v2-context"),
        source,
        &captured_v2,
        "PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED",
        entries,
    );
    // The captured group's presence, not a version, fixes the resolved
    // context: the opposite presence is refused by name.
    let mut other_context = invocation.clone();
    let model = &mut other_context["request"]["model"];
    if control.family.resolved() {
        model.as_object_mut().unwrap().remove("reference_configurations");
        for material in model["materials"].as_array_mut().unwrap() {
            material.as_object_mut().unwrap().remove("expansion_laws");
        }
        for case in model["load_cases"].as_array_mut().unwrap() {
            case.as_object_mut().unwrap().remove("analysis_state");
        }
    } else {
        model["reference_configurations"] = json!([]);
    }
    assert_ne!(carries_group(model), control.family.resolved());
    add_context_entry(
        format!("{prefix}:received-captured-state-context"),
        source,
        &other_context,
        "PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT",
        entries,
    );
    // B7b F1 and O4 (DEL-04-08 Design `o4-source-extension.md` §3.4): every
    // pipe section of the captured exact document declares an explicit
    // length-valued, finite corrosion allowance c_a >= 0 with c_m + c_a < t.
    // A genuine statement coherently bound to an invocation whose allowance is
    // changed (its invocation commitment and receipt resealed to that
    // invocation) is refused on the bound read; the unbound and transported
    // reads cannot see the invocation and read the statement as before
    // (admitted, not numerically eligible; transport passes).
    // - `nonzero` (0.001 m, the statement's sections stating 0 m): a tampered
    //   allowance. O4 admits the value, so the allowance check passes and the
    //   by-value section binding refuses it: ACTUAL_SOURCE_GEOMETRY on a
    //   pre-0.4 statement; on a load/reference-state statement (never
    //   eligible, its materials not read raw) G2's model check,
    //   SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY, with standing needs_recompute.
    // - `absent`, `null` and (O4) `negative`: outside the contract, refused
    //   by the allowance check.
    // The committed J2 corpus regenerates with the O4 limitation slice.
    let tampered = if control.family.resolved() {
        ("SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY", true)
    } else {
        ("PHYSICS_SOURCE_ACTUAL_SOURCE_GEOMETRY", false)
    };
    for (label, allowance, (code, recompute)) in [
        ("nonzero", Some(json!({"value": 0.001, "unit": "m"})), tampered),
        ("absent", None, ("PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE", false)),
        ("null", Some(Value::Null), ("PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE", false)),
        ("negative", Some(json!({"value": -0.001, "unit": "m"})), ("PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE", false)),
    ] {
        let mut captured = invocation.clone();
        for pipe in captured["request"]["model"]["pipe_segments"].as_array_mut().unwrap() {
            let section = pipe["section"].as_object_mut().unwrap();
            match &allowance {
                Some(value) => {
                    section.insert("corrosion_allowance".to_owned(), value.clone());
                }
                None => {
                    section.remove("corrosion_allowance");
                }
            }
        }
        assert_ne!(captured, *invocation);
        let mut bound = source.clone();
        bound["source_block_recovery"]["body"]["invocation"]["value"] =
            json!(domain_commitment("source_blocks_invocation_v1", &captured));
        reseal_receipt(&mut bound);
        add_context_entry_with(
            format!("{prefix}:F1-captured-corrosion-allowance-{label}"),
            &bound,
            &captured,
            code,
            if recompute { DispatchDisposition::PressureFindingNeedsRecompute(code) } else { DispatchDisposition::PressureFinding(code) },
            entries,
        );
        // Coherence: the resealed statement binds the changed invocation, so
        // offered with the genuine (zero-allowance) invocation it fails only
        // the invocation commitment.
        check_refusal(
            &rv::validate("pressure-source", &bound, Some(&invocation["request"]["model"]), Some(invocation)),
            "SOURCE_BLOCKS_INVOCATION_HASH",
        );
    }
    check_export_binds_the_received_receipt(source, &invocation["request"]["model"], &prefix);
}
/// T4-U4b S4 round 2 (§4.3, §5.1): the composite reader recomputes every
/// straight member's derived rows on every case, so each statement below is
/// refused by name though its publication and receipt are resealed. Each
/// change is made on both bases at once (end i), so the bit-equal twin rule of
/// these coincident sections passes and the derived-row checks are reached:
/// - τ one ulp up on the selected (source-recovered) case;
/// - σ⁺, and separately τ, one ulp up on an ordinary-method case;
/// - the station's σ⁺, σ⁻ and τ deleted on the selected case together with
///   their receipt treatments (coverage, not the receipt, finds the hole);
/// - on the mixed control, the σ± twin order broken on each case.
fn add_u4b_entries(control: &Control, entries: &mut Vec<Value>) {
    const EXACT: &str = "retained_source_blocks_exact_v1";
    const EXTREME: &str = "element_local_normal_stress_extreme";
    const SHEAR_MAX: &str = "element_local_transverse_shear_stress_maximum";
    const BASES: [&str; 2] = ["nominal_section", "reduced_section"];
    let prefix = format!("{}:{}:{}", control.family.name(), control.name, control.mode.as_str());
    let source = &control.joined.source;
    let cases = source["source_block_recovery"]["body"]["cases"].as_array().unwrap();
    let case_of = |exact: bool| {
        cases.iter().find(|c| (c["selected_method"] == EXACT) == exact).map(|c| c["basis_ref"]["ref_id"].as_str().unwrap().to_owned())
    };
    let rows_at = |doc: &Value, case: &str, kind: &str, component: &str| -> Vec<usize> {
        BASES.iter().map(|basis| doc["results"].as_array().unwrap().iter().position(|r| {
            r["basis_ref"]["ref_id"] == case && r["kind"] == kind && r["metadata"]["component"] == component
                && r["metadata"]["location"] == "end_i" && r["metadata"]["basis"].as_str().unwrap().ends_with(&format!("; section_basis={basis}"))
        }).unwrap_or_else(|| panic!("{prefix}: {case} {kind} {component} {basis}"))).collect()
    };
    let mut refused = |id: &str, doc: &Value, code: &str| {
        entries.push(corpus_entry(format!("{prefix}:{id}"), doc, &control.invocation, "pressure-source", Some(code), None,
            DispatchDisposition::PressureFinding(code)));
    };
    let tamper = |case: &str, kind: &str, component: &str| {
        let mut doc = source.clone();
        for at in rows_at(&doc, case, kind, component) {
            let value = doc["results"][at]["value"].as_f64().unwrap();
            doc["results"][at]["value"] = json!(value.next_up());
        }
        reseal_publication(&mut doc);
        doc
    };
    let selected = case_of(true).unwrap();
    refused("U4B-selected-tau-both-bases", &tamper(&selected, SHEAR_MAX, "tau_v_max"), "SOURCE_PHYSICS_SHEAR_VALUE");
    if let Some(ordinary) = case_of(false) {
        refused("U4B-ordinary-sigma-plus-both-bases", &tamper(&ordinary, EXTREME, "sigma_plus"), "SOURCE_PHYSICS_EXTREME_VALUE");
        refused("U4B-ordinary-tau-both-bases", &tamper(&ordinary, SHEAR_MAX, "tau_v_max"), "SOURCE_PHYSICS_SHEAR_VALUE");
    }
    let mut deleted = source.clone();
    let ids: Vec<Value> = [(EXTREME, "sigma_plus"), (EXTREME, "sigma_minus"), (SHEAR_MAX, "tau_v_max")]
        .iter()
        .flat_map(|(kind, component)| rows_at(&deleted, &selected, kind, component))
        .map(|at| deleted["results"][at]["id"].clone())
        .collect();
    deleted["results"].as_array_mut().unwrap().retain(|r| !ids.contains(&r["id"]));
    let mut removed = 0;
    for case in deleted["source_block_recovery"]["body"]["cases"].as_array_mut().unwrap() {
        let rows = case["rows"].as_array_mut().unwrap();
        let before = rows.len();
        rows.retain(|t| !ids.contains(&t["result_id"]));
        removed += before - rows.len();
    }
    assert_eq!(removed, 6, "{prefix}: the six treatments go with their rows");
    reseal_publication(&mut deleted);
    refused("U4B-derived-rows-and-treatments-deleted", &deleted, "SOURCE_PHYSICS_EXTREME_COVERAGE");
    // S4 round 3: a false `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN` (the composite's ν is always
    // in domain) appended for one member, whose ten τ rows (five stations, both bases) are deleted
    // with their treatments, on each recovery method's case: the warning is refused by name
    // rather than exempting the member's τ coverage.
    let mut false_warning = |label: &str, case: &str| {
        let mut doc = source.clone();
        let member = doc["results"].as_array().unwrap().iter().find(|r| r["basis_ref"]["ref_id"] == case && r["kind"] == SHEAR_MAX)
            .map(|r| r["entity_ref"].as_str().unwrap().to_owned()).unwrap_or_else(|| panic!("{prefix}: {case} τ"));
        let ids: Vec<Value> = doc["results"].as_array().unwrap().iter()
            .filter(|r| r["basis_ref"]["ref_id"] == case && r["kind"] == SHEAR_MAX && r["entity_ref"] == member.as_str())
            .map(|r| r["id"].clone()).collect();
        assert_eq!(ids.len(), 10, "{prefix}: {case} {member}: five stations on both bases");
        doc["results"].as_array_mut().unwrap().retain(|r| !ids.contains(&r["id"]));
        let mut removed = 0;
        for c in doc["source_block_recovery"]["body"]["cases"].as_array_mut().unwrap() {
            let rows = c["rows"].as_array_mut().unwrap();
            let before = rows.len();
            rows.retain(|t| !ids.contains(&t["result_id"]));
            removed += before - rows.len();
        }
        assert_eq!(removed, 10, "{prefix}: the ten treatments go with their rows");
        doc["diagnostics"].as_array_mut().unwrap().push(json!({
            "id": format!("diagnostic:transverse-shear:{case}:{member}:poisson"),
            "code": "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN", "severity": "warning",
            "message": format!("member {member} has nu = 0.75 outside -1 < nu < 1/2, where the Saint-Venant transverse-shear field is defined; no transverse-shear maximum is published for it in {case}"),
            "source": "core/product_physics", "affected_refs": [case, member]}));
        reseal_publication(&mut doc);
        refused(&format!("U4B-false-poisson-warning-tau-deleted-{label}"), &doc, "SOURCE_PHYSICS_POISSON_DOMAIN_WARNING");
    };
    false_warning("selected", &selected);
    if let Some(ordinary) = case_of(false) {
        false_warning("ordinary-method", &ordinary);
    }
    // Integrated-head review (T4): D-5's row order on the composite. On the
    // mixed control (a selected and an ordinary-method case), the end-i σ⁺
    // reduced row and σ⁻ nominal row exchange positions, values and ids
    // unchanged, publication and receipt resealed: every twin and value still
    // agrees, and only `ROW_BASIS_TWIN_ORDER` finds the reordering.
    if control.name == "mixed" {
        let ordinary = case_of(false).unwrap_or_else(|| panic!("{prefix}: ordinary-method case"));
        for (label, case) in [("selected", &selected), ("ordinary-method", &ordinary)] {
            let mut doc = source.clone();
            let plus = rows_at(&doc, case, EXTREME, "sigma_plus");
            let minus = rows_at(&doc, case, EXTREME, "sigma_minus");
            assert_eq!(plus[1] + 1, minus[0], "{prefix}: {case}: σ⁺ reduced precedes σ⁻ nominal");
            doc["results"].as_array_mut().unwrap().swap(plus[1], minus[0]);
            reseal_publication(&mut doc);
            refused(&format!("U4B-twin-order-sigma-plus-reduced-after-sigma-minus-nominal-{label}"), &doc, "SOURCE_PHYSICS_ROW_BASIS_TWIN_ORDER");
        }
    }
}
/// RF source-method extension §7 (readers judge a receipt against the current
/// producer, which solves exact blocks of order up to 4). On a genuine
/// control, each receipt resealed so that only the named statement is wrong:
/// - a free block of 5 DOFs (blocks merged, the partition kept) and an unknown
///   source `level` are refused by the receipt schema;
/// - a stored `unsupported_block` refusal of order 3 and of order 4 on the
///   ordinary-selected case is unreadable. The schema admits only qualified
///   cases (`failure` null), so the receipt shape refuses it before
///   `FAILURE_BLOCK_ORDER`, whose bound (RF-16) both readers test directly
///   (`source_blocks.rs` `rf16_failure_block_order_tests`;
///   `tests/test_pressure_source_corpus.py`).
///
/// No positive is fabricated: a 4-DOF block or the congruence level reads as
/// valid only on a genuine producer output, which a later producer change adds.
fn add_rf_receipt_entries(control: &Control, entries: &mut Vec<Value>) {
    const SHAPE: &str = "PHYSICS_SOURCE_RECEIPT_SHAPE";
    const EXACT: &str = "retained_source_blocks_exact_v1";
    let prefix = format!(
        "{}:{}:{}",
        control.family.name(),
        control.name,
        control.mode.as_str()
    );
    let source = &control.joined.source;
    let index = |method_is_exact: bool| {
        source["source_block_recovery"]["body"]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .position(|c| (c["selected_method"] == EXACT) == method_is_exact)
            .unwrap()
    };
    let (exact, ordinary) = (index(true), index(false));
    let mut refused = |id: String, mutated: &Value| {
        entries.push(corpus_entry(
            format!("{prefix}:{id}"),
            mutated,
            &control.invocation,
            "pressure-source",
            Some(SHAPE),
            Some(SHAPE),
            DispatchDisposition::HeaderShapeRefused(SHAPE),
        ));
    };
    let mut five = source.clone();
    let plan = &mut five["source_block_recovery"]["body"]["cases"][exact]["source"];
    let (mut merged, mut rest) = (Vec::new(), Vec::new());
    for block in plan["free_blocks"].as_array().unwrap() {
        if merged.len() < 5 {
            merged.extend(block.as_array().unwrap().iter().map(|d| d.as_u64().unwrap()));
        } else {
            rest.push(block.clone());
        }
    }
    assert_eq!(merged.len(), 5, "{prefix}: leading blocks merge to exactly 5 DOFs");
    merged.sort_unstable();
    rest.insert(0, json!(merged));
    plan["free_blocks"] = json!(rest);
    reseal_receipt(&mut five);
    refused("RF-free-block-5-dofs".into(), &five);
    let mut level = source.clone();
    level["source_block_recovery"]["body"]["cases"][exact]["source"]["level"] =
        json!("complete_identified_unknown_member_basis");
    reseal_receipt(&mut level);
    refused("RF-unknown-source-level".into(), &level);
    for order in [3, 4] {
        let mut stored = source.clone();
        let case = &mut stored["source_block_recovery"]["body"]["cases"][ordinary];
        assert!(case["source"].is_null() && case["failure"].is_null());
        let diagnostic = case["ordinary_attempt"]["structural_report_diagnostic_ref"].clone();
        case["outcome"] = json!("unsupported");
        case["selected_method"] = Value::Null;
        case["failure"] = json!({"stage":"exact_solve","code":"unsupported_block","diagnostic_ref":diagnostic,"block_order":order});
        reseal_receipt(&mut stored);
        refused(format!("RF-unsupported-block-order-{order}"), &stored);
    }
}
fn add_context_entry(
    id: String,
    source: &Value,
    invocation: &Value,
    code: &str,
    entries: &mut Vec<Value>,
) {
    add_context_entry_with(id, source, invocation, code, DispatchDisposition::PressureFinding(code), entries)
}
fn add_context_entry_with(
    id: String,
    source: &Value,
    invocation: &Value,
    code: &str,
    disposition: DispatchDisposition<'_>,
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
        "dispatch":checked_dispatch(source,model,invocation,disposition)}}));
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
        // The committed current request is this control's input; it is
        // rewritten only if its value changed, so its authored layout stays.
        let committed: Option<Value> = std::fs::read_to_string(&request)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok());
        if committed.as_ref() != Some(&control.invocation["request"]) {
            std::fs::write(
                &request,
                format!(
                    "{}\n",
                    serde_json::to_string_pretty(&control.invocation["request"]).unwrap()
                ),
            )
            .unwrap();
        }
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
/// RF design §7: genuine public bases for both new congruence publications
/// and the invented sensitive order-4 request. The census requests use the
/// same current-document re-authoring as the lib census; no result is edited.
fn add_rf_source_method_bases(entries: &mut Vec<Value>) {
    let inputs = corpus_storage::expand(serde_json::from_str(include_str!(
        "../../../../fixtures/results/j7_rf_census_inputs.json"
    )).unwrap());
    let mut requests: Vec<(String, Value)> = [
        "RF-SKEW-T-PIN-OFF-345-r1e-08",
        "RF-SKEW-T-PIN-OFF-345-r1e-12",
    ]
    .into_iter()
    .map(|id| {
        let case = inputs["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap();
        (id.to_owned(), current_model::reauthor_request(case["request"].clone()))
    })
    .collect();
    requests.push((
        "order_four_sensitive".into(),
        serde_json::from_str(include_str!("../fixtures/rf_source_method/order_four_sensitive.request.json")).unwrap(),
    ));
    for (id, request) in requests {
        current_model::assert_current_exact_request(&request, &id);
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let joined = produce(&request, mode);
            assert_eq!(joined.source["producer"]["semantic_contract_id"], ps::PRESSURE_CONTRACT_ID, "{id}");
            assert_eq!(joined.source["source_block_recovery"]["body"]["status"], "qualified", "{id}");
            let invocation = json!({"request":request,"solver_mode":mode.as_str()});
            entries.push(corpus_entry(
                format!("rf_source_method:{id}:{}:genuine", mode.as_str()),
                &joined.source,
                &invocation,
                "pressure-source",
                None,
                None,
                DispatchDisposition::PressureHealthy(false),
            ));
        }
    }
}
/// MJ9's kill (B7b meaning): every current control (twelve requests x two
/// modes) keeps its exact work arithmetic, checked as exact integers
/// (`check_publication_accounting`; counters are never ignored or compared
/// with a tolerance), and its exact committed producer bytes. It also keeps
/// the committed raw fixtures and the J2 reader corpus current. Its pre-B7b
/// v2/v3 twin leaf comparison lapsed with v2.
#[test]
fn pressure_source_controls_keep_exact_work_and_current_bytes() {
    let mut entries = Vec::new();
    let mut controls = Vec::new();
    for family in [Family::Physics, Family::LoadReference] {
        for name in family.names() {
            for mode in [
                PreviewSolverMode::SparseInteractive,
                PreviewSolverMode::DenseScrutiny,
            ] {
                let control = check_control(family, name, mode);
                check_publication_accounting(&control);
                current_fixture(&control);
                controls.push(control);
            }
        }
    }
    assert_eq!(controls.len(), 24);
    for control in &controls {
        add_control_entries(control, &mut entries);
        add_u4b_entries(control, &mut entries);
        if control.name == "mixed" {
            add_rf_receipt_entries(control, &mut entries);
        }
        // This consuming kill avoids changing an unrelated resolved-state
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
    add_rf_source_method_bases(&mut entries);
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
        let original = Family::Physics.request("n05");
        let region = Family::Physics.request("mixed")["model"]["load_cases"][1]
            ["pressure_regions"][0]
            .clone();
        for pressure in [-1e-5, 0.0, 1e-5] {
            let mut request = original.clone();
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
    for family in [Family::Physics, Family::LoadReference] {
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
    for family in [Family::Physics, Family::LoadReference] {
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
    assert_eq!(corpus["entries"].as_array().unwrap().len(), 486);
    for _ in 0..8 { assert!(std::ptr::eq(corpus, read_corpus())); }
    std::thread::scope(|scope| {
        let readers: Vec<_> = (0..8).map(|_| scope.spawn(read_corpus)).collect();
        for reader in readers { assert!(std::ptr::eq(corpus, reader.join().unwrap())); }
    });
    println!("pressure_source_corpus_cache=one_shared_immutable_value,486entries");
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
/// MJ2's kill. Genuine n06 controls on both families, in both modes, carry
/// case commitments equal to an independent hash of the exact case payload
/// under `pressure_source_case_evidence_v1` (`check_control`), never the
/// retired v2 domain's. A wrong-domain commitment with resealed outer hashes
/// fails raw and transport.
#[test]
fn pressure_source_case_commitments_use_the_v3_domain() {
    for family in [Family::Physics, Family::LoadReference] {
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
/// received evidence and contains no projection/reseal call. B7b deleted the
/// v2 joined reader with its projection/reseal path, so
/// `load_reference_source.rs` now has no projection caller at all and no
/// reader source defines the v2 projection functions. Verifying incoming
/// hashes before a shim does not satisfy this guard.
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
            assert!(
                !matches!(name.as_str(), "physics_source_projection" | "project_load_reference"),
                "{file}:{name}: the retired v2 projection path is deleted"
            );
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
    assert!(
        projection_callers.is_empty(),
        "no load_reference_source.rs function projects or reseals: {projection_callers:?}"
    );
}
/// MJ4's kill. Genuine eigen_motion, mixed and n06 controls with the group pass
/// raw and transport checks with v3 commitments; a stale receipt hash and a
/// coherently resealed changed publication commitment take their named
/// outcomes (corpus); and the source-qualified guard above excludes every
/// projection/reseal call from the direct `Method::PressureJoined` dispatch.
#[test]
fn v3_load_state_join_reads_received_v3_evidence_directly() {
    pressure_joined_dispatch_has_no_v2_projection_or_reseal();
    named_corpus_controls("load_reference_source:");
    named_corpus_controls("MJ4-publication-binding");
}
/// MJ5's kill (B7b meaning): the expected contract and reader are fixed by
/// the caller or the captured invocation, never inferred from a received
/// label or profile; a retired v2 identity is refused by name
/// (`SOURCE_EXACT_PRESSURE_V2_RETIRED`) and a captured retired contract by the
/// reader's actual-contract refusal.
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
fn pressure_source_export_binds_the_v3_receipt_after_u2() {
    let corpus = read_corpus();
    let mut exported = 0;
    for entry in corpus["entries"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry.get("raw_fixture").is_some())
    {
        check_export_binds_the_received_receipt(
            &entry["source"],
            &entry["invocation"]["request"]["model"],
            entry["id"].as_str().unwrap_or("corpus entry"),
        );
        exported += 1;
    }
    assert_eq!(exported, 24, "every genuine joined publication exports");
}
