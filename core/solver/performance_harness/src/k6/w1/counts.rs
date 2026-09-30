//! K6b's W1 counts and the W1 admission estimate (T3 K6b plan §3.4–§3.5;
//! ROOT's rulings on I16's plan).
//!
//! The counts phase works in O(nnz): it builds the adapter's `PrimitiveSource`,
//! the structural free–free adjacency and K4's own RCM order (the exported
//! `reverse_cuthill_mckee`), and counts the skyline by K4's rule
//! (`K4R/factor.rs:351-398`: first = the least rank over a row's neighbours
//! and itself). Nothing is solved. Every count is checked at run time against
//! each attempt's `StorageCounts` (parity lines).
//!
//! The estimate counts the bytes alive at W1's peak, as K6 derived E_adm
//! (K6 plan §7). Sizes of FK's crate-private types are derived from their
//! definitions (the constants below cite them); sizes of exported types come
//! from `size_of`. It is an admission estimate: the measured ratio ρ
//! calibrates it, and nothing asserts a memory bound.
//!
//! It bounds every modelled phase of the schedule, each Vec growth under the
//! allocator's move model (the old and new buffers alive together), which is
//! the model ρ reads: the shared builds, each solve with its fallback, each
//! verification build and pass (with the shift), each decision, and the end
//! (ROOT's ruling on RV22-2).

use super::super::models::K6Model;
use super::super::Fnv64;
use super::adapter;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    layout, reverse_cuthill_mckee, AttemptRecord, Constraint, NodalLoad, PublishedRow,
    QuantityMeta, Station, StraightMember,
};
use open_pipe_stress_frame_kernel::{FrameElement, DOF_PER_NODE};

/// W1's deterministic counts of one model (the counts line's `w1_*` keys).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct W1Counts {
    /// Whether `PrimitiveSource::new` accepted the adapter's parts.
    pub source_ok: bool,
    pub nodes: usize,
    pub members: usize,
    pub stations: usize,
    pub constraints: usize,
    pub loads: usize,
    pub dofs: usize,
    pub free_dofs: usize,
    pub bodies: usize,
    /// Stored entries of the structural pattern (both triangles).
    pub pattern_entries: usize,
    /// K4's skyline of the RCM-ordered structural free–free pattern.
    pub profile_entries: usize,
    pub half_bandwidth: usize,
    /// Connected components of the free–free pattern (K4's blocks).
    pub blocks: usize,
    /// The published layout's length (`layout(&source).len()`).
    pub rows: usize,
    /// K4SRC's length and FNV-1a digest (`PrimitiveSource::encoding`).
    pub source_encoding_len: usize,
    pub source_encoding_fnv64: u64,
}

/// K4's limbs per stored entry at each precision (`K4R/adaptive.rs:2455-2459`).
pub const LIMBS_PER_ENTRY: [(u32, usize); 4] = [(128, 4), (256, 4), (512, 8), (1024, 16)];

/// The limbs per entry of precision `p`, as K4 records it.
pub fn limbs_per_entry(p: u32) -> usize {
    match p {
        128 | 256 => 4,
        512 => 8,
        _ => 16,
    }
}

/// The FNV-1a digest of a byte string (K6's fixed digest; not cryptographic).
pub fn fnv64(bytes: &[u8]) -> u64 {
    let mut h = Fnv64::new();
    h.update(bytes);
    h.finish()
}

/// Node adjacency of the member graph (sorted, without self) and whether each
/// node carries a member.
fn node_graph(nodes: usize, members: &[StraightMember]) -> (Vec<Vec<usize>>, Vec<bool>) {
    let mut adjacent: Vec<Vec<usize>> = vec![Vec::new(); nodes];
    let mut touched = vec![false; nodes];
    for m in members {
        let (i, j) = (m.node_i as usize, m.node_j as usize);
        adjacent[i].push(j);
        adjacent[j].push(i);
        touched[i] = true;
        touched[j] = true;
    }
    for list in &mut adjacent {
        list.sort_unstable();
        list.dedup();
    }
    (adjacent, touched)
}

