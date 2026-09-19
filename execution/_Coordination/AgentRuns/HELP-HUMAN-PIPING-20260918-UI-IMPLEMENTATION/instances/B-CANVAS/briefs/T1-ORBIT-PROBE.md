# Sealed brief — T1-ORBIT-PROBE: the lane's guidance probe for per-frame cost

Sealed by B-CANVAS (WORKING_ITEMS, Type 1, the canvas lane manager of Tranche B) on 2026-09-19 before launch. Role of the reader: TASK (Type 2). Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, background. **You work alone and never delegate: launch no agent.** Your parent is B-CANVAS; corrections reach you by message with your agent id.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the git worktree your launch message names; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{LANE}` is `{RUN}/instances/B-CANVAS`. Write no absolute machine path in any file you author, in any output file the tool writes, or in your return. Work only in the worktree you are given. **Run no git command that changes state** (no add, commit, stash, checkout, reset, merge, rebase, push); your parent commits. On this host the file-writing tools refuse paths in that worktree, because the session was opened in another one: write files through the shell (heredocs or short exact-once scripts), as the last child did.

## Why this exists

The lane's brief says that per-frame work is measured, not assumed: a slice that adds work per frame is built behind a probe and kept only if the owner's limits hold (D-72: orbit p95 at most 16.7 ms in the centreline geometry mode and 33.3 ms at real outside diameter, at 1,000 and 10,000 pipes; a settled viewport with stable resources). The next slice, C1E, draws a one-pixel edge line on every tube, which is the largest unmeasured per-frame cost in the lane. Later slices (hover halos, Budget labels) need the same measurement. You build the measuring tool and show what its noise floor is on this host. You do not build the edge line and you change no product code.

The tool guides the lane. It decides nothing about D-72: the qualification runs are ROOT's, by a separate runner and a separate instrument. Say so in the tool's README and claim nothing more.

## Limits that bind this work (the lane's limits, carried in full)

- **One mutation route.** Every engineering action is a typed operation through `applyModelOperation` / `applyOperationBatch` into the Rust applier. No component mutates the model any other way. Tables and canvas are projections; neither holds state the model does not.
- **Result integrity.** Preserve the Current and Historical designation, the exact solve-input basis, the generation gates, `commitModelAfterSolveInvalidation`, `clearComputedModelState`, reviewed application, undo and redo, and persistence compatibility. The Stale standing is not drawn in this tranche.
- **Picking repair preserved.** PR #794's shared closest-point computation and its tests stay. Never alter a tolerance, an oracle expectation, a benchmark limit or a frozen characterization value to make something pass.
- **Rendering foundation retained**: persistent renderer, instancing and chunking, invalidation scheduler, resource ownership ledger, typed model index, selection and picking. You change none of these.
- **Every control maps to an operation or a classed gap**; never faked. This task adds, removes, enables and disables no control.
- **Semantic changes are named.** This task makes none: it changes no product file.
- **Separately scoped work stays separate**: pressure runtime, connector mechanics, sparse execution, export implementation, schemas, `core/**`, `src-tauri/**`, the identity layer.
- **Copy.** The product is SWBPIPE. No maturity sentence, no acceptance sentence, no other vendor's product named; Canadian spelling. This task changes no product copy.
- **Accessibility.** WCAG 2.2 AA for every control touched; PDU-045 and PDU-046 remain holds; claim no usability acceptance. This task touches no control.
- **Browser tests share the host.** Another lane works at the same time in another worktree of this repository on the same machine. **Every browser you launch and every server you start goes through `sh {RUN}/tools/with_e2e_lock.sh <command>`.** Waiting for the lock is normal, sometimes for many minutes. One series per lock hold; keep a hold under about fifteen minutes. Use a port other than 5174, 5175, 5176, 5177 and 5184: use 5185. Stop every server you start and confirm the port is free and the lock released. If your parent tells you ROOT has announced a timed benchmark, run no build, test or probe until told it is over.
- **Never weaken a test.** You edit no existing test.
- **Frozen history** is never edited: `{WORKING_ROOT}/execution/**` outside the two folders named under "Write scope", `validation/evidence/**`, `plans/**`, `docs/_history/**`, `docs/_ScopeChange/**`.
- **The benchmark instrument.** Nothing under `{DESKTOP}/e2e/ui-foundation/**` is edited, and the tool imports nothing from it. You may **read** it to learn how it opens a fixture in the page and how it drives an orbit (`benchmark-harness.ts`, `ui-foundation-performance.benchmark.ts`, `characterization-commands.ts`, `candidate-control-binding-v1.json`), and the tool reads the two fixture files as data: `e2e/ui-foundation/fixtures/ui-foundation-1000.model.json` and `ui-foundation-10000.model.json`. Never copy a tolerance, a limit or an oracle from it into the tool as the tool's own rule.
- **No product file changes.** Nothing under `{DESKTOP}/src/**`, no `package.json`, no Playwright configuration, no lockfile. If the tool seems to need a product change (a hook, a test id), stop and put the exact need in your return.

## Write scope

`{LANE}/tools/**` (the tool and its README) and `{LANE}/probes/T1/**` (the evidence this task produces). Build output (`{DESKTOP}/dist`, ignored) and a scratch directory outside the repository are yours to use. Nothing else.

## What to build: `{LANE}/tools/orbit_probe.mjs`

Plain Node ES module, no new dependency, no TypeScript. It finds `{WORKING_ROOT}` by walking up from its own location to the directory that holds `software-workflow.json`, and loads `playwright` with `createRequire` from there. Browser: `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` when set and present, otherwise Playwright's `channel: "chrome"`, otherwise the bundled Chromium; headless. Record the browser's version string and the WebGL renderer string in every output; never record an executable path.

It serves a built `dist` directory itself (a small static server on 127.0.0.1: correct content types including `application/wasm`, the single-page fallback, no caching), so that a measured build can be a copy kept outside `{DESKTOP}/dist` and later builds cannot disturb a run. It never builds.

### Subcommands (this interface is a contract; the next child's brief is written against it)

```
node orbit_probe.mjs run --dist <dir> --pipes <1000|10000> --mode <schematic|actual-od>
     --labels <on|off> --theme <light|dark> --dpr <1|2> --window <WxH> --pacing <vsync|uncapped>
     --variants "<name>=<query>;<name>=<query>" --pairs <n>
     [--warmup-ms 2000] [--measure-ms 10000] [--port 5185] --out <file.json>
node orbit_probe.mjs resources --dist <dir> --pipes <1000|10000> [--query <query>] [--dpr 2]
     [--window <WxH>] [--port 5185] --out <file.json>
node orbit_probe.mjs summarize <file.json> [<file.json> ...]
node orbit_probe.mjs self-test --dist <dir>
```

- A **variant** is a name and a URL query string appended to the page's address (it may be empty). The tool knows nothing about what a query means; the product code under test reads it. `--variants "off=;band=canvasProbe=edge-band" --pairs 3` runs off, band, off, band, off, band. Every run is a fresh browser and a fresh page.
- **`run`**, for each run in the interleaved order: record the host's load averages (`os.loadavg()`) and the UTC time; open the page at the window size and device pixel ratio given; open the fixture by the product's own means, the way the instrument does; set the theme, the geometry mode and the labels through the product's own controls; Isometric, then Fit; wait until the viewport has settled; then orbit with **real pointer input** (mouse down on the canvas, continuous moves along a smooth closed path whose position is a function of elapsed time, so every variant sweeps the same angles; at least one move per 8 ms) for the warm-up and then the measurement window; record the load averages again. Collect, for the measurement window only:
  1. frame intervals from `requestAnimationFrame` timestamps taken inside the page into a preallocated typed array (no allocation per frame): count, mean, p50, p95, p99, maximum, the fraction over 16.7 ms and over 33.3 ms, and the offsets of every interval over 33.3 ms;
  2. main-thread cost from the DevTools protocol's `Performance.getMetrics` deltas (task, script, layout and style durations) over the window, in total and per frame;
  3. from the product's diagnostics surface (`globalThis.__openPipeStressUiDiagnosticsV1.readCurrent()`; its type is in `src/features/workspace/uiDiagnostics.ts`): renderer information (draw calls, triangles, lines, points, geometries, textures), the ownership ledger's `live`, `created` and `disposed`, the canvas's CSS size, drawing-buffer size and pixel ratio, the geometry mode and its OD status, the label state and rendered count, the model's identity;
  4. every console error and warning, and every page error.
  `--pacing uncapped` launches the browser with `--disable-frame-rate-limit` and `--disable-gpu-vsync`, so that the interval reflects what a frame costs, not the display's period. State in the output whether it took effect (a p50 near 16.7 ms means it did not).
- **`resources`**: with no timing, record the ledger and renderer information, then switch the theme light to dark to light twice, switch the geometry mode and back twice, toggle the labels and the grid twice, and record them again; report whether `live` is identical before and after and whether `created − disposed` equals `live` throughout. Then a **settle check**: after the last input, wait 1.5 s and report whether any product-owned animation frame is pending and whether anything was drawn during the following second. Choose a method that does not depend on a product change, explain it in the README, and show in your evidence that it reports "not settled" while an orbit is under way.
- **`summarize`** prints a markdown table, one row per variant per file: runs, p50 and p95 of the per-run p95 values, mean interval, fraction over each limit, main-thread milliseconds per frame, draw calls and triangles, the mean load average, and any console error. It prints medians and ranges, never a verdict.
- **`self-test`** checks the percentile and summary arithmetic on fixed arrays, then does one two-second run on the 1,000-pipe fixture and validates the shape of the output.

### Output files

JSON, one per series, pretty-printed and small: no per-frame arrays longer than the list of long intervals. Each holds `tool: "b-canvas-orbit-probe"`, a format version, the full argument list with every path reduced to something that is not a machine path, and a **build identity**: the SHA-256 of `dist/index.html` and the names and sizes of the files under `dist/assets`. No absolute path anywhere; the harness's self-check scans this folder.

### A recommended extra

Today's layout gives a canvas of about 794 × 580 CSS pixels at 1440 × 920. The redesigned Model view will be 1000 × 828, which is not on this branch yet. `--window` exists so that a run can be made at a window large enough for the canvas's CSS area to reach 828,000 px²; find such a size, record the canvas size it gives, and name it in the README as the lane's conservative stand-in, clearly not the redesigned layout.

## Evidence to produce, under `{LANE}/probes/T1/`

Build the current product once (`npm run build:desktop` from `{WORKING_ROOT}`), copy `{DESKTOP}/dist` to your scratch directory, and measure the copy.

1. `self-test` passing.
2. An **A/A series**: the same build and the same empty query under two names (`--variants "a=;b=" --pairs 3`), at 10,000 pipes, light, labels on, device pixel ratio 2, window 1440 × 900, for each geometry mode and each pacing (four series), and the two `vsync` ones again at 1,000 pipes. The difference between `a` and `b` is the tool's noise floor on this host. Report it per measure, with the load averages beside it, and say what difference between two variants the tool could and could not detect.
3. One `resources` run at 1,000 pipes and one at 10,000.
4. The `summarize` output for all of it, saved as `SUMMARY.md` beside the JSON files, with two or three sentences on what reads oddly (a hitch that repeats, a pacing mode that did not take effect, a fixture that loads slowly).

Do not leave a server or a browser running between series. Do not run a build while a series is measuring.

## How to check your work

1. `node --check {LANE}/tools/orbit_probe.mjs`.
2. `sh {RUN}/tools/with_e2e_lock.sh node {LANE}/tools/orbit_probe.mjs self-test --dist <your copy>`, exit 0.
3. After every lock hold: the lock directory is absent and nothing listens on 5185.
4. `python3 tools/validation/validate_claims_language.py` from `{REPO_ROOT}`, VALID.
5. `grep -rn` for the home directory's prefix and for `/private/` under `{LANE}/tools` and `{LANE}/probes/T1`: no match.
6. `git status --short`: only files under your two folders.

## What to return

Your final message is the return; it is retained verbatim. Say which model you are. Give: what you read; the files you wrote with SHA-256; how the tool opens a fixture, drives the orbit, takes each measure and checks settling, in enough detail for a reviewer to judge the method; every command you ran with its result; the A/A numbers and the noise floor you conclude from them, per measure; whether uncapped pacing took effect headless on this host; the window size that reaches 828,000 px² of canvas; anything in the product that the tool needed and did not have, as an exact proposal and not a change; what the tool does not measure. Claim no performance, usability or conformance acceptance. End with the line: Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
