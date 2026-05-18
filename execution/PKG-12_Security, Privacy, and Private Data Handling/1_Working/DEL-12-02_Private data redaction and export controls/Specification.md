# Specification: DEL-12-02 Private data redaction and export controls

## Scope

This deliverable specifies the documentation-level contract for private-data redaction and export controls where reports, shared models, downstream-tool exports, public templates, or examples may expose protected or private values.

This setup run is documentation production only. It does not implement product redaction code, edit schemas, create a JSON config file, create executable tests, process real project data, create report templates, or approve any legal/compliance sufficiency claim.

## Requirements

| Requirement ID | Requirement | Source Basis | Verification |
|---|---|---|---|
| REXC-REQ-001 | Export and report workflows shall classify the export context before exposing project, rule-pack, material, component, owner, or code/design-basis data. | SOW-040; OPS-K-PRIV-1; `docs/SPEC.md` §§1,8 | Confirm the control contract includes explicit export context classes. |
| REXC-REQ-002 | Shared/public exports shall not silently include private project data, private rule-pack values, private material/component values, protected standards content, copied formulas, proprietary templates, or vendor data without documented rights. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` §§3,7 | Future protected-content/export tests scan public/shared outputs. |
| REXC-REQ-003 | Public report templates and public examples shall be protected-data-free and shall not embed protected formulas, standards tables, copied code text, proprietary report templates, or private project examples. | OPS-K-REPORT-2; `docs/SPEC.md` §8; `docs/IP_AND_DATA_BOUNDARY.md` §7 | Future report-template linter rejects disallowed content. |
| REXC-REQ-004 | Redaction configuration shall support at least warning, value redaction, field redaction, export blocking, and explicit local-private export allowance. | SOW-040; `docs/_Registers/Deliverables.csv` row DEL-12-02 | Future config schema tests check the supported policy vocabulary. |
| REXC-REQ-005 | Unknown or insufficiently proven redistribution status shall result in warning, redaction, or block behavior rather than silent public inclusion. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-AGENT-1 | Future export tests check unknown-source handling. |
| REXC-REQ-006 | Local private exports may retain private values only when the export context is local/private and user intent is explicit; the export record shall preserve a warning or audit note. | OPS-K-PRIV-1; `docs/DIRECTIVE.md` §§4.2,6 | Future workflow tests check explicit action and warning record. |
| REXC-REQ-007 | Redaction shall not mutate the source project model, private libraries, or rule packs; it shall operate on an export/report representation. | AB-00-04; OPS-K-DATA-3 | Future round-trip tests confirm source artifacts remain unchanged. |
| REXC-REQ-008 | Redacted exports shall preserve non-sensitive reproducibility evidence such as model/report hashes, solver/report versions, input-manifest identifiers, warning summaries, rule-pack identity/version/checksum, and provenance summaries where safe. | OPS-K-REPORT-1; SOW-039; AB-00-04; `docs/SPEC.md` §8 | Future report/export tests check manifests remain useful after redaction. |
| REXC-REQ-009 | Rule-pack details in public/shared reports shall be limited to safe metadata such as ID, version, checksum, and source note unless the user has documented rights to include formula/detail content. | OPS-K-RULE-3; OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` §7 | Future export tests check formula/detail suppression. |
| REXC-REQ-010 | Materials, components, sections, SIF/flexibility-like values, allowables, manufacturer/vendor values, and code/design-basis fields shall carry provenance and privacy/redistribution status into redaction decisions. | OPS-K-DATA-3; OPS-K-IP-2; `docs/TYPES.md` §§7-8 | Future schema/adapter tests check provenance and status inputs. |
| REXC-REQ-011 | Diagnostics and result/report envelopes shall surface redaction, protected-content, and private-data export findings using machine-readable diagnostics, including `IP_BOUNDARY_WARNING` where applicable. | AB-00-06; `docs/SPEC.md` §7 | Future diagnostics tests check code/class/severity/source/affected object/message/remediation/provenance fields. |
| REXC-REQ-012 | Adapters, plugins, CLI exports, GUI report preview/export, and downstream-tool handoffs shall not bypass redaction, provenance, unit, sandboxing, diagnostics, or report controls. | AB-00-02; AB-00-07; OPS-K-PRIV-1 | Future adapter/plugin tests exercise no-bypass routes. |
| REXC-REQ-013 | Export controls shall preserve the distinction among mechanics solved, user-rule checked, and human-approved states, and shall not claim certification, sealing, approval, authentication, or code compliance. | OPS-K-AUTH-1; OPS-K-AUTH-2; AB-00-03; `docs/TYPES.md` §4 | Report review checks professional-boundary notices and status vocabulary. |
| REXC-REQ-014 | Redaction/export tests shall cover public report export, local private export, shared model export, downstream-tool export, adapter/plugin routes, manifest preservation, unknown provenance, and source-model non-mutation. | AB-00-08; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` §5 | Test files are not created in this setup run; future implementation must add them. |
| REXC-REQ-015 | Any unresolved config schema, UI control, export format, public API transport, or physical project package/container choice shall remain `TBD` until resolved through an authorized implementation or architecture decision. | AB-00-04; AB-00-07; OPS-K-AGENT-1 | Review this deliverable for explicit TBD/open issue entries. |

## Standards

No external engineering code, standards clause, protected table, protected formula, material allowable, SIF/flexibility table, protected dimensional table, proprietary report template, vendor catalog, real private project, or real secret is used or reproduced by this deliverable.

The controlling project sources for this setup run are the OpenPipeStress governance and decomposition artifacts listed in `Datasheet.md` and `_REFERENCES.md`.

## Verification

| Verification ID | Check | Expected Result |
|---|---|---|
| REXC-VER-001 | Confirm `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and preserve default sections. | Four-document kit is present. |
| REXC-VER-002 | Validate `Dependencies.csv` with `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`. | Schema valid with all 29 v3.1 columns. |
| REXC-VER-003 | Confirm `_SEMANTIC.md` has no `MatrixError` or `MATRIX_ERROR` and no algebra/operator leaks in final result tables. | Semantic setup gate passes. |
| REXC-VER-004 | Confirm `_SEMANTIC_LENSING.md` has complete coverage for matrices A, B, C, F, D, X, and E. | 96 required lens coverage rows are present. |
| REXC-VER-005 | Search deliverable artifacts for protected standards content, real private project values, real secrets, cloud-operation assumptions, and certification/compliance/approval/seal claims. | No disallowed content found. |
| REXC-VER-006 | Confirm `_STATUS.md` is `SEMANTIC_READY`, not `ISSUED`, only after setup checks pass. | Lifecycle state is consistent with setup evidence. |

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

Implementation artifacts deferred by this run:

- product redaction configuration schema or file;
- report/export source code;
- GUI controls;
- CLI/API/export-adapter implementation;
- executable export tests;
- real project data, real private values, protected standards content, secrets, credentials, or cloud behavior.

