# V79 — GPU boundary diagnosis

The private GPU diagnostic supports a **producer future-estimate/deadline-phase explanation**, without a corresponding long serialized GPU fence or commit wait. It does **not** identify a justified product or scorer repair. Both original upper-p95 results remain **33.33400000000001 ms FAIL** against unchanged centerline **16.7 ms** and ActualOD **33.3 ms** targets. V76 is diagnostic-only, contributes zero cohort samples, and does not repair or replace V74 or any earlier failure.

This derivative packet consumes the Root-accepted V76 R1 snapshot and exactly its two declared raw captures, using Python's arbitrary-precision JSON integers. It does not invoke the extractor, scorer, browser, tests, Git, host probes or network. The 32 selected inputs comprise **30 metadata files and two raw traces**; Root's separate acceptance covers all 109 upstream members. Source/build/method/browser bindings remain the upstream 85d142407bf4d288810f0e3a9a996c043ca163e6 / V31 / V20Y / CfT 153.0.8010.36, Chromium 507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c.

## Exact original populations and joins

The original sole admissible cuts are centerline **118→719** and ActualOD **117→718**, each **602 endpoints / 601 gaps**, nearest-rank p95 rank 571. All ties, coordinates, original uncertainty intervals and population boundaries are retained. Each trace has 752 upstream-qualified endpoints and zero coalesced lineages. This analysis describes the existing selected populations; it does not issue replacement scores.

For **all 1,204 selected endpoints**, the exact qualified display trace ID finds one `STEP_BUFFER_SWAP_POST_SUBMIT` containing exactly one same-PID/TID `ImageTransportSurfaceOverlayMac::Present`, which contains exactly one `CommitPresentedFrameToCA`. There is no reuse of those source event occurrences or the exact display-ID finish occurrence. The report's exact async namespace and enclosing occurrence supply ready/latch stages: **ready is inside that Present and latch inside that commit in all 1,204 cases**. Thus this is not a nearest-time association with another presenter or an inference from intended state. The six pinned call-chain sources and their hashes/URLs are in `INPUT_BINDINGS.json`; indexed, compact joins are in `orbit-001-joins.json` and `orbit-002-joins.json`.

| Quantity | Centerline | ActualOD |
|---|---:|---:|
| Raw event count | 351,825 | 352,013 |
| Gap 0 µs / 1 µs | 19 / 36 | 32 / 32 |
| Gap 16,666 µs / 16,667 µs | 198 / 292 | 187 / 286 |
| Gap 33,332 µs / 33,333 µs | 16 / 40 | 14 / 50 |
| Long gaps immediately followed by 0–1 µs | 46 of 56 | 55 of 64 |
| Unique display IDs in original population | 602 | 602 |
| Source-frame and BeginFrame sequence steps | all 1 | all 1 |

The descriptive source windows, original action-trace time +2 s through +12 s, each contain **600 renderer BeginFrames**, interval and unthrottled interval **16,666 µs**, throttled count **0**, consecutive sequence steps **1**, and no renderer `BeginFrameDropped`. This explicitly establishes approximately 60 Hz source pacing, rather than inferring it from a mean or alleging 30 Hz throttling. These source windows are not alternative acceptance populations.

## Work and prediction are different quantities

All timings below are recorded microseconds; p95 is descriptive nearest-rank within the stated original endpoint population, not a new acceptance score.

| Recorded span, p95 / maximum (µs) | Centerline | ActualOD |
|---|---:|---:|
| Main-frame work | 3,248 / 3,932 | 3,373 / 3,856 |
| Reporter begin → compositor submit | 4,090 / 4,667 | 4,135 / 4,662 |
| Reporter begin → exact swap finish | 5,419 / 6,158 | 5,351 / 6,368 |
| GPU post-submit span | 414 / 727 | 421 / 566 |
| Present | 172 / 293 | 170 / 290 |
| Commit | 163 / 288 | 164 / 283 |
| Recorded backpressure duration | 2 / 12 (601 known) | 2 / 7 (602 known) |
| Ready → latch | 74 / 206 | 75 / 139 |
| `now_to_display` estimate lead | 34,553 / 34,830 | 34,584 / 34,831 |

There is exactly one Metal poll event per joined Present and no GL event, consistent with the recorded ANGLE Metal / GraphiteDawnMetal backend. Known Metal durations have p95 1 µs and maxima 11/7 µs; **171/162 Metal durations are omitted**, not imputed as zero. One centerline backpressure duration, source index 530 / raw event 155742, is also omitted. Its start precedes the uniquely bound commit by a recorded 1 µs inside a 109 µs Present. That enclosing source-ordered context excludes interpreting omission as a long standalone recorded wait, but is not a fabricated duration or hardware timing bound. No maximum-pending-swaps event occurs in either descriptive source window.

