# SCA-007 Propagation Plan

Execution order (Gate 5, after fan-in validation of nodes A/B/C):

1. **Relocation set** (parent, serialized): `git mv docs/PRD.md
   docs/_history/PRD_v0.1.md` and re-key only its Forward Authority Note
   (body verbatim); install the validated `PRD_v0.3_DRAFT.md` as
   `docs/PRD.md`; write the redirect stub at
   `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`.
2. **Deliverable-local edits** (one bounded child; write scope = the named
   `DEL-*` folders): DEL-09-04, DEL-10-04, and any prover-machinery currency
   notes from `Amendment_Actions.csv`, each with a dated `_STATUS.md`
   history line citing `DEC-080`.
3. **Shared governed surfaces** (parent, serialized integration owner):
   `loop/WORKPLAN_2026-07-10_piping_loop.md` yardstick lines; `docs/PLAN.md`
   pointer rows; `execution/_Coordination/_COORDINATION.md` stage-record
   re-expression (no stage advancement); D-46 packet supersession note;
   `execution/_ScopeChange/_LATEST.md` → SCA-007;
   `execution/_Decomposition/SOFTWARE_DECOMP.md` revision v0.10 notes.
4. **Closeout**: `RUN_SUMMARY.md` + `Handoff_State.md` here; AgentRuns
   returns log; loop receipt (Receipt-49, parent Receipt-48) + receipt
   validator; repo-wide self-check; practitioner-harness pytest;
   `git diff --check`; grep verifications per the plan; PR; session-
   authorized merge.

Deliberately not propagated: pre-amendment `docs/PRD.md` citations inside
ruled history (receipts, run records, ruled packets, §12 rows, plans/
archive, prior SCA snapshots) — governed by the citation-resolution note in
the relocated PRD and the D-21 Annex A crosswalk.
