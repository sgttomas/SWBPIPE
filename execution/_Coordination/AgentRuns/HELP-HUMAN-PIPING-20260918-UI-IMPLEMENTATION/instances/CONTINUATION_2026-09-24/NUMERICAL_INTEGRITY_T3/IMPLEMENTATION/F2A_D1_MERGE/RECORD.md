# PR #1082 merge record: the F2a D1 milestone (ROOT, 2026-10-05 UTC)

**PR [#1082](https://github.com/sgttomas/chirality/pull/1082) merged into main at 2026-10-05T01:09:29Z** as merge commit `0b00b8e8b6110a497d1f9eb67a1ddc60a1097c22`.
- **The parents** are main M `5fdc5ab6012ddb50ecf286621a291b4da577405e` and the frozen head F′ `5488136a193ef8921bd88c33a8124647c4cb352d`.
- **How it was merged:** `gh pr merge --merge --match-head-commit` F′, with auto-merge off.
- **Checked immediately before the merge:** `origin/main` was M, the PR head was F′, and the state was `MERGEABLE`/`CLEAN`.
- **The owner held other merges to main** from the freeze until this merge. The hold ends now.
- **The merge facts** are in `_run_records/` (`MERGED_PR.json`, `PARENTS.json`, `MERGE_COMMAND.txt`, `CI_RUNS.jsonl`, `PR_BODY_FINAL.md`).
- **The final PR body** equals the package's `PR_BODY.md`, apart from the one trailing newline that `jq` adds.

**What merged** is the milestone stated in the handoff: RF-SKEW-T-CANT-OFF-122-r1e-04 through the actual captured facade in both solver modes, under M03-INTEGRITY-MP-v2, matching its independent reference and preserving the refusal and coexistence controls.
- It holds in the registered dev/test build only, under M = 4,026,531,840 B.
- Reader eligibility comes in Python, Rust and TypeScript, together with the carriers.
- Scope, and what stays closed, are as in the package's `CHANGE_RECORD.md` §4–§5, in the merged tree at `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/F2A_D1/`.

**The heads:**
- **The source head** is `6cfe50d368`.
- **The integration head** is NUM `42009dba72`. 138 of the 140 maintained files are identical to it; the other two are the recorded three-way merges.
- **The first freeze,** F = `20dd3d929d`, failed DEC-025 and hosted CI on one test defect. It was repaired test-only and refrozen as F′ (RR "DEC-025 on F finds a test-walker defect; repaired and refrozen as F′ = 5488136a19").

## Gates on the merged head F′

| Gate | Result on F′ | Evidence |
|---|---|---|
| **G1, independent review** | **RV95:** complete review PASS (0/2/7). Then CONFIRMED on F (0/0/4), and on F′ for the repair and package (0/0/3) and for the final merge candidate (0/0/2). Nothing blocks | `R/REVIEW_RV95/u9_01/` (REVIEW, ADDENDUM_01–03) |
| **G2, hosted CI** | **The pull_request run 37247819786** succeeded on F′ (7 jobs succeeded, 1 skipped, the numerical cargo suite included). **The full-SHA dispatch 37247819679** (`target_base` = M) also succeeded. So did Harness Pre-merge Validation, pec-tests and governance-harness. **F's runs 37243941694 and 37244157642 failed** at nonlinear_integration | `_run_records/CI_RUNS.jsonl` |
| **G3, DEC-025** | **34 of 40 manifests are identical to M.** The other six differ only by added tests. The failing set is M's: PP `t13` and runner's two `load_reference` tests. pytest, vitest and both desktop builds pass. RV95 independently agrees | `dec025/` |
| **G4, GEN-8** | 1 passed, 10 deselected, on F′ | `_run_records/gen8_fp.txt` |
| **G5 (T9), G6 (both-entry)** | Run on C and C2. Carried to `35d8ae59a7`, then to F and to F′, by ruling. RV95 confirmed the premises | `R/I61/u9_g5g6_01/`; RR |
| **G7, src-tauri** | 116 = 116 on C. Carried to F (A-3) and to F′ | RR "U9 gates G7 and G8 pass…" |
| **G8, controls and the 324-output sweep** | Pressure refused at D1.5. Coexistence (n05/n06) refused with exact bytes. The sweep is byte-identical, registered and Stale. Carried to F′ | `R/I61/u9_g8_01/` |
| **G9a, Pass B** | **On F:** the full frozen-head Pass B exits 6, the delta being exactly the six added PP tests. **On F′:** every no-build gate is 0, with one added test-class row (`s11k_tests.rs`). **RV89:** PASS on both (ADDENDUM_01; ADDENDUM_02, 0/0/1) | `R/I65/u4_g7_06/`, `R/I65/u9_refreeze_01/`, `R/REVIEW_RV89/u4_g7_03/` |
| **G9b, Direct-caller scan** | No product caller. Carried | `R/I61/u9_g8_01/` |
| **Source equality, citations** | 5/5 PASS (140 files; 138 identical; 2 recorded merges). Citations 368/0/0 | `_run_records/se_fp.*`, `citations_fp.txt` |
| **G10, the native witness** | **OUTSTANDING, by the owner's decision** ("Merge; leave it outstanding"), per the T0R and T1 precedent | see below |

**G10 stays open on the owner's Mac.** It covers:
- the milestone model on the ordinary route, with unchanged standing and Current display;
- the result-export and stress-neutral panels refusing a successor.

Meanwhile, the following cover it:
- the desktop calls only the ordinary wrapper (RR:9261), and no Direct product caller exists (G9b);
- the panel gates are covered by vitest and the hosted browser E2E shards;
- src-tauri passed 116/116.

## Notes recorded at the merge

- **A-1, the records not brought.** **6,346 execution-record files (218.8 MB) at NUM `104ebc183e`,** the NUM head when #1082 merged. They stay on NUM.
  - The count is files added or modified since the base `381be775ae`, with sizes from Git's objects.
  - The same method reproduces RV95's 6,103 files (213.3 MB) at `bb3d766379`. The package's "5,677 (191.4 MB)" is the plan-basis count.
- **A-2, platform scope of the pins.** The successor pins (sparse `ac6986b0…`, dense `6cd1d249…`) are **identical on the two targets observed:** this Mac, and hosted Linux run 37240946900. No claim of platform stability is made (RV95 N-6, CHANGE_RECORD §5).
- **A-3:** G7 carried from C to F, after ROOT's disclosed reuse of the scratch tree. It carries from F to F′ because only a `#[cfg(test)]` module of a dependency crate changed. **A-4:** the review overlap with I61's G6 part 2 execution 1 is disclosed; execution 3 stands.
- **RV95 B-1, C-1 and C-2: walker limits.** None affects any scanned source today, and the behaviour is unchanged from main. They are optional hardening for nonlinear_integration's `non_test_modules`:
  - an inline module with its own `#[path]` directory;
  - a string literal earlier on a `#[path` line;
  - the inline-`#[path]` branch, untested by the crate itself (RV95's mutant W3 survives the crate's tests).
- **RV95 B-2:** RV95's own review did not run nonlinear_integration. This is the same class of gap as the walker defect, and the new rule closes it: the full 40-manifest suite runs before a freeze. **RV95 B-3:** one write to the system temp directory, removed at once.
- **RV89 N-1:** the header of I65's `g7_pass_nobuild.sh` understates its removals: the verdict and exit code, the early stops, the `text_summary` gate and delta-tool failure handling. The script is a one-off record, not a reusable gate tool; reuse would first require restoring the fail-closed verdict.
- **ROOT disclosure:** ROOT's records commit `104ebc183e` swept in I65's then in-progress `g7_pass_nobuild.sh`. It is byte-identical to the version I65 returned.

## Open after this merge

- **G10,** the native witness, on the owner's Mac.
- **U8,** the deferred witnesses: the native Ceiling row, the L = 0 base, and RV93 N-5 (a real-input Candidate test).
- **Wider F2a** (CHANGE_RECORD §5.2), including RV95 N-6: re-establish the milestone's bytes and verdicts on any further registered identity.
- **S-I, F2b per domain, and F3.**
- **The public-activation checklist** (CHANGE_RECORD §4).
- **The owner-held items** (CHANGE_RECORD §5.4).
- **T6:** the successor outputs, the v0.3 dispatcher, and RV95 N-5.
- **Routed:**
  - RV94 N-5: Python ignores `expected_by_reader`;
  - RV95 N-2: citation forms;
  - the Pass B tool's attribution of deletion-only hunks;
  - RV95 B-1, C-1 and C-2 (optional walker hardening);
  - RV89 N-1.
