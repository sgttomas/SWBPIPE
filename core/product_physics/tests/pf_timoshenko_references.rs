//! PF Timoshenko (DEL-05-03 Design `pf-timoshenko.md` §5): the product against
//! the frozen, refuted independent references in
//! `validation/references/pf_timoshenko/product_cases.json`, through the public
//! value entry in both solver modes, under the package's own criterion: relative
//! 1e-9 on nonzero rows, the stated dimensional zero scales on zero rows, and
//! `cancellation_zero_scale` on the six PHI2 θ_B rows. Every expected and wrong
//! value is read from the reference files; none is computed through the
//! product. A disagreement goes back to the reference owner and its refuter,
//! never to an edited expectation.
//!
//! Beside the rows: each record's expected diagnostics (the authored-ν warning
//! and the named arc disclosure, and nothing new on an Euler–Bernoulli twin);
//! the result identity (the Timoshenko identity on every Timoshenko document,
//! preview-physics-2 on every Euler–Bernoulli one, absent or explicit); the
//! listed mutants; the Euler–Bernoulli twins as the breach control on every row
//! marked `discriminating`; and the realized-arc controls, which the product must
//! not match on every row marked `arc_timoshenko_discriminating`.
//!
//! **Two request-authoring defects, returned to the reference owner.** Sixteen
//! frozen requests are refused by the product for reasons unrelated to beam
//! theory, both twins alike: SS-MID and the four PHI2 families author a `guide`
//! restraining RX (`SUPPORT_INPUT_INVALID`, "Rx is not valid for Guide"), and the
//! three NL families omit each nonlinear support's explicit `initial_state`
//! (`NONLINEAR_SUPPORT_INITIAL_STATE_MISSING`). Each such frozen request is solved
//! and must be refused by exactly that code; its copy is then re-authored in
//! memory ([`reauthor_defects`]: the same restraint set in the `anchor` family,
//! and the solver's initial seed, `inactive` or `sticking`, which the converged
//! state the rows record does not depend on) and judged like every other request.
//! No expected value is touched. Once the owner re-authors the requests (their
//! hashes move, their rows do not), the refusal assertion fails and the list goes.
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[path = "common/current_model.rs"]
mod current_model;
#[path = "common/frozen_rows.rs"]
mod frozen_rows;
use frozen_rows::{discriminated, number, passed, sha256_hex, solve, MODES};

