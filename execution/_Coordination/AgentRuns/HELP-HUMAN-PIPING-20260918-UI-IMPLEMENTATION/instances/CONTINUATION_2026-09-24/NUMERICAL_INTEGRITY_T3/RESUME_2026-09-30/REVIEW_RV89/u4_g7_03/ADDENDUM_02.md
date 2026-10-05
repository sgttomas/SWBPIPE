# RV89 G9a, addendum 02: the no-build Pass B on the refrozen head F′ = `5488136a19`

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This follows ADDENDUM_01, which confirmed F = `20dd3d929d`.

**Candidate:**
- I65's record `R/I65/u9_refreeze_01/` (RETURN.md `6e1c143e…`), with its driver `_run_records/g7_pass_nobuild.sh` and outputs `runs/refreeze/` (scratch `pass_refreeze/`).
- **F′ = `5488136a193ef8921bd88c33a8124647c4cb352d`.** Outside `execution/`, it differs from F only in `core/solver/nonlinear_integration/src/s11k_tests.rs` (`6cfe50d368`, the repair RV95 reviews).

**Method:** read-only. **No cargo or rustc**, as ROOT asked while DEC-025 runs.
- I made a `git archive` extract of F′'s projects/chirality-piping (without `execution/`). It matches the tree file for file: 2,950 of 2,950, with nothing extra.
- I ran Pass B's own checks (u4_g7_06's `pass_checks.py`, `delta_inventory2.py`, `g7_linemap.py` and `statics_list.py`) on the extract and on I65's recorded outputs.
- I compared every recorded output with u4_g7_06's on F.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 1 (N-1: the driver's header understates its changes; no effect on this run) |

**The build-gate argument holds. No registered build of F′ is needed.**

**Nothing for ROOT to rule on.** N-1 matters only if the no-build driver is reused.

## 1. The no-build driver against u4_g7_06's `g7_pass.sh` (`evidence/addendum_02/driver_diff.txt`)

**It matches apart from the stated removals:**
- the copy is a fresh `git archive <rev>` into `pass_<tag>/work`;
- there is no cargo: the law gate, the PP and runner outcomes, the witnesses and the challenge are gone;
- `price_delta` reads the given law record.

**The remaining gate calls are unchanged.** `tree`, `entry`, `statics`, `linemap`, `premise`, `text_run`, `delta`, `text`, `forms`, `noncand_run`, `noncand`, `controls_run` and `controls` are each called exactly as before, with u4_g7_06's tools and rules (`REC` = u4_g7_06), u4_g7_01's references, and D read from the run.

**But the header's "and nothing else" is not accurate** (N-1). Also removed are:
- **(a)** `finish()`, so there is no `VERDICT` line and no exit code. The script always exits 0, and the codes exist only in `GATES.txt` and `verdict.tsv`;
- **(b)** the early stops after `tree`, `entry`, `linemap`, `premise` and `delta = 5`;
- **(c)** the `text_summary` gate, which gave 6 if the TEXT chain left no summary;
- **(d)** the mapping of a delta-tool failure to 6, and the stop when no inventory was written;
- **(e)** the per-step memory-guard checks, which are now done once.

**None of these changes a result here.**
- All 13 recorded gates are 0 (`GATES.txt`).
- `text_run` is 0, and the summary exists, with D 14,734.
- The delta tool exited 0 and wrote its inventory.
- I re-ran the checks myself (§2).

**In that sense no gate was weakened for this run.** But as a reusable tool the driver fails open at the process level. The `tree` gate is also trivially satisfied, since the copy is extracted from the same revision it is checked against.

## 2. The gate outputs (`evidence/addendum_02/gate_checks.txt`, `outputs_vs_F.txt`)

**My checks:**

| Check | Run on | Result |
|---|---|---|
| tree | my extract | 2,950 / 0 / 0 |
| entry | F′'s `retained_memory.rs` against `git show 0c7827b6ad` | **equal, threshold `4_026_531_840`** |
| statics | my extract | none added or removed |
| linemap | `ba1faa1c..F′` | exit 0; 0 rules moved, 0 unmapped |
| premise | the three pins on my extract | 0 |
| forms | my extract | equal to regeneration from G7's tree |
| text | I65's refreeze outputs | 0 (D 14,734, every row and output identical to Pass A) |
| noncand | I65's refreeze outputs | 0 (410, no new, none absent) |
| controls | I65's refreeze outputs | 0 (12 of 12) |

