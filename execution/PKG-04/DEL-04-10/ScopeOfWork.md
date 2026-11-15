---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-10
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-082]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-10 Response-spectrum analysis

## Purpose and Objective Traceability

This deliverable computes the seismic response of the linear piping model to uniform support motion by the response-spectrum method, on DEL-04-09's modal basis. An engineer enters the spectra and gets the peak displacements, element end forces and support reactions the earthquake is estimated to cause (or, under an absolute-sum rule, the sum of the magnitudes it combines), each as an unsigned magnitude. It takes every spectrum, damping value and method choice from the user, corrects for the mass the extracted modes do not represent, treats snubbers as restraints in the dynamic analysis only, and refuses by name what it does not admit.

It serves SOW-082 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-10), and PRD §12.9 and §7 item 15. In `docs/RELEASE_TARGET_v1.0.md` it is the owning deliverable for response spectrum: it supplies the response-spectrum analysis of the release target's reference system, solved on the exact route in the desktop with no admitted family refused (exit criterion 1), and its closed-form or hand-calculation references, frozen with an independent generator (exit criterion 2). It also sets the spectrum screening class of the release target's benchmark method. The required capability is user-entered spectra, modal and directional combination, a missing-mass correction, and snubbers as a dynamic-only restraint.

- **OUT-001** — The spectrum request: spectra, direction assignments, method choices and the missing-mass input (REQ-001).
- **OUT-002** — The modal responses, the modal and directional combinations and the missing-mass correction (REQ-002 to REQ-004).
- **OUT-003** — Snubbers as dynamic-only restraints (REQ-005).
- **OUT-004** — The spectrum results, their table, the analysis-run record and the separate readers (REQ-006).
- **OUT-005** — The admitted domain, the frozen references and screening tolerance, the desktop and report surfaces, and the claim boundary (REQ-007 to REQ-010).
- **OUT-006** — Independent (multi-level) support motion, at Should (REQ-011).

## Deliverable Definition — Ontology

### CLM-001 — The method

**Uniform support motion.** For support motion along global direction k, every rigid support moves with the ground, and the free DOFs are excited by p_k = (M R_k)_f, the free rows of the full mass times the rigid translation of all DOFs, consistent-mass coupling to the restrained DOFs included. The modal basis publishes its participation on this excitation (DEL-04-09/REQ-005).

**Modal response.** With pseudo-spectral acceleration S_k(f) entered by the user and scaled by the user's factor s_k, each mode i of the modal basis, with participation Γ_ik = φ_iᵀp_k / m_i, has the modal displacement u_ik = Γ_ik · s_k·S_k(f_i) / ω_i² · φ_i, relative to the supports. At that displacement, recovered at the mode's displacement extremum:
- element end forces and moments are (K_e − ω_i² M_e)·u_ik on the element, so the element's own inertia is included and end forces at a node balance the nodal inertia;
- reactions are (K_cf − ω_i² M_cf)·u_ik at rigid restraints, so mass at restrained DOFs and consistent-mass coupling load the supports, and −K_s·u_ik at each compliant support s.

Every modal quantity stays signed per mode and per direction until combination.

**Modal combination**, on each response quantity separately, with R_i the signed periodic value of mode i in one direction:
- SRSS: R = √(Σ R_i²).
- CQC: R = √(Σ_i Σ_j ρ_ij R_i R_j), with ρ_ij = 8ζ²(1 + r)r^{3/2} / ((1 − r²)² + 4ζ²r(1 + r)²), r = ω_j/ω_i and ζ the spectra's damping ratio.
- Grouping: modes in ascending order form groups in which each frequency is within 10 % of the group's lowest, and R = √(Σ R_i² + 2 Σ_groups Σ_{i<j in the group} |R_i R_j|).
- Ten percent: R = √(Σ R_i² + 2 Σ |R_i R_j|), the second sum over every pair i < j with (f_j − f_i)/f_i ≤ 0.1, whether or not the two modes fall in one group. It is a different rule from grouping.
- Absolute sum: R = Σ |R_i|, taken over the cluster sums where modes of a cluster were summed first (Repeated modes, below). It is an upper bound on every other rule's combination of the same magnitudes. It is not an estimate of the peak, and it is not a bound on the response of unequal-frequency modes whose contributions cancelled inside a cluster.

