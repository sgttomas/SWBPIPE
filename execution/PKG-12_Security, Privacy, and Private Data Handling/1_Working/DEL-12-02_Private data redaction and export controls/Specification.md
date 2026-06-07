# Specification: DEL-12-02 Private data redaction and export controls

## Scope

This deliverable specifies the documentation-level contract for private-data redaction and export controls where reports, shared models, downstream-tool exports, public templates, or examples may expose protected or private values.

The original setup run was documentation production only. Current June 7 evidence now includes a metadata-only redaction helper, a schema-governed redaction/export-control contract, focused invented-fixture tests, and security documentation. This deliverable still does not process real project data, create report templates, integrate runtime report/export routes, move quarantine material, approve legal/security sufficiency, or make lifecycle, professional, certification, approval, sealing, authentication, or code-compliance claims.

## Requirements

| Requirement ID | Requirement | Source Basis | Verification |
|---|---|---|---|
| REXC-REQ-001 | Export and report workflows shall classify the export context before exposing project, rule-pack, material, component, owner, or code/design-basis data. | SOW-040; OPS-K-PRIV-1; `docs/SPEC.md` §§1,8 | Confirm the control contract includes explicit export context classes. |
| REXC-REQ-002 | Shared/public exports shall not silently include private project data, private rule-pack values, private material/component values, protected standards content, copied formulas, proprietary templates, or vendor data without documented rights. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` §§3,7 | Focused redaction tests cover private/unknown metadata handling; protected-content linter and runtime export-route tests remain `TBD`. |
| REXC-REQ-003 | Public report templates and public examples shall be protected-data-free and shall not embed protected formulas, standards tables, copied code text, proprietary report templates, or private project examples. | OPS-K-REPORT-2; `docs/SPEC.md` §8; `docs/IP_AND_DATA_BOUNDARY.md` §7 | Guardrail scans exist for changed surfaces; report-template linter/runtime integration remains `TBD`. |
| REXC-REQ-004 | Redaction configuration shall support at least warning, value redaction, field redaction, export blocking, and explicit local-private export allowance. | SOW-040; `docs/_Registers/Deliverables.csv` row DEL-12-02 | Schema vocabulary and focused tests now check the supported policy vocabulary. |
| REXC-REQ-005 | Unknown or insufficiently proven redistribution status shall result in warning, redaction, or block behavior rather than silent public inclusion. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-AGENT-1 | Focused redaction tests check unknown-source handling. |
| REXC-REQ-006 | Local private exports may retain private values only when the export context is local/private and user intent is explicit; the export record shall preserve a warning or audit note. | OPS-K-PRIV-1; `docs/DIRECTIVE.md` §§4.2,6 | Focused tests check explicit local-private intent; final UI/CLI approval flow remains `TBD`. |
| REXC-REQ-007 | Redaction shall not mutate the source project model, private libraries, or rule packs; it shall operate on an export/report representation. | AB-00-04; OPS-K-DATA-3 | Focused tests confirm the helper copies export/report representations; runtime source-model integration remains `TBD`. |
| REXC-REQ-008 | Redacted exports shall preserve non-sensitive reproducibility evidence such as model/report hashes, solver/report versions, input-manifest identifiers, warning summaries, rule-pack identity/version/checksum, and provenance summaries where safe. | OPS-K-REPORT-1; SOW-039; AB-00-04; `docs/SPEC.md` §8 | Schema/docs/helper support safe metadata preservation; full report/export manifest integration remains `TBD`. |
| REXC-REQ-009 | Rule-pack details in public/shared reports shall be limited to safe metadata such as ID, version, checksum, and source note unless the user has documented rights to include formula/detail content. | OPS-K-RULE-3; OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` §7 | Focused tests preserve safe checksum metadata while redacting private rule-pack values; runtime rule-pack/report integration remains `TBD`. |
| REXC-REQ-010 | Materials, components, sections, SIF/flexibility-like values, allowables, manufacturer/vendor values, and code/design-basis fields shall carry provenance and privacy/redistribution status into redaction decisions. | OPS-K-DATA-3; OPS-K-IP-2; `docs/TYPES.md` §§7-8 | Future schema/adapter tests check provenance and status inputs. |
| REXC-REQ-011 | Diagnostics and result/report envelopes shall surface redaction, protected-content, and private-data export findings using machine-readable diagnostics, including `IP_BOUNDARY_WARNING` where applicable. | AB-00-06; `docs/SPEC.md` §7 | Future diagnostics tests check code/class/severity/source/affected object/message/remediation/provenance fields. |
| REXC-REQ-012 | Adapters, plugins, CLI exports, GUI report preview/export, and downstream-tool handoffs shall not bypass redaction, provenance, unit, sandboxing, diagnostics, or report controls. | AB-00-02; AB-00-07; OPS-K-PRIV-1 | Future adapter/plugin tests exercise no-bypass routes. |
| REXC-REQ-013 | Export controls shall preserve the distinction among mechanics solved, user-rule checked, and human-approved states, and shall not claim certification, sealing, approval, authentication, or code compliance. | OPS-K-AUTH-1; OPS-K-AUTH-2; AB-00-03; `docs/TYPES.md` §4 | Report review checks professional-boundary notices and status vocabulary. |
| REXC-REQ-014 | Redaction/export tests shall cover public report export, local private export, shared model export, downstream-tool export, adapter/plugin routes, manifest preservation, unknown provenance, and source-model non-mutation. | AB-00-08; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` §5 | Focused helper/schema tests now exist; runtime report/export, GUI/CLI/API, adapter/plugin, and protected-content linter integration tests remain `TBD`. |
| REXC-REQ-015 | Any unresolved config schema, UI control, export format, public API transport, or physical project package/container choice shall remain `TBD` until resolved through an authorized implementation or architecture decision. | AB-00-04; AB-00-07; OPS-K-AGENT-1 | Review this deliverable for explicit TBD/open issue entries. |

## Standards

No external engineering code, standards clause, protected table, protected formula, material allowable, SIF/flexibility table, protected dimensional table, proprietary report template, vendor catalog, real private project, or real secret is used or reproduced by this deliverable.

The controlling project sources for this deliverable are the OpenPipeStress governance and decomposition artifacts listed in `Datasheet.md` and `_REFERENCES.md`.

## Verification

| Verification ID | Check | Expected Result |
|---|---|---|
| REXC-VER-001 | Confirm `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and preserve default sections. | Four-document kit is present. |
| REXC-VER-002 | Validate `Dependencies.csv` with `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`. | Schema valid with all 29 v3.1 columns. |
| REXC-VER-003 | Confirm `_SEMANTIC.md` has no `MatrixError` or `MATRIX_ERROR` and no algebra/operator leaks in final result tables. | Semantic setup gate passes. |
| REXC-VER-004 | Confirm `_SEMANTIC_LENSING.md` has complete coverage for matrices A, B, C, F, D, X, and E. | 96 required lens coverage rows are present. |
| REXC-VER-005 | Search deliverable artifacts for protected standards content, real private project values, real secrets, cloud-operation assumptions, and certification/compliance/approval/seal claims. | No disallowed content found. |
| REXC-VER-006 | Read `_STATUS.md` without editing it. | Current lifecycle state is `IN_PROGRESS`; this evidence alignment does not promote lifecycle state or imply acceptance. |
| REXC-VER-007 | Confirm June 7 evidence surfaces exist for `schemas/redaction_export_controls.schema.yaml`, `core/security/redaction/`, `tests/security/test_redaction_export_controls.py`, and `docs/security/redaction_export_controls.md`. | Evidence exists; lifecycle state remains separately governed. |
| REXC-VER-008 | Confirm June 7 run evidence records focused redaction/storage privacy hardening validation. | `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md` and package fan-in record passing focused tests. |

## Documentation

Required setup artifacts for this run:

- `Datasheet.md`;
- `Specification.md`;
- `Guidance.md`;
- `Procedure.md`;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- `_DEPENDENCIES.md`;
- `_run_records/*`;
- `_STATUS.md`.

Current implementation evidence:

- `schemas/redaction_export_controls.schema.yaml`;
- `core/security/redaction/`;
- `tests/security/test_redaction_export_controls.py`;
- `docs/security/redaction_export_controls.md`;
- `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md`;
- package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md`.

Artifacts and choices still deferred by this run:

- runtime report/export integration;
- GUI controls;
- CLI/API/export-adapter implementation;
- protected-content linter integration and legal review workflow;
- destructive quarantine movement workflow;
- cloud exception workflow;
- storage roots and persisted profile location;
- public transport and export-format choices;
- approval choices;
- real project data, real private values, protected standards content, secrets, credentials, or cloud behavior.
