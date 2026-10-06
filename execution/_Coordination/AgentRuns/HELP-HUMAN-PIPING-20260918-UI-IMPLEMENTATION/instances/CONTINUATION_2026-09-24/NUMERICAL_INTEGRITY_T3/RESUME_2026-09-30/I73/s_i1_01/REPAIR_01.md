# I73 — S-I1 repair round 01: RV99's S-1, S-2, S-3, N-4 and N-5

TASK (Type 2) I73, the slice's owner, dispatched by ROOT (HELP_HUMAN).

**Status: complete.** All five repairs are done. V3, V9 and V11 are now killed.

**Basis:**
- **The ruling:** RR "RV99 passes S-I1 with three SHOULD-FIX; the repair round" (`T/ROOT_RULINGS_V1.md` at NUM `443922fbad`, RR:12633, read at sha256 `4cc4d28a9bff7a21…`).
- **The review:** RV99's `R/REVIEW_RV99/s_i1_01/REVIEW.md` (`48cfdbc0fc148299…`): its findings table, its mutant patch texts (`evidence/tools/mut_rv99.py`) and its Python invalid-input probe.
- **The design:** D2 revision 5b.3, unchanged.

**Placeholders:** `WT`, `NUM`, `P`, `T`, `R` as in RETURN.md. No machine paths.

**Branch state:** the head is `ea7c0881f4`: I73's work `4920e4b1b0` plus ROOT's two schema-description commits. The repairs are **uncommitted** on top of it. There were no Git writes.

**Interruption.** The host's Claude process ended during this round, while the last candidate Python sweep was at 30%. Every result in this record was regenerated after the restart. On resuming I checked:
- each modified file against `ea7c0881f4`: every edit is whole and intended;
- each file's sha256 against the values recorded before the interruption: they match;
- the scratch copies used for the mutants, differentials and src-tauri: identical to the worktree files (`cmp`).

All results below were produced on these exact bytes.

**After the restart I re-ran everything:**
- all 41 mutants;
- control 1;
- the rules suites;
- src-tauri;
- the Python tests and the full sweep.

**A host artifact found and removed (no code effect).** The first post-restart worktree evaluator suite (`rep2_ee`) reported one failure in the property test. Its message showed a quantity with a `table_argument_range` note but a finite enclosure, which is RV99's mutant V14 behaviour, not this code's. The cause is the shared target directory:
- the worktree crate and its scratch copies have the same relative paths and the same unit hash;
- `expression_evaluator` has no dependencies;
- so cargo judged the V14 mutant's test binary, built moments earlier from the scratch copy, "fresh" against the worktree's older `src/lib.rs`.

I re-ran the three rules suites in a fresh target, `WT/targets/i73-s-i1/worktree-fresh`. They pass (`rep3_*`), and those are the numbers in §4.

Every mutant patch writes its file before building, so no kill can come from a stale binary; each mutant's failing tests are its own. `rep2_ee.summary.txt` is kept as the record of the artifact. RV99 should use separate targets per copy, as it already does.

## 1. Changed files (all inside S-I1's fence; diff against `ea7c0881f4`)

| File (`P/…`) | Before (`ea7c0881f4`) | After | Lines | Change |
|---|---|---|---|---|
| `core/rules/expression_evaluator/src/lib.rs` | `8925cc0d…` | `fbc42fee05211c45ed7cdfebb402a832abb63799665fc42c18c5f06fbff78e0a` | +28 / −0 | tests only (S-1, S-2); production code untouched |
| `core/rules/rule_check_runner/src/lib.rs` | `1803a02d…` | `3f58533fc46e8f145049dabb92fd3c0ed493a516eb3d7033176bd322c7de76a5` | +74 / −39 | N-5 (duplicate bounds), N-4 (downgraded check); most of the −39 is the re-indented outcome block |
| `core/rules/rule_check_runner/tests/interval_bounds_run.rs` | `6c8aa798…` | `6834c2114f3db48a5d8426ac5ff03a872d731bd1b700ac028ed2d48bdd9478ed` | +137 / −0 | N-4 and N-5 tests; both conversion ends pinned bit for bit (N-1) |
| `core/rules/rule_check_runner/tests/rule_interval_cases.rs` | `549e9910…` | `4fd95e6024798bb2941437d0304eda85393a7280404cc8903bcc22fe907eafd1` | +15 / −3 | explicit enclosures, and any b ≠ 0 through `enclosure_from_bound` |
| `core/analysis_runs/rule_interval.py` | `c5445be1…` | `e7dd8e0bb750b7c08facec222336cb080d5170c2a66712fbafd8e35e2fbe1374` | +44 / −8 | S-3 (input refusals as Rust) |
| `fixtures/rule_interval/rule_interval_cases.json` | `85af9846…` | `c3709b53569e05f32e086569fd2895c3246c9663cfc3179eabd2892e439bc09c` | +495 / −1 | 11 new cases (83 → 94); the 83 existing cases are byte-identical; the notice describes the new input forms |
| `tests/test_rule_interval.py` | `2b3dd690…` | `41f021dfb4f73bc45db2880d1646be8b3b6d7b1a851192f1e6cdb974b114a224` | +49 / −9 | S-3 test; explicit-enclosure inputs; the oracle sampler |

