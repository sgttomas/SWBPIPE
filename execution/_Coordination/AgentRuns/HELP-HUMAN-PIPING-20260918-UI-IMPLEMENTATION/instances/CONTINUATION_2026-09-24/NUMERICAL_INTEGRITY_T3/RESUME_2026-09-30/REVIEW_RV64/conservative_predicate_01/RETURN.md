# RV64 — independent conservative-predicate review

**CLEAR within the assigned one-row mathematical/source scope.** I independently reproduce I49's sufficient geometric obstruction and its point-truth comparisons. The optional represented/native-only SharperBinary64 statement is also valid after checking the actual directed helpers; no actionable blocking finding remains. A separately scoped tighter private certificate is a warranted next numerical dependency for this particular refusal with the truth, output and predicates unchanged. This is no approval of a particular correction or implementation.

The reviewed subject is `I49/conservative_predicate_01` at `d368447b7b0f88438911bbd6bcd372f99b88c9ea`, RETURN SHA-256 `7c5da65a02762ae34e3e2847e61571e5ae4f7fa7516165460d118f7da7bf353c`, SEAL `bfac20c908d6c7c96574f0bfaf3768fa99a36fb125ad6e1353a58d8b212d40b7`, inventory `067326ba5ba19e01626fb33fb76a49ded06735ae5bfacf9ba606fde25a800870`. All 25 inventory payloads were checked against their sizes and hashes. All 12 I49 source-copy identities, plus four additional source files needed for this review, match source revision `d0daa18717f8243a7232e898c9ef9b4f4d18d9e4` and the actual NUM and CODE production files. This verifies the source basis, not public availability or release.

## Independent basis and actual identity

`_run_records/independent_check.py` is newly authored here. It uses Python integer/Fraction arithmetic, its own exact round-to-nearest-even binary64 function, and a separate real-π enclosure. It does not execute I49 code, the captured oracle, a compiler, a model, a solver or a native runtime. I49's results were compared only after the independent arithmetic completed; `_run_records/CLAIM_COMPARISON.json` records that comparison. The original sealed packets remain unchanged.

The actual input is CODE's I47 `pp_debug.log`, SHA-256 `b443aabe00d24804e98dfe3f662db32d2a3277de976790998653354674f002c5`; the oracle file identity `43e37d0c2085487c2f86087612400398706dbe4a21d6d8a12ec74420bb8c117f` is checked without using its mathematical results. I extracted the actual `interpolated-loaded` section directly from that log. `_run_records/RAW_CASE_LINES.json` retains the exact line locations and `_run_records/RAW_CASE.json` the selected records.

I decoded the complete captured `K4SRC\x01` source bytes independently using `SOURCE_source.rs:775–887`, consuming the entire byte string. The result has one member from (0,0,0) to (1,0,0), y-reference (0,1,0), six zero anchor constraints, no springs or directional springs, and exactly three unit loads on tip UX, UY and RX. The encoded E/G/A/I/J values match the captured I47 input. The successful material selection identifies material ordinal 0, lower and upper point ordinals 1 and 2, and actual temperatures 293, 303 and 313 K. The captured source-point Pa/K bits agree with those request entries. No intended decimal replacement was used for D, t or moduli.

Final row 10 is `result:disp:node-N-DEC092-TIP:ux`, unit mm, case `case:i45-loaded`, body 0. `SOURCE_retained_product.rs:1011–1039` maps its kind to the native Ux displacement recipe and source body; `SOURCE_source.rs:27–70` fixes the component/global-DOF order. The row therefore uses native row 6, `Displacement(Dof { node: 1, component: Ux })`. Captured raw bits are `3ecd962166f5dd6e`; the separately rounded division by 1000 gives `3e2e4be8dc1e94ec`, exactly the native SI point. The actual final verdict is RelativeVerified with `[false,false,true,true]`.

The body extent is exactly 1. Recomputing native and final primary translation/rotation maxima and coupling under `SOURCE_adaptive.rs:2954–3010` and `SOURCE_final_case.rs:704–754,938–966` gives scale bits `3ed8d89883b941b9`, supplied by tip RX. Native and algebraically projected primary maxima agree in all four quantity kinds; records are in `PROJECTION.json` and `RESULTS.json`. That projection rounds each translation ×1000 and then ÷1000 separately. Row 10 has the same raw bits, normalized bits and scale as the captured final row. This is algebraic projection evidence, not execution of a complete producer.

## Point truths and unchanged tests

`SOURCE_product_certificate.rs:365–383,416–453` forms the four products with signs +,−,+,− and divides by the exact temperature difference. Independent exact arithmetic gives

