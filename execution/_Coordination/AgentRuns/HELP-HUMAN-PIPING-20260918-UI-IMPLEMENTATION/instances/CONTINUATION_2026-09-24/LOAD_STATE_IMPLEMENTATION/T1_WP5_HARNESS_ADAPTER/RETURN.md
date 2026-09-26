# T1_WP5_HARNESS_ADAPTER — RETURN

TASK (Type 2), requested by the T1 WORKING_ITEMS manager. Brief: `TASK_BRIEFS/T1_WP5_HARNESS_ADAPTER.md` with `TASK_BRIEFS/_T1_COMMON.md`. Start commit `d8f0dc4f7`, branch `codex/piping-load-states-20260925`. Python only. No Git writes, no product solver or runner binary run, and no WP6 fixtures written. All paths below are WORKING_ROOT-relative (`projects/chirality-piping/`).

**Status: complete, with one deliberate open signal.** `test_working_tree_reader_matches_pin` fails right now because WP1 has already changed `core/analysis_runs/load_reference_evidence.py` in the shared worktree. The brief anticipated this: the manager re-pins `MODULE_SHA256` at integration (§6). All other 42 tests pass. The manager's required change (a), several negatives per selector, is implemented (§9).

## 1. Files changed

All three are new files, so there is no "before" hash.

| File | sha256 before | sha256 after |
|---|---|---|
| `tools/validation/qualification_load_reference.py` | (absent) | `009ca9e895a751a90fa9a5fb3ebd0e2ff3d7d8dbb384b0d4ba63c74ffbba9aae` |
| `tools/validation/qualification_load_reference_helper.py` | (absent) | `c9de926cc19aaaffb3ed066b5bb2b00b0dc96683979e6c9be6f656ddf35916b9` |
| `tests/test_qualification_load_reference.py` | (absent) | `64436b472d0603086a0133e3e812df8268a004a421bee1c84b642553d9b5bfc0` |

No existing file was modified. `qualification_gate.py`, `qualification_process.py` and `qualification_physics*.py` are imported and reused, never copied or edited. Return records: this file and `_run_records/{mutants.py, mutants.log, wp5_suite.log, protected_suites.log}`.

## 2. What each change does

### 2.1 `qualification_load_reference.py`: the closed adapter

**Pinned identities**

| Item | Value |
|---|---|
| Transport | `load_reference_1_cli_1.0_raw0.2`: `openpipestress-runner solve --input - --solver-mode <sparse_interactive\|dense_scrutiny>`, plus `--explicit-local-private-intent` only when selected; ControlledExport / CLI 1.0 / raw mechanics 0.2.0 |
| Contract | `openpipestress.result_semantics/0.3.0/load-reference-1`; producer `open_pipe_stress_product_physics` 0.2.0 |
| Profile | `resolved_straight_load_state_v1` |
| Table | `fixtures/results/semantic_contract_v0_3_load_reference_1.json` `44bc41c0…4c4f4d` |
| Reader module | `core/analysis_runs/load_reference_evidence.py` `f7d50f1cd70685499352cd1ce025d2c9f7aa14d7fbad3f114702fcc25ba08b5a` (bytes at start commit `d8f0dc4f7`) |
| Inherited physics-1 reader and table | as pinned by the physics adapter: `40013aa4…` and `9a2cf626…` |
| Units | `core/units/src/lib.rs` `521717eb…`, as pinned by the physics adapter; bound read-only, never executed |
| Comparator | the existing PKG-14 `_classify_delta`, hash-pinned through `gate.CLASSIFIER_SHA256` (unchanged) |

The adapter refuses:
- `load-reference-source-1` and its source profile;
- physics-1 and `exact_straight_pressure_v2`;
- raw 0.1;
- `source_block_recovery` and `carrier_evidence`, even when null;
- a missing `load_reference_states` namespace.

**Inputs: two records, so the WP6 manifest stays case-only**

