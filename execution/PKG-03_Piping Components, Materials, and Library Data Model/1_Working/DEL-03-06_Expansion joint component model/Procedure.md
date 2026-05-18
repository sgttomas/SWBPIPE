# Procedure: DEL-03-06 Expansion joint component model

## Purpose

Define the setup procedure a future implementation worker can use to turn this evidence kit into product work without crossing the protected-data, architecture, or package-boundary constraints.

## Prerequisites

- Sealed brief for DEL-03-06 with explicit write scope.
- Current `_CONTEXT.md`, `_REFERENCES.md`, `Datasheet.md`, `Specification.md`, and `Guidance.md`.
- Human-approved implementation scope for product files outside this setup folder.
- Applicable architecture-basis constraints AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.

## Steps

1. Confirm the requested work is still bounded to DEL-03-06 and SOW-010.
2. Identify the exact product file targets and tests authorized by the implementation brief.
3. Define expansion joint fields for stiffness, effective area, movement limits, and hardware data with unit metadata and provenance metadata.
4. Leave all unsupported or source-dependent values as `TBD`; do not add defaults.
5. Add validation paths for missing required values, invalid dimensions, unsupported provenance, and protected-content risk.
6. Preserve architecture/API boundaries so adapters, plugins, persistence, GUI services, and reports consume validated envelopes rather than bypassing domain contracts.
7. Add validation tests for schema/model behavior, unit handling, diagnostics, provenance, and guardrails.
8. Record any unresolved field taxonomy, solver semantics, or standard/manufacturer interpretation question for human ruling.

## Verification

- Confirm no manufacturer proprietary values or protected standards content were introduced.
- Confirm no certification/compliance claim was introduced.
- Confirm missing values remain explicit diagnostics or `TBD`.
- Confirm unit/provenance fields are represented in future implementation design.
- Confirm tests exercise missing-data and provenance guardrails.

## Records

- Updated product implementation artifacts only under a future authorized write scope.
- Validation test results.
- Human rulings for any field taxonomy, standards interpretation, or manufacturer-data handling decisions.
- Dependency/register updates when future implementation creates or consumes cross-deliverable interfaces.

