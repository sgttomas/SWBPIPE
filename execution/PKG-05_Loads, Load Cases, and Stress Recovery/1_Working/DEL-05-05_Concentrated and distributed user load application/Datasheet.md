# Datasheet: DEL-05-05 Concentrated and distributed user load application

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-05-05 |
| Package ID | PKG-05 |
| Package Name | Loads, Load Cases, and Stress Recovery |
| Type | BACKEND_FEATURE_SLICE |
| Scope Items | SOW-052, SOW-013 |
| Objectives | OBJ-003, OBJ-012 |
| Context Envelope | M |
| Anticipated Artifacts | load application module; load tests; result hooks |

## Attributes

| Attribute | Evidence-grounded value |
|---|---|
| Primary subject | Implemented code-neutral user-load application for concentrated forces, concentrated moments, and uniform distributed user loads. |
| Implementation surface | `core/loads/user_loads` provides the bounded Rust crate for DEL-05-05. |
| Load boundary | General user loads are explicit mechanics inputs in addition to primitive piping load categories; code-specific combinations remain outside this crate. |
| Unit posture | `UserLoadQuantity` carries a `LoadDimension`; boundary records require explicit unit metadata, provenance references, canonical schema binding, JCS payload references, and payload-hash references. |
| Solver posture | Generic application emits nodal and element distributed contribution records; straight-pipe recovery emits equivalent global nodal loads for oriented straight-pipe elements. |
| Result posture | Result recovery hooks are implemented for nodal force, nodal moment, element distributed load, and bridged element axial-effect records; final result-envelope integration remains TBD. |
| Governance boundary | No design-code load combinations, public default factors, protected standards content, proprietary project data, rule-pack checks, or professional/code-compliance claims are introduced. |
| Remaining TBDs | Final result-envelope/API/persistence/GUI/CLI/report integration, production tolerance policy, release thresholds, primitive axial-effect provenance beyond `load_id`, and professional reliance remain TBD. |

## Conditions

- Must preserve the mechanics/rule-pack separation in `OPS-K-MECH-2`.
- Must treat code-specific values and combinations as user-supplied or rule-pack supplied under `OPS-K-DATA-1`.
- Must surface missing solve-required or rule-check-required values explicitly under `OPS-K-DATA-2`.
- Must remain unit-aware and dimensionally checked under `OPS-K-UNIT-1`.
- Must not introduce invented load magnitudes, default factors, code allowables, or certification claims.

## Construction

Implemented evidence from `core/loads/user_loads/README.md` and
`core/loads/user_loads/src/lib.rs` identifies these current artifact slots:

- `load application module`: `apply_user_loads` prepares explicit nodal
  concentrated force/moment contributions and element uniform distributed-load
  contribution records for the frame-solver boundary.
- `straight-pipe equivalent recovery`: `apply_straight_pipe_equivalent_user_loads`
  recovers full-span and partial-span distributed loads plus element-station
  concentrated forces into equivalent global nodal loads for a supplied
  `StraightPipeElement`.
- `oriented straight-pipe handling`: global `X`, `Y`, and `Z` directions are
  converted to global load vectors and handled by the straight-pipe helper for
  pipes that are not aligned with global `X`.
- `axial-effect bridge`: `apply_straight_pipe_equivalent_user_loads_with_axial_effects`
  appends already-prepared primitive axial-effect contributions without
  duplicating axial-effect mechanics in `user_loads`.
- `load tests`: Rust unit tests cover concentrated force, concentrated moment,
  distributed loads, partial-span recovery, oriented straight-pipe recovery,
  station point-force recovery, axial-effect bridge behavior, boundary metadata,
  deterministic ordering, and invalid/missing input findings.
- `result hooks`: recovery hooks preserve load identity, hook kind, target
  reference, dimension, and provenance reference for downstream stress
  recovery, reporting, export, GUI, and headless execution work.

Deterministic finding coverage includes missing targets or quantities,
out-of-range node/element indices, invalid dimensions or directions,
unsupported target/load-kind pairings, invalid distribution spans,
non-positive element lengths, missing straight-pipe geometry, and nonfinite
axial-effect inputs.

## References

- `_CONTEXT.md` for sealed deliverable identity, scope, artifacts, context budget, and architecture basis.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 rows for SOW-013, SOW-052, OBJ-003, OBJ-012, PKG-05, DEL-05-05, and AB-00-01/02/03/06/08.
- `docs/_Registers/Deliverables.csv` row DEL-05-05.
- `docs/_Registers/ScopeLedger.csv` rows SOW-052 and SOW-013.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-05-05.
- `docs/CONTRACT.md` rows OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-SOLVER-1, OPS-K-AGENT-1..4.
- `core/loads/user_loads/README.md` for current implementation scope and
  boundary evidence.
- `core/loads/user_loads/src/lib.rs` for implemented API, findings, recovery
  hooks, and unit tests.
