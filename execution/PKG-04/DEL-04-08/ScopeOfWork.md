---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-08
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:6fde28820074361ec81d6bd4a22b514e88fffda868d2591bc83e37eb728a4e89
project_scope_refs: [SOW-080]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-08 Retained-precision result verification

**Basis:** decomposition revision 0.15 (SOW-080, OBJ-003) and PRD §12.8, "Verified result bounds". On 2026-10-17 the owner approved these commitments and acceptance criteria: "I approve #1405 as required." They state what is built on `main` and what has been ruled; they assert no completion, acceptance or release. Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Purpose and Objective Traceability

This deliverable publishes verified numerical error bounds on solver results where the retained-precision method can establish them, and keeps the ordinary values and standing wherever it cannot. Consumers separate from the solver can check each published verified result. It serves OBJ-003 (a robust global centerline/frame solver) through SOW-080.

- **OUT-001** — The verified-quantity contract: the publication identities, receipts and row classes that state which result quantities carry a verified bound, and in what form (REQ-001, REQ-002).
- **OUT-002** — The fallback and fixed-limits rule, as the producer implements it and the reader checks it (REQ-003).
- **OUT-003** — The memory-pricing law, its registered profile and its qualification procedure (REQ-004).
- **OUT-004** — The separate consumers: the Rust product reader (native and wasm), the test-only Python verifier, and their comparison records and corpora (REQ-005).
- **OUT-005** — The stated limits, the open delivery duties and the claim boundary (REQ-006, REQ-007, REQ-008).

## Deliverable Definition — Ontology

### CLM-001 — The method

A verified bound states that a published value lies within a stated distance of the exact solution of the problem the method records (PRD §12.8). For a retained-precision result, that problem is the declared linear problem formed from the recorded inputs; for retained-source recovery, it is the represented source problem. The bound covers arithmetic and solution error only. Any difference between that problem and the authored inputs is disclosed, not bounded. The method has two publication families.

1. **Retained-precision successors.**
   - **Trigger.** The producer attempts a load case whose ordinary result is not `checks_passed`. It does not attempt cases whose ordinary attempt failed as a mechanism, an asymmetric system or invalid input, unless the W2 path published them (decision 21).
   - **Solve.** The case is re-solved at extended precision. Native candidates run at 128, 256 and 512 bits, each verified at twice its precision, and the product proof and projection run at 1024 bits.
   - **Publication.** When every check passes, a successor statement is published in place of the ordinary one. On route L, the pressure-free preview route, this is `preview-physics-retained-1`. On route E, the exact route with explicitly empty pressure regions, it is `physics-retained-1`.
   - **Receipt.** `schemas/retained_precision_mp_v2.schema.json`, policy `M03-INTEGRITY-MP-v2`.
   - **Formation definitions.** DEF-O, DEF-C and DEF-E (`fixtures/results/retained_precision_prepared_{ordinary,combination,exact}_v1.json`).
   - **Tables.** `fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json` and `semantic_contract_v0_3_physics_retained_1.json`.
   - **Entry.** `run_linear_static_preview_value_with_retained_direct` only, and only in a registered build (REQ-004).
2. **Retained-source recovery** (`pressure-source-1`; receipt policy `PRESSURE-SOURCE-1`, `schemas/physics_source_recovery.schema.json`). On the ordinary exact route of a solve that captured its invocation, an eligible case is solved exactly from its retained source blocks (`retained_source_blocks_exact_v1`, including the RF functional-entry and exact-congruence extension). A case is eligible when its ordinary report is Sensitive or its ordinary attempt errs. Each covered quantity is published as an outward interval.

### CLM-002 — What this deliverable owns, and what it takes as input

**This deliverable owns:**
- the two families' receipts, tables and formation definitions, and their publication and classification law;
- the producer's retained modules: `core/product_physics/src/retained_*.rs` and `build_identity.rs`, and the source-recovery and source-receipt modules;
- the Rust product reader for these statements and its CLI and wasm bindings: `core/reporting/result_export/src/retained_precision.rs`, `declared_member_load.rs`, `physics_source.rs` and `source_blocks.rs`;
- the test-only Python verifier, `tests/verifiers/`;
- the reader-outcome records and corpora;
- the memory-pricing law: `core/product_physics/src/retained_memory.rs` and `tools/retained_pricing`.

