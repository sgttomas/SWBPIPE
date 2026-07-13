# Guidance: DEL-13-03 Constraint validation engine

## Purpose

This deliverable exists to make physical-design constraint validation visible, deterministic, and provenance-aware before downstream transformation or review workflows rely on model data. It supports OBJ-014 by contributing to a schema-backed piping design model that captures physical design context, design knowledge, constraints, assumptions, and analytical transformation traceability. Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014 and DEL-13-03 rows.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Validate available knowledge only | Treat available design knowledge and explicit constraints as the validation surface. Do not infer hidden owner standards or protected code criteria. | `_CONTEXT.md`; SOW-068; `docs/CONTRACT.md` OPS-K-AGENT-1 |
| Missing data is a finding | Missing solve-required or rule-check-required values must become explicit diagnostics/findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3 |
| Preserve provenance | Where records include source/provenance, diagnostics should preserve reviewable source visibility through `source_references` and affected object references. Broader GUI presentation remains `TBD`. | SOW-068; `docs/SPEC.md` sections 1 and 4; `core/constraints/validation/engine.py` |
| Keep authority boundaries visible | Constraint validation is not professional approval, code compliance, certification, sealing, or owner-standard interpretation. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 4.3 |
| Keep public artifacts clean | Do not copy protected standards text, protected tables, proprietary values, private owner data, or code-specific defaults into public fixtures, messages, or docs. | `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |
| Use schema-first boundaries | The current implementation exposes a deterministic Python diagnostic dict shape and `ValidationResult.to_dict()`. Application runtime/result-envelope integration remains `TBD`. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` sections 1 and 9; `core/constraints/validation/engine.py` |

## Considerations

- The validation engine consumes supplied constraint and optional design-knowledge mapping envelopes. Current upstream DEL-13-02 evidence defines the constraint schema/provenance model; the local `Dependencies.csv` remains an evidence surface and is not edited by this refresh.
- Constraint categories named by SOW-068 are scope categories, not complete engineering acceptance criteria. Any concrete clearance, slope, vent, drain, route, or support-zone value must come from user/project/private sources or later lawful public fixtures.
- Provenance-aware messages can reveal where a value came from or that source evidence is missing, but they cannot create redistribution rights or legal clearance for protected data.
- Deterministic ordering, stable `CV-*` diagnostic codes, and replayable tests now exist in the Python validation slice. Localization/message cataloging and release-grade diagnostic policy remain `TBD`.
- The implementation validates represented conflict classes and referenced data availability; it does not perform full geometric conflict solving, route intersection, owner criteria/rule evaluation, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, or human acceptance.

## Trade-offs

| Trade-off | Conservative direction |
|---|---|
| Helpful defaults vs. engineering safety | Prefer explicit missing-data findings over defaults. |
| Broad validation vs. source authority | Validate only what the available design knowledge and accepted schemas can support. |
| User-facing clarity vs. protected-content risk | Explain the missing or conflicting condition without quoting protected standards or private owner criteria. |
| Strict blocking vs. reviewable warning | Use the implemented `blocking`, `warning`, and `info` severities as current diagnostic evidence; release policy and human acceptance remain outside this module. |
| Early implementation detail vs. architecture stability | Treat the current Python module/API and diagnostic fields as implementation evidence, while keeping formal runtime integration and release architecture decisions human-gated. |

## Examples

Executable invented fixtures exist in `tests/test_constraint_validation.py` for deterministic validation, missing-data, unit, provenance, and authority-boundary checks. Public documentation examples remain `TBD`; any later published examples must use invented or otherwise permitted data with provenance/review status.

## Conflict Table (for human ruling)

No cross-source conflicts were identified in this evidence refresh. Remaining `TBD` items are bounded deferrals: localization, full geometric conflict solving, owner criteria/rules, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, and human acceptance.
## D-41 R5 T2C PDU-023 Boundary

Schema capability is not runtime integration. Preserve the absent application-service/result-envelope home as a held residual.
