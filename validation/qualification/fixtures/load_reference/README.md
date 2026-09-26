# VP-STATIC load/reference-state cases (WP6 authoring candidates)

This package holds invented 0.4.0 product requests, runner inputs, selectors,
reference values and criteria for three load/reference-state families:

- support motion (M10);
- reference temperatures (M16);
- cold spring (M29).

**Status: authoring only.** No comparison has been run and no observed output is
recorded. Every reference file is `readiness: pending_independent_review`.
Every criteria profile is `draft_pending_independent_review`. Nothing here is
admitted. An independent freeze check comes before any run.

All inputs are invented test quantities. None of them is material-library,
component-library or code-rule data.

## Basis

- **Expected values.** They come only from the maintained independent
  analytical references,
  `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json`
  (sha256 `4d7b777708806c211cda0e126d7cd4eb7acba9b658b1da63cca36fd0523beeab`).
  Read its `README.md` for the consumer rules.
- **Wire.** `LOAD_STATE_IMPLEMENTATION/CP2_WIRE.md`, `CP2_WIRE_ADDENDUM_1.md`,
  `CP2_WIRE_ADDENDUM_2.md`, `CP3_WIRE_ADDENDUM.md` and `CP4_WIRE_ADDENDUM.md`.
- **Producer contract.** `openpipestress.result_semantics/0.3.0/load-reference-1`
  (table `fixtures/results/semantic_contract_v0_3_load_reference_1.json`,
  sha256 `44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d`), with profile `resolved_straight_load_state_v1`.

## Files

| File | Content |
|---|---|
| `MANIFEST.json` | `openpipestress.load_reference_qualification_manifest/1`: one entry per case. Every path is relative to `projects/chirality-piping` and carries its sha256. `required_scalar_rows` is the number of positive assertions (row plus evidence). |
| `<case>.preview_request.json` | The invented 0.4.0 product request (`{model, materials: []}`). |
| `<case>.runner_input.json` | An `openpipestress-runner solve` input: `{request, solve: {preview_model}}`. `preview_model` equals the product request byte for byte in content. The solver mode is the CLI `--solver-mode` flag. |
| `<case>.selectors.candidate.json` | `openpipestress.first_static_selector_candidate/1` with `producer_contract` load-reference-1: `assertions`, `negative_assertions`, `gaps` and `scoring_readiness`. |
| `<case>.reference.candidate.json` | `openpipestress.qualification_reference_values/1`: `values` for the positive assertions and `wrong_values` for the negative ones. Written by the generator. |
| `<case>.criteria.candidate.json` | A `tolerance_profile` of candidate rules. Written by the generator. |
| `generate_reference_values.py` | The generator. It reads `reference_cases.json` at run time. |
| `PROVENANCE.json` | Hashes of inputs, outputs, the generator and the authoring records, plus the invented inputs of each case. |

The authoring script, the check script and the mutation run are in
`LOAD_STATE_IMPLEMENTATION/T1_WP6_STATIC_CASES/_run_records/`.

## Regenerating the reference values and criteria

Run from `projects/chirality-piping`:

```
python validation/qualification/fixtures/load_reference/generate_reference_values.py --check
```

`--check` recomputes every reference and criteria file from `reference_cases.json`
and the selector files, and requires byte equality. `--write` rewrites them. The
generator imports no product module and reads no producer output.

**Evaluation.**

- `exact` is authoritative. It is evaluated in decimal at 120 digits:
  - `rational`, `rational_times_pi` and `rational_over_pi`, with pi from the file;
  - the `symbolic` forms `exp(p/q)-1` and `1-exp(p/q)`, re-evaluated with `Decimal.exp`.
- Each evaluation must agree with the file's `decimal` to 1e-80 relative, and with
  its `value` bitwise.
- The result is rounded once to binary64.

**What a selector declares.** Each assertion states in
`selector_origin.reference_origin`:

- the JSON pointer;
- the reference unit;
- one transform:
  - `identity`;
  - `negate`, for the element end-i axial force, which is `-N`;
  - `one_plus`, for a datum stretch `1 + dilation` or `1 + integral`;
  - `generic_area_to_annulus`, for the generic-area discriminators of the fit
    case, scaled by `As/area`;
