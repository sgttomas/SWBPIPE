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
| `user_released_report` | A report of the user's own session in which the user releases their own project data for sharing, for that one report, with their statement that they hold the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6). Admitted only on route `DREP-PACKAGE-USER-RELEASE-010` with the wrapper's user-release intent. After the professional-claim, network, SQL, storage-bypass, secret, payload, concrete-path, protected and quarantined/rejected checks, which apply unchanged, it decides in this order and returns: (10) a private class other than `private_project_data` is redacted (`PRIVATE_DATA_REDACTED`) whatever its redistribution status, so a lossless route withholds it; (11) `private_project_data`, whatever its redistribution status, is retained with a `USER_RELEASED_PROJECT_DATA` warning only with the wrapper's user-release intent, else blocks (`USER_RELEASE_INTENT_REQUIRED`); (12) public metadata and invented public examples are included; (13) anything else, including missing, unknown, `TBD` or unrecognized metadata, blocks (`USER_RELEASE_VALUE_UNBOUND`), never warning-only. Payload-carried intent of any name is not read. |

The five existing contexts keep every decision on every input, and the
release intent cannot enable export in them: the controls refuse
`explicit_user_release_intent` with any context other than
`user_released_report` (DEL-08-01 Design `report-user-release.md` §3), and
local-first route admission refuses it there with `block_storage` /
`USER_RELEASE_ROUTE_REQUIRED` on every route; an absent or false flag leaves
the route evidence unchanged. No route uses `user_released_report` yet.

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

User-release intent comes only from the wrapper's argument
(`explicitUserReleaseIntent` / `explicit_user_release_intent`), never from the
payload. The strip list above is unchanged: a payload member named
`user_release_intent` or `explicit_user_release_intent` grants nothing, so it is
not stripped, and in every existing context and route it is raw content,
preserved byte for byte. The user-release route `DREP-PACKAGE-USER-RELEASE-010`
has its own projector (`controlUserReleasedReportPackageRequest`). It first
decides every enclosing object of the request and of the model by the
professional-claim, network, SQL, storage-bypass, secret, payload,
concrete-path, protected and quarantined/rejected checks, exactly as the
classification decides that object. Only then does it label each leaf with the
privacy and redistribution of its nearest enclosing object that states them
(its own member, else its provenance's, as the classification reads them),
else `private_project_data` / `private_only`, with no path-token labelling. It
is lossless, and its local-first admission holds the route metadata under the
existing `private_report` storage context.

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

Remaining (not changed, since repairing them would change existing decisions):
in the five existing contexts the bounded Rust mirror reads some inputs
differently from Python and TypeScript. It does not screen `cache_payload` or
`sqlite_payload`; it does not fall back to the provenance for a null privacy or
redistribution member or for an `unknown` or null review status; and it does
not count a scalar storage-privacy metadata key as metadata present. No parity
case covers these inputs. In `user_released_report` the mirror reads them as
Python and TypeScript do, and parity cases hold all three to it. Separately,
Python reads `professional_claim` by its truth value, so a non-empty string
such as `"no"` or a non-empty list blocks there, while TypeScript and Rust
block only on `true`, a recognized true string or a non-zero number.

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
