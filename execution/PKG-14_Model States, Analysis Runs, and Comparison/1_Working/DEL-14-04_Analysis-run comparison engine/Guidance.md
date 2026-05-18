# Guidance: DEL-14-04 Analysis-run comparison engine

**Generated:** 2026-05-03
**Document Role:** Directional draft
**Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`

## Purpose

DEL-14-04 exists to make analysis-run comparisons deterministic, unit-aware, traceable, and suitable for diagnostic/audit review. Its output supports design iteration and review; it does not decide external validation or professional reliance.

## Principles

- Keep comparison evidence tied to exact run/model-state basis, solver version, settings, units, load cases, diagnostics, result hashes, and relevant rule/library references.
- Prefer stable IDs; use explicit manual mappings when stable IDs do not establish a safe comparison relationship.
- Treat unit-bearing values as comparable only when unit and dimension metadata support the comparison. Missing or ambiguous unit metadata is a diagnostic.
- Preserve raw delta evidence separately from tolerance-based classification. `ASSUMPTION`: this separation is needed so tolerance profiles can change without rewriting source evidence; the exact output structure is `TBD`.
- Keep diagnostics and settings in scope because the decomposition names them explicitly for DEL-14-04.
- Preserve the professional boundary: comparison output is review evidence, not approval, certification, sealing, authentication, or code compliance.

## Considerations

| Topic | Guidance |
|---|---|
| Mapping | DEL-14-05 owns mapping/tolerance/export contracts. DEL-14-04 should consume those contracts rather than inventing mapping semantics. |
| Tolerances | OI-014 records comparison tolerance defaults and mapping workflows as `TBD`; do not hard-code defaults without human/product approval. |
| Units | Unit normalization depends on accepted unit metadata and conversion governance. Unsupported conversion semantics should remain `TBD` or become diagnostics. |
| Result scope | Result export envelopes are the intended review/regression/report-consumption surface. Avoid ad hoc result shapes once the schema contract is available. |
| Hashes | JSON payload hashes use the accepted JCS-compatible basis where JSON payloads are hashed. Non-JSON and binary partitioning remains `TBD` in the broader architecture. |
| Protected content | Fixtures, examples, and report sections must not embed protected standards text, protected tables, proprietary values, private project data, or private rule-pack payloads. |

## Trade-offs

| Trade-off | Direction |
|---|---|
| Early implementation vs. contract maturity | Keep unresolved mapping, tolerance, and result-schema choices as `TBD` until upstream contracts are accepted. |
| Raw deltas vs. classified outcomes | Preserve raw evidence and expose tolerance-classified outcomes as derived interpretation. |
| Diagnostic detail vs. professional-boundary risk | Include enough diagnostics for review and audit while avoiding compliance/approval language. |
| Broad comparison scope vs. deliverable boundary | Cover mapped result entities, diagnostics, and settings; defer export semantics and mapping/tolerance contract definition to DEL-14-05. |

## Examples

Concrete examples are `TBD`. Future examples should use invented or otherwise permitted data, documented provenance, explicit units, and no protected standards text/tables or proprietary values.

## Open Questions

| Question ID | Question | Source |
|---|---|---|
| OQ-14-04-001 | What are the accepted default tolerance profiles and mapping workflows for comparison classification? | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-014 |
| OQ-14-04-002 | What is the final comparison output schema/API shape? | `docs/SPEC.md` result export baseline; DEL-14-05 dependency |
| OQ-14-04-003 | What fixture set proves deterministic ordering, unit-normalized delta behavior, and diagnostic/settings comparison without protected or private data? | `_CONTEXT.md` anticipated artifact; `docs/IP_AND_DATA_BOUNDARY.md` |

## Conflict Table (for human ruling)

No source conflicts were identified in the accessible source set. Open questions above are missing-decision gaps, not contradictions.
