# Dependencies: DEL-10-05 Headless CLI and structured I/O analysis runner

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16
- **Closure-status refresh:** 2026-07-22 (six R15-audited edge-grain updates)
- **Rows:** 29 total; 16 ACTIVE; 13 RETIRED.
- **Classes:** ANCHOR=6; EXECUTION=23.
- **Candidate/non-gating rows in register:** 0.

## Canonical Dependency Types
- `CONSTRAINT`: 1
- `INTERFACE`: 6
- `OTHER`: 19
- `PREREQUISITE`: 3

## Candidate / Non-Gating Handoff Notes
- None.

## Run Notes
- TaskSkill: dependency-extract; MODE=UPDATE; STRICTNESS=CONSERVATIVE; CONSUMER_CONTEXT=RECONCILIATION; ARCHITECTURE_BASIS_POLICY=PKG00_CONSISTENCY_TRACKERS.
- Decomposition path: `execution/_Decomposition/SOFTWARE_DECOMP.md`; located and used for anchor/target resolution.
- Anchor doc: `_CONTEXT.md`; execution docs reviewed as needed: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, existing `Dependencies.csv`, and prior `_DEPENDENCIES.md`.
- PKG-00 architecture-basis rows were reviewed read-only where cited; supported rows were preserved and no PKG-00 files were changed.
- Core enum fields conform to the canonical Chirality dependency model.
- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.
- Candidate/non-gating ideas require explicit human graph approval plus graph revalidation before promotion.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.
- 2026-07-22 targeted closure refresh: TaskSkill=`dependency-extract`; MODE=`UPDATE`; STRICTNESS=`CONSERVATIVE`; CONSUMER_CONTEXT=`RECONCILIATION`; ARCHITECTURE_BASIS_POLICY=`NONE` (default). SOURCE_DOCS=`AUTO`, DOC_ROLE_MAP=`DEFAULT`, ANCHOR_DOC=`AUTO`, and EXECUTION_DOC_ORDER=`AUTO` were recorded but not used to widen the sealed six-row refresh.
- 2026-07-22 chosen paths: run root `execution/`; decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md` (present); accepted audit `execution/_Evaluation/DEPENDENCY_READINESS_AUDIT_2026-07-21_R15/`; local register `Dependencies.csv`; local index `_DEPENDENCIES.md`. The existing anchor pass was preserved first; only the six named EXECUTION rows were then reconciled against their cited live evidence.
- The accepted R15 audit classifies `DEP-10-05-E003` through `DEP-10-05-E008` as `SATISFIED_IN_FACT_BUT_STALE` at `SEMANTIC_READY` edge grain with HIGH audit confidence and no change to dependency meaning, scope, or decomposition truth. Broader provider residuals remain open.
- The 2026-06-07/R12 deferral remains bounded to its recorded runner-contract work. This local closure-status refresh neither generalizes that deferral nor authorizes the report/export seam.
- This local register refresh is not DAG activation, a selection decision, lifecycle closure, or successor-DAG acceptance. Recorded aggregate graph authority changes only through its separate governed proposal and owner-acceptance path.
- `[WARNING] ID_FORMAT_HELPER_LEGACY_PATTERN`: `tools/validation/validate_id_format.sh` still expects legacy `PKG-###`, `DEL-###-##`, and `DEP-###-##-###` shapes. It rejects this project's canonical `PKG-XX`, `DEL-XX-YY`, and existing `DEP-10-05-E00N` identities; the unchanged IDs were instead checked against root `docs/TYPES.md`, the decomposition, and the accepted R15 register-equivalence evidence.

## Run History
- 2026-06-16: Dependency semantic refresh for PKG-10 shard; rows added=0, rows retired=0, rows changed=0; PKG-00 rows 7 reviewed / 0 changed; validation passed.
- 2026-07-22: Targeted dependency-satisfaction refresh; MODE=UPDATE; STRICTNESS=CONSERVATIVE; decomposition present at `execution/_Decomposition/SOFTWARE_DECOMP.md`; warning=`ID_FORMAT_HELPER_LEGACY_PATTERN`; rows added=0, rows retired=0, closure rows changed=6; ACTIVE=16, RETIRED=13; R12 bounded deferral and successor-DAG owner gate preserved.

## Lifecycle Summary
- ACTIVE rows: 16
- RETIRED rows: 13
- Closure-state breakdown: SATISFIED=23; TBD=6

## Validation Result
- PASS: `python3 tools/validation/validate_dependencies_schema.py Dependencies.csv` returned VALID for this deliverable.
- PASS (2026-07-22): v3.1 schema/29-column validation, target-row enum checks, uniqueness/count/provenance review, non-target aggregate hash preservation, and `git diff --check` all passed. The ID helper compatibility warning above is pre-existing and did not justify altering canonical IDs.

## Downstream Handoff Notes
- This file is a deliverable-local derivative dependency register index for reconciliation; it is not aggregate graph authority.
- Later DAG fan-in must consume accepted snapshots and preserve retired candidate dispositions unless human graph authority promotes them.
- The six refreshed local rows may feed a candidate successor graph, but do not activate it. Selection of the DEL-08-01/DEL-10-05 seam remains held until the owner separately accepts the successor DAG and authorizes the pointer update.
