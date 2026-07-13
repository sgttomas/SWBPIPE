# Guidance: DEL-03-08 Pipe section property and mass-property calculator

## Purpose

This deliverable provides a bounded, unit-aware calculation surface for pipe section and mass properties while preserving OpenPipeStress data-boundary rules. The key distinction is that calculation from explicit user-entered values is allowed, but bundled protected pipe tables, material tables, proprietary data, hidden unit conversion, and silent defaults are not.

## Principles

Use user-entered dimensions and material data as the source of truth. If a required value is missing, incompatible, unprovenanced, or not licensed for redistribution, record a blocking diagnostic rather than substituting a default.

Keep the calculator code-neutral. Section and mass properties can support solver and reporting workflows, but this deliverable does not determine code compliance, professional acceptance, or certification.

Keep unit behavior explicit. The calculator currently requires matching units at the calculation boundary and rejects mixed units; approved unit conversion support remains a separate `TBD`.

Keep provenance attached. Calculator inputs require provenance metadata, and calculated outputs identify that they were derived from user-entered dimensions. Exact private-library record linkage and downstream envelope mapping remain `TBD`.

## Considerations

The most important implementation risk is accidentally turning public fixtures or defaults into a protected data table. Synthetic examples may be used for tests, but they must not be copied from protected standards, commercial catalogs, or proprietary project records.

The second major risk is ambiguity about optional contributors such as contents, insulation, and corrosion basis. Current code treats these contributors as optional explicit inputs; downstream policy still must define when each contributor is required, optional, explicitly not applicable, or pending user input.

The third risk is unit drift between schema, UI, calculator, and solver-facing outputs. The calculator emits canonical dimension labels in current tests, but accepted schema ownership, dependency satisfaction, and downstream result-envelope integration remain `TBD`.

## Trade-offs

| Decision area | Conservative posture |
|---|---|
| Public default data | Do not ship defaults where provenance or redistribution rights are unclear. |
| Test fixtures | Use synthetic values designed for dimensional behavior, not copied industry tables. |
| Calculator scope | Calculate section/mass properties only; leave code checks and solver verification to their packages. |
| Schema timing | Treat hook names and exact field placement as `TBD` until schema contracts are accepted. |
| Review findings | Treat technical evidence as addressed pending human disposition; do not mark findings resolved without human action. |

## Examples

Current numerical examples live in `tests/test_section_properties.py` and use invented synthetic values to verify section properties, mass-per-length contributors, mixed-unit rejection, missing provenance, and invalid geometry. Additional examples must remain synthetic or clearly licensed/user-provided and must include units, provenance posture, and expected diagnostic behavior. Formal fixture-value policy remains `TBD`.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Current disposition | Human ruling |
|---|---|---|---|
| PKG03-DEL-03-08-PKG02-001 | Dimension-vocabulary compatibility has technical evidence but no human disposition. | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | `TBD` |
| PKG03-DEL-03-08-PKG02-002 | Input-provenance handling has technical evidence but no human disposition. | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | `TBD` |
| PKG03-DEL-03-08-PKG02-003 | Diagnostic-envelope fields have technical evidence but no human disposition. | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | `TBD` |

## P3 Enrichment Notes

Semantic lensing identified that diagnostic taxonomy, schema hook names, and optional mass contributors needed explicit treatment. The calculator now emits blocking diagnostics with diagnostic class, source, affected object, and provenance, and tests cover optional mass contributors when explicit densities are supplied. Accepted schema hook names, optional contributor requiredness policy, source catalog, fixture-value policy, dependency satisfaction, lifecycle disposition, human disposition, and downstream integration remain `TBD`.

## D-41 R5 T2B PDU-047 Boundary

The TP-PHYS-015 binding is deliberately narrow: it demonstrates that selected existing rights-safe section-property oracle values traverse the actual Python calculator and an existing governed result envelope. It does not approve catalogs, conversions, thresholds, broader mechanics suitability, or professional reliance.
