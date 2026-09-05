# Additive sealed amendment v2A — session race integrity
Parent HELP_HUMAN explicit direction, manager confirmed live defect. Same source fence and role.

handleApplyIntent currently awaits engine then writes outcome/receipt/checkpoint/model without checking intervening load/undo/edit. Use synchronous existing modelRevision/commitModel seam plus initial model hash to guard final renderer commit. Invocation captures session/model generation and basis; before any current-session outcome, receipt, checkpoint, queue removal or model commit, reject stale results. Busy/error/finally must not let superseded request clear or modify newer invocation. Apply same invariant to future batch consumer.

Meaningful deferred-promise test: begin apply, replace or undo model while pending, resolve successful old engine outcome; new session/model remains, no old receipt/checkpoint/queue removal occurs. Maintain existing shared engine mutation route. No disabling navigation as sole fix. B2 backend now reviewed accepted at N2_WI_PKG16/B2_ACCEPTED_SNAPSHOT.json, but module integration after B2 terminal return.

Parent B4 V3 clarification: compute authoritative initial model hash, then after successful engine response rehash current unchanged-generation model and compare request initial hash; recheck generation after this second await. Initial hash state being temporarily null does not permit stale fallback. Every load/new/undo/redo/edit invalidates synchronously. No result from outdated request may clear newer busy/error state.
