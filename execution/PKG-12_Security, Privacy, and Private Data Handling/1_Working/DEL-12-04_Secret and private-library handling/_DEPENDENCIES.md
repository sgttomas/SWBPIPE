# Dependencies: DEL-12-04 Secret and private-library handling

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- `DAG-002-E0375` -> `DEL-00-01` (CONSTRAINT)
- `DAG-002-E0376` -> `DEL-00-02` (CONSTRAINT)
- `DAG-002-E0377` -> `DEL-00-03` (CONSTRAINT)
- `DAG-002-E0378` -> `DEL-00-04` (CONSTRAINT)
- `DAG-002-E0379` -> `DEL-00-06` (CONSTRAINT)
- `DAG-002-E0380` -> `DEL-00-07` (CONSTRAINT)
- `DAG-002-E0381` -> `DEL-00-08` (CONSTRAINT)
- `DAG-002-E0606` -> `DEL-12-05` (PREREQUISITE)
- `DAG-002-E0607` -> `DEL-12-01` (PREREQUISITE)
- `DAG-002-E0608` -> `DEL-03-07` (PREREQUISITE)
- `DAG-002-E0609` -> `DEL-06-04` (PREREQUISITE)

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refreshed:** 2026-06-16
- **Rows:** 17 total; 17 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=4; EXECUTION=13.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Origin |
|---|---|---|---|---|---|---|
| `DEL-12-04-A001` | ANCHOR | UPSTREAM | OTHER | `PKG-12` | ACTIVE | EXTRACTED |
| `DEL-12-04-A002` | ANCHOR | UPSTREAM | OTHER | `SOW-040` | ACTIVE | EXTRACTED |
| `DEL-12-04-A003` | ANCHOR | UPSTREAM | OTHER | `SOW-029` | ACTIVE | EXTRACTED |
| `DEL-12-04-A004` | ANCHOR | UPSTREAM | OTHER | `OBJ-010` | ACTIVE | EXTRACTED |
| `DAG-002-E0375` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-01` | ACTIVE | DECLARED |
| `DAG-002-E0376` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-02` | ACTIVE | DECLARED |
| `DAG-002-E0377` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-03` | ACTIVE | DECLARED |
| `DAG-002-E0378` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-04` | ACTIVE | DECLARED |
| `DAG-002-E0379` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-06` | ACTIVE | DECLARED |
| `DAG-002-E0380` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-07` | ACTIVE | DECLARED |
| `DAG-002-E0381` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-08` | ACTIVE | DECLARED |
| `DAG-002-E0606` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-12-05` | ACTIVE | DECLARED |
| `DAG-002-E0607` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-12-01` | ACTIVE | DECLARED |
| `DAG-002-E0608` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-03-07` | ACTIVE | DECLARED |
| `DAG-002-E0609` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-06-04` | ACTIVE | DECLARED |
| `DEL-12-04-E001` | EXECUTION | UPSTREAM | INTERFACE | `DEL-12-02` | ACTIVE | EXTRACTED |
| `DEL-12-04-E002` | EXECUTION | UPSTREAM | INTERFACE | `DEL-12-03` | ACTIVE | EXTRACTED |

## Canonical Dependency Types
- `CONSTRAINT`: 7
- `INTERFACE`: 2
- `OTHER`: 4
- `PREREQUISITE`: 4

## Run Notes
- **TaskSkill:** dependency-extract
- **MODE:** UPDATE
- **STRICTNESS:** CONSERVATIVE
- **CONSUMER_CONTEXT:** RECONCILIATION
- **ARCHITECTURE_BASIS_POLICY:** PKG00_CONSISTENCY_TRACKERS
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; located and read.
- **Anchor doc:** `Datasheet.md` / `_CONTEXT.md` identity fields.
- **Execution docs:** `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, existing `Dependencies.csv`, and cited PKG-00 Specification/_CONTEXT excerpts.
- **PKG-00 rows reviewed:** 7; changed: 7.
- PKG-00 architecture-basis tracker rows were normalized to `TargetRefID=AB-00-*` and `DependencyType=CONSTRAINT` where local `_CONTEXT.md` evidence named the applicable basis ID.
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
- 2026-06-16 2358: MODE=UPDATE; STRICTNESS=CONSERVATIVE; decomposition=`execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings=None.; ACTIVE=17; RETIRED=0; PKG-00 reviewed=7; PKG-00 changed=7.

## Lifecycle Summary
- **ACTIVE rows:** 17
- **RETIRED rows:** 0
- **SATISFIED:** 17
