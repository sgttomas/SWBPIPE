---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-09
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-081]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-09 Modal analysis

## Purpose and Objective Traceability

This deliverable computes how a piping system vibrates: the natural frequencies and mode shapes of the linear model, with the mass each mode carries. An engineer uses them to judge whether a line is at risk from a pulsating or rotating source, and they are the basis of the response-spectrum seismic analysis. The mass is the same mass the model's self-weight uses, so the two cannot disagree. Each extracted set is checked for numerical integrity and for completeness, so that a missing mode is detected and refused instead of published, and anything the method does not admit is refused by name. Every engineering input (the extraction bound, every mass input and the weight-to-mass acceleration) is the user's, and none is defaulted.

It serves SOW-081 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-09), and PRD FR-SOL-016 and §12.9. In `docs/RELEASE_TARGET_v1.0.md` it is the owning deliverable for modal analysis under exit criterion 1 (the invented reference system, with its gap, lift-off, friction, variable spring, constant-effort support, expansion joint with tie rods, eccentric reducer, valves, flanges and tee with modifiers, solves its modal analysis on the exact route in the desktop with no admitted family refused) and under exit criterion 2, which requires closed-form references and at least one published piping benchmark (AC-010).

- **OUT-001** — The mass model: consistent mass on straight members, rigid-body lumps on realized arcs and component bodies, on the self-weight basis, with its weight-basis identity check (REQ-001).
- **OUT-002** — The modal request: its required extraction bound, modulus basis and weight-to-mass acceleration (REQ-002).
- **OUT-003** — The eigen-solution paths, dense and sparse, and their integrity checks (REQ-003, REQ-004).
- **OUT-004** — The modal results: the profile, the semantic table, the modal result family and the separate readers (REQ-005).
- **OUT-005** — The admitted domain, the resource limits, the desktop and report surfaces, the claim boundary and the published-benchmark comparison (REQ-006 to REQ-010).

## Deliverable Definition — Ontology

### CLM-001 — The method

The modal solve is the generalized eigenproblem K_ff φ = ω² M_ff φ on the free degrees of freedom of the linear model, on the pressure-free (PF) route or the exact pressure (EX) route.
- K_ff is the restraint-reduced stiffness of the route's static solve, without load-dependent terms (EX adds none: pressure enters only as loads). Nonlinear supports enter at the state of a named solved case (REQ-006), and dynamic-only restraints when the request includes them.
- M_ff is the restraint-reduced mass of the mass model (REQ-001).

For each mode it publishes the frequency, circular frequency and period; the shape, normalized to unit maximum translation, or, for a mode with no translation such as pure torsion, to unit maximum rotation (REQ-005), and its generalized mass; and the translational participation factors, effective masses and effective-mass fractions, with their cumulative sums. For each result it publishes the model's mass by category and the free-DOF mass.

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:** the mass model and its weight-basis identity check; the modal request, its refusals and disclosures; the eigen-solution (dense reduction and QL, shift-invert block Lanczos, the Sturm count with its shift placement and growth limit, the modal formation check, residual estimates and participation); the product's modal entry and the headless modal entry, a command the headless runner hosts (DEL-10-05/REQ-015) beside DEL-04-07's solve entry, reading that entry's built model and supplying no value of its own; the modal profile, the `modal-1` semantic table and the modal result family `openpipestress.modal_result`; the product's modal reader and its independent test verifier; the frozen references under `validation/references/f1_modal/`; and the desktop modes view, the mode-shape view and the report's modal section, within the results viewer's and report generator's contracts.

**Inputs from neighbours** (`deliverable.yaml`): DEL-04-01 (assembly, reduction, the curved-bend element and the sparse factorization, with the read-only factorization accessor and the element re-formation entry that DEL-04-01/REQ-019 and DEL-04-01/AC-017 commit to, and that AC-003 examines as this method uses them), DEL-04-02 (the straight pipe element and its uniform-load vector), DEL-04-03 (linear supports and imposed-displacement degrees of freedom), DEL-04-04 (the converged support states of a solved case), DEL-04-06 (singular-model detection), DEL-04-07 (the built model and the static solve), DEL-05-01 (self-weight and its mass per length on both routes), DEL-03-05 (rigid-family components), DEL-02-01 (the snubber family's document shape), DEL-04-10 (the snubber's dynamic meaning), DEL-02-02 (units, including the unit families of modal quantities), DEL-00-06 (the diagnostics contract), DEL-04-08 (the retained-memory pricing law), and DEL-07-05 and DEL-08-01 (the viewer's and report's contracts). Published piping benchmarks are external sources, each used only with its own redistribution evidence (REQ-009). These deliverables keep their scope.

### CLM-003 — What is outside these commitments

- Response-spectrum analysis is DEL-04-10's, on this deliverable's modal basis. Time-history and harmonic analysis are not part of either (PRD §12.9).
- A Timoshenko consistent mass, a mass-point helper and a concentrated-mass field are not committed here; a model that needs one is refused by name.
- The product's second, seismic mass basis is outside this deliverable.

## Completion and Reliance Basis — Epistemology

Nothing in these commitments bounds modelling, input or physical error. The integrity checks concern the discrete problem the method forms. They are numerical-integrity checks with stated premises, not certified enclosures: no commitment here is a mathematical bound on an eigenvalue.

**Undecided.**

- The dense path's work budget is undecided.
- The factor of the coarse-discretization warning is undecided; it is derived by benchmark.
- The first candidate published benchmark, NUREG/CR-1677 Volume 1, has no established redistribution status and is held under REQ-009.

