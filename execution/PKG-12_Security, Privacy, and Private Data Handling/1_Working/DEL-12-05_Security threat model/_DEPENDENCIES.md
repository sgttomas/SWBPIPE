# Dependencies: DEL-12-05 Security threat model

## Generated Dependency Register
- **Status:** REFRESHED_FOR_TP_DAG_004
- **Source of Truth:** `Dependencies.csv`
- **Graph Authority Context:** `execution/_DAG/DAG-006/` remains the approved aggregate graph authority. Historical `DAG-003` material was not used for promotion.
- **Local Register:** `Dependencies.csv`
- **Rows:** 23 total; 21 ACTIVE; 2 CANDIDATE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a refreshed deliverable-local evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Extracted Dependency Register

### Counts

| Field | Counts |
|---|---|
| Status | ACTIVE 21; CANDIDATE 2; RETIRED 0 |
| DependencyClass | ANCHOR 2; EXECUTION 21 |
| Direction | UPSTREAM 19; DOWNSTREAM 4 |
| DependencyType | ARCHITECTURE_BASIS 7; SECURITY_PREDECESSOR 4; CONSTRAINT 5; PREREQUISITE 1; ENABLES 4; OTHER 2 |
| TargetType | DELIVERABLE 15; DOCUMENT 6; WBS_NODE 1; REQUIREMENT 1 |

### Compact Table

