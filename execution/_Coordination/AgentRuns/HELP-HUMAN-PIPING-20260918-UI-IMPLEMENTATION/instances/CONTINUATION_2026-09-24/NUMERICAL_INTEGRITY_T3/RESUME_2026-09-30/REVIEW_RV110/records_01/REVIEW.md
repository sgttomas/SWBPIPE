# RV110: independent review of the T3 records-only PR after #1107 (#1108)

**Reviewer:** RV110, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of these records. 2026-10-07 UTC.

**Brief:** `R/BRIEFS/RV110_RECORDS_PR6_REVIEW.md` (sha256 `fa6586bd…`, verified at the start and again before writing), read in full. Its method is `R/BRIEFS/RV103_RECORDS_PR4_REVIEW.md` items 1–6, with the brief's substitutions. Also read: NUM's `AGENTS.md` (`f96feb19…`) and `agents/AGENT_TASK.md` (`1a13a5b0…`). Precedents read first: RV106's brief and `REVIEW.md` (#1105), then RV103's brief and `REVIEW.md` (#1103).

**Placeholders.** WT = the t3 workspace; NUM = WT/numerics; P = `projects/chirality-piping`; T = P/execution/…/NUMERICAL_INTEGRITY_T3 (written `T3/` in the run records); R = T/RESUME_2026-09-30; RR = T/ROOT_RULINGS_V1.md; WG = P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md; DT = P/apps/desktop/src; VENV = the piping venv's Python 3.13.14. `RR:n` and `WG:n` are line numbers at the PR head.

**NUM moved during the review.** At dispatch NUM was `7edc00f021`; by the end it was `a1180c37ce` ("I88's SI1c verified and ruled; RV111 and I90 (SR-RS) dispatched"). Both commits after N are records only and are not in #1108. Every check below is against N = `25c745f905` and the PR head H.

## The candidate

| Item | Value |
|---|---|
| PR | #1108, `codex/piping-t3-records-20261007b` → `main`, ready (not draft), MERGEABLE (merge state BLOCKED pending review) |
| H (head) | `145443e9e4c50d219c08222b89c4136d6bb95422`, one commit, sole parent M; origin's branch = H |
| M (base, main) | `2007709549e9701b302e0eb62a1474d82acc1c40` (#1107's merge); origin's main is still M |
| N (source, NUM) | `25c745f905250aec2ce7d2f94726561b5ed49de4` |
| Author and committer | the owner's configured Git identity; the only other identity is the agent co-author trailer |

## Verdict: **PASS** (no blocking finding)

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 4 |

The following pass:
- **Scope.** H is M plus one commit, not from NUM's history. **H's whole tree equals N's** (tree `9cfe3c3bcc…`). 768 paths change: **766 A, 2 M, 0 D**, all under `P/execution/`. The modified paths are exactly RR and WG.
- **Publication screen.** No credentials, whole-host data, binaries or large files. The only personal data is the owner's configured Git identity. **All 28 gzipped files decompress** (8,711 JSON lines, all parse) and screen clean.
- **Portability.** GEN-8 passes on the exact head (mine and ROOT's). The changed files hold 0 machine-absolute paths, and the living documents hold none.
- **Integrity, except S-1.** 30 sum files, 787 entries: 783 OK, 0 bad, **4 missing** (S-1). RR is append-only. None of the 13 redacted originals is in H.
- **Gates.** The four automatic CI runs succeeded on H.
- **The living documents.** #1105's, #1106's and #1107's merges, the dispatches, B6_MERGE's corrections, the owner's three decisions (quoted identically in RR and WG), the B6, SW, ST and R3 rulings, RV108's and RV109's routing, and WG's T3 section all agree with the records, Git, GitHub and the host. **ROOT took no owner-held decision.** I agree that B6's item-2 widening does not change public meaning (§4.5; N-1 adds a caveat).

One thing should be fixed, preferably before the squash:
- **S-1: four of I85's sealed evidence files were never committed.** P's `.gitignore` rule `build/` hid them. This is RV102 S-1's defect again, and the committed-tree rule adopted then (RR:13099) was not applied.

## Findings

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | `R/I85/b1_st_01/SHA256SUMS` (57/60), `SHA256SUMS.repair_01` (33/34); RR:13997 ("SHA256SUMS 60 of 60 OK"), RR:14171 ("34 of 34 OK") | **Four sealed files are absent from H** (`_run_records/i85_uncommitted_files.txt`):<br>- `_run_records/build/build01_norun.log`, `build_identities.txt` and `lib01.log`;<br>- `_run_records/repair_01/build/r1_lib01.log`.<br>`git check-ignore` attributes all four to `P/.gitignore:20` (`build/`), and `git ls-files` lists none of them. **On disk in NUM all four match their sums.** They hold placeholder paths only: 0 home, private-temp or tmp paths, 0 e-mails, 0 token-shaped strings.<br>RETURN.md:151 cites `_run_records/build/build_identities.txt`, and REPAIR_01.md:152 cites `build/`. Both citations are dangling in the committed tree.<br>RR's 60/60 and 34/34 held only on the host. **The rule "return verification checks sums against the committed tree" (RR:13099; WG:615) was not applied.** NUM's `git status --ignored` under `P/execution` shows no other ignored record file: only `__pycache__` and RV56's pre-existing `imported/`. | `git add -f` the four explicit paths in NUM, as RV102 S-1 was repaired, then either:<br>(a) re-cut #1108 from that NUM commit, and I confirm the delta; or<br>(b) carry them in the next records PR. Their sum files are sealed and need no change.<br>(a) is the precedent and matches "must verify". At the next append, add an erratum to RR:13997 and RR:14171, and run the committed-tree check (`git status --ignored` or `git ls-files`) for every return. |
| N-1 | NOTE | RR:13963–13968 (B6 item 2, "Public meaning does not change"); DT `features/results/HistoricalRunContext.tsx:291`, `:360` | **I agree with the judgement, but the refusal code is visible.**<br>- `HistoricalRunPanel` lists the reader's first refusal code as a finding when a saved `retained_preview_physics` result fails revalidation (`_run_records/public_meaning_check.txt`).<br>- So for such an input, B6 changes the displayed string, for example from `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` to `SOURCE_NUMERICAL_CASE_INVALID`.<br>Accept/refuse and standing do not change. The production build has no producer of such a file. RR's stated reason ("only the code aligns") is right but leaves out the strongest ground: RR:12374's standing rule that "successors are not public before B8". | Optional, at the next append: cite RR:12374 and decision 11's class as the basis, and note that the code is visible on the historical-run panel. Nothing needs undoing. |
| N-2 | NOTE | WG:600, WG:619, WG:586–590, WG:568, WG:629–643, WG:647 | **The living account lags its own rulings in six places:**<br>- **WG:619:** "Rulings in force" still lists "M up to 6.0 GiB is ROOT's", which RR:13699 and RR:13724 superseded. The 12 GiB/S3 section, R1, PLAN_v2's acceptance and the two new host rules are absent: RV104 N-6's pytest-under-lock rule (RR:13590) and the quiet-check fix (RR:13855).<br>- **WG:586–590:** "Owner decisions in force" keeps the 6.0 GiB and 64 GiB entries without marking them superseded. Only their successors say so.<br>- **WG:600:** the IDs line contradicts itself: "Dispatched … I68–I87 …" and then "Dispatched I68–I89 …".<br>- **WG:568:** the B1/B6 row gives ST's branch at `a8e719f5b4`. The branch moved to `98a77c716e` (repair) and then I1 `262bd687f0`, which only the State column has.<br>- **WG:629–643:** "Notes routed" has none of RV108's N1–N6, the A-N2 additions or RV109 N-5. They are reachable only through the rulings-in-force pointer (WG:627).<br>- **WG:647:** this names RV110 as SI1c's reviewer, but RV110 went to this review. NUM `7edc00f021` has already corrected it to RV111. | At the next WG touch: re-point the M entry to "Owner decision: M's practical limit is 12 GiB; target machines" and add the S3, R1/PLAN_v2 and host-rule entries; mark superseded owner decisions; fix the IDs line; update the row's ST head; list the routed RV108 and RV109 notes. |
| N-3 | NOTE | `IMPLEMENTATION/SI1B_MERGE/_run_records/PR_CUT.txt:5`; `IMPLEMENTATION/B6_MERGE/_run_records/gen8.txt`; RR:13649 vs RR:13903–13904 | **Three record details:**<br>- **The SI1b dispatch command has a short SHA.** PR_CUT.txt records it as `-f target_base=47a3bdfcf5`, but run 37555520168's plan shows `target_base` = `47a3bdfcf5a37e856465c45cf904383f10181498` (`_run_records/ci_dispatch_target_base_excerpt.txt`). If that line was the template for #1107's dispatch, it explains run 37620732340.<br>- **B6_MERGE's `gen8.txt` has only pytest's output.** It has no head SHA and no command, which E-4 requires (RR:12158). SI1B_MERGE's and RECORDS_MERGE's copies have both. B6_MERGE's RECORD.md states the head.<br>- **The expected count reads "49 → 57", the result "50 → 58".** RR:13649 expected `expression_evaluator` 49 → 57, and RR:13904 reports 50 → 58 "as expected". Both are right: 49 → 57 is the lib unit tests, and 50 → 58 is the manifest total with the 1 conformance-corpus test (`_run_records/dec025_counts.txt`). The records do not say so. | An RR erratum at the next append, covering all three. The sealed folders stay unchanged. |
| N-4 | NOTE | PR #1108 description; H's commit message | **The "What it contains" list is wrong in four places:**<br>- It names "B6's PR package". `IMPLEMENTATION/B6/` reached main with #1107; #1108 adds none of it.<br>- It names "RV105 (B0)" and "RV103 (#1103)" among the reviews. Both reached main with #1105; #1108 adds no file under either.<br>- It leaves out RV106's review (35 files), #1105's merge record (`RECORDS_MERGE_2026-10-07/`), `SESSION_2026-10-07/` (the aborted run's quiet log and the fixed `run_dec025.sh` copy) and I79's REPAIR_01.<br>- The commit message's "the reviews RV104 to RV109" likewise implies RV105.<br>The rest is accurate: 766 A, 2 M, 0 D; the SI1b and B6 merge records; I81–I87; RV104, RV107, RV108 and RV109 with their addenda; the briefs, RR and WG; the screening and gates. | Word the explicit squash body to match: "RV104, RV106 to RV109"; #1105's merge record and SESSION_2026-10-07; I79's REPAIR_01; no B6 package. |

## 1. Scope: PASS

Evidence: `_run_records/scope_checks.txt`, `changed_paths_name_status.tsv`, `pr1108_view.json` (identity e-mails dropped), `pr1108_body.md` and `open_prs.txt`.

**Parentage:**
- H's only parent is M, and `rev-list --count M..H` = 1. `ls-remote`: main = M, and the PR branch = H.
- H is not an ancestor of N or of NUM's later heads. **None of NUM's 806 commits in `M..N` is an ancestor of H.**
- M is an ancestor of N: NUM absorbed #1107 as `f40d8b7ded`.

**Equality with NUM:**
- **H's whole tree equals N's** (`9cfe3c3bcc…`). `P/execution` = `165019c2e5…` on both sides, and `git diff N H` is empty.
- The non-execution diff is empty against main, from both N and H.

**Changed paths** (`git diff --no-renames M H`):
- **768 paths: 766 A, 2 M, 0 D**, all under `P/execution/`. This matches the brief's expectation.
- **Modes:** 756 regular and 12 executable:
  - `SESSION_2026-10-07/host_tools/run_dec025.sh.txt`;
  - four of I82's `_run_records/*.sh`;
  - three of I86's timing scripts;
  - four of RV108's `evidence/scripts/*.sh`.

  There are no symlinks.
- Added by folder:

  | Folder | Files |
  |---|---|
  | `IMPLEMENTATION/SI1B_MERGE/` | 18 |
  | `IMPLEMENTATION/B6_MERGE/` | 18 |
  | `IMPLEMENTATION/RECORDS_MERGE_2026-10-07/` | 6 |
  | `IMPLEMENTATION/SESSION_2026-10-07/` | 3 |
  | `R/BRIEFS/` (B1_COMMON, B1_PLAN, B1_SA, B1_SP, B1_ST, B1_ST_REPAIR_01, B1_SW, B6_READER_ITEMS, RV106–RV109, SI1C_IMPLEMENT, SI1C_PLAN) | 14 |
  | `R/I79/si1b_01/` (REPAIR_01) | 14 |
  | `R/I81/b1_probe_01/` | 17 |
  | `R/I82/b1_cap_study_01/` | 54 |
  | `R/I83/b6_01/` | 35 |
  | `R/I84/b1_plan_01/` | 11 |
  | `R/I85/b1_st_01/` | 92 |
  | `R/I86/b1_w_probe_01/` | 84 |
  | `R/I87/si1c_plan_01/` | 6 |
  | `R/REVIEW_RV104/si1b_01/` | 86 |
  | `R/REVIEW_RV106/records_01/` | 35 |
  | `R/REVIEW_RV107/b1_plan_01/` | 7 |
  | `R/REVIEW_RV108/b6_01/` | 92 |
  | `R/REVIEW_RV109/rvp_round1_01/` | 174 |

**The modified paths are exactly the two expected:**

| Path | Main blob → head blob | Change | Lineage |
|---|---|---|---|
| RR | `3117de0f48` → `fab35c4bdc` | +677 / −0 | Main's blob = NUM's at `030020aca3` (#1105). Byte prefix (§5) |
| WG | `8ba1e1c333` → `8e99d2bd0f` | +31 / −16 | Main's blob = NUM's at `030020aca3`. Main last touched it in `47a3bdfcf5`, so **no change of main's is reverted** |

**A1-S-1 (no open product PR's package):**
- The only other open PR is #885, a draft from 2026-09-24. It is not T3's, and it shares 0 of its 191 paths with #1108.
- `IMPLEMENTATION/SI1B/` and `IMPLEMENTATION/B6/` reached main with #1106 and #1107, and are unchanged here.
- No SI1c or B1 package is in H.

## 2. Nothing that must not be published: PASS

**What the screen covers:**
- every line of the 766 added files, including the decompressed contents of the 28 `.gz` files;
- the `+` lines of RR and WG.

That is 177,253 lines. It uses RV103's and RV106's patterns.

Evidence: `_run_records/publication_scan.py`, `publication_scan_summary.txt`, `publication_hits.tsv` (owner e-mail written `<owner-email>`), `token_shape_scan.py`, `token_shape_scan.txt`, `gz_check.py` and `gz_check.txt`.

**Credentials: none.**
- **Token-shaped patterns** over every changed file in full, with `.gz` decompressed, find **0**: GitHub tokens of real length, `github_pat_` bodies, `sk-` keys, `AKIA` + 16, PEM blocks, password and token values, Authorization header values, Slack tokens and JWTs.
- **The name patterns hit only RV106's own records** (425 hits): its pattern tables, summary, review prose and committed `publication_hits.tsv`. That is scan vocabulary.
- **No added line outside RV106's records has a bare `sk-`.** The whole-file hits in RR and WG are in main's prefix: WG:515's "task-management" link and RR:13215's prose about it.

**The 28 gzipped files** (`R/REVIEW_RV108/b6_01/evidence/outputs/*.jsonl.gz`; `gz_check.txt`):
- every file decompresses;
- all 8,711 lines parse as JSON;
- each file has 0 hits for home, private-temp or tmp paths, e-mails, token-shaped strings, credential words, host data, the owner's name, and even the `WT/` placeholder.

They are the readers' probe and corpus outputs.

**Personal data: the owner's configured Git identity only.**
- **The owner's e-mail occurs twice in the added text,** both as commit attribution:
  - RV104's `evidence/addendum_02/pr1106.json` (GitHub's PR JSON);
  - RV108's `evidence/addendum_01/pr_commit_and_files.txt` (a `git cat-file` author line).

  It equals `git config user.email` and the author of M and H, so it is the configured identity the brief allows, and it is already in main's commit metadata. ROOT's "the only email address is the commit attribution" holds in that sense. RV103 and RV106 dropped e-mails from their own records, and so do I.
- **Every other e-mail-pattern hit** is the agent's no-reply address, in `SQUASH_BODY.txt`, `B1_COMMON.md` and the attribution trailers. The 5 hits in I83's `b6.diff` are `+@pytest.mark…` lines, not addresses.

**Whole-host data: none.**
- **0** UUIDs, tool-call or message ids, session ids, transcript paths, system paths or process-table headers outside RV106's vocabulary.
- **`ps`/`pgrep` occur only in scripts whose output goes nowhere:**
  - `run_dec025.sh.txt:16` (`busy()` counts matches with `wc -l`) and `:20` (the memguard check);
  - I85's two `run_suites.sh:14` memguard checks.
- **"macbook pro" (RR:13694)** is in the owner's quoted words: a hardware fact, as RV106 accepted for RR:13416.
- **"slack" (I86 `PROBE.md:214`)** is the word "slack", not the application.
- **The lock-log excerpts are each agent's own lines** (`cargo_jobs_i81/i85/i86`, RV104's two, RV109's two): timestamps, WAIT/START/END, own PIDs, `cwd=WT/<own folder>/…` or `ARCH/…`, and cargo arguments. There is no other agent's line and no process listing.
- **The aborted run's quiet log** (`SESSION_2026-10-07/…aborted1_quiet.log`, 148 lines) holds only `busy=1 quiet=0` counts.

**Size and type** (`_run_records/size_type_summary.txt`):
- The 766 added files total 11,205,203 B. The largest is 197,794 B (RV109's `probe_base.filtered.log`), and RR itself is 1,206,006 B. **No file is over 2 MB.**
- **`file --mime-type`:**
  - 582 plain text, 71 JSON, 32 shell, 30 Python and 1 CSV;
  - 11 diffs, and 6 sources misread as Java or Algol;
  - 28 gzip, the evidence above;
  - 5 empty logs, where `fmt` and `tsc` printed nothing.
- **No build output:** no `target/`, `node_modules/`, `dist/`, `__pycache__`, `.wasm`, `.rlib`, `.so`/`.dylib` or `.pyc` path. (S-1's files are logs that the `build/` rule hid, not build output.)

## 3. Portability: PASS

**GEN-8 by E-4's method** (`_run_records/gen8_pytest.log`):
- The brief's test, run in a Git checkout of the exact head. The only checkout of H on this host is ROOT's PR worktree `WT/records-pr-b`.
- It ran read-only: `GIT_OPTIONAL_LOCKS=0`, `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`, and `TMPDIR` in my scratch.
- HEAD was H before and after, with 0 status entries (including ignored) before and after.

  ```
  VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8
  ```

- **The result: 1 passed, 10 deselected in 28.59s**, exit 0, 14:07:26–14:07:55Z.

**Machine-absolute paths** in the 768 changed files (`_run_records/abs_scan.py`, `abs_paths_by_file.tsv`; `.gz` decompressed):

| Check | Result |
|---|---|
| GEN-8's detector (`surface_roles.iter_machine_path_lines`), added lines | 3 lines, all false positives: a pytest `--basetemp` of `{S}` + "/t-mp/mut/…" (written here with a hyphen) in I83's `scripts/mutants.py:73,85` and RV108's `evidence/scripts/mutants.py:46`. `{S}` is the agent's scratch variable, so the path is relative to it. GEN-8's own test does not scan record folders and passes |
| Broad pattern | 3 more added lines, all RV106's placeholders or quotations (`/U-sers/<user>`, `~/dev`) |
| **Machine-absolute paths in total** | **0** |

**The living documents have none:** RR's 677 appended lines, WG's diff and the 14 added briefs. My own records also pass GEN-8's detector, with 0 lines.

## 4. The living documents against the records and Git

Evidence: `_run_records/living_docs_git_checks.txt`, `dec025_host_vs_committed.txt`, `dec025_counts.txt`, `ci_dispatches.txt`, `ci_dispatch_target_base_excerpt.txt`, `pr1105.json`, `pr1106.json`, `pr1107.json`, `rr_cited_hashes.txt`, `sw_inputs_sha.txt`, `owner_quotes_check.txt`, `public_meaning_check.txt`, `heading_citations_check.txt` and `ids_check.txt`.

### 4.1 #1106 → `025c1cf326` at `b4f22e6ce7`, with its gates: confirmed

**GitHub and Git:**
- #1106 is MERGED at 03:00:18Z. Its merge commit is `025c1cf326`, with parents `47a3bdfcf5` and `b4f22e6ce7` (= `PARENTS.txt`). The base equals the first parent, so main had not moved.
- `MERGE_COMMAND.txt` reads `gh pr ready`, then `--merge --match-head-commit b4f22e6c…`.
- **Main's diff `M^1..M`** is 7 files (5 A, 2 M), +974/−6, as RECORD.md says.
- **The 3 slice blobs** are equal at `0730c87aef`, NUM `9e09bc2a35` and main `025c1cf326`.

**CI on `b4f22e6ce7`** (= `CI_RUNS_b4f22e6ce7.txt`):
- the four automatic runs succeeded: 37555521517, 37555521474, 37555521509 and 37555521560;
- **dispatch 37555520168** succeeded. Its plan's `target_base` is the full `47a3bdfcf5a37e856465c45cf904383f10181498`, and its numerical cargo suite, 4 remainder shards and source-mode E2E all succeeded; accessibility was skipped by selection. PR_CUT.txt abbreviates the input (N-3).

**The other gates:**
- `se.txt` is 5/5 PASS, |S| = 3, INT `9e09bc2a35`.
- `citations.txt` is PASS with 1 resolved.
- `gen8.txt` gives the head, the command and 1 passed.
- RV104's ADDENDUM_01 and ADDENDUM_02 are CONFIRMED.

**DEC-025 against the host:**
- `meta.txt`, `quiet.log` and both suites logs are byte-identical to the host's `WT/scratch/u9_dec025/SI1b_b4f22e6ce7{,_base}/`. `surfaces.txt` differs only by the VENV placeholder.
- The tails equal the host's lines.
- **The counts:**
  - 38/40 identical;
  - `expression_evaluator` 50 → 58 (lib 49 → 57), and `rule_check_runner` 33 → 35;
  - pytest 3,773 passed and 32 skipped;
  - vitest 141 files, 3,612 tests;
  - `ALL-DONE` at 02:59:56Z.
- **The lock log** shows WAIT 01:07:40Z, START 01:24:12Z, CANCELLED 02:13:19Z, then restart START 02:13:58Z and END rc=0 02:59:56Z.

**The quiet-check deadlock (RR:13844–13858): confirmed.**
- The committed aborted quiet log equals the host's: 147 `busy=1` lines from 01:24:12Z to 02:13:01Z.
- The committed `run_dec025.sh.txt` (`025f3410…`) equals the live wrapper. Its only change from the `.bak_20261007` copy is `busy()`'s added exclusion of `/usr/bin/lockf ` waiters, plus the two comment lines.

**NUM absorbed main** as `35cf618558`. Its tree equals its first parent's, and its non-execution tree equals main's.

### 4.2 #1107 → `2007709549` at `1199726f69`: confirmed

**GitHub and Git:**
- #1107 is MERGED at 13:15:15Z. Its merge commit is `2007709549`, with parents `025c1cf326` and `1199726f69` (= `PARENTS.txt`). Main had not moved.
- **Main's diff `M^1..M`** is 15 files, +900/−71: the 11 slice files plus the 4 package files.
- **The slice:**
  - the 11 slice blobs at `1199726f69` equal `a7de2a918f`'s;
  - `a7de2a918f` is three commits over `bfb26596bf` (11 files, +535/−71);
  - it touches nothing in PP `src`, a D1 crate's `src` or the schemas, and none of main's later changes.
- **The integration:** B6 merged into NUM as `1732ba108e` (`--no-ff`), and `c698a0b9a5` adds exactly the 4 package files.
- **The corpus:** 07m is `c21112fd…`, and the carrier case file `98a7213a…`.

**CI on `1199726f69`** (= `CI_RUNS_1199726f69.txt`):
- The four automatic runs succeeded: 37620715065, 37620714745, 37620714751 and 37620714810.
- **37620732340 failed in "Select source coverage".**
  - Its plan has `"target_base": "025c1cf326"`.
  - `e2e_plan.py:282–283` raises "Manual target must be an immutable commit SHA" for a non-40-hex dispatch target. Lines 286–287 re-raise it as "Update the PR base: event target base is missing, …", which is what the log shows.
  - So RR:14115's account is exact: the failure came from ROOT's input, not from the PR.
- **37621653258 succeeded** with the full `025c1cf3…` target: the numerical cargo suite, 4 remainder shards and source-mode E2E; accessibility was skipped by selection.

**B6_MERGE's corrections match RV108's ADDENDUM_01** (`ad9caae0…`, 12/12):
- **A-N1 (the "twin" wording)** has the same content and the same G0–G2 qualification.
- **A-N2** has the same three missing items (SC's entries for N1, N2 and N4; SC's transport scope sentence; the Python docstring under SR-PY) and the same N5 and N7 gap. RR:14131 also records ADDENDUM_01's point that "TS's doc comment" becomes true only once SR-PY repairs N1.
- **A-N3** (`R/REVIEW_RV92/u6f_01` is not parsed by `check_citations.py`) has the same content.

**The other gates:**
- `se.txt` is 5/5 PASS, |S| = 11, INT `c698a0b9a5`.
- `citations.txt` is PASS, 7 resolved.
- `gen8.txt` shows 1 passed, but carries neither head nor command (N-3).

**DEC-025 against the host:**
- `meta.txt`, `quiet.log`, `compare.txt` and both suites logs are byte-identical to the host's. `surfaces.txt` differs only by VENV, as RR:14152 says.
- **The counts:**
  - 39/40 identical, and `result_export` 177 → 180;
  - pytest 3,799 passed and 32 skipped (+26);
  - vitest 3,620 (+13, −5, as RV108 counted);
  - `ALL-DONE` at 13:14:22Z.
- The lock log shows START 12:26:37Z and END rc=0 13:14:22Z.

**NUM absorbed main** as `f40d8b7ded`, with tree = first parent's and 0 non-execution paths differing from main. **NUM carries no unmerged product slice at N.**

### 4.3 #1105 → `47a3bdfcf5` (squash): confirmed

- **GitHub:** MERGED at 00:42:00Z from head `736f3fb7b2`.
- **The squash** has the single parent `bfb26596bf` (= `PARENTS.txt`). Its tree equals the head's and `030020aca3`'s (`6efac947ce…`).
- **It changes 218 A and 2 M**, with 0 paths outside `execution/`.
- **The commit body equals `SQUASH_BODY.txt`.** The file has one more trailing newline, which Git strips. It names the `dec025_mac` seal (RV106 N-5).
- **CI:** the four runs on `736f3fb7b2` succeeded, as `CI_RUNS_736f3fb7b2.txt` says.
- **GEN-8:** `gen8.txt` has the head and the command.
- **NUM absorbed main** as `1527594642`, with its tree equal to the first parent's (RR:13607).

### 4.4 The rulings since `030020aca3`

RR appends 21 sections (677 lines), from "#1105 opened; …" to "RV109 confirms ST's repair; …".
- **All 29 heading citations resolve** to existing RR headings: those in WG's rulings-in-force list and the quoted headings in the appended text (`heading_citations_check.txt`). The check's other 2 quotations are code text, not headings: DEC-022's header line and `e2e_plan.py`'s message.
- **All 21 return sha256 prefixes cited** in the appended text match the files in H (`rr_cited_hashes.txt`).

**The owner's decisions, quoted as recorded** (`owner_quotes_check.txt`). RR and WG carry the same words.
1. **RR:13694, the host.** **"you will be working on this macbook pro with 128 GB of ram, so if you need to allocated 32 GiB it will be available. If you need more than 64 GiB we should negotiate."** WG:589 quotes the part from "if you need".
2. **RR:13719 and RR:13721, M.** The owner first wrote **"oh then use 12 GiB as a practical limit"**, and then decided: **"use 12 GiB, target is 32 GB workstations but 16 GB workstations still solving within practical timeframes."** WG:588 has the same words.
3. **RR:14063–14064, SI1c.** **"D: block at overflow"** and **"Repair within 1.0.0"**. WG:587 has the same words.

These answer I87 §4.4's package (D recommended; "a repair within grammar 1.0.0" recommended). I cannot see the chat itself, so "as recorded" means as RR records them.

**The arithmetic:**
- 64 GiB = 68,719,476,736 B and 12 GiB = 12,884,901,888 B.
- **S3 at 10.5 GiB:**
  - 0.9 M = 10,146,860,236 B;
  - dense E+R 9,747,725,678 B = 0.8646 M, which is 399,134,558 B under;
  - the margin is 9.53 % of TAV_W (4,189,696,338 B);
  - sparse is 0.8594 M, at 10.94 %;
  - at 12 GiB the dense margin is 44.1 % of TAV_W;
  - the priced heap is 9,680,616,814 B = 9.02 GiB (E_mov,max, I82 ADDENDUM_01:115).
- I82's ADDENDUM_01 is scoped to ≤ 12 GiB, and records that the 64 GiB request was superseded before it priced anything.

**No owner-held decision was taken by ROOT.** Each owner-held item against WG:575–584:
- **M:**
  - P1 (5.25 GiB) was adopted under the 6.0 GiB grant.
  - Under the 64 GiB reading (`80e88bece4`, 19:24:46 local), ROOT selected nothing. It reopened the target, raised the product implication of a large M with the owner, and the owner's 12 GiB decision followed within five minutes (`c3556c3519`).
  - S3's 10.5 GiB lies within 12 GiB. Anything above 12 GiB stays the owner's (RR:13724), and RR:13769's fallback (I82's options, or P1) stays below it.
  - The 16 GiB `CAP_BYTES` (A1-S-1) is the challenge test's counting-allocator abort cap. It is not M, and it lies within the host-job allowance.
- **The supported-machine statement** stays owner-held. ROOT only recommended 8 or 12 GiB, and the owner chose.
- **The dense and lane ceilings** are explicitly kept owner-held (RR:13703, RR:13733).
- **PHYS-R4, observation framing, KF2, KF3 and the native-app witnesses** are untouched. I81's ruling 4 makes W6's PHYS-R4 input a `NoTriggeredCase` witness of T-4, which changes neither its publication nor its availability.
- **B2's combination ceiling** is recorded as "nothing is decided now" (RR:13788).
- **SI1c:**
  - Ruling 1 (RR:14042) correctly finds that options A, B and D turn published `USER_RULE_CHECKED`/`USER_RULE_FAILED` outcomes into `RULE_INPUTS_INCOMPLETE` on the panel, the status bar, the saved record and the export. So it sent decisions 2 and 3 to the owner.
  - ROOT's decisions 4–15 are each I87's ROOT-labelled recommendation.
  - Decision 6 (N4-1) changes findings, not status. I87 §4.1 and §5.1 classify it so, and this is the same line as SI1b's precedent and as B6's below.
  - Decision 5 defers any corpus extension to the owner.

### 4.5 B6's item-2 widening: I agree it does not change public meaning

**Why I agree:**
1. **Accept/refuse is unchanged.** RV108 ran its own 1,032 probes and the 339 07m entries through five entry points per language:
   - every TS admission at BASE is byte-identical at HEAD;
   - the only movement is G7 refusals whose code goes from `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` to a base header code (631 probes, 9 corpus entries);
   - no gate, detail, transport or route moves.
2. **It is decision 11's class.** B0 selected decision 11 (TS's G7 header code for N-3's class) as ROOT's, and RV105 and RV106 confirmed that deciders matched the owner-held list. The four sibling classes have N-3's root cause and site, and they move in the same direction.
3. **No new vocabulary appears.** Each new TS code is the one Rust and Python already return for the same input, except RV108's pre-existing N1 Python defect.
4. **Successors are not public before B8** (RR:12374). The F2a D1 milestone is in the registered dev/test build only, and no production path produces a retained successor.

**The caveat (N-1):** the TS code is shown verbatim on `HistoricalRunPanel` for a saved successor that fails revalidation. So the visible change is a diagnostic string on an input that stays refused, with standing unchanged. That is not a change of meaning, but the ruling should say why on grounds 2–4.

### 4.6 The B6, SW, ST and R3 rulings, and RV108's and RV109's findings

- **B6 (RR:13947–13979):**
  - the branch facts and corpus shas hold (§4.2);
  - RV78-N1 goes to B3 and is reflected in WG's B2/B3/B4 row;
  - the routing of I83's §7 items 6–8 matches I83's RETURN and WG:631.
- **SW (RR:13927–13945):**
  - the seven input files' sha256s in `R/I86/…/_run_records/inputs/` equal RR's (`89b05619`, `719acbb0`, `39c44bc3`, `544330c8`, `b6e8138f`, `d05b5996`; `sw_inputs_sha.txt`);
  - PROBE §5.2 gives the assembled request's hash, as RR says.
- **ST (RR:13997–14007):**
  - `a8e719f5b4` is two commits over `47a3bdfcf5` (`4a51783e65`, `a8e719f5b4`): 6 files, +402/−53, all in PP `src`.
  - The repair `98a77c716e` changes only `retained_facade_tests.rs` and `retained_memory_witness_tests.rs`, and PP `lib.rs` stays `84fba5ca…`.
  - **I1** is `262bd687f0`, with parents `98a77c716e` and M. Its maintained diff from M is exactly ST's 6 files, and `b1` and `b1-a` are both at I1.
- **R3 (RR:14009–14021):** rulings 1–5 answer I85 RETURN §9 items 2, 3, 6, 5 and 4 one for one.
- **RV108's findings** are routed as RR:14069–14079 says:
  - N1 and N2 → SR-PY;
  - N4 → SR-TS;
  - N3, N5 and SC's half of N6 → SC (TS's doc comment → SR-TS);
  - N7 needs no action.

  They match REVIEW.md's table (`d5ea4a1b…`, PASS 0/0/7).
- **RV109's findings** (`707045b9…`, PASS 0/1/5; ADDENDUM_01 `6c7a9f87…`, CONFIRMED 0/0/1):
  - SF-1, N-1 and N-4 go to the repair round (`B1_ST_REPAIR_01.md`);
  - N-2 (R17) → `B1_SP.md:16`;
  - N-3 → `B1_SA.md:17`;
  - N-5 → SQ, tracked in RR.
- **RV107:** ADDENDUM_01 is "Not confirmed as written: 0/2/11" and itself proposes ruling A1-S-1 and A1-S-2 into the slice briefs. `B1_COMMON.md:8–9` does so.
- **RV106's notes** are resolved as RR:13609–13619 says. WG's position date and SI1b row are updated (N-4), and the squash body names the seal (N-5).

### 4.7 WG's T3 section: confirmed except N-2

- **The positions (WG:548–558):**
  - SI1b is on main as #1106 `025c1cf326`, and B6 as #1107 `2007709549`;
  - the records are through #1105 `47a3bdfcf5`;
  - B0 is selected and PLAN_v2 accepted;
  - phase 1 is done, with ST at I1 and phase 2 running;
  - NUM carries main `2007709549` with no unmerged product slice.

  Each of these agrees with Git and GitHub.
- **The owner-held list** (WG:575–584) changes only its M line, to "the formal supported-machine statement (drafted at B7/B8 from measurement), or M above 12 GiB (ROOT may select M up to 12 GiB: owner, 2026-10-07; T3's host jobs may use up to 64 GiB on this Mac)". The dense and lane ceilings are still listed.
- **The owner decisions in force** (WG:586–598) add the three 2026-10-07 decisions with the owner's words as RR records them. The superseded entries are not marked (N-2).
- **The next unused IDs at N are I90 and RV110** (`ids_check.txt`):
  - I90 occurs only in RR:14181, WG:600 and WG:646 (SR-RS, forecast);
  - RV110 occurs only in the "next unused" lines, WG:647's forecast and I87 PLAN's forecast (`:582`, `:658`);
  - I91 and RV111 occur nowhere;
  - I88 and I89 are dispatched, with briefs in H and no record folders.
- **The next safe action (WG:645–649)** matches RR:14157–14179 and PLAN_v2's I2/R3′/I3 order:
  - SR-RS waits for the first of I88 or I89;
  - the records PR is item 3;
  - the cleanup list rightly includes the merged `records-pr`, `s-i1b`, `si1b-pr`, `b6` and `b6-pr`;
  - it rightly omits `records-pr-b` (#1108's checkout), `main-baseline`, `sweep-skewpin`, `b1`, `b1-a` and `s-i1c`.

  The one stale item is RV110 at WG:647 (N-2).

## 5. Integrity: PASS except S-1

**The sum files**, verified against a `git archive` of H (the in-scope T folders; `_run_records/sums_verify.py`, `sums_verify.tsv`). "Uncovered" counts files in the folder that none of its sum files lists.

| Folder (T-relative) | Sum file(s) | Result | Uncovered |
|---|---|---|---|
| `IMPLEMENTATION/SI1B/` (on main) | SHA256SUMS | 3/3 OK | 0 |
| `IMPLEMENTATION/SI1B_MERGE/` | SHA256SUMS | 17/17 OK | 0 |
| `IMPLEMENTATION/B6/` (on main) | SHA256SUMS | 3/3 OK | 0 |
| `IMPLEMENTATION/B6_MERGE/` | SHA256SUMS | 17/17 OK | 0 |
| `IMPLEMENTATION/RECORDS_MERGE_2026-10-07/` | SHA256SUMS | 5/5 OK | 0 |
| `IMPLEMENTATION/SESSION_2026-10-07/` | SHA256SUMS | 2/2 OK | 0 |
| `R/I79/si1b_01/` | SHA256SUMS; SHA256SUMS.repair_01 | 31/31; 13/13 OK | 0 |
| `R/I81/b1_probe_01/` | SHA256SUMS | 16/16 OK | 0 |
| `R/I82/b1_cap_study_01/` | SHA256SUMS; SHA256SUMS.addendum_01 | 34/34; 18/18 OK | 0 |
| `R/I83/b6_01/` | SHA256SUMS | 34/34 OK | 0 |
| `R/I84/b1_plan_01/` | SHA256SUMS; SHA256SUMS.v2 | 7/7; 10/10 OK | 0 |
| **`R/I85/b1_st_01/`** | SHA256SUMS; SHA256SUMS.repair_01 | **57/60; 33/34: 4 MISSING (S-1)** | 0 |
| `R/I86/b1_w_probe_01/` | SHA256SUMS | 83/83 OK | 0 |
| `R/I87/si1c_plan_01/` | SHA256SUMS | 5/5 OK | 0 |
| `R/REVIEW_RV104/si1b_01/` | SHA256SUMS; .addendum_01; .addendum_02 | 54/54; 22/22; 7/7 OK | 0 |
| `R/REVIEW_RV105/b0_01/` (on main) | SHA256SUMS; .addendum_01 | 9/9; 2/2 OK | 0 |
| `R/REVIEW_RV106/records_01/` | SHA256SUMS | 34/34 OK | 0 |
| `R/REVIEW_RV107/b1_plan_01/` | SHA256SUMS; .addendum_01 | 4/4; 1/1 OK | 0 |
| `R/REVIEW_RV108/b6_01/` | SHA256SUMS; .addendum_01 | 78/78; 12/12 OK | 0 |
| `R/REVIEW_RV109/rvp_round1_01/` | SHA256SUMS; .addendum_01 | 96/96; 76/76 OK | 0 |

**Totals:**
- 30 sum files, 787 entries: **783 OK, 0 bad, 4 missing**.
- The 25 sum files this PR adds all lie in these folders.
- The other `*.sha256` files are run outputs (probe or mutant hashes), not seals.
- The 28 `.gz` files verify as compressed bytes in RV108's SHA256SUMS.

**RR is append-only: PASS** (`_run_records/rr_append_only.txt`).
- Main's RR (1,153,106 B, sha256 `34d1838a…`, 13,504 lines) is an **exact byte prefix** of H's (1,206,006 B, `2c84b787…`, 14,181 lines).
- H appends 52,900 B in 677 lines: 21 sections.
- H's blob equals N's.

**The 13 redacted originals: PASS** (`_run_records/redactions_check.py`, `redactions_check.txt`).
- For each `REDACTIONS.json` entry, the blob at `dfa5e2dc44` hashes to `original_sha256`, and H's blob hashes to `redacted_sha256`.
- **0 of the 13 original blobs** are in H's tree (66,356 distinct blobs) or in M's.
- 0 of the 768 changed files hash to an original.

## 6. Gate evidence: PASS (the records-only gate set)

The records-only gates are GEN-8, the PR's automatic CI and an independent review. This PR changes no product, test, CI or portability-policy path, and nothing under T's `REFERENCES/` or `DESIGN_NUMERICS/`.

| Gate | Evidence | Exact head? | Result |
|---|---|---|---|
| GEN-8 (ROOT) | Relayed at dispatch: 1 passed, 10 deselected, on H. It goes into the merge record | H | **pass**. The record should carry the head and the command (E-4; cf. N-3) |
| GEN-8 (RV110) | §3 | H | **pass** |
| governance-harness | 37632902364. It ran on the pull_request merge ref `c691744a05`, whose parents are M and H, with `CHIRALITY_REQUIRE_LIVE_TESTS: 1`: "1156 passed, 48 subtests passed" | H | **success** |
| Harness Pre-merge Validation | 37632902530 (Select App coverage and Harness pre-merge succeeded; App jobs skipped by selection) | H | **success** |
| pec-tests | 37632902690 (Select PEC coverage and pec succeeded; workspace tests skipped by selection) | H | **success** |
| Piping Desktop E2E | 37632902263 (Select source coverage and Desktop E2E source mode succeeded; numerical, remainder and accessibility skipped by selection) | H | **success** |
| Independent review | this report | H | **PASS** (S-1 to be fixed) |

## What ROOT must rule on

1. **S-1, before the squash:**
   - `git add -f` I85's four `build/` files;
   - choose (a), a re-cut with my delta confirmation (the precedent), or (b), the next records PR;
   - add an RR erratum on RR:13997 and RR:14171;
   - re-apply the committed-tree check to returns.
2. **N-1:** optionally anchor B6's public-meaning ruling in RR:12374 and decision 11, and record that the code is visible on `HistoricalRunPanel`. Nothing needs undoing.
3. **N-2 and N-3** are for the next WG touch and RR append.
4. **N-4** is for the squash body.
5. **Keep `WT/records-pr-b`** (#1108's only checkout) out of any cleanup until #1108 merges, re-cut included.
6. **Before the merge:**
   - check that main is still `2007709549`;
   - run `gh pr merge 1108 --squash --match-head-commit <final head>` with an explicit subject and body.

## Host and method

- **My copy:** a `git archive` of H's in-scope T folders in `WT/rv110/` (15 MB, 1,168 files): `IMPLEMENTATION/{SI1B,SI1B_MERGE,B6,B6_MERGE,RECORDS_MERGE_2026-10-07,SESSION_2026-10-07}/` and R's BRIEFS, I79, I81–I87 and REVIEW_RV104–RV109. It was used for the sums, sizes and types, and is deleted at the end. Logs and scripts are in `WT/scratch/rv110_records_01/`, with `TMPDIR` there.
- **Reads:**
  - NUM's object store and working tree, with `GIT_OPTIONAL_LOCKS=0`: `log`, `diff`, `ls-tree`, `show`, `merge-base`, `grep`, `cat-file`, `check-ignore`, `ls-files`, `status` and `ls-remote`;
  - `gh` read-only, for the PRs, runs, jobs, job logs and #885's file list;
  - host files read-only: `WT/scratch/u9_dec025/` (SI1b's and B6's DEC-025 outputs, the wrapper and its backup) and `WT/guard/cargo_jobs.log`;
  - the four I85 `build/` files in NUM's working tree.
- **The one departure from "your copy".** GEN-8 ran in ROOT's PR worktree `WT/records-pr-b`, because E-4 needs a Git checkout of the exact head (the RV106, RV103 and RV102 precedent). It was clean, with HEAD unchanged, before and after. I wrote nothing there.
- **What I ran:** one invocation of the single GEN-8 pytest (in the foreground; no wait loop), plus read-only Python scans and hashing. No other test, cargo, native work, install or Git write. Nothing went to the system temp directory, and no process of mine remains.
- **The scripts:**
  - `publication_scan.py`, `token_shape_scan.py` and `abs_scan.py` are RV106's, changed only in their docstrings and to decompress `.gz` files;
  - `sums_verify.py` and `redactions_check.py` are RV106's, with only the docstring changed;
  - `gz_check.py` and `heading_citations_check.py` are new.

## Evidence index (`_run_records/`)

- **Scope:** `scope_checks.txt`, `changed_paths_name_status.tsv`, `pr1108_view.json` (identity e-mails dropped), `pr1108_body.md` and `open_prs.txt`.
- **GEN-8:** `gen8_pytest.log`.
- **Machine paths:** `abs_scan.py` and `abs_paths_by_file.tsv`.
- **Publication screen:** `publication_scan.py`, `publication_scan_summary.txt`, `publication_hits.tsv` (home roots written `/U-sers/`; owner e-mail written `<owner-email>`), `token_shape_scan.py`, `token_shape_scan.txt`, `gz_check.py`, `gz_check.txt` and `size_type_summary.txt`.
- **Sums:** `sums_verify.py`, `sums_verify.tsv` and `i85_uncommitted_files.txt`.
- **RR:** `rr_append_only.txt`, `heading_citations_check.py`, `heading_citations_check.txt`, `rr_cited_hashes.txt` and `owner_quotes_check.txt`.
- **The originals:** `redactions_check.py` and `redactions_check.txt`.
- **Living documents:** `living_docs_git_checks.txt`, `ids_check.txt`, `sw_inputs_sha.txt`, `public_meaning_check.txt`, `pr1105.json`, `pr1106.json` and `pr1107.json`.
- **DEC-025:** `dec025_host_vs_committed.txt` and `dec025_counts.txt`.
- **CI:** `ci_runs_H.json`, `ci_jobs_H.txt`, `ci_governance_harness_H_excerpt.txt`, `ci_dispatches.txt` and `ci_dispatch_target_base_excerpt.txt`.
