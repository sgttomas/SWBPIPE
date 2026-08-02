# N3B authority/schema crosscheck — sealed brief

- Parent: EVALUATION N3 under HELP_HUMAN R23.
- Form: read-only ephemeral Agent 2; no delegation.
- Frozen Git basis: `23d15899fd0acf5d1d0513f3fe396438375c9e25`.
- Accepted parent graph: approved immutable
  `projects/chirality-piping/execution/_DAG/DAG-008/`.
- Candidate under review:
  `projects/chirality-piping/execution/_Evaluation/DAG009_CANDIDATE_EVALUATION_2026-08-02_R23/bundle/DAG-009/`.
- Required audit:
  1. Confirm exactly the same 16 artifact roles as approved DAG-008, with the
     three DAG-numbered roles renamed to DAG-009 and no extra/missing role.
  2. Verify bundle manifest, JSON parseability, canonical v3.1 schema, unique
     DependencyIDs, row width/count/order, status/satisfaction counts, strict
     graph facts, SCC/cycle/topological-wave consistency, and duplicate/
     bidirectional/endpoint counts.
  3. Confirm `DeliverableNodes.csv` is byte-identical to DAG-008 and every
     `dag.json` node/edge object is identical; only proposal metadata may differ.
  4. Confirm the approval file is explicitly a proposed-pending placeholder
     with `approved: TBD` and `approved_by: TBD`; the bundle-local pointer is
     proposal-only; root `_DAG/_LATEST.md` still identifies DAG-008.
  5. Confirm approved DAG-008, root pointer, and decomposition file hashes match
     the accepted frozen guards; no live `_DAG/DAG-009` exists.
  6. Confirm proposal/provenance/handoff language creates no activation,
     lifecycle, selection, DEC-092 implementation, product, or professional
     authority.
- Expected counts: nodes=101; rows=1480; ACTIVE=1395; RETIRED=85;
  candidate=0; unique active graph edges=972; SCCs=0; waves=15;
  SATISFIED=800; TBD=321; PENDING=118; NOT_APPLICABLE=241.
- Return schema: `Verdict`; role inventory; manifest/schema/JSON/graph/count
  checks; authority/pointer/decomposition guards; immutable-parent comparison;
  contradictions/unknowns; no-write/no-delegation confirmation.
- Tools: read-only filesystem/search/hash/CSV/JSON validation only. No writes,
  patches, Git mutation, network, or delegation.
- Stop and return FAIL on any mismatch. Do not repair.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
