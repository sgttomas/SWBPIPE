---
doc_id: OPS-SECURITY-THREAT-MODEL
doc_kind: security.threat_model
status: draft
created: 2026-05-02
deliverable_id: DEL-12-05
package_id: PKG-12
scope_items:
  - SOW-040
objectives:
  - OBJ-010
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: governed_by
    to: OPS-IP-DATA-BOUNDARY
  - rel: informed_by
    to: OPS-PROFESSIONAL-BOUNDARY
---

# Security Threat Model

This threat model records OpenPipeStress security and privacy risks for private
data, report sharing, plugins, imports, local-first storage boundaries,
telemetry, secrets, and supply chain exposure.

This document is a project security planning artifact. It is not legal advice,
professional engineering approval, certification, a security certification, a
compliance attestation, or a claim that any analysis result is fit for reliance
without competent human review.

## Scope

OpenPipeStress is local-first by default. Private project data, private rule
packs, private material and component libraries, owner standards, company
design bases, diagnostics, reports, and secrets remain user controlled unless a
user intentionally exports, shares, contributes, or attaches them with
documented rights.

SCA-003 selects a local SQLite-backed project store/index for the MVP. Canonical
JSON/JCS-compatible schema-governed payload bytes remain the domain and
interchange truth; SQLite tables and sidecar indexes are storage/projection
details behind application-service boundaries. Cloud operation is out of MVP
unless separately approved. Telemetry is disabled by default and must not
transmit private engineering or code data unless a user explicitly chooses a
payload.

SCA-004 adds deterministic export packages, target profiles, stable ID maps,
loss reports, CAEPIPE MBF and external-harness evidence, stress-neutral
CSV/JSON, conservative PCF, GLB/glTF review geometry, native open JSON export,
and export adapter SDK surfaces. These workflows remain local-first,
user-owned at external-tool boundaries, and claim-bounded: they do not bundle
commercial solvers, bypass licenses, reverse engineer protected formats, use
proprietary examples, leak private data by default, or create professional,
code-compliance, target-validation, or security-certification claims.

## Assets

| Asset | Sensitivity | Boundary expectation |
|---|---|---|
| Project store and model data | Private by default | Stored in the SCA-003 local SQLite-backed project store/index at user-controlled paths; not committed or exported publicly by default. |
| Rule packs | Private/user-owned by default | May include proprietary or licensed design-basis content; public examples must be invented or reviewed. |
| Material and component libraries | Private or provenance-gated | Public records require source, license, redistribution status, contributor certification, and review disposition. |
| Reports and result exports | Disclosure-sensitive | May expose project values, assumptions, warnings, source notes, and private rule/library identifiers. |
| Diagnostics, logs, and bug reports | Disclosure-sensitive | Must avoid automatic inclusion of private project files, paths, rule content, or calculation results. |
| Plugin and adapter manifests | Security-sensitive | Requests entrypoints, permissions, privacy posture, checksums, sandbox requirements, and no-bypass controls. |
| Import/export and local FEA handoff data | Disclosure-sensitive | Must preserve validation, units, provenance, diagnostics, public/private data boundaries, and report controls. |
| SCA-004 export packages | Disclosure- and integrity-sensitive | Include target profiles, manifests, stable ID maps, loss reports, native JSON, stress-neutral CSV/JSON, PCF, CAEPIPE MBF/evidence, and GLB/glTF review geometry. |
| Secrets and credentials | Secret | Must not appear in project files, reports, logs, examples, tests, or public artifacts. |
| Build and release artifacts | Supply-chain-sensitive | Require provenance, integrity, dependency review, and release-gate evidence. |

## Trust Boundaries