- **Run selection** `openpipestress.load_reference_qualification_run/1` (closed keys):

  ```
  {format, profile_id, purpose: harness_development|development_comparison, transport,
   runner: {candidate_commit (40 hex), executable_sha256 (64 hex), solver_mode, explicit_local_private_intent (bool)},
   case_manifest: {path, sha256}, reader_binding: {path, sha256}}
  ```

  The two paths are relative to the selection file, or absolute. This record locks the WP6 `MANIFEST.json` by sha256.

- **Case manifest** `openpipestress.load_reference_qualification_manifest/1`: exactly the format in the brief, with closed top-level and per-case keys. The adapter requires:
  - nonzero and unique `case_id`s;
  - `modes` a nonempty, unique subset of the two modes;
  - `required_scalar_rows` a positive integer;
  - every path WORKING_ROOT-relative (no absolute path, no `..`), resolved inside the selected candidate.

  `analytical_reference.path` must be exactly `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json`, its sha256 must match, and `case_key` must be a key of its `cases`. A case is required in a run when the run's mode is one of its `modes`. Cases listed only for the other mode appear under `cases_not_required_in_mode`. If no case is required in the mode, the run is refused.

**Gate behaviour (HARNESS_CUT)**

- **Ledger first.** The full ledger is materialized before any process starts: every positive and negative assertion ID, plus 7 structural obligations per case.
  - If a case's selector file cannot be admitted, the case keeps `required_scalar_rows` placeholder obligations (`required-row-NNN-unresolved`) and is `blocked`. Its denominator never shrinks.
  - If the locked manifest itself is unreadable or malformed, there is no intelligible denominator, so the run is refused before any output: `selection_error`, exit 2.
- **Per-case binding.**
  - `runner_input.solve.preview_model` must equal `product_request`.
  - The model must be `schema_version` 0.4.0 with `pressure_contract` `{version 2.0.0, mode exact_straight_pressure_v2}`.
  - Every selector basis must be a load case of the bound input.
  - The reference and criteria follow the existing gate rules: `readiness: ready` with a basis; `harness_synthetic` only for `harness_development`; `independent_review_ref` for `development_comparison`; `profile_status: reviewed`; closed rule fields; same unit only.
