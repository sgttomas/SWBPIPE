//! W-16 stage 2 (DEL-04-03 Design `pf-nonlinear-imposed-motion.md` §5): the
//! PP consumers of the independent references in
//! `validation/references/w16_nonlinear_motion/` (exact rational, generated
//! without product code), in both solver modes.
//!
//! R-a: rigid anchors and restraints carry stage-1 primitives as base
//! prescribed values in the nonlinear active-set loop and its K-D5
//! confirmation; every nonlinear ground stays fixed. A-1 (a moving anchor
//! under a one-way prop, and its anchor-rotation twin), A-2 (a settling
//! derived-normal source), A-3 (the coupled L-frame, with the two mutants it
//! kills) and a rigid translation of A-2 (the property, in a direction no
//! nonlinear support acts on).
//!
//! Criterion: each certified row within its K-D5 ball bound B of the exact
//! value; nodal displacements within 1e-9 relative (an exact zero against
//! 1e-9 of the case's largest translation or rotation); every prescribed DOF
//! bit for bit.
use super::*;
use crate::corrected_displacement_tests::with_publication_probe;

const SUPPORT_SLOT: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const COMPONENTS: [&str; 6] = ["ux", "uy", "uz", "rx", "ry", "rz"];
const REFERENCES: &str = include_str!("../../../validation/references/w16_nonlinear_motion/references.json");
const SUMS: &str = include_str!("../../../validation/references/w16_nonlinear_motion/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

pub(super) struct Request {
    pub(super) name: &'static str,
    pub(super) text: &'static str,
    pub(super) expected: &'static [(&'static str, Expect)],
}

/// A reference case's expected outcome.
#[derive(Clone, Copy, Debug)]
pub(super) enum Expect {
    /// Passed with the confirmation; one strict state.
    Passed,
    /// The confirmation fails closed with this named §2.4 reason (a weak
    /// boundary the loop converges on).
    NotEstablished(&'static str),
    /// As `NotEstablished`, with either reason: at a weak unilateral boundary
    /// the loop converges on either weak state, by solver mode.
    NotEstablishedAny(&'static [&'static str]),
    /// A weak unilateral boundary at which the loop's binary64 classifier
    /// alternates between the two weak states: the loop does not converge and
    /// the case is withheld before any confirmation (fails closed).
    LoopNotConverged,
    /// A gap case (R1): never confirmed; the exact strict-gap decisions select
    /// the state, and the standing stays `RECOVERY_BASIS_UNQUALIFIED`.
    /// Displacements and the slot reaction within 1e-9 relative.
    Gap,
    /// A linear case (R-c, no nonlinear support): passed, with no nonlinear
    /// confirmation line.
    Linear,
}
use Expect::{Gap, Linear, LoopNotConverged, NotEstablished, NotEstablishedAny, Passed};

const STICK_WEAK: &str = "support support:NL-F: stick capacity not established";

pub(super) const R_A: [Request; 4] = [
    Request {
        name: "a1",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/a1.json"),
        expected: &[
            ("load:ACTIVE", Passed),
            ("load:LIFTED", Passed),
            // s = s*: zero reaction and zero separation, weakly active (R3).
            ("load:BOUNDARY", LoopNotConverged),
            ("load:ROT-ACTIVE", Passed),
            ("load:ROT-LIFTED", Passed),
            ("load:ROT-BOUNDARY", LoopNotConverged),
        ],
    },
    Request {
        name: "a2",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/a2.json"),
        expected: &[("load:STICK", Passed), ("load:SLIDE", Passed), ("load:BOUNDARY", NotEstablished(STICK_WEAK))],
    },
    Request {
        name: "a3",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/a3.json"),
        expected: &[("load:SLIDE", Passed)],
    },
    Request {
        name: "a2_translated",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/a2_translated.json"),
        expected: &[("load:STICK", Passed), ("load:SLIDE", Passed), ("load:BOUNDARY", NotEstablished(STICK_WEAK))],
    },
];

pub(super) fn references() -> serde_json::Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn sha256(text: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

pub(super) fn bound_pair(value: &serde_json::Value) -> (f64, f64) {
    (value[0].as_f64().unwrap(), value[1].as_f64().unwrap())
}

/// |v - x| <= b for some exact x in [lo, hi], decided exactly: v <= hi + b and
/// v >= lo - b, with every operand binary64 and the sums exact.
pub(super) fn within_exact(v: f64, (lo, hi): (f64, f64), b: f64) -> bool {
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    let sign = |a: f64, c: f64, d: f64| {
        let mut sum = ExactAccumulator::new();
        sum.add(a).and_then(|()| sum.add(c)).and_then(|()| sum.add(d)).map(|()| sum.signum())
    };
    matches!(sign(hi, b, -v), Ok(s) if s >= 0) && matches!(sign(v, -lo, b), Ok(s) if s >= 0)
}

pub(super) fn single_case(request: &serde_json::Value, case: &str) -> serde_json::Value {
    let mut wire = request.clone();
    let cases = wire["model"]["load_cases"].as_array_mut().unwrap();
    cases.retain(|c| c["id"] == case);
    assert_eq!(cases.len(), 1);
    wire
}

/// The request, bound to the oracle and to SHA256SUMS by its bytes.
pub(super) fn bound_request(request: &Request) -> (serde_json::Value, serde_json::Value) {
    let oracle = references()["requests"][request.name].clone();
    let digest = sha256(request.text);
    assert_eq!(digest, oracle["request_sha256"].as_str().unwrap(), "{}: the oracle binds the request bytes", request.name);
    let path = format!("  {}", oracle["request"].as_str().unwrap());
    let line = SUMS.lines().find(|l| l.ends_with(&path)).unwrap_or_else(|| panic!("SHA256SUMS lists {path}"));
    assert_eq!(&line[..64], digest);
    (serde_json::from_str(request.text).unwrap(), oracle)
}

#[test]
fn the_references_are_the_frozen_bytes_and_every_cross_check_agrees() {
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).unwrap();
    assert_eq!(&line[..64], sha256(REFERENCES));
    let doc = references();
    let checks = doc["crosscheck"].as_array().unwrap();
    assert!(!checks.is_empty() && checks.iter().all(|c| c["agrees"] == true));
}

/// The actual normalized section leaves and moduli the generator was given.
fn leaves_match(request: &serde_json::Value) {
    let doc = references();
    let mut model: PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
    let mut materials = model.materials.clone();
    let mut diagnostics = Vec::new();
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).unwrap();
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    let section = &doc["normalized_section"];
    for frame in &built.frame_elements {
        for (actual, key) in [
            (frame.section.area, "As_m2"),
            (frame.section.second_moment_y, "I_m4"),
            (frame.section.second_moment_z, "I_m4"),
            (frame.section.torsion_constant, "J_m4"),
        ] {
            assert_eq!(actual.to_bits(), section[key].as_f64().unwrap().to_bits(), "{key}: {actual:?}");
        }
        assert_eq!(frame.section.elastic_modulus.to_bits(), doc["elastic_modulus_pa"].as_f64().unwrap().to_bits());
        assert_eq!(frame.section.shear_modulus.to_bits(), doc["shear_modulus_pa"].as_f64().unwrap().to_bits());
    }
}

fn row<'a>(output: &'a MechanicsEnvelope, kind: &str, entity: &str) -> &'a ResultItem {
    let rows = output.results.iter().filter(|r| r.kind == kind && r.entity_ref == entity).collect::<Vec<_>>();
    assert_eq!(rows.len(), 1, "one {kind} row for {entity}");
    rows[0]
}

