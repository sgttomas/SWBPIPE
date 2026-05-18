# Dependencies: DEL-07-02 Model tree and property inspector

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Source of Truth:** Deliverable-local dependency-extract refresh for `DEL-07-02`.
- **Local Register:** `Dependencies.csv`
- **Rows:** 19 total; 19 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- This local register is a deliverable-local evidence surface for RECONCILIATION, not an independent project graph authority.
- The refresh preserves prior DAG-002 row identities where rows remained matchable, while normalizing enum fields to dependency-extract v3.1.
- No source documents, status files, memory files, code, schema, tests, DAG files, or coordination artifacts were edited.
- Upstream `PKG-00` architecture-basis rows remain context constraints; they do not mark `PKG-00` as `ISSUED`.

## Extracted Dependency Register

### Counts

| Class | ACTIVE | RETIRED | Notes |
|---|---:|---:|---|
| ANCHOR | 3 | 0 | One parent anchor and two SOW trace anchors. |
| EXECUTION | 16 | 0 | Upstream prerequisites plus downstream GUI-slice interface boundaries. |
| TOTAL | 19 | 0 | v3.1 schema, 29 required columns. |

### Compact Table

| DependencyID | Class | Direction | Type | Target | Satisfaction |
|---|---|---|---|---|---|
| DEP-007-02-001 | ANCHOR | UPSTREAM | OTHER | PKG-07 | NOT_APPLICABLE |
| DEP-007-02-002 | ANCHOR | UPSTREAM | OTHER | SOW-020 | NOT_APPLICABLE |
| DEP-007-02-003 | ANCHOR | UPSTREAM | OTHER | SOW-021 | NOT_APPLICABLE |
| DAG-002-E0197 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | SATISFIED |
| DAG-002-E0198 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | SATISFIED |
| DAG-002-E0199 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 | SATISFIED |
| DAG-002-E0200 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-05 | SATISFIED |
| DAG-002-E0201 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | SATISFIED |
| DAG-002-E0202 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 | SATISFIED |
| DAG-002-E0203 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | SATISFIED |
| DAG-002-E0486 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 | TBD |
| DEP-007-02-004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | TBD |
| DAG-002-E0487 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-05 | TBD |
| DEP-007-02-005 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-01 | TBD |
| DAG-002-E0488 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-02 | TBD |
| DEP-007-02-006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-01 | TBD |
| DEP-007-02-007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-04 | TBD |
| DEP-007-02-008 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-07-03 | TBD |
| DEP-007-02-009 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-07-04 | TBD |

## Run Notes

- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **ConsumerContext:** RECONCILIATION
- **RUN_ROOT:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **DECOMPOSITION_PATH:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition status:** Located and used for target-label and anchor validation.
- **SOURCE_DOCS:** AUTO
- **DOC_ROLE_MAP:** DEFAULT
- **ANCHOR_DOC:** AUTO selected `Datasheet.md` with `_CONTEXT.md` used for explicit ID fields.
- **EXECUTION_DOC_ORDER:** AUTO selected `_CONTEXT.md`, `Specification.md`, `Procedure.md`, and `Guidance.md`; `_REFERENCES.md` was used only as a pointer surface.
- **Schema posture:** `Dependencies.csv` retained the v3.1 29-column schema.
- **Enum posture:** Legacy/noncanonical enum values from the prior DAG-002 mirror were normalized: `AnchorType=NOT_APPLICABLE` for execution rows; `DependencyType` mapped to `PREREQUISITE`, `INTERFACE`, or `OTHER`; `Explicitness` mapped to `EXPLICIT`; `Origin` mapped to `EXTRACTED`.
- **Non-destructive update:** Prior matchable DAG-002 dependency IDs were preserved. No rows were deleted.
- **Warnings:** None. Parent anchor count is exactly one; decomposition was available.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE counts |
|---|---|---|---|---|---|
| 2026-05-03 | SYNCHRONIZED_FROM_DAG_002 | N/A | DAG-002 aggregate mirror | N/A | 10 total |
| 2026-05-10 22:43 | UPDATE | CONSERVATIVE | `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` | None | 19 total: 3 ANCHOR, 16 EXECUTION |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 19 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 7 |
| NOT_APPLICABLE | 3 |
| TBD | 9 |

## Downstream Handoff Notes

- RECONCILIATION should treat the two downstream `INTERFACE` rows to `DEL-07-03` and `DEL-07-04` as boundary-control evidence, not as sequencing blockers by themselves.
- The refreshed register intentionally surfaces missing upstream satisfaction checks for domain, unit, persistence, material/component, and rule-pack contracts as `TBD`.
- Architecture-basis prerequisites remain marked `SATISFIED` because `_CONTEXT.md` states the PKG-00 architecture basis is dispatchable context for this sealed deliverable.
