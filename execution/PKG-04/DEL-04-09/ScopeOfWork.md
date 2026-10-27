---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-09
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:9224abe5374619c26509ffaebcabfdc227291caecb8f28b7bbfe59654f3c4762
project_scope_refs: [SOW-081]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-09 Modal analysis

**Basis:** decomposition revision 0.16 (SOW-081, OBJ-003), PRD FR-SOL-016 and §12.9, and SCA-013 (`execution/_ScopeChange/SCA-013_2026-10-27_MODAL_ANALYSIS/DECISION.md`). Agent 0 (HELP_HUMAN) decided this scope on 2026-10-27 under the owner's delegation in `loop/LOOP_INIT.md`. These commitments and acceptance criteria are pending an independent review of their text. Nothing in them is built. They assert no completion, acceptance or release. The governing design is [Design/modal-analysis.md](Design/modal-analysis.md). Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (claims taxonomy per DEC-081).

## Purpose and Objective Traceability

This deliverable computes the natural frequencies and mode shapes of the linear pressure-free piping model, on a mass basis identical to its self-weight basis. It checks each extracted set for accuracy and completeness, and refuses by name what it does not admit. It serves OBJ-003 (a robust global centerline/frame solver) through SOW-081.

- **OUT-001** — The mass model: consistent mass on straight members, rigid-body lumps on realized arcs and component bodies, on the self-weight basis, with its weight-basis identity check (REQ-001).
- **OUT-002** — The modal request: its required extraction bound, modulus basis and weight-to-mass acceleration (REQ-002).
- **OUT-003** — The eigen-solution paths, dense and sparse, and their integrity checks (REQ-003, REQ-004).
- **OUT-004** — The modal results: the profile, the semantic table, the analysis-run record and the separate readers (REQ-005).
- **OUT-005** — The admitted domain, the resource limits, the desktop and report surfaces, and the claim boundary (REQ-006 to REQ-009).

## Deliverable Definition — Ontology

### CLM-001 — The method

F1 solves the generalized eigenproblem K_ff φ = ω² M_ff φ on the free degrees of freedom of the linear pressure-free (PF) model.
- K_ff is the restraint-reduced stiffness of the PF static solve, without load-dependent terms.
- M_ff is the restraint-reduced mass of the mass model (REQ-001).

For each mode it publishes the frequency, circular frequency and period; the mass-normalized shape; and the translational participation factors, effective masses and effective-mass fractions. For each result it publishes the model's mass by category and the free-DOF mass. Response-spectrum, time-history and harmonic analysis are not part of this deliverable (SCA-013).

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:**
- the mass model and its weight-basis identity check;
- the modal request, its refusals and disclosures;
- the eigen-solution crate `core/solver/modal_eigen` (dense reduction, tridiagonal QL, shift-invert Lanczos, the Sturm count, residual bounds and participation);
- the product module `core/product_physics/src/modal.rs` and the entry `run_modal_preview_with_mode`;
- the modal profile, the `modal-1` semantic table and the modal analysis-run kind;
- the Rust modal reader and its test-only Python mirror;
- the frozen references under `validation/references/f1_modal/`;
- the desktop modes view, the mode-shape view and the report's modal section, within the results viewer's and report generator's contracts.

**Declared inputs** (`deliverable.yaml`):
- DEL-04-01: kernel assembly, reduction, the curved-bend element, `sparse_direct` and K-D5;
- DEL-04-02: the straight pipe element and its uniform-load vector;
- DEL-04-03: linear supports and imposed-displacement DOFs;
- DEL-04-06: singular-model detection;
- DEL-04-07: product solve integration;
- DEL-05-01: the self-weight generator and its mass per length;
- DEL-03-05: rigid-family components;
- DEL-04-08: the retained-memory pricing law;
- DEL-02-02: units;
- DEL-00-06: the diagnostics and result-envelope contract;
- DEL-04-05, DEL-05-03, DEL-10-05: non-gating;
- the NUREG/CR reports, as external sources.

These deliverables keep their scope. The code paths in `deliverable.yaml` indicate relevance only; most are planned and not yet present.

### CLM-003 — Current reach and sequence

- **Built:** nothing.
- **Sequence** (Agent 0, 2026-10-27):
  1. M1a, mass;
  2. M1b, the dense path;
  3. M2, the sparse path, with Sturm and inertia certification, where "the unpivoted-LDLᵀ inertia risk" is "a first-class test target";
  4. M3, UI.
