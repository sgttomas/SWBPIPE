# TASK — normalized thermal and fit kernel

Actual parent /root/physics_resume; executing reused handle /root/physics_resume/composite_receipt. Sole production write: core/product_physics/src/case_state/thermal.rs, with in-module meaningful tests. Sole evidence writes this THERMAL_FIT directory. Parent owns case_state/mod.rs/lib.rs and units/public DTOs.

Implement a private pure normalized-input kernel from the selected definitions: unchanged reference; explicit engineering interval strain; constant-alpha interval; free-length states for engineering secant (constant/table), engineering dilation table, differential-per-datum-length coefficient table, logarithmic-per-current-length coefficient table. Temperatures arrive normalizedKelvin, coefficients normalizedinverseKelvin, intervalchangesKelvin. Fit is explicit none, engineering fitstrain or signed natural-length change divided by positive reference assembly length. Compute thermal and fit stretches/strains separately and stable total eigenstrain = efit + ethermal + efit*ethermal. No pressure term, assembly or force is computed here.

Suggested interface: owned private enums ThermalInput/NormalizedExpansionLaw/FitInput and `resolve_strain(...) -> Result<ResolvedStrain, StateMathError>`. You may refine names with an early API message. Expose consumed point/segment indices and definition/datum/installation/operating values for parent evidence; use pub(crate), no public serialization contract. Parent resolves IDs/units/model coverage. Stable near-unity evaluation must avoid subtracting rounded long lengths. Use exact piecewise-linear integral arithmetic ordering and expm1 where appropriate; endpoints valid, no extrapolation, duplicate/unknown data/nonpositive stretches/range failure explicit. Do not fork existing annulus or exact-sum helpers. Explain arithmetic assumptions/bounds; do not claim arbitrary-range accuracy from a finite result.

Meaningful controls: design43/25009 datum example; constant interval0.0008; coefficient-integral0.0015 versus exp(0.0015)-1; interval reversal/splitting and exactendpoints; fitcold+40000/hot-89976 reference strain and noaccumulation; cancellation/smallstrain and malformed/range inputs. Independent reference TASK owns maintained analytical fixtures; your tests must not use observed product output as oracle. No standalone build until parent coordination; source/parser/pure scalar checks allowed and honestly labeled.

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
