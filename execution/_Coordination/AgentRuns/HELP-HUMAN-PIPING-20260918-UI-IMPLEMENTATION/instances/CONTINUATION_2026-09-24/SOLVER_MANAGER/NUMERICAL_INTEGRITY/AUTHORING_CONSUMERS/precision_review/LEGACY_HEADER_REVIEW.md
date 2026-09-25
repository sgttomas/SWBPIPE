# Legacy metadata presence discrepancy

**F4 — P2, repaired and statically backchecked.** The parent reported this discrepancy after the 32-file source review. All 32 frozen hashes still match at this check. [_run_records/F4_SOURCE_BASIS.json](_run_records/F4_SOURCE_BASIS.json) preserves the inspected source hashes.

At `projects/chirality-piping/apps/desktop/src/features/results/numericalResultQuality.ts:11`, the raw0.1 branch uses the truthiness of `producer`, `numerical_quality` and `formulation_basis` to identify contradictory precision metadata. An own field with value `null`, `false`, `0` or an empty string is therefore treated as absent and selects the legacy semantics table. The historical `buildAnalysisRunV02` guard at `projects/chirality-piping/apps/desktop/src/services/analysisRunCompatibility.ts:44` has the same issue for both raw0.1 and the expressly historical synthetic raw0.2 route.

Rust `semantic_contract::for_source` rejects any of those keys through `get(...).is_some()`, and Python `_source_contract` and `_build_analysis_run` use key membership. Thus TypeScript can produce or interpret a historical derivative/analysis from a version/header combination that the matching core source reader rejects. A populated producer object is already refused; the defect concerns presence of falsy values. This is not a Current-promotion finding: Preview still requires the recognized precision route and numerical Current qualification still rejects these carriers.

The bounded repair is own-property presence checks (`Object.hasOwn`) for each of the three forbidden keys, without changing stored evidence or opaque historical read/hash handling. Recommended controls:

- Parameterize all three fields over `null`, `false`, `0` and an empty string. Raw0.1 `sourceContract` must return unsupported; historical V02 construction must reject both raw0.1 and raw0.2 with any such own field.
- Retain clean no-header raw0.1 legacy dispatch and the existing exact historical synthetic raw0.2 analysis fixture/hash as positives.
- A saved raw0.1 carrier with an explicit null field should remain renderable historical evidence, report unsupported semantics, and preserve its original bytes and source checksum verification. Rejection of semantic admission must not erase history.

No runtime commands or source edits were performed. This new finding reopens the two affected TypeScript guards and their negative controls; it does not invalidate the earlier F1/F2/F3 repair logic. The repair/backcheck below restores expanded source clearance; runtime and integrated gates remain separately pending.

## First repair backcheck

The two production guards now use `Object.hasOwn` for all three keys. Classification tests cover all three fields over null/false/0/empty-string, and the historical builder covers both raw0.1/raw0.2. The explicit-null history test preserves renderability and source bytes without changing production history handling. All 32 refreshed source hashes match; only the five declared source/test paths differ from the pre-F4 freeze.

One repair-side test coverage adjustment remains: pinning `savedEnvelope` directly to the immutable mechanics fixture omitted the dimension enrichment previously applied by `runPreviewMechanics`. As first repaired, the false/true helper variants therefore both became dimension-absent. Restore `bindSourceResultDimensions` on the cloned fixture before the existing conditional dimension deletion, retaining the prior enriched/raw historical coverage and keeping fixture selection immutable. This is a test-semantics correction; no additional production issue was found.

## Final repair backcheck

The historical helper now applies `bindSourceResultDimensions` to the immutable cloned fixture before conditionally deleting dimensions, matching its former semantics. A focused test explicitly distinguishes enriched and dimension-absent variants. The five-file F4 delta is statically clear, prior units/native/applier source is unchanged, and all 32 final hashes match the refreshed manager freeze. [SOURCE_HASHES_F4_BACKCHECK.json](SOURCE_HASHES_F4_BACKCHECK.json) and [_run_records/F4_FINAL_FREEZE_MATCH.json](_run_records/F4_FINAL_FREEZE_MATCH.json) bind this conclusion. No actionable source finding remains. Runtime outputs have not yet been supplied or reviewed; no runtime pass is claimed.