| Boundary | Risk | Required posture |
|---|---|---|
| Local filesystem | Private files, SQLite stores, sidecar indexes, or exports may be placed near public examples or repository paths. | Keep public examples separate from private paths; warn when export or contribution paths may disclose private data. |
| Public repository | Protected standards content, proprietary data, private projects, or secrets could be committed. | Apply provenance, protected-content, private-data, and secret-review gates before public contribution. |
| Report/export/share | Reports, exports, screenshots, shared models, or copied values may disclose private or protected data. | Require explicit user action, warnings, redaction controls where available, and public-template protected-content review. |
| Target export packages | Target profiles, stable ID maps, loss reports, native JSON, stress-neutral CSV/JSON, PCF, CAEPIPE MBF, or GLB/glTF artifacts can disclose private data or imply target support beyond admitted evidence. | Require source-basis refs, deterministic manifests and hashes, sidecar ID maps where target formats cannot carry canonical IDs, explicit loss reporting, public-safe fixtures, and claim-boundary notices. |
| External CAEPIPE or harness execution | Optional user-configured commercial-tool execution and CSV result parsing can expose paths, license state, private results, or be mistaken for validation. | Do not bundle a solver or executable; require user configuration and license/environment responsibility acknowledgement; parse only documented outputs; record evidence as non-authoritative regression or handoff evidence. |
| Bug reports and diagnostics | Logs or attachments may disclose file paths, project names, values, private rule-pack metadata, or private libraries. | Default to redacted diagnostics; require explicit attachment review for project files or private payloads. |
| Telemetry | Operational metrics can become sensitive when tied to engineering work. | Keep telemetry off by default; opt-in only; exclude private engineering/code data unless the user explicitly chooses the payload. |
| Plugin/import/export/FEA handoff | Extensions or adapters may bypass validation, provenance, diagnostics, sandboxing, report controls, or application-service persistence boundaries. | Enforce no-bypass constraints through schemas, manifests, diagnostics, review gates, and the prohibition on direct SQL or raw SQLite project-store access. |
| Rule evaluator | User rule logic may be private and evaluator execution must not become arbitrary code execution. | Use sandboxed, deterministic, unit-aware evaluation; carry checksums, private/public markings, and source notices. |
| Build and release supply chain | Dependencies, plugins, build tools, or release packages may be compromised or provenance-weak. | Require dependency/license/provenance review, reproducible build evidence where available, artifact integrity checks, and release gates. |

## Threat Inventory

| ID | Surface | Threat | Initial risk | Required controls or open decisions |
|---|---|---|---|---|
| STM-001 | Local storage | Private project, rule-pack, material, component, owner-standard, SQLite store, or sidecar index data is stored in a public/example path or committed accidentally. | High | Private-path conventions, public/example exclusion, contribution review, protected-content checks, private-data checks, and `IP_BOUNDARY_WARNING` diagnostics. |
| STM-002 | Report/export/share | A report, shared model, result export, screenshot, or copied table discloses private values or protected code-derived content. | High | Redaction workflow `TBD`, export warnings, public-template protected-content review, provenance summaries, and explicit user action. |
| STM-003 | Bug reports | A diagnostic bundle or issue attachment includes private project files, rule packs, libraries, paths, or calculation results. | High | Redacted default bundles `TBD`, attachment review, telemetry exclusion, and explicit user attachment. |
| STM-004 | Telemetry | Optional telemetry transmits private project data, rule-pack data, library data, or calculation results. | High | Disabled by default, opt-in only, no private engineering/code data, explicit payload selection if private data is ever included. |
| STM-005 | Plugins and adapters | A plugin, scripting API, importer, exporter, or FEA handoff bypasses validation, units, provenance, privacy controls, sandboxing, diagnostics, or report controls. | High | Deny-by-default plugin posture, manifest validation, permission model `TBD`, sandboxing where execution exists, and no-bypass enforcement. |
| STM-006 | Imports and contributions | Imported data lacks provenance or appears copied from protected standards, vendor catalogs, or proprietary sources. | High | Required provenance fields, redistribution status, contributor certification, review disposition, quarantine for protected-suspected content, and human/legal review. |
| STM-007 | Rule evaluator | Rule expressions execute arbitrary code or leak private rule contents through diagnostics, reports, or exports. | High | Sandboxed evaluator, deterministic expression model, no file/network/process access, private/public marking, checksums, and redacted reporting. |
| STM-008 | Public examples | Public examples or tests include realistic values copied from protected standards, vendors, commercial software, or private projects. | High | Invented or public-permissive examples only, source/provenance records, protected-content review, and rejection/quarantine where rights are unclear. |
| STM-009 | Secrets | Tokens, passwords, license credentials, private-library credentials, or signing keys appear in project files, reports, logs, examples, or tests. | High | Secret storage policy `TBD`, secret scanning `TBD`, report/export exclusion, and no real secrets in public artifacts. |
| STM-010 | Hash/provenance spoofing | Checksums, source notices, or review records do not bind to the actual payload being reviewed or exported. | Medium/High | Canonical JSON/JCS-compatible hash basis for JSON payloads, manifest hashes for non-JSON assets, and re-review after content changes. |
| STM-011 | Build and release supply chain | Dependencies, plugin packages, build scripts, or release artifacts are compromised, unlicensed, or provenance-weak. | Medium/High | Dependency and license review, release quality gates, artifact checksums/signing `TBD`, reproducible build evidence `TBD`, and protected-content gates. |
| STM-012 | Authority overclaim | Reports, diagnostics, UI text, examples, or security documentation imply code compliance, professional approval, or security certification. | Medium/High | Professional-boundary review, report notices, prohibited-claim scans, and human acceptance records that bind only to specific content hashes where used. |
| STM-013 | Export target profiles | A target profile or writer claims target/version coverage before the source basis, target version assumptions, and source evidence are admitted. | High | Source-basis refs, target profile/version basis, `TBD` where unresolved, no target-compatibility claims, and export review gates. |
| STM-014 | Stable ID maps and loss reports | Export packages omit canonical-to-target ID maps or hide omitted, approximated, delegated, unsupported, or unresolved behavior. | High | Stable ID maps or authoritative sidecars, mandatory loss-report categories, blocking diagnostics for missing maps/reports, and downstream implication notes. |
| STM-015 | CAEPIPE MBF and external harness | A CAEPIPE export or harness bundles a solver, bypasses a license, invokes a tool without user configuration, embeds proprietary examples, or treats CSV parsing as validation. | High | No bundled solver, no license bypass, no reverse engineering, invented/public-permissive fixtures only, user-owned executable configuration, and non-authoritative evidence labels. |
| STM-016 | Stress-neutral CSV/JSON | Stress-neutral rows omit unit/dimension metadata, expose private rule content, or imply pass/fail/code semantics. | High | Unit and dimension required per row, result-envelope and run refs, stable ID map, loss report, redaction review, and diagnostic-export-only semantics. |
| STM-017 | Conservative PCF | A PCF subset relies on hidden translator defaults, unresolved target profile assumptions, or target record behavior not explicitly supported. | Medium/High | Conservative subset profile, explicit fields, hidden-default blocking or loss reporting, sidecar identity, and no compatibility overclaim. |
| STM-018 | GLB/glTF review geometry | Review geometry leaks private metadata or is mistaken for solver geometry, analysis fidelity, or professional validation. | Medium/High | Visual-review-only artifact role, minimized metadata, unit/coordinate checks, stable ID sidecar or extras, loss report for omitted context, and no solver-fidelity claim. |
| STM-019 | Export adapter SDK | A community/additional target adapter requests filesystem, network, process, private-data, storage, rule-pack, solver, or report-control access without admission review. | High | Denied-by-default runtime grants, target registry admission state, validation checklist, source-basis review, privacy screening, export review, and no direct SQLite or application-service bypass. |

