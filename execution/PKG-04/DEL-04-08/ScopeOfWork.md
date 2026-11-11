---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-08
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-080]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-08 Retained-precision result verification

## Purpose and Objective Traceability

This deliverable lets the product say how accurate a published number is. Ordinary results are computed in double precision and carry integrity checks; for a case those checks leave in doubt, the retained-precision method re-solves at extended precision, or solves exactly from retained source terms, and publishes each covered quantity with a verified bound on its distance from the exact solution of the recorded problem. Where it cannot establish a bound, the ordinary values and standing stay exactly as they were. A reader separate from the solver, and a test verifier that shares no code with it, can each check what was published. The engineer gains a stated arithmetic and solution error; nothing is claimed about modelling, input data or physical behaviour.

It serves SOW-080 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-08), and PRD §12.8, "Verified result bounds". No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it, and the release target does not require the method to be active on the desktop.

- **OUT-001** — The verified-quantity contract: the publication identities, receipts and row classes that state which result quantities carry a verified bound, and in what form (REQ-001, REQ-002).
- **OUT-002** — The fallback and fixed-limits rule, as the producer follows it and the reader checks it (REQ-003).
- **OUT-003** — The memory-pricing law, its registered profile and its qualification procedure (REQ-004).
- **OUT-004** — The separate consumers: the product reader in its native and WebAssembly builds, the test verifier, and their comparison records and corpora (REQ-005).
- **OUT-005** — The stated limits and the claim boundary (REQ-006, REQ-008).

## Deliverable Definition — Ontology

### CLM-001 — The method

A verified bound states that a published value lies within a stated distance of the exact solution of the problem the method records (PRD §12.8). For a retained-precision result, that problem is the declared linear problem formed from the recorded inputs; for retained-source recovery, it is the represented source problem. The bound covers arithmetic and solution error only. Any difference between that problem and the authored inputs is disclosed, not bounded. The method has two publication families.

1. **Retained-precision successors.**
   - **Trigger.** The producer attempts a load case whose ordinary result did not pass its integrity checks. It does not attempt a case whose ordinary attempt failed as a mechanism, an asymmetric system or invalid input, except under REQ-003 (d).
   - **Solve.** The case is re-solved at extended precision. Native candidates run at 128, 256 and 512 bits, each verified at twice its precision, and the product proof and projection run at 1024 bits.
   - **Publication.** When every check passes, a successor statement is published in place of the ordinary one. On route L, the pressure-free route, this is `preview-physics-retained-1`. On route E, the exact route with explicitly empty pressure regions, it is `physics-retained-1`.
   - **Receipt.** `schemas/retained_precision_mp_v2.schema.json`, policy `M03-INTEGRITY-MP-v2`.
   - **Formation definitions.** DEF-O (ordinary), DEF-C (combination) and DEF-E (exact route): the committed definitions of the problem each successor is a solution of. With the two semantic tables they are the method's reviewed inputs, bound into every publication by digest.
2. **Retained-source recovery** (`pressure-source-1`; receipt policy `PRESSURE-SOURCE-1`, `schemas/physics_source_recovery.schema.json`). On the ordinary exact route of a solve that captured its invocation, an eligible case is solved exactly from its retained source blocks. A case is eligible when its ordinary result is sensitive or its ordinary attempt errs. Each covered quantity is published as an outward interval.

**Further terms.**

- *Body scale.* The reference magnitude S of the body a row belongs to, against which the row's class is decided.
- *Registered build.* A build whose exact compiled identity, reviewed-input digests and reader layouts match a registered memory profile. Only a registered build can produce a successor.
- *D1.* The fixed admission domain of a retained-precision invocation (REQ-004). *D3.* The audit that prices the actual source against the memory profile; any change to a priced source or a reviewed input needs a new one.
- *Standing.* What the reader says a result may be used for: numerically eligible, needing recomputation, or unsupported.

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns** the two families' receipts, tables and formation definitions, and their publication and classification law; the producer's retained, source-recovery and source-receipt parts; the product reader for these statements, with its command-line and WebAssembly bindings; the test verifier; the reader-outcome records and corpora; and the memory-pricing law with its pricing tool.

