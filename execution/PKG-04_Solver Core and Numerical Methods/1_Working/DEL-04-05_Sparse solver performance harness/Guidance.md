# Guidance: DEL-04-05 Sparse solver performance harness

## Purpose

This deliverable prepares the evidence boundary for a future deterministic sparse-solver performance/regression harness. It exists to keep performance and conditioning checks observable, repeatable, and reviewable without coupling benchmark code to solver implementation choices.

## Principles

- Treat the harness as an observer and regression surface, not as solver logic.
- Prefer deterministic, schema-backed inputs and outputs so repeated runs can be compared.
- Preserve unknowns as `TBD`; do not invent runtime, memory, model-size, or conditioning thresholds.
- Use only invented, public-permissive, or otherwise lawful benchmark fixtures.
- Keep result wording mechanics-focused and avoid certification, compliance, approval, or professional-reliance claims.

## Considerations

Sparse performance measurements are sensitive to solver library, hardware, compiler settings, platform, matrix ordering, and fixture structure. Those implementation variables are not resolved here. The setup kit should therefore describe evidence expected from a future harness while leaving thresholds and tool choices open.

Conditioning cases should exercise solver diagnostic behavior without embedding protected standards examples or proprietary commercial models. If a fixture source cannot be proven redistributable, it should not enter the public harness.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Deterministic regression vs benchmark realism | Prefer reproducible invented/public fixtures first; practical realism improves only when provenance and redistribution rights are clear. |
| CI stability vs timing sensitivity | Timing gates are `TBD`; early harnesses may record metrics without failing builds on unsupported thresholds. |
| Solver abstraction vs library-specific diagnostics | Keep common harness records stable; library-specific fields should be explicit and reviewable if introduced. |
| Performance visibility vs professional claims | Publish observations and warnings, not certification, compliance, or engineering acceptance. |

## Examples

Concrete benchmark models, numerical thresholds, matrix sizes, timing budgets, and conditioning acceptance values are `TBD`. Future examples must use invented or public-permissive data with provenance.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in setup evidence. | N/A | N/A | N/A | N/A | N/A |

## Open Enrichment Items

| Item | Status |
|---|---|
| Approved performance metrics and threshold policy | TBD |
| Approved practical model-size taxonomy | TBD |
| Approved sparse solver/library choice | TBD |
| Public/permissive fixture source list | TBD |
