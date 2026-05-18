# Procedure: DEL-07-03 Material, component, and rule-pack editors

## Purpose

Define how the `DEL-07-03` setup artifacts are produced and checked, and record the operational boundaries that future implementation work must preserve for material, component, rule-pack, load/support, and private-library editors.

This procedure does not implement GUI code.

## Prerequisites

| Prerequisite | Required source/evidence |
|---|---|
| Sealed deliverable context | `_CONTEXT.md` for `DEL-07-03` |
| Governing invariants | `docs/CONTRACT.md`, especially data, unit, rule-pack, privacy, IP, agent, and professional-boundary invariants |
| Decomposition/register identity | `docs/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `docs/_Registers/ContextBudgetQA.csv` |
| Architecture basis constraints | `_CONTEXT.md` Architecture Basis Injection and `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8 |
| Skill contracts | `skills/four-documents/SKILL.md`; `skills/semantic-matrix-build/SKILL.md`; `skills/lens-register/SKILL.md`; `skills/dependency-extract/SKILL.md` |
| Write scope | Only this deliverable folder and its descendants |

## Steps

1. Confirm local scope.
   - Verify the target folder is `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/`.
   - Verify `_STATUS.md` is in an overwrite-allowed setup state before four-document generation.
   - Do not read protected/private libraries or create `ISSUED` artifacts.

2. Run `four-documents` with `RUN_PASSES=P1_P2`.
   - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and the decomposition/register source slices.
   - Produce `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
   - Keep default sections present in all four documents.
   - Mark unknown engineering values and implementation choices as `TBD`.
   - Update `_STATUS.md` from `OPEN` to `INITIALIZED` only if current state is `OPEN`.
   - Record the run in `_run_records/TASK_RUN_2026-04-30_1040_four-documents-p1-p2.md`.

3. Run `semantic-matrix-build`.
   - Read `_CONTEXT.md`, `_STATUS.md`, the four production documents, and `_REFERENCES.md`.
   - Write `_SEMANTIC.md` with canonical matrices A/B, derived matrices C/F/D/K/G/X/T/E, interpretation work, audit result, and matrix summary.
   - Audit final interpreted cells for algebra leaks, operator leaks, and long uninterpreted expansions.
   - On audit pass, set or verify `_STATUS.md` as `SEMANTIC_READY`.
   - Record the run in `_run_records/TASK_RUN_2026-04-30_1041_semantic-matrix-build.md`.

4. Run `lens-register`.
   - Read `_SEMANTIC.md`, `_CONTEXT.md`, `_STATUS.md`, the four production documents, and `_REFERENCES.md`.
   - Write `_SEMANTIC_LENSING.md` only.
   - Include lens coverage for every cell in matrices A, B, C, F, D, X, and E.
   - Record warranted gaps, questions, or conflicts only when supported by production-document evidence or explicit absence.
   - Record the run in `_run_records/TASK_RUN_2026-04-30_1042_lens-register.md`.

5. Run `four-documents` with `RUN_PASSES=P3_ONLY`.
   - Treat `_SEMANTIC_LENSING.md` as a candidate worklist, not an authority.
   - Before every substantive enrichment, reread the target section and the source slice supporting the change.
   - Do not regress `_STATUS.md`; if it is already `SEMANTIC_READY`, leave it as-is.
   - Record source rereads in `Guidance.md` and in `_run_records/TASK_RUN_2026-04-30_1043_four-documents-p3.md`.

6. Run `dependency-extract`.
   - Use `Specification.md` as the anchor document and `Procedure.md`, `Guidance.md`, and `Datasheet.md` as execution documents.
   - Create or refresh `Dependencies.csv` with v3.1 columns.
   - Refresh `_DEPENDENCIES.md` with declared lists preserved, extracted summary, run notes, run history, lifecycle summary, and handoff notes.
   - Run local validation commands for schema, enums, and ID formats.
   - Record the run in `_run_records/TASK_RUN_2026-04-30_1044_dependency-extract.md`.

7. Final setup check.
   - Confirm required artifacts exist.
   - Confirm no file outside the assigned deliverable folder changed because of this session.
   - Confirm no `ISSUED` artifact was created.
   - Confirm `_STATUS.md` says `SEMANTIC_READY` only if semantic and dependency gates passed.

## Verification

Run these checks from the repository root:

```sh
tools/validation/check_min_viable_fileset.sh "execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors"
python3 tools/validation/validate_dependencies_schema.py "execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/Dependencies.csv"
tools/validation/validate_id_format.sh DEL DEL-07-03
tools/validation/validate_id_format.sh PKG PKG-07
```

Additional checks:

- Ensure `_SEMANTIC.md` contains matrices A, B, C, F, D, K, G, X, T, E and an audit pass note.
- Ensure `_SEMANTIC_LENSING.md` contains lens coverage for matrices A, B, C, F, D, X, E.
- Ensure `Dependencies.csv` ACTIVE rows include `EvidenceFile` and `SourceRef`.
- Ensure no text claims code compliance, certification, sealing, approval, or professional reliance.
- Ensure no protected standards data or proprietary library content appears in the documents.

## Records

Records produced by this setup run:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/*.md`
