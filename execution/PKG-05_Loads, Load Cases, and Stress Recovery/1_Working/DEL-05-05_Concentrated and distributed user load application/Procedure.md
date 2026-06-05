# Procedure: DEL-05-05 Concentrated and distributed user load application

## Purpose

Define the operational path for validating and extending the implemented
DEL-05-05 user-load slice without crossing the deliverable boundary.

## Prerequisites

- Sealed task brief for DEL-05-05 with explicit write scope.
- Access to `_CONTEXT.md`, `_REFERENCES.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, relevant register rows, and `docs/CONTRACT.md`.
- Access to implementation evidence in `core/loads/user_loads/README.md` and
  `core/loads/user_loads/src/lib.rs`.
- Any extension task must resolve or explicitly inherit applicable unit,
  solver, schema, diagnostics, and result-envelope contracts.

## Steps

1. Confirm the task is scoped to DEL-05-05 and does not request code-specific
   combinations, default factors, protected standards data, allowables,
   compliance conclusions, or professional approval.
2. Identify whether the change concerns generic `apply_user_loads`,
   straight-pipe equivalent recovery, the axial-effect bridge, documentation,
   or downstream integration.
3. Preserve SOW-052 support for concentrated force, concentrated moment, and
   distributed user-load inputs while treating SOW-013 only as primitive
   load-case context; do not reimplement DEL-05-01.
4. Route numeric load values, dimensions, units, directions, target references,
   station fractions, spans, element lengths, provenance references, schema
   bindings, payload references, and payload-hash references through explicit
   validation or boundary records.
5. For generic application, confirm nodal concentrated forces and moments emit
   nodal contributions and recovery hooks, element distributed loads emit
   distributed contribution records and hooks, and element-station loads remain
   blocked unless straight-pipe geometry is supplied.
6. For straight-pipe equivalent recovery, confirm full-span and partial-span
   translational distributed loads, oriented global directions, and
   element-station concentrated forces route through `StraightPipeElement`
   equivalent-load helpers.
7. For axial-effect bridge behavior, accept only already-prepared
   `PrimitiveAxialEffectContribution` records and preserve findings for wrong
   element indices, nonfinite axial effects, and missing straight-pipe
   geometry.
8. Preserve architecture-basis constraints AB-00-01, AB-00-02, AB-00-03,
   AB-00-06, and AB-00-08 in design notes, diagnostics, and validation.
9. Run deterministic validation before closeout, at minimum
   `cargo test --manifest-path core/loads/user_loads/Cargo.toml` for changes
   that touch or document current implementation behavior.
10. Expose result recovery hooks with provenance/status fields sufficient for
    downstream stress recovery, reporting, export, GUI, and headless execution
    work, without compliance claims.

## Verification

- Four deliverable documents exist and consistently reflect the implemented
  evidence while excluding code-specific default behavior.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist as lens artifacts only.
- `Dependencies.csv` validates against v3.1 schema.
- `_STATUS.md` remains read-only unless an explicit authorized status-change
  task is dispatched.
- `cargo test --manifest-path core/loads/user_loads/Cargo.toml` is the
  targeted validation gate for the documented implementation surface; any
  failure is a blocker for code-level acceptance unless separately waived by a
  human ruling.
- `git diff --check` passes for touched deliverable-local files.

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `MEMORY.md`
- `core/loads/user_loads/README.md`
- `core/loads/user_loads/src/lib.rs`
- `_run_records/TASK_RUN_2026-04-30_1023_*.md`
