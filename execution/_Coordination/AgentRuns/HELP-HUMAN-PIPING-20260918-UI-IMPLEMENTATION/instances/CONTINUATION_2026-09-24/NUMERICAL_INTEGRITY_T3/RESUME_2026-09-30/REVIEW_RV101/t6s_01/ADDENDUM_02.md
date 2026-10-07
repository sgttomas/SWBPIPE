# RV101 addendum 02: the T6S PR head (#1104): scope, package, Pass B's premise and equality

**Reviewer:** RV101, TASK (Type 2), for ROOT (HELP_HUMAN, Agent 0). No descendants. 2026-10-06 UTC.

This is the same-reviewer confirmation of the PR head that carries the slice I reviewed. The review (`REVIEW.md`, `510fbdb5…`) and ADDENDUM_01 (`461507ab…`) are unchanged. This addendum has its own sum file, `ADDENDUM_02.SHA256SUMS`.

**Placeholders:** `WT`, `NUM`, `P`, `DT`, `RE`, `R` as in the review. `T` is the T3 records folder. No machine paths appear here or in `addendum_02/`.

**Basis read:**
- ROOT's request;
- RR's sections "RV101 confirms SF-1's repair; T6S merged into NUM" and "I80's package accepted; the full-suite and Pass B rulings for T6S; T6S's PR cut";
- the package at the PR head: `CHANGE_RECORD.md`, `PR_BODY.md`, `citations.json` and `SHA256SUMS`;
- I80's `_draft_run_records/outputs/passb_scope_scan.out` (for its counts only);
- the live PR body (`gh pr view`, read only).

**Candidate:** PR #1104, branch `codex/piping-t3-t6s-pr-20261006`, head `d953e121872576dfa998c676f51ac803d48426be`, on main `d8c88774d0a73bc99fe9c6e906db6296162f8953`.
- **Remote heads,** by `git ls-remote`: main is `d8c88774d0` and the PR branch is `d953e12187`.
- **NUM** is `0416b3b2ce`, as ROOT named it. NUM has since moved to `53f626a5c8`, a records-only commit (`T6S_MERGE/_run_records/`) that changes no maintained path. I checked against both.

**Method.** I used Git reads (`GIT_OPTIONAL_LOCKS=0`), `gh` reads and main's two tools, with my own `--work` directories under `WT/scratch/`. I ran no cargo, vitest, `tsc` or Git write, and nothing went to the system temp directory.

## Verdict: **CONFIRMED**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 4 |

## 1. The head's scope (`addendum_02/scope.txt`)

- **One commit on main.** The head's only parent is `d8c88774d0`. `rev-list --count d8c88774d0..d953e121` = 1.
- **No foreign history.** None of the slice commits (`055ee0c0bc`, `2033260c57`, `fdcdb5e024`) or the NUM commits (`0416b3b2ce`, `53f626a5c8`) is an ancestor of the head.
- **Exactly 23 paths, all `A` or `M`, with no deletion:**
  - the 19 slice files: 9 added and 10 modified, +2,062 / −1,903;
  - the 4 package files, all added: `T/IMPLEMENTATION/T6S/{CHANGE_RECORD.md, PR_BODY.md, citations.json, SHA256SUMS}`.
  
  No other path changes.
- **Blob equality.** Each of the 19 blobs at the head equals its blob at `fdcdb5e024`, at NUM `0416b3b2ce` and at NUM `53f626a5c8`. All 19 are mode 100644.
- **Main's later commits** (`c1bfc460fc..d8c88774d0`, first parent) touch none of the 19, so the PR's diff against main is the slice's diff:
  - #1098 and #1099: App v4 only;
  - #1100: S-I1, 8 maintained piping files;
  - #1101 and #1103: records only;
  - #1102: U8, 7 maintained piping files.
- **The package** blobs equal NUM's.
- **The live PR body** equals `PR_BODY.md`, apart from one trailing newline.

## 2. The package tells the truth

**`SHA256SUMS` verifies on the PR's blobs:** 3 of 3 OK (`package_sums_on_pr_blobs.txt`).

