---
run-id: TASK_RUN_2026-06-04_DEL-17-09_source-basis-boundary-guardrails
run-status: SUCCESS
deliverable-id: DEL-17-09
package-id: PKG-17
agent: TASK
parent-agent: WORKING_ITEMS
tranche: TP-PKG17-SIBLING-GUARDRAIL-HARDENING-001
date: 2026-06-04
lifecycle-changes: not_authorized
coordination-dag-edits: not_authorized
release-claims: not_made
professional-claims: not_made
---
# TASK Run Record - DEL-17-09 Source-Basis and Boundary Guardrails

## Objective

Harden export adapter SDK admission packages so contract-level source basis is present and target admission requires target-specific source evidence.

## Files Updated

- `core/handoff/export_adapter_sdk/package.py`
- `tests/test_export_adapter_sdk.py`
- `Specification.md`
- `MEMORY.md`
- This run record.

## Work Performed

- Added required adapter contract source-basis refs for DEL-17-01 and DEL-17-02.
- Added a blocking diagnostic when a `source_basis_admitted` target lacks target-specific source evidence beyond package-level contract refs.
- Added focused regression coverage for contract refs and target-specific admission evidence.

## Validation

- Python compilation for touched modules/tests: PASS.
- Focused pytest suite for DEL-17-05 through DEL-17-09: PASS, 40 passed.
- JSON parse check for touched schemas and fixtures: PASS.
- Deliverable-local checks for DEL-17-09 four documents, minimum fileset, dependency schema, semantic matrix, and lens register: PASS.

## Boundaries Preserved

- No lifecycle transition.
- No coordination DAG edit.
- No runtime loader, public endpoint, target writer, compatibility claim, release claim, code-compliance claim, solver-validation claim, or professional-acceptance claim.
