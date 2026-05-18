# Specification: DEL-06-05 Invented non-code example rule pack

## Scope

This deliverable defines the governed setup documentation for an invented, non-code demonstration rule pack. It is limited to the local `DEL-06-05` working folder and does not create or modify repo-level example artifacts such as `examples/rule_packs/invented_demo.yaml`.

The deliverable covers:

- the public/protected data boundary for a future invented rule-pack example;
- required non-engineering and professional-boundary notices;
- traceability to SOW-016, OBJ-005, and OBJ-011;
- setup evidence showing that the example concept can proceed without proprietary values or standards-derived content.

The deliverable excludes:

- real engineering allowables, stress limits, code formulas, SIF/flexibility values, protected dimensional tables, or owner/vendor data;
- implementation of the rule-pack schema, evaluator, completeness checker, or checksum registry;
- any claim that a future example demonstrates code compliance or professional approval.

## Requirements

| Requirement ID | Requirement | Source |
|---|---|---|
| DEL-06-05-REQ-01 | The setup documentation shall state that public rule-pack examples use invented non-code values and clear non-engineering notices. | `docs/CONTRACT.md` `OPS-K-RULE-1`; `docs/_Registers/ScopeLedger.csv` row `SOW-016` |
| DEL-06-05-REQ-02 | The setup documentation shall prohibit protected standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data. | `docs/CONTRACT.md` `OPS-K-IP-1`, `OPS-K-IP-3`; `docs/DIRECTIVE.md` sections 3 and 4.2 |
| DEL-06-05-REQ-03 | The setup documentation shall distinguish a user-rule check from professional approval and shall not claim certification, sealing, authentication, or code compliance for reliance. | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/TYPES.md` sections 4 and 6 |
| DEL-06-05-REQ-04 | The setup documentation shall keep code-specific values user-supplied or private and shall mark unknowns as `TBD` rather than supplying defaults. | `docs/CONTRACT.md` `OPS-K-DATA-1`, `OPS-K-DATA-2`; `INIT.md` Agent rule |
| DEL-06-05-REQ-05 | A future example rule pack shall carry source/provenance and redistribution status fields; actual checksum values remain `TBD` until a concrete payload exists. | `docs/CONTRACT.md` `OPS-K-DATA-3`, `OPS-K-RULE-3`; `docs/SPEC.md` section 6 |
| DEL-06-05-REQ-06 | A future example rule pack shall remain declarative and non-executable; evaluator grammar and sandbox details remain owned by separate PKG-06 deliverables. | `docs/CONTRACT.md` `OPS-K-RULE-2`; `docs/_Decomposition/SOFTWARE_DECOMP.md` `DEL-06-02` and `OI-006` |
| DEL-06-05-REQ-07 | This setup run shall not write outside the assigned deliverable folder or move any artifact to `ISSUED`. | `AGENTS.md` dispatch rule; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 4 |
| DEL-06-05-REQ-08 | Setup outputs shall preserve architecture-basis constraints that are applicable to rule-pack examples: schema-first records, provenance, diagnostics, protected-content gates, and no-bypass boundaries. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8 |

## Standards

No external engineering code, standard, table, or proprietary design basis is used as an authority for this setup deliverable.

| Standard or governing source | Status |
|---|---|
| OpenPipeStress governance and invariant documents | Accessible local governing source |
| Any engineering code or standards-body text | Not used; protected or user-supplied unless explicitly licensed and authorized |
| Rule-pack evaluator expression grammar | `TBD`; separate PKG-06 implementation decision |
| Rule-pack schema final file path | `TBD`; separate `DEL-06-01` implementation deliverable |

## Verification

| Verification ID | Verifies | Method |
|---|---|---|
| DEL-06-05-VER-01 | Four-document kit exists locally | Run `tools/validation/check_four_documents.sh` on this deliverable folder |
| DEL-06-05-VER-02 | No repo-level example path was edited | Check git status for changes under the assigned deliverable folder only |
| DEL-06-05-VER-03 | Dependency register is schema-valid | Run `python3 tools/validation/validate_dependencies_schema.py` on local `Dependencies.csv` |
| DEL-06-05-VER-04 | Dependency enums are canonical | Run `python3 tools/validation/validate_enum.py` for emitted enum values |
| DEL-06-05-VER-05 | Professional and protected-data boundaries are visible | Review these documents for explicit non-engineering, no-certification, and no-protected-content language |
| DEL-06-05-VER-06 | Semantic setup artifacts exist | Confirm `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_run_records/*` exist locally |

## Documentation

Required setup artifacts for this deliverable are:

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

Repo-level example artifacts remain outside this session's write scope.
