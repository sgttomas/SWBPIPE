# RV8: delta check of slice K1, from the reviewed head to the combined tree

**Verdict: PASS.** No BLOCKING and no SHOULD-FIX findings; 3 NOTEs. RV8's three SHOULD-FIX findings are resolved.
- **Each new test discriminates.** Each has a dense-only precondition that is not vacuous, and each kills its mutant at a behavioural assertion on clean archives of `b6f1724b4`.
- **The merge of main (K2a) is conflict-free** and changes no K1 file.
- **The combination is sound.** K1's sparse assembly forms every frame through the same K2a-checked call as the dense assembly, and a formation refusal propagates identically.
- **The dense results are unchanged.** On the combined tree they are byte-identical to merged main's, and to the reviewed base's.

## Reviewer, brief and revisions

- **Reviewer:** RV8, the reviewer of `K1_REVIEW.md` (numerics `369dc2f16`). I was dispatched by ROOT (HELP_HUMAN) as a background subagent of ROOT's session, with the same host rules and paths. The brief is ROOT's delta message (2026-09-28).
- **Reviewed revisions:**
  - the reviewed head: `43f9e6a78`;
  - the delta head: `b6f1724b4` (`codex/piping-k1-20260928`).
- **Delta commits** (first parent):

  | Commit | Content |
  |---|---|
  | `340e87a2d` | RV8 tests |
  | `3b86b111f` | merge of main `f12e06876` (K2a, PR #1032; K2a's reviewed head `aad23e82d`) |
  | `e70e4033c` | RETURN addendum 1 |
  | `1997935f8` | the K2a interaction tests |
  | `b6f1724b4` | RETURN addendum 2 |

- **Everything built** here came from `git archive` copies in `<scratch>/d2`. Nothing was built or written in `<wt>/k1`, and I made no Git writes.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV8-1 | resolved | `structural_adapter/k1_tests.rs`, `k1_bend_bend_tee_…_rv8_1` | A bend–bend tee with a branch at the shared node, at flexibility factors 1.0 and 1.7. Dense-only precondition: reversing the bends changes the dense bits. Then K bits, coalesced contributions, K·u, E12 reactions, and SA's plain and formation-checked entries in both modes, byte-identical and `Ok`. RV8-BLOCK-ORDER is killed at :1072, "bend-bend tee flex=1: K bits". | Closed |
| RV8-2 | resolved | `k1_bend_support_springs_…_rv8_2` | A bend with support springs on all 12 end DOFs, at 8 magnitudes and 2 flexibility factors, with the same checks. The dense precondition "some magnitude discriminates" is asserted after the checks, and holds (6 of 16). RV8-SPRING-FIRST is killed at :1072, "bend supports flex=1 k=4.4e7: K bits". | Closed |
| RV8-3 | resolved | `k1_split_ledger_…_rv8_3` | F122 and CSKEW_8_5 with three-term split ledgers. Precondition 1: some exact net ≠ the rounded net. Precondition 2, dense only: the record from the terms ≠ the record from the folded values. Then dense and sparse are byte-identical, and both demote with `Estimate`. RV8-FC-TERMS is killed at :1386, "F122 SparseInteractive: split ledger". | Closed |
| RV8-D1 | NOTE | `sparse.rs:447–471` (the order of the node checks against formation). The interaction tests do not pin it. | My mutant **RV8-K2A-NODESFIRST** checks every element's nodes before forming any element. It survives FK, SD and NI in full. The divergence is real: on a model where element 0 is K2a's reach_zero member and element 1 names a node outside the model, dense gives `NumericalRange { "12EIy/L^3: (12*E)*Iy" }`, while the mutant's sparse assembly gives `InvalidNodeIndex { 2, 2 }`. The unmutated head matches dense (`nodesfirst_demo.txt`). This is invalid input only: both representations refuse, and they differ only in which refusal is first. | Optional: add this two-element case to `k1_k2a_the_first_failing_element_…`. |
| RV8-D2 | NOTE | RETURN addendum 1 (`rv8_fixes/overlay.txt`) | Addendum 1's mutant runs overlaid an uncommitted `k1_tests.rs` on `43f9e6a78`, and `overlay.txt` names the file but not its hash. Addendum 2 records the interaction test's hash (`cf241452…`, equal to the committed file). My re-runs on the committed `b6f1724b4` (`k1_tests.rs` sha256 `ccd97e94…`) reproduce addendum 1's kill sites exactly, so the evidence stands. | None needed. For future overlays, record the hash. |
| RV8-D3 | NOTE | RETURN header (lines 3 and 11), §13 "(pending)", §15; CHANGE_RECORD | RETURN's opening status still reads "complete for the pre-K2a tree", with "Pending: the K2a interaction tests…". Only the appended addendum 2 says that this is done. CHANGE_RECORD was edited in place (its "Remaining" bullet, the Files header, and the "untouched" sentence). That is acceptable for a PR record, and the history is in Git. | Optional, additive: add one line under RETURN's status pointing to addenda 1 and 2. |

## 1. RV8-1, RV8-2 and RV8-3 are resolved

- **Append-only.** `340e87a2d` is one hunk at the end of `structural_adapter/k1_tests.rs` (`@@ -1018,3 +1018,392 @@`) with no removed line. At `b6f1724b4`, every K1 code file except that one and the new interaction test is blob-identical to the reviewed head (`git_verification.txt`). No existing test changed.
- **Discrimination.** Each test's precondition runs on the dense side only, so a mutant of the sparse code cannot make it vacuous (see the Findings table). The RV8-2 precondition holds on 6 of 16 cases (k = 4.4e7, 2.1e8 and 9.7e8 N/m at both flexibility factors), per addendum 1. My own probe found the same three magnitudes.
- **Re-kills on clean archives of `b6f1724b4`:** the review's `rv8_mutants.py`, unchanged; one archive and one target per mutant, deleted afterwards; FK, SD and NI in full.
  - NONE is clean: FK 154 + 3 + 13 + 3 + 6, SD 30, NI 97 + 4.
  - Each RV8 mutant fails exactly one test, in NI, at the site above, with 0 compile errors. These sites and messages equal addendum 1's.
  - Each kill is at a behavioural assertion: a K-bits comparison, or the dense/sparse `Debug` parity.

## 2. The merge, and K1 with K2a

**Conflict-free:**
- `git merge-tree --write-tree 340e87a2d f12e06876` exits 0 with tree `0c01ef125`, which is `3b86b111f`'s tree. So there was no conflict and no manual resolution.
- No K1 file changed in the merge.
- Main's product-tree changes since the base are K2a's five files: FK `lib.rs` (+130 −12), `diagnostics/src/lib.rs`, the K2a FK tests and models, and PP's `tests/k2a_formation_range_runtime.rs`. There are no fixture, schema or Cargo changes.

**Formation through the same checked call:**
- K2a changed only `local_stiffness`, checking every intermediate, with `NumericalRange { name }`. An accepted coefficient keeps its exact unchecked bits.
- `FrameElement::global_stiffness` calls `local_stiffness`. The dense assembly (`lib.rs:882`) and K1's sparse assembly (`sparse.rs:453`) both form frames through `element.global_stiffness()?`, in list order, after the same node checks.
- K1 forms every frame and user before accumulating, so a refusal returns the same error for the same first element, before any value is summed.
- User elements (`sparse.rs:462`) and realized bends (pre-formed blocks) do not go through `local_stiffness`, so K2a does not reach them.
- SA's evidence builder is shared by both evidences, so a refusal there is identical as well.

**Behaviour on the combined tree:**
- **Dense differential.** The review's probe, unchanged, was built against archives of merged main `f12e06876` and of `b6f1724b4`. The two outputs are byte-identical: 2,713 lines, sha256 `a96f5eae…`. That is the same output as the review's base and candidate, so K2a changes no dense result on these inputs and K1 changes none on the combined tree.
- **Parity probe.** The review's parity probe, unchanged, on `b6f1724b4`: 45,686 checks and 0 mismatches. Its coverage statistics are identical to the pre-merge run.

## 3. The interaction tests (`frame_kernel/tests/k1_k2a_interaction.rs`)

- **Against the brief:**

  | Requirement | Where it is met |
  |---|---|
  | The named error is the same in both representations | `assert_same` compares both assemblies: the same `FrameKernelError`, or bit-identical K |
  | RF-RANGE | All 162 members: 16 LEF-large are refused as `NumericalRange { "GJ/L: G*J" }`, each equal to its generated `refusal`; 98 form bit-identically; 48 are degenerate |
  | reach_zero and reach_lef | Both are refused as `NumericalRange { "12EIy/L^3: (12*E)*Iy" }` in both assemblies |
  | The first failing element | Normal, then reach_zero, then LEF-large: both report reach_zero's refusal, which equals reach_zero alone. In reverse order both report LEF-large's, and the test asserts the two orders differ |
  | Non-vacuous | Asserted: refused > 0, formed > 0, degenerate > 0 |

  I reproduced these counts with `--nocapture` (`interaction_nocapture.txt`).
- **LEF-small is reported honestly.**
  - Its members (L = 1.2e-60 m), and L-240's and SIM-a's, are at or below the 1e-12 m axis tolerance. The test asserts that `FrameElement::new` refuses each with exactly `DegenerateAxis { "element length" }`, and that LEF-small never reaches assembly.
  - Neither the test nor its commit message claims assembly-level parity for LEF-small. RETURN A2.1 says the requirement "holds only at the element", and that LEF-small's named `NumericalRange` exists only at `local_stiffness` level.
  - The LEF pattern at an admissible length is reach_lef, which the test covers at assembly.
- **Evasion attempts:** I8R's two mutants re-killed, and three of my own (`rv8_delta_mutants.py.txt`), all on clean archives of `b6f1724b4`:

  | Mutant | Change | Result |
  |---|---|---|
  | K1-K2A-SKIP (I8R) | a refused frame is accumulated as a zero block | killed: `k1_k2a_interaction.rs:104` ×3 (dense `Err`, sparse `Ok`) |
  | K1-K2A-FIRST (I8R) | formation checked in reverse first | killed: :101 ("ordered") |
  | RV8-K2A-SORTED | a label-dependent formation order: frames formed in ascending (node_i, node_j) | killed: :101 ("reversed"). The ordered case alone would not catch it; the reversed case does |
  | RV8-K2A-NAME | every `NumericalRange` renamed to "GJ/L: G*J" | killed: :101 ×2 (reach_zero, "ordered"). The RF-RANGE test alone would not catch it, because its only refusals are already "GJ/L: G*J" |
  | RV8-K2A-NODESFIRST | node checks for every element before any formation | **survives** FK, SD and NI (RV8-D1) |

## 4. The combined-tree evidence (`_run_records/combined/`, RETURN addendum 2)

- **Suites.** I compared every one of the 39 logs with ROOT's merged-main baseline (`<wt>/scratch/calib/suites_main_f12e06876/`) by summing their test-result lines (`suites_spotcheck.txt`).
  - Only three manifests differ: frame_kernel 162 → 179 (+17), nonlinear_integration 89 → 101 (+12), sparse_direct 25 → 30 (+5). That is 34 added tests, with 0 failures in those crates.
  - The only failures anywhere are the three Mac platform tests, with the same names as the baseline.
  - The failure-block hashes in `failure_blocks_sha256.txt` match (300 and 952,879 bytes).
  - My own FK, SD and NI runs on `b6f1724b4` give 179, 30 and 101, all passing.
- **T9 (Mac-only, records cross-checked, not re-run):**
  - the combined base `f12e06876` equals the candidate on 112 of 112;
  - the combined base equals ROOT's Mac main `649162522` hashes on 112 of 112;
  - the combined candidate equals the pre-merge K1 candidate's outputs on 112 of 112.
  - The summary states the result is Mac-only (`t9_crosscheck.txt`).
- **The mutation table.**
  - `kill_site_comparison.txt` covers 28 mutants with 0 kill sites lost. Its added sites all come from the RV8 tests.
  - My re-kills of 5 of them (RV8-BLOCK-ORDER, RV8-SPRING-FIRST, RV8-FC-TERMS, K1-K2A-SKIP, K1-K2A-FIRST) give the recorded `file:line` exactly.
  - `mutate_k2a.py` is byte-identical to the committed `mutate_k2a.py.txt`, and `rv8_fixes/rv8_mutants.py.txt` is identical to my committed copy.
- **Callers.**
  - I8R's combined `callers.txt` has 284 sites, 74 of them non-test. With line numbers dropped, the 74 non-test rows are identical to the pre-merge list.
  - My own lexer on `b6f1724b4`: 206 non-test uses, identical to the reviewed head. The 11 added rows are all test callers.

## 5. Records hygiene

- **SHA256SUMS:**
  - at `b6f1724b4`: 237 entries, all verify, covering every file;
  - at `e70e4033c`: 133 entries, all verify.
- **No committed evidence rewritten.**
  - Against `43f9e6a78`, 0 of the original 122 entries are missing, and only `RETURN.md` and `CHANGE_RECORD.md` changed hash. The same holds against `e70e4033c`.
  - Git shows only those two files (and SHA256SUMS) modified; 115 were added.
  - The RETURN edits are the two appended addenda, the §12 `pivots()` line (marked "addendum 1, N7"), and the N2 correction of the duplicated §16 bullet.
- **N1's mapping, verified:**
  - `19925122b` and `4319854dc` have the same code tree; `19925122b` also carried the 11 WIP record files.
  - `9d4ba0e17` is `826a9eed4` plus the 8-line `formation_check.rs` site-table hunk.
  - All four pre-reshape commits are reachable from `origin/codex/piping-k1-wip-20260928`.
- **N2's correction:**
  - The "three such lines" is exact: `git diff --check` on the records commit reports 3, in the two `.patch` files.
  - The attribution of the duplication to ROOT is consistent with the committed history: both commits that contain it (`3513fd8ab`, `43f9e6a78`) are ROOT's, and the bullet itself says "(ROOT, at commit)". No earlier uncommitted draft survives for me to check.
- **`git diff --check`** is clean on all four non-merge delta commits.
- **GEN-8 passes on `b6f1724b4`** (1 passed, 10 deselected). It was run read-only in `<wt>/k1` (at `b6f1724b4`, clean): same index mtime, size and hash, and no newer file (`gen8.txt`).
- **No machine paths or model identifiers** in the 237 record files, by grep and by the harness's own `MACHINE_ABS_PATH_RE`. The same regex finds 0 hits in my own records.

## What I ran

- **Environment:** `aarch64-apple-darwin`, rustc and cargo 1.97.1, `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 8` for single jobs or `-j 4` for mutants (three at once), and `RUST_TEST_THREADS=4`. The targets were under `<wt>/rv7-target`, and the mutant targets were deleted after each run.
- **Suites:** FK, SD and NI in full on `b6f1724b4`, plus the interaction test with `--nocapture`.
- **Dense differential:** merged main against `b6f1724b4`.
- **Parity probe:** on `b6f1724b4`.
- **Mutants:** 9 runs (NONE, 3 RV8, 2 I8R K2a, 3 of my own), plus the NODESFIRST demonstration program.
- **Other:** my caller scan; the suite, T9, SHA256SUMS, path and whitespace checks; GEN-8.
- **The memory guard never fired.**

## What I did not check

- **Not re-run:** T9 and the 39-manifest suites (both spot-checked from logs), and the 23 checkpoint-C mutants other than those named in §1 and §3.
- **K2a itself** was reviewed by the cloud's RV7 (PASS at `aad23e82d`), and I did not re-review it. I read only its `local_stiffness` change and its interaction with K1.
- **Outside my run:** hosted CI (Linux), the DEC-025 sweep, and the both-entry gate (ROOT: not run for K1).
