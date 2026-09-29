//! K4 tests of `retained/recover.rs` (the brief's H): recovery and
//! publication.
use super::super::adaptive::{solve_case, CaseLimit, CaseOutcome, InvocationMeter, StageGuard};
use super::super::assemble::{assemble, form_directional, form_members, Structure};
use super::super::ledger::RetainedLedger;
use super::super::source::{
    Component, Constraint, Dof, PrimitiveSource, SourceParts, StraightMember,
};
use super::super::wide::multi::Binary64Outcome;
use super::super::wide::Wide;
use super::super::wide_sum::ExactWideSum;
use super::*;
use std::collections::BTreeMap;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;

#[test]
fn every_recovered_kind_matches_its_exact_reference() {
    // Every model whose case is selected and has an exact reference; the
    // published kinds compared are tallied by key prefix.
    let mut tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut models_compared = 0;
    // D1 revision 5a.3 withholds DIRECTIONAL-SPAN (its springs span R³ only
    // through 2^-52); DIRECTIONAL-WELL keeps directional springs compared.
    let well = models::parse_models(include_str!("models5a3.txt"))
        .into_iter()
        .filter(|m| m.name == "DIRECTIONAL-WELL");
    for m in models::models().into_iter().chain(well) {
        if m.expect.is_empty() {
            continue;
        }
        let Ok(source) = PrimitiveSource::new(m.parts.clone()) else {
            continue;
        };
        let mut meter = InvocationMeter::new(u64::MAX);
        let CaseOutcome::Selected(solve) =
            solve_case(source.clone(), CaseLimit::new(u64::MAX), &mut meter)
        else {
            continue;
        };
        let rows = &solve.publish().rows;
        // Each row against the claim it publishes (D1 revision 5a.3, R7 §5.2;
        // ROOT's ruling at C): RIGID-UNLOADED, selected at 512, withholds its
        // rows near zero as `absolute_verified` with their own bound.
        let (worst, at, compared) =
            models::compare_honest(solve.publish(), solve.selected_precision(), &m.expect);
        assert!(worst <= 1.0, "{}: {worst} at {at}", m.name);
        assert!(compared > 0, "{}", m.name);
        let published = models::published(rows);
        for key in m.expect.keys().filter(|k| published.contains_key(*k)) {
            *tally
                .entry(key.split('.').next().unwrap().to_string())
                .or_default() += 1;
        }
        models_compared += 1;
    }
    println!("{models_compared} models; compared by kind: {tally:?}");
    for prefix in [
        "u", "mag", "end", "st", "spr", "dspr", "R", "sf", "sm", "N", "T", "Mb", "Mbs",
    ] {
        assert!(
            tally.get(prefix).copied().unwrap_or(0) > 0,
            "{prefix} never compared"
        );
    }
    // N01's asymmetric t = 0.25 station is among them.
    let m = models::model("N01");
    assert_eq!(m.parts.stations[0].fraction, 0.25);
    assert_eq!(
        m.expect.keys().filter(|k| k.starts_with("st.1.")).count(),
        6
    );
}

#[test]
fn an_exact_zero_is_published_as_plus_zero() {
    assert_eq!(
        publish_value(&Wide::<4>::ZERO.neg()),
        Binary64Outcome::Normal(0.0)
    );
    assert_eq!(
        publish_value(&Wide::<4>::ZERO.neg())
            .value()
            .unwrap()
            .to_bits(),
        0
    );
    // No published row of any selected model carries −0.0.
    for m in models::models() {
        let Ok(source) = PrimitiveSource::new(m.parts.clone()) else {
            continue;
        };
        let mut meter = InvocationMeter::new(u64::MAX);
        if let CaseOutcome::Selected(solve) =
            solve_case(source, CaseLimit::new(u64::MAX), &mut meter)
        {
            for r in &solve.publish().rows {
                if let Some(v) = r.value.value() {
                    assert_ne!(v.to_bits(), 0x8000_0000_0000_0000, "{}: {:?}", m.name, r.id);
                }
            }
        }
    }
}

#[test]
fn subnormal_underflow_and_overflow_are_published_with_their_outcome() {
    let one = Wide::<4>::ONE;
    let three_quanta = support::lift::<4>(3.0 * f64::from_bits(1));
    match publish_value(&three_quanta) {
        Binary64Outcome::Subnormal { value, .. } => assert_eq!(value.to_bits(), 3),
        other => panic!("{other:?}"),
    }
    assert_eq!(
        publish_value(&one.mul_pow2(-1080).unwrap()),
        Binary64Outcome::Underflow { negative: false }
    );
    assert_eq!(
        publish_value(&one.mul_pow2(-1080).unwrap().neg()),
        Binary64Outcome::Underflow { negative: true }
    );
    assert_eq!(
        publish_value(&one.mul_pow2(1030).unwrap()),
        Binary64Outcome::Overflow { negative: false }
    );
    assert_eq!(
        publish_value(&one.mul_pow2(1030).unwrap().neg()),
        Binary64Outcome::Overflow { negative: true }
    );
    assert_eq!(
        publish_value(&support::lift::<4>(1.5)),
        Binary64Outcome::Normal(1.5)
    );
}

