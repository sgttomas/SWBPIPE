//! b7-plan R2: the base-route readers' verdicts for callers outside Rust (the
//! desktop's wasm engine, the headless runner's `read` command). Rust is the only
//! product reader; the desktop presents these verdicts and keeps only its capture
//! and session state (the registered invocation, G4).
//!
//! What this module adds to the readers it calls, by the R2 rulings (T3 under the
//! owner's decision, 2026-10-11):
//! - G1: the desktop's standing findings, their codes and order as the TypeScript
//!   standing produced them, beside `numerical_use_standing_with_context`'s token
//!   (`standing`).
//! - G2: the model-bound physics checks (`MODEL_CASE_COVERAGE`,
//!   `MODEL_MEMBER_COVERAGE`, `SUPPORT_BINDING`; T4-U8: `SHEAR_MODEL_CONTRACT`
//!   and `ARC_BEAM_THEORY` against the model's contract), run after the
//!   unchanged reader when a model is given (`physics_model_checks`).
//! - G5: one header dispatch, the superset of both languages' (`route`): the
//!   TypeScript header's receipt-shape, evidence-presence and namespace checks
//!   around `for_source_metadata`, whose retained transport reading stays.
//! - pressure-2 (DEL-04-07 `pressure-contract.md`, "Reading pressure-2"): its
//!   own route, `pressure`, read by `validate_pressure_evidence` (the only
//!   product reader of `3.0.0/exact_pressure_v3`), with G2's model checks and
//!   the ordinary standing, so the desktop, Python product paths and the
//!   VP-STATIC `exact_pressure_1` gate take one verdict and one standing.
//!
//! No existing reader, table or published byte changes. Every function here is
//! reachable only from these callers.
//!
//! Outcomes follow `retained_precision::outcome_json`: `{"ok":true,…}` or
//! `{"ok":false,"error":{"code","detail"}}`, `detail` the reader's whole text when
//! it carries more than the code, else null.
use crate::semantic_contract as sc;
use serde_json::{json, Value};

/// The desktop's route names (its `SourceContract`), one per readable identity.
pub const ROUTES: [&str; 5] = [
    "pressure_source",
    "preview_physics",
    "pressure",
    "retained_preview_physics",
    "retained_physics",
];
/// A source the header dispatch refuses reads as unsupported.
pub const UNSUPPORTED: &str = "unsupported";
pub const SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED: &str = "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED";
pub const SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED: &str =
    "SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED";
pub const LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE: &str =
    "LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1";
pub const RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE: &str =
    "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE";
/// pressure-2 has no transport (metadata-only) reader: a transported
/// pressure-2 statement is refused by this name, never admitted unchecked.
pub const PRESSURE_TRANSPORT_UNSUPPORTED: &str = "SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED";

/// Each route's pinned table bindings `{id, sha256}` (the table file's sha256;
/// `binding_hashes_are_the_table_files` checks them). A route's first binding is its
/// default; the preview-physics route admits two identities (pf-timoshenko.md §4), and a
/// source's binding is keyed by its `semantic_contract_id` (`binding_of`).
const BINDINGS: [(&str, &str, &str); 6] = [
    ("pressure_source", sc::PRESSURE_SOURCE_ID, sc::PRESSURE_SOURCE_TABLE_SHA256),
    ("preview_physics", sc::PREVIEW_PHYSICS_ID, sc::PREVIEW_PHYSICS_SHA256),
    ("preview_physics", sc::PREVIEW_PHYSICS_TIMOSHENKO_ID, sc::PREVIEW_PHYSICS_TIMOSHENKO_SHA256),
    ("pressure", sc::PRESSURE_ID, sc::PRESSURE_TABLE_SHA256),
    ("retained_preview_physics", sc::PREVIEW_PHYSICS_RETAINED_ID, sc::PREVIEW_PHYSICS_RETAINED_SHA256),
    ("retained_physics", sc::PHYSICS_RETAINED_ID, sc::PHYSICS_RETAINED_SHA256),
];

/// A reader's error text as `{code, detail}`: the code is the leading
/// upper-case token; `detail` is the whole text when it carries more.
pub fn error_json(text: &str) -> Value {
    let end = text
        .char_indices()
        .find(|(i, c)| !(c.is_ascii_uppercase() || (*i > 0 && (c.is_ascii_digit() || *c == '_'))))
        .map_or(text.len(), |(i, _)| i);
    let code = &text[..end];
    json!({"code": code, "detail": if code == text { Value::Null } else { Value::from(text) }})
}
fn verdict_refusal(text: &str) -> Value {
    json!({"ok": false, "error": error_json(text)})
}
fn present(value: Option<&Value>) -> Option<&Value> {
    value.filter(|v| !v.is_null())
}

