# Dependencies: DEL-02-04 Plugin and extension domain contracts

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Schema:** Dependencies.csv v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Rows:** 12 total; 9 ACTIVE; 3 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG artifacts remain outside this deliverable-local write scope.
- This local register is an evidence-first dependency surface for reconciliation, not an independent project graph authority.
- Retired rows are preserved for auditability and are not deleted.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Declared Dependencies

No human-owned declared dependency list was present in the prior `_DEPENDENCIES.md`.

## Extracted Dependency Register

| DependencyID | Class | Status | Direction | Type | Target | Statement |
|---|---|---:|---|---|---|---|
| DEP-02-04-A001 | ANCHOR | ACTIVE | UPSTREAM | OTHER | SOW-038 | DEL-02-04 implements scope item SOW-038 for plugin and extension domain contracts. |
| DEP-02-04-A002 | ANCHOR | ACTIVE | UPSTREAM | OTHER | OBJ-009 | DEL-02-04 traces to objective OBJ-009 for interoperability and extensibility while preserving governance boundaries. |
| DAG-002-E0038 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-01 | DEL-02-04 uses SCA-001 architecture basis AB-00-01 from DEL-00-01 before product-development execution. |
| DAG-002-E0039 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-02 | DEL-02-04 uses SCA-001 architecture basis AB-00-02 from DEL-00-02 before product-development execution. |
| DAG-002-E0040 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-03 | DEL-02-04 uses SCA-001 architecture basis AB-00-03 from DEL-00-03 before product-development execution. |
| DAG-002-E0041 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-04 | DEL-02-04 uses SCA-001 architecture basis AB-00-04 from DEL-00-04 before product-development execution. |
| DAG-002-E0042 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-06 | DEL-02-04 uses SCA-001 architecture basis AB-00-06 from DEL-00-06 before product-development execution. |
| DAG-002-E0043 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-07 | DEL-02-04 uses SCA-001 architecture basis AB-00-07 from DEL-00-07 before product-development execution. |
| DAG-002-E0044 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-08 | DEL-02-04 uses SCA-001 architecture basis AB-00-08 from DEL-00-08 before product-development execution. |
| DAG-002-E0395 | EXECUTION | RETIRED | UPSTREAM | PREREQUISITE | DEL-02-01 | Current Procedure evidence says the document kit does not add the assumed DEL-02-01 dependency. |
| DAG-002-E0396 | EXECUTION | RETIRED | UPSTREAM | PREREQUISITE | DEL-02-02 | Current Procedure evidence says the document kit does not add the assumed DEL-02-02 dependency. |
| DAG-002-E0397 | EXECUTION | RETIRED | UPSTREAM | PREREQUISITE | DEL-02-03 | Conservative refresh found no explicit DEL-02-03 execution prerequisite in the source kit. |

## Run Notes

- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Anchor document selected: `Datasheet.md`.
- Execution documents scanned: `_CONTEXT.md`, `Procedure.md`, `Specification.md`, `Guidance.md`, `_REFERENCES.md`, prior `_DEPENDENCIES.md`, and existing `Dependencies.csv`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: found and used for SOW/OBJ/deliverable label validation.
- Parent anchor check: exactly one ACTIVE `IMPLEMENTS_NODE` anchor found.
- Warnings: `_REFERENCES.md` still calls the decomposition basis accepted v0.2 while `_CONTEXT.md` and the active decomposition identify revision 0.4/current basis. Source cleanup is outside this run's write scope.
- Validation tool warning: `tools/validation/validate_id_format.sh` rejects canonical active decomposition IDs such as `DEL-02-04` because it expects the legacy pattern `DEL-[0-9]{3}-[0-9]{2}`. Canonical IDs were preserved; no source IDs were rewritten.
- Failed-input notes: none.

## Run History

- 2026-05-10 21:49 MDT — MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition found at `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; ACTIVE counts: ANCHOR=2, EXECUTION=7; RETIRED counts: EXECUTION=3; warnings: reference revision wording conflict and legacy ID-format validator mismatch.

## Lifecycle Summary

| Class | ACTIVE | RETIRED |
|---|---:|---:|
| ANCHOR | 2 | 0 |
| EXECUTION | 7 | 3 |
| TOTAL | 9 | 3 |

| SatisfactionStatus | ACTIVE | RETIRED |
|---|---:|---:|
| NOT_APPLICABLE | 2 | 0 |
| SATISFIED | 7 | 0 |
| TBD | 0 | 3 |

## Downstream Handoff Notes

- For RECONCILIATION: the refreshed local surface differs from the prior DAG-002 mirror by adding explicit Tree anchors and retiring three direct PKG-02 predecessor rows that are not supported as current execution dependencies by the DEL-02-04 source kit under conservative extraction.
- Reconciliation should compare retired rows `DAG-002-E0395`, `DAG-002-E0396`, and `DAG-002-E0397` against aggregate DAG authority before changing any sequencing artifact.
