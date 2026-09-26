# CP2 wire addendum 1

This addendum supplements `CP2_WIRE.md` (sha256
`81a7adbaa212517c518a61c5ab54cd7d9444adaee6b7f0dbec8b7b6efd168996`), which is
unchanged. The facts below were given only by message during checkpoint 2, or
the runtime-test TASK (`CP2_RUNTIME_TESTS/RETURN.md`) reported them as gaps.
Downstream readers, schemas and authoring surfaces must use this addendum
together with `CP2_WIRE.md`. Paths are WORKING_ROOT-relative.

## 1. Rejection channels

Two channels exist.

**Typed-boundary rejection.** `run_linear_static_preview_value_with_mode`
returns `Err` and no envelope is produced when any of these occurs in
`analysis_state`, `reference_configurations` or a material's `expansion_laws`:

- an unknown field in any object;
- a missing required field;
- an unknown discriminant, in `kind`, `definition`, `interpolation`,
  `extrapolation`, `coefficient_meaning`, `meaning` or history `kind`.

Examples:

- `history: {"kind": "continuation"}`
- a fit with both `length_change` and `strain`
- a fit without `kind`
- an unknown expansion `definition`

**Blocking diagnostic.** An envelope is returned with blocking diagnostics and
no results. This covers every targeted rule, including:

- a missing `analysis_state` (`LOAD_STATE_ANALYSIS_STATE_REQUIRED`);
- an unknown `analysis_state.contract` string (`LOAD_STATE_CONTRACT_UNSUPPORTED`);
- a 0.1–0.3 document carrying the new namespace
  (`LOAD_STATE_CONTRACT_VERSION_MISMATCH`).

## 2. Diagnostic codes

Every code below is `blocking` unless marked otherwise.

**Document**

- `LOAD_STATE_CONTRACT_VERSION_MISMATCH`
- `LOAD_STATE_REQUEST_MATERIALS_UNSUPPORTED`
- `LOAD_STATE_CASES_REQUIRED`
- `PRESSURE_CONTRACT_UNSUPPORTED`: 0.4.0 without the exact contract.

**Reference configuration**

- `LOAD_STATE_REFERENCE_CONFIGURATION_REQUIRED`
- `_INVALID`
- `_UNRESOLVED`
- `LOAD_STATE_REFERENCE_MEMBER_UNKNOWN`
- `_DUPLICATE`
- `_MISSING`

**Laws**

- `LOAD_STATE_EXPANSION_LAW_INVALID`
- `LOAD_STATE_EXPANSION_LAW_UNRESOLVED`

**Case**

- `LOAD_STATE_ANALYSIS_STATE_REQUIRED`
- `LOAD_STATE_CASE_MODULUS_BASIS_UNSUPPORTED`
- `LOAD_STATE_CONTRACT_UNSUPPORTED`
- `LOAD_STATE_PROVENANCE_REQUIRED`

**Elements**

- `LOAD_STATE_ELEMENT_UNKNOWN`
- `_DUPLICATE`
- `_MISSING`
- `LOAD_STATE_MASS_STATE_UNSUPPORTED`
- `LOAD_STATE_MATERIAL_UNRESOLVED`
- `LOAD_STATE_MEMBER_MATERIAL_MISSING`

**Material selector** (one code per selector error)

- `LOAD_STATE_MATERIAL_REFERENCE_MISMATCH`
- `_BASIS_REQUIRED`
- `_OVERRIDE_INVALID`
- `_APPLICABILITY_REQUIRED`
- `_POINT_UNRESOLVED`
- `_POINT_AMBIGUOUS`
- `_TEMPERATURE_AMBIGUOUS`
- `_TEMPERATURE_UNRESOLVED`
- `_TEMPERATURE_OVERRIDE_REQUIRED`
- `_INTERPOLATION_RANGE`
- `_PAIR_MISSING`
- `_PAIR_INVALID`
- `_QUANTITY_INVALID`

**Thermal and fit**

- `LOAD_STATE_FREE_LENGTH_REQUIRES_TEMPERATURE_REFERENCE`
- `LOAD_STATE_OPERATING_TEMPERATURE_REQUIRED`
- `LOAD_STATE_STRAIN_UNRESOLVED`: carries the kernel error, such as coverage,
  duplicate or nonpositive stretch.
- `LOAD_STATE_QUANTITY_INVALID`
- `LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED`: exact identity out of range, or
  a binary64 order inversion.

**Supports**

- `LOAD_STATE_SUPPORT_UNKNOWN`
- `_DUPLICATE`
- `_STATE_MISSING`
- `LOAD_STATE_SUPPORT_PARTICIPATION_UNSUPPORTED`
- `LOAD_STATE_SUPPORT_REFERENCE_UNSUPPORTED`
- `LOAD_STATE_BOUNDARY_MOTION_DOF_INVALID`
- `_DUPLICATE`
- `_UNRESTRAINED`
- `_UNIT_INVALID`

**Sources**

- `LOAD_STATE_SOURCE_DUPLICATE`
- `_UNRESOLVED`
- `_AMBIGUOUS`
- `_FACTOR_INVALID`
- `LOAD_STATE_LEGACY_THERMAL_PRIMITIVE_UNSUPPORTED`

