# Exact dependency propagation plan — SCA-011 group 3

**Candidate instructions for the integration owner; not an approval record.** Proposed successor ID `DAG-011` was unused in the accepted `_DAG` tree on 2026-09-22. Recheck immediately before applying. The current approval/pointer remains DAG-010 until the authorized scope-change application records the successor.

This is a distinct dependency-authority act in the same authorized SCA-011 group-3 propagation, not a newly invented owner approval question. Ordinary scope-change review/closure still applies. The graph candidate and local mirrors may be propagated with truthful PENDING contract dependencies; their satisfaction is a later production-readiness question.

## Exact file actions

| Input candidate | Proposed landing / action |
|---|---|
| `DependencyEdges.csv` | New immutable `execution/_DAG/DAG-011/DependencyEdges.csv`: preserve the 1,487-row DAG-010 byte prefix and add exactly the 84 SCA011 rows. |
| `DeliverableNodes.csv` | New immutable `execution/_DAG/DAG-011/DeliverableNodes.csv`: preserve the 102-node DAG-010 byte prefix and add exactly the four new identities. |
| `StagedInterfaces.json`, `StageGraph.json`, `STAGED_INTERFACE_RATIONALE.md`, `staged-contracts/` | Retain by hash-linked reference to this SCA-011 evidence package; these expose the three DOCUMENT stage constraints outside the stock deliverable projection. |
| Stock `audit_dag.py` and `validate_candidate.py` results | Re-run on exact candidate bytes before application and on the final successor byte content. Store reports with the successor/provenance record; do not relabel a candidate report as a fresh successor run. |
| `LOCAL_MIRRORS_MANIFEST.json`, `LOCAL_MIRRORS_PROMOTION.json` | Apply the 20 candidate files to their exact proposed landings only after their baseline hashes and candidate source documents are checked by the owning bounded TASK. Execute the conditional Group 2 metadata replacements and verify applied hashes; after actual Group 3 acceptance and DAG-011 adoption, execute Group 3 replacements and verify accepted hashes. Added execution satisfaction remains PENDING/TBD. No unrelated local row is replaced by aggregate data. |
| `execution/_DAG/DAG-011/APPROVAL_RECORD.md` | Integration owner records the actual current human direction, SCA-011 gate result, reviewed candidate hashes, unchanged lifecycle/engineering/release limits, and the actual graph-authority propagation. Do not fabricate approval from this plan. |
| `execution/_DAG/DAG-011/PROVENANCE.json`, `MANIFEST.sha256`, `HANDOFF.md` | Record immutable predecessor hashes, accepted decomposition/amendment identity, exact delta, source evidence, actual validation/review and remaining contract witnesses. |
| `execution/_DAG/_LATEST.md` | After the actual group-3 propagation, name DAG-011 as approved authority with its approval record; retain DAG-010 in superseded history. No pointer edit during preparation. |

The existing DAG-010 files, original R4/R6 ledgers, historical owner words and SCA-009 annex remain unchanged. New node IDs and scope items must match the concrete candidate Deliverables register. No estimate/schedule recomputation, lifecycle promotion, hold release, native or engineering witness, or product acceptance follows from this graph propagation.

## Candidate acceptance facts for the eventual record

- 106 nodes, 1,571 rows: 1,486 ACTIVE and 85 RETIRED.
- 1,034 unique active deliverable-directed edges; no SCC, duplicate directed edge, bidirectional pair or dangling endpoint.
- All 1,487 predecessor rows / 102 predecessor nodes preserved as exact byte prefixes.
- 22 new trace anchors and 62 new execution rows. All new execution rows: required SEMANTIC_READY (proposed gate), proposed maturity TBD, satisfaction PENDING.
- 1,041 stage-expanded edges include all three new DOCUMENT bindings and four internal stage requirements. Seven negative probes establish that baseline damage, duplicate/dangling/false-satisfaction errors and all three reversed stage dependencies are detected.
- These numbers are candidate validation facts, not current graph-authority or implementation-completion claims.
