# K5 RETURN (W4: the constrained-body witness and the curved rule)

I14, a Type 2 TASK, for ROOT (HELP_HUMAN). This is the return at checkpoint D, with an addendum (§16) resolving RV14's review and its delta check (§16.7). CHANGE_RECORD.md is the PR record; SHA256SUMS covers every file of `IMPLEMENTATION/K5/` except itself.

**Placeholders:**
- `<wt>` is the T3 worktree root; `<scratch>` is `<wt>/scratch/i14`; `<VENV>` is the repository venv.
- `T3/` is `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`, and `P/` is `projects/chirality-piping/`.
- `FK` is `P/core/solver/frame_kernel`, `SA` is `P/core/solver/nonlinear_integration/src/structural_adapter.rs`, `NI` is `P/core/solver/nonlinear_integration`, `PP` is `P/core/product_physics` and `CB` is `P/core/solver/curved_bend`.

**Platform:** `aarch64-apple-darwin`, rustc 1.97.1. Every product comparison here is Mac against Mac, built from `git archive` copies of Mac main `24dea2dae` and of the candidate. None is compared with a Linux record. T9, the gate part 1 comparison and the product runs are Mac-only comparisons against Mac main.

**Delegation mechanism:** a Claude Code background subagent of ROOT's session, started through the Agent tool and resumed by SendMessage at each checkpoint. ROOT is its parent and only return path. I14 delegated nothing and started no descendant. Its write set and Git limits (no Git writes, no index operations) were the brief's and ROOT's, enforced by instruction; the host enforced only its own file permissions.

**Branch and commits** (`codex/piping-k5-20260928`, from main `24dea2dae`; every commit made by ROOT):

| Checkpoint | Commit | Content |
|---|---|---|
| A1 | `0c732061b` | FK: `assess_constrained_bodies` and its tests |
| A2 | `6bf64f5a9` | SA wiring, SA and PP tests, the product-run table |
| B | `416b0d456` | records: suites, T9, gate part 1 |
| C | `f89662e1f` | mutation table; two tests added (FK's `b9_directional_exactness` case, NI's `k5_frame_only_bodies_keep_todays_witness`); P6 derived |
| D | `b379e5b27` | these records only: RETURN, CHANGE_RECORD, `_run_records/callers.txt`, `_run_records/d/`, SHA256SUMS |
| Addendum | (ROOT) | RV14's four SHOULD-FIX items: FK `publish`'s exactness check (RV14-4); four tests and two vector files; RETURN §16, `_run_records/rv14/` (§16) |

## 1. Scope as ruled

The rulings are ROOT's, in the brief (`T3/TASK_BRIEFS/I14_K5_IMPLEMENTATION.md`, sha256 `a2f61531…`, "ROOT rulings for this slice") and in `ROOT_RULINGS_V1.md`: "K5: spawn and rulings" and "K5: rulings on I14's checkpoint-0 plan". ROOT's later rulings came by message: A1's refinements 1–4 approved; at A2, the P6 correction and K5-C2 approved; B and C verified and committed.

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

### 1.1 The checkpoint-0 positions, as ruled

I14's plan (`<scratch>/CHECKPOINT0_PLAN.md`, sha256 `7f50c788…`) was approved with positions P1–P11, and S1 was resolved by reversing Q7 to (a).

| Position | As ruled and built |
|---|---|
| FK API | `assess_constrained_bodies` assesses one connected body; directional ground rows (Q6); a disconnected tie graph is `InvalidInput` (P7) |
| Reduction | exact tree reduction, six unknowns per body, never a 6S×6S map |
| Screen (Q4(b)) | L a power of two from the bits; `root_one_plus_square` in place of `hypot(ζ, 1)`; τ_B with m = the nonzero rows kept (P3) |
| Witness | exact verification; the canonical representative r = k·p/p_j, smallest k ≤ 64 (P1) |
| Curved (Q2(b)) | qualification by the matched macro source plus coordinate agreement; the screen ratio is evidence only |
| User (Q5(a)) | a tie only with four finite positive stiffnesses and a valid orientation; T4 tripwire |
| Wiring (Q1(b)) | the four selected bodies call the private `constrained_geometry`; every other path passes `w4: None` |
| P2 | the reason string `"constrained-body rank unresolved"` (K5-C2) |
| P4 | one PP tests-only file, `k5_curved_mechanism_runtime.rs` |
| P5 | a matched slot whose moduli do not scale normally at 2^b is still qualified (the geometry does not depend on them) |
| P6 | an FK error on an SA-built body is `Unqualified(Geometry)`: the body goes to the matrix gate. Every cause is unreachable from built evidence (§2.7) |
| P8 | the typed product entry is PP's `run_linear_static_preview_with_mode` |
| P9 | B1: 4,000 generated cases, the first 1,000 committed |
| P10 | `W4Body` and `W4Unqualified` are `pub(crate)`; `TieRefusal` is public |
| P11 | RF-MECH's SA-level statuses established by the A2 harness |
| Q9 | no site-table change |
| S1 → Q7(a) | the basis text is unchanged on every entry; K5-C3 no longer exists |

### 1.2 Each item of the design, with the rulings

The design's words are D1 revision 5a.2 §4.9 and the §6 K5 row (`DESIGN.md`, sha256 `fb62ef4a…`).

