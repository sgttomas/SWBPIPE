# T4-RV28: independent admission review of the VP-STATIC exact_pressure_1 package

Reviewer: T4-RV28 (TASK, Type 2), for T4's WORKING_ITEMS. I did not write the
package or the references.

Reviewed: commit `92d53b69e0fc6181c42740086576e41e85c4cc15` (branch
`codex/piping-t4-u2-20261010`). The committed, unadmitted
`validation/qualification/fixtures/exact_pressure/MANIFEST.json` has sha256
`856991e27a9d983544a15e98ab52ac67a6e34cac419f995c76de238183c0185c`.

Paths below are relative to `projects/chirality-piping`.

**Verdict: ADMIT.** The package maps the frozen T4-I7 and T4-I8 references onto
product rows, terminal evidence, criteria and absences correctly. I found
nothing blocking and no required correction. The SHOULD-FIX items are optional
strengthenings. Admission changes statuses only. I checked that with
`ADMISSION.json` present the regenerated selectors, runner inputs and documents
are byte identical, and that reference and criteria files differ only in
`readiness`, `independent_review_ref`, `profile_status` and the rule `review`
text (168 of 168 cases).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This is a
development-comparison admission, not engineering acceptance or a release
criterion.

## Checks and results

1. **Frozen references.**
   - `shasum -c` passes in `t4_i7/` (4/4) and `t4_i8/` (3/3).
   - The archive's full `SHA256SUMS` files at `archive/piping-t4-2026-10-09`
     (commit `f2211ecef6`) hash to `daada386…5527` (T4-I7) and `65bf2f50…5509`
     (T4-I8). Their lines for the committed files are identical to the
     in-repo `SHA256SUMS`.
2. **Every value enters unchanged.** My own script read only the references and
   the generated case files and traced all 55,724 positive and 548 negative
   assertions, not a sample. It mapped each selector's identity (kind, entity,
   location, component, unit) to its reference quantity independently of the
   generator's pointers. Results:
   - **T4-I7.** 53,812 values bit-equal, after an exact decimal unit factor
     where one applies (12,318 converted). Every reference leaf is asserted or
     carries a recorded gap. The straight end-row and end-station identity
     (end_i negated) holds exactly in 3,920 checks.
   - **T4-I8.** 1,912 values come from my own exact evaluator at 150 digits.
     That covers 396 unit-converted rows (Pa to MPa, m to mm) and the 6
     symbolic rows: 4 `a + (b)/pi` and 2 `sqrt((a*pi)^2 + (b)^2)`. Each
     evaluation is bit-equal to the frozen `value` and to the case file.
   - **Zero-scale floors.** 1,164 floors recomputed equal to the frozen row
     criteria.
   - **I7 floor groups.** All 53,812 I7 rules equal 1e-9·zero_scale converted
     to the row unit. That value also equals the reference's written
     `absolute_floor`. The group assignments match the reference generator's
     `GROUP_OF` and `end_group`.
   - **Documents.** Every runner input carries its reference document:
     - I7: the sketch, unchanged.
     - I8: the v2 document with `v3_patch_for_both` applied.
3. **Selectors address the product rows the reference means.** I built
   `openpipestress-runner` (release, `--locked --offline`) at the reviewed
   commit and ran all 168 cases in both modes (336 runs). My own resolver works
   as follows:
   - It finds each row by (kind, entity, basis, location, component).
   - It requires exactly one match, and that match must carry the selector's
     ID, its full metadata and its unit.
   - It compares the row with |o−e| ≤ max(floor, 1e-9·|e|).

   Results:
   - All 111,448 positive assertions match. The worst case is 0.0037 of
     tolerance.
   - All 1,096 negative assertions are excluded.
   - All 3,272 declared absences hold, and no unit-blind assertion was found.

   Discrimination:
   - 78,056 positive rows are nonzero beyond their floor, so their sign
     discriminates.
   - Of 5,320 arc chord-frame end-row checks, 3,596 differ from the
     tangent-frame value beyond tolerance, so the frame is discriminated. The
     other 1,724 are coincident components, mostly zero.
   - The wall end action on arcs matches the tangent-frame value.
4. **Criteria.**
   - Relative criteria are 1e-9 throughout.
   - I8 zero rows use relative 0 with the frozen floor. No other tolerance
     appears.
   - The pinned classifier uses max(|o|,|e|) rather than |e|. The difference is
     below 1e-18 relative and is not a loosening in practice.
   - Every I7 control scores at least one row. 26 control rows at arc
     end-station pointers are recorded `not_published`.
   - Every negative lies outside its rule.
   - The I8 discriminator mapping (`I8_DISCRIMINATORS`) agrees with each
     control's own `quantity`, `case` and variant in all 28 placements.
