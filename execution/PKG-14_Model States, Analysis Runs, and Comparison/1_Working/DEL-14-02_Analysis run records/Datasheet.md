# Datasheet: DEL-14-02 Analysis run records

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-14-02 | `_CONTEXT.md` |
| Name | Analysis run records | `_CONTEXT.md` |
| Package ID | PKG-14 | `_CONTEXT.md` |
| Package name | Model States, Analysis Runs, and Comparison | `_CONTEXT.md` |
| Deliverable type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-14 table |
| Scope item | SOW-072 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-072 |
| Objective support | OBJ-016 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective mapping |
| Anticipated artifacts | `schemas/analysis_run.schema.json`; run reproducibility tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-14-02 |
| Lifecycle role | Draft production-unit document; not implementation evidence | `_CONTEXT.md` PREPARATION notes; `docs/CONTRACT.md` OPS-K-AGENT-4 |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Required product behavior | Save analysis runs bound to exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes. | `docs/_Registers/ScopeLedger.csv` row SOW-072; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-072 |
| Record immutability intent | Results attach to immutable analysis runs rather than a mutable current model only. | `docs/_Registers/ScopeLedger.csv` row SOW-072; `_CONTEXT.md` envelope notes |
| Package boundary | PKG-14 implements immutable model-state records, analysis-run records, deterministic state/run comparison, mappings, tolerances, and comparison exports. | `execution/_Decomposition/SOFTWARE_DECOMP.md` package table |
| Package exclusion | PKG-14 does not ingest commercial prover outputs comprehensively or determine external validation. | `execution/_Decomposition/SOFTWARE_DECOMP.md` package table |
| Architecture basis | Rust core/application services; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed. | `_CONTEXT.md` Architecture Basis Injection |
| Unit boundary | Unit-bearing physical values crossing schema, service, solver, import/export, report, or rule-evaluation boundaries must carry explicit unit metadata unless explicitly dimensionless or equivalent. | `docs/SPEC.md` section 4 |
| Hash boundary | JSON payload hashes use the accepted JCS-compatible canonical JSON basis where the payload is JSON; non-JSON and binary partitioning remains TBD. | `docs/SPEC.md` section 4.4 |
| Result-envelope relationship | Result export envelopes must identify result set, model/run basis, solver version, unit-system reference, load-case or combination basis, diagnostics, provenance, reproducibility hashes or audit-manifest reference, analysis statuses, rule-pack references where present, and professional-boundary notice. | `docs/SPEC.md` result export section |
| Professional boundary | Software must not emit automatic human approval, certification, sealing, authentication, or code-compliance labels. | `docs/SPEC.md` analysis boundary section; `docs/CONTRACT.md` OPS-K-AUTH-1 |
| Protected/private data boundary | Public artifacts must not copy private formulas, protected standards text, protected tables, proprietary values, or private rule-pack payloads. | `docs/SPEC.md` result export section; `docs/IP_AND_DATA_BOUNDARY.md` |

## Conditions

| Condition | Status | Source |
|---|---|---|
| Exact analysis-run schema fields | TBD pending implementation design. Only the required binding categories are source-supported here. | `_CONTEXT.md`; SOW-072 |
| Analysis run identifier format | TBD. No deliverable-local source defines the field name, format, or namespace. | `_CONTEXT.md`; `docs/TYPES.md` general identifier guidance only |
| Model-state reference semantics | Must bind to exact model states; detailed state-record schema is upstream and outside this folder. | SOW-072; `Dependencies.csv` DAG-002-E0783 |
| Solver version and settings representation | Required as part of the saved run record; exact object shape is TBD. | SOW-072 |
| Unit-system representation | Required as part of the saved run record; missing unit metadata for unit-bearing physical values must be diagnostic rather than silently supplied. | SOW-072; `docs/SPEC.md` section 4 |
| Load-case or combination basis | Required as part of the saved run record; exact references and cardinality are TBD. | SOW-072; `docs/SPEC.md` result export section |
| Diagnostics and analysis statuses | Required as part of the saved run record; must preserve the professional-boundary status model. | SOW-072; `docs/SPEC.md` analysis boundary section |
| Rule-pack and library references | Required as references/checksum/provenance surfaces; private payloads and protected content are not public artifact content. | SOW-072; `docs/SPEC.md` rule-pack/result export sections |
| Result hashes | Required; payload scope and canonicalization metadata must be explicit. | SOW-072; `docs/SPEC.md` section 4.4 |
| Reproducibility tests | Required artifact class; exact test harness, fixtures, and assertions are TBD. | `_CONTEXT.md` anticipated artifacts |

## Construction

This deliverable is a data-model change. The conservative construction surface is:

1. Define `schemas/analysis_run.schema.json` as the canonical analysis-run record contract.
2. Include fields or referenced sub-objects sufficient to bind each analysis run to the source-supported categories in SOW-072: model state, solver version, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.
3. Treat result attachment as immutable run-record evidence, not as mutable "current model" state.
4. Preserve unit metadata and dimensional diagnostics across run records wherever unit-bearing values are included or referenced.
5. Record reproducibility hash metadata using the accepted JSON/JCS-compatible basis for JSON payloads; mark non-JSON or binary partitioning as TBD until explicitly designed.
6. Preserve professional-boundary and protected/private-data controls in schema shape, diagnostics, tests, and examples.

Implementation evidence, committed product code, and passing tests are not present in this folder at setup time.

## References

| Reference | Use in this datasheet |
|---|---|
| `_CONTEXT.md` | Deliverable identity, scope, artifact expectations, architecture-basis injection |
| `_REFERENCES.md` | Reference inventory and authority boundary |
| `_DEPENDENCIES.md`; `Dependencies.csv` | Approved DAG-002 local mirror/evidence surface |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-072, OBJ-016, PKG-14, DEL-14-02 decomposition basis |
| `docs/_Registers/Deliverables.csv` | Deliverable row and anticipated artifacts |
| `docs/_Registers/ScopeLedger.csv` | Scope item SOW-072 wording |
| `docs/CONTRACT.md` | Agent, data, unit, report, privacy, and professional-boundary invariants |
| `docs/SPEC.md` | Unit, persistence/hash, analysis-boundary, result-export, runner/result-envelope guidance |
| `docs/TYPES.md` | Canonical model/schema vocabulary and boundary notes |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private and protected-content constraints |
