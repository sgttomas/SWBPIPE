# T4-U4b and T4-U5b on model 0.6.0: implementation design

This is the implementation design for shared epoch 4 (DEL-04-08 Design
[b7-plan.md](../../DEL-04-08/Design/b7-plan.md), "After B7b: the shared epochs", item 4).
T4-U4b publishes the arc quantities that U4a withheld, and the straight-member quantities
qualified with them ([arc-frames-and-signs.md](arc-frames-and-signs.md) §9). T4-U5b admits
explicit elastic tie rods on an objective connector ([joint-pressure-thrust.md](joint-pressure-thrust.md)
§9). Both land together on model 0.6.0. The new ordinary identities are `pressure-2` and
`preview-physics-2` ([pressure-contract.md](pressure-contract.md), "U4b/U5b formats and identities").

The physics and its qualification are not restated here. They are in the two §9 sections
and in the frozen references
[`validation/references/t4_u4b/`](../../../../validation/references/t4_u4b/README.md) and
[`validation/references/t4_u5b/`](../../../../validation/references/t4_u5b/README.md).
This file states what the product publishes, in which format, under which identity, and how
it is checked. It also decides the open points those sources leave to this design. The
oracles are never altered to fit the product.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Status

Designed on main `5c4aa0b747`; not implemented.

## Rulings this design applies

- **Owner, 2026-10-10:** "nothing needs to be preserved for backwards compatibility…";
  one current version per format, no migration ("no-compatibility principle carries").
- **HELP_HUMAN, 2026-10-13 (technical):** later U4b/U5b input additions use model 0.6.0 and
  retire and re-author 0.5.0. No inactive future fields were reserved in 0.5.0.
- **Owner, 2026-10-14 resumption steer:** the raw/component result format stays 0.2 and the
  canonical result format stays 0.3. The ordinary semantic identities become `pressure-2` and
  `preview-physics-2`, retiring `pressure-1` and `preview-physics-1`.
- **HELP_HUMAN, 2026-10-17, on U4b:** the two selected idealizations and how each is read
  ([arc-frames-and-signs.md](arc-frames-and-signs.md) §9, ruling (a)).
- **HELP_HUMAN, 2026-10-17, on U5b:** (1) non-parallel rods are admitted under the projected
  rule; (2) rods with p = 0 at the joint are admitted
  ([joint-pressure-thrust.md](joint-pressure-thrust.md) §9).
- **HELP_HUMAN, 2026-10-17, on retained and source carriage of the new kinds (ruling D).** The
  new kinds are published on every route, W1-L and W1-E successors and pressure-source-1
  publications included. In a retained successor they are `not_covered` rows and keep their
  ordinary values. The basis is DEL-04-08 ScopeOfWork REQ-001, which the owner approved on
  2026-10-17 ("I approve #1405 as required"):
  > Every published row of a successor is classified as exactly one of `relative_verified`,
  > `absolute_verified`, `input_derived`, `non_quantity` or `not_covered`.

  > Only these row kinds can carry a verified class: …

  > These rows are `not_covered` and keep their ordinary values: every other row kind; …

  Certifying the new kinds would add them to that closed list, which changes an owner-approved
  commitment. That stays the owner's and is not taken here. Two conditions return to HELP_HUMAN
  before acting:
  - a change to reviewed-input wording that TBD-002 governs;
  - an actual-source D3 that cannot fit the unchanged M = 12 GiB policy (Decision 22).
- **HELP_HUMAN, 2026-10-17, on this design's two proposals.** No objection to either:
  - the retained and source identities keep their names (§1.2);
  - frozen reference requests keep their bytes and are re-authored in memory (§1.4).

## 1. Formats and identities

### 1.1 Model document 0.6.0

0.6.0 is the only current model version. It is 0.5.0 with exactly two additions: the
directional SIFs (§2.6) and the tie-rod hardware (§3.1).
- **0.5.0 is retired.** It joins `RETIRED_MODEL_SCHEMA_VERSIONS` and is refused by name
  everywhere a retired version is refused today. On solve that is `MODEL_SCHEMA_VERSION_RETIRED`
  with the re-author text. On desktop open and save, in persistence and in the browser mirror it
  is `retired_schema`. Nothing is migrated.
- **No inactive fields.** Every member 0.5.0 could carry keeps its meaning. A 0.5.0 document is
  re-authored by changing `schema_version` alone, unless it uses an addition, which none can.
- **The constants move together.** These must all name 0.6.0:
  - `model_format.rs` `MODEL_SCHEMA_VERSION`;
  - `case_state::LOAD_STATE_MODEL_VERSION`, which follows it;
  - the desktop's `model_document_version.rs`, `projectService.ts` and `loadStateAuthoring.tsx`;
  - the qualification tools' `MODEL_VERSION`;
  - the integration test helper `tests/common/current_model.rs`;
  - `schemas/model.schema.yaml`'s `schema_version` enum;
  - every literal the model-format survey found in the applier, the readers, the Python
    verifiers and the desktop authoring code.

  A version-keyed literal that stays at 0.5.0 is a defect. The check is a repository grep: no
  `"0.5.0"` model-version literal outside the retired list, and frozen bytes are exempt.
- **Behaviour still keys on presence.** It keys on the pressure contract, the load-state group,
  the directional SIFs and the rod hardware, never on the version string.

### 1.2 Result identities

| Identity | Before | After |
|---|---|---|
| `openpipestress.result_semantics/0.3.0/pressure-1` | current | retired, refused by name: `SOURCE_PRESSURE_1_RETIRED` (re-run) |
| `openpipestress.result_semantics/0.3.0/preview-physics-1` | current | retired, refused by name: `SOURCE_PREVIEW_PHYSICS_1_RETIRED` (re-run) |
| `…/pressure-2` | — | current exact ordinary identity, profile `exact_pressure_v3`, table `semantic_contract_v0_3_pressure_2.json` |
| `…/preview-physics-2` | — | current preview ordinary identity, profile `product_preview_mechanics_v1`, table `semantic_contract_v0_3_preview_physics_2.json` |
| `physics-retained-1`, `preview-physics-retained-1`, `pressure-source-1` | current | current under the same names, revised in place |

