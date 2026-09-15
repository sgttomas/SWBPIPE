# Actual private pressure-kernel refutation V2

Verdict: FAIL — two actionable arithmetic defects remain in source `8533622eaa7d87c102791d60bc9eb3607d6daf119a2da41cbfe866e89e4d283f`. V2 repairs the five frozen V1 counterexamples. No successor source has been observed.

## Required baseline and repair checks

All 114 original valid cases pass (1,990 scalar comparisons). All 44 original error fixtures pass (47 constructor/function operations). All five frozen V1 diagnostic cases pass under their original tolerances. The unchanged original Decimal/Fraction oracle supplies every numerical expectation.

The root opposed-strain case was independently derived and frozen before invoking V2: ri=1, ro=2, E=2^-1000, nu=1/4, p=0, epsilon=+2^1023, thermal=-2^1023. Therefore epsilon-thermal=2^1024 in exact arithmetic and sigma=2^24, Nw=S=3π·2^24. The applied RHS identity gives eigen_i=-EAs·thermal=+3π·2^23 and eigen_j=-3π·2^23. This case passes. Twelve additional opposed-strain variants (both signs, E exponents -1002/-1000/-998 and strain exponents 1022/1023) also pass. The initial root negative eigen-i transcription is not an accepted alternative sign.

Additional expectations were frozen before any actual V2 invocation in EXPECTATION_FREEZE.json. There are 151 additional valid cases and 2,370 scalar comparisons: the opposed-strain cases, both signs of broad radius-ratio cases, admitted tiny geometries with different pressure scales, and nearby signed/exponent variants of all five V1 diagnostics. Twenty-six generated candidates with an independently nonrepresentable final output or ideal positive area/G rounded to zero were excluded before observing implementation outputs; their identifiers and reason are retained in EXCLUDED_GENERATOR_CASES.json.

Of the 151 additional cases, 84 fail: 78 cases involving normal final forces from subnormal geometry and six broad radius-ratio cases. They produce 492 scalar discrepancies. All other additional cases, including nearby versions of the prior multiplication-association and force-cancellation cases, pass. No comparison policy, geometry cutoff, numerical envelope or previously frozen expectation was changed.

## F4 — Subnormal stored areas corrupt normal final forces

Locations: pressure_exact.rs lines 221–225, 269–273 and 295–303 (especially fluid_force at 299). V2 repairs stress reconstruction but still scales those stresses using rounded stored As and constructs cap force from rounded stored Ai. Those area outputs have already lost significant digits at subnormal scale. A large pressure turns the error into an unacceptable error in a normal, representable force.

Small reproducible case: ri=2^-537, ro=2^-536, E=120, nu=1/4, p=2^200, epsilon=thermal=0. Exact Ai=π·2^-1074 and As=3π·2^-1074. Their stored binary64 values are 3·2^-1074 and 9·2^-1074. The exact cap load P=π·2^-874 is normal; the stored-area computation produces 3·2^-874. The exact pressure-only wall and eigen-i force is P/2, and S=-P/2.

| Quantity | Expected N | Actual N |
|---|---:|---:|
| Cap j | 2.4942137116196084e-263 | 2.381798647991063e-263 |
| Cap i | -2.4942137116196084e-263 | -2.381798647991063e-263 |
| Wall / eigen i | 1.2471068558098042e-263 | 1.1908993239955315e-263 |
| Effective / eigen j | -1.2471068558098042e-263 | -1.1908993239955315e-263 |

The ratio is 3/π (about 0.955). These final forces are normal, many orders above the subnormal comparison floor; increasing pressure preserves the relative discrepancy. Reversing pressure reverses the discrepancy. Pressure-stress outputs in this example are now correct, isolating the remaining dimensional force reconstruction failure.

Repair direction: compute force products from validated primary radius factors with a stable scale/order, retaining information until the final force rounding. Treat the rounded area accessor as a separately rounded output; do not use it as an exact intermediate when that corrupts another representable output. Apply this to cap/fluid force, wall-force scaling, eigen-force scaling and relevant direct-force fallback paths. Retain positive subnormal geometry admission, true final-overflow rejection and the frozen tolerances.

## F5 — Squaring a radius ratio before pressure scaling erases interior stress

Location: pressure_exact.rs lines 170–178, specifically `inner_radius_ratio * inner_radius_ratio` at 175. The helper receives an already underflowed zero, so its revised association choice cannot recover the lost information.

Reproducible case: ri=2^-300, ro=2^300, r=2^299, E=120, nu=1/4, p=2^900, epsilon=thermal=0. Every requested final value is finite. The radius ratio ri/r=2^-599 has a squared value 2^-1198 that rounds to zero, although multiplication by p yields a normal stress-scale value. Let a=p·ri²/(ro²-ri²)=2^-300/(1-2^-1200). At r=ro/2 the exact radial stress is -3a and hoop stress is 5a. The unchanged high-precision oracle calculates these from independent traction equations; ignoring the denominator's 2^-1200 correction changes nothing at binary64 scale.

| Midwall quantity | Expected Pa | Actual Pa |
|---|---:|---:|
| Radial | -1.472728039589318e-90 | -0 |
| Hoop | 2.4545467326488633e-90 | 9.818186930595453e-91 |

The constant transverse trace is correctly retained as 2a, so the lost radial term also makes the hoop stress wrong. The same defect appears at a quarter-wall-radius witness and with radius exponents ±350/±400, and reverses with pressure sign. Exact endpoint traction tests continue to pass; endpoint-only checks cannot detect this interior loss.

Repair direction: retain the two radius-ratio factors until pressure is incorporated in the stable product. Do not pre-square a potentially tiny ratio. Preserve the independent radial/hoop interior checks, constant-trace identity, endpoint pressure-scale checks and all existing admission/error behavior.

## Actual V2 mutation detection

All six required semantic mutation classes were applied to new evidence-local copies of V2 and compiled, then checked against the original 114-case set. V1 mutant binaries/counts were not reused. The changed stress-space Poisson expression and direct-force fallback were both changed for each Poisson mutation. The cap-subtraction mutation changes recovered wall force; the pressure-area mutation changes the cap/fluid-force helper. These narrower locations still have independently checked observables.

| Actual V2 code mutation | Scalar mismatches |
|---|---:|
| Omit 2nuP | 525 |
| Double 2nuP | 525 |
| Reverse 2nuP | 525 |
| Subtract cap from wall force | 218 |
| Mean-radius fluid/cap area | 324 |
| Omit fluid term from effective force | 109 |

Source copies, adapters, exact build/run argv, raw observed values and each comparison failure are retained in the corresponding directories. Replaceable executables live only in the authorized temporary build directory.

## Handoff

V2's baseline and V1 repair checks pass, but the two additional findings block an overall PASS. Added expectations use the original independent derivation and unchanged tolerance: 128 machine epsilon times the declared physical-term scale plus eight minimum-subnormal ULPs. Source and all earlier freeze hashes were reverified after execution. A successor source must be separately frozen and dispatched before further refutation. Root retains integration acceptance and the full independent diff review; dormant scalar checks do not qualify live pressure solver behavior.
