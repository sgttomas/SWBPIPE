# Retained return — B-CANVAS-C2 (selection and hover as halos), 2026-09-19

Retained by ROOT. What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `81a27dd5d75de6e60bc90860d7113a18dedf9e4e19f151cea54407500adfbd6a` (119 lines), by this script.

---

**Return B-CANVAS-C2: slice C2's candidate is frozen. Selection and hover are halos over the element's own colour, the renderer's construction is unchanged, and every check is green on the merged head. One result corrects my own first reading: drawn alone with no casing, the halo fails the pair rule in most samples.**

I ran as Claude Fable 5.1 (`claude-fable-5-1`), by my own statement. Both children name themselves Claude Fable 5.1 in their returns. The session limit cut both off once, and I resumed them by message with their agent ids.

## Candidate
- Commit `0758e91e378dcbf2927bd787194248a8a23980f4` on `codex/swbpipe-b-canvas-20260918`. The worktree is clean and nothing is pushed.
- `origin/main` was merged twice at clean points with no child writing and no conflict:
  - `172cd5689` as `732ce98aa`;
  - `d20eb1294` (PR #806) as `441a65b8b`.
- The build's `index.html` after the second merge equals the pre-merge working copy, so the merge moved no product byte.
- Diff range: `git diff origin/main...0758e91e3 -- projects/chirality-piping/apps/desktop`. It shows 6 files, +1260 −36, all under `src/features/viewport/`.
- Nothing under `e2e/**` changed. `viewportSelectionPresentation.ts` (`00384d28…`) and `viewportSelection.ts` (`fdf3eaa4…`) are byte-identical to the lane base.

| Commit | What it is |
|---|---|
| `d3bd50d83` | the product: new `viewportHalo.ts`, with its test file of 24 tests |
| `afc84e916` | the probe gains `run --select`, `hover` and `pairs` |
| `2796a18af`, `7ceb3a244`, `0758e91e3` | records |

## Checks
All run by me on the merged head. The browser lanes went through the lock.

| Command | Result |
|---|---|
| `npx vitest run src/features/viewport/viewportSelection.test.ts` | 69 passed, first and last |
| `npm run test:desktop` | 82 files, 1297 passed |
| `npm run build:desktop` | exit 0; `index.html` `e9aca49e…` |
| `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e` | 374 passed, 20 skipped, 0 failed |
| the same with `npm run test:e2e:dist` | 53 passed |
| `python3 tools/validation/validate_claims_language.py` | VALID |
| `python3 tools/practitioner_harness/harness.py self-check` | exit 0 |

- All 20 skips are the expected `D70_WRITER_*` skips.
- The lock was released afterwards, and ports 5185 and 5186 were free. I never bound 5183 or 5184.
- The picking test was also 69 after each merge.
- After the first merge the viewport folder was 215 passed and `npx tsc -b` was clean. After the second merge they were 239 passed and clean.

## Stencil options with costs
The full record is `{LANE}/probes/C2/MANAGER_SERIES.md`. The timed figures are with all 10,000 pipes selected, the case the instrument's manifest requires.

| | (a) depth-tested shell | (b) depth mask, built | (c) stencil, patch only |
|---|---|---|---|
| Renderer construction | unchanged | unchanged | `stencil: true` |
| When the element is covered | halo hidden; a centreline pipe inside its node spheres shows almost none | shows over nearer geometry | the same picture as (b) |
| Mean frame, uncapped, base 2.2 to 2.3 ms | not built | +0.45 to +0.66 ms | +2.0 to +2.4 ms |
| p95, against limits of 16.7 and 33.3 ms | not built | 3.7 to 4.0 ms | 6.3 to 7.8 ms |
| Draw calls, triangles | not built | 361 to 363; 2.13 M to 3.13 M | the same |
| Memory | 1 instance buffer per shape | 2 buffers; 2.1 MB at 10,000 pipes | the same, plus 8 stencil bits per sample |
| Liability | per-shape corner rules, z-fighting | near-plane depth left in masked pixels | changes the renderer's construction |

- Pictures of (b) and (c) are equal pixel for pixel in 18 fixture pictures, by C2-HALO's comparison.
- (b)'s leftover depth matters only to something drawn later with a depth test. Nothing is today: the first profile's cue has the test off, and the gizmo clears depth in its scissor.
- The differences hold in every pair and are far outside T1's noise floor of about 0.04 ms.
- No refresh is missed by any form on the 1000 × 828 stand-in canvas.
- With one pipe selected, or none, there is no difference. An unselected scene draws exactly the base's 359 calls.
- The ledger is stable and the viewport settles.
- **My recommendation: keep (b).** It draws the same picture at about a quarter of (c)'s cost on this host and changes no foundation. The patch is retained as `{LANE}/probes/C2/stencil_candidate.patch`, in case a later slice must draw depth-tested geometry after the halo.
- **Hover.** The halo adds nothing measurable. Both builds spend 4.26 ms of main thread per pointer event. That 4.3 ms per move at 10,000 pipes is today's product: it picks and redraws on every pointer move, and did so before this slice.
- These numbers guide the lane. Only your qualification runs decide D-72.

## The casing (P1 ASK-4): numbers for the owner
- **Halo alone, no casing.** The probe's simplified pair rule was run in centreline mode at the fitted camera at device pixel ratio 1.

  | Fixture | Light | Dark |
  |---|---|---|
  | 1,000 pipes | 42 of 200 | 43 of 200 |
  | 10,000 pipes | 1 of 200 | 1 of 200 |

- The halo was seen in all 800 samples. What fails is ground beside it: in these dense fixtures the halo lies over other tubes and spheres, where it is 2.21:1 in light and 1.85:1 in dark. P1 §4.5 predicted this.
- **The mistake I caught before reporting.** The product's own build passes 199 to 200 of 200. That is only because the first profile's diamond cue, whose rim is `canvas.bg`, satisfies the rule alone.
  - I made a probe-only build copy with the diamond hidden, outside the tree, and restored the tree by hash.
  - My first copy did not hide the diamond. It showed as results identical to the product's, so I deleted those runs and redid the copy.
- The lane draws no casing, lowers no 3:1 and drops no pair check.

## Controls touched, contrast not yet checked
None. The label plate's `canvas.selection` edge belongs in `styles.css`, the shell lane's file. The request is: `.viewport-select-target.active { border-color: var(--canvas-selection); }`, keeping the rule's border width.

## Appearance left for the closing pass
- **The halo's contrast.** It is 2.21:1 in light and 1.85:1 in dark on a tube, 1.81:1 and 1.42:1 on the edge line, and 4.36:1 and 7.39:1 on the ground.
- **Hover.** It is 2.67:1 in light on the ground and 1.64:1 against selection, so the two read apart mainly by width.
- **Halo coverage at device pixel ratio 1.** A diagonal 2 px halo has one to two pixels within ±48 of the token, with a median of 1.41 CSS px. Its rounded caps come from `discard` and are not multisampled.
- **The diamond cue beside the halo.** It is the stronger mark. With everything selected, 20,000 diamonds bury the figure.
- The casing's look also waits for the closing pass; its decision is the owner's.

## Semantic changes, named
1. **Selection.** Before, a selected element's colour was overwritten with `canvas.selection`. Now it keeps its own colour and edge line and gains a 2 px outline.
2. **Hover.** Before, hover showed only as a label. Now an unselected hovered element gains a 1 px `canvas.hover` outline, including hover that arrives from a label. A selected or hidden element shows none.
3. **Covered elements.** Before, a recoloured tube inside its node spheres was hidden there. Now the halo shows through nearer geometry.
4. **Overlaps.** Overlapping haloed elements are outlined as their union.
5. **Load arrows and the deformed overlay.** They no longer change colour when their owner is selected, and they get no halo.
6. **Hidden and selected.** A hidden selected element has no halo.

## What I bring back
1. **The choice between (b) and (c).** It is yours, under the owner's rule. I recommend (b).
2. **P4: `{LANE}/proposals/P4_FIRST_PROFILE_REPAIR.md` (`c7acfb60…`), three asks.**
   - The repair of the first profile's four defects becomes its own slice, I1, after C2's pull request and before C3, with its own review as instrument code.
   - Its write scope is four files under `e2e/ui-foundation/` plus one new record. No fixture, oracle, tolerance or limit is touched.
   - Defect 1 is a choice, because after C1b the first profile cannot pass its visual oracle on today's product whatever geometry hash it binds. I recommend making the first profile internally consistent and honest about the historical revision it binds. I do not recommend rebinding it to today's source.
3. **For the second profile's freeze package.**
   - Only the halo's outer pixel lies in P1 §4.3's band of 1 to 3 CSS px.
   - Halo pixels appear over nearer geometry.
   - The diamond still puts Selection-coloured pixels at the winner's centre.
   - The sample files' expected winners are for a camera 5.90 m from the one the product fits. At this canvas size only 18 or 19 of 200 clicks select the nominal expected element.
   - The diagnostics surface exposes no hovered entity. C2-PROBE's proposed field `hover: { ref, changeSequence }` would go in `uiDiagnostics.ts`, the shell lane's file.
4. **Carried to C3.** `applySelectionPresentation` still repaints every instance's base colour on each selection change. That is now needless, though it costs what it cost before. C3 reworks that path for dimming.
5. **Noted for later slices, not acted on.**
   - The review's three residual risks.
   - The shell's `--shell-canvas-foot-inset`.
   - "Save local" being hidden in the new shell; the affected file is `resource-accounting.ts`.
   - C1E's side rings, which join C5.

Returns are retained with their hashes in `{LANE}/briefs/_INDEX.md`. The slice record is `{LANE}/slices/C2.md`. Screenshots are under `{LANE}/screenshots/C2/` (71 pictures, console empty in every look).

## Next
On your go-ahead I start I1, if you take P4's order. Otherwise I start C3: isolate as dimming, with a stated pick rule for dimmed elements and a hidden count that counts everything hidden. With C3's return I will name what C4 needs from the shell lane.

I claim no usability, conformance or performance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
