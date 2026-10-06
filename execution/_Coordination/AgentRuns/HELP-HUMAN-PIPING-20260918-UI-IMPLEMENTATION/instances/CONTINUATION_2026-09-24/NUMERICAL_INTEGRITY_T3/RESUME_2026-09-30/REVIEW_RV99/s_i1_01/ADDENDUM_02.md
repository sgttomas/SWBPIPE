# RV99 addendum 02: S-I1's PR head, package and source equality

**Reviewer:** RV99, TASK (Type 2), the same reviewer as `REVIEW.md` and `ADDENDUM_01.md`, continued by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants.

**Placeholders:** as in `REVIEW.md`. `T` is the T3 records folder. No machine paths are recorded here or in `evidence/round2/`.

**Basis read:**
- ROOT's continuation message;
- RR "RV99 confirms S-I1's repairs; S-I1's PR is cut as the walking skeleton" and RR "S-I1's PR #1100 opened; the PR-head gates pass; DEC-025 started";
- ROOT's gate records in NUM `T/IMPLEMENTATION/S_I1_MERGE/_run_records/` (`SHA256SUMS.run_records`, 4 of 4 OK when checked from `_run_records/`).

**Work:** read-only Git (`GIT_OPTIONAL_LOCKS=0`) and two standard-library Python scripts. No cargo, no Git writes, nothing written outside `WT/scratch/rv99_s_i1_01/r2/` and this folder.

**PR head:** `20e7e3e5a2` (local branch head, equal to its remote-tracking ref). Base: main `c1571f7feb`.

## Verdict: **CONFIRMED.** The PR head carries exactly the reviewed source, and the package is truthful in substance.

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 4 (wording) |

## 1. The maintained diff equals what I reviewed

- **The maintained diff:** `git diff c1571f7feb 20e7e3e5a2` has 12 paths. These are the 8 slice files plus the 4 package files under `T/IMPLEMENTATION/S_I1/`.
- **The 8 slice files** have the same blobs at `20e7e3e5a2` as at `c26ecabbc1`, the head I confirmed in `ADDENDUM_01.md`. Their sha256 equal the change record's prefixes, and their line counts equal its table: evaluator +2,225 / −0 and runner +585 / −3.
- **The merge of main** `eddeab1e37` (parents `c26ecabbc1` and `c1571f7feb`):
  - it changed 209 paths, all under `projects/chirality-app-v4/`, and 0 under `projects/chirality-piping/`;
  - main changed no piping path between my base `c1bfc460fc` and `c1571f7feb`;
  - the merged tree differs from `c1571f7feb` in exactly the 8 slice files, and is identical to main everywhere else.
- **The package commit** `20e7e3e5a2` touches only its 4 files. Its `SHA256SUMS` verifies 3 of 3.

## 2. The package (CHANGE_RECORD.md and PR_BODY.md) against my review

