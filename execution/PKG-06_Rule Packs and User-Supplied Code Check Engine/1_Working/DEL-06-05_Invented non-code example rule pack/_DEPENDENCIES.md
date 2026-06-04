# Dependencies: DEL-06-05 Invented non-code example rule pack

## Generated Dependency Register
- **Prior status:** SYNCHRONIZED_FROM_DAG_002
- **Prior source of truth:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Refresh status:** UPDATED_BY_DEPENDENCY_EXTRACT_REFRESH
- **Local register:** `Dependencies.csv`
- **Rows:** 14 total; 12 ACTIVE; 2 RETIRED.
- **Generated:** 2026-05-03
- **Refreshed:** 2026-05-10 22:36 MDT

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a deliverable-local evidence surface for dependency extraction and reconciliation, not an independent graph authority.
- Retired rows are retained non-destructively and are not deleted.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| Class | Status | Count |
|---|---:|---:|
| ANCHOR | ACTIVE | 3 |
| EXECUTION | ACTIVE | 9 |
| EXECUTION | RETIRED | 2 |

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DEP-DEL-06-05-A001 | ANCHOR | UPSTREAM | OTHER | SOW-016 | ACTIVE | SATISFIED |
| DEP-DEL-06-05-A002 | ANCHOR | UPSTREAM | OTHER | DEL-06-05-REQ-01 | ACTIVE | SATISFIED |
| DEP-DEL-06-05-A003 | ANCHOR | UPSTREAM | OTHER | DEL-06-05-REQ-03 | ACTIVE | SATISFIED |
| DAG-002-E0183 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 | ACTIVE | SATISFIED |
| DAG-002-E0184 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 | ACTIVE | SATISFIED |
| DAG-002-E0185 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-03 | ACTIVE | SATISFIED |
| DAG-002-E0186 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 | ACTIVE | SATISFIED |
| DAG-002-E0187 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 | ACTIVE | SATISFIED |
| DAG-002-E0188 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 | ACTIVE | SATISFIED |
| DAG-002-E0189 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 | ACTIVE | SATISFIED |
| DAG-002-E0474 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-01 | ACTIVE | PENDING |
| DAG-002-E0475 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-02 | ACTIVE | PENDING |
| DAG-002-E0476 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-02 | RETIRED | TBD |
| DAG-002-E0477 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-04 | RETIRED | TBD |

## Run Notes
- **Mode:** UPDATE.
- **Strictness:** CONSERVATIVE.
- **Consumer context:** RECONCILIATION.
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- **Decomposition status:** located and used for SOW-016, PKG-06, DEL-06-05, DEL-06-01, and DEL-06-02 validation.
- **Anchor doc:** `Datasheet.md`.
- **Execution docs:** `_CONTEXT.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`.
- **Contract governance:** `docs/CONTRACT.md` was read as a required governance document referenced by the deliverable source set.
- **Contract lookup note:** no `CONTRACT.md` exists at repository root or under `execution/`; required contract content was found at `docs/CONTRACT.md`.
- **Warnings:** none. Parent anchor count is exactly one; decomposition was located.
- **Conservative retirement note:** `DAG-002-E0476` and `DAG-002-E0477` were retained but marked RETIRED because the allowed DEL-06-05 source evidence contains the protected-content and professional-boundary constraints, but does not explicitly cite target deliverables `DEL-01-02` or `DEL-01-04`.

## Run History
- 2026-05-03: Generated from `execution/_DAG/DAG-006/DependencyEdges.csv`; 11 ACTIVE rows.
- 2026-05-10 22:36 MDT: dependency-extract refresh, MODE=UPDATE, STRICTNESS=CONSERVATIVE, CONSUMER_CONTEXT=RECONCILIATION, decomposition located, ACTIVE rows=12, RETIRED rows=2, warnings=none.

## Lifecycle Summary
- **ACTIVE:** 12.
- **RETIRED:** 2.
- **Satisfaction SATISFIED:** 10.
- **Satisfaction PENDING:** 2.
- **Satisfaction TBD:** 2.
- **Closure state:** anchor and architecture-basis constraints are satisfied; rule-pack schema and evaluator prerequisites remain pending; retired governance-predecessor mappings require RECONCILIATION decision if they should remain graph edges.

## Downstream Handoff Notes
- For RECONCILIATION, review whether retired `DEL-01-02` and `DEL-01-04` mappings should be restored from an aggregate DAG authority, remapped to governance document targets, or left retired in the local extracted register.
- The active local extraction identifies `DEL-06-01` and `DEL-06-02` as evidence-backed prerequisites for future schema-specific and evaluator-specific example content.
- All rows now use canonical v3.1 enum values.
