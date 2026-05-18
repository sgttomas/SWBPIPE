# Guidance: DEL-06-05 Invented non-code example rule pack

## Purpose

This deliverable prepares the governed local setup record for a future invented rule-pack example. Its value is to show what an example must say and avoid before any public example content is placed in repo-level paths.

## Principles

| Principle | Guidance |
|---|---|
| Invented-only public content | Use original, artificial, non-engineering values only. When in doubt, use `TBD` and document the missing source. |
| Code-neutral example posture | Demonstrate the rule-pack mechanism without encoding a real code, owner specification, or proprietary design basis. |
| Professional boundary | A rule-pack check is software computation using user data. It is not professional approval, authentication, sealing, certification, or a reliance decision. |
| Provenance first | Every value class in a future example should state whether it is original/invented, user-supplied private, licensed, or unknown. |
| No silent defaults | Missing solve-required or rule-check-required information is a finding, not a placeholder to hide with a plausible number. |
| Local setup only | This run documents the example concept inside the deliverable folder and does not write the repo-level example file. |

## Considerations

The future example will sit near a sensitive boundary. A useful demonstration should be concrete enough to exercise rule-pack shape, notices, provenance fields, required inputs, and pass/fail mechanics. It must not be concrete in a way that resembles a copied standard, an owner design basis, or a real project criterion.

The safest setup posture is:

- keep formula grammar, evaluator semantics, and checksum generation `TBD` until the schema and evaluator deliverables authorize them;
- use invented labels such as "demo ratio" or "training limit" only as non-engineering concepts, not as real stress criteria;
- describe warnings and notices more strongly than values;
- avoid referencing any standards-body clauses, examples, equations, or tables;
- keep professional review required for any real project use.

## Trade-offs

| Trade-off | Preferred handling |
|---|---|
| Realism vs. protected-content risk | Prefer artificial values and clear notices over realistic code-like examples |
| Demonstration usefulness vs. schema uncertainty | Document expected sections now and keep schema-specific details `TBD` until `DEL-06-01` resolves them |
| Public example availability vs. private design bases | Public examples remain invented; user-owned design bases stay private unless intentionally contributed with documented rights |
| Pass/fail display vs. professional reliance | A future demo may show a user-rule result label, but it must also show that professional reliance requires competent human review |

## Examples

Permitted notice pattern for a future example:

> This invented sample is for software demonstration only. It is not an engineering design basis, not a standards interpretation, and not suitable for professional reliance.

Permitted placeholder pattern:

```yaml
source_notice: "Original invented demonstration data only"
redistribution_status: "public_permissive"
checksum: "TBD until concrete payload is generated"
required_inputs: []
checks: []
report_notice: "Demonstration output is not professional approval."
```

The placeholder above is not a production rule pack. It is a setup illustration of safe fields and notices.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None | No unresolved source conflict identified during P1/P2 setup drafting. | N/A | N/A | N/A | N/A | N/A |
