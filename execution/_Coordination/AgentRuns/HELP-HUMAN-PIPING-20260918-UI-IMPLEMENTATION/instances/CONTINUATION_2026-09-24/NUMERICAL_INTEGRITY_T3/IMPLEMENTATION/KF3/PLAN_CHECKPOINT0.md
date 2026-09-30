# KF3 checkpoint 0: diagnosis and plan (I19)

Status: a plan only. No code was written, and nothing was built or run: no cargo, and no Python beyond reading committed records. K6b holds a timed slot.

## 0. Basis

- **Brief:** `T3/TASK_BRIEFS/I19_KF3_IMPLEMENTATION.md` (sha256 `98e57d3d…08509be7`).
- **Rulings** (`T3/ROOT_RULINGS_V1.md`, `612c5fdb…7d5ab158`, read at numerics `382ab0943`):
  - "K6b: slot K6B-S3 stopped at W1-T4's first row …";
  - "V-K: B prepared; ExactSumSpan recorded …";
  - "V-K B accepted; KF3 spawned";
  - "K6b: slot K6B-S4 (b3) accepted …", the latest, which I also read.
- **Also read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md` and `I8R_K1_RESUME.md:24-50`;
  - R7 §5 (`T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md`, `5502aef9…f80d8f42`), items 7 to 15, and Lemmas A to E, the Theorem and the Corollary;
  - K4's `RETURN.md` §22 and KF1's `RETURN.md` §0–2.
- **Records read:**
  - V-K's B, at `3fd1baff3` on the V-K branch: `summary.txt` and `runs/13_…n10000-AX_w1a.jsonl`;
  - K6b's row 247 (`<wt>/scratch/i16/b2/records/247_RF-LARGE-CHAIN-n10000-AX_w1a.jsonl`, `30ab800e…`);
  - GEN's committed `K4T/bounds.txt`, for its U_c tokens.
- **Branch:** `codex/piping-kf3-20260929` in `<wt>/kf3`, at main `0f5d8c7b4`, clean.
- **Source hashes (sha256):**
  - `K4R/bound.rs` `a72db3a2…`, `verify.rs` `a5c9b95d…`, `adaptive.rs` `deae6c39…`, `directed.rs` `8aaace7b…` and `wide_sum.rs` `6b05c81b…`;
  - `K4T/gen_k4_vectors.py` `55c36778…`.
- **Paths:** `FK` = `projects/chirality-piping/core/solver/frame_kernel/`, `K4R` = `FK/src/structural/retained/`, `K4T` = `FK/tests/retained_k4/`. Line numbers are at `0f5d8c7b4`.
- **Delegation:** I19 is a background subagent of ROOT's session, dispatched directly under D-GOV-35. It delegates nothing and makes no Git write or index operation.

## 1. Diagnosis

### 1.1 Where it stops (the records)

The same on all five frames (V-K B; K6b row 247 is identical over 5 repeats):
- the 128 candidate's verification at 256 fails, so 128 is `Rejected(VerificationFailed)`;
- the 256 attempt is `Failed(Stop(Span))`, and the case ends `Unresolved(ExactSumSpan)` (`adaptive.rs:3026-3041`, `terminal`).

The stop is in `build_verify_shared`:
- `bounded_formation` and `wide_formation` are recorded, and `uc` is 0;
- the charged verification-shared total exceeds those two stages by the unrecorded partial `uc` work.

| Frame (10,000 members) | Partial `uc` work charged but not staged (M LME) |
|---|---|
| CHAIN-AX | 14.05 (V-K); 14.72 (K6b's binary) |
| CHAIN-ROT | 29.1 |
| TREE-AX | 75.4 |
| TREE-ROT | 74.5 |
| CONT-ROT | 54.1 |
| CONT-AX (completes) | full `uc` stage: 106.4 |

### 1.2 Which sum, and in which function (the code)

The stop lies between `t2` and `t3` in `verify.rs:471-485`: `gamma_m`, then `uc_bounds`, which runs `u_pass`, then `nl_pass`, then `bounds_from` per block.
- **`gamma_m` cannot refuse.** Its one sum is 2^P − m, which spans at most P + 1 bits.
- **`bounds_from` cannot be reached.** It runs only after both passes complete. Its sums are two-factor products (at most 2P + 2 bits) and 1 − t with t ≥ about 2^-1030.
- **`nl_pass` does not grow.**
  - Each value is a one-level sum over a row or column of |L|: a′_j = 1 + Σ|l_ij|, b′ = d·a′ and c′_i = b′_i + Σ|l_ij|·b′_j.
  - No value is fed back into its own recurrence.
  - GEN's N_L tokens are 2^2 to 2^4 on all six frames, at both 10 and 100 members.
- **`u_pass` compounds**, and this is where the stop is.
  - a = M(L)⁻¹e (a_i = 1 + Σ_{j<i}|l_ij|·a_j), and c = M(L)⁻ᵀD⁻¹a.
  - The exact sum that spans is the one inside `add_toward` (`directed.rs:88-106`): the two addends and the nearest result, one `ExactWideSum` span (`wide_sum.rs:215-218`, `SPAN_LIMIT_BITS` = 8,128).
  - Once the product term grows large enough, its small addend is more than 8,128 bits below it:
    - **forward, `bound.rs:257`:** the constant 1 in a_i = 1 + |l_ij|·a_j. This refuses once |l_ij|·a_j ≥ 2^8127;
    - **backward, `bound.rs:274`:** b_j in c_j = b_j + |l_ij|·c_i. This refuses once log2(|l_ij|·c_i / b_j) > 8,128 − P, which is 7,872 bits at P = 256.
  - `SumRefusal::Span` becomes `AttemptStop::Span` (`adaptive.rs:163-172`), and the whole attempt stops.
  - This is not an exponent overflow: `Wide`'s range is ±2^62 (`wide.rs:135`).

### 1.3 The growth law along a chain

R7 Lemma D's "Why Uc alone is not enough" gives the mechanism:
- M(L)⁻¹ drops every sign cancellation of L⁻¹;
- each node's DOFs carry several multipliers of order 1 to the next;
- so M(L)⁻¹'s entries grow like ρ^(i−j) along the elimination order, and U_c ≈ c_0 ≈ ρ^(2n).

GEN's U_c tokens (`K4T/bounds.txt`, P = 256, one block per frame) fix the rate, **log2 U_c ≈ α·N + β**, with N the member count:

| Frame | log2 U, 10 members | log2 U, 100 members | α (bits per member) | log2 est, 10 → 100 | Predicted log2 U at 1,000 / 10,000 | First N with a backward refusal (P = 256) |
|---|---|---|---|---|---|---|
| CHAIN-ROT | 67 | 661 | 6.60 | 20 → 34 | ≈ 6,600 / 66,000 | ≈ 1,190 |
| CHAIN-AX | 34 | 344 | 3.44 | 14 → 27 | ≈ 3,440 / 34,400 | ≈ 2,290 |
| TREE-ROT | 46 | 344 | 3.31 | 17 → 30 | ≈ 3,320 / 33,100 | ≈ 2,370 |
| CONT-ROT | 25 | 204 | 1.99 | 7 → 13 | ≈ 2,000 / 19,900 | ≈ 3,950 |
| TREE-AX | 24 | 181 | 1.74 | 17 → 30 | ≈ 1,750 / 17,400 | ≈ 4,510 |
| CONT-AX | 5 | 11 | U tracks est (tight) | 5 → 11 | tight | none |

- **The growth law.**
  - U_c grows geometrically in N, while the norm grows polynomially: est gains about 13 bits per decade, about N⁴ for a cantilever.
  - So U_c/‖K̃_c⁻¹‖₁ ≈ 2^(αN).
  - The ROT frames grow faster. The rotated axis mixes axial stiffness into every translation, so the pivots are smaller and the multipliers larger: +3.2 bits per member on the chain, against log2(AL²/12I) ≈ 7.4 for R1's section.
- **Consistency with the records.**
  - At 1,000 members the largest predicted log2 U is 6,600 + P < 8,128. The passes complete, so Uc merely does not exist and B = S. This is V-K's V2: all six are selected.
  - At 10,000 members all five exceed the limit by a factor of 2 to 8, and CONT-AX stays tight. This matches the records exactly.
- **Which call site refuses first.**
  - The backward threshold is reached first as N grows.
  - At 10,000 members the forward sum may already refuse, if log2 max a_i ≳ 8,127 (heuristically about log2 U/2).
  - The partial work (§1.1) cannot settle this across frames, because the ROT frames have about twice the nonzero multipliers per row, and the cost of a sum grows with its span.
  - **B records the refusal's pass and row per frame** (§2.4), which settles it.
- **A smaller model that reproduces it:** RF-LARGE-CHAIN-ROT at about 1,200–1,500 members (extrapolated). That is too large for CI (about 7,500–9,000 DOFs), so CI uses the constructed model of §6.2.

## 2. The change: amendment A2

### 2.1 The rule: draft text for ROOT to record

> **7e (revision 5a.3, amendment A2). A bound that cannot be formed.**
> - If an exact sum or a directed rounding in the formation of Uc_c (7b) is refused, Uc_c is **unavailable** for block c, and it is treated as not existing (+∞). A refusal is either of:
>   - the sum's terms span more than the exact accumulator's limit;
>   - a result lies outside the exponent range.
> - The passes never cross a block (7a), so the refusal leaves every other block's values unchanged.
> - Likewise, a refusal in the formation of S_c (7c) makes S_c unavailable, and that block is not retried.
> - **7d then reads:**
>   - B_c = min over the bounds that exist.
>   - If a block with data has none, then:
>     - if every missing bound was formed and failed its test (t_c ≥ 1; pivots or σ′_c ≤ 0), p is not selected (`uc`), as before;
>     - if a missing bound was refused, the attempt stops with the refusal, as before this amendment.
>   - A block without data needs no bound, and its refusal is only recorded.
> - Every refusal is recorded in the evidence, per block.
> - A budget stop is never a refusal.

"Unavailable" is R7's existing "Uc_c does not exist" (the t_c ≥ 1 branch), reached by a different route. Every consumer already treats a missing Uc_c as +∞: 7c runs the shift, and 7d takes the minimum over what exists.

### 2.2 The code

**`K4R/bound.rs`:**
- **`BoundRefusal { kind: Span | Exponent, pass, row }`**, where `pass` is forward, pivot division, backward, N_L column, N_L row, `bounds_from`, σ, shifted factor or S's formation.
  - A helper maps `AttemptStop::Span` and `Exponent` to a refusal.
  - Every other stop propagates: `Budget`, `Arithmetic` and the rest.
- **`u_pass` and `nl_pass`** take `block_of_row` and a per-block `refused` slot.
  - The first refusal on a row of block c records (kind, pass, row).
  - Every later operation on c's rows is skipped: no work and no values.
  - Other blocks run as today, operation for operation, so their values and work are bit-identical.
  - `guard.check` stays once per row, so budget stops land where they land today.
- **`uc_bounds`** never returns `Err` for a refusal. `BlockBound` gains `refused: Option<BoundRefusal>`; for a refused block `uc` is `None`, and `u`, `n_l` and `t` are recorded as not formed.
- **`shift_schedule`** catches refusals per block, in `shifted_factor`'s context operations (Exponent only), `nl_pass`, and δ, σ′ and S.
  - `ShiftResult` gains `refused`.
  - A refused block leaves `current` and is not retried.
- **`certified` is unchanged.** It already takes the minimum over the `Some` bounds. `certify`, the test entry, mirrors `verify_state`.

**`K4R/verify.rs`:**
- **`build_verify_shared`:** a refusal no longer fails the shared build. Its stage bookkeeping follows §5.
- **`verify_state`:**
  - `sigma_from_estimate` and `shift_needed` refusals make S_c unavailable. Both are derived unreachable, and are caught for uniformity.
  - The certificate loop (`:969-989`) applies 7d above, with a stop taking precedence over `uc` (Q2).
  - The per-block refusals are returned in `VerifySpent` on every path.

**`K4R/adaptive.rs`:**
- `AttemptRecord` gains `bound_refusals: Vec<BlockRefusal>`. `verify_precision` sets it from the shared build's refusals and the pass's, on success and on the stop alike.
- `terminal` is unchanged: `Span` still maps to `ExactSumSpan`.

**The shared accumulator after a refusal.**
- Nothing is written before the span check (`wide_sum.rs:215-218`), and every directed operation begins with `clear`. So `sum` is reusable after a refusal.
- The in-loop refusal (`wide_sum.rs:255-263`) is unreachable under the span check (K4 `RETURN.md` §5 item 3).
- Q7 offers a full reset for defence in depth.

### 2.3 When the attempt still stops

For a block with data and no bound, the attempt stops with the refusal (`Span` → `ExactSumSpan`, or `Exponent` → `ExponentRange`) when either:
- Uc_c was refused, and S_c was not attempted (est_c = 0), failed its pivots three times, or reached σ′_c ≤ 0; or
- S_c was refused, and Uc_c does not exist.

`gamma_m` (global, derived unreachable) and every budget stop still stop the attempt as today.

### 2.4 The evidence

Each `BlockRefusal` records the block, the bound (Uc or S), the kind, and the pass and elimination row of the first refusal.
- It sits on the verification attempt's record whether the attempt is selected, rejected or stopped, so F2a can publish it.
- **The pass and row give the diagnosis at scale.** They show where M(L)⁻¹ crossed 2^8128, per frame.

## 3. Every reader of Uc, S and B, and why honesty is unaffected

**The one fact the guarantee uses.** Lemmas A to C, the Theorem (steps 3, 5 and 7) and the Corollary use B_c only through B_c ≥ ‖K̃_c⁻¹‖₁ for each block with data. B_b uses it as the maximum over those blocks.

**B under A2** is the minimum of a nonempty set of formed bounds:
- each formed Uc_c is ≥ ‖K̃_c⁻¹‖₁ by Lemma D;
- each formed S_c is ≥ ‖K̃_c⁻¹‖₁ by Lemma E;
- so the minimum is too.

**A refusal changes no formed value.**
- A refused block's rows are skipped. Every operation on another block's rows reads only that block's values, because cross-block multipliers are exact zeros, which are already skipped (7a).
- So every formed Uc_c and S_c is bit-identical to the one formed without the refusal, and Lemmas D and E apply unchanged.
- Nothing a lemma needs is taken from a refused block.

| Reader | Uses | Under A2 |
|---|---|---|
| 7c's shift condition | Uc_c missing, or above 2⌈√n_c⌉·est_c | A refused Uc is "missing", the same branch as t_c ≥ 1. Availability only: Lemma E holds for any σ_c. |
| 7d, B_c = `certified` | min over existing | Only formed bounds enter. It is never 0 or a placeholder. |
| θ_c (item 9, Lemma C) | B_c | Needs only B_c ≥ ‖K̃_c⁻¹‖₁. |
| t₁, N_u and t₃ (item 11), and C_q | B_b | The same. The Theorem's steps 3 and 5 are unchanged. |
| W⁺ (item 12), hence (a)'s V_q for translation and rotation rows | B_b through t₁ and t₃ | The same (Theorem step 7). |
| `uc` rejection (item 13) | no bound on a block with data | Unchanged when nothing was refused. With a refusal it stops, as before. |
| Receipt and G5a: B_b finite and positive; item 14's encoding | B_b | θ ≤ 1/2 still gives B_c ≤ 2^(P−6). |
| g check (item 10) | the data flags, not B | Unaffected. |
| E, ê, Φ, the stop rule's M_q, the classification | the state and E | They never read Uc, S or B. |
| est_c | the screen's solves | Unchanged. It feeds only σ_c. |
| Work and budget | charged totals | A refusal's work is charged. Budget stops are never caught, so D1 §4.1.7 is unchanged. |

**Conclusion.** No honesty step depends on A2. It changes only whether a certified bound exists: an availability gain, never a looser bound. This is the argument RETURN will derive in full.

## 4. GEN's side

GEN models 7b to 7d with Fractions (`u_pass_em`, `nl_pass_em`, `uc_from_em`, `shift_schedule_em`, `verify_em`, `schedule_em`) and has no span limit. It will mirror the rule:
- **`add_up(x, y, p)` refuses exactly as Rust does.** It refuses when the span over {x, y, rp(x + y)} exceeds 8,128. Each term's span runs from its lowest set bit to its highest, which is `add_raw`'s measure.
  - Products and quotients cannot refuse at P ≤ 1,024: their spans are at most 2P + 2 bits. This is asserted.
  - The refusal is applied per block, with the same skipping and the same (pass, row).
- **The 7d stop** is mirrored in `verify_em` and `schedule_em`, as `failed:Span`.
- **`bounds.txt` and `charge.txt`** gain a `refused:<kind>:<pass>:<row>` token for a refused block.
  - Every existing line is byte-identical. No control refuses today: `outcomes.txt` has no span stop, and only RF-LARGE-100's three blocks lack a Uc, by t ≥ 1.
  - So `--check` holds on every committed record, and the diffs are purely additive.
- **The constructed models** (§6.2) are added to `models5a3.txt` with their exact rational expectations (`solve_exact`), and to `outcomes.txt`, `bounds.txt`, `charge.txt` and `estimate.txt`.
- GEN does not model work, so §5 has no GEN side.

## 5. Partial stage work on every error path

**Builds that record stages, and their unstaged error paths today:**

| Build | Stages | Errors that leave work unstaged |
|---|---|---|
| `build_shared` (`adaptive.rs:1142-1303`) | formation, assembly, residual_formation, factor, condition | `Pivot`, `ZeroDiagonal` and `NegativeEnergy` in `factor`; `Condition` in the screen; budget; span; exponent |
| `solve_case_at` (`adaptive.rs:1561-1720`) | rhs, solve, refinement, bounded_gate, recovery | `ResidualGate` (the refinement and fallback work); budget; span |
| `build_verify_shared` (`verify.rs:411-519`) | bounded_formation, wide_formation, uc | budget inside `uc`, and today's refusal |
| `verify_state` (`verify.rs:693-1190`) | scale, estimate, charge, bound, shift | All stages are assigned only at the end (`:1150-1154`), so any stop (budget, `ResolutionScale`, span, and A2's stop) stages nothing. |

The stop rule is already charged whole on every path (`adaptive.rs` schedule, `record.stages.stop_rule += decision.total`).

**The fix, in each build:**
1. Assign each segment's stage when the segment ends. `verify_state`'s two-part charge and bound become four assignments at `t3`, `t4`, `t6` and `t7`. The values are the same, so every success path is bit-identical.
2. Keep a `current` marker.
3. After `run()`, on `Err`, add `total − stages.sum()` to `current`.

Then the stages sum to the charged total on every path, by construction. Cached failures (`obtain`, `obtain_verify`) carry the corrected stages.

**Consequences:**
- **K6b's `w1_stages_equal_totals` then holds with equality everywhere.** K6b can remove the `builds_completed` relaxation and assert equality on stopped builds.
- K6b's `a_build_that_stops_partway…` test asserts `shared_stages.uc == 0` on a budget stop inside `uc`. After KF3 that value is the partial `uc` work, so K6b updates the assertion when it merges main.
- **Stage records of failed attempts of existing controls change; totals do not.** An example is SKEW-K1E-28's 128 attempt (`Condition`), which gains its condition stage.
- K4's goldens pin the stages of completed attempts only (`adaptive_tests.rs:427-640`). A lists every failed attempt whose stage record moves.

## 6. Tests

### 6.1 Unit, at bound level (`K4T/bound_tests.rs`; fast)

- **U1 (Uc refused, then B = S).** A synthetic profile (`ScaledProfile::from_rows`):
  - the scaled biharmonic stencil T² or T³. Its Cholesky rows tend to (1 − z)^k, so M(L)⁻¹ grows at about 1.3–1.9 bits per row. Every entry is an exact dyadic;
  - about 2,000–3,500 rows, sized at A so that `u_pass` refuses.
  - Asserts:
    - `uc_bounds` returns `Ok` with block 0's `refused` = Span and the recorded pass and row;
    - `shift_schedule` at σ = λ_min/2 (closed form) forms S_c;
    - `certified(None, Some(S))` = S, and S ≥ the exact norm (GEN).
- **U2 (per-block isolation).** U1's profile plus a second, well-conditioned block. Block 1's `BlockBound` and `ShiftResult` are bit-identical to its standalone run.
- **U3 (budget is never a refusal).** A guard with room ending inside `u_pass` gives `Err(Budget)`.
- **U4 (S refused, then B = Uc).** `shift_schedule` with σ_c = 2^-9000: σ′'s subtraction spans. S_c is refused and not retried, and B = the formed Uc.
- **U5 (neither).** U1 with no shift: `certified` is `None`, and the refusal is recorded.
- **U6 (the accumulator after a refusal).** A refused `add_toward`, then reuse of `sum`: the next results are exact.

### 6.2 The constructed model (W1 level, CI)

- **Design.**
  - A planar chain along (3, 4, 0), with L = 5, so its geometry is rational.
  - Fixed at node 0. The out-of-plane DOFs are fixed at every node. A tip force and moment are applied.
  - Invented slender section with R = AL²/(12I) ≈ 2^80–2^100 (E, G, A, I and J stated as binary64). The mixing term grows as about log2(R)/2 bits per member (§1.3).
  - Target: a refusal in the verification's `u_pass` at under about 200 members (about 600 free DOFs, like RF-LARGE-100's), with κ ≈ R·N⁴ below 2^200.
- **Sizing.** α is measured at A on 10- to 40-member versions (GEN's `u_pass_em`), and then confirmed on the full model.
- **Expected.**
  - Main's code: `Unresolved(ExactSumSpan)`. This is recorded once at A, from a `git archive` build of `0f5d8c7b4`.
  - KF3: selected at 256 (or 128), with the data block's Uc refused and B = S.
  - The refusal evidence also proves the before-state: the passes are identical up to the refusal, and before KF3 any refusal stopped the attempt.
- **Asserted.**
  - The outcome and the attempts are token-equal to GEN.
  - Every published row passes `compare_honest` against GEN's exact solution.
  - G5a passes, and the stage identity holds.
- **Variant W3 (a block without data).** A second, unloaded body of the same chain.
  - Its block is refused, has no data, and needs no bound: no stop.
  - Body 0's rows are bit-identical to the single-body run.
- **Variant W2 (neither).** The same model with S made unavailable by a `#[cfg(test)]` hook (Q5).
  - Asserts `Unresolved(ExactSumSpan)`, with the attempt `Failed(Stop(Span))` and its `bound_refusals`.

