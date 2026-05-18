# Guidance: DEL-01-04 Professional responsibility and product-claims policy

## Purpose

This guidance explains how to draft professional-boundary and product-claims language for OpenPipeStress without overstating what software, agents, rule packs, reports, or releases can warrant.

The central boundary is: OpenPipeStress may assist analysis, expose evidence, compute mechanics, evaluate user-defined rule packs, and produce auditable reports; it does not replace competent professional judgment.

## Principles

| Principle | Guidance |
|---|---|
| Use precise authority language | Say what the software computes or records. Do not say it certifies, seals, approves, authenticates, or declares professional/code compliance for reliance. |
| Separate statuses | Keep `MECHANICS_SOLVED`, `USER_RULE_CHECKED`, `HUMAN_REVIEW_REQUIRED`, and `HUMAN_APPROVED_FOR_PROJECT` distinct. |
| Preserve human decision rights | A human project authority may accept a development artifact; a competent engineering professional may accept a project calculation. These are different gates. |
| Make missing data visible | Missing code data, provenance, assumptions, and warnings are findings, not silent defaults. |
| Keep public examples safe | Public notices and examples use original, invented, or permissively sourced content only. |

## Considerations

- Product language should prefer "decision support", "mechanics result", "rule-pack result", "diagnostic", "warning", "provenance", and "human review required" over conclusory compliance language.
- Report notices should be attached to the report output and not hidden only in documentation.
- A release maturity claim can describe validation evidence and limitations, but it cannot imply approval for a specific piping design.
- Legal or jurisdiction-specific professional-practice wording is `TBD` until a qualified human authority provides it.
- Exact repo-level notice wording is `TBD` in this setup artifact. The local kit can state constraints and candidate checks, but it cannot issue project policy.

## Trade-offs

| Trade-off | Preferred Resolution |
|---|---|
| Clear marketing language vs. professional boundary precision | Prefer precision. Ambiguous claims create unacceptable reliance risk. |
| Automated report convenience vs. human review | Reports may assemble evidence, but reliance remains a human decision. |
| User-rule pass/fail vs. code compliance | Treat user-rule pass/fail as a computational result using user-supplied inputs, not a professional compliance declaration. |
| Draft agent output vs. accepted governance | Treat agent output as draft/proposal until human acceptance. |
| Local setup text vs. repo-level policy | Keep local setup text as draft evidence until an authorized repo-level edit and human gate accept it. |

## Examples

| Safer Draft Claim | Avoid |
|---|---|
| "The report includes mechanical results, user-rule check results, warnings, assumptions, provenance notes, and limitations for competent human review." | "The report certifies code compliance." |
| "The rule pack evaluated user-defined checks using the recorded inputs." | "The software approved the design." |
| "A competent human must review the model, data, rule basis, warnings, and report before professional reliance." | "No further engineering review is required." |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None | No source conflict identified in accessible setup materials. | TBD | TBD | TBD | TBD | TBD |
