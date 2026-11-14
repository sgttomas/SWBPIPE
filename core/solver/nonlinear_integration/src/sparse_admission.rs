//! Sparse-mode admission estimate for one nonlinear active-set loop call (part 1).
//!
//! This module implements the bound of the exact route performance Type 1's
//! `nl-part1/DERIVATION.md` (draft 2), whose terms come from the source
//! accounting `nl-part1/ACCOUNTING.md` at 946553b52. Row names in the doc
//! comments below are the accounting's "Final table" rows; section numbers are
//! the derivation's.
//!
//! It is NOT wired to the product's guard. The dense guard, its formula, code
//! and message are unchanged. The function is evidence and preparation for
//! part 2.
//!
//! What it bounds (DERIVATION.md §1): the peak live requested heap of one
//! sparse active-set loop call at its iteration cap I, including the
//! caller-held input and the returned result. It does not bound the rest of
//! the invocation (§4, the reserve), allocator rounding, fragmentation or the
//! relation of live heap to `phys_footprint`.
//!
//! All arithmetic is u128 with checked operations. Overflow returns
//! [`SparseAdmissionError::Overflow`] naming the term; nothing panics, wraps or
//! saturates.
#![allow(dead_code)] // Part 1: deliberately not wired to any guard.
#![allow(non_snake_case)] // Symbols are named as DERIVATION.md §2 names them.

use crate::structural_adapter::EXACT_GAP_MAX_DOFS;
use crate::{NonlinearFrameIteration, ProductCompletion};

/// The counts of DERIVATION.md §2, all known at the product guard before any
/// case is solved. Named as there. ε and c are derived ([`Counts::eps`],
/// [`Counts::c`]), never taken as inputs. The fields are independent: the
/// function does not check n = 6N or f₀ = n − r₀; a caller forms them
/// (checked) from the model.
///
/// z_f (§2) is not a field: §3 uses its substitution z_f ≤ z throughout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    /// N: nodes (`built.nodes`).
    pub N: u64,
    /// n: 6N global DOFs.
    pub n: u64,
    /// r₀: base-restrained DOFs (`boundary.restrained_dofs`).
    pub r0: u64,
    /// f₀: n − r₀.
    pub f0: u64,
    /// k: nonlinear supports.
    pub k: u64,
    /// k_f: friction supports.
    pub k_f: u64,
    /// k_x: explicit-normal records.
    pub k_x: u64,
    /// d̄: derived-normal friction candidates.
    pub d_bar: u64,
    /// E: frame elements.
    pub E: u64,
    /// C: objective connectors.
    pub C: u64,
    /// T: tie rods, summed over connectors.
    pub T: u64,
    /// C_r: connectors with rods.
    pub C_r: u64,
    /// B: curved slots.
    pub B: u64,
    /// s: loop springs.
    pub s: u64,
    /// z: stored pattern entries of the basis stiffness (PP:3223).
    pub z: u64,
    /// p_b: base-prescribed pairs.
    pub p_b: u64,
    /// g: ground-motion records.
    pub g: u64,
    /// σ_k: Σ|support_id| bytes over the nonlinear supports (measured).
    pub sigma_k: u64,
    /// σ_B: Σ|curved element_id| bytes (measured).
    pub sigma_B: u64,
    /// σ_x: Σ|id| bytes over explicit-normal records (measured).
    pub sigma_x: u64,
    /// σ_d: Σ(|support_id| + |source_ref|) bytes over derived normals (measured).
    pub sigma_d: u64,
    /// σ_g: Σ|id| bytes over ground-motion records (measured).
    pub sigma_g: u64,
    /// I: the iteration cap, 4 in the product (PP:201).
    pub I: u64,
}

/// The estimate could not be represented in u128.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SparseAdmissionError {
    /// A checked u128 operation overflowed while forming the named term.
    Overflow { term: &'static str },
}

impl std::fmt::Display for SparseAdmissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow { term } => {
                write!(
                    f,
                    "sparse admission estimate term {term} is more than u128::MAX bytes"
                )
            }
        }
    }
}

impl std::error::Error for SparseAdmissionError {}

type Est = Result<u128, SparseAdmissionError>;

