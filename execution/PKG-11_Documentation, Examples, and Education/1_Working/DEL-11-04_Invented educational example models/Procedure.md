# Procedure: DEL-11-04 Invented educational example models

## Purpose

Define the setup-time procedure and future production checks for invented educational examples without creating external example files, tutorials, source code, schemas, protected data, or issued artifacts in this session.

## Prerequisites

| Prerequisite | Status for setup | Notes |
|---|---|---|
| Sealed DEL-11-04 context | Available | `_CONTEXT.md` identifies SOW-033, OBJ-001, and OBJ-008. |
| Governing data/IP and professional boundary | Available | `INIT.md`, `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `docs/TYPES.md`. |
| Future model schema and persistence format | Future dependency | DEL-02-01 and DEL-02-05 are needed before actual public model files are materialized. |
| Future rule-pack schema and invented fake rule-pack pattern | Future dependency | DEL-06-01 and DEL-06-05 are needed before fake-rule-pack demonstration files are materialized. |
| Future validation fixture expectations | Future dependency | PKG-09 validation work determines when examples can become regression or validation fixtures. |

## Steps

1. Confirm the deliverable identity and scope match `DEL-11-04`, `PKG-11`, and SOW-033.
2. Preserve the protected-data boundary by excluding standards text, protected examples, code formulas, allowables, SIF/flexibility data, proprietary vendor data, commercial software examples, and private project data.
3. Classify future examples into mechanics-only invented demonstrations and fake-rule-pack demonstrations.
4. Require future example artifacts to carry a non-reliance notice, provenance fields, unit labels, and explicit `TBD` markers for unresolved inputs.
5. Keep mechanics solve examples separate from fake rule-check examples so users can see that mechanics solved, user-rule checked, and professionally approved are different states.
6. Record dependencies on schema, rule-pack, protected-content review, and validation deliverables in `Dependencies.csv`.
7. During this setup run, write only deliverable-local documentation, semantic artifacts, dependency artifacts, run records, and status history.
8. Leave external example model files, external tutorials, source code, schemas, and `ISSUED` movement untouched.

## Verification

For this setup run, verify:

- the four-document kit exists;
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and remain semantic aids rather than engineering authority;
- `Dependencies.csv` validates against the v3.1 schema;
- `_DEPENDENCIES.md` counts match the CSV;
- `_STATUS.md` reaches `SEMANTIC_READY` only after the setup sequence passes;
- no external example files, tutorials outside this deliverable, source code, schemas, repo-level artifacts, or `ISSUED` files were created;
- no protected standards examples, commercial software examples, realistic code allowables/formulas, or professional-reliance claims appear in the deliverable-local outputs.

## Records

The setup records are the four-document kit, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/*` in this DEL-11-04 folder.
