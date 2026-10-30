//! Comparison of product envelopes with frozen independent reference rows
//! (T4-U6, T4-I8 and T4-U8; DEL-05-03 Design `section-bases-and-timoshenko.md`
//! §§2.1, 2.5, 7 and 8). Every expected and wrong value comes from a reference
//! file; nothing here computes one. A row is identified by (kind, entity_ref,
//! component, location, load case) and, on the D-5 stress kinds, its section
//! basis; each comparison uses the reference's own criterion, and each
//! negative assertion the references' rule |observed − wrong| >
//! max(absolute tolerance, 1e-9·max(|observed|, |wrong|)).
#![allow(dead_code)]
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

/// The captured entry on a frozen request, re-authored in memory to the current
/// version by the shared rule (`common/current_model.rs`; a 0.5.0 request by its
/// version alone); the frozen bytes are never edited.
pub fn solve(request: &Value, mode: PreviewSolverMode) -> Value {
    let request = crate::current_model::reauthor_request(request.clone());
    serde_json::to_value(
        run_linear_static_preview_value_with_mode(request, mode).expect("the captured entry accepts the request"),
    )
    .unwrap()
}

/// Whether the envelope publishes NUMERICAL_INTEGRITY_CHECKS_PASSED.
pub fn passed(envelope: &Value) -> bool {
    envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == json!("NUMERICAL_INTEGRITY_CHECKS_PASSED"))
}

/// The row kinds published once per section basis on an exact document
/// (Design §2.1).
pub const D5_STRESS_KINDS: [&str; 8] = [
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_axial_membrane_stress_v2",
    "pipe_lame_radial_stress_v2",
    "pipe_lame_hoop_stress_v2",
    "pipe_elastic_normal_stress_maximum_v2",
];
/// The two section bases, nominal first (Design §2.5).
pub const BASES: [&str; 2] = ["nominal_section", "reduced_section"];

pub fn number(value: &Value) -> f64 {
    match value {
        Value::String(text) => text.parse().unwrap_or_else(|_| panic!("decimal {text}")),
        Value::Number(number) => number.as_f64().unwrap(),
        other => panic!("not a number: {other}"),
    }
}

/// The section basis a published row names (the `; section_basis=…` suffix
/// of its recovery token), or "" for a row published once.
pub fn section_basis(row: &Value) -> &str {
    row["metadata"]["basis"]
        .as_str()
        .and_then(|basis| basis.rsplit_once("; section_basis="))
        .map_or("", |(_, basis)| basis)
}

/// The exact definitional factor from a reference unit to the row unit.
pub fn unit_factor(reference_unit: &str, row_unit: &str) -> f64 {
    match (reference_unit, row_unit) {
        ("m", "mm") => 1000.0,
        ("Pa", "MPa") => 1e-6,
        (a, b) if a == b => 1.0,
        other => panic!("unit pair {other:?}"),
    }
}

/// The bases a frozen row is checked on: its own `section_basis`; or, for a
/// D-5 stress kind frozen with one value where the bases coincide (zero mill
/// tolerance and allowance), both families against that one value (Design
/// §7); otherwise the row published once.
pub fn bases_for(row: &Value) -> Vec<&'static str> {
    match row["section_basis"].as_str() {
        Some("nominal_section") => vec!["nominal_section"],
        Some("reduced_section") => vec!["reduced_section"],
        Some(other) => panic!("unknown section_basis {other}"),
        None if D5_STRESS_KINDS.contains(&row["kind"].as_str().unwrap()) => BASES.to_vec(),
        None => vec![""],
    }
}

/// The published rows with a frozen row's identity on `basis` ("" matches only
/// rows published once). `case` is the load case the row belongs to.
pub fn published<'a>(results: &'a [Value], row: &Value, case: &str, basis: &str) -> Vec<&'a Value> {
    results
        .iter()
        .filter(|r| {
            r["kind"] == row["kind"]
                && r["entity_ref"] == row["entity_ref"]
                && r["metadata"]["component"] == row["component"]
                && r["metadata"]["location"] == row["location"]
                && r["basis_ref"]["ref_id"] == case
                && section_basis(r) == basis
        })
        .collect()
}

/// A short identity for messages.
pub fn label(row: &Value, basis: &str) -> String {
    format!(
        "{} {} {} {}{}",
        row["kind"].as_str().unwrap_or(""),
        row["entity_ref"].as_str().unwrap_or(""),
        row["location"].as_str().unwrap_or(""),
        row["component"].as_str().unwrap_or(""),
        if basis.is_empty() { String::new() } else { format!(" [{basis}]") }
    )
}

/// Compares one frozen row, on each of its bases, with the envelope: exactly
/// one published row within `tolerance` of `expected` (both in the row unit).
/// Returns the number of comparisons; failures are appended.
pub fn compare(
    results: &[Value],
    row: &Value,
    case: &str,
    expected: f64,
    tolerance: f64,
    context: &str,
    failures: &mut Vec<String>,
) -> usize {
    let bases = bases_for(row);
    for basis in &bases {
        let found = published(results, row, case, basis);
        match found.as_slice() {
            [one] if (number(&one["value"]) - expected).abs() <= tolerance => {}
            other => failures.push(format!(
                "{context} {}: published {:?}, frozen {expected:e} (tolerance {tolerance:e})",
                label(row, basis),
                other.iter().map(|r| number(&r["value"])).collect::<Vec<_>>()
            )),
        }
    }
    bases.len()
}

/// The references' negative assertion: the published value is not the wrong
/// mapping's value.
pub fn discriminated(observed: f64, wrong: f64, absolute: f64) -> bool {
    (observed - wrong).abs() > absolute.max(1e-9 * observed.abs().max(wrong.abs()))
}