// ---------------------------------------------------------------------------
// OPEN terms (DERIVATION.md §5). Each is a draft value, not derived.
// ---------------------------------------------------------------------------

/// κ_k: the friction maps' bytes per support ("O(k)", NI:2309–2366).
/// OPEN: draft value from ACCOUNTING, not derived. The accounting gives no
/// figure (hashbrown layout not read), so this is a PLACEHOLDER of 0.
pub const OPEN_KAPPA_K: u128 = 0;

/// ρ: the rigid-body assessment's internals in the geometry screen
/// (`assess_rigid_body`, not read).
/// OPEN: draft value from ACCOUNTING, not derived. The accounting gives no
/// figure, so this is a PLACEHOLDER of 0.
pub const OPEN_RHO: u128 = 0;

/// D_as constant part: D_as ≈ 774 + 6σ_k + 24k (ACCOUNTING §3; row "D_as").
/// OPEN: draft value from ACCOUNTING, not derived.
pub const OPEN_D_AS_CONSTANT: u128 = 774;
/// D_as coefficient of σ_k. OPEN: draft value from ACCOUNTING, not derived.
pub const OPEN_D_AS_PER_SIGMA_K: u128 = 6;
/// D_as coefficient of k. OPEN: draft value from ACCOUNTING, not derived.
pub const OPEN_D_AS_PER_K: u128 = 24;

/// Σids in the active set: the changed supports' id clones (NS:485), taken as
/// Σids ≤ 1·σ_k (each support changes at most once per iterate).
/// OPEN: draft value from ACCOUNTING ("bounded by σ_k terms"), not derived.
pub const OPEN_SIGMA_IDS_PER_SIGMA_K: u128 = 1;

/// "ids" of the applied friction forces (row "Applied friction forces",
/// NI:1801–1815), taken as ≤ 1·σ_k. DERIVATION.md §3 writes "80k + ids" and
/// does not list this in §5; it is made explicit here so that nothing is
/// hidden. OPEN: draft value from ACCOUNTING (β_it, §2), not derived.
pub const OPEN_FRICTION_IDS_PER_SIGMA_K: u128 = 1;

/// The strict-gap evidence's small-model bound (row "Strict gap evidence":
/// "bounded (n ≤ 256)"), priced at m = min(n, 256) for every n (DERIVATION.md
/// draft 2, change 1). OPEN: draft value from ACCOUNTING, not derived. The
/// accounting's only figure here is the exact context's coarse bound,
/// "≤ 5 MiB at n = 256" (ACCOUNTING §2, "Small-model exact paths"); it gives
/// no figure for the retained `Qualified` report itself, and no form in m, so
/// the draft value is this one constant at every m.
pub const OPEN_STRICT_GAP_SMALL_MODEL: u128 = 5 * 1024 * 1024;

// ---------------------------------------------------------------------------
// Fixed figures that are not OPEN, named once.
// ---------------------------------------------------------------------------

/// The strict-gap evidence above 256 DOFs: the 185-byte Unsupported reason
/// (SA:3386; row "Strict gap evidence").
const STRICT_GAP_UNSUPPORTED_REASON: u128 = 185;

/// The small-model exact paths apply at n ≤ this (SA:3312); their terms are
/// priced at m = min(n, this) for every n (DERIVATION.md draft 2, change 1).
const SMALL_MODEL_MAX_DOFS: u128 = EXACT_GAP_MAX_DOFS as u128;

/// S_iter = size_of::<NonlinearFrameIteration>(), pinned at 2,512 by a test
/// (DERIVATION.md draft 2, change 3).
const S_ITER: u128 = std::mem::size_of::<NonlinearFrameIteration>() as u128;

/// The completion box: size_of::<ProductCompletion>(), pinned at 544 by a test.
/// It replaces the accounting's ≈ 552 in X (DERIVATION.md draft 2, change 3).
/// `LoopStiffness` (pinned at 104, against the accounting's ≈ 112) enters the
/// bound only inside this box.
const COMPLETION_BOX: u128 = std::mem::size_of::<ProductCompletion>() as u128;

