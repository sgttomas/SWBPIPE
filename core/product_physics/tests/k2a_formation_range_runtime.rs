//! K2a product-reach tests (T3 D1 revision 5a.2 section 4.7 step 1; ROOT's
//! rulings "K2a: product reach and the availability trade-off") through both
//! public entries, the captured `run_linear_static_preview_value_with_mode`
//! and the historical typed `run_linear_static_preview_with_mode`, in both
//! solver modes.
//!
//! Capture admits any finite value whose integral magnitude is at most
//! 2^53 - 1. Model validation requires E, G, OD, wall and the derived A, I
//! and J only to be positive and finite (with 2 * wall < OD), and a member to
//! be longer than 1e-12 m. So admissible, physically absurd section and
//! material values reach the kernel's `local_stiffness`. Before K2a, an
//! underflowed intermediate there became an exact 0, or a subnormal with few
//! bits, and was published through the solve. All inputs are invented, and
//! every one is a normal binary64 number.
//!
//! F1b (ROOT's rulings on I13's plan, Q10 approved): on a linear invocation
//! K2a's formation refusal is now the step-1 trigger of W2, K2b's force
//! scaling reached through its orchestrator. The linear variants' expectations
//! are restated: published at the orchestrator's b, or refused by W2's named
//! template with K2a's name kept as the trigger. The nonlinear (`open_gap`)
//! variants are unchanged: W2 never engages there.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::f64::consts::PI;

const PROV: &str = "invented_t3_k2a_product_reach_input_no_library_data";

/// One member N0 -> N1 along x. N0 is anchored. N1 is anchored in every DOF
/// but `free`, which carries the load and, when given, a ground spring.
struct Reach {
    id: &'static str,
    length: f64,
    od: f64,
    wall: f64,
    e: f64,
    g: f64,
    free: &'static str,
    spring: Option<f64>,
    load: f64,
    /// An open gap support on N1's free DOF that never closes: it makes the
    /// invocation nonlinear, so K-D5's formation check is not selected.
    open_gap: bool,
}

