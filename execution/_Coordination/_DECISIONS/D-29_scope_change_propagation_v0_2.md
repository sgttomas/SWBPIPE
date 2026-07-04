# D-29 - GATE PACKET: Execute v0.2 SCOPE_CHANGE propagation

**Status:** O-A selected 2026-07-04; `AWAITING_SCA_GATE_ACCEPTANCE` for SCA-005 impact assessment, amendment preview, and propagation-plan acceptance before truth edits.
**Date prepared:** 2026-07-04
**Decision ID:** D-29
**Prepared by:** bridge work loop agent, at owner direction to prepare and execute all agent-lawful coordination/control work on D-29.
**Structural precedent:** Follows the D-21 packet's §8 mechanism and the accepted SCA-004 snapshot pattern for SCOPE_CHANGE propagation.

This packet prepares the SCOPE_CHANGE propagation that D-21/DEC-056 left as
residual work. It does not itself modify `docs/PRD.md`, `docs/PLAN.md`,
`SOFTWARE_DECOMP.md`, `projects/chirality-piping/execution/_ScopeChange/_LATEST.md`, deliverable metadata, source
code, schemas, tests, lifecycle state, release claims, professional claims, or
live-binding state.

## Why truth edits are still gate-held

`agents/AGENT_SCOPE_CHANGE.md` requires impact assessment and propagation-plan
approval before decomposition truth or variant-local metadata are modified.
The owner ruling of 2026-07-04 selected O-A for this lane. That selection
authorizes preparing SCA-005 and routing it for acceptance, but the SCA-005
impact assessment, amendment preview, and propagation plan must be accepted
before `docs/PRD.md`, `docs/PLAN.md`, `plans/PLAN_2026-06-17_prd_completion.md`,
`execution/_Coordination/_COORDINATION.md`, `SOFTWARE_DECOMP.md`, or
`execution/_ScopeChange/_LATEST.md` are changed.

## Verified basis

| Fact | Source |
|---|---|
| D-21 adopted the full v0.2 milestone set and FR renumber, with the D-21 packet Annex A as the mandatory forward crosswalk. | `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-21_prd_scope_change_v0_2_milestone_set.md`; `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-056 |
| DEC-056 says SCOPE_CHANGE propagation executes separately under the governed SCA workflow and that no PRD, plan, coordination, decomposition, or profile file changed under D-21 itself. | `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-056 |
| SCOPE_CHANGE does not write production deliverable content, code, schemas, tests, lifecycle promotion beyond PREPARATION `OPEN`, dependency-extract registers, commits, release claims, or professional claims. | `projects/chirality-piping/execution/_ScopeChange/SCA-004_2026-05-18_0000/Propagation_Plan.md` |
| The latest accepted SCA snapshot is SCA-004. | `projects/chirality-piping/execution/_ScopeChange/_LATEST.md` |
| D-29 is the residual row for executing D-21/DEC-056 propagation. | `projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md` |

## Proposed SCA-005 action set

If ruled, open the next SCOPE_CHANGE amendment as SCA-005 and execute a
metadata-only propagation of D-21/DEC-056:

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

## Prepared SCA-005 gate bundle

The SCA-005 review bundle has been prepared at:

`execution/_ScopeChange/SCA-005_2026-07-04_0000/`

Prepared artifacts:

- `Brief.md`
- `Impact_Assessment.md`
- `Amendment_Preview.md`
- `Propagation_Plan.md`
- `Amendment_Actions.csv`
- `Decision_Log.md`
- `Handoff_State.md`

This bundle is not the active accepted amendment snapshot. `_LATEST.md` still
points to SCA-004 until the owner accepts the SCA-005 gate artifacts and the
Gate-5 execution/validation pass completes.

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

Effect: prepare and route SCA-005 for acceptance. Truth edits remain held until
the SCOPE_CHANGE gate artifacts are accepted.
