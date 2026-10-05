# RV57 — independent source-residual derivation

Conditional mathematics only. This derivation was written after reading the I43
proposal; it is an independent reconstruction and check, not a blind-before-exposure
stage. Active TASK `/root/rv57_source_residual_design`, parent ROOT `/root`, via
`delegated-harness-native`; no descendants. Sources and inspected origins are in
`_run_records/ORIGINS.json`. Source code is immutable 4afbac6e613203f93a499b780082b28be37f6c2c.

## Source operator and inverse transfer

Let F/C be the actual free/constrained split. K is the exact intended operator on
admitted stored E/G/A/I/J, exact normalized coordinates and exact frame construction.
It is not the formed verification matrix. G uses the same coordinates, frame,
connectivity, constraints, prescribed values, individual load ledger and positive
spring laws, with the source constitutive coefficients in place of K's coefficients.
The material variant must identify whether its source law is exact E/nu, ordinary
independent E/G, or the reviewed interpolation/hull. No E/nu substitution follows
from the captured ordinary E/G specimen.

At member level, the inspected equations are B^T D B, with D's positive axial,
torsion and two [[4,2],[2,4]] bending blocks. Changing positive coefficients under
the same B preserves the energy nullspace. A proved K uniqueness warrant therefore
transfers to the same source problem; floating factor success alone is insufficient.
The common potential pattern must contain every G/K coupling, including entries
that numerically cancel, and each selected free block must be closed under it.

Write S for the matching verification factor's positive diagonal radix scaling in
free-position order. The accepted I33/RV45 transfer gives

    ||(S K_FF S)^-1||inf <= beta = 2 B.

Symmetry permits the accepted one-norm bound to be used in the infinity norm.
B is the same retained upward body bound for the checked data block, not a caller
estimate, another precision's bound, or an unscaled quantity. Let E majorize
|G-K| and eta majorize ||S (G-K)_FF S||inf. For alpha >= beta eta with alpha < 1,

    A_G = S G_FF S = A_K + S (G-K)_FF S,
    ||A_G^-1||inf <= beta / (1-alpha).

This follows from the convergent Neumann inverse of
I + A_K^-1 S(G-K)_FF S. Keeping I42's conservative alpha is valid; a smaller
source-residual numerator does not require changing the perturbation proof.

## Arbitrary finite center and directed residual

Choose any finite representable y_F and set y_C to the exact actual prescriptions.
The choice may come from one retained-factor application, an inaccurate solve, or
no solve; its accuracy is not a premise. Define the exact residual

    r = S (f_F - G_F,all y_all).

All individual ledger terms, incident member contributions, prescribed columns
and springs enter their signed row accumulations. For outward interval rho
containing r, let omega >= max_i max(|rho_i.lo|,|rho_i.hi|). If

    d = RD(1-alpha) > 0,
    epsilon = RU(RU(beta omega) / d),

then u_G-y has constrained part zero and

    A_G S^-1 (u_G-y)_F = r,
    |u_G,i-y_i| <= s_i epsilon.

Thus [RD(y_i-s_i epsilon), RU(y_i+s_i epsilon)] encloses each free component.
This proof includes the actual midpoint, P conversion, factor approximation,
triangular-solve rounding, S shift and RN1024 center addition, because it proves
from the resulting actual point y. It does not need a forward-error promise for
those operations. A failed conversion/arithmetic/accounting extraction supplies
no center. A K-only residual would omit (G-K) times the correction and is unsound.

The proposed correction scaling is correct: solve_scaled accepts scaled RHS in
free-position order and returns the corresponding scaled unknown, also in that
order (factor.rs:639–668). Therefore a RHS chosen from S(f-Gx) produces a trial
z0 and the physical correction is S z0. Calling the unscaled solve wrapper would
scale twice. The existing WideContext::round can implement a single explicit
nearest-even P cast and records its actual Round work (wide/multi.rs:1219–1225).
The midpoint itself need not enclose the residual; it is merely a chosen point.

## Source recovery and exact-zero scope

