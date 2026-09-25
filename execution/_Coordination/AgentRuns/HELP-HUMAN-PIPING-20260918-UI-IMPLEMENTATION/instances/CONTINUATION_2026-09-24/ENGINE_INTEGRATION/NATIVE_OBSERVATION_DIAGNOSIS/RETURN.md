# Historical input-manifest observation — diagnosis

`HISTORICAL_INPUT_MANIFEST_MISSING` is expected under the current local-project persistence/history contract. The two supplied native saves retain their AnalysisRun0.3 records and the manifest reference/hash claims. They do not retain the manifest body. This is an explicit existing capability boundary, not evidence that native Save/Open dropped a body promised by this contract. No source repair is indicated for the selected native witness.

## Actual saved evidence

EVIDENCE.json binds the private sparse/dense captures by SHA256 without copying either native model or operation history. Both contain raw0.2 and AnalysisRun0.3, matching raw/analysis run identity, exactly one InputManifest reference and one matching SHA256 claim, and the reported model hash a335271236b6dbbcb3c807cd5bf040a33726c4dfd67aa33ec37a0f26f03483ce. Sparse manifest claim is c3e97ceaa93773e6ec9fea8f2f2ec7d2363dca4c4d0e5ae98d9010dcf02d64a8; dense is d1bdb14061aa58578452acd6715c014ffa4f6cff97ea3da9dc5ed6608116828c. Each claim matches its reference and reference suffix. Neither capture nor analysis record includes a manifest payload. Stored hash claims are present; this structural diagnosis does not newly recompute all of them.

## Causal chain and contract

1. workspaceSession.ts:853 builds a CurrentSessionInputManifest after a real completed native invocation and stores it in transient session state. inputManifestService.ts:59/108 explicitly declares package inclusion=false, portable replay=false and current-session ref/hash integrity only. The corresponding test:66 requires those limits.
2. analysisRunCompatibility.ts:113 stores the manifest reference/hash in the analysis record, not the body. AnalysisRun0.3 schema:530 defines a closed reproducibility object with reference/checksum fields; it has no embedded manifest field. The actual native records match that representation.
3. saveLocalProject at projectService.ts:581 and the native SaveLocalProjectRequest/LocalProjectEnvelope at lib.rs:519/502 have no manifest payload parameter or field. Native upsert_project at lib.rs:976 persists model, received mechanics, analysis and scoped hashes; its SQL has no input-manifest column. The frontend LocalProjectEnvelope type at types.ts:1013 agrees. This is the earliest persistence boundary: the transient body is deliberately outside the save DTO, rather than being lost in SQLite or the restart.
4. Opening at workspaceSession.ts:1826 creates historical context, clears live result/analysis and explicitly sets the current manifest to null at1858–1861. Its comment states that stored references do not contain the exact payload needed to recompute its hash. Undo/Redo are cleared separately at1846–1847.
5. HistoricalRunContext.tsx:10 describes the missing historical payload. buildHistoricalRunContext at276 unconditionally starts with HISTORICAL_INPUT_MANIFEST_MISSING for received historical evidence, before independent model/result/analysis/envelope checks. The reason is not a claim that the manifest reference/hash is absent or mismatched. HistoricalRunContext.test.tsx:130 and173 expect that reason while preserving valid received hashes and asserting no invented inputManifest.

Confidence is high: the observed fields, save DTO, database upsert, reopen branch and focused contract tests agree. A malformed analysis or corrupted hash would be a separate finding; this diagnostic by itself does not establish either. No native rerun was needed to trace this deterministic branch.

## Acceptance consequence and limits

The current tranche's native save/reopen witness—preserve the actual model, received raw result, AnalysisRun, scoped hashes and original association, then show Historical and withhold fresh-only use—is consistent with this observation. The missing-manifest reason alone does not invalidate the completed numerical solves or require a repair before continuing the remaining native witnesses. The persisted refs/hashes remain useful received evidence, but cannot independently verify an unavailable manifest preimage.

Do not claim complete historical input-context recovery, portable replay, resumed fresh Current, exact executable custody or full DEL-14-02 persistence conformance. DEL-14-02 ScopeOfWork CLM-005/006 and delivery commitment DEL-14-02:1 already leave producer/persistence conformance and exact source/run basis as broader delivery work. The current guarded view is not closure of that work. No new assignment, acceptance waiver or automatic repair scope is created here.

If full manifest persistence is later selected, it needs a coherent saved member/DTO/store/hash/version/reopen contract and an actual Quit/Open integrity witness. Rebuilding a manifest from the currently opened model/default settings or merely deleting this reason would not recover the original preimage. Even a verified persisted body would not by itself recreate the private native invocation registration required for fresh Current. This is a future implementation option, not work performed or authorized by this diagnosis.

Only this small diagnosis directory was written. Source remains at the frozen main-union candidate; no source/build/native/Git operation or delegation occurred.
