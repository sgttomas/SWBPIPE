# K-1 on exact twins: the option-(a) amendment and its bound (RT-A3 increment 4)

**Status.** Adopted after independent review (the third review, nl4, at `7678bce3a`, found no blocking
finding), replacing K-1's first wording in R-4 §3.5. It was revised for the second review (nl3): the reaction
enclosure's formation term and force-error closure, the iterate-delta basis on a derived history, and the
element-formula source trace. Agent 0 chose option (a): each route keeps its established section
formation (EX's D-5 sequence, PF's own). K-1 is reworded below, and its assertion
(`every_kd5_control_twin_is_within_the_k1_bound` in `core/product_physics/tests/rt_a3_nonlinear_k1_twins.rs`) is
implemented and runs. The basis is computed by
[`k1_twin_bound.py`](k1_twin_bound.py), run as `python3 execution/PKG-04/DEL-04-07/Work/k1_twin_bound.py`
(standard library only, no product code, about a second).

**Why K-1 needs amending.** EX forms each member's section constants by D-5's sequence and PF by its own. For
the controls' 0.168 × 0.007 m section, A differs by 3 units in the last place and I and J by 1
(`kd5_twin_section_constants_differ_from_pf_in_the_last_place`). The linear twin differs too. Bit-identical loop
results are therefore not available while both formations stand. The independent review of increment 4 asked for:
- a fixed, explicit, per-family, dimensionally defined bound;
- scales S_f defined independently of the candidate outputs;
- explicit zero-scale behaviour and a finiteness requirement;
- a numerical basis that is not the observed 4.5e-15.

## 1. Wording (R-4 §3.5, K-1)

**Old.**

> - **K-1, controls.** Every PF K-D5 nonlinear control (`kd5_nonlinear_tests.rs`, the
>   `references/kd5_nonlinear` set) is run on EX twins with explicitly empty regions, and
>   must give bit-identical loop results and the same standing.

**New.**

> - **K-1, controls (option (a)).** Every PF K-D5 nonlinear control that forms an exact twin is run
>   on its EX twin with explicitly empty regions, in both solver modes. The named corpus is the
>   seven `validation/references/kd5_nonlinear/` requests with the load cases
>   `kd5_nonlinear_tests.rs` solves, plus the U-1 gap containment control (17 cases, 34
>   case-modes). Controls with no exact twin are refused by name, not adapted. On this corpus only:
>   - each support's state code, the loop's iteration count, final residual count and converged
>     flag, and the standing are identical. The standing is the integrity code, the
>     `numerical_quality` record, and kd5's evidence line or its named §2.4 reason. Each case's
>     iteration count is the basis' (one to three), and neither route reports the first-iterate
>     all-active seed recovery;
>   - every other loop row is finite on both routes and satisfies |x_EX − x_PF| ≤ a_f + r_f·S_f in
>     its family's canonical unit. These rows are the nodal displacements and rotations, the support
>     action components and magnitudes, the nonlinear support displacements, reactions and friction
>     normals, and the loop's free-row residual, residual-work and iterate-delta observations. The
>     families, the fixed a_f and r_f, the scales S_f formed from the case's inputs, and the
>     zero-scale rule are those of DEL-04-07 `k1-twin-bound.md`. A non-finite value fails. An
>     iterate-delta row's bound is for the preceding iterate the basis derives from the authored
>     seeds and the first iterate; the product publishes no per-iteration state.
>
>   Each route keeps its established section formation, so bit identity is not required. This is a
>   bounded comparison on the named corpus. It does not claim identical results elsewhere, nor near
>   active-set transitions beyond the corpus's two weakly decided cases (F-1 BOUNDARY, U-1 IN-PLANE).
>   Their equal final states, the iteration counts and the absence of the seed recovery are observed
>   discrete premises, not consequences of the bound.

## 2. The bound

Every row is compared in its family's canonical unit. A mm row's bound is ×1000.

