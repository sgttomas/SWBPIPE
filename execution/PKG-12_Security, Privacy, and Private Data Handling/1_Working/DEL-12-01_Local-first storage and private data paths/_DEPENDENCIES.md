# Dependencies: DEL-12-01 Local-first storage and private data paths

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/` is the approved legacy graph pending `DAG-007` canonical approval.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refreshed:** 2026-06-16
- **Rows:** 14 total; 14 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=12.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Origin |
|---|---|---|---|---|---|---|
| `DEL-12-01-A001` | ANCHOR | UPSTREAM | OTHER | `SOW-029` | ACTIVE | EXTRACTED |
| `DEL-12-01-A002` | ANCHOR | UPSTREAM | OTHER | `OBJ-010` | ACTIVE | EXTRACTED |
| `DAG-002-E0354` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-01` | ACTIVE | EXTRACTED |
| `DAG-002-E0355` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-02` | ACTIVE | EXTRACTED |
| `DAG-002-E0356` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-03` | ACTIVE | EXTRACTED |
| `DAG-002-E0357` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-04` | ACTIVE | EXTRACTED |
| `DAG-002-E0358` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-06` | ACTIVE | EXTRACTED |
| `DAG-002-E0359` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-07` | ACTIVE | EXTRACTED |
| `DAG-002-E0360` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-08` | ACTIVE | EXTRACTED |
| `DAG-002-E0601` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-02-05` | ACTIVE | EXTRACTED |
| `DAG-002-E0602` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-12-05` | ACTIVE | EXTRACTED |
| `DAG-002-E0603` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-01-02` | ACTIVE | EXTRACTED |
| `DEL-12-01-E001` | EXECUTION | DOWNSTREAM | HANDOVER | `DEL-12-02` | ACTIVE | EXTRACTED |
| `DEL-12-01-E002` | EXECUTION | DOWNSTREAM | INTERFACE | `DEL-12-04` | ACTIVE | EXTRACTED |

## Canonical Dependency Types
- `CONSTRAINT`: 8
- `HANDOVER`: 1
- `INTERFACE`: 1
- `OTHER`: 2
- `PREREQUISITE`: 2

## Run Notes
- **TaskSkill:** dependency-extract
- **MODE:** UPDATE
- **STRICTNESS:** CONSERVATIVE
- **CONSUMER_CONTEXT:** RECONCILIATION
- **ARCHITECTURE_BASIS_POLICY:** PKG00_CONSISTENCY_TRACKERS
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; located and read.
- **Anchor doc:** `Datasheet.md` / `_CONTEXT.md` identity fields.
- **Execution docs:** `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, existing `Dependencies.csv`, and cited PKG-00 Specification/_CONTEXT excerpts.
- **PKG-00 rows reviewed:** 7; changed: 0.
- PKG-00 architecture-basis tracker rows were reviewed and already used supported canonical semantics.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- None.

## Downstream Handoff Notes
- Use this deliverable-local register as a refreshed PKG-12 shard only; do not treat it as aggregate DAG authority.
- Derivative-package status: this register is derivative of local deliverable evidence, `SOFTWARE_DECOMP.md` revision 0.7, and cited PKG-00 architecture-basis constraints.
- Remaining blockers: candidate/non-gating rows, if any, require human graph approval before promotion.

## Run History
- 2026-06-16 2358: MODE=UPDATE; STRICTNESS=CONSERVATIVE; decomposition=`execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings=None.; ACTIVE=14; RETIRED=0; PKG-00 reviewed=7; PKG-00 changed=0.

## Lifecycle Summary
- **ACTIVE rows:** 14
- **RETIRED rows:** 0
- **NOT_APPLICABLE:** 2
- **SATISFIED:** 12