**Against u4_g7_06's outputs on F** (`pass_frozen`):
- **86 common outputs are byte-identical.** That is every TEXT-chain output under `sens_pb/`, every gate JSON, `D.txt`, the edges, the lexicon, the non-candidate dump and comparison, the controls and `price_delta`. It goes beyond I65's 60.
- **Six differ, and I read each one:**
  - `tree_blobs.tsv`: the `s11k_tests.rs` blob only;
  - `linemap.out.json` and `rules/g7_linemap.out.json`: the `new` revision only. The carried rule files and premise pins are byte-identical;
  - `delta.out.txt` and `delta_inventory.json`: the one added `s11k_tests.rs` row;
  - `verdict.tsv`: the shorter set of gates.
- **Files that exist in only one run** are the build-gate outputs that a no-build run lacks, and the no-build run's own notes.

## 3. My delta inventory of `ba1faa1c..F′`

`delta_inventory2.py` on my extract, with I65's refreeze edges, gives **36 files and 87 rows**. They are row-for-row equal to I65's (file, lines, class, fingerprint).
- **Against my ADDENDUM_01 inventory of F** (86 rows), exactly one row is added and none removed: `core/solver/nonlinear_integration/src/s11k_tests.rs`, class `test` (a test file).
- **With an empty reviewed table** it stops (5), naming exactly the same 11 hunks as at F. With I65's table it passes, so **all 11 reviewed entries match**, including N-1's `27181031…`.

## 4. The build-gate argument (`evidence/addendum_02/build_gate_argument.txt`)

**It holds.**
- **F′ against F** changes one file, `nonlinear_integration/src/s11k_tests.rs`. It is declared only as `#[cfg(test)] mod s11k_tests;` (nonlinear_integration `lib.rs:10–11`).
- **PP and the runner don't compile it.** PP depends on nonlinear_integration as an ordinary path dependency (`Cargo.toml:20`), so every PP target and runner/headless compiles that crate without `cfg(test)`. The module is not in PP's lib test binary, the challenge binary or the runner's builds.
- **Nothing in PP or the runner reads the file.** PP's `tests/s11f_site_test.rs` reads two nonlinear_integration sources by `include_str!`, `structural_adapter.rs` and `lib.rs`, and both are unchanged. No PP or runner source, test or `build.rs` names `s11k_tests.rs`.
- **The registered entry and identity are untouched.**
  - **Reviewed inputs:** all 14 have the same blobs at `0c7827b6ad`, F and F′, which I checked with `git rev-parse`.
  - **Build identity:** it records the toolchain, target, profile and flags, not source content, so it is the same.
  - **The entry:** byte-identical (§2).
- **So the law gate, the PP and runner outcome lists, the 9 witnesses and the challenge compile and run the same code as at F.** F's results stand: I65's u4_g7_06 run, which ADDENDUM_01 confirmed against my own registered build of `6b9bb19a5f`.
  - nonlinear_integration's own tests (K-D5/K2b) are not part of Pass B's suites. They are RV95's to review.

**No registered build is needed to settle any part.** A routine post-DEC-025 confirmation run is optional, not required.

## Finding

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | `R/I65/u9_refreeze_01/_run_records/g7_pass_nobuild.sh:2–8` (the header) | The header claims three differences "and nothing else". The driver also drops `finish()` (no verdict line, always exit 0), the early stops, the `text_summary` gate, and the delta tool-failure mapping and no-inventory stop. Every recorded gate is 0 and I re-ran them, so this run is unaffected | If the driver is kept or reused: correct the header, and restore `finish()`, the `VERDICT` line and the `text_summary` gate, so a no-build run also fails closed by exit code |

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 18:37–18:45 MDT.
- **No build.** No cargo or rustc, and no native, solver-at-scale or DEC-025 job.
- **Git.** `git archive`, `ls-tree`, `rev-parse`, `show` and `diff`, all with `GIT_OPTIONAL_LOCKS=0`. No Git writes.
- **Other agents' files.** I65's records and scratch (`pass_refreeze`, `pass_frozen`) were only read.
- **Writes.** This addendum, `evidence/addendum_02/` and SHA256SUMS. Also the extract WT/rv89_pr/refrozen, deleted after this addendum, and WT/scratch/rv89_u4_g7_01/refreeze/. Machine paths in the evidence are replaced by `WT` and `R`.