/// The iterations Vec's element count as DERIVATION.md §3 writes it ("4·S_iter";
/// row "Iterations Vec", NI:1037).
const ITERATIONS_VEC_ELEMENTS: u128 = 4;

// ---------------------------------------------------------------------------
// Checked helpers.
// ---------------------------------------------------------------------------

fn w(value: u64) -> Option<u128> {
    Some(u128::from(value))
}

fn mul(a: u128, b: Option<u128>) -> Option<u128> {
    a.checked_mul(b?)
}

fn mul2(a: Option<u128>, b: Option<u128>) -> Option<u128> {
    a?.checked_mul(b?)
}

fn sum<const K: usize>(terms: [Option<u128>; K]) -> Option<u128> {
    terms
        .into_iter()
        .try_fold(0u128, |acc, term| acc.checked_add(term?))
}

fn max<const K: usize>(terms: [Option<u128>; K]) -> Option<u128> {
    terms
        .into_iter()
        .try_fold(0u128, |acc, term| Some(acc.max(term?)))
}

fn named(value: Option<u128>, term: &'static str) -> Est {
    value.ok_or(SparseAdmissionError::Overflow { term })
}

impl Counts {
    /// ε = E + C + T + B (DERIVATION.md §2).
    pub fn eps(&self) -> Est {
        named(sum([w(self.E), w(self.C), w(self.T), w(self.B)]), "eps")
    }

    /// c = 144ε + s; the loop passes no skew blocks (NI:2669, SA:695).
    pub fn c(&self) -> Est {
        named(sum([mul(144, Some(self.eps()?)), w(self.s)]), "c")
    }

    /// F = f₀(f₀ + 1)/2: P, P₀ ≤ F for any skyline under any order (§2).
    /// Formed as a checked u128 product, then halved (ACCOUNTING §7.2).
    pub fn F(&self) -> Est {
        let product = mul2(w(self.f0), sum([w(self.f0), Some(1)]));
        named(product.map(|p| p / 2), "F")
    }

    /// L ≤ (z + f₀)/2 (§2). L is an integer count, so the floor of the
    /// half is still an upper bound.
    pub fn L(&self) -> Est {
        named(sum([w(self.z), w(self.f0)]).map(|v| v / 2), "L")
    }

    /// r₀ + k, the bound on the active boundary a (NI:2415–2422).
    fn a(&self) -> Option<u128> {
        sum([w(self.r0), w(self.k)])
    }

    /// m = min(n, 256), the DOF count at which the small-model terms are
    /// priced for every n (DERIVATION.md draft 2, change 1).
    pub fn m(&self) -> u128 {
        u128::from(self.n).min(SMALL_MODEL_MAX_DOFS)
    }
}

// ---------------------------------------------------------------------------
// §3 terms.
// ---------------------------------------------------------------------------

/// H_in, the caller-held input. ACCOUNTING rows "Input frames" (152E, PP:7978),
/// "Input connectors" (712C, PP:7979), "Input curved slots" (3504B + σ_B,
/// PP:7966–7972), "Input force" (8n, PP:7983), "Input restrained" (8r_b,
/// PP:7984), "Input base_prescribed" (16·max(4, p_b), PP:7988–7990), "Input
/// ground motion" (32g + σ_g, PP:7993), "Input supports, states" (104k + 2σ_k,
/// PP:7994–7995), "Input friction normals" (32k_x + σ_x + 64d̄ + σ_d,
/// PP:7996–7999), "Policy ref, springs" (48 + 16s, PP:199, :7651).
///
/// The base-motion term 16·max(4, p_b) applies even with no motion, where the
/// accounting row gives 0: conservative over-count (DERIVATION §5), kept.
pub fn H_in(c: &Counts) -> Est {
    named(
        sum([
            mul(152, w(c.E)),
            mul(712, w(c.C)),
            mul(3504, w(c.B)),
            w(c.sigma_B),
            mul(8, w(c.n)),
            mul(8, w(c.r0)),
            mul(16, Some(u128::from(c.p_b).max(4))),
            mul(32, w(c.g)),
            w(c.sigma_g),
            mul(104, w(c.k)),
            mul(2, w(c.sigma_k)),
            mul(32, w(c.k_x)),
            w(c.sigma_x),
            mul(64, w(c.d_bar)),
            w(c.sigma_d),
            Some(48),
            mul(16, w(c.s)),
        ]),
        "H_in",
    )
}

