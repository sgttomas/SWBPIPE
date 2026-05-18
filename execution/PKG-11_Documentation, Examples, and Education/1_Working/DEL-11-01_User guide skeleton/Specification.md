# Specification: DEL-11-01 User guide skeleton

## Scope

This deliverable defines the deliverable-local skeleton for the OpenPipeStress user guide. It covers the guide structure needed for project setup, centerline modeling, mechanics solving, user rule checks, result review, report generation, limitations, and professional responsibility notices.

This deliverable does not edit `docs/user_guide/index.md`, does not create tutorials or example models, does not document protected standards content, and does not claim certification, endorsement, approval, sealing, authentication, or engineering code compliance.

## Requirements

| ID | Requirement | Source basis | Acceptance hook |
|---|---|---|---|
| UG-REQ-001 | The guide skeleton shall include sections for project setup, modeling, solving, rule checks, reports, and limitations. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row DEL-11-01 | Procedure verification confirms all required section slots exist. |
| UG-REQ-002 | Setup content shall preserve unresolved implementation details as `TBD` where packaging, dependency versions, public API transport, import/export formats, solver library, expression grammar, or project container are not yet decided. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` open issues | Review confirms no invented setup commands or product behavior are asserted. |
| UG-REQ-003 | Modeling content shall frame the global model as a unit-aware 3D centerline/frame model with explicit nodes, elements, components, supports, loads, and provenance-bearing fields. | `docs/DIRECTIVE.md` sections 2-3; `docs/SPEC.md` sections 1, 3, 4, and 7 | Datasheet Construction and Guidance Principles cover model-building categories. |
| UG-REQ-004 | Solving content shall distinguish mechanics solve status from rule-check status and human professional acceptance. | `docs/TYPES.md` sections 4 and 6; OPS-K-MECH-2; OPS-K-AUTH-1 | Search and review confirm no automatic `CODE_COMPLIANT` status or professional approval claim appears. |
| UG-REQ-005 | Rule-check content shall state that code-specific values, allowables, combinations, formulas, SIFs, flexibility factors, owner requirements, and proprietary data are supplied by users through lawful private data or rule packs. | `INIT.md`; `docs/DIRECTIVE.md` sections 1 and 3; OPS-K-DATA-1; OPS-K-RULE-1 and OPS-K-RULE-3 | Guidance examples exclude protected and proprietary source material. |
| UG-REQ-006 | Report content shall include auditable-report slots for software/solver version, model hash, input manifest, units, warnings, assumptions, source/provenance notes, rule-pack checksum, results, and limitations. | `docs/SPEC.md` section 8; OPS-K-REPORT-1 | Datasheet Construction includes a reporting section and Procedure checks the slots. |
| UG-REQ-007 | Limitation content shall cover professional responsibility, validation status, public/private data boundaries, local FEA handoff, missing data, and unresolved `TBD` items. | `docs/DIRECTIVE.md` sections 3-6; `docs/VALIDATION_STRATEGY.md` section 1; OPS-K-AUTH-1 | Guidance and Procedure include boundary checks. |
| UG-REQ-008 | Public guide content shall not reproduce protected standards text, tables, figures, examples, protected dimensional data, proprietary commercial data, or private user/project data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-7; OPS-K-IP-1 through OPS-K-IP-3 | Protected-content review finds no prohibited content. |
| UG-REQ-009 | The skeleton shall use canonical status and vocabulary terms where user-facing status semantics are referenced. | `docs/TYPES.md` sections 4-7 | Procedure verification checks canonical status terms. |
| UG-REQ-010 | The skeleton shall expose unknowns and assumptions rather than filling gaps with silent defaults. | OPS-K-AGENT-1; OPS-K-AGENT-2; OPS-K-DATA-2 | Review confirms remaining unknowns are `TBD` or explicit limitations. |

## Standards

No external engineering code or standard is used as source authority for this user guide skeleton. The governing sources are the OpenPipeStress project documents and registers listed in `_REFERENCES.md`.

Any future guide content that references an external code, standard, vendor catalog, owner requirement, commercial software example, or licensed data source is `TBD` until provenance, redistribution rights, and human/legal review are recorded.

## Verification

| Check | Method | Expected result |
|---|---|---|
| Four-document presence | Run `tools/validation/check_four_documents.sh` on the deliverable folder. | All four setup documents are present. |
| Required guide slots | Compare Datasheet Construction against UG-REQ-001. | Required setup, modeling, solving, rule-check, report, and limitation sections are present. |
| Protected-content boundary | Manual review against `docs/IP_AND_DATA_BOUNDARY.md` and OPS-K-IP invariants. | No protected standards data, proprietary examples, or private user data are introduced. |
| Professional boundary | Search for certification, approval, sealing, authentication, endorsement, and compliance overclaims. | Any hits are prohibitions or boundary statements, not product claims. |
| Dependency register schema | Run `python3 tools/validation/validate_dependencies_schema.py` on `Dependencies.csv`. | v3.1 schema is valid. |
| Lensing coverage | Count matrix lens coverage rows in `_SEMANTIC_LENSING.md`. | 96 rows for matrices A, B, C, F, D, X, and E. |

## Documentation

The setup artifact set for this deliverable consists of:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/*`

The repository-level `docs/user_guide/index.md` remains read-only for this deliverable.
