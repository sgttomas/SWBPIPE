# Guidance: DEL-12-02 Private data redaction and export controls

## Purpose

This deliverable keeps OpenPipeStress report and export workflows aligned with the product boundary: public mechanics and schemas are allowed, while private project values, user-supplied code data, rule-pack details, component/vendor data, material allowables, and protected standards content remain controlled.

The guidance is intentionally setup-level. It defines the redaction/export-control vocabulary and expected future verification without selecting a concrete config schema, UI design, export format, cloud workflow, or legal sufficiency claim.

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

A redacted export should not become useless for audit. The future implementation should preserve non-sensitive reproducibility evidence such as hashes, versions, redaction profile ID, warning summaries, and provenance summaries while avoiding leakage of the redacted values themselves.

### Adapter and Plugin Boundary

Adapters and plugins are export risk multipliers. They should never bypass redaction configuration, provenance checks, unit checks, report controls, diagnostics, or rule-pack sandboxing. A downstream-tool export is still an export boundary, not an exemption.

## Trade-offs

| Trade-off | Implication |
|---|---|
| Redact values vs. preserve review detail | Redaction lowers disclosure risk but can reduce review usefulness. Preserve non-sensitive manifest evidence and make the redaction profile visible. |
| Warn-only vs. block export | Warn-only preserves user agency but can permit accidental disclosure. Blocking is safer for public/shared contexts with unknown or protected-suspected provenance. |
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
| REXC-OI-001 | Redaction config schema | TBD | This setup run records required config slots only; exact JSON Schema and file location remain implementation-level TBD. |
| REXC-OI-002 | Export context UI and override flow | TBD | Exact GUI controls and user confirmation workflow are not selected here. |
| REXC-OI-003 | Public API transport and export formats | TBD | AB-00-07 leaves public transport and concrete import/export formats open. |
| REXC-OI-004 | Executable export tests | TBD | Future implementation must add tests; this setup run records test expectations only. |
| REXC-OI-005 | Legal sufficiency of redaction | TBD | This deliverable does not claim that any redaction policy satisfies legal, client, or professional obligations. |
| REXC-OI-006 | Physical project package/container | TBD | Redaction may depend on project package boundaries, which remain implementation-level TBD. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| REXC-CON-001 | The deliverable catalog anticipates redaction config and export tests, while the sealed setup run is limited to deliverable-local documentation and setup artifacts. | `docs/_Registers/Deliverables.csv` row DEL-12-02 | User TASK brief; `_CONTEXT.md#Anticipated Artifacts` | Specification Requirements; Procedure Future Implementation Procedure | Record config/test expectations now; defer executable config schema and tests to a later implementation task. | TBD |
| REXC-CON-002 | Local private exports may need unredacted values, but shared/public outputs must avoid private/protected disclosure. | `docs/SPEC.md#8. Reporting and audit`; `docs/IP_AND_DATA_BOUNDARY.md#7. Report boundary` | SOW-040; OPS-K-PRIV-1 | Guidance Export Contexts; Specification REXC-REQ-006 | Allow explicit local-private export with warning/audit record; require redaction or block for shared/public contexts. | TBD |

