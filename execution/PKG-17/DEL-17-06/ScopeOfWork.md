---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-06
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-046, SOW-074]
package_objective_refs: [OBJ-007, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-06

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open obligations remain in this Scope of Work and owning decisions; lifecycle remains in `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Current semantic-evidence status (owner direction)

The 2026-05-18 `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` are historical records, not current evidence under accepted D-67/CLM-042. Their dated text remains intact; current selected-version schema/hash meaning follows this Scope of Work, the accepted D-67/CLM-042 records and candidate-bound export evidence. Do not rely on their superseded unresolved-partition assertions as a current blocker.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-06` in service of project scope [SOW-046, SOW-074] and package objectives [OBJ-007, OBJ-017, OBJ-018].

- **OUT-001** — A project-controlled stress-neutral CSV/JSON package contract for review, regression comparison, debugging, and governed downstream tooling is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-06 Stress-neutral CSV/JSON package

> #### Datasheet: DEL-17-06 Stress-neutral CSV/JSON package
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-06 |
> | Package | PKG-17 Export Format Interoperability |
> | Name | Stress-neutral CSV/JSON package |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope Items | SOW-046, SOW-074 |
> | Objectives | OBJ-007, OBJ-017, OBJ-018 |
> | Historical preparation pass | Phase A four-document P1/P2 only |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current value | Source |
> |---|---|---|
> | Package purpose | Project-controlled stress-oriented neutral package for review, debugging, community adapter development, and comparison across target exporters. It is not a vendor format and not a replacement for a commercial solver input deck. | Preserved deliverable obligation; accepted SCA-004 export scope and SOW-083 in `_CONTEXT.md`. The unavailable historical export plan is not current field-level evidence. |
> | Formats | CSV for inspection/spreadsheet review and JSON for structured import/export. | Preserved deliverable obligation; accepted SCA-004 export scope and SOW-083 in `_CONTEXT.md`. The unavailable historical export plan is not current field-level evidence. |
> | Required relationship to result exports | Result exports are for review, regression comparison, report consumption, headless automation, and governed downstream tooling; they preserve units, diagnostics, provenance, hashes, and professional-boundary limits. | `docs/SPEC.md` result export section; `docs/TYPES.md` `ResultExportEnvelope` |
> | Required relationship to DEL-17-02 | Use loss reports and stable IDs for stress-neutral CSV/JSON result packages. | `DEL-17-02/Specification.md` "Downstream Requirements" |
> | Required relationship to DEL-17-01 | Do not use CAEPIPE CSV/text behavior as professional acceptance; target-specific CSV coverage remains `TBD` unless source-confirmed. | `DEL-17-01/Specification.md` boundary and downstream rules |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Source model reference | Required source identity/version/reference binding; field evidence is in `schemas/stress_neutral_export.v0.3.schema.json` and its producers. Actual source fidelity remains required. |
> | Source model hash basis | Required explicit source hash/payload-scope evidence. Preserve received source checksums separately from computed members/package projections under 0.3.0. |
> | Analysis-run reference | Required source analysis-run and load/solver basis where applicable; inspect the 0.3 schema and producers alongside DEL-14-02's analysis record 0.3. Actual producer completeness is not inferred. |
> | Package member path references | Required member inventory bound by manifest; concrete source evidence is `core/handoff/stress_neutral/package.py` with the 0.3 schema. The Python builder in that module and the desktop builder in `StressNeutralExportPanel.tsx` construct current 0.3 directly; B7a removes the 0.1 and 0.2 construction (DEL-04-08 Design, `b7-plan.md`). Since B7b the Python builder admits pressure-2, pressure-source-1 (with its receipt) and preview-physics-2, and reads a pressure-2 package through its source, since pressure-2 has no transport reader; the 0.3 schema keeps only current branches and gained a pressure-source-1 branch; the retired v2 identities are refused. Since epoch 8 the desktop panel admits pressure-2 and pressure-source-1 at the Rust reader's numerically eligible standing (an eligible pressure-2 load/reference-state result included; a pressure-source-1 one is never eligible), copies `contract_evidence` (and pressure-source-1's receipt) as the Python builder does, and reads a pressure-2 package through its source (DEL-04-07 Design `desktop-exact-outputs.md` §4.3); it admits the retained successors only at numerically eligible standing with the live native capture, which no product caller delivers before B8. |
> | Unit and dimensional metadata | Required explicit units/dimensions and diagnostics. Versioned schemas and producer column definitions carry current field evidence; missing or incompatible unit behavior remains a conformance check. |
> | Stable ID preservation | Required direct or sidecar canonical identity; versioned package schemas carry `stable_id_map` evidence. Demonstrate round-trip correlation and omissions against actual output. |
> | Diagnostics and boundary notes | Required so review consumers can see limitations, unresolved assumptions, warnings, and non-authoritative status. |
> | Protected/private content handling | Public fixtures and examples must not include protected standards data, proprietary examples, private project data, code allowables, SIF/flexibility tables, or owner criteria. |
> | Professional boundary | The package is evidence for review/regression/downstream tooling only. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). |
>

