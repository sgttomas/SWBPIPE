# Procedure: DEL-05-04 Analysis status semantics

## Purpose

Define the procedure for maintaining DEL-05-04 analysis status semantics evidence after bounded implementation. Product code remains outside this deliverable folder, but the local kit may cite implemented schemas, code, tests, and downstream consumers as evidence.

## Prerequisites

| Prerequisite | Source |
|---|---|
| Sealed DEL-05-04 context with write scope limited to this deliverable folder. | `_CONTEXT.md`; OPS-K-AGENT-3 |
| Decomposition revision 0.7 and register rows for DEL-05-04, SOW-047, OBJ-005, and OBJ-011. | `execution/_Decomposition/SOFTWARE_DECOMP.md`; registers |
| Applicable architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08. | `_CONTEXT.md` Architecture Basis Injection |
| Invariant catalog slices for authority, mechanics/rule separation, missing data, reports, and agent boundaries. | `docs/CONTRACT.md` |
| Analysis status vocabulary, architecture note, schemas, and tests. | `docs/TYPES.md`; `docs/architecture/analysis_status_semantics.md`; `schemas/analysis_status.schema.yaml`; `tests/test_analysis_status_schema.py` |

## Steps

| Step | Action | Evidence/record |
|---|---|---|
| 1 | Read the sealed context and confirm the deliverable is DEL-05-04 / PKG-05 / SOW-047. | `_CONTEXT.md`; run record |
| 2 | Read governing vocabulary and invariants for analysis statuses, authority boundaries, missing data, report provenance, and human acceptance records. | `docs/TYPES.md`; `docs/CONTRACT.md`; architecture note |
| 3 | Maintain the four-document kit as deliverable-local evidence for the analysis-status state model. | `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` |
| 4 | Build the semantic matrix lens as question-shaping evidence, not engineering authority. | `_SEMANTIC.md` |
| 5 | Build the semantic lensing register and capture warranted gaps/questions with `HumanRuling=TBD`. | `_SEMANTIC_LENSING.md` |
| 6 | Apply only source-supported Pass 3 refinements, preserving unresolved items as `TBD`. | Four documents and run record |
| 7 | Extract dependency anchors and information-flow constraints conservatively. | `Dependencies.csv`; `_DEPENDENCIES.md` |
| 8 | Validate dependency/schema evidence as needed and keep `_STATUS.md` unchanged unless a human explicitly authorizes a lifecycle gate. | Validation output; `_STATUS.md` |

## Implementation Evidence Fixture Selection

When maintaining or extending implementation evidence, select tests that demonstrate:

- `MECHANICS_SOLVED` can coexist with `RULE_INPUTS_INCOMPLETE`;
- missing physical solve inputs emit `MODEL_INCOMPLETE`;
- rule-pack evaluation can emit checked or failed outcomes without compliance claims;
- `HUMAN_REVIEW_REQUIRED` appears for reportable results;
- `HUMAN_APPROVED_FOR_PROJECT` cannot be emitted by ordinary software execution and cannot survive bound-content hash changes.

## Verification

| Check | Expected result |
|---|---|
| Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
| Local write boundary | No schema, API, Rust, TypeScript, GUI, or test implementation files are created or edited inside this deliverable-local maintenance tranche. |
| Status distinction preserved | Mechanics, rule-pack, incomplete-data, and human-acceptance statuses remain distinct. |
| Human approval boundary preserved | Automatic software approval/compliance claims are absent; human acceptance remains external and `TBD` where unresolved. |
| Dependency register valid | `Dependencies.csv` conforms to v3.1 required columns and `_DEPENDENCIES.md` counts match. |
| Lifecycle safe | `_STATUS.md` remains under human lifecycle authority and is not changed by evidence alignment. |

## Records

- `_run_records/TASK_RUN_2026-04-30_1530_four-documents-p1-p2.md`
- `_run_records/TASK_RUN_2026-04-30_1530_semantic-matrix-build.md`
- `_run_records/TASK_RUN_2026-04-30_1530_lens-register.md`
- `_run_records/TASK_RUN_2026-04-30_1530_four-documents-p3.md`
- `_run_records/TASK_RUN_2026-04-30_1530_dependency-extract.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_REVIEW.md`
- `Review_Findings.csv`

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Analysis-status semantics are implemented as a code-neutral status vocabulary with current verification evidence. External acceptance remains human-owned, and stale-hash negative behavior remains held where explicitly recorded.
