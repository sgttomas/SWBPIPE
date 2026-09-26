# RETURN — T1_WP6_STATIC_CASES (VP-STATIC load/reference-state cases, authoring only)

- **Role:** TASK (Type 2). I did not delegate.
- **Requested by:** the T1 WORKING_ITEMS manager.
- **Checkout:** branch `codex/piping-load-states-20260925`, base `d8f0dc4f7`.
- **Git:** no writes.
- **Paths:** WORKING_ROOT-relative. `LSI` is this folder's parent.
- **Status:** authoring complete, pending the independent freeze.
  - No comparison was run.
  - No observed value is recorded, committed or used as a target.
  - No reference, criterion or case is admitted.

## 1. What was produced

A package at `validation/qualification/fixtures/load_reference/`.

**14 manifest cases over 12 reference keys, with 36 load cases:**

- 467 positive scalar assertions: row and evidence;
- 40 negative assertions;
- 22 recorded gaps.

**Families:**

- **Support motion (M10):** `prescribed_translation_two_bar`,
  `prescribed_translation_all_fixed`, `prescribed_rotation_all_fixed`,
  `prescribed_rotation_free_tip`.
- **Reference temperatures (M16):** `shared_material_serial_companion`,
  `thermal_datum_ratio.fixed` and `.free`, `coefficient_definition`,
  `constant_alpha_interval`, `multi_segment_free_length`,
  `temperature_unit_identity`.
- **Cold spring (M29):** `signed_fit_states.fixed` and `.released`,
  `persistent_source_once`.

**Excluded:** `shared_material_parallel`. The reference itself marks it as an
analytical topology only (two coincident bars). `shared_material_serial_companion`
is its practical companion, and the package README, `PROVENANCE.json` and the
selector limits all record that relationship.

**States: a load case or a qualification case?**

- The states of one reference case are load cases of one request, each an
  independent equilibrium, whenever the product can represent them that way.
  Examples:
  - cold, hot and return;
  - three thermal-table variants;
  - several strain intervals;
  - the unit-identity groups.
- A topology that differs (fixed against free or released) is a separate
  qualification case, because supports belong to the model.
- An interval that needs a different installation temperature is a separate
  member (`coefficient_definition`, `multi_segment_free_length`), because the
  installation temperature belongs to the member reference.
- Each selector file's `scoring_readiness.load_state_decomposition` states the
  choice for that case.

**Files per case:**

| File | Note |
|---|---|
| `<case>.preview_request.json` | 0.4.0 |
| `<case>.runner_input.json` | `solve.preview_model` equals the request |
| `<case>.selectors.candidate.json` | |
| `<case>.reference.candidate.json` | `pending_independent_review` |
| `<case>.criteria.candidate.json` | `draft_pending_independent_review` |

**Package files:** `MANIFEST.json`, `README.md`, `PROVENANCE.json` and
`generate_reference_values.py` (the generator).

**The generator.**

- It reads `reference_cases.json` at run time. Its sha256 is
  `4d7b7777…beeab`, which matches the CP2-extension hash.
- It evaluates `exact` at 120 digits. For `symbolic` exacts it re-evaluates
  `exp(p/q)-1` with `Decimal.exp`.
- It cross-checks every value against the file's `decimal` (1e-80) and `value`
  (bitwise).
- It applies only the declared transforms: identity, negate (the end-i axial
  force), `one_plus` (datum stretches) and the generic-to-annulus area ratio
  (the fit discriminators). It applies the exact m-to-mm factor.
- It rounds once to binary64 and writes the reference and criteria files.
- `--check` requires byte equality.

**Outputs.**

- The generator's sha256 is `272864e8…d1fb`.
- The output hashes are in `PROVENANCE.json` (`generator_outputs`) and in
  `_run_records/file_hashes.txt`.

## 2. Files changed (sha256)

Every file is new: none existed before, and no existing file was modified.
`_run_records/file_hashes.txt` lists the full after-hashes (89 files).

| File | sha256 (after) |
|---|---|
| `validation/qualification/fixtures/load_reference/MANIFEST.json` | `324afa50f0d11898eef0fb50d0e68d666edf119085e82d4af020a911dab21f0e` |
| `…/load_reference/PROVENANCE.json` | `c5a0cee9671db51f9a273e5238c731ce74b269653f0b49d80b5a9b9bc77e865a` |
| `…/load_reference/README.md` | `c73a4f063669e8f48761cb411716c1d3680b19e3e42e7f31de2d96e42ca20542` |
| `…/load_reference/generate_reference_values.py` | `272864e80a4f8a18a5bd96844b0dea2fce9aa9d8f0d3ec21b4309ef59690d1fb` |
| 14 × 5 case files | see `file_hashes.txt` (also bound in `MANIFEST.json`) |
| `LSI/T1_WP6_STATIC_CASES/_run_records/*` | see `file_hashes.txt` |

## 3. What each part does

