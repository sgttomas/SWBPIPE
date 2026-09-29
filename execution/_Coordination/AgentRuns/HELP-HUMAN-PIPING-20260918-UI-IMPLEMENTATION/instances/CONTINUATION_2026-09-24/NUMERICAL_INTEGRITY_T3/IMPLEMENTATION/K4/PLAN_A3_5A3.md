# K4, checkpoint A3-0: the plan for D1 revision 5a.3 (R7)

> **Plan only.** No code, no builds and no Git writes were made for it. Author: I12 (Type 2 TASK, K4 implementer), for ROOT.
>
> - **Basis.** D1 revision 5a.3 as selected: `DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md`, sha256 `5502aef9803f05f42e93c2a05a0de495401e3f94fe92adb19fc2a0c3f80d8f42`. ROOT's ruling is in `ROOT_RULINGS_V1.md`, "D1 revision 5a.3 SELECTED", at numerics `9a93e8516`. R7 §5 (§5.1 to §5.9) is the governing text; §2 to §4 and §6 to §10 are its warrant.
> - **Code basis.** The K4 branch at `ba88db495`: main `59cb20073` merged, worktree clean. Line numbers below are at `ba88db495`. `FK` = `projects/chirality-piping/core/solver/frame_kernel`, and `retained/` = `FK/src/structural/retained/`.
> - **Read for this plan.**
>   - R7 in full; §5, §6.1, §6.2, §7 and §9 line by line.
>   - DS1's `_run_records_r7/emu7.py.txt` (1,627 lines): `solve_at`, `r6_ingredients`, `formation_scale`, `stop_rule`, `schedule`, the passes and the shift.
>   - `run_controls7.stdout.json` (59 controls), `large7_10_100.json` (12 RF-LARGE frames) and the key list of `mutants7.stdout.json`.
>   - K4's `factor.rs`, `adaptive.rs`, `assemble.rs`, `recover.rs`, `ledger.rs` and `wide_sum.rs`, and the brief's mutant and checkpoint lists.
> - **Checked (read-only):** `git diff cef218a10 ba88db495 -- FK/src/structural/retained/factor.rs` is empty. The loop that Lemmas D and E were read against is byte-identical at `ba88db495`.

## 1. Summary

- **New Rust files, all in the write set:**
  - `retained/directed.rs`: directed wide rounding. It rounds to nearest, then moves one ulp only when the nearest value is on the wrong side, decided exactly.
  - `retained/bound.rs`: the free–free blocks, est_c, Uc_c, the shifted factorization, S_c and B_c.
  - `retained/verify.rs`: the bounded operator, E, ê, V and Φ; W's q_W contributions, r, r₂, δ̂, Ŵ, the norms, θ, the g check, t₁ to t₃, C_q and W⁺; and the verification report.
- **Changed Rust files:**
  - `adaptive.rs`: the hybrid gate with the best state, the stop rule's (a) to (d) with reasons, item 6a, the evidence, the schedule wiring and the SEED hook.
  - `assemble.rs`: g, the bounded coefficients, Ā assembly, and a read accessor for the contributions.
  - `factor.rs`: **the factor loop is untouched.** Only `condition` gains a read-only observer for est_c, and L and D gain read accessors (§4).
  - `ledger.rs`, `wide_sum.rs`, `mod.rs`, `combine.rs` (one reason arm) and `tests/s11_site_table.rs`.
- **Lemmas D and E stay tied to `factor()`.** Its loop, pivot test and roundings do not change. The shifted factorization is a separate function that repeats the loop operation for operation, with the pivot test d′ > 0. A loop-parity test and a mutant bind the copy to the loop (§4). Q4 and Q5 flag the two factor.rs edits for ROOT.
- **Oracle.** `GEN` (`tests/retained_k4/gen_k4_vectors.py`) is extended to emulate R7 §5 in K4's operation order, with R7's directed roundings. DS1's emu7 is the cross-check at selection level. It cannot be the bit oracle, because it keeps θ, the norms and C exact and rounds est_c differently (Q1 to Q3).
- **Tests to add:**
  - E-UNIT, E-HEADROOM, E-ESTIMATE, E-CHARGE and E-UC;
  - every SD-G5 vector R7 §6.2 lists; SD-J1's item-6a vectors; SD-L1 re-pinned;
  - ROOT's named controls, with R7's expectations (§6);
  - the controls R7 §7's killable mutants need.
- **Mutants:**
  - R7 §7's killable list;
  - kept for the derivation, recorded without a kill: M17, M21, M24, and M27 at design precision;
  - not behavioural: M14, and M16, which is killed at evidence level;
  - seven new K4 mutants;
  - **a proposed kill for K4-M24** through a 5a.3 ceiling combination (§7.3, Q14).
- **Checkpoints:** A3 in two parts (A3a and A3b), then B, C and D (§8).
- **Questions:** 18, each with a recommendation (§9). None blocks A3a. Q6, Q7, Q8 and Q9 decide A3b details.

## 2. What A1 and A2 implement (5a.2), and what 5a.3 changes

| Area (D1) | At `ba88db495` (A1/A2) | 5a.3 (R7 §5) | Where |
|---|---|---|---|
| Formation (§4.1.2) | `form_member` (`assemble.rs` 152–279): d, normalize, the Gram–Schmidt residual `yc` (179–184, then discarded), ey, ez, 1/L, the D coefficients, B, K_e upper | **Adds g_m** (§4.1.6.2 item 2): the exact yc·yc and y_ref·y_ref and the least k ≥ 0 with 4^k·(yc·yc) ≥ y_ref·y_ref. `MemberOperators` gains `g_exp: u32`. K_e is unchanged. | `assemble.rs` |
| Assembly | `assemble` (476–525): one exact sum per upper entry, rounded once, then mirrored | **Adds Ā** (item 3): both triangles formed separately (R7 item 6 notes they can differ), at P for the verification and at q for the gate's fallback | `assemble.rs` `assemble_bounded` |
| Ordering | `order_free` (RCM, skyline) | **Adds `FreeBlocks`**: the connected components of the free–free pattern (7a), per group | `bound.rs`, `GroupPrep` |
| Factor (§4.1.3) | `factor` (483–581), `pivot_passes`, `negative_pair` | **Unchanged, byte for byte** | — |
| Condition screen | `condition` (647–758): Hager–Higham, rcond ≤ 2^-(p−1) escalates | Same arithmetic, same rcond. **Adds a read-only observer** collecting est_c from the screen's own solves (7c). §5.9's sentence goes in the module text. | `factor.rs` (Q4) |
| RHS and solve | `reduced_rhs`, `RetainedFactor::solve` | Unchanged; δ̂ reuses `solve` | — |
| Gate (§4.1.4 step 3) | `residual_rows` (921–983), refinement in `solve_case_at` (985–1122): coalesced d^c at q, ≤ 3 corrections, early stop when the worst ratio does not decrease, `ResidualGate` | Refinement driven by d^c, **unchanged**. New: when refinement ends without the coalesced pass, d^b over Ā^q for every evaluated state; the best state (exact, earliest on a tie); the bounded test on it | `adaptive.rs` (Q6, Q17) |
| Recovery | `recover` (`recover.rs` 239–455) | Unchanged. Ŵ = \|L_q(δ̂)\| calls it with an empty ledger | `ledger.rs` `RetainedLedger::empty` |
| Verification (§4.1.6.2, §4.1.6.3) | none | **New:** E, ê, V, Φ, W, r, r₂, δ̂, the norms, Uc_c, S_c, B_c, θ, g, C_q and W⁺ | `verify.rs`, `bound.rs` |
| Stop rule (§4.1.6) | `stop_rule` (1190–1275), `scales_at` (1142–1187): \|Δ\| ≤ 2^-64·M, S\* at 2p | (a) adds V_q (force and moment: 2^(8−2p)·ê) and W⁺_q (translation and rotation; plus 2^(1−2p)·\|q_2p\| for a magnitude). S\* is floored by Φ at p = 512. New tests (b) to (d), with reasons. The summary becomes (\|Δ\| + V)/M | `adaptive.rs` |
| Classification (§4.1.6.1) | `classify_rows` (1479–1531): items 1–8 on published values | **Item 6a:** Φ, when the selected p is 512, after item 6's coupling and before item 7 | `adaptive.rs` |
| Schedule | `run_schedule` (1787–1921): a verification becomes the next candidate after a rejection | Same flow. A verification pass runs on each state used as a verification, and the new rejections escalate like `StopRule` | `adaptive.rs` |
| Combinations (F-1) | their own solve; exact prescribed terms `CasePrep.prescribed`; K4-M33 | Unchanged. W's u⁰ uses those exact terms (item 1) | — |
| Budgets and work (§4.1.7) | `StageWork` per stage; shared stages charged in full to every case, and to the invocation once | New stages, charged to the verification attempt. The shift counts against the verification's budget | `adaptive.rs` |
| Evidence (§5 item 1) | attempts, S\*, classes; `not_covered` empty (marked pending) | **Adds** `resolution_scale`, `verification_estimate` and `verification_charge` (θ, B_b); the summary includes V; the attempt records gain a verification summary. `not_covered` stays empty (Q16) | `adaptive.rs` |
| Encodings (Q10) | K4SRC, K4STF, K4LED, K4RST, K4CMB | Unchanged | — |
| Tests | 89; the SD-tagged assertions listed in RETURN §11.2 | R7 §6.2: SD-G1 to G4, G6, I1, I2, J2 and K1 unchanged; SD-G5 extended; SD-L1 re-pinned; SD-J1 extended; the V4-S3 probe (RETURN §21) becomes a control | `tests/retained_k4/` |

## 3. Code: functions, signatures and how each R7 block is formed

Every generic below carries `where Wide<L>: SupportedWidth` (omitted here). Errors are `AttemptStop`. Every exact expansion is K4's `ExactWideSum`. **No binary64 load, force or right-hand-side fold is added. The binary64 arithmetic is limited to R7's pinned formulas: ê (item 6a), Φ and the summaries.**

