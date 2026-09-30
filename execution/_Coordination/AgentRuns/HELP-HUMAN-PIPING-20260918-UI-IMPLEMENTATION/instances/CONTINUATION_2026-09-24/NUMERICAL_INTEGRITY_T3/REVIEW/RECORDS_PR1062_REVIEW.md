# RV25: independent review of records PR #1062

- **Reviewer:** RV25, a Type 2 TASK (independent reviewer).
- **PR:** https://github.com/sgttomas/chirality/pull/1062, branch `codex/piping-t3-records-20260930`.
- **Head reviewed:** `b0e3579856fbf7bb4c6a68e8b2c46a060231e037`. Base main `7ad3a9adf`, which is also the merge base.
- **Date:** 2026-09-30.
- **Verdict: PASS.** There are no BLOCKING findings. There are 2 SHOULD-FIX findings and 10 NOTEs.

## Summary

- **Scope is records only.** All 1,031 changed paths are under `projects/chirality-piping/execution/_Coordination/`: 1,023 added and 8 modified, with none deleted or renamed. There is no code, test, manifest, `tools/` or `.github/` change, and no binary file.
- **Nothing on main is silently rewritten.**
  - `ROOT_RULINGS_V1.md` keeps main's 1,556 lines. The one exception is an insert-only bracket at :1400, which points to a real section. The 1,447 appended lines start at :1557.
  - `V4_VERIFICATION.md`, `K5_REVIEW.md` and `RECORDS_PR1049_REVIEW.md` begin with main's bytes and only append.
  - Each of the three modified SHA256SUMS files adds entries. The only hash that changes is `V4_VERIFICATION.md`'s, which was appended to.
  - The one in-place change that is not an insertion is to `WORK_GRAPH.md:62`. It applies RV16-D1's requested wording, and it is N1.
- **Every hash holds.**
  - All 25 SHA256SUMS files that cover a changed path verify at the head, run with `shasum -a 256 -c` from their own folders.
  - Each lists exactly its folder's tracked files. `_v4_records/` also lists the files that its nested `r3/`–`r7/` sums cover.
  - Main's unchanged `REVIEW/_run_records/SHA256SUMS` verifies 76/76, run from `REVIEW/` (N5).
- **No machine paths, host names or model identifiers leak.**
  - GEN-8's regex finds nothing in the PR's added lines.
  - Every broader or host-pattern hit is a placeholder (`<home>/…`) or a record that describes the scan patterns themselves.
  - **GEN-8 passes:** 1 passed, run in the numerics worktree at the head.
- **The nine merge records match GitHub and Git.** For each one I checked:
  - the PR, the merge SHA and time, `mergedBy`, the head and `--match-head-commit`;
  - that the merge's first parent is "Main was", which is an ancestor of the head;
  - every cited CI run's workflow, event, head and conclusion;
  - each dispatch's `target_base`, read from its run log.

  I re-derived every record's DEC-025 suite comparison from the committed `suites.log` files. The pytest and vitest counts, the build exits and the sanitized-sweep originals match. KF3's disclosed slip checks out exactly.
- **The rulings are mostly right, and the checked hashes all hold.** 25 cited sha256 prefixes and line counts, and more than 90 figures, match their sources. Three more restated figures are wrong or misattributed:
  - **S1:** KF3-B2's "19.5 MB" is presented as E_max's measured shortfall, but the source says the comparison is not like for like. It has spread into the K6c brief and the work graph.
  - **S2:** V-K B's heap and E_max are given in "GB" that are MiB/1,000. That is the E_adm slip again, and it has no bracket.
  - **N3:** 188 s is attributed to 1,000 members.

  The fourth error in handoff §7.2 was replaced in place, not bracketed (N2).
- **The work graph and the handoff agree with the records** on the PRs, SHAs, states, the remaining order and the owner decisions. The notes are about wording and pointers (N5–N8).

## Reviewer, brief and basis

- **Reviewer:** RV25, dispatched directly by ROOT (HELP_HUMAN) through the host's background-subagent mechanism. ROOT is my only return path.
  - I wrote none of these records and delegated nothing.
  - I made no Git writes and no index operations. I used only `git show`/`diff`/`log`/`rev-list`/`merge-base`/`patch-id` and `gh pr`/`run`/`api` reads.
  - No cargo was run.
- **Brief:** ROOT's dispatch message. Its seven priorities are the sections below. RV16's review of #1049 (`REVIEW/RECORDS_PR1049_REVIEW.md`) is the format.
- **Read:**
  - Root `AGENTS.md` and `agents/AGENT_TASK.md`;
  - `OPERATING_NOTES_FOR_LOCAL_ROOT.md` and `HANDOFF_2026-09-28_TO_LOCAL.md` §6;
  - RV16's review;
  - the PR body;
  - all 1,447 appended lines of `ROOT_RULINGS_V1.md`;
  - the nine merge records;
  - `HANDOFF_2026-09-30_AUDIT_PAUSE.md`;
  - the work graph's changed lines;
  - `I21_K6C_IMPLEMENTATION.md`;
  - the sources each sampled figure cites: RETURNs, reviews, `dec025/` files, plans, `V4_VERIFICATION.md` and `_run_records/`.
- **Paths:** `T3/` is `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`. `V1` is `T3/ROOT_RULINGS_V1.md`. `WG` is the work graph. `<wt>` is the T3 worktrees root.
- **Scripts and outputs:** `T3/REVIEW/_run_records/records_pr1062_review/` (README.txt, SHA256SUMS).

## 1. Scope (PASS)

- **The diff:** `git diff --name-status 7ad3a9adf b0e357985` has 1,023 A and 8 M, with 480,459 insertions and 5 deletions.
  - Every path is under `_Coordination/`: 1,030 under `T3/`, plus `WG`.
  - Modes: 1,017 files at 100644, and 6 `*.sh.txt` record copies at 100755 (N9). No binaries.