#[test]
fn n06s_torque_and_spring_action_are_nonzero_and_correct() {
    // §7.3-2: the root spring carries the torque through the member.
    let m = models::model("N06");
    let source = m.source();
    let mut meter = InvocationMeter::new(u64::MAX);
    let CaseOutcome::Selected(solve) = solve_case(source, CaseLimit::new(u64::MAX), &mut meter)
    else {
        panic!()
    };
    let published = models::published(&solve.publish().rows);
    for key in ["T.1", "spr.1.3"] {
        let (exp, obs) = (m.expect[key], published[key].0);
        assert!(exp != 0.0, "{key}");
        assert!(
            (obs - exp).abs() <= 1e-9 * exp.abs(),
            "{key}: {obs:e} vs {exp:e}"
        );
    }
}

#[test]
fn a_recovered_action_is_one_exact_expansion_where_a_sequential_fold_loses_a_term() {
    // D16's recovery half. A member along (1,2,2)/3 whose node i moves by
    // u_i = (2, 2^-200, −1) and node j not at all: e_x·u_i = 2/3 − 2/3 +
    // (2/3)·2^-200 at p (e_x = (1/3, 2/3, 2/3) rounded at p, 2·fl(1/3) =
    // fl(2/3) exactly). A sequential fold in the code's order adds the tiny
    // term to 2/3 first and loses it, so the axial force would be 0; its true
    // value is −(EA/L)·(2/3)·2^-200.
    let dof = |node, c| Dof {
        node,
        component: Component::from_index(c),
    };
    let tiny = f64::from_bits(0x3370_0000_0000_0000); // 2^-200
    let mut parts = SourceParts {
        nodes: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 2.0]],
        members: vec![StraightMember {
            id: 1,
            node_i: 0,
            node_j: 1,
            elastic_modulus: 200e9,
            shear_modulus: 80e9,
            area: 6e-3,
            second_moment_y: 2.7e-5,
            second_moment_z: 2.7e-5,
            torsion_constant: 5.4e-5,
            y_reference: [1.0, 0.0, 0.0],
        }],
        ..Default::default()
    };
    for (c, v) in [2.0, tiny, -1.0, 0.0, 0.0, 0.0].into_iter().enumerate() {
        parts.constraints.push(Constraint {
            dof: dof(0, c),
            value: v,
        });
        parts.constraints.push(Constraint {
            dof: dof(1, c),
            value: 0.0,
        });
    }
    let source = PrimitiveSource::new(parts).unwrap();
    let layout = layout(&source);
    let structure = Structure::new(&source).unwrap();
    let guard = StageGuard::unlimited();
    let mut c = support::ctx::<4>(128);
    let mut sum = ExactWideSum::new();
    let members = form_members(&mut c, &mut sum, &guard, &source).unwrap();
    let directional = form_directional(&mut c, &mut sum, &source).unwrap();
    let k = assemble(
        &mut c,
        &mut sum,
        &guard,
        &source,
        &structure,
        &members,
        &directional,
    )
    .unwrap();
    let ledger = RetainedLedger::from_source(&source).unwrap();
    let u: Vec<Wide<4>> = (0..source.dof_count())
        .map(|g| support::lift::<4>(source.constraint(g).unwrap()))
        .collect();
    let recovered = recover(
        &mut c,
        &mut sum,
        &guard,
        &source,
        &layout,
        &structure,
        &k,
        &members,
        &directional,
        &ledger,
        &u,
    )
    .unwrap();
    let index = layout
        .iter()
        .position(|m| {
            m.id == QuantityId::EndAction {
                member: 1,
                end: End::J,
                component: Component::Ux,
            }
        })
        .unwrap();
    let n = publish_value(&recovered.values[index]).value().unwrap();
    let exact = -(200e9 * 6e-3 / 3.0) * (2.0 / 3.0) * tiny;
    assert!(n != 0.0);
    assert!(
        (n - exact).abs() <= 1e-12 * exact.abs(),
        "{n:e} vs {exact:e}"
    );
    // The code-order fold of e_x·u_i at p loses the term.
    let ex = &members[0].axes[0];
    let mut folded = Wide::<4>::ZERO;
    for kk in 0..3 {
        let t = c.mul(&ex[kk], &u[kk]).unwrap();
        folded = c.add(&folded, &t).unwrap();
    }
    assert!(folded.is_zero());
}