**`_run_records/author_package.py`** writes the requests, runner inputs,
selectors and manifest in three phases.

- Every reference input is read from the `value` of `reference_cases.json`.
- Every other number is invented, and labelled so in its object's provenance.
- Selectors resolve each row lookup against a value-free row inventory.
- Evidence `definition` constraints are derived from the request, not from
  producer output.

**Assertions.**

- **All six reactions** at every support, wherever the reference defines
  mechanics:
  - reference-defined components take the reference value;
  - the other components are `equilibrium_zero`, with a stated reason.
- **Governing locations:**
  - the tension-positive member force at all five stations (end_i = −N);
  - the element end actions for the fixed rotation;
  - the reference displacements and rotations;
  - the prescribed values, including the `support_components` evidence.
- **Resolved-state evidence:** E, ν and G, the strains, stretches, temperatures
  in K, reference length and contributions.
- **Why each assertion exists** is in its `selector_origin.why_required`, and in
  the package README per case.

**Negatives.** Each numeric `wrong_result_discriminator` becomes a negative
assertion:

- same selector as its target;
- its own id and `wrong_values` entry;
- the predicate is: outside the rule around the wrong value.

The generator requires each discriminator to be distinct from the reference.

**Gaps.** Non-scalar discriminators are gaps with an explicit kind:

- text discriminators implied by positive assertions;
- null or presence expectations;
- two refusal controls, each with a mutation and its expected blocking code;
- one numerically identical discriminator;
- unpublished internals.

**Criteria (candidates).**

- `relative_1e-9` for a nonzero expectation. This is the README consumer rule.
- `zero_scale:<tag>` for an exact zero: absolute = 1e-9 × a named same-dimension
  magnitude from the reference. This is the existing zero-reference handling of
  `load_reference_state_runtime.rs close()`.
- Both relative and absolute are stated explicitly on every rule.
- No new or relaxed threshold is allocated.

## 4. Checks (commands and counts)

Run from WORKING_ROOT with the session venv and `PYTHONDONTWRITEBYTECODE=1`. The
cargo target was a private scratch target, deleted at the end.

**Package checks.** The command is:

```
python LSI/T1_WP6_STATIC_CASES/_run_records/check_package.py --raw-dir <scratch raw> --producer <example binary>
```

The result is 5,945 passed and 0 failed (`check_package_producer_example.log`):

| Group | Checks passed | What it checks |
|---|---|---|
| A — manifest | 241 | Closed keys; WORKING_ROOT-relative paths with no `..`; every sha256 matches; `case_key` exists; `required_scalar_rows` = positive assertions |
| B — requests and runner inputs | 140 | Each request parses as 0.4.0 exact-profile with `analysis_state`; each runner input has the first-static request keys; `preview_model == request` |
| C — generator | 29 | Byte-for-byte reproduction of all 28 reference and criteria files |
| D — selectors | 4,079 | Every row selector names exactly one load-reference-1 table signature (kind, unit, component, dimension, family, signature id); every evidence record, key, field, definition and family is in the closed vocabulary; every rule exists with the same unit and dimension; values cover exactly the positives and wrong_values exactly the negatives; each (selector, wrong value) pair is unique |
| E — identification | 1,452 | In both modes, every row selector matches exactly one producer row on id, kind, unit, entity, basis and metadata; every evidence selector resolves to exactly one entry with a finite number, and the definition equals the record. No value is read |
| F — refusal controls | 4 | The duplicate `source:weight` gives `LOAD_STATE_SOURCE_DUPLICATE`; the non-equal temperature on exact_point gives `LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED`; both modes |

**Runner transport.** Every runner input was solved through the
`openpipestress-runner solve --solver-mode <mode> --explicit-local-private-intent`
transport. The A–E checks against that output give 5,941 passed and 0 failed
(`check_package_runner_transport.log`). The row ids are identical.

**Producer standing** (`identification_*.log`, no values):

- all 14 requests solved in both modes;
- load-reference-1, profile `resolved_straight_load_state_v1`;
- `checks_passed` for every case;
- no blocking diagnostic;
- the ordinary route (`not_joined`), with no `source_block_recovery`.

No request was refused.

**Reproducibility.** The full authoring pipeline (inventory, requests, selectors,
generator, manifest, provenance) was rerun with the venv interpreter. It
reproduced every package JSON byte for byte.

**WP5 compatibility probe** (`wp5_admission_probe.log`, adapter sha256 `dc2a2510…`).

- WP5's current selector admission, row and evidence semantics and rule binding
  accept all 507 assertions.
- 5 selector files are refused only for "duplicate required selector" among the
  negatives. That is the pre-ruling uniqueness rule, which the manager has since
  told WP5 to change.

## 5. Mutation evidence

`_run_records/mutations.py` and `mutations.log`:

- Each mutant runs on a fresh scratch tree made from `git archive HEAD` of the
  tracked inputs, plus a copy of the package.
- Manifest hashes are recomputed where needed, so detection comes from the
  substantive check.
