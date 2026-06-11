---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRIMITIVE-LOAD-CREATION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-PRIMCREATE-001
---

# WORKING_ITEMS Run - Primitive Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The Load Cases
manager now includes a visible create-primitive form for concentrated nodal
forces and verifies that an accepted primitive appears in the manager list.

## Implementation Evidence

- `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` adds the
  concentrated-force create editor near the existing load-case create and
  primitive-magnitude controls.
- The preview declares `before=not_present`, target node, direction, unit,
  dimension `force`, `direct_model_mutation_allowed=false`, and
  `professional_approval=false`.
- The app-level test queues/applies `load:L-100-F300`, verifies the manager
  summary becomes `2 load cases; 8 primitive loads; 1 combinations`, verifies
  the `load:L-100` primitive count, and verifies the new primitive row.
- The Playwright R2 smoke verifies the rendered primitive create preview
  without queueing or applying, preserving the unchanged fixture model for
  solve and report smoke.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 26/26 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 41/41 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied the rendered
  primitive create intent and confirmed the concentrated-force row, zero
  pending operations, receipt boundary, and solve state `not_started`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

The GUI creates only an explicit concentrated nodal-force primitive after
user queue/apply. It does not infer targets, directions, units, distributed
loads, moments, imposed displacements, saved-project mutation without Save,
protected standards content, private data, or release, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

The Load Cases manager still needs distributed primitive-load creation,
concentrated moments, pressure/temperature primitive creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
