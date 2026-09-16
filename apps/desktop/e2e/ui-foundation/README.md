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
