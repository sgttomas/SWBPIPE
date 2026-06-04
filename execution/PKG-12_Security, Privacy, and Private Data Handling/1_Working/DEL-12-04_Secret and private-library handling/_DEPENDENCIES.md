# Dependencies: DEL-12-04 Secret and private-library handling

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004_LOCAL
- **Graph authority:** `execution/_DAG/DAG-006/` remains the approved graph authority; historical `DAG-003` material was not used or promoted.
- **Local Register:** `Dependencies.csv`
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed deliverable-local evidence surface for later RECONCILIATION, not an independent graph authority.
- Rows added by this refresh do not approve aggregate graph changes.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| Count | Value |
|---:|---|
| Total rows | 17 |
| ACTIVE rows | 17 |
| RETIRED rows | 0 |
| ANCHOR rows | 4 |
| EXECUTION rows | 13 |

| DependencyID | Class | Type | Direction | Target | Status | Origin |
|---|---|---|---|---|---|---|
| DEL-12-04-A001 | ANCHOR | OTHER | UPSTREAM | PKG-12 | ACTIVE | EXTRACTED |
| DEL-12-04-A002 | ANCHOR | OTHER | UPSTREAM | SOW-040 | ACTIVE | EXTRACTED |
| DEL-12-04-A003 | ANCHOR | OTHER | UPSTREAM | SOW-029 | ACTIVE | EXTRACTED |
| DEL-12-04-A004 | ANCHOR | OTHER | UPSTREAM | OBJ-010 | ACTIVE | EXTRACTED |
| DAG-002-E0375 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-01 | ACTIVE | DECLARED |
| DAG-002-E0376 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-02 | ACTIVE | DECLARED |
| DAG-002-E0377 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-03 | ACTIVE | DECLARED |
| DAG-002-E0378 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-04 | ACTIVE | DECLARED |
| DAG-002-E0379 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-06 | ACTIVE | DECLARED |
| DAG-002-E0380 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-07 | ACTIVE | DECLARED |
| DAG-002-E0381 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-00-08 | ACTIVE | DECLARED |
| DAG-002-E0606 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-12-05 | ACTIVE | DECLARED |
| DAG-002-E0607 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-12-01 | ACTIVE | DECLARED |
| DAG-002-E0608 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-03-07 | ACTIVE | DECLARED |
| DAG-002-E0609 | EXECUTION | PREREQUISITE | UPSTREAM | DEL-06-04 | ACTIVE | DECLARED |
| DEL-12-04-E001 | EXECUTION | INTERFACE | UPSTREAM | DEL-12-02 | ACTIVE | EXTRACTED |
| DEL-12-04-E002 | EXECUTION | INTERFACE | UPSTREAM | DEL-12-03 | ACTIVE | EXTRACTED |

## Run Notes

- Mode: UPDATE.
- Strictness: CONSERVATIVE.
- Consumer context: RECONCILIATION.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for target resolution.
- Approved graph authority read: `execution/_DAG/DAG-006/DependencyEdges.csv`; `DAG-003` was not used as authority.
- Anchor doc: `Datasheet.md`; supplemental anchor evidence from `_CONTEXT.md`.
- Execution docs scanned: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`, and DAG-002 rows for DEL-12-04.
- Existing DAG-002 mirror rows were preserved by `DependencyID` and normalized to dependency-extract enums: `AnchorType=NOT_APPLICABLE`, `DependencyType=PREREQUISITE`, and `Origin=DECLARED`.
- Added four anchor rows for PKG-12, SOW-040, SOW-029, and OBJ-010.
- Added two local execution interface rows for DEL-12-02 redaction/export controls and DEL-12-03 telemetry-off design because DEL-12-04 source documents explicitly require those handling surfaces.
- No uncertain candidate rows were promoted. No RETIRED rows were needed in this refresh.
- [WARNING] ENUM_NORMALIZATION: Original DAG-002 mirror values included graph-specific enums (`ARCHITECTURE_BASIS`, `SECURITY_PREDECESSOR`, `CONTEXT`, `DECOMPOSITION`) that are not accepted by `skills/dependency-extract` enum validation. Local rows now preserve that provenance in `Notes`.
- [WARNING] GRAPH_AUTHORITY_BOUNDARY: New extracted rows are local evidence for RECONCILIATION only and do not modify approved `DAG-006`.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE counts |
|---|---|---|---|---|---|
| 2026-05-10 23:31 MDT | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` found | ENUM_NORMALIZATION; GRAPH_AUTHORITY_BOUNDARY | 17 total; 4 ANCHOR; 13 EXECUTION |

## Lifecycle Summary

| Status | Rows |
|---|---:|
| ACTIVE | 17 |
| RETIRED | 0 |

| SatisfactionStatus | Rows |
|---|---:|
| SATISFIED | 11 |
| TBD | 6 |

## Downstream Handoff Notes

- RECONCILIATION should compare the two new local interface rows (`DEL-12-04-E001`, `DEL-12-04-E002`) against approved `DAG-006`; they are evidence-backed local additions but are not aggregate authority.
- RECONCILIATION should note that the local v3.1 enum validator is narrower than the DAG-002 mirror vocabulary. This refresh stores graph-specific source classifications in `Notes` while keeping CSV enum fields validator-clean.
- No cycle-producing downstream edges were added from DEL-12-04.
