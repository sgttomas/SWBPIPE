# CP2_RUNTIME_TESTS — return

TASK (Type 2) for the session-2 load-state WORKING_ITEMS manager. Brief sha256
`4cf1ae3d…9b77` and CP2_WIRE sha256 `81a7adba…6996` were verified before starting.
Paths are WORKING_ROOT-relative. Machine-specific paths are only in `_run_records/`.

## Deliverable

- `core/product_physics/tests/load_reference_state_runtime.rs` (new; sole writer)
  sha256 `d5be0bd812a2af51c1a32fc5c237a42b76b460ae75d0a610837f01b676f6960b`.
- No helper fixtures were added. `reference_cases.json` is unchanged
  (sha256 `5478bba8…4b891`, equal to HEAD).
- No other writes: no `src/**`, existing test, fixture, schema or UI file, and no Git operation.

Every request is built to CP2_WIRE (0.4.0, exact_straight_pressure_v2,
`reference_configurations`, case `analysis_state`). Each one goes through
`run_linear_static_preview_value_with_mode` in both `SparseInteractive` and `DenseScrutiny`.

Expected values and most inputs are read at test time from
`reference_cases.json`, using `value` from the annular companions and the geometry block.
No expected constant is copied. The criterion is the protected relative 1e-9. An exact
zero uses a floor of 1e-9 × the case's own named force, moment, displacement or strain
scale, as stated in the file header and at each call site.

Each solved case asserts:
- producer `openpipestress.result_semantics/0.3.0/load-reference-1`;
- profile `resolved_straight_load_state_v1`;
- one `contract_evidence.load_reference_states` record per case, keyed by `load_case_id`, with
  `source_recovery {status: not_joined, code: LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED}`;
- exactly one `info` diagnostic `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED` naming the case;
- no `source_block_recovery`.

## Tests (20; all loop over both modes)

| # | Test | Coverage and discriminators |
|---|---|---|
| 1 | `prescribed_translation_two_bar_couples_prescribed_root_motion` | Root UX +0.1 mm (authored in mm); far UX omitted and explicit 0; forward and reversed member orientation. Checks root/middle/far UX, station and end-force N for both members, root/far Fx and zero other actions. Root Fx ≠ `K_cf·u_f − f_c` = −(E1·As/L1)·u_mid; middle UX ≠ 0 (omitted coupling). |
| 2 | `prescribed_translation_all_fixed_single_bar_has_nonzero_reactions` | Zero free DOFs, root UX prescribed. N, root/far Fx, zero other actions; Fx ≠ 0. |
| 3 | `prescribed_rotation_all_fixed_reproduces_signed_end_actions` | Root RZ 1e-3 rad. Root/far Fy and Mz, zero other actions, ΣFy = 0 and ΣMz about the root = 0. |
| 4 | `prescribed_rotation_free_tip_is_rigid_radian_rotation` | Tip UY = Lθ and RZ = θ; tip UX, root actions and all midspan internal actions zero. UY ≠ 0 (missing coupling); UY ≠ 0.001 and RZ ≠ 0.0005 (length-normalized angle). |
| 5 | `shared_material_serial_companion_selects_point_per_member` | One material, two dated `exact_point`s (E/ν from the fixture) and an unused invented base. Member 2 has interval strain 0.001. Middle UX, N1/N2 and Fx; UX ≠ −1/2000 (material-ID-only E). Evidence E, ν, derived G and eigenstrain per member. A torsion variant checks middle RX and root/far Mx from the per-member derived G. |
| 6 | `thermal_datum_ratio_secant_table_fixed_and_free` | Engineering-secant table law, datum 20, install 50, operating 150. Variants: degC; all-K (1/degC coefficients); degF (1/degF coefficients); and the reviewed control-4 two-point table (50/150, datum outside the table) in degC and K. All must give identical mechanics. Fixed: station/end N, and wall N via `pipe_wall_axial_force_v2` in a zero-pressure-region variant, plus root/far Fx. Free: tip UX = 43/25009 m, N = 0. Rejects α_hot·ΔT and subtracted-dilation strains. Evidence strains and E. |
| 7 | `constant_alpha_interval_needs_no_absolute_temperature` | α 1e-5/K × 80 K, no absolute temperature anywhere; also 1/degC × degC interval. Fixed N = −E·As·0.0008 and Fx; free tip UX = L·0.0008; evidence strains. |
| 8 | `signed_fit_cold_hot_return_fixed_and_released` | Cut −2 mm, constant secant α, hot E from `exact_point` at 100 degC. Fixed (plain, zero-pressure-region wall rows, and a variant with installation/operating temperatures authored in K against degC points and datum, which must select the same points with no override) and released. N, Fx and tip UX per state; evidence E/thermal/fit/total. Return equals cold bit for bit. Rejects hot additive strains, hot using cold E, fit applied twice and flipped sign (generic values × As/area). |
| 9 | `signed_fit_baselines_cut_long_and_direct_fit_strain` | No-fit baseline (cold 0, hot −228000π N), cut long (−76000π N), and the direct `fit_strain` route with `explicit_interval_strain`, fixed and released. |
| 10 | `persistent_source_counted_once_and_unreferenced_excluded` | +100/−1000/+200 N listed; stored +999 N unreferenced. Tip UX = −7/(3800000π) m and root Fx = +700 N; not −600, not −700+999. `excluded_sources` names only the +999 source; `contributions` names the three listed sources. |
| 11 | `duplicate_source_ref_blocks_and_distinct_equal_sources_both_apply` | A duplicate `source_ref` (equal or unequal factors) gives `LOAD_STATE_SOURCE_DUPLICATE`. Two distinct +200 N sources give +400 N; one source at factor 2 matches. |
| 12 | `negative_unit_dimension_controls_block` | Rotation in m; translation in rad. |
| 13 | `negative_typed_boundary_rejections_return_err` | Unknown coefficient definition; fit without `kind`; fit carrying both length and strain (either kind); unknown `analysis_state` field; unknown thermal and basis kinds. All must return `Err`. |
| 14 | `negative_legacy_thermal_referenced_with_resolved_state_blocks` | A referenced legacy thermal blocks, with a resolved or unchanged state. An unreferenced one is excluded and reported, and only the resolved strain acts (not both). |
| 15 | `negative_legacy_version_documents_carrying_load_state_block` | 0.3.0 carrying any or all of analysis_state, reference_configurations and expansion_laws, and 0.2.0 carrying analysis_state, give `LOAD_STATE_CONTRACT_VERSION_MISMATCH`. |
| 16 | `negative_boundary_motion_on_unrestrained_dof_blocks` | UX motion on a support that does not restrain UX. |
| 17 | `negative_missing_element_state_blocks` | Missing element state; duplicate element state. |
| 18 | `negative_free_length_state_without_operating_temperature_blocks` | No operating temperature; direct-strain basis; law not on the material. |
| 19 | `negative_thermal_law_coverage_controls_block` | Duplicate T; operating above the table; installation below it; datum outside coverage (differential definition); nonpositive stretch (cut = −L, fit strain −1 and −1.5). |
| 20 | `negative_additional_wire_rules_block` | Duplicate motion DOF; spring motion (with a no-motion spring baseline that solves); material_ref mismatch; missing or duplicate support state; unknown, cross-case and zero-factor sources; `modulus_basis_ref`/`_temperature`; non-empty request materials; `mass_state_ref`; inactive participation; `base_motion`; missing or duplicate member reference; unknown reference configuration; non-independent history, missing analysis_state and unknown contract (channel left open); point-temperature mismatch without override blocks, and with override solves at N = −E_cold·As·ε*_hot. |

