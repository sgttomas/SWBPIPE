# SWBPIPE Tranche B, slices B2F and B2G — project-handler repairs: bounded technical evidence

Date: 2026-09-19 (UTC). HELP_HUMAN (ROOT, Claude Fable 5.1; this session began as Claude Opus 5 and the host changed its model during the day) with a Type 1 WORKING_ITEMS lane manager (Claude Fable 5.1), which dispatched one Type 2 TASK implementer for B2F and did B2G and its two corrections itself, and two Type 2 read-only reviewers (Claude Opus 5) dispatched by ROOT, under the owner's implementation authorization of 2026-09-18 and the re-sequencing of 2026-09-19 (the control layer first). Mechanism: Claude Code `Agent` tool children, nested one level; the implementer ran in the foreground of its manager.

## Accepted basis

The implementation handoff of the design program and its ten constraints, the third above all: result integrity preserved. Source base `64f86e17f`. Instructions: the shell lane's sealed addendum 2 (slice B2F) and addendum 4 (slice B2G), indexed at `{RUN}/lanes/B-SHELL/INDEX.md`.

## Implemented behaviour

Two product files: `src/features/workspace/workspaceSession.ts` and the new `src/App.projectHandlers.test.tsx` (14 tests, each written first and seen to fail). Named semantic changes:

- Opening a project clears the previous project's operation diagnostics from the issue count and the Issues drawer, as a blank create already did.
- An open that finds nothing, fails or is superseded, and a blank create that fails, leave the still-open project's two integrity cells, its rule revision gate and an in-flight rule-check revision as they were. Before, a recorded `mismatch_review_required` read as `open_verification_not_run_this_session` on a project that stayed open.
- A save or create that fails or is superseded before its bytes land leaves the two integrity cells as they were. Once its bytes have landed, the open-time record is cleared under the request-number test, before the epoch test, so that a model edit made while the request was pending cannot leave `verified_match` standing over bytes that were since rewritten. A blank create writes a new project and so never clears the open project's record before it commits.
- Listing projects does not start while a project request is busy, and releases only a busy state it owns.
- Left as it is, with reasons recorded: `handleApplyIntent`'s hand reset (inert); on a successful save the cells are nulled and not re-derived (a design question, open).

No guard condition, gate class, `commitModelAfterSolveInvalidation`, `clearComputedModelState`, solve-input basis, designation, undo or redo changed. `runMenuCommand` and every pre-existing test are byte-identical to the base. No control, copy or stylesheet change.

## Evidence

Reviews, briefs and returns with hashes: `{RUN}/lanes/B-SHELL/INDEX.md` and `{RUN}/lanes/B-SHELL/CLOSEOUT_CHECKS_B2F_B2G.json`. The second reviewer found, by reproduction, an edge the manager had called unreachable, and then withdrew its own note that had carried the correction one handler too far; both corrections were made test first and backchecked. DEC-025 sweep: pass on all five surfaces at `a1f7f8fe14f7a1266e595cc8bea40f518e4154c8`, summary SHA-256 `a1fc99144b8ba402b02d1ce175f5086864b6c26c03cddbc41accba3b61825f37`.

## Remaining work and authority boundary

For the owner, none blocking: the native menu is not disabled while a project request is busy, an existing test pins that reachability, and closing it needs `src-tauri/**`; with it, a save superseded by another project request after its bytes landed keeps the open-time record. Open design question: whether a landed save should re-derive the integrity record. No usability, conformance or performance acceptance is claimed; PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