**Admission.** No product surface (desktop, headless runner or report) admits a user's modal request, or shows or records a modal result as an engineering result, until on that surface the dense path's work guard has its measured budget, the checks of REQ-003 and the guards of REQ-007 apply, and the product reader of REQ-005, rederiving the mass from the model, gives the verdict the surface shows. Until then a modal document is experimental. The restriction lifts on a surface when the evidence on that surface shows each of these conditions holds, and is not released otherwise.

### Mass

- **REQ-001** — *The mass basis is the self-weight basis.*
  - **Straight members** carry the Hermitian consistent mass, with linear axial and torsional mass and Rayleigh rotary inertia. Their mass per length is exactly the self-weight generator's mass per length on the model's route (`stable_mass_per_length`): on PF, `source_od_effective_wall_areas/v1` (metal and contents on the wall t − c_m); on EX, `source_od_nominal_wall_pressure_bore_areas/v1` (metal on the nominal wall, contents on the pressure bore); insulation alike on both.
  - **Inertia terms.** Contents are in all three translations and excluded from torsional and rotary inertia, which is disclosed.
  - **Realized arcs** carry two rigid-body half-arc lumps with the half-arcs' exact mass, centroid and second moments.
  - **Components.** An eligible rigid-family component carries a rigid-body lump of mass W/a at its centre of gravity, in place of the member's pipe mass. Eligibility is decided by the same function self-weight uses, called once per member. A component that self-weight does not apply keeps the pipe mass, with the same warning.
  - **Mass-bearing members.** Mass is formed on every member except an objective connector's replaced span, which carries neither self-weight nor mass; one shared selection serves mass assembly and the identity check.
  - **Range.** W and a must be positive normal binary64 numbers, fl(W/a) a normal binary64 number, and a·fl(W/a) finite and normal, and every entry of the component's spatial mass block (m·r_i, m·r_i·r_j) finite and either zero or normal. Otherwise the request is refused by range (`MODAL_COMPONENT_MASS_OUT_OF_RANGE`, naming the component). Under that premise a·fl(W/a) differs from W by at most 2u + u² relative, u = 2⁻⁵³.
  - **Massless families.** Objective connectors, tie rods and constant-effort supports carry no mass, which is disclosed.
  - **Restrained DOFs.** Mass at restrained DOFs is excluded from M_ff and disclosed with the total model mass.
  - **No defaulted mass.** No density, mass or acceleration is defaulted; a missing required input refuses by name.
  - **Weight-basis identity.** Every result runs a blocking check that g·M·1 equals the self-weight. For each global direction k, a·M·r_k on the full mass is compared with the self-weight generator's loads at gravity a along k over the mass-bearing members, on the model's route. The identity holds nodally for straight members, against the consistent uniform-load vector, and on the resultant force and moment for arcs and components, each within a measured tolerance; it is not literal vector equality for every family. The tolerance is 2⁻⁴⁴ relative with an absolute member of 0, an acceptance criterion for the formed mass and not a guarantee that every error is detected; it is only tightened. A failure refuses with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`, naming the member or component.
- **AC-001** — On the frozen `f1_modal` mass references, on both routes' mass bases, for straights, arcs, an eligible component, a component with an offset centre of gravity, an ineligible component and the massless families:
  - the product mass matrices, totals by category and free-DOF masses agree with the references within their recorded tolerances;
  - the identity check passes on each reference model, within the stated tolerance;
  - a corrupted formed mass, with the model and its self-weight basis unchanged (a member's formed mass scaled as if its density differed, a component body dropped, a component body on the wrong node), is refused with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`;
  - a legitimate density edit to the model changes both bases together, and its result passes the identity;
  - a connector's replaced span carries no mass and the identity holds with it excluded;
  - a component outside the range premise, including a spatial-mass entry that overflows, is refused with `MODAL_COMPONENT_MASS_OUT_OF_RANGE`.

### Request