- The baseline passes. **24 of 24 mutants are killed.**

| Area | Mutants |
|---|---|
| Reference values | a value changed by 2 ulp; a pointer swapped; the negate transform dropped; the mm unit without its factor; the reference decimal corrupted; the reference exact rational changed; a value dropped |
| Criteria | the zero absolute relaxed ×10; relative relaxed to 1e-8 |
| Selectors | a kind not in the table; a row id retargeted; a metadata location changed; an unknown evidence field; a missing evidence key; a definition contradicting the record |
| Manifest | a corrupted sha; `required_scalar_rows` off by one; an absolute path |
| Requests | runner and request drift; a 0.4.0 request downgraded to 0.3.0 |
| Negatives and zeros | a negative equal to the reference; a zero scale removed; a nonzero reference under a zero rule |
| Refusal controls | a control expecting the wrong code |

## 6. Not done

- **No comparison, freeze or admission.** This is authoring only, by the brief.
- **Signed-fit baselines** (no fit, cut long) are not authored. They need
  further member-reference inputs. Recorded as a gap.
- **Strain-only references have no mechanics assertions:**
  `coefficient_definition`, `constant_alpha_interval`,
  `multi_segment_free_length` and `temperature_unit_identity`. The reference
  defines none; `multi_segment` states "no mechanics companion values". The
  six-reaction rule is applied only where the reference defines mechanics.
- **Unpublished internals** are not asserted:
  - the reduced free RHS;
  - the intermediate source sums;
  - the logarithmic datum stretches `exp(I)`;
  - the consumed-segment records.
- **Non-scalar expectations** are specified as gaps but not scored by the
  adapter:
  - the consumed point id;
  - the null temperatures of `constant_alpha`;
  - the absence of the excluded source from contributions.
- **The binary64-conversion identity discriminator** cannot be represented
  under 1e-9, because it is about 1e-16 relative. The exact-point selections
  succeeding without override are the witness.

## 7. Design questions and freeze-review items

1. **Equilibrium zeros (148 assertions).** These are reaction components that
   the reference does not list but its topology makes zero, added to satisfy
   T1_PLAN's "all six reactions". They are not reference-file quantities. The
   freeze check should confirm or strike them.
2. **Zero-scale magnitudes.** Each is a named reference magnitude of the same
   dimension:
   - the case's reaction or wall force;
   - force × span, for moments;
   - θ·L, for the rotation case's UY zeros;
   - the all-fixed beam's actions, for the free-tip rotation.

   Each is stated in its rule's provenance. The manager marked them a
   freeze-review item.
3. **Derived mappings to review:**
   - end_i axial = −N;
   - the element end actions of the fixed rotation = the support-on-member
     vector;
   - the persistent-source member force = `combined_rhs` (N = F for an axial
     cantilever);
   - the pair case root Fx = −`two_distinct_equal_actions_rhs`;
   - datum stretches = 1 + dilation or 1 + integral;
   - the fit wrong values scaled by As/area.
4. **Invented inputs.** They are listed per case in `PROVENANCE.json`. Among them:
   - the Poisson ratios;
   - the serial companion's point temperatures (20 and 300 degC) and base E/ν;
   - the strain-only carriers;
   - the `coefficient_definition` 70 degC midpoint installation temperature,
     which the reference implies but does not list as a quantity;
   - the `thermal_datum_ratio` kelvin load case, which authors the operating
     temperature from `operating_temperature_K`.
5. **Persistent-source preload.** It is authored as an ordinary stored
   primitive (reference: an "equivalent affine tip action"). `device_reference`
   blocks in this capability, so its classification is `ordinary_applied`, not
   the ledger's `affine_reference`. No assertion is made on the classification.
6. **WP5 coordination.**
   - Negative uniqueness: the ruling is applied here. WP5 still has to change
     its admission.
   - Evidence `definition` constraints and families follow WP5's current
     closed vocabulary.
   - Refusal controls and structural gaps are outside WP5's scalar format. If
     they are to be scored, they need a harness extension.
7. **Row-id reproducibility.** Selector ids and metadata depend on the product's
   row naming at `d8f0dc4f7`. They were identified with the producer example,
   and the runner transport confirmed them. A producer change to row naming
   would need a re-identification, not a new reference.

## 8. Run records (`_run_records/`)

| Record | Content |
|---|---|
| `author_package.py`, `write_provenance.py` | Authoring |
| `run_producer_identification.sh`, `identification_summary.py`, `identification_*.log` | Identification-only runs, with value-free summaries |
| `check_package.py`, `check_package_*.log` | The checks above |
| `generator_check.log` | Generator reproduction |
| `mutations.py`, `mutations.log` | Mutation run |
| `wp5_admission_probe.py`, `wp5_admission_probe.log` | WP5 compatibility probe |
| `file_hashes.txt` | sha256 of every file written |

Producer outputs and row inventories stayed in scratch and are not committed.
