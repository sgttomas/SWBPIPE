# RETURN — T1_WP6_FREEZE (independent freeze check of the VP-STATIC load/reference package)

- **Role:** TASK (Type 2), a fresh-context, non-author checker. I wrote none of the package and none of the analytical references. I did not delegate.
- **Requested by:** the T1 WORKING_ITEMS manager. ROOT spawned me.
- **Checkout:** branch `codex/piping-load-states-20260925` at `fb6ad29a7963075d2ec1bd8a11e241959ac6ebf6`. The package tree at HEAD is byte-identical to `91ec30630`, and the working tree is clean for it.
- **Git:** no writes. **Package:** not edited. I wrote only inside `LSI/T1_WP6_FREEZE/`.
- **Not run:** the product, the runner, the producer example and every comparison. I read no observed value. I did read product source in two places, only to confirm field semantics: `reference_length_m` is the geometric member length, and the local z axis is x × y.
- **Paths:** WORKING_ROOT-relative. `LSI` is this folder's parent.

## 0. Verdict

**FREEZE (admit as changed).**

- The package represents its references faithfully.
- All 467 positive and 40 negative values reproduce **bitwise** from an independent evaluation of `exact`.
- All 148 equilibrium zeros are implied by the topology. **None is struck.**
- One criterion is looser than the protected product test for the same quantity. Change **R1** fixes it, and it is mandatory.
- Two README statements are inaccurate or incomplete. Changes **R2** and **R3** fix them. They do not affect scoring and change no hash.
- After R1, no criterion is looser than `load_reference_state_runtime.rs` `close()`.

**What admission still needs.** Admission itself is still a separate step: `readiness`, `independent_review_ref` and `profile_status` stay pending until the manager's chosen admission path rewrites them (WP5 RETURN §7 Q2). See §6, Q5.

## 1. Inputs reviewed (sha256)

| File | sha256 |
|---|---|
| `validation/qualification/fixtures/load_reference/MANIFEST.json` | `324afa50f0d11898eef0fb50d0e68d666edf119085e82d4af020a911dab21f0e` |
| `…/load_reference/PROVENANCE.json` | `c5a0cee9671db51f9a273e5238c731ce74b269653f0b49d80b5a9b9bc77e865a` |
| `…/load_reference/README.md` | `c73a4f063669e8f48761cb411716c1d3680b19e3e42e7f31de2d96e42ca20542` |
| `…/load_reference/generate_reference_values.py` | `272864e80a4f8a18a5bd96844b0dea2fce9aa9d8f0d3ec21b4309ef59690d1fb` |
| 14 × 5 case files | bound by `MANIFEST.json`; all match (check_package group A) |
| `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json` | `4d7b777708806c211cda0e126d7cd4eb7acba9b658b1da63cca36fd0523beeab` |
| `core/product_physics/tests/fixtures/load_reference_states/README.md` | `fe8149c0545453275e012af4276e7808bd9179c4accac866e0681d38da0979fb` |
| `core/product_physics/tests/load_reference_state_runtime.rs` (protected oracle) | `d5be0bd812a2af51c1a32fc5c237a42b76b460ae75d0a610837f01b676f6960b` |
| `fixtures/results/semantic_contract_v0_3_load_reference_1.json` | `44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d` |
| `LSI/T1_WP6_STATIC_CASES/RETURN.md` | `b9516541dd21feb6bc134621bffd651435ac76e13cceca72dc195d47e538737d` |

I also read these, for context only:

- `LSI/T1_WP5_HARNESS_ADAPTER/RETURN.md` §2–3;
- `CP2_WIRE_ADDENDUM_1.md` §3;
- `CP3_WIRE_ADDENDUM.md`;
- `T1_PLAN.md` WP6;
- `T1_WAVE1_RULINGS.md`.

## 2. Per-case decision table

Families are abbreviated as follows:

- **D**: displacement or rotation rows;
- **F**: element force rows;
- **R**: reference-defined reactions;
- **EQZ**: equilibrium-zero reactions;
- **Ev**: evidence fields;
- **Neg**: negative assertions;
- **Gap**: gaps.

Each count is positives / negatives.

| Case | Load cases | Assertions (pos / neg) | Decision | Notes |
|---|---|---|---|---|
| `prescribed_translation_two_bar` | 1 | 27 / 1 | **ADMIT** (all families) | Represents control 1 on the annulus. Middle UX = k-weighted average, reproduced from first principles. End_i = −N. The omitted_Kfc_gc negative uses the protected scale root_UX. |
| `prescribed_translation_all_fixed` | 1 | 21 / 0 | **ADMIT** | Zero free DOFs. The text discriminator is implied by the nonzero Fx and N. |
| `prescribed_rotation_all_fixed` | 1 | 24 / 0 | **ADMIT** | `[6EIθ/L², 4EIθ/L, −6EIθ/L², 2EIθ/L]` reproduced from E·I(annulus). The element end actions equal support-on-member, because the local frame is global (y_ref = +Y, z = x × y). |
| `prescribed_rotation_free_tip` | 1 | 17 / 3 | **ADMIT** | Tip UY = Lθ, RZ in rad. The zero scales are the all-fixed actions, matching the protected precedent. All three negatives lie well outside tolerance. |
| `shared_material_serial_companion` | 1 | 35 / 1 | **ADMIT** | Explicit strain, not an inferred temperature, as the reference limits require. Per-member exact_point. G = E/(2(1+ν)). The wrong middle UX (−0.5 mm) is reproduced for any single E. |
| `thermal_datum_ratio.fixed` | 3 | 75 / 6 | **ADMIT** | Secant ratio 43/25009 reproduced. Datum stretches are `1+dilation`. T_K values are correct. The kelvin load case authors 423.15 K. |
| `thermal_datum_ratio.free` | 3 | 60 / 6 | **ADMIT** | Tip = Lε. The zero scales use the fixed wall force, as the protected test does. |
| `coefficient_definition` | 4 | 8 / 4 | **ADMIT** | The 70 degC midpoint is implied: I(20→70) = 1/1600, checked. Each installation temperature is its own member. |
| `constant_alpha_interval` | 2 | 4 / 0 | **ADMIT** | K and degC intervals. The null-temperature discriminator is an honest structural gap. |
| `multi_segment_free_length` | 6 | 42 / 12 | **ADMIT** | Piecewise trapezoid and dilation values were reproduced from first principles. The numerically-identical dilation discriminator was checked: 1/2001 both ways. |
| `temperature_unit_identity` | 5 | 9 / 1 | **ADMIT** | degF and degR identities are exact. The non-equal control is separated from 223.15 K by 4.48 tolerances; the reference designs it that way. |
| `signed_fit_states.fixed` | 3 | 66 / 4 | **ADMIT WITH CHANGES (R1)** | The cold/return `thermal_strain` zero rule is looser than the protected test (§3). Everything else, including the four As/area-scaled negatives, is admitted. |
| `signed_fit_states.released` | 3 | 51 / 0 | **ADMIT WITH CHANGES (R1)** | Same R1 item. Everything else is admitted. |
| `persistent_source_once` | 2 | 28 / 2 | **ADMIT** | N = F for an axial cantilever. Root Fx = −F. u = F/k. The preload as an ordinary primitive is an honest representability limit, since the reference calls it an equivalent affine tip action. |
| **Package** | 36 | 467 / 40 | **R2, R3** (README) | 22 gaps are honest (§5.7). Some reference quantities are neither asserted nor listed: R3 records them. |

**Struck assertions: none.**

## 3. Exact change list

### R1 (mandatory; criterion)

**Why.** The zero rule for the cold and return `thermal_strain` evidence in both signed-fit cases is looser than the protected oracle for the same quantity:

- the package uses absolute `8e-13`, which is 1e-9 × |hot thermal_strain| = 0.0008;
- `load_reference_state_runtime.rs::signed_fit_cold_hot_return_fixed_and_released` uses `strain_scale = |hot total_eigenstrain|` = 0.00059984, which gives `5.9984e-13`.

The brief forbids a criterion looser than the protected tests.

**Selector edits.** Make these edits in each of `signed_fit_states.fixed.selectors.candidate.json` (assertions 18 and 62) and `signed_fit_states.released.selectors.candidate.json` (assertions 13 and 47). The assertion ids are `case:cold.evidence.member.pipe:fit.thermal_strain` and `case:return.evidence.member.pipe:fit.thermal_strain`.

| JSON pointer | Old | New |
|---|---|---|
| `/assertions/{i}/criterion_rule_id` | `criterion:load_reference_strain:dimensionless:1:zero_scale:hot_thermal_strain` | `criterion:load_reference_strain:dimensionless:1:zero_scale:hot_total_eigenstrain` |
| `/assertions/{i}/selector_origin/reference_origin/zero_scale/tag` | `hot_thermal_strain` | `hot_total_eigenstrain` |
| `/assertions/{i}/selector_origin/reference_origin/zero_scale/base/pointer` | `/cases/signed_fit_states/variants/annular_companion/expected/hot/thermal_strain` | `/cases/signed_fit_states/variants/annular_companion/expected/hot/total_eigenstrain` |

**Then:**

1. Run `generate_reference_values.py --write`. The criteria rule becomes `…:zero_scale:hot_total_eigenstrain` with `absolute_tolerance_value` `5.9984e-13`, and the old rule disappears. The reference files are unchanged.
2. Update the four sha256 values in `MANIFEST.json` and the two in `PROVENANCE.json` `generator_outputs`.

**Resulting sha256** (verified on a scratch copy; `_run_records/R1_verification.log`, `_run_records/required_change_R1.diff`):

| File | Old | New |
|---|---|---|
| `signed_fit_states.fixed.selectors.candidate.json` | `bb48707f…09151a2` | `bc58af5bd8835d0f1cbd54a2b306b258e89dc16a0751c516d1bd822817779cd2` |
| `signed_fit_states.fixed.criteria.candidate.json` | `ed216050…945d3e35` | `95121e0ea7558541d97087368797e6529e7e649fafdd12bbaca7c1fba4bfe42d` |
| `signed_fit_states.released.selectors.candidate.json` | `4e2aec6f…283104` | `2af2fea95ac12f968fa390ef4f96ee37c8a763edd6fde044795cd11a7fef5373` |
| `signed_fit_states.released.criteria.candidate.json` | `77086fc7…4030af` | `1ecfc1972d76ed6b4164c44830954914777c7585eba93e536f3e799dd2ba1c2f` |
| `MANIFEST.json` | `324afa50…21f0e` | `a70eed580e03ed4b7c43cf9a50b8eb29a7de17eafa3e8046e8d50f3fff04ae9b` |
| `PROVENANCE.json` | `c5a0cee9…e865a` | `5f766002f0597873cb056da5fa20e42c08c25c7f0c73fd76a47a427408c99a91` |

**Checks after the change** (scratch copy):

- generator `--check`: 28 files, 0 differences;
- `check_package.py` no-producer mode: 4,489 passed, 0 failed;
- `independent_recompute.py`: 3,154 passed, 0 failed.

`_run_records/apply_R1.py` performs the selector edit.

**The authoring record needs the same change.** In `LSI/T1_WP6_STATIC_CASES/_run_records/author_package.py` line 1020, change

`strain_zero = scale('hot_thermal_strain', ann(c, 'expected/hot/thermal_strain'), '1')`

to

`strain_zero = scale('hot_total_eigenstrain', ann(c, 'expected/hot/total_eigenstrain'), '1')`.