**What did not change:** no schema, manifest, lock or dependency. The src-tauri lock is still `4de71f1b…`. ROOT's schema commits are untouched.

## 2. The repairs

### S-1: `=` and `≠` between identical non-point ranges

The production code was already correct; this repair adds tests only.
- **New shared cases:** `equal_identical_ranges` and `not_equal_identical_ranges`. Each has two independent inputs, x and y, both 1 ± 2⁻⁴⁰. Expected U; both are negative controls.
- **Evaluator test:** `comparisons_are_three_valued_with_strict_ends` gains the same two assertions.
- **Result:** RV99's **V3** (`=` is T when the ranges are equal) is killed by the evaluator's own suite (that test fails) and by the shared cases (`rust_interval_mode_matches_every_shared_case` fails).

### S-2: a divisor range ending exactly at zero

The production code was already correct; this repair adds tests only.
- **New shared case:** `divide_by_zero_end_range` is `0 / (−abs(z)) ≤ 1` with z = 0 ± 1. The divisor is [−1, −0]. Expected U, with the note `divide_by_zero_range`; it is a negative control, since the point path blocks at z = 0.
- **Evaluator test:** `division_by_a_range_containing_zero_is_indeterminate` gains the same case.
- **Result:** RV99's **V9** (a strict zero test) is killed by the evaluator's own suite and by the shared cases.

### S-3: the Python reference refuses invalid inputs exactly as Rust does

`evaluate_interval` in `rule_interval.py` now handles inputs as a Rust caller of `evaluate_interval` supplies them:
- **A `bound` other than 0** overlays `enclosure_from_bound(value, b)`. That gives no finite enclosure for a negative, NaN or infinite b: U, with the note `non_finite_enclosure` on the input. This is Rust's `enclosure_from_bound` returning `None`.
  - Before this repair, a NaN or negative bound was treated as "no bound" and could read T.
  - b = 0 is still the exact point.
- **An explicit `enclosure`** is validated as Rust's `build_interval_overlays` validates it, with the same codes and subjects:
  - a non-finite end gives `NonFiniteInput`;
  - an inverted pair gives `InvalidReference`;
  - an overlay on an input with no value gives `InvalidReference`;
  - a repeated overlay gives `DuplicateBinding`;
  - an empty id gives `InvalidReference` / `interval_binding`.

  These checks run in Rust's order: after the bindings and the required ids.

**The case-file harnesses** (`rule_interval_cases.rs` and `test_rule_interval.py`) read the same two input forms:
- `bound_bits`: any b ≠ 0 goes through the production `enclosure_from_bound`;
- an optional `enclosure_bits`: a bit pair, or `null` for no finite enclosure, which replaces the bound.

**Nine new parity cases:**
- `invalid_bound_nan`, `invalid_bound_negative` and `invalid_bound_infinite`: U, with `non_finite_enclosure` on x;
- `enclosure_explicit_valid`: T;
- `enclosure_explicit_none`: U;
- `enclosure_inverted`: blocked, `InvalidReference` x;
- `enclosure_nan_end` and `enclosure_infinite_end`: blocked, `NonFiniteInput` x.

Rust and Python both reproduce all 94 cases. The Python test `test_reference_refuses_invalid_inputs_as_rust_does` also covers −∞ bounds, a (0, +∞) enclosure, an overlay on an input with no value, a repeated id and an empty id.

**Reading.** At the evaluator level, Rust "refuses" an invalid b through `enclosure_from_bound` (`None`, so U). The runner refuses it as a blocking `RULE_EVALUATOR_ERROR`. Python has no runner, so it mirrors the evaluator level. RV99's remedy allowed either.

### N-4: a downgraded interval check emits what the point path emits

In `finish_interval_check`, when the pack's `result_statuses` omit the status (T → `USER_RULE_CHECKED`, F → `USER_RULE_FAILED`), `enforce_declared` downgrades it to `RULE_INPUTS_INCOMPLETE`, as before. Now the check no longer carries `RULE_INTERVAL_ALL_PASS` or `RULE_INTERVAL_ALL_FAIL`, nor their info finding. It emits what the point path emits for a downgraded check: the status and `STATUS_NOT_DECLARED`.

