# Stress Recovery Benchmarks

This crate contains original stress recovery verification fixtures for
`DEL-09-02 - Stress recovery benchmark suite`.

The fixtures are public project content because their inputs, expected values,
and derivations are generated from elementary open mechanics within this
repository. They do not copy protected standards examples, code formulas,
commercial software benchmarks, proprietary engineering values, allowables,
SIF/flexibility factors, or code-specific acceptance criteria.

Numerical comparison values here are regression evidence for current
code-neutral stress recovery behavior. Release thresholds, final tolerance
policy, CI gate policy, stress range acceptance, fatigue criteria, and
professional reliance remain `TBD` pending human approval.

## Fixture Unit Basis

Fixture values carry explicit unit identifiers in code under
`PKG09-STRESS-FIXTURE-UNITS-EXPLICIT-N-M-PA`. This is a fixture-local basis
only: it records units for evidence review and does not define project
conversion constants or the canonical unit catalog, which remain `TBD`.

## Fixture Families

| Family | Fixture IDs |
|---|---|
| Axial normal stress | `STRESS-AXIAL-NORMAL-ORIGINAL` |
| Bending normal stress | `STRESS-BENDING-NORMAL-ORIGINAL` |
| Torsional shear stress | `STRESS-TORSIONAL-SHEAR-ORIGINAL` |
| Pressure membrane stress | `STRESS-PRESSURE-MEMBRANE-ORIGINAL` |
| Stress range | `STRESS-RANGE-MECHANICS-ORIGINAL` |
| Integrated straight-pipe stress | `STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL` |
| Load-to-resultant stress | `STRESS-TP-PHYS-004-LOAD-TO-RESULTANT` |
| Oriented load-to-stress | `STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS` |
| Partial-span load-to-stress | `STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS` |
| Station-sweep stress | `STRESS-TP-PHYS-007-STATION-SWEEP-STRESS` |
| Thermal axial-effect-to-stress | `STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS` |
| Combined axial-bending-to-stress | `STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS` |
| Canonical analytical resultant stress | `STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY` |

Hand-calculation notes are in `validation/hand_calcs/stress/`.
