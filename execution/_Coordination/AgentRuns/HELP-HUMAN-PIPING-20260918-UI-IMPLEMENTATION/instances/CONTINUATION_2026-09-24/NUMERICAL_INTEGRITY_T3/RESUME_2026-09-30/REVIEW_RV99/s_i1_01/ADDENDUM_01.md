# RV99 addendum 01: confirmation of S-I1's repair round

**Reviewer:** RV99, TASK (Type 2), the same reviewer as `REVIEW.md`, continued by ROOT (HELP_HUMAN, Agent 0) for the same-reviewer confirmation. ROOT is the return path. No descendants. I wrote none of the repairs.

**Placeholders:** as in `REVIEW.md` (`WT`, `NUM`, `P`, `T`, `R`, `RR`, D2, `VENV`). No machine paths are recorded here or in `evidence/round1/`.

**Basis read:**
- ROOT's continuation message;
- RR "RV99 passes S-I1 with three SHOULD-FIX; the repair round" (RR read at sha256 `ee794f44…`);
- I73's `R/I73/s_i1_01/REPAIR_01.md` (sha256 `2d481f8b…`, verified), with `SHA256SUMS.repair_01` (29 of 29 OK);
- my own `REVIEW.md` (`48cfdbc0…`, unchanged);
- D2 revision 5b.3 (unchanged).

**Candidate:** `8f956d399a..c26ecabbc1` on `codex/piping-s-i1-20261005`:
- `ea7c0881f4`: ROOT's description-only edit for N-3;
- `c26ecabbc1`: I73's repair round, 7 files, +842 / −60.

The whole slice `c1bfc460fc..c26ecabbc1` is still the same 8 files. Read with `GIT_OPTIONAL_LOCKS=0`; nothing written to `WT/s-i1`.

**Copies and targets.** I made fresh `git archive` copies (`P/` without `P/execution/`), each with its own cargo target, so no copy can reuse another's stale binary (the artifact I73 reports):
- `c26ecabbc1` in `WT/rv99/`, target `cand`;
- `c1bfc460fc` in `WT/rv99/base/`, target `base`;
- a second copy of `c26ecabbc1` in `WT/rv99/mut/`, target `mut`, used only for mutants.

After all runs, each copy equals its commit file for file (2,955 / 2,950 / 2,955 files), apart from my scratch test files (`rv99_harness.rs`, `rv99_diff.rs`, `rv99_probes.rs`). I then deleted the copies and targets.

## Verdict: **CONFIRMED.** The repair round resolves S-1, S-2, S-3, N-4 and N-5, and closes N-1.

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE (new) | 1 |