- **The eight modified files are exactly the PR body's list.**
- **The head's merge of main, `b0e357985`,** has parents `07fb44d56` (numerics) and `7ad3a9adf` (main); the merge base is `0256decc6` (#1049).
  - It adds exactly main's delta and keeps exactly numerics' delta: the diffs hash equal. The one exception is `IMPLEMENTATION/K6/PLAN_CHECKPOINT0.md`, which both sides added with identical bytes (`4b4d9b27…`).
  - `--remerge-diff` is empty.
- **Hosted CI on the head:** 7 success and 6 skipped (runs 36677114375, 36677114483, 36677114522 and 36677114545), the records-only selection, as for #1049.

## 2. Append-only (PASS; N1)

Checked with `append_only.py.txt` (difflib line opcodes, and a character-subsequence test on each replaced line).

| File (main → head lines) | Result |
|---|---|
| `V1` (1,556 → 3,003) | **One replaced line, :1400, insert-only.** It adds "[Superseded: RV17-1 showed that exact-block does select range-triggered cases (CX-F, CX-G), so the next sentence is false. See "F1b: rulings on RV17's review".]". The target exists at :1706, and RV17-1 says exactly that (`REVIEW/F1B_REVIEW.md:19`, `:66`). Then 1,447 appended lines (:1557–:3003). |
| `V4_VERIFICATION.md` (410 → 921) | Head starts with main's bytes; 511 lines appended. |
| `K5_REVIEW.md` (424 → 493) | Head starts with main's bytes; 69 lines appended. |
| `RECORDS_PR1049_REVIEW.md` (289 → 365) | Head starts with main's bytes; 76 lines appended. |
| `REV_5A3_CANDIDATE/SHA256SUMS` (105 → 179) | 0 removed, 74 added. One hash changed: `V4_VERIFICATION.md` (`ed2031c3…` → `f3bd1204…`), which was appended to. |
| `_v4_records/SHA256SUMS` (83 → 164) | 0 removed, 81 added, 0 changed. |
| `k5_review/SHA256SUMS` (163 → 184) | 0 removed, 21 added, 0 changed. |
| `WG` (416 → 416) | **:38 and :41: insert-only** (the KF2 T6 and T9 notes). **:62: not insert-only.** "…, and DS1 is writing R4." became "…; DS1's R4 is committed (`c644b751d`), and V4 is checking it." That is RV16-D1's own proposed fix, made in `438f3c0ca`, with no bracket (N1). Everything else on :62 is insert-only, including the state paragraphs and the "[superseded by the audit-pause state that follows]" pointer. |

- **Inside the PR's own history,** five commits remove a `V1` line. Four of them turn the line into the same line plus a bracket:
  - `ed74fd813` (:1945);
  - `40e508008` (:2348);
  - `b60a2022a` (:2537);
  - `899f28965` (:2631).

  The fifth, `ffc489f52`, replaces a sentence of :2841 outright (N2).
- **Every other bracket in the appended text points somewhere real:**

  | Bracket | Target |
  |---|---|
  | :1945 | K4 RETURN §22.3 (:667), whose worst ratio of 1.5e-24 is at :672 |
  | :2243 | KF1's first commit `d0566126e` has parent `8cca91701`; #1055 is `chirality-app-v4` only |
  | :2348 | KF1 RETURN §5 (:284-292) |
  | :2427 | RV20's C-N1 (`KF1_REVIEW.md:259`) |
  | :2457 | I16's disclosed `git fetch` (K6B RETURN :513) |
  | :2537 | K6B RETURN :251-252 |
  | :2631 | K6B RETURN :578 (+3.28% and +2.33%; +0.04–0.22% at 10 and 100 members and on the nine) |

## 3. Hashes (PASS; N5)

`sums_coverage.py.txt` checks against the head's blobs; `shasum_c.sh.txt` runs `shasum -a 256 -c` from each folder.

