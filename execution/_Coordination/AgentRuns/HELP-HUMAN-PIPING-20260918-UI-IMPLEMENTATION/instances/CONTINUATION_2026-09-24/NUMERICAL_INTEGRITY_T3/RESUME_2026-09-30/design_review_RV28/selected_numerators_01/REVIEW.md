# RV28 — selected-numerator independent mathematical review

**VERIFIED for the named 138 rows in 18 fixed cases, conditional on ordinary
unmutated Selected at source 40129 and the accepted R7 verification theorem.**
The proposed numerator consequence is sound. No blocking or SHOULD-FIX
mathematical finding remains in this bounded derivation.

The independently recomputed largest rounded quotient bound is **2^1020**.
That exponent was obtained from primitive bits and the complete bound chain,
not taken as a target. It is below binary64's nearest-even overflow threshold.
This closes only the previously missing selected-numerator finiteness premise
for this hash-bound roster. It is not complete E_max, source14/estimator
implementation acceptance, source/build reconciliation, admission, measurement,
availability, or product qualification.

TASK `/root/rv28_a1_design`, direct return to ROOT `/root`.
Native mechanism: `collaboration.followup_task`; no descendants.
Actual start: **2026-10-01 17:27:21 UTC**; allotted deadline: 17:52:21 UTC.
Active role/instruction/skill origins and hashes are recorded in BASIS.json.
No new workflow, role or skill was selected beyond software-code-review.

Aliases: P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
K=FK/src/structural/retained;
VR=P/validation/benchmarks/numerical_robustness; R=this resumed run.

## Reviewed identity and source boundary

- Review brief: NUM at `5293da04d3987e1fa9dd9b6d54ff156a1bf0c39c`,
  R/BRIEFS/RV28_K6C_SELECTED_NUMERATORS.md.
- Proposal: R/metric_design_05_selected_numerators/DERIVATION.md,
  SHA256 `a54c0e88f09c59a6a728a8067805b6894e574ddd4ba69379f939059ceb48b069`.
- Proposal seal:
  `8de7420579e705b7930c8e22cb0322616f718e0379dcd907937f2b077b91982a`.
  Every sealed payload was verified, but the designer's derive.py was neither
  read as an oracle nor executed.
- Immutable mechanics/caller source:
  `40129a225d73860ac2a53da9a2fa73869df668f3`.
  All inspected source was read from that Git object.
- Inputs: rf_weak.jsonl SHA256
  `58228206b15c7dd571ad716940d1055ac20542e5dd1e7f03851aaf1d59a07ff1`;
  rf_range.jsonl SHA256
  `ae74938cdfc12a398f51728fee5304cf68e70acedd5df1b433e1f353db8a5a88`,
  both read from the same revision.
- Source14's final FIXED_INEQUALITIES_ALL.json and RV30
  source_review_RV30/finite_callers_12 were read and their seals checked.
  Their reviewed roster/coefficient proof is relevant prior evidence; the new
  amplitude claim was derived independently.

Current K6C adaptive.rs differs from 40129 because the A1 change has not yet
been integrated into that checkout. ROOT clarified that this is the older
committed KF3 basis, not an unexpected edit. Independent read-only queries
confirmed K6C HEAD `66c186743b4db005e942f241d7e72a30ae0365e5` and both its HEAD
adaptive.rs blob and current file blob
`5448ca262ab0346c138052e39b9a205b82739842`.
Nothing in this return certifies that older source as equivalent to 40129.
Final candidate/source/build reconciliation remains required.

## Independent primitive and operator binding

The checker decodes the stored binary64 bits with integer arithmetic. It does
not use reference values, scales, previous successful runs, expected-refusal
flags, NotCovered labels, or solved outputs. All 18 models:

- Have a connected acyclic member graph, at most 11 nodes and 10 members.
- Have positive E,G,A,Iy,Iz,J and nonzero chords, with nonparallel y_reference
  vectors, establishing a valid exact orthonormal local frame.
