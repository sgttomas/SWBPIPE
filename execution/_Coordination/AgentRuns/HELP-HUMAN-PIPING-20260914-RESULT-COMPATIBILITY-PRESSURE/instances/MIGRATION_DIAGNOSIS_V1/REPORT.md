# Native migration integrity diagnosis V1

**Disposition: narrow repair recommended; Agent 0 accepted the optional ledger evidence contract in-session.** This is a completed read-only diagnosis, not implementation, full candidate review, or qualification. Candidate: `6bb26b118fc038c451b97d3b86e3ae27d8ba8e91`. Evidence captured at `2026-09-15T02:16:00.063731+00:00`; source/context hashes are in `INPUT_MANIFEST.json`. The instance remains ephemeral Agent 2, configured gpt-6-astra/high, reporting to HELP_HUMAN with no delegation.

## Exact defect and reproduction

The early native evidence is correctly BLOCKED. First-save stored model claim is `sha256:b2cec8ee447c44fa7efbdf43c0970a5c690b8c63e24577a105ce12e7f613ccc3`, while the actual stored model hashes to `sha256:405db1a2316c1a937c308a91966f72f22de4018a156bb13dffa7160c67e9d3a2`. Stored envelope claim is `sha256:555ace20b7db1987bf9a887617988b3dfa57b1766a73f1892bf953b8c31101b8`; its actual seven-field projection, retaining the received old model-hash carrier, hashes to `sha256:c995d111b094445aa417b4db23410ec0c47d1ad6ef0bb027e69ab375f6dd7529`.

Independent comparison of captured synthetic JSON shows the persisted model differs from the maintained bundled model **only in `schema_version`**. Its native serialized model text hashes to `405db1…`; reversing its single `"schema_version":"0.2.0"` token to `0.1.0` hashes to `b2cec8…`. These two independent byte hashes agree with the separately captured exact-candidate WASM witness. This calculation is not a new JCS implementation or product execution. Details: `INDEPENDENT_COMPARISON.json`.

Source trace:

- `App.tsx:1391–1443,1586–1637` chooses model/envelope hashes before native persistence, and adopts only returned summary/metadata, not the returned model.
- `src-tauri/src/lib.rs:1936–2030` evaluates and replaces an old model with its migrated successor. Lines 1960–1964 derive migration pre/post fields from received/prior **claims**, rather than the actual source/final model payloads.
- `create_local_project` and `save_local_project` at 2034–2233 pass the transformed model and unchanged incoming hash carriers together to `upsert_project`. Its transaction faithfully stores that inconsistent tuple.
- `model_document_migration.rs:28–44,216–220` has an established shape-preserving 0.1→0.2 step; the walker changes the version token, which is inside the existing model checksum projection.
- The captured first-save row contains two migration records, both advertising the old model digest as post-migration. The stale in-session 0.1 model and unconditional open-status migration branch permit repeated misleading records.

Analysis 0.2, received-result and all row claims passed the original witness; their payloads are not the defect. Fresh solve/save on the already-normalized model passes, but does not verify or repair the failed first-save claims. Keep that failed evidence intact.

## Recommended behavior

