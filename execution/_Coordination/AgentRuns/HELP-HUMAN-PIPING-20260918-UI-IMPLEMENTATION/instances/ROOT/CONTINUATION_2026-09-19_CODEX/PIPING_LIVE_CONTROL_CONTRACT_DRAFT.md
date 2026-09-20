# Piping live control contract — discussion draft

Status: PROPOSED revision2,2026-09-20. Prior peer-reviewed revision1 is preserved at ROOT302073f0, SHA25632277c6248dfe8ee721f2a3410702fae96a1c87cbae60eaff0941b3d2dfd526e. Preparation only; no integration code, persistent configuration, VM/CAEPIPE access or Runtime adoption is performed by this document. Stable inspected product basis:226b1db57029c1df9f9151463c6c708f76f46704, production-equivalent to reviewed a635b01ad67e85a10b542e889f0de29715358e49. Current UI batch has not yet merged.

## Purpose and current owner direction

Use a development Codex session to control SWBPIPE through typed operations and operate CAEPIPE through Computer Use on this Mac's Windows environment. Embedded Runtime follows. Exact direct-chat dispositions are retained in OWNER_MVP_AGENT_CORRECTION_2026-09-20.md and OWNER_CODEX_VALIDATION_CONTROLLER_2026-09-20.md.

Peer dev - app additionally reports this owner condition in its session: “I agree to this only if it follows the V2 MCP protocols (which are stateless).” This is a condition on the proposed transport, not unconditional acceptance of the earlier stdio sketch. The peer owns a harmless isolated actual-client compatibility probe. No Piping implementation may assume that configured plugins or App Server dynamicTools evidence proves this MCP wire revision.

## Transport and ownership

Keep one transport-neutral application contract: controller adapter → Piping private local app bridge → existing live workspace controller. The actual bundled Codex0.155.0-alpha.9.2 probe used a2025-06-18 initialize handshake and failed against the strict modern server; that tested MCP route is unqualified. ROOT and the Runtime lead now recommend a small JSON CLI as the first adapter, subject to the owner choosing it. Codex can invoke that CLI through its existing command tool; describe/help supplies discovery. MCP may be a later thin adapter when its actual client meets the owner's V2 condition. Do not implement both now or silently substitute the CLI for an unresolved owner choice. Explicit application references remain required with either adapter.

Piping HELP_HUMAN retains controller, private bridge, selected adapter, fixtures and Piping documentation. The Runtime lead provides read-only interface/protocol review and its separately bounded client compatibility evidence; no Runtime/shared Root source change is currently justified. Any later shared-file change requires an explicitly agreed writer.

Prefer one per-app private local socket on this Mac if inspection finds no suitable existing transport. The bridge must validate app/workspace identity and authorization independently of self-reported MCP clientInfo. Its lifecycle, ready/disconnect handling and local access controls are part of the implementation contract. This proposal does not select or expose a network listener. Keep the domain handler interface independent of MCP or Runtime wire DTOs so later embedding can reuse its behavior.

## Modern MCP condition

Any chosen MCP adapter must pin protocol2026-07-28 after the actual Codex client proves support; the current tested stdio configuration does not. Every request carries protocol version and client capabilities metadata and explicitly references any application state it needs. Ordinary successful responses include resultType complete. No implicit session basis, missing-metadata acceptance or hidden legacy downgrade.

Evidence must come from the actual intended client: discover/list/call, independent workspace requests, reconnect and cancellation. Handcrafted modern frames or a compatible server SDK alone cannot qualify that client. If it speaks only legacy MCP, report the incompatibility and return the transport choice to the owner.

Primary sources independently opened by ROOT:
- https://modelcontextprotocol.io/specification/2026-07-28/basic/index (statelessness and response shape)
- https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio
- https://blog.modelcontextprotocol.io/posts/2026-07-28/

## Four domain tools

These are semantic shapes for review, not final schemas or new operation names.

| Tool | Required explicit input | Observable result |
|---|---|---|
| inspect | A declared app-instance reference; a workspace reference and requested selection/entity scope when inspecting model content. | Available workspace handle for that app, or the requested typed entity snapshot plus host-issued basis. Never use a previous call to choose the workspace. |
| preview | Workspace reference, inspection basis and supported typed operations with immutable targets and source units. | Validated proposed changes/diagnostics and an immutable preview reference; no model/history mutation. |
| submit | Workspace reference, preview reference and explicit stable idempotency reference. | New submission: ticket after queue publication or explicit failure. Repeated identical submission: the original ticket/current recorded outcome, even after its human Apply changes the revision. Queued is not committed. |
| status | Workspace/application reference and review-ticket reference. | Queued, committed, rejected, cancelled, expired or outcome-unknown, with stale/superseded/expiry reasons and an actual commit receipt only after model/history publication. Lost-ticket recovery uses the original submit key. |

Application references and any access capability must be supplied or explicitly referenced per call; they are not inferred from connection continuity. Initial app discovery may use a declared attachment descriptor or explicit app-instance inspection, not a remembered default workspace.