### CLM-006 — Construction

> ##### Construction
>
> | Candidate package member | Required role | Current detail level |
> |---|---|---|
> | Node table | Reviewable node identity and coordinate/result-reference surface. | Table fields `TBD`. |
> | Element table | Reviewable element identity and connectivity/result-reference surface. | Table fields `TBD`. |
> | Component table | Reviewable component identity and relationship to elements/nodes. | Table fields `TBD`. |
> | Restraint/support table | Reviewable support/restraint identity and mapping to model/result records. | Table fields `TBD`; nonlinear/support-state behavior remains source-dependent. |
> | Equipment/nozzle table | Reviewable terminal/equipment interface identity and mapping surface. | Table fields `TBD`. |
> | Material table | Reference identity/provenance surface only; no bundled proprietary or protected values. | Table fields `TBD`. |
> | Section table | Reference identity/provenance surface only; no protected dimensional tables or catalog defaults. | Table fields `TBD`. |
> | Load/design case table | Reviewable load/design case identity and basis. | Table fields `TBD`. |
> | Units metadata | Declares unit system, dimensions, and conversion basis needed by CSV and JSON consumers. | Exact layout `TBD`. |
> | ID map | Maps canonical SWBPIPE IDs to package rows, JSON objects, or sidecars. | Exact layout `TBD`; consume DEL-17-02 rules. |
> | Validation report | Records parse/shape checks, loss entries, warnings, and boundary notes. | Exact layout `TBD`; machine-readable and human-readable forms expected where applicable. |
>

### CLM-007 — Future Detail Slots

