# Datasheet: DEL-14-03 Model-state comparison engine

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-14-03 |
| Name | Model-state comparison engine |
| Package ID | PKG-14 |
| Package Name | Model States, Analysis Runs, and Comparison |
| Type | BACKEND_FEATURE_SLICE |
| Description | Implement deterministic state diffs using stable IDs and explicit mapping records. |
| Anticipated artifacts | state comparison engine; state diff tests |
| Scope coverage | SOW-073; SOW-071 |
| Objective support | OBJ-016 |
| Context envelope | L |

Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section "PKG-14 - Model States, Analysis Runs, and Comparison"; `docs/_Registers/Deliverables.csv` row `DEL-14-03`.

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary function | Compute deterministic added, removed, changed, and unchanged model-entity diffs between model states. | `_CONTEXT.md` Context Envelope; `execution/_Decomposition/SOFTWARE_DECOMP.md` row `DEL-14-03` |
| Identity basis | Stable IDs are the primary matching surface. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/TYPES.md` `Reference` definition |
| Mapping basis | Explicit mapping records are required where direct stable-ID comparison is insufficient. Mapping workflow details remain `TBD`. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `execution/_Decomposition/SOFTWARE_DECOMP.md` open issue `OI-014` |
| State basis | Model states are named immutable records with tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes. | `docs/_Registers/ScopeLedger.csv` row `SOW-071` |
| Authority boundary | Comparison output is diagnostic/audit functionality, not external validation or professional acceptance. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/CONTRACT.md` invariants `OPS-K-AUTH-1`, `OPS-K-MECH-2` |
| Architecture basis | Rust core/application services, schema-first command/query/job result envelopes, JSON Schema 2020-12, and JCS-compatible hash basis where JSON payloads are hashed. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 |

## Conditions

| Condition | Status |
|---|---|
| Input state schema | Depends on `DEL-14-01` immutable model state records. |
| Mapping and tolerance contracts | Depends on `DEL-14-05`; tolerance defaults and mapping workflows remain `TBD`. |
| Unit-aware comparison of unit-bearing values | Depends on `DEL-02-02` unit system and dimensional-analysis core contract. |
| Analysis-run result deltas | Out of this deliverable except where SOW-073 context informs state comparison; the analysis-run comparison engine is `DEL-14-04`. |
| External validation/prover status | Excluded. The package does not ingest commercial prover outputs comprehensively or determine external validation. |

Source: `Dependencies.csv` approved DAG-002 mirror rows `DAG-002-E0792`, `DAG-002-E0793`, `DAG-002-E0794`; `_CONTEXT.md` Package Exclusions; `execution/_Decomposition/SOFTWARE_DECOMP.md` rows `DEL-14-03`, `DEL-14-04`, `DEL-14-05`.

## Construction

| Construction surface | Conservative construction note |
|---|---|
| Backend slice | Implement as a backend/service feature aligned with the Rust core/application-service architecture basis. Exact module path is `TBD`. |
| Input contract | Accept two immutable model-state references or payloads once `DEL-14-01` defines the state record contract. |
| Matching contract | Prefer stable IDs. Require explicit mapping records for intentional correspondence where IDs differ. |
| Diff contract | Report added, removed, changed, and unchanged model entities. Exact entity categories and payload normalization are `TBD` pending state schema detail. |
| Hash/canonicalization | Where JSON payloads are hashed, use the accepted JCS-compatible canonical JSON basis. |
| Diagnostics | Preserve warnings, unresolved assumptions, provenance, and professional-boundary status in result envelopes. |
| Tests | Provide deterministic state diff tests covering stable-ID matches, mapped entities, and unmatched entities. Exact fixture model content is `TBD` and must avoid protected/private data. |

## References

- `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and dependency envelope.
- `_REFERENCES.md` - local reference index for this deliverable.
- `Dependencies.csv` - approved DAG-002 local mirror/evidence surface.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.5 decomposition basis.
- `docs/_Registers/Deliverables.csv` - deliverable row `DEL-14-03`.
- `docs/_Registers/ScopeLedger.csv` - rows `SOW-071` and `SOW-073`.
- `docs/CONTRACT.md` - invariants for IDs, units, professional authority, data, and agent behavior.
- `docs/SPEC.md` - unit, persistence, analysis-boundary, reporting/result-envelope, and validation mechanics.
- `docs/TYPES.md` - reference, traceability, checksum, diagnostic, result, and report boundary definitions.