**Truthful:**
- the 8 files, their changes and hashes;
- the design basis (D2 §4.11 at 5b.3, with ROOT's §4.11.3–§4.11.4 amendment);
- the interval mode and eager U;
- the runner outcomes;
- invalid and duplicate bounds blocking, and b = 0 running the point path;
- no product caller (S-I2);
- no schema shape, dependency or lock change;
- outside the D1 build closure (no Pass B);
- the pre-existing point-path panics (T3-SI1b), with interval mode never panicking on them;
- my figures: 30,609 evaluator and 6,615 runner cases, about 4.9M point and 450k exact samples, 63,086 differential runs, 94 + 30,515 parity cases, 46 of 46 mutants;
- my verdicts: PASS with 3 SHOULD-FIX, repaired and confirmed;
- N-6 and N-7 carried to S-I2;
- the suite counts;
- check_citations' 34 D2 citations at the 5b.3 pin. NUM `377d1d5cfb` holds D2 at sha256 `993f5f3a…` (5b.3); main holds `edc78f9c…` (5b.2).

The PR body's statement "These are agent reviews, not personal review by the owner" is right. Pending gates are marked "see the merge record" in the change record.

**No overclaim of substance.** Four wording points, none of which affects a decision:

| # | Sev | Where | Point | Suggested wording |
|---|---|---|---|---|
| N-8 | NOTE | `PR_BODY.md`, "Recorded on the integration branch: … hosted CI with the full-SHA dispatch, and the Mac DEC-025 …" | At this head, ROOT's records hold source equality, citations and GEN-8. CI is running and DEC-025 has started (RR, the #1100 ruling). As worded, the line is ahead of the records. | "Recorded on the integration branch: source equality, citations and GEN-8; hosted CI, the full-SHA dispatch and the Mac DEC-025 are recorded there before merge." Or update it when they land. |
| N-9 | NOTE | `PR_BODY.md`, "No soundness violation over 30,609 evaluator and 6,615 runner cases with exact-rational oracles" | The exact-rational oracle ran on evaluator cases only. Runner cases were checked against the point path (with dense re-sampling). Also, the 63,086 differential runs cover the committed packs and run-fixture rows **plus** demo variants and generated packs. | "…evaluator cases (point-path and exact-rational oracles) and 6,615 runner cases (point-path oracle)". For the differential: "over the committed rule packs, run-fixture rows, demo variants and generated packs: 63,086 runs". |
| N-10 | NOTE | `CHANGE_RECORD.md` §4, "Every box in which the point path both passes and fails reads U"; §3, "a NaN table argument" | In round 1, 15 straddling boxes whose inputs carried invalid explicit overlays were **blocked**, not U: still never a pass. The point path panics on a NaN **interpolation or step-lookup** argument; an exact lookup with NaN blocks (`TableKeyNotFound`). | "…reads U, or is blocked when an input's overlay is invalid"; "a NaN interpolation or step-lookup argument". |
| N-11 | NOTE | `CHANGE_RECORD.md` §3 and §5 | §5 attributes src-tauri 116 to "(I73, RV99)", but src-tauri was run by I73 only (my `REVIEW.md` "Not done"). §3 does not state the limit D2 §4.11.2 asks to be stated honestly: soundness is relative to the supplied b, whose basis is the stop rule's operational convergence evidence, not a forward-error enclosure. | Attribute src-tauri to I73. Add one line to §3: "Soundness is relative to the supplied bound b (D2 §4.11.2: operational evidence, not a forward-error enclosure)." |

## 3. `source_equality.py` and `check_citations.py`, rerun

**The tools** are main's copies from `T/IMPLEMENTATION/F2A_D1/`, exported unchanged. Their blobs (`fb96decf…`, `50c39a8a…`) are equal at main, at the PR head and on NUM. Commands are in `evidence/round2/commands.txt`. I used ROOT's arguments as recorded in `se.txt`: PR `20e7e3e5a2`, INT `b9030f501c`, MAIN `c1571f7feb`, and the package path. For check_citations: base main, head PR, and the PR head's `citations.json`.

| Tool | My result | ROOT's record |
|---|---|---|
| `source_equality.py` | exit 0, **RESULT PASS**, all 5 checks pass (\|S\| = 8; 8 of 8 identical in blob and mode; no merge rule needed; 4 execution files, all inside the package, sums verified; 8 rows equal) | `se.txt` **identical**; `se.json` **byte-identical** |
| `check_citations.py` | exit 0, **RESULT PASS**: 34 resolved, 0 ambiguous, 0 unresolved, 0 verification failures (all D2 §4.11.x, resolved at NUM `377d1d5cfb`) | `citations.txt` **identical** |

**They agree with ROOT's records.** I did not rerun GEN-8, and ROOT did not ask for it.

## Records

- `evidence/round2/`: `se.txt`, `se.json`, `citations.txt`, `resolved.md`, `commands.txt`, and `inputs_sha256.txt` (the tools and the exported package);
- `SHA256SUMS.addendum_02` covers this addendum and `evidence/round2/`;
- `REVIEW.md`, `ADDENDUM_01.md` and their sums are unaltered.
