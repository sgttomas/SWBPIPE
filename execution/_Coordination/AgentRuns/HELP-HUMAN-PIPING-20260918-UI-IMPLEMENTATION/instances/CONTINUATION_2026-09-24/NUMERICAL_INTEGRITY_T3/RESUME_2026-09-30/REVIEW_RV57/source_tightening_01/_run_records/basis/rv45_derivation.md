# RV45 stage 0 — independent sufficient source/action bridge

Frozen before reading I33's proposal, script or arithmetic. Receipt/start:
2026-10-02 20:56:02 UTC; cutoff 21:46:02; hard stop 21:56:02. Fresh native TASK
under ROOT HELP_HUMAN; no delegation. Read-only source basis main 49034a940f,
D1/R7 and corrected A1/addendum; B1C at a4e21279b9 and RV42 at 7a45f14b46.
Exact origins/hashes and preserved inspected bytes are in _run_records.
This is a conditional sufficient theorem, not an implementation or availability
claim. No I33 material was read to derive it.

## Source identity and two problems

K_K is the exact intended W1a operator on the admitted binary64 E,G,A,Iy,Iz,J
primitives and exact coordinate/frame/length construction; u_K is its exact
solution. It is not K_P, its finite-precision assembled verification matrix.
K_G uses source-geometric annulus properties, real pi, and exact E/[2(1+nu)]
where the selected exact material contract supplies E/nu. u_G is the intended
source-profile response. Loads, exact prescriptions, coordinates, y reference,
DOF partition, station fractions, member connectivity and support identities
must be proven the same. Different load/frame/source contracts require their own
extra perturbation terms or refusal, never a silent equality assumption.

B1C derives positive rational enclosures from exact normalized D and the actual
normalized effective-wall bit t, not an unevaluated nominal-minus-tolerance
expression. It encloses A_G,I_G,J_G,Z_G and c_G=D/2 using a fixed pi bracket.
Maintain E_G=the selected E bit and exact G_G=E_G/[2(1+nu)] with positive denominator;
compare against actual E_K,G_K/property bits. Endpoint products yield enclosures
for each of alpha=EA, tau=GJ, beta_z=EI_z, beta_y=EI_y. For each coefficient c,
set d_c=max(|c_lower-c_K|,|c_upper-c_K|), and C_c=max(|c_lower|,|c_upper|).
This does not substitute endpoints for any actual producer, scale or result bit.

Source references inspected: PP pressure_section_geometry.rs:287–343,459–469 and
SOURCE_ODWALL_EXPECTATIONS.json:6–8,407–408 preserve displacement/rotation and
stress source truth even without pressure regions. PP lib.rs:1024–1035 and
6508–6524,9436–9486 and pressure_exact.rs:236–264 identify actual operand formation.
Ordinary-route source truth is a separate unresolved warrant; no universal
ordinary-route geometric or stored-bit definition is selected here.

## Finite matrix and response majorants

For each common exact member length L and exact basic matrix B, obtain an exact
rational rho>=1/L. A simple conservative finite construction is
rho=1/max_a|x_j,a-x_i,a| using exact coordinate differences; the denominator is
positive by the admitted nonzero chord. Every exact normalized axis component
has magnitude<=1. Define nonnegative Bbar by replacing each axis component by 1,
each bending translation entry by rho, preserving structural zeros. Thus
|B|<=Bbar. No finite-precision frame or inv_length is treated as exact.

Define Ddelta=rho*(diag(d_alpha,d_tau) plus the two d_beta*[[4,2],[2,4]]
blocks), and DGbar similarly with C coefficients. Then

    |K_G,e-K_K,e| <= M_e = Bbar^T Ddelta Bbar,
    |K_G,e| <= KGbar_e = Bbar^T DGbar Bbar.

Assemble M by summing nonnegative contributions over all actual columns,
including prescribed columns. Identical springs contribute zero to M but their
absolute operators belong to KGbar. Directional springs require identical
source direction/stiffness and their exact rational operator; F2a's actual
support admission is narrower than the kernel-only surface. This is sparse
streamable mathematics, not a proposed dense implementation.

R7 provides B_c>=||(S K_P S)_c^-1||_1, not directly a bound on K_K. Under its
actual tested formation/theta/g premises, Lemma C provides beta_c=2 B_c for
||(S K_K S)_c^-1||_1 and, by symmetry, its infinity norm. A tighter
B_c/(1-theta_c) is justified only if theta is the same certified transfer bound.
All S entries are the actual positive radix powers, with global/free ordering
mapped correctly. Hager–Higham est is never a substitute.