/// Nodal displacement rows (mm, rad) against the exact displacements: 1e-9
/// relative, an exact zero against 1e-9 of the case's largest magnitude of
/// its family.
fn displacements_match(output: &MechanicsEnvelope, exact: &serde_json::Value, at: &str) -> f64 {
    let mut scale = [0.0_f64; 2];
    for components in exact.as_object().unwrap().values() {
        for (k, pair) in components.as_array().unwrap().iter().enumerate() {
            let (lo, hi) = bound_pair(pair);
            scale[k / 3] = scale[k / 3].max(lo.abs()).max(hi.abs());
        }
    }
    let mut checked = 0;
    let mut worst = 0.0_f64;
    for (node, components) in exact.as_object().unwrap() {
        for (k, pair) in components.as_array().unwrap().iter().enumerate() {
            let factor = if k < 3 { 1000.0 } else { 1.0 };
            let id = format!("result:disp:{}:{}", stable_suffix(node), COMPONENTS[k]);
            let observed = output.results.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{at}: {id}")).value / factor;
            let (lo, hi) = bound_pair(pair);
            let expected = if lo == hi { lo } else { 0.5 * lo + 0.5 * hi };
            let tolerance = 1e-9 * if expected == 0.0 { scale[k / 3] } else { expected.abs() };
            assert!((observed - expected).abs() <= tolerance, "{at}: {id} observed {observed:e} exact {expected:e}");
            worst = worst.max((observed - expected).abs() / if expected == 0.0 { scale[k / 3] } else { expected.abs() });
            checked += 1;
        }
    }
    assert!(checked > 0);
    worst
}