> ##### Future Detail Slots
>
> The original Phase A slots are historical setup provenance. Current construction is `schemas/stress_neutral_export.v0.3.schema.json`, with producer evidence in `core/handoff/stress_neutral/package.py`. The durable content obligations below remain; schema/producer presence does not establish runtime or rights clearance.
>
> | Slot | Future content | Current disposition | Source basis |
> |---|---|---|---|
> | CSV fields | Exact table names, column names, ordering, quoting/null rules, and per-table required/optional status. | Use the selected version producer column/serialization definitions; verify emitted conformance. | Stress-neutral package contents from `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; package-member inventory rules from `DEL-17-02/ScopeOfWork.md`. |
> | JSON properties | Exact object layout, property names, required/optional status, schema identifier, and validation profile. | Use the selected versioned JSON Schema; verify consumer/producer conformance. | Architecture basis in `_CONTEXT.md`; `DEL-17-02/ScopeOfWork.md` architecture-basis requirements. |
> | Manifest layout | Package ID, source model/run basis, profile ID, member inventory, hashes where applicable, diagnostics, and boundary notes. | Layout evidence is the selected version schema/producer; required binding retained. | `DEL-17-02/ScopeOfWork.md` manifest requirements; `docs/SPEC.md` result export and audit-manifest sections. |
> | ID-map layout | Canonical ID, package member path, row/object reference, target/generated identity where any exists, and omission/unsupported reason. | Versioned stable-ID map evidence exists; preserve traceability and test actual round trips. | `DEL-17-02/ScopeOfWork.md` stable ID map requirements; export plan stable identity rules. |
> | Loss-report layout | Exported, omitted, approximated, delegated, unsupported, and `TBD` behavior with affected IDs and downstream implication. | Versioned loss-report schema evidence exists; losses remain mandatory on success. | `DEL-17-02/ScopeOfWork.md` loss report requirements. |
> | Validation-report layout | Shape checks, synchronization checks, source-evidence checks, protected-content checks, diagnostics, and boundary findings. | Versioned validation/diagnostics contract evidence exists; assess actual scope/coverage. | `docs/SPEC.md` result export boundary; `docs/IP_AND_DATA_BOUNDARY.md` contribution and public-data policy. |
>

### CLM-008 — References

> ##### References
>
> | Source | Use |
> |---|---|
> | `execution/_ScopeChange/SCA-004_2026-05-18_0000/ACCEPTANCE_RECORD.md` and DEL-17-01 `ScopeOfWork.md` (current export scope/source boundary; the former export-plan path is unavailable historical strategy, not field-level authority) | Stress-neutral package purpose, recommended contents, CSV/JSON format split, export risk framing. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | DEL-17-06 scope, SOW-046/SOW-074 assignment, PKG-17 exclusions, architecture-basis context. |
> | `docs/SPEC.md` | Result export envelope boundary, required result-export metadata, `TBD` status for additional formats. |
> | `docs/TYPES.md` | Result/export, stable identity, data-boundary, no-bypass, and professional-boundary vocabulary. |
> | `docs/CONTRACT.md` | IP, data, unit, and professional-responsibility invariants. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private/protected data boundary and contribution constraints. |
> | `DEL-17-01` `ScopeOfWork.md` | Source-basis and target-claim boundaries for PKG-17. |
> | `DEL-17-02` `ScopeOfWork.md` | Common export package, profile, stable ID map, manifest, and loss-report contract consumed by DEL-17-06. |

### CLM-009 — D-41 R5 T2A canonicalization evidence (2026-07-12)

> ##### D-41 R5 T2A canonicalization evidence (2026-07-12)
>
> The 0.1 package's JSON checksums used `deterministic_sorted_compact_json_payload_hash` (sorted-key compact ASCII-escaped Python JSON, explicitly not RFC 8785/JCS). 0.1 is no longer current: the current 0.3.0 package hashes JSON through checked `openpipestress_jcs_ijson_v1`, and CSV members keep their separate labels (`normalized_ascii_lf_text`, or `utf8_csv_record_lf_v1` for a member that is not ASCII).
>
> Version boundary: stress-neutral 0.3.0 is the only version (owner, 2026-10-10). Its JSON member and package checksums use checked `openpipestress_jcs_ijson_v1`; CSV members keep separately labeled normalized text; received-source checksum metadata is preserved separately. 0.1 and 0.2 packages are refused by name and their bytes are never relabeled.

## Completion and Reliance Basis — Epistemology

### CLM-010 — Specification: DEL-17-06 Stress-neutral CSV/JSON package

> #### Specification: DEL-17-06 Stress-neutral CSV/JSON package
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-012 — Scope

> ##### Scope
>
> DEL-17-06 shall define the documentation-level requirements for a project-controlled stress-neutral CSV/JSON package used for review, regression comparison, and downstream tooling.
>
> The original Phase A run produced only a document kit. Current scope includes the accepted stress-neutral contract and bounded versioned producers under D-67; consult the schemas and `core/handoff/stress_neutral/` as implementation evidence. Further work needs its authorized scope and may not infer release compatibility, professional acceptance or whole-product completion.
>
> The stress-neutral package shall remain result/package evidence. It is not a vendor solver format, not a commercial solver input deck, not a CAEPIPE compatibility claim, and not a substitute for human professional review.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source basis |
> |---|---|---|
> | DEL-17-06-REQ-001 | The package shall support review, debugging, community adapter development, and comparison across target exporters without presenting itself as a vendor format. | Preserved deliverable obligation; accepted SCA-004 export scope and SOW-083 in `_CONTEXT.md` (retired in `497a8d81e4`; see Git history). The unavailable historical export plan is not current field-level evidence. |
> | DEL-17-06-REQ-002 | The package shall provide CSV output for inspection/spreadsheet review and JSON output for structured import/export. | Preserved deliverable obligation; accepted SCA-004 export scope and SOW-083 in `_CONTEXT.md` (retired in `497a8d81e4`; see Git history). The unavailable historical export plan is not current field-level evidence. |
> | DEL-17-06-REQ-003 | The package shall preserve canonical SWBPIPE identity through direct row/object fields or sidecar ID-map records. Exact field names are `TBD`. | `DEL-17-02/ScopeOfWork.md` stable ID map requirements |
> | DEL-17-06-REQ-004 | The package shall include or reference a manifest that records source model/run basis, package member inventory, hashes where applicable, diagnostics, and boundary notes. Exact manifest layout is `TBD`. | `DEL-17-02/ScopeOfWork.md` manifest requirements; `docs/SPEC.md` result export section |
> | DEL-17-06-REQ-005 | The package shall include a loss report even when export succeeds, with exported, omitted, approximated, delegated, unsupported, and `TBD` behavior visible. | `DEL-17-02/ScopeOfWork.md` loss report requirements |
> | DEL-17-06-REQ-006 | Result values shall carry explicit unit and dimensional metadata or produce blocking diagnostics. Exact CSV/JSON representation is `TBD`. | `docs/SPEC.md` result export section; `docs/CONTRACT.md` OPS-K-UNIT-1 |
> | DEL-17-06-REQ-007 | The package shall carry diagnostics, warnings, unresolved assumptions, provenance, reproducibility references, and professional-boundary notices when those are present in the source result/export envelope. | `docs/SPEC.md` result export section; `docs/TYPES.md` `ResultExportEnvelope` |
> | DEL-17-06-REQ-008 | The package shall not copy protected standards text, protected tables, proprietary formulas, code-specific allowables, SIF/flexibility values, private rule-pack payloads, private project data, or proprietary commercial examples into public artifacts. | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/TYPES.md` |
> | DEL-17-06-REQ-009 | The package shall not declare code compliance, certification, sealing, professional approval, formal validation, engineering acceptance, or release readiness (PRD §21.2). | `docs/CONTRACT.md` OPS-K-AUTH-1; `DEL-17-01/ScopeOfWork.md`; `DEL-17-02/ScopeOfWork.md` |
> | DEL-17-06-REQ-010 | Target-specific or version-sensitive behavior not resolved by source evidence shall remain `TBD` and shall not be represented as supported behavior. | `DEL-17-01/ScopeOfWork.md`; `DEL-17-02/ScopeOfWork.md` |
> | DEL-17-06-REQ-011 | The stress-neutral profile shall carry source-basis references for DEL-08-04, DEL-14-02, DEL-14-05, and DEL-17-02; missing references shall block package acceptance. | DAG-006 DEL-17-06 dependency edges; DEL-17-02 export contract; DEL-08-04 result export format; DEL-14-02 analysis run records; DEL-14-05 comparison export contracts. |
>

