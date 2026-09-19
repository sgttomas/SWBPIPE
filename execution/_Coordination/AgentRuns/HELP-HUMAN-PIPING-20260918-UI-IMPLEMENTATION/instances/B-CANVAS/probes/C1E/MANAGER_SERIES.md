# C1E measurement series, run by the lane manager

Run by B-CANVAS (Claude Fable 5.1) on 2026-09-19 between 09:06Z and 09:33Z, because the implementer child C1E-EDGE could not measure: the probe's README did not exist while it worked (`SUMMARY.md` beside this file says so). The tool is `{LANE}/tools/orbit_probe.mjs` at SHA-256 `59b1807659e6b82f0554c0af659076ec4e2f2b569ae35dc846ec0f9e835c7aac` for every series. Variant `off` is the base build (lane head `b415b43df`, `index.html` `e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb`); variant `on` is the final C1E build (`index.html` `0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1`, which `{DESKTOP}/dist` built from commit `442610cc7` reproduces). Both are copies outside the repository. Every series: labels on, device pixel ratio 2, `--pairs 3` interleaved (off, on, off, on, off, on), one series per lock hold, no build or unit suite of the lane's running meanwhile. Window 1440 x 900 gives a canvas of 794 x 560 CSS px; window 1646 x 1168 gives 1000 x 828, the lane's stand-in for D-72's canvas area and not the redesigned layout.

**This guides the lane and decides nothing about D-72.** No performance acceptance is claimed.

## The rule for keeping the line, applied

1. Wherever the base build holds a D-72 orbit limit in a series, the build with the line holds it: **yes, in all thirteen series.** No interval over 33.3 ms in any of 78 runs; under `vsync` no refresh was missed by either variant (mean interval 16.666 ms in every run).
2. With `uncapped` pacing the p95 frame interval with the line is 3.4 to 3.5 ms at 10,000 pipes and 2.8 ms at 1,000, against limits of 16.7 ms (schematic) and 33.3 ms (actual OD): within the limit by a factor of about five and ten.
3. `resources` at 1,000 and 10,000 pipes on the candidate: `live` identical before and after, 4 of 4 round trips identical, `created - disposed = live` throughout, settled 1.5 s after the last input, no console error or warning.
4. No margin is thin by the rule's first test (no p95 is above 80 % of its limit under `uncapped`; under `vsync` the p95 is the refresh period for both variants). By its second test every on-minus-off difference is inside the spread T1 found between identical builds (about 0.04 ms uncapped, about 0.25 ms of main thread under `vsync`), so nothing is there to repeat: the largest uncapped median difference is +0.032 ms (1.6 %), with pairs on both sides of zero.

**What this does not show.** T1 found that on this host the uncapped orbit is limited by the main thread (about 96 % busy at about 2 ms a frame). The line adds work in the fragment and vertex shaders and none on the main thread, so a GPU cost that stays under that bound is not visible to this tool, and "no difference" here does not speak for a GPU-bound host. What it does show is that with the line the frame still completes well inside both limits on this host at both canvas sizes. Whether D-72's limits hold is decided only by ROOT's qualification runs.

Load averages (1 minute) per variant are in the table below; they ranged from 2.8 to 5.8.

---

What follows is the unedited output of `node orbit_probe.mjs summarize` over the fifteen JSON files.

# Orbit probe summary

Guidance for the B-CANVAS lane only. This output decides nothing about D-72; the qualification runs are ROOT's, by a separate runner and a separate instrument.

Medians and ranges only. Percentiles are nearest-rank; with three runs the p95 of the per-run p95 values is the largest of them. Intervals are between requestAnimationFrame timestamps in the measurement window.

## Series