- Fix node 0's three translations. Each unfixed root rotation has an actual
  positive global-axis rotational spring. Extra springs/constraints may be
  released for the compliance bound.
- Have no omitted springs; constraints are the zero-valued constraints actually
  made by cases.rs:513–517. The fixture adapter copies primitive coordinates,
  section data, loads and source ids without unit or load transformations
  (cases.rs:163–219 and 470–531). Fixture unit metadata is m.
- Include the absolute magnitude of **every separate primitive load term** in
  the load bound; no cancellation or coalescing is assumed.

K/assemble.rs:238–297 forms the retained frame and coefficients from those
same primitive fields. Its exact intended axial/torsional terms are EA/L and
GJ/L; its two bending blocks are (EI/L)[[4,2],[2,4]]
(:298–329). Exact axes preserve Euclidean force/moment norms. The compliance
argument is about that exact intended operator K*, not the rounded assembled
K_p. The accepted R7 theorem supplies the separate connection to computed
verification and candidate states.

## Re-derived compliance inequality

Let d_m be floor(log2(max absolute exact chord component)). Since the largest
component lies in [2^d_m,2^(d_m+1)), the true length obeys
2^d_m<=L_m<=2^(d_m+2). Choose ell=2^D, with D the ceiling base-two logarithm of
the sum of these upper lengths. This is only a proof coordinate; it changes
neither inputs nor production scales.

Let P be diagonal with ell on translation coordinates and 1 on rotation
coordinates. Then u=P w, Kbar=P K* P, and fbar=P f. In particular a unit load
in a normalized translation coordinate corresponds to physical force 1/ell,
while a unit normalized rotation load is physical moment 1. This verifies the
work-pairing normalization rather than assuming equal dimensions.

For a unit generalized load at any free node component, transmit its wrench
along the unique root path. On that path, force norm is at most 1/ell and each
endpoint moment norm is at most 1, because distance to the loaded node is at
most the path length, which is at most ell. The root moment also has norm at
most 1. Other branches can carry zero wrench; reactions at additional fixed
supports and forces in additional springs can be zero in the statically
admissible trial. This is an equilibrium trial, not a compatibility claim.

Axial trial compliance contribution is at most L/(EA ell^2), torsional at
most L/(GJ). For each bending plane,

    D_b^-1 = L/(EI) [[1/3,-1/6],[-1/6,1/3]],
    q^T D_b^-1 q = L/(3EI)(q_i^2-q_i*q_j+q_j^2) <= L/EI

when |q_i|,|q_j|<=1. The maximum occurs at opposite endpoint signs and equals
L/EI; therefore the proposal has not lost a factor of two. The checker verifies
the corner maximum, while convexity makes that bound apply throughout the box.
Each required free root rotational spring contributes at most 1/k.

Summing upper lengths over all edges, even those not on a particular path,
therefore gives C at least every diagonal of Kbar^-1. Positivity of every
basic block makes zero member energy imply rigid continuation across every
edge. The grounded root removes all rigid modes, so the released tree is SPD.
Additional actual constraints and positive springs preserve stability and
cannot increase these unit-load compliances. This also supplies the
no-data nonsingularity premise; it is not inferred from computed zeros.

For completeness, minimum complementary energy follows from t=t*+z with
A^T z=0 and t*=D A w*: the cross term z^T D^-1 t*=z^T A w*=0.
Thus a trial force's complementary quadratic form exceeds the exact elastic
one. Inverse-SPD Cauchy–Schwarz then gives every
|(Kbar^-1)_ij|<=sqrt(C*C)=C. Hence

    ||w*||_infinity <= C * sum_j |fbar_j| = U* <= 2^u.

The exact rational C, every normalized primitive load magnitude, their sum,
their product, and u were independently computed for all 18 cases and match
the proposal. Large intermediate rational values are not evaluated in binary64.
For example, the THIN-B compliance bound can exceed binary64 while the
resulting truth/product bound is small; that does not break a mathematical bound
or imply a computed selection.

