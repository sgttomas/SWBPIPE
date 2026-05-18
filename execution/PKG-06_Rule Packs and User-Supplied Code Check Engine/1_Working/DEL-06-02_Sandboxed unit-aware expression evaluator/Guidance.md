# Guidance: DEL-06-02 Sandboxed unit-aware expression evaluator

## Purpose

This deliverable exists to make user-defined rule-pack checks evaluable without turning rule packs into arbitrary executable code. The evaluator boundary should let users apply their own design bases to solver results while preserving unit safety, explicit missing-data findings, protected-data boundaries, and professional-responsibility limits.

## Principles

| Principle | Guidance |
|---|---|
| Declarative before executable | Treat rule packs as data plus expressions. Do not permit host-language code, imports, reflection, filesystem access, network access, process execution, or hidden side effects. |
| Unit safety is structural | Unit compatibility is not a UI nicety. It is part of evaluator correctness and should fail visibly when incompatible or missing. |
| User-owned code data stays user-owned | Public setup and examples can define structure and invented checks only. Licensed formulas, protected interpretations, and owner design bases belong in private user rule packs. |
| Findings over defaults | Missing variables, missing rule inputs, incompatible dimensions, and unsupported constructs should produce diagnostics. They should not be guessed or defaulted. |
| Rule checked is not code compliant | `USER_RULE_CHECKED` and `USER_RULE_FAILED` are software computations using user data. Professional reliance still requires competent human review. |

## Considerations

- The evaluator is security-sensitive and numerically important, so the large context envelope is justified, but expansion beyond sandboxing/unit-awareness should be split or escalated.
- The exact expression grammar/library is deliberately unresolved. A future implementation brief should record the selected option, rejected options, threat model, and validation evidence.
- Unit algebra should coordinate with the unit-system contract rather than duplicate a second unit model.
- Rule-pack schema, required-input completeness, private lifecycle/checksum handling, and invented examples are sibling deliverables. This deliverable should not absorb those scopes.
- Diagnostics should be deterministic and suitable for result envelopes, reports, and review without exposing protected formulas in public examples.
- Public test fixtures should use invented or original examples only.

## Trade-offs

| Trade-off | Setup guidance |
|---|---|
| Expressiveness vs sandbox strength | Prefer a small declarative surface that can be validated and audited. Do not trade sandbox guarantees for convenience. |
| Unit richness vs implementation scope | Require dimensional checks, but leave final quantity representation and tolerance choices to the future sealed implementation brief. |
| Detailed diagnostics vs protected-content leakage | Diagnostics should identify missing inputs and rule-check findings without reproducing licensed formulas or protected standards language in public artifacts. |
| Plugin flexibility vs governance | Plugins and adapters may provide data, but they cannot bypass validation, sandboxing, provenance, diagnostics, or public/private boundaries. |

## Examples

No protected formulas or code-derived examples are provided. Acceptable future public examples should use invented non-engineering values and clear notices. Conceptual examples for test planning include:

- A malformed expression is rejected with a deterministic diagnostic.
- A comparison between incompatible dimensions is rejected with a unit-mismatch finding.
- A missing user-supplied variable produces a `RULE_CHECK_BLOCKING` style finding.
- A successful invented check is reported as user-rule checked, not professionally approved.

## Semantic Enrichment Notes

Pass 3 reviewed `_SEMANTIC_LENSING.md` and incorporated the warranted setup improvements as bounded notes:

| Lensing item | Disposition |
|---|---|
| Unsafe expression and bypass test coverage | Added to `Specification.md` verification and `Procedure.md` checks. |
| Diagnostic-code taxonomy | Retained as `TBD` in `Specification.md` open decisions. |
| Grammar/library decision | Retained as `TBD` and future human architecture decision. |
| Vocabulary normalization | Standardized on evaluator, expression, variable binding, rule-pack check, user-rule checked, and human review boundary. |
| Future interface handoff | Recorded as an implementation-stage dependency/proposal rather than a setup implementation step. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| OI-006 | Expression grammar/library is required before implementation but remains unresolved. | `docs/_Decomposition/SOFTWARE_DECOMP.md` open issue OI-006 | `docs/SPEC.md` section 6 requires sandboxed unit-aware evaluator | `Specification.md#Open-Decisions`; `Procedure.md#Prerequisites` | Future human architecture decision should seal grammar/library before implementation. | TBD |
