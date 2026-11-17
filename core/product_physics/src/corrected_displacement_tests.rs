//! T4-U5b item 6 (HELP_HUMAN ruling (A), 2026-10-18): nodal displacements
//! publish the formation check's corrected displacement fl(u + w + w2)
//! exactly when the load case's certified member bound exists, and the
//! solve's own u otherwise. Both paths on pressure-2 and preview-physics-2
//! (the fallback forced by the test-only `U4A_FORCE_BINARY64` hook, which
//! removes the bound as a refused or unavailable bound does); in every run
//! each displacement magnitude is the correctly rounded norm of the published
//! components (the base G7 magnitude guard).
use super::*;
use serde_json::Value;

#[path = "../tests/common/current_model.rs"]
mod current_model;
#[path = "../tests/common/t4_u5b.rs"]
mod t4_u5b;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

/// One load case's displacement publication, as `run_case` published it
/// formed it (test-only probe in `run_case`).
#[derive(Debug, Clone)]
pub(crate) struct DisplacementPublication {
    pub case_id: String,
    pub bound: Option<MemberBoundInputs>,
    pub solve: Vec<f64>,
    pub published: Vec<f64>,
    /// Whether the case's nonlinear state evidence (the held-status source's
    /// evidence) says Confirmed; `None` for a case without that evidence.
    pub confirmed_evidence: Option<bool>,
}

/// One nonlinear case's reaction vector as published (the source of its
/// nonlinear reaction rows and derived-normal evidence), beside the loop's own
/// reactions (test-only probe in `append_nonlinear_support_loop_results`).
#[derive(Debug, Clone)]
pub(crate) struct NonlinearReactionPublication {
    pub case_id: String,
    pub loop_reactions: Vec<f64>,
    pub published: Vec<f64>,
}

thread_local! {
    /// Records each nonlinear case's published reaction vector beside the loop's own.
    pub(crate) static NONLINEAR_REACTION_PROBE: std::cell::RefCell<Option<Vec<NonlinearReactionPublication>>> = const { std::cell::RefCell::new(None) };
    /// Makes the named case's solve displacement non-finite before publication.
    pub(crate) static FORCED_NONFINITE_DISPLACEMENT: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
    /// Forces `corrected_nodal_displacements` to fail at this DOF with this step.
    pub(crate) static FORCED_CORRECTION_FAILURE: std::cell::Cell<Option<(usize, open_pipe_stress_frame_kernel::structural::DisplacementFailureKind)>> = const { std::cell::Cell::new(None) };
    pub(crate) static DISPLACEMENT_PUBLICATION_PROBE: std::cell::RefCell<Option<Vec<DisplacementPublication>>> = const { std::cell::RefCell::new(None) };
}

/// Runs `run` with the probe armed and returns its records.
pub(crate) fn with_publication_probe<T>(run: impl FnOnce() -> T) -> (T, Vec<DisplacementPublication>) {
    DISPLACEMENT_PUBLICATION_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
    let out = run();
    let records = DISPLACEMENT_PUBLICATION_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
    (out, records)
}

/// The exact corrected displacement of every DOF, rounded once: the value the
/// rule publishes where the bound exists and every component forms.
pub(crate) fn corrected(record: &DisplacementPublication) -> Vec<f64> {
    let bound = record.bound.as_ref().expect("a bound");
    (0..record.solve.len()).map(|dof| {
        let mut sum = open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator::new();
        for v in [record.solve[dof], bound.w[dof], bound.w2[dof]] { sum.add(v).unwrap(); }
        let v = sum.round().unwrap();
        if v == 0.0 { 0.0 } else { v }
    }).collect()
}

/// Every nodal displacement row of a single-case run equals `expected`
/// (SI, by DOF) in its published unit, bit for bit.
pub(crate) fn assert_nodal_rows(results: &[ResultItem], node_ids: &[&str], expected: &[f64], label: &str) {
    const TAILS: [(&str, f64); 6] = [("ux", 1000.0), ("uy", 1000.0), ("uz", 1000.0), ("rx", 1.0), ("ry", 1.0), ("rz", 1.0)];
    let mut checked = 0;
    for (node, id) in node_ids.iter().enumerate() {
        for (k, (tail, scale)) in TAILS.iter().enumerate() {
            let row_id = format!("result:disp:{}:{tail}", stable_suffix(id));
            let row = results.iter().find(|r| r.id == row_id).unwrap_or_else(|| panic!("{label}: {row_id}"));
            assert_eq!(row.value.to_bits(), (expected[node * DOF_PER_NODE + k] * scale).to_bits(), "{label}: {row_id}");
            checked += 1;
        }
    }
    assert!(checked > 0, "{label}");
}

fn solve(request: &Value, mode: PreviewSolverMode, force_binary64: bool) -> Value {
    U4A_FORCE_BINARY64.with(|flag| flag.set(force_binary64));
    let source = run_linear_static_preview_value_with_mode(crate::pressure_runtime::with_pressurised_cases_named(request), mode);
    U4A_FORCE_BINARY64.with(|flag| flag.set(false));
    serde_json::to_value(source.unwrap()).unwrap()
}

