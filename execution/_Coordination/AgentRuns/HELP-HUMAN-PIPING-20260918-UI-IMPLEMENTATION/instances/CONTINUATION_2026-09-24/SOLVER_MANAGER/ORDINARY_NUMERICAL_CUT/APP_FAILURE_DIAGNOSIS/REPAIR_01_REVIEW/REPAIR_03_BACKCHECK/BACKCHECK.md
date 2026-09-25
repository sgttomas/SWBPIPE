# Repair 03 source backcheck

**CLEAR for bounded fan-in and the already-authorized affected runtime checks. No unresolved actionable source-review findings remain.** This clears the Repair02 test-setup finding; the Repair01 producer-identity finding remains corrected by the unchanged Repair02 production implementation. No product/test/build/native/Git execution was performed by this reviewer, and this is not a runtime pass claim.

Reviewed candidate: combined fourteen-file diff `624a6a5bd69f16a9f5f5c0ac4be4e67ae653695ebe24929a72a25d50f3ff8395`. Only `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.test.ts` changed from Repair02.

Both wrong-producer cases now assert the upstream strict-builder rejection at line 431. They then clone the valid base record, coherently rebind the record's producer identity, recorded build, settings/unit references and input-manifest reference/hash to the deliberately wrong but hash-valid manifest. The existing checked hash helper recomputes the record checksum over `analysisRecordProjection`, and `verifyAnalysisRunRecord` must report `match`. The original received-result checksum is asserted unchanged, and the raw precision source is compared with its original serialization. The separate report call at lines 445–446 now reaches the intended source-versus-record producer mismatch branch without weakening or bypassing the production validator.

The new fixture is explicitly adversarial test input; it is not labeled an admitted producer return. Positive current/legacy identity assertions, literal recorded-build preservation, privacy/source immutability checks, and all prior reviewed App/consumer changes are retained.

All fourteen actual file hashes match the final freeze. The single incremental preimage matches Repair02, all other thirteen files are byte-identical to the previously reviewed candidate, and the original legacy plus sparse/dense precision raw fixtures remain byte-identical to the initial review. Both incremental and complete combined diffs reconstruct exactly from preserved images. No duplicate full-source snapshot was created by this backcheck.

The manager may proceed with the ROOT-authorized eight affected Vitest files and TypeScript run on this candidate. Their actual results remain outstanding, as do broader merge-time checks and any applicable native witnesses. Later source changes require affected review/validation coverage. This source clearance does not imply engineering acceptance, release, lifecycle issuance, or external validation.

Evidence: `_run_records/SOURCE_CHECK.json` and `RUNTIME_ORIGINS.json`. Write scope stayed within this backcheck directory; parent `/root/solver_manager` remains sole source integrator. Separately reported generator review results were not re-executed or adopted as new reviewer-observed runtime evidence.