An indeterminate check is already `RULE_INPUTS_INCOMPLETE`, so it keeps `RULE_RESULT_INDETERMINATE` (D2 §4.11.5 pairs it with that status). If the pack omits that status too, `STATUS_NOT_DECLARED` is added beside it.

**Test:** `a_downgraded_interval_check_emits_what_the_point_path_emits`.
- For all-pass (50 ± 1) and all-fail (150 ± 1), the bounded check's status, `diagnostic_codes` and serialized `evaluator_findings` equal the point path's for the same check.
- Neither interval code appears in its wire form.
- An undeclared indeterminate check keeps its code.

### N-5: duplicate bounds for one input are refused

`run_rule_checks_with_bounds` now records a second bound for the same input id as refused, not last-wins. It reuses the invalid-bound route and code:
- a blocking completeness finding `RULE_EVALUATOR_ERROR` on the input, with the message "more than one absolute bound was supplied for this solver result; the input is treated as unsupplied";
- the input becomes unsupplied, with the note `duplicate absolute bounds: treated as unsupplied`;
- the pack's `missing_input` code applies.

Duplicates on an id the pack does not bind change nothing. The doc comment of `SolverResultBound` says so.

**Test:** `duplicate_bounds_for_one_input_block_it`. It covers [1, 60], [60, 1], [1, 1] and [0, 0], plus duplicates on an unbound id, which must give output byte-identical to `run_rule_checks`.

**Python** takes one bound per input. Its repeated ids already give `DuplicateBinding` (S-3 test), so there is no bound list to mirror.

### N-1 (accepted by ROOT; test only)

`unit_normalization_steps_each_end_outward` now pins both ends of the psi → MPa conversion bit for bit. Each end is the bound's outward end, then the units crate's four operations, each stepped outward. The test checks `actual ≤ limit` at:
- the exact upper end: `USER_RULE_CHECKED`;
- one ulp below it: indeterminate;
- the exact lower end: indeterminate;
- one ulp below it: `USER_RULE_FAILED`.

This kills RV99's V11, the one mutant ROOT had accepted as surviving. No production change.

## 3. Controls, rerun on the repaired candidate

### Control 1: point mode byte-identical to base `c1bfc460fc`

Same harnesses as RETURN §4.

| Dump | Base | Repaired candidate |
|---|---|---|
| evaluator `evaluate`: 69 corpus cases plus 36,000 seeded inputs, 36,069 lines | `e74a69c43af2b277…` | `e74a69c43af2b277…` (identical) |
| runner, 32,820 runs over the committed packs and run-fixture rows: `run_rule_checks` | `697b31fd1c32c420…` | `697b31fd1c32c420…` |
| `run_rule_checks_with_bounds(&[])` | — | `697b31fd1c32c420…` |
| inert bounds (b = 0 on solver inputs; bounds on user inputs and unknown ids) | — | `697b31fd1c32c420…` |

### Mutants: 41 of 41 killed (`_run_records/repair_01/repair_mutants_results.json`)

**Kill criteria** (I73's and RV99's):
- evaluator mutants: `cargo test --lib` in `expression_evaluator` must fail;
- runner mutants: the full runner suite must fail;
- Python mutants: `tests/test_rule_interval.py` must fail.

V3 and V9 were also run against the runner's shared-case test.

| Set | Mutants | Killed |
|---|---|---|
| I73's evaluator mutants | M1, M2, M4a, M4b, M4c, M5–M10 | 11/11 |
| I73's runner mutants (R4 re-pointed at the repaired `Some(Some(b))` arms, same semantics) | R3c (brief 3c), R2, R3, R4, R5 | 5/5 |
| I73's Python mutants | P1–P5 | 5/5 |
| RV99's evaluator mutants | V1–V9, V14 | 10/10, including **V3** (evaluator suite: `comparisons_are_three_valued_with_strict_ends`; shared cases: `rust_interval_mode_matches_every_shared_case`) and **V9** (evaluator suite: `division_by_a_range_containing_zero_is_indeterminate`; shared cases) |
| RV99's runner mutants (V12 re-pointed the same way) | V10–V13 | 4/4, including **V11** (`unit_normalization_steps_each_end_outward`) |
| RV99's Python mutants | PV1, PV2 | 2/2 |
| This round's mutants | R6 (downgraded check keeps its interval code), R7 (duplicate bounds last-wins), P6 (Python: invalid bound as a point), P7 (Python: enclosures unvalidated) | 4/4 |