| File | Pipes | Mode | Labels | Theme | DPR | Window | Canvas (CSS px) | Pacing | Pacing calibration p50 (ms) | Uncapped took effect | Warm-up / measure (ms) | Build index.html SHA-256 | Browser | WebGL renderer |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| c1e_10000_actual-od_uncapped.json | 10000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 [0.00 to 0.10] | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_actual-od_uncapped_1646x1168.json | 10000 | actual-od | on | light | 2 | 1646x1168 | 1000 x 828 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_actual-od_uncapped_dark.json | 10000 | actual-od | on | dark | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_actual-od_vsync.json | 10000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_actual-od_vsync_1646x1168.json | 10000 | actual-od | on | light | 2 | 1646x1168 | 1000 x 828 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_schematic_uncapped.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_schematic_uncapped_1646x1168.json | 10000 | schematic | on | light | 2 | 1646x1168 | 1000 x 828 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_schematic_vsync.json | 10000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_10000_schematic_vsync_1646x1168.json | 10000 | schematic | on | light | 2 | 1646x1168 | 1000 x 828 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_1000_actual-od_uncapped.json | 1000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_1000_actual-od_vsync.json | 1000 | actual-od | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_1000_schematic_uncapped.json | 1000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | uncapped | 0.10 | yes | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |
| c1e_1000_schematic_vsync.json | 1000 | schematic | on | light | 2 | 1440x900 | 794 x 560 | vsync | 16.70 | not requested | 2000 / 10000 | differs by variant; see Builds | 153.0.8010.48 | ANGLE (Apple, ANGLE Metal Renderer: Apple M5 Max, Unspecified Version) |

## Builds

A build identity is the SHA-256 of index.html with the names and sizes of the files under assets.

| File | Variant | Query | Build directory | index.html SHA-256 | Asset files | Asset bytes | Against the first variant |
|---|---|---|---|---|---|---|---|
| c1e_10000_actual-od_uncapped.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_actual-od_uncapped.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_actual-od_uncapped_1646x1168.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_actual-od_uncapped_1646x1168.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_actual-od_uncapped_dark.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_actual-od_uncapped_dark.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_actual-od_vsync.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_actual-od_vsync.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_actual-od_vsync_1646x1168.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_actual-od_vsync_1646x1168.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_schematic_uncapped.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_schematic_uncapped.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_schematic_uncapped_1646x1168.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_schematic_uncapped_1646x1168.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_schematic_vsync.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_schematic_vsync.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_10000_schematic_vsync_1646x1168.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_10000_schematic_vsync_1646x1168.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_1000_actual-od_uncapped.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_1000_actual-od_uncapped.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_1000_actual-od_vsync.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_1000_actual-od_vsync.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_1000_schematic_uncapped.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_1000_schematic_uncapped.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |
| c1e_1000_schematic_vsync.json | off | (empty) | {OUTSIDE_REPOSITORY}/dist_base_b415b43df | e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb | 12 | 2722778 | the reference |
| c1e_1000_schematic_vsync.json | on | (empty) | {OUTSIDE_REPOSITORY}/dist_stage2 | 0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1 | 12 | 2731193 | different build identity |

- c1e_10000_actual-od_uncapped.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_actual-od_uncapped_1646x1168.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_actual-od_uncapped_dark.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_actual-od_vsync.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_actual-od_vsync_1646x1168.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_schematic_uncapped.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_schematic_uncapped_1646x1168.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_schematic_vsync.json: the variants do not share one build identity; they compare different builds.
- c1e_10000_schematic_vsync_1646x1168.json: the variants do not share one build identity; they compare different builds.
- c1e_1000_actual-od_uncapped.json: the variants do not share one build identity; they compare different builds.
- c1e_1000_actual-od_vsync.json: the variants do not share one build identity; they compare different builds.
- c1e_1000_schematic_uncapped.json: the variants do not share one build identity; they compare different builds.
- c1e_1000_schematic_vsync.json: the variants do not share one build identity; they compare different builds.

## Variants

