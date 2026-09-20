# B3 dist wheel-target assertion — owner decision package

Prepared2026-09-20 after clean sweep065485671c7bf1c9b100643316bb263dd39421d8.
No protected test or product bytes have been changed for this proposal.

## Evidence

All Rust, Python1133 and desktop unit1436 checks passed. Full pinned-Chromium
source suite passed416 with20existing skips. Dist passed52 of53; its decorative
viewport gesture case failed only on exact wheel-target equality at line1696:
expected z=-0.027936806875621; observed z=-0.027936806875620994, a difference of
6.938893903907228e-18. The other two target coordinates were bit-identical.
Retained raw log, failed summary, trace and context: _run_records/B3_DIST_ROUNDOFF/.
The failed full sweep remains failed; production-build surface was not run after
that stop, although the dist lane itself built the application successfully.

The source twin already uses a named binary64-roundoff bound:
16 * Number.EPSILON * max(1, abs(target coordinates), abs(localRenderOrigin coordinates)).
It requires finite coordinates and compares each difference to that bound.
Its comment explicitly explains OrbitControls normalization/rescaling and
local/authored translation. The installed OrbitControls implementation calls
Vector3.clampLength on its target during active updates, including wheel updates.
This establishes a known numerical concern addressed in the accepted source
oracle; the exact causal sequence of this dist observation is not fully isolated.
No meaningful pan, picking or engineering-model change is established by this
roundoff-scale observation. The source/dist mismatch was also retained in the
run's historical handoff as a maintenance risk.

## Recommended disposition

1. Apply B3_DIST_WHEEL_PARITY.patch.txt: copy the existing source twin's exact
   finite/scale-aware roundoff predicate into the one dist wheel-target assertion.
   No new tolerance is invented. Keep exact idle/Box/selection/picking assertions,
   frozen Box16 endpoints, canvas geometry, camera movement checks and every other
   limit/oracle unchanged. This is an explicit named assertion amendment, not a
   product workaround or a passing retry of the old equality check.
2. For this test-only correction, retain the clean065 passed Rust/Python/unit/full
   source results; run the corrected full53-case dist suite and the remaining
   production-build surface, plus independent patch review and actual selected
   hosted CI. Bind unchanged product/source-test hashes across the final candidate.
   Label the result combined delta verification under this ruling, not a fresh
   complete sweep. Preserve the original failed sweep and every raw output.

Reason owner decision is needed: loop/LOOP_INIT.md says “Never weaken a test or
move a tolerance, oracle or limit to obtain a pass” and requires a clean sweep
before every product merge. The earlier D1–D3 ruling did not cover this wheel
assertion. ROOT can diagnose and prepare this exact package, but cannot silently
relax the assertion or replace the complete-sweep gate with partial reruns.
Approval of item1 alone still leaves the unchanged full-sweep requirement. Approval
of both permits the narrowly described parity correction and reuse of untouched
passing surfaces for PR825 only. General local DEC025 remains unchanged.

Current source and dist tests are the evidence; no vendor-source patch, viewport
engine change, new skip, timeout increase, benchmark/profile rebind, release or
acceptance is proposed. No Runtime integration or DAG reconciliation.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
