# Procedure: DEL-08-04 Result export format

## Purpose

This procedure describes how to maintain the setup artifact for the result export format and how a later implementation brief should convert the setup specification into a governed result export contract.

## Prerequisites

| Prerequisite | Why it matters | Source |
|---|---|---|
| Deliverable identity and scope for DEL-08-04 | Keeps the work bounded to SOW-046, OBJ-007, and OBJ-009. | `_CONTEXT.md` |
| PKG-00 result-envelope/API no-bypass architecture basis | Establishes schema-first envelopes, diagnostics, unit/provenance boundaries, and adapter no-bypass constraints. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03, AB-00-06, AB-00-07 |
| Unit system and dimensional-analysis core contract | Result exports need unit-aware values and dimensional checks. | `docs/CONTRACT.md` OPS-K-UNIT-1; upstream DEL-02-02 |
| Canonical domain model and result object semantics | Result exports need stable model/result references and object vocabulary. | `docs/TYPES.md` section 8; upstream DEL-02-01 |
| Audit manifest and model hash | Reproducible exports should carry or reference run/model manifest and hash metadata. | DEL-08-02; `docs/_Registers/ScopeLedger.csv` row SOW-039 |
| Solver/load/stress result producers | Export format must consume mechanics outputs without redefining solver behavior. | PKG-04 and PKG-05 deliverables |
| Rule-pack lifecycle and checksum handling | Rule-check exports need rule-pack identity/version/checksum without copying private/protected payloads. | DEL-06-04; `docs/SPEC.md` sections 6 and 8 |
| Headless runner and adapter surfaces | Downstream automation and adapter workflows need the governed envelope contract. | DEL-10-02 and DEL-10-05 |

## Steps

1. Confirm the current brief is limited to DEL-08-04 and does not authorize edits to `schemas/results.schema.yaml`, exporter source, tests, or documentation outside this deliverable folder.
2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/_Decomposition/SOFTWARE_DECOMP.md`, and the register rows for DEL-08-04 and SOW-046.
3. Treat the baseline export as a schema-first JSON result envelope. Record additional formats as `TBD` unless a later human-approved scope change chooses them.
4. Define future envelope responsibilities at setup level: result identity, model/run references, unit-aware values, diagnostics, provenance, analysis status, warnings, and reproducibility references.
5. Keep mechanics results, user-rule-check outcomes, missing-data findings, and human review/approval records separate in terminology and future fields.
6. Confirm that no public artifact embeds protected standards text, standards tables, proprietary formulas, private rule-pack payloads, private project data, or automatic compliance/approval claims.
7. Record dependency edges only when source documents state an explicit anchor, prerequisite, interface, or handoff. Do not add structural adjacency or coordination-only edges.
8. For a later implementation brief, create or edit schemas/exporter/tests only inside that later brief's write scope and run the applicable schema, unit, diagnostics, protected-content, and regression gates.

## Verification

| Check | Expected result |
|---|---|
| Four-document setup kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist with stable default sections. |
| Scope boundary | No files outside `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/**` are edited. |
| Format boundary | JSON result envelope baseline is stated; additional export formats remain `TBD`. |
| Unit/provenance/diagnostic boundary | Unit-aware values, provenance, diagnostics, and no-bypass adapter constraints are visible in the specification. |
| Protected/private boundary | No protected standards text, copied tables, proprietary formulas, private rule-pack payloads, private project data, or certification/compliance claims are introduced. |
| Dependency register | `Dependencies.csv` validates against v3.1 schema and `_DEPENDENCIES.md` counts match the CSV. |
| Lifecycle state | `_STATUS.md` is `SEMANTIC_READY` only after the four docs, semantic matrix, lensing register, and dependency extraction setup gates pass. |

## Records

The setup run should leave these records in the deliverable folder:

- Four production documents: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
- Semantic artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`
- Dependency artifacts: `Dependencies.csv`, `_DEPENDENCIES.md`
- Run records: `_run_records/TASK_RUN_2026-04-30_*.md`
- Lifecycle status: `_STATUS.md`
