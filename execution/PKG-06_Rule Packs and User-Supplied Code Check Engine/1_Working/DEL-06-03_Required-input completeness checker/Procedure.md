# Procedure: DEL-06-03 Required-input completeness checker

## Purpose

Define the setup-time procedure and future implementation checks for the required-input completeness checker without writing implementation files or executable rules in this session.

## Prerequisites

| Prerequisite | Status for setup | Notes |
|---|---|---|
| Sealed DEL-06-03 context | Available | `_CONTEXT.md` identifies SOW-004, OBJ-002, and OBJ-005. |
| Governing data/IP boundary | Available | `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
| Rule-pack schema contract | Future dependency | DEL-06-01 must define required-input declarations before checker implementation. |
| Analysis status semantics | Future dependency | DEL-05-04 supplies status vocabulary such as `RULE_INPUTS_INCOMPLETE`. |
| Rule expression grammar/library | TBD | Open issue OI-006; not resolved here. |

## Steps

1. Confirm the deliverable identity and scope match `DEL-06-03`, `PKG-06`, and SOW-004.
2. Preserve the protected-data boundary by excluding standards text, tables, code formulas, allowables, SIF/flexibility data, vendor proprietary data, and private rule-pack content.
3. Define required checker behavior in terms of declarative required-input metadata, missing-input findings, provenance expectations, and status gating.
4. Keep solve-required missing data separate from rule-check-required missing data.
5. Require future tests to verify that missing rule-pack inputs produce `RULE_CHECK_BLOCKING` / `RULE_INPUTS_INCOMPLETE` behavior without asserting professional compliance.
6. Record dependencies on schema/status/diagnostic contracts in `Dependencies.csv`.
7. Leave implementation code, checker modules, schemas, and tests untouched for this setup-only run.

## Verification

For this setup run, verify:

- the four-document kit exists;
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and preserve lens-not-authority language;
- `Dependencies.csv` validates against the v3.1 schema;
- `_DEPENDENCIES.md` counts match the CSV;
- `_STATUS.md` remains within the allowed setup lifecycle and reaches `SEMANTIC_READY` only after semantic and dependency artifacts exist;
- no files outside the DEL-06-03 write scope were edited.

## Records

The setup records are the deliverable-local documents, dependency register, semantic artifacts, status history, and run records under `_run_records/`.
