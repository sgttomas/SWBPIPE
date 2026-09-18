# UI foundation fixtures and benchmark

This directory holds deterministic generated UI workloads and the opt-in
production benchmark for the professional modelling workspace tranche. The
fixtures contain invented values only. They do not establish solver capacity,
engineering validation, professional acceptance, or minimum hardware.

Regenerate and verify the frozen fixtures:

```text
node apps/desktop/e2e/ui-foundation/generate-fixtures.mjs
node apps/desktop/e2e/ui-foundation/verify-fixtures.mjs
```

The maintained fast functional regression is
`e2e/ui-foundation.spec.ts`; the ordinary Playwright configuration discovers
it. The long benchmark file ends in `.benchmark.ts`, so ordinary E2E and the
DEC-025 sweep do not discover it. Run it explicitly against an already built
production bundle from `apps/desktop`:

```text
UI_FOUNDATION_PHASE=baseline \
UI_FOUNDATION_EVIDENCE_DIR=<absolute .../instances/VERIFY/... path> \
UI_FOUNDATION_MANIFEST_SHA256=6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739 \
PLAYWRIGHT_WORKERS=1 \
<playwright> test --config e2e/ui-foundation/playwright.performance.config.ts
```

`UI_FOUNDATION_PIPE_COUNTS=1000` and `UI_FOUNDATION_RUNS=1` select a bounded
diagnostic run. Omitting them executes the required five fresh production app
sessions for each frozen fixture. A skipped, partial, proxy, or `N/A` record is
not qualification.

The historical baseline/candidate timing configuration disabled Playwright trace,
video, and automatic screenshots, retaining only named manual captures. Its
unwrapped-RAF observer description and legacy global-RAF proxy samples are
historical evidence, not the current full-cohort method. Current causal candidate
qualification wraps the genuine `requestAnimationFrame` callback path (and its
cancel counterpart) to retain registration/invocation lineage and observe actual
product callbacks. It does not schedule replacement observation frames. Wrapper,
marker, diagnostics, DOM and trace costs remain in the instrumented workload;
no overhead subtraction or comparable-baseline speedup follows from these runs.

Current teardown disconnects the tree observer, restores instrumented clear and
RAF/cancel functions, and removes pointer/input listeners while preserving prior
errors/overflow and recording untouched pending native handles. Cleanup is a
required recorded result, not an assumed success. Browser compositor lineage and
bounded intervals are not proof of physical display scanout. Partial, failed or
focused runs do not establish current performance acceptance. Earlier assignment
capture/CDP ordering describes its original route; current full-cohort collection
and interpretation are defined by the maintained causal controller and extractor.

The corrected baseline point lane has a smaller, separately labelled
feasibility mode before its five full N=1,000 sessions:

```text
UI_FOUNDATION_PHASE=baseline \
UI_FOUNDATION_EVIDENCE_DIR=<absolute feasibility evidence path> \
UI_FOUNDATION_MANIFEST_SHA256=6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739 \
UI_FOUNDATION_RUNS=1 \
UI_FOUNDATION_FEASIBILITY_SAMPLE_COUNT=3 \
<playwright> test --config e2e/ui-foundation/playwright.baseline-points.config.ts
```

Unset `UI_FOUNDATION_FEASIBILITY_SAMPLE_COUNT` for the required five-session
run. Feasibility rows are never aggregated as qualification samples.
The trace-off feasibility additionally records before/after JavaScript heap,
Chromium-reported process IDs with an OS RSS snapshot, and the exact size and
capture elapsed upper bound of every explicit screenshot. These observations
are outside individual point timers, and no capture or instrumentation cost is
subtracted.

Baseline fixture injection replaces only the existing bundled
`invented_preview_model` module response. It leaves product source unchanged.
The baseline uses the visible Iso preset and real canvas orbit gesture. Current
camera state is unobservable and remains so in evidence. Point samples switch
the real Labels control OFF and send ordinary pointer input through the canvas.
Before every timed point, the driver uses the real project-tree row to restore
exclusive Project selection; that reset is recorded separately and excluded
from point timing. The timed gesture must publish a new action- and
generation-associated render submission, so a repeated oracle hit cannot pass
from stale selection state.
The point start is the capture-listener `performance.now()` observation of the
real `pointerdown`; the browser event's own `timeStamp`, matching `pointerup`,
pointer ID, and both target identities are retained separately. After exact
feedback is observed, the browser screenshot starts immediately. There is no
extra harness RAF wait in the point interval.
Each point also retains a pre/post 96-CSS-pixel ROI centred on the independently
projected expected primitive, decoded pixel hashes and phase-specific selected
color counts, plus separate visible selection and inspector-heading PNGs. The
baseline uses its unchanged orange source color. Candidate runs require the
separately frozen theme-specific token, minimum interior-pixel count, and local
rendered contrast rule; they cannot reuse the baseline orange. Overlapping
primitives still require the exact typed state and independent hit oracle in
addition to the pixel cue.
The independent oracle follows the SHA-bound point-hit policy V3. Twenty of the
N=1,000 anchor projections and nineteen of the N=10,000 projections lie outside
the baseline Iso frustum; they remain unattempted baseline limitations. All 200
candidate point actions remain required after candidate Fit and preset readback.
Labels return ON before label-budget, visual, and orbit conditions.

