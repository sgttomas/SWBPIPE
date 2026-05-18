# Guidance: DEL-11-02 Developer guide for solver and rule packs

## Purpose

The developer guide should give contributors enough structure to extend OpenPipeStress without weakening its governing boundaries. It should explain how the open solver mechanics, user-supplied rule packs, tests, reports, and adapter boundaries fit together, while making clear that protected standards data and professional approval remain outside the public software authority.

## Principles

| Principle | Guidance |
|---|---|
| Open mechanics first | Describe the solver as an auditable mechanics engine for global centerline/frame analysis. Avoid wording that makes the solver appear to implement a proprietary code. |
| Code-neutral rule checks | Present rule packs as user-owned design-basis artifacts. Public content may define schemas and evaluator mechanics, not protected rule content. |
| No silent defaults | Contributor-facing docs should remind implementers that missing solve-required or rule-check-required values become explicit diagnostics, not hidden assumptions. |
| Unit discipline | Any developer path touching model data, rule variables, imports, exports, reports, or tests must preserve unit awareness and dimensional checks. |
| Provenance discipline | Material, component, rule, allowable, SIF, flexibility, report, and public data values require source/provenance and redistribution status. |
| Test before reliance | Solver and rule-engine changes need deterministic tests before release use; documentation should state the expected evidence families. |
| Human authority | The guide must not suggest that software, maintainers, or agents certify, seal, approve, authenticate, or declare project-specific engineering compliance. |

## Considerations

### Solver Architecture

The guide should explain the solver at a level suitable for contributors:

- domain objects and unit-aware model data;
- six degree-of-freedom node semantics;
- local/global element orientation concepts without protected formulas;
- stiffness assembly and sparse solve boundary at a conceptual level;
- loads, stress recovery, result envelopes, and diagnostics;
- nonlinear support status reporting where applicable;
- interaction with rule packs through mechanical outputs and required-input mappings.

Solver documentation should not include protected code equations or code-specific acceptance categories unless those are supplied by the user in private rule packs.

### Rule-Pack Architecture

The guide should explain the rule-pack artifact as a data contract, not as bundled standards content. The safe baseline includes identity, version, source notice, redistribution status, checksum, required input declarations, variables, checks, unit metadata, and report notice fields.

The guide should make the evaluator boundary explicit: rule expressions are declarative, sandboxed, unit-aware, deterministic, and unable to execute arbitrary code. Exact expression grammar and implementation library remain `TBD` until a sealed implementation deliverable and human approval resolve them.

### Test Discipline

The guide should point contributors to evidence categories rather than rely on trust:

- unit/schema tests for domain objects and units;
- deterministic solver benchmarks for frame mechanics and transforms using rights-cleared fixtures;
- stress recovery and load application regression tests;
- nonlinear support convergence and active-state traces;
- rule-pack required-input, unit-mismatch, unsafe-expression, and invented-example tests;
- report reproducibility and checksum tests;
- protected-content and provenance gates for public docs, examples, fixtures, and reports.

### Contribution Boundaries

Developer documentation should give contributors simple stop rules:

- do not contribute protected standards text, tables, figures, examples, protected code-derived formulas, protected dimensional tables, or proprietary vendor data;
- do not add private project data, owner standards, company rule packs, or private component/material libraries to public examples;
- quarantine and escalate suspected protected content;
- label unknowns as `TBD`;
- label inferences as `ASSUMPTION` and proposals as `PROPOSAL`;
- keep mechanics outputs, user-rule checks, and human professional acceptance separate.

## Trade-offs

| Trade-off | Preferred direction |
|---|---|
| Helpful examples vs protected-content risk | Use invented non-code examples or public/permissive mechanics examples only. |
| Detailed formulas vs IP boundary | Explain architecture and verification expectations without copying protected standards formulas or tables. |
| Fast contribution vs review discipline | Require provenance, test evidence, and protected-content review before accepting public data or rule-pack examples. |
| Flexible plugins vs no-bypass governance | Allow extension points only when units, provenance, diagnostics, sandboxing, privacy, and report controls remain mandatory. |
| Mechanics solve vs rule check | Let the solver compute open mechanics; keep acceptability logic in user/private rule packs. |

## Examples

Safe example themes for the future guide:

- a non-code invented rule-pack field map that uses placeholder names and no engineering values;
- a mechanics-only test fixture described as an original/public-domain frame case, with expected values held in validation evidence rather than copied from standards;
- a contributor checklist for adding a solver feature without changing rule-pack semantics;
- a quarantine example that shows process fields and decisions without reproducing suspected protected content.

Unsafe example themes:

- copied standards text, tables, clause examples, or formulas;
- material allowable tables or dimensional tables copied from standards;
- private company rule packs, owner standards, project models, or commercial software benchmark files;
- report snippets that imply official compliance, certification, approval, or professional sealing by the software.

## Open Decisions

| Decision | Status |
|---|---|
| Exact solver numerical library | `TBD` |
| Rule expression grammar/library | `TBD` |
| Exact dependency versions | `TBD` |
| CI provider, coverage thresholds, and performance thresholds | `TBD` |
| Physical project package/container | `TBD` |
| License and contributor certification mechanism | `TBD` until human project authority records it |

## Conflict Table (for human ruling)

No source conflicts were identified during setup. If later guide drafting discovers conflict between architecture basis, implementation practice, legal/data-boundary policy, or validation requirements, record it here rather than silently resolving it.

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| TBD | None recorded. | TBD | TBD | TBD | TBD | TBD |

