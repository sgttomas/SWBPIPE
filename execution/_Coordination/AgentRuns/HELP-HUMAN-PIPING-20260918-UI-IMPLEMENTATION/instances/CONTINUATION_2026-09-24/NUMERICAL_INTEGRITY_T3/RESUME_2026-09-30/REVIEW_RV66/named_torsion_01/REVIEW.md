# RV66 — named torsional-shear feasibility

**Disposition: verified obstruction at the two captured ordinary scales and at
one recomputed hypothetical native-primary projection scale.** No common real
center satisfies the required SharperExact cover at those fixed scales. No
binary64 center satisfies SharperBinary64 there either. Changing classification
to Absolute cannot evade the obstruction. This is not a theorem for arbitrary
future scales, an executed retained-value producer, source-code acceptance, or
a public-availability ruling.

TASK Type 2 `/root/rv66_named_torsion_feasibility`, parent ROOT `/root`, native
harness delegation, no descendants. Receipt/host clock: 2026-10-03 07:05:35 UTC;
checkpoint10 07:15:35; analysis cutoff20 07:25:35; seal deadline30 07:35:35.
Only this new packet and sibling `scratch/rv66_named_torsion` were written.
No compiler, model, solver, native product run, Git/index/API write or maintained
edit occurred. `check.py` is an independent offline Fraction checker, not a
product tool. The configured model was inherited; no model override was made.

## Frozen basis and binding

R means this packet's `RESUME_2026-09-30` ancestor; T3 means its parent.
Dispatch basis is `dbffe0d814debfe6b892f56e6a6289c06fc1e2fa`.
R/verification/i50_first_observed_02/FIRST_RUN.json is byte-identical to
`d87eb0fb59`; all twelve named files matched their SHA-256 and size. The two
I50_RECORD objects in pp_named_second.log supply actual inputs/results. The
preserved command exited101 on dense row-count99 versus98; this review does
not relabel that command as successful or rewrite its original evidence. Author
oracle files were hash-checked only; none supplied a truth, expected result,
algorithm or numerical constant. Immutable changed-source snapshots explain
capture and row association. Public source inspected in NUM was verified equal
to `d0daa18717f8243a7232e898c9ef9b4f4d18d9e4`; CODE was not read.

The checker independently decodes canonical K4SRC and K4LED bytes, checks their
complete consumption, and cross-checks request, member properties, constraints,
loads, springs, stations and support groups. Both captures bind one case `case`,
member M1 (kernel member0), nodes N0/N1, base material mat:N with independent
E=200e9 and G=80e9 Pa, no temperature basis, no pressure, no wall tolerance.
Normalized D=0.2, effective t=0.01 and c=D/2 are exact binary64 inputs, not exact
short decimal numbers. Their bits and built properties are in RESULTS.json.
Built J is `3f0c52664442210e`, c is `3fb999999999999a`, A is
`3f7872fa3a37ac15`, and operational Z is `3f31b37feaa954a8`.

The inspected row is **result:stress:M1:end-i:torsional-shear**, kind
`element_local_torsional_shear_stress`, entity M1, load_case basis `case`,
location `end_i`, unit MPa, positive along the j-side section action toward end j.
It is zero-based row80 in sparse and row81 in dense. Its raw bits are respectively
`3efbf3ab94308c78` and `3efbf3ab9430390f`; RESULTS.json is the checked bit record.
Normalization is n=RN64(y*1e6). Stress factor k=1.

## Required readouts and statics

B1 RETURN §§1,3–4 and ROOT_RULINGS_V1 lines5048–5067 retain the distinction
between admitted K, source geometry, actual output, and operational scales.
B1C RETURN §§1–3 and ROOT's adoption at5103–5121 warrant positive source annulus
from normalized D/effective t. I36 preview RETURN §§1–3 and ROOT adoption
at5227–5238 select the faithful private cover of admitted-bit and source-annulus
readouts, preserving independent selected E/G. I36 coefficients RETURN §1
requires actual resolved moduli as well as exact selected material source values;
its correction was accepted at5342–5351. Here base selection makes those two
material readings identical. This check does not silently drop any selected
readout, replace G by an E/nu recipe, or make K exclusive public truth.

