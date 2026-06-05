# Guidance: DEL-03-01 Material library schema with provenance

## Purpose

This deliverable exists so OpenPipeStress can support piping-specific private material libraries without shipping protected material tables, proprietary commercial data, or unreviewed public data. The schema should make provenance, rights, units, and completeness visible enough for later validation, diagnostics, and review gates.

## Principles

- Treat material values and allowables as governed data, not as free public defaults.
- Separate schema slots from data content: defining an `allowable` field is allowed; populating public tables of protected allowables is not.
- Preserve source and rights metadata with every material value that could affect solving, rule checking, reporting, or downstream reliance.
- Use `TBD` for unresolved public source catalogs, accepted fixture value policy, interpolation policy, allowable storage policy, dependency satisfaction, and human review dispositions.
- Prefer explicit warnings and blocked states over silent fallbacks when required material data is absent or untrusted.

## Considerations

The implemented schema already distinguishes private/user-supplied values, public-permissive reviewed values, invented non-engineering fixture evidence, suspected protected content, and unresolved `TBD` states. Private records may carry user-entered or lawfully imported values, while public repository fixtures still require documented redistribution rights and review disposition before any real values are accepted.

The current invented fixture is schema-shape evidence. Its source/license and redistribution fields intentionally remain `TBD`, and that `TBD` status is not accepted public material data. The fixture omits engineering values and carries blocking diagnostics to demonstrate missing-data behavior.

The schema is structured so importers and adapters have provenance, redistribution, unit, and diagnostic fields to preserve. Concrete import/export formats and adapter behavior remain downstream work.

## Trade-offs

| Decision area | Tension | Current position |
|---|---|---|
| Public examples | Useful for tests and documentation, but risky if derived from protected standards or proprietary libraries. | The current fixture is invented/schema-shape evidence with omitted values. Real public material values require later source and redistribution review. |
| Required field strictness | Strict requirements improve safety but may block partial private libraries. | Required-for-solving/checking values produce explicit completeness findings and diagnostics when absent. Additional policy strictness remains downstream. |
| Source citations | Detailed source pointers improve traceability but can expose protected content if mishandled. | Store source/provenance pointers and rights status; do not reproduce protected text/tables. |
| Allowable values | Needed for some checks, but code-specific tables are protected or user-governed. | Provide schema slots and diagnostics; do not bundle public allowable tables. |

## Examples

The current public fixture is `fixtures/material/invented_material_library_valid.json`. It is not accepted material data for engineering use: it is an invented schema fixture with omitted values, `TBD` source/license disposition, and explicit blocking diagnostics.

Any later material editor fixture that supplies values must use invented non-engineering values or rights-cleared public-permissive data with documented review disposition. It must not include protected material allowable tables, copied standards examples, proprietary library data, or invented values presented as engineering data.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict found in accessible setup sources. | N/A | N/A | N/A | N/A | N/A |
