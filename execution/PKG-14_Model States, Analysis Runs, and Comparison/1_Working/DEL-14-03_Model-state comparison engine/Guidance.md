# Guidance: DEL-14-03 Model-state comparison engine

## Purpose

This deliverable exists to make immutable model states reviewable through deterministic comparison. The comparison should support design iteration by identifying added, removed, changed, and unchanged model entities without converting that comparison into an external validation or professional approval state.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` row `DEL-14-03`; `docs/_Registers/ScopeLedger.csv` rows `SOW-071` and `SOW-073`.

## Principles

| Principle | Guidance |
|---|---|
| Stable identity first | Treat stable IDs and typed references as the primary comparison surface. Positional or display-name matching is an implementation risk unless explicitly justified by a mapping contract. |
| Explicit mappings | When two states intentionally refer to corresponding entities with different IDs, use explicit mapping records. Do not infer hidden engineering equivalence from proximity, naming, or apparent similarity without a source-backed rule. |
| Deterministic output | The same inputs should produce the same classifications and serialized result shape. Hashing or canonical serialization details are governed by the persistence/hash basis where JSON payloads are involved. |
| No silent defaults | Missing mappings, missing units, unsupported entity categories, and unresolved assumptions should become diagnostics or `TBD`, not implicit acceptance. |
| Metadata preservation | Notes, warnings, external references, unresolved assumptions, provenance, and hashes attached to model states are part of the review context. |
| Boundary clarity | The comparison is diagnostic/audit functionality. It must not imply professional review, code compliance, external prover approval, or certification. |

Sources: `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/CONTRACT.md` invariants `OPS-K-DATA-2`, `OPS-K-AUTH-1`, `OPS-K-MECH-2`; `docs/SPEC.md` sections 4, 4.3, and 4.4.

## Considerations

| Topic | Consideration |
|---|---|
| Model-state versus analysis-run comparison | `DEL-14-03` is scoped to model-state diffs. Analysis-run result deltas are separately assigned to `DEL-14-04`; mapping/tolerance/export contracts are assigned to `DEL-14-05`. |
| Tolerances | Comparison tolerance defaults and mapping workflows are an open issue (`OI-014`). Treat detailed tolerance behavior as `TBD` until resolved. |
| Unit-normalized values | SOW-073 mentions unit-normalized result deltas, but the unit contract dependency (`DEL-02-02`) and comparison contract dependency (`DEL-14-05`) must be honored before numeric comparison policy is finalized. |
| Entity granularity | The exact model entity categories are `TBD` until the immutable state schema and canonical domain model detail are available. |
| Fixture content | Tests should use invented/public-safe fixtures and should carry provenance status. Do not import protected standards examples or proprietary project data. |
| Context size | The deliverable has an `L` context envelope and WATCH risk; scope should be split or escalated if it expands beyond deterministic model-state entity diffs. |

## Trade-offs

| Trade-off | Direction |
|---|---|
| Strict stable-ID matching vs. heuristic matching | Prefer strict stable-ID matching. Use explicit mapping records for intentional correspondence. Heuristics are `TBD` and require human-approved scope if introduced. |
| Rich diff payload vs. bounded backend slice | Start with source-supported classifications and diagnostic metadata. Defer UI overlays, report formatting, and export semantics to their owning deliverables. |
| Canonical hash comparison vs. field-level comparison | Hashes support reproducibility, but field-level changed classification needs a source-defined normalization policy. The exact field comparison policy remains `TBD`. |
| Numeric tolerance vs. exact equality | Numeric tolerance policy is not yet source-defined for this deliverable. Unit and tolerance dependencies must be resolved before implementation claims are made. |

## Examples

Source-grounded example categories are limited to:

- an entity appears only in the later state: added;
- an entity appears only in the earlier state: removed;
- an entity appears in both states with the same stable ID or explicit mapping and no relevant difference: unchanged;
- an entity appears in both states with the same stable ID or explicit mapping and a source-supported relevant difference: changed.

Concrete fixture entities, fields, values, units, tolerances, and mapping records are `TBD` until the state schema and mapping/tolerance contract are available.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| C-14-03-001 | Dependency-extract v3.1 write-form enums would normalize/reclassify several approved DAG-002 mirror values, but the project task rule says to preserve approved DAG-002 rows as ACTIVE without reclassification. | `skills/dependency-extract/SKILL.md` "Canonical enums"; `Dependencies.csv` approved mirror rows | User task instruction for `DEL-14-03` dependency handling | Dependency register handling; final report | Preserve the approved mirror unchanged and record the conflict. | TBD |