/// G5: the header dispatch, the superset of both languages' header checks: the
/// route, and the table `for_source_metadata` selects. In order: the downgrade
/// guard (a receipt member or a W1 row token off the successor), a carrier
/// namespace, `for_source_metadata` (with its retained transport reading), then
/// the TypeScript header's checks: each source route's receipt shape, the evidence
/// object the load-reference and pressure routes require, and no identity
/// outside these routes.
pub fn route(source: &Value) -> Result<(&'static str, &'static Value), String> {
    sc::forbid_retained_member(source)?;
    sc::forbid_retained_rows(source)?;
    if source.get("carrier_evidence").is_some() {
        return Err(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED.into());
    }
    let (table, _) = sc::for_source_metadata(source)?;
    let evidence_object = source.get("contract_evidence").is_some_and(Value::is_object);
    let receipt = &source["source_block_recovery"];
    let route = match source["producer"]["semantic_contract_id"].as_str() {
        Some(sc::PRESSURE_SOURCE_ID) => {
            // As the retired load-reference-source-1 route did: the composite
            // needs its evidence object before its receipt is read.
            if !evidence_object {
                return Err(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED.into());
            }
            crate::physics_source::SourceContract::PressureV3.check_received(source)?;
            crate::physics_source::validate_receipt_shape_in(receipt, crate::physics_source::SourceContract::PressureV3)?;
            "pressure_source"
        }
        Some(sc::PREVIEW_PHYSICS_ID | sc::PREVIEW_PHYSICS_TIMOSHENKO_ID) => "preview_physics",
        Some(sc::PRESSURE_ID) => {
            if !evidence_object {
                return Err(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED.into());
            }
            "pressure"
        }
        Some(sc::PREVIEW_PHYSICS_RETAINED_ID) => "retained_preview_physics",
        Some(sc::PHYSICS_RETAINED_ID) => "retained_physics",
        _ => return Err(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED.into()),
    };
    Ok((route, table))
}
/// A route's default table binding `{id, sha256}`; null for unsupported.
pub fn binding(route: &str) -> Value {
    BINDINGS
        .iter()
        .find(|b| b.0 == route)
        .map_or(Value::Null, |b| json!({"id": b.1, "sha256": b.2}))
}
/// A routed source's table binding, keyed by its `semantic_contract_id` within its route's
/// admitted set (pf-timoshenko.md §4); null for unsupported or an identity outside the set.
pub fn binding_of(route: &str, source: &Value) -> Value {
    let id = source["producer"]["semantic_contract_id"].as_str().unwrap_or_default();
    BINDINGS
        .iter()
        .find(|b| b.0 == route && b.1 == id)
        .map_or(Value::Null, |b| json!({"id": b.1, "sha256": b.2}))
}

/// pf-timoshenko.md §4 (model-bound): a preview-physics identity binds the beam theory
/// the model selects. `model_settings.shear_model = timoshenko_annulus_energy_v1` selects
/// the Timoshenko identity; absent or `euler_bernoulli` selects preview-physics-2. A
/// crossed pair is refused by name. Any other model value is the producer's refusal,
/// never a result, so it binds neither identity.
pub const SHEAR_MODEL_IDENTITY: &str = "SOURCE_PREVIEW_PHYSICS_SHEAR_MODEL_IDENTITY";
pub fn bind_preview_shear_model(source: &Value, model: &Value) -> Result<(), String> {
    let selected = &model["model_settings"]["shear_model"];
    let expected = match selected.as_str() {
        _ if selected.is_null() => sc::PREVIEW_PHYSICS_ID,
        Some("euler_bernoulli") => sc::PREVIEW_PHYSICS_ID,
        Some("timoshenko_annulus_energy_v1") => sc::PREVIEW_PHYSICS_TIMOSHENKO_ID,
        _ => return Err(format!("{SHEAR_MODEL_IDENTITY}: model_settings.shear_model is not a selectable value")),
    };
    if source["producer"]["semantic_contract_id"] != expected {
        return Err(format!("{SHEAR_MODEL_IDENTITY}: the result's semantic contract is not the one the model's shear model selects ({expected})"));
    }
    Ok(())
}

