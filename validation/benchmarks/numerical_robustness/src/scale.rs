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
//! - **The estimate** is K6b's E_max (`performance_harness/src/k6/w1/counts.rs`,
//!   `estimate`, at K6b's `082990c8d`, recomputed there on KF1's bounded
//!   trackers), ported term for term. Its constants are the sizes of FK's
//!   crate-private types, derived from their definitions, with K6b's citations.
//!   Only the model term is V-K's own (`ModelSizes`). K6's admission rule
//!   calibrates the estimate with the measured ratio ρ. Nothing asserts a
//!   memory bound. Once K6b's crate is on main, the two copies can be
//!   deduplicated in one follow-up.
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

// ------------------------------------------------------------------ estimate (K6b's, ported)

/// The bytes of one `Wide<L>` (`K4R/wide.rs`): 8L + 16 with padding.
pub const fn wide_bytes(limbs: u128) -> u128 {
    8 * limbs + 16
}

/// Each attempt's shared build: (p, L, R) (`K4R/adaptive.rs`).
pub const ATTEMPT_WIDTHS: [(u32, u128, u128); 4] =
    [(128, 4, 4), (256, 4, 8), (512, 8, 16), (1024, 16, 16)];
/// Each verification's shared data: (P, L_P, W).
pub const VERIFY_WIDTHS: [(u32, u128, u128); 3] = [(256, 4, 8), (512, 8, 16), (1024, 16, 16)];
/// `MemberOperators<L>` (`K4R/assemble.rs`): 164 values and three u32.
pub const MEMBER_OPERATOR_WIDES: u128 = 164;
pub const MEMBER_OPERATOR_EXTRA: u128 = 16;
/// `BoundedCoefficients<L>`: five values and a u32.
pub const BOUNDED_WIDES: u128 = 5;
pub const BOUNDED_EXTRA: u128 = 8;
/// A member's 12×12 block (`bounded_block`; K_e at q_W).
pub const BLOCK_WIDES: u128 = 144;
/// Per free row of `RetainedFactor<L>`: two values and 56 bytes.
pub const FACTOR_ROW_WIDES: u128 = 2;
pub const FACTOR_ROW_EXTRA: u128 = 56;
/// `BlockBound<L>`: three values and an option.
pub const BLOCK_BOUND_WIDES: u128 = 4;
pub const BLOCK_BOUND_EXTRA: u128 = 8;
/// `Structure`: 78 upper contributions per member, 8 bytes each, 16 while
/// tagged during the build.
pub const CONTRIBUTIONS_PER_MEMBER: u128 = 78;
pub const CONTRIBUTION_BYTES: u128 = 8;
pub const TAGGED_CONTRIBUTION_BYTES: u128 = 16;
/// The verification report's per-row option vectors.
pub const REPORT_ROW_VECTORS: u128 = 5;
pub const OPTION_EXTRA: u128 = 8;
/// A solve's working vectors at the residual width.
pub const SOLVE_VECTORS: u128 = 4;
/// `ExactWideSum` (`K4R/wide_sum.rs`): 2,144 bytes with alignment.
pub const EXACT_WIDE_SUM_BYTES: u128 = 2144;
/// An unevaluated tracker row, or a fallback row: 4,304 bytes (KF1).
pub const TRACKER_ENTRY_BYTES: u128 = 2 * EXACT_WIDE_SUM_BYTES + 16;
/// A tracker table entry: 40 bytes (KF1).
pub const TRACKER_TABLE_ENTRY_BYTES: u128 = 40;
/// KF1's bounds on unevaluated rows at T = 512, G = 8T, each at its peak.
pub const STOP_RULE_PEAK_ROWS: u128 = 4096 + 512;
pub const PIVOT_TRACKER_PEAK_ROWS: u128 = 768;
pub const SOLVE_TRACKER_PEAK_ROWS: u128 = 2816;
pub const DECIDE_TRACKER_SETS: u128 = 3;
pub const VEC_SLACK: u128 = 2;
pub const SOLVE_TABLE_ROWS_PER_FREE_DOF: u128 = 5;
pub const RESIDUAL_ROW_EXTRA: u128 = 16;
pub const LEDGER_ENTRY_BYTES: u128 = 64;
pub const PRESCRIBED_ENTRY_BYTES: u128 = 48;
pub const SOURCE_ID_BYTES: u128 = 16;
/// A heap string's allocation for a short label (node, member or source id).
pub const LABEL_BYTES: u128 = 16;
/// A `BTreeMap` entry's overhead per key and value (node share, amortized).
pub const MAP_ENTRY_EXTRA: u128 = 32;

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

