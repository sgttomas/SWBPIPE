# I33 — one bounded source/action bridge

**Conditional mathematical feasibility; not a selected design, implementation,
availability result or acceptance.** Fresh native TASK
`/root/i33_f2a_source_action_bridge` under ROOT `/root`. Receipt
2026-10-02 20:39:01 UTC; new-analysis cutoff 21:29:01; return deadline 21:39:01.
P = projects/chirality-piping; PP = P/core/product_physics;
FK = P/core/solver/frame_kernel. Source pin:
`49034a940f3f8cd3f3da4d4cbc839943b808063d`. The assigned brief and ROOT disposition
are at `6f64b65710`; B1/B1C/RV42 are at `751ac56ce8`, `a4e21279b9`,
`7a45f14b46`. Origin hashes and finite arithmetic are in `_run_records`.

**Answer.** A finite, conservative perturbation certificate can enclose the
source-geometric response and required linear action rows of the unchanged
admitted K solve, under the explicit premises below. It needs no new solve,
factorization, source operator, public bound, output value or precision retry.
It can succeed or refuse. It cannot establish that every admitted K publication
will satisfy the unchanged source-truth predicates. The remaining work is a
checked source/private-state interface and implementing arithmetic/resource proof,
followed by actual coverage/availability qualification. This packet does not
waive any of those obligations.

## 1. Required truth, producer values and ordinary route

| Layer | Existing warrant and consequence |
|---|---|
| K problem | D1 §4.1.1–2; FK retained/source.rs:1–21,90–104 admit E,G,A,Iy,Iz,J and other binary64 primitives exactly. Exact-coordinate chord/frame and the exact individual nodal-load ledger define K. R7 plus corrected A1 bounds each published x around this q_K. |
| Exact-profile source problem | PP lib.rs:1024–1035 says common selected E/nu, derived G, source OD/effective wall. SOURCE_ODWALL_EXPECTATIONS.json:6–8,407–408 specifies exact normalized OD/effective-wall geometry, real pi, unchanged protected comparisons. The no-pressure test at pressure_section_geometry.rs:459–469 still checks source-I displacement/rotation and source-J torsion at :287–324. This is a response promise, not only a stress-denominator promise. |
| Producer | PP lib.rs:6507–6525 uses rounded SourceAnnulus A/I/J/Z accessors. pressure_exact.rs:235–252 derives actual G_hat by Scaled operations; pressure_material.rs:84–91 installs it. These remain the actual K inputs and producer operands. They are not exact source-property singletons. |
| Source boundary | D is the normalized OD bit; t is the effective-wall bit actually passed to SourceAnnulus. PP lib.rs:9436–9471 and result_export/physics_source.rs:918–939 preserve RN64(normalized nominal wall minus normalized tolerance). Do not replace t by an unevaluated authored difference. Selected E/nu custody is checked at physics_source.rs:945–977; no material-id or base-material substitution. |
| Receipt/output | Actual A_hat/Z_hat/L_hat and stiffness-scale bits remain operational scale inputs (D1 §4.1.6.1; D2 §4.9.3). Actual final y, unit U, normalization n, classes, p512 floors and bounds remain unchanged. A geometric enclosure changes none of those bits. |
| Ordinary/preview | preview_physics.rs:73–80 and PP lib.rs:1929–1940 retain Euler–Bernoulli/section-formula promises. The ordinary mill-tolerance test at :18605–18650 verifies the rounded construction, not a universal exact-I_K/c truth contract. A1 proves q_K; neither that fact nor the word preview permits reinterpreting every ordinary readout as f(q_K,A_K,I_K/c). Ordinary row warrants still need explicit reconciliation. This packet selects no ordinary-route truth. |

For this derivation q_G denotes the *same intended linear Euler–Bernoulli
assembly and recovery* as K, with the exact-profile source section and selected
E/nu properties. The cited profile/reference fixes this intended source layer.
The implementing source map must prove that this is each covered row's actual
contract, including its frame, load and support law; the fixture alone is not a
proof of every product mapping. No pressure, eigen/member load, nonlinear support,
exact-profile combination or unsupported family is introduced through this notation.

## 2. Source and operator premises

All identities refer to the same captured invocation, normalized model, case,
selected material basis and immutable final rows. Source bytes, ledger terms,
node/member/DOF/station/support maps, row signs and units must be checked, as in
B1's binding proposal. Lifetimes alone do not prove that identity.

