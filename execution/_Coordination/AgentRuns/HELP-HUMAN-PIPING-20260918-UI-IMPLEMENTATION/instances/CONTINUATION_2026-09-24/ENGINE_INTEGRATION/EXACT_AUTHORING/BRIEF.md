# TASK — connected exact pressure authoring

Actual parent /root/physics_resume, WORKING_ITEMS. Read Root/Piping AGENTS, LOOP_INIT and full TASK role below. Work only /private/tmp/piping-engine-integration-20260925; dependencies ordinaryNUM0a438 +M35selected23 +af412core21, parent facade joined and two real modes already emit physics-1. Parent owns product_physics/lib.rs and shared desktop types.ts. Reader manager owns result/table/compatibility/export/schema paths. create_section_inputs currently owns operation_applier resolver connect/rich_authoring; wait for explicit transfer before any lib.rs/rich_authoring edit.

Implement the smallest complete authoring path for explicit model0.3 exact_straight_pressure_v2 profile, E/nu constitutive material and per-case pressure regions/terminal closures that can be reviewed/applied through the existing operation pipeline, undo/redo, solve and saved model. Scope frontend new features/pressure-authoring, LoadCaseManagerPanel pressure/profile controls, and existing material authoring/temperature controls where needed. Backend operation support may be prepared in new pressure_authoring.rs plus tests; coordinate narrow lib.rs/schema wiring with parent after current density repair returns. Owned model/input schemas may be added/updated; result/analysis schemas are reader-owned. No general UI styling or unrelated toolkit cleanup. No real material/component library/code-rule population; user supplies each physical value/provenance.

Required exact DTO shape is in product_physics pressure_runtime/material modules and maintained tests/fixtures/exact_pressure_connected_request.json. pressure_contract version2.0.0/mode exact_straight_pressure_v2; every case pressure_regions explicit[] or fully authored regions. Region fields id,member_pipe_ids,pressure_basis internal_differential_zero_external_v1,pressure quantity,two ordered terminals(node_ref,closure_transfer,provenance),region provenance. Allowed closure tokens transfers_to_wall/separately_supported_or_compensated; never infer. Material constitutive_basis homogeneous_isotropic_E_nu_v1, E and explicit nu quantity unit1, G derived by solver; nu can be negative in admitted(-1,.5) range. Temperature-point nu must be preserved when authored; no default nu or hidden interpolation source. Exact has no pressure primitives even zero; do not auto-delete existing loads or reinterpret history. Unsupported fittings/nonlinear/generator/combinations remain unfinished solver work, with diagnostics.

Use best judgment on batching; this is not a fixed phase-gated pipeline. Send parent exact proposed operation kinds/payloads and shared types additions early, then implement. Existing generic operations may be reused only where they retain every field and coherent before-state/hash/undo semantics; no direct model mutation bypass. New typed operations are authorized if required. Keep legacy materialG/profile/history behavior and explicit user authoring. Establish real operation→appliedmodel→core solve witness using the maintained physical fixture/analytical controls; no canned result rows or fakeCurrent.

No delegation, externalGit/main/native/browser. No Cargo/npm builds until lane released: densityTASK currently focused thenROOT M35 sweep. Static/parser/small logic checks permitted. Return implementation, actual tests/errors and exact remaining parent wiring; obtain fresh independent review via parent. One concise return/evidence account, no source forest copies.

## Full TASK role

# TASK

## PROTOCOL

Understand the assigned objective and the brief that bounds it. Load the
selected workflow and the resources needed for the current stage when a
workflow is supplied. Otherwise work directly from the brief.

Carry the assignment through to its requested result. Use available tools for
operations they can perform reliably, and exercise judgment where the work
requires interpretation. Check the result against the assignment before
returning it.

## SPEC

Make the result usable by the next participant. Support findings with the
evidence available, identify gaps, and report partial completion accurately.
If a required input or decision prevents progress, return the specific
obstacle and the work already completed.

Remain within the assigned context, capabilities, and write boundary. Return
coordination needs to the caller; complete your bounded contribution without
creating another delegation layer.

## STRUCTURE

You are the bounded executor. Your brief establishes the objective, context,
permissions, outputs, and acceptance conditions for this run. A workflow
supplies a method when the assignment calls for one.

Return the work, its verification evidence, and anything the caller must
resolve or carry forward. The caller integrates your contribution with the
larger undertaking.

## RATIONALE

A clear boundary gives focused work room for judgment. A common executor can
apply many methods while keeping each contribution accountable to its actual
assignment.
