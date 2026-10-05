# RV95 addendum 02: the walker repair and the refrozen head F′ (read-only part)

**Reviewer:** RV95, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0); ROOT is the return path; no descendants. I am the independent check on a ROOT-authored repair. `REVIEW.md` (`2918e063…`) and `ADDENDUM_01.md` (`92ff6e27…`) are unchanged.

**Candidate:** **F′ = `5488136a193ef8921bd88c33a8124647c4cb352d`**, the PR #1082 head (checked with `gh pr view`), on M `5fdc5ab601`. Since F = `20dd3d929d`:
- `6cfe50d368` is the repair, byte-identical (blob) to NUM `42009dba72`. That commit is on `origin/codex/piping-numerical-integrity-20260926`, and `bb3d766379` is its ancestor.
- `5488136a19` updates the package: CHANGE_RECORD, PR_BODY and SHA256SUMS.

**Basis:** RR "DEC-025 on F finds a test-walker defect; repaired and refrozen as F′ = 5488136a19" (NUM `104ebc183e`).

**Host:**
- This part is **read-only**: records, Git reads with `GIT_OPTIONAL_LOCKS=0`, the two package scripts, and my per-test comparison of the DEC-025 logs. No cargo, rustc or test run, as ROOT's host note requires (DEC-025 on F′ is running).
- **Item 6 has not run.** It waits for ROOT's all-clear, and this run ended before that came.
- No Git writes.
- **Disclosure:** one read-only command wrote `git show` output of PP `lib.rs` to the system temp directory as `/tmp/rv95_never_used`. I removed it at once and verified it is gone; nothing else went there. The intended files were under scratch.
- My F′ copy (`WT/rv95/`, 1,034 of 1,034 core/fixtures/schemas blobs verified) is deleted.

## Verdict on items 1–5: **CONFIRMED**. Item 6 is **PENDING**.

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| New NOTE | 3 (B-1 to B-3) |

## 1. The walker (`s11k_tests.rs`, `non_test_modules`): CONFIRMED by reading

**rustc's rule.** The Reference ("Modules", "The path attribute") fixes where a module declared inside inline modules lives:
- **Without `#[path]`:** the base directory is the file's own directory for a mod-rs file (`lib.rs`, `mod.rs`), and `<dir>/<stem>/` for any other file. One directory is added per enclosing inline module, then `name.rs` or `name/mod.rs`.
- **With `#[path]` outside inline modules:** the path is relative to the file's own directory.
- **With `#[path]` inside inline modules:** the path is relative to the same base plus the inline-module directories.

**The repair implements exactly this:**
- `target()` folds `enclosing()`'s names onto `child_dir(file)`. `child_dir` gives the parent for `lib`/`mod` and the stem directory otherwise.
- Its `#[path]` branch uses `file.parent()` only when no inline module encloses the declaration.
- PP's `mod grant2;` (`lib.rs:3296`, inside `#[cfg(test)] pub(crate) mod retained_tests_hooks {`, `:3175`) therefore resolves to `src/retained_tests_hooks/grant2.rs`, as rustc does.

**`enclosing()` is sound on lexed code:**
- `lex` drops comments and blanks string and char literal contents (including `'{'`, raw strings and byte strings), so the remaining braces are code braces.
- A `{` is an inline module only when the identifier before it is preceded by the keyword `mod` on a word boundary.
- Every other brace pushes `None`, and every `}` pops.

**Nothing is weakened:**
- No assertion is removed or loosened.
- Test-ness still comes only from declarations: `child_test = test || !non_test.contains(&child)` is unchanged, and `non_test` comes from the `#[cfg(test)]`-stripped code. `grant2` is therefore a test module, because its enclosing module's attribute strips it.
- The "every `.rs` under `src` is reached" assertion is unchanged.
- The doc comment is accurate.

**The other source readers in the tree are not affected.** `k5_tests.rs:903` walks the directory, not the module tree. PP's `s11f_site_test.rs` and the law tests use `include_str!`. All of them passed in DEC-025 on F.

## 2. The K2b pin rename: CONFIRMED

`ForceScale` occurrences, counted by body:

| Revision | `fn solve_load_case` (wrapper) | `fn solve_load_case_observed` |
|---|---|---|
| M | 1 | — |
| `52842022cc^` | 1 | — |
| `52842022cc` | 0 | 1 |
| F′ | 0 | 1 |

