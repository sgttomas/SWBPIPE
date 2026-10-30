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

Designed on main `5c4aa0b747` and revised through four rounds of dual review. Implemented through
slices S1 to S5 (§8) on the epoch integration branch `claude/piping-t4-u4b-u5b-20261017`; the
"as built" notes record where the build settled a point. The owner decided Q2 (option (ii)) and the
0.6.0 status sentences on 2026-10-17.

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
- **HELP_HUMAN, 2026-10-17, after the round-1 reviews.**
  - *Q2, the values of the derived kinds in retained and source publications, was reserved for
    the owner.* HELP_HUMAN recommended option (ii), forming them from each publication's own
    published operands.
  - *Version tokens in reviewed inputs are within the epoch.* These are DEF-E
    `scope.model_format` and the pressure-source-1 table's `supported_profile_limitations[0]`,
    0.5.0 → 0.6.0 and the token only. They are re-registered under D3 (DEL-04-08 REQ-007) and
    reviewed against `docs/claims_registry.md` (AC-008). The physics-retained-1 table's
    `accuracy_classification.scope` sentence stays untouched (TBD-002).
- **Owner, 2026-10-17, relayed by HELP_HUMAN.**
  - *Q2:* "Form from own resultants (Recommended)", which is option (ii). In every publication,
    σ⁺/σ⁻ and the SV maximum are formed in binary64 from that publication's own published
    resultants and basis section. They stay `not_covered` with no recipe, and REQ-001's closed
    verified list, its classes and G5b are unchanged. The REQ-001 clause is edited accordingly
    (§4.4). The fallback to (i) is dropped.
  - *The 0.6.0 status sentences:* "Confirm token edits", changing the version token only. This
    covers DEL-02-05 ScopeOfWork CLM-003 (lines 33, 35, 37 and 175) and DEL-04-08 TBD-002's status
    text. Requirements and criteria are unchanged. The physics-retained-1 table's TBD-002
    sentence stays the owner's open item.
- **HELP_HUMAN, 2026-10-17, after the round-1 reviews (continued).**
  - *No objection* to the W1 not-covered passthrough, to keeping row metadata unchanged, to the
    arc maximum's existing gap rule with a defined evidence record, to the t4_u5a RV-11
    supersession note, or to the full source-work law.
- **HELP_HUMAN, 2026-10-18, on the t4_u5b AS-F-R disagreement: "take (A)."** The product
  publishes the corrected displacement fl(u + w + w2) wherever the formation check's certified
  bound exists. AS-F-R's node-B displacements had missed the frozen criterion by up to 1.97×. An
  independent exact solve of the represented request showed the cause was the published binary64
  solve, not input representation.
  - No ScopeOfWork or PRD sentence commits to publishing the solve's own displacement. The
    change revises DEL-04-01 Design (`kd5-member-relative-bound.md` §2.2 and §3) and the published
    FormulationBasis limitation text.
  - The test carve-out is removed. All 38 t4_u5b cases pass the unchanged criterion in both
    modes, and t4_u5a stays green.
  - Only rows with a certified bound are corrected: nodal displacements, their magnitudes and the
    displacement headline. Which cases keep the solve's own displacement is stated where the rule
    is built, and both paths are tested.
  - The S11-F bit-identity under W2 force scaling holds (as built): u, w and w2 are each
    bit-identical at every forced even b, so their exact sum rounded once is too, and `k2b_tests.rs`
    asserts it. Only F8 at G = 1e80 compares across the fallback: its demoted case has no bound
    and keeps u, while its Passed reference publishes the corrected value, so F8's displacement
    rows use that test's existing 2^-34 floor clause (measured difference 2.2e-19 against a floor
    of 3.7e-14).
  - The rule, its fallback and the revised limitation text are recorded in DEL-04-01 Design
    `kd5-member-relative-bound.md` §2.2 and §3.4.
  - The change gets its own independent scrutiny, as well as the epoch's dual review.

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
    pressure-source-1, and its pressure-1 references become pressure-2. DEF-E's
    `scope.model_format` ("current model 0.5.0 with the explicit 3.0.0/exact_pressure_v3
    contract") takes the version token 0.6.0. Where the definitions enumerate row recipes they
    state the not-covered passthrough (§4.1). Their meaning hashes therefore move:
    `DEFINITION_HASH`, `COMBINATION_DEFINITION_HASH` and `EXACT_DEFINITION_HASH` in
    `retained_wire.rs`, the Rust reader and the Python verifier, and the tables'
    `product_formation_definitions`. Nothing else in the definitions changes.
  - **Version tokens in reviewed product texts move to 0.6.0** (HELP_HUMAN, 2026-10-17: within
    the epoch): DEF-E's `scope.model_format` and the pressure-source-1 table's
    `supported_profile_limitations[0]` ("Current model 0.5.0 …"). TBD-002's stale
    `accuracy_classification.scope` sentence in the physics-retained-1 table is left as it is.
    If any edit here would need to touch it, that goes back to HELP_HUMAN first.
- **The pressure result schema is renamed** from `schemas/results_pressure_1.v0.3.schema.json`
  to `schemas/results_pressure_2.v0.3.schema.json` (with `git mv`). Its identity `const`s name
  pressure-2. `results.v0.3`, `analysis_run.v0.3` and `stress_neutral_export.v0.3` keep only
  current branches, under the new identities.
- **The result formats stay.** Raw/component stays 0.2 and canonical stays 0.3, as the steer
  states.
- **Tests that used the -2 strings as unknown controls change.** They now use a -3 string. These
  are `result_export/tests/pressure_contract.rs:86`, `tests/test_pressure_contract_readers.py:113`,
  `tests/test_preview_physics_consumer_contract.py:458` and `load_reference_mutations.json:3027`.
  The physics-retained-2 controls (`tests/test_retained_precision_b3.py:950`) and pressure-source-2
  controls (`load_reference_source_mutations.json:190, 1886`) stay as they are, because those
  identities do not exist.

### 1.3 Reviewed inputs

The order of `REVIEWED_INPUTS` (`build_identity.rs`) is unchanged. These entries move:
- index 1, `physics_source_recovery.schema.json` (the source treatment `derived_not_qualified`,
  §4.3);
- index 3, DEF-O;
- index 4, the preview-retained table;
- index 5, now the preview-physics-2 table;
- index 6, DEF-C;
- index 7, DEF-E;
- index 8, the physics-retained table;
- index 9, the pressure-source table;
- index 10, now `results_pressure_2.v0.3.schema.json`;
- index 11, now the pressure-2 table.

All of these require the actual-source D3 (§6). PP's Cargo.lock (index 0) does not move. The
arc maximum and its trigonometric enclosures live in the frame kernel, which PP already depends
on (§2.3).

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
- **The U4b and U5b builders add 0.6.0 requests outside the frozen trees.** The t4_u4b and
  t4_u5b builders author 0.6.0 requests, as their READMEs allow, and the requests are bound to
  each case's `case_sha256` by a pinned hash in the consuming test (t4_u4b) or a committed
  bindings file beside it (t4_u5b). Nothing is added under `validation/references`, so each
  reference tree, its `SHA256SUMS` and its generator's own check are unchanged.
- **The four owner-confirmed 0.5.0 re-authorings** (b7-plan, "Decisions applied") become 0.6.0
  edits of the same files. `tests/test_model_schema.py::check_domain_fixtures` expects 0.6.0.
- **The frozen UI benchmark is unchanged.** Its four 0.2.0 models remained under the then-pending
  owner decision, since ruled (b7-plan, "Owner decision: the frozen UI benchmark", 2026-10-22).

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
| `normal_extremes.sigma_plus`, `.sigma_minus` (`max_abs_normal` = max(\|σ⁺\|, \|σ⁻\|); `*_fibre` derived from the published M rows, §2.3) | `element_local_normal_stress_extreme` | `sigma_plus`, `sigma_minus` | MPa | all | stations × bases | both |
| `sv_transverse_shear_max.value` | `element_local_transverse_shear_stress_maximum` | `tau_v_max` | MPa | all | stations × bases | both |
| `member_maximum` (arc) | `pipe_elastic_normal_stress_maximum_v2`, as on straights | as on straights | as on straights | arcs (joining straights) | `governing_station` plus location evidence | both |
| `headline` | the existing headlines, now covering arcs | — | — | — | — | both |
| `intensified_bending` | `curved_bend_directional_intensified_bending_stress_v1` | `intensified_bending` | Pa | arcs with directional SIFs | stations × bases | both |
| `intensified_bending_maximum` | `curved_bend_directional_intensified_bending_maximum_v1` | `intensified_bending_maximum` | Pa | arcs with directional SIFs | one per basis, with location evidence | both |
| `toroidal_membrane_hoop` (`y_pos`, `z_pos`, `y_neg`) | `pipe_toroidal_membrane_hoop_stress_v1` | `toroidal_intrados_hoop_stress`, `toroidal_crown_hoop_stress`, `toroidal_extrados_hoop_stress` | Pa | pressurized arcs | stations × bases | pressure-2 |
| `radial_surface` | `pipe_toroidal_radial_surface_stress_v1` | `toroidal_inner_radial_stress`, `toroidal_outer_radial_stress` | Pa | pressurized arcs | stations × bases | pressure-2 |

**Vocabulary, not format.** The new component, location and basis tokens join the closed
vocabularies: the tables' `canonical_metadata_vocabulary` and the component enums in
`results.v0.3` and `results_pressure_2`. That covers `sigma_plus`, `sigma_minus`, `tau_v_max`,
the intensified, toroidal and rod components, location `tie_rod`, and the new basis tokens. This
is how U5a added `pe_x` and `s_x`. The row-metadata object keeps its five fields, and the raw 0.2
and canonical 0.3 format versions stay. The arc intensified maximum's location evidence is a
record in the case evidence's `intensified_measures` list, with approximation
`arc_closed_form_directional_intensified` and the §2.3 record's fields. W1 already refuses a
non-empty list (`retained_product.rs:2896`), and arcs never reach W1.

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

**Table signatures (as built, S2/S3).** New signatures follow each table's existing id rule and
are numbered after the table's highest id (`supported-source-NNN` in pressure-2; the preview
table's numbering and its `preview-physics-arc-NNN` arc ids). The tables are edited by one
re-runnable repository script with a check mode, so a later slice renumbers mechanically.

### 2.3 Normal-stress extremes and the arc maximum

**Extremes.** At each station and basis, σ⁺ = N_eff/A_b + hypot(M_y, M_z)/Z_b and
σ⁻ = N_eff/A_b − hypot(M_y, M_z)/Z_b. N_eff is N_w on a pressurized member of pressure-2 and N
otherwise (t4_u4b item 3).
- **Formed from the published stress rows of the same station and basis.** Those rows are
  already j-side section-cut quantities on every route: the stress recovery consumes the j-side
  cuts (`lib.rs` around :5599 and :5661–5722), and arc stress rows carry the same j-side sign
  (arc-frames-and-signs.md §1). So no end-i sign conversion arises, and no section operand is
  needed. The element end-force rows, which are node-on-element, are never an operand. The
  fixed sequence:
  - σ_N, in MPa:
    - on a member without the pressure family, the `element_local_axial_normal_stress` row;
    - on a pressurized pressure-2 member, RN(`pipe_axial_membrane_stress_v2` / 1e6) of the
      same station and basis. That row is N_w/A_s in Pa, and the axial component row is removed
      there (`append_exact_pressure_results`).
  - h = `norm2`(σ_y, σ_z), the shared correctly rounded norm (E15) of the
    `element_local_bending_normal_stress_y` and `_z` rows (MPa).
  - σ⁺ = RN(σ_N + h) and σ⁻ = RN(σ_N − h).

  Every reader can recompute this bit for bit from the publication's own rows.
- **Their fibres are not row metadata.** Row metadata is a closed five-field object under the
  unchanged 0.2/0.3 result formats (`result_export/src/lib.rs` `ResultMetadata`;
  `results.v0.3.schema.yaml` row metadata, `additionalProperties: false`). The fibre is the
  outer-surface point ±r_o(−σ_z, σ_y)/h in the section's local frame, tied when h = 0. It is
  determined by the published bending rows, because σ_y = M_y/Z_b and σ_z = M_z/Z_b share Z_b.
  The basis text states this rule, and the reference test derives the fibre and compares it with
  the oracle's `sigma_plus_fibre` and `sigma_minus_fibre` (§7.1).
- **No certificate.** Neither ordinary route attaches a K-D5 certificate to them. In retained
  and source publications they are not covered (§4).
- **Basis token:** `section_extremes_from_published_stress_rows_v1`, on straights and arcs. The
  arc stress idealization is the U4a arc basis, already disclosed in LIMITATIONS[3] and
  `BEND_LIMITATION`.

**The arc maximum.** This is the maximum over ψ ∈ [0, φ] of
f(ψ) = |N_eff(ψ)|/A_b + hypot(M_y(ψ), M_z(ψ))/Z_b (t4_u4b item 4). ψ is measured from the
authored node i, and the location published is ψ/φ (t4_u4b convention 7). On pressure-2,
N_eff = S + pA_i, and pA_i is constant along the member, so N_eff″ = S″.

**Operands.** The arc maximum and the arc station rows are one binary64 problem.
- **The operands.** The arc maximum's problem is the section function formed from the
  published end_j force rows, taken through the transpose of the end-j frame, and the arc's
  `uniform_intensities` list with its load-state scales, generated self-weight included. It also
  uses R, φ and the basis section. On a pressurized arc it uses the same binary64 pA_i that the
  wall and membrane rows use.
  - On the binary64 route these are exactly the station rows' operands
    (`curved_bend_section_resultants`, `lib.rs` around :11636–11664).
  - On the certified route the station rows come instead from the certified sections, recovered
    through the corrected-displacement functional (`member.sections`, `lib.rs` around :5601;
    `certified_recovery.rs` around :1228). Those certified station values are not replaced or
    reconstructed. The maximum's reconstructed published-end-j problem stands beside them, and
    the station cover (step 5) ties the two together. A certified case with strong cancellation
    along the arc is a control.
- **The load is not rebuilt from K-D5 primitives.** Those exist per local DOF, so summing them
  would multiply-count a load.
- **The uniform-load convention is confirmed first.** The implementation confirms that the arc
  uniform load is a constant global force per unit arc length (t4_u4b inferred convention 1).
  If it is not, that is reported before any code relies on it.
- **This differs from the certified straight route.** That route re-forms its spans at p from the
  corrected displacement (`certified_recovery.rs` around :1778–1786). The arc maximum instead
  encloses the binary64 problem formed from these published operands, on both routes. The
  evidence states this in `enclosure_scope`, with "solution error is separate".

**Resultants.** They follow the closed forms of t4_u4b item 4, carried from end j, in the
tangent frame at ψ: F(ψ) = F_beyond + w R Δψ, M(ψ), and their exact derivatives. The bound terms
are F_max = |F(0)| + |w|Rφ and M_max = |M(0)| + RφF_max, with F the pressure-free statics force (S
on pressurized arcs). |w| is an upward bound of the norm of Σ_k w_k over `uniform_intensities`,
for example Σ_k |w_k| rounded upward. Either end gives a valid curvature bound. The values differ,
and the implementation starts from end j.

**Arithmetic and where it lives.** Every evaluation of f or f′ at a point is an enclosure, in
the frame kernel's existing ball arithmetic (`Balls`/`Wide2`, now `pub(super)` in
`frame_kernel::structural`).
- The arc maximum is therefore DEL-04-01's: a new `frame_kernel::structural::certify_arc_stress_maximum`
  behind a narrow public API. Placing it in `stress_recovery` would need a new crate edge, which
  would move PP's Cargo.lock (reviewed input 0).
