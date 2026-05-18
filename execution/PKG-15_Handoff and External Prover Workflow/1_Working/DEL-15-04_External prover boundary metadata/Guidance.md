# Guidance: DEL-15-04 External prover boundary metadata

## Purpose

DEL-15-04 exists to let OpenPipeStress record external-prover workflow context without converting that context into automatic professional approval. The useful metadata is descriptive and traceable: names, tags, notes, external references, attachments, and links to handoff/comparison evidence. Source: `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-15-04; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-04 row.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Descriptive, not authoritative | Treat external-prover metadata as context for review and handoff, not as proof that a model is code-compliant or professionally accepted. | `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/CONTRACT.md` OPS-K-AUTH-1; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-015 |
| Flexible metadata | Prefer open descriptive fields for names, tags, notes, references, attachments, and comparison links over fixed prover-status stages. | SOW-075; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-04 row |
| No lifecycle overclaim | Do not encode a formal prover lifecycle unless later authorized by explicit scope change. | `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-016 |
| Human authority remains external | Human acceptance, when it exists, is outside solver authority and must be a human-owned, hash-bound project record. | `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.4 |
| Preserve data boundaries | Do not embed protected standards content, commercial software examples, private project data, or proprietary values in public examples or fixtures. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6; `docs/SPEC.md` report/result export boundary sections |
| Expose unknowns | Missing or unsupported metadata should be recorded as `TBD` or a diagnostic rather than silently defaulted. | `INIT.md` Agent rule; `docs/DIRECTIVE.md` section 2.2; `docs/CONTRACT.md` OPS-K-DATA-2 |

## Considerations

- `ASSUMPTION`: The eventual implementation will likely be a schema-backed record or extension to an existing schema surface because the accepted architecture basis uses JSON Schema 2020-12 and schema-first contracts. The exact schema file and module are `TBD`.
- `ASSUMPTION`: Boundary validation tests should include both positive flexible-metadata cases and negative prohibited-status cases. The exact test harness is `TBD`.
- The approved local dependency mirror says this deliverable binds to handoff package, target mapping, export workflow, immutable state, and professional-boundary context. Those rows are evidence of dependency direction only; they do not provide the content of those predecessor deliverables.
- Comprehensive commercial-tool result ingestion is explicitly deferred for MVP; a metadata link to an external artifact is not the same as parsing, validating, or accepting that artifact.

## Trade-offs

| Trade-off | Direction |
|---|---|
| Flexible metadata vs. standardized lifecycle | Use flexible metadata now. A standardized lifecycle would conflict with SOW-075 and DEC-016 unless later approved. |
| External references vs. external result ingestion | Store references and descriptive context; do not ingest comprehensive commercial-tool results in this MVP deliverable. |
| Review usefulness vs. authority overclaim | Include enough provenance and hash/reference context for a reviewer, but avoid language that implies software certification or code compliance. |
| Public examples vs. realistic external artifacts | Use invented or cleared examples only. Real commercial files, protected standards material, and private owner data require permission and review. |

## Examples

The sources support only category-level examples. Concrete field names and payloads remain `TBD`.

| Supported Metadata Category | Example Shape |
|---|---|
| Name | `TBD`: human-readable external reference label |
| Tag | `TBD`: user-defined classification tag |
| Note | `TBD`: free-form review note subject to privacy/protected-content controls |
| External reference | `TBD`: pointer to external artifact or record |
| Attachment | `TBD`: attachment reference with provenance/privacy metadata |
| Comparison report link | `TBD`: link to diagnostic comparison evidence, not acceptance status |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No direct source conflict identified in accessible sources. | N/A | N/A | N/A | N/A | N/A |