### 3.1 Directed wide rounding (`directed.rs`, new)

R7 7b and 7c require every bound operation rounded upward, and the three subtractions 1 − t_c, σ_c and σ′_c downward. R7 allows "nearest then one ulp". K4 moves one ulp **only when the nearest value lies on the wrong side**, as emu7's `ru` and `rd` do. That is also allowed, and it is tighter.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Toward { Up, Down }

/// `sum`'s exact value (≥ 0) at ctx's precision P, rounded toward `toward`:
/// to nearest (`ExactWideSum::round`), then one P-bit step in that direction
/// when the nearest lies on the wrong side, decided exactly by adding
/// −nearest into the same expansion and reading its sign. Clears `sum`.
pub(crate) fn round_toward<const L: usize>(
    ctx: &mut WideContext<L>, sum: &mut ExactWideSum, toward: Toward,
) -> Result<Wide<L>, AttemptStop>;

pub(crate) fn add_toward<const L: usize>(ctx, sum, a: &Wide<L>, b: &Wide<L>, toward: Toward) -> Result<Wide<L>, AttemptStop>;
pub(crate) fn sub_toward<const L: usize>(ctx, sum, a: &Wide<L>, b: &Wide<L>, toward: Toward) -> Result<Wide<L>, AttemptStop>; // a ≥ b
pub(crate) fn mul_toward<const L: usize>(ctx, sum, a: &Wide<L>, b: &Wide<L>, toward: Toward) -> Result<Wide<L>, AttemptStop>;
/// b > 0: q = ctx.div(a, b) (nearest), then the exact sign of q·b − a decides
/// whether q is on the wrong side (`add_product`, then one step).
pub(crate) fn div_toward<const L: usize>(ctx, sum, a: &Wide<L>, b: &Wide<L>, toward: Toward) -> Result<Wide<L>, AttemptStop>;

/// The adjacent P-bit value of x > 0: x ± 2^(e−P+1), with 2^(e−P) below an
/// exact power of two (e = x.exponent()). Formed by one exact sum, which is
/// representable, so it is rounded exactly.
fn step<const L: usize>(ctx, sum, x: &Wide<L>, toward: Toward) -> Result<Wide<L>, AttemptStop>;

/// The least binary64 ≥ x ≥ 0 (+∞ beyond the range): `to_binary64` to
/// nearest, then `next_up` when the exact comparison shows it below x.
pub(crate) fn binary64_up<const L: usize>(x: &Wide<L>) -> Result<f64, AttemptStop>;
```

- **Operands.** Every operand has at most P bits, as `add_product` requires. The bound quantities are all formed in the verification's own context.
- **Tests (SD-G5, "the directed roundings").** GEN draws random Fraction operands in both directions at P ∈ {10, 53, 256, 512, 1024}, including exact results, results one bit from a power of two, and powers of two themselves. The Rust results must equal GEN's `ru` and `rd` bit for bit.

### 3.2 Blocks and data flags (`bound.rs`)

```rust
/// Connected components of the free–free structural pattern (`structure.pattern`
/// restricted to free DOFs; the adjacency `order_free` builds): R7 7a.
pub(crate) struct FreeBlocks {
    pub(crate) of: Vec<u32>,               // block of each free position
    pub(crate) positions: Vec<Vec<usize>>, // free positions per block, ascending; blocks numbered by least position
    pub(crate) body: Vec<u32>,             // each block's body (blocks refine bodies)
}
pub(crate) fn free_blocks(source: &PrimitiveSource, structure: &Structure, ordering: &Ordering) -> FreeBlocks;

/// 7a's data flag per block, at the verification's final state:
/// (i) a nonzero ledger term at one of its DOFs (a term, not the net: a DOF
///     whose terms cancel still counts; emu7's `f_terms`);
/// (ii) one of its DOFs is coupled by the pattern to a constrained DOF whose
///      exact prescribed value (Σ c·v) is nonzero;
/// (iii) a nonzero state value at one of its DOFs (seeds included).
pub(crate) fn data_blocks<const L: usize>(
    blocks: &FreeBlocks, ordering: &Ordering, structure: &Structure,
    ledger: &RetainedLedger, prescribed_nonzero: &[bool], u: &[Wide<L>],
) -> Vec<bool>;
```

- `GroupPrep` gains `blocks: FreeBlocks`, computed in `prepare_group`. It is integer data, independent of precision.
- `ledger.rs` gains `has_nonzero_term(dof) -> bool`. `RetainedLedger::from_source` and `combined` record the flag as they add terms.

### 3.3 est_c: the screen's own solves (`factor.rs` observer; `bound.rs`)

```rust
pub(crate) struct BlockRatios<const L: usize> { of: Vec<u32>, est: Vec<Wide<L>> }
impl<const L: usize> BlockRatios<L> {
    pub(crate) fn new(blocks: &FreeBlocks) -> Self;
    /// After one screen solve y = K̃⁻¹x (free-position order), for each block c
    /// with x_c ≠ 0: est_c := max(est_c, fl(fl(Σ_c|y_i|) / fl(Σ_c|x_i|))),
    /// each sum one exact expansion rounded once at P, the quotient at P.
    pub(crate) fn offer(&mut self, ctx: &mut WideContext<L>, sum: &mut ExactWideSum,
                        x: &[Wide<L>], y: &[Wide<L>]) -> Result<(), AttemptStop>;
    pub(crate) fn estimates(self) -> Vec<Wide<L>>;
}
// factor.rs — the only change to `condition`:
pub(crate) fn condition(&self, ctx, sum, structure, k, ordering,
                        observer: Option<&mut BlockRatios<L>>) -> Result<Wide<L>, AttemptStop>;
```

- **Which solves.** All of `condition`'s solves, as emu7 counts them: each x → y solve in the loop (the uniform vector, then the unit vectors), each sign-vector solve z, and the alternating-vector solve. The estimate, its argmax (over free positions, the last index on ties) and rcond are unchanged.
- **Where it runs.** `build_shared` passes `Some` at p ≥ 256 and `None` at 128, because a 128 state is never a verification. `Shared` stores `est_blocks: Vec<Wide<L>>`.
- **Rounding.** Three roundings, not emu7's single rounding of the exact quotient. est_c is availability only (7c, Lemma E). Q3.

### 3.4 Uc_c: the comparison-matrix bound per block (`bound.rs`; R7 7b, Lemma D)

```rust
/// A unit lower triangular L and positive D in elimination order (read-only view).
pub(crate) trait ProfileLdl<const L: usize> {
    fn n(&self) -> usize;
    fn first(&self, i: usize) -> usize;
    fn l(&self, i: usize, j: usize) -> Wide<L>; // j < i
    fn d(&self, i: usize) -> Wide<L>;
}
// Implemented for RetainedFactor<L> (through factor.rs's read accessors, §4) and ShiftedFactor<L>.

/// γ_m = m·2^-P/(1 − m·2^-P) = m/(2^P − m), m = 2n + 2, n = all free DOFs, rounded up.
pub(crate) fn gamma_m<const L: usize>(ctx, sum, free_count: usize) -> Result<Wide<L>, AttemptStop>;

/// c = M(L)⁻ᵀD⁻¹M(L)⁻¹e and c′ = |L|D|Lᵀ|e, every operation up (emu7's u_pass, nl_pass order):
///   a_i = ↑(a_i + ↑(|l_ij|·a_j)) over j = first(i)..i; b_i = ↑(a_i/d_i);
///   c starts at b; for i descending, j in first(i)..i: c_j = ↑(c_j + ↑(|l_ij|·c_i));
///   a′_j = ↑(a′_j + |l_ij|); b′_i = ↑(d_i·a′_i); c′_i = ↑(c′_i + ↑(|l_ij|·b′_j)).
/// Exact-zero l_ij are skipped (adding 0 changes nothing, and nothing crosses a block).
pub(crate) fn comparison_passes<const L: usize, F: ProfileLdl<L>>(
    ctx, sum, guard: &StageGuard, f: &F,
) -> Result<(Vec<Wide<L>>, Vec<Wide<L>>), AttemptStop>;

pub(crate) struct BlockBound<const L: usize> {
    pub(crate) u: Wide<L>,          // U_c = max_{i∈c} c_i
    pub(crate) n_l: Wide<L>,        // N_L,c = max_{i∈c} c′_i
    pub(crate) t: Wide<L>,          // t_c = ↑(↑(U_c·γ_m)·N_L,c)
    pub(crate) uc: Option<Wide<L>>, // t_c < 1: ↑(U_c / ↓(1 − t_c)); otherwise None
}
pub(crate) fn uc_bounds<const L: usize>(ctx, sum, guard, factor: &RetainedFactor<L>,
    blocks: &FreeBlocks, ordering: &Ordering, gamma: &Wide<L>) -> Result<Vec<BlockBound<L>>, AttemptStop>;
```

- **Precision.** Computed once per verification precision, from the shared factor, and cached in `VerifyShared` (§3.12).
- **Order.** The passes run in elimination order. The maxima are taken per block through `ordering.order` and `blocks.of`.

### 3.5 S_c: the shifted factorization (`bound.rs`; R7 7c, Lemma E)

```rust
/// K̃ = S·K_P·S in elimination order, built exactly as `factor()` builds it
/// (`k[index].mul_pow2(scale[a] + scale[b])`, the same `first`).
pub(crate) struct ScaledProfile<const L: usize> { first: Vec<usize>, rows: Vec<Vec<Wide<L>>>, block_of_row: Vec<u32> }
pub(crate) fn scaled_profile<const L: usize>(structure, k: &[Wide<L>], ordering, blocks, scale: &[i64])
    -> Result<ScaledProfile<L>, AttemptStop>;

