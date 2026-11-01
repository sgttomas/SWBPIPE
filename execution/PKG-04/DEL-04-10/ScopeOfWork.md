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

For uniform support motion along global direction k, every rigid support moves with the ground, and the free DOFs are excited by p_k = (M R_k)_f, the free rows of the full mass times the rigid translation of all DOFs, consistent-mass coupling to the restrained DOFs included (Design §2). With pseudo-spectral acceleration S_k(f) entered by the user and scaled by the user's factor s_k, each mode i of the modal basis (DEL-04-09), with participation Γ_ik = φ_iᵀp_k / m_i, has the modal displacement u_ik = Γ_ik · s_k·S_k(f_i) / ω_i² · φ_i, relative to the supports. Element end forces and moments and support reactions are recovered with the inertia of the mass, consistent-mass coupling and restrained-DOF mass included (Design §2, §3). The modal responses are combined across modes by the user's modal combination; a rigid response, for the mass the periodic modes leave out, is added when the user applies the missing-mass correction; and the directions are combined by the user's directional combination. Every combined response is an unsigned magnitude.

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
- DEL-02-01: the snubber family's document shape and the `spectra[]` records (0.9.0);
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
- **Snubber family (model document 0.9.0).** DEL-04-10 defines the solve meaning of the `snubber` support family, including the skew `direction` of a snubber with a single translational restraint: a restraint in the dynamic stiffness only, rigid or of its stated stiffness, and absent from every static load-case solve (REQ-005). *Status at S1's merge (#50):* DEL-02-01 carries and validates the format (DEL-02-01:3), and every static solve on both routes refuses a model containing a snubber by name (`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`); the dynamic meaning is open (the dynamics lane's I5).
- **Spectrum records (model document 0.9.0).** DEL-04-10 defines the solve meaning of the top-level `spectra[]` records: user-entered pseudo-acceleration spectra, each with its damping ratio and interpolation, referenced only by the spectrum request (REQ-001). *Status at S1's merge (#50):* DEL-02-01 carries and validates the format, no solve reads a spectrum and no surface displays a spectrum yet; the solve meaning is open (the dynamics lane's I5).
- **Sequence** (the dynamics lane's plan, approved by Agent 0 on 2026-10-31): after DEL-04-09's dense path and modal views, and after its sparse path, this deliverable's references and screening tolerance come first, then the solve and results, then the desktop and report surfaces.
- **Later, outside these commitments:** time history, harmonic and force-spectrum analysis (DEC-078); stresses from spectrum responses (REQ-006).

## Completion and Reliance Basis — Epistemology

A spectrum result is a numerical statement about the discrete linear model under the user's spectra and method choices. It inherits the modal basis's premises and limitations, and bounds no modelling, input or physical error. The spectrum method is approximate by construction: the combination rules estimate peak responses and are not exact. Nothing here determines code compliance.

### Request

- **REQ-001** — *Required inputs, no defaults.*
  - **Spectra.** Each spectrum is a model-document spectrum record (0.9.0 `spectra[]`, DEL-02-01): a user-entered table of ordinates against frequency or period, at least two points, with positive, finite, strictly increasing abscissae and positive finite ordinates in an acceleration unit (or in g multiples with a user-entered positive g), normalized through the unit catalog. Every ordinate is a **pseudo-spectral acceleration** (ω² times the peak relative displacement of a linear oscillator at the spectrum's damping), stated as such in the request, the result and every surface; F2 converts no other kind. Its damping ratio is recorded with it; a spectrum solve admits 0 ≤ ζ ≤ 0.2 and refuses a larger ratio (`SPECTRUM_DAMPING_OUT_OF_RANGE`). Its interpolation (linear or logarithmic on each axis) is the user's choice and has no default. No spectrum, design code spectrum or library ships.
  - **Directions.** One or more global directions, each with a spectrum and a positive finite scale factor; a direction without both is refused. The scale factor scales the whole response of its direction, rigid part included.
  - **Damping.** All spectra in one request have the same damping ratio, or the request is refused (`SPECTRUM_DAMPING_MISMATCH`).
  - **Methods.** The modal combination (SRSS, CQC, or the grouping method for closely spaced modes, Design §4.2) and the directional combination (SRSS or 100-40-40) are required choices with no default. CQC uses the spectra's damping ratio, has no separate damping input, and is refused for a damping ratio of 0 (`SPECTRUM_CQC_DAMPING_UNSUPPORTED`).
  - **Missing mass.** Whether to apply the correction is a required choice. When applied, the rigid frequency f_r and the combination of the rigid response with the periodic part (SRSS or absolute sum) are required inputs; the rigid acceleration is the scaled spectrum's ordinate at f_r.
  - **Modal basis.** The spectrum solve uses a modal request (DEL-04-09) that includes dynamic-only restraints and, where DEL-04-09 requires one (a load/reference-state document or nonlinear supports), names the state case whose resolved state defines the modal model.
  - **Range.** A periodic mode or a rigid frequency outside a spectrum's tabulated range is refused by name; no spectrum is extrapolated.
  - **Extraction premise.** When the correction is applied, the extracted modal set reaches f_r (its highest completed cluster at or above f_r, or the whole finite spectrum), or the request is refused (`SPECTRUM_EXTRACTION_BELOW_RIGID_FREQUENCY`).
  - **Recording.** The request is recorded with the result and in the analysis-run record.
- **AC-001** — Maintained tests show each refusal for a missing spectrum, scale factor, method choice, missing-mass input or modal-basis input; mismatched damping and CQC at zero damping refused; a spectrum with a non-positive or non-monotonic abscissa or a non-positive ordinate refused; a periodic mode or rigid frequency outside the tabulated range refused by name; the extraction premise refused; and no default supplied on any entry: product, headless and desktop.

### Solution

- **REQ-002** — *Modal responses.* Each modal response is formed from the published modal basis by CLM-001, on DEL-04-09's participation over the excitation p_k, in both of its normalization cases, and recovered to displacements, rotations, element end forces and moments, and support reactions, with the inertia terms of Design §2 and §3, on the model's route. Spectral ordinates are interpolated by the spectrum's stated rule. The modal basis's standing is carried: a Sensitive modal set gives a Sensitive spectrum result. Reactions of rigid restraints come from the full residual and forces of compliant supports (springs and finite-stiffness snubbers) from their own stiffness, −K_s·u. Damping is represented only through the spectra, and each mode's quantities are recovered at its displacement extremum; the velocity-proportional parts of forces and reactions are neglected, which the result discloses as an adopted approximation (Design §3).
- **REQ-003** — *Combinations.*
  - **Modal combination**, per direction and per response quantity: SRSS; CQC with the Der Kiureghian correlation coefficient at the spectra's damping ratio; or the grouping method (Regulatory Guide 1.92 Rev. 1's Grouping method) for closely spaced modes, with its closeness rule stated in the result. Each is applied to each response quantity separately, never to a combined vector.
  - **Directional combination:** SRSS of the directional responses, or 100-40-40, the largest of the three permutations.
  - Both rules are recorded with the result.
- **REQ-004** — *Missing-mass correction (rigid response).* When applied, modes below f_r are periodic and the rest, extracted or not, are rigid. For each direction k, the inertia at the rigid acceleration of everything the periodic modes leave out, over all DOFs, a_r,k·(M R_k − Σ_periodic Γ_ik M φ̂_i), is solved statically on the dynamic stiffness and recovered with its inertia to displacements, end forces and reactions in equilibrium (Design §4.3). Extracted rigid modes are never combined as periodic responses. The rigid response is combined with the periodic part by the user's rule before directional combination. The result states, per direction, the periodic modes' cumulative effective-mass fraction and the fraction of the total mass in that direction they do not represent, including mass that moves with the supports, and states f_r and the premise of the rigid response. Without the correction, all extracted modes are periodic and the omission is disclosed with the same fractions.
- **AC-002** — On the frozen `f2_spectrum` references, the product reproduces, within the verification tolerance of REQ-008, the single-degree-of-freedom response under each interpolation rule and the pseudo-acceleration convention; a damped force and reaction case under the displacement-extremum recovery; the one-element consistent-mass and restrained-mass witnesses; a ground-spring reaction beside a rigid-end reaction, in equilibrium; a multi-degree-of-freedom model's modal responses and each modal combination, including CQC on a closely spaced pair with opposite-sign contributions; the 100-40-40 and SRSS directional combinations; and the rigid response, including an extracted rigid mode entering the static solve and a stiff model whose response equals the static response to its rigid acceleration. Mutation tests show each method choice changing the result as its rule predicts. Results are byte-identical on repeated runs.

### Snubbers

- **REQ-005** — *Dynamic-only restraints.* A snubber (DEL-02-01's family) restrains its translational DOFs, or the motion along its one skew direction only, rigidly or by its stated stiffness (as k·d dᵀ/(dᵀd) for a skew direction d), in the dynamic stiffness: the modal basis used by a spectrum solve, the spectrum solve and its auxiliary static solve (REQ-004). Every static load-case solve on both routes ignores it, replacing 0.9.0's interim refusal (`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`), and discloses it as "dynamic-only restraint, not active in static cases". A modal request that excludes dynamic-only restraints also ignores it, with the same disclosure.
- **AC-003** — Maintained tests show a snubber changing the modal basis and the spectrum response as a restraint or spring would, a skew snubber leaving perpendicular motion free, a rigid response whose stiffness depends on a snubber, every static load-case result unchanged, and the disclosure in both.

### Results and readers

- **REQ-006** — *Results and separate readers.*
  - **Publication.** Results are published under the profile `product_preview_spectrum_v1`, with fixed limitation text, and the semantic table `openpipestress.result_semantics/0.3.0/spectrum-1`. Analysis runs record the spectrum run kind with the request.
  - **Content.** Two distinct kinds of row. **Combined rows**, per direction and combined over directions: displacements and rotations, element end forces and moments, and support reactions, each an unsigned magnitude with its unit, stating that it is an unsigned combined magnitude. **Contribution rows**: the signed per-mode, per-direction periodic values and the signed rigid response from which the combined rows were formed, each stating that it is a signed contribution, not a peak response. Signs are kept so that CQC and grouping can be recomputed.
  - **No signed use.** No signed quantity, deformed shape or stress is recovered from a combined magnitude. A stress from spectrum responses would need per-mode recovery combined by the same rules; it is outside these commitments.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no code with the producer each check the profile and limitation against the request, recompute each combined magnitude from the published signed contributions and the recorded rules, and check the range and missing-mass disclosures. They do not re-run the modal or spectrum solve.
- **AC-004** — The two readers return the same verdict for every committed spectrum result and every committed mutation of its profile, limitation, contributions (including a contribution's sign under CQC), combination rules and disclosures. A verifier-independence test shows that the Python verifier imports no producer code.

### Domain, references and surfaces

- **REQ-007** — *Admitted and refused.* The spectrum solve admits exactly what its modal basis admits, on PF and EX, plus snubbers. What the modal basis refuses is refused with the modal refusal's code. A model with a dynamic-only restraint and no spectrum request solves statically, with the disclosure of REQ-005.
- **AC-005** — Each refusal and disclosure is exercised by a maintained test that asserts its code and named object, on both routes.
- **REQ-008** — *References and screening tolerance.*
  - **References** are derived and frozen under `validation/references/f2_spectrum/` by a TASK that reads no product code, and refuted by a second TASK before any product comparison. They cover REQ-002 to REQ-005 with closed-form and hand-calculation cases.
  - **Verification tolerance.** The product is verified against the references at an analytic tolerance under DEC-026 (seeded at 1e-9 relative, with stated absolute floors near zero), measured and only tightened. It is not the screening class.
  - **The spectrum screening class** for `docs/RELEASE_TARGET_v1.0.md` §3, used only for cross-engine comparison, is set from those references before any comparison run, never from comparison results, and reported to Agent 0 for the owner. Its record names the reference revision, the derivation, the date reported and the first comparison run it precedes. It is only tightened.
- **AC-006** — Each committed reference package has its refutation record. The verification tolerance and its measurement are recorded. The screening-class record, bound to its reference revision, is dated before the first comparison run it names.
- **REQ-009** — *Desktop and report.* The desktop takes the spectrum request, with every spectrum and choice entered by the user, and shows the results table with its unsigned-magnitude labelling and its disclosures. The rendered report gains a "Response-spectrum analysis" section: request, spectra, modal basis, methods, missing-mass disclosure, results, refusals and limitations. These surfaces follow the results viewer's and report generator's contracts. A published piping benchmark may be compared as comparison evidence under DEL-04-09's provenance rule; it is not required.
- **AC-007** — Desktop tests (Vitest) cover the spectrum editor, including no pre-set spectrum or method, the results table and its labelling, and the report section.
- **REQ-010** — *Claims.* Product texts state that the spectrum method estimates peak responses by stated combination rules, that a result inherits its modal basis's premises, and that it establishes neither physical correctness nor code compliance; they use the claims registry's vocabulary (`docs/claims_registry.md`). No product text states or implies a claim that PRD §21.2 prohibits.
- **AC-008** — The spectrum product texts are pinned by maintained tests and reviewed against `docs/claims_registry.md`, and the claims-language lint (GEN-13) passes over them.

### Independent support motion (Should)

- **REQ-011** — *Independent support motion (Should).* Support groups, each with its own spectra per direction and its own influence vectors from static unit motion of its supports (Design §8). Each ground spring and finite-stiffness snubber is assigned to a group by the user, and its base moves with that group. Per mode and direction, group responses are combined first by the user's rule (absolute sum or SRSS), then modes (SRSS or grouping; CQC is refused for independent support motion, `SPECTRUM_INDEPENDENT_SUPPORT_MOTION_CQC_UNSUPPORTED`), then directions; a rigid response is formed per group. A group that moves a skewed restraint is refused (`SPECTRUM_INDEPENDENT_SUPPORT_MOTION_SKEW_UNSUPPORTED`) until its inhomogeneous constraint is defined. The rules and order are stated in the result. The pseudo-static part of differential support motion is seismic anchor movement (RT-A10) and is not included; its combination with the spectrum result is DEL-05-02's. Until built, a request with more than one support group is refused by name (`SPECTRUM_INDEPENDENT_SUPPORT_MOTION_UNSUPPORTED`).
- **AC-009** — When built: references for a two-group model, including the combination order and a moving compliant-support base, maintained tests of each group-combination rule and of the CQC refusal; until then, a test of the refusal.

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