fn request(r: &Reach) -> Value {
    let all = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
    let anchored: Vec<&str> = all.iter().copied().filter(|d| *d != r.free).collect();
    let mut supports = vec![
        json!({"id": "rigid:N0", "node": "N0", "restraints": all, "family": "anchor", "provenance": PROV}),
        json!({"id": "rigid:N1", "node": "N1", "restraints": anchored, "family": "anchor", "provenance": PROV}),
    ];
    let rotational = r.free.starts_with('R');
    if let Some(k) = r.spring {
        let unit = if rotational { "N*m/rad" } else { "N/m" };
        supports.push(
            json!({"id": "spring:N1:0", "node": "N1", "family": "spring", "restraints": [r.free],
            "stiffness": {"dof": r.free, "value": {"value": k, "unit": unit}}, "provenance": PROV}),
        );
    }
    if r.open_gap {
        supports.push(json!({"id": "support:k2a-open-gap", "node": "N1", "family": "nonlinear", "restraints": [],
            "nonlinear": {"behavior": "gap", "dof": r.free, "initial_state": "inactive",
                          "closes_when": "positive_displacement", "gap": {"value": 1000.0, "unit": "mm"}},
            "provenance": PROV}));
    }
    let load = if rotational {
        json!({"id": "load:0", "category": "concentrated_moment", "target": {"type": "node", "node": "N1"},
               "direction": r.free, "magnitude": {"value": r.load, "unit": "N*m"}, "dimension": "moment",
               "provenance": PROV})
    } else {
        let direction = format!("global_{}", r.free[1..].to_lowercase());
        json!({"id": "load:0", "category": "concentrated_force", "target": {"type": "node", "node": "N1"},
               "direction": direction, "magnitude": {"value": r.load, "unit": "N"}, "dimension": "force",
               "provenance": PROV})
    };
    json!({
        "model": {
            "schema_version": "0.1.0",
            "document_kind": "openpipestress.product_preview.model",
            "analysis_status": {
                "mechanics": "ready_for_preview_diagnostics",
                "rule_check": "not_performed_user_rule_inputs_missing",
                "professional_acceptance": "not_provided"
            },
            "project": {
                "id": format!("invented:t3-k2a:{}", r.id),
                "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}
            },
            "nodes": [
                {"id": "N0", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
                {"id": "N1", "position": {"x": r.length, "y": 0.0, "z": 0.0}, "provenance": PROV}
            ],
            "pipe_segments": [{
                "id": "M1", "from": "N0", "to": "N1", "material": "mat:K2A",
                "y_reference": {"x": 0, "y": 1, "z": 0},
                "section": {"outside_diameter": {"value": r.od, "unit": "m"}, "wall_thickness": {"value": r.wall, "unit": "m"}},
                "provenance": PROV
            }],
            "materials": [{
                "id": "mat:K2A",
                "elastic_modulus": {"value": r.e, "unit": "Pa"},
                "shear_modulus": {"value": r.g, "unit": "Pa"},
                "provenance": PROV
            }],
            "supports": supports,
            "load_cases": [{
                "id": "case", "label": r.id, "kind": "primitive_user_load",
                "primitive_loads": [load],
                "provenance": PROV
            }],
            "combinations": []
        },
        "materials": []
    })
}

/// The section as PP `derive_pipe_section` forms it: (A, I, J).
fn product_section(od: f64, wall: f64) -> (f64, f64, f64) {
    let id = od - 2.0 * wall;
    let area = PI * (od.powi(2) - id.powi(2)) / 4.0;
    let second_moment = PI * (od.powi(4) - id.powi(4)) / 64.0;
    (area, second_moment, 2.0 * second_moment)
}

fn both_entries(value: &Value, mode: PreviewSolverMode) -> [(&'static str, MechanicsEnvelope); 2] {
    let captured = run_linear_static_preview_value_with_mode(value.clone(), mode).unwrap();
    let typed_request: LinearStaticPreviewRequest = serde_json::from_value(value.clone()).unwrap();
    let typed = run_linear_static_preview_with_mode(typed_request, mode);
    [("captured", captured), ("typed", typed)]
}

/// On both entries and in both modes: no result is published, and a blocking
/// diagnostic carries K2a's named reason.
fn assert_refused_by_name(r: &Reach, name: &str) {
    let expected =
        format!("range: stiffness formation outside the binary64 normal range at {name}");
    let value = request(r);
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for (entry, envelope) in both_entries(&value, mode) {
            let ctx = format!("{} {entry} {mode:?}", r.id);
            assert!(envelope.results.is_empty(), "{ctx}: no result is published");
            let named: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.severity == "blocking" && d.message.contains(&expected))
                .collect();
            assert_eq!(named.len(), 1, "{ctx}: {:?}", envelope.diagnostics);
            assert_eq!(named[0].code, "SOLVER_SYSTEM_BLOCKED", "{ctx}");
        }
    }
}

/// F1b (Q10): on both entries and in both modes, W2 refuses the case: no
/// result is published, and the only blocking diagnostic is the case's
/// integrity diagnostic, `NUMERICAL_INTEGRITY_UNRESOLVED`, with the
/// orchestrator's `reason`, K2a's name kept as the step-1 trigger, and no b
/// (ROOT OQ1, c2: a step-4 refusal does not carry b).
fn assert_w2_refused(r: &Reach, reason: &str, name: &str) {
    let expected = format!(
        "Load case case: {reason}; range_scaling: attempted; step1_trigger=Formation(NumericalRange {{ name: {name:?} }}); global_dof_map="
    );
    let value = request(r);
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for (entry, envelope) in both_entries(&value, mode) {
            let ctx = format!("{} {entry} {mode:?}", r.id);
            assert!(envelope.results.is_empty(), "{ctx}: no result is published");
            let blocking: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.severity == "blocking")
                .collect();
            assert_eq!(blocking.len(), 1, "{ctx}: {:?}", envelope.diagnostics);
            assert_eq!(
                blocking[0].id, "diagnostic:numerical-integrity:case",
                "{ctx}"
            );
            assert_eq!(blocking[0].code, "NUMERICAL_INTEGRITY_UNRESOLVED", "{ctx}");
            assert!(
                blocking[0].message.starts_with(&expected),
                "{ctx}: {}",
                blocking[0].message
            );
            assert!(
                !blocking[0].message.contains("force_scale_exponent"),
                "{ctx}"
            );
        }
    }
}

