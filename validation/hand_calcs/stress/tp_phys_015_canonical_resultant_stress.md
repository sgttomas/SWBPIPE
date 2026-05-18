# STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY

## Purpose

Invented stress recovery benchmark linking the governed TP-PHYS-014 canonical
`analytical_solver_model` payload to mechanics-only station stress recovery. It
uses the canonical payload solver path resultants as the stress-recovery
resultant source.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Canonical source fixture:
  `validation/benchmarks/mechanics/fixtures/tp_phys_014_canonical_analytical_payload.json`.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, allowables, SIF/flexibility factors, or fatigue criteria.

## Traceability

| Boundary | Evidence |
|---|---|
| Physical source | TP-PHYS-014 invented physical source represented by the canonical analytical payload fixture. |
| Analytical model | `ANALYTICAL-TP-PHYS-014` with `model_role = analytical_solver_model`. |
| Solver input | Governed straight pipe with `y_reference = [0.0, 1.0, 0.0]`, one `force_per_length` distributed load, and one point force. |
| Resultant evidence | `solve_tp_phys_014_canonical_analytical_payload()` returns midspan resultants. |
| Stress recovery | `StationStressRecoveryInput::from_station_resultants` feeds mechanics-only recovery. |

## Canonical Midspan Resultants

The upstream TP-PHYS-014 mechanics fixture gives the midspan station resultants:

| Resultant | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `N` | 0.0 | N | force |
| `V_y` | 4.0 | N | force |
| `V_z` | 0.0 | N | force |
| `T` | 0.0 | N-m | moment |
| `M_y` | 0.0 | N-m | moment |
| `M_z` | 4.0 | N-m | moment |

## Governed Stress-Recovery Section Evidence

TP-STRESS-016 resolves the stress-section provenance gap by referencing
governed DEL-03-08 section-property calculation evidence instead of deriving or
defaulting stress section inputs inside the stress benchmark.

Evidence identity:
`SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25`.

Calculation basis:

| Input | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Outside diameter | 2.0 | m | length |
| Wall thickness | 0.25 | m | length |

The section values are the circular-pipe section-property outputs represented
for mechanics-only stress recovery. For this invented symmetric section,
`Z_y = Z_z`.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Area, `A` | 1.3744467859455345 | m^2 | area |
| Section modulus about `Y`, `Z_y` | 0.5368932757599744 | m^3 | section_modulus |
| Section modulus about `Z`, `Z_z` | 0.5368932757599744 | m^3 | section_modulus |
| Torsion constant, `J` | 1.0737865515199487 | m^4 | second_moment_area |
| Torsion radius, `r` | 1.0 | m | length |

## Expected Stress Components

Axial normal stress:

```text
sigma_axial = N / A
             = 0.0 / 1.3744467859455345
             = 0.0
```

Bending normal stress from `M_z`:

```text
sigma_bending_z = M_z / Z_z
                = 4.0 / 0.5368932757599744
                = 7.450270250336039
```

Bending normal stress from `M_y`:

```text
sigma_bending_y = M_y / Z_y
                = 0.0 / 0.5368932757599744
                = 0.0
```

Torsional shear stress:

```text
tau_torsion = T r / J
             = 0.0 * 1.0 / 1.0737865515199487
             = 0.0
```

## Gap Closed In TP-STRESS-016 And TP-SECTION-021

The current path proves that canonical TP-PHYS-014 resultants can feed
mechanics-only stress recovery while carrying governed section-property
evidence for stress section inputs.

TP-SECTION-021 adds a result-export fixture at
`fixtures/results/invented/tp_phys_015_section_property_stress_evidence_envelope.json`
that transports the governed section-property evidence identity and values as a
`section_property_evidence` result set, then links that evidence to the
mechanics-only bending stress value with an explicit result trace link. This is
schema-first validation evidence only; it does not introduce a public API, CLI,
report, persistence, release, acceptance, or professional-reliance behavior.

## Formal Witness Added In TP-WITNESS-023

TP-WITNESS-023 adds the machine-readable OpenMath hand-calc witness at
`validation/witness/fixtures/tp_phys_015_section_property_stress_witness.json`.
That witness is the authoritative source for its generated human rendering at
`validation/hand_calcs/stress/generated/tp_phys_015_section_property_stress_witness.md`
and generated Strict Content MathML at
`validation/witness/generated/tp_phys_015_section_property_stress_witness.mathml`.

The validator interprets the witness formula graph independently from the
OpenPipeStress section-property, stress-recovery, and solver implementation
code, checks dimensions, and compares the witness outputs to the existing
TP-SECTION-021 result-export fixture.

## Boundary

This fixture is mechanics-only stress recovery. It does not compare to
allowables, classify code stress categories, apply fatigue rules, use
SIF/flexibility factors, or make professional approval or code-compliance
claims.

Tolerance policy: `TBD`.
