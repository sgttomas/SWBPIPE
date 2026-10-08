# RV106: independent review of the T3 records-only PR after #1104 (#1105)

**Reviewer:** RV106, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of these records. 2026-10-07 UTC.

**Brief:** `R/BRIEFS/RV106_RECORDS_PR5_REVIEW.md` (sha256 `b0834e6c…`, verified at the start and again before writing), read in full. Its method is `R/BRIEFS/RV103_RECORDS_PR4_REVIEW.md` items 1–6, with the brief's substitutions. Also read: the repository root `AGENTS.md` (`f96feb19…`) and `agents/AGENT_TASK.md`. Precedents read first: RV103's `REVIEW.md` (#1103) and RV102's `REVIEW.md` (#1101).

**Placeholders.** WT = the t3 workspace; NUM = WT/numerics; P = `projects/chirality-piping`; T = P/execution/…/NUMERICAL_INTEGRITY_T3 (written `T3/` in the run records); R = T/RESUME_2026-09-30; RR = T/ROOT_RULINGS_V1.md; WG = P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md; VENV = the piping venv's Python 3.13.14. `RR:n` and `WG:n` are line numbers at the PR head.

**NUM moved during the review.** At dispatch NUM was `cc0c9f4e4e`. By the end it was `6a5242d800`: three records-only commits ("#1105 opened; …", "I81's B1-0 probe verified; …", "RV104 passes SI1b; …"), with 0 non-execution paths. None of them is in #1105. Every check below is against N = `030020aca3` and the PR head H.

## The candidate

| Item | Value |
|---|---|
| PR | #1105, `codex/piping-t3-records-20261007` → `main`, **draft**, MERGEABLE (merge state BLOCKED while draft) |
| H (head) | `736f3fb7b27154732b27b716152d7446e321acf9`, one commit, sole parent M |
| M (base, main) | `bfb26596bf81a90f98bfde06051f9a3b9907783a` (#1104's merge); origin's main is still M |
| N (source, NUM) | `030020aca359953ee7b925a52852d11bc183f11c` |
| Author and committer | the owner's configured Git identity; the only other identity is the agent co-author trailer |

## Verdict: **PASS** (no blocking finding)

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 5 |

The following pass:
- **Scope.** H is M plus one commit, not from NUM's history. **H's whole tree equals N's** (tree `6efac947ce…`). 220 paths change: 218 A, 2 M, 0 D, all under `P/execution/`. The modified paths are exactly RR and WG.
- **Publication screen.** No credentials, personal data, whole-host data, binaries or large files.
- **Portability.** GEN-8 passes on the exact head (mine and ROOT's). The changed files hold 0 machine-absolute paths, and the living documents hold none.
- **Integrity.** All 19 sum files in scope verify from the committed tree (342/342 entries). RR is append-only. None of the 13 redacted originals is in H.
- **Gates.** The four automatic CI runs succeeded on H.
- **The living documents.** #1104's and #1103's merges, RV103's S-1 fix, the owner's decision on M, B0's selection, the two owner notices, RV105's findings and WG's T3 section all agree with the records, Git, GitHub and the host. **ROOT took no owner-held decision.**

The five notes are wording, an interpretation worth anchoring, and one worktree that must survive until this PR merges (N-2).

## Findings

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | RR:13411–13415 ("Owner decision: ROOT may raise M up to 6.0 GiB without asking"); WG:579, WG:585 | **The owner said "the memory ceiling"; RR reads it as M.** The reading is well supported:<br>- The other memory ceilings, the dense-scrutiny and observation-lane ceilings, are already provisional at 6 GiB and owner-held (RR:1776). "Raise … to 6.0 GiB" can only mean M, which is 3.75 GiB.<br>- ROOT's preceding notice (RR:13407) named M (decision 17) as the item that might need the owner.<br>But RR does not record what ROOT put to the owner. Before this section, RR never calls M "the memory ceiling", while the owner-held list still carries "dense and lane ceilings" (WG:574). WG keeps those ceilings owner-held, so nothing was widened in practice. | Optional, at the next RR append: record the question the owner answered, and state that the dense-scrutiny and observation-lane ceilings are unchanged and owner-held. |
| N-2 | NOTE | RR:13280 ("N-4: `WT/records-pr` … may now be cleaned") | **The line is now stale.** `WT/records-pr` was reused for #1105. It is the only Git checkout of H (HEAD = H, clean), and ROOT's GEN-8 and mine both ran there. WG's cleanup list (WG:634) correctly leaves it out. | Keep `records-pr` out of any cleanup `apply` until #1105 merges (RV103 N-4's recurrence). |
| N-3 | NOTE | RR:13323; RR:13415 vs RR:13446–13448 | **Two count and numbering slips about B0's decisions.** Neither changes the selection, which RR:13439–13450 states correctly.<br>- RR:13323 says I78's twenty decisions are "sixteen for ROOT, and four owner-held items carried (… observation framing unchanged)". DESIGN v1 §8 gives ROOT 17 decisions (1–16 and 20) and the owner 3 (17–19). Observation framing is an owner-held item that decision 10 leaves untouched, not a fourth decision.<br>- RR:13415 says the owner's decision "resolves B0's decision 17". That is DESIGN v1's numbering, where 17 was "M above 3.75 GiB". In DESIGN_v2 the M ≤ 6.0 GiB lever is decision 16, and 17 is "M above 6.0 GiB", which stays owner-held (RR:13448). | Optional erratum at the next append: "17 ROOT, 3 owner, plus observation framing untouched"; and "decision 17 of DESIGN v1 (now v2's 16 and 17)". |
| N-4 | NOTE | WG:548 ("Position (2026-10-06 UTC)"); WG:562 (the T3-SI1b row) | **Two stale details in the living account.**<br>- The heading is dated 2026-10-06, but the section reports #1104's merge on 2026-10-07 (WG:552).<br>- The SI1b row still says "then its own PR after T6S merges to main". T6S has merged, and Next safe action 2 (WG:632) is already current. | Re-date the Position heading and reword the row at the next WG touch. |
| N-5 | NOTE | PR #1105 description | **One omission.** The "What it carries" list does not mention `IMPLEMENTATION/SESSION_2026-10-06/SHA256SUMS.dec025_mac`, which is RV103 N-3's seal. The rest is accurate: 218 added, 2 modified, 0 deleted; the #1103 and #1104 merge records; RV101's two addenda; RV103's and RV105's reviews; the I75, I78 and I79 returns; the 6 briefs; U8's counts extract; RR and WG. | Name the seal in the explicit squash body ("RV103 S-1's counts extract and N-3's seal"). Mark the PR ready before merging. |

## 1. Scope: PASS

Evidence: `_run_records/scope_checks.txt` and `changed_paths_name_status.tsv`.

**Parentage:**
- H's only parent is M, and `rev-list --count M..H` = 1. Origin's main = M.
- H is not an ancestor of N or of NUM's later heads. **None of NUM's 765 commits in `M..N` is an ancestor of H.**
- M is an ancestor of N: NUM absorbed #1104 as `25c4fa3e6a`.

**Equality with NUM:**
- **H's whole tree equals N's** (`6efac947ce…`). `P/execution` = `73594f312d…` on both sides, and `git diff N H` is empty.
- N's non-execution tree equals main's (0 paths), so the non-execution diff against main is empty.

**Changed paths** (`git diff --no-renames M H`):
- **220 paths: 218 A, 2 M, 0 D**, all under `P/execution/`. 0 paths fall outside it.
- Modes: 218 regular and 2 executable (`IMPLEMENTATION/T6S/_draft_run_records/scripts/run_checks.sh` and `R/I75/t6s_01/_run_records/repair_01/tools/run_repair_suites.sh`). No symlinks.
- Added by folder:

  | Folder | Files |
  |---|---|
  | `IMPLEMENTATION/T6S/_draft_run_records/` | 18 |
  | `IMPLEMENTATION/T6S_MERGE/` | 18 |
  | `IMPLEMENTATION/RECORDS_MERGE_2026-10-06B/` | 6 |
  | `IMPLEMENTATION/U8_MERGE/` (the counts extract and its sum file) | 2 |
  | `IMPLEMENTATION/SESSION_2026-10-06/` (`SHA256SUMS.dec025_mac`) | 1 |
  | `R/I75/t6s_01/` (REPAIR_01) | 55 |
  | `R/I78/b0_contract_01/` | 5 |
  | `R/I79/si1b_01/` | 32 |
  | `R/REVIEW_RV101/t6s_01/` (ADDENDUM_01 and ADDENDUM_02) | 32 |
  | `R/REVIEW_RV103/records_01/` | 30 |
  | `R/REVIEW_RV105/b0_01/` | 13 |
  | `R/BRIEFS/` (B1_0_PROBE, B1_S_CAP_STUDY, I80_T6S_PR_PACKAGE, RV103–RV105) | 6 |

**The modified paths are exactly the two expected:**

| Path | Main blob → head blob | Change | Lineage |
|---|---|---|---|
| RR | `d46bb31436` → `3117de0f48` | +296 / −0 | Main's blob = NUM's at `ea0e288e8a` (#1103). Byte prefix (§5) |
| WG | `dab8b204f6` → `8ba1e1c333` | +21 / −15 | Main's blob = NUM's at `ea0e288e8a`. Main has not touched it since `d8c88774d0`; 4 NUM commits change it after. **No change of main's is reverted** |

**A1-S-1 (no open product PR's package):** the only other open PR is #885, a draft from 2026-09-24 ("Connect a private live-control CLI …"). It is not one of T3's, and it shares 0 paths with #1105 (`_run_records/open_prs.txt`). #1104's package (`IMPLEMENTATION/T6S/`, 4 files) reached main with #1104. #1105 adds only its `_draft_run_records/`.

## 2. Nothing that must not be published: PASS

The screen covers the text this PR adds: every line of the 218 added files plus the `+` lines of RR and WG, 15,409 lines in all. It uses RV103's patterns. Evidence: `_run_records/publication_scan.py`, `publication_scan_summary.txt`, `publication_hits.tsv`, `token_shape_scan.py` and `token_shape_scan.txt`.

**Credentials: none.**
- **Token-shaped patterns** over every changed file in full find **0**: GitHub tokens of real length, `github_pat_` bodies, `sk-` keys, `AKIA` + 16, PEM blocks, password and token values, Authorization header values, Slack tokens and JWTs.
- The name patterns (`ghp_`, `gho_`, `github_pat_`, `sk-`, `AKIA`, `BEGIN .* PRIVATE KEY`, `password`, `secret`, `token=`, `Authorization:`, credential variables) hit only scan vocabulary and ordinary words:
  - RV103's records: its pattern tables, its summary, its review prose, its brief, and its own `publication_hits.tsv`. That file holds 1,744 hits that quote RV102's vocabulary. My hits file leaves them out and counts them in the summary.
  - `secret` ×3: test names in `R/I75/…/repair_tests.tsv` (for example "…quarantined/secret facts remain blocked").
  - `token=` ×18: keyword arguments in `IMPLEMENTATION/T6S/_draft_run_records/scripts/build_index.py` (`dict(token="RR decisions …")`).
  - `sk-` ×1 in RR's added text (RR:13215): prose about the "task-management" false positive.

**ROOT's pre-screen, checked independently: confirmed in substance.**
- No large file, no token-shaped string, and no machine-absolute path.
- **Host-data patterns hit three more places outside RV103's records, none of them host data:**
  - I79's `_run_records/tools/run_py.sh:7`, `pgrep -f "$WT/guard/memguard.sh" >/dev/null`. It checks that the memory guard is running and sends the output nowhere.
  - The agent co-author's no-reply address in `RECORDS_MERGE_2026-10-06B/_run_records/SQUASH_BODY.txt`.
  - The owner-name regex in RV103's committed `publication_scan.py`. It is the owner's configured Git identity.

**Personal data:**
- **0** e-mail addresses other than the agent no-reply address. The owner's name occurs only in that regex.
- The owner's configured identity appears as the commit author and committer. My records drop the e-mail from the PR metadata (`pr1105_view.json`).

**Whole-host data: none.**
- **0** hits for: UUIDs, tool-call or message ids, process-table headers, and `ps` or `lsof` invocations.
- The application, session, system-path and host-name hits are all RV103's or RV102's pattern tables and prose. I read each one.
- **The lock and job lines** are each TASK's own lock-wrapper lines: their own PIDs, `cwd=WT/…` and cargo arguments. They appear in I75's `oracle/rust_job.txt` and RV101's `addendum_01/suites/lock_lines.txt`, as accepted in RV101's and RV103's precedent. There is no process listing.
- **One host fact in RR:** "this Mac has 128 GiB of physical memory" (RR:13416). It is true (`sysctl hw.memsize` = 137,438,953,472 B), and it is a hardware size, not whole-host data.

**Size and type** (`_run_records/size_type_summary.txt`):
- The 218 added files total 1,536,587 B. The largest is 604,063 B (`R/I75/…/repair_tests.tsv`), and RR itself is 1,153,106 B. **No file is near 50 MB.**
- `file --mime-type` finds text only:
  - 175 plain text, 16 JSON, 10 Python and 6 shell;
  - 9 JavaScript, TypeScript or Rust sources read as Java, C or Algol, plus 1 diff;
  - 1 empty `tsc.log` (tsc printed nothing).
- **No build output:** no `target/`, `.wasm`, `.rlib`, `.so`/`.dylib`, `.pyc`, `__pycache__`, `node_modules/`, `dist/` or `build/` path.

## 3. Portability: PASS

**GEN-8 by E-4's method** (`_run_records/gen8_pytest.log`):
- The brief's test, run in a Git checkout of the exact head. The only checkout of H on this host is ROOT's PR worktree `WT/records-pr`.
- It ran read-only: `GIT_OPTIONAL_LOCKS=0`, `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`, and `TMPDIR` in my scratch.
- HEAD was H before and after, with 0 status entries (including ignored) before and after.

  ```
  VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8
  ```

- **The result: 1 passed, 10 deselected in 29.48s**, exit 0, 00:22:47–00:23:17Z, pytest 9.1.1.

**Machine-absolute paths** in the 220 changed files (`_run_records/abs_scan.py`, `abs_paths_by_file.tsv`):

| Check | Result |
|---|---|
| GEN-8's detector (`surface_roles.iter_machine_path_lines`), whole files | **0 lines** |
| GEN-8's detector, added lines | **0** |
| Broad pattern (user-home, private-temp, var-folders, tmp, Volumes, Linux home, drive letters, `~/` homes) | 4 added lines, all placeholders or quotations: RV103's `REVIEW.md:100` and `:142` (`/Users/<user>`), RV103's `abs_paths_by_file.tsv:4` (quoting RV102's rows), and RR:13215 (`/Users/<user>`, ROOT's screen) |
| **Machine-absolute paths in total** | **0** |

**The living documents have none:** RR's 296 appended lines, WG's diff and the 6 added briefs.

My own records also pass the GEN-8 detector (0 lines).

## 4. The living documents against the records and Git

Evidence: `_run_records/living_docs_git_checks.txt`, `ids_check.txt`, `heading_citations_check.txt`, `reserved_names_check.txt`, `s1_counts_vs_host.txt`, `dec025_t6s_host_vs_committed.txt`, `pr1103.json`, `pr1104.json`, `ci_runs_d953e12187.json`, `ci_dispatch_37546714187.json` and `ci_dispatch_target_base_excerpt.txt`.

### 4.1 #1104 → `bfb26596bf` at `d953e12187`, with its gates: confirmed

**GitHub and Git:**
- #1104 is MERGED at 00:17:29Z. Its merge commit is `bfb26596bf`, with parents `d8c88774d0` and `d953e12187` (= `PARENTS.txt`).
- `MERGE_COMMAND.txt` reads `gh pr ready`, then `--merge --match-head-commit d953e121…`. The base was `d8c88774d0`, the merge's first parent, so main had not moved.
- **Main's diff `M^1..M`:** 23 files, 13 A and 10 M, +3,177 / −1,903. That is the 19 slice files, whose blobs equal `fdcdb5e024`'s (0 differ), plus the 4 package files.

**CI on `d953e12187`** (= `CI_RUNS_d953e12187.txt`):
- The four automatic runs succeeded: pec-tests 37546715871, Harness Pre-merge 37546715912, governance-harness 37546715896 and Piping Desktop E2E 37546715857.
- **The dispatch 37546714187** succeeded on `d953e12187` with `target_base` = `d8c88774d0` in its selection log. Its jobs: Numerical cargo suite, Source remainder 1–4 and Desktop E2E all succeeded; accessibility was skipped by selection.

**The other gates:**
- `se.txt`: 5/5 PASS, |S| = 19, INT `0416b3b2ce`, MAIN/B `d8c88774d0`. RV101's ADDENDUM_02 reproduces it, against `53f626a5c8` too.
- `citations.txt`: PASS 2/0/0.
- `gen8.txt`: the head, the command, and 1 passed.

**DEC-025 (`dec025/`) against the host's own outputs** (`dec025_t6s_host_vs_committed.txt`):
- `meta.txt`, `quiet.log` and both suites logs are byte-identical to the host files. `surfaces.txt` differs only by the VENV placeholder.
- Base and candidate built in fresh targets: `WT/targets/T6S_d953e12187_base` and `…_cand`.
- The run held the T3 lock from 23:28:19Z to 00:16:58Z (`END rc=0`).
- **The counts equal the deltas ruled in advance (RR:13361–13365):**
  - suites: 39/40 identical, and `result_export` 173 → 177 (+4 ok);
  - pytest: 3,773 passed and 32 skipped, which is U8's 3,750 + 23;
  - vitest: 141/141 files and 3,612/3,612 tests, which is 3,574 + 38;
  - both builds exit 0.
- **The sweep's stop** is shown on the host: `sweep.log` fails at `s11g_tests::t13_committed_fallback_uz_is_byte_identical`.
- **RV101 A2-N4 ("every T6S test file passes on the combined tree"):** supported by the totals. Every vitest file passed, and the pytest skip count is unchanged at 32. The host vitest log has no per-file listing, so the evidence is the totals.

**NUM absorbed main** as `25c4fa3e6a`, with parents `22438afd01` and M. Its tree equals its first parent's, and its non-execution tree equals main's. **NUM carries no unmerged product slice** at N.

### 4.2 #1103 → `d8c88774d0` (squash): confirmed

- GitHub: MERGED at 23:19:13Z. The head was `3f8a405c33`, and the squash `d8c88774d0` has a single parent, `f8ed4f0551`.
- **The squash's tree equals the PR head's**, and both equal `ea0e288e8a`'s (`4457bbedf0…`). It changes 104 A and 2 M, with 0 paths outside `execution/`.
- **The commit body equals `SQUASH_BODY.txt` byte for byte.** It carries RV103 N-6's correction ("RV97's ADDENDUM_02").
- The four runs on `3f8a405c33` succeeded (37498561195, 37498561249, 37498560972 and 37498561197), as `CI_RUNS_3f8a405c33.txt` records.
- The absorbing merge `8afdf47153` has parents `8eaa4403a6` and `d8c88774d0`, and its tree equals its first parent's (RR:13264).

### 4.3 RV103's S-1 fix: confirmed on the host

`IMPLEMENTATION/U8_MERGE/dec025/operation_applier_shared_target_counts.txt` is sealed by `SHA256SUMS.addendum_01` (1/1). The sealed `SHA256SUMS` is untouched (28/28). The extract names its host log with a WT placeholder, and the log's sha256 is `c2ec8ce9…`.

I could read that log (`WT/scratch/u9_dec025/U8_61c35f56a8/suites/007_core_model_operations_operation_applier_Cargo.toml.log`). **Its sha256 is `c2ec8ce9772f…`, exactly as recorded.** Every count line of the extract is identical to my own recount (`s1_counts_vs_host.txt`):
- 279 errors: 244 E0308, 26 E0277 and 9 E0631;
- 355 `serde_json` and 8 `serde_core` version notes;
- registry sources for serde_json 1.0.150/1.0.151 and serde_core 1.0.228/1.0.229, with the registry prefix redacted;
- 8 test targets that did not compile.

RR:13267–13272 states this correctly. RV103's other items are handled as RR:13273–13286 says:
- **N-1:** the cdylib enabler, the three writers, and the fresh-target rule for every DEC-025 suite run, partial reruns included.
- **N-3:** the `dec025_mac` seal is `78afef53…`, equal to the live host script.
- **N-5:** the three errata.
- **N-6:** the squash body (§4.2).

N-4's "may now be cleaned" is stale again (N-2 above).

### 4.4 The rulings since `ea0e288e8a`

RR appends 9 sections (296 lines), from "#1103 opened; …" to "#1104 merged: T6S is on main". Both heading-like quotations in them, and all 22 in WG's T3 section, resolve to existing RR headings (`heading_citations_check.txt`).

**The owner's decision on M.** RR:13411 records it as: **"You can raise the memory ceiling to 6.0 GiB without asking."** WG:585 quotes the same words.
- The arithmetic is exact: 3.75 GiB = 4,026,531,840 B, and 6.0 GiB = 6,442,450,944 B.
- RR:8887 is D-7's definition of M.
- **The owner-held remainder is kept** in RR:13418–13420, WG:579 and DESIGN_v2 decision 17: M above 6.0 GiB, and any supported-machine statement.
- I82's brief (`B1_S_CAP_STUDY.md`) sends anything above 6.0 GiB to the owner.
- The interpretation is N-1.

**B0's selection (RR:13429–13477), with no owner-held decision taken by ROOT:**
- **The revision was in hand before the selection:**
  - DESIGN_v2 `5933b90b…` and REVISION_01 `79520623…` (2/2 OK) were committed in `77bac7532f`;
  - RV105's ADDENDUM_01 `9071403f…` (2/2 OK) CONFIRMS them, 0/0/3;
  - the selection followed in `e302ee8c64`.
- **The selected set is exactly DESIGN_v2 §8's recommendation:** decisions 1–16, 20 and 21. Every one of them has ROOT as its decider in §8.
  - Decisions 17 (M above 6.0 GiB, or a supported-machine statement), 18 (R-2) and 19 (the native-app witnesses) are marked **Owner** and stay "prepared, not decided" (RR:13447–13450).
  - **Decision 16's caps are model caps** (c, l, Σl and D1.9's rows), not the owner-held dense-scrutiny and observation-lane ceilings. Its M lever stops at 6.0 GiB.
  - **Decision 1 leaves PHYS-R4's publication and availability unchanged.** Decision 10 leaves observation framing unchanged.
  - RV105 found that the deciders match the owner-held list (REVIEW.md §6) and that decisions 16 and 17 match the owner's decision (ADDENDUM_01 §5).
- **"RV105 AGREES with decisions 1, 2, 4, 5, 6, 9, 11, 14, 15, 16 and 21. It agreed earlier with 3, 7, 8, 10, 12, 13 and 20"** matches ADDENDUM_01 §7 and the unchanged rows.
- **The reserved names** have 0 hits outside `P/execution` at N: `operand_preparations`, `OperandPreparation`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1`, `NoTriggeredCase`, the full `physics-retained-1` form and `exact_straight_retained_w1a_v2`. `combination_operand` hits 7 lines, all inside `check_combination_operand_load_case` or `combination_operand_lengths` (`reserved_names_check.txt`).
- **The cited lines are right:** RR:9013 is F3's "fail-closed, held" R-b′ ruling (A1-N1's correction), and RR:2190 is "W1 is selected only for Sensitive and D-5-routed cases".
- **Wording slips:** N-3.

**The two notices for the owner's information** are both accurate:
- **RR:13404–13407 (decision 1).** In the dev/test build only, a `checks_passed` case no longer attempts W1 or appends a notice, including W6's PHYS-R4 geometry. PHYS-R4's publication and availability are unchanged. Its last line raised M, and the owner's decision answered it.
- **RR:13457–13461 (decisions 1 and 21).** Decision 21 is added as RV105 A1-N3 asked. For a Mechanism, Asymmetric or InvalidInput failure, "no longer gets a notice" with "no public change" matches RV105 §4: only a notice on a blocked envelope is removed, and the readers do not see it.

**RV105's findings are resolved as RR says:**
- B-1, S-1 to S-3 and N-1 to N-11 are confirmed in ADDENDUM_01 §§1–3.
- A1-N1 is corrected in RR's text, A1-N2 is kept as harmless, and A1-N3 is the decision-21 notice.
- I81's brief (`B1_0_PROBE.md`) records decision 21's tags, and says which witness or test changes.

**The other appended rulings, checked against the records:**
- **I75's REPAIR_01** (`143ada4e…`):
  - 0 of 123,607 words differ from Rust, against 6,395 before;
  - 69 vectors, 430/430 T6S tests, 3,590/3,590 desktop tests, tsc clean, and 9/9 mutants;
  - the example word `4308628432e3716a` is exactly 857964921253421.25, a 16-digit tie;
  - `fdcdb5e024` changes exactly the two files, and T6S's 19 files overlap nothing main changed after `c1bfc460fc` (0 paths).
- **RV101's ADDENDUM_01** (`461507ab…`) has 185,401 words with 21,997 and 10,788 ties. **ADDENDUM_02** (`f3567235…`) is CONFIRMED at 0/0/4.
- **RV103's REVIEW.md** is `976d39f6…` (29/29).
- **I79's RETURN.md** (`44d82fc8…`; 31/31):
  - the branch is at `966113396e` on origin, three commits over `f8ed4f0551`;
  - the numstat is lib.rs +353/−3, the runner test +233, and the `.py` +5/−3;
  - the 503 changed panics are 18 + 276 + 209;
  - `e74a69c4…` reproduces;
  - the evaluator goes 49 → 55, the runner 33 → 35, and 10/10 mutants are killed.
- **"That is how S-I1 was treated"** (RR:13346) agrees with `S_I1_MERGE/RECORD.md`: its gate table has no separate full-suite row, and its DEC-025 has 40 manifests.

### 4.5 WG's T3 section: confirmed except N-4

**The positions (WG:549–556):**
- U8 is on main as #1102, and **T6S as #1104 `bfb26596bf`**.
- The records are on main through #1103 (`d8c88774d0`, squash).
- SI1b is at `966113396e`, under RV104.
- B0 is selected, and B1 has opened with I81 and I82.
- NUM carries main `bfb26596bf` with no unmerged product slice: N's non-execution tree equals M's.
- The host line includes the every-suite-run fresh-target rule.

**The table rows** (SI1b, SI1c, T6S, B0 and B1/B6) agree with RR's sections and the records above. The one stale clause is in N-4.

**The owner-held list (WG:573–582)** changes only in its M line: "any supported-machine statement of M, or M above 6.0 GiB (ROOT may select M up to 6.0 GiB: owner, 2026-10-06)". Every other item is unchanged.

**The owner decisions in force (WG:584–593)** add "2026-10-06: M up to 6.0 GiB", with the owner's words as RR records them.

**The next unused IDs at N are I83 and RV106** (`ids_check.txt`):
- At N, I83 occurs only in RR:13477 and WG:595, and RV106 only in RR:13333, RR:13477 and WG:595.
- I84 and RV107 occur nowhere.
- I81 and I82 are dispatched: their briefs are in H, and they have no record folders yet. RV104 is dispatched too: its brief is in H.

**The rulings-in-force list** gains four entries, each naming an existing heading. **The Next safe action** list matches RR:13501–13504. Its cleanup list rightly omits `records-pr` (N-2).

## 5. Integrity: PASS

**The sum files**, verified against a `git archive` of H (the in-scope T folders, `_run_records/sums_verify.py`, `sums_verify.tsv`). "Uncovered" counts files in the folder that none of its sum files lists.

| Folder (T-relative) | Sum file(s) | Result | Uncovered |
|---|---|---|---|
| `IMPLEMENTATION/T6S/` | SHA256SUMS (the package, on main) | 3/3 OK | 0 (`_draft_run_records/` is sealed by its own) |
| `IMPLEMENTATION/T6S/_draft_run_records/` | SHA256SUMS | 17/17 OK | 0 |
| `IMPLEMENTATION/T6S_MERGE/` | SHA256SUMS | 17/17 OK | 0 |
| `IMPLEMENTATION/RECORDS_MERGE_2026-10-06B/` | SHA256SUMS | 5/5 OK | 0 |
| `IMPLEMENTATION/U8_MERGE/` | SHA256SUMS; SHA256SUMS.addendum_01 | 28/28; 1/1 OK | 0 |
| `IMPLEMENTATION/SESSION_2026-10-06/` | SHA256SUMS; SHA256SUMS.dec025_mac | 2/2; 1/1 OK | **0** (RV103 N-3 closed) |
| `R/I75/t6s_01/` | SHA256SUMS; REPAIR_01.SHA256SUMS | 56/56; 54/54 OK | 0 |
| `R/I78/b0_contract_01/` | SHA256SUMS; SHA256SUMS.revision_01 | 1/1; 2/2 OK | 0 |
| `R/I79/si1b_01/` | SHA256SUMS | 31/31 OK | 0 |
| `R/REVIEW_RV101/t6s_01/` | SHA256SUMS; ADDENDUM_01.SHA256SUMS; ADDENDUM_02.SHA256SUMS | 54/54; 17/17; 13/13 OK | 0 |
| `R/REVIEW_RV103/records_01/` | SHA256SUMS | 29/29 OK | 0 |
| `R/REVIEW_RV105/b0_01/` | SHA256SUMS; SHA256SUMS.addendum_01 | 9/9; 2/2 OK | 0 |

That is 19 sum files and 342 entries, with no entry missing or bad. The 14 sum files this PR adds all lie in these folders.

**RR is append-only: PASS** (`_run_records/rr_append_only.txt`).
- Main's RR (1,129,719 B, sha256 `5f39fea6…`, 13,208 lines) is an **exact byte prefix** of H's (1,153,106 B, `34d1838a…`, 13,504 lines).
- H appends 23,387 B in 296 lines: nine sections.
- H's blob equals N's.

**The 13 redacted originals: PASS** (`_run_records/redactions_check.py`, `redactions_check.txt`).
- For each `REDACTIONS.json` entry, the blob at `dfa5e2dc44` hashes to `original_sha256`, and H's blob hashes to `redacted_sha256`.
- **0 of the 13 original blobs** are in H's tree (65,650 distinct blobs) or in M's.
- 0 of the 220 changed files hash to an original.

## 6. Gate evidence: PASS (the records-only gate set)

The records-only gates are GEN-8, the PR's automatic CI and an independent review. This PR changes no product, test, CI or portability-policy path, and nothing under T's `REFERENCES/` or `DESIGN_NUMERICS/`.

| Gate | Evidence | Exact head? | Result |
|---|---|---|---|
| GEN-8 (ROOT) | NUM `IMPLEMENTATION/RECORDS_MERGE_2026-10-07/_run_records/gen8.txt` (at `cc0c9f4e4e`): head `736f3fb7b2…`, the command, "cwd: records-PR checkout root", "1 passed, 10 deselected in 29.14s". It is on NUM after N, not in #1105 | H | **pass**; it records the head and the command, as E-4 requires |
| GEN-8 (RV106) | §3 | H | **pass** |
| governance-harness | 37551425229, pull_request on the merge ref `b2065853e` (H into M), `CHIRALITY_REQUIRE_LIVE_TESTS: 1`, "1156 passed, 48 subtests passed" | H | **success** |
| Harness Pre-merge Validation | 37551425131 (Select App coverage and Harness pre-merge succeeded; the App jobs were skipped by selection) | H | **success** |
| pec-tests | 37551425158 (Select PEC coverage and pec succeeded; the workspace tests were skipped by selection) | H | **success** |
| Piping Desktop E2E | 37551425087 (Select source coverage and Desktop E2E source mode succeeded; the numerical, remainder and accessibility jobs were skipped by selection) | H | **success** |
| Independent review | this report | H | **PASS** |

## What ROOT must rule on

1. **N-2, before any cleanup `apply`:** `WT/records-pr` is #1105's only checkout and must survive until #1105 merges.
2. **N-1:** whether to record the question the owner answered on M, and to restate that the dense-scrutiny and observation-lane ceilings are unchanged and owner-held. I recommend a one-line append. Nothing needs undoing.
3. **N-3 and N-4** are errata or wording for the next RR append or WG touch.
4. **N-5** is for the squash body.
5. **Before the merge:**
   - mark #1105 ready;
   - check that main is still `bfb26596bf`;
   - run `gh pr merge 1105 --squash --match-head-commit 736f3fb7b27154732b27b716152d7446e321acf9` with an explicit subject and body.

## Host and method

- **My copy:** a `git archive` of H's in-scope T folders in `WT/rv106/` (149 MB): `IMPLEMENTATION/`, `R/BRIEFS/`, and R's I75, I78, I79, RV101, RV103 and RV105 folders. It was used for the sums, sizes and types, and is deleted at the end. Logs and scripts are in `WT/scratch/rv106_records_01/`, with `TMPDIR` there.
- **Reads:**
  - NUM's object store with `GIT_OPTIONAL_LOCKS=0`: `log`, `diff`, `ls-tree`, `show`, `merge-base`, `grep` and `ls-remote`;
  - `gh` read-only for the PRs, runs, jobs and job logs;
  - host files read-only: the U8 and T6S DEC-025 outputs under `WT/scratch/u9_dec025/`, the lock log `WT/guard/cargo_jobs.log`, a listing of `WT/targets/`, and `sysctl hw.memsize`.
- **The one departure from "your copy".** GEN-8 ran in ROOT's PR worktree, because E-4 needs a Git checkout of the exact head (RV103, RV102 and RV100 precedent). It was clean, with HEAD unchanged, before and after. I wrote nothing there.
- **What I ran:** one invocation of the single GEN-8 pytest, plus read-only Python scans and hashing. No other test, cargo, native work, install or Git write. Nothing went to the system temp directory.
- **The scripts** `publication_scan.py`, `token_shape_scan.py` and `abs_scan.py` are RV103's, with only the docstring changed. `sums_verify.py` and `redactions_check.py` follow RV103's.

## Evidence index (`_run_records/`)

- **Scope:** `scope_checks.txt`, `changed_paths_name_status.tsv`, `pr1105_view.json` (identity e-mails dropped), `pr1105_body.md` and `open_prs.txt`.
- **GEN-8:** `gen8_pytest.log`.
- **Machine paths:** `abs_scan.py` and `abs_paths_by_file.tsv`.
- **Publication screen:** `publication_scan.py`, `publication_scan_summary.txt`, `publication_hits.tsv` (home roots written `/U-sers/`; RV103's nested hits counted, not copied), `token_shape_scan.py`, `token_shape_scan.txt` and `size_type_summary.txt`.
- **Sums:** `sums_verify.py` and `sums_verify.tsv`.
- **RR:** `rr_append_only.txt` and `heading_citations_check.txt`.
- **The originals:** `redactions_check.py` and `redactions_check.txt`.
- **Living documents:** `living_docs_git_checks.txt`, `ids_check.txt`, `reserved_names_check.txt`, `pr1103.json` and `pr1104.json`.
- **DEC-025 and S-1:** `dec025_t6s_host_vs_committed.txt` and `s1_counts_vs_host.txt`.
- **CI:** `ci_runs_H.json`, `ci_jobs_H.txt`, `ci_governance_harness_H_excerpt.txt`, `ci_runs_d953e12187.json`, `ci_dispatch_37546714187.json` and `ci_dispatch_target_base_excerpt.txt`.
