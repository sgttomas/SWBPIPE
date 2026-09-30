//! F1a tests (T3 D1 revision 5a.2 §4.3.1 D5C-3, §5 item 5a; the F1a split of
//! slice F1): K-D5's `FormationCheck` record is rendered as one evidence line
//! of the integrity diagnostic, only when present, after the S11-G step and
//! under S11-G's layout and no-op rule. SUP-17's text is pinned by
//! `tests::under_restrained_model_reports_solver_diagnostic` in `lib.rs`.
//!
//! Every product pin first asserts that the paths differ (the case is
//! K-D5-demoted, or S11-G's load-row guard fires, computed in the test), and
//! only then asserts the rendering. Inputs are invented; no material,
//! component, catalogue or code-rule data is used. P1's detection requests are
//! read byte for byte from `tests/formation_check_runtime.rs`.
use super::*;
use open_pipe_stress_frame_kernel::structural::FORMATION_CRITERION;
use serde_json::{json, Value};

const INVENTED: &str = "invented_t3_f1a_test_input_not_library_data";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
/// The evidence line's prefix.
const D5: &str = "formation_check: ";
const LOAD_ROW: &str = "S11-G formation-noise guard";
const KD5_TESTS: &str = include_str!("../tests/formation_check_runtime.rs");

// ------------------------------------------------------------------ entries

#[derive(Clone, Copy, Debug)]
enum Entry {
    /// `run_linear_static_preview_value_with_mode` (the captured entry).
    Captured,
    /// `run_linear_static_preview_with_mode` (the historical typed entry).
    Typed,
}
const ENTRIES: [Entry; 2] = [Entry::Captured, Entry::Typed];

fn solved(entry: Entry, request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let envelope = match entry {
        Entry::Captured => run_linear_static_preview_value_with_mode(request.clone(), mode)
            .unwrap_or_else(|e| panic!("{entry:?} {mode:?}: {e}")),
        Entry::Typed => {
            let typed: LinearStaticPreviewRequest =
                serde_json::from_value(request.clone()).unwrap();
            run_linear_static_preview_with_mode(typed, mode)
        }
    };
    let blocked: Vec<_> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.severity == "blocking")
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect();
    assert!(blocked.is_empty(), "{entry:?} {mode:?} blocked: {blocked:?}");
    envelope
}

fn integrity<'a>(envelope: &'a MechanicsEnvelope, case: &str) -> &'a Diagnostic {
    let found: Vec<_> = envelope
        .diagnostics
        .iter()
        .filter(|d| d.id == integrity_diagnostic_id(case))
        .collect();
    assert_eq!(found.len(), 1, "one integrity diagnostic of {case}");
    found[0]
}

/// A P1 detection request, byte for byte as K-D5's product tests hold it.
fn p1_request(name: &str) -> Value {
    let head = format!("const {name}: &str = r#\"");
    let start = KD5_TESTS
        .find(&head)
        .unwrap_or_else(|| panic!("{name} in formation_check_runtime.rs"))
        + head.len();
    let end = start + KD5_TESTS[start..].find("\"#;").unwrap();
    serde_json::from_str(&KD5_TESTS[start..end]).unwrap()
}

// ------------------------------------------------------- the evidence line

/// The D-5 evidence line's fields, parsed from a message that carries
/// exactly one line, which is the message's suffix.
struct D5Line {
    reason: String,
    row: String,
    doubled_correction: f64,
    scale: f64,
    trigger_ratio: f64,
}

fn d5_line(message: &str, label: &str) -> D5Line {
    assert_eq!(message.matches(D5).count(), 1, "{label}: {message}");
    let line = &message[message.find(D5).unwrap()..];
    assert!(
        message.ends_with(line) && message[..message.len() - line.len()].ends_with(' '),
        "{label}: appended with one space as the last element"
    );
    let fields: HashMap<&str, &str> = line[D5.len()..]
        .split("; ")
        .map(|kv| kv.split_once('=').unwrap_or_else(|| panic!("{label}: {kv}")))
        .collect();
    assert_eq!(
        line[D5.len()..]
            .split("; ")
            .map(|kv| kv.split_once('=').unwrap().0)
            .collect::<Vec<_>>(),
        vec!["reason", "row", "doubled_correction", "scale", "trigger_ratio"],
        "{label}: {line}"
    );
    let number = |key: &str| -> f64 {
        fields[key]
            .parse()
            .unwrap_or_else(|_| panic!("{label}: {key}={}", fields[key]))
    };
    D5Line {
        reason: fields["reason"].to_string(),
        row: fields["row"].to_string(),
        doubled_correction: number("doubled_correction"),
        scale: number("scale"),
        trigger_ratio: number("trigger_ratio"),
    }
}

