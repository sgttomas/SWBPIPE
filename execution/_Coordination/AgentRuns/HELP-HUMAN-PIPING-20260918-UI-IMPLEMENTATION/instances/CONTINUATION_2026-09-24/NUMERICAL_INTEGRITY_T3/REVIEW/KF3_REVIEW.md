# RV23: independent review of slice KF3 (A2: an unformed certified bound is unavailable; partial stage work recorded)

- **Reviewer:** RV23 (Type 2 TASK, independent reviewer). I did not write KF3, and I used I19's tests only as suites to re-run, never as my oracles.
- **PR:** [#1059](https://github.com/sgttomas/chirality/pull/1059), branch `codex/piping-kf3-20260929`.
- **Head reviewed:** `b8c55c92e` (`b8c55c92e4762f117ba60a608549b1786df4b433`). Base main `78f55f927` (K6b merged). The implementer was I19.
- **Date:** 2026-09-29 (host clock; the rulings are dated 2026-09-30).
- **Verdict: PASS.** No BLOCKING finding. 1 SHOULD-FIX and 5 NOTEs.

**In short.**
- **A2 is honest.** Every reader of Uc, S and B reads B_c through `certificates` (`K4R/bound.rs:1239-1284`), which is `certified` (min over the `Some` bounds, `:1114-1131`). A refused Uc_c is `None` (`BlockBound::refused`, `:601-611`), never a number. My three independent oracles agree:
  - **Synthetic, genuine refusals, exact norms.** Five blocks in one profile (two nilpotent chains of 600 and 585 rows whose comparison-matrix bound outgrows the span, a tridiagonal, a pentadiagonal and a near-singular 2×2). My own Python passes (written from R7 7b and the ruling, not from GEN or the Rust) give the Rust's refusal pass and row and every other block's U, N_L, t and Uc bit for bit; so do GEN's A2 passes. Each block alone equals its value in the combined run, Uc and shift alike. B_c ≥ the exact ‖K̃_c⁻¹‖₁ (Fraction inverse, or X with K̃·X = I checked exactly) and B_c = min over the formed bounds on every block. A second case, 1,209 rows, refuses in the forward pass (row 1,119), and a refused S (σ = 2^-9000) leaves B = Uc; GEN's A2 shift schedule agrees with the Rust on both.
  - **Forced refusals through W1.** In my copy only, one block's operation in one pass returns `Span` (a small probe patch, `scripts/rv23_probe_patch.diff.txt`). On 16 controls, 52 forced runs: Uc refused in the forward, pivot or backward pass on 34, S refused on 4. 50 stay selected with the same worst honesty ratio against GEN's exact solutions and G5a passing; the 2 others (HH-FOOL-m40-LOADED's hidden block, where every shift fails) end `Unresolved(ExactSumSpan)`, as the rule says. On 77 verification blocks B_c ≥ the exact norm of the verification's own K̃_c, and no other block's Uc, S or B moved.
  - **KF3-UC-SPAN in closed form.** It is a straight cantilever, so its exact solution is statics plus Euler–Bernoulli beam theory, in rationals. GEN's 8,983 expectations for it match that closed form to 2^-128 (the tokens' rounding) on every key. The Rust's 7,423 published rows (7,415 relative, 2 absolute, 6 input-derived) lie within their claims of that closed form, the worst at 0.954 of its claim (`u.80.1`).
- **The rules hold** (code, oracles and mutants): a block with data and no bound stops with its refusal, which outranks `uc`; a budget stop is never a refusal; refusals are recorded per block. **Except** that a refusal is dropped from the evidence when a budget stop follows it inside the same build (RV23-1).
- **The partial-stage identity holds on all four builds.** 3,611 W1 runs with case limits at every stage midpoint, every segment end ±1 and 10 LME, and 60–300 even points, on 11 models (KF3-UC-SPAN included): 0 mismatches. RV22's six limits on H's own CHAIN-n00010-AX reproduce exactly, and each formerly short side is now equal (for example `solve_128`: shared 28,740/28,740, was 0/28,740; `into_solve_128`: own 100,728/100,728, was 576). A 2,011-limit sweep on H agrees with H's `stages_equal_totals`.
- **Invariance.** `gen_k4_vectors.py --check` passes on every file (16:19 wall), and the only vector files the PR touches are the new `kf3.txt` and its `SHA256SUMS` line. FK's full suite passes (348 lib tests, 7 integration files, 6 doc-tests), with the goldens and every control token-equal to GEN. KF3's B records at 100 and 1,000 members and CONT-n10000-AX are byte-identical to V-K's pre-KF3 B.
- **B reproduces.** My release `vk_scale` from a clean archive of the head gives RF-LARGE-CHAIN-n10000-AX's and RF-LARGE-TREE-n10000-AX's per-case `record` lines byte for byte (2,891 and 5,491 bytes), and every other line equal outside its time and heap fields. Every published row of the four selected 10,000-member frames passes R1; the TREE frames publish nothing, and their 194 `fail` are VR's unpublished-row marks (`VR/src/lane.rs:314-322`).
- **Reach:** no product crate names `retained_api` or `retained`. KF3 is kernel plus harness only.
- **Mutations:** 18 mutants and NONE, each from a clean archive with its own target. 15 are killed, including the brief's seven and five of I19's (re-implemented from `CHECKPOINT_A.md` §4, since their diffs are not recorded). Three survive: RV23-M4b (RV23-N1), and RV23-M5c and RV23-M7, equivalent on every reachable path (RV23-N4).

## Findings

| ID | Class | Site | Evidence | Remedy |
|---|---|---|---|---|
| RV23-1 | SHOULD-FIX | `K4R/bound.rs:666-702` (`uc_bounds`: the `refused` slots are a local, lost on `?`), `:1036-1040` and `:1225-1230` (`shift_schedule`/`shift_run`: a refusal kept in `results` is lost when a later factorization or `nl_pass` returns `Err`); `K4R/adaptive.rs:3028-3030` (`let vs = vs?;` precedes `record.bound_refusals = …`); the claims at `adaptive.rs:2519-2521` ("on every path"), RETURN §3.1 and CHANGE_RECORD ("filled on every path"). | **A refusal is dropped from the evidence when a budget stop follows it inside the same build.** Probe `rv23_refusal_evidence_on_a_budget_stop_after_the_refusal` (`probes/sweep.out`): KF3-UC-SPAN with the case limit leaving the 256 verification's shared build 1, 2 or 100 LME short. The build did all of its work, the refusal included (verification-shared 82,390,690 of 82,390,690 LME; `uc` 11,879,623 of 11,879,623), then stopped at the next guard check. The 256 attempt ends `Failed(Stop(Budget(Case)))` with `bound_refusals: []`. The same holds for an S refusal in factorization k followed by a budget stop in factorization k+1 (by reading). Honesty is unaffected: a budget stop publishes nothing. But the ruled evidence ("refusals are recorded per block", A2's "every refusal is recorded in the evidence") and the doc's "on every path" do not hold on this path, and F2a will publish this field. | Either carry the slots out on the error path (the caller owns `refused` in `uc_bounds` and `shift_schedule`; `build_verify_shared` returns the shared build's refusals in `VerifySpent.refusals` on `Err`; `verify_precision` records them before `vs?`), with a test at my limit; or narrow the doc, RETURN and CHANGE_RECORD to "except a budget stop inside the build after the refusal". ROOT chooses. |
| RV23-N1 | NOTE | `K4R/verify.rs:1011-1015`; `K4R/bound.rs:1264-1270`. | **The precedence is tested only inside `certificates`.** RV23-M4b (`verify_state` honours the stop only when no block is `uc`) survives every KF3, K4 and method test. U5 tests `certificates`' two return values, not their composition. No control has both a refused block with no bound and a plain `uc` block, and building one needs a second test hook. Honesty is unaffected: both branches publish nothing at that precision. | Return one decision from `certificates` (stop, `uc`, or none) so the precedence lives and is tested in one place; or add a `verify_state`-level test when a hook exists. |
| RV23-N2 | NOTE | `K4R/verify.rs:960-1015`. | **"As before" holds for the outcome, not for the order.** Before KF3 a refusal stopped the shared build, before the pass. Now the refusal stop is decided at 7d, after the pass's scale, estimate and charge stages, because 7d needs the state's data flags. So another stop in those stages (for example `ResolutionScale`, or a budget stop) now pre-empts `Span`, and W2's attempt is charged its scale, estimate and charge work first (asserted by I19's W2 test). This is design-conformant, and honesty is unaffected. | Say so in RETURN §3.1 ("stops with the refusal at 7d, after the pass's earlier stages"). |
| RV23-N3 | NOTE | `K4R/bound.rs:1051-1066`. | **A refusal in N′_L's pass on a failed block's rows ends that block's retries.** 7c refactors a failed block at σ/2. Under A2, `shift_schedule` checks `refused[b]` before `f.failed[b]`, so a refusal in `nl_pass` over the failed block's (unused) rows marks S_c unavailable instead of halving σ. It is availability only and conservative (before KF3 the path stopped the attempt), and practically unreachable: N′_L's sums do not compound, so a refusal needs an entry near 2^8127. | None needed; optionally check `f.failed[b]` first. |
| RV23-N4 | NOTE | `K4R/bound.rs:306` (`sum.reset()` in `refusable`); `K4R/verify.rs:1007` (`current = Stage::Bound` after `t5`). | **Two mutants survive, both equivalent on reachable paths.** RV23-M7 removes the reset: the span check refuses before any write, and every directed operation starts with `clear`, so the reset is the ruled defence in depth (decision 7) and unobservable. RV23-M5c drops the stage marker after `t5`: no guard check lies between `t5` and `t6`, and the refusal stop there follows no work, so the remainder is 0 wherever it lands. | Record; no test is needed. |
| RV23-N5 | NOTE | `H/src/bin/k6_observe/w1.rs:6-10`. | The doc comment "zero on completed builds" is stale since KF3 (zero on every build). I19 recorded it and ROOT routed it to K6c. I confirm it is the only stale text in H: `staged.rs` and `k6b_w1.rs` hold every attempt to equality, and no runner or VR code encodes the old relaxation. | K6c, as routed. |

