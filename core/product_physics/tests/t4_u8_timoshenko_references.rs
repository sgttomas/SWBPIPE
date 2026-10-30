//! T4-U8 (DEL-05-03 Design `section-bases-and-timoshenko.md` §§3, 7 and 8):
//! the product against the frozen independent layer-B references in
//! `validation/references/t4_u8/product_cases.json`, through the captured
//! entry in both solver modes, under the references' own criterion. Every
//! expected and wrong value is read from the reference files; none is computed
//! through the product. A disagreement goes back to the reference owner and
//! its refuter, never to an edited expectation (§7).

use serde_json::{json, Value};
use std::collections::BTreeMap;

#[path = "common/current_model.rs"]
mod current_model;
#[path = "common/frozen_rows.rs"]
mod frozen_rows;
use frozen_rows::*;

const CASES: &str = include_str!("../../../validation/references/t4_u8/product_cases.json");
const I8: &str = include_str!("../../../validation/references/t4_i8/rebuilt_reference_cases.json");
// Literal include paths, which the hosted CI selector (tools/ci/numerical_ci.py)
// resolves; each request is still bound by the SHA-256 its reference names.
const REQUESTS: [(&str, &str); 17] = [
    (
        "requests/I8-CASE2-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/I8-CASE2-TIMOSHENKO.request.json"),
    ),
    (
        "requests/I8-CASE2-TIMOSHENKO-LOAD-STATE.request.json",
        include_str!("../../../validation/references/t4_u8/requests/I8-CASE2-TIMOSHENKO-LOAD-STATE.request.json"),
    ),
    (
        "requests/LOAD-STATE-PER-CASE-NU-EULER-BERNOULLI.request.json",
        include_str!("../../../validation/references/t4_u8/requests/LOAD-STATE-PER-CASE-NU-EULER-BERNOULLI.request.json"),
    ),
    (
        "requests/LOAD-STATE-PER-CASE-NU-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/LOAD-STATE-PER-CASE-NU-TIMOSHENKO.request.json"),
    ),
    (
        "requests/S-B-L1-DUALBASIS-EULER-BERNOULLI.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L1-DUALBASIS-EULER-BERNOULLI.request.json"),
    ),
    (
        "requests/S-B-L1-DUALBASIS-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L1-DUALBASIS-TIMOSHENKO.request.json"),
    ),
    (
        "requests/S-B-L1-EULER-BERNOULLI.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L1-EULER-BERNOULLI.request.json"),
    ),
    (
        "requests/S-B-L1-ROTATED-EULER-BERNOULLI.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L1-ROTATED-EULER-BERNOULLI.request.json"),
    ),
    (
        "requests/S-B-L1-ROTATED-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L1-ROTATED-TIMOSHENKO.request.json"),
    ),
    (
        "requests/S-B-L1-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L1-TIMOSHENKO.request.json"),
    ),
    (
        "requests/S-B-L10-EULER-BERNOULLI.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L10-EULER-BERNOULLI.request.json"),
    ),
    (
        "requests/S-B-L10-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L10-TIMOSHENKO.request.json"),
    ),
    (
        "requests/S-B-L2-EULER-BERNOULLI.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L2-EULER-BERNOULLI.request.json"),
    ),
    (
        "requests/S-B-L2-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-B-L2-TIMOSHENKO.request.json"),
    ),
    (
        "requests/S-E-SPRING-EULER-BERNOULLI.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-E-SPRING-EULER-BERNOULLI.request.json"),
    ),
    (
        "requests/S-E-SPRING-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/S-E-SPRING-TIMOSHENKO.request.json"),
    ),
    (
        "requests/TWIN-A-TIMOSHENKO.request.json",
        include_str!("../../../validation/references/t4_u8/requests/TWIN-A-TIMOSHENKO.request.json"),
    ),
];

fn cases() -> Value {
    serde_json::from_str(CASES).unwrap()
}