/// F1b (Q10): on both entries and in both modes, W2 publishes the case at
/// b = `b`: N1's `free` displacement (mm) is within the unchanged 1e-9
/// criterion of the exact reference `reference_m` (m); nothing blocks; the
/// case's integrity diagnostic is `NUMERICAL_INTEGRITY_SENSITIVE` (S11-G's
/// R-b' fails closed at b != 0, ROOT Q5(a)) and carries the `range_scaling:`
/// evidence line exactly once, before R-b''s text.
fn assert_w2_published(r: &Reach, b: i32, reference_m: f64) {
    let line = format!("range_scaling: force_scale_exponent={b}; basis=exact power-of-two");
    let kind = format!("global_nodal_displacement_{}", r.free[1..].to_lowercase());
    let expected_mm = reference_m * 1000.0;
    let value = request(r);
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for (entry, envelope) in both_entries(&value, mode) {
            let ctx = format!("{} {entry} {mode:?}", r.id);
            assert!(
                envelope
                    .diagnostics
                    .iter()
                    .all(|d| d.severity != "blocking"),
                "{ctx}: {:?}",
                envelope.diagnostics
            );
            let rows: Vec<_> = envelope
                .results
                .iter()
                .filter(|row| row.entity_ref == "N1" && row.kind == kind)
                .collect();
            assert_eq!(rows.len(), 1, "{ctx}");
            assert_eq!(rows[0].unit, "mm", "{ctx}");
            let error = ((rows[0].value - expected_mm) / expected_mm).abs();
            assert!(
                error <= 1e-9,
                "{ctx}: {} mm against {expected_mm} mm",
                rows[0].value
            );
            let integrity: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.id == "diagnostic:numerical-integrity:case")
                .collect();
            assert_eq!(integrity.len(), 1, "{ctx}");
            assert_eq!(integrity[0].code, "NUMERICAL_INTEGRITY_SENSITIVE", "{ctx}");
            let message = &integrity[0].message;
            assert_eq!(message.matches("range_scaling:").count(), 1, "{ctx}");
            let at = message
                .find(&line)
                .unwrap_or_else(|| panic!("{ctx}: {message}"));
            let guard = message.find("S11-G recovery guard (R-b')").expect("R-b'");
            assert!(at < guard, "{ctx}: the line precedes R-b''s text");
        }
    }
}

const SPRING_CARRIED: Reach = Reach {
    id: "product-reach-torsion-underflow-spring-carried",
    length: 2.0,
    od: 1.0e-6,
    wall: 1.0e-7,
    e: 2.0e11,
    g: 1.0e-300,
    free: "RX",
    spring: Some(1.0),
    load: 1.0,
    open_gap: false,
};

#[test]
fn k2a_product_reach_spring_carried_torsion_underflow_is_refused_on_both_entries() {
    // ROOT: kept as K2a's accepted interim, an availability change on
    // admissible but physically absurd inputs where main's value was
    // accurate (K2b's scaling should restore it). N1's only free DOF is RX,
    // carried by a 1 N*m/rad spring. Paths differ: on main's unchecked
    // formation G*J = 5.8e-326 rounds to exactly 0, so GJ/L = 0 and the case
    // solved on the spring alone (accurate, since GJ/L is truly ~3e-326).
    // F1b (Q10): W2 does not restore it. The one evaluation at the chosen b
    // also leaves the normal range (step 4), so W2 refuses the case by its
    // template and keeps K2a's name as the step-1 trigger.
    let r = &SPRING_CARRIED;
    let (area, i, j) = product_section(r.od, r.wall);
    assert!(r.g.is_normal() && j.is_normal() && i.is_normal());
    assert_eq!(r.g * j, 0.0);
    assert_eq!(r.g * j / r.length, 0.0);
    assert!((r.e * area / r.length).is_normal());
    assert!((12.0 * r.e * i / (r.length * r.length * r.length)).is_normal());
    assert_w2_refused(
        r,
        "range: scaled evaluation outside normal range",
        "GJ/L: G*J",
    );
}

