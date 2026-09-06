# KR independent kernel review

Verdict: CHANGES_REQUIRED. One actionable finding; no source edits. This is derivative review evidence, not lifecycle or engineering acceptance.

## KR-01 — Global eigenvalue stopping threshold can publish a finite condition for a singular coupled block

Priority: P2. Location: `core/solver/performance_harness/src/lib.rs:1052–1058` (condition computation at 970–989).

For the exactly singular symmetric matrix `[[1,0,0],[0,1e-20,1e-20],[0,1e-20,1e-20]]`, normalization leaves the matrix unchanged. Its maximum off-diagonal is `1e-20`, below machine epsilon, so the first convergence check returns diagonal entries `[1,1e-20,1e-20]` without a rotation. The condition helper therefore returns `1e20`. The exact null vector `[0,1,-1]` proves singularity directly; no numerical oracle or engineering threshold is needed. The same stopping rule underestimates a positive small coupled block: replacing its off-diagonal by `0.999e-20` gives smallest eigenvalue approximately `1e-23`, hence condition approximately `1e23`, while the helper still reports `1e20`.

This violates R03's honest singular/unavailable condition observation requirement. Overall matrix-scale normalization removes unit scaling sensitivity but does not establish relative accuracy for small eigenvalues. Returning a finite number after an absolute off-diagonal stopping check does not by itself establish a resolved minimum eigenvalue. Remedy: use a stopping/rotation or uncertainty check that resolves these blocks or explicitly reports the condition unavailable when the smallest spectrum cannot be distinguished from the remaining eigensolver error. Retain valid diagonal-small-spectrum behavior and the ordinary chain/tridiagonal references. Add the coupled singular and small-positive-block independent regression cases. This changes diagnostic accuracy, not the production pivot or engineering acceptance policy.

Evidence method: direct exhaustive control-flow evaluation of the first loop with elementary exact null-vector algebra; no compiled runtime reproduction claimed. The matrix is finite, symmetric, and passes every input precondition. The issue concerns the helper contract, not a claim that an existing named DEC053 fixture has this matrix.

## Review coverage and positive observations

All 528 added and 75 removed lines across seven frozen solver files were read, including every test change and surrounding relevant control flow. The eighth bound file, diagnostics, is unchanged. COVERAGE.json records every hunk and the combined diff hash; SOURCE_BINDING.json binds all eight source hashes and independently verified K1/N1 manifests (12 and 17 members respectively).

- frame_kernel: local, transformed and assembled stiffness finite checks; prescribed RHS adjustment; dense elimination/backsolve. Existing error channels and ordinary solve behavior preserved.
- sparse_direct: factor coefficient/pivot and all substitution stage checks; finite optional pivot ratio; no pivot threshold alteration.
- linear_supports: exact one-DOF scalar spring boundary; finite nonnegative stiffness including valid zero. Existing finding variant reused with precise text; no DTO change.
- straight_pipe: every direct point-load literal validated before the location-based accumulation skip. Existing constructor checks reused.
- nonlinear_supports: every supplied trial numeric value checked, and engaged gap zero reaction included consistently with existing released gap clearance equality. Strict pulling still releases. Friction and one-way policies unchanged.
- nonlinear_integration: existing convergence constructor enforced at execution boundary; duplicate support IDs rejected before iteration. Independent equilibrium tests cover signs, seeds, solve modes and coupled exact contacts. No new numerical tolerance introduced.
- performance_harness: finite residual/delta publication checks and optional finite ratio guards correctly prevent NaN from being folded to zero or serialized as a purported valid numeric observation. The eigensolver convergence issue above remains.

## Verification and limits

K1 reports 155 targeted crate tests and N1 41 passing tests, with pre-repair failing regressions. Their final evidence hashes were independently verified; tests were inspected but not rerun or claimed independently executed here. No build, network, Git mutation, user-app interaction, source editing, or child delegation occurred. Scope validation PASS covers only the declared kernel paths; unrelated concurrent changes are excluded. Full registered evidence sweep, native checks and DEC053 regenerated observations remain parent/integration gates. Review of this frozen kernel diff does not certify later product adapters, recovery, held friction/pressure/connector policy, or engineering adequacy.

## Handoff

Remediate KR-01 through an implementation owner, freeze a new source/evidence binding, and request a bounded independent backcheck. N1 and the other K1 changes have no actionable findings on this freeze; this is not acceptance of unreviewed later changes.
