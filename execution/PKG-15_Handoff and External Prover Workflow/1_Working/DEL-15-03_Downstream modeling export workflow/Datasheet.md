# Datasheet: DEL-15-03 Downstream modeling export workflow

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-15-03 | `_CONTEXT.md` |
| Name | Downstream modeling export workflow | `_CONTEXT.md` |
| Package ID | PKG-15 | `_CONTEXT.md` |
| Package name | Handoff and External Prover Workflow | `_CONTEXT.md` |
| Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md` |
| Discipline | ASSUMPTION: software backend / interop handoff workflow | `_CONTEXT.md`; `SOFTWARE_DECOMP.md#PKG-15` |
| Scope coverage | SOW-074 | `_CONTEXT.md`; `SOFTWARE_DECOMP.md#Structured Scope of Work` |
| Objective support | OBJ-017 | `_CONTEXT.md`; `SOFTWARE_DECOMP.md#Objectives` |
| Context envelope | L; WATCH | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |
| Anticipated artifacts | handoff exporter; export validation tests; invented target fixture | `_CONTEXT.md` |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Workflow purpose | Implement the generic handoff export path before target-specific commercial-tool parsers are in scope. | `_CONTEXT.md`; `SOFTWARE_DECOMP.md#PKG-15` |
| Required handoff package content | Model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags. | `_CONTEXT.md#Scope Detail`; `SOFTWARE_DECOMP.md#SOW-074` |
| Handoff object meaning | Schema-compliant export for downstream modeling and professional validation workflows. | `SOFTWARE_DECOMP.md#Vocabulary and semantic commitments` |
| Architecture baseline | Rust core/application services; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; producer-declared JSON hash byte-basis labels carried unchanged; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable. | `_CONTEXT.md#Architecture Basis Injection`; D-41 R5 T2A |
| Backend artifact boundary | Generic export workflow only; target-specific commercial parser behavior remains deferred. | `_CONTEXT.md#Context Envelope`; `SOFTWARE_DECOMP.md#Open issues` |
| Professional boundary | Handoff and external-prover metadata support must not create automatic professional approval states. | `SOFTWARE_DECOMP.md#OBJ-017`; `docs/CONTRACT.md#Invariant catalog` |
| Data boundary | Public artifacts must not bundle protected standards text, protected tables, proprietary commercial data, private rule packs, owner standards, or code-specific acceptance criteria. | `docs/IP_AND_DATA_BOUNDARY.md#Public repository must not contain`; `docs/CONTRACT.md#Invariant catalog` |

## Conditions

| Condition | Status |
|---|---|
| Target surfaces, canonical package container, and target-specific implementation | OI-015 names initial export and target surfaces; canonical package container, concrete mappings, target field coverage, and target-specific implementation remain gated by DEL-17-01 and DEL-17-02. |
| Exact dependency versions and package-specific implementation choices | TBD per `_CONTEXT.md#Architecture Basis Injection`. |
| Target-specific commercial stress output parsers | Deferred per `_CONTEXT.md#Context Envelope` and `SOFTWARE_DECOMP.md#SOW-074`. |
| Professional approval, certification, sealing, or code-compliance claims | Excluded by `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4. |
| Unit handling | Exports must remain unit-aware and validate against `schemas/handoff_package.schema.json` and `schemas/target_mapping.schema.json`; target-specific unit export strategy remains TBD under OI-015. |
| Private/protected data | Must be excluded unless intentionally supplied with documented rights through a governed path. |

## Construction

| Construct | Datasheet value |
|---|---|
| Primary input contracts | ACTIVE upstream rows in local `Dependencies.csv`, plus `schemas/handoff_package.schema.json` and `schemas/target_mapping.schema.json`. |
| Primary output | Generic handoff exporter at `core/handoff/exporter/workflow.py` capable of producing a schema-compliant handoff package and provider-neutral target mapping record. |
| Validation evidence | `tests/test_handoff_export_workflow.py` validates exported handoff/mapping inputs against schema contracts and checks `fixtures/invented_target_fixture.json` provenance. |
| Unsupported target behavior | Must surface unsupported-target flags and warnings; taxonomy and target mapping records are provided by upstream contract work. |
| Hash handling | Package/model checksum/reference fields preserve the producer-declared byte-basis label. DEL-15-03 does not recompute, relabel, or assert JCS conformance; package container and target-specific hash packaging remain TBD. |
| Fixture data | Invented target fixture only; no protected commercial-tool examples or proprietary data. |

## References

- `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and anticipated artifacts.
- `_REFERENCES.md` - source list for this deliverable.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-006 mirror/evidence surface for upstream workflow context.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.7 package, scope, objective, decision, and open-issue context.
- `docs/CONTRACT.md` - invariant catalog.
- `docs/SPEC.md` - schema-first, unit-aware, provenance, diagnostics, and handoff-related architecture context.
- `docs/TYPES.md` - canonical object and boundary vocabulary.
- `docs/IP_AND_DATA_BOUNDARY.md` - public/private and protected-content boundary.
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` - deliverable execution and review rules.
## D-41 R5 T2A supplied-checksum evidence (2026-07-12)

The downstream workflow carries supplied checksum records without recomputing them. Its governed invented records now use `deterministic_sorted_compact_json_payload_hash`; this is a narrow byte-basis label, not a JCS assertion by DEL-15-03.