/// `factor()`'s loop, operation for operation (the mul, sub and div in the same
/// order over the same ranges), on K̃ with d̃_i = fl(K̃_ii − σ_c) (nearest) for
/// every row of a block with σ_c; other blocks unshifted. The pivot test is
/// d′_i > 0 (no cancellation screen). A failing pivot marks its block failed and
/// is replaced by 1, so later rows continue (emu7); a failed block's rows are not
/// used, and by 7a no other block is affected.
pub(crate) struct ShiftedFactor<const L: usize> {
    first: Vec<usize>, rows: Vec<Vec<Wide<L>>>,
    pub(crate) failed: Vec<bool>,                 // per block
    pub(crate) shifted: Vec<Option<Wide<L>>>,     // d̃ per row of a shifted block
}
pub(crate) fn shifted_factor<const L: usize>(ctx, guard: &StageGuard, sum: &ExactWideSum,
    profile: &ScaledProfile<L>, sigma: &[Option<Wide<L>>] /* per block */) -> Result<ShiftedFactor<L>, AttemptStop>;

pub(crate) struct ShiftResult<const L: usize> {
    pub(crate) sigma: Wide<L>, pub(crate) tries: u8,
    pub(crate) n_l: Option<Wide<L>>, pub(crate) delta: Option<Wide<L>>,
    pub(crate) sigma_prime: Option<Wide<L>>, pub(crate) s: Option<Wide<L>>,
}
/// For the blocks in `need` (block, est_c): σ_c = ↓(1/(2·est_c)); up to three
/// factorizations in all (σ, σ/2, σ/4, the halving exact), each over the whole
/// matrix with the still-failing blocks shifted. For a block whose pivots all
/// pass: N′_L,c = max_{i∈c} c′_i of the shifted factor (↑); δ_c = 2^(1−P)·
/// max_{i∈c}|d̃_i| (exact); σ′_c = ↓(σ_c − ↑(↑(γ_m·N′_L,c) + δ_c)); if σ′_c > 0,
/// S_c = ↑(⌈√n_c⌉/σ′_c), with ⌈√n_c⌉ an exact integer. Passed pivots with σ′_c ≤ 0:
/// no S_c, not retried. Returns the results and the number of factorizations.
pub(crate) fn shift_bounds<const L: usize>(ctx, sum, guard, profile: &ScaledProfile<L>,
    blocks: &FreeBlocks, gamma: &Wide<L>, need: &[(usize, Wide<L>)])
    -> Result<(Vec<(usize, ShiftResult<L>)>, u8), AttemptStop>;
```

- **When it runs.** The shift runs only for a block that carries data and has est_c > 0, and whose Uc_c is missing or exceeds 2·⌈√n_c⌉·est_c. That comparison is exact: `add_wide_scaled(est_c, true, 2·⌈√n_c⌉, 0)` against Uc_c. The est_c > 0 guard is emu7's; it cannot bind for a block with data, and is kept as a defence.
- **Per case.** It runs at the verification only, because it depends on the data flags. K4 therefore counts one shifted factorization per verification where emu7 counts two: emu7 bounds every solved state (R7 §6.7). So `large7`'s `shift_factorizations: 2` is **1** in K4.

### 3.6 B_c and B_b (`bound.rs`; R7 7d)

- **Per block with data:** B_c = min(Uc_c, S_c) over those that exist, by exact comparison. If a block with data has neither, p is not selected, with reason `uc` (§3.11).
- **Per body:** B_b = max B_c over its blocks with data. A body without data blocks has no B_b (Q9).
- **Blocks without data** are neither bounded nor tested, and add nothing to any norm (7a, item 9).

### 3.7 g, the bounded operator and Ā (`assemble.rs`; §4.1.6.2 items 1–3)

```rust
// MemberOperators<L> gains:  pub(crate) g_exp: u32,   // g_m = 2^g_exp, from form_member
/// The per-member coefficients Ā needs (kept at q in `Shared` for the gate's fallback).
pub(crate) struct BoundedCoefficients<const L: usize> {
    pub(crate) inv_length: Wide<L>, pub(crate) axial: Wide<L>, pub(crate) torsion: Wide<L>,
    pub(crate) bend_z: Wide<L>, pub(crate) bend_y: Wide<L>, pub(crate) g_exp: u32,
}
/// g·B̄ᵀ(|D|B̄), 12×12, stage-rounded (emu7's `element_bounded`): B̄ has 1 at every
/// axis-component position and |1/L| at the bending rows' translation entries;
/// |D|B̄ is one exact sum per entry, rounded once; B̄ᵀ(|D|B̄) likewise; then the
/// exact ×2^g_exp. All 144 entries are formed (the triangles may differ in the last place).
pub(crate) fn bounded_block<const L: usize>(ctx, sum, m: &BoundedCoefficients<L>) -> Result<[[Wide<L>; 12]; 12], AttemptStop>;
/// Ā per pattern entry (both triangles): one exact sum per entry, rounded once, of the element
/// blocks' (a, b) entries (the lower triangle takes (b, a) through `pattern.transpose`), |k| of
/// global-axis springs (binary64, exact), and |k_ab| of the directional blocks as formed.
pub(crate) fn assemble_bounded<const L: usize>(ctx, sum, guard, source, structure,
    members: &[BoundedCoefficients<L>], directional: &[DirectionalBlock<L>]) -> Result<Vec<Wide<L>>, AttemptStop>;
// Structure gains a read accessor over an entry's contributions; `Contribution` becomes pub(crate).
pub(crate) fn contributions(&self, index: usize) -> &[Contribution];
```

- **g in `form_member`.** yc·yc and y_ref·y_ref are exact expansions: y_ref is binary64 and yc has P bits. k is the least integer ≥ 0 with 4^k·(yc·yc) − y_ref·y_ref ≥ 0 exactly: start from the exponent difference, then correct with exact signs. It is formed at every formation precision (p, q, q_W, and P at the verification). The existing K_e bits do not change. The formation stage's work does, by two 3-term exact expansions and at most a few signs per member (SD-L1 re-pin).
- **Where Ā is formed:**
  - at P, once per verification precision, in `VerifyShared` (§3.12);
  - at q, only when a gate falls back (§3.10), from `Shared.bounded_q: Vec<BoundedCoefficients<R>>` and `Shared.directional_q`, which `build_shared` keeps from its q-formation instead of dropping them.

### 3.8 E, ê, V and Φ (`verify.rs`; §4.1.6.2 item 4, §4.1.6.1 item 6a)

```rust
#[derive(Clone, Copy)] pub(crate) enum StageRounding { Nearest, Up }
/// E_q for every force or moment row of `layout` (None for the others), with the
/// operand w ≥ 0 at every DOF: w = |u| for E (Nearest), or w = s at free DOFs and
/// 0 at constrained ones for ‖ā_q S‖₁ (Up; Q2). Stages as recover(): d̄ (the node's
/// 1-norm per block, the same for the three local axes), ē, Q̄ = |D|·ē, V̄, the end
/// actions ×g; stations |t|·Q̄_j + |t|·Q̄_i + Q̄_i (×g), forces as at end j; springs
/// |k|·w; directional Σ_b|k_ab|·w_b; reactions |f_c| + Σ_j Ā_cj·w_j (f_c the exact
/// ledger net, omitted when `ledger` is None); a support group's magnitude row:
/// one exact sum of its components' contributors' values (Q12). Each stage is one
/// exact expansion of two-factor products, rounded once (nearest, or up).
pub(crate) fn formation_scale<const L: usize>(ctx, sum, guard, source, layout, structure,
    abar: &[Wide<L>], members: &[MemberOperators<L>], directional: &[DirectionalBlock<L>],
    ledger: Option<&RetainedLedger>, w: &[Wide<L>], rounding: StageRounding)
    -> Result<Vec<Option<Wide<L>>>, AttemptStop>;

