//! RT-U4 repair B (single pass): on the pressure route, `dispatch` reads
//! pressure-2's evidence verdict (`validate_pressure_evidence`) once and gives it
//! to both of its uses, the standing's token (`for_source`) and its findings (the
//! pressure reader's outcome). Every other route reads as before.
//!
//! The reference is the basis's two-pass read, verbatim but for its names:
//! `dispatch`, `standing`, `validate`, `numerical_use_standing_with_context` and
//! `for_source`, so it shares none of the changed functions. The basis is the
//! composed predecessor, RT-U4 order (ad3665dcc) with RT-A3 increment 4
//! (4a8287001, §8.50): the order repair leaves those five functions unchanged, so
//! they are increment 4's text, whose `validate` adds the nonlinear supports' model
//! checks to the pressure arm. Merged with RT-A3 §4 (the combination
//! certificate, 268c8289f), the reference takes that line's `validate`: the
//! certificate after the model checks, with the model. Its token's
//! `for_source` runs the reader, and its findings' `validate("pressure", ..)` runs it
//! again; nothing is computed once and fed to both. The pass count is `present`'s
//! (`exact_combination_evidence::PRESSURE_PASSES`, tests only): the reader asks it
//! first, and no other caller does.
//!
//! These tests live here, not in `reader_verdict.rs` or `semantic_contract.rs`,
//! because `product_physics`'s `retained_memory_law_tests` requires
//! `reader_verdict.rs` to carry no test region and audits `semantic_contract.rs`'s;
//! they are declared from `arc_evidence.rs`, which no retained-pricing model pins.
use crate::exact_combination_evidence::PRESSURE_PASSES;
use crate::reader_verdict::{
    bind_preview_shear_model, binding_of, dispatch, error_json, ordinary_findings, physics_model_checks, present, route,
    source_findings, source_recovery, source_validate_in, verdict_refusal, LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE,
    RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE, SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED,
    SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED, UNSUPPORTED,
};
use crate::semantic_contract::{
    self as sc, for_source_metadata, forbid_retained_rows, is_retained, retained_contract, retained_error, retained_standing,
    validate_preview_physics_evidence, validate_pressure_evidence, PRESSURE_ID, PRESSURE_SOURCE_ID, PREVIEW_PHYSICS_ID,
    PREVIEW_PHYSICS_TIMOSHENKO_ID,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

// ---- the reference: the basis's two-pass read --------------------------------------------------

fn reference_dispatch(source: &Value, model: Option<&Value>, invocation: Option<&Value>) -> Value {
    let (route_name, refusal) = match route(source) {
        Ok((route, _)) => (route, Value::Null),
        Err(code) => (UNSUPPORTED, error_json(&code)),
    };
    json!({"ok": refusal.is_null(), "route": route_name, "binding": binding_of(route_name, source),
        "error": refusal, "standing": reference_standing(source, model, invocation)})
}
fn reference_standing(source: &Value, model: Option<&Value>, invocation: Option<&Value>) -> Value {
    let (model, invocation) = (present(model), present(invocation));
    let requested_ids: Vec<&Value> = model
        .and_then(|m| m["load_cases"].as_array())
        .map(|cases| cases.iter().map(|c| &c["id"]).collect())
        .unwrap_or_default();
    let requested_refs: Vec<Value> = requested_ids
        .iter()
        .map(|id| json!({"ref_type": "load_case", "ref_id": id}))
        .collect();
    let token = reference_numerical_use_standing_with_context(source, &requested_refs, invocation);
    let mut findings: Vec<String> = Vec::new();
    let route = match route(source) {
        Ok((route, _)) => route,
        Err(code) => {
            findings.push(
                if matches!(code.as_str(), sc::RETAINED_PRECISION_DOWNGRADE_FORBIDDEN | "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN") || code == sc::MECHANICS_RESULT_VERSION_RETIRED { code } else { SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED.into() },
            );
            UNSUPPORTED
        }
    };
    let done = |findings: Vec<String>| {
        let eligible = findings.is_empty();
        json!({"route": route, "token": token, "status": if eligible { "integrity_checked" } else { "needs_recompute" },
            "eligible": eligible, "findings": findings})
    };
    match route {
        UNSUPPORTED => {}
        "pressure_source" => match invocation {
            None => findings.push(SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED.into()),
            Some(i) => {
                let contract = crate::physics_source::SourceContract::PressureV3;
                match source_validate_in(source, Some(i), contract) {
                    Err(code) => findings.push(code),
                    Ok(_) => {
                        findings.extend(source_findings(source, i).into_iter().map(String::from));
                        if source["contract_evidence"].get("load_reference_states").is_some() {
                            findings.push(LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE.into());
                        }
                    }
                }
            }
        },
        "retained_preview_physics" | "retained_physics" => {
            let eligible = token == "numerically_eligible";
            if !eligible {
                findings.push(match crate::retained_precision::validate(source, invocation) {
                    Err(e) => e.code,
                    Ok(_) => RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE.into(),
                });
            }
            return done(findings);
        }
        _ => {
            let checked = match route {
                "pressure" => Some(reference_validate("pressure", source, model, None)),
                "preview_physics" => Some(reference_validate("preview-physics", source, model, None)),
                _ => None,
            };
            if let Some(outcome) = checked.filter(|o| o["ok"] == false) {
                let error = &outcome["error"];
                findings.push(error["detail"].as_str().or(error["code"].as_str()).unwrap_or_default().into());
            }
            findings.extend(ordinary_findings(source, &requested_ids));
        }
    }
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
        findings.push("MECHANICS_NOT_SOLVED".into());
    }
    done(findings)
}
fn reference_validate(reader: &str, source: &Value, model: Option<&Value>, invocation: Option<&Value>) -> Value {
    let (model, invocation) = (present(model), present(invocation));
    let checked = |check: Result<(), String>| match check {
        Ok(()) => json!({"ok": true}),
        Err(e) => verdict_refusal(&e),
    };
    match reader {
        "source-recovery" => checked(source_recovery(source)),
        // with the captured model, τ bit for bit (G2).
        // with the model, a crossed identity is refused first.
        // RT-A4 (review r1 P2): with the model, the combination gates and unsigned rows
        // are re-derived from it; without, refused where the document cannot verify them.
        "preview-physics" => checked(crate::preview_physics_evidence::validate_preview_physics_evidence_with_model(source, model).and_then(|()| {
            // the operand records bound to the model's combinations.
            model.map_or(Ok(()), |m| {
                bind_preview_shear_model(source, m)
                    .and_then(|()| crate::u4b_evidence::preview_model_checks(source, m))
                    .and_then(|()| crate::preview_combination_operands::model_checks(source, m))
                    // RT-A4 (review r1 P2): the gates and unsigned rows, from the model.
                    .and_then(|()| crate::preview_combination_gates::check(source, Some(m)))
                    // the certificate's records bound to the model's combinations.
                    .and_then(|()| crate::combination_certificate::model_checks(source, m))
                    // the certificate last, after every other check.
                    .and_then(|()| crate::combination_certificate::check_with(source, Some(m)))
            })
        })),
        // Current pressure-2 reader, then G2's model checks,
        // which read only the cases, members, connectors and support rows.
        "pressure" => checked(match model {
            None => sc::validate_pressure_evidence(source),
            // with the model, the certificate is read after the model checks.
            Some(_) => crate::physics_evidence::validate_pressure_evidence_without_certificate(source),
        }.and_then(|()| {
            // W-16 slice b (§6): the combination records bound to the model's
            // combinations and its operands' modulus selections.
            // RT-A3 increment 4 (R-4 §2.4): the nonlinear supports bound to the
            // model's records and the DEC-067 laws that need their inputs.
            model.map_or(Ok(()), |m| physics_model_checks(source, m).and_then(|()| crate::u4b_evidence::exact_model_checks(source, m))
                .and_then(|()| crate::exact_combination_evidence::model_checks(source, m))
                .and_then(|()| crate::nonlinear_support_evidence::model_checks(source, m))
                .and_then(|()| crate::combination_certificate::model_checks(source, m))
                .and_then(|()| crate::combination_certificate::check_with(source, Some(m))))
        })),
        "pressure-source" => {
            let contract = crate::physics_source::SourceContract::PressureV3;
            match source_validate_in(source, invocation, contract) {
                Ok(eligible) => {
                    let findings = invocation
                        .map_or_else(Vec::new, |i| source_findings(source, i));
                    json!({"ok": true, "eligible": eligible, "findings": findings})
                }
                Err(e) => verdict_refusal(&e),
            }
        }
        other => verdict_refusal(&format!("READER_UNKNOWN: {other}")),
    }
}
fn reference_numerical_use_standing_with_context(
    source: &Value,
    requested_basis_refs: &[Value],
    actual_invocation: Option<&Value>,
) -> &'static str {
    if is_retained(source) {
        // Validated once, with the invocation; G7 includes the base dispatch.
        return retained_standing(source, requested_basis_refs, actual_invocation);
    }
    let Ok((_, version)) = reference_for_source(source) else {
        return "unsupported";
    };
    // T0R (A2 item 10): validate first. (B1 and B4/D1 (b) refuse precision-1
    // and source-blocks-1 by name, so no readable source has a standing reason.)
    if version != "0.3.0" || requested_basis_refs.is_empty() {
        return "needs_recompute";
    }
    if matches!(
        source["producer"]["semantic_contract_id"].as_str(),
        Some(PRESSURE_SOURCE_ID)
    ) {
        let expected = source["source_block_recovery"]["body"]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["basis_ref"].clone())
            .collect::<Vec<_>>();
        if expected != requested_basis_refs {
            return "needs_recompute";
        }
        let standing = crate::physics_source::validate_in(source, actual_invocation, crate::physics_source::SourceContract::PressureV3);
        return match standing {
            Ok(true) => "numerically_eligible",
            Ok(false) => "needs_recompute",
            Err(_) => "unsupported",
        };
    }
    let q = &source["numerical_quality"];
    // Sensitive backward-error evidence is retained for inspection, not qualified use.
    if q["status"] != "checks_passed" {
        return "needs_recompute";
    }
    let cases = q["cases"].as_array().unwrap();
    if cases.len() != requested_basis_refs.len() {
        return "needs_recompute";
    }
    let mut ids = std::collections::HashSet::new();
    for key in ["results", "diagnostics"] {
        let Some(items) = source[key].as_array() else {
            return "needs_recompute";
        };
        for item in items {
            let Some(id) = item["id"].as_str().filter(|id| !id.is_empty()) else {
                return "needs_recompute";
            };
            if !ids.insert(id) {
                return "needs_recompute";
            }
        }
    }
    for (index, basis) in requested_basis_refs.iter().enumerate() {
        if requested_basis_refs[..index].contains(basis) {
            return "needs_recompute";
        }
        let matching: Vec<_> = cases
            .iter()
            .filter(|case| case["basis_ref"] == *basis)
            .collect();
        if matching.len() != 1 {
            return "needs_recompute";
        }
        let case = matching[0];
        if case["solve_quality"] != "checks_passed"
            || case["structural_status"] != "passive_model_basis"
            || case["model_matrix_fidelity"] != "represented_equations_retained"
            || !matches!(
                case["accuracy_evidence"].as_str(),
                Some("not_claimed" | "reference_verified")
            )
            || !case["evidence_refs"].as_array().is_some_and(|refs| {
                !refs.is_empty()
                    && refs
                        .iter()
                        .all(|r| r.as_str().is_some_and(|id| ids.contains(id)))
            })
        {
            return "needs_recompute";
        }
    }
    "numerically_eligible"
}
fn reference_for_source(source: &Value) -> Result<(&'static Value, &'static str), String> {
    if is_retained(source) {
        // The accepted reader, G0-G7 without an invocation. Its G7 runs this
        // function's unchanged base branch on its own projection, which drops
        // the receipt and the method token, so this branch is not re-entered.
        crate::retained_precision::validate(source, None).map_err(retained_error)?;
        return Ok((retained_contract(source), "0.3.0"));
    }
    let selected = for_source_metadata(source)?;
    forbid_retained_rows(source)?;
    match source["producer"]["semantic_contract_id"].as_str() {
        Some(PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID) => validate_preview_physics_evidence(source)?,
        Some(PRESSURE_ID) => validate_pressure_evidence(source)?,
        Some(PRESSURE_SOURCE_ID) => {
            crate::physics_source::validate_in(source, None, crate::physics_source::SourceContract::PressureV3)?;
        }
        _ => {
            if source
                .get("contract_evidence")
                .is_some_and(|v| !v.is_null())
                || source.get("source_block_recovery").is_some()
                || source.get("carrier_evidence").is_some()
            {
                return Err("SOURCE_EVIDENCE_CONTRACT_UNSUPPORTED".into());
            }
        }
    }
    Ok(selected)
}