/// The estimate and its terms (bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Estimate {
    /// V-K's model and its lane state (the published map), alive throughout.
    pub model: u128,
    /// The stop rule's tracker bound (a transient at each decision).
    pub decide: u128,
    /// Alive through the call: the model, the source twice, the case, the group.
    pub fixed: u128,
    /// E_max: the peak over the whole schedule (128 … v1024).
    pub max: u128,
    /// E_sel128: the peak of the path selected at 128 (128, 256, v256).
    pub sel128: u128,
}

/// V-K's model term: `cases::Model` (node labels and coordinates, members,
/// springs, constraints, loads with their source ids, stations) and the
/// lane's published map (`lane::CaseRun::published`).
pub fn model_bytes(c: &Counts, s: &Sizes) -> u128 {
    let u = |x: usize| x as u128;
    u(c.nodes) * (24 + 24 + LABEL_BYTES)
        + u(c.members) * (u(s.vk_member) + LABEL_BYTES)
        + u(c.springs) * (u(s.vk_spring) + LABEL_BYTES)
        + u(c.constraints) * 16
        + u(c.loads) * (u(s.vk_load) + LABEL_BYTES)
        + u(c.stations) * 16
        + u(c.rows) * (u(s.quantity_id) + u(s.published_row) + MAP_ENTRY_EXTRA)
}

