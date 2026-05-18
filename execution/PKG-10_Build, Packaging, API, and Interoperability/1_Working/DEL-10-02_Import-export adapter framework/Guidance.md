# Guidance: DEL-10-02 Import/export adapter framework

## Purpose

This deliverable prepares the import/export adapter framework so later implementation work can add adapters without weakening OpenPipeStress governance boundaries. The framework is an extension surface, not a route around domain validation, unit safety, provenance, rule-pack sandboxing, reporting controls, or professional-responsibility limits.

## Principles

- Treat adapters as translators at the edge of the application service boundary.
- Require schema-first envelopes for nontrivial import/export operations.
- Validate units and dimensional meaning before external data becomes domain data.
- Preserve source/provenance, redistribution status, and private/public data markings.
- Emit diagnostics for missing fields, missing provenance, unclear redistribution, suspected protected content, unit inconsistencies, and private-data export risk.
- Keep concrete format support `TBD` until a human decision records the external format, license posture, redistribution status, and test obligations.
- Use invented data only for public samples.

## Considerations

An adapter may be technically able to parse many files, but parsing does not establish redistribution rights, engineering suitability, code compliance, or professional acceptance. The framework should therefore separate:

- syntactic parse success;
- schema and unit validation;
- provenance and redistribution review;
- mechanics-readiness;
- rule-check-readiness;
- human-review-needed state.

Adapters that import private material libraries, component records, rule-pack references, or project data should default to local/private handling. Export operations should warn before writing private or protected-suspected values to shared payloads.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Flexible adapter ecosystem vs. governance control | Favor a narrow framework with mandatory validation hooks; add format-specific adapters later. |
| User convenience vs. data provenance | Missing source or redistribution status is a finding, not an auto-filled default. |
| Broad public formats vs. IP safety | Do not bundle protected or proprietary defaults without documented rights and human approval. |
| Fast export vs. auditability | Exports should carry diagnostics, hashes/manifests where applicable, and warning state. |

## Examples

- Acceptable public sample: an invented adapter that imports a small invented component record with invented dimensions, invented source metadata, and permissive redistribution marked as invented/original.
- Not acceptable public sample: a bundled adapter fixture copied from a standards table, vendor catalog, commercial software example, or private project library without documented redistribution rights.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No current conflicts detected in setup sources. | NA | NA | NA | NA | TBD |
