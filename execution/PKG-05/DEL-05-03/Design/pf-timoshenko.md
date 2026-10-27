# Opt-in Timoshenko straight members on the pressure-free route

**Status: design for Agent 0's review; not built.** The owner's steer of 2026-10-27 (#1662) asks for an app "on par with or considered superior to CAEPIPE and other industry standard applications... in terms of the modelling characteristics". Agent 0 adopted this unit under it: an explicit, opt-in choice of transverse shear deformation for straight members on the pressure-free (PF) route. The default stays Euler–Bernoulli, and every existing document, result, table hash and recorded run stays byte-identical. The unit reuses the exact route's Timoshenko element and shear-factor definition (D-6, [section-bases-and-timoshenko.md](section-bases-and-timoshenko.md) §3). Here, "the exact route" means documents with `pressure_contract`, and "PF" means documents without it. Codex round 1 reviewed an earlier draft (`b272696f71`); this text answers its findings.

The CAEPIPE correlation pilot (DEL-17-05 Design `caepipe-correlation-pilot.md` §7) classified the SWBPIPE–CAEPIPE differences in seven of its eight cases as shear-deformation differences (`dd-shear-deformation`). Only hand calculations are public. CAEPIPE's shear factor for steel pipe is not established (CQ-17-01-010 is open). This unit gives PF the same element and factor definition as the exact route; it does not adopt CAEPIPE's factor.

## 1. Selection and persistence

- **Field.** `model_settings.shear_model` ∈ {`euler_bernoulli`, `timoshenko_annulus_energy_v1`}, defined in model document 0.8.0 (#1649; Agent 0's ruling (a), 2026-10-27; W-17 implements the member).
  - Absent means Euler–Bernoulli, and the product never writes it.
  - An authored `null`, or any other value, is refused: `MODEL-SETTING-SHEAR-MODEL-UNKNOWN`.
  - The field is refused on a document with `pressure_contract` (`MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT`), so each document has one selection. An exact document keeps `pressure_contract.shear_model`.
- **0.8.0 interim.** `euler_bernoulli` is admitted and solves exactly as absent: same rows and mechanics, only the model hash differs. `timoshenko_annulus_energy_v1` is refused by name at solve and at authoring (`MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED`).
  - This unit removes that refusal in the solve and the applier. It needs no schema or version change; T4-U8 used the same pattern for `pressure_contract.shear_model`.
- **Scope of the choice.** Per document, applying to every straight frame member, as H5 rules for the exact route. That includes tees' straight legs, W-15 rigid-family components and **geometry-only bends**, which PF analyses as their mapped straight frame (`lib.rs` ~8703–8714). Only **realized arcs** (`curved_bend_realized_pipe_ids`, the curved-bend consumption mode) stay Euler–Bernoulli, as on the exact route (`lib.rs` ~8411). Element-level selection is declined for now (Agent 0, 2026-10-27).

## 2. Formulation

- **Element (reused, unchanged).** FK `FrameSection::timoshenko` / `BeamTheory::Timoshenko { shear_area }`, through `StraightPipeSectionProperties::timoshenko`. The stiffness is section-bases-and-timoshenko.md §3.3 with φ = 12EI/(G·A_s·L²): axial EA/L and torsion GJ/L unchanged, and six rigid modes. No kernel, straight_pipe or curved_bend code changes.
- **Shear-factor definition: the exact route's.** A_s = κ_E·A, with κ_E = `shear::annulus_energy_shear_factor(r_i, r_o, ν)` in its fixed operation order, and `shear::shear_area`. The factor is energy-matched, not 5/6 and not Cowper. Each route applies the definition to its own constitutive and section basis.
  - **Annulus.** PF's stiffness section, computed by PF's existing arithmetic (`derive_pipe_section`): r_o = OD/2 and r_i = OD/2 − (t − c_m), with A its area. The exact route uses its stiffness (nominal) section. With zero mill tolerance the two coincide geometrically but not necessarily bit for bit, because the routes' area arithmetic differs by ulps. PF keeps its own arithmetic, so its Euler–Bernoulli bytes are untouched.
  - **ν: the one that gives the member's G.** The exact route takes the ν of "the E/ν basis that gives its G": it resolves G from E and ν, and ignores a redundant authored G with a warning (`pressure_material.rs` ~78–91). PF authors E and G independently. So on PF, the ν that gives G is ν = E/(2G) − 1, computed in binary64 from the **unscaled** moduli in the fixed order PF's stress recovery already uses (`u4b_rows.rs` ~573, `lib.rs` ~6786–6791). PF then has one ν for the shear stiffness (κ_E) and the published transverse-shear stresses.
    - E and G are resolved per case by PF's existing temperature selection (base, named point, interpolation), so ν follows the case with no resolver change. A W-15 scaled member uses its unscaled moduli.
    - An authored `poisson_ratio`, which is optional on PF materials, is not consumed. If it is present and differs from E/(2G) − 1 by more than 1e-6 relative, the result carries a warning diagnostic, `PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED`, naming the material, both values and the case basis. That mirrors the exact route's warning for its unused input. Euler–Bernoulli documents emit nothing new.
    - A derived ν for which `annulus_energy_shear_factor` or `shear_area` refuses (not finite, or out of the factor's range) blocks the solve by name, as on the exact route. There is no default factor.
- **Load equivalents** (§3.4 there):
  - Full-span uniform loads (authored, self-weight and contents) keep the Euler–Bernoulli vector, which is identical for a prismatic member.
  - Thermal and fit eigenstrains, nodal loads, springs and prescribed motions are unchanged.
  - A point member force does not exist on any route.
  - Partial spans: §3.
- **Recovery.** f = K_T·d − feq, with the same feq as assembly. Station resultants are statics from the i-end action, so they are valid for either theory. No interior displacement is reconstructed. Published nodal rotations of a Timoshenko member are section rotations θ, not slopes; the limitation discloses it (§4).
- **Formation check.** PF already solves through `solve_assembled_with_formation_check`. `frame_matrix` forms the Timoshenko element from the frame's own leaves, and the member-identity frame binding carries the leaf (`formation_check.rs` ~1238, ~2605). So PF Timoshenko formations are checked by the same code. Nonlinear cases additionally pass the converged-state confirmation.
- **Storage.** κ_E and A_s are computed in `build_model_for_members` on the PF branch. `ExactSectionBases` is exact-only and is not reused for PF.
- **W-15 scaling.** φ is invariant under s·E, s·G in exact arithmetic. In binary64 the two scaled moduli round separately (`lib.rs` ~8438–8445), so φ may differ by ulps; the references judge it at the tolerance below.

## 3. Interactions on PF: admitted, refused by name, unchanged

| Family | This unit |
|---|---|
| Straight members, geometry-only bends (mapped straight frames), tees' straight legs, W-15 rigid-family components | Timoshenko when selected |
| Realized arcs | Euler–Bernoulli, disclosed |
| Full-span uniform loads, self-weight, contents, thermal, nodal loads, springs | Admitted, unchanged vectors |
| Imposed displacement (W-16), combinations, the constant-effort gate | Admitted: partitioned elimination through the assembled K; superposition and the gate are independent of beam theory |
| Linear supports; nonlinear supports (one-way, gaps, friction) and constant-effort hangers | Admitted. The nonlinear adapter forms frames through the generic `local_stiffness`/`global_stiffness` (`structural_adapter.rs` ~1343). These are new on a Timoshenko element, so §5 adds reference cases for each, friction stick and slide included |
| Objective connectors and their tie rods | Unchanged: PF refuses an authored objective connector today (`PREVIEW_CONTRACT_VERSION_MISMATCH`, `pressure_runtime.rs` ~267–285; connector construction is exact-v3-only). Annotation joints analysed as pipe follow the straight-member rule |
| **Partial-span member loads** (wind's exposed spans) on a Timoshenko member | **Refused by name:** `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`, naming the member and load, before any route. straight_pipe has the exact Timoshenko partial-span vector. But the S11-G guard records it as `CannotBound` (`straight_pipe/src/lib.rs` ~767), and K-D5 as an unavailable reformation (`formation_check.rs` ~2230), so every such case would be demoted to Sensitive. A later slice can extend both, with independent references, and remove the refusal |
| **Retained route L** (W1-L, D1) | **Declined by name at D1**, PF only, under a new family fact `ShearModel`, before any charged work. The document takes the ordinary PF route unchanged. This keeps W1-L's shear-sidecar refusal (`retained_product.rs` ~2121–2125) unreached, and route L's committed domain ("straight Euler–Bernoulli members…", DEL-04-08 ScopeOfWork) true. Exact route E keeps its existing Timoshenko admission |
| Sparse and dense solver modes | Both, same formation |

## 4. Results, contract and readers: an additive identity

Changing the preview-physics-2 table would move its hash. Every recorded run bound to that hash would then fail compatibility (`core/analysis_runs/compatibility.py` ~845), and the hash is pinned in five places. So a Timoshenko PF result gets its **own semantic contract identity**, and nothing about Euler–Bernoulli results changes.

- **Euler–Bernoulli results** (absent or explicit) keep `semantic_contract_v0_3_preview_physics_2`, profile `product_preview_mechanics_v1`, limitation 0 "Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.", and every byte and hash.
- **A Timoshenko PF result** carries:
  - a new table, `semantic_contract_v0_3_preview_physics_timoshenko_1`. It is preview-physics-2's rows, vocabulary and policies, with limitation 0 replaced by "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_v1) and Poisson's ratio E/(2G) - 1, and realized arcs are Euler-Bernoulli; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness.";
  - a new results profile, `product_preview_mechanics_timoshenko_v1`, whose schema `const` is that text.

  The table is derived mechanically from preview-physics-2 by `semantic_contract_tables.py` (a new `derive` step with `--check`), never hand-copied.
- **Readers.**
  - **Model-free** (raw `preview_physics_evidence.rs` ~259 and its Python mirror ~299; the transport validator `retained_precision.rs` ~6657/6689 and its Python mirror ~543): each admits either complete identity, meaning table, profile and limitation array together. A mixed array is refused (`FORMULATION_BASIS`).
  - **Model-bound** (`reader_verdict.rs` ~250, `tests/verifiers/reader_verdict.py` ~313): a crossed pair is refused, i.e. a Timoshenko identity on a model without the selection, or the Euler–Bernoulli identity on a model with it.
  - **Compatibility and AnalysisRun.** `compatibility.py` (both), `analysis_run.v0.3.schema.json` and `stress_neutral_export.v0.3.schema.json` gain the new identity's hash beside the old one. They do not replace it, so existing recorded runs stay compatible.
  - **Desktop.** `contractBindings.ts` computes from the table and gains the new identity.
  - **Retained.** The retained table (`…_preview_physics_retained_1`) is unchanged.
- **Reader limits (stated, not hidden).** The reader binds the declared selection to the identity. It does not re-derive a member's κ_E, A_s or ν, because PF publishes no section evidence. So an implementation error in κ is caught by the references (§5), not by readers. A per-member record (case basis, ν, κ_E, A_s) is left open (§8).
- **Stress, action and reaction rows are unchanged in kind and basis.** Only their values move for Timoshenko documents.

## 5. Verification

- **Byte identity (regression).** Every existing PF fixture, golden and corpus document gives byte-identical results, and every existing table and schema hash is unchanged. The checks are `pressure_free_byte_identity`, the corpus replay at main against head, and the full suites with main comparison. A document with `shear_model: euler_bernoulli` gives the same rows as its twin without it.
- **Physical references** (independent). A new `validation/references/pf_timoshenko/` is derived and frozen by a TASK that neither reads nor writes product code, then refuted by a second TASK, before any product output is compared with it (Plan 01 §5).
  - The generator extends t4_u8's `u8_reference.py` arithmetic: exact rationals of the binary64 inputs, ν = E/(2G) − 1 and κ_E from them, and an assembly and solve of its own. It emits PF 0.8.0 requests.
  - The cases:
    - t4_u8's S-B cantilevers (L = 1, 2, 10 m) re-authored as PF documents, with c_m = 0 and c_m > 0;
    - a simply supported span under uniform load **with a midpoint node**, so the shear term wL²/(8κGA) in v_mid = 5wL⁴/384EI + wL²/(8κGA) is observable at a node;
    - φ = 2 and adjacent φ (2 ± a few ulps of L), where (2 − φ) cancels;
    - an L-line with a realized arc (an Euler–Bernoulli arc between Timoshenko tangents), and a geometry-only bend;
    - a skewed multi-span run with an interior spring;
    - an imposed support displacement;
    - self-weight with contents;
    - a W-15 rigid component;
    - a one-way support that lifts off, a gap that closes, and a friction support in both stick and slide;
    - a combination.

    The φ range runs from about 1e-4 to above 2. Both solver modes are covered, along with Euler–Bernoulli twins of every case, and a material whose authored ν differs from E/(2G) − 1 (κ from the derived ν; the warning present).
  - **Tolerance:** t4_u8's policy, |obs − exp| ≤ 1e-9·|exp| for nonzero rows, and dimensional zero scales for zero rows (`u8_reference.py` ~2889–2893). Discriminators must lie more than 1e-7 from the Euler–Bernoulli value and from the authored-ν value.
  - **Controls:**
    - the Euler–Bernoulli twins match the frozen Euler–Bernoulli expectation;
    - the Timoshenko product judged against the Euler–Bernoulli reference must breach at high φ;
    - judged against an authored-ν reference, it must breach where the two ν differ.
- **Certificate soundness (M3-T, carried to PF).** This is kept separate from physical agreement, by M3-T's method (section-bases-and-timoshenko.md §4.5).
  - A judged set of PF Timoshenko models, generated by the independent generator in exact or ≥ 80-digit arithmetic, covers:
    - cantilevers, L-lines with sharp corners, skewed multi-span runs, grounded and interior springs, prescribed motions, self-weight and contents;
    - φ from about 1e-6 to above 10, clustered within ulps of 2;
    - each topology at large coordinate offsets and skewed there;
    - stiff-contrast and short-member models near the Sensitive boundary;
    - Euler–Bernoulli twins, both solver modes.
  - **Criterion (M3's):** 0 judged member, section, support or stress breaches on Passed cases, meaning the bound B ≥ the actual group error, and 0 strict coverage failures.
  - **Premise census:** A1's Hager ratio (estimate against the exact inverse norm where the model is small enough) and ε_E/‖K̃_s‖₁ against the go/no-go 2^-40, with Passed, Sensitive and fail-closed counts against the Euler–Bernoulli twins.
  - **Stop conditions:** if a judged breach on a Passed case cannot be removed by a sound repair, or the census shows A1 or the ε_E go/no-go failing in a way that is not fail-closed, the unit stops and returns to Agent 0. The gate is never loosened.
- **Refusals and warnings by name**, each with a test:
  - the interim refusal (its test becomes the admission test);
  - `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`;
  - the factor-range block;
  - `PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED`;
  - the D1 decline (ordinary route, unchanged result);
  - `MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT`;
  - the objective connector's existing refusal on a Timoshenko document.
- **Readers.** Rust and Python mirrors: both identities admitted model-free, mixed arrays refused, crossed pairs refused model-bound, and existing recorded runs compatible.
- **Independent numerical scrutiny.** A Codex review covers the wiring, the ν and κ basis, the refusals, the readers and the references against their generator.
- **Desktop.** The Model settings section gains "Straight-member shear deformation": Euler–Bernoulli (the default, written as absence) or Timoshenko (energy-matched annular factor). A line states that realized arcs stay Euler–Bernoulli and that Poisson's ratio is E/(2G) − 1. On an exact document the control is disabled and points to the pressure panel. There are Vitest tests for the intents, and the native smoke runs a Timoshenko solve, save and reopen.

## 6. D3, sequencing and size

- **Priced surfaces touched:**
  - `core/product_physics/src/lib.rs` (the PF branch in `build_model_for_members`, ν, refusals, warning);
  - `retained_memory.rs` (D1's `ShearModel` fact);
  - `preview_physics.rs` (the identity selection);
  - `model_settings.rs`;
  - `core/reporting/result_export/src/preview_physics_evidence.rs`, `retained_precision.rs` (the transport validator), `reader_verdict.rs` and `semantic_contract.rs` (a new resident parsed table, if it is cached like the others);
  - the results schema and the new table.

  The preview-physics-2 and retained tables are unchanged. Route L's Euler–Bernoulli invocations still load whatever resident tables the readers cache, so a new resident table is a priced static owner even though Timoshenko documents decline D1. The pricing inventory covers the shared readers, resident table growth and the source-bound rules; the footprint is measured, not assumed. No FK or straight_pipe source changes.
- **Order.** The unit builds after #1649 (0.8.0) merges. It joins the D3 queue after stage 2 (#1654), and is priced only on Agent 0's go. The AC-004 witnesses run if D1's change or the stack moves them.
- **Size.**
  - About 600–900 lines of product, reader, table-derivation and desktop change.
  - The reference generator and its frozen physical and judged sets.
  - One build PR, and a second for partial spans if taken.

## 7. Commitment and design text

- **Commitment text (owner-reserved; exact old → new, Codex-reviewed, then to Agent 0).** DEL-05-03 `ScopeOfWork.md` CLM-008's last sentence:
  - old: "Pressure-free documents keep their single section and Euler–Bernoulli frame."
  - new: "Pressure-free documents keep their single section; their straight members are Euler–Bernoulli unless the document's `model_settings.shear_model` selects `timoshenko_annulus_energy_v1`, the same element and shear-factor definition with Poisson's ratio E/(2G) − 1, and their realized arcs are Euler–Bernoulli."

  DEL-04-01 ScopeOfWork ("per document … (D-6)"), DEL-05-05 ScopeOfWork:51 and DEL-04-08 ScopeOfWork:103 stay true as written. The PRD's FR-SOL-003 is neutral on beam theory.
- **Design and description text, updated in the build PR:**
  - `section-and-shear-decisions.md` §D-6 (a PF paragraph pointing here);
  - `section-bases-and-timoshenko.md` line 3;
  - DEL-04-07 `deliverable.yaml`'s kernel line;
  - `docs/COMPLETENESS_MAP.md` FR-SOL-003 ("PF Euler–Bernoulli; EX adds Timoshenko" → PF opt-in);
  - FK `README.md` line 14, which still says the W1 element stays Euler–Bernoulli although O4 forms Timoshenko on route E.

## 8. Open

- A per-member PF evidence record (case basis, ν, κ_E, A_s) that would let readers re-derive the factor.
- The partial-span slice: S11-G and K-D5 for Timoshenko partial spans.
- A formulation-matched published benchmark for κ_E. This remains open, as for the exact route (§11 there).
- CAEPIPE correlation after the change: re-running the pilot's shear-deformation series is the owner's private comparison (DEL-17-05 §8), not a repository record.
