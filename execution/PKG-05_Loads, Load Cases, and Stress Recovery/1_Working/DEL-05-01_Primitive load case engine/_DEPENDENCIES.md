# Dependencies: DEL-05-01 Primitive load case engine

## Extracted Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Schema:** Dependencies.csv v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 8 ACTIVE; 4 RETIRED.
- **Active Classes:** 2 ANCHOR; 6 EXECUTION.
- **Generated:** 2026-05-10

| DependencyID | Class | Direction | Type | Target | Status | Confidence |
|---|---|---|---|---|---|---|
| TP-DAG-004-DEL-05-01-A001 | ANCHOR | UPSTREAM | OTHER | SOW-013 | ACTIVE | HIGH |
| TP-DAG-004-DEL-05-01-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE | HIGH |
| DAG-002-E0130 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | HIGH |
| DAG-002-E0131 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | HIGH |
| DAG-002-E0132 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | HIGH |
| DAG-002-E0133 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | HIGH |
| DAG-002-E0134 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | HIGH |
| TP-DAG-004-DEL-05-01-E001 | EXECUTION | DOWNSTREAM | INTERFACE | DEL-05-02 | ACTIVE | HIGH |
| DAG-002-E0446 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 | RETIRED | MEDIUM |
| DAG-002-E0447 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | RETIRED | MEDIUM |
| DAG-002-E0448 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-01 | RETIRED | MEDIUM |
| DAG-002-E0449 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-03 | RETIRED | MEDIUM |

## Run Notes
- Runtime overrides: `SCOPE=DEL-05-01`; `RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution`; `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; `MODE=UPDATE`; `STRICTNESS=CONSERVATIVE`; `CONSUMER_CONTEXT=RECONCILIATION`.
- Source document selection used defaults: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor evidence: `_CONTEXT.md`, validated against `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Chosen execution evidence: `_CONTEXT.md`, `Specification.md`, `Datasheet.md`, `Guidance.md`, and `Procedure.md`.
- Existing DAG-synchronized rows were normalized to v3.1 canonical enums: `DependencyType` now uses `CONSTRAINT`, `INTERFACE`, or `PREREQUISITE`; `Origin` now uses `EXTRACTED`; `Explicitness` now uses `EXPLICIT` or `IMPLICIT`; unknown closure state now uses `TBD`.
- Four prior inferred predecessor rows were retained non-destructively and marked `RETIRED` because conservative local extraction did not find explicit DEL-02-01, DEL-02-02, DEL-04-01, or DEL-04-03 references in assigned DEL-05-01 source documents.
- Parent anchor check passed: exactly one ACTIVE `IMPLEMENTS_NODE` row.
- No `[WARNING] MISSING_DECOMPOSITION`: decomposition path was supplied and readable.

## Run History
- 2026-04-30 10:26 - Initial dependency-extract run produced 6 ACTIVE rows under the earlier setup basis.
- 2026-05-03 - Local register was synchronized from DAG-002 with 9 ACTIVE rows.
- 2026-05-10 22:26 - TP-DAG-004 dependency-extract refresh, UPDATE, CONSERVATIVE, RECONCILIATION context; produced 12 rows: 8 ACTIVE and 4 RETIRED.

## Lifecycle Summary
- ACTIVE rows: 8
- RETIRED rows: 4
- Closure states: 7 SATISFIED; 1 PENDING; 4 TBD.
- Required maturity: all rows retain `SEMANTIC_READY`.
- Proposed maturity: all rows retain `SEMANTIC_READY`.

## Downstream Handoff Notes
- For RECONCILIATION: inspect the four retired predecessor edges against the aggregate DAG. They may still be valid as coordination-level or inferred schedule edges, but they are not supported as conservative local dependency-extract rows from the DEL-05-01 assigned source set.
- For RECONCILIATION: the active DEL-05-02 downstream interface is explicit in `Specification.md` REQ-05-01-007 and should be reconciled with PKG-05 package sequencing.
- This local register remains an evidence surface for DEL-05-01, not an aggregate graph authority.