fn value(source: &Value, id: &str) -> f64 {
    source["results"].as_array().unwrap().iter().find(|r| r["id"] == id)
        .unwrap_or_else(|| panic!("{id}"))["value"].as_f64().unwrap()
}

fn integrity(source: &Value) -> String {
    source["diagnostics"].as_array().unwrap().iter()
        .find(|d| d["code"].as_str().unwrap().starts_with("NUMERICAL_INTEGRITY_"))
        .map(|d| format!("{} {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap()))
        .unwrap_or_default()
}

/// Every node's magnitude row is the correctly rounded norm of its published
/// mm components.
fn assert_magnitudes(source: &Value, label: &str) {
    for row in source["results"].as_array().unwrap().iter().filter(|r| r["kind"] == "displacement_magnitude") {
        let id = row["id"].as_str().unwrap();
        let [x, y, z] = ["ux", "uy", "uz"].map(|c| value(source, &format!("{id}:{c}")));
        assert_eq!(row["value"].as_f64().unwrap().to_bits(), norm3(x, y, z).to_bits(), "{label}: {id}");
    }
}

/// pressure-2, AS-F-R: with the bound, node B's published translation and
/// rotation lie within t4_u5b's criterion of the exact solution in both modes;
/// the forced fallback publishes the solve's own u, binary64 member rows and
/// the unavailable-check sentence, and its node B x translation (sparse) is
/// over the criterion: the defect ruling (A) repairs.
#[test]
fn pressure_2_publishes_the_corrected_displacement_only_with_the_bound() {
    let oracle = t4_u5b::oracle();
    let case = &oracle["cases"]["AS-F-R"];
    let request = t4_u5b::request("AS-F-R", case);
    let node = &case["nodes"]["node:B"];
    let fraction = |source: &Value, kind: &str, component: usize| {
        let (tail, scale) = match kind { "translation_global_m" => (["ux", "uy", "uz"][component], 1e-3), _ => (["rx", "ry", "rz"][component], 1.0) };
        let observed = value(source, &format!("result:disp:node-B:{tail}")) * scale;
        let expected = t4_u5b::number(&node[kind][component]);
        (observed - expected).abs() / (1e-9 * expected.abs().max(1e-5))
    };
    for mode in MODES {
        let corrected = solve(&request, mode, false);
        assert!(integrity(&corrected).starts_with("NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{mode:?}: {}", integrity(&corrected));
        for kind in ["translation_global_m", "rotation_global_rad"] {
            for component in 0..3 {
                let f = fraction(&corrected, kind, component);
                assert!(f <= 1.0, "{mode:?} node B {kind}[{component}]: {f:.3e} of the criterion");
            }
        }
        assert_magnitudes(&corrected, &format!("corrected {mode:?}"));
        let fallback = solve(&request, mode, true);
        assert!(integrity(&fallback).contains("member rows are binary64 recoveries"), "{mode:?}: {}", integrity(&fallback));
        assert_magnitudes(&fallback, &format!("fallback {mode:?}"));
        if mode == PreviewSolverMode::SparseInteractive {
            let f = fraction(&fallback, "translation_global_m", 0);
            assert!(f > 1.0, "the solve's own u at node B x: {f:.3e}");
        }
    }
}

fn nodal_bits(source: &Value) -> Vec<(String, u64)> {
    source["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"].as_str().unwrap().starts_with("global_nodal_"))
        .map(|r| (r["id"].as_str().unwrap().to_string(), r["value"].as_f64().unwrap().to_bits()))
        .collect()
}

/// preview-physics-2, both paths. The bundled demo model (Passed) publishes
/// the corrected displacement, differing from the forced fallback's solve u
/// in last bits at some component. The F2a milestone request is Sensitive:
/// it has no certified bound, so its nodal rows are the solve's own u, bit
/// for bit the forced fallback's. Magnitudes stay consistent on every path.
#[test]
fn preview_physics_2_publishes_the_corrected_displacement_only_with_the_bound() {
    let demo: Value = serde_json::json!({
        "model": serde_json::from_str::<Value>(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap(),
        "materials": [],
    });
    let milestone: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap();
    for mode in MODES {
        let corrected = solve(&demo, mode, false);
        assert_eq!(corrected["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/preview-physics-2");
        assert!(integrity(&corrected).starts_with("NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{mode:?}: {}", integrity(&corrected));
        let fallback = solve(&demo, mode, true);
        assert_magnitudes(&corrected, &format!("demo corrected {mode:?}"));
        assert_magnitudes(&fallback, &format!("demo fallback {mode:?}"));
        let (c, f) = (nodal_bits(&corrected), nodal_bits(&fallback));
        assert_eq!(c.iter().map(|r| &r.0).collect::<Vec<_>>(), f.iter().map(|r| &r.0).collect::<Vec<_>>());
        assert!(c.iter().zip(&f).any(|(a, b)| a.1 != b.1), "{mode:?}: the corrected and the solve's displacement agree in every bit");
        let sensitive = solve(&milestone, mode, false);
        assert_eq!(sensitive["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/preview-physics-2");
        assert!(integrity(&sensitive).starts_with("NUMERICAL_INTEGRITY_SENSITIVE"), "{mode:?}: {}", integrity(&sensitive));
        assert_eq!(nodal_bits(&sensitive), nodal_bits(&solve(&milestone, mode, true)), "{mode:?}: a case without the bound keeps the solve's u");
        assert_magnitudes(&sensitive, &format!("milestone {mode:?}"));
    }
}

/// Codex scrutiny 4 (a): a present bound with one component that cannot be
/// formed (its exact sum overflows binary64) yields no corrected vector at all,
/// naming the first such component and the step that failed (the caller then
/// withdraws the bound for the whole case); without the overflow every
/// component is corrected.
#[test]
fn an_unformable_component_names_itself_and_its_step() {
    use open_pipe_stress_frame_kernel::structural::DisplacementFailureKind;
    let bound = |w1: f64| MemberBoundInputs {
        w: vec![2.0_f64.powi(-40), w1],
        w2: vec![0.0, 0.0],
        exponents: vec![0, 0],
        lambda_sigma: 0.0,
        lift_radius: Vec::new(),
    };
    let u = [1.0, f64::MAX];
    let formed = corrected_nodal_displacements(&u, &bound(0.0)).unwrap();
    assert_eq!(formed.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), [(1.0 + 2.0_f64.powi(-40)).to_bits(), f64::MAX.to_bits()]);
    // u + w far beyond binary64 at component 1: the enclosure itself cannot be
    // formed (the kernel's first step), so certification fails.
    let unformable = corrected_nodal_displacements(&u, &bound(f64::MAX)).unwrap_err();
    assert_eq!((unformable.dof, unformable.kind), (1, DisplacementFailureKind::Certify));
    // Just past binary64 too (f64::MAX plus half its ulp): the enclosure's own
    // range check runs first and fails, so this is also a certification
    // failure. No input the enclosure admits fails afterwards at representing
    // the value; that step stays typed so that a later kernel change cannot be
    // reported as the wrong kind (the product-level test forces it).
    let edge = corrected_nodal_displacements(&u, &bound(2.0_f64.powi(970))).unwrap_err();
    assert_eq!((edge.dof, edge.kind), (1, DisplacementFailureKind::Certify));
    // A bound shorter than u (a component outside it) cannot certify that component.
    let short = MemberBoundInputs { w: vec![2.0_f64.powi(-40)], w2: vec![0.0], exponents: vec![0], lambda_sigma: 0.0, lift_radius: Vec::new() };
    let outside = corrected_nodal_displacements(&[1.0, 2.0], &short).unwrap_err();
    assert_eq!((outside.dof, outside.kind), (1, DisplacementFailureKind::Certify));
}

/// Codex's item-6 round-2 scrutiny: the as-built rule through the product's
/// output writers, on its counterexample u = 1 m, w = 2^-53, w2 = 2^-130 at
/// UX. The corrected displacement is rounded once in SI (1 + 2^-52 m), then
/// the nodal row and the nonlinear support's displacement row convert it to
/// mm as every row is: RN(1000 * RN_SI(u + w + w2)) = 1000.0000000000002 mm,
/// not the single rounding of the converted exact sum (1000.0000000000001 mm).
/// The magnitude is the correctly rounded norm of the three published mm rows.
#[test]
fn published_mm_rows_convert_the_si_corrected_displacement() {
    let mut u = vec![0.0; DOF_PER_NODE];
    u[UX] = 1.0;
    let mut bound = MemberBoundInputs { w: vec![0.0; DOF_PER_NODE], w2: vec![0.0; DOF_PER_NODE], exponents: vec![0; DOF_PER_NODE], lambda_sigma: 0.0, lift_radius: Vec::new() };
    bound.w[UX] = 2.0_f64.powi(-53);
    bound.w2[UX] = 2.0_f64.powi(-130);
    let published = corrected_nodal_displacements(&u, &bound).unwrap();
    assert_eq!(published[UX].to_bits(), (1.0 + 2.0_f64.powi(-52)).to_bits(), "rounded once in SI");
    let as_built = 1000.0 * published[UX];
    let single = {
        let mut sum = ExactAccumulator::new();
        for v in [1000.0, 1000.0 * 2.0_f64.powi(-53), 1000.0 * 2.0_f64.powi(-130)] { sum.add(v).unwrap(); }
        sum.round().unwrap()
    };
    assert_eq!((as_built, single), (1000.0000000000002, 1000.0000000000001), "the two roundings differ here");
    let mut results = Vec::new();
    append_node_displacement_component_results(&mut results, "node:N", &published, 0);
    let row = |id: &str| results.iter().find(|r| r.id == id).unwrap().value;
    assert_eq!(row("result:disp:node-N:ux").to_bits(), as_built.to_bits(), "nodal row");
    assert_eq!(displacement_magnitude_mm(&published, 0).to_bits(), norm3(row("result:disp:node-N:ux"), row("result:disp:node-N:uy"), row("result:disp:node-N:uz")).to_bits());
    let support = NonlinearSupport {
        support_id: "support:NL".to_string(),
        node_index: 0,
        dof: FrameDof::Ux,
        behavior: NonlinearSupportBehavior::OneWay { active_when: ActivationSense::PositiveReaction },
        gap: None,
        friction_coefficient: None,
    };
    let id = format!("result:nonlinear-support:{}:{}-displacement", stable_suffix("support:NL"), FrameDof::Ux.as_str());
    let mut support_rows = vec![ResultItem { component_body: None, id: id.clone(), value: 0.0, ..results[0].clone() }];
    restate_nonlinear_support_displacements(&mut support_rows, &[support], &published);
    assert_eq!(support_rows[0].value.to_bits(), as_built.to_bits(), "nonlinear support row");
}

/// DEL-04-07 *Published displacement* (Agent 0's decision, reading (ii)): where
/// the bound exists but a component's correction cannot be formed, the bound
/// and the confirmed state are withdrawn before anything is published. On the
/// bundled demo model (two confirmed nonlinear cases), with the correction
/// forced to fail at node 2's UX, in both modes and for both steps, every part
/// of the document agrees that no published result relies on the confirmation:
/// the case's verdict and its named reason, the named warning, the nodal rows
/// (the solve's own u), the member, support and connector recoveries (no
/// certified recovery record), the state evidence the held-status source would
/// take (not Confirmed), the case quality and its downstream eligibility.
#[test]
fn a_correction_that_cannot_be_formed_withdraws_the_bound_for_the_whole_case() {
    use open_pipe_stress_frame_kernel::structural::DisplacementFailureKind::{Certify, Represent};
    let demo: Value = serde_json::json!({
        "model": serde_json::from_str::<Value>(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap(),
        "materials": [],
    });
    let cases = ["load:L-100", "load:L-200"];
    let run = |mode, forced| {
        FORCED_CORRECTION_FAILURE.with(|f| f.set(forced));
        crate::tests::KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
        let (source, published) = with_publication_probe(|| solve(&demo, mode, false));
        let recoveries = crate::tests::KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
        FORCED_CORRECTION_FAILURE.with(|f| f.set(None));
        (source, published, recoveries)
    };
    let diagnostic = |source: &Value, id: String| {
        source["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == id.as_str()).cloned()
    };
    let bases = |source: &Value| {
        source["numerical_quality"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect::<Vec<_>>()
    };
    let node = serde_json::from_str::<Value>(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap()["nodes"][1]["id"]
        .as_str()
        .unwrap()
        .to_string();
    for mode in MODES {
        // The unforced run is the positive control: confirmed, bound present, certified recoveries.
        let (normal, published, recoveries) = run(mode, None);
        assert!(!recoveries.is_empty(), "{mode:?}");
        for case in cases {
            let integrity = diagnostic(&normal, integrity_diagnostic_id(case)).unwrap();
            assert_eq!(integrity["code"], "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{mode:?} {case}");
            assert!(diagnostic(&normal, format!("diagnostic:displacement-correction:{case}")).is_none());
            let record = published.iter().find(|r| r.case_id == case).unwrap();
            assert!(record.bound.is_some() && record.confirmed_evidence == Some(true), "{mode:?} {case}");
            // The two bases differ on this case: the corrected displacement is not the solve's own.
            assert!(record.published.iter().zip(&record.solve).any(|(a, b)| a.to_bits() != b.to_bits()), "{mode:?} {case}: the bases coincide");
        }
        for (kind, step) in [(Certify, "certified"), (Represent, "represented in binary64")] {
            let label = format!("{mode:?} {kind:?}");
            let (source, published, recoveries) = run(mode, Some((DOF_PER_NODE, kind)));
            assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED", "{label}");
            assert!(recoveries.is_empty(), "{label}: a certified recovery was formed");
            let clause = format!("the corrected displacement of {node}:UX could not be {step} (forced by test)");
            for case in cases {
                let integrity = diagnostic(&source, integrity_diagnostic_id(case)).unwrap();
                let message = integrity["message"].as_str().unwrap();
                assert_eq!(integrity["code"], "NUMERICAL_INTEGRITY_SENSITIVE", "{label} {case}");
                assert!(message.contains(&format!("the member-relative formation check ran, but {clause}; all published nodal displacements and member, support and connector rows for this case are recovered in binary64 from the solve's own displacement")), "{label} {case}: {message}");
                assert!(!message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE), "{label} {case}");
                assert!(!message.contains("is not available"), "{label} {case}");
                let warning = diagnostic(&source, format!("diagnostic:displacement-correction:{case}")).unwrap();
                assert_eq!((warning["code"].as_str(), warning["severity"].as_str()), (Some("DISPLACEMENT_CORRECTION_UNAVAILABLE"), Some("warning")), "{label}");
                assert_eq!(warning["message"], format!("Load case {case}: {clause}; no published result of this case relies on the member-relative formation check."), "{label}");
                assert_eq!(warning["affected_refs"], serde_json::json!([case]), "{label}");
                let quality = source["numerical_quality"]["cases"].as_array().unwrap().iter().find(|q| q["basis_ref"]["ref_id"] == case).unwrap();
                assert_eq!(quality["solve_quality"], "sensitive", "{label} {case}");
                let record = published.iter().find(|r| r.case_id == case).unwrap();
                assert!(record.bound.is_none(), "{label} {case}: the bound was used");
                assert_eq!(record.published.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), record.solve.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), "{label} {case}");
                assert_eq!(record.confirmed_evidence, Some(false), "{label} {case}: the state evidence still says Confirmed");
            }
            // The member, support and connector rows changed basis: some differ from the confirmed run's.
            assert!(differing_rows(&normal, &source) > 0, "{label}: no recovered row changed basis");
            assert_eq!(crate::result_export::semantic_contract::numerical_use_standing(&source, &bases(&source)), "needs_recompute", "{label}");
            assert!(crate::result_export::semantic_contract::for_source(&source).is_ok(), "{label}: the reader refuses");
        }
    }
}

/// The number of non-displacement result rows (member, section, support and
/// connector recoveries) whose value differs in bits between two runs.
fn differing_rows(a: &Value, b: &Value) -> usize {
    let rows = |source: &Value| {
        source["results"].as_array().unwrap().iter()
            .filter(|r| !r["id"].as_str().unwrap().starts_with("result:disp:"))
            .filter_map(|r| Some((r["id"].as_str()?.to_string(), r["value"].as_f64()?.to_bits())))
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    let (a, b) = (rows(a), rows(b));
    assert_eq!(a.keys().collect::<Vec<_>>(), b.keys().collect::<Vec<_>>(), "the same rows are published");
    a.iter().filter(|(id, bits)| b[*id] != **bits).count()
}

/// The same decision on the linear route (pressure-2, AS-F-R, a Passed case with
/// its bound): withdrawn, sensitive with the named reason and the warning, every
/// row the solve's own, no certified recovery, and the bases shown to differ.
#[test]
fn a_linear_correction_that_cannot_be_formed_withdraws_the_bound() {
    use open_pipe_stress_frame_kernel::structural::DisplacementFailureKind::Certify;
    let oracle = t4_u5b::oracle();
    let request = t4_u5b::request("AS-F-R", &oracle["cases"]["AS-F-R"]);
    let run = |mode, forced| {
        FORCED_CORRECTION_FAILURE.with(|f| f.set(forced));
        crate::tests::KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
        let (source, published) = with_publication_probe(|| solve(&request, mode, false));
        let recoveries = crate::tests::KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
        FORCED_CORRECTION_FAILURE.with(|f| f.set(None));
        (source, published, recoveries)
    };
    for mode in MODES {
        let (normal, published, recoveries) = run(mode, None);
        assert!(integrity(&normal).starts_with("NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{mode:?}");
        assert!(!recoveries.is_empty(), "{mode:?}");
        assert_eq!(published.len(), 1, "{mode:?}");
        assert!(published[0].bound.is_some() && published[0].confirmed_evidence.is_none(), "{mode:?}");
        assert!(published[0].published.iter().zip(&published[0].solve).any(|(a, b)| a.to_bits() != b.to_bits()), "{mode:?}: the bases coincide");
        let (source, published, recoveries) = run(mode, Some((0, Certify)));
        let case = published[0].case_id.clone();
        assert!(integrity(&source).starts_with("NUMERICAL_INTEGRITY_SENSITIVE"), "{mode:?}: {}", integrity(&source));
        assert!(integrity(&source).contains("the member-relative formation check ran, but the corrected displacement of "), "{mode:?}");
        assert!(source["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "DISPLACEMENT_CORRECTION_UNAVAILABLE" && d["affected_refs"] == serde_json::json!([case])), "{mode:?}");
        assert!(recoveries.is_empty(), "{mode:?}");
        assert!(published[0].bound.is_none(), "{mode:?}");
        assert_eq!(published[0].published.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), published[0].solve.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), "{mode:?}");
        assert!(differing_rows(&normal, &source) > 0, "{mode:?}: no recovered row changed basis");
        assert_magnitudes(&source, &format!("withdrawn {mode:?}"));
    }
}

/// The fallback authorises nothing the ordinary publication would refuse: with
/// the correction forced to fail AND a non-finite solve displacement in one
/// case, that case is still refused (the document is blocked) and no warning
/// stands for it as if its rows had been published.
#[test]
fn a_withdrawn_bound_is_no_route_around_a_non_finite_refusal() {
    use open_pipe_stress_frame_kernel::structural::DisplacementFailureKind::Represent;
    let demo: Value = serde_json::json!({
        "model": serde_json::from_str::<Value>(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap(),
        "materials": [],
    });
    for mode in MODES {
        FORCED_CORRECTION_FAILURE.with(|f| f.set(Some((DOF_PER_NODE, Represent))));
        FORCED_NONFINITE_DISPLACEMENT.with(|f| *f.borrow_mut() = Some("load:L-200".to_string()));
        let source = solve(&demo, mode, false);
        FORCED_NONFINITE_DISPLACEMENT.with(|f| *f.borrow_mut() = None);
        FORCED_CORRECTION_FAILURE.with(|f| f.set(None));
        assert_eq!(source["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert!(source["results"].as_array().is_none_or(|r| r.is_empty()), "{mode:?}");
        let diagnostics = source["diagnostics"].as_array().unwrap();
        assert!(diagnostics.iter().any(|d| d["severity"] == "blocking"), "{mode:?}");
        assert!(!diagnostics.iter().any(|d| d["id"] == "diagnostic:displacement-correction:load:L-200"), "{mode:?}");
    }
}

/// DEL-04-07, blocked documents: the demo model with load:L-200 made
/// non-finite after its verdict, in both modes. The document is blocked; each
/// retained passed evaluation (load:L-100's, which completed, and load:L-200's,
/// written before its refusal) is inspection-only under the exact prefix with
/// its prior outcome, its original message verbatim after it; every case and
/// the document are not assessed with their evidence references kept; the
/// blocking failure stands; and the readers read nothing as assessed. No
/// committed blocked document carries a verdict, so this constructed document
/// is where the behaviour is shown.
#[test]
fn a_blocked_document_keeps_its_evaluations_for_inspection_only() {
    use crate::result_export::{reader_verdict, semantic_contract};
    let demo: Value = serde_json::json!({
        "model": serde_json::from_str::<Value>(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap(),
        "materials": [],
    });
    for mode in MODES {
        let solved = solve(&demo, mode, false);
        let solved_message = |case: &str| {
            solved["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == integrity_diagnostic_id(case).as_str()).unwrap()["message"]
                .as_str()
                .unwrap()
                .to_string()
        };
        FORCED_NONFINITE_DISPLACEMENT.with(|f| *f.borrow_mut() = Some("load:L-200".to_string()));
        let blocked = solve(&demo, mode, false);
        FORCED_NONFINITE_DISPLACEMENT.with(|f| *f.borrow_mut() = None);
        assert_eq!(blocked["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        let diagnostics = blocked["diagnostics"].as_array().unwrap();
        assert!(diagnostics.iter().any(|d| d["severity"] == "blocking"), "{mode:?}");
        for case in ["load:L-100", "load:L-200"] {
            let id = integrity_diagnostic_id(case);
            let d = diagnostics.iter().find(|d| d["id"] == id.as_str()).unwrap_or_else(|| panic!("{mode:?} {case}"));
            assert_eq!((d["code"].as_str(), d["severity"].as_str()), (Some(INSPECTION_ONLY_CODE), Some("info")), "{mode:?} {case}");
            assert_eq!(d["affected_refs"], serde_json::json!([case]), "{mode:?} {case}");
            let message = d["message"].as_str().unwrap();
            assert!(message.starts_with(&format!("{INSPECTION_ONLY_LEAD}NUMERICAL_INTEGRITY_CHECKS_PASSED{INSPECTION_ONLY_TAIL}")), "{mode:?} {case}: {message}");
            let (prior, original) = semantic_contract::inspection_only_prior(message).unwrap();
            assert_eq!(prior, "NUMERICAL_INTEGRITY_CHECKS_PASSED");
            assert!(original.starts_with("M03-INTEGRITY-v1 represented original-equation structural evidence for load case "), "{mode:?} {case}");
            if case == "load:L-100" {
                assert_eq!(original, solved_message(case), "{mode:?}: the completed case's evaluation is kept verbatim");
            }
            let quality = blocked["numerical_quality"]["cases"].as_array().unwrap().iter().find(|q| q["basis_ref"]["ref_id"] == case).unwrap();
            assert_eq!(quality["solve_quality"], "not_assessed", "{mode:?} {case}");
            assert_eq!(quality["evidence_refs"], serde_json::json!([id]), "{mode:?} {case}");
        }
        assert_eq!(blocked["numerical_quality"]["status"], "not_assessed", "{mode:?}");
        let bases = blocked["numerical_quality"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect::<Vec<_>>();
        assert_eq!(semantic_contract::numerical_use_standing(&blocked, &bases), "needs_recompute", "{mode:?}");
        let standing = reader_verdict::standing(&blocked, None, None);
        assert_ne!(standing["eligible"], true, "{mode:?}");
    }
}

/// DEL-04-07 *Blocked document*, as narrowed (R1): no blocked document carries a
/// passing or sensitive verdict, and a failure keeps its classification. Each
/// case's expected standing is stated by kind; a case with no evaluation is not
/// assessed; the aggregate is the most severe case.
#[test]
fn a_blocked_document_carries_no_passing_verdict_and_keeps_its_failures() {
    let model: PreviewModel = serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap();
    let integrity = |case: &str, code: &str, severity: &str| {
        diag(&integrity_diagnostic_id(case), code, severity, format!("original evaluation of {case}"), vec![case.to_string()])
    };
    let blocking = diag("diagnostic:test:blocking", "SOLVER_SYSTEM_BLOCKED", "blocking", "the case's refusal".to_string(), vec!["load:L-200".to_string()]);
    let unassessed = ("numerically_unresolved", "not_assessed", "not_assessed", "not_claimed");
    let mechanism = ("physical_mechanism_witnessed", "failed", "not_assessed", "not_claimed");
    let unqualified = ("numerically_unresolved", "unresolved", "not_assessed", "unresolved");
    for (label, retained, expected, aggregate) in [
        ("mixed passed and mechanism",
         vec![integrity("load:L-100", "NUMERICAL_INTEGRITY_CHECKS_PASSED", "info"), integrity("load:L-200", "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM", "blocking"), blocking.clone()],
         vec![("load:L-100", unassessed), ("load:L-200", mechanism)], "failed"),
        ("unqualified recovery basis",
         vec![integrity("load:L-100", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "warning"), blocking.clone()],
         vec![("load:L-100", unqualified)], "unresolved"),
        ("passed and sensitive only",
         vec![integrity("load:L-100", "NUMERICAL_INTEGRITY_CHECKS_PASSED", "info"), integrity("load:L-200", "NUMERICAL_INTEGRITY_SENSITIVE", "warning"), blocking.clone()],
         vec![("load:L-100", unassessed), ("load:L-200", unassessed)], "not_assessed"),
    ] {
        let envelope = serde_json::to_value(blocked_envelope(model.clone(), retained.clone())).unwrap();
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{label}");
        assert_eq!(envelope["numerical_quality"]["status"], aggregate, "{label}");
        let cases = envelope["numerical_quality"]["cases"].as_array().unwrap();
        assert_eq!(cases.len(), model.load_cases.len(), "{label}");
        for case in cases {
            let id = case["basis_ref"]["ref_id"].as_str().unwrap();
            let want = expected.iter().find(|(case_id, _)| *case_id == id).map_or(unassessed, |(_, standing)| *standing);
            assert_eq!(
                (case["structural_status"].as_str().unwrap(), case["solve_quality"].as_str().unwrap(), case["model_matrix_fidelity"].as_str().unwrap(), case["accuracy_evidence"].as_str().unwrap()),
                want,
                "{label} {id}"
            );
            assert!(!matches!(case["solve_quality"].as_str(), Some("checks_passed" | "sensitive")), "{label} {id}: no passing verdict");
        }
        for original in &retained {
            let kept = envelope["diagnostics"].as_array().unwrap().iter().find(|d| d["id"] == original.id.as_str()).unwrap_or_else(|| panic!("{label}: {} kept", original.id));
            if matches!(original.code.as_str(), "NUMERICAL_INTEGRITY_CHECKS_PASSED" | "NUMERICAL_INTEGRITY_SENSITIVE") {
                assert_eq!(kept["code"], INSPECTION_ONLY_CODE, "{label}");
            } else {
                assert_eq!((kept["code"].as_str(), kept["severity"].as_str(), kept["message"].as_str()), (Some(original.code.as_str()), Some(original.severity.as_str()), Some(original.message.as_str())), "{label}: unrelabelled");
            }
            if original.id.starts_with(INTEGRITY_DIAGNOSTIC_PREFIX) {
                let case = &original.affected_refs[0];
                let quality = cases.iter().find(|q| q["basis_ref"]["ref_id"] == case.as_str()).unwrap();
                assert_eq!(quality["evidence_refs"], serde_json::json!([original.id]), "{label}: the evidence reference is kept");
            }
        }
    }
}

/// The rewrite is applied only to a passed or sensitive evaluation, so a
/// second application is the identity: nothing is ever wrapped twice.
#[test]
fn inspection_only_is_never_wrapped_twice() {
    for code in ["NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE"] {
        let mut d = diag("diagnostic:numerical-integrity:a", code, "info", "original".to_string(), vec!["a".to_string()]);
        inspection_only(&mut d);
        assert_eq!((d.code.as_str(), d.severity.as_str()), (INSPECTION_ONLY_CODE, "info"));
        assert_eq!(d.message, format!("{INSPECTION_ONLY_LEAD}{code}{INSPECTION_ONLY_TAIL}original"));
        let once = d.clone();
        inspection_only(&mut d);
        assert_eq!((d.code, d.message, d.severity), (once.code, once.message, once.severity));
    }
    for code in ["NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "NUMERICAL_INTEGRITY_FAILED", "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM"] {
        let mut d = diag("diagnostic:numerical-integrity:a", code, "warning", "original".to_string(), vec!["a".to_string()]);
        inspection_only(&mut d);
        assert_eq!((d.code.as_str(), d.message.as_str()), (code, "original"), "{code} is not a verdict on a published result and is kept");
    }
}

/// The held operand of op_mf_held_status's H-16a (a `minuend_final`
/// subtraction on the V-1 bar, friction mu = 1/4 with a zero normal: E by an
/// input decision), built as its test builds it.
fn held_operand_request() -> Value {
    const PROV: &str = "op_mf";
    let (od, wall, k, length) = (0.12_f64, 0.01_f64, 1_048_576.0_f64, 4.0_f64);
    let inner = od - 2.0 * wall;
    let modulus = 4.0 * k / (std::f64::consts::PI / 4.0 * (od * od - inner * inner));
    let fx = |id: &str, f: f64| serde_json::json!({"id":id,"category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_x","magnitude":{"value":f,"unit":"N"},"dimension":"force","provenance":PROV});
    let thermal = |id: &str, dt: f64| serde_json::json!({"id":id,"category":"thermal","target":{"type":"element","pipe":"pipe:a-b"},"direction":"global_x","magnitude":{"value":dt,"unit":"degC"},"dimension":"temperature_interval","provenance":PROV});
    let case = |id: &str, loads: Vec<Value>| serde_json::json!({"id":id,"primitive_loads":loads,"provenance":PROV});
    serde_json::json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:op-mf","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":"node:a","position":{"x":0.0,"y":0.0,"z":0.0},"provenance":PROV},{"id":"node:b","position":{"x":length,"y":0.0,"z":0.0},"provenance":PROV}],
        "pipe_segments":[{"id":"pipe:a-b","from":"node:a","to":"node:b","section":{"outside_diameter":{"value":od,"unit":"m"},"wall_thickness":{"value":wall,"unit":"m"}},
            "material":"material:m","y_reference":{"x":0.0,"y":1.0,"z":0.0},"provenance":PROV}],
        "supports":[{"id":"support:a","node":"node:a","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":PROV},
            {"id":"support:t","node":"node:b","family":"nonlinear","restraints":[],"provenance":PROV,
             "nonlinear":{"behavior":"friction","dof":"UX","initial_state":"sticking","friction_coefficient":{"value":0.25,"unit":"1"},"normal_reaction":{"value":0.0,"unit":"N"}}}],
        "components":[],
        "materials":[{"id":"material:m","elastic_modulus":{"value":modulus,"unit":"Pa"},"shear_modulus":{"value":modulus / 2.6,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":1.0 / 65_536.0,"unit":"1/degC"},"provenance":"op_mf_invented_test_values"}],
        "load_cases":[case("case:S", vec![fx("f-s", 512.0)]), case("case:O", vec![fx("f-o", 512.0), thermal("t-o", -16.0)])],
        "combinations":[{"id":"comb:o-s-held","basis":"result_state_subtraction","minuend_id":"case:O","subtrahend_id":"case:S",
            "difference_basis":"operating_minus_sustained","subtrahend_support_state":"minuend_final","provenance":PROV}]},"materials":[]})
}

/// DEL-04-07 *Published displacement* on the held route, both modes: the held
/// record is staged, and the case's publication-basis decision withdraws it.
/// In the control (H-16a) the record is established with its conditions. With
/// a correction that cannot be formed, it is not established, with the named
/// reason and no conditions, and the reader admits the document; the stale
/// form (the control's established record put back) is refused, so a
/// combination consumer cannot take established evidence from a withdrawn case.
#[test]
fn a_held_operand_whose_correction_fails_is_not_established() {
    use open_pipe_stress_frame_kernel::structural::DisplacementFailureKind::Certify;
    let request = held_operand_request();
    let model = request["model"].clone();
    let record = |v: &Value| v["contract_evidence"]["held_status_derivation"].as_array().unwrap_or_else(|| panic!("no held record: {:#}", v["diagnostics"]))[0].clone();
    let validate = |v: &Value| crate::result_export::reader_verdict::validate("preview-physics", v, Some(&model), None);
    for mode in MODES {
        let run = |forced: bool| {
            FORCED_CORRECTION_FAILURE.with(|f| f.set(forced.then_some((0, Certify))));
            let v = serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
            FORCED_CORRECTION_FAILURE.with(|f| f.set(None));
            assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?} forced={forced}: {:#}", v["diagnostics"]);
            v
        };
        let control = run(false);
        let established = record(&control);
        assert_eq!(established["standing"]["kind"], "established", "{mode:?}: {established:#}");
        assert!(established.get("conditions").is_some(), "{mode:?}");
        assert_eq!(validate(&control)["ok"], true, "{mode:?}: {}", validate(&control));
        let failed = run(true);
        let withdrawn = record(&failed);
        assert_eq!(withdrawn["standing"]["kind"], "not_established", "{mode:?}: {withdrawn:#}");
        let reason = withdrawn["standing"]["reason"].as_str().unwrap();
        assert!(reason.starts_with("Not established: the corrected displacement of ") && reason.contains("could not be certified (forced by test)"), "{mode:?}: {reason}");
        assert!(withdrawn.get("conditions").is_none(), "{mode:?}");
        assert_eq!(validate(&failed)["ok"], true, "{mode:?}: {}", validate(&failed));
        // The stale form: the control's established record in the withdrawn case's document.
        let mut stale = failed.clone();
        stale["contract_evidence"]["held_status_derivation"][0]["standing"] = established["standing"].clone();
        stale["contract_evidence"]["held_status_derivation"][0]["conditions"] = established["conditions"].clone();
        assert_ne!(validate(&stale)["ok"], true, "{mode:?}: the reader admits an established held record in a withdrawn case");
    }
}
