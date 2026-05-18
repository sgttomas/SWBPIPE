# Guidance: DEL-11-04 Invented educational example models

## Purpose

Invented educational examples let future users and contributors inspect OpenPipeStress mechanics and rule-pack workflow boundaries without importing protected standards data or implying engineering reliance.

## Principles

1. Public examples are invented, original, and clearly non-code.
2. Mechanics-only examples demonstrate solver workflow and reproducibility, not code acceptance.
3. Fake-rule-pack examples demonstrate missing-data and rule-check plumbing, not real design rules.
4. Every future example must carry unit and provenance information.
5. Missing or unresolved data remains visible as `TBD` or a finding.
6. Software examples never certify, approve, seal, or declare professional code compliance.

## Considerations

Mechanics-only examples should remain simple enough to audit while still showing the intended data flow: model identity, units, nodes/elements or comparable centerline entities, supports, load cases, diagnostics, results, and reproducibility metadata. Any future numeric values should be toy values created for demonstration and should not be copied from standards, vendor catalogues, owner specifications, or commercial software examples.

Fake-rule-pack demonstrations should show that user-rule checks are separate from mechanics solves. They can use fictional rule names, fictional required inputs, fake threshold labels, and placeholder outcomes. They must not teach or approximate real code formulas, material allowables, stress categories, SIF/flexibility factors, or load-combination rules.

Testing use must be described carefully. A public invented example can support regression or verification of software behavior, but it is not evidence that any real piping system is safe, compliant, or professionally approved.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Educational clarity vs. realism | Prefer auditability and boundary clarity over realistic engineering values. |
| Reusable fixture vs. hidden default | Make invented provenance explicit and keep unresolved data visible. |
| Mechanics demonstration vs. rule-check demonstration | Keep mechanics-only examples separate from fake-rule-pack examples so result status is not confused. |
| Public convenience vs. IP protection | Do not include protected examples or commercial software comparisons for convenience. |
| Validation support vs. compliance implication | Use examples for software behavior checks only; keep professional reliance outside software authority. |

## Example Concepts

Future work may define non-file concepts such as:

- a mechanics-only centerline model that demonstrates unit-aware loads and deterministic result review;
- a support/diagnostic model that demonstrates visible missing or invalid inputs;
- a fake-rule-pack demonstration that shows required fictional inputs, checksum metadata, and blocked rule-check status when those inputs are absent;
- a tutorial path that explains how to inspect provenance and limitations before running a demonstration.

These concepts are not actual model files in this setup session. They are constraints for future artifacts.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| CF-DEL-11-04-001 | The register anticipates future files under `examples/models/invented/*` and tutorials, but this sealed setup brief forbids creating actual example files or tutorials outside this deliverable. | `docs/_Registers/Deliverables.csv` row DEL-11-04 | User sealed brief for DEL-11-04 setup | Datasheet Construction; Specification Scope; Procedure Steps | Treat this run as setup/document production only and leave external artifacts for a later authorized task. | TBD |
| CF-DEL-11-04-002 | Exact future model file format and fake-rule-pack schema are not available in this deliverable. | `docs/SPEC.md` repository target and rule-pack section | DEL-02 and DEL-06 dependencies not yet accepted as issued artifacts | Specification External Inputs; Procedure Prerequisites | Keep future model and rule-pack materialization as `TBD` until schema deliverables are available. | TBD |

## Non-Reliance Notice

Any future example produced from this setup must say, in substance, that it is invented for education/testing, is not a design basis, is not a standards example, and does not certify or approve engineering work.
