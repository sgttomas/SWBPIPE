# D-29 - GATE PACKET: Execute v0.2 SCOPE_CHANGE propagation

**Status:** RULED / SCA-005 ACCEPTED AND APPLIED 2026-07-04.
**Date prepared:** 2026-07-04
**Decision ID:** D-29
**Prepared by:** bridge work loop agent, at owner direction to prepare and execute all agent-lawful coordination/control work on D-29.
**Structural precedent:** Follows the D-21 packet's §8 mechanism and the accepted SCA-004 snapshot pattern for SCOPE_CHANGE propagation.

This packet prepared the SCOPE_CHANGE propagation that D-21/DEC-056 left as
residual work. After owner acceptance of the SCA-005 amendments, Gate 5
modified the accepted PRD/PLAN/coordination/decomposition forward-authority
surfaces and `projects/chirality-piping/execution/_ScopeChange/_LATEST.md`.
It did not modify deliverable metadata, source code, schemas, tests, lifecycle
state, release claims, professional claims, app-dev package consumption, or
live-binding state.

## Gate-5 acceptance and execution

`agents/AGENT_SCOPE_CHANGE.md` requires impact assessment and propagation-plan
approval before decomposition truth or variant-local metadata are modified.
The owner ruling of 2026-07-04 selected O-A for this lane. That selection
authorized preparing SCA-005 and routing it for acceptance. The owner accepted
those gate artifacts in-session on 2026-07-04 ("I accept the D-29
amendments"), and Gate 5 applied the accepted forward-authority edits within
the stated non-write boundary.

## Verified basis

| Fact | Source |
|---|---|
| D-21 adopted the full v0.2 milestone set and FR renumber, with the D-21 packet Annex A as the mandatory forward crosswalk. | `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-21_prd_scope_change_v0_2_milestone_set.md`; `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-056 |
| DEC-056 says SCOPE_CHANGE propagation executes separately under the governed SCA workflow and that no PRD, plan, coordination, decomposition, or profile file changed under D-21 itself. | `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-056 |
| SCOPE_CHANGE does not write production deliverable content, code, schemas, tests, lifecycle promotion beyond PREPARATION `OPEN`, dependency-extract registers, commits, release claims, or professional claims. | `projects/chirality-piping/execution/_ScopeChange/SCA-004_2026-05-18_0000/Propagation_Plan.md` |
| The latest accepted SCA snapshot before Gate 5 was SCA-004; after Gate 5 it is SCA-005. | `projects/chirality-piping/execution/_ScopeChange/_LATEST.md` |
| D-29 is the residual row for executing D-21/DEC-056 propagation. | `projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md` |

## Accepted SCA-005 action set

The accepted SCOPE_CHANGE amendment SCA-005 executed a metadata/authority
propagation of D-21/DEC-056:

1. Record the change request and owner ruling in the SCA decision log.
2. Produce an impact assessment that classifies touched surfaces as
   authoritative working surfaces, snapshot/handoff artifacts, or downstream
   derivative packages.
3. Amend forward-facing PRD/plan/decomposition wording so D-21 is no longer
   described as contingent or held, while preserving immutable ruled history
   that used v0.1 numbering.
4. Carry the D-21 Annex A crosswalk forward as the governing traceability bridge
   for future work.
5. Create the immutable SCA snapshot, update `projects/chirality-piping/execution/_ScopeChange/_LATEST.md`, and
   leave downstream dependency/DAG/estimate/schedule/content reruns as explicit
   handoff work unless separately selected.
6. Update D-29 from `AWAITING_RULING` to `RULED` only after the SCA package is
   accepted and its handoff state is written.

## Accepted SCA-005 snapshot

The SCA-005 snapshot is accepted at:

`execution/_ScopeChange/SCA-005_2026-07-04_0000/`

Accepted artifacts:

- `Brief.md`
- `Impact_Assessment.md`
- `Amendment_Preview.md`
- `Propagation_Plan.md`
- `Amendment_Actions.csv`
- `Decision_Log.md`
- `Handoff_State.md`
- `ACCEPTANCE_RECORD.md`
- `RUN_SUMMARY.md`
- `Pre_Change_Coverage.json`
- `Post_Change_Coverage.json`
- `Supersession_Delta.csv`
- `Supersession_Map.csv`

This bundle is now the active accepted amendment snapshot. `_LATEST.md` points
to SCA-005 after the Gate-5 execution/validation pass completed.

## Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Execute SCA-005 propagation as scoped above.** | Brings PRD/plan/decomposition traceability surfaces into alignment with DEC-056 while keeping downstream deliverable/code/schema work out of the SCA pass. |
| **O-B** | **Pointer-only propagation.** Add only a narrow pointer to DEC-056 and Annex A, leaving most contingent language untouched. | Lower edit volume but likely leaves confusing live-map drift and weakens the value of adopting D-21. |
| **O-C** | **Defer.** | D-21 remains ruled but its propagation residue stays open; R6/R7 planning weight remains hard to consume. |

## Recommendation

Recommend **O-A**.

O-A is the only option that actually discharges the D-29 residual while staying
inside SCOPE_CHANGE's non-write boundary for downstream deliverable/code/schema
work.

## Human ruling

**Ruling recorded:** O-A selected by owner (Ryan Tufts), 2026-07-04.

Ruling/selection record:
`execution/_Coordination/_DECISIONS/D-29_RULING_2026-07-04.md`.

Effect: SCA-005 was accepted and Gate 5 applied the forward-authority truth
edits. D-29 is closed as RULED. Downstream DAG/dependency/estimate/schedule and
deliverable-local metadata refreshes remain handoff work for their owning
workflows.
