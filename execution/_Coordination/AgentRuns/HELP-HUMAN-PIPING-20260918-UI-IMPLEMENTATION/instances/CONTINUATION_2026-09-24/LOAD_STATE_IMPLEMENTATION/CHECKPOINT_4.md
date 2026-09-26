# Checkpoint 4 — checkpoint-3 review dispositions (SF-1, N-1 to N-5)

- **Author:** the session-3 T1 WORKING_ITEMS manager, parent HELP_HUMAN (ROOT). The earlier manager and its workers are no longer active; this session re-established ownership. The names in `OWNERSHIP.md` are historical.
- **Checkout:** branch `codex/piping-load-states-20260925`, from `a68326039`. That commit is ROOT's merge of main (PR905, PR916) into the CP3 branch.
- **Paths:** WORKING_ROOT-relative. Machine records are in `_run_records/session3/`.
- **Status:** completed execution, not acceptance. No M10, M16 or M29 finding closes. `load-reference-source-1` and profile `resolved_straight_load_state_source_v1` stay ROOT-reserved and inactive.

The wire of record is `CP2_WIRE.md`, ADDENDUM_1, ADDENDUM_2, `CP3_WIRE_ADDENDUM.md` and the new `CP4_WIRE_ADDENDUM.md`.

## 1. SF-1 — a selected join that cannot finalize

ROOT's selection is to fall back to ordinary publication. `CP4_WIRE_ADDENDUM.md` §1 defines the rules, and they apply to 0.4.0 only. This addendum corrects `CHECKPOINT_3.md` line 43 and `CP2_WIRE_ADDENDUM_2.md` line 102; those records are not rewritten.

**Code** (commit `6235f6b43`):

- **`core/product_physics/src/lib.rs`.**
  - `SourceRecoveryBudget` carries two new fields: `load_state_join_withheld` (input) and `load_state_join_failure` (output).
  - `run_linear_static_preview_captured` is now a wrapper around `run_linear_static_preview_captured_once`. For a captured 0.4.0 request whose join failure is recorded, it republishes once in a fresh ledger with the join withheld.
  - In `solve_load_case`, a 0.4.0 attempt first passes the replay reservation, or is declined when the join is withheld.
  - A selected case's finalization failure is recorded, and so is an invocation receipt failure.
  - Nothing else in `lib.rs` changed. In particular no ordinary-route summary, maxima or reaction publication changed; ROOT has reserved those for T0R.
- **`src/source_recovery.rs`.** Two methods, `SelectedSourceRecovery::reserve_captured_replay` and `decline_withheld`, construct the refusals as `RecoveryFailure` values. The existing unavailable path, its receipts and its diagnostics are therefore reused unchanged.
- **`src/source_budget_tests.rs`.** One struct literal gained `..Default::default()` for the new fields. No assertion changed.

**Measured basis for the reservation** (scratch measurement, not committed). On the committed `eigen_motion` witness, the replay stage charged 3,359,828 against a live charge of 3,360,029. With one, two and three of the reviewer's extra loads, replay was 4,061,046 against 4,061,151, 4,093,244 against 4,093,286, and 4,527,608 against 4,527,629. Replay is therefore bounded by the live charge. A complete finalization costs about 2c + 0.6–0.7M (7,344,183 total at c = 3,360,029).

**Tests** (`src/source_receipt/load_state_fallback_tests.rs`, five tests):

