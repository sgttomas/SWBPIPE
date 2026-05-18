# Procedure: DEL-08-06 State, comparison, and handoff report sections

## Purpose

Define an operational procedure for producing and verifying the DEL-08-06 report-section artifacts without inventing implementation evidence, engineering values, standards text, or product code. This procedure is scoped to future execution of state/run, comparison, and handoff manifest report sections.

## Prerequisites

| Prerequisite | Required state | Source |
|---|---|---|
| Deliverable context | `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, and `Dependencies.csv` are available in the deliverable folder. | Local folder |
| Report generator basis | Report sections integrate with report generator conventions. | `Dependencies.csv` row DAG-002-E0855; `docs/SPEC.md` section 9 |
| Audit manifest/hash basis | Audit manifest and model-hash conventions are available. | `Dependencies.csv` row DAG-002-E0856; `docs/SPEC.md` sections 4.4 and 9 |
| Warnings/provenance basis | Warnings, assumptions, provenance notes, limitations, unresolved TBDs, and human-review-needed findings are represented. | `Dependencies.csv` row DAG-002-E0857; `docs/SPEC.md` section 9 |
| Result export basis | Result export envelopes are available for report consumption. | `Dependencies.csv` row DAG-002-E0858; `docs/SPEC.md` section 9 |
| Protected-content controls | Protected-content linter controls and private redaction/export controls are available or explicitly marked TBD. | `Dependencies.csv` rows DAG-002-E0859 and DAG-002-E0860 |
| State/run/comparison sources | Immutable model states, analysis runs, state comparisons, run comparisons, and comparison export contracts are available or explicitly marked TBD. | `Dependencies.csv` rows DAG-002-E0861 through DAG-002-E0865 |
| Handoff sources | Handoff package manifests, export workflow records, and external-prover boundary metadata are available or explicitly marked TBD. | `Dependencies.csv` rows DAG-002-E0866 through DAG-002-E0868 |
| Professional/IP boundary | Boundary wording and public/private content controls are applied. | `docs/CONTRACT.md`; `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` |

## Steps

1. Confirm the deliverable identity is DEL-08-06 and the package is PKG-08.
2. Read the current source contracts for report generation, audit manifests, report sections, result exports, protected-content linting, state/run records, comparison records, handoff package manifests, export workflows, and external-prover metadata.
3. Define the state/run report-section inputs as references to immutable model-state and analysis-run records. Include stable IDs, model/run basis, hashes, solver version stamps, settings, units, load cases, diagnostics, warnings, assumptions, rule/library references, result hashes, and source/provenance notes where available.
4. Define the comparison report-section inputs as references to deterministic state/run comparison records. Include stable IDs, manual mappings where required, unmatched classifications, unit-normalized deltas, tolerance/profile references, diagnostics, settings, and source/provenance notes where available.
5. Define the handoff manifest report-section inputs as references to handoff package records. Include model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags, export workflow records, and external-prover boundary metadata where available.
6. For every field that carries an engineering value, confirm the value has unit/dimensional metadata, provenance, review state, privacy classification, and source envelope evidence where required. If evidence is absent, emit an explicit finding or TBD rather than a default.
7. Apply professional-boundary wording. The generated sections may support human professional review but must not state or imply software-generated code compliance, approval, certification, sealing, authentication, endorsement, or professional reliance.
8. Apply public/private and protected-content boundaries. Public sections, templates, examples, and fixtures must avoid protected standards text, protected tables, protected examples, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, and real secrets.
9. Preserve deterministic ordering and stable references so repeated generation from identical source envelopes produces repeatable output.
10. Record unresolved implementation decisions as TBD. Current TBDs include concrete code paths, exact schema fragments, API names, report layout, final notice wording, release thresholds, and external transport/export details.

## Verification

| Check | Expected result | Evidence |
|---|---|---|
| Scope completeness | State/run, comparison, and handoff report-section artifacts are represented. | Tests or review notes, TBD |
| SOW-024 coverage | Inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations are present or explicitly marked TBD. | Tests or fixture review, TBD |
| Boundary wording | No automatic professional approval, certification, sealing, authentication, endorsement, code-compliance, or reliance claim is emitted. | Boundary tests, TBD |
| Missing-data behavior | Missing solve-required or rule-check-required values become explicit findings, warnings, limitations, unresolved TBDs, or human-review-needed findings. | Tests, TBD |
| Unit/provenance behavior | Numeric values carry unit/dimensional metadata and provenance or explicit diagnostics. | Schema/unit tests, TBD |
| Protected-content behavior | Public report surfaces avoid protected/private content and preserve only permitted references/checksums/source notes. | Protected-content/provenance gate, TBD |
| Determinism | Repeated section generation from identical source envelopes is stable. | Snapshot or round-trip tests, TBD |
| Dependency mirror | Approved DAG-002 rows in `Dependencies.csv` remain ACTIVE and unmodified unless a later human-approved reconciliation changes the mirror. | Local dependency validation and final report |

## Records

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` for setup context.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` for semantic lensing, not engineering authority.
- `Dependencies.csv` and `_DEPENDENCIES.md` as the approved DAG-002 local mirror/evidence surface.
- Future implementation tests and review notes for section assembly, protected-content avoidance, professional-boundary wording, provenance/checksum preservation, deterministic output, and missing-data findings.
