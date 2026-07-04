---
doc_id: SCA-005-PROPAGATION-PLAN
doc_kind: scope_change.propagation_plan
status: accepted
created: 2026-07-04
---

# SCA-005 Propagation Plan

## Gate Status

This propagation plan was accepted by the owner in-session on 2026-07-04.
Gate-5 truth edits were applied within the direct SCOPE_CHANGE write lane
below.

## Direct SCOPE_CHANGE Write Lane

Gate 5 executed only these write classes:

1. Amend PRD/PLAN/decomposition/coordination traceability surfaces named in
   `Amendment_Preview.md`.
2. Write final SCA-005 run artifacts in this folder.
3. Update `execution/_ScopeChange/_LATEST.md` only after validation and handoff
   state are complete.
4. Update D-29 to `RULED` only after the accepted handoff state exists.

## Execution Sequence

1. Re-read current `docs/PRD.md`, `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`,
   `docs/PLAN.md`, `plans/PLAN_2026-06-17_prd_completion.md`,
   `execution/_Coordination/_COORDINATION.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md`,
   D-21, D-29, and `_ScopeChange/_LATEST.md`.
2. Apply the accepted amendment preview to direct working surfaces.
3. Preserve immutable ruled history and add forward pointers/crosswalks rather
   than rewriting historical DEC rows.
4. Carry the D-21 Annex A crosswalk into the forward plan surface or a stable
   cited artifact.
5. Generate/update SCA-005 `RUN_SUMMARY.md`, `Handoff_State.md`,
   `Supersession_Delta.csv`, `Supersession_Map.csv` if applicable, and
   pre/post validation notes.
6. Update `_LATEST.md` to SCA-005 only if the SCA package is complete and its
   handoff state is explicit.
7. Update D-29 register state to `RULED` only after the accepted SCA-005
   handoff state exists.

## Downstream Handoff Lane

The following are not executed by SCOPE_CHANGE:

- dependency extraction reruns;
- DAG successor generation or approval;
- estimate or schedule snapshots;
- deliverable-local metadata alignment;
- app-dev package consumption beyond the D-30 metadata/validator record;
- live-binding implementation;
- release-quality, certification, code-compliance, professional, sealing,
  authentication, or issuance claims.

## Validation Lane

Gate-5 closeout should include:

- `git diff --check`;
- SCA artifact completeness check by direct inspection;
- grep for stale forward-facing D-21 contingency language in touched live
  surfaces;
- confirmation that immutable DEC history remains unchanged except for the
  accepted SCA revision note;
- bridge `coord-check --diff main..HEAD`;
- bridge status/status/drift/self-check closeout.

## Non-Writes

SCOPE_CHANGE does not write production deliverable content, code, schemas,
tests, lifecycle promotion beyond a separately authorized PREPARATION `OPEN`,
dependency-extract registers, release claims, professional claims, protected
standards data, or live-binding state.