| Folder (under `T3/`) | Entries verified / tracked files |
|---|---|
| `DESIGN_NUMERICS/REV_5A3_CANDIDATE/` | 179/179 (every file outside `_v4_records/`) |
| `…/_v4_records/` | 164/164 (with the 97 files of `r3/`–`r7/`, which also have their own sums) |
| `…/_v4_records/r4/`, `r5/`, `r6/`, `r7/` | 22/22, 27/27, 25/25, 7/7 |
| `HANDOFF_2026-09-30_TOOLS/` | 1/1 |
| `IMPLEMENTATION/{F1B,K4,K5,K6,K6B,KF1,KF2,KF3,VK}_MERGE/` | 14, 19, 15, 14, 12, 18, 12, 12, 12: all verified and complete |
| `REVIEW/_run_records/{f1b,k4,k5,k6,k6b,kf1,kf2,kf3,vk}_review/` | 143, 93, 184, 74, 82, 39, 110, 82, 58: all verified and complete |
| `REVIEW/_run_records/` (main's, unchanged) | 76/76 **from `REVIEW/`**; 0 OK from its own folder (N5) |

- **The paths no SHA256SUMS lists:** the top-level `REVIEW/*_REVIEW.md` files, the briefs, `V1`, the handoff and `WG`. That matches main's precedent: `K1_REVIEW.md`, `K5_REVIEW.md` and the earlier briefs are in no sums either.

## 4. Machine paths, host names, GEN-8 (PASS)

- **`leak_scan.py.txt`** scans every added file, and the added lines of each modified file.
  - **GEN-8's `MACHINE_ABS_PATH_RE`** (`tools/practitioner_harness/surface_roles.py:22-26`): 0 hits.
  - **The broad patterns** (`/Users/`, `/private/`, `/home/`, `/tmp/`, `/var/folders`, `/Volumes/`, `~/`, `$HOME`): 5 hits. Each describes the patterns themselves: handoff :155, `VK_REVIEW.md:262`, `vk_review/records_checks.txt:11`, and two `sed` scrubbers in `kf2_review` scripts.
  - **The host's computer name, local host name and user name** (supplied at run time, never printed): 4 hits.
    - Two are `<home>/.local/…` placeholders in `k6_review` probe outputs.
    - Two are RV18's lists of scan patterns.
  - **Model identifiers:** 12 hits, all in scan-pattern lines of review scripts and check outputs.
  - **Also checked:** there are no e-mail addresses. Toolchain paths appear only as `<home>/.cargo/registry/…`, and the handoff's `<wt>` is defined relative to `<repo>`.
- **GEN-8:** `set -o pipefail; <VENV>/bin/python -m pytest -q tools/practitioner_harness/test_live_baseline.py -k gen8` in `<wt>/numerics` at `b0e357985` gives **1 passed, 10 deselected**. So that it tested the exact candidate:
  - I moved my own untracked output folder into my session scratch for the run, with `git status` empty and HEAD `b0e357985`;
  - then I restored the folder.
  - A second run with my files present is in `gen8.out.txt`.

## 5. The merge records against reality (PASS)

`merge_facts.py.txt` reads each record and queries `gh` and `git`. All nine match on every field:

| Slice | PR | Merge (UTC) | Head | Main = first parent, ancestor of head | Dispatch → target_base (from the run log) |
|---|---|---|---|---|---|
| K5 | #1044 | `1cdeae2c1` 2026-09-29 03:18:22 | `babcf5e65` | `b37331092` ✓ | 36511678387 → `b37331092` ✓ |
| F1b | #1052 | `59cb20073` 05:31:23 | `6fa422979` | `1cdeae2c1` ✓ | 36524065976 → `1cdeae2c1` ✓ |
| K6 | #1053 | `7ac7b1c37` 09:52:09 | `cd325c1fe` | `59cb20073` ✓ | 36548351414 → `59cb20073` ✓ |
| K4 | #1054 | `ab02ee3a6` 16:25:25 | `5a46a6278` | `7ac7b1c37` ✓ | 36593106169 → `7ac7b1c37` ✓ |
| KF1 | #1056 | `0f5d8c7b4` 21:34:25 | `66adfede4` | `8cca91701` ✓ | 36628173972 → `8cca91701` ✓ |
| V-K | #1057 | `f8400d290` 2026-09-30 00:26:51 | `5f0d39426` | `0f5d8c7b4` ✓ | 36646861753 → `0f5d8c7b4` ✓ |
| K6b | #1058 | `78f55f927` 01:08:38 | `597c81ba4` | `f8400d290` ✓ | 36650532005 → `f8400d290` ✓ |
| KF3 | #1059 | `dd61120ff` 05:28:58 | `aa83f6796` | **`45ffd91d1`, not an ancestor (disclosed)** | 36669370536 → `78f55f927` ✓ (an ancestor of the head) |
| KF2 | #1060 | `7ad3a9adf` 06:08:13 | `522167ac6` | `dd61120ff` ✓ | 36673660523 → `dd61120ff` ✓ |

- **The merges:** every one was by `sgttomas`, with a merge commit whose second parent is the head.
- **The CI runs:** every pull_request run the records cite (four per slice) is on its head and succeeded, and each PR's final rollup is 12 success and 4 skipped.
- **The earlier heads' dispatches** also match their records:
  - K4's 36570042397 (`7d8fa9c0e`) and 36584771469 (`a5fa0eaf7`);
  - KF1's 36621651732 (`1854911d1`).
- **The numerical cargo job's times** match: K4 20.07/18.33 min against K6's 14.05, and KF3 21.45/21.43.
- **KF3's disclosed slip holds exactly.**
  - #1061 merged at 05:01:47Z as `45ffd91d1`, 27 min before KF3. It changes 336 paths, all under `projects/chirality-app-v4/`.
  - `git diff aa83f6796 dd61120ff -- projects/chirality-piping tools .github` is empty, and the full `aa83f6796..dd61120ff` diff is those 336 app-v4 paths.
  - `dd61120ff`'s remerge diff is empty.
  - KF2's record says main had not moved. `7ad3a9adf`'s first parent is `dd61120ff`, which is consistent.
- **DEC-025, re-derived** (`dec025_chain.py.txt`: each record's `suites.log` against its stated baseline's, keyed by manifest):
  - Every "CHANGED" line in the records is reproduced, and nothing else changes. For example:
    - K5: FK 249→267, NI 120→133, PP 525→529;
    - K6: operation_applier 194→0, harness 25→54;
    - V-K: a new manifest, VR, at 47;
    - KF2: FK 417→432, with 1 ignored.
  - Every candidate fails only product_physics (1) and runner_headless (2).
  - pytest (3023, 3023, 3062, 3062, 3062, 3062, 3070, 3070, 3070), vitest (2822 of 2822, K5 by its re-run) and the build exits all match.
  - Each `meta.txt` head is its candidate head, and each sweep finished before its merge. The closest are K4 (46 s), V-K (27 s) and KF3 (35 s).
  - K6's operation_applier re-run adds to 194 (140+2+2+6+8+13+16+2+5).
  - The originals match: for KF2, KF3, K4 and V-K, the unsanitized sweep JSON's sha256 in `sweep_json_original_sha256.txt` equals the file in `<wt>/scratch/sweep_*`, and the committed `suites.log` is byte-identical to the scratch one.
  - Each record's baseline piping tree equals the main it names, with one wording point: K5's "piping tree" differs from `b37331092`'s only by 741 `_Coordination/` records paths (N10).
- **Merge chains:** the parents of the 14 slice-branch merges the records name match (`misc_checks.out.txt` §3).
  - K6b's FK equals main `f8400d290`'s.
  - The remerge diffs are as described: `597c81ba4` resolves to main in `adaptive.rs` and `verify.rs`; `c0473301e` has the ruled visibility edits in `structural.rs`, `adaptive.rs` and `bound.rs`, which V1 :2717 discloses; `522167ac6` and `e114b23c1` are empty.
- **Review verdicts in the records match the review files:** RV17 4/5, RV18 4/8, RV19 1/5/7 then 0/1/4 then none, RV20 1/5, RV21 2/5, RV22 3/7, RV23 1/5, and RV24 1/5. So do their confirmation and merge-check heads.
- **One pointer:** F1b's src-tauri evidence records no commit, but the `<wt>/scratch/tauri_f1b/tree` it built from has `6fa422979`'s bytes for the file that `6fa422979` changes (`f1b_w2_runtime.rs`).

## 6. Rulings against evidence (S1, S2; N2–N4)

- **The cited hashes:** `cited_hashes.py.txt` finds every one in Git history, with the line counts where stated. 25 were checked, including:
  - the K6, K4-A3, V-K, K6b, KF1, KF3 and KF2 plans;
  - R7's full `5502aef9…`;
  - RV19 `d906539e`/`319701f6`, RV20 `d1cde558`, RV21 `eab89fb5`, RV23 `96060f9e`/`0030aa39` and RV24 `6785aa13`;
  - K4's `_run_records` sums (214) and RV19's sums (47, 80).

  K6's `plan.txt` "138 rows" is 138 plus a header.
- **The sampled figures** (`figure_sample.py.txt`, V4 and misc checks): these match their sources, among them:
  - V4's four verdicts and figures (8.9e26, 3.3e66, 195, 46,315, 95,142, 14,186, 2^7.09–2^7.29, 2^37.8 and 2^97.8, and 57 hunks);
  - F1b's gate re-run (3,456 files, `runs.jsonl` `30d99bf0…` and `9139140c…`, and `compare_part1.json`'s PASS counts);
  - K6's A1/B1/B2/B3 (7.15e-8, the lane counts, 622 s, 3,319/3,411/4,782 MiB, 1.0032, 1.0078, 1.1010, 0.585, 1.402/1.093, 7.78/11.29 GiB, DOF 6001/6002, and 1.53 h);
  - K4 (99 controls and 5,490 rows at 0.28, 360 states, 222–819×, and the four a5fa0eaf7 runs);
  - the A0 export (240/20, and patch-id `b43efeb4` on both commits);
  - V-K (201, 2,590/80/50/0, 506, 507.67 and 3.8 s);
  - RV20 to RV24's counts;
  - KF1 (828, 131, 882, the +13.4 M LME and +16.2%, and 17.6 MB);
  - K6b's b3 (8.23e9, 814 MiB, 6.09 s, 7.6×, 31.5% and 0.92–0.98);
  - KF3's B (10.12/12.08/10.42 against 6.75/8.30/6.80 G LME, and 65.32/74.17) and KF3-B1 (20.5, 25.3, 74.1, 64.8, 142.9 and 177.1);
  - KF2's A and B (0.937/0.913 s, 203 s, 142/98, 818, 67.4–68.0 s, and `runs.jsonl` `c42981e5…`/`11dfe829…`).
