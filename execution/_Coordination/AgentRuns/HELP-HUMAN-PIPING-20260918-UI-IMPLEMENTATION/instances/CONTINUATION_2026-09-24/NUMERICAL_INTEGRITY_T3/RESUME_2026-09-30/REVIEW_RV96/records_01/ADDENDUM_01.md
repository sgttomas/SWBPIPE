# RV96 addendum 01: re-review of #1084 at the re-cut head R2

**Reviewer:** RV96, the same reviewer as `REVIEW.md` in this folder. TASK (Type 2), dispatched by ROOT; ROOT is the return path. I wrote none of the records, the policy entries or the redactions.

**Request:** ROOT's relay of 2026-10-05. It asked for six confirmations on R2, and pointed to the RR sections "Records PR #1084: GEN-8 flags historical records; the owner approves their portability registration" and "Records PR #1084: redactions, GEN-8 repair and re-cut" (NUM `eb0d6d7ae4`). Both sections were read in full at R2.

**Placeholders** are as in `REVIEW.md`. **H** = `dfa5e2dc44` (the first head). **R2** = `3798d5eca7be2fa87df08d753bf926c95a2a6916`. **N2** = NUM `eb0d6d7ae40849db1bf0baadd26514d6390cdd0f`. **M** = main `e916ad1789`. **POL** = `P/validation/portability_policy.json`.

## The candidate

- **Parentage:** R2 → `59b72619fe` → H → M. The PR branch's three commits are `dfa5e2dc44`, `59b72619fe` and `3798d5eca7`. `gh pr view` shows head R2, draft, MERGEABLE, mergeStateStatus UNSTABLE, because a run was still in progress.
- **Equalities:** `R2:P/execution` = `N2:P/execution` (tree `68c0987acf86…`), and `R2:POL` = `N2:POL` (blob `9fb269d2a069…`).

## Verdict: **FAIL** (B-2 needs ROOT's ruling on the merge method; S-2 is a residual of S-1)

| Severity | Count |
|---|---|
| BLOCKING | 1 |
| SHOULD-FIX | 1 |
| NOTE | 1 |

**B-1 and S-1 as asked are confirmed repaired.** GEN-8 passes on R2, locally and hosted. All 255 policy entries are correct and hash-bound. Each of the 12 listings is redacted, and its manifest is exact. The remaining problems are two:
- **How the redaction reaches main (B-2).** The unredacted listings are in R2's own ancestry.
- **One file outside the 12 still carries part of their content (S-2).**

