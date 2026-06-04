---
run-id: TASK_RUN_2026-06-04_DEL-17-05_source-basis-boundary-guardrails
run-status: SUCCESS
deliverable-id: DEL-17-05
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
# TASK Run Record - DEL-17-05 Source-Basis and Boundary Guardrails

## Objective

Harden the CAEPIPE external-run evidence package so attempted execution is explicitly bound to DEL-17-04 MBF export evidence and user-owned responsibility acknowledgements.

## Files Updated

- `core/handoff/caepipe_external/run.py`
- `tests/test_caepipe_external_run_package.py`
- `Specification.md`
- `MEMORY.md`
- This run record.

## Work Performed

- Added a blocking diagnostic when attempted external-run evidence does not bind to a DEL-17-04 CAEPIPE MBF export package reference.
- Added blocking diagnostics when attempted external execution lacks license-responsibility or environment-responsibility acknowledgement.
- Added focused regression coverage for unsafe MBF references and missing acknowledgements.

## Validation

- Python compilation for touched modules/tests: PASS.
- Focused pytest suite for DEL-17-05 through DEL-17-09: PASS, 40 passed.
- JSON parse check for touched schemas and fixtures: PASS.
- Deliverable-local checks for DEL-17-05 four documents, minimum fileset, dependency schema, semantic matrix, and lens register: PASS.

## Boundaries Preserved

- No lifecycle transition.
- No coordination DAG edit.
- No bundled executable, license-bypass path, live CAEPIPE invocation, compatibility claim, release claim, code-compliance claim, solver-validation claim, or professional-acceptance claim.
