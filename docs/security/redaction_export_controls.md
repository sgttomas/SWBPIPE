---
doc_id: OPS-SECURITY-REDACTION-EXPORT-CONTROLS
doc_kind: security.redaction_export_controls
status: draft
created: 2026-05-03
deliverable_id: DEL-12-02
package_id: PKG-12
scope_items:
  - SOW-040
objectives:
  - OBJ-010
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: informed_by
    to: OPS-SECURITY-LOCAL-FIRST-STORAGE-POLICY
  - rel: informed_by
    to: OPS-SECURITY-THREAT-MODEL
---

# Redaction And Export Controls

This document defines the `DEL-12-02` redaction and export-control behavior for
public reports, public examples, shared model exports, downstream-tool exports,
and local/private exports. The control is local-first and operates on an
export/report representation. It does not mutate source project data, move
quarantine material, transmit data to a cloud service, decide legal rights, or
provide professional approval.

## Classification Basis

Redaction decisions use explicit metadata only. The control does not infer
sensitivity from engineering-looking values, filenames, text patterns, or
hidden guesses.

A value-bearing export field should carry:

- `field_class`;
- `privacy_classification`;
- `redistribution_status`;
- `review_status`;
- `provenance` where the source is not self-evident public metadata.

Absent, unknown, `TBD`, pending, rejected, or quarantined metadata does not
silently pass into public/shared exports. It produces warning, redaction, or
blocking behavior.

## Export Contexts

| Context | Default handling |
|---|---|
| `public_report` | Include public metadata and invented public examples; redact private or unresolved values; block suspected protected content and professional-boundary claims. |
| `public_example` | Same as `public_report`; examples must be invented or public-permissive reviewed content. |
| `shared_model` | Redact private project, material, component, rule-pack, owner-standard, design-basis, path, and secret-like data unless metadata supports sharing. |
| `downstream_tool` | Redact unresolved or private values unless explicit metadata supports the handoff. |
| `local_private` | Retain private values only when explicit local/private user intent is recorded; emit warning findings. |

## Protected Classes

The default protected classes are:

- private project data;
- private material data;
- private component data;
- private rule-pack data;
- owner-standard data;
- company design-basis data;
- path data;
- secret-like data;
- suspected protected content;
- unresolved provenance or redistribution status.

Safe metadata such as IDs, versions, checksums, source notes, warning summaries,
schema versions, and provenance summaries may remain visible where metadata
marks them public or invented public examples.

## Actions

| Action | Meaning |
|---|---|
| `include` | Export the field as-is because metadata permits it. |
| `warning_only` | Retain the value and emit a warning, limited to local/private exports with explicit intent or unresolved metadata in local/private context. |
| `redact_value` | Replace the field value or text with a redaction marker while keeping non-sensitive metadata. |
| `redact_field` | Keep only minimal field identity and classification metadata. |
| `omit_field` | Remove the field from the export representation. |
| `block_export` | Mark the export as blocked until metadata, rights, protected-content, or professional-boundary issues are resolved. |

## Diagnostics

Findings are machine-readable and include a reason code, finding class,
severity, affected path, action, message, and remediation. The expected finding
classes include `IP_BOUNDARY_WARNING`, `PRIVATE_DATA_WARNING`,
`PROVENANCE_WARNING`, and `PROFESSIONAL_BOUNDARY_WARNING`.

Unknown provenance, unknown redistribution status, missing metadata, rejected
review status, quarantined review status, suspected protected content, and
professional-boundary claims are explicit findings. A clean redaction run is
review evidence only, not a legal or professional clearance.

## Source Data

The implementation copies the export/report representation before applying
redaction. Source project models, private libraries, rule packs, report inputs,
and local storage paths are not modified by this control.

## Verification Expectations

DEL-12-02 should be checked for:

- schema traceability to `DEL-12-02`, `PKG-12`, `SOW-040`, and `OBJ-010`;
- explicit metadata only classification;
- redaction of private project, material, component, rule-pack, owner-standard,
  company design-basis, path, and secret-like values in public/shared exports;
- warnings or redaction for missing provenance and unknown redistribution;
- explicit local/private intent before retaining private values;
- no cloud transmission, secret handling, source mutation, protected standards
  content, non-invented private payloads, or professional-authority assertions.