/// E(body, kind) = max over the body's force (moment) rows, unpublishable rows included,
/// rounded upward once to binary64 (`binary64_up`); [E_fo, E_mo] per body.
pub(crate) fn resolution_scale<const L: usize>(layout, e_rows: &[Option<Wide<L>>], bodies: usize) -> Result<Vec<[f64; 2]>, AttemptStop>;
/// Item 6a, pinned binary64: ê_fo = max(E_fo, fl(E_mo/L_b)), ê_mo = max(E_mo, fl(L_b·E_fo)); L_b = 0 ⇒ ê = E.
pub(crate) fn e_hat(e: [f64; 2], extent: f64) -> [f64; 2];
/// Φ = fl↑(2^-438·ê): r = ê·2^-438 to nearest, next_up(r) when r·2^438 < ê (decided exactly, as `absolute_bound`).
pub(crate) const PHI_SCALE_BITS: u64 = 0x2490_0000_0000_0000;
pub(crate) fn phi_512(e_hat: f64) -> f64;
```

- **The single source (R7 §3.1, S4).** V_q = 2^(8−2p)·ê, the estimate's threshold 2^(6−2p)·ê, the charge's allowance 60·2^-2p·ê and Φ are all formed from the binary64 ê, lifted exactly. In the exact comparisons they enter as `add_binary64` scaled by the exact power of two, never as a rounded product.
- **E's operands.**
  - |u| comes from the verification's state, with prescribed values as rounded at P, as `prescribed_at` wrote them.
  - E is formed from the **verification** state (item 4).
  - A state solved only as a candidate forms no E. E-HEADROOM's tests form E on any state through the same function (Q11).
- **An E that rounds to +∞** (Q8).

### 3.9 The verification estimate and the charge (`verify.rs`; §4.1.6.3 items 1–14)

```rust
/// Precision-shared verification data, cached per precision in `GroupCache` (§3.12).
pub(crate) struct VerifyShared<const L: usize, const W: usize> {
    pub(crate) q_w: u32,                                // min(3p + 64, 1024): 448, 832, 1024
    pub(crate) abar: Vec<Wide<L>>,                      // Ā at P, per pattern entry
    pub(crate) ke_w: Vec<[Wide<W>; 78]>,                // K_e at q_W per member (at P = 1024: the state's own)
    pub(crate) directional_w: Vec<[[Wide<W>; 3]; 3]>,
    pub(crate) gamma: Wide<L>,
    pub(crate) uc: Vec<BlockBound<L>>,
    pub(crate) work: AttemptWork, pub(crate) sum_work: SumWork, pub(crate) stages: StageWork, pub(crate) total: u64,
}
/// Everything items 1–14 compute at one verification state (a case's).
pub(crate) struct VerificationReport<const L: usize> {
    pub(crate) precision: u32, pub(crate) q_w: u32,
    pub(crate) resolution: Vec<[f64; 2]>,            // E_fo, E_mo per body (uncoupled, ↑ binary64)
    pub(crate) e_rows: Vec<Option<Wide<L>>>,         // E_q (force and moment rows)
    pub(crate) w: Vec<Option<Wide<L>>>,              // Ŵ_q (force and moment rows)
    pub(crate) w_plus: Vec<Option<Wide<L>>>,         // W⁺ (free translation/rotation rows; magnitudes)
    pub(crate) charge: Vec<Option<Wide<L>>>,         // C_q (force and moment rows)
    pub(crate) blocks: Vec<BlockReport<L>>,          // data; U, N_L, t, Uc; est_c; shift; B_c; the norms; θ_c
    pub(crate) bodies: Vec<BodyReport<L>>,           // B_b, θ_b, the body norms, t₁, t₂, t₃
    pub(crate) g_violation: Option<u32>,             // first member id with g > 2^(P−16) in scope
    pub(crate) uc_missing: Option<usize>,            // first data block with neither Uc_c nor S_c
    pub(crate) shift_factorizations: u8,
}
pub(crate) fn verify_state<const L: usize, const R: usize, const W: usize>(
    shared: &Shared<L, R>, vshared: &VerifyShared<L, W>, prep: &CasePrep, group: &GroupPrep,
    state: &Solved<L>, guard: StageGuard,
) -> Spent<VerificationReport<L>>;
// Instances: <4, 8, 8> at P = 256 (q = 320, q_W = 448 in Wide<8>);
//            <8, 16, 16> at 512 (576, 832 in Wide<16>); <16, 16, 16> at 1024 (1024, 1024).
```

**The passes, in order** (each checks the guard and is charged to its stage):

1. **q_W formation** (`VerifyShared`): `form_members` and `form_directional` at a `WideContext<W>` of precision q_W. Only K_e and the directional blocks are kept. At P = 1024 the state's own members are used (Q4 of the K4 rulings: no re-formation above 1024).
2. **r (item 1).** For each free row i, one exact expansion:
   - the exact ledger net f_i (`ledger.add_to`);
   - minus, for every contribution of every pattern entry (i, j) (the entry's own list, or its transpose's with (a, b) swapped), k^(q_W)·u⁰_j, where u⁰_j is:
     - the P-bit state widened to W at a free DOF;
     - at a constrained DOF, each exact term c·v of `CasePrep.prescribed` (exact in Wide<W>, ≤ 106 bits);
   - both operands of every product have ≤ q_W bits.
   - Springs contribute −k·u⁰_i (binary64 k lifted), and directional blocks their 3×3 entries. Nothing is summed into an assembled entry, and nothing is reused from the gate.
   - Also formed in the same pass: r̂_i = r_i rounded to nearest at P; ↑(s_i·|r_i|) per block into ‖S·r‖_∞; and the per-block data flags (§3.2).
3. **δ̂ (item 2):** `factor.solve(ctx, r̂)`, one scaled substitution pair, with 0 at constrained DOFs.
4. **Ŵ (item 3):** `recover(ctx, …, &RetainedLedger::empty(), &δ̂_full)`, the recovery at P with the ledger omitted; Ŵ_q = its \|value\| on the force and moment rows.
5. **r₂ (item 5):** per free row, r_i's expansion formed again, minus Σ_j K^c_ij·δ̂_j over the same contributions (free j), as one exact expansion; ↑(s_i·|r₂_i|) per block.
6. **Norms (item 6),** one pass over Ā at P's free rows. Every row or column sum is one exact expansion (Ā_ij·s_i·s_j exact) rounded up. Per block:
   - the 1-norm max_j s_j·Σ_i s_i·Ā_ij and the ∞-norm max_i s_i·Σ_j Ā_ij·s_j, taking ‖SĀS‖_c as the larger;
   - ‖SĀ|u⁰|‖_∞ = max_i s_i·Σ_j Ā_ij·|u⁰_j| over every column, with |u⁰| exact at prescribed DOFs (a combination's |Σc·v| by its exact sign);
   - ‖S⁻¹δ̂‖_∞ = max |δ̂_i|·2^(−scale_i), which is exact.
7. **The s-pass:** ‖ā_q S‖₁ = `formation_scale(…, ledger: None, w = s, StageRounding::Up)` (Q2).
8. **Bounds:** Uc_c from `VerifyShared`, est_c from `Shared.est_blocks`, then the shift for the blocks that need it (§3.5), then B_c and B_b (§3.6).
9. **θ (item 9):** θ_c = ↑(B_c·‖SĀS‖_c)·2^(7−P), the power-of-two scaling exact, for each data block; tested θ_c ≤ 1/2 exactly.
10. **g (item 10):** every member with a nonzero exact prescribed DOF, or with a free DOF in a data block, needs g_exp ≤ P − 16.
11. **t and C (item 11).** Per body b, over its data blocks:
    - N_u = ↑(‖SĀ|u⁰|‖_b + ↑(↑(2·B_b·‖SĀS‖_b)·‖S·r‖_b));
    - t₁ = ↑(B_b·N_u)·2^(7−q_W);
    - t₂ = ↑(70·‖S⁻¹δ̂‖_b)·2^-P;
    - t₃ = ↑(3·B_b·‖S·r₂‖_b).
    - For each force and moment row, **C_q is one exact expansion**, ‖āS‖·t₁ + ‖āS‖·t₂ + ‖āS‖·t₃, rounded up.
12. **W⁺ (item 12):**
    - a free translation or rotation row: W⁺_i = ↑(|δ̂_i| + s_i·t₁ + s_i·t₃), one expansion;
    - a node displacement magnitude: the exact sum of its free components' W⁺ terms and its constrained components' |u_P,c − Σc·v| (exact), rounded up.
13. **Evidence (item 14):** §3.13.

- **Reports are rounded up, never down.** Every stored bound (Uc, S, B, θ, the t's, C and W⁺) is a P-bit Wide rounded upward, so the tests at §3.11 compare exactly against bounds that are never low.
- **Case dependence.** Every item above is per case. `VerifyShared` holds only what depends on the stiffness identity and the precision.

### 3.10 The hybrid gate and the best state (`adaptive.rs`; §4.1.4 step 3)

- **The refinement loop stays as it is.** `solve_case_at` (985–1122) keeps the coalesced test, the correction rule, the early stop (`corrections == 3 || worst >= prior`) and K4's 64-bit approximate `worst` for the stop, as A2 had it. It also keeps each evaluated state's `u_free`, at most four.
- **When the loop ends without the coalesced pass** (formerly `ResidualGate`):
  1. Ā^q is assembled once at q from `Shared.bounded_q` and `directional_q` (`assemble_bounded::<R>`).
  2. For each evaluated state k (0 = the first solve), `bounded_rows` forms the row residuals again, identical to the first time, and per row the exact d_i^b = |f_i| + Σ_j Ā^q_ij·|u_j| and the exact test |r_i|·(2^p − m_i) ≤ 64·m_i·d_i^b, with m_i as today (2·count + 2).
  3. The **worst bounded ratio** of each state is the exact maximum of num/den over its rows. A row with d^b = 0 and r ≠ 0 makes the state ineligible, as in emu7.
  4. **The best state** has the smallest worst ratio, compared exactly, and the earliest wins a tie (Q6). If every row of that state passes the bounded test, it becomes the final state. Otherwise the attempt stops with `ResidualGate`, as today.
- **Pure helper, for SD-G5's unit vector and mutant M16:**

```rust
pub(crate) fn best_gate_state(worst: &[Option<(ExactWideSum, ExactWideSum)>]) -> Result<Option<usize>, AttemptStop>;
```

- **The record.** `AttemptRecord` gains `gate: GateTest`, either `Coalesced` or `Bounded { state: u8, evaluated: u8 }`. `residual_worst` is then fl↑ of the worst ratio under the test that passed.
- **Work.** The fallback's formation and rows are a new stage, `bounded_gate`. Where today's gate passes, K4's work and corrections do not change (Q17).

### 3.11 The acceptance decision (`adaptive.rs`; §4.1.6 (a)–(d), item 6a)

```rust
pub(crate) enum Rejection {
    StopRule { index: usize }, VerificationEstimate { index: usize },
    Uc { block: usize }, Theta { block: usize }, GValidity { member: u32 }, Charge { index: usize },
}
/// Replaces `stop_rule`'s decision; one pure function so SD-G5 can drive it with a synthetic report.
pub(crate) fn decide<const L: usize, const M: usize>(
    layout: &[QuantityMeta], extents: &[f64], candidate: &[Wide<L>], verification: &[Wide<M>],
    report: &VerificationReport<M>, candidate_precision: u32, guard: StageGuard,
) -> StopDecision; // gains `rejection: Option<Rejection>` and the three summaries
```

**The order is emu7's `stop_rule`, so K4's reasons equal R7's tables (Q7):**

1. **(a) on every published quantity:** |q_p − q_2p| + V_q ≤ 2^-64·max(|q_2p|, S\*), exactly.
   - S\* is `scales_at`'s coupled value at 2p. At p = 512 only, the force and moment kinds become max(S\*, Φ), with Φ = `phi_512(ê)` lifted.
   - V_q = ê·2^(8−2p) for force and moment rows.
   - V_q = W⁺_q for non-input-derived translation and rotation rows, plus 2^(1−2p)·|q_2p| for a magnitude.
   - V_q = 0 for input-derived rows.
2. **(b) on every force and moment row:** Ŵ_q ≤ ê·2^(6−2p), exactly. With ê = 0, any Ŵ_q > 0 rejects.
3. **(c):** `uc` (a data block without B), then θ, then g.
4. **(d) on every force and moment row:**
   - C_q ≤ 60·ê·2^-2p at p = 128 and 256;
   - C_q ≤ 2^-86·M_q at p = 512, with M_q the floored max(|q_2p|, S\*) of (a).

- **The first failure** gives the attempt's reason. The new `AttemptReason` variants are `VerificationEstimate { quantity, body, kind }`, `Uc { body }`, `Theta { body }`, `GValidity { member }` and `Charge { quantity, body, kind }`, and the schedule escalates on them as on `StopRule`.
- **When a data block has no B**, W⁺ and C are undefined. (a) then uses W⁺ = 0 and (d) is not reached, as in emu7 (Q7).
- **The classification's item 6a:**

```rust
pub(crate) fn classify_rows(layout, values, extents, floor: Option<&[[f64; 2]]>) -> Publication;
```

- `floor` is Some only when the selected p is 512. It carries Φ_fo and Φ_mo per body from the selected pair's verification. It applies after `coupled_scales` and before `threshold` and `absolute_bound`: the same Φ bits as the stop rule.

### 3.12 Schedule, caching, work and budgets (`adaptive.rs`)

- **`run_schedule`.** After a verification state is solved, and before `compare_states`:
  1. obtain `VerifyShared` at that precision through a new `obtain_verify`, on the pattern of `obtain`: counted in full against the case, and against the invocation when built here;
  2. run `verify_state`;
  3. charge both to the **verification** attempt's record, the budget and the meter.

  A stop there, such as budget or span, is handled like any stop of that attempt. `compare_states` then calls `decide`, and its work stays charged to the candidate, as the stop rule's is today.
- **`GroupCache`** gains `v256: Slot<VerifyShared<4, 8>>`, `v512: Slot<VerifyShared<8, 16>>` and `v1024: Slot<VerifyShared<16, 16>>`. `merged` and `Clone` carry them, so combinations reuse them (F-1).
- **`Shared`** gains `est_blocks`, `bounded_q` and `directional_q`.
- **`StageWork`** gains:
  - `bounded_gate`;
  - `scale`: E and the s-pass;
  - `estimate`: r, δ̂ and Ŵ;
  - `charge`: r₂, the norms, θ, t, C and W⁺;
  - `bound`: data flags and B;
  - `shift`.

  `VerifyShared.stages` records `bounded_formation`, `wide_formation` and `uc`. The est_c sums fall in the shared `condition` stage (Q4).
- **The shift** counts against the verification's budget (D1 §4.1.7, R7 item 15). Its cost on large models is for K6b and V-K to measure.
- **Reports are kept for the selected pair only:** its `VerificationReport` feeds `finish_selected` (E, ê, Φ and the summaries), and a rejected pair's report is reduced to the attempt's summary (§3.13).

### 3.13 Evidence (§5 item 1 and §5.8; the receipt fields F2a maps)

```rust
// RetainedEvidence gains:
pub(crate) resolution_scale: Vec<(u32, u64, u64)>,        // body, E_fo bits, E_mo bits (uncoupled, ↑, finite, +0.0 only when zero)
pub(crate) verification_estimate: Vec<(u32, Kind, f64)>,  // per body and kind: worst Ŵ/V, ↑ (≤ 2^-2); 0 when V = 0 and Ŵ = 0
pub(crate) verification_charge: Vec<(u32, Kind, f64)>,    // worst C/allowance, ↑ (≤ 1)
pub(crate) theta: Vec<(u32, f64)>,                        // per body: largest θ_c over its data blocks, ↑ (≤ 1/2); 0 without
pub(crate) certified_bound: Vec<(u32, u64)>,              // per body with a data block: B_b bits, ↑, finite and positive (Q9)
pub(crate) floor: Option<Vec<(u32, u64, u64)>>,           // Φ_fo, Φ_mo bits when selected at 512
// `stop_rule` now summarizes (|q_p − q_2p| + V_q)/M_q, ↑ (≤ 2^-64).
// AttemptRecord gains `gate: GateTest` and `verification: Option<VerificationSummary>`
// (E bits, the three worst ratios, θ, B_b, the blocks with data, shift factorizations).
```

- **How the summaries are formed.** They use the existing `ExtremeTracker` and `directed_ratio` (Up) on exact numerators and denominators.
- **Unencodable values.**
  - A B_b not below 2^1024 cannot occur at a selected p (R7 item 14, derived). K4 debug-asserts that, and maps it to the E case (Q8).
  - `not_covered` stays empty (Q16).
- **Dead code.** New entry points carry `#[allow(dead_code)]` with their consumer named (F2a API, or V-K API), as K4 does today.

