---
doc_id: OPS-SECURITY-TELEMETRY-POLICY
doc_kind: security.telemetry_policy
status: draft
created: 2026-05-02
deliverable_id: DEL-12-03
package_id: PKG-12
scope_items:
  - SOW-037
objectives:
  - OBJ-010
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: governed_by
    to: OPS-IP-DATA-BOUNDARY
  - rel: informed_by
    to: OPS-SECURITY-THREAT-MODEL
---

# Telemetry Policy

This policy defines OpenPipeStress telemetry posture for `DEL-12-03`. It is a
privacy and product-boundary artifact, not a telemetry implementation, vendor
selection, cloud-service design, professional engineering approval, security
certification, compliance attestation, or claim that analysis results are fit
for reliance without competent human review.

## Scope

OpenPipeStress is local-first by default. The MVP may remain a telemetry no-op:
no telemetry transport, endpoint, vendor, background upload job, queue, or
telemetry persistence is required or authorized by this policy.

Telemetry, if it is ever implemented, must be opt-in only, privacy-preserving,
and bounded by a human-approved event allowlist. A user installing the
software, opening a project, accepting general terms, importing data, running a
solve, exporting a report, or loading a plugin is not telemetry consent.

## Default-Off Semantics

Telemetry is disabled by default.

Absent, unset, empty, unknown, unsupported, or malformed telemetry
configuration resolves to disabled. Fail-closed behavior is required: the
application must not infer consent from ambiguous configuration, legacy
configuration, environment defaults, plugin metadata, import/export settings,
or runtime errors.

The exact product configuration schema, storage location, consent UI, endpoint,
vendor, retention policy, transport, and concrete event schema remain `TBD`
pending separate human approval.

## Initialization Rules

Telemetry must not initialize network transport, background upload jobs, upload
queues, local telemetry persistence, endpoints, vendors, or external service
clients unless all of these are true:

- the user has explicitly opted in through an approved product surface;
- a human-approved event allowlist exists for the exact product version;
- payload filtering is active before any event is constructed;
- protected-content, private-data, secret, and professional-boundary checks are
  available for telemetry payload review;
- plugin, adapter, import/export, report, and CLI paths cannot bypass the same
  opt-in and payload-filtering checks.

If any condition is missing, telemetry remains disabled and any attempted
event is dropped locally without network behavior.

## Event Allowlist Rules

No event is collectable until it appears in a human-approved allowlist. An
allowlist entry must define the event name, permitted fields, field
sensitivity, review date, approving human authority, and reason collection is
needed.

Unknown events, unapproved event versions, fields not listed on the allowlist,
or fields with unclear sensitivity are rejected before payload construction.
The allowlist may approve only low-sensitivity operational metadata. It cannot
approve private engineering or code data unless the user explicitly selects a
specific payload for a specific support or export action under a separate
approved workflow.

## Forbidden Payload Fields

Telemetry payloads must not include:

| Forbidden field class | Examples |
|---|---|
| Private project models | Project geometry, node coordinates, element topology, loads, load cases, boundary conditions, support settings, solve results, or project package content. |
| Code-specific rule data | Owner standards, company design bases, rule formulas, rule thresholds, rule inputs, rule evaluation results, or private rule logic. |
| Private rule packs | Rule-pack payloads, private rule-pack metadata beyond a user-approved public identifier, private checksums, or private source notes. |
| Private material or component libraries | Material values, component catalog rows, vendor-derived records, provenance notes, private source references, or library file contents. |
| Generated reports and exports | Report bodies, report templates, report attachments, screenshots, exported models, export payloads, or shared-model packages. |
| Model hashes | Project hashes, manifest hashes, report hashes, rule-pack hashes, private-library hashes, or other identifiers that can fingerprint private work. |
| Local file paths | User names, project names embedded in paths, directory names, file names, mount points, repository paths, or temporary paths. |
| Secrets and credentials | Tokens, passwords, API keys, license keys, certificates, private keys, signing keys, cookies, or authorization headers. |
| Protected standards content | Standards text, tables, figures, copied examples, copied formulas, proprietary engineering values, commercial benchmark content, or protected/code-derived content. |
| Professional or code-compliance claims | Claims of certification, sealing, professional approval, code compliance, safety approval, or fitness for reliance. |

## Diagnostics And Reports

Telemetry diagnostics, if added later, must use the project diagnostic envelope
where available and must not include private payload content. Diagnostics may
state that telemetry is disabled, blocked, or rejected, but must not print the
rejected private value.

Reports, exports, screenshots, public examples, and bug-report bundles must
not include telemetry payloads by default. Any future support bundle workflow
must require explicit attachment review by the user and must preserve the
protected-content and private-data boundary.

## No-Bypass Surfaces

The telemetry opt-in and payload-filtering rules apply equally to first-party
code, plugins, adapters, import/export paths, report generators, private
library mechanisms, CLI runners, diagnostics, tests, and future application
services.

No plugin manifest, adapter declaration, CLI option, imported file, report
template, or rule pack can enable telemetry, add telemetry fields, define an
endpoint, or bypass the allowlist by itself.

## Open Decisions

| Decision | Current state |
|---|---|
| Product configuration schema | `TBD`; absent, unset, unknown, unsupported, or malformed configuration resolves to disabled. |
| Consent UI or CLI surface | `TBD`; consent must be explicit and distinct from install, terms acceptance, project open, import, solve, export, or plugin load. |
| Endpoint, vendor, transport, and retention policy | `TBD`; no endpoint, vendor, transport, or retention behavior is authorized by this policy. |
| Concrete event schema and event allowlist | `TBD`; no event is collectable without human-approved allowlist evidence. |
| Support bundle or explicit user-selected payload workflow | `TBD`; no private payload is eligible for automatic telemetry. |

## Verification Expectations

Telemetry policy and future implementation changes should be checked for:

- traceability to `DEL-12-03`, `PKG-12`, `SOW-037`, and `OBJ-010`;
- default-off behavior for absent, unset, empty, unknown, unsupported, and
  malformed configuration;
- no initialization of telemetry transport, endpoints, vendors, upload jobs,
  upload queues, persistence, or external service clients without explicit
  opt-in and a human-approved event allowlist;
- rejection of forbidden private-data, protected-content, secret, path, hash,
  report, and professional-claim fields before payload construction;
- preservation of local-first behavior and no-bypass rules for plugins,
  adapters, import/export, reports, CLI, diagnostics, and application services;
- explicit `TBD` markers for configuration schema, consent surface, endpoint,
  vendor, retention policy, transport, and event schema;
- absence of protected standards content, proprietary engineering values, real
  private project data, real secrets, and professional/code-compliance or
  security-certification claims.