- for a zero expectation, the zero scale.

**Units.** The only unit factor applied is the exact m-to-mm factor, because
displacement rows are published in mm.

**Equilibrium zeros.** An `equilibrium_zero` origin marks a reaction component
that the reference topology leaves at zero by equilibrium. Examples are the
transverse components of an axial bar. It is not a reference-file quantity. It
exists because T1_PLAN WP6 requires all six reactions of every support. Each
carries its reason, and the freeze check may strike any of them.

## Selector formats

**Row selector.** This is the first-static shape
`{id, kind, unit, entity_ref, basis_ref, metadata, dimension}`. The row `id` and
`metadata` were identified by running the producer example on each request in
both modes. The row ids resolve identically through the `openpipestress-runner`
transport.

**Evidence selector.** This follows the WP5 convention relayed by the manager:

```
{id, namespace: "contract_evidence.load_reference_states",
 basis_ref: {ref_type: "load_case", ref_id}, record, key, field, definition, unit, dimension}
```

| `record` | `key` | Fields used here |
|---|---|---|
| `member` | `{pipe_id}` | `selected_E_pa`, `selected_nu`, `derived_G_pa`, `thermal_strain`, `thermal_stretch`, `fit_strain`, `total_eigenstrain`, `installation_datum_stretch`, `operating_datum_stretch`, `installation_temperature_k`, `operating_temperature_k`, `material_selection_temperature_k`, `coefficient_datum_k`, `reference_length_m` |
| `support_component` | `{support_id, dof}` | `prescribed_value` (m or rad) |
| `contribution` | `{owner_kind, source_id}` | `applied_magnitude` (N) |

`definition` holds text constraints on the same record entry, and the
adapter requires the entry to equal them. They are derived from the authored
request, never from producer output:

- member: `material_id`, `material_selection_kind`, `reference_basis`,
  `thermal_definition`, `fit_kind`, and `expansion_law_id` when a law is used;
- support component: `node_id`, `law_kind` (`rigid_prescribed`) and `meaning`;
- contribution: `classification` (`ordinary_applied`), `category` and `dimension`.

Criterion families follow the WP5 closed vocabulary, per field:

- `load_reference_material`;
- `load_reference_temperature`;
- `load_reference_strain`;
- `load_reference_geometry`;
- `load_reference_support_motion`;
- `load_reference_applied_load`.

**Negative assertions.** A negative assertion has the same shape as a positive
one, and its selector is identical to the positive assertion it negates
(`selector_origin.negates_assertion`). Its wrong value is in the reference
file's `wrong_values`, with the discriminator pointer.

The predicate is that the observation falls outside the named rule around the
wrong value:

`|observed - wrong| > max(absolute, relative*max(|observed|,|wrong|))`

The generator also requires each wrong value to be distinct from the correct
reference under the same rule.

Several references name two or more wrong values for the same quantity, so
several negatives can share one selector. Each has its own id and its own
`wrong_values` entry, and each (selector, wrong value) pair is unique, as the T1
manager ruled on 2026-09-26. Where two discriminators give the same value on
the same selector, one negative scores both. The second is recorded as a
`numerically_identical_discriminator` gap (multi-segment dilation).

**Gaps.** A `gaps` entry records a discriminator or expectation that is not a
scalar assertion:

- `implied_by_positive`: a text discriminator that is excluded whenever the
  listed positive assertions pass;
- `structural_expectation_not_scored`: null, presence or string expectations on
  the evidence record;
- `refusal_control`: a mutation of the request, with the blocking code it must
  produce;
- `not_representable_under_criterion`;
- `not_published`;
- `not_authored`;
- `representability_limit`;
- `numerically_identical_discriminator`.

The two refusal controls were exercised against the producer for
identification. Both block with the named code.

## Criteria (candidates, per assertion family)