- The pin now names the true site, with the same token and count.
- `k2b_…` compares the complete `(module, top-level item, token, count)` list with `assert_eq!(found, declared)`. A `ForceScale` in the wrapper would add a `("lib.rs", "fn solve_load_case", …)` row and fail, so exact equality enforces that the wrapper names none.
- `top_level_item` names items by keyword and first identifier, so the two functions cannot be confused.
- **History:** the walker panic dates from `664f8df7b7` (U3 grant 2's `mod grant2;`), and it masked the site defect from `52842022cc`.

## 3. The carry-over premise: CONFIRMED

- nonlinear_integration `lib.rs:10–11` declares `#[cfg(test)] mod s11k_tests;`.
- nonlinear_integration does not depend on PP. Its manifest lists curved_bend, frame_kernel, nonlinear_supports, diagnostics and sparse_direct; it reads PP's source as text.
- PP's build, the release harnesses (T9, the probe) and src-tauri compile nonlinear_integration only as a non-test dependency, so they never compile `s11k_tests`.
- `git diff F F′` outside execution is exactly that one file, so PP's sources, its lock and its 14 `REVIEWED_INPUTS` are unchanged.
- **G5–G8, G9b and T9 carry over to F′.** G9a's frozen-head Pass B on F stands, with the no-build rerun on F′ as ROOT ruled.

## 4. The package at F′: CONFIRMED

- **`source_equality.py`** (the package's own, `--pr 5488136a19 --int 42009dba72 --main 5fdc5ab601`): **5/5 PASS**. |S| = 140; 138 paths identical in blob and mode; `compatibility.py` (one conflict, the rule holds) and `source_blocks.rs` (clean) equal their merges; the execution files are exactly the 10 package files, and their SHA256SUMS verify.
- **`check_citations.py`:** 368 / 0 / 0, PASS.
- **Recomputed at F′:** 140 files, 59 added / 81 modified, 12,016,302 B, +204,707 / −908 lines; package 10 files, 194,107 B. All equal the package's statements.
- **The new text is true:**
  - the §3 sixth row: "five commits carried six source changes" counts the PR's commits after the cut;
  - the §7 DEC-025 row;
  - the status block's heads, `6cfe50d368` and NUM `42009dba72`;
  - the RR title it cites exists at NUM `11769`.
- **The post-merge items** say "the native witness … recorded". The owner's decision records it as outstanding, which is consistent.

## 5. The DEC-025 comparison on F: CONFIRMED

My own per-manifest, per-test comparison of `WT/scratch/u9_dec025/M/suites` against `F/suites` is in `evidence/confirm_02/dec025_compare.py` and `dec025_M_vs_F.json`. The baseline tree `base-m` is at `5fdc5ab601` with a clean status, and F's run recorded head `20dd3d929d`. There are 40 manifests on each side, 2,349 tests in M and 2,669 in F.

| Manifest | M | F | Difference |
|---|---|---|---|
| **33 others** | | | identical |
| product_physics | 569 / 1 / 1 | 705 / 1 / 10 | +145 added: 136 ok, and 9 ignored that are exactly U8's `witness_*`. FAILED is `t13` on both |
| runner/headless | 82 / 2 | 85 / 2 | +3 added, ok. The same two `load_reference` tests fail on both |
| result_export | 92 | 172 | +80 added, ok |
| performance_harness | 92 | 93 | +1 added, ok |
| numerical_robustness | 60 | 61 | +1 added, ok |
| frame_kernel | 456 / 0 / 1 | 546 / 0 / 1 | +93 added, all ok. The 3 removed are doc-tests renamed by the +22-line shift in `structural.rs` (lines 90, 96, 1909 → 112, 118, 1931); I counted the shift from the hunks 12 + 1 + 6 + 1 + 2 |
| **nonlinear_integration** | 134 | 132 / 2 FAILED | `k2b_force_scaled_entries_are_reached_by_neither_the_loop_nor_the_product` and `kd5_nonlinear_sources_name_no_formation_check_entry_point`, ok → FAILED. **The defect** |

- **Other surfaces:**
  - the fail-fast sweep stopped at PP `t13`, as expected;
  - pytest: 3,540 passed, 32 skipped;
  - vitest: 3,552 passed;
  - `build:wasm:desktop` and `build:desktop`: exit 0.
- **F′'s run** was still in progress at 00:39Z (28 of 41 suite logs). I have not compared it.

## 6. After the all-clear: PENDING

Prepared under `evidence/confirm_02/`, not run:
- `synth/`: a 14-file synthetic tree covering mod-rs and non-mod-rs files, inline modules with and without `#[path]`, nested inline modules, and brace-bearing char and string literals inside inline modules.
- `run_after_allclear.py`, which runs, one cargo job at a time:
  - `rustc --emit=metadata` on that tree, with and without `--cfg test`;
  - the walker probe `zz_rv95_walker_probe.rs` on the same tree;
  - nonlinear_integration's full suite on F′, expected 134, equal to M;
  - five mutants:
    - W1, inline handling removed: must panic;
    - W2, the old `fn solve_load_case` pin: must fail;
    - W3, `#[path]` inside inline modules resolved from the file's directory: expected to survive the real tests and be killed by the probe;
    - W4, a `ForceScale` added to PP's wrapper: must fail;
    - W5, test-ness no longer from declarations: expected to fail.

**My confirmation of item 6, and of DEC-025 on F′, follows when ROOT sends the all-clear.**

## New NOTEs

| # | Where | Evidence | Remedy |
|---|---|---|---|
| B-1 | `s11k_tests.rs` `enclosing()` and `target()` | An inline module that carries its own `#[path = "dir"]` attribute changes rustc's directory for nested declarations; the walker would still use the module's name. Neither PP nor nonlinear_integration has one today. The `#[path]` attributes in both are on file modules (`retained_memory.rs:13`, `:2954`, `:2957`; `structural_adapter.rs:2937–2949`; the `kd5_models.rs` includes). | Optional: handle it, or assert that no inline module carries `#[path]`. |
| B-2 | Process (RV95's own review) | My review ran PP, runner, result_export, Python and TS, but not nonlinear_integration, whose tests read PP's source at runtime (`s11k_tests` ×2, `k5_tests`). This is the same class as the CI-policy defect (PP reading src-tauri's source). ROOT's new rule (the full 40-manifest suite before a freeze) closes it. | None beyond ROOT's rule. Later reviewers should also run the crates whose tests read the changed crates' source text. |
| B-3 | Host disclosure | The `/tmp/rv95_never_used` write described above, removed at once. | None. |

## Records

`evidence/confirm_02/` (placeholder paths only) holds:
- `source_equality_Fp_vs_42009dba72.txt`;
- `check_citations_Fp.txt`;
- `dec025_compare.py` and `dec025_M_vs_F.json`;
- `run_after_allclear.py`, `zz_rv95_walker_probe.rs` and `synth/`.

`SHA256SUMS` covers the `u9_01` folder.