## 1. A2's honesty (priority 1)

**The one fact the guarantee uses.** R7 §5.5 Lemmas A to C, the Theorem (steps 3, 5 and 7) and the Corollary use B_c only through B_c ≥ ‖K̃_c⁻¹‖₁, for a block with data, and B_b as the largest B_c over the body's blocks with data. Lemma D certifies a formed Uc_c and Lemma E a formed S_c. A2 (ROOT's ruling) makes a refused bound "does not exist" and B_c the minimum over the formed ones.

**Every reader, in the code at the head:**

| Reader | Where | What it reads | Under A2 |
|---|---|---|---|
| 7c's start | `bound.rs:1152-1188` (`shift_start`), `shift_needed` `:969-998` | `bounds[b].uc` (`None` when refused) and est_c | A refused Uc is `None`, the same branch as t_c ≥ 1: the shift runs. Availability only (Lemma E holds for any σ_c). |
| B_c (7d) | `certificates` `:1239-1284` → `certified` `:1114-1131` | `bounds[b].uc`, the shift's `s` | min over `Some`. A refused block contributes `None`, never its zeroed `u`, `n_l` or `t` (those three are read by no product code: `grep` over FK). |
| the stop and `uc` | `certificates` `:1264-1270`; `verify.rs:1011-1015` | data flag, bound, refusals | a block with data and no bound: its refusal (Uc's before S's) stops the attempt, else `uc`. Only blocks with data. |
| θ_c (item 9) | `verify.rs:1039-1046` | `certificates[b].b` | Lemma C needs only B_c ≥ the norm. |
| B_b, N_u, t₁, t₃ (item 11) | `verify.rs:1068-1126` | `certificates[b].b` over the body's blocks with data | the same (Theorem steps 3 and 5). |
| C_q and W⁺ (items 11, 12) | `verify.rs:1128-1176` | `body.t1`, `body.t3`; formed only when no block is `uc` | the same (steps 3, 5 and 7). |
| g (item 10) | `verify.rs:1047-1066` | data flags only | unaffected. |
| E, ê, Φ, the stop rule, the classification | `verify.rs:730-870` (before the bound stage); `adaptive.rs` `classify_rows_floored` | the state, the ledger, E | never read Uc, S or B. |
| the evidence and G5a | `adaptive.rs:2342-2357` (`summarize`), `:3296-3304`, `:3366-3370` | `body.b` (B_b), θ | B_b is a formed bound; θ ≤ 1/2 still gives B_c ≤ 2^(P−6), so it encodes. `bound_refusals` is evidence only (RV23-1). |