| Family | Unit | Rows | S_f | a_f | r_f |
|---|---|---|---|---|---|
| translation | m | nodal UX–UZ; translational nonlinear displacement | S_F·C_t + g | 0 | 2^-32 ≈ 2.33e-10 |
| rotation | rad | nodal RX–RZ; rotational nonlinear displacement | S_F·C_r + g/L_min | 0 | 2^-31 ≈ 4.66e-10 |
| force | N | support Fx–Fz and force magnitude; nonlinear reaction (N); friction normals | S_F | 0 | 2^-26 ≈ 1.49e-8 |
| moment | N·m | support Mx–Mz and moment magnitude; nonlinear reaction (N·m) | S_F·L | 0 | 2^-29 ≈ 1.86e-9 |
| residual_force | N | `…_observed_free_dof_force_residual` | S_F | 0 | 2^-36 ≈ 1.46e-11 |
| residual_moment | N·m | `…_observed_free_dof_moment_residual` | S_F·L | 0 | 2^-38 ≈ 3.64e-12 |
| work | N·m | `…_free_dof_work_residual` | S_F·S_translation + S_M·S_rotation | 0 | 2^-40 ≈ 9.09e-13 |
| translation_delta | m | `…_observed_max_translation_delta` | as translation | 0 | 2^-32 ≈ 2.33e-10 |
| rotation_delta | rad | `…_observed_max_rotation_delta` | as rotation | 0 | 2^-31 ≈ 4.66e-10 |
| force_delta | N | `…_observed_max_force_reaction_delta` | as force | 0 | 2^-27 ≈ 7.45e-9 |
| moment_delta | N·m | `…_observed_max_moment_reaction_delta` | as moment | 0 | 2^-29 ≈ 1.86e-9 |

**The scales S_f** are formed from the PF request's inputs only, never from either route's outputs
(`scales` in the script, `k1_scales` in the test, by the same operations):
- the load force F = Σ|P| over the case's primitive loads, plus μ|N| for each explicit-normal friction support;
- the members' total length L and shortest length L_min;
- the authored annulus's A, I and J = 2I, in binary64 from the outside diameter and wall, with their minima and
  maxima over the members;
- E and G from the materials;
- the springs' stiffness Σk and the gaps' clearance g.

From these:
- C_t = L/(E·A_min) + L³/(E·I_min) + L³/(G·J_min) (m/N), and C_r = L²/(E·I_min) + L²/(G·J_min) (rad/N);
- K_max = E·A_max/L_min + 12·E·I_max/L_min³ + Σk (N/m);
- S_F = F + g·K_max.

**Zero scale.** a_f = 0 in every family. A scale is zero only when the case has no load, no applied friction and no
gap. Then u* = 0. Every iterate's certificate (§3) forces the computed solution to solve (K + δK)·u = 0 with
K + δK nonsingular, so u = 0 exactly on both routes, and every derived row is an exact zero. A zero-scale family
therefore admits equality only.

**Finiteness.** A NaN or infinite value on either route is a violation, as is a non-finite scale or bound. So is a
row without a family, and a pair whose routes publish different rows.

## 3. Basis: independent forward-error enclosures (review basis (ii))

For each control and load case, the script forms each route's intended system in exact rationals from that route's
own binary64 section leaves. The PF leaves are the kd5 oracles' `normalized_sections`; the EX leaves are the measured
D-5 values: A = 0.003540574920595697 m², I = 1.149359133598378e-05 m⁴ and J = 2.298718267196756e-05 m⁴. The
formation is kd5's independent reference (`validation/references/kd5_nonlinear/reference.py`): the 12×12
Euler–Bernoulli element, exact axes and assembly. It enumerates every nonsingular support state, with sliding in
both senses and both normal branches, and gap closure, and keeps those the DEC-067 classification admits as final
states. It also forms the iterates that precede the published final iterate (below, "Iterate deltas"). Per
published value x it bounds

  |x_EX − x_PF| ≤ |x*_EX − x*_PF| + E_EX(x) + E_PF(x) + P(x).

- **The reduced-system perturbation** x*_EX − x*_PF is the exact response of the intended system to the section
  difference, computed exactly.
