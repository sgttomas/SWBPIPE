# Datasheet: DEL-14-05 Comparison mapping, tolerance, and export contracts

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-14-05 | `_CONTEXT.md` |
| Name | Comparison mapping, tolerance, and export contracts | `_CONTEXT.md` |
| Package ID | PKG-14 | `_CONTEXT.md` |
| Package | Model States, Analysis Runs, and Comparison | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-14` |
| Type | API_CONTRACT | `_CONTEXT.md` |
| Scope item | SOW-073 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073` |
| Primary objective | OBJ-016 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#OBJ-016` |
| Context envelope | M | `_CONTEXT.md` |
| Anticipated artifacts | comparison mapping schema; tolerance profile schema; comparison exporters | `_CONTEXT.md` |

## Attributes

| Attribute | Current value |
|---|---|
| Contract subject | Manual comparison mappings, unmatched classifications, tolerance profiles, and CSV/JSON/report-section export semantics. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#DEL-14-05`. |
| Comparison basis | Two model states and/or two analysis runs are compared deterministically using stable IDs, manual mappings where required, unit-normalized result deltas, and tolerance profiles. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073`. |
| State/run context | Model states and analysis runs are first-class product records for design iteration and review. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md#OBJ-016`. |
| Export boundary | Result export envelopes identify result set, model/run basis, solver version, unit-system reference, diagnostics, provenance, hashes or audit manifest reference, analysis statuses, and professional-boundary notice. Source: `docs/SPEC.md#result-export-format`. |
| Unit boundary | Calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked. Source: `docs/CONTRACT.md#OPS-K-UNIT-1`. |
| Professional boundary | Comparison output is diagnostic/audit functionality, not automatic external validation or acceptance. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073`; `docs/TYPES.md#TraceabilityLink`. |
| Public data boundary | Public artifacts must not copy private formulas, protected standards text, protected tables, proprietary values, or private rule-pack payloads. Source: `docs/SPEC.md#result-export-format`; `docs/IP_AND_DATA_BOUNDARY.md#public-repository-must-not-contain`. |
| Architecture basis | JSON Schema 2020-12 contracts, schema-first envelopes, and canonical JSON/JCS-compatible hash basis apply where payloads are hashed. Source: `_CONTEXT.md#Architecture-Basis-Injection`. |

## Conditions

| Condition | Status |
|---|---|
| Tolerance defaults | TBD. `execution/_Decomposition/SOFTWARE_DECOMP.md#OI-014` states comparison tolerance defaults and mapping workflows are pending solver/result schema prototypes. |
| Exact CSV fields | TBD. DEL-14-05 is assigned CSV export semantics, but the accessible sources do not define field names or column order. |
| Exact JSON schema | TBD. The baseline is schema-first JSON, but the accessible sources do not include a deliverable-specific schema body. |
| Report-section layout | TBD. Report sections must preserve privacy, provenance, units, diagnostics, limitations, and professional-boundary notices; final rendering and layout remain outside the accessible source slice. |
| External validation | Excluded. PKG-14 does not ingest commercial prover outputs comprehensively or determine external validation. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-14`. |
| Protected standards data | Excluded from public artifacts unless separately authorized by provenance and review. Source: `docs/CONTRACT.md#OPS-K-IP-1`; `docs/IP_AND_DATA_BOUNDARY.md#public-repository-must-not-contain`. |

## Construction

| Construct | Description |
|---|---|
| Comparison mapping schema | Defines stable-ID mapping records and manual mapping evidence needed when state/run entities do not align automatically. Details remain TBD pending DEL-14-03/DEL-14-04 interfaces. |
| Unmatched classification schema | Defines explicit classifications for compared entities or results that have no accepted counterpart. Specific enum values are TBD unless later source material supplies them. |
| Tolerance profile schema | Defines unit-aware tolerance profile records for comparison deltas. Default numeric values are TBD and must not be silently supplied. |
| JSON export contract | Schema-first result/comparison envelope compatible with governed downstream tooling and report consumption. Exact schema fields are TBD. |
| CSV export contract | Tabular export semantics for comparison review. Exact columns, ordering, and serialization rules are TBD. |
| Report-section contract | Report-facing comparison section references that preserve units, diagnostics, provenance, hashes, assumptions, limitations, and professional-boundary notice. Exact layout is TBD. |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `_DEPENDENCIES.md`
- `Dependencies.csv`
- `execution/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/DIRECTIVE.md`
- `execution/_DAG/DAG-002/APPROVAL_RECORD.md`