- **The -1 tables are deleted.** Their stored results are refused at header dispatch, beside
  the other retired identities (`SOURCE_PRECISION_1_RETIRED`, `SOURCE_BLOCKS_1_RETIRED`,
  `SOURCE_EXACT_PRESSURE_V2_RETIRED`). The refusal is in the Rust reader, and through it in the
  desktop and `compatibility.py`, and in the Python verifier. An analysis-run record that binds
  a retired table is refused as today (`ANALYSIS_SEMANTIC_CONTRACT_MISMATCH`).
- **The retained and source identities keep their names** and re-inherit the new bases by hash.
  This follows B7b D4 (a), where `physics-retained-1` was rebased from physics-1 to pressure-1
  under the same name.
  - `physics-retained-1`'s `inherited_semantic_contract_sha256` becomes pressure-2's table hash,
    and its rows follow pressure-2's.
  - `preview-physics-retained-1` inherits preview-physics-2 by hash and keeps its one extra row,
    `preview-physics-retained-requested-mode-basis`.
  - `pressure-source-1` keeps its own table. That table takes the new kinds that its source
    family can carry (§4.3).
  - **The meaning hashes move.** DEF-O, DEF-C and DEF-E name their base in `inherits.base_contract`.
    That name becomes preview-physics-2 or pressure-2. DEF-E's prose names pressure-1 and
    pressure-source-1, and its pressure-1 references become pressure-2. Their meaning hashes
    therefore move: `DEFINITION_HASH`, `COMBINATION_DEFINITION_HASH` and `EXACT_DEFINITION_HASH`
    in `retained_wire.rs`, the Rust reader and the Python verifier, and the tables'
    `product_formation_definitions`. Nothing else in the definitions changes (§4.1). TBD-002's
    stale `accuracy_classification.scope` sentence in the physics-retained-1 table is left as it
    is. If any edit here would need to touch it, that goes back to HELP_HUMAN first.
- **The pressure result schema is renamed** from `schemas/results_pressure_1.v0.3.schema.json`
  to `schemas/results_pressure_2.v0.3.schema.json` (with `git mv`). Its identity `const`s name
  pressure-2. `results.v0.3`, `analysis_run.v0.3` and `stress_neutral_export.v0.3` keep only
  current branches, under the new identities.
- **The result formats stay.** Raw/component stays 0.2 and canonical stays 0.3, as the steer
  states.
- **Tests that used the -2 strings as unknown controls change.** They now use a -3 string. These
  are `result_export/tests/pressure_contract.rs:86`, `tests/test_pressure_contract_readers.py:113`,
  `tests/test_preview_physics_consumer_contract.py:458`, `tests/test_retained_precision_b3.py:950`,
  `load_reference_mutations.json:3027` and `load_reference_source_mutations.json:190, 1886`.
  The physics-retained-2 and pressure-source-2 controls stay as they are, because those
  identities do not exist.

### 1.3 Reviewed inputs

The order of `REVIEWED_INPUTS` (`build_identity.rs`) is unchanged. These entries move:
- index 3, DEF-O;
- index 4, the preview-retained table;
- index 5, now the preview-physics-2 table;
- index 6, DEF-C;
- index 7, DEF-E;
- index 8, the physics-retained table;
- index 9, the pressure-source table;
- index 10, now `results_pressure_2.v0.3.schema.json`;
- index 11, now the pressure-2 table.

All of these require the actual-source D3 (§6).

### 1.4 Re-authoring

- **Committed current documents are re-authored to 0.6.0** by their own writers wherever a
  writer exists. The model-format survey lists them: product_preview fixtures,
  pressure_source fixtures, w1_controls, the contract corpus, the results successors and
  corpora, the operation fixtures, and the invented examples.
- **Hand-authored documents are edited by version only.** Their binding hashes follow their
  bytes.
- **Compressed corpora are regenerated by their generators.** These are
  `retained_precision_cases.json`, `pressure_source_reader_corpus.json` and the
  `pressure_v3_*_reader_corpus.json` files. Every captured-invocation digest follows its actual
  bytes. That includes the j7 consequence digests, `invocation_sha256` and the two preview
  generation records' source pins.
- **Frozen reference requests keep their bytes.** That covers t4_u5a, t4_u6, t4_u8, t4_i8,
  kd5_timoshenko and the already-older t4_u4a, t4_i7, t4_i12 and t3_r1. Tests check each
  recorded SHA, then re-author an in-memory copy to 0.6.0 through the shared helper, the t4_u4a
  precedent.
  - No oracle, `SHA256SUMS` or t4_u5b base pin moves.
  - The helpers re-author 0.5.0 by version alone. Older versions keep their existing contract
    and allowance rules.
- **The U4b and U5b builders add 0.6.0 requests.** Only the t4_u4b and t4_u5b builders add
  request bytes and request SHA-256 values, as their READMEs allow. No oracle value moves.
- **The four owner-confirmed 0.5.0 re-authorings** (b7-plan, "Decisions applied") become 0.6.0
  edits of the same files. `tests/test_model_schema.py::check_domain_fixtures` expects 0.6.0.
- **The frozen UI benchmark is unchanged.** Its four 0.2.0 models remain under the pending owner
  decision (b7-plan).

## 2. U4b: quantities and rows

### 2.1 Scope

- **Arcs and straights.** U4b publishes these on every member of both ordinary routes, so a
  kind means one thing on every member:
  - section normal-stress extremes σ⁺/σ⁻ with their fibres;
  - the Saint-Venant transverse-shear maximum.
- **Arcs only.** On realized arcs it publishes:
  - the arc normal-stress maximum and the arc's part in the headline;
  - directional-SIF intensified bending and its along-arc maximum;
  - on pressure-2, the toroidal membrane hoop and the radial surface values.
- **Algebra marks.** It marks the algebra class of every row kind (§2.7).
- **On every route (ruling D).** The new kinds appear wherever their members appear. W1
  successors and pressure-source-1 publications carry the straight-member kinds as
  `not_covered` rows with their ordinary values (§4). Arcs still never reach W1 or the source
  route, because D7 is unchanged.

### 2.2 Row-kind map

