# Two original static reference bindings

These are new named input variants reusing the independently checked mathematics in `CORRECTNESS_DESIGN/NUMERICAL_REFERENCE.md` (annulus, axial/torsion, N01 and R01) and `STRESS_REFERENCE.md` §§1–3 (signs, equilibrium, circular normal maximum). No new literature search or external benchmark is needed. Expected values are calculated independently of the product. The existing result fixtures are used only to bind field names, IDs, metadata and units.

## Fixed physical problem

Both cases have one straight circular member from root(0,0,0) to tip(2,0,0) m, local y along global+Y, OD0.20m and wall0.01m. Root has one six-DOF rigid anchor. Explicit invented material is homogeneous isotropic E=200e9Pa, nu=0.25; G is derived as80e9Pa. No G override, library, code rule, SIF, gravity, pressure, thermal field, spring, contact or other component is present. Each case is a separate model/request with exact-pressure profile2.0, model0.3 and explicit empty pressure regions. This chooses the current coherent E/nu/normal-stress contract without applying pressure.

Theory is linear elastic, small displacement/rotation, Euler–Bernoulli centerline bending and circular Saint-Venant torsion. The2m/0.20m geometry is a mathematical verification case; it does not assert negligible real-pipe shear deformation. The annulus has A=0.0019pi, I=0.0000085975pi, J=2I and Z=I/0.1 in SI units. Constant E/nu data are fixture inputs, not operational materials. The script evaluates the ideal declared decimal geometry; input binary64 rounding is separately checked to be far below the proposed field criterion for these ordinary values.

| Variant | Tip loading | Root support-on-pipe action (Fx,Fy,Fz;Mx,My,Mz) |
|---|---|---|
| original_static_axial_v1 | Fx=+1000N | (-1000,0,0;0,0,0) |
| original_static_bending_torsion_v1 | Fy=+30N, Fz=-40N, Mx=+20N*m | (0,-30,+40;-20,-80,-60) |

No reaction is computed from a production stiffness matrix in the oracle. Force/moment balance about the root gives these values directly. For local station x and span L, the positive-x section cut carries `[Fx,Fy,Fz,T,-Fz(L-x),Fy(L-x)]`. The result row at end_i instead reports node-on-element action, the negative of that cut vector; end_j reports the positive cut vector there. Interior quarter/midpoint rows report the cut action. Stress recovery at both ends uses cut actions, not the unadjusted endpoint node action.

Displacements follow integrated EB compatibility: u_x=Fx*x/(EA), u_y=Fy*x²(3L−x)/(6EI), u_z=Fz*x²(3L−x)/(6EI), theta_x=T*x/(GJ), theta_y=−Fz*x(2L−x)/(2EI), theta_z=Fy*x(2L−x)/(2EI). Nodal translations publish in mm, rotations in rad. Stress rows use N/A, My/Z, Mz/Z and T*r_o/J, converted once from Pa to the actual MPa row unit. These scalar bending components are signed amplitudes, not simultaneous surface principal stresses. The combined case has transverse shear forces, but the current four stress components do not publish their cross-section shear-stress distribution. That missing stress component is not zero; this bounded verification makes no complete 3D/equivalent-stress claim.

At every section, sigma_plus=N/A+hypot(My,Mz)/Z and sigma_minus=N/A−hypot(My,Mz)/Z. Maximum absolute normal is abs(N/A)+hypot(My,Mz)/Z; torque does not enter it. In the combined case, root moments80/60 give peak100/Z, not140/Z, with torsional shear20r_o/J separately. The signed normal extrema occur at opposite fibers (y,z)=(-0.06,+0.08) and(+0.06,-0.08)m. Along the span the normal maximum decreases linearly to zero, so the root station is unique while the two absolute-stress fibers tie. In the axial case every station/fiber ties; a producer witness may be anywhere in[0,1] and must not be mistaken for a unique maximum. Translation magnitude increases to the tip in both cases.

## Finite expectation denominator

Each variant has73 scalar output expectations:

