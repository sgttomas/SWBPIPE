# Dependencies: DEL-04-03 Linear support and restraint models

## Extracted Dependency Register
- **Status:** TP-DAG-004_REFRESHED
- **Local Register:** `Dependencies.csv`
- **Register schema:** v3.1
- **Rows:** 10 total; 10 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated/Refreshed:** 2026-05-10

| Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|
| ANCHOR | UPSTREAM | OTHER | DEL-04-03 Linear support and restraint models | ACTIVE | SATISFIED |
| ANCHOR | UPSTREAM | OTHER | SOW-011 Support and restraint model scope item | ACTIVE | SATISFIED |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | SATISFIED |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | SATISFIED |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | ACTIVE | SATISFIED |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | SATISFIED |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 3D frame stiffness kernel | ACTIVE | TBD |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | ACTIVE | TBD |
| EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE | TBD |

## Run Notes
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **ConsumerContext:** RECONCILIATION
- **Scope:** DEL-04-03 / PKG-04 only.
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Anchor doc:** `Datasheet.md`
- **Execution docs reviewed:** `_CONTEXT.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`
- **Write scope used:** `Dependencies.csv`, `_DEPENDENCIES.md`, `_run_records/TASK_RUN_2026-05-10_2215_dependency-extract-refresh.md`
- Added explicit parent and SOW trace anchors from the deliverable datasheet.
- Preserved prior synchronized DAG-origin execution rows and normalized their enum fields to the current v3.1 validator set.
- Legacy semantic labels are retained in `Notes` for reconciliation: `ARCHITECTURE_BASIS`, `SOLVER_PREDECESSOR`, `DOMAIN_MODEL`, `UNIT_CONTRACT`, `CONTEXT`, `DECOMPOSITION`, `INFERRED_DIRECT`, and `UNKNOWN`.
- No source documents, status files, memory files, decomposition files, DAG files, code, schemas, or tests were edited.

## Run History
- 2026-04-30T10:15:41-0600 - dependency-extract created local v3.1 register and `_DEPENDENCIES.md`; seven ACTIVE rows reported in run record.
- 2026-05-03 - local register synchronized from `DAG-002`; eight ACTIVE execution rows.
- 2026-05-10T22:15:49-0600 - TP-DAG-004 refresh row executed in UPDATE / CONSERVATIVE / RECONCILIATION mode; ten ACTIVE rows; schema validation PASS; enum validation PASS.

## Lifecycle Summary
- **ACTIVE:** 10
- **RETIRED:** 0
- **Closure state:** 7 SATISFIED; 3 TBD.
- **Tree anchor check:** PASS; exactly one ACTIVE `IMPLEMENTS_NODE` parent anchor.
- **Evidence check:** PASS; every ACTIVE row includes `EvidenceFile` and `SourceRef`.

## Downstream Handoff Notes
- This register is a local dependency evidence surface for RECONCILIATION, not an independent sequencing authority.
- Architecture-basis rows remain evidence of applicable SCA-001 dispatch constraints and do not mark `PKG-00` as `ISSUED`.
- The three non-architecture execution prerequisites remain `TBD` satisfaction because this refresh did not read or adjudicate target deliverable maturity outside the assigned folder.