"Stations" means `end_i`, `quarter_1`, `midspan`, `quarter_3`, `end_j`. "Bases" means
nominal and reduced on pressure-2 (`push_basis_twins`, the D-5 suffixes) and the single section
on preview-physics-2.

| Oracle key (t4_u4b) | Row kind | Component(s) | Unit | Members | Locations | Route |
|---|---|---|---|---|---|---|
| `normal_extremes.sigma_plus`, `.sigma_minus`, `*_fibre` | `element_local_normal_stress_extreme` | `sigma_plus`, `sigma_minus` | MPa | all | stations × bases | both |
| `sv_transverse_shear_max.value` | `element_local_transverse_shear_stress_maximum` | `tau_v_max` | MPa | all | stations × bases | both |
| `member_maximum` (arc) | `pipe_elastic_normal_stress_maximum_v2`, as on straights | as on straights | as on straights | arcs (joining straights) | `governing_station` plus location evidence | both |
| `headline` | the existing headlines, now covering arcs | — | — | — | — | both |
| `intensified_bending` | `curved_bend_directional_intensified_bending_stress_v1` | `intensified_bending` | Pa | arcs with directional SIFs | stations × bases | both |
| `intensified_bending_maximum` | `curved_bend_directional_intensified_bending_maximum_v1` | `intensified_bending_maximum` | Pa | arcs with directional SIFs | one per basis, with location evidence | both |
| `toroidal_membrane_hoop` (`y_pos`, `z_pos`, `y_neg`) | `pipe_toroidal_membrane_hoop_stress_v1` | `toroidal_intrados_hoop_stress`, `toroidal_crown_hoop_stress`, `toroidal_extrados_hoop_stress` | Pa | pressurized arcs | stations × bases | pressure-2 |
| `radial_surface` | `pipe_toroidal_radial_surface_stress_v1` | `toroidal_inner_radial_stress`, `toroidal_outer_radial_stress` | Pa | pressurized arcs | stations × bases | pressure-2 |

**Not published, and why.** The product does not publish these oracle keys as rows. The
references keep them as checks of the definitions.
- `fibre_normal` at the four named fibres. Each is a signed sum of the published
  `element_local_*_normal_stress` components, so it adds no information. σ⁺/σ⁻ give the
  governing signed fibre values with their fibres.
- The `sv_transverse_shear` field at six fibres and both surfaces. It is the qualification of
  the field. A stress_recovery unit test consumes it against the field function (§5.1).
- `axial_plus_intensified_bending`. It is |N_eff|/A_b plus the published intensified value,
  and both terms are published. The mutants `sif_applied_to_whole_max_normal` and
  `sif_applied_to_axial` are therefore not formed in the product.
- `flexure_plus_torsion_check`. This is a check, not a quantity.
- `torsional_shear` is already published (`element_local_torsional_shear_stress`) and is
  compared unchanged.
- The diagonal hoop fibres and `z_neg`. They are the same function of β, and the crowns are
  equal.

**Row economy.** Each member gains three rows per station and basis (σ⁺, σ⁻, τ_v,max). That is
15 on preview-physics-2 and 30 on pressure-2. Arcs gain the intensified and hoop rows above.
The registered profile prices this through the row bounds in §6.

### 2.3 Normal-stress extremes and the arc maximum

**Extremes.** σ⁺ = N_eff/A_b + hypot(M_y, M_z)/Z_b and σ⁻ = N_eff/A_b − hypot(M_y, M_z)/Z_b at
each station and basis.
- N_eff is N_w on a pressurized member of pressure-2 and N otherwise (t4_u4b item 3).
- Each row carries metadata `fibre_location` = {`y_m`, `z_m`}, the outer-surface fibre
  ±r_o(−M_z, M_y)/hypot(M_y, M_z) in the section's local frame.
- When hypot(M_y, M_z) = 0, the fibre is `{"tied": true}` and σ⁺ = σ⁻.
- **Formation.** On the certified route (K-D5 Passed), each extreme is formed at p from the
  certified station resultants and rounded once. Its radius follows the components' radii
  through the hypot's Lipschitz bound. On the binary64 route it is the binary64 formula of the
  published resultants.
- **The basis token.**
  - On straights: `recovered_from_open_mechanics_stress_components`.
  - On arcs: the preview `nominal_straight_beam_formula_on_arc_resultants` and its pressure-2
    twin. This is the U4a arc stress basis, already disclosed.

**The arc maximum.** This is the maximum over ψ ∈ [0, φ] of f(ψ) = |N_eff(ψ)|/A_b + hypot(M_y(ψ), M_z(ψ))/Z_b
(t4_u4b item 4).
- **Resultants.** The resultants along the arc are formed from the end-i section action cut(0)
  and the uniform member load w per unit arc length. They are expressed in the tangent frame at
  ψ, with the closed forms of t4_u4b item 4: F(ψ) = F_beyond + w R Δψ, M(ψ), and their exact
  derivatives.
  - The implementation must confirm that the product's arc uniform load is a constant global
    force per unit arc length (t4_u4b inferred convention 1, `certify_curved_uniform_load`).
  - If it is not, that is reported before any code relies on the convention.
- **The method** is the certified branch-and-bound of t4_u4b item 4, in the product's own
  arithmetic:
  1. Start from a uniform grid of 4,096 cells in ψ.
  2. Upper bound per cell: max(endpoint values) + K h²/8, with K = (2R|w| + F_max)/A_b + (R²|w| + M_max + R F_max)/Z_b.
  3. Lower bound: the largest evaluated value.
  4. Bisect every cell whose upper bound exceeds the lower bound by more than the stopping gap.
     Stop when gap ≤ 2^-40·lower, or refuse at the existing `elastic_extrema` limits (131,072
     subdivisions, depth 48).
  5. Publish as the straight maximum publishes: the binary64 midpoint of [value_lower,
     upper_bound], with value_lower, upper_bound, the governing fraction ψ*/φ and the governing
     fibre in its evidence.

  Every evaluation carries an outward allowance for its binary64 rounding (a γ_n bound on the
  closed-form expression, stated in the module). So the upper bound is a bound on the
  binary64-represented problem, not an estimate.
- **On the certified route.** The resultant enclosure adds the radii of cut(0). The bound gains
  ΔF/A_b + (ΔM + R φ ΔF)/Z_b, which also covers the Lipschitz transport of the force radius along
  the arc.
