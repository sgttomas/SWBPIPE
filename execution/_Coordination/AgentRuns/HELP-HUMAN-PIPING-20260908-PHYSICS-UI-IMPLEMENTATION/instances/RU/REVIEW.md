# RU independent U7 product-diff review

Verdict: **CHANGES_REQUIRED**

The frozen 2,280-line, nine-path U7 diff was reviewed in full against source base `55df51ac3201456e0f181823e3aefefef47a73bb`, the sealed U7 brief, consumed-interface evidence, seven-row proof, and current operation, hash, persistence, unit, section-binding, and toolkit contracts. All 19 members declared by the frozen manifest matched their SHA-256 bindings. This is technical review evidence only; it is not source, dependency-row, lifecycle, native, engineering, or release acceptance.

## Confirmed defects

### RU-F1 — Cancel remains active during Apply and does not cancel App publication

**Location:** `apps/desktop/src/features/viewport/PipeViewport.tsx:605-627`, especially the local-only generation check at `:612`; `:1251-1256` leaves Cancel enabled while `draftReviewBusy`; App publication ownership remains in `apps/desktop/src/App.tsx:1050-1060`.

**Impact:** A user can click Apply, click Cancel while the service is pending, and still receive the node, pipe, and undo checkpoint when the delayed batch completes. This violates the frozen requirements that controls be disabled while Add/Apply is in flight and that a canceled/inactive route cannot publish from a delayed callback.

**Evidence:** RU's private App-level regression delayed `apply_model_operation_batch`, clicked Cancel, resolved a valid atomic receipt, and observed both records plus `1 undo / 0 redo`. The reproduction passes because it asserts the defective result.

**Remediation direction:** Disable the route controls, including Cancel and draft-changing selection/input controls, for the full Add/Apply flight. If any cancellation path remains available, it must synchronously invalidate the App request sequence/epoch before a response can publish. Add a delayed-Apply regression that requires no model publication or checkpoint after every permitted invalidation path.

### RU-F2 — Direct reviewed Apply can publish an incomplete or insufficiently bound receipt

**Location:** `apps/desktop/src/App.tsx:1071-1078` for batch and `:1117-1125` for single-operation publication.

**Impact:** The single-operation path requires an applied model and status but does not require a complete acceptance record or validate returned target/model-basis evidence. RU returned the exact operation/change IDs with `acceptance: {}` and observed the node publish, one checkpoint appear, and the ledger render empty acceptance fields. The batch path requires a truthy acceptance object but does not compare returned `initial_model_hash`, `submitted_operations`, or per-step identities/diffs to the frozen batch before publishing. This leaves the Apply response less strictly bound than the preceding validation and violates the missing-receipt/hash-mismatch fail-closed contract.

**Evidence:** Private App regression `observes a single reviewed Apply publish with an incomplete acceptance receipt` passed. Static trace confirms the omitted batch return bindings while the consumed `OperationBatchOutcome` exposes them.

**Remediation direction:** Before any receipt/checkpoint/model state write, require the complete acceptance shape and exact apply-result binding for the frozen single intent or batch, including model-basis/hash evidence and the operation/change/target or ordered submitted-step identities applicable to that response. A missing/malformed field or mismatch must return false and require a new Add. Add adversarial single and batch regressions.

### RU-F3 — Successful App publication clears requested pipe continuation

**Location:** `apps/desktop/src/features/viewport/PipeViewport.tsx:291-298` resets continuation and the pipe draft for every model prop change; `:630-637` tries to continue from the committed endpoint.

**Impact:** With “Continue from end after Apply” checked, a successful route commit changes the App model. The model effect then clears `continuePipe` and resets `from`, overriding the successful continuation state. The next route cannot begin from the committed endpoint as the visible control promises and as the frozen continuation invariant requires.

**Evidence:** RU's App-level reproduction applied an existing-end route through the real browser WASM service and observed the checkbox unchecked and `from` empty after commit. The existing continuation test at `App.test.tsx:15687` renders `PipeViewport` without `onAddDraft`, so it exercises only the queue fallback and cannot detect the App-path regression.

