//! DEL-04-07: the cap remainder warning stands exactly where a case's
//! published evidence carries the cancellation-screen exception record. A
//! case admitted under the exception and then refused (here by the numerical
//! integrity check) publishes neither, and its blocked envelope passes the
//! readers' warning rule; an admitted and published case still carries it.
//! `pressure_runtime::exception_probe` observes the admission itself, which a
//! blocked envelope no longer shows.
use super::*;
use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const WARNING: &str = pressure_runtime::CAP_REMAINDER_WARNING_CODE;

fn up(value: f64, ulps: u64) -> f64 {
    f64::from_bits(value.to_bits() + ulps)
}

/// One load case `case` with one closed straight region over the chain
/// `points` (pipe k from point k to k+1, outside diameter `outside[k]`, wall
/// 0.01 m), transferring at both terminals, elongation not included; nodes and
/// pipes are named with `prefix`, and the chain is anchored at its two ends.
fn chain(prefix: &str, case: &str, points: &[[f64; 3]], outside: &[f64]) -> (Vec<Value>, Vec<Value>, Vec<Value>, Value) {
    let node = |k: usize| format!("node:{prefix}{k}");
    let nodes = points.iter().enumerate()
        .map(|(k, p)| json!({"id":node(k),"position":{"x":p[0],"y":p[1],"z":p[2]},"provenance":"synthetic"}))
        .collect();
    let pipes = outside.iter().enumerate().map(|(k, od)| json!({"id":format!("pipe:{prefix}{k}"),"from":node(k),"to":node(k + 1),
        "section":{"outside_diameter":{"value":od,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"},"corrosion_allowance":{"value":0.0,"unit":"m"}},
        "material":"material:one","y_reference":{"x":0.0,"y":0.0,"z":1.0},"provenance":"synthetic"})).collect::<Vec<_>>();
    let last = outside.len();
    let supports = [0, last].iter().map(|k| json!({"id":format!("support:{prefix}{k}"),"node":node(*k),"family":"anchor",
        "restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"synthetic"})).collect();
    let terminal = |k: usize| json!({"node_ref":node(k),"closure_transfer":"transfers_to_wall","provenance":"synthetic"});
    let load_case = json!({"id":case,"primitive_loads":[],"pressure_regions":[{"id":format!("region:{prefix}"),
        "member_pipe_ids":pipes.iter().map(|p| p["id"].clone()).collect::<Vec<_>>(),
        "pressure_basis":"internal_differential_zero_external_v1","pressure":{"value":2e6,"unit":"Pa"},
        "terminals":[terminal(0),terminal(last)],"provenance":"synthetic"}],"provenance":"synthetic"});
    (nodes, pipes, supports, load_case)
}

fn document(chains: &[(Vec<Value>, Vec<Value>, Vec<Value>, Value)]) -> Value {
    let mut nodes = Vec::new();
    let mut pipes = Vec::new();
    let mut supports = Vec::new();
    let mut cases = Vec::new();
    for (n, p, s, c) in chains {
        nodes.extend(n.iter().cloned());
        pipes.extend(p.iter().cloned());
        supports.extend(s.iter().cloned());
        cases.push(c.clone());
    }
    json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"},
        "project":{"id":"project:cap-remainder-publication","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":nodes,"pipe_segments":pipes,"supports":supports,"components":[],
        "materials":[{"id":"material:one","constitutive_basis":"homogeneous_isotropic_E_nu_v1","elastic_modulus":{"value":200e9,"unit":"Pa"},
            "poisson_ratio":{"value":0.3,"unit":"1"},"thermal_expansion_coefficient":{"value":0.000012,"unit":"1/degC"},
            "temperature_points":[],"provenance":"synthetic"}],
        "load_cases":cases,"combinations":[]},"materials":[]})
}

/// Admitted and published: a 6 m run, outside diameters one ulp apart.
fn admitted(prefix: &str, case: &str, y: f64) -> (Vec<Value>, Vec<Value>, Vec<Value>, Value) {
    chain(prefix, case, &[[0.0, y, 0.0], [3.0, y, 0.0], [6.0, y, 0.0]], &[0.12, up(0.12, 1)])
}

/// Admitted, then refused by the integrity check: a 1 m run along x with a
/// member of 1e-8 m stepping off the axis by 63ε of the span, back to the
/// axis through a node off it by 63ε in z (the adverse set's `u-1-1e-08`
/// anchored model; NUMERICAL_INTEGRITY_UNRESOLVED in the sparse mode).
fn refused(prefix: &str, case: &str, y: f64) -> (Vec<Value>, Vec<Value>, Vec<Value>, Value) {
    let off = 63.0 * f64::EPSILON;
    chain(prefix, case, &[[0.0, y, 0.0], [0.5, y, 0.0], [0.5 + 1e-8, y + off, 0.0], [0.75, y, off], [1.0, y, 0.0]],
        &[0.12, up(0.12, 1), up(0.12, 2), 0.12])
}

fn solve(input: &Value, mode: PreviewSolverMode) -> (Value, usize) {
    pressure_runtime::exception_probe::take_admitted();
    let request: LinearStaticPreviewRequest = serde_json::from_value(input.clone()).unwrap();
    let output = serde_json::to_value(run_linear_static_preview_with_mode(request, mode)).unwrap();
    (output, pressure_runtime::exception_probe::take_admitted())
}

fn warnings(output: &Value) -> Vec<Value> {
    output["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == WARNING).cloned().collect()
}

fn blocking(output: &Value) -> Vec<String> {
    output["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking")
        .map(|d| d["code"].as_str().unwrap().to_owned()).collect()
}

fn records(output: &Value) -> usize {
    output["contract_evidence"]["exact_cases"].as_array().map_or(0, |cases| {
        cases.iter().filter(|c| c["pressure_rhs_assembly"].get("cancellation_screen_exception").is_some()).count()
    })
}

/// The refused case's blocked envelope, with the warning the producer would
/// have left beside it before this rule, is the double refusal the readers made.
fn with_stray_warning(output: &Value, case: &str) -> Value {
    let mut stray = output.clone();
    stray["diagnostics"].as_array_mut().unwrap().push(json!({"id":"diagnostic:stray","code":WARNING,"severity":"warning",
        "message":pressure_runtime::cap_remainder_warning_message(case, "node:R1", 0.0),"affected_refs":[case,"region:R","node:R1"]}));
    stray
}

#[test]
fn a_case_the_integrity_check_refuses_after_admission_publishes_no_warning() {
    let input = document(&[refused("R", "case:refused", 0.0)]);
    let mut refusals = 0;
    for mode in MODES {
        let (output, admitted) = solve(&input, mode);
        let codes = blocking(&output);
        if output["status"]["mechanics"] == "MODEL_INCOMPLETE" {
            assert!(codes.iter().any(|code| code == "NUMERICAL_INTEGRITY_UNRESOLVED"), "{mode:?}: {codes:?}");
            assert!(admitted >= 1, "{mode:?}: the case was admitted under the exception before it was refused");
            assert_eq!(records(&output), 0, "{mode:?}");
            assert!(warnings(&output).is_empty(), "{mode:?}: {:?}", warnings(&output));
            validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
            assert_eq!(validate_pressure_evidence(&with_stray_warning(&output, "case:refused")).unwrap_err(),
                "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING_UNRECORDED", "{mode:?}");
            refusals += 1;
        } else {
            // Published: the record and its one warning stand together.
            assert_eq!((records(&output), warnings(&output).len()), (1, 1), "{mode:?}");
            validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        }
    }
    assert!(refusals >= 1, "the sparse mode refuses this model after admission");
}

#[test]
fn an_admitted_and_published_case_still_carries_its_warning() {
    let input = document(&[admitted("A", "case:admitted", 0.0)]);
    for mode in MODES {
        let (output, admitted) = solve(&input, mode);
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", output["diagnostics"]);
        assert!(admitted >= 1, "{mode:?}");
        assert_eq!(records(&output), 1, "{mode:?}");
        let found = warnings(&output);
        assert_eq!(found.len(), 1, "{mode:?}");
        assert_eq!(found[0]["affected_refs"], json!(["case:admitted", "region:A", "node:A1"]), "{mode:?}");
        validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
    }
}

/// Two cases, both admitted under the exception: the first is solved, the
/// second carries a 1e308 N force and is refused by the integrity check
/// (force scaling does not admit an exact pressure operand). A refused case
/// blocks its document, which publishes no case evidence, so neither case's
/// record nor warning is published, the first case's included, and the
/// envelope passes the readers' warning rule. (A document cannot publish one
/// case and refuse another: every refusal of a case is blocking.)
#[test]
fn a_document_whose_second_admitted_case_is_refused_publishes_no_warning() {
    let mut loaded = admitted("B", "case:loaded", 10.0);
    loaded.3["primitive_loads"] = json!([{"id":"load:B","category":"concentrated_force","target":{"type":"node","node":"node:B1"},
        "direction":"global_y","magnitude":{"value":1e308,"unit":"N"},"dimension":"force","provenance":"synthetic"}]);
    let input = document(&[admitted("A", "case:admitted", 5.0), loaded]);
    for mode in MODES {
        let (output, admitted) = solve(&input, mode);
        assert_eq!(output["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        let refusals = output["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking")
            .map(|d| (d["code"].as_str().unwrap().to_owned(), d["affected_refs"].clone())).collect::<Vec<_>>();
        assert_eq!(refusals, [("NUMERICAL_INTEGRITY_UNRESOLVED".to_owned(), json!(["case:loaded"]))], "{mode:?}");
        assert_eq!(admitted, 2, "{mode:?}: both cases admitted under the exception");
        assert_eq!(records(&output), 0, "{mode:?}");
        assert!(warnings(&output).is_empty(), "{mode:?}: {:?}", warnings(&output));
        validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
    }
}

/// The exception's work (eligibility, family walk, record and warning) is
/// entered only when the original screen fails: a closed straight run of
/// bit-equal members, whose groups cancel exactly and whose screen passes,
/// never enters it, while the same run with members one ulp apart, whose
/// screen fails, enters it and is admitted.
#[test]
fn a_case_the_original_screen_admits_never_enters_the_exception() {
    let equal = document(&[chain("E", "case:equal", &[[0.0; 3], [3.0, 0.0, 0.0], [6.0, 0.0, 0.0]], &[0.12, 0.12])]);
    let apart = document(&[admitted("A", "case:admitted", 0.0)]);
    for mode in MODES {
        pressure_runtime::exception_probe::take_entered();
        let (output, admitted) = solve(&equal, mode);
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        let screen = output["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["cancellation_screen"].as_f64().unwrap();
        assert!(screen <= 1e-9, "{mode:?}: the original screen passes ({screen})");
        assert_eq!((pressure_runtime::exception_probe::take_entered(), admitted), (0, 0), "{mode:?}");
        assert!(warnings(&output).is_empty() && records(&output) == 0, "{mode:?}");
        let (_, admitted) = solve(&apart, mode);
        assert!(pressure_runtime::exception_probe::take_entered() >= 1 && admitted >= 1, "{mode:?}");
    }
}
