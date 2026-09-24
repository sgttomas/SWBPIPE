# Independent static annular shear refutation — M31

Date: 2026-09-24. TASK `/root/correctness_design/shear_refutation`, parent
HELPS_HUMANS `/root/correctness_design`. This is an independently derived
reference contribution, not a product run, merge review, or engineering
approval. Parent and TASK exchanged consequential findings during the work.
Actual supplied basis and arithmetic are recorded in
[`ORIGINS_AND_EXECUTION.md`](./_run_records/shear_refutation/ORIGINS_AND_EXECUTION.md).

## 1. Disposition and qualified source choice

The recalled Cowper annulus coefficient is correctly attributed and
mathematically reproducible. **It differs from the coefficient obtained by
matching Saint-Venant shear energy.** For the selected first-order static
variational model, use the energy coefficient:

```text
m = ri/ro; A = pi*(ro^2-ri^2); I = pi*(ro^4-ri^4)/4
G = E/[2*(1+nu)]
kappa_E = 6*(1+nu)^2*(1+m^2)^2 /
 [(7+14*nu+8*nu^2)*(1+m^2)^2+4*m^2*(5+10*nu+4*nu^2)]
Ks = kappa_E*G*A [N].
```

This matches the shear energy of the stated sectional stress field. It does
not make the resulting beam displacement the exact area-average displacement
of a three-dimensional pipe, reproduce all physical clamps/traction fields,
or qualify dynamics. Its v and theta are generalized beam translation and
section-director rotation, conjugate to nodal force and couple. Do not silently
identify them with Cowper's normalized three-dimensional displacement moments.

The selected constant-coefficient static Timoshenko element is exact **within
that one-dimensional model**. Its load projection and particular-solution
recovery must use the same model. A stiffness-only replacement is insufficient.

Sources inspected 2026-09-24; this is a bounded relevance check, not a survey:

| Primary source and inspected scope | Consequence |
|---|---|
| Cowper (1966), *The Shear Coefficient in Timoshenko's Beam Theory*, J. Applied Mechanics 33(2), 335–340, [DOI](https://doi.org/10.1115/1.3625046); pp. 335–338 of [original-paper reproduction](https://www.scribd.com/document/848371938/The-shear-coefficient-in-timoshenko-s-beam-theory) | Eq. 33 is the candidate annulus expression. It uses displacement averages and a normalized axial-displacement moment, with approximations to transverse-stress effects. The paper favors static and long-wavelength/low-frequency use. Publisher retrieval failed; this is an unauthenticated reproduction. Extracted mathematical text is not the numerical authority: section 2 derives the result independently. |
| Steinboeck, Kugi and Mang (2013), *Energy-consistent shear coefficients for beams with circular cross sections and radially inhomogeneous materials*, IJSS 50, 1859–1868, [author-hosted postprint](https://www.acin.tuwien.ac.at/file/publications/cds/pre_post_print/steinboeck13b.pdf), DOI 10.1016/j.ijsolstr.2013.01.030; §§2–3, 4.4–4.5 | Homogeneous-annulus Eq. 19 gives the selected energy stiffness. This is a directly applicable newer static alternative. The independent stress-square integration below reproduces it, rather than trusting extracted equation glyphs. |
| Kennedy and Martins (2012), *A homogenization-based theory for anisotropic beams with accurate through-section stress and strain prediction*, IJSS 49, 54–72, DOI 10.1016/j.ijsolstr.2011.09.012; [author-uploaded full text](https://www.researchgate.net/publication/222112597_A_homogenization-based_theory_for_anisotropic_beams_with_accurate_though-section_stress_and_strain_prediction), abstract, §3.1, §5.1.2 Eq. 45, conclusion | Corroborates Cowper's annulus expression, but its full theory includes load-dependent corrections and distinguishes displacement moments from average rotations. A scalar-factor Timoshenko element cannot claim to implement that entire theory. The author-hosted PDF timed out here; the inspected text identifies upload by Martins. |
| Mancusi (2023), *Shear Warping Behaviour of Elastic Beams With Annular Cross-Section*, [preprint v1](https://doi.org/10.21203/rs.3.rs-3016407/v1), [inspected abstract/introduction](https://www.researchgate.net/publication/371783983_Shear_Warping_Behaviour_of_Elastic_Beams_With_Annular_Cross-Section) | The newer annular model adds axial warping and spatially specified inner/outer loading. It is relevant if those fields are required. Only scope was inspected; no coefficient, numerical result or publication-status upgrade is adopted. It does not invalidate the deliberately reduced model. |

Frequency-matching factors require their own mass/dispersion/kinematics
qualification. Neither newness nor a dynamic accuracy claim establishes a
static energy law. No proprietary piping-code factor or commercial-solver
output was used. These source descriptions stay within their bounded inspected
scope; independent mathematical work below supplies the reference values.

## 2. Independent section derivation

Here a=ro, b=ri (the parent's CONTRACT uses the opposite radius letters).
Let s=a²+b², p=a²b², X=r*cos(alpha), Y=r*sin(alpha),
c=(3+2nu)/4 and e=(1+2nu)/4. Require 0<=b<a, E>0 and -1<nu<1/2.
The section is homogeneous, isotropic and prismatic; flexure is linear elastic
and Saint-Venant, excluding end boundary layers. The harmonic flexure function
is

```text
chi = -c*[s*X+p*X/r^2]+(X^3-3*X*Y^2)/4.
```

At either circular boundary it satisfies

```text
chi_n = -[nu*X^2/2+(1-nu/2)*Y^2]*nX -(2+nu)*X*Y*nY.
```

Its radial derivative at r=a or b is
`-c*r²*cos(alpha)+(3/4)*r²*cos(3alpha)`; reversal of the inner normal
reverses both sides. The singular-looking p term vanishes in the solid limit.
The resulting sectional shear field carrying force V is

```text
B0(r)=-c*s+r^2/2; B2(r)=c*p/r^2+e*r^2
tau_X=-V/[2*(1+nu)*I]*[B0+B2*cos(2alpha)]
tau_Y=-V/[2*(1+nu)*I]*B2*sin(2alpha).
```

Direct checks give integral(tau_X dA)=V, integral(tau_Y dA)=0, zero lateral
shear traction at both radii, and transverse divergence -V*X/I, consistent
with the axial-stress gradient. Orthogonal transverse forces have zero shear
energy cross-term by circular symmetry. Angular integration gives

```text
integral[(tau_X^2+tau_Y^2)/V^2 dA]
 = pi/[2*(1+nu)^2*I^2] * integral_b^a [B0^2+B2^2]*r dr
 = [(7+14nu+8nu^2)*s^2+4p*(5+10nu+4nu^2)]
    /[6*(1+nu)^2*A*s^2].
```

The unsimplified radial integral, to make the algebra recoverable, is

```text
c^2*s^2*(a^2-b^2)/2-c*s*(a^4-b^4)/4
+(1/4+e^2)*(a^6-b^6)/6
+c^2*p*(a^2-b^2)/2+c*p*e*(a^2-b^2).
```

Equating integral[(tau_X²+tau_Y²)/(2G) dA] to V²/(2Ks) gives kappa_E.
This fixes the matching observable; it does not assume all shear factors are
interchangeable. Separately, direct integration of the displacement-related
quantity Jchi=integral[X*(chi+X*Y²) dA] gives

```text
Jchi=-A*[(7+6nu)*s^2+(20+12nu)*p]/48
kappa_C=-2*(1+nu)*I^2/(A*Jchi)
 =6*(1+nu)*(1+m^2)^2/[(7+6nu)*(1+m^2)^2+(20+12nu)*m^2].
```

This is the Cowper mean-displacement relation obtained after eliminating the
residual warping contribution. It is a different normalization, not a typo.

| Limit | kappa_E | kappa_C |
|---|---|---|
| m->0 | 6(1+nu)²/(7+14nu+8nu²) | 6(1+nu)/(7+6nu) |
| m->1 | 1/2 | 2(1+nu)/(4+3nu) |
| nu=0 | Same as Cowper for every m | Same as energy for every m |

With t=p/s² in [0,1/4],
`1/kappa_E=[7+20t+(1-4t)*(nu/(1+nu))²]/6>0`.
Cauchy–Schwarz on the shear resultant also gives kappa_E<=1. As m->1 with
fixed outer radius, A and Ks vanish despite the finite factor. Stable annulus
differences, finite input validation and material-basis consistency remain
implementation obligations, not new physical cutoff policies.

## 3. Exact static element, loads and recovery

Use d=[vi,thetai,vj,thetaj], with positive theta in the positive slope sense.
The selected variational model is

```text
U=1/2 integral[EI*(theta')^2+Ks*(v'-theta)^2]dx
M=EI*theta'; V=Ks*(v'-theta)
V'=-q; M'=-V-c_line; theta'=M/EI; v'=theta+V/Ks
f=[-V(0),-M(0),V(L),M(L)].
```

For zero line loads, integrate V=V0 and M=M0-V0*x:

```text
theta=thetai+M0*x/EI-V0*x^2/(2EI)
v=vi+thetai*x+M0*x^2/(2EI)-V0*x^3/(6EI)+V0*x/Ks.
```

Eliminate V0,M0 at x=L. With phi=12EI/(KsL²), this gives exactly

```text
K=EI/[L^3*(1+phi)] *
[ 12, 6L, -12, 6L;
  6L, (4+phi)*L^2, -6L, (2-phi)*L^2;
 -12,-6L, 12,-6L;
  6L, (2-phi)*L^2, -6L, (4+phi)*L^2 ].
```

The two rigid modes are [1,0,1,0] and [0,1,L,1]. All non-rigid modes have
positive energy for positive EI,Ks,L. The negative (2-phi) entry at large phi
does not imply negative energy. The full spatial frame requires six rigid
modes and the physical plane sign map theta=rz for uy, theta=-ry for uz.

For t=x/L the exact homogeneous displacement shape row is

```text
Nv=1/(1+phi)*[
 1-3t^2+2t^3+phi*(1-t),
 L*(t-2t^2+t^3+phi*(t-t^2)/2),
 3t^2-2t^3+phi*t,
 L*(-t^2+t^3+phi*(-t+t^2)/2)].
```

Use feq=integral(Nv^T*q dx), plus the separately derived rotation-shape term
for distributed couples. The endpoint action is Kd-feq. Interior recovery
integrates equilibrium and the same constitutive equations, including the
loaded particular solution. Homogeneous interpolation alone omits interior
load deformation; EB Hermite reconstruction incorrectly imposes shear-rigid
kinematics. Point/partial loads need actual stations and one-sided values.

Full-span uniform q gives the familiar [qL/2,qL²/12,qL/2,-qL²/12]. This
coincidence cannot detect stale load projection. On [L/2,L], exact integration
instead gives

```text
[qL*(3+4phi)/(32*(1+phi)), qL^2*(5+8phi)/(192*(1+phi)),
 qL*(13+12phi)/(32*(1+phi)),-qL^2*(11+8phi)/(192*(1+phi))].
```

Both this and its phi=0 counterpart balance total force qL/2 and moment
3qL²/8. Thus equilibrium alone does not prove load compatibility. An exact
fixed-end transfer formulation is equivalent and avoids prescribing an
implementation. Splitting and condensing at the load boundary must preserve
endpoint responses and interval fields.

## 4. Original exact fixtures and realistic companions

All expected values below are independently derived, not product observations.
A cantilever fixes v(0)=theta(0)=0.

| Load | v(L) | theta(L) | Additional check |
|---|---|---|---|
| Tip force F | FL³/(3EI)+FL/Ks | FL²/(2EI) | v'(L)=theta(L)+F/Ks; v'(0)=F/Ks |
| Tip couple C | CL²/(2EI) | CL/EI | V=0; shear term vanishes |
| Full uniform q | qL⁴/(8EI)+qL²/(2Ks) | qL³/(6EI) | Tip V=0, so tip slope equals theta; root slope=qL/Ks |
| Distal-half uniform q | 41qL⁴/(384EI)+3qL²/(8Ks) | 7qL³/(48EI) | Root resultants V=qL/2, M=3qL²/8 |
| Guided translation delta, both rotations zero | Prescribed delta | 0 | Tip shear=12EI*delta/[L³(1+phi)] |

The distal-half tip displacement follows independently by integrating point
force kernels s²(3L-s)/(6EI) and s/Ks over s in [L/2,L], not by relying
only on the equivalent-load vector.

At EI=1 N*m², Ks=12 N, L=1 m, phi=1:

```text
K=[6,3,-6,3; 3,5/2,-3,1/2; -6,-3,6,-3; 3,1/2,-3,5/2].
F=1 N: v=5/12 m, theta=1/2 rad, slope=7/12.
q=1 N/m full span: v=1/6 m, theta=1/6 rad.
q=1 N/m distal half: feq=[7/64,13/384,25/64,-19/384],
                       v=53/384 m, theta=7/48 rad.
```

This is an abstract element fixture. Its large numerical rotations are not a
small-rotation physical specimen; scaling all loads down preserves the check.
The stale EB distal-half vector gives v=9/64=54/384 m with the new K, while
theta=7/48 and the force/moment balances still pass. This is a strict negative
control against a stiffness-only correction.

For an original exact SI pipe fixture, use ro=.1 m, ri=.09 m, E=200e9 Pa,
nu=.3. Direct arithmetic gives

```text
A=19*pi/10000 m^2; I=3439*pi/400000000 m^4; Jtorsion=2I
G=1e12/13 Pa; EI=1719500*pi N*m^2
kappa_C=1277679/2397284=.5329693936972006
kappa_E=16609827/33068756=.5022815796276098
Ks=606897525000000*pi/8267189 N=230625525.3172303 N.
```

For F=100*pi N, L=1 m, the exact responses are
`v_b=1/51585 m`, `v_s=8267189/6068975250000 m`,
`v=125917189/6068975250000 m`, `theta=1/34390 rad`,
and `slope=184742189/6068975250000`.
Lengths 2 and 10 m and all exact fractions are in the arithmetic output.
For the more ordinary F=100 N:

| L (m) | phi | Bending displacement (m) | Shear displacement (m) | Total displacement (m) | Section rotation (rad) |
|---:|---:|---:|---:|---:|---:|
| 1 | .2810773990650234 | 6.170590020040529e-6 | 4.336033483823956e-7 | 6.604193368422925e-6 | 9.255885030060794e-6 |
| 2 | .07026934976625585 | 4.936472016032423e-5 | 8.672066967647911e-7 | 5.023192685708902e-5 | 3.702354012024318e-5 |
| 10 | .002810773990650234 | .006170590020040529 | 4.336033483823956e-6 | .006174926053524352 | .0009255885030060794 |

Shear/bending displacement ratios are about 7.0269%, 1.7567%, and .070269%,
following phi/4. These are response sensitivities, not adopted tolerances or a
universal L/D cutoff. The shortest case is a model oracle; comparison with a
physical specimen still needs a qualified end/support/traction definition.
Cowper reduces this section's shear compliance by 476016/8267189, about
5.757894249%, relative to the selected energy coefficient. Using the solid
Cowper factor 39/44 for the annulus gives only .9715498254968192 of the
correct total displacement at L=1 m, but .9996957203176917 at L=10 m.
The long member alone poorly discriminates the wrong section factor.

## 5. Independent contract and indeterminate-assembly check

The parent asked for bounded refutation of its draft CONTRACT.md. Its section
factor, stable denominator form, plane signs, homogeneous stiffness,
load/recovery equations, partial-load vector, material-basis obligations and
explicit approximation/unsupported-domain interface are consistent with the
derivation here. A final S-C finding required qualifying the statement that
doubling E halves displacement without changing reactions: a fixed-valued
ground spring does not scale with beam E. The parent repaired the statement
by restricting it to the force-loaded S-B cantilever, requiring all contributing
stiffness to scale for a general model, and naming S-E as the counterexample.
The repaired wording was backchecked; no unresolved blocking finding remains.
The final inspected hash is recorded separately. This does not cover later edits or future
implementation. The contract correctly avoids using Ashear as metal area or
using V/(kappa*A) as a maximum physical shear stress.

For the proposed fixed-root cantilever with tip F and a ground spring k at
interior station a_s, reciprocity and force compatibility give

```text
c_aL=a_s^2*(3L-a_s)/(6EI)+a_s/Ks
c_aa=a_s^3/(3EI)+a_s/Ks; c_LL=L^3/(3EI)+L/Ks
u(a_s)=F*c_aL/(1+k*c_aa); R=-k*u(a_s)
u(L)=F*c_LL+R*c_aL
theta(L)=(F*L^2+R*a_s^2)/(2EI)
root force=-(F+R); root moment=-(F*L+R*a_s).
```

These formulas follow from the same point-force integration and the spring
constitutive law; the parent supplied the candidate values and this TASK
independently checked them with exact fractions. At EI=1, Ks=12, L=1,
a_s=1/2, F=1, k=12 in SI, the values are
`[R,u(a_s),u(L),theta(L),root force,root moment]`
`=[-7/8,7/96,37/128,25/64,-1/8,-9/16]`.
For the EB comparison they are
`[-5/6,5/72,71/288,19/48,-1/6,-7/12]`.
Thus even the resultants change in an indeterminate assembly.

The realistic pipe above with L=2 m, a_s=1 m, k=1e6 N/m, F=100 N gives
the same ordered vector approximately
`[-14.877537081183531,1.4877537081183531e-5,4.7872337812249824e-5,`
`3.564649239270416e-5,-85.12246291881647,-185.12246291881647]`.
These quantify an actual two-element/spring live-path reference for the
implementing owner, not a claim that it was run in the application.

## 6. Locking, invariance and return boundary

Fully integrated equal-order linear v/theta interpolation gives free-tip block
`[Ks/L,-Ks/2;-Ks/2,EI/L+Ks*L/3]` with a fixed left end. Direct inversion
under tip force gives `v_linear/v_exact=phi/(1+phi)`. It tends to zero as
phi->0, whereas the exact normalized compliance is `v_exact/v_EB=1+phi/4`.
At phi=1 it already halves deflection; at phi=1e-6 the bad ratio is
1/1000001. A coefficient correction alone does not cure this locking.
The closed-form element avoids that interpolation defect; floating-point
conditioning and rank tolerances remain separate checks under existing policy.

The retained standard-library Python Fraction arithmetic verifies both
section-factor identities in 16 rational parameter cases, stiffness symmetry,
two planar rigid modes, load fixtures, stale-load failure, transverse covariance
under a 3–4–5 rotation, locking limits, and spring-system values. It imports no
production source. These are mathematical checks, not product verification.

Implementation still owes six spatial rigid modes, general proper-3D-rotation
covariance, node-order reversal, work/energy identities, compatible full/partial
and point/couple loading, particular-solution interior recovery, exact-element
subdivision/condensation invariance, and slender-limit comparisons in normalized
quantities. Actual core/headless/native solves must carry theory/material/
section/load provenance and distinguish theta from centerline slope. A changed
shear law requires a fresh indeterminate solve and resultant/stress recovery.

No universal L/D switch is recommended. Small shear influence in one response
does not prove the accuracy of other responses or local fields. Shell
ovalization/Brazier behavior, restrained/end warping, load introduction,
attachments, section distortion, curved-pipe flexibility, finite strain,
anisotropy and dynamics remain beyond this straight-annulus reduced reference.
No test tolerance or protected acceptance criterion is loosened here.

The derivation/reference assignment is complete. Refuted shortcuts include
solid-circle substitution, treating Cowper as identical to shear-energy
matching, stiffness-only replacement, stale partial-load vectors, theta=v',
fully integrated linear interpolation without locking qualification, and a
universal span/diameter cutoff. Integrated implementation review and actual
candidate verification remain with the parent and implementing owner. No
product, test, build, native/browser UI, endpoint or Git mutation, and no
further delegation, was performed by this TASK.