/// A case's requests (the primary and any alternates), each bound by its
/// SHA-256 and read unmodified.
fn requests(case: &Value) -> Vec<(String, Value)> {
    let bindings = std::iter::once(&case["request"]).chain(case["alternate_requests"].as_array().into_iter().flatten());
    bindings
        .map(|binding| {
            let path = binding["path"].as_str().unwrap();
            let text = REQUESTS.iter().find(|(p, _)| *p == path).unwrap_or_else(|| panic!("{path} is not included")).1;
            assert_eq!(sha256_hex(text.as_bytes()), binding["sha256"].as_str().unwrap(), "{path}: the request the reference binds");
            let request: Value = serde_json::from_str(text).unwrap();
            assert_eq!(request["model"]["pressure_contract"]["shear_model"], case["shear_model"], "{path}");
            (path.to_owned(), request)
        })
        .collect()
}

/// Outcome of one case in one mode on one request.
#[derive(Default)]
struct Tally {
    rows: usize,
    marked: usize,
    discriminator_rows: BTreeMap<String, usize>,
}

/// Every frozen row of `case` against `envelope` at its own criterion, and
/// every discriminator row not at its wrong value.
fn check_case(case: &Value, envelope: &Value, context: &str, tally: &mut Tally, failures: &mut Vec<String>) {
    let results = envelope["results"].as_array().unwrap();
    for lc in case["load_cases"].as_array().unwrap() {
        let id = lc["id"].as_str().unwrap();
        let rows = lc["rows"].as_array().unwrap();
        for row in rows {
            assert_eq!(row["basis_ref"]["ref_id"], json!(id));
            let expected = number(&row["expected"]);
            let criterion = &row["criterion"];
            let tolerance = criterion.get("relative_tolerance").map_or(0.0, number) * expected.abs()
                + criterion.get("absolute_tolerance").map_or(0.0, number);
            tally.rows += compare(results, row, id, expected, tolerance, &format!("{context} {id}"), failures);
            if row.pointer("/status/shear_model") == Some(&json!("shear_dependent")) {
                tally.marked += 1;
            }
        }
        for discriminator in lc["discriminators"].as_array().unwrap() {
            let name = discriminator["id"].as_str().unwrap();
            for listed in discriminator["rows"].as_array().unwrap() {
                let frozen = rows
                    .iter()
                    .find(|r| ["kind", "entity_ref", "component", "location", "section_basis"].iter().all(|k| r.get(*k) == listed.get(*k)))
                    .unwrap_or_else(|| panic!("{context} {id} {name}: the listed row {} is frozen", label(listed, "")));
                assert_eq!(listed["unit"], frozen["unit"]);
                let wrong = number(&listed["wrong"]);
                let absolute = frozen["criterion"].get("absolute_tolerance").map_or(0.0, number);
                for basis in bases_for(listed) {
                    match published(results, frozen, id, basis).as_slice() {
                        [one] if discriminated(number(&one["value"]), wrong, absolute) => {
                            *tally.discriminator_rows.entry(name.to_owned()).or_default() += 1;
                        }
                        other => failures.push(format!(
                            "{context} {id} {name} {}: published {:?} is the wrong value {wrong:e}",
                            label(listed, basis),
                            other.iter().map(|r| number(&r["value"])).collect::<Vec<_>>()
                        )),
                    }
                }
            }
        }
    }
}

fn solved(request: &Value, mode: open_pipe_stress_product_physics::PreviewSolverMode, context: &str) -> Value {
    let envelope = solve(request, mode);
    assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{context}: {}", envelope["diagnostics"]);
    assert_eq!(envelope["formulation_basis"]["profile_id"], json!("exact_pressure_v3"), "{context}");
    assert!(passed(&envelope), "{context}: published Passed: {}", envelope["diagnostics"]);
    envelope
}