## No-Bypass Controls

Plugins, adapters, importers, exporters, FEA handoff paths, report generators,
and automation runners must preserve the same controls as first-party code:

- schema validation;
- unit checks and dimensional checks;
- provenance checks and redistribution status;
- privacy and private-data controls;
- protected-content screening;
- diagnostics and result-envelope warning classes;
- rule-pack sandboxing;
- report boundaries and professional-boundary notices;
- solver and mechanics/rule-check separation;
- human acceptance boundaries.
- deterministic export-package manifests, hashes, target profiles, stable ID
  maps, loss reports, and external-harness evidence boundaries;
- application-service persistence boundaries, with no direct SQL, raw SQLite
  handles, table-name dependencies, or direct project-store mutation.

A plugin manifest or adapter declaration grants nothing by itself. Runtime
permission model, loader mechanics, external API transport, and concrete
import/export format list remain `TBD`.

## Report And Export Rules

Public report templates and public examples must not include protected
standards text, copied formulas, standards tables, proprietary standards
content, commercial benchmark material, real private project data, real private
rule content, real private library values, or secrets.

Reports may reference a user rule-pack ID, version, checksum, source note,
warnings, assumptions, and limitations. Private report templates remain the
user's responsibility and must not be treated as public project content unless
they pass contribution review.

## SCA-004 Export Workflow Rules

| Workflow | Required threat posture |
|---|---|
| Common export packages, target profiles, and native open JSON | Export packages must identify source basis, target profile/version basis, source model/run/hash refs, deterministic manifests, stable ID maps, loss reports, diagnostics, privacy posture, and professional-boundary notices. Missing units, maps, loss reports, or source-basis refs are blocking findings, not inferred defaults. |
| CAEPIPE MBF and external harness evidence | CAEPIPE MBF profile and writer work is source-basis gated. External execution is optional and user-owned; OpenPipeStress must not bundle CAEPIPE, discover or invoke it without user configuration, bypass license responsibility, reverse engineer protected formats, ship proprietary examples, or treat parsed CSV evidence as professional validation. |
| Stress-neutral CSV/JSON | Stress-neutral exports are review, regression, and downstream-tooling artifacts only. Rows require units, dimensions, result/run/model refs, correlation status, and stable identity. They do not encode rule-pack formulas, pass/fail results, code-compliance semantics, or professional acceptance. |
| Conservative PCF subset | PCF export remains a conservative subset with explicit fields and loss reporting. Hidden translator defaults, unresolved target profile claims, unsupported components, and missing sidecar identity must block or warn through diagnostics instead of being silently accepted. |
| GLB/glTF review geometry | GLB/glTF is lightweight visual review geometry only. It may carry canonical identity through glTF extras and/or authoritative sidecars, but it must not be represented as solver geometry, local FEA, target validation, or professional review evidence. |
| Export adapter SDK and additional targets | SDK target admission is denied by default until source basis, target version, units/coordinates, stable identity, loss reporting, diagnostics, privacy screening, export review, and human-review boundaries are recorded. Runtime grants for filesystem, network, process, private data, storage, rule packs, solver, and report controls remain denied unless a later approved permission model grants them explicitly. |