- **The solve.** Every structural solve, on every iterate, returns only when each free row's original-equation
  residual meets 64·γ(n) of its row denominator Σ|K_ij·u_j| + |f_i|, its evaluation allowance included
  (`frame_kernel::structural::finish_structural` over `evaluate_original_residual`, after at most three refinements;
  otherwise it refuses). Since RT-A3 P the sparse-mode loop solves on the stiffness pattern, and the base, unit-load
  and final right-hand sides of an iterate share one factor (`finish_sparse_structural` and
  `finish_sparse_structural_on`). Each right-hand side still runs the same completion (`finish_checked_factor`)
  over the same binary64 residual (`evaluate_original_residual_bound`, which skips zero coefficients in both
  representations, so n is unchanged). The certificate below therefore holds per solve as before, in both modes. By Oettli and Prager, the computed u then solves (K + δK)·u = f + δf with
  |δK| ≤ ω·|K|_abs and |δf| ≤ ω·|f|, where ω = 64·γ(n_max) + γ(16) + 64·γ(n_max)·γ(16). The γ(16) term absorbs
  each route's binary64 formation of K about the exact-leaves K, and |K|_abs = Σ_e|K_e| + Σk. Here n = 2·(nonzeros
  in the row) + 2. That gives n_max = 10 and ω = 7.28e-14 for the one-member F-1 models, and n_max = 16 and
  ω = 1.155e-13 for the others.

  The free-DOF error e then satisfies e ≤ ω·|A⁻¹|·(|K|_abs·(|u*| + e) + |f| + |K_fc|·|p|) (Skeel), where |K_fc|·|p|
  is the gap's prescribed term. This is closed componentwise by a weighted fixed point,
  e ≤ b/(1 − θ) with θ = max_i (M·b)_i / b_i and M = ω·|A⁻¹|·|K|_abs. That is a dimensionally scaled condition
  estimate: M_ij carries the unit ratio of DOFs i and j, so θ is dimensionless. θ ≤ 3.0e-10 over every state of every case and both routes.
- **An applied force's own error.** A computed friction force f̂_t = f_t + δ_t with |δ_t| ≤ E_t (E_φ below, or
  fl(μ·|N|)'s rounding) makes the solve satisfy (K + δK)·û = f + δ_t·e_t + δf. So
  e ≤ |A⁻¹|·e_t·E_t + ω·|A⁻¹|·(|K|_abs·(|u*| + e) + |f| + E_t·e_t), and the same weighted fixed point closes it,
  with b now carrying both the direct term and E_t inside the certificate's |f̂|. No higher-order feedback is dropped.
- **The recovery.** A recovered reaction is R̂_i = fl(K̂_i·û − f_i) on the route's rounded assembled row K̂_i, with
  |K̂ − K| ≤ γ(16)·|K|_abs (the formation allowance inside ω). Since R̂_i − R*_i = [rounding] + (K̂_i − K_i)·û +
  K_i·(û − u*), it is bounded by
  Σ_j |K_ij|·e_j + γ(16)·Σ_j |K|_abs,ij·(|u*_j| + e_j) + γ(n_i + 1)·(1 + γ(16))·(|K|_abs·(|u*| + e) + |f|)_i.
  The direct formation term is needed separately: a constrained-constrained entry times a prescribed value moves a
  reaction and leaves the free solution unchanged, so no displacement error accounts for it (the nl3 review). A free
  row's computed residual (an inactive support's reaction, and the residual records) is bounded by
  (ω + γ(n_i + 1))·(…)_i. A spring action is bounded by k·e + u·k·|u|.
- **The coupled friction force.** A sliding derived-normal support's force is formed as φ = −c·R_b/(1 + c·g), from
  the loop's base and unit solves (`nonlinear_integration`, the derived-candidate block). Until RT-A3 P these were
  dense in both modes. Since P they run in the requested mode, on the iterate's factor in sparse mode. The
  enclosure below uses only each solve's own residual certificate and the recovery bound, never the
  factorization, so it holds for either. Here c = dir·μ·β and
  g = R_u − R_b is the cancellation-prone influence. Its error is
  E_φ = (|c|·E_Rb + |c|·E_g·|φ*|)/(|1 + c·g| − |c|·E_g) + γ(4)·|φ|, with E_Rb and E_g = E_Rb + E_Ru + u·|g| from the
  recovery bound above (formation term included). It enters the final solve as an applied force's own error.
- **Publication.** P(x) covers the mm conversion and each row's rounding, and the norm for magnitudes. For
  magnitudes, |‖a‖ − ‖b‖| ≤ Σ_k |a_k − b_k|.
- **Certified rows.** A Passed case publishes certified and corrected rows. These lie closer to the route's own
  p-rounded intended solution, which differs from the exact-leaves solution by the formation term already inside ω,
  so the loop enclosure still bounds them.

