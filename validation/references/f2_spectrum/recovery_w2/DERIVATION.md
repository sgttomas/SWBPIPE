# W2 derivation: inertia-inclusive recovery through a realized arc, massive face pipes and a massless connector

This file restates the mathematics the generator computes. Each step cites
the governing text it rests on:
- the scopes of DEL-04-10 (CLM-001, REQ-002 to REQ-004);
- the scope of DEL-04-09 (REQ-001, REQ-002, REQ-005);
- the scope of DEL-03-06 (objective connector, replaced span, tie rods);
- the scope of DEL-03-03 (realized bend);
- the independent reference contracts named in [README.md](README.md#independence-and-provenance).

Nothing here is taken from product code or product output.

Notation: u is a displacement over all DOFs, g an acceleration field over all
DOFs, F = M g the inertia load. Six DOFs per node (ux, uy, uz, rx, ry, rz);
rotations are small right-handed rotation vectors. An element's 12 DOFs are
node i's six, then node j's six.

## 1. Constituents

### 1.1 Straight members (P1, P2, P3)

- **Stiffness.** The Euler–Bernoulli 12×12 with A, I and J = 2I on the
  nominal wall, from the frozen f1 `elements.straight_stiffness`.
- **Mass.** The consistent mass `elements.straight_mass` (DEL-04-09 REQ-001):
  - Hermitian translation mass with m̄;
  - linear axial and torsional mass (j̄);
  - Rayleigh rotary inertia ī/(30L)·[…].
- **Inertia terms.** Contents count in the three translations. They are
  excluded from j̄ = ρ_m·J_metal and from ī = j̄/2.
- **Exactness.** The straights in the five original variants have rational
  lengths and axis-aligned unit axes. The additive V-ARC-OBL straights have
  rational lengths and oblique rational unit axes. In both cases K is formed
  exactly in ℚ + ℚπ, published as `K_exact`, then converted to 80 digits.
  M is formed by f1's `form_mass` at the working precision.
- **Section basis.** The section has c_m = c_a = 0, so the PF basis
  (`source_od_effective_wall_areas/v1`) and the EX basis
  (`source_od_nominal_wall_pressure_bore_areas/v1`) give identical A, I, J, m̄,
  j̄ and ī. The generator asserts this entry by entry.
  - **Limit:** this coincidence does not exercise, or qualify, the distinction
    between the nominal and reduced bases. W1 examines that distinction
    separately.

### 1.2 Realized arc BEND1

- **Stiffness.** f1 `elements.arc_stiffness`. It is the exact Castigliano
  stiffness of the clamped circular arc for the complementary energy density
  N²/(EA) + T²/(GJ) + k(|M|² − T²)/(EI):
  - one user factor k on both bending components, no shear deformation;
  - inverted and spread to node i by rigid-body equilibrium, K = Bᵀ K_jj B,
    with B = [−Γ, I] and Γ = [[I, −[δ]×], [0, I]].
- **Inherited limitation.** This is DEL-03-03's thin curved-beam model. Its
  application range is not established (LIM-021). Nothing here widens it.
- **Mass: the half-arc lumps.** These are DEL-04-09 REQ-001, built by f1
  `form_mass`. Each half-arc (ψ ∈ [0, φ/2] from node i, and [φ/2, φ] from
  node j) is a rigid body about its end node:
  - mass m = m̄Rφ/2;
  - first moment S = ∫m̄ρ ds;
  - second moment J_n = ∫m̄(|ρ|²I − ρρᵀ) ds + ∫(j̄ttᵀ + ī(I − ttᵀ)) ds.

  Its 6×6 is [[mI, −[S]×], [[S]×, J_n]] at the end node's DOFs. It comes from
  the kinetic energy of v = u̇ + θ̇ × ρ.
- **Inertia attributed to the ends.** Under an acceleration field g, lump i
  carries the wrench M_lump,i·g_i: its force, and its moment about node i.
  - This is `BEND1.lump_load.i.*`, and likewise at j.
  - It is the only arc inertia. No consistent arc mass exists, and the two
    lumps do not couple the ends.
- **Exactness.** The arc's chord is R√2, and its frame and half-angle
  functions are irrational. So arc quantities are formed in the 80-digit
  Decimal field, with π by Machin's series, the arc angle by f1's arctangent
  series, and square roots by Decimal's correctly rounded operation. No libm
  value enters.

### 1.3 Objective connector J1 (the replaced span D–E)

- **Kinematics and frame.** These are restated from DEL-03-06's definition and
  the T4-I12 kinematic contract (symmetric midpoint, small rotation):
  - v_i = u_i + θ_i × a_i and v_j = u_j + θ_j × a_j, with zero offsets here
    (a_i = a_j = 0);
  - r = (x_j + a_j) − (x_i + a_i) and θ_c = (θ_i + θ_j)/2;
  - q_t = Qᵀ[(v_j − v_i) − θ_c × r] and q_r = Qᵀ(θ_j − θ_i), so q = B d.
- **The frame Q.** Its columns are the connector axes:
  - x = r/|r| = (0, 1, 0);
  - y is the authored integer guide (4, 0, 3) made normal to x and
    normalized in exact rational arithmetic: (4/5, 0, 3/5);
  - z = x × y = (3/5, 0, −4/5).

  Q is an exact derived rational frame. It is not an input binary64 vector:
  0.8 and 0.6 have no exact binary64 form, and none is used. The generator
  asserts QᵀQ = I and det Q = 1 exactly.
- **B in closed form.** With θ × a = −[a]×θ and −θ_c × r = [r]×θ_c:
  - ∂q_t/∂u_i = −Qᵀ;
  - ∂q_t/∂θ_i = Qᵀ([a_i]× + [r]×/2);
  - ∂q_t/∂u_j = Qᵀ;
  - ∂q_t/∂θ_j = Qᵀ(−[a_j]× + [r]×/2);
  - ∂q_r/∂θ_i = −Qᵀ and ∂q_r/∂θ_j = Qᵀ.

  The generator also forms B column by column from the kinematic definition
  on the 12 unit vectors, and asserts the two agree exactly. It asserts
  B·d = 0 exactly for the three rigid translations and the three rigid
  rotations about the origin.
- **Stiffness.**
  - K_c = H/(d_a d_b), with d = (Ls, Ls, Ls, 1, 1, 1) and Ls = 1 m.
  - H is diagonal and positive, with work coefficients (150000, 400000,
    250000, 500000, 1500, 1000) N·m for dimensionless q̂ = D⁻¹q. The
    physical K_c = D⁻¹ H D⁻¹ has translational entries in N/m and rotational
    entries in N·m/rad. At Ls = 1 m and rotation scale 1 rad the numerical
    entries coincide; their dimensions do not.
  - K_e = Bᵀ K_c B, exact in ℚ.
  - The unequal lateral and bending terms are invented, so that the connector
    has no axis of symmetry and the rotated frame enters K itself. They also
    keep the rods-absent straight-line variant free of exactly repeated
    eigenvalues.
- **Observables.**
  - g = K_c B u: the work-conjugate connector force in connector axes
    (`J1.g.N`, `Vy`, `Vz`, `T`, `My`, `Mz`).
  - Bᵀg: its action on the connector at its two end nodes, in global axes
    (`J1.i.*`, `J1.j.*`). The generator checks it against K_e u.
- **No mass, no replaced span.** The connector is massless, and the replaced
  span D–E carries no pipe stiffness, mass, self-weight or member result
  (DEL-03-06; DEL-04-09 REQ-001).

### 1.4 Rods R1 and R2

- **Definition.** They are restated from DEL-03-06 and the T4-U5b rod
  definition (`explicit_axial_rods_v1`, bilateral linear):
  - offsets a = ±ρ·Q_y with ρ = 0.1875 m, at both ends, so each rod is
    parallel to Q_x and 0.5 m long;
  - e = n·(v_j − v_i) with v = u + θ × a;
  - since n·(θ × a) = θ·(a × n), b = (−n, −(a × n), n, a × n);
  - K_rod = k b bᵀ, with k = 1.2e8 N/m;
  - T = k(e − e_ref), with e_ref = 0. Nonzero rod pretension is outside this
    family.
- **Observables.**
  - `R*.T` is tension positive.
  - `R*.i.*` and `R*.j.*` are b·T: the action on the rod at its end node,
    global axes, with the moment about the node including the offset.
- Rods are massless.

### 1.5 Supports

Anchors restrain all six DOFs of their node. No compliant support is in this
family.

## 2. Modal basis (DEL-04-09 REQ-002, REQ-005)

- **The eigenproblem.** K_ff φ = λ M_ff φ on the free DOFs. It is solved by
  Cholesky of K_ff and cyclic Jacobi on R⁻¹M_ffR⁻ᵀ at 80 digits (f1
  `eigen.route_b`), then a Rayleigh quotient. Every finite mode is computed.
- **No exact repeats, no ambiguous gaps.** No two eigenvalues lie within
  1e-30 relative. No adjacent relative gap lies in [δ_c/2, 2δ_c], with
  δ_c = 2⁻²⁰. Every scope cluster is a single mode.
- **Normalization.**
  - Case (a): unit maximum translation.
  - Case (b): unit maximum rotation, when the largest translation is
    ≤ τ_t·ℓ·(largest rotation), with τ_t = 1e-6 and ℓ the diagonal of the node
    bounding box.
  - Ties within 1e-25 go to the lowest DOF.
  - The joint variants' pure-torsion modes 5 and 9 are case (b), with zero
    participation.
- **Participation.**
  - p_k = (M R_k)_f, with R_k the rigid unit translation of all DOFs;
  - m = φᵀM_ffφ, Γ_k = φᵀp_k/m, m_eff = (φᵀp_k)²/m;
  - T_k = R_kᵀ M R_k.

## 3. Spectrum response (DEL-04-10 CLM-001, REQ-002 to REQ-004)

### 3.1 Spectra

- **Tables.** Three invented pseudo-acceleration tables with dyadic knots:
  - `S-W2-LL`: frequency abscissa, linear/linear;
  - `S-W2-GG`: frequency abscissa, log/log;
  - `S-W2-PLG`: period abscissa, linear in period, log ordinate.
- **Directions.** X uses S-W2-LL ×1, Y uses S-W2-GG ×0.75 and Z uses
  S-W2-PLG ×0.5.
- **Interpolation.** It is on the table's own abscissa. A period table is
  located by T = 1/f.
- **Margins.** Every periodic mode and f_r lies inside the range and more
  than 1e-30 relative from a knot. ln and exp are Decimal's correctly rounded
  functions.

### 3.2 Requests

- **Damping.** ζ is the binary64 nearest 0.05,
  3602879701896397/72057594037927936, taken exactly.
- **Per variant.**
  - The extraction cut-off f_c lies in a spectral gap.
  - The rigid frequency f_r lies between modes and off every knot.
  - The extraction premise holds: the lowest frequency of the highest
    extracted cluster is ≥ f_r.
- **Rules.** The base rules are CQC, rigid SRSS and directional SRSS. The
  grid covers every modal rule × {SRSS, ABSOLUTE_SUM, OFF} × {SRSS, 100-40-40,
  ABSOLUTE_SUM}.

### 3.3 Periodic contributions, recovered with inertia

- **The response of mode i in direction k.**
  - u_ik = Γ_ik·s_k·S_k(f_i)/ω_i²·φ_i;
  - g_ik = ω_i² u_ik, so F = M g.
- **Recovery,** with each quantity kept signed:
  - straight element: K_e u_e − M_e g_e = (K_e − ω²M_e) u_e;
  - arc: K_arc u_e − [M_lump,i g_i; M_lump,j g_j]. The published parts are
    `elastic`, `lump_load` and the total;
  - arc ends in their end-tangent frames: the total is rotated per end before
    any combination (§4);
  - connector: g = K_c B u_e and Bᵀg. There is no inertia term (massless);
  - rods: T = k b·u_e and b·T. There is no inertia term;
  - anchors: the residual (K u − M g) at their DOFs, that is
    (K_cf − ω² M_cf)u. Mass at restrained DOFs and the consistent coupling
    therefore load the supports.

### 3.4 Rigid response (missing-mass correction)

- **Classification.** The modes of a cluster whose lowest frequency is below
  f_r are periodic. Every other mode, extracted or not, is rigid.
- **The static solve.** For each direction:
  - a_r,k = s_k·S_k(f_r);
  - g = a_r,k·(R_k − Σ_periodic Γ_ik φ_i), F_k = M g over all DOFs;
  - solve K_ff u_f = F_f, on the dynamic stiffness.
- **Recovery,** by the same formulas with g:
  - elements: K_e u − M_e g;
  - arc: K_arc u − lump_load;
  - connector and rods: elastic only;
  - anchors: K u − F_k at their DOFs.
- **The rigid load and its shares.** `rigid_load` publishes F_k over all
  DOFs, and its share per mass-bearing constituent: each straight's M_e g_e,
  and each half-arc lump's M_lump g_node. The connector and rods have no
  share. The shares sum to F_k.

### 3.5 Combinations

- **Rules.**
  - SRSS: √(ΣR²).
  - CQC: ρ_ij = 8ζ²(1 + r)r^{3/2} / ((1 − r²)² + 4ζ²r(1 + r)²), with
    r = ω_j/ω_i.
  - GROUPING: groups ascending, each frequency within 10 % of the group's
    lowest, inclusive.
  - TEN_PERCENT: every pair with (f_j − f_i)/f_i ≤ 0.1, inclusive.
  - ABSOLUTE_SUM: Σ|R|.
  - Rigid with periodic: SRSS √(R_p² + R_r²), or ABSOLUTE_SUM R_p + |R_r|.
  - OFF: every extracted mode is periodic.
  - Directional: SRSS; 100-40-40 = max_a(R_a + (2/5)Σ_{b≠a}R_b), with 2/5
    exact; or ABSOLUTE_SUM.
- **Margins.** No cluster-frequency ratio lies within 1e-6 of the 10 %
  boundary. No mode lies within 1e-6 relative of f_r.

### 3.6 Missing-mass disclosure

Per direction:
- T_k;
- the periodic cumulative effective-mass fraction, and the fraction it does
  not represent;
- the same with the correction off;
- the all-finite-mode fraction;
- the fraction moving with the supports, 1 − p_kᵀM_ff⁻¹p_k/T_k.

## 4. Arc end-tangent frames

The U4a frame contract states the chord-to-tangent rotations. This package
restates them here and checks them.

- **The chord frame of the arc.**
  - x_c = (x_j − x_i)/L;
  - y_c = y_reference projected normal to x_c and normalized, toward the bow;
  - z_c = x_c × y_c.
- **The tangent axes,** with s = L/(2R) and c = √(1 − s²):
  - at end i, the rows [[c, s, 0], [s, −c, 0], [0, 0, −1]] in chord
    components;
  - at end j, [[c, −s, 0], [−s, −c, 0], [0, 0, −1]].
- **Second construction.** The generator forms the same frames from the arc's
  own geometry:
  - x is the traversal tangent t = e₃ × e at the end, with e the unit vector
    from the centre to that end;
  - y = −e, toward the centre;
  - z = x × y.

  The two constructions agree to 3.7e-80. Both are orthonormal and
  right-handed to 2.0e-80.
- **The published rows.** For an end action (F_i, M_i, F_j, M_j) in global
  axes, the end-tangent row is (Λ_i F_i, Λ_i M_i, Λ_j F_j, Λ_j M_j), with Λ
  the frame rows. It keeps the node-on-element sign. Every signed
  contribution and rigid response is rotated before it is combined. A
  combined magnitude depends on the axes it is combined in.

## 5. Equilibrium and identities (all computed, `cross_checks`)

- **Nodal balance.** Σ over constituents of the end actions at each DOF is 0
  at free DOFs. At anchor DOFs it equals the reaction.
- **Global equilibrium.**
  - Σ anchor forces = −R_kᵀ M g.
  - Σ anchor moments (x × F + M, about the origin) = −Θ_jᵀ M g, with Θ_j the
    rigid rotation about global axis j.
  - Both hold per mode and per rigid response.
- **Rigid response.**
  - The total reaction along k is −a_r,k·(T_k − Σ_periodic m_eff).
  - The static solve equals a_r,k Σ_{non-periodic finite modes} Γ φ/ω².
- **Effective masses.** Σ m_eff over every finite mode = p_kᵀ M_ff⁻¹ p_k.
- **The half-arc lumps (independent route).** M_lump g_node equals the
  direct integral of the rigid half-arc's inertia under the node's
  acceleration (g_t, g_θ):
  - force density m̄(g_t + g_θ × ρ);
  - moment density ρ × f + (j̄ttᵀ + ī(I − ttᵀ))g_θ;
  - integrated by 48-point Gauss–Legendre with ds = R dψ.

  The geometry, sine, cosine, arcsine, π and the Gauss nodes are the
  generator's own code, not f1's. This holds for every mode and rigid
  response of V-ARC, V-COMB-0 and V-COMB-R.
- **The two lumps against the closed form.** Under unit translation they
  equal the whole arc's weight at the closed-form centroid
  c + R sin(φ/2)/(φ/2)·bisector.
- **The span excluded.** T_k = m̄ × (the straight lengths + Rφ). The span
  D–E is not counted.
- **Connector and rods.** Bᵀg = K_e u, and b·T = K_rod u.

## 6. Fixed-state exclusion of pressure and reference actions (control)

- **The held-state forms.** The static state's connector and joint actions
  are affine in the displacement:
  - g_s(d) = K_c(B d − q_ref);
  - f_s(d) = Bᵀ g_s(d) + P·(−Q_x, 0, +Q_x, 0), with P = p·A_e;
  - T_s(d) = k(b·d − e_ref).
- **What cancels.** At fixed K_c, B, b and geometry, the increment from any
  held state d0 is:
  - Δg = K_c B δ;
  - Δf = Bᵀ K_c B δ;
  - ΔT = k b·δ.

  These do not depend on p, A_e, q_ref and d0. The generator evaluates both
  sides in exact rational arithmetic. It uses every signed contribution and
  rigid response of V-JOINT-R and V-COMB-R as δ, a nonzero held state, and
  the zero state.
- **Mutants as discriminators.** Each adds a forbidden constant to every
  signed contribution and to the rigid response, then recombines under the
  base rules:
  - the held connector force g_s(d0);
  - the thrust P folded into the axial force;
  - the held face action with its thrust.
- **What this control does not establish.** It does not establish an EX
  static equilibrium, the mapping of a pressure region, or a same-run
  nonlinear support state. No pressure geometric stiffness is formed: EX adds
  none (DEL-04-09 CLM-001).

## 7. Replaced-span double ownership (control)

V-COMB-R is re-solved with the replaced span also present as pipe:
- its consistent mass only;
- its stiffness only;
- both.

The base request is then recombined. The mass-only and mass-plus-stiffness
mutants change T_k, frequencies and combined values. The stiffness-only
mutant leaves M and T_k unchanged while changing frequencies and combined
values. The reference publishes no member key for the span.

## 8. Additive arc frame-order control (V-ARC-OBL)

- **Geometry.** Take the unit tangents t1 = (3,4,0)/5 and t2 = (−12,9,20)/25,
  with t1·t2 = 0, and R = 25/32 m. Then B = A + (5/4)t1,
  centre = B + R t2, C = B + R(t1 + t2) and D = C + (25/16)t2. Every
  coordinate is dyadic. The straights are tangent to the arc. The bow
  direction is t1 − t2, normal to the chord t1 + t2; y_reference is its
  integer multiple (27, 11, −20).
- **End-tangent frames.** By §4, x is the traversal tangent, y points toward
  the centre and z = x × y:
  - Λ_B has the rows t1, t2 and t1 × t2 = (16,−12,15)/25;
  - Λ_C has the rows t2, −t1 and the same z.

  They are exact rationals. The generator checks them against the frames used
  in recovery, which come from the U4a rotation rows and from the arc's own
  geometry, both at 80 digits (5.0e-80).
- **The correct magnitude.** For one end component k with frame row λ_k,
  combine the rotated signed contributions λ_k·G_u over the units u (modes,
  clusters, or the rigid response), using rule C: C({λ_k·G_u}_u).
- **The two wrong orders.** Combine each global component first,
  m_c = C({G_u,c}_u), then:
  - a: Σ_c λ_k,c m_c;
  - b: Σ_c |λ_k,c| m_c.
- **Why b can differ only for a non-permutation frame.** For SRSS, CQC (whose
  ρ matrix is positive semidefinite) and ABSOLUTE_SUM, C is a norm of the
  vector of unit values, so the triangle inequality gives
  C({λ_k·G_u}) ≤ Σ_c |λ_k,c| C({G_u,c}), the wrong-b value. Equality holds
  when λ_k has one nonzero entry (a signed permutation), or when the
  contributions are suitably collinear. A non-permutation frame and unequal,
  non-collinear contributions make the difference strict. The witnesses are
  the components where it is large.
- **Not mutants.** Linear operations commute with Λ. Rotating a signed sum (a
  cluster sum, a superposition, or the elastic minus lump split) equals
  summing the rotated parts, so such orders are never flagged.