- **Start and queue.** The build starts after PF Timoshenko (#1663) is in review. D3 runs join the queue on Agent 0's go.
- **Later slices, outside this deliverable's current commitments:** M4 (Timoshenko mass), M5 (nonlinear supports linearized at a named state) and M6 (the mass-point helper), and a concentrated-mass field (W-17).

## Completion and Reliance Basis — Epistemology

Nothing in these commitments bounds modelling, input or physical error. The integrity checks concern the discrete problem the method forms.

### Mass

- **REQ-001** — *The mass basis is the self-weight basis.*
  - **Straight members** carry the Hermitian consistent mass, with linear axial and torsional mass and Rayleigh rotary inertia. Their mass per length is exactly the self-weight generator's PF mass per length (`stable_mass_per_length`: metal, contents and insulation).
  - **Inertia terms.** Contents are in all three translations and excluded from torsional and rotary inertia, which is disclosed.
  - **Realized arcs** carry two rigid-body half-arc lumps with the half-arcs' exact mass, centroid and second moments.
  - **Components.** An eligible rigid-family component carries a rigid-body lump of mass W/a at its centre of gravity, in place of the member's pipe mass. Eligibility is decided by the same function self-weight uses, called once per member. A component that self-weight does not apply keeps the pipe mass, with the same warning.
  - **Massless families.** Objective connectors, tie rods and constant-effort supports carry no mass, which is disclosed.
  - **Restrained DOFs.** Mass at restrained DOFs is excluded from M_ff and disclosed with the total model mass.
  - **No defaulted mass.** No density, mass or acceleration is defaulted; a missing required input refuses by name.
  - **Weight-basis identity.** Every result runs a blocking check, the regression "g·M·1 = self-weight" (Agent 0, 2026-10-27). For each global direction k, a·M·r_k on the full mass is compared with the self-weight generator's loads at gravity a along k over all members. Straight members are compared nodally, against the consistent uniform-load vector. Every member and component is compared on the resultant force and moment. The comparison is within a tolerance measured under DEC-026 and only tightened. A failure refuses with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`, naming the member or component.
- **AC-001** — On the frozen `f1_modal` mass references, for straights, arcs, an eligible component, a component with an offset centre of gravity, an ineligible component and the massless families:
  - the product mass matrices, totals by category and free-DOF masses agree with the references within their recorded tolerances;
  - the identity check passes on each reference model;
  - a mutated mass (a changed density, a dropped component, a component on the wrong node) is refused with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`.

### Request

- **REQ-002** — *Required inputs, no defaults.*
  - **Extraction bound.** It is a mode count, a frequency cut-off or both, and at least one is required. Extraction stops at whichever is reached first, and the result states which. The engine and the headless runner supply no value, and no code-derived cut-off ships. A request without one is refused (`MODAL_EXTRACTION_BOUND_REQUIRED`).
  - **Modulus basis.** It uses a load case's vocabulary and the existing resolver.
  - **Weight-to-mass acceleration.** `weight_to_mass_acceleration` is a positive acceleration, normalized through the unit catalog.
    - It is required when a component's weight enters the mass; without it the request is refused (`MODAL_WEIGHT_ACCELERATION_REQUIRED`, naming the components).
    - When the model carries a current applied component-weight self-weight primitive, the magnitude of that primitive's recorded acceleration must equal a, bit for bit after normalization. Otherwise the request is refused (`MODAL_WEIGHT_ACCELERATION_MISMATCH`, naming the case and both values).
  - **Recording.** The request is recorded with the result and in the analysis-run record.
- **AC-002** — Maintained tests show:
  - each refusal for a request without a bound, without a required acceleration, or with a mismatched acceleration;
  - extraction stopping at the count, at the cut-off, and at whichever comes first when both are given, with the stopping bound stated;
  - no default supplied on any entry: product, headless and desktop.

### Solution and integrity

- **REQ-003** — *Dense path and integrity checks.*
  - **Method.** The dense path reduces with K's Cholesky factor, tridiagonalizes and applies implicit QL, with no new numerical dependency.
  - **Checks.** Every published mode set carries:
    - (1) the formation check of M, re-formed at 128 bits from the authored operands, beside K's K-D5 check;
    - (2) each mode's residual bound on its eigenvalue;
    - (3) the mass-orthogonality deviation of the published modes;
    - (4) the Sturm count at a shift above the largest reported eigenvalue (or at the cut-off), which must equal the number of modes reported or below the cut-off;
    - (5) cluster identifiers for repeated or nearly repeated eigenvalues, with the extraction boundary moved to include a whole cluster;
    - (6) the weight-basis identity (REQ-001).
  - **Standing.**
    - A formation or residual breach that is not a completeness breach publishes the result as Sensitive.
    - A Sturm mismatch refuses the set (`MODAL_STURM_COUNT_MISMATCH`), as do a weight-basis or acceleration mismatch and a non-positive-definite K.
  - **Tolerances.** They are measured and recorded under DEC-026, and only tightened.
  - **Determinism.** The output is deterministic for a fixed model, request, solver version and settings.
- **AC-003** — On the frozen `f1_modal` references, the dense path:
  - reproduces the class (a) discrete references at their recorded tolerance (seeded at 1e-9 relative for frequencies), and converges to the class (b) continuum references at the expected rate;
  - compares mode shapes and clusters by MAC or subspace angle;
  - agrees with the independent Jacobi control in tests;
  - passes each check (1) to (6) on the references;
  - refuses or demotes each committed mutation at its named check;
  - publishes byte-identical results on repeated runs.
- **REQ-004** — *Sparse path with Sturm and inertia certification.*
  - **Method.** The sparse path uses shift-invert Lanczos with full mass-reorthogonalization, a fixed start vector and fixed operation order. It works on the existing `sparse_direct` factorization, through its public API, with an inertia-preserving diagonal scaling.
  - **Cut-off.** In cut-off mode it slices the spectrum until the Sturm count at the cut-off is exhausted.
  - **Unpivoted-LDLᵀ inertia risk.** It records pivot growth, places shifts away from computed eigenvalues, and retries at a perturbed shift on excess growth. It refuses a set whose count it cannot confirm, and never publishes an unchecked count.
  - **Checks.** The REQ-003 checks apply unchanged.
- **AC-004** — "The unpivoted-LDLᵀ inertia risk" is "a first-class test target":
  - maintained tests build models that provoke pivot growth (near-coincident eigenvalues, clusters, rigid-family scales from 1e3 to 1e6, shifts close to eigenvalues);
  - for each, the sparse path's eigenvalues and Sturm counts agree with the dense path and an independent count, within the dense guard, and the growth monitor's verdict is asserted;
  - the sparse path reproduces the AC-003 references;
  - scale benchmarks are timed under `t3_exclusive.sh` with their build named.

### Results and readers

- **REQ-005** — *Modal results and separate readers.*
  - **Publication.** Results are published under the profile `product_preview_modal_v1`, with its fixed limitation text, and the semantic table `openpipestress.result_semantics/0.3.0/modal-1`. Analysis runs record the modal run kind with the request.
  - **Units and signs.** Frequencies are in Hz, circular frequencies in rad/s and periods in s. Shapes are mass-normalized in global axes, with restrained DOFs exactly 0, under a deterministic sign convention. Participation factors, effective masses and fractions are in kilograms.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no code with the producer each check:
    - the profile and limitation against the request;
    - the total and free-DOF masses, rederived from the model;
    - that the effective masses do not exceed the free-DOF mass in any direction;
    - the counts, ordering, sign convention and cluster structure.
    They do not re-run the eigen-solve.
- **AC-005** — The two readers return the same verdict for every committed modal result and every committed mutation of the profile, limitation, masses, counts, ordering, sign or clusters. A verifier-independence test shows that the Python verifier imports no producer code.

### Domain, limits and surfaces

- **REQ-006** — *Admitted, disclosed and refused.*
  - **Admitted:** straight members and tee legs; realized arcs; geometry-only bends as their straight members; rigid-family components; linear supports and springs; constant-effort supports, objective connectors and tie rods, disclosed as massless and, for constant-effort supports, as stiffness-free; annotation-only expansion joints as pipe; imposed-displacement DOFs, kept restrained.
  - **Ignored and disclosed:** thermal, pressure-region, wind, seismic, nodal and member loads.
  - **Refused by name:**
    - nonlinear supports (`MODAL_NONLINEAR_SUPPORT_UNSUPPORTED`);
    - the exact route and load/reference-state documents (`MODAL_EXACT_ROUTE_UNSUPPORTED`);
    - Timoshenko straights until M4 (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`);
    - an unrestrained or mechanism model, through the existing singularity refusal.
  - **Warning.** A member that is too coarsely discretized for the highest reported frequency is warned (`MODAL_MASS_DISCRETIZATION_COARSE`). Its factor is derived by benchmark and stated in the warning.
