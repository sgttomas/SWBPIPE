# Dependencies: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- `DEL-17-02`

## Declared Downstream Dependencies
- `DEL-17-05`

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refreshed:** 2026-06-16
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=7, EXECUTION=10.
- **PKG-00 architecture-basis tracker rows:** 7 ACTIVE; reviewed under `ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS`.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | TargetType | Target | Origin |
|---|---:|---:|---:|---:|---|---:|
| `DEL-17-04-A-PKG17` | ANCHOR | UPSTREAM | OTHER | WBS_NODE | `PKG-17` | EXTRACTED |
| `DEL-17-04-A001` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-075` | EXTRACTED |
| `DEL-17-04-E001` | EXECUTION | UPSTREAM | INTERFACE | DELIVERABLE | `DEL-17-02` | EXTRACTED |
| `DEL-17-04-E002` | EXECUTION | UPSTREAM | PREREQUISITE | DELIVERABLE | `DEL-17-01` | EXTRACTED |
| `DEL-17-04-D001` | EXECUTION | DOWNSTREAM | HANDOVER | DELIVERABLE | `DEL-17-05` | EXTRACTED |
| `DEL-17-04-A-SR001` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-030` | EXTRACTED |
| `DEL-17-04-A-SR002` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `SOW-074` | EXTRACTED |
| `DEL-17-04-A-SR003` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-009` | EXTRACTED |
| `DEL-17-04-A-SR004` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-017` | EXTRACTED |
| `DEL-17-04-A-SR005` | ANCHOR | UPSTREAM | OTHER | REQUIREMENT | `OBJ-018` | EXTRACTED |
| `DEL-17-04-AB-01` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-01` | EXTRACTED |
| `DEL-17-04-AB-02` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-02` | EXTRACTED |
| `DEL-17-04-AB-03` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-03` | EXTRACTED |
| `DEL-17-04-AB-04` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-04` | EXTRACTED |
| `DEL-17-04-AB-06` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-06` | EXTRACTED |
| `DEL-17-04-AB-07` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-07` | EXTRACTED |
| `DEL-17-04-AB-08` | EXECUTION | UPSTREAM | CONSTRAINT | DELIVERABLE | `DEL-00-08` | EXTRACTED |

## Canonical Dependency Types
- `CONSTRAINT`: 7
- `HANDOVER`: 1
- `INTERFACE`: 1
- `OTHER`: 7
- `PREREQUISITE`: 1

## Run Notes
- Mode: UPDATE; strictness: CONSERVATIVE; consumer context: RECONCILIATION.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status: located and used for PKG-17 scope/objective/package anchors.
- Anchor doc selection: `AUTO`; primary anchor evidence used `_CONTEXT.md`, `Datasheet.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Execution doc order: `AUTO`; local `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, and cited PKG-00 architecture-basis files were reviewed as needed.
- PKG-00 policy: supported `DEL-00-*` architecture-consistency tracker rows are retained as upstream execution constraints and do not replace decomposition truth.
- Rows added in this semantic refresh: 13; rows retired: 0; semantic rows changed: 0.
- Core enum fields conform to the canonical Chirality dependency model; no `Status=CANDIDATE` rows were emitted.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- Warnings: none.

## Downstream Handoff Notes
- Later graph fan-in may consume these rows only as deliverable-local dependency evidence; DAG authority remains governed by the active coordination workflow.
- PKG-00 tracker rows are derivative architecture-consistency reminders and must not be treated as substitutes for accepted PKG-00 decomposition truth.

## Run History
- 2026-06-16 2357: dependency semantic refresh, MODE=UPDATE, STRICTNESS=CONSERVATIVE, CONSUMER_CONTEXT=RECONCILIATION; validation result: PASS; warnings: none; ACTIVE rows=17; RETIRED rows=0.

## Lifecycle Summary
- ACTIVE: 17
- RETIRED: 0
- SatisfactionStatus `NOT_APPLICABLE`: 6
- SatisfactionStatus `PENDING`: 8
- SatisfactionStatus `SATISFIED`: 3
