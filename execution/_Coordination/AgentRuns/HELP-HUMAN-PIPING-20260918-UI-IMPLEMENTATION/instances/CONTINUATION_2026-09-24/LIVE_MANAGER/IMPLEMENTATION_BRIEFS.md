# Bounded implementation handbacks and TASK briefs

Proposed by LIVE_MANAGER for ROOT activation after independent preparation review, exact wire acknowledgement and actual B4 handback. These written briefs are not executing children. No child was spawned during preparation. ROOT alone owns the work graph, central coordination, Git/index/branch operations, PRs and loop receipt. Model direction retained: WORKING_ITEMS Astra/high; TASK Astra/low; independent nonimplementing reviewer Astra/xhigh. Before any future TASK spawn, parent reads and supplies the full canonical agents/AGENT_TASK.md and records its hash. Resolve REPO_ROOT by Git, WORKING_ROOT=REPO_ROOT/projects/chirality-piping. All paths below are relative to WORKING_ROOT.

## Manager undertaking

Implement owner-authorized CLI-first inspect/preview/submit/status for Node position.x single and atomic batches against the one Piping live controller. Preserve the entire WIRE_PROPOSAL publication/recovery contract and accepted domain contract. Sequence: independent prep review → resolve findings → both writer ACKs to same bytes → ROOT grants exact ownership → native and frontend implementation → connected native integration → fresh independent complete-candidate review and checks → actual human single/batch witness → ROOT programme integration. Native source can proceed alongside independent interface work only after ownership and isolated testing resources are explicit. No assumption of free ports/build caches/browser/native session.

B4 currently owns App/ModelTree/table/styles and desktop/native/browser testing. Finish compact fit first. On handback ROOT must state actual candidate and hashes for workspaceSession, operationsSessionState, BatchReviewPanel and lib/Cargo, plus whether test/build/browser/native lanes are released or isolated. Dirty or newer files are source-checked before use. No concurrent shared controller writer, even if changes look disjoint. Later Pipes/table work may continue only with a mutually acknowledged source and resource split.

## Native/CLI TASK — product writer N

Purpose: one private owner-local macOS JSON carrier with trusted registration/correlation and a development CLI. Domain params remain opaque. Load Root/project AGENTS, canonical TASK, parent brief, owner activation and frozen WIRE_PROPOSAL. No engine/model/persistence or Runtime writer scope.

Exact proposed owned files:
- apps/desktop/src-tauri/src/live_control.rs (socket service, admission, registry, Tauri register/reply/unregister, cancellation and shutdown)
- apps/desktop/src-tauri/src/live_control_wire.rs (shared serde wire/error/descriptor DTOs and limits; no domain model or app startup)
- apps/desktop/src-tauri/src/bin/swbpipe-control.rs (CLI; imports shared protocol module, never invokes desktop run)
- apps/desktop/src-tauri/src/lib.rs (only module declaration, managed registry, opt-in setup/shutdown hooks, and three invoke registrations)
- apps/desktop/src-tauri/Cargo.toml and Cargo.lock only if needed
- apps/desktop/src-tauri/tests/live_control_transport.rs and tests/live_control_cli.rs, or equivalent inline owned-module tests if public-module exposure is unnecessary
- docs/LIVE_CONTROL_DEVELOPMENT.md (native/CLI invocation, descriptor hygiene, exact limits/errors, canonical method examples and evidence boundaries)

Cargo layout is explicit in existing desktop package, preserving its desktop identity:
```toml
[package]
# existing fields retained
default-run = "openpipestress-desktop"
autobins = false

[features]
live-control-cli = []

[[bin]]
name = "openpipestress-desktop"
path = "src/main.rs"

[[bin]]
name = "swbpipe-control"
path = "src/bin/swbpipe-control.rs"
required-features = ["live-control-cli"]
```
Do not create a new Cargo workspace or Node package. Reuse serde/serde_json/getrandom and std Unix facilities already present. Explicit feature keeps normal desktop build/run from accidentally selecting/bundling the CLI; test and verify actual Tauri target resolution. The observed tauri-utils mainBinaryName merely renames the main executable; it is not the mechanism for excluding another binary. Existing src/main.rs self-test argument and desktop startup remain byte-unmodified unless ROOT grants a concrete adjustment. tauri.conf.json is read-only initially; report a demonstrated need before expanding scope. CLI build command: `cargo build --manifest-path apps/desktop/src-tauri/Cargo.toml --features live-control-cli --bin swbpipe-control`. Normal packaged app must retain the existing main target and `--self-test-saved-edited-load` route; do not package a CLI as the app executable.