| File | Variant | Runs | Per-run p95 (ms): p50 | Per-run p95 (ms): p95 | Per-run p95 (ms): range | Per-run p50 (ms) | Mean interval (ms) | Over 16.7 ms | Over 33.3 ms | Main thread (ms/frame) | Draw calls | Triangles | Mean load (1 min) | Console errors |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| c1e_10000_actual-od_uncapped.json | off | 3 | 3.40 | 3.50 | 3.40 to 3.50 | 2.10 | 1.997 [1.975 to 2.026] | 0.0% | 0.0% | 1.929 [1.910 to 1.956] | 359 | 2133368 | 3.85 | 0 |
| c1e_10000_actual-od_uncapped.json | on | 3 | 3.40 | 3.50 | 3.40 to 3.50 | 2.10 [2.10 to 2.20] | 1.989 [1.981 to 2.013] | 0.0% | 0.0% | 1.922 [1.915 to 1.947] | 359 | 2133368 | 4.04 | 0 |
| c1e_10000_actual-od_uncapped_1646x1168.json | off | 3 | 3.40 | 3.50 | 3.40 to 3.50 | 2.20 | 1.984 [1.978 to 2.086] | 0.0% | 0.0% | 1.915 [1.910 to 2.006] | 359 | 2133368 | 4.59 | 0 |
| c1e_10000_actual-od_uncapped_1646x1168.json | on | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.20 [2.10 to 2.20] | 2.013 [2.004 to 2.016] | 0.0% | 0.0% | 1.940 [1.936 to 1.945] | 359 | 2133368 | 4.71 | 0 |
| c1e_10000_actual-od_uncapped_dark.json | off | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.10 | 1.993 [1.976 to 2.000] | 0.0% | 0.0% | 1.926 [1.912 to 1.930] | 359 | 2133368 | 3.56 | 0 |
| c1e_10000_actual-od_uncapped_dark.json | on | 3 | 3.40 | 3.50 | 3.40 to 3.50 | 2.10 [1.90 to 2.10] | 1.986 [1.971 to 2.003] | 0.0% | 0.0% | 1.919 [1.903 to 1.936] | 359 | 2133368 | 3.58 | 0 |
| c1e_10000_actual-od_vsync.json | off | 3 | 16.80 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 33.1% [31.7% to 33.7%] | 0.0% | 3.058 [2.958 to 3.081] | 359 | 2133368 | 4.70 | 0 |
| c1e_10000_actual-od_vsync.json | on | 3 | 16.80 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 32.7% [30.6% to 33.6%] | 0.0% | 3.112 [3.087 to 3.184] | 359 | 2133368 | 4.39 | 0 |
| c1e_10000_actual-od_vsync_1646x1168.json | off | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 32.4% [30.2% to 32.7%] | 0.0% | 3.145 [3.038 to 3.184] | 359 | 2133368 | 3.31 | 0 |
| c1e_10000_actual-od_vsync_1646x1168.json | on | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 30.4% [30.4% to 32.1%] | 0.0% | 3.021 [2.999 to 3.026] | 359 | 2133368 | 3.15 | 0 |
| c1e_10000_schematic_uncapped.json | off | 3 | 3.40 | 3.40 | 3.40 to 3.40 | 2.20 | 1.990 [1.974 to 1.991] | 0.0% | 0.0% | 1.925 [1.908 to 1.926] | 359 | 2133368 | 3.08 | 0 |
| c1e_10000_schematic_uncapped.json | on | 3 | 3.40 | 3.50 | 3.40 to 3.50 | 2.20 | 2.015 [1.998 to 2.034] | 0.0% | 0.0% | 1.949 [1.934 to 1.967] | 359 | 2133368 | 3.41 | 0 |
| c1e_10000_schematic_uncapped_1646x1168.json | off | 3 | 3.50 | 3.50 | 3.40 to 3.50 | 2.10 [2.10 to 2.20] | 2.035 [2.002 to 2.110] | 0.0% | 0.0% | 1.962 [1.932 to 2.016] | 359 | 2133368 | 4.69 | 0 |
| c1e_10000_schematic_uncapped_1646x1168.json | on | 3 | 3.50 | 3.50 | 3.50 to 3.50 | 2.20 [2.10 to 2.20] | 2.044 [2.037 to 2.048] | 0.0% | 0.0% | 1.969 [1.965 to 1.975] | 359 | 2133368 | 4.81 | 0 |
| c1e_10000_schematic_vsync.json | off | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 29.9% [29.2% to 30.5%] | 0.0% | 3.011 [2.984 to 3.114] | 359 | 2133368 | 4.74 | 0 |
| c1e_10000_schematic_vsync.json | on | 3 | 16.80 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 31.2% [30.2% to 33.9%] | 0.0% | 3.038 [3.000 to 3.047] | 359 | 2133368 | 4.28 | 0 |
| c1e_10000_schematic_vsync_1646x1168.json | off | 3 | 16.80 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 32.1% [31.7% to 33.1%] | 0.0% | 2.993 [2.987 to 3.059] | 359 | 2133368 | 2.82 | 0 |
| c1e_10000_schematic_vsync_1646x1168.json | on | 3 | 16.70 | 16.70 | 16.70 to 16.70 | 16.70 | 16.666 | 29.9% [28.7% to 30.6%] | 0.0% | 3.038 [2.953 to 3.120] | 359 | 2133368 | 2.75 | 0 |
| c1e_1000_actual-od_uncapped.json | off | 3 | 2.80 | 2.80 | 2.80 to 2.80 | 1.10 [1.10 to 1.20] | 1.394 [1.391 to 1.394] | 0.0% | 0.0% | 1.321 [1.319 to 1.322] | 54 | 213488 | 5.15 | 0 |
| c1e_1000_actual-od_uncapped.json | on | 3 | 2.80 | 2.80 | 2.80 to 2.80 | 1.20 [1.10 to 1.20] | 1.395 [1.393 to 1.397] | 0.0% | 0.0% | 1.324 [1.324 to 1.325] | 54 | 213488 | 5.28 | 0 |
| c1e_1000_actual-od_vsync.json | off | 3 | 16.70 | 16.70 | 16.70 to 16.70 | 16.70 | 16.666 | 28.2% [27.9% to 29.9%] | 0.0% | 2.987 [2.870 to 3.092] | 54 | 213488 | 3.15 | 0 |
| c1e_1000_actual-od_vsync.json | on | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 29.5% [29.4% to 30.1%] | 0.0% | 2.865 [2.856 to 3.209] | 54 | 213488 | 3.21 | 0 |
| c1e_1000_schematic_uncapped.json | off | 3 | 2.80 | 2.80 | 2.80 to 2.80 | 1.10 [1.00 to 1.20] | 1.392 [1.389 to 1.395] | 0.0% | 0.0% | 1.323 [1.319 to 1.324] | 54 | 213488 | 5.53 | 0 |
| c1e_1000_schematic_uncapped.json | on | 3 | 2.80 | 2.80 | 2.80 to 2.80 | 1.20 [1.00 to 1.30] | 1.399 [1.394 to 1.400] | 0.0% | 0.0% | 1.330 [1.325 to 1.332] | 54 | 213488 | 5.75 | 0 |
| c1e_1000_schematic_vsync.json | off | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 30.1% [28.7% to 31.7%] | 0.0% | 2.949 [2.936 to 3.048] | 54 | 213488 | 2.94 | 0 |
| c1e_1000_schematic_vsync.json | on | 3 | 16.70 | 16.80 | 16.70 to 16.80 | 16.70 | 16.666 | 29.9% [29.0% to 30.7%] | 0.0% | 2.746 [2.723 to 2.798] | 54 | 213488 | 3.03 | 0 |

