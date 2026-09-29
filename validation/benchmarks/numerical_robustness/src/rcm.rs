//! RCM equality (K4's brief Q7; D1 stale item 4; plan §11): K4's port
//! `reverse_cuthill_mckee` (`K4R/factor.rs`) and `sparse_direct`'s
//! (`SD/lib.rs`) must give the same order on the same adjacency. V-K builds
//! the adjacency itself: the free–free DOF graph of a model (members' 12×12
//! blocks, spring diagonals, directional springs' 3×3 node blocks), with the
//! restrained DOFs removed and the free DOFs numbered in ascending order.
use crate::cases::Model;
use open_pipe_stress_frame_kernel::structural::retained_api::reverse_cuthill_mckee as k4_rcm;
use open_pipe_stress_sparse_direct::reverse_cuthill_mckee as sd_rcm;
use std::collections::BTreeSet;

/// The free–free adjacency of a model's structural pattern.
pub fn free_adjacency(model: &Model) -> Vec<Vec<usize>> {
    let n = model.nodes.len() * 6;
    let mut restrained = vec![false; n];
    for &(node, c) in &model.constraints {
        restrained[node as usize * 6 + c] = true;
    }
    let mut position = vec![usize::MAX; n];
    let mut count = 0;
    for g in 0..n {
        if !restrained[g] {
            position[g] = count;
            count += 1;
        }
    }
    let mut sets: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); count];
    let mut couple = |dofs: &[usize]| {
        for &a in dofs {
            for &b in dofs {
                if a != b && position[a] != usize::MAX && position[b] != usize::MAX {
                    sets[position[a]].insert(position[b]);
                }
            }
        }
    };
    for m in &model.members {
        let dofs: Vec<usize> = [m.node_i, m.node_j]
            .iter()
            .flat_map(|&k| (0..6).map(move |c| k as usize * 6 + c))
            .collect();
        couple(&dofs);
    }
    for s in &model.springs {
        if s.axis.is_none() {
            let off = if s.translation { 0 } else { 3 };
            let dofs: Vec<usize> = (0..3).map(|c| s.node as usize * 6 + off + c).collect();
            couple(&dofs);
        }
    }
    sets.into_iter().map(|s| s.into_iter().collect()).collect()
}

/// K4's and SD's orders on one adjacency (valid input only).
pub fn both_orders(adjacency: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    (
        k4_rcm(adjacency),
        sd_rcm(adjacency).expect("valid adjacency"),
    )
}
