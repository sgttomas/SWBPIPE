# Embedded agent host

Status: implementation design for RT-E1, 2026-10-28. No live bridge or
supplier/model round trip is established by this document. SWBPIPE owns the
catalog and host; App v4 owns its separate implementation and release evidence.

## Basis and scope

The owner authorized SWBPIPE's App v4 integration for v1.0 and then explicitly
authorized this manager to build shared needs: “That agent can also build in
SWBPIPE in conjunction with the App v4 agent, working on shared needs.” The
closure-only hold on other SWBPIPE lanes does not prohibit this work. RT-E1 and
exit criterion 17 entered main through #1732, merge `541446a9f1`.

The objective is an agent that operates SWBPIPE effectively through its own
capabilities. The first increment queues every change for the person's review.
It introduces no autonomy grant, acceptance tool, issuance or release act.
DEC-041 and earlier App designs are historical context, not implementation
instructions. The Decisions API is excluded entirely.

The order is catalog/design, a pinned harmless local-model round trip, Rust
client/dispatcher, webview bridge/completion command, proposal review, then a
minimal functional panel. Substantial UI and interaction design awaits the
owner's focused discussion. The owner authorized official stock Codex binary
downloads, use of the existing oMLX server, and a hosted model provider if
needed. No other downloads are authorized; credentials remain owner supplied
and keychain held. Local and hosted evidence must identify which was used.

## Catalog ownership

[`agent-capability-catalog.json`](agent-capability-catalog.json) is SWBPIPE's
implementation contract. Its origin is App commit
`e1e3f4615b6f766b313c3492506f68254afd8a06`, path
`execution/PKG-09/DEL-09-06/Design/SWBPIPE_CATALOG_PROPOSAL.json`, SHA-256
`64aad707acd1d409f3d255381fc597e2ee74e7a4d048aadc5382414d63c64f8c`.
The initial local version 1 changed catalog identity/standing and added origin
metadata. Version 2 also adds bounded proposal assumptions and distinguishes
live-basis admission from completion against retained proposal/run evidence,
repairing independent review findings. Native operation intents, limits and
dynamic projection are unchanged. The App copy stays
origin evidence and is not a second editable catalog. A local contract commit,
integration into main, and demonstrated runtime conformance are distinct facts.

The six tools read the summary, read nodes, propose operations, observe a
proposal, solve, and read results. Initial proposals admit native
`EditorOperationIntent` only: `modify` / `Node` / `set_field` for
`position.x`, `position.y`, or `position.z`. This cannot build W9-A or the full
criterion 17 reference system. Further native operation discriminators and
subschemas must be added before those journeys; do not add a parallel field
language or infer completeness from a coordinate-edit witness.

The internal descriptor/result convention follows stateless MCP 2026-07-28,
with explicit workspace/proposal/run/page handles. It is projected to Codex
dynamic tools, not transported by MCP. Expand catalog-local schema references
into standalone descriptors. Retain output schemas and annotations in SWBPIPE;
annotations are descriptive and confer no permission. Project only the fields
supported by the selected supplier schema. Validate results before JSON text
encoding into dynamic `contentItems`; `success` follows the catalog policy.

## Existing authority and interfaces

Source inspected at `541446a9f1`; product source is unchanged from
`1560bbf7f7add4637a20572a2b0d404ef121af63` for these interfaces.

| Responsibility | Existing route | Required integration behavior |
|---|---|---|
| Live model, hash and undo | `apps/desktop/src/features/workspace/workspaceSession.ts` | Webview retains authority; bind each call to the session and revision |
| Queue registration | `workspaceSession.ts::registerOperationBatch` | Check claimed hash before registration; registration snapshots the current model and is not itself a stale-agent-basis check |
| Review/application | `workspaceSession.ts::handleRunOperationBatch`, `features/toolkit/BatchReviewPanel.tsx` | Existing explicit human apply; preserve preview requirements and immutable submissions |
| Validation/application | `services/operationBatchService.ts`; `src-tauri/src/lib.rs::model_operation_batch_args`; `core/model_operations/operation_applier` | Stateless Rust commands consume supplied model/batch/hash; the existing applier remains semantic authority |
| Solve | `workspaceSession.ts::handleRun`, `services/previewService.ts` | Recheck basis before admission, preserve evaluated hash, and retain actual job/run outcomes |
| Result disclosure | `features/results/resultReader.ts`, `features/result-export/csvTable.ts` | Preserve current view, exact-value semantics, withheld values and redaction floors |
| Native admission | `src-tauri/src/command_ingress.rs`, `user_release_reach_scan.rs` | Admit each new command; independently review permissions and repin reach evidence |

