# H numeric19 summary correction

ROOT additive correction to the sealed I21/h_numeric_19 candidate (seal
c31d1513c1fac6e73b3ea8672733de10f677b809cd852c58058f77931e017888),
in response to RV30 h_numeric_23/RETURN.md H23-F1 and H23-N1 (review seal
6ff5205bb500593bf02437fdc89c65df11acd8ea6d39aedf535598dee81e05b5).
Read the original numerical candidate together with this correction. Its
sealed bytes, exact byte tables and source-allocation formulas are unchanged.

[Correction H23-F1, ROOT, 2026-10-01] The W1 rho branch of the existing runner
evaluates the Python floating product E*rho, then footprint_estimate=int(E*rho).
It then evaluates footprint_estimate*rf and sets projected=int(footprint_estimate*rf),
comparing with int(PROJECTED_RSS_FRACTION*C). Thus its source-expression summary
is int(int(E*rho)*rf) <= int(0.8*C), with the actual Python floating evaluations
and positive-value truncations retained. This is not an exact-real identity
for rho or rf. The original SUMMARY condition omits the final int conversion
and must not be copied as an admission rule. The earlier footprint inequality,
ascent, calibration selection and binary backstop also remain required; this
correction performs no admission replay and changes no runner code.

[Correction H23-N1, ROOT, 2026-10-01] The displayed MiB values are truncated
to six decimal places, with 1 MiB=1,048,576 bytes. They are presentation values,
not outward-rounded upper bounds. Exact byte columns remain authoritative and
unchanged. No recomputation of allocation candidates or historical byte
comparisons is required by these two documentation corrections.

Source: core/solver/performance_harness/runner/k6_runner.py, actual W1 rho
branch and subsequent projected-RSS check (lines548–566 on frozen40129).
Same-reviewer backcheck is pending. No complete E_max, implementation, final
artifact, chronological admission or measurement acceptance is made.
