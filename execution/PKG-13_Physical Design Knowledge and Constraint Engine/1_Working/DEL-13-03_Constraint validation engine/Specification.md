# Specification: DEL-13-03 Constraint validation engine

## Scope

DEL-13-03 covers a backend feature slice for deterministic validation messages over available physical-design constraints and missing required data. The in-scope validation categories are connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. Sources: `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` rows SOW-068 and DEL-13-03.

Excluded from this deliverable are hidden owner standards, protected code requirements, protected standards content, proprietary project data, final engineering acceptance logic, and automatic professional/code-compliance claims. Sources: `_CONTEXT.md` Package Exclusions; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6; `docs/SPEC.md` section 4.3.

## Implementation Evidence

The current implementation evidence is `core/constraints/validation/engine.py`, exported through `core/constraints/validation/__init__.py`, and exercised by `tests/test_constraint_validation.py`. The module is a stdlib-only Python validation slice over supplied mapping data. It does not mutate accepted model state, compute geometric intersections, solve clearances, define owner criteria, or make final engineering decisions.

The implemented API is `validate_constraint_envelope(constraint_envelope, design_knowledge_envelope=None) -> ValidationResult`, with `diagnostic_dicts(...)` for deterministic test comparison. `Diagnostic.to_dict()` emits `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`; `ValidationResult.to_dict()` emits `diagnostics` plus `has_blocking_findings`.

The focused test file uses invented public fixtures for constraint and design-knowledge envelopes. It verifies deterministic output, represented conflict classes, explicit missing-data and unresolved-reference findings, unit metadata and PKG-02 dimension checks, protected/private provenance diagnostics, and absence of prohibited authority-claim text.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-13-03-R1 | The validation engine shall validate available design knowledge for the SOW-068 categories: connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. | `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 |
| DEL-13-03-R2 | Validation outputs shall be deterministic validation messages rather than hidden report prose or agent text. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 Notes; DEL-13-03 row |
| DEL-13-03-R3 | Missing required data shall be surfaced as explicit findings/diagnostics and shall not be silently defaulted. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3 |
| DEL-13-03-R4 | Validation shall preserve provenance visibility for messages where source or availability of design knowledge affects reliance. | SOW-068 provenance-aware message requirement; `docs/SPEC.md` sections 1 and 4 |
| DEL-13-03-R5 | The engine shall not infer hidden owner standards, protected code requirements, protected standards values, proprietary values, or final engineering acceptance logic. | `_CONTEXT.md` Context Envelope and Package Exclusions; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |
| DEL-13-03-R6 | Outputs shall remain compatible with the accepted schema-first architecture basis and diagnostics/result-envelope boundary. The current implementation exposes a Python diagnostic dict shape; formal runtime result-envelope integration remains `TBD`. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` sections 1, 4.3, and 9; `core/constraints/validation/engine.py` |
| DEL-13-03-R7 | The deliverable shall include validation diagnostics tests covering the grounded message categories and boundary conditions. | `_CONTEXT.md` Anticipated Artifacts; `docs/_Registers/Deliverables.csv` row DEL-13-03; `tests/test_constraint_validation.py` |
| DEL-13-03-R8 | Documentation refresh and later bounded implementation work shall preserve current ACTIVE dependency rows in `Dependencies.csv` as predecessor evidence unless a later human-approved CHANGE process authorizes reclassification. | `_DEPENDENCIES.md`; `Dependencies.csv`; project instruction for this run |

## Standards

No external engineering-code text, tables, values, or clause-level requirements are locally available for this deliverable. The governing local standards for this evidence refresh are project governance, architecture sources, and current implementation evidence:

| Reference | Applicability |
|---|---|
| `docs/CONTRACT.md` | Binding invariants for IP/data boundary, missing-data handling, units, authority boundaries, and agent non-invention. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundary. |
| `docs/SPEC.md` | Technical architecture, domain-core boundary, unit/provenance controls, diagnostics/result-envelope posture, and no-compliance-claim boundary. |
| `docs/TYPES.md` | Epistemic labels and canonical vocabulary for protected data, user-supplied code data, diagnostics, and professional approval. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | Accepted revision 0.7 current decomposition basis for DEL-13-03 scope and exclusions. |
| `_CONTEXT.md` | Deliverable-local identity, architecture-basis injection, and control-surface constraints. |
| `core/constraints/validation/engine.py` | Current validation module, diagnostic record implementation, boundary checks, and deterministic ordering. |
| `tests/test_constraint_validation.py` | Current focused validation diagnostics tests and invented public fixtures. |

## Verification

| Requirement | Verification approach |
|---|---|
| DEL-13-03-R1 | `tests/test_constraint_validation.py` verifies represented conflict classes and available records for connectivity, clearance, no-go/route, support-zone, slope/drain/vent, access, equipment-interface, and missing-required-data inputs. |
| DEL-13-03-R2 | `test_validation_is_deterministic_and_covers_represented_classes` repeats validation over deep-copied inputs and compares `diagnostic_dicts(...)`. |
| DEL-13-03-R3 | `test_missing_data_and_unresolved_references_are_explicit_findings` verifies missing fields, missing targets/parameters, unresolved design-knowledge references, and missing provenance become diagnostics. |
| DEL-13-03-R4 | Provenance and boundary tests verify diagnostics preserve `source_references` where supplied provenance carries source evidence. |
| DEL-13-03-R5 | Boundary tests verify protected/private provenance diagnostics and scan output text for prohibited authority-claim terms. |
| DEL-13-03-R6 | Current verification checks the Python diagnostic dict shape and `ValidationResult.to_dict()`; full application result-envelope integration remains `TBD`. |
| DEL-13-03-R7 | Validation diagnostics tests are recorded in `tests/test_constraint_validation.py` and executable directly with `python3 tests/test_constraint_validation.py`. |
| DEL-13-03-R8 | Dependency handling check confirms current ACTIVE local dependency rows remain unchanged unless a later human-approved CHANGE process replaces that instruction. |

## Documentation

Required records and remaining deferrals:

- Constraint validation module path and API boundary: `core/constraints/validation/engine.py`; `validate_constraint_envelope(...)`; `diagnostic_dicts(...)`.
- Constraint diagnostic record shape: implemented Python dict fields are `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`; formal result-envelope mapping remains `TBD`.
- Validation diagnostics test inventory: `tests/test_constraint_validation.py`.
- Public-safe validation fixtures: executable invented fixtures exist in `tests/test_constraint_validation.py`; publication-grade examples and owner/project examples remain `TBD` pending provenance review and human acceptance.
- Assumptions and unsupported inputs: must be recorded as `TBD` or explicit findings rather than inferred defaults.
- Dependency evidence handling: preserve current ACTIVE local dependency rows during this evidence-refresh workflow.
- Legitimate remaining `TBD` items include localization/message cataloging, full geometric conflict solving, owner criteria/rules, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, and human acceptance.
