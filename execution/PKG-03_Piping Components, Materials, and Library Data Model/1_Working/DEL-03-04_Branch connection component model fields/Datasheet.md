# Datasheet: DEL-03-04 Branch connection component model fields

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-04 |
| Package ID | PKG-03 |
| Package name | Piping Components, Materials, and Library Data Model |
| Type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-008 |
| Objective | OBJ-004 |
| Anticipated artifacts | branch component model; validation tests |

## Attributes

The branch connection component model is a data-model slice for branch connection records. It is scoped to fields for:

- branch connection geometry;
- user-entered reinforcement data;
- user-entered SIF data;
- user-entered flexibility data;
- local branch-connection data fields;
- source, provenance, unit, and redistribution metadata.

No protected branch connection tables, SIF tables, flexibility tables, code formulas, dimensional tables, or copied engineering examples are included in this setup evidence. Exact field names, enum values, serialization shape, and validation constraints are `TBD` for the later implementation pass.

## Conditions

| Condition | Source | Status |
|---|---|---|
| Component model shall support branch connections with user-entered reinforcement, SIF, flexibility, and local data fields. | docs/_Registers/ScopeLedger.csv row SOW-008; docs/_Decomposition/SOFTWARE_DECOMP.md rows SOW-008 and DEL-03-04 | In scope |
| Public repository must not contain protected standards text, protected tables, copied formulas, or protected branch/SIF/flexibility data. | docs/CONTRACT.md OPS-K-IP-1, OPS-K-IP-3 | Constraint |
| Code-specific values are user-supplied or lawfully imported private data, not bundled public defaults. | docs/CONTRACT.md OPS-K-DATA-1 | Constraint |
| Missing solve-required or rule-check-required values are findings, not silent defaults. | docs/CONTRACT.md OPS-K-DATA-2 | Constraint |
| Materials, components, SIFs, flexibility factors, allowables, and rule-pack values carry provenance fields. | docs/CONTRACT.md OPS-K-DATA-3 | Constraint |
| Calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked. | docs/CONTRACT.md OPS-K-UNIT-1 | Constraint |

## Construction

The future branch component model should be constructed as a schema-first domain record compatible with the architecture basis:

- Rust core/application-services placement where domain behavior belongs;
- JSON Schema 2020-12 contract exposure where serialized payloads are required;
- deterministic, unit-aware, provenance-preserving persistence;
- validation diagnostics that can identify blocking missing data, provenance warnings, and protected-content boundary warnings;
- tests covering valid/invalid field presence, unit checking, provenance completeness, and no bundled protected data.

This setup pass does not create implementation code or repo-level schema files.

## References

- `_CONTEXT.md` - deliverable identity, accepted decomposition revision, architecture basis injection.
- `_REFERENCES.md` - local governing reference list.
- `docs/_Registers/Deliverables.csv` row DEL-03-04.
- `docs/_Registers/ScopeLedger.csv` row SOW-008.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-04.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4, rows for PKG-03, DEL-03-04, SOW-008, OBJ-004, and AB-00-01/02/04/06/07/08.
- `docs/CONTRACT.md` invariants listed in this kit.
