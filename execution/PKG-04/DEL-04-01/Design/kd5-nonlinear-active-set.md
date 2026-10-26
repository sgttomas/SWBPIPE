# K-D5 on the converged active set (T4 nonlinear K-D5)

Revision 1. Revision 0 (9c150de821) is the governing design for the T4 unit that the owner's steer names "nonlinear K-D5" ("T4: K-D5 option (d), then U2/O-13, nonlinear K-D5 and U4/U5/U6/U8."); it replaced the sketch in [kd5-member-relative-bound.md](kd5-member-relative-bound.md) §5. Revision 1 applies HELP_HUMAN's rulings R1–R5 of 2026-10-12 and the T4 manager's rulings of the same day (see "The rulings"), folds in the independent design review of revision 0 (findings B1, I1–I5 and M1–M6), and fixes the implementation's choices: one shared confirmation factor, the influence solve as an unjudged K-D5 evaluation, the Stage-A ball published as a binary64 midpoint with a widened radius, k = 1 only, and the order of the named reasons. The 1e-9 criterion, the 2^-34·S* floor, the 2^-988 guard, the notice and the maximum declarations are unchanged. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

Revision 2 carries imposed motion on the pressure-free route into this unit (W-16 stage 2; DEL-04-03 Design [pf-nonlinear-imposed-motion.md](../../DEL-04-03/Design/pf-nonlinear-imposed-motion.md) §2). R-a: rigid anchors and restraints carry the case's motion as base prescribed values g_B (§0 M1, §1.1, §1.2, §1.4, §1.6), and §2.4 item 2 gains the intended-map check. R-b: a primitive on a nonlinear support's own DOF moves that support's ground by g_s; its state is measured on δ_s = u_d − g_s, an active or sticking state prescribes p_s, and §2.2's strict conditions shift by g_s (§1.1, §1.2, §2.2, §2.3, §9 M04). Both are built.

"Certified" has the meaning of kd5 §1: certified against this unit's intended system, under A1 (kd5 §2). It is a technical term of this Design and of K-D5; product text does not use it (§3.3).

**Result.**

- **One intended system per converged state (§1).** The state is the loop's final support states. Active unilateral supports and sticking friction supports are prescribed rows; inactive supports are free rows with no term; sliding friction supports with μ > 0 are free rows that carry a friction force. Translational and rotational one-way and lift-off supports are covered alike (moment family, separations in rad). Rotational friction is refused at validation.
  - An explicit-normal sliding force is a formed load term, re-formed exactly at p as −dir·μ·|N|.
  - A derived-normal sliding force couples to a base restraint's reaction. With one such support (k = 1) it is certified by a small exact correction (Stage A, §1.4). k > 1 fails closed by name.
