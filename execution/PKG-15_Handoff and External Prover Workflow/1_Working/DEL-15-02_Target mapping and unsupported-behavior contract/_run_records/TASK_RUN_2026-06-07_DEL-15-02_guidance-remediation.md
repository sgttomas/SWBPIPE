---
run-id: TASK_RUN_2026-06-07_DEL-15-02_guidance-remediation
run-status: SUCCESS
deliverable-id: DEL-15-02
package-id: PKG-15
agent: TASK
parent-agent: WORKING_ITEMS
task-skill: NONE
date: 2026-06-07
lifecycle-changes: not_authorized
review-disposition-edits: not_authorized
---
# TASK Run Record - DEL-15-02 Guidance Remediation

## Objective

Address formal REVIEW findings `RF-001` and `RF-002` by aligning `Guidance.md` and OI-015 language with the June 7 target-mapping schema/test evidence.

## Outputs

- Updated `Guidance.md` to cite `schemas/target_mapping.schema.json`, `core/handoff/target_mapping/contract.py`, provider-neutral behavior fields/statuses, and `tests/test_target_mapping_contract.py`.
- Removed setup-era claims that schema property names, exact taxonomy values, and schema path were unresolved.
- Narrowed OI-015 language to preserve gates for concrete mappings, target field coverage, target-specific taxonomy extensions, canonical package container, and target-specific implementation.

## Validation

- `python3 tests/test_target_mapping_contract.py` passed.
- Dependency CSV validation passed for `DEL-15-02`.
- Targeted stale-phrase scan found no remaining matching setup-era phrases.
- `git diff --check -- projects/chirality-piping` passed.

## Boundaries

No `_STATUS.md`, `Review_Findings.csv` `HumanDisposition`, dependency CSV, schema, code, or test edits were made by this remediation pass.
