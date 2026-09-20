# C2-PROBE evidence: `run --select`, `hover` and `pairs` on the build before slice C2

Written by C2-PROBE (TASK, Type 2; Claude Fable 5.1) for the canvas lane manager B-CANVAS, 2026-09-19. Placeholders as in the sealed briefs: `{LANE}` is this lane's folder, `{DESKTOP}` the desktop application. The tool and its README are `{LANE}/tools/orbit_probe.mjs` and `README.md` beside it.

**Guidance only.** Nothing here decides anything about D-72; the qualification runs are ROOT's, by a separate instrument. No performance, usability or conformance acceptance is claimed. Every series below ran on the **build the manager supplied, the lane's product before slice C2** (`index.html` SHA-256 `fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c`), in which selection is a **recolour** of the body in `canvas.selection` beside the first profile's diamond cue. **There is no halo in anything measured here.**

## Files

| File | What it is |
|---|---|
| `self-test.txt` | the self-test of the final tool bytes, through the lock, exit 0, PASS |
| `run_select-all_10000_schematic_uncapped.json` | `run --select all`, 10,000 pipes, centreline mode, labels on, light, pixel ratio 2, 1440 x 900, uncapped, A/A one pair, warm-up 2,000 ms, window 6,000 ms |
| `run_select-one_10000_schematic_uncapped.json` | the same with `--select one` |
| `hover_10000_schematic_uncapped.json` | `hover`, the same settings with labels **off** and nothing selected by the tool, A/A one pair |
| `pairs_1000_schematic_light_recolour.json` | `pairs`, 1,000 pipes, light, centreline mode, pixel ratio 1, all 200 samples; **describes the recolour, not a halo** |

The settings of the two `run` series are those of the A/A series in `{LANE}/probes/T1/` (labels on, pixel ratio 2) with a shorter window, so that they can be read beside them; note that T1 measured another build (`e79fafbe...`), so a difference from T1 is not the cost of selection. Each series was one hold of the lock on port 5185; after each the port was free and the lock released. All six timed runs and the pairs run ended without error and with no console error, warning or page error. Uncapped pacing took effect in all six (calibration p50 0.10 ms).

## What the numbers say

1. **`--select` works and the count is checked.** The diagnostics surface reported 10,000 pipes selected (primary `pipe:UIF-10000`, the Shift-clicked row) and 1 pipe (`pipe:UIF-00001`); the selection was still reported at the end of the window and after the orbit; the orbit started from the first press candidate (357, 330) in all four runs and moved the camera (3,629 to 5,572 camera steps). Selecting took about 1.1 s for all 10,000 (one wheel event and one Shift-click) and 0.8 s for one; that is the tool's time, not a product measure.
2. **What the recolour build costs selected, as far as one pair shows:** mean interval 2.47 and 2.23 ms with every pipe selected, 2.39 and 3.43 ms with one pipe selected; main thread 2.24, 2.04, 2.22 and 3.19 ms per frame; 361 draw calls (359 in the hover series with nothing selected on the canvas: the selection presentation adds two) and 2,133,368 triangles. **The A/A differences in these two series (0.24 ms and 1.04 ms of mean interval) are five to twenty-five times T1's floor of about 0.04 ms**, so one pair under this load resolves nothing between "one" and "all". See "What reads oddly".
3. **`hover` at 10,000 pipes is limited by the main thread**: busy 99.6 % and 99.1 % of the window; mean interval 6.44 and 5.07 ms, p95 11.9 and 6.4 ms; 1,070 and 1,212 pointer events dispatched in the 6 s window (1,700 and 1,759 moves sent over the whole 8.15 s, so nearly every move arrived as its own event), **5.58 and 4.90 ms of main-thread time per pointer event**, of which 5.36 and 4.64 ms script. All events in the window reached the canvas (labels off). The camera, the model, the geometry and the selection were unchanged; the product drew 1,210 and 1,620 times in the span. Per pointer event, hovering on this build costs more than twice what an orbit frame costs: the product picks on every move and redraws. That is the baseline a hover halo adds to.
4. **`pairs` on the recolour build: 199 of 200 samples pass the simplified rule with no casing**; the one failure is `too-few-pairs` (sample 60: 17 new pixels, 1 pair, ground in reach of one pixel only). No sample showed no cue and none lacked ground altogether. **This is the recoloured body and the diamond cue passing, as the README says the simplified rule lets them**: across the 200 samples the share of new pixels that lay over ground before the pick is 0 % (at most 1 pixel in any sample) and over the figure 100 %; inner pixels median 6 (0 to 17); new pixels median 28 (17 to 84); pairs median 21 (1 to 43); contrast refused no pair (`pairsIgnoringContrast` equals `pairs` in every sample). A halo build should show the opposite signature (new pixels over ground, few inner) **on top of** the diamond's pixels, so read its run against this one.
5. **Expected winners do not apply at the fitted camera.** The camera read back is at (16.23, 11.81, 11.18) looking at (5.05, 0.63, 0.00); the sample file's nominal camera is at (12.4, 8.1, 13.7) looking at (5, 0.65, 0.5): 5.90 m apart. Selected = nominal expected in 19 samples, = the anchor in 3, neither in 180, nothing selected in 0. Every click selected exactly one entity (179 pipes, 15 components, 5 supports, 1 node; the nominal list has 176, 15 and 9). With the camera that far from nominal this says nothing about picking; it does mean `pairs` cannot yet say "wrong element selected" outright (proposal 2 in the README).

