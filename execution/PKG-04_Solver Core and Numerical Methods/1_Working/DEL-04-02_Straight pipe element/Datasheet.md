# Datasheet: DEL-04-02 Straight pipe element

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-02 |
| Package ID | PKG-04 |
| Package | Solver Core and Numerical Methods |
| Type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-006 |
| Objective | OBJ-003 |
| Context envelope | M |
| Anticipated artifacts | straight pipe element; solver tests |

## Attributes

| Attribute | Setup value |
|---|---|
| Element family | Straight pipe centerline/frame element |
| Analysis model boundary | 3D centerline/frame mechanics with six degrees of freedom per node; shell/solid FEA remains a local-analysis handoff path. |
| Covered mechanics | Local stiffness, section-property integration, weight hooks, and element force recovery. |
| Excluded mechanics | Code compliance decisions, rule-pack acceptability, protected standard formulas/tables, and repo-bundled protected dimensional or material values. |
| Unit policy | All inputs, intermediate values, and outputs are unit-aware and dimensionally checked. Exact unit API is `TBD`. |
| Data provenance policy | Pipe dimensions, material values, and other solve inputs are user-supplied or lawfully imported private/project data. |

## Conditions

| Condition | Status |
|---|---|
| Solver numerical library | `TBD` from architecture/implementation decision. |
| Section-property source contract | `TBD`; must connect to user/project data or lawful library inputs. |
| Weight-load integration contract | `TBD`; must remain a hook to load-case behavior, not a hidden default load application. |
| Element force recovery conventions | `TBD`; must be unit-aware and consistent with solver result envelopes. |
| Test fixture data | Must be synthetic, public-domain, or otherwise cleared for repository use. |

## Construction

The setup kit describes the future implementation boundary only. It does not implement solver code, choose element dimensions, choose material values, encode protected formulas, or create repo-level tests.

The future element is expected to receive validated geometry, section properties, material/mechanical inputs, and load hooks through governed domain/service contracts. Missing solve-required values must produce explicit findings rather than silent defaults.

## References

- `_CONTEXT.md` for deliverable identity, scope, artifacts, and architecture-basis injection.
- `docs/_Registers/Deliverables.csv` row `DEL-04-02`.
- `docs/_Registers/ScopeLedger.csv` row `SOW-006`.
- `docs/_Registers/ContextBudgetQA.csv` row `DEL-04-02`.
- `docs/CONTRACT.md` invariants listed in the sealed brief.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 architecture basis IDs `AB-00-01`, `AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, and `AB-00-08`.

## Open Setup Questions

| Question | Status |
|---|---|
| Which upstream schema owns straight-pipe section-property inputs? | `TBD` |
| Which solver-kernel interface owns local-to-global transformation and assembly handoff? | `TBD` |
| Which load engine interface receives or invokes weight hooks? | `TBD` |
| Which deterministic verification cases are accepted for the element without protected data? | `TBD` |
