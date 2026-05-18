# Specification: DEL-04-01 3D frame stiffness kernel

## Scope

This setup specification covers the future backend feature slice for a 3D frame stiffness kernel. The sealed scope is DEL-04-01 in PKG-04, implementing the global 3D frame stiffness assembly, coordinate transforms, boundary conditions, and sparse solve interface for a 3D centerline/frame model with six degrees of freedom per node.

Out of scope for this deliverable:

- Solver implementation in this setup run.
- Numeric tolerances, convergence thresholds, performance targets, or solver library selection.
- Protected standards formulas, examples, tables, or proprietary commercial data.
- Straight pipe element details owned by DEL-04-02.
- Support/restraint model families owned by DEL-04-03 and DEL-04-04.
- Solver diagnostics layer owned by DEL-04-06.
- Rule-pack acceptability, code compliance decisions, certification claims, and professional approval.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-04-01-REQ-001 | Future implementation shall model the primary global analysis system as a 3D centerline/frame model. | SOW-005; OPS-K-MECH-1 |
| DEL-04-01-REQ-002 | Future implementation shall represent each frame node with six degrees of freedom. | SOW-005 |
| DEL-04-01-REQ-003 | Future implementation shall provide global stiffness assembly for the frame system. | Deliverables.csv row DEL-04-01 |
| DEL-04-01-REQ-004 | Future implementation shall provide coordinate transform handling between local and global frame representations; exact convention is TBD. | Deliverables.csv row DEL-04-01 |
| DEL-04-01-REQ-005 | Future implementation shall provide a boundary-condition application interface; supported restraint semantics are delegated to later support deliverables unless explicitly sealed in this deliverable. | Deliverables.csv row DEL-04-01; package scope |
| DEL-04-01-REQ-006 | Future implementation shall expose a sparse solve interface designed for sparse numerical performance and reproducible practical-model results; performance targets are TBD. | SOW-035 |
| DEL-04-01-REQ-007 | Future implementation shall remain unit-aware and dimensionally checked. | OPS-K-UNIT-1 |
| DEL-04-01-REQ-008 | Future implementation shall report missing solve-required values as explicit findings and shall not supply silent defaults. | OPS-K-DATA-2 |
| DEL-04-01-REQ-009 | Solver outputs shall compute mechanics only and shall not decide rule-pack acceptability or professional compliance. | OPS-K-MECH-2 |
| DEL-04-01-REQ-010 | Solver changes shall require deterministic verification tests before release. | OPS-K-SOLVER-1; AB-00-08 |
| DEL-04-01-REQ-011 | Diagnostics and result envelopes crossing service boundaries shall preserve code, class, severity, source, affected object, message, remediation, and provenance fields where applicable. | AB-00-03; AB-00-06 |
| DEL-04-01-REQ-012 | Future implementation shall not import protected formulas/data or embed protected standards text, tables, figures, examples, or proprietary data. | OPS-K-IP-1 |

## Standards

- Project invariants in `docs/CONTRACT.md` are governing for this setup kit.
- Decomposition and register rows cited in `_CONTEXT.md` are governing for scope and objectives.
- External mechanics references for future implementation are TBD and must be lawful, source-grounded, and compatible with the protected-data boundary.
- No code standard clause, protected formula, or professional compliance criterion is adopted by this setup kit.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| DEL-04-01-REQ-001, DEL-04-01-REQ-002 | Unit tests should verify model topology and six-DOF mapping after implementation; exact cases TBD. |
| DEL-04-01-REQ-003, DEL-04-01-REQ-004 | Unit tests should verify deterministic assembly and transform behavior using rights-cleared fixtures; equations and expected values TBD. |
| DEL-04-01-REQ-005 | Unit tests should verify boundary-condition application semantics once restraint interfaces are sealed. |
| DEL-04-01-REQ-006 | Sparse performance and reproducibility checks should be coordinated with DEL-04-05; target sizes and metrics TBD. |
| DEL-04-01-REQ-007 | Unit and schema tests should verify dimensional compatibility and unit-aware inputs/outputs. |
| DEL-04-01-REQ-008, DEL-04-01-REQ-011 | Tests should verify missing-value and diagnostics/result-envelope behavior without silent defaults. |
| DEL-04-01-REQ-009 | Tests/reviews should verify mechanics outputs remain separate from rule-pack or compliance determinations. |
| DEL-04-01-REQ-010 | CI must include deterministic solver verification gates before release use. |
| DEL-04-01-REQ-012 | Protected-content review gates must check that fixtures, docs, and source comments contain no protected standards or proprietary data. |

## Documentation

Future implementation should maintain:

- `core/solver/frame_kernel` module documentation.
- Unit test records for frame assembly, coordinate transforms, boundary conditions, unit handling, missing-value diagnostics, and sparse solve interface behavior.
- Result-envelope and diagnostics examples that disclose assumptions, warnings, model/solver versions, and provenance without certification claims.
- TBD register entries for unresolved formulation, tolerance, solver-library, and performance-target decisions.
