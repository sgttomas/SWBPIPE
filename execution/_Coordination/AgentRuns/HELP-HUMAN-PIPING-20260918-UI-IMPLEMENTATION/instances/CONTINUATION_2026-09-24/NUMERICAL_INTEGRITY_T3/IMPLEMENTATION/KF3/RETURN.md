# I19 return: slice KF3 (W1a at scale: an unformed certified bound is unavailable, not a stop; partial stage work recorded)

## 0. At a glance

**Status: complete, no stop.** Checkpoints 0, A, B and D are done. ROOT's two findings at B are derived (§12, §13). KF3-B2 is not KF3-induced, so D ends without a fix.

**What changed:**
- **D1 revision 5a.3, amendment A2, as ROOT ruled it.**
  - A Uc_c or S_c whose formation is refused (an exact sum spanning more than 8,128 bits, or an exponent overflow) is unavailable for its block. It is treated as R7's "does not exist" (+∞), and B_c is the minimum over the bounds formed.
  - A block with data left with no bound after a refusal stops the attempt with that refusal, as before. That stop takes precedence over another block's `uc`.
  - Each refusal is recorded per block on `AttemptRecord.bound_refusals`, now public.
- **Partial stage work.** Every build that records stages (`build_shared`, `solve_case_at`, `build_verify_shared`, `verify_state`) adds the work it was charged beyond its recorded stages to the stage in progress when it stops. The stages then sum to the charged total on every path.
- **K6b's parity check** is tightened back to equality everywhere. KF3 merges after K6b, so KF3 makes that change.

**Honesty:** no step of the guarantee depends on A2 (§4). A2 is availability only.

**What moved:**
- No existing control changes outcome, row, class, bound or golden work count.
- The controls that moved are the two constructed ones (`kf3.txt`) and the five RF-LARGE frames at 10,000 members that stopped with the refusal before.
- **The frames at scale** (§11):
  - CHAIN-AX, CHAIN-ROT and CONT-ROT are now selected at 128, and every published row passes R1.
  - TREE-AX and TREE-ROT now end `Unresolved(Ceiling)`, rejected by R7's verification estimate (b), and publish nothing.
  - CONT-AX is unchanged.

**ROOT's findings at B:**
- **KF3-B1** (§12): (b)'s ratio at the root's end force does not depend on the precision, so no rung of the ladder passes it.
  - Near the threshold it is slack in (b): at 4,000 members the row needs a coefficient of 64.8 against (b)'s 64, and the Corollary still holds with its stated margin.
  - From about 6,000 members up, the row needs more than the 127 that R7's split of λ = 2^8 leaves for (b). That is a design limit of the current split, like THIN.
  - The 10,000-member value is not in the records.
  - The runner's `fail` of 194 on each TREE frame counts rows that were never published. No wrong value was published.
- **KF3-B2** (§13): B compared the heap with VR's port of an earlier K6b estimate.
  - K6b's final formula, on `vk_scale`'s own fixed term, bounds both peaks by about 7 MB.
  - Still, K6b's final pass term leaves out `nl_pass`'s three vectors during the shift. At 1024 that is 10.8 MB net.
  - Pre-KF3 code holds the same allocations on that path. So it is a K6b omission that no earlier run reached, not KF3's.

**Checks:**
- FK's full suite, VR's suite, H's suite and H's runner suite pass.
- `gen_k4_vectors.py --check` passes.
- All 11 mutants are killed, and the NONE control passes.

## 1. Brief, basis, delegation and paths

- **Brief:** `T3/TASK_BRIEFS/I19_KF3_IMPLEMENTATION.md` (`98e57d3d…`).
- **Rulings** (`T3/ROOT_RULINGS_V1.md`):
  - "K6b: slot K6B-S3 stopped …";
  - "V-K: B prepared …";
  - "V-K B accepted; KF3 spawned";
  - "KF3: rulings on I19's diagnosis and plan; D1 revision 5a.3 amendment A2" (decisions 1–9);
  - "Main merged into K6b and KF3";
  - "KF3: main merged; K6b's parity restored in KF3; B's slot granted" (numerics `73542a000`);
  - "KF3: checkpoint B accepted; two findings for D" (numerics `32eba6f99`), with ROOT's in-session instruction for D. That instruction asked for KF3-B1 from the existing records, with no new heavy run and a light probe allowed; KF3-B2 with file:line; H's `counts.rs` and `counts.jsonl` not edited; and the stale doc listed as a note.
- **Plan:** `PLAN_CHECKPOINT0.md` (`721fc2d6…`, committed as `75a1222a6`). Checkpoint A is recorded in `CHECKPOINT_A.md`.
- **Branch:** `codex/piping-kf3-20260929` in `<wt>/kf3`, from main `0f5d8c7b4` (K4 and KF1).
- **Commits (made by ROOT):**
  - `75a1222a6`: the plan;
  - `29c0b69e4` and `a7ec4981a`: checkpoint A;
  - `c0473301e`: the merge of main `f8400d290` (V-K);
  - `e114b23c1`: the merge of main `78f55f927` (K6b);
  - `ae831ca51`: B, with H's parity update (`staged.rs`, `k6b_w1.rs`) and B's records.
  - D's candidate is `ae831ca51` plus the D records, which change no code: this file, `CHANGE_RECORD.md`, `_run_records/{d,h,merge}/`, `_run_records/toolchain.txt` and `SHA256SUMS`.