**A refusal changes no formed value.**
- **Blocks are contiguous in the elimination order,** and no profile entry crosses a block. RCM numbers one connected component at a time (`factor.rs:320-338`), and `first[i]` is the least rank among row i's own pattern neighbours (`factor.rs:381-392`). So `u_pass`, `nl_pass` and `shifted_factor` read no other block's rows at all. The "exact zeros skipped" argument is not even needed.
- **Shared state:** `WideContext` holds only its precision and work counters (`wide/multi.rs:1122-1125`). The accumulator's span check refuses before any write (`add_raw`, `wide_sum.rs:229-232`), every directed operation begins with `clear` (`directed.rs:98`), and `refusable` resets it in full anyway (`bound.rs:306`). `shifted_factor`'s `work` vector is per-row scratch: every entry a row reads was written earlier in the same row.
- **Budget checks** stay once per row, refused rows included (`bound.rs:876-878`, and the passes), so a skipped block moves no other block's guard position; only work moves.
- **Unreachable corner, checked:** after an exponent refusal in `shifted_factor`, the refused block's later rows keep their unfactored shifted diagonal (`:876-878`) where a failed row gets 1. No other block can read them (contiguity), and an exponent refusal needs a value beyond 2^(2^62).

**The synthetic oracle** (`probes/syn.out`, `checks/syn_check.out`; `scripts/rv23_probe.rs.txt` `rv23_synthetic_multiblock`, `scripts/rv23_oracle.py.txt`, `rv23_syn.py.txt`). One profile at P = 256 with five blocks:

