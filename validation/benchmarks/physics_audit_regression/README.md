# Physics audit product regressions

Runs actual `run_linear_static_preview_with_mode` in both sparse and dense modes. Independently derived expectations were frozen before product comparisons in `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P9/author/EXPECTED_BEFORE_RUN.json`; its manifest SHA is `51465611bebef370b1542308bcb93e89be09b0ea3b3fa3dbc963015415d92d1a`.

Original invented circular-annulus cantilever: D=0.168 m, t=0.007 m, L=2 m, E=200 GPa, G=77 GPa. Elementary section integration, force/moment equilibrium and Euler–Bernoulli curvature integration are the oracles. Accepted baseline references are V1/children/V1-R/EARLY_REFERENCE_CRITERIA.md and I1/children/{C,R}/EXPECTED_BEFORE_RUN.json. No protected standards data or repaired output was used to derive expectations.

The scalar allowance is the existing six-decimal publication quantization half-unit plus 1e-10 floating arithmetic allowance. It is fixture comparison precision, not a new engineering acceptance or production convergence policy.

Coverage: R06/R07 selected contact and spring state, dimension-correct reaction; R08 constant axial/torsion/shear and actual recovered moment; R09 cancellation, retained precision and case-qualified source edges; R11 full mechanical UDL and supported partial wind assembly plus cut recovery. Every test calls product computation. Tests select actual metadata locations without requiring the old interpolation basis.

The negative controls must reintroduce force-plus-moment mixing and zero actual recovered station outputs. Historical force-only W3 is corrective and should pass. No production mutation is performed by this crate.

Run `cargo test --manifest-path validation/benchmarks/physics_audit_regression/Cargo.toml --offline` from the project root with a temporary CARGO_TARGET_DIR. Execution is subject to parent build scheduling. Verification is software evidence only; pressure, friction history, connector constitutive semantics and engineering reliance remain outside this suite.
