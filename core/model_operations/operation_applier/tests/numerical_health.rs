//! DEL-07-05 numerical health (Design `numerical-health-indicator.md` §7): the
//! parse over live producer output in both solver modes, its tie to the Rust
//! reader, and every committed result fixture.
use open_pipe_stress_operation_applier::{numerical_health_json, reader_dispatch_json};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const LEAD_POLICY: &str = "M03-INTEGRITY-v1 represented original-equation structural evidence for load case ";
const LINEAR_TEXT: &str = "The contribution audit distinguishes intended assembly from stored equations; physical formulation limitations remain applicable.";
const NONLINEAR_TEXT: &str = "final same-state evaluated equilibrium and derived residual-work evidence";
const PAIR: [&str; 2] = ["NUMERICAL_CASE_EVIDENCE_INCOMPLETE", "NUMERICAL_INTEGRITY_NOT_QUALIFIED"];
const MODES: [(PreviewSolverMode, &str); 2] = [
    (PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    (PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
];

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures")
}

fn read(path: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn health(envelope: &Value, findings: Option<&str>) -> Value {
    serde_json::from_str(&numerical_health_json(&envelope.to_string(), findings).unwrap()).unwrap()
}

fn state_of(solve_quality: &Value) -> &'static str {
    match solve_quality.as_str() {
        Some("checks_passed") => "checks_passed",
        Some("sensitive") => "sensitive",
        Some("unresolved") => "quality_unresolved",
        Some("failed") => "solve_failed",
        _ => "not_assessed",
    }
}

/// The case's integrity diagnostics (by id).
fn integrity<'a>(envelope: &'a Value, case: &str) -> Vec<&'a Value> {
    let id = format!("diagnostic:numerical-integrity:{case}");
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["id"] == id.as_str()).collect()
}

/// An independent reading of a linear passing/sensitive report: the lead, the
/// linear sentence, no nonlinear evidence, and the rcond token by plain split.
fn expected_rcond(envelope: &Value, case: &str) -> Option<f64> {
    let found = integrity(envelope, case);
    let [d] = found.as_slice() else { return None };
    if !matches!(d["code"].as_str(), Some("NUMERICAL_INTEGRITY_CHECKS_PASSED" | "NUMERICAL_INTEGRITY_SENSITIVE")) {
        return None;
    }
    let message = d["message"].as_str().unwrap();
    if !message.starts_with(&format!("{LEAD_POLICY}{case}: StructuralReport {{")) || !message.contains(LINEAR_TEXT) || message.contains(NONLINEAR_TEXT) {
        return None;
    }
    assert_eq!(message.matches("reciprocal_condition_estimate: ").count(), 1, "{case}: one rcond field");
    let token = message.split("reciprocal_condition_estimate: ").nth(1).unwrap().split(',').next().unwrap();
    Some(token.parse().unwrap())
}

/// The checks every envelope meets: one case entry per `numerical_quality`
/// case with its state; a figure exactly for each linear passing/sensitive
/// report, equal to its text bit for bit, with the kernel's comparison.
fn check_cases(envelope: &Value, out: &Value, label: &str) -> (usize, usize) {
    let cases = out["cases"].as_array().unwrap();
    let quality = envelope["numerical_quality"]["cases"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let mut figures = (0, 0);
    for entry in quality {
        let Some(case) = entry["basis_ref"]["ref_id"].as_str() else { continue };
        let health: Vec<&Value> = cases.iter().filter(|c| c["case_id"] == case).collect();
        assert_eq!(health.len(), 1, "{label} {case}");
        let health = health[0];
        assert_eq!(health["state"], state_of(&entry["solve_quality"]), "{label} {case}");
        match expected_rcond(envelope, case) {
            Some(rcond) => {
                let parsed = health["rcond"].as_f64().unwrap_or_else(|| panic!("{label} {case}: no figure"));
                assert_eq!(parsed.to_bits(), rcond.to_bits(), "{label} {case}");
                assert!(parsed.is_finite() && parsed > 0.0 && parsed <= 1.0);
                assert_eq!(health["digits"].as_f64().unwrap(), 0.0 - parsed.log10());
                assert_eq!(health["conditioning_sensitive"], parsed < f64::EPSILON.sqrt());
                if health["state"] == "checks_passed" { figures.0 += 1 } else { figures.1 += 1 }
            }
            None => {
                assert_eq!((&health["rcond"], &health["digits"]), (&Value::Null, &Value::Null), "{label} {case}");
                assert_eq!(health["conditioning_sensitive"], false);
            }
        }
    }
    (figures.0, figures.1)
}

struct Live {
    request: &'static str,
    route: &'static str,
    linear: bool,
    states: &'static [&'static str],
    verdict_only: bool,
}

