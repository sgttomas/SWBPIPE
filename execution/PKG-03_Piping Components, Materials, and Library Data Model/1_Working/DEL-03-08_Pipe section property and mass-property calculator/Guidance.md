# Guidance: DEL-03-08 Pipe section property and mass-property calculator

## Purpose

This deliverable exists to create a bounded, unit-aware calculation surface for pipe section and mass properties while preserving OpenPipeStress data-boundary rules. The key setup distinction is that calculation capability is allowed, but bundled protected pipe tables, material tables, proprietary data, and silent defaults are not.

## Principles

Use user-entered dimensions and material data as the source of truth. If a value is missing, incompatible, unprovenanced, or not licensed for redistribution, record a finding or warning rather than substituting a default.

Keep the calculator code-neutral. Section and mass properties can support solver and reporting workflows, but this deliverable does not determine code compliance, professional acceptance, or certification.

Keep unit behavior explicit. The calculator should consume the accepted unit-system contract rather than implementing ad hoc unit conversion rules.

Keep provenance attached. If input values arrive from private libraries, imports, or user records, the output/schema hooks should preserve enough source and redistribution status for downstream review.

## Considerations

The most important implementation risk is accidentally turning public fixtures or defaults into a protected data table. Synthetic examples may be used for tests, but they must not be copied from protected standards, commercial catalogs, or proprietary project records.

The second major risk is ambiguity about optional contributors such as contents, insulation, and corrosion basis. Future implementation must define when each contributor is required, optional, explicitly not applicable, or pending user input.

The third risk is unit drift between schema, UI, calculator, and solver-facing outputs. This deliverable should rely on schema-first envelopes and the unit core once those contracts are available.

## Trade-offs

| Decision area | Conservative posture |
|---|---|
| Public default data | Do not ship defaults where provenance or redistribution rights are unclear. |
| Test fixtures | Use synthetic values designed for dimensional behavior, not copied industry tables. |
| Calculator scope | Calculate section/mass properties only; leave code checks and solver verification to their packages. |
| Schema timing | Treat hook names and exact field placement as `TBD` until schema contracts are accepted. |

## Examples

Specific numerical examples are intentionally `TBD` in this setup pass. Future examples must be synthetic or clearly licensed/user-provided and must include units, provenance posture, and expected diagnostic behavior.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Current disposition | Human ruling |
|---|---|---|---|
| None | No conflict identified between the scoped decomposition, registers, and contract invariants. | N/A | N/A |

## P3 Enrichment Notes

Semantic lensing identified that later implementation should make diagnostic taxonomy, schema hook names, and optional mass contributors explicit. Those remain `TBD` because no accepted source in this setup context defines them.