Without this, the author's claim that "the full authoring pipeline reproduces every package JSON byte for byte" goes stale. Its hash in `PROVENANCE.json` `authoring_records` and in `file_hashes.txt` would then also change. The manager decides whether to edit that record or to note that it is superseded.

### R2 (mandatory; README accuracy, no hash impact)

**File:** `validation/qualification/fixtures/load_reference/README.md`, lines 325–329. The README is hand-written and is not bound by MANIFEST or PROVENANCE.

**Old:**

```
- **Internal quantities.** These are not published and are recorded as gaps:
  - the reduced free right-hand side;
  - the intermediate source sums;
  - the logarithmic datum stretches `exp(I)`;
  - the consumed segment records.
```

**New:**

```
- **Unasserted quantities.**
  - Not published, recorded as gaps: the reduced free right-hand side and the
    intermediate source sums.
  - Published but not asserted: the logarithmic datum stretches `exp(I)`
    (`installation_datum_stretch`/`operating_datum_stretch` of a
    `logarithmic_per_current_length` member). The reference gives no value for
    them; this is a scoring limit recorded in the case's `scoring_readiness.limits`.
  - Published structural records, not scalar-scored: the consumed segment
    records (`consumed_law_segments`), recorded as a gap.
```

**Reason.**

- The datum stretches of a logarithmic member are published fields (CP2_WIRE_ADDENDUM_1 §3; CP3_WIRE_ADDENDUM "published datum stretches"), so "not published" is wrong.
- They also have no `gaps` entry; they appear only as a readiness limit.
- The consumed segment records are published structurally.

### R3 (mandatory; README completeness, no hash impact)

**File:** same README. Insert after line 330, the "Signed-fit baselines" bullet.

**New text:**

```
- **Reference quantities neither asserted nor listed as gaps.**
  - `thermal_datum_ratio/variants/verification_two_point/expected/required_coverage_low_K`,
    `required_coverage_high_K` and `minimum_datum_stretch_over_required_interval`:
    admissibility quantities of the ROOT coverage policy. No scalar field
    publishes them. The two-point load case solving without a coverage
    diagnostic is the witness, and the minimum stretch (1.00036) equals the
    asserted `installation_datum_stretch`.
  - `coefficient_definition/.../expected/integral`, `first_half_integral`,
    `second_half_integral`: intermediate integrals, pinned by the eight
    asserted strains.
  - `multi_segment_free_length/variants/linear_coefficient_table/expected/coefficient_install`,
    `coefficient_split`, `coefficient_operating`, `integral_install_to_operating`:
    interpolation samples and an intermediate integral, pinned by the asserted
    strains and datum stretches.
  - `signed_fit_states/variants/annular_companion/baselines` (listed as a gap)
    are other member-reference inputs; see the gap entry.
  - The `generic_reviewed` variants of the annular cases are not authored:
    the product takes OD and wall, not a generic area (T1_PLAN WP6 uses the
    annular companions).
```

**Reason.** A systematic sweep found these quantities uncovered by any assertion, negative or gap. The sweep compared every `expected`/`baselines` leaf of every used variant with every selector pointer and gap pointer. The package README's gap definition implies that every unscored expectation is recorded. None of these is a hidden required result, because each is intermediate, policy-level or implied by a positive assertion. But the record should say so.

**Optional.** For machine-readable completeness, the manager may later mirror R3 as `gaps` entries, which changes selector hashes. That is not required for the freeze.

## 4. Independent recomputations

**Method** (`_run_records/independent_recompute.py`):

- It does not import or call the generator.
- `exact` is evaluated with `fractions.Fraction`:
  - pi by Machin's formula on integers, as a rigorous interval;
  - `exp(p/q)` by an exact rational Taylor series with a remainder bound.
- The declared transform is applied by my own reading of the sign conventions.
- The exact m→mm factor is applied.
- The result is rounded once to binary64, and both interval ends must round to the same double.

**Results** (`independent_recompute.log`):

