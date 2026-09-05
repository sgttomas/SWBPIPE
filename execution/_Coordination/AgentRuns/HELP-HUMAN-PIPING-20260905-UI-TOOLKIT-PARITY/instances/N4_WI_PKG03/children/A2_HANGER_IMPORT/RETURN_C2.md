# A2_HANGER_IMPORT C2 repair return

Status: REPAIRED_FOCUSED_CHECKS_PASS_PENDING_INDEPENDENT_REVIEW. Role TASK Agent2, software-bounded-implementation v1; no delegation. Runtime delegated-harness-native; role/nondelegation instruction+config asserted; model unavailable. Original root/run-record override retained. Parent N4 WORKING_ITEMS.

## Authority and preserved evidence

Executed REPAIR_C2_BRIEF.md for exact findings in REVIEW_C1_RETURN.md under parent explicit release. Prior V2 contract, SCA009/decomposition0.12/DAG010 and N2 B3 accepted snapshot remain upstream references. This is a derivative repair evidence package, not decomposition or lifecycle truth. C1 RETURN.md, CHECKS.json and SOURCE_HASHES.json remain immutable; C2 before hashes match their frozen source after hashes.

## Repairs

P1: hanger-only disposition diagnostic views recognize project_private as private and block public import with IMPORT_PRIVATE_DATA_PUBLIC_BLOCKED at the original library/record path. The source object is never rewritten. Legacy material/section/component paths retain their existing behavior.

P2: hanger traversal considers explicit item dispositions even when provenance is absent. Missing/null/malformed provenance still yields schema/provenance rejection findings, but explicit protected_suspected or quarantined item markers independently add IMPORT_PROTECTED_CONTENT_SUSPECTED with quarantine severity at the original object path. Overall QUARANTINE takes precedence. This works for library, record and malformed quantity wrappers, preserving existing full-provenance outcomes.

Added 32 shared regression cases: public project_private at library and record; both protected_suspected and quarantined markers across library, record and quantity with missing, null, array, numeric and string provenance. The shared corpus now has 86 cases. Tests assert each new expected code/path/severity, then compare the complete Python/Rust outcome and ordered finding tuples. Existing no-mutation assertions retain all original source metadata.

## Verification and scope

Authorized focused Python command: 105 passed. Authorized offline Rust command: 11 unit + 8 integration passed; shared integration compares all86 cases dynamically with Python. CHECKS_C2.json stores exact commands, cwd, target directory {PIPING_HANGER_IMPORT_TARGET}, full output and exits. Scope validator PASS. Five original-fence source files changed during C2; SOURCE_HASHES_C2.json records before/after hashes of all10 original implementation source files and the five changed paths. No new code surface, legacy helper mutation, schema expansion, downstream product/operation/native/desktop change, Git mutation, network action or installation occurred.

Tools: targeted reads/edits via shell/Python, parent-authorized pytest/cargo, python3 tools/software_workflow/validate_change_scope.py. Tool/write policy PASS. Broad registered checks remain parent-owned and were not run.

## Acceptance and handoff

Repairs address DEL0307 OUT001/AC001/VER001 provenance/private/public/quarantine behavior and associated DEL0302 schema import parity for the accepted hanger contract. Author verification is not independent acceptance. Accepted upstream unchanged; derivative source package current at hashes below. Closure: bounded repairs complete, fresh independent full-diff review REQUIRED. Rerun on relevant source or contract changes; regenerate hashes after further repair. Native metadata match, manual identity selection, source snapshot persistence, actual consumed DOF/value, atomic whole-configuration replacement and stale-before tests remain downstream ownership as documented in C1. No human ruling requested. Next owner N4 WORKING_ITEMS / HELP_HUMAN.

## Frozen complete source hashes
- `core/library_import/provenance_checker.py`: `1c55afd98c2ec0d7780a67a5715ad521fbc3dea119c9cda77c458891a25e32a5`
- `core/library_import/README.md`: `3ecc0271b3c4b2d31cb8566c955e66fa94d9e3406835beb4bb08a614fe96bd98`
- `core/library_import/library_import_document/src/lib.rs`: `127d0fee760ea63d59eee1cc9cb73c121e671ccbfd0297d4218f1e3bfc41d979`
- `core/library_import/library_import_document/tests/provenance_parity.rs`: `e85f023b2d2be6756a055e4c48f02f7e2f1ee63a237a5bba01f76dee7b992b1a`
- `core/library_import/library_import_document/README.md`: `c8d738bc1e903bef87d19f6ddc409defddb9303bfce202ac11821dccd48c1abb`
- `tests/test_library_import_provenance.py`: `fa478e37bebfc02c51a1fbb042af6b7129c0fb0623bbd6d957e0d14099620520`
- `schemas/hanger.schema.yaml`: `f7ada4cab2536cb0a8f952cbcc6de0e13f99a58ca4601c22441542b09c33c2f9`
- `tests/test_hanger_library_schema.py`: `de8cf9a4d9f9ca4e4b4a6163c3f0d10db096c9b65e0cbf1c0856e312cf45a7d6`
- `fixtures/hanger/invented_hanger_cases.json`: `35edc5269668a0da4db9a8492b67f1ee1f26becd44c6fa2136f8853eeda10e96`
- `fixtures/hanger/invented_hanger_library_valid.json`: `d722b59b338a54410438ec39150138556873a68874f3e77c0f8f421f93fbbfc3`
