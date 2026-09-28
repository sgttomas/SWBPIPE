# K5 RETURN (W4: the constrained-body witness and the curved rule), draft at checkpoint A2

I14, a Type 2 TASK, for ROOT (HELP_HUMAN). **This is a draft.**
- §2 (the derivations) and §4 (the A2 evidence) are complete for A2's scope.
- The sections marked "at D" are written at checkpoint D, with CHANGE_RECORD and SHA256SUMS.

**Placeholders:**
- `<wt>` is the T3 worktree root; `<scratch>` is `<wt>/scratch/i14`; `<VENV>` is the repository venv.
- `T3/` is `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`, and `P/` is `projects/chirality-piping/`.

**Platform:** `aarch64-apple-darwin`, rustc 1.97.1. Every product comparison here is Mac against Mac, built from `git archive` copies of Mac main `24dea2dae` and of the candidate. None is compared with a Linux record.

**Delegation mechanism:** a Claude Code background subagent of ROOT's session, started through the Agent tool and resumed by SendMessage. I14 delegated nothing.

## 1. Scope as ruled

The rulings are ROOT's, in the brief and in `ROOT_RULINGS_V1.md`: "K5: spawn and rulings" and "K5: rulings on I14's checkpoint-0 plan".

| Ruling | Choice |
|---|---|
| Q1(b) | W4 runs in the four selected branches of the formation-checked entries only |
| Q2(b) | A curved slot is qualified by its matched macro source |
| Q3(a) | T4's confirmation is not a prerequisite |
| Q4(b) | A libm-free screen in the new function |
| Q5(a) | User elements are ties only with positive stiffnesses; a T4 tripwire guards the rule |
| Q6(a) | Directional ground rows are accepted |
| Q7 | Reversed to (a) after S1: the basis text is unchanged everywhere |
| Q8(b) | Gate part 1, compared with G1's baseline |
| Q9(a) | No site-table change |
| Q10(a) | The curved-formation item is not K5's |
| Q11 | One slice |

- A1 (FK) is committed as `0c732061b`.
- A2 (the SA wiring, the tests and the product runs) is in the working tree.

## 2. Derivations

Notation:
- A node motion is d = (u, θ), with u the translation and θ the rotation.
- A rigid motion of a point set is u(x) = t + θ × (x − o), with θ common.
- "Exact" means as real numbers on the binary64 inputs.

### 2.1 The energy zero sets (§4.9's proof, written out)

The families that can appear in the evidence of a selected invocation are the ones `EvidenceParts::new` records. They are straight frames, user-stiffness elements, curved slots and ground springs (`SA` `EvidenceParts::new`).

M03's contribution audit binds the assembled K to exactly these contributions, so K contains no other stiffness. The prescribed DOFs are the only other boundary data.

For each family, the energy E ≥ 0 and its zero set are as follows.

1. **A straight frame, intended element** (E, G, A, I, J > 0, L > 0).
   - E = ½ dᵀ Tᵀ K_L T d, with K_L the exact Euler–Bernoulli local matrix and T = blockdiag(R, R, R, R) exactly orthonormal.
   - K_L is positive semidefinite with null space exactly the six rigid modes of the two-node beam.
   - So E = 0 ⟺ d is a rigid motion of the pair. This is the welded-frame argument `assess_rigid_body` already rests on.
2. **A ground spring k > 0 at DOF g:** E = ½ k d_g², so E = 0 ⟺ d_g = 0. It is a ground row. A zero spring contributes nothing (E ≡ 0), and is not a ground (`SA` records only `value > 0.0` in `spring_ground`).
3. **A prescribed DOF:** d_g = 0 for any admissible variation. It is a ground row.
4. **A user element, today's (FK `user_stiffness_local_matrix`).**
   - K_L = Σ_d k_d (e_d − e_{d+6})(e_d − e_{d+6})ᵀ, with k = (axial, lateral, lateral, torsional, angular, angular).
   - So E = Σ_d k_d (Δ_d)², with Δ = T(d_j − d_i), and T invertible whenever the orientation is valid.
   - **If every k_d > 0:** E = 0 ⟺ Δ = 0 ⟺ d_j = d_i, that is u_a = u_b and θ_a = θ_b (the tie).
   - **If k_lateral = 0:** the zero set is {θ_a = θ_b, u_b − u_a ∈ span(local y, z)}, which strictly contains the tie space. That is why `user_element_tie` refuses it (Q5(a)).
   - **The represented global matrix** has the exact block form [[A, −A], [−A, A]] in value. `transform_global_stiffness` forms each 3×3 block with the same operation sequence on ±k_d, and rounding is sign-symmetric. So it annihilates the tie space exactly. `k5_t4_tripwire_user_tie_space_is_the_represented_null_space` asserts both facts.