1. **Initial create/save that changes the persisted model.** Prepare the final normalized model and normalized attachment values first. Build new model and project-envelope evidence over exactly the values to be stored. Use the existing `open_pipe_stress_operation_applier::canonical_json` and `sha256_hex`, with the same `rfc8785_jcs` label, `sha256:` prefix, scope/ref/status fields and seven-member envelope projection used by `hashService.ts`. The envelope includes the newly computed model-hash object. Keep received mechanics/analysis and their claims untouched. Preserve the original received model/envelope claims separately in the new migration ledger record before replacing top-level carriers with the new post-normalization evidence. Persist model, attachments, both new carriers and ledger atomically through the existing transaction.
2. **A native response actually changes the in-session model.** After existing request/epoch checks and any awaited context preparation, commit the returned model through `commitModel`; invalidate rule-revision/solve/operation ownership, clear pending actionable drafts and old undo/redo checkpoints, and reset Current result/analysis/manifest state using the established open replacement pattern. Preserve returned mechanics/analysis as readable Historical context, retain saved review context, and reconcile selection. Do not merely assign `setModel` or retain Current against the pre-normalization manifest. Refresh the local epoch after this intentional commit before later guarded updates.
3. **Essential same-model edge: open-time migration followed by save.** Open may already have returned model 0.2 while keeping old model/envelope claims. Request.model and response.model on its first save are therefore equal, but native persistence is committing the old stored model's migration and returns new post-normalization hash evidence. **Refresh the existing Historical context from that returned saved envelope even without `commitModel`.** Otherwise its old `historicalRun.modelHash` is reused on the next save and recreates the mismatch. Preserve raw result/analysis, keep designation Historical, and preserve the received originals in the ledger. Bind refresh to the same historical context/request ownership; a later fresh solve on the same model must not be overwritten by stale Historical context.
4. **Unchanged Historical save on an already-current stored model.** Preserve received result/analysis and hash objects exactly, including absent, invalid, contradictory or mislabelled claims. Do not recompute just because a received digest mismatches, and do not append a ledger record solely because retained `model_document_migration.status` says `migrated`. Refreshing Historical context from an unchanged response is safe under ownership guards; it must not invent evidence.
5. **Already-migrated historical open/save.** Open remains read-only and returns old claims unchanged. A stored 0.2 record with the known first-save inconsistency must continue to disclose it; this repair does not retroactively heal it. A subsequent unchanged save preserves it. The existing fresh solve creates new session evidence prospectively.
6. **Stored 0.1 historical open/save.** Open keeps the existing in-memory-only migration policy and does not rewrite the stored original. It cannot make old claims verify a changed in-memory model; retain migration context and the original claim findings. On the first actual migration save, use the old stored document as available source basis, preserve its claims and the incoming claims, then produce new post-normalization evidence. Subsequent saves are case 4. No historical input-manifest payload is fabricated.
7. **Delayed responses.** Keep the `projectRequest` and model epoch checks before/after every await. Changed-return adoption invalidates late solve, rule and operation responses against the old model. An obsolete create/save response cannot replace a later model or clear newer busy ownership. Same-model Current saves must not call `commitModel` or downgrade Current. Same-model Historical refresh must be conditional on the still-owned historical snapshot (for example identity-bound state replacement), so a fresh solve that completes while save is pending wins.

The native helper should recognize a real transition from either (a) incoming source document → normalized target, or (b) actual prior stored old document → incoming already-normalized target after open. Validate that source against the published migration chain. A caller's status flag alone is not proof of a transition. Append only for a real transition; do not recreate entries on unchanged repeat saves. Use the same chosen source when computing the ledger pre hash: incoming old document for case (a), prior stored old document for case (b). Never use a copied prior hash claim as the computed preimage digest. If a purported source is unavailable, disclose that limitation rather than manufacturing it.

## Accepted narrow ledger evidence semantics

Agent 0 accepted the following interpretation in-session and will issue the writer release. This is an additive optional member on **new** records in existing `model_migration_ledger_json`; no old entry is rewritten. Suggested member name/shape:

```text
hash_evidence: {
  schema: "model_migration_hash_evidence_v1",
  source_payload_basis:
    "incoming_pre_migration_model" | "stored_pre_open_migration_model",
  received: {
    model_hash: <exact received JSON value>,
    project_envelope_hash: <exact received JSON value>
  },
  prior_stored: null | {
    model_hash: <exact prior stored JSON value>,
    project_envelope_hash: <exact prior stored JSON value>
  },
  computed: {
    pre_migration_model_hash: <full actual-source model checksum>,
    post_migration_model_hash: <full actual-final model checksum>,
    post_migration_project_envelope_hash: <full actual-final envelope checksum>
  },
  received_claim_verification: "not_asserted"
}
```

The existing outer `pre_migration_model_hash` and `post_migration_model_hash` strings on a **new** record agree with those computed digests. `prior_stored` distinguishes evidence originally read from storage from claims received in the save request; it is null for no prior row. Capture raw received carriers before `normalized_model_hash`/`normalized_project_envelope_hash` discard nonobjects. Missing/null evidence stays missing/null, and bad algorithms, historical labels, extra fields and wrong values remain unchanged in the received carrier. Computed checksums describe new actual payloads; they do not verify the received claims. The suggested version identifies optional evidence metadata, not a new model, database or wire version.

No additional full source/result snapshot is required for this repair. Where an original preimage is not retained, the original claim remains unverified; do not imply that a preserved checksum object alone enables future verification. The ledger itself remains existing application evidence, not a newly authenticated chain.

