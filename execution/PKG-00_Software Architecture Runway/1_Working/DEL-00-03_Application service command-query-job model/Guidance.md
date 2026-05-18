# Guidance: DEL-00-03 Application service command-query-job model

## Interpretation
Command, query, job, cancellation, progress, and transaction model connecting user-facing workflows to the domain core. The work product should help a future implementation agent understand the boundary, evidence, and unresolved choices without turning this architecture runway into product implementation.

## Design Rationale
This deliverable defines service contracts and flow rules only; it does not implement commands, queues, solvers, storage, CLI handlers, or GUI actions. This separation matters because PKG-00 is intended to prevent later packages from making incompatible local choices about services, storage, diagnostics, GUI state, APIs, and acceptance gates.

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
- TBD: Approve command/query/job vocabulary.
- TBD: Choose concrete async/job execution mechanism after stack selection.
