# C2 measurement series and pair counts, run by the lane manager

Run by B-CANVAS (Claude Fable 5.1) on 2026-09-19 between 12:33Z and 13:50Z with `{LANE}/tools/orbit_probe.mjs` at SHA-256 `4c6769381c8983ca739ff9d560221983a75407a91c75692b32fe84415f2bacc9` (the probe as C2-PROBE extended it). Build copies outside the repository, by `index.html` SHA-256: `base` `fc785c1b…8180c` (the lane before C2: selection recolours); `halo` `e9aca49e…8db3` (the working form (b), which `npm run build:desktop` on the committed tree reproduces); `stencil` `3c53bf64…2922` (form (c), from `stencil_candidate.patch`, never in the tree). Interleaved variants, a fresh browser per run, one series per lock hold, no build or suite of the lane's beside a timed series. Light theme, device pixel ratio 2, labels on for orbits and off for hover. Window 1440 × 900 gives a canvas of 794 × 560 CSS px; 1646 × 1168 gives 1000 × 828, the lane's stand-in for D-72's canvas area and not the redesigned layout. 1-minute load averages ranged from 3.2 to 8.1 and are in the table.

**This guides the lane and decides nothing about D-72.** No performance acceptance is claimed.

## What the series say

1. **All 10,000 pipes selected, uncapped** (the case the instrument's manifest requires and the costliest for a halo): mean frame interval base 2.19 to 2.34 ms; **form (b) 2.66 to 2.99 ms (+0.45 to +0.66 ms, about +21 % to +28 %)**; **form (c) 4.31 to 4.66 ms (+2.0 to +2.4 ms, about +85 % to +105 %)**. The differences hold in every pair and are ten to fifty times T1's noise floor of about 0.04 ms. p95: base 3.1 to 3.4, (b) 3.7 to 4.0, (c) 6.3 to 7.8 ms, against limits of 16.7 ms (centreline) and 33.3 ms (real outside diameter). Draw calls 361 → 363; triangles 2.13 M → 3.13 M for both forms. The stencil form costs about four times what the depth-mask form costs on this host, with the same picture (C2-HALO found eighteen fixture pictures equal pixel for pixel).
2. **All selected, display pacing, the 1000 × 828 stand-in canvas**: no refresh missed by any variant in either geometry mode (mean 16.666 ms in all twelve runs).
3. **One pipe selected, and nothing selected**: no difference outside the noise floor (2.07 against 2.09 ms; 2.00 against 2.00 ms). With nothing selected the halo build draws exactly the base's 359 calls: an unselected scene pays nothing.
4. **Hover sweep at 10,000 pipes, no button pressed, labels off**: 4.26 ms of main thread per pointer event on the base and 4.26 ms on the halo build (medians; pairs on both sides of zero). **The hover halo adds nothing measurable. The 4.3 ms per pointer move is today's product**: it picks on every move and redraws the frame, and it did so before this slice. It is inside D-72's limits as the lane reads them (a move is not a pick gesture), and the manager names it because it is the largest per-move cost in the canvas.
5. **`resources` on the halo build** at 1,000 and 10,000 pipes: `live` identical before and after, 4 of 4 round trips, `created − disposed = live` throughout, settled after the last input, no console error or warning.

What this does not show: GPU cost that stays under the main-thread bound (T1's finding); selection sets that are spread across the model (the probe selects the tree's first pipes); everything selected including nodes and supports (the probe selects pipes; C2-HALO's count for that case is +2 draws, +2.3 M vertices and +3.4 M triangles); the act of selecting (point-pick and box timings are the instrument's).

## Pair counts with no casing (P1 ASK-4), the probe's simplified rule

Centreline mode, fitted isometric camera, canvas 794 × 560, device pixel ratio 1, the fixtures' 200 point samples, real pointer input. The rule is P1 §4.4 simplified (no predicted band, no exclusivity): a sample passes when the image changed and at least 4 pairs exist of a newly Selection-coloured pixel and a `canvas.bg` pixel within 2 CSS px at 3:1 or better.

**The product's build cannot answer the question, and the manager's first reading of it was wrong.** On the halo build as the product draws it, 200, 200, 199 and 200 of 200 pass. But the first profile's diamond cue still draws at the winner's centre, and its rim is `canvas.bg` (slice C1b), so the diamond alone satisfies the rule. The manager therefore made a **probe-only build copy** with the diamond forced invisible (one line in `updateSelectionCue`, built through the lock, the tree restored and hash-checked; `index.html` `2096e92a…13da`; never committed). A first attempt at that copy did not hide the diamond (the cue's own `update()` sets `visible`); its four runs were identical to the product's, were recognised as void and were deleted.

| File | Pass of 200 | no ground beside | too few pairs | no cue seen | pairs, median [min to max] | new pixels, median | of them over ground, median | inner, median |
|---|---|---|---|---|---|---|---|---|
| `m_pairs_10000_schematic_dark.json` | 200 | 0 | 0 | 0 | 25.0 [4 to 36] | 38.0 | 0.0 | 6.0 |
| `m_pairs_10000_schematic_light.json` | 199 | 0 | 1 | 0 | 25.0 [3 to 36] | 39.0 | 0.0 | 6.0 |
| `m_pairs_1000_schematic_dark.json` | 200 | 0 | 0 | 0 | 24.0 [19 to 53] | 35.0 | 0.0 | 6.0 |
| `m_pairs_1000_schematic_light.json` | 200 | 0 | 0 | 0 | 24.0 [19 to 52] | 35.0 | 0.0 | 6.0 |
| `m_pairs_nocue_10000_schematic_dark.json` | 1 | 143 | 56 | 0 | 0.0 [0 to 10] | 42.0 | 0.0 | 0.0 |
| `m_pairs_nocue_10000_schematic_light.json` | 1 | 199 | 0 | 0 | 0.0 [0 to 8] | 45.0 | 0.0 | 0.0 |
| `m_pairs_nocue_1000_schematic_dark.json` | 43 | 93 | 64 | 0 | 1.0 [0 to 29] | 41.5 | 0.0 | 0.0 |
| `m_pairs_nocue_1000_schematic_light.json` | 42 | 154 | 4 | 0 | 0.0 [0 to 27] | 43.0 | 0.0 | 0.0 |

Rows named `nocue` are the halo alone. **With no casing the halo alone passes 42 and 43 of 200 at 1,000 pipes (light, dark) and 1 of 200 at 10,000 pipes in both themes.** The halo was seen in every sample ("no cue seen" is 0 everywhere); what fails is ground beside it: in these fixtures at the fitted camera the 48 px region around a sample is almost all figure (a median of 51 to 64 ground pixels of 2,304 at 10,000 pipes), so the halo lies over other tubes and spheres, where `canvas.selection` is 2.21:1 (light) and 1.85:1 (dark) against `canvas.pipe`. In dark more failures are "too few pairs" than in light because more neighbours fall within ±48 of the dark ground. This is what P1 §4.5 predicted. It is guidance by a simplified rule on a camera the oracle was not computed for (only 18 or 19 of 200 clicks select the nominal expected element at this camera), and it is the lane's input to the owner's decision on the casing; the lane draws no casing, lowers no 3:1 and drops no pair check.

---

What follows is the unedited output of `node orbit_probe.mjs summarize` over the timed series and the two `resources` runs.

# Orbit probe summary

Guidance for the B-CANVAS lane only. This output decides nothing about D-72; the qualification runs are ROOT's, by a separate runner and a separate instrument.

Medians and ranges only. Percentiles are nearest-rank; with three runs the p95 of the per-run p95 values is the largest of them. Intervals are between requestAnimationFrame timestamps in the measurement window.

## Series

| File | Pipes | Mode | Labels | Theme | DPR | Window | Canvas (CSS px) | Pacing | Pacing calibration p50 (ms) | Uncapped took effect | Warm-up / measure (ms) | Build index.html SHA-256 | Browser | WebGL renderer |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| m_all_10000_actual-od_uncapped.json | 10000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| m_all_10000_actual-od_uncapped_1646x1168.json | 10000 | actual-od | on | light | 2 | 1646x1168 | 1000 x 828 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| m_all_10000_actual-od_vsync_1646x1168.json | 10000 | actual-od | on | light | 2 | 1646x1168 | 1000 x 828 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| m_all_10000_schematic_uncapped.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| m_all_10000_schematic_vsync_1646x1168.json | 10000 | schematic | on | light | 2 | 1646x1168 | 1000 x 828 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| m_one_10000_schematic_uncapped.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| m_none_10000_schematic_uncapped.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| m_hover_10000_schematic_uncapped.json | 10000 | schematic | off | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |

## Builds

A build identity is the SHA-256 of index.html with the names and sizes of the files under assets.

| File | Variant | Query | Build directory | index.html SHA-256 | Asset files | Asset bytes | Against the first variant |
|---|---|---|---|---|---|---|---|
| m_all_10000_actual-od_uncapped.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_all_10000_actual-od_uncapped.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |
| m_all_10000_actual-od_uncapped.json | stencil | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_stencil | 3c53bf6412961bdaf8fbeefc8fab643677f7218fa08d16ed72e6ff74b82f2922 | 12 | 2743584 | different build identity |
| m_all_10000_actual-od_uncapped_1646x1168.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_all_10000_actual-od_uncapped_1646x1168.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |
| m_all_10000_actual-od_uncapped_1646x1168.json | stencil | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_stencil | 3c53bf6412961bdaf8fbeefc8fab643677f7218fa08d16ed72e6ff74b82f2922 | 12 | 2743584 | different build identity |
| m_all_10000_actual-od_vsync_1646x1168.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_all_10000_actual-od_vsync_1646x1168.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |
| m_all_10000_actual-od_vsync_1646x1168.json | stencil | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_stencil | 3c53bf6412961bdaf8fbeefc8fab643677f7218fa08d16ed72e6ff74b82f2922 | 12 | 2743584 | different build identity |
| m_all_10000_schematic_uncapped.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_all_10000_schematic_uncapped.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |
| m_all_10000_schematic_uncapped.json | stencil | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_stencil | 3c53bf6412961bdaf8fbeefc8fab643677f7218fa08d16ed72e6ff74b82f2922 | 12 | 2743584 | different build identity |
| m_all_10000_schematic_vsync_1646x1168.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_all_10000_schematic_vsync_1646x1168.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |
| m_all_10000_schematic_vsync_1646x1168.json | stencil | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_stencil | 3c53bf6412961bdaf8fbeefc8fab643677f7218fa08d16ed72e6ff74b82f2922 | 12 | 2743584 | different build identity |
| m_one_10000_schematic_uncapped.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_one_10000_schematic_uncapped.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |
| m_none_10000_schematic_uncapped.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_none_10000_schematic_uncapped.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |
| m_hover_10000_schematic_uncapped.json | base | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| m_hover_10000_schematic_uncapped.json | halo | (empty) | {OUTSIDE_REPOSITORY}/dist_c2_working | e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3 | 12 | 2743401 | different build identity |

- m_all_10000_actual-od_uncapped.json: the variants do not share one build identity; they compare different builds.
- m_all_10000_actual-od_uncapped_1646x1168.json: the variants do not share one build identity; they compare different builds.
- m_all_10000_actual-od_vsync_1646x1168.json: the variants do not share one build identity; they compare different builds.
- m_all_10000_schematic_uncapped.json: the variants do not share one build identity; they compare different builds.
- m_all_10000_schematic_vsync_1646x1168.json: the variants do not share one build identity; they compare different builds.
- m_one_10000_schematic_uncapped.json: the variants do not share one build identity; they compare different builds.
- m_none_10000_schematic_uncapped.json: the variants do not share one build identity; they compare different builds.
- m_hover_10000_schematic_uncapped.json: the variants do not share one build identity; they compare different builds.

## Variants

| File | Variant | Runs | Per-run p95 (ms): p50 | Per-run p95 (ms): p95 | Per-run p95 (ms): range | Per-run p50 (ms) | Mean interval (ms) | Over 16.7 ms | Over 33.3 ms | Main thread (ms/frame) | Draw calls | Triangles | Mean load (1 min) | Console errors |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| m_all_10000_actual-od_uncapped.json | base | 3 | 3.10 | 3.20 | 2.90 to 3.20 | 2.10 | 2.194 [2.185 to 2.221] | 0.0% | 0.0% | 2.011 [2.005 to 2.036] | 361 | 2133368 | 5.43 | 0 |
| m_all_10000_actual-od_uncapped.json | halo | 3 | 3.70 | 3.70 | 3.70 to 3.70 | 2.50 [2.40 to 2.60] | 2.655 [2.578 to 2.662] | 0.0% | 0.0% | 2.441 [2.363 to 2.448] | 363 | 3133368 | 5.68 | 0 |
| m_all_10000_actual-od_uncapped.json | stencil | 3 | 7.60 | 7.70 | 7.40 to 7.70 | 4.30 [4.20 to 4.30] | 4.483 [4.439 to 4.540] | 0.0% | 0.0% | 4.346 [4.320 to 4.412] | 363 | 3133368 | 5.57 | 0 |
| m_all_10000_actual-od_uncapped_1646x1168.json | base | 3 | 3.40 | 3.60 | 3.40 to 3.60 | 2.20 [2.20 to 2.30] | 2.336 [2.303 to 2.398] | 0.0% | 0.0% | 2.112 [2.076 to 2.149] | 361 | 2133368 | 6.93 | 0 |
| m_all_10000_actual-od_uncapped_1646x1168.json | halo | 3 | 4.00 | 4.30 | 4.00 to 4.30 | 3.00 | 2.994 [2.944 to 3.059] | 0.0% | 0.0% | 2.735 [2.696 to 2.798] | 363 | 3133368 | 7.11 | 0 |
| m_all_10000_actual-od_uncapped_1646x1168.json | stencil | 3 | 6.30 | 6.40 | 5.80 to 6.40 | 4.10 [3.90 to 4.30] | 4.310 [4.041 to 4.395] | 0.0% | 0.0% | 4.112 [3.822 to 4.204] | 363 | 3133368 | 7.17 | 0 |
| m_all_10000_actual-od_vsync_1646x1168.json | base | 2 | 16.80 | 16.80 | 16.80 to 16.80 | 16.70 | 16.666 | 34.9% [34.7% to 35.2%] | 0.0% | 2.707 [2.699 to 2.715] | 361 | 2133368 | 5.54 | 0 |
| m_all_10000_actual-od_vsync_1646x1168.json | halo | 2 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 36.2% [36.0% to 36.4%] | 0.0% | 2.836 [2.822 to 2.850] | 363 | 3133368 | 5.27 | 0 |
| m_all_10000_actual-od_vsync_1646x1168.json | stencil | 2 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 35.4% [35.1% to 35.7%] | 0.0% | 2.732 [2.717 to 2.747] | 363 | 3133368 | 5.25 | 0 |
| m_all_10000_schematic_uncapped.json | base | 3 | 3.30 | 3.30 | 3.20 to 3.30 | 2.20 | 2.268 [2.252 to 2.295] | 0.0% | 0.0% | 2.071 [2.056 to 2.099] | 361 | 2133368 | 4.94 | 0 |
| m_all_10000_schematic_uncapped.json | halo | 3 | 4.00 | 4.00 | 3.90 to 4.00 | 2.70 [2.50 to 2.70] | 2.751 [2.690 to 2.784] | 0.0% [0.0% to 0.1%] | 0.0% | 2.545 [2.544 to 2.580] | 363 | 3133368 | 5.04 | 0 |
| m_all_10000_schematic_uncapped.json | stencil | 3 | 7.80 | 8.20 | 7.70 to 8.20 | 4.40 | 4.656 [4.655 to 4.662] | 0.0% | 0.0% | 4.536 [4.529 to 4.567] | 363 | 3133368 | 5.09 | 0 |
| m_all_10000_schematic_vsync_1646x1168.json | base | 2 | 16.80 | 16.80 | 16.80 to 16.80 | 16.70 | 16.666 | 33.4% [33.1% to 33.6%] | 0.0% | 2.758 [2.699 to 2.818] | 361 | 2133368 | 4.31 | 0 |
| m_all_10000_schematic_vsync_1646x1168.json | halo | 2 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 36.4% [35.6% to 37.2%] | 0.0% | 2.750 [2.699 to 2.802] | 363 | 3133368 | 4.08 | 0 |
| m_all_10000_schematic_vsync_1646x1168.json | stencil | 2 | 16.80 | 16.80 | 16.80 to 16.80 | 16.70 | 16.666 | 34.4% | 0.0% | 2.849 [2.838 to 2.861] | 363 | 3133368 | 3.86 | 0 |
| m_one_10000_schematic_uncapped.json | base | 3 | 3.20 | 3.30 | 3.10 to 3.30 | 2.00 [1.90 to 2.00] | 2.074 [1.941 to 2.112] | 0.0% | 0.0% | 1.944 [1.844 to 2.015] | 361 | 2133368 | 4.18 | 0 |
| m_one_10000_schematic_uncapped.json | halo | 3 | 3.30 | 3.40 | 3.20 to 3.40 | 2.00 [1.90 to 2.00] | 2.088 [1.958 to 2.108] | 0.0% | 0.0% | 1.967 [1.860 to 1.989] | 363 | 2133468 | 4.45 | 0 |
| m_none_10000_schematic_uncapped.json | base | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.20 [2.10 to 2.20] | 2.002 [1.992 to 2.033] | 0.0% | 0.0% | 1.936 [1.927 to 1.959] | 359 | 2133368 | 4.83 | 0 |
| m_none_10000_schematic_uncapped.json | halo | 3 | 3.40 | 3.50 | 3.40 to 3.50 | 2.20 | 1.998 [1.980 to 2.046] | 0.0% | 0.0% | 1.933 [1.915 to 1.977] | 359 | 2133368 | 5.26 | 0 |
| m_hover_10000_schematic_uncapped.json | base | 3 | 5.70 | 5.80 | 5.60 to 5.80 | 4.20 [4.10 to 4.20] | 4.403 [4.360 to 4.408] | 0.6% [0.2% to 0.6%] | 0.0% | 4.312 [4.299 to 4.314] | 359 | 2133368 | 6.08 | 0 |
| m_hover_10000_schematic_uncapped.json | halo | 3 | 5.90 | 5.90 | 5.80 to 5.90 | 4.10 [4.10 to 4.20] | 4.418 [4.388 to 4.420] | 0.3% [0.3% to 0.9%] | 0.0% | 4.324 [4.294 to 4.354] | 361 | 2133468 | 5.56 | 0 |

## Paired differences

Each later variant minus the first variant, pair by pair in run order: median [smallest to largest]. With one build under two names this is the noise floor.

| File | Variant minus reference | Pairs | p95 difference (ms) | As a share of the reference p95 | p50 difference (ms) | Mean-interval difference (ms) | As a share of the reference mean | Main-thread difference (ms/frame) |
|---|---|---|---|---|---|---|---|---|
| m_all_10000_actual-od_uncapped.json | halo - base | 3 | 0.60 [0.50 to 0.80] | 19.4% [16.1% to 25.8%] | 0.40 [0.30 to 0.50] | 0.441 [0.393 to 0.461] | 20.1% [17.9% to 21.0%] | 0.405 [0.358 to 0.437] |
| m_all_10000_actual-od_uncapped.json | stencil - base | 3 | 4.60 [4.20 to 4.70] | 148.4% [135.5% to 151.6%] | 2.20 [2.10 to 2.20] | 2.289 [2.254 to 2.319] | 104.3% [102.7% to 105.7%] | 2.335 [2.315 to 2.376] |
| m_all_10000_actual-od_uncapped_1646x1168.json | halo - base | 3 | 0.60 [0.60 to 0.70] | 17.6% [17.6% to 20.6%] | 0.80 [0.70 to 0.80] | 0.691 [0.546 to 0.723] | 29.6% [23.4% to 31.0%] | 0.659 [0.547 to 0.686] |
| m_all_10000_actual-od_uncapped_1646x1168.json | stencil - base | 3 | 2.80 [2.40 to 2.90] | 82.4% [70.6% to 85.3%] | 1.90 [1.60 to 2.10] | 1.974 [1.643 to 2.092] | 84.5% [70.3% to 89.6%] | 2.000 [1.673 to 2.128] |
| m_all_10000_actual-od_vsync_1646x1168.json | halo - base | 2 | -0.05 [-0.10 to 0.00] | -0.3% [-0.6% to 0.0%] | 0.00 | 0.000 | 0.0% | 0.129 [0.123 to 0.135] |
| m_all_10000_actual-od_vsync_1646x1168.json | stencil - base | 2 | -0.05 [-0.10 to 0.00] | -0.3% [-0.6% to 0.0%] | 0.00 | 0.000 | 0.0% | 0.025 [0.018 to 0.032] |
| m_all_10000_schematic_uncapped.json | halo - base | 3 | 0.70 [0.60 to 0.80] | 21.2% [18.2% to 24.2%] | 0.50 [0.30 to 0.50] | 0.456 [0.422 to 0.532] | 20.1% [18.6% to 23.5%] | 0.473 [0.446 to 0.524] |
| m_all_10000_schematic_uncapped.json | stencil - base | 3 | 4.50 [4.40 to 5.00] | 136.4% [133.3% to 151.5%] | 2.20 | 2.394 [2.361 to 2.403] | 105.6% [104.1% to 106.0%] | 2.465 [2.430 to 2.511] |
| m_all_10000_schematic_vsync_1646x1168.json | halo - base | 2 | -0.05 [-0.10 to 0.00] | -0.3% [-0.6% to 0.0%] | 0.00 | 0.000 | 0.0% | -0.008 [-0.016 to 0.000] |
| m_all_10000_schematic_vsync_1646x1168.json | stencil - base | 2 | 0.00 | 0.0% | 0.00 | 0.000 | 0.0% | 0.091 [0.020 to 0.162] |
| m_one_10000_schematic_uncapped.json | halo - base | 3 | 0.10 [0.00 to 0.20] | 3.1% [0.0% to 6.2%] | 0.00 [-0.10 to 0.10] | -0.004 [-0.116 to 0.147] | -0.2% [-5.6% to 7.1%] | -0.026 [-0.084 to 0.123] |
| m_none_10000_schematic_uncapped.json | halo - base | 3 | 0.00 [0.00 to 0.10] | 0.0% [0.0% to 2.9%] | 0.00 [0.00 to 0.10] | 0.006 [-0.053 to 0.044] | 0.3% [-2.6% to 2.2%] | 0.006 [-0.044 to 0.041] |
| m_hover_10000_schematic_uncapped.json | halo - base | 3 | 0.20 [0.10 to 0.20] | 3.5% [1.8% to 3.5%] | -0.10 [-0.10 to 0.10] | 0.017 [-0.020 to 0.058] | 0.4% [-0.5% to 1.3%] | 0.010 [-0.005 to 0.042] |

## Selection and hover

Kind run: an orbit with the left button down. Kind hover: the pointer sweeps the canvas with no button pressed, and the frame and main-thread columns above describe that sweep. Selected is the count the product's diagnostics surface reported before the measurement. A pointer event is a pointermove the page dispatched in the window; the browser may merge moves that arrive within one frame. With select none the count reported is the product's start-up selection (the project).

| File | Kind | Variant | Select | Selected (reported) | Selection took (ms) | Pointer events in window | Share received by the canvas | Main thread (ms per pointer event) | Script (ms per pointer event) | Product submissions in span | Camera sequence change |
|---|---|---|---|---|---|---|---|---|---|---|---|
| m_all_10000_actual-od_uncapped.json | run | base | all | 10000 | 1080 [1074 to 1082] | n/a | n/a | n/a | n/a | 5460 [5401 to 5580] | 8232 [8217 to 8384] |
| m_all_10000_actual-od_uncapped.json | run | halo | all | 10000 | 1105 [1102 to 1105] | n/a | n/a | n/a | n/a | 4540 [4520 to 4698] | 7295 [7290 to 7470] |
| m_all_10000_actual-od_uncapped.json | run | stencil | all | 10000 | 1099 [1076 to 1104] | n/a | n/a | n/a | n/a | 2706 [2637 to 2717] | 5050 [4953 to 5052] |
| m_all_10000_actual-od_uncapped_1646x1168.json | run | base | all | 10000 | 1086 [1077 to 1091] | n/a | n/a | n/a | n/a | 5387 [5033 to 5409] | 8182 [7801 to 8217] |
| m_all_10000_actual-od_uncapped_1646x1168.json | run | halo | all | 10000 | 1100 [1078 to 1100] | n/a | n/a | n/a | n/a | 4020 [3976 to 4133] | 6731 [6718 to 6846] |
| m_all_10000_actual-od_uncapped_1646x1168.json | run | stencil | all | 10000 | 1079 [1071 to 1101] | n/a | n/a | n/a | n/a | 3054 [2769 to 3184] | 5578 [5191 to 5783] |
| m_all_10000_actual-od_vsync_1646x1168.json | run | base | all | 10000 | 1228 [1225 to 1231] | n/a | n/a | n/a | n/a | 730 [729 to 730] | 3064 [3038 to 3090] |
| m_all_10000_actual-od_vsync_1646x1168.json | run | halo | all | 10000 | 1231 [1226 to 1235] | n/a | n/a | n/a | n/a | 730 | 3068 [3038 to 3097] |
| m_all_10000_actual-od_vsync_1646x1168.json | run | stencil | all | 10000 | 1232 [1228 to 1236] | n/a | n/a | n/a | n/a | 730 [729 to 730] | 3026 [3003 to 3049] |
| m_all_10000_schematic_uncapped.json | run | base | all | 10000 | 1090 [1069 to 1104] | n/a | n/a | n/a | n/a | 5318 [5256 to 5408] | 8120 [8049 to 8210] |
| m_all_10000_schematic_uncapped.json | run | halo | all | 10000 | 1098 [1080 to 1106] | n/a | n/a | n/a | n/a | 4372 [4326 to 4553] | 7100 [6780 to 7130] |
| m_all_10000_schematic_uncapped.json | run | stencil | all | 10000 | 1101 [1090 to 1105] | n/a | n/a | n/a | n/a | 2607 [2600 to 2615] | 4867 [4418 to 4892] |
| m_all_10000_schematic_vsync_1646x1168.json | run | base | all | 10000 | 1224 [1222 to 1225] | n/a | n/a | n/a | n/a | 730 [729 to 730] | 3034 [3028 to 3040] |
| m_all_10000_schematic_vsync_1646x1168.json | run | halo | all | 10000 | 1244 [1242 to 1245] | n/a | n/a | n/a | n/a | 730 [729 to 730] | 3099 [3091 to 3107] |
| m_all_10000_schematic_vsync_1646x1168.json | run | stencil | all | 10000 | 1243 [1242 to 1244] | n/a | n/a | n/a | n/a | 730 [729 to 730] | 3066 [3062 to 3070] |
| m_one_10000_schematic_uncapped.json | run | base | one | 1 | 753 [751 to 755] | n/a | n/a | n/a | n/a | 5793 [5712 to 6254] | 8620 [8234 to 9113] |
| m_one_10000_schematic_uncapped.json | run | halo | one | 1 | 756 [754 to 759] | n/a | n/a | n/a | n/a | 5857 [5820 to 6192] | 8693 [8655 to 9049] |
| m_none_10000_schematic_uncapped.json | run | base | none | 1 | n/a | n/a | n/a | n/a | n/a | 6092 [6007 to 6124] | 8942 [8855 to 8991] |
| m_none_10000_schematic_uncapped.json | run | halo | none | 1 | n/a | n/a | n/a | n/a | n/a | 6106 [5946 to 6130] | 8959 [8746 to 8993] |
| m_hover_10000_schematic_uncapped.json | hover | base | none | 1 | n/a | 2314 [2303 to 2317] | 100.0% | 4.250 [4.232 to 4.256] | 4.070 [4.047 to 4.070] | 2730 [2728 to 2758] | 0 |
| m_hover_10000_schematic_uncapped.json | hover | halo | none | 1 | n/a | 2311 [2289 to 2314] | 100.0% | 4.264 [4.200 to 4.303] | 4.086 [4.028 to 4.122] | 2727 [2721 to 2744] | 0 |

## Resources

| File | Pipes | Steps | live identical before and after | Round trips with identical live | created - disposed = live throughout | Settle check: frame pending / anything drawn / reported settled | Orbit control: frame pending / anything drawn / reported settled | Console errors |
|---|---|---|---|---|---|---|---|---|
| m_resources_1000.json | 1000 | 17 | yes | 4 of 4 | yes | no / no (0 draws) / yes | yes / yes (3480 draws) / no | 0 |
| m_resources_10000.json | 10000 | 17 | yes | 4 of 4 | yes | no / no (0 draws) / yes | yes / yes (21780 draws) / no | 0 |


Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
