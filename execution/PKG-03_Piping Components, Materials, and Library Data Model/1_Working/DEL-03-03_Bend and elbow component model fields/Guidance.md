# Guidance: DEL-03-03 Bend and elbow component model fields

## Purpose

This deliverable prepares a bounded evidence kit for a bend/elbow component model slice. The important boundary is that OpenPipeStress can store and validate user-entered or lawfully imported bend/elbow data, but must not publish protected SIF/flexibility tables, formulas, examples, or default code-derived values.

## Principles

- Treat bend/elbow SIFs and flexibility factors as user/private/library inputs with provenance, not public defaults.
- Make absent required values visible through validation findings.
- Keep unit metadata close to numeric geometry fields.
- Keep source metadata close to any data that may have licensing, redistribution, or protected-content implications.
- Preserve the distinction between model data, solver mechanics, rule-pack evaluation, and human engineering judgment.

## Considerations

The model should be designed so later solver and rule workflows can consume bend/elbow fields without crossing the protected-data boundary. Where exact schema names, enum values, units, or persistence formats are not yet fixed by upstream architecture, record `TBD` rather than making an implementation claim in setup evidence.

Diagnostics should be able to distinguish missing geometry from missing SIF/flexibility inputs and from missing provenance. This matters because the later response may be different: solve-blocking, rule-check-blocking, provenance warning, assumption warning, or IP-boundary warning.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Convenience defaults vs protected-data safety | Prefer no default over any value that could imply bundled code data. |
| Flexible generic component record vs bend-specific fields | Use bend-specific slots where validation, units, or provenance differ materially from generic components. |
| Public examples vs realistic engineering examples | Use invented non-code values only, and label them as non-engineering examples. |
| Early schema detail vs architecture TBDs | Keep field concepts stable, but leave exact names and formats `TBD` until implementation authority exists. |

## Examples

- Acceptable setup statement: "The model stores user-entered SIFs with provenance."
- Not acceptable: a copied or paraphrased table of SIF/flexibility factors, code formulas, or standard examples.
- Acceptable test fixture posture: invented values used only to test schema shape, units, provenance, and diagnostics.

## Conflict Table (for human ruling)

| Conflict ID | Description | Contenders | Human Ruling |
|---|---|---|---|
| None | No source conflict identified in this setup pass. | N/A | N/A |
