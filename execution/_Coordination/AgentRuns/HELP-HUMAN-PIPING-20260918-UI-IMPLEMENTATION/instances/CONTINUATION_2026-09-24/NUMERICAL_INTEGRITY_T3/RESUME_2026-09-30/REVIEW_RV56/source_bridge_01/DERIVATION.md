# RV56 independent mathematical/source review

Frozen subject: 4afbac6e613203f93a499b780082b28be37f6c2c against
57636e98704117d65e6b7ff085a3fb641f79f168. This review read the accepted I33/RV45
basis and independently re-derived the statements below. It does not claim a
blind derivation. No I43 conclusions were received or used.

## Sufficient fixed-anchor uniqueness

For a straight member in its exact orthonormal frame, its six basic strains are
axial displacement difference, twist difference, two z-plane end rotations minus
the transverse chord slope, and two y-plane end rotations plus the other slope.
The axial/torsion energy coefficients are positive; each bending 2x2 matrix
[[4,2],[2,4]] has eigenvalues 2 and 6 times its positive EI/L coefficient.
Therefore the element energy vanishes precisely when all six basic strains vanish.
Those conditions require equal end rotations and a translation difference equal
to the shared rotation crossed with the exact chord. This is precisely an
infinitesimal rigid displacement restricted to its two endpoints.

Consider the difference of two solutions; its prescribed motions are zero even
when the original prescriptions are nonzero. If one node of a member has all
six difference motions fixed to zero, zero energy forces the other node's six
motions to zero. Induction along the connected member graph fixes every node.
Thus a connected member body with one fully restrained node has a positive
definite free stiffness. Added positive global springs cannot weaken it. Changing
strictly positive member coefficients preserves the same nullspace, so this
warrant holds for both K and every source law in the positive coefficient box.

The actual source constructor computes bodies as connected components of the
member graph (source.rs:660–690); nodes share all six DOFs, with no member releases
in the admitted straight-member type. source.rs enforces positive finite E/G/A/
Iy/Iz/J, distinct nodes, nonzero chord and nonparallel reference. The checked
builder requires positive source coefficients. bridge.rs:anchored checks an actual
same-body node with all six actual constraints; it assumes no caller flag. A
body with an isolated node has this warrant only if all six motions are fixed.
It then has no free block. Consequently this is a sound sufficient recognizer;
its failure does not assert singularity or domain exclusion. The existing spring
fixture truthfully selects natively and then returns MissingUniquenessWarrant.

The no-data predicate is the exact existing bound.rs predicate: no individual
nonzero ledger term on any free row, no nonzero verification-state entry, and no
potential adjacency to a nonzero prescription. ledger.nonzero_term_spent searches
the preserved individual-term flag, not the net value. Cancelling +1/-1 loads
therefore remain data. Full member potential patterns are checked before this
predicate is trusted. With uniqueness and common zero forcing on a closed block,
its exact K and source solutions are zero. Missing B by itself proves none of this.

## Perturbation and contractions

In actual free ordering let S=diag(2^scale[a]) from the successful matching P=2p
factor. factor.rs builds scale in free-position order, using RCM rank only to
place the factored matrix. adaptive.rs retains the successful matching shared/
verification slots, selected and verification states, and upward body B summaries.
The private view ties these to its immutable owner, validates precision/policy,
source/ledger bytes, layouts, full free ordering, block/body maps, and row/radius
metadata. It checks that every potential member pair lies in the pattern and
that no free pattern edge crosses its claimed block. Public rcond is not used.
From the accepted R7 transfer theorem, beta=2*B_body bounds the infinity norm of
(S K_FF S)^-1 on every data block. Symmetry transfers its one-norm bound. The
body maximum is conservative for any associated block, not a bound on an absent
body or a different scale/precision.

