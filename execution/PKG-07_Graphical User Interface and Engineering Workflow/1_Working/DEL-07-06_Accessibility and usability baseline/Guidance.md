# Guidance: DEL-07-06 Accessibility and usability baseline

## Purpose

This deliverable frames accessibility and usability as engineering-review support, not as a standalone visual polish task. The baseline should help a user navigate major GUI panels, find missing inputs, understand warnings, review results, and inspect assumptions without hiding the project boundaries defined by OpenPipeStress governance.

## Principles

- Accessibility supports auditability: keyboard paths, readable statuses, copy/export, and clear labels help reviewers inspect the model and evidence.
- Missing data remains visible. The GUI must not hide or silently repair missing solve-required values, rule-check inputs, weak provenance, or assumptions.
- Warning classes should stay distinct. Solve readiness, rule-check readiness, provenance, assumptions, nonlinear uncertainty, and IP boundary warnings have different engineering meanings.
- Units and provenance are part of readability. A readable table that omits units or source status is not adequate for engineering review.
- Accessibility conformance is not finalized here. The exact WCAG or other target remains `TBD` until human ruling.
- Public fixtures and report examples must remain protected-data-free and private-data-free.
- The software may support review; it does not certify, approve, seal, authenticate, or declare engineering code compliance.

## Considerations

Keyboard navigation should cover primary work surfaces before detailed shortcuts are optimized. Major panels include the model tree, property inspector, editor surfaces, solver/diagnostic surfaces, results browser, and report preview/export paths named in PRD section 14 and PRD section 21.

Contrast/readability should be evaluated against engineering review content, not only decorative UI elements. Result color maps, warning badges, units, provenance labels, and status text are high-risk because they affect interpretation of the model and results.

Search/filter and copy/export matter because engineering models can grow large. These functions should preserve context such as object identity, units, warning class, and source/provenance notes.

Undo/redo needs a review-aware boundary. It should help users recover reversible modeling edits while preserving diagnostics and not implying that solved or checked states remain valid after content changes.

Reports are in scope only as report-facing review surfaces and output expectations. This deliverable does not edit report templates, but it preserves the need for readable warnings, assumptions, units, provenance, and professional-boundary notices.

The target decision is a human project-authority decision, not a TASK inference. Until that decision exists, implementation work should use the qualitative baseline in PRD section 21 and mark measurable conformance claims as `TBD`.

## Trade-offs

| Decision area | Setup guidance |
|---|---|
| Qualitative baseline vs. formal target | Use the qualitative PRD baseline now; record the formal target as `TBD` until a human decision. |
| Visual density vs. readability | Engineering review surfaces may be dense, but units, warning classes, and object identity must remain legible. |
| Color maps vs. status clarity | Color can help but must not be the only signal for result state, warning class, or rule-check status. |
| Keyboard workflow vs. shortcut breadth | Establish reliable navigation and focus order before broad shortcut catalogs. |
| Report preview vs. report template design | Preserve report accessibility requirements without changing report templates in this setup deliverable. |

## Examples

- A keyboard user can move from the model tree to the property inspector, inspect a selected component, see that a required rule-pack input is missing, and reach the diagnostic details without using a mouse.
- A result table copy/export action preserves the quantity name, object identifier, unit, result status, and relevant warning class.
- A stress-ratio visualization uses color plus a textual/status cue so review does not depend on color alone.
- A public screenshot fixture uses invented or neutral data and does not expose protected standards content, proprietary values, or private project data.

## Pass 3 Source Rereads

The semantic lensing items were treated as candidate improvements only. Enrichments above were checked against `_SEMANTIC_LENSING.md` items B-001 and E-001, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 1, `docs/DIRECTIVE.md` section 2.2, and `docs/TYPES.md` section 4.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| DEL-07-06-CF-001 | Baseline accessibility is in scope, but the detailed WCAG target is unresolved. | `docs/_Registers/ScopeLedger.csv` SOW-036 | `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-002 and DEL-07-06 note | Specification Standards and Verification; Guidance Principles | Keep conformance target as `TBD`; allow only qualitative setup requirements until human ruling. | TBD |
