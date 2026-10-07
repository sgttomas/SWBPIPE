# RV110 addendum 01: #1108's delta commit

RV110 (TASK, Type 2), resumed by ROOT (HELP_HUMAN, Agent 0), which is the return path. No descendants. I used Git reads with `GIT_OPTIONAL_LOCKS=0`, read-only `gh`, and the single GEN-8 pytest. I ran no cargo and no other test, made no Git write and installed nothing. `REVIEW.md` and its `SHA256SUMS` are unchanged.

**Placeholders:** as in `REVIEW.md` (WT, NUM, P, T, R, RR, WG, DT, VENV). In addition:
- H1 = `145443e9e4` (the head I reviewed);
- H2 = `ea3b1443ea4214ccb1975a25390fb382dc77674d` (the new head);
- N1 = `25c745f905`;
- N2 = `6f983f12f3307297e4d8762218560860f3904111`;
- M = `2007709549`;
- `ADD` = `_run_records/addendum_01/`.

**The basis:** ROOT's message, quoting RR "RV110 passes #1108 with S-1; I85's ignored evidence committed; errata E-7 to E-9; #1108 takes the delta" (NUM N2), with ROOT's proposed squash subject and body.

## Result: **CONFIRMED**

| BLOCKING | SHOULD-FIX | NOTE |
|---|---|---|
| 0 | 0 | 3 |

S-1 is fixed in the committed tree. The delta is exactly NUM's N1..N2 under `execution/`, it screens clean, GEN-8 and the four automatic CI runs pass on H2, and main has not moved. N-2 and N-3 are applied truthfully. Three small residuals are noted below; none affects the PR's content or blocks the squash. **The squash body needs one correction (A1-N-1).**

| ID | Sev. | Where | Finding | Remedy |
|---|---|---|---|---|
| A1-N-1 | NOTE | ROOT's proposed squash body | **One phrase is untrue as written.** "the reviews RV104, RV107, RV108, RV109 and RV110 (each with its addenda)": RV110's addendum (this file) is not in H2, which carries only RV110's `REVIEW.md` and its run records. The rest of the body is accurate against M..H2 (§6). | Write "RV104, RV107, RV108 and RV109 (each with its addenda) and RV110's review". Optionally add the counts "869 added, 2 modified, 0 deleted", as #1105's body did. Keep the agent co-author trailer. |
| A1-N-2 | NOTE | WG at H2: "Next safe action" 3; the rulings in force | **The work graph is partly stale in its own commit.**<br>- Item 3 still reads "#1108 (head `145443e9e4`, NUM's `execution/` at `25c745f905`; …): RV110 reviews it". The same commit's RR section moves #1108 to the delta head and records RV110's PASS.<br>- My N-2 also named RV104 N-6's pytest-under-lock rule (RR:13590). It was not added to the rulings in force.<br>- The host rule for waits is now listed twice. | At the next WG touch. |
| A1-N-3 | NOTE | RR (H2) E-7, "ROOT's own check … 26 files and 790 entries"; `WT/records-pr-b` | **Two small facts:**<br>- **E-7's count predates two seals in the same commit.** At H2 the sum files added since #1105 are **28, with 832 entries, all OK.** 28 − 26 and 832 − 790 are exactly B6_MERGE's `SHA256SUMS.addendum_01` (1) and RV110's `SHA256SUMS` (41).<br>- **The PR checkout has a stray file.** `WT/records-pr-b` holds an untracked, empty `ee_code_nontest.diff` at its root, modified 14:27:43Z. It is not mine and not in any commit; GEN-8 passes with it present. | Optional erratum. Remove the stray file from the checkout. |

## 1. The delta equals NUM's N1..N2: confirmed

`ADD/delta_scope.txt`, `delta_name_status.tsv`:
- **Parentage:** H2's only parent is H1, and `rev-list --count M..H2` = 2. `ls-remote` gives the branch = H2 and main = M. None of NUM's commits in M..N2 is an ancestor of H2.
- **Equality:**
  - **H2's whole tree equals N2's** (`45a54c1109…`), and `P/execution` = `329f980cf4…` on both sides;
  - `git diff N2 H2` is empty;
  - **`git diff --raw H1 H2` is byte-identical to `git diff --raw N1 N2`**, which covers paths, modes and blobs.
- **Outside `execution/`:** 0 paths, for M..H2, H1..H2 and N1..N2 alike.
- **The delta has 105 paths: 103 A and 2 M (RR and WG), 0 D**, all mode 100644. In NUM, the 4 commits in N1..N2 are `b08dad8c19`, `7edc00f021`, `a1180c37ce` and N2. The added paths are:
  - B6_MERGE's `gen8_addendum_01.txt` and `SHA256SUMS.addendum_01` (2);
  - 3 briefs (`B1_SR_RS`, `RV110_RECORDS_PR6_REVIEW`, `RV111_SI1C_REVIEW`);
  - I85's 4 `build/` files;
  - I88's return (52);
  - RV110's review (42).
