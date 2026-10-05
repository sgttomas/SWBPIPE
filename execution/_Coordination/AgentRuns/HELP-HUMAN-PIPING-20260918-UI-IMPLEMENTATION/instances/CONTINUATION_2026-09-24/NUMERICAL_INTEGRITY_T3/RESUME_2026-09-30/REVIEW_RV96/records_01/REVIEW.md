# RV96: independent review of the T3 records-only PR (#1084)

**Reviewer:** RV96, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of these records.

**Brief:** `R/BRIEFS/RV96_RECORDS_PR_REVIEW.md` (read in full at NUM `29160bbc1c`), with the repository root AGENTS.md, the precedent record `T3/IMPLEMENTATION/RESUME_RECORDS_MERGE/RECORD.md` (PR1068, RV27), and RR's sections "U9 cut…" (the GEN-8 method) through "Handoff prepared; main absorbed; host cleanup; a records-only PR next".

**Placeholders.** WT = the t3 workspace; NUM = the T3 integration worktree (`codex/piping-numerical-integrity-20260926`); P = `projects/chirality-piping`; T3 = P/execution/…/NUMERICAL_INTEGRITY_T3; R = T3/RESUME_2026-09-30; RR = T3/ROOT_RULINGS_V1.md; VENV = the repository venv's Python 3.13.14. To avoid a clash with R, the PR head (the dispatch's "R") is called **H** here.

## The candidate

| Item | Value |
|---|---|
| PR | #1084, draft, `codex/piping-t3-records-20261005` → `main` |
| H (head) | `dfa5e2dc445984cefbdc3bdd9ca33bea0c7415c9` (one commit; sole parent = M) |
| M (base, main) | `e916ad17892d24fc0d577fc62edd1be22d8d73b4` |
| N (source, NUM) | `29160bbc1cdfa4f6f75b6f64c57a7c76ec904de7` |
| Author | the owner's configured Git identity |

`gh pr view 1084` at review time: head `dfa5e2dc44…`, base `e916ad1789…`, 6,585 changed files, MERGEABLE.

## Verdict: **FAIL**

| Severity | Count |
|---|---|
| BLOCKING | 1 |
| SHOULD-FIX | 1 |
| NOTE | 6 |

Scope (item 1) and sealed-record integrity (item 5) pass. **GEN-8 (item 3) fails on H**, and the hosted `governance-harness` check fails on H for the same reason. The defect is inherited from NUM: NUM `29160bbc1c` carries the same 263 findings, and main carries none.