- **Delegation:**
  - ROOT (HELP_HUMAN, the SWBPIPE session) dispatched I19 directly as a Type 2 TASK, through the host's native background-subagent mechanism (D-GOV-35), with the brief as the assignment.
  - Rulings and later instructions came as in-session messages. I19 returned by in-session reports.
  - I19 delegated nothing, and made no Git write or index operation.
  - **The merge conflict:** ROOT started the merge. I19 resolved it by editing files only, and ROOT staged and committed it.
  - **Both merges of main** (V-K `f8400d290`, K6b `78f55f927`) and **the H update** were ROOT's rulings (§9, §3.4). ROOT made every commit.
- **Scope:**
  - the brief's write set: `K4R/{bound,verify,adaptive}.rs`, `K4T/` and `T3/IMPLEMENTATION/KF3/`;
  - widened by ROOT's rulings to `K4R/wide_sum.rs` (one method), `FK/src/structural.rs`'s facade lines, `VR/observations/**` (for the merge; no change was needed), `performance_harness/src/k6/w1/staged.rs` and `performance_harness/tests/k6b_w1.rs`.
- **Host:** the host enforced only its session permissions. I kept to the write set and to the Mac host rules myself:
  - one cargo job at a time, `-j 4`, `RUST_TEST_THREADS=2`;
  - target `<wt>/kf3-target`;
  - the memory guard running throughout, with no kill logged.
  - `_run_records/toolchain.txt` records rustc 1.97.1, rustfmt 1.9.0 stable and Python 3.13, on an Apple M5 Max with 128 GiB.
- **Paths:** `FK` = `projects/chirality-piping/core/solver/frame_kernel/`, `K4R` = `FK/src/structural/retained/`, `K4T` = `FK/tests/retained_k4/`, `VR` = `projects/chirality-piping/validation/benchmarks/numerical_robustness/`, and `H` = `projects/chirality-piping/core/solver/performance_harness/`.

## 2. The diagnosis (plan §1; confirmed at B)

**Where it stops.** The five frames stopped in `build_verify_shared`, in `uc_bounds`, in `u_pass`.
- The exact sum inside `add_toward` refuses when one addend lies more than 8,128 bits below the other. The small addend is the constant 1 in a_i = 1 + |l|·a_j (forward), or b_j in c_j = b_j + |l|·c_i (backward).
- `gamma_m`, `nl_pass` and `bounds_from` are not involved.
- It is not an exponent overflow.