**Inputs from neighbours** (`deliverable.yaml`): DEL-04-01 (the kernel's retained-precision lanes, their error proof and the exact boundary), DEL-04-07 (product solve integration and the memory budget), DEL-05-01 (primitive load records), DEL-05-05 (the distributed user load), DEL-04-02 (the consistent equivalent nodal loads and station recovery), DEL-05-03 (section bases and stress definitions), DEL-05-02 (combination semantics and the base-route readers), DEL-02-02 (units), DEL-08-02 (canonical JSON), DEL-00-06 (the diagnostics contract) and DEL-10-05 (the headless runner). These deliverables keep their scope.

### CLM-003 — Where the method runs

- **Retained-source recovery** runs only in a solve that captured its invocation. An entry with no captured invocation never attempts it.
- **Retained-precision successors** are produced only through the direct retained entry, in a registered build, on a surface the owner has activated (REQ-009).
- **Readers.** Every reader surface takes the product reader's verdict: the desktop through WebAssembly, the command line through the runner's read command, and Python product code through a binding that has no fallback.

## Completion and Reliance Basis — Epistemology

In these commitments, "exact solution" means the exact solution of the recorded problem defined in CLM-001. Nothing here bounds modelling, input or physical error.

### Verified quantities

- **REQ-001** — *Retained-precision successors.*
  - **Classes.** Every published row of a successor is classified as exactly one of `relative_verified`, `absolute_verified`, `input_derived`, `non_quantity` or `not_covered`. The classes are defined by the table's `accuracy_classification` (policy `RP-FACADE-SI-v2`) and applied by the reader's row-class gate (G5c).
  - **Which rows can be verified.** Only these row kinds can carry a verified class:
    - nodal translations and displacement magnitudes;
    - nodal rotations;
    - member-local axial and shear forces, and torsional and bending moments;
    - pipe-wall and effective axial forces, and pipe-wall endpoint actions;
    - support reaction components, force and moment magnitudes, and reaction resultants;
    - the stresses `element_local_axial_normal_stress`, `element_local_bending_normal_stress_y` and `_z`, `element_local_torsional_shear_stress`, `pipe_axial_membrane_stress_v2`, `pipe_elastic_normal_stress_maximum_v2` and `open_formula_stress_summary`.
  - **What each class guarantees.** Let S be the row's body scale.
    - A row of these kinds is `relative_verified` when S ≥ 2^-988 and |value| ≥ 2^-34·S. Its value then lies within 10^-9·|value| of the exact solution, and also within the kernel's sharper allowance (DEF-O `acceptance.final`).
    - Any other row of these kinds that has a body scale is `absolute_verified`, and its value lies within its published bound. The bound is RU64(2^-64·S); for 0 < S < 2^-988 it is one upward rounding of b0 + r plus the minimum subnormal; for S = 0 it is +0.
    - The bound holds for both of the formation's stiffness readouts: the admitted binary64 section products and the exact annulus geometry. For a Timoshenko member, the admitted readout uses the binary64 shear-area leaf and the exact-annulus readout the exact energy-matched factor κ_E.
  - **Rows without a verified class.**
    - Prescribed degrees of freedom are `input_derived` with value +0, as are the Lamé and section-pressure stresses and the user-input reviews. They are formed from the inputs and carry no solve bound.
    - These rows are `not_covered`: every other row kind; rows with no body scale; and each quantity row of an unavailable combination or of an ordinary combination that names a load case whose trigger is not `not_required`. They keep their ordinary values, except the section normal-stress extremes and transverse-shear maxima, which are formed in binary64 from the successor's own published stress, section-action and end-force rows and its captured section and material.
    - Intensified bending stresses carry no verified class: the reader refuses a successor that classifies one (G5b).
  - **Route L domain.**
    - It covers straight Euler–Bernoulli members, nodal loads and full-span uniform member loads, rigid restraints and global scalar springs, and base or named-point E and G. DEF-O and DEF-C also define interpolated E and G, which D1 refuses at admission.
    - With member loads, the bound covers the declared-load problem. Each declared entry lies within 2^-1022 of its exact consistent value, relative to that value. That difference is disclosed but not bounded.
    - It also covers mechanics combinations under DEF-C, which reproduces DEL-05-02's combination semantics and withholding gates. Each combination has its own solve on the combined exact ledger and at least one selected operand. It publishes exactly 7n + 50m + 8g rows (n nodes, m members, g attributed supports), of which constrained displacement components are `input_derived` +0.
    - Combination maxima, intensified rows, range envelopes, state differences and combinations with member-load operands are not covered.
  - **Route E domain.** It covers direct load cases with nodal loads only, under these conditions:
    - current model 0.9.0 with the explicit `3.0.0/exact_pressure_v3` contract;
    - explicitly empty pressure regions;
    - straight members;
    - base common E and ν;
    - either explicit shear model (`euler_bernoulli` or `timoshenko_annulus_energy_v1`) on straight members, and any admitted mill tolerance and corrosion allowance, on DEL-05-03's two section bases: stiffness and the Timoshenko shear area on the nominal wall, and each stress row on its own basis section.
- **REQ-002** — *Retained-source recovery.* For each case it qualifies, a `pressure-source-1` receipt publishes:
  - for nodal translations and rotations, member end and station actions, support action components and constraint reactions (`projection.quantity`): an outward interval that contains the exact value of the case's source problem, with `absolute_error_bound`, `relative_error_bound` and the fixed `relative_limit` of 10^-9;
  - checked support force and moment norms (`derived_check`), and section stress checks at five stations (`retained_source_straight_stress_v1`);
  - retained endpoint normal-stress maxima (`retained_source_endpoint_normal_max_v1`). These bound the source-action projection and the ordered binary64 arithmetic over the represented section operands. They do not bound primitive geometry or material accuracy.

  The source problem is formed from the case's represented source contributions; it is not the ordinary route's rounded aggregate.

  **Eligibility.** All of these must hold:
  - the solve captured its invocation;
  - current model 0.9.0 with the `3.0.0/exact_pressure_v3` contract;
  - no nonlinear supports and no combinations;
  - an explicitly empty pressure-region inventory;
  - at most 256 degrees of freedom;
  - the per-case and per-invocation work limits;
  - the ordinary report is Sensitive or the ordinary attempt errs;
  - the further limitations stated in the `pressure-source-1` table.

  A case with a load-row formation finding is declined and never selected.
- **AC-001** — REQ-001 holds for every committed successor, corpus entry and reader input:
  - the Rust reader classifies each row as REQ-001 states, and refuses each committed classification, scale and coverage mutation at its named gate;
  - the kernel's acceptance predicates and error-bound arithmetic match their independently generated vectors;
  - on every case the exact references cover, each published `relative_verified` or `absolute_verified` row of a selected successor lies within its stated bound of each stiffness readout's exact value;
  - the Python verifier reaches the same verdicts.
- **AC-002** — REQ-002 holds for every committed `pressure-source-1` publication and corpus entry:
  - the Rust reader and the Python verifier each return the corpus's stated outcome for every entry;
  - the retained-source census's classification, counts, first causes, selections and frozen-row verdicts are unchanged; its charged work changes only as a reviewed work-reduction change requires, with every charge still at least the executed work; its envelope and diagnostics digests change only as a stated product-text, identity, published-row or work change requires, and its results and summary digests only as a stated product-text, identity or published-row change requires;
  - the metered charges match their independent oracle;
  - on every case the exact references cover, each published interval contains the reference's exact value.

### Fallback and fixed limits

- **REQ-003** — The method falls back to ordinary results and relaxes no limit.
  - **(a) No retained work ran.** Where no retained work ran, the ordinary bytes are published exactly, with no notice. This covers:
    - an admission refusal (caller, domain, build identity or byte limit);
    - a stack-reservation fallback;
    - coexistence with retained-source recovery;
    - the late-gate and completion-gate refusals;
    - a domain or notice-reservation fallback;
    - an invocation with no triggered case.
  - **(b) Retained work ran but did not complete.** The case keeps its ordinary rows, standing and diagnostics byte for byte, and gains one `RETAINED_PRECISION_UNAVAILABLE` notice. That notice names a reason only for a receipt-encoding failure.
    - An invocation-level custody failure in preparation, or a combination-custody, staging, serializer or precommit fault, withdraws the whole successor, so every attempted case falls back with the notice.
    - A preparation, native or candidate failure of one case can leave other cases selected. The published successor's receipt then records that case, with its typed cause.
  - **(c) Retained-source attempt declined or failed.** The case stays on its ordinary route, and its `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic states the failure stage and cause. A `pressure-source-1` publication requires every case to qualify. If a captured invocation's selected source cannot finalize, whether or not the invocation carries a load/reference-state group, every case is republished on its ordinary route (`pressure-2`) without a receipt, and each case whose retained-source attempt is declined or fails in the republication states the first finalization failure in that diagnostic.
  - **(d) Ordinary attempt failed.** On route L only, for a single-case invocation without combinations, such a case publishes from the retained route only if its retained proof passes G0–G8. Its quality entry keeps the failed attempt's outcome and never reads `checks_passed`. Otherwise the blocked ordinary result stands, with no successor.
  - **(e) No relaxation.** No search, verification, tolerance or domain limit is relaxed to obtain a bound.
    - A search's stop tolerance bounds the search's own convergence on the problem it solves. Where a declared input is itself an interval (the admitted lane's represented-Z hull), the search converges at the interval's binding endpoint under the unchanged tolerance and limits, the input's width is carried soundly into the result's enclosure, and the unchanged gate decides publication.
    - The formation definitions fix the precision ladder (128 to 1024 bits), `projection_retry: false` and `search_or_retry: none`. The schema fixes `relative_limit` at 10^-9. These are reviewed inputs bound by digest, and the reader checks the digests at G0.
    - The D1 caps, the loaded-maximum search limits and the work limits are fixed in source. Their only overrides exist in test builds and cannot be enabled in a product or registered build, and a change to them is a priced-source change that needs D3 and independent review.
    - A caller limit above the qualified M is refused as `Unqualified`; it is never clamped.
- **AC-003** — Every refused, declined or faulted path behaves as REQ-003 states:
  - it publishes the exact ordinary bytes, or those bytes plus the one notice where retained work ran;
  - every stage fault, no-permit entry and refused exact request takes the ordinary route;
  - the reader refuses a changed definition or table digest at G0.

### Memory-pricing law

- **REQ-004** — Retained-precision invocations are admitted only within the registered memory profile.
  - **Admission.** An invocation is admitted only through the Direct entry, and only in a build whose exact compiled identity, reviewed-input digests and reader layouts match a registered profile. The caller supplies an immutable byte limit L. The invocation is admitted only when its priced requirement is within L and L ≤ M.
  - **M and its margins.** The registered profile's M is the owner-selected 12 GiB (12,884,901,888 bytes). A profile is registered only if every priced phase, in both modes, satisfies requested + moving + R + ⌈TEXT/20⌉ ≤ ⌊0.9 M⌋ = 11,596,411,699 bytes. Here R = 64 MiB of reserved stack, and ⌈TEXT/20⌉ is the 5 % TEXT reserve.
  - **Registration.** Registration is a reviewed source change. The pricing tool prices the actual source (D3) and reproduces the generated profile block. Any change to a priced source or to a reviewed input requires a new actual-source D3 audit.
  - **Other builds and limits.** Every other build identity reports `Stale` or `Missing` and takes the ordinary route. A caller limit above M is refused as `Unqualified`.
  - **Fixed D1 domain.** At most 32 nodes, 32 members, 32 supports and 4 materials; at most 3 load cases, each with at most 128 primitive loads; at most 2 combinations; and at most 3 result sets per invocation. A larger M admits no larger domain.
  - **What the margins mean.** They are qualification accounting. They are not measured resident memory, and they do not bound the whole desktop process.
- **AC-004** — The memory-pricing law holds:
  - **In the registered build identity:**
    - M = 12,884,901,888 B and ⌊0.9 M⌋ = 11,596,411,699 B;
    - in both modes, every priced phase fits requested + moving + R, plus the 5 % TEXT reserve, within ⌊0.9 M⌋;
    - the retained Direct peak stays within the profile's bounds.
  - **In an unregistered build,** the same tests assert `Stale` and the unchanged ordinary route.
  - The pricing tool's maintained test passes.

### Separate consumers

- **REQ-005** — Each published verified result can be checked by consumers separate from its solver. They check the published statement, its classes and the inputs they can rederive. They do not re-run the solve, replay the private proof or prove producer origin (DEF-O `trust`).
  - **(a) One product reader.** One reader, written in Rust, is the only product reader of these statements, in native and WebAssembly builds. The reader gives its result either as a gate and code (a refusal) or as a standing.
    - The reader applies these gates in order to a successor:
      - G0: producer, table and definition-digest bindings;
      - G1: receipt shape and hashes;
      - G2: encodings;
      - G3: coverage;
      - G4: diagnostics;
      - G5: native runs, the precision schedule and work accounting;
      - G5a and G5b: body scales and the p = 512 floor;
      - G5c: row classes;
      - G6: the recovery method;
      - G7: the base reader applied to the projected statement;
      - G8: rederivation of the invocation, model, materials and maps, the declared member-load ledger and the section allowances.
    - The same reader reads `pressure-source-1`.
    - A result's standing (`reader_verdict`) is numerically eligible only when the captured invocation is present. Otherwise it is `needs_recompute` or `unsupported`. Load/reference-state evidence also stays numerically ineligible.
    - The reader is separate from the solver, and the producer also runs it as its precommit check.
    - Each result format keeps only its current version; the reader refuses an older version by name.
  - **(b) Recomputation.** For a selected member-load case, Rust native, wasm and Python each recompute the declared entries and the complete exact ledger, and bind `selection.ledger_sha256`.
  - **(c) Test verifier.** A verifier written in Python reads the same statements and shares no code with the producer. No product module imports it, it imports nothing from product code, and it spawns no process.
  - **(d) Native/wasm comparison.**
    - The reader's native outcomes for committed, ordered input sets are pinned: R1 for retained-precision successors and R2 for the base readers, which covers `pressure-source-1`.
    - The wasm build returns the same outcome for every recorded input, in order.
  - **(e) Corpora.** The product reader, the test verifier and the desktop each read the single retained corpus and the pressure-source corpus. Every stated expectation of the retained corpus is required.
- **AC-005** — The separate consumers agree:
  - the verifier-independence test passes;
  - the pinned R1 and R2 native outcomes match their committed records, and the wasm build matches them input for input;
  - both corpora give their stated outcomes in Rust, Python and the desktop, and no stated expectation of the retained corpus is skipped;
  - the command-line binding and the Python binding return the reader's verdict.

### Stated limits

- **REQ-006** — These domains are declined and stated as limits, each publishing its ordinary results; (c) states which desktop outputs admit the exact-route and retained identities.
  - **(c) Desktop outputs for the current exact identity.**
    - An exact-route result (`pressure-2`, `pressure-source-1`) is Current on the desktop only at the Rust reader's numerically eligible standing, with the live native invocation and its matching record. Its result JSON export, stress-neutral export, rule check, rendered report and report package admit it, and each takes the reader's verdict and standing; the rendered report and the package carry DEL-08-01's exact-route report sections, and the package's boundary admits it with preview-physics-2. Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`). A `pressure-2` result carrying load/reference states takes the same standing: when eligible it is admitted, the two exports carry its resolved load/reference states, the rendered report and the package state them with their published limitations, and the admitting panels disclose them. A `pressure-source-1` result carrying them is never numerically eligible (`LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1`), so no desktop output admits it. The Python AnalysisRun record and the Python stress-neutral export accept them all.
    - If a retained-precision successor is opened on the desktop, every output surface refuses it (`RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE`), except the result JSON and stress-neutral exports, which admit it only at numerically eligible standing and with the live native capture of its exact bytes.
  - **(d) Outside the method's domain.**
    - **Retained-precision successors (both routes):** exclude curved members, nonlinear supports, directional springs and nonzero imposed motion (DEF-O, DEF-E); D1 also refuses components, equivalent-static cases, analysis states, modulus-basis temperatures and models beyond the D1 caps, and admits only rigid restraints and global scalar springs.
    - **Route L** also excludes pressure.
    - **Route E** also excludes present pressure regions, named-point modulus bases and combinations.
    - Arcs publish no rows on the retained identities.
    - **Retained-source recovery:** admits only straight circular members with linear restraints or springs. It declines present pressure regions (zero pressure included), components, nonlinear supports and non-nodal ordinary applied loads, and excludes combinations and models above 256 degrees of freedom.
  - **(e) The sharper allowance against both readouts.** A retained-precision successor publishes only when each `relative_verified` row's value, normalized to SI, lies within the kernel's sharper allowance of both stiffness readouts' exact values, and each `absolute_verified` row's within its absolute bound. Five properties of the method keep some rows from meeting it:
    - the admitted readout's bending stress ranges over hull(Ẑ, Î/c);
    - the admitted readout's binary64 section, shear and modulus leaves differ from the exact annulus readout's by up to a few binary64 units, and every stress row inherits the section leaves' difference;
    - a millimetre or megapascal value is rounded in its unit and again when normalized to SI;
    - an unloaded member's circular maximum publishes the ordinary binary64 value, and a support magnitude the norm of the rounded components, rather than the projection of the readouts' midpoint;
    - near an exact zero, the annular readout's enclosure can be wider than the absolute bound.

    A case with such a row keeps its ordinary result with the one `RETAINED_PRECISION_UNAVAILABLE` notice.
  - **(f) Loaded-member maxima too wide for the gate.** A loaded member's maximum whose certified hull is too wide for the unchanged gate keeps its ordinary result; the case is unavailable with its named cause.