## What reads oddly

- **The host was loaded and got more so during the series**: 1-minute load 3.4 to 5.0 in the first series, 4.8 to 11.5 in the second, 11.3 to 11.7 in the hover series (another child was building and testing in the same worktree outside the lock's timed work). Run b of `--select one` has 142 pointer send gaps over 8 ms against 0 in run a, and sent 1,492 moves against 1,849: the tool's own process was held back, which the README says to read with suspicion. Its 3.43 ms is that, not the product. Run a of `--select all` has 25 such gaps. The hover A/A differs by 1.36 ms of mean interval (21 %) and its p95 by 5.5 ms under a load of 11. **For sizing a halo's cost the manager needs three pairs on a quieter host, as T1 did; the brief allowed one short pair per series and no more.**
- **With labels on, 80 labels were rendered** (the budget) in the `run` series, selected or not.
- **In the `hover` series "Selected (reported)" is 1**: the product's start-up selection, the project, which has nothing on the canvas.
- **Nearly every move sent became a pointer event**, although a browser may merge moves that arrive within one frame: under `uncapped` frames come faster than the 4 ms send period, so there is little to merge. Under `vsync` expect fewer events than moves.
- **`pairs` took about 7 minutes** for 200 samples (two settles per sample).

## Summary tables (`summarize` over the four files)

## Series

| File | Pipes | Mode | Labels | Theme | DPR | Window | Canvas (CSS px) | Pacing | Pacing calibration p50 (ms) | Uncapped took effect | Warm-up / measure (ms) | Build index.html SHA-256 | Browser | WebGL renderer |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| run_select-all_10000_schematic_uncapped.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 6000 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| run_select-one_10000_schematic_uncapped.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 6000 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| hover_10000_schematic_uncapped.json | 10000 | schematic | off | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 6000 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |

## Builds

A build identity is the SHA-256 of index.html with the names and sizes of the files under assets.

| File | Variant | Query | Build directory | index.html SHA-256 | Asset files | Asset bytes | Against the first variant |
|---|---|---|---|---|---|---|---|
| run_select-all_10000_schematic_uncapped.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| run_select-all_10000_schematic_uncapped.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | same build identity |
| run_select-one_10000_schematic_uncapped.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| run_select-one_10000_schematic_uncapped.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | same build identity |
| hover_10000_schematic_uncapped.json | a | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | the reference |
| hover_10000_schematic_uncapped.json | b | (empty) | {OUTSIDE_REPOSITORY}/dist_base_c2 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 12 | 2731368 | same build identity |

- run_select-all_10000_schematic_uncapped.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.
- run_select-one_10000_schematic_uncapped.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.
- hover_10000_schematic_uncapped.json: every variant has the same build identity and the same query, so this is an A/A series and its differences are noise.

## Variants

| File | Variant | Runs | Per-run p95 (ms): p50 | Per-run p95 (ms): p95 | Per-run p95 (ms): range | Per-run p50 (ms) | Mean interval (ms) | Over 16.7 ms | Over 33.3 ms | Main thread (ms/frame) | Draw calls | Triangles | Mean load (1 min) | Console errors |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| run_select-all_10000_schematic_uncapped.json | a | 1 | 3.80 | 3.80 | 3.80 to 3.80 | 2.30 | 2.467 | 0.0% | 0.0% | 2.236 | 361 | 2133368 | 3.90 | 0 |
| run_select-all_10000_schematic_uncapped.json | b | 1 | 3.20 | 3.20 | 3.20 to 3.20 | 2.20 | 2.228 | 0.0% | 0.0% | 2.044 | 361 | 2133368 | 4.68 | 0 |
| run_select-one_10000_schematic_uncapped.json | a | 1 | 3.30 | 3.30 | 3.30 to 3.30 | 2.30 | 2.391 | 0.0% | 0.0% | 2.221 | 361 | 2133368 | 4.94 | 0 |
| run_select-one_10000_schematic_uncapped.json | b | 1 | 4.30 | 4.30 | 4.30 to 4.30 | 3.50 | 3.430 | 0.0% | 0.0% | 3.187 | 361 | 2133368 | 8.30 | 0 |
| hover_10000_schematic_uncapped.json | a | 1 | 11.90 | 11.90 | 11.90 to 11.90 | 5.50 | 6.436 | 0.4% | 0.0% | 6.408 | 359 | 2133368 | 11.59 | 0 |
| hover_10000_schematic_uncapped.json | b | 1 | 6.40 | 6.40 | 6.40 to 6.40 | 4.80 | 5.072 | 0.0% | 0.0% | 5.026 | 359 | 2133368 | 11.49 | 0 |

## Paired differences

Each later variant minus the first variant, pair by pair in run order: median [smallest to largest]. With one build under two names this is the noise floor.

| File | Variant minus reference | Pairs | p95 difference (ms) | As a share of the reference p95 | p50 difference (ms) | Mean-interval difference (ms) | As a share of the reference mean | Main-thread difference (ms/frame) |
|---|---|---|---|---|---|---|---|---|
| run_select-all_10000_schematic_uncapped.json | b - a | 1 | -0.60 | -15.8% | -0.10 | -0.239 | -9.7% | -0.192 |
| run_select-one_10000_schematic_uncapped.json | b - a | 1 | 1.00 | 30.3% | 1.20 | 1.039 | 43.5% | 0.966 |
| hover_10000_schematic_uncapped.json | b - a | 1 | -5.50 | -46.2% | -0.70 | -1.364 | -21.2% | -1.382 |

## Selection and hover

Kind run: an orbit with the left button down. Kind hover: the pointer sweeps the canvas with no button pressed, and the frame and main-thread columns above describe that sweep. Selected is the count the product's diagnostics surface reported before the measurement. A pointer event is a pointermove the page dispatched in the window; the browser may merge moves that arrive within one frame. With select none the count reported is the product's start-up selection (the project).

| File | Kind | Variant | Select | Selected (reported) | Selection took (ms) | Pointer events in window | Share received by the canvas | Main thread (ms per pointer event) | Script (ms per pointer event) | Product submissions in span | Camera sequence change |
|---|---|---|---|---|---|---|---|---|---|---|---|
| run_select-all_10000_schematic_uncapped.json | run | a | all | 10000 | 1095 | n/a | n/a | n/a | n/a | 3344 | 5061 |
| run_select-all_10000_schematic_uncapped.json | run | b | all | 10000 | 1102 | n/a | n/a | n/a | n/a | 3686 | 5572 |
| run_select-one_10000_schematic_uncapped.json | run | a | one | 1 | 750 | n/a | n/a | n/a | n/a | 3402 | 5243 |
| run_select-one_10000_schematic_uncapped.json | run | b | one | 1 | 815 | n/a | n/a | n/a | n/a | 2386 | 3629 |
| hover_10000_schematic_uncapped.json | hover | a | none | 1 | n/a | 1070 | 100.0% | 5.584 | 5.359 | 1210 | 0 |
| hover_10000_schematic_uncapped.json | hover | b | none | 1 | n/a | 1212 | 100.0% | 4.903 | 4.642 | 1620 | 0 |

## Halo pairs (simplified rule, no casing)

A simplified form of the rule in the lane's proposal P1, section 4.4: no predicted band and nothing exclusive to the winner, so every newly Selection-coloured pixel in the region counts, whatever drew it. A sample passes when the image changed and at least 4 pairs exist. No cue seen: the image did not change or no pixel newly qualified. No ground beside: pixels newly qualified and none has ground within 2 CSS px. Expected winners are the sample file's nominal ones; where the camera differs they need not apply.

| File | Describes | Pipes | Mode | Theme | DPR | Canvas (CSS px) | Build index.html SHA-256 | Samples | Measured | Passes | Fail: no cue seen | Fail: no ground beside | Fail: too few pairs | Not attempted / failed | Selected = nominal expected | Selected = anchor | Selected = neither | Nothing selected | Expected winners apply (camera) | Median new px | Median pairs | Median inner px | Median share over figure |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| pairs_1000_schematic_light_recolour.json | the build before slice C2: selection is a RECOLOUR of the body (plus the first profile's diamond cue), not a halo | 1000 | schematic | light | 1 | 794 x 560 | fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c | 200 | 200 | 199 | 0 | 0 | 1 | 0 / 0 | 19 | 3 | 180 | 0 | no (camera 5.90 from nominal) | 28 | 21 | 6 | 100.0% |


Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
