---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-10
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.17
project_scope_refs: [SOW-082]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-10 Response-spectrum analysis

**Basis:** decomposition revision 0.17 (SOW-082, OBJ-003), PRD §12.9 and §7 item 15, SCA-014 (`execution/_ScopeChange/SCA-014_2026-10-28_RESPONSE_SPECTRUM/DECISION.md`) and `docs/RELEASE_TARGET_v1.0.md` RT-A8. The owner decided SCA-014 on 2026-10-28, choosing OC-3(a): "Must: user-entered spectra, modal and directional combination, a missing-mass correction, and snubbers as a dynamic-only restraint." Agent 0 (HELP_HUMAN) placed it in this new deliverable on 2026-10-31 under the owner's delegation in `loop/LOOP_INIT.md`. These commitments and acceptance criteria are pending an independent review of their text. Nothing in them is built. They assert no completion, acceptance or release. The governing design is [Work/response-spectrum.md](Work/response-spectrum.md). Standard claim fence applies (claims taxonomy per DEC-081).

## Purpose and Objective Traceability

This deliverable computes the seismic response of the linear piping model to uniform support motion by the response-spectrum method, on DEL-04-09's modal basis. It takes every spectrum, damping value and method choice from the user, corrects for the mass the extracted modes do not represent, treats snubbers as restraints in the dynamic analysis only, and refuses by name what it does not admit. It serves OBJ-003 through SOW-082.

- **OUT-001** — The spectrum request: spectra, direction assignments, method choices and the missing-mass input (REQ-001).
- **OUT-002** — The modal responses, the modal and directional combinations and the missing-mass correction (REQ-002 to REQ-004).
- **OUT-003** — Snubbers as dynamic-only restraints (REQ-005).
- **OUT-004** — The spectrum results, their table, the analysis-run record and the separate readers (REQ-006).
- **OUT-005** — The admitted domain, the frozen references and screening tolerance, the desktop and report surfaces, and the claim boundary (REQ-007 to REQ-010).
- **OUT-006** — Independent (multi-level) support motion, at Should (REQ-011).

## Deliverable Definition — Ontology

### CLM-001 — The method

For uniform support motion along global direction k, with spectral acceleration S_k(f) entered by the user and scaled by the user's factor s_k, each mode i of the modal basis (DEL-04-09) has the modal response u_ik = Γ_ik · s_k·S_k(f_i) / ω_i² · φ_i, a displacement vector relative to the supports. Element end forces and moments and support loads are recovered from u_ik by the static recovery of the model's route. The modal responses are combined across modes by the user's modal combination, a missing-mass correction is added for the mass the extracted modes leave out, and the directions are combined by the user's directional combination. Every combined response is an unsigned magnitude.

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:**
- the spectrum request, its refusals and disclosures;
- the modal responses and their recovery, the modal and directional combinations and the missing-mass correction;
- the dynamic meaning of the snubber support family: a restraint in the dynamic analysis, absent from every static solve;
- the product module `core/product_physics/src/spectrum.rs` and its entry;
- the spectrum profile, the `spectrum-1` semantic table and the spectrum analysis-run kind;
- the Rust spectrum reader and its test-only Python mirror;
- the frozen references under `validation/references/f2_spectrum/` and the spectrum screening tolerance;
- the desktop spectrum editor and results view and the report's response-spectrum section, within the results viewer's and report generator's contracts.

**Declared inputs** (`deliverable.yaml`):
- DEL-04-09: the modal basis and the mass model;
- DEL-02-01: the snubber family's document shape (0.9.0);
- DEL-04-03: restraint partition and spring assembly;
- DEL-04-04: nonlinear support states, through DEL-04-09's linearization;
- DEL-04-07: the built model, stiffness and static recovery on both routes;
- DEL-05-02: combination of spectrum results with static cases;
- DEL-02-02: units;
- DEL-00-06: the diagnostics and result-envelope contract;
- DEL-04-08: the retained-memory pricing law;
- DEL-07-05, DEL-08-01, published piping benchmarks: non-gating.

