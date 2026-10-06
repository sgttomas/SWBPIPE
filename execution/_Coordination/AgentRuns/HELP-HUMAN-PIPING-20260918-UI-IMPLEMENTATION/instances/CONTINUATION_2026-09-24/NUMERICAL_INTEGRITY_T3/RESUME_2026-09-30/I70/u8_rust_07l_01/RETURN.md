# I70 U8-3: the Rust alignment to corpus 07l

TASK (Type 2), I70, for ROOT (HELP_HUMAN, Agent 0). 2026-10-06 UTC (2026-10-05 host local time). No delegation.

**Briefs (verified before work):** `R/BRIEFS/U8_COMMON.md`; `R/BRIEFS/I69_I70_I71_U8_CORPUS_AND_READERS.md` sha256 `acb04c2596ae1fdd96fff5fb4b13babba0472184cd14a1c6e723c096cf4962e7` (the I70 section). **Basis read:** `NUM/AGENTS.md`, `NUM/agents/AGENT_TASK.md`, `NUM/P/AGENTS.md`; I69's return `R/I69/u8_corpus_07l_01/RETURN.md`; RR "I69's corpus 07l committed; I70 and I71 dispatched" (reading 4 widens my fence) and "U8-1 committed; …".

**Placeholders:** `WT`, `NUM`, `P`, `RE` = `P/core/reporting/result_export`, `T`, `R`, `RR` as in the dispatch. `CAND` = `WT/f2a-u8` at the U8 head `69a925bd68` plus my uncommitted file. `BASE` = a disposable `git archive` of `d449097085` (U8-1, corpus 07k). `PROBE` = a disposable, records-only `git archive` of `69a925bd68` (the crate, its two path dependencies, `P/fixtures/results` and `P/schemas`), holding my candidate test file plus an appended listing test.

**Host rules kept:**
- Writes in CAND: exactly `RE/tests/retained_precision_contract.rs`, left uncommitted. `git status --short` in CAND also shows I71's TS file and I71's untracked `node_modules` symlink (`_run_records/candidate_status.txt`); I touched neither. No reader `src/` text (including its `cfg(test)` module), no corpus or fixture text changed.
- No Git writes; reads used `GIT_OPTIONAL_LOCKS=0`.
- Cargo only through `WT/tools/t3_cargo.sh` (memguard up), with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS unset, and target `WT/targets/i70-u8`. Eight jobs, all `test` on `result_export`: BASE, CAND, and six PROBE runs (`_run_records/logs/cargo_jobs_i70.log`). No DEC-025, native or solver job.
- Scratch was under `WT/scratch/i70_u8/` only.

## 1. The changed file

| File | sha256 before (`69a925bd68`, which equals `d449097085`) | sha256 after | `git diff --numstat --histogram` |
|---|---|---|---|
| `RE/tests/retained_precision_contract.rs` | `c74c8c852dbcdf5aad3a55ab9b37ad250bc3b53db939bc755ef4d6403b31c3a0` | **`7832a6024c8c460bd02d145ec490fbe5ee798e118ab14c6163fd919030d6d5d0`** | +23 / −6 (`_run_records/candidate.diff`) |

**The diff, item by item.** Line numbers are the base's; after the first item, the file's lines shift by one.

1. **`:1`, the module doc.** It now reads "Shared synthetic statement controls plus listed producer-solved bases (07l); no native Current evidence. These do not establish execution." This mirrors the Python docstring that I69 changed and ROOT accepted. 07l makes "synthetic" alone stale. See §5, item 1.
2. **`:206`, the stale eligibility comment.** It now says 15 bases are eligible (07l: 13 synthetic plus the two producer-solved L = 0 bases), and the two with an unavailable case are not.
3. **`:276–281`, the mutation count.** The doc comment adds "07l (U8-2) appends the 8 L = 0 mutations, making 286", and `assert_eq!(mutations.len(), 278)` becomes **`286`**. Lines 277–280 and 282–283 are unchanged.
4. **New: `snapshot_07l_mutation_outcomes`**, after `snapshot_07h_mutation_outcomes`. It is `slice_outcomes("I70_OUTCOME_07L", 278..286, &[("G5a RETAINED_PRECISION_SCALE_MISMATCH", 8)])`, with a doc comment naming the entries and their order.
5. **`:833–835`, the must-pass count.** The comment adds "07l (U8-2) appends the 4 L = 0 entries on the producer-solved bases, 24 to 28", and `assert_eq!(entries.len(), 24)` becomes **`28`**.