**Repeated modes.** For an exactly repeated eigenvalue, every M-orthogonal basis of its eigenspace gives the same signed sum of its modes' contributions to any response quantity, whatever each shape's normalization: the sum is the response of the excitation's projection on the eigenspace. No single mode's contribution has that property, and mode-by-mode SRSS and the rules that use per-mode magnitudes need not preserve it (CQC does at an exactly repeated eigenvalue, where its coefficient is 1). The modes of one cluster of the modal basis are therefore summed with their signs first, R_c = Σ_{i in the cluster} R_i, and every modal rule above is applied to the cluster sums. A cluster here is DEL-04-09's: adjacent computed eigenvalues chained by its relative cluster tolerance δ_c (DEL-04-09/REQ-002, REQ-003). It is a numerical convention, not a statement that the eigenvalues are equal, and it is not the 10 % closeness of the grouping and ten percent methods. Each mode keeps its own f_i and ω_i² for its spectral ordinate, its displacement and its recovery; the cluster's lowest frequency stands for the cluster only in the correlation coefficient, in the closeness tests of the grouping and ten percent methods and in the periodic or rigid classification. The independence from the basis is exact only under both premises, equal eigenvalues and M-orthogonal shapes. DEL-04-09 checks the modal basis's M-orthogonality to a tolerance, and a breach makes the modal set, and so the spectrum result, Sensitive (REQ-002). For a cluster whose modes have unequal frequencies, summing them first is an adopted numerical convention: such modes have different spectral ordinates and recovery factors, and no independence from the basis is claimed for it, exactly or to within the orthogonality tolerance. For close but distinct modes it can differ materially from applying the named rule mode by mode, SRSS included, however small δ_c is.

**Rigid response.** With a rigid frequency f_r, the modes of a cluster whose lowest frequency is below f_r are periodic; the modes of every other cluster, and every mode not extracted, are rigid. The rigid acceleration is a_r,k = s_k·S_k(f_r). The rigid response of direction k is the static response, on the dynamic stiffness, to F_k = a_r,k·(M R_k − Σ_periodic Γ_ik M φ̂_i) over all DOFs. Its element end forces are K_e u − F_e with F_e the element's own share of F_k, and its reactions are K_cf u − F_k,c at rigid restraints and −K_s·u at compliant supports, so that the reactions balance the total of F_k. It assumes that the modes classified as rigid respond in phase with the ground.

**Directional combination.** SRSS over the directions requested; 100-40-40: the largest of R_a + 0.4 R_b + 0.4 R_c over the permutations of the directional magnitudes; or the absolute sum of the directional magnitudes.

**Independent support motion.** For support group g and direction k, the influence vector ψ_gk is the static displacement of the free DOFs under unit motion along k of the group's restrained DOFs and of the bases of its compliant supports, with the other groups fixed; the sum over all groups is the uniform case. The excitation is p_gk = (M ψ̂_gk)_f.

Every combined response is an unsigned magnitude.

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:**
- the spectrum request, its refusals and disclosures;
- the modal responses and their recovery, the modal and directional combinations and the missing-mass correction;
- the dynamic meaning of the snubber support family: a restraint in the dynamic analysis, absent from every static solve. DEL-04-09 applies this meaning in its modal basis and does not restate it;
- the solve meaning of the model document's spectrum records;
- the product's spectrum entry and the headless spectrum entry, a command the headless runner hosts (DEL-10-05/REQ-015) beside DEL-04-07's solve entry, reading that entry's built model and supplying no value of its own; the spectrum profile, the `spectrum-1` semantic table and the spectrum analysis-run kind;
- the product's spectrum reader and its independent test verifier;
- the frozen references under `validation/references/f2_spectrum/` and the spectrum screening tolerance;
- the desktop spectrum editor and results view and the report's response-spectrum section, within the results viewer's and report generator's contracts.

**Inputs from neighbours** (`deliverable.yaml`): DEL-04-09 (the modal basis and the mass model), DEL-02-01 (the document shape of the snubber family and of the spectrum records), DEL-04-03 (restraint partition and spring assembly), DEL-04-07 (the built model, stiffness and static recovery on both routes), DEL-02-02 (units), DEL-00-06 (the diagnostics contract), DEL-14-02 (the analysis-run record), DEL-10-05 (the headless runner), and DEL-07-05 and DEL-08-01 (the viewer's and report's contracts). These deliverables keep their scope. Nonlinear support states reach the spectrum solve only through DEL-04-09's modal basis.