- **No sealed file is modified.** B6_MERGE's `SHA256SUMS` blob is unchanged from H1.
- **Totals and the PR:** M..H2 is 869 A, 2 M and 0 D, which GitHub reports as 871 files. The only other open PR is #885 (not T3's). No SI1c PR is open, and no package is in the delta.

## 2. S-1 is fixed: confirmed

From `ADD/s1_fixed.txt`, `sums_verify_delta.tsv` (an archive of H2) and `sums_committed_tree.py`/`.tsv` (blobs read straight from H2's tree):
- **The four files are in H2**, mode 100644: `build01_norun.log` `5012d058…`, `build_identities.txt` `5f3f2afd…`, `lib01.log` `4994ff18…`, and `repair_01/build/r1_lib01.log` `d87b5612…`.
- **`R/I85/b1_st_01/SHA256SUMS` verifies 60/60 and `SHA256SUMS.repair_01` 34/34** in the committed tree. No file in the folder is uncovered.
- **The citations resolve:**
  - RETURN.md:151's `_run_records/build/build_identities.txt` is in H2;
  - RETURN.md:262's `build/` holds the three files;
  - REPAIR_01.md:152's `build/`, under `_run_records/repair_01/`, holds `r1_lib01.log`.
- **Every sum file added since #1105 verifies in H2's committed tree:** 28 sum files and 832 entries, with 832 OK, 0 bad and 0 missing. This includes the delta's three:
  - B6_MERGE's `SHA256SUMS.addendum_01` (1/1), with `SHA256SUMS` still 17/17;
  - I88's `SHA256SUMS` (51/51; 0 uncovered);
  - RV110's `SHA256SUMS` (41/41).
- **My `REVIEW.md` in H2** is `5b302199…`, as I returned it.
- **NUM's `git status --ignored` under `P/execution`** now shows only the pre-existing `__pycache__` folders and RV56's `imported/` folders, the latter as ROOT states. The `build/` files are tracked.

## 3. The publication screen holds on the delta: confirmed

`ADD/publication_scan_delta_summary.txt`, `publication_delta_hits_summary.txt`, `token_shape_scan_delta.txt` and `abs_paths_delta.tsv`. The scripts are REVIEW.md's, run over H1..H2: 12,130 lines, every line of the 103 added files plus the `+` lines of RR and WG.
- **Token-shaped strings: 0.** All 2,446 name-pattern hits are in RV110's own records (scan vocabulary), and **0** are outside them.
- **E-mail:**
  - the owner's e-mail occurs 0 times in the delta;
  - the only address is the agent's no-reply attribution;
  - six `+@pytest.mark` lines are false positives.
- **Machine-absolute paths: 0.** GEN-8's detector finds 0 lines. The broad pattern finds only my own records' placeholders (`/U-sers/<user>`, `~/dev`). RR's two whole-file hits lie in main's prefix.
- **Host data: none.** I88's lock log has 504 lines, all its own (`cwd=WT/scratch/i88_si1c/…`); there is no process listing and no other agent's line. I85's four logs have 0 home, private-temp or tmp paths.
- **Size:** the delta's largest file is RR itself (1,212,894 B), and every file is text.
- **The originals:** none of the 13 redacted originals is in H2's tree (66,454 blobs), and none of the 871 changed files hashes to one (`ADD/redactions_check_H2.txt`).

## 4. N-2 and N-3 are applied truthfully: confirmed (residuals A1-N-2, A1-N-3)

**RR is append-only** (`ADD/rr_append_only_delta.txt`):
- H1's RR (1,206,006 B) is an exact byte prefix of H2's (1,212,894 B);
- H2 appends 59 lines in two sections: I88's ruling and the RV110 section.

**E-7** states S-1's cause, files, rule and repair correctly (§2). Its sharpened rule (`git status --ignored` before committing a return) addresses the cause. Its count is A1-N-3.

**E-8:** PR_CUT.txt's `target_base=47a3bdfcf5` against run 37555520168's full `47a3bdfcf5a37e856465c45cf904383f10181498` is exactly REVIEW §4.1's finding.

**E-9:** "49 → 57" is RR "RV104 confirms SI1b's repair round; …" and counts the lib tests; "50 → 58" is RR "#1106 merged: …" and is DEC-025's manifest total with the 1 conformance test. Both are as REVIEW §4.1 and `dec025_counts.txt` show.

**The gen8 addendum** (`IMPLEMENTATION/B6_MERGE/_run_records/gen8_addendum_01.txt`, sealed 1/1):
- It states head `1199726f69…`, the E-4 command, cwd `WT/b6-pr`, and gen8.txt's "1 passed, 10 deselected in 32.85s".
- That agrees with RR:14111 and with `WT/b6-pr`'s HEAD (`1199726f69`).
- It is a later attestation, not a fresh capture, and it says it is an addendum.

**N-1** is recorded with my grounds (RR:12374's rule and decision 11). ROOT's "so none reaches a user" holds for any product-produced successor. My caveat, that a dev/test-built or hand-made saved file would show the code, stands as written in REVIEW.md.

**N-2 in WG (H1..H2), item by item:**
- **Superseded decisions:** the 6.0 GiB and 64 GiB owner decisions are marked superseded; the 64 GiB one still covers host jobs.
- **Rulings in force:**
  - the M entry is re-pointed to the 12 GiB section, with the 64 GiB section for host jobs;
  - S3, R1, PLAN_v2/A1, the quiet-check fix, the wait rule and I88's points are added;
  - every added heading exists in RR. `ADD/heading_citations_check_H2.txt` resolves 37 of 38 quotations; the 38th is the quoted phrase "successors are not public before B8", not a heading.
- **The IDs line** is consistent: I68–I90 and RV101–RV111, next unused **I91 and RV112**.
  - I91 occurs only in RR:14206, WG:600 and my REVIEW's records ("I91 … occur nowhere" at N1).
  - RV112 occurs only in RR:14206 and WG:600.
- **The B1/B6 row** gives ST at `a8e719f5b4`, repair `98a77c716e`, and I1 `262bd687f0`, as Git shows.
- **Notes routed** now carries RV108's notes (SR-PY, SR-TS, SC, N7 no action), matching RR and RV108 ADDENDUM_01 A-N2, and RV109's N-2 → SP, N-3 → SA and N-5 → SQ.

**I88's section, spot-checked** (`ADD/living_docs_delta.txt`):
- RETURN.md is `7459391d…`, with 51/51 sums.
- `codex/piping-t3-si1c-20261007` is at `7f233b2e01`: four commits over `025c1cf326`, with 5 files, +1,271/−140.
- `codex/piping-t3-b1-r-20261007` descends from I1 `262bd687f0`.

## 5. GEN-8 on H2, CI and main: confirmed

**GEN-8** (`ADD/gen8_pytest_H2.log`):
- E-4's method, in `WT/records-pr-b`, with HEAD = H2 before and after.
- The command was `VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8`.
- **1 passed, 10 deselected in 29.63s**, exit 0, 14:38:00–14:38:30Z.
- The checkout's only status entry, before and after, is the stray empty file (A1-N-3).

**CI on H2** (`ADD/ci_runs_H2.txt`): all four automatic runs succeeded:
- Piping Desktop E2E 37637875013;
- Harness Pre-merge Validation 37637874699;
- pec-tests 37637874697;
- governance-harness 37637874737.

**Main** (`ADD/main_check.txt`): `ls-remote` gives `2007709549e9701b302e0eb62a1474d82acc1c40`, unchanged. GitHub reports #1108 MERGEABLE and CLEAN, not draft.

## 6. The squash body against N-4

Each item was checked against M..H2.

**True:**
- "NUM's … `execution/` at 6f983f12f3";
- "after #1105 (47a3bdfcf5)";
- the #1106 and #1107 merges;
- RV106's review and RECORDS_MERGE_2026-10-07;
- SESSION_2026-10-07;
- I79's REPAIR_01;
- SI1B_MERGE, and B6_MERGE with its GEN-8 addendum;
- I81, I82, I84, I85 (with the repair round and E-7's four files) and I86;
- I83's B6 return;
- I87's plan, with the owner's decision in RR;
- I88's return;
- the briefs;
- errata E-7 to E-9 in RR;
- RR and WG;
- "B6's package and the RV103 and RV105 reviews were already on main";
- "Nothing outside execution/";
- "agent reviews, not personal owner review".

**Correct:** RV110's addenda are not in H2 (A1-N-1).

**The subject** is accurate.

## 7. Host and disclosure

- **The copy:** `WT/rv110/` held a fresh `git archive` of H2's B6_MERGE, I85, I88, REVIEW_RV110 and BRIEFS (576 files). It is deleted.
- **Scratch:** in `WT/scratch/rv110_records_01/addendum_01/`, with `TMPDIR` in my scratch.
- **Waits:** none was needed. GEN-8 ran in the foreground, and no process of mine remains.
- **Nothing outside my scratch and this report was written.** In `WT/records-pr-b` I wrote nothing.
- **Records:** this file and `ADD/*`, sealed by `SHA256SUMS.addendum_01`, with placeholder paths only.
