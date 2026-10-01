# Selected j-end numerator finiteness — 138 fixed rows

Status: **SOURCE CONSEQUENCE PROPOSED FOR INDEPENDENT CHECK.** Under the accepted
R7 verification-error premises and the actual unmutated selection path at
`40129a225d73860ac2a53da9a2fa73869df668f3`, every one of the named 138 actual
twist/extension quotient rows in 18 fixed cases is finite. The largest derived
absolute quotient bound is **2^1020**, below binary64 overflow. This is not an
executed solver observation, a new acceptance of source14, or a full E_max.

No numerical/contract amendment is needed **if this derivation is independently
confirmed**. It uses primitive geometry/properties/loads, a compliance bound,
the complete retained T/R stop rule, and the actual j-end recovery. It does not
use expected outputs, reference scales, previous passes, finite tags alone,
NotCovered, an expected-refusal list, or a new input restriction.

Aliases: P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
K=FK/src/structural/retained; VR=P/validation/benchmarks/numerical_robustness;
R is this resumed run. `DERIVED_BOUNDS.json` preserves all 138 row identities,
the two exact family-file hashes, all intermediate exponents and the result.
`derive.py` reproduces only exact rational bounds from the immutable stored
primitive bits; it builds no stiffness matrix, model, solver state or expectation.

## 1. Exact scope and actual numerator

VR/src/lane.rs:101–103,162–169 resolves extension to the **published J-end Ux
action**, and twist to the **published J-end Rx action**. It divides that actual
binary64 v by VR's positive ka or kt respectively. It does not subtract rounded
nodal translations/rotations. K/recover.rs:326–365 forms those two actions as
the candidate's basic axial N and torsional T; the J-end entries are q[0] and
q[1]. No later prescription replacement changes these end-action rows.

The proof is conditional on the actual ordinary unmutated Selected path:
all existing R7 gates pass, then A1 certifies before Accepted/Verified is set
(K/adaptive.rs:3937 onward, especially :4069 onward). Test-only seeded faults
that bypass a gate are not a production premise. Source14's proposed roster
is checked against the raw fixture row index/key and member mapping, and all
138 denominator lower bounds are independently rederived here. Source14's
separate global/floor proof remains subject to RV30 and is not adopted by this
packet.

The 18 models contain at most 11 nodes and 10 members, form a connected tree,
have positive E,G,A,Iy,Iz,J, and have only zero prescribed constraints. Node 0's
three translations are fixed. Every root rotation is either fixed or grounded
by a positive global-axis rotational spring. These properties are checked from
the primitive records, not inferred from labels. The adapter copies the fields
directly and creates zero-valued constraints (VR/src/cases.rs:470–531).
All omitted-spring lists are empty. No member/load is removed in the proof.

## 2. A primitive-data upper bound on exact displacements

Use physical exact intended coefficients from the binary64 primitive values,
before solver formation rounding. For member m, let d_m be floor(log2 of its
largest absolute exact coordinate difference). Its exact physical length obeys

    2^d_m ≤ L_m ≤ 2^(d_m+2).

Choose a **proof coordinate only**, ell=2^D, where

    2^D ≥ sum_m 2^(d_m+2).

This is not a new implementation scale or physical input limit. Every root-to-
node path length is at most ell. Normalize generalized displacements and loads:

    w = (translation/ell, rotation),    fbar=(ell·force, moment).

Their work pairing is unchanged. Let Kbar be the exact intended stiffness in
these coordinates after the actual zero constraints. The tree is stable:
zero energy forces every member to be a rigid continuation of its parent,
and the fully grounded root removes all six global rigid modes. Positive
element coefficients and retained root springs therefore make Kbar SPD.
Additional actual constraints cannot introduce a mechanism.

For any one free generalized coordinate i, apply a unit generalized load there.
There is a statically admissible force system on the root-to-i path, with all
other branches unloaded and the other supports taking zero reaction:

- A normalized translation load has physical force norm 1/ell. Its member
  moment norm, including either endpoint, is at most path length/ell ≤1.
- A rotation load has unit physical moment norm, no physical force, and
  member moment norm at most 1 throughout that path.
- Root moment reactions have norm at most 1. Fixed root rotations cost no
  complementary energy; a root rotational spring of stiffness k costs at
  most 1/k. Root translations are fixed.

Exact local axes are orthonormal, so the same bounds hold componentwise locally.
For each member, axial complementary energy is at most L/(EA ell^2), torsion
at most L/(GJ). A bending basic block in the specified kernel is

    D_b = (EI/L) [[4,2],[2,4]].

Its inverse has largest eigenvalue L/(2EI). Each of its two endpoint moment
components has magnitude at most 1, so q_b^T D_b^-1 q_b ≤ L/EI. This explicitly
accounts for both endpoint moments; it does not omit a factor of two. These
are precisely K/assemble.rs:323–331's axial/torsion/4c/2c intended blocks.

