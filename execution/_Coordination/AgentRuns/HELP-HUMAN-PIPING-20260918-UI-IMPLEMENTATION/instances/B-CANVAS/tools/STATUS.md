# STATUS: `orbit_probe.mjs` is UNFINISHED work, paused

Written by T1-ORBIT-PROBE (TASK, Type 2; Claude Fable 5.1) on 2026-09-19, when the lane manager B-CANVAS asked by message for a pause at a clean point. This file is a placeholder for the tool's README, which is not written yet. It is to be removed or replaced when the task resumes and the README exists.

Path placeholders are those of the sealed brief `../briefs/T1-ORBIT-PROBE.md` (SHA-256 `9f67f88e696ada952fbb06eafab347f99c11403b1209e6b250be9b3bb2d2e6e1`).

## What is here

- `{LANE}/tools/orbit_probe.mjs`: whole as far as it goes. All four subcommands of the brief's interface are written: `run`, `resources`, `summarize`, `self-test`. It serves a built `dist` itself, opens the fixture through the product's own start-up load, drives an orbit with real pointer input, and records frame intervals, main-thread cost, the product's diagnostics and console output. It is a guidance tool for the lane. It decides nothing about D-72; the qualification runs are ROOT's, by a separate runner and a separate instrument.
- The manager's addition of 2026-09-19, made by message after sealing, is in: a variant may name its own build (`<name>@<distDir>=<query>`), `--dist` is optional when every variant names one, each run is served by a server rooted at that run's build alone, every variant's build identity is recorded, `summarize` prints each identity once and says whether the variants share it, and `self-test` covers two copies of one build and a directory without `index.html`.
- `{LANE}/probes/T1/`: empty. No evidence file has been written yet.

## What has been run

All browser runs went through `{RUN}/tools/with_e2e_lock.sh`, on port 5185, against the build copy the manager supplied outside the repository (its `index.html` has SHA-256 `8ebcffca356b3ec00186ef5b11343da0824b295b6f0444e058596b73222c058e`). No build was run.

- `node --check {LANE}/tools/orbit_probe.mjs`: passes on the file as it stands.
- `self-test`: passed (exit 0), including the two-copy case. Four small changes were made after that pass and the self-test has not been re-run since: a pacing calibration in a blank page, a histogram of pointer send gaps, the camera-sequence change in the `resources` orbit control, and a garbage collection in the tool's own process before each orbit. Short trial runs of `run` and `resources` after the first three of those changes ended without error.
- Trial runs only, written outside the repository: `run` at 1,000 and 10,000 pipes (both pacings), `resources` at 1,000 pipes, and two larger windows.

## Not done

- The README for the tool.
- Every evidence item of the brief: the saved `self-test` output, the six A/A series, the two `resources` runs, `SUMMARY.md`.
- The brief's closing checks 4 to 6 (claims-language validator, the scan for machine paths, `git status`).

## Exact next step on resume

Re-run `self-test` through the lock. Then run the evidence series one lock hold at a time, write `SUMMARY.md` and the README, remove this file, and run the closing checks.

## Found so far that the brief did not expect (trial runs; not evidence yet)

- With labels on, the product gives the hovered entity a label, and the label takes the pointer. A press at a fixed point after a hover lands on the label, so the orbit never starts (the camera sequence does not move, although frames are still drawn for hover picking). The tool now searches for a press point that the canvas still receives after the pointer has rested on it, and the self-test fails if the orbit does not move the camera.
- `--pacing uncapped` does take effect headless on this host (orbit mean interval about 1.4 ms at 1,000 pipes and about 2.0 ms at 10,000 pipes at real outside diameter, main thread about 95 % busy). An idle product page still ticks at about 16.7 ms under those flags, because it has nothing to present, so the tool judges "took effect" from a blank page that presents a moving square every frame (interval p50 0.1 ms uncapped).
- `requestAnimationFrame` timestamps come in steps of 0.1 ms. Under display pacing the fraction of intervals over 16.7 ms is therefore mostly rounding (about a fifth of intervals in a run with no missed refresh), so the tool also reports the fraction of missed refreshes.
- A window of 1646 x 1168 gives a canvas of exactly 1000 x 828 CSS pixels (828,000 px²) in today's layout.
- In a scratch experiment the uncapped interval at 10,000 pipes did not change with device pixel ratio 1 or 2, with the larger window, or with a GPU synchronisation call every frame (1.95 to 2.01 ms in all six cases). On this host the orbit is limited by the main thread, and the tool does not see GPU cost that stays under that limit. This is an open observation, to be stated in the README under what the tool does not measure.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