/// K6b's W1 estimate, ported; build transients are added on top of each
/// build's kept bytes, so it bounds rather than tracks the peak.
pub fn estimate(c: &Counts, s: &Sizes) -> Estimate {
    let u = |x: usize| x as u128;
    let (m, n, nf, nnz) = (
        u(c.members),
        u(c.dofs),
        u(c.free_dofs),
        u(c.pattern_entries),
    );
    let (p_entries, blocks, rows) = (u(c.profile_entries), u(c.blocks.max(1)), u(c.rows));
    let (nodes, r, loads, stations) = (u(c.nodes), u(c.constraints), u(c.loads), u(c.stations));
    let enc = u(c.source_encoding_len);
    let w = wide_bytes;

    let model = model_bytes(c, s);
    let source = 24 * nodes
        + m * u(s.straight_member)
        + r * u(s.constraint)
        + loads * (u(s.nodal_load) + SOURCE_ID_BYTES)
        + stations * u(s.station)
        + 16 * n
        + 4 * nodes;
    let case = loads * LEDGER_ENTRY_BYTES
        + r * PRESCRIBED_ENTRY_BYTES
        + enc
        + rows * u(s.quantity_meta)
        + 8 * u(c.bodies);
    let group = 8 * (n + 1)
        + 16 * nnz
        + 8 * (nnz + 1)
        + m * CONTRIBUTIONS_PER_MEMBER * CONTRIBUTION_BYTES
        + 32 * nf
        + 8 * n
        + 12 * nf
        + 24 * blocks;
    let group_build = m * CONTRIBUTIONS_PER_MEMBER * (TAGGED_CONTRIBUTION_BYTES + 16);
    let fixed = model + 2 * source + case + group;

    let shared = ATTEMPT_WIDTHS.map(|(_, l, r)| {
        m * (MEMBER_OPERATOR_WIDES * w(l) + MEMBER_OPERATOR_EXTRA)
            + nnz * (w(l) + w(r))
            + m * (BOUNDED_WIDES * w(r) + BOUNDED_EXTRA)
            + p_entries * w(l)
            + nf * (FACTOR_ROW_WIDES * w(l) + FACTOR_ROW_EXTRA)
            + blocks * w(l)
    });
    let shared_build = ATTEMPT_WIDTHS.map(|(p, _, r)| {
        if p == 1024 {
            0
        } else {
            m * (MEMBER_OPERATOR_WIDES * w(r) + MEMBER_OPERATOR_EXTRA)
        }
    });
    let state = ATTEMPT_WIDTHS.map(|(_, l, _)| (n + 6 * m + rows) * w(l));
    let solve_trackers = SOLVE_TRACKER_PEAK_ROWS * TRACKER_ENTRY_BYTES
        + VEC_SLACK * SOLVE_TABLE_ROWS_PER_FREE_DOF * nf * TRACKER_TABLE_ENTRY_BYTES
        + VEC_SLACK * nf * TRACKER_ENTRY_BYTES;
    let solve = ATTEMPT_WIDTHS.map(|(_, l, r)| {
        SOLVE_VECTORS * n * w(r) + nf * (RESIDUAL_ROW_EXTRA + w(l)) + solve_trackers
    });
    let verify = VERIFY_WIDTHS.map(|(_, l, ww)| {
        nnz * w(l)
            + m * BLOCK_WIDES * w(ww)
            + blocks * (BLOCK_BOUND_WIDES * w(l) + BLOCK_BOUND_EXTRA)
    });
    let verify_build = VERIFY_WIDTHS.map(|(p, l, ww)| {
        m * (BOUNDED_WIDES * w(l) + BOUNDED_EXTRA)
            + m * BLOCK_WIDES * w(l)
            + if p == 1024 {
                0
            } else {
                m * (MEMBER_OPERATOR_WIDES * w(ww) + MEMBER_OPERATOR_EXTRA)
            }
    });
    let report = VERIFY_WIDTHS
        .map(|(_, l, _)| REPORT_ROW_VECTORS * rows * (w(l) + OPTION_EXTRA) + 2 * nf * w(l));
    let pass = VERIFY_WIDTHS.map(|(_, l, _)| p_entries * w(l));
    let decide = STOP_RULE_PEAK_ROWS * TRACKER_ENTRY_BYTES
        + DECIDE_TRACKER_SETS * VEC_SLACK * rows * TRACKER_TABLE_ENTRY_BYTES;
    let pivot =
        PIVOT_TRACKER_PEAK_ROWS * TRACKER_ENTRY_BYTES + VEC_SLACK * nf * TRACKER_TABLE_ENTRY_BYTES;
    let end = rows * (u(s.published_row) + 40)
        + enc
        + (n + 6 * m) * (9 + 8 * 16)
        + 8 * u(s.attempt_record);

    let mut kept = fixed;
    let mut peak = fixed + group_build;
    let mut sel128 = 0;
    for (k, &(p, _, _)) in ATTEMPT_WIDTHS.iter().enumerate() {
        peak = peak.max(kept + shared[k] + shared_build[k] + pivot);
        kept += shared[k];
        peak = peak.max(kept + state[k] + solve[k]);
        kept += state[k];
        if let Some(v) = VERIFY_WIDTHS.iter().position(|&(vp, _, _)| vp == p) {
            peak = peak.max(kept + verify[v] + verify_build[v]);
            kept += verify[v];
            peak = peak.max(kept + report[v] + pass[v]);
            peak = peak.max(kept + report[v] + decide);
            if p == 256 {
                sel128 = peak.max(kept + report[v] + end);
            }
        }
    }
    let max = peak.max(kept + report[2] + end);
    Estimate {
        model,
        decide,
        fixed,
        max,
        sel128,
    }
}
