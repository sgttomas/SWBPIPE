# SWBPIPE Tranche B, slice B2 — the workspace session extracted from `App.tsx`, no behaviour change: bounded technical evidence

Date: 2026-09-19 (UTC). HELP_HUMAN (ROOT, Claude Fable 5.1) with a Type 1 WORKING_ITEMS lane manager (Claude Fable 5.1), which dispatched one Type 2 TASK implementer in three stages, and one Type 2 read-only reviewer dispatched by ROOT, under the owner's implementation authorization of 2026-09-18. Mechanism: Claude Code `Agent` tool children, nested one level; models as each return states them. Non-delegation by the Type 2 children is instruction asserted.

## Accepted basis

The implementation handoff of the design program (PRs #796 to #799) and its ten constraints, the first three above all: one typed-operation route, tables and canvas as projections of the canonical model, and result integrity preserved. Source base `8e4c5df6ec84928ca343224e5244ccdae5771cd4`; the lane's move audit reads against `7c6784d95ab83bd958d96938c978153b36734064`. Run: `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION` (`{RUN}` below); the lane's records are under `{RUN}/lanes/B-SHELL/` and `{RUN}/instances/B-SHELL/`.

## Implemented behaviour

`src/App.tsx` goes from 4,356 lines to 1,574. The session's state, handlers and effects now live in fourteen modules under `src/features/workspace/`: the generation gates, the solve proof, the solve-job audit, the persistence integrity check, the workspace sections, the menu commands, the session model, six state hooks (chrome, model, selection, results, operations, project) and `workspaceSession.ts`, whose `useWorkspaceSession()` holds all thirteen effects in their original order and returns the session to the view in six slices.

It is a move and nothing else. No executable line was edited: the only textual change to moved code is the `export` keyword on 29 top-level declarations, and the import statements resolve to the same 192 name and module pairs. `SolveRunGenerationGate`, `RuleRevisionGenerationGate`, `commitModelAfterSolveInvalidation`, `clearComputedModelState`, all eight `stillCurrent` sites, the Current and Historical designation, the solve-input basis, reviewed application, undo and redo are byte-identical. `setModel` is still written once, in `commitModel`, and the session hands the view eleven setters, none of which reaches the model, results, history or project state. `src/App.test.tsx` is unchanged and passes. The slice draws nothing: no stylesheet, markup, role, name, test id or copy changed.

One thing the move does change, which no user can observe: hook calls are now grouped by state hook, so some declarations run in a different order within a render pass. The manager and the independent reviewer each traced every reader of the four refs written during render and found that none can see a different value.

## Evidence

The sealed briefs with hashes, the implementer's three stage returns, the manager's return, its two tools (`b2_move_audit.mjs`, `b2_order_check.mjs`) and its two written findings are indexed at `{RUN}/lanes/B-SHELL/INDEX.md` and `{RUN}/instances/B-SHELL/`. The independent code review (Claude Opus 5: PASS, no actionable finding, four residual items carried) is retained at `{RUN}/lanes/B-SHELL/reviews/B2-REVIEW_RETURN.md`. Check results with the tested and reviewed revisions are at `{RUN}/lanes/B-SHELL/CLOSEOUT_CHECKS_B2.json`.

## Remaining work and authority boundary

Carried to the lane's next slices: four handler asymmetries that the move preserved and that become a small named fix slice with failing tests first; dead code the move exposed; a lint rule or module boundary so that no component calls a state hook directly; a test of the session object's key set. The benchmark instrument's executed-file list still names `src/App.tsx` alone; that file is the canvas lane's. This record establishes no review acceptance, usability acceptance, accessibility conformance, release or lifecycle change; PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