- 12 signed nodal DOFs and2 translation magnitudes;
- 6 support components and separate force/moment magnitudes;
- 6 section actions at each of five stations (both ends, quarter1, midpoint, quarter3);
- 4 stress components at each station;
- 1 circular maximum absolute normal stress in Pa.

Thus there are146 scalar expectations per requested solver mode,292 if both modes are executed. The expected-zero entries are explicit numerical zeros, never omissions. `*.expectations.json` also retains continuous fields, signed circumferential extrema at all stations, governing location/fibers and high-precision decimal expectations. Current wire has no separate sigma_plus/sigma_minus rows; these reference fields are not assigned invented selectors or claimed as directly observed. Their relationship to the compared components/maximum is declared, and any later derived observer needs explicit implementation/review. [OUTPUT_OBLIGATIONS.json](OUTPUT_OBLIGATIONS.json) preserves both that missing direct output and transverse-shear stress under COR-PRESSURE/COR-RESULTS (M14/M31; DEL-05-03/09-02). Future required observations remain in their gate denominator even while unsupported.

`STRUCTURAL_EXPECTATIONS.json` adds finite input/producer/evidence/summary obligations. Those must remain required alongside the scalar denominator; a row-only pass cannot claim the completed case binding. The thin main gate currently lacks raw0.2 physics-1 dispatch, actual mode flags, true absent metadata and these structured checks. This package is deliberately not a ready runnable manifest.

## Criteria selected for review

Keep historical stress benchmark absolute1e-9 internal checks and source-method relative1e-9 accuracy contracts unchanged at their actual owners. They are different predicates. The following are **new project-specific field-comparison criteria for these two cases**, within delegated engineering design, and await independent checking before admission. They are not generic release thresholds or inherited absolute tolerances in arbitrary units.

1. Nonzero scalar fields: abs(observed−reference) <=1e-9*max(abs(observed),abs(reference)), with zero absolute floor, in exactly the same declared unit. This matches the existing comparison engine's explicit relative predicate; it is not labelled a reference-only denominator.
2. Prescribed root translation/rotation and its translation magnitude: exact numeric zero (either signed-zero encoding), after finite/type checks. This tests boundary reconstruction without a noise floor.
3. Other mathematically zero scalar fields: absolute budget1e-9 times the named **case and dimension** scale, with no relative term. Force scale is1000N axial or50N combined; moment scale is force_scale*L. Axial translation scale is FL/(EA), rotation scale its ratio to L; combined scales are resultant-force EB tip translation and rotation. Stress scale is the case's analytical normal peak; MPa uses the same physical scale converted once. No observed quantity sets its own zero tolerance.

The exact decimal scales and actual numerical budgets are retained in `*.expectations.json` and `*.criteria.candidate.json`. For example, combined zero action budgets are5e-8N and1e-7N*m; zero stress budget is about3.702354e-10MPa. These allow cancellation roundoff without permitting an unobserved zero to disappear. A2×budget injected defect must fail; a missing row always fails regardless of expected zero. Section/stress signs, units, case IDs and metadata are categorical matches, not numerically tolerant.

The ordinary extrema enclosure bounds the supplied binary64 coefficient problem, not all solver/coefficient formation error. Require its own correct scope, coverage and internal ordering; do not falsely require the ideal-decimal oracle to lie inside that very narrow coefficient-only interval. Compare its reported value against the independently computed field criterion separately. Require summary values/units/references to equal their selected raw rows exactly. Governing station is exactly root for the combined reference; axial accepts any in-domain witness and retains the analytical tie.

## Verification and review state

`independent_reference.py` uses only Decimal/Machin pi and explicit formulas. Its80/100-digit comparison, annulus coefficients, equilibrium and complete expectation count passed. This is arithmetic preparation, not an independent second implementation or a solver run. `build_bindings.py` imports no product code, copies only observed wire metadata, checks pinned semantic signatures and writes new invented requests. Separate independent reference/input/selector/criterion review is still required. No output, invocation hash or candidate pass for these new cases has been fabricated.