/// One reference case in one mode: standing, states, every certified group
/// and slot within B of the exact value, the displacements, and the
/// prescribed DOFs bit for bit. Returns the output and the certified records.
pub(super) fn run_case(
    request: &Request,
    wire: &serde_json::Value,
    oracle: &serde_json::Value,
    case: &str,
    expect: Expect,
    mode: PreviewSolverMode,
) -> (MechanicsEnvelope, Vec<formation_guard::RecoveryRecord>) {
    let at = format!("{} {case} {mode:?}", request.name);
    let exact = &oracle["cases"][case];
    if let LoopNotConverged = expect {
        let output = run_linear_static_preview_value_with_mode(single_case(wire, case), mode).unwrap();
        assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE", "{at}");
        assert!(output.results.is_empty(), "{at}: nothing is published");
        let blocking = output.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect::<Vec<_>>();
        assert_eq!(blocking, ["NONLINEAR_SUPPORT_NONCONVERGENCE", "SOLVER_SYSTEM_BLOCKED"], "{at}");
        // The exact reference: no strict state; the two weak states the loop alternates between.
        assert!(exact["strict_states"].as_array().unwrap().is_empty(), "{at}");
        assert_eq!(exact["weak_states"].as_array().unwrap().len(), 2, "{at}");
        return (output, Vec::new());
    }
    tests::KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
    let (output, publications) = with_publication_probe(|| run_linear_static_preview_value_with_mode(single_case(wire, case), mode).unwrap());
    let records = tests::KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
    assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{at}: {:?}", output.diagnostics);
    let integrity = output.diagnostics.iter().find(|d| d.id == integrity_diagnostic_id(case)).unwrap();
    // Every prescribed base DOF publishes its case value g_b bit for bit.
    assert_eq!(publications.len(), 1, "{at}");
    let model: PreviewModel = serde_json::from_value(wire["model"].clone()).unwrap();
    for (key, pair) in exact["base_prescribed"].as_object().unwrap() {
        let (dof, node) = key.split_once('@').unwrap();
        let index = model.nodes.iter().position(|n| n.id == node).unwrap() * DOF_PER_NODE
            + ["UX", "UY", "UZ", "RX", "RY", "RZ"].iter().position(|d| *d == dof).unwrap();
        let (lo, hi) = bound_pair(pair);
        assert_eq!(lo, hi, "{at}: a prescribed value is binary64");
        assert_eq!(publications[0].published[index].to_bits(), lo.to_bits(), "{at}: {key} publishes g exactly");
        assert_eq!(publications[0].solve[index].to_bits(), lo.to_bits(), "{at}: {key} is the loop's g");
    }
    // R-b (§4): one relative-displacement row per moved ground, an authored
    // zero included, fl(U - g_s) from the actual published displacement U and
    // then the row's unit; none for a case that moves no ground.
    let strict = exact["strict_states"].as_array().unwrap();
    let grounds = exact["ground_motion"].as_object().unwrap();
    let relative = output.results.iter().filter(|r| r.kind == "nonlinear_support_final_relative_displacement").collect::<Vec<_>>();
    assert_eq!(relative.len(), grounds.len(), "{at}");
    for (support, pair) in grounds {
        let (g, hi) = bound_pair(pair);
        assert_eq!(g, hi, "{at}: a ground offset is binary64");
        let row = relative.iter().find(|r| &r.entity_ref == support).unwrap_or_else(|| panic!("{at}: {support}"));
        let record = model.supports.iter().find(|s| &s.id == support).unwrap();
        let k = ["UX", "UY", "UZ", "RX", "RY", "RZ"].iter().position(|d| *d == record.nonlinear.as_ref().unwrap().dof).unwrap();
        let index = model.nodes.iter().position(|n| n.id == record.node).unwrap() * DOF_PER_NODE + k;
        let scale = if k < 3 { 1000.0 } else { 1.0 };
        assert_eq!(row.unit, if k < 3 { "mm" } else { "rad" }, "{at}");
        assert_eq!(row.value.to_bits(), ((publications[0].published[index] - g) * scale).to_bits(), "{at}: {support} is fl(U - g_s)");
        if let Some(state) = strict.first() {
            let (lo, hi) = bound_pair(&state["signed"][support]["relative_displacement"]);
            let expected = if lo == hi { lo } else { 0.5 * lo + 0.5 * hi };
            let observed = row.value / scale;
            if expected == 0.0 {
                assert_eq!(observed.to_bits(), 0.0_f64.to_bits(), "{at}: {support} at its ground");
            } else {
                assert!((observed - expected).abs() <= 1e-9 * expected.abs(), "{at}: {support} {observed:e} exact {expected:e}");
            }
        }
    }
    if let Gap = expect {
        assert_eq!(integrity.code, "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "{at}: {}", integrity.message);
        assert!(integrity.message.contains("selected represented-exact-ratio"), "{at}: the exact strict-gap decisions select the state: {}", integrity.message);
        assert!(!integrity.message.contains(NONLINEAR_RELATIVE_DISPLACEMENT_CLAUSE), "{at}");
        assert!(records.is_empty(), "{at}: a gap case publishes no certified rows");
        // The exact state, or Active at the weak equality (the classifier admits R = 0).
        let state = strict.first();
        assert!(state.is_some() || !exact["weak_states"].as_array().unwrap().is_empty(), "{at}");
        let code = row(&output, "nonlinear_support_active_set_state_code", "support:GAP").value;
        let expected = state.map_or("active", |s| s["states"]["support:GAP"].as_str().unwrap());
        assert_eq!(code, if expected == "active" { 1.0 } else { 0.0 }, "{at}");
        if let Some(state) = state {
            let reaction = row(&output, "nonlinear_support_final_reaction", "support:GAP").value;
            let (lo, hi) = bound_pair(&state["signed"]["support:GAP"]["value"]);
            let exact_reaction = 0.5 * lo + 0.5 * hi;
            assert!((reaction - exact_reaction).abs() <= 1e-9 * exact_reaction.abs().max(1.0), "{at}: {reaction:e} exact {exact_reaction:e}");
            let worst = displacements_match(&output, &state["displacements"], &at);
            println!("W16-NL {at}: gap, exact decisions; displacement max relative error {worst:.3e}");
        }
        return (output, records);
    }
    let reasons = match expect {
        NotEstablished(reason) => Some(vec![reason]),
        NotEstablishedAny(reasons) => Some(reasons.to_vec()),
        _ => None,
    };
    if let Some(reasons) = reasons {
        assert_eq!(integrity.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{at}: {}", integrity.message);
        assert!(
            reasons.iter().any(|reason| integrity.message.contains(&format!("the converged support state is not established at 128-bit precision ({reason}); member rows are binary64 recoveries"))),
            "{at}: {}",
            integrity.message
        );
        assert!(records.is_empty(), "{at}: a fail-closed case publishes no certified rows");
        assert!(!integrity.message.contains(NONLINEAR_RELATIVE_DISPLACEMENT_CLAUSE), "{at}");
        assert!(exact["strict_states"].as_array().unwrap().is_empty(), "{at}: no state qualifies strictly");
        assert!(!exact["weak_states"].as_array().unwrap().is_empty(), "{at}: the boundary is exact");
        return (output, records);
    }
    assert_eq!(integrity.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{at}: {}", integrity.message);
    assert_eq!(integrity.message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE), !matches!(expect, Linear), "{at}");
    // The relative rows' observation clause follows the K-D5 line exactly when they exist.
    assert_eq!(integrity.message.contains(NONLINEAR_RELATIVE_DISPLACEMENT_CLAUSE), !grounds.is_empty(), "{at}");
    assert_eq!(strict.len(), 1, "{at}: the reference has one strict state");
    assert!(exact["weak_states"].as_array().unwrap().is_empty(), "{at}");
    let state = &strict[0];
    for (support, expected_state) in state["states"].as_object().unwrap() {
        let code = row(&output, "nonlinear_support_active_set_state_code", support).value;
        let expected_code = match expected_state.as_str().unwrap() {
            "inactive" => 0.0,
            "active" => 1.0,
            "sticking" => 2.0,
            "sliding" => 3.0,
            other => panic!("{other}"),
        };
        assert_eq!(code, expected_code, "{at}: {support}");
    }
    let mut judged = 0;
    let mut slot_worst = 0.0_f64;
    for record in &records {
        for group in &record.groups {
            let bound = group.bound.clone().unwrap_or_else(|e| panic!("{at}: {} {e}", record.member));
            let range = bound_pair(&state["groups"][&record.member][&group.location][&group.kind]);
            let slack = group.q.next_up() - group.q;
            assert!(
                within_exact(group.q, range, bound + slack),
                "{at}: {} {} {} q={:e} B={:e} exact={range:?}",
                record.member, group.location, group.kind, group.q, bound
            );
            judged += 1;
        }
    }
    assert!(judged > 0, "{at}");
    for (support, signed) in state["signed"].as_object().unwrap() {
        let Some(slot) = signed.get("slot") else { continue };
        let index = ["UX", "UY", "UZ", "RX", "RY", "RZ"].iter().position(|d| *d == slot.as_str().unwrap()).unwrap();
        let component = output
            .results
            .iter()
            .find(|r| r.kind == "support_reaction_component_v2" && &r.entity_ref == support && r.metadata.as_ref().is_some_and(|m| m.component == SUPPORT_SLOT[index]))
            .unwrap()
            .value;
        let scalar = row(&output, "nonlinear_support_final_reaction", support).value;
        assert_eq!(component.to_bits(), scalar.to_bits(), "{at}: {support}");
        let record = records.iter().find(|r| &r.member == support).unwrap();
        let family = if index < 3 { "support_force" } else { "support_moment" };
        let bound = record.groups.iter().find(|g| g.kind == family).unwrap().bound.clone().unwrap();
        assert!(within_exact(component, bound_pair(&signed["value"]), bound), "{at}: {support} {component:e} B={bound:e} exact={:?}", signed["value"]);
        if signed["value"][0] == 0.0 && signed["value"][1] == 0.0 {
            assert_eq!(component.to_bits(), 0.0_f64.to_bits(), "{at}: an inactive support publishes exact +0");
        } else {
            let (lo, hi) = bound_pair(&signed["value"]);
            slot_worst = slot_worst.max((component - (0.5 * lo + 0.5 * hi)).abs() / lo.abs().max(hi.abs()));
        }
    }
    let worst = displacements_match(&output, &state["displacements"], &at);
    println!("W16-NL {at}: {judged} certified groups within B; nonlinear slot max relative error {slot_worst:.3e}; displacement max relative error {worst:.3e}");
    (output, records)
}

/// §5 R-a: A-1, A-2, A-3 and the translated A-2, both modes.
#[test]
fn r_a_references_bind_and_match_in_both_modes() {
    for request in &R_A {
        let (wire, oracle) = bound_request(request);
        assert_eq!(oracle["part"], "R-a");
        leaves_match(&wire);
        assert_eq!(oracle["cases"].as_object().unwrap().len(), request.expected.len(), "{}", request.name);
        for &(case, expect) in request.expected {
            for mode in MODES {
                run_case(request, &wire, &oracle, case, expect, mode);
            }
        }
    }
}

/// A-3: the published coupled friction force lies within its bound of the
/// exact value, and outside it for both mutants the benchmark must kill (an
/// influence system that carries g_n, and a Stage A that omits the motion).
#[test]
fn a3_kills_the_motion_mutants() {
    let request = &R_A[2];
    let (wire, oracle) = bound_request(request);
    let closed = &references()["a3_reduced_closed_form"];
    for mode in MODES {
        let (output, records) = run_case(request, &wire, &oracle, "load:SLIDE", Passed, mode);
        let phi = row(&output, "nonlinear_support_final_reaction", "support:NL-F").value;
        let record = records.iter().find(|r| r.member == "support:NL-F").unwrap();
        let bound = record.groups.iter().find(|g| g.kind == "support_force").unwrap().bound.clone().unwrap();
        assert!(within_exact(phi, bound_pair(&closed["phi"]), bound), "{mode:?}");
        for mutant in ["mutant_influence_carries_g_phi", "mutant_stage_a_omits_motion_phi"] {
            assert!(!within_exact(phi, bound_pair(&closed[mutant]), bound), "{mode:?}: {mutant} survives");
        }
    }
}

/// An authored zero base motion is the no-motion path: every output row and
/// diagnostic is unchanged (the kd5 U-1 reference, its anchor moved by +0).
#[test]
fn an_authored_zero_base_motion_keeps_every_output() {
    let request: serde_json::Value =
        serde_json::from_str(include_str!("../../../validation/references/kd5_nonlinear/u1_request.json")).unwrap();
    let mut plain = request.clone();
    plain["model"]["schema_version"] = serde_json::json!(crate::MODEL_SCHEMA_VERSION);
    let mut zero = plain.clone();
    for case in zero["model"]["load_cases"].as_array_mut().unwrap() {
        let id = case["id"].as_str().unwrap().to_string();
        case["primitive_loads"].as_array_mut().unwrap().push(serde_json::json!({
            "id": format!("{id}-zero"), "category": "imposed_displacement",
            "target": {"type": "support", "support": "support:ANCHOR", "dof": "UZ"}, "direction": "UZ",
            "magnitude": {"value": 0.0, "unit": "m"}, "dimension": "displacement", "provenance": "w16 invented"}));
    }
    for mode in MODES {
        let a = serde_json::to_value(run_linear_static_preview_value_with_mode(plain.clone(), mode).unwrap()).unwrap();
        let b = serde_json::to_value(run_linear_static_preview_value_with_mode(zero.clone(), mode).unwrap()).unwrap();
        assert_eq!(a["status"], b["status"], "{mode:?}");
        assert_eq!(a["results"], b["results"], "{mode:?}");
        assert_eq!(a["diagnostics"], b["diagnostics"], "{mode:?}");
    }
}


// ------------------------------------------------ R-a negatives (§3)

fn codes(output: &MechanicsEnvelope) -> Vec<String> {
    output.diagnostics.iter().map(|d| d.code.clone()).collect()
}

/// A-1's request with one case: ACTIVE, plus `edit` applied to the model.
fn a1_with(edit: impl Fn(&mut serde_json::Value)) -> serde_json::Value {
    let (wire, _) = bound_request(&R_A[0]);
    let mut wire = single_case(&wire, "load:ACTIVE");
    edit(&mut wire["model"]);
    wire
}

fn motion_on(support: &str, dof: &str, value: f64) -> serde_json::Value {
    serde_json::json!({"id": format!("load:ACTIVE-{support}-{dof}"), "category": "imposed_displacement",
        "target": {"type": "support", "support": support, "dof": dof}, "direction": dof,
        "magnitude": {"value": value, "unit": if dof.starts_with('R') { "rad" } else { "m" }},
        "dimension": if dof.starts_with('R') { "rotation" } else { "displacement" }, "provenance": "w16 invented"})
}

fn refused_whole(wire: &serde_json::Value, code: &str, at: &str) {
    for mode in MODES {
        let output = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
        assert!(codes(&output).contains(&code.to_string()), "{at} {mode:?}: {:?}", codes(&output));
        assert!(output.diagnostics.iter().any(|d| d.code == code && d.severity == "blocking"), "{at}");
        assert_ne!(output.status.mechanics, "MECHANICS_SOLVED", "{at}");
        assert!(output.results.is_empty(), "{at}: the whole document is withheld");
    }
}

/// R-c retires the spring refusals (§11.3): a linear spring's base moves in a
/// model with nonlinear supports.
#[test]
fn a_linear_spring_base_moves() {
    let spring = a1_with(|m| {
        m["supports"].as_array_mut().unwrap().push(serde_json::json!({"id": "support:K", "node": "node:M", "family": "spring",
            "restraints": ["UY"], "stiffness": {"dof": "UY", "value": {"value": 1.0e6, "unit": "N/m"}}, "provenance": "w16 invented"}));
        m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(motion_on("support:K", "UY", 1.0e-4));
    });
    admitted(&spring, "a linear spring's base");
}

/// Solved in both modes with no imposed-displacement diagnostic.
fn admitted(wire: &serde_json::Value, at: &str) {
    for mode in MODES {
        let output = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
        assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{at} {mode:?}: {:?}", codes(&output));
        assert!(!codes(&output).iter().any(|c| c.starts_with("IMPOSED_DISPLACEMENT_")), "{at} {mode:?}: {:?}", codes(&output));
    }
}

/// A mixed document, one valid motion case and one invalid, is refused whole;
/// a combination with a motion case in a nonlinear model stays withheld
/// (`NONLINEAR_COMBINATION_REQUIRES_SOLVE`, §3.1).
#[test]
fn a_mixed_document_is_refused_whole_and_combinations_stay_withheld() {
    let (wire, _) = bound_request(&R_A[0]);
    let mut mixed = single_case(&wire, "load:ACTIVE");
    let mut invalid = mixed["model"]["load_cases"][0].clone();
    invalid["id"] = serde_json::json!("load:INVALID");
    for load in invalid["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = serde_json::json!(format!("{}-invalid", load["id"].as_str().unwrap()));
    }
    invalid["primitive_loads"].as_array_mut().unwrap().push(motion_on("support:NONE", "UZ", 1.0e-4));
    mixed["model"]["load_cases"].as_array_mut().unwrap().push(invalid);
    refused_whole(&mixed, "IMPOSED_DISPLACEMENT_SUPPORT_UNKNOWN", "a mixed document");
    let mut combined = single_case(&wire, "load:ACTIVE");
    let mut lifted = single_case(&wire, "load:LIFTED")["model"]["load_cases"][0].clone();
    lifted["id"] = serde_json::json!("load:LIFTED");
    combined["model"]["load_cases"].as_array_mut().unwrap().push(lifted);
    combined["model"]["combinations"] = serde_json::json!([{"id": "comb:lin", "basis": "mechanics",
        "terms": [{"load_case": "load:ACTIVE", "factor": 1.0}, {"load_case": "load:LIFTED", "factor": 1.0}], "provenance": "w16 invented"}]);
    for mode in MODES {
        let output = serde_json::to_value(run_linear_static_preview_value_with_mode(combined.clone(), mode).unwrap()).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED");
        let gates = output["contract_evidence"]["combination_gates"].as_array().unwrap();
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0]["withheld"], true, "{mode:?}: {gates:?}");
        assert_eq!(gates[0]["reason"], "NONLINEAR_COMBINATION_REQUIRES_SOLVE", "{mode:?}: {gates:?}");
    }
}