**Nothing weakened.** No assertion is deleted or loosened. The two count pins change only their expected values, for 07l's appended entries. Everything else is comments plus one new test that uses the existing slice helper.

**The 8 new mutations are now observed twice.** Before this change, only `shared_rehashed_first_failure_mutations` observed them. It checks every mutation against the corpus's own expectation, so a corpus that changed an entry and its expectation together would still pass. Each existing slice tally covers a fixed range ending at 277. The new slice adds the per-mutation `observed == expected` check, the literal tally of 8 at G5a SCALE, and an outcome line for each mutation. TM-A in §3.2 shows that this slice is the only check that pins the literal first gate and code.

## 2. The full-07l outcome (CAND, the Rust reader at `RE/src/retained_precision.rs` `4722b505…`, unchanged)

**Corpus:** `P/fixtures/results/retained_precision_cases.json` `5ac13296c69745ae837e41b93c42e4daa0c4100dc863229f8e6a4cdbe2692ccd` (07l), with counts (17, 286, 28).

**No stop fired.** The Rust reader accepts both faithful producer-solved bases.

| Set | Outcome | Where |
|---|---|---|
| **Cases, 17 of 17** | Each validates with its invocation; is invocation-bound; has its expected eligibility (15 eligible, 2 not); matches its `publication_sha256`; and matches every expected classification row (result id, basis ref, normalized, scale and bound bits, class). Each is also not eligible without its invocation, and the transport-metadata form is empty. **`u8_l0_isolated_node_sparse_interactive`**: 113 rows, 25 relative / 78 absolute / 9 input / 1 non-quantity / 0 not covered, eligible. **`u8_l0_isolated_node_dense_scrutiny`**: 114 rows, 25 / 78 / 9 / 2 / 0, eligible. Both equal I68's three-reader counts. | `complete_synthetic_controls_carry_their_shared_eligibility` (ok); PROBE listing `I70_CASE` |
| **Mutations, 286 of 286** | Each is refused with its expected first gate and code. Rust's own `expected_by_reader.rust` is used where present (index 277). **Indices 278–285**: `isolated_rotation_stop`, `isolated_has_data` and `isolated_estimate_coupled` (sparse, then dense), then `isolated_translation_rotation_stop` (sparse, then dense). All 8 are observed at **G5a `RETAINED_PRECISION_SCALE_MISMATCH`**, as expected. | `shared_rehashed_first_failure_mutations` (ok); the 15 existing slices over 0..277 (ok, 277 lines identical to BASE); **`snapshot_07l_mutation_outcomes` (ok, 8 of 8 matched; `_run_records/logs/cand_07l_slice_outcomes.txt`)**; PROBE listing `I70_MUTATION` |
| **Must-pass, 28 of 28** | Each is admitted with its stated eligibility (18 eligible) and its base's classifications. **Indices 24–27**: `isolated_estimate_uncoupled` and `isolated_translation_stop`, each sparse and dense. All four are admitted, eligible, with the base's classifications (25/78/9/1 sparse, 25/78/9/2 dense). | `shared_must_pass_entries_validate` (ok; 28 `I63_MUST_PASS … true` lines); PROBE listing `I70_MUST_PASS` |

**The per-entry listing.** `_run_records/logs/analysis/full_07l_listing.jsonl` has 331 lines: every case, mutation and must-pass entry, with expected and observed values. It comes from the PROBE control run (`logs/probes/none.log`): 64 of 64 passed, `I70_SUMMARY {"cases":[17,17],"mutations":[286,286],"must_pass":[28,28]}`. The listing test (`scripts/i70_probe_fn.rs`) makes the same comparisons as the contract tests, and was only ever in PROBE.

