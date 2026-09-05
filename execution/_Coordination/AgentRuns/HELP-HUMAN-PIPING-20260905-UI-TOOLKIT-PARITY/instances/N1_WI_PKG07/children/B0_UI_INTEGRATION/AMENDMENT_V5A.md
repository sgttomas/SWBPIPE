# Sealed B0 v5A — accepted persistence disposition
Parent accepted N2_WI_PKG16/BATCH_PERSISTENCE_DISPOSITION_V2.md. Read exact disposition. Same shared source fence; no native schema/migration.

Separate stored proposed review context from live pending. Only after successful generation/hash-guarded single OR batch model commit append exact original operation members, all author/source/rationale/before/after/flags unchanged, to retained review context; remove pending separately. Clear Queue invalidates requests and clears pending only, never retained. Save existing editor_intents array includes retained+pending atomically with saved model. Preserve live queue across Save response, do not assign returned combined records back into pending.

Load restored operation records as acceptance UNKNOWN distinct review context; explicit user requeue + fresh validation only, no implicit pending/accepted/receipt/auto-replay. Actual grouped batch receipt/session undo remains session-only. No full SOW070 closure claim. Optional session receipt export only if straightforward existing privacy-controlled route; do not delay core metadata preservation.

Tests save/load exact agent+user metadata, clear queue retains context, restored context no receipt/auto-replay, explicit requeue, stale responses append nothing, singles consistent with batch. Preserve async withdrawal/generation guard for every new asynchronous save/load interaction.

V2 exact coherence: same combined retained+pending snapshot must feed BOTH project envelope hash and native Save request; native JSON array preserved exactly. Never calculate envelope hash on pending-only while saving combined context.

Retained context survives explicit requeue; later Clear Pending never removes original retained record. No dedup by operation_id, which may recur across distinct source records; preserve sequence+identity or exact canonical equality.

Exact renderer fence addition authorized by manager: features/model-workspace/modelView.ts plus relevant tests for typed selectedProperties quantity metadata, retaining source values/units/dimensions and existing caller compatibility. No parsing formatted strings or duplicate label mappings. This helper is exclusively B0-owned.