**Iterate deltas.** A delta row is max_i |x_n,i − x_n−1,i| over the family's DOFs: the final iterate n against the
one before it (`residual_observation`). Bounding it needs the preceding iterate's state, slip direction,
derived-normal branch and applied friction force on both routes. The first review's basis took the largest
enclosure over every enumerated state. That bounds only matched preceding states: two exact solves on different
branches (F-2's +0.5 and −0.5 coupled branches) differ by 0.0116 m, far above any allowance (the nl3 review).
The product publishes no per-iteration state, direction, branch or force. It publishes the iteration count, the
final states, the final iterate's rows and the loop's solver diagnostics. The basis therefore derives the
preceding iterate from the product's rules, under observed discrete premises:
- *Premises (observed, asserted exactly by the K-1 test, `k1_history_premises`).* The iteration count is 1, 2 or
  3 per case (`ITERATIONS`; F-1 STICK, F-1 BOUNDARY and F-1 explicit STICK take 3). Neither route reports the
  first-iterate all-active seed recovery (`SOLVER_SINGULAR_SYSTEM`, "recovered by one all-active trial"). The twin
  keeps the PF request's supports, seeds included. The final states are equal on both routes.
- *The first iterate* solves the authored seeds (`normalized_initial_states`). A sliding seed has no previous
  iterate, so no friction force is applied (`sliding_friction_candidates` returns no candidate without one). This
  iterate is input-determined: no numerical decision precedes it.
- *The second iterate's input*, for a friction support seeded sliding: the deferred first iterate keeps it sliding
  unless its normal is ≤ 0 (`resolve_sliding_friction_states`). Its slip direction is the sign of the first
  iterate's displacement at the support DOF when nonzero (`sliding_direction`). Its derived-normal branch is the
  sign of the first iterate's source reaction when nonzero (`solve_iteration_with_sliding_friction_evidence`). The
  basis requires each deciding quantity to have the same nonzero exact sign on both routes and to exceed its
  enclosure on each. Each route's binary64 decision is then that sign. The smallest such margin is 3.3e11 times its
  enclosure (F-2's first-iterate slip, 2.04e-3 m against 6.2e-15 m).
- *The preceding iterate* is the first for n = 2 and the second for n = 3; n = 1 publishes no delta row. For n = 2,
  the derived second-iterate direction and branch are an admissible final state's, which checks the derivation
  against the final classification. For n = 3, the second iterate's output is the third's input, which is the
  published final state (zero changes in the final iterate).
- *The bound.* |max_i|a_i| − max_i|b_i|| ≤ max_i |a_i − b_i|. Per DOF this is the exact difference of the routes'
  exact deltas, plus the final and preceding iterates' enclosures on both routes, plus per route γ(2) of the
  operands for the subtraction and the unit conversion. Weakly decided finals take the larger over their
  admissible states. Under these premises the delta and value families are bounded by matched-history enclosures.
  No constant is widened to cover a cross-branch pair.

**Constants from the enclosures.** r_f is the next power of two at or above twice the largest enclosure/S_f over
every case. For value and residual rows these are the final states (the final iterate's rows). For delta rows they
are the final and the derived preceding iterates. The corrected recovery and force-error terms leave every value
and residual family under its constant, explicitly 2 × largest ≤ r_f:

| Family | Largest enclosure/S_f | 2 × largest | r_f | Where |
|---|---|---|---|---|
| translation | 6.942e-11 | 1.388e-10 | 2^-32 = 2.328e-10 | F-2 SLIDE |
| rotation | 1.782e-10 | 3.564e-10 | 2^-31 = 4.657e-10 | F-2 SLIDE |
| force | 5.386e-9 | 1.077e-8 | 2^-26 = 1.490e-8 | F-2 SLIDE (support:ANCHOR force magnitude) |
| moment | 5.449e-10 | 1.090e-9 | 2^-29 = 1.863e-9 | F-2 SLIDE |
| residual_force | 6.050e-12 | 1.210e-11 | 2^-36 = 1.455e-11 | U-1 LIFTED |
| residual_moment | 1.335e-12 | 2.670e-12 | 2^-38 = 3.638e-12 | U-1 LIFTED |
| work | 2.841e-13 | 5.682e-13 | 2^-40 = 9.095e-13 | U-1 LIFTED |
| translation_delta | 6.952e-11 | 1.390e-10 | 2^-32 = 2.328e-10 | F-2 SLIDE |
| rotation_delta | 1.784e-10 | 3.568e-10 | 2^-31 = 4.657e-10 | F-2 SLIDE |
| force_delta | 3.259e-9 | 6.518e-9 | 2^-27 = 7.451e-9 | F-2 SLIDE |
| moment_delta | 5.457e-10 | 1.091e-9 | 2^-29 = 1.863e-9 | F-2 SLIDE |

The value-family maxima were 6.94e-11, 1.78e-10, 5.38e-9 and 5.45e-10 before the formation term and the closure;
their constants are unchanged. The delta constants are re-derived from the corrected basis (previously 2^-27,
2^-25, 2^-20 and 2^-24 from the every-state maximum), not from observed differences.

F-2 governs because of the coupled friction force. Its final branch has c = +0.5, g = 1.496357 and 1 + c·g = 1.748178,
giving φ = −713.988003 N with an enclosure of 2.13e-6 N. kd5 §9 records v = −713.98800347588 N and c·g = −0.748 in
its own 1 − c·g sign convention, so the two agree. Its other normal branch has 1 + c·g = 0.251822 and φ = 4956.6 N.
That branch is not admissible as a final state, and the derived history excludes it as a preceding iterate (F-2's
preceding iterate is its seed solve, with no friction force). F-1's derived friction is decoupled (g = 0 exactly,
φ = ∓50 N, enclosure 2.8e-12 N).

**Active-set margins** (final states, exact, against the enclosure of their deciding quantity). Every strictly
decided state has a margin at least 1.3e8 times its enclosure:
- F-2's normal is 1427.976 N against an enclosure of 3.19e-6 N;
- F-2's slip is 5.83e-4 m against 4.35e-12 m;
- U-1's contact is 100 N against 5.6e-14 N;
- G-1's contact is 98.50 N against 1.24e-10 N;
- every separation is at least 3.0e-4 (m, or rad for the RX support) against at most 7.1e-15.

Two cases are weakly decided, with an exact zero margin. These are F-1 BOUNDARY (stick capacity and slip) and
U-1 IN-PLANE (contact and separation). There the states the routes publish are equal by observation, which K-1
asserts; the bound does not imply it.

**Validation against the products' outputs (not used to set constants).** Every value row of the 34 case-modes was compared
with its own final-state enclosure: 1,490 rows covering every family above. Every observed |x_EX − x_PF| is
within its enclosure; the largest ratio is 1.3e-3 (F-1 explicit SLIDE; measured before the formation term and the
closure, which only enlarge the enclosures). The 104 delta rows are each within their case's derived-history
enclosure, the largest ratio 1.5e-3 (F-1 explicit STICK). Against the family bound a_f + r_f·S_f, the observed
differences use at most:
- 7.6e-8 (translation), 3.7e-8 (rotation), 8.3e-9 (force) and 3.8e-8 (moment);
- 3.1e-4 (residual_force), 2.8e-4 (residual_moment) and 1.5e-4 (work);
- 6.5e-6, 8.3e-6, 9.3e-6 and 6.2e-6 for the four delta families.

The bound is valid and deliberately conservative. Its solve term rests on the 64·γ(n) residual certificate the product
actually verifies, not on the far smaller errors it typically achieves.

**Re-check for RT-A3 P (DEL-04-07 `rt-a3-p-scale-repair.md`).** The source trace was re-checked against P's loop.
- *Unchanged loop rules.* The authored seeds (`normalized_initial_states`), no friction force on the first iterate
  (`sliding_friction_candidates`), the deferred first iterate (`sliding_force_deferred`), the slip direction
  (`sliding_direction`), the normal branch, the classification (`resolve_sliding_friction_states`,
  `evaluate_active_set_iteration_with_resolved_friction_states`) and the seed recovery are byte-identical in P's
  diff. P changes only how each linearized solve is formed: the representation, the coupling solves' mode, and in
  sparse mode one factor per iterate.
- *Solve.* Every right-hand side keeps its own M03 gate (above), so ω and the Skeel closure hold per solve, and
  the coupled-force enclosure E_φ holds for φ⁰ from either factorization.
- *Corpus.* The twins are solved by the same loop on both routes in each mode, so P moves both routes alike. Its
  φ⁰ move (sparse mode) changes bits only on derived-normal sliding cases.
- *Derivation and script.* They model the intended exact system, the per-solve certificate and the loop rules,
  none of which P changes; the script's text names the solve path and its numbers are unchanged. The script and
  the K-1 corpus were re-run in both modes on P's head (the record in `rt-a3-p-scale-repair.md`).

**Why basis (ii) and not (i).** Basis (i) would inject EX's section leaves into PF's prepared element inputs and
show the loop rows move as EX's do. PF has no input that carries section leaves. Injecting them needs a product-side
test hook in PF's section formation, which is outside this lane's write scope. Basis (ii) obtains the quantity (i)
would measure, the response to the section difference, exactly. It encloses every other route difference by each
route's own certificate rather than attributing it.

**Element formula (source trace).** The basis takes EX's Euler–Bernoulli element to be the same 12×12 frame element
as PF's and kd5's, formed from its own leaves; a formula difference would add to x*_EX − x*_PF and is not enclosed.
The source establishes it:
- PF and EX populate the same `StraightPipeSectionProperties` and `FrameElement` path
  (`core/product_physics/src/lib.rs:8681`, `:8787`, `:8835`; `core/solver/straight_pipe/src/lib.rs:460`, `:507`);
- Euler–Bernoulli members use the shared `local_stiffness` (`core/solver/frame_kernel/src/lib.rs:727`);
- its EA/L, GJ/L and 12/6/4/2 EI blocks and signs match kd5's `local_stiffness`
  (`validation/references/kd5_nonlinear/reference.py:67`);
- the named corpus is axis-aligned, so the axes are exact signed permutations; the twin restores E and G bit for
  bit (`exact_twin`).

## 4. Resolution and limits

- **What the bound can detect.** A perturbation is detectable when it moves some value by more than
  a_f + r_f·S_f, so the detectable relative perturbation is (r_f·S_f)/|x| at a case's largest values.
  - A 1e-6 relative EX-only *response* perturbation is detected in every one of the 34 case-modes; the worst case
    needs 5.6e-7 (G-1 PRESSED).
  - A 1e-6 *section* perturbation is detected only where displacements are large against S_t. Examples are U-1
    LIFTED (from 2.2e-9), U-1 RX, the U-1 spring twin and F-2 (from 5e-8 on translations).
  - A near-uniform section change leaves statically fixed forces unchanged. It moves displacements by about ε·|u|.
    So it is not visible on U-1 PRESSED, whose displacement rows are all zero, and needs about 2e-6 to 7e-6 on F-1's
    translations.
- **Delta families** are bounded for the derived matched history only (§3, "Iterate deltas"), under the observed
  premises (iteration count, no seed recovery, equal final states). They do not cover arbitrary cross-branch
  preceding states, and a case outside the corpus needs its own history derivation.
- **Loop-rule changes.** The history is derived from a source trace of the loop rules, not observed: the
  product publishes no per-iteration state, and the script encodes those rules rather than reading them. Equal
  iteration counts and final states do not detect every rule change (for example a changed second-iterate force),
  and re-running the unchanged script returns the old history. A change to those rules therefore requires
  re-checking the source trace, revising the derivation and script, and only then re-running the script and the
  corpus before the constants are asserted again.
- **Scope.** The scales and constants are this corpus's: axis-aligned, Euler–Bernoulli, p = 0 controls. They
  claim nothing for other models, for Timoshenko members or bends, or for pressure-loaded cases. Extending K-1
  needs the basis re-run for the extension.
- **Discrete evidence.** States, counts, convergence and standing are asserted exactly on this corpus. Near an
  active-set transition (the two weak cases) their equality is an observation, not a consequence of the bound.

## 5. Controls and the assertion

`core/product_physics/tests/rt_a3_nonlinear_k1_twins.rs`:
- `every_kd5_control_twin_is_within_the_k1_bound` is K-1 as amended. It asserts the discrete evidence exactly,
  the history premises (`k1_history_premises`, `K1_ITERATIONS`), and every value row within its family's bound
  (`k1_violations`, `k1_scales`, `K1_FAMILIES`). It runs, and passes all 34 case-modes.
- `the_k1_bound_refuses_its_controls` tests the comparison, not the corpus. In both modes:
  - the U-1 lifted twin without its parallel spring keeps its state, counts and standing. The comparison reports
    the missing spring rows and translation values outside the bound;
  - a 1e-6 relative EX-only wall-thickness perturbation keeps the discrete evidence and fails on translations, on
    U-1 LIFTED and F-2's coupled SLIDE;
  - a 1e-6 EX-only response perturbation fails the value bound on every twin case-mode;
  - U-1 PRESSED under a 0 N load has every S_f = 0. Both routes publish exact zeros, which hold. A one-ulp offset
    from zero fails as "rotation, S_f 0e0";
  - NaN, +∞ and −∞ on either route are refused as not finite.
- Kept:
  - `kd5_twin_section_constants_differ_from_pf_in_the_last_place` (the section cause and the linear-twin regression);
  - `every_kd5_control_keeps_its_states_counts_and_standing_on_its_exact_twin`, which also asserts the history
    premises un-held.
- `every_kd5_control_twin_is_bit_identical` stays held as K-1's superseded first wording.
