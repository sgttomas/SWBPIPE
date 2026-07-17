---
doc_id: OPS-SECURITY-LOCAL-FIRST-STORAGE-POLICY
doc_kind: security.local_first_storage_policy
status: draft
created: 2026-05-02
deliverable_id: DEL-12-01
package_id: PKG-12
scope_items:
  - SOW-029
objectives:
  - OBJ-010
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: governed_by
    to: OPS-IP-DATA-BOUNDARY
  - rel: informed_by
    to: OPS-SECURITY-THREAT-MODEL
  - rel: aligned_with
    to: OPS-SECURITY-TELEMETRY-POLICY
---

# Local-First Storage Policy

This policy defines OpenPipeStress local-first storage and private path posture
for `DEL-12-01`. It is a privacy and product-boundary artifact, not a
filesystem layout, cloud-service design, encryption claim, secret storage
design, security certification, or compliance attestation (PRD §21.2).
Acceptance, professional judgment, and any certification, sealing, or
code-compliance determination remain with the responsible engineer and
project authority.

## Scope

OpenPipeStress is local-first by default. Private project models, private rule
packs, private material and component libraries, owner standards, company
design bases, credentials, secrets, diagnostics, reports, generated outputs,
and other user-owned engineering data remain user controlled unless a user
intentionally exports, shares, contributes, or attaches them with documented
rights.

The public repository is not a default durable storage location for private
project, rule-pack, material, component, report, diagnostic, owner-standard,
credential, secret, or proprietary data.

The MVP project store/index is local SQLite selected by SCA-003. It must be
free to use, bundled or otherwise available with the local desktop
installation, and usable offline. Cloud storage, cloud sync, hosted database,
daemon, required network, and cloud service behavior are out of MVP unless
separately approved by the human project authority. Telemetry remains disabled
by default and must not become a storage or synchronization path.

## Storage Resolution Rules

Storage choices must fail closed. Absent, unknown, unresolved, ambiguous, or
unsupported storage choices remain explicit warning, finding, or unsupported
states rather than silent defaults.

The physical project package/container decision is SCA-003: a local
SQLite-backed project store/index that stores canonical JSON/JCS-compatible
payload bytes and private indexed projections behind application-service
boundaries. Operating-system roots, application data directories, rollback
semantics, encryption, secret storage, key management, redaction workflow,
import/export formats, and cloud exception workflow remain unresolved unless
separately approved.

This policy does not create real private paths, write files, define a product
configuration schema, or select storage roots. It authorizes the local-only
storage profile, not a public SQL interface.

## Metadata-Only Storage Guard Helper

The `core.security.local_first_storage` helper provides a deterministic
metadata-only guard for storage planning records. Its public surface includes
`StorageRecord`, `StorageDiagnostic`, `StorageDecision`,
`StorageGuardResult`, `storage_record(...)`,
`classify_storage_record(...)`, and `guard_storage_records(...)`.

The helper classifies only explicit metadata. It does not read files, write
files, choose operating-system roots, store project payloads, store secrets,
open SQLite handles, expose direct SQL, transmit data, authorize cloud or
network storage, implement encryption, or make security-certification,
professional, approval, authentication, sealing, or code-compliance claims (PRD §21.2).

Public-repository and shared contexts block private project, rule-pack,
material, component, report, diagnostic, cache, and secret storage records by
default. Local-private retention of private metadata requires explicit user
intent. Cloud or network storage markers, direct-SQL markers, secret material,
payload material, and concrete path-like details are blocked or reduced to
symbolic, safe metadata in guard outputs.

## Symbolic Path Classes

Until implementation selects concrete roots, storage planning uses symbolic
path classes only.

| Symbolic class | Default sensitivity | Boundary rule |
|---|---|---|
| `PUBLIC_REPOSITORY_CONTENT` | Public-reviewed | May contain source code, schemas, blank templates, governance docs, and invented examples after protected-content review. |
| `PUBLIC_EXAMPLE_CONTENT` | Public-reviewed | May contain invented examples or public-permissive content with provenance and review disposition. |
| `USER_CHOSEN_PROJECT_PACKAGE` | Private by default | Holds user project model data in the SCA-003 local SQLite-backed project store/index at a user-controlled location. |
| `USER_PRIVATE_LIBRARY_ROOT` | Private by default | Holds user-controlled material, component, section, and owner/library data outside public repository paths. |
| `USER_PRIVATE_RULE_PACK_ROOT` | Private by default | Holds private rule packs, source notices, checksums, and redistribution status outside public repository paths. |
| `USER_REPORT_OUTPUT_ROOT` | Private by default | Holds reports, screenshots, review exports, and generated outputs selected by the user. |
| `USER_DIAGNOSTIC_BUNDLE_ROOT` | Private by default | Holds diagnostics/support bundles only after explicit user review of attachments and contents. |
| `USER_IMPORT_STAGING_ROOT` | Private or provenance-gated | Holds imported files pending validation, provenance capture, redistribution review, and protected-content checks. |
| `USER_EXPORT_STAGING_ROOT` | Private or review-gated | Holds export payloads pending redaction/export controls and explicit user action. |
| `LOCAL_CACHE_ROOT` | Private by default | Holds derived local cache data; cache contents must not become public artifacts by default. |
| `USER_SECRET_REFERENCE` | Secret reference only | Represents a pointer to future secret handling; actual secret storage is owned by `DEL-12-04` and remains `TBD`. |