/// Every published row of `case` whose kind is in `kinds`.
pub fn any_of_kinds(results: &[Value], case: &str, kinds: &[Value]) -> Vec<String> {
    results
        .iter()
        .filter(|r| r["basis_ref"]["ref_id"] == case && kinds.contains(&r["kind"]))
        .map(|r| format!("{} {}", r["kind"], r["id"]))
        .collect()
}

/// SHA-256 of a request's bytes, lowercase hex.
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

// ------------------------------------------------- T4-I8 MILLTOL (case 1)

const MILLTOL: &str = "milltol_lame_membrane";

/// T4-I8's MILLTOL variants, re-frozen on D-5: each document form (the frozen 0.5.0
/// document, re-authored in memory by [`solve`], and its load/reference-state twin, each also under the
/// reference's own `shear_model_variant` patch, which leaves every row) solved
/// in both modes and published Passed: (variant, form, mode, envelope).
pub fn milltol_runs(i8: &Value) -> Vec<(String, String, PreviewSolverMode, Value)> {
    let mut out = Vec::new();
    for (variant, body) in i8["cases"][MILLTOL]["variants"].as_object().unwrap() {
        let documents = &body["documents"];
        let patch = &documents["shear_model_variant"];
        assert_eq!((patch["op"].as_str(), patch["path"].as_str()), (Some("replace"), Some("/model/pressure_contract/shear_model")));
        for form in ["v3_model_0.5.0", "v3_model_0.5.0_load_state_group"] {
            let document = documents[form].clone();
            assert_eq!(document["model"]["schema_version"], json!("0.5.0"), "{variant} {form}: the frozen document");
            let mut patched = document.clone();
            patched["model"]["pressure_contract"]["shear_model"] = patch["value"].clone();
            for (label, request) in [(form.to_owned(), document), (format!("{form} + shear_model_variant"), patched)] {
                for mode in MODES {
                    let envelope = solve(&request, mode);
                    assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "MILLTOL {variant} {label} {mode:?}: {}", envelope["diagnostics"]);
                    assert!(passed(&envelope), "MILLTOL {variant} {label} {mode:?}: {}", envelope["diagnostics"]);
                    out.push((variant.clone(), label.clone(), mode, envelope));
                }
            }
        }
    }
    out
}

/// MILLTOL's frozen rows of `variant` that `keep` selects, each at its own
/// criterion. Returns the number of comparisons.
pub fn milltol_rows(
    i8: &Value,
    variant: &str,
    envelope: &Value,
    keep: &dyn Fn(&Value) -> bool,
    context: &str,
    failures: &mut Vec<String>,
) -> usize {
    let mut checked = 0;
    for (case, lc) in i8["cases"][MILLTOL]["variants"][variant]["load_cases"].as_object().unwrap() {
        for row in lc["rows"].as_array().unwrap().iter().filter(|row| keep(row)) {
            let origin = &row["reference_origin"];
            let expected = number(&i8.pointer(origin["pointer"].as_str().unwrap()).unwrap()["value"])
                * unit_factor(origin["reference_unit"].as_str().unwrap(), row["unit"].as_str().unwrap());
            let criterion = &row["criterion"];
            let tolerance = number(&criterion["relative_tolerance"]) * expected.abs() + number(&criterion["absolute_tolerance"]);
            checked += compare(envelope["results"].as_array().unwrap(), row, case, expected, tolerance, &format!("MILLTOL {context} {case}"), failures);
        }
    }
    checked
}

/// MILLTOL's discriminators named in `ids` (all when empty) that target
/// `variant`: every row carrying the discriminator's target quantity is not
/// its wrong value. Counts evaluated rows per discriminator.
pub fn milltol_discriminators(
    i8: &Value,
    variant: &str,
    envelope: &Value,
    ids: &[&str],
    context: &str,
    evaluated: &mut BTreeMap<String, usize>,
    failures: &mut Vec<String>,
) {
    let case = &i8["cases"][MILLTOL];
    for discriminator in case["wrong_result_discriminators"].as_array().unwrap() {
        let id = discriminator["id"].as_str().unwrap();
        let target = &discriminator["target"];
        if !(ids.is_empty() || ids.contains(&id)) || !target["variants"].as_array().unwrap().contains(&json!(variant)) {
            continue;
        }
        let mut targeted = 0;
        for (load_case, lc) in case["variants"][variant]["load_cases"].as_object().unwrap() {
            for row in lc["rows"].as_array().unwrap().iter().filter(|r| r["expected"] == target["expected"]) {
                targeted += 1;
                let wrong = number(&discriminator["wrong"]["value"])
                    * unit_factor(discriminator["wrong"]["unit"].as_str().unwrap(), row["unit"].as_str().unwrap());
                let absolute = number(&row["criterion"]["absolute_tolerance"]);
                for basis in bases_for(row) {
                    match published(envelope["results"].as_array().unwrap(), row, load_case, basis).as_slice() {
                        [one] if discriminated(number(&one["value"]), wrong, absolute) => {
                            *evaluated.entry(id.to_owned()).or_default() += 1;
                        }
                        other => failures.push(format!(
                            "MILLTOL {context} {id} {}: published {:?} is the wrong value {wrong:e}",
                            label(row, basis),
                            other.iter().map(|r| number(&r["value"])).collect::<Vec<_>>()
                        )),
                    }
                }
            }
        }
        assert!(targeted > 0, "MILLTOL {variant} {id}: a row carries {}", target["expected"]);
    }
}