**Recovery (`info`)**

- `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED`: one per case, with diagnostic ID
  `diagnostic:load-state:<case-suffix>:source-recovery-not-joined` and the
  case ID in `affected_refs`.

## 3. Evidence record

Each case publishes one entry of `contract_evidence.load_reference_states[]`
with these keys:

**Top level**

- `load_case_id`, `contract`, `profile`, `reference_configuration_id`,
  `provenance`.
- `reference_geometry`: `{kind: "authored_model_geometry", projection_sha256}`.
- `history`: `{kind: "independent_equilibrium"}`.
- `solve`: `{requested_mode, recovery_method, boundary, eigenload}`.
- `source_recovery`: `{status: "not_joined", code}`.

**`members[]`**

- Material: `pipe_id`, `material_id`, `material_selection_kind`,
  `consumed_material_points[] {point_id, temperature_k, E_pa, nu, retained_G_ignored}`,
  `interpolation_fraction`, `applicability_reference`,
  `analysis_basis_override`, `selected_E_pa`, `selected_nu`, `derived_G_pa`,
  `G_basis`, `retained_G_ignored`.
- Temperatures: `operating_temperature_k`, `material_selection_temperature_k`,
  `reference_basis`, `installation_temperature_k`.
- Thermal definition: `thermal_definition`, `expansion_law_id`,
  `coefficient_datum_k`.
- Law data: `consumed_law_point_indices`, `consumed_law_segments[]`,
  `consulted_law_point_indices`, `consulted_law_segments[]`. Each segment is
  `{use: "interpolation_sample"|"integration_interval", lower_index, upper_index, start_k, end_k}`.
  Consumed data entered the value. Consulted data only established coverage or
  positivity admissibility.
- Stretches and strains: `installation_datum_stretch`,
  `operating_datum_stretch`, `thermal_strain`, `thermal_stretch`,
  `fit_strain`, `fit_stretch`, `total_eigenstrain`, `eigenstrain_composition`.
- Fit: `fit_kind` (`none`|`natural_length_change`|`fit_strain`), `fit_input`,
  `reference_length_m`.

Absent optional values are JSON `null`.

**`support_components[]`**

`{support_id, node_id, dof, global_dof, law_kind: "rigid_prescribed", prescribed_value, unit: "m"|"rad", meaning, physical_state_source}`.
It lists only DOFs with an entered `boundary_motion`. Every other restrained
DOF is an explicit zero boundary value.

**`contributions[]`** (the complete physical-source ledger)

| owner_kind | classification | source_id |
|---|---|---|
| `stored_primitive` | `ordinary_applied` | the primitive ID |
| `resolved_member_state` | `eigenstrain` | `member_state:<pipe>` |
| `support_state` | `prescribed_boundary` | `support_state:<support>:<DOF>` |
| `pressure_region` | `pressure_eigen_and_closure` | `pressure_region:<id>` |

A `stored_primitive` entry also carries `factor`, `category`, `dimension`,
`authored_normalized_magnitude` and `applied_magnitude`. A
`resolved_member_state` entry also carries `consumed_input_refs` and `value`.

**`excluded_sources[]`**

`{source_id, owner_kind: "stored_primitive", classification: "excluded", category, reason}`
for each stored primitive not listed in `load_sources`.

`exact_cases[i].material_basis` is
`"resolved_per_member_load_reference_state_v1"`, and `pipe_materials` comes
from the same resolved members.

## 4. Support family rules

These are existing rules and unchanged. A support restrains rotational DOFs
only as `family: "anchor"`. So does an unlabelled support with six restraints,
which is an anchor by default. `guide` and `line_stop` restrain translations
only, and `vertical_support` restrains UZ only. `boundary_motion` is admitted
only on a DOF that the support restrains rigidly.

## 5. Basis × fit and temperature behaviour

- Any member basis (`temperature_reference` or `direct_strain_reference`) may
  combine with any fit (`none`, `natural_length_change` or `fit_strain`).
  `free_length_state` requires `temperature_reference`.
- An `engineering_secant` table needs coverage and positivity only over
  [min(T_install,T), max(T_install,T)]. The datum need not be covered. Integral
  and dilation laws must cover the datum.
- Temperature identity is exact across K, degC (alias C), degF and degR.
  Equality, bracket, endpoint, actual-versus-selected and law datum/table
  decisions compare exact rationals built from the shortest round-trip decimal
  and the exact affine unit definition. Numerical evaluation remains binary64.
  No tolerance snapping is applied.

## 6. Coverage notes from the runtime TASK

The runtime suite does not exercise the following. They remain for later
tests or the independent review:

- a positive runtime check of the `coefficient_definition` reference case
  (differential and logarithmic);
- `LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED`;
- basis × fit combinations beyond temperature_reference with a length change,
  and direct_strain_reference with fit_strain;
- anything beyond ID-presence checks of the `contributions` and
  `excluded_sources` entry shapes.

The kernel and implementer tests cover the log/datum definitions and the
identity mechanism privately.
