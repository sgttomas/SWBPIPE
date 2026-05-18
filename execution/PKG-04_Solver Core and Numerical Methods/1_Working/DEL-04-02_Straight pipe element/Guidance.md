# Guidance: DEL-04-02 Straight pipe element

## Purpose

This deliverable exists to isolate the straight pipe element as one bounded solver feature slice. It should give the global 3D frame solver a governed, testable element behavior while keeping section data, units, diagnostics, load hooks, and downstream force recovery explicit.

## Principles

- Keep mechanics computation separate from rule-pack acceptability and human compliance judgment.
- Treat dimensions, material values, and protected standard-derived values as external governed inputs, never public bundled defaults.
- Prefer explicit `TBD` and explicit diagnostics over silent assumptions when solve-required values are missing.
- Preserve unit and provenance information through element inputs and outputs.
- Keep weight behavior as an interface hook until the load-case contract determines how loads are formed and applied.

## Considerations

The element will likely depend on upstream contracts for section properties, units, domain objects, application-service result envelopes, primitive load cases, and solver kernel assembly. Those contracts are not resolved in this setup pass.

Verification should use open, synthetic, or cleared test cases. Any hand-check examples must avoid copying protected standard examples, tables, or protected formula presentations.

## Trade-offs

| Topic | Guidance |
|---|---|
| Completeness vs data boundary | Record missing section or material values as findings rather than providing public defaults. |
| Element scope vs solver kernel scope | Keep local element behavior here; keep global assembly, transforms, and sparse solve responsibilities in the solver kernel deliverable unless the accepted implementation boundary says otherwise. |
| Load hooks vs load application | Expose weight-related information without creating hidden primitive load behavior inside the element. |
| Force recovery vs stress checks | Recover mechanical element forces; downstream stress recovery and rule checks remain separate deliverables. |

## Examples

Concrete numerical examples are `TBD`. Future examples must use synthetic or cleared data and must not reproduce protected standards content.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| None | No setup conflict found. | N/A | N/A |