- **Where it lives.** The method is DEL-05-03's `stress_recovery` module (new function beside
  `elastic_extrema`; it is a supplier need of DEL-04-07, §8). PP calls it in the member loop
  where `exact_member_maximum_policy` withholds arcs today.
- **When it is withheld.** It is published only when the cut and its load are available. A case
  whose arc recovery is unavailable keeps the existing unavailable path and warning
  (`EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE`), now with the specific reason.
- **What is removed.**
  - `exact_member_maximum_policy`'s arc withholding.
  - `withheld_on_arcs`, `ARC_WITHHELD_REASON`, and the arc_evidence and verifier checks that
    require them.
  - The preview `outside_domain_pipe_ids` placement of arcs.

**Headlines.** The existing headline rules apply, with arcs no longer outside the domain. On
pressure-2 there is one headline per basis, nulled when any member lacks its basis maximum. On
preview-physics-2 the headline is set when coverage is complete. The interior location band of
t4_u4b is a comparison rule for the test. It is not published.

### 2.4 Saint-Venant transverse shear

τ_v,max = |V| ((3+2ν) r_o² + (1+2ν) r_i,b²) / (4(1+ν) I_b) at each station and basis, with V =
(V_y, V_z) in the section frame (t4_u4b §2).
- ν is the member's material ν.
- Each basis uses its own annulus.
- It is one formula on straights and arcs. On arcs the basis token discloses
  `local_straight_annulus_on_arc`; on straights it is `saint_venant_annulus_flexure_v1`.
- Torsional shear stays its own row.
- No location metadata is published. The two fibres are fixed by V (inner surface, normal to
  V), and the basis text states this.
- **Formation** follows §2.3's extremes: from certified V at p with its radius, or binary64.

The module also exposes the field τ(r, β) for its unit test (§5.1). The field is not published.

### 2.5 Toroidal hoop and radial values (pressure-2)

On each pressurized arc and basis b:
- σ_h(β) = (p r_i,b/t_b)(2R − r_i,b cos β)/(2R − (r_i,b + r_o) cos β) at β = 0 (intrados,
  `y_pos`), 90° (crown, `z_pos`) and 180° (extrados, `y_neg`);
- the radial values −p at r_i,b and 0 at r_o.

They are published at the five stations, as the straight Lamé rows are (the values are
station-independent; t4_u4b inferred convention 2).
- **Formation.** In binary64, as the straight Lamé rows are: an `input_derived` quantity with no
  solve bound. The domain R > r_o is already guaranteed by bend realization; a violation refuses
  by its existing geometry code.
- **The evidence.** The region evidence for an arc region names the definition
  `bore_load_transferred_toroidal_membrane_v1` and the shear definition
  `local_straight_annulus_on_arc`, each with its neglected effects (the §9 gap) in a static
  sentence.
- **`BEND_LIMITATION`** replaces its last sentence ("On arcs the straight Lame hoop and radial
  values and the governing normal-stress maximum are withheld; …") with:

  > On arcs the hoop is the toroidal membrane cut average (bore_load_transferred_toroidal_membrane_v1) and the radial values are -p at the bore and 0 outside, the transverse shear is the straight-annulus Saint-Venant field driven by the tangent-frame shears (local_straight_annulus_on_arc), and the normal-stress maximum is bounded along the arc; ovalization, hoop bending from moments, bending boundary layers, tangent-end effects and the curvature correction of the shear field are not represented; the excluded effects above apply to every bend result.

  The text is priced under D3.
- **Arcs outside every pressure region** carry no hoop or radial rows. This is the straight
  convention: the pressure family is published only for members of a region. It decides
  t4_u4b's open point; the formulas are linear in p, so nothing is lost.

### 2.6 Directional SIF input and intensified bending

**Field specification (model 0.6.0).**

| | `sif_in_plane_user_value` | `sif_out_of_plane_user_value` |
|---|---|---|
| Location | `components[].modifiers` of a bend component | same |
| Type | Quantity, dimensionless (unit `"1"`, as `sif_user_value`) | same |
| Constraint | finite and > 0 (a user factor; no code floor is imposed, F-PIP-1) | same |
| Multiplies | M_z of the arc's tangent frame | M_y of the arc's tangent frame |

- **Both or neither.** One without the other is refused.
- **Source required.** `modifiers.source_reference` is required when they are present. It is
  the same field k uses. A bend that carries both k and SIFs states one source for its user
  data.
- **The named refusals** all give `MODEL_INCOMPLETE` and no results:
  - `BEND_DIRECTIONAL_SIF_INCOMPLETE`: only one of the pair, a non-finite or non-positive value,
    a wrong dimension, or no source;
  - `BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC`: a pair on a component that is not a realized curved
    bend. At geometry-only markers no vetted basis fixes the in-plane axis (t4_u4b "Markers").
- **The scalar `sif_user_value` is unchanged.** At geometry-only markers and branches on the
  preview route it keeps its current meaning, with the equal-factor
  `component_equal_factor_intensified_bending_stress_v1` rows. On a realized arc it keeps the
  info diagnostic `COMPONENT_STRESS_INTENSIFICATION_NOT_APPLIED`. It is never used as both
  directional factors, which is the mutant `sif_one_scalar_for_both_directions`.

**Rows.** intensified = hypot(i_ip M_z, i_op M_y)/Z_b at each station and basis. The along-arc
maximum uses §2.3's method with the intensified measure (K scaled by max(i_ip, i_op) on the
moment terms).
- **Never combined** (§2.7). No factor multiplies N/A, the hoop, torsion or the normal-stress
  maximum. k never multiplies stress.
- **Both routes.** Pressure-2 arcs publish them on both bases; preview-physics-2 publishes them
  on its single section.
- **The preview limitation.** LIMITATIONS[4] keeps its marker sentence and gains the arc
  sentence (§2.8).

### 2.7 Algebra-class marks

Every row signature in the pressure-2, preview-physics-2 and inheriting tables gains
`combination_algebra`, one of two values:
- **`signed_linear`**: displacements, rotations, section resultants, signed stress components,
  torsional shear, pressure-family forces and stresses, the hoop and radial rows, support
  components and connector generalized actions, tie-rod forces;