const CASES: &str = include_str!("../../../validation/references/pf_timoshenko/product_cases.json");
// Literal include paths, which the hosted CI selector (tools/ci/numerical_ci.py)
// resolves; each request is still bound by the SHA-256 its record names.
const REQUESTS: [(&str, &str); 48] = [
    ("requests/PFT-ARC-ANCH-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-ARC-ANCH-EULER-BERNOULLI.request.json")),
    ("requests/PFT-ARC-ANCH-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-ARC-ANCH-TIMOSHENKO.request.json")),
    ("requests/PFT-ARC-TIP-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-ARC-TIP-EULER-BERNOULLI.request.json")),
    ("requests/PFT-ARC-TIP-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-ARC-TIP-TIMOSHENKO.request.json")),
    ("requests/PFT-AUTHNU-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-AUTHNU-EULER-BERNOULLI.request.json")),
    ("requests/PFT-AUTHNU-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-AUTHNU-TIMOSHENKO.request.json")),
    ("requests/PFT-IMPOSED-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-IMPOSED-EULER-BERNOULLI.request.json")),
    ("requests/PFT-IMPOSED-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-IMPOSED-TIMOSHENKO.request.json")),
    ("requests/PFT-NL-FRICTION-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-NL-FRICTION-EULER-BERNOULLI.request.json")),
    ("requests/PFT-NL-FRICTION-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-NL-FRICTION-TIMOSHENKO.request.json")),
    ("requests/PFT-NL-GAP-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-NL-GAP-EULER-BERNOULLI.request.json")),
    ("requests/PFT-NL-GAP-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-NL-GAP-TIMOSHENKO.request.json")),
    ("requests/PFT-NL-ONEWAY-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-NL-ONEWAY-EULER-BERNOULLI.request.json")),
    ("requests/PFT-NL-ONEWAY-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-NL-ONEWAY-TIMOSHENKO.request.json")),
    ("requests/PFT-PHI2-EXACT-AUXETIC-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-EXACT-AUXETIC-EULER-BERNOULLI.request.json")),
    ("requests/PFT-PHI2-EXACT-AUXETIC-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-EXACT-AUXETIC-TIMOSHENKO.request.json")),
    ("requests/PFT-PHI2-MINUS2ULP-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-MINUS2ULP-EULER-BERNOULLI.request.json")),
    ("requests/PFT-PHI2-MINUS2ULP-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-MINUS2ULP-TIMOSHENKO.request.json")),
    ("requests/PFT-PHI2-NEAREST-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-NEAREST-EULER-BERNOULLI.request.json")),
    ("requests/PFT-PHI2-NEAREST-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-NEAREST-TIMOSHENKO.request.json")),
    ("requests/PFT-PHI2-PLUS2ULP-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-PLUS2ULP-EULER-BERNOULLI.request.json")),
    ("requests/PFT-PHI2-PLUS2ULP-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-PHI2-PLUS2ULP-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L0P25-CM0-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L0P25-CM0-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L0P25-CM0-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L0P25-CM0-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L1-CM0-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L1-CM0-EXPLICIT-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-EXPLICIT-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L1-CM2MM-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM2MM-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L1-CM2MM-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM2MM-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L10-CM0-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L10-CM0-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L10-CM0-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L10-CM0-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L10-CM2MM-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L10-CM2MM-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L10-CM2MM-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L10-CM2MM-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L2-CM0-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L2-CM0-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L2-CM0-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L2-CM0-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L2-CM2MM-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L2-CM2MM-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L2-CM2MM-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L2-CM2MM-TIMOSHENKO.request.json")),
    ("requests/PFT-SB-L50-CM0-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L50-CM0-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SB-L50-CM0-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SB-L50-CM0-TIMOSHENKO.request.json")),
    ("requests/PFT-SKEW-SPRING-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SKEW-SPRING-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SKEW-SPRING-EXPLICIT-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SKEW-SPRING-EXPLICIT-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SKEW-SPRING-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SKEW-SPRING-TIMOSHENKO.request.json")),
    ("requests/PFT-SS-MID-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SS-MID-EULER-BERNOULLI.request.json")),
    ("requests/PFT-SS-MID-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-SS-MID-TIMOSHENKO.request.json")),
    ("requests/PFT-W15-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-W15-EULER-BERNOULLI.request.json")),
    ("requests/PFT-W15-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-W15-TIMOSHENKO.request.json")),
    ("requests/PFT-WEIGHT-EULER-BERNOULLI.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-WEIGHT-EULER-BERNOULLI.request.json")),
    ("requests/PFT-WEIGHT-TIMOSHENKO.request.json", include_str!("../../../validation/references/pf_timoshenko/requests/PFT-WEIGHT-TIMOSHENKO.request.json")),
];

const TIMOSHENKO_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
const TIMOSHENKO_PROFILE: &str = "product_preview_mechanics_timoshenko_v1";
const TIMOSHENKO_LIMITATION_0: &str = "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_v1) and Poisson's ratio E/(2G) - 1, and realized arcs are Euler-Bernoulli; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness.";
const EULER_BERNOULLI_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const EULER_BERNOULLI_PROFILE: &str = "product_preview_mechanics_v1";
const EULER_BERNOULLI_LIMITATION_0: &str = "Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.";
const AUXETIC: &str = "PFT-PHI2-EXACT-AUXETIC-TIMOSHENKO";
/// The frozen requests the product refuses for an authoring defect, with the refusal code.
const REQUEST_DEFECTS: [(&str, &str); 16] = [
    ("PFT-SS-MID-TIMOSHENKO", GUIDE_RX),
    ("PFT-SS-MID-EULER-BERNOULLI", GUIDE_RX),
    ("PFT-PHI2-NEAREST-TIMOSHENKO", GUIDE_RX),
    ("PFT-PHI2-NEAREST-EULER-BERNOULLI", GUIDE_RX),
    ("PFT-PHI2-PLUS2ULP-TIMOSHENKO", GUIDE_RX),
    ("PFT-PHI2-PLUS2ULP-EULER-BERNOULLI", GUIDE_RX),
    ("PFT-PHI2-MINUS2ULP-TIMOSHENKO", GUIDE_RX),
    ("PFT-PHI2-MINUS2ULP-EULER-BERNOULLI", GUIDE_RX),
    ("PFT-PHI2-EXACT-AUXETIC-TIMOSHENKO", GUIDE_RX),
    ("PFT-PHI2-EXACT-AUXETIC-EULER-BERNOULLI", GUIDE_RX),
    ("PFT-NL-ONEWAY-TIMOSHENKO", NO_INITIAL_STATE),
    ("PFT-NL-ONEWAY-EULER-BERNOULLI", NO_INITIAL_STATE),
    ("PFT-NL-GAP-TIMOSHENKO", NO_INITIAL_STATE),
    ("PFT-NL-GAP-EULER-BERNOULLI", NO_INITIAL_STATE),
    ("PFT-NL-FRICTION-TIMOSHENKO", NO_INITIAL_STATE),
    ("PFT-NL-FRICTION-EULER-BERNOULLI", NO_INITIAL_STATE),
];
const GUIDE_RX: &str = "SUPPORT_INPUT_INVALID";
const NO_INITIAL_STATE: &str = "NONLINEAR_SUPPORT_INITIAL_STATE_MISSING";

