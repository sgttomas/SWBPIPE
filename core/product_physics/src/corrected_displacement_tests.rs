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

/// One load case's displacement publication, as `published_nodal_displacements`
/// formed it (test-only probe in `run_case`).
#[derive(Debug, Clone)]
pub(crate) struct DisplacementPublication {
    pub case_id: String,
    pub bound: Option<MemberBoundInputs>,
    pub solve: Vec<f64>,
    pub published: Vec<f64>,
}

thread_local! {
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
    let source = run_linear_static_preview_value_with_mode(request.clone(), mode);
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
/// formed (its exact sum overflows binary64) publishes the solve's own u for
/// the whole case, the formable component included; without the overflow the
/// formable component is corrected.
#[test]
fn an_unformable_component_falls_back_for_the_whole_case() {
    let bound = |w1: f64| MemberBoundInputs {
        w: vec![2.0_f64.powi(-40), w1],
        w2: vec![0.0, 0.0],
        exponents: vec![0, 0],
        lambda_sigma: 0.0,
    };
    let u = [1.0, f64::MAX];
    let formed = published_nodal_displacements(&u, Some(&bound(0.0)));
    assert_eq!(formed.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), [(1.0 + 2.0_f64.powi(-40)).to_bits(), f64::MAX.to_bits()]);
    let unformable = published_nodal_displacements(&u, Some(&bound(f64::MAX)));
    assert_eq!(unformable.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), u.map(f64::to_bits), "the whole case keeps u");
    // A bound shorter than u (a component outside it) is unformable too.
    let short = MemberBoundInputs { w: vec![2.0_f64.powi(-40)], w2: vec![0.0], exponents: vec![0], lambda_sigma: 0.0 };
    assert_eq!(published_nodal_displacements(&[1.0, 2.0], Some(&short)), [1.0, 2.0]);
    assert_eq!(published_nodal_displacements(&u, None), u);
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
    let mut bound = MemberBoundInputs { w: vec![0.0; DOF_PER_NODE], w2: vec![0.0; DOF_PER_NODE], exponents: vec![0; DOF_PER_NODE], lambda_sigma: 0.0 };
    bound.w[UX] = 2.0_f64.powi(-53);
    bound.w2[UX] = 2.0_f64.powi(-130);
    let published = published_nodal_displacements(&u, Some(&bound));
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