| §4.9 | What K5 does |
|---|---|
| "`assess_constrained_bodies(sub_bodies, ties, grounds)` sits beside `assess_rigid_body`" | Added in `FK/src/rigid_body.rs` after line 249, with the coordinates as its first argument (the design's signature omits them). `assess_rigid_body` and `original_rigid_witness` are byte-identical (§2.4). |
| "Objective sub-bodies are node sets joined by straight frames, and by curved elements that pass the screen below" | Frames and curved slots matched to their macro source (Q2(b)); `objective_sub_bodies` forms the components. The screen is evidence only (§4.3). |
| "Each user element imposes u_a = u_b and θ_a = θ_b … because every stiffness is positive" | `user_element_tie`: a tie only with four finite positive stiffnesses (Q5(a)); the T4 tripwire test guards the rule. |
| "Grounds are the restrained DOFs and positive springs, as today" | `ConstrainedGround::Dof` rows as today, plus Q6's `Directional` rows in the FK API. SA passes `Dof` rows only. |
| "The null space of the stacked map uses the existing SVD rank screen and τ_B form. It returns Restrained, MechanismWitnessed … or NumericallyUnresolved" | The same one-sided Jacobi and τ_B rule on the reduced rows, with `hypot` replaced (Q4(b)); the witness exact and canonical; `NumericallyUnresolved` carries "constrained-body rank unresolved". |
| "Proof sketch. Each family's energy is non-negative …" | Written out in §2.1–§2.3. |
| "Curved objectivity screen … If it fails, the body stays unqualified for a witness, with a reason" | Replaced by source qualification (Q2(b)). The reason for an unqualified body is carried by `W4Unqualified`. The ratio is recorded per element (§4.3). |
| "Coordination. T4 should confirm the curved construction's null-space claim" | Not a prerequisite (Q3(a)); notice to T4 (§10). |
| SUP-17 | Delivered by F1a, not K5. |
| §6 row: "A user-element internal mechanism (NP-C-like), a stabilized companion, near-collinear ties, a curved screen positive and a seeded negative" | FK `k5_b2_np_c_like_internal_mechanism_and_its_companion` and `k5_b2_near_collinear_virtual_pins`; SA `k5_curved_slots_qualify_by_their_matched_source` (the positive is a matched slot; the seeded negatives are explicit, unmatched and coordinate-inconsistent slots). |

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
6. **Checked:**
   - the RF-MECH harness (§4.2) is byte-identical base against candidate;
   - `k5_first_failing_body_in_seed_order_decides` pins both seed orders;
   - **`k5_frame_only_bodies_keep_todays_witness` (added at C)** pins today's frame-only witness on the selected branches. It uses a free translation, where today's screen and W4 publish different bits: today's translates by the characteristic length (4.0 exactly on its frame), W4's canonical representative by 1.0. The test asserts both on the same body, so routing frame-only bodies through W4 (mutant K5-M9) fails it (§6).

### 2.5 No invocation with a nonlinear support changes; the contact-seed guard stays closed

1. **PP.** The one call is `assembly.solve_assembled_with_formation_check(…, &curved_sources, built.nonlinear_supports.is_empty())` (`PP:4441-4449`; s11k's pin: one call, in `solve_preview_reduced_system`).
   - With any nonlinear support, `selected` is false. The entry returns `self.solve_assembled(…)` before `constrained_geometry` (the `if !selected` return precedes it; `k5_w4_entry_is_named_only_in_the_four_selected_bodies`).
   - `solve_assembled` calls `geometry` with `w4: None` (§2.4).
2. **The nonlinear loop** calls `solve_binary64` (NI `:1990`), which calls `geometry` with `w4: None`. No NI or PP non-test source names `constrained_geometry`, `W4Context`, `W4Body` or `assess_constrained_bodies` (the text pin).
   - The loop's first solve is bit-equal to `solve_binary64` on a curved-mechanism model whose selected branches refuse it as a W4 mechanism (`k5_nonlinear_loop_keeps_todays_geometry`).
3. **W2's orchestrator** `solve_with_force_scaling` passes `case.selected`; a nonlinear invocation passes false.
   - Before F1b, PP does not call it. After F1b, PP passes the same `selected` (F1b's brief); checked at the merge, with no PP edit by K5.
4. **The contact-seed guard.**
   - `permits_contact_seed_trial` admits any `Mechanism` (`SA:1900` on the base `24dea2dae`; `SA:2142` at the head). A W4 `Mechanism` exists only on a selected branch, that is with `built.nonlinear_supports` empty.
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

**`Range("constrained relative coordinates")` is unreachable from built evidence as well (checkpoint C, P6).** The derivation follows; no test can construct the case.

*Every element that enters a W4 body has a span below 2^513.*
- **Frame (link).** `FrameElement::length` is √(Δ·Δ). If Δ·Δ overflows, the length is +∞ and `FrameProperties::new` refuses it ("length"; SA: "frame local stiffness"). So |Δ| ≤ 2^512.
- **User element (tie).** `orientation` normalizes Δ. If Δ·Δ overflows, the axis is 0, which `normalize` refuses as `DegenerateAxis`, or NaN, which `from_x_axis_and_y_reference` refuses as `NonFiniteInput`. SA refuses both as "user orientation". So |Δ| ≤ 2^512.
- **Curved slot (link, qualified only through its matched macro source).**
  - The constructor's `orientation()` refuses a zero or NaN radial axis at node i, so |r_i|² is finite.
  - The radius-match test does not bound R_j, because ∞ > 10⁻⁹·∞ is false.
  - But R = (R_i + R_j)/2 = ∞ makes every diagonal flexibility entry R·(≥ 0) non-finite. FK's `solve_dense` refuses that matrix, so `global_stiffness()` is Err, `w4_curved_source` matches nothing, and the slot is `CurvedUnmatched`.
  - So a qualified slot has |r_j|² finite too, and its chord is at most |r_i| + |r_j| ≤ 2^513.
- **The bounds hold for W4's coordinates.** SA's `node` refuses a node recorded with two coordinates. W4 refuses a curved source that disagrees with a recorded coordinate (Q2(b)).

*The sums the reduction forms stay far inside the range.*
- **Tree offsets.** s_B = s_A + x_near − x_far adds one tie span per tree tie, so |s| ≤ T·2^512 for T ties.
- **Virtual positions.** v(x) = s + x − o. The two ends of a tie share v, so v is a sum of within-sub-body link displacements along a path from o, and |v| ≤ (N − 1)·2^513.
- **When `Expansion::add` fails.** It returns Err only when a two-sum is non-finite, which needs an exact partial sum of at least 2^1024 − 2^970 in magnitude.
- **Every partial sum here is small enough.** Each is, in magnitude, at most one binary64 coordinate (≤ 2^1024 − 2^971) plus 2(|s| + |v|). The factor 2 covers the nonoverlapping terms of the expansion already held. Reaching the threshold would need 2(|s| + |v|) ≥ 2^970, that is T + 2(N − 1) ≥ 2^457. Element and node counts are `usize`, below 2^64.
- **Rounding.** `round` of v and of the cycle offsets (|v_a − v_b| ≤ 2(N − 1)·2^513) is representable. `exact_rounded_sum`'s accumulator has 64 carry bits, far beyond these term counts.

**The branch stays as written.** It leaves the body to the matrix gate (today's outcome) and never refuses it (P6). The SA comment on `W4Unqualified::Geometry` ("Unreachable from built evidence") now holds for every cause.

**This supersedes A2's "not excluded" paragraph**, which assumed element spans up to about 1e308.

**A NaN or infinite user stiffness never reaches W4:** `EvidenceParts::new` refuses a non-finite global stiffness ("user stiffness"; `k5_user_elements_tie_only_with_positive_stiffnesses`).

### 2.8 The prefilter changes no outcome: the declared-equivalent mutant (plan §16)

**The mutant (K5-PREFILTER).**
- It deletes the row loop of `WitnessContext::try_candidate` (`FK/src/rigid_body.rs:851-859`).
- For each reduced row r, that loop computes `dot = Σ r_i·ŝ_i` in binary64, and returns `None` unless |dot| ≤ fl(2^-20·size).
- Here ŝ = (fl(p₀/L), fl(p₁/L), fl(p₂/L), p₃, p₄, p₅) for the candidate p = [t, θ], and size = ‖ŝ‖∞.
- The guard before the loop (`size` finite and > 0) is not part of the mutant, and stays.

**Claim.** For every candidate that reaches the loop, if `null_translations(p)` returns `Ok(Some(_))`, then every row passes.
- So `try_candidate` returns the same value with and without the loop: a candidate the loop rejects fails `null_translations` anyway, and gives `None` either way.
- `try_candidate` has no side effects.
- `assess_constrained_bodies` returns at the first `Some`, in a candidate order that does not depend on `try_candidate`'s results.
- **So every field of its result is identical.**

**Notation.** ε = 2^-53 and η = 2^-1074. The standard model with gradual underflow:
- fl(ab) = ab(1 + δ) + μ, with |δ| ≤ ε, |μ| ≤ η/2, and μ ≠ 0 only for a subnormal result;
- fl(a ± b) = (a ± b)(1 + δ), exact when the result is subnormal;
- a scaling by a power of two is exact unless the result is subnormal, and then errs by at most η/2 per step.

**Step 1: what an exact null candidate satisfies.** `Ok(Some(_))` means that, as real numbers, with the exact virtual positions v (`Expansion`):
- a `Dof` translation ground (node n, axis k): (t + θ × v_n)_k = 0;
- a `Dof` rotation ground: θ_k = 0;
- a directional translation ground with direction d: d·(t + θ × v_n) = 0;
- a directional rotation ground: d·θ = 0;
- every tie (a, b), cycles included: θ × (v_a − v_b) = 0, since u_a − u_b = θ × (v_a − v_b) exactly.

**Step 2: each computed row is close to an ideal row ρ with ρ·s* = 0, where s* = (t/L, θ).**
- Write w = fl(round(v_n)/L). Since L ≤ M < 2L, |v|/L < 2(1 + ε), and |w_i − v_{n,i}/L| ≤ 2.01ε + η/2 (round is correctly rounded, and v is a multiple of η).
- ŝ differs from s* only in its translation part, by at most η/2 per component.
- Bounds, for the unnormalized row row₀:

| Kind | Ideal ρ (ρ·s* = 0 by Step 1) | Bound on \|row₀·ŝ\| / ‖row₀‖ |
|---|---|---|
| (a) `Dof` translation | `translation_row(v_n/L, k)` | (4.1ε + η)·size + η/2 (‖row₀‖ ≥ 1) |
| (b) `Dof` rotation | row₀ = e_{3+k} | 0: the dot is θ_k = 0 |
| (c) directional translation | [n*, (v_n/L) × n*], n* = 2^-e·d with \|n*\|∞ ∈ [1, 2), \|n̂_i − n*_i\| ≤ η | (72.9ε + 24.3η)·size + 3η. ‖row₀‖ ≥ ‖n̂‖ ≥ 1. Each computed cross component lies within 24.3ε + 7.1η of ((v_n/L) × n*)_j: two products and a subtraction, plus w's and n̂'s perturbations, with \|w\|, \|n̂\| < 2 |
| (d) directional rotation | [0, n*] | 3η·size |
| (e) cycle, k ∈ {0, 1, 2} | [0, c* × e_k], c* = 2^-e(v_a − v_b), \|c_i − c*_i\| ≤ ε\|c*_i\| + η | (8.3ε + 4.2η)·size |

- **The cycle rows need a relative argument,** because ‖row₀‖ can be tiny.
  - Step 1 gives θ = λc* (or θ = 0, when the dot is 0).
  - Then row₀·ŝ = θ·(c × e_k) = e_k·(θ × c) = λ·e_k·(c* × (c − c*)).
  - Over the row's two indices a and b, this is at most |λ|·[2ε|c*_a c*_b| + η(|c*_a| + |c*_b|)].
  - ‖row₀‖ = √(c_a² + c_b²) ≥ max(|c_a|, |c_b|) ≥ η, and |c*_x| ≤ 2.02·max(|c_a|, |c_b|).
  - |λ| ≤ size/0.99, because |c*|∞ ≥ 0.99.

**Step 3: normalization and the dot product.**
- `push_normalized` scales row₀ by a power of two. That is exact, except for entries that fall subnormal when scaled down, which err by at most η.
- It then divides by the computed norm, whose relative error is at most 5ε, and rounds (ε, η/2). So |r·ŝ| ≤ 1.01·|row₀·ŝ|/‖row₀‖ + (2.5ε + 10η)·size.
- The loop's sum of six products adds at most γ₆·Σ|r_i||ŝ_i| + 3.01η ≤ 14.7ε·size + 3.01η, since Σ|r_i| ≤ √6·(1 + 6ε).
- With kind (c) as the worst case, every row satisfies **|dot| ≤ (91ε + 35η)·size + 6.04η**.

**Step 4, case A: size ≥ 2^-1050. The row passes.**
- The threshold satisfies fl(2^-20·size) ≥ 2^-20·size − η/2.
- Since η ≤ 2^-24·size: |dot| + η/2 ≤ (2^-46.4 + 6.54·2^-24)·size < 0.42·2^-20·size.

**Step 5: only families 4 and 5 can have size < 2^-1050.**
- **Family 3 (the unit axes).**
  - A rotation axis has size = 1.
  - A translation axis has ŝ = fl(1/L) = 2^-e ≥ 2^-1023, where L = 2^e ≤ 2^1023. If e < −1023, 1/L = ∞, and the guard returns before the loop.
- **Family 2 (rationalized).** The pivot component of y is ±k, with k ≥ 1.
  - If it is a rotation component, size ≥ 1.
  - If it is a translation component and L ≥ 2^-1022, y·L is exact and normal, so ŝ = y.
  - Otherwise fl(yL) is a nonzero multiple of η, and ŝ ≥ η/L ≥ 2^-52.
- **Family 1 (the SVD column).**
  - The column is I's image under at most 960 binary64 Givens rotations, so its 2-norm is within 2^-40 of 1, and one component is at least 0.4.
  - The family-2 argument then gives size ≥ 2^-53.
  - The exception is L = η with every translation component at most 1/2: these round to 0, so the rotation part carries a component above 0.28.

**Step 6, case B: size < 2^-1050 (families 4 and 5). The dot is exactly 0.**
- **The candidate is exact.** Here t = 0, and θ = round(v_n) (family 4) or round(v_a − v_b) (family 5). Every component is below 2^-1050 and a multiple of η, so round is exact: θ_i = m_i·η with |m_i| < 2^24.
- **What `Ok` implies.** `null_translations` forms `add_product(τ, θ_b)` for every term τ of v_{x,c}, for every c ≠ b and every node x. It returns `Err` unless each product is a sum of two binary64 values, which makes it a multiple of η. So `Ok` implies θ_b·v_{x,c} ∈ ηℤ for every x, and every b ≠ c with θ_b ≠ 0.
- **So θ has one nonzero component.** Suppose θ_b and θ_c (b ≠ c) were both nonzero.
  - Family 4 would give θ_b·v_{n,c} = θ_bθ_c ∈ ηℤ.
  - Family 5 would give θ_b·(v_{a,c} − v_{b,c}) = θ_bθ_c ∈ ηℤ.
  - Both are impossible, since 0 < |θ_bθ_c| < 2^48·η² < η.
  - So θ = θ_j·e_j, ŝ = (0, 0, 0, θ_j·e_j), and the dot is fl(r_{3+j}·θ_j): every other product is exactly 0.
- **Kinds (a), (b), (d) and (e):** Step 1 forces r_{3+j} = 0 exactly, because a zero stays zero through rounding and power-of-two scaling.
  - (a): for k = j the row has no entry in column 3 + j. For k ≠ j, (e_j × v_n)_k = ±v_{n,l} = 0 for the third index l, so w_l = 0, which is the row's entry in column 3 + j.
  - (b): θ_k = 0 forces k ≠ j.
  - (d): d_j = 0.
  - (e): e_j × (v_a − v_b) = 0 forces c's other two components to 0, and those are the entries in column j.
- **Kind (c):** (v_n × d)_j = 0, so after normalization |r_{3+j}| ≤ 1.01·(24.3ε + 7.1η) + η < 2^-47. Then |r_{3+j}·θ_j| < 2^-23·η < η/2, which rounds to 0.
- So dot = 0, which passes: the threshold is ≥ 0.

**Conclusion.** The prefilter rejects no candidate that `null_translations` accepts, so the mutant is equivalent in outcome. The loop is a performance filter only: it skips exact checks of candidates that cannot verify. The plan's §5.3 estimate (about 300ε·size) omitted the underflow terms. Case B shows they cannot matter.

**Observed (supporting evidence, not the proof).**
- K5-PREFILTER ran from a clean archive of `f89662e1f`, after a NONE control on the same base (`_run_records/d/`).
- FK's k5 suites (B1's 1,000 committed records included), NI's library tests and the PP k5 test all pass: it survives, as the derivation predicts.

**Outside the claim: the guard before the loop.**
- When L < 2^-1023, fl(1/L) = ∞, and the guard withholds the three unit translation candidates. Such a body has every virtual coordinate below 2^-1022.
- The rationalized candidates (family 2) still carry translations, in scaled units.
- This is recorded, not changed.

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

## 5. Evidence at B (records in `_run_records/b/`; candidate `6bf64f5a9`)

**Suites (39 manifests, `--no-fail-fast`), against the Mac baseline of the base's product tree** (`suites_candidate.log`, `suites_vs_baseline.diff`):

| Manifest | Base | Candidate | Change |
|---|---|---|---|
| FK | 249 passed | 266 passed | +17: K5's 14 FK integration tests, `k5_scale`, and 2 unit tests in `rigid_body.rs` |
| NI | 120 passed | 130 passed | +10: K5's SA tests at A2 |
| PP | 525 passed, 1 failed | 528 passed, 1 failed | +3: `k5_curved_mechanism_runtime` |
| every other manifest | — | — | unchanged |

- **Failures.** The only failures are the three known Mac platform tests (PP `s11g_tests::t13_committed_fallback_uz_is_byte_identical`, and two headless load-reference tests). Their failure blocks are byte-identical, base against candidate (`failure_blocks_baseline.txt` and `failure_blocks_candidate.txt`).
- **After C.** C added tests only: one NI test, and one FK case inside an existing test. C's NONE control on `416b0d456` plus the C overlay, and D's on `f89662e1f`, passed FK (8 + 14 + 1), NI's library (127) and PP's k5 test (3). The full suites were not re-run after C.

**T9 (Mac-only)** (`t9_compare.txt`):
- Base (`git archive` of `24dea2dae`) and candidate (`git archive` of `6bf64f5a9`) gave 112 outputs each: core 10, fixtures 72, validation 30.
- **112 of 112 are byte-identical.**
- The base outputs also equal `PLATFORM_CALIBRATION_MAC/t9/output_sha256_main_mac_native.txt`.

**Gate part 1 (Q8(b))** (`gate_part1_record.txt`, `gate_part1_compare.log`):
- **Setup.** The candidate was run with the full-envelope probe variant (`main.rs` `cd1052f7…`) under G1's full driver. It was compared per run with G1's full re-run of `e7d930d49`, whose product tree equals the base's.
- **Runs.** 884 of 884 are identical in outcome, ok, exit code, summary sha256, `run.envelope_sha256` and error text.
- **Other rows.** The 24 heap-cap stderr tails are identical. The 764 `gate_check` rows are identical in outcome, quality, standing, trusted and breaches.
- **Result.** `gate_check` PASS: 764 evaluated, 328 trusted, 0 breach triples.
- **Expected.** The corpus realizes no curved bend and no user element, so W4 never runs in it.

## 6. Mutations (C, and D's prefilter run)

**Method.**
- Each mutant ran in its own clean `git archive` copy with its own target, deleted afterwards, with a NONE control first. At most two ran at once, each at `-j 4`.
- **Line numbers are at `b379e5b27`**, the head RV14 reviewed (RV14-N1). C's round 1 ran on the tree before C's pin was inserted, so its logs show NI `k5_tests.rs` lines above 703 32 lower (K5-M13's `:1065` in the log is `:1097` here). FK lines, and NI lines up to 703, are the same in both trees. The addendum (§16) only appends to `k5_tests.rs`, so the NI numbers also hold after it; it inserts lines in `k5_constrained_bodies.rs`'s `verify_witness` and `check`, so the FK numbers are `b379e5b27`'s.
- A kill counts only when the panic is located in test code. A panic in product code, an abort, a signal or a compile error is never counted. There were none of the last three.
- Records: `_run_records/c/` (`c_record.txt`, `MUTANTS.txt`, logs) and `_run_records/d/`.

| Mutant | Site | Killing test (assertion site) | Result |
|---|---|---|---|
| K5-12 (§7.3 item 12) | FK: a tree tie adds no offset (a rigid link) | `k5_b1`, `k5_b2` (both), `k5_b3` … (`k5_constrained_bodies.rs:235`) | killed |
| K5-7 (§7.3 item 7, W4) | SA: W4 branch disabled | `k5_w4_runs_in_the_four_selected_branches_only` (`k5_tests.rs:493`), `k5_first_failing_body…` (`:683`), `k5_nonlinear_loop_keeps_todays_geometry` (`:571`) | killed |
| K5-M1 | FK: cycle rows dropped, cycle ties skipped | `k5_b4_cycles`, `k5_b1` (`:234`) | killed |
| K5-M2 | FK: offset x_b − x_a | `k5_b1`, `k5_b2`, `k5_b3` (`:235`) | killed |
| K5-M3 | FK: stiffness check removed | `k5_c_user_tie_rule` (`:888`); NI `k5_user_elements_tie_only_with_positive_stiffnesses` (`k5_tests.rs:940`) | killed |
| K5-M4a, K5-M4b | SA: explicit or unmatched slot qualified | `k5_curved_slots_qualify_by_their_matched_source` (`k5_tests.rs:1049`) | killed |
| K5-M5 | Q2(a) screen | — | not applicable under Q2(b): no product code computes a screen ratio |
| K5-M6 | FK: exact tie and ground checks skipped | `k5_b5_exactness`, `k5_b9_directional_grounds` (`:249`), `k5_b2` (`:242`) | killed |
| K5-M7 | FK: `Restrained` when no witness verifies | `k5_b5_exactness`, `k5_b2` (`:242`) | killed |
| K5-M8a | SA: W4 in `solve_binary64` | `k5_w4_entry_is_named_only_in_the_four_selected_bodies` (`:867`), `k5_w4_runs…` (`:532`) | killed |
| K5-M8b | SA: dense W4 before `if !selected` | the same two (`:885`, `:532`) | killed |
| K5-M9 | SA: every body through W4 | `k5_frame_only_bodies_keep_todays_witness` (`k5_tests.rs:730`) | survived round 1; killed after C's test |
| K5-M9b | SA: mixed bodies screened after every frame-only body | `k5_first_failing_body_in_seed_order_decides` (`:683`) | killed |
| K5-M10 | SA: curved edges objective | `k5_basis_text_and_family_flag_are_unchanged` (`:1232`), `k5_curved_slots…` (`:1062`) … | killed |
| K5-M11 | FK: `zeta.hypot(1.0)` | `k5_b10_libm_free_source_scan` (`:858`) | killed |
| K5-M12 | SA: coordinate check removed | `k5_curved_slots…` (`:1049`) | killed |
| K5-M13 | SA: force-scaled match on b = 0 bits | `k5_curved_matching_agrees_with_the_formation_source` (`:1097`), `k5_force_scaled…` (`:493`) | killed |
| K5-M14a, K5-M14b | FK: grounds or ties not sorted | `k5_b6_order_independence` (`:511`) | killed |
| K5-M15 (own) | FK: L = M | `k5_b1`, `k5_b2`, `k5_b4` (`:165`) | killed |
| K5-M16 (own) | FK: zero rows kept | `k5_b1` (`:235`), `k5_b4` (`:234`) | killed |
| K5-M17 (own) | FK: directional translation row in binary64 | `k5_b9_directional_grounds` (`:249`), through C's constructed case `b9_directional_exactness` | killed |
| K5-M18 (own) | FK: canonical scaling dropped | `k5_b1`, `k5_b5`, `k5_b9` (`:260`) | killed |
| K5-M19 (own) | FK: rationalized candidates removed | `k5_b1` (`:235`), `k5_b7_power_of_two_invariance` (`:524`) | killed |
| K5-PREFILTER (declared equivalent) | FK: the prefilter's row loop removed | — | survives, as §2.8 derives (run at D, supporting only) |
| KD5-SELECT | SA: dense `if !selected` removed | `k1_tests.rs:405`, `k2b_tests.rs:437`, `k5_tests.rs:885`, `:532` | killed |
| K1-PIN-BINARY64, -B | SA: exact entry in `solve_binary64` | `s11k_tests.rs:1478`, `:807`, `:577` / `:595` | killed |
| K1-PIN-THIRD | SA: third formation-checked definition | `s11k_tests.rs:1141` | killed |
| K1-PIN-LOOP, -B | NI loop through a formation-checked entry or helper | `k1_tests.rs:997` / `:1000`, `s11k_tests.rs:1280`–`:1283`, and for -B `k5_tests.rs:613` | killed |
| KD5-M32a, KD5-M32b, KD5-E4 | K-D5's loop and unit-force pins | `k1_tests.rs:1000`, `s11k_tests.rs:1141`–`:1498` | killed |

**K5-M9: why it survived round 1.**
- Under M9, W4 screens every body. On round 1's tests, every frame-only body gave the same bits either way.
- One example is the 3-4-5 line's rotation witness (0, 0, 0, 3, 4, 0). W4 reaches it through its canonical k = 3, and today's screen through its unnormalized-difference candidate.
- A free translation separates the two, and C's pin uses one (§2.4, item 6).

**The original pins' mutants keep their kill sites.** The K1 and K-D5 mutants are killed at the s11k and k1 test sites, as in K1's table. Their panics in NI `lib.rs` are the loop's own `expect` sites under a rerouted solve, and are not counted.

## 7. Q10: zero and subnormal curved entries (observation; `_run_records/d/q10_*`)

**Method.**
- A scratch copy of `f89662e1f` was instrumented in `EvidenceParts::new` (`q10_instrument.py.txt`; never committed).
- For each curved slot, the copy printed its count of exactly zero and of subnormal global-matrix entries, and its smallest nonzero |entry|.
- It was run over NI's `k5_tests` (the SA corpus, K-D5's curved models included) and over A2's product-run corpus (`k5_product_runs.rs.txt`).
- `q10_curved_entries.txt` lists every distinct record: 44 from NI and 31 from PP.

**Result.**
- **No curved entry is subnormal.** The smallest nonzero |entry| is 1.48e-9, in PP's kinked arc.
- **65 records have exactly 72 zero entries of 144.** These are bends in a coordinate plane, where the product's element decouples the in-plane DOFs (u_x, u_y, r_z at each node) from the out-of-plane ones (u_z, r_x, r_y). 6 × 6 × 2 = 72 coupling entries are exactly zero. These zeros are structural, not underflow.
- **10 records have no zero entry.** These are skewed bends: K-D5's CSKEW, CPLANAR and PP_UTM controls, and the kinked arc.
- The curved-formation range item stays on the T3-close list (Q10(a)).

## 8. Callers (`_run_records/callers.txt`; `scan_callers_k5.py.txt`)

**The scan.**
- It is K1's lexer scan, with K1's lexer, `#[cfg(test)]` mask and output format unchanged, and K5's patterns. It covers every `.rs` file under `P/`, outside `execution/`, `target/` and `node_modules/`.
- It finds 122 call sites, 41 of them non-test.
- K5's FK-private stages are counted inside `rigid_body.rs` only, because their method names are generic.

**Non-test callers of each added or changed function:**

| Function | Non-test callers |
|---|---|
| `assess_constrained_bodies` | 1: `W4Context::body` (SA) |
| `user_element_tie`, `objective_sub_bodies` | 1 each: `W4Context::body` |
| `constrained_geometry` (dense and sparse) | 4: the `selected` bodies of `solve_assembled_with_formation_check` and `solve_force_scaled_with_formation_check`, dense and sparse |
| `BodyEvidence::geometry` (W4's branch) | 4: `AssemblyEvidence::geometry`, `SparseAssemblyEvidence::geometry` (with `w4: None`) and the two `constrained_geometry` |
| `AssemblyEvidence::geometry` | 4: `solve`, `solve_binary64`, `solve_assembled`, `solve_force_scaled` |
| `SparseAssemblyEvidence::geometry` | 3: `solve`, `solve_assembled`, `solve_force_scaled` |
| `W4Context::screen` | 1: `BodyEvidence::geometry` |
| `W4Context::body` | 1: `W4Context::screen` |
| `w4_curved_source` | 1: `W4Context::body` |
| `solve_assembled_with_formation_check` (changed body) | 1: PP `solve_preview_reduced_system` (`PP:4441`) |
| `solve_force_scaled_with_formation_check` (changed body) | 2: SA `evaluate_force_scaled`, for the dense and the sparse evidence |
| `solve_with_force_scaling` | 0: tests only, until F1b |
| `assess_rigid_body` (unchanged) | 1: `BodyEvidence::geometry`; `original_rigid_witness`: 1, `assess_rigid_body` |

- **Nothing outside FK and SA calls a new FK function.** PP reaches W4 only through `solve_assembled_with_formation_check` at `PP:4441`, with `selected = built.nonlinear_supports.is_empty()`.
- The NI loop reaches only `solve_binary64`, which passes `w4: None` (§2.5).

## 9. Interface for SA, F2a, F3 (W1b), W1c, K4, V-K, T4 and T5

### 9.1 FK: `open_pipe_stress_frame_kernel::rigid_body`

This is FK's `pub mod rigid_body` (`FK/src/lib.rs:9`). A consumer needs no `pub use`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroundKind { Translation, Rotation }                        // rigid_body.rs:265

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstrainedGround {                                          // :274
    Dof(usize),                                                       // 6·node + dof, local
    Directional { node: usize, kind: GroundKind, direction: [f64; 3] },
}

// The addendum (RV14-4): the named unresolved reasons.
pub const CONSTRAINED_RANK_UNRESOLVED: &str = "constrained-body rank unresolved";          // :290
pub const CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE: &str =
    "constrained-body witness parameters not representable";                              // :294

#[derive(Debug, Clone, PartialEq)]
pub struct ConstrainedAssessment {                                    // :301
    pub status: RigidBodyStatus,          // never UnqualifiedFamily
    pub singular_values: [f64; 6],
    pub rank_screen: f64,                 // τ_B = 64·γ(max(m, 6))·σ_max
    pub rows: usize,                      // m, the nonzero reduced rows
    pub cycles: usize,                    // non-tree ties
    pub characteristic_length: f64,       // L, a power of two
    pub origin: [f64; 3],                 // local node 0
    pub rigid_parameters: Option<[f64; 6]>,   // [t/L, θ] of the witness, exact (RV14-4)
    pub node_motion: Option<Vec<[f64; 6]>>,   // exact [u, θ] per local node
    pub iterations: usize,
    pub unresolved: Option<&'static str>,     // Some exactly when NumericallyUnresolved (RV14-4)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TieRefusal { Stiffness(&'static str), Orientation }         // :329

pub fn user_element_tie(element: &UserStiffnessElement)
    -> Result<[usize; 2], TieRefusal>;                                // :344
pub fn objective_sub_bodies(node_count: usize, links: &[[usize; 2]])
    -> Result<Vec<Vec<usize>>, StructuralError>;                      // :365
pub fn assess_constrained_bodies(
    coordinates: &[[f64; 3]],
    sub_bodies: &[Vec<usize>],
    ties: &[[usize; 2]],
    grounds: &[ConstrainedGround],
) -> Result<ConstrainedAssessment, StructuralError>;                  // :478
```

**The contract of `assess_constrained_bodies`.**
- **Input.** One connected constrained body, in local node indices:
  - finite coordinates;
  - `sub_bodies`, a partition of 0..n into non-empty sets;
  - ties with a ≠ b;
  - a tie graph that is connected over the sub-bodies;
  - valid grounds: a `Dof` below 6n, and a `Directional` direction that is finite and nonzero.
  - Anything else is `InvalidInput("constrained geometry")`.
- **Range.** An exact difference or virtual position outside binary64 is `Range("constrained relative coordinates")`. It is unreachable from SA-built evidence (§2.7), but reachable through direct FK calls.
- **Order.** Input order never matters: status and bits depend on the content only (B6).
- **Output.**
  - `MechanismWitnessed` carries the exact, canonical witness, and `rigid_parameters` = [t/L, θ] exactly (`rigid_parameters[i]·L` is node 0's u_i bit for bit).
  - A witness whose [t/L, θ] is not exactly representable (t/L overflows at a tiny L, or rounds at a huge one) is refused: `NumericallyUnresolved` with `unresolved = CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE`, nothing published (the addendum, RV14-4).
  - `NumericallyUnresolved` with `CONSTRAINED_RANK_UNRESOLVED` means the rank is in the τ_B band, or no candidate verified. It is never a rounded direction.
  - **The first verified witness decides** (RV14-D2, accepted as a conservative limitation; §16.7). If its [t/L, θ] is not exact, the result is `NumericallyUnresolved` with the parameters reason, and no later candidate is tried, even one that would publish exact parameters.
  - `unresolved` is `Some` exactly when the status is `NumericallyUnresolved`. SA publishes its own reason text for both ("constrained-body rank unresolved"), unchanged.
  - **`Restrained` is not a restraint proof.** A caller still runs its matrix gate, as SA does.
- **Platform.** The function calls no function of unspecified precision (§2.6). Its output is a function of the input bits on every IEEE-754 platform.
- **Directional rows.** The exact check uses `direction` as given. The screen uses it scaled by a power of two.

### 9.2 SA (NI's `structural_adapter`, crate-private)

```rust
struct BodyEvidence<'e> { …, w4: Option<W4Context<'e>> }             // SA:1249; `None` except in the four selected bodies
struct W4Context<'e> {                                                // SA:1347
    formation: &'e FormationPrimitives,
    curved_sources: &'e [CurvedBendMacroElement],
    force_scale: ForceScale,
}
pub(crate) enum W4Body {                                              // SA:1356
    Unqualified(W4Unqualified),
    Assessed { nodes: Vec<usize>, assessment: ConstrainedAssessment },
}
#[allow(dead_code)] // F2a API: the reason carrier; read by tests today.
pub(crate) enum W4Unqualified {                                       // SA:1368
    UserTie { element: usize, refusal: TieRefusal },
    CurvedExplicit { element_id: String },
    CurvedUnmatched { element_id: String },
    CurvedCoordinates { element_id: String, node: usize },
    Geometry(StructuralError),
}
impl W4Context<'_> {
    pub(crate) fn body(&self, evidence: &BodyEvidence<'_>, body: &[usize],
                       prescribed: &[(usize, f64)]) -> W4Body;
}
// On AssemblyEvidence (SA:146) and SparseAssemblyEvidence (SA:575), private:
fn constrained_geometry(&self, prescribed: &[(usize, f64)],
                        curved_sources: &[CurvedBendMacroElement]) -> Result<(), StructuralError>;