### CLM-014 — Standards

> ##### Standards
>
> | Standard or contract | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 | Baseline for future public JSON schemas/interchange contracts. | Applicable architecture basis; no schema file is created in this Phase A task. |
> | Declared deterministic JSON hash basis | Stress-neutral 0.3.0 JSON member and package checksums use checked `openpipestress_jcs_ijson_v1`; CSV normalized-text hashes and received-source checksum evidence remain separately identified. The 0.1 sorted-compact label belongs only to refused 0.1 packages. | Owner decision 2026-10-10; D-67; exact non-current payload partitioning remains `TBD`. |
> | SWBPIPE result export envelope | Governs result identity, model/run references, unit-aware values, diagnostics, provenance, reproducibility refs, statuses, rule-pack refs, and downstream-use declarations. | Source-grounded contract basis; layout evidence is in `schemas/stress_neutral_export.v0.3.schema.json` and `core/handoff/stress_neutral/package.py`. Verify actual output identity, units, diagnostics, references and member layout against 0.3.0. |
> | DEL-17-02 common export contract | Governs export package, profile, stable ID map, manifest, and loss-report behavior. | Required upstream contract for DEL-17-06. |
> | IP and data-boundary policy | Governs public/private/protected data use and contribution review. | Required for fixtures, examples, reports, and exported artifacts. |
>
>
> Version boundary: stress-neutral 0.3.0 is the only version (owner, 2026-10-10). Its JSON member and package checksums use checked `openpipestress_jcs_ijson_v1`; CSV members keep separately labeled normalized text; received-source checksum metadata is preserved separately. 0.1 and 0.2 packages are refused by name and their bytes are never relabeled.

### CLM-015 — Verification

> ##### Verification
>
> | Verification ID | Check | Acceptance evidence |
> |---|---|---|
> | DEL-17-06-VER-001 | Confirm all migrated `ScopeOfWork.md` sections exist and preserve required default sections. | `tools/validation/check_four_documents.sh` result. |
> | DEL-17-06-VER-002 | Confirm the deliverable-local minimum fileset remains present. | `tools/validation/check_min_viable_fileset.sh` result. |
> | DEL-17-06-VER-003 | Review the migrated `ScopeOfWork.md` sections for prohibited claims (PRD §21.2): code compliance, professional approval, formal validation, release compatibility, or engineering acceptance. | Manual text review or grep-based evidence in run record. |
> | DEL-17-06-VER-004 | Review the migrated `ScopeOfWork.md` sections for protected/proprietary copied examples or protected standards data. | Manual text review or grep-based evidence in run record. |
> | DEL-17-06-VER-005 | Check 0.3 columns, JSON shape and manifest against the schema/producer contract; carry genuinely unresolved target behavior, tolerance defaults and comparison semantics explicitly. | Versioned schema/source review and scoped output evidence; historical Phase A TBDs do not override D-67. |
>
> Future implementation verification, once schemas, writer outputs, fixtures, manifests, and comparison records exist:
>
> | Verification ID | Check | Acceptance evidence |
> |---|---|---|
> | DEL-17-06-VER-006 | Map DEL-17-06-REQ-001 through DEL-17-06-REQ-010 to concrete schema, exporter, fixture, manifest, loss-report, and validation evidence. | Traceability table in a future implementation run record; unresolved entries remain `TBD` rather than accepted. |
> | DEL-17-06-VER-007 | Confirm CSV and JSON representations are synchronized for canonical identity, units/dimensions, source model/run references, manifest basis, loss-report content, diagnostics, and boundary notices. | Future paired CSV/JSON fixture diff, manifest review, and validation-report result. |
> | DEL-17-06-VER-008 | Verify 0.3.0: JSON members/package projections use checked `openpipestress_jcs_ijson_v1`; received checksums are preserved separately; CSV normalized-text hashing stays separately labeled; 0.1 and 0.2 packages are refused by name. | 0.3 exact-byte/hash vectors, mutation, schema, member/package projection and JSON/CSV separation evidence; no legacy byte relabeling. |
> | DEL-17-06-VER-009 | Review generated schemas, CSV outputs, JSON outputs, manifests, loss reports, validation reports, and fixtures for prohibited claims and protected/private content. | Future protected-content and professional-boundary review evidence; suspected protected content quarantined per policy. |
> | DEL-17-06-VER-010 | Confirm CSV/JSON examples or fixtures are invented or rights-cleared and demonstrate only shape, identity, units, diagnostics, loss reporting, and boundary notices. | Future contribution/provenance review evidence; examples without redistribution evidence remain absent or `TBD`. |
> | DEL-17-06-VER-011 | Confirm any target-specific or version-sensitive support flag is backed by cited source evidence, or is marked `TBD` and blocked from support claims. | Future source-evidence review against DEL-17-01/DEL-17-02 and the applicable source-basis register. |
> | DEL-17-06-VER-012 | Confirm comparison tolerance profiles, pass/fail language, and export comparison semantics are consumed from DEL-14-02/DEL-14-05 before comparison claims are made. | Future upstream dependency review; comparison semantics remain diagnostic/audit-only until sourced. |
> Fixture-specific hold: For the accepted 830-row sparse fixture only, resolve or formally disposition unit preservation for `result:nonlinear-support:free-dof-work-residual` and `result:loadcase:load-L-200:nonlinear-support:free-dof-work-residual`. Retain every raw source row. No conclusion about other producers, schema conformance or universal row counts follows.
>
>
> Version boundary: stress-neutral 0.3.0 is the only version (owner, 2026-10-10). Its JSON member and package checksums use checked `openpipestress_jcs_ijson_v1`; CSV members keep separately labeled normalized text; received-source checksum metadata is preserved separately. 0.1 and 0.2 packages are refused by name and their bytes are never relabeled.

