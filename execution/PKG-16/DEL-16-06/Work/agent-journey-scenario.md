# Embedded agent W9-A journey scenario (RT-E1 D4, prerequisite 2)

The live session's native matrix (`agent-live-session.md`) qualified the
agent's process lifecycle with no window; the webview command and review
journey stayed unexercised. This scenario runs that journey in the real app
window: the agent builds W9-A from a blank model through one proposal, the
batch is applied through Batch review, and the agent solves and reports. It is
the vehicle for the D4 live run and is first qualified with a scripted provider.
It establishes no acceptance, release or criterion 17 standing.

## Parts

- **Driver**, `apps/desktop/native-smoke/agent-journey.js`. Evaluated only by a
  `native-smoke` build, through the existing `SWBPIPE_NATIVE_SMOKE_DRIVER` on
  stdin. It works the window as a person would: New Blank Project by the native
  menu dispatch, the provider entered in the Agent panel's own form with
  "No API key", Start agent, two typed messages, and the agent's batch applied
  by clicking Batch review's own Validate (when its preview gate asks) and
  Apply entire batch. That click is a qualification stand-in for the person,
  recorded as such, never the person's acceptance. It then reads the model
  tree, the Results section's rows for `node:c` and the solve proof, presses
  Stop agent and reports through `native_smoke_finish`. It calls only
  `native_smoke_plan`, `native_smoke_menu`, `native_smoke_log` and
  `native_smoke_finish`; no command, shortcut or IPC call applies anything
  (checked by `tools/agent_journey_fixture_test.py`).
- **Controller**, `tools/agent_journey_fixture.py`. It imports the live-session
  controller's discipline (`tools/agent_live_session_fixture.py`) rather than
  copying it:
  - the app is an owned Popen with a throwaway HOME, so its project store and
    provider settings are private to the run;
  - Start reaches the product's existing native-smoke agent gate
    (`live_runtime.rs`, `SWBPIPE_NATIVE_SMOKE_AGENT_GATE`), which publishes the
    app and original supplier identities and holds the protocol; the
    controller checks the app identity, observes the supplier by kqueue
    (exit, exit status, fork), joins an owned anchor to its group, and only
    then writes `READY`;
  - rescue is the anchor's own group, never a recorded PID; after the run the
    supplier's zero exit and absence are required, and a read-only process
    listing must find nothing naming the app copy, the run's HOME or the
    supplier's runtime scope;
  - the fixture mode's standalone preflight (probes only, no supplier or relay)
    runs before each launch, the bundle's supplier must match the pin in
    `agent-witness-pin.json`, one attempt runs at a time on the host, and a
    system free-memory gate (default 50 GiB) applies;
  - it judges: the seven driver steps; exactly one agent batch with the
    fixture's 11 operations; a receipt after Apply; the expected tree rows; and
    `node:c`'s six nodal motions as the Results section shows them against
    `validation/references/w9_benchmarks/references.json` (W9-A) under that
    file's criterion (relative 1e-9, zero rows on the family scale).
- **Provider key substitution** (option (b), live mode only),
  `apps/desktop/src-tauri/src/native_smoke.rs` and `embedded_agent/live_host.rs`.
  With `SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY=stdin` and the driver on stdin, the
  first stdin line is a provider key, checked by `Secret::new` and bounded by
  the relay's secret limit; nothing about a refused key is echoed. It is held
  only in process memory until the next Start with a provider saved with no
  key takes it, once. It is never written, logged, given to the webview or put
  in the plan, environment or report, and the Keychain is neither read nor
  written: the driver saves the provider with no key. The controller passes it
  with `--key-stdin` from its own stdin. Both compile only under the
  `native-smoke` feature.

## Provider modes

- `stub`: a loopback Responses-API script (`/v1/responses`, event stream) that
  answers as the model would, deriving each call from the product's actual
  tool outcomes: read the summary, propose the W9-A fixture on that basis;
  then read the summary, observe the proposal on its own basis, solve, and read
  `node:c`'s motions. Anything unexpected ends the turn and fails the run. No
  model runs, no network is used and no key exists. It qualifies the
  scenario's plumbing (supplier, relay, host validation, webview executors,
  Batch review and solve), not model behaviour. In this mode the controller
  also requires the solve to be on the applied model's hash and the agent's
  read values to equal the window's.
- `live`: the person-configured endpoint and model of the D4 plan. The key, if
  the provider needs one, comes from one read of the owner-supplied Keychain
  item, which waits for the owner's explicit approval.

## Reach

Product builds cannot reach the driver, the key substitution or the gate:

- `native_smoke` is declared under `#[cfg(feature = "native-smoke")]`, and the
  substitution in `live_host.rs` and the gate in `live_runtime.rs` sit under
  the same feature; `native-smoke` is not a default feature and no product
  build command passes it (only the smoke builds, such as
  `native-smoke/run.mjs`'s, do);
- the user-release reach scan pins `src-tauri/build.rs` by content, so a build
  script that sets `feature="native-smoke"` is a finding
  (`user_release_reach_scan_tests.rs`);
- the driver is a file under `apps/desktop/native-smoke/`, outside the
  frontend sources Vite bundles into `dist/`, and reaches the webview only
  through the feature-gated driver evaluation.

## Checks

Recorded on the candidate named in its PR: the native-smoke tests in
`native_smoke.rs` (key line before the driver, refusals without echo, taken
once), the embedded-agent tests with and without the feature, a product
`cargo check` with no features, the reach scan, and
`tools/agent_journey_fixture_test.py` (the stub's calls follow the tool
outcomes, the event stream, loopback-only answering, the judge's acceptance of
the reference and its named departures, and the driver's command set).

## Stub run

Pending: the stub run needs a private copy of the retained bundle that holds
the pinned supplier, named by its owner. Its start and end times, result and
cleanup evidence are recorded here when it has run.

## Open

- Criterion 17's evidence needs an owner-attended Apply (D4 option 2); this
  scenario's stand-in click does not supply it.
- The live run's key read waits for the owner's approval.
