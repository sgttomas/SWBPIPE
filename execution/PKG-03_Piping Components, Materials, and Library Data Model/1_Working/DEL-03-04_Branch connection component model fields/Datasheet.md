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

The branch connection component model is now evidenced by the strict component
library schema, invented component fixture, and schema test coverage:

- `schemas/component.schema.yaml` defines branch component field kinds for
  branch run size, branch header size, connection angle, connection type,
  reinforcement area, reinforcement reference, branch geometry source
  reference, user SIF value, and user flexibility-factor value.
- `fixtures/component/invented_component_library_valid.json` carries a branch
  component-family contract with geometry field kinds, rule-modifier field
  kinds, source metadata field kinds, mechanics-interface metadata, and
  `protected_value_policy: schema_slots_only`.
- The same fixture carries an invented branch record whose public field slots
  are intentionally `missing` and provenance-bearing, with schema-shape-only or
  no-public-code-specific-value policies.
- `tests/test_component_section_schema.py` asserts branch enum coverage,
  branch family-contract coverage, branch fixture field coverage, branch
  completeness diagnostics, component diagnostic `class`/`source` fields, and
  protected-content denylist coverage.

No protected branch connection tables, SIF tables, flexibility tables, code
formulas, dimensional tables, copied engineering examples, proprietary catalog
values, or bundled public branch defaults are included. Remaining `TBD` items
are policy/source decisions rather than absence of the implemented schema
surface.

## Conditions

| Condition | Source | Status |
|---|---|---|
| Component model shall support branch connections with user-entered reinforcement, SIF, flexibility, and local data fields. | docs/_Registers/ScopeLedger.csv row SOW-008; execution/_Decomposition/SOFTWARE_DECOMP.md rows SOW-008 and DEL-03-04 | In scope |
| Public repository must not contain protected standards text, protected tables, copied formulas, or protected branch/SIF/flexibility data. | docs/CONTRACT.md OPS-K-IP-1, OPS-K-IP-3 | Constraint |
| Code-specific values are user-supplied or lawfully imported private data, not bundled public defaults. | docs/CONTRACT.md OPS-K-DATA-1 | Constraint |
| Missing solve-required or rule-check-required values are findings, not silent defaults. | docs/CONTRACT.md OPS-K-DATA-2 | Constraint |
| Materials, components, SIFs, flexibility factors, allowables, and rule-pack values carry provenance fields. | docs/CONTRACT.md OPS-K-DATA-3 | Constraint |
| Calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked. | docs/CONTRACT.md OPS-K-UNIT-1 | Constraint |

## Construction

The implemented evidence is schema-first and repository-level:

- `schemas/component.schema.yaml` exposes a JSON Schema 2020-12 component
  library contract with `component_library`, `component_family_contracts`,
  `component_records`, `field_definitions`, `completeness_rules`,
  `diagnostics`, and `open_decisions`.
- Branch dimensional fields use `ComponentQuantityDimension` values that remain
  constrained to the accepted unit dimension vocabulary exercised by the schema
  test.
- Branch diagnostics include `BRANCH_GEOMETRY_INCOMPLETE`,
  `BRANCH_REINFORCEMENT_DATA_MISSING`, and `BRANCH_RULE_INPUT_MISSING`; the
  component diagnostic shape requires `class`, `source`, affected reference,
  message, remediation, and provenance.
- The invented branch fixture records missing branch run/header/angle,
  reinforcement reference, user SIF, and user flexibility-factor inputs as
  blocking completeness evidence instead of silent defaults.

This reconciliation records that implementation evidence. It does not edit
repo-level schema, fixture, test, code, DAG, dependency, review-disposition, or
lifecycle files.

## Remaining TBDs

- Accepted public branch component source catalogs remain `TBD`.
- Public branch fixture-value policy remains `TBD`; current public fixture
  values are schema-shape-only or omitted.
- Standard-specific branch interpretation, local-check methods, concrete import
  formats, and GUI behavior remain `TBD`.
- Dependency satisfaction and lifecycle state remain governed by their existing
  local files and human/reconciliation process.
- Review findings remain conceptually
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`; the CSV was
  not edited by this reconciliation.

## References

- `_CONTEXT.md` - deliverable identity, accepted decomposition revision, architecture basis injection.
- `_REFERENCES.md` - local governing reference list.
- `docs/_Registers/Deliverables.csv` row DEL-03-04.
- `docs/_Registers/ScopeLedger.csv` row SOW-008.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-04.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, rows for PKG-03, DEL-03-04, SOW-008, OBJ-004, and AB-00-01/02/04/06/07/08.
- `docs/CONTRACT.md` invariants listed in this kit.
- `schemas/component.schema.yaml` current component library schema evidence.
- `fixtures/component/invented_component_library_valid.json` current invented
  component fixture evidence.
- `tests/test_component_section_schema.py` current schema/fixture validation
  evidence.
- `MEMORY.md`, `_REVIEW.md`, and `Review_Findings.csv` local reconciliation and
  review evidence.