- **REQ-002** — *Required inputs, no defaults.*
  - **Extraction bound.** It is a mode count n, a frequency cut-off f_c or both, and at least one is required. The engine and the headless runner supply no value, and no code-derived cut-off ships. A request without one is refused (`MODAL_EXTRACTION_BOUND_REQUIRED`).
  - **Reported set.** The lowest modes up to whichever bound stops first: the lowest n, or all modes with f ≤ f_c (the cut-off is inclusive). The result states which bound stopped it.
  - **Clusters.** A cluster is a maximal run of adjacent computed eigenvalues, each within the stated relative cluster tolerance δ_c of the next (λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}). If the boundary falls inside a cluster, the set is extended to complete the cluster, and the result discloses the extension by count. A partial cluster is never published.
  - **Completeness count.** It is taken at a final shift in the gap between the completed last reported cluster and the next computed cluster, under REQ-003's separation premise. The result states the interval it covers. Two endpoint cases:
    - **Empty set:** a cut-off below the lowest frequency reports no modes. The count is taken at a shift in the gap between f_c and the lowest computed cluster, under the same premise, and must be 0. The result states that no mode lies at or below f_c under that premise.
    - **Whole spectrum:** if the reported set is the whole finite spectrum, the final shift lies a stated relative offset above its largest eigenvalue, and the separation applies on the lower side only. Only the dense path claims it, and only when the number of finite modes is established independently (structurally massless DOFs and a Cholesky of the remaining mass block); otherwise a request that needs it is refused (`MODAL_FINITE_COUNT_UNRESOLVED`).
  - **Cut-off sizing.** On the sparse path, a cut-off-only request is sized by a count at a stated relative offset above f_c. That count only sizes the extraction and its allocation; it is neither the completeness count nor a termination rule. When the cut-off controls stopping, extraction continues until a completed computed cluster lies above f_c; on the dense path, which computes every finite eigenvalue, a cut-off above the whole finite spectrum uses the whole-spectrum endpoint instead, and on the sparse path such a request is refused (REQ-004). If the factorization there is singular or rejected by the growth monitor, it is retried at a second, documented offset, which the result records; the offset never changes silently.
  - **Modulus basis.** Without a state case, it uses a load case's vocabulary and the existing resolver; with a state case, the modulus comes from that case's resolved state, and a request modulus basis is refused as a conflict (`MODAL_MODULUS_BASIS_CONFLICT`).
  - **State case.** When the document is a load/reference-state document or the model has a nonlinear support, the request names a load case whose resolved, solved state defines the modal model: its stiffness (each member's modulus selection included), its mass (the state's self-weight inputs, against which the weight-basis identity is checked), its nonlinear-support states (REQ-006) and its restrained DOFs, with any `boundary_motion` or imposed displacement kept restrained and its motion value excluded. The modal entry resolves and solves that case on the current model in the same run, reusing no stored result, after the static entry's own admission checks for it (including the replaced-span strain refusal and the refusal of a document whose every case is a zero-action reference state); prescribed values, eigenstrain and declared loads enter that state solve and are excluded only from the eigenproblem. The identity check inspects self-weight on the intact model without clearing, adding or renaming cases. It has no default; a request without it is refused (`MODAL_STATE_CASE_REQUIRED`, naming why). An unresolvable or unconverged state is refused (`MODAL_STATE_CASE_UNAVAILABLE`). The result states the case and what it supplied.
  - **Dynamic-only restraints.** When the model has a snubber, the request states whether dynamic-only restraints are included. It has no default; a request without the choice is refused (`MODAL_DYNAMIC_RESTRAINT_CHOICE_REQUIRED`). A response-spectrum solve includes them (DEL-04-10).
  - **Weight-to-mass acceleration.** `weight_to_mass_acceleration` is a positive acceleration, normalized through the unit catalog.
    - It is required when a component's weight enters the mass; without it the request is refused (`MODAL_WEIGHT_ACCELERATION_REQUIRED`, naming the components).
    - When the model carries a current applied component-weight self-weight primitive, the magnitude of that primitive's recorded acceleration must equal a, bit for bit after normalization. Otherwise the request is refused (`MODAL_WEIGHT_ACCELERATION_MISMATCH`, naming the case and both values).
  - **Recording.** The request is recorded with the result (the document's binding: its hashes, every option and the state case). A modal result is not recorded as an analysis run: it is not kept with the project and is run again after the project is reopened, and a report binds a modal result by the result's own identity.
- **AC-002** — Maintained tests show:
  - each refusal for a request without a bound, without a required acceleration, with a mismatched acceleration, without a required state case, with a modulus basis that conflicts with the state case, or without a required dynamic-restraint choice;
  - extraction stopping at the count, at the cut-off, and at whichever comes first when both are given, with the stopping bound stated;
  - a mode exactly at the cut-off included, with the completeness count taken at the final shift above it;
  - on the sparse path, the second sizing offset used and recorded when the first is singular, and an underestimated sizing count leading to further extraction, never to an omitted mode below f_c;
  - a boundary inside a cluster extending the set to the whole cluster, disclosed by count, for both the count and the cut-off;
  - a cut-off below the lowest frequency reporting no modes, with a zero count taken in the gap above f_c, and a request whose set is the whole finite spectrum, with the final shift above its largest eigenvalue; and the rounded-infinite witness (K = I₂, M with determinant 2⁻¹⁰⁴), whose second finite mode rounds to infinite, refused with `MODAL_FINITE_COUNT_UNRESOLVED` for a cut-off that needs it; and ordinary small-bore and short-member models admitted with their full finite counts;
  - no default supplied on any entry: product, headless and desktop.

### Solution and integrity

- **REQ-003** — *Dense path and integrity checks.*
  - **Method.** The dense path reduces with K's Cholesky factor, tridiagonalizes and applies implicit QL, with no new numerical dependency. Its Sturm counts factor the scaled K − σM with `sparse_direct`'s LDLᵀ, observed through one read-only accessor on the factorization, with no mutation and no allocation (an input from the kernel's code).
  - **What the checks are.** They are numerical-integrity checks with stated premises. They are not certified enclosures, and no eigenvalue bound is certified. The result's limitation text states what they do and do not establish (REQ-009).
  - **Checks.** Every published mode set carries:
    - (1) a modal formation check: K and M re-formed independently from their authored operands, element by element, in at least double-double precision (≈106-bit significand), the section properties re-derived from OD and wall (with EX's deductions), and the formation difference (formed minus re-formed, on the assembled matrices) measured as a norm and published, with the trusted leaves stated;
    - (2) per mode, the residual ‖Kφ − λMφ‖ evaluated against the re-formed pencil, scaled as a backward error and compared with a stated threshold, together with a residual-based error estimate of the eigenvalue. The estimate is first-order, under the premise that the eigenvalue is isolated, and is stated for a cluster as a whole; it is an estimate, not a bound;
    - (3) the mass-orthogonality deviation of the published modes, normalized by their generalized masses;
    - (4) Sturm-count agreement: the count at the final shift (REQ-002) must equal the number of modes reported, on the interval actually reported. Counts are taken only at shifts placed in gaps between computed clusters. A count is relied on only under two premises, on the scaled matrices: the **conditioning premise**, that the backward-error estimate e_b of forming and factoring the shifted matrix (the rounding of K̃ − σM̃ and its scaling, plus γ_n·‖|L̂||D̂||L̂ᵀ|‖) times an estimate of ‖(K̃ − σM̃)⁻¹‖ is at most a stated fraction, so that the backward error is small against the shifted matrix's distance from singularity; and the **separation premise**, that the shift's separation from the neighbouring computed eigenvalues exceeds a stated multiple of e_b. Both are disclosed with the shift, the estimates and the growth factor g;
    - (5) cluster identifiers by the one cluster definition of REQ-002, with the reported set completed to whole clusters;
    - (6) the weight-basis identity (REQ-001).
  - **Growth limit.** The growth monitor is an input to the premise, not a certificate. A factorization whose growth exceeds a stated limit is rejected.
  - **Standing.**
    - A mode whose residual exceeds the stated threshold, or an orthogonality breach, publishes the result as Sensitive when nothing refuses the set.
    - The set is refused:
      - on a Sturm mismatch (`MODAL_STURM_COUNT_MISMATCH`);
      - when no shift in the gap can be placed with the required separation (`MODAL_STURM_SHIFT_UNSEPARATED`, naming the modes on each side) or conditioning (`MODAL_STURM_SHIFT_ILL_CONDITIONED`), or every such shift exceeds the growth limit (`MODAL_STURM_GROWTH_LIMIT`); such a shift is never published as a count;
      - when the model has no mass on its free DOFs, so no finite mode exists (`MODAL_NO_FINITE_MODES`);
      - when an element family has no re-formation (`MODAL_REFORMATION_UNAVAILABLE`);
      - on a weight-basis or acceleration mismatch, a component mass out of range, or a non-positive-definite K.
    - The kernel integrity check's static criterion is not reused; only its element re-formation is, through a read-only entry.
  - **Tolerances.** The residual threshold is 2⁻³⁰ (a mode above it is Sensitive, never refused for that alone), the orthogonality tolerance 2⁻²³, the cluster tolerance δ_c = 2⁻²⁰, the growth limit 2⁵², the separation multiple 2⁷ and the conditioning fraction 2⁻¹⁰. Each is an acceptance, demotion or grouping criterion, not an error or count theorem, and is only tightened.
  - **Determinism.** The output is deterministic for a fixed model, request, solver version and settings.
- **AC-003** — On the frozen `f1_modal` references, the dense path:
  - reproduces the class (a) discrete references at their recorded tolerance (seeded at 1e-9 relative for frequencies), and converges to the class (b) continuum references at the expected rate;
  - compares mode shapes and clusters by MAC or subspace angle;
  - agrees with the independent Jacobi control in tests;
  - passes each check (1) to (6) on the references, with its Sturm counts equal to the reference generator's exact counts;
  - refuses or demotes each committed mutation at its named check, including a formed K or M perturbed away from its re-formation;
  - detects a deliberately missed eigenvalue (K = I, M = diag(1, 2, 3), one direction withheld) by the Sturm count, and refuses the set;
  - refuses, by name, a shift that cannot be separated, including the near-eigenvalue witness (a 32×32 tridiagonal on which a shift within 10⁻¹⁷ relative of an eigenvalue rounds away and gives a wrong count with unchanged growth observations), a factorization over the growth limit, the formation witness (K = I₂, σ = 1.125, a binary64 M whose exact shifted matrix is positive definite but whose formed shift counts one negative pivot), and the ill-conditioned witness (K = diag(1, 1, B), M = diag(1/32, 1/256, 8B), B = HHᵀ, H the 40×40 unit lower bidiagonal with subdiagonal 2), on which a widely separated shift gives a wrong count because the shifted matrix is ill-conditioned; tests assert the refusal of every shift that binary64 miscounts, not a particular count;
  - shows, on models that provoke pivot growth (near-coincident eigenvalues, clusters, rigid-family scales from 1e3 to 1e6, shifts close to eigenvalues), that each count used agrees with the exact count, and asserts the growth monitor's verdict and the separation decision;
  - shows the observation accessor leaves the factorization unchanged and allocates nothing;
  - publishes byte-identical results on repeated runs.
- **REQ-004** — *Sparse path.*
  - **Method.** The sparse path uses shift-invert **block** Lanczos with full mass-reorthogonalization, a deterministic start block (block size 2, from the paired bending modes of an axisymmetric section), locking and deflation of converged pairs, breakdown handling and fixed operation order. Block size is an algorithm parameter, not an engineering default. It works on the existing `sparse_direct` factorization, with an inertia-preserving diagonal scaling. It also converges the next cluster above the reported set, unpublished, so that the final shift lies in a gap between computed clusters.
  - **Breakdown.** A failed replacement column is not evidence that the finite spectrum is exhausted: it triggers the deterministic restart, and then refusal (`MODAL_LANCZOS_BREAKDOWN`). The sparse path never claims the whole finite spectrum; a request that needs it is refused there (`MODAL_SPARSE_WHOLE_SPECTRUM_UNSUPPORTED`) and served by the dense path.
  - **Singular mass.** M is only semidefinite, so every basis vector is kept in the range of (K − σM)⁻¹M, and each Ritz vector is purified by one more application before its checks.
  - **Missed directions.** The Sturm count detects a missed direction. A deterministic restart with a larger block and a new seed recovers it. If the restarts are exhausted, the set is refused; a count disagreement is never published.
  - **Cut-off.** In cut-off mode it slices the spectrum until a completed computed cluster lies above f_c, with q sized by REQ-002's sizing count. Each slicing shift is placed under REQ-003's premises and growth limit.
  - **Unpivoted-LDLᵀ inertia risk.** It uses the dense path's accessor, shift placement and growth limit unchanged. It refuses a set whose count it cannot take under the premise, and never publishes an unchecked count.
  - **Checks.** The REQ-003 checks apply unchanged.
- **AC-004** — The inertia risk of the unpivoted LDLᵀ factorization is tested directly:
  - maintained tests extend AC-003's growth-provoking models to the sparse path's slicing and final shifts;
  - for each, the sparse path's eigenvalues and Sturm counts agree with the dense path and the exact count, within the dense guard, and the growth monitor's verdict and the separation decision are asserted;
  - a breakdown with a stiff-mass-ratio witness (K = I₃, M = diag(1, 1, 10⁻¹²)) and a cut-off requiring all three modes is refused or recovered, never completed by assumption;
  - the Lanczos kernel recovers the correct purified vector of a singular lumped mass (the 2×2 singular-mass witness, K = [[2, 1], [1, 2]], M = diag(1, 0), as an internal kernel test, since its single finite mode is the whole finite spectrum), and the sparse path publishes a correct set for a singular-mass model with a further finite mode above the reported one;
  - the sparse path recovers both modes of each exactly repeated pair (a straight circular cantilever), and a case with a multiplicity above the initial block size shows the Sturm count detecting the missed direction and the restart recovering it, or the set refused when restarts are exhausted;
  - the sparse path reproduces the AC-003 references;
  - scale benchmarks are timed in an exclusive window with their build named, through the performance harness (DEL-04-05).

### Results and readers

- **REQ-005** — *Modal results and separate readers.*
  - **Publication.** Results are published as the modal result family `openpipestress.modal_result` under the profile `product_preview_modal_v1`, with its fixed limitation text, and the semantic table `openpipestress.result_semantics/modal-1`, outside the `results.v0.3` envelope. Each document binds its request exactly.
  - **Units.** Frequencies are in Hz, circular frequencies in rad/s and periods in s. Every unit has integer exponents in the unit catalog.
  - **Normalization: exactly two cases.** Shapes are in global axes. Components at axis-aligned rigidly restrained DOFs are exactly 0; for a rigid skew restraint, the physical vector satisfies the constraint as specified in REQ-006 (see also AC-006). Each mode records its case.
    - **(a) A mode with nonzero translation** is normalized to unit maximum translation: the largest translational component is +1, translations are dimensionless and rotations are in rad/m. Its generalized mass m = φᵀM_ffφ is in kg, its participation factors Γ_k = φᵀp_k / m are dimensionless and its effective masses (φᵀp_k)²/m are in kg, where p_k = (M R_k)_f is the uniform-support-motion excitation: the free rows of the full mass times the rigid unit translation R_k of all DOFs, so that consistent-mass coupling to restrained DOFs is included. Effective-mass fractions are relative to T_k = R_kᵀM R_k, the model's total mass in direction k.
    - **(b) A mode with no translation**, such as pure torsion, is normalized to unit maximum rotation: the largest rotational component is +1 (rad/rad), and the translations are the computed vector's own, unaltered (m/rad, at most τ_t·ℓ). Its generalized mass is in kg·m²·rad⁻², and its Γ (rad/m) and effective translational masses (kg) are computed from the published vector, so that Γ²·m = m_eff in kg holds in the catalog's dimensions. The integrity checks of REQ-003 are evaluated on the published vector.
    - **Zero denominators.** If the total mass along k is 0, the effective masses along k are 0 and their fractions are published as undefined, with the reason.
    - **The threshold.** A mode has no translation when its largest translation is at most τ_t·ℓ times its largest rotation, with ℓ the diagonal of the bounding box of all the model's nodes (positive for any model with an element) and τ_t = 10⁻⁶ (confirmed by measurement and only tightened). There is no participation condition: a case (b) mode's translational participation, which can be large through the mass coupling to restrained DOFs, is published as computed. The result states τ_t and each mode's case.
  - **Effective-mass fractions** and their cumulative sums are dimensionless.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no code with the producer each check:
    - the profile and limitation against the request;
    - the total and free-DOF masses, rederived from the model;
    - that the effective masses, summed over the published modes, do not exceed the total mass T_k in any direction;
    - each mode's normalization case and sign (its normalizing component +1; in case (b), every translation at most τ_t·ℓ times the largest rotation, with Γ and the effective masses recomputed from the published vector), and Γ²·m = m_eff;
    - the counts, ordering, cluster structure and any cluster extension;
    - the Sturm count against the reported set, its placement and premises, the numerical-evidence relations (e_λ = η·κ_λ), the Sensitive standing and every value's unit.
    They do not re-run the eigen-solve. With the bound request supplied, each independently derives the model's mass and restraint basis within its stated coverage, compares the formed mass and total/free-DOF masses, and recomputes mass-dependent modal quantities from that mass and the published shapes; the document's payload relations are also checked. Missing inputs, withheld payload or uncovered basis are scoped unsupported, and undecidable rounding comparisons are unresolved, never passed or reconstructed from a digest. A partition derived from reported nonlinear states remains conditional on their qualification.
- **AC-005** — The two readers return the same verdict for every committed modal result and every committed mutation of the profile, limitation, masses, normalization, counts, ordering, sign or clusters. The committed results include both normalization cases, among them a pure-torsion mode in case (b). A maintained producer test shows a mode with no free translation but large participation through restrained-DOF coupling (a one-element simply supported span) published in case (b) with that participation. Further producer tests show a case (b) mode published with its computed translations unaltered, its integrity checks evaluated on that published vector, and its Γ and effective masses recomputed from it; and a one-element cantilever, with one free node, classifying its torsional mode by the positive length ℓ. A verifier-independence test shows that the Python verifier imports no producer code.

### Domain, limits and surfaces

- **REQ-006** — *Admitted, disclosed and refused.*
  - **Admitted:** models on the PF route and on the EX route, each on its own section and mass basis, with every family the route's static solve admits, each carrying the mass REQ-001 states: straight members and tee legs; realized arcs; geometry-only bends as their straight members; rigid-family components; linear supports and springs; constant-effort supports, objective connectors and tie rods, disclosed as massless and, for constant-effort supports, as stiffness-free; annotation-only expansion joints as pipe; imposed-displacement DOFs, kept restrained.
  - **Nonlinear supports, linearized and disclosed.** Each nonlinear support takes its final state in the named case, solved in the same run (REQ-002), as the static solver represents it: an active one-way or lift-off support, a closed gap or sticking friction rigidly restrains its DOF; an inactive or open support, including friction whose normal does not bear, leaves it free; sliding friction leaves its tangential DOF free with its friction force frozen, contributing no stiffness, which is disclosed as a modal approximation. A friction support's normal restraint keeps its own state. The result names the case and each support's state. A named case that is absent or whose solve does not converge is refused (`MODAL_STATE_CASE_UNAVAILABLE`, naming the case). A state is admitted only when the static solve confirmed it; a state that is not confirmed is refused the same way.
  - **Load/reference-state documents,** admitted through the state case (REQ-002): stiffness, mass and support states from its resolved state.
  - **Contents mass follows the state case.** Metal and insulation mass are state-independent. The contents mass comes from the state case's resolved contents density wherever that case resolves one (a model document 0.9.0 condition, such as a hydrotest's test contents), and from the document's section contents otherwise; the result discloses which was used.
  - **Skewed restraints.** A skewed rigid restraint (line stop or rigid snubber) is reduced by the static path's division-free nullspace method with the authored direction; a skewed finite stiffness (spring or finite-stiffness snubber) enters as the rank-1 term k·d dᵀ/(dᵀd) with the authored direction. Both are admitted exactly when the static path admits skew. With a rigid skew restraint, shapes are published as the physical vectors E·φ through the constraint basis, the excitation is formed as EᵀM R_k, and the constraint dᵀu = 0 holds exactly in the reduced representation; in the published binary64 physical vector, after lifting and normalization, the constraint residual |dᵀu| is within a stated roundoff tolerance (for example 2⁻⁵³ for d = (1, 49/32, 0) with unit maximum translation), though the global components need not be zero.
  - **Snubbers.** When dynamic-only restraints are included, each snubber enters the modal stiffness as the dynamic-only restraint DEL-04-10/REQ-005 defines; otherwise it is ignored. Either way it is disclosed.
  - **Ignored and disclosed:** thermal, pressure, pressure-region, wind, seismic, nodal and member loads; on EX, the absence of pressure stiffening, as the EX static limitation states.
  - **Refused by name:**
    - Timoshenko straights on either route (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`): a Timoshenko consistent mass is outside these commitments;
    - an unrestrained or mechanism model, through the existing singularity refusal.
  - **Warning.** A member that is too coarsely discretized for the highest reported frequency is warned (`MODAL_MASS_DISCRETIZATION_COARSE`). Its factor is derived by benchmark and stated in the warning.
- **AC-006** — Each refusal and disclosure in REQ-006 is exercised by a maintained test that asserts its code and named object, on both routes. Maintained tests show the static admission checks refusing through the state case (a replaced-span strain; a reference-only document); a load/reference-state document solved at its state case, with per-member moduli and mass from that state matching a discrete reference built with them, and boundary-motion DOFs restrained; two state cases of one document that resolve different contents densities (section contents and a test contents) giving the two different frequency sets of their references, each disclosing its contents source; a nonlinear support taking each state of the constraint table from its named case, with frequencies equal to those of the matching linear model; sliding friction disclosed as frozen; a skewed rigid and finite-stiffness restraint matching the oblique references, perpendicular motion free, and a skewed rigid restraint whose normalized published vector has a nonzero rounded constraint residual within its stated tolerance; and a snubber changing the frequencies only when dynamic-only restraints are included. The discretization warning's factor is derived from a recorded benchmark.
- **REQ-007** — *Resource limits and the retained route.*
  - **Guards.** Before allocation, the dense path applies a memory guard (its live n²-sized buffers, the Sturm factorization's included, against the dense scrutiny ceiling) and a work guard (9n³ plus n³/3 per candidate Sturm shift, at most a stated number of them, against a measured work budget). The sparse path applies a memory guard on its peak live allocation, the maximum over its phases: factorization and extraction, 32P + 8n(q + b) + 24q² + c·n bytes (four profile value arrays: K, M, the shifted profile and its clone in the factor; the Lanczos workspace; the projected problem; the per-DOF metadata); and re-formation, 48P + 8n·q + c·n with the shifted profile and factor released, or 64P + 8n·q + c·n if they are kept (K and M plus the two double-double accumulators). The implementation states c and which buffers it releases between phases. It is rechecked whenever q or b grows. A cut-off-only request is sized from REQ-002's sizing count at the cut-off, taken before Lanczos. Each refusal states its formula. A guard refuses; it never truncates the extraction.
  - **Retained route.** A modal solve never enters the retained-precision entry or its admission domain, and the priced sources it touches stay within the registered memory profile (DEL-04-08/REQ-004).
- **AC-007** — Maintained tests show that:
  - each guard refuses with its formula before allocation, and admits at its limit, including a sparse case in which the re-formation phase governs;
  - the sparse guard is rechecked and refuses when q grows past the limit, and a cut-off-only request is sized from its sizing count;
  - the retained-precision entry is unreachable from the modal entry, and the priced sources price within the registered profile.
- **REQ-008** — *Desktop and report.*
  - **Request form.** The desktop takes the modal request with the extraction bound entered by the user. It remembers the last value per project, in the desktop's settings keyed by the project's identifier and outside the model document, and pre-fills it, visibly.
  - **Results.** The desktop shows a modes table and a mode-shape view. The view is a normalized display transform, labelled as not a physical length, with a static, stepped alternative under reduced motion.
  - **Report.** The rendered report gains a "Modal analysis" section: request and basis, mass summary, modes table, integrity evidence, refusals and disclosures, and limitations.
  - **Contracts.** These surfaces follow the results viewer's and the report generator's output and claims contracts.
- **AC-008** — Desktop tests (Vitest) cover:
  - the request form, including the absence of a pre-set bound in a new project and the remembered value in an existing one;
  - the modes table and its header lines;
  - the mode-shape view, including the reduced-motion path;
  - the report section, which cites the modal result by its own identity;
  - a project reopened after a modal solve: no modal result is shown, and the remembered bound is pre-filled and is not in the model document.

  A native smoke step solves a modal request and shows its table.

### Claim boundary and data

- **REQ-009** — *Claims and references.*
  - **Product texts.** They state the limit "numerical integrity does not establish physical correctness" and use the claims registry's vocabulary (`docs/claims_registry.md`). The profile's fixed limitation text also states what the integrity checks do and do not establish: the residual is evaluated against a double-double re-formation, the eigenvalue error is a first-order estimate, the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues, and none of them is a mathematical bound on the eigenvalues or bounds modelling, discretization or input error. No product text states or implies a claim that PRD §21.2 prohibits.
  - **Reference independence.** Each frozen reference is derived without reading product code and regenerates from a generator that imports none.
  - **Published benchmarks.** A problem is transcribed from a report, such as a NUREG/CR report, only with that report's own redistribution evidence (its rights statement and NRC distribution terms), recorded per fixture with the `docs/IP_AND_DATA_BOUNDARY.md` §4 provenance fields, whose `review_status` stays in that section's vocabulary; an independent agent's verification of the transcription against the source is recorded separately and is not acceptance. A report with uncertain rights is held for the owner's review under that policy, and nothing is transcribed from it. Copyrighted or adapted content inside a report, such as figures adapted from journals, is not transcribed. Sources with clear rights are preferred: closed-form benchmarks derived here, and textbook formulas restated with citation.
  - **Other sources.** No CAEPIPE output, licensed benchmark data or paywalled text is committed.
- **AC-009** —
  - The published limitation text is exactly the profile's fixed text, and no modal product text states or implies a claim PRD §21.2 prohibits.
  - Each frozen reference regenerates byte for byte from its generator, which imports no product code.
  - No published-benchmark fixture lacks a provenance field: each carries its report's rights statement and distribution terms and a list of what was excluded; no fixture exists from a held report.

### Published benchmark

- **REQ-010** — *A published piping benchmark.* At least one published piping benchmark with recorded redistribution evidence (REQ-009) is compared with the product, as comparison evidence, not exact verification (PRD FR-SOL-016; release target exit criterion 2).
- **REQ-011** — *The reference system.* The release target's reference system solves its modal analysis on the exact route in the desktop at a named operating state case, with no family refused that the exact route's static solve admits (release target exit criterion 1).
- **AC-011** — The reference system's modal request, naming an operating state case, solves on the exact route in the desktop with no family refused that the exact route's static solve admits, and its result is shown with its disclosures.
- **AC-010** — A maintained test compares the product with at least one committed published piping benchmark, on frequencies and on normalization-invariant participation (effective masses), at a stated comparison tolerance, and its fixture's provenance record is complete. The comparison is labelled comparison evidence.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *One mass basis with self-weight.* The mass model reads self-weight's own mass per length and eligibility rule, so mass and weight cannot drift apart.
- *No new numerical dependency.* Both eigen-solution paths are in-repository and use the kernel's sparse factorization.
- *Completeness by count.* A Sturm count at a shift in a gap between computed clusters must equal the number of modes reported. A count is relied on only under the conditioning and separation premises; where they cannot be met the set is refused.
- *Integrity against an independent re-formation.* Residuals are evaluated against stiffness and mass re-formed from their authored operands in at least double-double precision.
- *Guards refuse; they never truncate.* A request over a memory or work limit is refused with its formula.
- *Readers do not re-solve.* The product reader and the test verifier check what was published and what they can rederive from the bound request.

**How each criterion is examined.**

- **VER-001** — Comparison with a named reference: the product's mass matrices, totals and identity check against the frozen mass references in `validation/references/f1_modal/mass_discrete`, generated without product code, with corrupted-mass and out-of-range cases refused.
- **VER-002** — Executable check, with comparison: observes each request refusal and each stopping rule on every entry, with the reported set compared against the reference generator's exact eigenvalue counts.
- **VER-003** — Comparison with named references, with an executable check: the dense path against the discrete and continuum references in `validation/references/f1_modal` and an independent Jacobi control; the check observes each integrity check passing on the references, each committed mutation refused or demoted at its named check, the named witnesses refused, the accessor's invariance, and repeat-run byte identity.
- **VER-004** — Comparison with named references, with an executable check: the sparse path against the same references and the exact counts, on repeated modes, missed directions, singular mass and growth-provoking models; agreement with the dense path is observed as a guard; scale benchmarks are timed through the performance harness.
- **VER-005** — Executable check: observes the product reader and the test verifier returning the same verdict on every committed result and mutation, both normalization cases included, and that the verifier imports no producer code.
- **VER-006** — Comparison with named references, with an executable check: each refusal and disclosure of REQ-006 with its code and named object on both routes; state-case, contents, skew and snubber models against their discrete references; the discretization warning's factor against a recorded benchmark.
- **VER-007** — Executable check: observes each guard refusing with its formula before allocation and admitting at its limit, the retained entry unreachable from the modal entry, and the actual-source pricing audit on any change that touches a priced source.
- **VER-008** — Executable check: observes the request form, the modes table, the mode-shape view with its reduced-motion path and the report section in the desktop, and a native run that solves a modal request and shows its table.
- **VER-009** — Executable check with judgment by a reviewer who did not write the text: the check observes the pinned product texts, each reference regenerating from its generator, and every published-benchmark fixture's provenance fields; the reviewer examines the texts against `docs/claims_registry.md`. Each reference is derived by an agent that reads no product code, and each reference package is refuted by a second agent that reads no product code before any product comparison, and each transcription is verified against its source separately.
- **VER-010** — Comparison with a named reference: the product against at least one published piping benchmark, such as a problem of NUREG/CR-1677, on frequencies and effective masses at a stated comparison tolerance.
- **VER-011** — Executable check: the reference system's modal request is run in the desktop on the exact route, observing that no family the static route admits is refused and that each disclosure is shown.

## Governing Values and Decisions — Axiology

- **AX-001** — *No silent engineering defaults.* The extraction bound, the acceleration and every mass input are user-entered or refused (PRD §5.5). Better is fewer values the product supplies on any entry, desktop, headless or engine; ease of use never justifies a supplied value.
- **AX-002** — *One mass basis with self-weight.* Better is a smaller difference between a·M·r and the self-weight loads, measured by the identity check. Agreement is never bought by widening its tolerance or by forming mass from a second source.
- **AX-004** — *A missed mode is the failure to prevent.* Better is more models whose extracted set is shown complete, and fewer refused for want of a countable shift. A check is never loosened, and a count that cannot be taken under its premises is never published; tolerances are only tightened, and frozen references are never altered to make an implementation pass.
- **AX-008** — *A wider domain and larger models within the budget.* Better is fewer refusals of models the static route admits, and less memory and work per extracted mode. This is never bought by truncating an extraction, skipping an integrity check, giving up the weight-basis identity or defaulting an input.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-081 | REQ-001 CLM-001 | AC-001 | VER-001 | Outside source: `validation/references/f1_modal/mass_discrete`, derived without product code and refuted by a second agent |
| OUT-002 | OBJ-003 SOW-081 | REQ-002 | AC-002 | VER-002 | PRD §12.9 and §5.5 state the required inputs and refusals; the reported sets are compared with the reference generator's exact counts, an outside source |
| OUT-003 | OBJ-003 SOW-081 | REQ-003 | AC-003 | VER-003 | Outside source: closed-form beam, rod and ring frequencies and the discrete references of `validation/references/f1_modal`, with exact Sturm counts and an independent Jacobi control. Repeat-run identity is the product's own output and only guards against change |
| OUT-003 | OBJ-003 SOW-081 | REQ-004 | AC-004 | VER-004 | Outside source: the same references and exact counts. Agreement with the dense path is the product's own output and only guards against change |
| OUT-004 | OBJ-003 SOW-081 | REQ-005 | AC-005 | VER-005 | Outside source: an independent Python verifier that imports no producer code, recomputing from the published document and the bound request. The committed documents are the product's own earlier output and only guard against change |
| OUT-005 | OBJ-003 SOW-081 | REQ-006 CLM-002 CLM-003 | AC-006 | VER-006 | Outside source: the discrete and oblique references for state-case, contents, skew and snubber models; the refusals and disclosures are those REQ-006 names. Frequencies equal to those of the matching linear model compare the product with itself: no independent grounds |
| OUT-005 | OBJ-003 SOW-081 | REQ-007 | AC-007 | VER-007 | The criterion states the expected refusals from its own formulas; pricing within the registered memory profile is examined by the actual-source pricing audit (DEL-04-08/REQ-004) |
| OUT-005 | OBJ-003 SOW-081 | REQ-008 | AC-008 | VER-008 | PRD §5.5 for the absence of a pre-set bound; the criterion states the expected behaviour of the surfaces |
| OUT-005 | OBJ-003 SOW-081 | REQ-009 | AC-009 | VER-009 | Judgment of the texts against PRD §21.2 and `docs/claims_registry.md`; provenance fields against `docs/IP_AND_DATA_BOUNDARY.md` §4; each reference regenerated from a generator that imports no product code |
| OUT-005 | OBJ-003 SOW-081 | REQ-010 | AC-010 | VER-010 | Outside source: a published piping benchmark, transcribed with its own redistribution evidence; comparison evidence, not exact verification |
| OUT-005 | OBJ-003 SOW-081 | REQ-011 REQ-006 | AC-011 | VER-011 | Release target exit criterion 1 lists the families; admission is compared with the exact route's static admission, which this deliverable does not write. Frequency correctness rests on AC-003 and AC-006 |
