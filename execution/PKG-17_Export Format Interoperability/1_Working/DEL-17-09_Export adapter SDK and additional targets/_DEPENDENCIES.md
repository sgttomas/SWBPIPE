# Dependencies: DEL-17-09 Export adapter SDK and additional targets

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- `DEL-17-02`
- `DEL-02-04`
- `DEL-10-01`
- `DEL-10-02`

## Declared Downstream Dependencies
- No active downstream dependency is declared in this local register unless listed in `Dependencies.csv`; historical DAG-005 extraction wording is superseded by DAG-006 active graph authority.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refreshed:** 2026-06-16
- **Rows:** 18 total; 18 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=7, EXECUTION=11.
- **PKG-00 architecture-basis tracker rows:** 7 ACTIVE; reviewed under `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | TargetType | Target | Origin |
|---|---:|---:|---:|---:|---|---:|
| `DEL-17-09-A001` | ANCHOR | UPSTREAM | OTHER | WBS_NODE | `PKG-17` | EXTRACTED |
| `DEL-17-09-A002` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-030` | EXTRACTED |
| `DEL-17-09-A003` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-074` | EXTRACTED |
| `DEL-17-09-A004` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-075` | EXTRACTED |
| `DEL-17-09-A005` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-009` | EXTRACTED |
| `DEL-17-09-A006` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-017` | EXTRACTED |
| `DEL-17-09-A007` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-018` | EXTRACTED |
| `DEL-17-09-E001` | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | `DEL-17-02` | DECLARED |
| `DEL-17-09-E002` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-02-04` | DECLARED |
| `DEL-17-09-E003` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-10-01` | DECLARED |
| `DEL-17-09-E004` | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | `DEL-10-02` | DECLARED |
| `DEL-17-09-AB-01` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-01` | EXTRACTED |
| `DEL-17-09-AB-02` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-02` | EXTRACTED |
| `DEL-17-09-AB-03` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-03` | EXTRACTED |
| `DEL-17-09-AB-04` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-04` | EXTRACTED |
| `DEL-17-09-AB-06` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-06` | EXTRACTED |
| `DEL-17-09-AB-07` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-07` | EXTRACTED |
| `DEL-17-09-AB-08` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-08` | EXTRACTED |

## Canonical Dependency Types
- `CONSTRAINT`: 9
- `INTERFACE`: 1
- `OTHER`: 7
- `PREREQUISITE`: 1

## Run Notes
- Mode: UPDATE; strictness: CONSERVATIVE; consumer context: RECONCILIATION.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for PKG-17 scope/objective/package anchors.
- Anchor doc selection: `AUTO`; primary anchor evidence used `_CONTEXT.md`, `Datasheet.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Execution doc order: `AUTO`; local `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, and cited PKG-00 architecture-basis files were reviewed as needed.
- PKG-00 policy: supported `DEL-00-*` architecture-consistency tracker rows are retained as upstream execution constraints and do not replace decomposition truth.
- Rows added in this semantic refresh: 7; rows retired: 0; semantic rows changed: 1.
- Core enum fields conform to the canonical Chirality dependency model; no `Status=CANDIDATE` rows were emitted.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Warnings: none.

## Downstream Handoff Notes
- Later graph fan-in may consume these rows only as deliverable-local dependency evidence; DAG authority remains governed by the active coordination workflow.
- PKG-00 tracker rows are derivative architecture-consistency reminders and must not be treated as substitutes for accepted PKG-00 decomposition truth.

## Run History
- 2026-06-16 2357: dependency semantic refresh, MODE=UPDATE, STRICTNESS=CONSERVATIVE, CONSUMER_CONTEXT=RECONCILIATION; validation result: PASS; warnings: none; ACTIVE rows=18; RETIRED rows=0.

## Lifecycle Summary
- ACTIVE: 18
- RETIRED: 0
- SatisfactionStatus `NOT_APPLICABLE`: 7
- SatisfactionStatus `PENDING`: 7
- SatisfactionStatus `TBD`: 4