/// K4's profile rule on the structural free–free adjacency: (profile entries,
/// half-bandwidth, blocks).
pub fn structural_profile(
    dofs: usize,
    free: &[usize],
    adjacent: &[Vec<usize>],
) -> (usize, usize, usize) {
    let mut position = vec![usize::MAX; dofs];
    for (a, &g) in free.iter().enumerate() {
        position[g] = a;
    }
    let adjacency: Vec<Vec<usize>> = free
        .iter()
        .map(|&g| {
            let node = g / DOF_PER_NODE;
            let mut out = Vec::new();
            for &other in std::iter::once(&node).chain(adjacent[node].iter()) {
                for c in 0..DOF_PER_NODE {
                    let h = other * DOF_PER_NODE + c;
                    if h != g && position[h] != usize::MAX {
                        out.push(position[h]);
                    }
                }
            }
            out
        })
        .collect();
    drop(position);
    let order = reverse_cuthill_mckee(&adjacency);
    let mut rank = vec![0usize; free.len()];
    for (k, &a) in order.iter().enumerate() {
        rank[a] = k;
    }
    let (mut profile, mut bandwidth) = (0usize, 0usize);
    for (i, &a) in order.iter().enumerate() {
        let first = adjacency[a]
            .iter()
            .map(|&b| rank[b])
            .chain(std::iter::once(i))
            .min()
            .unwrap_or(i);
        profile += i - first + 1;
        bandwidth = bandwidth.max(i - first);
    }
    // Blocks: connected components of the free–free adjacency.
    let mut block = vec![usize::MAX; free.len()];
    let mut blocks = 0;
    let mut stack = Vec::new();
    for seed in 0..free.len() {
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
    (profile, bandwidth, blocks)
}

/// W1's counts of `model`, in O(nnz) memory, with the source's refusal if
/// `PrimitiveSource::new` refuses it.
pub fn compute(model: &K6Model) -> (W1Counts, Option<String>) {
    let source = match adapter::source(model) {
        Ok(source) => source,
        Err(error) => {
            let counts = W1Counts {
                source_ok: false,
                nodes: model.node_count(),
                members: model.member_count(),
                stations: model.member_count(),
                constraints: model.restrained_dofs().len(),
                loads: model.loads.len(),
                dofs: model.dofs(),
                free_dofs: 0,
                bodies: 0,
                pattern_entries: 0,
                profile_entries: 0,
                half_bandwidth: 0,
                blocks: 0,
                rows: 0,
                source_encoding_len: 0,
                source_encoding_fnv64: 0,
            };
            return (counts, Some(format!("{error:?}")));
        }
    };
    let (adjacent, touched) = node_graph(source.node_count(), source.members());
    let pairs: usize = adjacent.iter().map(Vec::len).sum();
    let touched = touched.iter().filter(|&&t| t).count();
    let pattern_entries = DOF_PER_NODE * DOF_PER_NODE * (touched + pairs);
    let free = source.free_dofs();
    let (profile_entries, half_bandwidth, blocks) =
        structural_profile(source.dof_count(), &free, &adjacent);
    let encoding = source.encoding();
    let counts = W1Counts {
        source_ok: true,
        nodes: source.node_count(),
        members: source.members().len(),
        stations: source.stations().len(),
        constraints: source.constraints().len(),
        loads: source.loads().len(),
        dofs: source.dof_count(),
        free_dofs: free.len(),
        bodies: source.body_count() as usize,
        pattern_entries,
        profile_entries,
        half_bandwidth,
        blocks,
        rows: layout(&source).len(),
        source_encoding_len: encoding.len(),
        source_encoding_fnv64: fnv64(&encoding),
    };
    (counts, None)
}

// ------------------------------------------------------------------ estimate

/// The bytes of one `Wide<L>`: `{ negative: bool, exponent: i64,
/// significand: [u64; L] }` (`K4R/wide.rs:200-204`), 8L + 16 with padding.
pub const fn wide_bytes(limbs: u128) -> u128 {
    8 * limbs + 16
}

/// Each attempt's shared build: (p, L, R) (`K4R/adaptive.rs:2529-2532`).
pub const ATTEMPT_WIDTHS: [(u32, u128, u128); 4] =
    [(128, 4, 4), (256, 4, 8), (512, 8, 16), (1024, 16, 16)];
/// Each verification's shared data: (P, L_P, W) (`K4R/adaptive.rs:2667-2669`).
pub const VERIFY_WIDTHS: [(u32, u128, u128); 3] = [(256, 4, 8), (512, 8, 16), (1024, 16, 16)];
/// `MemberOperators<L>` (`K4R/assemble.rs:65-83`): 9 axis + 5 coefficient +
/// 72 B + 78 K_e values, and three u32 (padded to 16 bytes).
pub const MEMBER_OPERATOR_WIDES: u128 = 164;
pub const MEMBER_OPERATOR_EXTRA: u128 = 16;
/// `BoundedCoefficients<L>` (`K4R/assemble.rs:89-97`): five values and a u32.
pub const BOUNDED_WIDES: u128 = 5;
pub const BOUNDED_EXTRA: u128 = 8;
/// A member's 12×12 block: `bounded_block` (`K4R/assemble.rs:400-404`) and
/// K_e at q_W (`K4R/verify.rs:390`).
pub const BLOCK_WIDES: u128 = 144;
/// Per free row of `RetainedFactor<L>` (`K4R/factor.rs:400-420`): the row's
/// `Vec` header (24), order, first and scale (8 each), and a `PivotScreen`
/// (two values and a u64).
pub const FACTOR_ROW_WIDES: u128 = 2;
pub const FACTOR_ROW_EXTRA: u128 = 56;
/// `BlockBound<L>` (`K4R/bound.rs:341-350`): three values and an option.
pub const BLOCK_BOUND_WIDES: u128 = 4;
pub const BLOCK_BOUND_EXTRA: u128 = 8;
/// `Structure` (`K4R/assemble.rs:516-620`): 78 upper contributions per member,
/// 8 bytes each (`Contribution`), 16 while tagged during the build.
pub const CONTRIBUTIONS_PER_MEMBER: u128 = 78;
pub const CONTRIBUTION_BYTES: u128 = 8;
pub const TAGGED_CONTRIBUTION_BYTES: u128 = 16;
/// The verification report's per-row `Option<Wide<L>>` vectors
/// (`K4R/verify.rs:571-600`: e_rows, w, a_s, charge, w_plus).
pub const REPORT_ROW_VECTORS: u128 = 5;
pub const OPTION_EXTRA: u128 = 8;
/// A solve's working vectors at the residual width (rhs, u, residual,
/// correction).
pub const SOLVE_VECTORS: u128 = 4;
/// The solve's evaluated states (`K4R/adaptive.rs:1606-1612`): the first
/// solve and up to three corrections, each n_f values, alive to the solve's
/// end with `u_free` (RV22-2).
pub const EVALUATED_STATES: u128 = 4;
/// A `Vec` header (pointer, capacity, length).
pub const VEC_HEADER_BYTES: u128 = 24;
/// The rows of the fallback's per-state row list at its peak under the move
/// model (RV22-2). The list starts empty and takes one push per free row
/// (`K4R/adaptive.rs:1476`, `:1515`), so its capacity doubles from 1 (the
/// minimum for elements over 1 KiB) to C = the next power of two ≥ its
/// length ≤ n_f; its last growth holds the old C/2 rows and the new C.
pub const fn fallback_row_list_rows(free_dofs: u128) -> u128 {
    let c = next_pow2(free_dofs);
    c + c / 2
}

/// The least power of two ≥ x (0 for 0).
pub const fn next_pow2(x: u128) -> u128 {
    if x == 0 {
        return 0;
    }
    let mut c = 1;
    while c < x {
        c *= 2;
    }
    c
}
/// A growing tracker table under the move model holds its old buffer too: at
/// most one more entry per counted row, for the one table growing.
pub const TABLE_MOVE_EXTRA: u128 = 1;
/// The verification pass's locals alive to its end (`K4R/verify.rs:731-925`):
/// three over every DOF (`w_abs`, `delta_full`, `w_s`) and eight over the free
/// DOFs (`u_free`, `r_hat`, `sr_row`, `delta`, `sr2_row`, `sas_inf_row`,
/// `sas_one_col`, `sau_row`), plus `recover`'s output (rows + 6m values,
/// `K4R/recover.rs:203-211`) and, per DOF, the two term lists' headers and
/// two flags (RV22-2).
pub const PASS_FULL_VECTORS: u128 = 3;
pub const PASS_FREE_VECTORS: u128 = 8;
pub const PASS_PER_DOF_EXTRA: u128 = 2 * VEC_HEADER_BYTES + 2;
/// The report's row vectors alive before the shift (e_rows, w, a_s); charge
/// and w_plus follow it.
pub const PASS_EARLY_ROW_VECTORS: u128 = 3;
/// The shift (`K4R/bound.rs:433-441`, `:548-600`): the scaled profile (its
/// rows, a header, `first` and `block_of_row` per free row) and
/// `shifted_factor`'s clone of the rows with `first`, `shifted` and `work`.
pub const PROFILE_ROW_EXTRA: u128 = VEC_HEADER_BYTES + 8 + 4;
pub const PROFILE_CLONE_ROW_EXTRA: u128 = VEC_HEADER_BYTES + 8;
/// `ExactWideSum` (`K4R/wide_sum.rs:112-124`): two 128-limb magnitudes
/// (`SUM_LIMBS`, `:43`), two i128, a usize, a bool and a `SumWork`: 2,129
/// bytes, 2,144 with its 16-byte alignment.
pub const EXACT_WIDE_SUM_BYTES: u128 = 2144;
/// An unevaluated tracker row (`BoundedExtremeTracker::lazy`,
/// `K4R/adaptive.rs:619`: two sums and two u64) and a fallback row
/// (`bounded_fallback`'s `rows`, `:1476`: two sums and an f64): 4,304 bytes
/// each with alignment (KF1 RETURN §4, "an unevaluated row 4,304 B").
pub const TRACKER_ENTRY_BYTES: u128 = 2 * EXACT_WIDE_SUM_BYTES + 16;
/// A tracker table entry (`:621`, a key and an `Evaluated`): 40 bytes (KF1
/// RETURN §4).
pub const TRACKER_TABLE_ENTRY_BYTES: u128 = 40;
/// KF1's bounds on unevaluated rows (T = 512, G = 8T = 4,096; `:539`, `:544`),
/// as KF1 RETURN addendum 2 restates them, each at its peak during one offer
/// (a `Vec` growth briefly holds both buffers):
/// - the stop rule's `TrackerSet` for one call (`rule`, `:1893`): G + T;
/// - a standalone tracker (the pivot margin, `:1236`): 1.5T;
/// - a solve attempt (the residual gate's tracker and the fallback's, up to
///   four states, `residual_rows` `:1301` and `bounded_fallback` `:1437`):
///   2,816 rows.
pub const STOP_RULE_PEAK_ROWS: u128 = 4096 + 512;
pub const PIVOT_TRACKER_PEAK_ROWS: u128 = 768;
pub const SOLVE_TRACKER_PEAK_ROWS: u128 = 2816;
/// The stop rule offers each published row to at most three trackers ((a),
/// the estimate (b) and the charge (d)); a call's tables hold at most one
/// entry per row kept in a window, so at most one per offered row (KF1 RETURN
/// addendum 2, RV20-N3), in a `Vec` that a collapse may briefly double.
pub const DECIDE_TRACKER_SETS: u128 = 3;
pub const VEC_SLACK: u128 = 2;
/// A solve attempt's tables: the residual gate's (one per evaluation, n_f
/// rows) and the fallback's (up to four states of n_f rows).
pub const SOLVE_TABLE_ROWS_PER_FREE_DOF: u128 = 5;
/// `residual_rows`' list per free DOF (`:1301`): (bool, f64, `Wide<L>`).
pub const RESIDUAL_ROW_EXTRA: u128 = 16;
/// A ledger entry: (usize, `LedgerNet`) plus its limbs (`K4R/ledger.rs:25-30`).
pub const LEDGER_ENTRY_BYTES: u128 = 64;
/// A prescribed row: (usize, `Vec<(f64, f64)>`) plus one term.
pub const PRESCRIBED_ENTRY_BYTES: u128 = 48;
/// A load's source-id string on the heap (`k6:<DOF>`, at most 16 bytes).
pub const SOURCE_ID_BYTES: u128 = 16;
/// Per model node and member in the harness's `K6Model` (label, coordinates,
/// indices).
pub const MODEL_NODE_BYTES: u128 = 64;
pub const MODEL_MEMBER_BYTES: u128 = 80;

/// Sizes of the exported and harness types, from this build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W1SizeFacts {
    pub straight_member: usize,
    pub constraint: usize,
    pub nodal_load: usize,
    pub station: usize,
    pub quantity_meta: usize,
    pub published_row: usize,
    pub attempt_record: usize,
    pub frame_element: usize,
}