## Findings

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| B-1 | BLOCKING | 263 files under T3 (list: `_run_records/gen8_findings.tsv`); the first is `T3/HANDOFF_2026-10-03_TO_NEXT_ROOT.md:23` | GEN-8 on H: **1 failed, 10 deselected** (`test_live_gen8_semantic_portability_invariants`; 261 `ABS_PATH_IN_UNCLASSIFIED_SURFACE` and 2 `ABS_PATH_IN_PROJECT_SURFACE` findings, 2,789 hit lines). A replay of GEN-8's classification on the git-tracked trees gives 263 for H, **0 for M**, and 263 for N. Hosted `governance-harness` run 37255803450 (pull_request, head H) fails: "1 failed, 1155 passed", "Left contains 263 more items", with the same first item. On main, the push run is the full suite with `CHIRALITY_REQUIRE_LIVE_TESTS=1`, so a merge would turn main's governance gate red. | **ROOT rules** (§3.4): (a) register the immutable, hash-bound records as `historical_role_overrides` / `control_path_exceptions` in `P/validation/portability_policy.json`, following precedent `f9ff31f163`. This adds one non-execution file to a records-only PR, so it needs a ruling. And/or (b) placeholder the machine paths in the living ROOT documents on NUM (RR:10932 and the three ROOT handoff files) with labelled corrections. Hash-binding a living file would raise `PORTABILITY_POLICY_HASH_DRIFT` at its next append. Then re-cut from NUM, and rerun GEN-8, hosted CI and the dispatch on the new head. A waiver alone is not viable. |
| S-1 | SHOULD-FIX | 12 whole-host process listings (`_run_records/process_listings.tsv`), 4 with full `ARGS`: `R/REVIEW_RV56/source_bridge_01/_run_records/{final,preflight}_processes.txt` and `R/REVIEW_RV56/source_bridge_repair_02/_run_records/{final,preflight}_processes.txt` | Each is a complete listing of about 1,040 processes on the owner's Mac. The ARGS listings expose the running application inventory (for example Teams, Messages, Mail, ChatGPT, Claude and Xcode), the apps' user-data directories, and local agent session identifiers (Claude `--resume` UUIDs and ChatGPT kernel `--session-id`). That goes beyond a directory layout. No credential was found. Main has 4 scoped listings ("piping-scope") and 29 files naming the same apps, but no whole-host ARGS listing. The files are bound in their folders' `INVENTORY.json` or `SHA256SUMS`. NUM is already public on origin. | **ROOT/owner disposition:** either accept publication explicitly, with a recorded acceptance, or replace the four ARGS listings with redacted copies under a supersession record that keeps the original hashes. Not merge-blocking once disposed. |
| N-1 | NOTE | 938 added files | Host paths of the form `/Users/<user>/…` appear in **938 of the 6,582 added files**. Main has 4,692 such files under P/execution by my pattern (the brief cites 4,688), and H has 5,630. Of the 3 modified files, none gains a `/Users/<user>/` host path; RR's new text uses the `<user>` placeholder. RR does gain one system-temp path at line 10932, which is part of B-1. Directory layout only, except as noted in S-1 and N-3. | None required. |
| N-2 | NOTE | `R/REVIEW_RV58/source_residual_01/_run_records/fixture/**` (104 entries, `_run_records/symlinks.tsv`) | 104 symlinks (mode 120000), all **absolute and dangling**, point to `<WT0>/.claude/t3/f2a-arithmetic/P/core/solver/frame_kernel/…`, a worktree since removed. They are the first symlinks in any execution tree (main has 0 there and 2 repo-wide). They are documented as references ("Symlinks are references, not immutable snapshots", `FIXTURE_README.md`) and counted in `SEAL.json` (`"symlinks": 104`). They are inert on any other clone. A tool that follows symlinks would read outside the repository only if that worktree path were recreated on the owner's host. | Accept as sealed evidence, or have ROOT rule otherwise. If ever replaced, use a supersession record, because SEAL.json counts them. |
| N-3 | NOTE | 11 added files (for example run `ENVIRONMENT`/`EXECUTION` JSON with `"host": …`) | The host's mDNS name (`<first-name>s-MacBook-Pro.local`, consistent with the owner's Git identity) appears in 11 added files; main has 1. | None required. Include it in the S-1 disposition if ROOT wants one decision. |
| N-4 | NOTE | `R/I65/u4_g7_03/SHA256SUMS` | 67/68 entries verify from Git. The one missing entry is `_run_records/__pycache__/delta_inventory2.cpython-313.pyc`, which is ignored by `P/.gitignore:4` (`__pycache__/`). The host copy on NUM matches the sealed hash (`a9371e4c…`). This is regenerable bytecode, not evidence. RV89's `u4_g7_03` (25/25) does not depend on it. | Optional: an erratum line, as `F2A_D1_MERGE/ERRATA.md` did. |
| N-5 | NOTE | RR:11996 | RR says the records PR "is recorded in `IMPLEMENTATION/RECORDS_MERGE_2026-10-05/` on NUM", but that folder does not exist at N or H. This is a forward reference, like PR1068's post-merge `RESUME_RECORDS_MERGE/`. | Create it with the post-merge record, or reword it at the next append. |
| N-6 | NOTE | method: GEN-8 "placement" on a `git archive` copy under WT | WT lies inside a Git worktree, so in the copy `git -C <copy> ls-files` resolves to the enclosing repository (prefix `.claude/t3/<copy>/`) and returns **0 paths, not None**. GEN-8 then walks only the active AgentRuns files, which is a subset of a real checkout's walk. The note in `R/I61/u9_freeze_01/_run_records/gen8_scratch.txt`, "a superset", does not hold there. For H, my replay against the real tracked set and the hosted run both agree on the same 263, so this verdict does not depend on the method. | For future placements, run with `GIT_CEILING_DIRECTORIES=<WT>` (verified: git then reports "not a git repository", so the walk is unrestricted as intended), or run on a real checkout. |