For a covered source row q=a_G u_G+b_G, directed evaluation over the proved
component box is sound. Equivalently, evaluate the signed center functional
and add epsilon_c sum_{i in c}|a_Gi|s_i over checked blocks. Source coefficients
and exact prescriptions remain in the center functional; no separate q_G-q_K
triangle decomposition is needed. This changes no published value, radius,
class, scale, predicate or original K radius gate. Epsilon is a source-solution
residual radius, not the old NativeSourceError.source_error.

The actual source operators inspected establish local ends H^T D_G B, global
reactions G_C,all u-f_C, station moments t Q_j+(t-1)Q_i, and spring actions -k u.
Support membership and signs select which reactions and spring actions are summed.
A Euclidean source magnitude is enclosed by component range squares, a directed
sum and directed square-root endpoints. These are private source values, separate
from the unchanged emitted hypot and other observable relations.

No-data means each individual free ledger term is zero, no potential prescribed
adjacency is nonzero, and the matching verification free state is zero. Together
with the common source uniqueness warrant it proves zero free source motion.
It does not prove zero constrained prescriptions, member action, or constrained
load offset. A fully fixed member requires no free inverse bound yet can have
nonzero prescribed-strain action and reaction -f_C even when all motions are zero.
Mixed no-data blocks can pin only their proved zero free center/radius. A cancelling
nonzero ledger, missing B, or observed zero is not this proof.

## General exact frame and arithmetic limits

The inspected source construction normalizes the chord, projects y_reference off
ex, normalizes that result to ey, and normalizes ex cross ey to ez
(assemble.rs:240–279). Signed interval evaluation of these same formulas encloses
the exact source axes; all three norm lower bounds must be strictly positive.
Near-parallel interval refusal at fixed p1024 is conservative insufficiency, not
permission to use the rounded formation frame as exact.

The B entries at assemble.rs:298–329 and recovery signs at recover.rs:295–392 match
the proposal. Four-corner multiplication and positive-denominator four-corner
division are valid for either numerator sign. directed.rs:105–170 implements
signed multiplication and the division side test q*b-a for b>0. Range squaring
uses lower zero on a sign crossing. No dependency cancellation may turn an inward
endpoint into evidence; natural interval widening is permitted to refuse.

The independent count gives 9 interval subtractions, 18 products, 8 adds,
9 range squares, 3 square roots and 10 divisions for the stated frame schedule:
DA16, DS18, DM162, DD80, DQ6. This is a scalar-call upper, not spent LME. The
same-owner solve has exactly 2 ell Mul, 2 ell Sub and N Div on a full successful
sweep. The counts neither cover casts/visits nor qualify allocation behavior.
Every actual producing context/sum prefix and simultaneous accounting loss must
survive failure. Fixed-format exponent/span refusal remains permitted.

## Independent finite controls

`_run_records/independent_controls.py` imports no author or production evaluator.
It decodes binary64 using integer fields; brackets pi by 4(atan(1/2)+atan(1/3))
with 600 alternating terms; builds the source annulus from differences of powers;
and checks source residual as an affine form in the one shared pi. It independently
reconstructs the closed P256 trial correction, checks the exact scaled K inverse,
actual source perturbation, claimed residual endpoints/epsilon, and the source
functional ranges of all 52 corrected intervals. All 52 original K row radii and
29 relative five-operation binary64 ceilings are independently checked.

The captured constructed-center predicate count is 52/52. The independent sign-only
and correctly recomputed wrong-zero-center counts are each 7/52. Original zero
rows remain 52 exact zeros under the existing proof. Forty separate rational SPD
2x2 controls use unrelated arbitrary centers, nonzero prescriptions and unequal
radix scalings; all obey the residual/action bound. Explicit controls distinguish
K-only residual omission, prescribed-column omission, fully fixed constrained loads,
cancelling individual ledger terms and I33's unavoidable unchanged-output violation.

These controls support the general derivation. They do not execute the retained
factor or show that the actual proposed p1024 implementation produces the same
endpoints, work prefixes, refusals or availability as exact Fraction arithmetic.