- **`never_combined`**: σ⁺/σ⁻, the transverse-shear maximum, every maximum and headline-bearing
  row, every intensified row, magnitudes, and review and basis rows.

This is a classification (STRESS_REFERENCE §5), not a new combination rule.
- The ordinary combination engine keeps its behaviour: maxima and intensified rows are not
  combined, with the existing info diagnostics. It extends that behaviour to the new
  `never_combined` kinds. `COMBINATION_STRESS_MAXIMUM_UNAVAILABLE`'s text becomes "stress maxima
  and section stress extremes are not combined; combine the signed stress component rows
  instead".
- A table test asserts that every `never_combined` kind is absent from every combination output
  in the committed corpora, and that every combined kind is `signed_linear`.
- Magnitudes stay recomputed from combined components, as today. Their mark is
  `never_combined`, because they are recomputed and not combined.

### 2.8 Preview-physics-2 limitations

- **LIMITATIONS[2]** becomes:

  > On every member the section normal-stress extremes are N/A ± hypot(My,Mz)/Z at each station and the transverse shear maximum is the straight-annulus Saint-Venant value. On straight members the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No equivalent stress or code stress.

- **LIMITATIONS[3]** becomes:

  > On arcs: signed tangent-frame resultants, nominal stress components and section extremes; endpoint force rows are node-on-element actions in the tangent frame at their end. The normal-stress maximum is bounded along the arc and enters the stress headline. The transverse shear uses the straight-annulus field without curvature correction.

- **LIMITATIONS[4]** appends:

  > On realized arcs with directional SIFs, intensified bending is hypot(i_ip*Mz, i_op*My)/Z in the tangent frame, with its along-arc maximum.

The pressure-2 table's limitations take the same additions where they apply, together with
`BEND_LIMITATION` (§2.5) and `CONNECTOR_LIMITATION` (§3.4).

## 3. U5b: explicit elastic tie rods

### 3.1 Field specification (model 0.6.0)

`components[].objective_connector.hardware` (objective_connector 1.0.0) is a closed object with
two kinds:
- `{ "kind": "untied" }` (unchanged);
- `{ "kind": "explicit_axial_rods_v1", "rods": [Rod, …] }`, with at least one rod.

`Rod` is a closed object:

| Member | Type | Constraint |
|---|---|---|
| `id` | string | non-empty, unique within the connector |
| `law` | string | `"bilateral_linear"` |
| `offset_i`, `offset_j` | `{ "x": Quantity<length>, "y": …, "z": … }` in the connector axes Q | finite; required |
| `stiffness` | Quantity, force per length | finite, > 0 |
| `reference_extension` | Quantity, length | finite (negative is pretension) |
| `source_reference` | string | non-empty |

- **Ends and offsets.** A rod's ends are always the connector's end nodes. Its attachment points
  are b_i = x_i + Q·offset_i and b_j = x_j + Q·offset_j, with each global offset formed as
  o = fl(Q·offset), as connector attachments are. A rod anchored anywhere else cannot be
  expressed. That decides the references' "rods whose end nodes are not the connector's end
  nodes" open point by construction.
- **Offsets are in the connector axes.** That is the natural frame for a rod at ±a along a
  connector axis. The t4_u5b builder writes `offset_connector_axes_m` directly.
- **Named refusals** (all `MODEL_INCOMPLETE`, on the connector):
  - `JOINT_HARDWARE_LAW_UNSUPPORTED`: a kind other than the two; a law other than
    `bilateral_linear`, for example `ideal_inextensible`, `tension_only`, `gapped` or `stop`.
    Each names the law, and an MPC or constraint law is a later unit, on the owner's choice
    only.
  - `JOINT_TIE_ROD_INPUT_INVALID`: an empty `rods`; a duplicate id; a missing offset or member;
    an unknown member; a non-finite or non-positive stiffness; a non-finite reference extension;
    a wrong dimension; an empty source. These are JR §5's "duplicate rods, missing offsets,
    contradictory hardware" (for example `untied` with `rods`).
  - `JOINT_TIE_ROD_ZERO_LENGTH`: l = |b_j − b_i| is zero. The check uses the represented
    difference (x_j − x_i) + (o_j − o_i), formed componentwise as the connector forms r.
- **Model findings come first.** These are connector model findings (`model_findings`), so §1
  precedence applies and a refused listed connector carries only its code.

### 3.2 Mechanics

**Elements.** Each rod is its own two-node element of DEL-04-01's frame kernel, beside the
connector. Its kinematics are e = n·(v_bj − v_bi) = C d, with v_b = u + θ × o and
n = (b_j − b_i)/l. Its stiffness is Cᵀ k_r C, and its reference load is +Cᵀ k_r e_ref. Its
recovery is T_r = k_r(C d − e_ref).
- **The connector stays elastic.** Its g = K(q − q_ref), its rows and its q_ref primitive are
  unchanged. Rods are not connector member loads, and rod forces are never folded into g (the
  mutant `rod_force_folded_into_g`).
- **Why a separate element.** It is the references' nodal stiffness formulation. The rod law
  needs no reduction e = c·q, which holds only for zero connector offsets. And the K-D5 closure
  binds each rod's primitives separately.
- **Arithmetic.**
  - n and l come from the represented difference.
  - C's rotational entries are the exact dot products of n with the binary64 offset cross
    products, rounded once.
  - Ke and the reference load are `ExactAccumulator` sums rounded once, as in the connector
    module.
  - The infinitesimal rigid modes C·d = 0 hold to the formation bound. A unit test checks all
    six modes on generic skew rods.
- **K-D5.**
  - FK's formation check re-forms each rod's stiffness at p from its binary64 inputs: the node
    coordinates, Q, the offsets and k_r. n and l are formed in ball arithmetic.
  - A new family `LoadFormula::TieRodReference { connector, rod, … }` re-forms +Cᵀk_r e_ref at p.
    Its summand identity is `{connector}:{rod}:tie_rod_reference`. "One ε": omitted only when
    e_ref is exactly 0.
  - Certified rod recovery `certify_tie_rod_recovery` gives T_r as a certified row of the
    corrected displacement. Its radius comes from the displacement ball through |C| and from
    k_r's formation.
  - W2 forms the same element at 2^b, with classification and the complete manifest retained.
