# Procedure: DEL-04-01 3D frame stiffness kernel

## Purpose

This procedure defines how a future TASK worker should proceed from the local setup kit into implementation without expanding beyond DEL-04-01.

## Prerequisites

- Confirm the active deliverable is DEL-04-01 and the write scope is the assigned deliverable or future implementation path explicitly authorized by a sealed brief.
- Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, and `_SEMANTIC_LENSING.md`.
- Confirm applicable invariants: OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-2, OPS-K-REPORT-1, OPS-K-AGENT-1..4, and OPS-K-IP-1.
- Confirm architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
- Obtain or cite lawful mechanics/numerics references before implementing formulas, expected numerical values, tolerances, or benchmark thresholds.

## Steps

1. Reconfirm scope boundaries against SOW-005 and SOW-035.
2. Identify any implementation decisions still TBD, including DOF ordering, coordinate convention, sparse storage, solver library, error handling policy, tolerance policy, and reproducibility target.
3. If a TBD decision is needed to write code, record it as a required human/architecture ruling rather than guessing.
4. Define interfaces for model topology, six-DOF node mapping, frame assembly, coordinate transforms, boundary-condition application, sparse solve invocation, unit validation, and result/diagnostic output.
5. Keep straight pipe element behavior, supports, nonlinear supports, loads, stress recovery, diagnostics specialization, reports, and rule packs at their assigned deliverable boundaries.
6. Implement only from lawful, source-grounded mechanics and numerics references after the implementation brief authorizes code work.
7. Create deterministic unit tests before release use; use rights-cleared fixtures with explicit units and no protected data.
8. Ensure missing solve-required inputs produce explicit findings and do not become silent defaults.
9. Ensure output envelopes preserve mechanics/rule/human approval separation and do not claim certification or compliance.
10. Run protected-content and provenance checks before any release or handoff.

## Verification

- Confirm no implementation values were invented from this setup kit.
- Confirm unit-aware and dimensionally checked interfaces are present in implementation scope.
- Confirm deterministic tests exist for every implemented kernel behavior.
- Confirm sparse performance and reproducibility checks are coordinated with DEL-04-05 when targets are sealed.
- Confirm diagnostics/result envelopes align with AB-00-03 and AB-00-06.
- Confirm protected-content review passes before release.

## Records

- Implementation brief and sealed write scope.
- Architecture decisions for TBD items.
- Source/provenance records for mechanics and numerical references.
- Unit test and CI results.
- Protected-content review result.
- Result-envelope and diagnostics samples.