// ------------------------------------------------ R-b (§5): a nonlinear support's own ground moves

const LIFT_STICK: &str = "support support:NL-F: stick capacity not established";

const LIFT_WEAK: &[&str] = &["support support:LIFT: separation not established", "support support:LIFT: contact force sign not established"];
// d = d*: zero reaction and zero separation; the loop converges on either weak
// state (by solver mode) and the confirmation fails closed (R3).
const B1: &[(&str, Expect)] = &[("load:ACTIVE", Passed), ("load:LIFTED", Passed), ("load:BOUNDARY", NotEstablishedAny(LIFT_WEAK))];
const B2: &[(&str, Expect)] = &[("load:OPEN", Gap), ("load:CLOSED", Gap), ("load:EQUAL", Gap), ("load:CANCEL", Gap)];
const B3: &[(&str, Expect)] = &[
    ("load:STICK", Passed),
    ("load:STICK-NEG", Passed),
    ("load:SLIDE", Passed),
    ("load:SLIDE-NEG", Passed),
    // |g_t| = g*: zero relative slip at capacity; the classifier chooses
    // Sticking, and the certificate is not established.
    ("load:BOUNDARY", NotEstablished(LIFT_STICK)),
];

pub(super) const R_B: [Request; 12] = [
    Request {
        name: "b1",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b1.json"),
        expected: B1,
    },
    Request {
        name: "b1_spring",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b1_spring.json"),
        expected: &[("load:ACTIVE", Passed), ("load:LIFTED", Passed)],
    },
    Request {
        name: "b1_rot",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b1_rot.json"),
        expected: &[("load:ACTIVE", Passed), ("load:LIFTED", Passed)],
    },
    Request {
        name: "b1_translated",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b1_translated.json"),
        expected: B1,
    },
    Request {
        name: "b2",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b2.json"),
        expected: B2,
    },
    Request {
        name: "b2_positive",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b2_positive.json"),
        expected: B2,
    },
    Request {
        name: "b2_negative_clearance",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b2_negative_clearance.json"),
        expected: &[("load:CLOSED", Gap)],
    },
    Request {
        name: "b2_translated",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b2_translated.json"),
        expected: B2,
    },
    Request {
        name: "b3",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b3.json"),
        expected: B3,
    },
    Request {
        name: "b3_explicit",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b3_explicit.json"),
        expected: B3,
    },
    Request {
        name: "b3_px",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b3_px.json"),
        expected: B3,
    },
    Request {
        name: "b3_translated",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/b3_translated.json"),
        expected: B3,
    },
];

/// §10.4 R-d: the rigid restraints of a mixed nonlinear record take base
/// values. D-1 (a mixed tip record, a rotational lift-off with rigid UY and
/// UZ; UZ drops), D-3 (one record moving its lift-off ground and its rigid
/// UZ, each alone and together) and D-1 translated rigidly in Y (the
/// anchor's and the mixed record's UY).
pub(super) const R_D: [Request; 3] = [
    Request {
        name: "d1",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/d1.json"),
        expected: D1,
    },
    Request {
        name: "d3",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/d3.json"),
        expected: &[("load:GROUND", Passed), ("load:RIGID", Passed), ("load:ACTIVE", Passed), ("load:LIFTED", Passed)],
    },
    Request {
        name: "d1_translated",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/d1_translated.json"),
        expected: D1,
    },
];

const TIP_WEAK: &[&str] = &["support support:TIP: separation not established", "support support:TIP: contact force sign not established"];
// s = s*: zero reaction and zero separation; the loop converges on either
// weak state and the confirmation fails closed (R3).
const D1: &[(&str, Expect)] = &[("load:ACTIVE", Passed), ("load:LIFTED", Passed), ("load:BOUNDARY", NotEstablishedAny(TIP_WEAK))];

/// §5 R-b: B-1 (with its spring and rotational twins), B-2 (both senses, the
/// equality, cancellation and a negative effective clearance), B-3 (both
/// signs, the weak sticking boundary, an explicit normal and P_x != 0) and the
/// property (B-1, B-2 and B-3 translated rigidly with every relevant ground),
/// both modes.
#[test]
fn r_b_references_bind_and_match_in_both_modes() {
    for request in &R_B {
        let (wire, oracle) = bound_request(request);
        assert_eq!(oracle["part"], "R-b");
        leaves_match(&wire);
        assert_eq!(oracle["cases"].as_object().unwrap().len(), request.expected.len(), "{}", request.name);
        for &(case, expect) in request.expected {
            for mode in MODES {
                run_case(request, &wire, &oracle, case, expect, mode);
            }
        }
    }
}

