# Guidance: DEL-07-08 Design-authoring state and comparison workspace

## Purpose

This deliverable adds a GUI workspace for design iteration and comparison review. It is the GUI-facing integration point for design knowledge, warnings, model operations, immutable states, analysis runs, and comparison records. Source: `_CONTEXT.md` / Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` / DEL-07-08 row.

## Principles

1. Consume upstream contracts, do not invent them. DEL-07-08 depends on GUI foundations plus design, transform, state/comparison, and operation contracts. The workspace should represent those contracts faithfully and mark unsupported details as TBD. Source: `Dependencies.csv`; `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` / DAG2-RD-015.
2. Keep authoring controlled. GUI-originated edits should become application-service command intents and controlled operations, not direct writes to durable project state. Source: `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-05.
3. Keep missing information visible. Missing solve-required data, rule-check data, provenance, assumptions, nonlinear uncertainty, and IP-boundary risks are findings or warnings, not values to hide or default. Source: `docs/SPEC.md` / GUI requirements; `docs/CONTRACT.md` / OPS-K-DATA-2.
4. Keep comparisons non-authoritative. State/run comparisons support design review and audit; they are not automatic professional validation, external approval, or code-compliance findings. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-073; `docs/CONTRACT.md` / OPS-K-AUTH-1.
5. Preserve public/private and protected-content boundaries. Design knowledge and project metadata may include user-owned or private information; public artifacts must not copy protected standards, owner requirements, private rule-pack payloads, or proprietary data. Source: `docs/IP_AND_DATA_BOUNDARY.md` / Public repository must not contain.

## Considerations

The workspace is context-envelope L with WATCH risk. Its scope includes several surface families: design knowledge, warnings, operations, state/run browsing, tables, and graphical overlays. If implementation expands beyond one bounded GUI workflow, the decomposition guidance says to confirm scope and split if needed. Source: `_CONTEXT.md` / Context Budget QA.

The GUI stack basis is established at an architectural level, but exact dependency versions, state library, package structure, transport API, and final app workflow behavior remain TBD. Source: `_CONTEXT.md` / Architecture Basis Injection; `docs/SPEC.md` / GUI requirements.

The warning panel should avoid flattening diagnostic meaning into plain prose. The local sources identify warning classes and diagnostic fields; display choices should preserve code, class, severity, source, affected object, message, remediation, and provenance when supplied. Source: `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06.

The design knowledge panel should not bundle owner standards, protected code data, proprietary catalog values, or invented engineering defaults. When values are unavailable, the UI should preserve TBD, missing, or provenance-warning state. Source: `docs/IP_AND_DATA_BOUNDARY.md`; `docs/CONTRACT.md` / OPS-K-DATA-1 and OPS-K-DATA-2.

## Trade-offs

| Decision pressure | Conservative guidance |
|---|---|
| Rich workspace versus bounded scope | Build around the SOW-076 surfaces and upstream contracts first; defer unrelated editors or report rendering to their owning deliverables. |
| Convenience editing versus auditability | Prefer structured operations and diff review over direct mutation because OBJ-015 requires controlled, reviewable, auditable model operations. |
| Visual overlays versus traceability | Overlays should be backed by state/comparison/diagnostic records where available; unsupported visual semantics remain TBD. |
| Smooth comparison UX versus professional boundary | Use neutral review wording. Do not label a comparison as approved, validated, certified, sealed, or code compliant unless a separate human-owned record is explicitly in scope. |
| Public examples versus realistic data | Use invented or cleared data only. Real owner/project/code/vendor data belongs in private user-controlled paths unless contribution review clears it. |

## Examples

No product UI screenshots, component implementations, or locally accessible PRD examples were available in this workflow. Example layout, copy, fixtures, and visual encoding remain TBD until an implementation brief supplies or creates product evidence.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None observed in production-document drafting | No cross-document content conflict was found during four-doc Pass 2. | TBD | TBD | TBD | N/A | TBD |
