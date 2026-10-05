# RV95 addendum 03: item 6 and the final read-only confirmation of the merge candidate F′

**Reviewer:** RV95, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0); ROOT is the return path; no descendants. `REVIEW.md` (`2918e063…`), `ADDENDUM_01.md` (`92ff6e27…`) and `ADDENDUM_02.md` (`51464018…`) are unchanged.

**Candidate:** **F′ = `5488136a193ef8921bd88c33a8124647c4cb352d`**, the PR #1082 head (checked with `gh pr view`). Its base, main, is `5fdc5ab6012ddb50ecf286621a291b4da577405e`; the GitHub API read gives main at that SHA, so main is unmoved.

**Host:**
- Run after ROOT's all-clear, one cargo job at a time, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`.
- A fresh `git archive` of F′ (P/core, fixtures and schemas; 1,034 of 1,034 blobs verified) in `WT/rv95/`, target `WT/targets/rv95/ni`, `TMPDIR` under scratch.
- No Git writes. The memory guard (PID 5387) was running, and its log has no KILLED line.
- The copy and target are deleted.

## Verdict: **CONFIRMED.** Nothing in my review blocks merging #1082 at F′.

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| New NOTE | 2 (C-1, C-2; neither needs a change to F′) |

Across `REVIEW.md` and addenda 01–03, every SHOULD-FIX (S-1, S-2) is repaired and confirmed, and no BLOCKING finding was ever raised. Required CI passes on F′ (§3), and the review and validation cover the actual candidate. The merge policy's conditions are therefore met on my side.

The owner holds two items outside my verdict:
- the native witness (G10) is recorded as outstanding by the owner's decision;
- main's merge hold is ROOT's to release.

## 1. Item 6: the walker repair, executed

**rustc against the walker, on a synthetic tree.** The tree is `evidence/confirm_03/synth2/`, 14 files. It covers:
- a mod-rs root and a `mod.rs`;
- non-mod-rs files;
- inline modules with and without `#[path]`;
- doubly nested inline modules;
- a `#[cfg(test)]` inline module holding a plain and a `#[path]` declaration;
- brace-bearing char and string literals inside inline modules.

`rustc --emit=dep-info` lists the source files rustc actually reads:

| Build | rustc reads | Walker |
|---|---|---|
| without `cfg(test)` | 12 files | classes exactly those 12 as non-test (`a.rs`, `a/inl/c.rs`, `a/inl/q.rs`, `b/m/d.rs`, `b/mod.rs`, `b/z.rs`, `c.rs`, `c/w/e.rs`, `c/x/y/deeper.rs`, `lib.rs`, `outer/inner/deep.rs`, `side.rs`): **equal** |
| with `--cfg test` | 14 files: the same 12 plus `t/g.rs` and `t/pp.rs` | reaches all 14 (its every-file assertion holds) and classes those two as test: **equal** |

The walker therefore matches rustc's resolution on every form the brief names, and it takes test-ness from declarations only.

**nonlinear_integration's suite on F′:** **134 passed, 0 failed, 0 ignored** (130 unit tests and 4 doc-tests). That equals M's 134 in DEC-025, including both `s11k_tests` that failed on F.

**Mutants.** Each is one exact edit, restored and hash-verified afterwards.

| # | Mutation | Result |
|---|---|---|
| W1 | Walker ignores enclosing inline modules for plain declarations | **Killed**: `k2b_…` and `kd5_…` fail (the `grant2` panic) |
| W2 | K2b site pin reverted to `fn solve_load_case` | **Killed**: `k2b_…` fails |
| W3 | `#[path]` inside inline modules resolved from the file's directory | **Survives** the crate's tests (no PP or nonlinear_integration declaration uses the form); **killed** by my synthetic probe, `…/synth2/src/pp.rs: No such file`. See C-2 |
| W4 | PP's wrapper `solve_load_case` names `ForceScale` (text only) | **Killed**: `k2b_…` fails, so exact equality enforces that the wrapper names none |
| W5 | Test-ness taken from the path rule rather than the declaration (`grant2` becomes non-test) | **Killed**: `k2b_…` and `kd5_…` fail |

**A pre-existing limit, found by my first synthetic tree (C-1).**
- My first tree put a string literal (`"}{"`) and a `#[path = "q.rs"]` declaration on one physical line.
- The walker resolved the module to `a/inl/}{`, because it takes the path value as the first string literal on the raw line that contains `#[path`.
- That parser is unchanged from M, and no PP or nonlinear_integration source has a string literal before a `#[path` on the same line.
- I rebuilt the tree with the literal on its own line; that is `synth2`. Both versions are recorded (`logs/probe_Fp.summary`, `probe2_Fp.summary`).

## 2. DEC-025 on F′: CONFIRMED

