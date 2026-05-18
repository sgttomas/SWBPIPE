# Guidance: DEL-04-04 Nonlinear support active-set solver

## Purpose

This deliverable prepares a future nonlinear support active-set solver slice within PKG-04. Its value is to keep nonlinear mechanical behavior, diagnostics, unit handling, and result reporting bounded before implementation work begins.

## Principles

- Treat active-set matrices and support states as mechanics-solving concerns, not rule-pack compliance decisions.
- Keep friction, gap, lift-off, and one-way support behavior explicit in state and diagnostics rather than hidden in defaults.
- Use `TBD` for numerical tolerances, friction defaults, and exact data contracts until an authorized implementation brief supplies or derives them.
- Preserve result-envelope discipline from AB-00-03 and diagnostic discipline from AB-00-06.
- Use invented verification fixtures only; do not reproduce protected code examples, tables, or formulas.

## Considerations

The future implementation will likely need to coordinate with linear support models, the frame stiffness kernel, solver diagnostics, and sparse solver behavior. Those are execution relationships for later work; this setup pass records them without editing other deliverables or implementing code.

Convergence reporting needs enough information for downstream diagnostics and reports to disclose unresolved non-convergence, assumptions, and limitations. It must not imply engineering approval or code compliance.

## Trade-offs

| Topic | Setup guidance |
|---|---|
| Scope size | DEL-04-04 is large but remains one numerical domain per ContextBudgetQA. Split only if later implementation crosses package boundaries or expands beyond nonlinear support behavior. |
| Completeness vs invention | Prefer `TBD` over invented convergence tolerances, friction coefficients, or activation defaults. |
| Diagnostic detail | Favor structured diagnostic/result fields over prose-only errors so AB-00-06 and OPS-K-SOLVER-2 remain testable. |
| Test realism | Use invented mechanical examples sufficient for deterministic verification; do not copy protected benchmark tables or code examples. |

## Examples

Example cases are intentionally not specified in this setup pass. Future examples should use invented geometry, invented loads, and invented support properties, with explicit units and no protected standards content.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected in setup pass. | N/A | N/A | N/A | N/A | N/A |