- **Transport** (`unwrap`, which reuses the gate's `strict_json`, `_diagnostics` and `finite`):
  - wrapper counts and blocked state;
  - the runner `run_id` bound to `request_id`;
  - raw identity and profile;
  - `model_ref` equal to the input project;
  - `MECHANICS_SOLVED`, and no model mutation;
  - unique row IDs and a closed row shape;
  - every row value finite. NaN, Infinity, overflow, underflow-to-zero, null, strings and bools are all refused;
  - exactly one mode row per actual load case, each carrying the selected mode code.
- **Exactly-once resolution.**
  - A row selector must match its row by ID. Its complete signature (kind, unit, entity, case basis, and all five metadata fields: component, frame, location, basis/definition, sign) must also match exactly one row, and that row must be the same one. A shadow row with the same signature under another ID therefore fails.
  - An evidence selector must resolve one record, one list item, and every declared definition field (§3).
- **Missing stays missing.** A missing row, record or field, or a null evidence value, becomes `error` with `observed: null`. It never becomes zero.
- **Comparison.** Uses the existing `_classify_delta`, unchanged. A negative assertion passes (`wrong_result_excluded`) only when the observed value lies outside the rule's tolerance of the wrong value. Otherwise it records `wrong_result_reproduced`.
- **Numerical standing is reported separately.** Each case carries `numerical_standing: {standing, envelope_status, load_cases[...]}`, and the summary counts standings apart from the outcome counts.
  - `checks_passed` requires:
    - envelope and case `solve_quality` both `checks_passed`;
    - `passive_model_basis`;
    - `represented_equations_retained`;
    - `accuracy_evidence: not_claimed`;
    - only `NUMERICAL_INTEGRITY_CHECKS_PASSED` evidence.
  - `sensitive` and `insufficient` (not_assessed, unresolved or failed) leave the comparisons evaluated and reported, but the `numerical_standing` obligation is `failed`, so the case cannot pass.
  - A missing numerical-quality block refuses the transport.
  - An unknown standing value, or standing that does not cover every load case, makes the obligation `error`.
- **Structural obligations** (always all 7, predeclared):
  1. `transport_identity_and_mode`
  2. `actual_input_binding`: raw exact cases and records equal the input's load cases, and no row carries a foreign load case
  3. `load_reference_contract`
  4. `numerical_standing`
  5. `load_reference_record_binding`: one record per input load case, with contract and profile; `reference_configuration_id` equal to the input's `analysis_state.reference_configuration_ref`; `requested_mode` and `ordinary_{sparse|dense}_structural_v1` matching the run; `source_recovery` equal to `not_joined`
  6. `owned_reader_consistency`
  7. `complete_unique_selector_coverage`
- **Failure handling.** Interruption, timeout, truncation, output limit, nonzero exit and empty output make the case `error` while keeping its full denominator. On interruption, later cases stay `not_run`. Run-level identity failures block every case: executable digest, candidate commit, table and classifier hash, reader binding not reviewed, and reader dependency not pinned.
- **Custody and output.** Custody reuses the gate's retained-artifact and publication-custody functions. `ledger.json` is written atomically, and `summary.md` is derived from the same ledger. Exit codes are 0 when everything matched, 1 when obligations remain unsatisfied, and 2 for a selection error. Both outputs state `qualification: not_established_by_this_harness`.

### 2.2 `qualification_load_reference_helper.py`: the isolated owned reader

This mirrors `qualification_physics_helper.py`. It runs under `python -I -S`.

1. It verifies a binding (`openpipestress.load_reference_consistency_binding/1`, entry `validate_load_reference_evidence`) over the closed set of five dependencies: the LR reader, the physics-1 reader, both tables and units.
2. It copies the reader, the physics reader and both tables into a private snapshot.
3. It loads the two modules as members of one private package with an empty `__path__`. The reader's deferred `from .physics_evidence import …` therefore resolves only to the verified snapshot, and `.source_blocks`, which is not needed for raw validation, cannot be found on disk.
4. It calls the reader's own `verify_load_reference_table(bytes)`, then `validate_load_reference_evidence(raw)`.

Top-level imports of both modules are AST-checked against allowlists. A refusal is returned as `verdict: refused` with the reader's own error code (for example `SOURCE_LOAD_REFERENCE_MEMBER_MATERIAL_BINDING`). The adapter records that as a `failed` obligation, while comparison outcomes stay independent. The response states "internal source consistency only". Its input is a derived parsed snapshot, not the runner's stdout bytes.

### 2.3 `tests/test_qualification_load_reference.py`

The tests use synthetic controls only.
- A fake runner script replays the committed `fixtures/product_preview/load_reference/{connected,pressure}-{mode}.raw.json` inside a synthetic ControlledExport wrapper, keyed on an invented `request_id`.
- Reference values are copied from those raws and labelled `reference_kind: harness_synthetic`, with the basis "synthetic harness control copied from a committed producer raw; not an analytical reference".
- Criteria are harness-only rules (relative 1e-9, absolute 0).
- Each test builds a temporary git candidate that holds the pinned reader dependencies, the analytical reference file and a synthetic package under `validation/qualification/fixtures/synthetic_load_reference_harness/`. That path exists only inside the temporary directory; nothing is written to the real WP6 folder.
- When the working tree has drifted, the fixture copies the pinned reader bytes out of Git history, so the tests exercise exactly the pinned reader.
- The owned reader genuinely runs against the committed raws.

## 3. Selector extension (for WP6)

**Selector file** (`openpipestress.first_static_selector_candidate/1`)

- Required keys: `format`, `case_id` (equal to the manifest case), `producer_contract` (load-reference-1), `raw_schema_version` `"0.2.0"`, `row_namespace` `"payload.mechanics_envelope.results"` and `assertions` (nonempty).
- Optional keys: `gaps`, `scoring_readiness` and `negative_assertions`. Nothing else is accepted.
- Each assertion is `{id, selector, criterion_rule_id}`, and may add `selector_origin`, which is descriptive and not read.
- Assertion IDs are unique across both lists. Positive selectors are unique.
- Negatives are keyed on their assertion ID. Several negatives may share a selector (one per wrong value), and a negative may address the same quantity as a positive. The pair (quantity identity, wrong value) must be unique, where the quantity is the row ID or the evidence case/record/key/field; -0.0 equals 0.0. A duplicated pair blocks the case with `duplicate negative (selector, wrong value) pair` and keeps its declared denominator (§9).
- `required_scalar_rows` equals the number of **positive** assertions: row selectors plus evidence selectors.

**Row selector:** exactly the first-static keys `{id, kind, unit, entity_ref, basis_ref, metadata|null, dimension}`.
- `basis_ref.ref_type` must be `load_case`.
- `metadata: null` means the raw field is absent. A raw field that is present but null does not match.
- `(kind, unit, component)` must name exactly one load-reference-1 table signature whose dimension equals `dimension`, and the rule's `result_family` must be that signature's non-null family.

**Evidence selector** (closed keys):

```
{"id": "<unique>", "namespace": "contract_evidence.load_reference_states",
 "basis_ref": {"ref_type": "load_case", "ref_id": "<load case>"},
 "record": "member" | "support_component" | "contribution",
 "key": member → {"pipe_id"} | support_component → {"support_id", "dof"} | contribution → {"owner_kind", "source_id"},
 "field": "<closed field>", "definition": {<exact identity fields, may be {}>},
 "unit": "...", "dimension": "..."}
```

Resolution rules:
- exactly one record with `load_case_id == basis_ref.ref_id`, and exactly one list item whose key fields are equal;
- every `definition` entry must be equal, with the same JSON type;
- a null field is `error` (unavailable, not zero).

Allowed `definition` keys:
- member: `material_id, material_selection_kind, reference_basis, thermal_definition, expansion_law_id, fit_kind, eigenstrain_composition, G_basis` (text);
- support_component: `node_id, law_kind, meaning, physical_state_source` (text) and `global_dof` (integer);
- contribution: `classification, category, dimension` (text).

Closed numeric fields, as (unit, dimension, rule `result_family`):

| record | field | unit | dimension | family |
|---|---|---|---|---|
| member | `selected_E_pa`, `derived_G_pa` | Pa | stress | load_reference_material |
| member | `selected_nu`, `interpolation_fraction` | 1 | dimensionless | load_reference_material |
| member | `operating_temperature_k`, `material_selection_temperature_k`, `installation_temperature_k`, `coefficient_datum_k` | K | temperature | load_reference_temperature |
| member | `installation_datum_stretch`, `operating_datum_stretch`, `thermal_strain`, `thermal_stretch`, `fit_strain`, `fit_stretch`, `total_eigenstrain` | 1 | dimensionless | load_reference_strain |
| member | `reference_length_m` | m | length | load_reference_geometry |
| support_component | `prescribed_value` | m (U*) / rad (R*) | length / angle | load_reference_support_motion |
| contribution `resolved_member_state` | `value` | 1 | dimensionless | load_reference_strain |
| contribution `support_state` | `value` | m / rad from the `source_id` DOF suffix | length / angle | load_reference_support_motion |
| contribution `stored_primitive` | `factor` | 1 | dimensionless | load_reference_load_factor |
| contribution `stored_primitive` | `authored_normalized_magnitude`, `applied_magnitude` | N (force) / N*m (moment), checked against the item's `dimension` | force / moment | load_reference_applied_load |

A `pressure_region` contribution has no numeric field and cannot be addressed.

**Negative assertions.** They use the same `{id, selector, criterion_rule_id}` shape in `negative_assertions`. Their wrong values live in the reference file's optional `wrong_values: [{assertion_id, value, unit, discriminator}]`, whose IDs must equal the negative IDs exactly. `values` covers exactly the positive IDs.

## 4. Checks

Commands are run from WORKING_ROOT with the given venv and `PYTHONDONTWRITEBYTECODE=1`. Unittest is used because the repository `conftest.py` builds Rust binaries; `GATE_USAGE.md` gives the same instruction.

| Command | Result |
|---|---|
| `python -m unittest -v tests.test_qualification_load_reference` | 43 run: 42 pass, 1 deliberate failure (`test_working_tree_reader_matches_pin`, §6) |
| `python -m pytest -p no:cacheprovider --noconftest -q tests/test_qualification_load_reference.py` | 42 passed, 1 failed (the same one) |
| `python -m unittest discover -s tests -p test_qualification_gate.py` (unchanged) | 31 OK |
| `python -m unittest discover -s tests -p 'test_qualification_physics*.py'` (unchanged) | 42 OK |

Logs, with machine paths replaced by placeholders: `_run_records/wp5_suite.log` and `_run_records/protected_suites.log`.

**Seeded faults covered**

- **Values:** wrong value, wrong sign, and a reproduced wrong result (negative assertion).
- **Unavailable, never zero:** missing row, missing record or member, null evidence value.
- **Duplicates:** duplicate evidence member, duplicate result ID, a shadow row with the same signature, duplicate case, assertion and positive selector; a duplicated negative (selector, wrong value) pair (same selector, signed zero, renamed evidence selector); several wrong values per selector admitted.
- **Selector mismatches:** wrong case, entity, unit, frame, location, sign, definition or component.
- **Contract:** physics-1, load-reference-source-1, source profile, exact-pressure profile, raw 0.1, `source_block_recovery`, `carrier_evidence`, missing `load_reference_states`.
- **Values that must refuse:** NaN, Infinity, 1e999, null, "1.0", true, 1e-400.
- **Transport and substitution:** wrong mode, substituted model, blocked wrapper, unsolved mechanics, a raw case set different from the input, a foreign case row.
- **Records:** record mode, join status and reference configuration.
- **Standing:** sensitive, insufficient (three variants), unknown or uncovered, missing.
- **Owned reader:** a reader refusal while every comparison matches; helper isolation, binding digest, dependency digests, producer identity.
- **Manifest:** zero cases, no case in the mode, bad format, extra keys, bad modes, zero rows, locked-hash mismatch.
- **Case bindings:** product request, changed bytes, analytical key and path, absolute path.
- **References and criteria:** readiness, draft criteria, synthetic reference for comparison, missing review, values or wrong-values denominator, null reference, unit.
- **Semantics:** row and evidence selector semantics (13 variants).
- **Run identity:** executable, commit, unreviewed binding, unpinned dependency.
- **Process:** nonzero exit, timeout, output limit, interruption, KeyboardInterrupt.
- **Custody:** a bound file changed during the run, with the ledger present before the process and reference files unchanged afterwards.

## 5. Mutation evidence

`_run_records/mutants.py` ran on a scratch copy: a `git archive` of `d8f0dc4f7` for `tools/validation`, `core/analysis_runs`, `core/comparison`, `fixtures/results`, the load-reference raws, units and the analytical reference, plus the three WP5 files. The copy was a git repository of its own. Each mutant replaces one refusal predicate with an always-true or neutral form, runs the WP5 suite without `PinTests` (the scratch repository lacks `d8f0dc4f7`), and restores the file.

| Mutant | Result |
|---|---|
| M01 duplicate case ID | killed (failures=1) |
| M02 zero-case manifest | killed (failures=1) |
| M03 no case required in mode | killed (failures=1) |
| M04 duplicate assertion ID | killed (failures=1) |
| M05 duplicate selector | killed (failures=1) |
| M06 required_scalar_rows count | killed (failures=1) |
| M07 selector producer contract | killed (failures=1) |
| M08 raw producer/contract | killed (failures=1) |
| M09 raw profile | killed (failures=1) |
| M10 foreign namespaces | killed (failures=1) |
| M11 load_reference_states namespace | killed (failures=1) |
| M12 standing shape (missing standing) | killed (failures=1) |
| M13 insufficient standing | killed (failures=2) |
| M14 unknown standing value | killed (failures=1) |
| M15 standing coverage | killed (failures=2) |
| M16 sensitive classified as passed | killed (failures=3) |
| M17 duplicate result ID | killed (failures=1) |
| M18 nonnumeric row value | killed (failures=1) |
| M19 mode evidence | killed (failures=1) |
| M20 model identity (substitution) | killed (failures=1) |
| M21 blocked wrapper | killed (failures=1) |
| M22 mechanics solved | killed (failures=1) |
| M23 row resolves once by signature | killed (failures=2) |
| M24 evidence resolves once | killed (failures=2) |
| M25 null evidence value | killed (failures=1) |
| M26 evidence definition | killed (failures=1) |
| M27 negative assertion inversion | killed (failures=1) |
| M28 process outcome | killed (failures=2) |
| M29 interruption stops later cases | killed (failures=1) |
| M30 KeyboardInterrupt stops later cases | killed (failures=1) |
| M31 owned reader verdict | killed (failures=2) |
| M32 reference readiness | killed (failures=1) |
| M33 synthetic target for comparison | killed (failures=1) |
| M34 reviewed criteria | killed (failures=1) |
| M35 product request binding | killed (failures=1) |
| M36 analytical case key | killed (failures=1) |
| M37 WORKING_ROOT-relative paths | killed (failures=1) |
| M38 reference denominator | killed (failures=1) |
| M39 row selector semantics | killed (failures=1) |
| M40 evidence field vocabulary | killed (failures=1) |
| M41 evidence unit/dimension | killed (failures=2) |
| M42 selector basis in bound input | killed (failures=1) |
| M43 raw load cases = input (substitution) | killed (failures=1) |
| M44 executable digest | killed (failures=1) |
| M45 candidate commit | killed (failures=1) |
| M46 reviewed reader binding | killed (failures=1) |
| M47 reader dependency pin | killed (failures=1) |
| M48 bound files unchanged during run | killed (failures=1) |
| M49 criterion rule binding | killed (failures=1) |
| M50 run transport | killed (failures=1) |
| M51 locked manifest hash | killed (failures=1) |
| M52 record mode/recovery method | killed (failures=1) |
| M53 record not joined | killed (failures=1) |
| M54 record reference configuration | killed (failures=1) |
| M55 foreign raw load cases | killed (failures=1) |
| M56 selector-inventory refusal blocks case | killed (failures=1) |
| M57 negative (selector, wrong value) pair uniqueness | killed (failures=1) |
| M58 positive selectors unique | killed (failures=1) |
| H01 helper isolation | killed (failures=1) |
| H02 helper dependency digest | killed (errors=1) |
| H03 helper binding digest | killed (errors=1) |
| H04 helper producer identity | killed (failures=1) |

**62 mutants, 62 killed, 0 survived.** These were run on the final bytes of all three files; the scratch copies are byte-identical to the returned files. The full log is `_run_records/mutants.log`.

An earlier partial run on older test bytes had two survivors, M02 (zero-case manifest) and M04 (duplicate assertion ID). In both, a later, more general refusal masked the removed one. The refusal tests were tightened to assert each specific refusal message, and every mutant was then rerun.

## 6. Not done, or open

- **Reader re-pin.** The working-tree `load_reference_evidence.py` is `eff1fb3b…`, not the start-commit `f7d50f1c…`, because of WP1's concurrent edit. `test_working_tree_reader_matches_pin` fails until `MODULE_SHA256` is updated at integration. That is intentional: a stale pin would block every real run with `unselected owned reader dependency`. I ran a read-only snapshot check: WP1's current bytes pass all four committed raws through the helper, and their top-level imports fit the helper allowlist. The re-pin should therefore be a one-line hash change. If WP1 adds a top-level import, `ALLOWED_IMPORTS` in the helper must be reviewed too.
- **No real run.** No real run, WP6 case binding or product solver execution was done, per the brief. WP6 runs after its freeze.
- **Joined transport.** `load-reference-source-1` is out of scope and refused. It needs its own identity later.
- **Analytical reference hash.** Not pinned as a constant. It is bound per case by the locked manifest's sha256, and the path is fixed.

## 7. Design questions and the manager's rulings (2026-09-26)

Each question below is followed by the manager's ruling.

1. **Comparator form.** The existing PKG-14 classifier is the symmetric maximum, `|Δ| ≤ max(abs, rel·max(|obs|,|exp|))`. The rule cited for WP6 is reference-sided, `|obs − exp| ≤ 1e-9·|exp|`. For `|obs| > |exp|` the symmetric rule admits up to about `rel·(|obs|−|exp|)` more, which at rel 1e-9 is roughly a 1e-18 relative window. HARNESS_CUT forbids silently changing the classifier, so I kept it. Should a reference-sided predicate be introduced as a separately identified rule kind? Also, zero-reference handling needs an explicit per-rule `absolute_tolerance_value`; none is inferred.
   - *Ruling:* keep the symmetric classifier unchanged. WP6 criteria state explicit relative and absolute values per rule, and the freeze review judges them under that classifier. No new rule kind.
2. **Admission path.** Like the existing gate, the adapter scores only `readiness: ready`, `profile_status: reviewed` and (for comparison) `independent_review_ref`, read from the manifest-bound files. WP6's candidates are pending or draft, so they block. Either the freeze step rewrites the admission fields and re-hashes the manifest, or a derivation helper like `build_first_static_selection.py` is added. Which should it be?
   - *Ruling:* admission follows the independent freeze. A small deterministic script, which the manager owns at integration, sets the admission fields from the freeze record and re-hashes the manifest. WP5 builds no derivation helper.
3. **Formats for WP6.** WP6 has to align with the negative-assertion format (`wrong_values` in the reference file), the evidence vocabulary names (units, dimensions, families), the `required_scalar_rows` meaning (positive assertions only), and WORKING_ROOT-relative paths. I sent these to the manager early. If WP6 chose differently, one side needs a small change.
   - *Outcome:* the manager required several negatives per selector (§9); that is implemented.
4. **Meaning of "refuse insufficient standing."** I compare the values, report standing separately, and fail the case on `sensitive` or `insufficient` standing. The alternative is to withhold comparisons entirely. Confirm the intended reading.
   - *Ruling:* confirmed as implemented.
5. **Separate run selection.** The run-level bindings (commit, executable, mode, purpose, reader binding) live in a separate run selection that locks the manifest, so WP6's manifest format stays exactly as briefed. Confirm that this split is acceptable.
   - *Ruling:* confirmed.

## 8. Process note

While stopping my own mutation run, I used a `pkill -f` pattern broad enough to match WP3's mutation runner command line. Process start times show WP3's runner did not yet exist at that moment; WP3 was in its cargo phase. Nothing of WP3's was affected. I switched to PID-based termination. The scratch space (`ls-wp5-scratch`) was deleted at the end.

## 9. Addendum: several wrong values per quantity (the manager's required change (a))

The manager ruled that WP6's references name several wrong values for one quantity: for example `thermal_datum_ratio` (two on the same member `thermal_strain`), `prescribed_rotation_free_tip` (two on tip UY) and `multi_segment_free_length` (up to five per strain). The first version rejected a repeated negative selector as a duplicate. Now:

- **Admission** (`admit_selectors`). Positive selectors are still unique (`duplicate required selector`). Negatives are checked only for unique assertion IDs, so any number may share a selector.
- **Pair check** (`prepare_case`). Once wrong values are read from the reference file's `wrong_values`, the pairs (quantity identity, wrong value) must be unique.
  - The quantity identity is the row ID for a row selector, and (case basis, record, key, field) for an evidence selector. Two evidence selectors with different `id`s that address the same quantity are therefore the same quantity.
  - Values compare as binary64, so -0.0 equals 0.0.
  - A duplicate blocks the case with `duplicate negative (selector, wrong value) pair`. The predeclared denominator is kept (`inventory_basis: selectors`).
- **Tests.**
  - `test_several_wrong_values_per_selector_are_admitted`: two wrong values on one support-reaction row, and two on one member `total_eigenstrain`. All are evaluated, and the run matches with 4 negatives.
  - `test_duplicate_negative_selector_value_pair_is_refused`: the same selector with the same value; a signed zero; and the same evidence quantity under a renamed selector ID.
  - The earlier inventory case that expected a repeated negative selector to be refused was removed. Positive duplicate selectors are still refused there.
- **Mutants.** M57 (pair uniqueness removed) and M58 (positive uniqueness removed) were added to the rerun in §5.