```

No public SA signature changed.
- `W4Body` and `W4Unqualified` are `pub(crate)` (P10). F2a adds a public accessor if it publishes the reason.

### 9.3 For F2a: the published outcomes

- **A witnessed mechanism** is `StructuralError::Mechanism { direction }`.
  - `direction` has length 6·(node count). The body's nodes carry their exact [u, θ], and every other entry is 0.
  - PP maps it to `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM` (`PP:1272`), and its message renders the direction through `Debug`.
- **An unresolved body** is `NumericallyUnresolved { reason: "constrained-body rank unresolved", global_dof: None }`, which PP maps to `NUMERICAL_INTEGRITY_UNRESOLVED`.
- **The basis text** is unchanged (Q7(a)). On the selected branches it under-claims for mixed bodies that W4 assessed. Correcting it is on F2a's input list and the T3-close list.
- **The unqualified reason** is `W4Unqualified`. It is not published today.

### 9.4 For W1b (F3's user matrices) and W1c (curved elements): geometry first (§4.1.3)

- **Before any factor,** build the call per connected body:
  - `sub_bodies` = `objective_sub_bodies(n, links)`, over frames and qualified curved elements;
  - `ties` = the user elements that `user_element_tie` accepts;
  - `grounds` = the `Dof` rows (restrained, prescribed, positive springs), plus a `Directional` row per skew support or spring axis.
- **Outcomes.** `MechanismWitnessed` refuses before any attempt, with the direction. `Restrained` and `NumericallyUnresolved` proceed to the method (K4's rule for frames).
- **W1b's limit.** `user_element_tie` is for today's element only (Q5(a)). A general user matrix (F3) is a tie only when its null space is exactly the tie space. Otherwise F3 must leave the body unqualified, or supply its own qualification.
- **W1c's limit.** The curved rule qualifies a slot by its matched macro source, and derives the refusal's soundness for the intended element (§2.3). If W1c re-forms curved elements (T4's objective element, for example), the source-match predicate (`w4_curved_source`, K-D5's) must follow the new formation.

### 9.5 For K4: the directional-row follow-up (T3-close list)

- **What `geometry_first` would pass,** per frame-only body:
  - `sub_bodies = [all body nodes]`;
  - `ties = []`;
  - `Dof` rows as today;
  - `Directional { node, kind, direction }` for each skew support or spring axis, with the global binary64 axis as given.
- This lets W4 decide the non-spanning directional grounds that O1 now sends on as `NumericallyUnresolved`.
- **The caveat: frame-only bodies would change behaviour.** Moving them from `assess_rigid_body` to this function changes witness bits, as K5-M9 showed. Today's witness translates by the characteristic length, and W4's canonical representative by k ≤ 64. Statuses near the τ_B band may also change, because the rows and the screen differ.
- So K4's follow-up either keeps `assess_rigid_body` for bodies without directional rows, or accepts the change as a declared class. K5 does not write K4's files.

### 9.6 For V-K

- RF-MECH has no user or curved mechanism, so W4 has no frozen reference. Ruling item 16 records this.
- V-K can reuse the generator (`FK/tests/k5_constrained/gen_k5_vectors.py --check`), whose exact oracle is the unreduced stacked map in `Fraction`.

### 9.7 For T4 and T5

See §10.

## 10. Notices

**To T4** (the M07 joint repair, and the curved null-space item).
- **The user element.** The tie rule is for today's user element: energy Σ k_d(Δ_d)², with four positive stiffnesses and no rigid-body moment coupling.
  - M07's repair changes the local form, and with it the zero-energy set.
  - The FK test `k5_t4_tripwire_user_tie_space_is_the_represented_null_space` then fails.
  - `user_element_tie` must be revised with the repair: a repaired element is a tie only if its null space is still exactly the tie space.
- **The curved element.** The curved rule is stated and derived for the intended element (§2.3; Q3(a)). T4's item stays open for the product's element, whose non-objectivity (the chord mismatch, R̃'s departure from orthogonality, and the formation rounding) is the only source of energy along a W4 witness (§2.3, Step 5).
  - If T4 changes the curved construction, K-D5's source match must follow it (§9.4).

**To T5** (the nonlinear loop).
- The loop's geometry is unchanged (Q1(b)), and the contact-seed guard stays closed (§2.5).
- **If the loop adopted W4** (curved sources and a W4 context in its geometry):
  - a mixed-body mechanism in an iteration would become a `Mechanism`;
  - `permits_contact_seed_trial` (`SA:2142`) admits every `Mechanism`, so the contact-seed trial would open at NI `lib.rs:648-652` (with `recovery_eligible` and an inactive support);
  - PP's arm at `PP:2884` would open when `eligible_contact_dofs` is `Some`.
- K5-C2's reason, "constrained-body rank unresolved", is not in the guard's admitted list, and a mixed family is not qualified. So an unresolved W4 outcome would not open the trial.
- Adopting W4 in the loop is T5's decision.

**To K4:** §9.5. **To F2a:** §9.3. **To F3 and W1c:** §9.4.

**To F1b** (merge order, from both briefs).
- Whichever of K5 and F1b merges second merges main, then re-runs its suites, T9, and its affected tables: for K5, the curved product-run table and gate part 1.
- After F1b, PP reaches W4 through the sparse sibling and W2's orchestrator, with the same `selected`. F1b refuses a realized curved bend at b ≠ 0, so in the product W4's force-scaled branches run at b = 0 only.

**Added to the T3-close list by K5's findings** (for ROOT):
- the basis text's under-claim on the selected branches (S1);
- `assess_rigid_body`'s `hypot`, already listed;
- K4's directional wiring, already listed, with §9.5's caveat;
- the guard before the prefilter, which withholds unit translation candidates when L < 2^-1023 (§2.8), recorded only;
- RV14-D2 (§16.7): a refused witness ends the candidate search. ROOT accepts this as a conservative limitation and lists it as a candidate refinement.

## 11. Files and line counts (against `24dea2dae`)

| File | Lines | Diff |
|---|---|---|
| `FK/src/rigid_body.rs` | 1,386 | +983 (lines 1–249 unchanged) |
| `SA` (`structural_adapter.rs`) | 3,249 | +253 −5 |
| `NI/src/structural_adapter/k5_tests.rs` (new) | 1,258 | +1,258 |
| `FK/tests/k5_constrained_bodies.rs` (new) | 1,001 | +1,001 |
| `FK/tests/k5_scale.rs` (new) | 131 | +131 |
| `FK/tests/k5_constrained/gen_k5_vectors.py` (new) | 758 | +758 |
| `FK/tests/k5_constrained/b1_sample.txt`, `b1_summary.txt`, `cases.txt`, `SHA256SUMS` (new) | 1,000, 11, 27, 4 | +1,042 |
| `PP/tests/k5_curved_mechanism_runtime.rs` (new) | 228 | +228 |

- **Totals.** 11 product and test files, +5,654 −5.
- **Records.** 164 files under `T3/IMPLEMENTATION/K5/` at C, before D's records.
- **Not changed:** no dependency or lockfile; neither site table; `FK/src/structural.rs`, `FK/src/lib.rs`, CB, NI `lib.rs`, PP source, `s11k_tests.rs` and `s11f_site_test.rs`.
- **Warnings.** The non-test build has no warnings. The one `#[allow(dead_code)]` K5 adds is on `W4Unqualified` ("F2a API").

