# Precision desktop consumer return — provisional freeze

TASK return to authoring manager. Implementation is authored, uncommitted and unexecuted pending the ROOT resource grant. This is not review, native proof, acceptance or release. Exact owned source paths and SHA-256 hashes are in `SOURCE_HASHES.json`; runtime execution provenance is confined to `_run_records/provenance.json`.

## Implemented

- Central pure source dispatch accepts only raw 0.2/product 0.2/precision-1 with exact metadata keys and enumerations. Unsupported/no-header raw 0.2 remains readable historical evidence. Historical raw 0.1 numerical standing reports absolute rounding and absent assessment without mutating the carrier.
- Numerical eligibility requires the actual supplied model load cases, unique complete per-case coverage, passive/retained equation evidence, passing or consistent sensitive status and nonempty references to unique actual emitted IDs. Missing/duplicate/empty IDs, unsupported headers, unassessed, failed and unresolved cases do not qualify. It is not engineering qualification.
- Current workspace result additionally requires analysis 0.3, source/model/run/manifest/semantic/solver identities. The Current export performs canonical manifest/result/row/analysis hash checks and new source interpretation validation. Manager owns the enclosing solve/history/shell joins.
- Explicit analysis V03 builder and source validator bind the exact frozen semantic ID/hash plus row signature, index, ref, interpretation and annotation. Ordinary Preview composition rejects legacy sources; explicit historical V02 builder retains old synthetic no-header fixture reproduction. Checksum-only verification remains separate.
- Canonical derivative 0.3 preserves exact producer, numerical quality and formulation metadata, source bindings and binary64 values. Historical raw 0.1 retains derivative 0.2 projection; old derivative version reader support remains.
- Stress-neutral 0.3 uses ops.stress_neutral.v3, the same nine member streams, exact metadata and semantic tuple, actual received source carrier checksum and package hash binding. New construction validates analysis/source/numerical standing; standalone validation preserves historical 0.2 checks and does not authenticate an external producer.
- Rule-check service forwards both model and solved envelope for native requested-case coverage. Existing panel already supplied both.
- QuantityReadout was inspected and already renders the raw JavaScript Number. No units implementation was changed. Added signed tiny/edge-value UI checks and shared 26-vector same-unit bit tests across JSON, canonical hashing, derivative and stress CSV/JSON. Synthetic gate fixtures are consumer-only mocks, never native physics evidence.

## Ready checks — not executed

Run from `projects/chirality-piping/apps/desktop` only after the resource grant:

```sh
npm exec -- tsc --noEmit
npm exec -- vitest run src/services/previewService.test.ts src/services/analysisRunCompatibility.test.ts src/services/ruleCheckService.test.ts src/features/result-export/resultExportAdapter.test.ts src/features/results/resultSemantics.test.ts src/features/results/numericalResultQuality.test.ts src/features/results/ResultsPanel.test.tsx src/features/results/HistoricalRunContext.test.tsx src/features/workspace/resultsSessionState.test.ts src/features/stress-neutral/StressNeutralExportPanel.test.tsx --maxWorkers=1
```

No Cargo, npm, build, browser or native tests were run by this child. The inherited WASM/fixture prerequisites and connected manager/native checks must be satisfied by the allocated lane. TypeScript and test failures remain possible until this provisional candidate is exercised. Independent frozen-diff review and affected connected/native checks remain required. Broader tests that formerly treated legacy browser fixtures as Current must retain historical expectations or use the separately owned new producer-generated fixture; they must not promote old values with passing headers.

## Shared frozen inputs

- `fixtures/results/semantic_contract_v0_3_precision_1.json`: SHA-256 `d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e`.
- `fixtures/results/precision_transport_v0_3.json`: SHA-256 `fc06b4a54073bbbdf933128360fc29454a0844ee0129d6b41728f60176b62897`.
- Prospective V2 consumer contract adopted from manager directions: raw/product 0.2; derivative/analysis/stress-neutral 0.3; model remains 0.2. Historical instruction bytes and their original semantics remain preserved.

Source is frozen at this return. No commits or Git mutations were performed; no other worker's changes were staged, reverted or repaired. Additional edits require a notified review/test repair. Parent integrates and verifies the candidate; this return does not claim work outside its listed files.

## F2 review repair

Fresh review identified that a correctly rehashed analysis record could name another model state. `validateAnalysisRunV03` now requires the exact ModelState object/ref for `state:${source.model_ref}:preview`. This also protects stress-neutral 0.3 construction through its existing shared validator call. Added correctly rehashed wrong-model and wrong-object-type analysis negatives, plus a correctly rehashed wrong-model stress-neutral construction negative. Checksum-only verification intentionally still reports the recomputed checksum as matching; source interpretation validation rejects it.

Only `services/analysisRunCompatibility.ts`, its focused test, and `features/stress-neutral/StressNeutralExportPanel.test.tsx` changed for F2. Their hashes are refreshed in `SOURCE_HASHES.json`. No tests were run under the continuing resource hold; independent reviewer backcheck remains required. Source is refrozen pending that backcheck.

## Allocated-check fixture repair

Manager-reported allocated checks passed TypeScript, the candidate WASM build, unit/display checks and eleven focused test files. The result-export suite had nine failures at the numerical Current gate. Inspection of the captured Vitest failure log confirmed the common gate rejection. Manager diagnosis identified four duplicate historical diagnostic IDs in the synthetic consumer helper.

Only `features/result-export/resultExportAdapter.test.ts` changed: its explicitly synthetic admission helper now gives every copied diagnostic a unique test ID that retains its index and prior ID. Every warning message, code, severity, load reference and other diagnostic field remains present; the imported historical fixture is untouched. The helper and suite explicitly disclaim new browser-demo, producer and numerical qualification. Production guards and existing causal assertions are unchanged.

This child ran no checks during the repair. The affected rerun remains pending the manager's allocated slot. The updated file hash is recorded in `SOURCE_HASHES.json`; source is refrozen.
