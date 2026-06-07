---
run-id: TASK_RUN_2026-06-07_DEL-15-04_guidance-remediation
run-status: SUCCESS
deliverable-id: DEL-15-04
package-id: PKG-15
agent: TASK
parent-agent: WORKING_ITEMS
task-skill: NONE
date: 2026-06-07
lifecycle-changes: not_authorized
review-disposition-edits: not_authorized
---
# TASK Run Record - DEL-15-04 Guidance Remediation

## Objective

Address formal REVIEW finding `RF-001` by aligning `Guidance.md` with the June 7 external-prover metadata schema/test evidence.

## Outputs

- Updated `Guidance.md` to cite `schemas/external_prover_metadata.schema.json`, `core/handoff/external_prover/metadata.py`, and `tests/test_external_prover_boundary_metadata.py`.
- Replaced category-only TBD examples with schema-backed metadata field groups.
- Preserved the non-authoritative metadata-only boundary, attachment-as-reference behavior, and commercial-result/lifecycle deferrals.

## Validation

- `python3 tests/test_external_prover_boundary_metadata.py` passed.
- Dependency CSV validation passed for `DEL-15-04`.
- Targeted stale-phrase scan found no remaining matching setup-era phrases.
- `git diff --check -- projects/chirality-piping` passed.

## Boundaries

No `_STATUS.md`, `Review_Findings.csv` `HumanDisposition`, dependency CSV, schema, code, or test edits were made by this remediation pass.
