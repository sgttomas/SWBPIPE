# Dependencies: DEL-10-05 Headless CLI and structured I/O analysis runner

## Declared Dependency Notes

- No human-declared dependency list was present in the prior container.
- Prior `SYNCHRONIZED_FROM_DAG_002` rows are preserved in `Dependencies.csv` as `RETIRED` rows after enum normalization; active rows below are the conservative dependency-extract refresh surface for RECONCILIATION.

## Extracted Dependency Register

- **Dependencies.csv:** present
- **Register schema:** v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Rows:** 29 total; 16 ACTIVE; 13 RETIRED.
- **ACTIVE ANCHOR rows:** 6
- **ACTIVE EXECUTION rows:** 10
- **Parent anchor:** present (`DEP-10-05-A001`)

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---:|---|---|---|
| DEP-10-05-A001 | ANCHOR | UPSTREAM | OTHER | PKG-10 Build, Packaging, API, and Interoperability | ACTIVE |
| DEP-10-05-A002 | ANCHOR | UPSTREAM | OTHER | SOW-054 Headless command-line or structured I/O analysis runner | ACTIVE |
| DEP-10-05-A003 | ANCHOR | UPSTREAM | OTHER | SOW-032 Reproducible build packaging and CI/CD workflows | ACTIVE |
| DEP-10-05-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-008 Verification, validation, regression testing, and release gates | ACTIVE |
| DEP-10-05-A005 | ANCHOR | UPSTREAM | OTHER | OBJ-009 Interoperability and extensibility with governance boundaries | ACTIVE |
| DEP-10-05-A006 | ANCHOR | UPSTREAM | OTHER | OBJ-012 Unit-safe deterministic and reproducible model data flow | ACTIVE |
| DEP-10-05-E001 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-03 Application service command-query-job model | ACTIVE |
| DEP-10-05-E002 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE |
| DEP-10-05-E003 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-04 Result export format | ACTIVE |
| DEP-10-05-E004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-10-04 Build, packaging, and CI/CD pipeline | ACTIVE |
| DEP-10-05-E005 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE |
| DEP-10-05-E006 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-05 Project persistence and round-trip serialization | ACTIVE |
| DEP-10-05-E007 | EXECUTION | UPSTREAM | INTERFACE | DEL-08-02 Audit manifest and model hash | ACTIVE |
| DEP-10-05-E008 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-06 Solver diagnostics and singularity detection | ACTIVE |
| DEV-001-STAGE2-DEL-10-05-PKG02-001 | EXECUTION | UPSTREAM | SCHEMA_CONTRACT | DEL-02-01 Canonical domain model schema | ACTIVE |
| DEV-001-STAGE2-DEL-10-05-PKG02-003 | EXECUTION | UPSTREAM | DOMAIN_MODEL | DEL-02-03 Code-neutral analysis boundary model | ACTIVE |

## Run Notes

- Defaults recorded: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`.
- Chosen anchor evidence: `_CONTEXT.md`, with decomposition validation from `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Chosen execution evidence order: `Specification.md`, `Guidance.md`, `Datasheet.md`, then contextual cross-checks in `_CONTEXT.md` and `_REFERENCES.md`.
- ACTIVE rows use only v3.1 canonical dependency enums.
- Prior DAG-002 synchronized rows used non-v3.1 values in enum-controlled fields. They were not deleted; they were normalized and retired to preserve history while restoring schema/enum validity.
- No `[WARNING] FLOATING_NODE`: one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] AMBIGUOUS_ANCHOR`: exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- No `[WARNING] MISSING_DECOMPOSITION`: decomposition path was available and read.
- Validation warning: `tools/validation/validate_id_format.sh` expects legacy three-digit package/deliverable patterns (`PKG-###`, `DEL-###-##`) and rejects canonical decomposition IDs used in this project revision such as `PKG-10` and `DEL-10-05`. IDs were preserved from the decomposition.

## Run History

| Timestamp | Mode | Strictness | Consumer | Decomposition | Rows | Warnings |
|---|---|---|---|---|---:|---|
| 2026-04-30 1105 | UPDATE | CONSERVATIVE | NONE | available | 14 ACTIVE | none |
| 2026-05-03 | DAG-002 sync | N/A | coordination | DAG-002 | 13 ACTIVE | non-v3.1 enum vocabulary later detected |
| 2026-05-10 2313 | UPDATE | CONSERVATIVE | RECONCILIATION | available | 14 ACTIVE; 13 RETIRED | ID-format helper pattern mismatch |

## Lifecycle Summary

- ACTIVE: 16
- RETIRED: 13
- `SatisfactionStatus=SATISFIED`: 17
- `SatisfactionStatus=TBD`: 12
- `SatisfactionStatus=PENDING`: 0
- `SatisfactionStatus=IN_PROGRESS`: 0
- `SatisfactionStatus=WAIVED`: 0
- `SatisfactionStatus=NOT_APPLICABLE`: 0

## Downstream Handoff Notes

- For RECONCILIATION, consume ACTIVE rows as the current dependency-extract surface.
- RETIRED `DAG-002-*` rows are retained for audit/history and should not be treated as current blockers.
- Concrete CLI command names, schema fields, public API transport, CI provider, release matrix, external format list, and physical package/container remain `TBD` until later sealed implementation scope or human approval.

## DEV-001 Stage 2 Finding Resolution

- Added package-local active rows `DEV-001-STAGE2-DEL-10-05-PKG02-001` and `DEV-001-STAGE2-DEL-10-05-PKG02-003`.
- These rows explicitly name accepted PKG-02 foundations used by the headless runner: `DEL-02-01` for canonical model references in schema-first runner requests and `DEL-02-03` for analysis-status and human-review/professional-boundary semantics in runner results.
- This resolves the technical dependency-coverage gap identified by `PKG10-DEL1005-PKG02-W001` while preserving the existing active `DEL-02-02` and `DEL-02-05` rows, and without changing aggregate DAG authority, lifecycle state, CLI implementation scope, or human disposition.
