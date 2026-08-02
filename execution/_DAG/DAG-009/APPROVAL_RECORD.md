---
doc_id: DAG-009-APPROVAL-RECORD
doc_kind: coordination.approval_record
status: approved_active_graph_authority
created: 2026-08-02
approved: 2026-08-02
approved_by: human_project_authority
approved_decomposition: execution/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.7"
dag_path: execution/_DAG/DAG-009/
approval_scope: dependency_satisfaction_currency_refresh
candidate_treatment: separate_non_authoritative_worklist_no_current_candidates
topology_basis: approved_DAG-008_unchanged
closure_basis: R23_five_consumer_dependency_registers
accepted_proposal_commit: d6431139384232008d1f73f72f907bc9738db103
accepted_proposal_merge: 84a5a1277f3f8bbe2821d131b9a8ba309b1b5aaa
type2_dispatch: not_authorized_by_approval_record
lifecycle_changes: not_authorized
---

# DAG-009 Approval Record

## Owner decision

On 2026-08-02, the human project authority, Ryan Tufts under K-AUTH-1,
directed:

```text
Merge PR #471 and if DAG-009 is ready to be activated, then I accept this and want you to update all pointers.
```

PR #471 merged the proposal-only head
`d6431139384232008d1f73f72f907bc9738db103` as
`84a5a1277f3f8bbe2821d131b9a8ba309b1b5aaa`. EVALUATION then returned PASS
for activation readiness on that exact merged basis. The condition in the
owner direction therefore passed. This record applies the resulting acceptance
of DAG-009 and updates both live DAG pointer surfaces: this bundle's local
`_LATEST.md` and the root `execution/_DAG/_LATEST.md`.

## Accepted basis

- Approved predecessor: `execution/_DAG/DAG-008/`.
- Owner-adopted and amended R23 O-A mechanism, its two-part evidence gate, N2
  fan-in, quarantined N3 EVALUATION proposal, N4 byte-copy materialization, N5
  validated proposal-only closeout, and Receipt-85 under
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260802-DEC092-DEPENDENCY-REFRESH-R23/`.
- Accepted proposal commit: `d6431139384232008d1f73f72f907bc9738db103`,
  merged proposal-only as `84a5a1277f3f8bbe2821d131b9a8ba309b1b5aaa`.
- Decomposition truth: revision `0.7` at
  `execution/_Decomposition/SOFTWARE_DECOMP.md`, unchanged by this acceptance.
- The five SHA-bound local consumer registers and indexes identified by
  `PROVENANCE.json`; their accepted bytes remain the R23 source for the 12
  execution-row closures and one objective-trace anchor normalization.

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
| SATISFIED rows | 800 |
| TBD rows | 321 |
| PENDING rows | 118 |
| NOT_APPLICABLE rows | 241 |

The exact delta from DAG-008 is limited to the 12 execution-row closures and
one objective-trace anchor normalization enumerated in
`DAG-009_APPROVAL_REVIEW_PACKET.md`. Only `SatisfactionStatus`, `LastSeen`, and
`Notes` change for those 13 dependency IDs. The 12 failed candidates and five
named holds remain ordered-field identical. Node identity, edge membership and
meaning, machine graph edges, topology, lifecycle `Status`, and decomposition
truth are unchanged.

## Approval conditions and effect

- DAG-009 supersedes DAG-008 as dependency coordination authority when this
  activation tranche lands.
- Deliverable lifecycle authority remains in deliverable-local `_STATUS.md`
  files. This approval does not perform a lifecycle transition, alter memory,
  or remove a `Remaining` item.
- Dependency unblocking and selectability must be re-derived from DAG-009 after
  activation. Work selection, brief adoption, and implementation dispatch
  remain separate governed loop acts.
- The 12 failed candidates and five holds remain unsatisfied as recorded.
- Candidate and duplicate worklists retain their recorded non-authoritative
  treatment.
- No DEC-092 implementation or product work is authorized by this acceptance.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
- After this activation metadata and manifest update land, the accepted DAG-009
  package is immutable. Later dependency truth changes require a successor
  snapshot rather than in-place graph mutation.

## Validation basis

Before recording activation, EVALUATION returned readiness PASS on the exact
proposal merge. CHANGE then revalidated the proposal manifest and immutable
bytes, dependency schema, strict canonical DAG audit, JSON and topology
consistency, the exact DAG-008-to-DAG-009 delta, consumer SHA pins, local and
root pointer currency, receipt structure, claims/path anchors, repository
self-check, practitioner-harness pytest, exact containment, and diff checks.
