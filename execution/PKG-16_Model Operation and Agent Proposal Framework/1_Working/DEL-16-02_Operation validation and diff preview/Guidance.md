# Guidance: DEL-16-02 Operation validation and diff preview

## Purpose

This deliverable exists to make proposed model edits reviewable and blockable before they become accepted model changes. It supports OBJ-015 by turning GUI and agent changes into validated, previewed, auditable model-operation flows. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objectives`; `docs/_Registers/ScopeLedger.csv`.

## Principles

| Principle | Guidance |
|---|---|
| Treat operations as the mutation boundary | All GUI and agent edits should be represented as structured operations before application. Do not design a bypass path for direct accepted-state mutation. Source: SOW-069; `_CONTEXT.md#Package Reference`. |
| Validate before applying | Schema validation, constraint validation, and preview generation are gatekeeping steps before controlled application. Source: SOW-069 and `_CONTEXT.md#Description`. Exact ordering beyond the source wording remains TBD. |
| Preview deterministically | A diff preview should be stable for the same operation and model basis. Source: SOW-069; `Dependencies.csv` rows for DEL-14-03 and DEL-14-05. Exact comparison payload shape remains TBD. |
| Preserve diagnostics | Validation failures and warnings should remain structured and provenance-aware. Source: architecture basis AB-00-06 in `_CONTEXT.md`; `docs/SPEC.md#4.3`. |
| Preserve professional boundaries | Validation and preview results are decision-support/control-surface outputs, not professional approval or code-compliance claims. Source: `docs/CONTRACT.md#Invariant index`; `docs/SPEC.md#4.3`. |
| Prefer explicit TBDs | When schema fields, API contracts, tolerance defaults, or operation granularity are not defined by accessible sources, keep them as `TBD` rather than inventing implementation evidence. Source: `docs/CONTRACT.md#Invariant index`. |

## Considerations

- DEL-16-02 depends on DEL-16-01 for structured operation schema semantics. Implementation should not invent operation shapes in this deliverable. Source: `Dependencies.csv` row `DAG-002-E0827`.
- DEL-16-02 depends on DEL-13-03 for constraint validation. Constraint failures should be surfaced as validation messages, not hidden report prose or agent text. Source: `Dependencies.csv` row `DAG-002-E0828`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Scope Ledger`.
- DEL-16-02 depends on DEL-14-03 and DEL-14-05 for state comparison and mapping/tolerance/export contract context. Preview behavior should remain deterministic and diagnostic, not automatic external validation. Source: `Dependencies.csv` rows `DAG-002-E0829` and `DAG-002-E0830`; SOW-073 notes in decomposition.
- DEL-16-02 depends on DEL-04-06 for diagnostics/warning contract context. Validation and preview failures should preserve diagnostic classes and provenance where the relevant contract is available. Source: `Dependencies.csv` row `DAG-002-E0831`.
- Approved architecture-basis rows from PKG-00 are dispatchable context evidence, not Type 2 implementation authority by themselves. Source: `_CONTEXT.md#Architecture Basis Injection`; `_DEPENDENCIES.md#Authority Boundary`.

## Trade-offs

| Trade-off | Conservative handling |
|---|---|
| Early blocking vs richer preview | Sources require invalid operations to be blocked before application. Whether preview runs after partial validation failure is TBD and should be resolved by implementation design. |
| Deterministic preview vs tolerance defaults | Sources require deterministic preview and reference mapping/tolerance contracts, but default tolerances are unresolved in decomposition open issue OI-014. Keep defaults TBD unless upstream contracts define them. |
| Agent convenience vs accepted-state safety | Agents may propose operations but do not mutate accepted engineering state directly. Favor an explicit review/apply boundary over convenience shortcuts. |
| Validation detail vs protected-data boundary | Do not encode protected standards text, proprietary values, or code-specific defaults in validation logic. Missing required values become explicit findings. |

## Examples

No approved local source provides operation fixtures, schema examples, or diff-preview payload examples for DEL-16-02. Example content is therefore `TBD` pending DEL-16-01 schema fixtures and implementation tests.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in the accessible slices; implementation details remain TBD where sources are silent. | N/A | N/A | N/A | N/A | N/A |
