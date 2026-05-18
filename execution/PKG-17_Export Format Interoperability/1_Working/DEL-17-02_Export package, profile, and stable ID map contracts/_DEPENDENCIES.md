# Dependencies: DEL-17-02 Export package, profile, and stable ID map contracts

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-005/` is the approved active graph authority.
- **Authority Boundary:** `DAG-005` active edges govern implementation-readiness coordination. Candidate rows, if any, remain non-gating until explicit promotion and revalidation.

## Declared Upstream Dependencies
- `DEL-17-01`

## Declared Downstream Dependencies
- `DEL-17-03`
- `DEL-17-04`
- `DEL-17-06`
- `DEL-17-07`
- `DEL-17-08`
- `DEL-17-09`

## Extracted Dependency Register
- `Dependencies.csv` was created by TP-EXPORT-002 TASK + `dependency-extract` after production documents and semantic artifacts were populated.
- Register schema version: `v3.1`.
- Upstream satisfied evidence includes committed `DEL-17-01` source-basis evidence at commit `6f1e3427`.
- Downstream rows record consumer relationships only; production documents for `DEL-17-03` through `DEL-17-09` were not populated in this tranche.

## Local Dependency Summary

| Direction | Target | Status | Notes |
|---|---|---|---|
| Upstream | DEL-17-01 | SATISFIED | Source-basis dossier committed and recorded in active DEV-001 evidence. |
| Upstream | CONTRACT / IP-DATA / PKG-00 basis | SATISFIED | Accepted architecture and governance baseline; no copied protected content. |
| Upstream | DEL-02-01 / model schema | SATISFIED | Identity vocabulary reference only; no schema edit. |
| Downstream | DEL-17-03 through DEL-17-09 | PENDING | Consumers under DAG-005; not populated here. |

## Notes
- Candidate rows remain non-gating until explicit promotion and graph revalidation.
- Dependencies do not authorize implementation, lifecycle promotion, release claims, or professional claims by themselves.
