---
doc_id: DAG-008-APPROVAL-RECORD
doc_kind: coordination.approval_record
status: approved_active_graph_authority
created: 2026-07-22
approved: 2026-07-22
approved_by: human_project_authority
approved_decomposition: execution/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.7"
dag_path: execution/_DAG/DAG-008/
approval_scope: dependency_satisfaction_currency_refresh
candidate_treatment: separate_non_authoritative_worklist_no_current_candidates
topology_basis: approved_DAG-007_unchanged
closure_basis: refreshed_DEL-08-01_and_DEL-10-05_local_registers
accepted_proposal_commit: 43d1306298758c0c5b2ec59d4b1f7dfcdc6b8298
accepted_proposal_merge: 751a880b43c34d8342730ca559ec3c91a6ba9251
type2_dispatch: not_authorized_by_approval_record
lifecycle_changes: not_authorized
---

# DAG-008 Approval Record

## Owner decision

On 2026-07-22, the human project authority, Ryan Tufts under K-AUTH-1,
performed these two explicit acts:

```text
(1) accept DAG-008 as immutable successor to DAG-007;
(2) authorize projects/chirality-piping/execution/_DAG/_LATEST.md to move DAG-007 -> DAG-008.
```

The first act accepts the validated satisfaction-currency snapshot represented
by `execution/_DAG/DAG-008/`. The second act separately authorizes the root DAG
pointer to make that accepted snapshot the current dependency authority.

## Accepted basis

- Approved predecessor: `execution/_DAG/DAG-007/`.
- Accepted advisory audit:
  `execution/_Evaluation/DEPENDENCY_READINESS_AUDIT_2026-07-21_R15/`.
- Governed local refreshes: the DEL-08-01 and DEL-10-05 dependency registers
  and run records cited by `PROVENANCE.json`.
- Candidate evaluation and exact application map:
  `execution/_Evaluation/DAG008_CANDIDATE_EVALUATION_2026-07-22_R15/`.
- Accepted proposal commit: `43d1306298758c0c5b2ec59d4b1f7dfcdc6b8298`,
  merged as proposal-only at `751a880b43c34d8342730ca559ec3c91a6ba9251`.
- Decomposition truth: revision `0.7` at
  `execution/_Decomposition/SOFTWARE_DECOMP.md`, unchanged by this acceptance.

## Accepted graph facts

| Fact | State |
|---|---:|
| Packages represented | 18 |
| Deliverable nodes represented | 101 |
| Edge register columns | 31 |
| Edge rows | 1480 |
| Active register rows | 1395 |
| Retired rows | 85 |
| Candidate edge rows | 0 |
| Unique active directed graph edges | 972 |
| Duplicate active directed edges | 0 |
| Bidirectional active pairs | 0 |
| Active-edge cycle status | ACYCLIC |
| Active plus candidate SCC warnings | 0 |
| Topological waves | 15 |
| SATISFIED rows | 788 |
| TBD rows | 329 |

The delta from DAG-007 is limited to the 13 closure-currency rows enumerated in
`DAG-008_APPROVAL_REVIEW_PACKET.md`. Node identity, edge membership and meaning,
machine graph edges, topology, lifecycle `Status`, and decomposition truth are
unchanged.

## Approval conditions and effect

- DAG-008 supersedes DAG-007 as dependency coordination authority when the
  separately authorized root pointer update lands with this record.
- Deliverable lifecycle authority remains in deliverable-local `_STATUS.md`
  files. This approval does not perform a lifecycle transition or remove a
  `Remaining` item.
- Dependency unblocking is re-derived from DAG-008 after activation. Any work
  selection, brief adoption, or implementation dispatch remains a separate
  governed loop act.
- D-45 remains separately `AWAITING_RULING`; this approval has no D-45 effect.
- Candidate and duplicate worklists retain their recorded non-authoritative
  treatment.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
- After this activation metadata and manifest update land, the accepted DAG-008
  package is immutable. Later dependency truth changes require a successor
  snapshot rather than in-place graph mutation.

## Validation basis

Before recording activation, CHANGE revalidated the dependency schema, strict
canonical DAG audit, JSON artifacts, topology and cycle consistency, 13-row
local-register equivalence, proposal and activation manifests, pointer currency,
status enumeration, claims, path anchors, repository self-check, receipt
structure, exact containment, and diff checks.
