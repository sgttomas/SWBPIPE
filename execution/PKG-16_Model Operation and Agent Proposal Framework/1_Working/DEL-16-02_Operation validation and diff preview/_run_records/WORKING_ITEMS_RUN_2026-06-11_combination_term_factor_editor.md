---
doc_id: WORKING-ITEMS-RUN-2026-06-11-COMBINATION-TERM-FACTOR-EDITOR-DEL-16-02
doc_kind: working_items.run_record
status: draft
created: 2026-06-11
agent: WORKING_ITEMS
package_id: PKG-16
deliverable_id: DEL-16-02
tranche_id: TP-APP-R2-COMBFACTOR-001
---

# WORKING_ITEMS Run - Combination Term Factor Editor

## Scope

Bounded app-integration tranche for completion-plan A4. The structured
operation validation/diff seam now accepts dynamic
`Combination.terms.N.factor` paths as existing dimensionless numeric fields.

## Implementation Evidence

- `core/model_operations/operation_applier/src/lib.rs` adds a bare numeric
  field kind and dynamic `terms.N.factor` support for `Combination`.
- `apps/desktop/src/services/operationService.ts` mirrors the same browser
  fixture behavior.
- Whole `Combination.terms` replacement and `Combination.basis` remain
  explicitly deferred, preserving the A4 residual boundary for term
  creation/deletion and basis authoring.
- New Rust tests verify a combination factor applies without mutating the
  input model and that whole `terms` editing still blocks with an explicit
  scope finding.

## Validation

- `cargo fmt --manifest-path core/model_operations/operation_applier/Cargo.toml --check` passed.
- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` passed with 24/24 Rust tests.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 26/26 Rust tests.
- `npm test --workspace apps/desktop` passed with 37/37 Vitest tests.
- `npm run build --workspace apps/desktop` passed through `tsc -b` and Vite
  production build with the existing chunk-size warning.
- `npm run test:e2e:desktop` passed with 1/1 Playwright tests.
- `git diff --check -- . ':!init/init-prompt.md'` passed for the touched
  scope.

## Boundaries

This tranche adds only explicit dimensionless numeric validation and diff
support for existing combination term factors. It does not add load-case
creation, arbitrary primitive-load creation, combination basis editing, term
creation/deletion, hidden defaults, unit conversion, protected standards
content, private data, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claims.

## Residual

Operation validation still needs future A4 support for explicit load-case
creation, primitive-load creation/edit breadth, imposed-displacement
authoring, combination basis editing, term creation/deletion, and broader
algebra authoring when those app surfaces are selected.
