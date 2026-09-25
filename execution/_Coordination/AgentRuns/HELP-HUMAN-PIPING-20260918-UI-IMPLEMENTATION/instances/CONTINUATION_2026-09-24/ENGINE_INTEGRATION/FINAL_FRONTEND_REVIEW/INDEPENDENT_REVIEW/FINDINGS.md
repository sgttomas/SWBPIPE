# Frozen 44-path review findings

## FE-JOIN-01 — P2: bind AnalysisRun signature to the source basis

Original frozen `apps/desktop/src/services/analysisRunCompatibility.ts:20` selects the first kind/unit/component match without checking `source_basis`. The composite table contains two signatures for `pipe_elastic_normal_stress_maximum_v2`: ordinary `supported-source-068` first, then `physics-source-endpoint-normal-maximum` for `retained_source_endpoint_normal_max_v1`.

Both genuine N05 sparse/dense captures contain the latter basis, but the actual frozen `analysisRowSemantics` function returns the ordinary signature. `buildAnalysisRecord` writes that wrong signature into `result_refs`, and `validateAnalysisRunV03` repeats the same lookup, so its self-check accepts the misclassification. This loses the explicit ordinary/source maximum distinction and can disagree with basis-aware downstream consumers. It is not a newly demonstrated numerical or native failure.

Apply the same exact basis predicate already used in `resultSemantics`, while preserving the intentionally different incomplete-component disclosure behavior and old tables that do not declare a basis. Add genuine source/ordinary/mixed controls and reject absent/unknown basis for the basis-specific composite variants. Reported promptly to Root and physics; the sole frontend owner accepted a bounded repair.

`SEMANTIC_PROBE_ORIGINAL.json` records the exact original function hash, semantic table hash, both genuine capture hashes and observed signatures. `semantic_probe.cjs` transpiles that original function in memory and supplies the actual composite table as a fixed dependency. This is a bounded function probe, not a full validator/native invocation, product solve or broad test. `ANALYSIS_ORIGINAL_FROZEN.ts` preserves the original preimage; `ORIGINAL_44_BASIS.json` binds all 44 patch reconstructions against the supplied base.

Repair status: RESOLVED. Exact three-path repair manifest78aa7776 and patch361a31f2 were independently checked. SEMANTIC_PROBE_REPAIRED.json confirms all14 genuine pairs now select the literal expected signature and reject absent/unknown bases;321 rows across precision-1, physics-1 and old source-blocks-1 retain prior interpretation. Owner174-test execution also checks coherently rehashed ordinary/source signature swaps; final TypeScript passes. No other actionable finding remains in the reviewed44-path scope.