- **AC-006** — Each decline in REQ-006 is exercised by a maintained test that asserts its refusal. The fallback tests and the coverage census tests assert the resulting ordinary result at the product entry. The desktop output policy refuses each named identity with its stated code, and the report package's Rust boundary refuses every identity except preview-physics-2, pressure-2 and pressure-source-1. The tables' limitation texts are bound by the reviewed-input digests.

### Activation

- **REQ-009** — No product surface produces a retained-precision successor until the owner activates the method on that surface; a surface that has not been activated takes the ordinary route (PRD §12.8).
- **AC-009** — On each product entry where the owner has not activated the method, a solve gives the ordinary-route outcome and no successor.

### Claim boundary

- **REQ-008** — A verified bound is a numerical statement about the problem the method records (PRD §12.8). PRD §21.2 governs the method's claims, and no product text of this deliverable may state or imply a claim it prohibits.
  - The method's product texts state the limit: "numerical integrity does not establish physical correctness" (route L), and "Source evidence and numerical standing do not authenticate a producer or grant engineering acceptance" (route E).
  - Each table's `accuracy_classification` and limitation texts state the domain the producer admits.
  - Identifiers that contain "certif" (for example `certified_gap_pa`, `certify_final` and `CertifiedRow`) are data contracts under §4 of the claims registry, and they keep their names.
