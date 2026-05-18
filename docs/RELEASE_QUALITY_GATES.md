---
doc_id: OPS-RELEASE-QUALITY-GATES
doc_kind: governance.release_quality_gates
status: draft
created: 2026-05-04
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: strategy
    to: OPS-VALIDATION-STRATEGY
  - rel: implements
    to: DEL-09-05
---

# Release Quality Gates

## 1. Purpose

This checklist routes release-impacting changes through software-quality gates.
It covers solver changes, rule-engine changes, GUI releases, report-template
releases, and mixed changes.

The checklist is not a professional engineering approval, code-compliance
ruling, legal opinion, release publication authorization, or CI implementation.
It defines evidence that maintainers must collect or explicitly waive before a
release label is considered.

## 2. Gate Routing

Route every release-impacting change to all applicable gate families.

| Change type | Gate family | Examples |
|---|---|---|
| Solver mechanics, loads, stress recovery, units, diagnostics, result envelopes | Solver gate | element stiffness, support behavior, load-case algebra, stress output, warning semantics |
| Rule schema, expression evaluation, completeness, rule-pack lifecycle | Rule-engine gate | required inputs, unit mismatch, sandbox behavior, checksum/provenance |
| Model editing, solve workflow, warning display, results viewing | GUI gate | missing-data UX, nonlinear warning visibility, result state presentation |
| Public report templates, export fields, notices, manifest data | Report-template gate | warning inclusion, reproducibility, protected-content lint, provenance disclosure |
| Any change spanning families | Mixed gate | solver output changing reports, GUI displaying new rule states |

Mixed changes run the union of applicable gates unless the human release
authority records a waiver and risk disposition.

## 3. Common Required Evidence

Every gate record must include:

- change identifier, scope, affected packages, and owner;
- commands run and complete pass/fail results;
- source artifacts and generated outputs reviewed;
- benchmark source/provenance and redistribution status where examples are
  public;
- known limitations, open risks, and unresolved `TBD` items;
- protected-content, private-data, and real-secret scan disposition;
- human governance acceptance or waiver record;
- statement that release evidence is software-quality evidence only.

## 4. Solver Gate

Use this gate for changes to solver mechanics, load handling, stress recovery,
units, diagnostics, or result envelopes.

Required evidence:

- applicable mechanics benchmarks pass;
- applicable stress-recovery benchmarks pass;
- applicable nonlinear support regression checks pass;
- unit/schema checks pass where units or serialization are touched;
- diagnostics and warning/result-envelope behavior are tested where affected;
- tolerance source is named, or the tolerance remains `TBD`;
- numerical warnings, non-convergence, and missing result states remain visible.

Not permitted:

- selecting final public release tolerances without a human governance record;
- claiming code compliance, certification, or professional acceptance;
- using protected standards examples or commercial benchmark files without
  documented rights.

## 5. Rule-Engine Gate

Use this gate for rule schemas, expression evaluation, rule-pack completeness,
rule-pack lifecycle, or rule-result presentation.

Required evidence:

- deterministic evaluator tests pass for invented examples;
- required-input completeness checks pass;
- unit-aware rule checks reject or flag incompatible units;
- sandboxing and arbitrary-code-execution boundaries are verified where touched;
- rule-pack checksum and provenance fields are preserved;
- public fixtures use invented or redistributable data only;
- pass/fail states are described as user-rule computations, not professional
  authentication.

## 6. GUI Gate

Use this gate for releases that affect model editing, property inspection,
missing-data behavior, solve execution UX, warning display, or result viewing.

Required evidence:

- missing solve data, missing rule-check data, and missing provenance are
  visible to users;
- warnings, assumptions, nonlinear uncertainty, and result-envelope states are
  not collapsed into generic success states;
- workflow tests or screenshots are recorded where practical;
- private data stays local and is not sent to unapproved services;
- GUI labels avoid compliance, certification, endorsement, and professional
  approval claims.

Final browser/device matrix, accessibility threshold, and screenshot tooling are
`TBD` until governed by a release-policy decision.

## 7. Report-Template Gate

Use this gate for public report templates, report notices, export fields,
manifest output, and report-facing wording.

Required evidence:

- report reproducibility checks pass where the generator is touched;
- audit manifest and checksum fields remain stable or changes are explained;
- warnings, assumptions, limitations, and rule-pack provenance remain visible;
- protected-content lint passes for public examples and templates;
- professional-boundary notices are present;
- private project data is not introduced into public fixtures or templates.

## 8. Release Labels

The minimum engineering-beta condition in `docs/VALIDATION_STRATEGY.md` remains
the governing release-label floor: required mechanics benchmarks, stress
benchmarks, rule-pack missing-input tests, report reproducibility tests, and
protected-content lint must pass where applicable, and open risks must be listed
and accepted by human maintainers.

Release labels communicate software maturity and validation evidence. They do
not approve a real piping calculation, authenticate a user rule pack, or replace
professional review.

## 9. Waivers And Risk Disposition

A waiver must be explicit. It must name:

- waived gate item;
- reason the gate cannot or should not run;
- affected release scope;
- compensating evidence, if any;
- known risk and limitation text;
- human governance decision record.

No waiver may authorize protected-content copying, private-data exposure,
certification claims, code-compliance claims, or professional reliance claims.

## 10. Open Decisions

- TBD: final numerical tolerance policy for solver, stress, and nonlinear
  benchmarks.
- TBD: performance thresholds and permitted variance policy.
- TBD: coverage thresholds for Rust, Python, GUI, validation, and
  protected-content gates.
- TBD: CI provider, release matrix, signing, release attestation, and maintainer
  quorum.
- TBD: exact automation owners, gate owners, waiver approver roles, and command
  names.
- TBD: release-note format for known limitations and accepted risks.
