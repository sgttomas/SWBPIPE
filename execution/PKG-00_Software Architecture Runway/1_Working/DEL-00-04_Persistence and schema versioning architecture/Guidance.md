# Guidance: DEL-00-04 Persistence and schema versioning architecture

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-00-04-DECL-003`.

## Interpretation
Persistence, schema versioning, migration, canonicalization, and round-trip architecture before project-file implementation. The work product should help a future implementation agent understand the boundary, evidence, and unresolved choices without turning this architecture runway into product implementation.

## Design Rationale
This deliverable defines persistence architecture only; it does not implement schemas, migrations, serializers, database tables, or project-file readers. This separation matters because PKG-00 is intended to prevent later packages from making incompatible local choices about services, storage, diagnostics, GUI state, APIs, and acceptance gates.

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
- Use approved `DAG-007` only for relationship context; do not infer lifecycle promotion, implementation readiness, or release acceptance from graph edges.
- Do not advance PKG-01 through PKG-12 from this deliverable.

## Human-Ruling Queue
- TBD: Select physical persistence format.
- TBD: Approve schema-version and migration policy.
- TBD: Approve canonicalization and hashing strategy.
