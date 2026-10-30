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
//! H admission now requires an explicit model/source/launch context and the named
//! conditional reference profile in `h_envelope`; current public sizes remain
//! separate observations. Executable qualification remains external.

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
    let (counts, error, _) = compute_described(model);
    (counts, error)
}

/// Captures inline source facts before the existing source drops. No second
/// source, graph, free-DOF, layout or encoding computation is performed.
pub fn compute_described(
    model: &K6Model,
) -> (
    W1Counts,
    Option<String>,
    Result<super::h_envelope::HSourceFacts, super::envelope::EnvelopeError>,
) {
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
            let capture = super::h_envelope::HSourceFacts::from_counts(model, &counts);
            return (counts, Some(format!("{error:?}")), capture);
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
    let ids = source.loads().iter().try_fold(0u128, |sum, load| {
        sum.checked_add(load.source_id.len() as u128)
    });
    let max_id = source
        .loads()
        .iter()
        .map(|load| load.source_id.len() as u128)
        .max()
        .unwrap_or(0);
    let capture = ids
        .ok_or(super::envelope::EnvelopeError::ArithmeticOverflow)
        .and_then(|ids| super::h_envelope::HSourceFacts::successful(&counts, ids, max_id));
    (counts, None, capture)
}

// ------------------------------------------------------------------ estimate

// Historical K6b scalar constants below remain compatibility exports. They are
// not the named reference-profile facts and are not used by the H estimator.
// Current public type observations remain separately available in W1SizeFacts.

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

/// Complete H moving-request fields under the selected reference premises.
/// For SourceRefused (source_ok=false), max=sel128 is the positive source-window
/// bound and fixed is the caller baseline; kernel arrays/decide are explicitly
/// unexecuted zeros. They are never missing proof or successful kernel values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W1Estimate {
    /// The stop rule's tracker bound (a transient at each decision).
    pub decide: u128,
    /// Max named own-solve phase extras above the padded kernel prefix.
    pub solve: [u128; 4],
    /// Max named pass/resolution/shift/report-construction extras above the prefix.
    pub pass: [u128; 3],
    /// Kernel Base plus the selected caller/saved-attempt/prefix baseline.
    /// SourceRefused has only its caller baseline.
    pub fixed: u128,
    /// S(p): each attempt's shared build (kept in the group cache).
    pub shared: [u128; 4],
    /// U(p): each attempt's state.
    pub state: [u128; 4],
    /// V(P): each verification's shared data (kept).
    pub verify: [u128; 3],
    /// Complete H source/solve/outer-prefix moving envelope for the full schedule.
    pub max: u128,
    /// Complete conditional H envelope for S/U128,256 + V256.
    /// A source refusal shares this prefix without asserting selection.
    pub sel128: u128,
}

/// Complete H composition requires explicit source/model/launch facts; there is
/// no Counts/SizeFacts fallback. See `h_envelope` for the reference contract.
pub use super::h_envelope::estimate;

/// K4's storage bytes at precision `p` as K4 counts them: (pattern + profile)
/// entries × limbs × 8.
pub fn storage_bytes(c: &W1Counts, p: u32) -> u128 {
    (c.pattern_entries as u128 + c.profile_entries as u128) * limbs_per_entry(p) as u128 * 8
}
