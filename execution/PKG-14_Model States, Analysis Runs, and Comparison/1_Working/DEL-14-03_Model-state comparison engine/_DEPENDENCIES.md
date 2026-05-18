# Dependencies: DEL-14-03 Model-state comparison engine

## Generated Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Source of Truth:** approved DAG-002 mirror plus conservative deliverable-local refresh evidence
- **Local Register:** `Dependencies.csv`
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Last Refreshed:** 2026-05-10 23:48 by TP-DAG-004 dependency-extract refresh.

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
| ACTIVE | 15 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| Class | Rows |
|---|---:|
| ANCHOR | 2 |
| EXECUTION | 13 |

| Type | Rows |
|---|---:|
| OTHER | 2 |
| PREREQUISITE | 10 |
| ENABLES | 3 |

### Compact Active Register

| DependencyID | Class | Direction | Type | Target | Evidence |
|---|---|---|---|---|---|
| DEL-14-03-A001 | ANCHOR | UPSTREAM | OTHER | SOW-073 Compare model states and analysis runs | `_CONTEXT.md` Scope Coverage / Scope Detail |
| DEL-14-03-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-016 Manage immutable model states and comparisons | `_CONTEXT.md` Objective Support / Package Reference |
| DAG-002-E0681 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0682 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0683 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 Application service command-query-job model | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0684 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 Persistence and schema versioning architecture | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0685 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0686 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 API boundary and adapter contract map | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0687 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001 architecture-basis injection |
| DAG-002-E0792 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-14-01 Immutable model state records | `Procedure.md` Prerequisites / Steps |
| DAG-002-E0793 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-14-05 Comparison mapping, tolerance, and export contracts | `Procedure.md` Prerequisites / Steps |
| DAG-002-E0794 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 Unit system and dimensional-analysis core contract | `Specification.md` Requirements REQ-14-03-007 |
| DEL-14-03-D001 | EXECUTION | DOWNSTREAM | ENABLES | DEL-16-02 Operation validation and diff preview | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0829 |
| DEL-14-03-D002 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-08 Design-authoring state and comparison workspace | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0848 |
| DEL-14-03-D003 | EXECUTION | DOWNSTREAM | ENABLES | DEL-08-06 State, comparison, and handoff report sections | `execution/_DAG/DAG-002/DependencyEdges.csv` DAG-002-E0863 |

## Run Notes

- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used to validate DEL-14-03, PKG-14, SOW-073, SOW-071, and OBJ-016 labels.
- Approved graph authority: `execution/_DAG/DAG-002`; DAG-003 was not used as authority and no aggregate DAG file was edited.
- Anchor document selected: `Datasheet.md` plus `_CONTEXT.md` for explicit identity, SOW, and objective fields.
- Execution documents scanned: `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, and approved DAG-002 edge rows for this deliverable.
- Existing DAG-002 row IDs were preserved where target and statement matched the approved graph surface.
- Added two ACTIVE anchor rows for SOW-073 and OBJ-016.
- Added three ACTIVE downstream ENABLES rows for approved DAG-002 consumers DEL-16-02, DEL-07-08, and DEL-08-06.
- Normalized local register enums to the validator's v3.1 set: execution rows use `AnchorType=NOT_APPLICABLE`, canonical `DependencyType`, `Origin`, `Explicitness`, and `SatisfactionStatus` values. Original DAG-002 specialized edge types remain visible in statements/notes.
- [WARNING] ID format helper expects three-digit package/deliverable IDs (`PKG-014`, `DEL-014-03`) and rejects this repository's current stable IDs (`PKG-14`, `DEL-14-03`); treated as tool/schema mismatch, not a deliverable blocker.
- No active cycles, bidirectional pairs, candidate promotions, protected-data assumptions, engineering default values, or professional-approval claims were introduced in this local surface.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE Rows |
|---|---|---|---|---|---:|
| 2026-05-03 | SYNCHRONIZE | N/A | `execution/_DAG/DAG-002/DependencyEdges.csv` | DAG-002 mirror only; no local extraction anchors | 10 |
| 2026-05-10 23:48 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | ID format helper mismatch with repo stable IDs | 15 |

## Lifecycle Summary

| Lifecycle Field | Breakdown |
|---|---|
| Status | ACTIVE: 15; RETIRED: 0; CANDIDATE: 0 |
| DependencyClass | ANCHOR: 2; EXECUTION: 13 |
| Direction | UPSTREAM: 12; DOWNSTREAM: 3 |
| DependencyType | OTHER: 2; PREREQUISITE: 10; ENABLES: 3 |
| SatisfactionStatus | SATISFIED: 7; PENDING: 8 |
| Origin | DECLARED: 7; EXTRACTED: 8 |

## Downstream Handoff Notes

- For RECONCILIATION, compare the 13 execution rows against DAG-002 and any later proposed graph; do not treat this local file as approval to promote DAG-003.
- The three downstream ENABLES rows are included to expose approved consumers of DEL-14-03 output; they are reconciliation context, not new blocker authority.
- Mapping workflow, tolerance defaults, exact model entity categories, field normalization, backend module path, service/API syntax, and deterministic result-envelope hash checks remain non-gating TBD items because local sources defer them to DEL-14-01, DEL-14-05, DEL-02-02, or later sealed implementation work.
- No protected standards content, owner/project private data, code-specific defaults, or professional acceptance claims were introduced.
