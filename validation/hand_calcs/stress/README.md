# Stress Recovery Hand Calculations

These notes support `DEL-09-02 - Stress recovery benchmark suite`.

All cases use invented values and elementary open mechanics. They are public
verification aids only. They do not reproduce protected standards examples,
copied code formulas, commercial benchmark files, proprietary engineering
values, allowables, SIF/flexibility factors, fatigue criteria, or professional
approval claims.

Final tolerances, release thresholds, CI gate policy, and professional reliance
remain `TBD` pending human approval.

## Fixture Unit Basis

These notes use explicit fixture-local unit identifiers only under
`PKG09-STRESS-FIXTURE-UNITS-EXPLICIT-N-M-PA`:

| Quantity family | Unit | Canonical dimension |
|---|---|---|
| Force | `N` | force |
| Moment | `N-m` | moment |
| Pressure | `Pa` | pressure |
| Stress | `Pa` | stress |
| Length | `m` | length |
| Area | `m^2` | area |
| Section modulus | `m^3` | section_modulus |
| Second moment style section property | `m^4` | second_moment_area |

The project unit catalog and conversion constants remain `TBD`.

## Fixture Notes

| Fixture | Note |
|---|---|
| `STRESS-AXIAL-NORMAL-ORIGINAL` | [axial_normal.md](axial_normal.md) |
| `STRESS-BENDING-NORMAL-ORIGINAL` | [bending_normal.md](bending_normal.md) |
| `STRESS-TORSIONAL-SHEAR-ORIGINAL` | [torsional_shear.md](torsional_shear.md) |
| `STRESS-PRESSURE-MEMBRANE-ORIGINAL` | [pressure_membrane.md](pressure_membrane.md) |
| `STRESS-RANGE-MECHANICS-ORIGINAL` | [stress_range.md](stress_range.md) |
| `STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL` | [integrated_straight_pipe_resultants.md](integrated_straight_pipe_resultants.md) |
| `STRESS-TP-PHYS-004-LOAD-TO-RESULTANT` | [tp_phys_004_load_to_resultant_stress.md](tp_phys_004_load_to_resultant_stress.md) |
| `STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS` | [tp_phys_005_oriented_load_to_stress.md](tp_phys_005_oriented_load_to_stress.md) |
| `STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS` | [tp_phys_006_partial_span_load_to_stress.md](tp_phys_006_partial_span_load_to_stress.md) |
| `STRESS-TP-PHYS-007-STATION-SWEEP-STRESS` | [tp_phys_007_station_sweep_stress.md](tp_phys_007_station_sweep_stress.md) |
| `STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS` | [tp_phys_008_thermal_axial_effect_to_stress.md](tp_phys_008_thermal_axial_effect_to_stress.md) |
| `STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS` | [tp_phys_009_combined_axial_bending_to_stress.md](tp_phys_009_combined_axial_bending_to_stress.md) |
| `STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY` | [tp_phys_015_canonical_resultant_stress.md](tp_phys_015_canonical_resultant_stress.md) |
| `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` | [tp_pmm_p3_milltol_effective_wall_stress.md](tp_pmm_p3_milltol_effective_wall_stress.md) |

## Formal Witness Renderings

Formal hand-calc witnesses use machine-readable witness JSON as the
authoritative calculation source. Markdown and MathML renderings are generated
from that source and checked for deterministic reproducibility.

Generated artifacts are current only when
`python3 -m pytest -q tests/test_calculation_witness.py` passes. The generated
Markdown is not authoritative; the witness JSON remains the source artifact.

| Witness | Source JSON | Generated Markdown | Generated MathML |
|---|---|---|---|
| `WITNESS-TP-PHYS-015-SECTION-PROPERTY-STRESS` | [../../witness/fixtures/tp_phys_015_section_property_stress_witness.json](../../witness/fixtures/tp_phys_015_section_property_stress_witness.json) | [generated/tp_phys_015_section_property_stress_witness.md](generated/tp_phys_015_section_property_stress_witness.md) | [../../witness/generated/tp_phys_015_section_property_stress_witness.mathml](../../witness/generated/tp_phys_015_section_property_stress_witness.mathml) |
