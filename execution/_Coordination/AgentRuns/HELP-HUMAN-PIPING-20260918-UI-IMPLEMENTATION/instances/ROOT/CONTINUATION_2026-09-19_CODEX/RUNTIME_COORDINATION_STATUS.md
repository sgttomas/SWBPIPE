# Piping / Runtime coordination status

Updated 2026-09-20 04:28 UTC. Sole editor: Piping HELP_HUMAN (this ROOT).
Peer: **dev - app**, task `01a07cd1-86da-7d30-9b17-e2cbbad3f10a`, host local.
Piping task: `01a0bc58-e7c0-7f11-a2ca-7b00e261014e`.
One shared note; acknowledged handoffs precede any ownership change. Silence
does not transfer a write scope. Both leads remain peers under the owner.

## Authority and current work

The owner's active-chat instruction: “This message authorizes coordination
discussion, not new integration implementation or changes to existing ownership.”
The peer subsequently reported separate owner authorization to implement
Runtime's generic application-specific dynamic-tool mechanism now, with SWBPIPE
as its first consumer. We acknowledged Runtime-only work as disjoint; no Piping
adoption or live binding is inferred. These are distinct authority records.

Live `origin/main` revalidated at `485051eac923c759238948a54cd7bb094eee4899`.
Its change from the prior `c459a0fa1` baseline is App release metadata only.

| Worktree name / branch | Owner and active scope |
|---|---|
| `b3d208ad-bb6f-4bed-aaab-c567e28cbe23/chirality` / `codex/swbpipe-continuation-20260919` | Piping ROOT: run graph, direction/coordination records, independent reviews and integration; HEAD `88d0228dd`, further records uncommitted. |
| `swbpipe-wt3` / `codex/swbpipe-b-shell-20260918` | B3 WORKING_ITEMS manager: Piping desktop shell/controller/styles, scoped viewport portal, native host/menu and tests; product `2882acab94120cfa6c3c115ddb416a368cebd9cd`. |
| `swbpipe-wt2` / `codex/swbpipe-b3-20260919` | ROOT review/integration checkout `1658a2511`; not yet the latest native repair. |
| `swbpipe-b3-native-probe-20260919` / detached | Manager's isolated native popup prototype; diagnostic writes only, never implicit production adoption. |
| `chirality-runtime-application-tools/chirality` / `codex/runtime-application-tools` | Peer: `projects/chirality-runtime/**` generic contracts, daemon, client, tests and evidence; base `485051eac`. No Piping, App or Root writes. |

B3 manager (Astra/high), original native worker (existing Sol/high assignment)
and fresh popup prototype worker (Astra/low) remain active. Independent code
reviewer is Astra/xhigh; prior three accessibility findings are closed. All new
formerly-Sol deployments use Astra/low. Canvas `f6c0bab8e` and design-system
`4dddd4412` branches remain parked. B3's native popup Escape repair and final
native/full-suite/review/sweep/CI gates remain open; B3 is not merged.

## Acknowledged split and resources

Piping owns domain schemas/tools, live controller, UI and the future Tauri host
adapter. Peer owns reusable Runtime and is the explicitly acknowledged writer
of its generic contract. Root/shared governance and Piping-domain contract
writes have no transferred ownership. Substantive scope/design disagreements
return to the owner before the affected work proceeds.

Peer claims no ports, test servers or native/CUA use and will message before
claiming them. Piping serializes native/CUA through its B3 manager; the correlated
popup prototype currently owns that slot. Source/dist E2E reserve 5174/5175 via
`with_e2e_lock.sh`, one Playwright worker. Tauri dev 5173, ROOT preview 5183/5184
and canvas probes 5185/5186 require explicit reservation before use. Use isolated
test profiles/stores and sockets; never share a live user model. Builds/tests
must not overlap timed D-72 measurements. No Runtime native slot is implied.

## Interface discussion, not a frozen integration contract

