# Procedure: DEL-05-04 Analysis status semantics

## Purpose

Define the setup procedure for producing and later using analysis status semantics without implementing product code in this deliverable folder.

## Prerequisites

| Prerequisite | Source |
|---|---|
| Sealed DEL-05-04 context with write scope limited to this deliverable folder. | `_CONTEXT.md`; OPS-K-AGENT-3 |
| Decomposition revision 0.4 and register rows for DEL-05-04, SOW-047, OBJ-005, and OBJ-011. | `docs/_Decomposition/SOFTWARE_DECOMP.md`; registers |
| Applicable architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08. | `_CONTEXT.md` Architecture Basis Injection |
| Invariant catalog slices for authority, mechanics/rule separation, missing data, reports, and agent boundaries. | `docs/CONTRACT.md` |
| Analysis status vocabulary and architecture note. | `docs/TYPES.md`; `docs/architecture/analysis_status_semantics.md` |

## Steps

| Step | Action | Evidence/record |
|---|---|---|
| 1 | Read the sealed context and confirm the deliverable is DEL-05-04 / PKG-05 / SOW-047. | `_CONTEXT.md`; run record |
| 2 | Read governing vocabulary and invariants for analysis statuses, authority boundaries, missing data, report provenance, and human acceptance records. | `docs/TYPES.md`; `docs/CONTRACT.md`; architecture note |
| 3 | Draft setup-only four-document evidence for the analysis-status state model. | `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` |
| 4 | Build the semantic matrix lens as question-shaping evidence, not engineering authority. | `_SEMANTIC.md` |
| 5 | Build the semantic lensing register and capture warranted gaps/questions with `HumanRuling=TBD`. | `_SEMANTIC_LENSING.md` |
| 6 | Apply only source-supported Pass 3 refinements, preserving unresolved items as `TBD`. | Four documents and run record |
| 7 | Extract dependency anchors and information-flow constraints conservatively. | `Dependencies.csv`; `_DEPENDENCIES.md` |
| 8 | Validate dependency schema and keep `_STATUS.md` in a safe non-ISSUED state. | Validation output; `_STATUS.md` |

## Future Implementation Fixture Selection

When a later sealed implementation brief resolves schema/API placement, select tests that demonstrate:

- `MECHANICS_SOLVED` can coexist with `RULE_INPUTS_INCOMPLETE`;
- missing physical solve inputs emit `MODEL_INCOMPLETE`;
- rule-pack evaluation can emit checked or failed outcomes without compliance claims;
- `HUMAN_REVIEW_REQUIRED` appears for reportable results;
- `HUMAN_APPROVED_FOR_PROJECT` cannot be emitted by ordinary software execution and cannot survive bound-content hash changes.

## Verification

| Check | Expected result |
|---|---|
| Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
| No product implementation | No schema, API, Rust, TypeScript, GUI, or test implementation files are created in this folder. |
| Status distinction preserved | Mechanics, rule-pack, incomplete-data, and human-acceptance statuses remain distinct. |
| Human approval boundary preserved | Automatic software approval/compliance claims are absent; human acceptance remains external and `TBD` where unresolved. |
| Dependency register valid | `Dependencies.csv` conforms to v3.1 required columns and `_DEPENDENCIES.md` counts match. |
| Lifecycle safe | `_STATUS.md` is `INITIALIZED` or `SEMANTIC_READY`, not `ISSUED`. |

## Records

- `_run_records/TASK_RUN_2026-04-30_1530_four-documents-p1-p2.md`
- `_run_records/TASK_RUN_2026-04-30_1530_semantic-matrix-build.md`
- `_run_records/TASK_RUN_2026-04-30_1530_lens-register.md`
- `_run_records/TASK_RUN_2026-04-30_1530_four-documents-p3.md`
- `_run_records/TASK_RUN_2026-04-30_1530_dependency-extract.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