- **Every one** of the 467 positive and 40 negative values equals the package value **bitwise**. That is at least one per case, as the brief requires, and in fact all of them.
- Every zero-rule absolute equals 1e-9 × its named scale, bitwise.
- Every nonzero rule is relative 1e-9 with absolute 0.
- Every zero rule is relative 0 with an absolute > 0.
- Each rule matches its selector's unit and dimension.
- **First-principles cross-checks.** 44 checks from the reference inputs and physics, independent of the `exact` fields, all hold. They cover:
  - As and I;
  - the two-bar solution;
  - the rotation stiffness actions;
  - serial N and G, and the single-E wrong value;
  - the secant ratio and the fixed and free mechanics;
  - the trapezoid integrals, including the implied 70 degC midpoint;
  - the multi-segment integrals, dilations and endpoint wrong value;
  - the degF and degR identities;
  - fit ε*, N and tip;
  - persistent F, u = F/k and root = −F.
- **Total:** 3,154 checks passed, 0 failed.

**Sensitivity.** `checker_mutations.py` seeded nine faults into a scratch copy, and my checker killed all nine (`checker_mutations.log`):

- +1 ulp;
- negate dropped;
- mm factor dropped;
- one_plus dropped;
- area ratio dropped;
- an equilibrium zero made nonzero;
- a zero absolute ×10;
- relative relaxed to 1e-8;
- a negative equal to its reference.

**Representative rows:**

| Case | Assertion | Reference `exact` | Transform | Independent value | Package |
|---|---|---|---|---|---|
| two_bar | middle UX | middle_UX = 1/15000 m | identity ×1000 | 0.06666666666666667 mm | bitwise equal |
| two_bar | pipe:1 end_i | member1_N = −38000/3·π N | negate | 39793.50694547071 N | bitwise equal |
| all_fixed | root Fx | root_Fx = 38000·π N | identity | 119380.52083641215 N | bitwise equal |
| rotation_all_fixed | root Mz | root_Mz = 3439·π N·m | identity | 10803.9371356953 N·m | bitwise equal |
| rotation_free_tip | tip UY | tip_UY = 1/500 m | identity ×1000 | 2.0 mm | bitwise equal |
| serial | pipe:1 derived G | G1 = 10¹²/13 Pa | identity | 76923076923.07692 Pa | bitwise equal |
| thermal.fixed | two-point installation datum stretch | dilation_install = 9/25000 | one_plus | 1.00036 | bitwise equal |
| thermal.fixed | root Fx | fixed_root_Fx = 16340000000/25009·π N | identity | 2052606.02021901 N | bitwise equal |
| thermal.free | kelvin tip UX | free_tip_UX = 43/25009 m | identity ×1000 | 1.7193810228317805 mm | bitwise equal |
| coefficient | current-length reverse | exp(−3/2000)−1 | identity | −0.0014988755622891258 | bitwise equal |
| constant_alpha | degC-interval stretch | 1251/1250 | identity | 1.0008 | bitwise equal |
| multi_segment | split operating stretch (load case b) | integral_datum_to_install = 1/1600 | one_plus | 1.000625 | bitwise equal |
| multi_segment | negative: interior breakpoints skipped | exp(7/2000)−1 | identity | 0.003506132152090317 | bitwise equal |
| unit_identity | degF operating T | 10303/20 K | identity | 515.15 K | bitwise equal |
| fit.fixed | hot end_i | hot fixed_wall_N = −854772/5·π N | negate | 537069.0871388509 N | bitwise equal |
| fit.fixed | negative: hot uses cold E | −119968 N × As/area | generic_area_to_annulus | −716092.1161851346 N | bitwise equal |
| fit.released | hot tip UX | 3749/625000 m | identity ×1000 | 5.9984 mm | bitwise equal |
| persistent | combined tip UX | −7/3800000/π m | identity ×1000 | −0.0005863603166543512 mm | bitwise equal |

