//! K4 tests of `retained/source.rs`: validation, canonical order and bodies.
use super::*;

fn member(id: u32, i: u32, j: u32) -> StraightMember {
    StraightMember {
        id,
        node_i: i,
        node_j: j,
        elastic_modulus: 200e9,
        shear_modulus: 80e9,
        area: 6e-3,
        second_moment_y: 2.7e-5,
        second_moment_z: 2.7e-5,
        torsion_constant: 5.4e-5,
        y_reference: [0.0, 0.0, 1.0],
    }
}

fn dof(node: u32, c: usize) -> Dof {
    Dof {
        node,
        component: Component::from_index(c),
    }
}

fn base() -> SourceParts {
    SourceParts {
        nodes: vec![
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [4.0, 0.0, 0.0],
            [9.0, 9.0, 9.0],
            [11.0, 9.0, 9.0],
        ],
        members: vec![member(2, 1, 2), member(1, 0, 1), member(7, 3, 4)],
        springs: vec![Spring {
            id: 4,
            dof: dof(2, 1),
            stiffness: 1e3,
        }],
        directional_springs: vec![DirectionalSpring {
            id: 5,
            node: 2,
            kind: SpringKind::Rotation,
            direction: [1.0, 1.0, 0.0],
            stiffness: 1e3,
        }],
        constraints: (0..6)
            .map(|c| Constraint {
                dof: dof(0, c),
                value: 0.0,
            })
            .collect(),
        loads: vec![NodalLoad {
            dof: dof(2, 1),
            value: 1.0,
            source_id: "w".into(),
        }],
        stations: vec![Station {
            id: 1,
            member: 2,
            fraction: 0.25,
        }],
        supports: vec![SupportGroup {
            id: 1,
            node: 2,
            restrained: [false; 6],
            springs: vec![4],
            directional_springs: vec![5],
        }],
    }
}

fn refused(edit: impl FnOnce(&mut SourceParts)) -> SourceError {
    let mut parts = base();
    edit(&mut parts);
    PrimitiveSource::new(parts).expect_err("refused")
}

#[test]
fn a_valid_source_is_canonical_and_its_bodies_are_numbered_by_lowest_node() {
    let source = PrimitiveSource::new(base()).unwrap();
    assert_eq!(
        source.members().iter().map(|m| m.id).collect::<Vec<_>>(),
        vec![1, 2, 7]
    );
    assert_eq!(source.body_count(), 2);
    assert_eq!(
        (0..5).map(|n| source.body_of_node(n)).collect::<Vec<_>>(),
        vec![0, 0, 0, 1, 1]
    );
    assert_eq!(source.body_nodes(1), vec![3, 4]);
    assert_eq!(source.free_dofs().len(), 24);
    assert_eq!(source.constraint(3), Some(0.0));
    assert_eq!(source.member_index(7), Some(2));
}

