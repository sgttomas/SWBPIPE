# Dependencies: DEL-11-02 Developer Guide For Solver And Rule Packs

## Generated Dependency Register

- **Status:** DAG-006_LOCAL_EVIDENCE_REFRESH
- **Graph Authority:** `execution/_DAG/DAG-006/`
- **Graph Approval Record:** `execution/_DAG/DAG-006/APPROVAL_RECORD.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Refreshed:** 2026-06-07

## Authority Boundary

- `DAG-006` is the approved graph coordination authority for active edge rows.
- This deliverable-local register is derivative evidence for downstream reconciliation and task planning; it is not an independent graph approval, lifecycle transition, release-readiness claim, or Type 2 dispatch authorization.
- `DAG-006` approval removed lifecycle metadata from DAG node artifacts. Upstream maturity checks here therefore cite deliverable-local `_STATUS.md` and `MEMORY.md` evidence.
- `CANDIDATE` rows remain non-gating unless later promoted by explicit human/CHANGE approval. No candidate rows were added for `DEL-11-02`.
- Architecture-basis rows remain context/coordination evidence only; they do not mark `PKG-00` deliverables as `ISSUED`.

## Extracted Dependency Register

| Count | Meaning |
|---:|---|
| 11 | Current `DAG-006` ACTIVE execution rows touching `DEL-11-02` |
| 4 | Active local anchor rows from explicit `DEL-11-02` identifiers |
| 0 | Retired rows |
| 0 | Candidate rows |

| DependencyID | Class | Anchor/Type | Direction | Target | Required | Current Evidence | Satisfaction |
|---|---|---|---|---|---|---|---|
| DAG-004-R0605 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-01 Architecture decision record baseline | SEMANTIC_READY | `_STATUS.md` records `CHECKING` after prior `SEMANTIC_READY` | SATISFIED |
| DAG-004-R0606 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-02 Repository and module boundary architecture | SEMANTIC_READY | `_STATUS.md` records `CHECKING` after prior `SEMANTIC_READY` | SATISFIED |
| DAG-004-R0607 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-06 Diagnostics, warning, and result-envelope contract | SEMANTIC_READY | `_STATUS.md` records `CHECKING` after prior `SEMANTIC_READY` | SATISFIED |
| DAG-004-R0608 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-07 API boundary and adapter contract map | SEMANTIC_READY | `_STATUS.md` records `CHECKING` after prior `SEMANTIC_READY` | SATISFIED |
| DAG-004-R0609 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-08 Layered software test and acceptance strategy | SEMANTIC_READY | `_STATUS.md` records `CHECKING` after prior `SEMANTIC_READY` | SATISFIED |
| DAG-004-R0610 | EXECUTION | GOVERNANCE_PREDECESSOR | UPSTREAM | DEL-01-02 Copyright and protected-data boundary policy | SEMANTIC_READY | `_STATUS.md` records `CHECKING`; `MEMORY.md` records current protected-data boundary evidence and deferred governance/legal TBDs | SATISFIED |
| DAG-004-R0611 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-02-01 Canonical domain model schema | SEMANTIC_READY | `_STATUS.md` records `CHECKING`; `MEMORY.md` records current schema/test evidence and downstream ownership TBDs | SATISFIED |
| DAG-004-R0612 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-02-02 Unit system and dimensional-analysis core contract | SEMANTIC_READY | `_STATUS.md` records `CHECKING`; `MEMORY.md` records unit-schema/test evidence and unit-system TBDs | SATISFIED |
| DAG-004-R0613 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-04-01 3D frame stiffness kernel | SEMANTIC_READY | `_STATUS.md` records `CHECKING`; `MEMORY.md` records frame-kernel validation and solver TBDs | SATISFIED |
| DAG-004-R0614 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-06-01 Rule-pack schema | SEMANTIC_READY | `_STATUS.md` records `CHECKING`; `MEMORY.md` records rule-pack schema/test evidence and rule-engine TBDs | SATISFIED |
| DAG-004-R0615 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-10-01 Public API and plugin boundary | SEMANTIC_READY | `_STATUS.md` records `CHECKING`; `MEMORY.md` records API/plugin boundary alignment and transport/plugin/export TBDs | SATISFIED |
| DEP-011-02-001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | PKG-11 Documentation, Examples, and Education | SEMANTIC_READY | Datasheet/context identity validated against `SOFTWARE_DECOMP.md` revision 0.7 | SATISFIED |
| DEP-011-02-002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | SOW-033 | SEMANTIC_READY | Datasheet/context scope trace validated against `SOFTWARE_DECOMP.md` revision 0.7 | SATISFIED |
| DEP-011-02-003 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-001 | SEMANTIC_READY | Datasheet/context objective trace validated against `SOFTWARE_DECOMP.md` revision 0.7 | SATISFIED |
| DEP-011-02-004 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-002 | SEMANTIC_READY | Datasheet/context objective trace validated against `SOFTWARE_DECOMP.md` revision 0.7 | SATISFIED |

## Satisfaction Refresh Notes

- Replaced stale local `DAG-002-*` mirror row IDs with current `DAG-006` active edge rows `DAG-004-R0605` through `DAG-004-R0615`.
- Updated the six task-targeted predecessor rows for governance, schema, units, solver, rule-pack, and API/plugin evidence from `UNKNOWN` to `SATISFIED` only after checking upstream local `_STATUS.md` and `MEMORY.md` evidence.
- Set the 11 current DAG execution rows to `ProposedMaturity=CHECKING` because the cited upstream local statuses are currently `CHECKING`, which meets or exceeds the `SEMANTIC_READY` required maturity for this dependency evidence.
- Preserved the four local anchor rows and refreshed their `LastSeen` dates against `SOFTWARE_DECOMP.md` revision 0.7.

## Residual TBDs And Warnings

- Governance: contributor/legal mechanism, reviewer/legal-review authority, maintainer roster/quorum, release authority, and related human-governance details remain `TBD` upstream.
- Units: conversion constants, unit catalog, tolerance policy, canonical calculation basis, and special-quantity semantics remain `TBD` upstream.
- Solver: sparse solver library, production tolerance policy, release thresholds, and final result-envelope integration remain `TBD` upstream.
- Rule packs: expression grammar/library, evaluator semantics, private lifecycle, checksum/storage details, and invented public examples remain downstream or `TBD`.
- API/plugin: public transport, endpoint syntax, plugin loading/signing/isolation, permission persistence, concrete writer behavior, and target field coverage remain `TBD`.
- These residual TBDs do not block the local dependency rows at `RequiredMaturity=SEMANTIC_READY`, but they remain relevant for later implementation, review, validation, and release gates.

## Run History

- 2026-04-30 1215: Initial dependency extraction run; local register later synchronized from early DAG evidence.
- 2026-05-11: TP-DAG-004 dependency surface refresh for reconciliation; 15 ACTIVE rows total.
- 2026-06-04: Authority refresh aligned local surfaces with `SOFTWARE_DECOMP.md` revision 0.7 and approved `DAG-006`.
- 2026-06-07: TASK evidence refresh replaced stale DAG mirror IDs with current `DAG-006` rows and updated predecessor satisfaction from current upstream local status/memory evidence.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 15 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 15 |
| TBD | 0 |
| UNKNOWN | 0 |

| DependencyClass | Count |
|---|---:|
| EXECUTION | 11 |
| ANCHOR | 4 |

## Downstream Handoff Notes

- Downstream reconciliation should treat this package as derivative local evidence based on accepted `DAG-006` rows plus cited upstream local status/memory evidence.
- The four `DEP-011-02-*` rows are local traceability anchors, not DAG sequencing edges.
- No lifecycle state, review disposition, release-readiness status, legal clearance, professional approval, certification, sealing, authentication, or code-compliance claim is made by this refresh.
