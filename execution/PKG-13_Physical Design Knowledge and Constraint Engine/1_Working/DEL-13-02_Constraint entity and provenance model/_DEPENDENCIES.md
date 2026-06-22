# Dependencies: DEL-13-02 Constraint entity and provenance model

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Canonicalized:** 2026-06-16
- **Rows:** 18 total; 18 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=5; EXECUTION=13.
- **Candidate rows moved to worklist:** 0.

### Compact Active Register

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DAG-002-E0646 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0647 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0648 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0649 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0650 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0651 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0652 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0762 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-01 Design knowledge schema and provenance model | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-002 |
| DAG-002-E0763 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-002 |
| DAG-002-E0764 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-02 Unit system and dimensional-analysis core contract | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-002 |
| DAG-002-E0765 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-05 Project persistence and round-trip serialization | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-002 |
| DAG-002-E0766 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-04 Professional responsibility and product-claims policy | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-002 |
| DEP-013-02-001 | ANCHOR | UPSTREAM | OTHER | PKG-13 Physical Design Knowledge and Constraint Engine | `Datasheet.md` Identification |
| DEP-013-02-002 | ANCHOR | UPSTREAM | OTHER | SOW-068 Constraint validation scope | `Datasheet.md` Scope coverage |
| DEP-013-02-003 | ANCHOR | UPSTREAM | OTHER | SOW-067 User-supplied design knowledge | `Datasheet.md` Scope coverage |
| DEP-013-02-004 | ANCHOR | UPSTREAM | OTHER | OBJ-014 Schema-backed design model objective | `Datasheet.md` Objective support |
| DEP-013-02-005 | ANCHOR | UPSTREAM | OTHER | OBJ-018 Professional and IP boundary objective | `Datasheet.md` Objective support |
| DEP-013-02-006 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-13-03 Constraint validation engine | `Guidance.md` Principles / Separate schema from validation engine |

## Canonical Dependency Types
- `HANDOVER`: 1
- `INTERFACE`: 2
- `OTHER`: 5
- `PREREQUISITE`: 10

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`; anchor validation resolved PKG-13, SOW-067, SOW-068, OBJ-014, and OBJ-018.
- PKG-00 rows reviewed: 7; PKG-00 rows changed in `Dependencies.csv`: 7; retained as architecture-consistency trackers.
- Warnings: none.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Rows |
|---|---|---|---|---|---:|
| 2026-06-16 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | None | 18 |

## Lifecycle Summary

| Lifecycle Field | Breakdown |
|---|---|
| Status | ACTIVE: 18; RETIRED: 0 |
| DependencyClass | ANCHOR: 5; EXECUTION: 13 |
| Direction | UPSTREAM: 17; DOWNSTREAM: 1 |
| SatisfactionStatus | SATISFIED: 7; TBD: 6; NOT_APPLICABLE: 5 |
| Origin | EXTRACTED: 18 |

## Downstream Handoff Notes

- For RECONCILIATION, treat the downstream handover to DEL-13-03 as context for validation-engine consumption, not as lifecycle closure.
- No candidate rows, retired rows, or PKG-00 mutations were introduced.
