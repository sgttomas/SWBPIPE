# Guidance: DEL-05-01 Primitive load case engine

## Purpose

This deliverable frames primitive load categories as mechanics inputs for the OpenPipeStress solver. The setup evidence keeps the category surface visible while deferring product implementation, coefficients, defaults, and code-specific combination logic.

## Principles

- Treat primitive load categories as category definitions and validation boundaries, not as a finished load-combination engine.
- Keep mechanics solving distinct from user-rule evaluation and human professional judgment.
- Preserve unit awareness for all load quantities that carry dimensions.
- Use explicit `TBD` entries for missing values, coefficients, dynamic treatment, and source policies.
- Avoid copying or deriving protected standard content, tables, examples, coefficients, or proprietary defaults.

## Considerations

| Topic | Guidance |
|---|---|
| Weight | Future work should identify lawful mass-property sources and gravity handling without inventing defaults here. |
| Pressure | Pressure belongs in mechanics input handling; code stress category mapping belongs elsewhere. |
| Thermal | Thermal input policy needs reference-temperature and material-property provenance before implementation. |
| Displacement | Imposed displacement must coordinate with support/restraint behavior without expanding this deliverable into support implementation. |
| Hydrotest | Hydrotest load setup must not imply a hydrotest code procedure or default fluid state without sourced input. |
| Wind/seismic/occasional | Environmental/event categories may require user/rule-supplied parameters and possibly deferred dynamic modules. |
| Diagnostics | Missing or invalid load inputs should become explicit findings with provenance rather than silent defaults. |

## Trade-offs

| Trade-off | Setup position |
|---|---|
| Category coverage vs. implementation detail | Cover all SOW-013 primitive categories, but leave schemas, algorithms, and fixtures TBD. |
| Public defaults vs. user-supplied data | Do not bundle protected or jurisdictional values; require lawful/user-supplied inputs later. |
| Static placeholders vs. dynamic methods | Dynamic methods for wind, seismic, and occasional loads remain TBD until a sealed implementation scope resolves them. |
| Load definitions vs. load combinations | Keep primitive load definitions in DEL-05-01 and load-case algebra/user combinations in DEL-05-02. |

## Examples

TBD. No numeric load examples, code-specific combinations, or coefficient examples are introduced during setup.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected in setup evidence. | N/A | N/A | N/A | N/A | N/A |