| Test | Pins |
|---|---|
| `reviewer_scenario_a_budget_cliff_…` | P9 with 1–3 extra loads, both modes. The value route returns `Ok` with `load-reference-1`, no receipt, `MECHANICS_SOLVED` and no blocking diagnostic. Quality is `sensitive`. The rows are bit-identical to the uncaptured ordinary route. The record is `not_joined`, the attempt is `unavailable`, and the refusal is "captured replay reservation" / `Exact(Budget)`. |
| `reviewer_scenario_b_composite_…` | P3, both modes. The same ordinary-publication checks for both cases. The first case's attempt is declined as "invocation join withheld", naming the invocation-receipt cause. The pressure case keeps its own refusal. |
| `a_selected_case_whose_own_finalization_fails_after_the_reservation_falls_back` | A private 7.0M case limit: the join passes the reservation, but its full finalization does not fit. The per-case failure is recorded, and the fallback republishes ordinarily in a separate ledger (`attempts == 1`, `charged == failed_charged`). |
| `the_committed_witness_still_selects_and_finalizes_within_the_public_limit` | `eigen_motion` still gives `load-reference-source-1`, qualified. |
| `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` | A characterization only: P12 still returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`. It is the open finding for T3 and has not changed. |

**Mutants** (`_run_records/session3/cp4_sf1_mutations.{py,log}`, in place, restored and sha256-verified). All six are killed:
- the reservation removed;
- the reservation weakened to `c ≤ L`;
- the fallback republication removed;
- a withheld attempt still selected;
- the per-case failure not recorded;
- the invocation failure not recorded.

**Pre-0.4 unchanged.** All 34 committed producer raw fixtures regenerate from their requests with the SF-1 producer:
- 28 byte-identical: load_reference, load_reference_source and physics_source;
- 6 JSON-equal: the source_blocks raws, which are stored with sorted keys.

See `_run_records/session3/cp4_regen_compare.{py,log}`. The inherited physics-source-1 composite behaviour did not need to change, so ROOT was not asked to change it.

## 2. N-1 to N-5

- **N-1** (commit `dd4286ab1`). The non-implementer TASK CP4_JOIN_TESTS wrote `src/source_receipt/load_state_join_tests.rs` (sha256 `a934c640c7405ccbc7a51e7367f6cde2f5e311f04c5fa0b50fdfcf8d25768a59`).
  - Its eight tests replace probes P1, P2, P4, P5, P6, P7 and P11, and add a motion-ownership control. P3, P9 and P12 are pinned by the SF-1 tests.
  - On a `git archive` scratch copy, using the reviewer's anchors verbatim:
    - K5, K7, K8, K9 and K10 are now killed by maintained tests;
    - K1–K4 and K6 remain killed;
    - K11 and K12 still survive; the reviewer judged them equivalent in practice, and they were not targets.
  - Return: `CP4_JOIN_TESTS/RETURN.md`. Disposition of its notes:
    - the optional `#[cfg(test)]` hook is not added: the K9 test guards its mirrored route with a byte-equal receipt control;
    - the optional K12 test is deferred.
- **N-2 and N-3** (commit `5a7baefef`). The non-implementer TASK CP4_READERS did this work; its return is `CP4_READERS/RETURN.md`.
  - **N-2.** The Python reader admits any finite JSON integer as a number, matching Rust's `as_f64`, and compares binary64 values. `U64_MAX` now bounds indices only. An integer that overflows binary64 is refused in Python (`NUMBER_INVALID`) and at the Rust text boundary; this is declared language-specific, like the `NONFINITE-*` cases.
  - **N-3.** Both readers now require, in the same order:
    - `upper_index == lower_index + 1`;
    - `start_k < end_k` for `integration_interval` and `start_k == end_k` for `interpolation_sample` (existing code `SOURCE_LOAD_REFERENCE_LAW_SEGMENT`);
    - no duplicate entry within one list (new code `SOURCE_LOAD_REFERENCE_LAW_SEGMENT_DUPLICATE`, identical in both). Consumed and consulted lists may share entries.
    - No other tightening. The four frozen envelopes are still accepted.
  - **Cases.** 27 were appended to the shared mutation file. Over 226 IDs there are 0 undeclared differences, and no case is accepted by one language and rejected by the other. 10 of 10 reader mutants are killed.
  - **ROOT disposition of the TASK's design question.** The readers are *not* tightened to the canonical number profile. They admit integral magnitudes above 2^53 − 1, which the checked-JSON carrier layer refuses in both languages (`UNSAFE_JSON_NUMBER`, `CHECKED-JSON-UNSAFE-INTEGER`, `CHECKED-JSON-NUMBER-OUTSIDE-PROFILE`).
    - This is an explicit, pinned boundary, not a hole: accepted by the evidence reader, refused at canonical materialization. It already applied to the `1e+20` form before CP4.
    - The new `carrier_rust` and `carrier_python` case keys are the maintained evidence of that boundary.
    - No physical bound is implied either way.
    - If a later tranche needs such values carried, that is a transport-profile change for T3 or T6, not a reader change.
- **N-4:** §3.
- **N-5** (commit `13f752115`).
  - `rustfmt` 1.8.0-stable was run over `core/reporting/result_export/src/load_reference.rs`, `src/semantic_contract.rs` and `tests/load_reference_contract.rs`, and all three are now clean.
  - Two files are token-equal to their previous bytes apart from whitespace and trailing commas.
  - In `semantic_contract.rs`, rustfmt also reordered two `pub use` items, one of which was already there; this is formatting only.
  - result_export: 64/64.
- **rustfmt for SF-1's own edits.**
  - `source_recovery.rs` and both new test modules are rustfmt-clean.
  - `lib.rs` still has its pre-existing rustfmt differences. They went from 47 to 48, because one SF-1 block sits inside a pre-existing unformatted block and splits it. Every new standalone block is in rustfmt style.