/// G2: the checks against the current model, after the reader admitted the
/// evidence (TS `physicsResultEvidence.ts`: every solved exact case is a model
/// load case; each case's members are the model's pipes but those an objective
/// connector replaces; T4-U6, each case section's captured authored values are
/// the model's section of that pipe, [`model_section_bound`]; a support row
/// names a model support; T4-U8, each section entry states the beam theory the
/// model's `pressure_contract` selects for its member,
/// `section_basis::bind_shear_model`). Each runs only when the model carries
/// the member it reads.
pub fn physics_model_checks(source: &Value, model: &Value) -> Result<(), String> {
    fn ids<'a>(items: &'a Value, key: &str) -> Vec<&'a Value> {
        items.as_array().map(|a| a.iter().map(|x| &x[key]).collect()).unwrap_or_default()
    }
    fn same_set(a: &[&Value], b: &[&Value]) -> bool {
        a.len() == b.len() && a.iter().all(|x| b.contains(x))
    }
    let evidence = &source["contract_evidence"];
    let cases = evidence["exact_cases"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    if source["status"]["mechanics"] == "MECHANICS_SOLVED" {
        let case_ids: Vec<&Value> = cases.iter().map(|c| &c["load_case_id"]).collect();
        if !same_set(&case_ids, &ids(&model["load_cases"], "id")) {
            return Err("SOURCE_PHYSICS_MODEL_CASE_COVERAGE".into());
        }
    }
    if model.get("pipe_segments").is_some_and(|v| !v.is_null()) {
        let replaced = ids(&evidence["connector"], "replaced_pipe_id");
        let pipes: Vec<&Value> = ids(&model["pipe_segments"], "id")
            .into_iter()
            .filter(|id| !replaced.contains(id))
            .collect();
        for case in cases {
            if !same_set(&ids(&case["pipe_materials"], "pipe_id"), &pipes) {
                return Err("SOURCE_PHYSICS_MODEL_MEMBER_COVERAGE".into());
            }
        }
        // T4-U6 (§1, §5; FINAL-B): the evidence reader rederived each case
        // section's three geometries from its captured (D, t, c_m, c_a); those
        // are the model's, bit for bit.
        let segments = model["pipe_segments"].as_array().map(Vec::as_slice).unwrap_or(&[]);
        for case in cases {
            for section in case["pipe_sections"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
                if !model_section_bound(section, segments) {
                    return Err("SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY".into());
                }
            }
        }
    }
    // T4-U8 (§3.6, §5): every section entry states the beam theory the model's
    // contract selects for its member (a realized arc, Euler-Bernoulli).
    if model.get("pressure_contract").is_some_and(|v| !v.is_null()) {
        crate::section_basis::bind_shear_model(evidence, model)?;
    }
    if model.get("supports").is_some_and(|v| !v.is_null()) {
        let supports = ids(&model["supports"], "id");
        for row in source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            if matches!(
                row["kind"].as_str(),
                Some(
                    "support_reaction_component_v2"
                        | "support_reaction_force_magnitude_v2"
                        | "support_reaction_moment_magnitude_v2"
                )
            ) && !supports.contains(&&row["entity_ref"])
            {
                return Err("SOURCE_PHYSICS_SUPPORT_BINDING".into());
            }
        }
    }
    Ok(())
}

/// T4-U6 (D-5 §1, §2.5, §5): one `pipe_sections` entry's captured authored
/// values are the model's section of the same pipe, bit for bit, as
/// pressure-source-1 binds them against its invocation
/// (`physics_source::actual_materials`): `outside_diameter_m`,
/// `wall_thickness_m`, `mill_tolerance_m` (0.0 where the model's section has
/// none) and `corrosion_allowance_m`, each the model's quantity in metres by
/// the units authority the producer normalizes with. A signed zero is not its
/// other zero. A pipe the model lacks, a quantity it lacks (the allowance is
/// required) or one that does not normalize to a finite length is unbound.
fn model_section_bound(section: &Value, segments: &[Value]) -> bool {
    use open_pipe_stress_units::Dimension;
    let Some(pipe) = segments.iter().find(|p| p["id"] == section["pipe_id"]) else {
        return false;
    };
    let authored = &pipe["section"];
    let length = |quantity: &Value| crate::physics_source::normalized_quantity(quantity, Dimension::Length).ok();
    let mill_tolerance = if authored["mill_tolerance"].is_null() { Some(0.0) } else { length(&authored["mill_tolerance"]) };
    [
        ("outside_diameter_m", length(&authored["outside_diameter"])),
        ("wall_thickness_m", length(&authored["wall_thickness"])),
        ("mill_tolerance_m", mill_tolerance),
        ("corrosion_allowance_m", length(&authored["corrosion_allowance"])),
    ]
    .into_iter()
    .all(|(key, model)| {
        model.is_some_and(|m| section[key].as_f64().is_some_and(|e| e.is_finite() && e.to_bits() == m.to_bits()))
    })
}

