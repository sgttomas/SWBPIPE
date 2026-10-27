# F1 modal analysis: governing design

**Status.** This is DEL-04-09's governing design for F1, modal analysis (natural frequencies and mode shapes). Scope entered by SCA-013 (`execution/_ScopeChange/SCA-013_2026-10-27_MODAL_ANALYSIS/DECISION.md`). Nothing is built. The decisions below are Agent 0's (HELP_HUMAN) of 2026-10-27, made under the owner's delegation in `loop/LOOP_INIT.md` (#1662). This design makes no acceptance, issuance or lifecycle statement (F-PIP-3). Standard claim fence applies (claims taxonomy per DEC-081).

**Basis.** It was researched against main `0280fd4486`, together with the PF Timoshenko design `execution/PKG-05/DEL-05-03/Design/pf-timoshenko.md` (#1663, in review; not on main at that commit) and the feature survey `execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md` (#1657, open). Paths are relative to `projects/chirality-piping/` unless they start with `/`. "PP `lib.rs`" is `core/product_physics/src/lib.rs`.

**Labels.**
- **[code]**: a fact about existing code, with file:line.
- **[design]**: a fact about an existing design or record.
- **[decided]**: settled by this design or by Agent 0's decisions, with the reason.
- **[open]**: listed in §9, with its owner.

---

## 0. Scope

- **[design]** FR-024 (dynamic analysis modules) was explicitly deferred by DEC-078 (D-12, owner 2026-07-15). DEC-078 required that "any future dynamics scope re-enters only through a new governed scope change with its own human decision packet" (`execution/_Decomposition/SOFTWARE_DECOMP.md`, DEC-078).
- **[decided]** SCA-013 re-enters it for **modal analysis only**: PRD FR-SOL-016 and §12.9, scope item SOW-081, and this deliverable. Agent 0 decided it under the owner's delegation of 2026-10-27; the reason is recorded in the SCA-013 decision.
- **Out of scope:** response spectrum (F2), time history (F3), harmonic analysis and every other dynamic module. They stay deferred under DEC-078.

---

## 1. What F1 solves

### 1.1 The problem

**[decided]** F1 solves the real symmetric-definite generalized eigenproblem

  K_ff φ = ω² M_ff φ

on the free degrees of freedom of the linear pressure-free (PF) model.
- K_ff is the assembled, restraint-reduced stiffness that the PF static solve forms. It is the case-independent part of the static K: thermal, pressure, wind and seismic loads do not enter it.
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
| Imposed displacement (W-16) | **Admitted**: the DOF is a rigid support DOF and stays restrained; the motion value does not enter | [design] PF imposed displacement acts only on rigid support DOFs, by partitioned elimination (DEL-04-08 `Design/retained-pricing-rules.md` §8.23). |
| Nonlinear supports (one-way, gaps, friction) | **Refused by name** (`MODAL_NONLINEAR_SUPPORT_UNSUPPORTED`, naming the support) | [decided] The stiffness depends on contact state. A modal result needs a stated state, and choosing one silently would be an engineering default. A later slice (M5) can linearize at a named static case's converged active set. |
| Exact route (`pressure_contract` present), load/reference-state documents | **Refused by name** (`MODAL_EXACT_ROUTE_UNSUPPORTED`) | [decided] Different section bases (D-5), the exact mass method (`self_weight.rs:840-845`) and its own integrity checks. A later slice. |
| Timoshenko straights (`model_settings.shear_model = timoshenko_annulus_energy_v1`) | **Refused by name in M1** (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`). **M4** admits them with a Timoshenko consistent mass, after the PF Timoshenko unit lands | [design] PF Timoshenko is in review (#1663) and needs model document 0.8.0 (#1649). [decided] Pairing a shear-flexible K with an Euler–Bernoulli mass would be inconsistent, so it is refused rather than mixed. |
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
  - `mode_count` (positive integer), `frequency_cutoff` (positive frequency quantity), or both. A request with neither is refused (`MODAL_EXTRACTION_BOUND_REQUIRED`). With both, extraction stops at whichever is reached first, and the result states which one stopped it.
  - The engine and the headless runner never supply a value. The desktop pre-fills the last value used in the project (M3, §5.1); the request records the value actually used.
- **Modulus basis.** The same vocabulary as a load case: `modulus_basis_ref` or `modulus_basis_temperature`, or neither (base material E, as for a load case without one).
  - [code] Load cases carry these (PP `lib.rs:815-828`), and `materials_for_modulus_basis_observed` resolves them (PP `lib.rs:10842`, called at `:3161`). F1 reuses that resolver unchanged.
- **Weight-to-mass acceleration.** `weight_to_mass_acceleration` (a positive acceleration quantity). It is required when a rigid-family component's weight enters the mass, and it is never defaulted (§2.3).
- **`solver_mode`** chooses the path: `dense_scrutiny` (M1b) or `sparse_interactive` (M2).

### 1.5 Outputs

**[decided]** Per mode i, in ascending frequency:
- f_i (Hz), ω_i (rad/s) and T_i (s).
- The mode shape, mass-normalized (φᵢᵀMφᵢ = 1), as six components per node in global axes. Translations are in m·kg^-½ and rotations in rad·kg^-½. Restrained DOFs are published as exactly 0.
  - Deterministic sign: the largest-magnitude translational component is positive; ties go to the lowest global DOF index.
- For k = X, Y, Z, with r_k the rigid unit translation (1 on each node's k translation, 0 elsewhere):
  - participation factor Γ_ik = φᵢᵀ M_ff r_k (kg^½);
  - effective mass m_eff,ik = Γ_ik² (kg);
  - effective-mass fraction m_eff,ik / (r_kᵀ M_ff r_k), and its cumulative sum.
- Per-mode integrity evidence (§3.4) and a cluster id when eigenvalues are repeated or nearly repeated (§3.4).

Per result:
- total model mass by category: pipe metal, contents, insulation and component bodies (this also gives the survey's A9 weight summary);
- free-DOF mass per direction;
- the Sturm count (§3.4);
- the extraction bound used, which bound stopped extraction, and any truncation ("N modes extracted; the Sturm count shows K more below the cut-off").

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
- m̄ = metal + contents + insulation, exactly `stable_mass_per_length` (`self_weight.rs:731`).
- j̄ (polar, per length) = ρ_p·J_metal + ρ_ins·J_ins, with J of an annulus = A(r_o² + r_i²)/2. Contents are excluded (D-e).
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
6. **Binary64.** m = W/a is one correctly rounded division, and the identity's a·m is one more rounding. So a·(W/a) is within two roundings of W, not always equal to it. The identity check (§2.5) is judged within its measured tolerance and against the p128 re-formation (§3.4); it is exact only in real arithmetic.

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
- Run the self-weight generator in inspection mode over **all** members (`generate_operations(…, true)`), with gravity a along k.
- Compare:
  1. **Straight members, nodally.** With the Hermitian consistent mass, M_e·r_k is exactly the consistent nodal vector of a uniform load: forces a·m̄L/2 at each end and moments ±a·m̄L²/12. The rotary and torsional terms vanish for a rigid translation. Compare with `straight_pipe`'s uniform-load vector for the generated intensity. [design] The PF Timoshenko design keeps the Euler–Bernoulli vector for full-span uniform loads (`pf-timoshenko.md`, #1663), which M4 relies on.
  2. **Every member and component, on the resultant.** Total force, and total moment about a fixed point (the node centroid, to limit cancellation).
- The tolerance is set by measurement under DEC-026 and is only ever tightened. A failure blocks with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`, naming the member or component.
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
6. Back-transform φ = R⁻ᵀ z, then mass-normalize.

- **Control.** The Jacobi pattern, generalized, runs on C in tests for models up to a few hundred DOF. It is an independent kernel-level check of the QL path, not a product path.
- **Memory.** K, M, C and the accumulated Q are alive together, plus one n² work array: about 40·n² B.
- **Time.** Cholesky n³/3, forming C about n³, tridiagonalization 4n³/3, QL with vectors about 6n³: roughly 9n³ flops. n = 3,000 is about 2.4·10¹¹ flops (minutes, single-threaded, optimized build); n = 8,192 is about 5·10¹² (an hour or more). The registered retained identity is a debug, opt-level 0 build (DEL-04-08 `Design/retained-pricing-rules.md` §9), so every timing statement names its build.
- **[decided]** (D-k) Two guards, both applied before allocation and stated with their formula in the refusal text, as `dense_scrutiny_guard` is:
  - **Memory:** `DENSE_MODAL_BYTES_PER_ENTRY` is the count of live n² binary64 buffers times 8 (40 for the layout above; the constant states the implementation's actual count), against the existing provisional 6 GiB ceiling. That admits n ≤ about 12,700 DOF.
  - **Work:** 9n³ against a fixed operation budget, which M1b sets by measurement under `t3_exclusive.sh` so that the dense path stays a scrutiny path for small and medium models (about 3,000 DOF is the target). The value and its measurement are recorded in this design when M1b lands.
  - These are resource limits, not modelling defaults; they refuse and never truncate the extraction.

### 3.3 Sparse path (M2): shift-invert Lanczos on the existing LDLᵀ

**[decided]**
- **Operator.** (K − σM)⁻¹M in the M-inner product, with σ = 0 unless the slicing below chooses otherwise; K's own factor, all pivots positive, is the common case.
  - Lanczos with **full M-reorthogonalization**, a fixed deterministic start vector (M_ff·Σ_k r_k, with a fixed fallback) and a fixed operation order.
  - Basis size q = max(2k, k + 8), grown if not converged, without thick restart in M2.
  - The projected tridiagonal eigenproblem uses the M1b QL routine.
- **Cut-off mode.** Spectrum slicing: repeat at shifts σ_j inside (0, ω_cut²] until the Sturm count at ω_cut² is exhausted.
- **Built on public `sparse_direct` API only:**
  - assemble K − σM over the union pattern;
  - apply the symmetric diagonal scaling D(K − σM)D, D = diag(|k_ii|^-½). This is a congruence, so it **preserves inertia**, and it makes the absolute 1e-12 pivot guard scale-free;
  - then `factorize_ldlt` and `solve_ordered`. `sparse_direct` itself needs no change.
- **The unpivoted-LDLᵀ inertia risk is a first-class test target** (Agent 0, 2026-10-27; R2). `factorize_ldlt` does no pivoting, and on an indefinite K − σM element growth can corrupt the negative-pivot count.
  - Mitigations: place σ away from computed eigenvalues (at least a few residual bounds from any of them); record pivot growth (max|L| and min|d|/|k_ii|); on excess growth, retry at a perturbed σ.
  - **Tests aimed at it:** models built to provoke growth (near-coincident eigenvalues, clusters, rigid-family scales 1e3–1e6, shifts placed close to eigenvalues), each checked against the dense path's eigenvalues and an independent count within the dense guard, with the growth monitor's verdict asserted.
  - If the mitigations fail, the result is refused, never published with an unchecked count.
- **Memory.** K profile, M profile, the K − σM factor and its scaling vector come to about 3·8P B; the Lanczos basis is 8·n·q B. Example: n = 60,000 DOF, mean half-bandwidth 60 (P ≈ 3.6·10⁶), k = 50, q = 110: about 85 MB of profiles plus 53 MB of basis, about 150 MB.
- **Time.** Factorization ≈ n·b̄² (≈ 2·10⁸ flops in the example); each Lanczos step ≈ 6P + 4nq flops; ≈ 5·10⁹ per slice. Each Sturm check costs one more factorization.
- **[decided]** A sparse memory guard states 8·(3P + n·q) against a ceiling before allocation, in the same style. The mode count enters it through q, so a large request is refused with its formula, never silently truncated.

### 3.4 Numerical integrity checks

Each check follows the K-D5 posture (`frame_kernel/src/structural/formation_check.rs:1-12`; `FORMATION_PRECISION = 128`, `FORMATION_CRITERION = 1e-9`, `:23-27`):
1. **Formation.** K is re-formed by the existing K-D5 machinery. M is re-formed at p128 from the authored operands: OD, wall, mill tolerance, densities, insulation, lengths, arc radius and angle, W, the centre of gravity and a. The binary64 M must be within the criterion entrywise, relative to the row scale, as K is. A breach demotes the result (`SolveQuality::Sensitive`, `structural.rs:365-368`).
2. **Residual bound per mode.** With μ = 1/λ and s = M_ffφ − μ K_ffφ, evaluated with the re-formed matrices and exact accumulation (`frame_kernel/src/exact_sum.rs`):
   - an eigenvalue μ_j exists with |μ − μ_j| ≤ ‖s‖_{K⁻¹} / ‖φ‖_K (the symmetric-definite pencil (M, K)); ‖s‖_{K⁻¹}² = sᵀK⁻¹s costs one solve with K's factor;
   - this gives the relative bound |λ_j − λ|/λ ≤ λ·δ/(1 − λ·δ);
   - a Kato–Temple refinement uses the gap to neighbouring computed eigenvalues where it is sharper;
   - the bound needs no M⁻¹, so it covers a singular lumped M.
3. **M-orthogonality.** max|ΦᵀM_ffΦ − I| over the published modes is within a stated tolerance.
4. **Sturm count (completeness).** Factor K − σM at σ just above the largest reported λ (above its residual bound, below the next computed eigenvalue less its bound).
   - By Sylvester's law of inertia, the negative pivot count (`nonpositive_pivot_count`) must equal the number of modes reported.
   - In cut-off mode σ = ω_cut², and the count gives the number of modes below the cut-off.
   - **A mismatch refuses the result set** (`MODAL_STURM_COUNT_MISMATCH`): a missed mode would otherwise go unnoticed.
5. **Clusters.** Eigenvalues within a relative δ_c of each other share a cluster id (a straight circular cantilever has exact pairs).
   - The k-th boundary moves to include a whole cluster.
   - Readers and references compare clusters as subspaces (principal angles), not as vectors. [design] VAL-REF-02 §10.4: "Cluster repeated or nearly repeated eigenvalues and compare the corresponding subspaces" (`docs/references/validation-programme/piping_solver_benchmark_research_and_agent_plan.md` at `d90e4ee0e5`, lines 669–682; catalogued at `validation/qualification/capability_inventory.json:70-82`).
6. **Weight-basis identity** (§2.5).

Standing:
- **Checks passed** when 1–6 hold within their tolerances.
- **Sensitive** (published, demoted, as on the static path) on a formation or residual breach that is not a completeness breach.
- **Refused** on a Sturm mismatch, a weight-basis mismatch, an acceleration mismatch (§2.3) or a non-SPD K.
- The tolerances are measured and recorded under DEC-026, tighten-only. The fixed limitation text keeps "numerical integrity does not establish physical correctness" (`preview_physics.rs:60`).
- Product texts use the claims registry's numerical-health vocabulary and avoid "verified", "certified" and "guaranteed" (`docs/claims_registry.md`); "Sturm count" is the user-facing name of check 4.

### 3.5 Where the code goes

**[decided]**
- A new crate, `core/solver/modal_eigen`: dense reduction, tridiagonal QL, Lanczos, the Sturm wrapper over `sparse_direct`, residual bounds and participation. Std, plus path dependencies on `sparse_direct` and `frame_kernel`'s `exact_sum`.
- A new module, `core/product_physics/src/modal.rs`: the request DTO, refusals, mass assembly, the weight-basis check and result rows.
- A public entry, `run_modal_preview_with_mode`, beside `run_linear_static_preview_with_mode` (PP `lib.rs:2580`).
- Mass is formed in a **separate pass over `BuiltModel`**, never inside `build_model_for_members` or `build_member`. [design] `build_member` is a 50,544 B frame in the measured AC-004 peak stack chain (DEL-04-08 `Design/stack-headroom-repair.md:30-45`), so growing it would risk the R/64 headroom witnesses.

### 3.6 Retained-memory pricing (D3) and AC-004

- **[design]** AC-004 and REQ-004 govern **retained-precision invocations through the Direct entry** only (DEL-04-08 `ScopeOfWork.md`, REQ-004). "Any change to a priced source or to a reviewed input requires a new actual-source D3 audit".
- **[decided]** F1 is a **separate route that never enters D1 or the retained Direct entry**, so it needs no retained admission and no new D1 premise.
- **It still triggers D3**, because it touches priced sources and one reviewed input:
  - PP `lib.rs` (`mod modal;` and the entry). [design] W-16's one-line `mod imposed_motion;` moved every snapshot-keyed line and its full-file pins (`retained-pricing-rules.md` §8.23).
  - `self_weight.rs`, to expose `stable_mass_per_length` and `component_weights` as `pub(crate)`. Its full file is pinned in `b7a_formation_model.py` (§8.23).
  - **`core/product_physics/Cargo.lock`**, if product_physics depends on the new crate. [code] The lock is reviewed input #1 (`build_identity.rs:158-159`), so a new dependency makes the registered profile `Stale` until re-registered.
  - `core/reporting/result_export/src/semantic_contract.rs`, if the modal table's constants go there (literal-only, §8.23).
- **Not touched:** the preview-physics-2 table (reviewed input #5, `build_identity.rs:166`); F1 adds its own table, which is not a reviewed input. `results.v0.3.schema.yaml` is not a reviewed input (`build_identity.rs:158-181`).
- **Expectation, to be measured:** modal code is unreached from the retained Direct entry, so the priced tree should move only by line shifts and pins, coefficient-identical as W-16's was. The AC-004 stack witnesses are re-run if the D3 moves them.
- **[decided]** The D3 is done once, in the first PR that touches priced files (M1a), and later slices stay out of priced files where possible. D3 runs join the queue on Agent 0's go (§6).

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
- **[decided]** (D-i) Agent 0, 2026-10-27: "NUREG/CR reports are US-government works. Transcribing reference values is allowed with the provenance and public-domain status recorded per IP_AND_DATA_BOUNDARY. Anything copyrighted inside them, such as figures adapted from journals, stays out."
- **Per-reference provenance record.** Each transcribed NUREG/CR problem is committed with a `PROVENANCE.json` beside its fixture under `validation/references/f1_modal/nureg/<report>/<problem>/`, carrying the `docs/IP_AND_DATA_BOUNDARY.md` §4 fields:

  | Field | Content |
  |---|---|
  | `source_name` | Report number, volume, title and the problem's own identifier |
  | `source_location` | The report's NRC accession number (ADAMS) or NTIS/OSTI identifier, and the pages and tables transcribed |
  | `source_license` | "US Government work (NUREG/CR series), public domain per Agent 0, 2026-10-27", with the report's own availability or notice statement quoted from its front matter |
  | `contributor` | The transcribing role (TASK) and its PR; no personal identity |
  | `contributor_certification` | That only problem inputs and reference values were transcribed, and that no figure, table or text marked as copyrighted or adapted from another source was copied |
  | `redistribution_status` | `public_permissive` |
  | `review_status` | `pending` until the maintainer records a disposition (§9) |

  The record also lists what was **excluded** as copyrighted or adapted, with its location in the report, so a reviewer can see the line drawn.
- **Use.** NUREG/CR frequencies are numerical results of other programs, not exact answers ("NRC and ANSYS outputs retain shared lineage", `capability_inventory.json:2530-2533`). They are class-c comparison evidence at a stated tolerance.
- **Other sources:**
  - **Code_Aster** SDLX02 (Hovgaard modal) and SDLL14 (elbow frequencies) (VAL-REF-02 lines 94–95): GPL-3.0-or-later, usable for independently implemented fixtures with the licence recorded, not by vendoring.
  - **ASME PVP benchmark papers and NAFEMS natural-frequency benchmarks:** licensed or paywalled. Cite only; copy no data.
  - **CAEPIPE:** no outputs are copied. Any comparison is the owner's private exchange.
- **[decided]** M1–M3 rely on classes a and b. One NUREG/CR-1677 Vol. 1 problem, whose modal part needs only frequencies and participation, is the first class-c target.

---

## 5. Desktop UI, result schema and readers

### 5.1 Where results appear (M3)

- **[code]** The viewport's deformed shape is a display transform of result rows. `buildDeformationOverlay` reads `displacement_magnitude` and `global_nodal_displacement_{x,y,z}` rows, normalizes by the maximum to a display offset of 0.65 and labels the boundary `scale=normalized_display_offset_not_physical_length` (`apps/desktop/src/features/viewport/deformationOverlay.ts:20-116`). It is consumed in `PipeViewport.tsx:618` (three.js, `:7`). No animation loop exists in `PipeViewport.tsx` today.
- **[decided]**
  - **Request entry.** The modal request form takes the extraction bound with no pre-set value in a new project. The desktop remembers the last value used per project and pre-fills it; the user sees and may change it before solving.
  - **Modes table** in `features/results/` (a new `ModalResultsPanel.tsx` beside `ResultsPanel.tsx`). Columns: mode; f (Hz), with T (s) and ω (rad/s) in project display units; effective-mass fractions X/Y/Z and cumulative; cluster; standing (checks passed or sensitive, with the residual bound).
  - **Header lines:** the Sturm count ("N modes below f_cut; N extracted"), the extraction bound and which bound stopped extraction, the modulus basis, the weight-to-mass acceleration, total and free-DOF mass, and disclosures (massless connectors and tie rods, constant-effort supports, mass at restrained DOFs, contents excluded from rotary inertia).
  - **Mode-shape view.** A new `features/viewport/modeShapeOverlay.ts`, reusing the 0.65 display offset and the `scale=…not_physical_length` boundary wording. Positions are x₀ + sin(phase)·offset·φ̂, with φ̂ normalized to unit maximum translation. It is animated by a viewport-owned loop that runs only while a mode is shown. Under `prefers-reduced-motion`, or by user choice, it shows the static extreme shape with a phase step control (DEL-07-06). Display-only Hermitian interpolation along straight members from nodal rotations is optional (Euler–Bernoulli only).
  - **Report section** "Modal analysis" (`core/reporting/report_sections`; the desktop's `features/report/`): request and basis, mass summary, modes table, integrity evidence, refusals and disclosures, limitations. No claim language beyond the existing fences.
  - **Survey A6** (key-results panel) gains frequencies once F1 exists, in A6's own slice.

### 5.2 Result schema and semantic contract (M1b)

- **[code]** Result profiles today: `formulation_basis.profile_id` ∈ {`product_preview_mechanics_v1`, `exact_pressure_v3`, `product_preview_retained_w1a_v2`, `exact_straight_retained_w1a_v2`} (`schemas/results.v0.3.schema.yaml:1035-1041`). Semantic tables are identified as at `core/reporting/result_export/src/semantic_contract.rs:126-128` (`…/preview-physics-2`).
- **[decided] Additions:**
  - **Profile:** `product_preview_modal_v1`, with its fixed limitation 0: "Small-displacement, linear-elastic Euler-Bernoulli frame modal preview about the unloaded configuration, with consistent mass on straight members and lumped rigid-body mass on realized arcs and component bodies; numerical integrity does not establish physical correctness."
  - **Table:** `openpipestress.result_semantics/0.3.0/modal-1` (`fixtures/results/semantic_contract_v0_3_modal_1.json`, new, not a reviewed input). Row kinds:
    - `modal_frequency`, `modal_circular_frequency`, `modal_period`;
    - `mode_shape_{ux,uy,uz,rx,ry,rz}` (mass-normalized);
    - `modal_participation_factor_{x,y,z}`, `modal_effective_mass_{x,y,z}`, `modal_effective_mass_fraction_{x,y,z}`;
    - `modal_mass_total_by_category`, `modal_mass_free_dof_{x,y,z}`;
    - `modal_residual_bound`, `modal_orthogonality_deviation`, `modal_sturm_count`.
    - Each row carries `basis_ref {ref_type: "mode", ref_id: "mode:<i>"}` and, where one exists, `cluster_ref`.
  - **Run kind:** a `modal` analysis run in `schemas/analysis_run.v0.3.schema.json`, so stored results keep the request (extraction bound, modulus basis, a).
  - **Readers:** a Rust `core/reporting/result_export/src/modal_evidence.rs` and a Python mirror under `tests/verifiers/`, sharing no code. They check the profile and limitation against the request; re-derive the free-DOF and total masses from the model (§2) and check Σ_i m_eff,ik ≤ r_kᵀM_ffr_k; and check the counts, ordering, sign convention and cluster structure. They do not re-run the eigen-solve (the DEF-O stance, DEL-04-08 REQ-005).
  - **Desktop and headless:** a Tauri command beside the static one (`apps/desktop/src-tauri/src/lib.rs:44`, `:1904`), TS types, and a headless runner command (`core/runner/headless`).

---

## 6. Sequence and size

Agent 0's sequence, 2026-10-27:
1. **M1a, mass:** `modal.rs` request DTO and refusals (§1.2, §1.4); mass assembly (§2.2–§2.4); the W/a basis and its refusals (§2.3); the weight-basis check (§2.5); mass summary rows; the discretization warning and the derivation of c (§2.6); the `pub(crate)` seams in `self_weight.rs`. References for the mass and resultants in the `w15_component_weight` generator style. **The one D3** (PP `lib.rs`, `self_weight.rs`).
2. **M1b, the dense path:** `core/solver/modal_eigen` (Cholesky reduction, tridiagonal QL, the Jacobi control in tests, residual bounds, M-orthogonality, the Sturm wrapper, participation); the product entry; the dense memory and work guards; the modal-1 table, profile and run kind; the Rust and Python readers; the headless command. References for §4.2 and §4.3. D3 for the PP `Cargo.lock` (and `semantic_contract.rs` if used), joined with M1a's if they merge together.
3. **M2, the sparse path, with Sturm and inertia certification:** shift-invert Lanczos with full reorthogonalization, spectrum slicing, scaled inertia factorization, growth monitoring and the sparse guard; "the unpivoted-LDLᵀ inertia risk" is "a first-class test target" (§3.3); scale benchmarks timed under `t3_exclusive.sh`. No D3 if confined to `modal_eigen` and `modal.rs`, to be confirmed by the pricing tool.
4. **M3, UI:** the Tauri command and types, the request form with the per-project remembered bound, the modes table, the mode-shape overlay and animation with reduced motion, the report section (Rust and TS), Vitest and a native smoke step.

**Start and queue.** The build starts after PF Timoshenko (#1663) is in review. D3 runs join the queue on Agent 0's go.

**Size (estimates, not measurements):**

| Slice | Product and test lines | Reference lines | PRs | D3 |
|---|---|---|---|---|
| M1a | 1,000–1,400 | 400–600 | 1 | yes, once |
| M1b | 2,000–2,800 | 800–1,200 | 2 | yes (lock), joinable with M1a's |
| M2 | 1,000–1,500 | reuse M1, plus generated large models checked against dense | 1–2 | expected none |
| M3 | 1,700–2,300 | – | 2 | none |
| **F1** | **≈ 5,700–8,000** | **≈ 1,200–1,800** | **6–7** | one or two |

**Later slices, outside F1's core:**
- **M4.** Timoshenko consistent mass (interdependent interpolation, e.g. Friedman & Kosmatka, *Computers & Structures* 47(3), 1993; cited, derived). It requires the PF Timoshenko unit and model document 0.8.0 (#1649).
- **M5.** Nonlinear supports linearized at a named case's converged state.
- **M6.** The mass-point helper: a model operation that subdivides spans.
- A concentrated-mass field (W-17, a new document version).

**Dependencies on other lanes:**
- **W-17 (model document):** none for M1–M3, since every parameter is on the request. M4 needs 0.8.0's `shear_model` (#1649).
- **PF Timoshenko** (#1663): M4 only, and the start condition above.
- **Survey A9** (weight and centre-of-gravity summary) falls out of M1a's mass assembly. A6 consumes M3.

---

## 7. Settled decisions

| ID | Decision | Settled as | Reason |
|---|---|---|---|
| **D-a** | Re-enter FR-024 for modal analysis and create the deliverable | **Taken:** SCA-013, FR-SOL-016, SOW-081, DEL-04-09. F2 and F3 stay deferred | Agent 0 under the owner's delegation (SCA-013) |
| **D-b** | Lumped or consistent mass | Consistent on straights; rigid-body lumps on arcs and component bodies; lumped diagonal only as a reference control | Agent 0, 2026-10-27 ("Agreed"); O(h⁴) convergence from above and an element-wise weight identity |
| **D-c** | Mode count, cut-off and any default | Required request input: a mode count, a cut-off or both, whichever stops first; no default; the desktop remembers the last value per project; clusters completed at the boundary. **Changed from the proposal:** no fixed source cap on the mode count | Agent 0, 2026-10-27: "No code-derived cut-off value ships." A fixed count cap would be a code-derived bound on the extraction; the resource guards (§3.2, §3.3) refuse an oversized request with their formula instead |
| **D-d** | Output units and normalization | f (Hz), ω (rad/s), T (s), mass-normalized shapes and kg-based participation; display in project units; unit-maximum shapes in the UI only | Unit-maximum shapes alone would lose the participation identity F2 needs |
| **D-e** | Contents in axial and torsional inertia | In all three translations; excluded from torsional and rotary inertia, disclosed | [code] Self-weight applies m̄, contents included, along any global axis (§2.1), so the identity holds in every direction only if contents are in every translation |
| **D-f** | Component mass m = W/a | `weight_to_mass_acceleration`, required when a component weight enters, never defaulted; equal bit for bit to any current applied component-weight case's acceleration magnitude, or refused (§2.3) | [code] W is an authored force that the acceleration's magnitude does not scale, and the model holds no gravity; only a request input gives a consistent a without a default. **Changed from the proposal:** the agreement with applied cases is a refusal, not a disclosure, because a different a would give masses that disagree with the model's own static weight |
| **D-g** | Refusals by name | Nonlinear supports, gaps, friction, the exact route, load/reference-state documents and (until M4) Timoshenko straights refused by name; constant-effort supports, connectors, tie rods and W-16 imposed displacement admitted with disclosures | Linearizing silently, or solving about an unstated state, would be an unstated engineering choice |
| **D-h** | Parameters on the request or in `model_settings` | On the request, captured in the analysis-run record | No W-17 version, and F1 stays uncoupled from the document lane |
| **D-i** | NUREG/CR transcription | Allowed, with the per-reference provenance record of §4.4; copyrighted or adapted content excluded | Agent 0, 2026-10-27. **Changed from the proposal**, which held transcription until a redistribution check |
| **D-j** | Participation and effective mass in M1 | Yes, translational X/Y/Z | Without them there is no mass-coverage evidence, and a close frequency does not qualify a wrong mass model (VAL-REF-02 line 682) |
| **D-k** | Dense modal guards | Memory (live n² buffers × 8 B against the provisional 6 GiB ceiling) and work (9n³ against a budget measured in M1b) | A memory-only guard would admit models that run for an hour on the dense path |
| **D-l** | Discretization factor c | Derived by benchmark in M1a; a warning, not a refusal, with c and its derivation in the warning text | Coarse models otherwise give high frequencies with clean residuals (R3) |

Items that reach the product or data boundary are marked **for Agent 0** in §9.

---

## 8. Risks

- **R2, inertia from unpivoted LDLᵀ.** `factorize_ldlt` does no pivoting (`sparse_direct/src/lib.rs:377-381`). On an indefinite K − σM, element growth can corrupt the negative-pivot count. Mitigations and tests: §3.3; a doubt refuses.
- **R3, mass discretization.** Too few nodes on long spans give wrong frequencies with clean residuals: the residuals check the discrete problem, not the physics. The warning (§2.6) and the M6 helper address it, and the limitation text says so.
- **R4, repeated eigenvalues.** Symmetric layouts give non-unique bases. Determinism holds (fixed start vector, fixed order, one thread), but bases can change under small input changes. Clusters are published and compared as subspaces.
- **R5, two mass bases in the product today** (`self_weight.rs:731` against PP `lib.rs:10293`). A finding, recorded on the remaining-items list (§2.1). No action in F1.
- **R6, rigid-family scaling.** s·E up to and beyond the 1e5 precision-warning scale (`rigid_component.rs:17-24`) adds very stiff, high-frequency directions and raises K's condition number. The low modes stay the well-conditioned end under K-based reduction, but residual bounds widen. The W-15 precision warning stays, and the references include a rigid component at s = 1e3…1e6.
- **R7, D3 and the reviewed lock.** A new crate dependency changes `Cargo.lock`, a reviewed input, so the profile is `Stale` until re-registered (§3.6).
- **R8, timing claims.** The dense path's cost is cubic and debug builds are much slower. Every timing statement names its build and runs under `t3_exclusive.sh`.
- **R9, stack.** Mass formation stays out of the frames on the W1 peak chain (§3.5). The AC-004 witnesses are re-run if D3 moves anything.
- **R10, public benchmarks' lineage.** NUREG/CR modal results are numerical, from other programs, so a match is comparison evidence, not an exact check.

---

## 9. Remaining open items

| # | Item | Owner | When |
|---|---|---|---|
| O-1 | Independent Codex review of the W/a basis (§2.3), as Agent 0 directed | Manager, then Agent 0 | Before M1a starts |
| O-2 | **For Agent 0 (data boundary):** who records `review_status` for a transcribed NUREG/CR fixture. `docs/IP_AND_DATA_BOUNDARY.md` §4 keeps records pending "until a maintainer records a review disposition", and DEC-027 names the owner as sole maintainer | Agent 0 | Before the first class-c fixture is committed |
| O-3 | **For Agent 0 (data boundary):** NUREG/CR reports are prepared by contractors (BNL) for the NRC. Where a report's front matter carries a copyright or contractor-rights notice, the rule proposed here is to transcribe nothing from it until Agent 0 rules | Agent 0 | At acquisition of each report |
| O-4 | The work budget of the dense guard (§3.2), set by measurement | M1b | M1b |
| O-5 | The discretization factor c (§2.6), derived by benchmark | M1a | M1a |
| O-6 | The integrity tolerances (§3.4) and the identity tolerance (§2.5), measured under DEC-026 | M1a, M1b, M2 | As each lands |
| O-7 | Where the desktop keeps the remembered extraction bound per project (project store or desktop settings), and whether that needs a store version under the one-version rule. It is persistence, so it takes targeted checks | M3 | M3 design |
| O-8 | Adding `product_preview_modal_v1` to `results.v0.3` and the `modal` run kind to `analysis_run.v0.3`: whether they are additions within the current versions or need new versions under the one-version rule (DEL-00-06's contracts) | M1b, with DEL-00-06 | M1b design |
| O-9 | A concentrated-mass field and component rotary inertia (W-17), entering self-weight and modal together | W-17 lane | After F1's core |
| O-10 | M4 (Timoshenko mass), M5 (linearized nonlinear supports), M6 (mass-point helper) | Later slices | After F1's core |
| O-11 | One mass basis: PF seismic generation and self-weight (finding R5), with the deferred PF seismic-mass alignment to D-5 | Remaining items (`docs/COMPLETENESS_MAP.md` §4.3) | Not in F1 |
