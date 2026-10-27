# Opt-in Timoshenko straight members on the pressure-free route

**Status: design for Agent 0's review; not built.** The owner's steer of 2026-10-27 (#1662) asks for an app "on par with or considered superior to CAEPIPE and other industry standard applications... in terms of the modelling characteristics". Agent 0 adopted this unit under it: an explicit, opt-in choice of transverse shear deformation for straight members on the pressure-free (PF) route. The default stays Euler–Bernoulli, and every existing document and result stays byte-identical. The unit reuses the exact route's Timoshenko formulation, references and certification (D-6, [section-bases-and-timoshenko.md](section-bases-and-timoshenko.md) §3). Here, "the exact route" means documents with `pressure_contract`, and "PF" means documents without it.

The CAEPIPE correlation pilot (DEL-17-05 Design `caepipe-correlation-pilot.md` §7) classified the SWBPIPE–CAEPIPE differences in seven of its eight cases as shear-deformation differences (`dd-shear-deformation`). Only hand calculations are public: 0.043 mm at 3 m and 0.173 mm at 12 m for a shape factor of 2. CAEPIPE's shear factor for steel pipe is not established (CQ-17-01-010 is open). This unit gives PF the same element as the exact route; it does not adopt CAEPIPE's factor.

## 1. Selection and persistence

- **Field.** `model_settings.shear_model` ∈ {`euler_bernoulli`, `timoshenko_annulus_energy_v1`}, defined in model document 0.8.0 (#1649; Agent 0's ruling (a), 2026-10-27; W-17 implements the member).
  - Absent means Euler–Bernoulli, and the product never writes it.
  - An authored `null`, or any other value, is refused: `MODEL-SETTING-SHEAR-MODEL-UNKNOWN`.
  - The field is refused on a document with `pressure_contract` (`MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT`), so each document has one selection. An exact document keeps `pressure_contract.shear_model`.
- **0.8.0 interim.** `euler_bernoulli` is admitted and solves exactly as absent: same rows and mechanics, only the model hash differs. `timoshenko_annulus_energy_v1` is refused by name at solve and at authoring (`MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED`); the document validator admits it as schema-valid.
  - This unit removes that refusal in the solve and the applier. It needs no schema or version change. This is the pattern T4-U8 used for `pressure_contract.shear_model`.
- **Scope of the choice.** Per document, applying to every straight member, as H5 rules for the exact route (§3.6 there). Element-level selection is declined for now (Agent 0, 2026-10-27). Realized arcs and geometry-only bends stay Euler–Bernoulli under either value, as on the exact route; a tee's branch members are straight members and follow the selection.

## 2. Formulation (reused, unchanged)

- **Element.** FK `FrameSection::timoshenko` / `BeamTheory::Timoshenko { shear_area }`, through `StraightPipeSectionProperties::timoshenko`. The stiffness is section-bases-and-timoshenko.md §3.3 with φ = 12EI/(G·A_s·L²): axial EA/L and torsion GJ/L unchanged, and six rigid modes. No kernel, straight_pipe or curved_bend code changes.
- **Shear area: the exact route's definition** (§3.1 there). A_s = κ_E·A, with κ_E = `shear::annulus_energy_shear_factor(r_i, r_o, ν)` in its fixed operation order. It is energy-matched, not 5/6 and not Cowper.
  - **Annulus:** PF's stiffness section (`derive_pipe_section`), r_o = OD/2 and r_i = OD/2 − (t − c_m), with A its area. The exact route uses its stiffness section, the nominal one under D-5. The two coincide when the mill tolerance c_m is zero. PF has one section basis, so κ_E and A_s are of the same section as A and I.
  - **ν:** the material's authored `poisson_ratio`, or on a load-state document the resolved member pair's ν, exactly as the exact route reads it (`lib.rs` `build_model_for_members`). It is optional on PF materials today, so a Timoshenko PF document without it is refused by name before any route: `PF_TIMOSHENKO_POISSON_RATIO_REQUIRED`, naming the material. There is no default factor and no ν derived from E/(2G) − 1: the exact route derives none.
  - A κ_E or A_s out of range is refused as on the exact route.
- **Load equivalents** (§3.4 there):
  - Full-span uniform loads (authored, self-weight and contents) keep the Euler–Bernoulli vector, which is identical for a prismatic member.
  - Thermal and fit eigenstrains, nodal loads, springs, connectors and prescribed motions are unchanged.
  - A point member force does not exist on any route and is refused by the primitive by name.
  - Partial spans: §3 below.
- **Recovery.** f = K_T·d − feq, with the same feq as assembly. Station resultants are statics from the i-end action, so they are valid for either theory. No interior displacement is reconstructed. Published nodal rotations of a Timoshenko member are section rotations θ, not slopes; limitation 0 discloses it (§4).
- **Formation check.** PF already solves through `solve_assembled_with_formation_check`, and `frame_matrix` forms the Timoshenko element from the frame's own leaves (§4.1 there). So PF Timoshenko formations are checked by the same code. The frame-binding check (§4.2 there) covers PF's straight-load and recovery frames, since they carry the same leaf.
- **Storage.** κ_E and A_s are computed in `build_model_for_members` on the PF branch. `ExactSectionBases` is exact-only and is not reused for PF.

## 3. Interactions on PF: admitted, refused by name, unchanged

| Family | This unit |
|---|---|
| Straight members, tees' straight legs, W-15 rigid-family components (scaled s·E, s·G; φ is invariant under that scaling, and κ_E uses the authored ν) | Timoshenko when selected |
| Realized arcs, geometry-only bends | Euler–Bernoulli, disclosed |
| Full-span uniform loads, self-weight, contents, thermal, nodal loads, springs, connectors, tie rods | Admitted, unchanged vectors |
| Imposed displacement (W-16), combinations, the constant-effort gate | Admitted: partitioned elimination through the assembled K; superposition and the gate are independent of beam theory |
| Linear supports; nonlinear supports (one-way, gaps, friction) and constant-effort hangers | Admitted. The nonlinear adapter forms frames through the generic `local_stiffness`/`global_stiffness`, with no Euler–Bernoulli-specific code. These are new on a Timoshenko element (the exact route refuses nonlinear supports), so §5 adds reference cases |
| **Partial-span member loads** (wind's exposed spans) on a Timoshenko member | **Refused by name in this unit:** `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`, naming the member and load, before any route. straight_pipe has the exact Timoshenko partial-span vector, but the S11-G load-row guard and K-D5 record a Timoshenko partial span as unbounded or unavailable, which would demote every such case to Sensitive. A later slice can extend S11-G's intended formula and K-D5's recorded vector, with independent references, and remove the refusal |
| **Retained route L** (W1-L, D1) | **Declined by name at D1** (`family_clauses`) before any charged work, under a new family fact `ShearModel`. The document then takes the ordinary PF route, unchanged. W1-L refuses a Timoshenko leaf today (`RETAINED_TIMOSHENKO_REFUSAL`), and the D1 decline keeps that refusal unreached. Route L's domain (DEL-04-08 ScopeOfWork, "straight Euler–Bernoulli members…") stays as committed |
| Sparse and dense solver modes | Both, same formation |

## 4. Results, contract and readers

- **Euler–Bernoulli results are byte-identical**, both absent and explicit. They keep limitation 0, "Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.", and every table and schema pin.
- **A Timoshenko PF result** carries a fixed alternative limitation 0:

  > "Small-displacement, linear-elastic frame preview: straight members are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_v1), and realized arcs and geometry-only bends are Euler-Bernoulli; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness."

  It is selected only by the document's `model_settings.shear_model`.
- The preview-physics-2 table (reviewed input #5) gains the variant as the second admitted limitation-0 text. The results schema's `const` for profile `product_preview_mechanics_v1` becomes a two-member `enum`.
  - The Rust reader (`preview_physics_evidence.rs`) and the Python reader (`tests/verifiers/preview_physics_evidence.py`) admit the variant only when the invocation's model selects Timoshenko, and the Euler–Bernoulli text only when it does not. A crossed pair is refused (`FORMULATION_BASIS`).
  - The retained table (`…_preview_physics_retained_1`, #4) is unchanged: route L declines Timoshenko, so no retained result carries the variant.
- **No per-member section evidence is added.** PF publishes none today, and the readers re-derive the limitation from the document. A per-member `beam_theory` record like the exact route's is left open.
- **Stress, action and reaction rows are unchanged in kind and basis.** Only their values move for Timoshenko documents.

## 5. Verification

- **Byte identity (regression).** Every existing PF fixture, golden and corpus document gives byte-identical results (`pressure_free_byte_identity`, the corpus replay at main against head, and the full suites with main comparison). A document with `shear_model: euler_bernoulli` gives the same result rows as its twin without it.
- **Independent references.** A new `validation/references/pf_timoshenko/` is derived and frozen by a TASK that neither reads nor writes product code, then refuted by a second TASK, before any product output is compared with it (Plan 01 §5, as for t4_u8).
  - The generator extends t4_u8's `u8_reference.py` arithmetic: exact rationals of the binary64 inputs, κ_E from the authored ν, and an assembly and solve of its own. It emits PF 0.8.0 requests.
  - The cases:
    - t4_u8's S-B cantilevers at L = 1, 2 and 10 m, re-authored as PF documents with c_m = 0 and with c_m > 0;
    - a simply supported span under uniform load (shear term 5wL⁴/384EI + wL²/(8κGA));
    - an L-line with a realized arc (Euler–Bernoulli arc, Timoshenko tangents);
    - a skew multi-span run with an interior spring;
    - an imposed support displacement;
    - self-weight with contents;
    - a W-15 rigid component;
    - a one-way support that lifts off, and a gap that closes;
    - a combination.

    φ ranges from about 1e-4 to above 1. Both solver modes are covered, along with Euler–Bernoulli twins of every case.
  - **Tolerance:** t4_u8's, |obs − exp| ≤ 1e-9·|exp|, with discriminators more than 1e-7 from the Euler–Bernoulli value.
  - **Controls:**
    - the Euler–Bernoulli twins match the frozen Euler–Bernoulli expectation;
    - the Timoshenko product judged against the Euler–Bernoulli reference must breach at high φ;
    - a reference that took ν from E/(2G) − 1 must disagree where the authored ν differs from it.
- **Refusals by name**, each with a test: the interim refusal (removed by this unit; its test becomes the admission test), `PF_TIMOSHENKO_POISSON_RATIO_REQUIRED`, `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`, the D1 decline (ordinary route, unchanged result), and `MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT`.
- **Readers.** Rust and Python mirrors: the variant is admitted only with the Timoshenko selection, and crossed pairs are refused.
- **Formation check.** The PF Timoshenko reference cases report Passed. The M3-T premise census (ε_E/‖K̃_s‖₁ and the Hager ratio) is measured on them, and its Passed, Sensitive and fail-closed counts are reported against the Euler–Bernoulli twins. If a judged breach on a Passed case cannot be repaired, the unit stops and returns to Agent 0; the gate is never loosened.
- **Independent numerical scrutiny.** A Codex review covers the formulation wiring, the κ_E and ν basis, the refusals, the readers and the references against their generator.
- **Desktop.** The Model settings section gains "Straight-member shear deformation": Euler–Bernoulli (the default, written as absence) or Timoshenko (energy-matched annular factor), with a line stating that arcs stay Euler–Bernoulli. On an exact document the control is disabled and points to the pressure panel. There are Vitest tests for the intents, and the native smoke runs a Timoshenko solve, save and reopen.

## 6. D3, sequencing and size

- **Priced files touched:**
  - `core/product_physics/src/lib.rs` (the PF branch in `build_model_for_members`, refusals);
  - `retained_memory.rs` (D1's `ShearModel` family fact);
  - `preview_physics.rs` (the limitation variant);
  - `model_settings.rs`;
  - `core/reporting/result_export/src/preview_physics_evidence.rs` and `semantic_contract.rs` (table digest);
  - reviewed input #5 (preview-physics-2) and the results schema.

  No FK or straight_pipe source changes. A D1-declined document never reaches route L, so the priced route-L tree should move only by the D1 clause and the text. That is to be measured.
- **Order.** The unit builds after #1649 (0.8.0) merges. It joins the D3 queue after stage 2 (#1654), and is priced only on Agent 0's go. The AC-004 witnesses run if D1's change or the stack moves them.
- **Size.** About 400–600 lines of product, reader and desktop change, plus the reference generator and its frozen cases. That is one PR, with a second for partial spans if taken.

## 7. Commitment and design text

- **Commitment text (owner-reserved; exact old → new, Codex-reviewed, then to Agent 0).** DEL-05-03 `ScopeOfWork.md` CLM-008's last sentence:
  - old: "Pressure-free documents keep their single section and Euler–Bernoulli frame."
  - new: "Pressure-free documents keep their single section; their straight members are Euler–Bernoulli unless the document's `model_settings.shear_model` selects `timoshenko_annulus_energy_v1`, the same element and shear factor, and their arcs are Euler–Bernoulli."

  DEL-04-01 ScopeOfWork ("per document … (D-6)"), DEL-05-05 ScopeOfWork:51 and DEL-04-08 ScopeOfWork:103 stay true as written. The PRD's FR-SOL-003 is neutral on beam theory.
- **Design and description text, updated in the build PR:**
  - `section-and-shear-decisions.md` §D-6 (a PF paragraph pointing here);
  - `section-bases-and-timoshenko.md` line 3;
  - DEL-04-07 `deliverable.yaml`'s kernel line;
  - `docs/COMPLETENESS_MAP.md` FR-SOL-003 ("PF Euler–Bernoulli; EX adds Timoshenko" → PF opt-in);
  - FK `README.md` line 14, which still says the W1 element stays Euler–Bernoulli although O4 forms Timoshenko on route E.

## 8. Open

- A per-member `beam_theory` evidence record on PF results.
- The partial-span slice: S11-G and K-D5 for Timoshenko partial spans.
- A formulation-matched published benchmark for κ_E. This remains open, as for the exact route (§11 there).
- CAEPIPE correlation after the change: re-running the pilot's shear-deformation series is the owner's private comparison (DEL-17-05 §8), not a repository record.
