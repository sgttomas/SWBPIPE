# RV100: independent review of the follow-up T3 records-only PR (#1088)

**Reviewer:** RV100, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of these records.

**Brief:** `R/BRIEFS/RV100_RECORDS_PR2_REVIEW.md`, read in full at NUM `656d0e274d`, with the repository root AGENTS.md. Precedent read first: RV96's `REVIEW.md`, `ADDENDUM_01.md` and `ADDENDUM_02.md` (B-1, S-1, S-2, B-2, N-6, N-8), and `T3/IMPLEMENTATION/RECORDS_MERGE_2026-10-05/RECORD.md`.

**Placeholders.** WT = the t3 workspace; NUM = WT/numerics, the T3 integration worktree; P = `projects/chirality-piping`; T3 = P/execution/…/NUMERICAL_INTEGRITY_T3; R = T3/RESUME_2026-09-30; RR = T3/ROOT_RULINGS_V1.md; VENV = the repository venv's Python 3.13.14.

## The candidate

| Item | Value |
|---|---|
| PR | #1088, draft, `codex/piping-t3-records-2-20261005` → `main`, MERGEABLE (BLOCKED while checks run) |
| H (head) | `e2b83da5841e749e90ba54af6d4a45971693bc4b`, one commit, sole parent M |
| M (base, main) | `f506f3e2dedf136361fbfac285374201b490e023` (#1084's squash); `gh api …/branches/main` returns M at review time |
| N (source, NUM) | `656d0e274d8855f037f5b843c580c653377297d8`, also the tip of NUM on origin |
| Author and committer | the owner's configured Git identity; the message's only other identity is the agent co-author trailer |

## Verdict: **PASS** (no blocking finding; only DEC-025 is pending, §5)

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 7 |

The following pass:
- **Scope.** H's whole tree equals N's.
- **Publication screen.** No credentials, whole-host data, binaries or large files.
- **Portability.** GEN-8 passes, and the changed files contain 0 machine-absolute paths.
- **Integrity.** All three SUMS folders verify. RR is append-only. None of the 13 originals is in H's tree.
- **The records and Git.** The records' facts agree with Git and the hosted runs.

S-1 is a rule change in the living handoff text that no ruling records. ROOT should rule on it, but it does not block a records PR.

## Findings

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | `T3/HANDOFF_2026-10-05_TO_NEXT_ROOT.md` ("Rules that continue" → Git → how to merge) and `T3/HANDOFF_2026-10-05_PROMPT.md` loop step 5 (both from NUM `656d0e274d`) | Both now allow a merge "after checking `origin/main` has not moved, **or that any move is disjoint from the PR's paths and from piping**", for product and records PRs alike. No ruling adopts that standing rule. RR's follow-up-PR section lists its "Handoff corrections made with it", and this is not among them. The only basis is #1084's one-off carry-over, which RR ruled "on that basis": GEN-8 rerun on the local combination, plus the owner's "merge what's ready". The new rule drops both conditions. It also contradicts the same handoff's step 3.8 ("Merge with `--match-head-commit` after checking main has not moved") and its existing rule "A carry-over by ruling needs a stated premise and the reviewer's confirmation". "Disjoint from piping" is undefined for main moves in `tools/`, `.github/workflows/` or the portability policy, which drive piping's gates (GEN-8 lives in `tools/practitioner_harness`). | ROOT rules one of: (a) append a ruling that adopts the exception with explicit conditions (a defined disjoint set that excludes gate tooling and CI, GEN-8 rerun on the combination, and main's head and changed paths recorded in the merge record), and make step 3.8 consistent; or (b) restore "only after confirming main has not moved", and take any exception to the owner. Fix it on NUM, then either re-cut this PR or carry the fix in the next records PR. |
| N-1 | NOTE | the handoff's "how to merge", prompt step 8, and RR "A follow-up records-only PR before the handoff" | The records-PR merge rule reads `--squash` "with an explicit subject and body", "after checking main". It **omits `--match-head-commit`**, which #1084 used (`MERGE_COMMAND.txt`). RV96 ADDENDUM_02 §3 rested B-2's closure on it ("the ruled `--match-head-commit` and the check that main is unmoved"). | Write `gh pr merge --squash --match-head-commit <reviewed head>` into the rule, and use it for #1088. |
| N-2 | NOTE | RR's last section (line 12115) and the handoff "What is done" | RR says the PR "carries NUM's 39 execution files added since #1084, plus this ruling and the updated handoff". The PR has **31 added + 10 modified = 41**. The 39 are the 30 A + 9 M at `99472d3630`, which already include RR and the handoff. The other two are the RV100 brief and ROOT_CURRENT. The handoff says #1084 "passed RV96's three passes", but RV96 gave FAIL, FAIL, then PASS. `RECORDS_MERGE_2026-10-05/RECORD.md` states that correctly. | Labelled corrections at the next RR append and handoff revision. |
| N-3 | NOTE | the handoff ("NUM has absorbed main through the follow-up records PR's merge", "The follow-up, reviewed by RV100"), ROOT_CURRENT and the work-graph T3 row ("on main: #1084 … and a follow-up records-only PR") | These are **forward-dated claims**. At H they are not yet true: the PR is unmerged, NUM has absorbed main only through M, and this review is in progress. They become true on main only if #1088 merges. On NUM they stay false until then. The handoff does tell the next ROOT to check `RECORDS_MERGE_2026-10-05B/` first. | The 05B record states the merge, NUM's absorption of main, and RV100's verdict. If #1088 does not merge, add labelled corrections on NUM. |
| N-4 | NOTE | GEN-8 method: RR:12050 (already on main) and the GEN-8 evidence files | **N-6's adopted method does not work.** With `GIT_CEILING_DIRECTORIES=WT` on the `git archive` copy, the live self-check fails at setup: `HarnessOperationalError`, `git ls-files --error-unmatch … -> exit 128`, raised from `brief_adoption._check_committedness` via `_add_bridge_receipt_findings` (`_run_records/gen8_pytest_attempt1_ceiling.log`). Separately, the committed `_run_records/gen8_combined.txt` and ROOT's `WT/scratch/records_pr2/gen8_H.txt` each hold only the summary line, with no head, tree or method. #1084's `gen8_R3.txt` records its head. | Correct RR:12050 at the next append: run GEN-8 on a clean checkout of the candidate, or on the archive copy without the ceiling plus a `git ls-tree` classification replay (as here). Record the head SHA and method with every GEN-8 result. |
| N-5 | NOTE | `IMPLEMENTATION/HANDOFF_2026-10-05/CLEANUP.md` addendum, `cleanup/strays_moved_20261005.sha256`, RR "Stray scratch gathered" | Of the 1,347 gathered files, **1,193 were removed about a minute later by the periodic cleanup**, as the two RV84 tree copies `srcb1f`/`src5ae`. They were moved at 13:52:25Z (by directory ctime) and removed at 13:53:39Z (manifest c). `apply`'s 2-hour recency guard reads mtimes, which a move preserves. That is within the "regenerable" class: both source commits, `b1f80234dc` and `5ae5fe4f0f`, are on origin's NUM. The addendum mentions "3 tree copies" but not that two of them were the just-gathered files. The `.sha256` list is relative to the stray root, so its `i54_direct_container_profile/` lines resolve to the older same-named folder unless mapped. Re-verification today gives 154 OK, 0 bad and 1,193 missing once mapped (`_run_records/strays_reverify.txt`). There is no `gather_*.jsonl`. The move precedes the tool's last modification (13:53:01Z), so it was evidently not made with `gather --apply`. The tool's own `gather` hashes skip symlinked files and read each file whole into memory. | Optional: one erratum line in the next CLEANUP or RR update. Consider streaming hashes in `gather` before using it on large strays. |
| N-6 | NOTE | DEC-025 tooling (`WT/scratch/u9_dec025/`; committed copy `host_tools/run_all.sh.txt`) | **The first DEC-025 attempt on H ran no step.** Its output folder did not exist, every redirect in the driver failed at 14:11:59Z, the driver still exited 0, and the wrapper logged `dec025-done` in the same second (`H1088_attempt1_no_outdir/wrapper_output.txt`). ROOT has noticed it, added `mkdir -p` to the wrapper (the driver is unchanged, sha `9e34865b…`), and restarted. The committed wrapper copy has the same gap. | Record attempt 1 as void in the 05B record. Treat only `ALL-DONE` in `meta.txt` as completion. Update the committed wrapper copy at the next handoff revision. |
| N-7 | NOTE | #1084's carry-over, `RECORDS_MERGE_2026-10-05/RECORD.md` | The handoff's rule wants a reviewer's confirmation for a carry-over by ruling, and #1084 merged after main moved without one. **RV100 confirms it after the fact.** <br>- `e916ad1789..09574ed9a5` touches only `projects/chirality-app-v4` (510 paths, 0 elsewhere). <br>- M's tree is exactly `09574ed9a5` plus R3's execution tree and policy. <br>- M's push `governance-harness` run 37317887110 succeeded. <br>- The GEN-8 classification replay on M gives 0 findings. | None. |

## 1. Scope: PASS

- **Parentage.** `rev-list --parents H` = H M, and `rev-list --count M..H` = 1. The only ancestor outside the PR is main. None of #1084's heads (`dfa5e2dc44`, `59b72619fe`, `3798d5eca7`, `5758e1c3df`), NUM's `29160bbc1c` or N is an ancestor of H or of M.
- **Changed paths.** `git diff --no-renames --name-status M H` gives **41 paths: 31 A, 10 M, 0 D**, all mode 100644 and all under `P/execution/`. The non-execution diff is empty. The list is in `_run_records/changed_paths_name_status.tsv`.
- **Equality with NUM.** `H:P/execution` = `N:P/execution` = tree `df9fd23b4c7f…`. Beyond the brief, **H's whole tree equals N's** (`abd5be28c5c5…`), because NUM's non-execution tree already equals main's.
- **How the delta arose.**
  - M's execution tree is `c438fd5da41b…`, identical to NUM `cc44bce7f3` (#1084's source).
  - The delta comes from NUM `7df76dfff8`, `185bff70b7`, `99472d3630` and `656d0e274d`.
  - The main merge `6001d2a91d` changed no execution path.
  - Main's blob of every modified file equals its blob at `cc44bce7f3`, so no change of main's is reverted.

**The modified paths,** with the NUM first-parent commits that changed each since `cc44bce7f3` (`_run_records/modified_paths_commits.tsv`):

| Path | Main → H blob | NUM commits |
|---|---|---|
| `T3/HANDOFF_2026-10-05_PROMPT.md` | `da505f88f1` → `810a2057aa` | `99472d3630`, `656d0e274d` |
| `T3/HANDOFF_2026-10-05_TO_NEXT_ROOT.md` | `aa415651f6` → `37c440d2b9` | `99472d3630`, `656d0e274d` |
| `T3/IMPLEMENTATION/HANDOFF_2026-10-05/CLEANUP.md` | `c22df8a477` → `ba7af28a47` | `99472d3630` |
| `T3/IMPLEMENTATION/HANDOFF_2026-10-05/SHA256SUMS` | `5d3e2ea05d` → `d97511fee7` | `99472d3630` |
| `T3/IMPLEMENTATION/HANDOFF_2026-10-05/host_tools/t3_cleanup.py.txt` | `daaa6666bc` → `bfe1cfbb04` | `99472d3630` |
| `R/BRIEFS/U8_COMMON.md` | `2cdd67d963` → `c99cf7f5b7` | `99472d3630` |
| `R/REVIEW_RV96/records_01/SHA256SUMS` | `27196f8220` → `e37e696d7b` (+8/−0) | `7df76dfff8` |
| `R/ROOT_CURRENT.md` | `f4e088bdf6` → `029112563b` | `656d0e274d` |
| `RR` | `734e35b193` → `dcdf658fa2` (+62/−0) | `185bff70b7`, `99472d3630`, `656d0e274d` |
| `P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md` | `410e86e395` → `4621bed5ee` (one row) | `185bff70b7`, `656d0e274d` |

**The 31 added paths:**
- `RECORDS_MERGE_2026-10-05/` (20 files, from `185bff70b7`);
- RV96's addendum 02 (8, from `7df76dfff8`);
- `cleanup/manifest_20261005c.jsonl` and `cleanup/strays_moved_20261005.sha256` (from `99472d3630`);
- the RV100 brief (from `656d0e274d`).

## 2. Nothing that must not be published: PASS

- **Credentials.** All 41 files at H were searched for `ghp_`, `gho_`, `ghs_`/`ghu_`/`ghr_`, `github_pat_`, `sk-`, `AKIA`, `BEGIN .*PRIVATE KEY`, `password`, `secret`, `token=`, `Authorization:`, `Bearer`, Slack tokens and credential environment variable names. **There are no credentials.** The only hits are:
  - the brief's own search list;
  - the module name `secret_private_library` in the strays list;
  - RV96's file names `secret_scan*`;
  - `sk-` inside ordinary words in unchanged work-graph text.
- **Personal data.** The only e-mail address is the agent co-author trailer's `noreply` address. The owner's name appears only as the Git author.
- **Whole-host data.** The search covered the 14,233 lines the PR adds. Patterns: `/Applications/`, Teams, Parallels, ChatGPT, `Claude.app`, `--resume`, `--session-id`/`--session_id`, `user-data-dir`, `/System/Library/`, `/usr/libexec/`, `launchd`, `MacBook`, UUIDs, process-table headers and `ps` rows, `/Users/`, `/private/` and `/var/folders`.
  - The **only hits are descriptions of searches**: RV96 ADDENDUM_02 §2's pattern list and its `rv62_redaction_check.txt` summary line.
  - **No process line, application path, session or agent identifier, or host name** is added.
  - The three UUIDs in the work graph are in unchanged rows that main already carries.
- **Size and type.** The largest file is `dec025/desktop_vitest.log` at 1,326,626 B, and **none exceeds 50 MB**. All 41 files are text, JSON or Python text. There are no binaries, archives, build outputs, symlinks or executable modes.

## 3. Portability: PASS (N-4 on method)

- **The GEN-8 run.** The brief's command was run on a `git archive` copy of H in `WT/rv100/`, from the copy's root, with VENV, `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR` in WT/scratch:

  ```
  VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8
  ```

  It ran from 14:13:06Z to 14:13:30Z: **`1 passed, 10 deselected`** in 23.32 s, exit 0 (`_run_records/gen8_pytest.log`).
- **An earlier attempt errored at setup.** At 14:12:35Z I ran the same command with N-6's `GIT_CEILING_DIRECTORIES`. It produced no verdict (N-4). It was the only other pytest invocation, and no DEC-025 step was running then (§5).
- **Replay of the real tracked set.** In the archive copy, Git resolves to the enclosing repository, so the live walk is RV96's N-6 subset. I therefore replayed GEN-8's classification on `git ls-tree` with RV96's `gen8_replay.py` (`_run_records/gen8_replay_summary.txt`):
  - H: 6,113 candidates, **0 findings, 0 policy issues**;
  - M: 6,109 candidates, 0 findings, 0 policy issues.
- **The hosted run agrees:** `governance-harness` 37322503556 on H, with `CHIRALITY_REQUIRE_LIVE_TESTS: 1`, gives "1156 passed, 48 subtests passed".
- **Machine-absolute paths in the 41 changed files** (`_run_records/abs_paths_by_file.tsv`):
  - GEN-8's detector (`surface_roles.iter_machine_path_lines`) finds **0 in every file**.
  - A broader pattern finds 1 line, RR:11998. It is in main's prefix and is the `<user>` placeholder.
  - **In the added lines, 0.** The living documents (RR, ROOT_CURRENT, the work graph, the handoff, its prompt, and the RV100 and U8_COMMON briefs) have none. The DEC-025 logs use `<WORKTREE>`/`<VENV>` (§6).

## 4. The living documents against the records and Git (S-1, N-1 to N-3, N-5)

**Confirmed:**
- **Heads and PRs.**
  - #1084 is merged at `2026-10-05T13:34:37Z` as `f506f3e2de`, with one parent, `09574ed9a5` (`MERGED_PR.json`, `PARENTS.json`, Git).
  - Main's execution tree and portability policy equal R3's: tree `c438fd5d…` and policy blob `9fb269d2…`, which H also carries unchanged.
  - #1084's earlier heads are not ancestors of main.
  - The move `e916ad1789..09574ed9a5` is app-v4 only (N-7).
  - #1082 is unchanged text.
  - The PR body and H's message ("31 added and 10 modified … portability policy unchanged") are exact.
- **The next unused IDs, I75 and RV101,** are consistent in the prompt, handoff, ROOT_CURRENT, the work graph and RR:12126; RR:11964's "RV100" is superseded there.
  - The highest records folder is I67, and the highest prepared briefs are I74 and RV100. No I75+ or RV101+ is used anywhere in T3 or the work graphs.
- **The merge-method rule:** records PRs squash with an explicit message, and product PRs use `--merge --match-head-commit`. This is consistent across the prompt, handoff, ROOT_CURRENT and RR, except for the precondition (S-1) and the missing `--match-head-commit` for squashes (N-1).
- **The never-merge-NUM rule holds in fact.**
  - NUM's history contains `29160bbc1c`, whose tree holds **13/13** original blobs.
  - On origin, `codex/piping-t3-records-20261005` and `refs/pull/1084/head` are R3, whose ancestry includes `dfa5e2dc44`. That matches the handoff's open item.
  - Product PRs are cut from main, and U8's work branch is cut from NUM, but its PR is from main (handoff 3.1/3.7). This is consistent.
- **The `gather` step.**
  - **Bytes:** `WT/tools/t3_cleanup.py` and the committed `host_tools/t3_cleanup.py.txt` are **byte-identical** (sha256 `6e3fdc7f54fd…`).
  - **Behaviour against CLEANUP.md step 0:** these agree on the stray definition, the `_from_<origin>` suffix, the refusal while cargo, rustc, pytest or vitest runs, post-move hash verification, no file deletion, the removal of an emptied stray folder only, and `plan`'s STRAY line.
  - **The cleanup log:** manifest c at H equals the host log, with 17 caches and 3 tree copies, 11.897 GB.
  - **The record of the move:** N-5.
- **The rest of RR's new sections** match the merge record and runs:
  - DEC-025 on R3, 03:18:01–03:50:59Z: 40/40 identical to F′; pytest 3,540 passed and 32 skipped; vitest 3,552; builds exit 0.
  - The full-SHA dispatch 37258656686 succeeded, and the four PR runs succeeded.

## 5. Gate evidence (at 2026-10-05T14:25Z)

| Gate | Evidence | Exact head? | Result |
|---|---|---|---|
| governance-harness | 37322503556, pull_request | H | **success**: 1156 passed, 48 subtests, live tests required |
| Harness Pre-merge Validation | 37322503494, pull_request | H | **success** |
| pec-tests | 37322503366, pull_request | H | **success** |
| Piping Desktop E2E (PR) | 37322503477, pull_request | H | **success**: Select and Desktop E2E (source mode) success; numerical and source shards skipped by selection, since no source changed |
| Full-SHA dispatch | 37322486897, `workflow_dispatch`, branch `codex/piping-t3-records-2-20261005` | H | **success** at 14:25:20Z. The plan reads `mode: full`, `base` = `target_base` = M, `head` = H, `numerical_required: true` (`_run_records/dispatch_37322486897_plan_excerpt.txt`). Select source coverage, the Numerical cargo suite, Source remainder 1–4 and Desktop E2E (source mode) succeeded; Accessibility was skipped by selection (`_run_records/dispatch_37322486897_final.json`). |
| DEC-025 (Mac, ROOT's) | `WT/scratch/u9_dec025/H1088/` | H | **pending.** Attempt 1 was void (N-6). Attempt 2 started at 14:15:11Z with `meta.txt` head = H, after six quiet samples. The sweep exited 1 at 14:16:31Z, as the fail-fast sweep does on this Mac on every earlier head, and the suites were running. I did not run or touch it. |
| GEN-8 | §3 | H | **PASS** (local run, replay, hosted). ROOT's pre-open `gen8_H.txt` says "1 passed, 10 deselected in 29.40s", which is consistent but records no head (N-4). |

**Pending, for ROOT to relay:** DEC-025 on H, with its per-manifest and per-test comparison against F′ and main's Mac baseline. The piping source is unchanged since #1084, so R3's run is the expected reference. At 14:25Z the PR is MERGEABLE, its merge state is CLEAN, and main is still M.

## 6. Integrity: PASS

**The SHA256SUMS of the named folders,** checked with `shasum -a 256 -c` on the archive copy of H. Each covers every file in its folder.

| Folder | SHA256SUMS (sha256 prefix) | Result |
|---|---|---|
| `T3/IMPLEMENTATION/RECORDS_MERGE_2026-10-05/` | `a69497bf467b` | **19/19 OK** |
| `T3/IMPLEMENTATION/HANDOFF_2026-10-05/` | `54a48a726652` | **17/17 OK**. It is resealed in place for CLEANUP.md and `t3_cleanup.py.txt`, and 2 entries were added; main's SUMS blob `5d3e2ea05d` keeps the prior hashes. |
| `R/REVIEW_RV96/records_01/` | `9a22601a44bb` | **33/33 OK**. The SUMS is append-only (+8/−0), and ADDENDUM_02 is `089188ac…`, as RR cites. |

**RR is append-only: PASS.** Main's RR (1,031,126 B, sha256 `4774516c…`) is an **exact byte prefix** of H's (1,035,698 B, `134d1b9f…`). H appends 4,572 B in 62 lines, the three new sections.

**The squash is adequate: PASS** (`_run_records/redactions_originals_check.tsv`).
- For all 13 REDACTIONS.json entries, the blob at `dfa5e2dc44` has sha256 = `original_sha256`, and H's blob has sha256 = `redacted_sha256`.
- **None of the 13 original blob IDs appears anywhere in H's full tree** (0/13), or in M's (0/13).
- No content hash of the 41 changed blobs equals any `original_sha256` (0/41).

**DEC-025 R3 evidence** (`_run_records/dec025_r3_placeholder_check.txt`):
- ORIGINALS.json's four hashes equal the host files in `WT/scratch/u9_dec025/R3/`.
- After the `<WORKTREE>`/`<VENV>` substitution, each committed file is byte-identical to its host original. The edits are placeholders only.

## Host and method

- **My copy:** a `git archive` of H in `WT/rv100/` (2.9 G), deleted at the end. Logs and scripts are in `WT/scratch/rv100_records_01/`, and `TMPDIR` pointed there.
- **Reads:** NUM with `GIT_OPTIONAL_LOCKS=0` (Git reads, `ls-remote`), and `gh` reads (PR, runs, the dispatch's Select job log through the API).
- **Not done:** Git writes, installs, cargo or native work, and touching DEC-025.
- **What I ran:**
  - the GEN-8 pytest (two invocations; the first errored at setup, see §3);
  - the read-only classification replay;
  - hashing and `grep`.
- **The memory guard** was PID 5387, running (elapsed more than 7 days), with no KILLED line in its logs.
- **Disclosure.** DEC-025 attempt 2's quiet wait began at 14:12:30Z, and my second GEN-8 run (14:13:06–14:13:30Z) fell inside it. Its samples stayed `busy=0` and it started at 14:15:11Z, so it ran after my pytest ended. Nothing of mine was written to the system temp directory.

## What ROOT must rule on

1. **S-1:** the standing "disjoint main move" merge exception. Either adopt it by ruling, with conditions, and reconcile handoff step 3.8; or restore the strict rule. Also decide whether the fix re-cuts #1088 or travels in the next records PR.
2. **N-1:** add `--match-head-commit` to the records-PR squash rule, and use it for #1088.
3. **N-4:** correct N-6's adopted GEN-8 method (RR:12050).
4. **Relay still pending:** DEC-025 on H. It is a pending gate, not a finding. Dispatch 37322486897 has completed with success.

## Evidence index (`_run_records/`)

- `gen8_pytest.log` and `gen8_pytest_attempt1_ceiling.log`: the GEN-8 runs.
- `gen8_replay.py` and `gen8_replay_summary.txt`: the replay for H and M (RV96's script, unchanged).
- `changed_paths_name_status.tsv` and `modified_paths_commits.tsv`: scope.
- `abs_scan.py` and `abs_paths_by_file.tsv`: machine-path counts per changed file.
- `redactions_originals_check.tsv`: the 13 originals.
- `ci_runs_H_at_review.jsonl`, `ci_governance_harness_H_excerpt.txt`, `dispatch_37322486897_plan_excerpt.txt`, `dispatch_37322486897_final.json` and `main_push_runs_f506f3e2de.tsv`: the CI evidence.
- `strays_reverify.txt`: N-5.
- `dec025_r3_placeholder_check.txt`: the DEC-025 R3 evidence.
