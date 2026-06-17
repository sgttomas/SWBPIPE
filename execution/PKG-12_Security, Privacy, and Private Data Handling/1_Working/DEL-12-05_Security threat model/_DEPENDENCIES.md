# Dependencies: DEL-12-05 Security threat model

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
- **Rows:** 23 total; 21 ACTIVE; 2 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=21.
- **Candidate rows moved to worklist:** 2.

| DependencyID | Class | Direction | Type | Target | Status | Origin |
|---|---|---|---|---|---|---|
| `DAG-002-E0382` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-01` | ACTIVE | EXTRACTED |
| `DAG-002-E0383` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-02` | ACTIVE | EXTRACTED |
| `DAG-002-E0384` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-03` | ACTIVE | EXTRACTED |
| `DAG-002-E0385` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-04` | ACTIVE | EXTRACTED |
| `DAG-002-E0386` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-06` | ACTIVE | EXTRACTED |
| `DAG-002-E0387` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-07` | ACTIVE | EXTRACTED |
| `DAG-002-E0388` | EXECUTION | UPSTREAM | CONSTRAINT | `DEL-00-08` | ACTIVE | EXTRACTED |
| `DAG-002-E0598` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-02` | ACTIVE | EXTRACTED |
| `DAG-002-E0599` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-02-04` | ACTIVE | EXTRACTED |
| `DAG-002-E0600` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-01-04` | ACTIVE | EXTRACTED |
| `DAG-002-E0619` | EXECUTION | UPSTREAM | PREREQUISITE | `DEL-10-02` | RETIRED | EXTRACTED |
| `DEP-012-05-001` | ANCHOR | UPSTREAM | OTHER | `SOW-040` | ACTIVE | EXTRACTED |
| `DEP-012-05-002` | ANCHOR | UPSTREAM | OTHER | `OBJ-010` | ACTIVE | EXTRACTED |
| `DEP-012-05-003` | EXECUTION | UPSTREAM | CONSTRAINT | `docs/CONTRACT.md` | ACTIVE | EXTRACTED |
| `DEP-012-05-004` | EXECUTION | UPSTREAM | CONSTRAINT | `docs/IP_AND_DATA_BOUNDARY.md` | ACTIVE | EXTRACTED |
| `DEP-012-05-005` | EXECUTION | UPSTREAM | CONSTRAINT | `docs/SPEC.md` | ACTIVE | EXTRACTED |
| `DEP-012-05-006` | EXECUTION | UPSTREAM | CONSTRAINT | `docs/PRD.md` | ACTIVE | EXTRACTED |
| `DEP-012-05-007` | EXECUTION | UPSTREAM | CONSTRAINT | `docs/DIRECTIVE.md` | ACTIVE | EXTRACTED |
| `DEP-012-05-008` | EXECUTION | UPSTREAM | PREREQUISITE | `execution/_Decomposition/SOFTWARE_DECOMP.md` | ACTIVE | EXTRACTED |
| `DEP-012-05-009` | EXECUTION | DOWNSTREAM | ENABLES | `DEL-12-04` | ACTIVE | EXTRACTED |
| `DEP-012-05-010` | EXECUTION | DOWNSTREAM | ENABLES | `DEL-12-02` | ACTIVE | EXTRACTED |
| `DEP-012-05-011` | EXECUTION | DOWNSTREAM | ENABLES | `DEL-16-04` | ACTIVE | EXTRACTED |
| `DEP-012-05-012` | EXECUTION | DOWNSTREAM | ENABLES | `DEL-06-02` | RETIRED | EXTRACTED |

## Canonical Dependency Types
- `CONSTRAINT`: 12
- `ENABLES`: 4
- `OTHER`: 2
- `PREREQUISITE`: 5

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
- [WARNING] NON_GATING_CANDIDATES_RETIRED: 2 candidate/non-gating rows remain retired in the register worklist representation.

## Downstream Handoff Notes
- Use this deliverable-local register as a refreshed PKG-12 shard only; do not treat it as aggregate DAG authority.
- Derivative-package status: this register is derivative of local deliverable evidence, `SOFTWARE_DECOMP.md` revision 0.7, and cited PKG-00 architecture-basis constraints.
- Remaining blockers: candidate/non-gating rows, if any, require human graph approval before promotion.

## Run History
- 2026-06-16 2358: MODE=UPDATE; STRICTNESS=CONSERVATIVE; decomposition=`execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings=[WARNING] NON_GATING_CANDIDATES_RETIRED: 2 candidate/non-gating rows remain retired in the register worklist representation.; ACTIVE=21; RETIRED=2; PKG-00 reviewed=7; PKG-00 changed=7.

## Lifecycle Summary
- **ACTIVE rows:** 21
- **RETIRED rows:** 2
- **SATISFIED:** 15
- **TBD:** 8