## 3. N-4 record corrections

These correct earlier records without rewriting them.

| Record statement | Correct fact |
|---|---|
| `CHECKPOINT_3.md` line 122 cites `CP3_READERS/RETURN.md` as `75f6f0a0…` | The committed, frozen file is `57f12bcb48048cb634ffab6d9f7df450bcb9106cae1aa1a0fb0f72030e5dea2a` |
| `CHECKPOINT_3.md` line 293 ("Record rule") says `CP3_READERS/RETURN.md` lines 178–179 still name the interpreter and target by machine path | They do not at `485cc2ed0`: they point to `RR/environment.json`. ROOT edited this TASK-owned return to remove the two machine paths before committing `485cc2ed0`. `75f6f0a0…` is presumably the pre-edit hash; the pre-edit bytes were never committed, so this session cannot verify it. No earlier record said who made the edit or when |
| `_run_records/session2/CHECKS_CHECKPOINT_3.json` line 164 cites `CP3_ROOT_SELECTIONS.json` as `e6b20097…` | The committed file is `767f0e99e615e1e5cfc5b54cec642f17d0517f640b82266c4cb2c66115162090` |
| `CHECKPOINT_3.md` lines 236–237: all 17 mutants are killed | True, but the manager's run used pre-final bytes, which was not disclosed. Of the six files in `cp3_mutation_prehash.txt`, four differ from the candidate `485cc2ed0`: `source_recovery.rs`, `source_receipt.rs`, `source_receipt/composite.rs` and `case_state/resolve.rs`. `lib.rs` and `case_state/input.rs` match. The CP3 reviewer reran all 17 with verbatim anchors on the frozen bytes, and all 17 are killed (`REVIEW_CHECKPOINT_3/_run_records/rerun_implementer_mutants.log`) |

This session verified every hash above against the files and `git show 485cc2ed0`.

**This session's own record edit.** After dispatch, the manager replaced the absolute machine paths (worktree, venv and target locations) in `TASK_BRIEFS/CP4_READERS.md` and `CP4_JOIN_TESTS.md` with placeholders, in the commit after `3cc984054`. No other content changed. The TASKs executed the bytes committed at `c0ef4a8e0`, and those remain in Git history. The concrete locations are given in the spawn requests, not in committed records.

## 4. Checks

Integrated head `5a7baefef`. The toolchain is `cargo +1.97.1 --locked --offline -j 2`, and the Python venv is the session's DEC-025 environment. Logs are in `_run_records/session3/`.

| Check | Result | Log |
|---|---|---|
| `core/product_physics`, whole crate | 415 passed, 0 failed, 1 pre-existing ignored. That is CP3's 402, plus 5 SF-1 tests and 8 N-1 tests; lib 324. Warnings are identical by name to the CP3 final gate. | `cp4_integrated_product_physics_all_tests.log`; at the SF-1 commit, 407 in `cp4_sf1_product_physics_all_tests.log` |
| `core/reporting/result_export` | 64/64, run twice: after N-5 and at the integrated head | CP4_READERS `final_cargo_test.log` |
| `core/runner/headless` | 64/64, at SF-1 and again at the integrated head | `cp4_dependent_headless.log`, `cp4_integrated_dependent_headless.log` |
| `core/model_operations/operation_applier`; `core/loads/self_weight_wasm` | 176 and 14, at SF-1. Neither depends on result_export, and neither is affected by the later test-only and reader commits. | `cp4_dependent_*.log` |
| pytest: the load-reference readers and schema, and the stress-neutral export, physics-source and precision suites | 957 passed, 1 skipped (the optional parity recorder) | `cp4_integrated_python_named.log` |
| Producer raw regeneration | 34 of 34 unchanged (28 byte-identical, 6 JSON-equal) | `cp4_regen_compare.{py,log}` |
| Mutants | SF-1 6/6, N-1 K5 and K7–K10 killed (K1–K4 and K6 still killed), readers 10/10 | `cp4_sf1_mutations.log`, `CP4_JOIN_TESTS/`, `CP4_READERS/` |
| rustfmt 1.8.0-stable | clean on every new file and on `source_recovery.rs`. `lib.rs` has its pre-existing differences (§2) | — |

**Not run:**
- the full piping pytest sweep;
- the DEC-025 five-surface sweep;
- desktop, native and browser checks;
- hosted CI.

This slice is not a mergeable PR candidate by itself. The T1 PR qualification in `T1_PLAN.md` §2 WP7 covers those checks.

## 5. Review

(filled after the independent review)
