# CLI-first live control — bounded activation proposal

Status: PROPOSED revision 2, following peer read-only fit review; owner activation remains pending. Prior supplied revision 1 is preserved at _run_records/live-control-proposal-v1/supplied.txt (SHA256 1890912d2430f2e51106a9d0741c2a5feb26d49e8ca348e8d9c1fed364425228). No live integration source or configuration has been written. Basis: merged fd195cf4287e84572a12183169478a6f7ddf6a92 / reviewed product 127677; subsequent B4 shared-controller changes require an acknowledged freeze before wiring this adapter.

## Decision requested

Authorize one development tranche exposing inspect, preview, submit and status through a small JSON CLI connected to SWBPIPE's live workspace. This is the bounded live-binding act reserved by DEC-042 and reaffirmed in DEC-103; the earlier CLI-first choice settles the method, not that activation. Preserve broader R7/Runtime adoption, solver/correlation, publication and independent-usability holds. No deliverable reconciliation or DAG rebuild.

The first supported primitive is an invented Node position.x edit through the existing modify/set_field operation, with the project's entered length unit and dimension. Demonstrate both a single edit and a multi-operation batch of coordinate edits using this same primitive. Preserve existing atomic batch semantics: a valid batch publishes once with one Undo checkpoint; an invalid later operation publishes none of the batch. Initial external tools cannot Apply. The existing user review/Apply route remains.

Completion of the requested loop requires a short real development-Codex → CLI → live App demonstration initiated by a human prompt, with the actual human reviewing/applying the proposal and the agent reading the confirmed outcome. Automated native/UI tests may establish implementation behavior first; they identify their actor as a test driver and never substitute for human acceptance. If the human is unavailable, implementation verification may finish while the human-approved loop remains explicitly pending. No broad agent-capability benchmark or autonomous engineering campaign is part of this tranche.

## Observable outcome and scope

An external Codex controller can inspect an explicit app/workspace and copied selected target; preview a supported change without mutation; submit the frozen preview to the actual review queue; and recover its real queued/committed/rejected/withdrawn outcome. User Apply publishes the same model/history/result transition as existing UI operations. Undo restores model content without making an old basis valid again.

Required controls: explicit app instance, workspace/project identity, generation, published revision and coherent canonical hash; trusted agent attribution at ingress; immutable target/operation payload; idempotency before fresh-basis checks; explicit cancellation/withdrawal semantics; no success inferred from scheduled React state or engine computation alone.

Recovery guarantee is deliberately bounded: the same running controller session retains key/payload/ticket/outcome associations across CLI process reconnects and project changes. App/controller restart expires old handles or returns outcome-unknown and requires reconciliation. It never blindly replays. Durable restart recovery is not part of this tranche and creates no new persisted field.

Only a private owner-local endpoint and explicit attachment descriptor are proposed. Requests name their target; connections do not supply an implicit current workspace. Bounded framing, authentication/registration binding and endpoint cleanup are required. First actual witnesses use invented fixtures and preserve other project rows and preferences.

## Writers and sequencing

ROOT retains integration, shared records, resource coordination and owner decisions. Existing allocations remain: WORKING_ITEMS Astra/high; bounded TASK Astra/low; independent fresh review Astra/xhigh.

1. **Shell/controller writer:** the retained B4 manager owns workspaceSession and associated UI state. Finish/freeze B4.1's shared-controller work, then give that same writer the live-control publication task. No concurrent writer receives those files.
2. **Disjoint native/CLI worker:** after the request/result contract is frozen, one ROOT-owned Astra/low TASK may implement native framing/dispatch and the CLI against a controlled handler fixture in parallel with B4.1. Component passes do not qualify the real UI connection.
3. **Connection and review:** integrate the frontend registration and private bridge only after the shell writer's acknowledged handback. Exercise the real native path before broadening the command set. One coherent integration candidate, affected development checks, complete independent review and one final clean sweep/hosted policy apply.

