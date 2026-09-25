# Repair 02 source backcheck

**Disposition: production P2 corrected; one new test defect prevents backcheck clearance.** No runtime pass or failure was observed, and no tests/builds/Git/native operations were performed. The combined fourteen-file diff is `be80fc3ef944668945045f6bccbf038ee10bb051a8a779a67ddeb13bf77fe429`.

## P2 — Wrong-producer tests reject during setup before reaching the report boundary

Location: `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.test.ts:426–432`, specifically the awaited `buildAnalysisRunPreview` call at line 430.

Both parameter cases deliberately construct a hash-valid manifest whose solver name or version disagrees with the genuine precision source. They then pass that manifest to the ordinary strict analysis builder. `previewService.ts:162–176` delegates this call to `buildAnalysisRunV03`; `analysisRunCompatibility.ts:122` unconditionally calls `validateAnalysisRunV03` before returning a precision record, and line 151 rejects exactly this source/record producer mismatch with `ANALYSIS_SOURCE_PRODUCER_MISMATCH`. Therefore the setup await throws before the line 431 report call and its rejection assertion. The two new cases cannot demonstrate `REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH` and would fail the intended focused suite.

Repair direction: preserve the production builder's strict rejection. Build a valid base session first, then make an explicitly adversarial test-only analysis-record copy that consistently points to the freshly hash-valid wrong-identity manifest, including recorded name/version and manifest references/hash. If the fixture claims a coherent record checksum, recompute it using the existing `analysisRecordProjection` and checked hash helper. Leave the actual raw precision source unchanged. Pass that adversarial record directly to `buildReportPackageRequest` and assert the report-specific mismatch. An independent assertion of the upstream builder's rejection is useful but does not replace the report-boundary negative test.

## Original finding backcheck

The production correction addresses the prior hardcoded solver identity: `reportSolverIdentity` runs after manifest-byte and manifest-reference/hash checks, requires record/manifest identity and build-reference agreement, compares actual precision producer name/version against that basis, and uses the checked identity in both report output locations. It does not derive a build reference from the version, confuse transport versions with producer versions, or upgrade historical source bytes. Legacy absent solver metadata is confined to the exact 0.1 analysis route; strict historical records require recorded identity. Current and legacy positive assertions and the literal alternate-build assertion cover the corrected output shape by source inspection.

No other actionable finding in this two-file backcheck. All fourteen actual files match the new freeze; the two incremental preimages match Repair01, and the other twelve files are byte-identical to the previously reviewed candidate. Reconstructed incremental and complete combined diffs match their preserved files exactly. The earlier fourteen-file source review therefore remains applicable with this incremental backcheck.

Repair the two negative-test setups, freeze the resulting candidate, and obtain bounded backcheck before the manager proceeds with the ROOT-granted affected eight-file Vitest and TypeScript run. Runtime checks remain outstanding; source review is not merge, engineering, release, or lifecycle acceptance.

Evidence: `_run_records/SOURCE_CHECK.json`, `SCOPE_CHECK.json`, and `RUNTIME_ORIGINS.json`. Only this backcheck directory was written; parent `/root/solver_manager` retains source integration ownership.