- The kernel gains enclosed sine and cosine for an argument ball in [0, π]. They use a Taylor
  polynomial with an explicit Lagrange remainder bound evaluated in `Wide2`, with no argument
  reduction beyond [0, π]. Unit tests compare them with independently precomputed constants,
  committed with their provenance (60-digit decimal values; no new dependency), near 0, near π/2,
  near π and at the reference arcs' φ, and check that each enclosure contains the reference
  value.
- No libm transcendental enters a bound. Each evaluation's lower value is directed downward
  and its upper value upward.

**Method** (t4_u4b item 4):
1. Start from a uniform grid of 4,096 cells in ψ.
2. Upper bound per cell: max(upper endpoint values) + K_up h²/8, with
   K = (2R|w| + F_max)/A_b + (R²|w| + M_max + R F_max)/Z_b formed upward.
3. Lower bound: the largest directed lower endpoint value.
4. Bisect every cell whose upper bound exceeds the lower bound by more than the stopping gap.
   - The gap rule is the existing `elastic_extrema` one, gap ≤ 1e-12 Pa + 1e-12·lower,
     including its absolute floor, so a zero or near-zero arc maximum closes. Readers enforce
     it as they do for straights (`physics_evidence.rs` around :637–645).
   - If the existing limits (131,072 subdivisions, depth 48) are reached first, the maximum is
     refused with a named reason.
5. **Station cover.** The upper bound is raised, if needed, to cover every published station
   value max(|σ⁺|, |σ⁻|) of this arc and basis, converted to Pa and rounded upward. The bound is
   then re-checked against the gap rule; if it no longer closes, the maximum is unavailable
   through the existing path.
   - Because the station rows and the maximum share their operands, the cover is a safety net
     against rounding.
   - The cover only raises the bound; a station value below the lower bound is not refused,
     because no allowance both admits every genuine binary64 station row and bounds the
     arc (load-cancellation and short-arc counterexamples). Operand mismatches are caught by
     tests. Passing N_w with pA_i double-counts pressure: it raises the maximum above every
     station row, or, where N_w changes sign, leaves the cover unclosed and the maximum
     refused. A kernel test asserts both outcomes, and a product test binds each arc maximum
     to its own station rows.
   - Each uniform load enters the enclosure separately. Under extreme cancellation (S2's
     G = 1e80 control) the bound cannot close and the maximum is withheld through the existing
     unavailable path (`PREVIEW_STRESS_MAXIMUM_UNAVAILABLE` on preview); the station rows are
     still published. Summing like-directed loads first would narrow this; it is not adopted.
   - This differs from the straight `maximum_cover` (`certified_publication.rs` around
     :512–535), which refuses on violation and checks each stress component.
   - Straight maxima, and their reader checks, are unchanged. A straight σ± and the straight
     maximum come from different arithmetic, and the published maximum is a midpoint, so no
     reader relation between them is imposed.