## Telemetry Rules

Telemetry is disabled by default. If telemetry is introduced later, it must be
opt-in and privacy-preserving. It must not transmit private project files,
material data, component data, rule packs, owner standards, company design
bases, calculation results, report payloads, or protected/code-derived content
unless a user explicitly chooses that payload.

Telemetry implementation details, event schema, consent UI, retention policy,
and review gates remain `TBD`.

## Supply Chain Rules

Build, packaging, dependency, and release workflows must treat dependency
source, license status, package integrity, generated artifacts, and release
notes as security-relevant evidence. Release artifacts should carry hashes or
other integrity records once the release process is selected.

Dependency-version policy, signing process, reproducible build target,
publisher identity, CI provider, and coverage thresholds remain `TBD`.

## Open Decisions

| Decision | Current state |
|---|---|
| MVP private project storage substrate | SCA-003 local SQLite-backed project store/index; canonical JSON/JCS-compatible payload bytes remain the domain and interchange truth, and SQLite is not a public contract. |
| Operating-system roots, storage-root selection, and portable export/copy workflow | `TBD`; no OS-specific root, final application data directory, or portable project copy/export UX is selected. |
| Product schema and DB migration mechanics | `TBD`; migration-aware persistence remains required. |
| Encryption for private rule packs or libraries | `TBD`; no default encryption claim is made here. |
| Secret storage and signing-key process | `TBD`; real secrets are prohibited in public artifacts. |
| Plugin permission model and loader mechanics | `TBD`; no-bypass and deny-by-default posture applies. |
| Public API transport and supported import/export formats | `TBD`; schema-first envelopes remain the architecture basis, and SCA-004 export target profiles remain source-basis gated. |
| SCA-004 target profile/version source basis | `TBD` per target unless admitted by the owning DEL-17 source-basis/profile deliverable; no broad target-compatibility claim is made here. |
| External harness invocation and license-responsibility workflow | `TBD`; external harnesses remain optional, user-configured, user-owned evidence paths. |
| Export adapter SDK runtime grant model | `TBD`; runtime grants remain denied by default and cannot bypass application services or direct SQLite restrictions. |
| Redaction workflow and bug-report bundle format | `TBD`; default posture is redacted and explicit user attachment. |
| Telemetry event schema and consent workflow | `TBD`; telemetry remains off by default. |
| Cloud exception workflow | `TBD`; cloud storage, cloud sync, hosted database, and cloud service behavior are out of MVP unless separately approved. |
| Dependency signing/reproducible build/release integrity process | `TBD`; release evidence must be selected before release reliance. |

## Review Triggers

Review and update this threat model when any of these change:

- public API transport, plugin permission model, or import/export format list;
- operating-system storage roots, project-store migration mechanics, portable
  export/copy workflow, encryption, or key-management choices;
- rule expression grammar, parser, or sandbox implementation;
- telemetry introduction or event schema changes;
- report/export fields, templates, copy paths, screenshots, or shared-model
  package behavior;
- SCA-004 target profiles, stable ID maps, loss-report categories, CAEPIPE
  MBF or external-harness evidence, stress-neutral CSV/JSON, PCF, GLB/glTF,
  native JSON, or export adapter SDK behavior;
- private-library storage, secret handling, or credential workflows;
- build/package/release signing, dependency review, or CI policy;
- any protected-content, private-data, secret, or supply-chain incident.

## Verification Expectations

Threat-model changes should be checked for:

- traceability to `DEL-12-05`, `PKG-12`, `SOW-040`, and `OBJ-010`;
- coverage of required assets, trust boundaries, and threat rows;
- preservation of local-first, telemetry-off-by-default, and no-bypass
  constraints;
- coverage of SCA-004 export profiles, stable ID maps, loss reports,
  CAEPIPE/external harness evidence, stress-neutral CSV/JSON, conservative PCF,
  GLB/glTF review geometry, and export adapter SDK risks;
- explicit `TBD` markers for unresolved implementation choices;
- absence of protected standards content, proprietary engineering values, real
  private project data, real secrets, and professional/code-compliance or
  security-certification claims.