The patch texts are I73's and RV99's own, the latter copied byte for byte into `rv99_mutants_patches.py`. Each patch applies exactly once. Every file was restored and checked with `cmp` afterwards.

## 4. Suites against base `c1bfc460fc`

| Suite | Base | Repaired candidate | Change since base |
|---|---|---|---|
| `expression_evaluator` lib / corpus / doc | 31 / 1 / 0 | 49 / 1 / 0 | +18 (unchanged since RETURN; this round adds assertions to existing tests) |
| `rule_check_runner` lib / acceptability / interval_bounds_run / demo / cases / doc | 14 / 4 / — / 3 / — / 0 | 14 / 4 / **11** / 3 / 1 (94 cases) / 0 | +12 (+2 this round: the N-4 and N-5 tests) |
| `rule_pack_document` | 6 / 1 / 3 / 0 | 6 / 1 / 3 / 0 | none |
| src-tauri lib / main / doc | 116 / 0 / 0 | 116 / 0 / 0 | none; lock unchanged |
| Python `tests/test_rule_interval.py` | — | 193 passed (94 parity, 94 oracle, 5 others) | +23 since RETURN |
| Python sweep (`pytest -q tests --dist loadscope`, cargo off PATH, `-n 6`) | 3,465 passed, 3 failed, 64 errors, 40 skipped | **3,658** passed, 3 failed, 64 errors, 40 skipped | +193: `test_rule_interval.py`; every base outcome identical |

**Rust per-test comparison:** every base test name and outcome is present and unchanged. Added: evaluator 18, runner 12, document 0, src-tauri 0 (`_run_records/repair_01/*.txt`).

**The sweep.**
- The full run was made after the restart, on an idle host. A `comm` of the junit outcomes against the base sweep (`final/sweep_base.outcomes`) gives 0 base outcomes changed, and the only additions are the 193 `test_rule_interval.py` tests (`sweep_cand_repair.outcomes`).
- The 3 failures and 64 errors are the same environmental set as in RETURN §4: cargo is off PATH, and the trees are archives without `.git`.
- **An earlier attempt is set aside.** A run made while this round's cargo jobs loaded the host had one more failure, `test_performance_harness_runner.LiveLimit::test_a_terminated_runner_leaves_no_survivor`, a 20-second process-termination timeout. That test passes alone in both trees (0.15 s), and it is outside S-I1.

## 5. Notes for RV99's confirmation and ROOT

- **No production change in the evaluator.** Its diff is test-only.
- **The runner's production diff** is the N-5 bound map, the N-4 guard and a doc line. The point path is untouched, and control 1 confirms it.
- **The 83 existing shared cases are byte-identical.** The case file's header notice now describes `enclosure_bits` and invalid bounds.
- **`rustfmt`:** the new Rust code is formatted. The runner's pre-existing test module is still not `rustfmt`-clean and was left alone.
- **N-2, N-3 and N-6** were ruled by ROOT and are not touched here. N-1 is closed by the pinned test.
- **Scratch:** `WT/scratch/i73_s_i1/` holds the overlays, dumps, logs, the sweep trees and the generator backup `gen_cases_v1.py`. It is left for ROOT to prune.

## 6. Commands and host

**Cargo.** Every cargo command ran through `WT/tools/t3_cargo.sh` (the wrapper `cargo_run.sh`), with:
- `--locked --offline`;
- `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`;
- targets `WT/targets/i73-s-i1{,-base}`;
- the memory guard up, and `TMPDIR` in scratch.

**Commands:**
- rules suites in the worktree;
- src-tauri, the differentials and the mutants on the scratch candidate overlay (`sync_cand.sh`: every file changed against base, committed or not);
- the Python sweep via `run_sweep.sh`.

**No** Git writes, installs, or native, DEC-025 or solver-at-scale jobs.

## 7. Run records (`_run_records/repair_01/`)

- `gen_cases.py` (updated) and `gen_cases_repair.diff`: the 11 new cases, derived by hand as before.
- `repair_mutants.py`, `rv99_mutants_patches.py` and `repair_mutants_results.json`.
- `differential_dump_sha256.txt`.
- Suite summaries and per-test outcome lists.
- `sweep_cand_repair.outcomes` (node ids shortened as in `final/`).
- `sync_cand.sh`.

The earlier records (`CHECKPOINT_1.md`, `RETURN.md`, `_run_records/checkpoint1/`, `_run_records/final/`) and their SHA256SUMS entries are unaltered. This round's files are listed in `SHA256SUMS.repair_01`.