| DependencyID | Class | Direction | Type | Target | Status | Notes |
|---|---|---:|---|---|---|---|
| DAG-002-E0382 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-01 Architecture decision record baseline | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0383 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-02 Repository and module boundary architecture | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0384 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-03 Application service command-query-job model | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0385 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-04 Persistence and schema versioning architecture | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0386 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0387 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-07 API boundary and adapter contract map | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0388 | EXECUTION | UPSTREAM | ARCHITECTURE_BASIS | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0598 | EXECUTION | UPSTREAM | SECURITY_PREDECESSOR | DEL-01-02 Copyright and protected-data boundary policy | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0599 | EXECUTION | UPSTREAM | SECURITY_PREDECESSOR | DEL-02-04 Plugin and extension domain contracts | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0600 | EXECUTION | UPSTREAM | SECURITY_PREDECESSOR | DEL-01-04 Professional responsibility and product-claims policy | ACTIVE | Preserved DAG-002 mirror row. |
| DAG-002-E0619 | EXECUTION | UPSTREAM | SECURITY_PREDECESSOR | DEL-10-02 Import/export adapter framework | CANDIDATE | Preserved non-gating DAG-002 candidate. |
| DEP-012-05-001 | ANCHOR | UPSTREAM | OTHER | SOW-040 Private data handling controls and redaction/export safeguards | ACTIVE | Parent anchor restored from deliverable datasheet/decomposition. |
| DEP-012-05-002 | ANCHOR | UPSTREAM | OTHER | OBJ-010 Protect private project, code, rule-pack, and component data | ACTIVE | Requirement trace restored from deliverable datasheet/decomposition. |
| DEP-012-05-003 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/CONTRACT.md` | ACTIVE | Governing invariant constraint. |
| DEP-012-05-004 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/IP_AND_DATA_BOUNDARY.md` | ACTIVE | Governing IP/data-boundary constraint. |
| DEP-012-05-005 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/SPEC.md` | ACTIVE | Technical baseline constraint. |
| DEP-012-05-006 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/PRD.md` | ACTIVE | Product security/privacy requirement constraint. |
| DEP-012-05-007 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/DIRECTIVE.md` | ACTIVE | Stop-rule and professional-boundary constraint. |
| DEP-012-05-008 | EXECUTION | UPSTREAM | PREREQUISITE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | ACTIVE | Decomposition prerequisite for PKG-12/SOW-040/OBJ-010/AB-00 context. |
| DEP-012-05-009 | EXECUTION | DOWNSTREAM | ENABLES | DEL-12-04 Secret and private-library handling | ACTIVE | Local downstream surface restatement from approved DAG-002 edge DAG-002-E0606. |
| DEP-012-05-010 | EXECUTION | DOWNSTREAM | ENABLES | DEL-12-02 Private data redaction and export controls | ACTIVE | Local downstream surface restatement from approved DAG-002 edge DAG-002-E0610. |
| DEP-012-05-011 | EXECUTION | DOWNSTREAM | ENABLES | DEL-16-04 Agent rationale and professional-boundary controls | ACTIVE | Local downstream surface restatement from approved DAG-002 edge DAG-002-E0839. |
| DEP-012-05-012 | EXECUTION | DOWNSTREAM | ENABLES | DEL-06-02 Sandboxed unit-aware expression evaluator | CANDIDATE | Local downstream restatement of non-gating DAG-002 candidate DAG-002-E0623. |

## Run Notes
- Mode: UPDATE.
- Strictness: CONSERVATIVE.
- Consumer context: RECONCILIATION.
- Decomposition path used: `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Approved graph authority used: `execution/_DAG/DAG-006/`.
- Preliminary graph not promoted: `execution/_DAG/DAG-006/` was not used as authority.
- Anchor doc selected by default heuristic: `Datasheet.md`.
- Execution docs selected by default heuristic: `Procedure.md`, `Guidance.md`, `Specification.md`, `_CONTEXT.md`, `_REFERENCES.md`, and existing dependency artifacts.
- Existing DAG-002 mirror rows were preserved. Their `LastSeen` values were refreshed to this run date.
- Added missing conservative anchors to `SOW-040` and `OBJ-010`.
- Added evidence-backed governing-document constraints because DEL-12-05 source documents explicitly identify them as content constraints.
- Added local downstream surface rows only where the relationship is already present in approved DAG-006; these rows do not mutate aggregate graph authority.
- No source documents, lifecycle files, aggregate DAG artifacts, coordination files, schemas, tests, code, `MEMORY.md`, or `_STATUS.md` were edited.
- Parent anchor check: PASS. One ACTIVE `IMPLEMENTS_NODE` anchor exists.
- Non-gating candidate treatment: `DAG-002-E0619` and `DEP-012-05-012` remain `CANDIDATE`.
- Warning: enum helper `tools/validation/validate_enum.py` is narrower than the DAG-002/local v3.1 surface because it does not accept graph values such as `ARCHITECTURE_BASIS`, `SECURITY_PREDECESSOR`, `CANDIDATE`, `GRAPH_REVIEW`, or `INFERRED_DIRECT`; schema validation remains the controlling local validation used for this refresh.
- Warning: `tools/validation/validate_id_format.sh` expects legacy three-digit package/deliverable formats and rejects current IDs such as `PKG-12` and `DEL-12-05`; this was treated as a tool/schema drift warning rather than a register defect.

## Run History
- 2026-05-10 23:31 MDT — TP-DAG-004 dependency surface refresh; MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; decomposition available; DAG authority `DAG-002`; rows 23 total / 21 ACTIVE / 2 CANDIDATE / 0 RETIRED; validation schema PASS; warnings: enum helper drift and ID-format helper drift.

## Lifecycle Summary
- ACTIVE rows: 21.
- CANDIDATE rows: 2.
- RETIRED rows: 0.
- Closure-state breakdown: SATISFIED 15; UNKNOWN 8.
- Parent anchor closure: SATISFIED.
- Candidate closure: UNKNOWN and non-gating.

## Downstream Handoff Notes
- RECONCILIATION should compare the restored anchors/document constraints and downstream surface rows against the preserved DAG-002 mirror rows before proposing any aggregate graph change.
- The `DEL-10-02` adapter-framework relationship remains CANDIDATE because DAG-002 explicitly retained it as non-gating to avoid forcing threat modeling after adapters without approval.
- The `DEL-06-02` evaluator-hardening relationship is represented as a local downstream CANDIDATE from approved DAG-006 candidate evidence only; it should not be promoted without reconciliation and CHANGE approval.
