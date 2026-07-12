# Datasheet: DEL-07-02 Model tree and property inspector

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-07-02 |
| Package ID | PKG-07 |
| Package | Graphical User Interface and Engineering Workflow |
| Type | UX_UI_SLICE |
| Scope items | SOW-020, SOW-021 |
| Objective | OBJ-006 |
| Context envelope | M |
| Anticipated artifacts | model tree; property inspector; UI tests |
| Accepted delegation | `DEL-00-05` owns GUI state/interaction architecture; `DEL-07-02` owns model-tree/property-inspector behavior implementation within it (`DEC-074` O2). |

## Attributes

| Attribute | Setup value |
|---|---|
| UI surface | Single GUI work surface for tree navigation and selected-entity property inspection. |
| Primary entities | Project, Model, Node, Element, Component, Material, Section, Support, LoadCase, Combination, RulePackRef, Result, and diagnostics where applicable. |
| GUI baseline | Tauri 2 desktop shell, TypeScript/React/Vite GUI, and Three.js viewport where viewport-facing. Exact component and state libraries are `TBD`. |
| State boundary | Durable project/model state is separate from transient session, viewport, selection, and job-progress state. GUI mutations route through application-service commands. |
| Missing-data posture | Missing solve-required and rule-check-required values are surfaced as explicit findings, not defaulted silently. |
| Data boundary | Private project, material, component, and rule-pack data remain user-controlled and are not transmitted or committed publicly by default. |

## Conditions

| Condition | Status |
|---|---|
| Tree hierarchy and grouping rules | Bounded implementation exists from accepted model identities; broader canvas gestures and full model-tree UX remain residual. |
| Property editor field inventory | Bounded implementation preserves governed unit-bearing fields and metadata; model-level `modulus_basis_records` entry and broader editor coverage remain residual. |
| Selection synchronization contract | Bounded tree/viewport/inspector selection is implemented within the `DEL-00-05` state/interaction architecture; transient state is not durable model truth. |
| Command/query contract | Current structured mutations route through the application-service operation seam; broader contract coverage remains governed residual work. |
| UI tests | Focused evidence covers bounded tree, selection, and inspector behavior; backfill and factoring for `PropertyInspector` and adjacent surfaces remain residual. |

## Construction

This kit records the bounded UI behavior already implemented and the remaining delivery boundary. `DEC-074` O2 assigns model-tree/property-inspector behavior implementation to `DEL-07-02` within the GUI state/interaction architecture owned by `DEL-00-05`; it does not transfer architecture authority, broaden functionality, choose unresolved UI libraries, introduce engineering default values, or move any artifact to `ISSUED`.

The model tree and property inspector consume accepted schema/service contracts for object identity, unit-bearing fields, provenance, diagnostics, rule-pack/private-library status, and command/query/result-envelope behavior to the extent supported by current bounded evidence. Missing or unresolved engineering data remains visible as `TBD` or diagnostic state.

## Ownership Boundary

- Architecture owner: `DEL-00-05` for GUI state/interaction architecture.
- Behavior owner: `DEL-07-02` for bounded model-tree/property-inspector implementation within that architecture.
- Adjacent scopes unchanged: `DEL-07-03` owns specialized material/component/rule-pack editors; `DEL-07-04` owns missing-data warning/blocking UX.
- Ruling basis: `DEC-074` option O2, resolving `PDU-009` without scope expansion.

## Setup Slot Checklist

| Slot | Setup status | Owner/source status |
|---|---|---|
| Tree grouping and hierarchy rules | `TBD` | Expected from accepted domain/schema and GUI state contracts. |
| Inspector field inventory | `TBD` | Expected from accepted domain/schema contracts for model entities and library references. |
| Unit display and edit hooks | `TBD` | Expected from accepted unit and command/query contracts. |
| Provenance and redistribution/private-status display | `TBD` | Expected from material/component/rule-pack/library contracts. |
| Diagnostic classes and affected-object display | `TBD` | Expected from diagnostics/result-envelope contract. |
| Fixture, screenshot, and UI test data policy | `TBD` | Must remain synthetic, public-domain, or otherwise cleared. |

## References

- `_CONTEXT.md` for deliverable identity, scope, artifacts, and architecture-basis injection.
- `docs/_Registers/Deliverables.csv` row `DEL-07-02`.
- `docs/_Registers/ScopeLedger.csv` rows `SOW-020` and `SOW-021`.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, `PKG-07`, `OBJ-006`, and architecture basis IDs `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
- `docs/CONTRACT.md` invariants `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`, `OPS-K-UNIT-1`, `OPS-K-RULE-1`, `OPS-K-RULE-3`, `OPS-K-PRIV-1`, `OPS-K-PRIV-2`, `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`, and `OPS-K-AGENT-1..4`.
- `docs/SPEC.md` sections 1, 3, 6, 7, 10, and 11.
- `docs/TYPES.md` sections 3, 4, 5, 6, 7, 8, and 9.
- `execution/_Coordination/_DECISIONS/D-41_R4_RULING_2026-07-12.md`, `DEC-074` option O2.

## Open Setup Questions

| Question | Status |
|---|---|
| Which accepted schema version supplies the property inspector field inventory? | `TBD` |
| Which GUI state library, if any, is accepted for transient selection and inspector state? | `TBD` |
| Which application-service commands and queries are accepted for model tree edits and property reads? | `TBD` |
| Which diagnostics contract shape is accepted for inline missing-data and provenance warnings? | `TBD` |
| Which UI test framework conventions apply to this slice beyond the architecture-basis Playwright/Vitest expectation? | `TBD` |
