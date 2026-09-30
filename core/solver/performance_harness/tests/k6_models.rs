//! K6 tests B: the kernel models (K6 brief, Required tests B; K6 plan §3).
//!
//! B1 compares the Rust generator's canonical bytes with the committed files
//! under `observations/k6/models/` (the 10- and 100-member RF-LARGE cases and
//! the DEC-053 nine). Those files were checked, sha256 for sha256, against the
//! independent Python generator (`runner/k6_runner.py`), whose hashes are
//! `observations/k6/models_sha256.txt` (B3, in the runner's tests). B2, the
//! comparison with R1's `references.py --model`, is a recorded one-off
//! (`T3/IMPLEMENTATION/K6/_run_records/crosscheck/`); CI cannot read R1.

use open_pipe_stress_solver_performance_harness::k6::canonical::{parse, serialize};
use open_pipe_stress_solver_performance_harness::k6::models::{
    extra_model_ids, model, p1_y_reference, rf_large_section, sealed_model_ids,
};
use std::path::PathBuf;

fn committed_ids() -> Vec<String> {
    sealed_model_ids()
        .into_iter()
        .filter(|id| {
            id.contains("-n00010-") || id.contains("-n00100-") || id.starts_with("DEC053:")
        })
        .collect()
}

fn committed_path(id: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("observations/k6/models")
        .join(format!("{}.k6model", id.replace(':', "_")))
}

/// B1: the 21 committed canonical models equal the generator's bytes.
#[test]
fn canonical_bytes_equal_committed_models() {
    let ids = committed_ids();
    assert_eq!(ids.len(), 21);
    for id in ids {
        let committed = std::fs::read_to_string(committed_path(&id))
            .unwrap_or_else(|e| panic!("{id}: committed model missing: {e}"));
        assert_eq!(
            serialize(&model(&id).expect("model")),
            committed,
            "{id}: canonical bytes differ from the committed file"
        );
    }
}

/// B4: the section formula, pinned by exact bits (explicit products and PI).
#[test]
fn section_bits_are_pinned() {
    let s = rf_large_section();
    assert_eq!(s.elastic_modulus.to_bits(), 0x4247_4876_e800_0000);
    assert_eq!(s.shear_modulus.to_bits(), 0x4232_a05f_2000_0000);
    assert_eq!(s.area.to_bits(), 0x3f78_72fa_3a37_ac1b);
    assert_eq!(s.second_moment_y.to_bits(), 0x3efc_5266_4442_2115);
    assert_eq!(s.second_moment_z.to_bits(), 0x3efc_5266_4442_2115);
    assert_eq!(s.torsion_constant.to_bits(), 0x3f0c_5266_4442_2115);
}

/// The model set: 33 sealed (24 RF-LARGE and the nine) and 6 extras.
#[test]
fn model_set_is_the_sealed_33_and_the_extras() {
    let sealed = sealed_model_ids();
    assert_eq!(sealed.len(), 33);
    assert_eq!(
        sealed
            .iter()
            .filter(|id| id.starts_with("RF-LARGE-"))
            .count(),
        24
    );
    assert_eq!(
        sealed.iter().filter(|id| id.starts_with("DEC053:")).count(),
        9
    );
    assert_eq!(extra_model_ids().len(), 6);
}

/// Every model's shape follows R1's rules: nodes, members, R1's node order
/// (CONT lists its supports before its midspans), and the member count.
#[test]
fn model_shapes_follow_r1() {
    for n in [10usize, 100] {
        for orientation in ["AX", "ROT"] {
            let chain = model(&format!("RF-LARGE-CHAIN-n{n:05}-{orientation}")).unwrap();
            assert_eq!((chain.node_count(), chain.member_count()), (n + 1, n));
            assert_eq!(chain.nodes[n].0, format!("N{n}"));
            let tree = model(&format!("RF-LARGE-TREE-n{n:05}-{orientation}")).unwrap();
            assert_eq!((tree.node_count(), tree.member_count()), (n + 1, n));
            assert_eq!(
                (tree.nodes[1].0.as_str(), tree.nodes[2].0.as_str()),
                ("P1", "B1")
            );
            assert_eq!(tree.members[1].0, "Q1");
            let cont = model(&format!("RF-LARGE-CONT-n{n:05}-{orientation}")).unwrap();
            assert_eq!((cont.node_count(), cont.member_count()), (n + 1, n));
            assert_eq!(cont.nodes[n / 2].0, format!("S{}", n / 2));
            assert_eq!(cont.nodes[n / 2 + 1].0, "C1");
            assert_eq!(
                cont.members[0],
                ("A1".to_string(), 0, n / 2 + 1, cont.members[0].3)
            );
            assert_eq!(cont.restrained_dofs().len(), 6 + 3 * (n / 2));
        }
    }
}

/// R1's loads at 10 members, as `references.json` states them (AX and ROT).
#[test]
fn loads_match_r1_at_ten_members() {
    let chain = model("RF-LARGE-CHAIN-n00010-AX").unwrap();
    assert_eq!(
        chain.loads,
        vec![
            (60, 0.09375),
            (61, -0.046875),
            (62, 0.140625),
            (63, 6.0),
            (64, 12.0),
            (65, -6.0)
        ]
    );
    let chain = model("RF-LARGE-CHAIN-n00010-ROT").unwrap();
    assert_eq!(
        chain.loads,
        vec![
            (60, 0.09375),
            (61, -0.046875),
            (62, -0.140625),
            (63, 6.0),
            (64, 12.0),
            (65, 6.0)
        ]
    );
    let cont = model("RF-LARGE-CONT-n00010-ROT").unwrap();
    // C1 is node 6: F = (-64, -32, -64).
    assert_eq!(&cont.loads[..3], &[(36, -64.0), (37, -32.0), (38, -64.0)]);
    let tree = model("RF-LARGE-TREE-n00010-ROT").unwrap();
    // B1 is node 2: F = (-4, 4, 2).
    assert_eq!(&tree.loads[..3], &[(12, -4.0), (13, 4.0), (14, 2.0)]);
}

/// P1's y_reference rule: the least-aligned axis, Y, then Z, then X on ties.
#[test]
fn p1_y_reference_rule() {
    assert_eq!(p1_y_reference([0, 0, 0], [3, 0, 0]), [0.0, 1.0, 0.0]);
    assert_eq!(p1_y_reference([0, 0, 0], [1, 2, -2]), [1.0, 0.0, 0.0]);
    assert_eq!(p1_y_reference([3, 0, 0], [3, 3, 0]), [0.0, 0.0, 1.0]);
    assert_eq!(p1_y_reference([0, 0, 0], [0, 0, 3]), [0.0, 1.0, 0.0]);
    assert_eq!(p1_y_reference([0, 0, 0], [2, 1, 2]), [0.0, 1.0, 0.0]);
}

/// The canonical form parses back to the same model and bytes.
#[test]
fn canonical_round_trip() {
    for id in committed_ids() {
        let m = model(&id).unwrap();
        let text = serialize(&m);
        let parsed = parse(&text).expect("parse");
        assert_eq!(parsed, m, "{id}");
    }
}
