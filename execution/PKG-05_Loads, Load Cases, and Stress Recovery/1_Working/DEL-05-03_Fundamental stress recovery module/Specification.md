# Specification: DEL-05-03 Fundamental stress recovery module

## Scope

This deliverable specifies the implemented fundamental stress recovery backend slice in `core/loads/stress_recovery`. It covers mechanics stress recovery from axial force, bending, torsion, pressure, section properties, straight-pipe station resultants, station sweeps, unit metadata, result-boundary records, and mechanics-only component ranges.

This documentation alignment records existing implementation evidence only. It does not edit core code, introduce protected code stress equations, encode allowables, classify code stress categories, publish benchmark acceptance criteria, or make certification/compliance claims.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-05-03-RQ-001 | The stress recovery module shall recover fundamental mechanics stresses from force resultants, moments, pressure, and section properties. | SOW-015; DEL-05-03 context | Implemented by `recover_stresses`; implemented test cases cover axial, bending, torsion, pressure, and mechanics summaries. |
| DEL-05-03-RQ-002 | The module shall remain mechanics-only and shall not encode code stress categories, code allowables, or professional compliance decisions. | SOW-015 note; OPS-K-MECH-2; OPS-K-RULE-1; OPS-K-IP-1 | Crate README boundary plus implemented test cases for summaries without allowables and analysis status boundary. |
| DEL-05-03-RQ-003 | Inputs and outputs shall be unit-aware and dimensionally checked across force, moment, pressure, section-property, and stress quantities. | OPS-K-UNIT-1 | Implemented by `StressRecoveryInputUnitMetadata`, `recover_stresses_with_unit_metadata`, and `StressComponents::to_result_boundary_record`; implemented test cases cover missing/wrong dimensions and boundary records. |
| DEL-05-03-RQ-004 | Missing solve-required stress recovery inputs shall produce explicit findings rather than silent defaults. | OPS-K-DATA-2; OPS-K-AGENT-1 | Implemented test cases cover missing resultants, non-positive properties, non-finite pressure, non-finite recovered values, incomplete statuses, blocked ranges, and asymmetric optional components. |
| DEL-05-03-RQ-005 | Result envelopes and diagnostics shall preserve source/provenance and shall not claim certification, compliance, or human approval. | AB-00-03; AB-00-06; OPS-K-AGENT-4 | Boundary records carry schema, provenance, payload, and payload-hash references; human approval statuses are rejected as automatic software output. Final application-service persistence remains `TBD`. |
| DEL-05-03-RQ-006 | Solver/load/stress behavior changes shall include deterministic verification tests before release use. | OPS-K-SOLVER-1; AB-00-08 | Crate unit tests are authored for the implemented behavior. Release benchmark publication scope and production tolerance policy remain `TBD`. |
| DEL-05-03-RQ-007 | Architecture choices shall preserve layer/module boundaries and must not let adapters, reports, or rule packs bypass governed mechanics results. | AB-00-02; OPS-K-MECH-2 | Implemented crate consumes explicit mechanics inputs and canonical metadata; rule-pack/report/API integration remains downstream. |
| DEL-05-03-RQ-008 | The module shall preserve station identity for straight-pipe station stress recovery and shall support ordered station stress sweeps without reordering caller input. | SOW-015; DEL-05-03 context | Implemented by `StationStressRecoveryInput`, `recover_station_stresses`, and `recover_station_stress_sweep`; implemented test cases cover station identity, station fraction validation, order preservation, and invalid station rejection. |
| DEL-05-03-RQ-009 | Mechanics-only stress range recovery shall compute component-wise ranges only between two unblocked recovered mechanics states. | SOW-015; mechanics-only boundary | Implemented by `recover_stress_range`; implemented test cases cover component ranges, blocked states, asymmetric optional components, and non-finite range differences. |

## Standards

No protected standard text, protected code formulas, protected tables, material allowables, or proprietary commercial data are available in this deliverable-local context. Any future standard or code basis must be introduced only as a lawful private/project input or non-protected pointer with provenance. Clause-level requirements are `TBD`.

## Verification

| Verification area | Current evidence |
|---|---|
| Mechanics boundary | `core/loads/stress_recovery/README.md` and implemented test cases evidence recovered stresses as mechanics results, not code acceptability outcomes. |
| Unit safety | Unit metadata validation checks present force, moment, pressure, section-property, geometry, and stress-boundary dimensions; conversion constants remain `TBD`. |
| Missing inputs | Findings are emitted for missing resultants, section properties, pressure inputs, unit metadata, incompatible dimensions, non-finite values, non-positive properties, incomplete mechanics statuses, blocked states, and status-boundary violations. |
| Station behavior | Station adapters validate station fraction and finite station fields, preserve station identity, and recover station-local mechanics stresses. |
| Sweep behavior | Station sweeps preserve caller order and create deterministic indexed station IDs from the provided prefix. |
| Range behavior | Component ranges are absolute deltas between two unblocked mechanics states; asymmetric optional pressure/range components block the range result. |
| Hand calculations | Current tests use synthetic/cleared values. Exact release hand-calc, benchmark publication, and tolerance acceptance scope remain `TBD`. |
| IP/data boundary | The crate excludes protected standards examples, copied code formulas, allowables, SIF/flexibility tables, public pipe tables, and proprietary data. |

## Documentation

Implemented artifacts evidenced for this deliverable are:

- `core/loads/stress_recovery/README.md`;
- `core/loads/stress_recovery/Cargo.toml`;
- `core/loads/stress_recovery/src/lib.rs`;
- crate unit tests embedded in `src/lib.rs`.

Final application-service command/query surfaces, persistence, public report/rule-pack labels, production tolerances, release benchmark scope, and professional reliance policy remain `TBD`.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified between deliverable context and implemented stress-recovery evidence. | N/A | N/A |