On a union of blocks closed under both exact operators, with every required
block carrying an available certified beta, let beta=max beta_c,
mu=||S M_FF S||_infinity and eta=beta*mu. If eta<1, Neumann gives

    ||(S K_G,FF S)^-1||_infinity <= beta/(1-eta) = beta_G.

Both operators must have a unique intended solution. Positivity of all member
constitutive factors and identical springs preserves the energy nullspace;
a valid K_K positive-definiteness witness therefore also establishes K_G
positive-definiteness. Merely observing a zero solution does not do so. R7's
no-data omission remains conditional on exact zero RHS for both operators,
no source cross-block coupling and uniqueness. Missing B on a formerly empty
block activated by changed source data/coupling requires coverage or refusal.

Given proved component envelopes U_j>=|u_K,j| (from final component certificates,
or retained verification state plus R7 W-plus), use exact prescriptions on C.
Since (u_G-u_K)_C=0 and

    K_G,FF (u_G-u_K)_F = -(K_G-K_K)_F,all u_K,

set h=max_i s_i sum_j M_ij U_j over free rows and ALL columns. Then

    |u_G,i-u_K,i| <= s_i beta_G h = d_i   (i free),
    d_i=0                                           (i prescribed).

All bounds are exact or directed outward, with eta<1 decided conservatively.
A limit, missing association or absent operand is a named refusal, never zero.

## Basic actions, reactions and magnitudes

For each member define ebar=Bbar U and edelta=Bbar d. Because Q=D B u,

    |Q_G-Q_K| <= DGbar edelta + Ddelta ebar.

This independently charges constitutive changes even if d=0, and includes
prescribed-motion columns. From the inspected recover.rs:295–391 map the six
basic-action radii to end actions: axial and twist copy one radius; bending
end moments copy their corresponding radius; local shear is bounded by rho
times the sum of the two bending radii. Station moment radius is
|t| radius(Q_j)+|t-1| radius(Q_i), valid for the actual exact fraction. Using
|t| radius(Q_j)+(|t|+1)radius(Q_i) is looser but sound.

For constrained reaction row c, unchanged exact ledger cancels and

    |R_G,c-R_K,c| <= sum_j KGbar_cj d_j + sum_j M_cj U_j.

Global-axis spring radii are |k|d_i; identical directional-spring radii are
sum_j |k_ij|d_j. Support-group component radii sum the actual constituent radii.
For any Euclidean magnitude, |norm(v_G)-norm(v_K)|<=norm(v_G-v_K)<=sum component
radii. Group membership and signed action orientation must bind to the same
source; absolute bounds do not authorize substituting a different functional.

Thus each kernel row with |x-q_K|<=R_K obtains source radius R_G=R_K+d_row and
interval [x-R_G,x+R_G]. Input-derived prescriptions retain their exact-source
one-round semantics. B1C's geometric intervals plus a valid action interval can
then feed B1's signed quotient/product corners; arbitrary derived formulas,
SIF, extrema and new hypot/span machinery are outside this bridge theorem.

## Final certificate and lifetime obligations

For each unchanged emitted raw y in U and pinned normalized n=N_U(y), enclose
intended SI truth in [l,h]. H_n=max(|n-l|,|n-h|); H_U=max(|y-l/a_U|,|y-h/a_U|).
Recompute final SI scales/classes from the actual full row set. Keep bare
H_n<=b_SI, both exact and pinned-operation binary64 sharper allowances, decimal
10^9 H_n<=|n| and raw 10^9 H_U<=|y| for relative rows, and all G5a/G5b/G5c,
p512 floors, source units and row identities unchanged. Source uncertainty does
not grant extra b bits. If b=0, only a proved exact equality can pass. A positive
radius's refusal does not prove actual output failure; a demonstrable nonzero
actual source error against b=0 would be an impossible unchanged predicate.

At main49034a940f, VerificationReport contains block certificates/norms/theta
(verify.rs:597–635); the scaling is available with live factors. RetainedSolve
retains states/cache and A1 radius bits (adaptive.rs:3423–3435), but does not
retain the report. finish_selected extracts summaries and moves publication
and radii (:4184–4213,4297–4306). A bridge requiring per-block B/S/W-plus must
run while correctly paired state/report/factor/source data are live or establish
an explicit new private bounded lifetime. Public summary maxima/radius bits
alone do not supply absent block facts. Retention/recomputation, source-map
ownership, clone/cache multiplicity, arithmetic sizes and actual stage accounting
remain implementation proofs. This stage derives no memory/work tariff or
universal success claim.
