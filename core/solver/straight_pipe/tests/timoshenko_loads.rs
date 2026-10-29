//! T4-U8 (D-6): the Timoshenko straight pipe's load equivalents and
//! recovery (`DEL-05-03` Design `section-bases-and-timoshenko.md` §§3.2,
//! 3.4, 3.5, §8).
//!
//! Expected values are SHEAR_REFERENCE's frozen §5 values (CONTRACT.md S-A
//! distal half and S-E interior spring, `reference_arithmetic.stdout.txt`,
//! f2211ecef6), quoted verbatim as rationals. None is computed through the
//! production element. The S-E system is assembled with FK.
use open_pipe_stress_frame_kernel::load_ledger::Formation;
use open_pipe_stress_frame_kernel::structural::{
    certify_straight_recovery, CertifiedMemberLoads, MemberBoundInputs,
};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, reduce_system, solve_dense, BeamTheory, FrameElement, FrameNode,
    DOF_PER_NODE, RZ, UY, UZ, RY,
};
use open_pipe_stress_straight_pipe::{
    LocalLoadDirection, PipeEnd, PointLocalForce, SpannedGlobalUniformLoad,
    SpannedUniformLocalLoad, StraightPipeElement, StraightPipeError,
    StraightPipeSectionProperties, UniformLoadSpan,
};

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tol * expected.abs().max(1.0),
        "{what}: {actual:e} vs {expected:e}"
    );
}

/// EI = 1 N·m², Ks = G·A_s = 12 N (S-A, S-E); E = G = A = J = 1.
fn timoshenko() -> StraightPipeSectionProperties {
    StraightPipeSectionProperties::timoshenko(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 12.0, None).unwrap()
}
fn euler_bernoulli() -> StraightPipeSectionProperties {
    StraightPipeSectionProperties::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, None).unwrap()
}

