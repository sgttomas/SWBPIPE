# Specification: DEL-05-05 Concentrated and distributed user load application

## Scope

This deliverable covers the implemented backend feature slice that applies
explicit concentrated forces, concentrated moments, and uniform distributed user
loads with unit-aware application records and result recovery hooks.

Implemented evidence is the bounded Rust crate at `core/loads/user_loads`.
The crate provides:

- generic user-load preparation through `apply_user_loads`;
- straight-pipe equivalent-load recovery through
  `apply_straight_pipe_equivalent_user_loads`;
- partial-span and oriented straight-pipe handling for translational
  distributed loads;
- element-station concentrated force recovery for straight-pipe elements;
- an axial-effect bridge through
  `apply_straight_pipe_equivalent_user_loads_with_axial_effects`;
- deterministic findings for missing, invalid, unsupported, or unresolved
  inputs.

This deliverable excludes:

- code-specific load combinations, factors, or default values;
- protected standard content or proprietary allowables;
- certification, compliance, or professional approval claims.
- final result-envelope/API/persistence/GUI/CLI/report integration unless
  separately dispatched.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-05-05-R1 | Support concentrated force, concentrated moment, and distributed user-load categories as explicit user load inputs. | ScopeLedger.csv row SOW-052; SOFTWARE_DECOMP.md row SOW-052 |
| DEL-05-05-R2 | Keep user loads separate from code-specific combinations supplied by users or rule packs. | Deliverables.csv row DEL-05-05; ScopeLedger.csv row SOW-052 |
| DEL-05-05-R3 | Keep load input, application records, and result handoff unit-aware and dimensionally checked. | CONTRACT.md OPS-K-UNIT-1; ScopeLedger.csv SOW-025 context via OBJ-012 |
| DEL-05-05-R4 | Attach load application to the centerline/frame mechanics model and avoid implying shell/solid FEA is the primary global model. | CONTRACT.md OPS-K-MECH-1 |
| DEL-05-05-R5 | Provide deterministic verification tests before release use. | CONTRACT.md OPS-K-SOLVER-1; architecture basis AB-00-08 |
| DEL-05-05-R6 | Preserve status/provenance distinctions in diagnostics and result hooks without certification or compliance claims. | architecture basis AB-00-03 and AB-00-06; CONTRACT.md OPS-K-MECH-2 |
| DEL-05-05-R7 | For straight-pipe elements, recover full-span and valid partial-span translational distributed loads into equivalent global nodal contributions. | `core/loads/user_loads/src/lib.rs`; `core/loads/user_loads/README.md` |
| DEL-05-05-R8 | For straight-pipe elements, recover element-station concentrated forces while rejecting station moments and invalid station fractions as findings. | `core/loads/user_loads/src/lib.rs` |
| DEL-05-05-R9 | Preserve oriented straight-pipe behavior by treating `GlobalX`, `GlobalY`, and `GlobalZ` user-load directions as global force vectors before straight-pipe projection. | `core/loads/user_loads/src/lib.rs` |
| DEL-05-05-R10 | Bridge already-prepared primitive axial-effect contributions into straight-pipe equivalent nodal loads without duplicating axial-effect mechanics in this crate. | `core/loads/user_loads/src/lib.rs` |

## Standards

- Governing invariant catalog: `docs/CONTRACT.md`, location rows listed in `_CONTEXT.md` and this document.
- Decomposition basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- Public schema/interchange baseline: JSON Schema 2020-12 from architecture
  basis injection. Current implementation binds model-load records to
  `schemas/model.schema.yaml#/$defs/LoadRecord` and recovery records to model
  result values or `schemas/results.schema.yaml#/$defs/QuantityResult`.
- Code-specific rules, combinations, and allowables: excluded from bundled defaults; user/rule-pack supplied.

## Verification

| Requirement | Verification approach |
|---|---|
| DEL-05-05-R1 | Existing Rust unit tests cover concentrated force, concentrated moment, and uniform distributed load input/application paths. |
| DEL-05-05-R2 | Existing boundary text and tests preserve the no-default/no-compliance boundary; attempts to embed code combinations remain outside this module. |
| DEL-05-05-R3 | Existing unit tests exercise `LoadDimension`, `ForcePerLength` metadata, explicit unit metadata, schema binding, payload references, and payload-hash references. |
| DEL-05-05-R4 | Existing generic contribution records use frame DOFs; straight-pipe recovery uses `StraightPipeElement` equivalent global nodal loads. |
| DEL-05-05-R5 | Existing deterministic test suite runs with `cargo test --manifest-path core/loads/user_loads/Cargo.toml`. |
| DEL-05-05-R6 | Existing result-hook tests verify schema-binding separation and provenance-preserving records without compliance claim language. |
| DEL-05-05-R7 | Existing tests verify full-span and partial-span straight-pipe distributed-load equivalent recovery. |
| DEL-05-05-R8 | Existing tests verify interior station point-force recovery and deterministic rejection of unsupported station moments or invalid station fractions. |
| DEL-05-05-R9 | Existing tests verify global-direction transformation for a straight pipe aligned with global `Y`. |
| DEL-05-05-R10 | Existing tests verify axial-effect contribution inclusion, wrong-element rejection, and nonfinite axial-effect rejection. |

## Documentation

Current implementation records include:

- `core/loads/user_loads/README.md` for crate scope, boundaries, and verification summary;
- `core/loads/user_loads/src/lib.rs` for API, findings, contribution records, recovery hooks, and tests;
- deliverable-local `MEMORY.md` for historical implementation slices, validation runs, and remaining TBDs.

Remaining TBDs are final result-envelope/API/persistence/GUI/CLI/report
integration, production tolerance policy, release thresholds, primitive
axial-effect provenance beyond `load_id`, and professional reliance.