These deliverables keep their scope.

### CLM-003 — Current reach and sequence

- **Built:** nothing.
- **Sequence** (the dynamics lane's plan, approved by Agent 0 on 2026-10-31): after DEL-04-09's dense path and modal views, and after its sparse path, this deliverable's references and screening tolerance come first, then the solve and results, then the desktop and report surfaces.
- **Later, outside these commitments:** time history, harmonic and force-spectrum analysis (DEC-078); stresses from spectrum responses (REQ-006).

## Completion and Reliance Basis — Epistemology

A spectrum result is a numerical statement about the discrete linear model under the user's spectra and method choices. It inherits the modal basis's premises and limitations, and bounds no modelling, input or physical error. The spectrum method is approximate by construction: the combination rules estimate peak responses and are not exact. Nothing here determines code compliance.

### Request

- **REQ-001** — *Required inputs, no defaults.*
  - **Spectra.** Each spectrum is a model-document spectrum record (0.9.0 `spectra[]`, DEL-02-01): a user-entered table of ordinates against frequency or period, at least two points, strictly monotonic in the abscissa, with positive finite ordinates in an acceleration unit (or in g multiples with a user-entered g), normalized through the unit catalog. Its damping ratio is recorded with it. Its interpolation (linear or logarithmic on each axis) is the user's choice and has no default. No spectrum, design code spectrum or library ships.
  - **Directions.** One or more global directions, each with a spectrum and a scale factor; a direction without both is refused.
  - **Methods.** The modal combination (SRSS, CQC, or the grouping method for closely spaced modes, Design §3.2), the directional combination (SRSS or 100-40-40) and, for CQC, the modal damping ratio are required choices with no default.
  - **Missing mass.** Whether to apply the correction is a required choice. When applied, the rigid (zero-period) acceleration per direction and the combination of the correction with the modal part (SRSS or absolute sum) are required inputs.
  - **Modal basis.** The spectrum solve uses a modal request (DEL-04-09) that includes dynamic-only restraints and, if the model has nonlinear supports, names the solved case at which they are linearized.
  - **Range.** A mode whose frequency lies outside a spectrum's tabulated range is refused by name; no spectrum is extrapolated.
  - **Recording.** The request is recorded with the result and in the analysis-run record.
- **AC-001** — Maintained tests show each refusal for a missing spectrum, scale factor, method choice, damping ratio, missing-mass input or modal-basis input; a spectrum with a non-monotonic abscissa or a non-positive ordinate refused; a mode outside the tabulated range refused by name; and no default supplied on any entry: product, headless and desktop.

### Solution

- **REQ-002** — *Modal responses.* Each modal response is formed from the published modal basis by CLM-001, in both of DEL-04-09's normalization cases, and recovered to displacements, rotations, element end forces and moments and support loads on the model's route. Spectral ordinates are interpolated by the spectrum's stated rule. The modal basis's standing is carried: a Sensitive modal set gives a Sensitive spectrum result.
- **REQ-003** — *Combinations.*
  - **Modal combination**, per direction and per response quantity: SRSS; CQC with the Der Kiureghian correlation coefficient at the user's modal damping ratio; or the grouping method for closely spaced modes, with its closeness rule stated in the result. Each is applied to each response quantity separately, never to a combined vector.
  - **Directional combination:** SRSS of the directional responses, or 100-40-40, the largest of the three permutations.
  - Both rules are recorded with the result.
- **REQ-004** — *Missing-mass correction.* When applied, for each direction k, the residual inertia load a_ZPA,k · M (r_k − Σ_i Γ_ik φ_i) over the extracted modes is solved statically on the same stiffness, recovered like a modal response, and combined with the modal part by the user's rule before directional combination. The result states, per direction, the extracted modes' cumulative effective-mass fraction and the missing-mass fraction.
- **AC-002** — On the frozen `f2_spectrum` references, the product reproduces, within the class tolerance set by REQ-008, the single-degree-of-freedom response, a multi-degree-of-freedom model's modal responses and each modal combination, the 100-40-40 and SRSS directional combinations, and the missing-mass correction, including a stiff model whose response equals the static response to its rigid acceleration. Mutation tests show each method choice changing the result as its rule predicts. Results are byte-identical on repeated runs.

### Snubbers

- **REQ-005** — *Dynamic-only restraints.* A snubber (DEL-02-01's family) restrains its translational DOFs, or its one skew direction, rigidly or by its stated stiffness, in the modal basis used by a spectrum solve and in the spectrum solve itself. Every static solve on both routes ignores it, replacing 0.9.0's interim refusal (`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`), and discloses it as "dynamic-only restraint, not active in static cases". A modal request that excludes dynamic-only restraints also ignores it, with the same disclosure.
- **AC-003** — Maintained tests show a snubber changing the modal basis and the spectrum response as a restraint or spring would, leaving every static result unchanged, and disclosed in both.

### Results and readers

- **REQ-006** — *Results and separate readers.*
  - **Publication.** Results are published under the profile `product_preview_spectrum_v1`, with fixed limitation text, and the semantic table `openpipestress.result_semantics/0.3.0/spectrum-1`. Analysis runs record the spectrum run kind with the request.
  - **Content.** Per direction and combined: displacements and rotations, element end forces and moments, and support loads, each an unsigned magnitude with its unit; the per-mode, per-direction contributions from which they were combined; and the missing-mass contribution. Each row states that it is an unsigned combined magnitude.
  - **No signed use.** No signed quantity, deformed shape or stress is recovered from a combined magnitude. A stress from spectrum responses would need per-mode recovery combined by the same rules; it is outside these commitments.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no code with the producer each check the profile and limitation against the request, recompute each combined magnitude from the published contributions and the recorded rules, and check the range and missing-mass disclosures. They do not re-run the modal or spectrum solve.
- **AC-004** — The two readers return the same verdict for every committed spectrum result and every committed mutation of its profile, limitation, contributions, combination rules and disclosures. A verifier-independence test shows that the Python verifier imports no producer code.

### Domain, references and surfaces

- **REQ-007** — *Admitted and refused.* The spectrum solve admits exactly what its modal basis admits, on PF and EX, plus snubbers. What the modal basis refuses is refused with the modal refusal's code. A model with a dynamic-only restraint and no spectrum request solves statically, with the disclosure of REQ-005.
- **AC-005** — Each refusal and disclosure is exercised by a maintained test that asserts its code and named object, on both routes.
- **REQ-008** — *References and screening tolerance.*
  - **References** are derived and frozen under `validation/references/f2_spectrum/` by a TASK that reads no product code, and refuted by a second TASK before any product comparison. They cover REQ-002 to REQ-005 with closed-form and hand-calculation cases.
  - **The spectrum screening class** for `docs/RELEASE_TARGET_v1.0.md` §3 is set from those references before any comparison run, never from comparison results, and reported to Agent 0 for the owner. It is only tightened.
- **AC-006** — Each committed reference package has its refutation record. The screening class, its derivation from the references and the date it was reported precede the first comparison run.
- **REQ-009** — *Desktop and report.* The desktop takes the spectrum request, with every spectrum and choice entered by the user, and shows the results table with its unsigned-magnitude labelling and its disclosures. The rendered report gains a "Response-spectrum analysis" section: request, spectra, modal basis, methods, missing-mass disclosure, results, refusals and limitations. These surfaces follow the results viewer's and report generator's contracts. A published piping benchmark may be compared as comparison evidence under DEL-04-09's provenance rule; it is not required.
- **AC-007** — Desktop tests (Vitest) cover the spectrum editor, including no pre-set spectrum or method, the results table and its labelling, and the report section.
- **REQ-010** — *Claims.* Product texts state that the spectrum method estimates peak responses by stated combination rules, that a result inherits its modal basis's premises, and that it establishes neither physical correctness nor code compliance; they use the claims registry's vocabulary (`docs/claims_registry.md`). No product text states or implies a claim that PRD §21.2 prohibits.
- **AC-008** — The spectrum product texts are pinned by maintained tests and reviewed against `docs/claims_registry.md`, and the claims-language lint (GEN-13) passes over them.

### Independent support motion (Should)

- **REQ-011** — *Independent support motion (Should).* Support groups, each with its own spectra per direction; responses per group combined across groups by the user's rule (absolute sum or SRSS), stated in the result. The pseudo-static part of differential support motion is seismic anchor movement (RT-A10) and is not included. Until built, a request with more than one support group is refused by name.
- **AC-009** — When built: references for a two-group model, and maintained tests of each group-combination rule; until then, a test of the refusal.

## Production and Verification Method — Praxeology

The checks named here are planned; they do not exist yet. Each slice registers its checks in `software-workflow.json` and names them with their head in its pull request.

- **VER-001** — For REQ-001: refusal tests at the product facade, the headless runner and the desktop.
- **VER-002** — For REQ-002 to REQ-004: product tests against the frozen references, mutation tests per method choice and a determinism test.
- **VER-003** — For REQ-005: snubber tests on the modal basis, the spectrum response and both static routes.
- **VER-004** — For REQ-006: Rust reader tests, the Python verifier with its independence test and the shared mutation corpus.
- **VER-005** — For REQ-007: one maintained test per refusal and disclosure, on both routes.
- **VER-006** — For REQ-008: the refutation records and the dated screening-class record.
- **VER-007** — For REQ-009: Vitest for the spectrum editor, results view and report section.
- **VER-008** — For REQ-010: pinned-text tests, GEN-13 and review against `docs/claims_registry.md`.
- **VER-009** — For DEL-04-08: the actual-source D3 run and the AC-004 stack witnesses on each head that touches priced sources.

Each slice also gets the independent review that `AGENTS.md` requires for numerical changes.

## Governing Values and Decisions — Axiology

- **AX-001** — *No silent engineering defaults.* Spectra, scale factors, damping, interpolation, combination rules and the missing-mass inputs are user-entered or refused (PRD §5.5).
- **AX-002** — *One modal basis.* The spectrum solve uses DEL-04-09's published modal basis and mass model, so the two cannot drift.
- **AX-003** — *Unsigned means unsigned.* A combined magnitude is never used as a signed quantity.
- **AX-004** — *No relaxation.* References are never altered to make an implementation pass; the screening class is set before comparison and only tightened.
- **AX-005** — *Independent scrutiny.* Numerical changes get targeted regression checks and independent review by a separate agent.
- **AX-006** — *Scope.* Response spectrum only (SCA-014); time history, harmonic and force-spectrum analysis stay deferred under DEC-078.
- **AX-007** — *Held by the owner:* release, acceptance and issuance, risk acceptance and holds (F-PIP-3), the review of any report held for uncertain rights, and the decision on the screening class it is reported for.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-082 | REQ-001 | AC-001 | VER-001 | Refusal and no-default tests on every entry |
| OUT-002 | OBJ-003 SOW-082 | REQ-002 REQ-003 REQ-004 CLM-001 | AC-002 | VER-002 | Reference, mutation and determinism tests on the candidate head |
| OUT-003 | OBJ-003 SOW-082 | REQ-005 | AC-003 | VER-003 | Snubber tests on modal, spectrum and both static routes |
| OUT-004 | OBJ-003 SOW-082 | REQ-006 | AC-004 | VER-004 | Matching Rust and Python verdicts on the committed corpus; independence test |
| OUT-005 | OBJ-003 SOW-082 | REQ-007 REQ-008 REQ-009 REQ-010 CLM-002 CLM-003 | AC-005 AC-006 AC-007 AC-008 | VER-005 VER-006 VER-007 VER-008 VER-009 | Refusal tests, refutation records and the dated screening class, desktop tests, pinned texts and claims lint, D3 on priced heads |
| OUT-006 | OBJ-003 SOW-082 | REQ-011 | AC-009 | VER-002 VER-005 | Refusal test until built; then references and group-combination tests |