/// H_loop, held for the whole loop. ACCOUNTING rows "Loop stiffness"
/// (8n + 8 + 32z, NI:787; FKS:135–148, :738), "Evidence pattern clone,
/// roundoff, counts" (8n + 8 + 32z, SA:724–726), "Evidence contributions"
/// (48c, SA:1911, :1834), "Evidence edges, groups, spring_ground"
/// (48ε + 16(ε+s) + 16s, SA:1907, :1922, :1842), "Evidence coordinates" (32N,
/// SA:1733), "Formation frames, curved, springs" (152E + 1200B + σ_B + 16s,
/// SA:1738–1750), "Formation connectors and rod Arcs" (1424C + 1032C_r + 496T,
/// SA:1725–1728), "`current_states`, friction maps" (32k + σ_k + κ_k·k, NI:2304,
/// :2309–2366; κ_k OPEN) and "Gap views" (NI:2704–2716; SA:3317–3349), priced
/// as 3(24m + 8m²) with m = min(n, 256) for every n (DERIVATION.md draft 2,
/// change 1). Above 256 DOFs the views are not allocated, so this over-counts
/// there by at most 3(24·256 + 8·256²) = 1,591,296 B.
pub fn H_loop(c: &Counts) -> Est {
    let eps = Some(c.eps()?);
    let cc = Some(c.c()?);
    let n = w(c.n);
    let m = Some(c.m());
    let gap_views = mul(3, sum([mul(24, m), mul(8, mul2(m, m))]));
    named(
        sum([
            // the loop stiffness
            sum([mul(8, n), Some(8), mul(32, w(c.z))]),
            // the evidence's pattern clone, roundoff and counts
            sum([mul(8, n), Some(8), mul(32, w(c.z))]),
            // contributions
            mul(48, cc),
            // edges, groups and spring_ground
            sum([mul(48, eps), mul(16, sum([eps, w(c.s)])), mul(16, w(c.s))]),
            // coordinates
            mul(32, w(c.N)),
            // formation
            sum([
                mul(152, w(c.E)),
                mul(1200, w(c.B)),
                w(c.sigma_B),
                mul(16, w(c.s)),
                mul(1424, w(c.C)),
                mul(1032, w(c.C_r)),
                mul(496, w(c.T)),
            ]),
            // current states and friction maps (κ_k OPEN)
            sum([mul(32, w(c.k)), w(c.sigma_k), mul(OPEN_KAPPA_K, w(c.k))]),
            // the gap views, at m = min(n, 256)
            gap_views,
        ]),
        "H_loop",
    )
}

/// H_asm_peak, the assembly transients, summed as §3 writes them (they are
/// freed before the first iterate; the sum is conservative). ACCOUNTING rows
/// "Assembly `blocks`" (1168B, NI:2631–2638), "Assembly `formed`"
/// (≤ 1168·max(E+C, 2(E+C+T)), FKS:680–712), "Assembly `connectivity`,
/// `spring_dofs`" (16ε + 8s, FKS:729–734), "Pattern builder"
/// (≤ 56N + 64ε + 33n + 40z + 8, FKS:84–160) and "Evidence magnitudes, scatter
/// counts" (16z, SA:727–728).
///
/// The max in `formed` is redundant: max(E+C, 2(E+C+T)) is always 2(E+C+T),
/// and the accounting gives exactly 1168(E+C) when T = 0. Conservative
/// over-count (DERIVATION §5), kept.
pub fn H_asm_peak(c: &Counts) -> Est {
    let eps = Some(c.eps()?);
    let e_plus_c = sum([w(c.E), w(c.C)]);
    named(
        sum([
            mul(1168, w(c.B)),
            mul(1168, max([e_plus_c, mul(2, sum([w(c.E), w(c.C), w(c.T)]))])),
            sum([mul(16, eps), mul(8, w(c.s))]),
            sum([
                mul(56, w(c.N)),
                mul(64, eps),
                mul(33, w(c.n)),
                mul(40, w(c.z)),
                Some(8),
            ]),
            mul(16, w(c.z)),
        ]),
        "H_asm_peak",
    )
}

