# Independent reaction, stress, combination and extrema reference

Date: 2026-09-24. TASK: `/root/correctness_design/stress_reference`.
Parent: `/root/correctness_design` (HELPS_HUMANS); root: `/root`.
Mechanism: executing delegated-harness-native descendant, explicitly configured
`gpt-6-astra` / `xhigh` with `fork_turns="none"`; no further delegation. Candidate inspected: `0be12dc968d78b747beb5e0d10c32e12cc991d75`.

This is a bounded analytical design/reference return for M-05, M-08, M-14,
M-15 and M-33, plus the parent's requested pressure companion cross-check.
The owner activation authorizes correctness work. This TASK writes only this
file. No product changes, product tests, builds, native/browser interaction,
Git mutation, acceptance or release occurred. A Python standard-library
arithmetic evaluation checked displayed decimal conversions; it did not call
product code. Source was read only to identify implementation ownership after
deriving the reference, and does not supply the expected values.

## 1. Quantity and sign contracts

Publish each ordinary support action as
`R_support_on_pipe = [Fx,Fy,Fz,Mx,My,Mz]`, in the global right-handed frame,
with force units N and moment units N*m. This is the support's action on the
pipe; the pipe's action on the support is its negative. Moment components
refer to the associated support/node origin. Retain force norm and moment norm
as separately named derived values. Never take a six-component Euclidean norm
mixing forces with moments.

For section recovery, use a local right-handed `(x,y,z)` frame. The section
traction is the action of the positive-x material on the negative-x free body,
on its positive-x cut face. Define `N` positive in tension, `T=Mx`, and signed
`My,Mz` by right-hand rule. Then, with transverse coordinates measured from
the section centroid,

```text
sigma_x(y,z) = N/A + My*z/Iy - Mz*y/Iz
tau_xy(y,z) = -T*z/J
tau_xz(y,z) =  T*y/J
```

These signs follow directly by integrating traction and `r cross traction`:
`N=integral(sigma_x dA)`, `My=integral(z*sigma_x dA)`, and
`Mz=-integral(y*sigma_x dA)`. An end-node action and a cut-face action can have
opposite signs; transform all six consistently before combining/recovering.
Store the convention, local frame and station, rather than depending on an
ambiguous i/j endpoint label.

For a proper orthogonal frame matrix Q whose columns are local axes in global
coordinates, `F_local=Q^T F_global` and `M_local=Q^T M_global` at the same
origin. Moving the moment origin from P to O additionally requires
`M_O=M_P+(r_P-r_O) cross F`. A frame rotation is not an origin shift.

## 2. Reaction reference and full equilibrium (M-05)

For a static free body and arbitrary global origin O, check independently:

```text
sum(R_force) + sum(applied forces) + integral(distributed force ds) = 0
sum(R_moment + (r_support-O) cross R_force)
  + sum(applied couples + (r_load-O) cross applied force)
  + integral(distributed couple + (r(s)-O) cross distributed force) ds = 0
```

Use the actual complete external load ledger. Pressure cap/wall forces,
constant-effort supports, springs, imposed motion and constraint forces must
each appear exactly once under the chosen subsystem definition. Internal
element forces cancel across compatible cuts and are not additional external
loads. Force residual and moment residual need separate dimensions/scales.

**R1 — general signed cantilever.** Root at `(0,0,0)` m, tip at `(2,0,0)` m;
apply tip force `(10,-20,30)` N and tip couple `(4,5,-6)` N*m. Since
`r cross F=(0,-60,-40)` N*m, the unique root reaction is
`(-10,20,-30; -4,55,46)` in N;N*m. Adding a uniform load `(0,-3,0)` N/m
over the 2 m span adds `(0,-6,0)` N and `(0,0,-6)` N*m, giving root reaction
`(-10,26,-30; -4,55,52)`. This distinguishes force balance from moment
balance and tests sign, distributed loading and all six published components.

**R2 — pure torque.** A `+4 N*m` tip torque about x gives root
`(0,0,0; -4,0,0)`. Force norm is zero; moment norm is 4 N*m. Any presentation
equating the zero force norm with zero anchor demand fails this reference.