Abbreviated frontend paths in this table are relative to `apps/desktop/src`;
`src-tauri` paths are relative to `apps/desktop`. Live-session source references are not evidence that the new
agent adapter already exists.

## Host and session protocol

SWBPIPE starts one stock Codex app-server child over private stdio for its
admitted context. There is no network listener or MCP child. Keep its Codex home
and empty working directory separate from authoritative project storage. Do not
inherit the developer's Codex home, project instructions, configured MCP servers
or credentials. Disable built-in shell, file/patch and web tools using features
supported by the selected pin; configuration and prompting alone do not prove
isolation. Harmless denied-operation probes must establish actual behavior.

Initialize with the selected pin's experimental opt-in; offer the catalog as
`dynamicTools` on `thread/start`. Keep the frame reader responsive while calls,
human review or solve wait. Bound frames before JSON materialization and retain
bounded notification queues. Unknown requests fail explicitly; never grant an
unrequested shell/file/network approval. Provider selection is configuration,
not proof of network destination control.

On `item/tool/call`, Rust admits size/count budgets and validates the full
arguments against the SWBPIPE schema, irrespective of provider schema handling.
It captures the original RPC id, conversation/turn/call/tool, host application
generation, workspace handle and model basis/revision. Check workspace and
generation against the admitted session before dispatch. Summary captures a live
basis; node reads, new proposals and solve require the current live basis.
Proposal observation instead binds its retained proposal basis, and result
reads bind the retained run's evaluated basis. A current-model revision change
does not alone invalidate those historical records. Issue an opaque `requestRef` for that
pending call and forward only to the admitted webview. Handles are scoped to
that generation/workspace and are neither filesystem paths nor identity proof.

The webview rechecks its generation and the call-specific revision/hash before executing. The
summary bootstraps the current handle/basis without caller-supplied paths.
Reads are bounded at the producer, not serialized wholesale and then truncated.
Page cursors bind tool, workspace, basis, run and next offset; drift yields an
explicit stale/unavailable outcome. Values retain the document's stored units;
proposed units must match those units exactly. No silent conversion is allowed.

One new ingress-admitted completion command returns `requestRef` and the bounded
outcome. Rust looks up the original RPC destination; the webview cannot supply
an arbitrary RPC id. Reject unknown, duplicate, wrong-session, wrong-generation
or mismatched completions. Binding means correspondence to the immutable pending
call, not requiring every call's revision to equal the latest live revision.
Summary/node completion requires an unchanged live revision. Proposal submission
completion binds the actual immutable queue registration; observation binds its
proposal record and original basis; solve completion binds the admitted job and
evaluated hash; result reads bind the retained run/cursor basis. These last four
may complete after a live edit with explicitly captured/historical standing.
The returned basis must never be rewritten to the newer live model. A human
Apply advancing the revision does not erase its correlated applied outcome.
Invalidate detached sessions and outstanding handles;
never automatically redispatch. Validate output schema and standing before
answering Codex. Missing/uncertain application evidence yields `unknown`, not
success, rejection or a retry that might duplicate mutation.

## Proposal and interruption behavior

Require `claimedModelHash == basis.modelHash == current session hash` before
`registerOperationBatch`; repeat the existing current-basis check at application.
Construct the batch in the host and stamp `author_type: agent` plus original
conversation/turn/call ids. Caller-supplied validation, attribution or source
fields are untrusted. Preserve native intent shape and per-operation rationale.
The proposal envelope also requires a separate `assumptions` array: at most 16
records, each with a bounded stable id, statement and affected native operation
ids. An empty array declares none. Reject duplicate assumption ids or references
outside the submitted operation ids. Capture these records immutably alongside
the proposal handle, batch id, attribution and original basis; display them in
review and retain them with session outcomes/receipts, including reject,
withdrawal and unknown. Do not flatten them into rationale or add undeclared
fields to the native intent. This context storage is not durable audit history;
attribution is not authenticated identity. Queue registration never applies.

