# Mechanics Hand-Calculation Notes

These notes support `DEL-09-01 - Mechanics benchmark suite`.

All numeric values are invented for public verification fixtures and are derived
from elementary open mechanics. They are not protected standards examples,
commercial software examples, proprietary engineering values, code-specific
acceptance criteria, or professional approval evidence.

Final tolerance policy, release thresholds, CI gate policy, benchmark
publication scope, external validation claims, and professional reliance remain
`TBD`.

## Fixture Unit Basis

These notes use explicit fixture-local unit identifiers only:

| Quantity family | Unit | Canonical dimension |
|---|---|---|
| Length and displacement | `m` | length |
| Force | `N` | force |
| Moment | `N-m` | moment |
| Stress or elastic modulus | `Pa` | stress |
| Pressure | `Pa` | pressure |
| Area | `m^2` | area |
| Second moment of area | `m^4` | second_moment_area |
| Section modulus | `m^3` | section_modulus |
| Linear stiffness | `N/m` | linear_stiffness |
| Mass per length | `kg/m` | mass_per_length |
| Acceleration | `m/s^2` | acceleration |
| Rotation | `rad` | rotation |
| Temperature interval | `K` | temperature_interval |
| Thermal expansion coefficient | `1/K` | thermal_expansion_coefficient |
| Dimensionless ratios and counts | `ratio` or `count` | dimensionless |
| Distributed force per length | `N/m` | force_per_length |

The project unit catalog and conversion constants remain `TBD`.

## Fixture Inventory

The mechanics benchmark crate source inventory is mirrored here so each fixture
has an explicit public-original hand-calculation note.

| Fixture ID | Hand-calculation note |
|---|---|
| `MECH-CANTILEVER-TIP-FORCE` | `cantilever_tip_force.md` |
| `MECH-PORTAL-SWAY-ORIGINAL` | `portal_frame_sway.md` |
| `MECH-BRANCH-ASSEMBLY-THREE-MEMBER` | `branch_assembly.md` |
| `MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY` | `straight_pipe_weight_recovery.md` |
| `MECH-SUPPORT-BOUNDARY-MIXED` | `support_boundary_mixed.md` |
| `MECH-PRIMITIVE-LOAD-PREP` | `primitive_load_preparation.md` |
| `MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION` | `tp_phys_002_linear_static_integration.md` |
| `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` | `tp_phys_004_load_to_resultant.md` |
| `MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT` | `tp_phys_005_oriented_load_to_resultant.md` |
| `MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT` | `tp_phys_006_partial_span_load_to_resultant.md` |
| `MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS` | `tp_phys_007_station_sweep_resultants.md` |
| `MECH-FIXED-FIXED-THERMAL-AXIAL` | `fixed_fixed_thermal_axial.md` |
| `MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS` | `tp_phys_008_thermal_pressure_axial_effects.md` |
| `MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS` | `tp_phys_009_combined_load_axial_effects.md` |
| `MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD` | `tp_phys_014_canonical_analytical_payload.md` |
| `MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE` | `tp_phys_015a_canonical_solve_result_envelope.md` |
| `MECH-IMPOSED-DISPLACEMENT-SPRING` | `imposed_displacement_spring.md` |
| `MECH-INCLINED-MEMBER-TRANSFORM` | `inclined_member_transform.md` |
| `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` | `expansion_loop_curved_bend_thermal.md` |
| `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` | `curved_bend_distributed_load_fixed_end.md` |
| `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` | `tp_pmm_p3_occloadgen_equivalent_static.md` |

## Notes

- `tp_phys_002_linear_static_integration.md` records the integrated
  TP-PHYS-002 invented mechanics case.
- `tp_phys_004_load_to_resultant.md` records the invented load-to-resultant
  integration case.
- `tp_phys_005_oriented_load_to_resultant.md` records the invented
  orientation-aware load-to-resultant integration case.
- `tp_phys_006_partial_span_load_to_resultant.md` records the invented
  partial-span distributed-load integration case.
- `tp_phys_007_station_sweep_resultants.md` records the invented ordered
  station-resultant sweep case.
- `tp_phys_008_thermal_pressure_axial_effects.md` records the invented
  thermal-restraint plus pressure-thrust axial-effect case.
- `tp_phys_009_combined_load_axial_effects.md` records the invented combined
  distributed user-load plus thermal/pressure axial-effect integration case.
- `tp_phys_014_canonical_analytical_payload.md` records the invented canonical
  analytical payload consumption case.
- `tp_phys_015a_canonical_solve_result_envelope.md` records the
  validation-local canonical solve-result envelope evidence.
- `branch_assembly.md` records the invented three-member branch topology
  benchmark required by PRD section 16.2.
