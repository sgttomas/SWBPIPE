# Specification: DEL-03-04 Branch connection component model fields

## Scope

This deliverable specifies the branch connection component model fields now
evidenced by `schemas/component.schema.yaml`,
`fixtures/component/invented_component_library_valid.json`, and
`tests/test_component_section_schema.py`. The implemented schema surface covers
branch connection geometry, reinforcement references/area slots, user
SIF/flexibility slots, source metadata, provenance, unit dimensions,
completeness rules, and diagnostics without bundling protected engineering
values.

This evidence-reconciliation pass excludes:

- product implementation code;
- repo-level schema edits;
- fixture or test edits;
- dependency, review-finding, DAG, coordination, lifecycle, or status edits;
- protected standards text, formulas, tables, figures, copied examples, or proprietary data;
- bundled branch, SIF, flexibility, dimensional, material, or allowable defaults;
- certification, sealing, approval, authentication, or compliance claims.

## Requirements

| Req ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-03-04-RQ-001 | The component schema shall support branch connection data fields for geometry, reinforcement, user SIF/flexibility, and local branch data fields. | SOW-008; DEL-03-04 row | Validation tests inspect the model/schema for supported field groups. |
| DEL-03-04-RQ-002 | Branch connection SIF, flexibility, reinforcement, and other code-specific values shall be user-supplied or lawfully imported private data, not bundled public defaults. | OPS-K-DATA-1; OPS-K-IP-1 | Protected-content and fixture review confirms no bundled protected tables/defaults/examples. |
| DEL-03-04-RQ-003 | Missing solve-required or rule-check-required branch values shall be represented as explicit diagnostics/findings, not silent defaults. | OPS-K-DATA-2; AB-00-06 | Validation tests cover missing required data and expected diagnostic classes. |
| DEL-03-04-RQ-004 | Branch component records shall carry provenance/source fields for component data, SIF values, flexibility factors, and imported values where applicable. | OPS-K-DATA-3; AB-00-04; AB-00-07 | Schema/model tests confirm provenance metadata fields and import boundary diagnostics. |
| DEL-03-04-RQ-005 | Units associated with branch geometry, reinforcement, SIF/flexibility inputs, and exports shall be unit-aware and dimensionally checked where dimensions apply. | OPS-K-UNIT-1; AB-00-04 | Unit validation tests cover accepted, rejected, and missing unit metadata. |
| DEL-03-04-RQ-006 | The implementation shall preserve public/private data boundaries and cannot bypass governance, validation, diagnostics, or provenance checks. | AB-00-02; AB-00-07; OPS-K-IP-3 | Architecture and service-boundary tests verify validation paths are used. |
| DEL-03-04-RQ-007 | Validation tests shall be included for model completeness, provenance, unit handling, and protected-content boundary behavior. | AB-00-08; anticipated artifacts | Test inventory and CI-local validation confirm coverage. |

## Current Evidence

- The component schema defines branch field kinds:
  `branch_run_size`, `branch_header_size`, `branch_connection_angle`,
  `branch_connection_type`, `branch_reinforcement_area`,
  `branch_reinforcement_reference`, `branch_geometry_source_reference`,
  `sif_user_value`, and `flexibility_factor_user_value`.
- The branch component-family contract in the invented fixture lists branch
  geometry, reinforcement, SIF/flexibility, source metadata, and
  mechanics-interface slots with `protected_value_policy: schema_slots_only`.
- The invented branch component record uses `public_schema_only`,
  `redistribution_status: TBD`, missing value statuses, provenance, and a
  `BRANCH_RULE_INPUT_MISSING` completeness finding.
- Component diagnostics now require `class` and `source` fields in addition to
  the earlier code, severity, affected reference, message, remediation, and
  provenance fields.
- The schema test asserts branch enum membership, branch fixture contract
  membership, branch fixture field slots, branch completeness diagnostics,
  diagnostic `class`/`source` requirements, fixture validation, and protected
  public-data denylist behavior.

## Standards

No protected engineering standard text is locally available or reproduced here.
Any standard-specific branch connection interpretation remains `TBD` and must
be supplied through lawful private data, a licensed source, or a human-approved
public abstraction that does not reproduce protected content.

## Verification

Current verification evidence includes:

- schema/model presence checks for each field group named in SOW-008;
- unit-aware validation for dimensional fields;
- provenance completeness checks for user/imported values;
- missing-data diagnostics for required analysis or rule-check inputs;
- protected-content scans confirming no bundled branch/SIF/flexibility tables, formulas, or examples;
- schema and fixture validation through `tests/test_component_section_schema.py`.

Remaining verification `TBD`s include dependency satisfaction disposition,
public fixture-value policy, accepted branch source catalog, persistence or
round-trip evidence beyond the current strict schema fixture, adapter/import
formats, and lifecycle acceptance.

## Documentation

Current implementation evidence:

- branch component schema surface: `schemas/component.schema.yaml`;
- invented branch component fixture: `fixtures/component/invented_component_library_valid.json`;
- schema/fixture validation: `tests/test_component_section_schema.py`.

This deliverable-local kit records evidence reconciliation only. It is not a
lifecycle promotion, release claim, professional/code-compliance claim, or human
acceptance decision.
