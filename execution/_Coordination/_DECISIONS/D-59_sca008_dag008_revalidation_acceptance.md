# D-59 — SCA-008 / DAG-008 revalidation acceptance

**Status:** RULED AND EFFECTIVE ON MERGE
**Date:** 2026-07-28
**Decision ID:** D-59
**Examined basis:** `main@4cd25b348196f7e6dfa925d8c7938184924cb383`
**Derivative:** `execution/_Evaluation/DepClosure/CLOSURE_SCA008_DAG008_REV011_REVALIDATION_2026-07-28_0901/`
**Derivative manifest SHA-256:** `3be1e5a5d5f629b39be656799a7e100cd7fe7a615e6188cd78198ae81e3876f0`

## Owner direction of record

> “Finish out your plan now (attaining your goal) with self merge of PRs and
> auto approve for owners rulings, which should still be recorded in the usual
> manner with your recommendation standing as what I approved.”

The Piping Agent 1 recommended acceptance of the completed revalidation. Under
the quoted direction, that recommendation stands as the owner's approval.
This record transcribes the approved disposition without enlarging it.

## Ruling

The owner accepts the SCA-008 / decomposition-revision-0.11 DAG-008
dependency-closure revalidation derivative identified above.

The accepted evidence proves:

- SCA-008 changed no dependency register or DAG path;
- all 101 accepted deliverables remain identical to the DAG-008 node set;
- DAG-008 retains 1,480 canonical rows: 1,395 `ACTIVE` and 85 `RETIRED`;
- the graph retains 972 unique active directed edges and the same 15
  topological waves;
- it has zero orphan endpoints, strongly connected components, duplicate
  active edges, or bidirectional pairs; and
- local active topology remains identical when the 30 approved
  aggregate-only duplicate retirements are preserved.

DAG-008 is therefore `CURRENT_BY_REVALIDATION` against decomposition
revision 0.11. DAG-008 remains immutable, and
`execution/_DAG/_LATEST.md` remains on DAG-008.

The derivative was produced at `404e47c16a88e7ffdc6d1fc5fac61ebb6864211e`.
No Piping path changed between that basis and the examined basis above, so the
accepted findings remain current.

## Boundaries

This ruling closes only the SCA-008 dependency/DAG downstream rerun. It
authorizes no dependency extraction, dependency-register edit, DAG-009,
edge, node, topology, satisfaction, lifecycle, selection, implementation,
estimate, schedule, release, professional-reliance, product, runtime, or
client-status effect. The DEC-063 / DEC-091 / DEL-16-04 reconciliation
remains separately governed by D-60.

## Effective-state rule

This ruling becomes effective only when this record, its register row, the
immutable derivative, and the updated DepClosure pointer are durably merged
to shared `main`. After merge, correction requires a successor decision; the
accepted derivative is not rewritten.