/// R_sr, the retained structural report: ≤ 468f₀ + 96z + 8(2c + z) + 340.
/// ACCOUNTING row "R_sr: structural report" (≤ 468f + 96R + 8(t_s + t_d) + 340,
/// FK:1985–2015) with f ≤ f₀, R ≤ z, t_s ≤ c, t_d ≤ c + z.
pub fn R_sr(c: &Counts) -> Est {
    let cc = Some(c.c()?);
    named(
        sum([
            mul(468, w(c.f0)),
            mul(96, w(c.z)),
            mul(8, sum([mul(2, cc), w(c.z)])),
            Some(340),
        ]),
        "R_sr",
    )
}

/// R_pe, the retained product equilibrium: ≤ 272f₀. ACCOUNTING row "R_pe"
/// (≤ 272f, PE:20–27, :70).
pub fn R_pe(c: &Counts) -> Est {
    named(mul(272, w(c.f0)), "R_pe")
}

/// D_as ≈ 774 + 6σ_k + 24k. ACCOUNTING row "D_as" (NS:678–710; DG:151–186).
/// OPEN: the accounting's approximation, not derived.
pub fn D_as(c: &Counts) -> Est {
    named(
        sum([
            Some(OPEN_D_AS_CONSTANT),
            mul(OPEN_D_AS_PER_SIGMA_K, w(c.sigma_k)),
            mul(OPEN_D_AS_PER_K, w(c.k)),
        ]),
        "D_as",
    )
}

/// R_it, retained per iteration, as §3 lists it. ACCOUNTING rows "R_sr", "R_pe",
/// "Displacements, reactions, boundary copies" (16n + 16a, NI:1028–1033),
/// "Active set + diagnostic" (32k + σ_k + 2·24k_c + Σids + D_as, NS:461–528;
/// DG:525–543; written 48k with k_c ≤ k), "D_as", "Applied friction forces"
/// (2·40k_sl + ids, NI:1801–1815; written 80k), "Sparse evidence strings"
/// (≤ 72 + ≤ 150, NI:3151–3154, :3214–3228), "Strict gap evidence" (185, or the
/// n ≤ 256 bound, SA:3386, :3611–3673; OPEN), "Recovery strings (iter 1)"
/// (≤ 2(400 + 2σ_k + 20k + 42a), NI:845–890) and "Iterations Vec" (4·S_iter,
/// NI:1037). Σids, the friction ids and the small-model strict-gap bound are
/// OPEN constants. The strict-gap evidence is priced as max(185, the
/// small-model bound at m = min(n, 256)) for every n (DERIVATION.md draft 2,
/// change 1).
///
/// The iterations Vec's 4·S_iter and the "once" recovery strings sit under
/// R_it, so M_loop counts them I times, and X's completion output I times
/// again: conservative over-count (DERIVATION §5), kept.
pub fn R_it(c: &Counts) -> Est {
    let a = c.a();
    let strict_gap = STRICT_GAP_UNSUPPORTED_REASON.max(strict_gap_small_model_bound(c.m()));
    named(
        sum([
            Some(R_sr(c)?),
            Some(R_pe(c)?),
            mul(16, w(c.n)),
            mul(16, a),
            // the active set: 32k + σ_k + 48k + Σids + D_as
            sum([
                mul(32, w(c.k)),
                w(c.sigma_k),
                mul(48, w(c.k)),
                mul(OPEN_SIGMA_IDS_PER_SIGMA_K, w(c.sigma_k)),
                Some(D_as(c)?),
            ]),
            // friction forces: 80k + ids
            sum([
                mul(80, w(c.k)),
                mul(OPEN_FRICTION_IDS_PER_SIGMA_K, w(c.sigma_k)),
            ]),
            // strings: ≤ 222
            Some(222),
            // strict-gap evidence: max(185, the small-model bound at m) (OPEN)
            Some(strict_gap),
            // recovery strings, once: 2(400 + 2σ_k + 20k + 42(r₀ + k));
            // counted I times (conservative over-count, DERIVATION §5, kept)
            mul(
                2,
                sum([Some(400), mul(2, w(c.sigma_k)), mul(20, w(c.k)), mul(42, a)]),
            ),
            // the iterations Vec: 4·S_iter; counted I times (conservative
            // over-count, DERIVATION §5, kept)
            Some(ITERATIONS_VEC_ELEMENTS * S_ITER),
        ]),
        "R_it",
    )
}