/// A 2^-20 m member (above the 1e-12 m axis tolerance), OD 3e-8 m, wall
/// 3e-9 m, E = 1.3e-292 Pa, G = 1e-200 Pa, a 1e-307 N load on N1's only free
/// DOF, UY. (12*E)*Iz is subnormal (3.5e-323, a few bits): main formed
/// 12EIz/L^3 normal but 5.56 % wrong, and 6EI/L^2, 4EI/L and 2EI/L subnormal.
/// E*A, G*J and every intermediate before (12*E)*Iy are normal.
const PARTIAL_UNDERFLOW: Reach = Reach {
    id: "product-reach-partial-underflow",
    length: 9.5367431640625e-07,
    od: 3.0e-8,
    wall: 3.0e-9,
    e: 1.3e-292,
    g: 1.0e-200,
    free: "UY",
    spring: None,
    load: 1.0e-307,
    open_gap: false,
};

/// The same model with an open gap support on N1 UY that never closes: the
/// invocation is nonlinear, so on main K-D5's formation check is not selected
/// and the active-set loop's binary64 solve runs instead.
const PARTIAL_UNDERFLOW_NONLINEAR: Reach = Reach {
    id: "product-reach-partial-underflow-open-gap",
    open_gap: true,
    ..PARTIAL_UNDERFLOW
};

/// Main's local stiffness for a member, operation for operation as FK
/// `local_stiffness` formed it before K2a (without its final finiteness
/// check), with Iy = Iz = I and J the product's section values.
fn unchecked_local(e: f64, g: f64, area: f64, i: f64, j: f64, length: f64) -> [[f64; 12]; 12] {
    let l2 = length * length;
    let l3 = l2 * length;
    let axial = e * area / length;
    let torsion = g * j / length;
    let (k12, k6, k4, k2) = (
        12.0 * e * i / l3,
        6.0 * e * i / l2,
        4.0 * e * i / length,
        2.0 * e * i / length,
    );
    let mut k = [[0.0; 12]; 12];
    for (a, b, v) in [(0, 6, axial), (3, 9, torsion)] {
        k[a][a] = v;
        k[b][b] = v;
        k[a][b] = -v;
        k[b][a] = -v;
    }
    // (UY, RZ) with +k6 and (UZ, RY) with -k6, as FK add_bending_z/_y.
    for (v, r, s) in [(1usize, 5usize, 1.0), (2, 4, -1.0)] {
        let idx = [v, r, v + 6, r + 6];
        let t = [
            [k12, s * k6, -k12, s * k6],
            [s * k6, k4, -s * k6, k2],
            [-k12, -s * k6, k12, -s * k6],
            [s * k6, k2, -s * k6, k4],
        ];
        for a in 0..4 {
            for b in 0..4 {
                k[idx[a]][idx[b]] = k[idx[a]][idx[b]] + t[a][b];
            }
        }
    }
    k
}

