# Guidance: DEL-12-02 Private data redaction and export controls

## Purpose

This deliverable keeps OpenPipeStress report and export workflows aligned with the product boundary: public mechanics and schemas are allowed, while private project values, user-supplied code data, rule-pack details, component/vendor data, material allowables, and protected standards content remain controlled.

The original guidance was setup-level. Current June 7 evidence now includes a concrete schema contract, metadata-only helper, focused invented-fixture tests, and security documentation. This guidance still does not select final persisted profile storage, UI design, CLI/API/public transport behavior, export format, cloud exception workflow, legal review workflow, destructive quarantine movement, or approval workflow.

## Principles

| Principle | Guidance |
|---|---|
| Explicit export context | Treat local private export, shared redacted export, public template/example export, and downstream-tool handoff as different privacy contexts. |
| Warn or redact by configuration | The product should not silently include private or protected values in outputs that may be shared. |
| Block when provenance is unsafe | Unknown, protected-suspected, or private-only redistribution status should trigger warning, redaction, or block behavior. |
| Preserve reproducibility evidence | Redaction should keep non-sensitive hashes, versions, warnings, provenance summaries, and rule-pack identifiers where safe. |
| Do not mutate source data | Redaction is an export/report transformation, not a change to the authoritative project model or private libraries. |
| No-bypass exports | GUI, CLI, adapters, plugins, public APIs, and downstream handoffs must use the same unit, provenance, diagnostic, sandboxing, and data-boundary checks. |
| Human authority | Redaction/export controls support review; they do not certify, seal, approve, authenticate, or declare code compliance. |

## Considerations

### Private and Protected Value Classes

Treat these as sensitive in export/report decisions unless documented redistribution rights and user intent say otherwise:

- private project model coordinates, loads, equipment loads, owner requirements, design bases, and project identifiers;
- private rule-pack formulas, allowables, interpretations, source excerpts, and code-specific checking logic;
- material properties, allowable-like values, temperature-dependent data, and source/license metadata;
- component and section library values, manufacturer/vendor data, SIF/flexibility-like values, stiffnesses, dimensions, and proprietary catalog-derived records;
- private report templates or owner/company calculation report formats;
- protected standards text, tables, copied formulas, figures, examples, or commentary.

### Export Contexts

`LOCAL_PRIVATE` can preserve private values for the user's own review, but it should still create an explicit warning/audit record. `SHARED_REDACTED`, `PUBLIC_TEMPLATE`, and `PUBLIC_EXAMPLE` should assume disclosure risk and prefer redaction or block behavior. `DOWNSTREAM_TOOL` may need values for technical continuity, but the adapter must still preserve privacy, provenance, unit, and diagnostic checks.

### Report Boundary

Reports may safely reference rule-pack ID, version, checksum, and source note when those fields do not disclose protected/private content. Reports must not embed protected standards formulas, copied tables, proprietary report templates, or private values in public templates/examples. Users remain responsible for private report templates that quote licensed standards outside the public project.

### Manifest Boundary

A redacted export should not become useless for audit. Runtime integration should preserve non-sensitive reproducibility evidence such as hashes, versions, redaction profile ID, warning summaries, and provenance summaries while avoiding leakage of the redacted values themselves.

### Adapter and Plugin Boundary

Adapters and plugins are export risk multipliers. They should never bypass redaction configuration, provenance checks, unit checks, report controls, diagnostics, or rule-pack sandboxing. A downstream-tool export is still an export boundary, not an exemption.

### Current Evidence Boundary

The current helper is metadata-only. It blocks or redacts explicit storage/privacy markers for payload presence, secret material, cloud/network references, direct SQL/raw SQLite access, storage bypasses, concrete path indicators, and local/private intent. It does not read or write private paths, move quarantine material, store secrets, select storage roots, integrate report/export runtime routes, authorize cloud behavior, or decide legal/security/professional sufficiency.

## Trade-offs

| Trade-off | Implication |
|---|---|
| Redact values vs. preserve review detail | Redaction lowers disclosure risk but can reduce review usefulness. Current schema/helper evidence preserves safe metadata classes; full report/export manifest integration remains `TBD`. |
| Warn-only vs. block export | Warn-only preserves user agency but can permit accidental disclosure. Current metadata hardening blocks high-risk storage/privacy markers while final approval and override workflow remains `TBD`. |
| Field-level vs. value-level redaction | Field-level redaction is simpler and safer; value-level redaction can retain structure but may leak through labels, units, hashes, or context. |
| Local private export vs. public template export | Local private exports may need full detail; public templates/examples must remain protected-data-free and invented-data-only. |
| Rich downstream handoff vs. minimum disclosure | Downstream tools may require detail, but adapters must not become a bypass path for private data or protected content. |