## 1. Scope: PASS

- **H's parentage.** H has one parent, M. `git merge-base M N` = M, so main is an ancestor of NUM.
- **Changed paths:** `git diff --no-renames --name-status M H` gives **6,585 paths: 6,582 A, 3 M, 0 D**. Every path is under `P/execution/`, and the diff outside `P/execution` is empty (0 paths).
  - Modes added: 6,406 regular, 72 executable, 104 symlinks.
- **Blob-for-blob equality with NUM.** `H:P/execution` and `N:P/execution` are the **same tree object, `8902eddb1c55f91084b7f21557457ba4c993dede`**, so every blob and mode is identical. M's tree is `3b0c9fe54bf8…`.
- **No re-additions.** None of the 6,582 added paths was ever deleted on main's history (there are 5,504 historically deleted execution paths; the intersection is empty).
- **Main's records modified by NUM.** Three main files change. For each, main's exact blob is present on NUM's first-parent line at `95934ffc41` (K6c records, 2026-10-02), and only NUM commits change it afterwards. Main has not touched them since `49034a940f` (#1071). No merge of main into NUM touched them (`git log --merges M..N -- <path>` is empty), so no change of main's is reverted.

| File | Main blob → H blob | NUM first-parent commits after `95934ffc41` | First … last |
|---|---|---|---|
| `R/ROOT_CURRENT.md` | `78d554d9` → `f4e088bd` (+33/−113, rewritten at handoff as RR records) | 99 | `8bc13fc5e9` … `29160bbc1c` (handoff to the next ROOT) |
| `RR` (`ROOT_RULINGS_V1.md`) | `87012ff9` → `699ff930` (+7,104/−0, append; §5) | 265 | `8bc13fc5e9` … `29160bbc1c` |
| `P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md` | `4c6dd132` → `410e86e3` (+18/−2) | 76 | `0774aa7193` … `f8f168f63b` (U8 plan ruled) |

## 2. Nothing that must not be published: PASS on credentials and size; S-1, N-1 to N-3 on host data

