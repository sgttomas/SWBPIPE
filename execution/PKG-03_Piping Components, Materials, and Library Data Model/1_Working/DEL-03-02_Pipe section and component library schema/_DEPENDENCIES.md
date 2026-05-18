# Dependencies: DEL-03-02 Pipe section and component library schema

## Extracted Dependency Register

- **Status:** REFRESHED_FOR_RECONCILIATION
- **Register schema:** `Dependencies.csv` v3.1
- **Local register:** `Dependencies.csv`
- **Rows:** 10 total; 10 ACTIVE; 0 RETIRED.
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Refreshed:** 2026-05-10

| DependencyID | Class | Direction | Type | Target | Satisfaction | Confidence |
|---|---|---|---|---|---|---|
| DAG-002-E0058 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-01 Architecture decision record baseline | SATISFIED | HIGH |
| DAG-002-E0059 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-02 Repository and module boundary architecture | SATISFIED | HIGH |
| DAG-002-E0060 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-04 Persistence and schema versioning architecture | SATISFIED | HIGH |
| DAG-002-E0061 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 Diagnostics, warning, and result-envelope contract | SATISFIED | HIGH |
| DAG-002-E0062 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 API boundary and adapter contract map | SATISFIED | HIGH |
| DAG-002-E0063 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-08 Layered software test and acceptance strategy | SATISFIED | HIGH |
| DAG-002-E0405 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-01 Canonical domain model schema | TBD | HIGH |
| DAG-002-E0406 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 Unit system and dimensional-analysis core contract | TBD | HIGH |
| DAG-002-E0407 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-02 Copyright and protected-data boundary policy | TBD | HIGH |
| DAG-002-E0408 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-03 Contributor certification workflow | TBD | HIGH |

## Run Notes

- Runtime overrides recorded: `SCOPE=DEL-03-02`, `RUN_ROOT=/Users/ryan/ai-env/projects/chirality-piping/execution`, `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.
- Read boundary used: assigned deliverable folder, root governance contract/agent index, dependency-extract skill, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- The register preserves the 10 existing dependency IDs and row set, with `LastSeen` refreshed to 2026-05-10.
- Enum refresh: local v3.1 validator accepts `DependencyType` values only from `PREREQUISITE`, `INTERFACE`, `HANDOVER`, `CONSTRAINT`, `ENABLES`, and `OTHER`; architecture-basis edges were normalized to `CONSTRAINT`, and schema/unit/governance predecessor edges were normalized to `PREREQUISITE`.
- Enum refresh: `AnchorType=DELIVERABLE` was normalized to `AnchorType=NOT_APPLICABLE` for EXECUTION rows; `Origin=CONTEXT/DECOMPOSITION` was normalized to `Origin=EXTRACTED`; `Explicitness=INFERRED_DIRECT` was normalized to `IMPLICIT`; `SatisfactionStatus=UNKNOWN` was normalized to `TBD`.
- No ANCHOR rows were added in this refresh because the assigned TP-DAG-004 row is a conservative update to the existing execution dependency surface for RECONCILIATION.
- No source documents, status files, memory files, code, schemas, tests, DAG files, or coordination files were edited.

## Run History

| Timestamp | Mode | Strictness | Decomposition path | Warnings | ACTIVE rows |
|---|---|---|---|---|---:|
| 2026-04-30T00:04:00-0600 | UPDATE | CONSERVATIVE | `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md` | ID helper used legacy three-digit formats; assigned software IDs preserved. | 5 |
| 2026-05-10T21:56:42-0600 | UPDATE | CONSERVATIVE | `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` | Current register parsed before refresh but required enum normalization for v3.1 validator compatibility. | 10 |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 10 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 6 |
| TBD | 4 |

## Downstream Handoff Notes

- For RECONCILIATION, treat this file as a deliverable-local dependency evidence surface, not as independent graph authority.
- Six architecture-basis constraints remain satisfied context injections from `AB-00-01`, `AB-00-02`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
- Four predecessor edges remain unresolved at this local surface (`TBD`) because this bounded worker did not inspect or adjudicate target deliverable maturity.
