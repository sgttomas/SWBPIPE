# F2 response-spectrum analysis: governing design

**Status.** This is DEL-04-10's governing design for F2, response-spectrum seismic analysis with uniform support motion. Scope entered by SCA-014 (owner, 2026-10-28); Agent 0 placed it in DEL-04-10 on 2026-10-31. This revision fixes the method and its boundaries so that the references and the screening class can be derived first. It answers the first independent review (gpt-6.1-sol, at `5b602fd33`). The product-side layout (§7) is settled when the build slice starts, after DEL-04-09's dense path, views and sparse path. Nothing is built. This design makes no acceptance, issuance or lifecycle statement (F-PIP-3). Standard claim fence applies (claims taxonomy per DEC-081).

**Labels.** **[decided]**: settled here, with the reason. **[open]**: listed in §9, with its owner.

**Notation.** DOFs are free (f) or rigidly restrained (c); a denotes all DOFs. R_k is the rigid unit translation along global k over **all** DOFs (1 on each node's k translation, 0 on rotations). M and K are the full assembled mass and stiffness of the dynamic model (DEL-04-09's mass model; the dynamic stiffness, snubbers included, §5). Springs to ground act on displacement relative to the ground. φ_i is a free-DOF mode with eigenvalue ω_i² and generalized mass m_i = φ_iᵀM_ffφ_i; φ̂_i is φ_i extended by zeros on the restrained DOFs.

