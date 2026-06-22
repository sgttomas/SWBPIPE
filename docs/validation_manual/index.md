---
doc_id: OPS-VALIDATION-MANUAL-SKELETON
doc_kind: governance.validation_manual
status: draft
created: 2026-05-04
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: strategy
    to: OPS-VALIDATION-STRATEGY
  - rel: implements
    to: DEL-09-04
---

# Validation Manual Skeleton

## 1. Authority Boundary

This manual organizes verification and validation evidence for OpenPipeStress
software quality. It does not certify a piping model, decide code compliance,
authenticate a professional judgment, or approve project-specific reliance.

Current authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md`
revision `0.7` and approved `execution/_DAG/DAG-007/` graph coordination
authority. `DAG-007` supplies active dependency context only; it does not
settle lifecycle state, release labels, legal clearance, professional approval,
certification, sealing, authentication, or code-compliance claims.

Use this manual to answer four separate questions:

| Question | Evidence class | Decision owner |
|---|---|---|
| Does the software match a declared mechanics problem within a declared tolerance? | Mechanics verification | Maintainers, using recorded benchmark evidence |
| Does the workflow preserve assumptions, warnings, reproducibility, and user-facing limitations? | Workflow validation | Maintainers and reviewers |
| Did a user-supplied rule pack run deterministically against its stated inputs? | User rule check | User or project authority |
| Is a real project calculation acceptable for reliance? | Professional reliance context | Competent human reviewer |

Software evidence can support the last question, but it cannot replace the
human review of model scope, inputs, assumptions, rule basis, limitations, and
reporting context.

## 2. Evidence States

Manual entries use these states:

| State | Meaning |
|---|---|
| `PLANNED` | Section exists but no accepted evidence is recorded. |
| `DRAFT_EVIDENCE` | Evidence exists but thresholds, provenance, review, or repeatability remain incomplete. |
| `MAINTAINER_REVIEWED` | Maintainers reviewed the evidence for software-quality use. |
| `BLOCKED` | Required evidence cannot be used until a source, provenance, licensing, or technical issue is resolved. |
| `TBD` | A governed decision or evidence record is still missing. |

Final release thresholds, public benchmark acceptance, release labels, and
professional reliance wording remain `TBD` unless a human governance record says
otherwise. GUI validation evidence requirements and validation evidence-bundle
storage also remain `TBD`.

## 3. Manual Section Map

| Section | Evidence class | Current source slots | Required content |
|---|---|---|---|
| Product scope and limitations | Professional reliance context | `docs/PROFESSIONAL_BOUNDARY.md`, `docs/IP_AND_DATA_BOUNDARY.md` | Intended support role, exclusions, private-data boundary, reliance warning, known unsupported cases |
| Solver theory summary | Verification context | `docs/theory/centerline_analysis.md`, solver docs | Stated mechanics assumptions, element families, load handling, nonlinear support limits, result-envelope policy |
| Unit and schema verification | Mechanics verification | schema tests, `core/units/`, `docs/TYPES.md` | Unit dimensions, schema validation, invalid-input behavior, serialization boundaries |
| Element verification | Mechanics verification | `validation/benchmarks/mechanics/` | Frame, straight-pipe, support, thermal, imposed displacement, and transform benchmarks |
| Load and stress recovery verification | Mechanics verification | `validation/benchmarks/stress/`, load and stress modules | Axial, bending, torsion, pressure membrane, stress range, load-case algebra, documented tolerances |
| Nonlinear support verification | Mechanics verification | `validation/benchmarks/nonlinear/` | Active-set behavior, gap/lift-off/friction cases, convergence and non-convergence diagnostics |
| Rule-pack evaluator verification | User rule check | rule schemas, evaluator tests, invented rule packs | Required inputs, unit awareness, sandboxing, deterministic pass/fail status, checksum/provenance |
| GUI workflow validation | Workflow validation | GUI workflow tests and screenshots when available | Missing-data behavior, warning visibility, assumptions, solve status, result state transitions |
| Report reproducibility validation | Workflow validation | report generator, audit manifest, protected-content linter | Stable output, checksums, warning inclusion, provenance disclosure, professional-boundary notice |
| Known limitations and open issues | Cross-cutting | issue records, release notes, `TBD` queue | Missing evidence, unapproved thresholds, source restrictions, model limitations, accepted risks |

## 4. Current Evidence Inventory

| Evidence area | Current repository surface | Manual status |
|---|---|---|
| Mechanics benchmarks | `validation/benchmarks/mechanics/`, `validation/hand_calcs/mechanics/` | `DRAFT_EVIDENCE`; final public benchmark acceptance and release tolerances are `TBD`. |
| Stress recovery benchmarks | `validation/benchmarks/stress/`, `validation/hand_calcs/stress/` | `DRAFT_EVIDENCE`; fatigue, allowable, and release-threshold decisions are `TBD`. |
| Nonlinear support regression | `validation/benchmarks/nonlinear/` | `DRAFT_EVIDENCE`; production release thresholds and external validation claims are `TBD`. |
| GUI workflow validation | GUI workflow tests, screenshots, and interaction evidence when available | `PLANNED`; required evidence type, coverage, and review criteria are `TBD`. |
| Report protected-content lint | `core/reporting/protected_content_linter/` | Draft review evidence for report/public-artifact checks; does not provide legal clearance, professional approval, or code-compliance proof. |
| Release quality evidence | Future `DEL-09-05` release quality gate checklist | `TBD`; this manual may organize evidence inputs but does not settle release labels or release readiness. |
| Validation evidence bundles | Future evidence package location/format | `TBD`; long-term storage, retention, and release attachment policy are unsettled. |
| Professional boundary | `docs/PROFESSIONAL_BOUNDARY.md` | Draft policy surface for reliance wording and release-claim limits. |

This inventory is not complete validation, release readiness, legal clearance,
or professional reliance evidence. It is the starting index for future evidence
review and release-quality decisions.

## 5. Section Requirements

Each manual section must include:

- purpose and evidence class;
- source artifacts and commands used to generate evidence;
- expected behavior and comparison basis;
- tolerance or threshold source, or `TBD` if not governed;
- provenance and redistribution status for public examples;
- known limitations and excluded interpretations;
- review disposition and open risks.

## 6. Hand-Calc Witness Evidence

A formal hand-calc witness is a machine-readable validation evidence artifact.
It records the problem statement, assumptions, units, symbolic relations,
numeric inputs, intermediate quantities, expected results, tolerances or `TBD`
tolerance markers, provenance, redistribution status, and review disposition.
The machine-readable artifact is authoritative for automated comparison,
hashing, reproducibility, and review traceability.

Markdown, MathML, PDF, or other human-facing hand-calc pages are deterministic
renderings generated from the witness artifact. They support inspection and
review, but they are not the source of record when they disagree with the
machine-readable witness.

Hand-calc witnesses must remain limited to open mechanics, invented or cleared
public evidence, and user-supplied private data where applicable. They must not
embed protected standards text, protected tables, proprietary benchmark files,
private rule-pack content, code-specific acceptance criteria, or any statement
that the software certifies, seals, approves, authenticates, or declares code
compliance for project reliance.

## 7. Provenance Rules

Public validation examples must be original, public-domain, or permissively
licensed. Public artifacts must not copy protected standards text, protected
tables, protected examples, proprietary commercial benchmark files, private
owner data, private user models, or vendor data without documented rights.

Private rule packs, private material libraries, owner requirements, and project
models belong in user-controlled project records. They may be referenced in a
private validation package, but they must not be copied into public
OpenPipeStress artifacts.

## 8. Manual Review Checklist

Before a manual section is treated as maintainer-reviewed software evidence,
reviewers check that:

- evidence commands and inputs are recorded;
- expected results are reproducible from repository artifacts or documented
  external sources with redistribution rights;
- generated hand-calc renderings trace back to their authoritative
  machine-readable witness artifacts;
- units and assumptions are explicit;
- warnings, non-convergence, and missing-data behavior are not collapsed into a
  generic success state;
- protected-content and private-data scans are clean or risk-dispositioned;
- professional reliance wording stays human-owned;
- unresolved gaps remain visible as `TBD` or open issues.

## 9. Open Decisions

- TBD: final benchmark tolerance policy for mechanics, stress recovery, and
  nonlinear support evidence.
- TBD: public benchmark source acceptance process and reviewer roster.
- TBD: release-label policy beyond the minimum validation strategy gate.
- TBD: required GUI validation evidence once the GUI tranche matures.
- TBD: long-term storage format for reviewed validation evidence bundles.
