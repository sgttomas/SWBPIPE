---
doc_id: WORKING-ITEMS-RUN-2026-06-11-PRESSURE-THERMAL-LOAD-CREATION-EDITOR-DEL-07-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-07
deliverable_id: DEL-07-02
tranche_id: TP-APP-R2-PRESSTEMP-001
---

# WORKING_ITEMS Run - Pressure/Thermal Load Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The Load Cases manager
UI now lets a user author explicit pressure and thermal primitive-load records
without editing raw model JSON.

## Implementation Evidence

- The create-primitive form adds selectable `pressure` and `thermal`
  categories and switches their target control to the pipe selector.
- Generated primitive ids use category-specific suffixes `P` and `T`
  (`load:L-100-P300`, `load:L-100-T300` in the invented preview model).
- The preview text exposes target, direction, unit, dimension,
  direct-mutation boundary, and professional-approval boundary before queueing.
- App tests apply both records through the manager and verify the resulting
  manager rows and local review context.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 29/29 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 48/48 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` applied the pressure and
  thermal create intents in clean sessions and confirmed row insertion,
  zero pending operations, and stale solve reset.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This is a local technical-preview authoring UI slice only. It does not add
durable persistence without user Save, release readiness, professional
approval, certification, sealing, authentication, code-compliance claims,
protected standards content, private data, network paths, or telemetry paths.

## Residual

The Load Cases manager still needs imposed-displacement authoring, combination
basis editing, combination term creation/deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project
smoke over edited load data.