**Generator reproduction.** `generate_reference_values.py --check` gives 28 files, 0 differences (`generator_check.log`).

**Package self-check.** `check_package.py` in no-producer mode passes groups A–D: 4,489 passed, 0 failed (`check_package_no_producer.log`).

## 5. Findings by brief item

### 5.1 Representation

Every request is a 0.4.0 model with `exact_straight_pressure_v2`, no pressure regions, no components and no combinations.

- **Sections.** Every pipe is OD 0.2 m / wall 0.01 m, matching the reference geometry. The product's As and I equal the reference's.
- **Orientation.** Every pipe runs along +X with y_reference +Y.
- **Lengths.** They match the reference: 1 m, 1 + 1 m, 2 m (rotation) and 10 m (fit).
- **Supports.** Rigid six-DOF anchors. The released and free topologies have the root only.
- **Units and signs:**
  - the prescribed UX, UY and RZ carry reference values and units, with meaning `absolute_reference_displacement`;
  - persistent tip forces are `global_x` with signed magnitudes;
  - the fit is `natural_length_change` −0.002 m (cut short);
  - interval units: 1/K with K, and 1/degC with degC.
- **Temperatures:**
  - thermal: datum 20 degC, installation 50 degC, operating 150 degC or 423.15 K;
  - coefficient: datum 20; installations 20, 120 and 70; operating temperatures as the intervals require;
  - multi-segment: datum 300 K; installations 350, 550 and 450 K;
  - fit: installation 20 degC, hot 100 degC, and the cold and hot points at those temperatures, so E is 200 or 150 GPa by exact_point.
- **Runner inputs.** `solve.preview_model` equals the request in all 14.

**Invented inputs, and whether each changes what the reference asserts:**

| Input | Effect on the asserted quantities |
|---|---|
| ν = 0.3 (all members except the serial ones) | None. Axial and in-plane Euler–Bernoulli results only. This depends on the product being Euler–Bernoulli, which the protected test shares (§6, Q6). |
| Serial base E 170 GPa / ν 0.28 | None. Selected by no member. It makes a base-property fallback detectable. |
| Serial point temperatures 20 and 300 degC | None. Exact_point selection does not use them, and no temperature is asserted there. |
| Strain-only carriers (1 m cantilevers, E 200 GPa, y offsets 0, 1, 2 m) | None. Strain and temperature evidence only. Being offset in y, they are not coincident members. |
| 70 degC installation of `pipe:install-mid` | None. Implied by the reference's half integrals; verified. |
| Thermal kelvin operating temperature 423.15 K | None. It is the reference's `operating_temperature_K`. |
| Fit point temperatures = the reference installation and hot temperatures | None. They select the reference E. |
| Persistent preload as an ordinary primitive | Classification only (`ordinary_applied`, not `affine_reference`). Not asserted; recorded as a representability gap. |
| `source:action-a/-b` | None. They are the reference's numeric_deduplication discriminator. |
| y_reference (0, 1, 0) | Not listed in PROVENANCE. It fixes local axes = global axes, which the rotation element-end selectors rely on. Correct as authored. |

### 5.2 Reference values

These reproduce by generator and independently (§4). No difference was found.

### 5.3 Selectors

- **end_i axial = −N: confirmed.** The row metadata says "element-local DOF at the i-end force vector", and the interior stations use the j-side section action. The protected `member_axial` applies the same end_i = −N, end_j = +N.
- **Fixed-rotation element end actions = support-on-member vector: confirmed.** They are K·u for u = (0, θ, 0, 0), in a local frame equal to the global frame (y_ref = +Y, z = x × y in the frame kernel).
- **N = `combined_rhs`: confirmed.** A tip force F on an axial cantilever gives N = F, so end_i = −F.
- **Pair root Fx = −`two_distinct_equal_actions_rhs`: confirmed.** Support-on-pipe, so root Fx = −F.
- **Datum stretch = 1 + dilation or 1 + integral: confirmed** for:
  - `engineering_secant`, where λ = 1 + α_sec(T)(T − T_m);
  - `differential_per_datum_length`, where λ = 1 + I(T_m, T);
  - `engineering_dilation`, where λ = 1 + d(T) and d(T_m) = 0.

  No logarithmic stretch is asserted.