| Item | Result |
|---|---|
| **S-1** (`=`/`≠` on identical ranges) | **Resolved.** Shared cases `equal_identical_ranges` and `not_equal_identical_ranges` (both U, negative controls), and two assertions in the evaluator test. **V3 is killed** in my rerun: the evaluator suite fails `comparisons_are_three_valued_with_strict_ends`, and the runner suite fails `rust_interval_mode_matches_every_shared_case`. |
| **S-2** (divisor range ending at zero) | **Resolved.** Shared case `divide_by_zero_end_range` (`0/(−abs(z)) ≤ 1`, z = 0 ± 1: U with `divide_by_zero_range`, negative control), and the same case in the evaluator test. **V9 is killed** by the evaluator's own suite (`division_by_a_range_containing_zero_is_indeterminate`) and by the shared cases. |
| **S-3** (Python fails open) | **Resolved.** My round-0 probe, rerun on the repaired module (`evidence/round1/probes/python_reference_invalid_inputs_after.txt`): a NaN, negative or infinite bound now reads U with `non_finite_enclosure` on x (was T). An inverted enclosure blocks with `InvalidReference` x, and a NaN or infinite end with `NonFiniteInput` x (were T). These are Rust's evaluator-level results; the reading that Python mirrors the evaluator, not the runner, is right, since Python has no runner and my remedy allowed either. Parity holds on the 9 new case-file cases and on 240 cases of my own with invalid bounds and explicit overlays (below). My mutants PW1–PW4 are all killed. |
| **N-4** (downgraded check keeps its code) | **Resolved, interval path only.** The guard is inside `finish_interval_check`, which only interval checks reach. My probes: undeclared all-pass and all-fail checks now carry `RULE_INPUTS_INCOMPLETE`, `diagnostic_codes: []` and `STATUS_NOT_DECLARED` only, the same as the point path's downgraded check (without `computed_value`, as for every interval check). An undeclared U keeps `RULE_RESULT_INDETERMINATE` with `STATUS_NOT_DECLARED`. A declared T, and empty `result_statuses`, still carry `RULE_INTERVAL_ALL_PASS`. This is consistent with D2 §4.11.5, which pairs ALL_PASS and ALL_FAIL with their own statuses and INDETERMINATE with `RULE_INPUTS_INCOMPLETE`. My mutants W1 and W2 are killed. |
| **N-5** (duplicate bounds) | **Resolved, bounded entry only.** Duplicates for a bound solver input are a blocking completeness `RULE_EVALUATOR_ERROR`, and the input is unsupplied, for [1, 60], [60, 1], [1, 1] and [0, 0] alike. Duplicates on an unbound id or on a user input change nothing: the output is byte-identical to `run_rule_checks`. My mutants W3 (last wins), W4 (equal duplicates allowed) and W5 (first wins) are killed. |
| **N-1** (lower conversion end) | **Closed.** Both ends of the psi → MPa enclosure are now pinned bit for bit; **V11 is killed** (`unit_normalization_steps_each_end_outward`). |
| **N-3** (schema description) | **Confirmed.** `ea7c0881f4` changes only `/$defs/RunFinding/properties/severity`'s description: with every description removed, the schema is identical at `8f956d399a`, `ea7c0881f4` and `c26ecabbc1`. The new text ("one of 'blocking', 'warning' or 'info' (completeness findings carry the completeness checker's FindingSeverity)", plus the interval severities) is accurate. Every `severity` the runner writes is `"blocking"`, `"info"` or `"warning"` (interval outcomes), or `severity_str`, which covers the checker's three values. `c26ecabbc1` does not touch the schema. |
| **Point mode** | **Byte-identical** to base over my differential (below). |
| **Parity** | **Holds** on all 94 case-file cases and 30,515 of my own (below). |
| **Fence and locks** | **Unchanged.** The 8 slice files only. No manifest, lock or dependency is in the diff. The src-tauri lock is `4de71f1b…`, and the rules crates' `Cargo.toml` and `Cargo.lock` sha256 are identical at both commits. |

## The new note

**N-7 (NOTE, information for S-I2): a duplicated b = 0 also blocks.**
- **What happens:** with `[b = 0, b = 0]` for one solver input, the check is `RULE_INPUTS_INCOMPLETE` ("duplicate absolute bounds"), where a single b = 0 is the point path.
- **Why it is right:** this is the ruling's literal scope ("duplicate bounds for one input id"), and it is consistent with the invalid-bound route. It never makes a pass.
- **What stays the same:** `run_rule_checks`, and every bound list with one entry per id, are unchanged (differential below).
- **For S-I2:** emit one bound per input, b = 0 included.

Nothing else remains open from my review:
- N-2 (confirmed by ROOT) and N-6 (to S-I2) are ROOT's;
- the reading in REPAIR_01 §2 (S-3) is accepted above.

## 1. Soundness and parity, rerun on `c26ecabbc1`

Same harness, checker and oracles as `REVIEW.md` §1.4. Two changes, both in `evidence/round1/tools/`:
- **The harness** now accepts the case file's explicit `enclosure_bits` (a pair or null), and passes any b ≠ 0, NaN included, through the production `enclosure_from_bound`.
- **My enclosure transcription** returns none for a negative or non-finite b, as Rust does.

The run script is `run_all.sh`.