- `E_s = 12451840000000001/65536` Pa, with `E_s − E_hat = 1/65536` Pa;
- `G_s = 13107200000000001/262144` Pa, with `G_s − G_hat = 1/262144` Pa.

The exact four-product operands are retained in RESULTS. For this source's unit axial load and uncoupled unit-length frame, `q_K = 1/(E_hat A_hat)` and `q_source = 1/(E_s π t(D−t))`, in metres. Source builders and the B/D assembly warrant those expressions (`SOURCE_product_certificate.rs:520–544`; `SOURCE_assemble.rs:280–349`). Both coordinates remain uncoupled from the Y and torsion loads in this frame.

For an independent π enclosure I use `π = 4(atan(1/2)+atan(1/3))`, not I49's Machin computation. The tangent addition identity gives π/4; each arctangent uses 180 rational alternating-series terms with the next term bounding the remainder. Inverting the positive source-area bracket gives a rational source displacement bracket. The exact predicate comparisons yield:

| Quantity | Independent SI value |
|---|---:|
| K point error | 6.1080658808120047e−26 m |
| Source point-error upper bound | 6.8081392744259433e−25 m |
| SharperExact allowance | 7.1270154216320684e−25 m |
| SharperBinary64 allowance | 7.1270154216320689e−25 m |

Both truths pass all four point predicates, including DecimalSi and the separately assessed raw-mm DecimalRaw test. The checker reconstructs SharperExact as `max(|n|,s)(2^-64+2^-85)+|n|2^-53+2^-1074`, and SharperBinary64 by the five distinct RN64 operations in `SOURCE_final_case.rs:655–677`. It also checks the actual relative classification threshold. Thus the captured sharper refusals are conservative for this bounded row under both truths.

## Why the geometric interval must refuse

1. **The two material values really enter the same source enclosure.** `material()` explicitly hulls each interpolated property with the represented actual value (`SOURCE_product_certificate.rs:438–452`). Positive geometry produces `J = π t(D−t)((D/2)^2+(D/2−t)^2)/2`, and the torsion coefficient is the interval product G·J. Fix any one positive J* in that interval, for example its exact source geometry value. Both `a = G_hat J*` and `b = G_s J*` belong to the actual torsion coefficient interval. This is a statement about the implemented enclosure; it does not declare two different physical source truths.

2. **The same arbitrary correction center is used against both values.** In the exact unit frame, the torsion B row is tip RX minus anchor RX. The anchor is constrained to zero, so the tip torsional residual includes `1−GJ*c` for the one actual free center c. `SOURCE_source_residual.rs:275–424,428–448,530–579` supplies the frame, B/D action, constraints, actual load terms and residual scaling. The other unit loads do not couple into RX. The interval arithmetic contains the pair of residuals for any real c; c need not be positive or close to the solution.

3. **The minimax inequality does not depend on the private center.** For positive a,b,
   `b(1−ac)−a(1−bc)=b−a`.
   The triangle inequality implies `max(|1−ac|,|1−bc|) ≥ (b−a)/(b+a)` for every c. Here the ratio is exactly `1/26214400000000001`. The unit torsional load is 1 N·m; in the kernel's canonical SI numerical coordinates this is the stated numeric residual lower bound. No private center, radius or endpoint is supplied or reconstructed.

4. **UX and RX share the data block and have the claimed scales.** `SOURCE_assemble.rs:546–568` inserts the full member structural 12×12 pattern, including entries whose numerical value is zero. `SOURCE_bound.rs:84–121` uses this pattern to form free components, placing all six free tip coordinates in one block. The actual nonzero load terms make that block a data block (`:164–197`). The selected precision is 128; the bridge explicitly uses twice that value, 256 (`SOURCE_adaptive.rs:5357–5364,5492–5535`). The axial and torsional diagonals are exact E_hat·A_hat and G_hat·J_hat at this precision, requiring only 81 and 78 significant bits. Their leading-bit exponents are 28 and 17. `SOURCE_wide_multi.rs:19–24,881–885` establishes the exponent convention, and `SOURCE_factor.rs:525–535` gives `s=−floor(e/2)`: UX −14, RX −8. This does not confuse the selected and verification precisions.

5. **The captured B is the radius recipe's actual B.** `SOURCE_retained_product_tests.rs:903–907` prints `ev.certified_bound`; its sole captured body-0 value is bits `40139d85e14169c8`, exactly `690152861609273/140737488355328`. `SOURCE_adaptive.rs:5322–5328` returns that same field for the block's body. The radius recipe sets β=2B and takes ω as the maximum absolute residual endpoint on the block. Alpha is built from nonnegative terms and must be less than one (`SOURCE_source_residual.rs:681–714`). Its directed denominator is positive and no greater than one. Therefore `ε = up(up(βω)/down(1−α)) ≥ 2B·2^-8/(26214400000000001)` (`:774–811`).

