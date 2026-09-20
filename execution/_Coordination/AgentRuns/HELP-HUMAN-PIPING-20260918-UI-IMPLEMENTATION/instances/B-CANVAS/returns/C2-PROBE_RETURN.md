# C2-PROBE return, retained (model that ran: Claude Fable 5.1, claude-fable-5-1, by its own statement)

Retained by B-CANVAS (ROOT's script). What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `947d714fa0e99714f8e72cf2fef0d26bbcaa852d761fd209edac0b9f818f33a6` (255 lines), by this script.

---

# C2-PROBE return: the probe selects, hovers and counts halo pairs

I am Claude Fable 5.1 (`claude-fable-5-1`), TASK (Type 2), child of B-CANVAS. I worked alone, launched no agent and ran no git command that changes state.

The three additions and their self-test cases are in `orbit_probe.mjs` with the README, and the evidence is under `{LANE}/probes/C2-PROBE/`. The final self-test passed through the lock with exit 0. Two things bound how you can use it: the 10,000-pipe series ran under host load of 3.4 to 11.7 and resolve no difference, and `pairs` cannot yet say "wrong element selected", because the sample file's expected winners are for a camera 5.90 m from the fitted one.

I claim no performance, usability or conformance acceptance. Everything here is guidance for the lane and decides nothing about D-72.

## Hashes verified

- **Brief:** `f32d9618…bed16`, matched; read whole.
- **Tool at launch:** `59b18076…c7aac`, matched.
- **README at launch:** `18df6cc3…9c1a`, matched.
- **Build copy:** `index.html` `fc785c1b…8180c`, matched. I did not modify it and ran no build.

## What I read

**Read whole:**
- the brief
- `orbit_probe.mjs`
- the tools `README.md`
- `{RUN}/tools/with_e2e_lock.sh`
- `{DESKTOP}/src/features/workspace/uiDiagnostics.ts`
- `P1_SECOND_PROFILE.md` §4
- `{DESKTOP}/src/features/workspace/VirtualList.tsx`

**Read in part:**
- `PipeViewport.tsx` (controls, pointer handlers, hover state)
- `ModelTree.tsx` (row ids, range selection, group titles)
- `{LANE}/probes/T1/SUMMARY.md` (series settings and noise floor)

**Read as data:**
- `e2e/ui-foundation/samples/ui-foundation-{1000,10000}.interactions.json` and the `point-oracle-v3.json` files beside them
- `{DESKTOP}/src/design/tokens.json`

## Files (SHA-256)

- `{LANE}/tools/orbit_probe.mjs`: `4c6769381c8983ca739ff9d560221983a75407a91c75692b32fe84415f2bacc9`
- `{LANE}/tools/README.md`: `43960616586af02ef9967ad165c7116a9f683532614f1f85c58f8adef1bdde96`
- `{LANE}/probes/C2-PROBE/SUMMARY.md`: `0b903a1b70820fd604db9c49d1d6d73a9e554379ce2c0e56e1af34e65c2cb4fa`
- `…/self-test.txt`: `6d3f5629587a76796a286eea91b0815e48717aca650a639ac15dbf381966dcd2`
- `…/run_select-all_10000_schematic_uncapped.json`: `d95d488453f8189d8ad9c7fa033fa5038e2fb6c618333ad9b1047130934d9e31`
- `…/run_select-one_10000_schematic_uncapped.json`: `23482380a9b74f626486e7753b75096835e7fe6268d8d730f29e8561304f5984`
- `…/hover_10000_schematic_uncapped.json`: `751aa70c7ff72fd31cb549d4564c07e0c8b5fb92c77fdf261f0cc3acac24bb60`
- `…/pairs_1000_schematic_light_recolour.json`: `e4348cac68de4945fe4b20edbf14e5ea0f32016fcebea8d6da7f24c25cf61dae`

I touched nothing under `{DESKTOP}/**`.

## How each addition works

Nothing that existed changed meaning. Existing `run` command lines behave as before.

**Additive changes to existing output and functions:**
- `run` output gains a `selection` record.
- Each diagnostics reading gains a `selection` summary (count, primary, input kind).
- The orbit record gains `selectionUnchanged`.
- The recorded argv gains `--select none`.
- `driveOrbit` and `mainThreadCost` each gained one optional parameter with a default.

### `run --select <none|one|hundred|all>` (default `none`)

- **When it runs:** after Isometric, Fit and settling, and before the press-point search. The existing search therefore sees whatever the selection puts on the canvas.
- **How it selects:** through the model tree, with real Playwright pointer input.
  1. It clicks the header of each expanded group until none is open.
  2. It clicks `tree-group-Pipes` and checks that the group's `aria-setsize` equals the fixture's pipe count.
  3. It clicks pipe row 1. That is `one`.
  4. For `hundred` and `all` it turns the mouse wheel over the virtual list until row 100 or the last row is rendered, then Shift-clicks that row. This is the product's own range selection.
- **Why the tree:** the product has no select-all control and no accelerator for one. Box select depends on the fitted view and the selection filter, cannot give exactly 100, and would leave a tool armed over the canvas.
- **Which pipes:** `hundred` and `one` are the first pipes in the tree's order. They are not the instrument's `selected_sets`, which the tool does not read.
- **The check:** the run fails unless `viewport.selection.orderedRefs` reports exactly the wanted count, all of type `pipe`. It polls for up to 120 s.
- **What is written:** a selection of every pipe is never written out ref by ref.
- **Input:** no `dispatchEvent` and no call into the app.

### `hover` (options and variants of `run`, including `--select`)

- **The sweep:** the pointer rests at the canvas centre until settled, then sweeps with no button pressed. A move is sent every 4 ms from the tool's process.
- **The path** is a closed figure of eight about the canvas centre, a function of elapsed time alone, which crosses the fitted model and stays out of the corners:
  - x = 0.36 × W × sin(2πt / 5 s)
  - y = 0.30 × H × sin(2πt / 2.5 s)
- **Windows:** warm-up, measurement window, then a 150 ms tail. It records what `run` records, under `hover`.
- **Pointer events:** a passive capturing `pointermove` listener on the document writes each event's time and whether its target was the canvas into preallocated arrays. It is installed by an expression just before the sweep and removed at collection.
- **What comes from it:** the events dispatched in the window, the share the canvas received, and main-thread task and script ms per pointer event. This is per dispatched event, not per move sent; both counts are in the output.
- **The canvas check:** with `--labels off` the run fails if under 90 % of the events in the window reached the canvas. With labels on the share is reported.
- **Distinct hovered entities cannot be counted.** The diagnostics surface exposes no hovered entity, and the output says so. It records the product's main-render submissions in the span instead.
- **Unchanged state:** the run also records that the camera, model generation, geometry and selection did not change.

### `pairs`

This is a **simplified** form of P1 §4.4: no band predicted from display primitives and no exclusivity. The output, the summary and the README all say so.

**Fields read as data:**
- From `…interactions.json`: `point_selection[].sample` and `.probe_anchor_ref`.
- From `…point-oracle-v3.json`:
  - `probes[].sample`
  - `.probe_anchor_ref`, cross-checked against the first file
  - `.authored_anchor`, the sample's point
  - `.candidate_nominal_preflight_only.oracle.expectedHitRef`, the expected winner
  - `candidate_nominal_camera`
- Both files' hashes are recorded. No instrument script is imported or run.

**Per sample:**
1. The anchor is projected by the product's read-only `projectAuthoredPoint`, with the generation and camera sequence read at that moment.
2. A sample is recorded as not attempted, with the reason, when its anchor projects outside the canvas, its 48 px region leaves the canvas, or something other than the canvas receives the point.
3. **Before:** the tool clicks the project tree row, which is the product's state with nothing on the canvas selected. It then clicks a resting point on the canvas, found once, where a click selects nothing.
   - That click gives the canvas keyboard focus, so a focus ring cannot appear between captures.
   - The pointer rests there for both captures, so neither holds a hover.
   - After settling it takes a screenshot of the 48 × 48 CSS px region at `floor(point) − 24`.
4. **After:** the tool clicks at the point and waits up to 2 s for the diagnostics to report a selection other than the project. It moves the pointer back to rest, settles, and captures again. The camera sequence must not have moved.

**Counting** (pure functions; a small PNG decoder on `node:zlib`, no new dependency):
- Colours come from the token file's `color["canvas.selection"]` and `color["canvas.bg"]` for the theme. The file's hash is recorded.
- A pixel newly qualifies when it is within ±48 per channel of Selection after the pick and was not before.
- A pair is a newly qualifying pixel and a pixel within ±48 of `canvas.bg` after the pick, their centres at most 2 CSS px apart (2 × the pixel ratio).
  - The two must be at a WCAG contrast of at least 3:1, computed from their own colours.
  - Each ground pixel is used once.
- **Fixed order:** newly qualifying pixels row-major; for each, the nearest free ground pixel, ties by row then column. The matching is greedy, so it can undercount and never overcount.
- A sample passes with no casing when the image changed and at least 4 pairs exist.

**Telling failures apart:**
- `failureClass` is `no-cue-seen`, `no-ground-beside` or `too-few-pairs`. `pairsIgnoringContrast` says whether contrast was the limit.
- Counts reported beside it:
  - new pixels, split into those over ground before the pick and those over the figure before
  - inner pixels, where all neighbours within 1 CSS px are Selection-coloured
  - new pixels with ground in reach
  - ground pixels after the pick
  - Selection-coloured pixels before, after and lost
- Wrong-element information: the selected entity is reported beside the nominal expected ref and the anchor. Totals are given for selected = expected, = anchor, neither, and nothing selected.

**A recoloured body with ground beside it passes this rule.** The simplified rule has no predicted outline to be outside of, so it cannot refuse it.
- A recoloured body is told from a halo by the separate counts: 0 pixels over ground, all over the figure, and inner pixels where it is 3 px thick or more.
- A 2 px halo on open ground is the opposite.
- The first profile's diamond cue is Selection-coloured and is counted with everything else.

**Options:** `--note` labels the output, `--samples a-b` limits the samples, and `--dpr` defaults to 1.

### `self-test` and `summarize`

**`self-test` gains, without a browser:**
- Option parsing.
- The hover path: starts at the centre, closes, reaches its extreme, and sends a move at least every 8 ms.
- Contrast and hex arithmetic.
- A PNG encode and decode round trip.
- The image rule on synthetic images:
  - a ring on ground passes, with every new pixel over ground and none inner
  - a ring wholly over a tube-coloured field fails as `no-ground-beside`
  - a recoloured body with ground beside it passes, and is told apart by inner pixels and by lying over the figure
  - an unchanged image fails as `no-cue-seen`
  - a ring in the hover colour fails as `no-cue-seen`
  - a ring at pixel ratio 2 passes
  - each ground pixel is used once
  - a pair under 3:1 is not counted
- The `pairs` totals.

**`self-test` gains, in the browser on the 1,000-pipe fixture:** `run --select hundred`, `run --select all`, `hover --select one` and `pairs --samples 1-4`. Each is checked for shape, for the reported count being the count asked for, and for no machine path in the text. The orbit cases are checked for the orbit moving the camera, and the hover case for the camera staying put and the canvas receiving the sweep.

**`summarize`** gains two tables, "Selection and hover" and "Halo pairs", and changes none. Hover series also appear in the existing tables.

## Commands and results

Every browser command ran through `sh {RUN}/tools/with_e2e_lock.sh`, on port 5185, in the foreground. After each hold port 5185 was free and the lock was released. The lock shown as held at the end belongs to another process, not mine.

1. `shasum -a 256` on the brief, tool, README and build `index.html`: all matched.
2. One exploration hold with a scratch script outside the repository, run twice: read the tree structure and the fitted camera.
3. `self-test`, first attempt: exit 1. My token lookup was wrong (the token file keys colours by dotted name). I fixed the lookup and one quoting error.
4. `self-test`, second attempt: exit 0, PASS.
5. `run --select all`, then `run --select one`, first attempt: exit 2 before any browser started. My shell variable was not word-split under zsh. I reran through a small `sh` script.
6. The evidence series, all 10,000 pipes, centreline mode, light, pixel ratio 2, 1440x900, uncapped, `--variants "a=;b=" --pairs 1 --warmup-ms 2000 --measure-ms 6000`:
   - `run --select all`, labels on: exit 0.
   - `run --select one`, labels on: exit 0.
   - `hover`, labels off, nothing selected: exit 0.
7. `pairs --pipes 1000 --mode schematic --theme light --note "…RECOLOUR…, not a halo"`: exit 0, about 7 minutes.
8. One wording fix in `summarize`, then the final `self-test` through the lock: exit 0, PASS. It is saved as `self-test.txt` and covers the final tool bytes.
9. `node --check` on the tool: OK.
10. `python3 tools/validation/validate_claims_language.py`: VALID, 338 files scanned.
11. `grep` for home-directory and `/private/` text under my two folders: none found.
12. `git status --short` shows:
    - mine: the tool, the README and `probes/C2-PROBE/`
    - not mine: C2-HALO's product files and the manager's `LANE_LOG.md`

## Evidence numbers

All on the recolour build. No halo was measured. Uncapped pacing took effect in all six timed runs. No console error, warning or page error occurred in any run.

**`run --select` (10,000 pipes):**

| Series | Run | Selected (reported) | Mean interval (ms) | Main thread (ms/frame) | p95 (ms) | 1-min load |
|---|---|---|---|---|---|---|
| `--select all` | a | 10,000 | 2.467 | 2.236 | 3.8 | 3.4 to 5.0 |
| `--select all` | b | 10,000 | 2.228 | 2.044 | 3.2 | 3.4 to 5.0 |
| `--select one` | a | 1 | 2.391 | 2.221 | 3.3 | 4.8 to 11.5 |
| `--select one` | b | 1 | 3.430 | 3.187 | 4.3 | 4.8 to 11.5 |

- Draw calls were 361 in all four runs, against 359 in the hover series with nothing selected on the canvas. Triangles were 2,133,368.
- The primary was `pipe:UIF-10000` for `all` and `pipe:UIF-00001` for `one`.
- The selection was unchanged through the orbit, which started from press candidate 1 every time.
- Selecting took about 1.1 s for `all` and 0.8 s for `one`. That is the tool's own time, not a product measure.
- Run b of `--select one` had 142 send gaps over 8 ms against 0 in run a. The host held the tool's process back, so its 3.43 ms is not the product.
- **The A/A differences, 0.24 and 1.04 ms, are about six and 25 times T1's floor of about 0.04 ms.** One pair under this load resolves nothing between `one` and `all`.

**`hover` (10,000 pipes, labels off, load about 11.5):**

| Run | Mean interval (ms) | p95 (ms) | Main thread busy | Pointer events in window | Main thread per event (ms) | Script per event (ms) |
|---|---|---|---|---|---|---|
| a | 6.44 | 11.9 | 99.6 % | 1,070 | 5.58 | 5.36 |
| b | 5.07 | 6.4 | 99.1 % | 1,212 | 4.90 | 4.64 |

- All events in the window reached the canvas.
- The camera sequence change was 0. The product drew 1,210 and 1,620 times in the span.
- On this build, hovering costs more than twice an orbit frame per pointer event, because the product picks on every move and redraws.
- The A/A difference is 1.36 ms (21 %) under that load.

**`pairs` (1,000 pipes, light, centreline mode, pixel ratio 1, canvas 794 × 560, recolour build):**
- 200 samples measured; **199 pass the simplified rule with no casing**; none were not attempted or failed outright.
- The one failure is sample 60, `too-few-pairs`: 17 new pixels, 1 pair.
- Medians: 28 new pixels (17 to 84), 21 pairs (1 to 43), 6 inner pixels (0 to 17).
- The share of new pixels over the figure was 100 %. At most 1 pixel in any sample lay over ground before the pick.
- Contrast refused no pair.
- This is the recoloured body and the diamond cue passing, as documented. Read a halo build's numbers against it.
- **The expected winners do not apply at the fitted camera:**
  - The camera read back is at (16.23, 11.81, 11.18); the nominal camera is at (12.4, 8.1, 13.7). They are 5.90 m apart.
  - Selected = nominal expected in 19 samples, = the anchor in 3, neither in 180, nothing selected in 0.
  - Every click selected exactly one entity.

**What reads oddly:**
- The host was loaded throughout: C2-HALO was building and testing beside me.
- For sizing a halo's cost the manager needs three pairs on a quieter host, as T1 did. The brief allowed one short pair per series and no more, and I ran no more.
- In the hover series "Selected" shows 1. That is the product's start-up project selection.
- Under uncapped pacing nearly every move sent arrived as its own pointer event.

## What the additions do not measure

- `--select` measures the selected state, not the act of selecting.
- The selected set is the tree's first pipes, which are neighbours in the model. A hundred pipes spread across the model may cost differently.
- `hover` cost per event divides all main-thread work in the window by the events, so it includes the frames the hover caused and is not the cost of the pick alone. It gives no count of hovered entities.
- `pairs` is not the second profile's witness:
  - no predicted band, no exclusivity, no negative control for a neighbour's halo
  - a halo on the wrong element passes it
  - the diamond cue is counted with everything else
  - the matching is greedy
  - it captures a headless screenshot at the pixel ratio asked for
  - it says nothing about whether a user can see the cue
- GPU cost under the main-thread limit remains invisible to the tool, as before.

## Needed from the product and not there (proposals, not changes)

1. **A hovered entity on the diagnostics surface.** The `UiDiagnosticsSnapshot` viewport object in `uiDiagnostics.ts` would gain `hover: Readonly<{ ref: FrozenRef | null; changeSequence: number }>`.
   - `ref` would come from the viewport's hovered entity key.
   - `changeSequence` would be incremented whenever that key changes.
   - It would be published lazily with the rest.
   - I did not touch that file.
2. **Expected winners computed for the camera the product actually fits at the probe's canvas size.** That list is the instrument's, and ROOT's to make. Until it exists, `pairs` reports both refs and the camera distance and cannot say "wrong element selected" outright.

Semantic changes to the product: none. Controls touched: none, so none is contrast-unchecked.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
