# Opt-in Timoshenko straight members on the pressure-free route

**Status: approved to build after Codex round 2 (Agent 0, 2026-10-27), with the rulings recorded below; built in #1663 (product, readers, desktop and report on the merged halves), under review.** The owner's steer of 2026-10-27 (#1662) asks for an app "on par with or considered superior to CAEPIPE and other industry standard applications... in terms of the modelling characteristics". Agent 0 adopted this unit under it: an explicit, opt-in choice of transverse shear deformation for straight members on the pressure-free (PF) route. The default stays Euler–Bernoulli, and every existing document, result, table hash and recorded run stays byte-identical. The unit reuses the exact route's Timoshenko element and shear-factor definition (D-6, [section-bases-and-timoshenko.md](section-bases-and-timoshenko.md) §3). Here, "the exact route" means documents with `pressure_contract`, and "PF" means documents without it. Codex rounds 1 (`b272696f71`), 2 (`6f2fcf675e`), 3 (`9be7209fa4`) and 4 (`1f1a2ab30c`) reviewed earlier drafts; this text answers their findings.

The CAEPIPE correlation pilot (DEL-17-05 Design `caepipe-correlation-pilot.md` §7) classified the SWBPIPE–CAEPIPE differences in seven of its eight cases as shear-deformation differences (`dd-shear-deformation`). Only hand calculations are public. CAEPIPE's shear factor for steel pipe is not established (CQ-17-01-010 is open). This unit gives PF the same element and factor definition as the exact route; it does not adopt CAEPIPE's factor.

## 1. Selection and persistence

