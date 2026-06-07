# Datasheet: DEL-12-02 Private data redaction and export controls

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-12-02 |
| Deliverable Name | Private data redaction and export controls |
| Package ID | PKG-12 |
| Package Name | Security, Privacy, and Private Data Handling |
| Deliverable Type | SECURITY_CONTROL |
| Scope Item | SOW-040 |
| Objective | OBJ-010 |
| Setup Run Date | 2026-04-30 |
| Lifecycle State | `_STATUS.md` currently records `IN_PROGRESS`; this evidence alignment does not authorize lifecycle promotion |

## Attributes

| Attribute | Value |
|---|---|
| Control surface | Report/export privacy guardrails for private rule, material, component, project, and code/design-basis values |
| Primary output class | Redaction configuration contract and export-test expectations |
| Product posture | Local-first; no cloud export or transmission unless separately approved |
| Default sharing posture | Shared/public exports require explicit private-data handling and must not silently include protected/private values |
| Private data classes | Project model values; private rule-pack values; material and allowable-like values; component/manufacturer values; owner/company design-basis fields; private report-template content |
| Protected data classes | Standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data |
| Diagnostic class | `IP_BOUNDARY_WARNING` where export/report content may expose private or protected data |
| Redaction mode vocabulary | `WARN_ONLY`, `REDACT_VALUE`, `REDACT_FIELD`, `BLOCK_EXPORT`, `ALLOW_PRIVATE_EXPORT` |
| Export context vocabulary | `LOCAL_PRIVATE`, `SHARED_REDACTED`, `PUBLIC_TEMPLATE`, `PUBLIC_EXAMPLE`, `DOWNSTREAM_TOOL` |
| Config persistence | `schemas/redaction_export_controls.schema.yaml` now records the local-first control profile, field policies, decisions, and findings contract; durable project/profile storage location and storage roots remain `TBD` |
| Export-test status | Focused invented-fixture tests now exist for schema vocabulary, metadata-only classification, public/shared redaction, local-private intent, source non-mutation, and storage/privacy marker hardening; runtime report/export integration tests remain `TBD` |
| Implementation status | Metadata-only helper, schema, focused tests, and security documentation now exist as June 7 evidence; runtime report/export integration, destructive quarantine movement, legal review workflow, cloud exception workflow, storage roots, UI/CLI/public transport/export-format choices, and approval choices remain `TBD` |

## Conditions

| Condition | Constraint |
|---|---|
| Configuration required | Any export path that can expose private values must have an explicit redaction/export policy, even if the policy is to allow a local private export. |
| Default redaction boundary | Public templates and public examples must not include protected standards content, private user data, copied proprietary formulas, or vendor data without documented rights. |
| Local private export | A user may need an unredacted local export for project review, but the workflow must make the privacy status and responsibility visible. |
| Shared/public export | Shared or public-facing exports must redact or block private values according to configuration and emit an auditable warning/manifest. |
| Report manifest | Export records should preserve model/report hashes, solver/report versions, rule-pack identity/version/checksum, provenance summaries, warnings, and limitations without revealing redacted values. |
| Adapter boundary | Import/export adapters and plugins cannot bypass units, provenance, diagnostics, sandboxing, report controls, or public/private data checks. |
| Professional boundary | Redaction status, report export, or rule-pack checksum must not be framed as certification, sealing, approval, authentication, or code compliance. |

## Redaction Configuration Contract

This deliverable now has current evidence for a schema-governed redaction/export-control contract in `schemas/redaction_export_controls.schema.yaml` and a metadata-only helper under `core/security/redaction/`. The exact persisted project/profile storage location, UI controls, CLI/API/public transport behavior, external export formats, and approval workflow remain implementation-level `TBD`.

