# I88 — T3-SI1c repair round 01: RV111 SF-1, N-1 and N-2, with ROOT's rulings 2 and 3

TASK (Type 2) I88, continued by ROOT for the repair round that RR "RV111 passes SI1c; I88's repair round dispatched" names, with rulings 2 and 3 of RR "I88's SI1c verified and ruled; …". The basis is RV111's review, `R/REVIEW_RV111/si1c_01/REVIEW.md` (sha256 `db9677e85ad610dd…`, verified), its probe `evidence/harness/rv111_probe.rs` and its mutant strings (`evidence/harness/rv111_mutants.py`, N06 and N09). My RETURN.md and its `SHA256SUMS` are unchanged; this round's files are sealed in `SHA256SUMS.repair_01`.

Placeholders are as in RETURN.md (`WT`, `NUM`, `P`, `EE`, `RCR`, `R`, `RR`, `VENV`). No machine paths are recorded.

## 1. Head and commits

**New head `f5665f8862959b7a862d7d69543718f1d7240148`** on `codex/piping-t3-si1c-20261007` in `WT/s-i1c`. Its parent chain is the previous head `7f233b2e01` plus two commits. Not pushed (ROOT pushes). The worktree is clean.

| Commit | What | Files (`+`/`−`) |
|---|---|---|
| `fa78c80c32` | Ruling 2 with RV111 N-1 (the N-4 note is appended), RV111 SF-1 (two runner tests), and the runner rename (N-2) | `RCR` +9 −5; `point_path_non_finite_run.rs` +177 −4 |
| `f5665f8862` | Ruling 3 (the split) and the evaluator renames (N-2); tests only | `EE` +30 −23 (all in `mod tests`) |

**Files** (`7f233b2e01` → `f5665f8862` sha256): `EE` `a6766e684119f084…` → `bfa5878cb140c15f…`; `RCR` `ce2281e25cafa40f…` → `c4ed16f9db4483ea…`; `point_path_non_finite_run.rs` `5c1b4e03ea746e5f…` → `4ad39fd8e99ac8b2…`. README and `test_rule_interval.py` are unchanged. **The fence** against main is still exactly the five files of I87 §6.2.

**Formatting.** `cargo fmt --check` on the evaluator crate is clean at the head; `rustfmt --check` on `RCR` shows only the two pre-existing drift hunks, and the runner test file is clean.

## 2. The changes

### 2.1 Ruling 2 with RV111 N-1: the N-4 note is appended (`RCR`)

