# Guidance: DEL-09-03 Nonlinear support regression suite

## Purpose

This deliverable prepares the document and semantic foundation for nonlinear support regression testing. The suite is intended to help future solver work demonstrate repeatable behavior for active-set, gap, friction, lift-off, convergence, and non-convergence diagnostics without importing protected examples or overstating the meaning of test success.

## Principles

- Keep the suite evidence-first: every future case needs a source/provenance record or an explicit original/invented basis.
- Treat missing convergence thresholds as `TBD`; do not invent final values during setup.
- Separate solver verification from user rule checks and professional approval.
- Exercise diagnostics as part of regression behavior, not only displacement or reaction magnitudes.
- Keep unit handling visible in fixtures, comparisons, and expected observations.
- Prefer small, original mechanics cases for public regression coverage unless a public/permissive source can be documented.

## Considerations

Nonlinear support behavior is sensitive to solver maturity. A regression suite created too early can freeze incidental behavior instead of intended behavior. This setup therefore records the categories and gates but defers final cases, tolerances, and pass/fail thresholds until the nonlinear active-set solver and diagnostics contract are mature enough to support them.

The suite should distinguish at least these future evidence categories:

| Category | Setup guidance |
|---|---|
| Active-set state | Verify active/inactive state reporting when the solver exposes it. |
| Gap closure/opening | Verify state transition and diagnostic reporting without inventing protected benchmark values. |
| Lift-off | Verify one-way support disengagement behavior using original or permissive cases. |
| Friction state | Verify stick/slip or equivalent reported state only after the solver defines the state vocabulary. |
| Convergence | Record iteration counts, tolerance basis, and non-convergence warnings through the result envelope. |
| Regression stability | Compare deterministic outputs for the same model, units, solver version, and input manifest. |

## Trade-offs

Early setup can identify coverage categories and source rules, but it cannot provide authoritative numerical thresholds. Later implementation should avoid broad, opaque acceptance bands unless they are justified by solver evidence, unit behavior, and repeatability analysis.

Regression tests should be specific enough to detect solver drift while avoiding dependence on protected standards data or commercial software results. Public/original examples are preferable where they can test mechanics and diagnostics directly.

## Examples

Specific regression case definitions are `TBD`. No example model, benchmark file, commercial comparison, or code-derived case is introduced by this setup pass.

## Open Issues

| Issue ID | Topic | Status |
|---|---|---|
| OI-09-03-001 | Final nonlinear convergence tolerances and pass/fail thresholds. | TBD pending nonlinear solver maturity and human review. |
| OI-09-03-002 | Exact diagnostic/result-envelope field names for active state, friction state, and non-convergence warning records. | TBD pending applicable schema/solver contract. |
| OI-09-03-003 | Public/original/permissive source list for future nonlinear support regression cases. | TBD; no protected examples may be copied or paraphrased into public artifacts. |

## Conflict Table (for human ruling)

No source conflicts were identified in the setup pass. Open issues above are gaps caused by deferred solver maturity, not contradictions between sources.
