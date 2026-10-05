# T3 handoff to the next ROOT session (ephemeral, 2026-10-05)

This note is for the owner's next T3 session only. It is not a standing record and is not maintained after use. Durable facts live in the records it points to.

T3's current account is the piping work graph's **T3 current route** section, `execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`. It holds the route, the owner-held choices, the decisions and rulings in force, the next IDs and the next safe action.

## Starting the session

- **The owner starts it** in the T3 integration worktree, `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics`. Here `<repo>` is the owner's main Chirality checkout on this host. `{REPO_ROOT}` then resolves to that checkout, on branch `codex/piping-numerical-integrity-20260926`.
- **The first message** is piping's init prompt with the steer filled in: [HANDOFF_2026-10-05_PROMPT.md](HANDOFF_2026-10-05_PROMPT.md).
- **The loop instructions** are piping's `loop/LOOP_INIT.md`, in the binding form adopted on 2026-10-05 (run `PIPING-LOOP-INIT-20261005`). If main does not yet carry that form, the session finds it on NUM.

## Where things are, in this folder

- **The rulings:** `ROOT_RULINGS_V1.md`, append-only. The graph lists the sections in force.
- **I61's plan:** `RESUME_2026-09-30/I61/u8_plan_01/PLAN.md`. It covers U8, the B0–B8 roadmap, S-I1's readiness, and absorbing main (§4).
- **The prepared briefs:** `RESUME_2026-09-30/BRIEFS/`: `U8_COMMON.md`, I68–I74 and RV97–RV99.
- **The merged milestone:** `IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` (scope in §4, which includes the public-activation checklist) and `IMPLEMENTATION/F2A_D1_MERGE/`.
- **The records merges:** `IMPLEMENTATION/RECORDS_MERGE_2026-10-05/` (#1084) and `IMPLEMENTATION/RECORDS_MERGE_2026-10-05B/` (#1088).
- **Terms:** `RESUME_2026-09-30/GLOSSARY.md`.

## The host (machine-local; check it)

| | |
|---|---|
| **Machine** | The M5 Max, 128 GiB, with little swap (about 1 GiB, dynamic). All heavy work runs here |
| **WT** | `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3` |
| **NUM** | `WT/numerics`, the integration branch's worktree |
| **The DEC-025 tree** | `WT/sweep-skewpin`, detached. Check out the candidate head, keep the tree clean, and move each sweep's summary JSON out to the run's scratch folder |
| **Worktrees** | `numerics` and `sweep-skewpin` are the standing ones. Create one per unit or PR (for example `WT/f2a-u8`, `WT/s-i1`), and remove it once merged |
| **The memory guard** | `WT/guard/memguard.sh`, PID 5387 at handoff. Check it with `pgrep -f memguard.sh`. It must run during any build |
| **VENV** | `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/projects/chirality-piping/.venv` |
| **node_modules** | The parent checkout's `projects/chirality-piping/node_modules`. Link it into a worktree with an untracked symlink, and remove the link before removing the worktree |
| **DEC-025** | `WT/scratch/u9_dec025/run_dec025.sh <label> [<baseline worktree>]`. It wraps `dec025_mac.sh` and `WT/scratch/calib/run_suites_nff.sh`. Compare runs per test with `IMPLEMENTATION/F2A_D1_MERGE/dec025/compare_suites.py`. Copies of the tools are in `IMPLEMENTATION/HANDOFF_2026-10-05/host_tools/` |
| **Cleanup** | `WT/tools/t3_cleanup.py`: `gather`, then `plan`, then `apply`. The procedure is in `IMPLEMENTATION/HANDOFF_2026-10-05/CLEANUP.md` |
| **Evidence only on disk** | `WT/scratch` (about 44 GB) and `WT/preserved-evidence`. Never pruned without the owner |

## Open items for the owner

- **G10,** the native witness, needs the owner's Mac.
- **#1084's PR branch,** `codex/piping-t3-records-20261005`, still holds that PR's first head with the unredacted listings. Deleting it is the owner's call. It would not remove the listings, which NUM's history and `refs/pull/1084/head` keep.