**The growth law.** log2 U_c ≈ α·N + β.
- α is 1.74–6.6 bits per member on the five frames (GEN's tokens at 10 and 100 members).
- The true norm grows only polynomially.

**Confirmed at B.** On all five frames, at every verification, the refusal is in the forward pass. At 256 the rows fall where the plan predicted them (§11):

| Frame | Refusal row, as a fraction of the elimination rows | Predicted |
|---|---|---|
| CHAIN-AX | 47% | 46% |
| CHAIN-ROT | 25% | 25% |
| TREE-AX | 91% | 93% |
| TREE-ROT | 46% | 49% |
| CONT-ROT | 82% | 82% |

## 3. The change

### 3.1 Amendment A2 (`K4R/bound.rs`, `verify.rs`, `adaptive.rs`)

**The evidence types:**
- `RefusalKind` (Span, Exponent);
- `BoundPass` (the pass of the first refusal);
- `BoundRefusal` (kind, pass, elimination row);
- `CertifiedBound` (Uc or S);
- `BlockRefusal` (block, bound, refusal).

All five are public and exported through `retained_api` (ROOT's merge ruling).

**`refusable`** records a block's first refusal and resets the accumulator in full (`ExactWideSum::reset`, ROOT's decision 7). Every other stop propagates, budget included.

**`u_pass` and `nl_pass`:**
- they take each row's block and a refusal slot per block;
- a refusal marks only its own block, and every later operation on that block's rows is skipped;
- every other block runs operation for operation as before.

**`uc_bounds`** never fails on a refusal. `BlockBound.refused` records it, and `uc` is `None`.

**The shift (7c):**
- `shifted_factor` records exponent refusals per block;
- `shift_schedule` carries the factorization's and N′_L's refusals, and those of δ, σ′ and S, into `ShiftResult.refused`;
- a refused block is not retried.

**The helpers,** shared by `verify_state` and the test entry `certify`:
- `shift_start` (7c's start; refusals of the comparison and of σ make S_c unavailable);
- `shift_run` (the profile and the schedule);
- `certificates` (7d with A2);
- `block_refusals` (the evidence).

**7d with A2:**
- B_c is the minimum over the bounds formed;
- a block with data and no bound, after a refusal, stops the attempt with that refusal's stop (`Span` or `Exponent`, as before A2), before any `uc` (ROOT's decisions 1 and 2);
- without a refusal, `uc` applies as before;
- a block without data needs no bound, and its refusal is only recorded.

**Where the refusals are recorded.** `verify_precision` fills `AttemptRecord.bound_refusals` on every path, from the shared build's Uc refusals and the pass's S refusals.

**A test-only hook.** `verify::hooks::set_no_shift` (`#[cfg(test)]`) reads every est_c as 0, for the "neither bound" control. V-K's `seeded` module was not on main when this was written (decision 5).

**Unchanged:**
- Every success path runs the same operations in the same order.
- The S11 site table is unchanged: no new accumulation shape outside `#[cfg(test)]`.
- No early exit (decision 8), and est_c is outside A2 (decision 9).

### 3.2 Partial stage work (`adaptive.rs`, `verify.rs`)

**The mechanism:**
- `StageWork` gains `Stage`, `total`, `add_to` and `close_stopped`.
- Each build keeps the stage in progress. After `run()`, on `Err`, it adds `total − stages.total()` to that stage.
- `verify_state` now assigns each segment at its boundary, with the same values. Before, it assigned them all at the end, so a stop recorded none of them.
- `solve_case_at` records the refinement before the fallback, then the fallback, as before on success.

**The identity.** For every attempt:
- own stages = work + K4 sums;
- shared stages = shared + verification-shared work.

This holds on every path, and cached failures carry their corrected stages.

### 3.3 GEN (`K4T/gen_k4_vectors.py`)

**The span mirror:**
- `sum_span` gives the span from the lowest set bit to the highest over a directed sum's terms and its nearest result;
- `add_dir` refuses as `add_toward` does;
- `mul_up` asserts that products never refuse.

**Updated for A2:**
- the passes (`u_pass_em` and `nl_pass_em` with blocks);
- `uc_from_em_a2`;
- `shift_schedule_em(a2=True)`;
- 7d in `verify_em` (`failed:Span`);
- `bounds_record`'s refusal token, `refused:<kind>:<pass>:<row>`.

**New:**
- `hp_only` expectations, from the two high-precision solves, for a model too large for the dense exact solve;
- `kf3_models` and `kf3_lines`, which write `kf3.txt`.

Every earlier record is byte-identical, and `--check` passes on every file: 17:48 of wall time, against 15:11 before.

### 3.4 H (K6b's checks; ROOT's ruling, KF3 merging second)

- **`staged.rs`:**
  - `stages_equal_totals` requires equality on every attempt, completed or stopped. Its doc cites KF3 and the ruling.
  - `unstaged` and `stages_complete` stay, because the attempt line records them (RV22-1). They are always (0, 0) and true now.
  - `builds_completed` is evidence only.
- **`k6b_w1.rs`:**
  - The budget-stopped build asserts the partial `uc` work (`0 < uc < full`) and `unstaged == (0, 0)` on both attempts.
  - One short side, own or shared, now fails (RV22's C-N2).
  - The new test `every_stopped_build_leaves_nothing_unstaged` sets case limits at every segment end and midpoint.
- **The runner is unchanged:** it reports these fields and does not decide the check.
- **Note, not edited (ROOT's instruction at B):** the doc comment in `H/src/bin/k6_observe/w1.rs:6-10` says the unstaged work is "zero on completed builds". Since KF3 it is zero on every build, stopped ones included. The comment is stale but not false. It is outside the write set, and I left it for H's next change.

## 4. Why honesty is unaffected (every reader of Uc, S and B)

**What the guarantee uses.**
- Lemmas A to C, the Theorem (steps 3, 5 and 7) and the Corollary use B_c only through B_c ≥ ‖K̃_c⁻¹‖₁, for each block with data.
- B_b uses it as the maximum over those blocks.
- Under A2, B_c is the minimum of a nonempty set of formed bounds. Each formed Uc_c is at least ‖K̃_c⁻¹‖₁ by Lemma D, and each formed S_c by Lemma E, so the minimum is too.

**A refusal changes no formed value.**
- A refused block's rows are skipped.
- Every operation on another block's rows reads only that block's values: cross-block multipliers are exact zeros, which the passes already skip (7a).
- The shared accumulator is reset after a refusal, so it holds no stale limb.
- So every formed Uc_c and S_c is bit-identical to the one formed without the refusal, and Lemmas D and E apply as they stand.
- This is tested: U2 checks bit-identity; the reset test and U4 check the accumulator.

| Reader | Uses | Under A2 |
|---|---|---|
| 7c's shift condition | Uc_c missing, or above 2⌈√n_c⌉·est_c | A refused Uc is "missing", the same branch as t_c ≥ 1. Availability only: Lemma E holds for any σ_c. |
| 7d (`certified`, `certificates`) | min over existing | Only formed bounds enter. It is never 0 or a placeholder (mutant M1 is killed). |
| θ_c (item 9, Lemma C) | B_c | Needs only B_c ≥ ‖K̃_c⁻¹‖₁. |
| t₁, N_u, t₃ and C_q (item 11) | B_b | The same. The Theorem's steps 3 and 5 are unchanged. |
| W⁺ (item 12), hence (a)'s V_q for translation and rotation rows | B_b through t₁ and t₃ | The same (step 7). |
| `uc` (item 13) | no bound on a block with data | Unchanged when nothing was refused. With a refusal the attempt stops, as before. |
| The receipt and G5a: B_b finite and positive; item 14's encoding | B_b | θ ≤ 1/2 still gives B_c ≤ 2^(P−6). |
| g check (item 10) | data flags, not B | Unaffected. |
| E, ê, Φ, the stop rule's M_q, the classification | the state and E | They never read Uc, S or B. |
| est_c | the screen's solves | Unchanged. It feeds only σ_c (decision 9). |
| Work and the budget | charged totals | A refusal's work is charged. Budget stops are never refusals (M6 is killed), so D1 §4.1.7 holds. |

**Measured:**
- Every constructed and RF-LARGE case that A2 moved to selected is honest:
  - KF3-UC-SPAN on 8,983 checks against GEN's high-precision solves;
  - the three RF-LARGE frames on every row against R1.
- G5a passes on every one.

## 5. The constructed controls (`K4T/kf3.txt`; decision 4)

**The model is larger than the plan said.**
- The plan's sizing assumed slenderness raises α. Measured through GEN's exact emulation of K4, it only adds a constant to log2 U.
- A search over chains with rational axes found **KF3-UC-SPAN**:
  - a straight chain of 390 members along (−1, 12, −12), each 17 long, with y_ref (−12, −9, −8), orthogonal to the axis, with norm 17;
  - fixed at node 0, with a tip force and moment;
  - R1's section scaled by exact powers of two: A·2^-26, Iy·2^16, Iz·2^28, J·2^10. These are invented inputs.
- α is 20.8 bits per member, and the first refusal at 256 falls between 370 and 380 members.
- The model has 2,340 free DOFs.

**Main `0f5d8c7b4`** (a `git archive`, `_run_records/a/before_main_0f5d8c7b4.txt`):
- KF3-UC-SPAN and KF3-UC-SPAN-ZERO both end `Unresolved(ExactSumSpan)`.
- The 256 verification-shared total is 82.39 M LME, of which 70.51 M is staged: RF-LARGE-10,000's pattern.

**KF3:**
- **KF3-UC-SPAN** is selected at 128, token-equal to GEN.
  - Its refusal is `refused:span:backward:66`, and B = S_c ≈ 2^62.
  - θ is 8.0e-55, C/allowance 2.6e-37 and Ŵ/V 1.4e-2. E-UNIT, E-UC and E-CHARGE are bit-equal to GEN.
  - It is honest on 8,983 checks. The worst is 0.969 of its allowance, at `mag.55`: the value is 2.03, just above a power of two, and publication's half-ulp alone can reach about 0.985. G5a passes.
- **KF3-UC-SPAN-ZERO** has no loads. It is selected; its block has no data, needs no bound, and its refusal is recorded.
- **With the hook (no S):** `Unresolved(ExactSumSpan)`, with tokens `128:rejected:verification_failed 256:failed:Span`.

**Debug CI time:** 13.6 s for KF3-UC-SPAN, 9.1 s for the ZERO model, about 14 s with the hook, and about 45 s for the bit-level test. No test exceeds 60 s.

## 6. Tests

**`bound_tests.rs`** (a synthetic nilpotent block, K̃ = L·Lᵀ with m = 2^20, growing 21 bits per node, with ‖K̃⁻¹‖₁ = (1 + 2m)² exactly):
- **U1 and U2:** the refusal, B = S ≥ the exact norm, the second block bit-identical, and B = min where both bounds exist;
- **U5 and the precedence:** a stop only with data, and a stop before `uc`;
- **U3:** a budget stop is not a refusal;
- **U4:** σ = 2^-9000, so S_c is refused and B = Uc_c, and the accumulator is exact afterwards.

**`wide_sum_tests.rs`:** `reset`.

**`kf3_tests.rs`:**
- `kf3.txt` pinned;
- bit-level against GEN: E-UNIT at 128 and 256, E-UC and E-CHARGE at 256, both models;
- KF3-UC-SPAN, KF3-UC-SPAN-ZERO and the hook control;
- budget sweeps through all four builds on N05 and TWO-SPAN;
- `Condition`, `Pivot`, `ResidualGate` and `ResolutionScale` stops;
- a budget stop inside `uc` (K6b's case).

**`method_tests.rs`:** the stage identity on every attempt of every control and combination.

**`scale_tests.rs`:** refusal tokens, the `-` norm, and `certify`'s stop.

**H:** see §3.4.

## 7. Mutants (checkpoint A; `_run_records/a/mutants.txt`)

One at a time, each on a clean copy with its own target, deleted afterwards. The NONE control passes 12 of 12. Every mutant is killed:

| Mutant | Killed by |
|---|---|
| M1: refused Uc as 0 | 5 tests, including G5a's positive-B check |
| M2: B = max | U1 |
| M3: stop dropped when neither bound exists | W2 and U5 |
| M4a–d: partial stage left unrecorded, in each of the four builds | the sweep |
| M5: refusal not per block | U2 |
| M6: budget as a refusal | U3 |
| M7: a refused block without data stops | W3 and U5 |
| M8: pre-KF3 behaviour | 7 tests |

## 8. What moved

- **K4's controls:** no outcome, row, class, bound or golden work count changes. GEN's earlier records are byte-identical.
- **Stage records of failed attempts** now hold their partial stage, with totals unchanged: 24 `Pivot`, 2 `Condition` and 3 `ResolutionScale` attempts in `outcomes.txt`. No golden pins them.
- **Every control that previously ended in the unavailable-bound stop, and its outcome now:**
  - KF3-UC-SPAN: selected at 128.
  - KF3-UC-SPAN-ZERO: selected at 128.
  - RF-LARGE-CHAIN-n10000-AX: selected at 128; 103 of 103 R1 rows pass.
  - RF-LARGE-CHAIN-n10000-ROT: selected at 128; 103 of 103 pass.
  - RF-LARGE-CONT-n10000-ROT: selected at 128; 214 pass and 1 absolute-range pass, of 215.
  - RF-LARGE-TREE-n10000-AX: `Unresolved(Ceiling)`, nothing published.
  - RF-LARGE-TREE-n10000-ROT: `Unresolved(Ceiling)`, nothing published.
- **Unchanged:** RF-LARGE-CONT-n10000-AX (no refusal), at the same charged work. All twelve frames at 100 and 1,000 members have byte-identical records.

## 9. The merges

**Main `f8400d290` (V-K), `c0473301e`:** one conflict hunk in `AttemptRecord`.
- A0's `pub` is kept on `verification_shared_work` and `verification_shared_built_here`, and `bound_refusals` is now `pub`.
- `BlockRefusal`, `BoundRefusal`, `RefusalKind`, `BoundPass` and `CertifiedBound` are `pub` and exported through `retained_api` (ROOT's ruling).
- I edited only; ROOT committed.
- **Results:**
  - FK, FK with `mutation-controls`, VR and H build with no warnings;
  - FK's full suite passes (348 lib, 7 integration files, 6 doc-tests), and so does VR's (47);
  - `vk_records --write` reproduced every committed VR record byte for byte, so no VR record changed (`_run_records/merge/`). No CI-scale VR case stops inside a build, and VR's records do not serialize `bound_refusals`;
  - the kill matrix was not re-run: ROOT's condition was a record change.

**Main `78f55f927` (K6b), `e114b23c1`:** no conflict; K6b changes no FK file. H's update is §3.4.

## 10. Suites at D

| Suite | Result |
|---|---|
| FK's full suite, on the merge (`_run_records/merge/fk_suite.txt`) | 348 lib tests pass (KF3's 11, K4's and KF1's included), plus 7 integration files and 6 doc-tests. No warnings. |
| VR (`_run_records/merge/vr_suite.txt`) | 47 of 47 |
| H, `--all-targets` (`_run_records/h/`) | lib 25, `k6_alloc`, k6_bin 11, counts 4, models 7, parity 4, staged 3, k6b_adapter 2, k6b_bin 3, k6b_export 1, k6b_w1 15. No warnings. |
| H's runner | 47 of 47 |
| GEN (`_run_records/a/gen_check.txt`) | `--check`: every file OK |

## 11. Scale evidence (checkpoint B, ROOT's slot)

**Setup:**
- `vk_scale` (release) was built from a read-only `git archive` of `e114b23c1`, with no warnings; its sha256 is `2bc2a436…`.
- V-K's runner, unedited, ran V1 → V2 → V3, with 10,000 members approved and C = 8 GiB.
- The 12 large models match `cases/large_models.sha256`.
- **V1 and V2** (12 models): every per-case record is byte-identical to V-K's B (pre-KF3), charged work included.

**V3** (`_run_records/b/summary.txt`, `runs/`):

| Frame | Outcome | R1 rows | Charged, KF3 / pre-KF3 (G LME) | W1 time | Heap (VR's E_max; see §13) | Uc refusal |
|---|---|---|---|---|---|---|
| CHAIN-AX | selected at 128 | 103/103 pass | 10.12 / 6.75 | 7.8 s | 854 MiB (2,749) | 256, forward, row 28,302 |
| CHAIN-ROT | selected at 128 | 103/103 pass | 12.08 / 8.30 | 14.7 s | 854 (2,749) | 256, forward, row 14,784 |
| TREE-AX | Unresolved(Ceiling) | none published | 65.32 / 6.64 | 40.2 s | 2,890 (2,750) | forward at 256, 512 and 1024 |
| TREE-ROT | Unresolved(Ceiling) | none published | 74.17 / 8.11 | 63.8 s | 2,892 (2,752) | forward at 256, 512 and 1024 |
| CONT-AX | selected at 128 (unchanged) | 211 + 4 absolute-range | 8.135 / 8.135 | 6.2 s | 817 (2,676) | none |
| CONT-ROT | selected at 128 | 214 + 1 absolute-range | 10.42 / 6.80 | 11.8 s | 818 (2,677) | 256, forward, row 36,891 |

**What the table shows:**
- **The three frames that moved to selected:** B = S_c (1.3e19, 9.1e20 and 6.9e10), with one shifted factorization each and θ at most 1.6e-52. Every published row passes R1's predicate, and every C9 S_full set is empty.
- **TREE-AX and TREE-ROT:** R7's verification estimate (b) rejects at 128, 256 and 512, at member 1's end force (the root).
  - A2 replaced the refused Uc with S_c at every verification (θ ≤ 7e-53); the rejection is (b)'s.
  - That is the DIRECTIONAL-SPAN kind of availability loss, and honest: nothing is published.
  - **The runner summary's `fail` column (194 on each TREE frame) counts unpublished rows.** When a case is not selected and not on the expected-unresolved list, VR's lane marks every R1 row of the case `Fail("case …")` without a comparison (`VR/src/lane.rs:314-322`). The W1 line has `published_rows: null` and `selected_precision: null`. **No wrong value was published.**
  - **Corrected at D (§13):** their heap peak, 2,890 MiB, exceeds the E_max that V-K's runner reports, 2,750 MiB (ρ ≈ 1.05). That E_max is VR's port of an earlier K6b estimate. My B reading, that "E_max leaves out the shifted factorization's two profile copies", holds for that port but not for K6b's final formula.
- **The runner's stop.** The runner stops a tier on "not selected". I re-ran `--run V3`, which skips runs already recorded, to finish the CONT frames. The refusals come from a scratch probe (`_run_records/b/kf3_refusals.rs.txt`, output `refusals.txt`).
- **Stop check:** no published result changed outside the frames that stopped with the refusal before. The memory guard logged no kill.

## 12. KF3-B1: the TREE frames' `Ceiling` is estimate (b)'s

**What the B records hold.** For each attempt, the records give only the quantity that (b) rejected, not Ŵ_q or its threshold (`_run_records/b/runs/15_…`, `16_…`, the `record` line):

| Attempt | Role | TREE-AX rejected at | TREE-ROT rejected at |
|---|---|---|---|
| 128 | candidate, verified at 256 | member 1, end I, Uy | member 1, end I, Ux |
| 256 | verifies 128; candidate, verified at 512 | member 1, end I, Uy | member 1, end I, Uz |
| 512 | verifies 256; candidate, verified at 1024 | member 1, end I, Uy | member 1, end I, Ux |
| 1024 | verifies 512 only | (verification solved) | (verification solved) |

So (b) is tested three times, at P = 256, 512 and 1024. Each of those verifications ran one shifted factorization, with `uc_missing: None`. There is no candidate at 1024, because W1 has no 2048 verification. The fourth rung is therefore the `Ceiling`.

**The light probe** (`_run_records/d/b1_probe.txt`; source `kf3_b1_probe.rs.txt`, models `gentree.py.txt`):
- It prints ratio = Ŵ_q / (2^(6−P)·ê) for every force and moment row. (b) rejects above 1.
- It builds each pair exactly as W1 does, in release, on the scratch archive of `e114b23c1`.
- The models are R1's comb tree through GEN's R1 adapter. The family and load case are those of RF-LARGE-TREE-AX, but these are not VR's committed files (K4SRC differs).
- The longest pair took 11.6 s. c = Ŵ_q·2^P/ê = 64 × ratio is the (b) coefficient the row would need.

| Members | P = 256 | P = 512 | P = 1024 | Worst row | c |
|---|---|---|---|---|---|
| 1,000 | 0.321 (selected) | – | – | member 11, end I, Uy | 20.5 |
| 2,000 | 0.395 (selected) | – | – | member 1, end I, Uz | 25.3 |
| 3,000 | 1.158 | – | – | member 3, end I, Uz | 74.1 |
| 4,000 | 1.0125195053 | 1.0125195054 | 1.0125195073 | member 1, end I, Uz | 64.80 |
| 6,000 | 2.233 | – | – | member 1, end I, Uz | 142.9 |
| 8,000 | 2.767 | – | – | member 1, end I, Uy | 177.1 |

**The probe reproduces the rejection.** From 4,000 members up, (b) rejects on member 1's end force at the root. At 4,000 members the three verifications reject the same row with the same ratio to nine digits. **At 10,000 members the ratio is not in the records.** I did not probe that size: ROOT allowed only a smaller one.

**Classification.**
1. **The ratio does not depend on P.** Ŵ_q = c·2^-P·ê, and c is a property of the model. It grows with N, though not monotonically. (b)'s threshold is 64·2^-P·ê. So a row that fails (b) at one precision fails it at every precision, and escalation can only reach the `Ceiling`. In that respect it is like THIN: no rung of the ladder helps. THIN's cause was a stiffness spread beyond the ladder's reach; here the cause is (b)'s coefficient against c(N).
2. **What the guarantee needs** (R7 §4.1.6.3, Corollary: `T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md:578-583`, on the T3 records branch at numerics `69737e699`, sha256 `5502aef9…`). At p = 128 and 256, the bracket is 256 − 69 − 64·(1 + 2^-2p) − 60. With c in place of (b)'s 64 it is 127 − c·(1 + 2^-2p). So:
   - b bounds |q_p − q*| while c·(1 + 2^-2p) ≤ 127;
   - the Corollary's stated margin (< 2^-64·M_q − 62·2^-2p·ê) holds while c·(1 + 2^-2p) < 65;
   - at p = 512 the charge enters as 2^-86·M_q, and the bracket is 256 − 69 − c·(1 + 2^-1024) ≈ 187 − c.
3. **The probe's rows, against those limits:**
   - **4,000 members, c = 64.80: slack in (b).** The Corollary holds with its stated margin: the bracket is 62.2.
   - **3,000 members, c = 74.1: slack in (b).** The guarantee holds; only the stated margin shrinks, to 52.9.
   - **6,000 and 8,000 members, c = 142.9 and 177.1: beyond (b)'s share.** At 128 and 256 these rows need more than the 127 that R7 leaves for (b) while (d) keeps its 60. That is a design limit of the current split of λ = 2^8.
     - A re-split of (b) and (d) within λ could still certify them, but only if C_q ≤ (187 − c)·2^-2p·ê, that is 44 or 10. The probe did not print C_q.
     - Beyond c ≈ 187, λ itself would have to grow, which tightens rule (a).
4. **At 10,000 members:** the rejecting row is the same kind (member 1's end force), and in the probe c exceeds 127 from 6,000 members up. Extrapolated, not measured: the RF-LARGE-TREE frames are most likely beyond (b)'s share, a design limit of the current split, and possibly beyond 187.

**My reading for ROOT's ruling:** near the threshold (3,000–4,000 members in this family) the loss is slack in (b). At the scale of RF-LARGE-10,000 it is most likely a design limit of R7's split, like THIN in that no precision helps. It is honest either way.

**Plainly:**
- The runner summary's `fail` column (194 on each TREE frame) counts R1 rows that were never published. VR's lane marks every row of a case that is not selected `Fail("case …")`, without comparing anything (`VR/src/lane.rs:314-322`).
- No wrong value was published: `published_rows` and `selected_precision` are null on both frames.
- Their charged work before `Ceiling` is recorded (65.3 and 74.2 G LME; §11) for W1's limits.

## 13. KF3-B2: the heap against E_max (`_run_records/d/b2_emax.py`, output `b2_emax.txt`)

**Two E_max implementations.**
- **VR's** is the one that V-K's runner reports and admits with (`VR/runner/vk_scale_runner.py:22-23`). It is `VR/src/scale.rs:279-390`, a port of K6b's `estimate` at `082990c8d` (`scale.rs:11-14`), from before RV22's review. Its pass term is one profile (`scale.rs:354`: `p_entries × w(l)`), taken with the report's row vectors.
- **K6b's final formula** is in H's `counts.rs:411-560`; RV22-2 covers the pass at `:501-518`. It adds the pass's locals and, at the shift, both profiles and the shifted factor's vectors.
- B compared the measured peaks with VR's port. My B reading ("the shifted factorization's two profile copies") described that port, not K6b's final formula.

| (bytes) | TREE-AX | TREE-ROT |
|---|---|---|
| Measured heap peak (`vk_scale`, K6's counting allocator) | 3,030,269,132 | 3,031,977,247 |
| VR's E_max (the port) | 2,883,922,498: +146.3 MB, ρ 1.051 | 2,886,171,220: +145.8 MB, ρ 1.051 |
| K6b's E_max (H `observations/k6b/counts.jsonl`; `k6_observe`'s harness) | 3,010,765,802: +19.5 MB | 3,012,624,342: +19.4 MB |
| K6b's kernel terms (E_max − H's fixed term) + `vk_scale`'s fixed term (VR's estimate, 76.8 and 79.0 MB) | 3,037,239,854: −7.0 MB | 3,039,488,576: −7.5 MB |

H's fixed term (50.3 and 52.2 MB) is `k6_observe`'s own model and source. `vk_scale` holds its own, which VR estimates at 76.8 and 79.0 MB. The like-for-like comparison is the last row: **on `vk_scale`'s own fixed term, K6b's final formula bounds both measured peaks.** The margin is 7.0 and 7.5 MB, and that fixed term is itself an estimate.

**Which allocations are alive at the peak.**
- The peak phase of K6b's formula on these frames is the 1024 verification's pass at the shift: kept 2,568,083,750 + pass 442,682,052 = E_max, for AX.
- In the code, that phase peaks inside `nl_pass` on the shifted factor: `shift_schedule` calls it at `K4R/bound.rs:1040`, and `ct` is formed at `:525`.
- Alive then:

| Allocation (file:line, KF3) | Code, at 1024 (w = 144 B) | H's term |
|---|---|---|
| Kept: 4 shared builds, 4 states, 3 verification-shared (`H counts.rs:530-548`) | as H | 2,568,083,750 |
| The pass's locals: `w_abs`, `terms_*`, `prescribed_*`, `u_free`, `r_hat`, `sr_row`, `delta`, `delta_full`, `recovered`, `sr2_row`, `sas_*`, `sau_row`, `w_s` (`K4R/verify.rs:751, 785-788, 813-815, 840, 845, 850, 872, 907-909, 940`) | as H | `live`, `:506-510`: 142,686,492 |
| `e_rows`, `w`, `a_s`, each `Vec<Option<Wide<16>>>` (`verify.rs:752, 863, 944`) | 3·rows·144 = 108,005,616 | 3·rows·(144 + 8) = 114,005,928 |
| The scaled profile (`bound.rs:750-783`, built at `:1215`) | 84,234,816 | 84,234,816 |
| The shifted factor: `first`, the rows' clone, `shifted` (`bound.rs:839-841`; returned at `:914-920`) | 92,634,816 | 93,114,816 (`shifted` at 152 B) |
| `shifted_factor`'s `work` (`bound.rs:873`) | freed at `:914`, before `nl_pass` | 8,640,000 |
| `nl_pass`'s `at`, `bt`, `ct` (`bound.rs:485, 509, 525`) | 3·nf·144 = 25,920,000 | not counted |
| Per-block `failed` and `refused` (`bound.rs:842, 872, 1039`) | 1 block: tens of bytes | – |

**The net under-count at the 1024 shift is 10,799,688 B.**
- `at`, `bt` and `ct` less `work`: +17,280,000 B.
- H's `OPTION_EXTRA` (`counts.rs:259`) over-counts 8 B on each `Option<Wide>` entry, because `Option<Wide<L>>` is the size of `Wide<L>` (checked with rustc 1.97.1: `_run_records/d/optsize.txt`). That offsets −6,480,312 B.
- With this phase taken from the code, K6b's E_max on H's harness would be 3,021,565,490 (AX) and 3,023,424,030 (ROT).
- So K6b's formula does not bound this phase by construction. The `vk_scale` measurement stays inside it through slack elsewhere, in the fixed term or other terms, not through this term.
- No `k6_observe` run of these frames exists, and I made none, because it is heavy.

**The other `nl_pass` site.** In the verification-shared build, `uc_bounds` (`bound.rs:680-681`) holds `c` plus `at`, `bt` and `ct`: 4·nf·w = 34.6 MB at 1024.
- That phase's modelled transient (`verify_build`, `counts.rs:488-495`) is 214.7 MB at 1024.
- I did not trace whether that phase's formation transients overlap `uc_bounds`. It does not set the peak.

**Pre-KF3 code holds the same allocations on the same path.** On main `78f55f927`:
- `nl_pass` (`bound.rs:282`) forms `at`, `bt` and `ct` at `:292`, `:302` and `:306`;
- `shifted_factor` (`:548`) clones the rows at `:560` and forms `work` at `:579`;
- `shift_schedule` (`:687`) calls `shifted_factor` at `:721`, then `nl_pass` at `:723`;
- `verify_state` builds the profile and runs the schedule at `verify.rs:964-971`.

KF3 adds only the per-block refusal slots. It moves the same calls, in the same order and with the same lifetimes, into `shift_run` (`bound.rs:1196-1232`), whose profile is dropped at return, as before at the end of its branch.

**What KF3 changed is reachability.**
- Before A2, the forward-pass Uc refusal at 256 stopped these frames in `build_verify_shared`, so they never reached 512 or 1024.
- Under A2, Uc is unavailable. A missing Uc needs 7c's shift (`bound.rs:986`), and the attempts run on to the 1024 verification with its shift.
- So this is **a K6b omission that no earlier run reached, not KF3-induced.** Per ROOT's rule, I stop at D without a fix, and I did not edit H's `counts.rs` or `counts.jsonl`.

**For ROOT's routing:**
- **The term.** In `at_shift` (`counts.rs:515`), `nl_pass`'s three vectors replace `work`: +2·nf·w(l). That is 17.28 MB at 1024 on these frames. The `OPTION_EXTRA` over-count, 6.48 MB here, is slack that ROOT may keep or remove.
- **VR's port** is behind K6b's final formula. On these frames its kernel terms are 153 MB below K6b's, and V-K's runner admits with it. Re-porting it, or using H's crate, is a VR change.

## 14. Findings for ROOT

1. **TREE-n10000-AX and -ROT now end `Unresolved(Ceiling)`,** rejected by R7's verification estimate (b) at every candidate precision, at the root member's end force.
   - This is honest and publishes nothing. The `fail` count of 194 is unpublished rows (§12).
   - It costs 65–74 G LME and 40–64 s per call, against about 8 G before, because the attempts now run to the ceiling.
   - It bears on ROOT's W1 limits.
   - (b)'s ratio does not depend on P. The loss is slack in (b) near the threshold, and a design limit of R7's split beyond c ≈ 127, which the probe reaches at 6,000 members (§12).
2. **E_max (§13).**
   - VR's port of K6b's E_max is stale, which is what B compared with.
   - K6b's final formula, on `vk_scale`'s fixed term, bounds the measured peaks by about 7 MB.
   - K6b's final pass term still under-counts the 1024 shift by 10.8 MB net, because it leaves out `nl_pass`'s three vectors.
   - This is not KF3-induced: it is a K6b omission that no earlier run reached.
3. **The constructed model has 390 members,** not the plan's "under about 200". Its debug time is within ROOT's 60 s per test.
4. **An observed rate, not a claim:** at 10,000 members the forward-pass refusal lands where α predicted it, on all five frames (§2).
5. **Note:** the doc comment in `H/src/bin/k6_observe/w1.rs:6-10` ("zero on completed builds") is stale since KF3, and is left as ROOT ruled (§3.4).

## 15. Files (sha256)

| File | sha256 |
|---|---|
| `FK/src/structural.rs` | `cc77bb4ceede7e5e687ab9d0be0e06598f39d4e3c7b48014365f5cc38d526847` |
| `K4R/adaptive.rs` | `9017fd724411928784d2bd9d77a178cf7e8195695008822147d914ae2eda3668` |
| `K4R/bound.rs` | `26e99a40dd84dbe017159903598b5af5c9f5d0348686ec9d216592cf17e82959` |
| `K4R/verify.rs` | `9d4055a8b0af167bdba12cd4664d0c008d5fe1deaf13a388a90af2c2b680cc73` |
| `K4R/wide_sum.rs` | `80441f99a0fdcb4544d5b95e1012bc5bf291089ff8cebbdcdcaac437d565a188` |
| `K4T/SHA256SUMS` | `3aebae45d93eb3bc017298f42d316d81a6ef2004a5e86ff414f485b418917fa9` |
| `K4T/bound_tests.rs` | `fdf10144c9490e12a071251d5b1915a124c17dc6290792255fa00f6901e23102` |
| `K4T/gen_k4_vectors.py` | `8c8f22aa8492736624045f866f5f8b47e9b3ee4639e1d038e02fa2425b6c7aa7` |
| `K4T/kf3.txt` | `729e7ef86ba0b61353a167a610d53910e0deddb357c71948dd742993fca6f75a` |
| `K4T/kf3_tests.rs` | `43b37a782d4985f2507323d027c0866a17373e94f117a39ae17abec254c88b68` |
| `K4T/method_tests.rs` | `8f60c0faab1538b0e6a3785998033d7d2d7c0ae080f6ba01ad63ac8e890c498e` |
| `K4T/scale_tests.rs` | `922f03cf28cdfa402b16e254077a2f2fc3bb553acf77581be9d828177243c350` |
| `K4T/wide_sum_tests.rs` | `94ee29a25bee27b2a21f31212a49a1ac7c6cee72796aa95a13158ab5bc0dcbc8` |
| `H/src/k6/w1/staged.rs` | `fd3e00c06aff970c44cd147f255fe371b16729fe0a80b62b32a09bc0afddc337` |
| `H/tests/k6b_w1.rs` | `09faf31674c432c189ac02361f5bbf01214419f0f490ac78440eed973529c100` |

D changed no code, so these hashes are those of `ae831ca51`. The records are in `T3/IMPLEMENTATION/KF3/`, with `SHA256SUMS`.

## 16. Not done, and limits

- **Mac only.** No timing or memory-growth claims; K6 and K6b own them. The times above are observations.
- **The kill matrix** was not re-run after the V-K merge. ROOT's condition was a VR record change, and none occurred.
- **No certified norm** is computed for the constructed models (`-` in E-UC): they are too large for GEN's bound. Their honesty rests on `compare_honest` against the high-precision solves.
- **Not observed:** the Exponent refusal and S's refusals occur in no model. They are exercised at unit level only (U3, U4).
- **KF3-B1 at 10,000 members:** the ratio is not in the records, and I made no probe at that size, following ROOT's instruction.
  - The probe covers the AX load case only.
  - Its models are GEN-adapter trees, not VR's committed files.
  - It did not print C_q, which a re-split of (b) and (d) would need.
- **KF3-B2:** no `k6_observe` run of the TREE frames exists on H's own harness. The phase breakdown is derived from the code, not measured.