**Credentials.** The scan covered 6,481 PR paths (6,478 added regular files and the 3 modified files). Gzip payloads were decompressed, and the 195 members of `T3/IMPLEMENTATION/K6C_MERGE/dec025/_run_records/raw_gate_evidence.tar.gz` were extracted to scratch, giving 6,676 scanned units in all. Per-pattern dispositions are in `_run_records/secret_scan_summary.txt`.
- **Zero hits** for `ghp_`, `gho_`, `ghs_`/`ghu_`/`ghr_`, `github_pat_`, key-like `sk-…`, `AKIA…`, `BEGIN … PRIVATE KEY`, `Bearer <token>`, Slack tokens, and credential environment variables (`GH_TOKEN`, `GITHUB_TOKEN`, `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `AWS_SECRET`, `SSH_AUTH_SOCK`).
- Every `Authorization:` hit in the tarball's GitHub Actions logs is GitHub-masked (`AUTHORIZATION: basic ***`).
- `password` hits are the macOS process name `PasswordBreachAgent`; `secret` hits are the product module `secret_private_library`, prose and Actions log lines.
- There are no personal e-mail addresses. The e-mail-like hits are `@pytest.mark` decorators and plugin identifiers.
- The `*tokens*` file names are code-name token tables.

**Personal and host data.** See S-1 (process listings), N-1 (host paths: 938 new files) and N-3 (host name: 11 new files). None of it is a credential.

**Size and binaries.**
- There are 6,478 added regular files, totalling **222,378,072 B (212.1 MiB)**. The largest is 6,698,509 B (`FULL_COVER_RESULTS.json`). **No file exceeds 50 MB**, and none exceeds 10 MB.
- `file --mime-type` finds text, JSON, scripts, diffs and empty files, plus 35 gzip payloads. Decompressed, those are 24 JSON, 10 text and 1 tar of 195 members: CI run logs and JSON, and DEC-025 suite logs, 16.7 MB uncompressed.
- There are **no executables, object files, `.rlib`, `.wasm`, `.so`/`.dylib`, `.pyc` or `target/` paths**. The 72 mode-100755 files are 62 `.sh` scripts and 10 `.txt` files, which carry the exec bit only.
- The three nested `.gitignore` files are record content: RV56's two ignore `target/` and `imported/`, and RV58's fixture file is a symlink (N-2).

## 3. GEN-8 placement: **FAIL** (B-1)

### 3.1 The run

On a `git archive` copy of H in `WT/rv96/` (2.9 G), from the copy's root:

```
TMPDIR=WT/scratch/rv96_records_01/tmp PYTHONDONTWRITEBYTECODE=1 \
  VENV -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8
```

The run lasted from 2026-10-05T02:33:46Z to 02:34:08Z, 22.07 s. The result was **`1 failed, 10 deselected`**, exit 1. The assertion fails at `test_live_baseline.py:243`: "Left contains 263 more items, first extra item: … `ABS_PATH_IN_UNCLASSIFIED_SURFACE` … `HANDOFF_2026-10-03_TO_NEXT_ROOT.md`, source_line=23". The log is `_run_records/gen8_pytest.log`.

### 3.2 Independent confirmation (read-only, no second pytest)

`_run_records/gen8_replay.py` replays `cmd_self_check.py`'s GEN-8 per-file loop. It uses `surface_roles.effective_role`, the project policy, and the same suffix and `.archive`/generated exclusions, applied to `git ls-tree` of each commit with blobs read from Git. The results (`_run_records/gen8_replay_summary.txt`):

| Tree | Candidates | Findings | Policy issues |
|---|---|---|---|
| H `dfa5e2dc44` | 6,367 | **263** | 0 |
| M `e916ad1789` | 5,165 | **0** | 0 |
| N `29160bbc1c` | 6,367 | **263** | 0 |

The hosted run agrees: `governance-harness` run 37255803450 (event `pull_request`, headSha H, conclusion failure) reports "1 failed, 1155 passed, 48 subtests passed", the same 263 count and the same first item (`_run_records/ci_governance_harness_failure_excerpt.txt`).

### 3.3 What is flagged

All 263 files are inside T3, and all are added or changed by this PR.
- **By rule:** 261 are "unknown managed AgentRuns artifact" (UNCLASSIFIED). Two are CONTROL by name token: `R/I42/source_bridge_rv56_repair_02/PLAN.md` and `R/BRIEFS/RV67_I51_C2_AMENDMENT_REVIEW.md`.
- **By name:** most are machine evidence that is not in `_run_records/` (`EXECUTION.json` 33, `BULK_MANIFEST.json` 26, `ORIGINS.json` 24, `MANIFEST.json` 20, `CHECKS.json` 14, `SEAL.json` 12, …), plus 45 as-issued dispatch briefs in `R/BRIEFS/`.
- **ROOT's living documents:** RR (line 10932, I61's system-temp disclosure, which quotes a system-temp path), `T3/HANDOFF_2026-10-03_TO_NEXT_ROOT.md`, `T3/HANDOFF_2026-10-05_TO_NEXT_ROOT.md` and `T3/HANDOFF_2026-10-05_PROMPT.md`.
- **Seal coverage:** 8 of the 263 are covered by a SHA256SUMS. Many more are bound in other ways: as-issued brief hashes, SEAL.json and BULK_MANIFEST.

### 3.4 Remedy space (for ROOT's ruling)

- **(a) Hash-bound policy registration.** Register entries in `P/validation/portability_policy.json`: 261 `historical_role_override` (EVIDENCE) and 2 `control_path_exception`.
  - This is the established practice. 530 overrides and 47 exceptions exist today, and `f9ff31f163` (2026-09-26) registered as-issued T1 briefs "instead of being edited".
  - It cannot land ahead of the records: `_parse_entries` requires each target to exist, with matching hash, so it must travel in the same PR. That breaks the "only `P/execution/`" property, which is why this needs a ruling.
  - It is unsuitable for living files. Every later append to RR or a handoff file would raise `PORTABILITY_POLICY_HASH_DRIFT`, which GEN-8 also asserts on.
- **(b) Placeholder repair on NUM** for the living ROOT documents, with RR's labelled-correction convention, plus (a) for the immutable records. Editing sealed or pinned evidence is not advisable.
- **(c) Waiver.** Not viable: main's push run (full suite, `CHIRALITY_REQUIRE_LIVE_TESTS=1`) would fail, and so would every later routed PR run that selects the practitioner harness.

After any repair: re-cut from NUM, then rerun GEN-8 (with N-6's method), hosted CI with the full-SHA dispatch, and DEC-025 on the new head.

## 4. Gate evidence: partial at review time (2026-10-05T02:53Z)

| Gate | Evidence | Exact head? | Result |
|---|---|---|---|
| Hosted PR runs | `Piping Desktop E2E` 37255803401 | H | success (the numerical cargo suite and others skipped by selection) |
| | `Harness Pre-merge Validation` 37255803582 | H | success |
| | `pec-tests` 37255803400 | H | success |
| | **`governance-harness` 37255803450** | H | **failure (GEN-8, B-1)** |
| Full-SHA dispatch | `Piping Desktop E2E` 37255801968, `workflow_dispatch` on `codex/piping-t3-records-20261005` | H (plan `head` = full SHA H) | **in progress.** The plan reads `mode: full`, `target_base` = `base` = M (main), `numerical_required: true`. Select source coverage and Source remainder 1–4 had passed; the Numerical cargo suite was running; Accessibility was skipped. |
| GEN-8 | §3 | H | **FAIL** |
| DEC-025 against the fresh Mac baseline on main | ROOT | — | **pending** (ROOT's) |

Run metadata is in `_run_records/ci_runs_head.json`, `ci_governance_harness_run.json`, `dispatch_37255801968_at_review.json` and `dispatch_plan_excerpt.txt`.

**Pending, for ROOT to relay:** the dispatch's final conclusion (in particular the Numerical cargo suite), DEC-025's comparison, and, after any B-1 repair, all gates again on the new head. The failing `governance-harness` check alone fails item 4 at H.

## 5. Integrity of the sealed records: PASS (N-4)

**All SHA256SUMS-style files in T3 at H.** There are 670 such files: 185 added by this PR and 485 already on main, unchanged. Each line was resolved against the SUMS folder, then its parent, then its grandparent; the `_run_records/SHA256SUMS` convention lists paths relative to the record folder. **660 verify completely.** **0 BAD in any file added by this PR.** Results are in `_run_records/sums_verify.tsv`.
- **The 1 added exception** is N-4: `R/I65/u4_g7_03`, with 67 OK and 1 gitignored `.pyc`.
- **The 9 exceptions on main** are unchanged by this PR. They are historical snapshots (`REFERENCES*/…/SHA256SUMS.revision*` and RV37's `original_warrants/*/SHA256SUMS.original`), and their stale entries or mismatches are what a superseded snapshot records.

**The named folders** were checked with `shasum -a 256 -c` in the folder:

| Folder | SHA256SUMS (sha256 prefix) | Result |
|---|---|---|
| `T3/IMPLEMENTATION/F2A_D1_MERGE/` | `a27bde22f916` | **119/119 OK** (118 + the ERRATA line) |
| `R/REVIEW_RV95/u9_01/` | `c698b7bd01f0` | **127/127 OK** |
| `R/I65/u9_refreeze_01/` | `b95cff98fc91` | **26/26 OK** |
| `R/REVIEW_RV89/u4_g7_03/` | `d17d48dd20f7` | **25/25 OK** |

**The rulings' last ten sections (RR:11635–11998).** Every cited record exists, and every cited hash matches:
- I61 `u9_g5g6_01` RETURN `37f7938d` (84/84 OK).
- I61 `u9_freeze_01` RETURN `a7e0aae7` (9/9 OK); its package SHA256SUMS `c8893d64` matches as `_run_records/package_SHA256SUMS`.
- I65 `u4_g7_06` RETURN `b4022b8b` (72/72 OK).
- RV89 ADDENDUM_01 `676e9f99` and ADDENDUM_02 `89403fc2`.
- RV95 ADDENDUM_01 `92ff6e27`, ADDENDUM_02 `51464018` and ADDENDUM_03 `3e6438b8`.
- I65 `u9_refreeze_01` RETURN `6e1c143e`.
- I61 `u8_plan_01/PLAN.md` `f274a614` (1/1 OK).
- The DEC-025 driver copy `9e34865b`, identical in `F2A_D1_MERGE/dec025/` and `HANDOFF_2026-10-05/host_tools/`.
- `IMPLEMENTATION/HANDOFF_2026-10-05/SHA256SUMS` (14/14 OK).
- The briefs RV97 and RV99 listed in the handoff table exist.
- The only cited folder that is absent is `IMPLEMENTATION/RECORDS_MERGE_2026-10-05/` (N-5).

**RR is append-only against main: PASS.** Main's RR is 460,817 B (sha256 `aab765b2…`). It is an **exact byte prefix** of H's 1,024,024 B (`96319da9…`), with 563,207 B and 7,104 lines appended. `ROOT_RULINGS_V2.md` is unchanged.

**Observation, no finding.** Within NUM's own region, beyond main's prefix, 11 of NUM's 265 first-parent changes to RR are not pure appends (`65a23ea156` … `7f88ac5f5c`). Each one I inspected is a labelled inline correction ("[Correction (ROOT, 2026-10-03): …]", "*Text repair (ROOT, 2026-10-04): …*") that keeps the original wording visible. Main's bytes are untouched.

## Host and method

- **My copy:** a `git archive` of H in `WT/rv96/` (deleted at the end). Logs and scripts are in `WT/scratch/rv96_records_01/`, where the tarball was extracted for scanning. `records-pr` and NUM were used read-only, with Git reads only. There were no Git writes, installs, cargo or native jobs, and no pytest beyond the single GEN-8 run.
- **Read-only diagnosis:** `gen8_replay.py` (classification replay; no self-check run), `secret_scan.py` and `sums_verify.py` (hashing), `git`, `file` and `shasum`. `gh` was used read-only for the PR metadata, run metadata and logs.
- **The memory guard** PID 5387 was running throughout (etime more than 7 days), with no KILLED line in its log.
- **Disclosure.** The agent harness stored the output of one backgrounded read-only `grep` under its own task directory in the system temp directory. That output was repo-relative file names and a compiler version string. No job output, log or temporary file of mine was written there; `TMPDIR` pointed to WT/scratch.

## What ROOT must rule on

1. **B-1:** how GEN-8 is made to pass. Options: (a) one non-execution policy file added to this records-only PR, with hash-bound entries for the immutable records; (b) placeholder repairs on NUM for RR:10932 and the three ROOT handoff files; or both. Then a re-cut and all gates again on the new head.
2. **S-1** (with N-3): accept publication of the whole-host process listings and host name with a record, or redact under supersession.
3. **N-2:** accept the 104 dangling absolute symlinks as sealed reference evidence.

## Evidence index (`_run_records/`)

- `gen8_pytest.log`: the GEN-8 run.
- `gen8_findings.tsv`: the 263 findings, with T3-relative path, code, hit lines, first hit line and SUMS coverage.
- `gen8_replay.py` and `gen8_replay_summary.txt`: the replay for H, M and N.
- `ci_runs_head.json`, `ci_governance_harness_run.json`, `ci_governance_harness_failure_excerpt.txt`, `dispatch_37255801968_at_review.json` and `dispatch_plan_excerpt.txt`: the CI evidence.
- `sums_verify.py` and `sums_verify.tsv`: all 670 SUMS files.
- `secret_scan.py` and `secret_scan_summary.txt`: the credential scan.
- `symlinks.tsv`: the 104 symlinks, with the worktree root replaced by `<WT0>`.
- `process_listings.tsv`: the 12 process listings.