## Full retained T/R implication, including publication-excluded components

Let V=max_i(|v_tr,i|/ell, |v_ro,i|) over **all** verification components, with
prescribed components exactly zero. Recover.rs:270–275 inserts these components
directly and forms each displacement magnitude as a rounded sum of squares
followed by rounded sqrt (:220–234). For P>=256,

    magnitude <= sqrt(3)*(1+2^-P)^(3/2)*ell*V < 2*ell*V.

The squared inequality 3*(1+2^-256)^3<4 was checked exactly.
O9 and input-derived exclusion only lower raw maxima. Each rounded coupling
has a factor-two upper bound; body extent satisfies
2^d_b<=L_b<=2^(d_b+2)<=ell in all cases. Therefore

    S_v,tr <= 2*ell*V,
    S_v,ro <= max(1,4*ell/L_b)*V <= 2^a*V,
    a=max(2,2+D-d_b).

Independently calculated a is 4–6, stronger than the proposed a<=20.
Thus alpha=2^(a-64)<1/2.

Source 40129 adaptive.rs:2084–2093 uses candidate range outcomes only to build
the scale-membership mask. The rule(a) loop at :2137–2185 visits every layout
row, including those masked out of scales. For every free T/R component it
tests the exact inequality

    |c_i-v_i| + Wplus_i <= 2^-64 max(|v_i|,S_v,kind).

verify.rs:1141–1159 supplies Wplus for each free component whenever no data block
lacks B. The later uc/theta/g gates (:2220–2235) must pass for ordinary selection;
missing data-block B cannot support a selected result. These inherited R7
prerequisites, its exact-source residual/formation assumptions, and the
unmutated path are explicit premises.

R7's free-component theorem bounds |v_i-u*_i|<=Wplus_i independently of binary64
publication. It is not limited to the subset carrying finite published values.
After translation normalization, Wplus_i<=alpha*V, yielding

    V <= U* + alpha*V,
    V <= U*/(1-alpha),
    ||c_normalized||_infinity <= (1+alpha)*V < 4*U* <= 2^(u+2).

The checker verifies the contraction inequality for every case. The combined
|c-v|+Wplus inequality could give a tighter candidate bound; the stated
4*U* is conservative and sufficient. Force/moment p512 floors do not enter
the T/R argument. A1's final publication gate is additional: it does not
replace rule(a), and this proof does not try to control excluded components
through an A1 finite-value certificate.

The inspected schedule :4027–4098 validates the pair, calls all R7 comparisons,
then certifies publication before Accepted/Verified. No seeded bypass is a
premise. No expected-selected outcome is assumed: the claim is conditional on
the actual ordinary Selected branch.

## Actual J-end recovery and rounded quotient

cases.rs:463–464 resolves each tw/ext key by member name.
lane.rs:114–115 uses J-end Rx for twist and J-end Ux for extension.
lane.rs:133–143 obtains kt/ka from the same model member, and :162–169 divides
the actual published action by that lane coefficient. It does not divide
a separately reconstructed nodal displacement difference.

For a retained member chord, monotone correct rounding keeps its largest
formed component at least 2^d and all components at most 2^(d+1).
The formed length is at least 2^d, so each formed axial-axis component is at
most 2 in magnitude. With positive primitive exponents ea,eb, monotonic
product/division rounding gives retained coefficient <=2^h,
h=ea+eb-d+2 (assemble.rs:241–248, 280–295).

For candidate global component cap 2^K, recover.rs:282–300's actual dot
products and endpoint difference satisfy

    |local endpoint component| <= RN(6*2^K) <= 2^(K+3),
    |local endpoint difference| <= 2^(K+4).

