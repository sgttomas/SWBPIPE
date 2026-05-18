# TASK RUN - TP-VERIFY-012C Stress Recovery Traceability Gap Triage

## Identity

- Date: 2026-05-17
- Persona: canonical `TASK`
- DeliverableID: `DEL-09-02`
- PackageID: `PKG-09`
- Purpose: stress recovery traceability gap triage
- RequestedBy: `WORKING_ITEMS orchestrator`

## Required Reads

Read before acting:

- `_CONTEXT.md`
- `_STATUS.md`
- `_REFERENCES.md`
- `_DEPENDENCIES.md`
- `Dependencies.csv`
- `MEMORY.md`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`

Additional read-only evidence inspected:

- `execution/_DAG/_LATEST.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `docs/CONTRACT.md`
- `validation/benchmarks/stress/src/lib.rs`
- `validation/hand_calcs/stress/tp_phys_015_canonical_resultant_stress.md`
- `validation/benchmarks/mechanics/fixtures/tp_phys_014_canonical_analytical_payload.json`
- `validation/benchmarks/mechanics/src/lib.rs`
- `validation/hand_calcs/mechanics/tp_phys_015a_canonical_solve_result_envelope.md`
- `schemas/results.schema.yaml`
- `schemas/model.schema.yaml`
- `schemas/section.schema.yaml`
- `core/loads/stress_recovery/src/lib.rs`
- `core/section_properties/calculator.py`
- `tests/test_section_properties.py`
- TP-PHYS-015 run records for slices A, B, C, D1, D2, and the parent fan-in.

## Evidence Summary

- TP-PHYS-015C proves that the canonical TP-PHYS-014 midspan resultants can feed `StationStressRecoveryInput::from_station_resultants` and `recover_station_stresses`.
- The TP-PHYS-015C fixture records mechanics-only axial normal, bending normal about `Y`/`Z`, and torsional shear components. Pressure remains absent.
- The canonical TP-PHYS-014 analytical payload section carries `area`, `second_moment_y`, `second_moment_z`, and `torsion_constant` with provenance, but it does not carry governed `section_modulus_y`, `section_modulus_z`, or `torsion_radius` stress-recovery inputs.
- `core/loads/stress_recovery` already requires explicit section modulus metadata for unit-checked stress recovery and blocks wrong dimensions at the stress boundary.
- `DEL-03-08` section-property evidence already calculates `section_modulus` as a canonical dimension, but this does not yet define how section-modulus outputs enter the canonical analytical payload or downstream result evidence.
- `schemas/results.schema.yaml` can carry stress quantities through `ResultFamily = stress` and stress component metadata, but its `DimensionId` enum does not include section-input dimensions such as `area`, `second_moment_area`, or `section_modulus`.

## Gap Classification

| Gap | Classification | Recommended owner | Rationale | Non-goals |
|---|---|---|---|---|
| Governed stress section-modulus fields are absent from the canonical analytical payload. | `NEEDS_CROSS_DELIVERABLE_RULING` | Primary: `DEL-03-08`; coordination with `DEL-13-04`, `DEL-02-01`, and `DEL-09-02`. | Section modulus can be calculated in `DEL-03-08`, named by PKG-02 dimensions, and consumed by stress recovery, but no approved boundary says whether analytical payloads must carry `section_modulus_y`, `section_modulus_z`, and `torsion_radius` directly or derive them through a governed section-property reference. | Do not derive section modulus silently from second moments; do not add fixture-local public defaults as schema behavior. |
| Canonical analytical payload section traceability stops before stress-specific section inputs. | `READY_FOR_SCHEMA_TRANCHE` | Primary: `DEL-13-04`; upstream schema coordination with `DEL-02-01` and `DEL-03-08`. | The payload carries section provenance for solver stiffness quantities, but stress recovery needs traceable section-modulus and torsion-radius inputs. A schema/adapter tranche can define explicit source refs without changing stress equations. | Do not add solver, GUI, report, CLI, or persistence behavior in this tranche. |
| Canonical resultant-to-stress mechanics evidence is benchmark-only and should be strengthened through unit metadata. | `READY_FOR_RUNTIME_TEST_TRANCHE` | Primary: `DEL-09-02`; upstream contract check against `DEL-05-03`. | Existing benchmark tests the resultant-to-stress path, and `DEL-05-03` already has unit-metadata boundary checks. A runtime-test tranche can prove TP-PHYS-015C resultants plus explicit stress section metadata remain unit-checked and diagnostic-preserving. | Do not introduce allowables, stress indices, SIF/flexibility factors, fatigue checks, or compliance wording. |
| Stress evidence is not yet schema-backed as a full result/export envelope. | `READY_FOR_SCHEMA_TRANCHE` | Primary: `DEL-08-04`; stress evidence source remains `DEL-09-02`. | The result schema can represent stress outputs but does not currently model stress-recovery input-basis quantities or per-value section-input trace chains. Schema-backed export evidence should be handled in a result-export tranche, not by changing benchmark semantics. | Do not expand headless runner behavior, report rendering, public API/CLI, or persistence here. |
| Final tolerance policy, release thresholds, CI gate policy, benchmark publication scope, and professional reliance remain open. | `KEEP_AS_TBD` | Future verification/release governance owner, not this TASK. | These are authority-controlled policy decisions already preserved as TBDs in DEL-09-02 memory. | Do not mark acceptance, release readiness, or professional/code compliance. |

## Recommended Next Work

1. Run a cross-deliverable ruling tranche to decide the canonical owner and shape of stress section-modulus payload fields before implementation.
2. After that ruling, run a schema tranche for the analytical payload/source-chain fields and result-export stress input-basis evidence.
3. Run a DEL-09-02 runtime-test tranche proving TP-PHYS-015C stress recovery with explicit unit metadata and diagnostic propagation.

## Validation

- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml` passed: 17 tests, 0 doc-tests.
- `git diff --check` passed after evidence edits.

## Boundary

This TASK changed only DEL-09-02 `MEMORY.md` and this run record. It did not change schemas, code, tests, lifecycle state, dependencies, DAG/blocker files, review findings, release records, acceptance records, public API/CLI/runtime/report/persistence behavior, protected standards content, allowables, SIF/flexibility data, fatigue criteria, code-compliance claims, or professional approval claims.
