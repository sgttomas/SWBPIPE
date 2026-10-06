# RV97 Addendum 02: #1102 from `61c35f56a8` to `f7a7572e35`

**Reviewer:** RV97, TASK (Type 2), continued by ROOT (HELP_HUMAN, Agent 0) for a short, read-only confirmation of everything since the head that ADDENDUM_01 confirmed. No descendants.

**Basis:**
- **The rulings:** RR "RV97 confirms #1102's head; A1-S-1 ruled: U8's package reaches main only with #1102", "#1101 squash-merged; RV102's addendum notes" and "#1102 absorbs main; DEC-025 carried to its head by ruling";
- **ROOT's records at the new head:** `IMPLEMENTATION/U8_MERGE/_run_records/` (`se3.txt`, `se3.json`, `citations3.txt`, `gen8_3.txt`). The copies in `WT/scratch/u8_pr/` are byte-identical to them.

**The earlier reports are sealed.** `REVIEW.md`, `REVIEW_ROUND2.md`, `ADDENDUM_01.md` and their sum files are unchanged. This addendum has its own sum file, `ADDENDUM_02.SHA256SUMS`.

**Host:** Git reads only (`GIT_OPTIONAL_LOCKS=0`) in NUM's checkout. I changed nothing there except this report's files; another reviewer's untracked `REVIEW_RV101/` folder appeared during the check, and I did not touch it. No cargo, no Git writes, no fetch. Scratch was in `WT/scratch/rv97_u8_01/a2/`, deleted at the end.

## Verdict: **PASS. `f7a7572e35` is confirmed, and the DEC-025 carry-over premise holds for both moves.**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 1 |