### 3.14 Test hooks

- **The SEED hook** (SEEDED-COMMON, SEEDED-SOFT; R7 §7).
  - It is a `#[cfg(test)]` thread-local list of (global DOF, binary64 power of two).
  - `solve_case_at` applies it after the gate and before recovery, at every precision: `u[g] = ctx.add(u[g], seed)`, as emu7's `rp(u + val)`.
  - It is compiled out of non-test builds. It affects only the test thread that sets it, which is safe under `RUST_TEST_THREADS=2`.
- **Mutant switches** are not built into the code. Each mutant is a source edit in a clean copy at checkpoint C, as K4's are today.

## 4. Lemmas D and E: how they stay tied to `factor.rs`

R7 takes K4's factor loop as read at `cef218a10` (Lemma D step 1, and §9). The ruling says that any change to its pivot or rounding logic reopens the lemmas.

1. **The loop does not change.** `factor()` (483–581), `pivot_passes` (422–437), `negative_pair` (440–480), `solve_scaled` (596–626) and `solve` (628–644) stay **byte-identical**. `git diff cef218a10 ba88db495` on the file is empty today. At each checkpoint the record will include `git diff cef218a10 <head> -- factor.rs`, and it must show changes only in the places listed below.
2. **The only factor.rs edits (flagged for ROOT, Q4):**
   - `condition` gains `observer: Option<&mut BlockRatios<L>>`, which is called after each of its solves. It reads x and y and writes only its own vector. The estimate's arithmetic, its argmax and rcond are unchanged, and the existing D-series and golden tests pin rcond.
   - `RetainedFactor` gains read-only accessors: `fn get` made `pub(crate)` as `entry(i, j)`, `first()`, `order()` and `scale()`. This is visibility only.
   - Neither touches the loop, the pivot test or any rounding of the factor.
3. **Lemma D's use.** The Uc passes (§3.4) read L and D from the verification's own `RetainedFactor` through those accessors. Lemma D's backward-error step uses only the loop's roundings, and the loop is unchanged.
4. **Lemma E's use.** R7 §9 requires "K4 to run the same loop with its pivot test replaced by d′_i > 0". `shifted_factor` (§3.5) repeats `factor()`'s operations exactly:
   - the same `first` and the same scaled rows (`scaled_profile` builds K̃ as `factor()` does);
   - for j in first[i]..i: s = K̃_ij − Σ_k fl(work_k·l_jk) by sequential `ctx.mul` and `ctx.sub`, work_j = s, l_ij = `ctx.div(s, d_j)`;
   - the pivot by sequential `ctx.mul` and `ctx.sub`.
   It omits only the cancellation sum (which feeds the screen, not L or D) and replaces the screen with d′ > 0. **It is a separate function, not a refactor of `factor()`**, because a shared loop would change factor.rs's loop code, which is what the ruling protects (Q5).
5. **The tie is tested.**
   - **Loop parity:** for every control and every precision at which `factor()` succeeds, `shifted_factor` with no block shifted gives L and D bit-identical to `factor()`'s.
   - **Mutant K4-M35** (a changed operation order in the copy) must be killed by it.
   - E-UC and the low-precision stress test Lemma E's result directly (§5.1).
6. **γ_m.** m = 2n + 2 with n the total number of free DOFs, as R7 and emu7 use. That is conservative per block.

## 5. Tests

### 5.1 The E tests (R7 §7)

| Test | Scope | Assertion | Oracle |
|---|---|---|---|
| **E-UNIT** | every control, every precision K4 solves | g per member; Ā (both triangles, per pattern entry); E_q per row; E(body, kind) binary64 bits; ê bits | GEN's emulation of §4.1.6.2 (Q1). Pinned as sha256 of canonical encodings per (control, P), plus explicit values for N05, TWO-SPAN, CHARGE-SLENDER and one directional-spring case |
| **E-HEADROOM** | every control, every P with states at P and 2P, except the states the estimate or the charge rejects (LEVER2, TILT-LEVER) | \|q_P − q_2P\| ≤ 2^8·2^-P·ê on every force and moment row, with ê from the P state (Q11); the observed maximum is recorded (R3: 1.77) | exact comparison in Rust |
| **E-ESTIMATE** | every control at P = 256 and 512 | Ŵ_q agrees with \|R\*(u_P) − q\*\| within a relative 2^-8 on rows where that error exceeds 2^-(P+20)·ê | GEN: q\* from the exact rational solution of the intended model, and R\*(u_P) the exact recovery of K4's P state, whose bits GEN reproduces (Q10) |
| **E-CHARGE** | every control, every verification | C_q, θ_c, W⁺ and every factor: blocks and data flags; U_c, N_L,c, γ_m, t_c, Uc_c; est_c, σ_c, d̃, δ_c, N′_L,c, σ′_c, S_c and the factorization count; B_c, B_b; S; ‖SĀS‖ in both norms; ‖SĀ\|u⁰\|‖_∞; ‖S·r‖_∞; ‖S·r₂‖_∞; ‖S⁻¹δ̂‖_∞; ‖ā_q S‖₁; r̂ and δ̂ bits | GEN's emulation of §4.1.6.3 items 1–12 in K4's order, with R7's directed roundings (digests plus explicit scalars). Also checked against R7 (not bits): the worst C/allowance 1.6e-3 on CHARGE-SLENDER and HH-SLENDER-m40, otherwise ≤ 5.8e-12, and 7.4e-49 on the probe set |
| **E-UC** | every control with ≤ 40 free DOFs at every verification; RF-LARGE at 10 members (exact) and 100 members (a 512-bit norm); V4's F2 family (`_v4_records/t_f2.py.txt`) at P = 128, 256 and 512; the low-precision stress | Uc_c, S_c and B_c ≥ ‖K̃_c⁻¹‖₁ per block | GEN, exact rationals, from K4's own K_P bits: each norm is emitted as an integer pair and compared exactly in Rust (`ExactWideSum::add_product_of`, §9 Q6) |

