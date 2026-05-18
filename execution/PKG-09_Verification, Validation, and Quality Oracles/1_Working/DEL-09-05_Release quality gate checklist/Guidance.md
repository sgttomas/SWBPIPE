# Guidance: DEL-09-05 Release quality gate checklist

## Purpose
This deliverable gives future release work a bounded checklist for release quality gates. It should make gate routing, required evidence, data-boundary checks, and human governance decisions visible before solver, rule-engine, GUI, or report-template changes are promoted.

## Principles
- Treat release gates as software-quality and governance controls, not professional engineering approval.
- Use deterministic evidence where available: tests, validation scripts, benchmark comparisons, report reproducibility checks, protected-content lint, and review records.
- Keep mechanics verification separate from validation of the workflow and separate again from user-rule checks or professional reliance.
- Require provenance for benchmark sources, public examples, rule-pack references, and report-facing data.
- Mark unresolved thresholds and release-authority choices as `TBD` until a human ruling exists.
- Block or escalate suspected protected content instead of paraphrasing, normalizing, or incorporating it.

## Considerations
The gate families overlap. A report-template release that changes warning presentation may also need GUI or status-semantics evidence. A solver change that affects result envelopes may need report reproducibility evidence. Mixed changes should run the union of applicable gate families, with duplicated evidence referenced once in the release bundle.

The checklist should be CI-friendly, but this setup deliverable does not create or alter CI workflows. Future CI implementation should consume this checklist as requirements input and keep final threshold choices visible for human approval.

## Release Label Rationale
Release labels communicate software maturity and validation evidence. They are not professional engineering approval of any piping calculation, model, rule-pack result, or report. The minimum engineering-beta conditions in `docs/VALIDATION_STRATEGY.md#4` should therefore be read as release-governance criteria: required benchmarks and checks pass, protected-content lint passes where applicable, and open risks are listed and accepted by human maintainers.

## Trade-offs
| Choice | Benefit | Risk | Guidance |
|---|---|---|---|
| Conservative gate routing | Reduces under-tested release paths | May run more checks than a narrow change strictly needs | Prefer conservative routing until package-specific ownership and CI thresholds are approved |
| Thresholds marked `TBD` | Preserves human authority and avoids invented targets | Leaves later implementation work with open decisions | Record the missing decision in the release bundle and route to human maintainers |
| Protected-content lint plus human review | Gives fast feedback while preserving legal/professional judgment | Lint cannot prove legal safety | Treat lint as evidence only, not a legal conclusion |
| Human maintainer acceptance | Records project governance approval | Could be mistaken for engineering approval | Use explicit wording that governance acceptance is not professional acceptance of a piping calculation |

## Examples
| Change example | Gate routing | Notes |
|---|---|---|
| Solver stiffness transform update | Solver gate; report gate if output manifests change | Requires deterministic benchmark/regression evidence and visible numerical-quality diagnostics |
| Rule-pack evaluator parser change | Rule-engine gate; security/privacy gate if sandbox surface changes | Requires sandbox, unit, missing-input, and invented-example evidence |
| Results viewer release | GUI gate; report gate if export/preview changes | Requires warning/status visibility and regression evidence |
| Public report template wording change | Report-template gate | Requires protected-content lint, reproducibility evidence, and professional-boundary notice check |

## Human-Ruling Queue
- TBD: final numerical tolerance policy for solver and stress recovery benchmarks.
- TBD: performance thresholds and permitted variance policy for release gates.
- TBD: coverage thresholds for Cargo, Vitest, Playwright, validation, and protected-content gates.
- TBD: CI provider, release matrix, signing/release attestation process, and maintainer quorum.
- TBD: engineering-beta label policy beyond the minimum conditions in `docs/VALIDATION_STRATEGY.md#4`.
- TBD: exact CI command names, automation owners, gate owners, and waiver approver roles.

## Conflict Table (for human ruling)
No source conflicts were identified during setup. Open decisions are recorded as `TBD` items rather than conflicts.

## Boundaries
This checklist is a draft setup artifact. It does not:
- certify a calculation or model;
- declare code compliance;
- replace competent professional review;
- grant redistribution rights for standards, vendor, owner, or user-private data;
- authorize CI workflow edits or release publication.
