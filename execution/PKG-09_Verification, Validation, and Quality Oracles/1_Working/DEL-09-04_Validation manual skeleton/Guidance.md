# Guidance: DEL-09-04 Validation manual skeleton

## Purpose

This guidance explains how to interpret the validation manual skeleton without blurring the OpenPipeStress authority boundary. The manual is intended to organize verification and validation evidence for software quality. It is not an engineering seal, code-compliance ruling, or project-specific acceptance record.

## Principles

| Principle | Guidance |
|---|---|
| Keep verification narrow. | Mechanics verification asks whether implemented mechanics match declared benchmark problems within declared tolerances. It does not decide whether a real project complies with a code. |
| Keep validation contextual. | Validation asks whether the software workflow is fit for its intended support role, including warnings, reports, missing-data behavior, and reproducibility. |
| Keep rule checks user-owned. | Rule packs use user-supplied or lawful private design-basis data. A software pass/fail result is a computation under that rule pack, not professional authentication. |
| Keep professional reliance human-owned. | A competent human must accept the model, data, assumptions, rule basis, limitations, and report before project reliance. |
| Keep public examples clean. | Public examples must be original, public-domain, or permissively licensed. Protected standards examples and commercial benchmark files are excluded unless rights are documented. |
| Keep gaps visible. | Missing benchmark evidence, missing provenance, open risks, and unresolved source questions should remain explicit `TBD` or open issues. |

## Considerations

The validation manual should be useful to maintainers, users, and reviewers, but each audience reads it with different authority:

- Maintainers use it to decide whether software evidence is adequate for a release label.
- Users use it to understand tested behavior, limitations, and evidence sources.
- Reviewers use it to check traceability, reproducibility, and data-boundary conformance.
- Professionals use it only as decision-support context, not as a substitute for judgment.

The manual should avoid embedding protected standards text or code-derived examples. When a future benchmark needs a comparison basis, the source must be public, original, or permissively licensed. If a private licensed basis is used by a user, it belongs in private project records and should not be copied into public artifacts.

## Trade-offs

| Trade-off | Preferred handling |
|---|---|
| Specificity vs protected content risk | Prefer public mechanics statements and provenance over copying code language or examples. |
| Automation vs human judgment | Automate repeatable checks, but keep professional acceptance as a human decision. |
| Completeness vs false precision | Use `TBD` for unproduced evidence rather than implying benchmark coverage that does not exist yet. |
| Release readiness vs project reliance | Release gates describe software quality only; project reliance requires separate professional review. |

## Examples

Acceptable example slots:

- An original cantilever or frame mechanics benchmark with independent hand calculation notes.
- An invented rule-pack test that demonstrates missing-input behavior without using protected code data.
- A report reproducibility check that verifies hashes, warnings, assumptions, and rule-pack checksum fields.

Excluded example slots:

- Copied protected standards tables, figures, examples, equations, or commentary.
- Commercial software benchmark files without documented redistribution rights.
- Vendor or owner data without documented permission for public redistribution.
- Any example implying software certification, engineering seal, official endorsement, or automatic code compliance.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None | No source conflict identified in setup pass. | Not applicable | Not applicable | Not applicable | Not applicable | TBD |
