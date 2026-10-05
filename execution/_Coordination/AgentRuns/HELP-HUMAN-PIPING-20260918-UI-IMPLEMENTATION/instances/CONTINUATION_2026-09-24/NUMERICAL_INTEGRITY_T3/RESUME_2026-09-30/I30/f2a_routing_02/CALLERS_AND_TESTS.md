# I30 — caller ownership, finite native reply option and checks

Revisable interface proposal paired with ROUTING.md. P-relative anchors below
refer to main49034a940f. No native byte profile or overlap count is adopted here.

## 1. Qualified caller roster and completion ownership

| Caller/profile | Actual objects that coexist; proposed reservation owner and end |
|---|---|
| Typed PP / typed HL | PP lib.rs:2141 passes capture=None; HL:804 uses it. Remain ordinary/exact behavior, without fabricated raw custody or W1. No new completion permit can turn typed data into a captured request. |
| Captured direct PP | Actual Value, CapturedInvocation raw/digest/encoded length and parsed/normalized request; ordinary prefix/base and private draft. A DirectPp completion context covers this call and its returned MechanicsEnvelope. End: transfer of that envelope to the caller; arbitrary later clone/serde/library use is outside this **named direct-only** window. |
| Captured headless | HL lib.rs:733–753 retains solve_payload, actual_invocation object, cloned RunnerRequest and PP's input clone; then mechanics/runner, temporary Values, checked and legacy canonical strings/hashes, result-export document, QualifiedPreviewEvidence and PreviewRunnerOutput. Caller-owned context remains live across PP, qualified export and final output assembly; end: transfer of PreviewRunnerOutput. Its later public Clone/serialization by arbitrary consumers is outside this profile. |
| Native direct | Native lib.rs:1557–1563 owns request Value, then MechanicsEnvelope **while to_value consumes/serializes it** into a second owned tree; then command response tree and Tauri's serialization String. NativeDirect context must cover all of these, and be held by an owned response wrapper until the explicit IpcResponse::body ownership transfer described below. Command return alone is not its end. |
| Native job worker/publication | Native:1675–1728 owns model payload/mode and jobs Arc, PP result, envelope→Value conversion and outcome Result; publication moves Value into the record. Job-owned completion lease moves with that result, not with worker completion. A cancellation request is only a flag until the existing checkpoint. |
| Native job retention/replies | Native:1629 records retain Value indefinitely; :1748 deep-clones on every poll. Proposed per-W1-job lease covers the resident record result and at most K admitted overlapping reply copies/serializers. Each reply has its own guard; the resident lease is released only when its record/table's **actual last owner** drops. Current code removes no terminal record on polling, cancellation-after-terminal or TS job-map cleanup. |

The routing entry must receive an explicit closed caller descriptor and completion
permit for DirectPp, Headless, NativeDirect or NativeJob. No generic Default may
silently qualify native as DirectPp. Proposed minimal seam: a context-taking PP
entry used explicitly by HL and native; the existing public Value API may select
only the separately qualified DirectPp profile for its documented return window.
If that profile is not qualified, it preserves ordinary behavior and declines W1.
Native/HL source-call tests must forbid their calling the direct-only wrapper.
An unknown/unqualified caller profile cannot produce a W1-selected response.

Correction to RV41's headless allocation observation: HL:931 calls
attach_result_envelope_document, but its current :960–970 body returns None even
for solved mechanics (`let _ = (request, runner_result); None`). There is no current
provisional document allocation. P4 counts the later actual qualified export and
must re-open this cell if that helper changes. The call site alone is not a live copy.

## 2. Native job transitions and actual last owners

| Event | Existing behavior preserved; lease/result consequence |
|---|---|
| start job | Allocate existing record/id/token and spawn one worker (:1646–1698,1800–1831). Admit the native W1 profile before W1-added ownership, not by changing ordinary job-start acceptance. The input/registry objects already resident still enter its phase census where the metric includes them. |
| cancel before solver start | Existing worker checkpoint sees flag, marks cancelled and returns; no solver/W1 work runs. Drop that worker's W1 preparation lease, if any. |
| cancel while PP/W1 runs | Existing cancellation does not interrupt numeric work. Keep all reservations until the real work returns; do not release when the flag is set or when the UI abandons the generation. No new cancellation architecture is proposed. |
| publish with cancellation_requested | :1712–1721 discards Ok(Value), or retains outcome.err() in the cancelled record. Drop the Value and its result lease only after actual destruction; retain/count any bounded error payload. No cancellation success claimed before this checkpoint. |
| publish success | Move Value into the one record, state completed, then transfer the resident lease to that record. The worker and its Arc can drop; the record still owns its Value. |
| poll queued/running | Existing small status, result=None. Preserve ordinary behavior; no large terminal-result clone exists. Bound W1-specific metadata if added. |
| poll completed W1 | Acquire a reply slot **before** result.clone; hold it through the status snapshot and serializer/transfer. Registry result stays resident. Repeated sequential polls each create/destroy a new reply, not one lifetime copy. |
| concurrent terminal W1 polls | Current Mutex only serializes the clone operation; detached copies can overlap indefinitely. A finite profile requires the enforcement below. Naming K in a document is insufficient. |
| cancel after completed/failed/cancelled | :1773 returns accepted=false and changes no record; do not release result/lease here. |
| record/table teardown | No current explicit eviction/removal exists. Record Value survives until the table's last Arc owner (app state/workers/other actual owners) drops. A detached guarded reply may outlive that table; its copy and guard survive independently until serialization/drop. No cleanup feature is necessary for this per-invocation bound. |

