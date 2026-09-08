# W10 portability clarification V3

Parent authorized format-only correction after the final pytest portability check identified BUILD_CONTEXT.json's environment-specific temporary_root as an active executable anchor. The original bytes are preserved under `_run_records/PORTABILITY_ORIGINALS_V1/BUILD_CONTEXT.json` as historical structural evidence. The current record substitutes `{TEMPORARY_DIAGNOSTIC_ROOT}` and explicitly points to that historical provenance. No source-file hashes, build method, diagnostic result or candidate recommendation changed. No diagnostic replay or source/product/authority edits occurred.

Original SHA256: `d33140013dfc0c61b573357aab517b70f51a55d82a247d52689dd273bb7a22e3`. Normalized SHA256: `0e0ea599e28c4e71f611257a80b4160647a9e74ef96ee2bdf54f9c43cc499a9d`. PORTABILITY_RELOCATION_V3.json records exact logical-to-historical relocation. Historical V1/V2 manifest entries remain byte-unchanged and resolve their original BUILD_CONTEXT.json entry through that relocation map; MANIFEST_V3.json binds both original and normalized bytes. All other historical manifest entries retain their existing paths/hashes.

Validation: original/archive byte equality, source_files equality, method equality, and successor-manifest hashes verified. R1 narrow review and CHANGE rerun remain pending; this correction makes no independent portability-suite PASS claim.
