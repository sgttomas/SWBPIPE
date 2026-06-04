# Dependencies: DEL-11-04 Invented educational example models

## Declared Dependency Notes

- No human-declared dependency list was present in the prior container.
- Prior `SYNCHRONIZED_FROM_DAG_002` rows are preserved as ACTIVE rows because `DAG-002` is the approved graph authority for this refresh. Legacy mirror enum values were normalized to the v3.1 register vocabulary in `Dependencies.csv`.
- Additional ACTIVE rows were added only where DEL-11-04 source documents explicitly identified anchors or prerequisites.

## Extracted Dependency Register

- **Dependencies.csv:** present
- **Register schema:** v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Rows:** 17 total; 17 ACTIVE; 0 CANDIDATE; 0 RETIRED.
- **ACTIVE ANCHOR rows:** 4
- **ACTIVE EXECUTION rows:** 13
- **Parent anchor:** present (`DEP-11-04-A001`)

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---:|---|---|---|
| DEP-11-04-A001 | ANCHOR | UPSTREAM | OTHER | PKG-11 Documentation, Examples, and Education | ACTIVE |
| DEP-11-04-A002 | ANCHOR | UPSTREAM | OTHER | SOW-033 User/developer documentation and invented-data examples for education and testing | ACTIVE |
| DEP-11-04-A003 | ANCHOR | UPSTREAM | OTHER | OBJ-001 Open auditable piping stress analysis platform | ACTIVE |
| DEP-11-04-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-008 Verification validation regression testing and release gates | ACTIVE |
| DEP-11-04-E001 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | ACTIVE |
| DEP-11-04-E002 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-05 Project persistence and round-trip serialization | ACTIVE |
| DEP-11-04-E003 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-01 Rule-pack schema | ACTIVE |
| DAG-002-E0344 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 Architecture decision record baseline | ACTIVE |
| DAG-002-E0345 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 Repository and module boundary architecture | ACTIVE |
| DAG-002-E0346 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE |
| DAG-002-E0347 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 API boundary and adapter contract map | ACTIVE |
| DAG-002-E0348 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 Layered software test and acceptance strategy | ACTIVE |
| DAG-002-E0590 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-05 Invented non-code example rule pack | ACTIVE |
| DAG-002-E0591 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-09-01 Mechanics benchmark suite | ACTIVE |
| DAG-002-E0592 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-09-02 Stress recovery benchmark suite | ACTIVE |
| DAG-002-E0593 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-08-05 Report protected-content linter | ACTIVE |
| DAG-002-E0594 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-02 Copyright and protected-data boundary policy | ACTIVE |

## Run Notes

- Defaults recorded: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.
- Chosen anchor evidence: `_CONTEXT.md` and `Datasheet.md`, with decomposition validation from `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Chosen execution evidence order: `Specification.md`, `Procedure.md`, `Guidance.md`, `Datasheet.md`, then contextual cross-checks in `_CONTEXT.md` and `_REFERENCES.md`.
- `DAG-002` mirror rows are retained as ACTIVE local historical evidence; current graph authority is `DAG-006`, and this local refresh does not approve, promote, or modify any aggregate DAG artifact.
- Legacy mirror values were normalized on write: `AnchorType=DELIVERABLE` became `NOT_APPLICABLE`; `ARCHITECTURE_BASIS` became `CONSTRAINT`; `DOCS_PREDECESSOR` and `GOVERNANCE_PREDECESSOR` became `PREREQUISITE`; `Origin=CONTEXT/DECOMPOSITION` became `EXTRACTED`; `SatisfactionStatus=UNKNOWN` became `TBD`; `Explicitness=INFERRED_DIRECT` became `IMPLICIT`.
- Added prerequisites for `DEL-02-01`, `DEL-02-05`, and `DEL-06-01` because `Specification.md` explicitly identifies model/schema, persistence, and rule-pack schema inputs before future example materialization.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] MISSING_DECOMPOSITION`: decomposition path was available and read.
- Validation warning: `tools/validation/validate_id_format.sh` expects legacy three-digit package/deliverable patterns (`PKG-###`, `DEL-###-##`) and rejects canonical decomposition IDs used in this project revision such as `PKG-11` and `DEL-11-04`. IDs were preserved from the decomposition.

## Run History

| Timestamp | Mode | Strictness | Consumer | Decomposition | Rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-04-30 1119 | UPDATE | CONSERVATIVE | NONE | available | dependency extract completed | none recorded in summary |
| 2026-05-03 | DAG-002 sync | N/A | coordination | DAG-002 | 10 ACTIVE | local surface was synchronized mirror |
| 2026-05-10 2323 | UPDATE | CONSERVATIVE | RECONCILIATION | available | 17 ACTIVE | ID-format helper pattern mismatch |

## Lifecycle Summary

- ACTIVE: 17
- CANDIDATE: 0
- RETIRED: 0
- `SatisfactionStatus=SATISFIED`: 9
- `SatisfactionStatus=TBD`: 8
- `SatisfactionStatus=PENDING`: 0
- `SatisfactionStatus=IN_PROGRESS`: 0
- `SatisfactionStatus=WAIVED`: 0
- `SatisfactionStatus=NOT_APPLICABLE`: 0

## Downstream Handoff Notes

- For RECONCILIATION, consume ACTIVE rows as the refreshed local dependency surface for DEL-11-04.
- `DAG-002-*` row IDs remain active because they mirror approved `DAG-006` authority and were not contradicted by local source documents; their enum fields were normalized locally for v3.1 validation.
- New `DEP-11-04-*` rows represent local evidence not present in the prior mirror, especially Tree anchors and explicit schema prerequisites.
- Future model format, fake-rule-pack schema details, validation-fixture promotion, and public tutorial materialization remain `TBD` until later sealed implementation scope or human approval.
