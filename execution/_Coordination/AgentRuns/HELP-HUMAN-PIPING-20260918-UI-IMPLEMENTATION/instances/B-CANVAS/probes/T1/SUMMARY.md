# T1 evidence summary: the orbit probe's noise floor on this host

Written by T1-ORBIT-PROBE-2 (TASK, Type 2; Claude Fable 5.1) for B-CANVAS on 2026-09-19, finishing the task of T1-ORBIT-PROBE from its pause state. Placeholders are those of the sealed briefs `{LANE}/briefs/T1-ORBIT-PROBE.md` and `{LANE}/briefs/T1-ORBIT-PROBE-2.md`. **This guides the lane and decides nothing about D-72**; the qualification runs are ROOT's, by a separate runner and a separate instrument. No performance, usability or conformance acceptance is claimed here.

## What was measured, and on what

- **Build:** a copy made by the lane manager outside the repository at the lane head `b415b43df` (after the lane merged `origin/main` with the shell lane's slice B2). Its `index.html` has SHA-256 `e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb`, checked before the first run and recorded by the tool in every file (12 asset files, identical identity in all eight files). No build was run and `{DESKTOP}/dist` was never served or read.
- **Tool:** `{LANE}/tools/orbit_probe.mjs` with one patch made before any evidence run (the reserved-port list gained 5183 and 5186). The addition by message of 2026-09-19T04:08Z (a variant may name its own build, `<name>[@<distDir>]=<query>`) is in the tool and is exercised by the self-test, which serves two copies of one build; the A/A series stay as the sealed brief says, one build under two names.
- **Host:** shared with other lanes; every browser run went through `{RUN}/tools/with_e2e_lock.sh` on port 5185, one series per hold. Headless Chrome 153.0.8010.48 by Playwright 1.60.0, WebGL renderer "ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version)".
- **Files:** `self-test.txt` (PASS, exit 0, after the patch); six A/A series (`--variants "a=;b=" --pairs 3`, light, labels on, device pixel ratio 2, window 1440 x 900, warm-up 2,000 ms, measurement 10,000 ms): `aa_10000_schematic_vsync.json`, `aa_10000_schematic_uncapped.json`, `aa_10000_actual-od_vsync.json`, `aa_10000_actual-od_uncapped.json`, `aa_1000_schematic_vsync.json`, `aa_1000_actual-od_vsync.json`; `resources_1000.json` and `resources_10000.json`. All 36 runs ended without error, with no console error, warning or page error. The first series was run at 06:19Z and the others between 08:37Z and 09:02Z, because the session that ran them was interrupted between; no series was repeated.

## The product moved under the tool? No

Every control, test id and surface the tool relies on was found in the measured build before the first run (`viewport-canvas`, `viewport-geometry-schematic`, `viewport-geometry-actual-od`, `viewport-view-isometric`, `viewport-fit-model`, `toggle-viewport-labels`, `toggle-viewport-grid`, `desktop-preview-shell`, the "Appearance theme" select, `__openPipeStressUiDiagnosticsV1`, and the start-up model module `invented_preview_model-*.js` that the fixture route answers). The self-test passed without any change to the tool's page handling.

## Window and canvas, re-measured on this build

Measured in one lock hold with the tool's own exported pieces (1,000-pipe fixture, device pixel ratio 2; canvas bounding box and the diagnostics surface agree in every case):

| Window | Canvas (CSS px) | Canvas area (px²) |
|---|---|---|
| 1440 x 900 (the evidence series) | 794 x 560 | 444,640 |
| 1600 x 1100 | 954 x 760 | 725,040 |
| 1645 x 1167 | 999 x 827 | 826,173 |
| **1646 x 1168** | **1000 x 828** | **828,000** |
| 1800 x 1200 | 1154 x 860 | 992,440 |

The layout around the canvas takes a constant 646 x 340 CSS px at these sizes, as it did before the merge. **`--window 1646x1168` still gives a canvas of exactly 1000 x 828, that is 828,000 px²; one pixel less each way (1645 x 1167) falls short.** It is the lane's conservative stand-in for the redesigned Model view's canvas area; it is not the redesigned layout. Every evidence series here ran at 1440 x 900, that is on a canvas of 794 x 560, a little over half that area.

## Noise floor per measure (b minus a, one build under two names, three pairs per series)

Load average (1 minute) beside each series, lowest to highest over its runs.

| Series | Load (1 min) | Mean interval | p50 | p95 | Missed refreshes / intervals over 33.3 ms | Main thread per frame |
|---|---|---|---|---|---|---|
| 10,000 schematic vsync | 4.2 to 5.3 | 0.000 ms | 0.00 | 0.00 to +0.10 ms | none in 6 runs | -0.233 to +0.012 ms (of about 2.5) |
| 10,000 actual OD vsync | 3.8 to 4.7 | 0.000 ms | 0.00 | 0.00 to +0.10 ms | none in 6 runs | -0.072 to +0.022 ms (of about 3.1) |
| 1,000 schematic vsync | 3.5 to 5.6 | 0.000 ms | 0.00 | 0.00 | none in 6 runs | -0.087 to -0.021 ms (of about 2.9) |
| 1,000 actual OD vsync | 3.1 to 4.3 | 0.000 ms | 0.00 | -0.10 ms | none in 6 runs | -1.314 to -0.123 ms (of about 3.0); see "reads oddly" |
| 10,000 schematic uncapped | 2.2 to 5.0 | -0.005 to +0.041 ms (-0.3 % to +2.1 % of 1.99) | -0.10 to 0.00 | 0.00 (3.4 ms in all 6 runs) | none over 33.3 ms | -0.009 to +0.036 ms (of about 1.93) |
| 10,000 actual OD uncapped | 4.1 to 6.4 | -0.025 to +0.021 ms (-1.3 % to +1.1 % of 2.00) | 0.00 | 0.00 (3.4 ms in all 6 runs) | none over 33.3 ms | -0.023 to +0.023 ms (of about 1.93) |

What this lets the tool detect, and not:

- **Under `vsync` the interval measures are pinned to the refresh period** (mean 16.666 ms in all 24 runs; p50 16.7; p95 16.7 or 16.8, which is one 0.1 ms timestamp step). They can show only a missed refresh, and there was none, so they say nothing about any cost that fits inside a refresh. The "over 16.7 ms" fraction (27 % to 33 %) is rounding of 16.67 and carries no information. The sensitive `vsync` measure is main-thread milliseconds per frame, with a floor of about 0.25 ms per frame (roughly 10 %) between single pairs; a slice that adds less than that per frame on the main thread is not seen in three pairs.
- **Under `uncapped` the mean interval and the main-thread cost per frame are the sensitive measures**, both with a floor of about 0.04 ms (about 2 %) between single pairs at 10,000 pipes, under load averages from 2 to 6. A difference of 0.1 ms per frame (5 %) or more that holds in all three pairs would stand out of this floor; one under about 0.05 ms would not. The p95 sat at 3.4 ms in all twelve uncapped runs and the p50 moves in 0.1 ms steps, so neither resolves a small change; the maximum ranged from 4.8 to 15.4 ms between identical runs and is not usable for comparison.
- **The uncapped orbit is limited by the main thread** (busy 96.6 % to 96.8 % of the window in all twelve runs, one product render per frame: for instance 6,083 submissions in 6,084 frames). GPU cost that stays under that limit is not visible to the tool. Schematic and actual OD gave the same interval (1.99 and 2.00 ms), the same 359 draw calls and the same 2,133,368 triangles at 10,000 pipes (54 and 213,488 at 1,000), with the geometry mode confirmed in the diagnostics as `actual-od`, OD status `available`.

## Did uncapped pacing take effect headless?

Yes, in all twelve uncapped runs: the blank-page calibration's interval p50 was 0.1 ms (against 16.6 to 16.7 ms in all 24 `vsync` runs), and the orbit's mean interval was 1.97 to 2.03 ms with about 5,000 intervals in the 10 s window against 599 under `vsync`.

## Resources and settling

At 1,000 and at 10,000 pipes, over 17 recorded steps each (theme, geometry mode, labels and grid, each switched away and back twice): `live` identical before and after (200 and 1,115 owned resources), identical after each of the 4 round trips, and `created - disposed = live` at every step. Settle check 1.5 s after the last input: no animation frame pending, none requested or fired, nothing drawn or cleared in the following second. The same observation made while a real-pointer orbit was under way reported "not settled" both times (60 frames fired in the second; 3,480 and 21,780 draw calls), which is the control the brief asked for.

## What reads oddly

1. In `aa_1000_actual-od_vsync.json` the last run (`b`, pair 3) shows 1.395 ms of main thread per frame against 2.6 to 3.0 in its five siblings. In that run the tool's own pointer timer was held back (34 send gaps over 8 ms, the largest 51 ms, against 0 to 5 in the other runs; the load average had just risen from 3.1 to 4.3), so fewer pointer moves reached the page (camera sequence 3,004 against about 3,250) and the page did less work. So main-thread cost per frame depends on the pointer delivery rate, and a run whose `pointer.sendGapsOver8Ms` is far above its siblings' should be read with suspicion; the tool records this but does not flag it.
2. Under `vsync` on this host nothing at either model size comes near a refresh: the main thread is busy 15 % to 18 % of the time. The `vsync` series are therefore a check that a slice does not start missing refreshes, not a way to size its cost; the `uncapped` series are the way to size it, and only for the main-thread part.
3. The fixtures load fast (model ready in about 0.23 s at 1,000 pipes and 0.7 s at 10,000), and nothing hitched: no interval over 33.3 ms in any of the 36 runs. One `resources` hold took ten minutes of wall time, all but 19 seconds of it waiting for another lane's lock hold.

Nothing was needed from the product that it lacks.

---

What follows is the unedited output of `node orbit_probe.mjs summarize` over the eight JSON files in this folder.

# Orbit probe summary

Guidance for the B-CANVAS lane only. This output decides nothing about D-72; the qualification runs are ROOT's, by a separate runner and a separate instrument.

Medians and ranges only. Percentiles are nearest-rank; with three runs the p95 of the per-run p95 values is the largest of them. Intervals are between requestAnimationFrame timestamps in the measurement window.

## Series

| File | Pipes | Mode | Labels | Theme | DPR | Window | Canvas (CSS px) | Pacing | Pacing calibration p50 (ms) | Uncapped took effect | Warm-up / measure (ms) | Build index.html SHA-256 | Browser | WebGL renderer |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| aa_10000_schematic_vsync.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 | not requested | 2000 / 10000 | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| aa_10000_schematic_uncapped.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| aa_10000_actual-od_vsync.json | 10000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 | not requested | 2000 / 10000 | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| aa_10000_actual-od_uncapped.json | 10000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| aa_1000_schematic_vsync.json | 1000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 | not requested | 2000 / 10000 | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| aa_1000_actual-od_vsync.json | 1000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 [16.60 to 16.70] | not requested | 2000 / 10000 | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |

## Builds

A build identity is the SHA-256 of index.html with the names and sizes of the files under assets.

| File | Variant | Query | Build directory | index.html SHA-256 | Asset files | Asset bytes | Against the first variant |
|---|---|---|---|---|---|---|---|
| aa_10000_schematic_vsync.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| aa_10000_schematic_vsync.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | same build identity |
| aa_10000_schematic_uncapped.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| aa_10000_schematic_uncapped.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | same build identity |
| aa_10000_actual-od_vsync.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| aa_10000_actual-od_vsync.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | same build identity |
| aa_10000_actual-od_uncapped.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| aa_10000_actual-od_uncapped.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | same build identity |
| aa_1000_schematic_vsync.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| aa_1000_schematic_vsync.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | same build identity |
| aa_1000_actual-od_vsync.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| aa_1000_actual-od_vsync.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | same build identity |

- aa_10000_schematic_vsync.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.
- aa_10000_schematic_uncapped.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.
- aa_10000_actual-od_vsync.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.
- aa_10000_actual-od_uncapped.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.
- aa_1000_schematic_vsync.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.
- aa_1000_actual-od_vsync.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.

## Variants

| File | Variant | Runs | Per-run p95 (ms): p50 | Per-run p95 (ms): p95 | Per-run p95 (ms): range | Per-run p50 (ms) | Mean interval (ms) | Over 16.7 ms | Over 33.3 ms | Main thread (ms/frame) | Draw calls | Triangles | Mean load (1 min) | Console errors |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| aa_10000_schematic_vsync.json | a | 3 | 16.70 | 16.70 | 16.70 to 16.70 | 16.70 | 16.666 | 30.2% [30.2% to 32.2%] | 0.0% | 2.555 [2.534 to 2.660] | 359 | 2133368 | 4.79 | 0 |
| aa_10000_schematic_vsync.json | b | 3 | 16.80 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 30.9% [29.5% to 31.4%] | 0.0% | 2.489 [2.427 to 2.567] | 359 | 2133368 | 4.82 | 0 |
| aa_10000_schematic_uncapped.json | a | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.20 [2.20 to 2.30] | 1.992 [1.985 to 2.009] | 0.0% | 0.0% | 1.929 [1.920 to 1.944] | 359 | 2133368 | 3.40 | 0 |
| aa_10000_schematic_uncapped.json | b | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.20 | 2.004 [1.994 to 2.026] | 0.0% | 0.0% | 1.935 [1.928 to 1.956] | 359 | 2133368 | 3.87 | 0 |
| aa_10000_actual-od_vsync.json | a | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 29.4% [28.7% to 29.4%] | 0.0% | 3.068 [3.065 to 3.071] | 359 | 2133368 | 4.09 | 0 |
| aa_10000_actual-od_vsync.json | b | 3 | 16.80 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 31.2% [30.7% to 32.9%] | 0.0% | 3.046 [2.999 to 3.087] | 359 | 2133368 | 4.04 | 0 |
| aa_10000_actual-od_uncapped.json | a | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.10 | 1.999 [1.987 to 2.002] | 0.0% | 0.0% | 1.931 [1.919 to 1.935] | 359 | 2133368 | 5.25 | 0 |
| aa_10000_actual-od_uncapped.json | b | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.10 | 1.990 [1.974 to 2.008] | 0.0% | 0.0% | 1.924 [1.908 to 1.942] | 359 | 2133368 | 5.63 | 0 |
| aa_1000_schematic_vsync.json | a | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 29.0% [28.9% to 29.9%] | 0.0% | 2.900 [2.840 to 2.934] | 54 | 213488 | 4.40 | 0 |
| aa_1000_schematic_vsync.json | b | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 30.6% [27.4% to 31.4%] | 0.0% | 2.879 [2.753 to 2.908] | 54 | 213488 | 4.06 | 0 |
| aa_1000_actual-od_vsync.json | a | 3 | 16.80 | 16.80 | 16.80 to 16.80 | 16.70 | 16.666 | 29.0% [28.4% to 29.5%] | 0.0% | 2.976 [2.709 to 2.976] | 54 | 213488 | 3.58 | 0 |
| aa_1000_actual-od_vsync.json | b | 3 | 16.70 | 16.70 | 16.70 to 16.70 | 16.70 | 16.666 | 31.6% [30.5% to 32.2%] | 0.0% | 2.641 [1.395 to 2.853] | 54 | 213488 | 3.71 | 0 |

## Paired differences

Each later variant minus the first variant, pair by pair in run order: median [smallest to largest]. With one build under two names this is the noise floor.

| File | Variant minus reference | Pairs | p95 difference (ms) | As a share of the reference p95 | p50 difference (ms) | Mean-interval difference (ms) | As a share of the reference mean | Main-thread difference (ms/frame) |
|---|---|---|---|---|---|---|---|---|
| aa_10000_schematic_vsync.json | b - a | 3 | 0.10 [0.00 to 0.10] | 0.6% [0.0% to 0.6%] | 0.00 | 0.000 | 0.0% | -0.045 [-0.233 to 0.012] |
| aa_10000_schematic_uncapped.json | b - a | 3 | 0.00 | 0.0% | 0.00 [-0.10 to 0.00] | 0.002 [-0.005 to 0.041] | 0.1% [-0.3% to 2.1%] | -0.001 [-0.009 to 0.036] |
| aa_10000_actual-od_vsync.json | b - a | 3 | 0.00 [0.00 to 0.10] | 0.0% [0.0% to 0.6%] | 0.00 | 0.000 | 0.0% | -0.022 [-0.072 to 0.022] |
| aa_10000_actual-od_uncapped.json | b - a | 3 | 0.00 | 0.0% | 0.00 | -0.012 [-0.025 to 0.021] | -0.6% [-1.3% to 1.1%] | -0.011 [-0.023 to 0.023] |
| aa_1000_schematic_vsync.json | b - a | 3 | 0.00 | 0.0% | 0.00 | 0.000 | 0.0% | -0.026 [-0.087 to -0.021] |
| aa_1000_actual-od_vsync.json | b - a | 3 | -0.10 | -0.6% | 0.00 | 0.000 | 0.0% | -0.335 [-1.314 to -0.123] |

## Resources

| File | Pipes | Steps | live identical before and after | Round trips with identical live | created - disposed = live throughout | Settle check: frame pending / anything drawn / reported settled | Orbit control: frame pending / anything drawn / reported settled | Console errors |
|---|---|---|---|---|---|---|---|---|
| resources_1000.json | 1000 | 17 | yes | 4 of 4 | yes | no / no (0 draws) / yes | yes / yes (3480 draws) / no | 0 |
| resources_10000.json | 10000 | 17 | yes | 4 of 4 | yes | no / no (0 draws) / yes | yes / yes (21780 draws) / no | 0 |


Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