#[test]
fn k2a_product_reach_partial_underflow_refused_unresolved_on_main_is_refused_by_name() {
    // ROOT (addendum 1): main does not publish this case. Its M03 structural
    // integrity path refuses it as NUMERICAL_INTEGRITY_UNRESOLVED with
    // Range("product overflow or underflow"), on both variants, on both
    // entries and in both modes (run record product_reach/, main's
    // envelopes), because its formed 6EI/L^2, 4EI/L and 2EI/L are subnormal.
    // K2a refuses it earlier, at formation, by name. K2a corrects no
    // published value here.
    // F1b (Q10): the linear variant is published at b = 898, within 1e-9 of
    // K2b's exact reference u_y = 0x3f63671db398fdda m; the nonlinear
    // variant is unchanged (W2 never engages there).
    use open_pipe_stress_frame_kernel::structural::{transform_roundoff, StructuralError};
    for r in [&PARTIAL_UNDERFLOW, &PARTIAL_UNDERFLOW_NONLINEAR] {
        let (area, i, j) = product_section(r.od, r.wall);
        assert!(
            (r.e * area).is_normal() && (r.g * j).is_normal(),
            "{}",
            r.id
        );
        // Main's formation: (12*E)*I is subnormal; 12EI/L^3 is normal and
        // wrong; the other bending coefficients are subnormal.
        let l3 = r.length * r.length * r.length;
        let intermediate = 12.0 * r.e * i;
        assert!(intermediate > 0.0 && !intermediate.is_normal(), "{}", r.id);
        let k12 = 12.0 * r.e * i / l3;
        let e_scaled = r.e * 2f64.powi(600);
        let reference = 12.0 * e_scaled * i / l3 * 2f64.powi(-600);
        assert!(k12.is_normal() && reference.is_normal());
        let error = ((k12 - reference) / reference).abs();
        assert!(
            error > 0.055 && error < 0.056,
            "{}: 12EI/L^3 error {error}",
            r.id
        );
        let local = unchecked_local(r.e, r.g, area, i, j, r.length);
        for (row, col) in [(1, 5), (5, 5), (5, 11)] {
            let v = local[row][col];
            assert!(v != 0.0 && !v.is_normal(), "{}: [{row}][{col}] {v}", r.id);
        }
        // The precondition pins main's refusal: M03's checked transform of
        // main's element (identity rotation for this x member with y
        // reference +y) refuses with the same Range as main's envelopes.
        let mut identity = [[0.0; 12]; 12];
        for (d, row) in identity.iter_mut().enumerate() {
            row[d] = 1.0;
        }
        assert_eq!(
            transform_roundoff(&local, &identity).map(|_| ()),
            Err(StructuralError::Range("product overflow or underflow")),
            "{}",
            r.id
        );
        // K2a: refused at formation, by name, on both entries, both modes.
        // F1b (Q10): the linear variant is published by W2 instead.
        if r.open_gap {
            assert_refused_by_name(r, "12EIy/L^3: (12*E)*Iy");
        } else {
            assert_w2_published(r, 898, f64::from_bits(0x3f63671db398fdda));
        }
    }
}

/// Main's linear route for a one-member model at kernel level: main's
/// (pre-K2a) element matrix, the ground spring on N1's free DOF, and K-D5's
/// formation check, which re-forms the frame from its primitives. The
/// element is an x member with y reference +y, so its rotation is the
/// identity and N0/N1 own DOFs 0..6 and 6..12.
fn main_linear_route_at_kernel_level(
    r: &Reach,
    free: usize,
) -> open_pipe_stress_frame_kernel::structural::StructuralSolution {
    use open_pipe_stress_frame_kernel::structural::{
        solve_formation_checked_structural_dense, transform_roundoff, FormationSource,
        StructuralSystem, SymmetryEvidence,
    };
    use open_pipe_stress_frame_kernel::{FrameElement, FrameNode, FrameSection};
    let (area, i, j) = product_section(r.od, r.wall);
    let local = unchecked_local(r.e, r.g, area, i, j, r.length);
    let mut identity = [[0.0; 12]; 12];
    for (d, row) in identity.iter_mut().enumerate() {
        row[d] = 1.0;
    }
    let roundoff = transform_roundoff(&local, &identity).expect("main's M03 accepts the element");
    let absolute: Vec<Vec<f64>> = roundoff
        .absolute_roundoff
        .iter()
        .map(|x| x.to_vec())
        .collect();
    let counts: Vec<Vec<usize>> = roundoff
        .operation_counts
        .iter()
        .map(|x| x.to_vec())
        .collect();
    let mut k: Vec<Vec<f64>> = local.iter().map(|x| x.to_vec()).collect();
    let springs: Vec<(usize, f64)> = r.spring.map(|s| (free, s)).into_iter().collect();
    for &(d, s) in &springs {
        k[d][d] = k[d][d] + s;
    }
    let mut f = vec![0.0; 12];
    f[free] = r.load;
    let prescribed: Vec<(usize, f64)> = (0..12).filter(|&d| d != free).map(|d| (d, 0.0)).collect();
    let frame = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [r.length, 0.0, 0.0]).unwrap(),
        FrameSection::new(r.e, r.g, area, i, i, j).unwrap(),
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    let source = FormationSource {
        node_count: 2,
        frames: vec![frame],
        springs,
        ..FormationSource::default()
    };
    let system = StructuralSystem {
        stiffness: &k,
        force: &f,
        free_dofs: &[free],
        prescribed: &prescribed,
        contributions: None,
        symmetry: Some(SymmetryEvidence {
            absolute_roundoff: &absolute,
            operation_counts: &counts,
            basis: roundoff.basis,
        }),
    };
    solve_formation_checked_structural_dense(&system.with_formation_source(&source)).unwrap()
}

