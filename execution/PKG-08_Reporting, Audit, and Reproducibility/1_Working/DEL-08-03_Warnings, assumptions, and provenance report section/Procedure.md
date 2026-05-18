# Procedure: DEL-08-03 Warnings, assumptions, and provenance report section

## Purpose

This procedure describes how a future TASK implementation should produce and verify the warnings, assumptions, and provenance report section while preserving the sealed scope, protected-data boundary, diagnostics/result-envelope boundary, and professional-responsibility boundary.

## Prerequisites

| Prerequisite | Expected Source | Status |
|---|---|---|
| Sealed DEL-08-03 brief and explicit write scope | `_CONTEXT.md` and human dispatch | Available for setup |
| Applicable invariants and architecture basis | `docs/CONTRACT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | Available for setup |
| Diagnostics/result-envelope contract | AB-00-06 and future schema/interface artifacts | Partly available; exact implementation `TBD` |
| Report generator integration point | DEL-08-01 | Upstream/integration dependency |
| Audit manifest/hash metadata | DEL-08-02 | Upstream/integration dependency |
| Warning data from GUI/core | DEL-07-04, DEL-07-07, solver/rule diagnostics deliverables | Upstream/integration dependency |
| Protected-content guardrail | DEL-08-05 and IP/data policy | Upstream/integration dependency for linter implementation |

## Steps

1. Confirm scope.
   - Verify the work remains limited to DEL-08-03 report-section behavior.
   - Do not implement unrelated report generator, manifest, export, linter, GUI, solver, or rule-engine code.

2. Identify input payloads.
   - Use diagnostics/result envelopes as the source for warning class, code, severity, source, affected object, message, remediation, and provenance.
   - Use report/audit manifest inputs for software version, solver version, model hash, rule-pack name/version/checksum, and source notes where supplied.
   - Mark unavailable fields `TBD` instead of inventing values.

3. Map report subsections.
   - Missing data findings.
   - Warnings grouped or filterable by class/severity/source.
   - Assumptions requiring review.
   - User-supplied values and private/public source status.
   - Source/provenance summary.
   - Professional-responsibility and protected-content notices.

4. Preserve boundaries.
   - Do not reproduce protected standards text, copied standards tables, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary vendor data in public templates/examples.
   - Do not claim certification, sealing, approval, authentication, endorsement, or automatic code compliance.
   - Do not transmit or publish private project/rule-pack/library data by default.
   - Bind final professional-boundary wording to a canonical product-claims policy or approved report notice when available; until then, keep wording conservative and mark exact final text `TBD`.

5. Implement or refresh report fixtures.
   - Include at least one fixture for each warning class listed in the diagnostics basis where upstream fixtures exist.
   - Include fixtures for missing solve data, missing rule-check data, missing provenance, assumptions, private rule-pack reference, and human-review-required status.
   - Use invented/non-code data only for public examples.

6. Verify output.
   - Run report snapshot or equivalent tests for expected section content.
   - Run protected-content lint once DEL-08-05 or equivalent tooling is available.
   - Confirm unit-bearing values retain units.
   - Confirm status language distinguishes mechanics solve, user rule check, and human review.

7. Record evidence.
   - List changed report-section files and tests.
   - Record validation commands/results.
   - Record warnings, `TBD` fields, and open dependencies.

## Verification

| Verification Item | Acceptance Signal |
|---|---|
| Four-document setup | `tools/validation/check_four_documents.sh <deliverable_path>` passes |
| Semantic setup | `_SEMANTIC.md` exists and semantic audit records PASS |
| Lensing setup | `_SEMANTIC_LENSING.md` exists with complete matrix coverage |
| Dependency setup | `Dependencies.csv` validates against v3.1 schema |
| Dependency schema command | `python3 tools/validation/validate_dependencies_schema.py <deliverable_path>/Dependencies.csv` passes |
| Scope guard | No files outside the DEL-08-03 working folder are modified by this setup run |
| Protected-data guard | Public setup artifacts contain no protected standards text/tables/formulas or proprietary data |
| Professional-boundary guard | Setup artifacts do not claim certification, approval, sealing, endorsement, or software-authenticated code compliance |
| Lint fallback guard | If protected-content lint tooling is unavailable, record that status as an open verification item requiring human review |

## Records

This setup run should leave these records in the deliverable folder:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/`
- `_STATUS.md`