/// Runs the product entry in both modes; checks the parse, and ties
/// `verdict_only` to the reader's own findings for the produced result.
fn live(spec: &Live) -> Vec<(Value, Value)> {
    let request = if spec.request.ends_with("invented_demo_model.json") {
        json!({"model": read(&fixtures().join(spec.request)), "materials": []})
    } else {
        read(&fixtures().join(spec.request))
    };
    let mut outs = Vec::new();
    for (mode, wire) in MODES {
        let label = format!("{} {wire}", spec.request);
        let envelope = serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
        let invocation = json!({"request": request, "solver_mode": wire}).to_string();
        let model = request["model"].to_string();
        let dispatch = reader_dispatch_json(&envelope.to_string(), Some(&model), Some(&invocation)).unwrap();
        let out = health(&envelope, Some(&dispatch));
        assert_eq!(out["route"], spec.route, "{label}");
        let states: Vec<&str> = out["cases"].as_array().unwrap().iter().map(|c| c["state"].as_str().unwrap()).collect();
        assert_eq!(states, spec.states, "{label}");
        let (passed, sensitive) = check_cases(&envelope, &out, &label);
        if spec.linear {
            let reported = spec.states.iter().filter(|s| matches!(**s, "checks_passed" | "sensitive")).count();
            assert_eq!(passed + sensitive, reported, "{label}: every linear passing/sensitive case has one figure");
        } else {
            assert!(out["cases"].as_array().unwrap().iter().all(|c| c["rcond"].is_null()), "{label}: nonlinear");
        }
        // The reader tie: verdict_only implies the reader's findings are the pair.
        let dispatch: Value = serde_json::from_str(&dispatch).unwrap();
        let mut findings: Vec<&str> = dispatch["standing"]["findings"].as_array().unwrap().iter().map(|f| f.as_str().unwrap()).collect();
        findings.sort_unstable();
        if out["verdict_only"] == true {
            assert_eq!(findings, PAIR, "{label}");
        }
        assert_eq!(out["verdict_only"], spec.verdict_only, "{label}: reader findings {findings:?}");
        // Without the reader's findings the flag is false.
        assert_eq!(health(&envelope, None)["verdict_only"], false);
        outs.push((envelope, out));
    }
    outs
}

#[test]
fn a_linear_passing_pressure_2_result_with_several_cases() {
    live(&Live {
        request: "product_preview/pressure/load_reference/pressure.request.json",
        route: "generic",
        linear: true,
        states: &["checks_passed", "checks_passed"],
        verdict_only: false,
    });
}

#[test]
fn a_sensitive_generic_result_is_verdict_only() {
    for (_, out) in live(&Live {
        request: "product_preview/w1_controls/ui/multicase-sparse_interactive.request.json",
        route: "generic",
        linear: true,
        states: &["sensitive", "sensitive"],
        verdict_only: true,
    }) {
        // Both cases are sensitive on conditioning alone (rcond about 1.06e-11).
        for case in out["cases"].as_array().unwrap() {
            assert_eq!(case["conditioning_sensitive"], true);
            let kinds: Vec<&str> = case["causes"].as_array().unwrap().iter().map(|c| c["kind"].as_str().unwrap()).collect();
            assert_eq!(kinds, ["conditioning"]);
        }
        assert_eq!(out["combinations"], json!([]));
    }
}

#[test]
fn a_kd5_formation_demotion_names_its_row() {
    for (envelope, out) in live(&Live {
        request: "product_preview/rf_skew_t_cant_off_122_r1e-04.request.json",
        route: "generic",
        linear: true,
        states: &["sensitive"],
        verdict_only: true,
    }) {
        let case = &out["cases"][0];
        let message = integrity(&envelope, "case")[0]["message"].as_str().unwrap();
        let causes = case["causes"].as_array().unwrap();
        let formation: Vec<&Value> = causes.iter().filter(|c| c["kind"] == "formation").collect();
        assert_eq!(formation.len(), 1, "{causes:?}");
        // The row the K-D5 line names, when it names one, is the location.
        let row = message.split("formation_check: reason=estimate; row=").nth(1).map(|r| r.split("; ").next().unwrap());
        let locations = formation[0]["locations"].as_array().unwrap();
        match row {
            Some(row) => {
                let (node, dof) = row.rsplit_once(':').unwrap();
                assert_eq!(locations, &vec![json!({"kind": "node_dof", "node": node, "dof": dof})]);
            }
            None => assert!(locations.is_empty()),
        }
    }
}

