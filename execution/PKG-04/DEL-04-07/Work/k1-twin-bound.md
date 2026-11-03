# K-1 on exact twins: the option-(a) amendment and its bound (RT-A3 increment 4)

**Status.** Drafted for independent review. Agent 0 chose option (a): each route keeps its established section
formation (EX's D-5 sequence, PF's own). K-1 is reworded below, and its assertion
(`every_kd5_control_twin_is_within_the_k1_bound` in `core/product_physics/tests/rt_a3_nonlinear_k1_twins.rs`) is
implemented and held `#[ignore]`d until this amendment is independently reviewed. The basis is computed by
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
>     `numerical_quality` record, and kd5's evidence line or its named §2.4 reason;
>   - every other loop row is finite on both routes and satisfies |x_EX − x_PF| ≤ a_f + r_f·S_f in
>     its family's canonical unit. These rows are the nodal displacements and rotations, the support
>     action components and magnitudes, the nonlinear support displacements, reactions and friction
>     normals, and the loop's free-row residual, residual-work and iterate-delta observations. The
>     families, the fixed a_f and r_f, the scales S_f formed from the case's inputs, and the
>     zero-scale rule are those of DEL-04-07 `k1-twin-bound.md`. A non-finite value fails.
>
>   Each route keeps its established section formation, so bit identity is not required. This is a
>   bounded comparison on the named corpus. It does not claim identical results elsewhere, nor near
>   active-set transitions beyond the corpus's two weakly decided cases (F-1 BOUNDARY, U-1 IN-PLANE),
>   whose equal states are observations.

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
| translation_delta | m | `…_observed_max_translation_delta` | as translation | 0 | 2^-27 ≈ 7.45e-9 |
| rotation_delta | rad | `…_observed_max_rotation_delta` | as rotation | 0 | 2^-25 ≈ 2.98e-8 |
| force_delta | N | `…_observed_max_force_reaction_delta` | as force | 0 | 2^-20 ≈ 9.54e-7 |
| moment_delta | N·m | `…_observed_max_moment_reaction_delta` | as moment | 0 | 2^-24 ≈ 5.96e-8 |

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
Euler–Bernoulli element, exact axes and assembly. It does this for EVERY support state the loop could visit: each
enumerated state that is nonsingular, with sliding in both senses and both normal branches, and gap closure. Per
published value x it bounds

  |x_EX − x_PF| ≤ |x*_EX − x*_PF| + E_EX(x) + E_PF(x) + P(x).

- **The reduced-system perturbation** x*_EX − x*_PF is the exact response of the intended system to the section
  difference, computed exactly.
- **The solve.** Every structural solve, on every iterate, returns only when each free row's original-equation
  residual meets 64·γ(n) of its row denominator Σ|K_ij·u_j| + |f_i|, its evaluation allowance included
  (`frame_kernel::structural::finish_structural` over `evaluate_original_residual`, after at most three refinements;
  otherwise it refuses). By Oettli and Prager, the computed u then solves (K + δK)·u = f + δf with
  |δK| ≤ ω·|K|_abs and |δf| ≤ ω·|f|, where ω = 64·γ(n_max) + γ(16) + 64·γ(n_max)·γ(16). The γ(16) term absorbs
  each route's binary64 formation of K about the exact-leaves K, and |K|_abs = Σ_e|K_e| + Σk. Here n = 2·(nonzeros
  in the row) + 2. That gives n_max = 10 and ω = 7.28e-14 for the one-member F-1 models, and n_max = 16 and
  ω = 1.155e-13 for the others.

  The free-DOF error e then satisfies e ≤ ω·|A⁻¹|·(|K|_abs·(|u*| + e) + |f| + |K_fc|·|p|) (Skeel), where |K_fc|·|p|
  is the gap's prescribed term. This is closed componentwise by a weighted fixed point,
  e ≤ b/(1 − θ) with θ = max_i (M·b)_i / b_i and M = ω·|A⁻¹|·|K|_abs. That is a dimensionally scaled condition
  estimate: M_ij carries the unit ratio of DOFs i and j, so θ is dimensionless. θ ≤ 3.0e-10 over every state of every case and both routes.
- **The recovery.** A recovered reaction is bounded by its cancellation,
  Σ_j |K_ij|·e_j + γ(n_i + 1)·(|K|_abs·(|u*| + e) + |f|)_i. A free row's computed residual (an inactive support's
  reaction, and the residual records) is bounded by (ω + γ(n_i + 1))·(…)_i. A spring action is bounded by
  k·e + u·k·|u|.
- **The coupled friction force.** A sliding derived-normal support's force is formed as φ = −c·R_b/(1 + c·g), from
  the loop's base and unit dense solves (`nonlinear_integration`, the derived-candidate block). Here c = dir·μ·β and
  g = R_u − R_b is the cancellation-prone influence. Its error is
  (|c|·E_Rb + |c|·E_g·|φ*|)/(|1 + c·g| − |c|·E_g) + γ(4)·|φ|, which propagates through |A⁻¹·e_t|. Explicit forces add
  fl(μ·|N|)'s rounding.