/// The in-memory repair of the two defects (see the module comment), or `None` when the
/// request has neither: a `guide` restraining a rotation becomes an `anchor` with the same
/// restraints, and a nonlinear support without `initial_state` is seeded `inactive`
/// (one-way, gap) or `sticking` (friction).
fn reauthor_defects(request: &Value) -> Option<Value> {
    let mut copy = request.clone();
    let mut changed = false;
    for support in copy["model"]["supports"].as_array_mut().unwrap() {
        let rotational = support["restraints"].as_array().unwrap().iter().any(|dof| dof.as_str().unwrap().starts_with('R'));
        if support["family"] == json!("guide") && rotational {
            support["family"] = json!("anchor");
            changed = true;
        }
        if let Some(nonlinear) = support.get_mut("nonlinear").and_then(Value::as_object_mut) {
            if !nonlinear.contains_key("initial_state") {
                let seed = if nonlinear["behavior"] == json!("friction") { "sticking" } else { "inactive" };
                nonlinear.insert("initial_state".into(), json!(seed));
                changed = true;
            }
        }
    }
    changed.then_some(copy)
}

/// A frozen row's identity: (kind, entity_ref, component, location, basis ref_id).
type Key = (String, String, String, String, String);

fn key(row: &Value, basis: &str) -> Key {
    let text = |name: &str| row[name].as_str().unwrap_or_else(|| panic!("{name} of {row}")).to_owned();
    (text("kind"), text("entity_ref"), text("component"), text("location"), basis.to_owned())
}

/// The rows a record carries: its own, or those of the record it names in `same_rows_as`.
fn rows<'a>(by_id: &BTreeMap<&str, &'a Value>, case: &'a Value) -> BTreeMap<Key, &'a Value> {
    let source = case["same_rows_as"].as_str().map_or(case, |id| {
        let target = by_id[id];
        assert!(target.get("same_rows_as").is_none(), "{id}: one alias hop");
        target
    });
    let mut rows = BTreeMap::new();
    for lc in source["load_cases"].as_array().unwrap() {
        let basis = lc["id"].as_str().unwrap();
        for row in lc["rows"].as_array().unwrap() {
            assert_eq!(row["basis_ref"]["ref_id"], json!(basis));
            assert!(rows.insert(key(row, basis), row).is_none(), "{basis}: a unique row key");
        }
    }
    rows
}

/// The frozen criterion's tolerance in the row unit.
fn tolerance(row: &Value) -> f64 {
    let criterion = &row["criterion"];
    match criterion["kind"].as_str().unwrap() {
        "relative" => number(&criterion["relative_tolerance"]) * number(&row["expected"]).abs(),
        "zero_scale" | "cancellation_zero_scale" => number(&criterion["absolute_tolerance"]),
        other => panic!("criterion {other}"),
    }
}

