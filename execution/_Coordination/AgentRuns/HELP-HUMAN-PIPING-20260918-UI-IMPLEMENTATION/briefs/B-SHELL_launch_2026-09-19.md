# Sealed launch brief — a new B-SHELL manager, 2026-09-19

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19, in the session that continues the run from `{RUN}/instances/ROOT/SESSION_HANDOFF_2026-09-19.md`. Role: WORKING_ITEMS (Type 1), lane manager of B-SHELL. Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type. Path placeholders as in the lane brief. Write no absolute machine path in any file you author. This file lives on ROOT's records branch `codex/swbpipe-records-20260919`; ROOT's launch message carries its text and hash.

## Who you are

You are a new manager. The manager that ran slice B2 belonged to an earlier session of ROOT; you have none of its context and cannot resume its children. The lane's records are written so that you can start from them.

## Where you work

- Git worktree `swbpipe-wt3` (find its path with `git worktree list`; work only there), branch `codex/swbpipe-b-shell-20260918`, head `5f70ad178f951afbdc7b126a4f1006975c0266c1`, clean. `node_modules` is present under `{WORKING_ROOT}`.
- **Your first git step is `git merge origin/main`** (`origin/main` is at `64f86e17f`, which carries B2's merge, the handoff and `{RUN}/tools/`). Then re-run the type check, the unit suite and the build before any other work, and note the results in the lane log.
- Everything else about Git, scope and children is as the lane brief and addendum 3 give it: you commit on the lane branch only; ROOT pushes, opens pull requests and merges.

## What binds you, in reading order

| Instrument | SHA-256 |
|---|---|
| `{RUN}/briefs/B-SHELL_shell_lane.md` | `00c98ec0f77363d3bf498c4729185b4761f77e7178853277996352b4b59c4b75` |
| `{RUN}/lanes/B-SHELL/briefs/B-SHELL_addendum_1.md` | `f91f8d2bf55de8c9eb524abc38ea196cba7782d25492f79239b7d2a90c833aa2` |
| `{RUN}/lanes/B-SHELL/briefs/B-SHELL_addendum_2.md` | `60406868afad389c56126276a1a06f4ee00967400363e3973ab2bc01e0cb7509` |
| `{RUN}/lanes/B-SHELL/briefs/B-SHELL_addendum_3.md` | `4b928f286036aa5a4e7de1152d9fa7b72bcb9d39dd3f038e02d6ad3735f93b35` |

Verify each hash before you rely on it. Addendum 3 is the latest: its section "If you are a new manager" gives your reading order after these four, and your first slice is **B2F**. The sequence is the control layer first; the visual redesign is last.

## Three things this session adds

1. **The owner works under a usage limit.** Prefer few, well-briefed children: one child per slice where the slice allows it, a slice small enough to do yourself done yourself, Sonnet for mechanical work against an exact inventory. Do not run the full Playwright lanes more often than a slice's return needs.
2. **Return at each frozen slice candidate and stop**, as the lane brief says. Your final message is the slice return: slice id, candidate commit, diff range against `origin/main`, the checks with commands and results, the tests addendum 3 names for the slice, the controls table, **contrast not yet checked**, **appearance left for the closing pass**, semantic changes named, what you bring back, the next slice, and the model you ran as, by your own statement. ROOT retains it, dispatches the independent review from another worktree, and resumes you by message with findings or the go-ahead.
3. **Ports.** Do not bind 5183 or 5184; ROOT uses them to look at builds. Ports 5174 and 5175 only through `with_e2e_lock.sh`.

## Pausing

If ROOT asks you to pause, stop at a clean point: commit or leave the worktree clean, hold no browser-test lock, leave no server running, and end your turn with the head commit, what is done and what is next.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