Proposed frontend scope, relative to WORKING_ROOT:
- apps/desktop/src/features/workspace/workspaceSession.ts and operationsSessionState.ts;
- new liveControlController.ts and liveControlTypes.ts beside them;
- new apps/desktop/src/services/liveControlBridge.ts;
- focused controller/session integration tests;
- toolkit/BatchReviewPanel.tsx only for truthful ticket/actor presentation.
App.tsx or model/project state hooks change only if a concrete registration/reset dependency requires it, under the same writer.

Proposed native/CLI scope:
- new apps/desktop/src-tauri/src/live_control.rs;
- src-tauri/src/lib.rs registration/readiness/reply wiring;
- a small Rust JSON CLI, preferably reusing current serde/getrandom/std Unix facilities;
- Cargo manifest/lock only as needed, preserving unambiguous desktop build/run and bundle targets;
- native transport/CLI tests and Piping developer guidance, including concise help/describe, one canonical request/result example and actionable error output.
Choose one CLI binary arrangement after checking Cargo/Tauri target behavior; do not introduce Node packaging or a second agent loop. Domain schemas/core operation semantics and shared Root/Runtime sources are outside scope.

## Publication boundary

Add a controller-owned transaction registry and committed-render observation. Queue acknowledgements require the exact entry, payload and basis in committed queue state. Apply acknowledgements require the published model transition, matching receipt/Undo checkpoint, removed queue entry and result invalidation.

Hash the corresponding immutable published model snapshot. Guard against acknowledging an interrupted or mismatched transition. Once a transition has actually been observed, a later edit/project switch must not rewrite that historical fact as “never committed”; keep its receipt distinct from the current workspace basis. Reset/unmount settles outstanding requests explicitly and stops readiness. No parallel model authority or second validation engine is created.

Before queue publication, cancellation can remove/invalidate the scheduled entry and leaves no published ticket. After publication, an interrupted CLI request does not withdraw the proposal or undo Apply. Same-key retry recovers the original ticket/outcome, including queue → lost reply → user Apply → retry. Different contents with the same key fail.

## Acceptance evidence

- Coherent hash/basis ownership, frozen target, busy/stale/withdrawn/unavailable outcomes.
- Preview with zero model/history mutation; delayed/failed publication cannot claim success.
- Duplicate/in-flight/reconnect recovery and conflicting-key refusal.
- Cancellation on both sides of publication; Clear, project switch, equal-hash-after-Undo, reset/unmount and late callbacks.
- Trusted agent attribution; no fabricated Codex identities or inherited GUI-user defaults.
- Native socket/CLI framing, authorization, registration correlation and restart expiry.
- Actual native CLI→live workspace single-change and atomic multi-operation journeys, including visible review, truthful Apply actor, model/history/result invalidation, one Undo checkpoint per accepted batch, Undo and stale rejection. An invalid later operation must publish none of its batch.
- A short actual human-prompted Codex/CLI/App witness: the human reviews/applies, then the agent reads the confirmed outcome. Automated test-driver evidence is separate; absent human availability leaves this final witness pending.

No CLI attempt substitutes for the private live controller by reading/writing SQLite or invoking only the file-input solver. No MCP adapter, embedded Runtime agent, model-server change/inference, CAEPIPE installation/control or correlation campaign is included. Those later steps keep their already recorded direction and qualification boundaries.

## Basis

PIPING_LIVE_CONTROL_CONTRACT_DRAFT.md revision 3; peer four-point interface review and the two narrow single/batch and actual-human-witness clarifications; read-only LIVE_CONTROL_ACTIVATION_INVENTORY return; owner direct and peer-transmitted controller/CLI decisions. The current UI batch and native foundation are merged through PR828. Future MCP remains subject to modern2026-07-28 stateless qualification; the observed legacy-client failure is canonical in the merged Runtime packet.

