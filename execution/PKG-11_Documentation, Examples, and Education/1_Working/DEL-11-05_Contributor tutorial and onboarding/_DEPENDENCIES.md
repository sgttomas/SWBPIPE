# Dependencies: DEL-11-05 Contributor tutorial and onboarding

## Generated Dependency Register
- **Status:** CURRENT_BASIS_DEPENDENCY_EVIDENCE_REFRESH
- **Prior Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Approved Graph Authority Used:** `execution/_DAG/DAG-006/`
- **Local Register:** `Dependencies.csv`
- **Rows:** 11 total; 11 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-10
- **Refreshed:** 2026-06-07 for PKG-01 predecessor satisfaction evidence.

## Authority Boundary
- Aggregate `DAG-006` remains the approved active graph authority within its approval boundary.
- Historical `DAG-002` row IDs are preserved as local evidence identifiers; they are not current aggregate authority by themselves.
- `DAG-003` remains historical preliminary evidence and was not used as an approved/promoted source.
- This local register is refreshed dependency evidence for later `RECONCILIATION`; it is not an independent aggregate graph authority.
- `CANDIDATE` rows remain non-gating until later `RECONCILIATION` plus `CHANGE` approval.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| `DAG-002-E0349` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-00-01` Architecture decision record baseline | ACTIVE | SATISFIED |
| `DAG-002-E0350` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-00-02` Repository and module boundary architecture | ACTIVE | SATISFIED |
| `DAG-002-E0351` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-00-06` Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED |
| `DAG-002-E0352` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-00-07` API boundary and adapter contract map | ACTIVE | SATISFIED |
| `DAG-002-E0353` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-00-08` Layered software test and acceptance strategy | ACTIVE | SATISFIED |
| `DAG-002-E0595` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-01` Project governance baseline | ACTIVE | SATISFIED |
| `DAG-002-E0596` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-02` Copyright and protected-data boundary policy | ACTIVE | SATISFIED |
| `DAG-002-E0597` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-03` Contributor certification workflow | ACTIVE | SATISFIED |
| `DEL-11-05-A001` | ANCHOR | UPSTREAM | OTHER | `SOW-033` Documentation and invented-data examples | ACTIVE | SATISFIED |
| `DEL-11-05-A002` | ANCHOR | UPSTREAM | OTHER | `OBJ-001` Open auditable piping stress analysis platform | ACTIVE | SATISFIED |
| `DEL-11-05-A003` | ANCHOR | UPSTREAM | OTHER | `OBJ-002` Protect standards-body and vendor intellectual property | ACTIVE | SATISFIED |

## Run Notes
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Consumer context: `RECONCILIATION`.
- Run root: `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Anchor doc selected: `Datasheet.md`.
- Execution docs scanned: `_CONTEXT.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, existing `Dependencies.csv`, and approved `execution/_DAG/DAG-002` references.
- Preserved all 8 existing DAG-002 dependency IDs and target/evidence statements.
- Normalized local register enum fields for dependency-extract v3.1 hygiene: execution `AnchorType` to `NOT_APPLICABLE`, graph-specific dependency types to `PREREQUISITE`, `INFERRED_DIRECT` to `IMPLICIT`, `CONTEXT`/`DECOMPOSITION` to `EXTRACTED`, and unresolved `UNKNOWN` satisfaction to `TBD`.
- Added one parent anchor for explicit scope item `SOW-033`.
- Added two trace anchors for explicit objective support `OBJ-001` and `OBJ-002`.
- No candidate rows were created; no rows were retired.
- No protected standards text, proprietary data, private rule-pack values, or professional/code-compliance claims were introduced.
- 2026-06-07 evidence refresh checked current `DAG-006` active rows `DAG-004-R0643`, `DAG-004-R0644`, and `DAG-004-R0645` plus upstream deliverable-local `_STATUS.md` and `MEMORY.md` files.
- `DEL-01-01` now has local lifecycle `ISSUED` evidence after prior `SEMANTIC_READY` history; `DEL-01-02` and `DEL-01-03` now have local lifecycle `CHECKING` evidence after prior `SEMANTIC_READY` history. These meet or exceed the `SEMANTIC_READY` predecessor requirement for this contributor-onboarding dependency purpose.
- Remaining upstream governance `TBD`s, including contributor legal mechanism, reviewer/legal-review authority, maintainer authority, release authority, and related human-governed decisions, are not resolved by this dependency evidence update.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | Active Rows |
|---|---|---|---|---|---:|
| 2026-05-10T23:22:22-0600 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` found | none | 11 |
| 2026-06-07T17:33:28-0600 | EVIDENCE_REFRESH | CONSERVATIVE | `execution/_DAG/DAG-006/` active graph authority plus upstream local status/memory evidence | upstream governance TBDs preserved | 11 |

## Lifecycle Summary
- Status counts: 11 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- Class counts: 8 EXECUTION; 3 ANCHOR.
- Type counts: 8 PREREQUISITE; 3 OTHER.
- Target type counts: 8 DELIVERABLE; 1 WBS_NODE; 2 REQUIREMENT.
- Closure-state counts: 11 SATISFIED; 0 TBD.
- Parent anchor check: PASS, exactly one ACTIVE `IMPLEMENTS_NODE` anchor.

## Downstream Handoff Notes
- This refreshed surface is intended for later `RECONCILIATION` comparison against approved `DAG-006` and preserved historical row lineage.
- Local enum normalization changes may require aggregate reconciliation because historical DAG rows carry graph-specific type labels in the same fields.
- The three governance predecessor rows remain ACTIVE and are now locally `SATISFIED` for the `SEMANTIC_READY` dependency purpose using upstream local lifecycle and memory evidence. This is deliverable-local dependency evidence only; it does not edit aggregate `DAG-006`, issue downstream work, close upstream governance `TBD`s, approve a release, make a legal conclusion, or create professional/code-compliance authority.