The inspection basis binds app instance, project identity, workspace identity/generation, model revision and canonical model hash. Reject cross-workspace reuse even when bytes and revision happen to match. Await a coherent current hash before minting it; report not-ready rather than using a stale hash. A selected target is a copied typed identity/snapshot. Later selection changes must not silently retarget the proposal.

The preview reference explicitly addresses Piping application state containing the exact operations and basis. For a NEW submission and for human Apply, revalidate the appropriate live basis. Resolve an existing idempotency record before applying fresh-submission basis checks; recovery of a committed outcome must not be rejected merely because that very commit advanced the revision. Equal canonical contents after Undo do not revive an old revision. Project replacement, app restart, cancellation and superseding state must produce explicit outcomes, not silent rebase.

The first journey retains human Apply through the existing controller. An agent-generated CUA click is not human acceptance. Any later owner-configured automatic-apply workflow is a separate bounded choice, not an inferred property of agent control.

Receipts identify actual application, workspace generation, operation IDs and before/after revision/hash as available from the authoritative controller. Bridge-issued invocation/request identities are truthful transport provenance; do not fabricate verified Codex thread/turn IDs. The bridge/controller assigns trusted actor=agent to incoming operations and records actual human acceptance separately. Supplied author_type=user must not become authoritative attribution; MCP clientInfo is not authentication.

Scope an idempotency reference to authorized application/workspace generation and retain its canonical payload/preview/outcome association through the claimed reconnect window. If the submit acknowledgement is lost after queue publication, retry with the SAME key and original preview returns that ticket/current outcome, including after human Apply; it never forces a new key or creates a second proposal. An idempotency reference must not cause duplicate application after a repeated request or adapter reconnect. Reuse of the same reference with different contents is an error. After a restart or lost response, recover an authoritative prior outcome where available, otherwise return uncertainty/expiry and require reconciliation; never replay blindly. Persistence requirements must be decided explicitly if the current session-only receipt carrier cannot meet the selected guarantee.

## Publication and cancellation boundary

At226b, handleQueueOperationBatch captures the basis at arrival and returns Promise<void>; handleRunOperationBatch uses multiple React setters without an external completion receipt. A thin transport wrapper around those signatures is insufficient. Evolve the shared controller boundary to accept the earlier explicit basis, return typed busy/stale/withdrawn/unavailable outcomes and acknowledge publication. Reuse the existing validation/apply/history/result-invalidation route; do not introduce a second model store.

The controller must expose a queue-publication acknowledgement and an applied-transaction acknowledgement covering model, history and result standing. The adapter resolves queued/committed only on those acknowledgements, never merely on engine success or scheduling React setters. The implementation activation must name the concrete publication hook and its reset/unmount/error handling before code is written.

Cancellation has a linearization boundary. Before queue publication, accepted cancellation leaves no ticket. After publication, cancellation/disconnect of the RPC does not withdraw the proposal or reverse a later commit. Recover the ticket by the same submit key; use existing UI withdrawal for an already queued proposal and report its actual resulting state. A late cancellation cannot undo a model commit. The first contract needs no additional tool solely for this distinction.

## First proof and subsequent extension

First use one existing supported change on an invented fixture: inspect → preview without mutation → submit → actual human Apply → observe model/history/result invalidation → Undo → reject the stale proposal, including equal-hash-after-Undo. Include duplicate submission, conflicting idempotency input, project switch and cross-workspace reuse, cancellation on each side of publication, disconnect, and queue → lost reply → human Apply → same-key retry. Select the exact supported operation from the maintained schema/controller rather than inventing a new domain command.

Then prove the SAME Codex controller can call a harmless Piping inspection and observe/interact reversibly with the chosen Windows application through its actual Computer Use provider. Guest accessibility or reliable screenshot control must be witnessed. This is distinct from an embedded Runtime agent qualification.

Only then extend the contract as needed for supported case preparation, solve/status, result inspection and model-batch/report export, and perform owner-authorized invented-case correlation. Preserve source units, input basis, solver identity and visible limitations. Product/user-guide export wording remains the governed .mbf wording under DEC-103; this design record may name CAEPIPE.

## Work sequence and activation boundary

Finish the current B3B/C3/CI integration. Peer reviews this revised domain contract; the owner selects the first adapter in light of the measured modern-client incompatibility. Reconcile any substantive interface disagreement with the owner. Prepare one bounded implementation activation with exact file ownership and the current live-binding ruling addressed; a product design discussion alone does not claim that held stage is active.

B4 table work already targets App/workspaceSession/styles, so live-controller integration cannot be assigned a competing writer there. Use one shell/controller writer or an acknowledged handoff. Canvas label work may run independently once its own protected product questions are resolved. Serialize native/CUA/guest interaction and browser ports; no UI/build work alongside later timed qualification. Runtime qualification can proceed independently with disjoint files and no shared desktop use.

Current holds: exact modern-client compatibility; concrete live-binding activation; chosen guest/provider/access witness; any required durable receipt carrier or unattended-apply decision. Current code, source review, qualification and release remain separate facts.

