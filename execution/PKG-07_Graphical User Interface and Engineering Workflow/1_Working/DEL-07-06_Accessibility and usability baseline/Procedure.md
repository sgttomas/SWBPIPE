# Procedure: DEL-07-06 Accessibility and usability baseline

## Purpose

Define the setup procedure for producing and later using the accessibility and usability baseline for engineering-review GUI and report-facing workflows.

## Prerequisites

- Sealed DEL-07-06 context with write scope limited to this deliverable folder.
- Register scope: SOW-036 and OBJ-006.
- Applicable invariants: OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV, OPS-K-AGENT-1..4.
- Architecture basis: AB-00-03, AB-00-05, AB-00-06, AB-00-08 where relevant to GUI commands, state, diagnostics, result envelopes, and tests.
- Human ruling remains required for the final accessibility conformance target.

## Steps

1. Confirm the work is setup/document production only and does not edit GUI source, tests, schemas, manifests, package files, or repo-level docs.
2. Read the deliverable context, registers, contract, decomposition, and relevant product/specification source slices.
3. Record the accessibility/usability baseline topics from PRD section 21: keyboard navigation, icon labels/tooltips, high-contrast result visualization, search/filter, copy/export, undo/redo, project templates, inline validation, and solve-vs-code-check warning separation.
4. Map baseline topics to engineering review needs from OBJ-006: model creation, missing data, results, and assumptions must be visible.
5. Preserve diagnostic class boundaries from SPEC section 7 and AB-00-06.
6. Mark exact WCAG or equivalent conformance target as `TBD` pending human decision, including whether the target applies to desktop GUI, report preview/export, generated report files, or separate surfaces.
7. Define verification hooks for future GUI workflow validation, report reproducibility validation, protected-content review, private-data review, and product-claims review.
8. Generate semantic and dependency setup artifacts without treating semantic matrices as engineering authority.
9. Set `_STATUS.md` to `SEMANTIC_READY` only if the four documents, semantic matrix, lensing register, dependency register, and local validation checks pass.

## Verification

| Check | Expected result |
|---|---|
| Four-document kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist with default sections. |
| Scope control | No files outside the DEL-07-06 working folder are modified by this task. |
| Accessibility target | Exact WCAG or equivalent target remains `TBD`; no final conformance claim is made. |
| Data boundary | No protected standards content, proprietary values, or private project data are introduced. |
| Warning boundary | Solve-blocking and rule-check-blocking warnings remain distinct in future verification hooks. |
| Fixture boundary | Future screenshots, public examples, and exported report fixtures are checked for protected standards content and private project data. |
| Professional boundary | No output claims certification, code compliance, sealing, approval, or professional reliance. |
| Dependency register | `Dependencies.csv` validates against v3.1 schema. |

Pass 3 lensing source rereads: `_SEMANTIC_LENSING.md` items X-001 and X-002 were checked against `docs/SPEC.md` section 7, `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06, `docs/IP_AND_DATA_BOUNDARY.md` section 3, and this procedure's verification table before this enrichment.

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`
