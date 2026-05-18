# Dependencies: DEL-02-02 Unit system and dimensional-analysis core contract

## Generated Dependency Register

- **Status:** REFRESHED_TP_DAG_004
- **Source of Truth:** Local `Dependencies.csv` v3.1 for this deliverable-local evidence surface.
- **Prior Source:** `execution/_DAG/DAG-002/DependencyEdges.csv` synchronized mirror rows preserved as declared rows.
- **Local Register:** `Dependencies.csv`
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary

- Aggregate DAG artifacts remain external sequencing and blocker-computation authority within their approval boundary.
- This local register is a deliverable-local evidence surface, not an independent aggregate graph authority.
- Declared DAG mirror rows are preserved and normalized to the v3.1 enum surface.
- Extracted rows are evidence-backed from DEL-02-02 local source documents and are intended for downstream RECONCILIATION review.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

| DependencyID | Class | Direction | Type | TargetType | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|---|
| DAG-002-E0024 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-01 Architecture decision record baseline | ACTIVE | SATISFIED |
| DAG-002-E0025 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-02 Repository and module boundary architecture | ACTIVE | SATISFIED |
| DAG-002-E0026 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-03 Application service command-query-job model | ACTIVE | SATISFIED |
| DAG-002-E0027 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | SATISFIED |
| DAG-002-E0028 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | SATISFIED |
| DAG-002-E0029 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-07 API boundary and adapter contract map | ACTIVE | SATISFIED |
| DAG-002-E0030 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | SATISFIED |
| DAG-002-E0393 | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | DEL-02-01 Canonical domain model schema | ACTIVE | TBD |
| DEL-02-02-DEP-001 | ANCHOR | UPSTREAM | OTHER | WBS_NODE | SOW-025 | ACTIVE | SATISFIED |
| DEL-02-02-DEP-002 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-001 | ACTIVE | SATISFIED |
| DEL-02-02-DEP-003 | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | OBJ-012 | ACTIVE | SATISFIED |
| DEL-02-02-DEP-004 | EXECUTION | UPSTREAM | CONSTRAINT | EXTERNAL | Human decision owner or review gate | ACTIVE | PENDING |
| DEL-02-02-DEP-005 | EXECUTION | UPSTREAM | CONSTRAINT | DOCUMENT | docs/IP_AND_DATA_BOUNDARY.md | ACTIVE | SATISFIED |
| DEL-02-02-DEP-006 | EXECUTION | UPSTREAM | PREREQUISITE | DOCUMENT | docs/VALIDATION_STRATEGY.md | ACTIVE | SATISFIED |
| DEL-02-02-DEP-007 | EXECUTION | DOWNSTREAM | ENABLES | UNKNOWN | downstream unit consumers | ACTIVE | NOT_APPLICABLE |
| DEL-02-02-DEP-008 | EXECUTION | DOWNSTREAM | HANDOVER | DOCUMENT | core/units module contract | ACTIVE | PENDING |
| DEL-02-02-DEP-009 | EXECUTION | DOWNSTREAM | HANDOVER | DOCUMENT | docs/SPEC.md unit-system section | ACTIVE | PENDING |

## Run Notes

- MODE: `UPDATE`.
- STRICTNESS: `CONSERVATIVE`.
- CONSUMER_CONTEXT: `RECONCILIATION`.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for SOW/OBJ/DEL label validation.
- Source document selection: `SOURCE_DOCS=AUTO`.
- Anchor doc selected: `Datasheet.md`.
- Execution docs selected: `Procedure.md`, `Specification.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`.
- Existing DAG-002 mirror rows were preserved as `Origin=DECLARED` and normalized from legacy local values to v3.1 enums.
- No extracted rows were retired; no prior `Origin=EXTRACTED` rows were present to retire.
- `[WARNING] TARGET_UNRESOLVED`: `DEL-02-02-DEP-004` uses `TargetType=EXTERNAL` and `TargetLocation=TBD` because the local sources require a human review gate but do not name a stable authority.
- `[WARNING] TARGET_UNRESOLVED`: `DEL-02-02-DEP-007` uses `TargetType=UNKNOWN` because downstream consumer classes are explicit but stable deliverable IDs are not named in the local source.
- `[WARNING] TOOL_ID_FORMAT_CONTRACT`: `tools/validation/validate_id_format.sh` rejected canonical project IDs `DEL-02-02` and `PKG-02` because the helper expects `DEL-[0-9]{3}-[0-9]{2}` and `PKG-[0-9]{3}`. The register preserves the established decomposition IDs.

## Run History

- 2026-05-03: Synchronized 8 ACTIVE rows from `execution/_DAG/DAG-002/DependencyEdges.csv`.
- 2026-05-10 21:48 MDT: TP-DAG-004 dependency-extract refresh, MODE=`UPDATE`, STRICTNESS=`CONSERVATIVE`, CONSUMER_CONTEXT=`RECONCILIATION`; decomposition located; 17 ACTIVE rows after refresh; warnings: `TARGET_UNRESOLVED`, `TOOL_ID_FORMAT_CONTRACT`.

## Lifecycle Summary

| Category | Count |
|---|---:|
| ACTIVE | 17 |
| RETIRED | 0 |
| ANCHOR | 3 |
| EXECUTION | 14 |
| DECLARED | 8 |
| EXTRACTED | 9 |
| SATISFIED | 12 |
| TBD | 1 |
| PENDING | 3 |
| NOT_APPLICABLE | 1 |

## Downstream Handoff Notes

- For RECONCILIATION: distinguish preserved declared DAG mirror rows from newly extracted local evidence rows by `Origin`.
- Review whether the downstream consumer class row `DEL-02-02-DEP-007` should be split into stable deliverable-level edges by a later aggregate/reconciliation pass.
- Review whether `DAG-002-E0393` should remain `SatisfactionStatus=TBD` until DEL-02-01 maturity/acceptance is confirmed by an authoritative coordination artifact.
- Treat the ID-format validator mismatch as a tool/schema alignment issue, not as a deliverable ID change request.
