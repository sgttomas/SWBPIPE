# VP-STATIC exact_pressure_1 cases (T4-U2; U4a tangent mapping; T4-U6 section bases)

This package scores the `exact_pressure_1_cli_1.0_raw0.2` transport
(`tools/validation/qualification_exact_pressure.py`) against the frozen,
independently refuted T4-U2 references, re-frozen on D-5 for T4-U6:

- `validation/references/t4_i7/u2_reference_cases.json` (T4-I7, pressure
  through realized bends; sha256 `4bb9d0fa…ab66`) with its documents
  `u2_document_sketches.json`;
- `validation/references/t4_i8/rebuilt_reference_cases.json` (T4-I8, the
  rebuilt straight cases; sha256 `6f305474…21e0`).

**Status: admitted for development comparison after independent review of the
T4-U6 section-basis mapping.** T4-I7 (the 72 cases with mill tolerance) and
T4-I8 MILLTOL were re-frozen on D-5 (DEL-05-03 Design
`section-bases-and-timoshenko.md` §7). That re-freeze was refuted independently
on 2026-10-16, which found no wrong frozen value and no loosening.
`INDEPENDENT_REVIEW_T4-U6.md` (verdict ADMIT) checked this package's mapping of
the re-frozen references onto rows, evidence, criteria and absences. It reuses
the unchanged warrants of `INDEPENDENT_REVIEW_T4-U4A.md` (the U4a tangent
mapping) and `INDEPENDENT_REVIEW_T4-RV28.md`; T4-RV3 and T4-RV4 refuted the
references. Since T4-U4b/U5b, `INDEPENDENT_REVIEW_T4-U4B-U5B.md` (verdict ADMIT)
checked the epoch-4 delta (pressure-2, model 0.6.0 submitted copies, the arc
maximum no longer a declared absence, the corrected nodal displacement), reusing
the T4-U6, U4a and RV28 warrants. `ADMISSION.json` binds the T4-U4b/U5b review
by SHA-256. Without
`ADMISSION.json`, every generated reference is `pending_independent_review`
and every profile is a draft, so the gate refuses the package. Admission
changes statuses only, never a value or rule. It is a development-comparison
admission, not engineering acceptance or a release criterion. Standard claim
fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Zero-scale floors after the re-freeze.** The T4-I7 floors are
`1e-9*zero_scale(group)`, and their rule is unchanged. A group's zero scale is
still the largest |expected| in that group for the case, or its stated
characteristic scale when the group is identically zero. The re-freeze moved
the values, so the floors moved with them. The stress groups are now split by
basis (`<group>@<basis>`). The T4-U6 review (check 4, "Floor changes") counts
the package floors that have a former floor (the 79 I7 cases on one version;
both versions are identical): of 1,241, 173 rose, by at most ×1.1711
(`effective_axial_force`, `U2-L-ANCH-PTW-K2-SKEW-X7P3E6`), 518 fell, to no
less than ×0.8706, and 550 are unchanged. The per-basis stress groups rose by
at most ×1.112 against the former group, and the MILLTOL floors are the same
or tighter. A higher floor follows from a larger reference value under the
same rule; it is not a relaxed criterion. T4-U4a's re-frozen floors follow the
same rule; this package does not consume them.

All inputs are invented test quantities from the references. None of them is
material-library, component-library or code-rule data.

## Files

| File | Committed | Content |
|---|---|---|
| `generate_package.py` | yes | The generator. It reads the references and the pinned pressure-2 table only. |
| `MANIFEST.json` | yes | `openpipestress.exact_pressure_qualification_manifest/1`: one entry per case, binding each case file by path and sha256. |
| `ADMISSION.json` | yes | `openpipestress.exact_pressure_admission/1`: the admission record, binding the T4-U4b/U5b review by path and sha256. |
| `INDEPENDENT_REVIEW_T4-U4B-U5B.md` | yes | The independent T4-U4b/U5b delta review, bound by `ADMISSION.json`. |
| `INDEPENDENT_REVIEW_T4-U6.md` | yes | The independent T4-U6 section-basis mapping review; it admitted the D-5 package. |
| `INDEPENDENT_REVIEW_T4-U4A.md` | yes | The independent U4a tangent-mapping review; it admitted the pre-D-5 package. |
| `INDEPENDENT_REVIEW_T4-RV28.md` | yes | The preserved historical review of the unchanged basis. |
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

