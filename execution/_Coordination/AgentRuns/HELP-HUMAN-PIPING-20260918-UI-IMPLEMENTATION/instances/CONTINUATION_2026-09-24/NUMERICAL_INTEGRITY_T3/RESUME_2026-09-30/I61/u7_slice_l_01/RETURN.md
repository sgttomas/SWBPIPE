# I61 RETURN: U7 slice L (the live reruns, U5, the PP sweeps, and 07j for C04)

**Status: complete, with no stop.** The work is uncommitted in WT/f2a-u7, on top of the U7 head `e5e1693ceb`. It touches three files, all inside the fence: the corpus fixture and two lines of shared-entry count pins. RV89's four Pass B conditions hold (§6).

**The results:**
- **Live successors.** PP's actual Direct entry in the registered build publishes U1's pinned successors, `ac6986b0…` (sparse) and `6cd1d249…` (dense). They are byte-identical to U6's carrier fixtures.
- **Tokens** on those live bytes, through each reader and carrier:
  - with the invocation: `numerically_eligible` in all three languages (TS through the real mocked direct and job IPC, with a live capture);
  - without it: `needs_recompute` in all three;
  - D-U7-4's two forms: `numerically_eligible` in Python and Rust, and `needs_recompute` with `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED` in TS, which is the declared difference.

  Every token equals slice A's oracle or the case file's D-U7-4 expectation; there are 0 mismatches. RV92's survival chain on the live bytes keeps the receipt byte-equal and revalidates it in all three languages.
- **U5** on the live bytes: identical to U5's report and log **except the reader's two eligibility fields per mode** (`numerical_eligible` false→true, `standing` needs_recompute→eligible). Those are U7's own switch. Substituting U5's two values back reproduces U5's report and log **byte for byte**. Every class claim, tally and stop is unchanged. A literal "byte-identical" cannot hold after U7, and I report it that way rather than claiming it.
- **PP sweeps:** cited, not rerun. PP's code and `result_export`'s production code at `e5e1693ceb` are hash-identical to I66's part-1 lane, so I66's registered `9a74ff16…` and Stale `0e2db8b8…` sweeps stand.
- **07j (C04): constructed and added.** `not_required_second_case_checks_passed` is a must-pass entry whose second case is `not_required`. It passes every gate in all three readers and is eligible with its invocation. The C04 narrowing mutant (`not_required` dropped) is **killed in all three languages on 07j, and survives all three on 07i**: the entry closes the gap.
- **Suites:**
  - Python retained 460 (459 plus 1);
  - `result_export` 169;
  - vitest 3,542 (3,541 plus 1);
  - `tsc` clean;
  - the Python 24-file sweep, 1,846 passed, 30 skipped, 0 failed (§5).
- **RV93 N-5** (the real-input Candidate test) is deferred to U8, as ruled. It is not added.

**Run facts.**
- **Role:** TASK Type 2 under ROOT, with no descendants.
- **Basis:** ROOT's dispatch message, RV89's fence note, and RR "U7 slice F part 1 returned…", "U7 slice F committed…" and "The withheld fix committed; slice L dispatched".
- **Time:** 2026-10-04, 18:50Z to about 19:36Z.
- **Host:**
  - The memory guard (5387) ran throughout.
  - Cargo ran one job at a time, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, in targets under WT/targets/i61-u7l/ only.
  - Python used the project venv with I52's CLIs, `TMPDIR` and `--basetemp` under scratch, `PYTHONDONTWRITEBYTECODE=1` and no pytest cache.
  - Vitest and tsc ran in a scratch lane: node_modules is the existing symlink, and the prebuilt `public/` is copied.
  - No Git writes; reads used `GIT_OPTIONAL_LOCKS=0`. No PP or runner run was needed, because neither was touched.
- **Lanes, under WT/scratch/i61_u7_slice_l_01/:**
  - `lane` is a `git archive e5e1693ceb` plus the three candidate files. Its two milestone fixture paths hold the live bytes (byte-identical). The harnesses were parked before the suites ran, so the lane equals WT apart from the archive's scope.
  - `mut` is a copy of `lane` for the mutants, restored after each run (checked).
- **Other agents' files:** none touched. RV92's harness was copied, and its two `!numerical_eligible` revalidation checks were adapted (§2).

## 1. The live successors (RV92 N-8, item 3)

- **The build.** The registered build of `e5e1693ceb`. Its build-script identity is the registered entry's (`live_successors.txt`).
- **The bytes.** The committed `u3g2_direct_entry_publishes_the_pinned_successor` wrote them with `I61_U3G2_OUT`. That test takes the Successor branch only in the registered build, and asserts U1's pins.
  - `u3g2_successor_sparse_interactive.json`: `ac6986b0…`
  - `u3g2_successor_dense_scrutiny.json`: `6cd1d249…`
- **Both are byte-identical to the carrier fixtures,** `P/fixtures/results/retained_precision_milestone_successor_{mode}.json`. In the lane, the live files were copied onto those paths, so every carrier test that reads them read the live output.

## 2. Tokens across the three languages (`tokens/TOKEN_TABLE.md`)

