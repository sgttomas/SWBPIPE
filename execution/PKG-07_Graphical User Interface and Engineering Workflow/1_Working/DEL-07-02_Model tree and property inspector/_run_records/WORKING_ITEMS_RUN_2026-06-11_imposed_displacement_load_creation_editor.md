---
doc_id: WORKING-ITEMS-RUN-2026-06-11-IMPOSED-DISPLACEMENT-LOAD-CREATION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-IMPOSED-001
---

# WORKING_ITEMS Run - Imposed-Displacement Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The model-tree/right
rail Load Cases manager now includes a support-target
imposed-displacement primitive-load create surface.

## Implementation Evidence

- The category selector includes `imposed_displacement` and switches the
  target picker from nodes/pipes to existing supports.
- The direction selector switches to support DOFs `UX|UY|UZ|RX|RY|RZ`.
- The preview updates identifier suffix `I`, target display, unit, dimension,
  and source note before queue/apply.
- Applied manager rows display support-target primitive loads as
  `support:<support id>` with their selected DOF and dimension.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 30/30 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 50/50 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests; the smoke
  checks translational `UZ` preview (`unit=m; displacement`) and rotational
  `RX` preview (`unit=rad; rotation`).
- In-app browser smoke at `http://127.0.0.1:5175/` applied
  `load:L-100-I300` through the manager and confirmed the post-apply row and
  zero pending operations. The browser plugin could not type a replacement
  magnitude because its virtual clipboard was unavailable, so this browser
  smoke used the default finite `250 m`; the negative value path is covered by
  Vitest.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add combination basis editing, combination term
creation/deletion, broader algebra authoring, unit conversion UI, saved-project
mutation without user Save, support coordinate policy, solver boundary
behavior, protected standards content, private data, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residual

The Load Cases manager still needs combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.
