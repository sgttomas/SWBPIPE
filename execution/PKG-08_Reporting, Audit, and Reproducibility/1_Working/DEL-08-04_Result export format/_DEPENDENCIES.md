# Dependencies: DEL-08-04 Result export format

## Extracted Dependency Register

- **Register schema:** `Dependencies.csv` v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Rows:** 22 total; 20 ACTIVE; 2 RETIRED
- **Active classes:** 3 ANCHOR; 17 EXECUTION
- **Retired rows:** `DAG-002-E0516`, `DAG-002-E0517`

| DependencyID | Class | Direction | Type | Target | Status | Confidence |
|---|---|---:|---|---|---|---|
| DEP-08-04-A001 | ANCHOR | UPSTREAM | OTHER | SOW-046 | ACTIVE | HIGH |
| DEP-08-04-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-007 | ACTIVE | HIGH |
| DEP-08-04-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-009 | ACTIVE | HIGH |
| DAG-002-E0260 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | HIGH |
| DAG-002-E0261 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | HIGH |
| DAG-002-E0262 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | HIGH |
| DAG-002-E0263 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 | ACTIVE | HIGH |
| DAG-002-E0264 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | HIGH |
| DAG-002-E0265 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 | ACTIVE | HIGH |
| DAG-002-E0266 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | HIGH |
| DAG-002-E0514 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 | ACTIVE | HIGH |
| DAG-002-E0515 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | ACTIVE | HIGH |
| DEP-08-04-E001 | EXECUTION | UPSTREAM | HANDOVER | DEL-08-02 | ACTIVE | HIGH |
| DEP-08-04-E002 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-04 | ACTIVE | HIGH |
| DEP-08-04-E003 | EXECUTION | UPSTREAM | INTERFACE | PKG-04 | ACTIVE | MEDIUM |
| DEP-08-04-E004 | EXECUTION | UPSTREAM | INTERFACE | PKG-05 | ACTIVE | MEDIUM |
| DEP-08-04-E005 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-05 | ACTIVE | HIGH |
| DEP-08-04-E006 | EXECUTION | DOWNSTREAM | ENABLES | DEL-08-01 | ACTIVE | HIGH |
| DEP-08-04-E007 | EXECUTION | DOWNSTREAM | ENABLES | DEL-10-02 | ACTIVE | HIGH |
| DEP-08-04-E008 | EXECUTION | DOWNSTREAM | ENABLES | DEL-10-05 | ACTIVE | HIGH |
| DAG-002-E0516 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-05-03 | RETIRED | LOW |
| DAG-002-E0517 | EXECUTION | UPSTREAM | INTERFACE | DEL-05-04 | RETIRED | LOW |

## Run Notes

- `RUN_ROOT`: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- `DECOMPOSITION_PATH`: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- `SOURCE_DOCS`: AUTO
- `DOC_ROLE_MAP`: DEFAULT
- `ANCHOR_DOC`: AUTO, resolved to `Datasheet.md`
- `EXECUTION_DOC_ORDER`: AUTO, resolved to `Procedure.md`, `Specification.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`
- Decomposition status: available and used to validate `SOW-046`, `OBJ-007`, `OBJ-009`, `PKG-08`, and named deliverables.
- Conservative handling: active deliverable-to-deliverable rows require explicit IDs or explicit architecture-basis IDs in the assigned deliverable sources. Broad producer references to `PKG-04` and `PKG-05` remain package-level rows.
- Retired handling: prior DAG mirror rows for `DEL-05-03` and `DEL-05-04` were normalized but marked `RETIRED` because the current assigned source documents do not explicitly name those deliverable IDs.
- Warning: none.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | Active Rows | Retired Rows | Notes |
|---|---|---|---|---|---:|---:|---|
| 2026-05-10 22:56 MDT | UPDATE | CONSERVATIVE | available | none | 20 | 2 | TP-DAG-004 refresh for RECONCILIATION; converted prior DAG mirror to deliverable-local v3.1 extracted register. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 20 |
| RETIRED | 2 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 10 |
| TBD | 12 |

## Downstream Handoff Notes

- RECONCILIATION should treat active `DEP-08-04-E005` through `DEP-08-04-E008` as consumer-surface candidates for GUI results review, report generation, import/export adapters, and headless automation.
- RECONCILIATION should review whether retired `DEL-05-03` and `DEL-05-04` edges should remain retired, be restored from a different authoritative source, or be replaced by the broader active package-level producer edges for `PKG-04` and `PKG-05`.
- This local register is not a project-level graph authority; aggregation and blocker computation remain downstream responsibilities.
