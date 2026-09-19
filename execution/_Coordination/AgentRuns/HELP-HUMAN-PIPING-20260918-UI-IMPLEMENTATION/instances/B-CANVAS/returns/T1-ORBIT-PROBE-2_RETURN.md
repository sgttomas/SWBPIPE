# T1-ORBIT-PROBE-2 return, retained (model that ran: Claude Fable 5.1, claude-fable-5-1, by its own statement)

Retained by B-CANVAS (ROOT's script). What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `864557f89e6e8fdda54f7de10c305ddb8d7dcb889a73fab75ddd529ebaee428d` (121 lines), by this script.

---

T1-ORBIT-PROBE-2 return. The probe is finished: the self-test passes and all eight evidence files ran without error. `SUMMARY.md` and the README are written, `STATUS.md` is removed, and the six closing checks pass. I am Claude Fable 5.1 (`claude-fable-5-1`).

## What I read and verified
All five SHA-256 hashes matched before I relied on the files: this brief (`b44ce7d7…`), the sealed `T1-ORBIT-PROBE.md` (`9f67f88e…`), the pause status (`690024b7…`), `tools/STATUS.md` (`9c17065d…`) and `tools/orbit_probe.mjs` (`ac5290bc…`). I read each whole; the tool was 1,560 lines.

The build copy's `index.html` hash matched `e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb` before the first run. I also read `briefs/_INDEX.md` ("Additions by message"), `{RUN}/tools/with_e2e_lock.sh`, and the field names in `uiDiagnostics.ts`.

Before the first run I confirmed the measured build still has every hook the tool uses: the seven test ids, the "Appearance theme" select, the diagnostics surface, and the start-up model module. The `origin/main` merge did not break the tool.

## Patch to the tool (one, made before any run)
On line 29, `RESERVED_PORTS` changed from `[5174, 5175, 5176, 5177, 5184]` to `[5174, 5175, 5176, 5177, 5183, 5184, 5186]`. This brief adds 5183 and 5186 to the ports never to bind, and C1E-EDGE will run this tool next.

`node --check` passes. The self-test, run through the lock after this last patch, printed `self-test: PASS` with exit 0. That self-test also covers the predecessor's four unverified patches and the two-copy case.

## Files written (SHA-256)
- `{LANE}/tools/orbit_probe.mjs`: `59b1807659e6b82f0554c0af659076ec4e2f2b569ae35dc846ec0f9e835c7aac`
- `{LANE}/tools/README.md`: `18df6cc3303b9494d0f1c95c741263e06de6aece6b3bb7666801ee6c56429c1a`
- `{LANE}/tools/STATUS.md`: removed, after the README was written.
- Under `{LANE}/probes/T1/`:

| File | SHA-256 |
|---|---|
| `self-test.txt` | `410a0e13…41c76` |
| `aa_10000_schematic_vsync.json` | `fb422286…da92` |
| `aa_10000_schematic_uncapped.json` | `00d89f29…45e0` |
| `aa_10000_actual-od_vsync.json` | `69df9cb8…43df` |
| `aa_10000_actual-od_uncapped.json` | `a90d85b8…13d8` |
| `aa_1000_schematic_vsync.json` | `91ed0255…477d` |
| `aa_1000_actual-od_vsync.json` | `50c500b0…0f27` |
| `resources_1000.json` | `9283be35…308b` |
| `resources_10000.json` | `e9843e61…942b` |
| `SUMMARY.md` | hash after my last edit not recomputed; re-hash before indexing |

Scratch only, outside the repository: a driver script, a path-free `measure_window.mjs`, the README draft, and the raw window-measurement output.

## Method
- **Opening a fixture:** the tool serves the build from its own no-cache static server on 127.0.0.1:5185. It intercepts the one request for the product's start-up model module and answers it with the fixture file, so the product's own load path opens it. It then waits for the fixture's project id, `committed` assignment and at least one render submission.
- **State:** set through the product's own controls: theme select, geometry buttons, labels toggle, then Isometric and Fit.
- **Settled (before and after each orbit):** for 500 ms the product reports `ownedPendingRafCount` 0, the submission and camera sequences stay static, and the main thread is under 20 % busy.
- **Press point:** the first candidate that `elementFromPoint` still resolves to the canvas after the pointer has rested on it. This is needed because hover labels take the pointer.
- **Orbit:** real mouse down, then a figure-of-eight path that depends only on elapsed time. The tool's own process sends moves every 4 ms and never awaits them.
- **Measures:**
  - Frame intervals come from `requestAnimationFrame` timestamps written into a preallocated array in the page.
  - Main-thread cost comes from `Performance.getMetrics` deltas over the measurement window.
  - The product's diagnostics are read before the orbit, at the end of the window, and after the orbit.
  - Console errors and warnings are captured.
  - Pacing is judged from a blank page that presents every frame.
- **`resources` settle check:** page-side wrappers count pending animation frames and WebGL draw and clear calls. A control observation made during an orbit must report "not settled", and it did.

## Commands and results
Every browser run went through `sh {RUN}/tools/with_e2e_lock.sh` on port 5185. Lock waits were normal; one hold waited about 9 min 45 s for another lane.

| Command | Result |
|---|---|
| `self-test --dist <copy>` | exit 0, PASS |
| six `run` series (`--variants "a=;b=" --pairs 3`, light, labels on, dpr 2, 1440x900) | exit 0 each, 36 of 36 runs without error, 0 console or page errors |
| `resources` at 1,000 and at 10,000 pipes | exit 0 each |
| one hold of `measure_window.mjs`, using the tool's exported pieces | exit 0 |
| `summarize` over the eight files | exit 0 |

- **Interruption:** my first attempt ran the holds from a background driver. It died after the first series when my turn ended, and the parent told me by message to continue in the foreground. No partial file was left, and no series was repeated.
- **Two timestamps:** the first series ran at 06:19Z and the rest between 08:37Z and 09:05Z.
- **Lock and port:** after every hold of mine the lock was released and 5185 was free. The lock directory seen at the final check belonged to another lane's process and was gone seconds later.

## A/A numbers and noise floor (b − a, three pairs)
| Series | Load (1 min) | Mean interval | p50 | p95 | Main thread per frame |
|---|---|---|---|---|---|
| 10,000 schematic, vsync | 4.2–5.3 | 0.000 ms | 0.00 | 0 to +0.10 ms | −0.233 to +0.012 ms (of ~2.5) |
| 10,000 actual OD, vsync | 3.8–4.7 | 0.000 ms | 0.00 | 0 to +0.10 ms | −0.072 to +0.022 ms (of ~3.1) |
| 1,000 schematic, vsync | 3.5–5.6 | 0.000 ms | 0.00 | 0 | −0.087 to −0.021 ms (of ~2.9) |
| 1,000 actual OD, vsync | 3.1–4.3 | 0.000 ms | 0.00 | −0.10 ms | −1.314 to −0.123 ms (one outlier run, see below) |
| 10,000 schematic, uncapped | 2.2–5.0 | −0.005 to +0.041 ms (−0.3 % to +2.1 % of 1.99) | −0.10 to 0 | 0 (3.4 ms in all runs) | −0.009 to +0.036 ms (of ~1.93) |
| 10,000 actual OD, uncapped | 4.1–6.4 | −0.025 to +0.021 ms (±1.3 % of 2.00) | 0.00 | 0 (3.4 ms in all runs) | −0.023 to +0.023 ms (of ~1.93) |

**Under vsync:**
- The interval measures are pinned to the refresh period. They can show only a missed refresh, and there were none in 24 runs.
- The "over 16.7 ms" figure of 27–33 % is rounding of 0.1 ms timestamp steps.
- The only sensitive measure is main-thread ms per frame, with a floor of about 0.25 ms (roughly 10 %).

**Under uncapped:**
- Mean interval and main-thread ms per frame both have a floor of about 0.04 ms (roughly 2 %).
- A difference of 0.1 ms per frame or more that holds in all three pairs would stand out. A difference under about 0.05 ms would not.
- p95 sat at 3.4 ms in all 12 runs, and the maximum ranged from 4.8 to 15.4 ms, so neither is usable for comparison.

**Uncapped pacing took effect headless** in all 12 runs. Calibration p50 was 0.1 ms against 16.6–16.7 ms under vsync, and the orbit mean was 1.97–2.03 ms.

## Window stand-in, re-measured on this build
- `1646x1168` still gives a canvas of exactly 1000 × 828 CSS px (828,000 px²).
- 1645 × 1167 gives 999 × 827, which falls short.
- The evidence series ran at 1440 × 900, where the canvas is **794 × 560** (444,640 px²).
- The layout around the canvas is a constant 646 × 340.
- The sealed brief's "794 × 580" was quoted at 1440 × 920, so the two figures are consistent.

## What the tool does not measure, and what read oddly
- **Main-thread-limited orbit:**
  - Under uncapped pacing the main thread is 96.6–96.8 % busy, with one product render per frame.
  - GPU cost that stays under that limit is invisible to the tool.
  - Schematic and actual OD gave identical intervals, 359 draw calls and 2,133,368 triangles. The actual-OD runs were confirmed in the diagnostics as `available`.
- **One outlier run** (1,000 pipes, actual OD, vsync, run 6) showed 1.395 ms per frame against 2.6–3.0 for its siblings. The tool's own pointer timer was held back in that run: 34 send gaps over 8 ms, the largest 51 ms. Fewer moves reached the page, so it did less work.
  - Main-thread cost therefore depends on the pointer delivery rate.
  - The tool records this but does not flag it.
  - The README tells readers to compare `pointer.sendGapsOver8Ms` across sibling runs.
- **Other limits:**
  - The browser is headless and the host is shared.
  - Today's layout is not the redesigned Model view.
  - The tool does not run the instrument or read its oracles.
- **Resources:** at both sizes, `live` was identical before and after all 17 steps and all 4 round trips (200 and 1,115 owned resources). `created − disposed = live` held throughout. The page was settled 1.5 s after the last input, and "not settled" during the orbit control.
- Nothing was needed from the product that it lacks, so I make no proposal.

## Closing checks
1. `node --check` passes.
2. The self-test through the lock exits 0.
3. After every hold of mine the lock was released and nothing listened on 5185.
4. The claims validator returns VALID (337 files scanned).
5. The scan for the home-directory prefix and `/private/` under both folders finds no match.
   - One false hit in my README's prose, which listed the scrubber's prefixes literally, was reworded and then rescanned clean.
6. `git status --short` shows only `tools/STATUS.md` (deleted), `tools/orbit_probe.mjs` (modified), `tools/README.md` and `probes/T1/`. I ran no state-changing git command and no build, and I never touched `{DESKTOP}/dist`.

This guides the lane and decides nothing about D-72. I claim no performance, usability or conformance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