- **AC-006** — Each refusal and disclosure in REQ-006 is exercised by a maintained test that asserts its code and named object. The discretization warning's factor is derived from a recorded benchmark.
- **REQ-007** — *Resource limits and the retained route.*
  - **Guards.** Before allocation, the dense path applies a memory guard (its live n² buffers against the provisional 6 GiB ceiling) and a work guard (9n³ against a budget measured in M1b). The sparse path applies a memory guard (8·(3P + n·q) bytes). Each refusal states its formula. A guard refuses; it never truncates the extraction.
  - **Retained route.** F1 never enters the retained Direct entry or D1.
  - **Pricing.** Every change to a priced source or a reviewed input is re-registered under the retained pricing law (D3) in the change that makes it, and the AC-004 stack witnesses stay passing.
- **AC-007** — Maintained tests show that:
  - each guard refuses with its formula before allocation, and admits at its limit;
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
  - **Product texts.** They state the limit "numerical integrity does not establish physical correctness" and use the claims registry's vocabulary (`docs/claims_registry.md`). No product text states or implies a claim that PRD §21.2 prohibits.
  - **Reference independence.** References are derived and frozen by a TASK that reads no product code, and refuted by a second TASK before any product comparison.
  - **NUREG/CR transcriptions.** A transcribed NUREG/CR problem is committed only with its per-reference provenance record (`docs/IP_AND_DATA_BOUNDARY.md` §4; Design §4.4). Copyrighted or adapted content inside a report, such as figures adapted from journals, is not transcribed.
  - **Other sources.** No CAEPIPE output, licensed benchmark data or paywalled text is committed.
