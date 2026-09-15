# Actual private pressure-kernel refutation V1

Verdict: FAIL — three actionable arithmetic findings on frozen source `855144bf880370b5e40b57ecfc5e08fe3dec5929fc6d4a0d824ef0c9487bcd32`. No V2 implementation has been read. These are private dormant-kernel findings, not observations of active solver behavior.

All original frozen expectations pass: 114 valid cases, 1,990 scalar comparisons, 44 invalid fixtures exercising 47 constructor/function operations. The five additional diagnostic cases produce 22 missing/incorrect scalar outputs. Baseline and diagnostic Rust adapters invoke the actual frozen private API. All independent expectations still come from the unchanged pre-source Decimal/Fraction oracle and unchanged comparison scales. The added inputs and expectations are frozen in DIAGNOSTIC_EXPECTATIONS.json.

## F1 — Rounded force/area intermediates corrupt representable stresses

Locations: pressure_exact.rs lines 165–181, 207–222 and 283–287. `lame_at_radius` reconstructs the transverse stress trace from a rounded fluid force and rounded wall area, and forms an intermediate partial outside area. `axial_state` divides rounded wall force by rounded wall area. Once these small intermediates lose information, later division magnifies the error far beyond the stress comparison policy.

Minimal independent normal-area input: ri=2^-400, ro=2^-399, E=120, nu=1/4, p=2^-400, thermal=epsilon=0. Both areas are normal binary64 numbers, but P underflows. The prescribed geometry has transverse trace 2p/3 and axial membrane stress p/6.

| Quantity | Expected | Actual |
|---|---:|---:|
| Axial membrane Pa | 6.454319858082197e-122 | 0 |
| Inner hoop Pa | 6.454319858082197e-121 | 3.8725919148493183e-121 |
| Outer hoop Pa | 2.5817279432328787e-121 | 0 |

The separate writer-reported probe was independently rederived: ri=2^-537, ro=(9/8)ri, r=(17/16)ri, E=120, nu=1/4, p=3, thermal=epsilon=0. Here the normalized wall area ratio is 17/64; consequently A=192/17 and transverse trace=384/17 exactly. The midpoint radial stress is -6720/4913, hoop is 117696/4913, and axial stress is 96/17. No rounded geometry or force output was used to derive these values.

| Quantity | Expected Pa | Actual Pa |
|---|---:|---:|
| Axial membrane | 5.647058823529412 | 4 |
| Inner hoop | 25.58823529411765 | 21 |
| Outer hoop | 22.58823529411765 | 18 |
| Midwall radial | -1.367799715041726 | -0 |
| Midwall hoop | 23.956035009159372 | 18 |

Repair direction: evaluate dimensionless geometry ratios and each final stress without first discarding information in dimensional area/force intermediates. Preserve required finite force rounding as its own output; do not derive a representable stress solely from that rounded force. Keep pressure-scale endpoint traction and admitted positive geometry behavior. No geometry cutoff, tolerance widening or blanket underflow rejection repairs this defect.

## F2 — Selecting the largest product association amplifies subnormal rounding

Location: pressure_exact.rs lines 295–306, especially `max_by` at 305. The largest finite association is not necessarily the most accurate one.

Input: ri=2^-500, ro=2^-499, E=2^600, nu=1/4, p=0, thermal=0, epsilon=2^-78. Correct wall area is 3π·2^-1000, so Nw=3π·2^-478 and axial stress=2^522. The intermediate As·epsilon equals 3π·2^-1078, approximately 0.589 of the smallest subnormal; rounding upward gives one subnormal. Multiplication by E then gives 2^-474. The code selects this larger, incorrect result over a normal-association result close to the exact answer.

| Quantity | Expected | Actual |
|---|---:|---:|
| Wall/effective force N | 1.2076280723910805e-143 | 2.0501330894674953e-143 |
| Axial membrane Pa | 1.372959532026122e157 | 2.3308084926623222e157 |

The ratio is 16/(3π), approximately 1.698. Setting epsilon=0 and thermal=2^-78 reproduces the negative force/stress discrepancy and incorrect thermal eigenload pair. Strain magnitude is small in both cases; every requested final value is finite and well above subnormal scale.

Repair direction: use exponent-scaled multiplication or an ordering/error method that preserves significant information. Do not select by absolute magnitude among already-rounded associations. Cover both elastic force and thermal eigenload paths.

## F3 — Separate force overflow rejects finite cancellation results

Locations: pressure_exact.rs lines 208–220 and 249–266. Each physical force term is required to fit binary64 before the cancelling sum is formed.

Input: ri=1, ro=1024, E=p=2^1022, nu=-3/4, epsilon=0, thermal=-1.5/(2^20-1) rounded to binary64. This is a valid material with G=2^1023 and a small thermal strain (about -1.43e-6). P=π·2^1022 is finite. The thermal and Poisson force terms each have magnitude approximately 1.5π·2^1022, which exceeds f64::MAX, although their sum is finite. Using the exact rational thermal strain they cancel exactly; the actual binary64 thermal input leaves the finite residual shown below.

Actual `axial_state` and `eigenload_pair` both return `NonRepresentableLoad`. Independently derived requested results are finite: wall/eigen-i force approximately -1.836948414308395e290 N, effective force -1.4119048864730642e308 N, axial stress approximately -5.576318724783634e283 Pa, and opposite eigen-j force. The original term-scale allowance handles cancellation and permits rounded zero for the small residual; an error result still fails. No requirement to reproduce more cancellation digits than the frozen policy permits is introduced.

Repair direction: form cancellation in an appropriately scaled force/stress representation before overflowing individual terms, then validate the final outputs. Keep genuinely nonrepresentable final-output rejection cases.

## Required actual implementation mutations

Six evidence-local source copies were compiled and run against all 114 frozen valid cases. Product source was never mutated. Each source copy and adapter, raw output, comparison failure set, compile argv and source hash are retained in its named directory.

| Actual code mutation | Scalar mismatches detected |
|---|---:|
| Omit 2nuP | 525 |
| Double 2nuP | 525 |
| Reverse 2nuP | 525 |
| Subtract cap from wall force | 327 |
| Use mean-radius pressure area | 1,150 |
| Omit fluid term from effective force | 109 |

This is additional to the original analytical mutation witnesses; it demonstrates detection against compiled implementation copies. Original valid/error expectations, baseline source hashes, and the new diagnostic expectations are unchanged during this refutation. Root owns repair integration and a separate successor refutation; V1 remains immutable failure evidence.
