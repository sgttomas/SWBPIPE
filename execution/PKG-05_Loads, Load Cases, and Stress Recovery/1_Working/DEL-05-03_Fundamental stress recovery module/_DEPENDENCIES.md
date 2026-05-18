# Dependencies: DEL-05-03 Fundamental stress recovery module

## Generated Dependency Register
- **Status:** REFRESHED_DEPENDENCY_EXTRACT_V3_1
- **Source of Truth:** Deliverable-local source documents plus `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 12 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG artifacts remain sequencing and blocker-computation authorities only within their approval boundary.
- This local register is a deliverable-local evidence surface for reconciliation, not an independent project graph authority.
- Architecture-basis rows are preserved as injected context evidence; they do not mark `PKG-00` as `ISSUED`.
- The 2026-05-16 Stage 2 local metadata alignment did not edit status files, schemas, aggregate DAG artifacts, or coordination files. PKG-05 stress source code was edited separately within the allowed finding-resolution scope.

## Extracted Dependency Register

| Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|
| ANCHOR | UPSTREAM | OTHER | SOW-015 Stress recovery scope item | ACTIVE | `_CONTEXT.md` Scope Coverage |
| ANCHOR | UPSTREAM | OTHER | OBJ-003 Robust global centerline/frame solver objective | ACTIVE | `_CONTEXT.md` Objective Support |
| EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 Architecture decision record baseline | ACTIVE | `_CONTEXT.md` Architecture Basis Injection |
| EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 Repository and module boundary architecture | ACTIVE | `_CONTEXT.md` Architecture Basis Injection |
| EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 Application service command-query-job model | ACTIVE | `_CONTEXT.md` Architecture Basis Injection |
| EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | `_CONTEXT.md` Architecture Basis Injection |
| EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | `_CONTEXT.md` Architecture Basis Injection |
| EXECUTION | UPSTREAM | INTERFACE | DEL-04-02 Straight pipe element | ACTIVE | `Datasheet.md` Conditions |
| EXECUTION | UPSTREAM | INTERFACE | DEL-03-08 Pipe section property and mass-property calculator | ACTIVE | `Datasheet.md` Conditions |
| EXECUTION | UPSTREAM | INTERFACE | DEL-05-01 Primitive load case engine | ACTIVE | `Guidance.md` Considerations |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE | `Specification.md` Requirements |
| EXECUTION | UPSTREAM | INTERFACE | DEL-05-04 Analysis status semantics | ACTIVE | `Specification.md` Requirements |

## Run Notes
- **Task:** TP-DAG-004 dependency-extract refresh row for `DEL-05-03`.
- **Mode:** `UPDATE`.
- **Strictness:** `CONSERVATIVE`.
- **ConsumerContext:** `RECONCILIATION`.
- **Scope:** `DEL-05-03`.
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- **Decomposition status:** available; used for anchor and target label validation.
- **Source docs:** `AUTO`; scanned deliverable-local `_CONTEXT.md`, `_REFERENCES.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, existing `Dependencies.csv`, and existing `_DEPENDENCIES.md`.
- **Anchor doc:** `AUTO`; `_CONTEXT.md` and `Datasheet.md` supplied explicit scope/objective anchors.
- **Execution doc order:** `AUTO`; `_CONTEXT.md`, `Specification.md`, `Datasheet.md`, `Guidance.md`, and `Procedure.md`.
- **Merge behavior:** prior `DAG-002-*` row IDs were preserved where the target and statement intent remained matchable.
- **Normalization:** legacy non-v3.1 enum values from the synchronized DAG mirror were mapped to canonical v3.1 values.
- **Warning status:** no `[WARNING] FLOATING_NODE`; one active `IMPLEMENTS_NODE` anchor exists.
- **Warning status:** no `[WARNING] AMBIGUOUS_ANCHOR`; only one active `IMPLEMENTS_NODE` anchor exists.
- **Validation note:** `validate_id_format.sh` is not compatible with the active project ID style (`PKG-05`, `DEL-05-03`, `SOW-015`) and was not used to rewrite canonical IDs.
- **2026-05-16 Stage 2 note:** DAG-002-E0457 was updated to `SATISFIED` based on stress boundary records requiring explicit unit metadata, accepted canonical dimensions, provenance references, and payload/hash refs.

## Run History
- 2026-04-30: Initial dependency extraction run created deliverable-local dependency artifacts.
- 2026-05-03: Local register synchronized from `DAG-002` dependency edges.
- 2026-05-10: TP-DAG-004 refresh in `UPDATE` / `CONSERVATIVE` / `RECONCILIATION` mode; decomposition available; 12 ACTIVE rows; 2 ANCHOR and 10 EXECUTION rows; v3.1 schema and enum validation passed; legacy ID-format helper mismatch noted.
- 2026-05-16: DEV-001 Stage 2 package-local alignment updated DEL-02-02 unit predecessor evidence and satisfaction for stress boundary metadata only; no aggregate DAG or lifecycle action.

## Lifecycle Summary
- **Total rows:** 12
- **ACTIVE rows:** 12
- **RETIRED rows:** 0
- **ANCHOR rows:** 2
- **EXECUTION rows:** 10
- **Closure states:** 8 `SATISFIED`; 4 `PENDING`; 0 `TBD`; 0 `WAIVED`; 0 `NOT_APPLICABLE`.

## Downstream Handoff Notes
- For `RECONCILIATION`, the old DAG mirror dependency meanings are preserved but normalized to the v3.1 enum surface.
- `OBJ-003` is retained as a trace anchor with `TargetType=UNKNOWN` because v3.1 has no `OBJECTIVE` target enum.
- `DEL-05-01` remains active with `Confidence=MEDIUM` because local evidence identifies primitive/load-case outputs as likely upstream contracts but does not define a finalized interface.
- `DEL-04-02`, `DEL-03-08`, and `DEL-05-04` remain the clearest implementation-facing upstream dependencies for stress recovery. `DEL-02-02` unit metadata is technically satisfied for the PKG-05 stress boundary only.
