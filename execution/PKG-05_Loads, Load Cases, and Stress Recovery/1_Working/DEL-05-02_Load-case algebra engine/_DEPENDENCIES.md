# Dependencies: DEL-05-02 Load-case algebra engine

## Extracted Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth:** deliverable-local evidence plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 19 total; 19 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

| Class | Anchor/Type | Direction | Target | Status | Satisfaction | Confidence |
|---|---|---:|---|---|---|---|
| ANCHOR | IMPLEMENTS_NODE | UPSTREAM | SOW-014 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-001 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-002 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-003 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-004 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-005 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-006 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-007 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-008 | ACTIVE | NOT_APPLICABLE | HIGH |
| ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQ-05-02-009 | ACTIVE | NOT_APPLICABLE | HIGH |
| EXECUTION | CONSTRAINT | UPSTREAM | DEL-00-01 | ACTIVE | SATISFIED | HIGH |
| EXECUTION | CONSTRAINT | UPSTREAM | DEL-00-02 | ACTIVE | SATISFIED | HIGH |
| EXECUTION | CONSTRAINT | UPSTREAM | DEL-00-03 | ACTIVE | SATISFIED | HIGH |
| EXECUTION | CONSTRAINT | UPSTREAM | DEL-00-06 | ACTIVE | SATISFIED | HIGH |
| EXECUTION | CONSTRAINT | UPSTREAM | DEL-00-08 | ACTIVE | SATISFIED | HIGH |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-05-01 | ACTIVE | PENDING | HIGH |
| EXECUTION | PREREQUISITE | UPSTREAM | DEL-02-02 | ACTIVE | SATISFIED | HIGH |
| EXECUTION | INTERFACE | UPSTREAM | DEL-05-04 | ACTIVE | PENDING | HIGH |
| EXECUTION | INTERFACE | UPSTREAM | DEL-06-02 | ACTIVE | TBD | LOW |

## Run Notes

- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor document: `Datasheet.md`; trace requirements extracted from `Specification.md`.
- Execution documents used: `_CONTEXT.md`, `Datasheet.md`, `Guidance.md`, and `Specification.md`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Register refresh normalized graph-local dependency enums to v3.1 canonical values: architecture-basis rows became `DependencyType=CONSTRAINT`; load/unit predecessor rows became `PREREQUISITE`; status-semantics and possible evaluator reuse rows became `INTERFACE`.
- `DAG-002-E0616` is retained as a non-gating `PROPOSAL` for RECONCILIATION because the expression grammar/library remains `TBD`.
- 2026-05-16 DEV-001 Stage 2 local metadata alignment updated the DEL-02-02 unit predecessor to `SATISFIED` based on algebra boundary records requiring explicit unit metadata, accepted canonical dimensions, provenance references, and payload/hash refs.
- No `[WARNING] FLOATING_NODE`: exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: only one ACTIVE parent anchor is present.
- No `[WARNING] MISSING_DECOMPOSITION`: the requested decomposition file was available.

## Lifecycle Summary

| Metric | Count |
|---|---:|
| ACTIVE rows | 19 |
| RETIRED rows | 0 |
| ANCHOR rows | 10 |
| EXECUTION rows | 9 |
| SATISFIED | 6 |
| PENDING | 2 |
| TBD | 1 |
| NOT_APPLICABLE | 10 |

## Downstream Handoff Notes

- Consumer context is `RECONCILIATION`; the register intentionally preserves the low-confidence DEL-06-02 evaluator interface as candidate evidence rather than promoting it to a gating predecessor.
- Two pending execution dependencies (`DEL-05-01`, `DEL-05-04`) require cross-deliverable maturity review outside this bounded TASK worker. `DEL-02-02` unit metadata is technically satisfied for the PKG-05 algebra boundary only.
- Architecture-basis constraints are marked `SATISFIED` because `_CONTEXT.md` explicitly injects AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08 as dispatchable context.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | Active Rows | Notes |
|---|---|---|---|---|---:|---|
| 2026-04-30T10:23:35-0600 | UPDATE | CONSERVATIVE | execution/_Decomposition/SOFTWARE_DECOMP.md | none | 7 | Initial dependency-extract run created v3.1 register. |
| 2026-05-03 | SYNC | N/A | execution/_DAG/DAG-006/DependencyEdges.csv | none | 8 + 1 candidate | Register synchronized from DAG-002 graph rows. |
| 2026-05-10T22:27:35-0600 | UPDATE | CONSERVATIVE | execution/_Decomposition/SOFTWARE_DECOMP.md | none | 19 | TP-DAG-004 refresh normalized v3.1 enums, added explicit SOW/REQ anchors, and prepared RECONCILIATION handoff. |
| 2026-05-16 | STAGE2_LOCAL_ALIGNMENT | CONSERVATIVE | accepted PKG-02 contract | none | 19 | Updated DEL-02-02 unit predecessor evidence and satisfaction for algebra boundary metadata only; no aggregate DAG or lifecycle action. |

## Authority Boundary

- This local register is an evidence surface for the assigned deliverable, not an independent project graph authority.
- `CANDIDATE` is not a valid v3.1 `Status` enum; candidate semantics are preserved in `Notes` and `SatisfactionStatus=TBD`.
- `PKG-00` architecture-basis rows remain injected context evidence and do not mark `PKG-00` deliverables as `ISSUED`.
