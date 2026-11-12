//! `CertifiedMemberLoads::from_member_force` tests only the primitives the
//! force's member index lists for a node pair. These controls hold it to
//! `from_member_primitives`, which tests every primitive of the case: every
//! member-load consumer must publish the same bits from either, for every
//! load formula that binds a member.
use super::super::formation_check::{
    CurvedFormation, EigenstrainPrimitive, FitPrimitive, IntensityPrimitive, LoadSummandIdentity,
    PressureCoefficient, PressureDirection, PressureGeometry, ThermalPrimitive,
};
use super::*;
use crate::connector::{ConnectorAttachment, ScaledWorkMatrix};
use crate::load_ledger::LoadLedger;
use crate::{ForceScale, FrameNode, FrameSection};
use std::collections::{BTreeMap, BTreeSet};

const NODES: usize = 5;
fn at(n: usize) -> [f64; 3] {
    [2.0 * n as f64, 0.25 * (n * n) as f64, 0.0]
}
fn frame(i: usize, j: usize) -> FrameElement {
    FrameElement::new(
        FrameNode::new(i, at(i)).unwrap(),
        FrameNode::new(j, at(j)).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 3.0, 3.0, 6.0).unwrap(),
        [0.0, 0.0, 1.0],
    )
    .unwrap()
}
/// A quarter bend between nodes i and j (its geometry is its own: the
/// recovery reads the arc's coordinates, not the nodes').
fn arc(i: usize, j: usize) -> CurvedFormation {
    CurvedFormation {
        node_i: i,
        node_j: j,
        coordinates_i: [1.0, 0.0, 0.0],
        coordinates_j: [0.0, 1.0, 0.0],
        radius: 1.0,
        y_reference: [1.0, 1.0, 0.0],
        elastic_modulus: 8.0,
        shear_modulus: 4.0,
        area: 2.0,
        second_moment: 3.0,
        torsion_constant: 6.0,
        in_plane_flexibility_factor: 1.0,
        out_of_plane_flexibility_factor: 1.0,
    }
}
fn connector(i: usize, j: usize) -> ObjectiveConnector {
    let mut upper = [0.0; 21];
    let mut at_entry = 0;
    for r in 0..6 {
        for c in r..6 {
            upper[at_entry] = if r == c { 8.0 + r as f64 } else { 0.0 };
            at_entry += 1;
        }
    }
    // Its own chord along its first axis, as `arc` has its own geometry.
    ObjectiveConnector::new(
        FrameNode::new(i, [0.0; 3]).unwrap(),
        FrameNode::new(j, [4.0, 0.0, 0.0]).unwrap(),
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        ScaledWorkMatrix {
            upper_triangle: upper,
            translation_scale: 1.0,
        },
        [0.25, -0.125, 0.0, 0.0, 0.0, 0.0],
    )
    .unwrap()
}
/// One load of one member: its twelve element-row primitives.
fn rows(family: &str, source: &str, formula: LoadFormula) -> Vec<LoadPrimitive> {
    (0..12)
        .map(|row| LoadPrimitive {
            term_index: 0,
            identity: LoadSummandIdentity {
                family: family.into(),
                source: source.into(),
                ordinal: row,
            },
            local_row: row,
            formula: formula.clone(),
            force_scale_exponent: 0,
        })
        .collect()
}
fn uniform(frame: &FrameElement, source: &str, w: [f64; 3], span: (f64, f64)) -> Vec<LoadPrimitive> {
    rows(
        "uniform_straight",
        source,
        LoadFormula::UniformStraight {
            frame: *frame,
            intensity: IntensityPrimitive::Authored(w),
            start_fraction: span.0,
            end_fraction: span.1,
        },
    )
}
fn strain(strain: f64) -> EigenstrainPrimitive {
    EigenstrainPrimitive {
        thermal: ThermalPrimitive::ExplicitIntervalStrain { strain },
        fit: FitPrimitive::None,
    }
}
fn free_strain(frame: &FrameElement, source: &str, eps: f64) -> Vec<LoadPrimitive> {
    rows(
        "free_strain_straight",
        source,
        LoadFormula::FreeStrainStraight {
            frame: *frame,
            strain: strain(eps),
        },
    )
}
fn curved_uniform(arc: CurvedFormation, source: &str, w: [f64; 3]) -> Vec<LoadPrimitive> {
    rows(
        "uniform_curved",
        source,
        LoadFormula::UniformCurved {
            arc,
            intensity: IntensityPrimitive::Authored(w),
        },
    )
}
fn curved_strain(arc: CurvedFormation, source: &str, eps: f64) -> Vec<LoadPrimitive> {
    rows(
        "free_strain_curved",
        source,
        LoadFormula::FreeStrainCurved {
            arc,
            strain: strain(eps),
            column: None,
        },
    )
}
fn curved_pressure(arc: CurvedFormation, source: &str, pressure: f64) -> Vec<LoadPrimitive> {
    rows(
        "pressure_free_strain_curved",
        source,
        LoadFormula::PressureFreeStrainCurved {
            arc,
            pressure,
            outer_diameter: 2.0,
            wall_thickness: 0.5,
            poisson_ratio: 0.3,
            column: None,
        },
    )
}
fn connector_reference(connector: ObjectiveConnector, source: &str) -> Vec<LoadPrimitive> {
    rows("connector_reference", source, LoadFormula::ConnectorReference { connector })
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
/// A pressure operand on A's frame with A's prefix: bound to no member.
fn pressure_operand(row: usize) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: "pressure_operand".into(),
            source: "member:1:A:pressure operand".into(),
            ordinal: row,
        },
        local_row: row,
        formula: LoadFormula::PressureOperand {
            geometry: PressureGeometry::Straight { frame: frame(0, 1) },
            pressure: 2.0,
            outer_diameter: 2.0,
            wall_thickness: 0.5,
            coefficient: PressureCoefficient::Constant(1.0),
            direction: PressureDirection::Chord { axis: 0 },
        },
        force_scale_exponent: 0,
    }
}
fn friction(dof: usize) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: "sliding_friction".into(),
            source: "member:1:A:friction".into(),
            ordinal: dof,
        },
        local_row: dof % 12,
        formula: LoadFormula::SlidingFrictionExplicit {
            global_dof: dof,
            coefficient: 0.3,
            normal: -4.0,
            direction: 1.0,
        },
        force_scale_exponent: 0,
    }
}
#[derive(Debug, Clone, Copy)]
enum Kind {
    Straight,
    Curved,
    Connector,
}
/// (prefix, node i, node j, kind): A and B are parallel members of one node
/// pair, C continues the run, D is A's pair reversed, E carries no load, F is
/// a bend and G a connector.
const MEMBERS: [(&str, usize, usize, Kind); 7] = [
    ("member:1:A:", 0, 1, Kind::Straight),
    ("member:1:B:", 0, 1, Kind::Straight),
    ("member:1:C:", 1, 2, Kind::Straight),
    ("member:1:D:", 1, 0, Kind::Straight),
    ("member:1:E:", 3, 4, Kind::Straight),
    ("member:1:F:", 2, 3, Kind::Curved),
    ("member:1:G:", 4, 2, Kind::Connector),
];
/// The case's primitives with every member's loads interleaved, so no node
/// pair's primitives are contiguous in the ledger and A's loads, whose
/// magnitudes differ by orders, stand in a definite order. Every formula
/// with a member pair is present; nodal, pressure operand and friction
/// primitives (no member pair) lie between them.
fn interleaved() -> Vec<LoadPrimitive> {
    let loads = [
        uniform(&frame(0, 1), "member:1:A:w1", [0.1, -0.3, 0.7], (0.0, 1.0)),
        uniform(&frame(1, 2), "member:1:C:w1", [0.9, 0.2, -0.4], (0.0, 1.0)),
        curved_uniform(arc(2, 3), "member:1:F:w1", [0.0, -1.0, 0.25]),
        uniform(&frame(0, 1), "member:1:B:w1", [-6.0, 12.0, 0.5], (0.125, 0.5)),
        free_strain(&frame(0, 1), "member:1:A:t1", 1.0e-4),
        connector_reference(connector(4, 2), "member:1:G:reference"),
        uniform(&frame(0, 1), "member:1:A:w2", [1.0e-3, 2.5, -1.0e-7], (0.25, 0.75)),
        uniform(&frame(1, 0), "member:1:D:w1", [0.3, 0.3, 0.3], (0.0, 0.625)),
        curved_strain(arc(2, 3), "member:1:F:t1", 2.5e-4),
        free_strain(&frame(1, 2), "member:1:C:t1", -3.0e-5),
        uniform(&frame(0, 1), "member:1:A:w3", [3.0e5, 0.2, 0.9], (0.5, 1.0)),
        curved_pressure(arc(2, 3), "member:1:F:p1", 1.5e6),
        uniform(&frame(1, 2), "member:1:C:w2", [1.0e-9, -7.0, 2.0], (0.375, 1.0)),
        curved_uniform(arc(2, 3), "member:1:F:w2", [7.0e3, 1.0e-6, -3.0]),
    ];
    let mut out = Vec::new();
    for row in 0..12 {
        for load in &loads {
            out.push(load[row].clone());
        }
        out.push(nodal(row));
        out.push(pressure_operand(row));
        out.push(friction(row));
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
/// The straight members' formation source (a bend or connector has no frame).
fn source() -> FormationSource<'static> {
    let straight = || MEMBERS.iter().filter(|m| matches!(m.3, Kind::Straight));
    FormationSource {
        node_count: NODES,
        frames: straight().map(|&(_, i, j, _)| frame(i, j)).collect(),
        frame_members: straight().map(|&(prefix, _, _, _)| prefix.into()).collect(),
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
/// What a member's recovery publishes from its loads, by kind: a straight
/// member's recovery and stress spans, a bend's arc recovery, a connector's
/// recovery.
fn recovered(loads: &CertifiedMemberLoads, member: usize) -> Vec<String> {
    let (_, i, j, kind) = MEMBERS[member];
    let u = displacements();
    let bound = bound();
    match kind {
        Kind::Straight => vec![
            format!("{:?}", certify_straight_recovery(&frame(i, j), loads, &u, &bound)),
            format!(
                "{:?}",
                certify_straight_stress_spans(&frame(i, j), loads, &u, &bound, 8.0, 3.0, None)
            ),
        ],
        Kind::Curved => vec![format!("{:?}", certify_arc_recovery(&arc(i, j), loads, &u, &bound))],
        Kind::Connector => vec![format!(
            "{:?}",
            certify_connector_recovery(&connector(i, j), loads, &u, &bound)
        )],
    }
}
/// Everything a member's loads publish, with every private midpoint and
/// radius, as text: `{:?}` of a binary64 is its shortest round-trip form (and
/// keeps the sign of zero), and the wide midpoints print their limbs.
fn published(loads: &CertifiedMemberLoads, member: usize) -> String {
    format!(
        "{:?}\n{:?}\n{:?}\n{}",
        loads.equivalent,
        loads.equivalent_scale,
        loads.uniform_loads().collect::<Vec<_>>(),
        recovered(loads, member).join("\n"),
    )
}
fn scan_loads<'a>(force: &'a AssembledForce, source: &'a FormationSource, member: usize) -> CertifiedMemberLoads<'a> {
    let (prefix, i, j, _) = MEMBERS[member];
    let loads = CertifiedMemberLoads::from_member_primitives(i, j, force.load_primitives(), prefix)
        .unwrap()
        .with_member_source(source, prefix);
    assert_eq!(loads.candidates().count(), force.load_primitives().len());
    loads
}
fn by_scan(force: &AssembledForce, source: &FormationSource, member: usize) -> String {
    published(&scan_loads(force, source, member), member)
}
fn by_index(force: &AssembledForce, source: &FormationSource, member: usize) -> String {
    let (prefix, i, j, _) = MEMBERS[member];
    let loads = CertifiedMemberLoads::from_member_force(i, j, force, prefix)
        .unwrap()
        .with_member_source(source, prefix);
    published(&loads, member)
}
/// The index is in use: each member's loads test only its node pair's
/// primitives. Each control asserts this after its numerical comparison, so
/// a fault that changes a published number fails that comparison first.
fn assert_index_in_use(force: &AssembledForce) {
    for (prefix, i, j, _) in MEMBERS {
        let loads = CertifiedMemberLoads::from_member_force(i, j, force, prefix).unwrap();
        let pair = force
            .load_primitives()
            .iter()
            .filter(|p| member_pair(&p.formula) == Some((i, j)))
            .count();
        assert_eq!(loads.candidates().count(), pair, "{prefix}: the primitives its loads test");
        assert!(pair < force.load_primitives().len());
    }
}

#[test]
fn the_member_index_publishes_what_the_whole_case_scan_publishes() {
    let source = source();
    let force = force_of(&interleaved());
    for member in 0..MEMBERS.len() {
        let scan = by_scan(&force, &source, member);
        assert!(scan.contains("Ok(") && !scan.contains("Err("), "{}: {scan}", MEMBERS[member].0);
        assert_eq!(by_index(&force, &source, member), scan, "{}", MEMBERS[member].0);
    }
    assert_index_in_use(&force);
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

/// Each formula's family and member pair, matched on the formula itself with
/// no wildcard: a new `LoadFormula` variant does not compile here until it is
/// named, and one that binds a member belongs in `MEMBER_FAMILIES` and in
/// `interleaved`.
fn family(formula: &LoadFormula) -> (&'static str, Option<(usize, usize)>) {
    match formula {
        LoadFormula::UniformStraight { frame, .. } => {
            ("UniformStraight", Some((frame.node_i.index, frame.node_j.index)))
        }
        LoadFormula::FreeStrainStraight { frame, .. } => {
            ("FreeStrainStraight", Some((frame.node_i.index, frame.node_j.index)))
        }
        LoadFormula::UniformCurved { arc, .. } => ("UniformCurved", Some((arc.node_i, arc.node_j))),
        LoadFormula::FreeStrainCurved { arc, .. } => ("FreeStrainCurved", Some((arc.node_i, arc.node_j))),
        LoadFormula::PressureFreeStrainCurved { arc, .. } => {
            ("PressureFreeStrainCurved", Some((arc.node_i, arc.node_j)))
        }
        LoadFormula::ConnectorReference { connector } => (
            "ConnectorReference",
            Some((connector.node_i().index, connector.node_j().index)),
        ),
        LoadFormula::NodalProduct { .. } => ("NodalProduct", None),
        LoadFormula::PressureOperand { .. } => ("PressureOperand", None),
        LoadFormula::JointEffectiveArea { .. } => ("JointEffectiveArea", None),
        LoadFormula::TieRodReference { .. } => ("TieRodReference", None),
        LoadFormula::SlidingFrictionExplicit { .. } => ("SlidingFrictionExplicit", None),
        LoadFormula::SlidingFrictionCoupled { .. } => ("SlidingFrictionCoupled", None),
    }
}
const MEMBER_FAMILIES: [&str; 6] = [
    "ConnectorReference",
    "FreeStrainCurved",
    "FreeStrainStraight",
    "PressureFreeStrainCurved",
    "UniformCurved",
    "UniformStraight",
];

/// The index lists exactly the ledger positions whose formula binds a member,
/// family by family, and each under its own node pair in ledger order.
#[test]
fn the_member_index_lists_every_member_bound_primitive_and_no_other() {
    let force = force_of(&interleaved());
    let primitives = force.load_primitives();
    let index = force.member_load_order();
    let mut bound_at: BTreeMap<&str, BTreeSet<usize>> = BTreeMap::new();
    let mut unbound: BTreeSet<&str> = BTreeSet::new();
    let mut want = Vec::new();
    for (k, p) in primitives.iter().enumerate() {
        match family(&p.formula) {
            (name, Some((i, j))) => {
                bound_at.entry(name).or_default().insert(k);
                want.push((i, j, k));
            }
            (name, None) => {
                unbound.insert(name);
            }
        }
    }
    // The ledger holds every member-bound family, and three that bind none.
    assert_eq!(bound_at.keys().copied().collect::<Vec<_>>(), MEMBER_FAMILIES);
    for name in ["NodalProduct", "PressureOperand", "SlidingFrictionExplicit"] {
        assert!(unbound.contains(name), "{name}");
    }
    let mut indexed_at: BTreeMap<&str, BTreeSet<usize>> = BTreeMap::new();
    for &(_, _, k) in index {
        indexed_at.entry(family(&primitives[k].formula).0).or_default().insert(k);
    }
    for name in MEMBER_FAMILIES {
        assert_eq!(indexed_at.get(name), bound_at.get(name), "{name}: its ledger positions in the member index");
    }
    for name in &unbound {
        assert_eq!(indexed_at.get(name), None, "{name} binds no member and is in no run");
    }
    want.sort_unstable();
    assert_eq!(index, &want[..]);
}

/// A's loads in pairs of opposite intensity over one span, with C's and nodal
/// primitives between them: each stress coefficient of A's sections cancels
/// to a zero midpoint at zero displacement, so its published bounds are its
/// radius alone, an upward sum taken in the order the loads are met.
fn cancelling() -> Vec<LoadPrimitive> {
    let a = frame(0, 1);
    let mut loads = Vec::new();
    for (name, w) in [
        ("big", [3.0e5, -2.0e5, 7.0e4]),
        ("small", [1.0e-3, 2.5e-3, -1.0e-4]),
        ("mid", [0.1, -0.3, 0.7]),
    ] {
        loads.push(uniform(&a, &format!("member:1:A:{name}"), w, (0.0, 1.0)));
        loads.push(uniform(&frame(1, 2), &format!("member:1:C:{name}"), w, (0.0, 1.0)));
        loads.push(uniform(&a, &format!("member:1:A:minus {name}"), w.map(|v| -v), (0.0, 1.0)));
    }
    let mut out = Vec::new();
    for row in 0..12 {
        for load in &loads {
            out.push(load[row].clone());
        }
        out.push(nodal(row));
    }
    out
}
/// A's stress spans at zero displacement and zero bound inputs.
fn resting_spans(loads: &CertifiedMemberLoads) -> Result<Vec<CertifiedStraightStressSpan>, RecoveryFailure> {
    let n = 6 * NODES;
    let rest = MemberBoundInputs {
        w: vec![0.0; n],
        w2: vec![0.0; n],
        ..bound()
    };
    certify_straight_stress_spans(&frame(0, 1), loads, &vec![0.0; n], &rest, 8.0, 3.0, None)
}

/// Every number a recovery publishes, by field name: each row's published
/// value, radius, remainder, absolute scale and rounding gap.
fn recovery_numbers(recovery: &CertifiedMemberRecovery) -> Vec<(&'static str, f64)> {
    let rows = recovery.ends.iter().chain(recovery.sections.iter().flatten());
    rows.flat_map(|row| {
        [
            ("published", row.published),
            ("radius", row.radius),
            ("remainder", row.remainder),
            ("absolute_scale", row.absolute_scale),
            ("rounding_gap", row.rounding_gap),
        ]
    })
    .collect()
}
/// Every number the stress spans publish, by field name.
fn span_numbers(spans: &[CertifiedStraightStressSpan]) -> Vec<(&'static str, f64)> {
    let mut out = Vec::new();
    for span in spans {
        out.extend([("start", span.start), ("end", span.end)]);
        for bounds in span.axial.iter().chain(&span.bending_y).chain(&span.bending_z) {
            out.extend([("lower", bounds.lower), ("upper", bounds.upper)]);
        }
        out.push(("publication_radius_pa", span.publication_radius_pa));
    }
    out
}
/// The fields whose bits differ between two equally shaped number lists.
fn differing(a: &[(&'static str, f64)], b: &[(&'static str, f64)]) -> BTreeSet<&'static str> {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .filter(|(x, y)| x.1.to_bits() != y.1.to_bits())
        .map(|(x, _)| x.0)
        .collect()
}

/// The published numbers depend on the order in which a member's primitives
/// are met, so the controls above fail if the index hands them over in
/// another order. Reversing the ledger reverses each member's primitives.
/// Compared as numbers, without the load list: A's straight recovery differs
/// in its rows' radius and absolute scale (upward sums of its equivalent
/// loads' radii and scales, taken in order; its published values are exact
/// sums and do not move). Its stress spans publish outward-rounded bounds of
/// a p128 midpoint, which an ulp of radius does not move, so they are compared
/// where A's loads cancel at rest: the coefficient bounds are then the radius
/// itself and differ, as does the publication radius.
#[test]
fn the_order_of_a_members_primitives_reaches_the_published_bits() {
    let source = source();
    let u = displacements();
    let bound = bound();
    let reversal = |primitives: Vec<LoadPrimitive>| {
        let mut reversed = primitives.clone();
        reversed.reverse();
        (force_of(&primitives), force_of(&reversed))
    };
    let (forward, reversed) = reversal(interleaved());
    let recovery = |force: &AssembledForce| {
        recovery_numbers(&certify_straight_recovery(&frame(0, 1), &scan_loads(force, &source, 0), &u, &bound).unwrap())
    };
    let differ = differing(&recovery(&forward), &recovery(&reversed));
    assert!(differ.contains("radius") && differ.contains("absolute_scale"), "the straight recovery: {differ:?}");
    // The index follows each ledger's own order.
    for member in 0..MEMBERS.len() {
        assert_eq!(by_index(&reversed, &source, member), by_scan(&reversed, &source, member));
    }
    assert_index_in_use(&reversed);

    let (forward, reversed) = reversal(cancelling());
    let spans = |loads: &CertifiedMemberLoads| span_numbers(&resting_spans(loads).unwrap());
    let differ = differing(&spans(&scan_loads(&forward, &source, 0)), &spans(&scan_loads(&reversed, &source, 0)));
    assert!(differ.contains("upper") && differ.contains("publication_radius_pa"), "the stress spans: {differ:?}");
    for force in [&forward, &reversed] {
        let (prefix, i, j, _) = MEMBERS[0];
        let indexed = CertifiedMemberLoads::from_member_force(i, j, force, prefix)
            .unwrap()
            .with_member_source(&source, prefix);
        let scanned = spans(&scan_loads(force, &source, 0));
        assert_eq!(differing(&spans(&indexed), &scanned), BTreeSet::new());
    }
}

/// A derived-normal friction force at `dof` (Stage B): no member pair either.
fn coupled_friction(dof: usize) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: "sliding_friction".into(),
            source: "member:1:A:coupled friction".into(),
            ordinal: dof,
        },
        local_row: dof % 12,
        formula: LoadFormula::SlidingFrictionCoupled {
            global_dof: dof,
            coefficient: 0.3,
            direction: -1.0,
            beta: 0.5,
            source_dof: dof + 1,
            midpoint: 2.0,
            radius: 1.0e-12,
        },
        force_scale_exponent: 0,
    }
}