1. **The delta is exactly as stated.** Nothing outside `projects/chirality-piping/execution/` differs between `61c35f56a8` and `f7a7572e35`, or between `75a8c3291f` and `d069e7130c` (0 paths each).
   - `b18dd4f369` changes only `IMPLEMENTATION/U8/CHANGE_RECORD.md` and `SHA256SUMS`.
   - The merge `f7a7572e35` is a clean union: its change from `b18dd4f369` is exactly main's change from `75a8c3291f` to `d069e7130c` (the same paths, patch-id `845fdf42…`), and its change from `d069e7130c` is exactly the PR's change from `75a8c3291f`.
   - Main's move is #1101's records only: 758 added and 3 modified, 760 in the T3 folder plus the work graph, with nothing under `IMPLEMENTATION/U8/` (A1-S-1's ruling holds).
2. **A1-N-1 and A1-N-2 are applied truthfully.**
   - **A1-N-1:** the record now names main `75a8c3291f` with #1100, and "through `75a8c3291f`" in §1. The load-bearing claim (#1100's files include none of U8's 7) is true. One minor imprecision: A2-N-1.
   - **A1-N-2:** the Python and TS rows now credit I69 and I71, and state what RV97 ran. Both statements are accurate.
   - The package's `SHA256SUMS` is 3 of 3 OK at the head, and all four package blobs equal `b18dd4f369`'s and NUM `55f5b7ecf1`'s.
3. **The carry-over premise holds for both moves.**
   - The two heads differ, and the two mains differ, only under `execution/`, in U8's package text and #1101's records.
   - No maintained code names `IMPLEMENTATION/U8`, `WorkGraphs` or `WORK_GRAPH`.
   - Maintained piping code names the T3 folder only in the three readers (and in `portability_policy.json`, which no piping code reads), and #1101 has 0 paths under `REFERENCES/` or `DESIGN_NUMERICS/`.
   - Its seven test-shaped record files lie outside every DEC-025 collection root (ADDENDUM_01 §4), and no collection code changed.
4. **Both tools agree with ROOT's records byte for byte at `f7a7572e35`:** `se3.txt` (with the work path normalised), `se3.json` and `citations3.txt`. As a control, the old head `61c35f56a8` against the new main and NUM fails check 4 (765 execution files outside the package), so the tool does see the absorption.

## Finding

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| A2-N-1 | NOTE | `IMPLEMENTATION/U8/CHANGE_RECORD.md` (`f5ebd6b9…`), the "Main has moved since" bullet | **The summary of #1100 leaves out one category.** It reads "#1100 (S-I1: 8 piping files in the rules crates, Python and schemas, none of them U8's 7)". Of the 8 files, 4 are in the rules crates, 2 are Python, 1 is a schema, and 1 is a fixture, `fixtures/rule_interval/rule_interval_cases.json`. The load-bearing clause, "none of them U8's 7", is true (`delta_and_premise.txt`). | None needed. Optionally add "and a fixture" at the next touch. |

## 1. The delta (`evidence/addendum_02/delta_and_premise.txt`)

**The commits:**

| Commit | Parents | What it is |
|---|---|---|
| `b18dd4f369` | `61c35f56a8` | the package wording |
| `f7a7572e35` | `b18dd4f369`, `d069e7130c` | main merged in |
| `d069e7130c` | `75a8c3291f` | #1101's squash |
| `55f5b7ecf1` | `9b8a18a20c`, `d069e7130c` | NUM absorbing main |

**Outside `execution/`:** `61c35f56a8` against `f7a7572e35` gives 0 paths, and `75a8c3291f` against `d069e7130c` gives 0 paths.

**Under `execution/`:**
- **The head move** is 758 added and 5 modified: #1101's 758 added and 3 modified, plus the 2 package files.
- **The two sides of the merge** are each exactly the other side's change, by name list, patch-id and full diff text.
- **#1101's 761 paths:**
  - 760 are in the T3 folder, of which 2 are modified: RR and D2 (`DESIGN_STANDING/DESIGN.md`);
  - 1 is `_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md` (modified);
  - 0 are under `IMPLEMENTATION/U8/`.

## 2. A1-N-1 and A1-N-2 (`evidence/addendum_02/change_record_A1_diff.txt`)

- **A1-N-1:**
  - The bullet now reads: "This PR is cut from main `75a8c3291f`, which includes #1098 and #1099 (App v4 only) and #1100 (…), none of them U8's 7). NUM carries main plus U8 at `b4140b7645`."
  - §1 now reads: "main's later commits through `75a8c3291f` touch none of the 7".
  - **Checked:**
    - `c1bfc460fc..c1571f7feb` has 0 paths outside `projects/chirality-app-v4/`;
    - #1100's 8 piping files include none of the 7;
    - main's blobs of the 5 modified files equal the U8 base's (ADDENDUM_01 §1);
    - `b4140b7645` was NUM's merge of U8 at the cut.
  - See A2-N-1 for the category list.
- **A1-N-2:**
  - **Python row:** "I69 (RV97 ran the retained suites and the 07l parity)". This is true: round 2 ran the three retained suites (480) and the 348-document parity.
  - **TS row:** "I71 (RV97 ran `retainedPrecision.test.ts` and `tsc`)". This is true: round 2.
  - The other rows were already accurate: PP registered "I68, RV97, I72", `result_export` "I70, RV97", PP Stale "I68".
- **Nothing else changed:**
  - `PR_BODY.md` and `citations.json` are unchanged since `61c35f56a8`;
  - the package's `SHA256SUMS` verifies against the head's blobs (3 of 3 OK);
  - the four package blobs at `f7a7572e35` equal `b18dd4f369`'s and NUM `55f5b7ecf1`'s.

## 3. The carry-over premise, for both moves

**DEC-025 runs on `61c35f56a8` with a baseline of `75a8c3291f`, and is carried to `f7a7572e35` against `d069e7130c`:**
- **The candidate side differs only in** the 2 package texts and #1101's records.
- **The baseline side differs only in** #1101's records.

**Neither is read by a DEC-025 surface:**

| Check | Result at `f7a7572e35` |
|---|---|
| DEC-025's collection roots (ADDENDUM_01 §4) | Unchanged: no change to `P/tools`, `apps/desktop/vite.config.ts` or `P/package.json` in the head move |
| Maintained piping files naming the T3 folder | Only the three readers and `validation/portability_policy.json`. The readers read only `REFERENCES/references.{py,json}` and `DESIGN_NUMERICS/_run_records/floor_kinds.json`; no piping code reads the policy file |
| Maintained code naming `IMPLEMENTATION/U8`, `WorkGraphs` or `WORK_GRAPH` | None |
| #1101 under `REFERENCES/` or `DESIGN_NUMERICS/` | 0 paths |
| #1101's test-shaped files | All 7 under `execution/`, outside every collection root (cargo: `core/` and `validation/benchmarks/`; pytest: `tests/`; vitest: `apps/desktop/src/**`) |

**GEN-8, which does read tracked records,** was re-run by ROOT on the exact head: `gen8_3.txt`, 1 passed. The premise does not depend on it.

## 4. The tools at `f7a7572e35` (`evidence/addendum_02/tools_*`)

**The tools:** main's `F2A_D1/source_equality.py` (`fb96decf…`) and `check_citations.py` (`50c39a8a…`), with the same blobs on main `d069e7130c`, at the head and in NUM's working tree. The index is `IMPLEMENTATION/U8/citations.json` (`ab044a39…`), the same blob at the head and in the working tree.

| Run | Result | Against ROOT's record |
|---|---|---|
| `source_equality.py --pr f7a7572e35 --int 55f5b7ecf1 --main d069e7130c --package …/U8/` | rc 0; B = `d069e7130c`, \|S\| = 7, checks 1–5 PASS | **`se3.txt` identical** (work path normalised); **`se3.json` identical** |
| `check_citations.py --base d069e7130c --head f7a7572e35 --index …/U8/citations.json` | rc 0: 10 resolved, 0 ambiguous, 0 unresolved; 1 code line pinned | **`citations3.txt` identical** |
| Control: `source_equality.py --pr 61c35f56a8` with the same INT and main | rc 1: check 4 FAIL, 765 execution files outside the package | — |

## Cleanup

`WT/scratch/rv97_u8_01/a2/` is deleted. Nothing else was written outside this report's files.

## Evidence (`evidence/addendum_02/`; placeholder paths only)

- `delta_and_premise.txt`: the commits, the outside-`execution/` comparisons, the delta structure, #1101's content, the readers, and the package checks.
- `change_record_A1_diff.txt`: the CHANGE_RECORD diff from `61c35f56a8` to `b18dd4f369`.
- `tools_se3.txt`, `tools_se3.json`, `tools_citations3.txt` and `tools_neg_se_oldhead.txt`: the tool outputs.
