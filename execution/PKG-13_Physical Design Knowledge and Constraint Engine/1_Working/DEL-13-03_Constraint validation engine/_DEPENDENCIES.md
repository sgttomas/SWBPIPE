# Dependencies: DEL-13-03 Constraint validation engine

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
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=12.
- **Candidate rows moved to worklist:** 0.

### Compact Active Register

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DEP-13-03-A001 | ANCHOR | UPSTREAM | OTHER | SOW-068 Constraint system validation | `_CONTEXT.md` Scope Coverage / Scope Detail |
| DEP-13-03-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-014 Schema-backed piping design model objective | `_CONTEXT.md` Objective Support |
| DAG-002-E0653 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0654 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0655 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0656 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0657 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0658 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0659 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | `_CONTEXT.md` Architecture Basis Injection |
| DAG-002-E0767 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-01 Design knowledge schema and provenance model | `Guidance.md` Considerations |
| DAG-002-E0768 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-02 Constraint entity and provenance model | `Guidance.md` Considerations |
| DAG-002-E0769 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 Unit system and dimensional-analysis core contract | `Guidance.md` Considerations |
| DAG-002-E0770 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-06 Solver diagnostics and singularity detection | `Guidance.md` Considerations |
| DAG-002-E0771 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-05 Project persistence and round-trip serialization | `Guidance.md` Considerations |

## Canonical Dependency Types
- `OTHER`: 2
- `PREREQUISITE`: 12

## Run Notes
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`, `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`; anchor validation resolved SOW-068 and OBJ-014.
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
| SatisfactionStatus | NOT_APPLICABLE: 2; SATISFIED: 7; TBD: 5 |
| Origin | EXTRACTED: 14 |

## Downstream Handoff Notes

- For RECONCILIATION, treat this register as a local evidence surface only; graph authority remains with the approved DAG and any later human-approved successor.
- No candidate rows, retired rows, or PKG-00 mutations were introduced.