## Commands and observed results

Raw logs and commands are in `_run_records/` (`COMMANDS.md`, `run_1.log` sha256
`b56cdc19…0d8`, `run_2.log` sha256 `7815ed71…f65`, `run_3.log` sha256 `8b9c8abc…5c56`). Before the manager's go I ran only
rustfmt. After it, I ran only the brief's command with `-- --nocapture`:
`cargo +1.97.1 test --locked --offline -j 1 --test load_reference_state_runtime`, with its own
CARGO_TARGET_DIR.

- Run 1: test file `734f59a3…`, 20 passed, 0 failed.
- Run 2 (file `eef0ddd5…`; the only change prints diagnostic messages): 20 passed, 0 failed.
  `src/case_state/resolve.rs` changed during this run, so run 2 is not pinned.
- Run 3 (final file `d5be0bd8…`): adds the SF2/SF3 variants, after the manager's notice that
  checkpoint-1 review repairs changed public behaviour. 20 passed, 0 failed.
- There is no failure text; no expectation or tolerance was changed.

**Candidate identity:** run 3 used HEAD `5511596af` plus the uncommitted implementer working
tree, with src tree hash `58dd2feb…b3e9`, identical before and after the run. Per-file hashes
are in `run_3.log`. This is not a committed candidate. Acceptance must rerun this binary on
the frozen, committed candidate.

Observed codes where CP2_WIRE names none. All were `MODEL_INCOMPLETE`, identical in both
modes, and each message states the intended reason (see `run_2.log`):
- `LOAD_STATE_BOUNDARY_MOTION_UNIT_INVALID`: both unit controls.
- `LOAD_STATE_LEGACY_THERMAL_PRIMITIVE_UNSUPPORTED`: referenced legacy thermal.
- `LOAD_STATE_BOUNDARY_MOTION_UNRESTRAINED`: unrestrained DOF, and spring motion.
- `LOAD_STATE_ELEMENT_MISSING`, and `LOAD_STATE_ELEMENT_DUPLICATE`.
- `LOAD_STATE_OPERATING_TEMPERATURE_REQUIRED`, `LOAD_STATE_FREE_LENGTH_REQUIRES_TEMPERATURE_REFERENCE`
  and `LOAD_STATE_EXPANSION_LAW_UNRESOLVED`.
