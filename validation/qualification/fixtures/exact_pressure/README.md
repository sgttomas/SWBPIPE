# VP-STATIC exact_pressure_1 cases (T4-U2; pending independent review)

This package scores the `exact_pressure_1_cli_1.0_raw0.2` transport
(`tools/validation/qualification_exact_pressure.py`) against the frozen,
independently refuted T4-U2 references:

- `validation/references/t4_i7/u2_reference_cases.json` (T4-I7, pressure
  through realized bends; sha256 `a91bfbc8…2828`) with its documents
  `u2_document_sketches.json`;
- `validation/references/t4_i8/rebuilt_reference_cases.json` (T4-I8, the
  rebuilt straight cases; sha256 `838578ab…724b`).

**Status: not admitted.** The references are frozen and refuted (T4-RV3,
T4-RV4), but this package's mapping of them onto rows, evidence, criteria and
absences has not had its independent review. Until an `ADMISSION.json` names
that review, every generated reference is `pending_independent_review` and
every profile a draft, so the gate refuses the package. Admission changes
statuses only, never a value or rule. It is a development-comparison
admission, not engineering acceptance or a release criterion. Standard claim
fence applies (F-PIP-2; claims taxonomy per DEC-081).

All inputs are invented test quantities from the references. None of them is
material-library, component-library or code-rule data.

## Files

| File | Committed | Content |
|---|---|---|
| `generate_package.py` | yes | The generator. It reads the references and the pinned pressure-1 table only. |
| `MANIFEST.json` | yes | `openpipestress.exact_pressure_qualification_manifest/1`: one entry per case, binding each case file by path and sha256. |
| `cases/` | no (`.gitignore`) | Five files per case, written by `--write`: `runner_input`, `preview_request`, `selectors`, `reference`, `criteria`. |

The case files total about 72 MB, mostly selectors, so they are generated
rather than committed. `MANIFEST.json` binds their bytes, and the generator
reproduces them exactly.

From `projects/chirality-piping`:

```
python validation/qualification/fixtures/exact_pressure/generate_package.py --write
python validation/qualification/fixtures/exact_pressure/generate_package.py --check
```

- `--check` regenerates the package in memory. It requires `MANIFEST.json` and
  every present case file to be byte identical.
- `tests/test_qualification_exact_pressure.py` repeats the manifest check.

## Cases

Each case below runs on 0.3.0 and on 0.4.0, in both solver modes.

- **T4-I7.** Each of the 79 valued cases, run on its own document.
- **T4-I8.** Each owner with rows. Its v2 document is patched to
  `3.0.0/exact_pressure_v3` by the reference's own `v3_patch_for_both`.

Case IDs are `<reference case>@<version>`.

| Family | Cases per version | Positive per version | Negative per version | Absences per version |
|---|---|---|---|---|
| L-FREE (I7) | 26 | 6614 | 33 | 170 |
| L-ANCH (I7) | 25 | 6508 | 127 | 164 |
| U-ANCH (I7) | 18 | 12576 | 44 | 386 |
| CBPT (I7) | 8 | 696 | 29 | 56 |
| KINK (I7) | 2 | 512 | 13 | 14 |
| milltol_lame_membrane (I8, two variants) | 2 | 204 | 13 | 8 |
| tp_phys_pressure_halves (I8) | 1 | 548 | 11 | 12 |
| pressure_membrane_thin_wall_limit (I8) | 1 | 98 | 4 | 4 |
| v3_straight_twin_separate_closures (I8) | 1 | 106 | 0 | 4 |
| **Total** | **84** | **27862** | **274** | **818** |

Totals:

- **Per mode:** 168 cases, 55,724 positive and 548 negative assertions.
- **Terminal evidence:** 2,094 of the positive assertions are terminal-evidence
  selectors.
- **I8 rows:** every row is asserted, 956 per version. That includes the 150
  rows published in MPa and the three symbolic rows.

**Not in the package:**

- **I7 `U2-L-MITRE-REFUSED-P-K2`:** it publishes nothing. Its refusal is
  checked in `core/product_physics/tests/t4_u2_pressure_references.rs`.
- **I7 polygon-limit convergence record:** it is a study, not case rows.
- **I8 `v3_straight_twin`:** it has no rows of its own. Its SP-1 twin clause is
  checked bit for bit in the same Rust test.

## Mapping

**Row selectors.** These use the first-static shape
`{id, kind, unit, entity_ref, basis_ref, metadata, dimension}`:

- `dimension` and the result family come from the pinned pressure-1 table.
- The row ID and published metadata follow the producer's naming
  (`metadata()` and `row_id()` in the generator). They were identified once
  from producer output.
- A mismatch shows as an unresolved selector in the gate, never as a changed
  value.

**Terminal evidence selectors.** These address one component of a region
terminal's global vector:

```
{id, namespace: "contract_evidence.pressure", basis_ref: {ref_type: "load_case", ref_id},
 record: "terminal", key: {node_ref}, field, component: "x"|"y"|"z",
 definition: {closure_transfer}, unit: "N", dimension: "force"}
```

- `field` is one of `closure_pressure_load_global_n`,
  `pipe_cap_transfer_global_n` and
  `remote_closure_support_reaction_global_n`.
- The family is `pressure_terminal_force`.

**Declared absences.** A selector file's `absences` lists two kinds of
absence. The gate's `declared_absences` obligation checks both.

- `{row: {kind, entity_ref}}`: no such row may be published.
- `{evidence: {basis_ref, record: "terminal", key, field}}`: that terminal
  field must be `null`.

**T4-I7 mapping:**

- **Nodes.** Six DOFs per node. Translations are converted from m to mm.
- **Supports.** Six components per support.
- **Stations.** Five stations per member, with `N_w`, `S` and `sigma_m`.
  - The section actions and the bending and torsion stresses are also
    asserted; stresses are converted from Pa to MPa.
  - The section actions are not asserted at the end stations (see end rows).
- **End rows.**
  - A straight's end station action equals its element-local end row: end_i
    negated, end_j as is. The generator checks this equality exactly, so it is
    asserted once, through the end row. The station pointers are recorded as an
    `implied_by_positive` gap.
  - Arc end rows are the reference's `chord_frame_elastic` block.
  - The wall end action is along the end tangent (`tangent_frame`).
  - Not published, recorded as a `not_published` gap: the arc end-station
    section actions, the tangent-frame end rows and the element-local axial
    end force.
- **Lamé values.** Straights only, at all five stations.
- **Arc absences.** Five kinds are withheld on every arc, each a declared
  absence:
  - the Lamé radial and hoop rows;
  - the straight-statics maximum;
  - `element_local_axial_force` and `element_local_axial_normal_stress`.
- **Terminals.** Each non-null vector is asserted by component. Each null one
  is a declared absence.
- **Negative controls.** Every listed row of every
  `wrong_result_discriminators` control is asserted under the positive
  assertion's rule.
  - A control at a straight end-station pointer scores the end row with that
    sign.
  - Identical (selector, wrong value) pairs are scored once.

**T4-I8 mapping:**

- **Rows.** Every row of each load case, with its own criterion and its own
  `reference_origin` pointer.
- **Absences.** Four kinds are declared absent on every pressurized member:
  `element_local_axial_force`, `element_local_axial_normal_stress`,
  `pipe_section_pressure_hoop_stress` and
  `pipe_section_pressure_longitudinal_stress`.
- **Discriminators.** Each is mapped explicitly (`I8_DISCRIMINATORS`) to the
  quantity it names, in the variant and load case it names. It is scored at the
  first row that carries that quantity; every such row is asserted positively.
- **Inactive control.** `cap_area_on_nominal_bore` is not producible before
  T4-U6, so it is a `not_active` gap.

## Values and criteria

**Values.**

- T4-I7 values are 20-digit decimals. Each is converted with an exact unit
  factor in decimal and rounded once to binary64.
- T4-I8 values are `exact` first, evaluated at 130 digits:
  - `rational`, `rational_times_pi` and `rational_over_pi`;
  - the two symbolic forms `a + (b)/pi` and `sqrt((a*pi)^2 + (b)^2)`.

  Each value is cross-checked against the file's `decimal` (to 1e-80 relative)
  and against its `value` (bitwise).
- The only unit factors are m to mm and Pa to MPa.

**Criteria.** No new or relaxed threshold is allocated. The gate classifies
with `max(absolute, relative*max(|observed|, |expected|))`.

| Source | Rule | Values |
|---|---|---|
| T4-I7 | `criterion:<family>:<dimension>:<unit>:floor:<group>` | relative 1e-9 with the group's absolute floor 1e-9*zero_scale, converted to the row unit (the reference's `1e-9*max(|expected|, zero_scale(group))`) |
| T4-I8 nonzero | `…:relative_1e-9` | relative 1e-9, absolute 0 |
| T4-I8 zero | `…:zero_scale:<case>:<tag>` | relative 0, absolute 1e-9*zero_scale in the row unit; recomputed and required equal to the frozen row criterion |

## Running the gate (DEC-025)

1. Generate the cases with `--write`.
2. Name an admitted package and a reviewed reader binding,
   `openpipestress.exact_pressure_consistency_binding/1`. The binding lists
   the adapter's four `DEPENDENCIES`, and its entry is
   `validate_pressure_evidence`.
3. Run once per mode.
4. Set `--output-limit-bytes` to at least 16 MiB. The largest CLI output
   observed is 12.8 MB, a 0.4.0 U-loop in dense mode.

`explicit_local_private_intent` must be `true`. Without it, the runner blocks
the export of the evidence fields.