- **Other checks:**
  - `reference_length_m` = the geometric length L = 10 m, confirmed from the product's resolver semantics;
  - wall_N is mapped to `element_local_axial_force` without a pressure region, as the protected test does;
  - reactions are support-on-pipe, as in the reference convention;
  - displacements are in mm, rotations in rad;
  - every evidence `definition` constraint matches the authored request: law id, definition, selection kind, basis and fit kind.

### 5.4 Equilibrium zeros (148)

**All ADMIT; none struck.** Each case is a linear, small-displacement straight member on global X, with no pressure, weight or transverse load.

| Case | Zeros | Components | Why they are zero |
|---|---|---|---|
| Translation, two-bar | 10 | Fy, Fz, Mx, My, Mz | Axial-only actions |
| Translation, all fixed | 10 | Fy, Fz, Mx, My, Mz | Axial-only actions |
| Serial | 10 | Fy, Fz, Mx, My, Mz | Axial-only actions |
| Thermal fixed | 30 | Fy, Fz, Mx, My, Mz | Axial-only actions |
| Fit fixed | 30 | Fy, Fz, Mx, My, Mz | Axial-only actions |
| Persistent | 10 | Fy, Fz, Mx, My, Mz | Axial-only actions |
| Rotation, all fixed | 8 | Fx, Fz, Mx, My | In-plane XY bending decouples axial, out-of-plane and torsion |
| Rotation, free tip | 4 | Fx, Fz, Mx, My | A stress-free rigid rotation |
| Thermal free | 18 | all six | A self-equilibrated eigenstrain on a free cantilever |
| Fit released | 18 | all six | A self-equilibrated eigenstrain on a released cantilever |

The protected tests assert the same zero components, where they test them, at the same scales.

### 5.5 Criteria

- **Nonzero references:** relative 1e-9, absolute 0. This is the README consumer rule under the ruled symmetric classifier; see §6, Q3.
- **Zero references:** 1e-9 × a named, same-dimension reference magnitude. Compared quantity by quantity with `close()`:
  - **Same scale as the protected test:**
    - two-bar and all-fixed forces, moments (F × span) and the root_UX length;
    - rotation all-fixed force, moment (|root_Mz|) and θ;
    - free-tip all-fixed force and moment, and the tip UY length for the missing-coupling negative;
    - serial force, moment and eps2;
    - thermal fixed-wall force for both topologies;
    - fit released per-state fixed N;
    - persistent |root Fx|.
  - **Fit fixed equilibrium zeros:** each state's own |fixed N|. The protected fit test asserts no fixed-topology zero reaction; its only fit force scale is |hot N|, which is at least as large, so these are not looser.
  - **No protected analogue**, and the natural same-dimension magnitude:
    - prescribed-value evidence zeros;
    - rotation UY zeros at θ·L;
    - serial root and far UX at |middle UX|;
    - moment zeros at F·L for thermal, fit and persistent.
  - **Looser than the protected test:** only the fit cold/return `thermal_strain`. R1 fixes it.

### 5.6 Negatives

- All 40 are the reference discriminators, correctly transformed. The fit discriminators are scaled by As/area, which is exact because N is linear in the area; the protected test uses the same transform.
- Each shares its positive's selector.
- Each lies outside both the negative rule around the wrong value and the positive rule around the correct value.
- **Smallest separation:** 4.48 tolerances, for the non-equal temperature control. The difference is the reference's 1e-6 K, about 4.5e-9 relative. The next smallest is 2.7e5.
- **Two derived negatives are correct:**
  - the non-equal control's negative against 223.15 K comes from the reference `limits`, not from `wrong_result_discriminators`;
  - the naive-sum reaction is `negate` of `naive_total_sum_rhs`.