## 12. Toolchain and host

| Item | Value |
|---|---|
| Toolchain | rustc 1.97.1 (`8bab26f4f`, 2026-07-14), cargo 1.97.1, host `aarch64-apple-darwin`; `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, `RUST_TEST_THREADS=4` |
| Python | `<VENV>`, Python 3.13, standard library only (the generator, the scans, the drivers) |
| Host | the owner's Mac (macOS 26.6.2, 128 GB, no swap), shared with I12 (K4) and I13 (F1b) under I8R's caps |
| Memory guard | `<wt>/guard/memguard.log` unchanged across every heavy phase: 2 start lines, sha256 `79e2ce8e…` |
| Timing | none compared |

## 13. What was not done

- **Excluded by the brief and the rulings:**
  - no change to `assess_rigid_body` or `original_rigid_witness`;
  - no change to the edges' objective flag, `qualified_passive_family`, exact-block eligibility or the basis text (Q7(a));
  - no field on `StructuralReport` or `StructuralSolution`;
  - no new `StructuralError` variant;
  - no loop change;
  - no SUP-17 text;
  - no curved formation-range work (Q10(a));
  - no T4 repair;
  - no K4 wiring;
  - no site-table change (Q9(a)).
- **No native witness:** the desktop cannot realize a curved bend (R5-4), and no build realizes a user element.
- **Not re-run after C** (tests-only changes; §5): the full 39-manifest suites, T9 and gate part 1. C's and D's NONE controls ran FK's, NI's and PP's K5 suites.
- **Hosted CI and the independent review** belong to ROOT's PR.

## 14. Disclosures

- **Raw outputs kept verbatim** (a scan of every file under `IMPLEMENTATION/K5/` finds these four, and no others):
  - `_run_records/a2/checkpoint0_probe_main.txt` keeps its trailing whitespace;
  - `_run_records/c/logs/C-VERIFY/ni5.txt` keeps a blank line at the end of the file;
  - `_run_records/b/gate_part1_record.txt` has one line with trailing whitespace (the gate run's "end" line);
  - `_run_records/b/gate_part1_result_candidate.json` has no final newline, as the gate check wrote it.
  - C-VERIFY is the worktree run of the new NI test before it joined the mutation overlay. It ran against the working tree, not a clean copy.
- **Machine paths replaced.** Every committed log has its machine paths replaced by `<wt>`, `<VENV>` or `<wt>/k5/T3`. Apart from that, the logs are the tools' output.
- **Instrumented scratch copies were never committed:**
  - A2's `eprintln` in `W4Context::screen` (§4.5);
  - D's in `EvidenceParts::new` (§7).
  - Only their scripts and outputs are records.
- **The P6 path.** The plan called the `Geometry` branch unreachable. A2 corrected that to "not excluded" for the range cause. C derived it unreachable (§2.7), and the derivation supersedes A2's paragraph.
- **The prefilter's equivalence** is derived in §2.8, not merely declared. The D run is supporting evidence only.
- **Q10's observation was not recorded before D.** The plan listed it among the product-run records, and it was taken at D (§7).
- **The mutation logs show product panics.** The K1 and K-D5 pin mutants panic in NI `lib.rs` as well as at test assertions. Only the test assertions are counted.
- **Wider reading.** I14 read other slices' records (K1's, K2b's and K-D5's RETURN, CHANGE_RECORD and `_run_records/`) for conventions, and reused K1's caller scanner and mutation script. I14 consulted no other role's instructions.

## 15. Records (`_run_records/`)

| Folder | Contents |
|---|---|
| `a2/` | the checkpoint-0 probe and its raw log; the RF-MECH harness and comparison; the product-run harness, both raw outputs and the table; the screen ratios; W4's decisions across PP's suite |
| `b/` | suites and manifests; failure blocks; T9 hashes and comparison; gate part 1 index, record, logs and result |
| `c/` | the mutation tooling, `MUTANTS.txt`, `c_record.txt` and every log |
| `d/` | the prefilter mutant with its NONE control; D's mutation tooling; Q10's instrument, summary and raw logs |
| `callers.txt`, `scan_callers_k5.py.txt` | the caller scan (§8) |
| `rv14/` | the addendum (§16): targeted test logs; the mutation tooling, RV14's patch script, `MUTANTS_RV14.txt` and every log; RV14's FK probe with the corpus comparison and RV14's oracle summary |

`IMPLEMENTATION/K5/SHA256SUMS` lists every file under `IMPLEMENTATION/K5/` except itself, as `./<path>`.

## 16. Addendum: RV14's review (PASS at `b379e5b27`; 0 BLOCKING, 4 SHOULD-FIX, 5 NOTEs)

**Basis.** RV14's review is `T3/REVIEW/K5_REVIEW.md`, with `REVIEW/_run_records/k5_review/`, committed on the numerics branch at `3a17799e4`. ROOT directed that all four SHOULD-FIX items be resolved, with RV14-M1, M2 and M4 and a new mutant re-killed from clean archives. Records are in `_run_records/rv14/`.

| Finding | Resolution | Pinned by | Mutant (clean archive) |
|---|---|---|---|
| RV14-1: the exact tie loop, the only guard against a cycle in the τ_B band (test gap) | two FK cases from RV14's construction | `k5_b4_cycles` | RV14-M1 killed at `k5_constrained_bodies.rs:256` |
| RV14-2: agreement of two curved sources at a curved-only node (test gap) | an SA test on RV14's P1 | `k5_curved_sources_agree_at_a_curved_only_node` | RV14-M2 killed at `k5_tests.rs:1308` |
| RV14-3: K5-C2's published reason (test gap, product-reachable) | a PP test on `constructed_mechanism_r0.2_o0` | `k5_curved_mechanism_without_a_representable_witness_is_unresolved` | RV14-M4 killed at `k5_curved_mechanism_runtime.rs:262` |
| RV14-4: `rigid_parameters` = +∞ at subnormal spans (FK API defect) | FK `publish` refuses a witness whose [t/L, θ] is not exact, with a named reason | `k5_b5_parameters_are_exact_or_refused`, and RV14's tiny corpus | K5-M20 killed at `k5_constrained_bodies.rs:1081` |
| RV14-N1: stale line numbers | §6 renumbered to `b379e5b27` and stated; §2.5 cites the base and the head lines | — | — |
| RV14-N2: spring grounds killed only by `unwrap` panics | an SA test on RV14's P2 (cheap) | `k5_positive_springs_ground_w4_bodies` | RV14-M3 now also killed at `k5_tests.rs:1328` (an assertion) |
| RV14-N3, N4 | no action | — | — |
| RV14-N5: main moved to `df6d59e3c` (app-v4 only) | ROOT merges main after committing the addendum; the piping tree is unchanged | — | — |

Line numbers in this table are those of the addendum's tree.

### 16.1 RV14-1: a cycle in the τ_B band

- **The cases.** `gen_k5_vectors.py` adds `b4_cycle_band_rv14_0` and `b4_cycle_band_rv14_4` to `cases.txt`: RV14's `T1_cycle_band_0` and `T1_cycle_band_4`, bit for bit.
  - Nodes (0,0,0), (1,0,0), (0,1,0) and (1−ε, 1, −1), with ε = 9u and 901u (u = 2^-53).
  - Sub-bodies {0,2} and {1,3}; tree tie 0-1; cycle tie 2-3, whose exact offset is c = (ε, 0, 1).
  - Grounds: node 0's translations, RX and RY.
- **Expectation `U`, from the generator's exact oracle.** The unreduced map has nullity 0: the cycle restrains the rotation about z.
- **What `k5_b4_cycles` asserts:**
  - `NumericallyUnresolved`, with `CONSTRAINED_RANK_UNRESOLVED`;
  - 8 rows and 1 cycle;
  - σ_min ≤ τ_B (the band).
- **Why it pins the tie loop.** The candidate θ = e_z passes the prefilter, since |θ × ĉ| ≈ ε. Without the exact tie loop, the function publishes it: RV14-M1 fails the first case with `MechanismWitnessed` against `NumericallyUnresolved`.
- **The other corpus tests pass on both cases:** B6 (order) and B7 (2^±k scaling; the rows are scale-invariant at normal scales).

### 16.2 RV14-2: two curved sources at a curved-only node

- **The model** is RV14's P1: two matched bends, A from node 0 (0,0,0) to node 1 (0.25, 0.25, 0), and B from node 1 (0.5, 0.25, 0) to node 2 (0.75, 0.5, 0). Translation pins at nodes 0 and 2, and a load at node 1.
- **What the test asserts:**
  - the evidence records no coordinate for any node;
  - `W4Context::body` gives `Unqualified(CurvedCoordinates { element_id: "bend-1", node: 1 })`;
  - every selected entry (dense and sparse, unscaled and force-scaled, both modes) equals its unselected sibling and is never a `Mechanism`.
- **Under RV14-M2** the body is `Assessed`, with a false `MechanismWitnessed`, and the first assertion fails.

### 16.3 RV14-3: K5-C2's published reason

- **The model** is I14's product input `constructed_mechanism_r0.2_o0`. PP's test model gains a radius parameter and the harness's second load case (`model_with`); the three existing tests' inputs are byte-identical.
- **What the test asserts, on both entries and in both modes:**
  - `MODEL_INCOMPLETE`;
  - every blocking diagnostic is `NUMERICAL_INTEGRITY_UNRESOLVED`, and its message contains `NumericallyUnresolved { reason: "constrained-body rank unresolved", global_dof: None }`;
  - no result rows.
- **Under RV14-M4** the outcome reverts to Mac main's ("nonpositive or cancellation-unresolved structural pivot"), and the reason assertion fails in the first run (captured, SparseInteractive).

### 16.4 RV14-4: `rigid_parameters` is exact, or the witness is refused

**The fix** (`FK/src/rigid_body.rs`, `WitnessContext::publish`; RV14 recommended "publish the field only when every r_i/L is finite and exact").
- `publish` forms t_i = fl(r_i/L) for the three translation components.
- It publishes only if, for each i, t_i is finite and fl(t_i·L) = r_i.
- Otherwise the status stays `NumericallyUnresolved`, with the new field `unresolved = Some(CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE)` ("constrained-body witness parameters not representable"), and nothing is published.
- `ConstrainedAssessment` gains `pub unresolved: Option<&'static str>`. It is `Some` exactly when the status is `NumericallyUnresolved`: the new reason, or `CONSTRAINED_RANK_UNRESOLVED` for the τ_B band or no verified candidate. Both reason strings are public consts.
- The SA mapping is unchanged: SA reads only `status` and `node_motion`, and publishes "constrained-body rank unresolved" for either reason.
- I chose the refusal, not ROOT's alternative of publishing the parameters in a form that is always exact. The field's documented meaning, [t/L, θ] as `assess_rigid_body` publishes it, then stays unchanged for every published witness.