/// §3.2: the R-b refusals, each withholding the whole document.
#[test]
fn r_b_refusals_are_named() {
    // A DOF other than the record's nonlinear DOF.
    let other = a1_with(|m| m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(motion_on("support:PROP", "UY", 1.0e-4)));
    refused_whole(&other, "IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH", "another DOF of the prop");
    // A spring on the nonlinear record itself, on a disjoint DOF: R-c retires
    // this refusal; the nonlinear ground moves alone (§11.3).
    let sprung = a1_with(|m| {
        let prop = m["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:PROP").unwrap();
        prop["family"] = serde_json::json!("spring");
        prop["restraints"] = serde_json::json!(["UY"]);
        prop["stiffness"] = serde_json::json!({"dof": "UY", "value": {"value": 1.0e6, "unit": "N/m"}});
        m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(motion_on("support:PROP", "UZ", 1.0e-4));
    });
    admitted(&sprung, "a spring on the nonlinear record");
    // Another nonlinear support acting on the same node DOF.
    let shared = a1_with(|m| {
        m["supports"].as_array_mut().unwrap().push(serde_json::json!({"id": "support:PROP2", "node": "node:T", "family": "nonlinear",
            "restraints": [], "nonlinear": {"behavior": "one_way", "dof": "UZ", "initial_state": "active", "active_when": "positive_reaction"},
            "provenance": "w16 invented"}));
        m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(motion_on("support:PROP", "UZ", 1.0e-4));
    });
    refused_whole(&shared, "IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT", "two nonlinear supports on one DOF");
    // The gap closing position: 1 m + 1 in is exact as authored, but 1.0 + 0.0254
    // is not in binary64; an overflowing closing position is not finite.
    let (b2, _) = bound_request(&R_B[4]);
    let gap_case = |clearance: f64, unit: &str, motion: f64, motion_unit: &str| {
        let mut wire = single_case(&b2, "load:OPEN");
        let m = &mut wire["model"];
        let gap = m["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:GAP").unwrap();
        gap["nonlinear"]["gap"] = serde_json::json!({"value": clearance, "unit": unit});
        let load = m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().iter_mut().find(|l| l["category"] == "imposed_displacement").unwrap();
        load["magnitude"] = serde_json::json!({"value": motion, "unit": motion_unit});
        wire
    };
    refused_whole(&gap_case(1.0, "m", -1.0, "in"), "IMPOSED_DISPLACEMENT_GAP_CLOSURE_INEXACT", "1 m + 1 in");
    // Overflow, through the typed entry (an integral binary64 of 2^53 or more is
    // refused at the JSON entry): c + sigma*g_s = 1.5e308 + 1.5e308 is not finite.
    let mut request: LinearStaticPreviewRequest = serde_json::from_value(gap_case(1.0, "m", -1.0, "m")).unwrap();
    request.model.supports.iter_mut().find(|s| s.id == "support:GAP").unwrap().nonlinear.as_mut().unwrap().gap.as_mut().unwrap().value = 1.5e308;
    request.model.load_cases[0].primitive_loads.iter_mut().find(|l| l.category == "imposed_displacement").unwrap().magnitude.value = -1.5e308;
    let output = run_linear_static_preview(request);
    assert!(output.diagnostics.iter().any(|d| d.code == "IMPOSED_DISPLACEMENT_GAP_CLOSURE_INEXACT" && d.severity == "blocking"), "{:?}", output.diagnostics);
    assert!(output.results.is_empty());
}

/// An authored zero ground motion keeps every row and diagnostic of the
/// no-motion path, and adds only its relative-displacement row (U - 0) and,
/// on a Passed case, the observation clause (the kd5 U-1 prop).
#[test]
fn an_authored_zero_ground_motion_adds_only_its_relative_row() {
    let request: serde_json::Value =
        serde_json::from_str(include_str!("../../../validation/references/kd5_nonlinear/u1_request.json")).unwrap();
    let mut plain = request.clone();
    plain["model"]["schema_version"] = serde_json::json!(crate::MODEL_SCHEMA_VERSION);
    let mut zero = plain.clone();
    for case in zero["model"]["load_cases"].as_array_mut().unwrap() {
        let id = case["id"].as_str().unwrap().to_string();
        case["primitive_loads"].as_array_mut().unwrap().push(serde_json::json!({
            "id": format!("{id}-zero"), "category": "imposed_displacement",
            "target": {"type": "support", "support": "support:NL-U", "dof": "UZ"}, "direction": "UZ",
            "magnitude": {"value": 0.0, "unit": "m"}, "dimension": "displacement", "provenance": "w16 invented"}));
    }
    for mode in MODES {
        let a = run_linear_static_preview_value_with_mode(plain.clone(), mode).unwrap();
        let b = run_linear_static_preview_value_with_mode(zero.clone(), mode).unwrap();
        assert_eq!(a.status.mechanics, b.status.mechanics);
        let (relative, rest): (Vec<_>, Vec<_>) = b.results.iter().cloned().partition(|r| r.kind == "nonlinear_support_final_relative_displacement");
        assert_eq!(serde_json::to_value(&rest).unwrap(), serde_json::to_value(&a.results).unwrap(), "{mode:?}");
        assert_eq!(relative.len(), 3, "{mode:?}: one per case");
        for row in &relative {
            let displacement = b.results.iter().find(|r| r.kind == "nonlinear_support_final_displacement" && r.basis_ref == row.basis_ref).unwrap();
            assert_eq!(row.value.to_bits(), displacement.value.to_bits(), "{mode:?}: U - 0 = U");
        }
        for (x, y) in a.diagnostics.iter().zip(&b.diagnostics) {
            if x.message != y.message {
                assert_eq!(y.message, format!("{} {NONLINEAR_RELATIVE_DISPLACEMENT_CLAUSE}", x.message), "{mode:?}");
            } else {
                assert_eq!(serde_json::to_value(x).unwrap(), serde_json::to_value(y).unwrap());
            }
        }
        assert_eq!(a.diagnostics.len(), b.diagnostics.len());
    }
}


/// §4 registration: the actual Rust reader (`semantic_contract::for_source`)
/// admits each R-b output with its relative-displacement rows, and refuses one
/// whose relative row names an unlisted support.
#[test]
fn the_rust_reader_admits_the_relative_rows() {
    use open_pipe_stress_result_export::semantic_contract::for_source;
    for (index, case) in [(0, "load:ACTIVE"), (2, "load:LIFTED"), (4, "load:CLOSED"), (8, "load:SLIDE")] {
        let (wire, _) = bound_request(&R_B[index]);
        for mode in MODES {
            let source = serde_json::to_value(run_linear_static_preview_value_with_mode(single_case(&wire, case), mode).unwrap()).unwrap();
            let rows = source["results"].as_array().unwrap();
            assert_eq!(rows.iter().filter(|r| r["kind"] == "nonlinear_support_final_relative_displacement").count(), 1);
            assert!(for_source(&source).is_ok(), "{} {case} {mode:?}: {:?}", R_B[index].name, for_source(&source));
            let mut unlisted = source.clone();
            for row in unlisted["results"].as_array_mut().unwrap() {
                if row["kind"] == "nonlinear_support_final_relative_displacement" {
                    row["entity_ref"] = serde_json::json!("support:UNLISTED");
                }
            }
            assert!(for_source(&unlisted).is_err(), "{} {case} {mode:?}", R_B[index].name);
        }
    }
}