Let ell be a positive lower bound on max(abs(exact chord components)). The code
forms each chord subtraction in both directed senses and chooses a lower bound
on its absolute value; this can conservatively refuse if the bound is zero.
Since ell<=L and each exact frame entry has magnitude<=1, Bbar's actual pattern
has 48 entries: axial translations 6, twist rotations 6, and four bending rows
with six translations/three appropriate end rotations each. Its coefficients
are 1 or upward 1/ell. Hbar has 16 entries in the local signed-basic pattern;
absolute signs are appropriate for error majorants. The constitutive map has
ten entries: two diagonal axial/twist entries and two 4/2 bending blocks.

The source coefficient endpoints enclose EA, GJ, EI_z, EI_y; the exact K products
are lifted separately, preserving up to 106 bits. dC is the max of the two
upward distances. Dividing C_hi and dC by ell and retaining the 4/2 pattern gives
DGbar>=abs(DG), DDbar>=abs(DG-DK). Hence
E_e=Bbar^T DDbar Bbar>=abs(G_e-K_e). The full global potential pattern contains
all its couplings, including numerical zeros in the actual K assembly.

For each free row, F2 bounds eta_i=s_i sum_free_j E_ij s_j and
v_i=s_i sum_all_j E_ij M_j. M is upward abs(x)+R on free DOFs, exact abs(u_C)
on prescribed DOFs. Every constrained column remains in v. Block maxima give
alpha=beta*eta. Code requires alpha<1 and separately requires downward
1-alpha>0, then computes tau upward as beta*v/(1-alpha). This follows from
G_FF e_F=-Delta_F,all uK and the Neumann inverse bound in S coordinates.
T_i=s_i*tau bounds the response change; T_C=0. No retry or solve is added.

F3 retains DGbar*(Bbar*T)+DDbar*(Bbar*M). Summing these six nonnegative terms
upward before a shared transpose is valid by distributivity and monotonicity.
Hbar^T covers local end actions; Bbar^T scatters full global member reaction
changes. Identical spring contributions to reaction change vanish on constrained
rows because e_C=0. Identical nodal ledger terms cancel in the reaction offset.
Spring rows use abs(k)*T for the native signed -k*u law. Exact station fractions
are in [0,1]; N/V/T map from J end, moments use the absolute coefficients of
 t*Qj+(t-1)*Qi. Magnitudes use their actual native certified magnitude row plus
the sum of mapped component errors, by the reverse triangle inequality. No new
hypot of published components or source-cut convention is substituted.

Every returned source row encloses x +/- (R_K+source_error), outward. Missing
verified radii refuse; an InputDerived displacement instead checks x against
the exact prescribed source value and returns that singleton. Positive scalar
proposals still do not establish actual PP material/geometry/case/final-row
custody or any product admission.

## Work, count and lifetime trace

All directed scalar paths use NumericWork's fresh context and sum and collect
both on failure. Checked work status gates successful extraction; an original
numeric refusal remains separate from a simultaneous accounting fault. The view's
binary64 ceiling-validation failure prefix has one confirmed exception, RV56-F1.
Auxiliary lookups, lifts, shifts, comparisons, allocation and initialization are
explicitly unqualified; this review does not upgrade them into complete visit
or tariff evidence.

F2 performs 2*(48+48)=192 B products and 20 D products per member. F3 performs
2*48+48=144 B products, 20 D products and 16 H products; the two action terms
share their transpose. Total 336/40/16 matches the observed counters. There are
three member coefficient builds. Source-size layout arithmetic uses checked
sums/products and allocator layouts; source constructor bounds nodes/DOFs and
related populations before its private maps are built.

The view owns data[blocks] and temporarily allocates prescribed[n]. The bridge
owns two n endpoint arrays, alpha/tau[blocks] and errors[q]. It drops v after
F2, reuses eta for reactions, then drops eta before rows construction. Output
rows hold three endpoints each while the consumed errors allocation overlaps;
peak endpoint capacity is max(2n+2b+q,4q+2b), subject to actual Vec capacities.
For the primary witness this is 210. This excludes flag arrays, fixed scratch,
caller/owner overlap and allocator/compiler costs as the implementation says.
The output keeps immutable owner and proposed-law borrows, not a copied solve,
publication, coefficient cache, report or free-standing radius authority.
