# Specification: DEL-06-04 Private rule-pack lifecycle and checksum handling

## Scope

This deliverable defines the setup contract for private rule-pack lifecycle and checksum handling in PKG-06. It covers metadata, lifecycle guardrails, checksum handling, audit/report hooks, and diagnostics needed to keep rule packs versioned, source-noted, checksum-addressed, and marked for redistribution/private status.

This setup does not implement registry modules, schemas, tests, encryption, private storage, access-control defaults, or private rule-pack content.

## Requirements

| Requirement ID | Requirement | Source basis |
|---|---|---|
| R-06-04-001 | A rule-pack lifecycle record shall include stable rule-pack identity, human-readable name, version, source/provenance notice, redistribution/private status, and checksum metadata. | docs/CONTRACT.md OPS-K-RULE-3; docs/SPEC.md Section 6; docs/PRD.md Section 12 |
| R-06-04-002 | Private rule packs shall be marked private by default unless a public-permissive redistribution basis is explicitly recorded. | docs/PRD.md Section 12.4 and 17.3; docs/IP_AND_DATA_BOUNDARY.md Section 6 |
| R-06-04-003 | Public project artifacts shall not contain private rule-pack content, protected standards text, protected tables, copied code formulas, proprietary vendor data, owner standards, or company design bases. | docs/CONTRACT.md OPS-K-IP-1/3; docs/IP_AND_DATA_BOUNDARY.md Section 3 |
| R-06-04-004 | JSON rule-pack payload hashes shall use canonical JSON with JCS-compatible canonicalization and shall record algorithm, canonicalization, payload reference, and checksum value. | docs/architecture/persistence_contract.md Hash Rules; SCA-001 architecture basis |
| R-06-04-005 | Non-JSON or binary rule-pack-related assets shall be represented by manifest hashes with explicit payload references; exact partitioning remains TBD. | docs/architecture/persistence_contract.md Hash Rules and Remaining TBDs |
| R-06-04-006 | A checksum shall identify what was hashed and shall not depend on environment-local paths, timestamps, UI session state, or other volatile fields unless their treatment is explicitly documented. | docs/architecture/persistence_contract.md Hash Rules |
| R-06-04-007 | The lifecycle shall emit diagnostics for missing source notice, missing redistribution status, stale or missing checksum, suspected protected content, attempted public export of private content, and rule-check-required data gaps. | docs/SPEC.md Sections 6-8; docs/architecture/code_neutral_analysis_boundary.md Boundary Rules |
| R-06-04-008 | Reports and exports may reference private rule-pack identity, version, checksum, and source note without exposing protected formulas or private values in public templates. | docs/IP_AND_DATA_BOUNDARY.md Section 7; docs/PRD.md Section 15 |
| R-06-04-009 | Rule-pack lifecycle status shall not state or imply professional certification, sealing, approval, endorsement, or automatic code compliance. | INIT.md Agent rule; docs/CONTRACT.md OPS-K-AUTH-1; docs/architecture/code_neutral_analysis_boundary.md Status Separation |
| R-06-04-010 | Numeric rule-pack inputs and values shall preserve unit metadata where applicable; missing required values or units shall be explicit findings. | docs/CONTRACT.md OPS-K-UNIT-1 and OPS-K-DATA-2/3 |
| R-06-04-011 | Storage location, encryption default, access-control policy, permission persistence, and private-library secret handling are deferred architecture decisions for PKG-12 and related persistence work. | DEL-06-04 Context Envelope Notes; docs/architecture/persistence_contract.md Remaining TBDs |
| R-06-04-012 | The implementation boundary shall preserve schema-first service/result envelope behavior and shall not allow adapters or plugins to bypass provenance, privacy, protected-content, checksum, or report controls. | docs/architecture/extension_domain_contracts.md No-Bypass Rules |

## Standards

No engineering code or standards-body requirement is implemented by this setup deliverable. Applicable project governance sources are:

- OPS-K-RULE-3: rule packs are versioned, checksummed, source-noted, and marked public/private.
- OPS-K-DATA-1/2/3: code-specific values are user-supplied/private, missing values are findings, and values carry provenance.
- OPS-K-PRIV-1/2: private data is not committed or transmitted publicly by default; telemetry is off by default and cannot include private engineering/code data.
- OPS-K-IP-1/2/3: protected content is excluded, provenance is required for public data, and suspected protected content is quarantined.
- OPS-K-UNIT-1: calculations, formulas, imported values, and exports are unit-aware and dimensionally checked.
- OPS-K-AGENT-1..4: no invented engineering values, gaps surfaced, sealed scope respected, outputs draft until accepted.

## Verification

| Verification ID | Check |
|---|---|
| V-06-04-001 | Four setup documents exist and retain required sections. |
| V-06-04-002 | `_SEMANTIC.md` exists and contains matrix sections A, B, C, F, D, K, G, X, T, and E. |
| V-06-04-003 | `_SEMANTIC_LENSING.md` exists and contains lens coverage for matrices A, B, C, F, D, X, and E. |
| V-06-04-004 | `Dependencies.csv` exists, is parseable, and contains all v3.1 required columns. |
| V-06-04-005 | Dependency enum fields use canonical values where checked. |
| V-06-04-006 | `_DEPENDENCIES.md` summarizes active anchors and execution edges consistently with `Dependencies.csv`. |
| V-06-04-007 | No private rule-pack contents, protected standards data, proprietary formulas, or certification/compliance claims are introduced. |
| V-06-04-008 | `_STATUS.md` is set to `SEMANTIC_READY` only after setup artifacts and local validations pass. |

## Documentation

Required setup outputs for this session are:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

Implementation outputs such as registry modules, schemas, tests, encryption/access policy defaults, private storage paths, and actual private rule-pack files are outside this setup session.
