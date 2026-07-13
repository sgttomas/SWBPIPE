# Stress Recovery Benchmarks

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-09-02-DECL-007`.

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

## Readiness Boundary

The crate records readiness metadata for unresolved authority items. Final
tolerance policy, release thresholds, CI gate policy, result-envelope/export
integration, benchmark publication scope, canonical unit/conversion policy, and
professional reliance remain `TBD`.

The Rust tests assert that every fixture has public-original provenance,
fixture-local units, dimensioned expected values, and unresolved tolerance
policy. They also assert that this README and the hand-calculation README list
the current fixture inventory.

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
| Mill-tolerance effective-wall stress | `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` |
| Modulus-basis stress range | `STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS` |

Hand-calculation notes are in `validation/hand_calcs/stress/`.