/// The published rows with a frozen key (PF publishes each row once).
fn published<'a>(results: &'a [Value], key: &Key) -> Vec<&'a Value> {
    results
        .iter()
        .filter(|r| {
            r["kind"] == json!(key.0)
                && r["entity_ref"] == json!(key.1)
                && r["metadata"]["component"] == json!(key.2)
                && r["metadata"]["location"] == json!(key.3)
                && r["basis_ref"]["ref_id"] == json!(key.4)
        })
        .collect()
}

fn the_value(results: &[Value], key: &Key) -> Result<f64, String> {
    match published(results, key).as_slice() {
        [one] => Ok(number(&one["value"])),
        other => Err(format!("{} published rows", other.len())),
    }
}

fn label(key: &Key) -> String {
    format!("{} {} {} {} [{}]", key.0, key.1, key.2, key.3, key.4)
}

fn request(case: &Value) -> Value {
    let binding = &case["request"];
    let path = binding["path"].as_str().unwrap();
    let text = REQUESTS.iter().find(|(p, _)| *p == path).unwrap_or_else(|| panic!("{path} is not included")).1;
    assert_eq!(sha256_hex(text.as_bytes()), binding["sha256"].as_str().unwrap(), "{path}: the request the reference binds");
    let request: Value = serde_json::from_str(text).unwrap();
    let model = &request["model"];
    assert!(model.get("pressure_contract").is_none(), "{path}: a pressure-free document");
    let authored = model.pointer("/model_settings/shear_model");
    match case["shear_model"].as_str().unwrap() {
        "timoshenko_annulus_energy_v1" => assert_eq!(authored, Some(&json!("timoshenko_annulus_energy_v1")), "{path}"),
        "euler_bernoulli" => assert_eq!(authored, Some(&json!("euler_bernoulli")), "{path}"),
        "absent (Euler-Bernoulli)" => assert_eq!(authored, None, "{path}: the absent member"),
        other => panic!("{path}: shear_model {other}"),
    }
    request
}

fn codes(envelope: &Value, severity: Option<&str>) -> Vec<String> {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| severity.is_none_or(|s| d["severity"] == json!(s)))
        .map(|d| d["code"].as_str().unwrap().to_owned())
        .collect()
}

#[derive(Default, Debug)]
struct Tally {
    documents: usize,
    solves: usize,
    rows: usize,
    cancellation_rows: usize,
    discriminator_rows: usize,
    twin_breaches: usize,
    arc_control_breaches: usize,
    passed: usize,
    refused_frozen: usize,
}

