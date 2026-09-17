# V63 — upstream orbit cadence throttling

**Conclusion:** V61's ~30 Hz orbit cadence is caused by explicit upstream Chromium frame throttling. It is not an inference from average presentation intervals or evidence that product rendering missed three of every four delivered frames. Low-battery Energy/Battery Saver is the strongly supported policy explanation, but the exact active browser preference/provider state was not captured, so that policy attribution remains an inference. No product or measurement-source correction is justified by these observations.

V61 remains a valid focused method run with both conservative orbit p95 bounds33.73300380709 ms exceeding unchanged16.7/33.3 ms targets. This is an observed instrumented battery-condition target failure; it must not be deleted or replaced by an AC result. V59's failed-window status and historical observations also remain unchanged. No full cohort or replacement score was calculated here.

## Bound trace comparison

All14 explicit inputs match their hashes. The adopted flatV9 manifest and V20V integration manifest match the brief; the inspected controller/extractor/harness/scorer match both manifests. Product, bundle, fixture, browser, oracle and sample identities in the two canonical records agree. Captured launch arguments differ only in the fresh temporary profile path. Both runs use CfT153.0.8010.36, revision507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c, ANGLE Metal/Apple M5 Max, headed1440x920/DPR2, reported120 Hz internal display and visible/focused documents. The before/after OS snapshots name CfT frontmost. These snapshots do not prove continuous OS foreground or physical VRR behavior.

Independent analysis examined only V59 centerline and the two V61 orbit raw traces declared in their owning manifests, plus the four declared derived records. The raw diagnostic window is action trace timestamp+2 s through+12 s; it is used to compare upstream cadence, not to recreate any accepted sample population.

| Diagnostic observation | V59 centerline | V61 centerline | V61 ActualOD |
|---|---:|---:|---:|
|Renderer Scheduler::BeginFrame events in10 s|1200|300|300|
|Every reported interval_us|8333|33333|33333|
|Every unthrottled_interval_us|8333|8333|8333|
|Every frames_throttled_since_last|0|3|3|
|Consecutive BeginFrame sequence increment|1|4|4|
|Renderer BeginFrameDropped events in window|0|0|0|
|Main proxy duration p95/max, ms|1.893/2.667|3.409/4.476|4.569/5.245|

The upstream Viz DisplayScheduler events continue to carry8333 us intervals. V61 centerline also records900 `SendBeginFrameDecision` events with reason `ThrottleRequested` and false send in that10 s window; V59 centerline has none. Those decision records have no frame-sink identifier, so they are supporting Viz-process evidence, not a fabricated per-event sink join. The renderer's own per-frame throttled counter and interval are the direct binding.

Concrete V61 centerline raw event16475 belongs to renderer39778, compositor thread59495945, sequence1709: interval33333, unthrottled8333, throttled-since-last3. V61 ActualOD event16185 has the same fields at sequence3586. V59 centerline event49538 belongs to renderer37112/thread59452742 and has sequence1489, interval8333, unthrottled8333, throttled0. Exact raw hashes and excerpts are sealed in RAW_CADENCE_ANALYSIS.json.

Across the complete qualified derived captures, V61 has376/375 unique presentations, median33.333 ms and p95 raw timestamp difference33.334 ms; their ranges are33.332–33.350 and33.317–33.357 ms. V59 has1500/1501 presentations, median8.333 and p95 raw difference8.334 ms, with a few tied timestamps/longer gaps. These are descriptive diagnostic statistics, not replacement performance scores. Ties and every original observation remain intact.

The source driver still uses its time-based gesture, awaited real moves and fixed waits. Observed move counts changed490/491→368/367; this is an outcome of the same closed-loop driver, not a new frozen sample count. The source does not identify which browser acknowledgement or timer accounted for that difference. The new population constructor runs after the action/extraction; it cannot account for a renderer BeginFrame trace explicitly arriving with a33.333 ms requested interval. Main-frame timings do not enclose every GPU/OS cost and are not an uninstrumented benchmark, but do not show a33 ms product work bottleneck. Optimizing product work cannot by itself make this capped source deliver120 frames/s.

