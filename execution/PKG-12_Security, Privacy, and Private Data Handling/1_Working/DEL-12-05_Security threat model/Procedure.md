# Procedure: DEL-12-05 Security threat model

## Purpose

This procedure defines how a bounded TASK worker drafts, refreshes, and checks the DEL-12-05 security threat model setup content. It is intended for deliverable-local setup work only and does not publish `docs/security/threat_model.md`.

## Prerequisites

| Prerequisite | Required handling |
|---|---|
| Sealed deliverable context | Confirm `DEL-12-05`, `PKG-12`, `SOW-040`, `OBJ-010`, and the allowed write scope before editing. |
| Governance sources | Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, relevant `SOFTWARE_DECOMP.md` revision 0.4 rows, register rows, and deliverable-local metadata. |
| Source boundary | Use only accessible source material; mark missing implementation details `TBD`. |
| Write boundary | Write only inside this deliverable folder. Do not create or edit repo-level `docs/security/threat_model.md`. |
| Protected/private data boundary | Do not include protected standards text/tables/data, real private project data, real secrets, legal sufficiency claims, certification claims, or professional approval claims. |
| Status boundary | Keep `_STATUS.md` at `SEMANTIC_READY` after a successful setup refresh; do not mark `ISSUED`. |

## Steps

1. Confirm the deliverable identity and scope.
   - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, and `docs/_Registers/ContextBudgetQA.csv`.
   - Verify the scope is limited to DEL-12-05 setup content.

2. Re-read governing sources.
   - Use `docs/CONTRACT.md` for invariant constraints.
   - Use `docs/PRD.md` sections 17-18 for private data, telemetry, report, and bug-report requirements.
   - Use `docs/SPEC.md` sections 1 and 6-8 for architecture, rule-pack, diagnostics, and reporting boundaries.
   - Use `docs/IP_AND_DATA_BOUNDARY.md` for provenance, quarantine, private data, and report boundaries.
   - Use `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for `PKG-12`, `SOW-040`, `OBJ-010`, and architecture basis rows `AB-00-01/02/03/04/06/07/08`.

3. Draft or refresh the four documents.
   - Keep `Datasheet.md` sections `Identification`, `Attributes`, `Conditions`, `Construction`, and `References`.
   - Keep `Specification.md` sections `Scope`, `Requirements`, `Standards`, `Verification`, and `Documentation`.
   - Keep `Guidance.md` sections `Purpose`, `Principles`, `Considerations`, `Trade-offs`, and `Examples`.
   - Keep `Procedure.md` sections `Purpose`, `Prerequisites`, `Steps`, `Verification`, and `Records`.
   - Label unknown implementation details `TBD`; label inferences `ASSUMPTION` if used.

4. Build the semantic matrix artifact.
   - Refresh `_SEMANTIC.md` using the deliverable perspective and the four documents.
   - Verify the final result tables for matrices `C`, `F`, `D`, `X`, and `E` have no algebra/operator leaks and no matrix errors.
   - Set or verify `_STATUS.md` as `SEMANTIC_READY` on pass.

5. Build the semantic lensing register.
   - Refresh `_SEMANTIC_LENSING.md` with complete `A`, `B`, `C`, `F`, `D`, `X`, and `E` lens coverage.
   - Record only warranted gaps, conflicts, or questions; do not treat lens cells as authority.

6. Apply warranted lensing items conservatively.
   - Re-read the target document section and relevant source slice before changing content.
   - Incorporate only supported items.
   - Convert unsupported items to `TBD` or add them to the conflict table.

7. Refresh dependency extraction.
   - Refresh `Dependencies.csv` using v3.1 columns.
   - Preserve human-owned coordination notes in `_DEPENDENCIES.md`.
   - Capture anchor links to `SOW-040` and `OBJ-010`, plus execution dependencies on governing documents that directly constrain the threat model.

8. Create run records.
   - Write one run record for each setup phase: four-documents P1/P2, semantic matrix, lens register, four-documents P3, and dependency extraction.
   - Each record must include `run-status: SUCCESS` or a concrete failure.

9. Run validation.
   - Run `python3 tools/validation/validate_dependencies_schema.py <ScopePath>/Dependencies.csv`.
   - Check `_SEMANTIC.md` for `MatrixError`/`MATRIX_ERROR`.
   - Check final result table rows for leaked algebra/operator tokens.

## Verification

| Verification item | Method |
|---|---|
| Four documents exist | File presence check for `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`. |
| Default sections preserved | Heading check for required sections in all four documents. |
| Semantic audit clean | Search `_SEMANTIC.md` for matrix errors and inspect final result tables for algebra/operator leaks. |
| Lensing coverage complete | Confirm `_SEMANTIC_LENSING.md` includes matrix sections for `A`, `B`, `C`, `F`, `D`, `X`, and `E`. |
| Dependency schema valid | Run the v3.1 dependency schema validator. |
| Status safe | Confirm `_STATUS.md` says `Current State: SEMANTIC_READY`, not `ISSUED`. |
| Write scope respected | Review changed paths before final report; no writes outside this deliverable folder. |

## Records

Required records for this run:

- `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/PHASE_2_2_four_documents_P1_P2_2026-04-30.md`
- `_run_records/PHASE_2_3_semantic_matrix_2026-04-30.md`
- `_run_records/PHASE_2_4_lens_register_2026-04-30.md`
- `_run_records/PHASE_2_5_four_documents_P3_2026-04-30.md`
- `_run_records/PHASE_2_6_dependency_extract_2026-04-30.md`