Candidate strict qualification additionally needs these read-only observations:

- app-level parsed-model assignment start and generation identity;
- generation-bound main-frame submission plus a browser-observed capture;
- selection, box, and filter state after paint, including typed ordered refs;
- camera position, target and up vector;
- renderer memory/render counters and app-owned pending RAF count;
- a world-anchor-to-canvas projection query used only to aim a real canvas
  pointer gesture after an offline independent oracle has frozen expected hits.

These observations do not assign models, invoke selection handlers, or bypass
normal product commands. Frame callbacks alone are paint opportunities and do
not prove compositor presentation. Candidate timing uses a browser-observed
capture as a conservative upper bound where required.

`observer-contract-binding-v1.md` preserves the historical observer boundary.
For the current instrumented causal/full-cohort route, use the maintained
`benchmark-harness.ts`, `full-cohort-controller.ts` and causal qualification
contracts with their exact prospectively frozen source/environment bindings.
The current callback wrapper described above must not be attributed retroactively
to the historical baseline. Typed tree rows use
`tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)}` so distinct
entity types sharing one raw ID cannot collide.

Before the candidate five-run benchmark, execute the separate untimed camera
preflight with a real standard preset and Fit:

```text
UI_FOUNDATION_EVIDENCE_DIR=<absolute .../instances/VERIFY/... path> \
UI_FOUNDATION_MANIFEST_SHA256=6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739 \
UI_FOUNDATION_CAMERA_RECIPE=isometric_then_fit_model \
<playwright> test --config e2e/ui-foundation/playwright.candidate-preflight.config.ts

node e2e/ui-foundation/freeze-candidate-point-oracle.mjs \
  --preflight-dir <absolute evidence/candidate-camera-preflight> \
  --output-dir <absolute evidence/candidate-point-oracle> \
  --visual-tokens <absolute frozen VISUAL_TOKENS_V4.json> \
  --cue-source <absolute frozen src/features/viewport/viewportSelectionPresentation.ts> \
  --geometry-source <absolute frozen src/features/viewport/viewportSelection.ts>
```

The three source/token inputs must be the exact prospectively bound candidate
files required by the freezer parser; do not substitute later mutable postimages.
The preflight contributes no measured samples. It must report 200/200
actionable probes for both fixtures and the offline freezer must independently
match every product projection before candidate timing. Pass
`UI_FOUNDATION_CANDIDATE_ORACLE_DIR=<absolute evidence/candidate-point-oracle>`
to the candidate benchmark. Nominal workload poses in the fixture manifest are
reference drafts. They do not authorize hidden camera mutation or benchmark
controls; the evidence freezes the actual recipe reached through the standard
Isometric and Fit Model controls.

Orbit qualification is a separate Chromium compositor measurement. Run the
opt-in compositor preflight first and inspect the captured category/event
inventory, browser version, launch flags, WebGL renderer, and SystemInfo GPU
backend. Playwright screencast frames and global RAF callback opportunities do
not qualify the orbit frame-gap targets.
The compositor configuration also disables Playwright trace, video, and
automatic screenshots, so its only trace is the explicitly bounded CDP trace.
The preflight requires exact ordered markers and a `tracingComplete` payload
with `dataLossOccurred === false`; loss, unknown completeness, or flush timeout
cannot qualify. Event names and raw flow/track arguments are inventory until a
later method binds the actual app renderer/surface and presented-frame gaps.

The same five fresh N=1,000 baseline point sessions also retain the startup
capture and twenty tree-filter captures. Filter baseline and candidate both use
the same next-RAF observer wait followed by the same viewport PNG screenshot
and post-capture page clock. That wait is retained overhead, not compositor
evidence. Wrong-feedback point durations stay in the observed p95; a bound over
target means the conservative method did not establish the target and does not
prove physical feedback itself exceeded it.

Owned-resource replacement qualification is also an explicit separate run:

