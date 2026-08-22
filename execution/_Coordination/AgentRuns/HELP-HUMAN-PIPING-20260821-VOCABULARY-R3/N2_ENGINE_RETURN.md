# N2 engine return — expansion-joint structured creation

- RunID: `HELP-HUMAN-PIPING-20260821-VOCABULARY-R3`
- Instance: `WORKING-ITEMS-VOCAB-R3-N2-ENGINE`
- Role: `WORKING_ITEMS`; package `PKG-16`; deliverable `DEL-16-01`
- Accepted basis: N1 commit `8ca1984db45a9a8f6f3111a905b07c7d3da47c33`
- Runtime attribution: inherited GPT-5.6 Codex runtime; no substitution
- Result: `PASS`
- Integrated-review posture: `READY`

## Implemented scope

`insert_component_symbol` now resolves `expansion_joint` through the same structured-operation applier seam as the accepted N1 component kinds. The resolver requires, validates, and persists only explicit user inputs:

- one selected existing pipe span incident to the selected component node;
- finite positive `effective_area` and `movement_limit` quantities with accepted area and length units;
- non-empty hardware, manufacturer, pressure-thrust, and expansion-joint geometry-source references;
- finite positive axial/lateral linear-stiffness and angular/torsional rotational-stiffness quantities;
- one non-empty stiffness `source_reference`.

The positive regression proves exact selected pipe, reference strings, quantity values, and entered units reach the applied model. No pipe role, unit, reference, stiffness, area, movement limit, mechanics-interface mode, or other engineering value is inferred. No geometry/connectivity contract question arose.

## Blocking evidence

Blocking regressions cover:

- missing geometry reference and missing axial-stiffness data;
- an unknown selected pipe and an existing but nonincident selected pipe;
- area and rotational-stiffness dimension mismatches;
- a nonpositive torsional stiffness value.

Every blocking case asserts `application_status = blocked` and `applied_model = None`; reference and unit state assertions are included where applicable. Existing bend, tee (including distinct header/branch enforcement), reducer, valve, and flange creation regressions remain green.

## Validation

- Focused expansion-joint slice: `4 passed`.
- Full component-creation slice: `11 passed`, including all N1 component kinds.
- Full operation-applier crate: `86` unit tests plus `3` integration tests passed; doc tests passed.
- `wasm32-unknown-unknown` release build with feature `wasm`: PASS.
- `cargo fmt --check`: PASS.
- Scoped `git diff --check`: PASS.

Exact commands and hashes are recorded in `N2_ENGINE_CHECKS.json`.

## Hash inventory

| Surface | SHA-256 |
|---|---|
| Accepted-basis `lib.rs` content | `f6ecdc476482d8798591689770c4a87a318ec4765e0362b886ae46922b58be7f` |
| Current `lib.rs` content | `13246f843fa9673b2de3fe8758695672d974f91bdd9be03e2e8b0698247f5bc8` |
| Binary basis-to-current Git diff | `6f2fb76572c079ffc5c5c2b1f12a05dc687602cc346124885150ec93c1ba109e` |

## Containment and handoff

Product writes are confined to `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`. The only other writes are these two authorized run-local return/check files. UI, shared status, vocabulary coverage, handoff, receipt, and Git surfaces were not changed by this lane.

This is validated bounded engine evidence, not lifecycle or release closure. Requested parent action: fan in with the N2 UI return and dispatch the required fresh read-only 100%-diff integrated review.