Registry capacities and any new lease metadata are real P4 terms, not a claim
that every record allocation belongs exclusively to one job. P4 must state how the
shared HashMap backing/current resident bytes are included, and how concurrency
from other independent jobs is excluded from this per-invocation metric. No bound
on all retained jobs, all threads, process heap, allocator overhead or RSS is claimed.

## 3. Small enforceable reply option, and its remaining choice

**Recommended derivation target:** keep current Value storage and deep-clone wire
behavior. Add a W1-only resident lease with a finite reply-slot counter and a private
`AdmittedNativeReply { report_or_value, reply_guard }` that implements IpcResponse
and does **not** implement Serialize/Clone. It is a local response owner, not a new
registry/transaction framework. Preserve the existing ordinary response path.

K is a positive, checked profile parameter selected with the native allowance.
Only completed W1 records require a slot. The slot is acquired atomically before
the deep clone and remains owned by the response object after the command returns.
No slot means a small, specific retryable W1 reply-admission refusal, no result clone,
no record mutation, no solver rerun and no substitution of an ordinary result.
This is a **concrete proposed W1 API behavior requiring ROOT/owning disposition**;
it is not an existing concurrency guarantee or permission to refuse ordinary polls.
Ordinary/exact-source records continue their present poll behavior byte-for-byte.

For a chosen K, a conservative phase expression includes resident job Value/lease
+ K·(full status/result clone + serialized reply buffer + serializer scratch/headers)
+ maximum simultaneously active solve/publication phase if a result can overlap it.
Current status semantics allow a result only after completion, so that last overlap
must be derived from actual ownership, not guessed from labels. Serialize/error/drop
paths and any old/new String growth are included. A single upfront native M1
reservation covers the supported repeated phase; no cumulative finite-time promise.

The alternative is W1-only serialized waiting rather than a retryable refusal.
It must bound waiting request ownership and hold its gate through transfer; merely
extending the registry Mutex over clone or serializer does not bound detached bodies.
No waiting queue or asynchronous scheduling framework is designed here. Without an
accepted finite overlap mechanism/profile, native W1 remains unqualified, and that
does not satisfy the required representable native witness.

## 4. Actual serializer boundary and qualification limit

Local pinned dependency source is available: Cargo.lock selects tauri2.11.1,
tauri-macros2.6.1, serde1.0.228 and serde_json1.0.149. Read-only source shows:

- tauri-macros command/wrapper.rs:431 calls command, then blocking_kind().block.
- tauri ipc/command.rs:250–257 passes Result to resolver.respond.
- tauri ipc/mod.rs:181–186 blanket IpcResponse for Serialize runs
  `serde_json::to_string(&self)`; :265–276 invokes body(); :392–400 then transfers
  the InvokeResponse to return_result; :477–494 passes it to the owned responder.
- tauri's public IpcResponse trait and Response/InvokeResponseBody expose an owned
  JSON String handoff. This supports a non-Serialize wrapper retaining its guard
  while it invokes the same serializer, then moving the completed String outward.

Proposed finite window ends on successful return of
InvokeResponseBody::Json(serialized) from the guarded body's body() to the Tauri
response conversion. Drop the guarded cloned Value/status after serialization;
release its reply slot only at this transfer or at an error/abandoned response Drop.
Record resident job ownership is separate and does not end at this boundary.
The selected serializer/source/features, exact UTF-8 escape/number bounds, capacity
growth/try-reservation, wrapper Drop ordering and panic/error paths require P4 proof
and native tests. The dependency source confirms a seam, not those byte proofs.

Tauri-owned response/JS/OS queue copies **after** that explicit transfer, WebView/TS
heaps and arbitrary consumers are outside this named application-owned Rust window.
It is not a claim through browser consumption or total Rust/process RSS. If the
owning criterion requires those later copies inside the allowance, this endpoint
does not satisfy it: return that exact extension to ROOT. No supplier patch or
custom IPC system is proposed. A command returning ordinary Value and dropping its
permit on function return demonstrably ends too early and must fail the seam test.

## 5. Custody and specific native refusal

S-H/S5-R implementation retains actual raw Value/mode plus optional checked digest
and optional encoded length; parse errors remain errors. Missing digest prevents
both exact-source and W1 proof selection, with the accepted specific method decline.
Representable source bytes and source selection order stay unchanged. Typed input
never enters this capture path. No normalization, placeholder or legacy hash stands
in for the unavailable checked-profile invocation digest.