## Pinned Chromium policy

All retrieved primary files use the exact revision above and have recorded byte hashes in PRIMARY_SOURCE_MANIFEST.json; failed path lookups are explicitly listed and unused.

- `battery_saver_mode_manager.cc:70–75,270–273,366–381,494,593–604,615–636` establishes a30 Hz all-frame-sinks throttle while saver is active, desktop activation on battery below the20% threshold for the relevant preference, and stopping the throttle when inactive. It also contains separate optional wake-up/process policies; their activation is not claimed here. [Pinned manager source](https://raw.githubusercontent.com/chromium/chromium/507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c/chrome/browser/performance_manager/user_tuning/battery_saver_mode_manager.cc)
- `prefs.cc:29–41` defaults the local preference to enabled-below-threshold. The enum/key are recorded in `prefs.h:53–61`. Defaults establish expected behavior, not proof of this run's effective preference. [Pinned preference source](https://raw.githubusercontent.com/chromium/chromium/507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c/components/performance_manager/user_tuning/prefs.cc)
- `compositor_frame_sink_support.cc:1158–1188,1539–1557` replaces the delivered interval with the configured throttled interval, reports/resets the throttled-since-last counter, and increments that counter in the explicit requested-throttle branch. Unresponsive and undrawn-frame branches are separate. `frame_sink_manager_impl.cc:1211–1218,1246–1257` applies/stops the global interval; the throttler also supports other requested/cadence sources. Thus the raw fields prove requested scheduling throttling, but are not a unique Battery Saver activation log. [Pinned sink source](https://raw.githubusercontent.com/chromium/chromium/507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c/components/viz/service/frame_sinks/compositor_frame_sink_support.cc)

V59 was23% discharging before/after; V61 was12% discharging before/after. Both recorded the same `pmset -g` text, including literal `powermode 1`; no semantic label for that numeric mode is inferred. The below-threshold policy, fresh-profile default, unchanged flags and explicit30 Hz throttle jointly strongly support Battery Saver. The packet does not capture its effective preference, browser-sampled percentage, managed-policy override, temporary disable state or activation call. Other throttle causes are not excluded solely by the policy match. Thermal state is also unavailable. No need to assert a unique policy cause to identify the actual scheduling cap.

## One bounded prospective discriminator

If the owner can supply stable AC power, ROOT should prospectively freeze **one ordinary headed AC reference qualification**, using the same product/bundle/browser/method, fresh-profile mechanism, existing launch arguments, fixture/camera/viewport/DPR, gesture/windows/waits and targets. Do not add unthrottling flags, override Energy Saver, change OS/browser power settings, alter the metric or perform repeated trials until a pass. The reference condition is a separately declared host condition, not a retrofit of V61.

Before measured actions, record external-power/charging status, reported display/refresh, ordinary power configuration, exact argv/profile identity and foreground state. If readily available through normal browser UI, record the existing Energy Saver preference/active indicator outside measurement; otherwise state unavailable rather than assume default activation. Retain the same after-run records and raw traces. Before/after samples alone remain point observations, not continuous power/foreground proof.

Predetermine interpretation: if AC removes the33333/3 requested throttle and ordinary8333/0 source cadence returns, this supports the host power-policy explanation; apply unchanged target scoring and retain either outcome. If throttling persists, inspect the recorded ordinary policy/host evidence before another run. If throttling disappears but gaps remain slow, distinguish missed frames and actual work/queue costs before proposing product repair. If AC is unavailable, hold reference qualification and retain the battery target failure; no alternative flags or lower targets are recommended. A successful focused AC result still is not the ten-run cohort or universal battery/native acceptance.

No runtime, browser, tests, maintained source, power setting, flag or Git changes were made. This packet is derivative diagnosis. ROOT owns power-basis agreement, launch authorization, fresh qualification and full-cohort admission; full cohort remains held. Existing true-observer-cost and Chromium-reported-versus-hardware-scan-out limits remain.
