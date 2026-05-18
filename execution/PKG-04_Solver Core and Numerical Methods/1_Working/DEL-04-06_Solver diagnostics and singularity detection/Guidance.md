# Guidance: DEL-04-06 Solver diagnostics and singularity detection

## Purpose

This deliverable frames how solver diagnostics should be prepared for later implementation without implementing numerical methods or choosing thresholds. The diagnostic layer exists to make solver failure states explicit, reproducible, machine-readable, and reportable.

## Principles

- Treat diagnostics as solver evidence, not engineering approval.
- Prefer explicit `TBD` for thresholds, tolerances, sparse-solver settings, and fixture details until an implementation brief or human decision resolves them.
- Preserve diagnostic provenance so a downstream report can explain which object, solver stage, and result envelope produced a warning or blocking condition.
- Keep invalid restraints and missing solve-required values visible as findings; do not normalize them away during setup.

## Considerations

- Singular models and invalid restraints overlap semantically but may require different diagnostic codes because one describes numerical rank/solvability and the other describes model-definition validity.
- Ill-conditioning is reportable under SOW-053 and SOW-035, but this setup cannot set numeric thresholds without inventing project-specific solver policy.
- Nonlinear nonconvergence must preserve active-set context when nonlinear supports are involved, while this deliverable remains separate from the nonlinear support solver implementation.
- Result envelopes should align with AB-00-03 and AB-00-06 so GUI, CLI, reports, and tests receive consistent status semantics.
- Thresholds remain `TBD` because the accessible setup sources require reportable singularity, conditioning, and convergence diagnostics but do not define numerical policy. Future values need solver-library evidence, deterministic tests, and human-approved implementation context, and must not imply code compliance.

## Vocabulary Notes (P3)

| Term | Setup use |
|---|---|
| Singular | A mechanics solve state where the assembled/constrained system cannot produce a determinate solution under the chosen solver policy. |
| Ill-conditioned | A numerical-quality warning state whose exact metric and threshold remain TBD. |
| Nonconverged | An iterative solve state that did not satisfy accepted convergence criteria; criteria remain TBD until implementation. |
| Invalid restraint | A model-definition finding for restraint/support data that prevents a valid solve or makes the restraint set contradictory. |
| Blocking diagnostic | A diagnostic that prevents treating the mechanics solve as completed. |

## Trade-offs

| Decision area | Setup guidance | Open issue |
|---|---|---|
| Diagnostic granularity | Use stable classes and machine-readable codes, then refine specific code taxonomy during implementation. | Exact diagnostic-code registry TBD. |
| Conditioning warnings | Preserve reportability and provenance before threshold selection. | Thresholds and sparse solver settings TBD. |
| Fixture design | Use original/public/invented models and avoid protected examples. | Exact singular-model fixture set TBD. |
| Status wording | State mechanics-solver facts only. | None for setup; human review remains outside solver authority. |

## Examples

Examples are TBD. Future examples should be invented/public/permissive and should not reproduce protected standard examples or proprietary benchmark cases.

## Conflict Table (for human ruling)

No source conflicts were found during setup. Numerical thresholds, solver library settings, diagnostic-code taxonomy, and fixture inventory remain `TBD`, not conflicts.