impl W1SizeFacts {
    pub fn of_this_build() -> Self {
        Self {
            straight_member: std::mem::size_of::<StraightMember>(),
            constraint: std::mem::size_of::<Constraint>(),
            nodal_load: std::mem::size_of::<NodalLoad>(),
            station: std::mem::size_of::<Station>(),
            quantity_meta: std::mem::size_of::<QuantityMeta>(),
            published_row: std::mem::size_of::<PublishedRow>(),
            attempt_record: std::mem::size_of::<AttemptRecord>(),
            frame_element: std::mem::size_of::<FrameElement>(),
        }
    }
}

/// The W1 estimate and its terms (bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W1Estimate {
    /// The stop rule's tracker bound (a transient at each decision).
    pub decide: u128,
    /// Each attempt's solve transient, with the fallback.
    pub solve: [u128; 4],
    /// Each verification pass's transient, at its larger of the shift and
    /// after it.
    pub pass: [u128; 3],
    /// Alive through the call: the harness's model and frames, the source
    /// twice, the case and the group.
    pub fixed: u128,
    /// S(p): each attempt's shared build (kept in the group cache).
    pub shared: [u128; 4],
    /// U(p): each attempt's state.
    pub state: [u128; 4],
    /// V(P): each verification's shared data (kept).
    pub verify: [u128; 3],
    /// E_max: the peak over the whole schedule (128 … v1024).
    pub max: u128,
    /// E_sel128: the peak of the path selected at 128 (128, 256, v256).
    pub sel128: u128,
}

