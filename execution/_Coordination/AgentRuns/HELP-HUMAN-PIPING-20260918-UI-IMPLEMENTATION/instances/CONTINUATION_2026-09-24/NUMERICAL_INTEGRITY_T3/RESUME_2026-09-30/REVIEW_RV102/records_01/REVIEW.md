# RV102: independent review of the T3 records-only PR after #1100 (#1101)

**Reviewer:** RV102, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of these records.

**Brief:** `R/BRIEFS/RV102_RECORDS_PR3_REVIEW.md`, read in full, with the repository root `AGENTS.md`, `agents/AGENT_TASK.md` and `P/AGENTS.md`. Precedents read first: RV96's `REVIEW.md` (#1084), RV100's `REVIEW.md` and `ADDENDUM_01.md` (#1088), and RR's rulings "Owner direction: proportionate CI…", "RV100 passes #1088 at H…" (E-4) and "T3's gate set and Git rules, consolidated after the handoff was made ephemeral".

**Placeholders.** WT = the t3 workspace; NUM = WT/numerics; P = `projects/chirality-piping`; T3 = P/execution/…/NUMERICAL_INTEGRITY_T3; R = T3/RESUME_2026-09-30; RR = T3/ROOT_RULINGS_V1.md; WG = P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md; D2 = T3/DESIGN_STANDING/DESIGN.md; VENV = the repository venv's Python 3.13.14.

## The candidate

| Item | Value |
|---|---|
| PR | #1101, `codex/piping-t3-records-20261006` → `main`, ready (not draft), MERGEABLE / CLEAN at 15:23Z |
| H (first head, dispatched) | `11b2d04f13f48ce7821aa08d0261318fadcb1d09`, one commit, sole parent M |
| **H2 (current head, relayed mid-review)** | **`e41566921d5a1edd2fd4bb373ad2cbb40c7de0fa`**, parent H. It removes `T3/IMPLEMENTATION/U8/` (4 files) under RV97's A1-S-1 |
| M (base, main) | `75a8c3291ffe5dad1bbd2833bcc8b9d501274bf0` (#1100's merge); origin's main is still M at 15:23Z |
| N (source, NUM) | `4e6c2fcbfed4472ed3272f94632c0dada5f0596a` |
| Author and committer | the owner's configured Git identity on both commits; the only other identity is the agent co-author trailer |

I reviewed H in full and then H2. H2's tree is H's minus four files, so every check on H's content carries to H2. The checks that depend on the head itself (scope, GEN-8, CI, the originals) were repeated on H2.

## Verdict: **PASS** at H2 (no blocking finding)

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 2 |
| NOTE | 6 |

The following pass:
- **Scope.** H2 is M plus two commits and nothing from NUM's history. It changes 708 added and 3 modified paths, all under `P/execution/`, with no deletion. H2's tree is exactly N's minus `T3/IMPLEMENTATION/U8/`, which is the one deliberate exclusion.
- **Publication screen.** There are no credentials, personal data, whole-host data, binaries or large files.
- **Portability.** GEN-8 passes on H and on H2. The changed files hold 0 machine-absolute paths in added text, and the living documents hold none.
- **Integrity.** RR is append-only, and none of the 13 redacted originals is in H2.
- **Gates.** The four automatic runs succeeded on H2. My GEN-8 run and ROOT's both pass on the exact head.

Two things need fixing:
- **S-1:** one sealed folder (RV98's) does not verify from Git. Four of its evidence files are Git-ignored.
- **S-2:** the work graph's T3 "Position" and "Next safe action" are stale at N.

Neither blocks a records merge. ROOT should rule whether they are fixed by a further commit on #1101 or in the next records PR (§ "What ROOT must rule on").

## Findings

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | `R/REVIEW_RV98/u8_passb_01/` (its `SHA256SUMS` lines for `evidence/build/*.txt`) | From the PR tree, **43 of 47 entries verify and 4 are missing**: `evidence/build/{build_inputs,gate_checks_on_i72_outputs,law_outcomes_vs_F,witness_compare}.txt`. The cause is `P/.gitignore:20` (`build/`), which ignores them, so they were never committed: 0 are tracked at N, at H2 or at NUM's current head. On NUM's disk they exist, their hashes match the sealed ones, and the folder verifies 47/47 there. RR's "SHA256SUMS 47/47 OK" (ruling "RV98 confirms U8's Pass B…") is true of the host only. RV98's `REVIEW.md` relies on them as evidence (§2, and lines 81, 108, 202, 245). Unlike RV96's N-4 (`.pyc` bytecode), they are not regenerable output. They are plain text with no machine paths (`_run_records/rv98_untracked_build_files.txt`). | On NUM, force-add the four explicit paths (`git add -f`). Their bytes must match the sealed hashes `7a47f90f…`, `c1e299af…`, `0734a6c8…` and `791671be…`. Record the omission in RR's next append. Return verification should then check sums against the committed tree (`git ls-files` or a `git archive`), not the folder on disk; a name matching `.gitignore` (`build/`, `target/`, `dist/`, `node_modules/`, `__pycache__/`) is the risk. Then either add the files to #1101 by a further commit, which I can confirm as a delta, or carry them in the next records PR. |
| S-2 | SHOULD-FIX | WG, T3 section, "Position (2026-10-06 UTC)" and "Next safe action" | **Both are stale at N, and they contradict the section's own table and RR:**<br>- "It absorbed main `c1bfc460fc` as `b1e2d7741e`". NUM has since absorbed `c1571f7feb` (`56196df010`), merged S-I1 (`66adc78f84`) and absorbed M (N itself).<br>- "**Running:** I68 Part 1 (U8's probe), I73 (S-I1) and I74 (the T6 slice plan)". All three returned and were ruled (RR "I68's probe verified…", "I73's checkpoint 1 and I74's plan ruled…", "S-I1 committed…", "S-I1's repair round committed…"). The table's own rows say S-I1 is MERGED and U8 complete. At N the only running TASK is RV101 (RR "T6S complete; RV101 dispatched"; the T6S row).<br>- Next action 1, "Verify each return (RV99's confirmation; RV101)". RV99's confirmations are done (RR "RV99 confirms S-I1's repairs…", "RV99 confirms #1100's amended head…").<br>- Next action 5, "Absorb main (#1098) into NUM at the next records commit". This was done at `56196df010`, and N then absorbed #1100.<br>The section is T3's designated current account ("kept up to date as T3 moves"; it replaced ROOT_CURRENT), so a resuming ROOT would read these lines as current. | On NUM, restate Position and Next safe action at the current state: NUM = main plus records; running: RV101 (and whatever is current then); next: this records PR, U8's PR #1102, T6S after RV101, T3-SI1b ready, B0 at U8's merge. Carry the fix with S-1. |
| N-1 | NOTE | WG, T3 section, "Assignment IDs" | It says "I68–I74 and RV97–RV99 **are prepared** for fresh instances" and lists only I68, I73, I74, I75, I76, RV101 and I77 as dispatched. I69–I72 and RV97–RV99 were dispatched and returned too (RR "U8-1 committed…", "I69's corpus 07l committed…", "I71's TS alignment committed…", "U8 Pass B returned…", "S-I1 committed; RV99 dispatched"). **The next unused IDs, I78 and RV102, are correct** (§4). | Reword at the next WG touch: "I68–I77 and RV97–RV101 are used". |
| N-2 | NOTE | WG, T3 section, "T3 rulings in force" | **The list omits this session's standing rules.** Some appear elsewhere in the section, but this list is the index a resuming ROOT reads. The missing rules are:<br>- the T3 cargo lock, with every cargo job and ROOT's DEC-025 under it (RR "Session resumed…", and the lock split in "I73's checkpoint 1…");<br>- the operating adjustments (RR "RV98 confirms U8's Pass B; operating adjustments…"): returns closed before a turn ends, the walking skeleton, the merge order S-I1 → U8 → T6S, at most two or three implementers, and heavy vitest and pytest under the lock;<br>- NUM sequencing for product PRs (RR "U8's full suite passes…");<br>- N-12's carry-over wording (RR "RV99 confirms #1100's amended head…");<br>- I74's 14 decisions and D2 5b.3 (RR "I73's checkpoint 1 and I74's plan ruled…"). | Add the five headings at the next WG touch. |
| N-3 | NOTE | RR:12843 and RR:12890 ("I77's package is `IMPLEMENTATION/U8/`" with `fd5e4d49…`, `143ee2fa…`, `ffa40481…`); WG:556 (the T3-U8 row: "PR package ready (`IMPLEMENTATION/U8/`, I77)") | **These follow from H2's exclusion.**<br>- At H2, `IMPLEMENTATION/U8/` is absent, so once #1101 merges these references name a folder that main lacks until #1102 merges.<br>- #1102 then brings a different package: CHANGE_RECORD `aee05a80…`, PR_BODY `6c6827f4…` and SHA256SUMS `7af69486…`. Only citations.json (`ffa40481…`) is unchanged. RR's cited hashes are I77's PENDING draft, which stays reachable on origin at NUM N and at #1101's first commit H, but not on main after the squash.<br>- **The exclusion's ruling and RV97's A1-S-1 are not in H2.** They are on NUM after N (`0bd46c5f8c`).<br>The exclusion itself is sound. #1102 adds the same four paths (`_run_records/scope_checks.txt`), so keeping them would conflict, and the draft carried PENDING rows (10 in CHANGE_RECORD and 9 in PR_BODY). | The next records PR carries the A1-S-1 ruling. Its RR append states that RR:12890's hashes are I77's draft at NUM `4e6c2fcbfe`, superseded by #1102's package, and the WG U8 row points to #1102. No change to #1101 is needed. |
| N-4 | NOTE | PR #1101 description (at H2) and H's commit message | The description says the PR carries "the S-I1 evidence package", and H's message says "the U8 and S-I1 packages". **S-I1's package (`IMPLEMENTATION/S_I1/`, 4 files) is already on main through #1100,** and #1101 adds none of it. It carries S-I1's merge record (`IMPLEMENTATION/S_I1_MERGE/`) and S-I1's TASK and review records. The rest of the description is accurate at H2: 708 added, 3 modified, U8 left to #1102, and the gates. | Reword in the explicit squash body ("S-I1's merge record and gate evidence"). The branch messages do not reach main. |
| N-5 | NOTE | `R/REVIEW_RV97/u8_01/evidence/mutants/mutants.json` (sealed) | **8 truncated cargo-warning snippets show home-relative paths** of the form `~/dev/chirality/.claude/worktrees/…`. Neither GEN-8's detector nor the broad pattern flags them, and they carry no user name, only the host's directory layout. Main already carries that layout in `/Users/<user>`-form paths in thousands of records (RV96 N-1). | None required. Future TASK outputs can write the WT placeholder. |
| N-6 | NOTE | WG:35, the T3 row in the route table (unchanged from main) | The row still reads "T3's records through the 2026-10-05 handoff reached main in #1084 and #1088" and "Stays closed after #1082: … U8, wider F2a, S-I, F2b and F3". S-I1 is now on main (#1100). The row defers to the T3 section for current state, so it holds as history, but "stays closed … S-I" now reads against #1100. | Optional: one clause at the next WG touch. |

## 1. Scope: PASS

**Parentage** (`_run_records/scope_checks.txt`):
- H2's parent is H, and H's is M, so `rev-list --count M..H2` = 2.
- None of NUM's 727 commits in `M..N` is an ancestor of H2. None of #1084's old heads (`dfa5e2dc44`, `59b72619fe`, `3798d5eca7`, `5758e1c3df`) or `29160bbc1c` is either.
- The squash will put one commit on main. The relay notes the two-commit branch, which matches the ruled records-PR method.

**Changed paths** (`git diff --no-renames --name-status M H2`):
- There are **711 paths: 708 A, 3 M, 0 D**, all under `P/execution/`. The non-execution diff is empty (0 paths).
- Modes: 709 regular and 2 executable (RV99's `round1/tools/run_all.sh` and `tools/rc.sh`). There are no symlinks.
- At H the count was 712 A, 3 M, 0 D. ROOT's screen ("712 added, 3 modified, 0 deleted, nothing outside `execution/`") is **confirmed for H**, and the relay's "708 added" for H2.

**Equality with NUM:**
- At H, **the whole tree equals N's**: tree `b2ac23f7bef0…`, with `P/execution` = `d71855b0d602…` on both sides.
- At H2, `git diff N H2` gives exactly four deletions, `T3/IMPLEMENTATION/U8/{CHANGE_RECORD.md, PR_BODY.md, SHA256SUMS, citations.json}`, and nothing else.
- So H2 = N minus `IMPLEMENTATION/U8/`, the single deliberate exclusion. #1102 (`b18dd4f369`) adds the same four paths, which confirms the premise (N-3).

**The modified paths** (main blob → head blob, identical at H and H2):

| Path | Blobs | Main's blob on NUM's first-parent line | NUM commits since |
|---|---|---|---|
| D2 | `ef6ff10242` → `52c51711b9` (+8/−4) | `7004eaeda3` | 1 (5b.3) |
| RR | `38a10296e7` → `b65adaf62c` (+733/−0) | `1c00d217fc` (the LOOP_INIT tranche commit) | 25 |
| WG | `1cca06c0b3` → `ad8031255e` | `1c00d217fc` | 15 |

For each file, main's exact blob sits on NUM's first-parent line, only NUM commits change it afterwards, and no merge of main into NUM touched it. **No change of main's is reverted.**

## 2. Nothing that must not be published: PASS (N-5)

The screen covers **the text the PR adds**: every line of each added file, plus the `+` lines of the three modified files, 119,027 lines at H, a superset of H2's. The scripts and summaries are `_run_records/publication_scan.py`, `publication_scan_summary.txt` and `publication_hits.tsv`.

**Credentials:**
- **0 hits** for `ghp_`, `gho_`, `ghs_`/`ghu_`/`ghr_`, `github_pat_`, `sk-` (any form), `AKIA`, `BEGIN .*PRIVATE KEY`, `password`, `token=`, `Authorization:`, `Bearer <token>`, Slack tokens, and credential variables (`GH_TOKEN`, `GITHUB_TOKEN`, `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `AWS_SECRET`, `SSH_AUTH_SOCK`, `NPM_TOKEN`).
- `secret` has 71 hits in 11 files, all of them test identifiers (`test_secret_private_library_handling…`, `…secret_values_and_unknown_rights`, `…direct_sql_secret…`) in suite listings and one review's prose.

**Personal data:**
- There is **no e-mail address** in the added text, and the owner's name does not occur in it.
- The owner's configured identity appears only as the commits' author and committer.

**Whole-host data: 0 hits** for:
- application paths (`/Applications/`) and named non-toolchain apps (Teams, Parallels, ChatGPT, `Claude.app`, Messages, Mail, Slack, Chrome, Safari, Spotify, Dropbox, Docker and others);
- session or resume identifiers (`--resume`, `--session-id`, `session_id`, `user-data-dir`), UUIDs, agent or tool-call ids and transcript paths;
- system paths (`/System/Library`, `/usr/libexec`, `launchd`), host names (`MacBook`, `.local`), and process-table headers or rows.

`pgrep` occurs only as the memory-guard check in scripts, and not as a listing. The rulings' prose mentions the guard's PID and that "the host's Claude process … ended", with no identifier.

ROOT's screen is **confirmed**. The two RR lines that match host patterns are RR:11998 (`/Users/<user>`, a placeholder) and RR:12067 (S-2's redaction description, which names the redacted apps). Both lie in main's byte prefix (main's RR has 12,302 lines), so neither is new.

**Size and type:**
- The 712 added files at H total 13,294,977 B. The largest is 879,029 B (`R/I68/u8_probe_01/_run_records/probe_run2.log`), and **no file is near 50 MB**.
- `file --mime-type` finds only text: 531 plain, 111 JSON, 22 shell, 16 diff, 15 Python, 8 CSV, 8 Rust or TS sources read as C or Java, and one empty log.
- There are **no binaries or build outputs**: no `target/`, `.wasm`, `.rlib`, `.so`/`.dylib`, `.pyc` or `__pycache__`. The two `shared_node_modules_*.txt` files are listings, not modules.
- Seven added files have test-like names (`*.test.ts(x)`, `test_mutants.py`). Main already carries 174 such files under `execution/`, and hosted pytest collects `tools/` only.

## 3. Portability: PASS

**GEN-8 by E-4's method:** the brief's command, run in a Git checkout of the exact head. The only checkout of the head on this host is ROOT's PR worktree `WT/records-pr`. I ran it there read-only:
- `GIT_OPTIONAL_LOCKS=0`, `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`, and `TMPDIR` in my scratch folder;
- checked clean with HEAD at the head before and after (0 status entries and 0 ignored entries after);
- with VENV and pytest 9.1.1:

  ```
  VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8
  ```

The results:

| Head | Window (UTC) | Result | Log |
|---|---|---|---|
| H | 15:09:44–15:10:15Z | **1 passed, 10 deselected**, exit 0 | `_run_records/gen8_pytest.log` |
| H2 | 15:20:59–15:21:29Z | **1 passed, 10 deselected**, exit 0 | `_run_records/gen8_pytest_H2.log` |

In a real checkout `git ls-files` resolves to the PR's own tracked set, so this is the full walk, not RV96 N-6's subset. These were the only tests I ran.

**Machine-absolute paths** in the 715 changed files at H, a superset of H2's 711 (`_run_records/abs_scan.py`, `abs_paths_by_file.tsv`):
- **GEN-8's detector** (`surface_roles.iter_machine_path_lines`) finds 0 lines in every file.
- **A broader pattern** (the macOS user-home, private-temp, var-folders, tmp, Volumes and Linux home roots, and Windows drive letters; assembled at run time in `abs_scan.py`) finds one line, RR:11998, which is main's prefix and the `<user>` placeholder.
- **In the added text: 0.**
- **The living documents have none:** RR's appended 733 lines, the WG diff, D2's diff, and the five added briefs (`B0_CONTRACT_AND_IDENTITIES.md`, `I75_T6S_TS.md`, `I76_T6S_SCHEMA_TESTS.md`, `RV101_T6S_REVIEW.md`, `T6S_COMMON.md`).
- `T6S_COMMON.md` names its Node and VENV locations as `<repo>/.claude/worktrees/…`. That form is placeholder-anchored, not machine-absolute, and two of main's T3 handoff notes already use it.
- One sealed evidence file carries home-relative `~/…` paths (N-5).

## 4. The living documents against the records and Git (S-2, N-1 to N-4, N-6)

**Confirmed against Git and GitHub:**
- **#1100.** It merged at 14:59:35Z (GitHub `mergedAt`) as **`75a8c3291f`**, with parents `c1571f7feb` and `ef266247de`. The command was `--merge --match-head-commit ef266247de…` (`S_I1_MERGE/_run_records/`).
  - Its runs on `ef266247de` (governance-harness 37476323418 and dispatch 37476322784) and the earlier dispatch 37475441330 on `20e7e3e5a2` succeeded, all on the cited heads.
  - DEC-025's `meta.txt` reads `ALL-DONE` 14:58:43Z, at head `20e7e3e5a2`.
  - `compare.txt` reads 40 manifests, 38 identical, with `expression_evaluator` +18 and `rule_check_runner` +12 added and ok. pytest has 3,733 passed.
  - #1092 = `87661be164`, #1088 = `efca0cf6b6` and #1084 = `f506f3e2de`, as the records cite.
- **Main's moves**, as ruled:
  - `0d151c6469..c1bfc460fc`: 201 files, all app-v4;
  - `c1bfc460fc..0329f8fe6b` (#1098): 111 files, app-v4;
  - `0329f8fe6b..c1571f7feb` (#1099): app-v4 only;
  - `c1571f7feb..75a8c3291f`: S-I1's 8 maintained files plus its 4 package files.
  - `b1e2d7741e`'s tree equals `c1bfc460fc`'s.
- **The cited commits exist with the stated roles and diffs:**
  - U8: `d449097085` (+244/−0 plus the two fixtures), `69a925bd68` (07l +16,139/−1), `de01e43bc4` (+23/−6) and `bd6b4be2c3` (+45/−0). That makes 7 maintained files from `b1e2d7741e`.
  - S-I1: `4920e4b1b0` and `8f956d399a` (+2,197/−0 and +550/−3, five new files, and the 1/1 schema description), then `ea7c0881f4`, `c26ecabbc1`, `eddeab1e37`, `20e7e3e5a2` and `ef266247de`. `20e7e3e5a2..ef266247de` is exactly the 3 package files.
  - T6S: `055ee0c0bc` (dispatcher +2/−1,808) and `2033260c57`.
  - NUM: `56196df010`, `66adc78f84` (maintained diff from main = S-I1's 8 files), `377d1d5cfb`, `b9030f501c`, `f9657c51c5` and `fd3990a710`.
  - The U8 and T6S branches are not ancestors of N, and S-I1's are, which matches "NUM sequencing for the product PRs".
- **The next unused IDs at N are I78 and RV102.**
  - RR's last ID statement (the "I77 dispatched" ruling) and WG agree.
  - The highest records folders at N are I77 and REVIEW_RV100. RV101 has no folder yet, being in progress.
  - Nothing in T3 or WG uses I78, RV102 or higher.
- **The owner decisions in force:** G10's redefinition (2026-10-06) is in RR, with the owner's quoted words, and in WG's decision list, G10 row and B8 row. The halves are split as RR rules them.
  - G10 keeps the ordinary-route half and stays OUTSTANDING under the 2026-10-04 decision.
  - The successor-panel witness moves to B8's native Current witness.
  - R-2's stress-neutral readiness choice is listed as prepared for B8 and owner-held.
- **D2's revision 5b.3 matches the ruling "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; …".**
  - The title and the narrow-revisions note read 5b.3, and §0.5's table gains a 5b.3 row citing `CHECKPOINT_1.md` (`b992efcf…`, verified) and the ruling.
  - §4.11.3's `interpolate` row is rewritten as per-segment interval evaluation of the point path's own chain, stepped outward and joined. The old rule is kept in an italic note as unsound.
  - The "Eager U (revision 5b.3)" note lists exactly RR's five causes and keeps Kleene logic for straddling comparisons.
  - §4.11.4's lemma drops "linear interpolation segments" from the monotone pieces and adds the chain sentence.
  - No other line of D2 changes.
  - The counterexample phrase ("reads T where the point path fails") is supported by I73's `CHECKPOINT_1.md` §6.1: the literal hull [7999.999…, 5000003584.000001] against 0.0 at next_down(1).
- **Every record and brief sha256 that RR's new sections cite matches** the file at H: 34 citations, 25 records and the 9 briefs dispatched this session. The cited product-file hashes (fixtures, goldens, sources) were not re-derived.
- **The SUMS results RR records match Git**, with S-1's exception.
- **The host-tool copies** `SESSION_2026-10-06/host_tools/t3_cargo{,_run}.sh.txt` equal `WT/tools/` except for the root line, which becomes `T3_WT`.

**Not supported or stale:**
- S-2: WG's Position and Next safe action.
- N-1: WG's ID line.
- N-2: the rulings-in-force list.
- N-3: the U8 package references after H2.
- N-4: the PR description's "S-I1 evidence package".
- N-6: the T3 route row.

I found no contradiction within RR's appended sections.

## 5. Integrity: PASS except S-1

**SHA256SUMS:** every sums file this PR adds, verified against the `git archive` copy of H (`_run_records/sums_verify.py`, `sums_verify.tsv`). The coverage check counts files in each folder that its sums files do not list.

| Folder (T3-relative) | Sums file(s) | Result | Uncovered |
|---|---|---|---|
| `R/I68/u8_probe_01/` | SHA256SUMS `7a8751f7…` | 22/22 OK | 0 |
| `R/I68/u8_witnesses_01/` | SHA256SUMS | 38/38 OK | 0 |
| `R/I69/u8_corpus_07l_01/` | SHA256SUMS | 29/29 OK | 0 |
| `R/I70/u8_rust_07l_01/` | SHA256SUMS | 41/41 OK | 0 |
| `R/I71/u8_ts_07l_01/` | SHA256SUMS `adcabb06…` | 54/54 OK | 0 |
| `R/I72/u8_passb_01/` | SHA256SUMS | 47/47 OK | 0 |
| `R/I73/s_i1_01/` | SHA256SUMS; `.repair_01` `73644b97…` | 56/56; 29/29 OK | 0 |
| `R/I74/t6_slice_plan_01/` | SHA256SUMS | 1/1 OK | 0 |
| `R/I75/t6s_01/` | SHA256SUMS `4118a5dd…` | 56/56 OK | 0 |
| `R/I76/t6s_01/` | SHA256SUMS | 51/51 OK | 0 |
| `R/I77/u8_package_01/` | SHA256SUMS | 23/23 OK | 0 |
| `R/REVIEW_RV97/u8_01/` | SHA256SUMS `a6d923fb…` | 69/69 OK | 0 |
| **`R/REVIEW_RV98/u8_passb_01/`** | SHA256SUMS | **43 OK, 4 MISSING (S-1)** | 0 |
| `R/REVIEW_RV99/s_i1_01/` | SHA256SUMS; `.addendum_01`; `_02`; `_03` | 45/45; 35/35; 7/7; 11/11 OK | 0 |
| `IMPLEMENTATION/U8/` | SHA256SUMS | 3/3 OK at H. **Not in H2**; its completed version travels with #1102 | — |
| `IMPLEMENTATION/U8_GATES/full_suite/` | SHA256SUMS | 4/4 OK | 0 |
| `IMPLEMENTATION/S_I1_MERGE/` | SHA256SUMS | 19/19 OK | 0 |
| `IMPLEMENTATION/SESSION_2026-10-06/` | SHA256SUMS | 2/2 OK | 0 |

`IMPLEMENTATION/S_I1/` is not in this PR: its 4 files reached main with #1100. Every listed folder is byte-identical at H2 except `IMPLEMENTATION/U8/`.

**RR is append-only: PASS** (`_run_records/rr_append_only.txt`).
- Main's RR (1,050,478 B, sha256 `265d67da…`, 12,302 lines) is an **exact byte prefix** of the head's (1,114,766 B, `67d438b4…`, 13,035 lines).
- The head appends 64,288 B in 733 lines, from "Session resumed; main absorbed; …" through "#1100 merged: S-I1 is on main".
- RR's blob (`b65adaf62c`) is the same at H and H2.

**The 13 redacted originals: PASS** (`_run_records/redactions_check.txt`).
- For each REDACTIONS.json entry, the blob at `dfa5e2dc44` has sha256 = `original_sha256`, and the head's blob has sha256 = `redacted_sha256`, equal to main's.
- **0 of the 13 original blob ids** are in H's full tree (80,440 entries), in H2's (80,436) or in M's (79,728).
- No changed file's content hash equals an original (0/715).

## 6. Gate evidence: PASS (records-only gate set)

The records-only gates are GEN-8, the PR's automatic CI and an independent review ("Owner direction: proportionate CI…"; A-1 in "#1088 squash-merged…"). This PR changes no portability policy and no path under T3's `REFERENCES/` or `DESIGN_NUMERICS/`. No test reads its changed paths at a moving revision; `capability_inventory.json` pins WG at a fixed revision. DEC-025 and the dispatch are not gates for it.

| Gate | Evidence | Exact head? | Result |
|---|---|---|---|
| GEN-8 (ROOT) | `WT/scratch/records_pr_20261006/gen8.txt`: head `11b2d04f13…`, command, "cwd: records-PR checkout root", "1 passed, 10 deselected in 33.09s". `gen8_2.txt`: head `e41566921d…`, "1 passed, 10 deselected in 29.97s" | H; H2 | **pass**. Both record the head SHA and the command, as E-4 requires |
| GEN-8 (RV102) | §3 | H; H2 | **pass** |
| governance-harness | 37484843634 on H; **37486604804 on H2**. pull_request on the merge ref (`438dd8759` = H2 into M, whose tree is H2's), `CHIRALITY_REQUIRE_LIVE_TESTS: 1`, "1156 passed, 48 subtests passed" | H; H2 | **success** |
| Harness Pre-merge Validation | 37484843578; **37486605024** | H; H2 | **success** |
| pec-tests | 37484843572; **37486605017** | H; H2 | **success** |
| Piping Desktop E2E | 37484843883; **37486604927**. Select source coverage and Desktop E2E (source mode) succeeded; the numerical, source-remainder and accessibility jobs were skipped by selection | H; H2 | **success** |
| Independent review | this report | H2 | **PASS** |

At 15:23Z the PR was MERGEABLE/CLEAN, and origin's main was M.

## What ROOT must rule on

1. **S-1 (RV98's four Git-ignored evidence files).** Fix on NUM with `git add -f` and explicit paths. Then choose:
   - (a) a further commit on #1101 adding the four files and the S-2 wording, which I confirm as a delta; or
   - (b) merge #1101 as is and carry the fix in the next records PR.

   Under (b), main briefly holds a sealed folder that verifies 43/47. I lean to (a), because it is four text files and one WG paragraph, but either is defensible. Also consider adding "verify sums from the committed tree" to the return-verification rule.
2. **S-2 (WG Position and Next safe action stale).** Fix on NUM, travelling with S-1's choice.
3. **The rest are wording** for the next WG touch or RR append: N-1, N-2, N-3 (record the A1-S-1 exclusion and the draft-versus-#1102 hashes), N-6. N-4 is the squash body, before merging.
4. **Before the merge:** check that main is still M, then `gh pr merge 1101 --squash --match-head-commit e41566921d5a1edd2fd4bb373ad2cbb40c7de0fa`, or the later head if (a) is chosen, with an explicit subject and body.

## Host and method

- **My copy:** a `git archive` of H in `WT/rv102/` (2.9 G), used for the sums, scans and size checks, and deleted at the end. H2 = H minus four files, so it needed no second copy. Logs and scripts are in `WT/scratch/rv102_records_01/`, with `TMPDIR` there.
- **Reads:**
  - NUM's object store and working tree with `GIT_OPTIONAL_LOCKS=0` (`log`, `diff`, `ls-tree`, `show`, `check-ignore`, `status`, and `ls-remote` to origin);
  - `gh` for the PRs, runs, job lists and the governance-harness log.
- **The one departure from the brief's "your copy" wording.** GEN-8 ran in ROOT's PR worktree, not in the archive copy. E-4 requires a Git checkout of the exact head, and in an archive under WT, Git resolves to the enclosing repository (RV96 N-6, RV100 N-4). The run was read-only: the worktree was clean, with HEAD unchanged before and after both runs. I wrote nothing there.
- **What I ran:** two invocations of the single GEN-8 pytest, one per head (the second after ROOT's relay of H2), plus read-only Python scans and hashing.
- **Not run:** no other test, cargo, native work, install or Git write.
- **The host.** RV101's vitest mutant loop held the T3 lock while I worked, and ROOT's DEC-025 for U8 was waiting on it. GEN-8 is a 30-second read-only scan that I ran outside the lock, as the dispatch allowed. Nothing of mine went to the system temp directory.

## Evidence index (`_run_records/`)

- `scope_checks.txt`, `changed_paths_name_status_H.tsv` and `changed_paths_name_status_H2.tsv`: scope, ancestry, the exclusion and #1102's paths.
- `gen8_pytest.log` and `gen8_pytest_H2.log`: GEN-8.
- `abs_scan.py` and `abs_paths_by_file.tsv`: machine paths.
- `publication_scan.py`, `publication_scan_summary.txt` and `publication_hits.tsv`: the publication screen.
- `sums_verify.py` and `sums_verify.tsv`: SHA256SUMS and coverage.
- `rv98_untracked_build_files.txt`: S-1.
- `rr_append_only.txt`: RR is append-only.
- `redactions_check.py` and `redactions_check.txt`: the 13 originals.
- `ci_runs_H.json`, `ci_runs_H2.json`, `ci_jobs_H.txt`, `ci_jobs_H2.txt`, `ci_governance_harness_H_excerpt.txt` and `ci_governance_harness_H2_excerpt.txt`: CI.
