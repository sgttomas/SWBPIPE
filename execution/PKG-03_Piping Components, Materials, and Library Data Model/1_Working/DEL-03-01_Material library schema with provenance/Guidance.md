# Guidance: DEL-03-01 Material library schema with provenance

## Purpose

This deliverable exists so OpenPipeStress can support piping-specific private material libraries without shipping protected material tables, proprietary commercial data, or unreviewed public data. The schema should make provenance, rights, units, and completeness visible enough for later validation, diagnostics, and review gates.

## Principles

- Treat material values and allowables as governed data, not as free public defaults.
- Separate schema slots from data content: defining an `allowable` field is allowed; populating public tables of protected allowables is not.
- Preserve source and rights metadata with every material value that could affect solving, rule checking, reporting, or downstream reliance.
- Use `TBD` for unresolved field names, fixture contents, external source requirements, and human review dispositions.
- Prefer explicit warnings and blocked states over silent fallbacks when required material data is absent or untrusted.

## Considerations

The schema will likely need conditional rules: private records may carry user-entered or lawfully imported values, while public repository fixtures require documented redistribution rights and review disposition. Exact validation mechanics are implementation-level decisions and remain `TBD`.

The schema should be designed so future importers and adapters cannot bypass provenance, redistribution, unit, or diagnostic requirements. This follows AB-00-07 but does not decide concrete import/export formats.

## Trade-offs

| Decision area | Tension | Current setup position |
|---|---|---|
| Public examples | Useful for tests and documentation, but risky if derived from protected standards or proprietary libraries. | Use invented non-engineering fixtures or licensed public data only after review; exact examples `TBD`. |
| Required field strictness | Strict requirements improve safety but may block partial private libraries. | Required-for-solving/checking values should produce explicit findings when absent; conditional schema rules are `TBD`. |
| Source citations | Detailed source pointers improve traceability but can expose protected content if mishandled. | Store source/provenance pointers and rights status; do not reproduce protected text/tables. |
| Allowable values | Needed for some checks, but code-specific tables are protected or user-governed. | Provide schema slots and diagnostics; do not bundle public allowable tables. |

## Examples

Public material editor fixtures are `TBD`. They must not include protected material allowable tables, copied standards examples, proprietary library data, or invented values presented as engineering data.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict found in accessible setup sources. | N/A | N/A | N/A | N/A | N/A |

