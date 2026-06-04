# Dependencies: DEL-10-02 Import/export adapter framework

## Declared Dependency Notes

- No separate `Origin=DECLARED` rows were present in the prior local register.
- Prior DAG mirror rows were re-extracted into canonical dependency-extract v3.1 enum values where supported by assigned deliverable evidence.

## Extracted Dependency Register

- **Schema:** `Dependencies.csv` v3.1
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Rows:** 18 total; 18 ACTIVE; 0 RETIRED.
- **Classes:** 5 ANCHOR; 13 EXECUTION.
- **Directions:** 17 UPSTREAM; 1 DOWNSTREAM.
- **Confidence:** 14 HIGH; 3 MEDIUM; 1 LOW.

| DependencyID | Class | Direction | Type | Target | Confidence | Status |
|---|---|---:|---|---|---|---|
| DEP-010-02-001 | ANCHOR | UPSTREAM | OTHER | SOW-030 | HIGH | ACTIVE |
| DEP-010-02-002 | ANCHOR | UPSTREAM | OTHER | REQ-10-02-01 | HIGH | ACTIVE |
| DEP-010-02-003 | ANCHOR | UPSTREAM | OTHER | REQ-10-02-02 | HIGH | ACTIVE |
| DEP-010-02-004 | ANCHOR | UPSTREAM | OTHER | REQ-10-02-03 | HIGH | ACTIVE |
| DEP-010-02-005 | ANCHOR | UPSTREAM | OTHER | REQ-10-02-05 | HIGH | ACTIVE |
| DEP-010-02-006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 | HIGH | ACTIVE |
| DEP-010-02-007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 | HIGH | ACTIVE |
| DEP-010-02-008 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-06 | HIGH | ACTIVE |
| DEP-010-02-009 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-07 | HIGH | ACTIVE |
| DEP-010-02-010 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | HIGH | ACTIVE |
| DEP-010-02-011 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-04 | MEDIUM | ACTIVE |
| DEP-010-02-012 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-07 | MEDIUM | ACTIVE |
| DEP-010-02-013 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-12-01 | MEDIUM | ACTIVE |
| DEP-010-02-014 | EXECUTION | DOWNSTREAM | ENABLES | DEL-10-03 | LOW | ACTIVE |
| DEV-001-STAGE2-DEL-10-02-PKG02-001 | EXECUTION | UPSTREAM | SCHEMA_CONTRACT | DEL-02-01 | HIGH | ACTIVE |
| DEV-001-STAGE2-DEL-10-02-PKG02-002 | EXECUTION | UPSTREAM | UNIT_CONTRACT | DEL-02-02 | HIGH | ACTIVE |
| DEV-001-STAGE2-DEL-10-02-PKG02-003 | EXECUTION | UPSTREAM | DOMAIN_MODEL | DEL-02-03 | HIGH | ACTIVE |
| DEV-001-STAGE2-DEL-10-02-PKG02-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-05 | HIGH | ACTIVE |

## Run Notes

- Source documents scanned: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`.
- Anchor document selected by AUTO heuristic: `Datasheet.md`.
- Execution document order selected by AUTO heuristic: `Procedure.md`, `Guidance.md`, `Specification.md`, `_CONTEXT.md`, `_REFERENCES.md`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Decomposition status: available; DEL-10-02, SOW-030, and target deliverable IDs were validated against revision 0.7.
- UPDATE behavior: prior local DAG mirror rows were not preserved verbatim because several fields used non-v3.1 enum values. Their supported intent was re-extracted into canonical enum rows.
- Warning: prior architecture-basis rows for DEL-00-01 and DEL-00-02 were not re-emitted as execution dependencies because assigned DEL-10-02 evidence did not state a concrete adapter-framework execution need beyond general architecture-basis applicability.
- Warning: target deliverables DEL-02-04, DEL-03-07, DEL-12-01, and DEL-10-03 are decomposition-resolved from assigned-source obligations rather than explicitly named in the assigned deliverable text; these rows use MEDIUM or LOW confidence for RECONCILIATION review.
- Warning: downstream DEL-10-03 edge is a low-confidence enables candidate because concrete external formats and handoff-specific contracts remain TBD.
- Warning: `tools/validation/validate_id_format.sh` rejects current decomposition IDs such as `PKG-10`, `DEL-10-02`, and `SOW-030` because its patterns expect older three-digit package/deliverable and four-digit SOW forms; canonical decomposition IDs were retained.

## Run History

- 2026-05-10 23:12 MDT — TP-DAG-004 dependency-extract refresh; MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition available at `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; ACTIVE counts: 5 ANCHOR, 9 EXECUTION; warnings: 4.

## Lifecycle Summary

- **ACTIVE:** 14
- **RETIRED:** 0
- **Satisfaction:** 5 NOT_APPLICABLE; 5 SATISFIED; 4 TBD.
- **Required maturity:** 14 SEMANTIC_READY.
- **Proposed maturity:** 14 SEMANTIC_READY.

## Downstream Handoff Notes

- RECONCILIATION should review the MEDIUM/LOW confidence decomposition-resolved rows before treating them as gating dependencies.
- RECONCILIATION should compare the omitted DEL-00-01 and DEL-00-02 architecture-basis mirror rows against aggregate DAG policy; they remain applicable basis context but were not emitted as local execution blockers in this conservative extraction.

## DEV-001 Stage 2 Finding Resolution

- Added package-local active rows `DEV-001-STAGE2-DEL-10-02-PKG02-001`, `DEV-001-STAGE2-DEL-10-02-PKG02-002`, `DEV-001-STAGE2-DEL-10-02-PKG02-003`, and `DEV-001-STAGE2-DEL-10-02-PKG02-005`.
- These rows explicitly name accepted PKG-02 foundations used by the adapter framework: `DEL-02-01` for model import/export schema records, `DEL-02-02` for unit validation, `DEL-02-03` for human-review and status/authority boundaries, and `DEL-02-05` for checksum/audit/persistence-hash controls.
- This resolves the technical dependency-coverage gap identified by `PKG10-DEL1002-PKG02-W001` without changing aggregate DAG authority, lifecycle state, external format support, runtime implementation scope, or human disposition.
