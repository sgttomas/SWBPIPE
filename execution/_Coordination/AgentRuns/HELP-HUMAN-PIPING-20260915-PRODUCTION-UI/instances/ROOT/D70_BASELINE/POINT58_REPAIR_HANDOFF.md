# Point58 product picking repair — bounded follow-on

Status: proposed product repair, not implemented or accepted. This is a derivative diagnosis/repair handoff; it does not change decomposition, numerical policy, product source, original benchmark outcomes or the design program's scope/approval.

The owner's question about root cause led to a deterministic comparison after the five allotted N10000 attempts ended. The accepted upstream product remains PR789 `8468a33c86adb622b25e98f98b0eaf28c7e9fa0e`. Evidence owner: `instances/EXPECTATIONS/POINT58_CAUSE/DIAGNOSIS.md` SHA256 `5ebda533507798034b1a7f75f3bd9d19a2c3cd5bb763c530319e6bc7892f4494`; structured return SHA256 `54a8ed283f3b2bfc06393c9f94a60b2c3e7f715e5185937e8d45dfdc7673b878`.

## Established mechanism

Product normalized-miss arithmetic for a sphere and adjoining pipe capsules produces a roughly1.739e-12 difference at recorded point58. The absolute miss tie cutoff is1e-12, so the node is excluded before the existing node-before-pipe priority can apply. Broad-phase includes the relevant node and pipes. The independent oracle keeps the geometrically equivalent hits tied. Actual pinned exported picker and a diagnostic source copy agree in all32 reconstructed cases; product/oracle differ in28, including all8 delivered-coordinate cases. Four after-camera/intended-coordinate cases agree. Pointer values exactly equal float32 rounding of intended coordinates, but rounding alone is not the cause.

Historical camera quaternion/matrix bits were not stored. The reconstruction uses the recorded authored camera fields and pinned Three.js; it establishes a sufficient reproducible numerical instability, not bit-for-bit recovery of every historical browser calculation. Four actual runs1/2/4/5 failed at this boundary. Attempt3 owner-reported manual camera interaction remains separate.

## Proposed bounded repair

Anchor in DEL-07-01 picking correctness. A small source scope should be the sphere/capsule normalized-miss helpers in `apps/desktop/src/features/viewport/viewportSelection.ts` and focused picking tests. Calculate equivalent closest-point distances consistently, especially a capsule endpoint coincident with a node. Prefer a stable geometric residual over cancellation-prone subtraction of large squared lengths. Preserve frustum/broad-phase, front grouping, the1e-12 miss policy, type priority, frozen oracle identities and actual human pointer semantics. A larger epsilon is not justified merely by this failed sample.

Freeze a regression from the retained model/camera/canvas and intended/delivered-coordinate matrix before implementing. Independently check the shared-endpoint analytic tie. Add nearby genuinely distinct-hit controls so the change cannot manufacture node priority when a pipe should win. Verify translation/origin behavior, intended and delivered coordinates, both recorded camera reconstructions, deterministic ordering and existing picking tests. Keep allocations and candidate-loop cost bounded.

The specific implementation and its numerical stability remain unproved until that focused repair and independent review occur. Product changes were expressly outside the approved D70 baseline tranche, so no patch has been applied here. Do not rerun or replace the exhausted cohort, modify its oracle, or relabel any old failure. Any later runtime demonstration must have a new product/method identity and prospective workload bindings under its owning brief.

## Coordination and closure

Route this finding through the owner to the redesign program; no direct edits or messages to its run record. The renderer architecture need not be replaced to repair this local picking calculation. Carry this as an open correctness item into redesigned-product acceptance. D70's performance handoff condition is satisfied by the final truthful baseline report even when N10000 is incomplete; this proposed repair does not itself reopen that condition or close performance/usability obligations.