```text
UI_FOUNDATION_EVIDENCE_DIR=<absolute .../instances/VERIFY/... path> \
<playwright> test --config e2e/ui-foundation/playwright.candidate-resource.config.ts
```

It saves the routed generated fixture through the visible `Save local` control
and invokes the real `Open local` control twenty times. It validates current
and cumulative owned mesh, geometry, material, texture, instance-buffer,
control, event-binding, observer, RAF, and context counters at each boundary.
The production browser has no viewport-unmount command. Real React unmount,
whole-page reload, and native quit/reopen are retained as separate lifecycle
evidence; the E2E suite does not inject an App mutation bridge.

The stronger source-browser lifecycle witness is also explicit and separate:

```text
UI_FOUNDATION_EVIDENCE_DIR=<absolute .../instances/VERIFY/... path> \
<playwright> test --config e2e/ui-foundation/playwright.resource-lifecycle-source.config.ts
```

It serves a source-bound test-only page with visible controls around the actual
React `App`, performs five real mount/unmount cycles, checks zero owner counts
and RAFs after cleanup, and requires stale rejection of prior projection
requests after remount. It is not a production task or performance result.

The large-coordinate precision pair is a bounded regression, not another
benchmark workload. Load the base and translated variants through the same
real model-open path, invoke Fit, compare screen geometry/picks within the
frozen tolerance, and measure the frozen authored-node pairs. Fit and any local
render origin must not change model bytes or hashes; captured route points stay
in authored project coordinates.

### Conservative orbit window populations

Orbit windows remain 2 s warm-up and 10 s measured from the captured action. All qualified consecutive presentations in the conservative strict-outside-bracket envelope are retained, with their source occurrence indexes, endpoint intervals and reported timestamp groups. Where endpoint uncertainty permits multiple contiguous window populations, acceptance uses the largest nearest-rank p95 of the unchanged gap upper bounds across every admitted population; every cut and count is recorded. Equal reported timestamps retain distinct qualified occurrences and cannot be split at a boundary. Local boundary conditions conservatively over-approximate shared-clock feasibility. This is a bound for the qualified instrumented Chromium-reported metric, not an exact percentile, physical scan-out timing or uninstrumented performance claim. Targets, clock allowances, waits and observer-cost limitations are unchanged.


The orbit duration basis `same-trace-integer-us/v1` applies only to caller-bound
reported timestamps from one finalized, hash-checked capture. It pins Chromium
`507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c`, Perfetto
`da65f7e907e0caf473ddec16e15427465f503d05`, JSON exporter SHA256
`dc3a3b53cc2df3cd66be8b331b805d31331c2a8dbcf42e3f58d6fe8a05377667`
and trace-time source SHA256
`6a5fa3cf626a07016997c3bdaf32610577c45c9e0fccd60f174205793da7fe2b`.
The positive integer nanosecond export truncates to microseconds. For an exact
integer difference `d` in microseconds, the interval is
`[max(0,nextDown((d-1)/1000)), nextUp((d+1)/1000)]` milliseconds.
This bounds exported reported timestamps, not hardware presentation or scan-out.
On the pinned Mac Graphite/Dawn Metal -> OutputPresenterGL ->
ImageTransportSurfaceOverlayMacEGL path, the feedback timestamp is a future
display-time estimate computed before the CA tree commit. Ready and latch
timestamps are user-space samples; the assigned HWCompletion flag does not
establish physical scan-out. This producer-specific meaning changes neither
the reported metric nor its targets and does not rescore historical results.

The controller binds raw SHA, clock domain `MAC_MACH_ABSOLUTE_TIME`, document,
action, epochs, process/layer and already-qualified reporter references. The
scorer independently checks integer-pair enclosure but remains caller-qualified.
Wide page endpoint uncertainty, clock/isolation guards, complete envelopes,
timestamp ties and maximum p95 over every admissible boundary population remain.
Independent page-interval subtraction keeps its original semantics. No target,
wait, workload or observer-cost subtraction changes. Clock metadata is absent
from the retained transport result, so one bounded hash-checked raw read/parse
occurs after the measured window; it is discarded after binding. Its transient
allocation and live execution cost still require prospective qualification.


Assignment, point selection and box selection now receive a fixed 250ms host-only
collection tail after existing work and immutable stopped evidence/hash, immediately
before trace finalization. The tail adds no browser reads/actions, RAF, feedback or
render scheduling. Filter and orbit collection remain unchanged. This is a bounded
serialization budget, never a metric endpoint, subtraction, retry or guarantee;
missing/dropped/partial/ambiguous original reporters still fail. Finalization and
error persistence remain required even when work, stop, persistence or waiting fails.

