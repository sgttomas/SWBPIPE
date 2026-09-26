# Backcheck: repairs to the checkpoint-4 review findings

**Verdict: CLEAR.** Every repair is implemented as `CP4_REVIEW_DISPOSITION.md` and ROOT's rulings describe, and its consequences hold under my probes. Nothing else changed. I have two notes. Neither needs a repair before merge: one is a test that does not discriminate, and one is a record statement left implicit.

## Scope and identity

- **Reviewer.** The same fresh-context, non-author TASK that wrote `RETURN.md`, at the T1 manager's request. I wrote none of the reviewed bytes, delegated nothing and made no Git writes.
- **Candidate.** `4e50f4a75`, which was HEAD when I started and when I finished. I reviewed the diff `14a74b793..4e50f4a75`, which has two commits:
  - `520af7abe`: records;
  - `4e50f4a75`: code.
- **Paths.** Paths are relative to WORKING_ROOT = `projects/chirality-piping/`, and `LSI` is the instance folder. Logs are in `_run_records/backcheck/`. Probes and mutants ran on one `git archive 4e50f4a75` scratch copy; its `product_physics/src` was checked hash-equal to the checkout before the mutant runs.

## Nothing else changed

The diff touches exactly these files:
- three code files: `lib.rs`, `source_recovery.rs` and `load_state_fallback_tests.rs`;
- `CHECKPOINT_4.md`, where only §5 was filled;
- the new `CP4_REVIEW_DISPOSITION.md` and five `session3/cp4r_*` records;
- my own `REVIEW_CHECKPOINT_4/` files.

My files were committed unchanged. Their mtimes are from my writes and predate `520af7abe`, and the tree is clean. The readers, result_export, schemas and fixtures are untouched. No earlier record was rewritten.

The code change in `lib.rs` consists of three things:
- the comments;
- the ledger copy in `withholding_load_state_join`;
- in `solve_load_case`, the 0.4.0 arm, which is now `reserve_captured_replay(..).and_then(withheld ? decline : Ok)`.

In `source_recovery.rs`, only the doc comment changed; the screen arithmetic is byte-identical. The first run's behaviour is unchanged, because `withheld` is `None` there.

## Each repair

| Finding | Repair | Confirmed by |
|---|---|---|
| **SF-1R** (ROOT (b): the screen is not a guarantee) | Code comments now describe a screen. `CHECKPOINT_4` §1 and ADDENDUM §1.1 are corrected through the disposition record, which quotes each statement, as is ROOT's superseded wording. New public-limit test for the RY input, in both modes | Code reading. The new test passes. Probe `ry`: `load-reference-1`, rows bit-equal to the typed route, "invocation join withheld … captured source replay". Mutants SF1-M2, M3 and M4 each fail this test |
| **N-1** | `cp4r_sf1_mutations.py`, anchored to the committed bytes (every anchor must occur exactly once), plus an M1 rerun | I reran the committed script on the scratch copy: **8/8 killed**, identical to `cp4r_sf1_mutations.log` apart from M1, whose committed rerun log I reproduce. The first M1 line in the committed log came from an earlier anchor that is not in the committed `.py`. The disposition discloses this |
| **N-2** (ROOT: same ledger) | The republication copies `charged`, `failed_charged`, `publication_charged`, `rejected`, `attempts` and the failure cause | Probes over 7 copy counts at the public limits, and 13 private invocation limits on each of 3 inputs (`probes.log`). Every fallback published `load-reference-1` with `MECHANICS_SOLVED`, no blocking diagnostic and rows bit-equal to the typed route, and `charged ≤ invocation_limit`. There was no `debit` assertion panic, even when the republication started with about 1k units remaining (copies 8–12). After exhaustion, the attempts are refused for budget at "source closure", as the disposition states |
| **N-3** | The screen is checked before the withheld branch | Probes `mixed` (heavy case second) and `heavy_first` (heavy case first). The heavy case reports "captured replay reservation"; the selectable case reports "invocation join withheld". N3-M7 is killed by the new test. With a reduced remaining budget, the screen can now refuse in the republication a case that passed it in the first run (for example `ry` at private limits of 14M and 15M). That refusal is a true statement of the republication's own ledger |
| **N-4** | Records in the disposition: the `CP4_REVIEW.md` edit and the pytest command | They match commit `ad6da6880` and my five-file run |

**Consequences I checked:**
- **Pre-0.4 unchanged.** Pre-0.4 is byte-identical: the 120-run differential at `4e50f4a75` equals both `a68326039` and `14a74b793` on every run (`pre04_differential_4e50f4a75.json`).
- **Committed raws unchanged.** 34/34 regenerate, and the log is byte-identical to `cp4r_regen_compare.log`.
- **Test counts.** product_physics: 418 passed, 1 ignored (lib 327).
- **rustfmt.** `source_recovery.rs` and the test module are clean. `lib.rs` still has 48 own hunks, so the new blocks add none.

## Notes

| # | Location | Observation | Suggestion (optional) |
|---|---|---|---|
| B-1 | `load_state_fallback_tests.rs`, `a_fallback_after_invocation_limit_exhaustion_stays_within_the_invocation_limit` | **This test does not discriminate the N-2 repair.** `budget` is replaced by the republication ledger. With the ledger reset (N2-M8), the republication charges about 33.6M, which is still ≤ 64M, so the test passes (`reviewer_mutations.log`, B1: SURVIVED). The repair is killed only by the private-limit test's `attempts == 2` and `charged > per_case_limit` (B2 and N2-M8 are killed there). The disposition does not claim otherwise. | Assert `budget.attempts == 20` (both runs), or assert that the republication's diagnostics show budget refusals, so the ten-copy test pins the shared ledger itself. |
| B-2 | `CP4_REVIEW_DISPOSITION.md` N-2 | **Two earlier statements are not quoted.** The N-2 change also falsifies ADDENDUM §1.2, "runs the same captured invocation again, in a fresh ledger with the same limits … every retained-source attempt still runs, so each case keeps its own attempt facts", and `CHECKPOINT_4.md` §1 ("in a fresh ledger"; test table "`attempts == 1`, `charged == failed_charged`", "separate ledger"). The disposition says "previously it used a fresh ledger" and states the new consequence, but it does not quote these statements as corrected, as it does for SF-1R. | Quote them in the next record. |

## Limits

- **Not rerun:** the readers, result_export, pytest and the dependent crates, all untouched by this diff; and the CP4 review's own mutants R1–R7.
- **Not reached:** a republication starting with exactly zero remaining budget (the closest was about 1k).
- **Cleanup:** the scratch copy and the cargo target were deleted. I wrote only inside `LSI/REVIEW_CHECKPOINT_4/`.
- **Status:** this backcheck is not acceptance, and it closes no M10, M16 or M29 finding.
