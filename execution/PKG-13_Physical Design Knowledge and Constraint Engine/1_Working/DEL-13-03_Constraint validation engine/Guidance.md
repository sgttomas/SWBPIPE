# Guidance: DEL-13-03 Constraint validation engine

## Purpose

This deliverable exists to make physical-design constraint validation visible, deterministic, and provenance-aware before downstream transformation or review workflows rely on model data. It supports OBJ-014 by contributing to a schema-backed piping design model that captures physical design context, design knowledge, constraints, assumptions, and analytical transformation traceability. Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014 and DEL-13-03 rows.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Validate available knowledge only | Treat available design knowledge and explicit constraints as the validation surface. Do not infer hidden owner standards or protected code criteria. | `_CONTEXT.md`; SOW-068; `docs/CONTRACT.md` OPS-K-AGENT-1 |
| Missing data is a finding | Missing solve-required or rule-check-required values must become explicit diagnostics/findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3 |
| Preserve provenance | Where design knowledge includes source/provenance, messages should preserve that visibility so later users can review the basis. Exact fields remain TBD. | SOW-068; `docs/SPEC.md` sections 1 and 4 |
| Keep authority boundaries visible | Constraint validation is not professional approval, code compliance, certification, sealing, or owner-standard interpretation. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 4.3 |
| Keep public artifacts clean | Do not copy protected standards text, protected tables, proprietary values, private owner data, or code-specific defaults into public fixtures, messages, or docs. | `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |
| Use schema-first boundaries | Future implementation should fit the accepted schema-first command/query/job/result-envelope posture. Exact constraint diagnostic schema remains TBD. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` sections 1 and 9 |

## Considerations

- The validation engine depends on upstream design-knowledge, constraint-entity/provenance, unit, diagnostics, persistence, and architecture-basis work according to the approved local `Dependencies.csv` mirror. This setup pass uses those rows as evidence only and does not inspect sibling DEL folders.
- Constraint categories named by SOW-068 are scope categories, not complete engineering acceptance criteria. Any concrete clearance, slope, vent, drain, route, or support-zone value must come from user/project/private sources or later lawful public fixtures.
- Provenance-aware messages can reveal where a value came from or that source evidence is missing, but they cannot create redistribution rights or legal clearance for protected data.
- Deterministic ordering, stable identifiers, and replayable tests are important for downstream audit, but the accepted sources do not yet define the exact message ID scheme.
- The accessible sources do not define localization, severity taxonomy, constraint graph representation, route geometry representation, or conflict-resolution algorithms for this deliverable. These remain `TBD`.

## Trade-offs

| Trade-off | Conservative direction |
|---|---|
| Helpful defaults vs. engineering safety | Prefer explicit missing-data findings over defaults. |
| Broad validation vs. source authority | Validate only what the available design knowledge and accepted schemas can support. |
| User-facing clarity vs. protected-content risk | Explain the missing or conflicting condition without quoting protected standards or private owner criteria. |
| Strict blocking vs. reviewable warning | Use the accepted diagnostic schema when it exists; until then, record severity policy as TBD rather than inventing blocking rules. |
| Early implementation detail vs. architecture stability | Keep module path, API shape, and diagnostic record fields TBD until resolved by sealed implementation work or human approval. |

## Examples

TBD. The accessible sources identify validation categories and boundaries but do not provide concrete public-safe example payloads or expected messages for DEL-13-03. Later examples must use invented or otherwise permitted data with provenance/review status.

## Conflict Table (for human ruling)

No cross-source conflicts were identified in this setup pass. The main unresolved items are `TBD` implementation details rather than contradictory source claims.
