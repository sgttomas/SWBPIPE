# I71 U8-3: the TypeScript alignment to corpus 07l

TASK (Type 2), I71, for ROOT (HELP_HUMAN, Agent 0). 2026-10-06 UTC. Run 1 03:20–03:31Z; a host interruption; run 2 (resume) 12:38–12:52Z.

**Briefs (verified before work):** `R/BRIEFS/U8_COMMON.md` sha256 `3146c3e6c2d0940cfe1870da75192db4b37e2a5c7f690bc3cc5acba953eb223b`; `R/BRIEFS/I69_I70_I71_U8_CORPUS_AND_READERS.md` sha256 `acb04c2596ae1fdd96fff5fb4b13babba0472184cd14a1c6e723c096cf4962e7` (the I71 section). **Basis read:** `NUM/AGENTS.md`, `NUM/agents/AGENT_TASK.md`, `NUM/P/AGENTS.md`; `R/I69/u8_corpus_07l_01/RETURN.md`; RR "I76's return verified; RV97 passes U8's round 1; erratum E-6" (N-1); RR "I69's corpus 07l committed; I70 and I71 dispatched"; RR "I70's Rust alignment committed; an interruption, and three TASKs resumed"; `R/REVIEW_RV97/u8_01/REVIEW.md` N-1.

**Placeholders:** `WT`, `NUM`, `P`, `DT`, `R`, `RR` as in the dispatch; `NMS` = the node_modules source named in the dispatch; `CAND` = `WT/f2a-u8`; `ARCH` = `WT/scratch/i71_u8/base`, a disposable `git archive` of `d449097085` (`P` only).

## 0. The interruption, and what each run covers

- **Run 1** ran on CAND at the dispatched U8 head `69a925bd68` plus my file, with I70's Rust file uncommitted beside it. It completed every check; its evidence is `_run_records/logs/`. The host's Claude process ended while I was deleting scratch, after the records were copied (ARCH was half-deleted; CAND was already clean).
- **On resume** I re-verified the state: CAND's HEAD is now `de01e43bc4` (ROOT's commit of I70's file; `git diff 69a925bd68 de01e43bc4` touches only `P/core/reporting/result_export/tests/retained_precision_contract.rs`); `git status` shows only my file; my file's sha256 is still `59531ed1…` and its diff is byte-identical to `_run_records/candidate_test.diff`; the corpus is still 07l `5ac13296…`; the TS reader is unchanged (`c3eadc8a…`); run 1's logs are whole (both suite runs ended `rc=0`, `mutants.jsonl` has all 9 rows).
- **Run 2** repeated every check on CAND at `de01e43bc4` plus my file, with a fresh ARCH and the same wasm assets (`_run_records/resume/`). Mutants were not repeated: the three files they touch are byte-identical to run 1's.

## 1. The changed file (CAND; inside the fence)

| File | sha256 before (`d449097085` = `69a925bd68` = `de01e43bc4`) | sha256 after | Change (`git diff --numstat --histogram`) |
|---|---|---|---|
| `DT/features/results/retainedPrecision.test.ts` | `385a626a1e3d36e90511964872a00cfae655f19f09578b564f79c13cca3656e3` | **`59531ed1ecf9aff190d9501a0eb120113380e36a2c37a25effd88f007629c287`** | **+45 / −0** (`_run_records/candidate_test.diff`) |

The addition is one `describe`, "07l round (U8-2): counts and the appended producer-solved L = 0 entries", appended at the end of the file. Nothing existing changed. It has four tests:

1. **The count pins, equal to Python's and Rust's:** 17 cases, 286 mutations and 28 must-pass entries; and 15 eligible bases and 18 eligible must-pass entries (Python's `(15, 18)` pin).
2. **The two appended bases** (cases 15–16, ids pinned): `provenance.kind` is `producer_solved`, each is eligible with its invocation, and their class counts are I68's three-reader observation, **25/78/9/1** (sparse) and **25/78/9/2** (dense). The classifications equal the corpus's.
3. **The 8 appended mutations** (278–285): ids and order pinned (I69's order), and each base pinned from its id's mode. Each entry states G5a `RETAINED_PRECISION_SCALE_MISMATCH` and has no `expected_by_reader`. Each is applied and observed refused at exactly that gate and code.
4. **The 4 appended must-pass entries** (24–27): ids, order and bases pinned. Each states `pass` with eligible standing, and each is observed admitted, eligible, with its base's classifications.

**Why tests 2–4 add something.** The per-entry tests above them already run every 07l entry, but against the expectation the corpus itself states (`m.expected_by_reader?.typescript ?? m.expected`). They pass if an L = 0 entry is dropped or reordered. They also pass if an entry is re-expected behind a TypeScript override. The new tests pin those outcomes in the test file (§3.2). The expected values are written in the test, not read from the corpus. No assertion is deleted or loosened, and the reader's `src` is unchanged.

## 2. The full 07l under vitest

| Run | Tree | `retainedPrecision.test.ts` |
|---|---|---|
| Unedited file (pre-check) | CAND at `69a925bd68` | **470 passed** (`logs/u8head_unedited_rp.log`) |
| Run 1 | CAND at `69a925bd68` + my file | **474 passed** (`logs/candidate_rp.log`) |
| Run 2 | CAND at `de01e43bc4` + my file | **474 passed** (`resume/candidate_rp.log`) |

**Breakdown, per corpus item** (`logs/full_07l_outcome.txt` and `resume/full_07l_outcome.txt`, the same text):
- **Case tests:** 51 of 51 (3 per base, all 17 bases). Both producer-solved bases are accepted, eligible, with their classifications; they stay needs-recompute without the invocation; and they snapshot their inputs.
- **Mutations:** 286 of 286 are refused at their TypeScript expectation. All 8 L = 0 mutations are refused at **G5a SCALE**.
- **Must-pass entries:** 28 of 28 are admitted with their stated eligibility and their base's classifications. All 4 L = 0 entries are eligible.
- **Other tests in the file:** 109 of 109, including the 4 new ones.

**The TS reader accepts both faithful 07l bases**, so the STOP did not fire.

## 3. Discrimination evidence (run 1, records only; in ARCH with 07l and my file installed by sha256)

`_run_records/scripts/mutants.py` → `logs/mutants.jsonl`. Each mutant edits ARCH's copy only, runs this test file, and is then restored and sha-checked (`restored True`). The control: 474 of 474 pass.

### 3.1 Reader mutants: TS analogs of I69's RM1–RM4 in `retainedPrecision.ts`

| Mutant | Failing tests | Killed by any pre-07l test? |
|---|---|---|
| RM1: the A-exclusion removed (in `stopFeasible`) | `isolated_rotation_stop` ×2 and `isolated_translation_rotation_stop` ×2; new test 3; the existing `stopFeasible` unit test | yes, the unit test (the mutant edits the exported helper itself) |
| RM2: feasibility coupled at L = 0 (the `selectedCoverage` call) | `isolated_translation_rotation_stop` ×2 and must-pass `isolated_translation_stop` ×2; new tests 3 and 4 | **no** |
| RM3: hats coupled at L = 0 | `isolated_estimate_coupled` ×2 and must-pass `isolated_estimate_uncoupled` ×2; new tests 3 and 4 | **no** |
| RM4: the no-free-DOF rule removed | `isolated_has_data` ×2; new test 3 | **no** |

### 3.2 Corpus mutants: what only the new pins catch

| Mutant | Failing tests |
|---|---|
| TM1: the last must-pass entry dropped | new tests 1 and 4 only |
| TM2: the dense base and its 6 entries dropped | new tests 1–4 only |
| TM3: `isolated_has_data_sparse_interactive` re-expected as G5 ATTEMPT, behind a TypeScript override of G5a SCALE | new test 3 only |
| TM4: mutations 278 and 279 swapped | new test 3 only |

## 4. The desktop vitest suite, base against candidate, test by test

The base is ARCH (`d449097085`). The candidate is CAND. The tests are keyed by (file, full name, occurrence) from vitest's JSON reports (`scripts/compare_suites.py`).

| | Base | Candidate | Differences |
|---|---|---|---|
| **Run 1** (candidate `69a925bd68` + my file) | 138 files, **3,552 passed**, 0 failed | 138 files, **3,574 passed**, 0 failed | **22, all added and passing:** the 18 per-entry 07l tests (2 bases × 3, 8 mutations, 4 must-pass) and my 4. All in `retainedPrecision.test.ts`; no removal, rename or status change (`logs/suite_comparison.txt`, `logs/suite_differences.tsv`, and the full lists `logs/suite_tests_{base,candidate}.tsv`) |
| **Run 2** (candidate `de01e43bc4` + my file) | 3,551 passed, **1 failed** | **3,574 passed**, 0 failed | the same 22 additions, plus that 1 base failure (`resume/suite_comparison.txt`) |

**Run 2's base failure is a load timeout, not a difference.**
- The test is `src/App.test.tsx` › "renders the engineering workspace from invented local fixtures". It failed with "Test timed out in 30000ms" at 30,616 ms (`resume/suite_base_failure.txt`).
- The host was loaded: another TASK's vitest mutant runs were active, and the load average was 18.8. The base suite took 244 s, against 146 s in run 1.
- The same test passed in run 1's base, and in both candidates.
- Run alone on the same ARCH right after, `App.test.tsx` passes **225 of 225**, and this test takes 6.0 s (`resume/base_app_rerun_summary.txt`, `resume/base_app_rerun_tests.tsv`). `vite.config.ts` records this class of load timeout for `App.test.tsx`.

**Run 2 against run 1** (`resume/suite_run2_vs_run1.txt`): every candidate row is identical. The base rows differ only in that one test's status. The run-2 lists' hashes are in `resume/suite_tests_run2.sha256`; their full text is not kept, being run 1's plus that one row.

## 5. tsc

`tsc --noEmit -p tsconfig.json` in `P/apps/desktop`, TypeScript 5.9.3: **rc 0, no output**, on the candidate and the base, in both runs (`logs/tsc_*.log`, `resume/tsc_*.log`).

## 6. The wasm assets (erratum E-6)

**Copied, not built**, from `WT/sweep-skewpin/P/apps/desktop/public/`. The same eight files were used for base and candidate in both runs, `cmp`-identical by sha256 (`logs/wasm_assets_{source,candidate,base}.sha256`; `resume/` the same):

```
af2b07abf12dac915fb0e90124135d578eb7c2b473ec402dbe285900608cb8a8  wasm-engine/open_pipe_stress_operation_applier.d.ts
5682432840e2199512059d7956dfdb51521179b797145a05835a5bfa0be71595  wasm-engine/open_pipe_stress_operation_applier.js
b95005290fe1f67a37699fba29981f2958e2d1190e5a7a4e13fd9d5382015710  wasm-engine/open_pipe_stress_operation_applier_bg.wasm
37a68fc929d21e62fad7683b94f26b7233691df567956908f345c257c8a3ae16  wasm-engine/open_pipe_stress_operation_applier_bg.wasm.d.ts
634546fac35bc42da31bf903e52a5799429422f44291d901495675bb4da4cc01  self-weight-engine/open_pipe_stress_self_weight_wasm.d.ts
ea2b3fd611a6478222e552b1230ceb717abd60cdd6df5ba30b4827511543f8f3  self-weight-engine/open_pipe_stress_self_weight_wasm.js
474667ecabdeb465e9406799cfcae9fc5f7ed49ad91153b82b2fe07b4f38f319  self-weight-engine/open_pipe_stress_self_weight_wasm_bg.wasm
dd78c7c8fa07894717b89a9bcda4d781200ca7a4d02a8dc153b7763290085c7b  self-weight-engine/open_pipe_stress_self_weight_wasm_bg.wasm.d.ts
```

They are the same eight (hash, file) pairs that I68, RV97 and I69 recorded (`logs/wasm_assets_vs_earlier_records.txt`).

**The revision they were built from: `e2b83da584`** (`git -C WT/sweep-skewpin rev-parse HEAD` = `e2b83da5841e749e90ba54af6d4a45971693bc4b`; `logs/wasm_assets_provenance.txt`).
- That worktree is clean.
- Its HEAD reflog's last move is the checkout of `e2b83da584` at 2026-10-05T08:10:02−06:00. The assets were written at 08:47:13 (`wasm-engine`) and 08:47:29 (`self-weight-engine`), after that checkout, and HEAD has not moved since.
- No build record names the run that wrote them. This attribution rests on the reflog and the file times.

**Are the engine sources at `e2b83da584` equal to the U8 head's?** The build script compiles `core/model_operations/operation_applier` and `core/loads/self_weight_wasm`. Their path-dependency closure is 17 crates (`scripts/engine_closure.py`, Cargo.toml read with `tomllib`, no cargo run; the same at both revisions). Within that closure, plus `apps/desktop/scripts`, `schemas` and `fixtures` (`logs/engine_source_diff.txt`, `resume/engine_source_diff_de01e43bc4.txt`, `logs/engine_test_only_evidence.txt`):

- **`e2b83da584` → `b1e2d7741e`:** no difference (RV97's finding).
- **→ `d449097085` (the base) and → `69a925bd68`:** the differences are `P/core/product_physics/src/retained_facade_tests.rs` (+244), which `lib.rs:128–129` compiles only under `#[cfg(test)]`, and three fixtures. Those fixtures are the corpus and the two L = 0 successors. They are referenced only from that test module, from `result_export`'s `#[cfg(test)] mod u6e_reader_round_tests` (`src/retained_precision.rs:4349–4353`), and from `result_export/tests/`.
- **→ `de01e43bc4`:** the same, plus I70's `result_export/tests/retained_precision_contract.rs`, an integration test outside the lib target.
- No `Cargo.toml`, `Cargo.lock` or `build.rs` in the closure changed. None of PP's 14 `REVIEWED_INPUTS` (the files `build.rs` digests) changed.

**So, for the release wasm32 build, the engine sources are equal** at `e2b83da584`, `d449097085`, `69a925bd68` and `de01e43bc4`. The trees are not byte-identical, but they differ only in test-only files and test-only fixtures.

## 7. Host rules kept, and side effects

- **Writes in CAND:** exactly the fenced file, left uncommitted.
- **Placed temporarily in CAND:** the untracked symlink `P/node_modules` → NMS, and `P/apps/desktop/public/{wasm-engine,self-weight-engine}`, which are Git-ignored. `public/` did not exist before.
- **Created by vitest in CAND:** its results cache, `P/apps/desktop/node_modules/.vite/vitest/…/results.json` (Git-ignored; Vite's default `cacheDir` under the package root). All three were removed in each run. At return, `git status --short` shows only my file (`_run_records/candidate_status.txt`), and `--ignored` shows nothing under `P/apps`.
- **In NMS:** vite passes its transient config through the pre-existing `NMS/.vite-temp/`, which is empty again (only its mtime moved). `NMS/.vite` was never created. NMS's own directory is untouched: same mtime (Oct 3) and link count (`logs/shared_node_modules_{pre,post}.txt`).
- **Not done:** no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no cargo, no DEC-025, native or solver job. `TMPDIR` was under `WT/scratch/i71_u8/`, and nothing went to the system temp directory.
- **The interruption's partial deletion of ARCH:** the `node_modules` symlink was unlinked before any `rm -rf`. The archive's own committed symlinks (104, pointing into a long-gone worktree) were unlinked, never followed.
- **Where this file was written.** The resumed session's host write guard refused to write RETURN.md into NUM: it names a different worktree as this session's own. I did not work around the guard. So I wrote RETURN.md and SHA256SUMS in my session scratchpad, and ROOT places them in `R/I71/u8_ts_07l_01/` unchanged.
- **Written into NUM before the guard showed itself:** `_run_records/` (run 1's copy, and run 2's after the resume), and CAND's cleanup. Both were done with shell commands, before the guard appeared.

## 8. Stops

None fired.
- TS's reader accepts both faithful 07l bases.
- No production or reader `src` change was needed or made.
- No existing byte changed outside the fence, and nothing was weakened.

## 9. For ROOT

1. **The asset revision is inferred, not built** (§6). The hashes equal I68's, RV97's and I69's, and the engine sources are equal for the release build. If ROOT wants a build-from-source witness, it is a cargo job (wasm32 target, wasm-bindgen CLI 0.2.123, `npm run build:wasm`) for a run that is allowed cargo, comparing the hashes above.
2. **Run 2's base `App.test.tsx` timeout** (§4) is recorded as host load, not a candidate difference. It passed in run 1 and when re-run alone. No action proposed.
3. **Test 2 also pins the bases' `provenance.kind` and I68's class counts.** That goes slightly past "the 8 mutations and 4 must-pass entries", but stays within "counts and pins"; Python pins both. For RV97's round 2 to confirm, or for ROOT to trim.
4. **Not mirrored in TS:** Python's D-U6-5 file-identity check (the fixture sha256 and value equality). It reads PP's fixture files, which is more than counts and pins, and Python already covers it.
5. **The write guard** (§7): ROOT places RETURN.md and SHA256SUMS. Future resumed TASKs on this host may meet the same refusal if their session's own worktree is not WT's.

## 10. Commands (cwd `CAND/P/apps/desktop` or `ARCH/P/apps/desktop`; `E` = `TMPDIR=WT/scratch/i71_u8/tmp`)

1. `GIT_OPTIONAL_LOCKS=0 git -C CAND archive d449097085 projects/chirality-piping | tar -x -C ARCH/..`
2. For each of CAND and ARCH: `mkdir -p P/apps/desktop/public && cp -R WT/sweep-skewpin/P/apps/desktop/public/{wasm-engine,self-weight-engine} P/apps/desktop/public/ && ln -s NMS P/node_modules`; then the sha256 checks.
3. `E ../../node_modules/.bin/vitest run --reporter=verbose --reporter=json --outputFile.json=<log> src/features/results/retainedPrecision.test.ts`
4. `E ../../node_modules/.bin/tsc --noEmit -p tsconfig.json` (both trees).
5. `scripts/run_suites.sh WT/scratch/i71_u8 CAND` (base, then candidate: `vitest run --reporter=default --reporter=json`), then `scripts/compare_suites.py`.
6. Run 1 only: `scripts/mutants.py ARCH/P <logs> <tmp>`, after `cp` of CAND's corpus and test file into ARCH (`logs/arena_installed.sha256`).
7. Run 2 only: `E ../../node_modules/.bin/vitest run --reporter=default --reporter=json src/App.test.tsx` in ARCH.
8. In `CAND/P` and `WT/sweep-skewpin/P`: `python3 scripts/engine_closure.py`; then the `git diff --stat` commands in the engine-diff logs.

## 11. Cleanup and limits

- **Removed at return:**
  - from CAND: `P/node_modules`, `P/apps/desktop/public/` and `P/apps/desktop/node_modules/`;
  - all of `WT/scratch/i71_u8/` (ARCH, logs, tmp).
- No target directory was created.
- **Limits:**
  - The asset build revision is inferred (§6).
  - The class counts and outcomes pinned in my test come from I68's three-reader observation and I69's corpus, not from an independent oracle.
  - Run 2 kept only hashes of its per-test lists (§4).
  - The suites ran beside other TASKs' vitest jobs on the same host.

## Files

- `RETURN.md`, `SHA256SUMS`.
- `_run_records/candidate_test.diff`, `candidate_status.txt`, `changed_files_sha256.txt`.
- `_run_records/scripts/`: `run_suites.sh`, `compare_suites.py`, `mutants.py`, `engine_closure.py`.
- `_run_records/logs/` (run 1):
  - the pre-state and NMS checks;
  - the two `retainedPrecision` runs and `full_07l_outcome.txt`;
  - the suite runs, summaries, full per-test lists, comparison and differences;
  - tsc;
  - `mutants.jsonl` and `arena_installed.sha256`;
  - the engine closure, diffs and test-only evidence;
  - the wasm asset hashes, provenance and the cross-check with earlier records.
- `_run_records/resume/` (run 2):
  - the pre-state, the `retainedPrecision` run and `full_07l_outcome.txt`;
  - the suite runs, summaries, comparison, differences, the run-2-against-run-1 diff, and the run-2 list hashes;
  - the base failure and its isolated re-run;
  - tsc;
  - the engine closure and diff at `de01e43bc4`;
  - the wasm asset hashes.
