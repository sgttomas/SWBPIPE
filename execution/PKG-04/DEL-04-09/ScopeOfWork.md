---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-09
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:578fe4139372fb68bafffddcf198a4c2f12594d60c18a3ff65aae025b084d393
project_scope_refs: [SOW-081]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-09 Modal analysis

**Basis:** decomposition revision 0.16 (SOW-081, OBJ-003), PRD FR-SOL-016 and §12.9, and SCA-013 (`execution/_ScopeChange/SCA-013_2026-10-27_MODAL_ANALYSIS/DECISION.md`). Agent 0 (HELP_HUMAN) decided this scope on 2026-10-27 under the owner's delegation in `loop/LOOP_INIT.md`, and amended it on 2026-10-31 for v1.0's exit criterion 1 (`docs/RELEASE_TARGET_v1.0.md` RT-A7): the exact route on its own mass basis, nonlinear supports linearized at a named solved case, and snubbers in the modal basis when requested. These commitments and acceptance criteria are pending an independent review of their text. Nothing in them is built. They assert no completion, acceptance or release. The governing design is [Work/modal-analysis.md](Work/modal-analysis.md). Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (claims taxonomy per DEC-081).

## Purpose and Objective Traceability

This deliverable computes the natural frequencies and mode shapes of the linear piping model, on the pressure-free (PF) and exact pressure (EX) routes, on a mass basis identical to the route's self-weight basis. It checks each extracted set for accuracy and completeness, and refuses by name what it does not admit. It serves OBJ-003 (a robust global centerline/frame solver) through SOW-081.

- **OUT-001** — The mass model: consistent mass on straight members, rigid-body lumps on realized arcs and component bodies, on the self-weight basis, with its weight-basis identity check (REQ-001).
- **OUT-002** — The modal request: its required extraction bound, modulus basis and weight-to-mass acceleration (REQ-002).
- **OUT-003** — The eigen-solution paths, dense and sparse, and their integrity checks (REQ-003, REQ-004).
- **OUT-004** — The modal results: the profile, the semantic table, the analysis-run record and the separate readers (REQ-005).
- **OUT-005** — The admitted domain, the resource limits, the desktop and report surfaces, the claim boundary and the published-benchmark comparison (REQ-006 to REQ-010).

## Deliverable Definition — Ontology

### CLM-001 — The method

F1 solves the generalized eigenproblem K_ff φ = ω² M_ff φ on the free degrees of freedom of the linear model, on the PF route or the EX route.
- K_ff is the restraint-reduced stiffness of the route's static solve, without load-dependent terms (EX adds none: pressure enters only as loads). Nonlinear supports enter at the state of a named solved case (REQ-006), and dynamic-only restraints when the request includes them.
- M_ff is the restraint-reduced mass of the mass model (REQ-001).

For each mode it publishes the frequency, circular frequency and period; the shape, normalized to unit maximum translation, or, for a mode with no translation such as pure torsion, to unit maximum rotation (REQ-005), and its generalized mass; and the translational participation factors, effective masses and effective-mass fractions, with their cumulative sums. For each result it publishes the model's mass by category and the free-DOF mass. Response-spectrum analysis is DEL-04-10's, on this deliverable's modal basis (SCA-014); time-history and harmonic analysis are not part of either (DEC-078).

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:**
- the mass model and its weight-basis identity check;
- the modal request, its refusals and disclosures;
- the eigen-solution crate `core/solver/modal_eigen` (dense reduction, tridiagonal QL, shift-invert block Lanczos, the Sturm count with its shift placement and growth limit, the modal formation check, residual estimates and participation);
- the product module `core/product_physics/src/modal.rs` and the entry `run_modal_preview_with_mode`;
- the modal profile, the `modal-1` semantic table and the modal analysis-run kind;
- the Rust modal reader and its test-only Python mirror;
- the frozen references under `validation/references/f1_modal/`;
- the desktop modes view, the mode-shape view and the report's modal section, within the results viewer's and report generator's contracts.

