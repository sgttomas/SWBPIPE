# RV108 addendum 01: B6's PR head against the review

RV108 (TASK, Type 2), resumed by ROOT (HELP_HUMAN, Agent 0), which is the return path. RV108 did not delegate. This addendum uses documents and Git reads only, with `GIT_OPTIONAL_LOCKS=0`: no cargo, vitest or pytest, and no Git write.

**Placeholders:** as in `REVIEW.md` (`WT`, `NUM`, `P`, `T`, `R`, `RR`, `VENV`, `S`). In addition, `PRH` = the PR head `1199726f695f92ddeccc36f7b47079dedf5b9854`; `MAIN` = `025c1cf326`; `SLICE` = B6's reviewed head `a7de2a918f`; `NUMH` = NUM at `c698a0b9a5`; `PKG` = `T/IMPLEMENTATION/B6/`; `ADD` = `R/REVIEW_RV108/b6_01/evidence/addendum_01/`.

**The basis:** ROOT's request, quoting RR "Owner decision: SI1c is option D, a repair within grammar 1.0.0; RV108 passes B6; RV109 passes ST with SF-1" (read at NUMH). PR #1107, branch `codex/piping-t3-b6-pr-20261007`, worktree `WT/b6-pr`.

## Verdict: **CONFIRMED**

**Counts: 0 BLOCKING, 0 SHOULD-FIX, 3 NOTE.**

PRH carries exactly the source I reviewed at SLICE, plus a package that matches it. The three notes concern the package's wording and the citation index's coverage. None of them changes the PR's content or its review.

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| A-N1 | NOTE (wording) | `PKG/CHANGE_RECORD.md` §2 item 3; `PKG/PR_BODY.md` "What changes" | Both call Python's transport validator "the twin of Rust's and TypeScript's" without qualification. This repeats the claim REVIEW N6(a) found too broad: the twin is exact on G0–G2 only, and at the base step Python runs both Rust's header check and TS's preview transport-metadata check. | At the next touch, say "the twin on G0–G2". Nothing to change in the PR's source. |
| A-N2 | NOTE (record completeness) | `PKG/CHANGE_RECORD.md` §4 and §6; `PKG/PR_BODY.md` "Review and gates" | §6 carries RR's routing only in part:<br>• it omits SC's corpus entries for N1, N2 and N4 ("SC adds their corpus entries"; "SC adds its entry");<br>• it omits SC's transport scope sentence (N6);<br>• neither RR nor §6 routes N6(a)'s actual target, the "twin" wording in Python's `validate_retained_precision_transport` docstring. RR routes "TS's doc comment"; that comment's overclaim is N1's, and it becomes true when SR-PY repairs N1.<br>Separately, "the notes are pre-existing reader differences and wording" (§4; PR body) does not describe N5 (test strength) or N7 (an account error, no action). | Add the missing SC items, and the Python docstring under SR-PY or SC, to B1's carried obligations (records only). |
| A-N3 | NOTE (citation index coverage) | `PKG/citations.json` `about` ("every record citation that T3-B6's PR adds"); `RE/tests/retained_precision_carriers.rs` and `P/tests/test_retained_precision_carriers.py` (one added comment line each) | Both added comments cite `R/REVIEW_RV92/u6f_01`. `check_citations.py` does not see that form: its `review_path` pattern refuses a match preceded by `/`, and its `record_path` pattern needs `R/I<n>`. So the index and the check count 7 citations, all on the carrier file's line 4, and these two are outside both. The path resolves at main, at the index's NUM commit and at NUMH. It is the same record as the indexed `RV92 u6f_01`. Nothing is broken; the "every citation" claim is short by two. | None for the PR. At a later tooling touch, let the checker's grammar accept `R/REVIEW_RV<n>/…`. |

## 1. Scope

`ADD/pr_commit_and_files.txt`, `ADD/ls_remote.txt`:
- **The PR head:** PRH is one commit whose only parent is MAIN, and main has not moved: `git ls-remote` gives `refs/heads/main` = MAIN, and both the branch and `refs/pull/1107/head` = PRH.
- **The diff from MAIN is 15 files:**
  - the **11** files I reviewed (`M`);
  - the **4** package files under PKG (`A`): `CHANGE_RECORD.md`, `PR_BODY.md`, `citations.json`, `SHA256SUMS`, each mode `100644`.
  - There is no other path. Nothing is under PP, a D1 crate's `src`, the schemas or other fixtures.