**Remediation direction:** Preserve the continuation request and successful endpoint across the component's own accepted commit while continuing to reset it for external model replacement, open/create, undo/redo, and Cancel. Add an App-level Add/Apply continuation test.

### RU-F4 — Reservation validation accepts malformed targets and misses collisions inside the new batch

**Location:** `apps/desktop/src/features/viewport/routeDraft.ts:398-404` accepts a create/connect/insert record with only a string `target.ref`; `:124-131` constructs the two-member batch after independent checks at `:165-170`.

**Impact:** Restored `{operation_kind:"create", target:{ref:"pipe:RU-malformed-reservation"}}` metadata with no `target.object_type` is malformed but still reserves the pipe ID, contrary to the required runtime kind/target/reference gate. Separately, a new node and pipe can be given the identical ID because neither is added to the reservation set before validating the other; the builder emits both batch targets with that same ref. This can create ambiguous global entity identity and defeats the builder's collision policy.

**Evidence:** Both pure private probes passed while asserting the defective outcomes. The final production unit test covers null/scalar/empty/invalid-ref records but not a string ref on an incomplete target or intra-submission collision.

**Remediation direction:** Treat a pending reservation as valid only after the complete target discriminator required by the runtime contract is present and consistent. Validate the current new endpoint and pipe IDs against each other before emission. Add both boundary cases to `routeDraft.test.ts`.

### RU-F5 — Required provenance is prefilled with a false user-entered default

**Location:** `apps/desktop/src/features/viewport/PipeViewport.tsx:1515-1516` and `:1547-1560`; `routeDraft.ts:147-148` and `:184` only require a nonempty value.

**Impact:** Node, new-endpoint, and pipe drafts begin with `user_entered_local_preview`, so Add can succeed without the user entering provenance. The value claims user entry although the UI manufactured it. This conflicts with the accepted explicit-provenance and no-invented-semantic-default contract. The pipe prefill predates this tranche, but the new Add/Apply flow now accepts and publishes it; prior origin does not satisfy the new flow's acceptance criteria.

**Remediation direction:** Start required provenance empty and keep Add disabled until the user enters it. Pointer placement may fill coordinates as a draft aid but must not synthesize semantic provenance. Add untouched-default rejection coverage for node and route creation.

## Evidence gaps and passing evidence

The frozen route, inspector, selected App, and browser tests are green, but they do not cover RU-F1 through RU-F5. Add tests for delayed Apply cancellation, App-path continuation, malformed/incomplete apply receipts and model-basis mismatch, incomplete target reservation, intra-batch ID collision, and untouched provenance defaults. A top-level warning is rejected by implementation logic, but the new direct-route tests do not independently exercise the warning path; add it with the receipt regressions.

The final-cut Playwright journey passed and asserts a 1024×768 viewport, no body horizontal overflow, a persistent canvas larger than 300×250, and an in-bounds authoring card. The retained screenshot was visually inspected only as the disclosed pre-remediation browser artifact. It remains neither final-cut nor packaged-native evidence. The separate native walkthrough remains pending and must stay held while this verdict is unresolved.

Positive checks: exact `[create_node, connect_pipe_run]` order, one-checkpoint happy-path publication, zero coordinates, object-key-insensitive/array-order-sensitive structural equality, normal malformed-null preservation/requeue/fail-closed behavior, inline inspector forwarding, incomplete node persistence, seven-row fields, compact palette, and final browser layout all have supporting source or focused test evidence. These passing areas do not offset the confirmed contract defects above.

## Residual risk and fan-in disposition

Residual risk after repair centers on asynchronous state ownership at the React/App boundary and untrusted runtime receipt shapes. The native route and full registered checks were outside RU's allowed execution and remain downstream gates. This return is valid technical fan-in evidence with **CHANGES_REQUIRED**; it is not valid as the required independent PASS for publication.