It follows that every diagonal of Kbar^-1 is at most

    C = sum_m [ L_upper/(EA ell^2) + L_upper/(GJ)
                + L_upper/(EIy) + L_upper/(EIz) ]
        + sum_required_root_rot_springs 1/k,

with L_upper=2^(d_m+2). Extra actual springs/constraints may be released for
this bound, not removed from the computation. The script retains the required
root spring in each free rotation and checks the necessary axis/property bits.

For completeness, the minimum-complementary-energy step is algebraic: if t*
is the exact elastic force system and t=t*+z is any equilibrated trial system,
the equilibrium difference gives A^T z=0. Therefore

    t^T D^-1 t = (t*)^T D^-1 t* + z^T D^-1 z ≥ (t*)^T D^-1 t*.

Zero-displacement support reactions do no work. Thus the trial system above
bounds unit-load compliance. SPD inverse Cauchy–Schwarz gives
`|(Kbar^-1)_ij| ≤ sqrt((Kbar^-1)_ii (Kbar^-1)_jj) ≤ C`, hence

    ||w*||_infinity ≤ C sum_j |fbar_j| = U*.

The script includes the magnitude of every separate primitive load term in
that sum; it needs no cancellation, solution, or reference value. It computes
an integer u with U*≤2^u by exact Fraction arithmetic. Values remain mathematical
rationals even where an intermediate compliance exceeds binary64 range.

## 3. Selection bounds the actual retained candidate, including excluded rows

Let v be the actual 2p verification displacement vector and c the actual p
candidate vector. Let V=max(|v_translation|/ell, |v_rotation|), over **all
components**, including any whose binary64 publication would overflow or
underflow. The constraints in these 18 fixtures are exact zero in both states.

The numerical body's operational binary64 extent L_b is bounded from its
primitive coordinate span by `2^d_b≤L_b≤2^(d_b+2)≤ell`. The last inequality is
checked for all 18 fixtures. Source14's normal-range body arithmetic premise
is independently checked at the needed exponent range here: rounded coordinate
differences, squares/sums and sqrt have representable bounding powers. None
of these bodies has zero extent.

Every verification translation-magnitude row is less than 2 ell V: its exact
three-component sum-of-squares norm is at most sqrt(3) ell V, and two retained
roundings at P≥256 keep it below 2 ell V. Raw translation maxima therefore are
≤2 ell V and raw rotation maxima ≤V. O9/input-derived exclusion can only lower
these maxima. Coupling operations rounded at P have at most a factor-two
upper bound, giving

    S_v,tr ≤ 2 ell V,
    S_v,ro ≤ max(1,4 ell/L_b) V ≤ 2^a V,
    a=max(2,2+D−d_b).

The arithmetic record verifies a≤20 (actual values are much smaller). Thus
alpha=2^-64·2^a≤2^-44<1/2. Force/moment p512 floors do not enter these T/R scales.

Crucially, R7 rule (a) iterates **every row**, not just the scale-membership
set. K/adaptive.rs:2129–2173 requires on each free T/R component i

    |c_i−v_i| + Wplus_i ≤ 2^-64 max(|v_i|,S_v,kind).

K/verify.rs:1153–1159 forms Wplus for each such component; after the other
R7 gates pass the accepted verification theorem gives
`|v_i−u*_i|≤Wplus_i`. This theorem is about retained rows and does not need a
finite binary64 publication at i. No missing Wplus is accepted as zero: successful
selection has a bound for every data block and the typed free-row construction.

Divide translations by ell and take maxima:

    V ≤ ||w*||_infinity + alpha V,
    V ≤ U*/(1−alpha),
    ||c_normalized||_infinity ≤ (1+alpha)V < 4U* ≤ 2^(u+2).

This is the missing amplitude step that a finite tag or an isolated A1 error
inequality does not supply. It applies even if a candidate component is O9-
excluded or theoretically unpublishable. It also prevents hypothetical arbitrary
MAX numerators from being assumed realized by this certified source solve.
No expected-Selected or expected-Unresolved label is used.

Upstream premises remain explicit: the accepted R7 Wplus theorem, its g/formation
counts, certified inverse bounds and theta tests, complete exact source ledger,
and actual unmutated selection order. This packet does not independently re-audit
all their floating-point implementation proofs. These fixtures' source stability
is established in §2; no-data nonsingularity is not inferred from an observed zero.

## 4. Bound the actual J-end recovered action and its binary64 quotient

The candidate code at K/assemble.rs:238–295 forms the first axis and coefficient
from the same primitive chord. Round-to-nearest is monotone and powers of two
are representable in each retained width. The largest chord component is between
2^d and 2^(d+1), so its formed length is at least 2^d and each formed axial-axis
component has magnitude at most 2. Positive input factors with exponents ea,eb
satisfy a·b<2^(ea+eb+2); rounding and division by the formed length give

    coefficient_p ≤ 2^h,   h=ea+eb−d+2.

