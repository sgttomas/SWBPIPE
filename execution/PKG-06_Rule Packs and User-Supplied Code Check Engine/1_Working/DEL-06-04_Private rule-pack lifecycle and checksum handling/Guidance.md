# Guidance: DEL-06-04 Private rule-pack lifecycle and checksum handling

## Purpose

DEL-06-04 exists to keep rule-pack lifecycle and checksum behavior auditable without putting protected or private rule-pack content into the public repository. The future implementation should let users reference and validate their own design bases while preserving the code-neutral, private-data boundary.

## Principles

- Treat the checksum as evidence about a specific payload, not as proof of engineering correctness.
- Treat privacy and redistribution status as explicit metadata, not an inference from file location.
- Prefer source notes and provenance over embedded protected text.
- Keep lifecycle diagnostics visible: missing provenance, missing checksum, stale checksum, private-data export attempts, and suspected protected content are findings.
- Preserve the mechanics/rule/professional boundary: a user-rule check is not professional approval.
- Keep storage, encryption, access policy, and secret handling decisions out of this deliverable unless the human project authority records a later architecture decision.

## Considerations

| Topic | Guidance |
|---|---|
| JCS basis | For JSON rule-pack payloads, canonicalize with the project-selected JCS-compatible basis before hashing. Record `algorithm`, `canonicalization`, `payload_ref`, and `value`. |
| Payload boundary | A hash must state what was hashed. Do not hash local paths, timestamps, UI state, or other volatile fields as reproducibility evidence unless excluded or documented in a manifest. |
| Private marking | A rule pack with unknown redistribution rights should not be treated as public. The safer state is private or unknown until reviewed. |
| Protected suspected | Suspected protected standards or proprietary content should trigger quarantine/human review rather than public commit. |
| Audit hooks | Reports and exports should identify rule-pack name or ID, version, checksum, and source note. They should not expose private formulas or protected tables in public templates. |
| Unit metadata | Rule-pack values used for numeric evaluation need units or dimension metadata where applicable. Missing units should block or warn rather than default silently. |
| Deferred architecture | Private storage directories, encryption defaults, access grants, revocation, and credential handling belong to PKG-12 and persistence decisions. |

## Trade-offs

| Trade-off | Direction for this setup |
|---|---|
| Strong reproducibility vs. privacy | Reference hashes and source notes without publishing private payloads. |
| Detailed source trace vs. protected-content risk | Store source notices and provenance summaries; do not copy licensed text or tables into public artifacts. |
| Local-first convenience vs. governed export | Keep private data local by default; require explicit export/report decisions for any exposure. |
| Schema stability vs. implementation detail | Record metadata and lifecycle expectations now; leave enum finalization and module contracts to implementation/schema deliverables. |

## Examples

Safe public examples for this deliverable may show metadata shape only, using invented placeholders such as:

```yaml
rule_pack_ref:
  id: "invented-demo-only"
  version: "0.1.0"
  source_notice: "Invented demonstration values only"
  redistribution_status: "public_permissive"
  checksum:
    algorithm: "sha256"
    canonicalization: "JCS"
    payload_ref: "rule_pack"
    value: "TBD"
```

Private or protected formulas, allowables, code text, tables, and owner design-basis values must not be substituted into public examples.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| C-06-04-001 | Exact private storage location, encryption default, and access-control policy are required for implementation but out of scope for this deliverable. | DEL-06-04 context notes: access control details defer to PKG-12 | docs/PRD.md Section 18.3 lists optional encrypted storage | Datasheet Conditions; Specification R-06-04-011; Procedure Prerequisites | Defer to PKG-12 and persistence architecture; do not decide here. | TBD |
| C-06-04-002 | Exact schema enum set for redistribution/private status is not finalized by this deliverable. | docs/SPEC.md Section 6 uses `private`, `public_permissive`, `unknown` | docs/IP_AND_DATA_BOUNDARY.md includes `private_only` and `protected_suspected` for data records | Specification R-06-04-003; Datasheet Attributes | Record minimum lifecycle need and defer final enum ownership to DEL-06-01/schema work. | TBD |
