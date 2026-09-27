# Dependencies: DEL-07-09 Interactive operation vocabulary and tool palette contract

## Dependency Tracking Mode
- **Mode:** TBD
- **Register:** the declared sections of this file together with Dependencies.csv (schema v3.1) when present (docs/SPEC.md §5.3)
- **Notes:** TBD

## Declared Upstream (I need these before I can proceed)
- TBD

## Declared Downstream (These need me)
- TBD

## Extracted Dependency Register
- **Status:** SYNCHRONIZED_FROM_DAG_011
- **Source of Truth:** `projects/chirality-piping/execution/_DAG/DAG-011/DependencyEdges.csv`
- **Local Register:** `Dependencies.csv` (schema v3.1): this deliverable's aggregate rows plus its local `Origin=DECLARED` rows
- **Run date:** 2026-09-26
- **Rows:** 7 total; 7 ACTIVE; 0 CANDIDATE.
- **ANCHOR rows (ACTIVE):** 4
- **EXECUTION rows (ACTIVE):** 3

### Authority Boundary
- These rows were materialized from aggregate `DAG-011`. Under `docs/SPEC.md` §5.4 an accepted project DAG version governs blockers only through its acceptance record and only while it is current with the local evidence.
- The local files (`_DEPENDENCIES.md` and `Dependencies.csv`) are the dependency evidence. A departure from the accepted version is decided by the human; local `Origin=DECLARED` rows are kept when this register is rewritten.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.
- Entries in this file's human-owned declared sections remain part of the recorded register (`docs/SPEC.md` §5.3).

## Lifecycle Summary
| Dimension | Count |
|---|---|
| Rows | 7 |
| Status=ACTIVE | 7 |
| SatisfactionStatus=NOT_APPLICABLE | 4 |
| SatisfactionStatus=SATISFIED | 3 |

## Run Notes
- `Dependencies.csv`, the Extracted Dependency Register and the Lifecycle Summary are written by `tools/coordination/materialize_local_dependencies.py --refresh-pointers` from the aggregate DAG named in the register section.
- The Dependency Tracking Mode and declared sections are human-owned; this tool never writes them (`docs/SPEC.md` §5.1). `TBD` there means the human has not yet recorded them.

## Run History
- 2026-09-26 — `materialize_local_dependencies.py --refresh-pointers`: synchronized from `DAG-011`; 7 rows (7 ACTIVE, 0 CANDIDATE).
