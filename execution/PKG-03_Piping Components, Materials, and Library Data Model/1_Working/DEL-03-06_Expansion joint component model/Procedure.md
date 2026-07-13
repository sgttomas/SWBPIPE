# Procedure: DEL-03-06 Expansion joint component model

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-03-06-DECL-004`.

## Purpose

Define the procedure for maintaining and extending the current expansion-joint data-model evidence without crossing protected-data, architecture, lifecycle, or package-boundary constraints.

## Prerequisites

- Sealed brief for DEL-03-06 with explicit write scope.
- Current `_CONTEXT.md`, `_REFERENCES.md`, `Datasheet.md`, `Specification.md`, and `Guidance.md`.
- Current implementation evidence in `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, and `tests/test_component_section_schema.py`.
- Human-approved implementation scope before editing product files outside this deliverable folder.
- Applicable architecture-basis constraints AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.

## Steps

1. Confirm the requested work is still bounded to DEL-03-06 and SOW-010.
2. Identify the exact file targets and tests authorized by the active brief.
3. Reconcile documentation against current implementation evidence before adding new claims.
4. Confirm expansion joint fields for `linear_stiffness`, `rotational_stiffness`, `effective_area`, `movement_limit`, `hardware_flag`/`hardware_reference`, and `manufacturer_reference` remain unit/provenance bounded.
5. Leave all unsupported or source-dependent values as `TBD`; do not add defaults.
6. Preserve validation paths for missing required values, invalid dimensions, unsupported provenance, and protected-content risk.
7. Preserve architecture/API boundaries so adapters, plugins, persistence, GUI services, and reports consume validated envelopes rather than bypassing domain contracts.
8. Run or update validation tests for schema/model behavior, unit handling, diagnostics, provenance, and guardrails when authorized.
9. Record any unresolved field taxonomy, solver semantics, source/value policy, lifecycle, dependency-satisfaction, or standard/manufacturer interpretation question for human ruling.

## Verification

- Confirm no manufacturer proprietary values or protected standards content were introduced.
- Confirm no certification/compliance claim was introduced.
- Confirm missing values remain explicit diagnostics or `TBD`.
- Confirm unit/provenance fields are represented in schema, fixture, and test evidence.
- Confirm tests exercise missing-data and provenance guardrails.

## Records

- Updated product implementation artifacts only under explicit authorized write scope.
- Validation test results.
- Human rulings for any field taxonomy, standards interpretation, or manufacturer-data handling decisions.
- Dependency/register updates only when separately authorized and when implementation creates or consumes cross-deliverable interfaces.
