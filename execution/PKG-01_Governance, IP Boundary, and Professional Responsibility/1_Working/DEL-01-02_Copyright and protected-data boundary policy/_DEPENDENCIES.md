# Dependencies: DEL-01-02 Copyright and protected-data boundary policy

## Generated Dependency Register
- **Status:** REFRESHED_BY_DEPENDENCY_EXTRACT
- **Previous source:** `execution/_DAG/DAG-006/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Authority Boundary
- Aggregate DAG artifacts remain the sequencing and blocker-computation authority only within their approval boundary.
- Aggregate `DAG-006` is the approved active graph authority within its approval boundary.
- This local register is a deliverable-local evidence surface for downstream RECONCILIATION, not an independent aggregate graph authority.
- Candidate or uncertain rows are preserved as evidence and are not promoted to aggregate authority by this refresh.
- Previously synchronized DAG rows are retained as `Origin=DECLARED` after v3.1 enum normalization.

## Extracted Dependency Register

| Class | Status | Count |
|---|---:|---:|
| ANCHOR | ACTIVE | 4 |
| EXECUTION | ACTIVE | 9 |
| ANCHOR | RETIRED | 0 |
| EXECUTION | RETIRED | 0 |

| DependencyID | Class | Direction | Type | Target | Status | Origin |
|---|---|---|---|---|---|---|
| DAG-002-E0005 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 Architecture decision record baseline | ACTIVE | DECLARED |
| DAG-002-E0006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 Repository and module boundary architecture | ACTIVE | DECLARED |
| DAG-002-E0007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE | DECLARED |
| DAG-002-E0008 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 Layered software test and acceptance strategy | ACTIVE | DECLARED |
| DAG-002-E0389 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-01-01 Project governance baseline | ACTIVE | DECLARED |
| DEL-01-02-A001 | ANCHOR | UPSTREAM | OTHER | PKG-01 Governance, IP Boundary, and Professional Responsibility | ACTIVE | EXTRACTED |
| DEL-01-02-A002 | ANCHOR | UPSTREAM | OTHER | SOW-003 Protected standards redistribution boundary | ACTIVE | EXTRACTED |
| DEL-01-02-A003 | ANCHOR | UPSTREAM | OTHER | SOW-028 Contributor governance and IP controls | ACTIVE | EXTRACTED |
| DEL-01-02-A004 | ANCHOR | UPSTREAM | OTHER | OBJ-002 Protect standards-body and vendor intellectual property | ACTIVE | EXTRACTED |
| DEL-01-02-E001 | EXECUTION | UPSTREAM | PREREQUISITE | docs/CONTRACT.md CONTRACT invariant catalog | ACTIVE | EXTRACTED |
| DEL-01-02-E002 | EXECUTION | UPSTREAM | PREREQUISITE | Human/legal review | ACTIVE | EXTRACTED |
| DEL-01-02-E003 | EXECUTION | UPSTREAM | PREREQUISITE | Human project authority assignment | ACTIVE | EXTRACTED |
| DEL-01-02-E004 | EXECUTION | DOWNSTREAM | HANDOVER | docs/IP_AND_DATA_BOUNDARY.md IP and data boundary policy | ACTIVE | EXTRACTED |

## Run Notes
- **Run timestamp:** 2026-05-10T21:40:52-0600
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Scope:** DEL-01-02
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition status:** available; used to validate PKG-01, SOW-003, SOW-028, OBJ-002, and referenced deliverable IDs.
- **Source docs:** AUTO.
- **DOC_ROLE_MAP:** DEFAULT.
- **Anchor doc:** `Datasheet.md`.
- **Execution doc order:** `Procedure.md`, `Guidance.md`, `Specification.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`.
- **Defaults applied:** `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- **Warnings:** `validate_id_format.sh` rejects project identifiers `PKG-01`, `DEL-01-02`, and `SOW-003` because the tool pattern expects three-digit package and four-digit SOW forms. Decomposition evidence uses the project identifiers as authoritative for this refresh.
- **Tree x DAG check:** exactly one ACTIVE parent anchor (`DEL-01-02-A001`); no `FLOATING_NODE` or `AMBIGUOUS_ANCHOR` warning.
- **Protected-content posture:** no protected standards content, private data, engineering defaults, legal conclusions, or professional/code-compliance claims were inferred.
- **Uncertainty handling:** unresolved human/legal review, reviewer role, and final governance/legal mechanism remain `TBD`/`PENDING`; checklist path and repo-level artifact handoff evidence are now resolved in the current-basis local register.

## Run History
- 2026-05-03: synchronized from `execution/_DAG/DAG-006/DependencyEdges.csv`; 5 ACTIVE rows; local register was a mirror/evidence surface.
- 2026-05-10: dependency-extract refresh, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `CONSUMER_CONTEXT=RECONCILIATION`; decomposition available; warnings: ID-format validator pattern mismatch for current project IDs; ACTIVE counts: 4 ANCHOR, 9 EXECUTION.
- 2026-06-03: WORKING_ITEMS current-basis dependency update after requested completion assessment; preserved historical dependency IDs while marking satisfied current-basis evidence for DEL-01-01 governance predecessor and repo-level policy/checklist handoff.

## Lifecycle Summary
- **Total rows:** 13
- **ACTIVE:** 13
- **RETIRED:** 0
- **Origin breakdown:** 5 DECLARED; 8 EXTRACTED.
- **SatisfactionStatus breakdown:** 11 SATISFIED; 2 PENDING; 0 TBD.
- **RequiredMaturity breakdown:** 13 SEMANTIC_READY.
- **ProposedMaturity breakdown:** 11 SEMANTIC_READY; 2 TBD.

## Current-Basis Notes
- For RECONCILIATION, treat this file as deliverable-local evidence only; do not use it as aggregate DAG authority without downstream approval.
- `DAG-002-E0389` remains a preserved historical predecessor row by ID, but current-basis evidence now satisfies the local DEL-01-01 governance predecessor dependency.
- Human/legal review and human project authority assignment remain explicit pending upstream prerequisites, not legal conclusions or implementation authorization.
- Repo-level artifacts exist at `docs/IP_AND_DATA_BOUNDARY.md` and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`; this satisfies local handoff evidence and does not edit or approve aggregate `DAG-006`.