5. **Absences.**
   - **I7 arcs:** exactly Lamé radial and hoop, the elastic maximum,
     `element_local_axial_force` and `element_local_axial_normal_stress`.
   - **I7 terminals:** null terminal fields exactly as in the reference.
   - **I8 pressurized members:** exactly the reference's
     `structural_expectations` kinds.
   - Nothing declared absent is published in any run, and nothing the
     reference values is declared absent. This agrees with the product's
     removal rule (`core/product_physics/src/bend_pressure.rs` 84–92 and
     `src/lib.rs` 11793–11801) and the consumer test
     `core/product_physics/tests/t4_u2_pressure_references.rs` 229–245.
6. **Exclusions are justified.**
   - **Mitre control:** the CLI returns `MODEL_INCOMPLETE` with
     `PRESSURE_REGION_MITRE_UNSUPPORTED` and no rows. The adapter cannot score
     a refusal, and the Rust test checks it.
   - **Polygon-limit record:** a study, not rows.
   - **I8 `v3_straight_twin`:** no rows of its own. Its twin is checked by
     `i8_v3_twins_are_bit_equal_to_v2`.
   - **`cap_area_on_nominal_bore`:** `producible_today: false` until T4-U6.
7. **Refusals.**
   - **`generate_package.admission()`.** It refuses each of these: an extra
     or missing key, the wrong format or status, extra review keys, an
     absolute or `..` path, a wrong hash and a missing file.
   - **Adapter.** I ran 24 refusal probes of my own: manifest shape, duplicate
     IDs, modes, counts, binding shape, selector file, other contract,
     duplicate absences or negatives, hash tamper, draft profile, pending
     reference, missing review reference, value denominator, unit, missing
     rule, dimension contradiction, foreign absence basis and negative
     tolerance. All were refused for the stated reason.
   - **Package tests.** `tests/test_qualification_exact_pressure.py` passes:
     23 tests.
8. **Regenerable.**
   - `generate_package.py --write` then `--check` reports 841 files and 0
     differences.
   - The committed `MANIFEST.json` is unchanged by `--write`.

## Findings

No BLOCKING findings.

**SHOULD-FIX (optional, not required for admission)**

- **S-1** `generate_package.py:420`, `:499`.
  - **Gap.** I7 declares no absence for kinds the product withholds on every
    pressurized member and I8 declares (I8's `structural_expectations`):
    - on I7 straights: `element_local_axial_force`,
      `element_local_axial_normal_stress`,
      `pipe_section_pressure_hoop_stress` and
      `pipe_section_pressure_longitudinal_stress`;
    - on I7 arcs: the two `pipe_section_pressure_*` kinds.
  - **Effect.** None of these is published in any of the 336 runs, so
    declaring them would pass. Leaving them out only means an I7 run would
    not catch their return.
- **S-2** `README.md:79`, selector `gaps`. Some published rows are neither
  asserted nor recorded as a gap, and the README does not name them:
  - **I7** (no reference value): the straight
    `pipe_elastic_normal_stress_maximum_v2`, the support force and moment
    magnitudes, `displacement_magnitude`, `curved_bend_macro_element_review`
    and the parity row;
  - **I8:** the non-null terminal evidence fields (I8 values none).

  Fix: add a `not_scored` gap or a README line.
- **S-3** `generate_package.py:548`, `:643`. The generator ignores each I8
  control's own `quantity` field.
  - **Today.** The mapping agrees with every control's `quantity`.
  - **Risk.** A T4-U6 or T4-U8 re-freeze could silently desynchronize them.
  - **Fix.** Assert that the mapped expected key agrees with
    `control['quantity']`.

**NOTE**

- **N-1** `generate_package.py:697–713`. `ADMISSION.json` binds only the review
  by hash. Its closed four-key shape has no field for the reviewed commit or
  `MANIFEST.json` digest, so the draft states both in `admitted_by`.
  - The review path is not checked through `resolve()` for symlinks. The risk
    is low.
  - With `ADMISSION.json` present, the regenerated `MANIFEST.json` changes,
    with status fields only. Commit `ADMISSION.json`, the review and the
    regenerated manifest together.
- **N-2** `tools/validation/qualification_exact_pressure.py:403–410`. A
  criteria file loosened and re-hashed into the manifest would be admitted
  by `prepare_case`. This is the same design as `load_reference`: the binding
  rests on the committed `MANIFEST.json`, `--check` and the package test.
- **N-3.** The package's unit tests create throwaway git repositories in
  temporary directories. They are not part of the repository.