- **The low-precision stress** (Lemma E; the M27 kill at low precision; Q15).
  - GEN generates symmetric positive definite matrices of order 2 to 8 at P = 10 to 32, with σ on both sides of λ_min, and their exact inverse norms. It includes the cases where ⌈√n⌉/σ falls below the norm: DS1 found 27 in 20,000, and V4 566 in 95,142.
  - Rust runs `scaled_profile`, `shifted_factor` and `shift_bounds` through a test entry that takes a raw profile and a given σ.
  - The assertion: S ≥ the exact norm wherever S exists, Uc ≥ it wherever Uc exists, and B ≥ it.
- **A test-only G5a checker.** It applies R7 §6.3 items 1 to 6, in binary64 as D2 pins them, to every selected control's evidence: the shape, the zero rule, the sanity bound with c = `0x3FF0000000001000`, the lower bound, and the estimate and charge summaries.
  - Every unmutated control must pass it.
  - Its failures count toward the kills R7 records as G5a failures (R7-M2, M5 and M7). Each of those mutants also has a behavioural kill.
  - D2 remains the owner of G5a. The checker only reads K4's evidence.

### 5.2 The SD tags (R7 §6.2)

- **Unchanged:** SD-G1, G2, G3, G4, G6, I1, I2, J2 and K1. The tests run as they are, and any movement is a stop.
- **SD-G5 (new vectors),** each built from synthetic inputs to `decide`, `best_gate_state`, `directed.rs` and `bound.rs`, or from a named model:
  - **(a), force kind:** \|Δ\| = ε·M − V (accepted), and one 2p-ulp above (rejected).
  - **(b):** Ŵ = V/4 and one ulp above.
  - **The gate:** the bounded test at its boundary, on a best state that is not the last. This is also M16's evidence-level kill.
  - **LEVER2's shape:** an estimate residual whose assembled and contribution-level forms differ (the assembled one is 0 at 256).
  - **(d):** C_q equal to its allowance and one ulp above, at p = 256 and 512.
  - **θ and g:** θ_c = 1/2 and one ulp above; g = 2^(P−16) and 2^(P−15).
  - **Translation:** a row at \|Δ\| + W⁺ = ε·M.
  - **Uc:**
    - t just below 1 (Uc exists) and at 1 (`uc`), found by GEN's search at low P;
    - the directed roundings (§3.1);
    - F2-m100 at p = 128, where U alone falls below the exact norm.
  - **θ and g scope:** a body without data (not tested: THETA-ZERO-BODY), and one with a nonzero prescription (tested: G-PRESC-MEMBER).
  - **Blocks:** a zero-state block (not tested: THETA-STUB), and a block coupled only to a nonzero prescription (tested: a new small model, BLOCK-PRESC).
  - **The shift:**
    - not triggered (RF-LARGE-CONT-n00010-AX);
    - the first shift succeeds (RF-LARGE-CHAIN-n00010-AX);
    - fails once and succeeds at σ/2 (a profile GEN finds);
    - all three fail, with Uc alone (HH-FOOL-m40-LOADED);
    - all three fail with `uc` (a low-P profile GEN finds; no model has it);
    - pivots pass but σ′ ≤ 0 (a low-P profile).
  - **K4's order on RF-LARGE-CHAIN-n00100-AX:** Uc does not exist at P = 256, and the shift bound selects 128.
- **SD-J1:** values are unchanged for p ≠ 512 or E = 0. `classification.txt` gains the item-6a vectors: Φ binding and not binding, the fl↑ boundary, and ê < 2^-584. The vector format gains (selected p, E_fo, E_mo).
- **SD-L1:** `golden_work_counts` is re-pinned with the new stages as their own columns. Where today's gate passes, today's stage values must not move except formation, by g (§3.7), and condition, by the est_c sums at p ≥ 256.

### 5.3 GEN's extension (the oracle)

- **Code.** Standard library only, with `sys.dont_write_bytecode` set and nothing written outside `tests/retained_k4/`. About 1,500 more lines, following emu7's structure but K4's rules:
  - K4's layout, including support groups and node magnitudes;
  - K4's est_c rounding;
  - R7's directed roundings for the norms, θ, t, C and W⁺;
  - one shifted factorization per verification.
