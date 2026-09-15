# PRESSURE_ORACLE V3 refutation return

PASS with a qualified overflow-admission boundary; no unresolved actionable finding under the recorded Agent 0 interpretation.

- Original 114 valid cases (1,990 scalar comparisons) and 44 invalid fixtures (47 operations): PASS.
- All five V1 diagnostics and all 151 V2 cases, including corrected opposed-strain signs: PASS.
- New exploratory cases: 419 complete passes and one explicitly qualified above-MAX constructor rejection; the original failed prediction and raw summary are preserved.
- Additional ideal-area <= MAX admission probes: 1,103 cases / 15,442 scalar comparisons PASS.
- Gradual-underflow/signed-zero checks: 36 exact cap-output bit checks PASS.
- Actual V3 mutation classes: all six detected.

The qualification is narrow: one ideal area equals MAX+0.0150087867ULP and would ideally round to MAX, but finite-precision constructor arithmetic rejects it. Agent 0 accepts this under the existing positive-finite area predicate; the contract does not promise correctly rounded exact-real admission throughout the overflow rounding interval. No clamping, tolerance change, source change or erased failure was used. Root must preserve this limitation in combined acceptance.

Read REPORT.md, ADJUDICATED_SUMMARY.json and BOUNDARY_COMPARISON_AND_DISPOSITION.json for the final disposition. SUMMARY.json intentionally retains its generic pre-ruling failure status. V3 source and all earlier artifact hashes were reverified unchanged. Root retains complete diff review, integration acceptance and later runtime qualification.
