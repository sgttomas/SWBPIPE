# Piping / Runtime coordination status

**Resumed, 2026-09-20:** The owner explicitly directed B3 repair/merge, a less wasteful CI strategy, then remaining authorized UI work. B3 manager is repairing the pinned-Chromium tooltip/Close failure in wt3; ROOT integrates CI changes in wt2. All new implementers are Astra/low. The peer acknowledged no conflicting resources and unchanged Piping-specific CI ownership. B3 has not merged. Runtime integration remains discussion only, with no ownership transfer. This notice supersedes older active-state summaries below.

Updated 2026-09-20 during final B3 checks. Sole editor: Piping HELP_HUMAN (ROOT).
Peer: **dev - app**, task `01a07cd1-86da-7d30-9b17-e2cbbad3f10a`, host local.
Piping task: `01a0bc58-e7c0-7f11-a2ca-7b00e261014e`.
The peer acknowledged this as the sole shared note. Both leads remain peers under
the owner; acknowledged handoffs precede ownership changes. Silence transfers nothing.

## Authority, ownership and active state

Owner: “This message authorizes coordination discussion, not new integration
implementation or changes to existing ownership.” The peer separately reported
owner authorization for Runtime's generic application-specific dynamic tools now.
Both leads acknowledged that Runtime-only work is disjoint; no Piping adoption or
live binding is inferred. No substantive ownership/design disagreement is open.

| Worktree name / branch | Owner and current scope |
|---|---|
| `b3d208ad-bb6f-4bed-aaab-c567e28cbe23/chirality` / `codex/swbpipe-continuation-20260919` | Piping ROOT: graph, records, coordination, independent reviews and integration. Records checkpoint `7d5603ece`, later evidence pending. |
| `swbpipe-wt3` / `codex/swbpipe-b-shell-20260918` | B3 manager: product24f5d9db and test-only5045bd1c. Controller/shell/native repairs reviewed; complete source rerun/dist and closeout pending. |
| `swbpipe-wt2` / `codex/swbpipe-b3-20260919` | ROOT review checkout7afd3149, matching5045 product/tests; includes current main21175b5d and ROOT records. |
| `chirality-runtime-application-tools/chirality` / `codex/runtime-application-tools` | Peer: Runtime only. PR824 merged as `21175b5d3668f29acd408812d95a33ed4f14bfc8`; reviewed head `cb08dbe2f`. Clean worktree; no active writers or processes. Peer retains Runtime ownership. |

Piping retains domain schemas/tools, live controller, UI and future Tauri host
adapter. Peer is the explicitly acknowledged generic Runtime-contract writer.
No Root/shared-governance or Piping-domain ownership is transferred.

Active Piping work: B3 manager Astra/high completes full tests and evidence fan-in.
Compact-selector implementer and final native worker Astra/low, independent
structure reviewer Astra/high, and code reviewer Astra/xhigh have returned and
backchecked their bounded work. Original native worker completed its Sol/high
assignment. All new
formerly-Sol deployments use Astra/low. Canvas `f6c0bab8e` and design-system
`4dddd4412` stay parked. Native popup prototypes remain diagnostic and unadopted.
B3 native/full-check/review/sweep/CI gates remain open; no B3 merge is claimed.

## Resources

Peer claims no ports, servers or native/CUA use and will message before claiming
them. Piping's B3 manager serializes native/CUA; final repaired native/ROOT checks
released that slot. Independent headless slots5183/5184 and their lock are clear.
Source/dist E2E use5174/5175 and one worker; Tauri dev5173 and canvas probes5185/5186
require reservation. Use isolated test profiles/stores and private sockets; no
shared live user model. No builds/tests alongside timed D-72 measurements.

## Contract fit and corrections

