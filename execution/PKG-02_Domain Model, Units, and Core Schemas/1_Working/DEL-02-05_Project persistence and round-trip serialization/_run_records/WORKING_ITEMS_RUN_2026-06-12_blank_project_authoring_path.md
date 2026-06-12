---
run_id: WORKING_ITEMS_RUN_2026-06-12_blank_project_authoring_path
tranche_id: TP-APP-R2-BLANK-001
date: 2026-06-12
persona: WORKING_ITEMS
primary_deliverable: DEL-02-05
package: PKG-02
completion_plan_item: A9
status: SUCCESS
---

# WORKING_ITEMS RUN — TP-APP-R2-BLANK-001 Blank Project Authoring Path

## Objective

Implement completion-plan Phase A9: create a new empty local model document in
the desktop app, persist it through the local project boundary, make it the
active authoring target, and preserve honest `MODEL_INCOMPLETE` solve/report
gating without seeding the invented preview fixture.

## Authority And Scope

- Authority intake: `AGENT_WORKING_ITEMS.md`, `AGENT_TASK.md`,
  `execution/_Coordination/_COORDINATION.md`,
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`, `docs/DIRECTIVE.md`,
  `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  `docs/PRD.md` sections 10 and 22,
  `execution/_Decomposition/SOFTWARE_DECOMP.md`,
  `execution/_DAG/_LATEST.md`,
  `execution/_DAG/DAG-006/APPROVAL_RECORD.md`,
  `plans/PLAN_2026-06-10_prd_completion.md`, and
  `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- Selected tranche: Phase A9, earliest unblocked current-stage item after the
  landed A1/A2/A7 and partial A3-A8 slices.
- Exclusion honored: no work on
  `plans/PLAN_2026-06-12_caepipe_external_oracle_feedback_loop.md` or SCA-005.
- Write scope used: `apps/desktop/src/**`, `apps/desktop/src-tauri/src/lib.rs`,
  `apps/desktop/SMOKE.md`, `plans/PLAN_2026-06-10_prd_completion.md`,
  `plans/PLAN_COMPLETION_LOG.md`, and deliverable-local memory/run-record
  evidence for DEL-02-05 and DEL-07-07.

## Changes

- Added `buildBlankLocalModelDocument()` in
  `apps/desktop/src/services/projectService.ts`. It creates a
  `project:blank-local-*` model document with:
  - current schema version `0.1.0`;
  - no materials, nodes, pipe segments, supports, components, load cases, or
    combinations;
  - explicit `MODEL_INCOMPLETE`, `RULE_INPUTS_INCOMPLETE`, and `NOT_PROVIDED`
    status;
  - visible data-boundary fields for no protected content, no repository
    private-data write, and human-review-only use;
  - blocking `BLANK_PROJECT_AUTHORING_TARGET` diagnostic.
- Added `New blank` to the local project toolbar in `apps/desktop/src/App.tsx`.
  The action computes model and project-envelope hashes, persists the blank
  document via `createLocalProject`, makes the returned document active,
  resets queued operations, undo/redo, proposals, result state, and analysis
  run state, and records `create_blank` as the project operation.
- Added frontend regression coverage:
  - `projectService.test.ts`: blank document factory and browser-memory
    persistence/open evidence.
  - `App.test.tsx`: blank action sets active authoring target, storage and
    validation evidence show persisted blank state, browser solve returns a
    zero-row incomplete result, and report output carries 0 refs.
- Added Tauri backend regression coverage:
  `run_preview_mechanics_reports_blank_model_incomplete_without_defaults`
  proves a supplied blank payload returns `MODEL_INCOMPLETE`, zero result rows,
  and `NODE_INPUT_MISSING`, `PIPE_INPUT_MISSING`, and `LOAD_INPUT_MISSING`.
- Updated plan/evidence surfaces: completion-plan A9 row, completion log, and
  `apps/desktop/SMOKE.md` TP-MAC-116.

## Validation

Passed:

- `npm test --workspace apps/desktop -- src/services/projectService.test.ts`
  — 5/5.
- `npm test --workspace apps/desktop -- src/App.test.tsx` — 32/32.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml run_preview_mechanics_reports_blank_model_incomplete_without_defaults`
  — 1/1.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` — 29/29.
- `npm test --workspace apps/desktop` — 174/174 when run without competing
  cargo build work.
- `npm run build --workspace apps/desktop` — passed.
- `npm run test:e2e --workspace apps/desktop` — Playwright R2 smoke 1/1.
- Live in-app browser smoke at `http://127.0.0.1:5173/`:
  - blank state: `MODEL INCOMPLETE`, `RULE INPUTS INCOMPLETE`, one project
    entity, `operation=create_blank`, no persisted mechanics results,
    `network=false`, zero console errors;
  - browser solve: `state=completed`, `result_rows=0`,
    `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, result table
    `0 of 0`, report export `0 refs; 4 diagnostics; no private payload`.

Notes:

- An attempted `vitest --runInBand` invocation failed because Vitest does not
  support that Jest option; the same focused App test passed with the supported
  Vitest command.
- One full desktop Vitest run timed out in the first large app test while
  running concurrently with Rust compilation/tests. Rerun by itself passed
  174/174; this is recorded as an execution-contention artifact, not a code
  failure.
- DEC-025 commit-bound evidence sweep is run after the tranche commit per
  `docs/BUILD_AND_RELEASE.md` section 5.1.

## Boundary Review

- No protected standards content, code tables, proprietary data, private rule
  packs, or private project data were added to the repository.
- The blank model contains no hidden fixture entities, loads, materials,
  supports, or engineering defaults.
- User-created blank models are local project-store payloads only; no
  repository-default private-data write, cloud, daemon, network, or telemetry
  path is introduced.
- The UI and reports retain `MODEL_INCOMPLETE` / `RULE_INPUTS_INCOMPLETE` and
  human-review-only language. No release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim is created.

## Residual Hand-Offs

- A10 is next unblocked Phase A scope: add support/material/section creation
  operation kinds, corpus cases, and UI forms so a blank model can progress
  toward a solvable from-scratch model.
- A11 deletion coverage remains open.
- A8/A5 still need packaged-Tauri saved-project smoke over the blank path.
- Phase B unit conversion/display work remains separate.