- **Publication.** P(x) covers the mm conversion and each row's rounding, and the norm for magnitudes. For
  magnitudes, |‖a‖ − ‖b‖| ≤ Σ_k |a_k − b_k|.
- **Certified rows.** A Passed case publishes certified and corrected rows. These lie closer to the route's own
  p-rounded intended solution, which differs from the exact-leaves solution by the formation term already inside ω,
  so the loop enclosure still bounds them.

**Constants from the enclosures.** Value and residual rows are the final iterate's, so r_f is the next power of two
at or above twice the largest enclosure/S_f over the final states of every case. A delta row compares
max|x_k − x_k−1| across routes. It is bounded by the final state's enclosure plus the largest over every enumerated
state, doubled and rounded up likewise.

| Family | Largest enclosure/S_f, final states | Largest, every state | Where |
|---|---|---|---|
| translation | 6.94e-11 | 3.34e-9 | F-2 SLIDE |
| rotation | 1.78e-10 | 8.58e-9 | F-2 SLIDE |
| force | 5.38e-9 | 2.59e-7 | F-2 SLIDE (support:ANCHOR force magnitude) |
| moment | 5.45e-10 | 2.62e-8 | F-2 SLIDE |
| residual_force | 6.05e-12 | 6.05e-12 | U-1 LIFTED |
| residual_moment | 1.34e-12 | 1.34e-12 | U-1 LIFTED |
| work | 2.84e-13 | 2.84e-13 | U-1 LIFTED |

F-2 governs because of the coupled friction force. Its final branch has c = +0.5, g = 1.496357 and 1 + c·g = 1.748178,
giving φ = −713.988003 N with an enclosure of 2.13e-6 N. kd5 §9 records v = −713.98800347588 N and c·g = −0.748 in
its own 1 − c·g sign convention, so the two agree. Its other normal branch, which the loop could visit, has
1 + c·g = 0.251822 and φ = 4956.6 N, above the input force scale. That branch sets the all-states column, which is
why the delta families are looser. F-1's derived friction is decoupled (g = 0 exactly, φ = ∓50 N, enclosure
2.8e-12 N).

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
within its enclosure; the largest ratio is 1.3e-3 (F-1 explicit SLIDE). Against the family bound
a_f + r_f·S_f, the observed differences use at most:
- 7.6e-8 (translation), 3.7e-8 (rotation), 8.3e-9 (force) and 3.8e-8 (moment);
- 3.1e-4 (residual_force), 2.8e-4 (residual_moment) and 1.5e-4 (work);
- 2.0e-7, 1.3e-7, 7.3e-8 and 1.9e-7 for the four delta families.

The bound is valid and deliberately conservative. Its solve term rests on the 64·γ(n) residual certificate the product
actually verifies, not on the far smaller errors it typically achieves.

**Why basis (ii) and not (i).** Basis (i) would inject EX's section leaves into PF's prepared element inputs and
show the loop rows move as EX's do. PF has no input that carries section leaves. Injecting them needs a product-side
test hook in PF's section formation, which is outside this lane's write scope. Basis (ii) obtains the quantity (i)
would measure, the response to the section difference, exactly. It encloses every other route difference by each
route's own certificate rather than attributing it.

**Assumption.** The basis takes EX's Euler–Bernoulli element to be the same 12×12 frame element as PF's and kd5's,
formed from its own leaves. A formula difference would add to x*_EX − x*_PF and is not enclosed. Three observations
support the assumption:
- the linear twins differ only in the last places;
- every observed difference is at most 1.3e-3 of its enclosure;
- the states and standings agree.

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
- **Delta families** are looser, from 2^-27 to 2^-20 against 2^-32 to 2^-26 for the value families. They cover any
  state the loop could have visited, including F-2's non-admissible coupled branch.
- **Scope.** The scales and constants are this corpus's: axis-aligned, Euler–Bernoulli, p = 0 controls. They
  claim nothing for other models, for Timoshenko members or bends, or for pressure-loaded cases. Extending K-1
  needs the basis re-run for the extension.
- **Discrete evidence.** States, counts, convergence and standing are asserted exactly on this corpus. Near an
  active-set transition (the two weak cases) their equality is an observation, not a consequence of the bound.

## 5. Controls and the held assertion

`core/product_physics/tests/rt_a3_nonlinear_k1_twins.rs`:
- `every_kd5_control_twin_is_within_the_k1_bound` is K-1 as amended. It asserts the discrete evidence exactly and
  every value row within its family's bound (`k1_violations`, `k1_scales`, `K1_FAMILIES`). It is held
  `#[ignore]`d. Run explicitly on this base, it passes all 34 case-modes.
- `the_k1_bound_refuses_its_controls` runs un-held. It tests the comparison, not the corpus. In both modes:
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
  - `every_kd5_control_keeps_its_states_counts_and_standing_on_its_exact_twin`.
- `every_kd5_control_twin_is_bit_identical` stays held as K-1's superseded first wording.