The source chord is d=(1,2,2), L=3, e1=d/3. Its y-reference (1,0,0) gives
Gram–Schmidt e2=(4,-1,-1)/(3 sqrt(2)) and e3=(0,1,-1)/sqrt(2), consistent with
retained/assemble.rs:240–279. Let x be exact binary64 `3f73a92a30553261`.
The canonical ledger has only the tip moments (x,2x,2x); the doubled values are
exact, bits `3f83a92a30553261`. Hence the tip moment is **T e1, T=3x**,

    T = 16602069666338595 / 1152921504606846976 N*m.

N0 has three zero translational prescriptions and three positive rotational
springs (144,1e6,1e6). Equilibrium gives root support moment −(x,2x,2x) and no
force. Root rotation is (x/144,2x/1e6,2x/1e6); its rigid-body motion is compatible
with the free tip. Superpose a pure relative twist T L/(G J) along e1. This
satisfies the beam and spring laws, with no internal bending/shear/axial force;
positive beam/spring energy and restrained root translation ensure uniqueness.
It applies to both positive section operators. Thus the signed j-side section
T is identical in K and source mechanics; the end-i nodal action is −T, whose
stress recipe reverses the sign (frozen final_case.rs: action).
Rounded ordinary and native recovered torques are not used as this truth.

Let c=D/2, ri=c−t and g=(c²−ri²)(c²+ri²). The two necessary readouts are

    K = T c / J_K;       s = T c / J_source,
    J_source = pi g / 2.

Actual operational Z is retained only for scales. The physical torsion recipe
is T*c/J (stress_recovery/lib.rs:899–927), not an algebraic T/(2 Z_hat) shortcut.

## Independent exact numerical result

The checker brackets pi using **pi=4(atan(1/2)+atan(1/3))**, with180 terms of
each alternating series. Both even partial sums are strict lower bounds; the
next term bounds each positive remainder. Tangent addition gives1, and both
angles sum inside(0,pi/2), proving the identity. Directed dyadic rounding gives
a pi interval of width2^-256. This is independent of B1C's Machin constants and
the author's oracle. Positive reciprocal endpoints rigorously enclose s.
All decisions use exact integers/Fractions; decimals below are descriptive.

| Quantity | Independently derived value, Pa |
|---|---:|
| Source-annulus stress s | 26.6569488865750778721060822962495… |
| Admitted-K stress K | 26.6569488865750634305754810710417… |
| Strict separation s−K | 1.4441530601225207768959694663760…e−14 |
| Sparse actual n | 26.6569488866050150477349234279245… |
| Dense actual n | 26.6569488865326675863798300269991… |
| Sparse |n−s| | 2.99371756288411316750…e−11 |
| Dense |n−s| | 4.24102857262522692504…e−11 |

The author's displayed near-values are consistent binary64 approximations;
they were not the checker inputs. Exact rational endpoints and every separation
fraction remain recoverable in the scratch results and independent script.
Sparse's actual verdict is RelativeVerified, refusing SharperExact and
SharperBinary64 while passing both decimal predicates. Dense FIRST_RUN has
**no verdicts**: its point-error and scale results here are algebraic checks of
captured rows, not an inferred dense certificate result.

## Fixed-scale proof, with proposed-value dependence

Write a=2^-64(1+2^-21), b=2^-53, h=2^-1074. The actual required exact allowance is

    A(n,S)=a max(|n|,S)+b|n|+h.

Do not freeze A at an old row value. If a proposed real center n passes the
represented inequality |n−K|≤A(n,S), then max(|n|,S)≤|n|+S implies

    |n| ≤ N = (|K|+aS+h)/(1−a−b),
    A(n,S) ≤ B = a max(N,S)+bN+h.

If it also passed the source inequality, triangle inequality would require
s−K≤2B. Exact rational calculation instead proves s_lower−K−2B>0 at every
scale in the following table. This rules out **all real centers**, hence every
possible binary64 normalized value and every MPa raw-to-SI roundtrip.

