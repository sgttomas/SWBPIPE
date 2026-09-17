# Render measurement method notes — working

The approved targets remain unchanged. Application RAF, main-render submission, compositor presentation and physical screen display are different observations; none is relabelled as another.

Fresh read-only Astra diagnosis found that the installed Playwright trace's screenshot cadence is throttled, so its screenshots cannot measure the 16.7/33.3 ms orbit targets. Visual captures remain useful for observed UI state and conservative completed-capture latency bounds. See the owning immutable BENCHMARK_DIAGNOSIS packet rather than duplicating its evidence.

Primary-source research by ROOT identifies a candidate measurement path, not current runtime proof: [Chromium's jank investigation guide](https://chromium.googlesource.com/chromium/src/+/master/docs/speed/debug-janks.md) describes FrameSequenceTracker and FramePresented events and distinguishes a submitted compositor frame from one presented. [CDP Tracing](https://chromedevtools.github.io/devtools-protocol/tot/Tracing/) provides trace collection. VERIFY must inspect actual installed-browser categories/events, bind the relevant renderer/surface and measured orbit window, and retain the raw trace before treating this method as qualified. Expected event names from documentation alone are insufficient.

Record browser version/launch flags, graphics vendor/renderer/backend and instrumentation settings. The host's M5 Max identity alone does not show whether a headless browser used hardware Metal or a software rasterizer. Baseline/candidate comparisons require matching recorded conditions. Hardware-capable production browser and native lifecycle observations remain distinct evidence classes.

No physical-display latency claim or minimum-hardware claim is introduced.

### Presentation qualification and cohort separation, 2026-09-15

The first completed trace-off baseline session remains run 1 of the five-run comparison; its original wrong-selection and out-of-frustum failures remain preserved. Four further baseline sessions are owed under the accepted V6 actionability/teardown correction. The timed path for the 180 actionable probes is unchanged; correcting an untimed null-oracle guard does not erase the first run.

ROOT authorizes a short candidate camera/visual and explicit CDP compositor preflight after the complete-slice build lease. Playwright trace/video stay off. The existing screenshot/identity endpoints remain comparable conservative bounds for baseline and candidate. A separately labelled, narrowly traced timing cohort may establish browser presentation latency only after its method is qualified, with matching conditions and instrumentation limits recorded; it does not retroactively turn screenshot bounds into precise timing.

Chronological proximity alone is insufficient: the first presented event after a JavaScript submission may belong to an older pending frame. The extractor must connect the relevant app renderer commit/damage that includes the new state to the same compositor frame's validated presentation endpoint. Require exact action and generation identity, clock calibration, complete loss-free trace, and independently expected visible feedback. Event names and submission timestamps alone do not qualify.

Source guidance consulted by ROOT: [Chromium compositor frame reporting](https://chromium.googlesource.com/chromium/src/+/HEAD/cc/metrics/compositor_frame_reporter.cc) records presentation state and layer-tree identity; [event latency recording](https://chromium.googlesource.com/chromium/src/+/HEAD/cc/metrics/event_latency_tracing_recorder.cc) includes mouse events and surface/display trace identifiers. These moving HEAD sources guide the bounded investigation only. VERIFY must bind the actual installed Chrome revision and emitted trace shape before acceptance. No threshold changes are authorized.
### Trace marker category correction, 2026-09-15

Root inspected the prepared compositor tracer and found that its category allowlist omitted `devtools.timeline`, although its window markers use `console.timeStamp`. Chromium's `ThreadDebugger::consoleTimeStamp` emits the `TimeStamp` event in `devtools.timeline`, and the DevTools trace type declares the same category. The separately named `disabled-by-default-devtools.timeline.frame` is not a substitute. VERIFY was instructed to preserve any earlier attempt, seal the category correction prospectively, and require the exact marker sequence before interpreting a sample window. This changes measurement instrumentation only and does not adjust any acceptance target.

Primary source references:

- https://chromium.googlesource.com/chromium/src/+/99baeeafe29b54168bc18411791025d4a9419eea/third_party/blink/renderer/core/inspector/thread_debugger.cc
- https://chromium.googlesource.com/devtools/devtools-frontend/+/b88894b14f63f84c460f66cff8918b2b4d079eae/front_end/models/trace/types/TraceEvents.ts

### Box feedback endpoint clarification

Before any candidate box timing, ROOT proposed and independent VERIFY agreed that the unchanged 200 ms feedback target starts at the real pointerup that commits selection. User-controlled drag duration is reported separately, together with the full gesture-to-capture bound. The frozen eight intermediate moves, coordinates, filters, oracle and twenty box actions remain unchanged. Pair down/up identity and coordinates before accepting an observation. Existing point/filter baseline timing is unchanged, and the final execution binding must prove that after shared-harness edits.

## Causal browser-presentation method V2

ROOT accepts the independent source diagnosis `instances/BENCHMARK_DIAGNOSIS/causal_presentation_v2/FINDINGS_AND_METHOD_V2.md` as a prospective method only. The complete output manifest was independently rehashed. VERIFY owns a harness-only callback/content marker and exact main-frame/surface/reporter lineage. Browser-reported presentation is explicitly distinguished from physical scan-out and native timing. The selected observer ceilings (0.5 ms p95, 2 ms maximum, 3% synchronous total, zero added RAF) invalidate an intrusive method; they do not change product targets or permit cost subtraction. Raw stream preservation must precede analysis and remain bounded/lossless. Runtime validation, complete lineage coverage and scored runs remain pending.


## Bounded preflight failures and new diagnosis — 2026-09-15

CAUSAL_METHOD_PREFLIGHT_V1 failed before any measured action on tuple coordinate indexing. The source correction passed that guard in V2, but V2 then failed postprocessing on selectionVisualOracle lexical scope before result persistence. Raw trace SHA71305baf7f1aa7ad5e49b67a454565332825c5c2feb58ef82414357db81821f0 is retained by VERIFY; it is not qualifying presentation evidence. The valid V2 return SHA49d33f576bbedb00155febc37f13760f9951b9fcd5f61fce612dedfba4da7324 identifies missing in-memory action/observer/trace-completion evidence.

A fresh TASK Astra/xhigh diagnosis now confirms that the preflight used canvas-local point coordinates as page coordinates, unlike the already corrected full benchmark driver. The point before/after images are identical; actual event-target evidence was not persisted. Orbit label interception requires explicit input-target proof. No gestures are accepted merely because mouse commands completed.

The diagnostic read API also enables product paint-opportunity callbacks, adding an observed RAF per render. A wrapper-only zero count would not describe complete observation perturbation. UI is preparing an evidence-only patch to retain actual-render diagnostics and remove eager paint-opportunity scheduling; product remains frozen during TESTS. The nullable historical diagnostic field must remain truthful. Fresh causal presentation and observer qualification follows source repair and durable phase-journal repair. Owner workloads, p95/idle/resource targets and exact-ID requirements remain unchanged.
