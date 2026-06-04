# Dependencies: DEL-16-01 Structured model operation schema

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Source of Truth:** `execution/_DAG/DAG-006/DependencyEdges.csv` remains the approved graph authority.
- **Local Register:** `Dependencies.csv`
- **Rows:** 16 total; 16 ACTIVE; 0 RETIRED; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Refreshed:** 2026-05-11

## Authority Boundary
- Aggregate `DAG-002` remains the approved active-edge sequencing and blocker-computation authority within its approval boundary.
- `DAG-003` was not approved, promoted, edited, or used as authority in this refresh.
- This local register is a refreshed deliverable evidence surface for later `RECONCILIATION`, not an independent graph approval.
- Candidate/non-gating items remain non-gating unless later `RECONCILIATION` plus `CHANGE` approval promotes them.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; this refresh does not create local `PKG-00` dependency registers.

## Extracted Dependency Register

| Metric | Count |
|---|---:|
| Total rows | 16 |
| ACTIVE | 16 |
| RETIRED | 0 |
| CANDIDATE | 0 |
| ANCHOR | 2 |
| EXECUTION | 14 |
| UPSTREAM | 13 |
| DOWNSTREAM | 3 |

| DependencyID | Class | Direction | Type | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| DEL-16-01-A001 | ANCHOR | UPSTREAM | OTHER | SOW-069 | ACTIVE | `_CONTEXT.md` `## Scope Coverage` |
| DEL-16-01-A002 | ANCHOR | UPSTREAM | OTHER | OBJ-015 | ACTIVE | `_CONTEXT.md` `## Objective Support` |
| DAG-002-E0730 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-01 | ACTIVE | `_CONTEXT.md` `## Architecture Basis Injection` |
| DAG-002-E0731 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-02 | ACTIVE | `_CONTEXT.md` `## Architecture Basis Injection` |
| DAG-002-E0732 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-03 | ACTIVE | `_CONTEXT.md` `## Architecture Basis Injection` |
| DAG-002-E0733 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-04 | ACTIVE | `_CONTEXT.md` `## Architecture Basis Injection` |
| DAG-002-E0734 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 | ACTIVE | `_CONTEXT.md` `## Architecture Basis Injection` |
| DAG-002-E0735 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-07 | ACTIVE | `_CONTEXT.md` `## Architecture Basis Injection` |
| DAG-002-E0736 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-08 | ACTIVE | `_CONTEXT.md` `## Architecture Basis Injection` |
| DAG-002-E0823 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 | ACTIVE | `Specification.md` `DEL-16-01-R007` |
| DAG-002-E0824 | EXECUTION | UPSTREAM | INTERFACE | DEL-13-01 | ACTIVE | `_CONTEXT.md` `## Description` |
| DAG-002-E0825 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-05 | ACTIVE | `Specification.md` `DEL-16-01-R010` |
| DAG-002-E0826 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-01-04 | ACTIVE | `Specification.md` `DEL-16-01-R004` |
| DEL-16-01-D001 | EXECUTION | DOWNSTREAM | ENABLES | DEL-16-02 | ACTIVE | `Specification.md` `## Scope` |
| DEL-16-01-D002 | EXECUTION | DOWNSTREAM | ENABLES | DEL-16-03 | ACTIVE | `Specification.md` `## Scope` |
| DEL-16-01-D003 | EXECUTION | DOWNSTREAM | ENABLES | DEL-07-08 | ACTIVE | `Guidance.md` `## Purpose` |

## Run Notes
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Consumer context: `RECONCILIATION`.
- Run root: `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; status found and used for anchor validation.
- Approved graph authority: `/Users/ryan/ai-env/projects/chirality-piping/execution/_DAG/DAG-006/`.
- Preliminary graph explicitly not used as authority: `/Users/ryan/ai-env/projects/chirality-piping/execution/_DAG/DAG-006/`.
- Anchor document selection: `Datasheet.md` and `_CONTEXT.md` because both explicitly identify DEL-16-01, SOW-069, and OBJ-015.
- Execution document order: `Specification.md`, `Procedure.md`, `Guidance.md`, `_CONTEXT.md`, `_REFERENCES.md`, plus approved DAG-006 edge rows for graph-authority preservation.
- Existing DAG-002 mirror rows were preserved by `DependencyID` and refreshed with canonical dependency-extract enums. Original DAG-002 semantic types are preserved in `Notes`.
- Added one parent anchor (`SOW-069`) and one requirement/objective trace (`OBJ-015`) from explicit sealed-context fields.
- Added three conservative downstream consumer rows already present as active DAG-002 edges: `DEL-16-02`, `DEL-16-03`, and `DEL-07-08`. These are local reconciliation evidence only; they do not change aggregate graph authority.
- No candidate rows were promoted. No active cycles were introduced in this local surface.
- `[WARNING] ID_FORMAT_VALIDATOR_STALE`: `tools/validation/validate_id_format.sh` expects legacy three-digit formats (`PKG-000`, `DEL-000-00`, `SOW-0000`) and rejects current revision 0.7 IDs such as `PKG-16`, `DEL-16-01`, and `SOW-069`; `OBJ-015` validates.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 16 |
| RETIRED | 0 |
| CANDIDATE | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 9 |
| TBD | 7 |

| DependencyType | Count |
|---|---:|
| OTHER | 2 |
| PREREQUISITE | 7 |
| INTERFACE | 3 |
| CONSTRAINT | 1 |
| ENABLES | 3 |

## Downstream Handoff Notes
- For `RECONCILIATION`, treat the active DAG-002 IDs retained here as approved graph carry-forward evidence, not as new graph approval.
- Review the new local anchor rows during fan-in because the TP-DAG-004 audit reported zero local anchors before refresh.
- Review downstream rows `DEL-16-01-D001` through `DEL-16-01-D003` against aggregate edge direction conventions before any future graph proposal.
- The `DEL-02-05` persistence interface remains active due to DAG-002 authority, but local source evidence keeps exact hash binding and operation persistence granularity as TBD.

## Run History
- 2026-05-03: Local register synchronized from approved `DAG-006`; 11 ACTIVE upstream execution rows; 0 CANDIDATE rows.
- 2026-05-11 00:02 MDT: TP-DAG-004 dependency surface refresh; mode `UPDATE`; strictness `CONSERVATIVE`; consumer context `RECONCILIATION`; decomposition found; DAG-002 authority used; 16 ACTIVE rows after refresh; warnings: `ID_FORMAT_VALIDATOR_STALE`.