The existing candidate benchmark supports explicit
`UI_FOUNDATION_DIAGNOSTIC_MODE=assignment-collection`: exactly five fresh sessions
for each of 1000 and10000, original assignment path, same-model responsiveness,
observer/restoration and pre/post bindings, and unchanged2000ms upper target.
Freeze workers1/retries0/maxfail1 at launch. It writes separate
`assignment-diagnostic-plan.json`, per-run `assignment-diagnostic-result.json`, and
`assignment-diagnostic-summary.json`; every result is an incomplete workload with
zero cohort contribution. Missing sessions fail diagnostic completion. Without the
flag the full candidate workload remains required; empty/unknown flags and conflict
with focused mode reject. The existing eight-segment focused proof is separate.
Chromium-reported timestamps retain their future-estimate limitation; neither this
collection tail nor diagnostic success establishes hardware timing or full-cohort
acceptance. Historical failed/unrun attempts remain unchanged.

## Explicit D-70 characterization (one attempt)

Qualification remains the default. Set `UI_FOUNDATION_COLLECTION_MODE=characterization`
only for the separately authorized candidate attempt. A valid original-target miss keeps
`FAIL_TARGETS` / `FAIL_COHORT` and its original numeric score, but permits the next run.
Invalid evidence, incomplete workload, binding/profile drift, cleanup failure, nonzero
settled owned rAF, or an existing timeout aborts the serial attempt. There are no retries
or replacement runs. The plan is five 1000-pipe sessions then five 10000-pipe sessions;
200 point, 20 box, 20 filter actions and both 2 s + 10 s orbits in each fresh session.
The baseline proxy and legacy focused/assignment routes are unchanged. Explicit
characterization conflicts with those diagnostics, unknown values and partial selections.

The characterization filter performs focus before arming, then
`page.keyboard.insertText(query)`: one whole-query browser input through Chromium's
`Input.insertText`, with no claim of keydown/keyup or physical keyboard events. The
existing first-input listener and final stopped-content witness remain unchanged.
Historical strict mode still calls Playwright `fill`, whose locked text-input
implementation also uses insertText. No clipboard, synthetic event, application handler,
or production mutation hook is used. Points keep labels off; boxes/filters inherit their
existing state; both orbit modes turn labels on. Boundary records retain actual labels,
canvas CSS and drawing-buffer dimensions, effective/browser DPR, camera, geometry,
conversion, theme/density/panels, model and run bindings. They do not prove continuous
foreground, occlusion or display placement.

Before launch, ROOT must provide the frozen final product build/source manifest for
`8468a33c86adb622b25e98f98b0eaf28c7e9fa0e`, distinct from the instrument checkout;
complete source inventory is the union of `source` and `mutableTestOnlySourceSnapshot`.
The manifest must include `productRevision`; the controller checks Git HEAD, final stage,
recursive src/src-tauri/dist/public-wasm coverage and all listed bytes. Freeze a method
manifest containing the **exact** `requiredMethodFiles` exported from
`full-cohort-controller.ts` (34 entries, including these new helpers/tests/declarations and
this README). Each `files` entry has `path` relative to the Piping project root and
`sha256`. Do not replace product source bindings with the instrument revision. Camera,
point/box/tree oracles, fixture manifest and browser executable must be independently
frozen and checked before the one timed attempt. Fresh complete-diff review, successful
smoke and ROOT's exclusive runtime lease remain required. The original 1000-only focused
preflight is not the both-size smoke below.

Reference profile JSON (`UI_FOUNDATION_REFERENCE_PROFILE`, absolute path, plus its
`UI_FOUNDATION_REFERENCE_PROFILE_SHA256`) has this exact field contract:

```json
{
  "schema": "ui-foundation.reference-profile/v1",
  "cohortId": "THE_FRESH_COHORT_ID",
  "productRevision": "8468a33c86adb622b25e98f98b0eaf28c7e9fa0e",
  "hostModel": "Apple M5 Max",
  "memoryBytes": 137438953472,
  "refreshHz": 60,
  "externallyVerified": true,
  "verificationEvidence": "/absolute/path/to/external-verification.json",
  "verifiedAt": "2026-09-17T23:00:00Z",
  "viewport": [1440, 920],
  "browserDpr": 2,
  "effectiveDprCap": 2,
  "display": {
    "name": "LG ULTRAFINE", "vendor": "1e6d", "product": "5bcb", "serial": "2011a",
    "pixels": "3840 x 2160", "resolution": "1920 x 1080 @ 60.00Hz", "mirror": "spdisplays_off"
  }
}
```

