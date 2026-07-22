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

The control also consumes explicit storage/privacy indicators when present:
`contains_payload`, `payload_present`, `secret_material_present`,
`direct_sql_access`, storage-bypass flags, `cloud_or_network_reference`,
`concrete_path_present`, symbolic path/locality metadata, and local/private
intent metadata. These indicators are treated as metadata gates only. They do
not authorize payload storage, direct SQLite access, raw SQL access, network or
cloud behavior, concrete user-path disclosure, secret handling, or source-data
mutation.

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
`PROVENANCE_WARNING`, `STORAGE_BOUNDARY_WARNING`, and
`PROFESSIONAL_BOUNDARY_WARNING`.

Unknown provenance, unknown redistribution status, missing metadata, rejected
review status, quarantined review status, suspected protected content, and
professional-boundary claims are explicit findings. A clean redaction run is
review evidence only, not a legal or professional clearance.

Storage/privacy hardening adds blocking findings for payload material, explicit
secret-material flags, cloud or network references, direct SQL or raw SQLite
access, and storage-bypass metadata. Concrete path indicators are reduced to a
redacted representation so concrete user paths are not emitted.

## Application Binding

The desktop technical preview binds this contract into the export workflow
(TP-E4-REDACTION-001):

- `apps/desktop/src/features/redaction-controls/redactionExportControls.ts` is
  a TypeScript mirror of `core/security/redaction/controls.py`. Semantic
  parity between the two implementations is pinned by the shared invented
  corpus `fixtures/redaction_export_controls/cases.json`, which both
  `tests/security/test_redaction_export_controls.py` and
  `apps/desktop/src/features/redaction-controls/redactionExportControls.test.ts`
  assert against; the corpus exercises every governed reason code.
- `apps/desktop/src/features/redaction-controls/RedactionExportControlsPanel.tsx`
  is the user-facing surface: the export context starts at `local_private`
  (private-by-default), every warning and blocking finding is rendered before
  any export artifact is offered, no download link exists while a blocking
  finding is present, private values are retained locally only after the user
  records explicit local-private intent, and the offered redacted manifest
  carries a canonical sha256 checksum computed through the same wasm hash seam
  the engines use.

The application binding classifies a metadata-only export representation of
the user-entered model; it never mutates the source model and never transmits
anything. Its warnings inform the local export decision only — they do not
certify redaction sufficiency and create no release or legal-clearance claim.
Acceptance, professional judgment, and any certification, sealing, or
code-compliance determination remain with the responsible engineer and
project authority.

## Final-Sink Route Binding

The DEL-12-02 breadth tranche applies the contract at every frozen final
exposure route. Each binding returns an observable controlled envelope before
creating a download, preview, print action, stdout value, output file, or
downstream handoff:

```text
ControlledExport<T> = {
  payload: T | null,
  decisions: RedactionDecision[],
  findings: RedactionFinding[],
  blocked: boolean,
  summary: RedactionSummary
}
```

The route projector deep-copies the source, assigns explicit metadata to each
value-bearing leaf, removes top-level and nested `local_private_intent`,
`explicit_local_private_intent`, and `user_intent`, and supplies only the
route's fixed context and wrapper-owned intent. Source-carried intent therefore
cannot grant export authority. The projector materializes only the controlled
copy and preserves the source object byte/deep-equal.

The frozen route set covers Python JSON/downstream/specialized-format writers,
both Rust headless runners, report DOM/JSON/render-IPC/HTML/save/print/lint,
and all other desktop download panels. Structured routes may emit a sanitized
payload. Lossless specialized formats are withheld if any destructive action
would corrupt their meaning. Local-private routes expose known private values
only after their own explicit-intent control is selected, with
`warning_only` evidence; unresolved local values remain unknown and
warning-only.

Desktop download links use `ControlledExportLink`. Reports are controlled
before DOM construction and before the Tauri render IPC call; blocked reports
have no raw DOM, iframe, JSON link, save link, print action, renderer call, or
browser fallback. Renderer and protected-content-lint diagnostics are composed
with the route findings when a sanitized report proceeds. The Python writers
and downstream workflow expose the same envelope and create no directory or
file on block. The Rust runners emit the same controlled shape, accept
wrapper-owned `--explicit-local-private-intent`, and write no output on block;
their mirror is pinned to the same invented parity corpus as Python and
TypeScript.

The protected-content lint CLI remains a separately governed DEC-058/DEC-059
diagnostic used only by the release scan and public-source exporter; it is not
a product report/model/result route and this tranche does not change it. No
plugin runtime/loader or bug/crash-report egress exists. Any future such route
must consume this controlled-export boundary before it can expose a payload.

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
- blocking or redaction for payload material, secret-material flags,
  cloud/network references, direct SQL/raw SQLite access, storage bypass
  markers, and concrete path indicators;
- explicit local/private intent before retaining private values;
- source-carried intent ignored at every final route and wrapper intent tested;
- observable decisions, findings, blocked state, and summary before exposure;
- no output directory/file, DOM, IPC, iframe, save, print, stdout payload, or
  browser fallback when blocking findings exist;
- canonical Python, TypeScript, and bounded Rust parity-corpus agreement;
- continued absence of plugin and bug-report egress and release-tool-only use
  of the protected-content diagnostic CLI;
- no cloud transmission, secret handling, source mutation, protected standards
  content, non-invented private payloads, concrete user paths, direct
  SQL/storage bypass, or professional-authority assertions.
