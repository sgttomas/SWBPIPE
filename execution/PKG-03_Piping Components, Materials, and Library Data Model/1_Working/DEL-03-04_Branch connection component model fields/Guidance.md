# Guidance: DEL-03-04 Branch connection component model fields

## Purpose

This deliverable exists to keep branch connection component data explicit, unit-aware, provenance-bearing, and separated from protected standards content. The model should let users or lawful private libraries supply branch-specific data without the public project embedding protected branch connection, SIF, flexibility, or reinforcement tables.

## Principles

- Treat branch connection data as user/imported input unless a public, rights-cleared source is documented.
- Preserve the difference between data capture, solver mechanics, rule-pack evaluation, and human professional judgment.
- Prefer explicit `TBD` or diagnostics over inferred defaults.
- Keep provenance and redistribution status close to the values they qualify.
- Make dimensional fields unit-aware; do not accept unitless dimensional values unless a later schema deliberately defines a safe representation.

## Considerations

The branch connection model may need more local fields than simpler component families. The decomposition notes that branch local checks may require future specialized modules; this setup pass should therefore avoid overfitting a future schema to a specific code method or formula.

Architecture basis implications:

- AB-00-02 and AB-00-07 point toward domain contracts and validation boundaries rather than GUI or adapter shortcuts.
- AB-00-04 points toward deterministic, provenance-preserving persistence when serialized.
- AB-00-06 points toward structured diagnostics for blocking missing data and boundary warnings.
- AB-00-08 points toward layered tests, including protected-content/provenance gates.

## Trade-offs

| Topic | Guidance |
|---|---|
| Field specificity vs protected content | Capture field categories and user-supplied values; do not encode protected lookup content. |
| Completeness vs implementation timing | Mark exact field names and specialized local-check needs as `TBD` until the implementation brief authorizes product code. |
| Public examples vs validation | Use synthetic, non-code, non-engineering fixtures only when later tests need examples, and label them accordingly. |
| User flexibility vs diagnostics | Permit user-supplied branch data, but report missing or unverifiable values explicitly. |

## Examples

No engineering examples are provided in this setup pass. Future examples, if any, must be invented non-code values with clear non-engineering notices and must not reproduce protected tables, formulas, or code examples.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None identified | No conflict was detected in the accessible setup sources. | N/A | N/A | N/A | N/A | TBD |