**Constraint convention.** Restraints are applied by a constraint basis E with full DOFs u = E·w (partitioned elimination of axis restraints, and the static path's nullspace basis T for skewed restraints, DEL-04-09 Design §1.3). Wherever this design writes free-row selection (·)_f, M_ff, K_ff or zero padding φ̂, read the congruence forms K_r = EᵀKE, M_r = EᵀME, p_r = EᵀM R_k and φ̂ = E·φ; the reactions of rigid restraints are recovered from the full residual K u + M ü_t − f over all DOFs, with the static path's attribution to its restraints. The forces of compliant supports (ground springs, finite-stiffness snubbers, and skewed springs as k·d dᵀ/(dᵀd)) are inside K, so they vanish from that residual; each is recovered separately as r_s = −K_s·(u − u_base), with u_base = 0 for the relative displacements of uniform support motion (review round 3: an axial member with a ground spring at its free end loses the spring's reaction in the residual). For axis restraints only, E selects the free DOFs and the two notations coincide.

---

## 1. Basis

- **[decided] The modal basis is DEL-04-09's.** F2 consumes a published modal result: f_i, ω_i, φ_i in its normalization case, m_i, the participation factors Γ_ik, cluster ids and the result's standing. It reads DEL-04-09's mass model for the inertia terms below. It forms no mass and solves no eigenproblem of its own (AX-002).
- **[decided] The modal request for a spectrum solve** includes dynamic-only restraints (§5) and, if the model has nonlinear supports, names the solved case at which DEL-04-09 linearizes them. Both are DEL-04-09 request inputs; F2 adds no support state of its own.
- **[decided] Routes.** PF and EX both, as far as the modal basis admits them.

## 2. Uniform support motion

- **[decided] Equations.** With every rigid support moving with the ground acceleration ẍ_g along k, and x the displacement relative to the ground (x_c = 0), the free rows are
  M_ff ẍ_f + K_ff x_f = −p_k ẍ_g,  p_k = (M R_k)_f = M_ff R_k,f + M_fc R_k,c.
  K's rigid-body property (K R_k = 0 for the structure; ground springs act on relative motion) removes the stiffness term of the ground motion. The coupling term M_fc R_k,c is nonzero for the consistent mass at every element adjacent to a support, so the excitation is **not** M_ff R_k,f alone. (Review finding 1: one axial element of mass m and stiffness k, fixed at one end, gives u/A = m/(2k) with p_k, against m/(3k) with M_ff R_f alone.)
- **[decided] Participation is DEL-04-09's, on p_k.** Γ_ik = φ_iᵀp_k / m_i and m_eff,ik = (φ_iᵀp_k)²/m_i. DEL-04-09 publishes them on this excitation vector (its §1.5, as reconciled with this design), so F2 uses the published Γ unchanged.
- **[decided] Support reactions** are recovered from the restrained rows of the full equation:
  r_c = K_cf x_f + M_cf ẍ_f + (M R_k)_c ẍ_g,
  so the mass attached at restrained DOFs, and the consistent-mass coupling, load the supports directly.
- **[decided] Element end forces and moments** are recovered with the element's own inertia, f_e = K_e x_e + M_e ẍ_t,e (ẍ_t the total acceleration), so that the end forces at a node balance the nodal inertia and the support reactions equal the sum of the element end forces at the restrained nodes. With the lumped bodies of arcs and components, their 6×6 rigid-body masses enter the same way.

## 3. Modal responses

For direction k, spectrum S_k (pseudo-acceleration, §4.1) and scale s_k:
- **Spectral pseudo-acceleration of mode i:** A_ik = s_k·S_k(f_i), interpolated by the spectrum's stated rule (§4.1).
- **Peak modal coordinate:** q_ik = Γ_ik·A_ik/ω_i², and the modal displacement u_ik = q_ik·φ_i. Γ_ik·φ_i, and so u_ik, does not depend on the scaling of φ_i, in either of DEL-04-09's normalization cases; in case (b), q_ik is a rotation and u_ik is still in metres and radians.
- **[decided] Damping and the recovery approximation.** Damping is classical modal damping, represented only through the spectra at their damping ratio ζ; the model has no damping matrix. Each mode's response quantities are recovered at its **displacement extremum** (q̇_i = 0), the pseudo-static modal recovery of piping spectrum practice. The velocity-proportional parts of element forces and reactions (from the damped modal equation q̈ + 2ζωq̇ + ω²q = −Γẍ_g) are neglected. This is an adopted, disclosed approximation. Under a harmonic modal comparison (q = sin ωt), a mode's periodic reaction is [(K_cf − ω²M_cf)φ]q − 2ζω(M_cfφ)q̇, so for a reaction dominated by its inertia term the true amplitude exceeds the displacement-extremum value by the factor √(1 + 4ζ²), about 2ζ²: the recovery understates it by about 0.5 % at ζ = 0.05 and 7.2 % at ζ = 0.2. The one-element axial witness gives √(2.25 + ζ²) against 1.5, an understatement of 5.1 % at ζ = 0.5. Where the stiffness and inertia terms nearly cancel, the relative effect can be larger, and no general bound on force or reaction error is claimed; the damping cap limits the effect, it does not bound it. The admitted damping range is 0 ≤ ζ ≤ ζ_max = 0.2, an algorithm parameter stated in the result; a spectrum with a larger damping ratio is refused (`SPECTRUM_DAMPING_OUT_OF_RANGE`). A damped force and reaction reference is frozen (§6).
- **Modal recovery.** In mode i, at its displacement extremum, the relative acceleration is −ω_i² u_ik plus a part in phase with the ground acceleration (q̈_i = −ω_i²q_i − Γ_ik ẍ_g in the undamped form). The first part is the periodic modal response:
  - displacements and rotations: u_ik;
  - element end forces: (K_e − ω_i² M_e)·u_ik,e;
  - support reactions: (K_cf − ω_i² M_cf)·u_ik at rigid restraints, and −K_s·u_ik at each compliant support s.
  The ground-phase parts belong to the rigid response (§4.3).
- Every modal quantity is kept **signed** per mode and per direction until combination. Only combination makes a magnitude.

## 4. Spectra, combinations and the rigid response

### 4.1 Spectra and their interpolation

- **[decided] Where spectra live.** Spectrum tables are top-level `spectra[]` records of model document 0.9.0, as agreed with the document lane and recorded in its design (DEL-02-01 Work `model-document-0.9.0.md` §2.4; selected basis: branch `claude/rt-load-cases` at `c42a72491`, the member list frozen at `31bde47a2` with its later validation and unit clarifications, pending merge to main): abscissa (frequency or period), ordinate (acceleration, or a g multiple with a required positive g value), damping ratio, interpolation and points. A spectrum is entered once and referenced only by the spectrum request. Directions, scale factors and method choices stay on the request.
- **[decided] Ordinate convention: pseudo-acceleration.** Every ordinate is the **pseudo-spectral acceleration** ω²·S_d(ω, ζ), with S_d the peak relative displacement of a linear oscillator at the spectrum's damping ζ. It is not the peak absolute acceleration of a damped oscillator, which differs for ζ > 0. The request, the result and every surface state this convention. A spectrum of another kind must be converted by the user before entry; F2 converts none. (Review finding 2.)
- **[decided] Domains** (beyond the document's own rules): abscissae positive, finite and strictly increasing; ordinates positive and finite; g value positive and finite; damping ratio in [0, 1) by the document, and in [0, ζ_max = 0.2] for a spectrum solve (§3). There is no zero-period point: the rigid response is defined by the rigid frequency (§4.3), not by a T = 0 ordinate.
- **[decided] Interpolation.** The user chooses linear or logarithmic on each axis; there is no default. A spectrum is interpolated on the abscissa it is entered in, at f_i or at T_i = 1/f_i. A **logarithmic abscissa** gives the same value whichever abscissa is entered, since log T = −log f; a linear abscissa does not, so the entered abscissa governs.
- **[decided] No extrapolation.** A periodic mode (§4.3) or a rigid frequency outside a spectrum's tabulated range is refused by name (`SPECTRUM_FREQUENCY_OUT_OF_RANGE`, naming the mode or the rigid frequency, and the spectrum).
- **[decided] Scale factors** are positive and finite, one per direction, and scale the whole response of that direction, rigid part included (§4.3).

### 4.2 Modal combination

Applied to each response quantity separately, with R_i the signed periodic value of mode i in one direction:
- **SRSS:** R = √(Σ R_i²).
- **CQC:** R = √(Σ_i Σ_j ρ_ij R_i R_j), with the Der Kiureghian (1981) correlation for equal modal damping ζ, r = ω_j/ω_i: ρ_ij = 8ζ²(1 + r)r^{3/2} / ((1 − r²)² + 4ζ²r(1 + r)²).
  - **[decided] ζ is the spectra's damping.** All spectra in one request must have the same damping ratio, bit for bit after normalization, or the request is refused (`SPECTRUM_DAMPING_MISMATCH`, naming the spectra and values). CQC uses that ratio; there is no separate CQC damping input, so the correlation cannot disagree with the spectra. (Review finding 3.)
  - **Domain.** CQC requires 0 < ζ < 1; a request for CQC on spectra with ζ = 0 is refused (`SPECTRUM_CQC_DAMPING_UNSUPPORTED`). SRSS and grouping admit ζ = 0.
- **Grouping of closely spaced modes:** the Grouping method of US NRC Regulatory Guide 1.92, Rev. 1, as NUREG/CR-6645 §2.1.2 describes it: modes in ascending order form groups in which each frequency is within 10 % of the group's lowest, and R = √(Σ R_i² + 2Σ_{groups} Σ_{i<j in the group} |R_i R_j|). The rule and its 10 % are stated in the result. (It is distinct from the pairwise Ten Percent Method, NUREG/CR-6645 §2.1.3, which is not offered.)
- **[decided] Why these three.** They are the established rules for piping spectrum analysis, and the vendor's public manual lists mode combination among its options (SCA-014's citations). Others are later options, [open] O-1.

### 4.3 The rigid response and the missing-mass correction

- **[decided] The rigid frequency.** When the correction is applied, the user enters a rigid frequency f_r (required, no default). Modes with f_i < f_r are **periodic**; modes with f_i ≥ f_r, and every mode not extracted, are **rigid**. The rigid acceleration is a_r,k = s_k·S_k(f_r), read from the same spectrum, so it scales with s_k like the modal part (review finding 8).
- **[decided] The extraction premise.** The extracted set must reach the rigid frequency: its highest completed cluster must lie at or above f_r, or the set must be the whole finite spectrum. Otherwise the request is refused (`SPECTRUM_EXTRACTION_BELOW_RIGID_FREQUENCY`, naming f_r and the highest extracted frequency), so no amplified mode below f_r is ever replaced by the rigid acceleration. (Review finding 4.)
- **[decided] The rigid response** of direction k is the static response to the inertia of everything the periodic modes leave out, at a_r,k, over all DOFs:
  F_k = a_r,k·(M R_k − Σ_{i periodic} Γ_ik·M φ̂_i).
  F_k is the d'Alembert inertia load −M ẍ_t of the rigid motion, applied as a load (with ẍ_g = −a_r,k; the sign is lost in combination but fixed within the rigid response). Its free rows are solved on the dynamic stiffness, u_r,k = K_ff⁻¹ F_k,f. Displacements are u_r,k. Element end forces are K_e u_r,k,e − F_e, with F_e = a_r,k·M_e(R_k − Σ_periodic Γ_ik φ̂_i)_e the element's own inertia; this matches §3's (K_e − ω_i² M_e)u_ik, whose inertia load is ω_i² M_e u_ik. Support reactions are K_cf u_r,k − F_k,c at rigid restraints and −K_s·u_r,k at each compliant support. Equilibrium holds: at each free node the end forces balance, and the reactions balance the total of F_k.
  - Extracted rigid modes enter this static solve exactly, with their signs, and are never combined as periodic responses (review finding 4: two same-sign rigid contributions of 1 give 2 here, not √2).
  - With v_k = R_k,f − Σ_periodic Γ_ik φ_i, the identity φ_jᵀ(p_k − Σ_periodic Γ_ik M_ff φ_i) = 0 holds for every periodic j: the free-row residual **load** is orthogonal to the periodic modes. It holds in exact arithmetic for exact modes, and only approximately for computed or Sensitive ones. (Review finding 7.)
- **[decided] Combination with the periodic part,** per direction and per quantity, by the user's rule: SRSS or absolute sum, before directional combination.
- **[decided] Without the correction** (the user's choice), all extracted modes are periodic, no rigid frequency is used, and the rigid response, including the ground-phase inertia of restrained-DOF mass, is omitted and disclosed.
- **[decided] Disclosures.** Per direction: the cumulative effective-mass fraction of the periodic modes, Σ_periodic m_eff,ik / T_k, and the mass not represented by them, as the fraction 1 − Σ_periodic m_eff,ik / T_k, with T_k = R_kᵀM R_k the total model mass in direction k. This includes the mass that moves with the supports, stated as such.
- **[decided] Stated limitation.** The rigid response assumes that modes at or above f_r respond in phase with the ground at a_r,k. That is the premise of the rigid frequency the user enters, and the result says so.

### 4.4 Directional combination

- **SRSS:** R = √(R_X² + R_Y² + R_Z²) over the directions requested.
- **100-40-40:** R = max over the permutations of (R_a + 0.4 R_b + 0.4 R_c), with R_a, R_b, R_c the directional magnitudes; with two directions, max(R_a + 0.4 R_b, R_b + 0.4 R_a).

### 4.5 Combination with static cases

- **[decided]** F2 does not combine with static cases. A spectrum result is offered to DEL-05-02's load-case algebra as an unsigned operand, so that, for example, sustained plus seismic by absolute sum or SRSS is a load-case combination with DEL-05-02's rules. The interface (an unsigned-operand marker on the result) is agreed with the load-case lane when F2's results land (O-2).

## 5. Snubbers

- **[decided] Document shape** (agreed with the load-case manager, who owns the 0.9.0 member list, frozen 2026-10-31 at `31bde47a2`; selected basis `c42a72491`, as §4.1; DEL-02-01 Work `model-document-0.9.0.md` §2.4, pending its merge): `SupportFamily` gains `snubber`, reusing `Support`'s members:
  - restraints are translational DOFs only (`SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED`);
  - an optional `stiffness` gives one finite dynamic stiffness on exactly its one restrained DOF (`SNUBBER_STIFFNESS_INVALID`);
  - `hanger` and a non-null `nonlinear` are refused on a snubber (`SNUBBER_MEMBER_CONFLICT`);
  - `supports[].direction` is admitted on a snubber with exactly one translational restraint.
- **Interim** (0.9.0's first slice): every static load-case solve on both routes refuses a model containing a snubber, by name (`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`). F2's slice replaces that refusal with the meaning below.
- **[decided] Dynamic meaning.** In the dynamic stiffness (a modal request that includes dynamic-only restraints, and so every spectrum solve, including its auxiliary static solve of §4.3), a snubber is:
  - **rigid:** a restraint of its global DOFs, or, with a skew direction d, the exact constraint dᵀu = 0 by the static path's division-free nullspace basis T (DEL-04-09 Design §1.3; DEL-05-02 Work `operating-case.md` §2.2), leaving perpendicular motion free;
  - **with stiffness k:** a spring k on its global DOF, or, with a skew direction, the rank-1 term k·d dᵀ/(dᵀd) with the authored d on the node's translations, never a diagonal term in global axes (review finding 9).
  - A non-axis direction is admitted exactly when the static path admits it; until the skew kernel lands it is refused with the static code (`SUPPORT_DIRECTION_SKEW_UNSUPPORTED`).
- **[decided] Static load-case solves** on both routes ignore snubbers, with the disclosure "dynamic-only restraint, not active in static cases". "Static load-case solves" means the analyses of load cases. It does not include F2's auxiliary static solve, which uses the dynamic stiffness.

## 6. References, verification tolerances and the screening class

- **[decided]** `validation/references/f2_spectrum/`: a standard-library Python generator, exact rationals of the binary64 inputs and `decimal` for roots, invented data only, frozen JSON and `SHA256SUMS`, written by a TASK that reads no product code and refuted by a second TASK before any comparison. Cases:
  - a single degree of freedom under a stated spectrum, each interpolation rule, and a damped case showing the pseudo-acceleration convention (S_d = A/ω², not the absolute-acceleration peak);
  - a damped force and reaction case showing the displacement-extremum recovery and the size of the neglected velocity term;
  - a one-element consistent-mass axial witness (u/A = m/(2k)) and a restrained-mass witness (support reaction including the attached mass's inertia);
  - a two- or three-degree-of-freedom chain with closed-form modes, under SRSS, CQC (ζ series and a closely spaced pair with opposite-sign contributions) and grouping;
  - a uniform cantilever with distributed mass under a flat spectrum, with the rigid response for 1, 2 and 3 periodic modes converging to the all-mode value, and a case with an extracted rigid mode entering the static solve;
  - a stiff model whose response equals its static rigid-acceleration response;
  - the extraction-premise refusal: a mode below f_r not extracted;
  - directional combinations, SRSS and 100-40-40, with two and three directions;
  - a ground spring at a free node together with a rigid end, checking both reactions and their equilibrium in a modal and the rigid response;
  - a skew snubber, rigid and with stiffness, leaving perpendicular motion free, and a rigid response whose stiffness depends on the snubber.
- **[decided] Verification tolerance (product against references).** It is analytic, under DEC-026: a relative tolerance seeded at 1e-9 for quantities formed by the same discrete operations as the references, with stated absolute floors for quantities near zero, measured and only tightened. It is separate from, and much tighter than, the screening class. (Review finding 6.)
- **[decided] The spectrum screening class** for `docs/RELEASE_TARGET_v1.0.md` §3 (cross-engine comparison, never product verification) is derived from the frozen references before any comparison run, never from comparison results. The method: the sensitivity of each combined response to a relative frequency perturbation at the modal class's 5e-3, propagated through interpolation, the 1/ω² dependence and combination, rounded up to a stated value. Its record names the reference package revision, the derivation, the date it was reported to Agent 0 for the owner, and the first comparison run it precedes, so the order can be audited. It is only tightened.

## 7. Product layout (settled at build start)

Planned, as in `deliverable.yaml`: `core/product_physics/src/spectrum.rs` with an entry beside DEL-04-09's modal entry, the `spectrum-1` table and profile, the Rust and Python readers, a headless command, and a Tauri command, editor, results panel and report section. Signed per-mode contribution rows and unsigned combined rows are distinct row kinds in the table. Priced-source and reviewed-input changes re-register D3 in their change.

## 8. Independent support motion (REQ-011, Should)

Defined now so the commitment is testable; built after the uniform-motion core, refused by name until then (`SPECTRUM_INDEPENDENT_SUPPORT_MOTION_UNSUPPORTED`).
- **Groups.** Each support group g has its own spectrum per direction.
- **Skewed restraints in a moving group.** A moving skew restraint needs an inhomogeneous constraint (dᵀψ = 1 along its direction), which the homogeneous basis u = E·w does not express. Until an affine lifting ψ_g = b_g + E·w_g is adopted, a request whose support groups move a skewed restraint is refused (`SPECTRUM_INDEPENDENT_SUPPORT_MOTION_SKEW_UNSUPPORTED`).
- **Groups of compliant supports.** Every ground spring and finite-stiffness snubber is assigned to a support group by the user (a required input for independent support motion, with no default), because its base moves with that group.
- **Influence vectors.** For group g and direction k, ψ_gk is the static displacement of the free DOFs under unit motion along k of the group's restrained DOFs and of the bases of its compliant supports, with the other groups fixed: K_ff ψ_gk,f = −K_fc R_gk,c + Σ_{s in g} K_s R_k, with K_s the ground stiffness of compliant support s (so that the sum over all groups gives ψ = R_k, the uniform case). A unit move of a rigid support joined by k to a free node that also has a ground spring k_s of the same group gives ψ = 1, not k/(k + k_s). The excitation is p_gk = (M ψ̂_gk)_f, with ψ̂_gk carrying R_gk,c on the restrained DOFs, and Γ_igk = φ_iᵀp_gk / m_i.
- **Order.** Per mode and direction, group responses are combined first by the user's group rule (SRSS or absolute sum), then modes by the modal rule, then directions. This follows the order of the published independent-support-motion benchmarks (NUREG/CR-1677 Vol. 2, cited for method only). For two modes with group contributions (1, 0) and (0, 1), absolute sum then SRSS gives √2; the other order would give 2, and is not used.
- **No CQC with independent support motion.** Group-first combination yields nonnegative per-mode magnitudes, which carry neither the signs nor the cross-group correlation CQC's cross terms need (two modes with group contributions (+1, 0) and (−1, 0) at ζ = 0.05 and frequency ratio 1.01 give 0.140 by signed CQC but 1.995 from magnitudes). CQC is therefore refused for independent support motion (`SPECTRUM_INDEPENDENT_SUPPORT_MOTION_CQC_UNSUPPORTED`) until a method with signed group contributions and stated cross-group correlation is adopted. SRSS and grouping, which use magnitudes, are offered.
- **Rigid response** per group, by §4.3 with p_gk and the group's rigid acceleration.
- **Not included:** the pseudo-static response to differential support displacement, which is seismic anchor movement (RT-A10, a static load case). Its combination with the spectrum result is DEL-05-02's.

## 9. Risks

- **R1, unsigned results misused.** Combined magnitudes look like static results. Every combined row and surface labels them unsigned, and no signed recovery, deformed shape or stress is made from them (AX-003). Per-mode contribution rows are signed and labelled as such.
- **R2, truncation.** The extraction premise and the rigid response address it when the correction is applied; without it, the omission is disclosed with the represented mass fraction.
- **R3, closely spaced modes.** SRSS underestimates correlated close modes. CQC and grouping are offered; the choice is the user's.
- **R4, nonlinear supports.** The spectrum result holds only at the linearized state the modal basis names, which the result states.
- **R5, stresses.** Users will want seismic occasional stresses. They need per-mode stress recovery combined by the same rules, not recovery from combined forces; outside these commitments (O-3).
- **R6, rigid frequency.** The rigid response is only as good as the user's f_r. The extraction premise prevents the worst error; the result states f_r and the spectrum's ordinate there.

## 10. Open items

| # | Item | Owner | When |
|---|---|---|---|
| O-1 | Further modal-combination rules (RG 1.92 Rev. 3 rigid-response methods, Ten Percent Method) | Later slice | After F2's core |
| O-2 | The unsigned-operand interface with DEL-05-02's load-case algebra | Dynamics and load-case lanes | When F2's results land |
| O-3 | Stresses from spectrum responses, by per-mode recovery and combination | HELP_HUMAN, for scope | After F2's core |
| O-4 | Independent support motion (§8, REQ-011, Should): build | F2 | After the uniform-motion core |
| O-5 | The spectrum screening class: derived from the references and reported | F2 | Before any comparison run |
| O-6 | DEL-04-09's participation on p_k (§2): reconciled in DEL-04-09's commitments in the same change as this design | Dynamics lane | This change |