### CLM-016 — Documentation

> ##### Documentation
>
> Required Phase A documentation artifacts:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_STATUS.md` safe update from `OPEN` to `INITIALIZED` if permitted by the four-documents skill
> - `_run_records/TASK_RUN_2026-05-18_1155.md`
> - `_run_records/TASK_RUN_2026-05-18_1218.md` for Pass 3 disposition evidence
>
> Future implementation artifacts listed in `_CONTEXT.md` remain not produced in this Phase A task:
>
> - stress-neutral CSV schema
> - stress-neutral JSON schema
> - export writer
> - comparison fixtures
>

### CLM-017 — Conflict Table

> ##### Conflict Table
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | none | No source conflict identified in Phase A. Remaining details are unresolved `TBD` items, not source contradictions. | n/a | n/a | n/a | n/a | n/a |

### CLM-018 — D-41 R5 T2A canonicalization requirement (2026-07-12)

> ##### D-41 R5 T2A canonicalization requirement (2026-07-12)
>
> DEL-17-06 JSON hashes SHALL use checked `openpipestress_jcs_ijson_v1`, the contract of stress-neutral 0.3.0, the only version (owner, 2026-10-10). The `deterministic_sorted_compact_json_payload_hash` label belonged to 0.1 packages, which are refused by name; it SHALL NOT appear on a current package and SHALL NOT be interpreted as RFC 8785/JCS conformance. Normalized CSV hashes SHALL remain separately labeled: `normalized_ascii_lf_text`, or `utf8_csv_record_lf_v1` for a member that is not ASCII.

- **AC-001** — The contract preserves synchronized CSV/JSON identity, units and dimensions, source model/run references, manifest, stable-ID map, loss report, diagnostics, provenance, declared JSON hashing of the one current version (0.3.0, checked `openpipestress_jcs_ijson_v1`), protected/private-data controls, explicit TBD behavior, and professional non-authority without claiming vendor compatibility or code compliance.

## Production and Verification Method — Praxeology

### CLM-019 — Procedure: DEL-17-06 Stress-neutral CSV/JSON package

> #### Procedure: DEL-17-06 Stress-neutral CSV/JSON package
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-021 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and review the Phase A `ScopeOfWork.md` for the stress-neutral CSV/JSON package, and how later implementation work should consume it without crossing into code, schema, release, compatibility, or professional-approval claims.
>

### CLM-022 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status in Phase A |
> |---|---|
> | DEL-17-01 source-basis kit available | Read as upstream context. |
> | DEL-17-02 export package/profile/stable-ID/loss-report contract available | Read as upstream context. |
> | DEL-08-04 result export format | Declared upstream dependency; detailed result schema behavior remains `TBD` in this deliverable. |
> | DEL-14-02 and DEL-14-05 comparison/run contracts | Declared upstream dependencies; exact comparison export semantics remain `TBD` in this deliverable. |
> | Governing references in `_REFERENCES.md` | Read as source basis. |
> | Protected/proprietary examples | Not admitted. |
>

### CLM-023 — Steps

> Historical population-run method and checks: retain the original commands, artifact expectations and permitted transition as evidence of that bounded run. Current maintenance uses `ScopeOfWork.md` and its applicable validation contract; this passage neither repeats that lifecycle transition nor asserts a new validation pass.

> ##### Steps
>
> 1. Confirm the deliverable identity from `_CONTEXT.md`: DEL-17-06, PKG-17, `BACKEND_FEATURE_SLICE`, SOW-046 and SOW-074.
> 2. Confirm `_STATUS.md` permits a P1/P2 four-documents write. If current state is outside the allowed overwrite states, stop without overwriting production documents.
> 3. Read `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, and the local `_SEMANTIC.md` placeholder.
> 4. Read the relevant decomposition, export plan, governance, data-boundary, and result-export source slices.
> 5. Read DEL-17-01 and DEL-17-02 four-document kits for source-basis, target-boundary, stable-ID, manifest, and loss-report carryforward.
> 6. Populate `ScopeOfWork.md` with source-grounded statements only.
> 7. Preserve `TBD` for unresolved target behavior, including exact CSV table names, CSV columns, JSON schema/object layout, package-member paths, comparison tolerances, validation thresholds, and external-target mappings.
> 8. Exclude proprietary examples, protected standards content, private project data, code-specific allowables, SIF/flexibility values, and professional/code-compliance claims.
> 9. Run local validation checks allowed for this Phase A documentation task.
> 10. If the starting lifecycle state is `OPEN`, update `_STATUS.md` to `INITIALIZED` using the four-documents safe-update rule.
> 11. Update the run record with changed files, validation, missing inputs, and remaining human-ruling items.
>