| Run | Evaluator cases | Point samples | Exact samples | T / F | Violations |
|---|---|---|---|---|---|
| Pass 1 (seed 990099), with 603 runner cases | 3,227 | 247,142 (+ 11,448 runner) | 39,290 | 426 / 411 | 0 |
| Dense re-sampling of every decided pass-1 case, with 341 runner cases | 1,080 | 2,394,299 (+ 707,783 runner) | 59,751 | 426 / 411 | 0 |
| Seeds 1–4, with 6,012 runner cases | 26,908 | 2,227,557 (+ 112,882 runner) | 348,897 | 2,893 / 2,901 | 0 |
| The 94 shared cases (I73's 83 + 11 new), re-sampled by me | 94 | 5,253 | 2,181 | 10 / 8 | 0 |
| My targeted corners (zero-end divisors; row-end table arguments) | 140 | 904 | 210 | 35 / 35 | 0 |
| **New:** my 240 invalid or explicit overlays on pass-1 formulas (NaN, negative, −∞ and +∞ bounds; valid, null, inverted, NaN-end and ∞-end enclosures; `gen_invalid.py`) | 240 | 19,666 | 361 | 4 / 8 | 0 |

- **Soundness:** no T or F disagrees with the point path or exact arithmetic, and every enclosure contains every point and exact value.
- **Straddles:** every straddling box reads U: 8,039 evaluator boxes, plus 15 blocked by an invalid overlay. All 1,548 runner straddles read `RULE_INPUTS_INCOMPLETE`.
- **Ruling 3:** neither interval mode nor any bounded check panicked, on the same 150 + 16 point-path panic boxes as round 0 (`straddles_and_panics_r1.txt`).
- **Parity:** Rust and Python agree on truth, enclosure bits, notes and finding codes and subjects for all 30,609 evaluator cases (94 shared, 30,515 mine), including the 9 new invalid-input shared cases and my 240.
- **The checker can see the S-3 mutants:** on my 240 cases it flags PW1, PW2 and PW3 (`evidence/round1/mutants/pw_on_invalid_cases.txt`). PW4 (an overlay on a missing value accepted) is outside them, and I73's new test kills it.

## 2. Point mode: my differential, rerun

Same input file as round 0 (`rv99_diff_cases.json`, sha256 `f81f3bec…`: 2,888 packs, 63,086 runs over both committed packs, 10 demo variants and all 1,863 run-fixture rows). Fresh base and candidate builds (`evidence/round1/differential/`):

| Dump | Base `c1bfc460fc` | Candidate `c26ecabbc1` |
|---|---|---|
| `run_rule_checks` | `a2bbfa3c…` | `a2bbfa3c…` |
| `run_rule_checks_with_bounds(&[])` | — | `a2bbfa3c…` |
| inert bounds (b = 0 once per solver input; 7.5 on user inputs and slots; an unknown id) | — | `a2bbfa3c…` |
| `evaluate` (point path) | `e8db0906…` | `e8db0906…` |

These are the round-0 hashes. b = 0 also gave the no-bound bytes on every runner case (6,615), except the T3-SI1b panic inputs (N-2).

## 3. Suites (fresh targets)

| Suite | Base | Candidate |
|---|---|---|
| `expression_evaluator` lib / corpus / doc | 31 / 1 / 0 | 49 / 1 / 0 (the S-1 and S-2 assertions are in existing tests) |
| `rule_check_runner` lib / acceptability / interval_bounds_run / demo / cases / doc | 14 / 4 / — / 3 / — / 0 | 14 / 4 / **11** / 3 / 1 / 0 |
| `rule_pack_document` | 6 / 1 / 3 / 0 | 6 / 1 / 3 / 0 |
| Python `test_rule_interval.py` | — | **193 passed** |
| Python related tests (`test_operation_result_schemas`, `test_ci_numerical`, `test_rule_pack_schema`, `test_retained_precision_carriers`) | outcomes | identical to base |

The Python related tests ran with the checked-JSON authority stubbed and cargo off `PATH`, so the same 24 environmental failures appear on both sides. These counts equal I73's §4. I did not rebuild src-tauri: the lock and the runner's public API are unchanged, the repair adds no public item, and I73 reports 116/116.

## 4. Mutants: 46 of 46 killed

These are my reruns on `WT/rv99/mut` (`evidence/round1/mutants/results.json`). Kill criteria are as in `REVIEW.md` §4. Each patch applies exactly once, and each restore was verified by sha256.

**I73's 21 mutants:**
- **M1–M10 (evaluator):** all killed.
- **R3c, R2, R3, R4, R5 (runner):** all killed. R4 is re-pointed to the repaired `Some(Some(b))` arms with the same semantics.
- **P1–P5 (Python):** all killed.

**My round-0 16:**
- **V1–V9, V14 (evaluator):** all killed. They include **V3** and **V9**, each killed by an evaluator test and by the shared cases.
- **V10–V13 (runner):** all killed. They include **V11**. V12 is re-pointed as I73 did.
- **PV1, PV2 (Python):** both killed.

**My 9 new mutants of the repairs:**

| Mutant | Repair | Killed by |
|---|---|---|
| W1 N-4 guard removed | N-4 | `a_downgraded_interval_check_emits_what_the_point_path_emits` |
| W2 N-4 code kept, finding dropped | N-4 | same |
| W3 duplicate bounds last-wins | N-5 | `duplicate_bounds_for_one_input_block_it` |
| W4 equal duplicates allowed | N-5 | same |
| W5 duplicate bounds first-wins | N-5 | same |
| PW1 Python: NaN or negative bound as a point | S-3 | 3 tests: shared cases `invalid_bound_nan` and `invalid_bound_negative`, and `test_reference_refuses_invalid_inputs_as_rust_does` |
| PW2 Python: inverted enclosure accepted | S-3 | 2 tests |
| PW3 Python: non-finite enclosure accepted | S-3 | 3 tests |
| PW4 Python: overlay on a missing value accepted | S-3 | 1 test |

My oracle independently flags 37 of the 46: 34 on pass 1, and PW1–PW3 on my 240 invalid-overlay cases. It does not flag R3, V11 (inside the outward margin, N-1), V6 (flagged on the corners in round 0), W1–W5 or PW4. W1–W5 need undeclared statuses or duplicate bounds, which my generated runner cases do not contain, so the probes cover them. **No survivors.**

## 5. Probes (`evidence/round1/probes/probes_r1.jsonl`)

- **b, round 1:** undeclared CHECKED and FAILED on both paths; undeclared INCOMPLETE with a U and with a T; empty `result_statuses`.
- **c, round 1:** duplicates [1, 60], [60, 1], [1, 1] and [0, 0]; duplicates on an unbound id and on a user input; the byte-equality of the last two with `run_rule_checks`.
- **Round 0's probes** a and g–j are unchanged in substance.

## Host and cleanup

**Cargo:**
- every job went through `WT/tools/t3_cargo.sh` (the memory guard was running), with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `RUSTUP_TOOLCHAIN=1.97.1` and `CARGO_INCREMENTAL=0`;
- `TMPDIR` was in my scratch, and the targets were `WT/targets/rv99/{cand,base,mut}`;
- my mutant jobs waited behind ROOT's U8 suite and other TASKs' jobs; no waiting job was killed.

**Python:** `VENV` with `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider` and `PATH=/usr/bin:/bin`.

**Not done:** no Git writes, installs, or DEC-025, native, solver-at-scale, src-tauri or TS jobs.

**Cleanup:**
- I deleted `WT/rv99/` and `WT/targets/rv99/`.
- `WT/scratch/rv99_s_i1_01/` (round 0, plus `r1/` for this round, about 1.6 GB of regenerable data and logs) stays for ROOT's post-merge cleanup.

**Records:**
- `evidence/round1/` holds tools, soundness checks, the differential, mutants, probes and suites;
- `SHA256SUMS.addendum_01` covers this addendum and every file under `evidence/round1/`;
- `REVIEW.md`, `SHA256SUMS` and the round-0 `evidence/` are unaltered.