6. **The UX radius lower bound reaches the final predicate unchanged.** UX gathering expands its center by `2^-14 ε` (`SOURCE_source_residual.rs:428–446`). In the defined canonical-coordinate convention this is a UX half-width in metres. Hence

   `L = 2B·2^(-14−8)/(26214400000000001)`

   `  = 690152861609273/7737125245533627013267431579352825856 m`

   `  = 8.920016669080989931239326570…e−23 m`.

   The Native recipe uses this enclosure (`SOURCE_final_case.rs:477–479`), then hulls it with the represented interval (`:1061–1079`). For any tested point n and interval [l,u], `max(n−l,u−n) ≥ (u−l)/2`. Directed endpoint-distance evaluation can only enlarge that lower bound (`:549–554`). L is greater than both unchanged sharper allowances; it is about 125.15781349380507 times SharperBinary64. Both refusals therefore follow without reconstructing private endpoints. A different correction center or greater solver precision alone cannot remove this particular lower bound while these actual operands and enclosure/radius recipe remain in place.

The inverse bound and common radius use the implementation's mixed canonical-coordinate norm; this proof does not assign a single free-standing physical unit to B or epsilon. The UX gather and final normalization determine the metre interpretation used by the predicate.

## Separate represented/native branch check

The captured native publication radius is unknown, not zero. Source publication and bridge checks nevertheless require `0 ≤ r ≤ A64`, where A64 is the native SharperBinary64 allowance (`SOURCE_adaptive.rs:3546–3570,3796–3860`). The native point and recomputed native scale equal the final normalized point and scale here, so this is the same A64 as the final test.

Exact representability alone would not justify a conclusion for an unconditional outward-step helper. I inspected the actual helper: `SOURCE_product_certificate.rs:255–291` calls the directed functions; `SOURCE_directed.rs:66–120` computes the sign of exact−nearest and steps only on the wrong side. Exact sums have zero side and are returned unchanged. The directed operators are therefore tight and monotone on this bounded use.

The two ceiling endpoints `n−A64` and `n+A64` each need only 104 significant bits. They are exactly representable at 1024 bits. For any unknown r in [0,A64], tight directed rounding of n−r cannot go below n−A64, and rounding of n+r cannot go above n+A64. This remains true even if an actual n±r were not exactly representable. Each directed distance from n is then at most representable A64. Thus the represented/native branch alone cannot force this row's SharperBinary64 refusal. This argument needs neither an actual radius nor exact representability of every possible n±r. It makes no claim about a sub-ulp SharperExact boundary effect.

## Return scope and preservation

The present shared-radius geometric enclosure itself forces refusal of a point that passes both truths. The direct native projection leaves the obstructed row and scale unchanged. Those facts warrant tighter private final-row certification as a next numerical dependency if this specific refusal is to disappear with unchanged truths, output and predicates. They do not select a correction, remove a material hull, promise that any replacement certificate will pass, or grant implementation authority. No additional controlled capture is required for the sufficient inequality proved here.

I46's distinct actual source-point miss remains outside scope. No cause is inferred for the other 46 conservative rows. No claim is made about uncaptured endpoints, native radii, complete producer execution, public/main availability, protected routing, C2/reader/receipt, resource qualification, engineering acceptance or release.

Actual role: fresh TASK Type2 `/root/rv64_conservative_predicate`, directly under ROOT HELP_HUMAN `/root`, delegated-harness-native; no descendants. Actual receipt clock: **2026-10-03 05:27:52 UTC**. Checkpoint deadline 05:34:52, new-analysis cutoff 05:41:52, return deadline 05:47:52. An early substantive result was sent to ROOT before checkpoint. Active NUM Root/TASK/Piping instructions and RV64 brief were checked against `03025d023bf1bc42c73ad9559b492134d100b338`; origins/hashes are recorded. No additional role, workflow or skill body was selected. No Git/index/API writes, host tooling work, maintained-source edits or delegated execution occurred. Every shell call had an explicit intended cwd; every Git read set `GIT_OPTIONAL_LOCKS=0`. The only writes were this new owned review packet. Generated source copies, scripts, raw checks and execution records are under `_run_records`.

See `SEAL.json` for completion time and packet hashes; `_run_records/FINAL_PRESERVATION.json` rechecks the original subject and raw inputs after the review. ROOT owns the next design and acceptance step.