- **AC-008** — The product texts this deliverable emits are pinned: the notice and diagnostic constants by maintained tests and committed successor bytes, and the limitation texts by the reviewed-input digests. Any change to them is reviewed against `docs/claims_registry.md`. Each table's scope text names the model and contract versions the producer admits on its route.
- **TBD-001** — *Terminology in product texts.*
  - **Where the words appear.**
    - The selected-case diagnostics use "certified classes" (`SELECTED_MESSAGE`), "the certificate certifies that declared-load problem" (`SELECTED_MEMBER_LOAD_MESSAGE`) and "its own solve and certificate" and "certified classes" (`COMBINATION_SELECTED_MESSAGE`).
    - The `preview-physics-retained-1` table's member-load limitation says "Retained-precision certificates certify that declared-load problem". The `pressure-source-1` table's maxima limitation says "do not certify primitive geometry or material accuracy".
    - The formation definitions and the tables' formation warrants use "certificate" throughout, as the technical name of the numerical error proof.
  - **The choice.** The texts can keep these words, or the user-facing ones can adopt "established" and "confirmed", as the kernel's product sentences do (DEL-04-01). The choice is the owner's.
  - **Consequences of a change.** The tables and definitions are reviewed inputs: changing their text changes their digests and requires a D3 re-registration. The messages are pinned in committed successor bytes, which would need to be regenerated.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *Two consumers, neither the solver.* A published verified result is checked by one product reader and by a test verifier that shares no code with the producer. Neither re-runs the solve or replays its proof; each checks the statement, its classes and the inputs it can rederive.