Use existing validation, immutable registration, diff/review and human Apply.
Observe pending review, rejected, withdrawn, applied and unknown distinctly.
Turn interruption withdraws a proposal only while it is still pending. Serialize
withdrawal with application admission: if application has begun, preserve the
actual result or unknown. Interruption is neither rollback nor solve-job
cancellation. Restart loses session handles and cannot manufacture durable
acceptance history. DEL-16-03's durable audit/history residual remains open.

Solve requires a matching claimed basis before `handleRun`; retain solver mode,
evaluated hash, actual run handle and health. Progress goes through existing
supported host notifications/panel mechanisms. Result pages retain route,
health/check standing and the evaluated basis even when the live model changes.
Withheld/unavailable values carry no substitute numeric value.

## Boundaries and reconciliation

F-PIP-3, FR-AGENT-005, DEC-081 and Root `docs/PRODUCT_BOUNDARIES.md` govern all
agent-visible claims. Agents do not record, imply or derive engineering
acceptance, issuance, certification or code compliance. Numerical results and
passed checks do not confer professional authority. Human model-change review
and professional acceptance remain distinct. Existing claims checks and runtime
boundary scrutiny must cover new emitting surfaces; do not claim the Python
rationale reference is already integrated into the native path.

`docs/IP_AND_DATA_BOUNDARY.md` §6.1 permits owner-configured runtime transmission
of the owner's private inputs to the selected provider. It grants no public
redistribution rights and no exception for third-party protected standards.
Fixtures and public observations use invented data only. Do not commit supplier
logs containing private inputs, credentials, account identities or machine data.

DEL-16-06 owns this adapter's controlled-operation contract. DEL-16-01/02 retain
schema and preview; DEL-16-03 retains acceptance/audit; DEL-16-04 retains claims
and rationale; DEL-02-05 retains persistence; the workspace retains session
coordination. The new bridge does not close their existing residuals. The old
R7/F3 reference and candidate-generator program in DEL-16-04 are not the plan
for this authorized host slice; its matching scope wording must explicitly
separate the current integration from that older program before integration.
Neither standalone App qualification nor workflow-act machinery is inferred.

## Verification sequence and open observations

### Rust component implementation

The desktop declares `embedded_agent` but exposes no agent command, start
control, session dispatcher or webview route yet. `catalog.rs` embeds a generated
byte-identical product resource of this catalog, expands its references for the
six dynamic declarations, validates
inputs/results and checks proposal assumption membership. `protocol.rs` owns
the fixed initialization/thread/turn RPC sequence and opaque pending-call
references. Invalid inputs fail before dispatch; turn end/disconnect detaches
pending calls without claiming cancellation or reversing an applied change.
The caller must still authenticate the session, supply a trusted error basis,
and retain admission leases through all derived events and outbound values.

`framing.rs` caps a complete private-stdio frame at 128 KiB before any `Value`
materialization. It uses the existing general interactive ingress reservation,
checks borrowed argument/outcome spans against catalog limits, and poisons a
reader after invalid/oversized input. `numeric.rs` rejects delivered number
tokens that change decimal value on materialization. This protects the RPC
bytes received from Codex; the pinned supplier protocol does not establish
preservation of the model's original number lexemes upstream. Native operation
decimal strings remain strings. The only dependency-feature change is enabling
the existing `serde_json` dependency's `raw_value`; Cargo.lock is unchanged.

`tools/agent_catalog_resource.py` generates
`apps/desktop/src-tauri/resources/agent-capability-catalog.json` directly from
the authoritative Design bytes; do not edit the product resource independently.
The registered project check `agent-catalog-resource` uses `--check` to fail on
absent, linked, oversized or differing input. Both project and Root selectors
route its inputs. The Root software-workflow regression suite executes this
registered command in the existing full-checkout Repository checks job. Sparse
desktop builds and public source exports compile the product resource without
requiring the excluded execution tree. No workflow file, required status or
harness aggregate is changed. Equality establishes source correspondence, not
catalog semantics or runtime conformance.