1. Exact finite normalized D,t satisfy the existing annulus premises, in
   particular 0<t<D/2; current SourceAnnulus and W1 admission remains intact.
   B1C's fixed reviewed pi bracket gives strictly positive A_G,I_G,J_G,Z_G
   intervals. Its formulas are c=D/2, ri=c-t, A=pi*t*(D-t),
   I=A*(c*c+ri*ri)/4, J=2I, Z=I/c. Do not treat reported rounded radii as source.
2. E_s>0 and -1<nu_s<1/2 are the actual selected source bits. E_G=E_s and
   G_G=E_s/[2(1+nu_s)] exactly; the latter is a positive rational, not G_hat.
   Verify E_K=E_s for this route. Compare G_G with the actual admitted G_K.
   If a different selected-material boundary is required, return the missing
   source warrant; do not synthesize an interval for the wrong material.
3. Both problems use identical exact normalized coordinates and y_reference,
   nonzero chord and nonparallel reference, identical exact orthonormal frame,
   free/constrained partition and exact prescribed values. No rounded product
   length or transform is substituted. Both use the same identified individual
   nodal load terms and global-axis positive spring coefficients. Therefore
   delta f=0 and spring delta K=0. Load cancellation does not authorize replacing
   the ledger by a rounded net. W1a restrictions still apply even though the
   algebra below explicitly includes prescribed coupling.
4. Write K for the exact admitted-bit stiffness and G for the source stiffness.
   Their element forms are B_e^T D_e B_e in the common exact frame. In basic
   order, D=diag(EA/L,GJ/L) plus (EI_z/L)[[4,2],[2,4]] and
   (EI_y/L)[[4,2],[2,4]]. All coefficients are positive. The same positive
   springs are added. These PSD energy terms have the same nullspaces when
   properties change positively. The accepted mechanical problem must have its
   stated unique solution; a successful floating factor is not that premise.
   On every certified data block K is invertible
   by R7 Lemma C, hence SPD; positive source ratios then preserve its nullspace
   and G's mathematical uniqueness. The numerical certificate below also proves
   invertibility wherever its strict inequality passes.

No unit label proves operator equality. A different frame/constraint/load law
invalidates premise 3. It needs its own delta-B/delta-f/prescription proof and is
outside this packet, rather than a zero perturbation by convention.

## 3. Finite majorants without a new geometric solver

There is a deliberately loose rational construction, so this proof does not
depend on assuming that rounded R7 matrices are exact geometric matrices.
For exact chord d let ell=max(|d_x|,|d_y|,|d_z|)>0. Then L>=ell. Exact frame
entries have absolute value <=1. Let Bbar majorize |B_e| as follows (12 global
columns, two nodes): axial row has 1 at each node's three translation columns;
twist row has 1 at each node's three rotation columns; each of four bending rows
has 1/ell at all six translation columns and 1 at its endpoint's three rotation
columns. Other entries are zero. Thus Bbar has 48 nonzeros. Let Hbar majorize
|B_local| using its actual signed pattern with 1/L replaced by 1/ell; it has
16 nonzeros. These bounds hold even when the reference is nearly parallel;
existing admission and R7's g check are still required.

For each of the four positive numerators

    C_K = (E_K A_K, G_K J_K, E_K I_zK, E_K I_yK),
    C_G = (E_s A_G, G_G J_G, E_s I_G, E_s I_G),

form source intervals [C-,C+] and d_C=max(|C--C_K|,|C+-C_K|).
All comparisons use exact/directed arithmetic with provenance-bound operands.
Form nonnegative six-by-six Dbar_G and Dbar_delta in the same ten-entry pattern
as D, using respectively C+/ell and d_C/ell, and the positive constants 4,2
in the bending blocks. Consequently

    |D_G| <= Dbar_G,  |D_G-D_K| <= Dbar_delta,
    |G_e-K_e| <= E_e := Bbar^T Dbar_delta Bbar.

Assemble E by summing these nonnegative contributions on actual DOF maps.
It majorizes |Delta|, Delta=G-K; it retains free-to-constrained columns too.
Identical springs contribute zero to E. These are proof calculations only;
they never replace an element matrix or actual output recovery.

## 4. One sufficient response and action theorem