/// §10.4 R-d, both modes; each mixed record's signed support components
/// (its rigid restraints' reactions and its nonlinear slot) within 1e-9
/// relative of the exact rows.
#[test]
fn r_d_references_bind_and_match_in_both_modes() {
    for request in &R_D {
        let (wire, oracle) = bound_request(request);
        assert_eq!(oracle["part"], "R-d");
        leaves_match(&wire);
        assert_eq!(oracle["cases"].as_object().unwrap().len(), request.expected.len(), "{}", request.name);
        for &(case, expect) in request.expected {
            for mode in MODES {
                let (output, _) = run_case(request, &wire, &oracle, case, expect, mode);
                let Some(state) = oracle["cases"][case]["strict_states"].as_array().unwrap().first() else { continue };
                let at = format!("{} {case} {mode:?}", request.name);
                let components = state["signed"]["support:TIP"]["components"].as_array().unwrap();
                let mut worst = 0.0_f64;
                for (index, pair) in components.iter().enumerate() {
                    let (lo, hi) = bound_pair(pair);
                    let exact = 0.5 * lo + 0.5 * hi;
                    let observed = output
                        .results
                        .iter()
                        .find(|r| r.kind == "support_reaction_component_v2" && r.entity_ref == "support:TIP"
                            && r.metadata.as_ref().is_some_and(|m| m.component == SUPPORT_SLOT[index]))
                        .unwrap_or_else(|| panic!("{at}: {}", SUPPORT_SLOT[index]))
                        .value;
                    let scale = components.iter().map(|p| bound_pair(p).1.abs().max(bound_pair(p).0.abs())).fold(0.0, f64::max);
                    assert!((observed - exact).abs() <= 1e-9 * scale, "{at}: {} {observed:e} exact {exact:e}", SUPPORT_SLOT[index]);
                    worst = worst.max((observed - exact).abs() / scale);
                }
                println!("W16-NL {at}: mixed record components max relative error {worst:.3e}");
            }
        }
    }
}

/// The R-d boundary: the same motion on a separate rigid record (D-1 with
/// its UY and UZ split from the lift-off) gives every displacement bit for bit,
/// so a mixed record's rigid restraint is the rigid restraint of stage 1.
#[test]
fn a_mixed_record_restraint_moves_as_a_separate_rigid_record() {
    let (wire, _) = bound_request(&R_D[0]);
    for case in ["load:ACTIVE", "load:LIFTED"] {
        let mixed = single_case(&wire, case);
        let mut split = mixed.clone();
        let supports = split["model"]["supports"].as_array_mut().unwrap();
        supports.iter_mut().find(|s| s["id"] == "support:TIP").unwrap()["restraints"] = serde_json::json!([]);
        supports.push(serde_json::json!({"id": "support:GUIDE", "node": "node:T", "family": "guide",
            "restraints": ["UY", "UZ"], "provenance": "w16 invented"}));
        for load in split["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap() {
            if load["category"] == "imposed_displacement" {
                load["target"]["support"] = serde_json::json!("support:GUIDE");
            }
        }
        for mode in MODES {
            let a = run_linear_static_preview_value_with_mode(mixed.clone(), mode).unwrap();
            let b = run_linear_static_preview_value_with_mode(split.clone(), mode).unwrap();
            assert_eq!(a.status.mechanics, "MECHANICS_SOLVED", "{case} {mode:?}: {:?}", a.diagnostics);
            assert_eq!(b.status.mechanics, "MECHANICS_SOLVED", "{case} {mode:?}: {:?}", b.diagnostics);
            let displacements = |o: &MechanicsEnvelope| {
                o.results.iter().filter(|r| r.id.starts_with("result:disp:")).map(|r| (r.id.clone(), r.value.to_bits())).collect::<Vec<_>>()
            };
            assert!(!displacements(&a).is_empty());
            assert_eq!(displacements(&a), displacements(&b), "{case} {mode:?}");
            let reaction = |o: &MechanicsEnvelope| row(o, "nonlinear_support_final_reaction", "support:TIP").value.to_bits();
            assert_eq!(reaction(&a), reaction(&b), "{case} {mode:?}");
        }
    }
}

