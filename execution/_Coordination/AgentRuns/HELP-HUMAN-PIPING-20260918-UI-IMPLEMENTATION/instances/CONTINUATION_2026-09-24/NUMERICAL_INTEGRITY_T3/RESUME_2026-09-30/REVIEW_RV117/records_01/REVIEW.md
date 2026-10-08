# RV117: independent review of the T3 records-only PR after #1108 (#1111)

**Reviewer:** RV117, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of these records. 2026-10-07 UTC.

**Brief:** `R/BRIEFS/RV117_RECORDS_PR7_REVIEW.md` (sha256 `01c05fff…`, verified before use), read in full. Its method is `R/BRIEFS/RV103_RECORDS_PR4_REVIEW.md` items 1–6 with the brief's substitutions and its new item-3 check. Precedents read first: RV110's brief, `REVIEW.md` and `ADDENDUM_01.md` (#1108), then RV103's brief.

**Placeholders.** WT = the t3 workspace; NUM = WT/numerics; P = `projects/chirality-piping`; T = P/execution/…/NUMERICAL_INTEGRITY_T3 (written `T3/` in the run records); R = T/RESUME_2026-09-30; RR = T/ROOT_RULINGS_V1.md; WG = P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md; VENV = the piping venv's Python 3.13.14. `RR:n` and `WG:n` are line numbers at the PR head.

**NUM moved after N.** At dispatch NUM was `52c36a03d3`: two commits after N (`083e1a06e9`, SI1c's `--no-ff` merge, and `52c36a03d3`, SI1c's PR package and this brief). Neither is in #1111. Every check below is against N = `0b8299e496` and the PR head H.

## The candidate

| Item | Value |
|---|---|
| PR | #1111, `codex/piping-t3-records-20261007c` → `main`, ready (not draft), MERGEABLE, merge state CLEAN |
| H (head) | `18a20d329fc09335f013131766d7f74dbfa7f675`, one commit, sole parent M; origin's branch = H |
| M (base, main) | `e33f3e2f1b0a0271da3ef4db8521b3981257b5ce` (#1110's merge); origin's main is still M |
| N (source, NUM) | `0b8299e4965fbb8eb2570405c5b846a1c39300b0` |
| Author and committer | the owner's configured Git identity; the only other identity is the agent co-author trailer |

## Verdict: **PASS** (no blocking finding)

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 5 |

The following pass:
- **Scope.** H is M plus one commit, not from NUM's history. **H's whole tree equals N's** (tree `91760a2626…`). 1,045 paths change: **1,043 A, 2 M, 0 D**, all under `P/execution/`. The modified paths are exactly RR and WG. No open product PR's package is in H: #1112 (SI1c, draft) shares 0 paths with it, and `IMPLEMENTATION/SI1C/` is absent, as RR rules.
- **Publication screen.** No credentials, whole-host data, binaries or large files. **No symlink**, and `R/REVIEW_RV58/` is exactly main's. Main's `validate_run_record_leaks.py` passes on M..H (1,044 files; 0 credentials; 0 machine-local symlinks). **All 8 gzipped files decompress, parse, and screen clean** except for the junit host attribute (N-4), which main already carries.
- **Portability.** GEN-8 passes on the exact head. The changed files hold 0 machine-absolute paths; the living documents hold none.
- **Integrity.** All 29 sum files the PR adds verify in the committed tree (1,012 entries, 0 bad, 0 missing), and so do the 6 sum files already on main in the same folders (35 files, 1,370 entries). No sealed folder has an uncovered file except I91's RETURN.md and REPAIR_01.md, which are outside their sums by design and anchored by sha in RR. RR is append-only. **None of E-10's 47 pre-redaction blobs and none of the 13 earlier originals is in H.**
- **Gates.** The four automatic CI runs succeeded on H.
- **The living documents.** #1108's squash, #1109, NUM's absorb of main, the 30 new rulings, every cited return hash (26 of 26), every routing of the reviews' findings, and the next unused IDs agree with the records, Git and GitHub. **ROOT took no owner-held decision, and no owner decision was made or quoted in this span.**

Nothing must be fixed before the squash. The five notes are for the squash body, the next RR append and the next WG touch.

## Findings

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | WG:564, :567, :575, :579, :611–612, :685, :687; the rulings in force (WG:615–659) | **WG's T3 section was last updated at `051a3a385b`, before RR's last three sections** (`769d0d0f46` RV116 confirms B3-D; `00658c76c1` NUM absorbs #1109 and E-11; N itself, RV111 confirms SI1c). So at N it is stale in seven places:<br>- **WG:564:** "B3-D is ruled (RV116), with its revision 01 under RV116's confirmation". RR:14886 records RV116's CONFIRMED and "B3-D is final for J1".<br>- **WG:567:** NUM "carries main `4f37590bfb`". It absorbed main `e33f3e2f1b` at `e34419d94e` (RR:14920).<br>- **WG:575 (T3-SI1c):** "RV111 confirms. Then NUM, and its compact PR". RV111 has confirmed, and SI1c is accepted at `f5665f8862` (RR:14941).<br>- **WG:579 (T3-B2/B3/B4):** "B3-D's revision 01 is with RV116 to confirm".<br>- **WG:611–612:** RV111 and RV116 are listed as running; both had returned, and the idle list lacks them.<br>- **WG:685 and :687** (next safe action 2 and 3) still begin with the two confirmations that N records as done.<br>- **The rulings in force** lack E-11's two host rules (no symlink in a T3 record, checked with `find -type l`; `validate_run_record_leaks.py` before review), "B3-D is final for J1", and SI1c's acceptance with its PR gate set.<br>This is RV110 N-2's and A1-N-2's pattern a third time. | At the next WG touch. Consider making the WG update part of the same commit as each RR section that changes a position, an assignment or a host rule. |
| N-2 | NOTE | `IMPLEMENTATION/RECORDS_MERGE_2026-10-07B/RECORD.md:21`; its `_run_records/CI_RUNS.txt` | **The merge record's CI evidence is for H1, not H2.** RECORD.md's gate table reads "The automatic CI on H2 `ea3b1443ea` · 4/4 succeeded · `_run_records/CI_RUNS.txt`". CI_RUNS.txt lists runs 37632902690, 37632902530, 37632902263 and 37632902364, all on `145443e9e4` (H1). **The fact is true:** GitHub gives four successful runs on H2 (37637875013, 37637874699, 37637874697, 37637874737), as RV110's ADDENDUM_01 also records (`_run_records/addendum_01/ci_runs_H2.txt`, in H). Only the cited evidence is wrong. (`_run_records/pr1108_merge_checks.txt`) | An RR erratum at the next append naming H2's four runs and pointing at RV110's addendum evidence. The sealed folder stays unchanged. |
| N-3 | NOTE | PR #1111 description (`_run_records/pr1111_body.md`) | **The "It adds" list is wrong in two places:**<br>- "**T3-SI1c:** I88's return and repair round". I88's return (52 files, RETURN.md and its SHA256SUMS) reached main with #1108; #1111 adds only the repair round (30 files: REPAIR_01.md, SHA256SUMS.repair_01 and `_run_records/repair_01/`).<br>- "RV109 (ST's repair confirmation and the early read of SP)". ST's repair confirmation (`R/REVIEW_RV109/rvp_round1_01/ADDENDUM_01.md`) also reached main with #1108; #1111 adds only `rvp_r3p_read_01/` (the early read).<br>The "Not in this PR" list names RV113's running reviews of SR-TS and SR-PY but not RV113's confirmation of SR-RS's repair, which is also running (untracked in NUM at review time).<br>The rest is accurate: 1,043/2/0; RV110's ADDENDUM_01 and RECORDS_MERGE_2026-10-07B; I85's SP, I89 (follow-up and E-10), I90 and I91 (each with its repair), I92; RV112, RV113 round 1; RV111's review and addendum; I93–I96 with RV114–RV116; the briefs; E-10 and E-11; RR and WG; the screening, leak check and GEN-8 claims. The commit message is accurate. | Word the explicit squash body to match: "I88's repair round"; "RV109's early read of SP". Optionally add RV113's SR-RS confirmation to "Not in this PR". Keep the agent co-author trailer. |
| N-4 | NOTE | `R/I91/b1_sr_py_01/_run_records/suites/py_base.xml.gz`, `py_head.xml.gz`, `py_head2.xml.gz`; `repair_01/suites/py_r01.xml.gz`, `py_r01b.xml.gz` | **The five pytest junit files carry the host's machine name** in the `testsuite` element's `hostname` attribute (23 characters, ending `.local`; it embeds the owner's first name and the hardware model). I91's sanitizer replaced the files' paths with `WT/…` placeholders but left this attribute.<br>- **No new exposure:** the same name is already on main in 12 text files: 11 T3 records (I57–I60's receipts and returns, RV75–RV77's records), published by #1084 (`f506f3e2de`), and one App runtime record from 2026-09-20. The first name is in the owner's configured Git identity, and the hardware model is in the owner's quoted words in RR. So this is not personal data beyond the brief's allowance, and not a process listing or session id.<br>- **But neither screen sees it:** ROOT's strict screen has no host-name form, and `validate_run_record_leaks.py` skips `.gz` files entirely. The PR body's "the strict screen is clean" is true of the strict pattern only. (`_run_records/gz_check.txt`; the hits file writes the value as `<host>`.) | Optional, for ROOT: add the host's machine name (or `hostname=`) to the commit-time screen, and have junit-writing harnesses drop the attribute. Redacting these five sealed files alone would not reduce exposure while main carries the same name elsewhere; any repository-wide redaction is a separate decision. |
| N-5 | NOTE | RR:14516, RR:14596, RR:14856, RR:14922 | **Four small precision points in the appended text:**<br>- **RR:14516:** "`b1`'s maintained diff from main is now 11 files." Against main (merge-base `2007709549`), I2 `eca6c00a72` changes **12** files; 11 is against I1, as RR:14663 correctly says for `603e238517`. (`_run_records/branch_facts_2.txt`)<br>- **RR:14596:** "Decisions 17–19 stay owner-held." In I93's own numbering, 17–19 are ROOT's (phase 0, reviewers, PR-B2's gates); the owner-held ones are **22–24**, which prepare B0's decisions 17–19. RR:14648 uses 22–24 correctly.<br>- **RR:14856:** "SCHEMA's B3b diff `0d5bb812…`." That is the sha256 of SCHEMA patched with B3b's change only (I96 REVISION_01's table; `b3d_statics_r1.out.json`'s `b3b_only_sha256`). The file `statics/r1/SCHEMA_ENUM.diff` itself is `b1597c7b…`. RV116's addendum uses the same shorthand.<br>- **RR:14922:** "#1109 (`4a58bf2a7d`, records only)". `4a58bf2a7d` is #1109's first commit (the fixture fix, records only). The PR's head is `b928a20f98`, which adds the validator, its test and two manifests outside `execution/`, and it merged as `cccc41a293`. RR's next bullet does describe the validator. | One RR erratum at the next append covering all four. Nothing else depends on them. |

## 1. Scope: PASS

Evidence: `_run_records/scope_checks.txt`, `changed_paths_name_status.tsv`, `pr1111_view.json` (identity e-mails dropped), `pr1111_body.md`, `open_prs.json` and `pr1112_files.txt`.

**Parentage:**
- H's only parent is M, and `rev-list --count M..H` = 1. `ls-remote`: main = M, and the PR branch = H.
- H is not an ancestor of N. **None of NUM's 849 commits in `M..N` is an ancestor of H.**
- M is an ancestor of N: NUM absorbed main as `e34419d94e` (parents `769d0d0f46` and M).

**Equality with NUM:**
- **H's whole tree equals N's** (`91760a2626…`). `P/execution` = `e5ad06e9aa…` on both sides, and `git diff N H` is empty.
- The non-execution diff is empty against main, from both N and H.

**Changed paths** (`git diff --no-renames M H`):
- **1,045 paths: 1,043 A, 2 M, 0 D**, all under `P/execution/`, as the brief expects. GitHub reports 1,045 files, +245,621/−14.
- **Modes:** 1,035 regular and 10 executable (I95's six `b3s_*.sh`; RV111's four harness scripts). **No mode `120000`.**
- Added by folder:

  | Folder (T-relative) | Files |
  |---|---|
  | `IMPLEMENTATION/RECORDS_MERGE_2026-10-07B/` | 6 |
  | `R/BRIEFS/` (B1_SR_PY, B1_SR_TS, B2B3_PLAN, B2KD_KERNEL_DESIGN, B3D_DESIGN, B3S_EXACT_PRICING, RV109_RVP_ROUND2, RV112–RV116) | 12 |
  | `R/I85/b1_sp_01/` | 132 |
  | `R/I88/si1c_01/` (repair round only) | 30 |
  | `R/I89/b1_sa_01/` | 99 |
  | `R/I90/b1_sr_rs_01/` | 89 |
  | `R/I91/b1_sr_py_01/` | 89 |
  | `R/I92/b1_sr_ts_01/` | 30 |
  | `R/I93/b2b3_plan_01/` | 12 |
  | `R/I94/b2_kd_01/` | 5 |
  | `R/I95/b3_s_01/` | 45 |
  | `R/I96/b3_d_01/` | 21 |
  | `R/REVIEW_RV109/rvp_r3p_read_01/` | 77 |
  | `R/REVIEW_RV110/records_01/` (ADDENDUM_01 and its records) | 21 |
  | `R/REVIEW_RV111/si1c_01/` | 137 |
  | `R/REVIEW_RV112/rvq_round1_01/` | 171 |
  | `R/REVIEW_RV113/rvr_sr_rs_01/` | 35 |
  | `R/REVIEW_RV114/b2b3_plan_01/` | 6 |
  | `R/REVIEW_RV115/b2_kd_01/` | 11 |
  | `R/REVIEW_RV116/b3_d_01/` | 15 |

**The modified paths are exactly the two expected:**

| Path | Main blob → head blob | Change | Lineage |
|---|---|---|---|
| RR | `a18317…` → `f20fdd…` (sha256) | +722 / −0 | Byte prefix (§5) |
| WG | `23a6e74ba2` → `5d0107fce6` | +53 / −14 | Main's blob = NUM's at `6f983f12f3` (#1108's delta), and main last touched it in `4f37590bfb`, so **no change of main's is reverted** |

**A1-S-1 (no open product PR's package):**
- **#1112** (SI1c, draft, opened two minutes after #1111) has 9 paths: 5 slice files and `IMPLEMENTATION/SI1C/` (4). **It shares 0 paths with #1111**, and H has no `IMPLEMENTATION/SI1C/` path, as RR:14962 rules.
- #885 (a draft from 2026-09-24, not T3's) shares 0 of its 100 paths.

## 2. Nothing that must not be published: PASS (N-4 noted)

**What the screen covers:** every line of the 1,043 added files, including the decompressed contents of the 8 `.gz` files, and the `+` lines of RR and WG. That is 272,844 lines, with RV103's and RV110's patterns plus the brief's strict pattern (assembled at run time in my scripts).

Evidence: `_run_records/publication_scan_summary.txt`, `publication_hits.tsv` (home roots written `/U-sers/`, `~-/`, `/pri-vate/`; host name written `<host>`), `token_shape_scan.txt`, `gz_check.txt`, `leak_validator.txt`, `lock_log_cwds.txt` and `size_type_summary.txt`, with `scripts/`.

**Credentials: none.**
- **Token-shaped patterns** over every changed file in full, `.gz` decompressed, find **0**: GitHub tokens of real length, `github_pat_` bodies, `sk-` keys, `AKIA` + 16, PEM blocks, password and token values, Authorization header values, Slack tokens and JWTs.
- **Main's `validate_run_record_leaks.py --base e33f3e2f1b --head 18a20d329f`**, run in my copy: "PASS: 1044 changed run-record file(s) scanned; 0 possible credential(s); 0 machine-local symlink(s)", exit 0. 1,044 = the 1,043 added files plus RR; WG is not under a run-record path.
- **The name-pattern hits** are all in RV110's addendum records (its scan vocabulary and summaries). No added line outside them has a bare `sk-`; the `sk-` hits in RR and WG are main's prefix.

**The 8 gzipped files** (all I91's; `gz_check.txt`):
- every file decompresses;
- the 3 `n1_differential` files parse (2 JSON, 1 text; 27,218 lines), and the 5 junit files parse as XML;
- each has 0 home, private-temp or tmp paths, 0 e-mails, 0 token-shaped strings, 0 credential words and 0 strict-pattern hits;
- the junit files' only `WT/` strings are their sanitized test paths. **Their one host datum is the `hostname` attribute (N-4).**

**Personal data: the owner's configured Git identity only** (with N-4's caveat).
- **The owner's e-mail occurs 0 times.** The only address is the agent's no-reply attribution (5 trailers, and RV110's addendum summary quoting it). The three `+@pytest.mark` hits in I91's diff are not addresses.
- **The owner's name** occurs as the Git author in I89's `commits.txt` files and RV110's `delta_scope.txt`, and inside the junit host name.

**Whole-host data: none.**
- **0** UUIDs, tool-call or message ids, session ids, transcript paths, system paths or process-table headers; 0 `vm_stat`/`sysctl`/`top`-style output.
- **`pgrep` occurs only in memguard checks** whose output goes nowhere (I85, I89, I90 scripts; RV112's three mutant drivers).
- **"slack"** (RV112) is the word, not the application.
- **The 13 lock-log excerpts are each agent's own lines**, with `cwd=WT/<own folder>/…` only (`lock_log_cwds.txt`); there is no other agent's line and no process listing.

**Size and type** (`size_type_summary.txt`, `size_ignored_checks.txt`):
- The 1,045 changed files total 23,692,723 B. The largest is RR (1,282,756 B); the largest record is RV113's `runner_head.log` (972,788 B). **No file is over 2 MB.**
- `file --mime-type`: 789 plain text, 119 JSON, 50 shell, 41 Python, 25 diffs, 8 gzip, 7 empty, 2 CSV, and 4 sources misread as C, Java or Algol.
- **No build output** (no `target/`, `node_modules/`, `dist/`, `__pycache__`, `.pyc`, `.so`, `.wasm` path). The one `build/` folder is I85 CHECKPOINT_R3P's five logs, force-added under E-7 as RR:14276 records.
- **Nothing ignored is left behind** in the added folders (`git status --ignored`: no `!!` entry). NUM's working tree under T has 202 symlinks, all in RV56's two ignored `imported/` folders (known, untracked, not in H).

## 3. Portability: PASS

**GEN-8 by E-4's method** (`_run_records/gen8_pytest.log`):
- My own detached worktree of H at `WT/rv117/rv117_head` (created for this review and removed after it), read-only: `GIT_OPTIONAL_LOCKS=0`, `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`, `TMPDIR` in my scratch.
- HEAD was H before and after, with 0 status entries (including ignored) before and after.

  ```
  VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8
  ```

- **The result: 1 passed, 10 deselected in 30.05s**, exit 0, 22:33:16–22:33:46Z.

**The new item-3 check** (`symlink_rv58_check.txt`):
- **No symlink:** 0 mode-`120000` entries in M..H's raw diff, and 0 under `P/execution` in H (the tree's only 2 links are main's, under `execution/_Evaluation/`, outside the PR).
- **`R/REVIEW_RV58/` is exactly main's:** tree `79bfb398d4…` at M, H and N; 0 paths differ; 188 files, 0 links. Before #1109 (at `4f37590bfb`) the folder had 104 links. A records PR from a NUM that had not absorbed #1109 would have restored them; this one does not.

**Machine-absolute paths** in the 1,045 changed files (`abs_paths_by_file.tsv`, `.gz` decompressed):

| Check | Result |
|---|---|
| GEN-8's detector (`surface_roles.iter_machine_path_lines`), added lines | 5 lines, all false positives: `S + "/tmp/…"` and `f"--basetemp={S}/tmp/mut/…"` built from the agent's scratch variable in I89's `redact_01.py`, I91's two `mutants*.py` and I92's two `mutants*.py`. GEN-8's own test passes |
| Broad pattern | 5 more lines: RV110's addendum placeholders (`/U-sers/<user>`, `~-/dev`, as written there) and RV111's `rv111_stage.sh` guard regex |
| **Machine-absolute paths in total** | **0** |

**The living documents have none:** RR's 722 appended lines, WG's diff and the 12 briefs. The strict pattern's only hits there are vocabulary: three briefs' host rules, which list the banned home-relative, home-root and private-temp forms and the worktree's name, and E-10's text at RR:14352, :14359 and :14372, which names the forms it bans. My own records pass the strict pattern.

## 4. The living documents against the records and Git

Evidence: `_run_records/pr1108_merge_checks.txt`, `pr1109_checks.txt`, `branch_facts.txt`, `branch_facts_2.txt`, `rr_cited_hashes.txt`, `rr_cited_paths.txt`, `heading_citations_check.txt`, `ids_check.txt` and `redactions_e10_check.txt`.

### 4.1 #1108 → `4f37590bfb` (squash): confirmed (N-2)

- **GitHub:** MERGED at 14:42:26Z, head `ea3b1443ea`, base `2007709549`, merge commit `4f37590bfb`.
- **Git:** the squash has the single parent `2007709549` (= `PARENTS.txt`). Its tree equals H2's and NUM `6f983f12f3`'s (`45a54c1109…`). It changes 869 A and 2 M.
- **The commit body equals `SQUASH_BODY.txt`**, and it carries RV110's A1-N-1 correction ("RV110's review", "869 added, 2 modified, 0 deleted").
- **GEN-8:** `gen8.txt` has both heads, the command and the cwd (E-4).
- **CI:** four successful runs on H2 on GitHub; the record's `CI_RUNS.txt` lists H1's (N-2).
- **No piping product PR merged between #1108 and this cut.** Main's first-parent history after `4f37590bfb` is #1109 (`cccc41a293`) and #1110 (`e33f3e2f1b`). #1110 is App v4's (255 paths, all under `projects/chirality-app-v4/`, 0 under P).
- **RV110's addendum notes** are applied as RR:14252–14258 says: A1-N-1 in the squash body; A1-N-2 in WG (the pytest-under-lock rule added, the duplicated wait rule merged into the SW entry, the next safe action updated); A1-N-3's erratum to E-7's count, and the stray file.

### 4.2 #1109 and NUM's absorb (E-11): confirmed (N-5's last point)

- **#1109** merged at 16:08:23Z as `cccc41a293`; its commits are `4a58bf2a7d` (the fixture, records only) and `b928a20f98` (the validator, its test and two manifests).
- **The 104 links:** #1084 (`f506f3e2de`) added exactly 104 links under `R/REVIEW_RV58/`; #1109 retyped 94 to files, replaced 10 directory links by directories, and added 37 files (`FIXTURE_MATERIALIZATION.md` and the directories' contents). RV58 has 0 links on main.
- **NUM's absorb `e34419d94e`:** parents `769d0d0f46` and M; NUM's tracked links under P/execution go from 104 to 0; RV58's tree equals main's; the merge changes only RV58's 141 paths under P/execution, and 0 non-execution paths differ from main. So "the merge was clean" holds.
- **E-11's account** (the links dangled; ROOT's screen read contents, not link targets) agrees with `FIXTURE_MATERIALIZATION.md`.
- **For information, no finding:** #1109's `FIXTURE_MATERIALIZATION.md` quotes the links' former absolute target, including the owner's home and the worktree's name. It is another session's record, already on main, among over 2,000 historical T3 files on main with such paths. #1111 does not touch it.

### 4.3 The rulings since `6f983f12f3`

RR appends **30 sections** (722 lines, 69,862 B), from "RV110 confirms #1108's delta; …" to "RV111 confirms SI1c's repair round; …".
- **All 26 return and review sha256 citations** in the appended text match the files in H (`rr_cited_hashes.txt`). So do the uncited-form hashes I checked: `REDACTION_01.md` `924532ef…`, I89's RETURN.md `05c2687d…` (unchanged), `B1_COMMON.md` `2d170307…`, I96's v0 and r1 statics (`5bf0d0dc…`, `6edae5ff…`; `71f63d39…` 9,733 B; `c4987e87…` 51,163 B).
- **Of the 68 record paths cited** in the appended text and WG's T3 section, 67 exist in H. The one absent is `IMPLEMENTATION/SI1C` (RR:14952), which RR:14962 itself keeps out of this PR (`rr_cited_paths.txt`).
- **Every heading citation resolves:** 51 of 51 in WG's rulings in force and 4 of 4 in the appended text. The 4 other quotations are not headings: two quoted phrases (RR:14414, :14579), a code fragment (RR:14581) and RR:14621's quotation of RR:13959's text.
- **Branch facts** (`branch_facts.txt`): `b1` `56c5579f07` (1 commit over I1, 6 files +796/−156); `b1-a` `6b62606778` (1, 3 files +569/−85), `77f4391a85` (`--no-ff` of `56c5579f07`), `9812c83ded` (2 files +29/−2); `b1-r` `cc81e78801` (3 commits, 3 files +483/−26), repair `b5cb7faaeb`; `b1-t` `7e47e51b5d` (4, 2 files +262/−9); `b1-p` `75132d2673` (6, 4 files +415/−11), `11cc14e3e6` (3 more, 2 files +148/−18); I2 `eca6c00a72` = (`c17340d50b`, `9812c83ded`); `b1` `603e238517` (11 files against I1); SP's `7458527ff7` is in `c17340d50b`; SI1c `f5665f8862` (2 commits over `7f233b2e01`, pushed; the same 5 files against `025c1cf326`, unchanged on main since). The local branch tips equal these. The only miscount is N-5's first point.
- **E-10's staging slip:** `a6801b8a29` added 63 I90 paths and `a74aa63084` removed them, as RR:14374 says.

**The owner's decisions:** no new owner decision was made in this span. The appended text quotes no owner words; its owner mentions are what ROOT tells the owner (RR:14624, :14639). WG's "Owner decisions in force" block is byte-unchanged from main, which RV110 verified against RR.

**No owner-held decision was taken by ROOT.** Each item the brief names:
- **B2/B3's decisions 22–24** (B0's 17–19 prepared: M above 12 GiB; R-2 for B8; the native-app witnesses) stay prepared and undecided: I93 PLAN §5 marks them **Owner**, REVISION_01 §6 amends only decision 22's package text ("about 13.25 GiB"), and RR:14648 says "22–24 stay owner-held and undecided". Each maps to an existing item on WG's owner-held list. RR:14596's numbering slip is N-5.
- **B3-S's per-route pricing and decision 26:** I93 makes decision 26 ROOT's within 12 GiB and the owner's above. I95's exact route prices 9,900,151,888 B dense at C = 3, which is 246,708,348 B inside 0.9 × 10.5 GiB; the provisional zero rules decide 10.5 against 11.25 GiB, both within 12 GiB; "no route caps" needs nothing above 12 GiB. One graph over both routes (14.75–16 GiB) is rejected, not selected. **Within ROOT's authority.**
- **SP's headline rule:** a clarification of DESIGN_v2 T-11 that the accepted base readers' G7 already forces (HEADLINE_BINDING and HEADLINE_PRESENCE), on successors only, and successors are not public before B8 (RR:12374, the B0 and B6 precedents). RV-P round 2 checks it. **Not public meaning; ROOT's.**
- **B3-D's rulings (B3D-1 to B3D-18; S-1, S-2; K3-1, K3-2):** I96 §9 marks none owner-held, and RV116 agrees with all 18. **None changes public meaning before B8**, I judge, because each concerns either a new identity or the retained successors (DEF-E, XTABLE, B3a/B3b's reader branches, the T6S entry, the interim admission), none of them public before B8, or an unchanged public outcome. B3D-12 changes no physics-source-1 outcome in RS or PY (RV116 N-9), and TS's form became an unchanged export (REVISION_01's N-9, confirmed). B3D-10's tightenings apply to 0.3.0 successors only and wait on a clean 07n census. K3-2 changes FK only for the new `ExactENu` operand; the six non-`ExactENu` oracle vectors are unchanged, and any other oracle byte is a stop. RV116's addendum finds "no new public meaning".
- **R-COMB-1** (I93's decision 8, decider ROOT, "the owner is informed"): RR:14639–14642 tells the owner and selects nothing for the owner; its representation is left to B2-C's review. It enforces C1 §5–§6's existing operand standing on successors that are not public before B8, as RV114 agrees. **Owner informed, not owner-decided, and not ROOT taking an owner choice.**

**B1's phase-2 rulings, SI1c's, B2-KD's and B3-K's, as routed:** each review's finding table is fully routed or answered:
- **RV109's early read** R3P-1 to R3P-9 (0/2/7) → RR:14428–14440, with R3P-1 a precondition of I2 that I2's section confirms (`c17340d50b` carries it).
- **RV112** SF-1, N-1 to N-8 (0/1/8): SF-1, N-1, N-2, N-5 → SQ; N-3, N-4 → I85; N-7 done in `7458527ff7`; N-8 explained by E-10; N-6 needs none.
- **RV113** S-1, N-1 to N-5 (0/1/5) → the SR-RS repair round (S-1, N-1, N-3), DESIGN kept (N-2), PR-B1's text (N-4), none (N-5).
- **SR-PY:** the Build check, (b)–(e) repaired and kept, (f) and (g) to RV-R, as I91's RETURN and REPAIR_01 report (1,922 → 1,934 → 1,937; 30/30 and 40/40 mutants).
- **SI1c rulings 2 and 3** (RR:14193–14194, on main) are carried out in I88's REPAIR_01 (`fa78c80c32`, `f5665f8862`) and confirmed in RV111's ADDENDUM_01 §2–§3, with no residual finding.
- **RV114** S-1 to S-4, N-1 to N-12 (0/4/12) → R1's items 1–7.
- **RV115** SF-1 to SF-4 (B2-K's brief), N-2 to N-9 (R-1/R-3/R-4's conditions, B2-C, B2-K), N-1 and N-10 needing none; R-1 to R-11 accepted with those conditions.
- **RV115's addendum** SA-1, SA-2, NA-3, NA-5, NA-6 → B3D-6 and B3D-16's rulings; NA-4's remedy is in J2k's acceptance; NA-1 and NA-2 need none.
- **RV116** S-1, S-2, N-1 to N-12 (0/2/12) and its addendum's NA-1 → RR:14824–14840 and RR:14915.

Every review's counts and the figures RR quotes from them (for example 88,155, 433,221 and 106,884 for RV111; 82,258, 326,320 and 402,352 for RV112; 12.25–14.0 GiB for RV114; 4,010 nets and 2^-423 for RV115) occur in the reviews.

**E-10** is checked in §5.

### 4.4 WG's T3 section (N-1)

- **The positions** agree with Git and RR at N, except WG:564 and WG:567 (N-1): #1108 at `4f37590bfb`; SA at I2 `eca6c00a72`; SP returned before I3; the readers under RV113; B2/B3 phase 0 with B2-KD, B3-S and B3-D ruled.
- **The owner-held list** is unchanged from main and still covers B2/B3's 22–24 through its existing items (M above 12 GiB; I75's R-2; the native-app witnesses).
- **The owner decisions in force** are unchanged from main.
- **The next unused IDs at N are I97 and RV117** (`ids_check.txt`). In text files, RV117 occurs only in RR:14778 and WG:613 ("next unused"), and I97 only there, in RR:14708 ("next unused") and in RV116's note that it was then unused (`REVIEW.md:137`); RV118 occurs nowhere. I90–I96 and RV110–RV116 all have folders. The dispatched ranges (I68–I96, RV101–RV116) are right.
- **The next safe action** matches RR's order for B1 (I3, I4, RV-P round 2, then SC/SQ/SG/SB/SK) and B2/B3, except items 2 and 3's first steps (N-1). Item 4 forecasts this PR. Item 5's cleanup list rightly adds `records-pr-b` (#1108's) and omits `records-pr-c` (#1111's).

## 5. Integrity: PASS

**The sum files**, read from H's committed tree with `git cat-file` (`_run_records/sums_committed_tree.tsv`, `sums_folders.tsv`). "Uncovered" counts files in the folder that none of its sum files lists.

| Folder (T-relative) | Sum file(s) | Result | Uncovered |
|---|---|---|---|
| `IMPLEMENTATION/RECORDS_MERGE_2026-10-07B/` | SHA256SUMS | 5/5 | 0 |
| `R/I85/b1_sp_01/` | SHA256SUMS; .return | 44/44; 86/86 | 0 |
| `R/I88/si1c_01/` | SHA256SUMS (main, RETURN's); .repair_01 | 51/51; 29/29 | 0 |
| `R/I89/b1_sa_01/` | SHA256SUMS; .followup_01; .redaction_01 | 75/75; 17/17; 4/4 | 0 |
| `R/I90/b1_sr_rs_01/` | SHA256SUMS; .repair_01 | 64/64; 23/23 | 0 |
| `R/I91/b1_sr_py_01/` | SHA256SUMS; .repair_01 | 56/56; 29/29 | 2: RETURN.md and REPAIR_01.md, by design ("SHA256SUMS over every other file"), each anchored in RR (`bc7fa865…`, `f97b46ba…`) |
| `R/I92/b1_sr_ts_01/` | SHA256SUMS | 29/29 | 0 |
| `R/I93/b2b3_plan_01/` | SHA256SUMS; .revision_01 | 9/9; 1/1 | 0 |
| `R/I94/b2_kd_01/` | SHA256SUMS | 4/4 | 0 |
| `R/I95/b3_s_01/` | SHA256SUMS | 44/44 | 0 |
| `R/I96/b3_d_01/` | SHA256SUMS (v0); .revision_01 | **11/11; 20/20** | 0 |
| `R/REVIEW_RV109/rvp_r3p_read_01/` | SHA256SUMS | 76/76 | 0 |
| `R/REVIEW_RV110/records_01/` | SHA256SUMS (main); .addendum_01 | 41/41; 20/20 | 0 |
| `R/REVIEW_RV111/si1c_01/` | SHA256SUMS; .addendum_01 | 111/111; 24/24 | 0 |
| `R/REVIEW_RV112/rvq_round1_01/` | SHA256SUMS | 170/170 | 0 |
| `R/REVIEW_RV113/rvr_sr_rs_01/` | SHA256SUMS | 34/34 | 0 |
| `R/REVIEW_RV114/b2b3_plan_01/` | SHA256SUMS | 5/5 | 0 |
| `R/REVIEW_RV115/b2_kd_01/` | SHA256SUMS; .addendum_01 | 4/4; 5/5 | 0 |
| `R/REVIEW_RV116/b3_d_01/` | SHA256SUMS; .addendum_01 | 8/8; 5/5 | 0 |
| `R/I85/b1_st_01/`, `R/REVIEW_RV109/rvp_round1_01/` (main; unchanged) | 2 each | 60/60, 34/34; 96/96, 76/76 | 0 |

**Totals:** the 29 sum files the PR adds hold 1,012 entries, all OK, 0 bad, 0 missing. With the 6 on main in these folders: 35 files, 1,370 entries, all OK. Every added folder with a sum file is in the table; the briefs are unsealed, as before.

**I96's layout:** v0's `SHA256SUMS` (11) still seals `statics/`'s three v0 drafts at their original paths and hashes (`165c9d74…`, `6edae5ff…`, `5bf0d0dc…`). `SHA256SUMS.revision_01` (20) covers the v0 files, v0's own `SHA256SUMS`, `REVISION_01.md`, `_run_records/revision_01/` and `statics/r1/`'s four files (`CARRIER_PROFILE_ENUMS.diff`, `SCHEMA_ENUM.diff`, DEF-E, XTABLE).

**The 8 `.gz` files** verify as compressed bytes in I91's sums, and decompress (§2).

**RR is append-only: PASS** (`_run_records/rr_append_only.txt`). Main's RR (1,212,894 B, 14,240 lines) is an **exact byte prefix** of H's (1,282,756 B, 14,962 lines); H's blob equals N's.

**The originals: PASS** (`redactions_check.txt`, `redactions_e10_check.txt`).
- **REDACTIONS.json's 13:** each original blob at `dfa5e2dc44` hashes to its `original_sha256`, and each H blob to its `redacted_sha256`. **0 of the 13 original blobs** are in H's tree (67,448 distinct blobs) or M's, and 0 of the 1,045 changed files hashes to an original.
- **E-10's originals (RR:14358: "The next records PR's reviewer checks that none is present"):** REDACTION_01's tables list 50 files. The 47 committed at `bc37d43a0a` (the 46 records and `SHA256SUMS`) hash there to the tables' old sha256, and **none of those 47 blobs is in H or M**. All 50 of H's blobs hash to the tables' new sha256. **No changed file hashes to any old sha256**, which covers the three never committed (FOLLOWUP_01.md's first form, `followup_01/sanitize.py`'s and `SHA256SUMS.followup_01`'s). Between `bc37d43a0a` and H, exactly 46 files under I89's `_run_records/` changed (and 19 were added).

## 6. Gate evidence: PASS (the records-only gate set)

The records-only gates are GEN-8, the PR's automatic CI and an independent review. This PR changes no product, test, CI or portability-policy path, and nothing under T's `REFERENCES/` or `DESIGN_NUMERICS/`.

| Gate | Evidence | Exact head? | Result |
|---|---|---|---|
| GEN-8 (ROOT) | Relayed at dispatch: 1 passed, 10 deselected, on H. I did not see ROOT's file; it goes into the merge record | H | **pass**. The record should carry the head and the command (E-4) |
| GEN-8 (RV117) | §3 | H | **pass** |
| `validate_run_record_leaks.py` (ROOT and RV117) | Relayed: 1,044 files, 0, 0; mine identical (§2) | M..H | **pass** |
| governance-harness | 37696600661, on the merge ref `2931f9176` (= M + H), with `CHIRALITY_REQUIRE_LIVE_TESTS: 1`: "1161 passed, 48 subtests passed" | H | **success** |
| Harness Pre-merge Validation | 37696600645 (Select App coverage and Harness pre-merge succeeded; App jobs skipped by selection) | H | **success** |
| pec-tests | 37696600647 (Select PEC coverage and pec succeeded; workspace tests skipped) | H | **success** |
| Piping Desktop E2E | 37696600726 (Select source coverage and source-mode E2E succeeded; numerical, remainder and accessibility skipped by selection) | H | **success** |
| Independent review | this report | H | **PASS** |

## What ROOT must rule on

Nothing blocks the squash. In order:
1. **N-3, for the squash body:** "I88's repair round" (not its return) and "RV109's early read of SP" (not ST's repair confirmation), both of which are on main already; optionally list RV113's SR-RS confirmation under "Not in this PR".
2. **Before the merge:** check that main is still `e33f3e2f1b` and #1112 is still separate; then `gh pr merge 1111 --squash --match-head-commit 18a20d329fc09335f013131766d7f74dbfa7f675` with the explicit subject and body. The merge record should carry ROOT's GEN-8 (head and command) and the leak check's output, and CI_RUNS for H itself (cf. N-2).
3. **N-2 and N-5:** one RR erratum at the next append.
4. **N-1:** the next WG touch; consider updating WG in the same commit as each position-changing RR section.
5. **N-4 (optional):** whether to add the host's machine name to the commit-time screen, and whether any repository-wide redaction of it is wanted. Nothing in #1111 needs to change for it.
6. **Keep `WT/records-pr-c`** (#1111's checkout) out of any cleanup until #1111 merges.

## Host and method

- **My copy:** a detached worktree of H at `WT/rv117/rv117_head` (created with `git worktree add --detach`, the brief's alternative to `git archive`, because E-4's GEN-8 needs a Git checkout). I wrote nothing in it, and removed it and its registration at the end. Logs and scripts are in `WT/scratch/rv117_records_01/`, with `TMPDIR` there.
- **Reads:** NUM's object store and working tree with `GIT_OPTIONAL_LOCKS=0` (`log`, `diff`, `ls-tree`, `show`, `cat-file`, `merge-base`, `rev-list`, `grep`, `status --ignored`, `check-ignore`, `ls-remote`); `gh` read-only for #1108–#1112, #885 and the CI runs, jobs and one log; ROOT's PR checkout `WT/records-pr-c` (HEAD = H, clean) and ROOT's scratch listing, read only.
- **What I ran:** one GEN-8 pytest and one `validate_run_record_leaks.py`, both in the foreground in my copy, plus read-only Python scans and hashing. No cargo, no other test, no install, no Git write to any branch or remote, no DEC-025, no use of the host lock. No wait was needed; no process of mine remains. Nothing went to the system temp directory.
- **The scripts** (`_run_records/scripts/`): `publication_scan.py`, `token_shape_scan.py`, `abs_scan.py`, `gz_check.py`, `redactions_check.py`, `heading_citations_check.py` and `sums_committed_tree.py` are RV110's, changed in their docstrings, in assembling the tilde form at run time, and (the first) by adding the strict pattern and an output sanitizer; `gz_check.py` also parses by type and counts the host attribute. `sums_folders.py`, `redactions_e10_check.py`, `rr_cited_hashes.py` and `rr_cited_paths.py` are new.
- **Records:** this file, `_run_records/` and `SHA256SUMS`, with placeholder paths only, no symlink and no folder named `build`, screened with the brief's strict pattern.
