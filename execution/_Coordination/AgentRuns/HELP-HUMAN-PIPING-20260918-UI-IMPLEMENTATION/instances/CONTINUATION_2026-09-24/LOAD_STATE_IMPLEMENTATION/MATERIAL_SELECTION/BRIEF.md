# TASK — per-element constitutive selection

Actual parent /root/physics_resume; executing reused handle /root/physics_resume/exact_authoring. Sole production write: core/product_physics/src/case_state/material.rs, with in-module meaningful tests. Sole evidence writes this MATERIAL_SELECTION directory. Parent owns all existing facade/pressure_material and new public DTOs.

Implement a private reusable pure selector for one actual user MaterialInput and one element's explicit selection: explicit base E/nu with applicability reference; exact named point on that material; piecewise-linear temperature selection with valid exact endpoints and no extrapolation. Reuse existing normalized quantity/units authority and `pressure_exact::IsotropicENu` domain/derivedG; no guessed nu or independently consumed recordedG. Retain actual material ID, consumed point IDs/temperatures and selected normalizedE/nu/G. Same material ID may resolve differently for two element states; never synthesize material aliases or mutate the model/globalmaterial map.

Suggested interface: private MaterialSelection enum; optional actual operating temperature and explicit analysis-basis override; `select_for_member(...) -> Result<ResolvedMemberMaterial, MaterialSelectionError>`. Send exact API early. The parent may pass normalized quantities, but any conversion here must use existing units; specify this precisely. Base and every consumed point/bracket pair must meet producer IsotropicENu representability domain. A point whose explicit temperature differs from known actual operatingT requires explicit reason/provenance override; absence of pointT cannot prove a propertyatoperatingT. Do not infer actualT from a point. Old case-wide resolver semantics stay untouched.

Controls: same actualmaterial two selected temperatures E200/100GPa; exactendpoints/strictadjacentinterior; duplicate normalizedT, missingpair/outsidebracket, wrongmaterialpoint, unknownselection, mismatchwithoutoverride, equivalentunits, retainedG ignored, adjacentrepresentabilitydomain. Keep old fullprecision reference criteria. No build until parent coordinates; parser/pure scalar checks allowed.

Resolve REPO_ROOT and WORKING_ROOT as parent BRIEF.md. The only source checkout is branch codex/piping-load-states-20260925 atc278; no engine/primary writes. Read full Root/Piping AGENTS, LOOP_INIT, parent BRIEF/OWNERSHIP and the full TASK role below. No delegation, public contract/schema/facade edits, Git, Cargo/npm/native/UI builds or external searches. Parent controls integration/build targets. Read the selected6 design documents via immutable Git9e8a55daecdeb9669131fd3e53c0e0303ee550d6 at continuation CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES; do not copy a whole source forest or repeat closed research/gates. Runtime machine paths/commands/observations go only under your evidence _run_records directory; current briefs/returns use portable paths.

Use user-supplied/invented test quantities only. Preserve protected1e-9 criteria and old methods. Unsupported/range conditions are explicit unfinished cases, not successful zero outputs. Read-only context may extend to the existing pressure_exact/material/units/frame code; no second unit catalog. Send an early exact private API before final implementation, then actual source/checks/limits and hash-bound return.

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