The Rust operation engine and live controller are the right seam. However,
`workspaceSession.ts:handleQueueOperationBatch` captures the basis at arrival
and returns no receipt. Preserve a host-issued workspace generation, revision
AND canonical hash from inspection through queue and human acceptance; a naive
wrapper would silently rebase. Equal contents after Undo do not revive an old
revision. Success must confirm the live model/history commit, not a detached
engine result or scheduled React update.

Both leads support the discussion sketch: inspect selection, preview operations,
submit proposal and get proposal status. Human UI/controller acceptance applies
changes. Submit returns a review ticket promptly; Piping retains proposal status
beyond Runtime's transient call retention. Bound trusted context supplies workspace
and caller identity; later selection cannot retarget a proposal. Deduplicate by
proposal identity plus binding/invocation/caller context. Project switch, cancellation
or Runtime restart must never replay an application.

Peer reports this later owner steer: “Use what is the most effective. If there's
a way to more directly interact with the command layer, that could provide a
significant speed advantage.” Carry it as future adapter preference, not new
integration authority. UX §§2.2/2.4 define shared operation semantics with distinct
actor/acceptance records; §2.8 makes keys accelerators for existing controls.
Prefer typed command IDs/arguments, availability checks and outcomes through the
same controller. Presentation commands may complement the four tools. Literal
agent keypresses do not establish human authorship or human acceptance.

ROOT inspected the proposed generic contract at `da95ec194`; peer's independently
backchecked cancellation repair `cb08dbe2f` leaves that API unchanged. PR824 merged
at `21175b5d3668f29acd408812d95a33ed4f14bfc8`; ROOT fetched and verified that live
main change touches only Runtime. This is the proposed pin for future adoption. See Runtime
`docs/APPLICATION_TOOLS.md` and `packages/contracts/src/application-tools.ts`.
Consumer-fit review found no substantive blocker; this is not Piping adoption.
Peer reports401 Runtime tests plus focused repair/backcheck coverage; actual stock
tool turns/resume/inheritance and SWBPIPE mutation remain unqualified.

Missing Piping work includes a Tauri process/private-socket adapter, packaging,
lifecycle/recovery, effective-home configuration and controller handlers. Existing
Electron launcher/Node client are not that adapter. Pin a reviewed Git revision.
Pinned Codex0.154 catalogs are set at thread start, immutable across resume; the
first target needs a fresh tool-enabled session. Start with the main HELP_HUMAN
session until descendant inheritance is qualified. Offline intake is not live
binding; submitted author metadata is untrusted. Runtime's legacy governance
proposal envelope is not OperationBatch. Tool permission and model acceptance
are distinct; domain-route equivalence does not isolate other user-enabled tools.

## Recommended order and first journey

Finish B3, preferably B3A/B3B dirty/save/busy behaviour, then an explicitly
authorized Piping adoption/journey tranche. Agree exact Runtime revision,
schemas/basis/receipts, failure behaviour, scopes and resource slots before writes.
Do not wait for the whole closing visual pass. Peer acknowledged this sequence
and is not scheduling Piping work.

First target: one supported load-magnitude update in an invented fixture. Inspect,
preview without mutation, submit, human accept once, verify value/history and
Current-result invalidation, then Undo/Redo without reviving Current results.
Reject stale-before-arrival and stale-after-preview, including equal-hash-after-Undo;
cover duplicate delivery, project switch and cancellation. Recommended, not committed
for execution. Prove this ad hoc journey before authoring reusable workflows.

D-58/DEC-091 successor/client adoption and DEC-042 live-binding holds remain.
DEC-051 and CONTRACT OPS-K-PRIV-1 already permit owner-configured provider
transmission without another app-side consent/residency gate; preserve that ruling
alongside public-commit/IP and telemetry boundaries. No reconciliation/DAG rebuild.

Basis: named Git revisions; Piping workspaceSession, operationBatchService and
OfflineProposalIntakePanel; Runtime contract/docs/supervisor; DEC-051, D-58/DEC-091;
peer's acknowledged direct task replies. Peer test claims are identified as such.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