This is a schema example, not verification evidence. Use the actual externally verified
record bound to the attempt. Host CPU/RAM are read back; `/usr/sbin/system_profiler
SPDisplaysDataType -json` is persisted before the first trace and after each run.
Exactly one main online display must match the externally frozen identity and 60 Hz.
Missing or ambiguous profile is an abort, never a substitute machine or CPU acceptance gate.

Run from the instrument **Piping project root**. The runner's frozen input JSON must
supply these literal environment keys (never inherit stale values):

```text
UI_FOUNDATION_PHASE=candidate
UI_FOUNDATION_COHORT_ID=<fresh explicit ID>
UI_FOUNDATION_EVIDENCE_DIR=<new absolute empty attempt directory>
UI_FOUNDATION_COLLECTION_MODE=characterization
UI_FOUNDATION_PIPE_COUNTS=1000,10000
UI_FOUNDATION_RUNS=1,2,3,4,5
UI_FOUNDATION_MANIFEST_SHA256=<unchanged fixture-manifest hash>
UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST=<absolute final bundle manifest>
UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256=<manifest hash>
UI_FOUNDATION_CANDIDATE_SOURCE_ROOT=<frozen product Piping root>
UI_FOUNDATION_CANDIDATE_OUTPUT_ROOT=<product root>/apps/desktop/dist
UI_FOUNDATION_CANDIDATE_SOURCE_STAGE=final
UI_FOUNDATION_CANDIDATE_ORACLE_DIR=<absolute frozen runtime oracle directory>
UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256=<runtime oracle manifest hash>
UI_FOUNDATION_METHOD_MANIFEST_PATH=<absolute final method manifest>
UI_FOUNDATION_METHOD_MANIFEST_SHA256=<method manifest hash>
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=<absolute pinned Chromium executable>
PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256=<executable hash>
UI_FOUNDATION_REFERENCE_PROFILE=<absolute externally verified profile>
UI_FOUNDATION_REFERENCE_PROFILE_SHA256=<profile hash>
```

First run the non-qualifying smoke with a separate empty evidence directory. It opens
fresh sessions for **both sizes**, performs assignment, typed point selection, box drag,
whole-query keyboard insertion and both orbit controls against the actual frozen
production artifact. It validates camera/oracle controls and records observations;
it starts no trace/latency observer and calls no scorer. Keep its cohort contribution zero.
With the frozen environment above loaded, these are the literal invocations:

```sh
UI_FOUNDATION_SMOKE=controls UI_FOUNDATION_EVIDENCE_DIR="$SMOKE_EVIDENCE_DIR" \
  node node_modules/@playwright/test/cli.js test \
  --config apps/desktop/e2e/ui-foundation/playwright.candidate-performance.config.ts \
  --headed --workers=1 --retries=0 --repeat-each=1

# Only after smoke/review/final freeze and ROOT's exclusive timed lease:
env -u UI_FOUNDATION_SMOKE -u UI_FOUNDATION_DIAGNOSTIC_MODE \
  node node_modules/@playwright/test/cli.js test \
  --config apps/desktop/e2e/ui-foundation/playwright.candidate-performance.config.ts \
  --headed --workers=1 --retries=0 --repeat-each=1

# Only after candidate-cohort-result.json declares timedAttemptEnded=true:
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs \
  "$UI_FOUNDATION_EVIDENCE_DIR/candidate-cohort-result.json" \
  "$UI_FOUNDATION_EVIDENCE_DIR/offline-observations.json"
```

`candidate-cohort-result.json` retains the original cohort score, independent collection
status, and every attempted/failed/unattempted run. A fatal process kill may prevent that
final file: stop and produce an explicit incomplete-attempt report from canonical partial
files; never restart a timed run. The implementation does not automate pause/resume.
Original 20-minute run, 30-second trace IO/completion and 256 MiB trace limits remain.

Offline observations hash and losslessly parse the canonical raw traces without rewriting
them. Every attributable `ProxyMain::BeginMainFrame` start in inclusive
[action + 2 s, action + 12 s] enters the population, independently of selected feedback
frames. PID/TID, raw occurrence indices, ties and missing durations remain explicit.
GPU-related spans, when uniquely identifiable, are separate unbound thread context and
are summarized by event name, never pooled hardware timing or nearest-time causal joins.
Input latency, main-frame work and Chromium-reported presentation remain different
quantities; presentation is not scanout. Startup, loading, heap/process memory and resource
findings have separate applicability/unavailable labels; no resource or performance
acceptance is implied by completed characterization.

Focused node-only checks (no browser/build) use a run-owned Playwright config selecting
`full-cohort-controller.spec.ts`, one worker and zero retries, with output under that run's
evidence directory. Run the new characterization/inventory/metadata/profile tests plus
existing lifecycle controls; the raw-transfer >64/256 MiB stress tests need not be repeated
when their unchanged transport is out of scope. Offline verifier:

