# P9-R2 authored-test review

Verdict: **CHANGES_REQUIRED** — one actionable test-contract gap. All four frozen files were read in full and hashes matched AUTHORED_SOURCE_FINAL.json. See REVIEWED_FILES.json. This is a nonauthor code review; the earlier reference review remains frozen and separate.

## P9-R2-001 — Assert published units before comparing scalars

Location: `validation/benchmarks/physics_audit_regression/src/lib.rs:98–113` (`scalar` and `station`). Severity: P2.

Both helpers discard the ResultItem unit and compare the naked value against a dimensional oracle. Consequently, a regression publishing the right number with the wrong unit (for example a 350 reaction labelled N*m, a displacement labelled m rather than mm, or stress labelled Pa rather than MPa) passes every scalar assertion. This is especially material for the force/moment dimensional defect this suite is intended to protect and OPS-K-UNIT-1. The current product ResultItem explicitly carries `unit`; observed publishers use mm, N, N*m, MPa and state_code for this suite's selected quantities.

Remediation: require an independently specified expected unit in the selector or an exhaustive fixture-local kind-to-unit mapping, and assert the selected row unit before returning its value. Include nonlinear displacement/reaction and state-code rows. Retain the six-decimal allowance in the asserted published unit. Do not derive expected units from the product implementation. A label-only wrong-unit negative test can establish selector rejection without a product run.

## Checks without other actionable findings

The original cantilever, annulus, spring and distributed-load expressions agree with the reviewed frozen expectations. Supplement equations follow force balance: open-gap state/reaction 0, closed positive stop reaction -350 N, end-j beam force 350-ks*u, and stress formulas N/A and Tr/J. State codes match the existing public enumeration and do not adopt a convergence criterion. The partial-wind q=pressure*shape*OD setup produces the intended q=100 N/m over x=[0.5,1.5] m. Its three queried stations lie within that interval, so the simpler b-x formulas are applicable.

All ten tests call the actual run_linear_static_preview_with_mode API using invented inputs, exercise both modes after successful preceding assertions, demand MECHANICS_SOLVED and finite published values, and require unique selected rows. Station selectors demand actual location metadata; no literal-only production observation is claimed. Point-loaded constant axial/torsion/shear and actual bending checks would reject station zeroing. The pure-moment force assertion rejects reintroduced force/moment norm mixing; historical force-only W3 is correctly described as corrective. End-j signed force in the spring equilibrium assertion matches the separately documented nodal end-action convention; magnitude station tests do not purport to prove all section signs. Derived source edges are nonvacuous through checked>0, must resolve, and compare the full target/source basis; this fixture has no combination exemptions to exercise.

No repaired output generates the expected values. Tests use independent statics/curvature formulas with numerical constants frozen before the parent checkpoint; 0.5e-6+1e-10 is a published scalar fixture allowance, not engineering acceptance. README disclaims pressure, friction, connector and engineering reliance. No public production API, schema, migration, dependency version policy or lifecycle state is changed by these four files. Cargo.toml has only the product path dependency and existing serde_json usage; Cargo.lock was fully inspected. No generated product artifact is introduced.

## Evidence and limits

Scope validation on all four explicit paths: PASS. Correct project-relative affected-check selection returns evidence-sweep, piping-pytest and harness-self-check. An initial repository-relative query selected only the always check; it was corrected before forming this assessment. Parent reported checkpoint compilation and ten expected pre-repair failures; this review does not independently reproduce those results or infer repaired product PASS. Full current source hash binding, post-repair runs, mutant observations and whole registered checks remain manager obligations. Fresh review after the unit assertion repair is required; no other actionable code issue identified in this bounded review.
