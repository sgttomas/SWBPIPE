# Sealed brief — B2G-REVIEW: independent code review of slice B2G, and of the combined candidate B2F with B2G

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19 before launch. Role: TASK (Type 2) with the `software-code-review` skill, read-only, fresh context, working alone; Type 2 does not delegate. Model requested: Claude Opus 5. Mechanism: Claude Code `Agent` tool, general-purpose type. You have no write target: your return is your final message, which ROOT retains at `{RUN}/lanes/B-SHELL/reviews/B2G-REVIEW_RETURN.md`.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the worktree you are started in; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DESIGN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`. Write no absolute machine path in your return; if you must name a path pattern you searched for, describe it in words.

Load the skill body first: `{REPO_ROOT}/.agents/skills/software-code-review/SKILL.md` (SHA-256 `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` at sealing; report the hash you read). Where it and this brief differ, this brief's limits win.

## Candidate

ROOT merges slices B2F and B2G as **one pull request**. The combined candidate is `0fc5b065e4822fea37ee8cf748f83e45eb3cf9df` against `64f86e17f` (`origin/main`, the merge base): `git diff 64f86e17f...0fc5b065e -- projects/chirality-piping/apps/desktop`, two files, `src/features/workspace/workspaceSession.ts` and `src/App.projectHandlers.test.tsx` (11 tests).

- **B2F** (`64f86e17f..dfa7c47d5`, product commits `ef81fc32e` and `572c4a34b`) was reviewed in this session by another reviewer: PASS, no actionable finding (`{RUN}/lanes/B-SHELL/reviews/B2F-REVIEW_RETURN.md`). That reviewer's context is not yours. Read its return after your own reading; you do not repeat its work, but you do own the combined diff: say whether anything in B2G disturbs what it established.
- **B2G** (`dfa7c47d5..0fc5b065e`, product commits `9b7687da9` tests and `74707b134` repair) is new and is yours at 100 %. Instruction: `{RUN}/lanes/B-SHELL/briefs/B-SHELL_addendum_4.md`, section "Slice B2G". The lane brief is `{RUN}/briefs/B-SHELL_shell_lane.md`; the design handoff's constraints are `{DESIGN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md` §3. Read the manager's return (`{RUN}/lanes/B-SHELL/returns/B2G_RETURN.md`) last.

## What to look for in B2G

1. **The move.** In `handleCreateProject` and `handleSaveProject`, `setModelHashIntegrity(null)` and `setProjectEnvelopeHashIntegrity(null)` moved from before the first `await` to directly after the `stillCurrent` check that follows `createLocalProject` or `saveLocalProject`. Enumerate every statement in each window and everything that can run while the awaits are pending; show that no reader can take a different code path, and that on every path where the persistence landed and the response is accepted the final state equals the base's, including the PROJECT-PERSISTENCE-RESPONSE-INTEGRITY refusal path the manager names, and any path that sets a cell to a derived value later in the handler (order of the null against that later set).
2. **Failure and supersession.** On a thrown request and on `stillCurrent` false the cells now stay. Is that right in each case? Give your own reading of the manager's edge (its "What I bring back" item 2): a superseded save whose bytes were in fact written, followed by an open that finds nothing, now keeps an open-time record although the stored bytes changed. Severity, reachability by a user, and whether it should block. Note it is reachable only through the native menu during a busy request, which is already listed for the owner.
3. **Success is exactly today's.** The fourth test pins it. Confirm from the code.
4. **The tests are real**: each of the three repair tests fails on `9b7687da9`'s source for the stated reason; each asserts the model line and the envelope line; none pins appearance; the mocks behave as the real bridge does.
5. **Untouched:** `runMenuCommand` byte for byte the base's; every existing test and Playwright spec byte-identical to `64f86e17f`; no `stillCurrent` condition, gate class, `commitModelAfterSolveInvalidation`, `clearComputedModelState`, solve-input basis, designation, undo or redo changed; only the two files under `apps/desktop`; no copy, DOM, role, name, test id or stylesheet change; no tolerance, oracle or limit changed.
6. **Records** in `dfa7c47d5..0fc5b065e` carry no absolute machine path.

You may run, one at a time, from `{DESKTOP}`: `npx vitest run <file>` and `npx tsc -p tsconfig.json --noEmit`. No Playwright, sweep or dev server. No state-changing git command; modify no file.

## Return

Begin with the line `# B2G-REVIEW return`. Verdict **PASS** (no actionable finding) or **FINDINGS**, for the combined candidate. For each finding: severity, whether actionable before merge, file and line, what is wrong, the evidence, the smallest correction. Give the statements you enumerated for check 1. State what you did not check and which model you are.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
