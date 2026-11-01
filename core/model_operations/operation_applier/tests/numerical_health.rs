//! DEL-07-05 numerical health (Design `numerical-health-indicator.md` §7): the
//! parse over live producer output in both solver modes, its tie to the Rust
//! reader, and every committed result fixture that carries integrity
//! diagnostics (listed exactly in `RESULT_FIXTURES`).
use open_pipe_stress_operation_applier::{numerical_health_json, reader_dispatch_json};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const LEAD_POLICY: &str =
    "M03-INTEGRITY-v1 represented original-equation structural evidence for load case ";
const LINEAR_TEXT: &str = "The contribution audit distinguishes intended assembly from stored equations; physical formulation limitations remain applicable.";
const NONLINEAR_TEXT: &str =
    "final same-state evaluated equilibrium and derived residual-work evidence";
const PAIR: [&str; 2] = [
    "NUMERICAL_CASE_EVIDENCE_INCOMPLETE",
    "NUMERICAL_INTEGRITY_NOT_QUALIFIED",
];
const MODES: [(PreviewSolverMode, &str); 2] = [
    (PreviewSolverMode::SparseInteractive, "sparse_interactive"),
    (PreviewSolverMode::DenseScrutiny, "dense_scrutiny"),
];
const BADGES: [&str; 5] = [
    "checks_passed",
    "not_assessed",
    "sensitive",
    "quality_unresolved",
    "solve_failed",
];

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures")
}

/// Reads one fixture, relative to `fixtures/`.
fn read(name: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(fixtures().join(name)).unwrap()).unwrap()
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
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["id"] == id.as_str())
        .collect()
}

/// An independent reading of a linear passing/sensitive report: the lead, the
/// linear sentence, no nonlinear evidence, and the rcond token by plain split.
fn expected_rcond(envelope: &Value, case: &str) -> Option<f64> {
    let found = integrity(envelope, case);
    let [d] = found.as_slice() else { return None };
    if !matches!(
        d["code"].as_str(),
        Some("NUMERICAL_INTEGRITY_CHECKS_PASSED" | "NUMERICAL_INTEGRITY_SENSITIVE")
    ) {
        return None;
    }
    let message = d["message"].as_str().unwrap();
    if !message.starts_with(&format!("{LEAD_POLICY}{case}: StructuralReport {{"))
        || !message.contains(LINEAR_TEXT)
        || message.contains(NONLINEAR_TEXT)
    {
        return None;
    }
    assert_eq!(
        message.matches("reciprocal_condition_estimate: ").count(),
        1,
        "{case}: one rcond field"
    );
    let token = message
        .split("reciprocal_condition_estimate: ")
        .nth(1)
        .unwrap()
        .split(',')
        .next()
        .unwrap();
    Some(token.parse().unwrap())
}

