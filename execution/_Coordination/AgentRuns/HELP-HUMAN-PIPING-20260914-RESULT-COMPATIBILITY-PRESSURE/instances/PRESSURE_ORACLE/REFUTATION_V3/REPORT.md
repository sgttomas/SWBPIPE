# Independent private pressure-kernel refutation V3

Verdict: PASS with the explicit overflow-admission boundary qualification below. There are no unresolved actionable findings under the frozen scalar policy and Agent 0's recorded interpretation. This is the private dormant-kernel refutation only; root owns the complete fresh diff review, integration acceptance and runtime gates.

## Required evidence and arithmetic method

The actual frozen V3 private API passes all 114 original valid cases (1,990 scalar comparisons) and all 44 invalid fixtures (47 constructor/function calls). It passes all five independently frozen V1 counterexamples and all 151 V2 diagnostic/additional cases, including both signs, nearby exponents and the corrected opposed-strain expectation. The original oracle script, expectations, tolerances and earlier failure evidence remain byte-identical.

The signed normalized mantissa/exponent method was reviewed for finite-input decomposition, normal/subnormal exponent ranges, multiplication and division normalization, exponent alignment, affine cancellation, final conversion and signed zero. Nonzero stored mantissas are normalized and exponents remain separate through the derived geometry and mechanics. The permitted inputs bound the exponent arithmetic well inside i32. Products/ratios do not discard their exponent range, and the final stress/force fields convert independently. Exponent alignment can discard a term far smaller than the larger mantissa; such a term cannot cause meaningful cancellation and remains within the unchanged physical-term comparison scale. This assessment is supported by the executed cases, not a proof for every possible binary64 input.

The V3 repair resolves all independently confirmed V1/V2 failures: subnormal stored areas no longer control normal stress/force outputs; tiny radius-ratio products survive pressure scaling; upward-rounded multiplication associations are eliminated; force/stress and opposed-strain cancellation occur before range loss; and final output checks preserve the original true-overflow error cases.

## New independent probes

Before invoking V3 on new inputs, ADDITIONAL_EXPECTATIONS.json and FINAL_CONVERSION_EXPECTATIONS.json were frozen with unchanged Decimal/Fraction calculations. The 420 admitted exploratory predictions cover:

- signed gradual-underflow cap loads and signed zero;
- geometry/cap quantities near the upper finite boundary;
- positive subnormal geometry with final normal forces;
- reproducible broad exponent/sign/material/strain combinations, including adjacent radii and interior stress witnesses.

419 of these cases pass every scalar comparison. Thirty-six explicit cap-output bit checks also pass. These bit checks address the directed V3 gradual-underflow/signed-zero requirement: well-separated cases that must round upward to the minimum subnormal or to the appropriately signed zero. They do not replace the general scalar tolerance with an exact-bit policy.

One exploratory geometry is conservatively rejected. Its raw failed expectation and generic pre-disposition summary remain preserved, as described next. No source, tolerance, cutoff or expectation was changed to obtain this final verdict.

## Qualified overflow-admission boundary

The retained case WALL_AREA_OVERFLOW_NEAR_0 uses ri=`0x1.4d8d7a58fa311p+510` and ro=`0x1.4d8d7a58fa311p+511`. Its independently calculated ideal As is f64::MAX + 0.015008786701717676 ULP. Ideal single nearest-binary64 rounding would therefore produce finite f64::MAX, because the value lies below MAX+0.5ULP. Actual finite-precision area arithmetic leads to `Err(NonRepresentableGeometry)`.

This is above the exact finite range; it is not a below-MAX pass or a below-MAX rejection. Agent 0's durable PRESSURE_OVERFLOW_BOUNDARY_INTERPRETATION_V1.json accepts this specific conservative rejection under the existing positive-finite binary64 derived-area predicate. The private contract does not promise correctly rounded exact-real admission throughout the overflow rounding interval. The ruling is an interpretation of the existing admission predicate, not a new tolerance, saturation rule or physical geometry cutoff.

BOUNDARY_COMPARISON_AND_DISPOSITION.json preserves the exact high-precision area, f64::MAX, ULP distance, rounding threshold, input hex values, observed error and ruling hash. ADDITIONAL/RESULT.json retains all 15 raw failure entries: one missing-case entry plus the fourteen expected scalar outputs unavailable after the constructor rejected. SUMMARY.json deliberately retains its earlier generic failure status. ADJUDICATED_SUMMARY.json records the final qualified verdict; no failed prediction has been relabeled as an observed pass.

A separate, independently frozen admission probe set then restricted ideal Ai and As to values <= f64::MAX. All 1,103 selected cases pass, comprising 15,442 scalar comparisons. These results support the distinction for this finite sample; a genuinely below-MAX rejection would remain a finding requiring investigation. Inputs/expectations were frozen before the strict-suite invocation; any repeated earlier input already had its earlier pre-invocation expectation. Excluded generator candidates and all raw results remain available.

## Actual V3 mutation detection

Six new evidence-local copies of the actual V3 source were compiled and checked against the unchanged 114-case set. No V1/V2 mutation counts or binaries were reused.

| Mutation | Scalar mismatches detected |
|---|---:|
| Omit 2nuP | 525 |
| Double 2nuP | 525 |
| Reverse 2nuP | 525 |
| Subtract cap from wall force | 218 |
| Mean-radius pressure area | 1,150 |
| Omit fluid term from effective force | 109 |

Both axial and applied-eigen Poisson paths were changed for each Poisson mutation. Every source copy, adapter, raw observed output, comparison failure, source hash and exact compile/run argv is retained. Actual product source was never mutated. Replaceable binaries remain under the explicitly authorized temporary build directory.

## Evidence, scope and handoff

V3 source SHA256 is baa83a5a62ea34d0677a5c5c5d6cd2fd2f977b6b62a5e2d4ddd06fa18951abc0; the private lib.rs declaration remains aa91613c48346654dc7e8b110f22fc74cf9578fb16f21ae9c40cec863b910f15. Both were verified before and after execution, and every earlier oracle/refutation artifact hash was reverified. The boundary ruling is SHA256 56070efd3aa53fef86e7966cbda7ac56e65d1e1dca2e06e7d18d1a0d2d4e9805.

This derivative evidence does not accept a new decomposition, lifecycle, DAG, public pressure profile, engineering tolerance or governing snapshot. It demonstrates the private kernel under the original policy and explicit recorded boundary interpretation. It does not qualify topology, supports, solver assembly/recovery, material selection, DTOs/migrations, public results, persistence/hashing, dense/sparse parity, native behavior or engineering reliance. Root must retain the qualified overflow-admission limitation in combined acceptance. No further child repair/refutation is required for this frozen V3 source unless source or accepted basis changes.
