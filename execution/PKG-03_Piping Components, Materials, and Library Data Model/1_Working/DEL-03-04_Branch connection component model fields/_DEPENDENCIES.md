# Dependencies: DEL-03-04 Branch connection component model fields

## TP-DAG-004 Refresh

- **Refresh Status:** REFRESHED_TP_DAG_004_CONSERVATIVE
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Schema Version:** v3.1
- **Local Register:** `Dependencies.csv`
- **Rows:** 9 total; 9 ACTIVE; 0 CANDIDATE; 0 removed.
- **Generated:** 2026-05-10

## Extracted Dependency Surface

| Dependency Class | Rows | Status |
|---|---:|---|
| ARCHITECTURE_BASIS | 6 | ACTIVE |
| DOMAIN_MODEL | 1 | ACTIVE |
| UNIT_CONTRACT | 1 | ACTIVE |
| GOVERNANCE_PREDECESSOR | 1 | ACTIVE |

## Conservative Findings

- The six SCA-001 architecture-basis rows remain explicit in `_CONTEXT.md`: AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
- The direct domain-model predecessor remains `DEL-03-02` because branch connection fields extend the component library/schema surface.
- The direct unit-contract predecessor remains `DEL-02-02` because branch geometry and reinforcement fields require unit-aware validation.
- The direct governance predecessor remains `DEL-01-02` because branch SIF/flexibility data must not introduce protected standards or vendor data into the public project.
- No new candidate rows were emitted under conservative strictness.

## Authority Boundary

- This local register is a deliverable-local dependency evidence surface for RECONCILIATION.
- Aggregate DAG or coordination files remain outside this task's write scope and were not modified.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers and is not marked `ISSUED` by this refresh.
- Rows with `SatisfactionStatus=UNKNOWN` are retained as unresolved predecessor evidence, not as implementation approval.

## Validation Closeout

- CSV shape and required v3.1 header validated.
- Enum fields validated for emitted values: `DependencyClass`, `AnchorType`, `Direction`, `DependencyType`, `TargetType`, `Explicitness`, `RequiredMaturity`, `ProposedMaturity`, `SatisfactionStatus`, `Confidence`, `Origin`, and `Status`.
- Row count validated: 9 data rows.
- No source docs, status, memory, code, schema, test, DAG, or coordination files were edited.
