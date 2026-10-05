# RV96 addendum 02: confirmation of #1084 at R3

**Reviewer:** RV96, the same reviewer as `REVIEW.md` and `ADDENDUM_01.md` in this folder. TASK (Type 2), dispatched by ROOT; ROOT is the return path.

**Request:** ROOT's relay of 2026-10-05 asked for four confirmations on R3. It points to RR "RV96 addendum 01 at R2: the merge method and one more redaction" (NUM `cc44bce7f3`), which I read in full at R3.

**Placeholders** are as before. **H** = `dfa5e2dc44`. **R2** = `3798d5eca7`. **R3** = `5758e1c3df70a51b80a2a94d0118c5c2fad7456f`. **N3** = NUM `cc44bce7f3210529af5b3ff0e2a6fa906ab8480f`. **M** = main `e916ad1789`. **POL** = `P/validation/portability_policy.json`.

## The candidate

- **Parentage:** R3 → R2 → `59b72619fe` → H → M.
- **Equalities with NUM:** `R3:P/execution` = `N3:P/execution` (tree `c438fd5da41b…`), and `R3:POL` = `N3:POL` = `R2:POL` (blob `9fb269d2a069…`).
- **Against main:** M→R3 is **6,613 paths: 6,609 added, 4 modified, 0 deleted**. The only path outside `P/execution` is POL.
- **Main has not moved.** `gh api …/branches/main` returns `e916ad17892d…`, which is the PR's base. The PR is a draft, MERGEABLE.

## Verdict: **PASS** (the remaining gates are pending; §4)

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 1 |

**B-2 is closed by ROOT's squash ruling,** which I checked against the tree (§3). **S-2 is repaired** (§1, §2). **N-7 is corrected** in RR.

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| N-8 | NOTE | the squash commit's message | GitHub's default squash body concatenates the four branch commit messages. Two of those claims are no longer true: `59b72619fe` says "256 … (211 run records …)", which N-7 corrected to 255 and 210, and H says "Empty non-execution diff", but POL is now in the diff. | Give `gh pr merge --squash` an explicit `--subject`/`--body`, for example the PR title with a body citing R3, NUM `cc44bce7f3`, 255 = 210 + 45 and REDACTIONS.json's 13 entries. Then main's log carries no stale claim. |

## 1. The R2→R3 delta: exactly as stated

`git diff --no-renames --raw R2 R3` lists **14 paths**, all mode 100644, with no deletions:

- **The S-2 redaction:** `R/REVIEW_RV62/selected_material_01/_run_records/whole_diff_whitespace.log` (`2092ba09fc` → `8a3a7b9b0e`).
  - Its blob is the same at H and R2, so it was unchanged until R3.
  - It keeps **165 lines**. **Only lines 2, 4, 7, 9, 45 and 47 differ**, and each now reads `+[REDACTED: whole-host process line quoted from I47's listing; owner decision 2026-10-05; see REDACTIONS.json]`. Every other line is byte-identical.
- **REDACTIONS.json's 13th entry** (`c88c32d067` → `67311deb68`).
  - The top-level keys and **the first 12 entries are unchanged**.
  - Entry 13 names the log: `original_sha256` `103536c8…` equals the sha256 of the **H and R2 blob**; `original_lines` is 165; `redacted_sha256` `e62fae3f…` equals the sha256 of the **R3 blob**; `redacted_line_numbers` is [2, 4, 7, 9, 45, 47]; and a note is included.
- **The handoff folder's reseal:** `IMPLEMENTATION/HANDOFF_2026-10-05/SHA256SUMS`. Only the REDACTIONS.json line changes (`58bf55b8…` → `cb396b72…`), and it verifies **15/15 OK**.
- **The rulings append:** R2's RR (1,029,135 B) is an **exact byte prefix** of R3's (1,031,126 B), and main's 460,817 B remain a prefix. The append is 18 lines: the section "RV96 addendum 01 at R2…", including N-7's correction to 255 (210 + 45). RR now has no machine-path line.
- **My addendum 01 files:** `ADDENDUM_01.md` (`cd067377…`), the folder's `SHA256SUMS` (`7faafe28…`) and the 8 files in `_run_records/addendum_01/`. All 10 are **byte-identical to what I wrote**, and the folder verifies 25/25 OK at R3.

**Credentials.** The four non-RV96 delta files have no credential hits and no machine-path lines.

**Integrity.** All 671 regular SUMS files at R3 give **660 PASS and 11 CHECK, the identical set to R2**:
- the 9 historical snapshots already on main;
- N-4's `.pyc`;
- I40's superseded `final_read_5.log`.

RV62's original hash stays bound only in its `_run_records/INVENTORY.json`, which the manifest supersedes, and in REDACTIONS.json.

## 2. No app or agent process lines in R3's added files: PASS

