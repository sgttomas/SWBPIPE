# Source-faithful annulus successor reference

Date: 2026-09-24. TASK /root/numerical_policy_review; actual parent /root; issuer/integrator /root/physics_manager. This is the first disposition for ROOT's revised source-faithful choice, before its production edit. Earlier REFERENCE_RETURN.md, REFERENCE_FREEZE.json and FROZEN_SECTION_EXPECTATIONS.json remain unchanged as the explicitly considered represented-radii comparison.

**PASS: source-faithful A/Ai/I/J/Z and runtime response expectations are frozen in SOURCE_ODWALL_EXPECTATIONS.json.** Successor tests target the exact normalized source OD/effective-wall geometry at the unchanged relative 1e-9 criterion. They do not lower the target to consistency with rounded ri.

## Chosen geometry and derivation

Let D and t be the exact represented normalized source outside diameter and effective wall. In these test cases there is no mill tolerance, so effective wall equals entered wall after normalization. The mathematical section is

    ro=D/2; ri=ro-t
    A=pi*t*(D-t); Ai=pi*(ro-t)^2
    I=pi*[ro^4-(ro-t)^4]/4 = A*[ro²+(ro-t)²]/4
    J=2I; Z=I/ro.

The equalities follow circular area/second-moment integration and were checked as exact Fraction identities. This definition retains the input wall when forming the tiny difference between radii would lose a significant fraction of it. Rounded ro/ri are derived output observations, not a replacement definition of A or I. All stiffness, pressure loads, wall recovery and stress division must use the same source-derived properties.

The wrapper can reuse the existing scaled arithmetic to avoid intermediate range errors. The original radii-only scalar kernel and its historical oracle remain unchanged. Named source-surface pressure values follow traction and compliance:

    radial_inner=-p; radial_outer=0
    hoop_outer=2*p*Ai/A; hoop_inner=hoop_outer+p
    Nw=E*A*(epsilon-alpha*DeltaT)+2*nu*p*Ai.

These identities avoid treating a rounded radial coordinate as the exact mathematical source surface. The whole source definition and the rounded-radius status belong in geometry_basis evidence.

## Independently frozen values

Source inputs D=.12 m, E=200 GPa, nu=.3, L=6 m:

| Property/response | t=.01 m, p=2 MPa | t=1e-12 m, p=1e-6 Pa |
|---|---:|---:|
| A, m² | .00345575191894877249 | 3.76991118427633574e-13 |
| Ai, m² | .00785398163397448233 | .0113097335525462637 |
| I, m⁴ | 5.27002167639687760e-6 | 6.78584013158430650e-16 |
| J, m⁴ | 1.05400433527937552e-5 | 1.35716802631686130e-15 |
| Z, m³ | 8.78336946066146300e-5 | 1.13097335526405113e-14 |
| Free transferred-pressure extension, m | 5.45454545454545444e-5 | 3.59999999990999998e-7 |
| Axial membrane stress, Pa | 4545454.54545454511 | 29999.9999992499981 |

The thin represented-radii alternative extension remains 3.60000469805440638e-7 m, +1.3050401129e-6 relative to the source value. Both are retained and named; only the source value is the successor acceptance target.

With the additional frozen transverse/torsional loads, use the same original cantilever/section formulas: uy=Fy*L³/(3EI), rz=Fy*L²/(2EI), rx=Mx*L/(GJ), G=E/[2(1+nu)], and maximum normal stress=P/A+Fy*L/Z. Inputs Fy=100 N/Mx=1 N*m for ordinary and Fy=1e-12 N/Mx=1e-10 N*m for thin give the complete reference values in JSON. No source solver/section helper or observed output generated these values.

## Range and scope

The independent ro≈1e77/ri≈5e76 control remains valid under the source definition: I≈7.36e307 and J≈1.47e308 are finite even though A*(ro²+ri²) before division exceeds binary64 range. The low-E, small-pressure fully fixed public fixture isolates section/range assembly without relying on an ill-conditioned free solve.

ROOT's selected initial contract still blocks a positive wall whose rounded ri equals ro, and cases with truly unrepresentable final properties. Preserving a source wall is not authority to fabricate a distinct reported radius or silently accept overflow/underflow. It is also not a physical minimum-wall rule.

The tiny/extreme geometries are arithmetic controls, not manufactured pipe or shell/collapse/engineering validity references. Source OD/wall arithmetic accuracy does not imply modeling accuracy outside the long, straight, small-strain, homogeneous-isotropic approximation. Earlier ordinary pressure14 expectations remain within their unchanged tolerance and their source file stays untouched.

## Next step

The new test file is only core/product_physics/tests/pressure_section_geometry.rs. Existing Ai/As/rounded-radius evidence can be checked alongside actual extension, bending, torque and stress. ROOT approved additional I/J/Z and all-member pipe_sections evidence; exact field-name assertions wait for the integrator's key plan. The full freeze hashes are in SOURCE_REFERENCE_FREEZE.json. Cargo still requires the shared slot. This reference return does not claim a source/runtime/native test pass.
