# Dependencies: DEL-13-04 Physical-to-analytical transformation contract

## Generated Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 20 total; 20 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Refreshed:** 2026-05-10

## Authority Boundary

- Aggregate `APPROVED_DAG002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed evidence surface for reconciliation, not an independent graph authority.
- `DAG-003` was not approved, promoted, or used as graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

### Counts

| Status | Rows |
|---|---:|
| ACTIVE | 20 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| Class | Rows |
|---|---:|
| ANCHOR | 2 |
| EXECUTION | 18 |

| Type | Rows |
|---|---:|
| OTHER | 2 |
| PREREQUISITE | 14 |
| PERSISTENCE_CONTRACT | 1 |
| ENABLES | 3 |

### Compact Active Register

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DEL-13-04-A001 | ANCHOR | UPSTREAM | OTHER | SOW-066 | `_CONTEXT.md` Scope Detail |
| DEL-13-04-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-014 | `Datasheet.md` Identification and Attributes |
| DAG-002-E0660 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0660 |
| DAG-002-E0661 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0661 |
| DAG-002-E0662 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0662 |
| DAG-002-E0663 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0663 |
| DAG-002-E0664 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0664 |
| DAG-002-E0665 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0665 |
| DAG-002-E0666 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0666 |
| DAG-002-E0772 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | `Procedure.md` Define Contract Inputs |
| DEV-001-PKG13-DEL1304-DEL0205 | EXECUTION | UPSTREAM | PERSISTENCE_CONTRACT | DEL-02-05 Project persistence and round-trip serialization | `Dependencies.csv`; DEV-001 Stage 2 finding resolution |
| DAG-002-E0773 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-01 Design knowledge schema and provenance model | `Procedure.md` Define Contract Inputs |
| DAG-002-E0774 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-02 Constraint entity and provenance model | `Procedure.md` Define Contract Inputs |
| DAG-002-E0775 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-03 Constraint validation engine | `Procedure.md` Define Contract Inputs |
| DAG-002-E0776 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 3D frame stiffness kernel | `Specification.md` DEL-13-04-REQ-007 |
| DAG-002-E0777 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-03 Linear support and restraint models | `Procedure.md` Define Contract Inputs |
| DAG-002-E0778 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-01 Primitive load case engine | `Procedure.md` Define Contract Inputs |
| DEL-13-04-D001 | EXECUTION | DOWNSTREAM | ENABLES | DEL-15-02 Target mapping and unsupported-behavior contract | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0809 |
| DEL-13-04-D002 | EXECUTION | DOWNSTREAM | ENABLES | DEL-15-03 Downstream modeling export workflow | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0816 |
| DEL-13-04-D003 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-08 Design-authoring state and comparison workspace | `execution/_DAG/DAG-006/DependencyEdges.csv` DAG-002-E0846 |

## Run Notes

- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used to validate DEL-13-04, PKG-13, SOW-066, and OBJ-014 labels.
- Approved graph authority: `execution/_DAG/DAG-006`; `DAG-003` was not used as authority.
- Anchor document selected: `Datasheet.md` plus `_CONTEXT.md` for explicit identity, SOW, and objective fields.
- Execution documents scanned: `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, and approved DAG-006 edge rows for this deliverable.
- Existing DAG-002 row IDs were preserved where target and statement matched the approved graph surface.
- Added two ACTIVE anchor rows for SOW-066 and OBJ-014.
- Added three ACTIVE downstream ENABLES rows for approved DAG-006 consumers DEL-15-02, DEL-15-03, and DEL-07-08.
- DEV-001 Stage 2 addendum: added one package-local ACTIVE DEL-02-05 persistence/hash/round-trip evidence row for transform outputs. This row is reconciliation evidence only and does not promote DAG-003, lifecycle state, or handoff readiness.
- Normalized local register enums to the validator's v3.1 set: execution rows use `AnchorType=NOT_APPLICABLE`, canonical `DependencyType`, `Origin`, `Explicitness`, and `SatisfactionStatus` values.
- [WARNING] ID format helper expects three-digit package/deliverable IDs (`PKG-013`, `DEL-013-04`) and rejects this repository's current stable IDs (`PKG-13`, `DEL-13-04`); treated as tool/schema mismatch, not a deliverable blocker.
- No active cycles, bidirectional pairs, or candidate promotions were introduced in this local surface.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Rows |
|---|---|---|---|---|---:|
| 2026-05-03 | SYNCHRONIZE | N/A | `execution/_DAG/DAG-006/DependencyEdges.csv` | DAG-002 mirror only; no local extraction anchors | 14 |
| 2026-05-10 23:38 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | ID format helper mismatch with repo stable IDs | 19 |
| 2026-05-16 | DEV-001_STAGE2_FINDING_RESOLUTION | PACKAGE_SCOPED | accepted PKG-02 contract | Added DEL-02-05 local evidence row; no DAG or lifecycle files changed | 20 |

## Lifecycle Summary

| Lifecycle Field | Breakdown |
|---|---|
| Status | ACTIVE: 20; RETIRED: 0; CANDIDATE: 0 |
| DependencyClass | ANCHOR: 2; EXECUTION: 18 |
| Direction | UPSTREAM: 17; DOWNSTREAM: 3 |
| SatisfactionStatus | NOT_APPLICABLE: 2; SATISFIED: 7; PENDING: 10; TBD: 1 |
| Origin | EXTRACTED: 9; DECLARED: 10; DEV001_STAGE2: 1 |

## Downstream Handoff Notes

- For RECONCILIATION, compare the 18 local execution rows against DAG-002 and any later proposed graph; do not treat this local file as approval to promote DAG-003.
- The three downstream ENABLES rows are included to expose approved consumers of DEL-13-04 output; they are reconciliation context, not new blocker authority.
- Semantic-lensing TODOs for transform-loss taxonomy, exact warning classes, fixtures, and module paths remain non-gating questions because local sources mark them `TBD`.
- No protected standards content, owner/project private data, code-specific defaults, or professional acceptance claims were introduced.
