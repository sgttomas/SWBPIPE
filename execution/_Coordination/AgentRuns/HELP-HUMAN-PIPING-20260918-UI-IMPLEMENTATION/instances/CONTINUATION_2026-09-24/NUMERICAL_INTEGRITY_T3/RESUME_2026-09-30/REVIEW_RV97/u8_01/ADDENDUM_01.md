# RV97 Addendum 01: U8's PR head (#1102, `61c35f56a8`)

**Reviewer:** RV97, TASK (Type 2), continued by ROOT (HELP_HUMAN, Agent 0) for a short, read-only confirmation of #1102's head, as RV99 did for #1100. No descendants.

**Basis:** RR "The records-only PR #1101 and U8's PR #1102 opened; RV102 dispatched", with the rulings before it that this checks against:
- "RV98 confirms U8's Pass B; …" (the `not-d1` fixture ruling);
- "U8's full suite passes; I77's package accepted; …";
- "RV99 confirms #1100's amended head and the DEC-025 carry-over" (N-12's wording).

**My two rounds** (`REVIEW.md`, `REVIEW_ROUND2.md`) and their `SHA256SUMS` are sealed and unchanged. This addendum has its own sum file, `ADDENDUM_01.SHA256SUMS`.

**Host:** Git reads only (`GIT_OPTIONAL_LOCKS=0`) in NUM's checkout. NUM's working tree was clean before and after. No cargo, no Git writes, no fetch. Scratch was in `WT/scratch/rv97_u8_01/a1/`, deleted at the end.

## Verdict: **PASS. The head is confirmed.** One SHOULD-FIX for the merge sequence with #1101.

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 3 |

1. **The maintained content is what I reviewed.** The 7 files' blobs at `61c35f56a8` equal `bd6b4be2c3`'s, and NUM `b4140b7645`'s. The branch is one commit on main `75a8c3291f`, with no U8 or NUM commit in its history.
2. **The package is truthful in substance against my two rounds, RV98 and the full suite, with no overclaim.** The Pass B and RV98 rows and the `not-d1` reading are accurate. Two stale or blurred lines are noted (A1-N-1, A1-N-2).
3. **Both tools reproduce ROOT's records byte for byte:** `se.txt`, `se.json` and `citations.txt`. The negative controls fail as they should.
4. **The carry-over premise holds:** no DEC-025 surface reads the T3 records beyond the three readers of `REFERENCES/` and `DESIGN_NUMERICS/`, and #1101 changes neither folder. **But #1101 also adds an earlier draft of U8's package at the same paths** (A1-S-1), so #1102's absorption of main is not conflict-free.

## Findings

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| A1-S-1 | SHOULD-FIX | #1101 (`11b2d04f13`) and #1102 (`61c35f56a8`): `T3/IMPLEMENTATION/U8/{CHANGE_RECORD.md, PR_BODY.md, SHA256SUMS}` | **Both PRs add U8's package, in different versions.** #1101 carries NUM `4e6c2fcbfe`'s copy, which is I77's draft with its rows marked **PENDING** (`c0275776…`, `700a94a4…`, `76b76866…`). #1102 carries the completed copy (`a5075a7d…`, `9acbec1e…`, `fe399414…`). `citations.json` is the same in both. If #1101 squash-merges first, #1102's absorption of main meets **add/add conflicts in three files**. Until #1102 merges, main would also hold a package whose gate rows say PENDING. DEC-025 is unaffected, because no suite reads these paths (item 4). | **Before #1101 merges** (RV102 is still reviewing it): re-cut #1101 without `IMPLEMENTATION/U8/`, or with the completed package. **Otherwise,** resolve #1102's absorption by taking #1102's three blobs exactly, then re-run `source_equality.py` (check 4 reads the package sums) and GEN-8 on the new head. |
| A1-N-1 | NOTE | CHANGE_RECORD.md, the "Status of this file" bullets, and §1's first paragraph | **Stale about main.** These say "The local `origin/main` is `c1571f7feb` (#1098 and #1099)… no piping path, and none of the 7 files". #1102's base is `75a8c3291f`, which adds #1100 (S-I1: 8 piping files under `core/rules`, `core/analysis_runs/rule_interval.py`, a fixture, a schema and a test). **The substantive claim holds at the actual base:** none of #1100's 8 files is among the 7, and main's blobs of the 5 modified files equal the U8 base's (`evidence/addendum_01/item1_content_and_history.txt`). | At the next package touch (e.g. the absorption): name `75a8c3291f` and #1100 in that bullet. |
| A1-N-2 | NOTE | CHANGE_RECORD.md §5, the acceptance-runs table, "Source" column | **Two rows credit RV97 for runs it did not make.** RV97 is listed for the Python row, which includes the 24-file sweep (1,873), and for the TS row, which includes the desktop suite (3,574). RV97 ran: PP registered, all targets (708/1/10); PP Stale `--lib`; `result_export` (173); the three Python retained suites (480); `retainedPrecision.test.ts` (474); and `tsc`. The sweep is I69's, and the desktop suite is I71's. | Split the attribution at the next touch. No number is wrong. |
| A1-N-3 | NOTE | RR's carry-over sentence, "That delta is T3 execution records only" | **#1101 is also the undertaking's work graph:** besides 714 paths in the T3 folder, it modifies `execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`. No maintained code names `WorkGraphs` or `WORK_GRAPH`, so the premise is unaffected. | Wording only: "execution records only (T3's folder and the undertaking's work graph)". |

## 1. The maintained content is what I reviewed

From `evidence/addendum_01/item1_content_and_history.txt`:

**The history.** `61c35f56a8`'s one parent is main `75a8c3291f`, and `75a8c3291f..61c35f56a8` is 1 commit. None of `b1e2d7741e`, `d449097085`, `69a925bd68`, `de01e43bc4`, `bd6b4be2c3` or `b4140b7645` is an ancestor. The branch carries no U8 or NUM history.

**The content.** `git diff 75a8c3291f 61c35f56a8` has 11 paths: the 7 maintained files and the 4 package files. Outside `execution/` the diff is 7 files, +29,026 / −14, and 6,372,777 B at the head.

**The blobs.** All 7 blobs equal `bd6b4be2c3`'s, and NUM `b4140b7645` is identical to the head over those 7 files.

**Main has not moved under them.** Main's blobs of the 5 modified files equal the U8 base `b1e2d7741e`'s, and the 2 fixtures are absent from both. Main's piping changes since the U8 base are #1100's 8 files, none of them among the 7.

So the head's maintained content is exactly what my two rounds reviewed (`b1e2d7741e..bd6b4be2c3`), on a main that changed nothing beneath it.

## 2. The package is truthful

**What I checked:** CHANGE_RECORD.md and PR_BODY.md at the head, against my two rounds, RV98's report, `U8_GATES/full_suite/` and RR. The package's `SHA256SUMS` is 3 of 3 OK.

**Accurate:**
- **§1–§2:** the 7 files and their hashes; each test's content; the W-C1 ladder; the L = 0 pins and value controls; 07l's counts and entries; the class counts.
- **§3:**
  - no production, `src`, schema, build-identity or CI change;
  - no Ceiling row;
  - F-1 is routed, not pinned;
  - no native Current evidence;
  - hosted CI is Stale-only, as ruled.
- **§4:** both of my rounds, their counts and the rulings on my notes. R2-N-1 extends E-6; R2-N-2 is routed to B1's touch, as RR rules. "The two rounds together cover the complete diff" is true.
- **Pass B row:**
  - `DELTAS TO READ`, exit 6, the gate vector of F, every gate 0 except `pp_outcomes`, whose only new outcomes are U8's three tests, all `ok`;
  - the entry, maxima 0.8881 / 0.8929 M, TEXT D 14,734, the 11 reviewed entries, the witnesses and the challenge all unchanged;
  - 10 rows added over F′: 8 `not-d1` documents from main and the 2 L = 0 fixtures, which the tool labels `unreachable` and I72 read as test rows;
  - all of this matches RV98 §2–§4.
- **RV98 row:** PASS 0/0/2, with the fixture rows `not-d1` by ROOT's ruling on RV98's recommendation.
  - **The reading is right.** The fixtures' only embedder is `#[test] fn u8_d_u6_5_l0_fixtures_are_the_live_successors` (`retained_facade_tests.rs:1051–1052`), in a module declared only under `#[cfg(test)]` (`PP/src/lib.rs:128–129`).
  - **My round-1 evidence agrees:** the fixtures are compared only by the U8 tests.
- **Full-suite row:** `compare_vs_Fp.txt` has 40 manifests, 38 identical. PP goes from (705, 1, 10) to (708, 1, 10) with +3 ok and `t13` failing on both sides; `result_export` goes from 172 to 173 with +1 ok. Its `SHA256SUMS` is 4 of 4 OK.
- **PR_BODY:** its claims are a subset of the record's, each one true.
  - "W-C1 … ends `Unresolved(Ceiling)`" is true (round 1), and is stated as an outcome, not an assertion.
  - "All three readers agree on every 07l document" is my round 2.
  - It says "agent reviews, not personal review by the owner".

**No overclaim found.** Two lines need care: A1-N-1 (stale main) and A1-N-2 (attribution).

## 3. The two tools reproduce ROOT's records

From `evidence/addendum_01/item3_tools.txt` and `tools_*`: main's `IMPLEMENTATION/F2A_D1/source_equality.py` (`fb96decf…`) and `check_citations.py` (`50c39a8a…`) have the same blobs on main, NUM and the head. I ran them in NUM's checkout, read-only.

| Run | Arguments | Result |
|---|---|---|
| `source_equality.py` | `--pr 61c35f56a8 --int b4140b7645 --main 75a8c3291f --package …/U8/` | rc 0, \|S\| = 7, 5 of 5 PASS. **`se.txt` and `se.json` are byte-identical to ROOT's** (`se.txt` with the work path normalised) |
| `check_citations.py` | `--base 75a8c3291f --head 61c35f56a8 --index …/U8/citations.json` | rc 0: 10 resolved, 0 ambiguous, 0 unresolved; the code anchor is pinned. **`citations.txt` is byte-identical to ROOT's** |
| Control | `source_equality.py` with `--pr` = main | rc 1; checks 1, 2, 4 and 5 FAIL |
| Control | `check_citations.py` without `--index` (#1082's index) | rc 1; 0 resolved, 6 unresolved |
| Cross-check | `source_equality.py` with `--int bd6b4be2c3` | 5 of 5 PASS, \|S\| = 7 |

## 4. The carry-over premise, ahead of time

From `evidence/addendum_01/item4_carry_over.txt`.

**DEC-025's surfaces** (`run_evidence_sweep.py`'s plan and the Mac driver):

| Surface | What it collects |
|---|---|
| cargo | manifests under `core/` and `validation/benchmarks/` only (`CARGO_SEARCH_ROOTS`; `target/` skipped) |
| pytest | `pytest -q tests`, with no other test paths configured |
| vitest | the wasm build, then vitest on `src/**/*.test.{ts,tsx}` under `apps/desktop` |
| build | the desktop production build |

**Readers of the T3 records.** Outside `execution/`, maintained code names the T3 folder in exactly three files, and each reads only `REFERENCES/references.{py,json}` and `DESIGN_NUMERICS/_run_records/floor_kinds.json`:
- `gen_k4_vectors.py`;
- `gen_vk_cases.py`;
- `run_harness_mutants.py`.

One data file also names it, `validation/portability_policy.json`. No piping code reads that file; its readers are repo-root tools, such as GEN-8, which is a separate gate.

**No broad walker reaches `execution/`.** The walkers in the DEC-025 suites cover `apps/`, `core/`, `tools/` or the cargo roots, or skip `execution` by name. The other `execution/` readers name specific non-T3 paths: PKG deliverable files, `_DAG`, and `ENGINE_INTEGRATION/UI_PRODUCER_CAPTURE.json`.

**#1101** (`11b2d04f13`, on `75a8c3291f`) is 712 added and 3 modified paths, all under `execution/`:
- 714 are in the T3 folder (including RR and D2's `DESIGN_STANDING/DESIGN.md`);
- 1 is the work graph (A1-N-3).

**0 paths are under `REFERENCES/` or `DESIGN_NUMERICS/`.** Its seven test-shaped record files (probe `*.test.ts`, `test_mutants.py`) lie outside every collection root.

**So N-12's premise holds for absorbing #1101.** Two things remain:
- A1-S-1's package overlap must be resolved first;
- GEN-8, which does read tracked records, must be re-run on the absorbed head, as it is for every head.

## Cleanup

`WT/scratch/rv97_u8_01/a1/` is deleted. Nothing else was written outside this report's files.

## Evidence (`evidence/addendum_01/`; placeholder paths only)

- `item1_content_and_history.txt`: the history, the diff, the blobs, NUM's equality and main's piping changes.
- `item3_tools.txt`: the commands, the byte comparisons and the controls. `tools_se.txt`, `tools_se.json`, `tools_citations.txt` and `tools_neg_*.txt` are the outputs.
- `item4_carry_over.txt`: DEC-025's surfaces, the T3 readers, the walkers, #1101's paths, and the package overlap with both PRs' blobs.
