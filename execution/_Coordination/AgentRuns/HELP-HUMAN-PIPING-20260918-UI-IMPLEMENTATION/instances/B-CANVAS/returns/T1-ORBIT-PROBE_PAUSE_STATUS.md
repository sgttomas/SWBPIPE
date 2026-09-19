# T1-ORBIT-PROBE: pause status of 2026-09-19 (not the task's return)

Retained by B-CANVAS. What follows the rule is the child's final message as the host stored it; machine paths, if any, are replaced by placeholders (0 substitution(s)) and nothing else is changed. The message exactly as stored has SHA-256 `24ff782e465ce73560af8499bf0e4c22124dc22a7e3f983827aecbd476835829`.

---

Paused at a clean point: the tool is written and works, but no evidence has been produced yet. I hold no lock, nothing listens on port 5185, no process of mine is running, and I ran no state-changing git command.

I am T1-ORBIT-PROBE, running as Claude Fable 5.1 (`claude-fable-5-1`). Nothing was reverted or deleted.

**Written** (placeholders as in the sealed brief):
- `{LANE}/tools/orbit_probe.mjs` is whole at 1,560 lines and passes `node --check`. Its SHA-256 is `ac5290bcb93fba13f45c8f198b6cb13ee617a2b52766426cdc81f1d344598368`.
- `{LANE}/tools/STATUS.md` is a plain notice that the task is unfinished. It lists what works, what has been run, what is not done, and the next step. Its SHA-256 is `9c17065d0adabe29152a3beead255e7f8d11ae4d3ecb9f92eb16035aede1f545`. It is to be replaced by the README on resume.
- `{LANE}/probes/T1/` exists and is empty.
- The README and `SUMMARY.md` are not written yet.

**What works:** all four subcommands are in the tool (`run`, `resources`, `summarize`, `self-test`). Every browser run so far went through `{RUN}/tools/with_e2e_lock.sh` on port 5185, against the build copy you supplied. I ran no build and wrote nothing into `{DESKTOP}/dist`.
- **Self-test:** it passed once (exit 0), including the case of two copies of one build. I then made four small patches and have not re-run it: a blank-page pacing calibration, a histogram of pointer send gaps, the camera-sequence change in the `resources` orbit control, and a garbage collection in the tool's own process before each orbit.
- **Trial runs after the patches:** short `run` and `resources` trials ended without error after the first three patches. The garbage-collection patch has only been syntax-checked and imported.
- **All trial runs so far:** `run` at 1,000 and 10,000 pipes with both pacings, `resources` at 1,000 pipes, and two larger windows. Their output sits outside the repository and is not evidence.

**Per-variant build addition (2026-09-19):** it is in the tool and passed the self-test.
- **Syntax:** a variant may name its own build as `<name>@<distDir>=<query>`, and `--dist` is optional when every variant names one.
- **Serving:** each run is served only from that run's build.
- **Output:** each variant's build identity is recorded. `summarize` prints each identity once and says whether the variants share it.
- **Refusal:** a variant with no build, or a directory without `index.html`, is refused before any browser starts.

**Evidence series:** none is done. Still to run:
- the saved self-test output;
- four A/A series at 10,000 pipes (schematic and actual OD, each with `vsync` and `uncapped`);
- two `vsync` A/A series at 1,000 pipes;
- `resources` at 1,000 and at 10,000 pipes;
- `SUMMARY.md`.

Closing checks so far: the claims-language validator returns VALID with `STATUS.md` present (323 files scanned), and the scan for machine paths under `{LANE}/tools` and `{LANE}/probes/T1` finds no match. `git status --short` lists `{LANE}/tools/` as mine; the other entries (three viewport files, `briefs/_INDEX.md`, `drafts/`) are not mine and I did not touch them.

**Trial observations so far** (not evidence, recorded in `STATUS.md`):
- With labels on, the product's hover label takes the pointer, so a press at a fixed point after a hover never starts the orbit. The tool now looks for a press point the canvas still receives after the pointer has rested on it, and the self-test fails if the camera does not move.
- `--pacing uncapped` does take effect headless on this host. Mean orbit interval is about 1.4 ms at 1,000 pipes and about 2.0 ms at 10,000 pipes with actual OD. An idle page stays paced at about 16.7 ms, so the tool judges "took effect" from a blank page that presents every frame.
- Frame timestamps come in 0.1 ms steps. Under `vsync` the fraction of intervals over 16.7 ms is therefore mostly rounding.
- `--window 1646x1168` gives a canvas of exactly 1000 × 828 CSS px, the 828,000 px² stand-in.
- At 10,000 pipes uncapped, the orbit is limited by the main thread (about 95% busy). GPU cost under that limit is not visible to the tool.
- Nothing has been needed from the product that it lacks.

**Exact next step on resume:**
1. Re-run `self-test` through the lock and save its output under `{LANE}/probes/T1/`.
2. Run the six A/A series and the two `resources` runs, one lock hold each.
3. Write `SUMMARY.md`, including the supplied build's `index.html` hash and the 2026-09-19 addition.
4. Write the README, remove `STATUS.md`, run the brief's six checks, and return.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
