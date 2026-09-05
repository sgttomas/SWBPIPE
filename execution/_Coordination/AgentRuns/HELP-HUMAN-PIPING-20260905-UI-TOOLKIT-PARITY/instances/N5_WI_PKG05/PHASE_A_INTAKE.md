# PKG-05 Phase A intake snapshot

RunID: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY
Instance: N5_WI_PKG05; role WORKING_ITEMS (Agent 1); parent HELP_HUMAN.
Delegation class: delegated-harness-native; role enforcement instruction-asserted.
Model attribution: inherited parent model; exact runtime model identifier not exposed to this instance.
Accepted basis: parent-declared decomposition revision 0.12 / SCA-009 and DAG-010; checkout basis 740569598f9d00440636b8ea25264127f418e4ec. Annex historical candidate header is consumed through parent's accepted SCA-009 basis, not independently promoted here.
Scope: DEL-05-01 SOW-013 OBJ-003 automatic self-weight generation row 22; DEL-05-02 SOW-014 OBJ-003/005 is dependency/exclusion context only. Both contracts are SOW_V1. No algebra behavior change.
Authority: owner implementation instruction transcribed in run OWNER_DIRECTION.md; parent's Phase A brief permits design evidence only.
Reads: actual AGENTS.md, project AGENTS.md, AGENT_WORKING_ITEMS.md, software-workflow.json, SOFTWARE_WORKFLOW_PROFILE.md, DEL-05-01/02 ScopeOfWork.md, context/status, CONTRACT.md, product_physics and operation_applier implementation. Writes: this instance directory and DEL-05-01 candidate records only.
Invariants: OPS-K-DATA-1/2/3, UNIT-1, MECH-1/2, SOLVER-1, AGENT-1/2/3/4. Applicable architecture AB-00-01/02/03/06/08.

## Proposed interface

Pure function generate_self_weight_operations(model: &PreviewModel, request: SelfWeightRequest) -> Result<SelfWeightOperationPlan, diagnostics>.
Request explicitly supplies case_id, label, nonempty pipe_refs, gravity {value,unit,axis}, and provenance. Axis is global_x/global_y/global_z; gravity is signed finite nonzero acceleration with compatible explicit unit. Pipe IDs must be unique and resolved; duplicate/existing case IDs block. No inferred gravity direction or constant, density, dimensions, code factors, or support restraint.

Output contains ordered creation changes (case shell first, sorted pipe loads second), source snapshot binding supplied by caller, and mass/gravity provenance. Standard case payload: id, label, kind=primitive_user_load, status=draft, provenance, primitive_loads=[]. Standard primitive payload: stable id, category=distributed_force, target={type:element,pipe:pipe_id}, direction=axis, magnitude={value:computed_signed_force_per_length,unit:N/m}, dimension=force_per_length, provenance. Category weight is NOT an accepted create_primitive_load token and must not be emitted. Full operation author/session metadata remains PKG-16 adapter responsibility or explicit request metadata, never synthesized approval.

IDs derive injectively from length-prefixed case/pipe IDs, not lossy stable_suffix or ambient counters/time. Collision-check across existing load identities before output. Input selection order must not affect output. Return no partial plan on invalid selection/input.

Normalize cloned selected pipe quantities through existing unit conversion before reusing compute_pipe_mass_per_length. Do not normalize the entire model: unrelated invalid material/load/support data must not block authoring. Validate positive finite OD, thickness, material density, effective wall; validate supplied optional mill tolerance nonnegative, contents density nonnegative, insulation thickness/density both present or both absent and nonnegative, and all compatible units. Require source provenance on selected pipes plus request. Reject nonfinite computed mass/intensity (including overflow). Missing optional contributions follow existing explicit absence semantics and are enumerated in returned source summary, never claimed as complete plant/component weight.

No support loads, imposed displacements, hanger forces, or component point weights are generated. Target selected pipe spans only. Existing support references and existing case contents remain unchanged. A model with unresolved selected pipe endpoints must block; unrelated dangling objects need not be inspected by generator, while normal operation engine remains authoritative.

## Minimal reusable location and fence request

New core/product_physics/src/self_weight.rs accesses existing parent-module mass helper and normalization functions without duplicating formulas. Parent integration owner adds pub mod self_weight in core/product_physics/src/lib.rs. Need parent authorization because product_physics is shared read-only at intake. PKG-07 owns UI and bridge transport; PKG-16 owns operation application. Avoid introducing primitive_loads -> product_physics dependency cycle. No new Cargo crate/dependency necessary.

PKG-16 must apply ordered operations transactionally to a private working model and publish one final checkpoint only when every operation validates; stale/collision failure leaves session unchanged. This preserves one mutation route. If batch semantics are unavailable, whole-plan capability remains unavailable until that owner integrates them; do not partially create an empty case before a later load fails.

## Work graph and acceptance

A intake (manager, design records only) -> parent interface/fence acceptance -> B Agent2 implementation in self_weight.rs -> C fresh read-only Agent2 software-code-review of frozen 100% diff -> D manager validation/fan-in -> parent PKG-16/07 integration and cross-package checks. Existing-capability wiring acceptance is a predecessor to Tier-3 source implementation. B/C serial; package UI/op owners integrate after accepted interface, with serialized lib.rs ownership.
Tests: non-default positive/negative axis acceleration; entered SI/US unit equivalence; metal-only and contents/insulation/mill tolerance; absent/invalid/NaN/overflow inputs; duplicate/missing selection and ID collisions; deterministic reordering; source unchanged; sequential standard operation application fixture and failed-batch rollback under PKG-16.
No tests executed in design-only intake. Profile maps core changes to piping-pytest and evidence-sweep plus always harness-self-check; targeted Cargo test and fresh review precede parent full suite.

## Handoff and closure

Derivative candidate design, not decomposition truth. Intake complete; implementation NOT started. Parent actions: accept or amend interface, grant precise new-module fence and serial shared export owner, coordinate PKG-16 transactional operation plan and PKG-07 transport/UI. No new human engineering-policy gate identified for this explicit-input mechanics-only generator; full component self-weight or automatic standards-based combinations would be new scope and are excluded. No lifecycle, status, accepted pointer, receipt, commit, or push change. Required derivative closure and comprehensive audit deferred to parent run; rerun on schema/operation contract or mass-helper changes.
