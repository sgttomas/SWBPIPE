//! Tests of the sparse admission estimate (DERIVATION.md draft 2, part 1).
use super::*;
use std::mem::size_of;

/// One small counts value, used for every hand computation below.
/// ε = E + C + T + B = 1 + 1 + 1 + 1 = 4; c = 144·4 + 1 = 577;
/// F = 6·7/2 = 21; L = (144 + 6)/2 = 75; a = r₀ + k = 6 + 1 = 7.
/// n = 12, so m = min(n, 256) = 12 for the small-model terms.
fn small() -> Counts {
    Counts {
        N: 2,
        n: 12,
        r0: 6,
        f0: 6,
        k: 1,
        k_f: 1,
        k_x: 0,
        d_bar: 1,
        E: 1,
        C: 1,
        T: 1,
        C_r: 1,
        B: 1,
        s: 1,
        z: 144,
        p_b: 2,
        g: 1,
        sigma_k: 5,
        sigma_B: 4,
        sigma_x: 0,
        sigma_d: 10,
        sigma_g: 3,
        I: 4,
    }
}

#[test]
fn derived_symbols_by_hand() {
    let c = small();
    assert_eq!(c.eps(), Ok(4)); // 1 + 1 + 1 + 1
    assert_eq!(c.c(), Ok(577)); // 144·4 + 1
    assert_eq!(c.F(), Ok(21)); // 6·7/2
    assert_eq!(c.L(), Ok(75)); // (144 + 6)/2
    assert_eq!(c.m(), 12); // min(12, 256)
    let mut big = c;
    big.n = 1_000_000;
    assert_eq!(big.m(), 256); // min(10⁶, 256)
}

#[test]
fn h_in_by_hand() {
    // 152·1 + 712·1 + 3504·1 + 4 + 8·12 + 8·6 + 16·max(4, 2) + 32·1 + 3 + 104·1
    // + 2·5 + 32·0 + 0 + 64·1 + 10 + 48 + 16·1
    // = 152 + 712 + 3504 + 4 + 96 + 48 + 64 + 32 + 3 + 104 + 10 + 0 + 0 + 64 + 10 + 48 + 16
    let expected = 152 + 712 + 3504 + 4 + 96 + 48 + 64 + 32 + 3 + 104 + 10 + 64 + 10 + 48 + 16;
    assert_eq!(expected, 4867);
    assert_eq!(H_in(&small()), Ok(4867));
}

#[test]
fn h_loop_by_hand() {
    let stiffness = 8 * 12 + 8 + 32 * 144; // 4712
    let evidence_pattern = 8 * 12 + 8 + 32 * 144; // 4712
    let contributions = 48 * 577; // 27,696
    let edges_groups_spring_ground = 48 * 4 + 16 * (4 + 1) + 16 * 1; // 288
    let coordinates = 32 * 2; // 64
    let formation = 152 + 1200 + 4 + 16 + 1424 + 1032 + 496; // 4324
    let states = 32 * 1 + 5 + 0 * 1; // 37, κ_k = 0 (OPEN placeholder)
    let gap_views = 3 * (24 * 12 + 8 * 12 * 12); // 4320, m = min(12, 256) = 12
    let expected = stiffness
        + evidence_pattern
        + contributions
        + edges_groups_spring_ground
        + coordinates
        + formation
        + states
        + gap_views;
    assert_eq!(expected, 46_153);
    assert_eq!(H_loop(&small()), Ok(46_153));
}

#[test]
fn h_asm_peak_by_hand() {
    let blocks = 1168 * 1;
    let formed = 1168 * std::cmp::max(1 + 1, 2 * (1 + 1 + 1)); // 1168·6 = 7008
    let connectivity = 16 * 4 + 8 * 1; // 72
    let pattern_builder = 56 * 2 + 64 * 4 + 33 * 12 + 40 * 144 + 8; // 6532
    let magnitudes = 16 * 144; // 2304
    let expected = blocks + formed + connectivity + pattern_builder + magnitudes;
    assert_eq!(expected, 17_084);
    assert_eq!(H_asm_peak(&small()), Ok(17_084));
}