The shared Rust engine and live workspace controller are the right foundation.
However, `workspaceSession.ts:handleQueueOperationBatch` captures the model at
queue time and returns no receipt. A naive wrapper would silently rebase an
older inspection. The adapter must preserve the issued workspace generation,
revision and canonical hash through preview, queue and human acceptance. An
equal hash after Undo does not restore the original revision or revive a stale
proposal. Success must report a committed live-state/history receipt, not only
the detached engine result or a scheduled React update.

Proposed first tools, accepted as a discussion sketch by both leads:

- `swbpipe_inspect_selection`: explicit object refs, bounded values/units,
  host-issued basis token and capability/schema identity.
- `swbpipe_preview_operations`: existing batch validation/diff; no live mutation.
- `swbpipe_submit_proposal`: explicit queued/stale/cancelled/busy/unavailable
  receipt, review ticket and trusted Runtime/thread/turn/call idempotency scope.
- `swbpipe_get_proposal_status`: review state and actual live commit/refusal.

Human UI/controller acceptance performs the change. No agent Apply tool in this
first target. Later selection never retargets a submitted proposal. Project
switch/reopen, cancellation and Runtime restart cannot replay an application.
Runtime transports application schemas/results; Piping owns their semantics.

Existing Runtime v1 session/turn/event/request APIs are usable building blocks,
but the Electron launcher and Node client are not a Tauri host adapter. Piping
still needs process packaging, private socket transport, lifecycle/recovery and
effective-home configuration. Pin a reviewed Git revision, not just package
version 0.1.0. Peer verified stock Codex 0.154.0 supplies dynamic tools only at
thread start: catalog identity must persist across resume, an existing tool-free
thread needs a new tool-enabled session, and descendant inheritance is not yet
qualified. The baseline supervisor rejects dynamic calls; the peer's new work
addresses that missing generic route.

Offline proposal intake is local and author attribution is unverified. Runtime's
legacy governance proposal envelope is not Piping's OperationBatch contract.
Runtime permission approval is separate from the engineer's UI acceptance.
Domain-route equivalence is not isolation from other user-configured Codex tools.

## Recommended order and first journey

Continue B3; generic Runtime work may proceed independently under the peer's
reported grant. Begin Piping adoption after B3, preferably after already-authorized
B3A/B3B settle dirty/save/busy controller semantics. Agree the exact Runtime SHA,
schema/basis/receipt contracts, failure behavior, scopes and resource slots before
integration writes. Do not wait for the entire closing visual pass.

Refine the proposed journey to one supported load-magnitude update in an invented
fixture: inspect selected load; preview without mutation; submit for human review;
accept once; verify model, one history entry and Current-result invalidation;
Undo and Redo without reviving Current results. Reject stale submissions both
before arrival and after preview, including equal-hash-after-Undo; cover duplicate
delivery, project switch and cancellation. Start with the main HELP_HUMAN session,
not descendant tools. This target is recommended, not yet committed for execution.

Piping live binding/adoption remains separately gated by D-58 / DEC-091 and
DEC-042; this discussion does not lift those holds or add client status. Do not
misread the historical no-network fence as a new residency approval requirement:
DEC-051 and `docs/CONTRACT.md` OPS-K-PRIV-1 already permit owner-configured
model-provider transmission without another app-side opt-in. Preserve that
ruling, public-commit/IP boundaries and telemetry policy in any adoption proposal.
No deliverable reconciliation or DAG rebuild is initiated.

Source basis: Piping `workspaceSession.ts`, `operationBatchService.ts`,
`OfflineProposalIntakePanel.tsx`; Runtime `contracts/src/protocol.ts`,
`client/src/client.ts`, `daemon/src/codex-supervisor.ts`; DEC-051, D-58 / DEC-091;
peer's direct task replies. Source paths are under their project package roots;
peer-reported host/schema details are identified above, not native Piping proof.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
