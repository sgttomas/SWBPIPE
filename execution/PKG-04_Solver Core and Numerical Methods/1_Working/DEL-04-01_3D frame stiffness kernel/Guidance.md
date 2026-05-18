# Guidance: DEL-04-01 3D frame stiffness kernel

## Purpose

This deliverable prepares the bounded implementation surface for the central 3D frame stiffness kernel. Its value is to keep global centerline mechanics, sparse numerical behavior, reproducibility, units, diagnostics, protected-data controls, and professional-boundary language aligned before code is written.

## Principles

- Keep the global model line-element based unless a later handoff explicitly routes local analysis to shell/solid FEA.
- Treat six degrees of freedom per node as a scope fact, not as permission to invent an ordering or sign convention.
- Keep coordinate conventions, matrix storage format, solver library, tolerances, and benchmark targets as TBD until a source-backed implementation brief or human ruling seals them.
- Separate mechanics computation from rule-pack evaluation and professional approval.
- Use deterministic, rights-cleared verification fixtures. Do not use copied standards examples or protected commercial data.
- Surface missing values, singular/ill-conditioned states, and assumptions as explicit result-envelope findings rather than silent behavior.

## Considerations

The accessible source set is the decomposition, registers, contract, and local context. It does not provide detailed mechanics formulas, reference-element derivations, numerical tolerances, sparse storage formats, or benchmark thresholds. Future implementation must therefore source those details from lawful references or human-approved architecture decisions.

Architecture basis rows constrain the future implementation shape:

- AB-00-01 requires decision records for accepted architecture choices and reconsideration triggers.
- AB-00-02 keeps dependencies pointed inward toward domain contracts and preserves layer responsibilities.
- AB-00-03 separates commands, queries, jobs, result envelopes, progress/cancellation, and mechanics/rule/human approval states.
- AB-00-06 requires structured diagnostics and avoids certification/compliance claims.
- AB-00-08 requires layered solver tests and protected-content/provenance gates.

## Trade-offs

| Topic | Current guidance |
|---|---|
| Sparse solver library | TBD. Choosing one now would exceed setup scope. |
| Coordinate convention | TBD. Must be consistent across element, support, load, result, and report interfaces. |
| Performance targets | TBD. SOW-035 requires sparse performance and reproducibility, but concrete thresholds are not available. |
| Verification fixtures | Use invented, rights-cleared, non-authoritative fixtures only after implementation is scoped; label them as test fixtures, not engineering examples. |
| Kernel boundaries | Keep element stiffness, support behavior, diagnostics, load application, and stress recovery in their assigned deliverables unless a human-approved change amends the decomposition. |

## Examples

- `TBD`: A future fixture set may include a minimal frame topology to test DOF mapping, but this setup kit does not define values, tolerances, or expected results.
- `TBD`: A future architecture decision may choose a sparse matrix representation, but this setup kit does not select one.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict found in setup sources. | N/A | N/A | N/A | N/A | N/A |
