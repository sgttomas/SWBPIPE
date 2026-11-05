//! R-6/R-7 D-1 tests (DEL-04-03 Work `skew-restraints.md` §11.1 D-1, §11.3 C1-C9;
//! C9: `test_local_parameters()` only). The product oblique cases are checked against
//! an independent twin that never reaches the skew code: the whole model rotated about
//! Z by the exact rational rotation R = [[4/5, −3/5], [3/5, 4/5]], which maps the
//! authored direction (3, 4, 0) to +Y, so the twin's record is an axis-aligned UY
//! record. A rigid rotation of every input leaves the spectrum invariant, and the
//! in-plane effective mass m_x + m_y and m_z with it. All models are invented.
use super::outcome::{run_modal_dense, ModalOutcome, ModalStanding};
use super::tests::{modal, q, section, with, PROV};
use super::tests_eigen::test_local_parameters;
use serde_json::{json, Value};

/// **TEST-LOCAL** relative bound for the twin comparison (the design's seed of 1e-9).
const TEST_LOCAL_TWIN: f64 = 1e-9;

/// A(0) – M(2) – B(4) along +X (or its rotation by R), anchored at A, rotations held
/// at B, and `record` at B.
fn request(rotated: bool, record: Value) -> Value {
    let at = |x: f64| if rotated { [0.8 * x, 0.6 * x] } else { [x, 0.0] };
    let node = |id: &str, x: f64| {
        let p = at(x);
        json!({"id": id, "position": {"x": p[0], "y": p[1], "z": 0.0}, "provenance": PROV})
    };
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "section": section(), "material": "m",
        "y_reference": {"x": 0, "y": 0, "z": 1}, "provenance": PROV});
    let model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:skew-modal", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("A", 0.0), node("M", 2.0), node("B", 4.0)],
        "pipe_segments": [pipe("pipe:AM", "A", "M"), pipe("pipe:MB", "M", "B")],
        "materials": [{"id": "m", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(80e9, "Pa"), "provenance": PROV}],
        "supports": [
            {"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
            {"id": "rot_B", "node": "B", "family": "anchor", "restraints": ["RX", "RY", "RZ"], "provenance": PROV},
            record,
        ],
        "components": [],
        "load_cases": [{"id": "case:f", "primitive_loads": [{"id": "load:f", "category": "concentrated_force", "target": {"type": "node", "node": "B"},
            "direction": "global_z", "magnitude": q(1000.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}],
        "combinations": []});
    with(json!({"model": model, "materials": []}), json!({"mode_count": 8}))
}

fn stop(direction: Option<[f64; 3]>) -> Value {
    let mut s = json!({"id": "stop_B", "node": "B", "family": "line_stop", "restraints": ["UY"], "provenance": PROV});
    if let Some(d) = direction {
        s["direction"] = json!({"x": d[0], "y": d[1], "z": d[2]});
    }
    s
}

fn spring(direction: Option<[f64; 3]>) -> Value {
    let mut s = json!({"id": "spring_B", "node": "B", "family": "spring", "restraints": ["UY"],
        "stiffness": {"dof": "UY", "value": q(2.0e6, "N/m")}, "provenance": PROV});
    if let Some(d) = direction {
        s["direction"] = json!({"x": d[0], "y": d[1], "z": d[2]});
    }
    s
}

fn solved(request: Value) -> ModalOutcome {
    run_modal_dense(&modal(request), &test_local_parameters()).unwrap_or_else(|found| panic!("refused: {found:#?}"))
}

fn close(a: f64, b: f64, scale: f64, what: &str) {
    assert!((a - b).abs() <= TEST_LOCAL_TWIN * scale.max(b.abs()), "{what}: {a:e} against {b:e}");
}

fn compare_with_twin(name: &str, skew: &ModalOutcome, twin: &ModalOutcome, d: [f64; 3]) {
    assert_eq!(skew.modes.len(), twin.modes.len(), "{name}: mode count");
    assert_eq!(skew.guard.free_dimension, twin.guard.free_dimension, "{name}: the reduced n′");
    assert_eq!(skew.guard.global_dimension, 18, "{name}: the guard stays on the global 6N");
    assert!(matches!(skew.standing, ModalStanding::ChecksPassed), "{name}: {:?}", skew.standing);
    assert!(matches!(twin.standing, ModalStanding::ChecksPassed), "{name}: twin {:?}", twin.standing);
    let norm = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    // The effective masses' scale: the twin's largest modal effective mass.
    let total = twin.modes.iter().flat_map(|m| m.effective_mass).fold(0.0_f64, f64::max);
    for (a, b) in skew.modes.iter().zip(&twin.modes) {
        close(a.eigenvalue, b.eigenvalue, 0.0, &format!("{name}: ω² of mode {}", a.index));
        close(a.effective_mass[0] + a.effective_mass[1], b.effective_mass[0] + b.effective_mass[1], total, &format!("{name}: in-plane m_eff of mode {}", a.index));
        close(a.effective_mass[2], b.effective_mass[2], total, &format!("{name}: m_eff,z of mode {}", a.index));
        // The lifted, published shape is a full global vector, normalized on itself.
        let largest = a.shape.iter().enumerate().filter(|(i, _)| i % 6 < 3).fold(0.0_f64, |m, (_, v)| m.max(v.abs()));
        let rotations = a.shape.iter().enumerate().filter(|(i, _)| i % 6 >= 3).fold(0.0_f64, |m, (_, v)| m.max(v.abs()));
        // B1: the reference entry is exactly 1 and its class is at most 1; the flags
        // name global DOFs published as 0.
        assert_eq!(a.shape[a.reference_dof].to_bits(), 1.0f64.to_bits(), "{name}: mode {} reference", a.index);
        // The normalizing class (translations in case (a), rotations in (b)) is ≤ 1.
        let class = if a.reference_dof % 6 < 3 { largest } else { rotations };
        assert!(class <= 1.0, "{name}: lifted normalization {largest} {rotations}");
        assert!(a.below_normal_range.shape.iter().all(|&d| d < 18 && a.shape[d] == 0.0), "{name}: flags");
        assert_eq!(a.shape.len(), 18);
        if name.contains("stop") {
            // dᵀφ_B = 0 on the published binary64 lift, within its rounding.
            let b0 = 12;
            let dot: f64 = (0..3).map(|i| d[i] * a.shape[b0 + i]).sum();
            assert!(dot.abs() <= 8.0 * f64::EPSILON * norm * largest.max(rotations), "{name}: dᵀφ = {dot:e} in mode {}", a.index);
        }
    }
}

#[test]
fn an_oblique_stop_matches_its_rotated_axis_twin() {
    let d = [3.0, 4.0, 0.0];
    let skew = solved(request(false, stop(Some(d))));
    let twin = solved(request(true, stop(None)));
    compare_with_twin("stop", &skew, &twin, d);
    // C3: check 1's formation difference on the oblique case, reported.
    eprintln!(
        "D-1 oblique stop: formation difference K′ {:e}, M′ {:e} (axis twin K {:e}, M {:e})",
        skew.integrity.formation.stiffness.norm, skew.integrity.formation.mass.norm,
        twin.integrity.formation.stiffness.norm, twin.integrity.formation.mass.norm
    );
    // C4: the kept-subspace mass is labelled as such.
    assert!(skew.disclosures.iter().any(|d| d.contains("kept-subspace mass")), "{:?}", skew.disclosures);
    assert!(!twin.disclosures.iter().any(|d| d.contains("kept-subspace mass")));
}

#[test]
fn an_oblique_spring_matches_its_rotated_axis_twin() {
    let d = [3.0, 4.0, 0.0];
    let skew = solved(request(false, spring(Some(d))));
    let twin = solved(request(true, spring(None)));
    compare_with_twin("spring", &skew, &twin, d);
}

/// RT-A3 condition 4, modal: the same skew-only mechanism (A, M, B on X, each with a
/// stop (3, 4, 0) and a spring (0, 3, 4); six ground rows) passes prepare's count and
/// is refused by name by the static path's ground screen in C1's evidence solve.
#[test]
fn a_skew_only_mechanism_is_refused_before_any_eigen_solve() {
    let mut wire = request(false, stop(Some([3.0, 4.0, 0.0])));
    let mut supports = Vec::new();
    for node in ["A", "M", "B"] {
        supports.push(json!({"id": format!("stop_{node}"), "node": node, "family": "line_stop", "restraints": ["UY"], "direction": {"x": 3.0, "y": 4.0, "z": 0.0}, "provenance": PROV}));
        supports.push(json!({"id": format!("spring_{node}"), "node": node, "family": "spring", "restraints": ["UY"], "direction": {"x": 0.0, "y": 3.0, "z": 4.0},
            "stiffness": {"dof": "UY", "value": q(2.0e6, "N/m")}, "provenance": PROV}));
    }
    wire["model"]["supports"] = json!(supports);
    let found = run_modal_dense(&modal(wire), &test_local_parameters()).err().expect("refused");
    let blocking: Vec<_> = found.iter().filter(|d| d.severity == "blocking").collect();
    assert_eq!(blocking.len(), 1, "{blocking:?}");
    assert_eq!(blocking[0].code, "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM");
    assert!(blocking[0].message.contains("Mechanism { direction: [0.0, 0.0, 0.0, 1.0"), "{}", blocking[0].message);
}

/// C8 (a), dynamics' B2: `modal_result_document` refuses a model with a restraint basis
/// by name before the solved projection (`modal_doc_solved`, whose text is untouched); its
/// rotated axis twin still publishes a solved document.
#[test]
fn the_modal_document_refuses_a_skew_model_by_name() {
    let params = test_local_parameters();
    for record in [stop(Some([3.0, 4.0, 0.0])), spring(Some([3.0, 4.0, 0.0]))] {
        let doc = super::document::modal_result_document(&request(false, record.clone()), &params, "test-local");
        assert_eq!(doc["status"], "refused", "{doc}");
        assert!(doc.to_string().contains(super::integrity::MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED), "{doc}");
        let mut axis = record;
        axis.as_object_mut().unwrap().remove("direction");
        let twin = super::document::modal_result_document(&request(true, axis), &params, "test-local");
        assert_ne!(twin["status"], "refused", "{twin}");
    }
}

/// C8 (b), dynamics' B2: the test-only phase replay mirrors the axis path only, so it refuses
/// a model with a restraint basis by name; no observation or measurand replays it.
#[test]
fn the_phase_replay_refuses_a_skew_model_by_name() {
    let refused = super::phase_measurand_tests::replay_modal_phases(&modal(request(false, stop(Some([3.0, 4.0, 0.0])))), &test_local_parameters())
        .err()
        .expect("refused");
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].code, super::integrity::MODAL_PHASE_REPLAY_SKEW_RESTRAINT_UNSUPPORTED);
    assert!(super::phase_measurand_tests::replay_modal_phases(&modal(request(true, stop(None))), &test_local_parameters()).is_ok());
}
