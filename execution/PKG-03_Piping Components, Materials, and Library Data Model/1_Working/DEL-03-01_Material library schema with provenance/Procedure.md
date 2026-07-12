# Procedure: DEL-03-01 Material library schema with provenance

## PDU-024 Integration Check

1. Validate material records against the DEL-03-01 schema and provenance rules.
2. When material records are carried by a project model, consume DEL-02-05 version-check and migration evidence rather than a local version literal.
3. Preserve explicit stale/unsupported/newer/failed project-version diagnostics and do not rewrite the material fixture as part of model-document compatibility handling.

## Purpose

Describe the maintenance and review procedure for the DEL-03-01 material-library schema, invented fixture, and validation evidence while preserving protected-content, provenance, unit, privacy, and agent-output boundaries.

## Prerequisites

- Sealed DEL-03-01 brief and write scope.
- `_CONTEXT.md`, `_REFERENCES.md`, `docs/CONTRACT.md`, register rows, SOFTWARE_DECOMP revision 0.7, and approved DAG-006 context.
- Current evidence files: `schemas/material.schema.yaml`, `fixtures/material/invented_material_library_valid.json`, and `tests/test_material_schema.py`.
- Human-approved rules for any public fixture source, license, redistribution status, and review disposition.
- No protected material tables or proprietary library data in the working folder.

## Steps

1. Confirm `_STATUS.md` is not `ISSUED` and that the intended tranche is evidence reconciliation, not lifecycle promotion.
2. Read the DEL-03-01 context, register rows, SOW-017, OBJ-004, and applicable CONTRACT invariants.
3. Inspect `schemas/material.schema.yaml` for identity, property, allowable, provenance, redistribution, completeness, diagnostic, and open-decision coverage.
4. Inspect `fixtures/material/invented_material_library_valid.json` to confirm it remains invented/schema-shape evidence with omitted engineering values and explicit diagnostics.
5. Run `python3 tests/test_material_schema.py` and record the result in a deliverable-local run record.
6. Run a stale-language check against the active four-document kit for phrases that incorrectly describe the implemented schema as only setup/future evidence.
7. Preserve unresolved public source, fixture value, interpolation, allowable storage, dependency satisfaction, and human review disposition items as `TBD`.
8. Route public data acceptance, quarantine disposition, package audit finding disposition, dependency satisfaction, and lifecycle transitions through separate human/reconciliation gates.

## Verification

| Check | Expected result |
|---|---|
| Material schema test | `python3 tests/test_material_schema.py` passes. |
| Stale active-doc language | Active four-document kit no longer says implementation evidence is only setup/future work. |
| Protected-content scan | No protected standards text, material allowable tables, copied examples, or proprietary material data is introduced. |
| Provenance validation | Every governed material value has source/provenance and rights status, or produces an explicit finding. |
| Unit validation | Material property dimensions remain aligned with the accepted PKG-02 dimension vocabulary. |
| Missing-value behavior | Missing solve-required or rule-check-required values produce explicit diagnostics. |
| Privacy check | Private libraries are not transmitted or committed publicly by default. |
| Review disposition check | `Review_Findings.csv` remains pending human disposition unless a separate human gate authorizes edits. |

## Records

- Schema review notes: deliverable-local run records.
- Fixture provenance review: `TBD` until human/source review accepts a public fixture policy.
- Protected-content review disposition: `TBD`.
- Human rulings on public fixture sources and package audit findings: `TBD`.
- Dependency register and run records in this deliverable folder.