## 3. The suite comparison and the discrimination evidence

### 3.1 The full `result_export` suite, BASE against CAND

| Run | Inputs | Result |
|---|---|---|
| BASE (`d449097085`, archive) | corpus 07k `482449bf…`, test `c74c8c85…` | **172 passed, 0 failed, 0 ignored**, rc 0 (`logs/base_suite.log`) |
| CAND (`WT/f2a-u8`) | corpus 07l `5ac13296…`, test `7832a602…` | **173 passed, 0 failed, 0 ignored**, rc 0 (`logs/cand_suite.log`) |

**Test by test** (`scripts/compare_suites.py`; `logs/suite_comparison.tsv` and `.txt`):
- Every binary has the same per-test status in both runs. The `src` lib tests, including the `cfg(test)` module that `include_str!`s the corpus, pass 24 and 24. The other binaries pass 6, 5, 5, 10, 7, 4, 19, 16 and 14 in each.
- **The only difference is the added `retained_precision_contract::snapshot_07l_mutation_outcomes`** (absent → ok), which takes the contract test from 62 to 63.
- The outcome lines of the existing slices are byte-identical between the runs: 277 slice lines, all `match:true`. CAND adds exactly the 4 new `I63_MUST_PASS … true` lines (`logs/existing_outcome_lines_base_vs_cand.diff`).
- Both builds emit the same one lib warning (`function derived is never used`). It is in BASE too, and not mine.

### 3.2 Rust reader and test mutants (records only, in PROBE)

`scripts/mutate.py` and `scripts/run_probes.sh` ran each mutant through one PROBE test run of the contract binary plus the listing. The pristine files were restored afterwards and their sha256 checked (`logs/run_probes.out`). The reader mutants are the Rust analogs of I69's RM1–RM4, in `coverage_g5a` of `RE/src/retained_precision.rs`. The `src` hashes of each run are in `logs/probes/<name>.inputs`, and the analysis is in `logs/analysis/probe_summary.json`.