/// The W1 estimate (plan §3.5; ROOT's ruling on RV22-2). Each phase's
/// transient is added to the bytes kept when it runs, so the estimate bounds
/// every modelled phase rather than tracking the peak.
pub fn estimate(c: &W1Counts, s: &W1SizeFacts) -> W1Estimate {
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

    let harness =
        nodes * MODEL_NODE_BYTES + m * MODEL_MEMBER_BYTES + loads * 16 + m * u(s.frame_element);
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
    let fixed = harness + 2 * source + case + group;

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
    // KF1: a solve attempt's trackers (bounded) and their tables.
    let solve_trackers = SOLVE_TRACKER_PEAK_ROWS * TRACKER_ENTRY_BYTES
        + (VEC_SLACK * SOLVE_TABLE_ROWS_PER_FREE_DOF + TABLE_MOVE_EXTRA)
            * nf
            * TRACKER_TABLE_ENTRY_BYTES;
    // The solve (`K4R/adaptive.rs:1554-1700`): its working vectors and the
    // residual gate's row list, then the fallback (`:1437-1500`, RV22-2):
    // `u_free`, the evaluated states and `abar_q`, alive throughout; then
    // either `assemble_bounded`'s member blocks (while `abar_q` is built) or,
    // per state, the copy of u and the row list under the move model.
    let solve = ATTEMPT_WIDTHS.map(|(_, l, r)| {
        let fallback = nf * w(l)
            + EVALUATED_STATES * (nf * w(l) + VEC_HEADER_BYTES)
            + nnz * w(r)
            + (m * BLOCK_WIDES * w(r))
                .max(n * w(l) + fallback_row_list_rows(nf) * TRACKER_ENTRY_BYTES);
        SOLVE_VECTORS * n * w(r) + nf * (RESIDUAL_ROW_EXTRA + w(l)) + solve_trackers + fallback
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
    // The report, kept after the pass: its row vectors and two over the free
    // DOFs.
    let report = VERIFY_WIDTHS
        .map(|(_, l, _)| REPORT_ROW_VECTORS * rows * (w(l) + OPTION_EXTRA) + 2 * nf * w(l));
    // The pass (`K4R/verify.rs:731-925`, RV22-2): its locals, with either the
    // first three row vectors and the shift's two profiles (`K4R/bound.rs`
    // `scaled_profile`, `shifted_factor`) or, after the shift, all five row
    // vectors.
    let pass = VERIFY_WIDTHS.map(|(_, l, ww)| {
        let live = PASS_FULL_VECTORS * n * w(l)
            + PASS_FREE_VECTORS * nf * w(l)
            + (rows + 6 * m) * w(l)
            + n * PASS_PER_DOF_EXTRA
            + r * (w(ww) + w(l));
        let at_shift = PASS_EARLY_ROW_VECTORS * rows * (w(l) + OPTION_EXTRA)
            + p_entries * w(l)
            + nf * PROFILE_ROW_EXTRA
            + p_entries * w(l)
            + nf * (PROFILE_CLONE_ROW_EXTRA + (w(l) + OPTION_EXTRA) + w(l));
        let after = REPORT_ROW_VECTORS * rows * (w(l) + OPTION_EXTRA);
        live + at_shift.max(after)
    });
    // KF1: the stop rule's trackers at their peak, and their tables.
    let decide = STOP_RULE_PEAK_ROWS * TRACKER_ENTRY_BYTES
        + (DECIDE_TRACKER_SETS * VEC_SLACK + TABLE_MOVE_EXTRA) * rows * TRACKER_TABLE_ENTRY_BYTES;
    // KF1: the pivot margin's tracker in each shared build, and its table.
    let pivot = PIVOT_TRACKER_PEAK_ROWS * TRACKER_ENTRY_BYTES
        + (VEC_SLACK + TABLE_MOVE_EXTRA) * nf * TRACKER_TABLE_ENTRY_BYTES;
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
            peak = peak.max(kept + pass[v]);
            peak = peak.max(kept + report[v] + decide);
            if p == 256 {
                sel128 = peak.max(kept + report[v] + end);
            }
        }
    }
    let max = peak.max(kept + report[2] + end);
    W1Estimate {
        decide,
        solve,
        pass,
        fixed,
        shared,
        state,
        verify,
        max,
        sel128,
    }
}

/// K4's storage bytes at precision `p` as K4 counts them: (pattern + profile)
/// entries × limbs × 8.
pub fn storage_bytes(c: &W1Counts, p: u32) -> u128 {
    (c.pattern_entries as u128 + c.profile_entries as u128) * limbs_per_entry(p) as u128 * 8
}