```sh
node apps/desktop/e2e/ui-foundation/verify-characterization-observations.mjs
```

## Owner-authorized continuation of the original attempt budget

The later owner clarification permits the next unattempted slot after an individual
failure, subject to independent cleanup and unchanged external bindings. This applies
only to the explicitly claimed characterization continuation. Strict/default collection
remains the full serial fail-fast route. Original `1000.1` is invalid and permanently
consumed; only `1000.2`–`1000.5`, then `10000.1`–`10000.5`, remain. One invocation runs
one slot; ROOT supervises each next launch. There is no retry/replacement loop.

Boundary metadata retains positive inner tree/inspector widths and heights as content
observations; selection, the frozen empty-tree query and overflow scrollbars can change
those dimensions. The retained continuation smoke records inspector width338→323 and
height299.96875→1530.625 with unchanged outer pane and canvas. Outer unique
`.workspace-pane-tree` / `.workspace-pane-inspector` x/y/width/height, inner selector and
visibility, canvas/DPR, model, theme/density and bindings stay invariant. The rejected
historical snapshot was not saved, so the precise historical differing field is unproved.
Future rejection writes the actual snapshot, reference identity/hash, field differences,
run/ordinal/method identity and error before propagation. Failed persistence is itself
reported. The both-size untimed smoke now witnesses metadata immediately after point
selection and after frozen sample18 filtering/clearing, without traces or consumed slots.

Freeze and independently review a new 34-member method manifest before continuation.
Create a policy JSON with these fields; all paths are absolute. `attemptRoots` keys must
be in the exact listed order, with nine distinct new directories whose parents exist.
`instrumentRevision` is the exact current instrument Git HEAD. `seed.path` must be the
canonical original RUNNER/RETURN.json beside the reference-profile path already bound
inside that original record; copying it elsewhere is rejected.

```json
{
  "schema": "ui-foundation.continuation-policy/v1",
  "seed": {"path": "/canonical/instances/RUNNER/RETURN.json", "sha256": "b9cd6955f39698ee611e1375153b81a9714e41c9320de745e099f93b5d91fee6"},
  "cohortId": "D70-8468a33c-20260917-CHARACTERIZATION-01",
  "ledgerRoot": "/approved/shared/continuation-ledger",
  "instrumentProjectRoot": "/instrument/projects/chirality-piping",
  "instrumentRevision": "<reviewed 40-character Git revision>",
  "method": {"path": "/absolute/successor-method.json", "sha256": "<64-character SHA256>"},
  "attemptRoots": {
    "1000.2": "/attempts/1000.2", "1000.3": "/attempts/1000.3",
    "1000.4": "/attempts/1000.4", "1000.5": "/attempts/1000.5",
    "10000.1": "/attempts/10000.1", "10000.2": "/attempts/10000.2",
    "10000.3": "/attempts/10000.3", "10000.4": "/attempts/10000.4", "10000.5": "/attempts/10000.5"
  }
}
```

The launcher exclusively creates `<canonical seed path>.continuation-ledger.json` to
pin policy hash, ledger path, method and revision. Existing mismatch fails; no reset or
relocation is allowed. Each immutable `claims/<slot>.json` consumes its slot **before**
subprocess launch. Missing result or terminal files cannot release it. Driver entry gets
another exclusive `entered-<slot>.json` seal, preventing replay. Original full preflight
still executes. Each subprocess retains its actual Playwright exit code/signal/error;
a failed slot is never relabeled passed merely to allow a later invocation.

ROOT provides a separately hash-bound receipt per next slot, from actual independent
checks. Its shape is:

```json
{
  "schema": "ui-foundation.continuation-preconditions/v1",
  "slot": "1000.2",
  "previousClaimSha256": "<seed SHA for1000.2; previous immutable claim SHA thereafter>",
  "verifiedAt": "<fresh ISO timestamp after prior claim>",
  "cleanup": {"status": "VERIFIED_NO_REMAINING_BROWSER_OR_SERVER", "evidence": {"path": "/cleanup-proof.json", "sha256": "<SHA>"}},
  "bindingsStatus": "VERIFIED_UNCHANGED",
  "externalBindings": {"<exact original frozenEnvironment entries, except the three keys below>": "<original values>"},
  "bindingEvidence": {"path": "/binding-proof.json", "sha256": "<SHA>"},
  "evidence": [{"path": "/independent-check-record.json", "sha256": "<SHA>"}]
}
```

