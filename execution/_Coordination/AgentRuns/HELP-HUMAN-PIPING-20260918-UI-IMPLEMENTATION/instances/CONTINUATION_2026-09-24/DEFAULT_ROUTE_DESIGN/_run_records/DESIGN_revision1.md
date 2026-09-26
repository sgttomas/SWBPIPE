# T0R — the default route must not publish silently wrong results

HELPS_HUMANS design record for ROOT (HELP_HUMAN), who launched this session and selects from it. It is a proposal: nothing here is selected, implemented or accepted until ROOT says so.

- **Basis:** main `eb56e1083`. The worktree is at `13df6cc74`, which adds only records (the T0 return and the T0R row in the work graph).
- **Paths:** `P/` means `projects/chirality-piping/`. `PP` means `P/core/product_physics/src/lib.rs`; line numbers are at `eb56e1083`.
- **What I did:** read the records listed in §14; ran one read-only probe on main (§2.1); wrote the candidate hand-statics references (`references.py`, §9.1).
- **What I did not do:** edit any product source, test, fixture or other record; run any Git write.

## 1. Recommendation in brief

Adopt a combination, **option (d)**, under one new result-semantics identity for fresh solves on the ordinary route. The proposed name is `openpipestress.result_semantics/0.3.0/preview-physics-1`. The rule is simple: publish what can be computed correctly now, and withhold, with an explicit reason, what cannot.

**Repaired on the ordinary route:**

- **Reactions:** six signed support-on-pipe components per support, in both cases and linear combinations, with force and moment magnitudes derived from them.
- **Straight members:** the certified circular-section normal-stress maximum. The exact route already has it; this route reuses that recipe without pressure.
- **Headlines:** stress and displacement headlines that cover every load case and withhold themselves when the domain is incomplete.
- **Combinations:** signed-only combination algebra, using a closed list of kinds that may be combined.

**Withheld, with a stated reason:**