`run_one_check` now writes the note as `match (non_finite, note)`: no change when the value is finite; N-4's note alone when there was none; and **"<existing>; <N-4 note>"** when the input already carried one. On the runner's paths, the inputs that can carry a note and be non-finite are a bounded solver input ("interval ±b from receipt") and a private library input (its provenance note). A refused input, an invalid or duplicate bound, and an unresolved library reference carry a note but no value, so N-4 never applies to them. Nothing else in `RCR` changes (+9 −5, one hunk and the constant's doc comment).

### 2.2 RV111 SF-1: two N-4 properties pinned (runner tests)

| Test | Pins | RV111 input |
|---|---|---|
| `a_non_finite_input_listed_twice_is_named_once` | A formula whose `input_refs` list `x` twice, with x NaN: exactly one `NonFiniteInput` for `x`, `RULE_EVALUATOR_ERROR`, plain, b = 0 and b = 0.5 | probe `x_listed_twice_nan` |
| `a_non_finite_value_in_a_padded_copy_of_the_declared_unit_is_named_not_unsupplied` | A NaN entered as `" Pa "` for a declared `Pa`. Referenced: one `NonFiniteInput`, no completeness finding, `RULE_EVALUATOR_ERROR`, supplied, unit `Pa`, the note. Unreferenced: the check stays `USER_RULE_CHECKED`, with no finding; the input is supplied with the note | probes `x_nan_padded_unit_referenced`, `u_nan_padded_unit_unreferenced` |

And for ruling 2:

| Test | Pins |
|---|---|
| `the_n4_note_follows_an_existing_note` (new) | A +inf solver input bounded with b = 2: note "interval ±2e0 from receipt; <N-4 note>". A NaN private library input: note "resolved from private library invented_kind:invented_library record invented_record slot invented_slot (value stays in the private library; never embedded in the rule pack); <N-4 note>", with its one `NonFiniteInput` and no `null` |
| `a_non_finite_input_is_named_and_never_bound` (revised) | For the bounded solver input (b = 0.5) the note is now "interval ±5e-1 from receipt; <N-4 note>"; the unbounded cases keep the note alone |

### 2.3 Ruling 3 and RV111 N-2: test names

| Before | After | Why |
|---|---|---|
| `same_dimension_quotients_that_did_not_panic_are_unchanged` | **split:** `same_dimension_quotients_of_finite_operands_are_unchanged` (the largest ratio, a unit mismatch first, a zero divisor) and `a_same_dimension_quotient_over_an_overflowing_divisor_blocks_at_the_multiply` (the case SI1c changed) | ruling 3; assertions moved unchanged |
| `blocks_nan_interpolation_and_step_lookup_arguments_instead_of_panicking` | `nan_forming_interpolation_and_step_arguments_block_at_the_multiply` | N-2: the NaN argument is never formed since SI1c |
| `blocks_generated_nan_table_arguments` | `generated_nan_forming_table_arguments_block_at_their_producer` | N-2, the same |
| runner `a_nan_table_argument_check_blocks` | `a_table_check_over_an_overflowing_argument_blocks_at_the_multiply` | N-2, the same |
| `blocks_overflowing_same_dimension_quotient_instead_of_panicking` | **kept** | its first half still pins the ratio arm, which is reachable; its second half is commented as SI1c's |

## 3. The differential on the changed lines

**Method.** The candidate's dumps were regenerated at `f5665f8862` (a fresh `git archive` tree and target, through the lock) with the same harnesses at the same hashes (RETURN.md §4.1, `harness_sha256.txt`): I79's point and runner harnesses, RV104's evaluator harness and its runner harness in both forms (recorded and all-full), and the SI1c family. The base dumps and the instrumented base's side records are round 0's (main is unchanged), kept gzipped in scratch. Two checks ran under `WT/guard/cargo_job.lock`:
- `si1c_r1_delta.py`: the new candidate dumps against the round-0 candidate dumps (`7f233b2e01`);
- `si1c_compare.py`, with one change: the expected N-4 record now carries "<base note>; <N-4 note>" when the base had a note. All six pass conditions were re-run over every dump.

**Every line other than an N-4 line with an existing note is byte-identical to `7f233b2e01`** (`r1_delta_report.json`, 0 violations):
- **evaluator:** all 11 dumps byte-identical (point, interval and panic files of I79's three sets, RV104's and the SI1c family's);
- **runner, by dump:**

| Dump | Lines | Identical to `7f233b2e01` | Differ | Appended notes in those lines: interval / library |
|---|---|---|---|---|
| I79 fixture rows | 32,820 | 32,820 | 0 | — |
| I79 extreme | 39,438 | 35,793 | 3,645 | 3,969 / 0 |
| RV104 runner, all-full | 216,478 | 192,929 | 23,549 | 15,538 / 11,019 |
| RV104 runner, recorded form | 216,478 | 192,929 | 23,549 (21,511 hash-only, covered by the all-full form) | 145 / 1,893 on its full lines |
| SI1c family | 11,858 | 10,472 | 1,386 | 1,386 / 0 |

In every differing line, every difference is in `bound_inputs[].note`, on a check the instrumented base marks N-4, where `7f233b2e01` wrote N-4's note alone and the new head writes the base's note, "; ", and N-4's note. Nothing else in any record, check or line differs.

**The six pass conditions, re-run at the new head: all hold, 0 violations** (`r1_diff_report.json`). The counts equal round 0's (RETURN.md §4.2) in every class: 51,135 flagged evaluator lines and 11,064 + 90 + 518 flagged runner checks verified; 92,199 + 20,898 + 6,010 N-4 checks verified against the appended-note expectation, each with the base's status and diagnostic; 906,300 interval lines identical; plain = b = 0 on 28,479 + 5,670 + 1,694 lines; every candidate runner line schema-valid (300,594, and the 216,478 all-full lines). The all-full RV104 form still rebuilds the recorded form byte for byte on both sides (base `31dc84473bc88028…`, candidate `3c8b372496789162…`, equal to the new candidate's `rv104_run.txt`).

## 4. Suites at the new head

Through the lock, in the fresh tree (`cargo test --locked --offline` and `-- --list`); the worktree ran the same before the commits.

| Suite | Main `025c1cf326` | `7f233b2e01` | `f5665f8862` |
|---|---|---|---|
| `expression_evaluator` lib / conformance / doc | 57 / 1 / 0 | 64 / 1 / 0 | **65** / 1 / 0 |
| `rule_check_runner` lib / acceptability / interval_bounds / invented_demo / point_path_non_finite_run / rule_interval_cases / doc | 14 / 4 / 11 / 3 / 2 / 1 / 0 | … / 8 / … | … / **11** / … |
| `rule_pack_document` | 6 / 1 / 3 / 0 | the same | the same |
| `test_rule_interval.py` | 193 | 193 | 193 (own checked binaries, built through the lock, and run under the lock) |

**Test names against `7f233b2e01`** (`test_name_delta_r1.txt`): removed `same_dimension_quotients_that_did_not_panic_are_unchanged`, `blocks_nan_interpolation_and_step_lookup_arguments_instead_of_panicking`, `blocks_generated_nan_table_arguments` and the runner's `a_nan_table_argument_check_blocks`; added the two split tests, the three renamed ones and the three new runner tests. Against main the delta is in the same file. Every test passes.

## 5. Mutants

**57 mutants at `f5665f8862`: 40 killed, 17 survive, and the 17 are exactly round 0's equivalents** (`r1_mutants.json`, `mutant_logs_summary_r1.txt`; kill rule as in RETURN.md §6; unmutated: 65 and 11 pass, three times).

- **RV111's N06** (the one-finding-per-input check removed, RV111's patch string verbatim): **killed**, by `a_non_finite_input_listed_twice_is_named_once` alone.
- **RV111's N09** (`trim` dropped from the raw-unit test, verbatim): **killed**, by `a_non_finite_value_in_a_padded_copy_of_the_declared_unit_is_named_not_unsupplied` alone.
- **Ruling 2's:** N11 (N-4's note replaces an existing note, the old behaviour) and N12 (N-4's note put first): **killed**, each by `the_n4_note_follows_an_existing_note` and `a_non_finite_input_is_named_and_never_bound`.
- **Round 0's 36 killed mutants are killed again:** D1–D10, E1, E6, E8, E9, N1–N10, and SI1b's Q1, Q3, Q4, R1–R6. N1, N2, N4, N7 and N10 are now also killed by the new SF-1 tests.
- **The 17 equivalent mutants** are round 0's set (RETURN.md §6): E2–E5, E7, and I79's Q2, S1, S2, I1, I2 survive with 0 of 808,318 dump lines differing, re-run at the new head; I79's T1, T2 and RV104's S1, S2, I1, I2, N1 survive the suites, and their dump evidence is carried from round 0, because `EE`'s non-test code is byte-identical to `7f233b2e01` (sha256 `2eecabcc3ab399f4…`, 2,676 lines; `ee_non_test_identity.txt`).

**A run note.** The lock was held for long stretches by RV109's jobs. After 29 mutants I stopped my own mutant run to put the runner mutants first. Stopping its waiting `lockf` made one entry (I79's T1) a false "killed" (exit −15); I discarded it, restored the mutant tree from the candidate (checked identical), and re-ran T1 with the rest. No other job was touched.

## 6. Host

- Every cargo command went through `WT/tools/t3_cargo.sh` with `--locked --offline` (fmt without them); the analysis scripts and pytest ran under `WT/guard/cargo_job.lock`. No test binary was run directly. My jobs waited behind RV113's and I85's; I killed nothing (`cargo_jobs_i88_r1.txt`).
- **A correction to round 0's host record:** round 0's Python analysis (`si1c_compare.py`, `si1c_status_tally.py`, the dump hashing, about 30 s to 2 min each) ran outside the lock, before ROOT's restatement that every heavy job goes under it. This round's ran under it.
- One wait per job, each ending when its process was gone; none is running.
- Git: two commits on the branch; no other Git writes; nothing pushed; nothing committed in NUM.
- Scratch in `WT/scratch/i88_si1c/`, `TMPDIR` there. The trees `trees/{cand,mut}` and the targets `WT/targets/i88-si1c-{wt,cand,mut,bins}` are deleted, and so are the decompressed working copies and the survivors' equivalence dumps after their counts were recorded; the new candidate dumps are kept gzipped beside round 0's (`dumps/cand`, `dumps/cand_r0`).

## 7. For ROOT

Nothing new to rule on. Ruling 2's two note kinds are pinned by assertions; SF-1's two properties are pinned and RV111's N06 and N09 are killed (§5); ruling 3 and N-2's renames are as in §2.3.

## 8. Run records (`_run_records/repair_01/`)

- `rcr_r1.diff` (the `RCR` change of this round);
- `r1_delta_report.json`, `r1_diff_report.json`, `dump_sha256_r1.txt`;
- `suites/`: the suite and list logs, `test_name_delta_r1.txt`, `pytest_cand_r1.log`, `fmt_cand_ee_r1.log`, `rustfmt_check_r1.txt`, the worktree logs;
- `mutants/`: `r1_mutants.json`, `mutant_logs_summary_r1.txt`, `ee_non_test_identity.txt`;
- `harness/`: `si1c_r1_delta.py`, `r1_analysis.sh`, `r1_mutants_part2.sh`, `r1_mutant_summary.py`, and the changed `si1c_compare.py` and `si1c_mutants.py`;
- `host/`: `driver_r1.log`, `cargo_jobs_i88_r1.txt`.
