//! K4 tests of `retained/combine.rs` (written at checkpoint A2).
use super::super::adaptive::{solve_case, CaseLimit, CaseOutcome, InvocationMeter, RetainedSolve};
use super::*;

#[allow(dead_code)]
#[path = "support.rs"]
mod support;
#[allow(dead_code)]
#[path = "models.rs"]
mod models;

fn selected(name: &str) -> Box<RetainedSolve> {
    let mut meter = InvocationMeter::new(u64::MAX);
    match solve_case(models::model(name).source(), CaseLimit::new(u64::MAX), &mut meter) {
        CaseOutcome::Selected(s) => s,
        other => panic!("{name}: {other:?}"),
    }
}

#[test]
fn probe_ceiling_combination() {
    for combo in models::combos() {
        let operands: Vec<(f64, Box<RetainedSolve>)> =
            combo.operands.iter().map(|(c, n)| (*c, selected(n))).collect();
        let refs: Vec<(f64, &RetainedSolve)> = operands.iter().map(|(c, s)| (*c, s.as_ref())).collect();
        let mut meter = InvocationMeter::new(u64::MAX);
        match RetainedCombination::solve(&refs, CaseLimit::new(u64::MAX), &mut meter) {
            CombinationOutcome::Selected(c) => {
                let source = operands[0].1.source();
                let (worst, at, n) = models::compare(source, &c.publish().rows, &combo.expect);
                let nonzero = c.publish().rows.iter().filter(|r| r.value.value().is_some_and(|v| v != 0.0)).count();
                println!(
                    "{}: selected {} worst {:.3e} at {} ({} compared, {} nonzero rows) attempts {:?}",
                    combo.name,
                    c.selected_precision(),
                    worst,
                    at,
                    n,
                    nonzero,
                    c.evidence().attempts.iter().map(|a| (a.precision, a.accepted)).collect::<Vec<_>>()
                );
            }
            CombinationOutcome::Unresolved { reason, attempts } => println!(
                "{}: unresolved {:?} {:?}",
                combo.name,
                reason,
                attempts.iter().map(|a| (a.precision, a.accepted)).collect::<Vec<_>>()
            ),
        }
    }
}

#[test]
fn probe_single_case_lost_load() {
    use super::super::source::{Component, Constraint, Dof, NodalLoad, PrimitiveSource, SourceParts, StraightMember};
    let dof = |node, c| Dof { node, component: Component::from_index(c) };
    let mut parts = SourceParts {
        nodes: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        members: vec![StraightMember {
            id: 1, node_i: 0, node_j: 1, elastic_modulus: 1024.0, shear_modulus: 1.0, area: 1.0,
            second_moment_y: 1.0, second_moment_z: 1.0, torsion_constant: 1.0, y_reference: [0.0, 1.0, 0.0],
        }],
        ..Default::default()
    };
    for c in 0..6 {
        parts.constraints.push(Constraint { dof: dof(0, c), value: if c == 0 { 1.0 } else { 0.0 } });
    }
    for c in 1..6 {
        parts.constraints.push(Constraint { dof: dof(1, c), value: 0.0 });
    }
    parts.loads.push(NodalLoad { dof: dof(1, 0), value: f64::from_bits(0x2D30_0000_0000_0000), source_id: "f".into() });
    let source = PrimitiveSource::new(parts).unwrap();
    let mut meter = InvocationMeter::new(u64::MAX);
    match solve_case(source, CaseLimit::new(u64::MAX), &mut meter) {
        CaseOutcome::Selected(s) => {
            for r in &s.publish().rows {
                println!("{:?} {:?} {:?}", r.id, r.value, r.class);
            }
            println!("selected {} {:?}", s.selected_precision(), s.evidence().attempts.iter().map(|a| (a.precision, a.outcome.clone())).collect::<Vec<_>>());
        }
        other => println!("{other:?}"),
    }
}
