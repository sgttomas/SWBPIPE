# DEL-09-01 independent mixed-reference report

Status: COMPLETE BOUNDED REFERENCE PACKAGE / POST-FREEZE COMPARISON COMPLETE.

Source SHA: `779dedb8670625b36af07b89fc5557470e47c50e`.

Fixture: `fixtures/product_preview/invented_preview_model.json`, SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`.

## Result and case boundary

M9 produced a reproducible, independently authored signed reference for the selected L-100 retained-spring/distributed-load/one-way-support/current-normal-friction composition. `FREEZE_MANIFEST.json` binds the input, method, executable and expected results before any production result was read. `EXPECTED_RESULTS_FROZEN.json` contains all global nodal displacement vectors, support-action vectors, local element-end vectors, five local station-cut vectors per pipe, branch trials, complementarity checks, and force/moment balances.

The original actual product fixture remains unchanged. It contains L-100 and L-200, their pressure primitives, their combination, P-100 through P-130, C-150, SH-140, NL-140 and NL-130-FRIC. The selected pressure-free L-100 seed removes L-200, clears the combination, and removes L-100-P and L-100-P-EJ. No numeric input is changed. `VARIANT_LEDGER.json` records every transformation.

The reported physical-frame case removes C-150 while retaining P-130. It is the pressure-free connector-excluded physical control used to resolve the frame, distributed load, retained zero-reference spring, contact and static current-normal friction calculation. It is not the unaltered full fixture and does not establish expansion-joint adequacy.

The literal-adapter case retains C-150 exactly as authored. It reproduces the current adapter's interpretation: the relative local stiffness vector `[3.2e6, 0.9e6, 0.9e6, 0.62e6, 0.48e6, 0.48e6]` in `[UX,UY,UZ,RX,RY,RZ]` order is placed between the endpoints of unchanged P-130, in parallel with P-130. This is a D02 mapping diagnostic only.

## Units and independent method

The reference uses a fixture-local explicit-SI basis: m, N, N·m, rad, Pa, and K-equivalent temperature intervals. Product displacement comparison converts metres to emitted millimetres. This ordinary explicit-SI calculation is isolated from the held DEL-02-02 canonical conversion acceptance and from human disposition `PKG09-0901-PKG02-001`.

The standalone NumPy calculation assembles a small-displacement 3D Euler–Bernoulli frame from the authored geometry, annular section and material values. It integrates the uniform line load into the consistent element-load vector, applies thermal initial strain, adds SH-140's 42,000 N/m UZ stiffness at zero reference displacement, enumerates NL-140 active/open states, and solves stick and both slip signs using the same returned S-130/UY normal reaction. `METHOD_AND_SIGNS.md` gives the equations and sign definitions; `SOURCE_BASIS.md` records the independent public mechanics references. No DEL-04-01/04-02 generated output, production result, or bundled expected constant derives an oracle quantity.

The three elementary controls were run first:

| Control | Input | Independent result |
|---|---|---|
| Consistent UDL cantilever | E=200 GPa, I=1e-5 m⁴, L=2 m, qy=-100 N/m | tip Uy=-1.0e-4 m; root Ry=+200 N; root Mz=+200 N·m |
| Retained ground spring | k=42,000 N/m, F=+350 N | u=+0.008333333333333333 m; spring action=-350 N |
| Frictionless unilateral scalar | k=100 N/m, F=+10 N, negative-reaction active side | u=0; R=-10 N; g=0; λ=10 N; λg=0 |

## Frictionless L-100 physical-frame control

With NL-130-FRIC removed, the unique admissible NL-140 branch is active. Selected signed values are:

| Quantity | Independent value |
|---|---:|
| N-130 UZ | -0.006652737126526824 m |
| N-140 UZ | -0.006651869142198679 m |
| S-130 UY action | -52.4314345134016 N |
| NL-140 UY action | -297.5685654865984 N |
| SH-140 UZ action | +279.37850397234456 N |
| max whole-frame force residual | 2.602291715447791e-10 N |
| max whole-frame moment residual about origin | 1.9776962290052325e-9 N·m |

All 157 compared displacement, support, pipe-end and station rows equal the independent values after rounding each independent quantity to the product's six-decimal emission precision. Dense and sparse product values are identical at that precision. The largest raw difference before expected-value rounding is 4.865984237767407e-7 N, 4.794610504177399e-7 N·m, 4.3807960414810054e-7 mm, or 4.5995357026573517e-7 rad, depending on dimension. These are observations, not pass/fail thresholds.

## Current-normal static-friction L-100 physical frame

The admissible zero-reference branch has NL-140 active and NL-130-FRIC sliding at negative UZ. Positive friction action opposes that motion. Signed global nodal vectors are ordered `[UX,UY,UZ,RX,RY,RZ]`:

| Node | Independent signed vector [m,m,m,rad,rad,rad] |
|---|---|
| N-100 | `[0, 0, 0, 0, 0, 0]` |
| N-110 | `[0, 0, -0.000803163771290182, 0.000297443369097172, 0.000588517038039602, 0]` |
| N-120 | `[0, 0, 0, 0.000323181203091575, 0.00141462361043179, 0]` |
| N-130 | `[0.00066, 0, -0.00664392679739248, 0.0000367576848420256, 0.00141097859709564, 0]` |
| N-140 | `[0.00376415291361042, 0, -0.00664305996255025, -0.0000183788424210128, 0.00141097859709564, 0]` |

Support actions act on the structure. S-100 is `[FX,FY,FZ,MX,MY,MZ]`; S-120 is `[FX,FZ]`; the rest are their named scalar DOFs:

| Support | Independent signed action |
|---|---|
| S-100 | `[0, 0, -116.560749324345, -164.524578006302, -236.263696674223, 0]` N/N·m |
| S-120 | `[0, +673.028498077101]` N |
| S-130 UY | `-52.3732819873299 N` |
| NL-140 UY | `-297.626718012670 N` |
| NL-130-FRIC UZ | `+0.523732819873299 N` |
| SH-140 UZ | `+279.008518427110 N` |

The current normal is `N=abs(S-130 UY)=52.37328198732987 N`; `mu*N=+0.5237328198732987 N`, exactly the signed friction action in the independent branch. NL-140 has `g=-UY=0`, `lambda=-R=297.62671801267015 N`, and `lambda*g=0`. Whole-frame residual maxima are `2.602291715447791e-10 N` and `1.976331986952573e-9 N·m`.

Local element-end vectors are ordered `[Fx,Fy,Fz,Mx,My,Mz]i,[Fx,Fy,Fz,Mx,My,Mz]j`:

| Pipe | Independent local end vector [N/N·m] |
|---|---|
| P-100 | `[0,-116.560749324345,0,-164.524578006302,0,236.263696674223, 0,116.560749324345,0,164.524578006302,0,-609.258094512129]` |
| P-110 | `[0,-116.560749324345,0,-609.258094512129,0,-164.524578006302, 0,116.560749324345,0,609.258094512129,0,-115.221220372126]` |
| P-120 | `[0,556.467748752757,0,115.221220372126,0,609.258094512128, 0,279.532251247243,0,-115.221220372126,0,0]` |
| P-130 | `[-279.008518427067,-52.3732819873299,0,0,0,-115.221220372126, 279.008518427067,52.3732819873299,0,0,0,0]` |

The five signed station vectors per pipe are in `EXPECTED_RESULTS_FROZEN.json` and `COMPARISON.csv`, using the stated action-on-positive-x-cut-face convention.

Production reports `+0.524314 N` friction with a returned current normal of `52.373217 N`; `mu*N` from that same production state is `0.52373217 N`. The signed production relation residual is therefore `+0.0005818300000000054 N`, and production minus the independent expected action is `+0.0005811801267012751 N`. Dense and sparse outputs agree at emitted precision. The physical mixed comparison contains 161 rows: 89 equal after independent-value rounding to six decimals and 72 differ. The largest absolute differences are `0.0005811801267012751 N`, `0.0007515121289998206 N·m`, `9.962550245212753e-6 mm`, and `4.829619603979483e-7 rad`. `PRODUCTION_RELATION_CHECKS.json` retains the direct same-return-state contract check.

## Literal C-150 adapter diagnostic

The zero-reference current-normal literal mapping gives N-130 UZ `-0.003855526276589945 m`, S-130 UY `-40.90937314143258 N`, friction `+0.40909373141432565 N`, NL-140 UY `-309.0906268585674 N`, and SH-140 UZ `+161.91118424439495 N`. Its net force residual is only `1.4568968254025094e-10 N`, but its global moment residual is `+658.4367607905363 N·m`. The independently calculated C-150 connector-end actions alone form an equal/opposite force pair with a net moment `+658.4367607916422 N·m`. That uncancelled couple is significant D02 evidence and prevents treating this mapping as a physically adequate whole-frame oracle.

Production reports literal-case friction `+0.409403 N` with returned current normal `40.90935 N`, for `mu*N=0.40909350000000005 N` and a signed relation residual of `+0.0003094999999999626 N`. The literal mixed comparison has 161 rows: 79 equal after expected-value rounding and 82 differ. It remains a mapping/defect diagnostic, not full actual-fixture adequacy.

## Independent review and reproducibility

One bounded Agent 2 independently assembled the pressure-free frame from the source fixture without reading production source, production output, manager output, or expected constants. It used `gpt-5.6-sol` with reasoning `high`, performed no delegation, and returned full L-100/L-200 vectors under `instances/M9/children/C1/`. For the L-100 physical-frame mixed branch, the maximum manager/child difference across 198 corresponding nodal, end and station components is `3.7624658943968825e-10` in the component's N/N·m/m/rad dimension. The current-normal/friction values agree within `7.8e-14 N`.

Reproduction uses `{PYTHON_WITH_NUMPY}`, resolved from the bundled workspace runtime; the host resolution used for this run appears only in `PRODUCTION_COMPARISON_EXECUTION.md`:

```text
{PYTHON_WITH_NUMPY} independent_mixed_reference.py --repo-root <repository-root> --output <temporary.json>
{PYTHON_WITH_NUMPY} verify_frozen_reference.py --repo-root <repository-root> --expected EXPECTED_RESULTS_FROZEN.json
CARGO_TARGET_DIR=<temporary-directory> cargo run --manifest-path production_comparison_harness/Cargo.toml -- <fixture> production_outputs
{PYTHON_WITH_NUMPY} compare_frozen_to_production.py --expected EXPECTED_RESULTS_FROZEN.json --production-dir production_outputs --json-output COMPARISON.json --csv-output COMPARISON.csv
```

## Applicability and retained decisions

This reference applies to a linear small-displacement Euler–Bernoulli frame, the selected pressure-free L-100 primitive loads, zero force-free displacement for SH-140, the explicit block-positive-UY interpretation for NL-140, and a static zero-reference current-normal Coulomb branch. It does not establish pressure behavior, C-150 physical topology, spring installed/cold/hot preload, incremental or cyclic friction history, the opposite one-way gap side, an engineering tolerance, lifecycle closure, or professional acceptance.

The retained owner decisions are C-150 topology/objectivity (D02), SH-140 preload/reference selection, friction history/committed slip and rollback semantics, explicit one-way allowed-displacement side, comparison thresholds, and canonical unit/conversion acceptance. The independent static reference remains usable while those forks stay open.

`DAG-002-E0532`, `DAG-002-E0533`, and `TP-DAG-004-DEL-09-01-E001` remain unchanged. `CONSUMER_INTEGRATION_EVIDENCE.md` records the exact executable fixture-to-adapter route that may support a later owner-run revalidation; R23 producer maturity alone did not establish consumer integration, and this run does not mutate or satisfy any dependency row.
