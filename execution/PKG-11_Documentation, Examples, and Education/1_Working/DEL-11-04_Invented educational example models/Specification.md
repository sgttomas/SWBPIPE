# Specification: DEL-11-04 Invented educational example models

## Scope

This deliverable specifies setup constraints for invented educational example models that can later support mechanics-only demonstrations and fake-rule-pack demonstrations. The examples are documentation and testing aids, not engineering templates.

This setup run does not create actual model files under `examples/models/invented/*`, tutorials outside this deliverable, source code, schemas, protected standards examples, commercial software comparisons, code allowables, code formulas, or professional-reliance material.

## Requirements

| ID | Requirement | Source basis | Verification approach |
|---|---|---|---|
| R-DEL-11-04-001 | Public educational examples must use invented or original data only and must not contain protected standards content, standards-derived examples, commercial-software examples, proprietary vendor data, code allowables, SIF/flexibility data, or protected dimensional tables. | OPS-K-IP-1/2/3; OPS-K-RULE-1; SOW-033 | Protected-content and provenance review before any future public example is materialized. |
| R-DEL-11-04-002 | Future example artifacts must state that they are educational/test fixtures only and are not suitable for engineering reliance, certification, approval, sealing, or code-compliance claims. | OPS-K-AUTH-1; `docs/DIRECTIVE.md` human authority principle; `docs/TYPES.md` professional boundary | Documentation review verifies no professional reliance or automatic compliance language is present. |
| R-DEL-11-04-003 | Mechanics-only examples must illustrate open mechanics and unit-aware reproducibility without embedding code-specific load combinations, allowables, acceptance formulas, or standards interpretations. | OPS-K-DATA-1/2; OPS-K-UNIT-1; `docs/SPEC.md` loads and stress recovery sections | Future example review checks unit labels, provenance fields, and absence of code-specific defaults. |
| R-DEL-11-04-004 | Fake-rule-pack demonstrations must use fictional labels and non-engineering placeholder values and must not approximate realistic design-code formulas, allowables, or pass/fail criteria. | OPS-K-RULE-1/3; `docs/SPEC.md` rule-pack evaluator section | Future rule-pack demo review checks fictional notices, checksum/provenance metadata, and no protected rule content. |
| R-DEL-11-04-005 | Missing solve-required, rule-check-required, or provenance information in future examples must be explicit as `TBD` or a visible finding, never a silent default. | OPS-K-DATA-2; OPS-K-AGENT-1/2 | Future review checks example manifests for explicit unresolved fields and no invented hidden defaults. |
| R-DEL-11-04-006 | Future examples used for regression or validation support must distinguish mechanics verification from code compliance and professional approval. | OBJ-008; `docs/SPEC.md` V&V mechanics; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` review checklist | Future validation handoff checks state that examples are original or invented and are not compliance evidence. |
| R-DEL-11-04-007 | This setup session must write only inside the DEL-11-04 deliverable folder and must not move artifacts to `ISSUED`. | User sealed brief; OPS-K-AGENT-3/4 | Final file list and git status are scoped to the deliverable folder. |

## Standards

No standards-body formulas, code text, tables, examples, allowables, acceptance rules, or protected interpretations are included in this setup artifact. Any future example that requires code-specific material must use user-supplied private data or a lawfully redistributable source with documented rights and human review.

## Example Families

| Family | Allowed setup intent | Exclusions |
|---|---|---|
| Mechanics-only invented examples | Demonstrate open centerline mechanics concepts, unit handling, diagnostics, reproducibility, and auditable inputs using invented toy data. | No code-specific combinations, allowables, protected equations, standards examples, or professional reliance. |
| Fake-rule-pack demonstrations | Demonstrate the user-rule-check workflow with fictional rule names, fictional required inputs, fake thresholds, checksums, and notices. | No realistic code formulas, no protected rule text, no material allowable tables, no claim that a result is code compliant. |

## External Inputs

| Input | Required from | Notes |
|---|---|---|
| Canonical model/schema structure | DEL-02-01 and DEL-02-05 | Needed before future actual example model files are materialized outside this setup folder. |
| Rule-pack schema and invented rule-pack example boundary | DEL-06-01 and DEL-06-05 | Needed before future fake-rule-pack demonstration files are materialized. |
| Mechanics benchmark and validation expectations | DEL-09-01 and related validation work | Needed before examples are promoted as regression or validation fixtures. |
| Protected-content/report lint expectations | DEL-08-05 and governance review | Needed before future examples or tutorials are published as public repository artifacts. |

## Verification

For this setup run, verification requires:

- four-document kit exists in the DEL-11-04 folder;
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and preserve lens-not-authority language;
- `Dependencies.csv` validates against the v3.1 schema;
- `_DEPENDENCIES.md` summarizes the same active dependency rows as `Dependencies.csv`;
- `_STATUS.md` reaches `SEMANTIC_READY` only after semantic, lensing, P3, and dependency gates pass;
- no actual example files, tutorials outside this deliverable, source code, schemas, repo-level files, or `ISSUED` artifacts are created;
- no protected standards examples, commercial software examples, realistic code allowables/formulas, or professional-reliance claims are introduced.

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

Register-listed future artifacts under `examples/models/invented/*` and tutorials are not created by this setup session.