fn pipe(
    id: &str,
    i: usize,
    j: usize,
    xi: f64,
    xj: f64,
    section: StraightPipeSectionProperties,
) -> StraightPipeElement {
    StraightPipeElement::new(
        id,
        FrameNode::new(i, [xi, 0.0, 0.0]).unwrap(),
        FrameNode::new(j, [xj, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap()
}

/// Displacements of a model with node 0 clamped and ground springs.
fn solve(frames: &[FrameElement], nodes: usize, springs: &[(usize, f64)], force: &[f64]) -> Vec<f64> {
    let mut k = assemble_global_stiffness(nodes, frames).unwrap();
    for &(dof, stiffness) in springs {
        k[dof][dof] += stiffness;
    }
    let fixed: Vec<usize> = (0..DOF_PER_NODE).collect();
    let reduced = reduce_system(&k, force, &fixed).unwrap();
    let free = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
    let mut u = vec![0.0; nodes * DOF_PER_NODE];
    for (value, &dof) in free.iter().zip(&reduced.free_dofs) {
        u[dof] = *value;
    }
    u
}

fn distal_half(direction: LocalLoadDirection) -> SpannedUniformLocalLoad {
    SpannedUniformLocalLoad::new(direction, 1.0, UniformLoadSpan::new(0.5, 1.0).unwrap()).unwrap()
}

/// SHEAR_REFERENCE §5 S-A, +1 N/m over the distal half at phi = 1:
/// feq = [7/64, 13/384, 25/64, -19/384], tip v 53/384, theta 7/48, root
/// Fy -1/2, Mz -3/8. The stale Euler–Bernoulli vector
/// [3/32, 5/192, 13/32, -11/192] with Timoshenko K gives v 9/64 = 54/384
/// with the same theta (the killed mutant), so rotation and balance alone
/// cannot see it; the formed S11-G record bounds the Timoshenko vector.
#[test]
fn timoshenko_distal_half_load_kills_the_stale_vector() {
    let member = pipe("T", 0, 1, 0.0, 1.0, timoshenko());
    let stale = pipe("EB", 0, 1, 0.0, 1.0, euler_bernoulli());
    let load = [distal_half(LocalLoadDirection::Y)];
    let feq = member.equivalent_nodal_loads_with_spans(&load, &[]).unwrap();
    let eb = stale.equivalent_nodal_loads_with_spans(&load, &[]).unwrap();
    let j = DOF_PER_NODE;
    for (value, expected, what) in [
        (feq[UY], 7.0 / 64.0, "V_i"),
        (feq[RZ], 13.0 / 384.0, "M_i"),
        (feq[j + UY], 25.0 / 64.0, "V_j"),
        (feq[j + RZ], -19.0 / 384.0, "M_j"),
        (eb[UY], 3.0 / 32.0, "stale V_i"),
        (eb[RZ], 5.0 / 192.0, "stale M_i"),
        (eb[j + UY], 13.0 / 32.0, "stale V_j"),
        (eb[j + RZ], -11.0 / 192.0, "stale M_j"),
    ] {
        close(value, expected, 1e-15, what);
    }
    // Both balance total force (wL/2) and moment about i (3wL²/8).
    for v in [feq, eb] {
        close(v[UY] + v[j + UY], 0.5, 1e-15, "force balance");
        close(v[RZ] + 1.0 * v[j + UY] + v[j + RZ], 3.0 / 8.0, 1e-15, "moment balance");
    }

    let frames = [member.frame_element().unwrap()];
    let solve_with = |vector: [f64; 12]| solve(&frames, 2, &[], &vector);
    let u = solve_with(feq);
    close(u[j + UY], 53.0 / 384.0, 1e-15, "tip v");
    close(u[j + RZ], 7.0 / 48.0, 1e-15, "tip theta");
    // The loaded end-force recovery uses the same Timoshenko terms.
    let root = member
        .recover_end_resultants_from_global_model_with_spans_and_axial_effects(&u, PipeEnd::I, &load, &[], &[])
        .unwrap();
    close(root.shear_force_y, -0.5, 1e-15, "root Fy");
    close(root.bending_moment_z, -3.0 / 8.0, 1e-15, "root Mz");
    let tip = member
        .recover_end_resultants_from_global_model_with_spans_and_axial_effects(&u, PipeEnd::J, &load, &[], &[])
        .unwrap();
    close(tip.shear_force_y, 0.0, 1e-15, "free tip shear");
    close(tip.bending_moment_z, 0.0, 1e-15, "free tip moment");
    // The stale vector with Timoshenko K: v 54/384, theta unchanged.
    let wrong = solve_with(eb);
    close(wrong[j + UY], 54.0 / 384.0, 1e-15, "stale tip v");
    close(wrong[j + RZ], 7.0 / 48.0, 1e-15, "stale tip theta");
    assert!((wrong[j + UY] - u[j + UY]).abs() > 2e-3, "the stale vector is distinguished");

    // The proximal half is the mirror image of the distal half.
    let proximal = [SpannedUniformLocalLoad::new(
        LocalLoadDirection::Y,
        1.0,
        UniformLoadSpan::new(0.0, 0.5).unwrap(),
    )
    .unwrap()];
    let p = member.equivalent_nodal_loads_with_spans(&proximal, &[]).unwrap();
    close(p[UY], 25.0 / 64.0, 1e-15, "proximal V_i");
    close(p[RZ], 19.0 / 384.0, 1e-15, "proximal M_i");
    close(p[j + UY], 7.0 / 64.0, 1e-15, "proximal V_j");
    close(p[j + RZ], -13.0 / 384.0, 1e-15, "proximal M_j");

    // The Iy plane (v = u_z, theta = -r_y) carries the same correction.
    let z = member
        .equivalent_nodal_loads_with_spans(&[distal_half(LocalLoadDirection::Z)], &[])
        .unwrap();
    close(z[UZ], 7.0 / 64.0, 1e-15, "z V_i");
    close(z[RY], -13.0 / 384.0, 1e-15, "z r_y moment i");
    close(z[j + UZ], 25.0 / 64.0, 1e-15, "z V_j");
    close(z[j + RY], 19.0 / 384.0, 1e-15, "z r_y moment j");

    // S11-G (W-16t TPS-G): a Timoshenko partial span records every slot as
    // `Bounded` against the corrected held-operand formula (its soundness and
    // tightness are judged by `tps_g_tests`); a full span keeps the exact
    // Euler–Bernoulli formula.
    let global = [SpannedGlobalUniformLoad::new([0.0, 1.0, 0.0], UniformLoadSpan::new(0.5, 1.0).unwrap()).unwrap()];
    let (values, formations) = member.equivalent_global_nodal_loads_with_spans_formed(&global).unwrap();
    assert_eq!(values, feq);
    assert!(formations
        .iter()
        .all(|f| matches!(f, Formation::Bounded { bound } if bound.is_finite() && *bound <= 1e-15)));
    let full = [SpannedGlobalUniformLoad::full([0.0, 1.0, 0.0]).unwrap()];
    let (_, formations) = member.equivalent_global_nodal_loads_with_spans_formed(&full).unwrap();
    assert!(formations.iter().all(|f| matches!(f, Formation::Exact { .. })));
}

/// A full-span uniform load uses the Euler–Bernoulli terms unchanged, bit for
/// bit (SHEAR_REFERENCE §4: [wL/2, wL²/12, wL/2, -wL²/12]); a point member
/// force in a Timoshenko element is refused by name, never formed with
/// Hermite terms.
#[test]
fn timoshenko_full_span_terms_are_euler_bernoulli_and_point_forces_are_refused() {
    let member = pipe("T", 0, 1, 0.0, 2.5, timoshenko());
    let stale = pipe("EB", 0, 1, 0.0, 2.5, euler_bernoulli());
    for direction in [LocalLoadDirection::X, LocalLoadDirection::Y, LocalLoadDirection::Z] {
        let load = [SpannedUniformLocalLoad::full(direction, -3.7).unwrap()];
        let t = member.equivalent_nodal_loads_with_spans(&load, &[]).unwrap();
        let e = stale.equivalent_nodal_loads_with_spans(&load, &[]).unwrap();
        assert_eq!(t.map(f64::to_bits), e.map(f64::to_bits), "{direction:?}");
    }
    let point = [PointLocalForce::new(0.3, LocalLoadDirection::Y, 1.0).unwrap()];
    assert_eq!(
        member.equivalent_nodal_loads_with_spans(&[], &point),
        Err(StraightPipeError::UnsupportedLoadForBeamTheory {
            name: "point member force in a timoshenko element (no Hermite equivalent)"
        })
    );
    assert!(stale.equivalent_nodal_loads_with_spans(&[], &point).is_ok());
}

/// The leaf travels StraightPipeSectionProperties -> frame_section ->
/// frame_element; `new` stays Euler–Bernoulli.
#[test]
fn timoshenko_leaf_reaches_the_frame_element() {
    let eb = euler_bernoulli();
    assert_eq!(eb.beam_theory(), BeamTheory::EulerBernoulli);
    assert_eq!(eb.frame_section().unwrap().beam_theory, BeamTheory::EulerBernoulli);
    let t = timoshenko();
    assert_eq!(t.beam_theory(), BeamTheory::Timoshenko { shear_area: 12.0 });
    let member = pipe("T", 0, 1, 0.0, 1.0, t);
    assert_eq!(
        member.frame_element().unwrap().section.beam_theory,
        BeamTheory::Timoshenko { shear_area: 12.0 }
    );
    assert_eq!(member.timoshenko_phi().unwrap(), Some([1.0, 1.0]));
    assert_eq!(pipe("EB", 0, 1, 0.0, 1.0, eb).timoshenko_phi().unwrap(), None);
    for bad in [0.0, -1.0, f64::NAN] {
        assert!(StraightPipeSectionProperties::timoshenko(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, bad, None).is_err());
    }
    assert_eq!(std::mem::size_of::<StraightPipeSectionProperties>(), 80);
    let _ = (UZ, RY);
}

/// SHEAR_REFERENCE §5 S-E: a fixed-root cantilever (EI = 1, Ks = 12, L = 1)
/// with a ground spring k = 12 at a = 1/2 and a tip force F = 1, assembled
/// with FK from two elements. Timoshenko: R = -7/8, u(a) = 7/96,
/// u(L) = 37/128, theta(L) = 25/64, root force -1/8, moment -9/16.
/// Euler–Bernoulli: R = -5/6, u(a) = 5/72, u(L) = 71/288, theta(L) = 19/48,
/// root -1/6, -7/12. The end actions are recovered with the Timoshenko
/// matrix, in binary64 and in K-D5's certified recovery; Euler–Bernoulli K
/// applied to the Timoshenko displacements does not give them.
#[test]
fn timoshenko_recovered_end_actions_use_the_timoshenko_matrix() {
    let spring = [(DOF_PER_NODE + UY, 12.0)];
    let mut force = vec![0.0; 18];
    force[2 * DOF_PER_NODE + UY] = 1.0;
    for (section, expected) in [
        (
            timoshenko(),
            [-7.0 / 8.0, 7.0 / 96.0, 37.0 / 128.0, 25.0 / 64.0, -1.0 / 8.0, -9.0 / 16.0],
        ),
        (
            euler_bernoulli(),
            [-5.0 / 6.0, 5.0 / 72.0, 71.0 / 288.0, 19.0 / 48.0, -1.0 / 6.0, -7.0 / 12.0],
        ),
    ] {
        let [reaction, ua, ul, theta, root_force, root_moment] = expected;
        let members = [pipe("A", 0, 1, 0.0, 0.5, section), pipe("B", 1, 2, 0.5, 1.0, section)];
        let frames: Vec<FrameElement> = members.iter().map(|m| m.frame_element().unwrap()).collect();
        let u = solve(&frames, 3, &spring, &force);
        let what = format!("{:?}", section.beam_theory());
        close(u[DOF_PER_NODE + UY], ua, 1e-14, &format!("{what} u(a)"));
        close(-12.0 * u[DOF_PER_NODE + UY], reaction, 1e-14, &format!("{what} R"));
        close(u[2 * DOF_PER_NODE + UY], ul, 1e-14, &format!("{what} u(L)"));
        close(u[2 * DOF_PER_NODE + RZ], theta, 1e-14, &format!("{what} theta(L)"));
        // Binary64 recovery: the root element's i-end actions K_e·d.
        let root = members[0].recover_end_resultants_from_global_model(&u, PipeEnd::I).unwrap();
        close(root.shear_force_y, root_force, 1e-14, &format!("{what} root force"));
        close(root.bending_moment_z, root_moment, 1e-14, &format!("{what} root moment"));
        // At node a the two elements' end actions sum to the spring force
        // R = -k u(a) (K_frames u = f - K_spring u there).
        let a_j = members[0].recover_end_resultants_from_global_model(&u, PipeEnd::J).unwrap();
        let b_i = members[1].recover_end_resultants_from_global_model(&u, PipeEnd::I).unwrap();
        close(a_j.shear_force_y + b_i.shear_force_y, reaction, 1e-14, &format!("{what} node a balance"));
        // K-D5's certified recovery (frame_matrix at p) of the root element,
        // with zero corrections.
        let bound = MemberBoundInputs {
            w: vec![0.0; 18],
            w2: vec![0.0; 18],
            exponents: vec![i32::MIN; 18],
            lambda_sigma: 0.0,
        };
        // T4-U8 (Design §4 item 2): bound to the member's formation-source frame.
        let source = open_pipe_stress_frame_kernel::structural::FormationSource {
            node_count: 3,
            frames: frames.clone(),
            frame_members: vec!["member:1:A:".into(), "member:1:B:".into()],
            ..Default::default()
        };
        let loads = CertifiedMemberLoads::default().with_member_source(&source, "member:1:A:");
        let certified = certify_straight_recovery(&frames[0], &loads, &u, &bound).unwrap();
        close(certified.ends[UY].published, root_force, 1e-14, &format!("{what} certified root force"));
        close(certified.ends[RZ].published, root_moment, 1e-14, &format!("{what} certified root moment"));
        if section.beam_theory().is_timoshenko() {
            // Stiffness-only replacement: Euler–Bernoulli K on these
            // displacements misses the Timoshenko root actions.
            let stale = pipe("A", 0, 1, 0.0, 0.5, euler_bernoulli());
            let wrong = stale.recover_end_resultants_from_global_model(&u, PipeEnd::I).unwrap();
            assert!((wrong.shear_force_y - root_force).abs() > 1e-2, "{wrong:?}");
        }
    }
}
