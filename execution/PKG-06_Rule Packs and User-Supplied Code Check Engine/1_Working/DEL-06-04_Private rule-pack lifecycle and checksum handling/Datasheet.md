# Datasheet: DEL-06-04 Private rule-pack lifecycle and checksum handling

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-06-04 |
| Deliverable name | Private rule-pack lifecycle and checksum handling |
| Package ID | PKG-06 |
| Package name | Rule Packs and User-Supplied Code Check Engine |
| Deliverable type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-042 |
| Objectives | OBJ-002, OBJ-005 |
| Current setup role | Document and setup artifact production only; no implementation files changed |

## Attributes

| Attribute | Setup value |
|---|---|
| Rule-pack boundary | Rule packs are user-owned or private design-basis artifacts; public project content is limited to schemas, mechanics, workflows, and invented examples. |
| Lifecycle subject | Version identity, source/provenance notes, public/private or redistribution status, checksum metadata, and audit/report references for rule packs. |
| Checksum basis | JSON rule-pack payloads use canonical JSON with JCS-compatible canonicalization before hashing. Non-JSON or binary assets require manifest-level hashes and an explicit payload reference. |
| Required checksum metadata | Algorithm, canonicalization where applicable, payload reference, checksum value, and statement of what was hashed. |
| Redistribution status | Must be explicit. Supported values are owned by the rule-pack schema deliverable; project sources require at least private, public-permissive, unknown, and suspected-protected handling. |
| Public repository posture | Private rule-pack content, licensed standards text, copied formulas, proprietary tables, owner requirements, and vendor data without redistribution rights are not committed publicly. |
| Report posture | Reports may reference rule-pack ID/name, version, checksum, and source note without exposing protected formulas or private values in public templates. |

## Conditions

| Condition | Constraint |
|---|---|
| Protected data | Do not include protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, dimensional standards, or proprietary vendor data. |
| Private data | Private rule packs remain user-controlled and are not transmitted, exported, or committed by default. |
| Professional responsibility | Rule-pack evaluation is software decision support using user data. It is not certification, sealing, approval, endorsement, or a code-compliance claim. |
| Unit awareness | Rule-pack values and evaluator inputs must remain unit-aware where numeric quantities are involved; missing required units or values are findings, not silent defaults. |
| Architecture basis | SCA-001 requires JSON Schema 2020-12 contracts, schema-first command/query/job result envelopes, and canonical JSON/JCS-compatible hash basis where JSON payloads are hashed. |
| Deferred decisions | Private storage location, encryption defaults, access-control policy defaults, permission grant persistence, and physical project container are deferred to PKG-12/PKG-02 architecture decisions unless separately approved. |

## Construction

This setup surface describes the future implementation contract without creating implementation files. A later implementation task for DEL-06-04 should produce a local lifecycle and checksum mechanism that:

- records rule-pack identity, version, source notice, redistribution/private status, checksum metadata, and review/quarantine disposition;
- binds checksum values to explicit payload references rather than environment-local paths or volatile session state;
- records diagnostics for missing provenance, missing redistribution status, stale hashes, suspected protected content, and attempts to expose private content publicly;
- exposes audit-manifest hooks for reports and reproducibility artifacts without embedding private rule-pack formulas or values in public templates;
- defers storage location, encryption, and access-control defaults to the security/privacy package.

## References

- INIT.md - bootstrap boundaries for open mechanics, private code data, and professional responsibility.
- docs/CONTRACT.md - OPS-K-RULE-3, OPS-K-DATA-1/2/3, OPS-K-PRIV, OPS-K-IP-1/2/3, OPS-K-UNIT-1, OPS-K-AGENT-1..4.
- docs/SPEC.md - rule-pack evaluator minimum sections and report/audit requirements.
- docs/IP_AND_DATA_BOUNDARY.md - public/private data and quarantine policy.
- docs/PRD.md - rule-pack requirements, user journey, reporting, and private data handling.
- docs/architecture/persistence_contract.md - canonical JSON/JCS hash basis and hash metadata.
- docs/architecture/code_neutral_analysis_boundary.md - rule-pack reference and professional-boundary separation.
- docs/architecture/extension_domain_contracts.md - no-bypass constraints for provenance, privacy, checksums, and protected-content controls.
- docs/_Decomposition/SOFTWARE_DECOMP.md - DEL-06-04 scope and SCA-001 architecture basis.
