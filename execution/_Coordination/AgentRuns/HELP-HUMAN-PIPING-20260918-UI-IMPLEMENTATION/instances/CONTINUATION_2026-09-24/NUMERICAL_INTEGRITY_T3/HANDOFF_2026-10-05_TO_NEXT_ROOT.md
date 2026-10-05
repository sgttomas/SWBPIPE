# T3 handoff to the next ROOT — 2026-10-05

**Handed off at the owner's request.**
- **Nothing is running:** no TASK, compiler, solver or test. The memory guard stays running.
- **The F2a D1 milestone is on main** (PR #1082).
- **T3's records through this handoff go to main in two records-only PRs:** #1084 (merged, squash `f506f3e2de`) and #1088, which carries this version of the handoff. #1088's merge record, `IMPLEMENTATION/RECORDS_MERGE_2026-10-05B/` on NUM, confirms whether it merged.
- **U8 is planned and ruled, and its briefs are ready.** Nothing is dispatched.
- **The host was cleaned** (about 0.5 TiB freed), and every T3 branch is pushed.

You continue as **HELP_HUMAN, Type 0, Agent 0 (ROOT)** for T3, under the owner's standing directions and delegated technical authority ("carry on with T3 in the manner you see fit"). The workflow in use is `coordinated-knowledge-work` (Root `workflows/`).

## Start here

1. **The instructions:** Root `AGENTS.md`, `agents/AGENT_HELP_HUMAN.md` and `projects/chirality-piping/AGENTS.md`.
2. **This handoff,** then [ROOT_CURRENT](RESUME_2026-09-30/ROOT_CURRENT.md) and the [glossary](RESUME_2026-09-30/GLOSSARY.md).
3. **The latest rulings** in [ROOT_RULINGS_V1.md](ROOT_RULINGS_V1.md), which is append-only. Read from "U9 planned and ruled" to the end. The decisive ones:
   - "DEC-025 on F finds a test-walker defect; repaired and refrozen as F′ = 5488136a19";
   - "#1082 merged at F′: the F2a D1 milestone is on main";
   - "Owner decision: F2a's delivery steps move to the end of F2a; erratum E-1";
   - "I61's U8 plan ruled; the owner pulls T6's successor-output slice forward; dispatches prepared for the handoff".
4. **The plan:** [`R/I61/u8_plan_01/PLAN.md`](RESUME_2026-09-30/I61/u8_plan_01/PLAN.md), covering U8, the F2a-breadth roadmap (B0–B8), S-I1's readiness, and absorbing main.
5. **The briefs** in `RESUME_2026-09-30/BRIEFS/`: `U8_COMMON.md`, I68–I74 and RV97–RV99.
6. **The merged milestone's package:** `IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` (scope §4, obligations §5) and the merge record `IMPLEMENTATION/F2A_D1_MERGE/` (RECORD, ERRATA).
7. **The records merges:**
   - `IMPLEMENTATION/RECORDS_MERGE_2026-10-05/` covers #1084.
   - `IMPLEMENTATION/RECORDS_MERGE_2026-10-05B/` covers #1088, the follow-up records PR and the outgoing ROOT's last act. It is on NUM only, and says whether that PR merged. **Check it first.**
   
   A records PR's post-merge record is necessarily written after the merge, so it stays on NUM until the next records PR. NUM is therefore normally ahead of main by that record and the rulings after it.

**Notation:** P = `projects/chirality-piping`; T3 = this directory; R = `T3/RESUME_2026-09-30`; RR = the rulings file.

## The host (check it first)

| | |
|---|---|
| **Machine** | The M5 Max, 128 GiB. All heavy work runs here |
| **WT** | `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3`, where `<repo>` is the owner's main Chirality checkout on this host (the folder that holds `.claude/worktrees/`) |
| **NUM** | `WT/numerics`, branch `codex/piping-numerical-integrity-20260926`, the T3 integration branch. Its maintained source equals main's; only records differ |
| **The DEC-025 tree** | `WT/sweep-skewpin`, detached. Check out the candidate head and keep it clean. After each run, move the sweep's summary JSON out to the run's scratch folder |
| **Worktrees** | Only these two remain under WT. Create others per unit (e.g. `WT/f2a-u8`, `WT/s-i1`) and remove them when merged |
| **The memory guard** | `WT/guard/memguard.sh`, PID 5387 at handoff; check with `pgrep -f memguard.sh`. It must run during any build |
| **VENV** | `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/projects/chirality-piping/.venv` |
| **node_modules** | The parent checkout's `projects/chirality-piping/node_modules`, linked into a worktree by an untracked symlink, which you remove before removing the worktree |
| **DEC-025 tooling** | `WT/scratch/u9_dec025/dec025_mac.sh` (sha `9e34865b…`, unchanged from `M03_SKEW_PIN_MERGE`); the quiet-host wrapper `run_dec025.sh` (which replaced `run_all.sh`); the suite runner `WT/scratch/calib/run_suites_nff.sh`. Copies are in `IMPLEMENTATION/HANDOFF_2026-10-05/host_tools/`. The per-test comparator is `IMPLEMENTATION/F2A_D1_MERGE/dec025/compare_suites.py` |
| **Cleanup** | `WT/tools/t3_cleanup.py`, which has `gather`, then `plan`, then `apply`. Procedure in `IMPLEMENTATION/HANDOFF_2026-10-05/CLEANUP.md`. It is a documented procedure, not a registered workflow |
| **Evidence that exists only on disk** | `WT/scratch` (about 44 GB) and `WT/preserved-evidence`. Never pruned without the owner |

## What is done

- **#1082 merged** at 2026-10-05T01:09:29Z as `0b00b8e8b6` (parents M `5fdc5ab601` and F′ `5488136a19`). What it delivers:
  - the milestone RF-SKEW-T-CANT-OFF-122-r1e-04 through the Direct entry, both modes, in the registered dev/test build only (M = 4,026,531,840 B);
  - three-language reader eligibility, and the carriers.
  
  **No product caller exists.**
- **The first freeze failed, and was repaired.** DEC-025 on F caught a test-walker defect in nonlinear_integration, repaired test-only, and the head was refrozen as F′. **Rule since then: the full 40-manifest suite runs before any freeze.**
- **Erratum E-1:** the 9 ignored PP `witness_*` tests are U4's stack witnesses, not U8's.
- **NUM absorbs main after #1088 merges,** and `RECORDS_MERGE_2026-10-05B/` confirms both. Before that, NUM had absorbed main through #1084 (`f506f3e2de`). Every T3 branch is on origin.
- **T3's records reached main** in two records-only PRs:
  - **#1084,** squash `f506f3e2de`. It passed RV96's review at its third head (FAIL, FAIL, then PASS), GEN-8, hosted CI, the full-SHA dispatch and DEC-025, with 255 historical records registered in the portability policy and 13 files redacted.
  - **#1088, the follow-up,** reviewed by RV100 (PASS; its SHOULD-FIX was repaired by a re-cut), carrying everything since: #1084's merge record, the stray-scratch gathering and this handoff's final form.
- **Stray scratch was gathered** into `WT/scratch`, and the cleanup tool gained its `gather` step.
- **I61 planned U8 and the rest of F2a.** ROOT ruled all 13 ROOT decisions as recommended. The owner decided the 14th.

## Owner decisions in force

**Dated:**
- **2026-10-04: main held for #1082.** The hold ended at the merge, and the owner was told.
- **2026-10-04: the native witness (G10) stays outstanding on the owner's Mac.** It covers the milestone on the ordinary route, and the export panels refusing a successor.
- **2026-10-05: the F2a order:**
  1. U8;
  2. F2a's numerical breadth on the dev/test build;
  3. the release identity, registered once (B7);
  4. public activation with native Current (B8);
  5. S-I2, F2b per family, and F3.
  
  S-I1 runs alongside.
- **2026-10-05: the T6 successor-output slice is pulled forward,** to run in parallel. It closes activation checklist item 4.
- **2026-10-05: cleanup.** Remove what is regenerable, periodically. Removing evidence is the owner's call.
- **2026-10-05: #1084's records.** 255 historical run records are registered in the portability policy; 13 files with whole-host process data were redacted before merging.
- **2026-10-05: all T3 scratch lives in `WT/scratch`.** Each cleanup starts with `gather`.
- **2026-10-05: "merge what's ready"; then the follow-up records PR is merged.**

**Earlier:**
- T1 option (a);
- D-3 = S1 (a reserved stack plus a witness);
- D-6 = (a) (a fail-closed build check);
- M selected under D-7.

**Owner-held, and not yours to decide:**
- dense and lane ceilings;
- PHYS-R4 named refusal and availability;
- observation framing;
- the KF3 lambda split;
- the KF2 dense screen;
- any supported-machine statement of M, or M above the provisional 3.75 GiB;
- any change to public meaning;
- the native-app witnesses, which need the owner's Mac.

## The next work, in order

1. **Verify the state:**
   - the guard is running;
   - `git -C WT/numerics worktree list`;
   - NUM's and main's heads;
   - the records-merge records;
   - free disk space.
2. **Absorb main into NUM** if main moved, using PLAN §4's dry run. Flag:
   - any of the 140 S files;
   - any crate in PP's dependency closure;
   - PP's `Cargo.lock` or the 13 reviewed statics. A change to any of these makes the registered build Stale until it is re-registered.
3. **U8** (`BRIEFS/U8_COMMON.md`):
   1. Cut `codex/piping-f2a-u8-<date>` from NUM into `WT/f2a-u8`.
   2. Dispatch **I68 Part 1** (the probe), then rule on its outcomes. Those decide whether L = 0 publishes and what W-C1's reason is.
   3. Then I68 Part 2.
   4. Only if L = 0 publishes: **I69**, then **I70** and **I71**.
   5. Then **I72** (Pass B) and **RV97** (the review). RV98 confirms Pass B. Each repair is confirmed by the same reviewer.
   6. Then the full 40-manifest suite, and the freeze.
   7. **U8's own compact PR** from main: source equality and a small evidence package, as #1082 did. Its gates are hosted CI with the full-SHA dispatch (`target_base` = main), GEN-8, and DEC-025 against a fresh Mac baseline of main.
   8. Merge with `--match-head-commit` after checking main has not moved, or carry the gates over by ruling under "Gates before a main merge" below. Write the post-merge record on NUM.
4. **In parallel, S-I1** (`BRIEFS/I73_S_I1.md`, `RV99_S_I1_REVIEW.md`): branch `codex/piping-s-i1-<date>` from main into `WT/s-i1`, with its own PR. It needs no D1 gates, but does need DEC-025 and hosted CI. **One cargo job at a time across U8 and S-I1: you order them.**
5. **In parallel, the T6 slice plan** (`BRIEFS/I74_T6_SUCCESSOR_SLICE_PLAN.md`). Rule on it, then dispatch its implementation with its own reviewer and PR.
6. **Then breadth:** B0 (reserve `openpipestress.result_semantics/0.3.0/physics-retained-1`) → B1 and B6 (W-C2, D38's pin, and early in B1 the cap-growth study) → PR-B1 → B2 and B3 → B4 if ruled → PR-B2 → B7 → B8 → S-I2 → F2b per family → F3.
   - **Re-qualify once per PR candidate** on the registered identity: TEXT, the identifier audit, Pass B and an independent confirmation, plus the G5/G6 work for a widening.
   - **B8 needs** caller qualification for the desktop workspace (RR:10407), I53's open native-window premise, and the owner's Mac.

**Assignment IDs.** Use the prepared IDs I68–I74 and RV97–RV99 as fresh instances; an ID is a records folder and a role, not a memory. The next unused are **I75 and RV101**. RV96 reviewed #1084, and RV100 the follow-up records PR.

## Rules that continue

- **Git:**
  - TASKs make no Git writes; ROOT commits, merges and pushes;
  - never rebase or force-push;
  - **how to merge:**
    - a product PR with `gh pr merge --merge --match-head-commit`;
    - a records-only PR with `gh pr merge --squash --match-head-commit` and an explicit subject and body;
    - either one only after checking `origin/main` has not moved. If it has, refresh the gates on the new combination, or carry them over by ruling (see "Gates before a main merge");
  - **never merge NUM itself, or any branch that carries its history, into main.** NUM's history holds the 13 unredacted originals listed in #1084's `REDACTIONS.json`;
  - no auto-merge;
  - merges use the owner's standing authorization (AGENTS.md): required CI passes, and independent review has no unresolved blocking finding on the actual candidate.
- **PR packaging.** A product PR is cut compactly from main, with maintained-source equality to the integration head (`source_equality.py` in `IMPLEMENTATION/F2A_D1/`), a concise evidence package, and checked citations. Records reach main only through a separate records-only PR.
- **Send records to main** after each main merge, and at a handoff, with a records-only PR:
  - cut it from main, taking `projects/chirality-piping/execution/` from NUM;
  - run GEN-8 on the candidate before opening it;
  - **its gates are GEN-8, the PR's automatic CI checks, and an independent review.** DEC-025 and the full-SHA dispatch are not run, because they test source, which a records-only PR leaves identical to main. If a changed record is read by a test, run the suites that read it. T3's `REFERENCES/` and `DESIGN_NUMERICS/` are read by `gen_k4_vectors.py`;
  - it is squash-merged with `--match-head-commit`.
- **Gates before a main merge,** for a PR that changes source, tests, CI, tools or the portability policy. A records-only PR has its own smaller gate set, above:
  - a fresh independent complete-diff review, with same-reviewer repair confirmation;
  - hosted CI and the full-SHA dispatch;
  - the full 40-manifest suite before the freeze;
  - an exact-final-head Mac DEC-025, compared per manifest and per test (`compare_suites.py`) against a fresh baseline of current main;
  - GEN-8, run in a Git checkout of the exact head, with its output saved alongside the head SHA and the command;
  - for D1-call-graph changes, Pass B with an independent confirmation, plus T9 and the both-entry gates where product behaviour can change.
  
  **A carry-over by ruling** needs a stated premise and the independent reviewer's confirmation, both recorded in the merge record. One kind is established:
  - **Main moved after the gates (#1084).** Every path main's move changes lies under another project's own directory (`projects/<name>/`, never `projects/chirality-piping/`), with nothing in `tools/`, `.github/`, the portability policy or root build files. GEN-8 also passes on a local combination of the head with the moved main.

- **DEC-025 runs on a quiet host:** the wrapper waits for six quiet samples, with a fresh target, and nothing else builds meanwhile. Tell reviewers not to build during the window.
  - **The wrapper** is `WT/scratch/u9_dec025/run_dec025.sh <label> [<baseline worktree>]`, which replaced `run_all.sh`.
  - **A run is complete only when its `meta.txt` ends with `ALL-DONE`.** The wrapper logs `dec025-complete`, or `dec025-INCOMPLETE` and exit 3. The old `run_all.sh` logged `dec025-done` even when nothing ran.
- **Records:**
  - placeholder paths, and SHA256SUMS per folder;
  - the rulings stay append-only;
  - corrections are errata, not edits;
  - add records with explicit paths. A `git add -A` over records swept in a TASK's in-progress file once.
- **Living documents carry no machine-absolute paths.** The rulings, ROOT_CURRENT, the work graph, handoffs and briefs write `WT/…` or `<repo>/…`. Never hash-bind a living document in the portability policy.
- **Screen run records for whole-host data** before publishing: process listings, app names, session IDs. Capture only the processes a check needs.
- **Never:** solver-at-scale or native jobs by TASKs; installs; writes to the system temp directory.
- **All T3 scratch lives in `WT/scratch`,** never the parent checkout's `scratch/` or a worktree's own folder. Each periodic cleanup starts with `gather`, which moves strays into `WT/scratch` with hash verification and never deletes.

## Lessons from the last session

- **A test in one crate can pin another crate's source.** nonlinear_integration's module walker reads PP's source. The integration branch ran PP, runner, result_export, Python and TS only, so the walker defect went unnoticed until DEC-025. Hence the full-suite rule.
- **Label tests from their source, not from memory.** That caused erratum E-1.
- **The cleanup tool's first plan would have deleted tracked record folders inside worktrees.** Always plan, read the plan, then apply. The tool now refuses tracked paths.
- **Ask the owner early** when a decision blocks a far-off step but affects parallel scheduling. The T6 pull-forward was decided as soon as the plan showed it.
- **#1084 needed three heads.**
  - GEN-8 flagged 263 machine paths that nobody had run it to find.
  - Whole-host process listings had to be redacted.
  - A merge commit would have carried the unredacted originals into main.
  
  Run GEN-8 and the host-data screen before opening a records PR, and squash it.
- **Keep gates proportionate.** Records PRs ran the full Mac DEC-025 and the CI dispatch for documentation-only changes until the owner asked about it. Match the gates to what the change can affect.
- **A wrapper reported done when nothing had run.** #1088's first DEC-025 attempt failed on a missing output folder, yet the old wrapper still logged `dec025-done`. Check for `ALL-DONE`.

## Open items not yet in a brief

- **G10:** the native witness, on the owner's Mac.
- **Routed notes:**
  - RV94 N-5 (Python ignores `expected_by_reader`);
  - RV95 N-2 (citation forms);
  - Pass B's attribution of deletion-only hunks;
  - RV95 B-1, C-1 and C-2 (optional walker hardening);
  - RV89 N-1 (handled in I72's brief).
- **RV95 N-6:** re-establish the milestone's bytes and verdicts on any further registered identity (B7).
- **The public-activation checklist,** in CHANGE_RECORD §4.
- **The PR branch `codex/piping-t3-records-20261005`** on origin still holds #1084's first head, with the unredacted listings.
  - Deleting the branch is the owner's call.
  - It would not remove them, since NUM's history and `refs/pull/1084/head` keep them.