#[test]
fn r_sr_r_pe_d_as_by_hand() {
    // R_sr = 468·6 + 96·144 + 8(2·577 + 144) + 340 = 2808 + 13,824 + 10,384 + 340
    assert_eq!(468 * 6 + 96 * 144 + 8 * (2 * 577 + 144) + 340, 27_356);
    assert_eq!(R_sr(&small()), Ok(27_356));
    // R_pe = 272·6
    assert_eq!(R_pe(&small()), Ok(1632));
    // D_as = 774 + 6·5 + 24·1
    assert_eq!(D_as(&small()), Ok(828));
}

/// R_it of [`small`] without the iterations Vec, by hand.
fn small_r_it_without_iterations_vec() -> u128 {
    let r_sr = 27_356;
    let r_pe = 1632;
    let displacements_reactions = 16 * 12; // 192
    let boundary_copies = 16 * (6 + 1); // 112
    let active_set = 32 * 1 + 5 + 48 * 1 + 5 + 828; // 918; Σids = 1·σ_k (OPEN)
    let friction_forces = 80 * 1 + 5; // 85; ids = 1·σ_k (OPEN)
    let strings = 222;
    let strict_gap = 5 * 1024 * 1024; // max(185, OPEN draft 5 MiB at m)
    let recovery = 2 * (400 + 2 * 5 + 20 * 1 + 42 * (6 + 1)); // 2·724 = 1448
    let total = r_sr
        + r_pe
        + displacements_reactions
        + boundary_copies
        + active_set
        + friction_forces
        + strings
        + strict_gap
        + recovery;
    assert_eq!(total, 31_965 + 5_242_880);
    total
}

#[test]
fn r_it_by_hand() {
    let s_iter = size_of::<NonlinearFrameIteration>() as u128;
    assert_eq!(
        R_it(&small()),
        Ok(small_r_it_without_iterations_vec() + 4 * s_iter)
    );
}

#[test]
fn x_by_hand() {
    let r_it = R_it(&small()).unwrap();
    // 16·12 + 2566 + 4867 + 16·1 + 8·12 + 8 + 24·144 + 8·12 + 16·6 + 16·7
    // + [4·R_it + 16·12 + 32·1 + 5 + 2662] + 544 (size_of::<ProductCompletion>(), pinned)
    let outside = 192 + 2566 + 4867 + 16 + 96 + 8 + 3456 + 96 + 96 + 112 + 544;
    assert_eq!(outside, 12_049);
    let bracket_fixed = 192 + 32 + 5 + 2662;
    assert_eq!(bracket_fixed, 2891);
    assert_eq!(X(&small()), Ok(12_049 + 2891 + 4 * r_it));
}

