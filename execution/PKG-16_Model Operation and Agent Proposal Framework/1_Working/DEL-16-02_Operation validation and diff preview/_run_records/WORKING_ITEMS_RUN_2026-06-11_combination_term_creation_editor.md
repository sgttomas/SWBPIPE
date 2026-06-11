---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-CREATION-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-COMBTERMCREATE-001
---

# WORKING_ITEMS Run - Combination Term Creation Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation and diff-preview seam now accepts explicit
`create_combination_term` payloads.

## Implementation Evidence

- Browser local validation and Rust `operation_applier` add
  `create_combination_term` to the supported operation taxonomy.
- Accepted intents must target an existing `Combination` with
  `field_path=terms`, use `operation_kind=create`, carry
  `before=not_present`, unit `none`, dimension `dimensionless`, and JSON
  payload `{ load_case: <existing load id>, factor: <finite number> }`.
- Diff preview reports the append as a single `terms` create row. Missing load
  cases, invalid payloads, wrong unit/dimension metadata, and missing
  combination `terms` arrays are blocked.
- Whole `Combination.terms` replacement remains deferred for ordinary
  inspector field editing.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 32/32 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 54/54 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed the rendered
  term creation applies only after user action and leaves persistence at
  `session_state_only_not_yet_saved`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add direct model mutation, whole-term replacement,
combination term deletion, code/rule combinations, unit conversion,
protected/private data, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

Operation validation still needs future A4 coverage for combination term
deletion and broader algebra authoring, plus Phase B unit conversion and unit
picker/display retirement.
