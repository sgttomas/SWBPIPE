# Datasheet: DEL-14-04 Analysis-run comparison engine

**Generated:** 2026-05-03
**Document Role:** Descriptive
**Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-14-04 | `_CONTEXT.md` |
| Name | Analysis-run comparison engine | `_CONTEXT.md` |
| Package ID | PKG-14 | `_CONTEXT.md` |
| Package name | Model States, Analysis Runs, and Comparison | `_CONTEXT.md` |
| Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Scope items | SOW-073, SOW-072 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Objective support | OBJ-016 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Anticipated artifacts | run comparison engine; result delta tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |

## Attributes

| Attribute | Recorded Value |
|---|---|
| Primary function | Implement unit-normalized result deltas for mapped entities, diagnostics, and settings. |
| Comparison subject | Analysis runs and/or model states, as bounded by SOW-073; this deliverable focuses on analysis-run comparison. |
| Required comparison basis | Stable IDs, manual mappings where required, unit-normalized result deltas, and tolerance profiles. |
| Analysis-run basis | Exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes. |
| Entity/result coverage | Mapped nodes, elements, supports, terminals, stress/result locations, diagnostics, and settings. |
| Role boundary | Diagnostic/audit functionality only; not automatic external validation, professional approval, certification, sealing, authentication, or code-compliance determination. |
| Architecture basis | Rust core/application services; schema-first command/query/job result envelopes; JSON Schema 2020-12; JCS-compatible hash basis for JSON payloads when hashed; Cargo/Vitest/Playwright/validation/protected-content gates as applicable. |
| Open implementation decisions | Exact dependency versions, solver numerical library, public API transport, import/export format list, CI provider/coverage thresholds, physical project package/container, comparison tolerance defaults, and mapping workflows remain `TBD` unless separately approved. |

## Conditions

- The comparison engine must remain source-grounded in PKG-14 scope: immutable model states, analysis-run records, deterministic state/run comparison, mappings, tolerances, and comparison exports.
- `ASSUMPTION`: This deliverable consumes analysis-run records, mapping/tolerance contracts, result-export envelopes, and unit-system contracts as upstream evidence surfaces because `Dependencies.csv` lists those rows as ACTIVE DAG-002 mirror entries.
- The local dependency mirror records eleven ACTIVE rows and identifies the source of truth as approved DAG-002 coordination. These rows are evidence surface only and are not reclassified by this datasheet.
- Missing or ambiguous units on unit-bearing physical values are diagnostics, not silent defaults, per `docs/SPEC.md` unit-contract text.
- Result export envelopes must preserve units, diagnostics, provenance, hashes/status boundaries, and professional-boundary notices per `docs/SPEC.md` and `docs/TYPES.md`.
- Comparison tolerance defaults and mapping workflows are explicitly `TBD` in `execution/_Decomposition/SOFTWARE_DECOMP.md` open issue OI-014.

## Construction

| Construct | Description | Status |
|---|---|---|
| Input run references | References to two analysis runs and their exact model-state/run basis. | Required by SOW-072/SOW-073; concrete schema field names `TBD`. |
| Mapping source | Manual mappings where stable IDs do not directly align. | Required by SOW-073; owned contract dependency DEL-14-05 remains upstream. |
| Unit normalization | Same-dimension comparison of result values using accepted unit metadata and conversion contracts. | Required by SOW-073 and unit contract; accepted unit catalog/tolerance details `TBD`. |
| Delta categories | Result, diagnostic, and settings deltas for mapped entities/locations. | Required by DEL-14-04 description; exact enum/API shape `TBD`. |
| Tolerance profiles | Profile-driven thresholding for comparison outcomes. | Required by SOW-073; defaults/workflows `TBD` under OI-014. |
| Deterministic output | Stable ordering and reproducible results for the same inputs. | Required by deterministic comparison scope; concrete hash/output fixture basis `TBD`. |
| Tests | Result delta tests. | Anticipated artifact; acceptance fixture set `TBD`. |

## References

- `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and context budget.
- `_REFERENCES.md` - governing reference list and accessible public context.
- `Dependencies.csv` - approved DAG-002 mirror/evidence surface for upstream dependency rows.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.5 package, deliverable, scope, objective, and open-issue basis.
- `docs/CONTRACT.md` - invariant catalog, including no invention, unit-awareness, professional boundary, IP/privacy, and agent constraints.
- `docs/SPEC.md` - unit contract, analysis boundary, persistence/hash, result export, runner output, and validation mechanics.
- `docs/TYPES.md` - stable reference, diagnostic, checksum, result, result-export, and analysis-boundary vocabulary.
- `docs/IP_AND_DATA_BOUNDARY.md` - protected-content and private-data boundary.