#[test]
fn a_source_selected_ordinary_failure_and_a_recovered_sensitive_case() {
    // n06: the ordinary attempt is refused (assembly unresolved); the
    // composite's standing comes from its recovery evidence.
    for (envelope, out) in live(&Live {
        request: "product_preview/pressure_source/physics_source/n06.request.json",
        route: "pressure_source",
        linear: true,
        states: &["quality_unresolved"],
        verdict_only: false,
    }) {
        let case = &out["cases"][0];
        assert_eq!(case["code"], "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED", "{envelope:?}");
        assert!(case["rcond"].is_null());
    }
    live(&Live {
        request: "product_preview/pressure_source/physics_source/n05.request.json",
        route: "pressure_source",
        linear: true,
        states: &["sensitive"],
        verdict_only: false,
    });
}

#[test]
fn a_nonlinear_normal_publication_has_no_figure_and_its_combination_is_withheld() {
    for (_, out) in live(&Live {
        request: "product_preview/invented_demo_model.json",
        route: "generic",
        linear: false,
        states: &["checks_passed", "checks_passed"],
        verdict_only: false,
    }) {
        assert_eq!(out["combinations"], json!([
            {"combination_id": "combination:C-OPER-ALT", "state": "not_assessed", "cases": [], "withheld": true}]));
    }
}

/// Every object under `value` that is a result envelope.
fn envelopes<'a>(value: &'a Value, out: &mut Vec<&'a Value>) {
    match value {
        Value::Object(map) => {
            if map.get("numerical_quality").is_some_and(Value::is_object) && map.get("diagnostics").is_some_and(Value::is_array) {
                out.push(value);
            }
            map.values().for_each(|v| envelopes(v, out));
        }
        Value::Array(items) => items.iter().for_each(|v| envelopes(v, out)),
        _ => {}
    }
}

fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

#[test]
fn every_committed_result_fixture_parses() {
    let root = fixtures();
    let mut files = Vec::new();
    json_files(&root, &mut files);
    files.sort();
    let (mut envelope_count, mut preview, mut all) = (0, (0, 0), (0, 0));
    for path in files {
        let text = std::fs::read_to_string(&path).unwrap();
        if !text.contains("NUMERICAL_INTEGRITY_") {
            continue;
        }
        let value: Value = serde_json::from_str(&text).unwrap();
        let mut found = Vec::new();
        envelopes(&value, &mut found);
        let label = path.strip_prefix(&root).unwrap().display().to_string();
        for envelope in found {
            envelope_count += 1;
            let out = health(envelope, None);
            assert_eq!(out["verdict_only"], false);
            let (passed, sensitive) = check_cases(envelope, &out, &label);
            all = (all.0 + passed, all.1 + sensitive);
            if label.starts_with("product_preview") {
                preview = (preview.0 + passed, preview.1 + sensitive);
            }
            // The reader tie over the corpus: with the reader's own findings
            // (unbound), verdict_only implies the pair.
            let dispatch = reader_dispatch_json(&envelope.to_string(), None, None).unwrap();
            let bound = health(envelope, Some(&dispatch));
            if bound["verdict_only"] == true {
                let dispatch: Value = serde_json::from_str(&dispatch).unwrap();
                let mut findings: Vec<&str> = dispatch["standing"]["findings"].as_array().unwrap().iter().map(|f| f.as_str().unwrap()).collect();
                findings.sort_unstable();
                assert_eq!(findings, PAIR, "{label}");
            }
        }
    }
    println!("envelopes {envelope_count}; figures (passed, sensitive): product_preview {preview:?}, all {all:?}");
    // The committed corpus as of this change (linear reports only; the four
    // nonlinear passing product_preview cases carry no figure by design).
    assert_eq!(preview, (14, 31));
    assert!(envelope_count > 0 && all.0 >= preview.0 && all.1 >= preview.1);
}