/// The true 12EIz/L^3 of a member: E scaled by 2^600 (every operand and the
/// result normal), then scaled back exactly; accurate to a few ulps.
fn true_bend_12(r: &Reach) -> f64 {
    let (_, i, _) = product_section(r.od, r.wall);
    let l3 = r.length * r.length * r.length;
    let value = 12.0 * (r.e * 2f64.powi(600)) * i / l3 * 2f64.powi(-600);
    assert!(value.is_normal());
    value
}

/// An exact zero hidden by the 1/L^3 lift (I6's correction of the stop
/// report): a 2^-39 m member (above the 1e-12 m axis tolerance), OD 1e-11 m,
/// wall 1e-12 m, E = 6.4e-280 Pa, G = 1e-100 Pa. (12*E)*I = 2.23e-324 rounds
/// to exactly 0, so main's 12EIz/L^3 is 0, while its true value is 3.70e-289
/// (2^-958.2), above M03's acceptance floor. E*A is normal (EA/L = 2^-963.4)
/// and GJ/L is large. N1's only free DOF, UY, has a 3.7e-289 N/m ground
/// spring (about the true 12EIz/L^3) and a 9.25e-290 N load.
const EXACT_ZERO: Reach = Reach {
    id: "product-reach-exact-zero-12EI",
    length: 1.8189894035458565e-12,
    od: 1.0e-11,
    wall: 1.0e-12,
    e: 6.4e-280,
    g: 1.0e-100,
    free: "UY",
    spring: Some(3.7e-289),
    load: 9.25e-290,
    open_gap: false,
};

/// The LEF-small pattern on the product route: the same member with
/// E = 9.6e-280 Pa and no spring. (12*E)*I rounds to the least subnormal
/// (5e-324) while (6*E)*I, (4*E)*I and (2*E)*I round to exactly 0: main's
/// 12EIz/L^3 = 8.21e-289 is normal and 48 % wrong (true 5.55e-289), and its
/// siblings are 0. A 2.05e-289 N load on N1 UY, carried by 12EIz/L^3 alone.
const LEAST_SUBNORMAL: Reach = Reach {
    id: "product-reach-least-subnormal-12EI",
    e: 9.6e-280,
    spring: None,
    load: 2.05e-289,
    ..EXACT_ZERO
};

