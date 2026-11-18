---
name: swbpipe-theory-modal-analysis
description: Explain the modal method, its integrity checks and its current experimental standing.
metadata:
  kind: explanation
  surface: reference
---

# Modal Analysis

## Purpose and boundary

This note explains how SWBPIPE computes the natural frequencies and mode shapes
of a piping model, with the mass each mode carries. It covers the inputs as the
method uses them, the method, its assumptions and its limitations. The method
is defined by DEL-04-09 (`execution/PKG-04/DEL-04-09/ScopeOfWork.md`); where
this note and that scope differ, the scope stands and this note is the one to
correct.

**Standing.** A modal result is **experimental**. Every modal document states
`admission.status: experimental` and `public_engineering_result: false`. The
method's thresholds and parameters have selected values, which the document
states as its parameter standing, separately from that result standing. What
is still missing before a modal result can be relied on as an engineering
result is in [Limitations](#limitations).

**Numerical integrity does not establish physical correctness.** The checks
described below concern the discrete eigenproblem the method forms. They do not
bound modelling, discretization, input or physical error, and none of them is a
mathematical bound on an eigenvalue.

SWBPIPE does not determine code compliance. A frequency, a mode shape or a
participation factor is a mechanics result; whether a line is acceptable is the
user's engineer's judgement, under their own responsibility.

## Inputs

Every engineering input is the user's. No input below is defaulted by the
engine, the headless runner or any other entry.

| Input | What it means to the method |
|---|---|
| The model and its materials | The linear model whose free degrees of freedom form the eigenproblem, on its own route: pressure-free (PF) or the exact pressure route (EX). |
| Extraction bound | A mode count n, a frequency cut-off f_c, or both. At least one is required. The reported set is the lowest modes up to whichever bound stops first; the cut-off is inclusive. |
| State case | Required when the document is a load/reference-state document or the model has a nonlinear support. It names the load case whose resolved, solved state defines the modal model: its moduli, its mass inputs (including a test contents density), its nonlinear-support states and its restrained DOFs. |
| Modulus basis | Without a state case, the modulus comes from a load case's vocabulary through the existing resolver. With a state case, a request modulus basis is refused as a conflict. |
| Weight-to-mass acceleration a | Required when a component's weight enters the mass: a component body's mass is W/a. When the model carries an applied component-weight primitive, its recorded acceleration must equal a exactly. |
| Dynamic-only restraint choice | `include_dynamic_only_restraints` is recorded with the result. No support family the product admits today uses it. |

## Method

### The eigenproblem

The method solves the generalized eigenproblem

K_ff φ = ω² M_ff φ

on the free degrees of freedom of the linear model.

- **K_ff** is the restraint-reduced stiffness of the route's static solve,
  without load-dependent terms. The exact route adds no pressure stiffening:
  pressure enters it only as loads, which the modal solve ignores.
- **M_ff** is the restraint-reduced mass of the mass model below.
- Nonlinear supports enter at their final state in the named state case, solved
  in the same run: an active one-way or lift-off support, a closed gap or
  sticking friction rigidly restrains its DOF; an inactive or open support
  leaves it free; sliding friction leaves its tangential DOF free with no
  stiffness, which is disclosed as a modal approximation. A state is used only
  when the static solve confirmed it.

For each reported mode the result gives the frequency f in Hz, the circular
frequency ω = 2πf in rad/s and the period T = 1/f in s; the shape; the
generalized mass m = φᵀM_ffφ; and, in each global direction k, the
participation factor Γ_k = φᵀp_k / m, the effective mass (φᵀp_k)²/m, the
effective-mass fraction and its cumulative sum. Here p_k = (M R_k)_f, the free
rows of the full mass times the rigid unit translation R_k of every DOF, so
that consistent-mass coupling to restrained DOFs is included. Fractions are
relative to T_k = R_kᵀ M R_k, the model's total mass in direction k. The
generalized mass is in kg and Γ_k is dimensionless for a mode with
translation; they are in kg·m²·rad⁻² and rad/m for a mode with none (see
[Normalization](#normalization-a-display-convention)). The effective mass is
in kg in both cases, and fractions and their sums are dimensionless. If T_k is
0, the effective masses along k are 0 and their fractions are undefined, with
the reason.

### The mass model

The mass is formed on the same basis as the model's self-weight, so that the
two cannot disagree:

- **Straight members** carry the consistent mass of Euler–Bernoulli beam theory
  (the Hermitian consistent mass with Rayleigh rotary inertia), with linear
  axial and torsional mass. Their mass per length is exactly the self-weight
  generator's on the model's route: on PF, metal and contents on the wall
  t − c_m; on EX, metal on the nominal wall and contents on the pressure bore;
  insulation alike on both.
- **Contents** are in all three translations and excluded from torsional and
  rotary inertia; this is disclosed.
- **Realized arcs** carry two rigid-body half-arc lumps with the half-arcs'
  exact mass, centroid and second moments.
- **Rigid-family components** that self-weight applies carry a point mass
  m = W/a at their centre of gravity, offset c from the component's node, in
  place of the member's pipe mass. W and a must be positive normal binary64
  numbers, fl(W/a) a normal binary64 number, a·fl(W/a) finite and normal, and
  each entry of the body's spatial mass block (m·c_i, m·c_i·c_j) finite and
  either zero or normal. Otherwise the request is refused
  (`MODAL_COMPONENT_MASS_OUT_OF_RANGE`, naming the component). Under that
  premise the rounded recovered weight fl(a·fl(W/a)) differs from W by at
  most 2u + u² relative, u = 2⁻⁵³; the arithmetic argument follows below.
- **Massless families:** objective connectors (their replaced span carries
  neither self-weight nor mass), tie rods and constant-effort supports carry no
  mass; this is disclosed.
- **Restrained DOFs:** mass at restrained DOFs is excluded from M_ff and
  disclosed with the total model mass.

**Weight recovery in binary64.** Assume round to nearest, ties to even, for
both division and multiplication, with the admitted range conditions above.
Write η = 2⁻¹⁰²² for the smallest normal number and u = 2⁻⁵³. When the exact
quotient W/a is normal, m̂ = fl(W/a) = (W/a)(1+δ₁), |δ₁| ≤ u. For the
normal exact product am̂, Ŵ = fl(am̂) = W(1+δ₁)(1+δ₂), |δ₂| ≤ u, hence
|Ŵ−W|/W = |δ₁+δ₂+δ₁δ₂| ≤ 2u+u².

There is one additional division-boundary case: an exact subnormal quotient
can round to the admitted normal result m̂ = η. It must lie in
[η(1−u), η). To see which normal binary64 operands can do this, write
W = w·2ᵉ and a = v·2ᵈ with integers 2⁵² ≤ w,v ≤ 2⁵³−1. Then
(W/a)/η = (w/v)·2ᵏ. Since 1/2 < w/v < 2, a value in [1−u,1) requires
k = 0 or −1. For k = 0, w < v gives 1−w/v ≥ 1/v > u, so it is outside
that interval. For k = −1, w/(2v) ≤ (2⁵³−1)/2⁵³ = 1−u; equality alone
is admitted, forcing w = 2⁵³−1 and v = 2⁵². Thus the exact quotient is
the midpoint η(1−u), a is a power of two, and ties-to-even gives m̂ = η.
The admitted finite normal product aη is a representable power of two,
so multiplication is exact. Its relative error is u/(1−u), which is less
than 2u+u². The division's relative error in this case exceeds u; the
ordinary two-rounding proof does not apply to it.

Together these cases prove the same final bound for the admitted binary64
operands, without narrowing their range. The exact product am̂ and its
rounded evaluation Ŵ are distinguished throughout. The bound does not
cover uncertainty in the authored weight or gravity.

**Discretization warning required by the scope.** A member too coarsely
discretized for the highest reported frequency must be warned by name
(`MODAL_MASS_DISCRETIZATION_COARSE`). Its warning factor must be derived by
benchmark and stated in the warning. That calibration remains unresolved;
this note does not present the warning as delivered or a numerical-integrity
check as evidence that the mesh resolves the physical vibration.

**Weight-basis identity.** Every result runs a blocking check that a·M·r_k,
for each global direction k, equals the self-weight generator's loads at
gravity a along k over the mass-bearing members. It holds node by node for
straight members against the consistent uniform-load vector, and on the
resultant force and moment for arcs and components, within 2⁻⁴⁴ relative. A
failure refuses the request.

### The dense path

The product's modal entry uses the dense path. It reduces the pencil with K's
Cholesky factor, tridiagonalizes and applies implicit QL. It uses no new
numerical dependency.

### Clusters, completeness and the Sturm count

- A **cluster** is a maximal run of adjacent computed eigenvalues, each within
  δ_c = 2⁻²⁰ relative of the next. When the extraction bound falls inside a
  cluster, the reported set is extended to the whole cluster, and the result
  discloses the extension by count. A partial cluster is never published.
- **Completeness** is checked by a Sturm count: when the unpivoted LDLᵀ
  factorization of the scaled K − σM succeeds with nonzero scalar pivots, its
  number of negative pivots equals, in exact arithmetic, the number of finite
  eigenvalues below σ ([D4](#derivations)). The shift σ is placed in the gap between the last
  reported cluster and the next computed one. That count must equal the number
  of modes reported. A mismatch refuses the set. A shift whose factorization
  fails gives no count; another candidate shift is tried, and the set is
  refused by name when none succeeds.
- **Empty set.** A cut-off below the lowest frequency reports no modes. The
  count is then taken at an eigenvalue shift σ in the gap between
  λ_c = (2πf_c)² and the lowest eigenvalue of the first computed cluster,
  in rad²/s² throughout. Under the same premises the count must be 0. The result states
  that no mode lies at or below f_c under those premises.
- **A count is relied on only under two premises**, both disclosed with the
  shift. The estimated shifted-matrix backward error e_b times an estimate ν
  of its inverse norm must be at most 2⁻¹⁰ (conditioning). Each neighbouring
  mode's normalized Rayleigh separation estimate s_j must be at least 2⁷e_b
  (separation). [D5](#d5-normalized-separation-and-estimated-conditioning)
  defines the scaling, quantities and their limits. A factorization whose
  growth exceeds 2⁵² is rejected. Where no shift meets the premises the set
  is refused. These are empirical integrity criteria, not verified separation.
- **Whole spectrum.** When the reported set is the whole finite spectrum, the
  final shift lies above its largest eigenvalue. The dense path claims this
  only when the number of finite modes is established independently: the
  massless DOFs are identified structurally and a Cholesky factorization of
  the remaining mass block establishes its positive definiteness. D1’s
  exact rank argument alone does not establish this computational premise;
  otherwise the request is refused (`MODAL_FINITE_COUNT_UNRESOLVED`).

### Integrity checks

Every published mode set carries:

1. a **formation check**: K and M re-formed independently from their authored
   operands, element by element, in double-double precision (about a 106-bit
   significand), with the formation difference measured and published;
2. per mode, the **residual** ‖Kφ − λMφ‖ against the re-formed pencil, scaled
   as a backward error and compared with τ_r = 2⁻³⁰, with a first-order,
   residual-based estimate of the eigenvalue error. The estimate rests on the
   premise that the eigenvalue is isolated; for a cluster it is stated for the
   cluster as a whole. It is an estimate, not a bound;
3. the **mass-orthogonality** deviation of the published modes, against 2⁻²³;
4. the **Sturm-count** agreement above;
5. the **cluster** identifiers and any extension;
6. the **weight-basis identity**.

A mode whose residual exceeds τ_r, or an orthogonality breach, publishes the
result as **Sensitive** when nothing refuses the set. Each threshold is an
acceptance, demotion or grouping criterion, not an error or count theorem.

### Normalization: a display convention

Shapes are in global axes, and normalization fixes only their scale and sign:

- **(a) A mode with translation** is scaled so that its largest translational
  component is +1. Translations are then dimensionless and rotations in rad/m;
  the generalized mass is in kg.
- **(b) A mode with no translation**, such as pure torsion, is scaled so that
  its largest rotational component is +1 (rad/rad). Its translations are the
  computed vector's own, in m/rad, and its generalized mass is in kg·m²·rad⁻².
  A mode has no translation when its largest translation is at most
  τ_t·ℓ = 10⁻⁶·ℓ times its largest rotation, with ℓ the diagonal of the
  bounding box of the model's nodes.

Normalization is a convention for display and comparison, not a physical
amplitude: a mode shape has no physical size until a response analysis scales
it. Frequencies, effective masses and fractions do not depend on it; Γ and the
generalized mass do, consistently, so that Γ²·m equals the effective mass in
either case. Each mode records its case.

### Readers

A published document can be checked by the product's Rust modal reader
(`core/reporting/modal_result`), which shares no code with the producer, and,
in the repository's tests, by an independent Python verifier. Neither re-runs
the eigen-solve. Given the bound request, the reader re-derives the model's
mass and restraint basis within its coverage and recomputes the mass-dependent
quantities from that mass and the published shapes; it checks the counts,
ordering, clusters, Sturm placement and premises, the Sensitive standing and
every value's unit. Its verdict states each relation as passed, failed,
unresolved, scoped unsupported or not checked, and states that public
engineering-result admission is disabled. A consistent verdict shows that the
published relations agree with each other and, with the request, with the
model's mass; it does not show that the eigenvalues are correct.

## Derivations

The relations above are classical, and this note derives each of them from
the assumptions stated with it (DEL-11-03 REQ-004). Each derivation names the
frozen independent reference that corroborates particular cases of it, a
reference whose generator imports no product code. A reference case
corroborates a derivation; it does not replace it.

**D1. The eigenproblem and its finite modes.** Undamped free vibration of the
linear model on its free DOFs is M_ff ü + K_ff u = 0. K_ff is symmetric
positive definite, because the model is restrained (an unrestrained model or
mechanism is refused first). M_ff is symmetric positive semidefinite. A
synchronous motion u(t) = φ·sin(ωt) gives (K_ff − ω²M_ff)φ·sin(ωt) = 0 for
every t, so K_ff φ = λ M_ff φ with λ = ω². Then f = ω/(2π) in Hz and
T = 1/f in s.

*Massless directions.* Choose an orthogonal Q with
QᵀM_ffQ = diag(0, M_CC) and M_CC positive definite (the eigenvectors of M_ff,
null space N first). Write QᵀK_ffQ in the same blocks K_NN, K_NC, K_CN, K_CC.
K_NN is positive definite, as a principal block of a positive definite matrix.
With φ = Qψ, the N rows of K φ = λ M φ carry no mass:
K_NN ψ_N + K_NC ψ_C = 0, so ψ_N = −K_NN⁻¹ K_NC ψ_C. The C rows then read
K̂ ψ_C = λ M_CC ψ_C, with K̂ = K_CC − K_CN K_NN⁻¹ K_NC, which is positive
definite as a Schur complement of a positive definite matrix. So the finite
eigenvalues are those of (K̂, M_CC): there are rank(M_ff) of them, all
positive. A direction with no mass gives no finite mode. If M_ff is positive
definite, N is empty and nothing is eliminated.

*Orthogonality, repeated eigenvalues included.* With M_CC = G Gᵀ (Cholesky),
G⁻¹K̂G⁻ᵀ is symmetric, so it has an orthonormal eigenbasis, and within a
repeated eigenvalue that basis is a choice. Lifting each eigenvector
z_i = Gᵀψ_C,i gives modes with φ_iᵀM_ffφ_j = ψ_C,iᵀM_CCψ_C,j = 0 for i ≠ j and
m_i = φ_iᵀM_ffφ_i > 0. For λ_i ≠ λ_j that orthogonality is forced:
subtracting φ_jᵀ(K φ_i = λ_i M φ_i) from the transposed equation for j gives
(λ_i − λ_j) φ_jᵀ M φ_i = 0. Within a repeated eigenvalue it is the choice.
The dense path makes that choice by construction. It reduces with K's
Cholesky factor K = L Lᵀ to the symmetric C = L⁻¹ M L⁻ᵀ, whose orthonormal
eigenvectors y_i (μ_i = 1/λ_i > 0) give φ_i = L⁻ᵀ y_i with
φ_iᵀMφ_j = y_iᵀ C y_j = μ_i δ_ij in exact arithmetic. Check 3 measures what
rounding leaves. *Reference:*
`validation/references/f1_modal/mass_discrete/eigen_references.json` (class (a)
discrete eigenvalues, residuals and orthogonality, with an independent Jacobi
control) and `validation/references/f1_modal/continuum/continuum.json`
(closed-form beam, rod and ring frequencies).

**D2. Consistent mass and the weight identity.** *Straight member.* Take
member axes x along the member and y, z across it, length L and ξ = x/L. The
fields are the axial displacement u(x), the transverse displacements v(x) and
w(x), and the twist θ_x(x). The section rotations follow the right-hand rule:
θ_z = dv/dx and θ_y = −dw/dx. The interpolations are the ones the stiffness
uses. u and θ_x are linear in their end values. v is cubic Hermite in
(v_i, θ_z,i, v_j, θ_z,j), and w is cubic Hermite in (w_i, −θ_y,i, w_j, −θ_y,j),
with N₁ = 1 − 3ξ² + 2ξ³, N₂ = L(ξ − 2ξ² + ξ³), N₃ = 3ξ² − 2ξ³ and
N₄ = L(−ξ² + ξ³). The kinetic energy has three separate terms:

T = ½ ∫₀ᴸ [ m̄ (u̇² + v̇² + ẇ²) + j̄ θ̇_x² + ī (θ̇_y² + θ̇_z²) ] dx.

- m̄ (kg/m) is the mass per length: metal, contents and insulation, exactly
  the self-weight generator's.
- j̄ (kg·m, that is kg·m²/m) is the polar mass moment of inertia per length:
  metal density times the annulus's polar second moment on the route's metal
  wall, plus insulation density times the insulation annulus's.
- ī = j̄/2 (kg·m) is the transverse rotary inertia per length. For an annulus
  the polar second moment is the sum of two equal transverse ones.
- Contents enter m̄ only, so they are excluded from j̄ and ī.

Each term has the dimensions of energy: (kg/m)(m/s)² m and (kg·m)(rad/s)² m
are both kg·m²/s². Writing T = ½ q̇ᵀ M_e q̇ gives the element mass:

- axial: m̄L/6·[2, 1; 1, 2] on (u_i, u_j);
- torsional: j̄L/6·[2, 1; 1, 2] on (θ_x,i, θ_x,j);
- in each bending plane, the entries ∫ m̄ N_a N_b dx + ∫ ī N_a′ N_b′ dx,
  that is m̄L/420·[156, 22L, 54, −13L; 22L, 4L², 13L, −3L²; 54, 13L, 156, −22L;
  −13L, −3L², −22L, 4L²] plus ī/(30L)·[36, 3L, −36, 3L; 3L, 4L², −3L, −L²;
  −36, −3L, 36, −3L; 3L, −L², −3L, 4L²]. In the x–z plane these act on
  (w, −θ_y), so entry (a, b) carries the sign s_a·s_b with s = (1, −1, 1, −1).

The 12×12 is turned to global axes by TᵀM_eT with the member's rotation.

*Weight identity, straight member.* Let q = r_k be the nodal vector of a unit
rigid translation along global k, which is d = R e_k in member axes. Linear
and cubic Hermite interpolation reproduce a constant field exactly with zero
slope: N₁ + N₃ = 1, and the slope of N₁ + N₃ is 0. So u, v and w equal d_x,
d_y and d_z everywhere, and every rotation, θ_x included, is 0. The torsional
and rotary terms then contribute nothing, and
a·M_e r_k = ∫₀ᴸ Nᵀ (m̄ a d) dx. That is the consistent nodal vector of the
uniform load m̄·a along k, node by node.

*Weight identity, rigid bodies.* A component body is a point mass m at
global offset c (m) **from its node** to its centre of gravity, carried
rigidly by the node. Under small rotation θ its displacement is
u + θ × c = u − [c]×θ, so its mass block is
m·[I, −[c]×; [c]×, [c]×ᵀ[c]×]. Applied to a rigid translation (e_k, 0), it
gives m·(e_k, c × e_k). Times a, that is the force m·a·e_k and the moment
c × (m·a·e_k) about the node. These are the force and the moment about the
node of a weight W = m·a acting along k at the centre of gravity, up to
fl(W/a)'s rounding (bounded above). A realized arc's two rigid half-arc lumps
follow the same argument, with the half-arc's exact mass and its centroid
offset from its node. Under a rigid translation the lump's second moments
are multiplied by a zero rotation and do not enter. So the identity holds on
the resultant force and moment for arcs and components. *Reference:*
`validation/references/f1_modal/mass_discrete/mass_references.json` (the mass
matrices by shape-function integration and quadrature with these three
energy terms, offset component bodies, arc resultants and the
`weight_identity` cases).

**D3. Participation and effective mass.** *Premise (uniform support
motion).* Every support, a spring's grounded end included, moves with one
ground translation g(t) along k, so every DOF moves as u = R_k g + w, with w
zero at restrained DOFs. A rigid translation strains no element and no
spring, so the elastic forces are K w. The free rows of M ü + K w = 0 are
then M_ff ẅ_f + K_ff w_f = −p_k g̈, with p_k = (M R_k)_f.

*p_k lies in the range of M_ff.* Let A = M_ff, T_k = R_kᵀ M R_k, and let E
select the free DOFs. The matrix [A, p_k; p_kᵀ, T_k] is the Gram matrix
[E, R_k]ᵀ M [E, R_k], so it is positive semidefinite because M is. For a
positive semidefinite block matrix, p_k ∈ range(A) and
T_k − p_kᵀ A⁺ p_k ≥ 0, with A⁺ the pseudoinverse. (If Ax = 0, the quadratic
form at (t·x, −1) is −2t·xᵀp_k + T_k ≥ 0 for every t, so xᵀp_k = 0, and
p_k ⟂ null(A) = range(A)⊥. Minimizing the form over its first block gives
the inequality.)

*Modal equations.* In D1's coordinates, Qᵀp_k has no N part, because
range(A) is spanned by the C columns of Q. So the N rows carry neither mass
nor excitation, and they condense as in D1. The C rows are
M_CC ψ̈_C + K̂ ψ_C = −p_C g̈. Expand ψ_C = Σ_i ψ_C,i y_i in D1's complete,
mutually orthogonal finite modes, and multiply by ψ_C,iᵀ. Since
ψ_C,iᵀ p_C = φ_iᵀ p_k, this gives m_i ÿ_i + λ_i m_i y_i = −(φ_iᵀ p_k) g̈.
So Γ_k = φ_iᵀ p_k / m_i, and the mode's effective mass is
(φ_iᵀ p_k)²/m_i = Γ_k² m_i.

*Normalization.* Scaling φ_i by c scales m_i by c² and φ_iᵀp_k by c, so the
effective mass and the frequency are unchanged and Γ scales by 1/c. In case
(a) φ is dimensionless in translation, so m_i is in kg and Γ is
dimensionless. In case (b) translations are in m/rad and rotations in
rad/rad, so φᵀp_k is in kg·m/rad, m_i in kg·m²·rad⁻² and Γ in rad/m. The
effective mass is in kg in both.

*The bound.* Write p_k = A z with z = A⁺p_k (possible because
p_k ∈ range(A)). In the semi-inner product ⟨x, y⟩ = xᵀAy, φ_iᵀp_k = ⟨φ_i, z⟩,
and the published modes are mutually orthogonal with ⟨φ_i, φ_i⟩ = m_i > 0.
Bessel's inequality, which holds for a semi-inner product because it needs
only ‖z − Σ_i c_i φ_i‖² ≥ 0, gives
Σ_i (φ_iᵀp_k)²/m_i ≤ ⟨z, z⟩ = p_kᵀA⁺p_k ≤ T_k. So the effective masses
summed over the published modes never exceed the total mass in that
direction, in exact arithmetic. *Reference:*
`validation/references/f1_modal/mass_discrete/eigen_references.json`
(participation of the class (a) cases, including `E-AXIAL-WITNESS`, with
p = m/2 and m_eff = 3m/4, and `E-SS-1`, a rotation-normalized mode with
translational participation).

**D4. The Sturm count.** *Theorem, with its premise.* Let σ be a shift such
that the **unpivoted** factorization K − σM = L D Lᵀ succeeds: L is unit
lower triangular and every scalar pivot in D is nonzero. Then K − σM is
congruent to D. By Sylvester's law of inertia, K − σM has as many negative
eigenvalues as D has negative pivots. The same holds for the scaled matrices
the method factors, because a diagonal scaling S(K − σM)S is a congruence.

- If M is positive definite, K − σM = M^½ (M^{−½} K M^{−½} − σI) M^½ is
  congruent to M^{−½}KM^{−½} − σI. Its negative eigenvalues are exactly the
  pencil's eigenvalues below σ.
- If M is singular, in D1's basis K − σM has the blocks K_NN, K_NC and
  K_CC − σM_CC. Its inertia is that of K_NN plus that of the Schur complement
  K̂ − σM_CC (Haynsworth). K_NN is positive definite and adds no negative
  eigenvalue. The eliminated equation ψ_N = −K_NN⁻¹ K_NC ψ_C (D1) shows that
  (K̂, M_CC) has exactly the finite eigenvalues of the pencil.

So a successful factorization's negative-pivot count is the number of finite
eigenvalues below σ.

*A shift away from every eigenvalue does not ensure success.* The frozen
witness `WIT-ZERO-PIVOT` takes K = [2, −1, 0; −1, 2, −1; 0, −1, 1] and M = I,
with eigenvalues 2 − 2cos((2j − 1)π/7). At σ = 2 the shifted matrix is
nonsingular and has exactly two negative eigenvalues, yet the first
unpivoted pivot is exactly 0, so no count exists there. A pivoted
factorization with 1×1 and 2×2 blocks would count the negative eigenvalues of
its blocks, not merely its negative scalar entries. The method does not use
one. It rejects a candidate whose pivot magnitude falls at or below its guard, or
whose growth exceeds the limit, tries its next candidate shift, and refuses
the set by name when none is accepted.

*What binary64 adds.* The computed factorization is that of a matrix near
K − σM. The conditioning and separation premises above are criteria for
relying on the computed count. They are not a theorem that the binary64
count is correct, and this derivation adds no guarantee to them.
*Reference:* `validation/references/f1_modal/continuum/inertia_witnesses.json`
(exact counts by exact rational inertia, `WIT-ZERO-PIVOT` among them), the
exact counts in `mass_discrete/eigen_references.json`, and `LC-TIPMASS` in
`continuum/continuum.json`, a singular-mass condensation case.

### D5. Normalized separation and estimated conditioning

Let D₀ = diag(k_ii⁻½), K̃₀ = D₀KD₀, M̃₀ = D₀MD₀ and y = D₀⁻¹φ, for the
positive stiffness diagonal. The variable λ = (2πf)² and shift σ have units
rad²/s². Stiffness-diagonal equilibration makes Ã₀(σ) = K̃₀ − σM̃₀
dimensionless. For an exact eigenpair Kφ = λMφ,

q = (φᵀMφ)/(Σ_i k_ii φ_i²) = (yᵀM̃₀y)/(yᵀy),

yᵀÃ₀(σ)y/(yᵀy) = (λ − σ)q.

Thus q has reciprocal-eigenvalue units and converts a gap in λ to the
scale of a matrix perturbation. A raw absolute gap has the wrong units;
a relative gap alone omits that conversion. At a simple zero eigenvalue
α(λ) of Ã₀, differentiate Ã₀(σ)y(σ) = α(σ)y(σ) and left-multiply by yᵀ.
The terms involving y′ cancel because Ã₀y = αy, giving α′(λ) = −q.
Consequently |λ−σ|q is a first-order estimate of |α(σ)| near λ, not a
finite-gap distance bound. For repeated eigenvalues the directional
Rayleigh identity still holds; branch derivatives require diagonalizing
−M̃₀ on an orthonormal basis of the zero eigenspace. An arbitrary modal
direction does not identify one of those derivatives.

For computed neighbours the method uses
s_j = (|λ_j−σ| − e_λ,j λ_j)q_j and requires s_j ≥ 128 e_b, where e_λ,j is
the cluster's relative error estimate. The subtraction is an estimate's
allowance, not a certified correction. This comparison is an empirical
integrity criterion resting on the first-order interpretation above,
not a sufficient condition for the correct inertia. The frozen exact counts
corroborate particular cases; they supply no universal guarantee for it.

**Ideal and computed scaling.** Formation uses
D̂_i = fl(1/fl(√k_ii)); q uses the ideal D₀. With round-to-nearest arithmetic
and admitted normal intermediates,
D̂_i/D₀,i = (1+δ_div)/(1+δ_sqrt), with |δ_div|, |δ_sqrt| ≤ u = 2⁻⁵³.
Writing r₋ = (1−u)/(1+u) and r₊ = (1+u)/(1−u) therefore gives
r₋ ≤ D̂_i/D₀,i ≤ r₊. For the same vector, the quotient formed using D̂ has
denominator Σ_i k_ii φ_i²/(D̂_i/D₀,i)², a sum of nonnegative terms.
Its ratio to the ideal q is in [r₋², r₊²]; its relative difference is at most
r₊²−1 = 4u/(1−u)² < 4.45×10⁻¹⁶. This scaling difference is negligible
against the empirical factor 128. It does not promise identical decisions
exactly at a comparison boundary, or bound the other errors in s_j.

**Separate conditioning check.** The method estimates the shifted-matrix
error in the infinity norm by the formation/scaling term plus
γ_n‖|L̂||D̂_factor||L̂ᵀ|‖_∞, γ_n = nu/(1−nu). This is e_b, evaluated using
the computed scaling. For a symmetric perturbation E, ‖E‖₂ ≤ ‖E‖_∞;
this relates the matrix-norm scale to the Rayleigh comparison. The separate
check e_bν ≤ 2⁻¹⁰ uses an estimate ν of the inverse 1-norm. Neither e_b nor
ν is a certified bound. All growth, range and count-mismatch refusals remain.
Verified separation would additionally require justified enclosures and
certified error and inverse-norm bounds, as DEL-04-09's “Not established”
paragraph records. This note establishes none of those missing bounds.

*Checks and their reach:* `conditioning_and_separation_premises_against_independent_references`
in `core/solver/modal_eigen/tests/sturm_counts.rs` compares these estimates
with a separate shifted-matrix Jacobi calculation and inverse calculation
on the frozen reference population. The threshold checks are empirical.
The product's `modal_empty_set_rejects_a_missed_mode_below_cutoff` in
`core/product_physics/src/modal/tests_eigen.rs` exercises the reported-set
and count-refusal path on an authored two-mode pencil: withholding a 0.5 Hz
pair at a 1 Hz cutoff still gives count 1 and refuses an empty set. This
covers the product's modal count caller, not a full-model end-to-end solve.

## Assumptions

- Small displacements, linear elasticity, and free vibration about the
  unloaded configuration of the linear model, with nonlinear supports
  linearized at the named state.
- No damping: the frequencies are undamped natural frequencies.
- Euler–Bernoulli straight members (Timoshenko members are refused; see
  [Limitations](#limitations)).
- No pressure stiffening on either route; thermal, pressure, wind, seismic,
  nodal and member loads are ignored by the eigenproblem and disclosed.
- The mass is the self-weight mass only. There is no separate seismic mass
  basis and no concentrated-mass field.

## Limitations

- **Experimental, not an engineering result.** DEL-04-09's Admission
  conditions are not met: the calibrated input class and its complete
  allocation, stopping and publication protections are not established.
  Production caps remain unset. The existing headless entry supplies no
  internal weighted-work budget and remains experimental; a supplied legacy
  internal budget still meets the preflight's unbounded-term refusal. The
  command applies the dense memory guard, request/model refusals and integrity
  checks. The selected empirical resource policy gives no elapsed-time or
  process-memory guarantee and changes no numerical check.
- **Dense path only.** The product's entry solves on the dense path, under the
  dense memory guard; a model too large for that guard is refused. The sparse
  path is not a product entry.
- **Timoshenko straight members are refused** on either route
  (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`) while the Timoshenko mass is not yet
  formed by the product's mass assembly, the integrity re-formation and the
  readers. Exact-route authoring writes Timoshenko members by default, so a
  model authored with that default is refused (`docs/RELEASE_LIMITATIONS_v1.0.md`
  LIM-020).
- **Realized bends** enter the stiffness with a thin curved-beam element whose
  application range SWBPIPE's verification has not established; a modal result
  of a model with realized bends inherits that limitation
  (`docs/RELEASE_LIMITATIONS_v1.0.md` LIM-021).
- **Skewed restraints** are refused by the modal document
  (`MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED`): the document states a global
  mass and global shapes only.
- **An unrestrained model or a mechanism** is refused through the existing
  singularity refusal; a model with no mass on its free DOFs has no finite
  mode and is refused (`MODAL_NO_FINITE_MODES`).
- **Not recorded.** A modal result is not an analysis run: it is not saved
  with a project and is run again when needed. It is not part of the static
  result envelope, a released report or an ordinary mechanics export. Saving
  the headless command's output through `--output` saves the experimental
  document itself, with its stamp, under the local-private control.
- **Normalized shapes** are not physical amplitudes (see
  [Normalization](#normalization-a-display-convention)).

## Evidence

The frozen references under `validation/references/f1_modal/` (mass, discrete
and continuum references) and `validation/references/modal_timoshenko_mass/`
were each derived by a generator that imports no product code. The product's
modal tests compare against them, and the readers are held to the committed
reader corpus (`fixtures/results/modal_corpus.json`). Which acceptance
criteria of DEL-04-09 these cover, and which remain open, is the scope's
account.

## Source notes

- Commitments: `execution/PKG-04/DEL-04-09/ScopeOfWork.md`.
- Implementation: `core/product_physics/src/modal.rs` and `modal/` (request,
  mass, weight basis, dense path, integrity, document);
  `core/solver/modal_eigen` (the eigen kernel and Sturm count);
  `core/reporting/modal_result` (the Rust reader).
- The generalized eigenproblem, consistent beam mass, the weight identity,
  participation and effective mass, and the inertia count of an LDLᵀ
  factorization are classical structural dynamics and linear algebra. This
  note derives each one in [Derivations](#derivations), names the independent
  reference that reproduces it, and takes no formula, diagram or wording from
  an outside source.