5. **A curved element, intended (R5-4 §2).**
   - K_int = Tᵀ [[H K_t Hᵀ, −H K_t], [−K_t Hᵀ, K_t]] T, with R exactly orthonormal (x = r_i/|r_i|, z = normalized r_i × r_j, y = z × x), T = blockdiag(R, R, R, R), H = [[I, 0], [skew(c), I]], **c = R(x_j − x_i)** (the actual chord), and K_t = F⁻¹ symmetric.
   - Hᵀ[a; b] = [a + b × c; b], so E = ½ eᵀ K_t e with e = T_j d_j − Hᵀ T_i d_i.
   - **(a) Every rigid motion of the pair is a zero** (for any positive semidefinite K_t).
     - For u_j = u_i + θ × Δ, θ_j = θ_i = θ, with Δ = x_j − x_i: e = [R u_j − R u_i − (Rθ) × (RΔ); 0].
     - That is [R(θ × Δ) − (Rθ) × (RΔ); 0] = 0, because R is proper orthogonal.
   - **(b) If K_t is positive definite,** E = 0 ⟺ e = 0 ⟺ d is a rigid motion of the pair. F is positive definite for 0 < φ < π with positive rigidities and factors, because the six unit-load section-action fields are then independent. This direction serves only W4's completeness, never the soundness of a refusal (§2.3).

**The total energy** E_tot(d) = Σ_families E(d) is a sum of non-negative terms. It vanishes exactly when every term does: when d is rigid on each objective sub-body (frames and qualified curved elements join nodes into sub-bodies), equal at every tie, and zero on every ground row. That is the null space of the unreduced stacked map.

### 2.2 The tie reduction (A1; the generator cross-checks it on 4,000 cases)

This is as in I14's plan §4, and in the doc comment of `assess_constrained_bodies`.
- A spanning tree of the tie graph over the sub-bodies solves the tree ties' 6(S−1) rows uniquely for each child's (t_B, θ_B) from its parent's: θ_B = θ, t_B = t + θ × s_B, with s_B = Σ (x_a − x_b) exactly.
- So the solution set of the tree rows is the image of the injective linear map P: (t, θ) ↦ ((t + θ × s_B, θ))_B.
- The remaining rows are the cycle ties and the grounds, evaluated at every node as u(x) = t + θ × (x − o + s_B). They are the reduced rows.
- Hence null(stacked) = P(null(reduced)), with equal dimensions and identical node motions.
- A different tree changes s_B by cycle offsets only, and the cycle rows carry the difference.
- **Cross-check:** the generator asserts, on all 4,000 B1 cases, that the reduced rows' exact nullity equals the unreduced stacked map's.

### 2.3 The Q2 condition: a K5-C1 refusal never refuses a model the intended physics restrains (Q2(b), Q3(a))

**Claim.** Suppose W4 publishes a witness d on the selected branch, as `Mechanism { direction: d }`. Then:
- (i) d is an exact zero-energy motion of the intended system, with every frame and qualified curved element intended and every user element as represented;
- (ii) so the intended free-DOF stiffness is singular, and the intended physics does not restrain the model;
- (iii) the product's own curved element carries energy along d only through its non-objectivity.

**Step 1: what W4 verified.**
- `assess_constrained_bodies` publishes d only after checking, exactly (`Expansion`), against the original coordinates:
  - every tie: u_a = u_b; θ is common by construction;
  - every ground row: every prescribed DOF and every positive spring, as zero;
  - that d is nonzero.