I searched all **6,609 files R3 adds over main** (`git grep -F` at R3, intersected with the added list) for: `/Applications/`, `Microsoft Teams`, `Parallels`, `ChatGPT`, `Claude.app`, `Claude Helper`, `Messages.app`, `Mail.app`, `Slack`, `zoom.us`, `Spotify`, `Google Chrome`, `Safari.app`, `Codex.app`, `--resume=`, `--session-id`, `--session_id`, `user-data-dir`, `cua-repl`, `claude-code/`, `PasswordBreachAgent`, `/System/Library/` and `/usr/libexec/`.

- `/Applications/` appears in 13 files. 12 are the Xcode toolchain path `…/Xcode.app/Contents/Developer/usr/bin/python3` in command records, and 1 is this folder's `ADDENDUM_01.md` prose.
- Every other hit is prose naming the apps in this RV96 folder (`REVIEW.md`, `ADDENDUM_01.md`, `secret_scan_summary.txt`), or REDACTIONS.json's note on entry 13.
- There are **no process lines**, and `/System/Library/` and `/usr/libexec/` appear in no added file.
- The 13 redacted files have 0 app or agent lines; addendum 01 checked the 12 and §1 the 13th.
- The gzip and tar payloads are unchanged since R2, which addendum 01 found clean.

## 3. B-2's squash is adequate: PASS

- **The squash commit's tree is R3's.** M is R3's merge base and its only non-branch ancestor, and main is unmoved at M. Squashing R3 onto an unmoved main therefore produces a single-parent commit on M whose tree is exactly R3's tree. The ruled `--match-head-commit` and the check that main is unmoved guarantee both conditions.
- **No original blob reaches main.** The original blob IDs of all **13** redacted files at H (the 12 listings and RV62's log, taken from REDACTIONS.json) are **absent from R3's entire tree** (0 of 13 present) and from M's tree (0 of 13). A squash commit therefore references none of them, and none of `dfa5e2dc44`, `59b72619fe` or `3798d5eca7` enters main's ancestry.
- **Limits, already recorded in RR:**
  - The originals stay reachable on origin through the NUM branch, the PR branch, and GitHub's `refs/pull/1084/head`, which keeps the PR's commits after the merge.
  - Only an owner-decided history rewrite or ref deletion would remove them.
  - The squash message is N-8.

## 4. GEN-8 and the gates

**GEN-8 on R3: PASS.**
- On the R3 checkout (`WT/records-pr`, clean at R3, used read-only), with the same command as addendum 01 §3, it ran from 03:15:50Z to 03:16:19Z and gave **`1 passed, 10 deselected`** (28.89 s). The tree stayed clean.
- The classification replay on `git ls-tree R3` gives 6,109 candidates, **0 findings, 0 policy issues**.

**Hosted runs on R3,** as of 2026-10-05T03:19Z:

| Gate | Run | Head | State |
|---|---|---|---|
| governance-harness | 37258661309 (pull_request) | R3 | **success**: "1156 passed, 48 subtests passed", `CHIRALITY_REQUIRE_LIVE_TESTS: 1` |
| Harness Pre-merge Validation | 37258661233 | R3 | **success** |
| pec-tests | 37258661340 | R3 | **success** |
| Piping Desktop E2E (PR) | 37258661311 | R3 | **in progress**: Select source coverage success; Numerical cargo suite running; others skipped by selection |
| Full-SHA dispatch | 37258656686 (`workflow_dispatch`, `codex/piping-t3-records-20261005`, headSha R3) | R3 | **pending** (queued, no jobs yet). Its plan's `head` and `target_base` cannot be read yet, so ROOT's "target_base = main `e916ad1789`" is **not yet confirmed** by me. |
| DEC-025 | ROOT | — | pending (to be relayed) |

R2's runs (dispatch 37257701935 and PR E2E 37257706153) ended **cancelled**; they were superseded. RR records that DEC-025 on R2 was stopped and does not count.

**Pending, for ROOT to relay or for me to confirm:**
- the dispatch's plan (`head` R3, `target_base` M, `mode`, `numerical_required`) and its conclusion;
- the PR E2E run's conclusion;
- DEC-025 on R3.

## Host and method

- **Reads:** the R3 checkout, read-only, with `GIT_OPTIONAL_LOCKS=0`; Git blob and tree reads; and `gh` reads. The one pytest was GEN-8.
- **Not run:** cargo, native jobs, Git writes, installs and archive copies.
- **The memory guard** was PID 5387, running.
- **Scratch:** `WT/scratch/rv96_records_01/r3/`. `TMPDIR` pointed there.

## What ROOT must rule on

Nothing new. N-8 asks ROOT to write the squash commit's subject and body explicitly.

## Evidence (`_run_records/addendum_02/`)

- `gen8_pytest_r3.log` and `gen8_replay_r3_summary.txt`: GEN-8 on R3.
- `rv62_redaction_check.txt`: the S-2 redaction.
- `sums_verify_r3_checks.tsv`: the SUMS exceptions at R3.
- `ci_runs_r3.json`, `ci_governance_harness_r3_excerpt.txt` and `dispatch_37258656686_at_review.json`: the CI evidence.