Compatibility basis: native `LocalProjectEnvelope.model_migration_ledger` and stored records are `serde_json::Value`; upsert/load serialize and parse the entire value. TypeScript's `ModelMigrationLedgerRecord` is a structural type, not a runtime exact-property validator. `projectService.ts:310–324` JSON-clones the ledger without projecting away members. `ProjectValidationPanel.tsx:578–599` exports ledger records verbatim and only counts them for its summary. No strict ledger-entry schema/reference was found in the scoped schemas, persistence core or tests. Add an optional typed member for new evidence, keeping old ledger entries readable. There are no new request fields, commands, columns, database versions or hash projections.

## Exact six-path writer release

All paths below are relative to `projects/chirality-piping/`:

| Path | Bounded change |
|---|---|
| `apps/desktop/src-tauri/src/lib.rs` | Shared preparation of the final persistence tuple; existing-profile hash construction; original claim capture; actual-transition/idempotence logic; create/save integration; meaningful inline native tests. Preserve `upsert_project` transaction behavior. |
| `apps/desktop/src-tauri/src/model_document_migration.rs` | Optional new-record evidence construction and focused constructor tests if factored here. Keep the migration chain/version unchanged. |
| `apps/desktop/src/App.tsx` | Guarded changed-model response adoption and same-model Historical evidence refresh for create/save, using existing replacement/currentness infrastructure. |
| `apps/desktop/src/App.test.tsx` | Realistic native-normalization responses, History/Current behavior, repeat-save preservation and delayed response regression cases. |
| `apps/desktop/src/types.ts` | Optional typed ledger evidence member; received/prior JSON must represent malformed values without narrowing them into valid checksum types. |
| `apps/desktop/src/services/projectService.test.ts` | Roundtrip preservation of old ledger entries and extended metadata; retain browser behavior and unchanged invoke command/request fields. |

`hashService.ts`, `projectService.ts`, canonicalizer sources, operation sources, model fixtures, schemas, database migration tables and `HistoricalRunContext.tsx` need no behavior change for this proposed repair. If implementation reveals a concrete necessity, route it to root before extending the exact fence.

## Meaningful regression triggers

- First create and first save of the maintained 0.1-shaped model, with real pre-normalization model/envelope claims: final stored/returned model and seven-field envelope recompute correctly using the unchanged authority; received claims survive verbatim in ledger; ledger actual pre/post digests differ; results/analysis remain equal. Do not rely on placeholder `sha256:post-migration` strings used by current tests.
- Seed an old stored 0.1 document, open normalized in memory, then save twice: first saves the migration/new evidence; second preserves complete carriers and ledger. App refreshes Historical model/envelope evidence even though both save request and response model are 0.2.
- Seed already-current 0.2 history with deliberately wrong/mislabelled model/envelope claims and old ledger entries: open/unchanged save preserves all of them and truthful findings; no migration event or synthetic repair. Include null and nonobject received carrier preservation in the migration evidence path.
- A changed native save/create response after a fresh solve clears Current evidence, retains the submitted received result/analysis as Historical, and prevents old undo/redo or late operation/rule/solve completions from restoring the old basis. A normal same-model Current save keeps Current.
- Delayed save after another model opens is inert; delayed same-model Historical save after fresh solve completion cannot reintroduce Historical or overwrite fresh session evidence. Existing `App.test.tsx:16378` is the adjacent stale-save regression to retain.
- Old and extended ledger records survive native load/response and frontend clone/export without dropping extra members. The browser's documented no-ledger/no-persisted-migration behavior remains unchanged.

After repair root owns targeted checks, revised native first-save/reopen/unchanged-save witness, final complete diff review and existing qualification. Fresh recovery is not sufficient. No extra broad gate is introduced by this diagnosis.

## Evidence and handoff

`INPUT_MANIFEST.json` binds 34 inspected source/context/evidence files. `INDEPENDENT_COMPARISON.json` records the independent synthetic-data comparison. The original native manifest and after-GUI source binding are referenced there; no original evidence was copied over or changed.

This derivative report consumes the approved plan/D-67, source-release preservation interpretation, candidate `6bb26b1`, and blocked `EARLY_NATIVE_V1` evidence. Bounded diagnosis is complete; the defect remains unremediated here. Root's optional ledger interpretation is accepted for the source release, not implemented or qualified. No remaining user decision is needed. The sole compatibility writer owns repair; root owns acceptance. No product edits, tests, builds, GUI, network, Git mutation, application-store access or delegation occurred. Only already-captured synthetic JSON was analyzed, and all writes are inside `instances/MIGRATION_DIAGNOSIS_V1`.