### 5.7 Gaps and exclusions

- **Excluding `shared_material_parallel` is right.** The reference marks it "analytical topology only; public/native admissibility must be established separately". T1_PLAN names the serial companion as the practical witness.
- **The 22 gaps are honest:**
  - `not_published`: the free RHS and the intermediate sums;
  - `implied_by_positive`: three text discriminators, each backed by the listed positives;
  - `structural_expectation_not_scored`: nulls, point ids, the implicit-array absence and the segment records;
  - two `refusal_control` mutations, with codes that the protected tests also expect;
  - `not_representable_under_criterion`: the binary64 discriminator, whose ~1e-16 relative difference is below 1e-9;
  - `numerically_identical_discriminator`: 1/2001 both ways;
  - `representability_limit`: the preload classification;
  - `not_authored`: the fit baselines.
- **None hides a required result quantity.**
- R2 corrects one README misstatement, and R3 records the quantities that were silently omitted.

## 6. Open questions for the manager

1. **Signed-fit baselines** (`not_authored`).
   - The product can represent them:
     - a no-fit request (fit `none`) for cold, hot and return;
     - a cut-long request (+0.002 m, the opposite signed change the reference implies) for cold and return.
   - The protected test covers them (`signed_fit_baselines_cut_long_and_direct_fit_strain`).
   - The omission is a scope choice, not a representability limit. It weakens nothing in the admitted set, because the fit strain and fixed N assertions already exclude "fit ignored".
   - *Recommendation:* author `signed_fit_states.baselines` in a follow-up pass with its own freeze, or record the scope choice explicitly. This is not a freeze blocker.
2. **Optional coverage in `coefficient_definition`.** Its datum-length members could also assert the published datum stretches via `one_plus` of `integral` and `first_half_integral`, as `multi_segment_free_length` does. This is optional and not required.
3. **Symmetric classifier (ruled).** `max(|obs|,|exp|)` admits at most about 1e-18·|exp| beyond `1e-9·|exp|`. That is below one binary64 ulp, so it cannot materially loosen any rule. I accept the manager's ruling. I record it because the brief's "no criterion looser" test is strictly met only up to this window.
4. **Authoring-record drift after R1.** `author_package.py` line 1020, its PROVENANCE `authoring_records` hash and `file_hashes.txt` (§3).
5. **Admission mechanics.** After R1–R3, the candidate files still carry these values:
   - `readiness: pending_independent_review`;
   - `independent_review_ref: null`;
   - `profile_status: draft_pending_independent_review`.

   WP5 scores only reviewed and ready files. Whichever admission path the manager chooses will change hashes again. It should cite this RETURN as the independent review.
6. **Euler–Bernoulli dependence.** The rotation-case values are Euler–Bernoulli. Invented ν matters only if the product ever adds shear deformation. The protected test has the same dependence.

## 7. Files written (all new; no existing file modified)

All in `LSI/T1_WP6_FREEZE/`:

| File | Content |
|---|---|
| `RETURN.md` | This return |
| `_run_records/independent_recompute.py` | Independent evaluator and cross-checks |
| `_run_records/independent_recompute.log` | 3,154 passed, 0 failed |
| `_run_records/checker_mutations.py`, `.log` | 9 of 9 seeded faults killed (scratch only) |
| `_run_records/generator_check.log` | Generator `--check`: 28 files, 0 differences |
| `_run_records/check_package_no_producer.log` | 4,489 passed, 0 failed |
| `_run_records/apply_R1.py` | R1 selector edit |
| `_run_records/R1_verification.log` | R1 applied on scratch; new hashes; all checks pass |
| `_run_records/required_change_R1.diff` | Exact R1 byte diff, WORKING_ROOT-relative |

The scratch space was deleted. No cargo target was used.