6. **Location.**
   - *End maxima.* If the governing evaluation (the largest directed lower value) is an
     endpoint, and the derivative enclosure there points out of the member (f′(0) < 0 at ψ = 0,
     f′(φ) > 0 at ψ = φ), the location is exactly 0 or 1.
   - *Interior maxima.*
     1. **Candidate cells.** These are the surviving cells (upper ≥ global lower) whose own
        directed lower value at an endpoint is within the gap of the global lower. That is
        evidence the cell attains the maximum within the gap; an upper bound alone does not
        qualify a cell.
     2. **A bracket.** Within one candidate cell, or one contiguous run of candidate cells, the
        smallest-ψ pair of trial points whose derivative enclosures are strictly positive at the
        left and strictly negative at the right. A bracket never spans a non-candidate gap.
     3. **Bisection.** Evaluate the f′ enclosure at the midpoint. A strictly positive enclosure
        moves the left end, and a strictly negative one moves the right end. An enclosure that
        contains zero at the midpoint is an unresolved sign: it stops the refinement there. The
        bisection also stops when the bracket is at most 2^-40·φ wide.
     4. **Publication.** The midpoint of the final bracket, or the unresolved trial point.
   - *No bracket and no end rule.* The location is the governing evaluation point.
   - *Ties within the gap.* Among candidates (step 1), the smallest ψ governs.
   - *Tests.* Two close competing peaks, an unresolved-sign case and every frozen location band.

   The value enclosure never depends on the location's success.
7. **Publication.** The same way the straight maximum publishes: the binary64 midpoint of
   [value_lower, upper_bound].

**The evidence record.** Each arc's entry in `pipe_stress_extrema` uses exactly the straight
record's fourteen keys (`physics_evidence.rs` around :592–650). That keeps one closed shape.
- `pipe_id`, `section_basis` (pressure-2) and `result_id`, as on straights.
- `station_fraction` and `local_fraction`, both ψ*/φ (exactly 0 or 1 at an end).
- `span_index` = 0. An arc is one span.
- `value_lower_pa`, `value_upper_pa`, `global_upper_bound_pa`, `certified_gap_pa` and
  `subdivisions`.
- `approximation` = `"arc_closed_form_section_statics"`.
- `coefficient_basis` = `"arc_end_j_force_and_uniform_intensities_binary64_enclosures"`.
- `enclosure_scope` = `"published_binary64_end_j_force_and_uniform_intensity_operand_enclosures; solution error is separate"`.

The approximation token selects the arc branch in each closed consumer. Each consumer accepts
it only for a member whose model pipe is a realized arc, and refuses it otherwise:
- `physics_evidence.rs` and `preview_physics_evidence.rs`;
- `retained_product.rs` (around :3031) and `source_receipt/composite.rs` (around :668). Arcs
  never reach these (D7), so both refuse the arc token by name;
- `schemas/stress_neutral_export.v0.3.schema.json` (around :2902–2945), `results.v0.3` and
  `results_pressure_2`;
- the Python verifier.

**Where it is called, withheld and removed.**
- PP calls it in the member loop, where `exact_member_maximum_policy` withholds arcs today.
- When the cut or the load is unavailable, the existing unavailable path and warning stay
  (`EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE`), with the specific reason.
- **The preview arc marker.** Removing arcs from `outside_domain_pipe_ids` removes the place where
  preview evidence identifies them. Preview readers identify an arc member by the arc basis
  token `nominal_straight_beam_formula_on_arc_resultants`, which every arc component stress row
  carries (`preview_physics.rs` `ARC_STRESS_KINDS` relabel). The arc maximum token, the arc τ
  basis token and the directional rows are accepted only on members so marked, and refused
  elsewhere.
- Removed: `exact_member_maximum_policy`'s arc withholding; `withheld_on_arcs`,
  `ARC_WITHHELD_REASON` and the checks that require them; and the preview
  `outside_domain_pipe_ids` placement of arcs.

**Headlines.** The existing headline rules apply, with arcs no longer outside the domain. On
pressure-2 there is one headline per basis, nulled when any member lacks its basis maximum. On
preview-physics-2 the headline is set when coverage is complete. The interior location band of
t4_u4b is a comparison rule for the test, not a publication.

### 2.4 Saint-Venant transverse shear

τ_v,max = |V| ((3+2ν) r_o² + (1+2ν) r_i,b²) / (4(1+ν) I_b) at each station and basis, with V =
(V_y, V_z) in the section frame (t4_u4b §2).
- It is one formula on straights and arcs; each basis uses its own annulus. The basis token is
  `local_straight_annulus_on_arc` on arcs and `saint_venant_annulus_flexure_v1` on straights.
- Torsional shear stays its own row. No location metadata is published: the two fibres are
  fixed by V (inner surface, normal to V), and the basis text states this.
- **ν, by route.**
  - *pressure-2* uses the member's authored and validated ν. The exact route requires
    `homogeneous_isotropic_E_nu_v1` and checks −1 < ν < 1/2 (`pressure_material.rs`), and the
    named-point pair supplies ν where a basis is selected.
  - *preview-physics-2* uses ν = RN(RN(E/RN(2·G)) − 1), from the E and G the solve itself uses
    for that member and case:
    - for a selected named point, the point's moduli;
    - for a base case, the base moduli;
    - for a case with `modulus_basis_temperature`, the linearly interpolated moduli the solve
      forms (`lib.rs` around :10166–10310).
    - An authored `poisson_ratio` is not consulted on this route. The preview route neither
      validates it nor pairs it with point moduli: the named-point resolver keeps the base ν
      (`lib.rs` around :10386).
    - The derivation assumes isotropy, which preview admission does not establish
      (`constitutive_basis` is optional there). So the assumption is disclosed in LIMITATIONS[2]
      and in the basis text ("ν = E/(2G) − 1, assuming an isotropic material").
    - The derivation does not invent a value the solve does not already imply.
  - *Domain.* The SV field is defined for −1 < ν < 1/2. On preview, a member whose derived ν lies
    outside that range has no τ rows, and the case carries the warning diagnostic
    `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`, naming the member and its ν. The coverage rule
    exempts exactly the members that diagnostic names.
    - The diagnostic travels as other ordinary warnings do.
    - If W1 or the source route do not carry ordinary warnings into their publications, they
      decline such a case by name. The implementation confirms which applies, and reports any
      selection that changes.