- Rigidity on each sub-body holds by construction: u(x) = (t + θ × s_B) + θ × (x − o) for x ∈ B.
- The node motions are exactly representable (`exact_scalar`).
- The FK test `k5_constrained_bodies` re-checks every published witness with `ExactAccumulator` (rigid per sub-body, ties, grounds). The SA and PP tests check the published bits against exact derived values.

**Step 2: which coordinates W4 used.**
- For every node, the coordinates are those `EvidenceParts` recorded from the frames and users.
- For a node touched only by curved elements, they are the matched macro element's `node_i` or `node_j` coordinates.
- `W4Context::body` refuses (`CurvedCoordinates`) any matched macro whose node coordinates differ from the recorded ones, or from another matched macro's at the same node.
- So every qualified curved element's two nodes carry, in W4, exactly the coordinates its macro source (and hence its intended element, §2.1 item 5) is formed from.

**Step 3: every family's intended energy vanishes on d.**
- **Frames:** both nodes of a frame lie in one objective sub-body (frames are links), and d is a rigid motion of that sub-body's actual coordinates. So E_frame(d) = 0 (§2.1 item 1).
- **Qualified curved elements:** likewise, a link with both nodes in one sub-body and d rigid on it with the macro's coordinates. So E_curved,int(d) = 0 (§2.1 item 5(a)). **This needs only K_t positive semidefinite, never K_t's accuracy or positive definiteness.**
- **User elements:** every user element in the body is a tie (an unqualified one leaves the body to the matrix gate), and d satisfies the tie. So E_user(d) = 0 for the element as represented (§2.1 item 4).
- **Positive springs:** d_g = 0, so E = 0. Zero springs have E ≡ 0.

**Step 4: singularity.**
- d vanishes at every prescribed DOF, so its nonzero components are free DOFs: d_f ≠ 0.
- The intended stiffness K_int is positive semidefinite and d_fᵀ K_int,ff d_f = E_tot(d) = 0, so K_int,ff d_f = 0.
- **So K_int,ff is singular:** there is a zero-energy motion compatible with every support. A load with a component along d has no equilibrium, and the intended physics does not restrain the model.
- **Hence a K5-C1 refusal (`NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`) is never issued for a model the intended physics restrains.**

**Step 5: the product's element along d.**
- Let K̃ be the represented curved matrix. Write K̃ = K_prod + δK_form:
  - K_prod is the product's formulation evaluated exactly on its own binary64 factors: axes R̃ from binary64 normalization, chord c_prod = (R̃_m(cos φ̃ − 1), R̃_m sin φ̃, 0) through the platform's `sin`, `cos` and `atan2`, and K̃_t;
  - δK_form is the rounding of H·K·Hᵀ and of TᵀKT.
- Along d, K_prod's energy is ½ ẽᵀ K̃_t ẽ, with ẽ = [R̃(θ × Δ) − (R̃θ) × c_prod; 0] (the rotation rows cancel exactly).
- **If R̃ were exactly orthogonal,** ẽ = [(R̃θ) × δc; 0] with **δc = R̃Δ − c_prod, the chord mismatch.**
- **Otherwise** a further term R̃(θ × Δ) − (R̃θ) × (R̃Δ) of order ε·|θ|·|Δ| appears, from R̃'s departure from orthogonality.
- So E_prod(d) = ½ ẽᵀ K̃_t ẽ + ½ dᵀ δK_form d. It is nonzero only through:
  - the chord mismatch;
  - R̃'s non-orthogonality;
  - the formation rounding.
- These three are the non-objectivity R5-4 §2 names ("the chord and trigonometry mismatch, the rounding of H·K_t·Hᵀ, and the transform") and the T4 row records.
- For arc-consistent geometry with exact trigonometry, δc = 0.
- On Mac main, the matrix gate sees only this residual energy along d. It then refuses with a pivot failure, or it would publish a value along d, held only by the formation error.
- **Observed** (§4.4): on this corpus Mac main refuses every constructed curved mechanism `NUMERICAL_INTEGRITY_UNRESOLVED`. K5 changes the refusal's code and adds the witness, and removes no published result.

