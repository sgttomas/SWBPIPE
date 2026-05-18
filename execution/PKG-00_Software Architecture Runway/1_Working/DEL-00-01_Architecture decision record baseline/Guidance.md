# Guidance: DEL-00-01 Architecture decision record baseline

## Interpretation
Decision surface for stack, runtime, GUI framework, solver-library, and packaging target choices. The work product should help a future implementation agent understand the boundary, evidence, and unresolved choices without turning this architecture runway into product implementation.

## Design Rationale
This deliverable records decision structure and required evidence; it does not choose a language, GUI framework, solver library, storage format, license, or packaging target. This separation matters because PKG-00 is intended to prevent later packages from making incompatible local choices about services, storage, diagnostics, GUI state, APIs, and acceptance gates.

## Architecture Guidance
- Prefer explicit contracts over package-local assumptions.
- Keep architecture language concrete enough for later implementation but abstract enough to avoid premature stack decisions.
- Use `TBD` when a decision needs human authority or later technical evaluation.
- Treat diagnostics, provenance, units, and data-boundary checks as cross-cutting architecture obligations.
- Preserve the distinction between mechanical calculation, user rule checking, and professional approval.

## Decision Handling
- Record a choice as `TBD` when no cited human ruling exists.
- Record a choice as `PROPOSAL` only when it is explicitly framed for review.
- Do not convert a proposed architecture option into an accepted decision inside this deliverable.
- When a future package depends on this deliverable, cite the accepted architecture document or note that the dependency is still awaiting human ruling.

## Guardrails
- Do not copy or paraphrase protected standards tables, code text, or proprietary engineering values.
- Do not claim code compliance or professional approval.
- Do not infer dependency edges while Full DAG authoring is deferred.
- Do not advance PKG-01 through PKG-12 from this deliverable.

## Human-Ruling Queue
- TBD: Select implementation stack and runtime.
- TBD: Select GUI framework and packaging targets.
- TBD: Select numerical library policy and licensing constraints.
