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
    then answers the gate `READY`. The journey runs the gate in handshake mode
    (`SWBPIPE_NATIVE_SMOKE_AGENT_GATE_MODE=handshake`, native-smoke only): it
    holds the original supplier unreaped, with no deadline, until the
    controller answers `READY` or `REFUSE` or the app cancels, so admission
    always happens inside native custody and the supplier's id cannot be
    reused meanwhile. Without that variable the gate keeps the ten-second
    wait the live-session fixture expects, and any other value refuses the
    start. Admission is made once and never retried; if it is refused, the
    controller answers `REFUSE`, native cleanup ends the supplier as the
    product does for a failed start, and the run fails;
  - rescue is the anchor's own group, never a recorded PID. A launch that does
    not end itself has a gate never answered refused (so native cleanup ends
    that supplier), gets a bounded grace (default 120 s) to end itself, and is
    then released by `release_app` from evidence only: the owned app is killed
    by its own handle only when exactly one supplier was published and either
    an anchor holds its group or the kqueue observer saw that supplier exit.
    Otherwise the app may be a supplier's only custodian and is left running,
    the run fails, and a host latch
    (`/private/tmp/swbpipe-agent-journey-<uid>.retained`), checked under the
    host lock before any key is read or run state created, refuses every
    further attempt until a person has quit that app normally (its quit
    performs native cleanup), confirmed nothing remains and removed the latch.
    The driver retries Start only on the fingerprint refusal, which comes before
    anything is registered or spawned, so a run publishes at most one
    supplier; a second published supplier fails the run. After a run the
    supplier's zero exit and absence are required, and a read-only listing of
    executables must find nothing under the app copy, the run's HOME or the
    supplier's runtime scope;
  - the fixture mode's standalone preflight (probes only, no supplier or relay)
    runs before each launch, the bundle's supplier must match the pin in
    `agent-witness-pin.json`, one attempt runs at a time on the host, and a
    system free-memory gate (default 50 GiB) applies;
  - it judges: the seven driver steps; each turn completed (the bare standing
    "Ready."; a failed or interrupted turn is not) with an agent message in the
    window; exactly one agent batch whose count is exactly the fixture's 11 and
    whose submitted operations, as Batch review shows them, are the fixture's
    ids, kinds and targets in order; a receipt after Apply; every node, pipe
    and support row the tree shows, equal to the entities the batch creates;
    `node:c`'s six nodal motions as the Results section shows them, each finite
    and in its unit, against `validation/references/w9_benchmarks/references.json`
    (W9-A) under that file's criterion (relative 1e-9, zero rows on the family
    scale); and the agent's report in the window giving exactly one
    `node:c <component> = <value> <unit>` record per component (turn 2 asks for
    that form), in its unit and equal to the window's value (exactly with the
    stub; to four significant figures live, where a model may round; a zero
    as zero).
- **Provider key substitution** (option (b), live mode only),
  `apps/desktop/src-tauri/src/native_smoke.rs` and `embedded_agent/live_host.rs`.
  With `SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY=stdin` and the driver on stdin, the
  first stdin line is a provider key; any other present value of the variable,
  including one that is not Unicode, starts no driver, so a key line can never
  reach the driver reader. The key line is checked by `Secret::new` and bounded by
  the relay's secret limit; nothing about a refused key is echoed. It is held
  only in process memory until the next Start with a provider saved with no
  key takes it, once. It is never written, logged, given to the webview or put
  in the plan, environment or report, and the Keychain is neither read nor
  written: the driver saves the provider with no key. The controller passes it
  with `--key-stdin` from its own stdin, reading one line of at most 8192
  printable ASCII bytes that must end in a newline, refused without echo. Both compile only under the
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
  also requires the solve to be on the applied model's hash, the agent to have
  read exactly the six rows the window shows (same ids, kinds, entity, units
  and values), and the agent's report in the window to state each value it
  read.
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

## Defects the journey found

