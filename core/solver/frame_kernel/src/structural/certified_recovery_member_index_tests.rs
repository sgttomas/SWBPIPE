//! `CertifiedMemberLoads::from_member_force` tests only the primitives the
//! force's member index lists for a node pair. These controls hold it to
//! `from_member_primitives`, which tests every primitive of the case: every
//! member-load consumer must publish the same bits from either.
use super::super::formation_check::{IntensityPrimitive, LoadSummandIdentity};
use super::*;
use crate::load_ledger::LoadLedger;
use crate::{ForceScale, FrameNode, FrameSection};

const NODES: usize = 5;
fn frame(i: usize, j: usize) -> FrameElement {
    let at = |n: usize| [2.0 * n as f64, 0.25 * (n * n) as f64, 0.0];
    FrameElement::new(
        FrameNode::new(i, at(i)).unwrap(),
        FrameNode::new(j, at(j)).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 3.0, 3.0, 6.0).unwrap(),
        [0.0, 0.0, 1.0],
    )
    .unwrap()
}
/// One uniform load of one member: its twelve element-row primitives.
fn uniform(frame: &FrameElement, source: &str, w: [f64; 3], span: (f64, f64)) -> Vec<LoadPrimitive> {
    (0..12)
        .map(|row| LoadPrimitive {
            term_index: 0,
            identity: LoadSummandIdentity {
                family: "uniform_straight".into(),
                source: source.into(),
                ordinal: row,
            },
            local_row: row,
            formula: LoadFormula::UniformStraight {
                frame: *frame,
                intensity: IntensityPrimitive::Authored(w),
                start_fraction: span.0,
                end_fraction: span.1,
            },
            force_scale_exponent: 0,
        })
        .collect()
}
fn nodal(dof: usize) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: "nodal".into(),
            source: "member:1:A:not a member load".into(),
            ordinal: dof,
        },
        local_row: dof % 6,
        formula: LoadFormula::NodalProduct {
            global_dof: dof,
            magnitude: 3.0,
            factor: 0.5,
        },
        force_scale_exponent: 0,
    }
}
/// (prefix, node i, node j): A and B are parallel members of one node pair, C
/// continues the run, D is A's pair reversed, E carries no load.
const MEMBERS: [(&str, usize, usize); 5] = [
    ("member:1:A:", 0, 1),
    ("member:1:B:", 0, 1),
    ("member:1:C:", 1, 2),
    ("member:1:D:", 1, 0),
    ("member:1:E:", 3, 4),
];
/// The case's primitives with every member's loads interleaved, so no node
/// pair's primitives are contiguous in the ledger and A's three loads, whose
/// magnitudes differ by orders, stand in a definite order.
fn interleaved() -> Vec<LoadPrimitive> {
    let loads = [
        uniform(&frame(0, 1), "member:1:A:w1", [0.1, -0.3, 0.7], (0.0, 1.0)),
        uniform(&frame(1, 2), "member:1:C:w1", [0.9, 0.2, -0.4], (0.0, 1.0)),
        uniform(&frame(0, 1), "member:1:B:w1", [-6.0, 12.0, 0.5], (0.125, 0.5)),
        uniform(&frame(0, 1), "member:1:A:w2", [1.0e-3, 2.5, -1.0e-7], (0.25, 0.75)),
        uniform(&frame(1, 0), "member:1:D:w1", [0.3, 0.3, 0.3], (0.0, 0.625)),
        uniform(&frame(0, 1), "member:1:A:w3", [3.0e5, 0.2, 0.9], (0.5, 1.0)),
        uniform(&frame(1, 2), "member:1:C:w2", [1.0e-9, -7.0, 2.0], (0.375, 1.0)),
    ];
    let mut out = Vec::new();
    for row in 0..12 {
        for load in &loads {
            out.push(load[row].clone());
        }
        out.push(nodal(row));
    }
    out
}
fn force_of(primitives: &[LoadPrimitive]) -> AssembledForce {
    let mut ledger = LoadLedger::new();
    for primitive in primitives {
        ledger.attach_load_primitive(primitive.clone());
    }
    ledger.finish(6 * NODES).unwrap()
}
fn source() -> FormationSource<'static> {
    FormationSource {
        node_count: NODES,
        frames: MEMBERS.iter().map(|&(_, i, j)| frame(i, j)).collect(),
        frame_members: MEMBERS.iter().map(|&(prefix, _, _)| prefix.into()).collect(),
        ..FormationSource::default()
    }
}
fn bound() -> MemberBoundInputs {
    let n = 6 * NODES;
    MemberBoundInputs {
        w: (0..n).map(|k| 1.0e-12 * (k as f64 - 7.0)).collect(),
        w2: (0..n).map(|k| -1.0e-21 * (k as f64 + 1.0)).collect(),
        exponents: vec![0; n],
        lambda_sigma: 2.0_f64.powi(-40),
        lift_radius: Vec::new(),
    }
}
fn displacements() -> Vec<f64> {
    (0..6 * NODES).map(|k| 1.0e-3 * ((k * k) as f64 - 11.0 * k as f64 + 0.1)).collect()
}
/// Everything a member's loads publish, with every private midpoint and
/// radius, as text: `{:?}` of a binary64 is its shortest round-trip form (and
/// keeps the sign of zero), and the wide midpoints print their limbs.
fn published(loads: &CertifiedMemberLoads, frame: &FrameElement) -> String {
    let u = displacements();
    let bound = bound();
    format!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        loads.equivalent,
        loads.equivalent_scale,
        loads.uniform_loads().collect::<Vec<_>>(),
        certify_straight_recovery(frame, loads, &u, &bound),
        certify_straight_stress_spans(frame, loads, &u, &bound, 8.0, 3.0, None),
    )
}
fn by_scan(force: &AssembledForce, source: &FormationSource, member: usize) -> String {
    let (prefix, i, j) = MEMBERS[member];
    let loads = CertifiedMemberLoads::from_member_primitives(i, j, force.load_primitives(), prefix)
        .unwrap()
        .with_member_source(source, prefix);
    assert_eq!(loads.candidates().count(), force.load_primitives().len());
    published(&loads, &frame(i, j))
}
fn by_index(force: &AssembledForce, source: &FormationSource, member: usize) -> String {
    let (prefix, i, j) = MEMBERS[member];
    let loads = CertifiedMemberLoads::from_member_force(i, j, force, prefix)
        .unwrap()
        .with_member_source(source, prefix);
    // The index is in use: only this node pair's primitives are tested.
    let pair = force
        .load_primitives()
        .iter()
        .filter(|p| member_pair(&p.formula) == Some((i, j)))
        .count();
    assert_eq!(loads.candidates().count(), pair);
    assert!(pair < force.load_primitives().len());
    published(&loads, &frame(i, j))
}

