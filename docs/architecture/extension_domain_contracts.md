---
doc_id: OPS-EXTENSION-DOMAIN-CONTRACTS
doc_kind: architecture.domain_contract
status: draft
created: 2026-04-30
deliverable_id: DEL-02-04
package_id: PKG-02
scope_item: SOW-038
---

# Extension Domain Contracts

This document defines the domain-level contract for OpenPipeStress plugins and extensions. It complements `docs/architecture/plugin_boundary.md`, which describes the public API/plugin boundary. This document focuses on what extensions are allowed to touch in the domain model and which controls they can never bypass.

The companion manifest schema is `schemas/plugin_manifest.schema.yaml`, written as strict JSON syntax so it can be checked without a YAML dependency.

## Domain Posture

Extensions are denied by default. A plugin manifest may request an entrypoint and permissions, but the manifest itself grants nothing. Any later runtime grant must preserve the same domain controls required of first-party code.

Plugins and adapters are translators at the boundary. They may help import, export, validate, report, or connect rule-pack metadata, but they do not own the domain core, solver internals, provenance policy, privacy policy, professional-boundary language, or protected-content decisions.

Extensions do not own project storage. Any persistence-facing extension must call host application-service commands, queries, or jobs and receive governed envelopes back. Direct SQL, raw SQLite handles, database connections or cursors, table names, repository/store mutation handles, and direct writes to persistence files are outside the extension domain contract.

## Extension Families

The domain contract recognizes these extension point families as schema categories, not as runtime loader commitments:

- `model_import_adapter` maps external user data into canonical model structures through schema, unit, provenance, privacy, and protected-content checks.
- `model_export_adapter` and `result_export_adapter` produce governed payloads through result envelopes, privacy filters, provenance summaries, checksums, and diagnostics.
- `diagnostic_provider` may emit structured diagnostics but cannot suppress core diagnostics.
- `validation_hook` may add checks but cannot downgrade solve-blocking, provenance, privacy, rule, or IP-boundary findings.
- `rule_pack_provider` may expose rule-pack metadata or user-owned rule inputs through the sandboxed rule boundary only.
- `report_output_extension` may format outputs but cannot embed protected standards content or claim professional approval.

Exact transport, loader mechanics, signing, packaging, persistence of grants, and supported external formats remain `TBD`.

## Required Manifest Declarations

Every plugin manifest must declare:

- plugin identity, version, author, description, and review status;
- compatibility with this domain contract and `api/api_boundary_contract.yaml`;
- JSON Schema 2020-12 contract basis, host API status, and unresolved transport
  status where applicable;
- one or more entrypoints, their operation mode, and the domain surface they
  touch, such as canonical model, project persistence, units, analysis
  boundary, rule-pack metadata, results, reports, or diagnostics;
- requested permissions, with `denied_by_default` fixed to `true`;
- provenance, redistribution status, contributor certification, and review disposition;
- privacy posture, including local-first behavior and telemetry off by default;
- manifest and payload checksums using the canonical JSON/JCS-compatible basis when JSON payloads are hashed;
- sandbox requirements, explicit capability declaration, and default denial of
  arbitrary code execution, filesystem access, network access, and process
  spawning;
- no-bypass constraints for units, provenance, privacy, rule sandboxing,
  analysis-boundary state, persistence, schema validation, diagnostics,
  checksums, protected-content checks, report controls, solver boundary
  controls, and human-acceptance controls.
- an explicit persistence access posture that preserves application-service
  commands, queries, and jobs as the only persistence boundary, with direct
  SQL/raw SQLite/table-name/direct-store mutation access denied.

## Denied-By-Default Behavior

Absence of a permission means denial. A permission request with no recorded grant means denial. A plugin that cannot express its provenance, privacy posture, sandbox posture, or no-bypass constraints is not eligible to run through a governed extension boundary.

Private project models, private rule packs, private material/component libraries, owner design bases, licensed code data, and calculation results are not transmitted or exported by default. Network access and filesystem access require explicit permission and remain subject to privacy, provenance, checksum, and protected-content controls.

Privacy declarations must preserve local-first behavior, telemetry off by
default, export permission checks, and redaction support. A plugin may request
private data access, but the request does not grant access and must remain
visible to the host boundary and later human review.

## No-Bypass Rules

Plugins and adapters must not bypass:

- unit and dimensional validation for numerical or dimensional data;
- explicit findings for missing solve-required or rule-check-required data;
- provenance, source, redistribution-status, contributor-certification, and review-status checks;
- private/public data classification, telemetry-off-by-default behavior, and export permission checks;
- protected-content screening and quarantine routing;
- sandboxed, deterministic, unit-aware rule-pack evaluation;
- analysis-boundary states that distinguish mechanics solved, user-rule
  checked, failed, incomplete, and human-review-required states;
- persistence, round-trip, migration, hash, and private-data markers;
- application-service persistence boundaries, with no direct SQL, raw SQLite
  handles, table-name coupling, or direct store mutation;
- command/query/job/result-envelope boundaries;
- diagnostics, warnings, limitations, hashes, and reproducibility metadata;
- report boundary language that prevents certification, sealing, approval, endorsement, or automatic code-compliance claims (PRD §21.2).

If an extension encounters suspected protected standards text, copied protected tables, protected examples, copied code formulas, proprietary vendor data without rights, private rule values, or private project data in a public contribution path, it must block or quarantine the payload and emit an appropriate diagnostic for human review.

## Professional Boundary

Extension outputs are software artifacts. They may report mechanics results, user-rule outcomes, warnings, assumptions, provenance, privacy posture, and checksums. They must not state or imply that OpenPipeStress, a plugin, an adapter, or an agent has certified, sealed, approved, authenticated, endorsed, or declared a calculation code-compliant for professional reliance (PRD §21.2).

Human review remains required for professional use. Any future human acceptance record must be external to automatic plugin output and bound to the relevant model, result, rule-pack, report, and manifest hashes.

Plugin manifests must declare that software-generated human acceptance records
are not allowed. Plugins may reference external human acceptance evidence only
through hash-bound records governed by the analysis-boundary and persistence
contracts.

## Remaining TBDs

- Runtime plugin loader and isolation technology.
- Permission grant storage, consent UX, revocation, and signing requirements.
- Public API transport and concrete external import/export formats.
- Rule expression grammar and sandbox implementation details.
- Canonicalization edge cases for non-JSON payloads.
- CI gates for plugin submission, protected-content screening, and security/privacy review.