DEC-025 on F′ is `WT/scratch/u9_dec025/Fp/`. It started at 00:33:23Z at head `5488136a19`, finished its suites at 00:46:15Z, and was all done at 01:05:26Z. I compared it per manifest and per test with my own script (`evidence/confirm_02/dec025_compare.py`; outputs `dec025_M_vs_Fp.json`, `dec025_F_vs_Fp.json`).

**Against M** (40 manifests each side; 2,349 tests in M, 2,669 in F′):
- **34 manifests are identical,** nonlinear_integration included (134).
- **The other six differ only by added tests:**

  | Manifest | Added tests |
  |---|---|
  | PP | +145: 136 ok, 9 ignored (U8's `witness_*`) |
  | result_export | +80, ok |
  | runner/headless | +3, ok |
  | frame_kernel | +93, ok; plus the same three doc-test renames from the +22-line shift |
  | performance_harness | +1, ok |
  | numerical_robustness | +1, ok |

- **The failing set is exactly M's:** PP `t13` and runner's two `load_reference` tests.
- **The fail-fast evidence sweep** stopped at PP `t13` (542 passed, 1 failed, 10 ignored), as at every Mac DEC-025.

**Against F:** only nonlinear_integration changes. `k2b_force_scaled_entries_are_reached_by_neither_the_loop_nor_the_product` and `kd5_nonlinear_sources_name_no_formation_check_entry_point` go from FAILED to ok, and every other test outcome is identical.

**The other surfaces** (`Fp/surfaces.txt`), all exit 0:
- pytest: 3,540 passed, 32 skipped;
- `build:wasm:desktop`;
- vitest: 138 files, 3,552 tests passed;
- `build:desktop`.

ROOT's reading is exact.

## 3. The remaining gate evidence on F′

**Hosted CI.** Both runs are "Piping Desktop E2E", completed, conclusion success, on head `5488136a19`, with 7 jobs succeeded and 1 skipped (the accessibility barrier). The succeeded jobs are Select source coverage, the four Source remainder shards, the Numerical cargo suite, and Desktop E2E (source mode).
- **pull_request run** 37247819786.
- **Full-SHA dispatch** 37247819679 (`workflow_dispatch`). Its plan log records `"target_base": "5fdc5ab6012ddb50ecf286621a291b4da577405e"`.
- **`gh pr checks 1082`:** 12 pass and 4 skipping, 0 failing or pending. That count is the PR's check view; ROOT's 19 counts across both runs.
- **The PR** reads `MERGEABLE` / `CLEAN`. It is still a **draft**.

**Pass B on F′:**
- I65's no-build gates are recorded in `R/I65/u9_refreeze_01/` (SHA256SUMS 26/26 OK). All gate codes are 0, and the delta inventory adds exactly one `test` row, for `s11k_tests.rs`.
- RV89's ADDENDUM_02 (`89403fc2…`; `R/REVIEW_RV89/u4_g7_03/` SHA256SUMS 25/25 OK) gives **PASS**.

**Carried over, as confirmed in ADDENDUM_02 §3:** G5–G8, G9b and T9. GEN-8 on F′ is ROOT's record (1 passed).

## New NOTEs

| # | Where | Evidence | Remedy |
|---|---|---|---|
| C-1 | `s11k_tests.rs` `target()`, the `#[path]` reader `attr.split('"').nth(1)` | It takes the first string literal on the raw line containing `#[path`. A string literal earlier on that line, as in my first synthetic tree, misleads it. The behaviour is the same at M, and no source the walker scans has that shape. | Optional, at the walker's next touch: parse the attribute's own literal. |
| C-2 | `s11k_tests.rs` `target()`, the inline `#[path]` branch | W3 survives the crate's own tests, because no scanned declaration exercises that branch. Only my synthetic probe kills it. | Optional: add a synthetic-tree control, like `synth2` and `zz_rv95_walker_probe`, to the walker's own controls. |

## For ROOT (no ruling needed)

Immediately before the merge, as already planned:
- confirm `origin/main` is still `5fdc5ab601` (it was at my read);
- mark the PR ready (it is a draft);
- merge with `gh pr merge --merge --match-head-commit 5488136a193ef8921bd88c33a8124647c4cb352d`;
- verify both merge parents;
- record G10 as outstanding and A-1/A-2/A-3/B-1/C-1/C-2 as notes in the merge record.

## Records

`evidence/confirm_03/` (placeholder paths only) holds:
- `after_allclear.json`, the run's summary;
- `probe2.json` and `run_probe2.py`;
- `rustc_synth2_{plain,cfg_test}.files`, from rustc's dep-info;
- `synth2/`;
- `dec025_M_vs_Fp.json` and `dec025_F_vs_Fp.json`;
- `logs/`: the nonlinear_integration outcomes (134), the probe and mutant summaries, and the run log.

The scripts are in `evidence/confirm_02/`. `SHA256SUMS` covers the `u9_01` folder. **Cleanup done:** `WT/rv95/` and `WT/targets/rv95/` are deleted; scratch logs remain in `WT/scratch/rv95_u9_01/`.