| Block | Rows | Construction | Uc | S | B (log2) | Exact norm (log2) | Checks |
|---|---|---|---|---|---|---|---|
| A | 9 | tridiagonal (3, −1) | formed | formed at σ = 1/2 | Uc, −0.024 | −0.024 | B = min(Uc, S), B ≥ norm |
| F | 600 | nilpotent 3-row chain, K̃ = L·D·Lᵀ, L = I − N, N's link 2^20·u·vᵀ with vᵀu = 0, D not I | **refused, backward, row 69** | formed at σ = 2^-60 | S, 64.644 | 44.044 | B = S, B ≥ norm |
| C | 7 | pentadiagonal (6, −2, 1/2), no data | formed | — | none (no data) | −1.487 | no stop |
| F2 | 585 | a second chain, other u, v, D = I | **refused, backward, row 616** | fails 3 times (σ = 4) | none | 43.585 | **stop, `refused:span:backward:616`** |
| M | 2 | [[1, 1 − 2^-250], [·, 1]] | none, t ≥ 1 (no refusal) | fails 3 times (σ = 2^-240) | none | 250.000 | `uc` |

- The factors of F and F2 are exact (L·D·Lᵀ = K̃ in Fractions); the exact norms of F and F2 are from X = (I + N)ᵀD⁻¹(I + N), accepted only after K̃·X = I is checked exactly.
- **My passes and GEN's** (`u_pass_em`, `nl_pass_em`, `uc_from_em_a2`) both give `refused:span:backward:69` and `…:616`, and A's, C's and M's U, N_L, t and Uc bit for bit.
- **Isolation:** each block alone (same γ_m; the refusal row shifted by the block's offset) has the same `BlockBound`, and the shift of A, F, F2 and M together (three factorizations, because F2 and M fail) gives each block the same `ShiftResult` as its own schedule.
- **7d:** stop = F2's refusal, `uc` = M. With M ordered before F2 the stop still wins.

**The second synthetic case** (`rv23_synthetic_forward_refusal_and_refused_s`; `probes/probe2.out`, `checks/syn2_check.out`, `scripts/rv23_syn2.py.txt`): the tridiagonal A and a 400-node chain (1,200 rows), so that M(L)⁻¹e itself passes 2^8127.
- Uc of the chain refuses **in the forward pass, row 1,119**, as at scale. My passes and GEN's agree, and A's U, N_L, t and Uc are bit for bit.
- A shifted at σ = 2^-9000: σ′'s subtraction spans more than 8,128 bits, so **S_A is refused** (`shift_form`) and B_A = Uc_A = 2^-0.024, the exact norm to three decimals. The chain takes B = S = 2^65.129 ≥ 2^44.044.
- **GEN's `shift_schedule_em(a2=True)`** on the same K̃ gives the same S and the same refusal token as the Rust.

**Forced refusals through W1** (`probes/forced.out`, `checks/forced_check.out`; the patch `scripts/rv23_probe_patch.diff.txt`; `rv23_forced.py.txt`). For each control, the 256 verification is built directly and each block's K̃_c dumped from K4's own `scaled_profile`; then one block's operation is forced to `Span` in the forward (or pivot) pass, the backward pass, or S's formation, at every verification of a full W1 solve.

| | Count |
|---|---|
| models | 16: N05, TWO-SPAN, SKEW6-K1E-12, ALL-ZERO-BODY, THETA-STUB(-COUPLED), HH-FOOL-m40-LOADED, CHARGE-SLENDER, BLOCK-PRESC, GROUP-DIR, G-PRESC-MEMBER, three RF-LARGE-n00010 frames, SPRING-CARRIED, DIRECTIONAL-WELL |
| W1 runs | 68 (16 unforced, 52 forced) |
| forced runs with a Uc refusal recorded / with an S refusal | 34 / 4 (S is refused only where a shift runs) |
| selected, with the unforced worst `compare_honest` ratio, G5a ok, stage identity equal | 50 of 52 |
| `Unresolved(ExactSumSpan)` | 2: HH-FOOL-m40-LOADED's hidden block (its three shifts fail, so no bound remains) |
| verification blocks with a B, checked against the exact norm | 77, 0 below |
| blocks other than the forced one whose Uc, S or B changed | 0 |

Examples: SKEW6-K1E-12 with Uc refused takes B = S = 2^82.18 against the norm 2^78.37; with S refused it takes B = Uc = 2^90.57. RF-LARGE-TREE-n00010-AX: B = S = 2^21.49 or Uc = 2^24.54, norm 2^17.49.

**KF3-UC-SPAN.** See §5.

## 2. The rules

- **A block with data and no bound stops with its refusal** (`certificates:1264-1270`, `verify.rs:1013-1015`). Checked: synthetic F2; forced HH-FOOL; KF3's W2 (the hook) in the suite. Killed mutants: RV23-M3a (the stop becomes `uc`), RV23-M3b (the stop dropped, which would publish with B_b = 0), I19-M7 (a refused block without data stops).
- **The stop outranks `uc`:** in `certificates` (synthetic, both orders; RV23-M4a killed). At `verify_state` it is untested (RV23-N1).
- **A budget stop is never a refusal:** `refusal_kind` maps only `Span` and `Exponent` (`bound.rs:314-320`); `Budget` propagates through every `?`. I19-M6 is killed.
- **Refusals are recorded per block** on success, rejection and a refusal stop (the forced runs record `precision/block:bound:token` on every attempt that refused), but not after a budget stop in the same build (RV23-1).

## 3. The partial-stage identity on all four builds

- **By construction:** each build keeps `current`, and on `Err` adds `total − stages.total()` to it (`adaptive.rs:1140-1151`, called at `adaptive.rs:1396` and `:1845`, `verify.rs:519` and `:1207`). Each stage is a difference of the same counter the total is, over disjoint intervals: I checked every assignment in `build_shared` (`adaptive.rs:1243-1410`), `solve_case_at` (`:1669-1855`, the fallback at most once), `build_verify_shared` (`verify.rs:437-500`) and `verify_state` (`:730-1177`, the two-part charge and bound).
- **My sweep** (`rv23_stage_sweep_small`, `rv23_stage_sweep_kf3`; `probes/sweep.out`): 11 models, 3,611 W1 runs, 3,578 budget-stopped attempts at 128, 256 and 512. Every attempt: own stages = work + K4 sums, and shared stages = shared + verification-shared work. 0 mismatches.
- **RV22-N7's two paths, on H's own model** (`scripts/rv23_h_probe.rs.txt`, `probes/h_probe.out`), with RV22's exact limits (the charged totals are unchanged by KF3):

| RV22's limit | Attempt | Before KF3 (RV22) | At the head |
|---|---|---|---|
| `solve_128` = 2,367,491 (`build_shared` at 256) | 256 | shared 0/28,740 | 28,740/28,740 |
| `into_solve_128` = 2,074,282 (`solve_case_at`, 10 LME in) | 128 | own 576/100,728 | 100,728/100,728 |
| `solve_256` = 5,447,988 | 256 | shared 2,749,799/2,782,745 | 2,782,745/2,782,745 |
| `into_uc` = 6,826,161 | 256 | shared 4,050,070/4,152,466 | 4,152,466/4,152,466 |
| `verify_256`, `into_decide_128` | 128 | equal | equal |

  A further 2,011 limits on the same model (2,000 even points and every segment end ±1): 0 failures, and H's `stages_equal_totals` agrees with my own arithmetic each time.
- The paths also reproduce on FK's own GEN-adapter copy of the model and on nine other controls (`RV23 N7` lines).

## 4. Invariance

- **GEN:** `python3 gen_k4_vectors.py --check` from my archive: all 24 files OK, exit 0 (`suites/gen_check.log`). The PR changes no vector file but adds `kf3.txt` and its `SHA256SUMS` line (`checks/reach_and_scope.txt`), so every earlier record, `outcomes.txt`, `bounds.txt` and `charge.txt` included, is byte-identical and regenerates under GEN's A2 model.
- **GEN's model of A2, independently of I19's tests:** my synthetic runs call GEN's `u_pass_em`, `nl_pass_em` and `uc_from_em_a2` on a factor I did not choose from GEN's vectors. On the second synthetic case they also run `shift_schedule_em(a2=True)`: S and the `shift_form` refusal equal the Rust's. `kf3.txt` exercises only a backward refusal (KF3-UC-SPAN, `refused:span:backward:66`), so the forward-pass case is the first check of GEN's forward branch; it agrees.
- **FK's full suite** (debug, one cargo job, `RUST_TEST_THREADS=2`): 348 lib tests (every control token-equal to GEN, the goldens, KF1's tests), 7 integration files and 6 doc-tests pass, in 12:43, with no warning (`suites/fk_suite.log`).
- **B's frames that did not move** are byte-identical to V-K's pre-KF3 B: all 12 at 100 and 1,000 members and CONT-n10000-AX, charged work included (`checks/b_records.txt`).
- **Stage records of failed attempts** gain their partial stage with totals unchanged; no golden pins them (I19's list). My sweeps confirm the totals.
- **VR's suite** (debug, clean archive) passes 47 of 47 with no warning, its lane tests comparing the committed per-case records byte for byte (`suites/vr_suite.log`).

## 5. KF3-UC-SPAN's honesty

- **The reference is right.** The model is a straight 390-member cantilever along (−1, 12, −12) with rational local axes, fixed at node 0 and loaded at the tip. Its end forces are statics and its nodal displacements Euler–Bernoulli beam theory, both exact in rationals from the model's binary64 inputs. GEN's expectations (the two 300- and 240-digit solves, as 128-bit tokens) match on all 8,983 keys to 2^-128.0 relative, the tokens' own rounding (`scripts/rv23_kf3_closed_form.py.txt`, `checks/kf3_closed_form.out`).
- **So `compare_honest` against GEN is a sound check.** Its δ (the `err` slack, ~2^-734 absolute here) is far below every claim. I19's test passes in my suite run (worst 0.969, G5a ok).
- **The Rust's published rows against the closed form directly** (`rv23_kf3_uc_span_published_rows`, `probes/probe2.out`; `checks/kf3_closed_form.out`), independently of `compare_honest`: selected at 128; 7,415 `relative_verified` rows (claim 2^-64·max(|q|, S)·(1 + 2^-21) plus publication's half ulp), 2 `absolute_verified` rows (`mag.0` and `R.0.2`, whose exact values are 0) and 6 `input_derived` rows (node 0, exactly 0). All 7,423 lie within their claims; the worst uses 0.954 of its claim (`u.80.1`). Magnitude rows are checked as intervals on the exact square.
- **G5a** passes in I19's test (my suite run) and in my forced runs.