| Proposed Config Slot | Purpose | Default Setup Expectation |
|---|---|---|
| `profile_id` | Stable identifier for a redaction/export profile. | Schema slot exists; persisted profile lifecycle remains `TBD`. |
| `export_context` | Classifies the intended export surface. | Configured context vocabulary exists for public report/example, shared model, downstream tool, and local private export. |
| `field_classification_rules` | Maps model/report fields to public, private, protected-suspected, or unknown handling classes. | Helper uses explicit metadata only; unknown risky fields produce warning, redaction, or blocking behavior. |
| `rule_pack_detail_policy` | Controls formula, allowable, interpretation, source-note, and checksum exposure. | Public/shared exports may include safe metadata such as identity/version/checksum/source note, not protected formula text. |
| `material_value_policy` | Controls material properties, allowables, provenance, and redistribution-status exposure. | Focused tests cover private and unknown material-like metadata redaction; runtime source integration remains `TBD`. |
| `component_value_policy` | Controls manufacturer/vendor component fields, geometry catalogs, stiffnesses, and private modifiers. | Focused tests cover private component-like metadata redaction; runtime source integration remains `TBD`. |
| `project_value_policy` | Controls project-specific loads, owner requirements, coordinates, equipment loads, paths, and design-basis values. | Shared/public exports redact private metadata and concrete path indicators; storage roots and runtime export paths remain `TBD`. |
| `manifest_policy` | Defines what evidence remains in an export manifest after redaction. | Safe metadata such as hashes/checksums/provenance summaries can remain visible; full runtime report/export manifest integration remains `TBD`. |
| `override_policy` | Defines whether unredacted private export is allowed. | Local-private retention requires explicit local/private intent and emits warnings; approval and user-confirmation workflow remain `TBD`. |
| `diagnostic_policy` | Defines warning classes and blocking findings. | `IP_BOUNDARY_WARNING`, `PRIVATE_DATA_WARNING`, `PROVENANCE_WARNING`, `STORAGE_BOUNDARY_WARNING`, and professional-boundary findings are represented. |
| `template_guard_policy` | Controls public report templates and examples. | Guardrail documentation and focused scans exist; protected-content linter/runtime template integration and legal review workflow remain `TBD`. |

## Export Test Expectations

| Test Expectation | Risk Covered | Setup Status |
|---|---|---|
| Redacted public report excludes private rule/material/component values. | Private data leakage. | Focused invented-fixture test exists; runtime report/export integration remains `TBD`. |
| Public template linter rejects protected code text, copied tables, protected formulas, and proprietary examples. | Protected content leakage. | Guardrail scans exist for changed surfaces; protected-content linter and legal-review workflow remain `TBD`. |
| Local private export can retain private values only with explicit user intent and warning/audit record. | Accidental disclosure. | Focused invented-fixture test exists; final UI/CLI approval flow remains `TBD`. |
| Redacted export preserves non-sensitive manifest evidence such as hashes, versions, warnings, and provenance summaries. | Reproducibility loss after redaction. | Schema/docs/helper preserve safe metadata classes; full report/export manifest integration remains `TBD`. |
| Adapter/plugin export path cannot bypass redaction, units, provenance, sandboxing, or diagnostics. | No-bypass failure. | Metadata hardening blocks storage/privacy bypass markers; adapter/plugin runtime routing remains `TBD`. |
| Redaction never mutates the source project model or private libraries. | Destructive export behavior. | Focused invented-fixture source non-mutation test exists; runtime source-model integration remains `TBD`. |

## Construction

The original setup artifact constructed a documentation-level redaction/export control contract. Later bounded implementation evidence added a metadata-only redaction helper, schema, focused invented-fixture tests, and security documentation. This alignment records that evidence without changing product code or lifecycle state.

Current evidence includes:

- redaction/export-control requirements in `Specification.md`;
- configuration-slot and export-test expectations in this datasheet;
- guidance on export contexts, warning behavior, and deferred implementation decisions in `Guidance.md`;
- setup and future integration procedure in `Procedure.md`;
- schema contract in `schemas/redaction_export_controls.schema.yaml`;
- metadata-only helper in `core/security/redaction/`;
- focused tests in `tests/security/test_redaction_export_controls.py`;
- implementation guidance in `docs/security/redaction_export_controls.md`;
- semantic matrix/lensing and dependency setup artifacts.

This run does not create product source code, tests, schemas, real project data, private rule packs, protected standards content, secrets, credentials, cloud operations, destructive quarantine movement, legal sufficiency, security certification, lifecycle acceptance, or professional/code-compliance claims.

## References

| Source | Use |
|---|---|
| `INIT.md` | Bootstrap boundaries: open mechanics, protected data, private data, and no certification claims. |
| `AGENTS.md` | Type 2 scoped execution and write-scope constraints. |
| `docs/DIRECTIVE.md` | Product principles for private code data, provenance, unit safety, report auditability, stop rules, and hidden cloud/telemetry exclusion. |
| `docs/CONTRACT.md` | OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-REPORT, OPS-K-PRIV, OPS-K-RULE, and OPS-K-AGENT invariants. |
| `docs/TYPES.md` | SECURITY_CONTROL type, private/user-supplied data vocabulary, report object, rule-pack reference, and analysis-status boundaries. |
| `docs/SPEC.md` | Layered architecture, adapter no-bypass rule, report/audit content, warning classes, and validation expectations. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy, quarantine rule, and report boundary. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, DEL-12-02, SOW-040, OBJ-010, and AB-00 architecture basis. |
| `docs/_Registers/Deliverables.csv` | Deliverable identity, anticipated artifacts, context/risk notes. |
| `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-040. |