/// A force-scaled copy and a force with appended terms carry an index of their
/// own primitives (`AssembledForce::force_scaled`, `with_appended`). Appended
/// sliding friction primitives (the nonlinear confirmation's appended ledger)
/// are in no member's run and change no member's published values.
#[test]
fn derived_forces_keep_a_member_index_of_their_own_primitives() {
    let source = source();
    let force = force_of(&interleaved());
    let scaled = force.force_scaled(ForceScale::new(4).unwrap()).unwrap();
    let frictions = || [friction(9), coupled_friction(14), friction(26)];
    let mut extra = LoadLedger::new();
    for primitive in uniform(&frame(0, 1), "member:1:A:appended", [0.5, 0.25, -2.0], (0.0, 0.875)) {
        extra.attach_load_primitive(primitive);
    }
    extra.attach_load_primitive(nodal(7));
    for primitive in frictions() {
        extra.attach_load_primitive(primitive);
    }
    let appended = force.with_appended(extra).unwrap();
    let mut extra = LoadLedger::new();
    for primitive in frictions() {
        extra.attach_load_primitive(primitive);
    }
    let friction_only = force.with_appended(extra).unwrap();
    let before = force.load_primitives().len();
    assert_eq!(appended.load_primitives().len(), before + 16);
    assert_eq!(friction_only.load_primitives().len(), before + 3);
    // The appended friction primitives are in no run.
    for (derived, frictions_from) in [(&appended, before + 13), (&friction_only, before)] {
        let indexed: BTreeSet<usize> = derived.member_load_order().iter().map(|&(_, _, k)| k).collect();
        for k in frictions_from..derived.load_primitives().len() {
            assert!(!indexed.contains(&k), "appended friction primitive {k}");
        }
    }
    for member in 0..MEMBERS.len() {
        assert_eq!(by_index(&scaled, &source, member), by_scan(&scaled, &source, member));
        assert_eq!(by_index(&appended, &source, member), by_scan(&appended, &source, member));
        assert_eq!(by_index(&friction_only, &source, member), by_scan(&friction_only, &source, member));
        assert_eq!(by_index(&friction_only, &source, member), by_index(&force, &source, member));
    }
    for derived in [&scaled, &appended, &friction_only] {
        assert_index_in_use(derived);
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
        let (prefix, i, j, _) = MEMBERS[member];
        let scan = CertifiedMemberLoads::from_member_primitives(i, j, force.load_primitives(), prefix);
        let index = CertifiedMemberLoads::from_member_force(i, j, &force, prefix);
        assert_eq!(scan.is_err(), refused);
        assert_eq!(format!("{:?}", scan.err()), format!("{:?}", index.err()));
    }
}