/// Every request through the value entry in both modes, judged against its record.
#[test]
fn pf_timoshenko_references_agree_in_both_solver_modes() {
    let all: Value = serde_json::from_str(CASES).unwrap();
    let cases = all["cases"].as_array().unwrap();
    let by_id: BTreeMap<&str, &Value> = cases.iter().map(|c| (c["id"].as_str().unwrap(), c)).collect();
    let mut tally = Tally::default();
    let mut failures = Vec::new();
    let mut auxetic = Vec::new();
    for case in cases {
        let id = case["id"].as_str().unwrap();
        if case["request"].is_null() {
            assert_eq!(case["must_not_match"], json!(true), "{id}: only the arc controls have no document");
            continue;
        }
        tally.documents += 1;
        let frozen_request = request(case);
        let defect = REQUEST_DEFECTS.iter().find(|(defective, _)| *defective == id).map(|(_, code)| *code);
        let repaired = reauthor_defects(&frozen_request);
        assert_eq!(defect.is_some(), repaired.is_some(), "{id}: the defect list names exactly the requests the repair changes");
        let request = repaired.unwrap_or_else(|| frozen_request.clone());
        let timoshenko = case["shear_model"] == json!("timoshenko_annulus_energy_v1");
        let frozen = rows(&by_id, case);
        let twin = case["twin"].as_str().map(|twin| rows(&by_id, by_id[twin]));
        let control = by_id
            .values()
            .find(|c| c["must_not_match"] == json!(true) && c["twin"] == json!(id))
            .map(|control| rows(&by_id, control));
        for mode in MODES {
            let context = format!("{id} {mode:?}");
            if let Some(code) = defect {
                let refused = solve(&frozen_request, mode);
                assert!(refused["status"]["mechanics"] != json!("MECHANICS_SOLVED") && codes(&refused, Some("blocking")).contains(&code.to_owned()),
                    "{context}: the frozen request is refused by {code}; once re-authored, remove it from REQUEST_DEFECTS: {}", refused["diagnostics"]);
                let identity = if timoshenko { TIMOSHENKO_ID } else { EULER_BERNOULLI_ID };
                assert_eq!(refused["producer"]["semantic_contract_id"], json!(identity), "{context}: a blocked envelope's identity");
                tally.refused_frozen += 1;
            }
            let envelope = solve(&request, mode);
            tally.solves += 1;
            // The result identity, on solved and blocked envelopes alike.
            let (identity, profile, limitation) = if timoshenko {
                (TIMOSHENKO_ID, TIMOSHENKO_PROFILE, TIMOSHENKO_LIMITATION_0)
            } else {
                (EULER_BERNOULLI_ID, EULER_BERNOULLI_PROFILE, EULER_BERNOULLI_LIMITATION_0)
            };
            if envelope["producer"]["semantic_contract_id"] != json!(identity)
                || envelope["formulation_basis"]["profile_id"] != json!(profile)
                || envelope["formulation_basis"]["limitations"][0] != json!(limitation)
            {
                failures.push(format!("{context}: identity {} {} {}", envelope["producer"]["semantic_contract_id"],
                    envelope["formulation_basis"]["profile_id"], envelope["formulation_basis"]["limitations"][0]));
            }
            if id == AUXETIC && envelope["status"]["mechanics"] != json!("MECHANICS_SOLVED") {
                // The README leaves admission to the product's factor-range check.
                let by_range = codes(&envelope, Some("blocking")).contains(&"PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE".to_owned());
                if !by_range {
                    failures.push(format!("{context}: refused, but not by the factor range: {}", envelope["diagnostics"]));
                }
                auxetic.push(format!("{mode:?}: refused{}", if by_range { " by factor range" } else { " otherwise" }));
                continue;
            }
            if envelope["status"]["mechanics"] != json!("MECHANICS_SOLVED") {
                failures.push(format!("{context}: not solved: {}", envelope["diagnostics"]));
                continue;
            }
            if id == AUXETIC {
                auxetic.push(format!("{mode:?}: admitted and solved"));
            }
            tally.passed += usize::from(passed(&envelope));
            let results = envelope["results"].as_array().unwrap();
            // Every frozen row at its own criterion.
            for (key, row) in &frozen {
                let expected = number(&row["expected"]);
                tally.rows += 1;
                tally.cancellation_rows += usize::from(row["criterion"]["kind"] == json!("cancellation_zero_scale"));
                match the_value(results, key) {
                    Ok(observed) if (observed - expected).abs() <= tolerance(row) => {}
                    Ok(observed) => failures.push(format!("{context} {}: published {observed:e}, frozen {expected:e} (tolerance {:e})", label(key), tolerance(row))),
                    Err(error) => failures.push(format!("{context} {}: {error}", label(key))),
                }
            }
            // Each listed mutant's rows are not at the wrong value.
            for lc in case["load_cases"].as_array().into_iter().flatten() {
                let basis = lc["id"].as_str().unwrap();
                for mutant in lc["discriminators"].as_array().into_iter().flatten() {
                    for listed in mutant["rows"].as_array().unwrap() {
                        let key = key(listed, basis);
                        let absolute = frozen[&key]["criterion"].get("absolute_tolerance").map_or(0.0, number);
                        let wrong = number(&listed["wrong"]);
                        match the_value(results, &key) {
                            Ok(observed) if discriminated(observed, wrong, absolute) => tally.discriminator_rows += 1,
                            other => failures.push(format!("{context} {} {}: published {other:?} is the wrong value {wrong:e}", mutant["mutant"], label(&key))),
                        }
                    }
                }
            }
            // The twin control: every discriminating row breaches the twin's expectation.
            if let Some(twin) = &twin {
                for (key, row) in frozen.iter().filter(|(_, row)| row["discriminating"] == json!(true)) {
                    let Some(twin_row) = twin.get(key) else {
                        failures.push(format!("{context} {}: no twin row", label(key)));
                        continue;
                    };
                    let wrong = number(&twin_row["expected"]);
                    let absolute = row["criterion"].get("absolute_tolerance").map_or(0.0, number);
                    match the_value(results, key) {
                        Ok(observed) if discriminated(observed, wrong, absolute) => tally.twin_breaches += 1,
                        other => failures.push(format!("{context} {}: published {other:?} matches the twin's {wrong:e}", label(key))),
                    }
                }
            }
            // The realized-arc control: the arc stays Euler-Bernoulli, so the product must not
            // match the arc-Timoshenko solve on any row marked to discriminate it.
            if let Some(control) = &control {
                for (key, row) in frozen.iter().filter(|(_, row)| row["arc_timoshenko_discriminating"] == json!(true)) {
                    let Some(control_row) = control.get(key) else {
                        failures.push(format!("{context} {}: no control row", label(key)));
                        continue;
                    };
                    let wrong = number(&control_row["expected"]);
                    let absolute = row["criterion"].get("absolute_tolerance").map_or(0.0, number);
                    match the_value(results, key) {
                        Ok(observed) if discriminated(observed, wrong, absolute) => tally.arc_control_breaches += 1,
                        other => failures.push(format!("{context} {}: published {other:?} matches the arc-Timoshenko control's {wrong:e}", label(key))),
                    }
                }
            }
            // Expected diagnostics: exactly the listed PF Timoshenko codes, nothing new on a twin.
            let diagnostics = envelope["diagnostics"].as_array().unwrap();
            let pf: Vec<&Value> = diagnostics.iter().filter(|d| d["code"].as_str().unwrap().starts_with("PF_TIMOSHENKO_")).collect();
            let expected: Vec<&Value> = case["expected_diagnostics"].as_array().into_iter().flatten().filter(|d| d["code"] != json!("none new")).collect();
            if pf.len() != expected.len() {
                failures.push(format!("{context}: PF Timoshenko diagnostics {pf:?}, expected {expected:?}"));
                continue;
            }
            for want in expected {
                let Some(found) = pf.iter().find(|d| d["code"] == want["code"]) else {
                    failures.push(format!("{context}: {} missing", want["code"]));
                    continue;
                };
                let message = found["message"].as_str().unwrap();
                match want["code"].as_str().unwrap() {
                    "PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED" => {
                        let authored = format!("{:?}", number(&want["authored_poisson_ratio"]["value"]));
                        let derived = format!("{:?}", number(&want["derived_poisson_ratio"]["value"]));
                        let material = want["material"].as_str().unwrap();
                        if found["severity"] != json!("warning") || !message.contains(material) || !message.contains(&format!("poisson_ratio {authored}"))
                            || !message.contains(&format!("= {derived}")) || !message.contains("the base material basis")
                        {
                            failures.push(format!("{context}: {found}"));
                        }
                    }
                    "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI" => {
                        let counts = &want["message_counts"];
                        let straight = counts["straight_members_with_shear_deformation"].as_u64().unwrap();
                        let arcs = counts["realized_arcs_without_shear_deformation"].as_u64().unwrap();
                        let arc_word = if arcs == 1 { "realized arc" } else { "realized arcs" };
                        if found["severity"] != json!("info") || found["affected_refs"] != want["entity_refs"]
                            || !message.contains(&format!("{straight} straight members with shear deformation"))
                            || !message.contains(&format!("{arcs} {arc_word} without shear deformation"))
                        {
                            failures.push(format!("{context}: {found}"));
                        }
                    }
                    other => failures.push(format!("{context}: unexpected expected code {other}")),
                }
            }
        }
    }
    eprintln!("PF Timoshenko references: {tally:?}; auxetic exact phi = 2 control: {auxetic:?}");
    assert!(failures.is_empty(), "{} disagreements:\n{}", failures.len(), failures.join("\n"));
    assert_eq!(tally.documents, 48);
    assert_eq!(tally.solves, 96);
    assert_eq!(tally.cancellation_rows, 12, "the six theta_B rows in both modes");
    assert_eq!(tally.refused_frozen, 32, "the sixteen defective frozen requests in both modes");
    assert_eq!(auxetic, ["SparseInteractive: admitted and solved", "DenseScrutiny: admitted and solved"],
        "nu = -5/9 lies in the factor's range (-1, 1/2), so the product admits the exact phi = 2 control");
}
