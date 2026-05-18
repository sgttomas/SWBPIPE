# Guidance - DEL-02-05 Project Persistence and Round-Trip Serialization

## Purpose

DEL-02-05 exists to make project files auditable, deterministic, and reusable across the OpenPipeStress workflow. The persistence layer is the handoff surface between domain schemas, unit-aware modeling, rule-pack references, solver inputs, reports, automation, and future adapters. Source: `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-050 and OBJ-012.

## Principles

- Treat the project file as a versioned domain artifact, not as an implementation dump. Required persisted content must be described by schema and service contracts. Source: `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04.
- Preserve semantics, not formatting accidents. Round-trip success means required project content is preserved and canonical JSON/hash behavior is deterministic. Source: `docs/PRD.md` section 10 FR-001; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04.
- Keep units explicit. Every numerical value that can affect solving, rule checks, reports, or exports needs unit metadata or a traceable unit-system reference. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.
- Preserve provenance. Missing provenance is a finding or warning, not a reason to invent source information. Source: `docs/CONTRACT.md` OPS-K-DATA-2 and OPS-K-DATA-3; `docs/TYPES.md` section 5.
- Store rule-pack references and checksums without bundling protected rule content into public project examples. Source: `docs/SPEC.md` section 6; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7.
- Route persistence through application/service/storage boundaries that keep validation and diagnostics enforceable. Source: `docs/SPEC.md` section 1; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-03, AB-00-06, and AB-00-07.
- Separate software-computed state from professional approval. Persistence may record user-rule check states or human review records only with appropriate authority labels and hashes; it must not turn software output into code compliance. Source: `docs/TYPES.md` section 4; `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2.

## Considerations

- Use a schema-versioned project envelope with explicit document kind, project identity, schema version, unit-system reference, migration status, and payload sections. ASSUMPTION: this envelope shape is a suitable implementation pattern; the exact schema file layout remains TBD under SCA-001.
- Make canonicalization scope explicit before hashing. A hash over volatile, environment-specific, or session-only fields will undermine reproducibility. The exact hash payload subset is TBD; the binding basis is canonical JSON with JCS-compatible canonicalization for JSON payloads.
- Preserve unknowns as `TBD`, `UNKNOWN_SOURCE`, warnings, or diagnostics rather than filling defaults. This applies especially to units, provenance, rule-pack references, and migration status.
- Keep public fixtures small and invented. They can verify schema shape and round-trip behavior without exposing protected standards data, proprietary values, or commercial software examples.
- Represent private data boundaries in the contract. Project files may reference private libraries and rule packs, but the public project must not assume those files are redistributable.
- Design diagnostics for machine and user review. Open/save/migrate failures should identify affected objects, source, severity, remediation, and provenance rather than returning unstructured text only.
- Avoid overclaiming SCA-001. The architecture basis resolves JSON Schema 2020-12, canonical JSON/JCS-compatible hash basis, schema-first envelopes, and test-gate classes; it does not resolve exact dependency versions, physical container, migration framework, or binary packaging.
- Record unresolved persistence decisions in an ADR/open-decision mechanism before implementation depends on them. Source: `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-01, section 8.2, and OI-011.
- Treat local-first behavior as a boundary condition for persistence, not only as a publication policy. Create/open/save must not imply cloud storage, public commit, telemetry, or redistribution of private project/rule/library data by default; the exact split between persistence checks and PKG-12 privacy controls remains TBD. Source: `docs/CONTRACT.md` OPS-K-PRIV-1; `docs/PRD.md` sections 18.1-18.3.

### Vocabulary Normalization

Use these terms consistently until a human/architecture decision replaces them:

| Term | Use in DEL-02-05 | Avoid using it to mean |
|---|---|---|
| Project document | The logical persisted project content governed by schema and round-trip requirements. | A specific file extension or physical package format. |
| Project envelope | The versioned top-level contract containing metadata, payload slots, diagnostics/migration status, and reproducibility metadata. | A final schema file path or code-generated type. |
| Project file | A user-facing persistence artifact created/opened/saved by the product. | A commitment to a single JSON file. |
| Project package/container | The physical storage choice, such as single file versus packaged container. | A settled decision; this remains TBD under SCA-001/OI-011. |
| Input manifest | A reproducibility record that identifies hash inputs, referenced artifacts, and private/rule/library checksums where applicable. | A report approval or compliance claim. |

## Trade-offs

| Decision area | Conservative guidance | Why |
|---|---|---|
| Single JSON file vs packaged project container | Keep the physical package/container TBD until a human/architecture decision resolves it. | SCA-001 explicitly leaves the container unresolved. |
| Schema file layout and code generation | Record the exact schema layout/tooling decision in an ADR or open-decision log before implementation depends on it. | SCA-001 selects JSON Schema 2020-12 but leaves exact schema file layout and code-generation tooling TBD. |
| Embedded rule-pack data vs references | Prefer references with version/checksum/source notes for persistence; avoid embedding protected or private rule content in public examples. | Maintains code-neutral and private-data boundaries. |
| Human-readable JSON vs strict canonical form | Permit authoring/readability where useful, but tests should compare normalized semantic content and canonical JSON/hash output. | Determinism is required for reproducibility. |
| Full-file hash vs payload/manifests | Use canonical JSON for JSON payload hashes and manifest hashes for non-JSON/binary assets; exact split is TBD. | Matches SCA-001 without deciding binary package structure prematurely. |
| Migration now vs migration framework later | Require schema version and migration status now; leave migration framework/tooling TBD. | This records compatibility boundaries without inventing implementation details. |
| Local project operations vs public sharing | Make create/open/save local-first by default; treat public export, bug-report attachment, telemetry, and repository commit as separately reviewed actions. | PRD section 18 and OPS-K-PRIV-1 prohibit default transmission or public commitment of private project data. |

## Examples

TBD. No deliverable-specific project file example was available in the accessible sources. Any future public fixture should use invented or permissively licensed data, include provenance/redistribution fields where data records exist, and avoid protected standards/code content.

Future fixtures should also carry a durable review record with, at minimum, source name/location, license or redistribution basis, contributor certification, review status, and confirmation that the fixture does not include protected standards/code data, proprietary values, or professional approval claims. Source: `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5; `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3 and OPS-K-AUTH-1.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict affecting DEL-02-05 content was identified during Pass 1+2. Open items are tracked as TBD rather than conflicts. | N/A | N/A | N/A | N/A | N/A |
