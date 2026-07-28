# Dependency-closure revalidation brief

| Field | Value |
|---|---|
| `EXECUTION_ROOT` | `projects/chirality-piping/execution/` |
| `SCOPE` | `ALL` |
| `RUN_LABEL` | `SCA008_DAG008_REV011_REVALIDATION` |
| `REQUESTED_BY` | Piping Agent 1 |
| `FILTER_ACTIVE_ONLY` | `true` |
| `NORMALIZE_IDS` | `true` |
| `HUB_THRESHOLD` | `20` |
| Accepted decomposition | revision `0.11` through SCA-008 |
| Graph authority under test | immutable `execution/_DAG/DAG-008/` |
| Examined Git basis | `404e47c16a88e7ffdc6d1fc5fac61ebb6864211e` |

## Objective

Determine whether SCA-008 and decomposition revision 0.11 changed any
dependency input or invalidated DAG-008. Revalidate the accepted graph against
all current deliverable-local dependency registers without regenerating,
rewriting, or superseding DAG-008.

## Required proof

1. No SCA-008 or later path change touches a local `Dependencies.csv`,
   DAG-008, or the DAG pointer.
2. All 101 accepted deliverables equal the DAG-008 node set.
3. Local and aggregate registers retain 1,480 matching dependency IDs and
   byte-equivalent common fields other than `Status` and `Notes`.
4. The only status differences are the 30 accepted aggregate-only duplicate
   retirements in the DAG-008 worklist.
5. Local and aggregate current topology both resolve to 972 unique active
   directed edges.
6. Canonical schema, endpoint, cycle, duplicate-active-edge,
   bidirectional-pair, manifest, and 15-wave checks pass.

## Write boundary

Write only this immutable derivative snapshot and
`execution/_Evaluation/DepClosure/_LATEST.md`. Do not edit DAG-008,
`execution/_DAG/_LATEST.md`, any dependency register, decomposition,
deliverable, lifecycle, product, runtime, release, estimate, or schedule
surface.