- **The spawn bases in V1** (K6b, V-K, KF3, KF2, and KF1 as corrected) equal the parents of each branch's first commit.
- **V1 :1400's superseding** stands: the new ground is the arm order (:1711).
- **Additional errors, which this section looks for:**
  - S1: KF3-B2.
  - S2: V-K B's units.
  - N3: 188 s.
  - N4: "before `uc`".
  - N2: the fourth listed error, rewritten in place.

## 7. The work graph and the handoff (PASS; N5–N8)

- **The T3 state at the pause (`WG:62`)** agrees with the records. It names:
  - the PRs and merge SHAs of K5, F1b, K6, K4, KF1, V-K, K6b, KF3 and KF2;
  - #1049 at `0256decc6`;
  - the three frames that KF3 made selectable, and the TREE `Ceiling`;
  - KF2's runs of about 68 s;
  - K6c briefed, not spawned;
  - the next order: audit, K6c, W1 limits, F2a, S-I, F2b, F3;
  - the routed candidates;
  - the three owner decisions.

  The one exception is "about 19 MB" (S1).
- **Handoff §1.1:** all 14 PRs and merge SHAs are right, the earlier five included (#1032, #1034, #1038, #1040 and #1041).
- **§2 and §3:** they match V1's last sections and the work graph.
- **§4:** every heading it names exists in V1 (N6: the line count).
- **§5.2:**
  - The driver's argument order matches the committed `dec025_mac.sh.txt`, and the scratch driver is byte-identical to it.
  - `run_suites_nff.sh` has committed copies.
  - The three expected Mac failures are the names in every `suites_vs_baseline.txt`.
  - `<wt>/scratch/sweep_kf2` is at `522167ac6`, whose piping tree equals `7ad3a9adf`'s.
- **§5.3:** the `gh` commands are well formed, and `target_base` is the workflow's input.
- **§5.4:** `c0473301e` is the KF3 merge.
- **§7.2:** items 2–6 match the records. Item 1 is N2.
- **§9:** the listed folders exist, but see N7.
- **`APP:1688-1692` and `:1711-1717`** point to the right lines of `P/apps/desktop/src-tauri/src/lib.rs` at main (N8).

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `V1:2806`; `TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md:19-20`; `WG:62` ("about 19 MB at 10,000 members") | **KF3-B2's size is stated on a basis that KF3's own record calls not like for like, and it has spread into the next brief.**<br>• **V1 :2806:** "Against K6b's final formula, the peak inside `nl_pass` … exceeds E_max by 19.5 MB (AX) and 19.4 MB (ROT)."<br>• **I21 :19-20:** "KF3's scale run found that it is not [an upper bound]" and "The peak exceeds K6b's final E_max by 19.5 MB and 19.4 MB".<br>• **What KF3's RETURN §13 says** (:420-424): +19.5/+19.4 MB compares `vk_scale`'s measured peak with K6b's E_max as computed for `k6_observe`, whose fixed term is 50.3/52.2 MB. On `vk_scale`'s own fixed term (76.8/79.0 MB), "the like-for-like comparison … **K6b's final formula bounds both measured peaks**", by 7.0 and 7.5 MB.<br>• **Where the finding actually stands:** on the code derivation. The net under-count is 10,799,688 B at the 1024 shift: `at`/`bt`/`ct` at 25.92 MB less `work` at 8.64 MB, which is +17.28 MB, and `OPTION_EXTRA` at −6.48 MB. RETURN :445: "The `vk_scale` measurement stays inside it through slack elsewhere".<br>• No measurement shows E_max exceeded, and none of these figures is "about 19 MB".<br>• **A stale pointer:** I21's "`K4R/bound.rs:1040` at KF3's head" is `b8c55c92e`'s line. At I21's base (main, equal to `aa83f6796`'s FK) the call is `bound.rs:1059`, and :1040 is a comment. | **Before K6c spawns,** bracket V1 :2806 and I21 :20, for example: "[Correction (RV25-S1): +19.5/+19.4 MB is `vk_scale`'s peak against E_max on `k6_observe`'s fixed term. Like for like, K6b's formula bounds both peaks by 7.0/7.5 MB (KF3 RETURN §13). The finding rests on the code: net under-count 10,799,688 B at the 1024 shift.]"<br>Reword I21 :19 ("KF3's derivation found …") and give the base's line (`bound.rs:1059`).<br>In `WG:62`, bracket "about 19 MB" with the same figures.<br>K6c's task is unchanged. |
| S2 | SHOULD-FIX | `V1:2521` | **V-K B's heap and E_max are given in GB that are MiB/1,000: the E_adm slip of :2537 again, with no bracket.**<br>• "about 0.82 GB of heap, against E_max of 2.7 GB (ρ 0.32–0.37)".<br>• VK RETURN §14.3 (:502) gives heap 816.5–835.0 MiB and E_max 2,676–2,752 MiB. That is 0.86–0.88 GB (0.80–0.82 GiB) and 2.81–2.89 GB (2.61–2.69 GiB).<br>• ρ matches, and nothing relied on the absolute figures. | Bracket: "[Correction (RV25-S2): heap 816.5–835.0 MiB (0.86–0.88 GB); E_max 2,676–2,752 MiB (2.81–2.89 GB); VK RETURN §14.3.]" Add it to handoff §7.2's list (N2). |
| N1 | NOTE | `WG:62`; PR body, "Modified files" and "ROOT_RULINGS_V1.md" bullets | **(a) The work-graph change is not append-only.** The PR body says all modified files are expected to be append-only. `438f3c0ca` rewrote main's "and DS1 is writing R4" as "; DS1's R4 is committed (`c644b751d`), and V4 is checking it". That is RV16-D1's proposed fix, and accurate, but unbracketed.<br>**(b) The appended range starts earlier than the PR body says.** V1's 1,447 appended lines begin at :1558, "K6: rulings on I15's checkpoint-0 plan (ROOT, 2026-09-28)", 24 lines before "Records PR #1049 merged". | Say so in the PR body: "WG :62 carries RV16-D1's rewording of one clause; V1's append starts at the K6 checkpoint-0 ruling." |
| N2 | NOTE | `HANDOFF_2026-09-30_AUDIT_PAUSE.md:180-186`; `V1:2841` | **Handoff §7.2 says each of ROOT's four figure errors "is recorded as a bracketed correction". The fourth is not.**<br>• `e487742929` (20:03:44 −0600) wrote "…, and no admission decision there relied on it at the margin."<br>• `ffc489f52`, 12 s later ("correct an unverified claim in the K6c ruling"), replaced it with "K6c re-checks B's admission decisions …; ROOT has not verified them."<br>• V1 carries no trace of the withdrawn claim. It was probably never pushed alone, and Git keeps it.<br>• "Four times" is also now at least seven: S1, S2 and N3. | Either bracket :2841 ("[Correction: this sentence first said '… relied on it at the margin', unverified; replaced at `ffc489f52`.]"), or reword §7.2 to say the fourth was replaced in place. Add S1, S2 and N3 to the list. |
| N3 | NOTE | `V1:2785` | "After KF2 the dense factor is the long step (65–188 s at 1,000 members)", in an owner-facing note. The source is KF2's plan :415: "65–86 s at 1,000 members and 166–188 s at the c[eiling]". KF2 RETURN :211 has "65–188 s at 1,000 members up to K6's ceiling". | Optional bracket: "[65–86 s at 1,000 members; 166–188 s at the 1,364-member ceiling]". |
| N4 | NOTE | `V1:2519` (V-K B: "in the 256 verification's shared build, before `uc`") | V-K's record says the Span arises "after the bounded and wide formation, before the Uc bounds complete" (VK RETURN :493). K6b's stop ruling (:2485) had already found that the shared build records no stage for partial `uc` work, and KF3's diagnosis (:2548) puts the stop inside `uc_bounds` → `u_pass`. So "before `uc`" misreads the missing stage record. :2532 says "in the … `uc` stage". | Optional: "[before the Uc bounds complete; KF3 places it in `uc_bounds`]". |
| N5 | NOTE | Handoff :154 (§6); main's `T3/REVIEW/_run_records/SHA256SUMS` (unchanged) | "`shasum -a 256 -c SHA256SUMS` must run from the folder that holds it" has one exception on main. `REVIEW/_run_records/SHA256SUMS` lists `./RETURN.md`, `./_run_records/…`, which are relative to `REVIEW/`. Run from its own folder it gives 153 not-OK and 0 OK; from `REVIEW/` it gives 76/76. | Add "(except `REVIEW/_run_records/SHA256SUMS`, which is run from `REVIEW/`)", or place a one-line README beside it. |
| N6 | NOTE | Handoff :162 (§7.1 item 1); :82 (§4); :56 (§1.4) | **(a)** Checkpoint 0 is credited with having "surfaced THIN in V-K before B". THIN surfaced at V-K's A1 stop ("V-K: rulings on I17's A1 stop", V1 :2175), as item 2 correctly lists.<br>**(b)** §4 says V1 is "about 2,850 lines". It is 3,003.<br>**(c)** §1.4's "the TASKs of 2026-09-29 and 30 (I16 to I20, and RV19 to RV24)" leaves out that day's I12–I15, RV17, RV18, V4 and DS1. Its "no agent is running" point stands. | Optional wording fixes. |
| N7 | NOTE | `IMPLEMENTATION/KF2_MERGE/RECORD.md:78`; handoff :228, :231 | **The latest Mac baseline is marked both keep and prune.**<br>• KF2's merge record lists `<wt>/scratch/sweep_kf2` under "Scratch to prune".<br>• The handoff's prune paragraph (:228) groups "`sweep_k4` to `sweep_kf2`" with the prunable sweep folders.<br>• But :231 and §5.2 step 7 keep it as the latest Mac DEC-025 baseline.<br>• The merge record is hash-bound. | In the handoff, exclude `sweep_kf2` at :228 and add "(KF2_MERGE's prune list names it; keep it)". |
| N8 | NOTE | `WG:38` (T6 note); handoff :80 (§3); `V1:2785` | "APP:1688-1692, :1711-1717" uses an abbreviation defined only in KF2's plan (:29: `P/apps/desktop/src-tauri/src/lib.rs`). The lines are right at main `7ad3a9adf`, but a work-graph or handoff reader cannot resolve "APP". | Spell out the path, and the commit, in `WG` and the handoff. |
| N9 | NOTE | `REVIEW/KF2_REVIEW.md:88`, `:94`, `:177`; raw run records; 6 `*.sh.txt` records at mode 100755 | **Hygiene.**<br>• `git diff --check` flags 47 files: three trailing-whitespace lines in one authored review, and 46 raw run records and logs (for example the `kf2_review/mutations/` logs, `k6b_review/checks/adapter_check.out` and the vitest logs).<br>• Six script copies are executable (main has 28 such records).<br>• The PR body discloses neither (RV16-N9's class). | Optional disclosure line in the PR body. |
| N10 | NOTE | `IMPLEMENTATION/K5_MERGE/RECORD.md:42` | "K2b's final head `33e33c723`, whose piping tree equals main's (`e7d930d49` and `b37331092`)". It equals `e7d930d49`'s. Against `b37331092` it differs by 741 paths, all `_Coordination/` records from #1042 and #1049, and 0 elsewhere, so the suites are unaffected. Later records say "piping source". | None needed; recorded for the audit. |

## What I did not do

- **No builds, suites, gates or mutants.** I ran GEN-8 twice, at the head only (§4). I did not re-run GEN-8 on the nine slice heads; the governance-harness `pull_request` run on each head succeeded.
- **Not re-derived:** the D1 5a.3 mathematics, amendments A1 and A2, and KF3-B2's byte accounting. I checked ROOT's statements against the RETURNs and reviews that derive them, not the derivations.
- **Sampled, not exhaustive:** more than 90 of V1's figures and every cited hash. K6b's B-stage (pre-KF1) figures were checked against `K6B/_run_records/b/` only where the B report or `r1_compare.out` states them, for example 8,773, 0.43/0.33, and 10,078 comparisons with a worst of 1.746e-6.
- **Uncommitted evidence read** (hashes only, read-only):
  - `<wt>/scratch/i13/gate2/`;
  - `<wt>/scratch/gate_base_e7d930d49_full/`;
  - `<wt>/scratch/i20/b/gate/`;
  - `<wt>/scratch/sweep_{k4,vk,kf3,kf2}/`;
  - `<wt>/scratch/tauri_f1b/tree` (one file);
  - `<wt>/scratch/sweep_skewpin/dec025_mac.sh`.
- **Writes:** this file and `T3/REVIEW/_run_records/records_pr1062_review/` (scripts `*.py.txt`/`*.sh.txt`, their outputs, `README.txt`, `SHA256SUMS`), all uncommitted. The host-name patterns for the leak scan lived only in my session scratch.

## Delta check at 18b625268

**Delta verdict: PASS.** S1 and S2 are fixed at every site I named, and the corrected statements agree with KF3 RETURN §13 and VK RETURN §14.3. N2–N8 are handled as the rulings say. N1 and N9 are disclosed in the PR body, and N10 is recorded in the rulings. The check adds 4 new NOTEs (D1–D4) and no BLOCKING or SHOULD-FIX finding. The review's verdict stands: PASS.

**Scope.** ROOT resumed me for this check. The mechanism is unchanged: a background subagent of ROOT's session, with no delegation and no Git writes or index operations.
- **The head:** `18b625268b76606401961fc512fb0ec8fcb5947d`, PR #1062's `headRefOid` and `<wt>/numerics` HEAD. It has one parent, `b0e357985`, the head I reviewed. Base main is `7ad3a9adf`.
- **The commit changes 26 paths, all under `_Coordination/`:** 22 added and 4 modified.
  - **Added:** this review (sha256 `45b27642…`) and my 21 run-record files (`SHA256SUMS` `fe00bcef…`). Both are byte-identical to what I returned.
  - **Modified:** `ROOT_RULINGS_V1.md`, `TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md`, `HANDOFF_2026-09-30_AUDIT_PAUSE.md` and the work graph.
- **Records:** `_run_records/records_pr1062_review/delta_18b625268/`. That covers `delta_checks.sh.txt` and its output, the review's scripts re-run on the delta (append-only, SUMS coverage, leak scan), and `gen8_delta.out.txt`.

### Findings: resolution

| ID | Status at `18b625268` | Evidence |
|---|---|---|
| S1 | **resolved** | Insert-only brackets now stand at every site I named, and at one more:<br>• **V1 :2806:** "not like for like … On `vk_scale`'s own fixed term, K6b's final formula **bounds** both measured peaks, by 7.0 and 7.5 MB … net 10,799,688 B … On main the call is at `bound.rs:1059`."<br>• **V1 :2757**, the checkpoint-B finding: its 2,750 is VR's stale port, and like for like the peaks are bounded.<br>• **I21 :20:** a bracket giving the premise, the basis and `:1059`.<br>• **I21 :37:** a new like-for-like requirement in K6c's bar.<br>• **`WG:62`:** the phrase is re-worded to "by code derivation, its 1024 shift term under-counts by a net 10.8 MB …, chiefly `nl_pass`'s buffers; like for like the measured peaks were still within it, by 7.0–7.5 MB". This text is new in this PR, so rewording it is acceptable.<br>• **Handoff §8.1 item 4:** the same.<br>**Against the source:** KF3 RETURN §13's table (+19.5/+19.4 MB on `k6_observe`'s fixed term, −7.0/−7.5 MB on `vk_scale`'s) and its derivation (+17,280,000 − 6,480,312 = 10,799,688 B) match every corrected figure. The call is at `bound.rs:1059` at main, where the file equals `aa83f6796`'s. No "about 19 MB" is left in `WG`. See D1 for one clause of I21's bracket. |
| S2 | **resolved** | V1 :2521 gains "[Correction (ROOT, 2026-09-30, RV25-S2): … a heap of 816.5–835.0 MiB (0.86–0.88 GB) and E_max 2,676–2,752 MiB (2.81–2.89 GB). ρ is unaffected.]". VK RETURN :502 gives 816.5–835.0 and 2,676–2,752 MiB, and the conversions are right: 0.86–0.88 GB and 2.81–2.89 GB. |
| N1 | disclosed | The PR body's "Modified files (main's text)" section names `WG:62`'s RV16-D1 rewording, and says V1's append starts at "K6: rulings on I15's checkpoint-0 plan". Against main, `WG:62`'s only non-insert edit is still that one clause. See D3. |
| N2 | resolved | V1 :2841 gains a bracket quoting the withdrawn words and citing `e48774292` and `ffc489f52`, 12 s apart. Handoff §7.2 item 1 now lists seven errors, each with its V1 bracket: :2348, :2537, :2631, :2841, :2521, :2757/:2806 and :2785. Every one exists. |
| N3 | resolved | V1 :2785 gains "[… 65–86 s at 1,000 members; 166–188 s at K6's 1,364-member ceiling (B3).]", which matches KF2's plan :415. |
| N4 | resolved | V1 :2519 gains "[… 'before the Uc bounds complete' … inside `uc_bounds`.]", which matches VK RETURN :493 and V1 :2548. |
| N5 | resolved | Handoff :154 now names `REVIEW/_run_records/SHA256SUMS` as the exception, run from `REVIEW/`. |
| N6 | resolved | Handoff §7.1 item 1 no longer credits checkpoint 0 with THIN. §4 now says "about 3,000 lines"; the file has 3,035. §1.4 now lists I12–I20, RV17–RV25, V4 and DS1. |
| N7 | resolved | Handoff :233 now reads "`sweep_k4` to `sweep_kf3` … `sweep_kf2` is kept as the current Mac baseline, which overrides the prune list in `KF2_MERGE/RECORD.md`". The hash-bound record is untouched. |
| N8 | resolved | "APP:" is gone from `WG` and the handoff. Both now cite `P/apps/desktop/src-tauri/src/lib.rs:1688-1692` and `:1711-1717`. V1 :2785 keeps "APP:", which is acceptable in a ruling. See D4. |
| N9 | disclosed | The PR body's "Hygiene (RV25-N9)" line. |
| N10 | recorded | The new V1 section, "Records PR #1062: rulings on RV25's review" (:3004-3035), records it, and the hash-bound K5 record is untouched. |

### No main or earlier ruling text removed

- **Against `b0e357985`** (`append_only_delta.out.txt`):
  - **V1:** six replaced lines (:2519, :2521, :2757, :2785, :2806 and :2841). Each is insert-only: the old line is a character subsequence of the new line, and the only additions are the "[Correction (ROOT, 2026-09-30, RV25-…)]" brackets. Then 32 appended lines, the new rulings section.
  - **I21:** two insert-only lines, :20 and :37.
  - **`WG` :38 and :62, and the handoff (:56, :80, :82, :162, :180, :209 and :233):** these are rewordings. Each rewords only text that this PR itself added: `WG:38`'s T6 note and `WG:62`'s audit-pause paragraph, both added in `07fb44d56`, and the handoff, which is new in this PR. :154 and :184 are insert-only.
- **Against main `7ad3a9adf`** (`append_only_vs_main.out.txt`):
  - main's 1,556 lines of V1 are kept, with the one insert-only bracket at :1400 as before, and then 1,479 appended lines;
  - `V4_VERIFICATION.md`, `K5_REVIEW.md` and `RECORDS_PR1049_REVIEW.md` still start with main's bytes;
  - the three modified SHA256SUMS files are as before;
  - `WG`'s only non-insert change is still N1's clause.

### Hashes, leaks, GEN-8 and CI

- **SHA256SUMS** (`sums_coverage_18b625268.out.txt`, checked against the head's blobs): all 26 files that cover a changed path verify and are complete. That includes my own folder, 20/20, which `delta_checks.out.txt` §2 also verifies with `shasum -a 256 -c`. Main's `REVIEW/_run_records/SHA256SUMS` is 76/76 from `REVIEW/`, as before.
- **Machine paths:** `leak_scan_delta.out.txt` over the 26 changed paths finds hits only in my own records, and only where a pattern describes itself:
  - GEN-8's regex matches the two pattern-list lines of my `leak_scan.py.txt` (:9, :26);
  - the host patterns match my review's `<home>/.local` placeholder mention (:128) and RV18's pattern list as copied into `leak_scan.out.txt` (:12).
  - None is a machine path, and ROOT's new text has none.
- **GEN-8 passes at `18b625268`** (`gen8_delta.out.txt`): 1 passed, 10 deselected. It was run twice:
  - with this delta folder moved aside, so the tree was exactly the head, with `git status` empty;
  - again with the delta folder present.
- **Hosted CI on `18b625268`:** 7 success and 6 skipped, the records-only selection. The four runs are pec-tests 36680738836, governance-harness 36680738848, Harness Pre-merge 36680738849 and Piping Desktop E2E 36680738860, all `pull_request` on the head.

### Delta findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| D1 | NOTE | `TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md:20` (the S1 bracket's last sentence); also V1 :2806 and handoff :209 ("not yet *shown* to bound that phase") | **The bracket understates what KF3's derivation shows.** It ends "So E_max is not yet *shown* to bound that phase; it has not been shown to fail it either."<br>KF3 RETURN §13 (:444-445): "With this phase taken from the code, K6b's E_max on H's harness would be 3,021,565,490 (AX) …. So K6b's formula **does not bound this phase by construction**." Against the current 3,010,765,802 B, the derivation shows the formula falls short for that phase. What no one has shown is a *measured* peak above it (no `k6_observe` run exists), and the one like-for-like measurement is within it.<br>K6c's task and bar are unaffected. It re-derives and corrects either way, and D1 does not reopen S1. | Before K6c spawns, reword I21's last clause, for example: "By the code derivation, E_max falls short of that phase by 10,799,688 B ('does not bound this phase by construction', KF3 RETURN §13); no measured peak has yet exceeded it." Optionally align V1 :2806 and handoff :209 in the same way. |
| D2 | NOTE | V1 :3017-3018 (the new section, S1: "Corrected in place with brackets: … the work graph's state paragraph; and the handoff §8.1"); commit `18b625268`'s message ("All corrections are brackets or additions") | **The ruling says two S1 corrections are brackets; they are rewordings.** `WG:62`'s phrase and handoff §8.1 item 4 were reworded, which ROOT's dispatch message and the PR body state correctly. The rewordings touch only this PR's own new text, and Git keeps the old wording, so nothing is lost. | Optional: "[the work-graph and handoff fixes are rewordings of this PR's own text]" in the next rulings section. |
| D3 | NOTE | PR #1062 body, "What", the `ROOT_RULINGS_V1.md` bullet (line 10) | **The body contradicts itself.** It still says "1,447 lines appended (from 'Records PR #1049 merged' onward)". Its own "Modified files" section (line 24) gives the correct start, and at `18b625268` V1 appends 1,479 lines. | Edit the "What" bullet to match line 24, with 1,479 lines. |
| D4 | NOTE | `WG:38` (the T6 note) | **"APP" is replaced by a prefix the work graph doesn't define.** `P/` is defined in the handoff (:17), but the work graph defines no `P/` and uses it nowhere else; main's work graph has none. | Optional: `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs`. |

### What I did not do in the delta

- **Not re-checked:** the first review's facts outside the delta. The merge records, DEC-025 and the figure sample are unchanged by `18b625268`.
- **Not re-derived:** KF3's §13 byte accounting. I checked the corrections against §13's stated table and derivation.
- **Runs:** GEN-8 twice at the head, plus read-only `git`, `gh` and `shasum`.
- **Writes:** this appended section and `delta_18b625268/` (7 files), with SHA256SUMS extended by appending their entries (the 20 existing lines are unchanged). All are uncommitted.

## Delta check at 7b59a1efc

**Delta verdict: PASS.** D1–D4 are fixed, and the D1 wording agrees with KF3 RETURN §13. I raise no new finding. The review's verdict stands: PASS.

- **The head:** `7b59a1efc3a1578d1ecbbe78f3a876c9608b68b1`, PR #1062's `headRefOid`. It has one parent, `18b625268`, and changes 13 paths, all under `_Coordination/`: 7 added and 6 modified.
- **My files went in byte-identical to what I returned:** the review (sha256 `a131c10d…`), `SHA256SUMS` (`fb294698…`) and the seven `delta_18b625268/` files, each hash equal to its SUMS line.
- **Records:** `_run_records/records_pr1062_review/delta_7b59a1efc/`.

| ID | Status at `7b59a1efc` | Evidence |
|---|---|---|
| D1 | **resolved** | **Two insert-only "[Correction (ROOT, 2026-09-30, RV25-D1) …]" brackets:** after the S1 bracket at V1 :2806, and inside I21 :20's S1 bracket, right after the sentence it corrects. Each reads "By construction, K6b's formula does not bound this phase: taken from the code, the phase needs 3,021,565,490 B (AX) against E_max 3,010,765,802 B … Only a *measured* excess is unshown".<br>**Against KF3 RETURN §13:** :444 gives 3,021,565,490 (AX), and :445 says "does not bound this phase by construction … through slack elsewhere". The difference is 10,799,688 B, the net under-count. The cite ":440-445" covers everything except E_max's 3,010,765,802, which is §13's table at :420.<br>**Reworded to the same effect:** handoff §8.1 item 4 (:209) and `WG:62`'s KF3-B2 phrase ("by construction, E_max does not bound the 1024 shift phase … a net 10.8 MB more … No measured peak has exceeded it"). Both are text new in this PR. |
| D2 | resolved | V1 :3018 gains an insert-only bracket saying that the work-graph and handoff S1 fixes were rewordings of this PR's own text, and that `18b625268`'s message overstated it. |
| D3 | resolved | The PR body's "What" bullet now reads "1,479 lines appended to main's 1,556, from 'K6: rulings on I15's checkpoint-0 plan' onward (RV25-N1, D3)". |
| D4 | resolved | `WG:38` cites `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:1688-1692` and `:1711-1717`, and no `` `P/ `` remains in `WG`. |

- **Append-only** (`append_only_delta.out.txt`, `append_only_vs_main.out.txt`):
  - Against `18b625268`, the V1 and I21 changes are insert-only brackets: V1 :2806 and :3018, and I21 :20. The review and `SHA256SUMS` only append.
  - The rewordings touch only text this PR added: handoff :209, and `WG` :38 (the T6 note) and :62 (the pause paragraph).
  - Against main, nothing changes from `18b625268`: V1 keeps main's 1,556 lines, with the :1400 bracket, then appends 1,479, and `WG:62`'s only non-insert edit is still N1's clause.
- **SHA256SUMS** (`sums_coverage_7b59a1efc.out.txt`): all 26 files that cover a changed path verify and are complete. My folder is 27/27 with `shasum -a 256 -c`, run from the folder. Main's `REVIEW/_run_records/SHA256SUMS` still verifies from `REVIEW/`.
- **Machine paths** (`leak_scan_delta.out.txt`): hits only in my own records, as pattern lists and the `<home>/.local` placeholder mention. ROOT's new text has none.
- **GEN-8 passes at `7b59a1efc`** (`gen8_delta.out.txt`): 1 passed, 10 deselected. It was run twice:
  - with this folder moved aside, so the tree was exactly the head, with `git status` empty;
  - again with this folder and this section present.
- **Hosted CI on `7b59a1efc`:** 7 success and 6 skipped. The four runs are pec-tests 36681884821, Piping Desktop E2E 36681884888, Harness Pre-merge 36681884935 and governance-harness 36681885065, all `pull_request` on the head.
- **Writes:** this section and `delta_7b59a1efc/` (7 files), with SHA256SUMS extended by appending their entries (the 27 existing lines are unchanged). All are uncommitted. No Git writes.
