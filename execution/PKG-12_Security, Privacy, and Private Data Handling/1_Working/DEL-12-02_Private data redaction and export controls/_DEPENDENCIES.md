# Dependencies: DEL-12-02 Private data redaction and export controls

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- `DAG-002-E0610` -> `DEL-12-05` (PREREQUISITE)
- `DAG-002-E0611` -> `DEL-12-01` (PREREQUISITE)
- `DAG-002-E0612` -> `DEL-08-01` (PREREQUISITE)
- `DAG-002-E0613` -> `DEL-08-04` (PREREQUISITE)
- `DAG-002-E0614` -> `DEL-06-04` (PREREQUISITE)
- `DAG-002-E0615` -> `DEL-03-07` (PREREQUISITE)

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refreshed:** 2026-06-16
- **Rows:** 18 total; 18 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=16.
- **Candidate rows moved to worklist:** 0.

| DependencyID | Class | Direction | Type | Target | Status | Origin |
|---|---|---|---|---|---|---|
| `TP-DAG-004-DEL-12-02-A001` | ANCHOR | UPSTREAM | OTHER | `SOW-040` | ACTIVE | EXTRACTED |
| `TP-DAG-004-DEL-12-02-A002` | ANCHOR | UPSTREAM | OTHER | `OBJ-010` | ACTIVE | EXTRACTED |
| `DAG-002-E0361` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-01` | ACTIVE | EXTRACTED |
| `DAG-002-E0362` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-02` | ACTIVE | EXTRACTED |
| `DAG-002-E0363` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-03` | ACTIVE | EXTRACTED |
| `DAG-002-E0364` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-04` | ACTIVE | EXTRACTED |
| `DAG-002-E0365` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-06` | ACTIVE | EXTRACTED |
| `DAG-002-E0366` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-07` | ACTIVE | EXTRACTED |
| `DAG-002-E0367` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-08` | ACTIVE | EXTRACTED |
| `DAG-002-E0610` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-12-05` | ACTIVE | DECLARED |
| `DAG-002-E0611` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-12-01` | ACTIVE | DECLARED |
| `DAG-002-E0612` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-08-01` | ACTIVE | DECLARED |
| `DAG-002-E0613` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-08-04` | ACTIVE | DECLARED |
| `DAG-002-E0614` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-06-04` | ACTIVE | DECLARED |
| `DAG-002-E0615` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-03-07` | ACTIVE | DECLARED |
| `TP-DAG-004-DEL-12-02-E001` | EXECUTION | UPSTREAM | CONSTRAINT | `OPS-CONTRACT` | ACTIVE | EXTRACTED |
| `TP-DAG-004-DEL-12-02-E002` | EXECUTION | UPSTREAM | CONSTRAINT | `OPS-IP-DATA-BOUNDARY` | ACTIVE | EXTRACTED |
| `TP-DAG-004-DEL-12-02-E003` | EXECUTION | UPSTREAM | CONSTRAINT | `OPS-SPEC` | ACTIVE | EXTRACTED |

## Canonical Dependency Types
- `CONSTRAINT`: 10
- `OTHER`: 2
- `PREREQUISITE`: 6

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
- 2026-06-16 2358: MODE=UPDATE; STRICTNESS=CONSERVATIVE; decomposition=`execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings=None.; ACTIVE=18; RETIRED=0; PKG-00 reviewed=7; PKG-00 changed=7.

## Lifecycle Summary
- **ACTIVE rows:** 18
- **RETIRED rows:** 0
- **SATISFIED:** 18