// ---- the inputs: committed pressure-2 reader envelopes and their mutations ---------------------
// Invented data only: every input is a committed fixture or a deterministic edit of one. The
// mutations are refused under many codes, alone and in competition (two defects, both orders), and
// include #105's competing connector and global end defects.
struct Input {
    label: String,
    source: Value,
    model: Option<Value>,
}

fn corpus(text: &str, model_key: Option<&str>) -> Vec<(String, Value, Option<Value>)> {
    let corpus: Value = serde_json::from_str(text).unwrap();
    corpus["cases"].as_array().unwrap().iter().map(|case| {
        // The arc corpus keeps each model inside its request (`document.model`).
        let model = model_key.and_then(|k| case.get(k)).map(|m| if m.get("model").is_some_and(Value::is_object) { &m["model"] } else { m })
            .filter(|m| m.is_object()).cloned();
        (case["label"].as_str().unwrap().to_string(), case["envelope"].clone(), model)
    }).collect()
}

fn bases() -> Vec<(String, Value, Option<Value>)> {
    let mut out = Vec::new();
    out.extend(corpus(include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"), None));
    out.extend(corpus(include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json"), Some("document")));
    out.extend(corpus(include_str!("../../../../fixtures/results/pressure_v3_connector_reader_corpus.json"), None));
    out.extend(corpus(include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json"), Some("model")));
    let joint: Value = serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_joint_reader_control.json")).unwrap();
    out.push(("joint reader control".into(), joint["envelope"].clone(), None));
    out.push(("U4b arc sparse".into(), serde_json::from_str(include_str!("../../../../fixtures/results/pressure_u4b_arc_sparse.json")).unwrap(), None));
    let (source, model) = nonlinear_base();
    out.push(("RT-A3 nonlinear B1 loaded gap".into(), source, Some(model)));
    out
}

/// RT-A3 increment 4's committed nonlinear pressure-2 envelope (a gap support that ends
/// inactive under load) and its model: the pass's nonlinear support binding
/// (`nonlinear_support_evidence::check`) and the pressure arm's model checks run on it.
fn nonlinear_base() -> (Value, Value) {
    let fixture: Value = serde_json::from_str(include_str!("../../../product_physics/tests/fixtures/rt_a3_nonlinear/b1_loaded_inactive_reaction.json")).unwrap();
    (fixture["source"].clone(), fixture["model"].clone())
}

/// The nonlinear support defects, alone and in competition with an ordinary row defect
/// (both row orders), and the model side's nonlinear records.
fn nonlinear_mutations() -> Vec<(String, Value, Option<Value>)> {
    let (base, model) = nonlinear_base();
    let at = |s: &Value, kind: &str| s["results"].as_array().unwrap().iter().position(|r| r["kind"] == kind).unwrap_or_else(|| panic!("no {kind} row"));
    let label = "RT-A3 nonlinear B1";
    let mut sources: Vec<(String, Value)> = Vec::new();
    let reaction = at(&base, "nonlinear_support_final_reaction");
    let perturb = |s: &mut Value, i: usize| {
        let v = rows(s)[i]["value"].as_f64().unwrap();
        rows(s)[i]["value"] = json!(v * 1.5 + 1000.0);
    };
    sources.push((format!("{label} / reaction perturbed"), edited(&base, |s| perturb(s, reaction))));
    sources.push((format!("{label} / state code changed"), edited(&base, |s| {
        let i = at(s, "nonlinear_support_active_set_state_code");
        rows(s)[i]["value"] = json!(1);
    })));
    sources.push((format!("{label} / not converged"), edited(&base, |s| {
        let i = at(s, "nonlinear_support_active_set_converged_flag");
        rows(s)[i]["value"] = json!(0);
    })));
    for kind in ["nonlinear_support_active_set_iteration_count", "nonlinear_support_final_displacement", "nonlinear_support_observed_free_dof_force_residual"] {
        sources.push((format!("{label} / no {kind}"), edited(&base, |s| { let i = at(s, kind); rows(s).remove(i); })));
    }
    sources.push((format!("{label} / state row twice"), edited(&base, |s| {
        let i = at(s, "nonlinear_support_active_set_state_code");
        let mut twin = rows(s)[i].clone();
        twin["id"] = json!("result:probe:state-twin");
        rows(s).push(twin);
    })));
    sources.push((format!("{label} / no state review"), edited(&base, |s| {
        s["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != "NONLINEAR_SUPPORT_STATE_REVIEW");
    })));
    // Competing: the nonlinear reaction perturbed and an ordinary row removed, in both row orders.
    let far = if reaction > base["results"].as_array().unwrap().len() / 2 { 0 } else { base["results"].as_array().unwrap().len() - 1 };
    let competing = edited(&base, |s| { perturb(s, reaction); rows(s).remove(far); });
    sources.push((format!("{label} / reaction perturbed, row {far} removed, reversed"), edited(&competing, |s| rows(s).reverse())));
    sources.push((format!("{label} / reaction perturbed, row {far} removed"), competing));
    let mut out: Vec<(String, Value, Option<Value>)> = sources.into_iter().map(|(l, s)| (l, s, Some(model.clone()))).collect();
    // The model's nonlinear records (`nonlinear_support_evidence::model_checks`).
    let edits: [(&str, fn(&mut Value)); 3] = [
        ("model gap widened", |m| for s in m["supports"].as_array_mut().unwrap() { if s["nonlinear"].is_object() { s["nonlinear"]["gap"]["value"] = json!(2.0) } }),
        ("model gap now one-way", |m| for s in m["supports"].as_array_mut().unwrap() { if s["nonlinear"].is_object() { s["nonlinear"]["behavior"] = json!("one_way") } }),
        ("model without its nonlinear support", |m| m["supports"].as_array_mut().unwrap().retain(|s| !s["nonlinear"].is_object())),
    ];
    for (name, edit) in edits {
        out.push((format!("{label} / {name}"), base.clone(), Some(edited(&model, edit))));
    }
    out
}

fn rows(source: &mut Value) -> &mut Vec<Value> {
    source["results"].as_array_mut().unwrap()
}

/// About `per` evenly spaced row positions of `n`.
fn stride(n: usize, per: usize) -> Vec<usize> {
    if n == 0 {
        return Vec::new();
    }
    let step = (n / per).max(1);
    (0..n).step_by(step).take(per).collect()
}

fn edited(base: &Value, edit: impl FnOnce(&mut Value)) -> Value {
    let mut source = base.clone();
    edit(&mut source);
    source
}

/// The mutations of one base envelope: single and competing row defects, evidence and
/// numerical-quality defects, and row order changes.
fn mutations(label: &str, base: &Value, per: usize) -> Vec<(String, Value)> {
    let mut out: Vec<(String, Value)> = Vec::new();
    let n = base["results"].as_array().unwrap().len();
    let positions = stride(n, per);
    for &i in &positions {
        out.push((format!("{label} / remove row {i}"), edited(base, |s| { rows(s).remove(i); })));
        out.push((format!("{label} / perturb row {i}"), edited(base, |s| {
            let v = rows(s)[i]["value"].as_f64().unwrap_or(0.0);
            rows(s)[i]["value"] = json!(v * 1.5 + 1000.0);
        })));
        out.push((format!("{label} / unknown entity row {i}"), edited(base, |s| { rows(s)[i]["entity_ref"] = json!("pipe:probe-unknown"); })));
        out.push((format!("{label} / unknown component row {i}"), edited(base, |s| { rows(s)[i]["metadata"]["component"] = json!("probe_component"); })));
        if i > 0 {
            out.push((format!("{label} / duplicate id row {i}"), edited(base, |s| {
                let id = rows(s)[i - 1]["id"].clone();
                rows(s)[i]["id"] = id;
            })));
        }
    }
    // Competing defects: two removals far apart, in the array order and reversed.
    for (a, b) in positions.iter().zip(positions.iter().rev()).take(per / 2) {
        let (a, b) = (*a.min(b), *a.max(b));
        if a == b {
            continue;
        }
        out.push((format!("{label} / remove rows {a} and {b}"), edited(base, |s| { rows(s).remove(b); rows(s).remove(a); })));
        out.push((format!("{label} / remove rows {a} and {b}, reversed"), edited(base, |s| { rows(s).remove(b); rows(s).remove(a); rows(s).reverse(); })));
        out.push((format!("{label} / perturb {a}, remove {b}"), edited(base, |s| {
            rows(s).remove(b);
            let v = rows(s)[a]["value"].as_f64().unwrap_or(0.0);
            rows(s)[a]["value"] = json!(v * 1.5 + 1000.0);
        })));
    }
    out.push((format!("{label} / rows reversed"), edited(base, |s| rows(s).reverse())));
    // The evidence object, one member at a time, and each key of the first exact case.
    for key in base["contract_evidence"].as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()).unwrap_or_default() {
        out.push((format!("{label} / no evidence {key}"), edited(base, |s| { s["contract_evidence"].as_object_mut().unwrap().remove(&key); })));
    }
    for key in base["contract_evidence"]["exact_cases"][0].as_object().map(|o| o.keys().cloned().collect::<Vec<_>>()).unwrap_or_default() {
        out.push((format!("{label} / first case without {key}"), edited(base, |s| {
            s["contract_evidence"]["exact_cases"][0].as_object_mut().unwrap().remove(&key);
        })));
    }
    if base["contract_evidence"]["pressure"].as_array().is_some_and(|r| !r.is_empty()) {
        out.push((format!("{label} / region p perturbed"), edited(base, |s| {
            let p = s["contract_evidence"]["pressure"][0]["p_pa"].as_f64().unwrap_or(0.0);
            s["contract_evidence"]["pressure"][0]["p_pa"] = json!(p * 2.0 + 1.0);
        })));
        out.push((format!("{label} / region result id dropped"), edited(base, |s| {
            if let Some(ids) = s["contract_evidence"]["pressure"][0]["result_ids"].as_array_mut() { ids.pop(); }
        })));
        out.push((format!("{label} / region member unknown"), edited(base, |s| {
            if let Some(ids) = s["contract_evidence"]["pressure"][0]["member_pipe_ids"].as_array_mut() { ids.push(json!("pipe:probe-unknown")); }
        })));
    }
    // Numerical quality: the ordinary findings and the token.
    out.push((format!("{label} / quality sensitive"), edited(base, |s| s["numerical_quality"]["status"] = json!("sensitive"))));
    out.push((format!("{label} / first case failed"), edited(base, |s| s["numerical_quality"]["cases"][0]["solve_quality"] = json!("failed"))));
    out.push((format!("{label} / first case no refs"), edited(base, |s| s["numerical_quality"]["cases"][0]["evidence_refs"] = json!([]))));
    out.push((format!("{label} / quality case unknown"), edited(base, |s| s["numerical_quality"]["cases"][0]["basis_ref"]["ref_id"] = json!("case:probe-unknown"))));
    out.push((format!("{label} / not solved"), edited(base, |s| s["status"]["mechanics"] = json!("MECHANICS_NOT_SOLVED"))));
    // Header refusals on the same identity: no evidence object, a carrier namespace.
    out.push((format!("{label} / evidence null"), edited(base, |s| s["contract_evidence"] = Value::Null)));
    out.push((format!("{label} / carrier evidence"), edited(base, |s| s["carrier_evidence"] = json!({}))));
    out
}

/// #105's competing connector defects (connector_order_tests.rs: W16B MO-F with a second record),
/// and the earlier probe's connector duplicate + unbound rows.
fn connector_competitions() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json")).unwrap();
    let mut two = corpus["cases"].as_array().unwrap().iter()
        .find(|c| c["label"] == "W16B MO-F: two tie rods withheld by name").unwrap()["envelope"].clone();
    let mut second = two["contract_evidence"]["connector"][0].clone();
    second["component_id"] = json!("component:C-K");
    second["replaced_pipe_id"] = json!("pipe:K");
    two["contract_evidence"]["connector"].as_array_mut().unwrap().push(second);
    let cloned: Vec<Value> = two["results"].as_array().unwrap().iter().filter(|r| r["entity_ref"] == "component:C-J").map(|r| {
        let mut row = r.clone();
        row["entity_ref"] = json!("component:C-K");
        row["id"] = json!(r["id"].as_str().unwrap().replace("component-C-J", "component-C-K"));
        row
    }).collect();
    rows(&mut two).extend(cloned);
    let remove = |s: &mut Value, case: &str, component: &str, tail: &str| {
        let qualifier = if case == "load:U5A" { String::new() } else { format!("loadcase:{}:", case.replace(':', "-")) };
        let id = format!("result:{qualifier}connector:{component}:{tail}");
        let at = rows(s).iter().position(|r| r["id"] == id.as_str()).unwrap_or_else(|| panic!("no row {id}"));
        rows(s).remove(at);
    };
    const ROD: &str = "tie-rod-force:tie_rod:rod-yp:t_r";
    const END_FX: &str = "connector-endpoint-force:end_i:fx";
    let mut out = vec![("two connectors".to_string(), two.clone())];
    let within = edited(&two, |s| { remove(s, "load:U5A", "component-C-J", ROD); remove(s, "load:U5A", "component-C-K", END_FX); });
    let across = edited(&two, |s| { remove(s, "load:U5A", "component-C-K", END_FX); remove(s, "case:second", "component-C-J", ROD); });
    for (name, s) in [("rod on C-J, end on C-K", within), ("C-K first case, C-J second case", across)] {
        out.push((format!("two connectors / {name}"), s.clone()));
        out.push((format!("two connectors / {name}, records reversed"), edited(&s, |s| s["contract_evidence"]["connector"].as_array_mut().unwrap().reverse())));
        out.push((format!("two connectors / {name}, rows reversed"), edited(&s, |s| rows(s).reverse())));
    }
    let connector: Value = serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_connector_reader_corpus.json")).unwrap();
    let base = connector["cases"][0]["envelope"].clone();
    let conn: Vec<Value> = base["results"].as_array().unwrap().iter().filter(|r| r["kind"] == "connector_generalized_force_v1").cloned().collect();
    out.push(("connector dup + unbound".into(), edited(&base, |s| {
        let mut dup = conn[0].clone();
        dup["id"] = json!("result:probe:dup");
        let mut unbound = conn[1].clone();
        unbound["id"] = json!("result:probe:unbound");
        unbound["entity_ref"] = json!("component:probe-unknown");
        rows(s).push(dup);
        rows(s).push(unbound);
    })));
    out.push(("connector unbound + dup, reversed".into(), edited(&out.last().unwrap().1, |s| rows(s).reverse())));
    out
}

/// The earlier probe's global end competition: member A loses a global end row (COVERAGE), member
/// B's global end row is perturbed (ROTATION_NORM), on every straight and arc corpus envelope with
/// two global-end members, in both row orders.
fn global_end_competitions() -> Vec<(String, Value)> {
    let mut out = Vec::new();
    for (label, base, _) in bases().into_iter().take(10) {
        let global = |r: &Value| r["kind"].as_str().is_some_and(|k| k.contains("global_end"));
        let mut members: Vec<String> = base["results"].as_array().unwrap().iter().filter(|r| global(r))
            .filter_map(|r| r["entity_ref"].as_str().map(String::from)).collect();
        members.sort();
        members.dedup();
        if members.len() < 2 {
            continue;
        }
        for (a, b) in [(0, 1), (1, 0), (0, members.len() - 1)] {
            if a == b {
                continue;
            }
            let (ma, mb) = (members[a].clone(), members[b].clone());
            let s = edited(&base, |s| {
                let ia = rows(s).iter().position(|r| r["entity_ref"] == ma.as_str() && global(r)).unwrap();
                rows(s).remove(ia);
                let ib = rows(s).iter().position(|r| r["entity_ref"] == mb.as_str() && global(r)).unwrap();
                let v = rows(s)[ib]["value"].as_f64().unwrap();
                rows(s)[ib]["value"] = json!(v * 1.5 + 1000.0);
            });
            out.push((format!("{label} / global end {ma} missing, {mb} off"), s.clone()));
            out.push((format!("{label} / global end {ma} missing, {mb} off, reversed"), edited(&s, |s| rows(s).reverse())));
        }
    }
    out
}

/// Every input: each base with and without its model, each mutation with its base's model and
/// without, and the competitions.
fn inputs(per: usize) -> Vec<Input> {
    let mut out = Vec::new();
    for (label, base, model) in bases() {
        out.push(Input { label: label.clone(), source: base.clone(), model: model.clone() });
        for (l, s) in mutations(&label, &base, per) {
            out.push(Input { label: l, source: s, model: model.clone() });
        }
        // The model's side (G2's model checks after an admitted reader): a case,
        // a support and a pipe the source does not state.
        let Some(model) = model else { continue };
        let edits: [(&str, fn(&mut Value)); 3] = [
            ("model with an extra load case", |m| if let Some(cases) = m["load_cases"].as_array_mut() { cases.push(json!({"id": "load:probe-extra"})) }),
            ("model support renamed", |m| if let Some(s) = m["supports"].get_mut(0) { s["id"] = json!("support:probe-renamed") }),
            ("model pipe renamed", |m| if let Some(p) = m["pipe_segments"].get_mut(0) { p["id"] = json!("pipe:probe-renamed") }),
        ];
        for (name, edit) in edits {
            out.push(Input { label: format!("{label} / {name}"), source: base.clone(), model: Some(edited(&model, edit)) });
        }
    }
    for (l, s) in connector_competitions().into_iter().chain(global_end_competitions()) {
        out.push(Input { label: l, source: s, model: None });
    }
    for (label, source, model) in nonlinear_mutations() {
        out.push(Input { label, source, model });
    }
    out
}

// ---- counting and comparison ------------------------------------------------------------------

/// `read`'s result and the pressure-2 reader passes it ran on this thread.
fn counted<R>(read: impl FnOnce() -> R) -> (R, usize) {
    PRESSURE_PASSES.with(|n| n.set(0));
    let result = read();
    (result, PRESSURE_PASSES.with(|n| n.get()))
}
fn bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).unwrap()
}
/// The standing's requested refs: the model's load cases, as `standing` forms them.
fn requested(model: Option<&Value>) -> Vec<Value> {
    model.and_then(|m| m["load_cases"].as_array()).map_or_else(Vec::new, |cases| {
        cases.iter().map(|c| json!({"ref_type": "load_case", "ref_id": c["id"]})).collect()
    })
}
fn fixture(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// The counter is the reader's passes: one per call, with and without a
/// combination (whose direct pass reads a view of the document), refused or not.
#[test]
fn the_counter_counts_one_per_reader_pass() {
    let mut combination = 0;
    for input in inputs(2) {
        let (_, n) = counted(|| sc::validate_pressure_evidence(&input.source));
        assert_eq!(n, 1, "{}", input.label);
        combination += usize::from(input.source["contract_evidence"].get("exact_combinations").is_some());
    }
    assert!(combination > 0);
    let preview = fixture(include_str!("../../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json"));
    assert_eq!(counted(|| sc::validate_preview_physics_evidence(&preview)).1, 0);
}

/// The single pass against the two-pass reference over the committed reader
/// corpora and their mutations, each with its model and without: the whole
/// serialized dispatch outcome (route, binding, header refusal, and the standing's
/// token, status, eligibility and findings in order) is the reference's, byte for
/// byte; a second read is the same bytes; the token is the standalone
/// numerical-use token; and the pressure route takes one pass where the reference
/// took two. A pressure-2 source the header refuses keeps its passes (its token's
/// read, when that reaches the reader).
#[test]
fn dispatch_is_the_two_pass_reference_with_one_pass() {
    let inputs = inputs(12);
    let mut first_codes: BTreeMap<String, usize> = BTreeMap::new();
    let mut tokens: BTreeMap<String, usize> = BTreeMap::new();
    let (mut pressure_reads, mut header_refused, mut modeled, mut several) = (0, 0, 0, 0);
    // RT-A3 increment 4's nonlinear inputs and the nonlinear codes they reach first.
    let (mut nonlinear_reads, mut nonlinear_codes) = (0, std::collections::BTreeSet::new());
    for input in &inputs {
        for model in std::iter::once(None).chain(input.model.as_ref().map(Some)) {
            let what = format!("{} (model: {})", input.label, model.is_some());
            let (reference, before) = counted(|| reference_dispatch(&input.source, model, None));
            let (head, after) = counted(|| dispatch(&input.source, model, None));
            assert_eq!(String::from_utf8(bytes(&head)).unwrap(), String::from_utf8(bytes(&reference)).unwrap(), "{what}");
            assert_eq!(bytes(&dispatch(&input.source, model, None)), bytes(&head), "{what}: a second read");
            assert_eq!(head["standing"]["token"], sc::numerical_use_standing_with_context(&input.source, &requested(model), None), "{what}");
            if head["route"] == "pressure" {
                assert_eq!((before, after), (2, 1), "{what}");
                pressure_reads += 1;
            } else {
                // The header refused a pressure-2 source: the token's own read, if it reaches the reader.
                assert!(before == after && after <= 1, "{what}: {before} then {after}");
                header_refused += 1;
            }
            modeled += usize::from(model.is_some());
            let findings = head["standing"]["findings"].as_array().unwrap();
            several += usize::from(findings.len() > 1);
            let first = findings.first().and_then(Value::as_str).map_or("none".into(), |f| error_json(f)["code"].as_str().unwrap().to_string());
            if input.label.starts_with("RT-A3 nonlinear") {
                nonlinear_reads += 1;
                if first.contains("NONLINEAR") {
                    nonlinear_codes.insert(first.clone());
                }
            }
            *first_codes.entry(first).or_default() += 1;
            *tokens.entry(head["standing"]["token"].as_str().unwrap().into()).or_default() += 1;
        }
    }
    println!("single_pass_agreement inputs={} pressure_reads={pressure_reads} header_refused={header_refused} with_model={modeled} \
        several_findings={several} first_codes={} tokens={tokens:?} nonlinear_reads={nonlinear_reads} nonlinear_codes={nonlinear_codes:?}", inputs.len(), first_codes.len());
    assert!(pressure_reads > 3000 && header_refused > 0 && modeled > 1000 && several > 0);
    assert!(first_codes.len() >= 30, "{first_codes:?}");
    assert!(nonlinear_reads > 50 && nonlinear_codes.len() >= 3, "{nonlinear_reads} {nonlinear_codes:?}");
    assert!(["numerically_eligible", "needs_recompute", "unsupported"].iter().all(|t| tokens.contains_key(*t)), "{tokens:?}");
}

/// The token's `for_source` reads the verdict it is given and no pass of its
/// own; `None` reads one; another identity never consults it.
#[test]
fn the_token_reads_the_verdict_it_is_given() {
    let (_, valid, _) = bases().into_iter().find(|(l, ..)| l.starts_with("Y0")).unwrap();
    let refs: Vec<Value> = valid["numerical_quality"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect();
    assert_eq!(sc::numerical_use_standing_with_context(&valid, &refs, None), "numerically_eligible");
    let given = |source: &Value, verdict: Option<Result<(), String>>| {
        counted(|| sc::numerical_use_standing_given(source, &refs, None, verdict.as_ref()))
    };
    assert_eq!(given(&valid, Some(Err("SOURCE_PHYSICS_GIVEN".into()))), ("unsupported", 0));
    assert_eq!(given(&valid, Some(Ok(()))), ("numerically_eligible", 0));
    assert_eq!(given(&valid, None), ("numerically_eligible", 1));
    let mut refused = valid.clone();
    refused["contract_evidence"]["exact_cases"][0].as_object_mut().unwrap().remove("pressure_rhs_assembly");
    assert!(sc::validate_pressure_evidence(&refused).is_err());
    assert_eq!(given(&refused, Some(Ok(()))), ("numerically_eligible", 0));
    assert_eq!(given(&refused, None), ("unsupported", 1));
    let preview = fixture(include_str!("../../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json"));
    let expected = sc::numerical_use_standing_with_context(&preview, &refs, None);
    assert_eq!(given(&preview, Some(Err("SOURCE_PHYSICS_GIVEN".into()))), (expected, 0));
}

/// The other routes read as before: the reference's outcome, byte for byte, and
/// the same pressure-2 passes on both sides (preview physics with and without a
/// model and pressure-source-1 without its invocation: none; a retained successor
/// with its invocation: the passes its accepted reader runs on its projection).
#[test]
fn other_routes_read_as_before() {
    let model = fixture(include_str!("../../../../fixtures/product_preview/invented_preview_model.json"));
    let retained = fixture(include_str!("../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"));
    let reads: Vec<(&str, Value, Option<Value>, Option<Value>)> = vec![
        ("preview", fixture(include_str!("../../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json")), None, None),
        ("preview + model", fixture(include_str!("../../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json")), Some(model.clone()), None),
        ("preview u4b arc", fixture(include_str!("../../../../fixtures/results/preview_physics_u4b_arc_sparse.json")), None, None),
        ("preview rf skew + model", fixture(include_str!("../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_sparse_interactive.json")),
            Some(fixture(include_str!("../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json"))), None),
        ("pressure source", fixture(include_str!("../../../../fixtures/product_preview/pressure_source/load_reference_source/fields-sparse_interactive.raw.json")), None, None),
        ("retained successor", retained["source"].clone(), None, Some(retained["invocation"].clone())),
    ];
    let mut routes = Vec::new();
    for (what, source, model, invocation) in &reads {
        let (reference, before) = counted(|| reference_dispatch(source, model.as_ref(), invocation.as_ref()));
        let (head, after) = counted(|| dispatch(source, model.as_ref(), invocation.as_ref()));
        assert_eq!(bytes(&head), bytes(&reference), "{what}");
        assert_eq!(after, before, "{what}");
        routes.push((head["route"].as_str().unwrap().to_string(), after));
    }
    let routes: Vec<(&str, usize)> = routes.iter().map(|(r, n)| (r.as_str(), *n)).collect();
    assert_eq!(routes, [("preview_physics", 0), ("preview_physics", 0), ("preview_physics", 0), ("preview_physics", 0), ("pressure_source", 0), ("retained_physics", 2)]);
}