/// The checks every envelope meets: one case entry per `numerical_quality`
/// case with its state; a figure exactly for each linear passing/sensitive
/// report, equal to its text bit for bit, with the kernel's comparison; and
/// each combination's state the worst badge of its cases.
fn check_cases(envelope: &Value, out: &Value, label: &str) -> (usize, usize) {
    let cases = out["cases"].as_array().unwrap();
    let quality = envelope["numerical_quality"]["cases"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let mut figures = (0, 0);
    for entry in quality {
        let Some(case) = entry["basis_ref"]["ref_id"].as_str() else {
            continue;
        };
        let health: Vec<&Value> = cases.iter().filter(|c| c["case_id"] == case).collect();
        assert_eq!(health.len(), 1, "{label} {case}");
        let health = health[0];
        assert_eq!(
            health["state"],
            state_of(&entry["solve_quality"]),
            "{label} {case}"
        );
        match expected_rcond(envelope, case) {
            Some(rcond) => {
                let parsed = health["rcond"]
                    .as_f64()
                    .unwrap_or_else(|| panic!("{label} {case}: no figure"));
                assert_eq!(parsed.to_bits(), rcond.to_bits(), "{label} {case}");
                assert!(parsed.is_finite() && parsed > 0.0 && parsed <= 1.0);
                assert_eq!(health["digits"].as_f64().unwrap(), 0.0 - parsed.log10());
                assert_eq!(
                    health["conditioning_sensitive"],
                    parsed < f64::EPSILON.sqrt()
                );
                if health["state"] == "checks_passed" {
                    figures.0 += 1
                } else {
                    figures.1 += 1
                }
            }
            None => {
                assert_eq!(
                    (&health["rcond"], &health["digits"]),
                    (&Value::Null, &Value::Null),
                    "{label} {case}"
                );
                assert_eq!(health["conditioning_sensitive"], false);
            }
        }
    }
    for combination in out["combinations"].as_array().unwrap() {
        if combination["withheld"] == true {
            assert!(combination["state"].is_null(), "{label} {combination}");
            continue;
        }
        let named = combination["cases"].as_array().unwrap();
        assert!(!named.is_empty(), "{label} {combination}");
        let worst = named
            .iter()
            .map(|case| {
                cases
                    .iter()
                    .find(|c| c["case_id"] == *case)
                    .map_or("not_assessed", |c| c["state"].as_str().unwrap())
            })
            .map(|state| BADGES.iter().position(|b| *b == state).unwrap())
            .max()
            .unwrap();
        assert_eq!(combination["state"], BADGES[worst], "{label} {combination}");
    }
    (figures.0, figures.1)
}

/// The reader's findings in a dispatch outcome, sorted.
fn findings(dispatch: &str) -> Vec<String> {
    let dispatch: Value = serde_json::from_str(dispatch).unwrap();
    let mut codes: Vec<String> = dispatch["standing"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f.as_str().unwrap().to_string())
        .collect();
    codes.sort_unstable();
    codes
}

fn dispatch(envelope: &Value, request: &Value, mode: &str) -> String {
    let invocation = json!({"request": request, "solver_mode": mode}).to_string();
    reader_dispatch_json(
        &envelope.to_string(),
        Some(&request["model"].to_string()),
        Some(&invocation),
    )
    .unwrap()
}

struct Live {
    request: &'static str,
    route: &'static str,
    linear: bool,
    states: &'static [&'static str],
    verdict_only: bool,
}

/// `{"model": …, "materials": []}` around a committed model document.
fn model_request(name: &str) -> Value {
    json!({"model": read(name), "materials": []})
}

/// Runs the product entry in both modes; checks the parse, and ties
/// `verdict_only` to the reader's own findings for the produced result.
fn live(spec: &Live) -> Vec<(Value, Value)> {
    let request = if spec.request.ends_with("_model.json") {
        model_request(spec.request)
    } else {
        read(spec.request)
    };
    live_request(spec, request)
}

fn live_request(spec: &Live, request: Value) -> Vec<(Value, Value)> {
    live_entry(spec, request, false)
}