**Why the test is exact.** L = 2^e is a power of two.
- **L ≥ 1** (a quotient scaled down):
  - if r_i/L is exact, t_i·L = r_i exactly;
  - if the quotient rounds (subnormal), t_i·L is an exact scaling up of a different value, so it differs from r_i;
  - if it underflows to 0 (r_i ≠ 0), 0·L = 0 ≠ r_i.
- **L < 1** (a quotient scaled up):
  - r_i/L is exact unless it overflows, and ∞ fails the finiteness test;
  - if it is exact, t_i·L = r_i, because the product's exact value r_i is representable.
- **So the test passes exactly when [t/L, θ] is r scaled exactly.**
- **The θ components are never divided.**

**Mutant K5-M20** removes the check (`if false && …`). It publishes `MechanismWitnessed` with +∞ on RV14's minimal case, and is killed at the test's first assertion (`k5_constrained_bodies.rs:1081`).

**The control: RV14's tiny-coordinate sweep.**
- **The corpus.** `gen_k5_vectors.py` ports RV14's `tiny_case` (seed 1403). Its 1,500 cases are **byte-identical to RV14's `cases_tiny.txt`**, checked in scratch.
- **The minimal case.** RV14's `H_tiny_free_x` precedes them, as `rv14_h_tiny_free_x`: two nodes 2^-1070 apart, grounds d1–d5.
- **Expectations**, from the same exact oracle plus the function's L, taken from the exact virtual positions rounded to binary64:
  - `N`: nullity 0;
  - `D`: nullity 2 or more;
  - `U`: nullity 1 with no representable canonical motion;
  - `M`: nullity 1, with a representable canonical witness and representable parameters;
  - **`P` (new):** nullity 1, with a representable canonical witness whose t/L is not exact.
