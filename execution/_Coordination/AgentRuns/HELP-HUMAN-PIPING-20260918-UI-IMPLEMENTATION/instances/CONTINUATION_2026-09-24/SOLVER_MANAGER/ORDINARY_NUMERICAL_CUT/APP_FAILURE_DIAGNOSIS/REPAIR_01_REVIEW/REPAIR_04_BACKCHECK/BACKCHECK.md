# Repair 04 App applicability backcheck

**CLEAR for the already-authorized single-case runtime and TypeScript checks. No actionable source-review findings.** The complete narrow App diff and changed-assertion basis table were reviewed against the actual consumer source, schemas and retained initial guards. No runtime was executed by this reviewer.

Candidate: combined fourteen-file diff `995fe7ef1194c86fbb684b0947809b48aefd9d6126156aa12bc2d784601b538e`. Only the named App test and its required imports changed from Repair03. All fourteen actual files match the final freeze; the other thirteen files and the suffix after this case are unchanged. The incremental preimage matches the previously reviewed candidate, and both preserved diffs reconstruct exactly. No duplicate full-source images were produced.

The recorded Repair03 run genuinely reports 297 passes and one failure: the App case stops at its old 0.2 export-version assertion while actual output is 0.3. That is predecessor evidence, not a pass for this repair or for later assertions in the case.

The repaired expectations reflect the current contract:

- Genuine raw wire 0.2, the precision semantic contract, and complete absence of raw dimension fields are explicit preconditions. The derivative's outer/inner 0.3 versions and copied producer/numerical/formulation metadata agree with `results.v0.3.schema.yaml` and `deriveResultDocument`.
- Dimension-absent row/carrier checksum scopes and origin class match `buildCurrentResultExport` and schema enums. Independent original-producer attestation stays false/null. Exact raw-row, target-row, model, carrier and whole-document digest checks are retained using the checked profile helper.
- The accepted `diagnostic_work` signature has null physical/derivative dimension. Both work rows remain in the stress export, receive no physical witness, and carry the expected informational withholding diagnostics plus the blocking aggregate. All retained rows are checked for exact value/unit and interpreted dimension; no inference from `N*m` is introduced.
- Stress-neutral 0.3/profile v3, metadata, bound received-result checksum and package checksum match the strict builder/schema. Review-only, blocked-validation, no-conversion, privacy and professional-claim guards remain.
- Full-precision display assertions use exact received row/summary values. This repairs stale presentation literals without altering a solver oracle, tolerance or numerical acceptance limit. The final model/source equality guard adds coverage.

The initial test still waits for the real Current proof result, refuses a missing/unavailable export, checks model/run identity and review boundaries, verifies the disabled private export before explicit local-private intent, and verifies that withdrawing intent disables it again. No production, fixture, schema, protected numerical tolerance or legacy maintained test was changed.

Prior Repair01–03 source clearance remains applicable to the unchanged consumers. The manager may run the planned named App case and TypeScript check after the CPU lease is available, retaining actual failure/pass evidence. New runtime failures or subsequent source changes need affected diagnosis, correction and backcheck. This source clearance does not establish runtime, native/practitioner, merge, engineering, lifecycle or release acceptance.

Evidence: `_run_records/SOURCE_CHECK.json` and `RUNTIME_ORIGINS.json`. Parent `/root/solver_manager` remains sole source integrator; reviewer writes stayed inside this backcheck directory.