HL:741–755 already catches unavailable qualified evidence/export and returns raw
ordinary mechanics with canonical_export_unavailability. Its existing legacy
runner checksum remains that legacy checksum, not newly qualified custody.
Native TS captureNativeInvocation currently erases failure into null (:102–112);
validateCapturedSource swallows fingerprint/reader/hash failure (:114–131).
Workspace:847 consequently reports only generic missing binding and never setResult.

Freeze a typed private Capture/Registration outcome with distinct codes for:
request outside checked representation, returned result outside representation,
stale/mutated captured model/mode, and failed successor receipt/registration.
Proposed names for review: SOLVE_NATIVE_REQUEST_NOT_REPRESENTABLE,
SOLVE_NATIVE_RESULT_NOT_REPRESENTABLE, SOLVE_NATIVE_INVOCATION_CHANGED and
SOLVE_NATIVE_RECEIPT_REGISTRATION_FAILED; the W1-only slot refusal is proposed as
SOLVE_NATIVE_W1_REPLY_ADMISSION_UNAVAILABLE. These are not reserved by this packet.
Preserve the reason at direct IPC and completed known-job reception; a small
WeakMap association with the actual returned object or the existing job context
can carry it without adding fields to MechanicsEnvelope or minting registration.
workspaceSession consults this actual outcome before its generic unknown-binding
fallback and reports the specific refusal. It does not set result/AnalysisRun/
solveProof/Current/export for the refused source. Successor G0–G8 validation remains
mandatory before the existing qualified registration; a header alone cannot pass.

Raw native inspection is a **separate unfinished T3 obligation** under ROOT's
“F2a review disposition and finite derivation grants,” D2 §4.6.2 and §4.8/DD-8/T6.
This slice makes the refusal specific; it does not claim completed native fallback
display. Accepted finite values must be preserved bitwise or specifically refused.
The ordinary identity/quality inside a returned PP envelope is not rewritten to
compensate for unavailable outer custody.

## 6. Exact proposed seams and bounded tests

| Prospective maintained path(s), not a grant | Bounded change / decisive tests |
|---|---|
| `core/product_physics/src/lib.rs`; new `src/retained_routing.rs`; `tests/retained_routing.rs` | Closed seed/classifier, optional observer, first-terminal capture, exploration-only exact hook and explicit caller-context seam. Test both error/selection orders, late invalid terminal, suffix admission denied before first new basis, original prefix bytes/quality, no rerun and single actual source debit. Spy phase counts distinguish legacy internal W2 evaluations from duplicated orchestration. |
| `core/product_physics/src/source_receipt.rs`; new `tests/retained_custody.rs` | Fallible digest/encoded length; old selected bytes invariant. Parse-invalid remains invalid. Unsafe digest captured/typed ordinary values compare; S11-F 1e80 cases keep no-Passed-breach protection. No parsed/hash constructor. |
| `core/runner/headless/src/lib.rs`; `tests/retained_precision_admission.rs` | Explicit Headless context remains through real qualified export and output ownership. Force checked evidence/export failure: raw mechanics survive with the exact unavailability reason; no forged qualified evidence or hidden provisional-document allocation claim. |
| `apps/desktop/src-tauri/src/lib.rs`; new `src/retained_native_tests.rs` (test-only module) | NativeDirect/NativeJob contexts, record resident lease, W1-only reply guard and owned IpcResponse wrapper. Deterministic barriers at before clone, after clone, during serialization and before body transfer; K slots succeed, K+1 gets selected W1 refusal without clone; repeated polls release/reacquire. Ordinary concurrent polls/bytes unchanged. |
| Same native files | Cancellation before start, during compute, before publication, after terminal; result/lease Drop observed at actual owner loss, not cancellation flag or worker exit. Abandoned reply, serializer error, guard early-drop mutant, completed record retained across polls and table teardown with outstanding reply. No timing sleeps as correctness oracle. |
| `apps/desktop/src/services/previewService.ts`; new `services/retainedPrecisionNative.test.ts` | Distinct capture/registration outcomes for direct and job return. Unsafe request/result codes, stale model/mode, mutated payload, missing receipt, bad v2 policy; none registers. Actual direct/job validated successor registers with frozen fingerprints. |
| `apps/desktop/src/features/workspace/workspaceSession.ts`; new `features/workspace/retainedPrecisionRefusal.test.ts` | Specific refusal precedes generic binding error; no fresh result/AnalysisRun/proof/export state; representable success unchanged. Existing solveJobAudit:150 sequential polling is a positive client control, not enforcement of K against other callers. |

The source paths above extend I30's prior prospective map; no edits are granted.
I29 owns the resource/count implementation proposal after these interfaces freeze;
I31 supplies final-row bounds and I32 source/cache/wire/terminal evidence. No
kernel numerical, schema or protected-oracle edit is part of this routing packet.

Remaining finite cells: suffix raw-to-ordinary capacity/work expressions; exact
typed-seed and SourceBudget accounting backcheck; shared registry/container treatment;
native K/refusal profile selection; envelope→Value and serialized-byte bounds;
guard-to-IpcResponse transfer proof and qualified build facts. Then actual paired
native/HL/direct behavior, current-head T9/both-entry and complete atomic review.
No new runtime was run here, and disabling native W1 earns no native qualification.