/// §10.3 R-d admission: what stays refused on a nonlinear record, and the
/// coincident-restraint count that now includes a mixed record's restraints.
#[test]
fn r_d_refusals_are_named() {
    let (d1, _) = bound_request(&R_D[0]);
    let with = |edit: &dyn Fn(&mut serde_json::Value)| {
        let mut wire = single_case(&d1, "load:ACTIVE");
        edit(&mut wire["model"]);
        wire
    };
    // A DOF the mixed record neither restrains nor acts on.
    let unrestrained = with(&|m| m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(motion_on("support:TIP", "UX", 1.0e-4)));
    refused_whole(&unrestrained, "IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH", "a DOF the mixed record does not hold");
    // Two primitives on one rigid restraint of the record, equal values.
    let duplicate = with(&|m| {
        let loads = m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
        let mut again = loads.iter().find(|l| l["category"] == "imposed_displacement").unwrap().clone();
        again["id"] = serde_json::json!("load:ACTIVE-R2");
        loads.push(again);
    });
    refused_whole(&duplicate, "IMPOSED_DISPLACEMENT_DOF_DUPLICATE", "two values for the mixed record's UZ");
    // The mixed record's UZ coincident with another rigid UZ at the node,
    // targeting either record: named, ahead of the generic repeated-DOF refusal.
    for target in ["support:TIP", "support:STOP"] {
        let coincident = with(&|m| {
            m["supports"].as_array_mut().unwrap().push(serde_json::json!({"id": "support:STOP", "node": "node:T", "family": "vertical_support",
                "restraints": ["UZ"], "provenance": "w16 invented"}));
            for load in m["load_cases"][0]["primitive_loads"].as_array_mut().unwrap() {
                if load["category"] == "imposed_displacement" {
                    load["target"]["support"] = serde_json::json!(target);
                }
            }
        });
        refused_whole(&coincident, "IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT", target);
        for mode in MODES {
            let output = run_linear_static_preview_value_with_mode(coincident.clone(), mode).unwrap();
            assert!(!codes(&output).contains(&"SUPPORT_INPUT_INVALID".to_string()), "{target} {mode:?}: the named check runs first");
        }
    }
    // Unchanged refusals on a nonlinear record (§10.1): a constant-effort
    // record carrying a nonlinear field, on any DOF; a rigid restraint
    // overlapping the record's own nonlinear DOF. A spring-family record's
    // spring axis moves its base (R-c).
    let constant = with(&|m| {
        let tip = m["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:TIP").unwrap();
        tip["family"] = serde_json::json!("constant_effort_support");
        tip["hanger"] = serde_json::json!({"hanger_type": "constant_effort_support", "constant_load": {"value": 375.0, "unit": "N"},
            "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "w16 invented"});
    });
    refused_whole(&constant, "IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID", "a constant-effort record with a nonlinear field");
    let sprung = with(&|m| {
        let tip = m["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:TIP").unwrap();
        tip["family"] = serde_json::json!("spring");
        tip["restraints"] = serde_json::json!(["UZ"]);
        tip["stiffness"] = serde_json::json!({"dof": "UZ", "value": {"value": 1.0e6, "unit": "N/m"}});
    });
    // At the R-d layer this spring axis was refused; R-c moves its base (§11).
    admitted(&sprung, "the spring axis of a nonlinear record");
    let overlap = with(&|m| {
        let tip = m["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:TIP").unwrap();
        tip["restraints"] = serde_json::json!(["UY", "UZ", "RY"]);
    });
    for mode in MODES {
        let output = run_linear_static_preview_value_with_mode(overlap.clone(), mode).unwrap();
        assert!(codes(&output).contains(&"SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT".to_string()), "{mode:?}: {:?}", codes(&output));
        assert!(output.results.is_empty(), "{mode:?}");
    }
    // The exact route refuses every support-targeted primitive before any
    // target classification (`tests/w16_imposed_motion.rs`,
    // the_exact_route_refuses_support_targeted_primitives); R-d does not
    // change that path.
}

/// R-d byte identity: an authored zero on a mixed record's rigid restraint is
/// the no-motion path, every row and diagnostic unchanged (D-1's model).
#[test]
fn an_authored_zero_on_a_mixed_restraint_keeps_every_output() {
    let (wire, _) = bound_request(&R_D[0]);
    for case in ["load:ACTIVE", "load:LIFTED"] {
        let mut plain = single_case(&wire, case);
        plain["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().retain(|l| l["category"] != "imposed_displacement");
        let mut zero = plain.clone();
        zero["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(motion_on("support:TIP", "UZ", 0.0));
        for mode in MODES {
            let a = serde_json::to_value(run_linear_static_preview_value_with_mode(plain.clone(), mode).unwrap()).unwrap();
            let b = serde_json::to_value(run_linear_static_preview_value_with_mode(zero.clone(), mode).unwrap()).unwrap();
            assert_eq!(a["status"]["mechanics"], "MECHANICS_SOLVED", "{case} {mode:?}");
            assert_eq!(a["status"], b["status"], "{case} {mode:?}");
            assert_eq!(a["results"], b["results"], "{case} {mode:?}");
            assert_eq!(a["diagnostics"], b["diagnostics"], "{case} {mode:?}");
        }
    }
}


// ------------------------------------------------ R-c (§11): a linear spring's base moves

const C1: &[(&str, Expect)] = &[("load:AXIAL", Linear), ("load:AXIAL-NEG", Linear), ("load:INEXACT", Linear)];
const C5: &[(&str, Expect)] = &[("load:ACTIVE", Passed), ("load:LIFTED", Passed), ("load:UNEQUAL", Passed)];
const D2: &[(&str, Expect)] = &[("load:SPRING", Passed), ("load:GROUND", Passed), ("load:ALL", Passed)];

/// §11.4: C-1 (axial, its RX twin and its hanger twin, with an inexact k·g),
/// C-2, C-3 (a pair and its equivalent), C-5 (equal and unequal grounds, and
/// the property), a moved spring on a rigidly prescribed DOF, a moved spring
/// reducing a derived friction normal, and D-2 with its same-record twin.
pub(super) const R_C: [Request; 12] = [
    Request { name: "c1", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c1.json"), expected: C1 },
    Request {
        name: "c1_rot",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c1_rot.json"),
        expected: &[("load:TWIST", Linear), ("load:TWIST-NEG", Linear)],
    },
    Request { name: "c1_hanger", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c1_hanger.json"), expected: C1 },
    Request {
        name: "c2",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c2.json"),
        expected: &[("load:SETTLE", Linear), ("load:HALF", Linear), ("load:RISE", Linear)],
    },
    Request { name: "c3", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c3.json"), expected: &[("load:PAIR", Linear)] },
    Request {
        name: "c3_equivalent",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c3_equivalent.json"),
        expected: &[("load:PAIR", Linear)],
    },
    Request { name: "c5", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c5.json"), expected: C5 },
    Request { name: "c5_translated", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c5_translated.json"), expected: C5 },
    Request { name: "c6", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c6.json"), expected: &[("load:SHARED", Linear)] },
    Request {
        name: "c7",
        text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/c7.json"),
        expected: &[("load:SLIDE-N", Passed), ("load:SLIDE-SAME", Passed)],
    },
    Request { name: "d2", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/d2.json"), expected: D2 },
    Request { name: "d2_twin", text: include_str!("../../../validation/references/w16_nonlinear_motion/requests/d2_twin.json"), expected: D2 },
];

fn spring_ids(wire: &serde_json::Value) -> Vec<String> {
    wire["model"]["supports"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["family"] == "spring" || s["family"] == "variable_spring_hanger")
        .map(|s| s["id"].as_str().unwrap().to_string())
        .collect()
}

fn component(output: &MechanicsEnvelope, support: &str, index: usize) -> f64 {
    output
        .results
        .iter()
        .find(|r| r.kind == "support_reaction_component_v2" && r.entity_ref == support
            && r.metadata.as_ref().is_some_and(|m| m.component == SUPPORT_SLOT[index]))
        .unwrap_or_else(|| panic!("{support} {}: {:?}", SUPPORT_SLOT[index],
            output.results.iter().filter(|r| r.entity_ref == support).map(|r| (&r.kind, &r.id, r.value)).collect::<Vec<_>>()))
        .value
}

/// Each spring's six published components within their certified family
/// bound of the exact -k·(u - g) (§11.2); returns the largest relative error.
fn spring_actions_match(
    output: &MechanicsEnvelope,
    records: &[formation_guard::RecoveryRecord],
    state: &serde_json::Value,
    springs: &[String],
    wire: &serde_json::Value,
    at: &str,
) -> f64 {
    let mut worst = 0.0_f64;
    let supports = wire["model"]["supports"].as_array().unwrap();
    for support in springs {
        // A spring on a node DOF a rigid restraint also holds: the existing
        // attribution rule withholds both records' signed rows
        // (`preview_physics::ambiguous_supports`); the displacements and the
        // certified groups are judged in `run_case`.
        let record = supports.iter().find(|s| &s["id"] == support).unwrap();
        let shared = supports.iter().any(|other| other["id"] != record["id"] && other["node"] == record["node"]
            && other["family"] != "spring" && other["restraints"].as_array().unwrap().contains(&record["restraints"][0]));
        if shared {
            assert!(!output.results.iter().any(|r| r.kind == "support_reaction_component_v2" && &r.entity_ref == support), "{at}: {support} withheld");
            continue;
        }
        let components = state["signed"][support]["components"].as_array().unwrap();
        let record = records.iter().find(|r| &r.member == support).unwrap_or_else(|| panic!("{at}: {support} certified"));
        for (index, pair) in components.iter().enumerate() {
            let observed = component(output, support, index);
            let family = if index < 3 { "support_force" } else { "support_moment" };
            let bound = record.groups.iter().find(|g| g.kind == family).unwrap().bound.clone().unwrap();
            let range = bound_pair(pair);
            assert!(within_exact(observed, range, bound), "{at}: {support} {} {observed:e} B={bound:e} exact={range:?}", SUPPORT_SLOT[index]);
            let scale = range.0.abs().max(range.1.abs());
            if scale > 0.0 {
                worst = worst.max((observed - (0.5 * range.0 + 0.5 * range.1)).abs() / scale);
            }
        }
    }
    worst
}

/// §11.4: every R-c reference in both modes, each spring's action against
/// its exact row within its certified bound.
#[test]
fn r_c_references_bind_and_match_in_both_modes() {
    for request in &R_C {
        let (wire, oracle) = bound_request(request);
        assert_eq!(oracle["part"], "R-c");
        leaves_match(&wire);
        assert_eq!(oracle["cases"].as_object().unwrap().len(), request.expected.len(), "{}", request.name);
        let springs = spring_ids(&wire);
        assert!(!springs.is_empty());
        for &(case, expect) in request.expected {
            for mode in MODES {
                let at = format!("{} {case} {mode:?}", request.name);
                let (output, records) = run_case(request, &wire, &oracle, case, expect, mode);
                let state = &oracle["cases"][case]["strict_states"][0];
                let worst = spring_actions_match(&output, &records, state, &springs, &wire, &at);
                println!("W16-NL {at}: spring actions max relative error {worst:.3e}");
            }
        }
    }
}

/// The hanger twin publishes the spring twin's displacements and spring
/// rows bit for bit: a variable spring hanger uses the same incremental law.
#[test]
fn the_hanger_twin_is_the_spring_twin_bit_for_bit() {
    let (spring, _) = bound_request(&R_C[0]);
    let (hanger, _) = bound_request(&R_C[2]);
    for (case, _) in C1 {
        for mode in MODES {
            let a = run_linear_static_preview_value_with_mode(single_case(&spring, case), mode).unwrap();
            let b = run_linear_static_preview_value_with_mode(single_case(&hanger, case), mode).unwrap();
            let rows = |o: &MechanicsEnvelope| {
                o.results
                    .iter()
                    .filter(|r| r.id.starts_with("result:disp:") || (r.kind == "support_reaction_component_v2" && r.entity_ref == "support:K"))
                    .map(|r| (r.id.clone(), r.value.to_bits()))
                    .collect::<Vec<_>>()
            };
            assert!(rows(&a).iter().filter(|(id, _)| !id.starts_with("result:disp:")).count() == 6, "{case} {mode:?}");
            assert_eq!(rows(&a), rows(&b), "{case} {mode:?}");
        }
    }
}

fn c2_case(case: &str) -> serde_json::Value {
    let (wire, _) = bound_request(&R_C[3]);
    single_case(&wire, case)
}

fn set_spring_motion(wire: &mut serde_json::Value, value: Option<f64>) {
    let loads = wire["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
    let index = loads.iter().position(|l| l["category"] == "imposed_displacement").unwrap();
    match value {
        Some(value) => loads[index]["magnitude"]["value"] = serde_json::json!(value),
        None => {
            loads.remove(index);
        }
    }
}

/// §11.1: an authored zero spring base, of either sign, is the fixed-base
/// case: every row, bit for bit, and every diagnostic.
#[test]
fn a_zero_spring_base_of_either_sign_is_the_fixed_base_case() {
    let mut plain = c2_case("load:SETTLE");
    set_spring_motion(&mut plain, None);
    for zero in [0.0, -0.0] {
        let mut moved = c2_case("load:SETTLE");
        set_spring_motion(&mut moved, Some(zero));
        assert_eq!(moved["model"]["load_cases"][0]["primitive_loads"][1]["magnitude"]["value"].as_f64().unwrap().to_bits(), zero.to_bits());
        for mode in MODES {
            let a = run_linear_static_preview_value_with_mode(plain.clone(), mode).unwrap();
            let b = run_linear_static_preview_value_with_mode(moved.clone(), mode).unwrap();
            assert_eq!(a.status.mechanics, "MECHANICS_SOLVED");
            assert_eq!(a.status.mechanics, b.status.mechanics);
            assert_eq!(a.results.len(), b.results.len(), "{zero:?} {mode:?}");
            for (x, y) in a.results.iter().zip(&b.results) {
                assert_eq!((x.id.as_str(), x.value.to_bits()), (y.id.as_str(), y.value.to_bits()), "{zero:?} {mode:?}");
            }
            assert_eq!(serde_json::to_value(&a.diagnostics).unwrap(), serde_json::to_value(&b.diagnostics).unwrap(), "{zero:?} {mode:?}");
        }
    }
}

/// C-4 (§11.4): the spring-base case against the same model with the nodal
/// force k·g (exact here): displacements and member rows bit for bit; the
/// spring actions, each exactly representable here, differ by exactly k·g.
#[test]
fn c4_a_spring_base_is_the_force_k_g_on_the_displacements() {
    let moved = c2_case("load:SETTLE");
    let model = &moved["model"];
    let k = model["supports"][1]["stiffness"]["value"]["value"].as_f64().unwrap();
    let g = model["load_cases"][0]["primitive_loads"][1]["magnitude"]["value"].as_f64().unwrap();
    let kg = k * g;
    assert_eq!(kg / g, k, "k·g is exact (g is a power of two)");
    let mut forced = moved.clone();
    let loads = forced["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
    loads[1] = serde_json::json!({"id": "load:SETTLE-KG", "category": "concentrated_force", "target": {"type": "node", "node": "node:T"},
        "direction": "global_z", "magnitude": {"value": kg, "unit": "N"}, "dimension": "force", "provenance": "w16 invented"});
    for mode in MODES {
        let a = run_linear_static_preview_value_with_mode(moved.clone(), mode).unwrap();
        let b = run_linear_static_preview_value_with_mode(forced.clone(), mode).unwrap();
        let rows = |o: &MechanicsEnvelope| {
            o.results
                .iter()
                .filter(|r| r.entity_ref != "support:K" && r.entity_ref != "support:ANCHOR" && r.metadata.as_ref().is_none_or(|m| m.location != "node"))
                .map(|r| (r.id.clone(), r.value.to_bits()))
                .collect::<Vec<_>>()
        };
        let disp = |o: &MechanicsEnvelope| o.results.iter().filter(|r| r.id.starts_with("result:disp:")).map(|r| r.value.to_bits()).collect::<Vec<_>>();
        assert_eq!(disp(&a), disp(&b), "{mode:?}");
        assert!(!rows(&a).is_empty());
        assert_eq!(rows(&a), rows(&b), "{mode:?}");
        let (spring, force) = (component(&a, "support:K", 2), component(&b, "support:K", 2));
        assert_eq!(spring - force, kg, "{mode:?}: -k(u - g) - (-k u) = k g");
        assert_eq!((spring - force) - kg, 0.0);
    }
}

/// §11.4: closure mutants on the spring-base primitive. A missing operand, a
/// wrong factor and a wrong DOF binding each fail the K-D5 closure.
#[test]
fn spring_base_closure_mutants_fail_closed() {
    use crate::certified_loads::supplied_mutant;
    use open_pipe_stress_frame_kernel::structural::{LoadFormula, LoadPrimitive};
    let wire = c2_case("load:SETTLE");
    type Edit = fn(&mut Vec<LoadPrimitive>) -> bool;
    let mutants: [(&str, Edit); 3] = [
        ("missing", |supplied| {
            let before = supplied.len();
            supplied.retain(|p| p.identity.family != "spring_base_motion");
            supplied.len() != before
        }),
        ("factor", |supplied| {
            let mut hit = false;
            for p in supplied.iter_mut().filter(|p| p.identity.family == "spring_base_motion") {
                if let LoadFormula::NodalProduct { factor, .. } = &mut p.formula {
                    *factor *= 2.0;
                    hit = true;
                }
            }
            hit
        }),
        ("binding", |supplied| {
            let mut hit = false;
            for p in supplied.iter_mut().filter(|p| p.identity.family == "spring_base_motion") {
                if let LoadFormula::NodalProduct { global_dof, .. } = &mut p.formula {
                    *global_dof -= 1;
                    hit = true;
                }
            }
            hit
        }),
    ];
    for mode in MODES {
        let plain = serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap()).unwrap();
        assert_eq!(plain["numerical_quality"]["status"], "checks_passed", "{mode:?}");
        for (name, edit) in mutants {
            supplied_mutant::arm(Box::new(move |_, supplied: &mut Vec<LoadPrimitive>| edit(supplied)));
            let mutated = serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap()).unwrap();
            assert!(supplied_mutant::disarm().is_some(), "{name} {mode:?}: the mutant reached the spring-base primitive");
            assert_ne!(mutated["numerical_quality"]["status"], "checks_passed", "{name} {mode:?}");
        }
    }
}

/// §11.3 R-c admission: what stays refused, the per-spring duplicate key,
/// parallel springs moving independently, and combinations.
#[test]
fn r_c_admission_and_combinations() {
    // A spring record's non-axis DOF.
    let mut other = c2_case("load:SETTLE");
    let load = &mut other["model"]["load_cases"][0]["primitive_loads"][1];
    load["target"]["dof"] = serde_json::json!("UY");
    load["direction"] = serde_json::json!("UY");
    refused_whole(&other, "IMPOSED_DISPLACEMENT_DOF_UNRESTRAINED", "a spring's non-axis DOF");
    // Two values for one spring base, equal.
    let mut duplicate = c2_case("load:SETTLE");
    let loads = duplicate["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
    let mut again = loads[1].clone();
    again["id"] = serde_json::json!("load:SETTLE-G2");
    loads.push(again);
    refused_whole(&duplicate, "IMPOSED_DISPLACEMENT_DOF_DUPLICATE", "two values for one spring base");
    // A constant-effort device stays refused (stage 1's test; and with a nonlinear field, R-d's).
    // Combinations: superposed in a linear model, withheld in a nonlinear one.
    let (c2, _) = bound_request(&R_C[3]);
    let (c5, _) = bound_request(&R_C[6]);
    for (wire, withheld) in [(c2, false), (c5, true)] {
        let mut combined = wire.clone();
        combined["model"]["load_cases"].as_array_mut().unwrap().truncate(2);
        let ids = combined["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["id"].clone()).collect::<Vec<_>>();
        combined["model"]["combinations"] = serde_json::json!([{"id": "comb:pair", "basis": "mechanics",
            "terms": [{"load_case": ids[0], "factor": 1.0}, {"load_case": ids[1], "factor": 1.0}], "provenance": "w16 invented"}]);
        for mode in MODES {
            let output = serde_json::to_value(run_linear_static_preview_value_with_mode(combined.clone(), mode).unwrap()).unwrap();
            assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
            let gates = output["contract_evidence"]["combination_gates"].as_array().unwrap();
            assert_eq!(gates.len(), 1);
            assert_eq!(gates[0]["withheld"], withheld, "{mode:?}: {gates:?}");
        }
    }
}