/// Design §7 (T4-I8 case 2 and twin A): the Timoshenko documents of T4-I8
/// case 2 (plain and with the load/reference-state group) and of twin A,
/// read unmodified. The rows T4-RV4 marked (case 2's u_y at B, C and D; twin
/// A's tip deflection) take the frozen Timoshenko values and none of the
/// listed wrong factors' values; every other row is T4-I8's (or the twin
/// fixture's) unchanged value, which the reference states row by row and which
/// is checked here against T4-I8's own frozen file.
#[test]
fn u8_i8_case2_and_twin_a_marked_rows_are_refrozen_and_the_rest_unchanged() {
    let all = cases();
    let i8: Value = serde_json::from_str(I8).unwrap();
    let i8_case = &i8["cases"]["tp_phys_pressure_halves"];
    let mut failures = Vec::new();
    let mut marked = BTreeMap::new();
    for case in all["cases"].as_array().unwrap() {
        let case_id = case["case_id"].as_str().unwrap();
        if !["I8-CASE2-TIMOSHENKO", "TWIN-A-TIMOSHENKO"].contains(&case_id) {
            continue;
        }
        // The reference's standing of each row against T4-I8.
        if case_id == "I8-CASE2-TIMOSHENKO" {
            for lc in case["load_cases"].as_array().unwrap() {
                let id = lc["id"].as_str().unwrap();
                let i8_rows = i8_case["load_cases"][id]["rows"].as_array().unwrap_or_else(|| panic!("T4-I8 {id}"));
                for row in lc["rows"].as_array().unwrap() {
                    let standing = row["status"]["t4_i8"].as_str().unwrap();
                    let twin = i8_rows
                        .iter()
                        .find(|r| ["kind", "entity_ref", "component", "location"].iter().all(|k| r[*k] == row[*k]))
                        .unwrap_or_else(|| panic!("T4-I8 {id} {}", label(row, "")));
                    let origin = &twin["reference_origin"];
                    let frozen_i8 = number(&i8.pointer(origin["pointer"].as_str().unwrap()).unwrap()["value"])
                        * unit_factor(origin["reference_unit"].as_str().unwrap(), twin["unit"].as_str().unwrap());
                    let expected = number(&row["expected"]);
                    let same = (frozen_i8 - expected).abs() <= 1e-12 * expected.abs().max(frozen_i8.abs());
                    match standing {
                        "unchanged_from_t4_i8" => assert!(same, "{id} {}: {expected:e} vs T4-I8 {frozen_i8:e}", label(row, "")),
                        "refrozen_t4_u8" => assert!(!same, "{id} {}: re-frozen", label(row, "")),
                        other => panic!("{other}"),
                    }
                }
            }
        }
        for (path, request) in requests(case) {
            for mode in MODES {
                let context = format!("{case_id} [{path}] {mode:?}");
                let envelope = solved(&request, mode, &context);
                let mut tally = Tally::default();
                check_case(case, &envelope, &context, &mut tally, &mut failures);
                assert!(tally.rows > 0 && !tally.discriminator_rows.is_empty(), "{context}");
                *marked.entry(case_id).or_insert(0) += tally.marked;
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures[..failures.len().min(20)].join("\n"));
    // Three marked rows per request and mode in case 2 (two requests), one in twin A.
    assert_eq!(marked, BTreeMap::from([("I8-CASE2-TIMOSHENKO", 3 * 2 * 2), ("TWIN-A-TIMOSHENKO", 2)]));
}

/// Every layer-B case of the T4-U8 freeze (S-B cantilevers and their rotated
/// and dual-basis forms, the S-E interior spring, T4-I8 case 2, twin A and the
/// per-case-ν load-state documents, each with its Euler–Bernoulli twin): every
/// row at its own criterion and every listed discriminator row not at its
/// wrong value, in both modes.
#[test]
fn u8_every_product_case_matches_in_both_modes() {
    let all = cases();
    let mut failures = Vec::new();
    let mut by_case = BTreeMap::new();
    let mut discriminators = BTreeMap::new();
    for case in all["cases"].as_array().unwrap() {
        let case_id = case["case_id"].as_str().unwrap();
        for (path, request) in requests(case) {
            for mode in MODES {
                let context = format!("{case_id} [{path}] {mode:?}");
                let envelope = solved(&request, mode, &context);
                let mut tally = Tally::default();
                check_case(case, &envelope, &context, &mut tally, &mut failures);
                *by_case.entry(case_id.to_owned()).or_insert(0) += tally.rows;
                for (name, n) in tally.discriminator_rows {
                    *discriminators.entry(name).or_insert(0) += n;
                }
            }
        }
    }
    eprintln!("T4-U8: rows per case {by_case:?}; discriminator rows {discriminators:?}");
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures[..failures.len().min(30)].join("\n"));
    assert_eq!(by_case.len(), all["cases"].as_array().unwrap().len());
}