/// The readers by name: `(reader, source, model, invocation)` to an outcome.
pub const READERS: [&str; 4] = [
    "preview-physics",
    "pressure-source",
    "source-recovery",
    "pressure",
];

/// One current reader's raw validation. `model` (G2) reaches the pressure
/// reader; `invocation` reaches the retained-source reader. A pass is
/// `{"ok":true}`, with `eligible` from the readers that grant eligibility and
/// `findings` (the source readers' standing findings, G1) beside it.
pub fn validate(reader: &str, source: &Value, model: Option<&Value>, invocation: Option<&Value>) -> Value {
    let (model, invocation) = (present(model), present(invocation));
    let checked = |check: Result<(), String>| match check {
        Ok(()) => json!({"ok": true}),
        Err(e) => verdict_refusal(&e),
    };
    match reader {
        "source-recovery" => checked(source_recovery(source)),
        // T4-U4b (§5.1): with the captured model, τ bit for bit (G2).
        // pf-timoshenko.md §4: with the model, a crossed identity is refused first.
        "preview-physics" => checked(sc::validate_preview_physics_evidence(source).and_then(|()| {
            model.map_or(Ok(()), |m| {
                bind_preview_shear_model(source, m).and_then(|()| crate::u4b_evidence::preview_model_checks(source, m))
            })
        })),
        // Current pressure-2 reader, then G2's model checks,
        // which read only the cases, members, connectors and support rows.
        "pressure" => checked(sc::validate_pressure_evidence(source).and_then(|()| {
            model.map_or(Ok(()), |m| physics_model_checks(source, m).and_then(|()| crate::u4b_evidence::exact_model_checks(source, m)))
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
/// The current source reader's validation. With an invocation it also takes
/// G2's model checks against the invocation's model, as the TypeScript validation
/// ran its physics checks with that model.
fn source_validate_in(source: &Value, invocation: Option<&Value>, contract: crate::physics_source::SourceContract) -> Result<bool, String> {
    let eligible = crate::physics_source::validate_in(source, invocation, contract)?;
    if let Some(i) = invocation {
        physics_model_checks(source, &i["request"]["model"])?;
    }
    Ok(eligible)
}
/// Portable receipt/publication consistency for the analysis-run and derivative
/// builders. This deliberately reads only the checks they previously ran: the
/// header, the source route's transport metadata, and the publication commitment.
/// It does not validate raw rows, an invocation, or numerical eligibility.
pub fn source_recovery(source: &Value) -> Result<(), String> {
    match route(source).map(|r| r.0) {
        Ok("pressure_source") => crate::physics_source::validate_transport_metadata_in(source, crate::physics_source::SourceContract::PressureV3)?,
        _ => return Err("SOURCE_RECOVERY_METADATA_CONTRADICTION".into()),
    }
    crate::source_blocks::validate_publication_hash(source)
}

/// One reader's transport (metadata-only) checks; never eligible.
pub fn transport(reader: &str, source: &Value) -> Value {
    let result = match reader {
        "preview-physics" => {
            crate::retained_precision::validate_preview_physics_transport_metadata(source)
        }
        "pressure-source" => crate::physics_source::validate_transport_metadata_in(source, crate::physics_source::SourceContract::PressureV3),
        "pressure" => Err(PRESSURE_TRANSPORT_UNSUPPORTED.into()),
        other => Err(format!("READER_UNKNOWN: {other}")),
    };
    match result {
        Ok(()) => json!({"ok": true}),
        Err(e) => verdict_refusal(&e),
    }
}

/// G1: the findings of a pressure-source-1 validation with
/// its invocation, as the TypeScript validation produced them: per case in
/// order, an unqualified case and each inspection-only row of a physical (or
/// unknown) kind; then an envelope that is not qualified or whose model has
/// combinations; then an unsolved envelope. Each code once, first place kept.
pub fn source_findings(source: &Value, invocation: &Value) -> Vec<&'static str> {
    let table = sc::pressure_source_contract();
    let rows: std::collections::HashMap<&str, &Value> = source["results"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[])
        .iter()
        .filter_map(|r| r["id"].as_str().map(|id| (id, r)))
        .collect();
    // TS `semantic(row)`: the first table row of this kind and unit whose
    // component is null or the row's, and whose source_basis (if any) is the row's.
    let physical = |row: &Value| {
        table["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| {
                s["kind"] == row["kind"]
                    && s["unit"] == row["unit"]
                    && (s["component"].is_null() || s["component"] == row["metadata"]["component"])
                    && (s.get("source_basis").is_none() || s["source_basis"] == row["metadata"]["basis"])
            })
            .is_none_or(|s| s["category"] == "physical_quantity")
    };
    let mut findings = Vec::new();
    let body = &source["source_block_recovery"]["body"];
    for case in body["cases"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        if case["outcome"] != "qualified" {
            findings.push("SOURCE_BLOCKS_CASE_UNQUALIFIED");
        }
        for treatment in case["rows"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            // T4-U4b §4.3: `derived_not_qualified` is neutral for the two
            // derived kinds only: no finding, and no eligibility of its own.
            let neutral = treatment["treatment"] == crate::source_blocks::NOT_QUALIFIED
                && treatment["result_id"].as_str().and_then(|id| rows.get(id)).is_some_and(|row| {
                    row["kind"].as_str().is_some_and(|k| crate::source_blocks::NOT_QUALIFIED_KINDS.contains(&k))
                });
            let known = neutral
                || matches!(
                    treatment["treatment"].as_str(),
                    Some("qualified_projection" | "checked_derived" | "ordinary_physics_checked")
                );
            if !known {
                if let Some(row) = treatment["result_id"].as_str().and_then(|id| rows.get(id)) {
                    if physical(row) {
                        findings.push("SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY");
                    }
                }
            }
        }
    }
    let combinations = invocation["request"]["model"]["combinations"]
        .as_array()
        .is_some_and(|a| !a.is_empty());
    if body["status"] != "qualified" || combinations {
        findings.push("SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED");
    }
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
        findings.push("MECHANICS_NOT_SOLVED");
    }
    let mut seen = std::collections::HashSet::new();
    findings.retain(|f| seen.insert(*f));
    findings
}

/// G1: the desktop's numerical standing, `{route, token, status, eligible,
/// findings}`. `token` is `numerical_use_standing_with_context` with the model's
/// load cases as the requested refs and the caller's invocation, unchanged.
/// `findings` are the TypeScript standing's codes in its order: a refusal's code;
/// a source route's validation findings (or
/// `SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED` without an invocation, G4); the
/// joined route's validation code or its T1 reason; a successor's reader code or
/// its not-eligible reason; otherwise the evidence reader's code and the
/// ordinary checks of `numerical_quality` against the model;
/// then, but for the joined and successor routes, an unsolved envelope.
/// `eligible` is no findings, and `status` is `integrity_checked` when eligible.
pub fn standing(source: &Value, model: Option<&Value>, invocation: Option<&Value>) -> Value {
    let (model, invocation) = (present(model), present(invocation));
    let requested_ids: Vec<&Value> = model
        .and_then(|m| m["load_cases"].as_array())
        .map(|cases| cases.iter().map(|c| &c["id"]).collect())
        .unwrap_or_default();
    let requested_refs: Vec<Value> = requested_ids
        .iter()
        .map(|id| json!({"ref_type": "load_case", "ref_id": id}))
        .collect();
    let token = sc::numerical_use_standing_with_context(source, &requested_refs, invocation);
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
                "pressure" => Some(validate("pressure", source, model, None)),
                "preview_physics" => Some(validate("preview-physics", source, model, None)),
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
/// The ordinary checks of `numerical_quality` (TS `numericalResultStanding`'s
/// generic branch), in its order.
fn ordinary_findings(source: &Value, requested: &[&Value]) -> Vec<String> {
    const ORDER: [&str; 5] = ["checks_passed", "sensitive", "not_assessed", "unresolved", "failed"];
    let q = &source["numerical_quality"];
    let cases = q["cases"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let mut findings = Vec::new();
    if q["status"] != "checks_passed" {
        findings.push("NUMERICAL_INTEGRITY_NOT_QUALIFIED".to_string());
    }
    let nonempty = |v: &Value| v.as_str().is_some_and(|s| !s.is_empty());
    let results = source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let diagnostics = source["diagnostics"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    // TS: a diagnostic contributes its id only when it has a truthy one.
    let truthy = |v: &Value| !(v.is_null() || v == &json!(false) || v == &json!(0) || v == &json!(""))
        && !v.as_f64().is_some_and(|n| n == 0.0);
    let emitted_ids: Vec<&Value> = results
        .iter()
        .map(|r| &r["id"])
        .chain(diagnostics.iter().map(|d| &d["id"]).filter(|id| truthy(id)))
        .collect();
    let mut emitted = Vec::new();
    for id in &emitted_ids {
        if !emitted.contains(id) {
            emitted.push(*id);
        }
    }
    if !results.iter().all(|r| nonempty(&r["id"]))
        || !diagnostics.iter().all(|d| nonempty(&d["id"]))
        || emitted.len() != emitted_ids.len()
    {
        findings.push("NUMERICAL_EVIDENCE_ID_AMBIGUOUS".into());
    }
    let mut distinct = Vec::new();
    for id in requested {
        if !distinct.contains(id) {
            distinct.push(*id);
        }
    }
    if requested.is_empty() || distinct.len() != requested.len() {
        findings.push("REQUESTED_NUMERICAL_BASIS_UNAVAILABLE".into());
    }
    if cases.len() != requested.len()
        || requested.iter().any(|id| {
            cases
                .iter()
                .filter(|c| c["basis_ref"]["ref_type"] == "load_case" && c["basis_ref"]["ref_id"] == **id)
                .count()
                != 1
        })
    {
        findings.push("NUMERICAL_CASE_COVERAGE_INCOMPLETE".into());
    }
    if cases.iter().any(|c| !ordinary_case_eligible(c, &emitted)) {
        findings.push("NUMERICAL_CASE_EVIDENCE_INCOMPLETE".into());
    }
    // TS: the worst case's position in ORDER (an unknown status counts as none),
    // or not_assessed with no cases.
    let aggregate = if cases.is_empty() {
        "not_assessed"
    } else {
        ORDER[cases
            .iter()
            .filter_map(|c| ORDER.iter().position(|s| c["solve_quality"] == *s))
            .max()
            .unwrap_or(0)]
    };
    if q["status"] != aggregate {
        findings.push("NUMERICAL_AGGREGATE_CONTRADICTION".into());
    }
    findings
}
/// One `numerical_quality` case's ordinary eligibility (TS `ordinaryCaseEligible`).
fn ordinary_case_eligible(case: &Value, emitted: &[&Value]) -> bool {
    case.is_object()
        && case["structural_status"] == "passive_model_basis"
        && case["solve_quality"] == "checks_passed"
        && case["model_matrix_fidelity"] == "represented_equations_retained"
        && matches!(case["accuracy_evidence"].as_str(), Some("not_claimed" | "reference_verified"))
        && case["evidence_refs"]
            .as_array()
            .is_some_and(|refs| !refs.is_empty() && refs.iter().all(|r| emitted.contains(&r)))
}

/// The dispatch entry: the route, its table binding, the header refusal (or
/// null), and the standing (`standing`).
pub fn dispatch(source: &Value, model: Option<&Value>, invocation: Option<&Value>) -> Value {
    let (route_name, refusal) = match route(source) {
        Ok((route, _)) => (route, Value::Null),
        Err(code) => (UNSUPPORTED, error_json(&code)),
    };
    json!({"ok": refusal.is_null(), "route": route_name, "binding": binding_of(route_name, source),
        "error": refusal, "standing": standing(source, model, invocation)})
}

/// One result row's semantics under its source's table: its signature or null,
/// its complete-metadata flag and canonical metadata. No source, a source the
/// dispatch refuses, or a contradictory row, is refused.
pub fn row_semantics(row: &Value, source: Option<&Value>) -> Value {
    let table = match present(source).map(route) {
        Some(Ok((_, table))) => table,
        _ => return verdict_refusal("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED"),
    };
    match sc::signature_in(table, row) {
        Ok(signature) => json!({"ok": true, "signature": signature,
            "complete_metadata": sc::complete_metadata(row),
            "canonical_metadata": sc::canonical_metadata_in(table, row)}),
        Err(e) => verdict_refusal(&e),
    }
}