`stdio_pump.rs` supplies an explicitly constructed portable stdio pump, with
32-slot incoming/outgoing queues and separate reader/writer threads. The catalog
admits 16 pending calls; the extra slots accommodate their control/status burst
without requiring a coordinator timeslice between each frame. It does
not launch a process or own protocol/session authorization. GENERAL interactive
admission reserves two 512 KiB stacks plus a 128 KiB bookkeeping allowance
before queue allocation/spawn; framing's reader buffers and each frame's backing
are separately admitted. Outbound values are moved and serialized to the stream,
with a byte-counting pass before enqueue and no full encoded-frame copy. Shared
backing references preserve custody, not a fresh budget for each clone. Queue
exhaustion is a terminal fault; no unknown operation is resubmitted.

Both workers wait behind a start gate. A second-spawn failure aborts that gate
and returns typed partial custody, rather than claiming completed rollback.
Stop request, terminal I/O, EOF and actual joined worker completion are separate
observations. A blocked generic read/write cannot be interrupted by this pump;
its caller must close/stop the owned process/streams and retain the pump until
workers can be joined. Observing joined workers confers no process-group or
descendant-empty evidence. Bookkeeping/finished joins have no hard wall-clock
guarantee. Abandoning unfinished custody deliberately retains it and its startup
reservation, even if a worker later finishes; integrating owners must keep that
custody rather than silently drop it and release live-stack accounting. This is
a fail-closed memory-retention limit, not crash cleanup qualification.

Pump component tests cover frame/backing custody, EOF before joined
completion, queue exhaustion, streamed output/newline/flush, serialization cap,
stalled read/write retention, second-spawn partial custody, static I/O faults and
constructor heap against the control/buffer reservations (not an OS/RSS proof).
They use controlled synthetic streams, not a supplier/model or native session.

`session_driver.rs` connects the pump to the fixed protocol operations. A trusted
host owner calls bounded polling continuously while domain/human work proceeds
asynchronously elsewhere. Each poll handles at most one frame and returns at
most 17 events, including the catalog's 16 detachments at turn end. This module
adds no scheduler, threads, process authority, Tauri route or live-model binding.
The host supplies a unique unpredictable connection token and current fallback
error basis; authenticating the generation/workspace and each call-specific
basis remains the dispatcher/webview's duty.

GENERAL admission reserves a nominal 8 MiB for v2 catalog/protocol bookkeeping,
then separate 2 MiB allowances before control construction, outcome encoding or
incoming-frame derivatives. Caller-owned input/outcome heap must already be
admitted. Leased event batches retain the incoming/derivative and session
backing; borrowers copying their payload must admit and retain that copy's
backing. Queue submission reports `Queued`, never peer delivery or application.
Control `Busy` refusals occur before protocol state changes; their admitted
input can be retried. Polling may instead defer one leased batch until a fresh
terminal observation. Invalid schema/reference completions leave valid pending
calls intact. Fallback-basis updates never substitute a newer basis into an
actual supplied outcome.

On observed EOF, I/O failure or invalid peer protocol, the driver stops dispatch
and disconnects once, reporting unresolved references without cancellation,
rollback or an invented domain outcome. Already-observed terminal state
suppresses queued new tool work. Partial startup returns custody for subsequent
observation; blocked I/O still needs the external process/stream owner. Actual
domain results remain outside the driver, including when an admitted completion
cannot be queued or its peer delivery becomes unknown. Synthetic driver cases
exercise handshake/declarations, original-RPC completion, preserved pending
references, interruption/detachment, terminal suppression, backing lifetime and
an admitted 16-call burst. The live host coordinator/session bridge remains
unimplemented and none of these checks establishes AC-005.

The protocol also treats the selected pin's unoffered built-in tool-activity
items as terminal on start/completion, including command/file/web, MCP,
sub-agent and image activity. Pending calls become lost references, never
fabricated application failures or cancellation. This observes a configuration
or isolation mismatch; notification timing cannot prove that an action was
prevented. The native process owner must still stop/clean up its supplier, and
actual disablement/OS isolation remains a separate activation proof obligation.

