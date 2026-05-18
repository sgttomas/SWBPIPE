# Dependencies: DEL-11-02 Developer guide for solver and rule packs

## Generated Dependency Register
- **Status:** TP-DAG-004_LOCAL_REFRESH
- **Prior Source of Truth:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Approved graph authority:** `execution/_DAG/DAG-002/`
- **Local Register:** `Dependencies.csv`
- **Rows:** 15 total; 15 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Refreshed:** 2026-05-11

## Authority Boundary
- Aggregate `DAG-002` remains the approved sequencing and blocker-computation authority within its approval boundary.
- `DAG-003` remains preliminary and was not read as approval authority, promoted, or modified by this run.
- This local register is a refreshed deliverable-local evidence surface for later RECONCILIATION, not an independent graph approval.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval. No candidate rows were introduced for DEL-11-02 in this run.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| Count | Meaning |
|---:|---|
| 11 | Preserved active DAG-002 mirror execution rows |
| 4 | Added active local anchor rows from explicit DEL-11-02 identifiers |
| 0 | Retired rows |
| 0 | Candidate rows |

| DependencyID | Class | Anchor/Type | Direction | Target | Status | Origin |
|---|---|---|---|---|---|---|
| DAG-002-E0334 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-01 Architecture decision record baseline | ACTIVE | CONTEXT |
| DAG-002-E0335 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-02 Repository and module boundary architecture | ACTIVE | CONTEXT |
| DAG-002-E0336 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | CONTEXT |
| DAG-002-E0337 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-07 API boundary and adapter contract map | ACTIVE | CONTEXT |
| DAG-002-E0338 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | CONTEXT |
| DAG-002-E0580 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-02-01 Canonical domain model schema | ACTIVE | DECOMPOSITION |
| DAG-002-E0581 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE | DECOMPOSITION |
| DAG-002-E0582 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-04-01 3D frame stiffness kernel | ACTIVE | DECOMPOSITION |
| DAG-002-E0583 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-06-01 Rule-pack schema | ACTIVE | DECOMPOSITION |
| DAG-002-E0584 | EXECUTION | DOCS_PREDECESSOR | UPSTREAM | DEL-10-01 Public API and plugin boundary | ACTIVE | DECOMPOSITION |
| DAG-002-E0585 | EXECUTION | GOVERNANCE_PREDECESSOR | UPSTREAM | DEL-01-02 Copyright and protected-data boundary policy | ACTIVE | DECOMPOSITION |
| DEP-011-02-001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | PKG-11 Documentation, Examples, and Education | ACTIVE | EXTRACTED |
| DEP-011-02-002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | SOW-033 | ACTIVE | EXTRACTED |
| DEP-011-02-003 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-001 | ACTIVE | EXTRACTED |
| DEP-011-02-004 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-002 | ACTIVE | EXTRACTED |

## Run Notes
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Consumer context: `RECONCILIATION`.
- Run root: `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` located and used for label validation.
- Approved graph authority used for prior mirror context: `execution/_DAG/DAG-002/`.
- Preliminary graph boundary: `execution/_DAG/DAG-003/` was not promoted or edited.
- Anchor doc: `Datasheet.md`.
- Execution docs reviewed: `_CONTEXT.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`.
- Existing DAG-002 execution rows were preserved because they remain valid approved graph-authority mirror evidence for this deliverable.
- Added one `IMPLEMENTS_NODE` anchor to `PKG-11` and three `TRACES_TO_REQUIREMENT` anchors to `SOW-033`, `OBJ-001`, and `OBJ-002`.
- [WARNING] ENUM_TOOL_SCOPE: `tools/validation/validate_enum.py` does not include approved DAG-002 local values such as `ARCHITECTURE_BASIS`, `DOCS_PREDECESSOR`, `GOVERNANCE_PREDECESSOR`, `CONTEXT`, `DECOMPOSITION`, `INFERRED_DIRECT`, `UNKNOWN`, or `CANDIDATE`. Those rows were preserved under DAG-002 authority; the four newly extracted anchor rows use canonical enum values accepted by the tool where applicable.
- No protected standards content, private data, engineering default values, or professional approval claims were added.

## Run History
- 2026-04-30 1215: Initial dependency extraction run; local register later synchronized from DAG-002.
- 2026-05-03: Local dependency register synchronized from approved `DAG-002` mirror; 11 ACTIVE execution rows.
- 2026-05-11: TP-DAG-004 dependency surface refresh for RECONCILIATION. Mode `UPDATE`, strictness `CONSERVATIVE`, decomposition located, DAG-002 authority preserved, 15 ACTIVE rows total: 11 EXECUTION and 4 ANCHOR.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 15 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 9 |
| UNKNOWN | 6 |

| DependencyClass | Count |
|---|---:|
| EXECUTION | 11 |
| ANCHOR | 4 |

## Downstream Handoff Notes
- RECONCILIATION should treat the four `DEP-011-02-*` rows as local traceability anchors, not DAG sequencing edges.
- RECONCILIATION should continue to use `DAG-002` as approved graph authority until a later refreshed graph proposal is explicitly approved.
- The preserved DAG-002 mirror rows contain project-specific dependency types and origins that exceed the current narrow enum validator; this is a tooling/schema-alignment warning, not a local data rewrite request.