## Findings (this addendum)

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| B-2 | BLOCKING (until ROOT rules) | the PR branch's history | R2's parent chain contains H, whose tree holds the 12 **unredacted** listings (the original blobs, sha256 equal to REDACTIONS.json's `original_sha256`). A merge commit, which is how #1068 merged (`gh pr merge --merge`), would make H, and with it the originals, part of **main's** history. Main's history is permanent unless main itself is rewritten. The owner decided "Redact before merging". The ruling notes that the originals stay in the integration branch's history, but not that a merge commit would carry them into main. The repository allows squash merges (`allow_squash_merge: true`). | ROOT rules one of: (i) **squash-merge** #1084, which gives main a single-parent commit with R2's tree; (ii) **re-cut** the branch as one commit on M with R2's tree, then rerun hosted CI and the dispatch on it, with DEC-025 transferring by tree identity if ROOT so rules; or (iii) a recorded owner or ROOT acceptance that main's history may carry the originals. |
| S-2 | SHOULD-FIX | `R/REVIEW_RV62/selected_material_01/_run_records/whole_diff_whitespace.log` (165 lines; bound in that folder's `_run_records/INVENTORY.json`) | This is a whitespace-check log. It quotes 6 lines verbatim from I47's **original** `FINAL_PROCESS.log` and `PROCESS_DURING_FINAL_OPTIMIZED.log`: 3 Microsoft Teams ModuleHost lines with full ARGS, including a Teams `--session_id=…` value, and 3 Parallels Desktop service lines. It is the only file at R2 outside the 12 with application process lines. The other 12 new files under T3 with `/Applications/` reference only the Xcode toolchain's `python3`. Hosted CI tarballs and the 35 gzip payloads have none. | Redact those 6 lines, or the log's process content, in the same way as the 12. Add the file to REDACTIONS.json with its original hash, then re-cut. Alternatively ROOT/owner accepts it explicitly. |
| N-7 | NOTE | RR "Records PR #1084: redactions, GEN-8 repair and re-cut" (and ROOT's relay) | Both say **256** entries (211 overrides and 45 exceptions). POL at R2 has **255** new entries: **210** `historical_role_override` and **45** `control_path_exception`. NUM's history explains the gap: `2c312d909a` registered 211 + 43 = 254, including RR, and `43f3d4c902` removed RR's entry and added 2 CONTROL entries, giving 255. The removal was not subtracted from the total. | Correct the count at the next RR append, with a label. |

**Observations, no finding:**
- I40's redacted listing keeps PID 1 (`/sbin/launchd`), as its first line, alongside the cargo and rustc lines. That is harmless.
- The cargo and rustc lines keep the host's toolchain home path, which is the N-1 class.
- Four of the 210 overrides are ROOT-authored historical state records rather than TASK or review records, although the reason text says "TASK or review run record": `R/HANDOFF_2026-10-03/{FINAL_CHECK,READER_STATE}.json` and `R/RESUME_2026-10-03_1904/{STATE,INTERRUPTED_RECORDS}.json`. They are dated, not living, so the role is right.

## 1. Scope: PASS

- **M→R2:** `git diff --no-renames --name-status M R2` gives **6,604 paths: 6,600 A, 4 M, 0 D**.
  - The only path outside `P/execution/` is POL.
  - The modified paths are the three from `REVIEW.md` §1 (ROOT_CURRENT, RR, WORK_GRAPH) and POL.
- **POL is append-only against M.**
  - Textually, `git diff --numstat` gives **+2,040 / −0**.
  - Semantically, the keys and the `schema_version` and `project_root` values are unchanged. M's 530 `historical_role_overrides` and 47 `control_path_exceptions` are exact, order-preserving prefixes of R2's 740 and 92.
  - There are **255 appended entries**. None duplicates another, and none overlaps an existing entry.
- **POL's NUM commits:** `2c312d909a` (registration) and `43f3d4c902` (RR's entry removed; RV67's brief and I42's PLAN.md added).
- **H→R2:** 24 A, 6 D, 18 M; all 42 written blobs are mode 100644.
  - **Added:** this RV96 folder (17 files, blob-identical to what I wrote: REVIEW.md `f19e30b6…`, SHA256SUMS `608833a3…`, 16/16 OK); REDACTIONS.json; and the 6 preserved files, moved under `IMPLEMENTATION/HANDOFF_2026-10-05/_run_records/preserved_untracked/`.
  - **Deleted:** those same 6 at their old paths, with identical blobs. They were H-only files, so no record of main's is deleted.
  - **Modified:** the 12 redacted listings; RR (§5); `HANDOFF_2026-10-05_PROMPT.md` and `HANDOFF_2026-10-05_TO_NEXT_ROOT.md` (host paths become `<repo>/…`, word diff only); `IMPLEMENTATION/HANDOFF_2026-10-05/CLEANUP.md` (the preserved path); that folder's SHA256SUMS; and POL.
- **NUM `29160bbc1c..eb0d6d7ae4`:** 5 commits (`8047bfe83f`, `2c312d909a`, `43f3d4c902`, `a7f93756e9`, `eb0d6d7ae4`). They change nothing outside `P/execution` and POL.
- **The intermediate commit:** `59b72619fe` carried both copies of the preserved files, and `3798d5eca7` drops the pre-move ones (6 files, −352 lines). This matters only through B-2.

## 2. The policy entries: PASS (N-7 on the count)

Each of the 255 entries was checked against R2 (`_run_records/addendum_01/policy_new_entries.tsv`):
- **Fields and hashes.** Every entry has exactly the six required fields, and every `sha256` equals the sha256 of **R2's blob**. **255/255.**
- **Roles by structural class** (`surface_roles.classify_surface`):
  - All **210 overrides** have role EVIDENCE and target active, structurally UNCLASSIFIED "unknown managed AgentRuns artifacts". They are run records: `EXECUTION.json`, `MANIFEST.json`, `ORIGINS.json`, `SEAL.json`, `CHECKS.json`, review addenda and the like, in I50–I60, RV42–RV95, `verification/`, and the 2026-10-03 handoff and resume records.
  - All **45 exceptions** have role CONTROL. 43 are as-issued documents of UNCLASSIFIED class: 42 dispatch briefs in `R/BRIEFS/` and `T3/HANDOFF_2026-10-03_TO_NEXT_ROOT.md`. Two are CONTROL by name token: `R/BRIEFS/RV67_I51_C2_AMENDMENT_REVIEW.md` and `R/I42/source_bridge_rv56_repair_02/PLAN.md`. None reclassifies structural EVIDENCE.
  - All 255 still contain a machine path, so none is stale.
- **No living document is targeted.** None of the entries is RR, ROOT_CURRENT, WORK_GRAPH, `HANDOFF_2026-10-05_*` or anything under `IMPLEMENTATION/HANDOFF_2026-10-05/`. None is a handoff-prepared brief (`I61_U8_PLAN`, `I68_`, `I69_I70_I71_`, `I72_`, `I73_`, `RV96`–`RV99`, `U8_COMMON`), and none is `ROOT_SELECTION*`, `OPERATING_NOTES*`, `OWNER_DIRECTION` or a DISPATCH file.
- **Authority.** All 255 cite the owner's in-session approval and the RR ruling. The reasons fall in four classes: run record 210, as-issued brief 43, as-issued 2026-10-03 handoff 1, as-issued I42 plan 1.
- **Reconciliation with B-1.** H's 263 GEN-8 findings = **255 registered** + **8 repaired**. The 8 are RR:10932 (placeholder), the two 2026-10-05 handoff documents (`<repo>/…`) and the 5 preserved files, now under a structural `_run_records/` directory. Nothing was registered that GEN-8 had not flagged, and nothing flagged was left out.

## 3. GEN-8 on R2: **PASS**

- **On the R2 checkout,** `WT/records-pr`, clean at R2, used read-only:

  ```
  TMPDIR=WT/scratch/… PYTHONDONTWRITEBYTECODE=1 GIT_OPTIONAL_LOCKS=0 \
    VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8
  ```

  It ran from 03:02:46Z to 03:03:17Z and gave **`1 passed, 10 deselected`** (30.50 s), with the tree left clean. The log is `_run_records/addendum_01/gen8_pytest_r2.log`. A real checkout gives GEN-8 the true tracked set, so N-6 does not arise here.
- **Replay of the classification on `git ls-tree R2`,** with R2's policy loaded: 6,108 candidates, **0 findings, 0 policy issues**.
- **Hosted:** `governance-harness` run 37257706157 (pull_request, R2, with `CHIRALITY_REQUIRE_LIVE_TESTS: 1`) gives **success, "1156 passed, 48 subtests passed"**, compared with 1 failed and 1155 passed at H.

## 4. The redactions: PASS for the 12 (S-2 outside them)

Checked against `IMPLEMENTATION/HANDOFF_2026-10-05/_run_records/REDACTIONS.json` (`_run_records/addendum_01/redactions_check.tsv`):
- **Original hashes and line counts.** For **12/12**, `original_sha256` equals the sha256 of the blob **at `dfa5e2dc44`**, and `original_lines` equals that blob's line count.
- **Redacted hashes.** For 12/12, `redacted_sha256` equals the sha256 of R2's blob.
- **Content.** Every redacted file is the original's header and kept process lines, verbatim, plus one top note giving the original's line count and sha256. No other line was altered.
- **Nothing remains to identify apps or agents.** **0 app or agent lines remain** in any of the 12: no `/Applications/`, Teams, Messages, Mail, ChatGPT, Claude, Codex, Xcode, Parallels, `--resume` or session identifiers.
  - The kept lines are the four RV56 `python3 -` lines, the two cargo/rustc lines in I40 with I40's `launchd` line, and the headers.
  - The four RV56 lines' parent PIDs are bare numbers and identify nothing.
- **Seals.** Each original's sealed binding still carries the original hash, and the manifest supersedes it. The bindings are I40's `SHA256SUMS` and `WRITE_INVENTORY.json`, and the `INVENTORY.json` of I42 ×2, I47 ×2 and RV56 ×2. The only SHA256SUMS that now fails is I40's, on `final_read_5.log` (§5).
- **Residuals elsewhere: S-2.** A search of R2's T3 tree outside the 12 found:
  - `Microsoft Teams` in 1 file (S-2) and `ChatGPT.app` in 1 file, which is also on main (a `PATH` string in I28's `BUILD_INPUTS.json`);
  - no `Claude.app`, `Messages.app`, `--resume=` or `user-data-dir`;
  - `--session-id` only in this folder's `REVIEW.md` prose.

## 5. Integrity of the delta: PASS

- **Credentials** (a rerun of `REVIEW.md` §2 over the 42 written blobs): the only hits are in this RV96 folder's own scanner and its description. There are no credentials, no file over 50 MB (the largest is RR at 1,029,135 B), no binaries, and no new symlinks or executable modes.
- **All SHA256SUMS at R2.** There are 671 regular SUMS files: H's 670 plus this folder's. The symlinked `fixture/tests/retained_k4/SHA256SUMS` in RV58 is dangling, per N-2, and not counted. **660 verify completely.** The 11 exceptions are:
  - **the same 10 as at H:** the 9 historical snapshots on main and N-4's `.pyc`;
  - **plus I40's** `R/I40/kernel_origins_01/SHA256SUMS`: 33 OK, 1 BAD on `_run_records/final_read_5.log`. That is the expected supersession by REDACTIONS.json, whose `original_sha256` is the hash this SUMS seals.
- **Folders checked:** `IMPLEMENTATION/HANDOFF_2026-10-05/SHA256SUMS` 15/15 OK (resealed with REDACTIONS.json and the moved paths); `…/_run_records/preserved_untracked/SHA256SUMS` 5/5 OK after the move; this folder's SHA256SUMS 16/16 OK as committed.
- **RR, append-only against main: PASS.** Main's 460,817 B are an exact byte prefix of R2's 1,029,135 B.
  - From H to R2, exactly **one line changes in place**: RR:10932. That is past main's prefix, and it is E-2's labelled placeholder ("`<system-temp>/x` [path placeholder-repaired: see erratum E-2]").
  - The rest of the change is **54 appended lines** (the two new sections). No other H line changed.

## 6. Gates on R2 (at 2026-10-05T03:11Z)

| Gate | Run | Exact head? | State |
|---|---|---|---|
| governance-harness | 37257706157 (pull_request) | R2 | **success** (1156 passed) |
| Harness Pre-merge Validation | 37257706278 | R2 | **success** |
| pec-tests | 37257706154 | R2 | **success** |
| Piping Desktop E2E (PR) | 37257706153 | R2 | **in progress**: source selection success; the Numerical cargo suite was running |
| Full-SHA dispatch | 37257701935 (`workflow_dispatch`, branch `codex/piping-t3-records-20261005`) | R2 | **pending** (queued; no jobs yet). Its `target_base` and plan have not been read yet. |
| DEC-025 | ROOT | — | pending (to be relayed) |
| GEN-8 | §3 | R2 | **PASS** |

For the record, H's dispatch 37255801968 later **completed with success**. It is superseded by R2.

**Pending, for ROOT to relay:** the dispatch's plan (`head` = R2, `target_base` = M) and its conclusion, the PR E2E run's conclusion, and DEC-025. These must be redone on any new head that B-2's option (ii) or S-2's repair produces.

## Host and method

- **Reads:** the R2 checkout, read-only, with `GIT_OPTIONAL_LOCKS=0`; `git show` and `ls-tree` blob reads; and `gh` reads. The one pytest was GEN-8.
- **Not run:** cargo, native jobs, Git writes and installs. I did not make the archive copy, because the real checkout made it unnecessary. The tarball was extracted to scratch only to search it, then deleted.
- **The memory guard** was PID 5387, running.
- **Scratch:** `WT/scratch/rv96_records_01/r2/`. `TMPDIR` pointed there.

## What ROOT must rule on

1. **B-2:** the merge method (squash, or a single-commit re-cut on M), or an explicit acceptance that main's history may carry H's unredacted listings.
2. **S-2:** redact the 6 quoted process lines in RV62's `whole_diff_whitespace.log` (and add it to REDACTIONS.json), or accept them.
3. **N-7:** correct the 256/211 count in RR at the next append.

## Evidence (`_run_records/addendum_01/`)

- `gen8_pytest_r2.log` and `gen8_replay_r2_summary.txt`: GEN-8 on R2.
- `policy_new_entries.tsv`: the 255 entries, each with its structural class and R2 blob hash prefix.
- `redactions_check.tsv`: the 12 redactions.
- `sums_verify_r2_checks.tsv`: the SUMS exceptions at R2.
- `ci_runs_r2.json`, `ci_governance_harness_r2_excerpt.txt` and `dispatch_37257701935_at_review.json`: the CI evidence.