#[test]
fn t_phases_by_hand() {
    let c = small();
    let prep = 56 * 144 + 20 * 6 + 16 * 12 + 339; // 8715
    let aud = 344 * 144 + 48 * 577 + 12 + 384; // 77,628
    let ord = 144 * 75 + 170 * 6 + 16 * 21; // 12,156
    let red = 24 * 6 + 9 * 12 + 16 * 7 + 8 * 12; // 460
    let s = 8 * 21 + 16 * 144 + 156 * 6 + 8 + 41 * 6; // 3662
    let fin = 8 * 12 + 168 * 144 + 32 * 577 + 548 * 6; // 46,040
    let eq = 8 * 12 + 272 * 6; // 1728
    let obs = 144 * 75 + 176 * 6 + 16 * 21 + 24 * 21; // 12,696
    let v = 48 * 12; // 576
    let f = 24 * 1 * 1 + 72 * 1; // 96
    let base = 24 * 12 + 27_356 + 1632; // 29,276, d̄ = 1 ≥ 1
    let gap = 92 * 12 + 16 * 6 + 16 * 7 + 300; // 1612
    let asi = 176 * 1 + 3 * 5; // 191
    let geo = 17 * 2 + 0 + 340; // 374, ρ = 0 (OPEN placeholder)
    assert_eq!(
        (prep, aud, ord, red, s, fin, eq, obs),
        (8715, 77_628, 12_156, 460, 3662, 46_040, 1728, 12_696)
    );
    assert_eq!(
        (v, f, base, gap, asi, geo),
        (576, 96, 29_276, 1612, 191, 374)
    );
    assert_eq!(T_Prep(&c), Ok(prep));
    assert_eq!(T_Aud(&c), Ok(aud));
    assert_eq!(T_Ord(&c), Ok(ord));
    assert_eq!(T_Red(&c), Ok(red));
    assert_eq!(T_S(&c), Ok(s));
    assert_eq!(T_Fin(&c), Ok(fin));
    assert_eq!(T_Eq(&c), Ok(eq));
    assert_eq!(T_Obs(&c), Ok(obs));
    assert_eq!(T_V(&c), Ok(v));
    assert_eq!(T_F(&c), Ok(f));
    assert_eq!(T_Base(&c), Ok(base));
    assert_eq!(T_Gap(&c), Ok(gap));
    assert_eq!(T_ASI(&c), Ok(asi));
    assert_eq!(T_Geo(&c), Ok(geo));
    // max(98,959, 136,505, 5390, 16,358) = 136,505
    let phases = [
        prep + aud + ord + red,
        s + prep + aud + fin + red,
        s + eq,
        s + obs,
    ];
    assert_eq!(phases, [98_959, 136_505, 5390, 16_358]);
    let t = 136_505 + v + f + base + gap + asi + geo;
    assert_eq!(t, 168_630);
    assert_eq!(T(&c), Ok(168_630));
}

#[test]
fn m_loop_by_hand() {
    let r_it = R_it(&small()).unwrap();
    // H_in + H_loop + H_asm_peak + I·R_it + X + T
    // = 4867 + 46,153 + 17,084 + 4·R_it + (14,940 + 4·R_it) + 168,630
    let expected = 4867 + 46_153 + 17_084 + 4 * r_it + (14_940 + 4 * r_it) + 168_630;
    assert_eq!(M_loop(&small()), Ok(expected));
    let b = breakdown(&small()).unwrap();
    assert_eq!(b.M_loop, expected);
    assert_eq!(
        b.M_loop,
        b.H_in + b.H_loop + b.H_asm_peak + b.I_times_R_it + b.X + b.T
    );
}

#[test]
fn branches_by_hand() {
    // Base is 0 when d̄ = 0.
    let mut c = small();
    c.d_bar = 0;
    assert_eq!(T_Base(&c), Ok(0));
    // Above 256 DOFs the small-model terms are priced at m = min(257, 256) = 256:
    // gap views 3(24·256 + 8·256²) = 3(6144 + 524,288) = 1,591,296, and the
    // strict-gap evidence max(185, 5 MiB) = 5,242,880.
    let mut c = small();
    c.n = 257;
    let gap_views_256 = 3 * (24 * 256 + 8 * 256 * 256);
    assert_eq!(gap_views_256, 1_591_296);
    let h_loop_257 = 2 * (8 * 257 + 8 + 32 * 144) + 27_696 + 288 + 64 + 4324 + 37 + gap_views_256;
    assert_eq!(H_loop(&c), Ok(h_loop_257));
    let s_iter = size_of::<NonlinearFrameIteration>() as u128;
    let r_it_257 = 27_356 + 1632 + 16 * 257 + 112 + 918 + 85 + 222 + 5_242_880 + 1448 + 4 * s_iter;
    assert_eq!(R_it(&c), Ok(r_it_257));
}

// ---------------------------------------------------------------------------
// Monotonicity.
// ---------------------------------------------------------------------------

/// SplitMix64, fixed seed: deterministic pseudo-random counts.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A count with a uniformly chosen bit length 0..=20 (so 0 to ~10⁶).
    fn count(&mut self) -> u64 {
        let bits = self.next() % 21;
        if bits == 0 {
            0
        } else {
            self.next() & ((1u64 << bits) - 1)
        }
    }
}