- **The confirmation attempt (§1.5).** Exactly one formation-checked factorization of the state's matrix after convergence (R2). Its gate, factor, scale exponents D, ε_E and Λ serve every K-D5 evaluation of the unit. K-D5 is evaluated at the loop's own final displacement u, so published displacements are unchanged. The loop is unchanged (S11-K's `solve_binary64` stays in every iteration).
- **The active-set certificate (§2).** Every support's deciding quantity gets a certified ball from Stage B's corrected displacement u + w + w₂: the contact reaction, the separation, the stick margin μN − |R_t|, the slip sign and the derived normal. Each must have the state's sign strictly, with the whole ball on that side. Weakly active supports fail closed (R3).
- **Fail closed (§2.4).** Each failure is a named reason, decided in a fixed order. The case is `NUMERICAL_INTEGRITY_SENSITIVE`, and its rows stay today's binary64 loop rows.
- **Publication (§3).** A Passed nonlinear case publishes (d)'s certified member, stress and support rows, certified rows for the nonlinear supports (R4: the sliding force moves to the certified coupled value; inactive supports become exact +0; active and sticking reactions become certified), and the unchanged loop displacements and evidence rows.
- **Gap supports (R1).** Gap-bearing cases keep their present non-passing `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` branches byte for byte; NGR02 containment applies now. No confirmation runs on them.
- **W1 and pricing (§4).** Four refusals in code mean W1 and retained-source recovery never see a nonlinear case; the exact route refuses nonlinear supports too. The retained route's behaviour is unchanged. The static pricing chain reaches the nonlinear helper, so the TEXT chain and O-10 rules are re-run.
- **Declared moves (§5).** The 8 committed case-modes move from Sensitive to Passed, in 5 committed outputs (10 integrity diagnostics), together with the desktop demo standing. Their nonlinear support rows move as R4 declares.

## The rulings (2026-10-12) and what they settle

**HELP_HUMAN, under the owner's delegation:**

- **R2, selection on the nonlinear route.** The owner's steer "T4: K-D5 option (d), then U2/O-13, nonlinear K-D5 and U4/U5/U6/U8", together with kd5 §5's named next unit, supersedes the earlier ROOT ruling (nonlinear invocations are never formation-checked) for exactly one confirmation factorization at the converged active set. The iteration loop stays unchecked. T3's source pin `kd5_nonlinear_sources_name_no_formation_check_entry_point` is re-scoped to: no formation-check entry inside the nonlinear iteration, and exactly one source-qualified confirmation entry after convergence. A mutant that adds a second entry or an in-loop entry is kept (§6).
- **R4, published friction force (property).** The published sliding-friction force moves to the certified coupled value. It is a stated property, declared like kd5 §3.4, with its blast radius declared before re-pinning (§3.4, §5). The criterion is unchanged. The other changed nonlinear rows are declared with it: inactive supports become exact +0 instead of binary64 residual noise, and active and sticking reactions become certified recoveries.
- **R5, claims wording.** Accepted subject to the claim fence. "Certified" keeps its K-D5 technical sense (against K-D5's intended system). Every surface sentence is checked against `docs/claims_registry.md` (F-PIP-2; claims taxonomy per DEC-081); product sentences use "established" and "confirmed", never "certified" (§3.3). Uniqueness is stated only for frictionless states (§2.3).
- **R1, gap supports.** Gaps keep their current non-passing branches; NGR02 containment applies now. This is revision 0's G1. G2 remains a later option.
- **R3, weakly active supports.** They fail closed, with no tolerance.

**The T4 manager:**

- **Rotational friction** is refused by name at validation (`NONLINEAR_ROTATIONAL_FRICTION_UNSUPPORTED`, blocking). It is unit-inconsistent: μ·N, a force, would be applied as a moment.
- **A derived friction normal whose source is not a base-restrained DOF** fails closed by name before Stage A. Admission is unchanged.
- **Coupled derived-normal friction with k > 1** fails closed by name for now. All committed cases have k = 1. The Rump verified solve of revision 0's A-4 is dropped from this unit and recorded as a later extension (Open items).

## The steer and what this unit supersedes

- **Owner's steer (2026-10-12).** "T4: K-D5 option (d), then U2/O-13, nonlinear K-D5 and U4/U5/U6/U8."
- **kd5 §5's sketch** (HELP_HUMAN's ruling of 2026-10-11, option (d)) is replaced by this design. Its fail-closed rule for the nonlinear-support route is retired: on a nonlinear case the only no-`member_bound` sentence is this unit's §2.4 sentence. Gap-bearing cases keep their `RECOVERY_BASIS_UNQUALIFIED` branches, which never publish `CHECKS_PASSED`, so the rule's other variant ("on this route") stays for the other routes only.
- **"ROOT's ruling" on selection** (the adapter's doc comment: "`selected` is false for an invocation with any nonlinear support"; kd5 §5: "no `FormationCheckedSystem` is built when nonlinear supports exist") is superseded by R2 for the single confirmation attempt. The linear route's `selected` flag is unchanged; the confirmation has its own entry (§1.5, §7).
- **The sentence retired.** "the member-relative formation check is not available on the nonlinear-support route; member rows are binary64 recoveries" (kd5 §5).

## 0. The route today (facts from code at a881a65631)

- **Owners.** DEL-04-04 owns the classifier (`core/solver/nonlinear_supports`, unchanged by this unit). DEL-04-04 and DEL-04-07 own the loop (`core/solver/nonlinear_integration`, NI). DEL-04-07 owns the PP integration (`core/product_physics/src/lib.rs`). DEL-04-01 owns FK (`core/solver/frame_kernel`). No Design file exists for the nonlinear route; this file is its K-D5 design.
- **Support kinds and states** (`nonlinear_supports` `classify_support_state`, DEC-067). A support acts on one DOF d, translational or rotational. r = (K·u − f)_d is the support-on-pipe force, or moment at a rotational DOF.
  - **One-way and lift-off, sense +.** Engaged stays active iff r > 0. Released becomes active iff u_d < 0.
  - **Sense −.** Engaged stays active iff r < 0. Released becomes active iff u_d > 0.
  - **Gap, closes in + (clearance g ≥ 0).** Engaged stays closed iff r ≤ 0. Open closes iff u_d ≥ g. Closes in −: engaged iff r ≥ 0; open closes iff u_d ≤ −g.
  - **Friction** (one tangential DOF; μ ≥ 0; N explicit (`normal_reaction`) or derived as |R_n| from a linear support DOF n). N ≤ 0 means inactive. Otherwise the raw classifier returns Sticking when |r_t| ≤ μ|N|, including the boundary |r_t| = μN. A sliding support stays sliding only through `resolve_sliding_friction_states` (NI lib.rs:1559–1658): u_t ≠ 0 and, for μN > 0, its applied force and reported reaction both oppose u_t; for μN = 0, no applied force and u_t ≠ 0.
- **Rotational supports (review B1).** Validation refuses only rotational gaps (`validation.rs`, `NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED`). `build_nonlinear_supports` admits one-way, lift-off and friction supports on any DOF, and PP publishes their rows in N·m and rad (lib.rs, the per-support state, displacement and reaction rows). Rotational one-way and lift-off supports are covered by this unit in the moment family. Rotational friction is refused at validation from this unit on (manager's ruling).
- **Derived normal sources (review I2).** `derived_friction_normal_source` requires a linear (non-nonlinear) support record with a restraint on the named translational DOF. A spring-family support declares its acting axis as a restraint (`validation.rs`, `SUPPORT_SPRING_AXIS_CONFLICT`) but is not a rigid restraint, so the source DOF can be a free DOF carrying a spring. Admission is unchanged; the confirmation fails closed on such a source (§2.4).
- **μ = 0 (review I3).** The loop drops zero forces (`if force != 0.0`, lib.rs:1359 and 1487) and excludes μ = 0 from the derived coupling (lib.rs:1368). A sliding support with μ = 0 stays sliding when u_t ≠ 0 and no force is applied (lib.rs:1623–1631).
- **The linearized state** (`active_boundary`, `prescribed_displacement_for_state`). Active one-way and lift-off supports and sticking friction supports are prescribed 0. An active gap is prescribed ±g. Inactive and sliding supports are free.
  - Sliding applies F = −dir·μ·|N| at the friction DOF, dir being the previous iterate's slip sign. A derived F solves the affine coupling F = c·R_n(F), c = −dir·μ·β, from base and unit-load reaction influences (`solve_iteration_with_sliding_friction_evidence`). Those coupling solves are hard-coded `DenseScrutiny` in both modes (lib.rs:1382–1388, 1415–1421), so φ⁰ is mode-independent. The final solve uses the requested mode.
  - The reported reaction vector is `reported_reactions`: fl(K·u − f − φ⁰) + φ⁰ (lib.rs:1505, 1731–1741). At a sliding DOF it is φ⁰ plus the binary64 residual of that free row.
  - K includes ground springs. Connectors are refused on this route. Realized arcs enter as `from_macro_element` slots, which are re-formable.
  - The loop's force is the ledger's `AssembledForce`, checked bit for bit (`solve_active_set_frame_with_mode_and_springs_assembled`).
- **Prescribed values (review M1; revision 2, W-16 stage 2, DEL-04-03 Design `pf-nonlinear-imposed-motion.md`).** The loop prescribes each base-restrained DOF at its case value g_b, which is 0 unless a pressure-free case authors a support-targeted primitive on a rigid restraint (R-a). A primitive on a nonlinear support's own DOF moves that support's ground by g_s (R-b): its state is measured on u_d − g_s and an active or sticking state prescribes g_s (g_s ± c for a gap). The confirmation checks the published u against the state's prescribed map, and that map against the case's motion, bit for bit (§2.4). The load-state route is exact-route only, where nonlinear supports are refused.
- **Convergence and qualification.** The governed residual is the state-change count. A converged result passes `product_selected_state_is_qualified`: zero changes and contact branches admissible; the classifier reproduces the non-friction final states from the final iterate; M03 original-equation equilibrium passes. Each iteration's solve is `AssemblyEvidence::solve_binary64`, with no formation check. A case that does not converge, or fails qualification, publishes `SOLVER_SYSTEM_BLOCKED` (or the integrity failure) and never reaches the confirmation (PP `append_nonlinear_support_loop_results`, and the `selected-state-unavailable` return in `solve_load_case_observed`).
- **Gap scrutiny.** Every gap-bearing converged case publishes `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (both branches). The NGR02 containment tests pin it.
- **Publication today.** A converged non-gap case publishes `append_integrity_report` from the last iteration's `StructuralReport` and its product-equilibrium report (PP `solve_load_case_observed`, the nonlinear branch). kd5 §5's sentence then demotes a would-be `CHECKS_PASSED` to Sensitive. Member rows are binary64 recoveries from the loop's u. Support rows come from the loop's reported reactions. Combinations of a nonlinear model are withheld.

## 1. The intended system at the converged active set

### 1.1 Notation

- S is the set of nonlinear supports, s at DOF d_s, σ_s its final state (`final_states`).
- B is the base prescribed set: the linear restraints, with values g_B, each 0 unless the case authors motion of that rigid restraint (§0, M1; revision 2).
- Each support s has a ground offset g_s, 0 unless the case moves it (revision 2, R-b), and δ_s = u_{d_s} − g_s.
- A = {s : σ_s ∈ {Active, Sticking}}, each with prescribed value p_s: g_s for one-way, lift-off and sticking friction (g_s + σc for a gap closing in σ with clearance c, not run under R1).
- L = {s friction : σ_s = Sliding, μ_s > 0} is the sliding set, split into L_e (explicit normal) and L_d (derived normal). k = |L_d|. A sliding support with μ = 0 carries no force and is not in L (review I3).
- I = {s : σ_s = Inactive}.
- C = B ∪ {d_s : s ∈ A}, with values u_C = (g_B, p_A). F is the free set.
- K_int and f_int are K-D5's p = 128 re-formations (kd5 §1). No connectors exist on this route.
- dir_s, for s ∈ L, is the sign opposite the loop's final applied force at d_s; the closure checks it against the sign of the loop's final δ_t = u_t − g_t (§1.6). For a μ = 0 sliding support, dir_s is the sign of δ_t. For finite binary64 operands fl(u − g) has the exact sign of u − g.

### 1.2 Per support kind

| Support, state | Row treatment | Term added | Closure class |
|---|---|---|---|
| Linear restraint | prescribed g_b (0 unless the case moves it; revision 2) | none | input (u_C) |
| Ground spring | stiffness (`FormationSource::springs`) | none | stiffness primitive |
| One-way or lift-off (translational or rotational), Active | prescribed, u_d = g_s | none | input |
| One-way or lift-off, Inactive | free | none | none |
| Gap | not run (R1) | — | — |
| Friction, Sticking | prescribed, u_t = g_t | none | input |
| Friction, Inactive | free | none | none |
| Friction, Sliding, μ = 0 | free | none | none |
| Friction, Sliding, μ > 0, explicit normal | free | φ_s = −dir_s·μ_s·\|N_s\| at d_s | **formed**: `SlidingFrictionExplicit` |
| Friction, Sliding, μ > 0, derived normal (k = 1) | free | φ_s from the coupling (§1.3) at d_s | **formed**: `SlidingFrictionCoupled` (Stage B); **input** φ⁰_s (Stage A) |
| Friction state `Active` | not a friction state | — | fail closed (§2.4) |
| Rotational friction | refused at validation | — | — |

### 1.3 The intended system

For s ∈ L_d with source DOF n_s ∈ B (required; §2.4), R_n(u) = K_int,n·u − f_int,n is the source restraint's support-on-pipe reaction, and

  φ_s = c_s·R_{n_s}(u),  c_s = −dir_s·μ_s·β_s,  β_s = sign R_{n_s}(u*),

so that φ_s = −dir_s·μ_s·|R_{n_s}(u*)|. β_s is certified in §2. The intended system of the state is

  K_int,FF·u_F = f_int,F − K_int,FC·u_C + Σ_{s∈L_e} φ_s·e_{d_s} + Σ_{s∈L_d} c_s·R_{n_s}(u)·e_{d_s} .

- With k = 0 it is symmetric: K-D5's linear problem with extra formed terms. Each explicit φ_s is the exact product of the binary64 inputs μ_s and N_s with an exact sign. It fits a Wide2 at p, so its re-formation has radius 0. It is not fl(μ·N).
- With k = 1 the coupling makes the free-row matrix nonsymmetric. It is solved exactly through the symmetric system (§1.4).

### 1.4 The coupled friction force (Stage A, k = 1)

Let ũ(φ) be the exact solution of the symmetric system with the derived force fixed at φ, z = K_int,FF⁻¹·e_d (z_C = 0) and g = K_int,n,F·z. Exactly, R_n(ũ(φ)) = R_n(ũ(φ⁰)) + g·(φ − φ⁰), because the difference of the two states satisfies zero prescribed values; this holds with u_C carrying the case's motion (revision 2). The influence system stays at zero prescribed values: one that carried g_C would be a mutant, and W-16 stage 2's A-3 benchmark kills it. So the coupled force φ* satisfies

  M·(φ* − φ⁰) = b,  M = 1 − c·g,  b = c·R_n(ũ(φ⁰)) − φ⁰ .

φ⁰ is the loop's final applied force: binary64, exact as an input.

1. **A-1.** K-D5 on the symmetric system with φ⁰ as an **input term**, evaluated at the loop's u, judged by D5C-1. `certify_support_reaction` at n gives a ball N⁰ ∋ R_n(ũ(φ⁰)).
2. **A-2 (the influence solve).** K-D5 on the unit system (force e_d as an input term, every prescribed value 0), evaluated at z = 0. Its w is the factor's solve of e_d and w₂ its correction; the identity e = w + w₂ + K⁻¹·s₂ holds at any evaluation point, so z ∈ w + w₂ ± (kd5 §2's remainder). `certify_support_reaction` at n on that system gives the ball g. D5C-1 does not apply: nothing of this system is published, and its per-row rule would judge every nonzero w against a zero displacement. Contraction, second solve, Λσ finite and Neumann apply.
3. **A-3.** β := the sign of N⁰, which must exclude 0; otherwise "support <id>: friction normal not established".
4. **A-4.** δ = b/M in ball arithmetic; 0 ∉ M is required, and every ball must be finite. Otherwise "friction coupling not established".
5. **A-5.** φ* ∈ φ⁰ + δ, a ball (m, r) at p. It is published as the binary64 midpoint v = fl(m) with the widened radius r' = r + |v − m|, rounded upward. The ball (v, r') contains φ*.

**Stage B.** K-D5 at the loop's u on the system whose derived force is the **formed** term `SlidingFrictionCoupled`, which re-forms as the ball (v, r'). The radius enters ψ at row d, as kd5 §2 step 1 carries load enclosure radii. Stage B's bound therefore covers the exact solution u* of §1.3's coupled system. Its `MemberBoundInputs` are those the certified recovery and §2 use.

With k = 0 there is only Stage B. With k > 1 the case fails closed before Stage A.

### 1.5 The confirmation attempt

After convergence and the unchanged qualification, NI's `confirm_converged_state` makes the unit's only formation-checked factorization, through the adapter's single entry `AssemblyEvidence::confirm_with_formation_check`:

- **The gate.** The loop's assembled stiffness K (incl. springs and arcs), the state's boundary C with values u_C, the loop's `AssemblyEvidence`, and the force of Stage B (k = 0) or of Stage A (k = 1). It is prepared and gated exactly as the linear route's typed solve (`prepare_assembled_structural`), factored by mode (dense Cholesky, or the profile LDLᵀ in sparse mode) and finished by `finish_confirmation`. A refused or `Sensitive` gate fails closed: "confirmation attempt: …". The gate's own solution is discarded.
- **The evaluations.** `ConfirmationAttempt::evaluate_at(system, u, judged)` runs kd5 §2 (closure, ρ, w, s, w₂, s₂, σ, ε_E, Λ) on any system with the factor's stiffness, free map and contributions, at a supplied u, with the factor's solves, scale exponents, prepared matrix and Hager estimate. ε_E is formed once per attempt and reused. The unit runs at most three evaluations: Stage A (judged), the influence system (unjudged, at 0) and Stage B (judged). Every kd5 §2.3 fail-closed rule applies to each; kd5 §2.2 does not arise (no W2 on this route).
- **K-D5 is evaluated at the loop's final u, not at the gate's own solution.** The identity e = u* − u = w + w₂ + K⁻¹·s₂* holds for any u. The published displacements are therefore the loop's, byte for byte; the loop's evidence rows describe the same u; D5C-1's per-DOF rule judges it.
- **The loop is not changed.** Its iterations keep `solve_binary64` (S11-K, ROOT option (c)). Its evidence and qualification are unchanged. The confirmation calls no retained or W1 code.

### 1.6 Load closure: what the route adds

The ledger's terms keep kd5 §1's classes and manifests. The route appends terms after the ledger's own (`AssembledForce::with_appended`, which re-nets every DOF exactly), so every original term keeps its index:

| Term | Where | Class | Primitive and re-formation at p | Manifest binding |
|---|---|---|---|---|
| Explicit-normal sliding force | Stage A and Stage B | formed | `SlidingFrictionExplicit{global_dof, coefficient, normal, direction}`: −dir·μ·\|N\| exactly, radius 0 | identity (`nonlinear_sliding_friction`, support id, 0); bits of DOF, μ, N, dir |
| Derived-normal sliding force, Stage A | Stage A | input | φ⁰ | none |
| Derived-normal sliding force, Stage B | Stage B | formed | `SlidingFrictionCoupled{global_dof, coefficient, direction, beta, source_dof, midpoint, radius}`: the ball (v, r') | identity as above; bits of DOF, μ, dir, β, source DOF, v and r' |
| Unit right-hand side e_d | influence system | input | exact 1 | none |
| Prescribed values u_C = (g_B, p_A) | boundary | input | as K_fc·u_c today | none |

- **The expected manifest** is enumerated from `final_states` ∩ friction ∩ `Sliding` ∩ μ > 0, with the support's own μ and normal input, and dir from the sign of the loop's final u_t, independently of the supplied primitives (kd5 §1's complete-summands rule). The primitive takes dir from the loop's final applied force. A missing, duplicate, extra or mismatched friction primitive fails closed through the closure: "formation check: … load closure: …".
- **Stage A's input class.** The Stage-A system is an auxiliary system certified exactly at its own input φ⁰. φ⁰ is never published.

## 2. The active-set certificate

### 2.1 Balls

All balls come from Stage B's `MemberBoundInputs` (w, w₂, exponents, Λσ) and its source and force. The midpoint m is evaluated at p and the radius ρ (evaluation radius plus remainder) is upward. A sign condition is decided by exact comparison of the Wide2 midpoint with the binary64 radius, never by rounding.

- **Displacement or rotation at a free DOF d:** m = u_d + w_d + w₂,d (an exact sum), ρ = 2^{exp_d}·Λσ (`certify_displacement`).
- **Reaction at a constrained DOF d:** `certify_support_reaction(d)`: K_int,d·(u + w + w₂) − f_int,d, ρ = r + R. A ground spring at d is part of K_int's diagonal, so the row is the nonlinear support's own contact force (or moment), which is also the loop's reported reaction.
- **Sliding friction force:** the Stage-A ball (v, r') (derived), or the exact φ (explicit, ρ = 0).
- **Derived normal:** the reaction ball at n.

### 2.2 Pass conditions (strict, the whole ball on the state's side)

| Support, state | Condition |
|---|---|
| One-way or lift-off, sense +, Active | lower(r_d) > 0 |
| One-way or lift-off, sense +, Inactive | lower(u_d) > g_s |
| One-way or lift-off, sense −, Active | upper(r_d) < 0 |
| One-way or lift-off, sense −, Inactive | upper(u_d) < g_s |
| Gap (kept for G2; not run under R1) | closes in +: Active upper(r_d) < 0, Inactive upper(u_d) < g_s + c; closes in −: Active lower(r_d) > 0, Inactive lower(u_d) > g_s − c |
| Friction, explicit N, Inactive | N ≤ 0 (an input decision) |
| Friction, explicit N, Sticking | N > 0 and μ·N > \|m_t\| + ρ_t |
| Friction, explicit N, Sliding, μ > 0 | N > 0 and the slip sign: dir = +1 needs lower(u_t) > g_t; dir = −1 needs upper(u_t) < g_t |
| Friction, Sliding, μ = 0 | the normal row as for μ > 0 (explicit: N > 0; derived: 0 ∉ R_n ball) and the slip sign, with dir = sign δ_t; no β |
| Friction, derived, Inactive | not established (needs R_n = 0 exactly): fail closed |
| Friction, derived, Sticking | 0 ∉ R_n ball and μ·(\|m_n\| − ρ_n) > \|m_t\| + ρ_t |
| Friction, derived, Sliding, μ > 0 | 0 ∉ R_n ball, sign = β from Stage A, and the slip sign |

- **Revision 2 (R-b).** The ground g_s is 0 unless the case moves it. g_s is an exact binary64 threshold, so each shifted comparison stays exact; every reaction, friction-normal and stick-margin condition is unchanged.
- Every sign and the stick inequality are decided exactly, not in binary64: the ball's midpoint (a double-binary64 value) and radius enter an exact rational sum, and the sign of μ·(|m_n| − ρ_n) − (|m_t| + ρ_t) (derived) or μ·N − (|m_t| + ρ_t) (explicit) is the sign of that exact sum. The comparison is strict. A rounded-to-nearest comparison is a killed mutant (§9).
- With μ = 0, sticking fails by construction.
- **Weakly active supports (R3).** A support at zero reaction and zero separation, or exactly at stick capacity, has a ball touching the boundary and fails closed. No tolerance is added.
- The classifier's non-strict boundaries are each the strict condition's closure (engaged one-way r > 0 is already strict; released u ≥ 0; stick |r_t| ≤ μN).

### 2.3 What is certified

**Claim (under A1).** Let u* be the exact solution of §1.3's intended system for the published state. If every condition of §2.2 holds and K-D5 passes on every evaluation of the confirmation attempt, then:

1. every active unilateral support bears on the pipe with its admissible sign, strictly;
2. every inactive unilateral support is strictly separated from its ground (δ_s on its side of 0, revision 2);
3. every sticking friction support carries |R_t| < μN, strictly;
4. every sliding friction support with μ > 0 slips relative to its ground (δ_t) with sign dir, strictly, against a force of magnitude μN exactly; a μ = 0 sliding support slips with a nonzero δ_t.

So u* with the state satisfies the intended contact and Coulomb conditions with strict complementarity. For every non-friction support the classifier, applied to u*, returns the published state. For friction supports the raw classifier returns Sticking at the boundary |r_t| = μN; sliding is the state that `resolve_sliding_friction_states` keeps when the applied force and the reported reaction oppose u_t ≠ 0 (review M4). Conditions 3 and 4 are the strict forms of those two rules at u*, so the loop's own friction resolution, applied at u*, keeps the published friction states.

**Argument.** Under A1 each ball encloses its exact value at u*: displacements and reactions by kd5 §2's bound; the friction force by Stage A's enclosure, which Stage B carries in ψ; the derived normal by the reaction bound. Each condition puts the whole ball on the required side. For a derived sliding support, φ* = c·R_n(u*) holds exactly by §1.3–1.4, and the Stage-B ball at n certifies sign R_n(u*) = β, so φ* = −dir·μ·|R_n(u*)| opposes the certified slip.

**Uniqueness (R5).**

- **Frictionless states** (no friction support, or only inactive explicit-normal friction supports). The intended problem minimizes ½uᵀKu − fᵀu over the base-prescribed set with linear inequality constraints. For the exact-element model K is positive semidefinite, so every KKT point is a global minimizer; strict complementarity and the state's nonsingular reduced matrix make u* the unique one. This is a property of the exact-element model; positive semidefiniteness of the p-rounded K_int is not separately certified, so uniqueness is stated, not certified.
- **States with active friction.** No uniqueness is claimed. Static Coulomb friction can admit several consistent states; the certificate establishes that the published state is one of them, in DEC-067's path-independent model.

### 2.4 Fail closed

The case is `NUMERICAL_INTEGRITY_SENSITIVE` with one sentence; nothing is refused. Reasons are checked in this order and the first failing reason is reported. Decisions that need no factorization come first.

1. **Not run.** A gap-bearing case (R1), a case whose last iteration's report is not Passed, and a case with a load-row finding never run the confirmation: their standing is today's (the load-row sentence stands first; kd5 §3.3's no-op rule). Where such a case's standing would otherwise still be Passed, the reason is `confirmation attempt: not run`.
2. `prescribed row mismatch`: the loop's u differs from u_C, bit for bit, at a constrained DOF; or (revision 2) the loop's active prescribed map differs from the intended map that the confirmation builds independently of the loop's boundary code: every base-restrained DOF at its value in `base_prescribed` (+0 when absent) and every Active or Sticking support's DOF at p_s. The comparison covers the complete canonical maps (default zeros, coverage, duplicates and bits). It closes the boundary-consistency obligation; it does not check the product's primitive-to-map resolution, which the stage-1 refusals and route-parity tests cover.
3. Preconditions, per support in support-id order:
   - `support <id>: shares its DOF with another restraint` (two nonlinear supports, or a nonlinear support and a base restraint, on one DOF; review M5);
   - `support <id>: friction state active has no confirmation`;
   - `support <id>: friction normal source is not a base restraint` (manager's ruling; review I2);
   - `support <id>: constrained state is not a prescribed row` (an active or sticking support whose DOF is not in the state's prescribed set C, where its reaction ball is formed);
   then `coupled friction with <k> derived-normal sliding supports is not supported` (k > 1).
4. `confirmation attempt: <the gate's refusal or Sensitive>`.
5. `formation check: <stage>: <kd5's rendered detail>` for any kd5 §2.3 unavailability or D5C-1 demotion on Stage A, the influence system or Stage B (closure failures included).
6. `friction coupling not established` (0 ∈ M, a nonfinite ball).
7. Per support, in support-id order, at most six named: `support <id>: contact force sign not established` (active unilateral); `separation not established` (inactive unilateral); `stick capacity not established`; `slip direction not established`; `friction normal not established` (derived ball contains 0, β mismatch, derived Inactive, or explicit Inactive with N > 0). A nonfinite ball fails its condition and is reported under the same name. A support whose own ball (its reaction when constrained, its displacement when free) cannot be formed fails under the name of its kind and state; an inactive friction support, decided by its normal, fails under `friction normal not established`.

**Stage order within items 5–7 (k = 1).** Stage A's two evaluations (A-1, then the influence system A-2) are made before β is decided, so a formation-check failure of either is reported before A-3's `friction normal not established`. Stage B's evaluation needs Stage A's ball, so A-3's β failure and A-4's `friction coupling not established` are reported before any Stage-B formation-check failure; that is the one exception to the item order.

Rotational friction never reaches this list: it is refused at validation. A non-converged or unqualified case never reaches it either (§0).

**Which reasons are exercised end to end.** Through the product route (PP solving a model), only `stick capacity not established` (F-1 BOUNDARY) and `separation not established` (U-1 IN-PLANE, and the constant-effort precedence variant of `invented-loop-01`) are reached; the committed models and references reach no other reason. NI's tests reach `shares its DOF with another restraint`, `rotational friction has no confirmation`, `friction normal source is not a base restraint`, the k > 1 reason, the gap branch and Stage A's and Stage B's load-closure failures on real converged solves (with a mutated applied-force list for the closure), and `contact force sign`, `stick capacity`, `slip direction` and `friction normal not established` on synthetic balls. The others are defensive branches that no test reaches end to end: `confirmation attempt: not run` (a case that would still be Passed without the confirmation), NI's `the converged state is not qualified` and gap returns (PP calls the confirmation only on a qualified non-gap state), `prescribed row mismatch` (pinned on its pure helpers, `prescribed_row_mismatch` and, from revision 2, `intended_prescribed_map` with `prescribed_map_mismatch`; the qualification's receipt binds u), `friction state active has no confirmation`, `constrained state is not a prescribed row`, the gate's refusal or Sensitive, a K-D5 estimate demotion or unavailability other than load closure, `friction coupling not established`, an unformed ball, and `gap clearance not established` (unreachable under R1). In the product, rotational friction is refused at validation before NI's defensive reason.

**Sentence (product text, claims-checked):**

> "the converged support state is not established at 128-bit precision (<reason>); member rows are binary64 recoveries"

**Consequence.** The case keeps today's integrity diagnostic (the last iteration's report and product-equilibrium evidence), demoted by `formation_guard::amend_integrity_report` with this sentence, and today's binary64 loop rows and routing, byte for byte except the sentence. No notice is emitted. These cases never reach W1 (§4).

## 3. Publication

### 3.1 A Passed nonlinear case

A nonlinear case is pre-guard Passed when the last iteration's report is Passed with its product-equilibrium evidence (today's `CHECKS_PASSED`), there is no load-row finding, the confirmation runs, every evaluation passes and §2.2 holds. `member_bound` is then Stage B's.

**The Passed diagnostic (review M6)** is today's: the last loop iteration's `StructuralReport` with its product-equilibrium report. As on the linear route, a passing K-D5 evaluation adds no `formation_check` line; the confirmation gate's own report is not published. One evidence line is appended (§3.3).

| Rows | Published as |
|---|---|
| Displacements and rotations, all DOFs | fl(u + w + w₂) of Stage B, the exact sum rounded once, since T4-U4b/U5b (HELP_HUMAN, 2026-10-18; kd5 §3.4); the loop's u if any component cannot be formed. Each nonlinear support's final-displacement row carries the same published value. The loop's residual rows stay its own |
| Member, section and station actions, stresses, pipe-wall rows, governing maximum and extrema | (d)'s certified recoveries at u + w + w₂ of Stage B (kd5 §3); member loads carry no friction term, so they are those of the case ledger |
| Linear supports and springs | (d)'s certified rows, from Stage B's source and force |
| Nonlinear support, Active or Sticking | the certified reaction at d_s, fl(m), in its slot; the other five components exact +0 |
| Nonlinear support, Sliding, μ > 0 | v = fl(φ*) of the Stage-A ball (derived; B = r'), or fl(φ) (explicit; B = \|fl(φ) − φ\|) |
| Nonlinear support, Inactive, or Sliding with μ = 0 | +0, B = 0 |
| `nonlinear_support_final_reaction` | the same published value as that support's component |
| `nonlinear_support_friction_normal_reaction_derived` | \|v\| of the source restraint's published certified component |
| explicit `friction_normal_reaction` row | unchanged (input) |
| `nonlinear_support_final_displacement`, state codes, iteration count, final residual count, converged flag, delta and free-DOF residual observation rows | unchanged loop evidence |

- **The explicit-normal sliding row (review I1).** fl(φ) equals the loop's applied force bit for bit, but today's published row is the reported reaction fl(K·u − f − φ⁰)_t + φ⁰, which adds the binary64 residual of the free row. The certified row therefore differs from today's by that residual.
- **Groups.** Each nonlinear support is a support group of its body in the force or moment family by its DOF (kd5 §3.2), with its component vector and its separately published scalar magnitude. The friction-normal row is the source restraint's row.
- **The predicate, the notice and the maximum** run unchanged over all groups (kd5 §3.2–3.3). An inactive support's zero group is zero-compatible, with a = 0 and B = 0. A demotion keeps the certified rows, as in (d).
- **Combinations** of nonlinear models stay withheld.

### 3.2 Criterion

The 1e-9 criterion, RD(1e-9), the floor q̂ ≥ 2^-34·S* and the 2^-988 guard are unchanged. The strict margins in §2.2 are sign conditions on certified balls, not tolerances.

### 3.3 Text (claims-checked against `docs/claims_registry.md`)

- **The §2.4 sentence**, and its reasons, use "established"; no product text says "certified" (R5).
- **The Passed evidence line**, appended to the integrity diagnostic of a Passed nonlinear case:

  > "Converged support state: established at 128-bit precision by one formation-checked solve; each support's state has a sign margin larger than its bound, and a sliding friction force satisfies the friction law of that state within its bound; the published nodal displacements, and the support displacement rows that follow them, are the formation check's corrected displacements; the iteration's residual rows are its own."

  T4-U4b/U5b revised its last clause, which read "displacements and residual rows are the iteration's own", when the corrected displacement became the published one (§3.1; kd5 §3.4).

- **No formulation-basis change.** Revision 0 drafted a limitation sentence for runs with nonlinear supports. The `FormulationBasis.limitations` lists are schema constants, so a run-dependent sentence would change every reader of the profile. kd5 §3.4's two sentences already state what a Passed nonlinear case publishes (rows recovered at 128-bit precision from the formation check's corrected displacement; since T4-U4b/U5b, displacements are that corrected displacement rounded once wherever the member bound exists), so the case-specific statement goes in the evidence line above instead.
- **Retired:** the nonlinear-route variant of kd5 §2.3's sentence. Its `route` literal (pricing rule KD5A3) becomes the single "on this route".

### 3.4 The published values change (property, R4)

**Property.** On a load case with nonlinear supports published `NUMERICAL_INTEGRITY_CHECKS_PASSED`, member rows and support reactions, including the nonlinear support rows, are certified recoveries of the converged state's intended system. They are not binary64 recomputations from the published displacements, nor the loop's reported reactions.

- The sliding-friction force is the certified coupled value (derived normal) or the exact product −dir·μ·|N| rounded once (explicit normal). On the committed models it moves by the loop's coupling miss plus the free-row residual, at most about 1.4e-9 relative (§5), and becomes the same in both modes.
- Inactive nonlinear support rows become exact +0 instead of binary64 residual noise.
- Active and sticking reactions become certified recoveries.
- Blast radius: §5.

## 4. W1, pricing and runtime

**W1 never sees a nonlinear case.** Four W1-side refusals in code: PP `source_eligible(captured, nonlinear, combinations)` is false for nonlinear invocations (no retained-source attempt, no N-5 path); `source_recovery.rs` refuses the "nonlinear/contact source family"; `retained_memory.rs` D1.6 refuses `support.nonlinear`; `retained_product.rs` W1 refuses any built model with nonlinear supports. Separately, the exact pressure route refuses nonlinear supports (`exact_admission`); that is an exact-route refusal, not a W1 one (review M2). Nothing must happen in W1, and no fail-closed reason here routes to retained-source recovery. The confirmation calls no retained or W1 code.

**Pricing.** The retained route's behaviour is unchanged. The static TEXT and cap chain still reaches shared code, and `solve_load_case_observed` reaches the nonlinear helper.

- The confirmation and certificate sit behind the existing early return of `append_nonlinear_support_loop_results` (no nonlinear supports → return). Under D1.6 the chain computes multiplicity 0 for their sites from the existing nonlinear-helper premise; they keep honest classes and need no new zero rule (§9).
- **Outside the helper (review M3).** The publication changes in `solve_load_case_observed` (the nonlinear `member_bound` and its Stage-B force, the evidence line) and in `certified_publication::support` (the nonlinear slot) are reached by the static chain on every invocation. Their multiplicity is argued from the same D1.6 premise: each new branch is entered only when the built model has nonlinear supports (`built.nonlinear_supports` non-empty, or a confirmed state, which exists only then), so under D1.6 each has computed multiplicity 0 and holds no new allocation.
- Each new text site gets a class: the §2.4 sentence with its reason (support id ≤ 128 B) and the evidence line.
- KD5A3's route literal changes; the evidence-line site in `solve_load_case_observed` is charged positively (§9).
- `price.py run <head>`, then `compare`, then `generate`; re-registration stays under the owner's M = 12 GiB with kd5 §6's margins.

**Runtime cost model.** T_f is one gate and factorization; T_s one right-hand-side solve; T_p one K-D5 re-formation pass over all elements at p. Today a converged case costs about I·(T_f + T_s) plus I·(k + 1) dense coupling solves. The unit adds T_f + 4·T_s (gate and Hager estimate) + T_ε (ε_E, once) and per evaluation 3·T_p + 2·T_s, for one (k = 0) or three (k = 1) evaluations, plus the adjacency-limited reaction and displacement balls. The implementation measures it (§9).

## 5. Committed outputs and declared moves

**Probe** (revision 0's head a881a65631, release build, `openpipestress-runner solve` in both modes; evidence in `WT/scratch/t4m/nlkd5-design/`). The converged state's auxiliary symmetric system (NL-140 as a linear UY restraint; NL-130-FRIC's force as a nodal Z force equal to the loop's force) publishes `NUMERICAL_INTEGRITY_CHECKS_PASSED` through today's linear producer with K-D5 and (d), in both modes, on both models. Its coupling gain is c·g = −1.11e-3.

| Model (both identical in structure and loads) | Mode | Case | NL-140 reaction (N) | S-130 \|R_n\| (N; signed −) | u_t (mm) | published NL-130-FRIC row today (N) | (μ\|R_n\| − row)/row |
|---|---|---|---|---|---|---|---|
| demo and `preview_physics_invented` | sparse | L-100 | −297.627 | 52.373 | −6.6439 | 0.52373282006 | −3.57e-10 |
| | sparse | L-200 | −98.813 | 26.187 | −3.3220 | 0.26186641003 | −3.57e-10 |
| | dense | L-100 | −297.627 | 52.373 | −6.6439 | 0.52373281913 | +1.42e-9 |
| | dense | L-200 | −98.813 | 26.187 | −3.3220 | 0.26186640956 | +1.42e-9 |

- **What the friction column is (review I1).** It is today's published reported reaction fl(K·u − f − φ⁰)_t + φ⁰, not φ⁰. φ⁰ itself is mode-independent (§0); the two modes differ by the free row's binary64 residual, quantized at 2^-31 N on these models (one or two quanta). The relative miss against μ|R_n| combines that residual with the loop's coupling miss. Revision 0 attributed the whole miss to the coupling; that was wrong. The mode-independent φ⁰ and its own miss are measured at implementation.
- **R4's rationale, restated.** Neither the published row nor φ⁰ is the coupled-law value of the published state to within the criterion's scale on the friction row, and the row depends on the mode. The certified value is mode-independent to its bound. Publishing φ⁰ instead (Stage A skipped) is a mutant that only an exact reference can kill: F-2 (§6). The demo cannot, since its miss is not known exactly.
- **Margins** are of order 10^2 N, 10 N and mm, against kd5 bounds of order 1e-12 relative. NL-140 is sense −, Active (upper(r) < 0); R_n excludes 0; u_t < 0 with dir = −1.

**Declared moves (both models, both cases, both modes: 8 case-modes), declared before re-pinning.**

- `NUMERICAL_INTEGRITY_SENSITIVE` becomes `NUMERICAL_INTEGRITY_CHECKS_PASSED`; the nonlinear-route sentence is replaced by the evidence line (§3.3).
- Member, stress and support rows become certified: mostly last bits; S-130 and NL-140 reactions change at the level of the loop's solve error.
- The NL-130-FRIC row moves to the certified coupled value, the same in both modes; the `nonlinear_support_final_reaction` row of each nonlinear support equals its component; the derived-normal row equals |S-130 UY|'s certified component.
- Maximum metadata moves per kd5 §3.2. Displacements, state codes and loop-evidence rows are byte-identical. No notice is emitted (the cases are loaded).

**Committed outputs (5; 10 integrity diagnostics):** `fixtures/product_preview/invented_demo_result_preview_physics_1_{sparse,dense}.json`, `fixtures/product_preview/invented_demo_result_legacy_0_1.json`, `fixtures/results/preview_physics_invented_{sparse,dense}.json`. Every other committed nonlinear output is enumerated by the implementation's regeneration and listed with its standing in §9. The desktop demo's cases show Passed: its native-smoke expectation and desktop tests that pin the demo standing move with it. Under R1, gap outputs and `represented_gap_publication.rs` are byte-identical. The historical witness files under `validation/witness/` carry no integrity codes and do not move.

**Pins that move:** PP `kd5_eight_committed_nonlinear_case_modes_have_named_sensitive_reason` (re-pinned to Passed); PP `kd5_nonlinear_route_named_fail_closed_in_both_modes` (re-pinned to the measured outcome); the comment in RE `preview_physics_contract.rs`; NI `s11k_tests::kd5_nonlinear_sources_name_no_formation_check_entry_point` (R2); pricing KD5A3; every PP nonlinear-route integrity assertion that read the retired sentence; the committed outputs above.

**Blast radius as regenerated (implementation, before re-pinning).** The regeneration moved exactly the outputs declared above and their consumers; nothing else moved:

- The 5 committed outputs, regenerated by their generators (`tools/serialization/generate_product_preview_mechanics.mjs` for the three demo files and `demo_fixture_generation.json`; the `preview_physics_capture` example for the two `preview_physics_invented` files). In each, the 2 cases are Passed with the evidence line; displacements, state codes and loop-evidence rows are byte-identical. In `preview_physics_invented_{sparse,dense}.json`, 332 and 345 numeric values move: most by about 1e-12 relative or less; about 50 rows that were binary64 residual noise near zero (about 1e-11) become certified values below 1e-34 or exact zero; and the six NL-130-FRIC rows (its component, magnitude and `nonlinear_support_final_reaction`, both cases) move to the certified coupled value, the same in both modes (L-100: 0.5237328198735871 N, from 0.5237328200600512 sparse, −3.56e-10 relative, and 0.5237328191287286 dense, +1.42e-9; L-200 likewise), as §5's table predicted.
- The headline `max_open_formula_stress` moves from P-120 to P-100: their certified maxima are now exactly equal, and the existing identity tie-break picks P-100 (RE `preview_physics_contract.rs` re-pinned to the tie).
- Consumers of those bytes: `fixtures/results/retained_precision_carrier_cases.json` (two carrier SHA-256 values), `fixtures/results/base_reader_outcomes.json` and `fixtures/results/retained_precision_reader_outcomes.json` (regenerated by their writers), RE `reader_verdict.rs` (the invented standing has no findings), `apps/desktop/native-smoke/expected.json` (the demo's standing is `checks_passed`, so its result export is offered) and the desktop tests that pinned the Sensitive demo.
- Unchanged: every gap output (R1), `validation/witness/`, the historical reproduction records under `validation/evidence/`, and every linear-route output.

## 6. Evidence plan

**Independent references** (invented models only; no product code as an oracle). `validation/references/kd5_nonlinear/` holds a generator that forms each Euler–Bernoulli 12×12 frame element in exact rationals from the request's binary64 leaves (Przemieniecki, *Theory of Matrix Structural Analysis*; McGuire, Gallagher and Ziemian, *Matrix Structural Analysis*), assembles, enumerates the support states (each sliding support in both slip senses) and solves each state exactly (`fractions`). It records every state that satisfies all DEC-067 conditions strictly, with its exact rows. The models are axis-aligned with power-of-two lengths, so every element entry and every direction cosine is exact at p and the exact-element system is the p-rounded intended system; the premise allowance of revision 0 is not needed. Each request and oracle is bound by SHA-256, and the PP consumer checks the request binding and the actual normalized section leaves (kd5 §3.3 pattern).

- **Friction non-uniqueness.** When several states qualify, the oracle lists them all; the PP test requires the published state to be one of them and compares the rows with that state's exact solution.
- **What the PP consumer checks:** the published state; every published member and support group within its B of the exact value (‖v − v_ref‖ ≤ B); the friction row within B of the exact −dir·μ·|R_n|; Passed, or the named reason for the controls.

| Id | Model (invented) | Checks |
|---|---|---|
| G-1 | Anchored cantilever with a tip gap (both closed and open loads) | R1 containment control: `RECOVERY_BASIS_UNQUALIFIED`, no confirmation, byte-identical rows |
| F-1 | Anchored cantilever along X; tip linear UY restraint (normal source) and friction on UX with derived normal; tip loads P_y, P_x. Decoupled (g = 0): R_n = −P_y; stick iff \|P_x\| < μ\|P_y\|; slide u_x = (P_x − sign(P_x)·μ\|P_y\|)·L/(EA) | stick and slide certified, P_y of each sign; near-boundary \|P_x\| ≈ μ\|P_y\| fails closed ("stick capacity"); explicit-normal twin with μ·N inexact in binary64; μ = 0 sliding control |
| F-2 | L-frame: anchored X leg, vertical Z leg, top restraint UZ (normal source) and friction on UX, μ = 0.5; strong coupling (\|c·g\| reported by the generator) | Stage A; the published friction force within B of the exact coupled force in both modes |
| U-1 | One-way support on an L line, lifted (Inactive) and pressed (Active); a twin with a parallel spring on the same DOF; a rotational twin with the support on RX (moment family); a zero-load control (u_d = 0, r = 0 exactly) | separation and contact sign; spring included in the contact force; the rotational twin's moment and rotation; the zero-load control fails closed (R3) |

The committed demo models are natural positives (Passed in both modes).

**Mutants that must be killed** (each by a named test, §9):

- **Certificate.** Midpoint-only comparison (radius dropped); `≥` in place of `>` (U-1's exact-zero control); reaction sign convention reversed (U-1 Active and the demo's NL-140); a spring at d excluded from the contact force (U-1's parallel-spring twin; this mutant proved equivalent at implementation, §9); derived N signed instead of |R_n|, or β not checked (F-1 with P_y of each sign); stick capacity without ρ_n or ρ_t, and an inexact (rounded-to-nearest) stick comparison (F-1 near-boundary and a synthetic-ball unit test); slip sign not checked (synthetic-ball unit test); deciding from the loop's binary64 trial values instead of the balls (synthetic balls whose midpoint has the right sign but whose radius straddles 0).
- **Coupling.** Publishing φ⁰ (Stage A skipped) and (1 − c·g)⁻¹ dropped (F-2); the Stage-A radius missing from ψ, or g's radius dropped (unit test on a constructed ball case); k > 1 not refused (unit test); using Stage A's bound inputs for §2.2 or publication (F-2).
- **Closure.** Missing, duplicate, extra or wrong-binding friction primitive (NI unit tests on the manifest and the closure); the explicit force re-formed from fl(μN) instead of the exact μ·N (FK re-formation test with μN inexact, and F-1's explicit twin).
- **Confirmation.** K-D5 evaluated at the gate's own solution (displacement bytes pinned equal to the loop's); the influence system judged by D5C-1 (the demo fails closed under it); `prescribed row mismatch` not detected (unit test); a second formation-check entry or an in-loop entry (the re-scoped source pin and its mutant).
- **Publication.** Nonlinear rows not certified, or an inactive row nonzero (demo, U-1); `nonlinear_support_final_reaction` differing from the component (equality pin); loop-evidence rows altered (byte pins).
- **Controls.** n ∉ B (a spring-family source), rotational friction (validation refusal), μ = 0 sliding, k > 1, and each §2.4 reason with its sentence and byte-identical rows.
- **Scope.** Under R1 every gap test and output is byte-identical. W1: a source pin shows the confirmation calls no retained code.

**Re-run unchanged:** FK `formation_check_tests` (`kd5_*`), PP `s11g_tests` and the kd5 sets on the linear route. This unit changes no linear-route arithmetic.

## 7. Implementation plan

**FK (DEL-04-01).**

- `formation_check.rs`: `LoadFormula::SlidingFrictionExplicit` and `SlidingFrictionCoupled`, with bindings, nodal DOF binding in the closure, and re-formation (exact product; ball). `evaluate` gains `judged` (D5C-1 on or off) and a per-attempt ε_E cache.
- `structural.rs`: `finish_confirmation(&PositiveFactor) -> (StructuralSolution, ConfirmationAttempt)`; `ConfirmationAttempt::evaluate_at(&FormationCheckedSystem, u, judged)`, which requires the factor's stiffness, free map and contributions.
- `certified_recovery.rs`: `certify_displacement(dof, u, bound)`; `CertifiedRow::enclosure`, exact sign and stick comparisons (`lower_exceeds`, `upper_below`, `established_sign`, `stick_capacity_exceeds`, `stick_capacity_exceeds_explicit`); `CertifiedValue::into_row`.
- `load_ledger.rs`: `AssembledForce::with_appended(LoadLedger)`.

**NI (DEL-04-04, DEL-04-07).**

- `structural_adapter.rs`: `AssemblyEvidence::confirm_with_formation_check`, the route's single formation-check entry, which prepares, factors, finishes and hands an evaluation context to its caller.
- `confirmation.rs`: `confirm_converged_state(input, springs, solve, force, curved_sources, mode) -> Confirmation`, with the preconditions, the manifest, Stage A, the influence evaluation, Stage B, and the pure predicate `support_reason` with `certificate_reason`, unit-testable on synthetic balls. The loop, the classifier and `product_selected_state_is_qualified` are unchanged.
- The S11-K pin is re-scoped (R2): the loop's functions contain no formation entry; `confirm_converged_state` contains exactly one call to the confirmation entry; the product calls `confirm_converged_state` once, inside the nonlinear helper, and names neither the adapter entry nor FK's confirmation plumbing (`finish_confirmation`, `ConfirmationAttempt`), which no non-adapter NI module names either (implementation review I1).

**PP (DEL-04-07).**

- Validation refuses rotational friction.
- After qualification (inside the nonlinear helper's non-empty branch), call the confirmation for non-gap states whose last report is Passed with no load-row finding (§2.4 item 1).
- On Passed: `member_bound` and the Stage-B force from the confirmation; (d)'s recovery loop with the loop's u; `certified_publication::support` gains the nonlinear slot; the nonlinear scalar rows publish the same values; the evidence line.
- Otherwise: the §2.4 sentence, today's rows.

**Dependency conditions.** DEL-04-07's need on DEL-04-01 gains the friction primitives, the confirmation attempt with its supplied-u evaluation, and the displacement ball. The implementing change updates the condition text. DEL-04-04's classifier is unchanged.

## 8. Rulings

All of revision 0's R1–R5 are ruled (see "The rulings"). None is a criterion change. Remaining owner choices: G2 for gaps (a later unit); a tolerance for weakly active supports (not proposed).

## 9. Measured at implementation

Measured on the implementing head with an instrumented scratch copy of the code (the instrumentation prints each ball; it is not committed). "ρ" is a ball's decision radius (evaluation radius plus remainder); "max w" and "λσ" are Stage B's `MemberBoundInputs`.

**The 8 committed case-modes** (demo and `preview_physics_invented` are identical in structure and loads and give identical balls; both modes):

| Case | NL-140 contact m (N), ρ | S-130 R_n (N), ρ | NL-130-FRIC u_t (m), ρ | friction v (N), B | max w; λσ |
|---|---|---|---|---|---|
| L-100 | −297.627, 8.7e-11 | −52.373, 1.09e-10 | −6.644e-3, 1.3e-18 | 0.5237328198735871, 3.5e-17 | ≤ 6.7e-15; 2.1e-14 |
| L-200 | −98.813, 4.4e-11 | −26.187, 5.5e-11 | −3.322e-3, 6.4e-19 | 0.26186640993679355, 1.7e-17 | ≤ 3.4e-15; 1.0e-14 |

- Every margin exceeds its radius by at least 12 orders of magnitude. All 8 case-modes are established and Passed.
- **Stage A** (both cases, both modes): c = −0.01, g = 0.11103 (c·g = −1.11e-3, as the probe found), β = −1 (R_n < 0). φ⁰'s own coupling miss (c·N⁰ − φ⁰)/φ⁰ is −5.77e-13 and is mode-independent. The Stage-A radius r' is 3.5e-17 N (L-100). The published friction row moved by −3.56e-10 (sparse) and +1.42e-9 (dense) relative (§5): almost all of that is the free row's binary64 residual in the old reported reaction, not the coupling miss (review I1 confirmed).
- The certified friction value equals μ|R_n| of the certified source reaction to the last bit printed (0.5237328198735871 against 0.5237328198735872).

**References** (`validation/references/kd5_nonlinear/`, both modes):

| Id | Case | Outcome | Margin, ρ |
|---|---|---|---|
| F-1 | STICK (\|R_t\| = 30 N, μ\|R_n\| = 50 N) | established | stick margin 20 N, ρ = 0 (an exact solve: w = 0) |
| F-1 | SLIDE-POS / SLIDE-NEG | established | u_t = ±8.47e-8 m, ρ 6e-42; R_n = ±100 N, ρ 1.7e-34; v = ∓50 N, B 2.9e-37 |
| F-1 | BOUNDARY (\|P_x\| = μ\|P_y\| exactly) | `stick capacity not established` | margin exactly 0 (R3) |
| F-1 explicit | STICK / SLIDE (N = 70, μ = 0.3) | established | slide v = −21 N exactly rounded, B = 7.8e-16 (the exact product's rounding) |
| F-1 μ = 0 | FREE | established | u_t = 2.82e-8 m, ρ 8e-47; slot +0 |
| F-2 | SLIDE (strong coupling) | established | c·g = −0.748; φ⁰ miss 2.4e-13; r' = 1.6e-14 N; R_n = 1427.976 N, ρ 4.1e-9; v = −713.98800347588 N |
| U-1 | PRESSED / LIFTED | established | contact 100 N, ρ 0; separation 6.67e-4 m, ρ 2.3e-42 |
| U-1 spring twin | PRESSED / LIFTED | established | contact 100 N, ρ 0; separation 4.00e-4 m, ρ 1.2e-42 |
| U-1 rotational twin (RX) | PRESSED / LIFTED | established | contact moment Mx = 1400/9 N·m exactly (oracle bounds 155.55555555555554 to 155.55555555555557); separation θx = 700/EI = 3.045e-4 rad |
| U-1 | IN-PLANE (zero control, u_d = 0 exactly) | `separation not established` | margin exactly 0 (R3) |
| G-1 | PRESSED / LIFTED with a gap | not run (R1) | `RECOVERY_BASIS_UNQUALIFIED`, rows byte-identical |

**Fail-closed rate** over every nonlinear model the product-physics test suite solves (693 tests): 50 distinct model/case/mode outcomes: 36 established, 6 not established, 8 not run. All 6 not-established outcomes are deliberate: F-1's exact boundary (2), U-1's zero control (2), and the constant-effort precedence variant of `invented-loop-01`, whose CE-120 is given NL-140's one-way template and stays inactive with a separation ball that does not exclude 0 (L-100 and L-200, sparse; R3). All 8 not-run outcomes are gap-bearing cases (G-1 and the invented loop's L-GAP and L-MIXED-NONLINEAR; R1). No committed output fails closed.

**Mutants** (each applied to a scratch copy, the named suites run; `mutants/run_all.log` and `run_pp_only.log` in the implementation's scratch evidence):

| Mutant | Killed by |
|---|---|
| M01 midpoint-only (radius dropped) | NI `stick_capacity_is_decided_exactly_with_both_radii`, `unilateral_signs_need_the_whole_ball_strictly_on_their_side`, `sliding_needs_the_slip_sign_and_the_coupling_branch` |
| M02 `≥` for `>` | NI `unilateral_signs_need_the_whole_ball_strictly_on_their_side` |
| M03 reaction sign convention reversed | PP `kd5_eight_committed_nonlinear_case_modes_pass_with_the_confirmation`, `kd5_nonlinear_references_bind_and_match_in_both_modes` |
| M04 spring at d excluded from the contact force | **equivalent at zero motion only**: a contact is evaluated only at an Active (prescribed) DOF, where the spring's term k·u_d multiplies the prescribed u_d = 0 exactly, and an inactive support is decided from its displacement ball. Revision 1's claim that U-1's twin would kill it was wrong. Revision 2: when the support's ground moves, u_d = g_s ≠ 0, so the term is k·g_s and the mutant is observable; W-16 stage 2's B-1 spring twin (R = P − (k + 3EI/L³)·d) kills it. |
| M05 derived N signed, M07 without ρ_n, M08 without ρ_t, M09 rounded stick comparison | NI `stick_capacity_is_decided_exactly_with_both_radii` |
| M06 β not checked, M10 slip sign not checked | NI `sliding_needs_the_slip_sign_and_the_coupling_branch` |
| M11 φ⁰ published (Stage A skipped) | PP references (F-2) |
| M12 (1 − c·g)⁻¹ dropped, M13 g's radius dropped | NI `coupled_ball_contains_the_coupled_force_far_from_the_loop_value` |
| M14 Stage-A radius missing from ψ | FK `coupled_sliding_friction_re_forms_as_its_stage_a_ball` |
| M15 k > 1 not refused, M26 derived source outside B admitted | NI `named_preconditions_fail_closed_before_the_attempt` |
| M16 Stage A's bound used for §2.2 and publication | NI `evaluations_take_the_loops_displacement_and_only_the_influence_is_unjudged`; PP references |
| M17 explicit force re-formed from fl(μN) | FK `explicit_sliding_friction_is_the_exact_product_of_its_inputs` |
| M18 K-D5 at the gate's own solution | NI `evaluations_take_the_loops_displacement_and_only_the_influence_is_unjudged`. In PP it is numerically equivalent on every model: the gate solves the same matrix and right-hand side as the loop's final solve, so u_C equals the loop's u bit for bit. |
| M19 influence system judged by D5C-1 | PP references and the eight committed case-modes (the demo fails closed under it) |
| M20 confirmation not run | PP `kd5_nonlinear_route_named_fail_closed_in_both_modes`, references, eight committed case-modes |
| M21 prescribed row mismatch not detected | NI `prescribed_rows_must_match_bit_for_bit` |
| M22 inactive nonlinear row nonzero, M23 final reaction ≠ component, M28 nonlinear slot left at zero | PP references |
| M24 rotational friction admitted | PP `rotational_friction_is_refused_by_name_at_validation` |
| M25 μ = 0 sliding given a friction term | NI `a_zero_coefficient_sliding_support_carries_no_term_and_is_established` |
| M27 a second formation-check entry | NI `r2_confirmation_entry_pin_kills_a_second_or_in_loop_entry` |
| M29 the product calls the adapter entry directly (implementation review I1), or names FK's confirmation plumbing | NI `r2_confirmation_entry_pin_kills_a_second_or_in_loop_entry` and `kd5_nonlinear_sources_name_no_formation_check_entry_point` (the reviewer's probe, which passed every S11-K test before the repair, now fails both) |

**Pricing** (DEL-04-08 retained-pricing-rules.md §G). The static chain prices the confirmation's sites at multiplicity 0 under D1.6. The evidence-line site is charged positively (+15,309,300 B on TAV_text, TAV_W and TAV_X); 685 of 689 coefficients are unchanged. The block is regenerated and qualifies at the owner's M = 12 GiB (W3 sparse 10,206,071,081 B, dense 10,265,202,425 B, without R). A per-diagnostic id allocation in the evidence line's scan (62 MB on route E) was found by the pricing run and removed in code.

**Runtime** (release `openpipestress-runner solve`, end to end, under the exclusive wrapper; median of 20 runs after 2 warm-ups; base = the merge base `c00eba0cf2`):

| Model | Mode | Base (ms) | Head (ms) |
|---|---|---|---|
| demo | sparse | 31.9 | 115.3 |
| demo | dense | 32.5 | 115.6 |
| `preview_physics_invented` | sparse | 31.9 | 118.2 |
| `preview_physics_invented` | dense | 33.8 | 117.0 |

- Each run solves two load cases. The confirmation itself (gate, factorization, Stage A, the influence evaluation, Stage B and the certificate) takes 1.2–1.7 ms per case, measured inside the release build with an instrumented scratch copy. The remaining ≈ 80 ms per run is therefore not the confirmation. The other work a Passed case adds is (d)'s certified publication (kd5 §3: member, section, station and support rows recovered at p, and the certified maxima), which a Sensitive case skipped; it was not timed separately. The figures are reported, not judged against a threshold: no runtime threshold applies.

## Open items

- **k > 1** (several coupled derived-normal sliding supports) fails closed. A later extension can certify it with Rump's verified-linear-system theorem (Rump, "Verification methods: Rigorous results using floating-point arithmetic", *Acta Numerica* 19, 2010, §10), as revision 0's A-4 sketched.
- **G2 for gaps** remains open (R1).
- **The loop's own coupling precision** is a loop-side numerical issue; publication no longer depends on it. DEC-046's convergence policy is unchanged.
- **The nonlinear route has no Design file of its own** (DEL-04-04 has no `Design/`). This file is the route's K-D5 design.