Let S=diag(s_i)>0 be the **verification factor's own radix scaling** at P=2p,
in its actual free ordering. R7 §5.5 Lemmas D/E bound the inverse of the
*formed scaled* matrix by B_c. Lemma C, using the admitted theta_c<=1/2 and g
scope, gives for the exact bit-input free block

    ||(S K_cc S)^-1||_infinity = ||(S K_cc S)^-1||_1 <= 2 B_c.

Use beta_c=2 B_c, or conservatively 2 times the retained upward **body** maximum
B_b for every data block in that body. This is not rcond, a condition estimate,
the unscaled inverse norm, nor an inverse bound for G. Do not omit the factor 2.

For each free component, A1 supplies a checked actual x_j and R_j with
|x_j-u_Kj|<=R_j; set M_j=|x_j|+R_j. At a constrained component use its exact
prescribed value and M_j=|u_Cj|. Missing/unpublishable radii are not zero.
For each data block c compute outward upper bounds

    eta_c >= max_i_in_c s_i sum_j_in_c E_ij s_j,
    v_c   >= max_i_in_c s_i sum_all_j E_ij M_j,
    alpha_c = beta_c eta_c.

The same structural pattern must contain every Delta coupling. If a purported
new coupling crosses blocks, refuse the association. Require **alpha_c<1**;
with a positive downward denominator compute

    tau_c >= beta_c v_c / (1-alpha_c).

Proof: for e=u_G-u_K, e_C=0 and

    (K_FF+Delta_FF)e_F = -Delta_Fall u_K.

In scaled variables z=S^-1 e_F the right side has infinity norm <=v_c.
The Neumann inequality bounds the inverse by beta_c/(1-alpha_c), whence
|e_i|<=s_i tau_c. It is a theorem about the two exact operators, not the
computed factor's residual. All inequalities remain sound if majorants grow.

A block with no R7 data may be assigned zero bridge error only with the same
source-level zero proof: every individual free ledger term is zero, no pattern
adjacency has a nonzero prescription, and the verified state is zero there.
Common pattern and positive-energy/nullspace premises preserve the homogeneous
source solution. Never use an observed zero or missing B as that proof. If its
zero/uniqueness premises cannot be established, return a missing-bound/source
refusal; do not extend R7's bound beyond its data-block scope.

For a required linear action row write, with its exact signs and frame,

    q_K=a_K u_K+b_K,  q_G=a_G u_G+b_G.

Given row majorants Ag_j>=|a_Gj| and Da_j>=|a_Gj-a_Kj|, set

    T_j=s_j tau_block(j) on free DOFs,  T_j=0 on constrained DOFs,
    e_q = sum_j Ag_j T_j + sum_all_j Da_j M_j + |b_G-b_K|.

Then |q_G-q_K|<=e_q. Local end rows use
Hbar^T Dbar_G Bbar and Hbar^T Dbar_delta Bbar. Constrained global reactions use
the assembled global rows and b=-f_C; its difference is zero for identical
ledgers. Spring actions use their actual signed -k u law (Da=0), and attributed
support sums need the exact source-selected component/group map. Station rows
use FK recover.rs:373–395's exact fraction and basic-action signs: moments are
t Q_j+(t-1)Q_i. Apply absolute coefficients t and 1-t to the corresponding
majorants; do not substitute rounded L or a product statics row of another
functional. N,V,T use their identified end/basic rows. This covers linear
actions, not SIF, stress hypot or span maxima.

For displacement/rotation components e_q=T_i. If a required direct nodal/support
magnitude is mapped to its already certified kernel magnitude row, the sum of
its component bridge bounds is a sufficient additional error by the norm
triangle inequality; no new hypot evaluation or maximum claim is made here.
Prescribed InputDerived components follow their existing exact-source contract.

Finally source truth is enclosed by

    [x_q-R_q-e_q, x_q+R_q+e_q].

B1 can then apply its signed source A/Z and c/J recipes to these **source-action**
intervals. For a directly mapped row this interval itself is the recipe input.
An equilibrium identity can set e_q=0 only for the exact row it proves; equality
of a total reaction never proves equality of its branches or displacement.

## 5. Unchanged publication predicates and finite refusal

Use the actual final y,U, normalized n and source-recipe interval [l,u]. For the
positive raw-to-SI factor a, B1 gives

    H_n=max(|n-l|,|n-u|),  H_U=max(|y-l/a|,|y-u/a|).

