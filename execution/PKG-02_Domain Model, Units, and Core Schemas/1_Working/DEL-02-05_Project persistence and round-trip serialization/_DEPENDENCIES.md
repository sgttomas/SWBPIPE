# Dependencies: DEL-02-05 Project persistence and round-trip serialization

## Generated Dependency Register

- **Status:** REFRESHED_BY_DEPENDENCY_EXTRACT
- **Register schema:** `Dependencies.csv` v3.1
- **Source of truth:** deliverable-local extraction surface for `DEL-02-05`; aggregate DAG artifacts remain downstream/coordination artifacts.
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary

- This local register is an evidence surface for downstream RECONCILIATION, not an independent project graph authority.
- Aggregate DAG artifacts, blocker queues, and coordination files were not edited by this TASK run.
- Architecture-basis rows are preserved as extracted execution prerequisites from sealed context evidence; they do not mark `PKG-00` as `ISSUED`.
- No protected standards text, protected tables, proprietary values, engineering defaults, legal conclusions, or professional approval claims were inferred.

## Extracted Dependency Register

### Counts

| Class | Status | Count |
|---|---:|---:|
| ANCHOR | ACTIVE | 3 |
| EXECUTION | ACTIVE | 10 |

### Compact Table

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DEP-02-05-A001 | ANCHOR | UPSTREAM | OTHER | `PKG-02` Domain Model, Units, and Core Schemas | ACTIVE | NOT_APPLICABLE |
| DEP-02-05-A002 | ANCHOR | UPSTREAM | OTHER | `SOW-050` Project persistence and deterministic round-trip serialization | ACTIVE | NOT_APPLICABLE |
| DEP-02-05-A003 | ANCHOR | UPSTREAM | OTHER | `SOW-041` Machine-readable project and model schemas | ACTIVE | NOT_APPLICABLE |
| DAG-002-E0045 | EXECUTION | UPSTREAM | PREREQUISITE | `AB-00-01` / `DEL-00-01` Architecture decision record baseline | ACTIVE | SATISFIED |
| DAG-002-E0046 | EXECUTION | UPSTREAM | PREREQUISITE | `AB-00-02` / `DEL-00-02` Repository and module boundary architecture | ACTIVE | SATISFIED |
| DAG-002-E0047 | EXECUTION | UPSTREAM | PREREQUISITE | `AB-00-03` / `DEL-00-03` Application service command-query-job model | ACTIVE | SATISFIED |
| DAG-002-E0048 | EXECUTION | UPSTREAM | PREREQUISITE | `AB-00-04` / `DEL-00-04` Persistence and schema versioning architecture | ACTIVE | SATISFIED |
| DAG-002-E0049 | EXECUTION | UPSTREAM | PREREQUISITE | `AB-00-06` / `DEL-00-06` Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED |
| DAG-002-E0050 | EXECUTION | UPSTREAM | PREREQUISITE | `AB-00-07` / `DEL-00-07` API boundary and adapter contract map | ACTIVE | SATISFIED |
| DAG-002-E0051 | EXECUTION | UPSTREAM | PREREQUISITE | `AB-00-08` / `DEL-00-08` Layered software test and acceptance strategy | ACTIVE | SATISFIED |
| DAG-002-E0398 | EXECUTION | UPSTREAM | INTERFACE | `DEL-02-01` Canonical domain model schema | ACTIVE | TBD |
| DAG-002-E0399 | EXECUTION | UPSTREAM | INTERFACE | `DEL-02-02` Unit system and dimensional-analysis core contract | ACTIVE | TBD |
| DAG-002-E0400 | EXECUTION | UPSTREAM | INTERFACE | `DEL-02-03` Code-neutral analysis boundary model | ACTIVE | TBD |

## Run Notes

- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer context:** `RECONCILIATION`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition status:** located and used for anchor/target label validation.
- **Source docs:** `AUTO`; scanned deliverable-local `_CONTEXT.md`, `_REFERENCES.md`, `Datasheet.md`, `Procedure.md`, `Specification.md`, and `Guidance.md`.
- **Anchor doc:** `Datasheet.md` selected by explicit identification, scope item, and package fields.
- **Execution doc order:** `_CONTEXT.md`, `Procedure.md`, `Specification.md`, `Datasheet.md`, `Guidance.md`.
- **Update action:** prior aggregate mirror rows were normalized to v3.1 enum vocabulary where evidence-supported; no prior row was deleted.
- **Enum normalization:** `AnchorType=DELIVERABLE` became `NOT_APPLICABLE` for execution rows; custom execution dependency types became `PREREQUISITE` or `INTERFACE`; `Origin=CONTEXT/DECOMPOSITION` became `EXTRACTED`; `Explicitness=INFERRED_DIRECT` became `IMPLICIT`; `SatisfactionStatus=UNKNOWN` became `TBD`.
- **Integrity warnings:** none for Tree x DAG anchors. Exactly one ACTIVE `IMPLEMENTS_NODE` parent anchor is present.
- **Failed-input note:** `tools/validation/validate_id_format.sh` appears to use older ID regexes and rejects current project IDs such as `DEL-02-05`, `PKG-02`, and `SOW-050`; enum and schema validation passed.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE counts |
|---|---|---|---|---|---|
| 2026-05-10 21:48 America/Edmonton | UPDATE | CONSERVATIVE | located: `execution/_Decomposition/SOFTWARE_DECOMP.md` | ID-format helper regex mismatch for current IDs | ANCHOR=3; EXECUTION=10 |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 13 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 3 |
| SATISFIED | 7 |
| TBD | 3 |

## Downstream Handoff Notes

- For RECONCILIATION, compare the retained `DAG-002-*` IDs against current aggregate DAG lineage before treating them as aggregate-authoritative.
- The three PKG-02 interface rows are execution-surface dependencies, not proof that target deliverables are complete.
- The ID-format validation helper mismatch should be resolved outside this deliverable before using that helper as a hard gate on current project IDs.
