# Guidance: DEL-08-05 Report protected-content linter

## Purpose

The protected-content linter exists to reduce the risk that public report templates or examples accidentally carry content the OpenPipeStress public repository must not redistribute. It supports the IP/data boundary and reproducible professional review, but it remains a heuristic guard plus review workflow, not a legal opinion or engineering acceptance decision.

## Principles

| Principle | Guidance |
|---|---|
| Protect public artifacts | Default scanning should target public report templates/examples and configured public report surfaces. |
| Preserve private responsibility | User-private report templates, licensed standards quotes, owner design bases, and private rule packs remain user responsibility unless the user explicitly opts into local scanning. |
| Use synthetic fixtures | Future tests should use invented markers and safe placeholders rather than copied standards/code/vendor content. |
| Flag, do not certify | Findings should route suspected content to quarantine/review. A clean scan should not be described as legally sufficient. |
| Allow safe metadata | Rule-pack IDs, versions, checksums, source notices, redistribution status, and required-input status can be safe when they do not expose protected content. |
| Keep professional boundary visible | Public report text should avoid claims of certification, sealing, approval, authentication, or automatic code compliance. |
| Keep diagnostics reproducible | Finding output should be stable, traceable to files/locations, and suitable for CI and review records. |

## Considerations

The linter cannot know whether every phrase, formula, or table is protected. It should combine conservative heuristic checks, required metadata, safe fixture policy, and human/legal review routing. False positives are acceptable when they protect the public repository boundary; false negatives are a residual risk that review must address.

Public report templates should prefer placeholders, invented values, and metadata references over copied explanatory text or engineering formula bodies. User-private report templates may include licensed or proprietary content under user control, but the public project should not assume those templates are redistributable.

The future implementation should avoid a fixture strategy that recreates the very material it is intended to detect. Synthetic markers, invented table structures, and non-code phrases are preferable to copied source examples.

## Trade-offs

| Topic | Conservative direction |
|---|---|
| Detection breadth vs false positives | Prefer review-blocking warnings for plausible protected-content risk in public artifacts. |
| Pattern specificity vs protected fixture risk | Use safe synthetic markers, metadata checks, and structural heuristics instead of copied protected examples. |
| CI automation vs legal judgment | CI can block or warn based on policy, but legal/provenance disposition remains a human process. |
| Public template convenience vs IP boundary | Public templates should be sparse and metadata-driven where protected content might otherwise be copied. |
| User-private template support vs privacy | Local opt-in scanning can help users, but default behavior must not transmit or commit private content. |

## Examples

Safe public examples may use:

- invented placeholder notices;
- synthetic rule-pack IDs and checksums;
- fake non-code formula markers that are explicitly marked as invented;
- empty or schematic table headings that do not reproduce protected table content;
- professional-review notices that describe decision support without claiming approval.

Unsafe public-example patterns include:

- copied standards-body explanatory text;
- copied standards or vendor tables;
- copied proprietary formulas or formula commentary;
- private owner/rule-pack text committed as a public template;
- report wording that states the software certifies or declares code compliance for reliance.

These examples are categorical only. They intentionally do not include protected source text or copied proprietary values.

## Conflict Table (for human ruling)

No source conflicts were identified in the local reference set for this setup run.

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | None identified | N/A | N/A | N/A | N/A | N/A |

## Open Questions

| ID | Question | Current disposition |
|---|---|---|
| OQ-08-05-001 | Exact linter implementation language/library and integration point. | TBD; future implementation-level decision. |
| OQ-08-05-002 | Exact public template/example path list to scan by default. | TBD; should be resolved when report template locations exist. |
| OQ-08-05-003 | Exact severity policy for fail, warn, quarantine, and review-required outcomes. | TBD; must be confirmed before CI guard implementation. |
| OQ-08-05-004 | Exact diagnostic schema fields for linter output. | TBD; align with AB-00-06 result-envelope/diagnostic basis. |
| OQ-08-05-005 | Human/legal review ownership for protected-content findings. | TBD; governance process must record disposition authority. |