Every case runs in both solver modes.

- **T4-I7.** Each of the 79 valued cases, run on its own document on 0.3.0
  and on 0.4.0.
- **T4-I8.** Each owner with rows. Its v2 document is patched to
  `3.0.0/exact_pressure_v3` by the reference's own `v3_patch_for_both`, and
  runs on 0.3.0 and on 0.4.0. The D-5 re-authored MILLTOL owners are the
  exception. They carry their own model 0.5.0 documents under the current
  contract, run as `@0.5.0` and `@0.5.0_load_state_group` (with the
  load/reference-state group). Both run again under the owner's declared
  `shear_model_variant`, as `@0.5.0_timoshenko` and
  `@0.5.0_load_state_group_timoshenko`. These assert the same rows, values and
  rules: the case has no transverse load, so Timoshenko leaves its axial-only
  response unchanged. Since T4-U4b/U5b the frozen documents and case labels keep 0.5.0; the
  transport submits each as an in-memory 0.6.0 copy changed by version alone,
  preserving its shear model, allowances and load/reference-state group
  (`GATE_USAGE.md`).

Case IDs are `<reference case>@<version>`.

| Family | Cases | Positive | Negative | Absences |
|---|---|---|---|---|
| L-FREE (I7) | 52 | 18428 | 68 | 288 |
| L-ANCH (I7) | 50 | 18016 | 266 | 278 |
| U-ANCH (I7) | 36 | 35232 | 96 | 628 |
| CBPT (I7) | 16 | 1712 | 62 | 96 |
| KINK (I7) | 4 | 1424 | 26 | 24 |
| milltol_lame_membrane (I8, two variants, four documents each) | 8 | 1144 | 124 | 32 |
| tp_phys_pressure_halves (I8) | 2 | 1588 | 22 | 24 |
| pressure_membrane_thin_wall_limit (I8) | 2 | 278 | 8 | 8 |
| v3_straight_twin_separate_closures (I8) | 2 | 294 | 0 | 8 |
| **Total per mode** | **172** | **78116** | **672** | **1386** |

Totals:

- **Per mode:** 172 cases, 78,116 positive and 672 negative assertions.
- **T4-U6 section bases:** every stress row is asserted on both bases. Before
  D-5 the 168 cases had 55,724 positive assertions.
- **U4a tangent mapping:** 2,660 tangent arc end assertions; 1,798 values
  differed from the former chord mapping. Twenty-six formerly unscored arc
  endpoint negative controls score the end rows. These counts are from the
  U4a round's generated selectors and reference files.
- **Terminal evidence:** 2,094 of the positive assertions are terminal-evidence
  selectors.
- **I8 rows:** every row is asserted: 3,304 positive assertions, 720 of them
  in MPa.

**Not in the package:**

- **I7 `U2-L-MITRE-REFUSED-P-K2`:** it publishes nothing. Its refusal is
  checked in `core/product_physics/tests/t4_u2_pressure_references.rs`.
- **I7 polygon-limit convergence record:** it is a study, not case rows.
- **I8 `v3_straight_twin`:** it has no rows of its own. Its SP-1 twin clause is
  checked bit for bit in the same Rust test.

## Mapping

**Row selectors.** These use the first-static shape
`{id, kind, unit, entity_ref, basis_ref, metadata, dimension}`:

- `dimension` and the result family come from the pinned pressure-2 table.
- The row ID and published metadata follow the producer's naming
  (`metadata()` and `row_id()` in the generator). They were identified once
  from producer output.
- A mismatch shows as an unresolved selector in the gate, never as a changed
  value.
- **Section bases (D-5, Design §2.1, §2.5).** Every stress row is asserted
  on both section bases, the nominal selector immediately followed by its
  reduced twin. Each names its basis: the ID's final segment
  (`:nominal_section` on `result:pressure-exact:` and
  `result:elastic-maximum:` IDs, `:nominal-section` on the others) and
  `; section_basis=<basis>` after the recovery token. A re-frozen reference
  supplies each basis's own value. Where a document's bases coincide (no mill
  tolerance, no corrosion allowance), its one frozen value and rule apply to
  both families (Design §7). The generator requires each case's layout to agree
  with its document's sections.

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
  - In a re-frozen case each station's stresses are under
    `stresses_by_section_basis/<basis>`.
  - The section actions are not asserted at the end stations (see end rows).
