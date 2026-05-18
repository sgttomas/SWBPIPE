# Guidance: DEL-06-03 Required-input completeness checker

## Purpose

The required-input completeness checker protects the boundary between an open mechanics solve and a user-defined rule-pack check. A mechanics result may exist while a rule-check result is blocked because the user has not supplied required code-specific or project-specific data.

## Principles

1. Missing rule-check data is a finding.
2. Code-specific defaults are not invented by the public project.
3. User rule packs are user-owned or private design-basis artifacts unless explicitly contributed with redistribution rights.
4. The checker gates software status only; it does not certify code compliance or professional acceptability.
5. Provenance is part of completeness whenever reliance on the supplied value may affect engineering judgment.

## Considerations

The checker should eventually be strict about required-input declarations while staying neutral about the content of private rule packs. It can say that a required value is absent, unit-incompatible, unprovenanced, or unresolved. It must not embed standards-body tables, quote protected clauses, derive protected formulas, or silently choose engineering values.

Completeness should be machine-checkable from declarative metadata. If a future rule pack needs code-specific formulas or allowables, those belong in the user's private rule pack or another lawfully redistributable source with provenance.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Strict blocking vs. user convenience | Prefer explicit blocking for missing declared required inputs. Convenience must not create hidden defaults. |
| Provenance warning vs. missing input | Distinguish "value absent" from "value present but source weak or unknown." |
| Public examples vs. realistic code data | Use invented non-code examples in public artifacts. Do not approximate protected standards data. |
| Software finding vs. professional judgment | Report a machine finding and keep professional acceptance outside the rule engine. |

## Examples

This setup artifact intentionally includes no engineering example values. Later public tests should use invented/non-code placeholder rule packs that demonstrate missing-input behavior without resembling protected standards content.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| CF-DEL-06-03-001 | Exact future expression grammar/library for rule-pack declarations is not selected. | `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-006 | `docs/SPEC.md` rule-pack evaluator section | Specification External Inputs; Procedure Prerequisites | Treat grammar/library as `TBD` and do not encode executable rules in setup artifacts. | TBD |