**Q3(a).**
- The claim is made for the intended element (Steps 1–4). It needs no property of the product's element beyond the matching of its slot to its macro source.
- T4's null-space item remains open for the product's element (Step 5; R5-4 §5).
- **Notice to T4:**
  - the tie rule is for today's user element;
  - the M07 repair changes the local form, and the tripwire test fails;
  - the curved rule (qualification by source) is stated for the intended element.

**What the condition does not claim.**
- W4's `Restrained` is not a restraint proof. The matrix gate still runs, and its refusals stand.
- An unqualified body is left to the matrix gate as before.
- So a completeness gap in W4 (a mechanism it does not witness) can only reproduce today's outcome.

### 2.4 Frame-only byte identity

1. `assess_rigid_body` and `original_rigid_witness` are byte-identical: `rigid_body.rs:1-248` hashes `d9598efa…` on the base and on the candidate, and A1's diff is one insertion hunk after line 249.
2. `BodyEvidence::geometry` gains one branch, `if !qualified { if let Some(w4) = &self.w4 { w4.screen(…)?; } }`, before the unchanged skip.
   - With `w4 = None`, the branch is inert. The loop's search order, qualification test, coordinates, ground order and `assess_rigid_body` call are today's lines.
   - Every call site except the four selected bodies passes `w4: None`: `AssemblyEvidence::geometry` and `SparseAssemblyEvidence::geometry`, reached from `solve`, `solve_binary64`, `solve_assembled`, `solve_force_scaled` and the four `selected = false` returns.
3. With `Some`, the branch runs only for a body with a non-objective edge. A frame-only body takes today's path unchanged, and an isolated node (no edge) is skipped as today.
4. `symmetry_basis`, `qualified_passive_family` and the edges' objective flags are unchanged (Q7(a); no edge flag is set).
5. **So** a model whose bodies are all frame-only has identical outcomes, errors and direction bits on every entry.
   - A model whose first failing body in seed order is frame-only keeps today's error.
   - Only a mixed body met in seed order before any failing frame-only body can change the outcome, and only on the selected branches (K5-C1 and K5-C2).
6. **Checked:** the RF-MECH harness (§4.2) is byte-identical base against candidate. `k5_first_failing_body_in_seed_order_decides` pins both seed orders.

### 2.5 No invocation with a nonlinear support changes; the contact-seed guard stays closed