- *Reviewed inputs, bound by digest.* The formation definitions and semantic tables fix the precision ladder, the retry rules and the limitation texts. A publication carries their digests and the reader refuses any other.
- *Fallback is byte-exact.* A case the method does not complete is published with its ordinary bytes, plus at most one notice.
- *Admission before work.* An invocation is priced against the registered profile before any retained work runs. An unregistered build takes the ordinary route.
- *Registration-dependent checks.* A check that needs a registered build runs in that build; in any other build the same check asserts the stale verdict and the unchanged ordinary route.

**How each criterion is examined.**

- **VER-001** — Comparison with named references, with an executable check: the kernel's acceptance predicates and bound arithmetic against independently generated vectors; published values against the exact references `validation/references/o4_retained_source`, `kd5_timoshenko` and `m3_judge`, which read no product code; and the reader's and the test verifier's classification of every committed successor, corpus entry and mutation.
- **VER-002** — Comparison with named references, with an executable check: the reader's and the test verifier's outcome on every `pressure-source-1` publication and corpus entry; the retained-source census against its pinned classification, counts, first causes, selections and verdicts; the metered work charges against their independent oracle; and published intervals against the exact references of VER-001 on the cases they cover.
- **VER-003** — Executable check: observes, for each admission refusal, stage fault, declined or failed case and republication, the published bytes against the ordinary result, the single notice or diagnostic, and the reader's refusal of a changed definition or table digest.
- **VER-004** — Executable check: observes the profile's arithmetic and the measured peak of a retained invocation in the registered build, the stale verdict and ordinary route in an unregistered build, and the pricing tool's reproduction of the registered profile.
- **VER-005** — Executable check: observes that the test verifier imports no product code, that the native reader's outcomes equal the pinned R1 and R2 records and the WebAssembly build's outcomes equal them input for input, that both corpora give their stated outcomes in the reader, the verifier and the desktop with no expectation skipped, and that the bindings return the reader's verdict.
- **VER-006** — Executable check: observes each decline of REQ-006 as a refusal with the ordinary result standing, the desktop output policy's refusal of each named identity with its code, and the report package's refusal of every other identity.
- **VER-008** — Executable check with judgment by a reviewer who did not write the text: the check observes the pinned notice, diagnostic and limitation texts and compares each table's scope text with the model and contract versions the producer admits; the reviewer examines any change to them against `docs/claims_registry.md`.
- **VER-009** — Executable check: exercises the desktop solve, the headless retained entry and the command line, each on a surface the owner has not activated, and observes the ordinary-route outcome and the absence of a successor.

