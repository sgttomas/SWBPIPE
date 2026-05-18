# Dependencies: DEL-13-02 Constraint entity and provenance model

## Generated Dependency Register

- **Status:** TP-DAG-004_LOCAL_REFRESH
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Prior Source of Truth:** `execution/_DAG/DAG-002/DependencyEdges.csv`
- **Approved graph authority:** `execution/_DAG/DAG-002/`
- **Local Register:** `Dependencies.csv`
- **Rows:** 18 total; 18 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Refreshed:** 2026-05-11

## Authority Boundary

- Aggregate `DAG-002` remains the approved sequencing and blocker-computation authority within its approval boundary.
- `DAG-003` remains preliminary and was not read as approval authority, promoted, or modified by this run.
- This local register is a refreshed deliverable-local evidence surface for later RECONCILIATION, not an independent graph approval.
- Candidate rows remain non-gating until later RECONCILIATION plus CHANGE approval. No candidate rows were introduced for DEL-13-02 in this run.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| Count | Meaning |
|---:|---|
| 12 | Preserved active DAG-002 mirror execution rows |
| 5 | Added active local anchor rows from explicit DEL-13-02 identifiers |
| 1 | Added active downstream handoff row supported by local source text and approved DAG-002 inverse evidence |
| 0 | Retired rows |
| 0 | Candidate rows |

| DependencyID | Class | Anchor/Type | Direction | Target | Status | Origin |
|---|---|---|---|---|---|---|
| DAG-002-E0646 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-01 Architecture decision record baseline | ACTIVE | CONTEXT |
| DAG-002-E0647 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-02 Repository and module boundary architecture | ACTIVE | CONTEXT |
| DAG-002-E0648 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-03 Application service command-query-job model | ACTIVE | CONTEXT |
| DAG-002-E0649 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | CONTEXT |
| DAG-002-E0650 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | CONTEXT |
| DAG-002-E0651 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-07 API boundary and adapter contract map | ACTIVE | CONTEXT |
| DAG-002-E0652 | EXECUTION | ARCHITECTURE_BASIS | UPSTREAM | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | CONTEXT |
| DAG-002-E0762 | EXECUTION | DOMAIN_MODEL | UPSTREAM | DEL-13-01 Design knowledge schema and provenance model | ACTIVE | GRAPH_REVIEW |
| DAG-002-E0763 | EXECUTION | DOMAIN_MODEL | UPSTREAM | DEL-02-01 Canonical domain model schema | ACTIVE | GRAPH_REVIEW |
| DAG-002-E0764 | EXECUTION | UNIT_CONTRACT | UPSTREAM | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE | GRAPH_REVIEW |
| DAG-002-E0765 | EXECUTION | PERSISTENCE_CONTRACT | UPSTREAM | DEL-02-05 Project persistence and round-trip serialization | ACTIVE | GRAPH_REVIEW |
| DAG-002-E0766 | EXECUTION | GOVERNANCE_PREDECESSOR | UPSTREAM | DEL-01-04 Professional responsibility and product-claims policy | ACTIVE | GRAPH_REVIEW |
| DEP-013-02-001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | PKG-13 Physical Design Knowledge and Constraint Engine | ACTIVE | EXTRACTED |
| DEP-013-02-002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | SOW-068 | ACTIVE | EXTRACTED |
| DEP-013-02-003 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | SOW-067 | ACTIVE | EXTRACTED |
| DEP-013-02-004 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-014 | ACTIVE | EXTRACTED |
| DEP-013-02-005 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | OBJ-018 | ACTIVE | EXTRACTED |
| DEP-013-02-006 | EXECUTION | HANDOVER | DOWNSTREAM | DEL-13-03 Constraint validation engine | ACTIVE | EXTRACTED |

## Run Notes

- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor source: `Datasheet.md`.
- Chosen execution sources: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, and approved local-mirror rows from `execution/_DAG/DAG-002`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`; anchor identifiers were validated against the decomposition/register content available in the repo.
- Approved graph authority consulted: `execution/_DAG/DAG-002`; `DAG-003` was treated as preliminary and was not approved, promoted, or used as authority.
- Existing DAG-002 execution rows were preserved unchanged because they remain the approved graph-authority mirror evidence for this deliverable.
- Added one `IMPLEMENTS_NODE` anchor to `PKG-13` and four `TRACES_TO_REQUIREMENT` anchors to `SOW-068`, `SOW-067`, `OBJ-014`, and `OBJ-018`.
- Added one downstream handoff to `DEL-13-03` because local guidance explicitly assigns deterministic validation behavior to DEL-13-03 and approved DAG-002 already contains the inverse dependency `DAG-002-E0768`.
- No candidate row was introduced. The possible downstream consumer `DEL-13-04` remains represented only in approved DAG-002 inverse evidence because the local DEL-13-02 source text was less direct than the DEL-13-03 handoff.
- [WARNING] ENUM_TOOL_SCOPE: `tools/validation/validate_enum.py` does not include approved DAG-002 local mirror values such as `ARCHITECTURE_BASIS`, `DOMAIN_MODEL`, `UNIT_CONTRACT`, `PERSISTENCE_CONTRACT`, `GOVERNANCE_PREDECESSOR`, `CONTEXT`, `GRAPH_REVIEW`, `INFERRED_DIRECT`, `UNKNOWN`, or `CANDIDATE`. Those prior rows were preserved under DAG-002 authority; newly extracted rows use canonical enum values accepted by the tool where applicable.
- [WARNING] ID_FORMAT_TOOL_SCOPE: `tools/validation/validate_id_format.sh` expects legacy three-digit package/deliverable IDs and rejects current decomposition IDs such as `PKG-13`, `DEL-13-02`, `SOW-068`, and `OBJ-014`. Canonical current IDs were preserved.
- No protected standards content, private data, engineering default values, implementation schema claims, or professional approval/code-compliance claims were added.

## Run History

- 2026-05-03: Local dependency register synchronized from approved `DAG-002` mirror; 12 ACTIVE execution rows.
- 2026-05-11: TP-DAG-004 dependency surface refresh for RECONCILIATION. Mode `UPDATE`, strictness `CONSERVATIVE`, decomposition located, DAG-002 authority preserved, 18 ACTIVE rows total: 13 EXECUTION and 5 ANCHOR.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 18 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| DependencyClass | Count |
|---|---:|
| EXECUTION | 13 |
| ANCHOR | 5 |

| DependencyType | Count |
|---|---:|
| ARCHITECTURE_BASIS | 7 |
| DOMAIN_MODEL | 2 |
| UNIT_CONTRACT | 1 |
| PERSISTENCE_CONTRACT | 1 |
| GOVERNANCE_PREDECESSOR | 1 |
| OTHER | 5 |
| HANDOVER | 1 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 7 |
| UNKNOWN | 5 |
| NOT_APPLICABLE | 5 |
| TBD | 1 |

| Origin | Count |
|---|---:|
| CONTEXT | 7 |
| GRAPH_REVIEW | 5 |
| EXTRACTED | 6 |

## Downstream Handoff Notes

- RECONCILIATION should treat the five `DEP-013-02-00*` anchor rows as local traceability anchors, not DAG sequencing edges.
- RECONCILIATION should treat `DEP-013-02-006` as local source-supported downstream handoff evidence aligned with approved DAG-002 inverse edge `DAG-002-E0768`.
- RECONCILIATION should continue to use `DAG-002` as approved graph authority until a later refreshed graph proposal is explicitly approved.
- The preserved DAG-002 mirror rows contain project-specific dependency types, origins, and satisfaction values that exceed the current narrow enum validator; this is a tooling/schema-alignment warning, not a request to rewrite approved DAG-002 evidence locally.