1. **PP.** The one call is `assembly.solve_assembled_with_formation_check(…, &curved_sources, built.nonlinear_supports.is_empty())` (`PP:4441-4449`; s11k's pin: one call, in `solve_preview_reduced_system`).
   - With any nonlinear support, `selected` is false. The entry returns `self.solve_assembled(…)` before `constrained_geometry` (the `if !selected` return precedes it; `k5_w4_entry_is_named_only_in_the_four_selected_bodies`).
   - `solve_assembled` calls `geometry` with `w4: None` (§2.4).
2. **The nonlinear loop** calls `solve_binary64` (NI `:1990`), which calls `geometry` with `w4: None`. No NI or PP non-test source names `constrained_geometry`, `W4Context`, `W4Body` or `assess_constrained_bodies` (the text pin).
   - The loop's first solve is bit-equal to `solve_binary64` on a curved-mechanism model whose selected branches refuse it as a W4 mechanism (`k5_nonlinear_loop_keeps_todays_geometry`).
3. **W2's orchestrator** `solve_with_force_scaling` passes `case.selected`; a nonlinear invocation passes false.
   - Before F1b, PP does not call it. After F1b, PP passes the same `selected` (F1b's brief); checked at the merge, with no PP edit by K5.
4. **The contact-seed guard.**
   - `permits_contact_seed_trial` admits any `Mechanism` (`SA:1900`). A W4 `Mechanism` exists only on a selected branch, that is with `built.nonlinear_supports` empty.
   - PP's arm (`PP:2884`) also requires `eligible_contact_dofs(…, &built.nonlinear_supports, …).is_some()`, which is `None` for an empty support list (NI `:439`).
   - The loop never sees a W4 error (item 2).
   - **So the guard stays closed.**
5. **Observed:**
   - every gap variant in the product-run table (§4.4) is byte-identical to Mac main;
   - `k5_curved_mechanism_with_a_nonlinear_support_keeps_todays_refusal` pins it;
   - no PP in-crate test with a realized joint reaches W4 (§4.5).

**Notice to T5.**
- If the loop adopted W4 (by passing curved sources and a W4 context to its geometry), a mixed-body mechanism in iteration 1 would become a `Mechanism`.
- `permits_contact_seed_trial` would then admit the contact-seed trial (NI `:648-652`, with `recovery_eligible` and an inactive support), and so would PP's arm at `:2884`.

### 2.6 The screen's platform independence (Q4(b); A1)

This is as in I14's plan §5.7.
- K5's code uses IEEE +, −, ×, ÷ and `sqrt`, integer bit operations, and `Expansion`/`exact_rounded_sum`. It uses no other function of unspecified precision (FK test B10 scans it).
- Rust never contracts a·b + c into an FMA, and every loop order is fixed by the canonical input.
- Transitively, `Expansion::add_product` calls FK's existing `exact_radix`. It uses `2.0_f64.powi(step)` with |step| ≤ 512 and a round-trip self-check, so an inexact `powi` could only withhold a witness, never produce a wrong one.
- **So** the status, `rigid_parameters` and `node_motion` are deterministic functions of the input bits on every IEEE-754 platform.

**`assess_rigid_body`'s own `hypot`** (`:52`, `:108`) is unchanged.
- It decides only a refusal, and contributes no published value except a frame witness direction.
- That direction is compared base against candidate on the Mac (§4.2).
- It is on the T3-close list.

### 2.7 The adapter's `Geometry` branch: what can reach it (SA)

**`W4Unqualified::Geometry(_)` records an FK refusal of SA-built input.** Every `InvalidInput` cause is unreachable from built evidence:
- **Coordinates are finite.** `EvidenceParts::new` refuses a frame or user element with a non-finite coordinate (its length or axis fails validation). A macro source with a non-finite coordinate has no global matrix, so it matches no slot.
- **Every node of the body is in exactly one sub-body:** `objective_sub_bodies` partitions the body's nodes.
- **Ties have a ≠ b:** a user element whose nodes coincide has no valid orientation, and `EvidenceParts::new` refuses it.
- **The tie graph over the sub-bodies is connected:** the body is connected through frames, curved elements and users, and the sub-bodies are the components of the first two.
- **The ground DOFs are local** by construction.

**`Range("constrained relative coordinates")` is not excluded by this argument.**
- An exact virtual position sums tie offsets along the spanning tree. Each element's own span is finite, but their sum along a path need not be.
- It needs sub-bodies spanning about 1e308, joined by ties that alternate in direction. For a chain of single-node sub-bodies the offsets telescope (v = 0).
- Whether FK's frame and curved formation admit elements that long (their stiffness terms 12EI/L³ and so on underflow at such L) is not claimed here.

**The branch leaves the body to the matrix gate** (today's outcome) and never refuses it (P6). This corrects I14's plan, which called the branch unreachable. At checkpoint C, I14 either constructs the case as a test or records why built evidence cannot reach it.

**A NaN or infinite user stiffness never reaches W4:** `EvidenceParts::new` refuses a non-finite global stiffness ("user stiffness"; `k5_user_elements_tie_only_with_positive_stiffnesses`).

## 3. Refinements (ROOT-approved at A1) and A2's

1. **A1: rationalized candidates in the scaled units [t/L, θ].** B7 (coordinates times 2^±60) showed that physical-unit snapping drops the translation components at 2^-60.
2. **A1: the snap grid** is 2^-20 absolute on k·q, with |k·q| ≤ 64, and each component may move at most 2^-24. The plan said "26 significant bits".
3. **A1: canonical scaling** tries three binary64 forms of k·p_i/p_j, each accepted only when exact.
4. **A1: the `use crate::UserStiffnessElement` import** sits inside the K5 block, keeping `rigid_body.rs:1-248` byte-identical.
5. **A1: two generator bugs, fixed before any vector was committed:**
   - an empty `sum` returned an int, so a float leaked into the exact pipeline;
   - a random sign was drawn per component.
6. **A2: the dense force-scaled selected body** binds `let screened = self.constrained_geometry(…);` then `screened.and_then(…)`, so rustfmt does not re-indent the unchanged closure. The SA diff deletes exactly five lines: the import and the four `geometry` calls.
7. **A2: the screen ratio** is computed in the SA test module only, as an observation (Q2(b)). No product code computes it.

## 4. A2 evidence (records in `_run_records/a2/`)

### 4.1 Tests

| Suite | Result |
|---|---|
| NI (`--no-fail-fast`) | 126 unit + 4 doc tests pass: K-D5's, K1's, K2b's, the s11k pins and K5's 10 |
| FK `s11_site_table`, `k2b_force_scaling` | pass, with no table change (Q9) |
| PP `s11f_site_test` | passes (SA is scanned by rules 2, 3, 5 and 6; K5's SA code has no force token and no value copy) |
| PP `k5_curved_mechanism_runtime` | 3 of 3 |
| PP `formation_check_runtime`, `preview_physics_runtime` | pass |
| PP full suite (instrumented candidate copy, §4.5) | one failure: `s11g_tests::t13_committed_fallback_uz_is_byte_identical`, one of the three known Mac platform tests. Its model is frame-only; the byte-identical failure-block comparison belongs to checkpoint B |

K5's ten SA tests:
- `k5_w4_runs_in_the_four_selected_branches_only`
- `k5_nonlinear_loop_keeps_todays_geometry`
- `k5_first_failing_body_in_seed_order_decides`
- `k5_w4_entry_is_named_only_in_the_four_selected_bodies`
- `k5_user_elements_tie_only_with_positive_stiffnesses`
- `k5_curved_slots_qualify_by_their_matched_source`
- `k5_curved_matching_agrees_with_the_formation_source`
- `k5_force_scaled_branches_give_the_same_witness`
- `k5_screen_ratio_is_recorded_per_element`
- `k5_basis_text_and_family_flag_are_unchanged`

### 4.2 RF-MECH, base against candidate (SA level)

- R1's nine models (`references.py --model`; references.json `7b176dbb…`, references.py `80d473a7…`) were converted by `convert_rfmech.py.txt` to `rfmech_models/`.
- They were run through `geometry` and the four selected entries (b = 0 and b = 2; both modes; dense evidence up to 200 members, sparse always; LINE-IN-CHAIN1000 sparse and SparseInteractive only).
- **133 output lines, byte-identical**: sha256 `2a400707…` (`rfmech_compare.txt`).
- **Statuses:** 8 `Mechanism` (LINE122-TORQUE, -PERP, -UNLOADED, LINE345-RZ, K0, DISC-CHAIN100, DISC-CHAIN100-SPRING, LINE-IN-CHAIN1000) and LINE345-RX-COMPANION restrained, as the brief expected.

### 4.3 The screen ratio per element (observation; `screen_ratios.txt`)

| Element | Ratio |
|---|---|
| The dyadic, arc-consistent elbows (the constructed mechanism at 0, 1,024 m and 5e6 m; the lone bend) | 0.0179, identical at every offset |
| E1 | 0.047 |
| E6's four bends | 0.13 to 0.18 |
| CSKEW_8_5 | 0.020 |
| CSKEW_30_RADIUS_MISMATCH | 7.1e4 |
| CPLANAR_60 | 1.3e5 |
| CSKEW_30_N122 | 2.3e5 |
| PP_UTM_2 | 1.1e5 |

The last four are K-D5's non-dyadic curved controls: their chord mismatch dominates (R5-4). Q2(a)'s screen would have left those bodies unqualified.

### 4.4 The product-run table (F; `product_runs_table.md`)

**Corpus and runs:**
- the constructed curved mechanisms, R = 0.25 m (dyadic) at o = 0, 1,024 and 5e6 m; R = 0.2 m at 0 and 5e6; R = 0.3 m at 0, 1,000 and 5e6; each as mechanism, companion (RX at a) and gap (an open gap support at b);
- PP's `arc_model` (k = 2 and 4; kinked) and REF-B2 lone arcs (k = 1, 2, 4);
- K-D5's PP-route elbows (5e5 and 5e6 m) and the review's UTM elbows (5e6 φ2, 5e6 φ5, 7.3e6 φ10);
- the headless producer cases `curved-tip-weight-full` and `curved-pressure-full` (pressure set to zero, as the headless test does);
- the committed `invented_preview_model.json`;
- each through PP's captured and typed entries, in both modes: 152 runs.

**Classes:** 128 unchanged (byte-identical envelopes), 16 K5-C1, 8 K5-C2, **0 outside K5-C1 and K5-C2**, and **0 changes on a nonlinear-support invocation**.

| Input | Main | Candidate | Class |
|---|---|---|---|
| constructed mechanism, R 0.25, at 0, 1,024 and 5e6 m | `NUMERICAL_INTEGRITY_UNRESOLVED` (pivot), standing unresolved | `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`, standing failed (as frame mechanisms on main), direction θ = (1, 1, 0), u(b) = u(c) = (0, 0, −1) | K5-C1 (12 runs) |
| constructed mechanism, R 0.3, at 1,000 m | unresolved (pivot) | physical mechanism | K5-C1 (4) |
| constructed mechanism, R 0.2 and R 0.3, at 0 m | unresolved (pivot) | unresolved, reason "constrained-body rank unresolved" | K5-C2 (8) |
| every companion, every gap variant, the R 0.2 and 0.3 rows at 5e6 m (refused upstream by the curved-bend radius check), every arc, lone arc, route elbow, review elbow, producer case and the committed model | — | byte-identical | unchanged |

**Why the R 0.2 and R 0.3 mechanisms at the origin are K5-C2:**
- The model is an exact mechanism, but its canonical witness is not representable. u(c) = θ × (c − a) needs 0.2 − 1.2 exactly, which is (2^54 − 1)·2^-54 in binary64 terms, 54 significant bits.
- No k ≤ 64 makes it representable.
- So W4 ends unresolved and never publishes a rounded direction (B5's rule).
- The code and standing equal main's; only the reason text differs.
- At 1,000 m the same geometry's differences are short, and the witness is published (K5-C1).

The candidate's refusal envelopes are identical in both modes: the geometric screen refuses before any mode-specific factorization.

### 4.5 W4's decisions across PP's in-crate suite (`pp_suite_w4_decisions.txt`)

- **Method:** PP's full suite was run once on a scratch copy of the candidate. The copy's only change is an `eprintln` in `W4Context::screen`, which logs each decision with the test's thread name. It is never committed.
- **Result:** W4 ran in 22 tests, all on curved-bend models: b1/b2 arcs, K-D5's route elbows, s11f F8, s11g T15/ruling1/curved thermal, and PP `lib.rs`'s curved tests. **Every decision was `Restrained`**, so each outcome is today's.
- **No user element reached W4.**
  - PP's historical-scope tests with a realized joint C-150 all run requests with nonlinear supports (derived from the demo model's NL-130-FRIC and NL-140), so they are not selected. These are `current_composite_derived_normal_friction_and_reversal`, `expansion_joint_user_stiffness_emits_macro_element_review_rows` and the other historical-premise tests.
  - s11f's F10 joint carries only an axial stiffness. PP does not realize it as an element (`UserStiffnessElement::new` refuses a zero lateral), so its body is frame-only.

## 5. At D

The following are written at checkpoint D:
- the files and line counts;
- each item with the design's words;
- the callers (lexer scan);
- the mutation table;
- T9 and the per-crate suite counts against the Mac baseline;
- gate part 1;
- the interface section for SA, F2a, F3, W1c, K4, V-K, T4 and T5;
- the toolchain and host;
- what was not done;
- CHANGE_RECORD and SHA256SUMS.