`externalBindings` is `seed.frozenEnvironment` excluding only
`UI_FOUNDATION_EVIDENCE_DIR`, `UI_FOUNDATION_METHOD_MANIFEST_PATH` and
`UI_FOUNDATION_METHOD_MANIFEST_SHA256` (the policy binds successor method separately).
Preserve the original key order when generating this object. Cleanup proof has
`previousClaimSha256`, `browserProcessesRemaining:0`, `serverListening:false`,
`verificationStatus:"VERIFIED"`, and `processDisposition:"NORMAL_EXIT"` or
`"EXTERNAL_RECOVERY_VERIFIED"`. Missing terminal record, interrupted process or signal
requires the latter backed by real recovery checks. Binding proof has
`status:"PASS_INDEPENDENT_EXTERNAL_REVALIDATION"` and the same `externalBindings`.
These are evidence contracts, not permission to fabricate successful checks. The
launcher additionally rehashes method, browser, fixture manifest, runtime oracle manifest,
reference profile, source and build bytes before claiming. The normal controller rechecks
full profile/display/fixture/oracle conditions before collecting.

With `POLICY`/`POLICY_SHA` and each independently prepared `R_*` receipt path and `H_*`
receipt hash assigned literally by ROOT, execute the following **separate commands**,
only releasing the next after its independent checks. Do not put them in an automatic loop.

```sh
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 1000.2 "$R_1000_2" "$H_1000_2"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 1000.3 "$R_1000_3" "$H_1000_3"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 1000.4 "$R_1000_4" "$H_1000_4"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 1000.5 "$R_1000_5" "$H_1000_5"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.1 "$R_10000_1" "$H_10000_1"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.2 "$R_10000_2" "$H_10000_2"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.3 "$R_10000_3" "$H_10000_3"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.4 "$R_10000_4" "$H_10000_4"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.5 "$R_10000_5" "$H_10000_5"
```

Child environment is derived from the hash-bound original environment plus successor
method/output and launcher-only `UI_FOUNDATION_CONTINUATION_{POLICY,POLICY_SHA256,
CLAIM,CLAIM_SHA256,TOKEN}`. Stale inherited UI_FOUNDATION/PLAYWRIGHT variables are removed.
The command is fixed headed Playwright, one worker, zero retries, one repetition. Original
20-minute per-run and action/trace deadlines remain, with no new global cap. Launcher
progress reads only existing prepared/stopped/derived/result file names every ten seconds;
it installs no timed observer, reads no raw trace archives and changes no timing endpoint.

Each root gets `selected-slot-plan.json` / `selected-slot-result.json`; original per-run
scores remain in `raw/.../result.json`. The shared ledger gets actual terminal outcomes and
immutable `budget-after-<slot>.json` reports covering original1000.1 plus all remaining
slots. Consumed budget, valid complete workloads and original target outcomes are separate.
Original aggregate failure is retained. Mixed methods never become one ten-run qualification
cohort. Offline analysis accepts the closed selected-slot result via its existing CLI.

### Owner-directed one-success transition

The later `OWNER_DIRECTION_ONE_SUCCESS_20260917.md` supersedes exhaustive scheduling:
1000.2 already provides one valid complete actual run; 1000.1/3 remain invalid and
consumed, 1000.4 remains owner-interrupted and consumed, and 1000.5 is waived without
a claim. Attempt only10000.1–5 in order until one valid complete actual run. A valid
`FAIL_TARGETS` result satisfies characterization stopping without becoming metric PASS.

Keep the v1 policy's seed/cohort/ledger/instrument project/attempt-root map unchanged.
Set its new instrument revision and successor34 method, and add:

```json
{
  "ownerTransition": {
    "schema": "ui-foundation.one-success-transition/v1",
    "authority": {"path": "/canonical/D70_BASELINE/OWNER_DIRECTION_ONE_SUCCESS_20260917.md", "sha256": "ba5e8bceea55838cc0d23e815cc9a890ed543534085a9e9bdd932d133fb37fa2"},
    "previousPolicy": {"path": "/canonical/CONTINUATION/POLICY_V4.json", "sha256": "<original policy SHA>"},
    "registry": {"path": "/canonical/RETURN.json.continuation-ledger.json", "sha256": "<original registry SHA>"},
    "history": [
      {"claim": {"path": "/original/claims/1000.2.json", "sha256": "<SHA>"}, "terminal": {"path": "/original/terminal-1000.2.json", "sha256": "<SHA>"}, "return": {"path": "/original/RETURN_1000.2.json", "sha256": "<SHA>"}},
      {"claim": {"path": "/original/claims/1000.3.json", "sha256": "<SHA>"}, "terminal": {"path": "/original/terminal-1000.3.json", "sha256": "<SHA>"}, "return": {"path": "/original/RETURN_1000.3.json", "sha256": "<SHA>"}},
      {"claim": {"path": "/original/claims/1000.4.json", "sha256": "<SHA>"}, "terminal": {"path": "/original/terminal-1000.4.json", "sha256": "<SHA>"}, "return": {"path": "/original/RETURN_1000.4_INTERRUPTED.json", "sha256": "<SHA>"}}
    ]
  }
}
```

