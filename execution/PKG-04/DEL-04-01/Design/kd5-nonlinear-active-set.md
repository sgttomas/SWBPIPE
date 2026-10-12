# K-D5 on the converged active set (T4 nonlinear K-D5)

Revision 0. This is the governing design for the T4 unit that the owner's steer names "nonlinear K-D5" ("T4: K-D5 option (d), then U2/O-13, nonlinear K-D5 and U4/U5/U6/U8."). It replaces the sketch in [kd5-member-relative-bound.md](kd5-member-relative-bound.md) §5 ("The next T4 unit: K-D5 on the converged active set"). It applies K-D5 and option (d) unchanged to the linear system that the nonlinear-support loop's converged state solves. It adds one decision: an **active-set certificate**. It changes no product code. The 1e-9 criterion, the 2^-34·S* floor and the 2^-988 guard are unchanged. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

"Certified" has the meaning of kd5 §1: certified against this unit's intended system, under A1 (kd5 §2).

**Result.**

- **One intended system per converged state (§1).** The state is the loop's final support states. Active unilateral supports and sticking friction supports are prescribed rows; inactive supports are free rows with no term; sliding friction supports are free rows that carry a friction force.
  - An explicit-normal sliding force is a formed load term, re-formed exactly at p.
  - A derived-normal sliding force couples to a support reaction, so the system is the loop's affine Coulomb coupling. It is certified by a small exact correction (Stage A, §1.4). The loop's own coupling misses the friction law by 3.6e-10 (sparse) to 1.4e-9 (dense) of the force on the committed models (§5). That is at the criterion, so the loop's binary64 force is never published as certified.
- **The confirmation attempt (§1.5).** One formation-checked factorization of the state's matrix supplies K-D5's factor, Λ and ε_E. K-D5 is then evaluated at the loop's own final displacement u, so published displacements are unchanged. The loop itself is unchanged (S11-K's `solve_binary64` stays).
- **The active-set certificate (§2).** Every support's deciding quantity gets a certified ball from the same corrected displacement u + w + w₂: the contact reaction, the separation, the stick margin μN − |R_t|, the slip sign and the derived normal. Each must have the state's sign strictly, with the whole ball on that side. If all conditions hold, the exact solution of the state's intended system satisfies every contact and Coulomb condition strictly. The published state is then a solution of the intended contact problem with exactly that active set. Uniqueness is a model property for frictionless supports and is not claimed with friction (§2.3).
- **Fail closed (§2.4).** Each failure is a named reason in one sentence. The case is `NUMERICAL_INTEGRITY_SENSITIVE`, and its rows stay today's binary64 loop rows.
- **Publication (§3).** A Passed nonlinear case publishes (d)'s certified member, stress and support rows. It adds certified rows for the nonlinear supports, and the certified friction force. Displacements and loop-evidence rows are unchanged. The unified predicate, the notice and the maximum declarations apply unchanged.
- **Gap supports (§1.2, R1).** The intended system and certificate cover gaps. Revision 0 keeps gap-bearing cases on their present `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` branches, byte for byte, until HELP_HUMAN rules on R1.
- **W1 and pricing (§4).** Five independent refusals in code mean W1 and retained-source recovery never see a nonlinear case. The retained route's behaviour is unchanged. The static pricing chain reaches the nonlinear helper, so the TEXT chain and O-10 rules are re-run.
- **Predicted moves (§5).** The 8 committed case-modes move from Sensitive to Passed, in 5 committed outputs (10 integrity diagnostics). The prediction rests on a probe in which the state's auxiliary symmetric system, run through today's linear producer, is Passed in both modes on both models. Other probe results:
  - certified-margin quantities are 10^2 N and mm-scale;
  - the coupling gain is |c·g| = 1.1e-3;
  - the friction force row moves to the mode-independent coupled-law value (3.6e-10 sparse, 1.4e-9 dense, relative).

## The steer and what this unit supersedes

