//! The near-duplicate check's tolerance parameter and its cell grid
//! (DEL-04-06 `Design/model-topology-checks.md`). The runtime tests in
//! `tests/topology_checks_runtime.rs` cover the default through the desktop's
//! value entry.

use super::*;
use crate::DEFAULT_NODE_PROXIMITY_TOLERANCE_M as NODE_PROXIMITY_DEFAULT_M;

fn codes(coordinates: &[[f64; 3]], edges: &[(usize, usize)], tolerance: f64) -> Vec<Diagnostic> {
    let node_id = |index: usize| format!("node:{index}");
    let mut diagnostics = Vec::new();
    near_duplicate_nodes(coordinates, edges, &node_id, tolerance, &mut diagnostics);
    diagnostics
}

fn count(diagnostics: &[Diagnostic], code: &str) -> usize {
    diagnostics.iter().filter(|d| d.code == code).count()
}

#[test]
fn the_tolerance_is_the_callers_value_inclusive() {
    let pair = [[0.0, 0.0, 0.0], [0.003, 0.0, 0.0]];
    assert_eq!(count(&codes(&pair, &[], 0.002), "NODE_NEAR_DUPLICATE"), 0);
    assert_eq!(count(&codes(&pair, &[], 0.003), "NODE_NEAR_DUPLICATE"), 1);
    assert_eq!(count(&codes(&pair, &[], NODE_PROXIMITY_DEFAULT_M), "NODE_NEAR_DUPLICATE"), 1);
    let found = codes(&pair, &[], 0.003);
    assert!(found[0].message.contains("node-proximity tolerance (3e-3 m)"), "{}", found[0].message);
    // A joined pair is the element's to judge.
    assert!(codes(&pair, &[(0, 1)], 0.003).is_empty());
}

#[test]
fn a_pair_across_a_cell_boundary_is_found() {
    // Cells are 8 x 5 mm = 0.04 m wide; the pair straddles x = 0.04.
    let pair = [[0.038, 0.0, 0.0], [0.042, 0.0, 0.0]];
    assert_eq!(count(&codes(&pair, &[], NODE_PROXIMITY_DEFAULT_M), "NODE_NEAR_DUPLICATE"), 1);
}

#[test]
fn a_node_far_from_the_origin_does_not_move_a_near_pair_apart() {
    // Offset by the lowest coordinate (-4e15 m, where the spacing is 0.5 m),
    // 0.248 and 0.252 round to 4e15 and 4e15 + 0.5: twelve cells apart.
    let nodes = [[-4.0e15, 0.0, 0.0], [0.248, 0.0, 0.0], [0.252, 0.0, 0.0]];
    let found = codes(&nodes, &[], NODE_PROXIMITY_DEFAULT_M);
    assert_eq!(count(&found, "NODE_NEAR_DUPLICATE"), 1);
    assert!(found[0].message.starts_with("1 pair "), "{}", found[0].message);
}

#[test]
fn a_saturated_cell_index_counts_each_pair_once() {
    // 4e15 / (8 x 1e-300) overflows i64: both nodes saturate into one cell.
    let pair = [[4.0e15, 0.0, 0.0], [4.0e15, 0.0, 0.0]];
    let found = codes(&pair, &[], 1e-300);
    assert_eq!(count(&found, "NODE_COINCIDENT"), 1);
    assert!(found[0].message.starts_with("1 pair "), "{}", found[0].message);
}
