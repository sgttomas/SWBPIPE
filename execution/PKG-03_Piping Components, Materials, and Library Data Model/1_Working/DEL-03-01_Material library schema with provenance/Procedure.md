# Procedure: DEL-03-01 Material library schema with provenance

## Purpose

Describe the setup procedure for converting the DEL-03-01 evidence into a future material-library schema and fixtures while preserving protected-content, provenance, unit, privacy, and agent-output boundaries.

## Prerequisites

- Sealed DEL-03-01 brief and write scope.
- `_CONTEXT.md`, `_REFERENCES.md`, `docs/CONTRACT.md`, register rows, and SOFTWARE_DECOMP revision 0.4.
- Human-approved rules for any public fixture source, license, redistribution status, and review disposition.
- No protected material tables or proprietary library data in the working folder.

## Steps

1. Confirm `_STATUS.md` is not `ISSUED` and that local setup artifacts may be refreshed.
2. Read the DEL-03-01 context, register rows, SOW-017, OBJ-004, and applicable CONTRACT invariants.
3. Define schema sections for identity, temperature-dependent properties, allowable slots, units, provenance, redistribution status, completeness flags, and diagnostics.
4. Mark any unresolved field naming, external standard source detail, fixture source, or review policy as `TBD`.
5. Create only invented or rights-cleared fixtures in a later implementation pass; do not add real material allowable tables in this setup pass.
6. Add negative fixture concepts for missing units, missing provenance, missing redistribution status, suspected protected content, and missing required values.
7. Run schema, protected-content, unit, and provenance checks when deterministic tools exist.
8. Record human rulings for public data acceptance, quarantine disposition, and fixture source approval before treating data as accepted.

## Verification

| Check | Expected result |
|---|---|
| Protected-content scan | No protected standards text, material allowable tables, copied examples, or proprietary material data in public artifacts. |
| Provenance validation | Every governed material value has source/provenance and rights status, or produces an explicit finding. |
| Unit validation | Temperature-dependent values and allowables carry explicit units and dimensional categories. |
| Missing-value behavior | Missing solve-required or rule-check-required values produce explicit diagnostics. |
| Privacy check | Private libraries are not transmitted or committed publicly by default. |

## Records

- Schema review notes: `TBD`
- Fixture provenance review: `TBD`
- Protected-content review disposition: `TBD`
- Human rulings on public fixture sources: `TBD`
- Dependency register and run records in this deliverable folder.