- **End rows.**
  - Every member's end station action equals its element-local end row: end_i
    negated, end_j as is. The generator checks this equality exactly, so it is
    asserted once, through the end row. The station pointers are recorded as an
    `implied_by_positive` gap.
  - Arc end rows are the reference's `tangent_frame` block, with the
    tangent-frame node-on-element sign texts of U4a Design §4.
  - The wall end action is along the end tangent (`tangent_frame`).
  - Not published, recorded as a `not_published` gap: the element-local axial
    end force, replaced by the pressure family.
- **Lamé values.** Straights only, at all five stations. In a re-frozen case
  `lame_surface` is keyed by basis.
- **Arc absences.** Four kinds are withheld on every arc, each a declared
  absence:
  - the Lamé radial and hoop rows;
  - `element_local_axial_force` and `element_local_axial_normal_stress`.

  T4-U4b: an arc publishes its certified maximum, so the maximum is no
  longer a declared absence. The T4-I7 reference states no arc maximum, so
  it is not scored. The regenerated cases need an independent re-review
  before admission statuses rely on them.
- **Terminals.** Each non-null vector is asserted by component. Each null one
  is a declared absence.
- **Negative controls.** Every listed row of every
  `wrong_result_discriminators` control is asserted under the positive
  assertion's rule.
  - A control at an end-station pointer scores the end row with that
    sign.
  - Identical (selector, wrong value) pairs are scored once.

**T4-I8 mapping:**

- **Rows.** Every row of each load case, with its own criterion and its own
  `reference_origin` pointer. A MILLTOL stress row names its `section_basis`;
  the other owners' bases coincide, so each of their stress rows is asserted
  on both.
- **Absences.** Four kinds are declared absent on every pressurized member:
  `element_local_axial_force`, `element_local_axial_normal_stress`,
  `pipe_section_pressure_hoop_stress` and
  `pipe_section_pressure_longitudinal_stress`.
- **Discriminators.** Each is mapped explicitly (`I8_DISCRIMINATORS`) to the
  quantity it names, in the variant and load case it names. It is scored at the
  first row that carries that quantity; every such row is asserted positively.
  A control that declares its own `target` must name exactly the mapped
  quantities.
- **D-5 controls.** The MILLTOL controls name their section basis (for example
  `sigma_z_reduced_section`). The three renamed controls must carry the
  `former_id` the generator lists (`I8_FORMER_IDS`).
  `cap_area_on_nominal_bore` is active from T4-U6 and is scored. A control
  active only from a later unit would be a `not_active` gap.

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
| T4-I7 | `criterion:<family>:<dimension>:<unit>:floor:<group>` | relative 1e-9 with the group's absolute floor 1e-9*zero_scale, converted to the row unit (the reference's `1e-9*max(|expected|, zero_scale(group))`); a re-frozen case's stress groups are `<group>@<basis>` |
| T4-I8 nonzero | `…:relative_1e-9` | relative 1e-9, absolute 0 |
| T4-I8 zero | `…:zero_scale:<case>:<tag>` | relative 0, absolute 1e-9*zero_scale in the row unit; recomputed and required equal to the frozen row criterion |

## Running the gate (DEC-025)

1. Generate the cases with `--write`.
2. Name an admitted package and a reviewed reader binding,
   `openpipestress.exact_pressure_consistency_binding/2`. The binding lists
   the adapter's three `DEPENDENCIES` (the Rust reader's source, the
   pressure-2 table and the unit authority), and its entry is
   `validate_pressure_evidence`. The verdict is the Rust product reader's,
   taken from the explicitly built `openpipestress-runner read pressure` named
   by `OPENPIPESTRESS_RUNNER_BIN`.
3. Run once per mode.
4. Set `--output-limit-bytes` to 32 MiB (33,554,432). Under D-5 six
   U-ANCH-ALL-K2-SKEW cases (on 0.3.0 and 0.4.0) exceed 16 MiB in both modes,
   and a run over the limit fails closed. The largest CLI output observed is
   17,276,452 bytes (`U2-U-ANCH-ALL-K2-SKEW-X7P3E6@0.4.0`, dense). The
   transport allows at most 64 MiB.

`explicit_local_private_intent` must be `true`. Without it, the runner blocks
the export of the evidence fields.
