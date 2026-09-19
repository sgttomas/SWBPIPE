# Sealed launch brief — a new B-CANVAS manager, 2026-09-19

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19, in the session that continues the run from `{RUN}/instances/ROOT/SESSION_HANDOFF_2026-09-19.md`. Role: WORKING_ITEMS (Type 1), lane manager of B-CANVAS. Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type. Path placeholders as in the lane brief. Write no absolute machine path in any file you author. This file lives on ROOT's records branch `codex/swbpipe-records-20260919`; ROOT's launch message carries its text and hash.

## Who you are

You are a new manager. The manager that ran the lane to its pause point belonged to an earlier session of ROOT; you have none of its context and cannot resume its children. The lane's records are written so that you can start from them.

## Where you work

- Git worktree `swbpipe-wt4` (find its path with `git worktree list`; work only there), branch `codex/swbpipe-b-canvas-20260918`, head `4a7408e5512c425cbf9cdedd0b17396ad176607b` (product head `3a5f7c103`, a labelled pause point), clean. `node_modules` is present under `{WORKING_ROOT}`.
- **Your first git step is `git merge origin/main`** (`origin/main` is at `64f86e17f`, which carries B2's merge, the handoff and `{RUN}/tools/`; the branch has not taken `main` since `8e4c5df6e`). Then run the picking tests first, the viewport folder's tests, `tsc -b` and the build, as addendum 4's first step says, and note the results in the lane log.
- Everything else about Git, scope and children is as the lane brief and addenda give it: you commit on the lane branch only; ROOT pushes, opens pull requests and merges.

## What binds you, in reading order

| Instrument | SHA-256 |
|---|---|
| `{RUN}/briefs/B-CANVAS_canvas_lane.md` | `df89f5b622a63f206ccc0f879e7255642e4b9ee16174faf84e539ab51bc12084` |
| `{RUN}/lanes/B-CANVAS/briefs/B-CANVAS_addendum_1.md` | `4d5af33abf5b916ab3ddae09214e0d98070f6f2a984f0604d953e5381cd25ad5` |
| `{RUN}/lanes/B-CANVAS/briefs/B-CANVAS_addendum_2.md` | `a5b11b94b23a3eaaabc69680ee03f8dcdb6377ddcce6ea5a44b08fa6b54bdc74` |
| `{RUN}/lanes/B-CANVAS/briefs/B-CANVAS_addendum_3.md` | `a86cb318bb0b49a49df1c13526999ae0a8115849d2f032f45450e88c10dfb210` |
| `{RUN}/lanes/B-CANVAS/briefs/B-CANVAS_addendum_4.md` | `1bf6d1e58547bd826f587aa729cc60d93df4a464a8c0b3f409b2d83337192323` |
| `{RUN}/lanes/B-CANVAS/briefs/B-CANVAS_addendum_5.md` | `9bb00659a2656e5de1b2ef85c330042e9d193aff1089205a6f846f251b7dddd2` |

Verify each hash before you rely on it. Addendum 4 gives your reading order after these ("If you are a new manager") and your first steps from the pause point; addendum 5 governs where it differs from addendum 4. Your first return is the candidate of the lane's first pull request: C1's first part with F1 to F3 disposed, T1, C1E and C1b. The sequence is the control layer first; the visual redesign is last.

## Three things this session adds

1. **The owner works under a usage limit.** Prefer few, well-briefed children: one child per slice where the slice allows it, a slice small enough to do yourself done yourself, Sonnet for mechanical work against an exact inventory. Do not run the full Playwright lanes more often than a return needs.
2. **Return at each frozen candidate and stop**, as the lane brief says. Your final message is the return: its id, candidate commit, diff range against `origin/main`, the checks with commands and results (the picking tests first), the controls touched with **contrast not yet checked**, **appearance left for the closing pass** with the numbers you have, semantic changes named, what you bring back, what is next, and the model you ran as, by your own statement. ROOT retains it, dispatches the independent review from another worktree, and resumes you by message with findings or the go-ahead.
3. **Ports.** Do not bind 5183 or 5184; ROOT uses them to look at builds. Ports 5174 and 5175 only through `with_e2e_lock.sh`. No timed measurement runs in this session unless ROOT announces it; T1's guidance runs are not timed qualification, and they still go through the lock.

## Pausing

If ROOT asks you to pause, stop at a clean point: commit or leave the worktree clean, hold no browser-test lock, leave no server running, and end your turn with the head commit, what is done and what is next.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