**The harnesses** (scratch only, `harness/`):

| Language | Harness | What it runs |
|---|---|---|
| Python | `py_tokens.py` | `validate_retained_precision`, `numerical_use_standing`, `classification_summary` |
| Rust | `zz_i61_tokens.rs` | `retained_precision::validate`, `semantic_contract::numerical_use_standing(_with_context)`, `classification_summary` |
| TypeScript | `zzI61Tokens.test.tsx` | The real `runPreviewMechanics` and job polling with mocked IPC (a live capture), `retainedPrecisionStanding`, `numericalResultStanding`, `classificationSummary`. **No reader is wrapped** |

**The tokens:**

| Form (both modes) | Python | Rust | TypeScript | Expected |
|---|---|---|---|---|
| Milestone with its invocation | `numerically_eligible` | `numerically_eligible` | `numerically_eligible`, status `integrity_checked` (direct and job IPC) | Slice A oracle: match |
| Milestone without it | `needs_recompute` | `needs_recompute` | `needs_recompute` (`NOT_NUMERICALLY_ELIGIBLE`) | Slice A oracle: match |
| D-U7-4 (a) `invocation_without_native_capture` | `numerically_eligible` | `numerically_eligible` | `needs_recompute` (`NATIVE_CAPTURE_REQUIRED`) | Case file, declared: match |
| D-U7-4 (b) `stale_current_model_same_case_ids` | `numerically_eligible` | `numerically_eligible` | `needs_recompute` (`NATIVE_CAPTURE_REQUIRED`) | Case file, declared: match |

- **The reader level:** `numerical_eligible` is true with the invocation and false without it, in all three languages.
- **The summaries.** `withheld` is 69 with the invocation and 97 without it, in all three languages. TS shows 97 on both D-U7-4 forms, per I67's fail-closed fix.
- **The only cross-language difference** is the declared D-U7-4 (TS is stricter and fails closed). Python and Rust read the invocation argument; TS also requires the live native capture.

**RV92's survival chain on the live bytes** (`survival/`, `harness/run_survival.sh`):
- **Step a:** Rust derives and validates; Python and TS build and validate the AnalysisRun; TS saves and reopens.
- **Step b:** each language puts every carrier's receipt back on the raw source and revalidates it.
- **The results:**
  - every receipt is byte-equal;
  - every revalidation equals the base validation;
  - Python's and Rust's tokens are `numerically_eligible` with the invocation and `needs_recompute` without it;
  - TS's registration without an IPC capture reads `needs_recompute` (`NATIVE_CAPTURE_REQUIRED`, D-U7-4);
  - the derivative and both AnalysisRun records pass their schemas.
- **The one adaptation:** RV92's harness checked `!numerical_eligible` as part of "revalidated", which held only before U7. My copies compare it with the base validation's value instead (`*.adapted.*`). RV92's originals are unchanged.

## 3. U5 on the U7 head (`u5/`)

**The inputs:**
- the live bytes;
- the pinned `u5_compare.py` (`df4684d3…`);
- RV86's 7,240-byte extract;
- the U7 head's Python reader, with its flag on.

**The result:** no stops, and in each mode all 97 class claims pass against both readouts. The report and log differ from U5's only in `reader.numerical_eligible` (true) and `reader.standing` (`eligible`), per mode. `u5_delta.py` substitutes U5's two values back and gets U5's report and log byte for byte (`u5_delta.txt`).

So U5's comparison is unchanged, and the difference is exactly U7's switch, on the oracle's value.

## 4. The PP sweeps: cited (`pp_re_vs_i66_lane.txt`)

At `e5e1693ceb`, every PP file and every `result_export` production file is hash-identical to I66's part-1 lane:
- PP: `lib.rs`, `retained_wire_tests.rs`, and every other file unchanged from `12a849a7bd`;
- `result_export`'s `retained_precision.rs` and `semantic_contract.rs`.

Only `result_export/tests/retained_precision_carriers.rs` and the carrier case file differ: part 2's scope-assertion line and the v4 case file. The PP sweep compiles neither.

**I66's sweeps therefore stand:** registered `9a74ff16…`, Stale `0e2db8b8…`, both byte-identical to `u3_grant2_02`'s.

## 5. 07j and C04

**The entry** (`build_07j.py`). On `two_case_preparation_failure_synthetic`, whose case 1 has no run and no source, it makes case 1 the contract's `not_required` form:

| Edit | Basis |
|---|---|
| Case 1 becomes `{basis_ref, ordinary, product_attempt_ref: null, status: "not_required"}` | The schema's `Case` not_required branch; C1 WIRE_CONTRACT.md:101 (common members only) |
| Product attempt 1 is removed | A not_required case has no attempt (C1:101; D6b) |
| The `RETAINED_PRECISION_UNAVAILABLE` diagnostic is removed | Only an unavailable case carries it (G4) |
| The case's integrity diagnostic becomes info `NUMERICAL_INTEGRITY_CHECKS_PASSED` | The preview-physics-1 base's checks-passed form |
| `numerical_quality.cases[1].solve_quality` and the ordinary initial report's outcome become `checks_passed` | D6b; the initial report states the quality |

