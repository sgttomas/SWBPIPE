# Procedure: DEL-14-05 Comparison mapping, tolerance, and export contracts

## Purpose

This procedure describes how to produce and verify the DEL-14-05 contract artifacts using only the current governed source basis. It is a setup-stage production procedure, not an implementation procedure for runtime comparison code.

## Prerequisites

| Prerequisite | Source |
|---|---|
| Deliverable context for DEL-14-05 is present and readable. | `_CONTEXT.md` |
| Accepted SOFTWARE_DECOMP revision 0.5 is available. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Governing project invariants are available. | `_REFERENCES.md`; `docs/CONTRACT.md`; `docs/TYPES.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/DIRECTIVE.md` |
| Approved DAG-002 local mirror is present and preserved. | `_DEPENDENCIES.md`; `Dependencies.csv`; `execution/_DAG/DAG-002/APPROVAL_RECORD.md` |
| Upstream architecture basis rows remain context evidence only. | `_CONTEXT.md#Architecture-Basis-Injection`; `_DEPENDENCIES.md#Authority-Boundary` |

Approved local dependency mirror, summarized:

- architecture basis: DEL-00-01, DEL-00-02, DEL-00-03, DEL-00-04, DEL-00-06, DEL-00-07, DEL-00-08;
- state/run prerequisites: DEL-14-01 and DEL-14-02;
- result export prerequisite: DEL-08-04;
- unit contract prerequisite: DEL-02-02.

## Steps

1. Confirm scope identity from `_CONTEXT.md`: DEL-14-05, PKG-14, API_CONTRACT, SOW-073, OBJ-016.
2. Confirm decomposition scope from `execution/_Decomposition/SOFTWARE_DECOMP.md`: DEL-14-05 defines manual mappings, unmatched classifications, tolerance profiles, and CSV/JSON/report-section export semantics.
3. Record exclusions before defining contract details: no comprehensive commercial prover result ingestion, no external validation decision, no professional approval state, and no protected/private data in public artifacts.
4. Define the comparison mapping schema surface at a contract level:
   - stable source and target references;
   - manual mapping evidence/provenance;
   - relation to model-state and analysis-run comparison outputs;
   - unresolved fields marked `TBD`.
5. Define the unmatched classification surface:
   - explicit classification slot for entities/results without an accepted counterpart;
   - specific enum values marked `TBD` unless later source material supplies them.
6. Define the tolerance profile surface:
   - unit-aware comparison quantities and delta interpretation;
   - required unit/dimensional metadata;
   - tolerance default values marked `TBD`.
7. Define JSON export semantics:
   - schema-first envelope basis;
   - result set, model/run basis, solver version, unit-system reference, diagnostics, provenance, hashes or audit-manifest reference, analysis statuses, and professional-boundary notice;
   - exact fields marked `TBD`.
8. Define CSV export semantics:
   - parseable tabular representation for comparison review;
   - no protected standards text, proprietary values, private project data, or private rule-pack payloads;
   - columns and ordering marked `TBD`.
9. Define report-section export semantics:
   - report-facing comparison evidence, limitations, diagnostics, assumptions, and professional-boundary notice;
   - final layout marked `TBD`.
10. Verify the resulting contract draft against unit safety, data-boundary, professional-boundary, and no-silent-defaults invariants.
11. Preserve approved DAG-002 mirror rows as ACTIVE. Do not delete, retire, or reclassify them during setup handling.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Contract content is limited to DEL-14-05 and does not implement DEL-14-03 or DEL-14-04 engines. |
| Source-grounding check | Non-trivial requirements cite `_CONTEXT.md`, `_REFERENCES.md`, decomposition, local dependency mirror, or referenced governance docs. |
| TBD check | Unsupported details are marked `TBD` or `ASSUMPTION`; no engineering values are invented. |
| Unit check | Unit-bearing comparison/export values are specified as unit-aware or diagnostic-producing. |
| Boundary check | No professional approval, certification, code-compliance, or external-validation claim is introduced. |
| IP/privacy check | No protected standards content, proprietary data, private project data, or private rule-pack payload is embedded. |
| Dependency mirror check | Existing approved DAG-002 rows remain present and ACTIVE. |

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
