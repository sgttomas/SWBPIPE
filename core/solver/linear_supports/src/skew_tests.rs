//! S-K1 step 5 (DEL-04-03 Work `skew-restraints.md` §15): the basis built
//! from support records equals the kernel's, and the record and node codes.
use super::*;

fn stop(id: &str, node: usize, direction: [f64; 3]) -> SkewSupport {
    SkewSupport {
        support_id: id.into(),
        family: SkewFamily::LineStop,
        node_index: node,
        direction,
        stiffness: None,
    }
}

fn spring(id: &str, node: usize, direction: [f64; 3], k: f64) -> SkewSupport {
    SkewSupport {
        support_id: id.into(),
        family: SkewFamily::Spring,
        node_index: node,
        direction,
        stiffness: Some(SupportQuantity::new(k, QuantityDimension::TranslationalStiffness).unwrap()),
    }
}

/// A rigid record holding a node's three rotations.
fn rotations(id: &str, node: usize) -> LinearSupport {
    LinearSupport {
        restrained_dofs: vec![FrameDof::Rx, FrameDof::Ry, FrameDof::Rz],
        ..LinearSupport::anchor(id, node)
    }
}

fn kernel(node_count: usize, prepared: &BoundaryPreparation, skew: &[SkewSupport]) -> RestraintBasis {
    let stops = skew
        .iter()
        .filter(|s| s.family == SkewFamily::LineStop)
        .map(|s| SkewStop {
            support_id: s.support_id.clone(),
            node: s.node_index,
            direction: SkewDirection::new(s.direction).unwrap(),
        })
        .collect();
    let springs = skew
        .iter()
        .filter(|s| s.family == SkewFamily::Spring)
        .map(|s| SkewSpring {
            support_id: s.support_id.clone(),
            node: s.node_index,
            direction: SkewDirection::new(s.direction).unwrap(),
            stiffness: s.stiffness.as_ref().unwrap().value,
        })
        .collect();
    RestraintBasis::build(node_count, &prepared.restrained_dofs, stops, springs).unwrap()
}

#[test]
fn basis_from_records_equals_the_kernel_basis() {
    // The guided cantilever's shape: anchor at A, rotations and UZ held at B,
    // a skewed stop and a skewed spring at B.
    let linear = vec![
        LinearSupport::anchor("A", 0),
        rotations("rot_B", 1),
        LinearSupport::line_stop("guide_uz", 1, FrameDof::Uz),
    ];
    let prepared = prepare_boundary(2, &linear);
    assert!(!prepared.is_blocked());
    let skew = [stop("stop", 1, [3.0, 4.0, 5.0]), spring("spring", 1, [0.0, 3.0, 4.0], 2.0)];
    let basis = prepare_skew_boundary(2, &prepared, &skew).unwrap();
    assert_eq!(basis, kernel(2, &prepared, &skew));
    assert_eq!(basis.axis_restrained(), prepared.restrained_dofs.as_slice());
    let node = basis.node_basis(1).unwrap();
    assert_eq!((node.free_axes, node.pivot, node.rank()), ([true, true, false], 1, 2));
    // The pivot coordinate joins the partition; no physical DOF is restrained twice.
    assert!(basis.restrained_coordinates().contains(&7));
    assert!(!prepared.restrained_dofs.contains(&7));
    // R4_tiny_restricted_direction from records: ŝ = 600, w = (2^600, 1, 0).
    let linear = vec![
        rotations("rot", 0),
        LinearSupport::line_stop("ux", 0, FrameDof::Ux),
        LinearSupport::line_stop("uz", 0, FrameDof::Uz),
    ];
    let prepared = prepare_boundary(1, &linear);
    let tiny = 2.0_f64.powi(-600);
    let skew = [stop("stop", 0, [1.0, tiny, 0.0])];
    let basis = prepare_skew_boundary(1, &prepared, &skew).unwrap();
    assert_eq!(basis, kernel(1, &prepared, &skew));
    let node = basis.node_basis(0).unwrap();
    assert_eq!(node.restricted_scale_exponent, 600);
    assert_eq!(node.w, [2.0_f64.powi(600), 1.0, 0.0]);
    // Springs only: no constraint transform.
    let prepared = prepare_boundary(1, &[LinearSupport::anchor("A", 0)]);
    let basis = prepare_skew_boundary(1, &prepared, &[spring("s", 0, [1.0, 1.0, 0.0], 1.0)]).unwrap();
    assert!(!basis.has_constraint_transform());
}

#[test]
fn record_findings_name_the_record() {
    let prepared = prepare_boundary(2, &[LinearSupport::anchor("A", 0)]);
    let subnormal = f64::from_bits(1u64 << 51); // 2^-1023
    let findings = prepare_skew_boundary(
        2,
        &prepared,
        &[
            stop("range", 1, [1.0, subnormal, 0.0]),
            stop("axis", 1, [0.0, -2.0, 0.0]),
            stop("far", 2, [1.0, 1.0, 0.0]),
            SkewSupport { stiffness: None, ..spring("bare", 1, [1.0, 1.0, 0.0], 1.0) },
            SkewSupport {
                stiffness: Some(SupportQuantity::new(1.0, QuantityDimension::RotationalStiffness).unwrap()),
                ..spring("rotational", 1, [1.0, 1.0, 0.0], 1.0)
            },
            spring("negative", 1, [1.0, 1.0, 0.0], -1.0),
        ],
    )
    .unwrap_err();
    let got: Vec<(FindingCode, &str)> = findings.iter().map(|f| (f.code, f.support_id.as_str())).collect();
    assert_eq!(
        got,
        vec![
            (FindingCode::SkewDirectionRange, "range"),
            (FindingCode::InvalidSupportDof, "axis"),
            (FindingCode::NodeOutOfRange, "far"),
            (FindingCode::MissingSupportStiffness, "bare"),
            (FindingCode::InvalidSupportDimension, "rotational"),
            (FindingCode::MissingSupportStiffness, "negative"),
        ]
    );
    assert!(findings[0].message.contains("y are subnormal"), "{}", findings[0].message);
    assert!(findings[0].message.contains("axis-aligned direction is the authored form"));
}

#[test]
fn node_findings_stops_per_node_and_redundant() {
    let linear = vec![
        LinearSupport::anchor("A", 0),
        LinearSupport::line_stop("ux", 2, FrameDof::Ux),
        LinearSupport::line_stop("uy", 2, FrameDof::Uy),
    ];
    let prepared = prepare_boundary(3, &linear);
    let findings = prepare_skew_boundary(
        3,
        &prepared,
        &[
            stop("redundant", 2, [3.0, 4.0, 0.0]),
            stop("first", 1, [3.0, 4.0, 0.0]),
            stop("second", 1, [4.0, 3.0, 0.0]),
        ],
    )
    .unwrap_err();
    let got: Vec<(FindingCode, &str)> = findings.iter().map(|f| (f.code, f.support_id.as_str())).collect();
    assert_eq!(
        got,
        vec![(FindingCode::SkewStopsPerNode, "first"), (FindingCode::ConstraintRedundant, "redundant")]
    );
    assert!(findings[0].message.contains("first, second"));
    // The existing boundary is unchanged by the skew preparation.
    assert_eq!(prepare_boundary(3, &linear), prepared);
}
