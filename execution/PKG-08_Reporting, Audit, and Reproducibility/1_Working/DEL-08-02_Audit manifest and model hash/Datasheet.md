# Datasheet: DEL-08-02 Audit manifest and model hash

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-08-02 |
| Deliverable name | Audit manifest and model hash |
| Package ID | PKG-08 |
| Package name | Reporting, Audit, and Reproducibility |
| Deliverable type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-039 |
| Supported objectives | OBJ-007; OBJ-012 |
| Setup status | Draft setup artifact; not implementation |

## Attributes

| Attribute | Draft setup value |
|---|---|
| Primary artifact family | Audit manifest and reproducibility metadata for OpenPipeStress runs. |
| Hash basis for JSON payloads | Canonical JSON with JCS-compatible canonicalization. Source: `docs/_Registers/ScopeLedger.csv` row SOW-039; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2. |
| Hash basis for non-JSON or binary assets | Manifest hashes recorded as separate asset entries. Source: `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2. |
| Required reproducibility markers | Model hash, solver version, rule-pack checksum, and input manifest. Source: `docs/_Registers/Deliverables.csv` row DEL-08-02. |
| Rule-pack boundary | Rule packs are user/private design-basis artifacts; public artifacts may reference ID, version, checksum, and source note without embedding protected formulas. Source: `docs/SPEC.md` sections 6 and 8; `docs/IP_AND_DATA_BOUNDARY.md` section 7. |
| Professional boundary | The manifest supports review and reproducibility; it does not certify, seal, approve, or authenticate engineering work. Source: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` section 3. |
| Physical project container | TBD. The architecture basis says the physical package/container remains implementation-level TBD. Source: `_CONTEXT.md` Architecture Basis Injection. |

## Conditions

- The setup artifact is constrained to document production only; no hashing code, schemas, or test files are implemented here.
- The audit manifest must avoid private/protected payload inclusion by default. It may identify private assets by controlled references and checksums, but public templates must not embed protected standards text, proprietary formulas, private rule-pack payloads, or owner data.
- Hashes must be deterministic for equivalent canonical input payloads. Solver version, rule-pack checksum, unit system, and manifest inputs are part of the reproducibility context.
- Missing source, provenance, version, or rule-pack checksum information is a finding to expose, not a default to fill silently.

## Construction

The future implementation surface is expected to include:

| Component | Purpose | Boundary |
|---|---|---|
| Input manifest | Lists canonical model inputs, rule-pack references, unit basis, selected solver settings, and external asset references needed to reproduce a run. | Records references and provenance; does not copy protected/private payloads into public artifacts. |
| Model hash | Stable digest over the canonical JSON representation of the model payload selected for hashing. | Uses JCS-compatible canonicalization; exact hashing API is implementation work, not this setup run. |
| Solver version stamp | Captures solver/application version and deterministic settings relevant to replay. | Does not imply validation or professional approval. |
| Rule-pack checksum capture | Records rule-pack identity, version, checksum, source notice, and redistribution status. | Rule-pack formulas and protected values remain user/private unless lawfully redistributable. |
| Binary/non-JSON asset manifest | Records digest, media/type, source/provenance, and inclusion policy for assets not covered by canonical JSON hashing. | Physical project container remains TBD. |

## References

- `_CONTEXT.md` for deliverable identity, architecture basis IDs, and setup constraints.
- `docs/_Registers/Deliverables.csv` row DEL-08-02 for artifact and objective mapping.
- `docs/_Registers/ScopeLedger.csv` row SOW-039 for hash-basis acceptance notes.
- `docs/SPEC.md` sections 6, 8, 9, and 11 for rule-pack, report, V&V, and acceptance constraints.
- `docs/IP_AND_DATA_BOUNDARY.md` sections 6 and 7 for private data and report-boundary rules.
- `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-UNIT, OPS-K-RULE-3, OPS-K-PRIV, OPS-K-AUTH, and OPS-K-AGENT invariants.
