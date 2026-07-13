# Procedure: DEL-16-04 Agent rationale and professional-boundary controls

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-16-04-DECL-004`.

## Purpose

Define a conservative procedure for reviewing the current bounded DEL-16-04 rationale implementation/test slice and for producing or reviewing later DEL-16-04 artifacts. This procedure is document-level guidance only; it does not implement product code or claim engineering acceptance.

## Prerequisites

| Prerequisite | Source |
|---|---|
| Confirm deliverable identity, package, scope, objectives, and artifact expectations from `_CONTEXT.md`. | `_CONTEXT.md` |
| Use the accepted revision 0.7 decomposition for SOW-070, OBJ-015, OBJ-018, PKG-16, and DEL-16-04. | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Apply project authority invariants for professional boundary and agent proposal status. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4 |
| Apply status vocabulary boundaries and epistemic labels. | `docs/TYPES.md` sections 4 and 5 |
| Apply data-boundary and protected-content constraints. | `docs/IP_AND_DATA_BOUNDARY.md` |
| Preserve the approved DAG-006 local dependency mirror unless RECONCILIATION plus CHANGE approval supersedes it. | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Use the current rationale implementation and focused tests as bounded evidence for this slice. | `core/model_operations/agent_rationale/engine.py`; `tests/test_agent_rationale_boundary.py` |
| Use adjacent PKG-16 schema/audit/preview implementation and tests as read-only context for boundaries consumed by rationale tests. | `schemas/model_operation.schema.json`; `core/model_operations/audit_trail/engine.py`; `core/model_operations/validation_preview/engine.py`; adjacent focused tests |
| Treat standalone rationale schema, final UI/agent workflow presentation, broader persistence/application behavior, dependency versions, and human review dispositions as TBD until later Type 2 work or human ruling resolves them. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` section 12 |

## Steps

1. Confirm the work is still bounded to DEL-16-04 and SOW-070.
2. Review source authority in this order: `_REFERENCES.md` listed governing sources, `_CONTEXT.md`, approved local `Dependencies.csv`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
3. Identify the rationale-record categories supported by sources and implementation: operation history/context, rationale, assumptions, affected entities, audit metadata/references, validation context, source/actor metadata, diagnostics, provenance, and stable rationale ID/hash.
4. Confirm the record posture remains decision-support only: no accepted operation record creation, no accepted-model-state mutation, no user-acceptance bypass, and explicit human-review requirement.
5. Mark unsupported schema fields, standalone schema path, persistence path, UI/agent workflow presentation, broader application behavior, and human dispositions as `TBD` rather than inventing them.
6. Identify prohibited professional-boundary outputs from `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `PROHIBITED_CLAIM_PATTERNS` in `core/model_operations/agent_rationale/engine.py`.
7. Check that rationale text and copied operation/audit/validation/source/actor/reference context produce blocking diagnostics for prohibited authority language.
8. Check that lowercase coordination references such as approved DAG authority are not misclassified as professional approval unless they use blocked authority language.
9. Check that assumptions, warnings, limitations, missing context, and `TBD` values remain explicit findings.
10. Check that no protected standards text, code-specific values, private project data, or private rule-pack payloads are introduced.
11. Check dependency handling against the local DAG-006 mirror; do not retire, delete, or reclassify approved ACTIVE rows during this setup pass.
12. Record unresolved issues as `TBD`, `ASSUMPTION`, or `PROPOSAL` using `docs/TYPES.md` epistemic labels.

## Verification

| Check | Expected Result |
|---|---|
| Scope check | Documents and rationale artifacts reference DEL-16-04, PKG-16, SOW-070, OBJ-015, and OBJ-018 without expanding into sibling deliverables. |
| Rationale preservation check | Operation history, rationale, assumptions, affected entities, and audit metadata are present as required categories or explicitly marked TBD pending schema work. |
| Non-acceptance check | Current rationale output is decision-support only and does not create accepted operation records, bypass user acceptance, or mutate accepted model state. |
| Prohibited-claim check | Automatic output cannot state or imply certification, sealing, approval, authentication, professional reliance, external validation authority, autonomous engineering acceptance, or code compliance. |
| Status-vocabulary check | Automatic statuses remain within the permitted vocabulary from `docs/TYPES.md` and `docs/SPEC.md`. |
| Human-acceptance check | Any human acceptance reference is external, human-actor-owned, and hash-bound; it is not generated by software. |
| Copied-context check | Prohibited authority language in copied operation, audit, or validation context is blocked before rationale is treated as captured for user review. |
| Visible-TBD check | Missing audit context, validation context, source metadata, actor metadata, rationale text, and timestamp remain visible `TBD_VISIBLE` diagnostics. |
| Data-boundary check | No protected standards text, proprietary data, private project data, or private rule-pack payload is introduced. |
| Dependency preservation check | `Dependencies.csv` approved DAG-006 rows remain ACTIVE and unchanged by this setup pass. |

## Records

The expected records for this deliverable are:

- agent rationale record: current implementation path `core/model_operations/agent_rationale/engine.py`; standalone schema and persistence path TBD;
- professional-boundary guard tests: current focused path `tests/test_agent_rationale_boundary.py`; broader UI/API/report coverage TBD;
- unresolved assumptions and `TBD` decisions surfaced in deliverable documents;
- validation evidence from focused implementation tests;
- this setup document kit and semantic/lensing metadata.