- **What is committed.** `subnormal.txt` holds the first 301 records. `subnormal_summary.txt` holds the full set's counts and sha256, and `--full-subnormal` writes the full set.
- **What the test asserts:**
  - the minimal case directly: `NumericallyUnresolved` with the named reason, L = 2^-1070, and neither field published;
  - `check()` on every record;
  - that the test's `verify_witness` now asserts finite parameters. Its power-of-two check on L now also admits a subnormal L, which the corpus reaches.
- **Results** (`_run_records/rv14/tests/fk.txt`):

| Set | P→U | M→W | D→W | D→U | N→R | N→U | U→U |
|---|---|---|---|---|---|---|---|
| Sample (301 records) | 24 | 25 | 55 | 47 | 134 | 1 | 15 |
| Full (1,501 records; `K5_SUBNORMAL_VECTORS`) | 146 | 127 | 282 | 277 | 583 | 4 | 82 |

**The fix against RV14's whole corpus** (`_run_records/rv14/probe/comparison.txt`).
- RV14's own FK probe (unchanged) ran on RV14's 4,226 cases in the addendum's clean copy.
- **Exactly the 349 head witnesses that published a non-finite component change,** `MechanismWitnessed` → `NumericallyUnresolved`. Only the status, `rigid_parameters` and `node_motion` columns differ.
- **Every other result is byte-identical** to RV14's head results (`862d4614…`).
- **RV14's oracle on the new results:** 0 findings, 0 false witnesses, 0 non-finite parameters.