- `LOAD_STATE_STRAIN_UNRESOLVED`, with its message naming the specific cause:
  - duplicate T: `TableNotStrictlyIncreasing`;
  - the brackets and datum outside coverage: `OutsideTableCoverage`;
  - nonpositive stretch: `NonPositiveStretch`.

  The datum control uses `differential_per_datum_length`. After SF2 a secant table no longer
  has to cover the datum, which the two-point positive variant checks.
- `LOAD_STATE_BOUNDARY_MOTION_DUPLICATE`, `LOAD_STATE_MATERIAL_REFERENCE_MISMATCH`,
  `LOAD_STATE_SUPPORT_STATE_MISSING` and `LOAD_STATE_SUPPORT_DUPLICATE`.
- `LOAD_STATE_SOURCE_UNRESOLVED` (unknown and cross-case source) and `LOAD_STATE_SOURCE_FACTOR_INVALID`.
- `LOAD_STATE_CASE_MODULUS_BASIS_UNSUPPORTED`, `LOAD_STATE_REQUEST_MATERIALS_UNSUPPORTED`,
  `LOAD_STATE_MASS_STATE_UNSUPPORTED`, `LOAD_STATE_SUPPORT_PARTICIPATION_UNSUPPORTED` and
  `LOAD_STATE_SUPPORT_REFERENCE_UNSUPPORTED`.
- `LOAD_STATE_REFERENCE_MEMBER_MISSING`/`_DUPLICATE` and `LOAD_STATE_REFERENCE_CONFIGURATION_UNRESOLVED`.
- `LOAD_STATE_ANALYSIS_STATE_REQUIRED`, `LOAD_STATE_CONTRACT_UNSUPPORTED` and
  `LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED`.
- A history kind other than `independent_equilibrium` returns `Err` (unknown variant).

## Wire ambiguities and interpretations

1. These are not in the frozen CP2_WIRE; they come from the manager's message, and the tests
   assert them:
   - the evidence key `load_case_id`;
   - the fields `members[]` (`selected_E_pa`, `selected_nu`, `derived_G_pa`, `thermal_strain`,
     `fit_strain`, `total_eigenstrain`), `excluded_sources[]`, `contributions[]` and `source_recovery`;
   - the not-joined diagnostic as `info`, one per case, with the case ID in `affected_refs`.

   I check the entry shapes of `excluded_sources` and `contributions` only by the ID appearing
   in them. These keys belong in the wire before downstream consumers rely on them.
2. The existing `Guide` family allows only translations. Partial restraints that include
   rotations therefore use `family: "anchor"` (for example UY–RZ at a middle node). CP2_WIRE
   is silent on this; the manager confirmed it.
3. Basis × fit pairing: INTERFACE pairs a length change with `temperature_reference` and a fit
   strain with `direct_strain_reference`, while CP2_WIRE makes them independent fields. The tests
   use only those two pairings; cross pairings are not exercised.
4. The rejection channel is unstated for a non-independent history kind, a missing
   `analysis_state` and an unknown `contract`. The tests accept `Err` or a blocking diagnostic
   and record which one occurred.
5. "Wall N" is checked on two row families. Members without a pressure region use the
   `element_local_axial_force` station and end-force rows. Members in a zero-pressure region
   use `pipe_wall_axial_force_v2`.
6. SF2 and SF3 (manager message, after the frozen wire) changed public behaviour: secant
   tables no longer need datum coverage, and temperature identity is exact across units.
   Their positive consequences are tested as noted in tests 6 and 8. The expectations come
   from the fixture and VERIFICATION control 4. `LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED`
   (binary64 ordering loss) is not exercised.
7. The expected annular values are derived rather than read directly in three cases. The
   constant-alpha interval has only a generic reference, so annular N and UX are derived from
   its strain on the thermal companion's bar. Signed-fit wrong values are generic, scaled by
   As/area. The torsion and override checks use fixture E/ν/G/ε\* with the elementary statics
   stated in the test.

## Invented inputs (not library data)

- ν 0.3 where results do not depend on it.
- Shared-material base: E 170 GPa, ν 0.28, used by neither member.
- Point temperatures 20 and 300 degC.
- A 1000 N·m torque.
- A legacy thermal record: 1.2e-5/degC over 50 degC.
- A 1e6 N/m spring.
- A conflicting duplicate-point α of 1.3e-5.
- Bracket offsets of ±10 and +50 degC.

## Not covered or outstanding

- The `coefficient_definition` reference case (differential and logarithmic dilation values)
  was not required and has no positive runtime check. Only its coverage negative runs.
- deg angle units, `device_reference`/locked shapes, the native and browser routes, and
  persistence were not exercised; they are outside this brief.
- A rerun on the frozen candidate is required (see the candidate caveat).
