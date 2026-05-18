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
| Lifecycle Target | SEMANTIC_READY after setup gates pass |

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
| Config persistence | Future implementation must keep redaction configuration local and project/version aware; exact schema file remains TBD |
| Export-test status | Test expectations documented only; executable tests are not created by this setup run |
| Implementation status | Not implemented by this setup run |

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

This setup run records a documentation-level contract for a future redaction configuration. The exact JSON Schema, field names, and UI controls remain implementation-level TBD.

| Proposed Config Slot | Purpose | Default Setup Expectation |
|---|---|---|
| `profile_id` | Stable identifier for a redaction/export profile. | Required in future implementation. |
| `export_context` | Classifies the intended export surface. | Must be one of the configured export context values. |
| `field_classification_rules` | Maps model/report fields to public, private, protected-suspected, or unknown handling classes. | Must default unknown risky fields to warning or block, not silent inclusion. |
| `rule_pack_detail_policy` | Controls formula, allowable, interpretation, source-note, and checksum exposure. | Public/shared exports may include identity/version/checksum/source note, not protected formula text. |
| `material_value_policy` | Controls material properties, allowables, provenance, and redistribution-status exposure. | Public/shared exports redact private/protected values unless rights are documented. |
| `component_value_policy` | Controls manufacturer/vendor component fields, geometry catalogs, stiffnesses, and private modifiers. | Public/shared exports redact private/vendor-restricted values unless rights are documented. |
| `project_value_policy` | Controls project-specific loads, owner requirements, coordinates, equipment loads, and design-basis values. | Shared/public exports require explicit user intent and redaction review. |
| `manifest_policy` | Defines what evidence remains in an export manifest after redaction. | Preserve hashes/provenance/statuses without leaking redacted values. |
| `override_policy` | Defines whether unredacted private export is allowed. | Allowed only for local private context with explicit user action and warning record. |
| `diagnostic_policy` | Defines warning classes and blocking findings. | Emit `IP_BOUNDARY_WARNING` and related diagnostics where risk exists. |
| `template_guard_policy` | Controls public report templates and examples. | Protected standards text/tables/formulas and private examples are prohibited. |

## Export Test Expectations

| Test Expectation | Risk Covered | Setup Status |
|---|---|---|
| Redacted public report excludes private rule/material/component values. | Private data leakage. | Deferred executable test. |
| Public template linter rejects protected code text, copied tables, protected formulas, and proprietary examples. | Protected content leakage. | Deferred executable test. |
| Local private export can retain private values only with explicit user intent and warning/audit record. | Accidental disclosure. | Deferred executable test. |
| Redacted export preserves non-sensitive manifest evidence such as hashes, versions, warnings, and provenance summaries. | Reproducibility loss after redaction. | Deferred executable test. |
| Adapter/plugin export path cannot bypass redaction, units, provenance, sandboxing, or diagnostics. | No-bypass failure. | Deferred executable test. |
| Redaction never mutates the source project model or private libraries. | Destructive export behavior. | Deferred executable test. |

## Construction

This setup artifact constructs a documentation-level redaction/export control contract, not a product implementation. The deliverable output is limited to:

- redaction/export-control requirements in `Specification.md`;
- configuration-slot and export-test expectations in this datasheet;
- guidance on export contexts, warning behavior, and deferred implementation decisions in `Guidance.md`;
- setup and future implementation procedure in `Procedure.md`;
- semantic matrix/lensing and dependency setup artifacts.

No product source code, JSON Schema file, actual redaction config, executable export test, real project data, private rule pack, protected standards content, secret, credential, cloud operation, or certification/compliance claim is created by this run.

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
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, DEL-12-02, SOW-040, OBJ-010, and AB-00 architecture basis. |
| `docs/_Registers/Deliverables.csv` | Deliverable identity, anticipated artifacts, context/risk notes. |
| `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-040. |