**Declared inputs** (`deliverable.yaml`):
- DEL-04-01: the kernel's retained lanes and their error proof;
- DEL-04-07: product solve integration and the memory-budget policy;
- DEL-05-01: primitive load records;
- DEL-05-05: the distributed user-load contract;
- DEL-05-03: section bases and stress definitions;
- DEL-05-02: combination semantics and the base-route readers;
- DEL-02-02: units;
- DEL-02-05: canonical JSON;
- DEL-00-06: the diagnostics contract;
- DEL-10-05: the headless runner.

These deliverables keep their scope. The code paths in `deliverable.yaml` indicate relevance only. Several are shared with these owners: `physics_source.rs`, for example, is also one of DEL-05-02's base-route readers.

### CLM-003 — Current reach, as built

- **Retained-source recovery.**
  - It runs in solves that go through the captured dispatch: the value entry `run_linear_static_preview_value_with_mode`, used by the desktop's native solve and `openpipestress-runner`, and the ordinary route of the retained Direct and Headless entries.
  - The typed entry (`run_linear_static_preview_with_mode`, used by `headless_preview_runner`) has no captured invocation and never attempts it.
  - The Python AnalysisRun record and the stress-neutral export accept its results. On the desktop an eligible result can be Current, and its result JSON and stress-neutral exports and rule check admit it; a `pressure-source-1` result carrying load/reference states is never eligible (REQ-006 (c)).
- **Retained-precision successors.**
  - They are produced only through the Direct entry, in a registered build.
  - No product caller uses that entry. The desktop solve uses the non-retained entry, because activation (B8) is owner-held. The headless retained entry refuses at D1.0, and the CLI solves on the ordinary route.
  - Only one build identity is registered: a debug build for aarch64-apple-darwin with rustc 1.97.1 at opt-level 0. Every other build, hosted CI builds included, reports `Stale`.
- **Readers.** Every reader surface takes the Rust reader's verdict:
  - the desktop, through wasm;
  - the CLI, through `openpipestress-runner read`;
  - Python product code, through `core/analysis_runs/rust_reader.py`, which has no fallback.

## Completion and Reliance Basis — Epistemology

In these commitments, "exact solution" means the exact solution of the recorded problem defined in CLM-001. Nothing here bounds modelling, input or physical error.

### Verified quantities

- **REQ-001** — *Retained-precision successors.*
  - **Classes.** Every published row of a successor is classified as exactly one of `relative_verified`, `absolute_verified`, `input_derived`, `non_quantity` or `not_covered`. The classes are defined by the table's `accuracy_classification` (policy `RP-FACADE-SI-v2`) and applied by the reader's `row_kind` and gate G5c.
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
    - These rows are `not_covered`: every other row kind; rows with no body scale; and, under R-COMB-1, each quantity row of an unavailable combination or of an ordinary combination that names a load case whose trigger is not `not_required`. They keep their ordinary values, except the section normal-stress extremes and transverse-shear maxima, which are formed in binary64 from the successor's own published stress, section-action and end-force rows and its captured section and material.
    - Intensified bending stresses carry no verified class: the reader refuses a successor that classifies one (G5b).
  - **Route L domain.**
    - It covers straight Euler–Bernoulli members, nodal loads and full-span uniform member loads, rigid restraints and global scalar springs, and base or named-point E and G. DEF-O and DEF-C also define interpolated E and G, which D1 refuses at admission.
    - With member loads, the bound covers the declared-load problem. Each declared entry lies within 2^-1022 of its exact consistent value, relative to that value. That difference is disclosed but not bounded.
    - It also covers mechanics combinations under the B2-C contract. Each combination has its own solve on the combined exact ledger and at least one selected operand. It publishes exactly 7n + 50m + 8g rows (n nodes, m members, g attributed supports), of which constrained displacement components are `input_derived` +0.
    - Combination maxima, intensified rows, range envelopes, state differences and combinations with member-load operands are not covered.
  - **Route E domain.** It covers direct load cases with nodal loads only, under these conditions:
    - current model 0.8.0 with the explicit `3.0.0/exact_pressure_v3` contract;
    - explicitly empty pressure regions;
    - straight members;
    - base common E and ν;
    - either explicit shear model (`euler_bernoulli` or `timoshenko_annulus_energy_v1`) on straight members, and any admitted mill tolerance and corrosion allowance, under the D-5 bases: stiffness and the Timoshenko shear area on the nominal wall, and each stress row on its own basis section.
