# Procedure: DEL-17-02 Export package, profile, and stable ID map contracts

## Population Procedure

1. Confirm DAG-005 is the approved active graph authority.
2. Confirm `DEL-17-01` has committed implementation evidence before treating `DEL-17-02` as the next unblocked PKG-17 population target.
3. Read the local `DEL-17-01` four-document kit, `Source_Basis_Register.md`, and `CAEPIPE_Question_Dossier.md`.
4. Populate this four-document kit at contract level only.
5. Build semantic artifacts that map source basis, requirements, downstream consumers, and boundary exclusions.
6. Extract the local dependency register into `Dependencies.csv`.
7. Refresh deliverable memory and create a run record.
8. Validate the four-document kit, minimum fileset, dependency schema, coordination blocker queue, and diff hygiene.

## Downstream Consumption Procedure

For each later PKG-17 target deliverable:

1. Start from this contract and the `DEL-17-01` source-basis dossier.
2. Declare the target profile and target version basis.
3. Declare supported, omitted, approximated, delegated, unsupported, and `TBD` behavior.
4. Define stable ID behavior before target writing or parser work.
5. Require an export manifest and loss report.
6. Keep target-specific implementation inside that deliverable's approved write scope.
7. Preserve project data-boundary and professional-boundary exclusions.

## Validation Commands

Run from repository root:

```bash
tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
python3 tools/validation/validate_dependencies_schema.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/Dependencies.csv"
python3 tools/coordination/build_dev001_blocker_queue.py --dag-dir execution/_DAG/DAG-005 --evidence execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv --csv-out execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv --markdown-out execution/_Coordination/DEV-001_BLOCKER_QUEUE.md --generated-date 2026-05-18
python3 tools/validation/validate_semantic_matrix.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
python3 tools/validation/validate_lens_register.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
git diff --check
```

## Closeout Procedure

Closeout requires:

- four-document validation pass;
- minimum viable fileset validation pass;
- dependency schema validation pass;
- semantic matrix validation pass;
- lens-register validation pass;
- blocker queue recomputation from DAG-005 active edges and active evidence;
- memory update with touched files, validation results, remaining TBDs, and boundary exclusions;
- run record creation;
- no edits to `DEL-17-03` through `DEL-17-09` production documents;
- no lifecycle promotion beyond TASK-owned local status update;
- no code, schema, release, compatibility, professional, or code-compliance claim.

## Semantic Enrichment Verification

Pass 3 semantic-lensing enrichment checked `_SEMANTIC_LENSING.md` items `A-001`, `B-001`, `X-001`, and `E-001` against `Specification.md` architecture-basis requirements, `Datasheet.md` local artifact inventory, this procedure's validation and closeout checks, and `Guidance.md` reviewer checklist. The resulting closeout checks require downstream target profiles to preserve the architecture basis, keep the local contract artifact set auditable, validate semantic/lens artifacts before consumption, and carry semantic readiness evidence into review.