/// X, made once at exit:
/// 16n + 2,566 + H_in + 16s + 8n + 8 + 24z + 8n + 16f₀ + 16(r₀ + k)
/// + [I·R_it + 16n + 32k + σ_k + 2,662] + size_of::<ProductCompletion>()
/// (pinned 544; draft 1's ≈ 552).
/// ACCOUNTING rows "Result displacements, reactions clones" (16n,
/// NI:1072–1073), "Assumptions, limitations" (2,566, NI:1368–1388), "Completion
/// input clone" (H_in, NI:1095), "Completion springs, stiffness"
/// (16s + 8n + 8 + 24z, NI:1096, :1098, :2698), "Completion final force, free,
/// prescribed" (8n + 2·8f + 16a, NI:1083–1092), "Completion output"
/// (I·R_it + 16n + 32k + σ_k + 2.6 KiB, NI:1102) and "Completion box"
/// (NI:377–386; the pinned size replaces ≈ 552).
pub fn X(c: &Counts) -> Est {
    let n = w(c.n);
    named(
        sum([
            mul(16, n),
            Some(2566),
            Some(H_in(c)?),
            mul(16, w(c.s)),
            mul(8, n),
            Some(8),
            mul(24, w(c.z)),
            mul(8, n),
            mul(16, w(c.f0)),
            mul(16, c.a()),
            sum([
                mul2(w(c.I), Some(R_it(c)?)),
                mul(16, n),
                mul(32, w(c.k)),
                w(c.sigma_k),
                Some(2662),
            ]),
            Some(COMPLETION_BOX),
        ]),
        "X",
    )
}

/// The strict-gap evidence's small-model bound at m = min(n, 256). OPEN: the
/// draft value is [`OPEN_STRICT_GAP_SMALL_MODEL`] at every m (no form in m is
/// given). It must stay non-decreasing in m for M_loop to stay monotone.
pub fn strict_gap_small_model_bound(_m: u128) -> u128 {
    OPEN_STRICT_GAP_SMALL_MODEL
}

/// Prep ≤ 56z + 20f₀ + 16n + 339. ACCOUNTING row "Preparation"
/// (≤ 56z_f + 20f + 16n + 8 + 331, FKS:1544–1723).
pub fn T_Prep(c: &Counts) -> Est {
    named(
        sum([
            mul(56, w(c.z)),
            mul(20, w(c.f0)),
            mul(16, w(c.n)),
            Some(339),
        ]),
        "Prep",
    )
}

/// Aud ≤ 344z + 48c + n + 384. ACCOUNTING row "Contribution audit"
/// (FKS:1410–1437, :1770–1870).
pub fn T_Aud(c: &Counts) -> Est {
    named(
        sum([mul(344, w(c.z)), mul(48, Some(c.c()?)), w(c.n), Some(384)]),
        "Aud",
    )
}

/// Ord ≤ 144L + 170f₀ + 16F. ACCOUNTING row "Ordering"
/// (≤ 144L + 170f + 16P, SDS:54–76; SD:506–581, :215–280).
pub fn T_Ord(c: &Counts) -> Est {
    named(
        sum([
            mul(144, Some(c.L()?)),
            mul(170, w(c.f0)),
            mul(16, Some(c.F()?)),
        ]),
        "Ord",
    )
}

/// Red ≤ 24f₀ + 9n + 16(r₀ + k) + 8n. ACCOUNTING rows "Per-solve reduction"
/// (≤ 2·8f + 8f + 8n + n, FKS:1026–1061) and "Per-solve `prescribed`,
/// `prescribed_position`" (16a + 8n, NI:2789; FKS:1105).
pub fn T_Red(c: &Counts) -> Est {
    named(
        sum([
            mul(24, w(c.f0)),
            mul(9, w(c.n)),
            mul(16, c.a()),
            mul(8, w(c.n)),
        ]),
        "Red",
    )
}