The macOS-only `macos.rs`/`sandbox.rs` launcher is not connected to product
startup. It verifies and copies the selected binary, uses private home/cwd and
an environment-held credential, and denies reads outside fixed OS locations and
its private runtime. Literal root-directory reading and metadata on fixed OS
aliases are needed for the selected pin's startup; these do not grant reads of
descendant user files. Only private home/cwd writes and outbound loopback port
8000 are offered. No hosted-provider profile is implemented. Production stderr
is discarded; the native startup test uses an invented credential and checks
that its bounded diagnostic does not contain it. Real credential acquisition,
crash-diagnostic absence and a complete native-session witness remain unproved.

On shutdown the launcher retains the original unreaped `Child` through every
group observation and signal, then reaps it only after observing no live group
members. A census is evidence, not authority to signal an unrelated numeric
PID. Unknown cleanup retains a nonsecret exclusive marker and refuses the next
launch; no saved numeric PID is signalled. Parent-crash termination and absence
of escaped descendants remain unqualified. **Do not expose this launcher until
those cleanup duties and the remaining native-session isolation duties have
been established.** The early solver-worker dispatch and solver-specific memory
machinery are unchanged.

The opt-in native probes establish bounded observations on the selected Mac:
the actual pinned supplier initializes under this profile without a model;
owned-group stopping preserves an unrelated group; and the standalone invented
probe in `tools/agent_host_sandbox_probe.rs` checks 13 filesystem/network/process
outcomes. Private-home/cwd writes and the configured loopback endpoint succeed;
outside canary reads/writes, symlink escape, another loopback port, an off-host
address, listener creation and both other/self executable launches are denied.
The canaries named project/documents/keychain are invented files outside the
private runtime, not actual private documents or keychain files. These component
checks do not establish model execution under this Rust launcher, crash cleanup,
Windows capability, review-queue conformance or AC-005 completion.

Remaining implementation is the responsive runtime client/dispatcher, trusted
session bridge and ingress completion command, proposal-only review wiring,
minimal explicit-start panel, operation expansion and criterion 17. Existing
ScopeOfWork commitments and dependency conditions remain open and unchanged.

### Integration sequence

1. Validate all catalog schemas and standalone dynamic projections; compare the
   adopted fields with the identified App proposal. Review exact commitment
   wording independently and obtain Agent 0's confirmation.
2. Pin an official Codex binary and existing oMLX endpoint/model. Record version,
   official download URL and binary SHA-256 here before the live witness. Observe
   one harmless dynamic call, host validation, returned invented value, and a
   follow-up grounded in that value. Capture declared versus observed tool
   isolation separately. Greeting/listing/request capture alone is not a pass.
3. Exercise malformed/oversized frames and arguments, wrong initial binding,
   duplicate/unsolicited completions, revision/session replacement, interruption
   and uncertain outcomes with controlled protocol/session checks. Include a
   human Apply advancing revision while observation waits, solve completion
   after an edit, historical result pages, and retained structured assumptions.
4. Exercise actual proposal review, reject, stale refusal and one human-applied
   edit through the native route. Obtain independent permissions review for new
   Tauri/ingress commands and repin the reach scan as #1718 did. Preserve existing
   memory admission; re-price any changed retained route as its rules require.
5. Add admitted operation kinds for the build/solve/explain journey; verify the
   full criterion 17 reference separately from the smaller W9-A witness. A hosted
   journey must not be mislabeled as a local-model witness. Native packaging and
   product release remain separate owner acts.

The selected supplier/model pin and actual harmless round trip are recorded in
[`agent-local-witness.md`](agent-local-witness.md). Those bounded observations do
not establish the desktop bridge or complete AC-005. The current
[official app-server guide](https://learn.chatgpt.com/docs/app-server), read
2026-10-28, documents stdio, initialization, generated version-specific schemas
and experimental opt-in. The selected binary's generated schema and actual
observations govern implementation, not assumptions from newer documentation.