// ============================================================== product level

/// A K-D5-demoted case (P1's RF-SKEW-T-CANT-OFF-122-r1e-04, the design's
/// required true positive) carries exactly one D-5 evidence line on both
/// entries in both modes. Precondition: the ordinary report is demoted by
/// K-D5 (its text says `quality: Sensitive` and no S11-G sentence exists).
/// The printed values are the record's: the trigger value equals
/// 2|w| / (1e-9 * scale) bit for bit, above 1.
#[test]
fn f1a_kd5_demoted_case_carries_exactly_one_d5_line() {
    let request = p1_request("RF_SKEW_T_CANT_OFF_122_R1E_04");
    for entry in ENTRIES {
        for mode in MODES {
            let label = format!("122 {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            let d = integrity(&envelope, "case");
            assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{label}");
            assert_eq!(d.severity, "warning", "{label}");
            assert!(d.message.contains("quality: Sensitive"), "{label}: K-D5 demoted");
            assert!(!d.message.contains("S11-G"), "{label}");
            let line = d5_line(&d.message, &label);
            assert_eq!(line.reason, "estimate", "{label}");
            assert!(
                ["N0:RX", "N0:RY", "N0:RZ"]
                    .iter()
                    .chain(["N1:UX", "N1:UY", "N1:UZ", "N1:RX", "N1:RY", "N1:RZ"].iter())
                    .any(|r| *r == line.row),
                "{label}: a free row, labelled: {}",
                line.row
            );
            assert!(line.scale > 0.0 && line.scale.is_finite(), "{label}");
            assert!(line.doubled_correction > 0.0, "{label}");
            assert!(line.trigger_ratio > 1.0 && line.trigger_ratio.is_finite(), "{label}");
            assert_eq!(
                line.trigger_ratio.to_bits(),
                (line.doubled_correction / (FORMATION_CRITERION * line.scale)).to_bits(),
                "{label}: the printed values are the record's"
            );
            eprintln!("f1a 122 {entry:?} {mode:?}: {}", &d.message[d.message.find(D5).unwrap()..]);
        }
    }
}

/// Non-demoted cases carry no D-5 line: P1's 345 and RF-CHAIN r1e-04
/// continuity controls (K-D5 Passed), and an S11-G-demoted case K-D5 leaves
/// Passed (RF-CANCEL-UDL-W1e8), whose record keeps S11-G's sentence alone.
#[test]
fn f1a_non_demoted_cases_carry_no_d5_line() {
    for name in [
        "RF_SKEW_T_CANT_OFF_345_R1E_04",
        "RF_CHAIN_T_N03_R1E_04",
        "RF_CHAIN_A_N03_R1E_04",
    ] {
        let request = p1_request(name);
        for entry in ENTRIES {
            for mode in MODES {
                let label = format!("{name} {entry:?} {mode:?}");
                let d = integrity(&solved(entry, &request, mode), "case").clone();
                assert_eq!(d.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{label}");
                assert!(!d.message.contains("formation_check"), "{label}");
            }
        }
    }
    let data: Value =
        serde_json::from_str(include_str!("../tests/fixtures/s11f/rf_cancel_cases.json")).unwrap();
    let request = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == json!("RF-CANCEL-UDL-W1e8"))
        .unwrap()["request"]
        .clone();
    for entry in ENTRIES {
        for mode in MODES {
            let label = format!("UDL-W1e8 {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            let d = integrity(&envelope, "case");
            // Precondition: S11-G demoted a report K-D5 left Passed.
            assert!(d.message.contains("quality: Passed"), "{label}");
            assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{label}");
            assert_eq!(d.message.matches(LOAD_ROW).count(), 1, "{label}");
            assert!(!d.message.contains("formation_check"), "{label}");
        }
    }
}

/// The inputs the product's guard reads for case 0 (as `solve_load_case`
/// forms them, pre-0.4, no exact pressure): the assembled force, and the
/// load-row finding.
fn guard_inputs(request: &Value) -> (AssembledForce, Option<formation_guard::FormationFinding>) {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).expect("model builds");
    let load_case = &model.load_cases[0];
    let loads = build_load_case_primitive_loads(&model, load_case, &mut diagnostics);
    let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
    let bends = built
        .curved_bend_elements
        .iter()
        .map(|e| (e.pipe_index, e))
        .collect::<HashMap<_, _>>();
    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let thermal = build_thermal_element_loads(
        &model,
        load_case,
        &material_map,
        &pipe_map,
        &built.sections,
        &mut diagnostics,
    );
    let thrust = build_pressure_thrust_loads(&model, load_case, &pipe_map, &built.sections);
    let ledger = case_force_ledger(
        &model,
        &built,
        &application,
        &bends,
        &thrust,
        &thermal,
        None,
        &load_case.id,
        &mut diagnostics,
    );
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    let force = ledger.finish(built.nodes.len() * DOF_PER_NODE).unwrap();
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let finding = formation_guard::load_row_finding(
        &force,
        &formation_bodies(&built),
        &boundary.restrained_dofs,
        |dof| integrity_dof_label(&model, dof),
        &load_case.id,
    );
    (force, finding)
}

const DOF_NAMES: [&str; 6] = ["global_x", "global_y", "global_z", "RX", "RY", "RZ"];

fn nodal(id: &str, node: &str, dof: usize, value: f64) -> Value {
    let moment = dof >= 3;
    json!({"id": id, "category": if moment { "concentrated_moment" } else { "concentrated_force" },
        "target": {"type": "node", "node": node}, "direction": DOF_NAMES[dof],
        "magnitude": {"value": value, "unit": if moment { "N*m" } else { "N" }},
        "dimension": if moment { "moment" } else { "force" }, "provenance": INVENTED})
}

/// 122 plus formation noise on its load rows: an invented uniform load of
/// W = 1e8 N/m in global y on M1, cancelled at every free row (N0 RX, RY, RZ
/// and all of N1) by authored nodal loads equal to minus the product's own
/// rounded per-DOF resultant of that uniform load alone. The rows then keep
/// P1's moments plus sub-ulp remainders, while the formed fixed-end terms
/// (about 1e8 N and 7.5e7 N*m) carry formation noise far above 1e-9 of the
/// rows' nets.
fn skew_122_with_load_row_noise() -> Value {
    let base = p1_request("RF_SKEW_T_CANT_OFF_122_R1E_04");
    let udl = json!({"id": "load:f1a:w", "category": "distributed_force",
        "target": {"type": "element", "pipe": "M1"}, "direction": "global_y",
        "magnitude": {"value": 1e8, "unit": "N/m"}, "dimension": "force_per_length",
        "provenance": INVENTED});
    let mut udl_only = base.clone();
    udl_only["model"]["load_cases"][0]["primitive_loads"] = json!([udl.clone()]);
    let (force, _) = guard_inputs(&udl_only);
    let mut request = base;
    let loads = request["model"]["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap();
    loads.push(udl);
    for dof in (3..6).chain(6..12) {
        let value = force.values()[dof];
        assert!(value.is_finite());
        if value != 0.0 {
            let node = if dof < 6 { "N0" } else { "N1" };
            loads.push(nodal(&format!("load:f1a:cancel:{dof}"), node, dof % 6, -value));
        }
    }
    request
}

/// A case demoted by both K-D5 and S11-G's load-row guard: the record is
/// SENSITIVE with no S11-G sentence, and exactly one D-5 line after the
/// report text. Preconditions, computed here: the guard fires on this case's
/// own rows, and K-D5 demotes it (`quality: Sensitive`).
///
/// Why the sentence is suppressed (S11-G's no-op rule; lines at this F1a
/// candidate):
/// - the kernel sets `quality: Sensitive` whenever it attaches a record
///   (`FK/structural.rs:1480-1490`: `quality` is Sensitive if
///   `ordinary_sensitive || formation_check.is_some()`; this is the only
///   constructor of `StructuralSolution.formation_check`);
/// - `append_integrity_report` takes the code from `report.quality`
///   (`PP:1089`), so the record starts as NUMERICAL_INTEGRITY_SENSITIVE;
/// - the load-row step (`PP:1098-1100`) calls `formation_guard::demote`,
///   which returns at once unless the code is CHECKS_PASSED
///   (`formation_guard.rs:481-484`), so no sentence is added; the D-5 line
///   is then appended (`PP:1101-1107`).
///
/// The reverse composition (a sentence and a line on one record) cannot
/// arise: a record implies a Sensitive report, so the record is never
/// CHECKS_PASSED, and R-b' after the recovery loop (`PP:3853`,
/// `amend_integrity_report`) uses the same `demote` and is a no-op too
/// (pinned at unit level in `f1a_composition_with_s11g_layout_and_no_op_rule`).
#[test]
fn f1a_case_demoted_by_both_kd5_and_s11g_has_the_d5_line_alone() {
    let request = skew_122_with_load_row_noise();
    let (_, finding) = guard_inputs(&request);
    let finding = finding.expect("precondition: S11-G's load-row guard fires");
    assert!(finding.sentence.starts_with(LOAD_ROW));
    for entry in ENTRIES {
        for mode in MODES {
            let label = format!("122+noise {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            let d = integrity(&envelope, "case");
            assert!(
                d.message.contains("quality: Sensitive"),
                "{label}: precondition, K-D5 demoted: {}",
                d.message
            );
            assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{label}");
            assert_eq!(d.severity, "warning", "{label}");
            assert!(!d.message.contains("S11-G"), "{label}: no-op rule");
            let line = d5_line(&d.message, &label);
            assert_eq!(line.reason, "estimate", "{label}");
            assert!(line.trigger_ratio > 1.0, "{label}");
        }
    }
}

// ================================================================= unit level

fn unit_model() -> PreviewModel {
    serde_json::from_value::<LinearStaticPreviewRequest>(p1_request(
        "RF_SKEW_T_CANT_OFF_122_R1E_04",
    ))
    .unwrap()
    .model
}

fn unit_report(quality: SolveQuality) -> StructuralReport {
    StructuralReport {
        policy: "M03-INTEGRITY-v1",
        quality,
        factorization: "unit",
        scale_exponents: vec![],
        pivots: vec![],
        condition_estimator: "unit",
        reciprocal_condition_estimate: 0.5,
        residual_rows: vec![],
        refinement_attempts: 0,
        contribution_audit_performed: true,
        contribution_rounding: vec![],
        assembly_relative_perturbation_estimate: 0.0,
        assembly_amplification_estimate: 0.0,
        assembly_load_perturbation_estimate: 0.0,
        intended_residual_rows: vec![],
        symmetry_projection_performed: false,
        maximum_scaled_skew: 0.0,
        symmetry_basis: None,
    }
}

fn unit_record(
    quality: SolveQuality,
    finding: Option<&formation_guard::FormationFinding>,
    check: Option<&FormationCheck>,
) -> Diagnostic {
    let mut diagnostics = Vec::new();
    append_integrity_report(
        &mut diagnostics,
        "c",
        &unit_report(quality),
        &unit_model(),
        None,
        finding,
        check,
        None,
    );
    assert_eq!(diagnostics.len(), 1);
    diagnostics.pop().unwrap()
}

fn estimate(global_dof: usize, doubled: f64, scale: f64) -> FormationCheck {
    FormationCheck {
        reason: FormationCheckReason::Estimate,
        global_dof: Some(global_dof),
        doubled_correction: doubled,
        scale,
        ratio: doubled / (FORMATION_CRITERION * scale),
    }
}

/// The exact rendering of both reasons, including the zero-scale clause's
/// infinite trigger value and a row outside the model's labels.
#[test]
fn f1a_evidence_line_exact_text() {
    let model = unit_model();
    let check = estimate(9, 2.5e-9, 0.5);
    assert_eq!(
        formation_check_evidence_line(&model, &check),
        format!(
            "formation_check: reason=estimate; row=N1:RX; doubled_correction=2.5e-9; scale=0.5; trigger_ratio={:?}",
            2.5e-9 / (1e-9 * 0.5)
        )
    );
    let zero_scale = FormationCheck {
        reason: FormationCheckReason::Estimate,
        global_dof: Some(4),
        doubled_correction: 1e-300,
        scale: 0.0,
        ratio: f64::INFINITY,
    };
    assert_eq!(
        formation_check_evidence_line(&model, &zero_scale),
        "formation_check: reason=estimate; row=N0:RY; doubled_correction=1e-300; scale=0.0; trigger_ratio=inf"
    );
    let outside = estimate(12, 4e-9, 1.0);
    assert!(formation_check_evidence_line(&model, &outside).contains("; row=global_dof=12; "));
    let unavailable = FormationCheck {
        reason: FormationCheckReason::FormationCheckUnavailable {
            detail: "curved slot 0 has no matching macro element".into(),
        },
        global_dof: None,
        doubled_correction: f64::NAN,
        scale: f64::NAN,
        ratio: f64::NAN,
    };
    assert_eq!(
        formation_check_evidence_line(&model, &unavailable),
        "formation_check: reason=formation_check_unavailable; detail=curved slot 0 has no matching macro element"
    );
}

/// The composition matrix at `append_integrity_report`:
/// - no record: the diagnostic is exactly today's (the byte-identity half);
/// - a record: today's message, one space, the line; nothing else changes;
/// - K-D5 and S11-G together (a Sensitive report and a finding): the no-op
///   rule drops the sentence and the line stands alone;
/// - a Passed report with a finding and a record (not reachable in the
///   product, where a record implies Sensitive): the line follows the
///   S11-G sentence, so the ordering is S11-G's step first.
#[test]
fn f1a_composition_with_s11g_layout_and_no_op_rule() {
    let finding = formation_guard::FormationFinding {
        guard: formation_guard::Guard::LoadRow,
        sentence: "S11-G formation-noise guard: y".into(),
        fired: vec![],
    };
    let check = estimate(7, 3e-9, 1.0);
    let line = formation_check_evidence_line(&unit_model(), &check);
    let json = |d: &Diagnostic| serde_json::to_string(d).unwrap();

    let sensitive = unit_record(SolveQuality::Sensitive, None, None);
    let passed = unit_record(SolveQuality::Passed, None, None);
    assert_eq!(sensitive.code, "NUMERICAL_INTEGRITY_SENSITIVE");
    assert_eq!(passed.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED");
    assert!(!sensitive.message.contains(D5) && !passed.message.contains(D5));

    // A record appends exactly " <line>"; id, code, severity and refs stand.
    let with = unit_record(SolveQuality::Sensitive, None, Some(&check));
    let mut expected = sensitive.clone();
    expected.message = format!("{} {line}", sensitive.message);
    assert_eq!(json(&with), json(&expected));

    // Both demotions: the no-op rule leaves the Sensitive record without a
    // sentence; the D-5 line alone follows the report text.
    let both = unit_record(SolveQuality::Sensitive, Some(&finding), Some(&check));
    assert_eq!(json(&both), json(&expected));
    assert!(!both.message.contains("S11-G"));
    // Control: the same finding demotes a Passed record (it is live).
    let s11g = unit_record(SolveQuality::Passed, Some(&finding), None);
    assert_eq!(s11g.code, "NUMERICAL_INTEGRITY_SENSITIVE");
    assert_eq!(s11g.message, format!("{} {}", passed.message, finding.sentence));

    // R-b' after the recovery loop leaves a record carrying a line untouched
    // (the no-op rule), so no sentence can follow the line.
    let recovery = formation_guard::FormationFinding {
        guard: formation_guard::Guard::Recovery,
        sentence: "S11-G recovery guard (R-b'): x".into(),
        fired: vec![],
    };
    let mut after_loop = vec![both.clone()];
    assert!(!formation_guard::amend_integrity_report(
        &mut after_loop,
        &integrity_diagnostic_id("c"),
        &recovery
    ));
    assert_eq!(json(&after_loop[0]), json(&expected));

    // Ordering (not reachable in the product, where a record implies a
    // Sensitive report): S11-G's sentence, then the line.
    let ordered = unit_record(SolveQuality::Passed, Some(&finding), Some(&check));
    assert_eq!(
        ordered.message,
        format!("{} {} {line}", passed.message, finding.sentence)
    );
    assert_eq!(ordered.code, "NUMERICAL_INTEGRITY_SENSITIVE");
}

/// ROOT's condition (a): each `reason=` token is K-D5's own identifier. K-D5
/// has no string form for `FormationCheckReason`, so the token is the
/// snake_case of the variant name (the names its doc comments and D1 §4.3.1
/// give: `estimate`, `formation_check_unavailable`). The variant name is
/// read from the enum's Debug form, so a rename breaks this test, and the
/// exhaustive match breaks the build when a variant is added.
#[test]
fn f1a_reason_tokens_are_the_enum_variants() {
    fn snake(debug: &str) -> String {
        let name = debug
            .split(|c: char| !c.is_ascii_alphanumeric())
            .next()
            .unwrap();
        let mut out = String::new();
        for (i, c) in name.chars().enumerate() {
            if c.is_ascii_uppercase() {
                if i > 0 {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
            } else {
                out.push(c);
            }
        }
        out
    }
    let model = unit_model();
    let reasons = [
        FormationCheckReason::Estimate,
        FormationCheckReason::FormationCheckUnavailable { detail: "d".into() },
    ];
    for reason in reasons {
        let expected = match &reason {
            FormationCheckReason::Estimate => "estimate",
            FormationCheckReason::FormationCheckUnavailable { .. } => "formation_check_unavailable",
        };
        assert_eq!(snake(&format!("{reason:?}")), expected);
        let check = FormationCheck {
            reason: reason.clone(),
            global_dof: Some(6),
            doubled_correction: 2e-9,
            scale: 1.0,
            ratio: 2.0,
        };
        let line = formation_check_evidence_line(&model, &check);
        assert!(
            line.starts_with(&format!("{D5}reason={expected}; ")),
            "{line}"
        );
    }
}

/// Main's `append_integrity_report` (`5ae22926e`, `PP:1063-1090`), copied
/// verbatim as the byte-identity oracle for ROOT's condition (c).
fn main_append_integrity_report(
    diagnostics: &mut Vec<Diagnostic>,
    case_id: &str,
    report: &StructuralReport,
    model: &PreviewModel,
    equilibrium: Option<
        &open_pipe_stress_nonlinear_integration::product_equilibrium::ProductEquilibriumReport,
    >,
    formation: Option<&formation_guard::FormationFinding>,
) {
    let code = if report.quality == SolveQuality::Sensitive {
        "NUMERICAL_INTEGRITY_SENSITIVE"
    } else {
        "NUMERICAL_INTEGRITY_CHECKS_PASSED"
    };
    diagnostics.push(diag(&integrity_diagnostic_id(case_id), code, if report.quality == SolveQuality::Sensitive { "warning" } else { "info" },
        format!("{} represented original-equation structural evidence for load case {}: {:?}; global_dof_map={:?}. {} No certified inertia, guaranteed forward accuracy, or pressure/component/stress engineering qualification is claimed.", report.policy, case_id, report, integrity_dof_map(model),
            equilibrium.map(|e|format!("{} final same-state evaluated equilibrium and derived residual-work evidence: {:?}; residual units are N for global DOF%6<3 and N*m otherwise; work units N*m; observed maximum only, exact represented maximum not claimed; general-energy historical alias is residual work, not total energy balance; separate zero count/cap/contact/sliding checks passed",e.policy,e)).unwrap_or_else(||"The contribution audit distinguishes intended assembly from stored equations; physical formulation limitations remain applicable.".into())),
        vec![case_id.to_string()]));
    if let (Some(finding), Some(record)) = (formation, diagnostics.last_mut()) {
        formation_guard::demote(record, finding);
    }
}

/// ROOT's condition (c) at the composing function: with no `FormationCheck`,
/// the integrity diagnostic is byte-identical to main's for every report
/// quality, with and without an S11-G finding of either guard, and appended
/// after existing diagnostics. (Product level: the committed-raw tests and
/// the committed-fixture diff against main.)
#[test]
fn f1a_no_record_is_byte_identical_to_main() {
    let model = unit_model();
    let findings = [
        None,
        Some(formation_guard::FormationFinding {
            guard: formation_guard::Guard::LoadRow,
            sentence: "S11-G formation-noise guard: load case c is Sensitive".into(),
            fired: vec![],
        }),
        Some(formation_guard::FormationFinding {
            guard: formation_guard::Guard::Recovery,
            sentence: "S11-G recovery guard (R-b'): load case c is Sensitive".into(),
            fired: vec![],
        }),
    ];
    let existing = diag("diagnostic:x", "X", "info", "x", vec![]);
    for quality in [SolveQuality::Passed, SolveQuality::Sensitive] {
        for finding in &findings {
            let report = unit_report(quality);
            let mut ours = vec![existing.clone()];
            let mut mains = vec![existing.clone()];
            append_integrity_report(&mut ours, "c", &report, &model, None, finding.as_ref(), None, None);
            main_append_integrity_report(&mut mains, "c", &report, &model, None, finding.as_ref());
            assert_eq!(
                serde_json::to_string(&ours).unwrap(),
                serde_json::to_string(&mains).unwrap(),
                "{quality:?} {finding:?}"
            );
            // With a record, main's bytes plus exactly " <line>".
            let check = estimate(3, 4e-9, 1.0);
            let mut with = vec![existing.clone()];
            append_integrity_report(&mut with, "c", &report, &model, None, finding.as_ref(), Some(&check), None);
            mains[1].message = format!(
                "{} {}",
                mains[1].message,
                formation_check_evidence_line(&model, &check)
            );
            assert_eq!(
                serde_json::to_string(&with).unwrap(),
                serde_json::to_string(&mains).unwrap()
            );
        }
    }
}