/// S ≤ 8F + 16z + 156f₀ + 8 + 41f₀. ACCOUNTING rows "Shared factor S"
/// (≤ 8P + 16z_f + 156f + 8, FKS:2018–2090; FK:2306) and "Factor phase extras"
/// (41f, SDS:82–83; FKS:2007–2014; FK:2305–2307).
pub fn T_S(c: &Counts) -> Est {
    named(
        sum([
            mul(8, Some(c.F()?)),
            mul(16, w(c.z)),
            mul(156, w(c.f0)),
            Some(8),
            mul(41, w(c.f0)),
        ]),
        "S",
    )
}

/// Fin ≤ 8n + 168z + 32c + 548f₀. ACCOUNTING row "Completion Fin"
/// (FK:1894–2030, :1144–1210).
pub fn T_Fin(c: &Counts) -> Est {
    named(
        sum([
            mul(8, w(c.n)),
            mul(168, w(c.z)),
            mul(32, Some(c.c()?)),
            mul(548, w(c.f0)),
        ]),
        "Fin",
    )
}

/// Eq ≤ 8n + 272f₀. ACCOUNTING row "Product equilibrium Eq"
/// (SA:1047–1064; PE:62–70).
pub fn T_Eq(c: &Counts) -> Est {
    named(sum([mul(8, w(c.n)), mul(272, w(c.f0))]), "Eq")
}

/// Obs ≤ 144L + 176f₀ + 16F + 24F. ACCOUNTING row "Observation Obs"
/// (≤ 144L + 176f + 16P₀ + 24P, NI:2827–2833, :2883–2913; SD:726–766, :412).
pub fn T_Obs(c: &Counts) -> Est {
    let f = Some(c.F()?);
    named(
        sum([
            mul(144, Some(c.L()?)),
            mul(176, w(c.f0)),
            mul(16, f),
            mul(24, f),
        ]),
        "Obs",
    )
}

/// V ≤ 48n. ACCOUNTING row "Force vectors V" (NI:1694, :1734, :1817, :1821,
/// :2851).
pub fn T_V(c: &Counts) -> Est {
    named(mul(48, w(c.n)), "V")
}

/// F ≤ 24d̄² + 72d̄. ACCOUNTING row "Coupling matrices F" (24d² + 72d, d ≤ d̄,
/// NI:1731, :1763, :1789).
pub fn T_F(c: &Counts) -> Est {
    let d = w(c.d_bar);
    named(sum([mul(24, mul2(d, d)), mul(72, d)]), "F (coupling)")
}

/// Base ≤ 24n + R_sr + R_pe, only when d̄ ≥ 1. ACCOUNTING row "Held `base`
/// solve" (16n + R_sr + R_pe + 8n, NI:1703–1793).
pub fn T_Base(c: &Counts) -> Est {
    if c.d_bar == 0 {
        return Ok(0);
    }
    named(
        sum([mul(24, w(c.n)), Some(R_sr(c)?), Some(R_pe(c)?)]),
        "Base",
    )
}

/// Gap ≤ 92n + 16f₀ + 16(r₀ + k) + 300. ACCOUNTING row "Gap `terms` etc."
/// (92n + 16f + 16a + 0.3 KiB, SA:3405–3446).
pub fn T_Gap(c: &Counts) -> Est {
    named(
        sum([mul(92, w(c.n)), mul(16, w(c.f0)), mul(16, c.a()), Some(300)]),
        "Gap",
    )
}

/// ASI ≤ 176k + 3σ_k. ACCOUNTING row "ActiveSetIterationInput"
/// (NI:986–993, :3347).
pub fn T_ASI(c: &Counts) -> Est {
    named(sum([mul(176, w(c.k)), mul(3, w(c.sigma_k))]), "ASI")
}

/// Geo ≤ 17N + ρ + 340. ACCOUNTING rows "Geometry screen" (≤ 17N plus
/// rigid-body internals, SA:1941–2040; ρ OPEN) and "Symmetry basis String"
/// (340, SA:2262).
pub fn T_Geo(c: &Counts) -> Est {
    named(sum([mul(17, w(c.N)), Some(OPEN_RHO), Some(340)]), "Geo")
}

