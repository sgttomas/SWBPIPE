# Dependencies: DEL-13-01 Design knowledge schema and provenance model

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Canonicalized:** 2026-06-16
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=12.
- **Candidate rows moved to worklist:** 0.

### Compact Active Register

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DAG-002-E0639 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0640 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0641 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0642 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0643 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0644 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0645 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0758 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-001 |
| DAG-002-E0759 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-02 Unit system and dimensional-analysis core contract | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-001 |
| DEV-001-PKG13-DEL1301-DEL0205 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-05 Project persistence and round-trip serialization | `_DEPENDENCIES.md` DEV-001 Stage 2 finding resolution |
| DAG-002-E0760 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-02 Copyright and protected-data boundary policy | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-001 |
| DAG-002-E0761 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-04 Professional responsibility and product-claims policy | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-001 |
| DEL-13-01-A001 | ANCHOR | UPSTREAM | OTHER | SOW-067 Support user-supplied design knowledge | `_CONTEXT.md` Scope Coverage / Scope Detail |
| DEL-13-01-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-014 Schema-backed physical design model objective | `_CONTEXT.md` Objective Support |

## Canonical Dependency Types
- `INTERFACE`: 2
- `OTHER`: 2
- `PREREQUISITE`: 10

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`; anchor validation resolved SOW-067 and OBJ-014.
- PKG-00 rows reviewed: 7; PKG-00 rows changed in `Dependencies.csv`: 7; retained as architecture-consistency trackers.
- Warnings: none.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Rows |
|---|---|---|---|---|---:|
| 2026-06-16 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | None | 14 |

## Lifecycle Summary

| Lifecycle Field | Breakdown |
|---|---|
| Status | ACTIVE: 14; RETIRED: 0 |
| DependencyClass | ANCHOR: 2; EXECUTION: 12 |
| Direction | UPSTREAM: 14 |
| SatisfactionStatus | SATISFIED: 7; TBD: 5; PENDING: 2 |
| Origin | EXTRACTED: 14 |

## Downstream Handoff Notes

- For RECONCILIATION, treat this register as a local evidence surface only; graph authority remains with the approved DAG and any later human-approved successor.
- No candidate rows, retired rows, or PKG-00 mutations were introduced.