### CLM-024 — Later Implementation Consumption

> ##### Later Implementation Consumption
>
> For authorized maintenance or extension of the stress-neutral package, use this `ScopeOfWork.md` and the selected version contract; keep work within the active brief. Existing versioned reference producers are evidence, not proof of product integration.
>
> 1. Re-read DEL-17-01 and DEL-17-02 before defining support claims, package contracts, stable-ID behavior, manifest content, or loss-report behavior.
> 2. Inspect the 0.3 schema and `core/handoff/stress_neutral/package.py` for fields, member paths, object layouts and CSV columns. Modify those interfaces only within authorized scope; do not relabel bytes of refused older versions or infer a schema change from reference output.
> 3. Implement writer behavior against the common package/profile/ID-map/manifest/loss-report contract and the result export envelope boundary.
> 4. Create invented or rights-cleared fixtures only after recording provenance, redistribution status, and protected-content review evidence.
> 5. Validate CSV and JSON outputs for shared identity, unit/dimensional metadata, source model/run references, manifest basis, loss-report content, diagnostics, and boundary notices.
> 6. Preserve comparison tolerances, pass/fail wording, and export comparison semantics as `TBD` until DEL-14-02/DEL-14-05 provide the applicable upstream basis.
> 7. Bind generated schemas, CSV/JSON outputs, manifests, loss/validation reports and fixtures to their actual candidate/version and validation record. These are implementation evidence, not code-compliance or professional-acceptance evidence.
>

### CLM-025 — Verification

> ##### Verification
>
> Run from repository root:
>
> ```bash
> tools/validation/check_four_documents.sh "execution/PKG-17/DEL-17-06"
> tools/validation/check_min_viable_fileset.sh "execution/PKG-17/DEL-17-06"
> rg -n "certif|seal|code compliance|professional approval|engineering acceptance|formal validation|release readiness|proprietary example|protected standards" "execution/PKG-17/DEL-17-06" # prohibited-claims screen per PRD §21.2
> git diff --check -- "execution/PKG-17/DEL-17-06"
> ```
>
> Expected Phase A verification result:
>
> - `ScopeOfWork.md` exists;
> - minimum viable fileset remains present;
> - professional-boundary terms appear only as prohibited-claim language;
> - no implementation code, schema files, fixtures, or dependency extraction artifacts are created;
> - `TBD` remains visible for unresolved implementation and target behavior.
>

### CLM-026 — Records

