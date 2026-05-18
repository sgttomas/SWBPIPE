# Specification: DEL-03-04 Branch connection component model fields

## Scope

This deliverable specifies setup evidence for a later implementation of branch connection component model fields. The production implementation is expected to cover geometry, reinforcement, user SIF/flexibility, local data fields, source metadata, and validation tests for the branch component model.

This setup pass excludes:

- product implementation code;
- repo-level schema edits;
- protected standards text, formulas, tables, figures, copied examples, or proprietary data;
- bundled branch, SIF, flexibility, dimensional, material, or allowable defaults;
- certification, sealing, approval, authentication, or compliance claims.

## Requirements

| Req ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-03-04-RQ-001 | The later model shall support branch connection data fields for geometry, reinforcement, user SIF/flexibility, and local branch data fields. | SOW-008; DEL-03-04 row | Validation tests inspect the model/schema for supported field groups. |
| DEL-03-04-RQ-002 | Branch connection SIF, flexibility, reinforcement, and other code-specific values shall be user-supplied or lawfully imported private data, not bundled public defaults. | OPS-K-DATA-1; OPS-K-IP-1 | Protected-content and fixture review confirms no bundled protected tables/defaults/examples. |
| DEL-03-04-RQ-003 | Missing solve-required or rule-check-required branch values shall be represented as explicit diagnostics/findings, not silent defaults. | OPS-K-DATA-2; AB-00-06 | Validation tests cover missing required data and expected diagnostic classes. |
| DEL-03-04-RQ-004 | Branch component records shall carry provenance/source fields for component data, SIF values, flexibility factors, and imported values where applicable. | OPS-K-DATA-3; AB-00-04; AB-00-07 | Schema/model tests confirm provenance metadata fields and import boundary diagnostics. |
| DEL-03-04-RQ-005 | Units associated with branch geometry, reinforcement, SIF/flexibility inputs, and exports shall be unit-aware and dimensionally checked where dimensions apply. | OPS-K-UNIT-1; AB-00-04 | Unit validation tests cover accepted, rejected, and missing unit metadata. |
| DEL-03-04-RQ-006 | The implementation shall preserve public/private data boundaries and cannot bypass governance, validation, diagnostics, or provenance checks. | AB-00-02; AB-00-07; OPS-K-IP-3 | Architecture and service-boundary tests verify validation paths are used. |
| DEL-03-04-RQ-007 | Validation tests shall be included for model completeness, provenance, unit handling, and protected-content boundary behavior. | AB-00-08; anticipated artifacts | Test inventory and CI-local validation confirm coverage. |

## Standards

No protected engineering standard text is locally available for this setup pass. Any standard-specific branch connection interpretation is `TBD` and must be supplied through lawful private data, a licensed source, or a human-approved public abstraction that does not reproduce protected content.

## Verification

Verification for the later implementation should include:

- schema/model presence checks for each field group named in SOW-008;
- unit-aware validation for dimensional fields;
- provenance completeness checks for user/imported values;
- missing-data diagnostics for required analysis or rule-check inputs;
- protected-content scans confirming no bundled branch/SIF/flexibility tables, formulas, or examples;
- round-trip serialization checks if the model participates in persistence.

Exact test names, modules, and fixtures are `TBD`.

## Documentation

Expected implementation artifacts remain:

- branch component model;
- validation tests.

This deliverable-local kit records setup evidence only and is not an accepted product implementation.