- **Owner's steer (2026-10-12).** "T4: K-D5 option (d), then U2/O-13, nonlinear K-D5 and U4/U5/U6/U8."
- **kd5 §5's sketch** (HELP_HUMAN's ruling of 2026-10-11, option (d)) is replaced by this design. The sketch's fail-closed rule for the nonlinear-support route stays in force until this unit is implemented. After that, it applies only to gap-bearing cases while R1 keeps them on their present branches; those branches never publish `CHECKS_PASSED`.
- **"ROOT's ruling" on selection.** The adapter's doc comment says: "`selected` is false for an invocation with any nonlinear support: ROOT's ruling that such a case is never selected." kd5 §5 says: "no `FormationCheckedSystem` is built when nonlinear supports exist (ROOT's ruling)". This unit supersedes that ruling for one new **confirmation attempt** after convergence. The loop's iterations keep ROOT's option (c) on the S11-K stop report: the unchanged `solve_binary64`. T3's source pin `kd5_nonlinear_sources_name_no_formation_check_entry_point` (T3 D1 §4.3.1 callers, mutation 32) must be re-scoped with T3 (R2).
- **The sentence retired.** "the member-relative formation check is not available on the nonlinear-support route; member rows are binary64 recoveries" (kd5 §5). The generic §2.3 rule (no `CHECKS_PASSED` without `member_bound`, "on this route") stays.

## 0. The route today (facts from code at a881a65631)

- **Owners.**
  - DEL-04-04 owns the classifier (`core/solver/nonlinear_supports`, unchanged by this unit).
  - DEL-04-04 and DEL-04-07 own the loop (`core/solver/nonlinear_integration`).
  - DEL-04-07 owns the PP integration (`core/product_physics/src/lib.rs`).
  - DEL-04-01 owns FK (`core/solver/frame_kernel`).
  - No Design file exists for the nonlinear route; this file is its K-D5 design.
- **Support kinds and states** (`nonlinear_supports` `classify_support_state`, DEC-067). A support acts on one translational DOF d. In every condition, r = (K·u − f)_d is the support-on-pipe force. The rules below apply when the state is unchanged at convergence.
  - **One-way and lift-off, sense +** (`active_when`/`contact_when` = positive reaction). Engaged stays active iff r > 0. Released becomes active iff u_d < 0.
  - **Sense −.** Engaged stays active iff r < 0. Released becomes active iff u_d > 0.
  - **Gap, closes in +** (clearance g ≥ 0). Engaged stays closed iff r ≤ 0. Open closes iff u_d ≥ g.
  - **Gap, closes in −.** Engaged stays closed iff r ≥ 0. Open closes iff u_d ≤ −g.
  - **Friction** (one tangential DOF; coefficient μ ≥ 0; normal N either explicit, the input `normal_reaction`, or derived as |R_n| from a *linear* restraint DOF n).
    - N ≤ 0 means inactive.
    - Otherwise the support sticks when |r_t| ≤ μ|N|.
    - A sliding support stays sliding iff u_t ≠ 0 and its applied force and reported reaction both oppose u_t (`resolve_sliding_friction_states`).
- **The linearized state** (`active_boundary`, `prescribed_displacement_for_state`). Active one-way and lift-off supports, and sticking friction supports, are prescribed 0. An active gap is prescribed +g or −g. Inactive and sliding supports are free.
  - Sliding applies F = −dir·μ·|N| at the friction DOF. dir is the previous iterate's slip sign.
  - For a derived normal, F solves the affine coupling F = c·R_n(F) with c = −dir·μ·β, where β is the normal's sign. The loop solves this coupling from base and unit-load reaction influences (`solve_iteration_with_sliding_friction_evidence`).
  - The stiffness K includes ground springs. Connectors are refused on this route. Realized arcs enter as `from_macro_element` slots, which are re-formable.
  - The loop's force is the ledger's `AssembledForce`, checked bit for bit (`solve_active_set_frame_with_mode_and_springs_assembled`). The case's closure manifests and primitives therefore exist.
- **Convergence and qualification.** The governed residual is the state-change count. A converged result passes `product_selected_state_is_qualified`:
  - zero changes and contact branches admissible;
  - the classifier reproduces the final state from the final iterate;
  - M03 original-equation equilibrium passes.

  Each iteration's solve is `AssemblyEvidence::solve_binary64`, with no formation check.
- **Gap scrutiny.** `scrutinize_gaps` runs an exact-rational strict-gap proof for gap-only straight-frame bodies (`Qualified`) and is otherwise `Unsupported`. **Every** gap-bearing converged case publishes `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (`lib.rs`, both branches). The NGR02 containment tests (`tests/represented_gap_publication.rs`) pin the nonpassing publication. A gap case therefore never reaches `CHECKS_PASSED`.
- **Publication today.**
  - A converged non-gap case publishes `append_integrity_report` from the last iteration's `StructuralReport` and product-equilibrium report. kd5 §5's sentence then demotes a would-be `CHECKS_PASSED` to Sensitive.
  - Member rows are binary64 recoveries from the loop's u. Support rows come from the loop's reactions K·u − f.
  - Combinations of a nonlinear model are withheld (`NONLINEAR_COMBINATION_REQUIRES_SOLVE`).

## 1. The intended system at the converged active set

### 1.1 Notation

- S is the set of nonlinear supports, with s at DOF d_s, and σ_s is s's final state (`final_states`).
- B is the base prescribed set: linear restraints and imposed displacements, with their values, as today.
- A = {s : σ_s ∈ {Active, Sticking}} is the active set. Each s ∈ A has a prescribed value p_s:
  - 0 for one-way, lift-off and sticking friction;
  - +g_s for a gap that closes in +, and −g_s for one that closes in −.
- L = {s friction : σ_s = Sliding} is the sliding set, split into L_e (explicit normal) and L_d (derived normal). k = |L_d|.
- I = {s : σ_s = Inactive}.
- C = B ∪ {d_s : s ∈ A} is the constrained set, with values u_C. F is the free set.
- K_int and f_int are K-D5's p = 128 re-formations (kd5 §1): frames, realized arcs and springs; the ledger's input and formed terms through the closure. No connectors exist on this route.

### 1.2 Per support kind

| Support, state | Row treatment | Term added | Closure class |
|---|---|---|---|
| Linear restraint, imposed displacement | prescribed, as today | none | input (u_C) |
| Ground spring | stiffness (`FormationSource::springs`) | none | stiffness primitive |
| One-way or lift-off, Active | prescribed, u_d = 0 | none | input (prescribed 0) |
| One-way or lift-off, Inactive | free | none | none |
| Gap, Active (closed) | prescribed, u_d = ±g (binary64 g, exact) | none | input (prescribed ±g) |
| Gap, Inactive (open) | free | none | none |
| Friction, Sticking | prescribed, u_t = 0 | none | input (prescribed 0) |
| Friction, Inactive | free | none | none |
| Friction, Sliding, explicit normal | free | φ_s = −dir_s·μ_s·\|N_s\| at d_s | **formed**: `SlidingFrictionExplicit` |
| Friction, Sliding, derived normal | free | φ_s from the coupling (§1.3) at d_s | **formed**: `SlidingFrictionCoupled` (Stage B); **input** φ⁰_s (Stage A) |
| Friction state `Active` | not a friction state | none | fail closed (§2.4) |

Gaps are fully specified here. Whether gap-bearing cases run the unit is R1 (§8).

### 1.3 The intended system

With the state fixed, the friction law is linear in u. For s ∈ L_d with source DOF n_s:

  R_n(u) = K_int,n·u − f_int,n

This is the source restraint's support-on-pipe reaction. n_s ∈ C, and no friction force acts at n_s.

  φ_s = c_s·R_{n_s}(u),  c_s = −dir_s·μ_s·β_s,  β_s = sign R_{n_s}(u*)

so that φ_s = −dir_s·μ_s·|R_{n_s}(u*)|. dir_s is the loop's final slip direction. β_s is certified in §2.

The intended system of the state is:

  K_int,FF·u_F = f_int,F − K_int,FC·u_C + Σ_{s∈L_e} φ_s·e_{d_s} + Σ_{s∈L_d} c_s·R_{n_s}(u)·e_{d_s} .

- With k = 0 it is symmetric. It is K-D5's linear problem with the extra formed terms φ_s, s ∈ L_e. Each φ_s is the exact product of the binary64 inputs μ_s and N_s with an exact sign. It fits a Wide2 at p, so its re-formation has radius 0.
- With k > 0 the coupling makes the free-row matrix nonsymmetric: K_FF − E·C·K_{N,F}. Here E = [e_{d_s}] and K_{N,F} holds the source rows' free columns. K-D5's symmetric gate and Λ do not cover a nonsymmetric matrix. The coupling cannot be folded into ε_E either: μ·‖K_n‖ ≫ 2^-36·‖K̃_s‖₁, so the Neumann condition would fail. It is therefore solved exactly through the symmetric system (§1.4).

### 1.4 The coupled friction force (Stage A, k > 0)

Let ũ(φ) be the exact solution of the **symmetric** system with the derived forces fixed at φ ∈ ℝ^k. All other terms are as in §1.3. Let z_j = K_int,FF⁻¹·e_{d_j}, and G_ij = K_int,{n_i},F·z_j. These quantities are exactly related:

  ũ(φ) = ũ(φ⁰) + Σ_j (φ_j − φ⁰_j)·z_j ,   R_n(ũ(φ)) = R_n(ũ(φ⁰)) + G·(φ − φ⁰) ,

so the coupled force φ* satisfies

  M·(φ* − φ⁰) = b,   M = I − C·G,   b = C·R_n(ũ(φ⁰)) − φ⁰ .

φ⁰ is the loop's final applied force: binary64, exact as an input.

**Stage A.**

1. **A-1.** Evaluate K-D5 (§1.5) on the symmetric system with φ⁰ as **input terms**, at the loop's u. The certified reaction recovery (`certify_support_reaction`) at each n_i gives a ball N⁰_i ∋ R_{n_i}(ũ(φ⁰)).
2. **A-2.** For each j, solve z̃_j with the confirmation factor, in the same scaled form as w. Form the exact residual e_{d_j} − K_int·z̃_j at p, and correct twice as kd5 §2 steps 2–6 do: w_z, w₂z, then σ_z. The contraction and second-solve rules apply.
   - G_ij ∈ K_int,{n_i},F·(z̃_j + w_z + w₂z) ± ‖K_int,{n_i},F·D‖₁·Λ·σ_z.
   - The midpoint is evaluated at p in ball arithmetic. The radius uses the same triangle form as `certify_support_reaction`, with no load terms.
3. **A-3.** C needs β, and β_i := sign of the ball N⁰_i, which must exclude 0. Otherwise the case fails closed with "friction normal not certified".
4. **A-4. Verified solve.**
   - **k = 1.** δ = b/M in ball arithmetic, and 0 ∉ M is required.
   - **k > 1.** Use Rump's verified-linear-system theorem (Rump, "Verification methods: Rigorous results using floating-point arithmetic", *Acta Numerica* 19, 2010, §10). Take R ≈ mid(M)⁻¹ in binary64 and x̃ = R·mid(b). If ‖I − R·M‖_∞ ≤ θ < 1 over the whole ball M, every M in it is nonsingular, and |δ − x̃| ≤ ‖R·(b − M·x̃)‖_∞ / (1 − θ). All norms are upward.
   - Failure is "friction coupling not certified".
5. **A-5.** φ*_j ∈ φ⁰_j + δ_j. The midpoint is a Wide2 at p, and the radius r_j is upward.

**Stage B.** Evaluate K-D5 at the loop's u on the system whose derived forces are the **formed** terms `SlidingFrictionCoupled`. Each re-forms as the Stage-A ball (midpoint φ*_mid,j, radius r_j). The radius enters ψ at row d_j, exactly as kd5 §2 step 1 carries load enclosure radii. Stage B's u* is then the exact solution of §1.3's coupled system, because the true coupled force lies in the ball. Its w, w₂ and Λσ are the `MemberBoundInputs` that the certified recovery and §2 use.

With k = 0 there is only Stage B.

On the committed models the coupling gain is c·g = −1.1e-3, so M ≈ 1.0011 (§5).

### 1.5 The confirmation attempt

After convergence and the unchanged qualification, the nonlinear crate makes one **confirmation attempt** on the state's system: K (the loop's assembled stiffness) and the state's boundary C with values u_C.

- **The force.** The gate runs once, with the Stage-A force when k > 0 and the Stage-B force when k = 0. Its own solution is discarded. Each K-D5 evaluation (Stage A, the influence solves, Stage B) takes its own `AssembledForce` and `FormationSource`.
  - The Stage-B force is the case's ledger plus one formed term per sliding support. The term's binary64 value is fl(φ*_mid) for derived normals and fl(φ) for explicit normals. Its closure manifest is enumerated from the converged state's sliding supports (§1.6).
  - The Stage-A force is the ledger plus the explicit formed terms and the φ⁰ input terms.
- **The gate.** It runs through the formation-checked gate (`prepare_formation_checked_structural`, or its sparse sibling by mode). It uses the loop's `AssemblyEvidence`, the macro elements as `curved_sources`, and the case's `FormationSource`. Its factor, scale exponents D, gate (r̂, est, ‖K̃_s‖₁), ε_E and Λ are the attempt's own (kd5 §2). They are shared by Stage A, the influence solves and Stage B.
- **K-D5 is evaluated at the loop's final u, not at the attempt's own solution.** The identity e = u* − u = w + w₂ + K⁻¹·s₂* holds for any u. The factor affects only tightness and the contraction (kd5 §2). FK gains an evaluation-at-supplied-u entry for this (§7).
  - The published displacements are therefore the loop's, byte for byte.
  - The loop's evidence rows (free-DOF residuals, deltas, per-support displacement) describe the same u.
  - D5C-1's per-DOF rule judges the loop's u.
  - u must equal u_C bit for bit at every constrained DOF. Otherwise the case fails closed with "prescribed row mismatch".
- **Every kd5 §2.3 fail-closed rule applies to the attempt** (Neumann, contraction, second solve, Λσ finite, load closure, no ledger), and kd5 §2.2 does not arise: the route has no W2.
- An attempt that the gate refuses, or reports `Sensitive`, fails closed with "confirmation attempt" and the gate's reason.
- The loop is not changed. Its iterations keep `solve_binary64` (S11-K, ROOT option (c)). Its evidence and its qualification are unchanged.

### 1.6 Load closure: what the route adds

The ledger's terms keep kd5 §1's classes and manifests unchanged. The route adds only:

| Term | Where | Class | Primitive and re-formation at p | Manifest identity and binding |
|---|---|---|---|---|
| Explicit-normal sliding force | Stage B (and Stage A as a fixed term) | formed | `SlidingFrictionExplicit{support, dof, μ, N, dir}`: −dir·μ·\|N\| exactly | support id, ordinal 0; bits of μ and N, dir and the DOF |
| Derived-normal sliding force, Stage A | Stage A only | input | φ⁰: the loop's binary64 applied force | none (input) |
| Derived-normal sliding force, Stage B | Stage B | formed | `SlidingFrictionCoupled{support, dof, μ, dir, β, source dof, φ*_mid, r}`: the Stage-A ball | support id, ordinal 0; bits of μ, dir, β, the source DOF, and the Stage-A record identity |
| Unit right-hand side e_{d_j} | influence solves | input | exact 1 | none |
| Prescribed values 0, ±g | boundary | input | as K_fc·u_c today | none |

- **The expected manifest.** It is enumerated from `final_states` ∩ friction ∩ `Sliding` independently of the supplied primitives (kd5 §1's complete-summands rule). A missing, duplicate or extra friction primitive fails closed: "load closure: nonlinear sliding friction …".
- **Stage A's input class.** The Stage-A system is an auxiliary system that is certified exactly at its own input φ⁰. φ⁰ is never published, and it is not a ledger value of a formed term of the published system.

## 2. The active-set certificate

### 2.1 Balls

All balls come from Stage B's `MemberBoundInputs` (w, w₂, exponents, Λσ). The midpoint m is evaluated at p, and the radius ρ is upward. lower = m − ρ and upper = m + ρ are decided by exact comparison of the Wide2 midpoint with the binary64 radius, never by rounding.

- **Displacement at a free DOF d:** m = u_d + w_d + w₂,d, an exact sum of three binary64 values, and ρ = 2^{exp_d}·Λσ. This is kd5's remainder with A_r = e_dᵀ, so ‖A_r·D‖₁ = 2^{exp_d}.
- **Reaction at a constrained DOF d:** `certify_support_reaction(d)`: K_int,d·(u + w + w₂) + K_int,dC·u_C − f_int,d. The ball is m with ρ = r + R (the evaluation radius and the remainder).
  - A ground spring k at d is part of K_int's diagonal. Equilibrium at d is K_pipe·u − f = r_c + r_k with r_k = −k·u_d, so (K_int·u − f)_d = r_c: the row is the nonlinear support's own contact force.
  - This is also the loop's reported reaction, since its K includes the springs.
- **Sliding friction force:** φ*_j's Stage-A ball (derived normal), or the exact φ (explicit normal, ρ = 0).
- **Derived normal:** the reaction ball at n_s.

### 2.2 Pass conditions (strict, with the whole ball on the state's side)

| Support, state | Condition |
|---|---|
| One-way or lift-off, sense +, Active | lower(r_d) > 0 |
| One-way or lift-off, sense +, Inactive | lower(u_d) > 0 |
| One-way or lift-off, sense −, Active | upper(r_d) < 0 |
| One-way or lift-off, sense −, Inactive | upper(u_d) < 0 |
| Gap closing in +, Active (u_d = g) | upper(r_d) < 0 |
| Gap closing in +, Inactive | upper(u_d) < g |
| Gap closing in −, Active (u_d = −g) | lower(r_d) > 0 |
| Gap closing in −, Inactive | lower(u_d) > −g |
| Friction, explicit N, Inactive | N ≤ 0 (an input decision; no numerical condition) |
| Friction, explicit N, Sticking | N > 0 and μ·N > \|m_t\| + ρ_t |
| Friction, explicit N, Sliding | N > 0 and slip sign: dir = +1 needs lower(u_t) > 0; dir = −1 needs upper(u_t) < 0 |
| Friction, derived, Inactive | not certifiable (needs R_n = 0 exactly): fail closed |
| Friction, derived, Sticking | 0 ∉ R_n ball and μ·(\|m_n\| − ρ_n) > \|m_t\| + ρ_t |
| Friction, derived, Sliding | 0 ∉ R_n ball, sign = β_s from Stage A, and the slip sign as above |

- The stick inequality is decided exactly. μ·(|m_n| − ρ_n) is formed downward and |m_t| + ρ_t upward.
- With μ = 0, sticking fails by construction; sliding needs only the slip sign, as the loop does.
- The classifier's non-strict boundaries are each the strict condition's closure, so every passing state also passes the classifier: engaged gap r ≤ 0, released one-way u ≥ 0, and stick |r_t| ≤ μN.

### 2.3 What is certified

**Claim (under A1).** Let u* be the exact solution of §1.3's intended system for the published state. Suppose every condition of §2.2 holds and K-D5 passes on the confirmation attempt. Then:

1. every active unilateral support bears on the pipe with its admissible sign, strictly;
2. every inactive unilateral support is strictly separated: for a gap, strictly inside its clearance;
3. every sticking friction support carries |R_t| < μN, strictly;
4. every sliding friction support slips with sign dir, strictly, against a force of magnitude μN, exactly.

So u* together with the state satisfies the intended contact and Coulomb conditions with strict complementarity. The classifier, applied to u*, returns the published state for every support. The published active set is therefore the true active set of the solution u*.

**Argument.** Under A1 each ball encloses its exact value at u*:

- displacements and reactions by kd5 §2's bound (e = w + w₂ + K⁻¹·s₂*, |A_r·K⁻¹·s₂*| ≤ ‖A_r·D‖₁·Λ·σ);
- the friction force by Stage A's verified enclosure, which Stage B carries in ψ;
- the derived normal by the same reaction bound.

Each condition puts the whole ball on the required side, so the exact value is there too. For a sliding support, φ* = −dir·μ·|R_n(u*)| holds exactly by the construction of §1.3–1.4, and β is certified by the sign of R_n(u*). The force therefore opposes the certified slip.

**Uniqueness.**

- **Frictionless states.** These have no friction support, or only inactive friction supports with explicit N ≤ 0. The intended problem is then the minimization of ½uᵀKu − fᵀu over the base-prescribed set with linear inequality constraints. For the exact-element model, K is positive semidefinite (elastic strain energy), so the problem is convex and every point satisfying the complementarity conditions (KKT point) is a global minimizer. Let v be another minimizer. Strict complementarity forces every active constraint to stay tight at v. (v − u*)ᵀK(v − u*) = 0 on the active subspace, where the state's reduced matrix is nonsingular, and therefore v = u*.
  - This is a property of the exact-element model. Positive semidefiniteness of the p-rounded K_int is not separately certified, so uniqueness is stated, not certified.
- **States with active friction.** No uniqueness is claimed. Static Coulomb friction can admit several consistent states. The certificate establishes that the published state is one of them, in DEC-067's path-independent model.

### 2.4 Fail closed

The case is `NUMERICAL_INTEGRITY_SENSITIVE` with one sentence, and nothing is refused. Reasons are checked in this order, and the first reason class that fails is reported. Per-support items name at most six supports in support-id order, each no longer than kd5's per-item bound.

1. `confirmation attempt: <the gate's refusal or Sensitive reason>`.
2. `formation check: <kd5's rendered record>`. This is any kd5 §2.3 unavailability on Stage A, an influence solve or Stage B, or a D5C-1 demotion on Stage B.
3. `prescribed row mismatch`.
4. `friction coupling not certified` (§1.4 A-4, or a nonfinite ball).
5. Per support:
   - `support <id>: contact force sign not certified` (active unilateral);
   - `support <id>: separation not certified` (inactive one-way or lift-off);
   - `support <id>: gap clearance not certified` (inactive gap);
   - `support <id>: stick capacity not certified`;
   - `support <id>: slip direction not certified`;
   - `support <id>: friction normal not certified` (derived ball contains 0, β mismatch, or derived Inactive);
   - `support <id>: friction state active has no certificate`.

**Sentence (draft; claims lint at implementation):**

> "the converged support state is not certified (<reason>); member rows are binary64 recoveries"

**Consequence.**

- The case keeps today's integrity diagnostic: the last iteration's report and product-equilibrium evidence, demoted by `formation_guard::amend_integrity_report` with this sentence.
- It keeps today's binary64 loop rows and routing, byte for byte except the sentence. No notice is emitted.
- These are decided before publication, in the nonlinear crate and FK. They never reach W1 (§4).

## 3. Publication

### 3.1 A Passed nonlinear case

A case is pre-guard Passed on this route when all of these hold:

- the confirmation attempt's gate is Passed;
- K-D5 passes on Stage B (and on Stage A and the influence solves);
- §2.2 holds;
- there is no load-row finding.

`member_bound` is then Stage B's. The integrity diagnostic is today's, built from the last iteration's report with its product-equilibrium evidence. The confirmation attempt's formation-check record is added as its `formation_check` line, as on the linear route.

| Rows | Published as |
|---|---|
| Displacements and rotations, all DOFs | the loop's u, unchanged; judged by D5C-1 |
| Member, section and station actions, stresses, pipe-wall rows, governing maximum and extrema | (d)'s certified recoveries at u + w + w₂ of Stage B (kd5 §3) |
| Linear supports and springs | (d)'s certified rows |
| Nonlinear support, Active or Sticking | certified reaction at d_s, fl(m), in its slot; the other five components exact +0 |
| Nonlinear support, Sliding | fl(φ*_mid) (derived) or fl(φ) (explicit; bit-identical to the loop's value); B = \|v − m\| + r |
| Nonlinear support, Inactive | +0, B = 0 |
| `nonlinear_support_final_reaction` | the same published value as that support's component |
| `nonlinear_support_friction_normal_reaction_derived` | fl(\|m_n\|) of the source's certified reaction, which equals \|its published component\| |
| explicit `friction_normal_reaction` row | unchanged (input) |
| `nonlinear_support_final_displacement`, state codes, iteration count, final residual count, converged flag, delta and free-DOF residual observation rows | unchanged loop evidence |

- **Groups.** Each nonlinear support is a support group of its body, in the force family (kd5 §3.2). It has its component vector and its separately published scalar magnitude. The friction-normal row is not a separate group; it is the source support's row.
- **The predicate.** kd5 §3.3's unified predicate runs unchanged over all groups. A demotion keeps the certified rows, as in (d).
- **The notice.** `STRESS_FREE_WITHIN_NUMERICAL_PRECISION` runs unchanged. Condition 3 includes the nonlinear support groups. An inactive support's zero group is zero-compatible, with a = 0 and B = 0.
- **The maximum.** The maximum declarations are kd5 §3.2's: the certified basis on Passed cases, and the fallback replay otherwise.
- **Combinations.** Combinations of nonlinear models stay withheld.

### 3.2 Criterion

The 1e-9 criterion, RD(1e-9), the floor q̂ ≥ 2^-34·S* and the 2^-988 guard are unchanged. No criterion change is needed. The strict margins in §2.2 are sign conditions on certified balls, not tolerances.

### 3.3 Text (drafts; claims lint at implementation)

- **The §2.4 sentence.**
- **Limitation sentence**, for the formulation basis of runs with nonlinear supports, beside kd5 §3.4's sentences:

  > "On a load case with nonlinear supports reported NUMERICAL_INTEGRITY_CHECKS_PASSED, member actions, stresses and support reactions are recovered at 128-bit precision from a formation-checked solve of the converged support state, each support's state has a sign margin larger than its bound, and a sliding friction force satisfies the friction law of that state to within its bound; displacements and the iteration's residual rows are the iteration's own."

- **Retired:** the nonlinear-route variant of kd5 §2.3's sentence. Its `route` literal (pricing rule KD5A3) becomes the single "on this route".

## 4. W1, pricing and runtime

**W1 never sees a nonlinear case.** Five independent refusals in code at this head:

1. PP `source_eligible(captured, nonlinear, combinations)` is false for nonlinear invocations. No retained-source attempt and no N-5 path exist.
2. `source_recovery.rs` refuses the "nonlinear/contact source family".
3. `retained_memory.rs` D1.6 refuses `support.nonlinear` (`F::Nonlinear`).
4. `retained_product.rs` W1 refuses any built model with nonlinear supports.
5. The exact pressure route refuses nonlinear supports (`exact_admission`).

Consequences:

- Nothing must happen in W1. N-5 cannot fire, and no fail-closed reason here can route to retained-source recovery.
- The confirmation code must call no retained or W1 code.

**Pricing.** The retained route's behaviour is unchanged. The static TEXT and cap chain still reaches shared code (rule L3 already prices `structural_adapter.rs new` in this crate), and `solve_load_case_observed` reaches the nonlinear helper. Requirements:

- Place the confirmation and certificate behind the existing early return of `append_nonlinear_support_loop_results` (no nonlinear supports → return). Under D1.6 their loops then have computed multiplicity 0. They need O-10 rules of the J kind: computed multiplicity, a guard that fails on positive multiplicity, and honest classes.
- Each new text site gets a class: the §2.4 sentence and the support id ≤ 128 B.
- KD5A3's route literal changes.
- Run `price.py run <head>`, then `compare`, then `generate`. Predicted: no positive row changes except TEXT; the implementation measures it. Re-registration stays under the owner's M = 12 GiB with kd5 §6's margins.

**Runtime cost model.** Notation: T_f is one gate and factorization; T_s one right-hand-side solve; T_p one K-D5 re-formation pass over all elements at p.

- Today a converged case costs about I·(T_f + T_s), with I iterations, plus I·(k + 1) dense solves for derived friction, plus the strict-gap exact work.
- The unit adds:

  T_f + T_s + 5·T_p                         (confirmation, Stage B: ρ, s, s₂, ε_E, recovery)
  + [k > 0]·(3·T_p + k·(T_s + 3·T_p) + recoveries of k reactions)   (Stage A)

  Stage A's three passes are its ρ, s and s₂; ε_E and the factor are shared, and reaction recoveries are adjacency-limited.
- **Measured anchors.** On the linear route K-D5 costs a median 5.3× a linear case (kd5 §6). On a release build on a shared host (median of 7 whole-process runs, startup included), the committed demo model takes 21 ms today. Its linear proxy (§5), which adds K-D5 and also solves the combination, takes 33 ms.
- **Predicted.** About 2.5–4× today's case solve time on the committed models (I = 2, k = 1). The implementation measures it on an exclusive host, as kd5 §6 did.

## 5. Committed outputs and predictions

**Probe** (this design's head a881a65631, release build, `openpipestress-runner solve` in both modes). Evidence is in `WT/scratch/t4m/nlkd5-design/` (`proxy.py`, `show.py` and outputs).

- **The proxy.** It is the converged state's **auxiliary symmetric system** as an ordinary linear model:
  - NL-140 (one-way, Active) becomes a linear UY restraint;
  - NL-130-FRIC (sliding, derived normal from S-130 UY) becomes a nodal Z force equal to the loop's φ⁰.
- **Proxy result.** Today's linear producer with K-D5 and (d) publishes `NUMERICAL_INTEGRITY_CHECKS_PASSED` for both cases, in both modes, on both models. A second proxy with the force removed gives g = ∂R_n/∂φ = 0.1110 and c·g = −1.11e-3.

| Model (both identical in structure and loads) | Mode | Case | NL-140 reaction (N) | S-130 \|R_n\| (N; signed −) | u_t (mm) | loop φ⁰ (N) | (μ\|R_n\| − φ⁰)/φ⁰ |
|---|---|---|---|---|---|---|---|
| demo and `preview_physics_invented` | sparse | L-100 | −297.627 | 52.373 | −6.6439 | 0.52373282006 | −3.57e-10 |
| | sparse | L-200 | −98.813 | 26.187 | −3.3220 | 0.26186641003 | −3.57e-10 |
| | dense | L-100 | −297.627 | 52.373 | −6.6439 | 0.52373281913 | +1.42e-9 |
| | dense | L-200 | −98.813 | 26.187 | −3.3220 | 0.26186640956 | +1.42e-9 |

- **Margins** are of order 10^2 N, 10 N and mm. kd5's measured bounds on such cases are of order 1e-12 relative or less. Every §2.2 condition is therefore predicted to hold:
  - NL-140 is sense −, Active, so upper(r) < 0 is needed;
  - R_n excludes 0;
  - u_t < 0 with dir = −1.
- **The loop's friction law misses** by a load-independent 3.6e-10 (sparse) and 1.4e-9 (dense) of the force. This is consistent with binary64 solve error in the derived normal between the loop's coupling solves and its final solve. The dense miss is above the 1e-9 criterion on the friction support's own row.
  - Publishing φ⁰ as certified would be wrong. Stage A is required.
  - Stage A moves the published force to φ* ≈ μ|R_n(u*)|, which is mode-independent to rounding.

**Predicted moves (both models, both cases, both modes: 8 case-modes).**

- `NUMERICAL_INTEGRITY_SENSITIVE` becomes `NUMERICAL_INTEGRITY_CHECKS_PASSED`, and the nonlinear-route sentence is removed.
- Member, stress and support rows become certified: mostly last bits; S-130 and NL-140 reactions change at the level of the loop's solve error.
- The friction force row moves by −3.6e-10 (sparse) or +1.4e-9 (dense) relative, to the same value in both modes.
- Maximum metadata moves per kd5 §3.2. Displacements, state codes and loop-evidence rows are byte-identical. No notice is emitted (the cases are loaded).

**Committed outputs (5; 10 integrity diagnostics; from kd5 §5):**

- `fixtures/product_preview/invented_demo_result_preview_physics_1_{sparse,dense}.json`;
- `fixtures/product_preview/invented_demo_result_legacy_0_1.json`;
- `fixtures/results/preview_physics_invented_{sparse,dense}.json`.

The desktop demo's cases show Passed. Under R1's G1, gap outputs and `represented_gap_publication.rs` are byte-identical. The historical witness files under `validation/witness/` carry no integrity codes and do not move. Every other committed nonlinear output is enumerated by the implementation's regeneration.

**Pins that move:**

- PP `kd5_eight_committed_nonlinear_case_modes_have_named_sensitive_reason` (re-pinned to Passed);
- PP `kd5_nonlinear_route_named_fail_closed_in_both_modes` (re-pinned to the measured outcome of its sticking explicit-normal model);
- the comment in RE `preview_physics_contract.rs` `numerical_standing_progression_on_a_fresh_identity`;
- NI `s11k_tests::kd5_nonlinear_sources_name_no_formation_check_entry_point` (R2);
- pricing KD5A3;
- every PP nonlinear-route integrity assertion: 16 helper uses and `friction_reversal_stop_release_runtime.rs`, enumerated at implementation;
- the committed outputs above.

## 6. Evidence plan

**Independent references** (invented models only; LOOP_INIT's independent basis; no product code as an oracle). A new `validation/references/kd5_nonlinear/` holds a generator, a request bound by SHA-256 and an oracle, following `kd5_member_bound`'s pattern.

- **Generator.** The generator forms each Euler–Bernoulli 12×12 frame element itself, in exact rationals, from the request's binary64 leaves. Sources: Przemieniecki, *Theory of Matrix Structural Analysis*; McGuire, Gallagher and Ziemian, *Matrix Structural Analysis*. FK's frame is Euler–Bernoulli (`local_stiffness`). The generator then assembles and solves exactly (`fractions`).
- **Contact problem.** It solves the contact problem by enumerating support states and keeping the state whose exact solution satisfies every DEC-067 condition. It records that exactly one state qualifies, and its exact margins.
- **What the PP consumer checks.** It checks the request binding and the actual normalized leaves (kd5 §3.3 pattern). It then asserts:
  - the published state equals the oracle's;
  - every published member and support row satisfies |v − v_ref| ≤ B;
  - the friction row is within B of the exact −dir·μ·|R_n|;
  - Passed, or the named reason for the controls.
- **Premise.** The exact-element reference differs from the p-rounded intended system by ~2^-120 relative (kd5 §1's premise). That is negligible against B.

| Id | Model (invented) | Checks |
|---|---|---|
| G-1 | Anchored straight cantilever along a skew axis, tip gap on the transverse DOF (closes in −, clearance g), tip load −P. Closed form: open u = −PL³/(3EI); closed u = −g, contact force P − 3EIg/L³, root moment 3EIg/L² | open and closed cases certified (G2) or certificate-level (G1); a near-degenerate load with P·L³/(3EI) − g inside the ball fails closed with "gap clearance not certified" or "contact force sign not certified" |
| F-1 | Anchored cantilever along X; tip linear UY restraint (normal source) and friction on UX with derived normal; tip loads P_y, P_x. Decoupled (g = 0): R_n = −P_y exactly; stick if \|P_x\| < μ\|P_y\|; slide u_x = (P_x − sign(P_x)·μ\|P_y\|)·L/(EA) | stick and slide certified; near-boundary \|P_x\| ≈ μ\|P_y\| fails closed with "stick capacity not certified"; explicit-normal twin |
| F-2 | L-frame: anchored X leg, vertical Z leg, top shoe with linear UZ restraint (normal source) and friction on UX, μ = 0.5. Strong coupling (\|c·g\| of order 0.1, reported by the generator) | Stage A's verified solve; published friction force within B of the exact coupled force; both modes give the same certified force |
| U-1 | One-way support lifted (Inactive) and pressed (Active) on a short L line | separation and contact sign; a zero-load control (u_d = 0, r = 0 exactly) fails closed (weakly active) |

The committed demo models are natural positives (Passed in both modes, §5); their proxies stand as the reference shape.

**Mutants that must be killed** (each by a named test):

- **Certificate.**
  - Midpoint-only comparison (radius dropped). Killed by G-1, F-1 and U-1 near-degenerate controls whose margin lies in (0, ρ).
  - `≥` in place of `>`. Killed by U-1's exact-zero control.
  - Gap direction swapped. Killed by G-1 closed.
  - Reaction sign convention reversed. Killed by U-1 Active and the demo's NL-140.
  - A spring at d excluded from the contact force. Killed by a gap-plus-spring control.
  - Derived N signed instead of |R_n|, or β not checked. Killed by F-1 with P_y of each sign.
  - Stick capacity without ρ_n or ρ_t. Killed by F-1 near-boundary.
  - Slip sign not checked. Killed by a certificate-function unit test with synthetic balls.
- **Coupling.**
  - Publishing φ⁰ (Stage A skipped). Killed by the demo in dense mode (1.4e-9 > c on the friction row) and by F-2.
  - (I − C·G)⁻¹ dropped (φ* = C·N⁰). Killed by F-2: the error is |c·g|·|δ| against a bound at rounding level.
  - Stage-A radius missing from ψ, or G's radius dropped. Killed by a unit test against a constructed ball case.
  - k > 1 without the θ < 1 check. Killed by a unit test with a singular M ball.
- **Confirmation.**
  - K-D5 evaluated at the attempt's own solution instead of the loop's u. Killed by displacement bytes pinned equal to the loop's.
  - The last iteration's report used as the factor's record. Killed by a source pin and a test whose confirmation gate differs.
  - `selected = false`. Killed by a Passed assertion.
  - Prescribed-row mismatch not detected. Killed by a unit test.
- **Publication.**
  - Nonlinear support rows not certified, or an inactive row nonzero. Killed by the demo and U-1.
  - `nonlinear_support_final_reaction` differing from the component. Killed by an equality pin.
  - Loop-evidence rows altered. Killed by byte pins.
- **Fail closed.** Each §2.4 reason has a demoting test with its named sentence, and the rows are byte-identical to today's.
- **Scope.** Under G1, every gap test and output is byte-identical. W1: a source pin shows the confirmation module calls no retained code.

**Re-run unchanged:** FK `formation_check_tests` (`kd5_*`), PP `s11g_tests`, the kd5 sets (realistic, RV29, W1, RV30) on the linear route. They must be byte-identical: this unit changes no linear-route code path.

## 7. Implementation plan

**FK `structural/formation_check.rs`, `certified_recovery.rs` (DEL-04-01):**

- `LoadFormula::SlidingFrictionExplicit` and `SlidingFrictionCoupled`, with re-formation (exact and ball) and closure identities (§1.6). Producer-side manifests are enumerated by the nonlinear crate.
- An evaluation of a formation-checked attempt at a supplied u. It runs ρ, w, s, w₂, s₂, σ and Λ with the attempt's factor, and returns `MemberBoundInputs` or `FormationCheckUnavailable`. ε_E and Λ are computed once per attempt.
- A certified unit response for e_d: z̃, w_z, w₂z, σ_z with the same rules.
- `certify_displacement(d, u, bound)`. `certify_support_reaction` is reused unchanged.
- Separate formation-checked attempts per right-hand side are acceptable instead of a shared factor. Each then carries its own D, Λ and ε_E, and quantities from different attempts are combined only through their balls.

**NI `nonlinear_integration` (DEL-04-04 and DEL-04-07):**

- `confirm_converged_state(input, springs, solve, force, curved_sources, mode) -> ConfirmedState { formation_check, member_bound, friction: Vec<CertifiedFriction>, finding: Option<ActiveSetFinding> }`. It covers Stage A, Stage B and the pure §2.2 predicate `active_set_certificate(...)`, which is unit-testable on synthetic balls.
- The k×k verified solve.
- The loop, the classifier and `product_selected_state_is_qualified` are unchanged.
- The S11-K pin is re-scoped (R2): the loop's functions contain no formation entry, and `confirm_converged_state` contains exactly one.

**PP `lib.rs` (DEL-04-07):**

- After qualification (inside the nonlinear helper's non-empty branch), call the confirmation for non-gap states (G1).
- On Passed: build `member_bound` from Stage B and append the formation-check line, then run (d)'s recovery loop with the loop's u. Extend `certified_publication::support` to nonlinear supports (§3.1). Publish the nonlinear scalar rows from the same values.
- Otherwise: apply the §2.4 sentence and keep today's rows.
- Remove the nonlinear variant of the no-`member_bound` sentence. Add the limitation sentence.

**Dependency conditions.** DEL-04-07's need on DEL-04-01 gains the friction primitives, the supplied-u evaluation and the unit response. The implementing PR updates the condition text. DEL-04-04's classifier is unchanged.

**Measured by the implementation, not assumed:**

- the margins and B/c on the 8 case-modes and on G-1, F-1, F-2 and U-1;
- the friction-row move;
- runtime (§4);
- the pricing comparison;
- the fail-closed rate on every committed and test nonlinear model.

## 8. Rulings needed

HELP_HUMAN holds technical delegation; criteria remain the owner's. None of these is a criterion change.

- **R1. Gap supports.**
  - **G1 (this revision's default):** gap-bearing cases keep both `RECOVERY_BASIS_UNQUALIFIED` branches and the NGR02 containment unchanged. Consequence: no gap case can be Passed, and no gap output moves.
  - **G2:** gap-bearing cases run the confirmation. K-D5 is evaluated at the published u (the selected exact-ratio projection on `Qualified`, the loop's u on `Unsupported`), and §2.2's gap rows apply. A certified case publishes `CHECKS_PASSED` with certified rows, replacing the containment that "mixed recovery remains numerically unresolved". Consequence: the NGR02 tests and every converged gap output move; G-1 becomes a publication test.
  - Recommendation: G1 now, G2 as a follow-on after the non-gap evidence, because G2 changes a separately contained publication.
- **R2. Selection on the nonlinear route.** Confirm that the owner's steer supersedes "ROOT's ruling" (nonlinear invocations are never selected) for the single confirmation attempt, with the loop's binary64 solves unchanged. T3's agreement is needed to re-scope its source pin (T3 D1 §4.3.1, mutation 32). Without R2 the unit cannot proceed, and kd5 §5's fail-closed rule stands.
- **R3. Weakly active supports.** These are supports at zero reaction and zero separation, or exactly at stick capacity.
  - Default: fail closed, which is fail-safe.
  - The alternative, accepting a state within a bounded perturbation of a degenerate one, would add a tolerance. That is the owner's choice and is not proposed.
- **R4. Published friction force (property).** On a Passed case it is the certified coupled-law force, not the loop's binary64 value. On the committed models it moves by up to 1.4e-9 relative and becomes mode-independent. Accept as a property, as kd5 §3.4's repair was.
- **R5. Claims wording.** Existence is certified. Uniqueness is stated for frictionless states as a model property and is not claimed with friction (§2.3). Accept this wording for surface text through `docs/claims_registry.md`.

## Open items

- **The loop's own coupling precision.** It misses the friction law by up to 1.4e-9 relative, which is a loop-side numerical issue. This unit makes publication independent of it. Improving the loop is outside this unit and DEC-046's convergence policy is unchanged.
- **The nonlinear route has no Design file of its own** (DEL-04-04 has no `Design/`). This file is the route's K-D5 design.
- **Explicit-normal sliding with N ≤ 0** cannot occur, because the classifier makes it Inactive. It is noted for completeness.