> ##### Records
>
> Phase A records:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_STATUS.md`
> - `_run_records/TASK_RUN_2026-05-18_1155.md`
> - `_run_records/TASK_RUN_2026-05-18_1218.md`
>
> Records intentionally not produced in this Phase A task:
>
> - `Dependencies.csv`
> - `_SEMANTIC_LENSING.md`
> - stress-neutral CSV schema
> - stress-neutral JSON schema
> - export writer
> - comparison fixtures
> - implementation tests
>

### CLM-027 — Closeout Checks

> ##### Closeout Checks
>
> - Changed files remain inside the allowed DEL-17-06 write scope.
> - `MEMORY.md` remains unchanged as instructed.
> - No sibling DEL-17 folders are edited.
> - No later skills are run.
> - No schema, code, release, or compatibility claim is made. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>
> Future implementation closeout checks, once implementation artifacts exist:
>
> - Generated stress-neutral CSV schemas, JSON schemas, CSV outputs, JSON outputs, manifests, loss reports, and fixtures are present only if authorized by that later task.
> - Package-member paths, source model references, analysis-run references, manifest references, ID-map references, and validation-report references are populated or explicitly left `TBD`.
> - CSV and JSON representations pass a synchronization check for identity, units, manifest basis, loss-report content, diagnostics, and boundary notices.
> - Hash records identify payload scope; JSON payload hashes use checked `openpipestress_jcs_ijson_v1`, CSV hashes use normalized ASCII/LF text, and non-JSON partitioning remains explicit.
> - Fixture/example data has provenance and redistribution evidence, or remains absent/`TBD`.
> - Target-specific support flags cite source evidence or remain `TBD`.
> - Comparison semantics cite DEL-14-02/DEL-14-05 or remain diagnostic/audit-only and `TBD`.
>
> Version boundary: stress-neutral 0.3.0 is the only version (owner, 2026-10-10). Its JSON member and package checksums use checked `openpipestress_jcs_ijson_v1`; CSV members keep separately labeled normalized text; received-source checksum metadata is preserved separately. 0.1 and 0.2 packages are refused by name and their bytes are never relabeled.

### CLM-028 — D-41 R5 T2A canonicalization check (2026-07-12)

> ##### D-41 R5 T2A canonicalization check (2026-07-12)
>
> Verify exact serialized bytes, a fixed SHA-256 vector, ordering stability, mutation sensitivity, the JSON/CSV label split and schema acceptance before accepting checksum evidence; a JCS label is claimed only for the checked profile.

- **VER-001** — Validate the contract and review source parity, synchronized CSV/JSON package obligations, canonical identity, units and hash labels, source/run and manifest bindings, loss and diagnostic visibility, retained TBDs, protected-content exclusions, and absence of vendor, release, compliance, validation, or professional-acceptance claims.
>
> Version boundary: stress-neutral 0.3.0 is the only version (owner, 2026-10-10). Its JSON member and package checksums use checked `openpipestress_jcs_ijson_v1`; CSV members keep separately labeled normalized text; received-source checksum metadata is preserved separately. 0.1 and 0.2 packages are refused by name and their bytes are never relabeled.

## Governing Values and Decisions — Axiology

### CLM-029 — Guidance: DEL-17-06 Stress-neutral CSV/JSON package

> #### Guidance: DEL-17-06 Stress-neutral CSV/JSON package
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-031 — Purpose

> ##### Purpose
>
> Use this deliverable to keep the stress-neutral CSV/JSON package narrow and auditable. Its job is to expose result/package evidence in a project-controlled form that humans and downstream tools can inspect, compare, and adapt.
>
> The package must not be described as a vendor compatibility guarantee; it is handoff evidence for external validation, not a validation outcome. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-032 — Principles

> ##### Principles
>

### CLM-033 — Keep the package stress-neutral

> ###### Keep the package stress-neutral
>
> The package should describe stress-model and result concepts in SWBPIPE terms, then preserve enough identity, units, diagnostics, provenance, and loss reporting for consumers to understand what was exported. It should not encode target-solver hidden defaults or code-specific acceptance criteria.
>

### CLM-034 — Prefer explicit gaps over silent meaning

> ###### Prefer explicit gaps over silent meaning
>
> A useful stress-neutral export can still be incomplete. Missing columns, omitted entity families, unsupported result classes, unresolved unit mappings, and unsupported comparison semantics should be represented as `TBD`, loss-report entries, diagnostics, or blocking findings rather than guessed defaults.
>

### CLM-035 — Preserve identity before convenience

> ###### Preserve identity before convenience
>
> CSV is useful for review, but row order is not identity. JSON is useful for structured interchange, but object nesting is not enough by itself. Both formats should preserve stable canonical IDs or reference an ID map so rows and objects can be reconciled with source model entities, analysis runs, and comparison records.
>

### CLM-036 — Treat results as evidence

> ###### Treat results as evidence
>
> The package may help review, regression comparison, downstream tooling, and adapter development. It does not authenticate an engineering design or establish professional reliance. Any human acceptance reference remains separate and hash-bound outside this package.
>
> If a future workflow needs to point at a human acceptance reference, this package should carry only a neutral external reference such as a model-state, report, analysis-run, or project-governance record identifier plus the reviewed hash basis. The reference target, reviewer authority, and acceptance meaning stay outside the stress-neutral CSV/JSON package, and the package must not convert that reference into a software-generated authority status (PRD §21.2).
>

### CLM-037 — Considerations

> ##### Considerations
>
> | Topic | Guidance |
> |---|---|
> | CSV tables | Use table-oriented outputs for inspection and spreadsheet workflows, but bind exact table/column/serialization evidence to the selected version schema and producer. |
> | JSON package | Use JSON for structured import/export and future validation, but use the 0.3 JSON Schema 2020-12 schema; the original Phase A no-schema limit is historical. |
> | Units | Every result value needs explicit unit and dimensional context or a blocking diagnostic. Do not rely on spreadsheet headers alone for unit meaning. |
> | Result values | Displacements, rotations, forces, moments, reactions, stress, ratios, and rule-check values are governed by result-export boundaries. Current supported/withheld family evidence is versioned in the schemas and producers; retain unknown-family withholding and source diagnostics. |
> | Materials and sections | Include identity/provenance surfaces only unless user-cleared data is present. Do not bundle protected tables, allowables, catalog defaults, SIF/flexibility values, or proprietary values. |
> | Load/design cases | Preserve source basis and identity. Do not imply code-specific load combinations unless user/private rule data supplies them with provenance. |
> | Comparisons | Use stable IDs and declared tolerance profiles when comparison semantics are later defined. Tolerances and pass/fail language are `TBD` in Phase A. |
> | External solver data | CAEPIPE CSV/text parsing can provide regression or handoff evidence under DEL-17-05, but it does not make this package professionally authoritative. |
>

### CLM-038 — Trade-offs

> ##### Trade-offs
>
> | Choice | Benefit | Risk | Phase A disposition |
> |---|---|---|---|
> | CSV plus JSON | Serves both spreadsheet review and structured downstream tooling. | Two representations can drift. | Require shared identity, units, manifest, and validation basis; exact synchronization checks `TBD`. |
> | Broad table inventory | Makes the package useful across adapters. | Can imply support for semantics not yet implemented. | List candidate table families but keep field-level behavior `TBD`. |
> | Loss report on successful exports | Prevents silent omissions and approximations. | Adds package complexity. | Required by DEL-17-02 common contract. |
> | Stress-neutral vocabulary | Avoids vendor lock-in. | Consumers may need adapter-specific mapping. | Use profile/manifest/ID map/loss report to keep mappings explicit. |
> | Regression comparison support | Supports deterministic review over time. | Users may mistake comparison evidence for professional acceptance. | Comparison remains diagnostic/audit functionality only. |
>

### CLM-039 — Examples

> ##### Examples
>
> No concrete CSV rows, JSON payloads, proprietary examples, solver outputs, protected standards data, or public engineering fixtures are included in Phase A.
>
> Acceptable future examples should be invented or otherwise rights-cleared and should demonstrate shape, identity, units, diagnostics, loss reporting, and boundary notices without embedding protected or private engineering values.
>

### CLM-040 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | none | No source conflict identified in Phase A. | n/a | n/a | n/a | n/a | n/a |

### CLM-041 — D-41 R5 T2A canonicalization guidance (2026-07-12)

> ##### D-41 R5 T2A canonicalization guidance (2026-07-12)
>
> Do not describe Python's sorted-compact `canonical_json` as JCS. The current version hashes through the checked shared Rust canonicalizer (`openpipestress_jcs_ijson_v1`); the `deterministic_sorted_compact_json_payload_hash` label belongs only to refused 0.1 packages.

### CLM-042 — One current version: stress-neutral 0.3.0 (owner, 2026-10-10)

D-67 (2026-09-14) adopted the checked JSON hash contract; 0.3.0 carries it. The owner decided on 2026-10-10 that each format keeps only its current version ("as you recommended, no-compatibility principle carries"). Stress-neutral 0.3.0 (`schemas/stress_neutral_export.v0.3.schema.json`) is that version. It uses the shared Rust canonicalization authority with the checked `openpipestress_jcs_ijson_v1` input profile for JSON member and package hashes; CSV keeps its separately labeled normalized text contract. 0.1 and 0.2 packages are refused by name ("re-export") with no migration; B7a removes their code and replaces the 0.1-derived 0.3 builders with native ones (DEL-04-08 Design, `b7-plan.md`).

Received source checksum metadata is preserved separately from newly computed package checksums. Exact payload projections bind each checksum; the complete package checksum excludes only its own carrier. No historical Python bytes are relabeled or verified. Complete emitted-payload schema validation, byte/hash parity, source preservation and declared-member materialization are required.

Authority: `execution/_Coordination/_DECISIONS/D-67_RULING_2026-09-14.md`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-046 SOW-074 OBJ-007 OBJ-017 OBJ-018 | CLM-010, CLM-042 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-17-06:1** — R5 continuation delivery task: Demonstrate actual stress-neutral producers and emitted member bytes against the 0.3.0 schema, preserving identity, dimensions/units, diagnostics, losses, received checksum evidence and member/package projections. Keep comparison/default-tolerance and fixture-provenance residuals explicit (CLM-005/007/013/015/037; AC-001).
- **DEL-17-06:2** — For the accepted 830-row sparse fixture only, resolve or formally disposition the two explicitly withheld unit-preservation witnesses for diagnostic-work rows `result:nonlinear-support:free-dof-work-residual` and `result:loadcase:load-L-200:nonlinear-support:free-dof-work-residual`. The package retains every raw row; this fixture-scoped hold is not a dropped-row, schema-failure, universal-row-count, or other-producer claim.
- **DEL-17-06:3** — `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` were marked historical under the owner's selected disposition; their 2026-05-18 statements that exact schema/hash partitioning is unresolved must not be used as current evidence. `_DEPENDENCIES.md` now points to the approved DAG-011 for current navigation, while its older row counts remain dated extraction evidence. A source-faithful dependency-workflow backcheck against `Dependencies.csv` and DAG-011 is still open; neither the local register nor graph authority changed in this retirement.