| Fixed scale | S (Pa; descriptive) | Scale bits | s_lower−K−2B lower margin |
|---|---:|---|---:|
| Captured sparse ordinary rows | 54.11804906462261755 | 404b0f1c3b53f053 | >8.5166314237e−15 Pa |
| Captured dense ordinary rows | 54.11804906447574126 | 404b0f1c3b539f94 | >8.5166314237e−15 Pa |
| Hypothetical native primary projection, both modes | 54.11804906456180930 | 404b0f1c3b53cee5 | >8.5166314237e−15 Pa |

In all three, B<2.962449588731e−15 Pa. One failed required predicate suffices.
We also checked **operational SharperBinary64 separately**. For finite
nonnegative operands, RN(z)≤(1+b)z+h. With l=1+b and q=1+2^-21, the specified
five-step allowance is bounded above by

    a_op max(|n|,S)+b_op|n|+h_op,
    a_op=l³a; b_op=l³b; h_op=h(l²q+2l²+2l+1).

Apply the same N/B argument with these coefficients. Its strictly positive
separation margins are in RESULTS.json. This operational conclusion concerns
finite **binary64** centers; the exact comparison theorem concerns real centers.
It does not conflate the two allowances or rely on their rounding direction.

Classification cannot escape the result. These scales exceed2^-988. A center
classified Absolute has |n|<RN64(2^-34 S), about3.15e−9 Pa. Its absolute bound
RU64(2^-64 S) is about2.934e−18 Pa. For every table row, the checker proves
K−threshold>bound (gap over26 Pa). The stress row is not InputDerived or
NonQuantity under the unchanged table. No classification/coverage change is
selected.

## Projection scope, control and consequence

D1 §4.1.6.1 supplies the closed row classes, exact maxima of normalized values,
body extent and original-operand coupling. Frozen final_case.rs:757–805 and
900–1119 implement the relevant sequence. The checker rebuilds ordinary primary
maxima, excluding prescribed DOFs and nonquantity records, uses L_b=3, then
fo=max(S_force,RN(S_moment/3)) and mo=max(S_moment,RN(3*S_force)); component stress
scale is RN(RN(fo/A_hat)+RN(1*RN(mo/Z_hat))). Sparse matches its actual verdict
scale bits exactly. The dense parity observation is nonquantity under D1 and
contributes no scale, independent of its unfinished binding implementation.

The58 actual captured native rows in each mode suffice for the **hypothetical
primary projection's scales**: use captured native primary values, output
translations/magnitudes as RN(value*1000) mm and renormalize by RN(/1000), retain
native magnitudes, put the attributed native reaction/spring values into their
support slots, and zero absent support components. Exclude InputDerived DOFs.
The source support map proves each nonzero component is already represented by
its sole captured native contributor. No stress/maximum row feeds the primary
maxima. Both native captures select p128, so no p512 floor applies.
The resulting mo=0.01439999999999999787114735… N*m and
fo=0.004799999999999999579503… N give the table's projected scale. This is not
an executed complete producer, nor a check of all projected rows, G5a,
observables, custody, resources or publication.

A discriminating scalar control sets S=2^18 Pa solely for this offline test.
The MPa center `3efbf3ab943069f2`, normalized `403aa82dcd5efbca`, passes both
sharper and both decimal inequalities for the same two readouts. This shows the
checker does not manufacture an arbitrary-scale impossibility. It is **not**
an admissible scale proposal or evidence that a full coupled producer can attain
that scale. No future scale closeness to the present or verification scales is
assumed; a theorem constraining every future producer's coupled scales remains
outside this assignment.

Consequently the planned first publishing case cannot succeed by changing this
stress center alone at either current scale, or at the specified native-primary
projection scale. Tightening conservative intervals cannot eliminate this exact
readout separation. This is no false K publication finding, no executed false
retained-product publication, and no automatic protected-availability violation.
I48 RETURN's distinction was checked against D1 §7.1/product lane and F2a gate,
and D2 §§4.9.3–4/native S-G1 gate: refusal is not a successful publication witness,
and the required public route/complete producer has not been established here.
ROOT retains milestone, public-contract and owner-held availability decisions.
No replacement truth, tolerance, output, algorithm, fixture or availability
exception is selected.