The long-gap following endpoints still have short work: Present maxima 293/211 µs, and reporter-begin→finish maxima 6,158/6,368 µs. Their main-frame medians are 3,103/3,256 µs, versus 2,044/2,186 µs for ordinary-gap following endpoints. Work and scheduling phase can influence a predictor deadline; these observations do **not** prove zero product influence or that optimization could never change the reported metric. They do exclude attributing these long reported intervals to an observed extra 16 ms serialized fence/commit stall.

Let `P` be the exact reporter endpoint, `C` the bound commit trace timestamp, and `L` its `now_to_display` argument. **P − (C + L) is within ±1 µs for every endpoint.** Consequently, each adjacent reported gap decomposes into commit-start spacing plus estimate-lead change, with the observed residual also within ±1 µs:

| For original long gaps | Centerline (56) | ActualOD (64) |
|---|---:|---:|
| Consecutive commit spacing range (µs) | 16,862–20,617 | 17,021–20,988 |
| Estimate-lead increase range (µs) | 12,716–16,472 | 12,345–16,312 |
| Estimate-lead increase median (µs) | 14,430 | 14,878 |

Concrete exact example: centerline source indices **181→182→183** report gaps **33,333 then 0 µs**. Their commit starts advance **18,005 then 16,244 µs** while estimate leads change **+15,327 then −16,244 µs**; the first decomposition has a +1 µs residual and the second is exact. Present durations are 114/188/129 µs, with distinct surface/display/reporter occurrences throughout. The near-tie is not a duplicated observation to remove.

Pinned `image_transport_surface_overlay_mac.mm:235–255` applies backpressure, computes `GetDisplaytime`, then records this estimate lead. Lines 310–339 choose a future display time using private current/next display state, a 1,500 µs latch buffer and period snapping. `ca_layer_tree_coordinator.mm:288–298` forwards that **estimated** `display_time` as `feedback.timestamp`; its flags do not turn it into observed physical scanout. The bound trace therefore locates the additional apparent period in the producer estimate. It does not show a hardware wait from swap finish to that future timestamp.

## Callback and profile limits

There are 600 `OnVSyncPresentation` callbacks in each descriptive source window on the selected GPU thread. Their delay p95/max is **88/727 µs** and **89/437 µs**; callback-timebase→display median is **22,988/22,987 µs**. Same-thread chronological brackets put long-gap following commits a median **5,353/5,151 µs** after the immediately preceding callback, versus **3,471/3,552 µs** for ordinary-gap following commits. These are **unbound thread-context statistics**, not callback-to-frame joins. The callbacks expose neither a presenter/frame identity nor all effective private predictor state; no exact per-frame `GetDisplaytime` branch or complete predictor replay is claimed. The source and bound estimate transitions support deadline/phase behavior, while the private branch remains unknown.

AC and page foreground guards passed. CfT PID 79142 was OS-frontmost at the recorded before/final boundaries; the page's initial/final records were focused and visible with no intervening recorded focus/visibility event. This is not continuous OS-window foreground or occlusion proof. The CDP window geometrically overlaps external display 5 (60 Hz mode) fully, and display 1 (120 Hz) not at all, at both declared boundaries. The physical and CDP observations are not simultaneous; there is no direct historical CGWindow/PID/provider binding or continuous display/VRR/scanout measurement. This makes external-display pacing plausible and prevents treating the internal 120 Hz mode as the measured source, but does not independently identify the provider. Adding `gpu` can perturb work/phase; no cost is subtracted and no observer-free inference is made.

## One next decision

**Retain the full-cohort hold and explicitly carry orbit performance as NOT ACCEPTED under the current reference profile and Chromium-reported timestamp contract. Stop automatic diagnostic retries and do not dispatch a product/scorer patch from this evidence.** The bounded diagnostic has answered its work-versus-estimate question. Root should route the remaining acceptance/profile meaning to the owner as a decision, rather than assume that another run or a faster display repairs the failed targets.

Any later owner-authorized change to the metric or reference profile must have an engineering rationale independent of the desired score—for example which deployment display/profile and which presentation concept the requirement actually intends—and must be frozen prospectively, applied fairly to baseline and candidate, and use fresh qualification. It cannot reuse an incomparable historical baseline, relabel 60 Hz inherently unsuitable, select 120 Hz merely for a pass, deduplicate timestamp ties, round 33.333 ms into the 33.3 ms target, or rescue these failed results. This packet proposes no such amendment and authorizes no run. The existing same-trace +1 µs allowance matches every saved gap upper bound; removing it would not resolve the raw 33.333 ms p95 observations.

Diagnosis closure only: source/runtime unchanged, no cohort authorization, no product/performance/tranche closure. Remaining blockers are the unchanged failed orbit acceptance, unobserved physical scanout/private predictor attribution, and an explicit owner disposition if a different acceptance concept is intended. Exact bindings are entry/exit checked; authoritative upstream records remain intact.