- **Field.** `model_settings.shear_model` ∈ {`euler_bernoulli`, `timoshenko_annulus_energy_v1`}, defined in model document 0.8.0 (#1649; Agent 0's ruling (a), 2026-10-27; W-17 implements the member).
  - Absent means Euler–Bernoulli. The product never writes the default unprompted. An **explicit** user set of `euler_bernoulli` writes the member, and clear restores absence. This is the tolerance member's rule (never written into a document the user did not author), as refined on #1649 (Codex r7, option A, Agent 0, 2026-10-27). Absent and explicit Euler–Bernoulli solve identically.
  - An authored `null`, or any other value, is refused: `MODEL-SETTING-SHEAR-MODEL-UNKNOWN`.
  - The field is refused on a document with `pressure_contract` (`MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT`), so each document has one selection. An exact document keeps `pressure_contract.shear_model`.
- **0.8.0 interim.** `euler_bernoulli` is admitted and solves exactly as absent: same rows and mechanics, only the model hash differs. `timoshenko_annulus_energy_v1` is refused by name at solve and at authoring (`MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED`).
  - This unit removes that refusal in the solve and the applier. It needs no schema or version change; T4-U8 used the same pattern for `pressure_contract.shear_model`.
- **Named consumer: the MBF importer** (DEL-17-04, the CAEPIPE lane; Agent 0, 2026-10-27). It gives pressure imports `timoshenko_annulus_energy_v1` by default, with a caller `shear_model` override. Once this unit lands, it applies the same rule to pressure-free imports by writing `model_settings.shear_model = timoshenko_annulus_energy_v1`, so an import's beam theory never depends on whether pressure is present. This unit keeps for it:
  - the member's name, its two values and their meaning;
  - its admission on PF documents, and its refusal alongside `pressure_contract`;
  - the explicit-set rule (an explicit value is written, the default never unprompted).

  The CAEPIPE lane coordinates when this unit nears merge.
- **Scope of the choice.** Per document, applying to every straight frame member, as H5 rules for the exact route. That includes tees' straight legs, W-15 rigid-family components and **geometry-only bends**, which PF analyses as their mapped straight frame (`lib.rs` ~8703–8714). Only **realized arcs** (`curved_bend_realized_pipe_ids`, the curved-bend consumption mode) stay Euler–Bernoulli, as on the exact route (`lib.rs` ~8411). Element-level selection is declined for now (Agent 0, 2026-10-27).

## 2. Formulation

- **Element (reused, unchanged).** FK `FrameSection::timoshenko` / `BeamTheory::Timoshenko { shear_area }`, through `StraightPipeSectionProperties::timoshenko`. The stiffness is section-bases-and-timoshenko.md §3.3 with φ = 12EI/(G·A_s·L²): axial EA/L and torsion GJ/L unchanged, and six rigid modes. No element, straight_pipe or curved_bend code changes; the FK shear module gains the moduli form of the factor (below).
- **Shear-factor definition: the exact route's.** A_s = κ_E·A, with κ_E the energy-matched annular factor of `shear::annulus_energy_shear_factor`, not 5/6 and not Cowper, and `shear::shear_area`. Each route applies the definition to its own constitutive and section basis. PF forms the same value from its moduli without 1 + ν (the factor bullet below); the exact route's function and operation order are unchanged.
  - **Annulus.** PF's stiffness section, computed by PF's existing arithmetic (`derive_pipe_section`): r_o = OD/2 and r_i = OD/2 − (t − c_m), with A its area. The exact route uses its stiffness (nominal) section. With zero mill tolerance the two coincide geometrically but not necessarily bit for bit, because the routes' area arithmetic differs by ulps. PF keeps its own arithmetic, so its Euler–Bernoulli bytes are untouched.
  - **ν: the one that gives the member's G.** The exact route takes the ν of "the E/ν basis that gives its G": it resolves G from E and ν, and ignores a redundant authored G with a warning (`pressure_material.rs` ~78–91). PF authors E and G independently. So on PF, the ν that gives G is ν = E/(2G) − 1, computed in binary64 from the **unscaled** moduli in the fixed order PF's stress recovery already uses (`u4b_rows.rs` ~573, `lib.rs` ~6786–6791). PF then has one ν for the shear stiffness (κ_E) and the published transverse-shear stresses. **Ruling (Agent 0, 2026-10-27, under the owner's engineering-judgment delegation):** "On PF, G is authored, so κ_E uses ν = E/(2G) − 1 from the case's unscaled moduli. That keeps the shear stiffness consistent with the G the solve uses, and PF's stress recovery already does the same." Codex round 2 found it numerically faithful to the exact factor definition: the exact route starts from E and ν and derives G, PF starts from its authored E and G and derives ν.
    - E and G are resolved per case by PF's existing temperature selection (base, named point, interpolation), so ν follows the case with no resolver change. A W-15 scaled member uses its unscaled moduli.
    - **Admission follows consumption.** PF first builds the model on the base materials (`lib.rs` ~2956), then rebuilds per selected basis (~3160). A base build whose moduli no published case consumes must not impose the factor check. Otherwise base E = 210 GPa and G = 60 GPa (ν = 0.75) would block a document whose every case selects E = 200 GPa and G = 80 GPa (ν = 0.25).
      - **Construction boundary.** The set of consumed bases is known from the document's case temperature selections before any build.
        - If a case consumes the base basis, the seed build (basis state zero) forms its straight members as Timoshenko on the base moduli. A factor failure there is a blocking diagnostic naming that case and the base basis, as for any consumed basis.
        - If no case consumes the base basis, the seed build forms every member Euler–Bernoulli. It is complete for the shared topology and model diagnostics (`lib.rs` ~2956–2965), and its stiffness is assembled as main does (~3034, `assemble_basis_stiffness`). It is never solved for a published case and never selected as a basis, so no factor is formed for it. Later seed reuse (`preview_physics::render`) reads only topology, supports and member metadata.
        - Each consumed basis's rebuild (~3160) forms its Timoshenko members on its own moduli. Its factor failures and ν warnings are local to that basis and are emitted only for the cases that select it.
        - No solved member ever takes a fallback factor or an Euler–Bernoulli leaf in place of a failed Timoshenko one.
      - Regressions cover a named-point selection and an interpolated selection with an out-of-range base ν, both with a nonlinear support; a document whose cases consume both the base and a selected basis; and an out-of-range selected ν, which is blocked by name.
    - An authored `poisson_ratio`, which is optional on PF materials, is not consumed. If the material's own `poisson_ratio` is present and |ν_a − ν_d| > 1e-6·max(|ν_a|, |ν_d|), with ν_a authored and ν_d derived, the result carries a warning diagnostic, `PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED`, naming the material, both values and the case basis, once per material and basis. A temperature point's `poisson_ratio` is not compared (§8). Two zeros, or two equal values, give no warning. The threshold is for disclosure only, separate from numerical tolerances. Its purpose matches the exact route's warning for its unused G, which is unconditional (`pressure_material.rs` ~84); the trigger differs. Euler–Bernoulli documents emit nothing new.
    - **The factor from the moduli** (Codex review of the build; Agent 0, 2026-10-27: the cancellation is removed, not refused). Re-forming 1 + ν from ν = fl(E/(2G)) − 1 loses up to 2^-54/(E/(2G)) relative below E/(2G) = 1/2, and κ_E ∝ (1+ν)² there: Codex's E = 1 GPa, G = 1.5·2^52 GPa (E/(2G) ≈ 7.4e-17) published a deflection 55.6% low. PF therefore calls `shear::annulus_energy_shear_factor_of_moduli(r_i, r_o, E, G)`, which never forms 1 + ν:
      - **Identity.** c = 8ν² + 14ν + 7 = 7(1+ν)² + ν², so with m = r_i/r_o, a = 1 + m², b = 1 − m² and t = ν/(1+ν), κ_E = 6a²/((7 + t²)b² + 48m²): the module form divided through by (1+ν)², equal to SHEAR_REFERENCE's (7+14ν+8ν²)(1+m²)² + 4m²(5+10ν+4ν²) form. On PF, 1 + ν = E/(2G) exactly, so t = 1 − 2G/E.
      - **Order** (binary64, no fused multiply-add): x = fl(2G)/E, t = 1 − x, D = (7 + t·t)·(b·b) + 48·m², κ_E = (6·(a·a))/D, with m, m², a, b as in the existing function.
      - **Bound.** The domain ν < 1/2 gives x > 2/3. For x in [1/2, 2], 1 − x is exact (Sterbenz), so 7 + t² carries at most 2u relative (u = 2^-53). For x > 2, |1 − x| ≥ x/2, so x's rounding is amplified at most twice: t carries 3u, t² 7u and 7 + t² 8u. Every later operation adds or multiplies positive terms, so to first order |κ̂/κ_E − 1| ≤ (21 + 6β)u, with β = m²/(1 − m²) the section's condition (shared with the existing function), for every E/(2G) in (0, 3/2). On a grid from E/(2G) = 1.4 to 1e-300 on seven annuli (ν within ulps of 0, both Sterbenz edges, the domain edge, 2^-13, 2^-54 and both sides of the overflow below), its 1,512 formed vectors meet the bound against the exact rational contract value, the worst at 0.251 of it, and the rest are refused as stated (`frame_kernel/tests/shear_moduli_factor.rs`, vectors from `shear_moduli_vectors.py`). Sampled by band of E/(2G) on six annuli, no band is worse than the 1 + ν form, which reaches 2.4e16 u near ν = −1.
      - **Admission is unchanged.** The range check and every reported ν keep ν = fl(E/(2G)) − 1 (`preview_poisson_ratio`), so the admitted set does not move: E/(2G) ≥ 3/2 (ν ≥ 1/2) and E/(2G) ≤ 2^-54, where that ν rounds to −1, block by name. Codex's model (E/(2G) ≈ 7.4e-17) is admitted and now agrees with its independent exact deflection to 6e-16 relative.
      - **Representability backstop**, from binary64 range, not chosen thresholds: D is not normal exactly when t·t overflows, at E/(2G) ≲ 7.458e-155; with D finite, κ_E ≥ 6/D is normal; then `shear_area` (A_s normal) and the element's checked G·A_s, (G·A_s)·L² and φ formations. All lie far below PF's 2^-54 admission edge, where κ_E ≈ 6a²(E/(2G))²/b² is still of order 1e-30 for the reference annulus, so on PF they are unreachable backstops.
    - A derived ν outside the factor's range (not finite, or outside −1 < ν < 1/2), or a refusal of the factor or `shear_area`, blocks the solve by name, `PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE`, naming the member, the case basis and the refusal, as on the exact route. There is no default factor.
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
| Realized arcs | Euler–Bernoulli, never silently: the identity's limitation and a named per-result record (§4) |
| Full-span uniform loads, self-weight, contents, thermal, nodal loads, springs | Admitted, unchanged vectors |
| Imposed displacement (W-16), combinations, the constant-effort gate | Admitted: partitioned elimination through the assembled K; superposition and the gate are independent of beam theory |
| Linear supports; nonlinear supports (one-way, gaps, friction) and constant-effort hangers | Admitted. The nonlinear adapter forms frames through the generic `local_stiffness`/`global_stiffness` (`structural_adapter.rs` ~1343). These are new on a Timoshenko element, so §5 adds reference cases for each, friction stick and slide included |
| Objective connectors and their tie rods | Unchanged: PF refuses an authored objective connector today (`PREVIEW_CONTRACT_VERSION_MISMATCH`, `pressure_runtime.rs` ~267–285; connector construction is exact-v3-only). Annotation joints analysed as pipe follow the straight-member rule |
| **Partial-span member loads** (wind's exposed spans) on a Timoshenko member | **Refused by name:** `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`, naming the member and load, before any route. straight_pipe has the exact Timoshenko partial-span vector. But the S11-G guard records it as `CannotBound` (`straight_pipe/src/lib.rs` ~767), and K-D5 as an unavailable reformation (`formation_check.rs` ~2230), so every such case would be demoted to Sensitive. It joins a later slice beside the solver lane's exact-route partial-span item, so the two share one formation check (Agent 0, 2026-10-27) |
| **Retained route L** (W1-L, D1) | **Declined by name at D1**, PF only, under a new family fact `ShearModel`, before any charged work. The document takes the ordinary PF route unchanged. This keeps W1-L's shear-sidecar refusal (`retained_product.rs` ~2121–2125) unreached, and route L's committed domain ("straight Euler–Bernoulli members…", DEL-04-08 ScopeOfWork) true. Exact route E keeps its existing Timoshenko admission |
| Sparse and dense solver modes | Both, same formation |

## 4. Results, contract and readers: an additive identity

Changing the preview-physics-2 table would move its hash. Every recorded run bound to that hash would then fail compatibility (`core/analysis_runs/compatibility.py` ~845), and the hash is pinned in five places. So a Timoshenko PF result gets its **own semantic contract identity**, and nothing about Euler–Bernoulli results changes.

- **Euler–Bernoulli results** (absent or explicit) keep `semantic_contract_v0_3_preview_physics_2`, profile `product_preview_mechanics_v1`, limitation 0 "Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.", and every byte and hash.
- **A Timoshenko PF result** carries:
  - a new table, `semantic_contract_v0_3_preview_physics_timoshenko_1`. It is preview-physics-2's rows, vocabulary and policies, with limitation 0 replaced by "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_v1) and Poisson's ratio E/(2G) - 1, and realized arcs are Euler-Bernoulli; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness.";
  - a new results profile, `product_preview_mechanics_timoshenko_v1`, whose schema `const` is that text.

  The table is derived mechanically from preview-physics-2 by `semantic_contract_tables.py` (a new `derive` step with `--check`), never hand-copied.
- **Named arc disclosure** (Agent 0's condition: "never silent"). The identity's limitation text states the rule. In addition, a Timoshenko PF result that contains realized arcs carries one nonblocking `Diagnostic` of severity `info` in `MechanicsEnvelope.diagnostics` (`lib.rs` ~1111, ~2436), with code `PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI`.
  - **Emission.** Once per result, model-wide.
  - **References.** The diagnostic's entity references are the realized-arc members.
  - **Message.** It gives the counts: n straight members with shear deformation, k realized arcs without. It names the arc ids in ascending code-point order, up to 32 ids, then "and N more". The whole message is bounded to 1,000 Unicode scalar values, the report renderer's evidence threshold (`report_renderer/src/lib.rs` ~1337). Ids beyond the bound are counted, never dropped silently.
  - A result with no realized arc carries no such diagnostic. The limitation array is not touched, so the readers' fixed-limitation equality holds.
  - **Report.** The report has no analysis-basis section; its eight sections are in `report_generator/src/lib.rs` ~62 and `report_renderer/src/lib.rs` ~36. The disclosure appears in two places:
    - the diagnostic, in the report's diagnostics section, which the desktop already copies (`renderableReportInput.ts` ~165);
    - the Timoshenko identity's limitation 0, among the producer limitations. The desktop copies those today only for exact results (~884); it gains the PF Timoshenko identity, and only that identity, so Euler–Bernoulli report bytes are unchanged.

    **Source binding on PF.** The existing producer-limitation and diagnostic bindings are exact-only:
    - the desktop's `exactReportSections.ts` ~127 and `exactReportLimitations.ts` ~110 (a closed, digest-keyed statement table);
    - `report_package/src/bindings.rs` ~679, which compares report diagnostics with the source only for exact documents;
    - `exact_sections.rs` ~331;
    - native `report_render_exact.rs` ~108, which binds only when `exact_route` is present.

    So the PF Timoshenko identity gets its own projection rather than a widened exact branch. A PF section carries the identity's limitation 0 and the arc diagnostic.
    - The package (`bindings.rs`) compares that section's limitation and the arc diagnostic, text and references, with the delivered result's envelope, for the PF Timoshenko identity only.
    - The native renderer binds the section to the result's source digest the same way it binds exact sections.
    - Euler–Bernoulli PF reports take neither path, so their bytes and checks are unchanged.

    Regressions: removing or altering the limitation, the diagnostic's text or its references in the report, while keeping the result, is refused at packaging and at native render.
  - **Regressions:** a model with realized arcs (the diagnostic, its references and message, the report text); a model without (absence); and a model with more than 32 arcs (the bound and the count).
- **Identity-aware bindings.** Today one identity per route is assumed in several places. Each becomes keyed by the envelope's `semantic_contract_id`, with the route's admitted set {preview-physics-2, preview-physics-timoshenko-1}:
  - `reader_verdict.rs` ~122/~553, which selects a binding by route name;
  - the desktop's `analysisRunCompatibility.ts` ~39 (a closed set of identities);
  - `StressNeutralExportPanel.tsx` ~925 (one table filename per route);
  - `contractBindings.ts` (test expectations);
  - `result_export/src/derivative.rs` ~158 (the allowlist that copies contract evidence);
  - fresh-result admission: the desktop's `knownSemanticLimitations.ts` ~18/~86 and its shared constant `numericalResultQuality.ts` ~18, which feeds the Current result (`resultsSessionState.ts` ~49), and its Rust and Python counterparts, `semantic_contract.rs` ~662 and `core/analysis_runs/compatibility.py` ~521;
  - the Python stress-neutral package: `core/handoff/stress_neutral/package.py` ~99/~103 (supported methods and preview evidence namespaces, used at construction ~434 and transport ~128);
  - report packaging: `report_package/src/document_form.rs` ~19 and the desktop's `reportPackageRequest.ts` ~57.

  **Inventory guard.** These sites were found across three review rounds. So the build adds a test that scans Rust, Python, TypeScript and schema sources for the preview-physics-2 identity string, table filename and profile. It fails on any occurrence that is not on a reviewed list, each entry marked either "admits both identities" or "Euler–Bernoulli only, by design" with its reason. A later site that names one identity cannot be missed silently.
  - The scan covers the identity string, table filename, profile, the table's hash literal and limitation 0's text.
  - The reviewed list also enumerates the symbol-only gates that carry no literal, from Codex round 4's inventory:
    - `knownSemanticLimitations.ts`;
    - `reader_verdict.rs` (~58, ~108, ~122, ~250, ~553) and `semantic_contract.rs` (~513–662);
    - `derivative.rs` ~158;
    - both `compatibility.py`;
    - `package.py`;
    - `document_form.rs` and `reportPackageRequest.ts`;
    - the desktop's record-table and export-table maps;
    - the report bindings above.
  - A label on the list proves nothing by itself. Each "admits both" entry is exercised by a round-trip test below that fails if the gate refuses the Timoshenko identity.

  The producer selects the identity for both successful and blocked PF envelopes (`lib.rs` ~1293, ~3417, ~14601).
- **Readers.**
  - **Model-free** (raw `preview_physics_evidence.rs` ~259 and its Python mirror ~299; the transport validator `retained_precision.rs` ~6657/6689 and its Python mirror ~543): each admits either complete identity, meaning table, profile and limitation array together. A mixed array is refused (`FORMULATION_BASIS`).
  - **Model-bound** (`reader_verdict.rs` ~250, `tests/verifiers/reader_verdict.py` ~313): a crossed pair is refused, i.e. a Timoshenko identity on a model without the selection, or the Euler–Bernoulli identity on a model with it.
  - **Compatibility and AnalysisRun.** `compatibility.py` (both), `analysis_run.v0.3.schema.json` and `stress_neutral_export.v0.3.schema.json` gain the new identity's hash beside the old one. They do not replace it, so existing recorded runs stay compatible.
  - **Desktop.** `contractBindings.ts` computes from the table and gains the new identity.
  - **Retained.** The retained table (`…_preview_physics_retained_1`) is unchanged.
- **Reader limits (stated, not hidden).** The reader binds the declared selection to the identity. It does not re-derive a member's κ_E, A_s or ν, because PF publishes no section evidence. So an implementation error in κ is caught by the references (§5), not by readers. A per-member record (case basis, ν, κ_E, A_s) is left open (§8).
- **Stress, action and reaction rows are unchanged in kind and basis.** Only their values move for Timoshenko documents.

## 5. Verification

- **Byte identity (regression).** Every existing PF fixture, golden and corpus document gives byte-identical results. Every existing semantic-table hash and every recorded payload's bytes are unchanged. The schema files that admit the new identity (results, AnalysisRun, stress-neutral) do change, additively. The checks are `pressure_free_byte_identity`, the corpus replay at main against head, and the full suites with main comparison. A document with `shear_model: euler_bernoulli` gives the same rows as its twin without it.
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
  - the factor-range block, `PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE`;
  - `PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED`;
  - the D1 decline (ordinary route, unchanged result);
  - `MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT`;
  - the objective connector's existing refusal on a Timoshenko document.
- **Readers and round trips.** Rust and Python mirrors: both identities admitted model-free, mixed arrays refused, crossed pairs refused model-bound, and existing recorded runs compatible. A Timoshenko solve also round-trips, with its own identity and table at each step, through:
  - becoming the desktop's Current result;
  - AnalysisRun recording;
  - result export;
  - stress-neutral export (Rust and the Python package's construction and validation);
  - report packaging.

  A blocked Timoshenko envelope carries the new identity. The identity inventory test passes.
- **Independent numerical scrutiny.** A Codex review covers the wiring, the ν and κ basis, the refusals, the readers and the references against their generator.
- **Desktop.** The Model settings section gains "Straight-member shear deformation": Euler–Bernoulli (the default) or Timoshenko (energy-matched annular factor). Choosing either writes the member, as an explicit user set; "Reset to default" clears it back to absence; and a document whose user never chose keeps it absent. A line states that realized arcs stay Euler–Bernoulli and that Poisson's ratio is E/(2G) − 1. On an exact document the control is disabled and points to the pressure panel. There are Vitest tests for the intents, and the native smoke runs a Timoshenko solve, save and reopen.

## 6. D3, sequencing and size

- **Priced surfaces touched:**
  - `core/product_physics/src/lib.rs` (the PF branch in `build_model_for_members`, ν, refusals, warning);
  - `retained_memory.rs` (D1's `ShearModel` fact);
  - FK `core/solver/frame_kernel/src/shear.rs` (`annulus_energy_shear_factor_of_moduli` and the `NonPositiveModulus` refusal; the existing factor and its order are unchanged);
  - `preview_physics.rs` (the identity selection);
  - `model_settings.rs`;
  - `core/reporting/result_export/src/preview_physics_evidence.rs`, `retained_precision.rs` (the transport validator), `reader_verdict.rs`, `derivative.rs` and `semantic_contract.rs` (a new resident parsed table, if it is cached like the others);
  - the results schema and the new table.

  The preview-physics-2 and retained tables are unchanged. Route L's Euler–Bernoulli invocations still load whatever resident tables the readers cache, so a new resident table is a priced static owner even though Timoshenko documents decline D1. The pricing inventory covers the shared readers, resident table growth and the source-bound rules; the footprint is measured, not assumed.
  - **The Timoshenko table's cache** (Codex review of the build). `semantic_contract::preview_physics_timoshenko_contract` parses the table into a `OnceLock` on the first read of a Timoshenko source. From then on it is resident for the process, through every later route-L Euler–Bernoulli invocation, so it is a process-lifetime static owner like the reader statics of DEL-04-08 `retained-pricing-rules.md` §3.6. At #1663's D3 turn:
    - the table (`fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json`) joins `build_identity.rs`'s `REVIEWED_INPUTS`, so its bytes are bound into the retained build identity;
    - its cache joins the static-owner inventory (`STATIC_OWNERS`, the §3.6 owner list) and is priced as resident in every phase;
    - the affected shared-reader work is priced: `current_header`'s and `for_source`'s identity dispatch, `reader_verdict.rs`'s identity-keyed bindings and `bind_preview_shear_model`, and the model-free readers' second identity.

    Until then `REVIEWED_INPUTS`, the registered profile and every price are unchanged: registering them changes the build identity, which is D3's act. The only FK change is the shear module's new function; no straight_pipe source changes.
- **Order.** The unit builds after #1649 (0.8.0) merges. D3 is queued after #1649, W-16b/c and #1654, and is priced only on Agent 0's go. The AC-004 witnesses run if D1's change or the stack moves them.
- **Size.**
  - About 600–900 lines of product, reader, table-derivation and desktop change.
  - The reference generator and its frozen physical and judged sets.
  - One build PR, and a second for partial spans if taken.

## 7. Commitment and design text

- **Commitment text (owner-reserved; exact old → new, Codex-reviewed, then to Agent 0).** DEL-05-03 `ScopeOfWork.md` CLM-008's last sentence:
  - old: "Pressure-free documents keep their single section and Euler–Bernoulli frame."
  - new: "Pressure-free documents keep their single section; their straight members are Euler–Bernoulli unless the document's `model_settings.shear_model` selects `timoshenko_annulus_energy_v1`, the same element and shear-factor definition with Poisson's ratio E/(2G) − 1, and their realized arcs are Euler–Bernoulli."

  DEL-04-01 ScopeOfWork ("per document … (D-6)"), DEL-05-05 ScopeOfWork:51 and DEL-04-08 ScopeOfWork:103 stay true as written. The PRD's FR-SOL-003 is neutral on beam theory.
- **Design and description text, updated in the build PR (#1663):**
  - `section-and-shear-decisions.md` §D-6 (a PF paragraph pointing here);
  - `section-bases-and-timoshenko.md` line 3;
  - DEL-04-07 `deliverable.yaml`'s kernel line;
  - the `docs/COMPLETENESS_MAP.md` FR-SOL-003 row ("PF Euler–Bernoulli; EX adds Timoshenko" → PF opt-in), sent to Agent 0 for W-17 (#1642) to reconcile, not edited here (Agent 0, 2026-10-27);
  - FK `README.md` line 14, which still says the W1 element stays Euler–Bernoulli although O4 forms Timoshenko on route E.

## 8. Open

- A per-member PF evidence record (case basis, ν, κ_E, A_s) that would let readers re-derive the factor.
- Disclosure of a temperature point's authored `poisson_ratio`. The warning compares only the material's own `poisson_ratio`; a point-level value that differs from that point's E/(2G) − 1 is not used and not disclosed.
- **The partial-span slice.** S11-G and K-D5 for Timoshenko partial spans, beside the exact-route partial-span item (one formation check). The solver lane's exact-route build (branch `claude/piping-w16t-timoshenko-partial-wind-20261027`, its design §2) adds two checks this slice can call unchanged once PF has Timoshenko members:
  - **TPS-G**, an S11-G guard. It holds the element's own binary64 L, T, φ_y and φ_z, interprets them exactly, and multiplies the defect by D = (1+φ_y)(1+φ_z), so that it is exactly computable.
    - The correction is ΔV₀ = φ/(1+φ)·(wL/2)·[g(a) − g(b)], with g(ξ) = ξ²(1−ξ)² and pattern [1, L/2, −1, L/2].
    - Each slot is `Formation::Bounded` with a directed post-hoc bound. A range failure makes every slot `CannotBound` and demotes.
  - **TPS-K**, the K-D5 re-formation. `uniform_straight_balls` forms φ from the frame's leaves in `frame_matrix`'s order and adds the correction in ball arithmetic. A ball failure gives `FormationCheckUnavailable` and demotes.

  Neither adds a formation kind, ledger field or closure change. The full span bypasses both, bit for bit, and their interfaces are generic in φ and frame. The slice then removes `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`, with PF references for partial spans.
- A formulation-matched published benchmark for κ_E. This remains open, as for the exact route (§11 there).
- CAEPIPE correlation after the change: re-running the pilot's shear-deformation series is the owner's private comparison (DEL-17-05 §8), not a repository record.
