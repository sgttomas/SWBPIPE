# Guidance: DEL-15-03 Downstream modeling export workflow

## Purpose

This deliverable exists to provide the generic handoff export workflow that turns OpenPipeStress model and analysis context into schema-compliant downstream modeling handoff packages. Its purpose is traceable transfer, not external-tool validation or professional approval.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Generic before target-specific | Keep the workflow around `schemas/handoff_package.schema.json`, `schemas/target_mapping.schema.json`, and the generic exporter before adding target-specific parsers or profiles. | `_CONTEXT.md#Context Envelope`; `SOFTWARE_DECOMP.md#SOW-074`; OI-015 |
| Explicit missingness | Missing mapping support, unresolved assumptions, warnings, and unsupported target behavior should be visible in the export output. | SOW-074; `docs/CONTRACT.md` OPS-K-DATA-2 |
| Unit-aware export | Unit metadata must cross the export boundary unless a field is explicitly dimensionless or otherwise classified. | `docs/SPEC.md#Unit system and dimensional analysis`; OPS-K-UNIT-1 |
| Provenance and hash discipline | Exported packages should preserve references, provenance, rule/library refs, diagnostics, and hashes without treating them as professional acceptance. | `docs/TYPES.md` Assumption, TraceabilityLink, Diagnostic, Checksum, ResultExportEnvelope |
| Professional boundary | The workflow may support professional review; it must not claim approval, certification, sealing, authentication, or code compliance. | OBJ-017; DEC-015; OPS-K-AUTH-1 |
| Protected-content boundary | Fixtures must be invented or cleared; do not import protected standards examples or commercial tool files. | `docs/IP_AND_DATA_BOUNDARY.md`; OPS-K-IP-1 through OPS-K-IP-3 |

## Considerations

- The local `Dependencies.csv` records upstream architecture-basis and interop/security/model-contract inputs as ACTIVE DAG-002 mirror evidence. Treat those rows as coordination evidence, not as permission to copy sibling deliverable prose or implementation details into this folder.
- Upstream contracts named by the mirror include the canonical handoff package schema, target mapping and unsupported-behavior contract, import/export adapter framework, local FEA handoff data contract, private-data redaction/export controls, physical-to-analytical transformation contract, and comparison export contracts.
- The workflow should report unsupported-target behavior rather than silently approximating it. Provider-neutral unsupported behavior and target mapping records are now supplied by `schemas/target_mapping.schema.json`; target-specific extensions remain gated by DEL-17-01 and DEL-17-02.
- The current fixture is `fixtures/invented_target_fixture.json`; it is invented public metadata used by `tests/test_handoff_export_workflow.py` to exercise schema shape, warnings, assumptions, provenance, and unsupported-target flags without resembling commercial-tool examples.
- The generic exporter entry point is `core/handoff/exporter/workflow.py`. Package container structure, concrete mappings, target field coverage, and target-specific implementation remain gated.

## Trade-offs

| Trade-off | Conservative direction |
|---|---|
| Generic exporter vs. target-specific parser | Prefer a generic exporter with explicit unsupported-target flags; defer target-specific parsing behavior. |
| Strict schema validation vs. permissive export | Prefer schema validation and explicit findings over silent coercion. |
| Helpful external metadata vs. professional status | Preserve references, notes, hashes, and diagnostics, but avoid approval/certification/status claims. |
| Realistic fixture vs. protected/commercial data risk | Prefer invented fixture data with clear provenance. |

## Examples

The current example fixture is `fixtures/invented_target_fixture.json`. It is invented public metadata only and is checked by `tests/test_handoff_export_workflow.py` alongside schema validation for `schemas/handoff_package.schema.json` and `schemas/target_mapping.schema.json`.

Do not derive future example values from protected standards, commercial software files, proprietary catalogs, owner standards, or private project data.

## Open Questions and TBDs

| ID | Item | Source |
|---|---|---|
| DEL-15-03-TBD-001 | Canonical package container, concrete mappings, target field coverage, and target-specific implementation. | `SOFTWARE_DECOMP.md` OI-015; DEL-17-01; DEL-17-02 |
| DEL-15-03-TBD-002 | Target-specific parser/profile behavior and commercial-tool result ingestion remain out of this deliverable. | `_CONTEXT.md#Context Envelope`; OI-015 |
| DEL-15-03-TBD-003 | Package-specific dependency versions and broader architecture implementation choices remain governed by architecture-basis TBDs. | `_CONTEXT.md#Architecture Basis Injection` |
| DEL-15-03-TBD-004 | Future fixtures for target-specific profiles require protected-content and provenance review. | `docs/IP_AND_DATA_BOUNDARY.md`; DEL-17-01; DEL-17-02 |

## Conflict Table (for human ruling)

No source conflicts were identified during the setup pass. TBDs above are missing-detail boundaries, not resolved conflicts.
