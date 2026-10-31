# F1 modal analysis: governing design

**Status.** This is DEL-04-09's governing design for F1, modal analysis (natural frequencies and mode shapes). Scope entered by SCA-013 (`execution/_ScopeChange/SCA-013_2026-10-27_MODAL_ANALYSIS/DECISION.md`). Nothing is built. The decisions below are Agent 0's (HELP_HUMAN) of 2026-10-27, made under the owner's delegation in `loop/LOOP_INIT.md` (#1662), amended by Agent 0's decisions of 2026-10-31 on the dynamics lane's plan for v1.0 (`docs/RELEASE_TARGET_v1.0.md` RT-A7, exit criterion 1): the exact route on its own mass basis (D-r), nonlinear supports linearized at a named solved case (D-s), snubbers in the modal basis when requested (D-t), and the published-fixture review record (D-u); and participation on the uniform-support-motion excitation (D-v), from the DEL-04-10 review. This design makes no acceptance, issuance or lifecycle statement (F-PIP-3). Standard claim fence applies (claims taxonomy per DEC-081).

**Basis.** It was researched against main `0280fd4486`, together with the PF Timoshenko design `execution/PKG-05/DEL-05-03/Work/pf-timoshenko.md` (#1663, in review; not on main at that commit) and the feature survey `execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md` (#1657, open). Paths are relative to `projects/chirality-piping/` unless they start with `/`. "PP `lib.rs`" is `core/product_physics/src/lib.rs`.

**Labels.**
- **[code]**: a fact about existing code, with file:line.
- **[design]**: a fact about an existing design or record.
- **[decided]**: settled by this design or by Agent 0's decisions, with the reason.
- **[open]**: listed in §9, with its owner.

---

## 0. Scope

- **[design]** FR-024 (dynamic analysis modules) was explicitly deferred by DEC-078 (D-12, owner 2026-07-15). DEC-078 required that "any future dynamics scope re-enters only through a new governed scope change with its own human decision packet" (`execution/_Decomposition/SOFTWARE_DECOMP.md`, DEC-078).
- **[decided]** SCA-013 re-enters it for **modal analysis only**: PRD FR-SOL-016 and §12.9, scope item SOW-081, and this deliverable. Agent 0 decided it under the owner's delegation of 2026-10-27; the reason is recorded in the SCA-013 decision.
- **Out of scope:** response spectrum (F2), which SCA-014 re-entered and DEL-04-10 owns on this deliverable's modal basis (`execution/PKG-04/DEL-04-10/Work/response-spectrum.md`); time history (F3), harmonic analysis and every other dynamic module, which stay deferred under DEC-078.

---

## 1. What F1 solves

### 1.1 The problem

**[decided]** F1 solves the real symmetric-definite generalized eigenproblem

  K_ff φ = ω² M_ff φ

on the free degrees of freedom of the linear model, on the pressure-free (PF) route or the exact pressure (EX) route (D-r).
- K_ff is the assembled, restraint-reduced stiffness that the route's static solve forms. It is the case-independent part of the static K: thermal, pressure, wind and seismic loads do not enter it. [code] EX shares PF's assembly (`build_model_for_basis`, `assemble_basis_stiffness`, PP `lib.rs:4316`) with its own D-5 nominal-wall sections, and adds no load-dependent stiffness (`exact_admission.rs:117`: pressure stiffening "Not modelled").
- Nonlinear supports enter at the converged state of a named solved case (D-s), and snubbers when the request includes dynamic-only restraints (D-t).
- M_ff is a new mass matrix whose basis is the self-weight basis (§2).
- The only case-dependent input is the modulus basis (§1.4).

### 1.2 Admitted, refused by name, unchanged

| Family | F1 (M1) | Why |
|---|---|---|
| Straight members, tee legs | **Admitted**, consistent mass (§2) | [code] Ordinary PF frames (PP `lib.rs:2457-2480`, `BuiltModel.frame_elements`). |
| Realized arcs (DEC-070 curved macro-element) | **Admitted**, mass lumped as two rigid-body half-arcs (§2.4) | [code] One 12×12 element per bend from Castigliano end flexibility (`core/solver/curved_bend/src/lib.rs:1-9`, `:300-318`). It has no interior interpolation, so no consistent mass is defined for it, and lumping is the honest choice. |
| Geometry-only bends | **Admitted** as their straight members | [design] Euler–Bernoulli under either shear model (`pf-timoshenko.md`, #1663). |
| W-15 rigid-family components (valve, flange, reducer, rigid, specialty) | **Admitted.** Stiffness s·E and s·G as today. Mass is the component body W/a at its centre of gravity (a from §2.3), replacing the member's pipe mass under the **same predicate** that self-weight uses | [code] `core/product_physics/src/rigid_component.rs:35-37`; `self_weight.rs:3-9`, `:548-650`. |
| Linear supports: anchors, guides, line stops, vertical supports | **Admitted**: restrained DOFs are eliminated from K and M | [code] Support families (PP `lib.rs:8645-8655`). |
| Springs, variable spring hangers | **Admitted**: diagonal spring stiffness | [code] PP `lib.rs:8596-8600`. |
| Constant-effort supports | **Admitted, disclosed**: zero stiffness and zero mass | [code] Already excluded from K (PP `lib.rs:8567-8569`) and applied only as a load (PP `lib.rs:13903`), so it contributes nothing to a linear eigenproblem. If removing it leaves a mechanism, the existing singularity refusal applies. |
| Objective connectors (expansion joints), tie rods | **Admitted, massless, disclosed** | [code] Linear elastic (`core/solver/frame_kernel/src/connector.rs:1-3`; `tie_rod.rs:1-3`, "bilateral linear elastic"). Self-weight applies no weight to them (`self_weight.rs:569` filters to the rigid family), so massless keeps the two bases identical. |
| Annotation-only expansion joints | **Admitted as pipe**, with the existing disclosure | [code] `joint.rs:5-6`. |
| Imposed displacement (W-16) | **Admitted**: the DOF is a rigid support DOF and stays restrained; the motion value does not enter | [design] PF imposed displacement acts only on rigid support DOFs, by partitioned elimination (DEL-04-08 `Work/retained-pricing-rules.md` §8.23). |
| Nonlinear supports (one-way, gaps, lift-off, friction) | **Admitted, linearized at a named solved case, disclosed** (D-s, formerly M5): active or sticking, the support restrains its DOF with the stiffness the static solve uses in that state; inactive or sliding, its DOF is free. The case is a required request input (`MODAL_LINEARIZATION_CASE_REQUIRED`); an absent, unsolved or unconverged case is refused (`MODAL_LINEARIZATION_CASE_UNAVAILABLE`) | [decided] The stiffness depends on contact state, so a modal result needs a stated state; the user names it, which is not a default. [code] The converged states survive a solve as `nonlinear_support_active_set_state_code` rows (PP `lib.rs:7499-7530`). The result names the case and each support's state. |
| Exact route (`pressure_contract` present) | **Admitted** on its own section basis (D-5 nominal wall) and mass basis (`EXACT_MASS_METHOD`, §2.1), with every family EX itself admits (more as RT-A3 lands) | [decided] D-r: exit criterion 1 needs modal on EX. EX's stiffness is PF's assembly on its own sections, with no load-dependent term, and its self-weight basis gives the mass, so the identity of §2.5 holds on the route's own basis. Pressure is a load and is ignored, with EX's own disclosure that pressure stiffening is not modelled. |
| Load/reference-state documents | **Refused by name** (`MODAL_LOAD_REFERENCE_STATE_UNSUPPORTED`) | [decided] Their cases resolve per-case model states; a modal basis on one of them needs a stated case, a later slice. |
| Snubbers (model document 0.9.0, `SupportFamily` `snubber`) | **Admitted when the request includes dynamic-only restraints** (a required choice when the model has one, `MODAL_DYNAMIC_RESTRAINT_CHOICE_REQUIRED`): rigid in their translational DOFs or skew direction, or a spring of their stated stiffness; otherwise ignored. Disclosed either way | [decided] D-t: snubbers are dynamic-only restraints (SCA-014). A spectrum solve includes them (DEL-04-10). |
| Timoshenko straights (`model_settings.shear_model` on PF, `pressure_contract.shear_model` on EX, `timoshenko_annulus_energy_v1`) | **Refused by name on either route** (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`). **M4** admits them with a Timoshenko consistent mass | [design] PF Timoshenko has landed (#1663, model document 0.8.0). Agent 0, 2026-10-31: refused until M4; the v1.0 reference system uses Euler–Bernoulli members. [decided] Pairing a shear-flexible K with an Euler–Bernoulli mass would be inconsistent, so it is refused rather than mixed. |
| Retained route (route L/E, D1) | **Not entered** | [decided] F1 is a separate entry point with no retained-precision variant (§3.6). |
| Thermal, pressure-region, wind, seismic, nodal and member loads | **Ignored**: not inputs to a modal solve; disclosed, not refused | [decided] |

**[decided]** An unrestrained model (rigid-body modes) is refused through the static path's singularity refusal. Kernel tests may use a negative shift for free-free references (§4.3); the product path does not.

### 1.3 Boundary conditions

- **[code]** Rigid restraints are eliminated by partitioned reduction (`core/solver/frame_kernel/src/lib.rs:1666-1800`; sparse `structural/sparse.rs:798`, `reduce_assembled_sparse_system`). Linear springs enter the global diagonal (`assemble_sparse_stiffness(…, springs, …)`, `sparse.rs:599-606`).
- **[decided]** M is reduced by the same partition: M_ff keeps the free rows and columns.
  - Mass lumped at restrained DOFs drops out. The result discloses it ("mass at restrained DOFs, not participating") together with the total model mass, which keeps the effective-mass totals honest (§1.5).
- **[decided]** M's sparsity pattern is the union of the element patterns with K's. A rigid-body node block couples translation and rotation inside one node's 6×6 block, which K's pattern may not fill. The ordering and profile are therefore formed over the union pattern before reverse Cuthill–McKee (`core/solver/sparse_direct/src/lib.rs:505`), so that no M entry falls outside K's profile.

### 1.4 Inputs on the request (no model-document change in M1)

**[decided]** Every new parameter is on the modal analysis request, as `solver_mode` is today (PP `lib.rs:1053-1092`), not in `model_settings`. This needs no W-17 document version.
- **Extraction bound (required, no default).** Agent 0, 2026-10-27: "no built-in default. It's a required request input: a number of modes, a frequency cut-off, or both, whichever stops first. The desktop remembers the last value per project. No code-derived cut-off value ships."
  - `mode_count` n (positive integer), `frequency_cutoff` f_c (positive frequency quantity), or both. A request with neither is refused (`MODAL_EXTRACTION_BOUND_REQUIRED`).
  - The engine and the headless runner never supply a value. The desktop pre-fills the last value used in the project (M3, §5.1); the request records the value actually used.
- **[decided] The reported set** (D-c). Agent 0's "whichever stops first" governs, and cluster completion is its one stated extension.
  - **Bounds.** The reported set is the lowest modes, in ascending frequency, up to whichever bound stops first: the lowest n modes, or all modes with f ≤ f_c. The cut-off is **inclusive**. With both bounds, the set is the smaller of the two, and the result states which bound stopped it. Membership is decided on the computed frequencies; a reported or excluded mode whose error estimate (§3.4, check 2) reaches across f_c is disclosed by name.
  - **Cluster completion.** A cluster is the one defined in §3.4, check 5: a maximal run of adjacent computed eigenvalues, each within the relative cluster tolerance δ_c of the next. That is the only definition used, here, in the ScopeOfWork and in PRD §12.9. If the stopping boundary falls inside a cluster, the set is **extended to complete the cluster**, and the result discloses the extension by count ("n requested; m modes added to complete a cluster"). With a cut-off, this can include modes just above f_c, disclosed the same way. A partial cluster is never published, because the basis of a partial eigenspace would be arbitrary.
  - **Completeness.** The completeness count is taken at a **final shift** placed in the gap between the completed last reported cluster and the next computed cluster, under the separation premise of §3.4, check 4. It must equal the number of modes reported, cluster extensions included. With both bounds, the interval checked is the one actually reported. A final shift that cannot be placed with that separation refuses the set by name; it is never replaced by an unseparated count.
  - **The cut-off shift sizes; it does not decide completeness.** On the sparse path (M2), a cut-off-only request is sized by a count at ω_c²(1 + ε_s), with ω_c = 2πf_c and ε_s a stated relative offset (an algorithm parameter fixed in M2 and recorded here). If K − σM is singular there (the 1e-12 pivot guard) or the growth monitor rejects it (§3.4, check 4), the sizing count is retried at a second, documented offset, which the result records; the offset never changes silently. The sizing count only chooses q (§3.3). An error in it is caught by the final count, never published. The dense path computes the whole spectrum and needs no sizing count.
- **Modulus basis.** The same vocabulary as a load case: `modulus_basis_ref` or `modulus_basis_temperature`, or neither (base material E, as for a load case without one).
  - [code] Load cases carry these (PP `lib.rs:815-828`), and `materials_for_modulus_basis_observed` resolves them (PP `lib.rs:10842`, called at `:3161`). F1 reuses that resolver unchanged.
- **Weight-to-mass acceleration.** `weight_to_mass_acceleration` (a positive acceleration quantity). It is required when a rigid-family component's weight enters the mass, and it is never defaulted (§2.3).
- **`solver_mode`** chooses the path: `dense_scrutiny` (M1b) or `sparse_interactive` (M2).
- **`linearization_case`** (D-s): the id of a solved case whose converged support states linearize the nonlinear supports. Required when the model has one, refused when absent; recorded with each support's state.
- **`include_dynamic_only_restraints`** (D-t): a boolean, required when the model has a snubber, with no default. A response-spectrum solve sets it (DEL-04-10).

### 1.5 Outputs

**[decided]** (D-d) Every published quantity has integer unit exponents, so each one is expressible in the unit catalog (`core/units/src/lib.rs:26-56`, `DimensionVector` of `i8` exponents, `:224`). A mass-normalized shape (φᵀMφ = 1) would not be: its translations are in kg^-½, its rotations in rad/(m·kg^½) and its participation factors in kg^½. Per mode i, in ascending frequency:
- f_i (Hz), ω_i (rad/s) and T_i (s).
- The **mode shape**, as six components per node in global axes, in exactly one of two normalization cases. Each mode records its case (`unit_max_translation` or `unit_max_rotation`), and each row carries its own unit. Restrained DOFs are published as exactly 0. Ties for the largest component go to the lowest global DOF index.
  - **The "no translation" threshold.** Let t = the largest free translational component, θ = the largest free rotational component (rad) and ℓ = the diagonal of the bounding box of all the model's nodes, free and restrained (m), which is positive for any model with an element, including a one-element cantilever with one free node. A mode has **no translation** when both of the following hold, and otherwise it is case (a):
    1. t ≤ τ_t·ℓ·θ, with τ_t = 1·10⁻⁶ (dimensionless). It is an algorithm parameter, not an engineering default. M1b confirms it on the pure-torsion and rigid-scaled references (§4) at no less than 100 times the largest translation found there in a mode that is rotational in exact arithmetic, and records the measurement here. It is only tightened (DEC-026).
    2. Every translational effective-mass fraction (below) is at most τ_t. Rotation–translation coupling in M (the consistent beam mass, an offset centre of gravity) can give a rotation-only shape a translational participation. Case (b) is used only where that participation is also negligible.

    A mode that meets condition 1 but not condition 2 is refused by name (`MODAL_NORMALIZATION_UNRESOLVED`, naming the mode). It is not published under either case.
  - **(a) A mode with nonzero translation:** normalized to **unit maximum translation**. The largest-magnitude translational component is +1. Translations are dimensionless (m/m) and rotations are in rad/m. The generalized mass is in kg, Γ is dimensionless and the effective mass is in kg.
  - **(b) A mode with no translation**, such as pure torsion of a straight run: normalized to **unit maximum rotation**. The largest-magnitude rotational component is +1 (rad/rad). The translations are the computed vector's own, in m/rad, and are not altered: under the conditions above they are at most τ_t·ℓ. The generalized mass is in kg·m². Γ (in rad/m) and the effective translational masses (kg) are computed from that same published vector, so each translational effective-mass fraction is at most τ_t; none is set to zero. The integrity checks (§3.4) are evaluated on the published vector itself, so the evidence describes the published shape. A display may show translations below τ_t·ℓ as 0, and says so; the published values are unchanged.
- The **generalized mass** m_i = φᵢᵀ M_ff φᵢ: kg in case (a), kg·m² in case (b).
- For k = X, Y, Z, with R_k the rigid unit translation over **all** DOFs (1 on each node's k translation, 0 elsewhere) and p_k = (M R_k)_f = M_ff R_k,f + M_fc R_k,c the uniform-support-motion excitation of the free DOFs (DEL-04-10 Design §2, D-v):
  - participation factor Γ_ik = φᵢᵀ p_k / m_i: dimensionless in case (a); in case (b), in rad/m, computed from the published vector and negligible (its effective-mass fraction at most τ_t);
  - effective mass m_eff,ik = (φᵢᵀ p_k)² / m_i: kg, normalization-invariant in both cases; in case (b) computed from the published vector, each translational fraction at most τ_t;
  - effective-mass fraction m_eff,ik / T_k (dimensionless), with T_k = R_kᵀ M R_k the model's total mass in direction k, and its cumulative sum. Because the modes are M-orthogonal, Σ_i m_eff,ik ≤ T_k over any set of modes (Bessel's inequality in the M semi-inner product, with φ̂ᵢ the mode extended by zeros on the restrained DOFs: φᵢᵀp_k = φ̂ᵢᵀM R_k). Over all modes the shortfall is the mass that moves with the supports, which the result states.
  - Γ, m_i and m_eff are what F2 needs: a modal response is Γ_ik·φᵢ times a modal coordinate, whatever the normalization.
- Per-mode integrity evidence (§3.4) and a cluster id when eigenvalues are repeated or nearly repeated (§3.4).
- **[decided] Units for these quantities.** The catalog has Time (s) but no frequency, angular-frequency, rotation-per-length, length-per-rotation or mass-moment-of-inertia family. These families are an extension of the unit catalog, owned by DEL-02-02 (`core/units/**` is in its code paths). Hz is canonical for frequency, rad/s for angular frequency, and s (Time, already present) for period. The extension is a gating input of M1b (`deliverable.yaml`). `core/units/src` is a priced crate directory (`tools/retained_pricing/base/crate_dirs.txt`), so the change that makes the extension re-registers D3 (§3.6).

Per result:
- total model mass by category: pipe metal, contents, insulation and component bodies (this also gives the survey's A9 weight summary);
- free-DOF mass per direction;
- the Sturm count, its final shift, that shift's separation ratio and pivot-growth factor, and the premise under which the count is relied on (§3.4, check 4);
- the extraction bound used, which bound stopped extraction, any cluster extension by count, and, on the sparse path, the sizing shift used for a cut-off with its offset.

Rotational participation (about axes) is left to F2.

---

## 2. The mass model, on the self-weight basis

Agent 0, 2026-10-27: "consistent mass on straights, rigid-body lumps on arcs and components, on the self-weight basis, with g·M·1 = self-weight as a regression. Agreed. Settle the component W/a basis in Design, with Codex review."

### 2.1 What self-weight forms today

- **[code]** Self-weight is not computed in the solve. It is a **generated model edit**: `generate_self_weight_operations` drafts primitive loads with evidence and a method string (`self_weight.rs:135-142`). An inspection mode (`generate_operations(…, true)`, `self_weight.rs:142`, `:1363`) regenerates a plan for lineage checks without the source-hash and provenance checks (`:157`, `:241`, `:325`).
- **[code]** The gravity of a self-weight case is a request input: `SelfWeightGravity { value, unit, axis }` (`self_weight.rs:20-35`). The axis must be `global_x`, `global_y` or `global_z`. The value must be finite and nonzero, is signed, and is normalized to m/s² through the unit catalog (`:175-205`). The model document has no model-level gravity, and the product applies no physical-constant default for it (seismic generation states the same rule: "no physical-constant default is applied", PP `lib.rs:10491`).
- **[code]** Pipe self-weight is `mass_per_length × normalized acceleration`, applied as a distributed force along the global gravity axis on every selected member, whatever the member's orientation (`self_weight.rs:381-384`, `:416`).
- **[code]** Mass per length on the PF basis (`STABLE_MASS_METHOD = "source_od_effective_wall_areas/v1"`, `stable_mass_per_length`, `self_weight.rs:731-845`):
  - metal: `source_wall_area_m2(OD, t − c_m) · material_density`;
  - contents: `source_bore_area_m2(OD, t − c_m) · contents_density`;
  - insulation: `source_insulation_area_m2(OD, t_ins) · insulation_density`.
  - Density is required ("explicit material density required"). Insulation needs both thickness and density, and absence is never a zero default.
  - The inputs are on the pipe section (PP `lib.rs:509-536`); the area functions are in `annulus_geometry.rs:54-61`.
- **[code]** Rigid-family components (W-15 slice b): for a selected member that an eligible component maps, the member's distributed self-weight is **replaced** by the component's weight. It is applied at the node as F = W·ĝ and M = r × F, with r the centre-of-gravity vector in global axes from the node (`self_weight.rs:3-9`, `:503-516`, `:548-650`).
  - W is the authored force. The acceleration contributes only its sign: `wrench` takes `f = if acceleration > 0.0 { weight } else { -weight }` (`self_weight.rs:507-508`). [design] The W-15 reference states the convention: "The acceleration's magnitude does not scale W: W is the user's force." (`validation/references/w15_component_weight/README.md`, Convention.)
  - The component primitive's evidence records the case's acceleration as `normalized_acceleration_m_per_s2` (`self_weight.rs:707-708`).
  - A component that fails a condition (consumption not geometry-only, missing weight or centre of gravity, mapping conflict) leaves the member its pipe weight, with a warning naming it (`self_weight.rs:600-620`; `COMPONENT_WEIGHT_NOT_APPLIED`, `:473`).
- **[code] The EX mass basis** (D-r). The same function serves EX with `exact = true` (`self_weight.rs:311`, `:784-798`): `EXACT_MASS_METHOD = "source_od_nominal_wall_pressure_bore_areas/v1"` (`self_weight.rs:855`), metal on the nominal wall t and contents on the pressure bore from t_r = (t − c_m) − c_a, with an explicit `corrosion_allowance` required; insulation as on PF. EX seismic generation uses the same basis (`exact_generation_mass_per_length`, `self_weight.rs:844`). F1 on EX takes its m̄ from this method, and the component-weight path (`component_weights`) is route-independent.
- **[code] Finding: a second mass basis.** Seismic equivalent-static generation forms its mass with `compute_pipe_mass_per_length` (PP `lib.rs:10293-10420`, called at `:10502`), not with `stable_mass_per_length`.
  - It forms π/4·(OD² − ID²) directly, not through the annulus source-area functions.
  - It loops over every pipe segment and does not substitute component weights for rigid-family members.
  - So the product has two PF mass bases today. **[decided]** F1 joins the self-weight basis, which is the one the static weight check can confirm.
  - Agent 0, 2026-10-27: "record it as a finding. It goes on the remaining-items list beside the deferred PF seismic-mass alignment, since both should reconcile on one mass basis. No action in F1." It is recorded in `docs/COMPLETENESS_MAP.md` §4.3 (Remaining), in the row with the PF seismic-mass alignment to D-5 that Agent 0 deferred on 2026-10-27 (DEL-05-01 Design `exact-equivalent-static.md`, W-16c).

### 2.2 Element masses

**[decided]** (D-b)

| Element | Mass matrix |
|---|---|
| Straight, Euler–Bernoulli | **Consistent.** Hermitian-cubic transverse mass m̄L/420·[156, 22L, 54, −13L; 22L, 4L², 13L, −3L²; 54, 13L, 156, −22L; −13L, −3L², −22L, 4L²] in both bending planes. Linear axial m̄L·[1/3, 1/6; 1/6, 1/3]. Linear torsional j̄L·[1/3, 1/6; 1/6, 1/3]. Rayleigh rotary-inertia terms ī/(30L)·[36, 3L, −36, 3L; 3L, 4L², −3L, −L²; −36, −3L, 36, −3L; 3L, −L², −3L, 4L²] in both planes. Transformed with the member's own frame orientation. Standard results, e.g. Przemieniecki, *Theory of Matrix Structural Analysis* (1968), ch. 11; cited, not copied. |
| Realized arc | **Lumped**: two rigid half-arcs, each a 6×6 rigid-body mass about its end node, with the half-arc's true mass, first moment (centroid offset) and second moment (§2.4) |
| Rigid-family component body | **Lumped**: one 6×6 rigid-body mass about the component node. Mass m = W/a (§2.3), offset r = the centre of gravity, rotational block m·[r]×ᵀ[r]× (a point mass at the centre of gravity; the body's own inertia about its centre of gravity is not authored, so it is zero, and that is disclosed) |

Per-length quantities on the self-weight section basis:
- m̄ = metal + contents + insulation, exactly `stable_mass_per_length` (`self_weight.rs:731`) on the model's route (PF or EX, §2.1).
- j̄ (polar, per length) = ρ_p·J_metal + ρ_ins·J_ins, with J_metal on the metal wall of the route's mass basis (t − c_m on PF, t on EX), with J of an annulus = A(r_o² + r_i²)/2. Contents are excluded (D-e).
- ī (bending rotary, per axis, per length) = j̄/2 for the circular sections PF has.

Reasons:
- **Accuracy.** For conforming elements, consistent-mass frequencies converge from above (Rayleigh–Ritz), with O(h⁴) frequency error for the Hermitian beam. A lumped diagonal mass converges at O(h²) and gives no bound.
  - On one simply supported span, one consistent element gives ω₁ ≈ 10.95·√(EI/m̄L⁴) against π² ≈ 9.87, about 11 % high; two lumped elements give √96 ≈ 9.80, about 0.7 % low. These figures are stated from memory and are re-derived by the reference generator.
  - Either way, long spans need intermediate nodes (§2.6).
- **Memory.** Consistent M has the element pattern of K. In profile storage it costs one more P-entry array (8P B), where P is K's profile entry count (`sparse_direct/src/lib.rs:283`). A diagonal lumped M costs 8n B. K − σM needs one profile either way.
- **Robustness.** A purely translational lumped mass makes M singular. Both paths reduce with K, not M (§3.2, §3.3), so a singular M only adds infinite eigenvalues, which are never extracted.
- **Checkability.** Consistent M gives an element-wise weight identity (§2.5): g·M·r_k reproduces the consistent nodal vector of a uniform load, moments included. A lumped M gives only a resultant identity.
- A lumped-diagonal variant exists **only as an internal control** in the references, not as a user option, which would double the reference and reader matrix for little gain.

**[decided]** (D-e) Contents are in all three translations, as self-weight applies m̄ along any global axis whatever the member's direction (§2.1). This keeps g·M·r_k equal to self-weight in every direction, member-axial directions included. Contents are excluded from torsional and rotary inertia, which is disclosed in the result.

### 2.3 Component bodies: the W/a basis

**[decided]** (D-f) The component mass is m = W/a with a = `weight_to_mass_acceleration`, a required request input whenever a component's weight enters the mass. There is no default. The reasoning:

1. **What the self-weight basis fixes.** For a self-weight case at acceleration g (normalized, signed), the generator applies m̄·g per length to each pipe and W, with the sign of g, to each eligible component (§2.1). The pipe term scales with |g|; the component term does not.
2. **The identity that defines "the same basis".** The mass basis equals the self-weight basis when, for the same acceleration, g·M·r_k reproduces the self-weight loads. For the pipe terms this holds for any g, since m̄ is independent of g. For a component it requires |g|·m = W, so m = W/|g|.
3. **Where |g| comes from.** No model-level gravity exists, and the product ships no physical constant for it (§2.1). The g of a self-weight case is that case's request input, recorded only in the generated primitives' evidence. So there is no consistent a without an input: the modal request must carry it, in the same vocabulary and normalization as the self-weight request (an acceleration quantity through the unit catalog), with only its magnitude used, so it must be positive.
4. **When it is required.** It is required exactly when the shared eligibility function (`component_weights`, below) yields at least one component body. A request without it is then refused (`MODAL_WEIGHT_ACCELERATION_REQUIRED`, naming the components). For a pipe-only model it is optional; if given, it is recorded and has no effect on the mass.
5. **Agreement with the model's own weight cases.** When the model carries a current applied self-weight case with component-weight primitives (`inspect_applied_self_weight`, `self_weight.rs:1266`; method `COMPONENT_WEIGHT_METHOD`), their recorded `normalized_acceleration_m_per_s2` magnitude must equal a after normalization, bit for bit. Otherwise the request is refused (`MODAL_WEIGHT_ACCELERATION_MISMATCH`, naming the case and both values). The reason: with a different a, the modal component masses would not be the masses implied by the static weight the model already carries. A stale or manually overridden component primitive is disclosed and does not take part in the comparison.
6. **Binary64 range and rounding.** The premise has three parts, each checked in binary64 before the mass is formed:
   - W and a are positive normal binary64 numbers;
   - m = fl(W/a) is a normal binary64 number;
   - fl(a·m) is finite and normal.

   Outside that premise the request is refused by range (`MODAL_COMPONENT_MASS_OUT_OF_RANGE`, naming the component and the operand). The third part is needed because W normal and W/a normal do not exclude overflow or underflow of a·m when W lies within a few ulps of the binary64 range's ends. Under the premise, m = (W/a)(1 + ε₁) and fl(a·m) = W(1 + ε₁)(1 + ε₂), with |ε₁|, |ε₂| ≤ u = 2⁻⁵³. So a·m differs from W by a relative amount of at most 2u + u², about 2.2·10⁻¹⁶, and is not always equal to it. The identity check (§2.5) is judged within its measured tolerance and against the modal formation check (§3.4); it is exact only in real arithmetic.

**[decided]** F1 calls the **same** eligibility function as self-weight (`component_weights`, `self_weight.rs:548`, exposed `pub(crate)`), over all members, so each predicate is read once and the two bases cannot drift.
- Where a component's weight applies, the member's distributed mass is replaced by the component body.
- Where it does not, the pipe mass stays and the same `COMPONENT_WEIGHT_NOT_APPLIED` warning is emitted.

**[code]** The model has no other concentrated mass: no node mass field and no mass on non-rigid-family components (PP `lib.rs:558-582`, `:586-650`). Valves and flanges are rigid-family, so their weights are covered.
- **[open]** A free-standing concentrated mass (an instrument, an actuator on a non-rigid member) or a component's own rotary inertia needs a model-document field (W-17). It must enter self-weight and modal together to keep the bases identical (§9).

This basis is settled here and goes to an independent Codex review, as Agent 0 directed (§9).

### 2.4 Arcs

- **[code]** Self-weight on a realized arc uses `consistent_uniform_nodal_loads`, documented as in equilibrium with the distributed-load resultant (`curved_bend/src/lib.rs:509-528`). The arc length is available (`:395`).
- **[decided]** Each half-arc (angle θ/2, mass m̄·R·θ/2) is a rigid body about its end node, with its exact centroid and second moments; both are closed-form integrals of a circular arc.
  - Together the two lumps reproduce the arc's total mass and centroid, and so the self-weight resultant force and moment exactly in real arithmetic.
  - They do not reproduce the nodal split of `consistent_uniform_nodal_loads`, which depends on the flexibility factors. Arcs are therefore checked on the resultant (§2.5).
  - User flexibility factors change stiffness only, never mass.
- **[decided]** A typical bend (R ≈ 1.5 D) is short against its spans, so lumping is adequate. A long-radius bend falls under the discretization warning (§2.6). The product cannot subdivide a macro-element; the user can author two bends.

### 2.5 The weight-basis identity check

Agent 0's regression, "g·M·1 = self-weight", is a **blocking integrity check on every modal result**.
- For each global direction k, form F_k = a·M_full·r_k on the full (unreduced) mass, with a the request's acceleration, or 1 m/s² when no component weight enters. (With no component, the mass is independent of a; multiplying by 1 is exact in binary64, and the check scale publishes nothing.)
- Run the self-weight generator in inspection mode over **all** members (`generate_operations(…, true)`), with gravity a along k, on the model's route, so that on EX the exact mass method is compared.
- Compare. The identity is not literal vector equality for every family: it holds **nodally for straight members**, and **on the resultant force and moment for arcs and components**, each within a measured tolerance.
  1. **Straight members, nodally.** With the Hermitian consistent mass, M_e·r_k is, in real arithmetic, the consistent nodal vector of a uniform load: forces a·m̄L/2 at each end and moments ±a·m̄L²/12. The rotary and torsional terms vanish for a rigid translation. Compare with `straight_pipe`'s uniform-load vector for the generated intensity. [design] The PF Timoshenko design keeps the Euler–Bernoulli vector for full-span uniform loads (`pf-timoshenko.md`, #1663), which M4 relies on.
  2. **Arcs and components, on the resultant.** Total force, and total moment about a fixed point (the node centroid, to limit cancellation). The half-arc lumps do not reproduce the arc's nodal split (§2.4), and a component body is compared as the wrench W·ĝ, r × W·ĝ (§2.1). Straight members also enter the resultant.
- **The tolerance: measured, then pinned (DEC-026).** Until M1a measures it, the identity tolerance has no value and is `TBD`, and no modal result is published. M1a evaluates the identity on the frozen `f1_modal` mass references and its test models, and records the largest relative and absolute discrepancies, nodal and resultant, with the models and the build. The tolerance is then pinned as a relative and absolute pair at a stated multiple of those discrepancies and recorded in this design and in the test. From then on it is only tightened; loosening it would be a governance event under DEC-026, not a code edit.
- A failure blocks with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`, naming the member or component.
- If the model also carries a current applied self-weight case along k whose scope covers every member, its resultants are compared as a second, **non-blocking** row. A partial-scope case or a manual override (`MANUAL_OVERRIDE_METHOD`, `self_weight.rs:848`) gives a disclosure, not a failure. The acceleration rule in §2.3 item 5 is separate and blocking.

### 2.6 Mass discretization adequacy

- **[design]** The survey asks for SWBPIPE's own spacing rule, derived and benchmarked (survey R8).
- **[decided]** (D-l) A warning, not a refusal:
  - for each straight member, compute the first fixed-fixed frequency of the member alone, f_e = (4.7300²/2π)·√(EI/(m̄L_e⁴));
  - warn (`MODAL_MASS_DISCRETIZATION_COARSE`, naming the member) when f_e < c·f_max, where f_max is the highest reported frequency;
  - c is derived by benchmark in M1a, so that a span meshed at the threshold is within a stated error of its continuum frequency (with consistent mass, c ≈ 2–3 is expected). The factor and its derivation are published in the warning text.
- A helper that adds intermediate nodes is a model operation through the operation applier (user-visible, undoable): a separate slice (M6), not part of the solve.

---

## 3. Eigen-solver: dense and sparse paths

### 3.1 What exists

- **[code]** `PreviewSolverMode::{SparseInteractive, DenseScrutiny}`, sparse by default (PP `lib.rs:1053-1092`).
- **[code]** The dense scrutiny guard counts 96 B per n² entry (`DENSE_SCRUTINY_BYTES_PER_ENTRY`, PP `lib.rs:4279`) against a provisional 6 GiB ceiling (`DENSE_SCRUTINY_CEILING_BYTES`, `:4283`). That admits at most 8,192 DOF. It refuses before any n² allocation, and there is no automatic dense fallback (`dense_scrutiny_guard`, `:4303`; applied at `:3041-3048`).
- **[code]** The sparse solver (DEC-023, "zero new numerical dependencies") is reverse Cuthill–McKee ordering plus skyline LDLᵀ (`sparse_direct/src/lib.rs:1-11`, `:377-458`):
  - single-threaded, fixed operation order, std only;
  - no pivot reordering;
  - pivots at or below an absolute 1e-12 refused (`SPARSE_SOLVE_ZERO_PIVOT_GUARD`, `:23`);
  - **negative pivots counted and located**, not refused (`FactorizationReport.nonpositive_pivot_count`, `first_nonpositive_pivot`, `:126-130`);
  - `SymmetricProfileMatrix::from_entries_with_order`, `factorize_ldlt` and `ProfileFactorization::solve_ordered` are public (`:215`, `:382`, `:339`).
- **[code]** The frame kernel's positive factor has dense and profile backends (`structural.rs:2103-2133`). Its `solve` is private, and `finish_structural` is the only completion seam (`:2116`, `:2134-2141`).
- **[code]** The only eigenvalue routine in the tree is a cyclic Jacobi for condition-number evidence (`jacobi_symmetric_eigenvalues`, `core/solver/performance_harness/src/lib.rs:993`).
- **[decided]** No new numerical dependency (DEC-023). Both paths are written in-house, std only.

### 3.2 Dense path (M1b): reduce with K's Cholesky, tridiagonalize, implicit QL

**[decided]**
1. Assemble dense K_ff and M_ff.
2. Factor K_ff = R Rᵀ (Cholesky). K_ff is SPD on an admitted model; a non-positive pivot refuses as singular, as the static path does.
3. Form C = R⁻¹ M_ff R⁻ᵀ (symmetric positive semidefinite) with two triangular solves, symmetrizing from the lower triangle.
4. The eigenvalues of C are μ = 1/ω². The largest μ are the lowest frequencies, so the wanted modes are the **best-conditioned** end of C's spectrum, and a singular M only adds μ = 0.
5. Householder tridiagonalization, then implicit QL with Wilkinson shifts and accumulated transformations (the tql2 algorithm). Public algorithm: Golub & Van Loan, *Matrix Computations*, ch. 8; written in-house.
6. Back-transform φ = R⁻ᵀ z, then normalize (§1.5).

- **Control.** The Jacobi pattern, generalized, runs on C in tests for models up to a few hundred DOF. It is an independent kernel-level check of the QL path, not a product path.
7. **[decided] The completeness count on the dense path** (M1b). The Cholesky factor of K gives no inertia of K − σM. The dense path takes its Sturm counts (§3.4, check 4) from **`sparse_direct`'s `factorize_ldlt`**, on the scaled profile of K − σM, with the read-only observation accessor of §3.3 for the growth monitor. There is no dense LDLᵀ of its own. The reason: one inertia implementation and one growth monitor serve both paths, on an existing, public, tested factorization, and the inertia-risk tests aim at the code the product actually uses. The independent counts in tests come from the reference generator (§4.1) and the Jacobi control. So the accessor, the shift placement and the growth limit are M1b's, with their D3 obligation (§3.6), and M1b publishes dense results with every check of §3.4.

- **Control.** The Jacobi pattern, generalized, runs on C in tests for models up to a few hundred DOF. It is an independent kernel-level check of the QL path, not a product path.
- **Memory.** K, M, C and the accumulated Q are alive together, plus one n² work array, plus, for a Sturm count, the shifted profile and its clone inside the factor (each at most n(n + 1)/2 entries): about 48·n² B. The re-formation of §3.4 is element by element and stores no global matrix.
- **Time.** Cholesky n³/3, forming C about n³, tridiagonalization 4n³/3, QL with vectors about 6n³: roughly 9n³ flops, plus at most n³/3 for each Sturm factorization. n = 3,000 is about 2.4·10¹¹ flops (minutes, single-threaded, optimized build); n = 8,192 is about 5·10¹² (an hour or more). The registered retained identity is a debug, opt-level 0 build (DEL-04-08 `Work/retained-pricing-rules.md` §9), so every timing statement names its build.
- **[decided]** (D-k) Two guards, both applied before allocation and stated with their formula in the refusal text, as `dense_scrutiny_guard` is:
  - **Memory:** `DENSE_MODAL_BYTES_PER_ENTRY` is the count of live n²-sized binary64 buffers times 8 at the peak phase (48 for the layout above; the constant states the implementation's actual count), against the existing provisional 6 GiB ceiling. That admits n ≤ about 11,600 DOF.
  - **Work:** 9n³ + s_max·n³/3 against a fixed operation budget, with s_max the stated largest number of candidate Sturm shifts (§3.4, check 4). M1b sets the budget by measurement under `t3_exclusive.sh` so that the dense path stays a scrutiny path for small and medium models (about 3,000 DOF is the target). The value and its measurement are recorded in this design when M1b lands.
  - These are resource limits, not modelling defaults; they refuse and never truncate the extraction.

### 3.3 Sparse path (M2): shift-invert block Lanczos on the existing LDLᵀ

**[decided]** (D-m)
- **Operator.** (K − σM)⁻¹M in the M-inner product, with σ = 0 unless the slicing below chooses otherwise; K's own factor, all pivots positive, is the common case.
- **Block Lanczos.** A scalar Lanczos run from one start vector spans at most one direction of each repeated eigenspace, so it cannot recover a repeated-mode subspace. A straight circular-pipe cantilever has exactly paired bending modes. F1 therefore uses **block Lanczos**:
  - **Block size.** b₀ = 2. Its basis is the paired bending modes of an axisymmetric section, the multiplicity the product's circular sections produce. b is an algorithm parameter, not an engineering default. A higher multiplicity, from a symmetric layout, is detected by the Sturm count (§3.4, check 4) and recovered by the restart below.
  - **Deterministic start block.** b columns from a fixed-seed generator (the algorithm and the seed are stated in the code and recorded with the result), M-orthonormalized by Gram–Schmidt with one reorthogonalization, in fixed order. The start block is deliberately not the rigid translations r_k: they are M-orthogonal to every mode with zero participation, such as the antisymmetric modes of a symmetric layout, which exact arithmetic would then never find.
  - **Full M-reorthogonalization** of each new block against the whole basis, in fixed operation order. The projected block-tridiagonal matrix is solved with the M1b dense routine (Householder tridiagonalization and QL).
  - **Locking and deflation.** A Ritz pair whose residual backward error (§3.4, check 2) meets the convergence tolerance is locked. Later blocks are M-orthogonalized against the locked vectors, so a converged pair is never recomputed or perturbed.
  - **Breakdown.** If a column of a new block loses almost all of its M-norm to orthogonalization (below a stated relative threshold), the basis has found an invariant subspace or the block is rank-deficient. The column is replaced by the next column of the fixed-seed generator, orthogonalized against the basis and the locked vectors, and the event is recorded. If no replacement survives, the free space is exhausted and the basis is complete.
  - **Basis size.** q = max(2k, k + 8), rounded up to a multiple of b, grown if not converged, without thick restart in M2. The run also converges the next cluster above the reported set, which is not published, so that the final shift (§3.4, check 4) lies in a gap between computed clusters. If the free space is exhausted first, the reported set is the whole finite spectrum.
  - **A missed direction.** If the Sturm count exceeds the number of converged modes, a direction was missed. The run restarts deterministically with block size 2b and the next seed of a fixed sequence. The number of restarts and the largest b are algorithm parameters stated in the code. If the counts still disagree when they are exhausted, the set is refused (`MODAL_STURM_COUNT_MISMATCH`). A count disagreement is never published.
- **Cut-off mode.** Spectrum slicing: repeat at shifts σ_j inside (0, ω_c²] until the sizing count at the cut-off (§1.4) is exhausted. Completeness is then decided by the final count of §3.4, check 4, not by the sizing count. Each slicing shift is placed in a gap between computed clusters, under the same separation premise as the final shift.
- **Built on `sparse_direct`'s existing public API, plus the read-only accessor that M1b adds:**
  - assemble K − σM over the union pattern;
  - apply the symmetric diagonal scaling D(K − σM)D, D = diag(|k_ii|^-½). This is a congruence, so it **preserves inertia**, and it makes the absolute 1e-12 pivot guard scale-free;
  - then `factorize_ldlt` and `solve_ordered`.
  - **[decided] The observation seam.** `ProfileFactorization` keeps its factor private (`sparse_direct/src/lib.rs:325-327`), and its report carries only the extreme pivot magnitudes and the negative-pivot count (`:119-131`), so max|L_ij| is not observable through the present API. F1 adds one narrow, **read-only** accessor to `ProfileFactorization`: it returns max|L_ij| over the strictly lower factor, and the pivot magnitudes |d_j| in ordered position as a borrowing iterator. It adds no mutation and no allocation. This is an **API addition to `sparse_direct`**, owned there by DEL-04-01, made in **M1b**, because the dense path's Sturm counts use it (§3.2, item 7). M2 uses it unchanged. `core/solver/sparse_direct/src` is a priced crate directory and its `lib.rs` is named in the pricing rules (`tools/retained_pricing/base/crate_dirs.txt`; `rules/o10_rules.py:3020`), so the change that adds it re-registers D3 (§3.6).
- **The unpivoted-LDLᵀ inertia risk is a first-class test target** (Agent 0, 2026-10-27; R2). `factorize_ldlt` does no pivoting, and on an indefinite K − σM element growth, or a shift too close to an eigenvalue, can corrupt the negative-pivot count. Nothing in binary64 makes the count exact; the count is relied on only under the separation premise of §3.4, check 4.
  - Mitigations, from M1b: place each shift in a gap between computed clusters, separated from both by a stated multiple of the estimated backward error of its factorization; read the growth (max|L_ij| and the |d_j| against the scaled diagonal) through the seam above; refuse a shift whose growth exceeds the stated limit, and try the next candidate position in the same gap. The growth limit and the separation multiple are algorithm parameters, fixed in M1b by the inertia-risk suite and recorded here (§9, O-6).
  - **Tests aimed at it** (M1b for the dense path's counts, extended in M2 to slicing shifts): models built to provoke growth (near-coincident eigenvalues, clusters, rigid-family scales 1e3–1e6, shifts placed close to eigenvalues), each checked against the reference generator's exact counts, and in M2 against the dense path within the dense guard, with the growth monitor's verdict and the separation decision asserted. They include Codex's round-2 witness: a 32×32 tridiagonal on which a shift within 10⁻¹⁷ relative of an eigenvalue rounds away and gives a wrong count with unchanged growth observations. The separation rule must refuse that shift, not count it.
  - If no shift can be placed under the premise, the affected set is refused by name, never published with an unchecked count.
- **[decided] Memory: the peak, counted.** At factorization, four P-entry value arrays are live together: K's profile, M's profile, the shifted and scaled profile, and its clone inside the factor (`factorize_ldlt` clones its input, `sparse_direct/src/lib.rs:386`). Each `SymmetricProfileMatrix` also carries three index arrays of about n entries (`:142-148`). The guard counts the peak live allocation of the phase that is largest, as the implementation states it:
  - B = 32P + 8n(q + b) + 24q² + c·n bytes;
  - 32P: the four value arrays; 8n(q + b): the Lanczos workspace, basis and current block; 24q²: the projected problem and its vectors; c·n: the per-DOF metadata (the index arrays of four profiles, the factor's work vector and the scaling vector; about 112 B on a 64-bit target), with c stated by the implementation. Assembly transients (the entry list for K − σM) are counted if they outlive into the factorization phase, and otherwise form their own phase in the maximum.
  - Example: n = 60,000 DOF, mean half-bandwidth 60 (P ≈ 3.6·10⁶), k = 50, b = 2, q = 110: about 115 MB of profiles, 54 MB of Lanczos workspace and 7 MB of metadata, about 180 MB.
- **Time.** Factorization ≈ n·b̄² (≈ 2·10⁸ flops in the example), with b̄ the mean half-bandwidth; each block step ≈ b·(6P + 4nq) flops. Each Sturm check costs one more factorization.
- **[decided]** The sparse memory guard states B against a ceiling before allocation, in the same style as the dense guard.
  - **Admission is rechecked whenever q grows**, and whenever b grows on a restart, before the new allocation. A request is refused with the formula, never silently truncated.
  - **A cut-off-only request** is sized from the sizing count at the cut-off (§1.4), taken before Lanczos. That first factorization is admitted on its own term (32P + c·n). The count k_c then sets q.

### 3.4 Numerical integrity checks

**[decided] What the checks are** (D-n, Codex round 2). They are **numerical-integrity checks with stated premises**, not certified enclosures. Binary64 LDLᵀ inertia counts, a Rayleigh quotient on the formed pencil and one floating-point solve cannot enclose an eigenvalue without interval or exact arithmetic. Codex gave reproducible witnesses of each (§3.3; §9, O-13). Each check below therefore states what it computes, the premise under which it is relied on, and what it does not establish. This is the posture of the PF route's existing limitation: "numerical integrity does not establish physical correctness" (`preview_physics.rs:60`). No eigenvalue bound is certified, and no Kato–Temple refinement or inverse-norm bound is used. Rigorous enclosures are a later option (§9, O-13).

The checks re-form the model from authored operands, as K-D5 does (`frame_kernel/src/structural/formation_check.rs:1-12`), but the modal checks have criteria of their own. K-D5's criterion (D5C1) judges a static displacement correction against response scales (`formation_check.rs:972-995`). It is not an entrywise comparison of K, and a modal run has no static response, so it is not reused.

Notation: D = diag(|k_ii|^-½) of the formed K_ff (§3.3); a tilde marks a scaled quantity (K̃ = DKD, M̃ = DMD, φ̃ = D⁻¹φ); ‖·‖ of a matrix is the ∞-norm; u = 2⁻⁵³.

1. **[decided] Re-formation and formation difference** (D-o).
   - **Re-formation in double-double.** K and M are re-formed **independently from their leaves** in at least **double-double precision (≈106-bit significand)**. For K, the authored operands are the element geometry, sections and moduli, the bends' radius and plane reference, the connector and tie-rod stiffnesses, the spring stiffnesses and the rigid-family scaling. For M, they are OD, wall, mill tolerance, densities, insulation, lengths, arc radius and angle, W, the centre of gravity and a. The binary64 matrices used by the solve are not inputs. The re-formation is element by element; no re-formed global matrix is stored.
   - **Why double-double, not true quad.** The re-formation's own rounding, and so the resolution of the measured formation difference, is at the 10⁻³¹ relative scale for well-scaled entries (a few units of 2⁻¹⁰⁶ over an element formula's few operations). The formation difference itself is at the 10⁻¹⁶ scale, so it is resolved to about 15 digits, and the residual thresholds of check 2 lie above 10⁻¹². A 128-bit or 113-bit significand would add nothing a check can use. Entries formed by heavy cancellation are less well scaled; the statement is for well-scaled entries, and the formation difference is measured, not assumed.
   - **Formation difference.** ΔK = K − K̂ and ΔM = M − M̂ (formed minus re-formed) are measured as ‖DΔKD‖ and ‖DΔMD‖, with row sums accumulated element by element (an upper estimate of the assembled norm) and exactly (`frame_kernel/src/exact_sum.rs`). They are published as evidence. A perturbation of the formed K or M away from its re-formation shows in check 2's residual, which is evaluated on the re-formed pencil.
   - **Seams it needs.**
     - **(i) K's element re-formation.** K-D5 already re-forms element stiffness from authored operands at p128, a 128-bit-significand correctly rounded software float (`intended_stiffness_pass`, `formation_check.rs:703`; `FORMATION_PRECISION = 128`, `:23`; `structural/retained/wide.rs`), inside a private module (`structural.rs:4`). That exceeds the double-double floor. M1b adds a read-only frame_kernel entry that returns those re-formed element matrices for the elements K-D5 forms (straights, realized bends, connectors and tie rods), without its D5C1 judgment. This is the only K-D5 code reused, and only where it applies. Springs and the rigid-family scaling are re-formed from their authored values in `modal_eigen`.
     - **(ii) M's re-formation.** It is new code in `modal_eigen`, in double-double built from binary64 error-free transformations (TwoSum, TwoProduct), deterministic on IEEE binary64 with round-to-nearest. The arc's trigonometric terms are evaluated to the same ≈106-bit target. If frame_kernel's wide arithmetic is exposed, M1b may use it instead; its design records which.
     - A family that the re-formation does not cover refuses the set (`MODAL_REFORMATION_UNAVAILABLE`, naming the element). The check fails closed.
     - frame_kernel is priced, so (i) re-registers D3 in its change (§3.6).
2. **Residual and error estimate per mode.** For a computed pair (λ, φ), λ in binary64 and φ the binary64 vector:
   - **Residual on the re-formed pencil.** r̂ = K̂φ − λM̂φ is evaluated element by element from the re-formed element matrices, with double-double products accumulated exactly. Its scaled size is the normwise backward error of the pair for the re-formed pencil:
     η = ‖D r̂‖₂ / ((‖K̃̂‖ + λ‖M̃̂‖)·‖φ̃‖₂).
     The eigensolver's own error and the formation difference both show in η.
   - **Threshold.** η is compared with the stated residual threshold τ_r, which lies above 10⁻¹². A mode with η > τ_r is **demoted**, not refused: it is published Sensitive with its η, as the static path demotes. τ_r is measured under DEC-026 (§9, O-6).
   - **Error estimate.** The published relative eigenvalue error estimate is e_λ = η·κ_λ, with κ_λ = (‖K̃̂‖ + λ‖M̃̂‖)·‖φ̃‖₂² / (λ·φ̃ᵀM̃̂φ̃), the first-order condition number of a simple eigenvalue of a symmetric-definite pencil. **Premise:** first-order perturbation theory, with the eigenvalue isolated from the rest of the spectrum. For a cluster, e_λ is stated for the cluster as a whole (the largest over its members), not for each member. e_λ is an estimate under that premise, not a bound; it can understate the error where the premise fails, for example where a computed eigenvalue has a near neighbour that was not computed.
   - The check needs no M⁻¹ and no K⁻¹ norm, so it covers a singular lumped M.
3. **M-orthogonality.** max over i ≠ j of |φᵢᵀM̂φⱼ| / √(m_i m_j), over the published modes, evaluated in double-double on the re-formed M, is within a stated tolerance. It is invariant to the shape normalization (§1.5) and is evaluated on the published vectors. A breach demotes the result to Sensitive.
4. **Sturm-count agreement (completeness).** By Sylvester's law of inertia, in exact arithmetic the negative-pivot count (`nonpositive_pivot_count`) of K − σM is the number of eigenvalues below σ. The binary64 count is relied on only under the separation premise below.
   - **Where shifts go.** Counts are taken **only at shifts placed in gaps between computed clusters**: the final shift in the gap between the completed last reported cluster and the next computed cluster (§1.4), and, on the sparse path, the slicing shifts (§3.3). If the reported set is the whole finite spectrum, the final shift is placed a stated relative offset above its largest eigenvalue, and the separation applies on the lower side only. If the reported set is empty (a cut-off below the lowest frequency), the count is taken at a shift in the gap between f_c and the lowest computed cluster and must be 0; the result states that no mode lies at or below f_c under the premise. The cut-off's sizing count (§1.4) is not a completeness count.
   - **Estimated backward error of a count.** For a factorization of K̃ − σM̃ (§3.3's scaling), e_b(σ) = (‖K̃‖ + |σ|·‖M̃‖)·u·g(σ), with g(σ) the pivot-growth factor read through the observation accessor (§3.3). The proposed form is g = max(1, max_ij|L_ij|)²·max_j|d_j| / max_j|ã_jj|, with ã the scaled shifted matrix. M1b states the formula in the code and records it here (§9, O-6). The growth monitor is an input to this estimate, not a certificate.
   - **Growth limit.** A factorization whose g exceeds the stated limit g_max is rejected, as is one that the 1e-12 pivot guard refuses.
   - **The separation premise.** For a neighbouring computed eigenvalue λ_j with scaled vector φ̃_j, the separation of σ is
     s_j(σ) = (|λ_j − σ| − e_λ,j·λ_j)·(φ̃_jᵀM̃φ̃_j) / ‖φ̃_j‖₂²,
     the first-order size of a symmetric perturbation of K̃ − σM̃ that would move λ_j to σ, after allowing for λ_j's own error estimate (check 2). This is the gap's half-width, measured on the scaled pencil. The count at σ is relied on only when s_j(σ) ≥ κ_s·e_b(σ) for the nearest member of each neighbouring cluster, with κ_s a stated multiple (an algorithm parameter fixed in M1b, §9, O-6). The result discloses the premise, the shift, its separation ratio min_j s_j/e_b and g.
   - **Placement.** The first candidate is σ = √(λ_lo·λ_hi), the geometric mean of the gap's ends. If it is rejected (growth or pivot guard) or fails the premise, a stated finite sequence of other positions in the same gap is tried, at most s_max candidates in all.
   - **Refusal, never publication.** If no candidate satisfies the separation premise, the affected set is refused (`MODAL_STURM_SHIFT_UNSEPARATED`, naming the modes on each side of the gap). If every separated candidate exceeds the growth limit, it is refused (`MODAL_STURM_GROWTH_LIMIT`, naming the gap and the largest g). A shift is never moved out of its gap to obtain a count.
   - **Rule.** The count at the final shift must equal the number of modes reported, cluster extensions included. A missed eigenvalue below σ makes the count larger, which is how a missed direction is detected (§3.3). A mismatch refuses the set (`MODAL_STURM_COUNT_MISMATCH`), after the sparse path's restarts (§3.3). A count disagreement is never published.
   - **What it does not establish.** Agreement shows that the count and the reported set agree under the premise. It does not prove that no eigenvalue is missing: the premise is a first-order estimate, and g is an estimate of growth, not a bound on the factorization's backward error.
5. **Clusters.** Adjacent computed eigenvalues λ_i ≤ λ_{i+1} share a cluster id when λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}. Membership is chained, so a cluster is a maximal run. That is the one cluster definition (§1.4; PRD §12.9; ScopeOfWork REQ-002). A straight circular cantilever has exact pairs.
   - δ_c is an algorithm parameter, fixed in M1b and recorded here. It is not an engineering default.
   - Two clusters whose error estimates overlap stay separate clusters by this definition. A boundary between them is a gap in which the separation premise of check 4, which subtracts each neighbour's e_λ, decides whether a shift can be placed; if it cannot, the set is refused.
   - The reported set is extended to complete a cluster at the boundary (§1.4).
   - Readers and references compare clusters as subspaces (principal angles), not as vectors. [design] VAL-REF-02 §10.4: "Cluster repeated or nearly repeated eigenvalues and compare the corresponding subspaces" (`docs/references/validation-programme/piping_solver_benchmark_research_and_agent_plan.md` at `d90e4ee0e5`, lines 669–682; catalogued at `validation/qualification/capability_inventory.json:70-82`).
6. **Weight-basis identity** (§2.5).

Standing:
- **Checks passed** when 1–6 hold within their tolerances and premises.
- **Sensitive** (published, demoted, as on the static path) for a mode whose η exceeds τ_r, or on an orthogonality breach, when nothing refuses the set.
- **Refused** on a Sturm mismatch, a final or slicing shift that cannot be placed under the separation premise or within the growth limit, an uncovered re-formation, a normalization the two cases do not resolve (§1.5), a weight-basis mismatch, an acceleration mismatch or a component mass out of range (§2.3), or a non-SPD K.
- The tolerances and thresholds are measured and recorded under DEC-026, tighten-only.
- **Limitation text.** The profile's fixed limitations (§5.2) keep "numerical integrity does not establish physical correctness" and state what the checks do and do not establish: the residual is evaluated against a double-double re-formation of the stiffness and mass; the eigenvalue error is a first-order estimate; the Sturm count is relied on only at shifts separated from the computed eigenvalues by a stated multiple of the estimated backward error of their factorization; and none of these is a mathematical bound on the eigenvalues, or a bound on modelling, discretization or input error.
- Product texts use the claims registry's numerical-health vocabulary and avoid "verified", "certified" and "guaranteed" (`docs/claims_registry.md`); "Sturm count" is the user-facing name of check 4.

### 3.5 Where the code goes

**[decided]**
- A new crate, `core/solver/modal_eigen`: dense reduction, tridiagonal QL, block Lanczos, the Sturm wrapper over `sparse_direct` with its shift placement and growth limit, the double-double re-formation and formation difference, residual estimates and participation. Std, plus path dependencies on `sparse_direct` and `frame_kernel` (`exact_sum` and the re-formation seam).
- **Seams in other crates**, each read-only and each owned by DEL-04-01: the `ProfileFactorization` observation accessor (§3.3, M1b) and the frame_kernel element re-formation entry (§3.4 check 1 (i), M1b). The unit-catalog families of §1.5 are DEL-02-02's (before M1b).
- A new module, `core/product_physics/src/modal.rs`: the request DTO, refusals, mass assembly, the weight-basis check and result rows.
- A public entry, `run_modal_preview_with_mode`, beside `run_linear_static_preview_with_mode` (PP `lib.rs:2580`).
- Mass is formed in a **separate pass over `BuiltModel`**, never inside `build_model_for_members` or `build_member`. [design] `build_member` is a 50,544 B frame in the measured AC-004 peak stack chain (DEL-04-08 `Work/stack-headroom-repair.md:30-45`), so growing it would risk the R/64 headroom witnesses.

### 3.6 Retained-memory pricing (D3) and AC-004

- **[design]** AC-004 and REQ-004 govern **retained-precision invocations through the Direct entry** only (DEL-04-08 `ScopeOfWork.md`, REQ-004). "Any change to a priced source or to a reviewed input requires a new actual-source D3 audit".
- **[decided]** F1 is a **separate route that never enters D1 or the retained Direct entry**, so it needs no retained admission and no new D1 premise.
- **It still triggers D3**, because it touches priced sources and one reviewed input:
  - PP `lib.rs` (`mod modal;` and the entry). [design] W-16's one-line `mod imposed_motion;` moved every snapshot-keyed line and its full-file pins (`retained-pricing-rules.md` §8.23).
  - `self_weight.rs`, to expose `stable_mass_per_length` and `component_weights` as `pub(crate)`. Its full file is pinned in `b7a_formation_model.py` (§8.23).
  - **`core/product_physics/Cargo.lock`**, if product_physics depends on the new crate. [code] The lock is reviewed input #1 (`build_identity.rs:158-159`), so a new dependency makes the registered profile `Stale` until re-registered.
  - `core/reporting/result_export/src/semantic_contract.rs`, if the modal table's constants go there (literal-only, §8.23).
  - **`core/solver/sparse_direct/src/lib.rs`**, for the observation accessor (§3.3), in M1b. Its crate directory is priced and the file is named in the pricing rules (`tools/retained_pricing/base/crate_dirs.txt`; `rules/o10_rules.py:3020`). The accessor is priced because `sparse_direct` is.
  - **`core/solver/frame_kernel/src`**, for the re-formation entry (§3.4, check 1 (i)), and the wide arithmetic if it is exposed, in M1b.
  - **`core/units/src`**, for the unit-catalog families (§1.5), in DEL-02-02's change.
- **Not touched:** the preview-physics-2 table (reviewed input #5, `build_identity.rs:166`); F1 adds its own table, which is not a reviewed input. `results.v0.3.schema.yaml` is not a reviewed input (`build_identity.rs:158-181`).
- **Expectation, to be measured:** modal code is unreached from the retained Direct entry, so the priced tree should move only by line shifts and pins, coefficient-identical as W-16's was. The AC-004 stack witnesses are re-run if the D3 moves them.
- **[decided]** The first D3 is done in the first PR that touches priced files (M1a). The units extension, the frame_kernel seam and the sparse_direct accessor (both M1b) each re-register D3 in the change that makes them, or join a neighbouring slice's D3 when they merge together. M2 is expected to touch no priced file beyond what M1b has registered; if it does, it re-registers D3 in that change. Other slices stay out of priced files where possible. D3 runs join the queue on Agent 0's go (§6).

---

## 4. Validation references

### 4.1 The independent-reference rule

- **[design]** References are "derived and frozen by a TASK that neither reads nor writes product code, then refuted by a second TASK, before any product output is compared with it" (`pf-timoshenko.md`, #1663). Existing example: `validation/references/w15_component_weight/` ("The generator imports, reads and runs no product code. All model data are invented.").
- **[decided]** For F1, `validation/references/f1_modal/`:
  - a Python generator, standard library only;
  - exact rationals of the binary64 inputs (`fractions`), and transcendental roots by bisection or Newton in `decimal` at about 50 digits;
  - its own element K and M from the published formulas, its own generalized eigen-solve and its own Sturm counts;
  - frozen JSON and `SHA256SUMS`, then refutation by a second TASK.
- **Reference classes:**
  - **(a) Formulation-matched discrete references**: the same discretization as the product, compared at the analytic class tolerance, seeded at 1e-9 relative for frequencies (DEC-026).
  - **(b) Continuum references**: compared at a stated convergence tolerance across mesh refinement. They show the element converges at the expected rate; they do not check an individual mode.
  - **(c) Published piping-system benchmarks** (§4.4): comparison evidence at a stated tolerance, not exact answers.
  - Mode shapes are compared by MAC or subspace angle at common nodes (VAL-REF-02 §10.4).

### 4.2 Closed-form beam frequencies (class b, and class a with the matched discretization)

The generator re-derives every formula and root below; they are stated from memory here.
- **Euler–Bernoulli, ω_n = (β_nL)²·√(EI/(m̄L⁴)):**
  - cantilever, cos βL·cosh βL = −1: β_nL = 1.875104069, 4.694091133, 7.854757438, 10.995540735 ([design] the first also at VAL-REF-02 line 290, `AN-MODAL-02`);
  - simply supported: nπ;
  - fixed-fixed, cos βL·cosh βL = 1: 4.730040745, 7.853204624, 10.995607838.
- **Axial and torsional rods:** cantilever axial ω_n = ((2n − 1)π/2)·√(EA/(m̄L²)); torsional ω_n = ((2n − 1)π/2)·√(GJ/(j̄L²)). These isolate D-e: contents are in m̄, not in j̄.
- **Rotary inertia (Rayleigh) and Timoshenko, simply supported**, α = nπ/L: (κGAα² − m̄ω²)(EIα² + κGA − īω²) − (κGAα)² = 0. The Rayleigh limit (κG → ∞) is ω² = EIα⁴/(m̄ + īα²). The Timoshenko cantilever uses its transcendental frequency equation (cite Han, Benaroya & Wei, *J. Sound Vib.* 225(5), 1999; derive, do not copy tables). Rayleigh cases are in M1; Timoshenko cases belong to M4.
- **Lumped-mass controls (class a):** a single DOF, f = √(k/m)/2π (VAL-REF-02 line 289, `AN-MODAL-01`); a point mass on a massless cantilever, ω = √(3EI/(M_tL³)), on the kernel directly; a fixed-free axial chain of N equal lumped masses, ω_j = 2√(k/m)·sin((2j − 1)π/(2(2N + 1))).
- **Point mass on a cantilever with distributed mass:** 1 + cos βL·cosh βL + (M_t/(m̄L))·βL·(cos βL·sinh βL − sin βL·cosh βL) = 0. In the product the tip mass is a rigid-family component weight with a = the request acceleration, which exercises §2.3. Variants: centre of gravity at the node; offset along and across the axis (rotary terms m·[r]×ᵀ[r]×); and the fall-back case, where an ineligible component keeps the pipe mass and emits the warning.

### 4.3 Rings and arcs

- **Class a.** An L-line, a Z-bend and a U-loop with realized arcs, with flexibility factors of 1 and other than 1, against the generator's own Castigliano arc stiffness and half-arc rigid lumps.
- **Class b.** A free complete ring of n_b bend macro-elements: in-plane inextensional ω_n² = (EI/(m̄R⁴))·n²(n² − 1)²/(n² + 1) (Hoppe, 1871); out-of-plane ω_n² = (EI/(m̄R⁴))·n²(n² − 1)²/(n² + EI/GJ) (Love, *Treatise on the Mathematical Theory of Elasticity*). The ring is free, so this is a **kernel-level test** with a negative shift (K − σM SPD for σ < 0), not a product run. It shows lumped arcs converging as n_b grows.

### 4.4 Piping-system benchmarks and their provenance

- **[design]** The repository catalogues them as `NRC-DYNAMIC`: "NUREG/CR-1677 volumes 1/2, CR-6414, CR-6645; ANSYS descendants", readiness `catalogued_unadmitted` (`validation/qualification/capability_inventory.json:2523-2535`). NUREG/CR-1677 Vol. 1 (uniform support motion, with reference frequencies and participation factors) and Vol. 2 (independent support motion) are BNL problem sets for the NRC; NUREG/CR-6414 has three AP600 problems; NUREG/CR-6645 covers modal-response combination (an F2 concern).
- **[design]** NUREG/CR reports are prepared by contractors (for these, BNL) for the NRC. NRC sponsorship does not by itself make a contractor-authored report a US-government work. `docs/IP_AND_DATA_BOUNDARY.md` §1 requires human or legal review "when redistribution rights or protected-content status are uncertain", and §4 holds a record with `source_license = TBD` or `redistribution_status = unknown` until a maintainer records a disposition.
- **[decided]** (D-i) Agent 0, 2026-10-27, revising the earlier premise:
  > "Confirmed: the per-report condition. My premise was too broad. NRC sponsorship doesn't by itself make a contractor-authored NUREG/CR a US-government work. So:
  > - Transcribe from a report only with that report's own redistribution evidence (its rights statement and NRC distribution terms), recorded per fixture.
  > - Hold any report with uncertain rights for owner review, per IP_AND_DATA_BOUNDARY.
  > - Prefer sources whose rights are clear: original closed-form benchmarks we derive ourselves, and textbook formulas restated in our own words with citation.
  > - FR-SOL-016 stays partial until at least one published piping benchmark is compared with that evidence. Agreed."
- **Transcription is conditional on report-specific evidence.** Nothing is transcribed from a report until its own redistribution evidence is in hand: the report's rights statement (its front-matter notice, including any copyright or contractor-rights notice) and the NRC's distribution terms for it. A report whose rights are uncertain, or whose notice reserves rights, is **held for the owner's review** under `docs/IP_AND_DATA_BOUNDARY.md`, and nothing is transcribed from it in the meantime. Content inside a report that is marked as copyrighted or adapted from another source, such as figures adapted from journals, is never transcribed.
- **Per-fixture provenance record.** Each transcribed problem is committed with a `PROVENANCE.json` beside its fixture under `validation/references/f1_modal/published/<source>/<problem>/`, carrying the `docs/IP_AND_DATA_BOUNDARY.md` §4 fields:

  | Field | Content |
  |---|---|
  | `source_name` | Report number, volume, title and the problem's own identifier |
  | `source_location` | The report's NRC accession number (ADAMS) or NTIS/OSTI identifier, and the pages and tables transcribed |
  | `source_license` | The redistribution basis this report's own evidence supports, with its rights statement quoted from the front matter and the NRC distribution terms cited. `TBD` while the evidence is incomplete, in which case nothing is committed |
  | `contributor` | The transcribing role (TASK) and its PR; no personal identity |
  | `contributor_certification` | That only problem inputs and reference values were transcribed, and that no figure, table or text marked as copyrighted or adapted from another source was copied |
  | `redistribution_status` | `public_permissive` only when the report's own evidence supports it; otherwise the report is held and no fixture exists |
  | `review_status` | Agent verification, never acceptance (D-u): an independent Type 2 reviewer verifies the transcription against the source and records it as agent verification, naming the reviewer role and the source. If the rights are unclear, the fixture is not committed and the report holds for the owner |

  The record also lists what was **excluded** as copyrighted or adapted, with its location in the report, so a reviewer can see the line drawn.
- **Use.** NUREG/CR frequencies are numerical results of other programs, not exact answers ("NRC and ANSYS outputs retain shared lineage", `capability_inventory.json:2530-2533`). They are class-c comparison evidence at a stated tolerance. A report's participation factors may use another shape normalization, so they are compared through normalization-invariant quantities: effective masses, or Γ·φ.
- **Other sources:**
  - **Preferred, rights clear:** closed-form benchmarks derived in this repository (§4.2, §4.3), and textbook formulas restated in our own words with citation.
  - **Code_Aster** SDLX02 (Hovgaard modal) and SDLL14 (elbow frequencies) (VAL-REF-02 lines 94–95): GPL-3.0-or-later code; usable for independently implemented fixtures, not by vendoring, with the redistribution evidence for the transcribed data recorded per fixture as above.
  - **ASME PVP benchmark papers and NAFEMS natural-frequency benchmarks:** licensed or paywalled. Cite only; copy no data.
  - **CAEPIPE:** no outputs are copied. Any comparison is the owner's private exchange.
- **[decided] The published-benchmark obligation** (D-q). PRD §12.9 requires comparison with published piping benchmarks.
  - **Criterion.** Before FR-SOL-016 is reported complete, at least one published piping benchmark with recorded redistribution evidence is compared with the product, and the comparison is recorded as **comparison evidence, not exact verification**.
  - **Prerequisite and timing.** The per-report evidence above is the prerequisite. The comparison comes after M1b (the dense path).
  - **Until then**, FR-SOL-016 stays partial (`docs/COMPLETENESS_MAP.md`). M1–M3 rely on classes a and b. The first class-c target is one NUREG/CR-1677 Vol. 1 problem, whose modal part needs only frequencies and participation, if its report's evidence supports redistribution. Otherwise it is another published piping benchmark whose evidence does.
  - **Status of the first candidate (2026-10-31).** NUREG/CR-1677 Vol. 1 (BNL-NUREG-51267, August 1980; OSTI 6403318), Problem 1, is recorded with `source_license = TBD` and `redistribution_status = unknown` and held for the owner's review (O-3). Its notices mark distribution as unlimited and carry no copyright or contractor-rights notice, but they grant no licence, and the report is mostly contractor-authored. Its Problem 2 is adapted from the literature and is excluded regardless. Vol. 2, NUREG/CR-6414 and Code_Aster SDLX02 are held or unsuitable on their own evidence. Nothing is transcribed until the owner rules. A search for a problem in a report authored by US government employees runs in parallel.

---

## 5. Desktop UI, result schema and readers

### 5.1 Where results appear (M3)

- **[code]** The viewport's deformed shape is a display transform of result rows. `buildDeformationOverlay` reads `displacement_magnitude` and `global_nodal_displacement_{x,y,z}` rows, normalizes by the maximum to a display offset of 0.65 and labels the boundary `scale=normalized_display_offset_not_physical_length` (`apps/desktop/src/features/viewport/deformationOverlay.ts:20-116`). It is consumed in `PipeViewport.tsx:618` (three.js, `:7`). No animation loop exists in `PipeViewport.tsx` today.
- **[decided]**
  - **Request entry.** The modal request form takes the extraction bound with no pre-set value in a new project. The desktop remembers the last value used per project and pre-fills it; the user sees and may change it before solving.
  - **Modes table** in `features/results/` (a new `ModalResultsPanel.tsx` beside `ResultsPanel.tsx`). Columns: mode; f (Hz), with T (s) and ω (rad/s) in project display units; effective-mass fractions X/Y/Z and cumulative; cluster; standing (checks passed or sensitive, with the residual backward error and error estimate).
  - **Header lines:** the Sturm count ("N modes up to the checked shift; N reported"), with its separation premise, the extraction bound, which bound stopped extraction and any cluster extension, the modulus basis, the weight-to-mass acceleration, total and free-DOF mass, and disclosures (massless connectors and tie rods, constant-effort supports, mass at restrained DOFs, contents excluded from rotary inertia).
  - **Mode-shape view.** A new `features/viewport/modeShapeOverlay.ts`, reusing the 0.65 display offset and the `scale=…not_physical_length` boundary wording. Positions are x₀ + sin(phase)·offset·φ̂, with φ̂ the published shape (case (a), unit maximum translation, §1.5); a case (b) mode (unit maximum rotation, no translation) is shown by its nodal rotations, labelled as such. It is animated by a viewport-owned loop that runs only while a mode is shown. Under `prefers-reduced-motion`, or by user choice, it shows the static extreme shape with a phase step control (DEL-07-06). Display-only Hermitian interpolation along straight members from nodal rotations is optional (Euler–Bernoulli only).
  - **Report section** "Modal analysis" (`core/reporting/report_sections`; the desktop's `features/report/`): request and basis, mass summary, modes table, integrity evidence, refusals and disclosures, limitations. No claim language beyond the existing fences.
  - **Survey A6** (key-results panel) gains frequencies once F1 exists, in A6's own slice.

### 5.2 Result schema and semantic contract (M1b)

- **[code]** Result profiles today: `formulation_basis.profile_id` ∈ {`product_preview_mechanics_v1`, `exact_pressure_v3`, `product_preview_retained_w1a_v2`, `exact_straight_retained_w1a_v2`} (`schemas/results.v0.3.schema.yaml:1035-1041`). Semantic tables are identified as at `core/reporting/result_export/src/semantic_contract.rs:126-128` (`…/preview-physics-2`).
- **[decided] Additions:**
  - **Profile:** `product_preview_modal_v1`, with two fixed limitations.
    - Limitation 0: "Small-displacement, linear-elastic Euler-Bernoulli frame modal preview about the unloaded configuration, with any nonlinear supports at the stated state of a named solved case, with consistent mass on straight members and lumped rigid-body mass on realized arcs and component bodies; numerical integrity does not establish physical correctness."
    - Limitation 1: "The integrity checks are numerical estimates under stated premises, not mathematical bounds: each mode's residual is evaluated against a double-double re-formation of the stiffness and mass, its eigenvalue error is a first-order estimate, and the Sturm count is relied on only at shifts separated from the computed eigenvalues by a stated multiple of the estimated backward error of their factorization. They do not bound modelling, discretization or input error."
  - **Table:** `openpipestress.result_semantics/0.3.0/modal-1` (`fixtures/results/semantic_contract_v0_3_modal_1.json`, new, not a reviewed input). Row kinds:
    - `modal_frequency`, `modal_circular_frequency`, `modal_period`;
    - `mode_shape_{ux,uy,uz,rx,ry,rz}` (normalized per §1.5, each mode carrying its normalization case);
    - `modal_generalized_mass`;
    - `modal_participation_factor_{x,y,z}`, `modal_effective_mass_{x,y,z}`, `modal_effective_mass_fraction_{x,y,z}`, `modal_cumulative_effective_mass_fraction_{x,y,z}`;
    - `modal_mass_total_by_category`, `modal_mass_free_dof_{x,y,z}`;
    - `modal_residual_backward_error`, `modal_eigenvalue_error_estimate`, `modal_formation_difference`, `modal_orthogonality_deviation`, `modal_sturm_count`, `modal_sturm_shift` (with its separation ratio and growth factor).
    - Every row's unit is a catalog unit with integer exponents (§1.5).
    - Each row carries `basis_ref {ref_type: "mode", ref_id: "mode:<i>"}` and, where one exists, `cluster_ref`.
  - **Run kind:** a `modal` analysis run in `schemas/analysis_run.v0.3.schema.json`, so stored results keep the request (extraction bound, modulus basis, a).
  - **Readers:** a Rust `core/reporting/result_export/src/modal_evidence.rs` and a Python mirror under `tests/verifiers/`, sharing no code. They check the profile and limitation against the request; re-derive the free-DOF and total masses from the model (§2) and check Σ_i m_eff,ik ≤ T_k = R_kᵀM R_k; check each mode's normalization case and sign (its normalizing component is +1; in case (b) every translation is at most τ_t·ℓ and every translational effective-mass fraction at most τ_t), and that Γ_ik² m_i = m_eff,ik; and check the counts, ordering, cluster structure and any cluster extension. They do not re-run the eigen-solve (the DEF-O stance, DEL-04-08 REQ-005).
  - **Desktop and headless:** a Tauri command beside the static one (`apps/desktop/src-tauri/src/lib.rs:44`, `:1904`), TS types, and a headless runner command (`core/runner/headless`).

---

## 6. Sequence and size

Agent 0's sequence, 2026-10-27:
1. **M1a, mass:** `modal.rs` request DTO and refusals (§1.2, §1.4); mass assembly (§2.2–§2.4); the W/a basis, its range premise and its refusals (§2.3); the weight-basis check and the measurement that pins its tolerance (§2.5); mass summary rows; the discretization warning and the derivation of c (§2.6); the `pub(crate)` seams in `self_weight.rs`. References for the mass and resultants in the `w15_component_weight` generator style. **The one D3** (PP `lib.rs`, `self_weight.rs`).
2. **M1b, the dense path, with every check of §3.4:** after DEL-02-02's unit-catalog families (§1.5); `core/solver/modal_eigen` (Cholesky reduction, tridiagonal QL, the Jacobi control in tests, the double-double re-formation with frame_kernel's re-formation seam, residual estimates, M-orthogonality, the reported-set and cluster rules, both normalization cases, participation); **the count infrastructure the dense results need**: the scaled inertia factorization through `sparse_direct`, the `sparse_direct` observation accessor, the shift placement under the separation premise and the growth limit, with the inertia-risk tests for the dense path's counts (§3.2 item 7, §3.3, §3.4 check 4); the product entry; the dense memory and work guards; the modal-1 table, profile and run kind; the Rust and Python readers; the headless command. References for §4.2 and §4.3. D3 for the PP `Cargo.lock` (and `semantic_contract.rs` if used), the frame_kernel seam and the `sparse_direct` accessor, joined with M1a's if they merge together.
3. **M2, the sparse path:** shift-invert block Lanczos with full reorthogonalization, locking, breakdown handling and deterministic restarts; spectrum slicing, with its shifts under M1b's separation premise; the sparse guard with its peak count; the inertia-risk tests extended to slicing shifts and to dense-against-sparse agreement, since "the unpivoted-LDLᵀ inertia risk" is "a first-class test target" (§3.3); scale benchmarks timed under `t3_exclusive.sh`. It uses M1b's count infrastructure unchanged, and no D3 is expected (§3.6).
4. **M3, UI:** the Tauri command and types, the request form with the per-project remembered bound, the modes table, the mode-shape overlay and animation with reduced motion, the report section (Rust and TS), Vitest and a native smoke step.

**Order (Agent 0, 2026-10-31).** M1a and M1b cover both routes, and M1b includes the linearization of nonlinear supports (D-s). M3 comes before M2: the sparse path follows the views and precedes response-spectrum results, reordered if the dense guard bites earlier.

**Start and queue.** PF Timoshenko (#1663) has landed. D3 runs join the queue on Agent 0's go.

**Size (estimates, not measurements):**

| Slice | Product and test lines | Reference lines | PRs | D3 |
|---|---|---|---|---|
| M1a | 1,000–1,400 | 400–600 | 1 | yes, once |
| M1b | 2,600–3,600 | 800–1,200 | 2 | yes (lock, frame_kernel seam, `sparse_direct` accessor), joinable with M1a's |
| M2 | 1,000–1,500 | reuse M1, plus generated large models checked against dense | 1–2 | none expected |
| M3 | 1,700–2,300 | – | 2 | none |
| **F1** | **≈ 6,300–8,800** | **≈ 1,200–1,800** | **6–7** | one or two, plus the units change's own |

**Later slices, outside F1's core:**
- **M4.** Timoshenko consistent mass (interdependent interpolation, e.g. Friedman & Kosmatka, *Computers & Structures* 47(3), 1993; cited, derived). It requires the PF Timoshenko unit and model document 0.8.0 (#1649).
- **M5.** Nonlinear supports linearized at a named case's converged state.
- **M6.** The mass-point helper: a model operation that subdivides spans.
- A concentrated-mass field (W-17, a new document version).

**Dependencies on other lanes:**
- **W-17 (model document):** none for M1–M3, since every parameter is on the request. M4 needs 0.8.0's `shear_model` (#1649).
- **PF Timoshenko** (#1663): M4 only, and the start condition above.
- **Survey A9** (weight and centre-of-gravity summary) falls out of M1a's mass assembly. A6 consumes M3.
- **DEL-02-02 (units):** the unit-catalog families of §1.5, before M1b.
- **DEL-04-01 (kernel):** the frame_kernel re-formation entry and the `sparse_direct` observation accessor (both M1b), both read-only seams that F1's slices make in DEL-04-01's code.
- **The deliverable graph.** `python3 -m tools.deliverables … check` reports one cycle, a strongly connected group. DEL-08-01 needs DEL-04-09 (the report section), DEL-04-09 needs DEL-04-07 and DEL-04-08, which are in the group, and it needs DEL-04-05, which needs DEL-04-07. These declarations bring DEL-04-09 and DEL-04-05 into the group, which grows from 9 to 11 members (DEL-04-05, DEL-04-07, DEL-04-08, DEL-04-09, DEL-05-02, DEL-08-01, DEL-08-04, DEL-08-05, DEL-09-05, DEL-10-04, DEL-10-05). The cycle is a fact of the declarations, not an ordering error; the slices' own sequence (above) orders the work.

---

## 7. Settled decisions

| ID | Decision | Settled as | Reason |
|---|---|---|---|
| **D-a** | Re-enter FR-024 for modal analysis and create the deliverable | **Taken:** SCA-013, FR-SOL-016, SOW-081, DEL-04-09. F2 and F3 stay deferred | Agent 0 under the owner's delegation (SCA-013) |
| **D-b** | Lumped or consistent mass | Consistent on straights; rigid-body lumps on arcs and component bodies; lumped diagonal only as a reference control | Agent 0, 2026-10-27 ("Agreed"); O(h⁴) convergence from above and an element-wise weight identity |
| **D-c** | Mode count, cut-off and any default | Required request input: a mode count, a cut-off or both; the lowest modes up to whichever bound stops first, the cut-off inclusive; a cluster (one definition: adjacent computed eigenvalues within the relative tolerance δ_c, chained, §3.4 check 5) at the boundary completed and disclosed by count, never published partially; completeness counted at a separate final shift in the gap above the completed cluster, under the separation premise, on the interval actually reported; the cut-off's offset shift used only to size the sparse extraction (§1.4); no default; the desktop remembers the last value per project. **Changed from the proposal:** no fixed source cap on the mode count. **Changed after Codex round 2:** the final shift replaces the cut-off shift as the completeness count | Agent 0, 2026-10-27: "No code-derived cut-off value ships." A fixed count cap would be a code-derived bound on the extraction; the resource guards (§3.2, §3.3) refuse an oversized request with their formula instead |
| **D-d** | Output units and normalization | f (Hz), ω (rad/s), T (s); exactly two normalization cases (§1.5): (a) a mode with nonzero translation, unit maximum translation (translations dimensionless, rotations in rad/m), m_i in kg, Γ_ik = φᵢᵀM r_k / m_i dimensionless, m_eff,ik = (φᵢᵀM r_k)²/m_i in kg; (b) a mode with no translation (t ≤ τ_t·ℓ·θ, τ_t = 10⁻⁶, and every translational effective-mass fraction ≤ τ_t), unit maximum rotation (rad/rad), the computed translations unaltered in m/rad (at most τ_t·ℓ), m_i in kg·m², and Γ in rad/m and effective translational masses in kg computed from the published vector (each fraction at most τ_t); a mode meeting only the first condition refused (`MODAL_NORMALIZATION_UNRESOLVED`); cumulative fractions dimensionless; display in project units. **Changed after Codex round 1:** mass-normalized shapes were withdrawn. **Changed after Codex round 2:** the two cases and the threshold made exact | Every quantity keeps integer unit exponents, which the unit catalog requires (`core/units/src/lib.rs:224`); Γ, m_i and m_eff keep what F2 needs |
| **D-e** | Contents in axial and torsional inertia | In all three translations; excluded from torsional and rotary inertia, disclosed | [code] Self-weight applies m̄, contents included, along any global axis (§2.1), so the identity holds in every direction only if contents are in every translation |
| **D-f** | Component mass m = W/a | `weight_to_mass_acceleration`, required when a component weight enters, never defaulted; equal bit for bit to any current applied component-weight case's acceleration magnitude, or refused (§2.3) | [code] W is an authored force that the acceleration's magnitude does not scale, and the model holds no gravity; only a request input gives a consistent a without a default. **Changed from the proposal:** the agreement with applied cases is a refusal, not a disclosure, because a different a would give masses that disagree with the model's own static weight |
| **D-g** | Refusals by name | Nonlinear supports, gaps, friction, the exact route, load/reference-state documents and (until M4) Timoshenko straights refused by name; constant-effort supports, connectors, tie rods and W-16 imposed displacement admitted with disclosures | Linearizing silently, or solving about an unstated state, would be an unstated engineering choice |
| **D-h** | Parameters on the request or in `model_settings` | On the request, captured in the analysis-run record | No W-17 version, and F1 stays uncoupled from the document lane |
| **D-i** | Transcription from published benchmarks | Only with the report's own redistribution evidence (its rights statement and NRC distribution terms), recorded per fixture; a report with uncertain rights is held for the owner's review and nothing is transcribed from it; copyrighted or adapted content excluded; sources with clear rights preferred (§4.4) | Agent 0, 2026-10-27, revising the earlier premise that NUREG/CR reports are US-government works (quoted in §4.4) |
| **D-j** | Participation and effective mass in M1 | Yes, translational X/Y/Z | Without them there is no mass-coverage evidence, and a close frequency does not qualify a wrong mass model (VAL-REF-02 line 682) |
| **D-k** | Dense modal guards | Memory (live n² buffers × 8 B against the provisional 6 GiB ceiling) and work (9n³ + s_max·n³/3, including the candidate Sturm factorizations, against a budget measured in M1b) | A memory-only guard would admit models that run for an hour on the dense path |
| **D-l** | Discretization factor c | Derived by benchmark in M1a; a warning, not a refusal, with c and its derivation in the warning text | Coarse models otherwise give high frequencies with clean residuals (R3) |
| **D-m** | Sparse eigensolver | Shift-invert block Lanczos, b₀ = 2, deterministic start block, locking and deflation, breakdown replacement, deterministic restart with a larger block and a new seed on a Sturm miss (§3.3) | Scalar Lanczos spans one direction per repeated eigenspace; circular sections give paired modes. Codex round 1 |
| **D-n** | What the integrity checks establish | Numerical-integrity checks with stated premises, not certified enclosures: per mode, the residual's backward error on the double-double re-formed pencil against a stated threshold, with a first-order error estimate; M-orthogonality; Sturm-count agreement at shifts placed in gaps between computed clusters, relied on only when the separation exceeds κ_s times the estimated backward error (‖K̃‖ + \|σ\|‖M̃‖)·u·g; a shift that cannot be so placed, or whose growth exceeds g_max, refuses the affected set by name (§3.4). No Kato–Temple refinement and no inverse-norm bound; rigorous enclosures are a later option (§9, O-13) | Binary64 inertia counts, a Rayleigh quotient on the formed pencil and one floating-point solve give no enclosure without interval or exact arithmetic; Codex round 2's witnesses (a shift that rounds away on a 32×32 tridiagonal, a re-formed quotient displaced by 0.01 despite isolation, a solve that underestimates xᵀK⁻¹x by 3×). **Changed after Codex round 2** from inertia-certified Kato–Temple gaps |
| **D-o** | Modal formation check | K and M re-formed independently from their leaves in at least double-double precision (≈106-bit significand), element by element; the formation difference measured and published; each mode's residual evaluated on the re-formed pencil, a mode over the threshold demoted; an uncovered family refuses the set; K-D5's p128 element re-formation reused through a read-only seam, its D5C1 criterion not reused (§3.4) | K-D5 judges a static correction, and a modal run has no static response (Codex round 1). Double-double resolves the formation difference at the 10⁻³¹ relative scale for well-scaled entries, ample against residual thresholds above 10⁻¹², so true quad is not required (Codex round 2) |
| **D-p** | Units for modal quantities | Frequency (Hz), angular frequency (rad/s), rotation per length, length per rotation and mass moment of inertia added to the unit catalog by DEL-02-02 before M1b; period uses Time (s) (§1.5) | The catalog has no such families today. Codex round 1 |
| **D-r** | The exact route | Admitted on its own section basis and mass basis (`EXACT_MASS_METHOD`); the weight-basis identity on the route's own generator; pressure ignored with EX's disclosure | Agent 0, 2026-10-31: exit criterion 1 needs modal on EX. EX shares PF's assembly and adds no load-dependent stiffness (§1.1) |
| **D-s** | Nonlinear supports | Linearized at the converged state of a named solved case, a required input with no default: active or sticking restrains the DOF with the static solve's stiffness in that state, inactive or sliding frees it; the case and each state are disclosed; absent, unsolved or unconverged cases refused | Agent 0, 2026-10-31 (formerly M5): criterion 1's reference system has a gap, lift-off and friction. A named state is the user's statement, not a default |
| **D-t** | Snubbers | In the modal basis when the request includes dynamic-only restraints, a required choice when the model has one; otherwise ignored; disclosed either way | Agent 0, 2026-10-31; SCA-014 (snubbers as dynamic-only restraints). A spectrum solve includes them |
| **D-u** | `review_status` of a transcribed published fixture (O-2) | Recorded as agent verification by an independent Type 2 that checked the transcription against the source, naming the reviewer and source; never as accepted; a fixture with unclear rights holds for the owner | Agent 0, 2026-10-31 |
| **D-v** | Participation excitation | Γ and m_eff on p_k = (M R_k)_f, consistent-mass coupling to restrained DOFs included; fractions relative to the total mass T_k | DEL-04-10's first independent review (gpt-6.1-sol, finding 1): with M_ff R_f alone, one fixed axial element gives u/A = m/(3k) instead of m/(2k). The response spectrum uses these factors unchanged |
| **D-q** | Published-benchmark obligation | At least one published piping benchmark with recorded redistribution evidence compared, after M1b, as comparison evidence, before FR-SOL-016 is reported complete; FR-SOL-016 stays partial until then (§4.4) | PRD §12.9 requires the comparison. Codex round 1; Agent 0, 2026-10-27 |

Agent 0 accepted D-c, D-d, D-m, D-n, D-o and D-p as Design decisions on 2026-10-27, subject to the next independent review round. Round 2 (at `d3c54e4f72`) revised D-c, D-d, D-n and D-o, and moved the count infrastructure from M2 into M1b (§6), under the manager's decisions; those revisions go to Agent 0 after the next review.

Items that reach the product or data boundary are marked **for Agent 0** in §9.

---

## 8. Risks

- **R2, inertia from unpivoted LDLᵀ.** `factorize_ldlt` does no pivoting (`sparse_direct/src/lib.rs:377-381`). On an indefinite K − σM, element growth, or a shift within rounding of an eigenvalue, can corrupt the negative-pivot count. The count is relied on only under the separation premise (§3.4, check 4), from M1b on; mitigations and tests: §3.3. A shift that cannot be placed under the premise refuses the set.
- **R3, mass discretization.** Too few nodes on long spans give wrong frequencies with clean residuals: the residuals check the discrete problem, not the physics. The warning (§2.6) and the M6 helper address it, and the limitation text says so.
- **R4, repeated eigenvalues.** Circular sections and symmetric layouts give repeated eigenvalues with non-unique bases. Block Lanczos recovers every direction, and the Sturm count checks it under its separation premise (§3.3, §3.4). Determinism holds (fixed start block and seed sequence, fixed order, one thread), but bases can change under small input changes. Clusters are completed at the boundary, published and compared as subspaces.
- **R5, two mass bases in the product today** (`self_weight.rs:731` against PP `lib.rs:10293`). A finding, recorded on the remaining-items list (§2.1). No action in F1.
- **R6, rigid-family scaling.** s·E up to and beyond the 1e5 precision-warning scale (`rigid_component.rs:17-24`) adds very stiff, high-frequency directions and raises K's condition number. The low modes stay the well-conditioned end under K-based reduction, but residuals and error estimates widen, and so does the backward-error estimate that the separation premise uses. The W-15 precision warning stays, and the references include a rigid component at s = 1e3…1e6.
- **R7, D3 and the reviewed lock.** A new crate dependency changes `Cargo.lock`, a reviewed input, so the profile is `Stale` until re-registered (§3.6).
- **R8, timing claims.** The dense path's cost is cubic and debug builds are much slower. Every timing statement names its build and runs under `t3_exclusive.sh`.
- **R9, stack.** Mass formation stays out of the frames on the W1 peak chain (§3.5). The AC-004 witnesses are re-run if D3 moves anything.
- **R10, public benchmarks' lineage.** NUREG/CR modal results are numerical, from other programs, so a match is comparison evidence, not an exact check.
- **R12, linearized state.** A modal result with nonlinear supports holds only at the named case's support states; a different operating state gives different frequencies. The result names the case and each state, and the limitation text says so.
- **R11, benchmark rights.** A contractor-authored report may carry rights that prevent transcription. Then no published benchmark may be available, and FR-SOL-016 stays partial (§4.4).

---

## 9. Remaining open items

| # | Item | Owner | When |
|---|---|---|---|
| O-1 | Independent Codex review of the W/a basis (§2.3), as Agent 0 directed, and of this design. Round 1 (at `a640e10e50`) and round 2 (at `d3c54e4f72`) are answered in this revision; the next round is pending | Manager, then Agent 0 | Before M1a starts |
| O-2 | **Settled (D-u, Agent 0, 2026-10-31):** an independent Type 2 verifies a transcription against its source and records agent verification, naming reviewer and source; never acceptance. Unclear rights hold for the owner | — | — |
| O-3 | **For the owner (data boundary):** the review of each report held for uncertain rights (§4.4; `docs/IP_AND_DATA_BOUNDARY.md`). Nothing is transcribed from a held report | Owner | At acquisition of each report |
| O-4 | The work budget of the dense guard (§3.2), set by measurement | M1b | M1b |
| O-5 | The discretization factor c (§2.6), derived by benchmark | M1a | M1a |
| O-6 | The integrity tolerances, the residual threshold τ_r and the orthogonality tolerance (§3.4) and the identity tolerance (§2.5): measured, then pinned under DEC-026. The algorithm parameters, recorded with their basis: the confirmation of τ_t (§1.5); δ_c, the growth-factor formula g, the growth limit g_max, the separation multiple κ_s, the candidate-shift sequence and s_max (§3.4, M1b); the cut-off sizing offsets ε_s (§1.4), the block size limits, restart count and breakdown threshold (§3.3, M2) | M1a, M1b, M2 | As each lands |
| O-7 | Where the desktop keeps the remembered extraction bound per project (project store or desktop settings), and whether that needs a store version under the one-version rule. It is persistence, so it takes targeted checks | M3 | M3 design |
| O-8 | Adding `product_preview_modal_v1` to `results.v0.3` and the `modal` run kind to `analysis_run.v0.3`: whether they are additions within the current versions or need new versions under the one-version rule (DEL-00-06's contracts) | M1b, with DEL-00-06 | M1b design |
| O-9 | A concentrated-mass field and component rotary inertia (W-17), entering self-weight and modal together | W-17 lane | After F1's core |
| O-10 | M4 (Timoshenko mass, on either route), M6 (mass-point helper); M5 moved into M1b (D-s) | Later slices | After F1's core |
| O-11 | One mass basis: PF seismic generation and self-weight (finding R5), with the deferred PF seismic-mass alignment to D-5 | Remaining items (`docs/COMPLETENESS_MAP.md` §4.3) | Not in F1 |
| O-12 | The unit-catalog families of §1.5 (frequency, angular frequency, rotation per length, length per rotation, mass moment of inertia), with their D3 re-registration | DEL-02-02 | Before M1b |
| O-13 | **A later option: rigorous eigenvalue enclosures.** The checks of §3.4 are estimates under premises. Enclosures would need interval or exact arithmetic: interval-verified inertia counts (or exact rational LDLᵀ for small models), and then Kato–Temple or Lehmann bounds across a gap that those counts establish, on a pencil whose formation is itself enclosed. Kato–Temple across gaps between *computed* eigenvalues proves nothing if one is missed (K = I, M = diag(1, 2, 3): with μ = 2 missed, the computed vector (cos t, sin t, 0) has a Kato–Temple bound of sin²t·cos²t / (2 − sin²t) against a true error of sin²t). Codex's round-2 witnesses show that the binary64 versions of these steps do not give enclosures. Any such option is a new commitment, outside F1's | Later change | Not in F1 |