Acceptance: framing/size/deadline/concurrency/permissions/symlink defense, auth redaction, wrong-app rejection, ready gating, exact main-webview source binding, stale/duplicate reply rejection, registration replacement, disconnect/cancellation correlation and unsupported-host compile guards. Controlled domain handler fixtures prove carrier only. Share exact request/response fixtures with frontend, including an old workspace status response; native must not apply a current-workspace filter that destroys recovery. Normal disabled startup must remain usable. Return source hashes, commands/raw outcomes, failed/unrun checks, changes to shared ABI, and ready-for-integration status to LIVE_MANAGER. No native launch without ROOT resource release.

## Frontend/controller TASK — product writer F

Purpose: add the narrow domain projection and observed-publication registry within existing workspace session ownership. Load same basis and full TASK. Expose no external Apply and no model/history setter.

Exact proposed owned files after B4 release:
- apps/desktop/src/features/workspace/workspaceSession.ts (one writer for registry lifetime, coherent current basis, queue/Apply/Clear/reset integration and committed layout observation)
- apps/desktop/src/features/workspace/operationsSessionState.ts (only additional operation-owned refs/cells if required)
- apps/desktop/src/features/workspace/liveControlTypes.ts and liveControlController.ts (wire domain types, frozen reference/idempotency registry, intent construction, request/outcome state machine; no second model store)
- apps/desktop/src/services/liveControlBridge.ts (native event registration/readiness/reply/unregister, latest-controller ref, cancellation lifecycle; inert browser path)
- apps/desktop/src/features/toolkit/BatchReviewPanel.tsx (only live ticket/origin/Apply route presentation and optional owned entry metadata, retain existing UI callbacks and behavior)
- new apps/desktop/src/features/workspace/liveControlController.test.ts, workspaceSession.liveControl.test.tsx; new apps/desktop/src/services/liveControlBridge.test.ts
- existing workspaceSession.shell.test.tsx only for genuinely affected contract assertions; never weaken the six-slice/no-setter boundary

App.tsx, modelSessionState.ts, existing table/inspector editors and styles are NOT default writer scope. Hook-owned registration can avoid App changes. Return a concrete dependency to ROOT if unavoidable, with one named integration owner. operationBatchService, hashService and core are read-only by default; use existing validate/apply/canonical hash services.

Source-specific requirements: external basis revision is uiModelRevision, while existing modelRevision remains the internal stale callback guard; guard project generation as well. Neither modelAssignment.status nor B4 ownedOutcome is publication evidence. Keep immutable observed transaction snapshots before async hash awaits. Atomic one-member and multi-member batches must create one transition/checkpoint. Legacy engine audit actor/runtime strings remain raw evidence; do not turn them into verified actor identity. Fixed external source classification `agent` means externally proposed operation, not authenticated Codex authorship. Trusted facts are local endpoint authorization, native dispatch/registered controller lineage and the locally observed review Apply route. An automated test can traverse that same route and must be identified as test-driver evidence in its actual execution record; it does not become human acceptance. Do not invent Codex thread/turn IDs or verified person IDs.

Frontend acceptance is TEST_PLAN.md controller/bridge rows plus unchanged relevant existing shared-session tests. Return exact source candidate, hashes and evidence; report resource needs before testing. Native fixture pass is not connected native qualification.

## Integration and independent review

LIVE_MANAGER reconciles the two returns, shared DTO fixtures and actual main-webview event wiring, then asks ROOT to activate serialized native evidence resources. Review covers the complete frozen diff, native security/lifecycle, frontend atomicity/publication, side effects of cancellation/restore and guidance/fixtures. Reviewer did not implement. Any repair reopens affected checks and review. ROOT owns overall candidate-bound evidence sweep, registered software checks and merge readiness. Neither manager completion nor PR merge lifts actual-human, usability, engineering or release boundaries.

Return to ROOT includes accepted writer handback, implemented status, native integration evidence, remaining real-human witness status and exact restoration/cleanup condition. No new permissions question is needed for implementation already authorized.
