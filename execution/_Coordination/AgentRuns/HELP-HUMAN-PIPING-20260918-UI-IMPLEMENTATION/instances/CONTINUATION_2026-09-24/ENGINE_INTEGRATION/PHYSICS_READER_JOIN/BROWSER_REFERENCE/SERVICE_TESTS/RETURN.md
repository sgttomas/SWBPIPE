# Service tests and replay helper — source handoff

Owned source: `apps/desktop/src/services/previewService.test.ts` and `apps/desktop/src/test/nativeMechanicsReplay.ts` only. No production source, raw fixture or quality field was changed.

The helper exposes `nativeMechanicsReplayPair(mode?)` and `createNativeMechanicsReplay({jobIdPrefix?}) -> {invoke,notice}`. It clones the exact full-UI model/result pairs recorded in UI_PRODUCER_CAPTURE.json, refuses mismatched or omitted request/model/mode and unknown jobs/commands, and supports direct/start/poll/cancel mocks. It installs no host, global mock or registrar. Every use is explicitly a unit transport replay, NOT actual native UI qualification.

Service tests use the explicit reference loader for old byte/mode/copy oracles. Browser omitted/null/unchanged and edited requests refuse. New native-boundary tests cover exact returned-object provenance, retained request defensive copies, imported/cloned/reparsed data, changed model/mode, signed-zero source mutation, asynchronous request capture, transport mutation, unknown/running/completed/replayed/cancelled/failed/mismatched jobs. Original nonlinear0.411203/41.120279, iteration2 and exact reference analysis hash/unit/dimension assertions remain.

Readonly AST parse reports0 syntax diagnostics; scoped diff whitespace check passes. No npm/Cargo/tsc execution was authorized for this assignment. Tests remain queued for a parent-granted lane. EXECUTION.json pins owned sources and verifies both producer model/output pair hashes against the actual capture receipt. Parent retains native product witness and production corrections.

## Resolved follow-up and static readiness

The parent supplied a genuine ordinary precision full-UI producer pair, captured in PRECISION_UI_COMPANION.json. The replay helper now accepts explicit `{profile: "precision"}` while defaulting to physics; source/model/mode mismatches still refuse. Both source families remain unchanged captured data. Ordinary pair source counts67/68 are asserted in service controls.

Report-package Current tests now use the ordinary precision pair. Historical component/hanger/unknown-row oracles call only the pure result projection and renderer-input composition. New package tests preserve the exact missing-native-invocation and physics-transport-unavailable errors. The parent explicitly withholds qualified physics report transport in this tranche while retaining canonical physics export. The existing origin enum vocabulary is retained only for clearly unqualified historical-format protocol projections, as directed; no new schema identity or Current export is claimed.

All owned files parse with0 syntax diagnostics and pass scoped whitespace checking. Static edits are ready; runtime tests and typecheck remain unrun until the parent’s coordinated parity/WASM/test lane. EXECUTION.json binds the updated owned files and exact ordinary producer pair hashes.