These names are planning classes, not filesystem paths. They must not be
interpreted as operating-system roots, repository directories, cloud locations,
or default user paths. SQLite files, write-ahead logs, sidecar indexes, cache
files, and diagnostics inherit the same private-by-default posture when they
contain project-derived data.

## Repository Leakage Prohibitions

Public repository paths must not be used as default durable storage for:

- project geometry, loads, supports, load cases, solve results, SQLite project
  stores, sidecar indexes, or project package payloads;
- private rule packs, owner standards, company design bases, rule formulas,
  thresholds, inputs, or evaluation results;
- private material, component, section, or vendor-derived library values;
- generated reports, screenshots, review exports, support bundles, diagnostics,
  local SQLite stores, sidecar indexes, or local cache payloads;
- model hashes, manifest hashes, report hashes, rule-pack hashes, or private-
  library hashes that can fingerprint private work;
- tokens, passwords, API keys, license keys, certificates, private keys,
  signing keys, cookies, authorization headers, or other secrets;
- protected standards text, copied examples, copied formulas, standards tables,
  proprietary engineering values, commercial benchmark content, or protected/
  code-derived content.

Contribution workflows may accept public examples or data only after source,
provenance, license or redistribution status, contributor certification, and
review disposition are recorded.

## Persistence Baseline

Local-first storage must preserve the project persistence baseline:

- versioned persistence;
- schema-governed payloads;
- unit-aware data where quantities are present;
- provenance-preserving records;
- migration-aware status;
- deterministic round-trip serialization;
- canonical JSON/JCS-compatible hashes where JSON payloads are hashed;
- local SQLite storage as a payload/index substrate rather than domain truth;
- rebuildable, hash-neutral SQLite FTS5/BM25 retrieval sidecars;
- explicit findings for missing storage, privacy, provenance, redistribution,
  migration, or validation decisions.

Canonical JSON/JCS-compatible payload bytes remain the domain and interchange
truth. SQLite tables are storage/projection details, not public contracts.
Large files are referenced in place by path or URI plus hash, size,
classification, and verification metadata by default; they must not be copied
into a project package without an explicit export/review workflow.

## No-Bypass Surfaces

The private/public path classification and local-first rules apply equally to
first-party code, plugins, adapters, import/export paths, reports, telemetry,
CLI runners, diagnostics, tests, private-library mechanisms, application
services, and support bundle workflows.

Application services must use the same private/public path classification and
local-first rules.

No plugin manifest, adapter declaration, CLI option, imported file, report
template, rule pack, telemetry setting, diagnostic setting, or environment
default can bypass:

- path classification;
- application-service persistence boundaries;
- the prohibition on direct SQL, raw SQLite handles, table-name dependencies,
  or direct project-store mutation by plugins and adapters;
- unit checks and dimensional checks;
- provenance checks and redistribution status;
- protected-content screening;
- private-data controls;
- diagnostics and result-envelope warning classes;
- rule-pack sandboxing;
- report/export controls;
- human acceptance boundaries.

## Diagnostics And Reports

Storage diagnostics must identify unresolved or unsafe storage states without
printing private payload contents. A diagnostic may state that a path class is
unresolved, private, public-review-required, export-review-required, missing,
stale, moved, hash-mismatched, or blocked, but it must not reveal private
values or secrets.

Reports and exports remain user-controlled private outputs by default. `DEL-12-02`
owns redaction and export controls and should consume this policy's symbolic
path-class vocabulary.

## Open Decisions

| Decision | Current state |
|---|---|
| Physical project package/container | SCA-003 local SQLite-backed project store/index. |
| Operating-system roots and application data directories | `TBD`; no OS-specific root or real user path is selected. |
| DB migration and product schema migration framework | `TBD`; the concerns are separate and versioned, migration-aware persistence remains the baseline. |
| Encryption, secret storage, and key management | `TBD`; no encryption or secret-storage claim is made here. |
| Redaction workflow and export staging behavior | `TBD`; owned downstream by `DEL-12-02`. |
| Private-library registry and secret/private-library handling | `TBD`; owned downstream by `DEL-12-04`. |
| Import/export formats and adapter behavior | `TBD`; no adapter behavior is authorized here. |
| Cloud exception workflow | `TBD`; cloud storage, cloud sync, and cloud service behavior are out of MVP unless separately approved. |

## Verification Expectations

Local-first storage policy and future implementation changes should be checked
for:

- traceability to `DEL-12-01`, `PKG-12`, `SOW-029`, and `OBJ-010`;
- local-first, user-controlled defaults for private project, rule-pack,
  material/component library, report, diagnostic, generated output, credential,
  secret, owner-standard, and proprietary data;
- prohibition on public repository paths as default durable storage for private
  data;
- symbolic path classes instead of real paths, OS-specific roots, cloud
  locations, or user-specific examples;
- explicit warning or finding states for unresolved or unsafe storage choices;
- preservation of versioned, schema-governed, provenance-preserving,
  migration-aware, round-trip testable persistence;
- preservation of canonical JSON/JCS-compatible payload truth over SQLite
  projections or sidecars;
- separation of DB migrations from product schema migrations;
- no-bypass rules for plugins, adapters, import/export, reports, telemetry,
  CLI, diagnostics, tests, and application services;
- absence of protected standards content, proprietary engineering values, real
  private project data, real secrets, real filesystem paths, hosted database
  commitments, cloud storage commitments, network requirements, encryption
  claims, and professional/code-compliance or security-certification claims (PRD §21.2).