- **Hygiene:** `git diff --check` is clean. No added line, and no package file, carries a machine path.

## 2. Equality

`ADD/blob_equality.txt`, `ADD/file_sha256_numstat.txt`, `ADD/source_equality_rerun.txt`:
- **By blob:** each of the 11 files has the same blob at PRH, SLICE and NUMH (11 of 11). The 4 package files are equal at PRH and NUMH, and absent at SLICE.
- **By content:**
  - the corpus at PRH is `c21112fd…` (07m), and the case file `98a7213a…`, as reviewed;
  - every per-file sha256 prefix and `+/−` count in the change record's §1 table matches PRH.
- **ROOT's `source_equality.py`, reproduced:** `T/IMPLEMENTATION/F2A_D1/source_equality.py` (sha256 `18786526…`), run with Git reads only, its work files in `S`. It gives **PASS on all 5 checks** in two runs:
  - `--int SLICE`: B = BASE `bfb26596bf`, |S| = 11;
  - `--int NUMH`: B = MAIN, |S| = 11.
  - In both runs the 11 paths are identical in blob and mode, no three-way merge is needed, the 4 execution files lie inside the package, and the package's SHA256SUMS verify.
- **NUMH:** `1732ba108e` (B6's `--no-ff` merge) is an ancestor of NUMH, and NUMH adds only the 4 package files to it.

## 3. The package against my review

**`CHANGE_RECORD.md`** (sha256 `7862b89a…`, equal to the package's SHA256SUMS):
- **True as written:**
  - §1: files, changes, sha256 prefixes and counts;
  - §2 items 1, 2 and 4;
  - §3: no PP, D1 `src`, schema, other-fixture, dependency or lock change; eligibility stays closed; no existing corpus entry moves, only 277's TS value changes; RV78-N1 goes to B3;
  - §4: I83's mutants, and my 1,032 probes, 98 tampered transports, the raw differential, 30 mutants and PASS 0/0/7;
  - §5: the suites, exactly my figures.
- **Two exceptions:** the "twin" wording in §2 item 3 (A-N1), and the routing gaps in §4 and §6 (A-N2).

**`PR_BODY.md`** (`a8375cdc…`):
- **True as written:** five aligned classes, each pinned by a new entry; the transport validator; decision 8; 07m; the six-to-five entries; no build-identity change; no dropped assertion; the review counts; "agent reviews, not personal review by the owner".
- **Exceptions:** the same as above (A-N1; A-N2's sentence about the notes).

**`citations.json`** (`571648d8…`; `ADD/citations_independent.txt`, `ADD/check_citations_run.txt`):
- **The seven citations:** four runs (`RV92 u6f_01`, `RV92 u6f_02`, `I66 u7_slice_f_01`, `I67 u7_slice_f_01`) and three RR titles.
- **All seven on line 4:** each token is on the carrier file's line 4, the `scope` line, at PRH.
- **Present before B6:** each is also on line 4 at BASE and at MAIN.
- **They resolve:**
  - the four run folders exist at the index's NUM commit `1732ba108e`, at NUMH and at MAIN;
  - the three RR headings are at the recorded lines 10346, 10501 and 10836 in each.
- **Main's checker:** `check_citations.py` (the copy at PRH, sha256 `0ccfaecb…`, equal to NUM's) gives **RESULT PASS**: resolved 7, ambiguous 0, unresolved 0, verification failures 0, unused entries 0.
- **The one gap:** A-N3.

## 4. Anything else at the head

- **The commit message** is accurate: its four items, the package, and "Implemented by I83; reviewed by RV108 (PASS, 0/0/7). Agent reviews, not personal owner review", with agent attribution.
- **RR's rulings on my notes** match my findings.
- **RV107 A1-N-10** is recorded as closed, as I recommended.
- Nothing else at PRH would draw a finding from my review.

## 5. Host and disclosure

- **Git reads only:** `rev-parse`, `diff`, `show`, `log`, `ls-tree`, `cat-file`, `merge-base` and `ls-remote`, with `GIT_OPTIONAL_LOCKS=0`. ROOT's two Python checkers were run as read-only Git scripts. No cargo, vitest or pytest; no wait was needed; no process of mine remains.
- **Disclosed:** one shell redirect of a `git diff --name-status` listing wrote a 1.6 KB file to the system temp directory by mistake. I deleted it at once. Everything else is in `S/addendum/`.
- **Records:** this file; `SHA256SUMS.addendum_01` over this file and `ADD/*`; placeholder paths only.