The first launches exercised the webview journey for the first time and found
that the merged agent could never start in the real app: the workspace
snapshot compared two different revision counters, and the workspace's
`sha256:` hash form failed the catalog's bare-hex `hash` schema at `register`.
Both were fixed in their own PR (#40, merge `cc91aadc9`), with regressions;
`agent-live-session.md` records them and the binding rule.

To read a refusal the Agent panel generalizes, a native-smoke build also
prints the host's refusal code and the request's kind on stderr
(`[native-smoke-agent-host] register refused AGENT-Input`); the code is a
payload-free enum and the request is never printed.

## Stub run

Candidate: native-smoke app, driver and controller from `cb3897f1c` (the
third review's fixes, including the handshake gate), with the wasm engine and
frontend built from `bd1c8eff3`, unchanged since; macOS ARM. The bundle was a private copy of the
retained bundle at `targets/appv4-host/aarch64-apple-darwin/debug/bundle/macos/SWBPIPE.app`
(the originals untouched), whose supplier matched the pin
(`112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b`), with the
fresh executable (SHA-256
`c23a852d85fc4e07e50b7b982e75e72c430def174958fceba66dfd60c7d519a7`) placed in it.

Seal: ad-hoc local seal of a private test copy, not distribution signing
(`codesign --force --sign - --options runtime` on the app copy only; no
identity, team id or Keychain item). Its identity output was
`Signature=adhoc`, `TeamIdentifier=not set`,
`CodeDirectory flags=0x10002(adhoc,runtime)`. The supplier was not sealed or
re-signed; its SHA-256 and its own signature verified unchanged before each
launch.

The evidence run, 2026-10-31T14:14:30Z to 14:14:50Z UTC, under `t3_slot.sh`:

| Check | Observed |
|---|---|
| Free-memory gate | 49.3 GiB free; this stub run used a 16 GiB gate (no model runs), the live default stays 50 GiB |
| Standalone preflight | passed (probe exits, boundary checks, `preflight_complete`) |
| Gate | handshake mode; one supplier published, observed, anchored to its group, then answered `READY` |
| Driver | all seven steps, each turn ending "Ready." with an agent message; one agent batch whose submitted operations are the fixture's 11 in order; receipt after the stand-in Apply |
| Stub provider | summary, propose, message; summary, observe (`applied`), solve (`completed`), read results, message; no unexpected step; no request carried an Authorization header |
| Window | the tree shows exactly nodes `node:a`, `node:c`, `node:d`, pipes `pipe:a-c`, `pipe:c-d` and supports `support:a`, `support:d`; `node:c` ux 4.771112073942932 mm, uy -3.586865438767269 mm, uz 0, rx 0, ry 0, rz 0.0007867143613246695 rad, each within W9-A's criterion |
| Agreement | the solve was on the applied model's hash; the agent read exactly the window's six rows; its report in the window gives one `node:c <component> = <value> <unit>` record per component, each equal to the window's |
| Cleanup | app exit 0; supplier exit observed with status 0 and absent; `qualified_native_owned_group`; no resident process |

The result JSON SHA-256 is
`3d30ebf1bc5c767d8ce6d9f05cbfb57abfe77f8c45ca7efa3eb4e3cbb15e72e6`; it holds
no credential and is a local report that no permanent check reads. The same
journey had passed on earlier candidates (11:37 UTC with a draft of #40's fix,
12:31 UTC on the merged fix, 13:42 UTC with the second review's fixes); review
then replaced the timed gate with the handshake and tightened the judge, so
it was run again on this candidate. Each run used a fresh private bundle
copy, removed after.

Earlier attempts, each a short launch that left nothing resident (run 1's three
listed "residents" were the controller's own wrapper, whose arguments name the
app; the check now matches executables): runs 1 to 9 (11:13 to 11:34 UTC)
stopped before the supplier started, while the two defects and two driver
waits (the bundled demo's load, Start's refusal line) were found and fixed;
run 10 (11:36) completed the journey and was judged against row ids of the
wrong shape, which the judge now matches as the product forms them; run 11
(11:37) passed on the draft candidate. These
runs qualify the scenario's plumbing with a scripted provider, not model
behaviour.

## Open

- Criterion 17's evidence needs an owner-attended Apply (D4 option 2); this
  scenario's stand-in click does not supply it.
- The live run's key read waits for the owner's approval.
