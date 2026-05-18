# Dependencies: DEL-03-08 Pipe section property and mass-property calculator

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Schema:** v3.1, 29 required columns, 0 extension columns
- **Rows:** 9 total; 9 ACTIVE; 0 CANDIDATE
- **Generated:** 2026-05-11

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror and evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Preserved Active Upstream Edges

| DependencyID | Target | Type | Status | Refresh disposition |
|---|---|---|---|---|
| DAG-002-E0094 | DEL-00-01 Architecture decision record baseline | ARCHITECTURE_BASIS | ACTIVE | Preserved from `_CONTEXT.md` AB-00-01 injection. |
| DAG-002-E0095 | DEL-00-02 Repository and module boundary architecture | ARCHITECTURE_BASIS | ACTIVE | Preserved from `_CONTEXT.md` AB-00-02 injection; reinforced by calculator/solver boundary requirements. |
| DAG-002-E0096 | DEL-00-04 Persistence and schema versioning architecture | ARCHITECTURE_BASIS | ACTIVE | Preserved from `_CONTEXT.md` AB-00-04 injection; relevant to provenance-preserving schema hooks. |
| DAG-002-E0097 | DEL-00-06 Diagnostics, warning, and result-envelope contract | ARCHITECTURE_BASIS | ACTIVE | Preserved from `_CONTEXT.md` AB-00-06 injection; Procedure keeps exact diagnostics as `TBD`. |
| DAG-002-E0098 | DEL-00-07 API boundary and adapter contract map | ARCHITECTURE_BASIS | ACTIVE | Preserved from `_CONTEXT.md` AB-00-07 injection; relevant to unit/provenance validation boundaries. |
| DAG-002-E0099 | DEL-00-08 Layered software test and acceptance strategy | ARCHITECTURE_BASIS | ACTIVE | Preserved from `_CONTEXT.md` AB-00-08 injection; reinforced by local verification expectations. |
| DAG-002-E0426 | DEL-03-01 Material library schema with provenance | DOMAIN_MODEL | ACTIVE | Preserved; Specification requires library-sourced material/provenance carry-through. |
| DAG-002-E0427 | DEL-03-02 Pipe section and component library schema | DOMAIN_MODEL | ACTIVE | Preserved; Procedure identifies section/component schema as a required input for hooks. |
| DAG-002-E0428 | DEL-02-02 Unit system and dimensional-analysis core contract | UNIT_CONTRACT | ACTIVE | Preserved; Specification requires unit-aware and dimensionally checked calculations. |

## Run Notes
- Applied defaults: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=_CONTEXT.md`, `EXECUTION_DOC_ORDER=Specification.md, Procedure.md, Guidance.md, Datasheet.md`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- The refresh found no conservative basis for adding new dependencies beyond the nine active DAG-002 rows already mirrored here.
- No `RESET_EXTRACTED` behavior was applied; prior active edge IDs were retained.
- Satisfaction for the three non-architecture upstream contract edges remains `UNKNOWN` because target deliverable maturity and content were outside this bounded read scope.
- No protected dimensional tables, material tables, proprietary data, or code-compliance content were introduced.

## Downstream Handoff Notes
- RECONCILIATION should treat `DEL-03-08` as blocked on unit-system, schema-hook, provenance, and diagnostic contract resolution before implementation.
- Open dependency-facing TBDs are: unit API/dimensions/tolerances, schema field ownership for inputs/outputs, diagnostic codes/classes for invalid geometry or missing provenance, and synthetic fixture policy.
- The local evidence supports existing `DEL-02-02`, `DEL-03-01`, `DEL-03-02`, and `DEL-00-06` edges; it does not establish a new validation-suite dependency under conservative strictness.

## Lifecycle Summary
- 2026-05-03: Local register synchronized from DAG-002.
- 2026-05-11: TP-DAG-004 dependency-extract refresh completed in UPDATE mode for RECONCILIATION consumer context.
