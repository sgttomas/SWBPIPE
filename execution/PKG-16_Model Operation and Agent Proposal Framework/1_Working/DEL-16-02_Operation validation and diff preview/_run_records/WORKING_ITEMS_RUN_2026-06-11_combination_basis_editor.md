---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-BASIS-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-COMBBASIS-001
---

# WORKING_ITEMS Run - Combination Basis Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation and diff-preview seam now accepts explicit
`Combination.basis` text edits.

## Implementation Evidence

- Browser local validation and Rust `operation_applier` add `basis` to the
  `Combination` text field rule set.
- Accepted intents must target an existing `Combination`, use
  `change_kind=update_load`, carry `field_path=basis`, provide a current
  before-value, non-empty after-value, unit `none`, and dimension
  `dimensionless`.
- Whole `Combination.terms` replacement remains deferred with
  `OP-FIELD-EDIT-DEFERRED`.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 31/31 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 52/52 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- In-app browser smoke at `http://127.0.0.1:5175/` confirmed the rendered
  basis edit applies only after user action and leaves persistence at
  `session_state_only_not_yet_saved`.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche does not add direct model mutation, whole-term replacement,
combination term creation/deletion, code/rule combinations, unit conversion,
protected/private data, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

Operation validation still needs future A4 coverage for combination term
creation/deletion and broader algebra authoring, plus Phase B unit conversion
and unit picker/display retirement.
