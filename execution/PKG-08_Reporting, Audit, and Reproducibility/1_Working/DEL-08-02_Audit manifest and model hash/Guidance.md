# Guidance: DEL-08-02 Audit manifest and model hash

## Purpose

The audit manifest is the bridge between a computed model run and a reviewable evidence package. It should let a reviewer determine which model payload, rule-pack reference, solver version, unit basis, warnings, and external assets were involved without treating the software output as professional approval.

## Principles

- Prefer deterministic identity over convenience. Equivalent JSON payloads should hash the same way after canonicalization; materially different inputs should produce different identifiers.
- Keep the hash boundary explicit. JSON payloads use the canonical JSON/JCS-compatible basis; binary and other non-JSON assets use manifest entries with their own hashes.
- Record references without leaking payloads. Rule-pack IDs, versions, checksums, source notices, and redistribution statuses are useful; protected formulas, private owner data, and copyrighted standard text do not belong in public artifacts.
- Treat missing data as evidence. Missing provenance, checksum, solver version, or manifest inputs should be visible warnings/findings.
- Preserve the professional boundary. A manifest can support replay and review; it cannot certify code compliance, seal a calculation, or replace competent engineering review.

## Considerations

| Topic | Guidance |
|---|---|
| Canonicalization | The canonicalization basis is selected at the architecture/decomposition level. This deliverable should not choose a physical project container or hashing library without a later implementation brief or human decision. |
| Units | The manifest should make the unit system and unit-bearing values reproducible enough that replay does not depend on implicit defaults. |
| Rule packs | Rule-pack checksum capture should reference the user/private artifact without copying protected rule text or proprietary values into public outputs. |
| Solver version | Version stamping should be granular enough to explain deterministic replay and regression comparison. Exact versioning format remains implementation work. |
| Binary assets | Non-JSON assets should be individually addressed in the manifest with digest and provenance fields, because the physical project container is still TBD. |
| Reports | Report generation should consume the manifest so report reproducibility and protected-content linting can be tested. |

## Trade-offs

| Choice | Benefit | Risk / open item |
|---|---|---|
| Hash only canonical JSON payloads | Stable digest under key ordering and formatting changes. | Requires a strict canonicalization contract and tests. |
| Include binary assets by manifest entry | Keeps non-JSON handling explicit and container-independent. | Requires clear asset inclusion/exclusion policy in future implementation. |
| Record private rule-pack references instead of payloads | Protects user/proprietary content while preserving reproducibility metadata. | Replay may require private artifacts that cannot be shared publicly. |
| Surface missing fields as warnings | Avoids silent defaults and supports professional review. | Review workflows must decide which missing fields are blockers. |

## Examples

No concrete engineering model, code-specific rule, material allowable, SIF/flexibility value, protected standard example, or private project payload is included in this setup artifact.

Illustrative manifest slots, without values, are:

- `model_hash`
- `canonicalization_basis`
- `solver_version`
- `unit_system`
- `rule_pack_refs[]`
- `input_payload_refs[]`
- `asset_manifest[]`
- `warnings[]`
- `professional_boundary_notice`

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in setup pass. | N/A | N/A | N/A | N/A | N/A |
