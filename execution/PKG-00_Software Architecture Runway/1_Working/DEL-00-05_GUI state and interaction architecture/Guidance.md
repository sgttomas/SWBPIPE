# Guidance: DEL-00-05 GUI state and interaction architecture

## Interpretation
GUI state, editing, selection, undo/redo, viewport integration, and workflow architecture before GUI slices proceed. The work product should help a future implementation agent understand the boundary, evidence, and unresolved choices without turning this architecture runway into product implementation.

## Design Rationale
This deliverable defines GUI interaction architecture only; it does not implement screens, components, styling, viewport rendering, or user interface code. This separation matters because PKG-00 is intended to prevent later packages from making incompatible local choices about services, storage, diagnostics, GUI state, APIs, and acceptance gates.

The accepted D-41 `DEC-074` O2 boundary makes this relationship explicit:
`DEL-00-05` owns the cross-cutting GUI state and interaction architecture,
while `DEL-07-02` owns model-tree and property-inspector behavior. Behavior
evidence belongs with `DEL-07-02`; this kit supplies architecture-basis
constraints and must not describe that delegated behavior as absent local
scope.

## Architecture Guidance
- Prefer explicit contracts over package-local assumptions.
- Keep architecture language concrete enough for later implementation but abstract enough to avoid premature stack decisions.
- Use `TBD` when a decision needs human authority or later technical evaluation.
- Treat diagnostics, provenance, units, and data-boundary checks as cross-cutting architecture obligations.
- Preserve the distinction between mechanical calculation, user rule checking, and professional approval.
- Treat model-tree/property-inspector behavior as delegated `DEL-07-02` scope; review it here only for conformance with the GUI state and interaction architecture.

## Decision Handling
- Record a choice as `TBD` when no cited human ruling exists.
- Record a choice as `PROPOSAL` only when it is explicitly framed for review.
- Do not convert a proposed architecture option into an accepted decision inside this deliverable.
- When a future package depends on this deliverable, cite the accepted architecture document or note that the dependency is still awaiting human ruling.

## Guardrails
- Do not copy or paraphrase protected standards tables, code text, or proprietary engineering values.
- Do not claim code compliance or professional approval.
- Use approved `DAG-007` only for relationship context; do not infer lifecycle promotion, implementation readiness, or release acceptance from graph edges.
- Do not advance PKG-01 through PKG-12 from this deliverable.

## Human-Ruling Queue
- TBD: Select GUI framework and viewport technology.
- TBD: Approve state-management and undo/redo architecture.