## Examples

The following are symbolic examples only:

| Scenario | Acceptable Planning Expression | Avoid |
|---|---|---|
| Public report example | Invented model values plus redaction profile `PUBLIC_EXAMPLE` | Real user project values or protected standards examples |
| Rule-pack summary | Rule-pack ID/version/checksum/source note with private formula details redacted | Copying protected formula text or code-derived tables into a public report |
| Material library export | Provenance summary and redistribution status; private values redacted for shared output | Public material allowable table without documented rights |
| Local private report | Unredacted values only after explicit local-private export selection and warning/audit record | Silent unredacted export to a shareable/public location |
| Downstream-tool handoff | Adapter route that preserves units, provenance, diagnostics, and privacy controls | Plugin export that bypasses data-boundary checks |

## Open Issues and TBDs

| Issue ID | Topic | Status | Notes |
|---|---|---|---|
| REXC-OI-001 | Redaction config schema | PARTIAL / TBD | `schemas/redaction_export_controls.schema.yaml` now exists; persisted profile storage location, migration behavior, and storage roots remain `TBD`. |
| REXC-OI-002 | Export context UI and override flow | TBD | Exact GUI controls, CLI prompts, user confirmation, and approval workflow are not selected here. |
| REXC-OI-003 | Public API transport and export formats | TBD | AB-00-07 leaves public transport and concrete import/export formats open. |
| REXC-OI-004 | Executable export tests | PARTIAL / TBD | Focused helper/schema tests now exist; runtime report/export, GUI/CLI/API, adapter/plugin, and protected-content linter integration tests remain `TBD`. |
| REXC-OI-005 | Legal sufficiency of redaction | TBD | This deliverable does not claim that any redaction policy satisfies legal, client, security, or professional obligations. |
| REXC-OI-006 | Physical project package/container | TBD | Redaction may depend on project package boundaries, storage roots, and export staging behavior, which remain implementation-level `TBD`. |
| REXC-OI-007 | Runtime report/export integration | TBD | The helper/schema evidence exists, but report generator, result export, GUI, CLI, API, adapter, and public template runtime routes are not wired through this control here. |
| REXC-OI-008 | Quarantine and legal review workflow | TBD | The helper blocks suspected/protected/private metadata but does not perform destructive quarantine movement or legal review routing. |
| REXC-OI-009 | Cloud exception workflow | TBD | Default posture remains no cloud export or transmission unless separately approved; exception process and controls are not selected here. |
| REXC-OI-010 | Storage roots and concrete paths | TBD | Concrete path indicators are redacted in focused helper behavior; allowed storage roots and portable project/export package behavior remain `TBD`. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| REXC-CON-001 | The deliverable catalog anticipated redaction config and export tests; current evidence now provides a schema and focused helper tests but not full runtime integration. | `docs/_Registers/Deliverables.csv` row DEL-12-02 | `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md`; package fan-in record | Specification Requirements; Procedure Future Implementation Procedure | Treat schema/focused tests as current evidence; defer runtime integration, UI/CLI/API/adapter routes, and approval workflows. | TBD |
| REXC-CON-002 | Local private exports may need unredacted values, but shared/public outputs must avoid private/protected disclosure. | `docs/SPEC.md#8. Reporting and audit`; `docs/IP_AND_DATA_BOUNDARY.md#7. Report boundary` | SOW-040; OPS-K-PRIV-1 | Guidance Export Contexts; Specification REXC-REQ-006 | Allow explicit local-private export with warning/audit record; require redaction or block for shared/public contexts. | TBD |
| REXC-CON-003 | Focused helper/schema/tests now exist, but runtime report/export paths and approval workflows are not integrated. | `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md`; package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` | `_STATUS.md` current `IN_PROGRESS`; Specification Documentation deferred items | Specification Verification; Procedure Future Implementation Procedure; Dependencies | Treat current evidence as readiness evidence only; defer lifecycle promotion and runtime integration to owning workflows. | TBD |