## 6. H's parity update

- `stages_equal_totals` (`H/src/k6/w1/staged.rs:212-216`) is plain equality on both sides for every attempt. `builds_completed` is kept as evidence only; `unstaged` and `stages_complete` stay for the attempt line.
- `k6b_w1.rs`: the budget-stopped build asserts `0 < uc < full` and `unstaged == (0, 0)`; one short side, own or shared, now fails; `every_stopped_build_leaves_nothing_unstaged` sweeps segment ends and midpoints.
- No other relaxation remains: `grep` over H's sources, tests and runner and over VR finds `unstaged` only in reporting (`k6b_analysis.py`, a synthetic runner fixture) and the stale doc (RV23-N5).
- **Suites** (debug, clean archive): H `--all-targets` passes lib 25, `k6_alloc`, k6_bin 11, counts 4, models 7, parity 4, staged 3, adapter 2, k6b_bin 3, export 1 and k6b_w1 15, with 0 warnings; the runner passes 47 of 47 (`suites/h_suite.log`, `h_runner.log`).
- My H probe (§3) confirms equality on RV22's six paths and 2,011 more.

## 7. B's records

- **Reproduced** from a clean `git archive` of the head (FK and VR are identical to B's source `e114b23c1`): `vk_scale` in release, the 12 large models regenerated with `cases/gen_vk_cases.py --large` (all 12 match `cases/large_models.sha256`), run directly with the runner's arguments (`b_runs/`, `scripts/rv23_bcmp.py.txt`):
  - **RF-LARGE-CHAIN-n10000-AX:** `record` line byte-identical (2,891 bytes); `start`, `counts`, `w1`, `report`, `rcm`, `binary64` and `summary` equal outside `elapsed_ns` and the heap fields; selected at 128, 10,121,160,213 LME, 103 of 103 R1 rows pass. 9.8 s.
  - **RF-LARGE-TREE-n10000-AX:** `record` line byte-identical (5,491 bytes); the rest equal outside time and heap; `Unresolved Ceiling`, 65,322,991,628 LME, nothing published. 43.3 s, peak RSS 3.6 GB.
  - The heap figures differ by about 100 bytes between runs (the model path's length); K6 owns memory claims.
- **The committed records** (`checks/b_records.txt`, `checks/kf3_records_integrity.txt`): every published row of CHAIN-AX (103), CHAIN-ROT (103), CONT-AX (211 + 4 absolute-range) and CONT-ROT (214 + 1) passes R1, with 0 fail, 0 not-covered and empty C9 S_full. TREE-AX and TREE-ROT have `published_rows: null`, `selected_precision: null`, 0 pass, and 194 `fail` that are VR's `Fail("case …")` marks for a case not selected (`VR/src/lane.rs:314-322`), not comparisons. `SHA256SUMS` verifies 115 of 115, with no unlisted file and no machine path.

## 8. Reach

`checks/reach_and_scope.txt`: `retained_api` is named only by `FK/src/structural.rs`, H and VR (validation); no `Cargo.toml` outside those names H or VR; the PR changes no `Cargo.toml` or `Cargo.lock`; its files are FK's `structural.rs` facade (+3: five types), `K4R/{adaptive,bound,verify,wide_sum}.rs`, `K4T/`, H's `staged.rs` and `k6b_w1.rs`, and the KF3 records. `retained` stays private. KF3 is kernel plus harness only.

## 9. Mutations

One cargo job at a time: each mutant is a clean `git archive` of the head's FK, one substitution, its own release target (deleted afterwards), and the filters `kf3`, the reset test, `method_tests`, `scale_tests`, `verify::tests::`, `bound::tests` and `adaptive::tests::` (56 tests, the goldens and every control token-equal to GEN included). Release rather than debug for time; no kill rests on a debug assertion. Driver `scripts/rv23_mut.py.txt`, results `mutations/mutants.jsonl`, logs `mutations/*.log`.

| Mutant | Change | Result | Killing tests |
|---|---|---|---|
| NONE | the head | 56 of 56 pass | — |
| RV23-M1 | unavailability as 0: a refused Uc enters `certified` as 0 | killed | 5: U1, U5, W1 (`kf3_uc_span…`), W2, the bit-level test |
| RV23-M1b | a refused block's `uc` formed as `Some(0)` | killed | the same 5 |
| RV23-M2 | B = the maximum | killed | U1, both E-CHARGE tests, `golden_work_counts` |
| RV23-M3a | the no-bound stop becomes `uc` | killed | W2, U5 |
| RV23-M3b | the no-bound stop dropped (the block is simply skipped: B_b = 0 would publish) | killed | W2, U5 |
| RV23-M4a | precedence swapped in `certificates` | killed | U5 |
| RV23-M4b | precedence swapped in `verify_state` | **survives** | RV23-N1 |
| RV23-M5 | partial stage unrecorded in `build_verify_shared` | killed | the sweep, the `uc` budget test |
| RV23-M5b | partial stage unrecorded in `solve_case_at` | killed | the sweep |
| RV23-M5c | the partial stage filed under `shift` after `t5` in `verify_state` | survives, equivalent | RV23-N4 |
| RV23-M6 | the shared build's refusals not recorded | killed | W1, W2, W3 |
| RV23-M6b | a refusal not marked on its block (its later rows keep running, so a wrong U could form) | killed | 9 tests |
| RV23-M7 | the reset removed | survives, equivalent | RV23-N4 |
| I19-M5 | a refusal marks every block | killed | U2 (U1's second block) |
| I19-M6 | a budget stop taken as a refusal | killed | U3 |
| I19-M7 | a refused block without data stops | killed | W3, U5, the bit-level test |
| I19-M8 | pre-KF3: any refusal stops | killed | 7 tests |
| I19-M4d | partial stage unrecorded in `verify_state` | killed | the sweep, `every_control_follows_gens_schedule…` |

- **No survivor is an honesty gap.** RV23-M4b changes which of two non-publishing outcomes a doubly-failing verification takes.
- My probes also act as extra kills for the honesty mutants: RV23-M1 and M2 would fail the synthetic and forced B ≥ norm and B = min checks.

## 10. The routing claims

- **KF3-B2** (read only, as asked). At `78f55f927`, `shift_schedule` calls `shifted_factor` (which clones the profile's rows and frees its `work` on return) and then `nl_pass` on its result (`bound.rs:721-723` there). `nl_pass` holds `at` and `bt` while it forms `ct = bt.clone()` (`:292-306` there). So pre-KF3 code holds `at`, `bt` and `ct` on the shifted factor on the same path. KF3 moves the same calls, in the same order, into `shift_run`. I19's reading is right: a K6b omission reached for the first time.
- **KF3-B1.** (`rv23_b1_estimate_ratio_against_p`, `probes/probe2.out`). Ŵ_q/(2^(6−P)·ê), the largest over force and moment rows, at verification states of P = 256, 512 and 1024 built directly:

| Model | P = 256 | P = 512 | P = 1024 | spread |
|---|---|---|---|---|
| RF-LARGE-TREE-n00010-AX | 3.6588e-3 | 3.6582e-3 | 3.6653e-3 | 1.9e-3 |
| RF-LARGE-TREE-n00100-AX | 2.44846e-2 | 2.44833e-2 | 2.44845e-2 | 5.4e-5 |
| RF-LARGE-TREE-n00100-ROT | 3.62e-2 | 1.12e-2 | 1.71e-2 | 0.69 |
| RF-LARGE-CHAIN-n00100-AX | 3.25e-5 | 3.81e-5 | 3.42e-5 | 0.15 |
| N05 | 6.4e-5 | 9.1e-5 | 9.2e-6 | 0.90 |
| SKEW6-K1E-12 | 2.3e-5 | 5.3e-5 | 1.3e-5 | 0.75 |

  - On the TREE-AX family, whose worst row is systematic, the ratio is P-independent to 5e-5 at 100 members, as I19's 4,000-member probe shows to nine digits. That is the case KF3-B1 concerns, and it supports "no rung of the ladder helps".
  - On the other models the worst ratio is rounding noise, 1e-5 to 1e-2 of (b)'s threshold, and varies by up to 10× with P. So "c is a property of the model" is right where (b) binds, not in general. RETURN §12 could say so; it changes nothing in the routing.

## Reviewer, brief and delegation

- **Reviewer.** RV23 is a Type 2 TASK, dispatched by ROOT (HELP_HUMAN) directly through the host's background-subagent mechanism. ROOT is the only return path. I delegated nothing.
- **Brief:** ROOT's RV23 dispatch message (the candidate, the ten review items, the host rules and the output), with `_COMMON.md` and `I8R_K1_RESUME.md:24-50`.
- **Read:** Root `AGENTS.md`; `agents/AGENT_TASK.md`; `T3/TASK_BRIEFS/_COMMON.md`, `I8R_K1_RESUME.md:1-80` and `I19_KF3_IMPLEMENTATION.md`; every KF3 section of `ROOT_RULINGS_V1.md` from "V-K B accepted; KF3 spawned" to "KF3: D accepted; KF3-B1 and KF3-B2 routed; PR to review"; R7 §5.5 (`DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md`); on the KF3 branch `PLAN_CHECKPOINT0.md`, `CHECKPOINT_A.md`, `RETURN.md`, `CHANGE_RECORD.md` and `_run_records/`; `REVIEW/K6B_REVIEW.md` and RV22's probe; the full code diff `78f55f927...b8c55c92e`, and at the head `bound.rs`, `verify.rs` (the shared build and the pass), `adaptive.rs` (the four builds, `verify_precision`, the evidence), `directed.rs`, `wide_sum.rs`, `factor.rs` (the ordering), `seeded.rs`, GEN's A2 functions, `models.rs` (`compare_honest`, `claim_ratio`), H's `staged.rs`, VR's `lane.rs` and runner; `bound.rs` at `78f55f927` for KF3-B2.
- **Ran** (clean `git archive` copies of the head under `<wt>/rv23/` and `<wt>/rv23-mut/`, targets `<wt>/rv23-target`, `<wt>/rv23-mut/probe-target` and one per mutant; `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 4`, `RUST_TEST_THREADS=2`, one cargo job at a time; Python 3.13, standard library only):
  - FK's full suite, H's suite and runner suite, VR's suite, `gen_k4_vectors.py --check`;
  - my probe tests in my copy only (the patch and tests are in `scripts/`), release;
  - `vk_scale` (release) on two 10,000-member frames;
  - 19 mutant runs.
  - The memory guard ran throughout and logged no kill. I20 (KF2) built alongside.
- **Git: read-only** in `<wt>/kf3` (`rev-parse`, `log`, `diff`, `show`, `grep`, `archive`) and `<wt>/numerics` (`status`, `log`). No commit, stash, reset, checkout, fetch or push, no index operation, and no GitHub access.
- **Writes:** this file and `T3/REVIEW/_run_records/kf3_review/**`, uncommitted. My copies and targets are deleted.
- **Not done:** T9 and the both-entry gate (ruled out of scope: no product reach); the other four 10,000-member frames; a `k6_observe` run.

## Records (`T3/REVIEW/_run_records/kf3_review/`)

- `README.txt`, `SHA256SUMS`.
- `scripts/`: the probe patch and tests (my copy only), the Python oracles, the B comparison, the mutation driver, as `.txt`.
- `probes/`: the probe outputs.
- `checks/`: the oracles' verdicts, the reach and scope scan, B's records, the KF3 folder's integrity, the closed form.
- `suites/`: FK, H, the runner, VR, GEN's check.
- `b_runs/`: my two `vk_scale` runs.
- `mutations/`: `mutants.jsonl` and each mutant's log.
- Paths are shown as `<wt>`, `<scratch>`, `<home>` and `<tmp>`.