**The expectation,** `expected_eligibility` = {invocation_bound true, numerical_eligible true, standing "eligible"}, comes from slice A's rules R1/R2 (C1:160; D2 §4.9.4). Slice A's unchanged oracle, run on 07j, gives the same, with carrier token `numerically_eligible`. Every 07j base and must-pass expectation equals the oracle's (`oracle_07j.json`).

**The corpus** is 07j, `6ecd0ae2…`, from 07i `07f6f95c…`: 15 cases, 277 mutations, 24 must-pass entries, and every 07i byte kept (asserted).

**Pickup.** All three languages already iterate `must_pass` and assert `expected_eligibility`, so the entry is picked up with no iteration change. Only the count pins moved:
- Python `test_snapshot_07_counts_and_entry_format`: (15, 277, 23) → (15, 277, 24); eligible (13, 13) → (13, 14); one assertion that this entry is the corpus's only not_required case;
- Rust `shared_must_pass_entries_validate`: 23 → 24.

TS has no count pin.

**The C04 mutant** (`c04/mutants_c04.json`; one edit each; pristine bytes restored):

| Mutant | On 07j (candidate) | On 07i (U7 head corpus and tests) |
|---|---|---|
| PY: `("selected",)` | **killed**: `must_pass[not_required_second_case_checks_passed]`, `test_public_entry_equals_the_draft…` | survives |
| RS: `matches!(…, "selected")` | **killed**: `shared_must_pass_entries_validate` | survives |
| TS: `['selected'].includes(…)` | **killed**: `must pass: not_required_second_case_checks_passed` | survives |

None was killed by a compile or syntax error.

**The carrier level** (`c04/`):
- On the 07j statement, Rust's `numerical_use_standing_with_context` is `numerically_eligible`, as is Python's. So the carriers' not_required ordinary-eligibility conjunct is now exercised with a gate-passing statement.
- TS's reader marks it eligible with its invocation.
- TS's carrier route through mocked IPC refuses it at G8 (`INVOCATION_MISMATCH`). This is a harness limit, not a reader finding: the preview service captures its own request, and the corpus invocation is not a desktop-shaped request.

**The suites:**

| Suite | Result |
|---|---|
| Python retained (contract, schema, carriers) | **460 passed** (459 plus the new entry) |
| `result_export` | **169 passed**, 0 failed |
| Vitest | **3,542 passed**, 138 files (3,541 plus the new entry) |
| `tsc --noEmit` | clean |
| Python 24-file sweep plus the retained suites, in WT/f2a-u7 | **1,846 passed, 30 skipped, 0 failed**. Against I66's candidate sweep (1,843): this entry's must-pass parameter, plus I66's two `test_a_solved_status_is_required_before_the_eligibility_conjunct` cases, committed after that sweep. No other outcome differs (`suites.txt`) |

The sweep's first run, in the lane, failed 27 tests. Each was a lane-scope artifact: the archive lacked `examples/` and the `execution/` handoff fixtures. I reran it in the worktree, read-only, as I66 did.

## 6. RV89's four Pass B conditions

- **(a) holds.** The corpus is embedded only by `result_export`'s `#[cfg(test)] mod u6e_reader_round_tests` (`retained_precision.rs:4349`) and by integration tests. It is not one of the 14 reviewed inputs, and no D1 production code reads it.
- **(b) holds.** PP's `Cargo.toml` (`1d314e63…`) and `Cargo.lock` (`4f494db6…`) are unchanged (`changed_files_sha256.txt`).
- **(c) holds.** No law, witness or challenge test file changed. No `#[cfg(test)]` statement went into production code.
- **(d) holds.** No production `.rs` changed. The only `.rs` change is `result_export/tests/retained_precision_contract.rs`, an integration test.

**So no Pass B rerun is needed.**

## Records (`_run_records/`)

All paths are placeholders, with no machine paths.
- **Candidate:** `candidate_tests.diff`, `candidate_status.txt`, `changed_files_sha256.txt`, `build_07j.py`, `oracle_07j.json`.
- **Live:** `live_successors.txt`.
- **Tokens:** `tokens/{py,rs,ts}_tokens.json`, `tokens/TOKEN_TABLE.md`, `token_table.py`.
- **Survival:** `survival/*.jsonl`, `survival/carrier_outputs_sha256.txt`.
- **U5:** `u5/u5_report.json`, `u5_run.log`, `u5_delta.py`, `u5_delta.txt`.
- **C04:** `c04/mutants_c04.py`, `c04/mutants_c04.json`, `c04/{rs,ts}_tokens.json`.
- **Sweeps:** `pp_re_vs_i66_lane.txt`.
- **Harnesses:** `harness/` (`py_tokens.py`, `zz_i61_tokens.rs`, `zzI61Tokens.test.tsx`, `zz_rv92_survival.adapted.rs`, `zzRV92Survival.adapted.test.ts`, `rv92_py_survival.py`, `run_survival.sh`).
- **Suites:** `run_suites.sh`, `suites.txt`.