| Mutant | Contract tests that fail | Entries that mismatch | Any 07k entry? |
|---|---|---|---|
| none (control) | none (64 of 64) | none | — |
| **RM1**: A-exclusion removed (`if (0..4).any(\|k\| a[k] && !non_input[k]) { return false; }`) | `shared_rehashed_first_failure_mutations`, `snapshot_07l_mutation_outcomes` | `isolated_rotation_stop` ×2, `isolated_translation_rotation_stop` ×2 | **no** |
| **RM2**: feasibility coupled at L = 0 (`if l == 0.0 { a }` → coupled) | the two above, plus `shared_must_pass_entries_validate` | `isolated_translation_rotation_stop` ×2, `isolated_translation_stop` ×2 (I69's pair only) | **no** |
| **RM3**: hats coupled at L = 0 (`let hats = if l == 0.0 { e } …` → coupled) | the same three | `isolated_estimate_coupled` ×2, `isolated_estimate_uncoupled` ×2 | **no** |
| **RM4**: no-free-DOF rule removed (`fail(!has_data[bi])?`) | `shared_rehashed_first_failure_mutations`, `snapshot_07l_mutation_outcomes` | `isolated_has_data` ×2 | **no** |
| **TM-A** (corpus): `isolated_has_data_sparse_interactive` gets an `after_rehash` forged receipt digest and the expectation `G1 RETAINED_PRECISION_RECEIPT_MISMATCH`, so it is self-consistent | **only `snapshot_07l_mutation_outcomes`** | none, since the corpus agrees with itself | — |

The Rust kill matrix equals I69's Python matrix entry for entry. TM-A shows why the slice is needed: without it, a corpus whose L = 0 entry and expectation drifted together would pass the Rust contract test.

## 4. Stops

None fired:
- the Rust reader accepts both faithful 07l bases, with their eligibility, classifications and bounds;
- no production or reader `src/` change;
- no existing byte changed outside the fenced file;
- nothing was weakened;
- every write in CAND is inside the widened fence.

## 5. For ROOT

1. **The module doc comment (`:1`) changed.** I read it as a comment that 07l makes stale: the corpus is no longer only synthetic controls. The new wording mirrors I69's accepted Python docstring. If ROOT reads the fence as only `:206`, `:276–283` and `:835`, reverting that one line leaves everything else valid.
2. **A pre-existing gap, not 07l-made, left as is:** mutation index 277, `g7_not_required_quality_enum_invalid` (RV94 N-3's G7 probe, from 07k). It is in no slice tally: the slices cover 0..277 and now 278..286. Only `shared_rehashed_first_failure_mutations` observes it, against its corpus expectation (`G7 SOURCE_NUMERICAL_CASE_INVALID`, which the PROBE listing matches). A one-entry slice `277..278` would close the gap, but it lies outside my fence.
3. **A pre-existing comment imprecision, left as is.** The must-pass comment ("06d's 18 plus the equal-E bracket control; 07h …; 07j …") itemizes 21 of the 24 base entries. It omits entries 19–21 (`integral_float_integers_and_references`, `model_schema_version_0_1_0_accepted`, `verification_estimate_names_moment_row`). My 07l clause says "24 to 28", so it is self-consistent.
4. **A test name, left as is.** `complete_synthetic_controls_carry_their_shared_eligibility` now also checks the two producer-solved bases. Renaming it would change a test id, so I did not.
5. **For RV97's round 2 and I72's Pass B:**
   - the CAND build of the lib `cfg(test)` module sees the 07l corpus and passes 24 of 24, the same as BASE;
   - the new slice's tag is `I70_OUTCOME_07L`, beside the existing `I63_OUTCOME_*` and `I61_OUTCOME_07G/07H` tags.

## 6. Commands

The working directory is the crate directory of each tree. `ENV` = `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=WT/targets/i70-u8`.

1. **BASE:** `GIT_OPTIONAL_LOCKS=0 git -C WT/f2a-u8 archive d449097085 projects/chirality-piping | tar -x -C WT/scratch/i70_u8/base`. Then, in `BASE/P/core/reporting/result_export`: `ENV WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast -- --show-output` (rc 0, 03:19:38–03:21:26Z).
2. **CAND:** the five edits above. Then, in `WT/f2a-u8/P/core/reporting/result_export`, the same command (rc 0, 03:21:40–03:23:50Z).
3. **PROBE:** `git archive 69a925bd68` of the crate, `core/serialization/canonical_json`, `core/units`, `fixtures/results` and `schemas`; then CAND's test file with `scripts/i70_probe_fn.rs` appended (sha256 `b996e0c9…`). Then `WT=… scripts/run_probes.sh none RM1 RM2 RM3 RM4 TMA`, each step being `ENV WT/tools/t3_cargo.sh test --locked --offline --test retained_precision_contract -- --show-output` (03:23:53–03:28:23Z; rc 0 for none and 101 for each mutant, as intended).
4. **Analysis:** `scripts/compare_suites.py base_suite.log cand_suite.log suite_comparison.tsv`; `scripts/analyze_probes.py logs/probes logs/analysis`.

**Log sanitization.** The records' logs and scripts have the machine path of the T3 root replaced by `WT`, with `sed`. `run_probes.sh` takes `WT` from the environment. Nothing else in them was edited.

## 7. Cleanup and limits

- **Deleted on return:** `WT/scratch/i70_u8/` (BASE, PROBE, the pristine copies, the working logs) and `WT/targets/i70-u8/`. CAND keeps my one uncommitted file for ROOT.
- **Limits:**
  - the full-07l per-entry listing comes from PROBE (CAND's test file plus the appended listing test, on the same `src` and corpus bytes), not from the candidate binary itself. The candidate binary's own outcome is the 173-test pass and its printed slice and must-pass lines;
  - the reader mutants are in-file edits in a scratch copy, not a mutation-testing tool;
  - only `result_export` was run. Python and TS are I69's and I71's.
