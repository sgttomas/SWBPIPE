# Specification: DEL-15-03 Downstream modeling export workflow

## Scope

This deliverable covers a generic backend export workflow for downstream modeling handoff packages. It is scoped to SOW-074 and OBJ-017: producing traceable, schema-compliant handoff packages for downstream modeling and professional validation workflows without creating automatic professional approval states.

Included artifacts are:

- handoff exporter;
- export validation tests;
- invented target fixture.

Excluded work:

- target-specific commercial stress output parsers;
- comprehensive commercial-tool result ingestion;
- formal prover-status lifecycle, automatic professional acceptance record, certification, sealing, approval, authentication, or code-compliance status;
- protected standards text, tables, examples, formulas, proprietary commercial data, private rule packs, owner standards, or code-specific acceptance criteria;
- canonical package container, concrete mappings, target field coverage, target-specific implementation, dependency versions, and package-specific implementation choices until separately resolved.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-15-03-REQ-001 | The workflow shall generate schema-compliant handoff packages. | SOW-074 in `_CONTEXT.md` and `SOFTWARE_DECOMP.md` | `tests/test_handoff_export_workflow.py` validates exported handoff packages against `schemas/handoff_package.schema.json`. |
| DEL-15-03-REQ-002 | The package data surfaced by the workflow shall include or preserve model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags. | SOW-074 in `_CONTEXT.md` and `SOFTWARE_DECOMP.md` | Fixture-based validation that required fields are present or explicitly reported as TBD/unsupported according to upstream schema contracts. |
| DEL-15-03-REQ-003 | The workflow shall remain generic and shall not implement target-specific commercial-tool parsers in this deliverable. | `_CONTEXT.md#Context Envelope`; `SOFTWARE_DECOMP.md#SOW-074`; OI-015 | Tests or review evidence confirming invented target fixture use and no commercial parser fixture dependency. |
| DEL-15-03-REQ-004 | The workflow shall preserve unit metadata across the export boundary. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md#Unit system and dimensional analysis` | Schema validation and unit-manifest checks. |
| DEL-15-03-REQ-005 | The workflow shall preserve diagnostics, warnings, unresolved assumptions, provenance, and hashes rather than hiding missing data or boundary risks. | `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-REPORT-1; `docs/TYPES.md` Diagnostic, Assumption, Checksum, ResultExportEnvelope | Export validation tests and review of generated diagnostics/warnings. |
| DEL-15-03-REQ-006 | The workflow shall not claim certification, sealing, approval, authentication, professional acceptance, or engineering code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-AGENT-4; `SOFTWARE_DECOMP.md` OBJ-017 and DEC-015 | Protected/professional-boundary wording review and tests where applicable. |
| DEL-15-03-REQ-007 | Public fixtures and examples shall use invented or otherwise cleared data and shall not copy protected standards content, commercial software examples, or proprietary data. | `docs/IP_AND_DATA_BOUNDARY.md`; `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3 | Protected-content review, fixture provenance review, and validation gate. |
| DEL-15-03-REQ-008 | The workflow shall respect schema-first contracts and JSON Schema 2020-12 and shall preserve the producer-declared checksum byte-basis label. DEL-15-03 carries supplied hashes without recomputing or relabeling them and therefore makes no independent JCS claim. | `_CONTEXT.md#Architecture Basis Injection`; `SOFTWARE_DECOMP.md` DEC-009 through DEC-012; D-41 R5 T2A | Implementation review verifies checksum metadata is carried unchanged and no unproved JCS assertion is introduced. |
| DEL-15-03-REQ-009 | The workflow shall consume upstream handoff/mapping/security/model-transformation context only through explicit contracts or approved dependency evidence. | Local `Dependencies.csv`; `_DEPENDENCIES.md` | Dependency review; no sibling deliverable source copying without an approved input contract. |
| DEL-15-03-REQ-010 | Missing target behavior, missing mapping support, or unsupported features shall be explicit findings or unsupported-target flags, not silent defaults. | SOW-074; `docs/CONTRACT.md` OPS-K-DATA-2; OI-015 | Validation tests using the invented target fixture. |

## Standards

| Standard or governing source | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Contract basis for schema-backed handoff/package validation. | Applicable from `_CONTEXT.md#Architecture Basis Injection`; current upstream schemas are `schemas/handoff_package.schema.json` and `schemas/target_mapping.schema.json`. |
| Producer-declared JSON hash basis | Carry supplied checksum algorithm, payload scope, digest, and canonicalization label unchanged; no recomputation or conformance ruling by this workflow. | Applicable from `_CONTEXT.md#Architecture Basis Injection`; exact producer payload boundaries remain external to DEL-15-03. |
| OpenPipeStress invariant catalog | Governs data, unit, professional-boundary, IP, privacy, and agent-output constraints. | Applicable from `docs/CONTRACT.md`. |
| IP and Data Boundary Policy | Governs protected-content, private-data, fixture, and provenance handling. | Applicable from `docs/IP_AND_DATA_BOUNDARY.md`. |
| Commercial stress tool formats | Target-specific parsers and commercial examples are out of scope. | Deferred; concrete mappings, target field coverage, and target-specific implementation remain gated by OI-015, DEL-17-01, and DEL-17-02. |

## Verification

| Verification item | Evidence expected |
|---|---|
| Schema compliance | Validation tests for handoff package output against `schemas/handoff_package.schema.json` and target mapping input against `schemas/target_mapping.schema.json`. |
| Unit manifest preservation | Tests that exported values carry explicit unit metadata or are explicitly dimensionless/TBD according to schema rules. |
| Hash/provenance preservation | Tests or review evidence for model/package hash fields, provenance fields, and unresolved assumptions. |
| Unsupported-target behavior | Invented target fixture that exercises unsupported or approximate mapping behavior without using proprietary commercial examples. |
| Boundary wording | Review confirming no generated metadata or report-adjacent output claims professional approval, certification, sealing, authentication, or code compliance. |
| Protected-content screening | Protected-content/provenance gate for fixture and example data. |

## Documentation

Required records for this deliverable are:

- implementation notes identifying `core/handoff/exporter/workflow.py` and the schema-backed output contract;
- export validation test evidence;
- invented target fixture provenance and protected-content review status;
- any unresolved target mapping, package container, hash boundary, or dependency-version TBDs;
- dependency closure evidence for required upstream contracts before this workflow is treated as implementation-ready.
## D-41 R5 T2A checksum-boundary requirement (2026-07-12)

DEL-15-03 SHALL preserve supplied checksum metadata without relabeling it as JCS and SHALL NOT imply that carrying a checksum constitutes recomputation, validation, or RFC 8785 conformance.
