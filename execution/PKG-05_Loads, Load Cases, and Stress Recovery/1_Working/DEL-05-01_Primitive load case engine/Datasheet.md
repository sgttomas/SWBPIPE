# Datasheet: DEL-05-01 Primitive load case engine

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-05-01 |
| Name | Primitive load case engine |
| Package | PKG-05 Loads, Load Cases, and Stress Recovery |
| Type | BACKEND_FEATURE_SLICE |
| Scope items | SOW-013 |
| Objectives | OBJ-003 |
| Context envelope | M |

## Attributes

| Attribute | Setup value | Source |
|---|---|---|
| Primary subject | Primitive load definitions for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional categories. | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` row DEL-05-01; `docs/_Registers/ScopeLedger.csv` row SOW-013 |
| Analysis model boundary | Primitive loads are inputs to a 3D centerline/frame mechanics model. | `docs/CONTRACT.md` OPS-K-MECH-1 |
| Mechanics/rule boundary | This deliverable defines mechanics load categories and application evidence; it does not evaluate user-rule acceptability or professional compliance. | `docs/CONTRACT.md` OPS-K-MECH-2 |
| Unit boundary | Load magnitudes, temperature changes, pressures, displacements, accelerations, and distributed quantities are unit-aware and dimensionally checked. | `docs/CONTRACT.md` OPS-K-UNIT-1 |
| Data boundary | Code-specific defaults, load factors, combinations, allowables, and jurisdictional coefficients are not bundled public defaults. | `docs/CONTRACT.md` OPS-K-DATA-1; OPS-K-IP-1 |
| Missing values | Missing solve-required load inputs are explicit findings rather than silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2 |
| Dynamic scope | Dynamic modules may be deferred. | `docs/_Registers/ScopeLedger.csv` row SOW-013 |

## Primitive Load Category Register

| Category | Setup meaning | Open item |
|---|---|---|
| Weight | Gravity-related load definition for piping/component mass effects. | Mass-source, gravity-vector, and density provenance policy TBD. |
| Pressure | Internal or external pressure load definition for mechanics solving. | Pressure-to-result coupling remains bounded to mechanics, not code stress evaluation. |
| Thermal | Temperature-change or thermal-expansion load definition. | Reference temperature and thermal-property source policy TBD. |
| Displacement | Imposed displacement load or boundary-motion input. | Relationship to restraint/support deliverables remains interface-dependent. |
| Hydrotest | Hydrotest load category for mechanics analysis setup. | Hydrotest fluid/procedure defaults are not invented. |
| Wind | Environmental wind load category placeholder. | Any coefficients, profiles, or code factors are user/rule-supplied TBD. |
| Seismic | Seismic load category placeholder. | Dynamic treatment and response parameters TBD; dynamic modules may be deferred. |
| Occasional | Occasional load category placeholder. | Event definition and user/rule mapping TBD. |

## Conditions

- Primitive load categories remain separate from code-specific load combinations and allowables. Source: `_CONTEXT.md`; PKG-05 exclusions; SOW-014 note.
- Solver changes require deterministic verification tests before release. Source: `docs/CONTRACT.md` OPS-K-SOLVER-1.
- Agent-authored setup evidence is draft/proposal material until human acceptance. Source: `docs/CONTRACT.md` OPS-K-AGENT-4.
- Unknown coefficients, default magnitudes, and jurisdictional values remain `TBD`. Source: OPS-K-AGENT-1; OPS-K-DATA-1; OPS-K-IP-1.

## Construction

| Artifact | Description | Setup status |
|---|---|---|
| Load case model | Domain/backend model surface for primitive load definitions and category identity. | TBD implementation path |
| Load application tests | Deterministic tests for applying primitive load categories to analysis inputs without code-specific combinations. | Required in principle; fixtures TBD |
| Diagnostic/result envelope hooks | Load input findings should fit architecture-basis diagnostics and result envelopes. | Governed by AB-00-03 and AB-00-06 |
| Unit validation checks | Checks that primitive load values carry expected dimensions. | Required in principle; exact schemas TBD |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4
- `docs/_Registers/Deliverables.csv` row DEL-05-01
- `docs/_Registers/ScopeLedger.csv` row SOW-013
- `docs/_Registers/ContextBudgetQA.csv` row DEL-05-01
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