### 6.3 The stage identity

A helper asserts, on every attempt:
- `stages.sum() == work + k4_work`;
- `shared_stages.sum() == shared_work + verification_shared_work`.

It is applied to:
- every control in `models.txt` and `models5a3.txt`, whatever their outcome: the `Pivot`, `Condition` and `ResolutionScale` stops included;
- the `ResidualGate` cases of `refinement_stops_after_three_corrections…`;
- W2;
- **a budget sweep.** For N05, SKEW-K1E-28, TWO-SPAN, EHAT-OVERFLOW and the constructed model, the case limit is set at every stage boundary and at the midpoint of every stage (the `uc` stage included, as in K6b's test), and also at 64 even steps. Each run's stop falls inside a different stage of each build.

### 6.4 The suites

K4's full suite, KF1's tests, FK's full suite (one cargo job, `-j 4`, `RUST_TEST_THREADS=2`, `<wt>/kf3-target`, the memory guard running) and `gen_k4_vectors.py --check`.

## 7. Mutants

One clean copy and one target per mutant, under `<wt>/kf3-mut/`, with at most three at once.

| Mutant | Killed by |
|---|---|
| M1: a refused Uc taken as 0 (`certified` returns `Some(0)`) | U1, and 6.2: B_b = 0 fails G5a, and GEN's `charge` tokens |
| M2: B = the maximum | U1, and E-CHARGE on SKEW6-K1E-12 (S < Uc) |
| M3: the stop dropped when neither is available (becomes `uc`) | W2, U5 |
| M4a–d: the partial stage left unrecorded in `build_shared`, `solve_case_at`, `build_verify_shared` and `verify_state` | 6.3's sweep |
| M5: refusal not per block (all blocks refused) | U2 |
| M6: a budget stop taken as a refusal | U3, and the sweep inside `uc` |
| M7: a refused block without data still stops | W3 |
| M8: the pre-KF3 behaviour (any refusal stops) | 6.2 |
| NONE | passes |

## 8. What moves

- **No existing control changes outcome, row, class or bound.**
  - None ends in a span or exponent stop today, and none refuses in 7b or 7c.
  - GEN's diffs are additive.
- **Golden work counts are unchanged.** Success paths are bit-identical, and the charged totals are unchanged on every path.
- **Controls that move:** only the new constructed models (6.2), from a stop to selected (W1, W3) or to the same stop with evidence (W2).
- **At scale (B):** the five RF-LARGE frames at 10,000 members, now expected to be selected or rejected on their merits.
- **Stage records of failed attempts gain their partial stage**, with totals equal (§5).

## 9. Scale evidence for checkpoint B (ROOT's slot)

- **Binary.** A release build from a `git archive` of the candidate, with the driver per Q6.
- **Runs, one process at a time:**
  1. RF-LARGE-CHAIN-n10000-AX through W1. Record the outcome, attempts, the `bound_refusals` (pass, row), S, B, θ, the worst C and W ratios, the stages (partial `uc` and `shift`), charged work, heap peak and time. Check every published row against R1's references (V-K's comparison), and confirm the stage identity (K6b's parity item) with equality.
  2. The other four frames (CHAIN-ROT, TREE-AX, TREE-ROT, CONT-ROT), likewise.
  3. **Regression checks:** CONT-AX at 10,000 members and the six at 1,000 must equal V-K B's records in outcome, rows and charged work.
  4. Optional: CHAIN-ROT at 1,500 members, to confirm §1.3's threshold, if the generator takes N.
- **Size of the slot.**
  - Each 10,000-member call is about 5–11 s today, plus up to three shifted factorizations (each about the factor stage, roughly 0.7 G LME at 256).
  - A rejection that escalates to a 512 verification multiplies this.
  - About 15–25 minutes in all, including builds. Heap stays under 1.1 GB per process, against E_adm of 2.7 GB.

## 10. Risks

- **Outcome at scale is not guaranteed.** S should exist: at 1,000 members S/norm is 2^7.1–2^7.3, and at 10,000 members 2⌈√n⌉ ≈ 2^8.9. θ should pass (B ≈ 2^66 ≪ 2^248). The stop rule, (b) or the charge may still reject 128 and escalate, with more work.
- **The work and time of W1 at 10,000 members rise** (the shifts). K6b's and V-K's re-runs after KF3 carry the figures for ROOT's W1 limits.
- **Constructed-model cost.** Debug CI time, and GEN's Fraction emulation at 256 and 512 on about 600 DOFs. RF-LARGE-100 already costs 1–4 s. If the model needs more than about 60 s, I ask ROOT.
- **Merge order.** V-K's PR #1057 and K6b's D may merge first.
  - Both edit `adaptive.rs` and `verify.rs`: visibility, and V-K's `seeded` hooks.
  - If they merge first, ROOT merges main into KF3, and B can use their drivers from main.
  - `AttemptRecord`'s new field and the stage changes need K6b's parity check and test updated (§5).

## 11. Open questions for ROOT

1. **Neither bound available, after a refusal.**
   - (a) Stop with the refusal (`ExactSumSpan` or `ExponentRange`), as the brief says ("as today").
   - (b) Reject with `uc` and escalate.
   - **Recommend (a).** It keeps every outcome that S cannot rescue identical and keeps the refusal visible. (b) is an availability extension ROOT could rule on later; honesty is unaffected either way.
2. **Precedence between blocks with data.** One block has a refusal stop, another a plain `uc` miss.
   - (a) The stop wins, as today, where the span ended the attempt before any rejection.
   - (b) Block order.
   - **Recommend (a).**
3. **Where the refusal evidence lives.**
   - (a) `AttemptRecord.bound_refusals` on every path.
   - (b) Also in `VerificationSummary`.
   - **Recommend (a) only.** The summary already sits in the attempt record, and one new field keeps K6b's and V-K's merges small.
4. **The constructed model.**
   - (a) The planar slender chain (§6.2).
   - (b) RF-LARGE-CHAIN-ROT at about 1,300 members: R1's family, but about 7,800 DOFs, likely too slow for debug CI and GEN.
   - **Recommend (a).**
5. **W2's hook** (S unavailable on a refusing block).
   - (a) A `#[cfg(test)]` thread-local in `verify.rs` forcing est_c = 0, as K4's `seed` hook does. If V-K merges first, use its `seeded` module instead.
   - (b) Only U5, at bound level.
   - **Recommend (a).** M3 is then killed at W1 level.
6. **The scale driver for B.**
   - (a) V-K's `vk_scale` and K6b's `k6_observe`, built from scratch compositions (`git archive` of the candidate plus those branches' diffs applied with `git apply` under `<scratch>`; no index operations). These reuse V-K's R1 comparison and K6b's parity check.
   - (b) An `#[ignore]` test in `K4T/` that reads a model file and R1's references. This duplicates V-K's comparison.
   - (c) Main's drivers, if V-K and K6b merge first.
   - **Recommend (c) if available, else (a).**
7. **A full accumulator reset after a refusal** (defence in depth for the unreachable in-loop branch). It needs one method in `wide_sum.rs`, which is outside the write set.
   - **Recommend** authorizing it: KF3 is the first code to reuse a shared accumulator after a refusal.
8. **Not proposed, for the record:**
   - **An early exit.** Once a computed a_i·γ_m ≥ 1, t_c ≥ 1 is certain (computed t_c ≥ (a_i/d_i)·γ·d_i). The passes could stop without any refusal and with far less work.
   - **A "sticky" directed addition** that decides x + y upward without an exact sum when y is below x's ulp.
   - Either one moves the U, N_L and t tokens of the RF-LARGE-100 frames, and `uc`'s work. They are outside "nothing else changes". Recommend a follow-up only if the partial `uc` work at scale matters.
9. **est_c** (`BlockRatios::offer`, in `build_shared`) is also availability-only. It could, in principle, refuse and stop the candidate's shared build. There is no evidence at any size (it completes at 10,000 members), and the build also serves the solve.
   - **Recommend** leaving it out of A2.

## What I did not do

- I ran no build, test or scale run.
- I did not measure the constructed model's α, or which call site refuses first at 10,000 members. §1.3 extrapolates from GEN's 10- and 100-member tokens.
- I did not open V-K's or K6b's drivers beyond their records and stage checks.