| Rule | When | Value |
|---|---|---|
| `criterion:<family>:<dimension>:<unit>:relative_1e-9` | Nonzero expectation | `relative_tolerance_value` 1e-9, `absolute_tolerance_value` 0 |
| `criterion:<family>:<dimension>:<unit>:zero_scale:<tag>` | Exact-zero expectation, from the reference or an equilibrium zero | `relative_tolerance_value` 0, `absolute_tolerance_value` = 1e-9 × zero_scale |

**Relative rule.** This is the reference README's consumer rule,
`|observed - expected| <= 1e-9*|expected|`. It is the existing protected
relative criterion.

**Zero-scale rule.** This is the existing dimension-aware zero-reference
handling of `core/product_physics/tests/load_reference_state_runtime.rs`
(`close()`). The zero scale is the case's own magnitude of the same dimension,
taken from the reference file. Examples:

- a reaction zero uses the case's nonzero reaction or wall force;
- a moment zero uses that force times the span;
- the free-tip rotation case uses the same beam's all-fixed end actions.

Each rule's `provenance` names its pointers and value.

No new or relaxed threshold is allocated.

**Two encodings of the same rule.**

- The existing gate classifier (`_classify_delta`) compares with
  `max(|observed|, |expected|)`.
- The README states `|expected|`.

At 1e-9 the two differ by less than 1e-18 relative. The T1 manager ruled that
the harness keeps the symmetric classifier. Each rule states explicit relative
and absolute values. Zero-reference absolutes carry their rationale and are a
freeze-review item.

## Cases

The states of one reference case are load cases of one request whenever the
product can represent them that way. Topologies that differ (fixed or free
supports) are separate requests, because supports belong to the model.

| Case id | Reference key | Load cases | Positive | Negative | Gaps |
|---|---|---|---|---|---|
| `prescribed_translation_two_bar` | same | 1 | 27 | 1 | 1 |
| `prescribed_translation_all_fixed` | same | 1 | 21 | 0 | 1 |
| `prescribed_rotation_all_fixed` | same | 1 | 24 | 0 | 1 |
| `prescribed_rotation_free_tip` | same | 1 | 17 | 3 | 0 |
| `shared_material_serial_companion` | same | 1 | 35 | 1 | 2 |
| `thermal_datum_ratio.fixed` | `thermal_datum_ratio` | 3 | 75 | 6 | 0 |
| `thermal_datum_ratio.free` | `thermal_datum_ratio` | 3 | 60 | 6 | 0 |
| `coefficient_definition` | same | 4 | 8 | 4 | 0 |
| `constant_alpha_interval` | same | 2 | 4 | 0 | 2 |
| `multi_segment_free_length` | same | 6 | 42 | 12 | 2 |
| `temperature_unit_identity` | same | 5 | 9 | 1 | 6 |
| `signed_fit_states.fixed` | `signed_fit_states` | 3 | 66 | 4 | 2 |
| `signed_fit_states.released` | `signed_fit_states` | 3 | 51 | 0 | 1 |
| `persistent_source_once` | same | 2 | 28 | 2 | 4 |
| **Total** | 12 reference keys | 36 | 467 | 40 | 22 |

### What each case asserts, and why

**Support motion.**

- **Translation cases.**
  - The complete displacement includes the prescribed value, so the prescribed
    node's UX and the explicit zero at the far node are asserted.
  - Two-bar only: the coupled free middle UX.
  - The resolved `support_components.prescribed_value` of each motion.
  - The tension-positive member force at all five stations of every member:
    end_i is `-N`, and the quarters, midspan and end_j are `+N`.
  - All six reactions at every support.
- **Rotation, all fixed.**
  - The prescribed root RZ and the explicit zeros.
  - The four signed support-on-member actions, as reactions and as element end
    actions. The element frame equals the global frame for a +X member.
  - The other reaction components, which are zero.
- **Rotation, free tip.**
  - Tip UY `L·θ` and tip RZ in radians.
  - Zero root actions and zero wall force.
  - Negatives: missing rotation coupling, and a length-normalized angle.

**Reference temperatures.**