## Governing Values and Decisions — Axiology

- **AX-001** — *More results verified, no gate loosened.* Better is a larger share of the judged cases and rows that carry a verified bound, and a tighter bound on each. A gate, tolerance, search limit, domain limit or frozen reference is never changed to obtain a pass; a declined domain is stated as a limit, not approximated; a case that cannot be verified keeps its exact ordinary bytes and standing.
- **AX-003** — *Less memory per admitted invocation.* Better is a lower priced requirement for the same invocation. This is never bought by giving up qualification against the registered profile or by admitting a larger domain than the fixed one.
- **AX-006** — *Independent scrutiny.* Better is more of the method's statements checkable outside the producer, with fewer places where a verdict can be formed: a second product reader is a defect.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-080 | REQ-001 CLM-001 | AC-001 | VER-001 | Outside source, on the cases they cover: exact references `validation/references/o4_retained_source`, `kd5_timoshenko`, `m3_judge` and independently generated vectors, none reading product code. The committed successors and corpora are the product's own earlier output and only guard against change. For a case outside the references the bound rests on the method's internal proof: no independent grounds |
| OUT-001 | OBJ-003 SOW-080 | REQ-002 CLM-001 | AC-002 | VER-002 | Outside source, on the cases they cover: the exact references above and the independent work oracle. The census pins are the product's own earlier output and only guard against change. For a case outside the references the interval rests on the method's exact solve: no independent grounds |
| OUT-002 | OBJ-003 SOW-080 | REQ-003 | AC-003 | VER-003 | The criterion determines the expected bytes: the ordinary result of the same request |
| OUT-003 | OBJ-003 SOW-080 | REQ-004 | AC-004 | VER-004 | Owner ruling: the 12 GiB budget and its margins; the arithmetic is determined by the criterion |
| OUT-004 | OBJ-003 SOW-080 | REQ-005 | AC-005 | VER-005 | The criterion is agreement between separate consumers, which it determines; the pinned outcome records are the product's own earlier output and only guard against change |
| OUT-005 | OBJ-003 SOW-080 | REQ-006 CLM-002 | AC-006 | VER-006 | The criterion determines the expected refusals |
| OUT-005 | OBJ-003 SOW-080 | REQ-008 | AC-008 | VER-008 | Owner ruling: PRD §21.2 and `docs/claims_registry.md`; the pinned texts are the product's own earlier output |
| OUT-002 | OBJ-003 SOW-080 | REQ-009 CLM-003 | AC-009 | VER-009 | Owner ruling: PRD §12.8 reserves activation to the owner; the expected outcome is the ordinary result of the same request |