- every SIF-intensified row (the i×k and abs-sum multiplier);
- any maximum on a curved-bend arc, and therefore the stress headline for models that contain one;
- support attribution the solve cannot determine (shared degrees of freedom, and nonlinear/constant-effort cases where the formulation does not fix the device's force);
- combination stress maxima.

**What does not change:**

- **No model change.** The model version, the authoring default, pressure behaviour and the exact route all stay as they are.
- **Old records stay as they were.** Historical `precision-1` records keep their bytes and meaning. They remain readable, but they stop being Current, rule-eligible or report-eligible. They carry a derived notice listing their known limitations.

This closes none of M05, M14, M33 or M08. It removes every silently wrong quantity those groups put on the default route, and it completes the parts §11 lists. The rest stays with T3, T4, T5 and T6.

## 2. What the default route publishes today

Taken from T0 and confirmed here. The T0 probe covers M33, M14, M05 and M08; see `../T0_REASSESSMENT/_run_records/t0_probe_output.log`.

| Defect | Where (PP) | Observed |
|---|---|---|
| M33: headline is first case only | `1497-1507`, `load_case_solves.first()` | 22.77 MPa published; the governing case B reaches 34.16 MPa |
| M14: summary is the abs-sum of components | `open_formula_summary_mpa` `8930`, `straight_summary_extrema` `7570` | 22.77 MPa where the section maximum is 16.10 MPa (√2 high) |
| M05: reaction is a force norm only | `2180-2211`; combination vectors are 3-component (`10277`) | pure 500 N·m torque publishes 0 N and no moment |
| M08: SIF × k, applied to the abs-sum, then combined linearly | `9128-9230`; not excluded at `10584` | cases that sum to zero stress publish 28.28 MPa |

### 2.1 New observations from this design's probe

Source, raw output and toolchain are in `_run_records/design_probe/`, `_run_records/design_probe_output.log` and `_run_records/toolchain.txt`.

1. **A consuming constant-effort support publishes `reaction_resultant = 0 N`, while its own applied-load row reads 375 N.** Model: the invented preview model, with `CE-120` given one translational DOF and its legacy pressure loads removed.
   - Cause: `build_model` drops constant-effort supports from the linear support list (`PP:4526`), so the reaction vector stays zero.
   - This is a second silently wrong M05 quantity on the default route.

2. **A curved-bend arc publishes correct signed resultants, but only if its geometry is tangent-consistent.** Case REF-B1: a 90° arc, R = 0.2 m, flexibility k = 2 and k = 4.
   - With consistent geometry, the tangent-frame station rows match hand statics to about 1e-12. Midspan torsion is 765.685 N·m and out-of-plane bending 848.528 N·m. The endpoint *stress* rows use the tangent frame; the endpoint *force* rows use the chord frame, as documented at `PP:2263`.
   - The resultants do not change with k, as expected for a statically determinate model.
   - The sampled station maximum (11.3899 MPa at quarter_3) exceeds the endpoint value (11.3852 MPa). The station values are therefore not a certified arc maximum.
   - A `y_reference` that is inconsistent with the adjacent pipes builds a kinked arc in another plane, with no diagnostic. That arc gives 382.7 N of axial force under a load normal to the intended bend plane. The route does not validate tangent continuity with the neighbouring pipes. This is M02 and belongs to T4 (§12).

3. **With actions held fixed, the SIF row scales with k.** Case C, a geometry-only marker: k 1.08 → 2.16 moves the row from 14.14 to 28.28 MPa. This is the negative control for "stress never scales with flexibility".

4. **Retained-source publications have the same M14 fault.** When the ordinary attempt is Sensitive and the source gate admits the case (straight members, no components; `source_recovery.rs:419-436`), a fresh ordinary solve publishes `source-blocks-1`. That envelope's `open_formula_stress_summary` and headline are the abs-sum (PP third branch at `2702`). Its reactions and all-case headline are already correct.

## 3. Distinctions this design depends on

- **Physical section quantities versus code-assessment quantities.**
  - Physical: signed section actions (N, Vy, Vz, T, My, Mz) at a declared station and frame; support actions; and elastic stresses of the represented beam model. For a straight circular pipe the latter are N/A, M/Z and the circular maximum |N/A| + hypot(My,Mz)/Z (STRESS_REFERENCE §3).
  - Code assessment: anything that applies a stress intensification factor, a category, an allowable or a combination formula taken from a code. Users supply these through rule packs; agents do not populate them.
  - A product-computed "SIF × something" is a code-assessment quantity, whatever formula it uses.
- **Straight versus curved members.**
  - On a straight member, beam theory's section stress is the physical wall stress of the represented model.
  - On a bend, beam-theory stress of the recovered resultants is only a *nominal* value. The real wall stress includes ovalization and curved-beam effects, which are T4 mechanics.
  - So a bend may truthfully carry signed resultants and nominal station components. It may not carry a "maximum", an intensified stress, or a place in a headline.
- **State versus derived quantity.** Linear combination and subtraction apply to signed state quantities. Magnitudes, maxima and intensified values are recomputed from the combined signed state, or are selected in an envelope. They are never added or subtracted (STRESS_REFERENCE §5).
- **Complete domain.** A headline is a maximum over a declared domain. If any member or case in that domain lacks a qualified value, the headline is withheld, not shrunk. This is the exact route's rule (`maximum_across_cases`, `PP:1665`) and RAW02's rule: "a missing governing calculation must never appear as a qualified maximum".

## 4. Options compared

### 4.1 Summary

| | (a) Repair under a new identity | (b) Contain only | (c) Exact profile as the authoring default | **(d) Repair where correct, withhold the rest (recommended)** |
|---|---|---|---|---|
| **Users and models** | Every existing model still solves. Straight-member stress, reactions and headlines become correct. Bends and SIF need a policy anyway, so (a) done honestly becomes (d) | Every model still solves, but reactions, the stress summary, the headline and SIF are all withheld. No correct replacement exists; the ordinary route becomes nearly useless for support or stress work | New models are refused if they have bends, fittings, nonlinear or constant-effort supports, combinations or equivalent-static cases, or E/G-only materials (the exact profile needs nu). Existing 0.2.0 models keep solving on the defective route | Every model still solves. Straight members, reactions (including moments) and all-case headlines are correct. Bend-arc models lose the stress headline, with its reason shown; SIF rows disappear, with a notice |
| **Identity and history** | New semantic identity; precision-1 becomes history | Can keep precision-1 if withheld rows are simply omitted, but fresh records then differ from historical ones under the same identity and keep the defect in their bytes. Consumer-side containment changes derived standing for historical records too | No identity change; the model version changes for new models | One new identity; precision-1 frozen and historical-only; notices are derived only |
| **Consumers** | Rust, Python, TS readers; schemas; exports; rule standing | Current, rule, report and export gates, plus panels | Desktop authoring default (`projectService.ts:36,466`), backend migration (`model_document_migration.rs:23`), material and pressure-region authoring UI, refusal UX | As (a), plus notices; the legacy report refuses the new identity |
| **Minimum UI** | Reader dispatch; label the new quantities | Withheld states and banners in every panel | Substantial authoring UI (default schema, nu, `pressure_regions: []` per case, component refusal) | Reader dispatch; headline label and withheld reason; historical notice; report unavailability reason |
| **Group completion** | Partial M05, M14, M33, M08 | None ("Containment alone does not close a group") | None on the ordinary route, which still exists for old models | As in §11 |

### 4.2 Option detail

**(a) Repair under a new identity.** Taken literally, as "correct circular summary on straight members, SIF rows kept out of linear algebra", (a) still leaves two things to decide:

- **What the SIF row means.** The row itself is the M08 defect: i×k, applied to an abs-sum. Keeping any product-computed SIF row, even SIF × circular maximum, picks a code formula (it is B31.1-like), which is excluded.
- **What bends publish.** On a bend arc there is no certified circular maximum.

So an honest (a) turns into (d). Tests for (a) are the §9 references, minus the withholding controls.

**(b) Contain.** It withholds the four quantities from Current, rules, reports and exports, with disclosure. It stops the harm, but users get no reactions and no stress headline on any real model, and nothing is repaired. Implemented in consumers only, it gives historical and fresh precision-1 records the same derived standing, which is consistent. But fresh records keep the defective bytes: a record exported under a precision-1 profile still contains 28.28 MPa SIF rows. Tests would be negative controls only; they prove absence, not correctness. The variant T0 suggested, "retire the ordinary route for fresh Current solves", is (b) at its extreme.

**(c) Exact profile by default.** It fixes new straight-pipe-only models. It refuses the models a practitioner builds most: those with bends, tees, rests or gaps, hangers and load combinations. It also requires nu for every material the user supplies. The exact profile refuses combinations, so every sustained-plus-thermal study is refused. Existing 0.2.0 documents keep producing the defects, so (c) never removes them. It is a large UI change against the owner's route direction. It becomes the right default only once T4, T5 and T6 extend the exact profile. I recommend recording it as the long-term convergence path (§13), not as T0R.

**(d) Recommended.** Detailed in §5 to §10.

## 5. Recommended design: `preview-physics-1`

### 5.1 Identity and versions (the decision ROOT needs to make)

| Family | Decision |
|---|---|
| Semantic identity for fresh ordinary-route solves | New: `openpipestress.result_semantics/0.3.0/preview-physics-1` (proposed name). Table `P/fixtures/results/semantic_contract_v0_3_preview_physics_1.json`, pinned by sha256 in every consumer. Table header: `inherited_semantic_contract_sha256` = precision-1's `d75aacee…`; explicit `retired_source_kinds`; `combination_policy`; `contract_evidence_policy`; and `supported_profile_limitations` |
| Which models it applies to | Any model where `pressure_runtime::is_exact` is false: 0.1.0, 0.2.0, and 0.3.0 with `legacy_pressure_v1` and zero pressure. Blocked envelopes on this route use it too (`mechanics_producer_for_model`) |
| Raw mechanics schema | Unchanged, `0.2.0` |
| Product producer | Unchanged, `open_pipe_stress_product_physics` `0.2.0`. RAW02 and VERSION_RESERVATION_V2 treat the semantic identity as the discriminator; physics-1 and source-blocks-1 followed the same precedent |
| Canonical derivative, analysis record, stress-neutral export | Unchanged at `0.3.0`, `strict_analysis_run_v0_3`, `ops.stress_neutral.v3`. Each carrier's typed registry gains one exact id/hash pair, as for physics-1 |
| Formulation profile | Keep `product_preview_mechanics_v1`, because the mechanics formulation does not change. Replace the limitations text with the list in §5.8 |
| Model document | Unchanged. No migration, no authoring default change, no pressure change |
| `precision-1` | Frozen. Table, bytes, fixtures and hash verification unchanged. The fresh producer never emits it again. Readers verify and display it as historical. It is not admitted as Current, rule-eligible, report-eligible or a Current stress-neutral export (§5.7) |
| `source-blocks-1` (ordinary retained-source) | Unchanged in T0R. T1 is editing the receipt code, and the default route reaches it only for Sensitive straight-only cases. Its M14 abs-sum gets the derived notice. Re-homing it (for example as a preview-physics source composite, or through physics-source-1) is recorded as a T3 item |
| Reserved `reactions-1` / `stress-1` (precision-1 table) | Stay inactive. One envelope carries one identity, and T0R changes reactions and stress together. The new table records that it supersedes their use on the ordinary route |
| physics-1, physics-source-1, T1's load-reference-1 | Untouched. T1's 0.4.0 is an exact-route model (`is_exact` extended on its branch), so it is outside this route |

The name is a proposal. ROOT may rename it before S1 freezes the table bytes. Avoid `ordinary-physics-1`: `P/validation/qualification/fixtures/ordinary_physics_1_semantics.json` already names a copy of physics-1.

### 5.2 Reactions (M05)

Each support device publishes its support-on-pipe action with the existing kinds and metadata from `append_signed_support_results` (`PP:8712`):

- six `support_reaction_component_v2` rows (global frame, at the attachment node, Fx/Fy/Fz in N and Mx/My/Mz in N·m);
- `support_reaction_force_magnitude_v2` and `support_reaction_moment_magnitude_v2`, computed from those components.

`reaction_resultant` is retired for this identity. The device laws are those of RESULTS_AND_COMBINATIONS §"Source quantities":

- **Rigid linear restraint:** the nodal residual `K u − f` on its restrained DOFs, and physical zero on the DOFs it does not restrain. The force vector already includes consuming constant-effort loads (`PP:9764`), so the residual already excludes them.
- **Linear spring:** `−k·u` on its DOF (existing law, `PP:2138`).
- **Consuming constant-effort support:** its applied constant force on its declared DOF (DEC-049 ideal element: zero stiffness, force known exactly). This repairs the 0 N row in §2.1. A non-consuming constant-effort support is not part of the solved mechanics: its rows are withheld with reason `CONSTANT_EFFORT_NOT_CONSUMED`, never published as zero.
- **Nonlinear support:** the nodal reaction at its DOF, but only when attribution is unique (next item).
- **Attribution uniqueness:** a DOF at a node can be acted on by two or more devices across different support records. Duplicate rigid restraints, and spring-plus-rigid on one axis, already block (`PP:1356`; `validation.rs:70`; M09 closed). If one of the devices is residual-determined (rigid or nonlinear), that device's rows are withheld with `SUPPORT_ACTION_ATTRIBUTION_WITHHELD`. For a nonlinear device, its inherited `nonlinear_support_final_reaction` row for that DOF is withheld too. Law-determined devices (springs, constant-effort) stay attributed. This contains the inactive-contact shared-DOF read without inventing a split. T5 supplies real attribution.
- **Combinations** (`mechanics`, `result_state_subtraction`):
  - The six signed components combine linearly. Magnitudes are recomputed from the combined vector.
  - Combination rows group by (kind, support, component), not by the case-bearing row id. v2 ids embed the case id (`PP:1535`), so the current `rows_by_base_id` join would never match them.
  - `support_force_vectors` becomes six-component.
  - A `range_envelope` selects each component and each magnitude separately, and is labelled as not being a simultaneous state (existing semantics).
- **Frame:** global only. Skewed and support-local frames are T6/T7.

### 5.3 Straight-member stress (M14)

- **What replaces the summary row.** For every straight member and case, `open_formula_stress_summary` is retired and replaced by `pipe_elastic_normal_stress_maximum_v2`: unit Pa, basis `recovered_from_open_mechanics_stress_components`, location `governing_station`.
- **How it is computed.** It reuses `exact_straight_summary_extrema` (`PP:7487`) with `pressure_state = None`, which gives a certified enclosure over all statics intervals. The published value is the enclosure midpoint and the evidence carries the bounds, as on the exact route.
- **Scope of the formula.** On this route nonzero pressure is refused, so N is the wall force and no pressure-longitudinal term is added. Thermal axial force enters through the existing corrected end actions.
- **Torsion** stays in its signed `element_local_torsional_shear_stress` rows. No equivalent or code stress is formed.
- **Unavailable maxima.** A member with `STRESS_RECOVERY_LIMITED` findings, or an enclosure failure, has no maximum row. It is listed as unavailable in the coverage evidence (§5.6).

### 5.4 Bends: what the ordinary route may truthfully publish until T4

| Bend representation | Published | Withheld |
|---|---|---|
| `curved_bend_macro_element` arc span | Signed section resultants at the ends and three stations, in the arc tangent frame (x tangent, z bend-plane normal, y toward the centre), exactly as now: in-plane moment is `Mz`, out-of-plane `My`, torsion `Mx`. The endpoint force rows stay in the chord frame, as documented. Nominal station stress components stay, with their existing kinds and signs; the limitations text (§5.8) declares them nominal straight-beam values of those resultants, which exclude ovalization, curved-beam and intensification effects. The user's k enters stiffness only, as DEC-070 already does | Any `pipe_elastic_normal_stress_maximum_v2` for the arc. The straight-interval enclosure does not apply to an arc, and the sampled stations provably miss the arc maximum (§2.1). Any intensified row. The span is listed in `outside_domain_pipe_ids`, so every case's stress coverage is incomplete and the stress headline is withheld with `PREVIEW_STRESS_HEADLINE_WITHHELD` naming the spans |
| `mechanics_geometry_only` bend marker | Nothing bend-specific. The adjacent straight members publish their correct straight maxima. The formulation limitation states that the marker changes neither stiffness nor stress | The SIF/k row. The member domain is complete (all straight), so the headline stays, labelled as nominal with no component intensification. One info diagnostic per component, `COMPONENT_STRESS_INTENSIFICATION_NOT_APPLIED`, echoes the user's SIF, k and source reference as inputs, with no stress value |
| Branch/tee SIFs | Same as the marker | Same as the marker |

- **User rule packs.** A pack can still compute a code stress from the physical quantities: signed arc-station moments in a declared frame, Z from the section, and the user's own SIF values. For geometry-only markers and branches, the adjacent members' end moments are in their own element frames. Resolving them into a component frame is M08 directional recovery (T4).
- **No default intensified measure.** T0R publishes no "user-intensified bending measure". The earlier design proposed `hypot(ii·Mi, io·Mo)/Z` (STRESS_REFERENCE §4, RESULTS_AND_COMBINATIONS). Whether the product should ever publish such a measure, as opposed to leaving intensification entirely to user rules, is the owner question in §12.

### 5.5 Headlines (M33)

- **Stress headline:** `summary.max_open_formula_stress`. The field name is carried over, as on the exact route. It is computed as `maximum_across_cases(load_case_solves, true)` for every model on this route, replacing the `first()` branch at `PP:1500-1506`.
  - Unit: Pa. It points at the governing `pipe_elastic_normal_stress_maximum_v2` row. Ties break deterministically by identity, not input order (existing).
  - Withheld (`None`) when any case lacks a complete member domain.
- **Displacement headline:** `maximum_across_cases(…, false)`. `HIGH_DISPLACEMENT_REVIEW` uses the same domain.
- **Domain:** all requested load cases. Combinations are not included in either headline in T0R.
  - When the model has combinations, an info diagnostic `PREVIEW_HEADLINE_SCOPE_LOAD_CASES` states this.
  - Combination-scoped maxima are T6, which may later fold combination states into a declared headline domain.
- **Mixed modulus bases:** stays T6. Across modulus bases, normal stresses are still comparable physical quantities (STRESS_REFERENCE §7).

### 5.6 Combinations and evidence

- **Combination policy.** Only kinds on an explicit list enter `mechanics` or `result_state_subtraction` algebra. The list replaces the exclusion list at `PP:10584` for this identity:
  - global nodal displacement and rotation components;
  - element-local end and station forces and moments;
  - signed element-local stress components, and the zero pressure rows;
  - `support_reaction_component_v2`;
  - attributable `nonlinear_support_final_{reaction,displacement}`;
  - `constant_effort_support_applied_load`.
  - Derived kinds are recomputed from combined signed rows: `displacement_magnitude`, and the support force and moment magnitudes.
  - `pipe_elastic_normal_stress_maximum_v2` is not combined. `COMBINATION_STRESS_MAXIMUM_UNAVAILABLE` replaces `COMBINATION_STRESS_SUMMARY_SKIPPED`.
  - Every other kind (reviews, records, counts, codes, residuals, the retired kinds) is never combined.
- **`contract_evidence`**, a closed finite namespace; S1 freezes the field names: `{"preview_cases": [{load_case_id, pipe_stress_extrema: [...], stress_maximum_coverage: {complete, unavailable_pipe_ids, outside_domain_pipe_ids}, support_attribution: {attributed_support_ids, withheld: [{support_id, reason}]}}]}`.
  - `pipe_stress_extrema` entries use the exact route's 13-field shape (`physicsResultEvidence.ts:195`), so the enclosure checks can be shared.
  - The precision-1 route rejects `contract_evidence` (`semantic_contract.rs:269-276`). The new identity requires it.
- **Reader checks** (Rust, Python and TS all apply them):
  - Rows appear only with kinds from the table; any retired kind is rejected.
  - Each maximum row binds to exactly one enclosure, and the value lies inside its certified bounds.
  - Coverage equals the members minus the unavailable and outside-domain sets.
  - The headline is present exactly when every case is complete, and it equals the maximum across cases with deterministic ties.
  - Each support has either six components plus two consistent magnitudes, or one withheld record.
  - Combination rows resolve their `source_result_refs` to the matching case rows, and derived magnitudes match their combined components.
  - Direct rows carry no derivation.

### 5.7 Historical records and standing

- **Historical records** are precision-1, and the source-blocks-1 abs-sum.
  - Bytes, hashes and verification are unchanged. They are readable and inspectable.
  - A derived notice is attached on display and never written into the record. It lists M05 (norm-only reactions, 0 N constant-effort), M14 (abs-sum summary), M33 (first-case headline) and M08 (i×k SIF rows combined linearly). The equivalent for source-blocks-1 notes only its M14 summary.
- **Current, rule checks, reports and Current exports** admit only the identity the running producer emits for the model's route:
  - ordinary → preview-physics-1 or source-blocks-1;
  - exact → physics-1 or physics-source-1;
  - T1's 0.4.0 → its own identities.
  - Today `currentSolvedResult` (`resultsSessionState.ts:56-74`) and the backend rule gate (`src-tauri/src/lib.rs:2794-2824`) have no route→identity restriction. Add one helper per language: `result_export::semantic_contract::fresh_route_identities(model)` plus its Python and TS mirrors. Stored precision-1 results then read as `needs_recompute`, consistent with VERSION_RESERVATION_V2.
- **Legacy report package:** unavailable for preview-physics-1 through the existing `reportPackageUnavailableReason` pattern, as for physics-1. The report is the surface that today prints the four defects, and carrying signed support rows needs its M26 successor (T6). Canonical result export and stress-neutral v3 carry the new identity. They already handle the physics-1 v2 kinds, including stress-neutral CSV for the support components (`StressNeutralExportPanel.tsx:825`).
- **Browser preview fixtures** stay the historical precision-1 bundles, which are not Current, and show the notice. T6 still owns the precision-1 fixture pair captured before the legacy-pressure refusal. T0R does not regenerate it.

### 5.8 Formulation limitations for the new identity

1. Small-displacement, linear-elastic Euler–Bernoulli frame preview; numerical integrity does not establish physical correctness.
2. Nonzero pressure is refused on this route; the exact pressure profile carries pressure.
3. On straight members, the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. There is no transverse shear, equivalent stress or code stress.
4. On curved-bend spans: signed resultants in the arc tangent frame. Stress components are nominal straight-beam values of those resultants. No maximum is published; a model with such spans has no stress headline.
5. User SIF and flexibility inputs never multiply stress. Flexibility enters stiffness only for `curved_bend_macro_element`. Geometry-only markers change neither stiffness nor stress.
6. Signed six-component support-on-pipe actions for linear restraints, springs and consuming constant-effort supports. Undetermined attribution is withheld.
7. Headlines cover all load cases, not combinations. Combinations use signed algebra; derived magnitudes are recomputed or envelope-selected. There are no combination stress maxima and no code compliance.

## 6. Consumers that must change

| Language | File | Change |
|---|---|---|
| Rust producer | `P/core/product_physics/src/lib.rs` | Call sites only (§10.2); logic in the new `src/preview_physics.rs` |
| Rust readers | `P/core/reporting/result_export/src/semantic_contract.rs` | Id constant, table, `for_source_metadata` / `for_source` dispatch, profile mapping, standing, `fresh_route_identities` |
| | `.../derivative.rs` | Carry `contract_evidence` for the new id, as at `98` |
| | new `.../preview_physics_evidence.rs` | Reader checks of §5.6 |
| | `.../lib.rs` | Module line |
| | `.../tests/` | New contract tests |
| Rust runner | `P/core/runner/headless/src/{lib.rs,result_envelope_binding.rs}` | Admission; the `1475` ordinary→PRECISION_ID expectation becomes the new id |
| Rust app | `P/apps/desktop/src-tauri/src/lib.rs` | `qualify_rule_mechanics_with_context` only: route→identity supersession |
| Python | `P/core/analysis_runs/compatibility.py` | Id sets at `94`, `216`, `290`, `404`, `414`; table path and hash |
| | new `.../preview_physics_evidence.py` | Reader checks |
| | `P/core/handoff/stress_neutral/package_v0_3.py` | Method CSV contract (`87-92`), `semantic_contract_ref` |
| Schemas | `P/schemas/results.v0.3.schema.yaml`, `analysis_run.v0.3.schema.json`, `stress_neutral_export.v0.3.schema.json` | Enumerations and `const` branches for the new id; evidence shape in results |
| Tools | `P/tools/serialization/generate_product_preview_mechanics.mjs` | An added mode for the new fixtures; the precision-1 mode is kept |
| | `P/tools/validation/qualification_*` | Checked and unaffected: the first-static comparisons run the exact route |
| TS | `features/results/numericalResultQuality.ts` | New `SourceContract` route, id/sha, standing; the profile check at `~64` |
| | new `features/results/previewPhysicsEvidence.ts` | Reader checks |
| | new `features/results/knownSemanticLimitations.ts` | Historical notice |
| | `features/results/resultSemantics.ts` | Table pin |
| | `features/workspace/resultsSessionState.ts` | Route→identity condition in `currentSolvedResult` |
| | `features/report/reportPackageRequest.ts` | Unavailable reason |
| | `features/report/ReportPanel.tsx` | Empty state for the SIF review section; headline label |
| | `services/previewService.ts` | Dimension mapping of the kinds (`319`, `356`, `392`, `745`) |
| | `services/ruleCheckService.ts` | Pre-check mirrors the backend |
| | `features/stress-neutral/StressNeutralExportPanel.tsx`, `features/result-export/*` | Route dispatch |
| | `features/handoff/HandoffPanel.tsx`, `features/local-fea-handoff/LocalFeaHandoffPanel.tsx`, `features/native-package/NativePackagePanel.tsx` | Headline label and withheld reason (these already fall back when `max_open_formula_stress` is null) |
| Fixtures | New `P/fixtures/results/semantic_contract_v0_3_preview_physics_1.json` | The table |
| | New `P/fixtures/results/preview_physics_connected_{sparse,dense}.json` and request | Generated from the actual producer |

All precision-1 fixtures stay as they are.

## 7. The minimum UI change the fix needs

No authoring or model UI changes, and there is no layout work. The display changes needed for honest standing, all small and text-only:

1. **Reader dispatch**, not visible: the new route, table pin and evidence validation, so fresh results can be Current.
2. **Headline label:** "Maximum elastic normal stress (nominal; no component intensification; not a code stress)". When withheld, show "Withheld: model has curved-bend spans without a qualified maximum" or the listed reason, taken from the withholding diagnostic. Apply it in the result summary and in the Report, Handoff, LocalFeaHandoff and NativePackage references.
3. **Historical notice:** one line on precision-1 and source-blocks-1 results, listing the known limitations and "re-solve to obtain corrected semantics".
4. **Report panel:** the explicit unavailable reason for preview-physics-1, and an empty state for the component SIF section: "user intensification is not applied by the solver; see rule packs".
5. **Rule-check:** the supersession message when a stored precision-1 envelope is offered.

A support table showing six signed components is not required. The existing generic result rows and exports already present `support_reaction_component_v2` for physics-1. Richer tables belong to T6 or UI-SUCCESSOR.

## 8. Test and verification plan per option

- (a) is the §9 plan without the withholding controls.
- (b) needs only negative controls: every withheld quantity is absent from Current, rule, report and export. They prove containment, not correctness.
- (c) needs authoring-default native witnesses and refusal coverage for every refused family, plus the existing exact-route evidence. It leaves the ordinary-route defects untested and unrepaired.
- The rest of this section is (d).

## 9. Verification plan for (d)

### 9.1 Frozen references (candidate)

`references.py` derives these values; they are printed in `_run_records/references.stdout.txt`. Derivation rests only on rigid-body statics, Euler–Bernoulli cantilever compliance, spring compatibility and the elastic annulus relations of STRESS_REFERENCE §3. The script imports no product code. It reproduces the T0 probe's hand values and STRESS_REFERENCE's S1, X1, R1, R2 and I1-adjacent numbers. The inputs are invented test values, not library data or code rules.

**Before anyone implements:** ROOT should obtain an independent refutation of this reference set, then freeze it (§10, S0). I wrote it as the designer; I am independent of the implementation but not of this design.

| Ref | Case | Frozen expectation | Negative control (must fail on main, or on the mutation) | What it proves |
|---|---|---|---|---|
| REF-M14-A (probe A) | 1 m cantilever, OD 0.12 / wall 0.01, tip Fy = Fz = 1000 N | Root maximum 16.101036950648692 MPa | Abs-sum 22.770304823877722 MPa | Circular resultant formula, not the component sum |
| REF-M14-S1 | OD 0.10 / wall 0.01; tip Fx = 1800π, Fy = 73.8π, Fz = 55.35π | 7 MPa; rotated variant (Fz = 92.25π only) also 7 MPa; add Mx = 221.4π → torsional shear 6 MPa, normal unchanged; pure torque → 0 normal | Abs-sum 9 MPa, and not rotation-invariant | Axis-rotation invariance; torsion kept separate |
| REF-M14-X1 | S1 section, pin–roller, w_z = 8e6·Z N/m, end couple 1e6·Z N·m about z at j | 1.1340862821217075 MPa at t* = 0.57322; the certified bounds must contain it | Eight-sign candidates 1.12673 MPa; abs-sum objective 1.5625 MPa | Interior extremum found on the ordinary route with the new objective |
| REF-M33-G | Case A (probe A), then case B (tip Fy = 3000); both orders | Headline 34.15545723581658 MPa at case B root; displacement headline 0.9487627009949051 mm at case B tip | First-case headline 16.10 MPa | All-case governing maximum; order independence; displacement scope |
| REF-M05-T | Tip torque +500 N·m | Anchor (0,0,0; −500,0,0); force magnitude 0; moment magnitude 500 | Norm-only 0 N and no moment | Signed moments published |
| REF-M05-R1 | 2 m cantilever, tip (10,−20,30) N and couple (4,5,−6) N·m; plus (0,−3,0) N/m | (−10,20,−30; −4,55,46), then (−10,26,−30; −4,55,52); rotated by an exact axis permutation and by Rz(30°): equals Q·R | Any sign or origin error | All six components, sign convention, distributed-load moment, frame covariance |
| REF-M05-SPRING | 1 m cantilever, tip spring k = 1e6 N/m in y, tip Fy = 1000 | Spring on pipe −240.2683505787423 N; anchor Fy = Mz = −759.7316494212578 | Copying the nodal total to both devices | Device-law attribution; the balance closes |
| REF-M05-COMB | A1 (tip Fy = 1000), T (tip Mx = 500) | 2·A1 − T gives (0,−2000,0; 500,0,−2000), \|M\| = 2061.5528128088304; A1 − T gives \|M\| = 1118.033988749895; envelope of the force magnitude = 1000 | Magnitude algebra gives 1500 N·m | Combination of signed support components; magnitudes recomputed |
| REF-CE (from the §2.1 probe) | Consuming constant-effort support, 375 N on UY | Six components with Fy = +375 N on the pipe, others zero; the global balance counts it once | Main publishes 0 N | Constant-effort action published once, not zero |
| REF-M08-L (probe L) | L model, geometry-only marker with SIF 1.15, k 1.08; cases ±1000 N in z; combination up+down | No multiplier row in any case or combination. Combined signed rows and supports are all zero. Headline 11.385152411938861 MPa, tied between a-b at a and b-c at b, resolved deterministically. Anchor (up case) (0,0,−1000; −1000,1000,0). Diagnostic echoes SIF and k | 28.28 MPa row; a row that scales with k (14.14 → 28.28 when k doubles) | M08 rows gone; no linear combination of derived values; stress independent of k |
| REF-B1 | Tangent-consistent 90° arc R = 0.2, straight legs, tip Fz = 1000; k = 2 and 4 | Tangent-frame magnitudes: at b, T = 1200 and Mo = 200; midspan T = 765.6854249492379 and Mo = 848.5281374238571; at c, T = 0 and Mo = 1000; in-plane quantities 0. Identical for both k. No arc maximum row; headline withheld with its reason | A published arc maximum or headline; any in-plane action | What bends may publish: truthful resultants, no false maximum |

**How to compare.** Use the existing independent-test criterion, `1e-9·max(|expected|, stated scale)`, as in `tests/support_reactions_runtime.rs`, and check the X1 enclosure containment. No new tolerance is introduced and no protected criterion changes.

**Coverage.** Run every reference in both solver modes. Include one mm/kN-unit variant of REF-M05-R1, and one of REF-M33-G with cases on different modulus bases (the headline still compares normal stresses). Every reference also checks global force and moment balance of the published support rows against the applied loads, about the origin. That check is computed in the test from published rows only.

**Where the tests live.** They are written as `P/core/product_physics/tests/preview_physics_runtime.rs` against the public captured entry point (`run_linear_static_preview_value_with_mode`), with expectations copied from the frozen output and no production helper imported.

### 9.2 Identity, consumer and negative controls

- **Pre-implementation detection run.** The frozen references run against main must fail exactly on the defect cases (M14-A, S1 sum, M33-G, M05-T, M05-COMB, CE, M08-L). This proves that the checks detect the defects. Record the run.
- **Mutation controls on the candidate.** Each must fail its test:
  - swap in the abs-sum objective;
  - restore the `first()` headline;
  - drop the moment rows;
  - combine magnitudes linearly;
  - put the SIF row back;
  - publish an arc maximum;
  - zero-fill a withheld support.
- **Reader tamper tests, in all three languages:**
  - an injected retired kind is rejected;
  - a headline that does not govern, or appears while coverage is incomplete, is rejected;
  - a maximum outside its bounds, a combination magnitude inconsistent with its components, a missing withheld record, or unknown or mismatched id and hash each lead to `unsupported` or `needs_recompute`.
- **History preserved.**
  - Every precision-1 and source-blocks-1 fixture, table and hash-verification test passes byte-unchanged.
  - A stored precision-1 envelope is inspectable with the notice, and is not Current, rule-eligible or report-eligible.
  - The headless runner and the Python analysis-run builder reject precision-1 as a fresh publication.
- **Superseded unit tests.** Tests asserting the retired fresh behaviour include `bend_component_user_multipliers_emit_stress_review_rows` (PP ~`15210`) and any test expecting `reaction_resultant` or `open_formula_stress_summary` from a fresh ordinary solve. They are replaced by the successor tests above, with a disposition table in the PR. They are regression assertions of defective behaviour, not protected criteria, so replacing them weakens nothing. Their history stays in Git and in the historical fixtures.
- **Native witness** on the real desktop path, with the candidate, model hash and case ids recorded:
  - author a fresh 0.2.0 model (straight members plus one geometry-only marker and one combination), solve, and check Current standing;
  - run a rule check with an invented, user-authored rule pack bound to the new maximum row id;
  - check that a pack bound to a retired id reports `RULE_INPUTS_INCOMPLETE`;
  - run canonical and stress-neutral export; confirm the report is unavailable with its reason; save and reopen;
  - reopen a stored precision-1 result and confirm the notice and that it is not Current;
  - solve an arc model and confirm the headline is withheld, with its reason shown.
- **Gates:** hosted CI, including the dual-viewport dispatch, and a clean DEC-025 sweep on the merge candidate.

## 10. Slices, order and integration

### 10.1 Slices

Write sets are pairwise disjoint.

| Slice | Owner | Write set | Needs |
|---|---|---|---|
| **S0** Reference freeze | Independent TASK, arranged by ROOT; not the implementer | `DEFAULT_ROUTE_DESIGN/REFERENCE_CHECK/` (records only) | This design. It refutes `references.py`, then freezes it with SHA256SUMS |
| **S1** Table and interface freeze | T0R WORKING_ITEMS manager | `P/fixtures/results/semantic_contract_v0_3_preview_physics_1.json`; an interface note under its run folder (field names for §5.6, diagnostic codes, row id formats) | ROOT's §5.1 decision. Independent review of the table bytes before any consumer pins the hash |
| **S2** Producer | The single `core/product_physics` writer for T0R (the T0R manager, or one TASK under it) | New `P/core/product_physics/src/preview_physics.rs`; the `lib.rs` call sites of §10.2; new `P/core/product_physics/tests/preview_physics_runtime.rs`; dispositions of superseded `lib.rs` unit tests; new `P/fixtures/results/preview_physics_connected_*` generated by the actual producer | S0, S1 |
| **S3** Rust consumers | One TASK | `P/core/reporting/result_export/**`; `P/core/runner/headless/**`; `P/apps/desktop/src-tauri/src/lib.rs`, only in `qualify_rule_mechanics_with_context` and its tests | S1 table; S2's first actual raw output |
| **S4** Python, schemas, tools | One TASK | `P/core/analysis_runs/**`, `P/core/handoff/stress_neutral/package_v0_3.py`, the three `P/schemas/*v0.3*` files, `P/tools/serialization/generate_product_preview_mechanics.mjs`, `P/tests/test_preview_physics_*.py` and the id enumerations in existing `P/tests/test_*` | S1 table |
| **S5** TS readers and minimum UI | One TASK, under the AUTHORING_MANAGER convention for desktop files | The TS files of §6; `apps/desktop/src/types.ts` only if a type is missing (coordinate: T1 plans type work there) | S1 table; S2 raw output |
| **S6** Join and qualification | ROOT with the T0R manager | Run records, work graph and MEMORY rows; the PR | S2–S5 joined; §9.2 checks; complete-diff independent review; native witness; DEC-025; CI |

**Order:** S0 → S1 → S2, with S3, S4 and S5 in parallel once the table and the first raw output exist → S6. One PR, or two if ROOT prefers: producer plus Rust first, then Python, TS and UI. In that case the producer must not merge before its consumers admit the new identity; otherwise fresh solves stop being Current in between.

### 10.2 The shared `core/product_physics` facade and T1

**Integrator.** The T0R manager is the only writer of `core/product_physics` for T0R, on a branch cut from main. ROOT owns fan-in to main.

**Serialization.** Merge T0R to main first. T0R is small and urgent (two of its defects are unconservative), while T1 still has connected types, persistence, authoring inputs, adapters and its PR ahead. The T1 manager then merges main into `codex/piping-load-states-20260925` and resolves the list below. If T1 reaches main first, the T0R manager rebases instead. In either case, one owner resolves each shared file.

**How T0R keeps the facade diff small:**

- all new logic goes in `preview_physics.rs`;
- `lib.rs` changes are limited to the call sites below;
- the `solve_load_case` signature is not changed, since T1 already changes it.

**Conflict surface,** comparing the T1 branch head `c0ef4a8e0` with main:

| File | T0R touch | T1 touch | Kind |
|---|---|---|---|
| `lib.rs` `mechanics_producer_for_model` / `formulation_basis_for_model` (`817-842`) | Ordinary branch → new id and limitations | Adds a load-state branch | Adjacent lines; merge by hand |
| `lib.rs` headline selection (`1497-1507`) | Replace `first()` | Hunks at `1483` and `1516` | Adjacent |
| `lib.rs` result loop and id qualification (`1520-1560`) | v2 id rule for the ordinary route; six-component vectors | Adds `load_reference_states` (`1516`) | Adjacent |
| `lib.rs` envelope `contract_evidence` and producer override (`1612-1650`) | Add `preview_cases` for the ordinary route | Restructures the same expression | **Real overlap**; combine both branches |
| `lib.rs` `LoadCaseSolve` (`2870-2886`) | Six-component vectors; preview evidence | Adds `load_state_evidence` | Adjacent |
| `lib.rs` reactions (`2107-2211`), stress summary (`2640-2720`), SIF functions (`8956-9230`), combinations (`10277-10600`) | Call into `preview_physics` | None | No conflict |
| `pressure_runtime.rs`, `source_receipt*`, `source_recovery.rs` | None | Heavy | No conflict |
| `result_export/src/semantic_contract.rs`, `derivative.rs`, `lib.rs` | Id registry | Id registry (+75/+4/+1) | Mechanical enumeration merges |
| `analysis_runs/compatibility.py`, `stress_neutral/package_v0_3.py` | Id sets | Id sets | Mechanical |
| The three `schemas/*v0.3*` files | Enumerations | Enumerations and new branches | Mechanical |
| Desktop TS | S5 | Planned (types, persistence, readers) | Future; T0R first avoids it |

## 11. What T0R completes and what remains

Nothing closes. The closure rule requires fixed, verified and merged as a whole.

| Group | T0R completes (fresh ordinary route) | Remains |
|---|---|---|
| **M05** | Signed six-component support-on-pipe actions for linear restraints, springs and consuming constant-effort supports, in cases and in `mechanics`/subtraction combinations. Magnitudes derived from the components. Undetermined attribution withheld, never zero or duplicated. Global-balance verification (R1, R2, R3, rotation, combination) | **T5:** nonlinear, contact, friction and hanger attribution, including a labelled aggregate for shared DOFs. **T6:** support-local and nozzle frames; vector envelopes; report and structured-export carriage (M26/M36) |
| **M14** | Retires the abs-sum. Certified circular maximum on every straight member of the ordinary route, torsion separate; verified by S1 rotation invariance and the X1 interior peak | **T4:** curved-bend maximum and wall stress (ovalization, curved-beam, M37 arc extrema); direct transverse-shear and signed circumferential fibre outputs. **T3:** the source-blocks-1 abs-sum, and loaded-span retained-source maxima |
| **M33** | All-case stress and displacement headlines with deterministic ties and complete-domain withholding; `HIGH_DISPLACEMENT_REVIEW` on the same domain; scope disclosed when combinations exist | **T6:** combination-scoped maxima and their headline domain; mixed-modulus diagnosis. **T4:** bend and pressure maxima |
| **M08** | Removes i×k and every product-computed intensified row from fresh publication and from combinations; closed allowlist for combinable kinds; stress independent of k (REF-B1, REF-M08-L) | **T4:** directional signed-moment recovery in component frames for markers and branches; flexibility realisation for geometry-only bends (M02); any intensified measure the owner permits; the arc tangent-continuity check. **T6:** directional stresses and ranges after combination (M15) |

## 12. Decisions

### For ROOT, under the delegated correctness authority

1. Select (d) and the §5.1 identity. Confirm or rename `preview-physics-1` before S1.
2. The headline domain rule for bend models. Recommended: withhold it when an arc span exists, and keep it, labelled nominal, for geometry-only markers and branches. The stricter alternative is to withhold it whenever any SIF-bearing component exists. That is more conservative and makes the headline unavailable for most real models.
3. Publish consuming constant-effort actions (recommended), or withhold them for T5.
4. Leave source-blocks-1 unchanged in T0R, with its notice, and give its re-homing to T3 (recommended).
5. The report package is unavailable for fresh ordinary-route results until T6. This is a user-visible regression of a surface that currently prints the defects. I recommend it and suggest ROOT tell the owner.
6. User rule packs bound to retired result ids (`result:stress:…`, `result:reaction:…`, the SIF rows) stop resolving and report `RULE_INPUTS_INCOMPLETE`. This is explicit and never silent; notify the owner.
7. An adjacent finding for T4: the route builds a kinked arc without a diagnostic when `y_reference` disagrees with the neighbouring pipes (§2.1). An optional containment is a warning in T0R that uses `CurvedBendMacroElement::end_tangents()` against the adjacent pipe directions. It fits S2's write set, but it is M02 scope; the choice is ROOT's.

### Genuinely for the owner

- **Whether the product itself should ever compute an intensified stress from user SIF inputs.** One example is the named `hypot(ii·Mi, io·Mo)/Z` measure from the earlier design. The alternative is to publish only physical section quantities and leave all intensification to user rule packs.
  - This sits on the boundary the owner drew ("code rules are user-supplied"). Any such measure mirrors a code formula.
  - T0R is compatible with either answer, because it publishes none. The question must be settled before T4 designs M08.

## 13. Convergence path (not part of T0R)

Once T4 (bends and fittings), T5 (nonlinear supports) and T6 (combinations) extend the exact profile, the ordinary route and the exact route can converge. Option (c), the exact profile as the authoring default with migration of 0.2.0 documents, then becomes the right default. At that point preview-physics-1 becomes historical in turn.

## 14. Sources read, and limits

**Instructions:** Root `AGENTS.md`; `agents/AGENT_HELPS_HUMANS.md`; `P/AGENTS.md`.

**Records** (under `…/CONTINUATION_2026-09-24/` unless noted):

- `T0_REASSESSMENT/RETURN.md` and its probe; read only, not edited;
- `SOLVER_FINDINGS_ASSESSMENT/ASSESSMENT.md`;
- `CORRECTNESS_DESIGN/{VERSION_RESERVATION_V2.md, VERSION_RESERVATION.md, RESULTS_AND_COMBINATIONS.md, STRESS_REFERENCE.md, COMPOSITE_ENGINE/SELECTION.md}`;
- `RAW02_INTERFACE_ALIGNMENT.md` at `9e8a55da`;
- `ENGINE_INTEGRATION/PHYSICS_READER_JOIN/BRIEF.md`;
- `OWNER_PHYSICS_AUTHORITY_2026-09-25.md`, `OWNER_ROUTE_DIRECTION_2026-09-25.md`, `OWNER_RESUME_2026-09-26.md`;
- the work graph's current route, and its T0R row at `13df6cc74`.

**Source read only:**

- `PP` (producer, reactions, stress, SIF, combinations, macro-bend construction), `pressure_runtime.rs`, `source_recovery.rs` gate, `curved_bend/src/lib.rs`;
- `result_export/src/semantic_contract.rs`, `derivative.rs`;
- `analysis_runs/compatibility.py`, `stress_neutral/package_v0_3.py`;
- the desktop readers and panels named in §6;
- the T1 branch diff `eb56e1083..c0ef4a8e0` for product paths.

**Executed:**

- `references.py`, with the Python standard library only;
- one read-only probe crate, built outside the repository against the worktree's `core/product_physics` with the design target directory. The target directory and the scratch copy were deleted afterwards.

No product, test, fixture or other record was changed, and no Git write was run.

**Limits:**

- The reference set is mine and not yet independently checked; S0 exists for that.
- The probe observes main only on the invented cases stated.
- The line numbers drift with T1.
- I did not run the existing test suites, the native app, the browser, CI or a DEC-025 sweep.
- The consumer inventory comes from grep plus reading and may miss a site. Each implementing PR must add its own actual inventory, as RESULTS_AND_COMBINATIONS requires.

**Records in this folder:**

- `references.py`, `_run_records/references.stdout.txt`;
- `_run_records/design_probe/` (source and manifest; the manifest's dependency path is repository-relative), `_run_records/design_probe_output.log`, `_run_records/toolchain.txt`;
- `_run_records/SHA256SUMS`.
