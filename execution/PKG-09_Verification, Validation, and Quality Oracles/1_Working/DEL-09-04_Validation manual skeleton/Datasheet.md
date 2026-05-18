# Datasheet: DEL-09-04 Validation manual skeleton

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-09-04 |
| Deliverable name | Validation manual skeleton |
| Package ID | PKG-09 |
| Package name | Verification, Validation, and Quality Oracles |
| Deliverable type | DOC_UPDATE |
| Scope item | SOW-027 |
| Objectives | OBJ-008; OBJ-011 |
| Context envelope | M |
| Lifecycle artifact | Draft setup document, not an issued validation manual |

## Attributes

| Attribute | Value |
|---|---|
| Primary purpose | Define a validation manual structure that separates mechanics verification, workflow validation, user rule checks, and professional reliance. |
| Required boundary | The manual must not certify, approve, seal, authenticate, or declare code compliance. |
| Engineering authority | Human engineering judgment remains required for project-specific reliance. |
| Code-specific data posture | User-supplied or private; not bundled into public validation examples. |
| Benchmark source posture | Original, public-domain, or permissively licensed examples only. |
| Protected-content posture | Protected standards text, tables, figures, examples, code formulas, material allowables, SIF/flexibility tables, and proprietary commercial data are excluded. |
| Status vocabulary | `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`, `HUMAN_APPROVED_FOR_PROJECT`; no automatic `CODE_COMPLIANT` status. |

## Conditions

| Condition | Source basis | Effect on the manual skeleton |
|---|---|---|
| Verification vs validation distinction is mandatory. | `docs/VALIDATION_STRATEGY.md` section 1; SOW-027 | The outline separates mechanics benchmark evidence from intended-use workflow evidence. |
| Rule checks are user-defined computations. | `INIT.md`; `docs/TYPES.md`; `docs/SPEC.md` section 6 | The outline treats rule-pack checks as private/user design-basis evaluation, not professional authentication. |
| Professional reliance is outside software authority. | `docs/CONTRACT.md` OPS-K-AUTH-1; OBJ-011 | Every report-facing or validation-facing section must preserve the human review boundary. |
| Public validation examples must avoid protected and proprietary sources. | `docs/IP_AND_DATA_BOUNDARY.md`; `docs/VALIDATION_STRATEGY.md` section 5 | Source/provenance review is part of the validation manual records. |
| Unit-aware and diagnostic result boundaries remain relevant. | OPS-K-UNIT-1; `docs/SPEC.md` sections 7-9 | Manual sections include unit/schema verification and diagnostic/result-envelope checks where applicable. |

## Construction

The validation manual skeleton is organized as the following outline. Section content is intentionally skeletal until benchmark deliverables, GUI workflow evidence, report reproducibility evidence, and release gates mature.

| Manual section | Purpose | Initial state |
|---|---|---|
| Product scope and limitations | Define what OpenPipeStress assists with and what remains outside software authority. | Skeleton required |
| Solver theory summary | Summarize open mechanics at a non-protected, public level. | Skeleton required |
| Unit and schema verification | Record dimensional, schema, and invalid-input checks. | Skeleton required |
| Element verification | Record frame/pipe element benchmark families and evidence links. | Skeleton required |
| Load and stress recovery verification | Record load application and open mechanics stress-recovery benchmark evidence. | Skeleton required |
| Nonlinear support verification | Record convergence, active-state, and unresolved non-convergence evidence. | Skeleton required |
| Rule-pack evaluator verification | Record invented-value evaluator tests, missing-input behavior, unit mismatch behavior, and sandbox behavior. | Skeleton required |
| GUI workflow validation | Record intended-use workflow evidence and warning presentation checks. | Skeleton required |
| Report reproducibility validation | Record model hash, manifest, warning inclusion, checksum stability, and protected-content lint evidence. | Skeleton required |
| Known limitations and open issues | Preserve risks, TBDs, exclusions, and human acceptance requirements. | Skeleton required |

## References

- `INIT.md`
- `AGENTS.md`
- `docs/DIRECTIVE.md`
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/VALIDATION_STRATEGY.md`
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/ContextBudgetQA.csv`