- **Supplier need.** The element, its family and its certified recovery are DEL-04-01's (§8).
  DEL-04-01 Design `kd5-member-relative-bound.md` §1 gains the family row when it lands.
- **Pressure.** Rods carry no pressure load. The ledger (representation B), its caps, the joint
  pair and the Poisson pairs are unchanged (t4_u5b "Pressure RHS": rods never change it).

### 3.3 Publication

- **`connector_tie_rod_force_v1`.** One row per rod: component `t_r`, unit N, location
  `tie_rod`, and metadata `tie_rod_id`. Row id
  `result:connector:{suffix}:tie-rod-force:tie_rod:{encoded rod id}:t_r`. Tension is positive,
  T_r = k_r(e − e_ref). It is certified on the certified route and binary64 otherwise.
- **`s_x`** (`connector_effective_axial_force_v1`) becomes S_joint = g_x + Σ T_r (n_r·Q_x) − Pe
  (ruling 1). On the certified route it is formed from the certified g_x and T_r rows and
  n_r·Q_x formed at p, rounded once, with the radii summed. It is published:
  - for every constant-effective-area connector, as today (Pe = 0 when no region lists it,
    ruling 2);
  - and for every connector with rods, whatever its pressure model. For an unpressurized
    connector with rods, Pe = 0 and s_x is the face axial force. That decides t4_u5b's open
    point: the face-equilibrium identity is published wherever rods carry part of it.
- **`pe_x`** is published only for a constant-effective-area connector, as today. An
  unpressurized connector has no Ae.
- **Unchanged rows.** An untied unpressurized connector's rows and evidence are unchanged, byte
  for byte.
- **The connector evidence record.** `hardware` becomes the authored kind. With rods, `tie_rods`
  lists, per rod: `id`, `law`, `offset_global_m` [i, j], `stiffness_n_per_m`,
  `reference_extension_m`, `length_m`, `direction_global`, `axial_share` (n·Q_x) and
  `source_reference`.

### 3.4 The open points this design decides

- **A rod nearly perpendicular to the joint axis is admitted, with no refusal.** Ruling 1 lets
  this design refuse it by name only if that is justified, and nothing justifies it:
  - the rod element's assembly and certificate do not depend on n·Q_x;
  - its stiffness, extension and force are well defined at every direction;
  - S_joint's projected term only becomes small.

  A threshold would be a parallelism tolerance with no physical basis, the same reason ruling 1
  gives for admitting non-parallel rods. At n·Q_x = 0 the rod contributes nothing to S_joint and
  remains a transverse elastic element. The evidence publishes `axial_share`.
- **Rods with p = 0 are admitted (ruling 2).** No route or certificate reason against them was
  found:
  - TieRodReference has no pressure dependence;
  - the rod element is independent of the pressure ledger;
  - Pe = 0 makes s_x the face axial force.

  So nothing returns to HELP_HUMAN.
- **JR §5's input refusals:** §3.1's codes.
- **`ideal_inextensible` and the collective constraint** stay refused by name (§3.1).
- **`CONNECTOR_LIMITATION`** becomes:

  > Objective connectors (expansion joints, objective_connector 1.0.0): symmetric-midpoint, small-displacement, small-rotation linear elastic law with the authored 6x6 work matrix, frame, offsets and installed reference q_ref, replacing exactly one straight span (replaces_span); untied hardware or explicit bilateral linear elastic tie rods (explicit_axial_rods_v1) between the connector's end nodes with rigid offsets, each with its own stiffness and reference extension. A pressurized joint (constant_effective_area_reference_v1) is a coaxial edge of one equal-bore pressure region between straight members, with zero attachment offsets, and carries the dead pressure load p(Ae - Ai) at its attachments, Ae being the authored effective area; unequal bore or pressure, offset faces and joints at region terminals or beside realized arcs are refused. Ideal inextensible rods, collective constraints, tension-only or gapped rods, nuts and stops are refused. No finite rotation, follower or geometric stiffness, pressure stiffening, squirm or stability, temperature law or stop is modelled; the replaced span carries no load, member result or self-weight. Connector actions are published as generalized and end-node elastic actions, with each tie-rod force, the joint pressure force pAe and the effective axial force S = g_x + sum T_r (n_r.Q_x) - Pe published separately; they are outside the straight-member recovery coverage.

## 4. Retained and source routes (ruling D)

### 4.1 W1-L and W1-E

The new straight-member kinds are `element_local_normal_stress_extreme` and
`element_local_transverse_shear_stress_maximum`.
- **Classed `not_covered`.** They join "every other row kind" of REQ-001: `not_covered` rows with
  their ordinary values, never `RowClass::Unpublishable`. The classification is consistent
  across every layer that applies it:
  - the kernel's row class (`k::RowClass`) and `retained_wire`'s translation;
  - the Rust reader's `row_kind` and G5c, native and wasm;
  - the Python verifier;
  - the inheriting tables' `accuracy_classification`, if they enumerate kinds.
- **No new retained recipe, no G8 re-derivation and no DEF text** beyond `inherits.base_contract`
  (§1.2). The arc-only kinds and tie-rod rows cannot reach W1: realized arcs and connectors stay
  declined at capture.
- **Unchanged.** The intensified-measures refusal (`retained_product.rs:2896`), G5b, every gate,
  limit, tolerance and verified-kind list.
- **A genuine ordinary failure.** In a failed-ordinary successor there are no ordinary values.
  The new kinds follow whatever the existing `not_covered` kinds do there. The implementation
  confirms that treatment in the producer and both readers before relying on it, and reports it
  if no existing `not_covered` kind reaches that path.
- **The maintained W1 checks keep their capability.** The B4/D1 census
  (`b4_w1_covers_every_formerly_selected_source_block_case`), the group-3 census and the
  member-load censuses must publish the same selections at the same or better standing. The new
  rows only add `not_covered` entries. A selection lost to the new kinds is a defect, not a
  declared change.

### 4.2 Combinations