#[test]
fn every_invalid_input_is_refused_with_its_reason() {
    use MemberProperty as P;
    assert_eq!(refused(|p| p.nodes.clear()), SourceError::NoNodes);
    assert_eq!(
        refused(|p| p.nodes[1][2] = f64::NAN),
        SourceError::NonFiniteCoordinate { node: 1 }
    );
    assert_eq!(
        refused(|p| p.members[0].node_j = 9),
        SourceError::NodeOutOfRange { node: 9 }
    );
    assert_eq!(
        refused(|p| p.members[2].id = 1),
        SourceError::DuplicateMemberId { id: 1 }
    );
    assert_eq!(
        refused(|p| p.members[0].node_j = 1),
        SourceError::RepeatedMemberNode { member: 2 }
    );
    assert_eq!(
        refused(|p| p.members[0].elastic_modulus = f64::INFINITY),
        SourceError::NonFiniteProperty {
            member: 2,
            property: P::ElasticModulus
        }
    );
    assert_eq!(
        refused(|p| p.members[0].area = 0.0),
        SourceError::NonPositiveProperty {
            member: 2,
            property: P::Area
        }
    );
    assert_eq!(
        refused(|p| p.members[0].torsion_constant = f64::from_bits(1)),
        SourceError::SubnormalDerivedPrimitive {
            member: 2,
            property: P::TorsionConstant
        }
    );
    assert_eq!(
        refused(|p| p.members[0].y_reference = [2.0, 0.0, 0.0]),
        SourceError::DegenerateAxis { member: 2 }
    );
    assert_eq!(
        refused(|p| p.nodes[2] = p.nodes[1]),
        SourceError::ZeroLength { member: 2 }
    );
    assert_eq!(
        refused(|p| p.springs[0].stiffness = 0.0),
        SourceError::NonPositiveSpring { id: 4 }
    );
    assert_eq!(
        refused(|p| p.directional_springs[0].id = 4),
        SourceError::DuplicateSpringId { id: 4 }
    );
    assert_eq!(
        refused(|p| p.directional_springs[0].direction = [0.0; 3]),
        SourceError::ZeroDirection { id: 5 }
    );
    assert_eq!(
        refused(|p| p.directional_springs[0].direction[1] = f64::NAN),
        SourceError::NonFiniteDirection { id: 5 }
    );
    assert_eq!(
        refused(|p| p.constraints.push(Constraint {
            dof: dof(0, 3),
            value: 1.0
        })),
        SourceError::DuplicateConstraint { dof: dof(0, 3) }
    );
    assert_eq!(
        refused(|p| p.loads[0].source_id.clear()),
        SourceError::EmptyLoadSource { dof: dof(2, 1) }
    );
    assert_eq!(
        refused(|p| p.stations[0].member = 3),
        SourceError::UnknownMember {
            station: 1,
            member: 3
        }
    );
    assert_eq!(
        refused(|p| p.stations[0].fraction = 1.5),
        SourceError::StationOutOfRange { id: 1 }
    );
    assert_eq!(
        refused(|p| p.supports[0].restrained[0] = true),
        SourceError::SupportMismatch { id: 1 }
    );
    assert_eq!(
        refused(|p| p.supports[0].springs = vec![9]),
        SourceError::SupportMismatch { id: 1 }
    );
}

#[test]
fn the_degenerate_axis_decision_is_exact() {
    // chord (1 + 2^-52, 1, 0) and y (1, 1 − 2^-53, 0): the exact cross product
    // is 2^-53 − 2^-105 ≠ 0, while binary64 evaluates it as 0.
    let (dx, yy) = (1.0 + f64::EPSILON, 1.0 - f64::EPSILON / 2.0);
    assert_eq!(dx * yy - 1.0, 0.0, "binary64 would call it parallel");
    // Member 7 runs from node 3 to node 4.
    let mut parts = base();
    parts.nodes[3] = [0.0, 5.0, 0.0];
    parts.nodes[4] = [dx, 6.0, 0.0];
    parts.members[2].y_reference = [1.0, yy, 0.0];
    assert!(PrimitiveSource::new(parts).is_ok());
    // Exactly parallel (a scaled chord): refused.
    let mut parts = base();
    parts.nodes[3] = [0.0, 5.0, 0.0];
    parts.nodes[4] = [dx, 6.0, 0.0];
    parts.members[2].y_reference = [2.0 * dx, 2.0, 0.0];
    assert_eq!(
        PrimitiveSource::new(parts).unwrap_err(),
        SourceError::DegenerateAxis { member: 7 }
    );
}

#[test]
fn the_source_encodings_do_not_depend_on_list_order() {
    let a = PrimitiveSource::new(base()).unwrap();
    let mut parts = base();
    parts.members.reverse();
    parts.constraints.reverse();
    parts.loads.push(NodalLoad {
        dof: dof(1, 0),
        value: 2.0,
        source_id: "a".into(),
    });
    let b = PrimitiveSource::new(parts.clone()).unwrap();
    parts.loads.reverse();
    let c = PrimitiveSource::new(parts).unwrap();
    assert_eq!(b.encoding(), c.encoding());
    assert_ne!(
        a.encoding(),
        b.encoding(),
        "a new load changes the source encoding"
    );
    assert_eq!(
        a.stiffness_encoding(),
        b.stiffness_encoding(),
        "but not the stiffness identity"
    );
}