### CLM-003 — What is outside these commitments

- Time-history, harmonic and force-spectrum analysis.
- Stresses from spectrum responses. They need per-mode stress recovery combined by the same rules, never recovery from combined forces.
- Combination of a spectrum result with static cases.
- The retained-precision route: the spectrum entry is not a retained-precision entry.
- The pseudo-static response to differential support displacement, which is seismic anchor movement, a static load case.
- Conversion of a spectrum of any other kind to pseudo-acceleration, and any shipped spectrum, design-code spectrum or library.

## Completion and Reliance Basis — Epistemology

**Undecided.**

- The spectrum screening class has no value. It is derived from the frozen references before any comparison run (REQ-008).
- No deliverable currently commits to combining a spectrum result with static cases (for example sustained plus seismic). Whether and how the load-case algebra (DEL-05-02) admits a spectrum result as an unsigned operand is not decided.
- Independent support motion that moves a skewed restraint has no defined constraint, and CQC has no defined form for independent support motion; both are refused (REQ-011).

### Request

- **REQ-001** — *Required inputs, no defaults.*
  - **Spectra.** Each spectrum is a model-document spectrum record (`spectra[]`, DEL-02-01): a user-entered table of ordinates against frequency or period, at least two points, with positive, finite, strictly increasing abscissae and positive finite ordinates in an acceleration unit (or in g multiples with a user-entered positive g), normalized through the unit catalog. Every ordinate is a **pseudo-spectral acceleration** (ω² times the peak relative displacement of a linear oscillator at the spectrum's damping), stated as such in the request, the result and every surface; the spectrum solve converts no other kind. Its damping ratio is recorded with it; a spectrum solve admits 0 ≤ ζ ≤ 0.2 and refuses a larger ratio (`SPECTRUM_DAMPING_OUT_OF_RANGE`). Its interpolation (linear or logarithmic on each axis) is the user's choice and has no default. No spectrum, design code spectrum or library ships.
  - **Directions.** One or more global directions, each with a spectrum and a positive finite scale factor; a direction without both is refused. The scale factor scales the whole response of its direction, rigid part included.
  - **Damping.** All spectra in one request have the same damping ratio, or the request is refused (`SPECTRUM_DAMPING_MISMATCH`).
  - **Methods.** The modal combination (SRSS, CQC, the grouping method, the ten percent method or the absolute sum) and the directional combination (SRSS, 100-40-40 or the absolute sum) are required choices with no default. CQC uses the spectra's damping ratio, has no separate damping input, and is refused for a damping ratio of 0 (`SPECTRUM_CQC_DAMPING_UNSUPPORTED`).
  - **Missing mass.** Whether to apply the correction is a required choice. When applied, the rigid frequency f_r and the combination of the rigid response with the periodic part (SRSS or absolute sum) are required inputs; the rigid acceleration is the scaled spectrum's ordinate at f_r.
  - **Modal basis.** The spectrum solve uses a modal request (DEL-04-09) that includes dynamic-only restraints and, where DEL-04-09 requires one (a load/reference-state document or nonlinear supports), names the state case whose resolved state defines the modal model.
  - **Range.** A periodic mode or a rigid frequency outside a spectrum's tabulated range is refused by name; no spectrum is extrapolated.
  - **Extraction premise.** When the correction is applied, the extracted modal set reaches f_r (the lowest frequency of its highest completed cluster is at or above f_r, or it is the whole finite spectrum), or the request is refused (`SPECTRUM_EXTRACTION_BELOW_RIGID_FREQUENCY`).
  - **Recording.** The request is recorded with the result and in the analysis-run record.
- **AC-001** — Maintained tests show each refusal for a missing spectrum, scale factor, method choice, missing-mass input or modal-basis input; mismatched damping and CQC at zero damping refused; a spectrum with a non-positive or non-monotonic abscissa or a non-positive ordinate refused; a periodic mode or rigid frequency outside the tabulated range refused by name; the extraction premise refused; and no default supplied on any entry: product, headless and desktop.

### Solution

- **REQ-002** — *Modal responses.* Each modal response is formed from the published modal basis by CLM-001, on DEL-04-09's participation over the excitation p_k, in both of its normalization cases, and recovered to displacements, rotations, element end forces and moments, and support reactions, with the inertia terms of CLM-001, on the model's route. Spectral ordinates are interpolated by the spectrum's stated rule. The modal basis's standing is carried: a Sensitive modal set gives a Sensitive spectrum result. Reactions of rigid restraints come from the full residual and forces of compliant supports (springs and finite-stiffness snubbers) from their own stiffness, −K_s·u. Damping is represented only through the spectra, and each mode's quantities are recovered at its displacement extremum; the velocity-proportional parts of forces and reactions are neglected, which the result discloses as an adopted approximation. No bound on the resulting force or reaction error is claimed.
- **REQ-003** — *Combinations.*
  - **Modal combination**, per direction and per response quantity: SRSS; CQC with the Der Kiureghian correlation coefficient at the spectra's damping ratio; the grouping method (Regulatory Guide 1.92 Rev. 1's Grouping method) for closely spaced modes, with its closeness rule stated in the result (CLM-001); the ten percent method, with its closeness rule stated in the result (CLM-001); or the absolute sum. Each is applied to each response quantity separately, never to a combined vector.
  - **Repeated modes.** The modes of one cluster of the modal basis (DEL-04-09's cluster, by its cluster tolerance δ_c; not the 10 % closeness of the grouping and ten percent methods) are summed with their signs first, and the modal rule is applied across clusters (CLM-001), so that a combined response does not depend on which M-orthogonal basis the modal solve returned for an exactly repeated eigenvalue; for a cluster of unequal frequencies it is an adopted convention (CLM-001). The result states each cluster so combined: its modes, their frequencies and the frequency that stands for it.
  - **Directional combination:** SRSS of the directional responses; 100-40-40, the largest of the three permutations; or their absolute sum.
  - Both rules are recorded with the result.
- **REQ-004** — *Missing-mass correction (rigid response).* When applied, the modes of each cluster whose lowest frequency is below f_r are periodic and the rest, extracted or not, are rigid (CLM-001); every mode of a periodic cluster enters the periodic sum that the rigid load leaves out, and no mode of a rigid cluster does. For each direction k, the inertia at the rigid acceleration of everything the periodic modes leave out, over all DOFs, a_r,k·(M R_k − Σ_periodic Γ_ik M φ̂_i), is solved statically on the dynamic stiffness and recovered with its inertia to displacements, end forces and reactions in equilibrium (CLM-001). Extracted rigid modes are never combined as periodic responses. The rigid response is combined with the periodic part by the user's rule before directional combination. The result states, per direction, the periodic modes' cumulative effective-mass fraction and the fraction of the total mass in that direction they do not represent, including mass that moves with the supports, and states f_r and the premise of the rigid response. Without the correction, all extracted modes are periodic and the omission is disclosed with the same fractions.
- **AC-002** — On the frozen `f2_spectrum` references, the product reproduces, within the verification tolerance of REQ-008, the single-degree-of-freedom response under each interpolation rule and the pseudo-acceleration convention; a damped force and reaction case under the displacement-extremum recovery; the one-element consistent-mass and restrained-mass witnesses; a ground-spring reaction beside a rigid-end reaction, in equilibrium; a multi-degree-of-freedom model's modal responses and each modal combination, including CQC on a closely spaced pair with opposite-sign contributions and the grouping and ten percent methods on a set of modes where the two differ; an exactly repeated pair supplied in two bases, the second a rotation of the first as a mass-orthonormal basis followed by the published normalization, giving under every modal rule the same combined response, equal to the reference; a cluster of two modes with different frequencies within δ_c, combined as CLM-001 states with each mode's own ordinate, including a response quantity to which the two contributions nearly cancel; a cluster that straddles f_r, periodic as a whole; the 100-40-40, SRSS and absolute-sum directional combinations; and the rigid response, including an extracted rigid mode entering the static solve and a stiff model whose response equals the static response to its rigid acceleration. Mutation tests show each method choice changing the result as its rule predicts. Results are byte-identical on repeated runs.

### Snubbers

- **REQ-005** — *Dynamic-only restraints.* A snubber (DEL-02-01's family) restrains its translational DOFs, or the motion along its one skew direction only, rigidly or by its stated stiffness (as k·d dᵀ/(dᵀd) for a skew direction d), in the dynamic stiffness: the modal basis used by a spectrum solve, the spectrum solve and its auxiliary static solve (REQ-004). Every static load-case solve on both routes ignores it and discloses it as "dynamic-only restraint, not active in static cases". A modal request that excludes dynamic-only restraints also ignores it, with the same disclosure.
- **AC-003** — On the frozen `f2_spectrum` snubber references (axis and skew, rigid and finite stiffness), the product reproduces the modal basis and the spectrum response within REQ-008's verification tolerance, with perpendicular motion free under a skew snubber and a rigid response whose stiffness depends on the snubber. Every static load-case result is unchanged by the snubber, and the disclosure appears in both.

### Results and readers

- **REQ-006** — *Results and separate readers.*
  - **Publication.** Results are published as the spectrum result family `openpipestress.spectrum_result` under the profile `product_preview_spectrum_v1`, with fixed limitation text, and the semantic table `openpipestress.result_semantics/spectrum-1`, outside the `results.v0.3` envelope. Each document binds its request exactly. Analysis runs record the spectrum run kind with the request.
  - **Content.** Two distinct kinds of row. **Combined rows**, per direction and combined over directions: displacements and rotations, element end forces and moments, and support reactions, each an unsigned magnitude with its unit, stating that it is an unsigned combined magnitude. **Contribution rows**: the signed per-mode, per-direction periodic values and the signed rigid response from which the combined rows were formed, each stating that it is a signed contribution, not a peak response, and each periodic row naming its mode's cluster. Signs are kept so that every combination can be recomputed.
  - **No signed use.** No signed quantity, deformed shape or stress is recovered from a combined magnitude. A stress from spectrum responses would need per-mode recovery combined by the same rules; it is outside these commitments.
  - **Readers.** A Rust product reader and a test-only Python verifier that shares no code with the producer each check the profile and limitation against the request, recompute each combined magnitude from the published signed contributions and the recorded rules, summing each cluster's contributions with their signs first, and check the result's statement of its clusters and the range and missing-mass disclosures. They do not re-run the modal or spectrum solve.
- **AC-004** — The two readers return the same verdict for every committed spectrum result and every committed mutation of its profile, limitation, contributions (including a contribution's sign under CQC), cluster statement (a contribution moved to another cluster, a cluster split, a changed standing frequency), combination rules and disclosures. The committed results include one in which a cluster's contributions to a response quantity cancel. A verifier-independence test shows that the Python verifier imports no producer code.

### Domain, references and surfaces

- **REQ-007** — *Admitted and refused.* The spectrum solve admits exactly what its modal basis admits, on PF and EX, plus snubbers. What the modal basis refuses is refused with the modal refusal's code. A model with a dynamic-only restraint and no spectrum request solves statically, with the disclosure of REQ-005.
- **AC-005** — Each refusal and disclosure is exercised by a maintained test that asserts its code and named object, on both routes.
- **REQ-008** — *References and screening tolerance.*
  - **References.** Closed-form and hand-calculation references for REQ-002 to REQ-005 are frozen under `validation/references/f2_spectrum/`, each derived without reading product code, with a generator that imports none.
  - **Verification tolerance.** The product is verified against the references at the governed analytic tolerance (seeded at 1e-9 relative, with stated absolute floors near zero), measured and only tightened. It is not the screening class.
  - **The spectrum screening class** for `docs/RELEASE_TARGET_v1.0.md` ("Benchmark method"), used only for cross-engine comparison, is set from those references before any comparison run, never from comparison results, by agents with no access to the exchange folder, to any output of the external prover or to anything derived from one, and reported to the owner. It is only tightened.
- **AC-006** — Each frozen reference regenerates byte for byte from its generator, and the generator imports no product code. The verification tolerance in force is stated with its measured margin. The spectrum screening class exists in the release target's benchmark profile, derived from those references, before any spectrum comparison run, and was set by agents that met REQ-008's no-access condition.
- **REQ-009** — *Desktop and report.* The desktop takes the spectrum request, with every spectrum and choice entered by the user, and shows the results table with its unsigned-magnitude labelling and its disclosures. The rendered report gains a "Response-spectrum analysis" section: request, spectra, modal basis, methods, missing-mass disclosure, results, refusals and limitations. These surfaces follow the results viewer's and report generator's contracts. A published piping benchmark may be compared as comparison evidence under DEL-04-09's provenance rule; it is not required.
- **AC-007** — Desktop tests cover the spectrum editor, including no pre-set spectrum or method, the results table and its labelling, and the report section.
- **REQ-010** — *Claims.* Product texts state that the spectrum method estimates peak responses by stated combination rules, or under an absolute-sum rule sums the magnitudes it combines, that a result inherits its modal basis's premises, and that it establishes neither physical correctness nor code compliance; they use the claims registry's vocabulary (`docs/claims_registry.md`). Each combination rule is named by what it computes: the grouping and ten percent methods by their closeness rules, and the absolute sum as the sum of the magnitudes it combines (cluster sums, where modes of a cluster were summed first), an upper bound on every other rule's combination of those magnitudes, and neither an estimate of the peak response nor a bound on it; a result that summed a cluster of modes first says so. Product texts state that SWBPIPE applies the rule the user selects and does not determine whether a rule is acceptable under any code or guide. No product text states or implies a claim that PRD §21.2 prohibits.
- **AC-008** — Every spectrum product text states REQ-010's limits in the claims registry's vocabulary, and none states or implies a claim PRD §21.2 prohibits.
- **AC-010** — The release target's reference system, with a user-entered spectrum request, solves by response spectrum on the exact route in the desktop with no admitted family refused, and its results and disclosures are shown.

### Independent support motion (Should)

- **REQ-011** — *Independent support motion (Should).* Support groups, each with its own spectra per direction and its own influence vectors from static unit motion of its supports (CLM-001). Each ground spring and finite-stiffness snubber is assigned to a group by the user, and its base moves with that group. Within each support group and direction, the modes of one cluster are summed with their signs first (REQ-003). Per cluster and direction, group responses are then combined by the user's rule (absolute sum or SRSS), then clusters by the modal rule (SRSS, grouping, ten percent or absolute sum; CQC is refused for independent support motion, `SPECTRUM_INDEPENDENT_SUPPORT_MOTION_CQC_UNSUPPORTED`), then directions; a rigid response is formed per group. A group that moves a skewed restraint is refused (`SPECTRUM_INDEPENDENT_SUPPORT_MOTION_SKEW_UNSUPPORTED`). The rules and order are stated in the result. The pseudo-static part of differential support motion is seismic anchor movement, a static load case, and is not included; its combination with the spectrum result is not this deliverable's, and no deliverable currently commits to it. Where independent support motion is not admitted, a request with more than one support group is refused by name (`SPECTRUM_INDEPENDENT_SUPPORT_MOTION_UNSUPPORTED`).
- **AC-009** — Where independent support motion is admitted, the product reproduces references for a two-group model, including the combination order, a moving compliant-support base and a repeated cluster supplied in two rotations of its mass-orthonormal basis, under each group-combination rule; CQC is refused for independent support motion; and a request outside the admitted domain is refused by name. Where it is not admitted, a request with more than one support group is refused by name.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *One modal basis.* The spectrum solve reads a published modal result and the modal mass model. It forms no mass and solves no eigenproblem of its own.
- *A cluster's modes are summed on the published participation factors.* The alternative is to form a cluster's modal coordinates η from the cluster's own mass Gram matrix, (Φ_cᵀ M Φ_c) η = Φ_cᵀ p_k, which is exact for any basis of an eigenspace, M-orthogonal or not. It is not adopted: it departs from the participation factors the modal result publishes, and the readers could not recompute it from that result. It is taken up again if measurement shows that M-orthogonality inside clusters limits agreement with the references at the verification tolerance.
- *Signed until combined.* Per-mode and rigid contributions are kept signed and published, so every combined magnitude can be recomputed from them.
- *The rigid response is one static solve per direction* on the dynamic stiffness; extracted rigid modes enter it with their signs.
- *References first.* The references and the screening class exist before any comparison run.
- *Readers do not re-solve.* The product reader and the test verifier recompute combined magnitudes from published contributions.

**How each criterion is examined.**

- **VER-001** — Executable check: observes each request refusal, and the absence of any default, at the product entry, the headless runner and the desktop.
- **VER-002** — Comparison with a named reference: the product against the frozen references in `validation/references/f2_spectrum`, at the verification tolerance, with a mutation of each method choice and repeat-run byte identity. Where independent support motion is admitted, it also examines AC-009 against the two-group references.
- **VER-003** — Comparison with a named reference, with an executable check: snubber models against their references in `validation/references/f2_spectrum` on the modal basis and the spectrum response; the check observes every static load-case result unchanged and the disclosure, on both routes.
- **VER-004** — Executable check: observes the product reader and the test verifier returning the same verdict on every committed result and mutation, and that the verifier imports no producer code.
- **VER-005** — Executable check: observes each refusal and disclosure with its code and named object, on both routes, including the independent-support-motion refusals.
- **VER-006** — Executable check with judgment by a reviewer who did not write the references: the check regenerates each reference from its generator and scans the generator's imports; each reference is derived by an agent that reads no product code, and each reference package is refuted by a second agent that reads no product code before any product comparison; the reviewer confirms the stated verification tolerance, that the screening class precedes the first spectrum comparison run, and that those who set it met REQ-008's no-access condition.
- **VER-007** — Executable check: observes the spectrum editor without a pre-set spectrum or method, the results table and its labelling, and the report section.
- **VER-008** — Executable check with judgment by a reviewer who did not write the text: the check observes the pinned product texts; the reviewer examines them against `docs/claims_registry.md` and PRD §21.2.
- **VER-010** — Executable check: the reference system's response-spectrum request, entered by the user, solves on the exact route in the desktop with no admitted family refused, and its results and disclosures are shown.

## Governing Values and Decisions — Axiology

- **AX-001** — *No silent engineering defaults.* Fewer values the product supplies on the user's behalf is better; the target is none. No spectrum, scale factor, damping, interpolation, combination or missing-mass input is ever defaulted to save an entry (PRD §5.5).
- **AX-002** — *One modal basis.* Better is one modal basis and mass model shared with DEL-04-09, with no spectrum-side re-derivation. That identity is never traded for speed.
- **AX-003** — *Unsigned means unsigned.* Better is a combined magnitude whose label leaves its sign status beyond doubt. A combined magnitude is never offered or used as a signed quantity.
- **AX-004** — *Closer to the references, no relaxation.* Better is tighter agreement with the independent references. Their independence is never traded, a tolerance or the screening class is never loosened, and a reference is never altered to pass.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-082 | REQ-001 | AC-001 | VER-001 | SOW-082 and PRD §5.5 (no defaults); each refusal is named in REQ-001 |
| OUT-002 | OBJ-003 SOW-082 | REQ-002 REQ-003 REQ-004 CLM-001 | AC-002 | VER-002 | Outside source: closed-form and hand-calculation references in `validation/references/f2_spectrum`, whose generator imports no product code. Repeat-run identity is the product's own earlier output and only guards against change |
| OUT-003 | OBJ-003 SOW-082 | REQ-005 | AC-003 | VER-003 | Outside source for the snubber response: closed-form references in `validation/references/f2_spectrum`. Unchanged static results are the product's own earlier output and only guard against change |
| OUT-004 | OBJ-003 SOW-082 | REQ-006 | AC-004 | VER-004 | Outside source: a verifier that imports no producer code, recomputing from the recorded rules. The committed documents are the product's own earlier output and only guard against change |
| OUT-005 | OBJ-003 SOW-082 | REQ-007 CLM-002 CLM-003 | AC-005 | VER-005 | SOW-082; the refusals are those REQ-007 names |
| OUT-005 | OBJ-003 SOW-082 | REQ-008 | AC-006 | VER-006 | Outside source: references with an independent generator (release target exit criterion 2); the screening class is set before comparison under the release target's benchmark method; the governed analytic tolerance for the verification tolerance |
| OUT-005 | OBJ-003 SOW-082 | REQ-009 | AC-007 | VER-007 | Release target exit criterion 1 and PRD §5.5 |
| OUT-005 | OBJ-003 SOW-082 | REQ-010 | AC-008 | VER-008 | Outside source: `docs/claims_registry.md` and PRD §21.2 |
| OUT-006 | OBJ-003 SOW-082 | REQ-011 | AC-009 | VER-002 VER-005 | Where independent support motion is admitted, outside source: two-group closed-form references in `validation/references/f2_spectrum`, whose generator imports no product code; otherwise the criterion states the expected refusal |
| OUT-005 | OBJ-003 SOW-082 | REQ-007 REQ-009 | AC-010 | VER-010 | Release target exit criterion 1 names the reference system. The method's correctness rests on AC-002; the reference system's values have no reference of their own |