#[test]
fn the_member_index_publishes_what_the_whole_case_scan_publishes() {
    let source = source();
    let force = force_of(&interleaved());
    for member in 0..MEMBERS.len() {
        let scan = by_scan(&force, &source, member);
        assert!(scan.contains("Ok("), "{scan}");
        assert_eq!(by_index(&force, &source, member), scan, "{}", MEMBERS[member].0);
    }
    // A loaded member is not an unloaded one, and parallel members differ.
    assert_ne!(by_index(&force, &source, 0), by_index(&force, &source, 1));
    assert_eq!(
        CertifiedMemberLoads::from_member_force(3, 4, &force, MEMBERS[4].0)
            .unwrap()
            .candidates()
            .count(),
        0
    );
}

/// The published bits depend on the order in which a member's primitives are
/// met (upward-rounded scales and radii are summed in that order), so the
/// control above fails if the index hands them over in another order.
#[test]
fn the_order_of_a_members_primitives_reaches_the_published_bits() {
    let source = source();
    let forward = interleaved();
    let mut reversed = forward.clone();
    reversed.reverse();
    let forward = force_of(&forward);
    let reversed = force_of(&reversed);
    assert_ne!(by_scan(&forward, &source, 0), by_scan(&reversed, &source, 0));
    // The index follows each ledger's own order.
    assert_eq!(by_index(&reversed, &source, 0), by_scan(&reversed, &source, 0));
}

/// A force-scaled copy and a force with appended terms carry an index of their
/// own primitives (`AssembledForce::force_scaled`, `with_appended`).
#[test]
fn derived_forces_keep_a_member_index_of_their_own_primitives() {
    let source = source();
    let force = force_of(&interleaved());
    let scaled = force.force_scaled(ForceScale::new(4).unwrap()).unwrap();
    let mut extra = LoadLedger::new();
    for primitive in uniform(&frame(0, 1), "member:1:A:appended", [0.5, 0.25, -2.0], (0.0, 0.875)) {
        extra.attach_load_primitive(primitive);
    }
    extra.attach_load_primitive(nodal(7));
    let appended = force.with_appended(extra).unwrap();
    assert_eq!(appended.load_primitives().len(), force.load_primitives().len() + 13);
    for member in 0..MEMBERS.len() {
        assert_eq!(by_index(&scaled, &source, member), by_scan(&scaled, &source, member));
        assert_eq!(by_index(&appended, &source, member), by_scan(&appended, &source, member));
    }
    assert_ne!(by_index(&scaled, &source, 0), by_index(&force, &source, 0));
    assert_ne!(by_index(&appended, &source, 0), by_index(&force, &source, 0));
}

/// A member primitive beyond the twelve element rows is refused from either.
#[test]
fn a_member_primitive_row_out_of_range_is_refused_from_either() {
    let mut primitives = interleaved();
    let last = primitives.iter().rposition(|p| p.identity.source == "member:1:C:w2").unwrap();
    primitives[last].local_row = 12;
    let force = force_of(&primitives);
    for (member, refused) in [(0, false), (2, true)] {
        let (prefix, i, j) = MEMBERS[member];
        let scan = CertifiedMemberLoads::from_member_primitives(i, j, force.load_primitives(), prefix);
        let index = CertifiedMemberLoads::from_member_force(i, j, &force, prefix);
        assert_eq!(scan.is_err(), refused);
        assert_eq!(format!("{:?}", scan.err()), format!("{:?}", index.err()));
    }
}
