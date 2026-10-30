//! K6 tests C: the deterministic counts (K6 brief, Required tests C; K6 plan
//! §4). Debug tests stay at 100 members or fewer and the DEC-053 nine.
//!
//! - C1: the closed forms derived in the plan (§4.2) and the full-block
//!   identity-profile bounds.
//! - C2: K1's storage table (`T3/IMPLEMENTATION/K1/RETURN.md:407-414`).
//! - C3: the independent Python oracle's values
//!   (`T3/IMPLEMENTATION/K6/_run_records/oracle/`), committed here so CI pins
//!   them, and K6's count equal to `order_sparse_structural`'s own.

use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, prepare_sparse_structural, SparseAssemblyOptions,
    SparseStructuralSystem, SparseSymmetryEvidence,
};
use open_pipe_stress_nonlinear_integration::structural_adapter::SparseAssemblyEvidence;
use open_pipe_stress_solver_performance_harness::k6::counts::{
    compute, K6Counts, FRAME_FAMILY_SYMMETRY_BASIS,
};
use open_pipe_stress_solver_performance_harness::k6::models::{model, Family};
use open_pipe_stress_sparse_direct::structural::order_sparse_structural;

fn counts_of(id: &str) -> K6Counts {
    let m = model(id).expect("model");
    let frames = m.frames().expect("frames");
    compute(&m, &frames, None).expect("counts")
}

fn small_ids() -> Vec<String> {
    ORACLE.iter().map(|row| row.0.to_string()).collect()
}

/// C1: pattern = 36(N + 2m), lower = 18(N + 2m) + 3N, contributions = 144m,
/// dense = (6N)², and the free-block and identity-profile forms per family.
#[test]
fn closed_forms() {
    for id in small_ids() {
        let m = model(&id).unwrap();
        let c = counts_of(&id);
        let (nodes, members) = (m.node_count(), m.member_count());
        assert_eq!(
            c.pattern_entries,
            36 * (nodes + 2 * members),
            "{id} pattern"
        );
        assert_eq!(
            c.lower_entries,
            18 * (nodes + 2 * members) + 3 * nodes,
            "{id} lower"
        );
        assert_eq!(c.contributions, 144 * members, "{id} contributions");
        assert_eq!(c.dense_entries, (6 * nodes as u128).pow(2), "{id} dense");
        assert_eq!(
            c.free_lower_entries,
            (c.free_entries + c.free_dofs) / 2,
            "{id} free lower from both triangles"
        );
        let (free_entries, free_dofs, identity_bound) = match (m.family, id.as_str()) {
            (Family::Chain, _) => (108 * members - 72, 6 * members, 57 * members - 36),
            (Family::Tree, _) => (108 * members - 72, 6 * members, 75 * members - 72),
            (Family::Cont, _) => {
                let s = members / 2;
                (117 * s - 36, 9 * s, 27 * s * s + 36 * s - 18)
            }
            (Family::Dec053, id) if id.contains("chain") => {
                (108 * members - 72, 6 * members, 57 * members - 36)
            }
            (Family::Dec053, id) => {
                let dims = id.rsplit('-').next().unwrap();
                let (x, y) = dims.split_once('x').unwrap();
                let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
                let entries = 36 * x * (y - 1) + 72 * ((x - 1) * (y - 1) + x * (y - 2));
                (entries, 6 * x * (y - 1), usize::MAX)
            }
            (Family::Grid, _) => unreachable!("no Q5 grid in the debug set"),
        };
        assert_eq!(c.free_entries, free_entries, "{id} free entries");
        assert_eq!(c.free_dofs, free_dofs, "{id} free dofs");
        assert!(
            c.identity_profile_entries <= identity_bound as u128,
            "{id}: the identity profile exceeds its full-block bound"
        );
    }
}

/// C2: K1's storage table, n = 10 and 100 (the 1,000- and 10,000-member rows
/// are recorded from the counts-only runs). Stored, lower and free-lower
/// counts depend only on N, m and the restraints, so they match at every size
/// and orientation. The nonzero count, the profile and the half-bandwidth
/// match K1's for CHAIN (both orientations) and TREE-AX; TREE-ROT's nonzero
/// count differs, because K1's comb numbers its spine before its branches and
/// picks its own y-references (the profile and half-bandwidth still match).
#[test]
fn k1_table_cross_check() {
    // (id, stored, lower, f_lower, f_lo_nz, profile, half-bandwidth); K1's
    // TREE-ROT nonzero figures (446 and 4,721) are not K6's model.
    let rows: [(&str, usize, usize, usize, Option<usize>, u128, usize); 8] = [
        (
            "RF-LARGE-CHAIN-n00010-AX",
            1116,
            591,
            534,
            Some(152),
            168,
            4,
        ),
        (
            "RF-LARGE-CHAIN-n00010-ROT",
            1116,
            591,
            534,
            Some(396),
            529,
            13,
        ),
        (
            "RF-LARGE-CHAIN-n00100-AX",
            10836,
            5721,
            5664,
            Some(1592),
            1788,
            4,
        ),
        (
            "RF-LARGE-CHAIN-n00100-ROT",
            10836,
            5721,
            5664,
            Some(4176),
            5569,
            13,
        ),
        (
            "RF-LARGE-TREE-n00010-AX",
            1116,
            591,
            534,
            Some(172),
            389,
            17,
        ),
        ("RF-LARGE-TREE-n00010-ROT", 1116, 591, 534, None, 595, 17),
        (
            "RF-LARGE-TREE-n00100-AX",
            10836,
            5721,
            5664,
            Some(1792),
            4933,
            19,
        ),
        (
            "RF-LARGE-TREE-n00100-ROT",
            10836,
            5721,
            5664,
            None,
            6715,
            17,
        ),
    ];
    for (id, stored, lower, f_lower, nonzero, profile, bandwidth) in rows {
        let c = counts_of(id);
        assert_eq!(
            (c.pattern_entries, c.lower_entries, c.free_lower_entries),
            (stored, lower, f_lower),
            "{id}"
        );
        if let Some(nonzero) = nonzero {
            assert_eq!(c.free_lower_nonzero, nonzero, "{id}");
        }
        assert_eq!(
            (c.rcm_profile_entries, c.rcm_half_bandwidth),
            (profile, bandwidth),
            "{id}"
        );
    }
}

