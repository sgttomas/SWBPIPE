# Dependencies: DEL-13-03 Constraint validation engine

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Schema:** Dependencies.csv v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Source of Truth Boundary:** `execution/_DAG/DAG-002/` remains the approved graph authority.
- **Local Register:** `Dependencies.csv`
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG artifacts remain outside this deliverable-local write scope.
- This local register is an evidence-first dependency surface for reconciliation, not an independent project graph authority.
- `DAG-003` is not approved, promoted, or used as graph authority by this refresh.
- Retired rows, if later created, must be preserved for auditability and not deleted.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Declared Dependencies

No human-owned declared dependency list was present in the prior `_DEPENDENCIES.md`.

## Extracted Dependency Register

| DependencyID | Class | Status | Direction | Type | Target | Statement |
|---|---|---:|---|---|---|---|
| DEP-13-03-A001 | ANCHOR | ACTIVE | UPSTREAM | OTHER | SOW-068 | DEL-13-03 implements scope item SOW-068 for validating available design knowledge and constraints with provenance-aware messages. |
| DEP-13-03-A002 | ANCHOR | ACTIVE | UPSTREAM | OTHER | OBJ-014 | DEL-13-03 traces to objective OBJ-014 by contributing constraint validation to the schema-backed physical design model and transformation traceability surface. |
| DAG-002-E0653 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-01 | DEL-13-03 uses SCA-001 architecture basis AB-00-01 from DEL-00-01 before product-development execution. |
| DAG-002-E0654 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-02 | DEL-13-03 uses SCA-001 architecture basis AB-00-02 from DEL-00-02 before product-development execution. |
| DAG-002-E0655 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-03 | DEL-13-03 uses SCA-001 architecture basis AB-00-03 from DEL-00-03 before product-development execution. |
| DAG-002-E0656 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-04 | DEL-13-03 uses SCA-001 architecture basis AB-00-04 from DEL-00-04 before product-development execution. |
| DAG-002-E0657 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-06 | DEL-13-03 uses SCA-001 architecture basis AB-00-06 from DEL-00-06 before product-development execution. |
| DAG-002-E0658 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-07 | DEL-13-03 uses SCA-001 architecture basis AB-00-07 from DEL-00-07 before product-development execution. |
| DAG-002-E0659 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-00-08 | DEL-13-03 uses SCA-001 architecture basis AB-00-08 from DEL-00-08 before product-development execution. |
| DAG-002-E0767 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-13-01 | DEL-13-03 needs DEL-13-01 as a predecessor because constraint validation consumes available design-knowledge schema and provenance inputs. |
| DAG-002-E0768 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-13-02 | DEL-13-03 needs DEL-13-02 as a predecessor because constraint validation consumes constraint entity and provenance inputs. |
| DAG-002-E0769 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-02-02 | DEL-13-03 needs DEL-02-02 as a predecessor because constraint validation must remain unit-aware and dimensionally checked. |
| DAG-002-E0770 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-04-06 | DEL-13-03 needs DEL-04-06 as a predecessor because deterministic validation messages need diagnostic and warning semantics. |
| DAG-002-E0771 | EXECUTION | ACTIVE | UPSTREAM | PREREQUISITE | DEL-02-05 | DEL-13-03 needs DEL-02-05 as a predecessor because provenance-aware validation needs persisted project data and round-trip serialization context. |

## Run Notes

- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Anchor document selected: `Datasheet.md`.
- Execution documents scanned: `_CONTEXT.md`, `Procedure.md`, `Specification.md`, `Guidance.md`, `_REFERENCES.md`, prior `_DEPENDENCIES.md`, and existing `Dependencies.csv`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: found and used for SOW/OBJ/deliverable label validation.
- Approved graph authority read: `/Users/ryan/ai-env/projects/chirality-piping/execution/_DAG/DAG-002/`; status: used only to preserve prior approved mirror rows and evidence boundary.
- Parent anchor check: exactly one ACTIVE `IMPLEMENTS_NODE` anchor found.
- Conservative row treatment: preserved all 12 approved DAG-002 mirror execution rows as ACTIVE because DEL-13-03 source documents explicitly instruct later implementation to treat approved ACTIVE dependency rows as predecessor evidence and not retire, delete, or reclassify them during this setup workflow.
- Canonicalization: local mirror rows were normalized to dependency-extract enum values (`AnchorType=NOT_APPLICABLE`, `DependencyType=PREREQUISITE`, `Origin=EXTRACTED`, `SatisfactionStatus=TBD` where prior mirror value was `UNKNOWN`) while preserving the source `DependencyID` values.
- Validation tool warning: `tools/validation/validate_id_format.sh` rejects canonical active decomposition IDs such as `DEL-13-03`, `PKG-13`, and `SOW-068` because it expects legacy patterns. Canonical IDs were preserved; no source IDs were rewritten.
- Failed-input notes: none.

## Run History

- 2026-05-10 23:37 MDT — MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition found at `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; graph authority read from `/Users/ryan/ai-env/projects/chirality-piping/execution/_DAG/DAG-002/`; ACTIVE counts: ANCHOR=2, EXECUTION=12; RETIRED counts: none; warnings: legacy ID-format validator mismatch.

## Lifecycle Summary

| Class | ACTIVE | RETIRED |
|---|---:|---:|
| ANCHOR | 2 | 0 |
| EXECUTION | 12 | 0 |
| TOTAL | 14 | 0 |

| DependencyType | ACTIVE | RETIRED |
|---|---:|---:|
| OTHER | 2 | 0 |
| PREREQUISITE | 12 | 0 |

| TargetType | ACTIVE | RETIRED |
|---|---:|---:|
| DELIVERABLE | 12 | 0 |
| REQUIREMENT | 1 | 0 |
| WBS_NODE | 1 | 0 |

| SatisfactionStatus | ACTIVE | RETIRED |
|---|---:|---:|
| NOT_APPLICABLE | 2 | 0 |
| SATISFIED | 7 | 0 |
| TBD | 5 | 0 |

## Downstream Handoff Notes

- For RECONCILIATION: the refreshed local surface differs from the prior DAG-002 mirror by adding explicit Tree anchors for `SOW-068` and `OBJ-014` and by canonicalizing enum fields for the 12 preserved execution rows.
- No rows were retired in this refresh; no candidate rows were promoted; no aggregate DAG file was changed.
- Reconciliation should compare the preserved execution row semantics against aggregate graph authority before changing any sequencing artifact.