**CHANGE_RECORD.md, checked against Git:**
- **Each table row is correct:** the per-file +/− counts (`git diff --numstat`), the new files' line counts and both golden sizes. All 19 sha256 values appear in its table and match the head's blobs.
- **The totals are correct:** 9 added and 10 modified; +2,062 / −1,903; 1,281,065 B at the head.
- **The commits are as stated:** `055ee0c0bc` and `2033260c57` on the T6 branch, then `fdcdb5e024` (REPAIR_01).
- **The NUM merge** `8eaa4403a6` is the `--no-ff` merge of `fdcdb5e024` (parents `7e953fd837`, `fdcdb5e024`), and it is in NUM. NUM `2b1f244faf` adds no maintained path to it.
- **Main's first-parent list** is right.
- **The citation pin** `2b1f244faf` is reachable from the pushed NUM (`53f626a5c8`).
- **The two cited records** (`R/I76/t6s_01/inputs` and `REVIEW_RV101/t6s_01/evidence/oracle/exp_differences.txt`) exist on main.

**CHANGE_RECORD.md, checked against my review and addenda:** §4 and §5's tables state my findings correctly:
- PASS with 0 / 1 / 10;
- closure;
- 63 byte-identical statements;
- 891 identical step outcomes over 69 fixtures;
- 651 dispatcher refusals;
- 10 of 13 own mutant edits killed, with the 3 survivors named correctly;
- SF-1 and its reach bound of 2^-25;
- ADDENDUM_01's 185,401 words, 21,997 / 10,788 ties and 90 guard cases;
- 430 / 430 and `tsc` clean at `fdcdb5e024`;
- E-1;
- the suite figures (3,552 → 3,590, 172 → 176, Python +23, RV101's 1,315 → 1,338).

§7's routed notes match my routing: NT-1 to PR-B1, NT-9 to S-I2, NT-7, NT-10 and I75 (d) to T6's later slot, and NT-3 for GEN-8.

**The two rulings as written into the gate rows** match RR "I80's package accepted; …" in substance:
- **Gate item 3:** satisfied by the exact-head DEC-025's 40 manifests against a fresh main baseline, in fresh targets, as for S-I1, with no separate D1 freeze;
- **Pass B:** not applicable (ruled), on §5.1, with RV101 re-reading the scope at the head (§3 below).

**PR_BODY.md** makes the same claims, and they check: 19 files, 9 / 10, +2,062 / −1,903; byte-identical to `fdcdb5e024` and NUM; the 4 records; main's tools; PASS 0 / 1 / 10; SF-1 confirmed on 185,401 words; the acceptance figures; and what is still open. It ends with the attribution line.

## 3. Pass B's premise at the PR head (`passb_premise_scan.py`, `.json`)

My own read-only scan of `d8c88774d0..d953e121`, over Pass B's `crate_dirs.txt` (the 15 D1 crate `src` directories):
- **No D1 crate `src` path changes.** Under the 15 crate roots, only RE's two `tests/` files change.
- **No manifest, lock or build script changes:** no `Cargo.toml`, `Cargo.lock`, `build.rs`, `package*.json`, `pyproject` or toolchain file.
- **No embedded static changes.** The head has 142 `include_str!`, `include_bytes!` and `include!` literals in the D1 sources and PP's `build.rs`, resolving to 71 targets. None of them is a changed path, and every one has the same blob on main and at the head.
  - On the slice's own range my scan finds 138 / 69, against I80's 137 / 68. The extra target is one path my regex mis-resolves (`src/src/build_identity.rs`) for the same file.
  - At the head, the further increase comes from main's U8 test modules.
- **No reviewed input changes:** none of PP's 14 `REVIEWED_INPUTS` is touched.
- **No reader or carrier source changes.** RE's `retained_precision.rs`, `derivative.rs`, `semantic_contract.rs` and `source_blocks.rs`, TS `retainedPrecision.ts` and `retainedPrecisionStanding.ts`, `core/analysis_runs/*.py` and `core/handoff/stress_neutral/*.py` are all unchanged.
- **Nothing else outside the slice changes:** no PP, `src-tauri` or `e2e_plan.py` path.
- **The dispatcher** is the only schema changed:
  - `$defs` is value-identical;
  - every other top-level keyword is value-identical, except the description;
  - `oneOf[0..1]` is unchanged;
  - `oneOf[2]` is `{"$ref": "results.v0.3.schema.yaml"}`;
  - the version file's blob is unchanged.

**So CHANGE_RECORD §5.1's premise holds at the PR head,** and Pass B's not-applicable ruling stands on it.

## 4. The tools, reproduced on the head

**I used main's tools** from `T/IMPLEMENTATION/F2A_D1/`. Their blobs are identical at main, at the head and at NUM (`tools_used.txt`).

**`source_equality.py`,** with `--pr d953e121 --int 0416b3b2ce --main d8c88774d0 --package T/IMPLEMENTATION/T6S`:
- **5/5 PASS, |S| = 19,** with B = `d8c88774d0` (NUM has merged main).
- Check 2: 19 identical in blob and mode.
- Check 3: no three-way merge needed.
- Check 4: 4 execution files, all in the package, with no sha256 mismatch.
- **My JSON is byte-equal to ROOT's** `T6S_MERGE/_run_records/se.json`.
- **Rerun with `--int 53f626a5c8`** (current NUM): 5/5 PASS, the same result.

**`check_citations.py`,** with `--base d8c88774d0 --head d953e121 --index` the package's `citations.json`:
- **PASS:** 2 resolved, 0 ambiguous, 0 unresolved, 0 verification failures, 0 unused entries.
- The summary lines are identical to ROOT's `T6S_MERGE/_run_records/citations.txt`.

## 5. Notes

| # | Sev | Where | Note |
|---|---|---|---|
| A2-N1 | NOTE | `CHANGE_RECORD.md` §7 "For ROOT's ruling", items 1–2 | **Stale wording.** These still read as open questions ("The records do not say which"; "recommended not applicable"), while §5's gate rows (ROOT's edits) record both rulings. Nothing is false about the decisions, but a reader of §7 alone would miss them. Suggest the merge record says they were ruled, or a "ruled, see §5" at a later touch. A package edit now would mean resealing and rerunning the head gates, which is not worth a re-cut. |
| A2-N2 | NOTE | `CHANGE_RECORD.md` §0, §5, §7 | **Two cited RR headings are on NUM but not yet in main's RR:** "RV101 confirms SF-1's repair; T6S merged into NUM" and "I80's package accepted; …". They reach main with the next records PR. `check_citations.py` does not scan the package, so this is outside its check. The other cited headings are on main. |
| A2-N3 | NOTE | `CHANGE_RECORD.md` §2, "RV95 N-5" | **A precision point.** "The test sets the 12 receipt fields that `source_blocks::integer` reads": `integer` reads 13; `failure.block_order` is not set. This is my NT-1, already routed to PR-B1 in §7, note 1. Wording only. |
| A2-N4 | NOTE | The combined tree at the head | **Limit.** Main's U8 changed inputs that the slice's tests read: the 07l corpus `retained_precision_cases.json` (read by id: `two_case_synthetic`, `two_case_facade_after_certificate_synthetic`) and two new L = 0 successor fixtures. None of the 19 files changed, but the slice's tests have run only on the pre-U8 tree (mine and the implementers'). The combined tree is exercised only by the exact-head DEC-025 and hosted CI. RR's expected deltas assume no interaction, so a T6S test that newly fails or changes at the head would need a look. |

## 6. Records (`R/REVIEW_RV101/t6s_01/`)

- `ADDENDUM_02.md` (this file) and `ADDENDUM_02.SHA256SUMS`, which covers this file and `addendum_02/`.
- `addendum_02/` holds:
  - `scope.txt` (ancestry, name-status and the blob tables);
  - `passb_premise_scan.py` with its output at the head and on the slice's range;
  - `source_equality.out`, `source_equality.json` and the rerun against current NUM;
  - `check_citations.out` and `citations_resolved.md`;
  - the package's sum check on the PR's blobs;
  - the live PR body's diff against `PR_BODY.md`;
  - `tools_used.txt`.