**Reach.** The fix changes an outcome only for a witness whose t/L overflows or rounds.
- **SA-built bodies.** L ≥ 2^-72 for any body with a frame link (span above 10^-12) or a matched curved link (a chord of at least 2R·sin(φ/2) with R > 10^-12 and φ ≥ 10^-9). L = 1 for a body of ties only (v ≡ 0).
- **So for an SA-built body:**
  - an overflow needs |t_i| > 2^952;
  - a rounding needs |t_i| < 2^-1022·L.
- Neither arises at product-scale coordinates. If one did, SA would publish K5-C2's refusal instead of a mechanism, which is conservative.
- **None of the regression corpora can see the change.** W4 runs in no T9 or gate part 1 run (no curved bend, no user element), and SA and PP sources are untouched. So the addendum re-runs targeted tests only (ROOT's direction). T9, gate part 1, the 39-manifest suites and the 152-run product table stand as recorded at B.

### 16.5 The NOTEs

- **N1.**
  - §6's NI line numbers are renumbered to `b379e5b27` (+32 above line 703: K5-M3 `:940`, K5-M4 and M12 `:1049`, K5-M8a `:867`, K5-M8b and KD5-SELECT `:885`, K5-M10 `:1232` and `:1062`, K5-M13 `:1097`), and the table says which tree it uses.
  - §2.5 now cites `SA:1900` on the base and `SA:2142` at the head.
  - §9.1's FK anchors are updated to the addendum's tree.
- **N2.** `k5_positive_springs_ground_w4_bodies` is RV14's P2: K5's curved line held about a–d by an RX spring of 1e6 at a.
  - It asserts W4's `Restrained` and each selected entry equal to its unselected sibling.
  - RV14-M3 is now killed at that assertion (`k5_tests.rs:1328`) as well as by the K1 and K-D5 helpers' `unwrap` panics.
- **N3** (M0438's completeness gap) and **N4** (the host name at RETURN.md:12) need no action.
- **N5.** ROOT merges main (`df6d59e3c`, app-v4 only) after committing the addendum. I14 made no Git write.

### 16.6 Evidence

**Tests** (targeted, `<wt>/k5`'s working tree, target `<wt>/k5-target`):

| Suite | Result |
|---|---|
| FK `--lib` (`rigid_body::`, `k5_`) | 8 |
| FK `k5_constrained_bodies` | 15 (+1: `k5_b5_parameters_are_exact_or_refused`), with the full subnormal set |
| FK `k5_scale` | 1 |
| FK `s11_site_table` | 3 |
| NI `--lib` | 129 (+2: `k5_curved_sources_agree_at_a_curved_only_node`, `k5_positive_springs_ground_w4_bodies`) |
| PP `k5_curved_mechanism_runtime` | 4 (+1: `k5_curved_mechanism_without_a_representable_witness_is_unresolved`) |

- NI's non-test build has no warnings.
- `gen_k5_vectors.py --check` is OK, and `b1_sample.txt` and `b1_summary.txt` are unchanged. The reduction was factored into `reduction()` with no output change.
- `rigid_body.rs:1-248` still hashes `d9598efa…`.

**Mutations** (`_run_records/rv14/mutations/`).
- Each ran in a clean `git archive b379e5b27` copy, plus the addendum's nine files (`overlay_d2.txt`), with its own target deleted afterwards. The NONE control ran first: FK 8 + 15 + 1, NI 129, PP 4.
- RV14's patches were applied from its committed `rv14_mutate.py` (sha256 `9ef6da73…`, copied as `rv14_mutate.py.txt`); K5-M20 from `mutate_k5.py`. At most two ran at once, at `-j 4`.

| Mutant | Result (assertion site) |
|---|---|
| RV14-M1 (exact tie loop removed) | killed: `k5_b4_cycles` (`k5_constrained_bodies.rs:256`) |
| RV14-M2 (curved-to-curved agreement dropped) | killed: `k5_curved_sources_agree_at_a_curved_only_node` (`k5_tests.rs:1308`) |
| RV14-M4 (K5-C2 mapped to `Ok`) | killed: `k5_curved_mechanism_without_a_representable_witness_is_unresolved` (`k5_curved_mechanism_runtime.rs:262`); NI 129 still pass |
| RV14-M3 (spring grounds dropped) | killed: `k5_positive_springs_ground_w4_bodies` (`k5_tests.rs:1328`), and the `unwrap` panics RV14 recorded (`k1_tests.rs:349`, `:1364`, `kd5_tests.rs:188`) |
| K5-M20 (own; RV14-4's check removed) | killed: `k5_b5_parameters_are_exact_or_refused` (`k5_constrained_bodies.rs:1081`) |

**Host.** The Mac was shared with I13's F1b build, at load 6 to 10. The memory guard was unchanged: 2 lines, `79e2ce8e…`.

**The addendum's files (sha256):**

| File | Lines | Diff against `b379e5b27` | sha256 |
|---|---|---|---|
| `FK/src/rigid_body.rs` | 1,407 | +30 −9 | `3e5b9be1…` |
| `FK/tests/k5_constrained_bodies.rs` | 1,109 | +112 −4 | `f56d77d0…` |
| `FK/tests/k5_constrained/gen_k5_vectors.py` | 910 | +157 −5 | `2e4a850c…` |
| `FK/tests/k5_constrained/cases.txt` | 29 | +2 | `75657e93…` |
| `FK/tests/k5_constrained/SHA256SUMS` | 6 | +4 −2 | `4b00b516…` |
| `FK/tests/k5_constrained/subnormal.txt` (new) | 301 | +301 | `7e41bf9c…` |
| `FK/tests/k5_constrained/subnormal_summary.txt` (new) | 15 | +15 | `63ad0f61…` |
| `NI/src/structural_adapter/k5_tests.rs` | 1,333 | +75 (appended) | `bacf221b…` |
| `PP/tests/k5_curved_mechanism_runtime.rs` | 267 | +43 −4 | `6450e9db…` |

No SA, PP, NI `lib.rs`, CB or other source changes. No dependency or lockfile change.

**Disclosures.**
- `_run_records/rv14/tests/fk.txt`, `fk_site.txt`, `ni.txt` and `pp.txt` are cargo's raw output, with machine paths replaced by placeholders. They keep cargo's blank line at the end of the file, verbatim. They are the only whitespace items the addendum adds; §14's four remain.
- RV14's committed scripts are reused unchanged: `rv14_mutate.py` (`9ef6da73…`), `rv14_fk_probe.rs` (`9a2c7878…`) and `rv14_oracle.py` (`03586d4b…`). They were copied from `REVIEW/_run_records/k5_review/`, and the first two are kept here as `.txt`. RV14's `tiny_case`, `partition` and `rand_dir` are ported into `gen_k5_vectors.py` with attribution, and their output is byte-identical to RV14's corpus.
- The candidate's probe results (1.7 MB) are not committed. Their sha256 is in `probe/comparison.txt`.

### 16.7 RV14's delta check at `28517eaaa` (PASS; 1 SHOULD-FIX, 1 NOTE)

**Basis.**
- RV14's delta check is in `REVIEW/K5_REVIEW.md`, section "Delta check at 28517eaaa", committed at `f3e50948b` (numerics branch), with its records in `REVIEW/_run_records/k5_review/delta/`.
- ROOT's rulings are in `ROOT_RULINGS_V1.md`, "K5: rulings on RV14's delta check at 28517eaaa" (`9b13481fa`): fix RV14-D1 before merge; accept RV14-D2 as a conservative limitation.
- The head is `28517eaaa`: the addendum `95c7501a7` on `b379e5b27`, plus the merge of main `65e2d6c2a`.

**RV14-D1 (SHOULD-FIX): the exactness half of RV14-4's check was untested.**
- **The gap.** RV14's mutant RV14-M5 keeps only the finiteness test, `(0..3).any(|i| !t[i].is_finite())`, and survived FK's K5 tests. Every existing refusal (the minimal tiny-L case and the subnormal corpus) was an overflow, never a rounding.
- **The fix.** `k5_b5_parameters_are_exact_or_refused` gains RV14's construction as a `P` record, built in the test from the bits: `rv14_d1_huge_underflow_L2e1023_r2e-60`.
  - Nodes (0,0,0) and (2^1023,0,0), one sub-body.
  - Grounds d2–d5: u_z and every θ at node 0, so θ = 0.
  - A directional translation row n = (2^-60, −1, 0) at node 0, which forces t_y = 2^-60·t_x.
- **Why it must be refused.**
  - The only null motion is the translation along (1, 2^-60, 0) (nullity 1). Its canonical witness moves both nodes by exactly (1, 2^-60, 0).
  - L = 2^1023. So t_x/L = 2^-1023 is an exact subnormal, but t_y/L = 2^-1083 underflows to 0, and fl(0·L) = 0 ≠ 2^-60.
  - `check()`'s `P` arm asserts `NumericallyUnresolved` with `CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE`. The test also asserts L = 2^1023.
- **The control** (RV14's `D_huge_subnormal_exact_L2e1023_r2e-40`): the same body with n = (2^-40, −1, 0).
  - t_y/L = 2^-1063 is an exact subnormal, so the witness must publish (`W`), with canonical motion (1, 2^-40, 0, 0, 0, 0) at both nodes.
  - `verify_witness` checks the parameters exactly, and the test asserts `rigid_parameters[1]` = 2^-1063 bit for bit.
- **The edit.**
  - `FK/tests/k5_constrained_bodies.rs`: 51 lines inserted after line 1087, inside the test. Every earlier line number, including K5-M20's kill site `:1081`, is unchanged.
  - No source, vector file or generator changes.
- **The mutant.** RV14-M5, RV14's text: `<scratch>/mut/RV14-M5.patch` (sha256 `4ebe0aec…`, kept as `_run_records/rv14/delta/RV14-M5.patch.txt`), applied through `mutate_k5.py`.
- **Stage 1** wrote the test with nothing built or run, while F1b's gate part 2 needed a quiet host. **Stage 2** ran after ROOT released the host (`_run_records/rv14/delta/`):
  - **FK's K5 tests on the working tree:** 8 + 15 + 1 pass, the full subnormal set included (`fk_worktree.txt`). The new `P` record and its `W` control pass.
  - **The NONE control,** from a clean `git archive 28517eaaa` plus the test file (`overlay_d3.txt`): FK 8 + 15 + 1 pass.
  - **RV14-M5,** from its own clean archive: **killed** at `check()`'s `P` assertion (`k5_constrained_bodies.rs:269`) on `rv14_d1_huge_underflow_L2e1023_r2e-60`, with `(MechanismWitnessed, None)` against `(NumericallyUnresolved, Some("constrained-body witness parameters not representable"))`.
  - **Host.** At `-j 8`, `RUST_TEST_THREADS=4`, one cargo job at a time, at load about 5. Each copy and target was deleted afterwards. The memory guard was running and unchanged (2 lines, `79e2ce8e…`).
  - **Disclosure.** `fk_worktree.txt` is cargo's raw output, with machine paths replaced by placeholders. It keeps cargo's blank line at the end of the file, verbatim, as `rv14/tests/*.txt` do.

**RV14-D2 (NOTE): accepted as a known, conservative limitation. The search is not changed.**
- **The behaviour.** `publish`'s refusal returns from `assess_constrained_bodies` at once (`return Ok(context.publish(result, found))`), so no later candidate is tried.
- **RV14's construction** `E_tiny_two_modes`: nodes (0,0,0) and (2^-1070,0,0), grounds d1, d2, d4, d5, d7, d8, exact nullity 2 (free t_x and free θ_x).
  - It ends `NumericallyUnresolved` with the parameters reason.
  - Yet the verified rotation witness θ = e_x, t = 0 has exact parameters [0, 0, 0, 1, 0, 0]. `E_tiny_rotation_only`, which also grounds t_x, publishes it.
- **ROOT's ruling (`9b13481fa`).**
  - The result is a refusal, never a wrong value. It is reachable only through the FK API: from SA-built evidence L ≥ 2^-72 (§16.4).
  - Moving on to the next candidate would change FK's result classes and the probe hash, which would reopen the oracle and probe checks for no correctness gain.
  - So it is recorded here and in §9.1 ("the first verified witness decides"), and it joins the T3-close list (§10) as a candidate refinement.

**Files changed after `28517eaaa`** (uncommitted; ROOT commits):

| File | Change | sha256 |
|---|---|---|
| `FK/tests/k5_constrained_bodies.rs` | +51 (1,160 lines) | `ca7ffd1e…` |
| `IMPLEMENTATION/K5/RETURN.md` | §9.1, §10, §16.7, header | see SHA256SUMS |
| `IMPLEMENTATION/K5/CHANGE_RECORD.md` | the delta section | see SHA256SUMS |
| `IMPLEMENTATION/K5/_run_records/rv14/delta/` (new) | stage 2's runs: the worktree FK log, the runner, `overlay_d3.txt`, `mutate_k5.py.txt`, `parse_results.py.txt`, `RV14-M5.patch.txt`, `MUTANTS_DELTA.txt`, the D3-NONE and RV14-M5 logs, `batch_delta.log`, `memguard.txt` | see SHA256SUMS |
| `IMPLEMENTATION/K5/SHA256SUMS` | regenerated | — |
