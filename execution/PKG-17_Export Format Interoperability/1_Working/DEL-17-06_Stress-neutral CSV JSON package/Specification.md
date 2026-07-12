# Specification: DEL-17-06 Stress-neutral CSV/JSON package

## Scope

DEL-17-06 shall define the documentation-level requirements for a project-controlled stress-neutral CSV/JSON package used for review, regression comparison, and downstream tooling.

This Phase A deliverable is a four-document kit only. It shall not implement code, create or edit JSON schemas, create CSV fixtures, write exporters, define release compatibility, or claim professional/code-compliance acceptance.

The stress-neutral package shall remain result/package evidence. It is not a vendor solver format, not a commercial solver input deck, not a CAEPIPE compatibility claim, and not a substitute for human professional review.

## Requirements

| Req ID | Requirement | Source basis |
|---|---|---|
| DEL-17-06-REQ-001 | The package shall support review, debugging, community adapter development, and comparison across target exporters without presenting itself as a vendor format. | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` "Stress Neutral CSV/JSON Package" |
| DEL-17-06-REQ-002 | The package shall provide CSV output for inspection/spreadsheet review and JSON output for structured import/export. | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` "Stress Neutral CSV/JSON Package" |
| DEL-17-06-REQ-003 | The package shall preserve canonical OpenPipeStress identity through direct row/object fields or sidecar ID-map records. Exact field names are `TBD`. | `DEL-17-02/Specification.md` stable ID map requirements |
| DEL-17-06-REQ-004 | The package shall include or reference a manifest that records source model/run basis, package member inventory, hashes where applicable, diagnostics, and boundary notes. Exact manifest layout is `TBD`. | `DEL-17-02/Specification.md` manifest requirements; `docs/SPEC.md` result export section |
| DEL-17-06-REQ-005 | The package shall include a loss report even when export succeeds, with exported, omitted, approximated, delegated, unsupported, and `TBD` behavior visible. | `DEL-17-02/Specification.md` loss report requirements |
| DEL-17-06-REQ-006 | Result values shall carry explicit unit and dimensional metadata or produce blocking diagnostics. Exact CSV/JSON representation is `TBD`. | `docs/SPEC.md` result export section; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| DEL-17-06-REQ-007 | The package shall carry diagnostics, warnings, unresolved assumptions, provenance, reproducibility references, and professional-boundary notices when those are present in the source result/export envelope. | `docs/SPEC.md` result export section; `docs/TYPES.md` `ResultExportEnvelope` |
| DEL-17-06-REQ-008 | The package shall not copy protected standards text, protected tables, proprietary formulas, code-specific allowables, SIF/flexibility values, private rule-pack payloads, private project data, or proprietary commercial examples into public artifacts. | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/TYPES.md` |
| DEL-17-06-REQ-009 | The package shall not declare code compliance, certification, sealing, professional approval, formal validation, engineering acceptance, or release readiness. | `docs/CONTRACT.md` OPS-K-AUTH-1; `DEL-17-01/Specification.md`; `DEL-17-02/Specification.md` |
| DEL-17-06-REQ-010 | Target-specific or version-sensitive behavior not resolved by source evidence shall remain `TBD` and shall not be represented as supported behavior. | `DEL-17-01/Specification.md`; `DEL-17-02/Specification.md` |
| DEL-17-06-REQ-011 | The stress-neutral profile shall carry source-basis references for DEL-08-04, DEL-14-02, DEL-14-05, and DEL-17-02; missing references shall block package acceptance. | DAG-006 DEL-17-06 dependency edges; DEL-17-02 export contract; DEL-08-04 result export format; DEL-14-02 analysis run records; DEL-14-05 comparison export contracts. |

## Standards

| Standard or contract | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Baseline for future public JSON schemas/interchange contracts. | Applicable architecture basis; no schema file is created in this Phase A task. |
| Declared deterministic JSON hash basis | JSON package members use sorted-key compact ASCII-escaped Python JSON labeled `deterministic_sorted_compact_json_payload_hash`; this is explicitly not JCS. CSV normalization is separately labeled. | D-41 R5 T2A; exact non-current payload partitioning remains `TBD`. |
| OpenPipeStress result export envelope | Governs result identity, model/run references, unit-aware values, diagnostics, provenance, reproducibility refs, statuses, rule-pack refs, and downstream-use declarations. | Source-grounded contract basis; additional CSV/JSON package layout remains `TBD`. |
| DEL-17-02 common export contract | Governs export package, profile, stable ID map, manifest, and loss-report behavior. | Required upstream contract for DEL-17-06. |
| IP and data-boundary policy | Governs public/private/protected data use and contribution review. | Required for fixtures, examples, reports, and exported artifacts. |

## Verification

| Verification ID | Check | Acceptance evidence |
|---|---|---|
| DEL-17-06-VER-001 | Confirm all four documents exist and preserve required default sections. | `tools/validation/check_four_documents.sh` result. |
| DEL-17-06-VER-002 | Confirm the deliverable-local minimum fileset remains present. | `tools/validation/check_min_viable_fileset.sh` result. |
| DEL-17-06-VER-003 | Review the four documents for prohibited claims: code compliance, professional approval, formal validation, release compatibility, or engineering acceptance. | Manual text review or grep-based evidence in run record. |
| DEL-17-06-VER-004 | Review the four documents for protected/proprietary copied examples or protected standards data. | Manual text review or grep-based evidence in run record. |
| DEL-17-06-VER-005 | Confirm unresolved target behavior, exact table columns, JSON shape, manifest layout, tolerance thresholds, and comparison semantics remain `TBD`. | Four-document review. |

Future implementation verification, once schemas, writer outputs, fixtures, manifests, and comparison records exist:

| Verification ID | Check | Acceptance evidence |
|---|---|---|
| DEL-17-06-VER-006 | Map DEL-17-06-REQ-001 through DEL-17-06-REQ-010 to concrete schema, exporter, fixture, manifest, loss-report, and validation evidence. | Traceability table in a future implementation run record; unresolved entries remain `TBD` rather than accepted. |
| DEL-17-06-VER-007 | Confirm CSV and JSON representations are synchronized for canonical identity, units/dimensions, source model/run references, manifest basis, loss-report content, diagnostics, and boundary notices. | Future paired CSV/JSON fixture diff, manifest review, and validation-report result. |
| DEL-17-06-VER-008 | Confirm manifest or package-member hashes identify payload scope and use the declared sorted-compact JSON basis for JSON payloads; confirm the label makes no JCS claim and non-JSON partitioning remains explicit. | Exact-byte/fixed-hash, ordering, mutation, schema/fixture, and JSON/CSV label-split evidence. |
| DEL-17-06-VER-009 | Review generated schemas, CSV outputs, JSON outputs, manifests, loss reports, validation reports, and fixtures for prohibited claims and protected/private content. | Future protected-content and professional-boundary review evidence; suspected protected content quarantined per policy. |
| DEL-17-06-VER-010 | Confirm CSV/JSON examples or fixtures are invented or rights-cleared and demonstrate only shape, identity, units, diagnostics, loss reporting, and boundary notices. | Future contribution/provenance review evidence; examples without redistribution evidence remain absent or `TBD`. |
| DEL-17-06-VER-011 | Confirm any target-specific or version-sensitive support flag is backed by cited source evidence, or is marked `TBD` and blocked from support claims. | Future source-evidence review against DEL-17-01/DEL-17-02 and the applicable source-basis register. |
| DEL-17-06-VER-012 | Confirm comparison tolerance profiles, pass/fail language, and export comparison semantics are consumed from DEL-14-02/DEL-14-05 before comparison claims are made. | Future upstream dependency review; comparison semantics remain diagnostic/audit-only until sourced. |

## Documentation

Required Phase A documentation artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_STATUS.md` safe update from `OPEN` to `INITIALIZED` if permitted by the four-documents skill
- `_run_records/TASK_RUN_2026-05-18_1155.md`
- `_run_records/TASK_RUN_2026-05-18_1218.md` for Pass 3 disposition evidence

Future implementation artifacts listed in `_CONTEXT.md` remain not produced in this Phase A task:

- stress-neutral CSV schema
- stress-neutral JSON schema
- export writer
- comparison fixtures

## Conflict Table

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| none | No source conflict identified in Phase A. Remaining details are unresolved `TBD` items, not source contradictions. | n/a | n/a | n/a | n/a | n/a |
## D-41 R5 T2A canonicalization requirement (2026-07-12)

DEL-17-06 JSON hashes SHALL use `deterministic_sorted_compact_json_payload_hash` for the existing sorted-key compact ASCII-escaped Python JSON serializer. The label SHALL NOT be interpreted as RFC 8785/JCS conformance. Normalized CSV hashes SHALL remain separately labeled `normalized_ascii_lf_text`.
