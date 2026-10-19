//! O4 (DEL-04-08 Design `o4-source-extension.md` §4.2–§4.4, §6): the retained
//! Timoshenko straight member. Expected values are closed forms of the 1D
//! model (v_tip = FL³/(3EI) + FL/Ks, θ_tip = FL²/(2EI)) or the independent
//! outward reference `timoshenko_member.txt` (`gen_timoshenko_member_vectors.py`),
//! never the production element. Invented inputs only.
use super::super::adaptive::{CaseLimit, ExecutionOutcome, StageGuard};
use super::super::origins::{OriginCapacity, RecordedInvocation};
use super::super::recover::QuantityId;
use super::super::source::*;
use super::super::wide::multi::WideContext;
use super::super::wide::Wide;
use super::super::wide_sum::ExactWideSum;
use super::{form_members_with, MemberOperators};

fn member(id: u32, i: u32, j: u32, iy: f64, iz: f64) -> StraightMember {
    StraightMember {
        id,
        node_i: i,
        node_j: j,
        elastic_modulus: 1.0,
        shear_modulus: 1.0,
        area: 1.0,
        second_moment_y: iy,
        second_moment_z: iz,
        torsion_constant: 1.0,
        y_reference: [0.0, 1.0, 0.0],
    }
}

/// A cantilever along x of `n` equal members, clamped at node 0, tip load
/// `force` on `dof` (local component index at the tip node).
fn cantilever(length: f64, n: u32, iy: f64, iz: f64, dof: usize, force: f64) -> PrimitiveSource {
    let nodes = (0..=n).map(|k| [length * f64::from(k) / f64::from(n), 0.0, 0.0]).collect();
    let members = (0..n).map(|k| member(k, k, k + 1, iy, iz)).collect();
    PrimitiveSource::new(SourceParts {
        nodes,
        members,
        constraints: (0..6).map(|g| Constraint { dof: Dof::from_global(g), value: 0.0 }).collect(),
        loads: vec![NodalLoad {
            dof: Dof::from_global(n as usize * 6 + dof),
            value: force,
            source_id: "tip".into(),
        }],
        ..Default::default()
    })
    .unwrap()
}

fn leaves(source: &PrimitiveSource, shear_area: f64) -> TimoshenkoLeafSet {
    let leaves = source.members().iter().map(|m| TimoshenkoLeaf { member: m.id, shear_area }).collect();
    TimoshenkoLeafSet::new(source, leaves).unwrap()
}

fn solve(source: &PrimitiveSource, shear: Option<&TimoshenkoLeafSet>) -> super::super::adaptive::RetainedSolve {
    let mut invocation =
        RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1], &[]).unwrap()).unwrap();
    let refs = [MemberSourceRef::nodal(source)];
    let mut cases = match shear {
        Some(set) => invocation.solve_timoshenko_cases(&refs, set, CaseLimit::new(u64::MAX)).unwrap(),
        None => invocation.solve_member_cases(&refs, CaseLimit::new(u64::MAX)).unwrap(),
    };
    match cases.remove(0).outcome {
        ExecutionOutcome::Selected(owner) => *owner,
        other => panic!("{other:?}"),
    }
}

fn displacement(owner: &super::super::adaptive::RetainedSolve, global: usize) -> f64 {
    owner
        .publish()
        .rows
        .iter()
        .find(|r| r.id == QuantityId::Displacement(Dof::from_global(global)))
        .unwrap()
        .value
        .value()
        .unwrap()
}

/// |actual − expected| ≤ tol·|expected|.
fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol * expected.abs(), "{what}: {actual:e} vs {expected:e}");
}

#[test]
fn retained_timoshenko_cantilever_matches_the_closed_form() {
    // E = G = 1, L = 2, I_z = 1/2, I_y = 3/4, A_s = 1/4: Ks = 1/4.
    let (length, iy, iz, a_s, f) = (2.0, 0.75, 0.5, 0.25, 3.0);
    for n in [1u32, 3] {
        let source = cantilever(length, n, iy, iz, 1, f);
        let shear = leaves(&source, a_s);
        let owner = solve(&source, Some(&shear));
        assert!(owner.timoshenko().is_some());
        let tip = n as usize * 6;
        // v_tip (y direction bends about z): FL³/(3EI_z) + FL/Ks.
        let v = f * length.powi(3) / (3.0 * iz) + f * length / a_s;
        close(displacement(&owner, tip + 1), v, 1e-14, "v_tip");
        close(displacement(&owner, tip + 5), f * length * length / (2.0 * iz), 1e-14, "theta_z tip");
        // The Euler–Bernoulli twin has no shear term.
        let eb = solve(&source, None);
        assert!(eb.timoshenko().is_none());
        close(displacement(&eb, tip + 1), f * length.powi(3) / (3.0 * iz), 1e-14, "v_tip EB");
        // The other plane bends about y with I_y.
        let source = cantilever(length, n, iy, iz, 2, f);
        let owner = solve(&source, Some(&leaves(&source, a_s)));
        let w = f * length.powi(3) / (3.0 * iy) + f * length / a_s;
        close(displacement(&owner, tip + 2), w, 1e-14, "w_tip");
    }
}

/// §4.2: the formed d₁, d₂ and d̄ at p follow the fixed rounding schedule;
/// at φ = 2 exactly d₂ is an exact zero.
#[test]
fn retained_timoshenko_entries_follow_the_fixed_schedule() {
    // L = 1, E = G = 1, A_s = 3, I_z = 1/2: φ_z = 12·(1/2)/3 = 2; I_y = 3: φ_y = 12.
    let source = cantilever(1.0, 1, 3.0, 0.5, 1, 1.0);
    let shear = leaves(&source, 3.0);
    let mut ctx = WideContext::<4>::new(128).unwrap();
    let mut sum = ExactWideSum::new();
    let members: Vec<MemberOperators<4>> =
        form_members_with(&mut ctx, &mut sum, &StageGuard::unlimited(), &source, Some(&shear)).unwrap();
    let t = members[0].timoshenko.unwrap();
    let w = |x: f64| Wide::<4>::from_f64(x).unwrap();
    // c_z = 1/2, t₃ = 3/2, sl = 3, φ = 2, op = 3, c_s = 1/2: d₁ = 1, d₂ = 0, d̄ = 2.
    assert_eq!(t.d1[0], w(1.0));
    assert!(t.d2[0].is_zero());
    assert_eq!(t.dbar[0], w(2.0));
    // c_y = 3, t₃ = 9, φ = 12, op = 13, c_s = RN(9/13) at 128 bits.
    let cs = ctx.div(&w(9.0), &w(13.0)).unwrap();
    assert_eq!(t.d1[1], ctx.add(&w(3.0), &cs).unwrap());
    assert_eq!(t.d2[1], ctx.sub(&cs, &w(3.0)).unwrap());
    assert!(t.dbar[1].cmp_value(&ctx.add(&w(3.0), &cs).unwrap().mul_pow2(1).unwrap()).is_ge());
    // The Euler–Bernoulli member keeps no Timoshenko entries.
    let eb: Vec<MemberOperators<4>> =
        form_members_with(&mut ctx, &mut sum, &StageGuard::unlimited(), &source, None).unwrap();
    assert!(eb[0].timoshenko.is_none());
    assert!(eb[0].bounded().dbar.is_none());
}