**Declared inputs** (`deliverable.yaml`):
- DEL-04-01: kernel assembly, reduction, the curved-bend element, `sparse_direct` with one read-only observation accessor (M1b), and K-D5's element re-formation through one read-only entry (M1b);
- DEL-04-02: the straight pipe element and its uniform-load vector;
- DEL-04-03: linear supports and imposed-displacement DOFs;
- DEL-04-04: the converged support states of a solved case, read for the linearization of nonlinear supports;
- DEL-02-01: the snubber support family's document shape (model document 0.9.0);
- DEL-04-06: singular-model detection;
- DEL-04-07: product solve integration;
- DEL-05-01: the self-weight generator and its mass per length on both routes;
- DEL-03-05: rigid-family components;
- DEL-04-08: the retained-memory pricing law;
- DEL-02-02: units, including the unit families for modal quantities (frequency, angular frequency, rotation per length, length per rotation, rotational generalized mass in kg·m²·rad⁻²; carried by model document 0.9.0's S1), before M1b;
- DEL-00-06: the diagnostics and result-envelope contract;
- DEL-04-05, DEL-05-03, DEL-10-05: non-gating;
- published piping benchmarks, such as the NUREG/CR reports, as external sources, each only with its own redistribution evidence (REQ-009).

These deliverables keep their scope. The two read-only seams are additions to DEL-04-01's code, made by F1's slices. The code paths in `deliverable.yaml` indicate relevance only; most are planned and not yet present. With these declarations the deliverable graph's one cycle has 11 members, DEL-04-09 and DEL-04-05 among them (Design §6).

### CLM-003 — Current reach and sequence

- **Built:** nothing.
- **Sequence** (Agent 0, 2026-10-27):
  1. M1a, mass on both routes;
  2. M1b, the dense path, with every integrity check of REQ-003, including the Sturm count's shift placement, its growth limit and the `sparse_direct` observation accessor, and the linearization of nonlinear supports at a named case (formerly M5);
  3. M3, UI;
  4. M2, the sparse path (Agent 0, 2026-10-31: after the views and before response-spectrum results, reordered if the dense guard bites earlier).

  "The unpivoted-LDLᵀ inertia risk" is "a first-class test target" from M1b, whose counts first use it, through M2. The count infrastructure moved from M2 into M1b after Codex round 2, so that M1b can publish dense results with the full checks.
- **Start and queue.** PF Timoshenko (#1663) has landed. D3 runs join the queue on Agent 0's go. On EX, the families that EX itself admits only after RT-A3 (DEL-04-07) are admitted by modal when RT-A3 admits them.
- **FR-SOL-016 stays partial** until REQ-010's published-benchmark comparison is recorded, whatever the state of M1a to M3.
- **Later slices, outside this deliverable's current commitments:** M4 (Timoshenko mass, on either route) and M6 (the mass-point helper), and a concentrated-mass field (W-17).

## Completion and Reliance Basis — Epistemology

Nothing in these commitments bounds modelling, input or physical error. The integrity checks concern the discrete problem the method forms. They are numerical-integrity checks with stated premises, not certified enclosures: no commitment here is a mathematical bound on an eigenvalue.

### Mass

- **REQ-001** — *The mass basis is the self-weight basis.*
  - **Straight members** carry the Hermitian consistent mass, with linear axial and torsional mass and Rayleigh rotary inertia. Their mass per length is exactly the self-weight generator's mass per length on the model's route (`stable_mass_per_length`): on PF, `source_od_effective_wall_areas/v1` (metal and contents on the wall t − c_m); on EX, `source_od_nominal_wall_pressure_bore_areas/v1` (metal on the nominal wall, contents on the pressure bore); insulation alike on both.
  - **Inertia terms.** Contents are in all three translations and excluded from torsional and rotary inertia, which is disclosed.
  - **Realized arcs** carry two rigid-body half-arc lumps with the half-arcs' exact mass, centroid and second moments.
  - **Components.** An eligible rigid-family component carries a rigid-body lump of mass W/a at its centre of gravity, in place of the member's pipe mass. Eligibility is decided by the same function self-weight uses, called once per member. A component that self-weight does not apply keeps the pipe mass, with the same warning.
  - **Mass-bearing members.** Mass is formed on every member except an objective connector's replaced span, which carries neither self-weight nor mass; one shared selection serves mass assembly and the identity check (Design §2.5).
  - **Range.** W and a must be positive normal binary64 numbers, fl(W/a) a normal binary64 number, and a·fl(W/a) finite and normal, and every entry of the component's spatial mass block (m·r_i, m·r_i·r_j) finite and either zero or normal. Otherwise the request is refused by range (`MODAL_COMPONENT_MASS_OUT_OF_RANGE`, naming the component). Under that premise a·fl(W/a) differs from W by at most 2u + u² relative, u = 2⁻⁵³.
  - **Massless families.** Objective connectors, tie rods and constant-effort supports carry no mass, which is disclosed.
  - **Restrained DOFs.** Mass at restrained DOFs is excluded from M_ff and disclosed with the total model mass.
  - **No defaulted mass.** No density, mass or acceleration is defaulted; a missing required input refuses by name.
  - **Weight-basis identity.** Every result runs a blocking check, the regression "g·M·1 = self-weight" (Agent 0, 2026-10-27). For each global direction k, a·M·r_k on the full mass is compared with the self-weight generator's loads at gravity a along k over all members, on the model's route. The identity holds nodally for straight members, against the consistent uniform-load vector, and on the resultant force and moment for arcs and components, each within a measured tolerance; it is not literal vector equality for every family. The tolerance is measured in M1a on the frozen references, then pinned as a relative and absolute pair and only tightened (DEC-026); until then no modal result is published. A failure refuses with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`, naming the member or component.
- **AC-001** — On the frozen `f1_modal` mass references, on both routes' mass bases, for straights, arcs, an eligible component, a component with an offset centre of gravity, an ineligible component and the massless families:
  - the product mass matrices, totals by category and free-DOF masses agree with the references within their recorded tolerances;
  - the identity check passes on each reference model, within the tolerance pinned in M1a, whose measurement is recorded;
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
    - **Whole spectrum:** if the reported set is the whole finite spectrum, the final shift lies a stated relative offset above its largest eigenvalue, and the separation applies on the lower side only (Design §3.4).
  - **Cut-off sizing.** On the sparse path, a cut-off-only request is sized by a count at a stated relative offset above f_c. That count only sizes the extraction and its allocation; it is neither the completeness count nor a termination rule. When the cut-off controls stopping, extraction continues until a completed computed cluster lies above f_c. If the factorization there is singular or rejected by the growth monitor, it is retried at a second, documented offset, which the result records; the offset never changes silently.
  - **Modulus basis.** It uses a load case's vocabulary and the existing resolver.
  - **Linearization case.** When the model has a nonlinear support, the request names a load case whose converged support states linearize it (REQ-006); the modal entry solves that case on the current model in the same run, reusing no stored result. It has no default; a request without it is refused (`MODAL_LINEARIZATION_CASE_REQUIRED`, naming the supports).
  - **Dynamic-only restraints.** When the model has a snubber, the request states whether dynamic-only restraints are included. It has no default; a request without the choice is refused (`MODAL_DYNAMIC_RESTRAINT_CHOICE_REQUIRED`). A response-spectrum solve includes them (DEL-04-10).
  - **Weight-to-mass acceleration.** `weight_to_mass_acceleration` is a positive acceleration, normalized through the unit catalog.
    - It is required when a component's weight enters the mass; without it the request is refused (`MODAL_WEIGHT_ACCELERATION_REQUIRED`, naming the components).
    - When the model carries a current applied component-weight self-weight primitive, the magnitude of that primitive's recorded acceleration must equal a, bit for bit after normalization. Otherwise the request is refused (`MODAL_WEIGHT_ACCELERATION_MISMATCH`, naming the case and both values).
  - **Recording.** The request is recorded with the result and in the analysis-run record.
- **AC-002** — Maintained tests show:
  - each refusal for a request without a bound, without a required acceleration, with a mismatched acceleration, without a required linearization case or without a required dynamic-restraint choice;
  - extraction stopping at the count, at the cut-off, and at whichever comes first when both are given, with the stopping bound stated;
  - a mode exactly at the cut-off included, with the completeness count taken at the final shift above it;
  - on the sparse path, the second sizing offset used and recorded when the first is singular, and an underestimated sizing count leading to further extraction, never to an omitted mode below f_c;
  - a boundary inside a cluster extending the set to the whole cluster, disclosed by count, for both the count and the cut-off;
  - a cut-off below the lowest frequency reporting no modes, with a zero count taken in the gap above f_c, and a request whose set is the whole finite spectrum, with the final shift above its largest eigenvalue;
  - no default supplied on any entry: product, headless and desktop.

### Solution and integrity

- **REQ-003** — *Dense path and integrity checks.*
  - **Method.** The dense path reduces with K's Cholesky factor, tridiagonalizes and applies implicit QL, with no new numerical dependency. Its Sturm counts factor the scaled K − σM with `sparse_direct`'s LDLᵀ, observed through one read-only accessor added to `sparse_direct` in M1b, with no mutation and no allocation; that change re-registers D3.
  - **What the checks are.** They are numerical-integrity checks with stated premises. They are not certified enclosures, and no eigenvalue bound is certified. The result's limitation text states what they do and do not establish (REQ-009).
  - **Checks.** Every published mode set carries:
    - (1) a modal formation check: K and M re-formed independently from their authored operands, element by element, in at least double-double precision (≈106-bit significand), the section properties re-derived from OD and wall (with EX's deductions), and the formation difference (formed minus re-formed, on the assembled matrices) measured as a norm and published, with the trusted leaves stated (Design §3.4);
    - (2) per mode, the residual ‖Kφ − λMφ‖ evaluated against the re-formed pencil, scaled as a backward error and compared with a stated threshold, together with a residual-based error estimate of the eigenvalue. The estimate is first-order, under the premise that the eigenvalue is isolated, and is stated for a cluster as a whole; it is an estimate, not a bound;
    - (3) the mass-orthogonality deviation of the published modes, normalized by their generalized masses;
    - (4) Sturm-count agreement: the count at the final shift (REQ-002) must equal the number of modes reported, on the interval actually reported. Counts are taken only at shifts placed in gaps between computed clusters. A count is relied on only under two premises, on the scaled matrices: the **conditioning premise**, that the factorization's componentwise backward-error estimate e_b = γ_n·‖|L̂||D̂||L̂ᵀ|‖ times an estimate of ‖(K̃ − σM̃)⁻¹‖ is at most a stated fraction, so that the backward error is small against the shifted matrix's distance from singularity; and the **separation premise**, that the shift's separation from the neighbouring computed eigenvalues exceeds a stated multiple of e_b. Both are disclosed with the shift, the estimates and the growth factor g;
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
    - K-D5's static criterion is not reused; only its element re-formation is, through a read-only entry.
  - **Tolerances.** The tolerances, thresholds, growth limit and separation multiple are measured and recorded under DEC-026, and only tightened.
  - **Determinism.** The output is deterministic for a fixed model, request, solver version and settings.
- **AC-003** — On the frozen `f1_modal` references, the dense path:
  - reproduces the class (a) discrete references at their recorded tolerance (seeded at 1e-9 relative for frequencies), and converges to the class (b) continuum references at the expected rate;
  - compares mode shapes and clusters by MAC or subspace angle;
  - agrees with the independent Jacobi control in tests;
  - passes each check (1) to (6) on the references, with its Sturm counts equal to the reference generator's exact counts;
  - refuses or demotes each committed mutation at its named check, including a formed K or M perturbed away from its re-formation;
  - detects a deliberately missed eigenvalue (K = I, M = diag(1, 2, 3), one direction withheld) by the Sturm count, and refuses the set;
  - refuses, by name, a shift that cannot be separated, including Codex's round-2 witness (a 32×32 tridiagonal on which a shift within 10⁻¹⁷ relative of an eigenvalue rounds away and gives a wrong count with unchanged growth observations), a factorization over the growth limit, and the round-3 witness (K = diag(1, 1, B), M = diag(1/32, 1/256, 8B), B = HHᵀ, H the 40×40 unit lower bidiagonal with subdiagonal 2), on which a widely separated shift gives a wrong count because the shifted matrix is ill-conditioned; tests assert the refusal of every shift that binary64 miscounts, not a particular count;
  - shows, on models that provoke pivot growth (near-coincident eigenvalues, clusters, rigid-family scales from 1e3 to 1e6, shifts close to eigenvalues), that each count used agrees with the exact count, and asserts the growth monitor's verdict and the separation decision;
  - shows the observation accessor leaves the factorization unchanged and allocates nothing;
  - publishes byte-identical results on repeated runs.
- **REQ-004** — *Sparse path.*
  - **Method.** The sparse path uses shift-invert **block** Lanczos with full mass-reorthogonalization, a deterministic start block (block size 2, from the paired bending modes of an axisymmetric section), locking and deflation of converged pairs, breakdown handling and fixed operation order. Block size is an algorithm parameter, not an engineering default. It works on the existing `sparse_direct` factorization, with an inertia-preserving diagonal scaling. It also converges the next cluster above the reported set, unpublished, so that the final shift lies in a gap between computed clusters.
  - **Singular mass.** M is only semidefinite, so every basis vector is kept in the range of (K − σM)⁻¹M, and each Ritz vector is purified by one more application before its checks (Design §3.3).
  - **Missed directions.** The Sturm count detects a missed direction. A deterministic restart with a larger block and a new seed recovers it. If the restarts are exhausted, the set is refused; a count disagreement is never published.
  - **Cut-off.** In cut-off mode it slices the spectrum until a completed computed cluster lies above f_c, with q sized by REQ-002's sizing count. Each slicing shift is placed under REQ-003's premises and growth limit.
  - **Unpivoted-LDLᵀ inertia risk.** It uses M1b's accessor, shift placement and growth limit unchanged. It refuses a set whose count it cannot take under the premise, and never publishes an unchecked count.
  - **Checks.** The REQ-003 checks apply unchanged.
- **AC-004** — "The unpivoted-LDLᵀ inertia risk" is "a first-class test target":
  - maintained tests extend AC-003's growth-provoking models to the sparse path's slicing and final shifts;
  - for each, the sparse path's eigenvalues and Sturm counts agree with the dense path and the exact count, within the dense guard, and the growth monitor's verdict and the separation decision are asserted;
  - the sparse path recovers the correct shapes of a model with a singular lumped mass (the review's 2×2 witness, K = [[2, 1], [1, 2]], M = diag(1, 0)), with purified residuals;
  - the sparse path recovers both modes of each exactly repeated pair (a straight circular cantilever), and a case with a multiplicity above the initial block size shows the Sturm count detecting the missed direction and the restart recovering it, or the set refused when restarts are exhausted;
  - the sparse path reproduces the AC-003 references;
  - scale benchmarks are timed under `t3_exclusive.sh` with their build named.

### Results and readers

- **REQ-005** — *Modal results and separate readers.*
  - **Publication.** Results are published under the profile `product_preview_modal_v1`, with its fixed limitation text, and the semantic table `openpipestress.result_semantics/0.3.0/modal-1`. Analysis runs record the modal run kind with the request.
  - **Units.** Frequencies are in Hz, circular frequencies in rad/s and periods in s. Every unit has integer exponents in the unit catalog.
  - **Normalization: exactly two cases.** Shapes are in global axes, with restrained DOFs exactly 0. Each mode records its case.
    - **(a) A mode with nonzero translation** is normalized to unit maximum translation: the largest translational component is +1, translations are dimensionless and rotations are in rad/m. Its generalized mass m = φᵀM_ffφ is in kg, its participation factors Γ_k = φᵀp_k / m are dimensionless and its effective masses (φᵀp_k)²/m are in kg, where p_k = (M R_k)_f is the uniform-support-motion excitation: the free rows of the full mass times the rigid unit translation R_k of all DOFs, so that consistent-mass coupling to restrained DOFs is included. Effective-mass fractions are relative to T_k = R_kᵀM R_k, the model's total mass in direction k.
    - **(b) A mode with no translation**, such as pure torsion, is normalized to unit maximum rotation: the largest rotational component is +1 (rad/rad), and the translations are the computed vector's own, unaltered (m/rad, at most τ_t·ℓ). Its generalized mass is in kg·m²·rad⁻², and its Γ (rad/m) and effective translational masses (kg) are computed from the published vector, so that Γ²·m = m_eff in kg holds in the catalog's dimensions. The integrity checks of REQ-003 are evaluated on the published vector.
    - **Zero denominators.** If the total mass along k is 0, the effective masses along k are 0 and their fractions are published as undefined, with the reason.
    - **The threshold.** A mode has no translation when its largest translation is at most τ_t·ℓ times its largest rotation, with ℓ the diagonal of the bounding box of all the model's nodes (positive for any model with an element) and τ_t = 10⁻⁶ (confirmed by measurement in M1b and only tightened). There is no participation condition: a case (b) mode's translational participation, which can be large through the mass coupling to restrained DOFs, is published as computed. The result states τ_t and each mode's case.
  - **Effective-mass fractions** and their cumulative sums are dimensionless.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no code with the producer each check:
    - the profile and limitation against the request;
    - the total and free-DOF masses, rederived from the model;
    - that the effective masses, summed over the published modes, do not exceed the total mass T_k in any direction;
    - each mode's normalization case and sign (its normalizing component +1; in case (b), every translation at most τ_t·ℓ times the largest rotation, with Γ and the effective masses recomputed from the published vector), and Γ²·m = m_eff;
    - the counts, ordering, cluster structure and any cluster extension.
    They do not re-run the eigen-solve.
- **AC-005** — The two readers return the same verdict for every committed modal result and every committed mutation of the profile, limitation, masses, normalization, counts, ordering, sign or clusters. The committed results include both normalization cases, among them a pure-torsion mode in case (b). A maintained producer test shows a mode with no free translation but large participation through restrained-DOF coupling (a one-element simply supported span) published in case (b) with that participation. Further producer tests show a case (b) mode published with its computed translations unaltered, its integrity checks evaluated on that published vector, and its Γ and effective masses recomputed from it; and a one-element cantilever, with one free node, classifying its torsional mode by the positive length ℓ. A verifier-independence test shows that the Python verifier imports no producer code.

### Domain, limits and surfaces

- **REQ-006** — *Admitted, disclosed and refused.*
  - **Admitted:** models on the PF route and on the EX route, each on its own section and mass basis, with every family below that the route itself admits; straight members and tee legs; realized arcs; geometry-only bends as their straight members; rigid-family components; linear supports and springs; constant-effort supports, objective connectors and tie rods, disclosed as massless and, for constant-effort supports, as stiffness-free; annotation-only expansion joints as pipe; imposed-displacement DOFs, kept restrained.
  - **Nonlinear supports, linearized and disclosed.** Each nonlinear support takes its final state in the named case, solved in the same run (REQ-002), as the static solver represents it: an active one-way or lift-off support, a closed gap or sticking friction rigidly restrains its DOF; an inactive or open support leaves it free; sliding friction leaves its tangential DOF free with its friction force frozen, contributing no stiffness, which is disclosed as a modal approximation (Design §1.3). A friction support's normal restraint keeps its own state. The result names the case and each support's state. A named case that is absent or whose solve does not converge is refused (`MODAL_LINEARIZATION_CASE_UNAVAILABLE`, naming the case).
  - **Skewed restraints.** A skewed line stop, spring or snubber is reduced by the static path's division-free nullspace method with the authored direction, and admitted exactly when the static path admits it (Design §1.3).
  - **Snubbers.** When dynamic-only restraints are included, each snubber restrains its translational DOFs rigidly or with its stated stiffness; otherwise it is ignored. Either way it is disclosed.
  - **Ignored and disclosed:** thermal, pressure, pressure-region, wind, seismic, nodal and member loads; on EX, the absence of pressure stiffening, as the EX static limitation states.
  - **Refused by name:**
    - load/reference-state documents (`MODAL_LOAD_REFERENCE_STATE_UNSUPPORTED`);
    - Timoshenko straights on either route until M4 (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`);
    - an unrestrained or mechanism model, through the existing singularity refusal.
  - **Warning.** A member that is too coarsely discretized for the highest reported frequency is warned (`MODAL_MASS_DISCRETIZATION_COARSE`). Its factor is derived by benchmark and stated in the warning.
- **AC-006** — Each refusal and disclosure in REQ-006 is exercised by a maintained test that asserts its code and named object, on both routes. Maintained tests show a nonlinear support taking each state of the constraint table from its named case, with frequencies equal to those of the matching linear model; sliding friction disclosed as frozen; a skewed rigid and finite-stiffness restraint matching the oblique references, perpendicular motion free; and a snubber changing the frequencies only when dynamic-only restraints are included. The discretization warning's factor is derived from a recorded benchmark.
- **REQ-007** — *Resource limits and the retained route.*
  - **Guards.** Before allocation, the dense path applies a memory guard (its live n²-sized buffers, the Sturm factorization's included, against the provisional 6 GiB ceiling) and a work guard (9n³ plus n³/3 per candidate Sturm shift, at most a stated number of them, against a budget measured in M1b). The sparse path applies a memory guard on its peak live allocation, 32P + 8n(q + b) + 24q² + c·n bytes: four profile value arrays (K, M, the shifted profile and its clone in the factor), the Lanczos workspace, the projected problem and the per-DOF metadata, with c stated by the implementation. It is rechecked whenever q or b grows. A cut-off-only request is sized from REQ-002's sizing count at the cut-off, taken before Lanczos. Each refusal states its formula. A guard refuses; it never truncates the extraction.
  - **Retained route.** F1 never enters the retained Direct entry or D1.
  - **Pricing.** Every change to a priced source or a reviewed input is re-registered under the retained pricing law (D3) in the change that makes it, and the AC-004 stack witnesses stay passing.
- **AC-007** — Maintained tests show that:
  - each guard refuses with its formula before allocation, and admits at its limit;
  - the sparse guard is rechecked and refuses when q grows past the limit, and a cut-off-only request is sized from its sizing count;
  - the retained Direct entry is unreachable from the modal entry.

  On the head of each change that touches priced sources, the actual-source D3 registration passes and the AC-004 stack witnesses pass.
- **REQ-008** — *Desktop and report (M3).*
  - **Request form.** The desktop takes the modal request with the extraction bound entered by the user. It remembers the last value per project and pre-fills it, visibly.
  - **Results.** The desktop shows a modes table and a mode-shape view. The view is a normalized display transform, labelled as not a physical length, with a static, stepped alternative under reduced motion.
  - **Report.** The rendered report gains a "Modal analysis" section: request and basis, mass summary, modes table, integrity evidence, refusals and disclosures, and limitations.
  - **Contracts.** These surfaces follow the results viewer's and the report generator's output and claims contracts.
- **AC-008** — Desktop tests (Vitest) cover:
  - the request form, including the absence of a pre-set bound in a new project and the remembered value in an existing one;
  - the modes table and its header lines;
  - the mode-shape view, including the reduced-motion path;
  - the report section.

  A native smoke step solves a modal request and shows its table.

### Claim boundary and data

- **REQ-009** — *Claims and references.*
  - **Product texts.** They state the limit "numerical integrity does not establish physical correctness" and use the claims registry's vocabulary (`docs/claims_registry.md`). The profile's fixed limitation text also states what the integrity checks do and do not establish: the residual is evaluated against a double-double re-formation, the eigenvalue error is a first-order estimate, the Sturm count is relied on only at shifts separated from the computed eigenvalues by a stated multiple of the estimated backward error of their factorization, and none of them is a mathematical bound on the eigenvalues or bounds modelling, discretization or input error. No product text states or implies a claim that PRD §21.2 prohibits.
  - **Reference independence.** References are derived and frozen by a TASK that reads no product code, and refuted by a second TASK before any product comparison.
  - **Published benchmarks.** A problem is transcribed from a report, such as a NUREG/CR report, only with that report's own redistribution evidence (its rights statement and NRC distribution terms), recorded per fixture with the `docs/IP_AND_DATA_BOUNDARY.md` §4 provenance fields (Design §4.4), whose `review_status` stays in that section's vocabulary; an independent agent's verification of the transcription against the source is recorded separately and is not acceptance. A report with uncertain rights is held for the owner's review under that policy, and nothing is transcribed from it. Copyrighted or adapted content inside a report, such as figures adapted from journals, is not transcribed. Sources with clear rights are preferred: closed-form benchmarks derived here, and textbook formulas restated with citation (Agent 0, 2026-10-27).
  - **Other sources.** No CAEPIPE output, licensed benchmark data or paywalled text is committed.
- **AC-009** —
  - The modal product texts are pinned by maintained tests and reviewed against `docs/claims_registry.md`, and the claims-language lint (GEN-13) passes over the desktop texts.
  - Each committed reference package has its refutation record.
  - Each transcribed published-benchmark fixture has a complete provenance record, with its report's rights statement and distribution terms, a list of what was excluded and its separate transcription verification; no fixture exists from a held report.

### Published benchmark

- **REQ-010** — *A published piping benchmark.* Before FR-SOL-016 is reported complete, at least one published piping benchmark with recorded redistribution evidence (REQ-009) is compared with the product, after M1b, and the comparison is recorded as comparison evidence, not exact verification. Until a redistributable benchmark exists, FR-SOL-016 stays partial.
- **AC-010** — A maintained test compares the product with at least one committed published piping benchmark, on frequencies and on normalization-invariant participation (effective masses), at a stated comparison tolerance, and its fixture's provenance record is complete. The comparison is labelled comparison evidence.

## Production and Verification Method — Praxeology

The checks named here are planned; they do not exist yet. Each slice registers its checks in `software-workflow.json` and names them with their head in its pull request.

- **VER-001** — For REQ-001: Rust tests of `modal.rs` against the frozen `f1_modal` mass references, the identity check's pass and mutation cases, and the shared-eligibility tests (M1a).
- **VER-002** — For REQ-002: refusal and stopping-rule tests at the product facade and the headless runner (M1a, M1b).
- **VER-003** — For REQ-003: `modal_eigen` unit tests, including the Jacobi control, the missed-eigenvalue case, the separation and growth refusals with Codex's round-2 tridiagonal witness, the dense-path inertia-risk suite against exact counts, and the accessor test. They are joined by product tests against the class (a) and (b) references, mutation tests for each integrity check, and a determinism test (M1b).
- **VER-004** — For REQ-004: the repeated-mode and missed-direction tests, the inertia-risk suite extended to slicing and final shifts, dense-against-sparse agreement within the dense guard, and scale benchmarks under `t3_exclusive.sh` (M2).
- **VER-005** — For REQ-005: Rust reader tests, the Python verifier with its independence test, and the shared mutation corpus, with both normalization cases (M1b).
- **VER-006** — For REQ-006: one maintained test per refusal and disclosure, and the discretization benchmark (M1a).
- **VER-007** — For REQ-007:
  - guard tests (M1b, M2);
  - the actual-source D3 run (`tools/retained_pricing`, `generate --check`) and the AC-004 stack witnesses, on each priced head.
- **VER-008** — For REQ-008: Vitest for the modal panels, overlay and report section, and a native smoke step (M3).
- **VER-009** — For REQ-009:
  - the pinned-text tests;
  - GEN-13 (`tools/validation/validate_claims_language.py`);
  - review of each product text against `docs/claims_registry.md`;
  - each reference package's refutation record;
  - a check that every published-benchmark fixture has its provenance record.
- **VER-010** — For REQ-010: the published-benchmark comparison test and its provenance check (after M1b).

Each slice also gets the independent review that the Root `AGENTS.md` requires for numerical changes.

## Governing Values and Decisions — Axiology

- **AX-001** — *No silent engineering defaults.* The extraction bound, the acceleration and every mass input are user-entered or refused (PRD §5.5; Agent 0, 2026-10-27).
- **AX-002** — *One mass basis with self-weight.* The mass model reads self-weight's own mass per length and eligibility function, so the two cannot drift. The product's second, seismic mass basis is a recorded finding outside F1 (`docs/COMPLETENESS_MAP.md` §4.3).
- **AX-003** — *No new numerical dependency* (DEC-023). Both paths are in-house, std only.
- **AX-004** — *No relaxation.* A check is never loosened to obtain a pass. Tolerances are measured under DEC-026 and only tightened. Frozen references are never altered to make an implementation pass.
- **AX-005** — *Independent scrutiny.* Numerical changes get targeted regression checks and independent review by a separate agent (Root `AGENTS.md`).
- **AX-006** — *Scope.* Modal analysis only. Response spectrum re-entered scope by SCA-014 (owner, 2026-10-28) and is DEL-04-10's, on this deliverable's modal basis; time history stays deferred under DEC-078.
- **AX-007** — *Held by the owner:* release, acceptance and issuance, risk acceptance and holds (F-PIP-3), and the review of any report held for uncertain rights. This deliverable activates nothing beyond its own entry.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-081 | REQ-001 CLM-001 | AC-001 | VER-001 | Mass and identity tests against the frozen mass references, with mutations, on the candidate head |
| OUT-002 | OBJ-003 SOW-081 | REQ-002 | AC-002 | VER-002 | Refusal and stopping-rule tests on every entry |
| OUT-003 | OBJ-003 SOW-081 | REQ-003 REQ-004 | AC-003 AC-004 | VER-003 VER-004 | Reference, mutation, determinism, separation-refusal, inertia-risk and dense-sparse agreement tests; timed scale runs |
| OUT-004 | OBJ-003 SOW-081 | REQ-005 | AC-005 | VER-005 | Matching Rust and Python verdicts on the committed corpus; independence test |
| OUT-005 | OBJ-003 SOW-081 | REQ-006 REQ-007 REQ-008 REQ-009 REQ-010 CLM-002 CLM-003 | AC-006 AC-007 AC-008 AC-009 AC-010 | VER-006 VER-007 VER-008 VER-009 VER-010 | Refusal and guard tests, D3 and stack witnesses on priced heads, desktop tests and native smoke, pinned texts, claims lint, refutation and provenance records, the published-benchmark comparison |