- **Formation** (fixed sequence, MPa):
  - c_b = RN(num/den), with
    num = RN(RN(RN(3+2ν)·RN(r_o·r_o)) + RN(RN(1+2ν)·RN(r_i,b·r_i,b))) and
    den = RN(RN(4·RN(1+ν))·I_b), r_o = D/2, and r_i,b and I_b the basis section's.
    This is the form on pressure-2, and on preview-physics-2 while q = RN(E/RN(2·G)) ≥ 1/2
    (G ≤ E; for binary64 G > E, q < 1/2). There ν is the rounded ν = RN(q − 1), and
    the operations ν = q − 1, 1+ν = q and 1+2ν = 2q − 1 are exact.
  - On preview-physics-2 with q < 1/2, write ν* = E/(2G) − 1 in exact arithmetic, so
    1/(1+ν*) = 2G/E and ν*/(1+ν*) = 1 − 2G/E, with q* = E/(2G). The rounded ν would recover
    1+ν* only to within u + 2^-54/q* relative (u = 2^-53). So c_b is formed without ν from the rounded x = RN(RN(2·G)/E), which
    approximates 1/(1+ν*) within one rounding (frame kernel `shear::moduli_ratio_terms`, which
    also gives t = RN(1 − x), approximating ν*/(1+ν*); these are the pressure-free Timoshenko
    factor's terms): c_b = RN(num/den), with
    num = RN(RN(2·RN(RN(r_o·r_o) + RN(r_i,b·r_i,b))) + RN(x·RN(RN(r_o − r_i,b)·RN(r_o + r_i,b))))
    and den = RN(4·I_b). In exact arithmetic (3+2ν*)/(1+ν*) = 2 + 2G/E and
    (1+2ν*)/(1+ν*) = 2 − 2G/E, so this is the same Saint-Venant value, and every term is
    nonnegative.
  - On q < 1/2 the rounded ν = RN(q − 1) serves admission only (the domain rule above), so the admitted set
    does not move. It is not 1/x − 1: just below q = 1/2 the rounded ν is still −1/2, and at
    E/(2G) = 1/(3·2^52) it is −1 + 2^-53, where 1/(1+ν) = 2^53 but x = 3·2^52. To first order in
    u, for admitted evaluations without overflow or underflow and taking the binary64 moduli,
    section and force operands as exact, |c_b/c_b* − 1| ≤ 8.5u for every E/(2G) in (0, 3/2)
    (7u on the x form), with c_b* the exact value at ν*, and τ_v,max is within 11.5u relative of
    |V|·c_b*/1e6. Section-property and solve errors are separate.
  - t = `norm2`(V_y, V_z) (N). At the three interior stations it uses the published j-side
    section rows. At `end_i` and `end_j` there are no section rows, so it uses the published
    end-force shear rows of that end. The norm ignores their node-on-element sign, and in exact
    statics they equal the section action there in magnitude. At end_j this is the stiffness end
    action, not the section function evaluated at fraction 1. A test binds the end-station τ to
    those rows.
  - τ = RN(RN(t·c_b)/1e6).
- **No new evidence list.** The closed case-evidence shapes on W1-L capture, W1-E capture, the
  failed-ordinary evidence, composite finalization and the source schema refuse any new key
  (`retained_product.rs` around :2729–2730, :2858–2867 and :4558–4560; `composite.rs` around
  :500–511; `physics_source_recovery.schema.json`, `additionalProperties: false`). So c_b is not
  published. A reader that holds the captured model recomputes c_b from it, and then τ bit for
  bit from the rows, by the route's ν rule above. These readers hold the model:
  - the exact readers, which hold it already (`section_basis.rs` around :378–411);
  - the preview readers when they are given the model (G2);
  - the retained reader at its captured-invocation stage;
  - the source reader with its invocation.

  A reader without the model, such as G7's model-free base read or a preview read without a
  model, checks custody, coverage, non-negativity and the unit only. That is the existing reach
  of model checks (b7-plan G2).
  - Interpolated preview moduli are a second exception. No reader reproduces the temperature
    interpolation, and adding two independent interpolation implementations is out of scope.
    So for a case with `modulus_basis_temperature`, the model-holding readers apply the same
    model-free checks to τ. W1 refuses such a case at D1 already.
  - Tests include an interpolated preview control.

### 2.5 Toroidal hoop and radial values (pressure-2)

On each pressurized arc and basis b:
- σ_h(β) = (p r_i,b/t_b)(2R − r_i,b cos β)/(2R − (r_i,b + r_o) cos β) at β = 0 (intrados,
  `y_pos`), 90° (crown, `z_pos`) and 180° (extrados, `y_neg`);
- the radial values −p at r_i,b and 0 at r_o.

They are published at the five stations, as the straight Lamé rows are (the values are
station-independent; t4_u4b inferred convention 2).
- **Formation.** In binary64, as the straight Lamé rows are: an `input_derived` quantity with no
  solve bound. The readers recompute these rows bit-equal, by the normative fixed sequence that
  `stress_recovery` states, from p and the basis bore, wall and r_o in the evidence, and R. The
  existing Lamé 64ε rule is not enough here: near R = r_o the intrados denominator amplifies
  rounding by about 2R/t (80ε at D = 1.2192 m, t = 6.35 mm).
- **R in the evidence.** Neither the arc region evidence (`arc_evidence.rs` around :35–54) nor
  `pipe_sections` carries R today. Each realized arc's member record in the arc region evidence
  therefore gains `bend_radius_m` (schemas, readers, verifier; priced). Arcs never reach W1 or
  the source route, so their closed shapes are untouched.
- **The torus domain is guarded here.** Bend realization checks only R > half-chord (`lib.rs`
  around :8108–8119), so it does not guarantee the definition's domain R > r_o. The hoop and
  radial rows of a pressurized arc are published only when R > r_o, decided by exact comparison
  of the binary64 R and r_o = D/2. Otherwise they are unavailable on that arc, with the named
  warning `ARC_TOROIDAL_HOOP_DOMAIN` ("bend radius R does not exceed the outer radius; the toroidal
  membrane hoop is undefined"). The readers require the rows to be present exactly when R > r_o.
  Tests cover R below, at and one ulp above r_o, and at R = (r_i + r_o)/2, where the intrados
  denominator vanishes. The solve itself is unaffected.
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

**Rows.** intensified = hypot(i_ip M_z, i_op M_y)/Z_b at each station and basis.
- **Formation.** It is formed from the same station's published bending stress rows, by a fixed
  sequence: intensified = RN(1e6·`norm2`(RN(i_ip·σ_z), RN(i_op·σ_y))), in Pa. The i values are
  the captured binary64 factors.
- **The along-arc maximum** uses §2.3's method and record with the intensified measure
  (K scaled by max(i_ip, i_op) on the moment terms).
- **The evidence records.** The marker records keep their key set and bytes
  (`INTENSIFIED_KEYS`, `preview_physics_evidence.rs` around :33–44). The new records are told
  apart from them by their own closed key sets, not by a `kind` field:
  - *a directional station record:* `pipe_id`, `result_id`, `section_basis` (pressure-2),
    `location`, `i_in_plane` and `i_out_of_plane`;
  - *the maximum record:* `i_in_plane`, `i_out_of_plane` and §2.3's record keys, with
    approximation `"arc_closed_form_directional_intensified"`.

  The readers' row-to-record bijection (Rust and verifier) accepts the three key sets.
- **On pressure-2.** Exact case evidence gains `intensified_measures` only when it is non-empty,
  that is, only for a case with a directional arc. An arc never reaches W1-E capture or composite
  finalization, so their closed exact-case key sets never see the key. It is added to
  `results_pressure_2`, the stress-neutral schema, the Rust reader and the verifier, and priced.
- **Basis token.** The directional rows and their maximum carry the basis token
  `user_directional_sif_times_tangent_frame_bending_stress_v1` (as built, S2). It joins the closed
  vocabularies with the other new tokens.
- **Never combined** (§2.7). No factor multiplies N/A, the hoop, torsion or the normal-stress
  maximum. k never multiplies stress.
- **Both routes.** Pressure-2 arcs publish them on both bases; preview-physics-2 publishes them
  on its single section.
- **The preview limitation.** LIMITATIONS[4] keeps its marker sentence and gains the arc
  sentence (§2.8).

### 2.7 Algebra-class marks

Every row signature in the pressure-2, preview-physics-2 and inheriting tables gains
`combination_algebra`, one of three values:
- **`signed_linear`**: displacements, rotations, section resultants, signed stress components,
  torsional shear, pressure-family forces and stresses, the hoop and radial rows, support
  components, connector generalized actions and tie-rod forces. A combination combines these
  directly, as matching signed quantities.
- **`recomputed_from_combined`**: displacement magnitudes and support force and moment
  magnitudes. A linear combination, or a labelled difference, forms them from its own combined
  components, as today (`preview_physics.rs` around :892 and :958). They are never summed. A
  range envelope keeps its existing per-row mode selection: it envelopes each magnitude
  independently of the component envelopes (`preview_physics.rs` around :892 and :963; the table's
  range-envelope rule). The reader already exempts range envelopes from the component-norm check
  (`preview_physics_evidence.rs` around :709).
- **`never_combined`**: σ⁺/σ⁻, the transverse-shear maximum, every maximum and headline-bearing
  row, every intensified row, and review and basis rows. A combination publishes none of them,
  except its own `combination_modulus_basis_record`: one record per combination naming the modulus
  basis it used. That record combines no operand values.

This is a classification (STRESS_REFERENCE §5), not a new combination rule.
- The ordinary combination engine keeps its behaviour: maxima and intensified rows are not
  combined, with the existing info diagnostics. It extends that behaviour to the new
  `never_combined` kinds. `COMBINATION_STRESS_MAXIMUM_UNAVAILABLE`'s text becomes "stress maxima
  and section stress extremes are not combined; combine the signed stress component rows
  instead".
- A table test checks the committed corpora:
  - every combination row's kind is `signed_linear` or `recomputed_from_combined`;
  - no `never_combined` kind appears in a combination;
  - every `recomputed_from_combined` row of a linear combination or difference equals the norm
    of its own combination's components, not a combination of operand magnitudes;
  - a range envelope's magnitude rows keep their per-row selection, with the existing
    exemption.
- REQ-001's combination row count, 7n + 50m + 8g, is unchanged. Magnitudes are already counted
  in it, and the new kinds are not combination rows.

### 2.8 Preview-physics-2 limitations

- **LIMITATIONS[2]** becomes:

  > On every member the section normal-stress extremes are N/A ± hypot(My,Mz)/Z at each station and the transverse shear maximum is the straight-annulus Saint-Venant value, with ν = E/(2G) − 1 assuming an isotropic material. On straight members the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No equivalent stress or code stress.

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
  - `JOINT_TIE_ROD_ZERO_LENGTH`: the represented length is degenerate. The represented difference
    is (x_j − x_i) + (o_j − o_i), formed componentwise as the connector forms r. The length counts
    as degenerate when l ≤ 32·u·(‖x_j − x_i‖∞ + ‖o_i‖∞ + ‖o_j‖∞), with u = 2^-53, decided exactly or
    by directed rounding. A length at roundoff level would otherwise give a noise direction. This
    is a representation guard, like the connector's `ALIGNMENT_GUARD`, not a physical tolerance.
    Physical near-perpendicular directions stay admitted (§3.4).
  - `JOINT_TIE_ROD_INPUT_INVALID` (range): a rod whose C coefficients or reference load cannot be
    represented. This covers an overflowing offset cross product, and |k·e_ref| below the
    exact-split floor. It is refused at construction, never as a panic.
- **Model findings come first.** These are connector model findings (`model_findings`), so §1
  precedence applies and a refused listed connector carries only its code.
- **U5a's hardware condition is lifted for rods.** U5a admission condition 2 (joint-pressure-thrust.md
  §1) and its field-specification line "The connector's `hardware` is `untied`" become: hardware
  is `untied` or `explicit_axial_rods_v1` with valid rods. Every other admission condition of a
  listed connector stays, zero attachment offsets included. A connector's attachment offsets are
  separate from its rods' offsets, which are free. Thirty-four of the t4_u5b cases are
  pressurized listed connectors with rods.
- **t4_u5a RV-11 is superseded.** RV-11's frozen patch sets
  `{"kind": "explicit_axial_rods_v1", "rods": []}` on a pressurized joint and expects
  `JOINT_HARDWARE_LAW_UNSUPPORTED`; the generator describes it as "explicit axial rods with a
  pressurized joint (until U5b)". After U5b the kind is defined, and the empty list takes
  `JOINT_TIE_ROD_INPUT_INVALID`.
  - The implementing change records the supersession as a text-only note in
    `validation/references/t4_u5a/README.md`, as T4-U5a recorded t4_i12's supersession
    (joint-pressure-thrust.md §7). Only that README line of `SHA256SUMS` is regenerated.
  - No request byte, oracle value or generator byte changes.
  - The consuming test runs RV-11 and asserts the superseded code by name, beside the other
    sixteen variants, whose frozen expectations it keeps.
  - The HELP_HUMAN confirmation is recorded in Rulings.

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
    coordinates, Q, the authored local offsets and k_r. It uses plain 128-bit arithmetic, as frames
    and connectors are formed, because the intended stiffness has no radius slot. ε_E measures
    the difference between that p re-formation and the binary64 element entry by entry, including
    the rounding of o = fl(Q·offset). Ball arithmetic forms n and l for the reference load and the
    certified recovery.
  - As built, the rods ride on their connector (`ObjectiveConnector::with_tie_rods`). Each remains
    its own element, with its own Ke, reference load, K-D5 block, family and certified recovery,
    walked after all connectors and before the arcs in every assembly, census, ε_E and reaction
    pass.
  - A new family `LoadFormula::TieRodReference { connector, rod, … }` re-forms +Cᵀk_r e_ref at p.
    Its summand identity is `{connector}:{rod}:tie_rod_reference`. "One ε": omitted only when
    e_ref is exactly 0.
  - Certified rod recovery `certify_tie_rod_recovery` gives T_r as a certified row of the
    corrected displacement. Its radius comes from the displacement ball and from C's ball: k_r is
    an exact input.
  - W2 forms the same element at 2^b, with classification and the complete manifest retained.
- **Supplier need.** The element, its family and its certified recovery are DEL-04-01's (§8).
  DEL-04-01 Design `kd5-member-relative-bound.md` §1 carries the family row.
- **Pressure.** Rods carry no pressure load. The ledger (representation B), its caps, the joint
  pair and the Poisson pairs are unchanged (t4_u5b "Pressure RHS": rods never change it).

### 3.3 Publication

- **`connector_tie_rod_force_v1`.** One row per rod: component `t_r`, unit N, location
  `tie_rod`, `coordinate_system` `element_local` (along the rod's own axis) and basis token
  `recovered_from_tie_rod_extension_v1`. The rod is identified by its row id and its
  `tie_rods` evidence entry, not by metadata. Row id
  `result:connector:{suffix}:tie-rod-force:tie_rod:{encoded rod id}:t_r`. Tension is positive,
  T_r = k_r(e − e_ref). It is certified on the certified route and binary64 otherwise.
- **`s_x`** (`connector_effective_axial_force_v1`) becomes S_joint = g_x + Σ T_r (n_r·Q_x) − Pe
  (ruling 1). On the certified route it is formed at p from the certified g_x and T_r and each
  rod's published binary64 `axial_share` taken as an exact operand, then rounded once, with the
  radii summed. Both routes thus use the share the evidence publishes. The share formed at p
  differs from the published one by about 1e-17 absolute, independent of the share's size, so
  a share formed at p would leave near-perpendicular rods outside the reader rule (S3 dual
  review). It is published:
  - for every constant-effective-area connector, as today (Pe = 0 when no region lists it,
    ruling 2);
  - and for every connector with rods, whatever its pressure model. For an unpressurized
    connector with rods, Pe = 0 and s_x is the face axial force. That decides t4_u5b's open
    point: the face-equilibrium identity is published wherever rods carry part of it.
- **`pe_x`** is published only for a constant-effective-area connector, as today. An
  unpressurized connector has no Ae.
- **The readers' s_x binding** is s_x = gt_x + Σ t_r·axial_share − Pe, with Pe the published
  `pe_x` where it exists and 0 on an unpressurized connector.
- **Unchanged rows.** An untied unpressurized connector's rows and evidence are unchanged, byte
  for byte.
- **The connector evidence record.** `hardware` becomes the authored kind. With rods, `tie_rods`
  lists, per rod: `id`, `law`, `offset_global_m` [i, j], `stiffness_n_per_m`,
  `reference_extension_m`, `length_m`, `direction_global`, `axial_share` (n·Q_x) and
  `source_reference`.

### 3.4 The open points this design decides

- **A rod nearly perpendicular to the joint axis is admitted, with no refusal.** Ruling 1 lets
  this design refuse it by name only if that is justified, and nothing justifies it:
  - the rod element's assembly and certificate do not depend on n·Q_x. As built, the kernel's
    assembly-absorption refusal applies per element: it fires only when an element has a
    positive diagonal contribution at a free DOF and every such contribution is absorbed by
    rounding (prescribed DOFs never enter the solved system). A contribution that opens a zero
    diagonal is represented, not absorbed. Elements are grouped from the
    adapter's element walk; callers without groups keep the per-entry rule. A rank-1 rod with tiny
    entries along some global axis (near-perpendicular, or a parallel rod with coordinate
    noise) is therefore not refused, while a series-absorbed member keeps its refusal (F1b);
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
`element_local_transverse_shear_stress_maximum`. The arc-only kinds and tie-rod rows cannot reach
W1: realized arcs and connectors stay declined at capture (`retained_product.rs` `capture_case_structure`).

**Classed `not_covered`.** The new kinds join REQ-001's "every other row kind", never
`RowClass::Unpublishable`. One classification applies at every layer:
- the kernel's row class (`k::RowClass`) and `retained_wire`'s translation (`retained_wire.rs`
  around :1276–1285);
- the Rust reader's `row_kind` and G5c, native and wasm;
- the Python verifier;
- the inheriting tables' `accuracy_classification`, where it enumerates kinds.

**The not-covered passthrough.** Classification alone cannot carry these rows. W1 capture
refuses any stress kind outside its closed list (`retained_product.rs` around :2615–2626,
`uncovered actual row`). The kernel's `ProductRecipe` (`final_case.rs` around :108–123) has
quantity recipes only. So the epoch adds one explicit mechanism, which is a recipe-free
passthrough, not a numerical recovery recipe:
- **Capture.** The capture classifies the two kinds as `Derived` rows and records each one's
  identity, unit, metadata and basis twin against the captured member, station and basis, in
  the order of the ordinary row manifest.
- **Projection and final proof.** These rows enter no lane, functional, certificate, scale or
  bound. The kernel's final case gains `ProductRecipe::NotCovered { row }`, beside `NonQuantity`
  and `ModulusBasisRecord` (`final_case.rs` around :108–123). It carries the row's position only.
  It contributes nothing to any verified class's body scale, maximum or coverage set, and the
  existing `final_case` checks exclude it from every verified count. Its verdict takes the
  class-`None` path, which the wire lists as `not_covered` (`retained_wire.rs` around :1276–1285),
  not the path that omits `NonQuantity` and record recipes.
- **Value.** In the successor the row's value is formed after projection from the successor's
  own published rows, by the same binary64 formula as on the ordinary route
  (§§2.3–2.4; the owner's Q2 decision, below). Nothing is recovered, and nothing is bound to a
  certificate.
- **Binding (as built, S4).** Each derived row binds its `NotCovered` recipe at its own
  position, so the passthrough does not depend on row order. Derived rows are not paired as
  basis twins, are not hull-projected and do not enter the G5b overlay; the readers exclude
  them by kind, and a relabelled row still meets the signature and metadata checks. Route L
  forms c_b from the model section (d = D − 2t, I = π(D⁴ − d⁴)/64, r_o = D/2), which reproduces
  the ordinary route's section bit for bit. Route E forms it from the captured evidence.
- **Reading (as built, S4).** On route L the retained readers run the preview model checks on
  the projection with the captured invocation's model, at G7, so τ is recomputed inside the
  shipped retained entry. Route E's G7 base read already recomputes it from the evidence.
- **Receipt.** The receipt lists the rows under `not_covered` exactly as other `not_covered`
  rows, with no bound. The reader checks their custody (identity, unit, metadata, member and
  basis coverage) and, under (ii), recomputes them from the successor's published operands
  under the base reader's one rule.
- **Definitions.** DEF-O, DEF-E and DEF-C state the passthrough in the recipe enumeration they
  already have: "derived section extremes and transverse-shear maxima: not covered, formed from
  the successor's published rows". This moves their meaning hashes (§1.2).
- **Priced owners.** The capture, final-case and wire code are DEL-04-08's method and DEL-04-01's
  kernel (a supplier need, §8). Their new owners, loops and texts are priced in the actual-source
  D3 (§6).

**The values (Q2, owner, 2026-10-17: "Form from own resultants").** In every publication the
derived kinds are formed in binary64 from that publication's own published stress and
section-action rows (§§2.3–2.4), and classed `not_covered`. Publications here means ordinary,
W1-L and W1-E successors, failed-ordinary successors and pressure-source-1. REQ-001's value
clause is edited for these two derived kinds (§4.4). This is necessary because G7 runs the base
reader on the projected successor statement (`retained_precision.rs` around :6396), and a
failed-ordinary successor has no ordinary rows (`retained_product.rs` around :4452).

**The failed-ordinary path (N06 and CX-F).** `failed_manifest` gains the derived rows in the
row order of preview-physics-2, as placeholders. Its fixed per-member count `FAILED_MEMBER_ROWS`
goes from 51 to 66 (15 per member: σ⁺, σ⁻ and τ at five stations; a member whose preview ν is
out of domain omits τ and carries `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`, as the ordinary
path does; `retained_product.rs` around
:275–278 and :4500), and the change is priced. The freeze replaces them, after projection, with
values formed from the projected resultants. They are `not_covered` there too. The maintained
N06 and CX-F successors are regenerated and stay selected, bound and eligible
(`fixtures/results/retained_precision_failed_ordinary_n06.json` and the CX-F controls).

**Unchanged.** The intensified-measures refusal (`retained_product.rs:2896`), G5b, every gate,
limit, tolerance, and the verified-kind list and recipes.

**The maintained W1 checks keep their capability.** The B4/D1 census
(`b4_w1_covers_every_formerly_selected_source_block_case`), the group-3 census, the member-load
censuses and the failed-ordinary controls must publish the same selections at the same or
better standing. The new rows only add `not_covered` entries. A selection lost to the new kinds
is a defect, not a declared change. A test builds a case whose ordinary and retained resultants
deliberately diverge and checks that the successor's derived rows follow the retained
resultants.

### 4.2 Combinations

The W1 combination route (DEF-C) does not publish `never_combined` kinds. That is already its
rule for maxima and intensified rows (REQ-001: "Combination maxima, intensified rows … are not
covered"). The new extremes and shear maxima follow the ordinary engine's treatment (§2.7) and are
absent from combination owners. The REQ-001 count of 7n + 50m + 8g rows per combination stays
exact.

### 4.3 pressure-source-1

A selected retained-source publication carries the new straight-member kinds as rows outside the
receipt's `projection.quantity` and outside the REQ-002 checks. They are formed from that
publication's published rows (§4.1, Q2).
- `pressure-source-1`'s table gains their signatures.
- The source family is unchanged (D7, D1), so no arc or connector row reaches it.

**A treatment of their own.** The receipt cannot carry them as it is.
- **Why a new treatment.** `rows::bind_for` gives every row that is not projected, derived or an
  observation the treatment `inspection_only` (`source_receipt/rows.rs` around :572–591). The
  composite-exact path refuses any such physical row ("unqualified composite physical row",
  `composite.rs` around :924–931), and the reader marks one unusable (`source_blocks.rs` around
  :943–950). So without a change, every source case that publishes these rows would be refused,
  which is a capability loss.
- **Why not `checked_derived`.** Under REQ-002 that treatment means a checked derived quantity,
  as for the support norms. Using it would add a guarantee to REQ-002's published list, which is
  an owner-approved commitment.
- **What is added.** The receipt gains one treatment, `derived_not_qualified`, which carries no
  REQ-002 guarantee:
  - it names the formula recipe `section_extremes_from_published_stress_rows_v1` or
    `transverse_shear_from_published_shear_rows_v1`, with the station row ids as inputs;
  - `bind_for` assigns it to exactly the two kinds;
  - composite finalization admits it for those two kinds only, and still refuses every other
    `inspection_only` physical row;
  - the Rust reader and the verifier admit it for those kinds only. They leave `usable` unchanged
    by it: such a row neither qualifies the case nor disqualifies it. They recompute it (§5.1).
  - As built (S4): the composite readers run the straight-member U4b checks on every published
    case, exact-recovered and ordinary-method alike, recomputing σ± and τ and checking station ×
    basis coverage. Derived rows on an ordinary-method case keep that case's
    `ordinary_physics_checked` treatment.
- **Its exact shape.** `projection_id` is null. `recipe_id` is one of the two formula recipes.
  `input_result_ids` are exactly the station rows the formula reads.
  The two recipes are not `derived_recipes` and carry no REQ-002 check.
- **Reviewed input 1 moves.** In `physics_source_recovery.schema.json` the treatment vocabulary
  gains the token, and the closed `recipe_id` enum (around :470) gains the two recipe names (§1.3).
- **Standing.** `reader_verdict.rs` (around :343) recognizes only `qualified_projection`,
  `checked_derived` and `ordinary_physics_checked`. Every other physical row raises
  `SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY` (:350), and standing derives eligibility from those
  findings (:402, :416). The verifier duplicates that allowlist (`tests/verifiers/reader_verdict.py`
  around :367). Both admit `derived_not_qualified` neutrally, for the two kinds only: it raises no
  finding and grants no eligibility. T1's existing exclusions are retained.
- **A disclosure.** The pressure-source-1 table's limitations promise complete case, row and
  evidence coverage, so they gain one priced sentence: "Section normal-stress extremes and
  transverse-shear maxima are published beside the qualified rows, formed from them, and are not
  qualified." This sentence is outside TBD-002.
- **Tests.** Schema-valid genuine receipts, and wrong-recipe, wrong-treatment, wrong-kind and
  wrong-input mutations, each refused by name.
- **The receipt and source method are otherwise unchanged.** Nothing projected, derived or
  checked changes, and REQ-002 is untouched.

**Work accounting.** The source-work counters change under the complete D5/MJ9 law (b7-plan,
"D5/MJ9's accounting clarification"). For each selected case, `cases[k].work.charged` changes by
24ΔE + ΔF_k + ΔQ_k, and `invocation_work.publication_charged` changes by 24ΔE + ΔP. The terms:
- ΔE: the encoded-request delta. The 0.6.0 token has the same length as 0.5.0, so only actual
  request changes count.
- ΔF_k: the new rows' identity, kind, entity, unit and metadata lengths inside the selected-case
  finalization, which `finalization_size` charges once, as 24E + F_k.
- ΔQ_k: the other local charges. It includes the composite-exact wrapper's own terms,
  `1024 + 64·N_actual_rows + N_support_actions·RECIPE_WORK`, but not the finalization it also
  calls (`composite.rs` around :744; b7-plan "outside finalization_size"). The wrapper term alone
  adds 64 per new row, which is 1,920 per exact member per selected case at 30 rows. The new
  treatment adds no recipe work: it is not a `derived_recipes` entry, and the formula is checked
  by the reader, not reserved by the producer. Its per-recipe charge is therefore zero, and the
  test asserts that.
- ΔP: the publication's `32N_rows` and per-case `256N_rows` terms.

**Its checks.**
- *An independent test* derives every term from the actual inputs, as the MJ9 test does. It
  checks finalization, the wrapper-only terms, the other local charges and publication
  separately, then reconciles their sum with the counters exactly.
- *Capability.* Every currently selected source case still selects, under the unchanged
  8,000,000 limit. The largest committed per-case charge today is 2,855,814
  (`j7_rf_census_expected.json`), and the test computes each case's new charge rather than
  assuming it fits.
- *The limit.* An input at the limit still refuses with its named budget refusal.
- *Byte checks.* MJ6's and MJ9's byte checks re-pin only to the actual producer bytes that these
  deltas explain.

### 4.4 Commitment sentences edited in this epoch

**The 0.6.0 status sentences** (owner, 2026-10-17: "Confirm token edits"). Only the version token
changes, 0.5.0 → 0.6.0. Requirements and criteria are unchanged.
- DEL-02-05 ScopeOfWork CLM-003 and its version-check and runtime-authoring sentences (lines 33,
  35, 37 and 175).
- DEL-04-08 TBD-002's status text ("DEF-E and the code admit current model 0.5.0 …"). The
  physics-retained-1 table's TBD-002 sentence is not touched.
- DEL-04-08 REQ-001 "Route E domain" ("current model 0.5.0 …") and REQ-002 "Eligibility"
  ("current model 0.5.0 …"). The owner, 2026-10-17: "Confirm both (Recommended)". Only the version
  token changes.
- DEL-04-08 REQ-003 (c), REQ-006 and REQ-007 (c): `pressure-1` → `pressure-2`,
  `PRESSURE-1-OUTPUT-NOT-YET-AVAILABLE` → `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`, and "replace" →
  "replaced" in REQ-007 (c). The owner, 2026-10-17: "Confirm token edits (Recommended)". "Merge
  with the gap" stands.
- DEL-04-08 AC-002's second bullet now reads "the RF census classification, counts, first
  causes, selections, charged work and frozen-row verdicts are unchanged; its envelope, results
  and summary digests change only as a stated product-text, identity or published-row change
  requires". The owner, 2026-10-19: "I approve this one". The published rows and the corrected
  displacement move results and summary digests as well as envelopes. The epoch lists each J7
  digest that moves with the change that requires it, and shows the classification, counts, first
  causes, selections, charged work and frozen-row verdicts unchanged (S5).
- Status and evidence prose naming the old tokens is reconciled under the same token-only rule
  (HELP_HUMAN, 2026-10-17): DEL-03-01, DEL-05-02, DEL-05-03, DEL-07-05, DEL-08-06, DEL-14-02 and
  DEL-17-06.

**REQ-001's value clause** (owner, 2026-10-17: "Form from own resultants (Recommended)"). Only that
clause changes, and only for derived kinds formed from published operands.
- The current text:
  > These rows are `not_covered` and keep their ordinary values: every other row kind; rows with no
  > body scale; and, under R-COMB-1, each quantity row of an unavailable combination or of an
  > ordinary combination that names a load case whose trigger is not `not_required`.
- The edited text:
  > These rows are `not_covered`: every other row kind; rows with no body scale; and, under
  > R-COMB-1, each quantity row of an unavailable combination or of an ordinary combination that
  > names a load case whose trigger is not `not_required`. They keep their ordinary values, except
  > the section normal-stress extremes and transverse-shear maxima, which are formed in binary64
  > from the successor's own published stress, section-action and end-force rows and its captured
> section and material.

## 5. Readers, verifier and consumers

### 5.1 Checks per new kind

The Rust reader (`result_export`, native and wasm) and the Python verifier check each new kind
on its route, independently of the producer. The verifier shares no code with it.

| Kind | Reader check |
|---|---|
| `element_local_normal_stress_extreme` | Recomputed bit for bit from the same station's and basis's published stress rows, by §2.3's fixed sequence. This includes the /1e6 conversion of the membrane row on pressurized pressure-2 members, and the shared correctly rounded `norm2` (E15) in native Rust, wasm and the verifier's exact-integer oracle (`tests/verifiers/physics_source.py`). It applies on every route: ordinary, retained projections (G7's base read) and source publications, under (ii). Coverage: every member, station and basis. |
| `element_local_transverse_shear_stress_maximum` | With the captured model: c_b recomputed from it by the route's ν rule, then τ bit for bit from the station's published shear rows (the section rows inside, the end-force shear rows at the ends) (§2.4). Without the model: custody, coverage, non-negativity and the unit. Coverage: every member, station and basis, except the members named by `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`. |
| arc `pipe_elastic_normal_stress_maximum_v2` | The §2.3 record's closed fourteen keys and arc tokens, accepted only on a realized arc. The gap rule and midpoint as on straights (`physics_evidence.rs` around :637–645). `local_fraction` in [0, 1]. Coverage, basis twins and headline binding as on straights. The station cover: `value_upper_pa` ≥ every published station max(\|σ⁺\|, \|σ⁻\|) of the arc and basis, converted to Pa and rounded upward. The readers attest custody and do not replay the branch-and-bound (DEF-O `trust.validation` precedent). |
| directional intensified rows | Present exactly on arcs whose captured model has the directional pair. Each is bound to its `intensified_measures` record. Recomputed bit for bit from the station's bending stress rows and the record's factors (§2.6). The maximum's record follows the arc maximum's checks, with cover by the station intensified rows. |
| toroidal hoop and radial | Present exactly on pressurized arcs with R > r_o, at every station and basis. Recomputed bit-equal by the normative `stress_recovery` sequence, from p and the basis bore, wall, r_o and R in the evidence (§2.5). |
| `connector_tie_rod_force_v1` | One row per authored rod, bound to its `tie_rods` entry by row id. Finite and in N. The value is not rederived from the published displacements. The published displacements are the corrected displacement rounded once to binary64, or the solve's own where no member bound exists, while the certified T_r is formed from the unrounded corrected displacement, so an exact relation does not hold between them. This is the existing connector practice for g. |
| `s_x` | \|s_x − Σ\| ≤ (64 + n_r)ε(\|g_x\| + \|Pe\| + Σ\|t_r·axial_share\|) + (n_r + 2)·2^-1074. Σ = g_x − Pe + Σ_r RN(t_r·axial_share_r) is accumulated exactly, rounded once: `ExactAccumulator` in Rust, exact rationals in the verifier. So the reader adds no addition error for any rod count. The (64 + n_r)ε coefficient covers the producer's at-p rounding and the n_r rounded published products, so no rod cap is needed. This extends today's rule, 64ε\|g\| + 64ε\|Pe\| plus a least-subnormal reserve (`connector_evidence.rs` around :271–280 and :345), to the rod terms. Pe is the published `pe_x`, or 0 on an unpressurized connector. The rule applies to every connector with rods or a constant-effective-area model. The current `UNPRESSURIZED_ROWS` rule, which forbids s_x on an unpressurized connector (around :336–339), becomes keyed on hardware: s_x is required with rods and forbidden without them. Tests cover cancellation, subnormal edges, and a many-rod control (for example 1,024 parallel rods each contributing 2^-54 beside g_x = 1). |

- **Arc evidence.** `arc_evidence.rs` and `tests/verifiers/pressure_1.py` (renamed
  `pressure_2.py`) drop `withheld_on_arcs`. They require the two definition ids and the new
  rows' coverage. `physics_evidence` stops refusing the formerly withheld kinds on arcs.
- **Connector record law.** `connector_evidence.rs` `RECORD_LAW` and the verifier accept the two
  hardware kinds and the `tie_rods` record, and refuse a rod record without rows, rows without a
  record, a duplicate rod and an `untied` record with `tie_rods`.
- **Shared corpora, regenerated by their generators.** These are the J2 corpus, the pressure
  reader corpora and `base_reader_outcomes.json`. Each gains genuine entries for the new kinds,
  plus tamper mutations: a resealed changed σ±, τ, hoop, rod force or s_x; a missing row; a
  withheld-on-arcs revival; a retained successor whose derived row keeps a value that disagrees
  with its published resultants. Native Rust, wasm and the verifier consume them unmodified.

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
- **The report package's fresh-result check.** `core/reporting/report_package/src/wire.rs`
  (`fresh_semantics_metadata`, around :660–674) recognizes fresh results by tokens only they
  carry. It gains the new component, location and basis tokens, so the check stays complete.
- **Python product paths** read through Rust and change only their pinned identities and
  tables: `compatibility.py`'s `CURRENT_RECORD_CONTRACT_IDS` and table pins, and
  `stress_neutral/package.py`'s method sets.

## 6. D3, pricing and qualification

The epoch moves priced sources (the PP producer, the readers, `model_format.rs`) and ten
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
  - At the caps (n = m = g = 32, C_eq = 3), route E rises to about 11,241 rows, below its 16,384
    capacity. Route L's 2·C_eq·P_final text measure rises to about 15,570 of 16,384. D3
    re-establishes, on actual source, the premise that route E's row text stays below route L's
    (retained-pricing-rules.md §9, "Row text"), and every capacity, rather than assuming them.
  - The not-covered passthrough's capture records, final-case entries and wire entries are new
    positive owners.
- **Public regeneration.** Corpora, successors and outcome files are regenerated by the actual
  public producer after D3, with no private writer or temporary gate change.
- **The exact-pressure qualification package.** U4b removes the arc maximum's former absence
  from the package, which is regenerated by its generator. Its admission statuses are relied on
  only after an independent re-review of the regenerated package; its admission record is not
  edited by the implementer.
- **Actual-source result and its reduction (2026-10-18/19).** Priced on the integrated source
  (55ad2c0260) with the unchanged policy, 16 of the 70 evaluations failed. The headroom was W3
  −0.92 GB, W4 −1.14 GB and X1 −1.05 GB (dense), after Codex's independent pricing corrections.
  The dense W4 change from the registered +113,088,665 B to −1,142,708,640 B is attributed site
  by site, with every byte assigned:
  - new source at the old row counts: 293.4 MB, of which new TEXT sites are 196.4 MB and new
    diagnostics carried through the envelope copies are 112.5 MB;
  - new owners: 21.3 MB;
  - route E rows 72 → 102 per member: 922.0 MB of per-row scaling of existing sites;
  - route L rows 51 → 66 via the merge: 19.1 MB.

  No site was priced twice. The pricing corrections are a missing contribution-group owner,
  a stale formulation allowance, U4B-NQ priced at 30m, and U4B-W1's diagnostics capacity.
  The decisive cost was materializing each publication to hash it (T16, T17, T25:
  `canonical_json_checked_v1_text(to_string(...))` over a clone). HELP_HUMAN approved, within
  authority, replacing it with one canonical writer over the borrowed Value with a sink. A
  SHA-256 sink streams the canonical form into the digest at the three hash sites, and a
  String sink serves only where text is needed. Digests and refusals are unchanged, including
  the 128-level recursion limit. The estimated headroom after it is at least +2.93 GB in every
  phase. Registration follows on that source (S6).
- **Headroom.** Before the epoch the tightest phase was merged dense W4, with 113,188,985 B of
  headroom. The registered epoch-4 profile (S7) fits all 70 evaluations; its tightest phase is
  merged dense W3, with +2,923,863,736 B (retained-pricing-rules.md §9–§10 record the
  registration).

## 7. Evidence plan

These checks are targeted regressions and independent scrutiny for numerical meaning and
persistence (AGENTS.md). Both solver modes run throughout.

1. **t4_u4b.**
   - A builder test authors the eight requests in 0.6.0 from `cases.json`, following its
     "Binding" rules (bend components attached at the from node, with R, the binary64 bend angle,
     k and the directional pair). For the two preview cases, which give E and ν but no G, the
     builder authors G = RN(E/RN(2·RN(1+ν))), so the route's derived ν matches the oracle's within
     a few ulps, inside the criterion. It adds the request SHA-256 values to the oracle by the
     generator's `--write`, which changes bindings only.
   - The PP test runs every case through the typed and captured entries in both modes and
     compares every mapped row (§2.2) at the README criterion and zero scales:
     - interior maxima within their `location_band`, end maxima exactly at their end;
     - the headline per basis;
     - `max_abs_normal` as max(|σ⁺|, |σ⁻|);
     - the fibres, derived from the published M rows by §2.3's rule, at the criterion with zero
       scale r_o. Where the oracle records a tie and the published hypot(M_y, M_z) is within the
       criterion times the member's moment zero scale, the fibre comparison is skipped (ARC106
       end_j).
   - **Discriminators.** Each t4_u4b mutant is mapped to one of three kinds of check.
     - *Product rows.* Its listed rows lie at least 1e3 tolerances from the product's rows:
       - the hoop, radial, SV-maximum, `sv_nu_fixed_0_3`, maximum, `max_station_only`,
         `max_quadratic_straight_method_on_arc`, `max_abs_sum_objective`,
         `fibre_component_abs_sum` (on σ⁺ and `max_abs_normal`), `rows_other_basis_section`,
         `sif_factors_swapped`, `sif_one_scalar_for_both_directions`, `sif_applied_to_torsion`
         and `k_multiplying_stress` mutants.
     - *Field functions.* They are checked in the stress_recovery unit tests, against the
       oracle's field and fibre values:
       - `sv_vy_vz_swapped` and the SV field components;
       - `fibre_mz_y_sign_flipped` and `fibre_frame_rotated_y_away_from_centre`, through a
         fibre-normal function that is used in tests only and not published.
     - *Not formed in the product*, because their observables are not published:
       `sif_applied_to_whole_max_normal` and `sif_applied_to_axial`, on
       `axial_plus_intensified_bending`.
     - *The fibre convention.* The fibres are not a producer output (§2.3). Their sign is a
       property of the published bending rows, which U4a's references already discriminate. The
       derived-fibre comparison confirms the convention end to end, and a test-side mirror,
       (σ_z, −σ_y) in place of (−σ_z, σ_y), must fail against the oracle.
   - The stress_recovery unit tests compare the SV field at the six fibres and both surfaces,
     and the hoop at all six oracle fibres, with the oracle at the criterion.
   - The arc maximum function has its own tests:
     - ARC106's closed form;
     - a zero and a near-zero arc, which close under the absolute floor;
     - the subdivision limit refusal;
     - the location refinement against the oracle ψ* to the band;
     - that every bound is directed: no evaluation's upper value is below a 256-bit evaluation
       of the same represented problem.
2. **t4_u5b.**
   - A builder test authors the 38 requests in 0.6.0 from the case descriptions. Each base is
     re-expressed by the t4_u5a in-memory helper, and the rods are added in §3.1's shape, with
     offsets in the connector axes.
   - The PP and headless tests run all 38 in both modes with export and compare every frozen
     observable at t4_u5b's criterion: q, g, Tb, the connector endpoint rows Bᵀg, each T_r, ΣT_r,
     Pe, S_joint, the interface identity at both faces, each rod's `direction_global` and
     `axial_share` against the oracle, pipe Nw/S/end actions, displacements, reactions and
     `assembled_pressure_rhs_global`.
   - The discriminators' wrong values lie at least the stated separation away.
3. **t4_u5a, t4_u4a, t4_u6, t4_u8 and T4-I7 stay green**, through in-memory 0.6.0 re-authoring,
   in both modes. Every expected value is unchanged, except t4_u5a RV-11's code, which is
   superseded by name (§3.1).
4. **Refusals and guards.**
   - Unit tests cover every §2.6 and §3.1 code, with `MODEL_INCOMPLETE` and no results, and every
     retired identity's refusal by name.
   - The torus-domain guard has tests at R below, at and one ulp above r_o, and at
     R = (r_i + r_o)/2 (§2.5).
   - ν resolution has controls for authored ν, ν from E and G, a named-point basis, and an
     out-of-domain ν (§2.4).
   - The enclosed sine and cosine have their unit tests (§2.3).
   - End-i tension and compression with nonzero bending: σ± follow the j-side stress rows, not
     the node-on-element end-force rows (§2.3).
5. **Producer mutants.** Each is applied to the producer, run, reverted and recorded in the PR,
   with no committed seam. They are the t4_u4b mutants formed in the product and the t4_u5b
   mutants (`j4_double_count`, `rods_lumped_on_centreline`, `offset_sign_error`,
   `arm_sign_end_i_only`, `e_ref_ignored`, `reference_load_sign_flipped`,
   `reference_load_omitted`, `rod_force_folded_into_g`, `s_joint_without_rods`,
   `s_joint_section9_literal`). `ideal_in_place_of_finite` is not formed in the product, because
   the ideal law is refused by name (§3.1). Its refusal test covers it.
6. **K-D5 closure.** For the TieRodReference family:
   - drop, duplicate and extra summands, wrong-rod and wrong-connector bindings, and wrong
     operands, on the ordinary and W2 routes;
   - FK's family and certified-recovery tests against an exact reference;
   - the rigid-mode test.
7. **Readers.** RE (native and wasm) and the verifier accept the regenerated corpora and refuse
   every tamper (§5.1).
8. **Retained capability (§4.1).**
   - The censuses and the N06 and CX-F failed-ordinary controls keep every selection and
     standing.
   - Every new kind classifies `not_covered` in the producer, both readers and wasm, and none
     reaches `Unpublishable`.
   - No `NotCovered` entry changes a verified row's class, bound or scale. This is checked by
     comparing every verified row of the regenerated successors with its pre-epoch value and
     class.
   - The divergent ordinary-versus-retained case of §4.1.
   - The pressure-source-1 treatment, capability and work-law tests of §4.3. Every maintained
     source control still selects, and the J2 corpus and the j7 census keep their selections.
9. **M3.** The arc and connector status census in both modes, in the batch-5 form: arcs gain no
   Passed↔Sensitive move from these rows, and rod cases are Passed with the member bound.
10. **D3 and registration** as §6, with independent source and pricing scrutiny.
11. **Independent review.** This design, then each major slice, gets the dual review. Codex
    supplies the independent numerical scrutiny AGENTS.md requires.

## 8. Dependencies and staging

**Supplier needs** (recorded in `deliverable.yaml`):
- **DEL-04-01:**
  - the tie-rod element, `LoadFormula::TieRodReference` and `certify_tie_rod_recovery` (§3.2);
  - the retained kernel's `ProductRecipe::NotCovered` (§4.1);
  - `certify_arc_stress_maximum`, with its enclosed sine and cosine (§2.3).
- **DEL-05-03:** the extremes, the transverse-shear factor, maximum and test-only field, the
  fibre-normal test function, and the toroidal hoop and radial values, in `stress_recovery`
  (§§2.3–2.5). These are binary64 formulas with no new crate edge.
- **DEL-05-02:** the base identities and readers it consumes become preview-physics-2 and
  pressure-2. The combination engine applies §2.7's marks.
- **DEL-04-08:**
  - the not_covered classification, the W1 passthrough and the re-inheritance (§4.1);
  - the source treatment `derived_not_qualified` (§4.3);
  - D3, applied under its pricing law.
- **DEL-02-05 and DEL-02-01:** model 0.6.0 (§1.1). DEL-02-05's commitment sentences that name the
  version were edited, version token only, under the owner's 2026-10-17 confirmation (§4.4).

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
6. **S6, hashing reduction (added after the D3 attribution).** One canonical writer over the
   borrowed Value with a sink (§6), byte-identity regression, the pricing corrections, then D3
   and registration on that source.

S2 and S3 touch the same producer, tables and readers. They proceed in that order on one
branch.

## 9. Blast radius

**Moves:**
- every ordinary envelope (identity, table binding, limitation texts, and the new rows);
- every model fixture (version);
- every corpus, outcome file and successor that carries those bytes;
- the ten reviewed inputs (§1.3);
- the registered profile;
- the desktop route vocabulary and pins, and the report package's fresh-token list (`wire.rs`);
- the analysis-run and stress-neutral schemas and packages.

**Unchanged:**
- every frozen oracle and reference request byte, apart from the U4b/U5b builders' additions;
- every numerical criterion, tolerance, gate, cap and work limit;
- D7;
- the retained verified-kind list and the recipes;
- every physical value of every existing row;
- the frozen UI benchmark.

## Open questions

- **Returns to HELP_HUMAN:**
  - an edit that would touch TBD-002's table sentence;
  - a D3 shortfall under the unchanged policy;
  - any further ScopeOfWork commitment sentence that names 0.5.0 or a retired identity. The status
    prose in DEL-03-01, DEL-05-02, DEL-05-03, DEL-04-08, DEL-07-05, DEL-08-06, DEL-14-02 and
    DEL-17-06 was put to HELP_HUMAN on 2026-10-17 and reconciled under the token-only rule (§4.4).
