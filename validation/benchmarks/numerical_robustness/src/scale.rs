//! V-K's scale runs (checkpoint B; plan §13): W1's deterministic counts of a
//! model, in O(nnz) and without a solve, and the admission estimate.
//!
//! - **The counts** follow K4's own rules. The structural pattern is the
//!   members' node blocks plus the springs at nodes no member touches (K4R
//!   `assemble.rs` `Structure`). The profile is K4's skyline rule
//!   (`K4R/factor.rs` `order_free`: first = the least rank over a row's
//!   neighbours and itself) over the exported `reverse_cuthill_mckee`, on the
//!   free–free adjacency (`rcm::free_adjacency`). `tests/scale.rs` checks both
//!   against K4's `StorageCounts` in every committed record.
//! - **The estimate** is the complete checked VR caller/shared-kernel composition
//!   in `envelope`, under explicitly named conditional reference facts. These
//!   facts do not qualify the current executable or supply a measured bound.
use crate::cases::{Member, Model, SpringSpec};
use crate::rcm::free_adjacency;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    layout, reverse_cuthill_mckee, AttemptRecord, Constraint, NodalLoad, PrimitiveSource,
    PublishedRow, QuantityId, QuantityMeta, Station, StraightMember,
};
use std::collections::BTreeSet;

const DOF: usize = 6;

/// W1's deterministic counts of one model (K6b's `W1Counts`, the same fields).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counts {
    pub nodes: usize,
    pub members: usize,
    pub springs: usize,
    pub stations: usize,
    pub constraints: usize,
    pub loads: usize,
    pub dofs: usize,
    pub free_dofs: usize,
    pub bodies: usize,
    /// Stored entries of K4's structural pattern (both triangles).
    pub pattern_entries: usize,
    /// K4's skyline of the RCM-ordered free–free pattern.
    pub profile_entries: usize,
    pub half_bandwidth: usize,
    /// Connected components of the free–free adjacency (K4's blocks).
    pub blocks: usize,
    /// The published layout's length (`layout(&source).len()`).
    pub rows: usize,
    /// K4SRC's length (`PrimitiveSource::encoding`).
    pub source_encoding_len: usize,
}

/// K4's structural pattern entries: 36 per node a member touches, 36 per
/// ordered pair of member-adjacent nodes, and the spring entries at nodes no
/// member touches (a global-axis spring's diagonal; a directional spring's
/// 3×3 block), each entry once.
pub fn pattern_entries(model: &Model) -> usize {
    let n = model.nodes.len();
    let mut touched = vec![false; n];
    let mut pairs: BTreeSet<(u32, u32)> = BTreeSet::new();
    for m in &model.members {
        touched[m.node_i as usize] = true;
        touched[m.node_j as usize] = true;
        if m.node_i != m.node_j {
            pairs.insert((m.node_i, m.node_j));
            pairs.insert((m.node_j, m.node_i));
        }
    }
    let mut spring_entries: BTreeSet<(u32, usize, usize)> = BTreeSet::new();
    for s in &model.springs {
        if touched[s.node as usize] {
            continue;
        }
        match s.axis {
            Some(a) => {
                spring_entries.insert((s.node, a, a));
            }
            None => {
                let off = if s.translation { 0 } else { 3 };
                for a in 0..3 {
                    for b in 0..3 {
                        spring_entries.insert((s.node, off + a, off + b));
                    }
                }
            }
        }
    }
    let touched = touched.iter().filter(|&&t| t).count();
    DOF * DOF * (touched + pairs.len()) + spring_entries.len()
}

/// K4's skyline on the free–free adjacency: (profile entries, half-bandwidth,
/// blocks).
pub fn profile(adjacency: &[Vec<usize>]) -> (usize, usize, usize) {
    let order = reverse_cuthill_mckee(adjacency);
    let mut rank = vec![0usize; adjacency.len()];
    for (k, &a) in order.iter().enumerate() {
        rank[a] = k;
    }
    let (mut entries, mut bandwidth) = (0usize, 0usize);
    for (i, &a) in order.iter().enumerate() {
        let first = adjacency[a]
            .iter()
            .map(|&b| rank[b])
            .chain(std::iter::once(i))
            .min()
            .unwrap_or(i);
        entries += i - first + 1;
        bandwidth = bandwidth.max(i - first);
    }
    let mut block = vec![usize::MAX; adjacency.len()];
    let mut blocks = 0;
    let mut stack = Vec::new();
    for seed in 0..adjacency.len() {
        if block[seed] != usize::MAX {
            continue;
        }
        block[seed] = blocks;
        stack.push(seed);
        while let Some(a) = stack.pop() {
            for &b in &adjacency[a] {
                if block[b] == usize::MAX {
                    block[b] = blocks;
                    stack.push(b);
                }
            }
        }
        blocks += 1;
    }
    (entries, bandwidth, blocks)
}

/// The counts of `model` and its source, with no solve.
pub fn counts(model: &Model, source: &PrimitiveSource) -> Counts {
    let (profile_entries, half_bandwidth, blocks) = profile(&free_adjacency(model));
    Counts {
        nodes: source.node_count(),
        members: source.members().len(),
        springs: model.springs.len(),
        stations: source.stations().len(),
        constraints: source.constraints().len(),
        loads: source.loads().len(),
        dofs: source.dof_count(),
        free_dofs: source.free_dofs().len(),
        bodies: source.body_count() as usize,
        pattern_entries: pattern_entries(model),
        profile_entries,
        half_bandwidth,
        blocks,
        rows: layout(source).len(),
        source_encoding_len: source.encoding().len(),
    }
}

/// Sizes of the exported types and of V-K's model types, from this build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sizes {
    pub straight_member: usize,
    pub constraint: usize,
    pub nodal_load: usize,
    pub station: usize,
    pub quantity_meta: usize,
    pub quantity_id: usize,
    pub published_row: usize,
    pub attempt_record: usize,
    pub vk_member: usize,
    pub vk_spring: usize,
    pub vk_load: usize,
}

impl Sizes {
    pub fn of_this_build() -> Self {
        Self {
            straight_member: std::mem::size_of::<StraightMember>(),
            constraint: std::mem::size_of::<Constraint>(),
            nodal_load: std::mem::size_of::<NodalLoad>(),
            station: std::mem::size_of::<Station>(),
            quantity_meta: std::mem::size_of::<QuantityMeta>(),
            quantity_id: std::mem::size_of::<QuantityId>(),
            published_row: std::mem::size_of::<PublishedRow>(),
            attempt_record: std::mem::size_of::<AttemptRecord>(),
            vk_member: std::mem::size_of::<Member>(),
            vk_spring: std::mem::size_of::<SpringSpec>(),
            vk_load: std::mem::size_of::<(u32, usize, f64, String)>(),
        }
    }
}

// Complete caller composition; public Sizes above are separate observations.
pub use crate::envelope::{estimate, CompleteEstimate, Estimate};
