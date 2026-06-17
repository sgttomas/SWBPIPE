# Dependencies: DEL-17-06 Stress-neutral CSV JSON package

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- `DEL-17-02`
- `DEL-08-04`
- `DEL-14-02`
- `DEL-14-05`

## Declared Downstream Dependencies
- No active downstream dependency is declared in this local register unless listed in `Dependencies.csv`; historical DAG-005 extraction wording is superseded by DAG-006 active graph authority.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refreshed:** 2026-06-16
- **Rows:** 23 total; 23 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=11, EXECUTION=12.
- **PKG-00 architecture-basis tracker rows:** 7 ACTIVE; reviewed under `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | TargetType | Target | Origin |
|---|---:|---:|---:|---:|---|---:|
| `DEL-17-06-A001` | ANCHOR | UPSTREAM | OTHER | PACKAGE | `PKG-17` | EXTRACTED |
| `DEL-17-06-A002` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-046` | EXTRACTED |
| `DEL-17-06-A003` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-074` | EXTRACTED |
| `DEL-17-06-A004` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-007` | EXTRACTED |
| `DEL-17-06-A005` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-017` | EXTRACTED |
| `DEL-17-06-A006` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-018` | EXTRACTED |
| `DEL-17-06-A007` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `DEL-17-06-REQ-003` | EXTRACTED |
| `DEL-17-06-A008` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `DEL-17-06-REQ-004` | EXTRACTED |
| `DEL-17-06-A009` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `DEL-17-06-REQ-005` | EXTRACTED |
| `DEL-17-06-A010` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `DEL-17-06-REQ-006` | EXTRACTED |
| `DEL-17-06-A011` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `DEL-17-06-REQ-009` | EXTRACTED |
| `DEL-17-06-D001` | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | `DEL-17-01` | EXTRACTED |
| `DEL-17-06-D002` | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | `DEL-17-02` | DECLARED |
| `DEL-17-06-D003` | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | `DEL-08-04` | DECLARED |
| `DEL-17-06-D004` | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | `DEL-14-02` | DECLARED |
| `DEL-17-06-D005` | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | `DEL-14-05` | DECLARED |
| `DEL-17-06-AB-01` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-01` | EXTRACTED |
| `DEL-17-06-AB-02` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-02` | EXTRACTED |
| `DEL-17-06-AB-03` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-03` | EXTRACTED |
| `DEL-17-06-AB-04` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-04` | EXTRACTED |
| `DEL-17-06-AB-06` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-06` | EXTRACTED |
| `DEL-17-06-AB-07` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-07` | EXTRACTED |
| `DEL-17-06-AB-08` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-08` | EXTRACTED |

## Canonical Dependency Types
- `CONSTRAINT`: 7
- `INTERFACE`: 4
- `OTHER`: 11
- `PREREQUISITE`: 1

## Run Notes
- Mode: UPDATE; strictness: CONSERVATIVE; consumer context: RECONCILIATION.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for PKG-17 scope/objective/package anchors.
- Anchor doc selection: `AUTO`; primary anchor evidence used `_CONTEXT.md`, `Datasheet.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Execution doc order: `AUTO`; local `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, and cited PKG-00 architecture-basis files were reviewed as needed.
- PKG-00 policy: supported `DEL-00-*` architecture-consistency tracker rows are retained as upstream execution constraints and do not replace decomposition truth.
- Rows added in this semantic refresh: 7; rows retired: 0; semantic rows changed: 0.
- Core enum fields conform to the canonical Chirality dependency model; no `Status=CANDIDATE` rows were emitted.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Warnings: none.

## Downstream Handoff Notes
- Later graph fan-in may consume these rows only as deliverable-local dependency evidence; DAG authority remains governed by the active coordination workflow.
- PKG-00 tracker rows are derivative architecture-consistency reminders and must not be treated as substitutes for accepted PKG-00 decomposition truth.

## Run History
- 2026-06-16 2357: dependency semantic refresh, MODE=UPDATE, STRICTNESS=CONSERVATIVE, CONSUMER_CONTEXT=RECONCILIATION; validation result: PASS; warnings: none; ACTIVE rows=23; RETIRED rows=0.

## Lifecycle Summary
- ACTIVE: 23
- RETIRED: 0
- SatisfactionStatus `PENDING`: 7
- SatisfactionStatus `SATISFIED`: 11
- SatisfactionStatus `TBD`: 5