/// `typed`: the historical typed entry (`run_linear_static_preview_with_mode`),
/// as the engine's own S11-G probes run, for inputs the captured entry's
/// checked JSON refuses (an integral 1e80).
fn live_entry(spec: &Live, request: Value, typed: bool) -> Vec<(Value, Value)> {
    let mut outs = Vec::new();
    for (mode, wire) in MODES {
        let label = format!("{} {wire}", spec.request);
        let envelope = serde_json::to_value(if typed {
            let typed: LinearStaticPreviewRequest =
                serde_json::from_value(request.clone()).unwrap();
            run_linear_static_preview_with_mode(typed, mode)
        } else {
            run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()
        })
        .unwrap();
        let dispatch = dispatch(&envelope, &request, wire);
        let out = health(&envelope, Some(&dispatch));
        assert_eq!(out["route"], spec.route, "{label}");
        let states: Vec<&str> = out["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["state"].as_str().unwrap())
            .collect();
        assert_eq!(states, spec.states, "{label}");
        let (passed, sensitive) = check_cases(&envelope, &out, &label);
        if spec.linear {
            let reported = spec
                .states
                .iter()
                .filter(|s| matches!(**s, "checks_passed" | "sensitive"))
                .count();
            assert_eq!(
                passed + sensitive,
                reported,
                "{label}: every linear passing/sensitive case has one figure"
            );
        } else {
            assert!(
                out["cases"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|c| c["rcond"].is_null()),
                "{label}: nonlinear"
            );
        }
        // The reader tie: verdict_only implies the reader's findings are the pair.
        let codes = findings(&dispatch);
        if out["verdict_only"] == true {
            assert_eq!(codes, PAIR, "{label}");
        }
        assert_eq!(
            out["verdict_only"], spec.verdict_only,
            "{label}: reader findings {codes:?}"
        );
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
            let kinds: Vec<&str> = case["causes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| c["kind"].as_str().unwrap())
                .collect();
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
        // The K-D5 line names a row, and that row is the location.
        let row = message
            .split(" formation_check: reason=estimate; row=")
            .nth(1)
            .map(|r| r.split("; ").next().unwrap());
        let (node, dof) = row
            .expect("the K-D5 line names its row")
            .rsplit_once(':')
            .unwrap();
        assert_eq!(
            formation[0]["locations"],
            json!([{"kind": "node_dof", "node": node, "dof": dof}])
        );
    }
}

#[test]
fn a_source_selected_ordinary_failure_and_a_recovered_sensitive_case() {
    // n06: the ordinary attempt is refused (assembly unresolved); the
    // composite's standing comes from its recovery evidence.
    for (_, out) in live(&Live {
        request: "product_preview/pressure_source/physics_source/n06.request.json",
        route: "pressure_source",
        linear: true,
        states: &["quality_unresolved"],
        verdict_only: false,
    }) {
        let case = &out["cases"][0];
        assert_eq!(case["code"], "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED");
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
        assert_eq!(
            out["combinations"],
            json!([
            {"combination_id": "combination:C-OPER-ALT", "state": null, "cases": [], "withheld": true}])
        );
    }
}

/// The engine's own mixed nonlinear construction
/// (`product_physics` tests `mixed_nonlinear_preview_request`), formed from the
/// committed invented preview model: a two-node line with one-way, gap and
/// friction supports at its free end and one mechanics combination.
fn mixed_nonlinear_request() -> Value {
    let mut model = read("product_preview/invented_preview_model.json");
    let nodes = model["nodes"].as_array_mut().unwrap();
    nodes.truncate(2);
    nodes[0]["id"] = json!("node:N-100");
    nodes[0]["position"] = json!({"x": 0.0, "y": 0.0, "z": 0.0});
    nodes[1]["id"] = json!("node:N-110");
    nodes[1]["position"] = json!({"x": 1.0, "y": 0.0, "z": 0.0});
    let pipes = model["pipe_segments"].as_array_mut().unwrap();
    pipes.truncate(1);
    pipes[0]["id"] = json!("pipe:P-100");
    pipes[0]["from"] = json!("node:N-100");
    pipes[0]["to"] = json!("node:N-110");
    pipes[0]["y_reference"] = json!({"x": 0.0, "y": 1.0, "z": 0.0});
    let nonlinear = |id: &str, law: Value| {
        json!({"id": id, "node": "node:N-110", "restraints": [], "family": "nonlinear", "nonlinear": law,
        "provenance": "invented_example_user_entered_nonlinear_support_no_catalog"})
    };
    model["supports"] = json!([
        {"id": "support:S-100", "node": "node:N-100", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "family": "anchor", "provenance": "invented_example"},
        nonlinear("support:NL-MIX-ONEWAY-110", json!({"behavior": "one_way", "dof": "UX", "initial_state": "active", "active_when": "positive_reaction"})),
        nonlinear("support:NL-MIX-GAP-110", json!({"behavior": "gap", "dof": "UY", "initial_state": "inactive", "closes_when": "positive_displacement", "gap": {"value": 0.05, "unit": "mm"}})),
        nonlinear("support:NL-MIX-FRIC-110", json!({"behavior": "friction", "dof": "UZ", "initial_state": "sticking", "friction_coefficient": {"value": 0.30, "unit": "none"}, "normal_reaction": {"value": 10.0, "unit": "N"}})),
    ]);
    model["components"] = json!([]);
    let cases = model["load_cases"].as_array_mut().unwrap();
    cases.truncate(1);
    let id = "load:L-MIXED-NONLINEAR";
    cases[0]["id"] = json!(id);
    let load = |suffix: &str, direction: &str, value: f64| {
        json!({"id": format!("{id}-{suffix}"), "category": "occasional",
        "target": {"type": "node", "node": "node:N-110"}, "direction": direction, "magnitude": {"value": value, "unit": "N"},
        "dimension": "force", "provenance": "invented_example_user_input"})
    };
    cases[0]["primitive_loads"] = json!([
        load("X", "global_x", 100.0),
        load("Y", "global_y", 100_000.0),
        load("Z", "global_z", 100.0)
    ]);
    model["combinations"] = json!([{"id": "combination:C-MIXED-NONLINEAR", "basis": "mechanics", "terms": [{"load_case": id, "factor": 1.0}],
        "provenance": "invented_example_no_code_combination"}]);
    json!({"model": model, "materials": []})
}

#[test]
fn a_nonlinear_recovery_basis_refusal_is_unresolved_with_its_cause() {
    let spec = Live {
        request: "product_preview/invented_preview_model.json (mixed nonlinear)",
        route: "generic",
        linear: false,
        states: &["quality_unresolved"],
        // Held back only by its verdict: the reader's findings are exactly the pair.
        verdict_only: true,
    };
    for (envelope, out) in live_request(&spec, mixed_nonlinear_request()) {
        let case = &out["cases"][0];
        assert_eq!(
            case["code"], "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED",
            "{:?}",
            envelope["diagnostics"]
        );
        assert_eq!(
            case["causes"],
            json!([{"kind": "recovery_basis_unqualified",
            "diagnostic_id": "diagnostic:numerical-integrity:load:L-MIXED-NONLINEAR", "locations": []}])
        );
        assert!(case["rcond"].is_null());
    }
}

/// The engine's S11-G probe A (`product_physics` s11g_tests `probe_a(1e80,
/// [1, 0, -1])`): a cantilever under three uniform loads whose 1e80 terms
/// cancel, so the certified rows' bound far exceeds 1e-9 of the group norm and
/// the recovery guard demotes the case. `pipe` names its single member.
fn probe_a(pipe: &str) -> Value {
    const INVENTED: &str = "invented_t3_s11g_test_input_not_library_data";
    let load = |i: usize, value: f64| {
        json!({"id": format!("udl:{i}"), "category": "distributed_force",
        "target": {"type": "element", "pipe": pipe}, "direction": "global_y", "magnitude": {"value": value, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": INVENTED})
    };
    json!({"model": {
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:t3-s11g:probe-a", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [{"id": "root", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": INVENTED},
            {"id": "tip", "position": {"x": 2.0, "y": 0.0, "z": 0.0}, "provenance": INVENTED}],
        "pipe_segments": [{"id": pipe, "from": "root", "to": "tip", "material": "material",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
            "provenance": INVENTED}],
        "materials": [{"id": "material", "elastic_modulus": {"value": 200e9, "unit": "Pa"},
            "shear_modulus": {"value": 80e9, "unit": "Pa"},
            "thermal_expansion_coefficient": {"value": 1.2e-5, "unit": "1/degC"}, "provenance": INVENTED}],
        "supports": [{"id": "anchor", "node": "root", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": INVENTED}],
        "components": [],
        "load_cases": [{"id": "case", "label": "probe-a", "kind": "primitive_user_load",
            "primitive_loads": [load(0, 1e80), load(1, 0.3), load(2, -1e80)], "provenance": INVENTED}],
        "combinations": []},
        "materials": []})
}

#[test]
fn a_recovery_demotion_keeps_its_cause_for_any_member_id() {
    for pipe in ["pipe", "pipe]"] {
        let spec = Live {
            request: "probe A (S11-G recovery guard)",
            route: "generic",
            linear: true,
            states: &["sensitive"],
            verdict_only: true,
        };
        for (envelope, out) in live_entry(&spec, probe_a(pipe), true) {
            let message = integrity(&envelope, "case")[0]["message"].as_str().unwrap();
            assert!(
                message
                    .contains(" S11-G recovery guard (R-b'): load case case is Sensitive because "),
                "{pipe}: precondition"
            );
            let case = &out["cases"][0];
            let recovery: Vec<&Value> = case["causes"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|c| c["kind"] == "formation_recovery")
                .collect();
            assert_eq!(recovery.len(), 1, "{pipe}: {}", case["causes"]);
            let locations = recovery[0]["locations"].as_array().unwrap();
            if pipe == "pipe" {
                // Group names are `{member}.{location}.{kind}` (the pipe's and
                // the anchor support's).
                assert!(
                    locations
                        .iter()
                        .any(|l| l["text"].as_str().unwrap().starts_with("pipe.")),
                    "{locations:?}"
                );
            } else {
                // A name holding `]` is not parsed: the cause stays without locations.
                assert!(locations.is_empty(), "{locations:?}");
            }
        }
    }
}

/// Every object under `value` that is a result envelope, with the binding a
/// wrapper carries (`{"invocation": {request, solver_mode}, "source": …}`).
fn envelopes<'a>(
    value: &'a Value,
    binding: Option<(&'a Value, &'a str)>,
    out: &mut Vec<(&'a Value, Option<(&'a Value, &'a str)>)>,
) {
    match value {
        Value::Object(map) => {
            if map.get("numerical_quality").is_some_and(Value::is_object)
                && map.get("diagnostics").is_some_and(Value::is_array)
            {
                out.push((value, binding));
            }
            let carried = match (map.get("invocation"), map.get("source")) {
                (Some(invocation), Some(_)) if invocation["request"]["model"].is_object() => {
                    invocation["solver_mode"]
                        .as_str()
                        .map(|mode| (&invocation["request"], mode))
                }
                _ => None,
            };
            for (key, child) in map {
                envelopes(
                    child,
                    if key == "source" {
                        carried.or(binding)
                    } else {
                        None
                    },
                    out,
                );
            }
        }
        Value::Array(items) => items.iter().for_each(|v| envelopes(v, None, out)),
        _ => {}
    }
}

/// A product-preview fixture's request and mode, from its committed
/// neighbours: `{name}-{mode}.raw.json` beside `{name}.request.json` (or
/// `{name}-{mode}.request.json`), the desktop result beside its request, and
/// the invented demo result beside its model.
fn neighbour(name: &str) -> Option<(Value, &'static str)> {
    let mode = |text: &str| MODES.iter().map(|m| m.1).find(|m| text.contains(m));
    if let Some(stem) = name.strip_suffix(".raw.json") {
        let wire = mode(stem)?;
        let base = stem.strip_suffix(wire)?.strip_suffix('-')?;
        let request = REQUESTS.iter().find(|r| {
            **r == format!("{base}.request.json") || **r == format!("{stem}.request.json")
        })?;
        return Some((read(request), wire));
    }
    if let Some(stem) =
        name.strip_prefix("product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_")
    {
        return Some((
            read("product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"),
            mode(stem)?,
        ));
    }
    if let Some(stem) = name.strip_prefix("product_preview/invented_demo_result_preview_physics_1_")
    {
        let wire = if stem.starts_with("sparse") {
            "sparse_interactive"
        } else {
            "dense_scrutiny"
        };
        return Some((
            model_request("product_preview/invented_demo_model.json"),
            wire,
        ));
    }
    None
}

/// The paired requests the fixture-wide test reads.
const REQUESTS: [&str; 16] = [
    "product_preview/pressure/load_reference/connected.request.json",
    "product_preview/pressure/load_reference/pressure.request.json",
    "product_preview/pressure_source/load_reference_source/eigen_motion.request.json",
    "product_preview/pressure_source/load_reference_source/fields.request.json",
    "product_preview/pressure_source/load_reference_source/mixed.request.json",
    "product_preview/pressure_source/load_reference_source/n05.request.json",
    "product_preview/pressure_source/load_reference_source/n06.request.json",
    "product_preview/pressure_source/physics_source/fields.request.json",
    "product_preview/pressure_source/physics_source/mixed.request.json",
    "product_preview/pressure_source/physics_source/mixed_units.request.json",
    "product_preview/pressure_source/physics_source/n05.request.json",
    "product_preview/pressure_source/physics_source/n05_unicode.request.json",
    "product_preview/pressure_source/physics_source/n05_units.request.json",
    "product_preview/pressure_source/physics_source/n06.request.json",
    "product_preview/w1_controls/ui/multicase-dense_scrutiny.request.json",
    "product_preview/w1_controls/ui/multicase-sparse_interactive.request.json",
];

/// The envelope with every case made passing: the integrity code, the
/// verdict-linked fields `assessed_numerical_quality` derives, and the status.
fn all_passing(envelope: &Value) -> Value {
    let mut passing = envelope.clone();
    let cases: Vec<String> = passing["numerical_quality"]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["basis_ref"]["ref_id"].as_str().unwrap().to_string())
        .collect();
    for case in passing["numerical_quality"]["cases"]
        .as_array_mut()
        .unwrap()
    {
        case["structural_status"] = json!("passive_model_basis");
        case["solve_quality"] = json!("checks_passed");
        case["model_matrix_fidelity"] = json!("represented_equations_retained");
        case["accuracy_evidence"] = json!("not_claimed");
    }
    passing["numerical_quality"]["status"] = json!("checks_passed");
    for d in passing["diagnostics"].as_array_mut().unwrap() {
        if cases
            .iter()
            .any(|c| d["id"] == format!("diagnostic:numerical-integrity:{c}").as_str())
        {
            d["code"] = json!("NUMERICAL_INTEGRITY_CHECKS_PASSED");
            d["severity"] = json!("info");
        }
    }
    passing
}

#[test]
fn every_committed_result_fixture_parses() {
    let (mut envelope_count, mut bound, mut verdict_only, mut preview, mut all) =
        (0, 0, 0, (0, 0), (0, 0));
    for name in RESULT_FIXTURES {
        let value = read(name);
        let mut found = Vec::new();
        envelopes(&value, None, &mut found);
        assert!(!found.is_empty(), "{name}");
        for (envelope, carried) in found {
            envelope_count += 1;
            let out = health(envelope, None);
            assert_eq!(out["verdict_only"], false);
            let (passed, sensitive) = check_cases(envelope, &out, name);
            all = (all.0 + passed, all.1 + sensitive);
            if name.starts_with("product_preview") {
                preview = (preview.0 + passed, preview.1 + sensitive);
            }
            // The reader tie, bound to the fixture's request and model.
            let binding = carried
                .map(|(request, mode)| (request.clone(), mode))
                .or_else(|| {
                    neighbour(name).map(|(request, mode)| {
                        (
                            request,
                            MODES.iter().map(|m| m.1).find(|m| *m == mode).unwrap(),
                        )
                    })
                });
            let Some((request, mode)) = binding else {
                continue;
            };
            bound += 1;
            let reader = dispatch(envelope, &request, mode);
            let tied = health(envelope, Some(&reader));
            if tied["verdict_only"] != true {
                continue;
            }
            verdict_only += 1;
            assert_eq!(findings(&reader), PAIR, "{name}");
            // Held back only by the verdict: passing cases make it eligible.
            let eligible: Value =
                serde_json::from_str(&dispatch(&all_passing(envelope), &request, mode)).unwrap();
            assert_eq!(
                eligible["standing"]["eligible"], true,
                "{name}: {}",
                eligible["standing"]
            );
            // Malformed evidence through the real reader is never verdict-only.
            for index in 0..envelope["numerical_quality"]["cases"]
                .as_array()
                .unwrap()
                .len()
            {
                for refs in [json!([]), json!(["diagnostic:numerical-integrity:renamed"])] {
                    let mut broken = envelope.clone();
                    broken["numerical_quality"]["cases"][index]["evidence_refs"] = refs.clone();
                    let reader = dispatch(&broken, &request, mode);
                    assert_eq!(
                        health(&broken, Some(&reader))["verdict_only"],
                        false,
                        "{name} case {index} {refs}"
                    );
                }
            }
        }
    }
    println!("envelopes {envelope_count}; bound {bound}; verdict_only {verdict_only}; figures (passed, sensitive): product_preview {preview:?}, all {all:?}");
    // The committed corpus as of this change (linear reports only; the four
    // nonlinear passing product_preview cases carry no figure by design).
    assert_eq!(preview, (14, 31));
    assert!(verdict_only > 0 && bound >= verdict_only);
    assert!(envelope_count > 0 && all.0 >= preview.0 && all.1 >= preview.1);
}

/// Every committed fixture under `fixtures/` carrying a result envelope with
/// integrity diagnostics.
const RESULT_FIXTURES: [&str; 112] = [
    "model_operations/pressure_v3_l_line_authoring.json",
    "product_preview/invented_demo_result_preview_physics_1_dense.json",
    "product_preview/invented_demo_result_preview_physics_1_sparse.json",
    "product_preview/invented_mechanics_result_preview_physics_1_dense.json",
    "product_preview/invented_mechanics_result_preview_physics_1_sparse.json",
    "product_preview/pressure/load_reference/connected-dense_scrutiny.raw.json",
    "product_preview/pressure/load_reference/connected-sparse_interactive.raw.json",
    "product_preview/pressure/load_reference/pressure-dense_scrutiny.raw.json",
    "product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json",
    "product_preview/pressure_source/load_reference_source/eigen_motion-dense_scrutiny.raw.json",
    "product_preview/pressure_source/load_reference_source/eigen_motion-sparse_interactive.raw.json",
    "product_preview/pressure_source/load_reference_source/fields-dense_scrutiny.raw.json",
    "product_preview/pressure_source/load_reference_source/fields-sparse_interactive.raw.json",
    "product_preview/pressure_source/load_reference_source/mixed-dense_scrutiny.raw.json",
    "product_preview/pressure_source/load_reference_source/mixed-sparse_interactive.raw.json",
    "product_preview/pressure_source/load_reference_source/n05-dense_scrutiny.raw.json",
    "product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json",
    "product_preview/pressure_source/load_reference_source/n06-dense_scrutiny.raw.json",
    "product_preview/pressure_source/load_reference_source/n06-sparse_interactive.raw.json",
    "product_preview/pressure_source/o4/o4_source_controls.json",
    "product_preview/pressure_source/physics_source/fields-dense_scrutiny.raw.json",
    "product_preview/pressure_source/physics_source/fields-sparse_interactive.raw.json",
    "product_preview/pressure_source/physics_source/mixed-dense_scrutiny.raw.json",
    "product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json",
    "product_preview/pressure_source/physics_source/mixed_units-dense_scrutiny.raw.json",
    "product_preview/pressure_source/physics_source/mixed_units-sparse_interactive.raw.json",
    "product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json",
    "product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json",
    "product_preview/pressure_source/physics_source/n05_unicode-dense_scrutiny.raw.json",
    "product_preview/pressure_source/physics_source/n05_unicode-sparse_interactive.raw.json",
    "product_preview/pressure_source/physics_source/n05_units-dense_scrutiny.raw.json",
    "product_preview/pressure_source/physics_source/n05_units-sparse_interactive.raw.json",
    "product_preview/pressure_source/physics_source/n06-dense_scrutiny.raw.json",
    "product_preview/pressure_source/physics_source/n06-sparse_interactive.raw.json",
    "product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_dense_scrutiny.json",
    "product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_result_sparse_interactive.json",
    "product_preview/w1_controls/ui/multicase-dense_scrutiny.raw.json",
    "product_preview/w1_controls/ui/multicase-sparse_interactive.raw.json",
    "results/pressure_connected_mechanics_dense.json",
    "results/pressure_connected_mechanics_sparse.json",
    "results/pressure_connected_ui_mechanics_dense.json",
    "results/pressure_connected_ui_mechanics_sparse.json",
    "results/pressure_load_state_connected_dense.document.json",
    "results/pressure_load_state_connected_dense.stress_neutral.json",
    "results/pressure_load_state_connected_sparse.document.json",
    "results/pressure_load_state_connected_sparse.stress_neutral.json",
    "results/pressure_load_state_pressure_dense.document.json",
    "results/pressure_load_state_pressure_dense.stress_neutral.json",
    "results/pressure_load_state_pressure_sparse.document.json",
    "results/pressure_load_state_pressure_sparse.stress_neutral.json",
    "results/pressure_plain_connected_dense.document.json",
    "results/pressure_plain_connected_dense.stress_neutral.json",
    "results/pressure_plain_connected_sparse.document.json",
    "results/pressure_plain_connected_sparse.stress_neutral.json",
    "results/pressure_source_load_state_eigen_motion_dense.document.json",
    "results/pressure_source_load_state_eigen_motion_dense.stress_neutral.json",
    "results/pressure_source_load_state_eigen_motion_sparse.document.json",
    "results/pressure_source_load_state_eigen_motion_sparse.stress_neutral.json",
    "results/pressure_source_load_state_fields_dense.document.json",
    "results/pressure_source_load_state_fields_dense.stress_neutral.json",
    "results/pressure_source_load_state_fields_sparse.document.json",
    "results/pressure_source_load_state_fields_sparse.stress_neutral.json",
    "results/pressure_source_load_state_mixed_dense.document.json",
    "results/pressure_source_load_state_mixed_dense.stress_neutral.json",
    "results/pressure_source_load_state_mixed_sparse.document.json",
    "results/pressure_source_load_state_mixed_sparse.stress_neutral.json",
    "results/pressure_source_load_state_n05_dense.document.json",
    "results/pressure_source_load_state_n05_dense.stress_neutral.json",
    "results/pressure_source_load_state_n05_sparse.document.json",
    "results/pressure_source_load_state_n05_sparse.stress_neutral.json",
    "results/pressure_source_load_state_n06_dense.document.json",
    "results/pressure_source_load_state_n06_dense.stress_neutral.json",
    "results/pressure_source_load_state_n06_sparse.document.json",
    "results/pressure_source_load_state_n06_sparse.stress_neutral.json",
    "results/pressure_source_plain_n05_dense.document.json",
    "results/pressure_source_plain_n05_dense.stress_neutral.json",
    "results/pressure_source_plain_n05_sparse.document.json",
    "results/pressure_source_plain_n05_sparse.stress_neutral.json",
    "results/pressure_thermal_ui_mechanics_dense.json",
    "results/pressure_thermal_ui_mechanics_sparse.json",
    "results/pressure_u4b_arc_sparse.json",
    "results/pressure_v3_arc_reader_corpus.json",
    "results/pressure_v3_connector_reader_corpus.json",
    "results/pressure_v3_joint_reader_control.json",
    "results/pressure_v3_straight_reader_corpus.json",
    "results/preview_physics_connected_dense.json",
    "results/preview_physics_connected_sparse.json",
    "results/preview_physics_invented_dense.json",
    "results/preview_physics_invented_sparse.json",
    "results/preview_physics_range_envelope_sparse.json",
    "results/preview_physics_u4b_arc_sparse.json",
    "results/preview_physics_unicode_ids_sparse.json",
    "results/retained_precision_combination_successor_dense_scrutiny.json",
    "results/retained_precision_combination_successor_sparse_interactive.json",
    "results/retained_precision_exact_successor_dense_scrutiny.json",
    "results/retained_precision_exact_successor_derivative_dense_scrutiny.json",
    "results/retained_precision_exact_successor_derivative_sparse_interactive.json",
    "results/retained_precision_exact_successor_sparse_interactive.json",
    "results/retained_precision_failed_ordinary_cxf.json",
    "results/retained_precision_failed_ordinary_n06.json",
    "results/retained_precision_l0_successor_dense_scrutiny.json",
    "results/retained_precision_l0_successor_sparse_interactive.json",
    "results/retained_precision_milestone_successor_dense_scrutiny.json",
    "results/retained_precision_milestone_successor_sparse_interactive.json",
    "results/retained_precision_o4_w1e_mixed_successor_sparse_interactive.json",
    "results/retained_precision_o4_w1e_successor_dense_scrutiny.json",
    "results/retained_precision_o4_w1e_successor_sparse_interactive.json",
    "results/retained_precision_o4_w1e_unavailable_successor_sparse_interactive.json",
    "results/retained_precision_successor_derivative_dense_scrutiny.json",
    "results/retained_precision_successor_derivative_sparse_interactive.json",
    "results/retained_precision_w_c2_successor_dense_scrutiny.json",
    "results/retained_precision_w_c2_successor_sparse_interactive.json",
];
