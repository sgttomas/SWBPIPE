# Procedure: DEL-17-06 Stress-neutral CSV/JSON package

## Purpose

This procedure describes how to produce and review the Phase A four-document kit for the stress-neutral CSV/JSON package, and how later implementation work should consume it without crossing into code, schema, release, compatibility, or professional-approval claims.

## Prerequisites

| Prerequisite | Status in Phase A |
|---|---|
| DEL-17-01 source-basis kit available | Read as upstream context. |
| DEL-17-02 export package/profile/stable-ID/loss-report contract available | Read as upstream context. |
| DEL-08-04 result export format | Declared upstream dependency; detailed result schema behavior remains `TBD` in this deliverable. |
| DEL-14-02 and DEL-14-05 comparison/run contracts | Declared upstream dependencies; exact comparison export semantics remain `TBD` in this deliverable. |
| Governing references in `_REFERENCES.md` | Read as source basis. |
| Protected/proprietary examples | Not admitted. |

## Steps

1. Confirm the deliverable identity from `_CONTEXT.md`: DEL-17-06, PKG-17, `BACKEND_FEATURE_SLICE`, SOW-046 and SOW-074.
2. Confirm `_STATUS.md` permits a P1/P2 four-documents write. If current state is outside the allowed overwrite states, stop without overwriting production documents.
3. Read `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, and the local `_SEMANTIC.md` placeholder.
4. Read the relevant decomposition, export plan, governance, data-boundary, and result-export source slices.
5. Read DEL-17-01 and DEL-17-02 four-document kits for source-basis, target-boundary, stable-ID, manifest, and loss-report carryforward.
6. Populate `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` with source-grounded statements only.
7. Preserve `TBD` for unresolved target behavior, including exact CSV table names, CSV columns, JSON schema/object layout, package-member paths, comparison tolerances, validation thresholds, and external-target mappings.
8. Exclude proprietary examples, protected standards content, private project data, code-specific allowables, SIF/flexibility values, and professional/code-compliance claims.
9. Run local validation checks allowed for this Phase A documentation task.
10. If the starting lifecycle state is `OPEN`, update `_STATUS.md` to `INITIALIZED` using the four-documents safe-update rule.
11. Update the run record with changed files, validation, missing inputs, and remaining human-ruling items.

## Later Implementation Consumption

When a later sealed TASK is authorized to implement the stress-neutral package, use this document kit as an input and keep the implementation inside that later task's approved write scope.

1. Re-read DEL-17-01 and DEL-17-02 before defining support claims, package contracts, stable-ID behavior, manifest content, or loss-report behavior.
2. Generate or update stress-neutral CSV and JSON schemas only in the later authorized schema/code scope; keep exact field names, package paths, and object layouts `TBD` until then.
3. Implement writer behavior against the common package/profile/ID-map/manifest/loss-report contract and the result export envelope boundary.
4. Create invented or rights-cleared fixtures only after recording provenance, redistribution status, and protected-content review evidence.
5. Validate CSV and JSON outputs for shared identity, unit/dimensional metadata, source model/run references, manifest basis, loss-report content, diagnostics, and boundary notices.
6. Preserve comparison tolerances, pass/fail wording, and export comparison semantics as `TBD` until DEL-14-02/DEL-14-05 provide the applicable upstream basis.
7. Record generated schemas, CSV outputs, JSON outputs, manifests, loss reports, validation reports, and fixtures as future implementation evidence, not as code-compliance or professional-acceptance evidence.

## Verification

Run from repository root:

```bash
tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package"
tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package"
rg -n "certif|seal|code compliance|professional approval|engineering acceptance|formal validation|release readiness|proprietary example|protected standards" "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package"
git diff --check -- "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package"
```

Expected Phase A verification result:

- four-document kit exists;
- minimum viable fileset remains present;
- professional-boundary terms appear only as prohibited-claim language;
- no implementation code, schema files, fixtures, or dependency extraction artifacts are created;
- `TBD` remains visible for unresolved implementation and target behavior.

## Records

Phase A records:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_STATUS.md`
- `_run_records/TASK_RUN_2026-05-18_1155.md`
- `_run_records/TASK_RUN_2026-05-18_1218.md`

Records intentionally not produced in this Phase A task:

- `Dependencies.csv`
- `_SEMANTIC_LENSING.md`
- stress-neutral CSV schema
- stress-neutral JSON schema
- export writer
- comparison fixtures
- implementation tests

## Closeout Checks

- Changed files remain inside the allowed DEL-17-06 write scope.
- `MEMORY.md` remains unchanged as instructed.
- No sibling DEL-17 folders are edited.
- No later skills are run.
- No schema, code, release, compatibility, code-compliance, professional-approval, formal-validation, or engineering-acceptance claim is made.

Future implementation closeout checks, once implementation artifacts exist:

- Generated stress-neutral CSV schemas, JSON schemas, CSV outputs, JSON outputs, manifests, loss reports, and fixtures are present only if authorized by that later task.
- Package-member paths, source model references, analysis-run references, manifest references, ID-map references, and validation-report references are populated or explicitly left `TBD`.
- CSV and JSON representations pass a synchronization check for identity, units, manifest basis, loss-report content, diagnostics, and boundary notices.
- Hash records identify payload scope; JSON payload hashes use the canonical JSON/JCS-compatible basis where applicable, and non-JSON partitioning remains explicit.
- Fixture/example data has provenance and redistribution evidence, or remains absent/`TBD`.
- Target-specific support flags cite source evidence or remain `TBD`.
- Comparison semantics cite DEL-14-02/DEL-14-05 or remain diagnostic/audit-only and `TBD`.