Recompute final scales/classes exactly as corrected A1/D2 require after all row
replacement/qualification. Absolute requires H_n<=the existing b_SI bits.
Relative requires H_n<=A_exact, H_n<=A_f64, 10^9 H_n<=|n| and
10^9 H_U<=|y|, with A_exact=2^-64 max(|n|,S)(1+2^-21)+2^-53|n|+2^-1074
and A_f64 the unchanged five-step binary64 comparator in A1's addendum §2.
No normalization round trip erases error; no new public radius or class follows.
Keep p512 force/moment floor placement, small-scale bounds, b=0 exactness and
G5a checks. Operational scale A_hat/Z_hat stay distinct from source denominators.

Fixed source passes, one inequality evaluation and one final recipe check suffice.
Use B1C's constants; no runtime pi series, root solver, new factorization,
precision escalation or retry. A fixed reviewed arithmetic format must bound
mantissa, exponent/span, counters, scratch and source-derived iteration counts.
Per-member exact rational work can use checked fixed integer buffers; row sums
can be rounded outward to a fixed precision to avoid accumulating products of
unrelated denominators. Every rounding direction, cancellation/difference bound
and positive denominator must be proved. No format size or universal recipe-fit
claim is selected here. Cap, work or range failure returns a named arithmetic
refusal, never truncation, zero or arbitrary-precision heap fallback.

Other finite refusals include missing source/material/row identity, unsupported
functional or missing radius/bound, invalid geometric/material premises,
unproved common frame/load/constraint map, alpha>=1, or a failed final predicate.
alpha>=1 is certificate insufficiency, not a singularity claim. Refusal does not
turn a covered row into NotCovered, relax a protected criterion or block otherwise
publishable ordinary output. Routing remains the existing F2a transaction; this
calculation does not add a kernel attempt or amend the selected schedule.

## 6. Exact controls, consequence and next check

`_run_records/exact_bridge_controls.py` has 12 passing standard-library exact
ABSTRACT groups, using only closed scalar/two-by-two formulas. They cover a
distinct-operator success, sharper-predicate refusal despite decimal success,
bare-b zero-row refusal, alpha=1 refusal with an invertible source matrix,
prescribed coupling, stiffness-dependent branch action versus equilibrium total,
radix scaling, nonzero K publication radius, annulus and E/nu-to-G differences,
invalid/absent operands, exact ledger terms, and checked-cap refusal.

In particular K=I, f=(0,1), G=[[1,d],[d,1]], d=2^-52 gives u_K=(0,1) but
u_G,0=-d/(1-d*d). G is SPD, obtainable from positive mode weights 1+d and 1-d.
With normalized scale 1 the unchanged zero-row bound is 2^-64, which this source
response exceeds. No truthful certificate can admit that unchanged output under
that bound. This is an abstract discrimination, **not a product-admission witness,
real W1 defect or availability measurement**. It shows why a universal transfer
claim cannot follow from closeness of positive properties.

If the first conservative certificate is too loose, a bounded sign-aware
enclosure of Delta*u_K and actual recovery rows, or a separately proved static
identity for specified actions, can tighten the same theorem. It needs new proof
and accounting, not changed truth. It cannot rescue a row whose actual source
error already exceeds a protected predicate.

If actual protected availability and immutable outputs conflict, concrete ROOT
alternatives have distinct consequences: retain the existing ordinary/source
route and record the W1 proof gap under current routing; separately authorize
source-aware operator/output work (which changes D1's operand/operator or actual
output-method contract and requires new proof/verification); or explicitly
reconsider public bounds/identity/source promises with their owner-held reader,
reference and replay consequences. Calling source q_G a q_K readout would change
the exact-profile promise and protected source reference; it is not a correction
within this assignment. No alternative is selected here, and no accepted
availability requirement is waived.

The smallest next check is a fresh independent derivation of §§2–4 from the
named source profile, D1/R7 Lemma C and actual basic-action formulas, followed
by review of the private custody/format seam in `INTERFACE_AND_COST.md` and the
12 exact controls. Only then can ROOT scope arithmetic/memory and source mapping
implementation work. Full F2a, SIF/hypot/span maxima, memory qualification and
real availability remain outside this packet. Stop for that review before reliance.