const FIELDS: [&str; 23] = [
    "N", "n", "r0", "f0", "k", "k_f", "k_x", "d_bar", "E", "C", "T", "C_r", "B", "s", "z", "p_b",
    "g", "sigma_k", "sigma_B", "sigma_x", "sigma_d", "sigma_g", "I",
];

fn field(c: &mut Counts, index: usize) -> &mut u64 {
    match index {
        0 => &mut c.N,
        1 => &mut c.n,
        2 => &mut c.r0,
        3 => &mut c.f0,
        4 => &mut c.k,
        5 => &mut c.k_f,
        6 => &mut c.k_x,
        7 => &mut c.d_bar,
        8 => &mut c.E,
        9 => &mut c.C,
        10 => &mut c.T,
        11 => &mut c.C_r,
        12 => &mut c.B,
        13 => &mut c.s,
        14 => &mut c.z,
        15 => &mut c.p_b,
        16 => &mut c.g,
        17 => &mut c.sigma_k,
        18 => &mut c.sigma_B,
        19 => &mut c.sigma_x,
        20 => &mut c.sigma_d,
        21 => &mut c.sigma_g,
        22 => &mut c.I,
        _ => unreachable!("23 fields"),
    }
}

fn random_counts(rng: &mut SplitMix64) -> Counts {
    let mut c = small();
    for index in 0..FIELDS.len() {
        *field(&mut c, index) = rng.count();
    }
    c.I = rng.next() % 9;
    c
}

/// Every single-count increment of `base` that lowers M_loop, as messages.
fn decreases(base: &Counts) -> Vec<String> {
    let before = M_loop(base).expect("monotonicity vectors stay far below u128::MAX");
    let mut found = Vec::new();
    for index in 0..FIELDS.len() {
        let mut next = *base;
        *field(&mut next, index) += 1;
        let after = M_loop(&next).expect("monotonicity vectors stay far below u128::MAX");
        if after < before {
            found.push(format!(
                "{} {} -> {}: M_loop {before} -> {after} in {base:?}",
                FIELDS[index],
                *field(&mut base.clone(), index),
                *field(&mut next, index),
            ));
        }
    }
    found
}

#[test]
fn m_loop_is_monotone_on_pseudo_random_counts() {
    let mut rng = SplitMix64(0x5357_4250_4950_4531); // fixed seed
    let mut found = Vec::new();
    for _ in 0..400 {
        found.extend(decreases(&random_counts(&mut rng)));
    }
    assert!(
        found.is_empty(),
        "M_loop decreased on {} single-count increments (DERIVATION.md §3 says it is monotone):\n{}",
        found.len(),
        found.join("\n")
    );
}

/// Draft 1 applied the gap views and the strict-gap small-model bound only at
/// n ≤ 256, so M_loop fell between n = 256 and 257 (the finding; this test
/// failed on draft 1). Draft 2 prices both at m = min(n, 256) for every n.
/// This checks monotonicity across the boundary: every single-count increment
/// at n = 256, and M_loop over n = 240 to 272.
#[test]
fn m_loop_is_monotone_at_the_small_model_boundary() {
    let mut base = small();
    base.n = 256;
    let found = decreases(&base);
    assert!(
        found.is_empty(),
        "M_loop is not monotone at the small-model boundary (DERIVATION.md draft 2, \
         change 1):\n{}",
        found.join("\n")
    );
    let mut previous = 0;
    for n in 240..=272 {
        let mut c = small();
        c.n = n;
        let total = M_loop(&c).unwrap();
        assert!(
            total >= previous,
            "M_loop fell at n = {n}: {previous} -> {total}"
        );
        previous = total;
    }
}

// ---------------------------------------------------------------------------
// Overflow.
// ---------------------------------------------------------------------------