- **REQ-002** — *Retained-source recovery.* For each case it qualifies, a `pressure-source-1` receipt publishes:
  - for nodal translations and rotations, member end and station actions, support action components and constraint reactions (`projection.quantity`): an outward interval that contains the exact value of the case's source problem, with `absolute_error_bound`, `relative_error_bound` and the fixed `relative_limit` of 10^-9;
  - checked support force and moment norms (`derived_check`), and section stress checks at five stations (`retained_source_straight_stress_v1`);
  - retained endpoint normal-stress maxima (`retained_source_endpoint_normal_max_v1`). These bound the source-action projection and the ordered binary64 arithmetic over the represented section operands. They do not bound primitive geometry or material accuracy.

  The source problem is formed from the case's represented source contributions; it is not the ordinary route's rounded aggregate.

  **Eligibility.** All of these must hold:
  - the solve captured its invocation;
  - current model 0.8.0 with the `3.0.0/exact_pressure_v3` contract;
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
  - the Python verifier reaches the same verdicts.
- **AC-002** — REQ-002 holds for every committed `pressure-source-1` publication and corpus entry:
  - the Rust reader and the Python verifier each return the corpus's stated outcome for every entry;
  - the RF census classification, counts, first causes, selections and frozen-row verdicts are unchanged; its charged work changes only as a reviewed work-reduction change requires, with every charge still at least the executed work; its envelope and diagnostics digests change only as a stated product-text, identity, published-row or work change requires, and its results and summary digests only as a stated product-text, identity or published-row change requires (owner, 2026-10-20, approving this wording for R-A, Design `source-work-nondyadic-inputs.md` §6);
  - the metered charges match their independent oracle.

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
    - The D1 caps, the loaded-maximum search limits and the work limits are fixed in source. Their only overrides are test-only (`#[cfg(test)]` code, or the opt-in `loaded-maximum-hooks` feature that only the genuine-exhaustion control's command enables and no product or registered build can), and a change to them is a priced-source change that needs D3 and independent review.
    - A caller limit above the qualified M is refused as `Unqualified`; it is never clamped.
- **AC-003** — Every refused, declined or faulted path behaves as REQ-003 states:
  - it publishes the exact ordinary bytes, or those bytes plus the one notice where retained work ran;
  - every stage fault, no-permit entry and refused exact request takes the ordinary route;
  - the reader refuses a changed definition or table digest at G0.

### Memory-pricing law

- **REQ-004** — Retained-precision invocations are admitted only within the registered memory profile.
  - **Admission.** An invocation is admitted only through the Direct entry, and only in a build whose exact compiled identity, reviewed-input digests (`REVIEWED_INPUTS`, 12 files) and reader layouts match an entry of `REGISTERED_PROFILES`. The caller supplies an immutable byte limit L. The invocation is admitted only when its priced requirement is within L and L ≤ M.
  - **M and its margins.** The registered profile's M is the owner-selected 12 GiB (12,884,901,888 bytes). A profile is registered only if every priced phase, in both modes, satisfies requested + moving + R + ⌈TEXT/20⌉ ≤ ⌊0.9 M⌋ = 11,596,411,699 bytes. Here R = 64 MiB of reserved stack, and ⌈TEXT/20⌉ is the 5 % TEXT reserve.
  - **Registration.** Registration is a reviewed source change. `tools/retained_pricing` prices the actual source (D3), and `generate --check` reproduces the generated profile block. Any change to a priced source or to a reviewed input requires a new actual-source D3 audit.
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
  - **(a) One product reader, in Rust.** Rust is the only product reader of these statements, in native and wasm builds. The reader gives its result either as a gate and code (a refusal) or as a standing.
    - `retained_precision::validate` applies the gates in order to a successor:
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
    - `physics_source.rs` and `source_blocks.rs` read `pressure-source-1`.
    - A result's standing (`reader_verdict`) is numerically eligible only when the captured invocation is present. Otherwise it is `needs_recompute` or `unsupported`. Load/reference-state evidence also stays numerically ineligible.
    - The reader is a separate crate from the solver, and the producer also runs it as its precommit check.
  - **(b) Recomputation.** For a selected member-load case, Rust native, wasm and Python each recompute the declared entries and the complete exact ledger, and bind `selection.ledger_sha256`.
  - **(c) Test verifier.** `tests/verifiers/` reads the same statements in Python and shares no code with the producer. No module outside `tests/` imports it, it imports nothing from product code, and it spawns no process.
  - **(d) Native/wasm comparison.**
    - The reader's native outcomes for committed, ordered input sets are pinned by Rust tests. R1 is `fixtures/results/retained_precision_reader_outcomes.json`. R2 is `fixtures/results/base_reader_outcomes.json`, which covers the `pressure-source-1` readers.
    - The wasm build returns the same outcome for every recorded input, in order.
  - **(e) Corpora.** The Rust reader, the Python verifier and the desktop each read the single retained corpus, `fixtures/results/retained_precision_cases.json`, and the pressure-source corpus, `fixtures/results/pressure_source_reader_corpus.json`. Every stated expectation of the retained corpus is required (D8).
- **AC-005** — The separate consumers agree:
  - the verifier-independence test passes;
  - the pinned R1 and R2 native outcomes match their committed records, and the wasm build matches them input for input;
  - both corpora give their stated outcomes in Rust, Python and the desktop, and no stated expectation of the retained corpus is skipped;
  - the CLI binding and Python `rust_reader` return the reader's verdict.

### Stated limits and open duties

- **REQ-006** — These domains are declined and stated as limits. Each publishes its ordinary results.
  - **(a)** [retired by O4, #1468]
  - **(b)** [retired by the W1 coverage repair, #1484]
  - **(c) Desktop outputs for the current exact identity.**
    - An exact-route result (`pressure-2`, `pressure-source-1`) is Current on the desktop only at the Rust reader's numerically eligible standing, with the live native invocation and its matching record. Its result JSON export, stress-neutral export, rule check, rendered report and report package admit it, and each takes the reader's verdict and standing; the rendered report and the package carry the exact-route sections of DEL-08-01 Design `exact-route-report-sections.md`, and the package's Rust boundary admits it with preview-physics-2 (owner, 2026-10-26: "My two decisions are to approve your recommendations in both cases. Proceed accordingly."). Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`). A `pressure-2` result carrying load/reference states takes the same standing: when eligible it is admitted, the two exports carry its resolved load/reference states, the rendered report and the package state them with their published limitations, and the admitting panels disclose them. A `pressure-source-1` result carrying them is never numerically eligible (`LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1`), so no desktop output admits it. The Python AnalysisRun record and the Python stress-neutral export accept them all.
    - No desktop solve produces a retained-precision successor before B8. If one is opened on the desktop, every output surface refuses it (`RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE`), except the result JSON and stress-neutral exports, which admit it only at numerically eligible standing and with the live native capture of its exact bytes.
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

    A case with such a row keeps its ordinary result with the one `RETAINED_PRECISION_UNAVAILABLE` notice. On O4's frozen judged set, the design-time ceiling admits no sensitive document under the method's projection with enclosures as narrow as the existing proof's (Design `w1e-sharper-exact-coverage.md`).
  - **(f) Loaded-member maxima too wide for the gate.** A loaded member's maximum whose certified hull is too wide for the unchanged gate keeps its ordinary result; the case is unavailable with its named cause. Recorded under the owner's decision of 2026-10-20 ("I approve 1A, 2A, 3A."; Design `w1-coverage-repair.md` §10).
- **REQ-007** — These delivery duties are open unless marked discharged. Each is delivered under its own design and independent review, keeps every gate unchanged, and is re-registered under D3 wherever it changes a priced source or a reviewed input.
  - **(a) O4: the Timoshenko and corrosion-allowance source extension.** This is the fifth shared epoch in the owner's order of 2026-10-14. It extends retained-source and W1-E publication to the documents REQ-006 (a) declined, including those with a nonzero mill tolerance. It keeps RF's closure, witness and charging rules. **Discharged on 2026-10-20.** O4 is on main through #1468. The stack-headroom repair (#1476, b7-plan item 5a) makes the AC-004 stack-headroom witnesses pass, completing AC-007's conditions. HELP_HUMAN records this discharge under the owner's decision of 2026-10-20 ("All as recommended").
  - **(b) The W1 coverage repair.** The owner said on 2026-10-17: "We will need to address that W1 coverage gap at some point." It is planned as the sixth shared epoch in this deliverable's `Design/b7-plan.md` ("After B7b"). The epoch starts with a census of how often current sections and maxima reach each REQ-006 (b) corner, then repairs the cause with every gate unchanged. Implementation follows O4 unless that design shows the two should merge. **Discharged on 2026-10-20.** The repair is on main through #1483 and #1484: corner 2 (the receipt states the product gate's final scales) and corner 1 (the admitted lane's search converges at the represented-Z hull's binding endpoint), with every gate, tolerance and limit unchanged (Design `w1-coverage-repair.md`). The decline tests they retire are replaced by admission and refusal tests (VER-003, VER-006). HELP_HUMAN records this discharge under the owner's decision of 2026-10-20 ("I approve 1A, 2A, 3A.").
  - **(c) Desktop outputs for the current exact identity.** This is a follow-on unit. The owner chose "Merge with the gap" on 2026-10-15, and on 2026-10-20 placed the unit as epoch 8 in this deliverable's `Design/b7-plan.md`, after epoch 7, with its design started alongside epoch 7. T4-U4b/U5b replaced `pressure-1` with `pressure-2`. **Discharged on 2026-10-21.** The desktop outputs are on main through #1505: the result JSON export, the stress-neutral export and the rule check admit `pressure-2` and `pressure-source-1` results at the Rust reader's numerically eligible standing, an eligible `pressure-2` load/reference-state result included, each taking the reader's verdict and standing; every other desktop output surface refuses them (DEL-04-07 Design `desktop-exact-outputs.md`). AC-005 and the desktop output-policy tests pass on that head. Recorded under the owner's decision of 2026-10-21 ("A accept your recommendations for the entire set.").
- **AC-006** — Each decline in REQ-006 is exercised by a maintained test that asserts its refusal. The fallback tests and the W1 census tests assert the resulting ordinary result at the facade. The desktop output policy refuses each named identity with its stated code, and the report package's Rust boundary refuses every identity except preview-physics-2, pressure-2 and pressure-source-1. The headless retained entry refuses at D1.0. The tables' limitation texts are bound by the reviewed-input digests.
- **AC-007** — A duty in REQ-007 is discharged only when the conditions for it hold. Until then it remains listed here as open.
  - **(a) O4 and (b) the W1 repair:**
    - the cases they extend to publish verified results under REQ-001 or REQ-002;
    - the decline tests they retire are replaced by admission and refusal tests;
    - the AC-001 to AC-005 checks pass on their head, with every gate, limit and frozen reference unchanged;
    - their actual-source D3 registration passes.
  - **(c) Desktop outputs:** the output surfaces it opens take the Rust reader's verdict and standing, as every reader surface does (CLM-003), and AC-005 and the desktop output-policy tests pass on its head.

### Claim boundary

- **REQ-008** — A verified bound is a numerical statement about the problem the method records (PRD §12.8). PRD §21.2 governs the method's claims, and no product text of this deliverable may state or imply a claim it prohibits.
  - The method's product texts state the limit: "numerical integrity does not establish physical correctness" (route L), and "Source evidence and numerical standing do not authenticate a producer or grant engineering acceptance" (route E).
  - Identifiers that contain "certif" (for example `certified_gap_pa`, `certify_final` and `CertifiedRow`) are data contracts under §4 of the claims registry, and they keep their names.
- **AC-008** — The product texts this deliverable emits are pinned: the notice and diagnostic constants by maintained tests and committed successor bytes, and the limitation texts by the reviewed-input digests. Any change to them is reviewed against `docs/claims_registry.md`. The claims-language lint (GEN-13) passes over the literal desktop policy texts of `outputPolicy.ts`.
- **TBD-001** — *Terminology in product texts.*
  - **Where the words appear.**
    - The selected-case diagnostics use "certified classes" (`SELECTED_MESSAGE`), "the certificate certifies that declared-load problem" (`SELECTED_MEMBER_LOAD_MESSAGE`) and "its own solve and certificate" and "certified classes" (`COMBINATION_SELECTED_MESSAGE`).
    - The `preview-physics-retained-1` table's member-load limitation says "Retained-precision certificates certify that declared-load problem". The `pressure-source-1` table's maxima limitation says "do not certify primitive geometry or material accuracy".
    - The formation definitions and the tables' formation warrants use "certificate" throughout, as the technical name of the numerical error proof.
  - **The choice.** The texts can keep these words, or the user-facing ones can adopt the nonlinear K-D5 ruling R5's wording ("established", "confirmed"). The choice is the owner's.
  - **Consequences of a change.** The tables and definitions are reviewed inputs: changing their text changes their digests and requires a D3 re-registration. The messages are pinned in committed successor bytes, which would need to be regenerated.
- **TBD-002** — *A stale reviewed-input sentence.* The `physics-retained-1` table's `accuracy_classification.scope` still reads "model 0.3.0 with the 2.0.0/exact_straight_pressure_v2 contract". DEF-E and the code admit current model 0.8.0 with `3.0.0/exact_pressure_v3`. Correcting the sentence changes a reviewed input and requires D3.

## Production and Verification Method — Praxeology

The registered checks are those in `software-workflow.json`.
- **Rust tests.** Among the registered checks, the Rust tests of the crates named below run in `macos-release-sweep`'s cargo crate sweep, which runs `cargo test` for each crate. `piping-pytest` only builds the runner binary that its tests need. On pull requests, the hosted `harness` numerical job runs the selected crates.
- **Desktop tests.** `desktop-test` needs the wasm engine built first (`npm run build:wasm:desktop`), as the sweep does. Hosted pull-request CI runs desktop Vitest when the diff selects `piping-desktop` (`docs/CI_STRATEGY.md`), and on dispatch.
- **Registration-dependent tests.** The registered branch runs only in the registered build identity, which is the qualification host. Hosted builds are unregistered, so there these tests assert only the `Stale` branch.
- **Host rules.** Heavy runs follow the host's memory and slot rules.

- **VER-001** — For REQ-001:
  - `macos-release-sweep`:
    - `core/reporting/result_export/tests/retained_precision_contract.rs` (including `current_shared_corpus_contract`, `current_corpus_requires_bound_and_must_pass_declarations` and `d13_absolute_bound_small_scale_switch`), `retained_precision_carriers.rs`, `retained_precision_derivative_golden.rs` and `declared_member_load_workspace.rs`;
    - `core/model_operations/operation_applier/tests/retained_precision_reader.rs` and `retained_precision_semantics.rs`;
    - `core/product_physics/src/retained_product_tests.rs` and `retained_wire_tests.rs`;
    - the kernel's `core/solver/frame_kernel/tests/retained_k4/` suite, which compiles into the crate's tests and runs the classification, bound, error-bound arithmetic and loaded-maximum tests against generated vectors.
  - `piping-pytest`: `tests/test_retained_precision_contract.py`, `test_retained_precision_b2.py`, `_b3`, `_carriers`, `_failed_ordinary`, `_schema` and `_si_first`, and `tests/test_declared_member_load_reader.py`.
- **VER-002** — For REQ-002:
  - `macos-release-sweep`:
    - `core/reporting/result_export/tests/pressure_source_contract.rs` and `source_blocks.rs`;
    - `core/product_physics/tests/pressure_source_join.rs`;
    - `core/product_physics/src/source_recovery/j7_rf_census.rs` (`j7_rf_census_classification_and_pin`);
    - `core/solver/frame_kernel/src/structural/exact_boundary/metered_tests.rs` (`metered_mul_charges_match_the_independent_oracle`).
  - `piping-pytest`: `tests/test_pressure_source_verifier.py`, `test_pressure_source_corpus.py` and `test_j7_rf_census_inputs.py`.
  - `desktop-test`: `apps/desktop/src/features/results/pressureSourceCorpusParity.test.ts`.
- **VER-003** — For REQ-003, in `macos-release-sweep`:
  - `core/product_physics/src/retained_facade_tests.rs`: `u3_each_stage_fault_falls_back_to_the_ordinary_bytes`, `u3_no_permit_entries_are_the_ordinary_route`, `u3g2_direct_entry_w1_fallbacks_append_one_notice`, `b3b_refused_exact_requests_take_the_ordinary_route`, `o4_w1e_timoshenko_case_is_admitted_and_selected` and `o4_w1e_dual_basis_case_is_admitted`;
  - `core/product_physics/src/w1_domain_stage1_tests.rs`: `w1_represented_z_hull_converges_on_ordinary_sections`, `w1_loaded_scale_halfway_publishes` and `w1_native_scales_stay_native_evidence`;
  - `o4_timoshenko_source_case_selects_and_publishes` in `core/product_physics/src/source_recovery.rs`, and `o4_dual_basis_source_case_selects` in `core/product_physics/src/source_receipt/o4_source_tests.rs`;
  - `core/product_physics/src/b7b_admission_tests.rs`;
  - `core/product_physics/src/source_receipt/load_state_fallback_tests.rs`, both routes' republication: on 0.4, `a_selected_case_whose_own_finalization_fails_after_the_reservation_falls_back` and `reviewer_scenario_b_composite_with_sensitive_pressure_case_publishes_the_ordinary_route`; on pre-0.4, `a_pre04_composite_finalization_failure_republishes_ordinarily`, `a_pre04_case_finalization_failure_republishes_ordinarily`, `a_support_norm_refusal_is_the_stated_cause`, `a_republished_case_that_needed_no_recovery_states_no_cause` and `a_republished_case_whose_ordinary_attempt_failed_keeps_its_blocked_result`; and `pre04_selection_takes_the_replay_screen` in `core/product_physics/src/source_recovery.rs`; and `u3g2_direct_entry_republishes_a_source_that_cannot_finalize` in `core/product_physics/src/retained_facade_tests.rs`;
  - the reader's G0 table and definition tests (`b2_g0_table_tests` in `core/reporting/result_export/src/retained_precision.rs`).
- **VER-004** — For REQ-004:
  - **Registered branch:** `macos-release-sweep` on the qualification host's registered build identity. **Stale branch:** the hosted `harness` numerical job. Both run:
    - `core/product_physics/src/retained_memory_law_tests.rs` (`profile_laws_hold_in_this_build`);
    - `retained_memory_witness_tests.rs`;
    - the `registered()` branches of `retained_facade_tests.rs`;
    - `core/product_physics/tests/retained_memory_challenge.rs` (`retained_direct_peak_is_within_the_profiles_bounds`).
  - `piping-pytest`: `tests/test_retained_pricing_tool.py`.
  - The tool's own tests under `tools/retained_pricing/tests/` are not part of any registered check.
- **VER-005** — For REQ-005:
  - `piping-pytest`: `tests/test_verifier_independence.py`, `tests/test_base_reader_verifier.py`, `tests/test_rust_reader.py` and `tests/test_retained_precision_corpus_storage.py`;
  - `macos-release-sweep`:
    - `r1_native_reader_outcomes` in `core/reporting/result_export/tests/retained_precision_contract.rs`;
    - `core/reporting/result_export/tests/base_reader_outcomes.rs` and `corpus_storage.rs`;
    - `core/runner/headless/tests/reader_cli.rs`;
  - `desktop-test`: `apps/desktop/src/features/results/retainedPrecisionWasmEquivalence.test.ts`, `baseReaderWasmEquivalence.test.ts` and `apps/desktop/src/test-support/retainedPrecisionCorpus.test.ts`.
- **VER-006** — For REQ-006:
  - the VER-003 decline and fallback tests, and the facade-level W1 census tests `core/product_physics/src/w1_source_census_tests.rs` and `w1_successor_census_tests.rs` (`macos-release-sweep`);
  - `desktop-test`: `apps/desktop/src/features/results/outputPolicy.test.ts`, `retainedPrecisionOutputRefusal.test.tsx`, `loadReferenceOutputRefusal.test.tsx`, `apps/desktop/src/features/report/reportPackageRequest.test.ts` and `apps/desktop/src/services/ruleCheckService.test.ts`;
  - the report package's identity refusals: `core/reporting/report_package/tests/current_result_package.rs`, exercised by `cargo test --manifest-path core/reporting/report_package/Cargo.toml --test current_result_package` and run in hosted CI by manifest selection (`core/reporting/report_package/Cargo.toml`; `tools/hosted-ci-routing.json`);
  - `macos-release-sweep`: `core/runner/headless/tests/retained_precision_admission.rs` and `core/product_physics/tests/retained_precision_admission.rs`;
  - for (e), `macos-release-sweep`: `o4_w1e_sharper_allowance_refusal_keeps_the_ordinary_result` in `core/product_physics/src/o4_m3_judging_tests.rs`.
  - for (f), `macos-release-sweep`: `w1_loaded_maximum_too_wide_for_the_gate_keeps_the_ordinary_result` in `core/product_physics/src/w1_domain_stage1_tests.rs`.
- **VER-007** — For REQ-007, on the head of each duty's own change:
  - VER-001 to VER-006;
  - for O4 and the W1 repair, the actual-source D3 run (`tools/retained_pricing`, `generate --check`);
  - for O4, M3-O4 on `validation/references/o4_retained_source/` (`core/product_physics/src/o4_m3_judging_tests.rs`) and its declared-problem oracle (`declared_problem/run.sh`);
  - for the desktop outputs, the desktop rule-check backend's exact-route tests in `apps/desktop/src-tauri/src/lib.rs`'s test module, run with `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` and named with their head in the pull request;
  - the independent review that the Root `AGENTS.md` requires for numerical changes.
- **VER-008** — For REQ-008:
  - `macos-release-sweep`: the pinned notice and successor-byte tests in `retained_facade_tests.rs` and `w1_domain_stage1_tests.rs`;
  - `harness-self-check`, and Repository checks in hosted CI when one of its inputs changes: GEN-13, repository-root `tools/validation/validate_claims_language.py`. GEN-13 scans the desktop TS sources, `docs/` and every `execution/PKG-*/DEL-*/ScopeOfWork.md`, this one included, not Rust constants or JSON tables;
  - `piping-pytest`: `tests/test_claims_language_surface.py`.

  Each text change is also reviewed against `docs/claims_registry.md`.

## Governing Values and Decisions — Axiology

- **AX-001** — *One product reader.* The owner ruled on 2026-10-10: "move to one product reader in Rust". Asked whether that covered the base-route readers, the owner answered: "All readers." Python is kept only as an independent test verifier that shares no code with the producer.
- **AX-002** — *No compatibility.* The owner ruled on 2026-10-10: "as you recommended, no-compatibility principle carries." Each result format keeps only its current version, and older versions are refused by name.
- **AX-003** — *Memory budget.*
  - The owner set the product's per-invocation M at up to 12 GiB on the 16 GiB base machine. Current qualification uses M = 12 GiB, with the 0.9 M margin, the 5 % TEXT reserve and the 64 MiB stack reservation, all unchanged. The owner also adopted a shared pool of 75 % of physical RAM (DEL-04-07 `Design/memory-budget-decision.md`).
  - Host testing may use up to 64 GiB. That figure is a test ceiling, not the product M.
  - Decision 22 keeps these for the owner: changes outside the 12 GiB base allowance and the shared 75 % pool policy, larger numerical domains, and risk acceptance.
- **AX-004** — *Held by the owner:* B8 (retained-route activation on the desktop), the G10 interpretation, further changes to the frozen UI benchmark (re-pinned by the owner, 2026-10-22), and release. This deliverable activates nothing.
- **AX-005** — *No relaxation.* A gate is never loosened to obtain a pass ("do not change the law to obtain a pass", W1 Design §11). A declined domain is stated as a limit, not approximated. A case that cannot be verified keeps its ordinary values and standing.
- **AX-006** — *Independent scrutiny.* Numerical changes to this method need targeted regression checks and independent review by a separate agent (Root `AGENTS.md`). Frozen numerical references are never altered to make an implementation pass.
- **AX-007** — *Terminology.*
  - This ScopeOfWork uses "verified bound" for the method's numerical statements, and "proof" for the kernel's internal error proof.
  - "Certified", "certificate" and "certify" appear here only in identifiers, in quoted product or Design text, and in TBD-001.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 SOW-080 | REQ-001 REQ-002 CLM-001 | AC-001 AC-002 | VER-001 VER-002 | Rust, Python and desktop verdicts on the committed successors, corpora and kernel vectors, on the candidate head |
| OUT-002 | OBJ-003 SOW-080 | REQ-003 | AC-003 | VER-003 | Fallback, decline and G0 tests showing exact ordinary bytes, the single notice and the internal refusals |
| OUT-003 | OBJ-003 SOW-080 | REQ-004 | AC-004 | VER-004 | Law and witness tests in the registered build identity and in an unregistered build; actual-source D3 for any priced change |
| OUT-004 | OBJ-003 SOW-080 | REQ-005 CLM-003 | AC-005 | VER-005 | Verifier-independence test, R1 and R2 native records, matching wasm outcomes, D8 corpora |
| OUT-005 | OBJ-003 SOW-080 | REQ-006 REQ-007 REQ-008 CLM-002 | AC-006 AC-007 AC-008 | VER-006 VER-007 VER-008 | Decline and output-policy tests, each duty's own head checks and review, pinned texts and claims lint |