/// T, the per-iterate transients:
/// max(Prep + Aud + Ord + Red, S + Prep + Aud + Fin + Red, S + Eq, S + Obs)
/// + V + F + Base + Gap + ASI + Geo.
/// The phase sequence is the design's, with the accounting's §1.4 sequence
/// check; the ordering is freed before the factor's rows (SDS:54–82).
pub fn T(c: &Counts) -> Est {
    let prep = Some(T_Prep(c)?);
    let aud = Some(T_Aud(c)?);
    let ord = Some(T_Ord(c)?);
    let red = Some(T_Red(c)?);
    let s = Some(T_S(c)?);
    let fin = Some(T_Fin(c)?);
    let eq = Some(T_Eq(c)?);
    let obs = Some(T_Obs(c)?);
    named(
        sum([
            max([
                sum([prep, aud, ord, red]),
                sum([s, prep, aud, fin, red]),
                sum([s, eq]),
                sum([s, obs]),
            ]),
            Some(T_V(c)?),
            Some(T_F(c)?),
            Some(T_Base(c)?),
            Some(T_Gap(c)?),
            Some(T_ASI(c)?),
            Some(T_Geo(c)?),
        ]),
        "T",
    )
}

/// M_loop = H_in + H_loop + H_asm_peak + I·R_it + X + T (DERIVATION.md §3).
pub fn M_loop(c: &Counts) -> Est {
    named(
        sum([
            Some(H_in(c)?),
            Some(H_loop(c)?),
            Some(H_asm_peak(c)?),
            mul2(w(c.I), Some(R_it(c)?)),
            Some(X(c)?),
            Some(T(c)?),
        ]),
        "M_loop",
    )
}

/// Every term of the bound, in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Breakdown {
    pub eps: u128,
    pub c: u128,
    pub F: u128,
    pub L: u128,
    pub m: u128,
    pub S_iter: u128,
    pub completion_box: u128,
    pub H_in: u128,
    pub H_loop: u128,
    pub H_asm_peak: u128,
    pub R_sr: u128,
    pub R_pe: u128,
    pub D_as: u128,
    pub R_it: u128,
    pub I_times_R_it: u128,
    pub X: u128,
    pub Prep: u128,
    pub Aud: u128,
    pub Ord: u128,
    pub Red: u128,
    pub S: u128,
    pub Fin: u128,
    pub Eq: u128,
    pub Obs: u128,
    pub V: u128,
    pub F_coupling: u128,
    pub Base: u128,
    pub Gap: u128,
    pub ASI: u128,
    pub Geo: u128,
    pub T: u128,
    pub M_loop: u128,
}

/// Every term of the bound, or the first overflow.
pub fn breakdown(c: &Counts) -> Result<Breakdown, SparseAdmissionError> {
    let r_it = R_it(c)?;
    Ok(Breakdown {
        eps: c.eps()?,
        c: c.c()?,
        F: c.F()?,
        L: c.L()?,
        m: c.m(),
        S_iter: S_ITER,
        completion_box: COMPLETION_BOX,
        H_in: H_in(c)?,
        H_loop: H_loop(c)?,
        H_asm_peak: H_asm_peak(c)?,
        R_sr: R_sr(c)?,
        R_pe: R_pe(c)?,
        D_as: D_as(c)?,
        R_it: r_it,
        I_times_R_it: named(mul2(w(c.I), Some(r_it)), "I*R_it")?,
        X: X(c)?,
        Prep: T_Prep(c)?,
        Aud: T_Aud(c)?,
        Ord: T_Ord(c)?,
        Red: T_Red(c)?,
        S: T_S(c)?,
        Fin: T_Fin(c)?,
        Eq: T_Eq(c)?,
        Obs: T_Obs(c)?,
        V: T_V(c)?,
        F_coupling: T_F(c)?,
        Base: T_Base(c)?,
        Gap: T_Gap(c)?,
        ASI: T_ASI(c)?,
        Geo: T_Geo(c)?,
        T: T(c)?,
        M_loop: M_loop(c)?,
    })
}

#[cfg(test)]
#[path = "sparse_admission_tests.rs"]
mod tests;
