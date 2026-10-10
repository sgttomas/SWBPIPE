//! T4-U3 slots S1, S2 and S8 for the objective connector: the dense and
//! sparse assemblies agree bit for bit in the dense order (frames, user
//! elements, connectors, blocks, springs), K2b forms the connector's Ke at
//! 2^b by `force_scaled_matrix` (N-5) so the sparse matrix at 2^b is 2^b
//! times the unscaled one bit for bit, and the census takes K and Ke.
//! Invented inputs only.
use open_pipe_stress_frame_kernel::connector::{
    ConnectorAttachment, ObjectiveConnector, ScaledWorkMatrix,
};
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, assemble_sparse_stiffness_with_connectors, SparseAssemblyOptions,
    StiffnessBlock,
};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, assemble_global_stiffness_with_connectors, ForceScale,
    ForceScaleCensus, FrameElement, FrameNode, FrameSection,
};

fn generic_connector(i: usize, j: usize, xi: [f64; 3]) -> ObjectiveConnector {
    let (third, two_thirds) = (1.0 / 3.0, 2.0 / 3.0);
    let h = [
        12500.0, 625.0, 0.0, 0.0, 500.0, 0.0, 9375.0, 312.5, 0.0, 0.0, -750.0, 7500.0, 250.0, 0.0,
        0.0, 800.0, 50.0, 0.0, 900.0, 100.0, 1200.0,
    ];
    // r = (1.3, 2.6, 2.6) along Q.x = (1, 2, 2)/3.
    ObjectiveConnector::new(
        FrameNode::new(i, xi).unwrap(),
        FrameNode::new(j, [xi[0] + 1.0, xi[1] + 2.0, xi[2] + 2.0]).unwrap(),
        ConnectorAttachment::global([0.1, 0.2, 0.2]),
        ConnectorAttachment::global([0.4, 0.8, 0.8]),
        [
            [third, two_thirds, -two_thirds],
            [two_thirds, third, two_thirds],
            [two_thirds, -two_thirds, -third],
        ],
        ScaledWorkMatrix {
            upper_triangle: h,
            translation_scale: 0.25,
        },
        [1e-3, -5e-4, 2.5e-4, 2e-3, 0.0, -1e-3],
    )
    .unwrap()
}

fn model() -> (Vec<FrameElement>, Vec<ObjectiveConnector>) {
    let section = FrameSection::new(200e9, 80e9, 6.0e-3, 2.7e-5, 2.7e-5, 5.4e-5).unwrap();
    let p = [
        [0.0, 0.0, 0.0],
        [3.0, 0.0, 0.0],
        [4.0, 2.0, 2.0],
        [7.0, 2.0, 2.0],
    ];
    let node = |k: usize| FrameNode::new(k, p[k]).unwrap();
    let frames = vec![
        FrameElement::new(node(0), node(1), section, [0.0, 0.0, 1.0]).unwrap(),
        FrameElement::new(node(2), node(3), section, [0.0, 0.0, 1.0]).unwrap(),
    ];
    // The connector joins nodes 1 and 2 (offsets bring it from 3.0 to 4.0).
    let mut connector = generic_connector(1, 2, p[1]);
    assert_eq!(connector.node_j().coordinates, p[2]);
    connector = ObjectiveConnector::new(
        node(1),
        node(2),
        ConnectorAttachment::global(connector.offsets().0),
        ConnectorAttachment::global(connector.offsets().1),
        connector.axes(),
        connector.work_matrix(),
        connector.q_ref(),
    )
    .unwrap();
    (frames, vec![connector])
}

fn bits(m: &[Vec<f64>]) -> Vec<u64> {
    m.iter().flatten().map(|v| v.to_bits()).collect()
}

#[test]
fn dense_and_sparse_agree_bit_for_bit_with_connectors() {
    let (frames, connectors) = model();
    let dense = assemble_global_stiffness_with_connectors(4, &frames, &connectors).unwrap();
    let sparse = assemble_sparse_stiffness_with_connectors(
        4,
        &frames,
        &[],
        &connectors,
        &[],
        &[],
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    assert_eq!(bits(&sparse.to_dense()), bits(&dense));
    // The connector is present: it couples nodes 1 and 2.
    assert_ne!(dense[6][12], 0.0);
    // With no connector both entries are today's, bit for bit.
    assert_eq!(
        bits(&assemble_global_stiffness_with_connectors(4, &frames, &[]).unwrap()),
        bits(&assemble_global_stiffness(4, &frames).unwrap())
    );
    let options = SparseAssemblyOptions::new();
    assert_eq!(
        assemble_sparse_stiffness_with_connectors(4, &frames, &[], &[], &[], &[], &options)
            .unwrap(),
        assemble_sparse_stiffness(4, &frames, &[], &[], &[], &options).unwrap()
    );
    // Blocks and springs come after the connectors, as in the product.
    let block = StiffnessBlock {
        node_i: 0,
        node_j: 3,
        stiffness: connectors[0].global_stiffness().unwrap(),
    };
    let with_tail = assemble_sparse_stiffness_with_connectors(
        4,
        &frames,
        &[],
        &connectors,
        &[block],
        &[(0, 1.0e7)],
        &options,
    )
    .unwrap()
    .to_dense();
    let mut expected = dense.clone();
    let map: Vec<usize> = (0..6).chain(18..24).collect();
    for r in 0..12 {
        for c in 0..12 {
            expected[map[r]][map[c]] += block.stiffness[r][c];
        }
    }
    expected[0][0] += 1.0e7;
    assert_eq!(bits(&with_tail), bits(&expected));
}

#[test]
fn k2b_scales_the_formed_connector_exactly() {
    let (frames, connectors) = model();
    let unscaled = assemble_sparse_stiffness_with_connectors(
        4,
        &frames,
        &[],
        &connectors,
        &[],
        &[(0, 1.0e7)],
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    for b in [-40, 20, 64] {
        let scale = ForceScale::new(b).unwrap();
        let scaled = assemble_sparse_stiffness_with_connectors(
            4,
            &frames,
            &[],
            &connectors,
            &[],
            &[(0, 1.0e7)],
            &SparseAssemblyOptions::new().with_force_scale(scale),
        )
        .unwrap();
        let factor = 2f64.powi(b);
        assert_eq!(scaled.pattern(), unscaled.pattern());
        for (s, u) in scaled.values().iter().zip(unscaled.values()) {
            assert_eq!(s.to_bits(), (u * factor).to_bits(), "b = {b}");
        }
    }
}

#[test]
fn census_takes_the_connector_stiffness_and_formed_matrix() {
    let (_, connectors) = model();
    let mut census = ForceScaleCensus::new();
    census.connector(&connectors[0]).unwrap();
    let k = connectors[0].stiffness();
    let ke = connectors[0].global_stiffness().unwrap();
    let exponent = |v: f64| ((v.abs().to_bits() >> 52) as i32) - 1023;
    let values = k
        .iter()
        .flatten()
        .chain(ke.iter().flatten())
        .filter(|v| **v != 0.0);
    let low = values.clone().map(|v| exponent(*v)).min().unwrap();
    let high = values.map(|v| exponent(*v)).max().unwrap();
    assert_eq!(census.span(), Some((low, high)));
    assert!(!census.has_subnormal());
}