The actual basic q[0]/q[1] multiplication, rounded once at p
(:320–336), is at most 2^(h+K+4); :352–364 puts those values directly at J.
Here K=u+2 for torsion and K=u+2+D for axial. Bending/shear and later
prescription replacement do not enter these two action entries.

The lane's coefficient is a separate binary64 construction. Each of its
lower exponents was recomputed from the primitive factors and exact chord.
floor.rs:34–37 returns the ordinary product/division only when both are normal;
otherwise :84–87 prescales b by 2^(-(ea+eb)). Its normalized product lies
in [1,4], and the length powers bound the division. Source14's exact-axis
refinement is valid when the sole nonzero chord is exactly a power of two:

    ell_k=ea+eb-d-2 generally, or ea+eb-d for an exact power-of-two axis,
    k_lane >= 2^ell_k > 0.

The independent checks cover positive normal factor exponents, normal
prescale endpoints, normalized product bounds, normal quotient bounding
powers, and normal final coefficient bounds. scale2's steps are monotone
between those normal endpoints; no tiny intermediate can silently cross zero.
All 138 lower/upper descriptor values match source14 and the proposal.

For these 138 rows the computed numerator-cap exponent N=h+K+4 ranges
**-984 to 46**. Every 2^N is finite representable binary64, so monotone
one-round candidate publication preserves the cap. lane::value_of maps
Underflow to signed zero and makes Overflow unavailable (:80–87); in addition,
this cap rules out numerator overflow on the stated selected path.

The mathematical quotient is at most 2^Q, Q=N-ell_k, with every Q<=1020.
Because the upper power is itself representable, nearest-even final division
also stays <=2^Q. Specifically,

    2^1020 < 2^1024 - 2^970,

the nearest-even overflow threshold. Underflow to zero remains finite.
The actual final quotient, not an intermediate reference scale, is what this
inequality controls.

## Every-row result

INDEPENDENT_BOUNDS.json records row index, key, actual member id, J-end
component, retained-component exponent, retained coefficient upper, lane
coefficient lower, numerator cap and quotient cap for every row. The raw
fixture roster was rederived independently by tw/ext key and lower
coefficient exponent, then compared with both source14 and the proposed list:
no missing, duplicate or extra open row.

| Case | Rows | Maximum Q |
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

The worst row family is CONT-E-1000 extension: u=1003, D=7,
h-ell_k=4, Q=1003+7+4+6=1020. Independent ranges are D=-237..127,
u=-13..1003, a=4..6, denominator lower=-1021..-6. The check count is
**2,876 passing assertions**, including every descriptor/row binding and
mathematical exponent condition. This is arithmetic verification evidence,
not a count of solver experiments.

## Findings, residual limits and disposition

**No blocking or SHOULD-FIX defect found.** No changed guarantee, arbitrary
cutoff, new safe margin, oracle change or new numerical refusal policy is
needed for the reviewed consequence.

Remaining limits:

1. R7's previously accepted verification-error theorem and its implementation
   premises remain inherited; this review checks their correct use, source
   applicability and all-component scope, not every upstream floating-point
   count or factor-bound theorem from scratch.
2. The consequence is pinned to this exact fixture roster and unmutated 40129
   source. It neither promises selection nor bounds arbitrary user models or
   mutated tests. Changed model/source/row-mapping/gates require reassessment.
3. The separately reviewed actual24 RF-LARGE vk_scale route is not mixed into
   this 18-case proof. Its k>=1 reasoning remains separate.
4. Finite final observations remove only this specific successful from_f64
   finiteness gap. Full allocation/serialization/caller-union/error-path
   accounting, final A1 integration/build identity, E_max/admission and
   measurements remain with their owners.
5. No Rust, source algorithm emulation, stiffness matrix, generated model,
   solve, probe, runtime experiment or native application was executed.
   No maintained code, oracle, contract, tool or Git/index state was changed.

ROOT may use this review to dispose the bounded proposed source consequence.
This review grants no broader acceptance.