#[test]
fn k2a_product_reach_wrong_12ei_accepted_by_m03_and_flagged_only_downstream_is_refused_by_name() {
    // On main, M03 accepts both elements (every nonzero entry is above its
    // transform_roundoff floor, about 2^-974.6) and main publishes u_y wrong:
    // 250 mm against 125.03 mm (exact zero) and 249.72 mm against 369.55 mm
    // (least subnormal). It publishes them only as untrusted values:
    // NUMERICAL_INTEGRITY_SENSITIVE on the linear route (K-D5's re-formation)
    // and unresolved on the nonlinear route (the strict-gap proof's
    // exact-radix range), both entries and both modes (run record
    // product_reach/main_zero_probe.jsonl and main_lef_probe.jsonl). K2a
    // refuses the formation, by name.
    // F1b (Q10): the linear variants are published at b = 734, within 1e-9
    // of the exact references u_y = 0x3fc001034445ee29 m (exact zero) and
    // 0x3fd7a6bdbbfabce5 m (least subnormal); the nonlinear variants are
    // unchanged.
    use open_pipe_stress_frame_kernel::structural::{FormationCheckReason, SolveQuality};
    assert_eq!(EXACT_ZERO.length, 2f64.powi(-39));
    for (r, low, high, twelve_e_i, reference) in [
        (&EXACT_ZERO, 0.99, 1.0, 0.0, 0x3fc001034445ee29_u64),
        (
            &LEAST_SUBNORMAL,
            0.32,
            0.33,
            f64::from_bits(1),
            0x3fd7a6bdbbfabce5,
        ),
    ] {
        let (area, i, j) = product_section(r.od, r.wall);
        assert!((r.e * area / r.length).is_normal() && (r.g * j / r.length).is_normal());
        // The roundings: (12*E)*I is exactly 0 (reach_zero) or the least
        // subnormal (reach_lef), and (6*E)*I, (4*E)*I, (2*E)*I are exactly 0.
        // Main's evaluation order is (k*E)*I. The product before rounding,
        // formed with E scaled by 2^600 (the same k*E mantissa, every value
        // normal, relative error below 2^-52), is compared with the rounding
        // boundaries 2^-1075 (below it: 0) and 2^-1074 (the least subnormal),
        // scaled the same way; every margin is at least 10 %.
        let half_least = 2f64.powi(-475); // 2^-1075 scaled by 2^600
        let before_rounding = |k: f64| (k * (r.e * 2f64.powi(600))) * i;
        assert_eq!(12.0 * r.e * i, twelve_e_i, "{}", r.id);
        let twelve = before_rounding(12.0);
        if twelve_e_i == 0.0 {
            assert!(twelve < 0.95 * half_least, "{}: {twelve:e}", r.id);
        } else {
            assert!(
                twelve > 1.25 * half_least && twelve < 0.75 * 2.0 * half_least,
                "{}: {twelve:e}",
                r.id
            );
        }
        for k in [6.0, 4.0, 2.0] {
            assert_eq!(k * r.e * i, 0.0, "{}: ({k}*E)*I", r.id);
            assert!(
                before_rounding(k) < 0.75 * half_least,
                "{}: ({k}*E)*I",
                r.id
            );
        }
        // Main's formation of 12EIz/L^3 and its siblings.
        let local = unchecked_local(r.e, r.g, area, i, j, r.length);
        let formed = local[1][1];
        for (row, col) in [(1, 5), (5, 5), (5, 11)] {
            assert_eq!(local[row][col], 0.0, "{}: [{row}][{col}]", r.id);
        }
        let true_k = true_bend_12(r);
        let spring = r.spring.unwrap_or(0.0);
        let u_main = r.load / (formed + spring);
        let u_true = r.load / (true_k + spring);
        let error = ((u_main - u_true) / u_true).abs();
        assert!(
            error > low && error < high,
            "{}: main's u_y error {error}",
            r.id
        );
        // Main's linear route at kernel level: M03 accepts, the solve gives
        // main's wrong u, and K-D5 demotes it (Sensitive, estimate).
        let solution = main_linear_route_at_kernel_level(r, 7);
        let solved = solution.displacements[7];
        assert!(
            ((solved - u_main) / u_main).abs() < 1e-12,
            "{}: {solved} {u_main}",
            r.id
        );
        assert_eq!(solution.report.quality, SolveQuality::Sensitive, "{}", r.id);
        assert_eq!(
            solution.formation_check.map(|c| c.reason),
            Some(FormationCheckReason::Estimate),
            "{}",
            r.id
        );
        // K2a: refused at formation, by name, on the linear and the
        // nonlinear variant, both entries and both modes.
        // F1b (Q10): the nonlinear variant only; W2 publishes the linear one,
        // and u_y is the true value, not main's.
        let nonlinear = Reach {
            open_gap: true,
            ..*r
        };
        assert_refused_by_name(&nonlinear, "12EIy/L^3: (12*E)*Iy");
        let reference = f64::from_bits(reference);
        assert!(
            ((reference - u_true) / u_true).abs() < 1e-12,
            "{}: the reference is the true u_y ({reference} m, {u_true} m)",
            r.id
        );
        assert_w2_published(r, 734, reference);
    }
}
