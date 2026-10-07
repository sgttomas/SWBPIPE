# RV103: independent review of the T3 records-only PR after #1102 (#1103)

**Reviewer:** RV103, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of these records. 2026-10-06 UTC.

**Brief:** `R/BRIEFS/RV103_RECORDS_PR4_REVIEW.md` (sha256 `7a22129f…`, verified at the start and again before writing), read in full, with the repository root `AGENTS.md` (`f96feb19…`) and `agents/AGENT_TASK.md`. Precedent read first: RV102's `REVIEW.md` and `ADDENDUM_01.md` (#1101), and the merge record `IMPLEMENTATION/RECORDS_MERGE_2026-10-06/`. Rulings consulted in RR: the five sections appended by this PR, and the proportionate-CI, E-4 and consolidated gate-set rulings as RV102 cites them.

**Placeholders.** WT = the t3 workspace; NUM = WT/numerics; P = `projects/chirality-piping`; T3 = P/execution/…/NUMERICAL_INTEGRITY_T3; R = T3/RESUME_2026-09-30; RR = T3/ROOT_RULINGS_V1.md; WG = P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md; VENV = the piping venv's Python 3.13.14.

**Resumption.** ROOT relayed mid-review that NUM had moved to `ecb541d63f` (records only, not in #1103) and that the review stands against `ea0e288e8a`. Every check below is against N = `ea0e288e8a` and the PR head H; my partial state was intact and was continued, not redone.

## The candidate

| Item | Value |
|---|---|
| PR | #1103, `codex/piping-t3-records-20261006b` → `main`, **draft**, MERGEABLE (merge state BLOCKED while draft) |
| H (head) | `3f8a405c3352da3eba8c4a3b1132abb9d747b2bf`, one commit, sole parent M |
| M (base, main) | `f8ed4f055126cf19315d2d8b06d7d11796786a82` (#1102's merge); origin's main is still M |
| N (source, NUM) | `ea0e288e8a5a7ac2e985a50656c940aa8e9047ee` |
| Author and committer | the owner's configured Git identity; the only other identity is the agent co-author trailer |

## Verdict: **PASS** (no blocking finding)

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 6 |

The following pass:
- **Scope.** H is M plus one commit, not from NUM's history. **H's whole tree equals N's** (tree `4457bbedf0…`). 106 paths change: 104 A, 2 M, 0 D, all under `P/execution/`. The modified paths are exactly RR and WG.
- **Publication screen.** No credentials, personal data, whole-host data, binaries or large files.
- **Portability.** GEN-8 passes on the exact head (mine and ROOT's). The changed files hold 0 machine-absolute paths; the living documents hold none.
- **Integrity.** Every sum file in scope verifies from the committed tree; RR is append-only; none of the 13 redacted originals is in H.
- **Gates.** The four automatic CI runs succeeded on H.
- **DEC-025's shared-target account.** Its cause, the rerun's sufficiency and "no earlier result is reopened" are supported in substance (§4.3).

One thing should be fixed, and it need not block this PR:
- **S-1:** U8's merge record attributes the first pass's error counts and rustc's version notes to a committed excerpt that contains neither. They are true: I verified them on the host log.

## Findings

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | `IMPLEMENTATION/U8_MERGE/RECORD.md:24` (and RR:13191, same content) | RECORD says the first pass had "279 errors (244 E0308, 26 E0277, 9 E0631)" and that "rustc's notes name two versions of `serde_json` (1.0.150 and 1.0.151) and of `serde_core` in one build (`operation_applier_shared_target_excerpt.txt`)". **The cited excerpt (24 lines) holds three E0308 snippets ("expected `serde_json::value::Value`, found `Value`") and nothing else:** no error tally, no version number, no `serde_core`. No other committed file in `U8_MERGE/` carries them. **On the host the claim is exactly true** (`_run_records/dec025_host_vs_committed.txt`). The host log `WT/scratch/u9_dec025/U8_61c35f56a8/suites/007_…operation_applier….log` has 279 `error[` lines (244/26/9), 355 serde_json "multiple different versions" notes naming 1.0.150 and 1.0.151, and 8 serde_core notes (1.0.228 and 1.0.229). The excerpt's 21 numbered lines match that log at their line numbers. The notes were probably left out because each carries a cargo-registry path under the user's home. | In the next records PR, add a placeholder-redacted extract to `U8_MERGE/dec025/`: the tally line, one serde_json note and one serde_core note, with the home prefix written as `<cargo-home>`. Seal it with its own sum file (for example `SHA256SUMS.addendum_01`, as RV102 did) so the sealed `SHA256SUMS` is untouched. Point an RR append at it. #1103 itself needs no change. |
| N-1 | NOTE | RR:13192 and RR:13199; `U8_MERGE/RECORD.md:25` (the mechanism) | **The cause is right but incompletely stated** (`_run_records/lockfile_check.txt`):<br>- **The enabler is not named.** `operation_applier`'s `[lib] crate-type = ["cdylib", "rlib"]`, so cargo writes its library without a hash suffix. The shared target holds a single `libopen_pipe_stress_operation_applier.rlib` (and `.dylib`) that every workspace building it overwrites. It was last written at 16:05:29Z, the end of U8's shared-target suites.<br>- **There are three writers, not two.** Besides 004 `self_weight_wasm` (1.0.151) and 007 `operation_applier` (1.0.150), 038 `physics_audit_regression` depends on it with 1.0.151.<br>- **The records do not say why earlier persistent-target runs passed.** SI1 (`20e7e3e5a2`), H1088 and R3 all show `operation_applier` 194/0 in that target, and SI1's and #1088's heads have the same 1.0.150/1.0.151 lockfile pair. The trigger depends on state: whether 007's own unit is still fresh when a 1.0.151 writer was last. I could not establish the exact sequence read-only.<br>**The ruling's conclusions still hold:** the rerun suffices, and no earlier result is reopened (§4.3). | At the next RR append, record the cdylib enabler and the third writer. Rule that any partial or targeted suite rerun also uses a fresh target. One manifest re-invoked into an already-used target, after 004 or 038 ran there, can hit the same mix. |
| N-2 | NOTE | RR:13161 (the lock-overlap times); RR:13187 (the sweep's stop at `t13`) | **Both rest on host files that are not committed.**<br>- The quiet-wait times (activity until 14:07:48, quiet until 14:09:48Z, then the baseline) come from SI1's `quiet.log`, and `S_I1_MERGE/dec025/` does not carry one.<br>- U8's sweep stop is shown in committed form only as `meta.txt`'s "sweep exit 1".<br>I checked both on the host: SI1's `quiet.log` reads exactly as ruled, and U8's `sweep.log` fails at `s11g_tests::t13_committed_fallback_uz_is_byte_identical`. RV101's own account (its `REVIEW.md` §8) puts its last unlocked job at about 14:05Z, so the "no overlap" conclusion holds. Leaving out `sweep.log` follows `S_I1_MERGE`'s precedent. | None required. Optionally, later DEC-025 records commit `quiet.log` (it has no machine paths) and a one-line sweep-stop extract. |
| N-3 | NOTE | `IMPLEMENTATION/SESSION_2026-10-06/host_tools/dec025_mac.sh.txt` (new, mode 100755) | **The file is not covered by its folder's sum file.** `SESSION_2026-10-06/SHA256SUMS` (on main since #1101) lists 2 of the 3 files, and RR:13198 cites the copy without a hash.<br>- The copy (sha256 `78afef53…`) equals the live `WT/scratch/u9_dec025/dec025_mac.sh` byte for byte.<br>- Its diff from the HANDOFF copy is exactly the 2 comment lines and the per-label target.<br>- Mode 100755 matches the HANDOFF `host_tools/` copies. | In the next records PR, add a sum file beside the sealed one (for example `SHA256SUMS.dec025`), or cite `78afef53…` in RR. |
| N-4 | NOTE | WG:628 ("Next safe action" 4) | **The line lists `records-pr` among the merged worktrees to clean up.** That was true at N, but since `e61ee13040` `WT/records-pr` holds #1103's head. It is the only Git checkout of H, and GEN-8 ran there for ROOT and for me. | Keep `records-pr` out of any cleanup `apply` until #1103 merges. Reword at the next WG touch. |
| N-5 | NOTE | RR:13144, RR:13157, RR:13154 | **Three wording details in "RV101 passes the T6 slice; …":**<br>- The heading "SF-1 repaired by I75" reads as done. At N the repair was only dispatched: WG:562 (the T6S row) says "I75 repairing SF-1", NUM's own commit `bb46d041e8` says "SF-1 to I75", and the repair records arrive after N (`22a79f7d7c`, `ecb541d63f`). The body ("is repaired before the PR …; RV101 confirms") reads as a ruling.<br>- "by I75 (the owner)" can be read as the human owner. RV101 says nothing about ownership.<br>- "refuses all 651 of RV101's mutations": RV101 made 693 mutations. 651 are the ones the version file refuses, and the dispatcher refuses all of those (RV101 `REVIEW.md`, headline). | RR is append-only. Clarify in the next append, which also records I75's repair and RV101's confirmation. |
| N-6 | NOTE | PR #1103 description | **One overstatement.** It says the PR carries "RV97's addenda for #1102". Only `ADDENDUM_02` (8 files) is added; `ADDENDUM_01` reached main with #1101. The rest is accurate: 104 added, 2 modified, 0 deleted, the U8 and #1101 merge records, the RV101 and RV102 records, the SI1b brief and the host-tool copy. | Word the squash body "RV97's ADDENDUM_02". Mark the PR ready before merging. |

## 1. Scope: PASS

Evidence: `_run_records/scope_checks.txt` and `changed_paths_name_status.tsv`.

**Parentage:**
- H's only parent is M, and `rev-list --count M..H` = 1. Origin's main = M.
- H is not an ancestor of N or of NUM's later heads. **None of NUM's 744 commits in `M..N` is an ancestor of H.**
- M is an ancestor of N: NUM absorbed #1102 as `02b96aa3cc`.

**Equality with NUM:**
- **H's whole tree equals N's** (`4457bbedf0…`). `P/execution` = `112ff8e884…` on both sides, and `git diff N H` is empty.
- So the PR carries exactly NUM's execution tree at N. N's non-execution tree equals main's, so the non-execution diff against main is empty.

**Changed paths** (`git diff --no-renames M H`):
- **106 paths: 104 A, 2 M, 0 D**, all under `P/execution/`. 0 paths fall outside it.
- Modes: 105 regular, plus 1 executable (`SESSION_2026-10-06/host_tools/dec025_mac.sh.txt`, N-3). No symlinks.
- Added by folder:

  | Folder | Files |
  |---|---|
  | `IMPLEMENTATION/U8_MERGE/` | 21 |
  | `RECORDS_MERGE_2026-10-06/` | 5 |
  | `SESSION_2026-10-06/` | 1 |
  | `R/REVIEW_RV101/` | 55 |
  | `R/REVIEW_RV102/` (ADDENDUM_01) | 13 |
  | `R/REVIEW_RV97/` (ADDENDUM_02) | 8 |
  | `R/BRIEFS/SI1B_POINT_PATH_PANICS.md` | 1 |

**The modified paths are exactly the two expected:**

| Path | Main blob → head blob | Change | Lineage |
|---|---|---|---|
| RR | `4ecd3865c8` → `d46bb31436` | +100 / −0 | Main's blob = NUM's at `1720a5c06b`; byte prefix (§5) |
| WG | `d2aecd77fd` → `dab8b204f6` | +17 / −14 | Main's blob = NUM's at `1720a5c06b`. Only NUM commits change it after (`9b8a18a20c`, `bb46d041e8`, N), and the absorbing merges kept the tree. **No change of main's is reverted** |

## 2. Nothing that must not be published: PASS

The screen covers the text this PR adds: every line of the 104 added files plus the `+` lines of RR and WG, 8,424 lines in all. Evidence: `_run_records/publication_scan.py`, `publication_scan_summary.txt`, `publication_hits.tsv`, `token_shape_scan.py` and `token_shape_scan.txt`.

**Credentials: none.**
- The name patterns (`ghp_`, `gho_`, `github_pat_`, `sk-`, `AKIA`, `BEGIN .* PRIVATE KEY`, `password`, `secret`, `token=`, `Authorization:`, credential variables) hit only 2 files:
  - RV102's `_run_records/addendum_01/publication_hits_delta.tsv`;
  - RV102's `publication_scan_delta_summary.txt`.
- Every hit is scan vocabulary: RV102's own pattern table, its summary counts, its review prose and its brief. `secret` adds one prose line in RV102's `ADDENDUM_01.md`.
- **Token-shaped patterns** run over every changed file in full find **0**: GitHub tokens of real length, `github_pat_` bodies, `sk-` keys, `AKIA` + 16, PEM blocks, password/token values, Authorization header values, Slack tokens and JWTs.

**ROOT's pre-screen, checked independently:**
- The bare `sk-` occurs 28 times, in RV102's two files above and once in WG. The WG line is WG:515 ("task-management" in a link), which is in main's text (main's WG has the same 1 occurrence), not in the added lines.
- `/Users/<user>` occurs only in placeholder form, and RR:11998 is in main's prefix.
- **Confirmed.**

**Personal data:**
- **0** e-mail addresses and **0** occurrences of the owner's name in the added text.
- The owner's configured identity appears only as the commit author and committer. My own records drop the e-mail from the PR metadata (`pr1103_view.json`).

**Whole-host data: none.**
- **0** hits for: application paths, named non-toolchain apps, session or resume flags, UUIDs, tool-call or message ids, transcript paths, system paths, host names, process-table headers, and `ps`, `pgrep` or `lsof` invocations.
- The remaining app, session, system and host-name hits are again RV102's pattern table and review prose, which is vocabulary, not data. One such entry is RV102's pattern `claude-501`, the temp-directory form; it is a pattern string, not an observed path.

**RV101's 55 files, read with the host-lock caveat:**
- The one lock excerpt, `evidence/host/cargo_jobs_rv101.log` (74 lines), holds only RV101's own lock-wrapper lines: timestamps, WAIT/START/END, its own jobs' PIDs, `cwd=WT/rv101/…` and cargo arguments.
- One line is a CANCELLED record of its own job.
- There is no process listing, no other process, no app name and no session id.
- The tools use `subprocess` and `process.env` only for their own runs.

**Size and type:**
- The 104 added files total 1,183,197 B. The largest is 387,910 B (`dispatcher_oracle.jsonl`), and RR itself is 1,129,719 B. **No file is near 50 MB.**
- `file --mime-type` finds text only: 71 plain, 17 JSON, 9 shell, 5 TS/Rust sources read as Java, 2 Python and 2 diff.
- **No build output:** no `target/`, `.wasm`, `.rlib`, `.so`/`.dylib`, `.pyc`, `__pycache__`, `node_modules/`, `dist/` or `build/` path.

## 3. Portability: PASS

**GEN-8 by E-4's method** (`_run_records/gen8_pytest.log`):
- The brief's test, run in a Git checkout of the exact head. The only checkout of H on this host is ROOT's PR worktree `WT/records-pr`.
- It ran read-only: `GIT_OPTIONAL_LOCKS=0`, `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`, and `TMPDIR` in my scratch.
- HEAD was H before and after, with 0 status entries (including ignored) before and after.

  ```
  VENV -m pytest -q -p no:cacheprovider -rA tools/practitioner_harness/test_live_baseline.py -k gen8
  ```

- **The result: 1 passed, 10 deselected in 28.65s**, exit 0, 16:50:56–16:51:25Z, pytest 9.1.1.
- Git printed one warning about a self-referential symlink, `R/REVIEW_RV58/…/fixture/.gitignore`. It is on main already and is not this PR's.

**Machine-absolute paths** in the 106 changed files (`_run_records/abs_scan.py`, `abs_paths_by_file.tsv`):

| Check | Result |
|---|---|
| GEN-8's detector (`surface_roles.iter_machine_path_lines`), whole files | **0 lines** |
| GEN-8's detector, added lines | **0** |
| Broad pattern (user-home, private-temp, var-folders, tmp, Volumes, Linux home, drive letters, `~/` homes) | 8 lines: 7 in added text, all placeholders or quotations inside RV102's ADDENDUM_01 evidence (`/Users/<user>` ×6, the `~/dev/…` form quoted from RV102's N-5 ×1); 1 is RR:11998, in main's prefix |
| **Machine-absolute paths in total** | **0** |

**The living documents have none:** RR's 100 appended lines, WG's diff and the added brief `SI1B_POINT_PATH_PANICS.md`.

My own records also pass the GEN-8 detector (0 lines).

## 4. The living documents against the records and Git

Evidence: `_run_records/living_docs_git_checks.txt`, `ids_check.txt`, `lockfile_check.txt`, `dec025_host_vs_committed.txt`, `pr1102.json`, `ci_runs_f7a7572e35.json`, `ci_dispatch_*`.

### 4.1 #1102, its merge and its gates: confirmed

- **GitHub:** #1102 is MERGED at 16:41:07Z. The merge commit is `f8ed4f0551` and the head is `f7a7572e35`. Its parents are `d069e7130c` and `f7a7572e35` (= `PARENTS.txt`).
- **The merge command** in `MERGE_COMMAND.txt` is `--merge --match-head-commit f7a7572e35…`, preceded by `gh pr ready`.
- **Main's diff** `M^1..M`: 11 files, +29,864 / −14 (U8's 7 plus the 4 package files), as RECORD says.
- **CI on `f7a7572e35`:**
  - The four automatic runs succeeded: Piping Desktop E2E 37489440331, pec-tests 37489440345, Harness Pre-merge 37489440330 and governance-harness 37489440320.
  - **The dispatch 37489501933** succeeded on that head, with `target_base` = `d069e7130c` in its selection log. Its **Numerical cargo suite** job succeeded and ran `operation_applier`'s tests `--locked` on a fresh runner.
  - All of this equals `CI_RUNS_f7a7572e35.txt`.
- **Source equality and citations:**
  - `se.txt`, `se2.txt` and `se3.txt` are 5/5 PASS at the three heads. The last has INT `55f5b7ecf1` and MAIN/B `d069e7130c`.
  - `citations*.txt` read PASS 10/0/0.
- **GEN-8:** `gen8*.txt` record each head with the command, 1 passed.
- **RV97's ADDENDUM_02** (PASS 0/0/1, sha256 `8d301e00…`) was committed on NUM at 15:45Z, before the merge.
- **The carry-over premise** holds:
  - `61c35f56a8..f7a7572e35` and `75a8c3291f..d069e7130c` differ only under `P/execution/`;
  - #1101 has no path under `REFERENCES/` or `DESIGN_NUMERICS/`.
- **The absorbing merges:**
  - `55f5b7ecf1` has parents `9b8a18a20c` and `d069e7130c`. Main's RR and WG blobs equal NUM's at `1720a5c06b`, and NUM's maintained diff from main is exactly 7 files.
  - `f7a7572e35` has parents `b18dd4f369` and `d069e7130c`, with a non-execution diff of 0 from `b18dd4f369`.
  - `02b96aa3cc` has parents `b9dc450c1f` and M. Its tree equals its first parent's, and its non-execution tree equals main's.

### 4.2 IDs, dispatches and WG's T3 section: confirmed except N-4

**The next unused IDs at N are I80 and RV103** (`ids_check.txt`).
- At N, I80 and RV103 occur only in the two "next unused" statements (RR:13208, WG:592) and in RV102's earlier ID line, and nothing uses RV104. I81's only match is a gzip binary.
- I78 and I79 are named as dispatched: B0's brief is on main, and SI1b's is added here.
- The branch `codex/piping-t3-si1b-20261006` exists at `f8ed4f0551`, as do the worktree `WT/s-i1b` and the brief's lines 37–41.

**WG's T3 rows agree with RR and Git:**
- U8 MERGED as #1102 `f8ed4f0551`, with "L = 0 publishes; W-C1 is Ceiling" (RR:12344);
- #1101 `d069e7130c`, squash (GitHub: merged 15:38:38Z, head `93d15f1bd3`, single parent `75a8c3291f`);
- T6S "RV101 PASS (0/1/10); I75 repairing SF-1" (RV101's table);
- B0 to I78 and SI1b to I79;
- "NUM … carries main `f8ed4f0551` … with no unmerged product slice" (the non-execution trees are equal);
- the fresh DEC-025 target (`host_tools/dec025_mac.sh.txt`);
- the Assignment IDs line, with RV100 placed on 2026-10-05 (RV102's A-2);
- the new rulings-in-force entry and the parked RV101 notes (RR:13164–13166).

**The #1101 merge record:**
- `RECORDS_MERGE_2026-10-06/` agrees with GitHub: squash `d069e7130c`, head `93d15f1bd3`, 758 A / 3 M against `75a8c3291f`.
- Its four H3 runs succeeded on `93d15f1bd3` (37488176306, 37488175882, 37488175982 and 37488176537).
- Its GEN-8 files record H, H2 and H3.

**Not supported or stale:** N-4 (WG's cleanup list) and N-5 (RR's wording).

### 4.3 The ruling "#1102 merged; DEC-025's suites rerun in a fresh target; …"

**The committed `dec025/` files are the host's own outputs** (`dec025_host_vs_committed.txt`):
- `compare_shared_target.txt`, `compare.txt`, both suites logs, `meta.txt`, `suites_fresh_meta.txt`, `quiet.log` and `suites_main_baseline.log` are byte-identical to the host files.
- `surfaces.txt` differs only by the VENV placeholder.

**What the records state, confirmed:**
- **The ALL-DONE run:** 16:25:56Z on `61c35f56a8`, after a quiet wait and a baseline of `75a8c3291f` from 15:42:13Z.
- **pytest:** 3,750 passed, 32 skipped. That is S-I1's 3,733 plus 17.
- **vitest:** 138 files, 3,574 tests. That is S-I1's 3,552 plus 22.
- **Builds:** both exit 0.
- **The first comparison:** 37/40 identical, with `operation_applier` at rc 101 and 0 tests against the baseline's 194/0.
- **The fresh rerun:**
  - It ran 16:27:57Z–16:40:24Z under the T3 lock (lock log: `START ROOT suites-fresh U8_61c35f56a8 (fresh target targets/U8_61c35f56a8_cand_fresh; head 61c35f56a8…)`, `END rc=0`).
  - **38/40 identical.** The two differences are PP 705 → 708 (3 added ok) and result_export 172 → 173 (1 added ok).
  - `t13` fails on both sides, and `operation_applier` passes 194/0.
- **The baseline** ran in its own fresh target, `targets/U8_61c35f56a8_base`.
- **U8 touches no Cargo manifest, no lockfile and no `operation_applier` file:** 7 paths, 0 matches.

**Does the evidence support the cause it names? Yes, in substance.** Four facts point to it:
- rustc's notes name exactly the two `serde_json` versions of the two lockfiles: `operation_applier` 1.0.150 / serde_core 1.0.228, and `self_weight_wasm` 1.0.151 / 1.0.229, at `61c35f56a8`, `75a8c3291f` and M.
- `self_weight_wasm` depends on `operation_applier` by path.
- The same tree passes in a fresh target.
- The hosted Numerical cargo suite passed on `f7a7572e35`.

The committed excerpt shows the symptom but not the version notes or counts (S-1). The mechanism's enabler, its third writer and its state-dependence are unstated (N-1).

**Is the rerun sufficient? Yes.**
- It is the baseline's method on the same tree: one fresh target for the 40 manifests, in order.
- In a fresh target, 007 has no prior unit of its own, so it rebuilds the unhashed library under its own lockfile before its tests compile. The later 1.0.151 writer (038) then uses its own build consistently.
- The sweep, pytest, vitest and the builds do not receive the shared target. `dec025_mac.sh` passes it only to `run_suites_nff.sh`, as `CARGO_TARGET_DIR`, and `run_evidence_sweep.py` does not set one. So the ALL-DONE run's results stand for them.

**Does "no earlier result is reopened" hold? Yes, on a sharper basis than RR gives:**
1. All 40 manifests build from one checkout of the candidate. The shared library file is therefore only ever written from the current candidate's sources, or left alone when every writer judged those sources unchanged, so no stale source can be linked.
2. The only possible deviation is the serde resolution, and `operation_applier`'s API passes `serde_json::Value`. A mixed build therefore fails to compile rather than passing, as here.
3. Every earlier persistent-target run shows `operation_applier` 194/0, equal to its fresh-target baseline: SI1, H1088, R3, F and Fp.

### 4.4 Other appended rulings

- **"#1101 squash-merged; …":** RV102's ADDENDUM_01 is `32261a96…`, 12/12 OK, PASS 0/0/4 at `93d15f1bd3`. The merge facts are as in §4.2.
- **"#1102 absorbs main; …":** confirmed in §4.1.
- **"RV101 passes the T6 slice; …":**
  - RV101's `REVIEW.md` is `510fbdb5…`, 54/54 OK, with no machine paths.
  - Its verdict is PASS 0/1/10. The counts 891 steps, 69 fixtures, 63 statements and 10 of 13 mutants match RV101.
  - Wording details: N-5. The lock-overlap evidence: N-2.
- **"RV97 confirms #1102 at `f7a7572e35`; …":** ADDENDUM_02 is 7/7 OK, PASS 0/0/1, with note A2-N-1.

I found no contradiction among RR's appended sections.

## 5. Integrity: PASS

**SHA256SUMS**, verified against a `git archive` of H's `P/execution` (`_run_records/sums_verify.py`, `sums_verify.tsv`). "Uncovered" counts files in the folder that no sum file lists.

| Folder (T3-relative) | Sum file(s) | Result | Uncovered |
|---|---|---|---|
| `IMPLEMENTATION/U8_MERGE/` | SHA256SUMS | 28/28 OK | 0 |
| `IMPLEMENTATION/RECORDS_MERGE_2026-10-06/` | SHA256SUMS | 6/6 OK | 0 |
| `IMPLEMENTATION/SESSION_2026-10-06/` | SHA256SUMS | 2/2 OK | **1** (`dec025_mac.sh.txt`, N-3) |
| `R/REVIEW_RV97/u8_01/` | SHA256SUMS; ADDENDUM_01.SHA256SUMS; ADDENDUM_02.SHA256SUMS | 69/69; 10/10; 7/7 OK | 0 |
| `R/REVIEW_RV101/t6s_01/` | SHA256SUMS | 54/54 OK | 0 |
| `R/REVIEW_RV102/records_01/` | SHA256SUMS; SHA256SUMS.addendum_01 | 23/23; 12/12 OK | 0 |

No entry is missing or bad.

**RR is append-only: PASS** (`_run_records/rr_append_only.txt`).
- Main's RR (1,121,319 B, sha256 `b66c5bd0…`, 13,108 lines) is an **exact byte prefix** of H's (1,129,719 B, `5f39fea6…`, 13,208 lines).
- H appends 8,400 B in 100 lines: five sections, from "#1101 squash-merged; …" through "#1102 merged; …".
- H's blob equals N's.

**The 13 redacted originals: PASS** (`_run_records/redactions_check.py`, `redactions_check.txt`).
- For each `REDACTIONS.json` entry, the blob at `dfa5e2dc44` hashes to `original_sha256`, and H's blob hashes to `redacted_sha256`.
- **0 of the 13 original blobs** are in H's tree (80,596 entries) or in M's.
- 0 of the 106 changed files hash to an original.

## 6. Gate evidence: PASS (the records-only gate set)

The records-only gates are GEN-8, the PR's automatic CI and an independent review. This PR changes no product, test, CI or portability-policy path, and nothing under T3's `REFERENCES/` or `DESIGN_NUMERICS/`.

| Gate | Evidence | Exact head? | Result |
|---|---|---|---|
| GEN-8 (ROOT) | NUM `IMPLEMENTATION/RECORDS_MERGE_2026-10-06B/_run_records/gen8.txt`: head `3f8a405c33…`, the command, "cwd: records-PR checkout root", "1 passed, 10 deselected in 32.16s". It is on NUM after N, not in #1103 | H | **pass**; it records the head and command, as E-4 requires |
| GEN-8 (RV103) | §3 | H | **pass** |
| governance-harness | 37498561197, pull_request on the merge ref `7fa13139b` (H into M), `CHIRALITY_REQUIRE_LIVE_TESTS: 1`, "1156 passed, 48 subtests passed" | H | **success** |
| Harness Pre-merge Validation | 37498561195 (Select App coverage and Harness pre-merge succeeded; the App jobs were skipped by selection) | H | **success** |
| pec-tests | 37498560972 | H | **success** |
| Piping Desktop E2E | 37498561249 (Select source coverage and Desktop E2E source mode succeeded; the numerical, remainder and accessibility jobs were skipped by selection) | H | **success** |
| Independent review | this report | H | **PASS** |

## What ROOT must rule on

1. **S-1:** fix in the next records PR, as an additive, separately sealed extract plus an RR pointer, or by an RR append stating that the counts and versions were read from the host log. I recommend the first. **#1103 can merge as is.**
2. **N-1:** whether to record the cdylib enabler and rule that partial suite reruns also use a fresh target.
3. **N-4 before any cleanup `apply`:** `WT/records-pr` is #1103's checkout and must survive until #1103 merges.
4. **N-2, N-3, N-5 and N-6** are wording or sealing items for the next records PR or RR append. N-6 is the squash body.
5. **Before the merge:**
   - mark #1103 ready;
   - check that main is still `f8ed4f0551`;
   - run `gh pr merge 1103 --squash --match-head-commit 3f8a405c3352da3eba8c4a3b1132abb9d747b2bf` with an explicit subject and body.

## Host and method

- **My copy:** a `git archive` of H's `P/execution` in `WT/rv103/` (2.3 G), used for the sums, sizes and types. It is deleted at the end. Logs and scripts are in `WT/scratch/rv103_records_01/`, with `TMPDIR` there.
- **Reads:**
  - NUM's object store with `GIT_OPTIONAL_LOCKS=0`: `log`, `diff`, `ls-tree`, `show`, `merge-base`, a fetch of origin's main and the PR branch, and `ls-remote`;
  - `gh` read-only for the PRs, runs, jobs and job logs;
  - host files read-only: the U8 and SI1 DEC-025 outputs under `WT/scratch/u9_dec025/`, the lock log `WT/guard/cargo_jobs.log`, the `WT/tools/` scripts, and a directory listing of `WT/sweep-skewpin-target/debug/deps` and `.fingerprint`.
- **The one departure from the brief's "your copy".** GEN-8 ran in ROOT's PR worktree, because E-4 needs a Git checkout of the exact head and an archive under WT resolves to the enclosing repository (RV102 and RV100 precedent). The worktree was clean, with HEAD unchanged, before and after. I wrote nothing there.
- **What I ran:** one invocation of the single GEN-8 pytest, plus read-only Python scans and hashing. No other test, cargo, native work, install or Git write. Nothing went to the system temp directory.

## Evidence index (`_run_records/`)

- **Scope:** `scope_checks.txt`, `changed_paths_name_status.tsv`, `pr1103_view.json` (identity e-mails dropped) and `pr1103_body.md`.
- **GEN-8:** `gen8_pytest.log`.
- **Machine paths:** `abs_scan.py` and `abs_paths_by_file.tsv`.
- **Publication screen:** `publication_scan.py`, `publication_scan_summary.txt`, `publication_hits.tsv` (home roots written `/U-sers/`), `token_shape_scan.py` and `token_shape_scan.txt`.
- **Sums:** `sums_verify.py` and `sums_verify.tsv`.
- **RR:** `rr_append_only.txt`.
- **The originals:** `redactions_check.py` and `redactions_check.txt`.
- **Living documents:** `living_docs_git_checks.txt`, `ids_check.txt` and `pr1102.json`.
- **DEC-025:** `lockfile_check.txt` and `dec025_host_vs_committed.txt`.
- **CI:** `ci_runs_f7a7572e35.json`, `ci_dispatch_37489501933.json`, `ci_dispatch_target_base_excerpt.txt`, `ci_runs_H.json`, `ci_jobs_H.txt` and `ci_governance_harness_H_excerpt.txt`.