The W1 combination route (DEF-C) does not combine `never_combined` kinds. That is already its
rule for maxima and intensified rows (REQ-001: "Combination maxima, intensified rows … are not
covered"). The new extremes and shear maxima follow the ordinary engine's treatment (§2.7). They
are absent from combination owners, so no combination row count changes. The REQ-001 count of
7n + 50m + 8g rows per combination stays exact; the new kinds are not combination rows.

### 4.3 pressure-source-1

A selected retained-source publication carries the new straight-member kinds as ordinary rows,
outside the receipt's `projection.quantity` and the REQ-002 checks.
- `pressure-source-1`'s table gains their signatures.
- Its receipt and source method are unchanged.
- The source family is unchanged (D7, D1), so no arc or connector row reaches it.
- MJ6's and MJ9's byte checks re-pin to the actual new bytes, with every reservation expression
  re-derived (b7-plan, MJ9). The work counters change only through the encoded-input lengths
  that the new rows and the 0.6.0 version string change. The independent test accounts for
  each delta under the D5/MJ9 law.

### 4.4 Commitments that name 0.5.0 (reserved)

DEL-04-08 ScopeOfWork REQ-001 ("Route E domain": "current model 0.5.0 …") and REQ-002
("Eligibility": "current model 0.5.0 …") name the model version. DEL-02-05 ScopeOfWork CLM-003
and its version-check states do too. At 0.6.0 those sentences become false.
- **Proposed edit.** The version token only, 0.5.0 → 0.6.0, with no other change.
- **These are owner decisions.** They are brought with the PR, with the exact sentences and
  their consequence.
- **Until decided, the PR does not edit them.**

## 5. Readers, verifier and consumers

### 5.1 Checks per new kind

The Rust reader (`result_export`, native and wasm) and the Python verifier check each new kind
on its route, independently of the producer. The verifier shares no code with it.

| Kind | Reader check |
|---|---|
| `element_local_normal_stress_extreme` | Recomputed from the same station's published N or N_w (exact route) and M_y, M_z rows and the basis section: σ± agrees under the existing component-stress binding rule. The fibre equals ±r_o(−M_z, M_y)/hypot, or is tied when hypot = 0. Coverage: every member, station and basis. |
| `element_local_transverse_shear_stress_maximum` | Recomputed from the published V_y, V_z, ν and the basis annulus, under the same rule. Coverage as above. |
| arc `pipe_elastic_normal_stress_maximum_v2` | Coverage, basis twins and headline binding as on straights. The value is at least every station's |N_eff|/A + hypot/Z, within the binding rule. The readers attest custody and do not replay the branch-and-bound (DEF-O `trust.validation` precedent). |
| directional intensified rows | Present exactly on arcs whose captured model has the directional pair. Recomputed per station from M_y, M_z and the captured factors. The maximum is at least every station value. |
| toroidal hoop and radial | Present exactly on pressurized arcs, at every station and basis. Recomputed from p, the basis bore, wall, r_o and R in the region evidence. |
| `connector_tie_rod_force_v1` | One per authored rod. Bound to the `tie_rods` record: the extension and law are rederived from the published node displacements and rotations under the binding rule. |
| `s_x` | gt_x + Σ t_r·axial_share − pe_x under the binding rule, for every connector with rods or a constant-effective-area model. |

- **Arc evidence.** `arc_evidence.rs` and `tests/verifiers/pressure_1.py` (renamed
  `pressure_2.py`) drop `withheld_on_arcs`. They require the two definition ids and the new
  rows' coverage. `physics_evidence` stops refusing the formerly withheld kinds on arcs.
- **Connector record law.** `connector_evidence.rs` `RECORD_LAW` and the verifier accept the two
  hardware kinds and the `tie_rods` record, and refuse a rod record without rows, rows without a
  record, a duplicate rod and an `untied` record with `tie_rods`.
- **Shared corpora, regenerated by their generators.** These are the J2 corpus, the pressure
  reader corpora and `base_reader_outcomes.json`. Each gains genuine entries for the new kinds,
  plus tamper mutations: a resealed changed σ±, τ, hoop, rod force or s_x; a missing row; a
  withheld-on-arcs revival; a wrong fibre. Native Rust, wasm and the verifier consume them
  unmodified.

### 5.2 Desktop and Python products

- **Route vocabulary.** `numericalResultQuality.ts`'s routes and identities, `outputPolicy.ts`,
  the Tauri literals and every desktop pin move to the -2 identities. The owner's 2026-10-15
  "Merge with the gap" stands: every desktop output surface still refuses the current exact
  identity. Its code is renamed `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`, with its text.
- **Authoring.**
  - The applier gains an edit path `objective_connector.hardware` on 0.6.0 exact-contract
    documents, with §3.1's checks. Connector creation validates hardware.
  - The bend realization accepts the directional pair with its source.
  - The desktop's connector form offers `explicit_axial_rods_v1` with a rod table. Its bend form
    offers the directional pair.
  - The live path is part of the unit, as JR J-B requires for U5a.
- **Python product paths** read through Rust and change only their pinned identities and
  tables: `compatibility.py`'s `CURRENT_RECORD_CONTRACT_IDS` and table pins, and
  `stress_neutral/package.py`'s method sets.

## 6. D3, pricing and qualification

The epoch moves priced sources (the PP producer, the readers, `model_format.rs`) and nine
reviewed inputs (§1.3). It therefore re-registers its actual selected profile on its actual
source (D3).
- **The tool and rules.** It uses `tools/retained_pricing` and DEL-04-08 Design
  `retained-pricing-rules.md`, adding a named rule for each new loop or site with its argument.
- **The qualification policy is unchanged.** M = 12 GiB (12,884,901,888 B), floor(0.9 M), the
  upward 5 % TEXT reserve and R = 64 MiB.
- **Row bounds.**
  - Route L's P_final gains three rows per member per station: 15m on the preview single
    section.
  - Route E's P_final = 7n + 72m + 8g + 3 gains 30m (two bases).
  - The census bounds (U6-RL) gain the corresponding evidence entries.
  - New static texts (limitations, sign and basis texts, the rod and hoop sentences) are priced
    TEXT.
- **Public regeneration.** Corpora, successors and outcome files are regenerated by the actual
  public producer after D3, with no private writer or temporary gate change.
- **Headroom.** The tightest phase today is merged dense W4, with 113,188,985 B of headroom. If
  the actual-source evaluation does not fit, the measured shortfall goes to HELP_HUMAN before
  any budget change (Decision 22).

## 7. Evidence plan

These checks are targeted regressions and independent scrutiny for numerical meaning and
persistence (AGENTS.md). Both solver modes run throughout.

1. **t4_u4b.**
   - A builder test authors the eight requests in 0.6.0 from `cases.json`, following its
     "Binding" rules (bend components attached at the from node, with R, the binary64 bend angle,
     k and the directional pair). It adds the request SHA-256 values to the oracle by the
     generator's `--write`, which changes bindings only.
   - The PP test runs every case through the typed and captured entries in both modes and
     compares every mapped row (§2.2) at the README criterion and zero scales:
     - interior maxima within their `location_band`;
     - fibres at the criterion with zero scale r_o;
     - the headline per basis.
   - Every discriminator formed in the product is checked to lie at least 1e3 tolerances away.
     That covers all hoop, radial, SV, fibre, maximum, SIF, k and basis mutants except the two
     `axial_plus` SIF mutants and `fibre_component_abs_sum`'s `fibre_normal` rows, which are
     not formed (§2.2).
   - The stress_recovery unit test compares the SV field at the six fibres and both surfaces,
     and the hoop at all six oracle fibres, against the oracle at the criterion.
2. **t4_u5b.**
   - A builder test authors the 38 requests in 0.6.0 from the case descriptions. Each base is
     re-expressed by the t4_u5a in-memory helper, and the rods are added in §3.1's shape, with
     offsets in the connector axes.
   - The PP and headless tests run all 38 in both modes with export and compare every frozen
     observable: q, g, Tb, each T_r, ΣT_r, Pe, S_joint, the interface identity at both faces,
     pipe Nw/S/end actions, displacements, reactions and `assembled_pressure_rhs_global`.
   - The discriminators' wrong values lie at least the stated separation away.
3. **t4_u5a, t4_u4a, t4_u6, t4_u8 and T4-I7 stay green**, through in-memory 0.6.0 re-authoring,
   in both modes. Every expected value is unchanged.
4. **Refusals.** Unit tests cover every §2.6 and §3.1 code, with `MODEL_INCOMPLETE` and no
   results, and every retired identity's refusal by name.
5. **Producer mutants.** Each is applied to the producer, run, reverted and recorded in the PR,
   with no committed seam. They are the t4_u4b mutants formed in the product and the t4_u5b
   mutants (`j4_double_count`, `rods_lumped_on_centreline`, `offset_sign_error`,
   `arm_sign_end_i_only`, `e_ref_ignored`, `reference_load_sign_flipped`,
   `reference_load_omitted`, `rod_force_folded_into_g`, `s_joint_without_rods`,
   `s_joint_section9_literal`).
6. **K-D5 closure.** For the TieRodReference family:
   - drop, duplicate and extra summands, wrong-rod and wrong-connector bindings, and wrong
     operands, on the ordinary and W2 routes;
   - FK's family and certified-recovery tests against an exact reference;
   - the rigid-mode test.
7. **Readers.** RE (native and wasm) and the verifier accept the regenerated corpora and refuse
   every tamper (§5.1).
8. **Retained capability (§4.1).** The censuses keep every selection and standing. Every new
   kind classifies `not_covered` in the producer, both readers and wasm, and none reaches
   `Unpublishable`.
9. **M3.** The arc and connector status census in both modes, in the batch-5 form: arcs gain no
   Passed↔Sensitive move from these rows, and rod cases are Passed with the member bound.
10. **D3 and registration** as §6, with independent source and pricing scrutiny.
11. **Independent review.** This design, then each major slice, gets the dual review. Codex
    supplies the independent numerical scrutiny AGENTS.md requires.

## 8. Dependencies and staging

**Supplier needs** (recorded in `deliverable.yaml` when the code lands):
- **DEL-04-01:** the tie-rod element, `LoadFormula::TieRodReference` and
  `certify_tie_rod_recovery` (§3.2).
- **DEL-05-03:** the extremes, the transverse-shear maximum and field, the toroidal hoop and the
  arc branch-and-bound maximum in `stress_recovery` (§§2.3–2.5).
- **DEL-05-02:** the base identities and readers it consumes become preview-physics-2 and
  pressure-2. The combination engine applies §2.7's marks.
- **DEL-04-08:** the not_covered classification and the re-inheritance (§4). D3 is applied
  under its pricing law.
- **DEL-02-05 and DEL-02-01:** model 0.6.0 (§1.1). Their commitment sentences that name the
  version are reserved (§4.4).

**Slices.** They land in one integration branch, in this order, each with focused checks:
1. **S1, formats.** Model 0.6.0, the identity change, re-authoring and the -1 refusals, with no
   new quantity. Expected values are unchanged; only bytes that carry the version, identity or
   hashes move.
2. **S2, U4b.** stress_recovery functions, producer rows on both routes, tables, readers,
   verifier, desktop vocabulary and the t4_u4b consumer.
3. **S3, U5b.** FK element and K-D5 family, producer, readers, verifier, authoring and the t4_u5b
   consumer.
4. **S4, retained.** The not_covered classification, the retained and source tables, and the
   census checks.
5. **S5, registration.** D3, registration, the one public regeneration, corpora and census
   refresh, and reconciliation.

S2 and S3 touch the same producer, tables and readers. They proceed in that order on one
branch.

## 9. Blast radius

**Moves:**
- every ordinary envelope (identity, table binding, limitation texts, and the new rows);
- every model fixture (version);
- every corpus, outcome file and successor that carries those bytes;
- the nine reviewed inputs;
- the registered profile;
- the desktop route vocabulary and pins;
- the analysis-run and stress-neutral schemas and packages.

**Unchanged:**
- every frozen oracle and reference request byte, apart from the U4b/U5b builders' additions;
- every numerical criterion, tolerance, gate, cap and work limit;
- D7;
- the retained verified-kind list and the recipes;
- every physical value of every existing row;
- the frozen UI benchmark.

## Open questions

None left to this design. The reserved items are §4.4's version sentences. The conditions that
return to HELP_HUMAN are the TBD-002 wording and a D3 shortfall.