fn all(value: u64) -> Counts {
    let mut c = small();
    for index in 0..FIELDS.len() {
        *field(&mut c, index) = value;
    }
    c
}

#[test]
fn counts_near_u64_max_return_the_error() {
    for value in [u64::MAX, u64::MAX - 1, u64::MAX / 2] {
        let c = all(value);
        assert!(
            matches!(M_loop(&c), Err(SparseAdmissionError::Overflow { .. })),
            "all counts at {value}"
        );
        assert!(breakdown(&c).is_err());
    }
    // A single quadratic count suffices: 24·F with f₀ near u64::MAX, and 24d̄².
    let mut c = small();
    c.f0 = u64::MAX;
    assert_eq!(
        T_Ord(&c),
        Err(SparseAdmissionError::Overflow { term: "Ord" })
    );
    assert!(M_loop(&c).is_err());
    let mut c = small();
    c.d_bar = u64::MAX;
    assert_eq!(
        T_F(&c),
        Err(SparseAdmissionError::Overflow {
            term: "F (coupling)"
        })
    );
    assert!(M_loop(&c).is_err());
    // I·R_it with both near the top.
    let mut c = all(u64::MAX / 4096);
    c.I = u64::MAX;
    assert!(M_loop(&c).is_err());
}

#[test]
fn every_single_count_at_u64_max_returns_without_panicking() {
    for index in 0..FIELDS.len() {
        for value in [u64::MAX, u64::MAX - 1] {
            let mut c = small();
            *field(&mut c, index) = value;
            // Either a value or the error; a panic fails the test.
            let total = M_loop(&c);
            let parts = breakdown(&c);
            assert_eq!(total.is_ok(), parts.is_ok(), "{}", FIELDS[index]);
            if let (Ok(total), Ok(parts)) = (total, parts) {
                assert_eq!(total, parts.M_loop, "{}", FIELDS[index]);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Size pins.
// ---------------------------------------------------------------------------

macro_rules! pin {
    ($wrong:ident, $ty:ty, $measured:expr, $uses:expr) => {
        let actual = size_of::<$ty>();
        println!("size_of::<{}>() = {actual}", stringify!($ty));
        if actual != $measured {
            $wrong.push(format!(
                "size_of::<{}>() is {actual}, pinned {}. It enters {}. Re-examine \
                 DERIVATION.md §3 and ACCOUNTING.md before changing this pin.",
                stringify!($ty),
                $measured,
                $uses
            ));
        }
    };
}

#[test]
fn size_pins() {
    use open_pipe_stress_frame_kernel::connector::ObjectiveConnector;
    use open_pipe_stress_frame_kernel::structural::exact_boundary::ForceContribution;
    use open_pipe_stress_frame_kernel::structural::{
        ContributionRounding, PivotEvidence, ResidualRow, SparseStiffness, StiffnessBlock,
        StiffnessContribution, StructuralReport,
    };
    use open_pipe_stress_frame_kernel::tie_rod::TieRod;
    use open_pipe_stress_frame_kernel::{FrameElement, Matrix12};
    use open_pipe_stress_nonlinear_supports::{
        ActiveSetIteration, NonlinearSupport, SupportStateRecord, TrialSupportState,
    };
    use open_pipe_stress_solver_diagnostics::SolverDiagnostic;
    use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;

    use crate::product_equilibrium::{ProductEquilibriumReport, WorkRow};
    use crate::structural_adapter::{ExactGapReport, ExactGapWork, StrictGapEvidence};
    let mut wrong: Vec<String> = Vec::new();
    use crate::{
        AppliedSlidingFrictionForce, CurvedBendStiffnessElement, DerivedFrictionNormalReaction,
        FrictionNormalReaction, LinearizedSolve, LoopStiffness, NonlinearFrameSolveInput,
        NonlinearFrameSolveResult, ProductCompletion, SlidingFrictionCandidate,
        SlidingFrictionEvidence, SparseLinearSolveEvidence,
    };

    pin!(
        wrong,
        NonlinearFrameIteration,
        PIN_S_ITER,
        "S_iter: R_it's 4·S_iter and X"
    );
    pin!(
        wrong,
        ProductCompletion,
        544,
        "X's completion box (COMPLETION_BOX)"
    );
    pin!(wrong, LinearizedSolve, 664, "T's Base (held base solve)");
    pin!(
        wrong,
        ExactGapReport,
        1592,
        "S_iter (StrictGapEvidence::Qualified inline)"
    );
    pin!(wrong, StrictGapEvidence, 1592, "S_iter");
    pin!(wrong, ExactGapWork, 72, "S_iter");
    pin!(
        wrong,
        LoopStiffness,
        104,
        "X's completion box, inside ProductCompletion"
    );
    pin!(
        wrong,
        NonlinearFrameSolveInput,
        328,
        "X's completion box (input 328)"
    );
    pin!(
        wrong,
        NonlinearFrameSolveResult,
        208,
        "X's completion output box"
    );
    pin!(wrong, SparseLinearSolveEvidence, 256, "S_iter");
    pin!(wrong, ProductEquilibriumReport, 88, "S_iter");
    pin!(wrong, StructuralReport, 248, "S_iter");
    pin!(wrong, ActiveSetIteration, 96, "S_iter");
    pin!(wrong, WorkRow, 64, "R_pe and Eq (64f work_rows)");
    pin!(
        wrong,
        ResidualRow,
        104,
        "R_sr, R_pe, Fin, Eq (208f per row vector)"
    );
    pin!(wrong, PivotEvidence, 48, "R_sr and S (48f, 96f)");
    pin!(wrong, ContributionRounding, 96, "R_sr (96R) and Aud (192R)");
    pin!(
        wrong,
        StiffnessContribution,
        24,
        "H_loop contributions (48c)"
    );
    pin!(wrong, SymmetricMatrixEntry, 24, "Ord and Obs entries (48L)");
    pin!(wrong, StiffnessBlock, 1168, "H_asm_peak blocks (1168B)");
    pin!(
        wrong,
        (usize, usize, Matrix12),
        1168,
        "H_asm_peak formed (1168·…)"
    );
    pin!(
        wrong,
        FrameElement,
        152,
        "H_in (152E) and H_loop formation (152E)"
    );
    pin!(
        wrong,
        ObjectiveConnector,
        712,
        "H_in (712C) and H_loop formation (1424C)"
    );
    pin!(wrong, TieRod, 248, "H_loop formation (1032C_r + 496T)");
    pin!(wrong, CurvedBendStiffnessElement, 3504, "H_in (3504B)");
    pin!(wrong, NonlinearSupport, 72, "H_in (104k) and ASI (176k)");
    pin!(
        wrong,
        SupportStateRecord,
        32,
        "H_in, H_loop, R_it and X (32k)"
    );
    pin!(wrong, TrialSupportState, 72, "ASI (176k)");
    pin!(wrong, FrictionNormalReaction, 32, "H_in (32k_x)");
    pin!(wrong, DerivedFrictionNormalReaction, 64, "H_in (64d̄)");
    pin!(
        wrong,
        AppliedSlidingFrictionForce,
        40,
        "R_it friction forces (80k)"
    );
    pin!(
        wrong,
        SlidingFrictionCandidate,
        64,
        "per-iterate O(k) transients"
    );
    pin!(
        wrong,
        SlidingFrictionEvidence,
        48,
        "per-iterate O(k) transients"
    );
    pin!(wrong, SolverDiagnostic, 304, "D_as (304 per diagnostic)");
    pin!(wrong, ForceContribution, 40, "Gap (92n)");
    pin!(wrong, SparseStiffness, 104, "the loop stiffness's struct");
    pin!(wrong, Option<[f64; 3]>, 32, "ACCOUNTING §1 struct sizes");
    assert!(wrong.is_empty(), "layout changed:\n{}", wrong.join("\n"));
}

/// S_iter as measured on rustc 1.97.1, aarch64-apple-darwin.
const PIN_S_ITER: usize = 2512;

// ---------------------------------------------------------------------------
// The reference rack.
// ---------------------------------------------------------------------------

/// The reference rack's counts (nonlinear-ex, one OPE case, I = 4).
/// N, n, r₀, f₀ (all-inactive free), k (nonlinear DOFs) and z are the design's
/// rack_counts.jsonl; the rest were counted from the same rack model documents
/// by the product's build rules (ESTIMATE.md gives the method and sources).
fn rack(nodes: u64) -> Counts {
    let (r0, f0, k, k_f, E, B, z, sigma_k, sigma_B, sigma_d) = match nodes {
        1000 => (335, 5665, 74, 25, 903, 96, 107_928, 1406, 1920, 925),
        1500 => (496, 8504, 112, 38, 1355, 144, 161_928, 2128, 2880, 1406),
        1750 => (577, 9923, 131, 44, 1581, 168, 188_928, 2489, 3360, 1628),
        2000 => (657, 11_343, 150, 50, 1807, 192, 215_928, 2850, 3840, 1850),
        _ => unreachable!("four rack sizes"),
    };
    Counts {
        N: nodes,
        n: 6 * nodes,
        r0,
        f0,
        k,
        k_f,
        k_x: 0,
        d_bar: k_f,
        E,
        C: 0,
        T: 0,
        C_r: 0,
        B,
        s: 0,
        z,
        p_b: 0,
        g: 0,
        sigma_k,
        sigma_B,
        sigma_x: 0,
        sigma_d,
        sigma_g: 0,
        I: 4,
    }
}

#[test]
fn the_reference_rack() {
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    // The design's candidate B and dense estimate (GiB), for comparison only.
    let design = [
        (1000, 0.90, 3.43),
        (1500, 1.89, 7.72),
        (1750, 2.52, 10.50),
        (2000, 3.24, 13.72),
    ];
    println!("nodes | M_loop bytes | M_loop GiB | candidate B GiB | dense GiB");
    for (nodes, b, dense) in design {
        let parts = breakdown(&rack(nodes)).expect("the rack fits in u128");
        println!(
            "{nodes} | {} | {:.3} | {b:.2} | {dense:.2}",
            parts.M_loop,
            parts.M_loop as f64 / GIB
        );
        println!("  breakdown (MiB): {}", {
            let p = parts;
            format!(
                "H_in {:.2}; H_loop {:.2}; H_asm_peak {:.2}; R_it {:.2} (R_sr {:.2}, R_pe {:.2}); \
                 I·R_it {:.2}; X {:.2}; T {:.2} [Prep {:.2}, Aud {:.2}, Ord {:.2}, Red {:.2}, \
                 S {:.2}, Fin {:.2}, Eq {:.2}, Obs {:.2}, V {:.2}, F {:.4}, Base {:.2}, \
                 Gap {:.2}, ASI {:.4}, Geo {:.4}]; F = {}, L = {}, c = {}, S_iter = {}",
                p.H_in as f64 / MIB,
                p.H_loop as f64 / MIB,
                p.H_asm_peak as f64 / MIB,
                p.R_it as f64 / MIB,
                p.R_sr as f64 / MIB,
                p.R_pe as f64 / MIB,
                p.I_times_R_it as f64 / MIB,
                p.X as f64 / MIB,
                p.T as f64 / MIB,
                p.Prep as f64 / MIB,
                p.Aud as f64 / MIB,
                p.Ord as f64 / MIB,
                p.Red as f64 / MIB,
                p.S as f64 / MIB,
                p.Fin as f64 / MIB,
                p.Eq as f64 / MIB,
                p.Obs as f64 / MIB,
                p.V as f64 / MIB,
                p.F_coupling as f64 / MIB,
                p.Base as f64 / MIB,
                p.Gap as f64 / MIB,
                p.ASI as f64 / MIB,
                p.Geo as f64 / MIB,
                p.F,
                p.L,
                p.c,
                p.S_iter
            )
        });
    }
}