## Paired differences

Each later variant minus the first variant, pair by pair in run order: median [smallest to largest]. With one build under two names this is the noise floor.

| File | Variant minus reference | Pairs | p95 difference (ms) | As a share of the reference p95 | p50 difference (ms) | Mean-interval difference (ms) | As a share of the reference mean | Main-thread difference (ms/frame) |
|---|---|---|---|---|---|---|---|---|
| c1e_10000_actual-od_uncapped.json | on - off | 3 | 0.00 [-0.10 to 0.10] | 0.0% [-2.9% to 2.9%] | 0.00 [0.00 to 0.10] | 0.014 [-0.045 to 0.016] | 0.7% [-2.3% to 0.8%] | 0.012 [-0.041 to 0.018] |
| c1e_10000_actual-od_uncapped_1646x1168.json | on - off | 3 | 0.00 [-0.10 to 0.00] | 0.0% [-2.9% to 0.0%] | 0.00 [-0.10 to 0.00] | 0.032 [-0.082 to 0.035] | 1.6% [-4.1% to 1.8%] | 0.030 [-0.070 to 0.030] |
| c1e_10000_actual-od_uncapped_dark.json | on - off | 3 | 0.00 [0.00 to 0.10] | 0.0% [0.0% to 2.9%] | 0.00 [-0.20 to 0.00] | -0.005 [-0.007 to 0.003] | -0.3% [-0.4% to 0.2%] | -0.007 [-0.009 to 0.006] |
| c1e_10000_actual-od_vsync.json | on - off | 3 | 0.00 [-0.10 to 0.10] | 0.0% [-0.6% to 0.6%] | 0.00 | 0.000 | 0.0% | 0.103 [0.029 to 0.154] |
| c1e_10000_actual-od_vsync_1646x1168.json | on - off | 3 | 0.00 [-0.10 to 0.10] | 0.0% [-0.6% to 0.6%] | 0.00 | 0.000 | 0.0% | -0.146 [-0.158 to -0.017] |
| c1e_10000_schematic_uncapped.json | on - off | 3 | 0.00 [0.00 to 0.10] | 0.0% [0.0% to 2.9%] | 0.00 | 0.025 [0.007 to 0.060] | 1.3% [0.4% to 3.0%] | 0.024 [0.008 to 0.059] |
| c1e_10000_schematic_uncapped_1646x1168.json | on - off | 3 | 0.00 [0.00 to 0.10] | 0.0% [0.0% to 2.9%] | 0.00 [0.00 to 0.10] | 0.013 [-0.073 to 0.042] | 0.6% [-3.6% to 2.1%] | 0.013 [-0.051 to 0.037] |
| c1e_10000_schematic_vsync.json | on - off | 3 | 0.10 [-0.10 to 0.10] | 0.6% [-0.6% to 0.6%] | 0.00 | 0.000 | 0.0% | 0.036 [-0.114 to 0.054] |
| c1e_10000_schematic_vsync_1646x1168.json | on - off | 3 | -0.10 [-0.10 to 0.00] | -0.6% [-0.6% to 0.0%] | 0.00 | 0.000 | 0.0% | 0.045 [-0.034 to 0.061] |
| c1e_1000_actual-od_uncapped.json | on - off | 3 | 0.00 | 0.0% | 0.00 [0.00 to 0.10] | 0.003 [-0.001 to 0.004] | 0.2% [-0.1% to 0.3%] | 0.004 [0.002 to 0.005] |
| c1e_1000_actual-od_vsync.json | on - off | 3 | 0.00 [0.00 to 0.10] | 0.0% [0.0% to 0.6%] | 0.00 | 0.000 | 0.0% | -0.005 [-0.236 to 0.222] |
| c1e_1000_schematic_uncapped.json | on - off | 3 | 0.00 | 0.0% | 0.10 [-0.20 to 0.30] | 0.005 [0.004 to 0.008] | 0.4% [0.3% to 0.6%] | 0.006 [0.006 to 0.009] |
| c1e_1000_schematic_vsync.json | on - off | 3 | 0.00 | 0.0% | 0.00 | 0.000 | 0.0% | -0.213 [-0.302 to -0.151] |

## Resources

| File | Pipes | Steps | live identical before and after | Round trips with identical live | created - disposed = live throughout | Settle check: frame pending / anything drawn / reported settled | Orbit control: frame pending / anything drawn / reported settled | Console errors |
|---|---|---|---|---|---|---|---|---|
| c1e_resources_1000.json | 1000 | 17 | yes | 4 of 4 | yes | no / no (0 draws) / yes | yes / yes (3480 draws) / no | 0 |
| c1e_resources_10000.json | 10000 | 17 | yes | 4 of 4 | yes | no / no (0 draws) / yes | yes / yes (22143 draws) / no | 0 |


Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
