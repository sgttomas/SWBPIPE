# Dependencies: DEL-11-05 Contributor tutorial and onboarding

## Generated Dependency Register
- **Status:** TP-DAG-004_REFRESHED_FROM_LOCAL_EVIDENCE
- **Prior Source of Truth:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Approved Graph Authority Used:** `execution/_DAG/DAG-002/`
- **Local Register:** `Dependencies.csv`
- **Rows:** 11 total; 11 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- `DAG-003` is preliminary and was not used as an approved/promoted source.
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
| `DAG-002-E0595` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-01` Project governance baseline | ACTIVE | TBD |
| `DAG-002-E0596` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-02` Copyright and protected-data boundary policy | ACTIVE | TBD |
| `DAG-002-E0597` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-03` Contributor certification workflow | ACTIVE | TBD |
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

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | Active Rows |
|---|---|---|---|---|---:|
| 2026-05-10T23:22:22-0600 | UPDATE | CONSERVATIVE | `execution/_Decomposition/SOFTWARE_DECOMP.md` found | none | 11 |

## Lifecycle Summary
- Status counts: 11 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- Class counts: 8 EXECUTION; 3 ANCHOR.
- Type counts: 8 PREREQUISITE; 3 OTHER.
- Target type counts: 8 DELIVERABLE; 1 WBS_NODE; 2 REQUIREMENT.
- Closure-state counts: 8 SATISFIED; 3 TBD.
- Parent anchor check: PASS, exactly one ACTIVE `IMPLEMENTS_NODE` anchor.

## Downstream Handoff Notes
- This refreshed surface is intended for later `RECONCILIATION` comparison against approved `DAG-002` and preliminary `DAG-003`.
- Local enum normalization changes may require aggregate reconciliation because DAG-002 carries graph-specific type labels in the same fields.
- The three governance predecessor rows remain ACTIVE but closure is `TBD` locally because the source edge evidence does not prove satisfaction inside this bounded refresh.
