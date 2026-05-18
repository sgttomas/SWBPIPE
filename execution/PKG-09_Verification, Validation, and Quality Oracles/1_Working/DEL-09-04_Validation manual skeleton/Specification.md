# Specification: DEL-09-04 Validation manual skeleton

## Scope

This deliverable defines the local skeleton for a validation manual. It covers the manual structure, required boundaries, evidence categories, and acceptance checks needed to distinguish:

- mechanics verification;
- intended-use workflow validation;
- user rule-pack checks;
- professional reliance and human acceptance.

This deliverable does not edit `docs/VALIDATION_STRATEGY.md`, does not create issued validation evidence, does not certify code compliance, and does not approve any piping calculation for reliance.

## Requirements

| ID | Requirement | Source basis | Acceptance hook |
|---|---|---|---|
| VAL-REQ-001 | The manual skeleton shall separate mechanics verification from workflow validation. | `docs/VALIDATION_STRATEGY.md` section 1; SOW-027 | `Procedure.md` verification checks confirm separate sections and terminology. |
| VAL-REQ-002 | Mechanics verification sections shall describe software comparison against declared mechanics problems and tolerances, not code compliance. | `docs/VALIDATION_STRATEGY.md` section 1; `docs/TYPES.md` section 6 | Review confirms no automatic compliance wording appears. |
| VAL-REQ-003 | Validation sections shall evaluate intended-use workflow fitness while preserving project-specific professional judgment. | `docs/VALIDATION_STRATEGY.md` section 1; OBJ-011 | Review confirms professional reliance language remains human-bound. |
| VAL-REQ-004 | Rule-pack verification sections shall treat pass/fail results as user-supplied rule computations, not professional authentication. | `docs/SPEC.md` section 6; `docs/TYPES.md` section 4 | Review confirms statuses do not include automatic `CODE_COMPLIANT`. |
| VAL-REQ-005 | The skeleton shall include the ten manual sections listed by the validation strategy. | `docs/VALIDATION_STRATEGY.md` section 3 | Datasheet outline and Procedure records include all ten sections. |
| VAL-REQ-006 | Benchmark and validation examples referenced by the skeleton shall be original, public-domain, or permissively licensed, with protected and proprietary examples excluded. | `docs/VALIDATION_STRATEGY.md` section 5; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 | Review confirms no protected examples, code tables, or proprietary values are introduced. |
| VAL-REQ-007 | The skeleton shall expose missing evidence, open risks, limitations, and `TBD` entries rather than filling gaps silently. | OPS-K-AGENT-1; OPS-K-AGENT-2; `docs/VALIDATION_STRATEGY.md` section 4 | Review confirms open items are visible in the manual outline. |
| VAL-REQ-008 | Release-gate language shall describe software maturity and validation evidence only, not engineering approval. | `docs/VALIDATION_STRATEGY.md` section 4; OPS-K-AUTH-1 | Review confirms no certification, sealing, approval, authentication, or code-compliance claims. |
| VAL-REQ-009 | Unit, schema, diagnostic, and result-envelope checks shall be represented where relevant to validation manual sections. | OPS-K-UNIT-1; `docs/SPEC.md` sections 7-9; AB-00-06 | Review confirms those check families have section slots and do not overclaim. |
| VAL-REQ-010 | The skeleton shall preserve the public/private data boundary for rule packs, materials, component data, owner requirements, and project models. | OPS-K-IP-1 through OPS-K-IP-3; OPS-K-DATA-1 through OPS-K-DATA-3 | Review confirms user/private data is described as supplied by users or lawful private sources. |

## Standards

No external engineering code or standard is used as source authority for this skeleton. The governing sources are the OpenPipeStress project documents and registers listed in `_REFERENCES.md`.

Any future validation manual reference to an external standard, protected code, commercial benchmark, vendor data, or owner requirement is `TBD` until provenance, redistribution rights, and human/legal review are recorded.

## Verification

| Check | Method | Expected result |
|---|---|---|
| Four-document presence | Run `tools/validation/check_four_documents.sh` on the deliverable folder. | All four setup documents are present. |
| Protected-content boundary | Manual review against `docs/IP_AND_DATA_BOUNDARY.md` and OPS-K-IP invariants. | No protected standards data, proprietary values, or commercial examples introduced. |
| Professional boundary | Search for certification, approval, sealing, authentication, and compliance claims. | No software/agent claim exceeds decision-support authority. |
| Manual outline coverage | Compare Datasheet Construction against `docs/VALIDATION_STRATEGY.md` section 3. | Ten manual sections are represented. |
| Dependency register schema | Run `python3 tools/validation/validate_dependencies_schema.py` on `Dependencies.csv`. | v3.1 schema is valid. |

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

The repository-level `docs/VALIDATION_STRATEGY.md` remains read-only for this deliverable.