All references are exact hash-bound original files. The original registry is read-only;
`ledger/one-success-policy.json` exclusively seals the successor policy and waiver1000.5.
Historical claims validate against V4 and its method. New10000 claims validate against
the successor, preserving the same previous-claim chain across the explicit waiver.
Measured method files must match the predecessor; only the authorized orchestration,
declaration, tests and documentation entries may differ. No old claim/terminal/raw file
is rebound, rewritten or replaced. Once sealed, the old full-budget route is rejected.

Use the same five separate `launch-slot` commands for10000.1 through10000.5; never loop
or launch after success. The first receipt's predecessor is actual1000.4's claim and
requires `EXTERNAL_RECOVERY_VERIFIED`, plus fresh external binding checks. Each subsequent
failed/interrupted slot requires the same independent prerequisites as before.

A candidate success requires a matching launched run/session/method, complete243-segment
workload with200/20/20 action records, valid scorer evidence and actual exit0 without
signal/launch error. It blocks every further launch immediately, including stale commands.
Then independently verify cleanup/bindings and issue a close receipt using the existing
receipt schema, `slot` equal to the successful10000 slot and `previousClaimSha256` equal
to **that same successful slot's claim**. Run once:

```sh
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs close-one-success "$POLICY" "$POLICY_SHA" "$CLOSE_RECEIPT" "$CLOSE_RECEIPT_SHA"
```

Close launches no browser; it revalidates receipt and external files, exclusively writes
`one-success-complete.json` and `one-success-final-budget.json`, and explicitly waives only
remaining unattempted10000 slots. No valid actual run means close fails; allfive failed
attempts report exhaustion without fabricated success. Original per-run targets and failures
remain intact. Reports separate consumed, interrupted, invalid, valid/complete, confirmed
success after cleanup, waived and eligible slots. Method populations remain distinct.

### Explicit internal120 display successor

The owner separately authorized the internal Color LCD120Hz after the externalLG60Hz
became unavailable before any10000 claim. Retain all1000 evidence under its original60Hz
profile; this is a different display profile, not a same-profile repeated cohort. Add to
the one-success policy:

```json
{
  "displayTransition": {
    "schema": "ui-foundation.internal120-transition/v1",
    "authority": {"path": "/canonical/D70_BASELINE/OWNER_DIRECTION_INTERNAL120_20260917.md", "sha256": "6df272b7af0367b0083302229018143f8fa397a6bde0f5e368c6ec1e454157b9"},
    "previousProfile": {"path": "/canonical/RUNNER/REFERENCE_PROFILE_V2.json", "sha256": "117aac9e5792d3274cbe5e5d710612bce374fb091dc32d28d4dc2190326b5922"},
    "profile": {"path": "/canonical/RUNNER/CONTINUATION/INTERNAL120/REFERENCE_PROFILE_INTERNAL120.json", "sha256": "<sealed actual profile SHA>"}
  }
}
```

Only `UI_FOUNDATION_REFERENCE_PROFILE` and its SHA override the original external-binding
map; receipt/binding-proof `externalBindings` must carry precisely these two replacements.
All other seed external values stay exact. The profile retains schema/cohort/product/host,
M5Max128GiB, viewport1440×920 andDPR2; refreshHz is120. Its exact display identity contains
name `Color LCD`, vendor610, producta05f, serialfd626d62, pixels3456×2234,
resolution `1728 x 1117 @ 120.00Hz`, mirroroff and `connection:"spdisplays_internal"`.
Use actual sealed readbacks, never a synthesized successful verification.

Legacy profile validation remains60-only.120 is accepted only with the owner/hash-bound
successor policy and matching current10000 claim. Live before/after display checks compare
exact identity, connection, resolution/refresh and prior observation; missing, ambiguous,
60Hz or drifting120 observations reject. New claims and reports carry the new profile;
historical claims retain the old profile. The method transition permits only the display
validator block of characterization-commands.ts to differ, checking all source outside
that block byte-for-byte against V4; other measured files remain hash-identical. No new
observer, target, timing endpoint, workload or product change follows. One-success stopping,
remaining five-slot cap and fresh cleanup/source/profile prerequisites remain unchanged.