- **`shared_material_serial_companion`.**
  - Middle UX, member forces and all reactions.
  - The per-member selected E, ν and derived G, the thermal strain and the total
    eigenstrain.
  - Negative: the material-ID-only E override.
  - It is the practical companion of `shared_material_parallel` (next section).
- **`thermal_datum_ratio`.**
  - Three load cases per topology:
    - the annular three-point table;
    - the reviewed two-point table;
    - the three-point table with the operating temperature authored in K.
  - Fixed topology: wall force and all reactions.
  - Free topology: tip UX, zero wall force and zero reactions.
  - Both topologies: the thermal strain and stretch, both datum stretches (via
    `one_plus` of the reference dilations), the three temperatures in K, and E.
  - Negatives on the strain: alpha_hot·ΔT, and subtracted datum dilations.
- **`coefficient_definition`.**
  - Datum-length and current-length strains: forward, reverse, first half and
    second half.
  - Each installation temperature is its own member.
  - Negatives: the endpoint coefficient, and reverse by negation.
- **`constant_alpha_interval`.**
  - Strain and stretch, with the interval authored in K and in degC.
  - The null-temperature discriminator is a gap.
- **`multi_segment_free_length`.**
  - The six strain directions for each of three definitions.
  - The datum stretches that have reference values (differential and dilation).
  - Negatives: the forward segment-truncation, interior-breakpoint and endpoint
    discriminators.
- **`temperature_unit_identity`.**
  - Four exact-point selections whose operating temperature is authored in
    another unit than the point: degC against K, degC and degF against K, and
    degC against degR.
  - Each asserts the operating temperature and the selected-point temperature
    in K.
  - The non-equal control (−49.999999 degC) is kept at 223.150001 K, with a
    negative against 223.15 K.
  - A refusal control makes an exact-point case block with the non-equal
    temperature.

**Cold spring.**

- **`signed_fit_states`.**
  - Cold, hot and return are three independent load cases.
  - Fixed topology: wall force and all reactions.
  - Released topology: tip UX, zero wall force and zero reactions.
  - Both: E, thermal strain, fit strain, total eigenstrain and reference length.
  - Negatives: additive strains, hot with cold E, fit applied twice, and a
    flipped sign.
- **`persistent_source_once`.**
  - The combined case: tip UX, all reactions, member force and the three
    contributions.
  - The two-distinct-equal-actions case: reaction, member force and both
    contributions.
  - Negatives: the naive total sum, on displacement and on reaction.
  - Gaps: the implicit-case-array absence, and the duplicate-source refusal
    control.

The per-assertion reason is `selector_origin.why_required` in each selector file.

## Representability limits and exclusions

- **`shared_material_parallel` is excluded.** It is two coincident parallel bars,
  which the reference marks as an analytical topology only, whose public
  admissibility is not established. `shared_material_serial_companion` is the
  practical companion that witnesses the same per-member material selection.
- **Persistent-source preload.** The +100 N preload is authored as an ordinary
  stored primitive. That is how the reference's annular topology declares it:
  an equivalent affine tip action. `device_reference` parses and blocks in this
  capability. The preload's classification is therefore `ordinary_applied`, and
  no assertion is made on it.
- **Strain-only references.** `coefficient_definition`, `constant_alpha_interval`,
  `multi_segment_free_length` and `temperature_unit_identity` define no mechanics.
  Their requests use an invented 1 m cantilever with invented E and ν, only as a
  carrier. No reaction or member-force assertion is made for them.
- **Internal quantities.** These are not published and are recorded as gaps:
  - the reduced free right-hand side;
  - the intermediate source sums;
  - the logarithmic datum stretches `exp(I)`;
  - the consumed segment records.
- **Signed-fit baselines.** The no-fit and cut-long baselines are not authored.
- **Invented inputs.** Every invented input is listed per case in
  `PROVENANCE.json`:
  - Poisson ratios where the reference gives none;
  - the base E and ν of materials selected by no member;
  - property-point temperatures that the reference leaves open;
  - the layout of the strain-only carriers;
  - the 70 degC midpoint installation temperature of `coefficient_definition`.