/// The independent oracle's values (Python port of the RCM, the skyline rule
/// and F1b's identity rule, run on the binary's dumped positions): (id, free
/// lower nonzero, RCM profile, RCM half-bandwidth, identity profile, identity
/// half-bandwidth, lane entries).
const ORACLE: [(&str, usize, u128, usize, u128, usize, usize); 21] = [
    ("RF-LARGE-CHAIN-n00010-AX", 152, 168, 4, 444, 10, 242),
    ("RF-LARGE-CHAIN-n00010-ROT", 396, 529, 13, 524, 11, 686),
    ("RF-LARGE-CHAIN-n00100-AX", 1592, 1788, 4, 4764, 10, 2582),
    ("RF-LARGE-CHAIN-n00100-ROT", 4176, 5569, 13, 5564, 11, 7346),
    ("RF-LARGE-TREE-n00010-AX", 172, 389, 17, 590, 16, 242),
    ("RF-LARGE-TREE-n00010-ROT", 442, 595, 17, 674, 17, 686),
    ("RF-LARGE-TREE-n00100-AX", 1792, 4933, 19, 6530, 16, 2582),
    ("RF-LARGE-TREE-n00100-ROT", 4672, 6715, 17, 7379, 17, 7346),
    ("RF-LARGE-CONT-n00010-AX", 90, 98, 3, 675, 33, 152),
    ("RF-LARGE-CONT-n00010-ROT", 225, 279, 10, 832, 35, 407),
    ("RF-LARGE-CONT-n00100-AX", 945, 1043, 3, 57510, 303, 1592),
    ("RF-LARGE-CONT-n00100-ROT", 2385, 3024, 10, 69232, 305, 4277),
    (
        "DEC053:invented-cantilever-chain-8",
        120,
        132,
        4,
        348,
        10,
        190,
    ),
    (
        "DEC053:invented-cantilever-chain-24",
        376,
        420,
        4,
        1116,
        10,
        606,
    ),
    (
        "DEC053:invented-cantilever-chain-48",
        760,
        852,
        4,
        2268,
        10,
        1230,
    ),
    ("DEC053:invented-grid-frame-4x3", 164, 308, 9, 782, 29, 292),
    (
        "DEC053:invented-grid-frame-6x8",
        1002,
        3980,
        22,
        8462,
        41,
        1894,
    ),
    (
        "DEC053:invented-grid-frame-7x8",
        1176,
        5036,
        25,
        11390,
        47,
        2240,
    ),
    (
        "DEC053:invented-grid-frame-5x5",
        456,
        1398,
        18,
        3086,
        35,
        846,
    ),
    (
        "DEC053:invented-cantilever-chain-32",
        504,
        564,
        4,
        1500,
        10,
        814,
    ),
    (
        "DEC053:invented-grid-frame-5x6",
        580,
        1938,
        19,
        4046,
        35,
        1080,
    ),
];

/// C3: the counts equal the oracle's.
#[test]
fn oracle_constants() {
    for (id, nonzero, profile, bandwidth, identity, identity_bandwidth, lane) in ORACLE {
        let c = counts_of(id);
        assert_eq!(
            (
                c.free_lower_nonzero,
                c.rcm_profile_entries,
                c.rcm_half_bandwidth,
                c.identity_profile_entries,
                c.identity_half_bandwidth,
                c.lane_entries
            ),
            (
                nonzero,
                profile,
                bandwidth,
                identity,
                identity_bandwidth,
                lane
            ),
            "{id}"
        );
    }
}

/// C3: K6's O(nnz) RCM count equals `order_sparse_structural`'s, which
/// allocates the profile (K1 review N3).
#[test]
fn rcm_count_equals_ordering() {
    for id in small_ids() {
        let m = model(&id).unwrap();
        let frames = m.frames().unwrap();
        let c = counts_of(&id);
        let k = assemble_sparse_stiffness(
            m.node_count(),
            &frames,
            &[],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        )
        .unwrap();
        let evidence =
            SparseAssemblyEvidence::new(k.pattern(), m.node_count(), &frames, &[], &[], &[])
                .unwrap();
        let restrained = m.restrained_dofs();
        let free: Vec<usize> = (0..m.dofs()).filter(|d| !restrained.contains(d)).collect();
        let prescribed: Vec<(usize, f64)> = restrained.iter().map(|&d| (d, 0.0)).collect();
        let force = vec![0.0; m.dofs()];
        let system = SparseStructuralSystem::new(
            &k,
            &force,
            &free,
            &prescribed,
            None,
            Some(SparseSymmetryEvidence {
                absolute_roundoff: evidence.absolute_roundoff(),
                operation_counts: evidence.operation_counts(),
                basis: FRAME_FAMILY_SYMMETRY_BASIS,
            }),
        );
        let prepared = prepare_sparse_structural(&system).unwrap();
        let ordering = order_sparse_structural(&prepared).unwrap();
        assert_eq!(
            (
                ordering.profile_entry_count as u128,
                ordering.max_half_bandwidth
            ),
            (c.rcm_profile_entries, c.rcm_half_bandwidth),
            "{id}"
        );
    }
}