**R3 — spring and ownership.** A translational spring gives
`R=-k*(u-u_base)` on its axis, plus a separately defined preload if present.
A rotational spring uses the corresponding relative rotation and N*m/rad.
If support stiffness is included in assembled K, the residual `K*u-F`
does not by itself recover each spring force. Recover spring/contact laws and
rigid-constraint residuals with one clear accounting convention. Inactive
contact force is zero even if another restraint at the same node has a nonzero
nodal residual. A nodal total does not determine a unique split between
coincident perfectly rigid restraints. Preserve duplicate-constraint rejection
or report an unattributed nodal total with an explicit diagnostic; do not
invent individual reactions. For co-located springs, each force follows its
own stiffness and relative motion, and their sum must match the nodal balance.

Acceptance controls: rotate R1 and its whole geometry/load set by a known Q;
translate the global origin; reverse all signed linear loads; verify spring
force under nonzero base displacement; inspect inactive contact and constant
effort case inclusion. Components must survive result combination and export
with their case, support association, frame, origin and source result IDs.

## 3. Circular-annulus stress reference (M-14)

For a homogeneous circular annulus with outer radius ro and inner radius ri,
direct polar-area integration gives

```text
A = pi*(ro^2-ri^2)
Iy = Iz = I = pi*(ro^4-ri^4)/4
J = 2*I
Z = I/ro
```

On the outer circle `y=ro*cos(theta), z=ro*sin(theta)`, the bending part is
the dot product of `(My,-Mz)` with `(sin(theta),cos(theta))/Z`.
Cauchy–Schwarz, with equality at the aligned/opposed vectors, gives

```text
B = hypot(My,Mz)/Z
sigma_plus = N/A + B
sigma_minus = N/A - B
max_abs_normal = max(abs(sigma_plus),abs(sigma_minus)) = abs(N/A)+B
max_abs_torsional_shear = abs(T)*ro/J
```

`sigma_plus` is the maximum signed normal stress and `sigma_minus` the
minimum signed normal stress; they are not principal stresses. The fiber
coordinates for sigma_plus when B is nonzero are
`(y,z)=ro*(-Mz,My)/hypot(My,Mz)`. The opposite fiber gives sigma_minus.
When B=0, every fiber is tied. Record this instead of fabricating an angle.

This is elastic circular-section normal/torsional recovery. It excludes
transverse shear, ovalization/local shell effects, a piping-code stress
definition and pressure hoop/radial stresses. If N is physical wall axial
force, pressure's axial contribution is already in N: adding another
longitudinal pressure stress would double count it. A mechanically separated
axial term needs an explicit, compatible reconstruction of wall stress.

