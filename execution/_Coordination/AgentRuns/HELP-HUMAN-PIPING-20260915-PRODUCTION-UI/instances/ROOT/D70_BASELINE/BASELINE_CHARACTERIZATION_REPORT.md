# Rendering baseline and redesign handoff

The bounded attempt is complete, with an incomplete 10,000-pipe characterization. One 1,000-pipe run completed validly and met every original timing target. None of the five allotted 10,000-pipe attempts completed validly. No performance acceptance is claimed.

This is the final owner-facing report, derived from the [runner's detailed evidence report](instances/RUNNER/FINAL/BASELINE_REPORT_V2.md) and [recorded-input cause calculation](instances/EXPECTATIONS/POINT58_CAUSE/DIAGNOSIS.md). Repository review, registered checks and Git integration are recorded separately at closeout.

## Product, method and environment

Product: PR #789 at `8468a33c86adb622b25e98f98b0eaf28c7e9fa0e`; production artifact manifest SHA256 `800ec17958e47de299e39e3da1942dbbef0093eeed3c5c4311b5e9bcfa66567f`. Host: Apple M5 Max, 128 GiB; pinned Chromium 153.0.8010.36, macOS arm64. Frozen fixtures, camera/oracles, action populations and timing targets remain attributed to each run.

The successful 1,000-pipe run used method `6ea1ed09…` and the external LG display in its reported 60 Hz mode. The 10,000-pipe attempts used instrument `df1562e0…`, method `1e0ab8a4…` and the owner-approved internal display in its reported 120 Hz mode. These results do not support a same-display scaling comparison. Full source, artifact, method, browser, fixture, oracle and profile identities are retained in the [canonical manifest](instances/RUNNER/FINAL/CANONICAL_EVIDENCE_MANIFEST.json) and [binding proof](instances/RUNNER/FINAL/FINAL_BINDINGS.json).

At the recorded continuation boundaries: browser viewport 1440×920 CSS px; canvas 794×557 CSS px; drawing buffer 1588×1114 px; browser and effective rendering DPR 2. Labels were OFF with 0 visible, or ON with 1–3 visible, under the unchanged budget of 80. The [boundary record](instances/RUNNER/FINAL/BOUNDARY_OBSERVATIONS.json) retains scenario-specific display mode, conversion, camera, theme, density, panels and resource observations. These snapshots are not continuous monitoring.

## Attempt accounting and original targets

| Size | Valid complete | Invalid incomplete | Interrupted | Waived |
|---|---:|---:|---:|---:|
| 1,000 pipes | Attempt 2 | Attempts 1, 3 | Attempt 4, owner-directed stop | Attempt 5 |
| 10,000 pipes | None | Attempts 1–5 | None | None |

All eligible attempts are exhausted. The earlier external-display preflight stopped before a claim and consumed no attempt. Attempt 10,000.3 was affected by owner-reported manual camera interaction. Attempts 10,000.1, .2, .4 and .5 stopped at point-selection 58. Original failed/partial evidence remains intact; no replacement sample, extra attempt or false successful-run closure was created. See [actual accounting](instances/RUNNER/FINAL/ACCOUNTING.json).

| Successful 1,000-pipe metric | Recorded value (ms) | Original target (ms) |
|---|---:|---:|
| Assignment | 261.696 | ≤2,000 |
| Point selection p95 | 46.699 | ≤100 |
| Box selection p95 | 47.073 | ≤200 |
| Tree filtering p95 | 43.695 | ≤200 |
| Centerline orbit interval p95 | 16.668 | ≤16.7 |
| Actual OD orbit interval p95 | 16.668 | ≤33.3 |

That run completed 200 point actions, 20 box actions, 20 filters and both prescribed orbit windows. Settled owned animation-frame count was 0. The failed 10,000-pipe runs recorded assignment values of 642.417–654.143 ms, but lacked complete interaction and orbit populations; their missing scores are unavailable. A null settled count after abort is not evidence of a measured nonzero count.

## Separate work, presentation and resource observations

For the successful 1,000-pipe run, each orbit window contains 1,800 attributable main-frame event occurrences: 600 with known durations and 1,200 with ambiguous/invalid phase attribution. Known-duration p95 was 2.168 ms for centerline and 2.225 ms for Actual OD; these statistics cover the 600 known durations, not all occurrences. Renderer identity was PID 78726 / TID 61136922.

GPU-related trace context was separately identified at PID 78720 / TID 61136826. SwapBuffers span p95 was 0.522/0.539 ms for the two modes. Those spans are not hardware GPU execution time. Chromium-reported presentation interval remains the separate 16.668 ms result above and is not physical scanout. The [orbit summary](instances/RUNNER/FINAL/ORBIT_SUMMARY.json) retains all event counts, ambiguous/missing evidence and raw references; no overhead subtraction or new CPU threshold was applied.

Cold startup, heap and native RSS measurements are unavailable. Assignment includes the recorded model-presentation workload; it is not a native startup measurement. Final cleanup and exact bindings passed. Per-run boundary resources and prior native/resource evidence retain their original applicability; this work establishes no new full resource or native qualification.

## Root cause and follow-on

The repeated point58 failure is a reproduced product picking numerical instability. Sphere and capsule distance calculations differ by about `1.739e-12` in normalized miss, exceeding the `1e-12` tie cutoff and excluding the node before node-before-pipe priority. Broad-phase includes the node. Actual product and independent oracle disagree in 28 of 32 reconstructed cases, including every delivered-coordinate case. Pointer float32 rounding is confirmed but is not the sole cause. Exact historical camera matrix bits were unavailable; the calculation reproduces a sufficient mechanism from recorded camera fields.

No product fix has been applied. The [bounded repair handoff](POINT58_REPAIR_HANDOFF.md) proposes stable, consistent closest-point distance computation and independently checked regression cases, preserving existing tolerances and priority. The frozen product, oracle and failed verdicts remain unchanged.

Under D-70, this truthful incomplete report supplies the performance handoff condition for redesign implementation. The design program retains its own readiness and approval responsibilities. Carry the picking defect, D-68 performance acceptance, settled/resource obligations and independent-usability holds forward. Overlay/deformation observations remain separate work after the design program's rendering brief and relevant implementation; they block neither this baseline handoff nor redesign progress. No project, release, lifecycle or professional acceptance follows.