- **Its own existing parts it builds on:** `emulate_state`, which reproduces K4's states bit for bit (O8), and `rcm_em`.
- **Models.** The new models are ported from DS1's `models*.py`, `lever3.py` and `sweep.py` (R115-SEED3), and from R1's `references.py` (RF-LARGE, through the existing adapter).
- **New vector files:** `verification.txt`, `bounds.txt`, `estimate.txt` and `stress.txt`, all in SHA256SUMS; `--check` must pass.
- **The cross-check.** GEN's selections, attempts and reasons are compared with `run_controls7`'s "R6" column (R7's rule) and `large7_10_100`. Class counts are compared only where the layouts agree, because emu7's layout has no support-group rows. A disagreement is a stop and a report, never a silent re-pin.

## 6. Controls and their expected outcomes

The expectations are R7's §6.1 and §7 tables and `run_controls7`'s "R6" column (R7's rule, in K4's order and indexing). "Honest" is checked against GEN's exact solution.

**ROOT's named controls:**

| Control | Source | Expected | Also asserted |
|---|---|---|---|
| LEVER2-k90-s40, -k100-s40, -k110-s20 | `lever3.py` | **Unresolved (Ceiling):** 128 `Failed(Pivot)`; 256 `Rejected(VerificationEstimate)`; 512 `Rejected(StopRule)` | W/V at the 256 pair ≈ 2,681, 2.7e6, 2.8e9 (emu); the charge there is also above its allowance (recorded, not the reason); the gate's residual at 256 is 0 |
| TILT-LEVER-k90, -k100, -k110 | `models4.py` (V4's `r3_tilt`) | **Unresolved (Ceiling):** 128 `Pivot`; 256 and 512 `StopRule` at the leak site | the 256 pair's charge (≥ 1.0e30 of its allowance) and θ (≥ 6.0e10) recorded |
| SEEDED-COMMON | N05 plus SEED 2^-110 m at node 1 uy | **Unresolved:** `VerificationEstimate` at 128, 256 and 512 (end 1 i, row 2) | the charge does not catch it |
| SEEDED-SOFT | `models5.py`, SEED 2^40 m at node 2 ux | **Unresolved:** `StopRule` at u(node 2, ux) at 128, 256 and 512, through W⁺ | — |
| CHARGE-SLENDER | `models4.py` | **128** | C/allowance 1.6e-3; B equals the exact norm |
| HH-FOOL-m40, -m100, -m40-LOADED, -m100-LOADED | `models5.py` | **128, 128, 128 and 256** (m100-LOADED: 128 rejected by the stop rule at u(0, ux)) | on H's block, est_c/norm = 2^-37.8 at m = 40 and 2^-97.8 at m = 100, and the global est misses by 2^29.5 and 2^89.5; U_c/exact = 1; on the LOADED forms the shift is triggered for H, runs 3 factorizations and fails each time, so B_c = Uc_c; C/allowance ≤ 6.3e-46 |
| HH-SLENDER-m40 | `models5.py` (HH-FOOL-m40 plus CHARGE-SLENDER as a third body) | **128** | the cantilever's own B; C/allowance 1.6e-3. **Not expected to kill R7-M24:** it stays at 128 under M24. It kills M26 and M29 (at 256), and M3, M7 and M18; not M17 |
| THETA-STUB-COUPLED | `models6.py` | **256:** 128 `Rejected(Theta)` (θ = 28 at the 256 verification) | the charge there is 5.6e-12 of its allowance |
| RF-LARGE-{CHAIN, TREE, CONT}-n00010-{AX, ROT} | R1 `references.py`, through K4's adapter | **128** each; honest against R1's references (the 1e-9 predicate); today's classes | 1 shifted factorization at the 256 verification except CONT-AX (0, Uc tight); S/exact 2^3.3 to 2^4.0; Uc/exact up to 2^47.1 (CHAIN-ROT) |
| RF-LARGE-{…}-n00100-{…} (600 or 450 free DOFs) | as above | **128** each, honest, today's classes | the first σ succeeds except on CONT-AX (not run); S/norm 2^5.5 to 2^5.6; Uc missing (t ≥ 1) on CHAIN-AX, CHAIN-ROT and TREE-ROT, and 151.2 and 190.6 bits loose on TREE-AX and CONT-ROT; worst C/allowance ≤ 4.5e-44 |

**Further controls,** needed for R7 §7's killable mutants, with the expected precision:

- **At 512** (128 and 256 rejected by the stop rule):
  - F-2 (\|N\| = \|R\| = 2^-300 `relative_verified`), F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL (combination A = 1, B = 2^-1000, factors (1, 2^-100)), PRESCRIBED-TAIL-FREE, ASSEMBLY-SAT;
  - DEMOTION2 (relative rows 7 → 4, b ≤ 4.7e-6), EXACT-RIGID (b ≈ 1.1e-150 on the force rows), RIGID-UNLOADED (formerly Unresolved), F-3-FREE, F-3-ROT;
  - GS-TRANS-y345, GS-ROT-y345 (emulation-specific, V4-N8: K4 pins GEN's outcome), M7-GS1, M10-ANISO, R115-SEED3, SKEW-K1E-60.
- **At 256:** M10-G; G-PRESC-MEMBER (128 `Rejected(GValidity)`); SKEW-K1E-28 (128 `Condition`); PIVOT (128 `Pivot`); SKEW6-K1E-12; REACTIONS-ONLY (128 rejected by R(0, Uy)).
- **At 128:**
  - LOADONLY-y345, LOADONLY-y001, GS-ROT-y345-LOADED;
  - **all 20 of K4's V4-S3 probe cases** (RETURN §21 becomes a control; the 8 single-mode y_ref (3, 4, 5) cases are refused under the coalesced gate alone, which is M12);
  - THETA-ZERO-BODY, G-FIXED-MEMBER, THETA-STUB, LEDGER-AT-RESTRAINT, TWO-SPAN (one correction), B1-L, N05, N06 and the S8-W cases.
- **K4's other existing models and combinations** (the N-series, SKEW, AXIS, OBLIQUE, DUPLICATE, PRESCRIBED, ZERO-TORSION-345, ALL-ZERO-BODY, DIRECTIONAL-SPAN, SKEW-K1E-300, the CEIL, B1-C and B1-E combinations, and PROBE_D, PROBE_C, BENDING_SOFT, SPRING-CARRIED):
  - where R7 names them: unchanged precision and classes;
  - the rest: GEN's outcome, which must equal today's unless R7's derivation says why it moves. A move is reported, never silently re-pinned.
- **R1 lane:** 120 of 120 unchanged precision (all at 128) and classes, as R7 measured. The not-covered set is unchanged: RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2.

## 7. Mutants (checkpoint C)

### 7.1 R7 §7's list, as R7-Mn (kills expected as R7 records them; reproduced in K4 at C)

| # | Mutant | Expected kill in K4 |
|---|---|---|
| R7-M1 | drop V | F-2, F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL, ASSEMBLY-SAT at 128 with false claims; M10-G and EXACT-RIGID at 128 |
| R7-M2 | ê uncoupled | F-3-FREE, GS-ROT-y345, R115-SEED3, both HH-FOOL LOADED forms and RF-LARGE-TREE-n00100-AX unresolved; G5a-equivalent checks fail on TWO-SPAN and THETA-STUB-COUPLED |
| R7-M3 | Φ at every p | REACTIONS-ONLY 256 → 128; the floored controls at 128; R115 at 256; CHARGE-SLENDER and HH-SLENDER-m40 relative rows 8 → 3 |
| R7-M4 | no Φ | F-2-CEIL, PRESCRIBED-TAIL, the F-3 forms, GS, M7-GS1, M10-ANISO, R115 and EXACT-RIGID unresolved |
| R7-M5 | E without the prescribed \|u\| | PRESCRIBED-TAIL and ASSEMBLY-SAT false at 128; RIGID-UNLOADED and M7-GS1 unresolved |
| R7-M6 | E from Σ\|ledger terms\| (built from the source's loads) | LEDGER-AT-RESTRAINT at 256 |
| R7-M7 | entrywise operator in E and Ā | LOADONLY-y345, GS and M10-ANISO refused by the gate; THETA-STUB-COUPLED at 128 |
| R7-M8 | Φ = 2^-(2p−74)·ê | the F-3 forms, GS, M7-GS1, M10-ANISO, R115 and F-2-CEIL unresolved |
| R7-M9 | floor at 256 | F-2, F-2-SPOS and the floored controls at 256 |
| R7-M10 | g = 1 | M10-G 256 → 128; G-PRESC-MEMBER at 128 |
| R7-M11 | skip (b) | SEEDED-COMMON at 128, claim ratio 23 |
| R7-M12 | coalesced gate only (A2) | LOADONLY-y345, GS, M10-ANISO and the 8 probe cases unresolved |
| R7-M13 | bounded test drives refinement | TWO-SPAN 0 corrections instead of 1 (evidence level, O5) |
| R7-M15 | W's residual reused from the gate | SEEDED-COMMON at 128 (23); SEEDED-SOFT at 128, false |
| R7-M18 | W and the charge at q = 2p + 64 | SKEW-K1E-28, PIVOT, SKEW6-K1E-12, N06, CHARGE-SLENDER, HH-SLENDER-m40, HH-FOOL-m100-LOADED, THETA-STUB and THETA-STUB-COUPLED at 512 |
| R7-M20 | drop θ | THETA-STUB-COUPLED at 128, honest (availability) |
| R7-M22 | no W⁺ | SEEDED-SOFT at 128, claim ratio 954, false |
| R7-M23 | drop the g check | G-PRESC-MEMBER at 128, honest (availability) |
| R7-M25 | θ and g unscoped | THETA-ZERO-BODY, G-FIXED-MEMBER and THETA-STUB at 256 |
| R7-M26 | F·est (F = 2^16) in place of B | CHARGE-SLENDER and HH-SLENDER-m40 at 256 |
| R7-M28 | no shift (Uc_c alone) | RF-LARGE-CHAIN-n00100-AX at 512, TREE-n00100-AX at 256 |
| R7-M29 | R5's form (one global Uc, per-body scope) | THETA-STUB and HH-SLENDER-m40 at 256; RF-LARGE-CHAIN-n00100-AX at 512, TREE-n00100-AX at 256 |

### 7.2 Recorded without a kill

- **Kept for the derivation (ROOT's ruling):**
  - **R7-M17** (drop the charge);
  - **R7-M21** (prescribed values rounded in W's residual: vacuous for every published W1a case);
  - **R7-M24** (est in place of B). No control is expected to kill it, and HH-SLENDER-m40 in particular must not be;
  - **R7-M27 at design precision** (the shift bound without its backward-error term). At low precision it is expected to be killed by the stress (§5.1, Q15), as R7 records.

  Each is run at C, and its outcome (no selection change) is recorded as the expected survivor. It is not a stop.
- **Not behavioural:**
  - **R7-M14** (W's residual over assembled entries): covered by t₁ (derived), kept for ROOT's ruling, and recorded;
  - **R7-M16** (the last state, not the best): killed at evidence level by SD-G5's gate-choice vector.
- **Retired:** R7-M19 (with F).

### 7.3 K4's own

- **K4-M1 to K4-M33** stay as in the brief and RETURN, rerun against the 5a.3 code. NONE goes first.
- **K4-M24** (a combination's escalation tied to its operands). Proposed kill: **CEIL5A3**.
  - The operands are A = LEVER2-k90 with loads P + Q, and B = LEVER2-k90 with loads P − Q, where Q is a unit load on the same body and kind, so that S\* ≫ P.
  - Each operand is expected to be selected: the estimate and the stop rule pass relative to Q's scale.
  - ½A + ½B is LEVER2-k90's own case, so the combination is expected to be `CombinationUnresolved` with the operands unchanged. The mutant would publish it at the operands' 256, which is LEVER2's false claim.
  - GEN confirms this before it is pinned (Q14).
- **New:**
  - **K4-M34:** nearest instead of directed rounding in the Uc passes or the shift. Killed by the directed vectors, E-CHARGE's bits and the t-at-1 boundary.
  - **K4-M35:** a changed operation order in `shifted_factor`. Killed by the loop-parity test (§4).
  - **K4-M36:** data flags without the prescribed-coupling rule. Killed by SD-G5's BLOCK-PRESC vector.
  - **K4-M37:** Φ in the stop rule but not in the classification (item 6a). Killed by SD-J1's item-6a vectors and EXACT-RIGID's b.
  - **K4-M38:** E rounded to nearest, not upward, to binary64. Killed by E-UNIT's E bits.
  - **K4-M39:** V formed from E at P instead of from the published ê (S4). Killed by SD-G5's \|Δ\| = ε·M − V boundary.
  - **K4-M40:** W's residual without the prescribed columns. Intended kill: F-2, EXACT-RIGID and G-PRESC-MEMBER; confirmed at C.
- **Harness.** A clean copy and target per mutant, with one cargo job at a time (ROOT's current host limit), NONE first.

## 8. Checkpoint sequence

All builds use `RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 --offline --locked`, `CARGO_TARGET_DIR=<wt>/k4-target`, **one cargo job at a time at `-j 4` with `RUST_TEST_THREADS=2`**, and no dense matrix above 1,000 members. `<wt>/guard/memguard.log` is checked after each heavy phase: RF-LARGE-100, E-CHARGE on every control, the stress and the mutants. There are no Git writes; ROOT commits.

- **A3-0 (this plan).** ROOT rules on §9.
- **A3a (bounds and the scale; no behaviour change yet).**
  - Code: `directed.rs`, `bound.rs`, g and Ā (`assemble.rs`), `formation_scale`, `resolution_scale`, `e_hat`, `phi_512`, the `condition` observer and the factor accessors. The verification pass is built but not yet wired into `decide`.
  - Tests: the directed vectors; blocks; Uc and the shift on profiles (F2, the stress); loop parity; E-UNIT; E-UC (controls ≤ 40 DOFs, RF-LARGE-10); GEN part 1.
  - Gate: all 89 existing tests pass **unchanged**, except SD-L1's formation and condition columns, re-pinned with their deltas stated; K3a, K3, K-D5 and `exact_sum` are unchanged; the non-test build is warning-free; `factor.rs`'s diff is limited to §4 item 2.
  - Status: the files, their sha256, the results and the debug wall times.
- **A3b (the method).**
  - Code: the hybrid gate, W, the charge, `decide`, item 6a, the evidence, the schedule wiring, the SEED hook and `VerifyShared` caching.
  - Tests:
    - E-CHARGE, E-ESTIMATE and E-HEADROOM;
    - SD-G5, SD-J1 and SD-L1;
    - every control in §6, including the probe set as a control and CEIL5A3;
    - the R1 lane with RF-LARGE at 10 and 100 members;
    - the combinations;
    - RETURN §11.2's S\*-dependent assertions, updated only where R7 §6.2 says they change.
  - Stops: any control or reference outcome that differs from R7 and GEN, or any change to an unchanged SD tag.
  - Status as for A3a, plus the debug times of RF-LARGE-100 and E-CHARGE (Q13).
- **B:** FK's full suite and the CI 39-manifest profile (`--no-fail-fast`) against ROOT's Mac baseline of main; T9, 112 of 112 byte-identical; DEC-025's Mac sweep, as the brief states.
- **C:** the mutation table: NONE first; K4-M1 to K4-M40; the D-series K4 owns; R7's killable list (§7.1); and the survivors of §7.2 recorded as expected. **Any other survivor is a stop.**
- **D:**
  - CHANGE_RECORD and RETURN: the 22 `[pending D1 5a.3]` markers resolved; §9 step 10 restated per R7 §6.6, route 1's constant under the bounded majorant (V4-R8); §11.2 per R7 §6.2; §21 turned into a control.
  - `_run_records/` and SHA256SUMS, with placeholders only.

## 9. R7 against the Rust: inconsistencies and questions, each with a recommendation

- **Checked consistent** (no question needed):
  - q_W: emu7's `min(P + P/2 + 64, 1024)` with P = 2p equals R7's min(3p + 64, 1024). The widths are 448 → `Wide<8>` and 832 or 1024 → `Wide<16>`, since `WideContext<L>` accepts any precision ≤ 64L.
  - K̃_ii ∈ [1, 4): K4's `scale = −⌊e/2⌋` gives it.
  - δ̂ uses `RetainedFactor::solve`, the scaled substitution pair.
  - m_i as today (2·count + 2).
  - The prescribed terms W needs are those K4-M33 already carries.
  - `factor.rs` is unchanged since `cef218a10`.
- **Notation, not substance.** Lemma D writes work_j = fl(K̃_ij − Σ_k fl(work_k·l_jk)). K4 subtracts term by term, rounding each subtraction. V4 checked the lemma against the loop as coded, and m = 2n + 2 covers it. No action.

**Q1. The bit oracle.**
- Recommendation: GEN (K4's generator, extended) is the oracle for E-UNIT, E-CHARGE, E-UC and SD-G5, as R7 itself says ("the generator's emulation"). emu7 is the selection-level cross-check.
- Why: emu7 keeps the norms, θ, t and C as exact Fractions, rounds est_c once from exact sums, and bounds every solved state. None of its bits can equal an R7-conformant K4.

**Q2. ‖ā_q S‖₁'s rounding.**
- R7 item 6 says every norm is rounded upward. emu7 evaluates ‖ā_q S‖₁ through E's nearest stages.
- Recommendation: upward at every stage. The data are nonnegative and monotone, so this is a certified upper bound. E itself stays nearest, per §4.1.6.2 item 4.

**Q3. est_c's form.**
- K4 rounds each block sum once and then divides, three roundings. emu7 rounds the exact quotient once.
- "The screen's own solves" is taken as every solve in `condition`: x → y, the sign vectors and the alternating vector, as emu7 counts them.
- Recommendation: K4's form, mirrored by GEN. est_c only chooses σ_c, so this is availability only (Lemma E holds for any σ_c > 0).

**Q4. The `condition` observer and the read accessors in `factor.rs`.**
- These are the only factor.rs edits. Neither touches the loop, the pivot test or any rounding (§4).
- Recommendation: accept them as outside the ruling's "pivot or rounding logic".
- The alternative is to duplicate Hager–Higham in `bound.rs`, which costs up to 11 more solves per verification.
- The est_c sums are then charged to the shared condition stage at p ≥ 256, not to the verification attempt as R7's work list says. SD-L1 pins them.

**Q5. The shifted factorization as a separate copy of the loop.**
- It is tied to the loop by the parity test and K4-M35, and it factors the whole matrix per try, as R7 says. A failed pivot is replaced by 1 so that later rows continue, as emu7 does.
- Recommendation: accept this, rather than refactoring `factor()` into a shared loop with a pivot-rule parameter, which would change the protected code.

**Q6. "Compared exactly" for the best state.**
- Comparing two states' worst ratios needs products of two exact expansions, which K4 does not have.
- Recommendation: add `ExactWideSum::add_product_of(&mut self, a: &ExactWideSum, b: &mut ExactWideSum, negate)`.
  - It uses integers only and stack buffers, with no allocation per operation, and is built from `add_scaled` over b's netted limbs.
  - Directed binary64 ratios prefilter it, so the exact products run only on near-ties.
  - A product beyond the span limit stops the attempt with `Span` (terminal, O4).
  - E-UC's exact comparisons in tests use the same helper.
- Alternative (availability only, the M16 class): compare fl↑ binary64 worst ratios, the earliest winning a tie.

**Q7. The order of rejection reasons, and W⁺ when a B is missing.**
- Recommendation: emu7's order, (a) then (b) then `uc`, θ, g, then (d), so that K4's reasons equal R7's tables.
- When a data block has no B, (a) uses W⁺ = 0 and the attempt is then rejected with `uc`, as in emu7. This affects the reason only, never acceptance.

**Q8. An E that rounds to +∞.**
- R7 makes the case `unavailable (receipt_encoding)` at the receipt.
- Recommendation: K4 ends the case as terminal `UnresolvedReason::ResolutionScaleUnencodable { body, kind }` at the first such verification (escalation cannot shrink E), and F2a maps that to `receipt_encoding`.
- `combine.rs` gains the arm. A B_b ≥ 2^1024 maps the same way; it cannot occur at a selected p.

**Q9. B_b for a body with no data block.**
- R7 §5.8 takes the maximum over its data blocks, an empty set here.
- Recommendation: publish no `certified_bound` entry for such a body, and θ = 0. G5a's check 6 then applies to the entries present. **F2a and D2 should confirm.**

**Q10. E-ESTIMATE's reference.**
- Recommendation: GEN's exact rational q\* and R\*(u_P), in place of emu7's P = 2048 pipeline. K4 has no 2048-bit context, and the exact form is stronger.

**Q11. E-HEADROOM's ê.**
- Recommendation: ê from the P state's own E, the resolution bound of state P. At a verification this is the same E the stop rule uses.

**Q12. The support-group magnitude's E.**
- emu7's models have no support groups, so R7's "the sum of their components' E_q" is unmeasured.
- Recommendation: one exact sum over every contributor of the group's three components (reactions, spring actions and directional components), rounded once. Components are not published rows, so no intermediate rounding is needed. The s-pass does the same.

**Q13. RF-LARGE-100 in the debug suite.**
- Recommendation: measure at A3b.
  - If a model takes ≤ 60 s in debug, all six run in the default suite.
  - Otherwise CHAIN-AX and TREE-AX (the R7-M28 and M29 kills) stay in it, and the other four run in an `--ignored` lane that B and C run explicitly.

**Q14. K4-M24's kill through CEIL5A3** (§7.3).
- Recommendation: build it at A3b and pin it only after GEN confirms both operands are selected and the combination is Unresolved.
- If an operand is not selected, report, and K4-M24 stays open.

**Q15. The low-precision stress in Rust (P = 10 to 32, through `ScaledProfile`).**
- Recommendation: include it. It makes R7-M27 killable at low precision, as R7 records, while M27 at design precision is recorded as kept for the derivation.
- GEN supplies the matrices, including the M27 killers, with their exact norms.

**Q16. `not_covered`.**
- Recommendation: stays empty in K4. D1's not-covered classes are F2a's stress, nonlinear and curved rows, and K4's kernel rows are all covered kinds.
- The pending marker is removed at D.

**Q17. The gate's d^b.**
- Recommendation: form d^b only when refinement ends without the coalesced pass. The evaluated states' rows are formed again then, and Ā^q is assembled then from the per-member coefficients kept at q.
- The chosen state and the outcome are those of R7's "every state carries d^b". Today's gate work and correction counts do not change where it passes (R7 §6.2 SD-L1).

**Q18. The verification pass's placement.**
- It runs only on a state used as a verification. A 256 state solved as a candidate after 128 escalates forms no E, W or charge, because E serves the verification (§4.1.6.2 item 6).
- Its report is charged to the verification attempt; the decision (a) to (d) is charged to the candidate, like today's stop rule.
- Recommendation: as stated. Confirm.

## 10. Costs and risks (stated, not measured)

- **Size.**
  - New Rust: about 1,600 lines (`directed.rs` ≈ 150, `bound.rs` ≈ 500, `verify.rs` ≈ 950), plus about 450 in `adaptive.rs`, 150 in `assemble.rs` and 60 across `factor.rs`, `ledger.rs` and `wide_sum.rs`.
  - Tests: about 2,000 lines. GEN: about 1,500 lines.
- **Runtime.**
  - Per verification: one q_W formation, two contribution passes, one Ā pass, one s-pass, one substitution pair and one recovery for W, the Uc passes, and up to three shifted factorizations where triggered.
  - A factorization dominates on banded models, and the shift can at most quadruple the verification's factorization work where it runs.
  - Debug test time grows mainly with E-CHARGE over every control and RF-LARGE-100 (Q13).
- **Memory.** `ke_w` at 1,000 members and W = 16 is about 11 MB. Ā and the profile are proportional to K's. Nothing is dense.
- **Oracle risk.** GEN and K4 are written by the same author. The independent review's oracles (the brief's gate) and emu7's selection-level agreement are the independent checks.
