# Mechanics Hand-Calculation Notes

These notes support `DEL-09-01 - Mechanics benchmark suite`.

All numeric values are invented for public verification fixtures and are derived
from elementary open mechanics. They are not protected standards examples,
commercial software examples, proprietary engineering values, code-specific
acceptance criteria, or professional approval evidence.

Final release tolerances and CI gate thresholds remain `TBD`.

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