- **AC-009** —
  - The modal product texts are pinned by maintained tests and reviewed against `docs/claims_registry.md`, and the claims-language lint (GEN-13) passes over the desktop texts.
  - Each committed reference package has its refutation record.
  - Each transcribed NUREG/CR fixture has a complete provenance record listing what was excluded.

## Production and Verification Method — Praxeology

The checks named here are planned; they do not exist yet. Each slice registers its checks in `software-workflow.json` and names them with their head in its pull request.

- **VER-001** — For REQ-001: Rust tests of `modal.rs` against the frozen `f1_modal` mass references, the identity check's pass and mutation cases, and the shared-eligibility tests (M1a).
- **VER-002** — For REQ-002: refusal and stopping-rule tests at the product facade and the headless runner (M1a, M1b).
- **VER-003** — For REQ-003: `modal_eigen` unit tests, including the Jacobi control. They are joined by product tests against the class (a) and (b) references, mutation tests for each integrity check, and a determinism test (M1b).
- **VER-004** — For REQ-004: the inertia-risk suite, dense-against-sparse agreement within the dense guard, and scale benchmarks under `t3_exclusive.sh` (M2).
- **VER-005** — For REQ-005: Rust reader tests, the Python verifier with its independence test, and the shared mutation corpus (M1b).
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
  - a check that every NUREG/CR fixture has its provenance record.

Each slice also gets the independent review that the Root `AGENTS.md` requires for numerical changes.

## Governing Values and Decisions — Axiology

- **AX-001** — *No silent engineering defaults.* The extraction bound, the acceleration and every mass input are user-entered or refused (PRD §5.5; Agent 0, 2026-10-27).
- **AX-002** — *One mass basis with self-weight.* The mass model reads self-weight's own mass per length and eligibility function, so the two cannot drift. The product's second, seismic mass basis is a recorded finding outside F1 (`docs/COMPLETENESS_MAP.md` §4.3).
- **AX-003** — *No new numerical dependency* (DEC-023). Both paths are in-house, std only.
- **AX-004** — *No relaxation.* A check is never loosened to obtain a pass. Tolerances are measured under DEC-026 and only tightened. Frozen references are never altered to make an implementation pass.
- **AX-005** — *Independent scrutiny.* Numerical changes get targeted regression checks and independent review by a separate agent (Root `AGENTS.md`).
- **AX-006** — *Scope.* Modal analysis only. Response spectrum and time history stay deferred under DEC-078; any further dynamics scope needs its own scope change.
- **AX-007** — *Held by the owner:* release, acceptance and issuance, risk acceptance and holds (F-PIP-3). This deliverable activates nothing beyond its own entry.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-081 | REQ-001 CLM-001 | AC-001 | VER-001 | Mass and identity tests against the frozen mass references, with mutations, on the candidate head |
| OUT-002 | OBJ-003 SOW-081 | REQ-002 | AC-002 | VER-002 | Refusal and stopping-rule tests on every entry |
| OUT-003 | OBJ-003 SOW-081 | REQ-003 REQ-004 | AC-003 AC-004 | VER-003 VER-004 | Reference, mutation, determinism, inertia-risk and dense-sparse agreement tests; timed scale runs |
| OUT-004 | OBJ-003 SOW-081 | REQ-005 | AC-005 | VER-005 | Matching Rust and Python verdicts on the committed corpus; independence test |
| OUT-005 | OBJ-003 SOW-081 | REQ-006 REQ-007 REQ-008 REQ-009 CLM-002 CLM-003 | AC-006 AC-007 AC-008 AC-009 | VER-006 VER-007 VER-008 VER-009 | Refusal and guard tests, D3 and stack witnesses on priced heads, desktop tests and native smoke, pinned texts, claims lint, refutation and provenance records |