For foundational corroboration only, MIT's [Unified Engineering handout,
printed pages 1 and 3](https://ocw.mit.edu/courses/16-01-unified-engineering-i-ii-iii-iv-fall-2005-spring-2006/1f1b4e82bae2ff8ce609bc96fe20eedb_sprh04.pdf)
states elastic bending and circular-shaft torsion relations. The annulus
integration, circular maximum and fixtures above/below are independently
derived here, not taken from product recovery or an extracted project equation.

**S1 — consistent SI annulus and biaxial state.** Use `ri=0.04 m`,
`ro=0.05 m`, no pressure, no transverse shear, and

| Quantity | Exact fixture value | SI decimal |
|---|---:|---:|
| A | `0.0009*pi m^2` | 0.002827433388230814 |
| I | `0.0000009225*pi m^4` | 0.000002898119222936584 |
| Z | `0.00001845*pi m^3` | 0.00005796238445873168 |
| J | `0.000001845*pi m^4` | 0.000005796238445873168 |
| N | `1800*pi N` | 5654.866776461628 |
| My | `55.35*pi N*m` | 173.8871533761950 |
| Mz | `73.8*pi N*m` | 231.8495378349267 |
| T | `221.4*pi N*m` | 695.5486135047802 |

The signed axial/bending amplitudes are `(2,3,4)` MPa. Hence B=5 MPa,
`sigma_plus=7 MPa`, `sigma_minus=-3 MPa`, `max_abs_normal=7 MPa`, and
torsional shear magnitude is 6 MPa. The governing tensile fiber is
`(y,z)=(-0.04,0.03)` m. Integrating the explicit stress field recovers N,
My and Mz; integrating its shear traction moment recovers T.

The component-absolute sum produces 9 MPa and therefore fails S1. Rotating the
section axes to align with the bending vector changes `(3,4)` to `(5,0)`
while preserving the correct 7 MPa result. The old sum changes from 9 to 7 MPa.
This is a decisive rotation-invariance control.

**S2 — torsion isolation.** Set N=My=Mz=0 but keep S1's T. Normal extremes
are zero and torsional shear is 6 MPa. Label and expose both quantities.
If a future explicitly named von Mises quantity is requested for the restricted
axial-normal-plus-torsion stress state, its point value is
`sqrt(sigma_x^2+3*tau^2)` and the S1 outer-surface maximum is
`sqrt(157) MPa`. This is not the default normal-stress headline and does not
apply unchanged after adding pressure hoop/radial or transverse shear stresses.

## 4. Directional intensification (M-08)

First resolve signed section bending moments into the documented component
frame and component side. The definition must say which moment axis represents
in-plane and out-of-plane bending; the bend-plane name alone is insufficient.
For declared user factors `i_ip,i_op`, define a separately named intensified
bending measure from `(i_ip*M_ip,i_op*M_op)`, with the stated section basis.
This is a user-defined intensification measure, not proof of actual local
elastic shell stress or compliance with a code edition.

Do not multiply N/A, pressure membrane stress, torsion or the already recovered
max-normal scalar by a bending factor. A torsion factor, if supported, needs
its own quantity, applicability and source. Flexibility belongs in the
stiffness formulation; holding the recovered resultants fixed while changing
flexibility must leave recovered stress unchanged. Re-solving with changed
flexibility may legitimately change the resultants and thus stress.

**I1 — directional fixture.** In the named component frame, take S1's bending
amplitudes as `(3,4)` MPa and factors `(2,3)`. The intensified bending measure
is `hypot(6,12)=6*sqrt(5) MPa`, and an explicitly declared
axial-plus-intensified-bending normal measure is `2+6*sqrt(5) MPa`, about
15.4164 MPa. Axial remains 2 MPa and torsion remains 6 MPa. With pure first-axis
bending only its factor acts; with pure second-axis bending only that factor
acts. Swapping axes requires swapping the factor directions as well. A rigid
rotation of the complete geometry/component frames leaves the result invariant.

Coverage must follow the declared component mechanics: named bend member(s),
both tangents and applicable interior stations; each tee side has its own
mapping. A component node's presence does not justify modifying every attached
pipe without a component-side association. Missing directional applicability
must be explicit; it is not permission to apply an arbitrary scalar everywhere.

## 5. Signed algebra, nonlinear states and envelopes (M-08/M-15/M-33)

Use three explicitly distinct contracts:

1. **Combined-load equilibrium solve:** combine authored loads, prescribed
   motions, temperatures/pressures and case-dependent support inputs with their
   physical semantics, then solve that state. For nonlinear contact/friction,
   check equilibrium, compatibility and admissibility in the resulting state.
   History-dependent states also need their actual load path/base state.
2. **Signed result algebra or state difference:** combine matching signed
   section actions, displacement components and reaction components in a common
   frame and station, then recover derived quantities. State subtraction is a
   response difference between two solved states. It need not be an equilibrium
   state attainable by applying the difference load to the original model.
3. **Envelope selection:** select a named scalar quantity among valid states,
   retaining the governing case/station/fiber and source row. A componentwise
   envelope may select different cases for different components; it is not one
   coherent reaction or section-action vector and cannot be fed into stress
   recovery as though all its maxima occurred simultaneously.

Linear solution superposition additionally requires the same linear operator,
frame, boundary conditions and compatible reference/preload treatment. It
must not repeat a baseline preload merely because that preload occurs in each
operand. Vendor documentation independently corroborates the limitation:
[Abaqus, linear versus nonlinear analyses](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEGSARefMap/simagsa-c-nlnlinvsnonlin.htm)
requires unchanged boundary conditions for linear superposition and separate
solutions for nonlinear load cases. The counterexample below is derived here.

**C1 — opposite signed states.** Let q be S1's signed `(N,My,Mz,T)` and let
case A have q, case B have -q on the same section/frame/basis. Both states have
max-absolute normal stress 7 MPa. The signed sum A+B is zero and recovers zero
normal/torsional stress; adding their magnitudes incorrectly gives 14 MPa.
The signed state difference A-B is 2q and recovers max-absolute normal range
14 MPa and torsional shear range magnitude 12 MPa; subtracting the two scalar
maxima incorrectly gives zero. At the fixed S1 tensile fiber the normal
stresses are +7 and -7 MPa, demonstrating the 14 MPa range directly. The
stress envelope over A and B remains 7 MPa, with both states tied.

**C2 — cross-axis range.** At a common section take states with bending
amplitudes `(3,4)` and `(3,-4)` MPa and the same axial stress. Their bending
magnitudes are both 5 MPa but the difference field has bending magnitude
8 MPa. Subtracting scalar magnitudes loses the range. A true maximum range
over multiple states requires pairwise response differences at a common
material location and the declared recovery basis, or a proven equivalent
optimization. `max(summary)-min(summary)` does not compute it.

**C3 — identical inactive operand states still invalidate superposition.**
A 1D node has spring `k=1e6 N/m` and a rigid stop at `u<=g=0.001 m`.
Let stop action be `-lambda`, with `lambda>=0` and
`lambda*(g-u)=0`. Static equilibrium is `F-k*u-lambda=0`.
Two separate cases each apply `F=800 N`; each has `u=0.0008 m`, `lambda=0`
and an inactive stop. Adding their results gives `u=0.0016 m`, `lambda=0`.
It satisfies the force equation for 1600 N but penetrates the stop. The actual
combined-load solution is `u=0.001 m`, `lambda=600 N`. Thus even identical
operand active sets and a zero equilibrium residual are insufficient.

For a difference example, subtract the solved 400 N state
`(u,lambda)=(0.0004 m,0)` from the solved 1600 N state
`(0.001 m,600 N)`. The response difference is `(0.0006 m,600 N)`.
Solving a fresh 1200 N load instead gives `(0.001 m,200 N)`. The legitimate
state difference must retain both source state identities, rather than being
relabelled as a fresh equilibrium solution.

**C4 — incompatible moduli and scalar rescaling.** A node restrained by two
parallel elastic paths, each initially `100 N/m`, under force 100 N gives
50 N in each path. Changing only the first path to `200 N/m` gives
`200/3 N` and `100/3 N`, not a common rescaling of both old reactions.
Changing all stiffnesses by the same factor in a force-controlled statically
determinate problem changes displacement but leaves reactions fixed; under
prescribed motion it can instead scale reactions. Therefore a hot/cold modulus
ratio is not a universal reaction transformation. Combining states with
different operators may be useful arithmetic or a physical state difference,
but not an unqualified combined equilibrium solve. Material-dependent
stress-range normalization needs its declared section/material/modulus mapping.

## 6. Complete straight-member station extrema (M-14, adjacent M-37)

The required normal maximum is `f(t)=abs(a(t))+sqrt(b(t)^2+c(t)^2)` on
each constant-section, constant-basis interval. Here `a=N/A` after the chosen
wall-pressure reconstruction; `b=My/Z`, `c=Mz/Z`, with any explicitly selected
constant directional factors already applied only to their moments. The
coordinate t is normalized within the interval. With piecewise-uniform
distributed loads, a is at most affine and b,c at most quadratic; allowing
a quadratic below covers the existing broader component representation too.

The identity `|a|+|b|+|c| = max of eight signed quadratics` belongs to the old
objective. It is not an identity for the circular-section objective. Evaluating
the new objective only at those eight quadratics' stationary points can miss
its maximum.

An exact candidate construction for real polynomial coefficients is:

1. Split the member at all load starts/ends, point actions, changes of section,
   pressure/basis/factor or other formula changes. Include endpoints and both
   one-sided values at discontinuities. Use analytically derived statics
   coefficients; three interpolated samples alone do not prove quadratic form.
2. On each smooth polynomial interval, set `D=b^2+c^2` and
   `E=b*b'+c*c'`. Include all roots of a and all common zeros of b,c. These are
   the possible nonsmooth points of f. Split at those points.
3. On each remaining open interval D>0 and `s=sign(a)` is constant (if a is
   identically zero, take its contribution and derivative as zero). An interior
   stationary point satisfies `s*a'*sqrt(D)+E=0`. Every such point is a root of
   `H=(a')^2*D-E^2`, a polynomial of degree at most six for quadratic a,b,c.
4. Isolate **all** real roots of H in the interval, including repeated roots.
   Retain only those satisfying the original unsquared stationary equation
   and the interval's sign/domain conditions. Squaring creates extraneous
   roots, so testing H alone is insufficient. Include the nonsmooth points and
   endpoints from steps 1–2, evaluate f, and select the greatest value with its
   station and fiber/source provenance.
5. Handle identically zero H explicitly. Subdivide additionally at roots of a'.
   Where a' is nonzero and D>0, the continuous ratio
   `E/(a'*sqrt(D))` is identically +1 or -1; f is constant or has derivative
   `2*s*a'` and is monotone on that subdivision. Endpoints suffice. If a' is
   identically zero, H=0 implies E=0 and D is constant, so f is constant.
   Identically zero b,c reduces the problem to maximizing |a|.

For finite-precision production, this construction needs a robust all-real-root
isolation implementation and residual/domain filtering, not a small number of
Newton starts. Exact/rational coefficients permit square-free factorization
and Sturm isolation; interval arithmetic or certified polynomial bounds can
bound the maximum for rounded coefficients. Near degeneracies must produce a
documented bound or an explicit unresolved-extremum diagnostic. An adaptive
interval method with a proven upper bound is an acceptable alternative. Dense
sampling is a useful independent convergence comparison, not a proof that no
peak was missed. Do not weaken a protected stress tolerance to hide search error.

**X1 — strict missed-peak counterexample.** Over `t in [0,1]` let a=0,
`b=4*t*(1-t) MPa`, `c=t MPa`. These are realizable straight-member moment
fields: one axis has uniform transverse loading and the other constant shear.
With S1's Z, moments are `My=4e6*Z*t*(1-t) N*m` and
`Mz=1e6*Z*t N*m`. The squared normalized bending magnitude is
`D=17*t^2-32*t^3+16*t^4`. Its interior stationary points are
`t=(6 +/- sqrt(2))/8`, from `D'=2*t*(17-48*t+32*t^2)`.
Comparing both roots and both endpoints gives the global maximum at

```text
t* = (6-sqrt(2))/8 = 0.5732233047033631
f(t*) = sqrt(71+8*sqrt(2))/8 MPa = 1.1340862821217075 MPa
```

The old signed sums supply only interior candidates t=3/8 and 5/8; the largest
new-objective value on those candidates/endpoints is
`sqrt(325)/16 MPa = 1.1267347735824966 MPa`, at 5/8. Thus merely changing
the value formula while retaining the eight-sign search provably misses this
peak. Set physical span L=1 m, then the uniform-load magnitude for the first
moment field is `8e6*Z/L^2 N/m`; endpoint loads/restraints must reproduce the
stated moments and shear in a live-path fixture. This reference is the section
statics field, not a claim that such a product fixture has already been run.

Additional controls: zero a; zero bending; constant D; a zero crossing;
simultaneous bending zero; repeated H root; extrema exactly on a partial-load
boundary; a short interval; nearly coincident roots; a pressure/material-basis
change; rigid section-axis rotation. Bent members require their own verified
function/bound; this polynomial proof does not extend automatically to arcs.

## 7. Per-case and overall governing maxima (M-33)

Each maximum record needs: quantity kind and units, scalar value, solve/case or
combination ID, entity, station/endpoint, fiber when meaningful, frame,
section/material/pressure/recovery basis, source result IDs and validity. A
straight-member maximum cannot be fully represented by an element ID if the
governing station is interior. Distinguish full-domain extrema from sampled
maxima, and distinguish maximum signed, minimum signed and maximum absolute.

Compute per-case maxima first. Compute an overall maximum across all requested
valid comparable quantities, including recomputed valid combinations where
their semantics permit inclusion. Comparability requires the same physical
quantity/meaning and convertible units; different physical section sizes alone
do not prohibit comparing actual normal stresses. Code stress measures or
utilizations with different rule/basis meanings need explicit grouping.
Unavailable/invalid quantities must not become zero; if any requested domain
is missing, disclose incomplete coverage. Resolve ties deterministically by
stable identity or retain all co-governing references, independent of case
input order. A user warning using an overall maximum must use the same domain
and source, with its threshold policy still explicitly identified.

**G1 — order control.** In a valid linear fixture, case A has S1's q and case
B has 2q: per-case normal maxima are 7 and 14 MPa; overall maximum is 14 MPa
and points to B's governing section/fiber. Permuting `[A,B]` to `[B,A]` must
not change it. Independently set displacement cases with maxima 1 and 2 mm;
overall displacement maximum is 2 mm at the actual source node/case. Add an
invalid/unavailable later case: do not silently claim complete overall coverage.
For an envelope, selecting max Fx from A and max Fy from B does not establish
that the norm of that synthetic vector ever occurred in either case.

## 8. Pressure companion cross-check requested by parent

Inputs: `ri=.05 m`, `ro=.06 m`, `L=6 m`, `E=200e9 Pa`, `nu=.3`,
internal gauge pressure `p=2e6 Pa`, exterior pressure zero, and uniform free
thermal strain `e_th=.0012` when specified. Assume isotropic, small-strain
linear elasticity, axisymmetry, negligible end effects, freely deformable
radial surfaces, and explicitly named cap load paths. This is a consistency
cross-check of the parent's fixture, not new qualification of the dormant kernel.

Direct area and elasticity give `Ai=.0025*pi m^2`,
`As=.0011*pi m^2`, `P=p*Ai=5000*pi N`, `EA=220e6*pi N`, and
`G=E/(2*(1+nu))=(1000/13) GPa`. Radial equilibrium and the two radial
tractions give `sigma_r=C-D/r^2`, `sigma_theta=C+D/r^2`, where
`C=50e6/11 Pa`, `D=C*ro^2`. Thus `sigma_r(ri)=-2e6 Pa`,
`sigma_r(ro)=0`, `sigma_theta(ri)=122e6/11 Pa`, and
`sigma_theta(ro)=100e6/11 Pa`. Their sum is the constant `2C`.

Define wall force `Nw=As*sigma_z` and preserve the parent's effective-section
definition **`S=Nw-P` in every row**, including separate closures. S is not
itself a support reaction or a measure of cap load transferred into the pipe.
Axial Hooke compatibility is
`e_z=(Nw/As-nu*2C)/E+e_th`, with `u_j-u_i=L*e_z`.

| Configuration | Nw (N) | S (N) | Axial strain / relative displacement | Support-on-pipe axial reactions at i,j |
|---|---:|---:|---|---|
| Free closed ends, no thermal strain | `5000*pi` | 0 | `1/110000`; `3/55000 m` | `0,0` |
| Axially fixed closed ends, no thermal strain | `3000*pi` | `-2000*pi` | `0`; `0` | `+2000*pi,-2000*pi N` |
| Free wall, separate independently supported closures | 0 | `-5000*pi` | `-3/220000`; `-9/110000 m` | `0,0` on pipe |
| Free closed ends with thermal strain | `5000*pi` | 0 | `.0012+1/110000`; `0.00725454545454545 m` | `0,0` |
| Axially fixed closed ends with thermal strain | `-261000*pi` | `-266000*pi` | `0`; `0` | `+266000*pi,-266000*pi N` |

For separate closures the remote closure-support reactions are
`+5000*pi,-5000*pi N`, balancing the two cap pressure resultants. Those are
not pipe-support reactions. The nonzero negative effective S coexists with
zero Nw and zero axial pipe reaction under this explicit definition/topology.
For fixed closed ends the root reaction is `P-Nw=-S` and the far reaction its
negative. The realistic magnitudes, Poisson contraction, thermal cancellation,
wall stress versus effective force, and radial traction checks all agree with
the parent's supplied values. Do not infer a universal external bend thrust
from these straight-cylinder relations or add cap force twice.

## 9. Implementation ownership and integration checks

Current source coordinates below refer to the inspected candidate, not a
claim about later edits. Let P be `core/product_physics/src/lib.rs`, S be
`core/loads/stress_recovery/src/lib.rs`, and A be
`core/loads/load_case_algebra/src/lib.rs`, all beneath Piping.

| Finding | Source ownership observed | Required reference consequence |
|---|---|---|
| M-05 | P:1445–1503 reaction recovery/publication; P:9068–9069 keeps vectors private | Recover/attribute six signed components; preserve vector and support provenance; R1–R3 and global balance |
| M-08 | P:7658 onward component modifiers; P:7842–7847 scalar SIF/flexibility; P:9170–9223 scalar combination | Directional signed moment recovery, separate flexibility, no magnitude algebra; I1 and C1/C2 |
| M-14 | P:7632–7655 summary; S:1041–1053 summary; P:6463–6540 straight extrema | Change all affected definitions together; circular resultant/fibers, torsion label, X1 and degenerate extrema |
| M-15 | P:1037–1050 separate solves; P:9073 onward result algebra; A:28–40 operation kinds | Distinguish load solve, signed difference and envelope; C3 contact admissibility, not only residual |
| M-33 | P:1058–1063 first-case selection; P:667–678 summary/location shape; P:1183–1184 publication | Per-case/global extrema and governing case/station/basis; G1 and C4 |

A correction must update affected consumers and independently derived expected
values together; historical references remain historical. The old absolute-sum
summary and legacy scalar modifier contract are not physics oracles. This file
does not declare a protected test changed or passed. Parent/implementation
integration still owes actual live-path fixtures, candidate-sealed evidence,
independent review of the final change and required checks. A product result
matching only the scalar values while losing signs, station, case or quantity
identity has not met these references.

## 10. Supplied basis, provenance and limits

Instruction bodies actually read: Root AGENTS, full TASK role, Piping AGENTS,
Piping LOOP_INIT and owner activation. No other full role, workflow, skill,
governance corpus or unreviewed domain equation artifact was loaded. The prior
stress assessment and pressure clarification are assessment/authority context,
not a numerical oracle. The parent's later pressure request and definition
clarification were received directly through collaboration messages; those
messages supplied the fixture and required preserving S=Nw-P. All expressions
were independently checked by force/moment integration, extrema calculus or
Hooke/traction compatibility as set out here. One executing TASK is the
independent analytical contributor relative to the parent design; this is not
a claim of an additional reviewer checking this return.

Paths in the following SHA-256 table are repository-relative. `C` abbreviates
`projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24`;
`PIP` abbreviates `projects/chirality-piping`.

| Read source | SHA-256 |
|---|---|
| AGENTS.md | `c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd` |
| agents/AGENT_TASK.md | `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7` |
| PIP/AGENTS.md | `d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792` |
| PIP/loop/LOOP_INIT.md | `c712b6487faa3fab461e181aad1a6c16a2cb0eb734c510744e5037289105df1b` |
| C/CORRECTNESS_ACTIVATION.md | `a3a22c0af6250fafb611c776e605d2f36989c919948904047f373ce5c139c170` |
| C/SOLVER_FINDINGS_ASSESSMENT/_run_records/stress/RETURN.md | `3d6f40f5b9e2d0f5addc522922a3a59f53ff8862117e7c300c201231ada9c9fe` |
| C/SOLVER_FINDINGS_ASSESSMENT/ASSESSMENT.md (selected findings) | `c948cb13ac5d825b0d201074cad50e9b8f4d9de916bc246f574e8304121b72e6` |
| C/SOLVER_FINDINGS_ASSESSMENT/PRESSURE_THRUST_CLARIFICATION.md | `362e1d539898c2ae40949a5115a0f5e78975b69fe602be0677cf1d4b9e75945d` |
| PIP/core/product_physics/src/lib.rs (bounded function excerpts) | `aa91613c48346654dc7e8b110f22fc74cf9578fb16f21ae9c40cec863b910f15` |
| PIP/core/loads/load_case_algebra/src/lib.rs (operation definitions) | `fd70056f81320e33fad4f7e05152a4c83f2a97fd20aaaad384f25eff0b66a050` |
| PIP/core/loads/stress_recovery/src/lib.rs (summary function) | `0c3a0b6278a1cce4a802fc6c2e4c81af1b6f393c5cde4bb28ff55b99d5d956f2` |

External primary corroboration was retrieved 2026-09-24 through the web tool:
the linked MIT handout, Abaqus documentation and the original papers assessed
in section 11. MIT's course readings index and other search results were
discovery only. External response/PDF
bytes were not archived or hashed; these links are supporting context, while
the explicit derivations and exact SI fixtures in this file are the recoverable
oracle proposal. No proprietary piping-code formulas or commercial solver
outputs are asserted.

## 11. Current primary-source relevance check and newer alternatives

The parent relayed the owner's additional direction to verify analytical
references for accuracy and relevance, including newer theories/methods,
before dependent production changes. The search below was performed on
2026-09-24. It is a bounded relevance review, not a systematic literature
review or a claim that no other relevant method exists. Original technical
documentation and author-written paper HTML were inspected; third-party
summaries, search snippets alone and project OCR equations were not used as
the acceptance basis. A hash establishes which bytes were consulted, not
whether their mechanics are correct.

| Primary source and inspected scope | Relevant result and disposition |
|---|---|
| [Abaqus 2025: Choosing a Beam Element](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-beamelem.htm), Euler–Bernoulli, Timoshenko, coupled sections and pipe sections | Current documentation distinguishes shear-rigid/slender and shear-deformable beam models, plus coupled meshed-section behavior. **Retain** this reference's linear elastic circular resultants/stress relation under its stated assumptions; **do not infer** that it validates all displacement/stiffness models. Shear deformation must be assessed for short/stout spans. Vendor slenderness guidance is contextual, not an imported product acceptance limit or a substitute for convergence/compatibility checks. |
| [Abaqus 2025: Beam element formulation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAETHERefMap/simathe-c-beamform.htm), kinematics and circular-section subsection | The formulation explicitly treats section directors, stretch and warping, and its circular torsion specialization has no warping. **Retain** Saint-Venant circular torsion for the present small-strain isotropic annulus; **defer** finite-strain/stretch coupling and noncircular/constrained-warping generalization. This is not a claim that a circular section cannot warp under every possible transverse-shear or nonlinear state. |
| [Abaqus 2025: General beam section output](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-usingbeamgensect.htm), section-point output and MAXSS | Its linear normal-strain field is affine over the section; MAXSS selects from specified section points. **Select** analytic circumferential extrema for this circular linear field rather than treating sampled section points as a guaranteed full maximum. Station extrema remain a separate along-member problem, solved by section 6's derivation. |
| [Abaqus 2025: Mechanical contact properties](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEITNRefMap/simaitn-c-contactmechanical.htm) and [Coulomb friction theory](https://docs.software.vt.edu/abaqusv2025/English/SIMACAETHERefMap/simathe-c-coulombfric.htm), hard-contact properties and stick/slip law | Hard contact requires separation/contact consistency, while Coulomb friction links tangential capacity to current normal pressure and slip state; enforcement can be exact or approximate. **Select** equilibrium plus admissibility/compatibility as distinct checks. This reinforces C3 and the prohibition on assuming that equal operand states validate a nonlinear sum. Actual product penalty/regularization choices need their own law and convergence qualification. |
| [Alzate Cobo, Henkels and Weeger, 14 April 2026, original v1](https://arxiv.org/html/2604.12886v1), introduction, section 2.1, section 5 examples and section 6 conclusion | A newer computational cross-section warping formulation addresses hyperelastic finite-strain beam constitutive behavior with material stress/strain measures. **Defer as a replacement oracle** here: it changes the strain, constitutive and sectional assumptions beyond the selected small-strain annulus. It is a relevant future alternative if finite strain or nonlinear sectional warping enters scope, requiring its own stress-measure mapping and independent benchmark. The numerical examples were read, not reproduced; this is an author preprint, not a product qualification. |
| [Song, Fan, Ascher and Pai, 21 July 2026, original v1](https://arxiv.org/html/2607.19599v1), introduction, discussion/limitations and conclusion; [publication DOI](https://doi.org/10.1111/cgf.70576) | A newer splitting method solves the reduced Coulomb law using a cone subproblem and an outer coupling iteration. **Defer adoption** for piping statics: the paper's stated scope is rigid-body velocity-level contact; deformable position-level contact lies outside its implemented scope, and a formal convergence rate for its nonmonotone outer iteration remains open. It supports the need to check the full contact law, not replacing that law with a convenient magnitude or assuming newer means qualified for this solver. No reported performance or accuracy number is adopted. |

An independently derived shear-relevance comparison makes the choice concrete:
for a prismatic, linear elastic cantilever with end transverse force F,
Euler–Bernoulli bending displacement is `F*L^3/(3*E*I)`. If its selected
Timoshenko section shear stiffness is Ks (N), integrating constant shear
strain `F/Ks` adds `F*L/Ks`. Their ratio is `3*E*I/(Ks*L^2)`.
This shows why a short-span displacement oracle can require shear flexibility
even though its statically determined section moments and the circular normal
stress extremum above remain unchanged. Do not insert an unqualified solid-
circle shear coefficient for an annulus. Indeterminate structures can also
redistribute resultants when shear flexibility changes; they require a
compatible formulation and fresh solve, not this determinate argument.

Disposition: the current-source check found no reason to replace the selected
small-strain circular elastic stress relation with a nonlinear sectional theory
for S1/S2. It did identify explicit exclusion boundaries for transverse shear,
finite strain, noncircular warping, ovalization and nonlinear contact. The
mathematical extrema method and negative controls remain independently derived
proposals. **Literature/derivation acceptance by the integrating independent
review must be recorded before dependent production edits use this packet as
an oracle.** Existing accepted references and their hashes alone do not fulfill
that check. No dependent production edit or test was performed by this TASK.