For torsion use a=G,b=J; for axial use a=E,b=A. All successful retained arithmetic
retains these monotone bounds. No assumption that the binary64 lane coefficient
equals the retained coefficient is made.

If a candidate global component is bounded by 2^K, the local-axis dot product
in K/recover.rs:292–303 is bounded by RN(3·2·2^K)≤2^(K+3). The difference of the
two endpoint local components is bounded by 2^(K+4). The actual J-end action is
one multiplication by the retained axial/torsion coefficient, rounded once:

    |q_p,J| ≤ 2^(h+K+4),
    K = u+2          for torsion,
    K = u+2+D        for axial.

This is the actual recovery path, not a reconstructed displacement difference
in the observation caller. Bending and transverse shear do not enter these
two basic-force rows. K/recover.rs:351–365 publishes q[0]/q[1] at J.

For the lane denominator k, independently reproduce source14's bound from the
same primitive factors and binary64 member-length operation:

    k ≥ 2^ell_k,
    ell_k = ea+eb−d−2,

or ell_k=ea+eb−d when the chord is exactly a single power-of-two axis component.
Both the ordinary and prescaled paths in VR/src/floor.rs:33–92 satisfy this.
For the prescaled path, b is scaled normally to exponent −ea; its product with
a lies in [1,4], division has bounds [2^(-d-2),2^(2-d)], and unscaling reaches
the checked normal final interval without crossing a smaller intermediate.
The exact-axis case has an exact binary64 length. All required factor, chord,
prescale and final exponent inequalities are checked independently for these
138 rows; each rederived ell_k/h matches source14's descriptor. No arbitrary
small k or unproved finite denominator enters the result.

For every row here, h+K+4 is between −1022 and 1023, so its bounding power is
itself a finite binary64. Monotonic candidate publication rounding preserves
`|v_J|≤2^(h+K+4)`; an Underflow maps to zero and is finite, while an Overflow
would not reach scalar Value. Finally

    |v_J/k| ≤ 2^Q,   Q = u + (D for extension, 0 for twist)
                            + h − ell_k + 6.

All **138** Q values are ≤1020. Therefore their exact real quotient is below
the nearest-even overflow threshold `2^1024−2^970`; since 2^Q is representable,
the actual rounded division is also bounded by 2^Q and is finite. Underflow
to zero remains finite. No allowance or comparison oracle is changed.

## 5. Per-case result and scope separation

| Fixed case | Rows | Maximum Q |
|---|---:|---:|
| RF-WEAK-W-AX-rho1e-08 | 1 | 32 |
| RF-WEAK-W-AX-rho1e-12 | 2 | 50 |
| RF-WEAK-W-3D-rho1e-08 | 1 | 27 |
| RF-WEAK-W-3D-rho1e-12 | 2 | 46 |
| RF-RANGE-CHAIN-L-240 | 10 | 477 |
| RF-RANGE-CHAIN-E-1000 | 10 | 1002 |
| RF-RANGE-CHAIN-LEF-small | 10 | 97 |
| RF-RANGE-CHAIN-SIM-b | 10 | 122 |
| RF-RANGE-SKEW-L-240 | 2 | 478 |
| RF-RANGE-SKEW-E-1000 | 2 | 1001 |
| RF-RANGE-SKEW-LEF-small | 2 | 98 |
| RF-RANGE-SKEW-SIM-b | 2 | 121 |
| RF-RANGE-CONT-L-240 | 20 | 493 |
| RF-RANGE-CONT-E-1000 | 20 | 1020 |
| RF-RANGE-CONT-LEF-small | 20 | 113 |
| RF-RANGE-CONT-SIM-b | 20 | 140 |
| RF-RANGE-THIN-A | 2 | 5 |
| RF-RANGE-THIN-B | 2 | 39 |

For the worst case, the proof scale is ell=2^7, U*≤2^1003, a=5,
and extension has h−ell_k=4. Thus Q=1003+7+4+6=1020. The slack is modest but
explicit; no decimal rounding hides it. Exact source terms and intermediate
ceil exponents are retained so an independent reviewer can tighten or refute
any step.

The **actual vk_scale RF-LARGE24** route remains distinct: source14 claims its
twist/extension coefficients satisfy k≥1, making arbitrary finite numerator
division finite without this additional proof. This packet does not turn the
18-case derivation into a new scale run or mix the two rosters. It also does
not approve source14's remaining floor/global-memory equations.

If independently accepted, this closes only the specified remaining scalar
observation-finiteness premise on these hash-bound fixtures and this source.
Changed models, arithmetic, recovery, row mapping or selected gate need a new
assessment. Remaining K6c allocation/serialization/live-overlap, source14/RV30,
final H/VR estimates, admission replay and measurements remain with their
owners. No panic/library/error-path programme or new nonfinite handling is
necessary to state this bounded consequence, and none was investigated.
