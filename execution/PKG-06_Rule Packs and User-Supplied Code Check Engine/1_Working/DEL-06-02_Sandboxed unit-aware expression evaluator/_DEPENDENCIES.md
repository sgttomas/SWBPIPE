# Dependencies: DEL-06-02 Sandboxed unit-aware expression evaluator

## Extracted Dependency Register
- **Status:** REFRESHED_FROM_DELIVERABLE_SOURCES
- **Schema:** `Dependencies.csv` v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Rows:** 16 total; 15 ACTIVE; 1 RETIRED.
- **Generated:** 2026-05-10

| Class | Active | Retired | Notes |
|---|---:|---:|---|
| ANCHOR | 6 | 0 | Parent SOW anchor plus selected explicit requirement trace anchors from `Specification.md`. |
| EXECUTION | 9 | 1 | Architecture-basis constraints and local interface dependencies retained where assigned docs provide evidence. |

## Active Execution Edges

| DependencyID | Direction | Type | Target | Satisfaction | Evidence |
|---|---|---|---|---|---|
| DAG-002-E0162 | UPSTREAM | CONSTRAINT | DEL-00-01 / AB-00-01 | SATISFIED | `_CONTEXT.md#Architecture Basis Injection` |
| DAG-002-E0163 | UPSTREAM | CONSTRAINT | DEL-00-02 / AB-00-02 | SATISFIED | `_CONTEXT.md#Architecture Basis Injection` |
| DAG-002-E0164 | UPSTREAM | CONSTRAINT | DEL-00-03 / AB-00-03 | SATISFIED | `_CONTEXT.md#Architecture Basis Injection` |
| DAG-002-E0165 | UPSTREAM | CONSTRAINT | DEL-00-04 / AB-00-04 | SATISFIED | `_CONTEXT.md#Architecture Basis Injection` |
| DAG-002-E0166 | UPSTREAM | CONSTRAINT | DEL-00-06 / AB-00-06 | SATISFIED | `_CONTEXT.md#Architecture Basis Injection` |
| DAG-002-E0167 | UPSTREAM | CONSTRAINT | DEL-00-07 / AB-00-07 | SATISFIED | `_CONTEXT.md#Architecture Basis Injection` |
| DAG-002-E0168 | UPSTREAM | CONSTRAINT | DEL-00-08 / AB-00-08 | SATISFIED | `_CONTEXT.md#Architecture Basis Injection` |
| DAG-002-E0467 | UPSTREAM | INTERFACE | DEL-06-01 Rule-pack schema | TBD | `Procedure.md#Prerequisites` |
| DAG-002-E0468 | UPSTREAM | INTERFACE | DEL-02-02 Unit system and dimensional-analysis core contract | TBD | `Procedure.md#Prerequisites` |

## Retired / Non-Gating Edges

| DependencyID | Former Target | Refresh Disposition |
|---|---|---|
| DAG-002-E0623 | DEL-12-05 Security threat model | RETIRED as a gating local edge. Assigned docs mention a future evaluator threat model, but do not explicitly bind it to DEL-12-05 under CONSERVATIVE extraction. |

## Run Notes

- `SOURCE_DOCS`: AUTO within the assigned deliverable folder.
- `ANCHOR_DOC`: AUTO, using `_CONTEXT.md`, `Datasheet.md`, and `Specification.md` for explicit identity, scope, and requirement signals.
- `EXECUTION_DOC_ORDER`: AUTO, using `_CONTEXT.md` for architecture-basis constraints and `Procedure.md` / `Specification.md` for implementation prerequisites.
- `DECOMPOSITION_PATH`: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- `MODE=UPDATE`: Existing DAG edge IDs were preserved where the assigned docs still support the dependency intent.
- `STRICTNESS=CONSERVATIVE`: No new cross-deliverable edge was promoted without an explicit deliverable ID or architecture-basis ID in the assigned sources.
- v3.1 enum normalization was applied: architecture basis rows use `DependencyType=CONSTRAINT`, interface rows use `DependencyType=INTERFACE`, and all origins use `EXTRACTED`.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 15 |
| RETIRED | 1 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 7 |
| TBD | 3 |
| NOT_APPLICABLE | 6 |

## Downstream Handoff Notes

- For RECONCILIATION, this register is a deliverable-local extraction surface, not an independent aggregate graph authority.
- The seven `PKG-00` architecture-basis rows remain satisfied context constraints and should not be interpreted as making `PKG-00` locally issued.
- The `DEL-06-01` and `DEL-02-02` interface rows remain open coordination dependencies for implementation planning.
- The former `DEL-12-05` security-threat-model row is retained as RETIRED/non-gating local evidence; reconciliation may reintroduce or promote it only through an approved graph/change workflow.

## Run History

- 2026-04-30T10:32:35-0600: Initial dependency-extract produced a local register from setup documents.
- 2026-05-03: Local register synchronized from DAG-002 aggregate edge set.
- 2026-05-10T22:35:57-0600: TP-DAG-004 dependency-extract refresh for `DEL-06-02`; mode UPDATE; strictness CONSERVATIVE; consumer context RECONCILIATION; schema v3.1 and enum values normalized; 15 ACTIVE and 1 RETIRED rows.
