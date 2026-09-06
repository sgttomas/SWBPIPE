# P9-R independent reference review

Verdict: **PASS** for the frozen analytical expectation packet. No actionable reference error. This verdict does not review authored tests, execute product code, accept repaired behavior, or adopt engineering tolerances.

Reviewed author EXPECTED_BEFORE_RUN.json SHA `69b58a92b893589bb693e3b5a6acf43477bc53f9be03e908b3d2d9995005b5a3`, manifest SHA `51465611bebef370b1542308bcb93e89be09b0ea3b3fa3dbc963015415d92d1a`; both hashes independently verified. Source basis is `2be412ccea62bdc4bd96deb082c46d7a792076ea`. Parent launch brief SHA is `5b8e035f5d3a5c3dfbcb04e4c42b8dcce57a6dc2760a524727e43c271a7d79f4`.

## Arithmetic and dimensions

Independent 60-digit Decimal arithmetic using annulus integrals and original beam statics/curvature derivations corroborates all 18 numeric expected entries. Differences from the frozen binary-floating arithmetic are at most 7.05e-14 in the spring reaction N and are immaterial to six-decimal published scalar comparison. These are representation differences, not corrected oracles. A has m², I has m⁴, EI has N m², tip stiffness and spring stiffness have N/m. Multiplication by 1000 maps displacement m to mm; spring reaction converts mm back to m. Force norms contain N components only, moment norms N m components only. The independently checked spring values are 0.18796797141911283045 mm and 187.96797141911283045 N.

For a cantilever point force at x=L, integrating EI v''=M gives v(L)=FL³/(3EI), and equilibrium gives root force magnitude F and moment magnitude FL. A pure moment has zero root force resultant. Interior axial force, torsion and transverse shear magnitudes are constant for their separate end-loaded cases; bending moment magnitude is F(L-x). End nodal actions and section cut signs must not be conflated.

For distributed load q on [a,b], a point force at coordinate s contributes tip compliance s²(3L-s)/(6EI). Integrating this yields q*[L*s³-s⁴/4]/(6EI) evaluated at a,b. Integrating the outboard load and its lever arm separately yields the stated shear/moment expressions. Independent full q=100 N/m and partial [0.5,1.5] m checks corroborate both frozen displacements and section values in INDEPENDENT_ARITHMETIC.json. The piecewise zero condition x>b in the moment expression is necessary and is present.

## Applicability and observation boundary

The formulas apply to a straight constant-section, small-displacement Euler–Bernoulli cantilever with specified E, no additional gravity/thermal/pressure load, and 0<=a<=b<=L. Separate load cases must isolate axial, torsion, transverse force and pure moment. The spring oracle assumes a tip translational spring in parallel with cantilever bending stiffness and an unrelated inactive 1 m gap. The zero-gap axial oracle assumes the stop blocks the applied force direction at the loaded tip, the selected displacement is zero and there are no imposed strain loads; contact then carries the applied load and pipe axial force is zero. These constraints follow the accepted V1-R/I1 baseline cases and must be preserved by later test review.

Opposite-load SUM zero follows linear signed component superposition on identical boundary conditions, not envelope/range algebra or arbitrary nonlinear superposition. Tiny load is 0.00035 N and its factor 1e6 must be applied before published rounding to reproduce the 350 N displacement. The metadata source-reference invariant concerns primitive case qualification with explicit combination operand links exempted. Signed cut conventions, exact entity/location/basis selectors, missing-row failures and quantity-specific published units remain checks for the separately activated authored-test review; no broad behavior acceptance is implied here.

## Provenance and tolerance

Read accepted V1/children/V1-R/EARLY_REFERENCE_CRITERIA.md and I1/children/{C,R}/EXPECTED_BEFORE_RUN.json. Original derivations corroborate those reference packets without production imports or observations. No OCR equation, protected standard, commercial benchmark or new external source was used. No claim of independently examining the external links in V1 is made. Project docs/CONTRACT.md OPS-K-UNIT-1 and DEL-09-01 original/public provenance and fixture-versus-release boundary are respected.

The 0.5e-6 + 1e-10 absolute comparison is in each published scalar's unit and derives from six-decimal rounding plus arithmetic allowance. It is suitable for these moderate-scale fixture outputs, including zero identities; it is not a relative accuracy guarantee, solver residual tolerance, engineering limit or project-wide threshold. It must not be applied to unrounded section-property inputs or to arbitrary magnitudes as a universal acceptance criterion. No repaired output was used in this review.

## Handoff

Reference review complete; fresh authored-test review and actual implementation comparisons remain separately required. No production changes, test writes, compilation, product execution, network, Git operation or delegation occurred. Runtime role enforcement is instruction+config asserted; model identity is unknown when unexposed.
