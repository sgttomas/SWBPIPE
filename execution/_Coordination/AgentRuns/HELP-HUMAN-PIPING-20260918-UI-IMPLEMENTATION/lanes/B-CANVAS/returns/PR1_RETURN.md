# Retained return — B-CANVAS-PR1 (the lane's first pull request candidate), 2026-09-19

Retained by ROOT. What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `475f546f279ee673dfe9cfebd65497ebec1931099a9e2eeef51d140925a5875c` (89 lines), by this script.

---

**Return B-CANVAS-PR1: the candidate of the lane's first pull request is frozen. It holds C1's first part with F1 to F3 disposed, T1, C1E and C1b. Every check is green on it. One thing is not what the brief pictured: the edge line's GPU cost cannot be seen by the probe on this host.**

I ran as Claude Fable 5.1 (`claude-fable-5-1`), by my own statement. Both children name themselves Claude Fable 5.1 in their returns.

## Candidate
- Commit `4e267d02bccd320607cf6ccab4c52f05cf4048b3` on `codex/swbpipe-b-canvas-20260918`. The worktree is clean and nothing is pushed.
- `origin/main` at `64f86e17f` was merged first, as `bafa0a067`, with no conflict.
- Diff range: `git diff origin/main...4e267d02b -- projects/chirality-piping/apps/desktop`. It shows 11 files, +2259 −355: ten under `src/features/viewport/`, plus `src/styles.css`.
- Nothing under `e2e/**` changed. `viewportSelectionPresentation.ts` (`00384d28…`) and `viewportSelection.ts` (`fdf3eaa4…`) are byte-identical to the lane base.

| Commit | What it is |
|---|---|
| `2f4208976` | F1, the earlier manager's |
| `5fad40f6e` | F2, the earlier manager's |
| `b415b43df` | F3: `opacity` is an own accessor over the uniform. Six tests failed first with "expected 1 to be 0.82" |
| `5bfdb9909` | the empty `layer.add` guard with its test. It already existed and was not redone |
| `442610cc7` | C1E, including F4 |
| `29d89b16d` | T1's tool, README and evidence |
| `fbdcc925e` | my measurement series for C1E |
| `58a4fefda` | C1b |
| `6dca626cf`, `9f4ddec92`, `4e267d02b` | records |

## Checks
All run by me from `{DESKTOP}` or `{WORKING_ROOT}`. The browser runs went through the lock.

| Command | Result |
|---|---|
| `npx vitest run src/features/viewport/viewportSelection.test.ts`, after the merge, at F3, and first and last on the C1b tree | 69 passed every time |
| `npx vitest run src/features/viewport` and `npx tsc -b`, after the merge | 180 passed; exit 0 |
| `npm run test:desktop` | 80 files, 1259 passed |
| `npm run build:desktop` | exit 0; `index.html` `5e994481…` |
| `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e` | 374 passed, 20 skipped, 0 failed |
| the same with `npm run test:e2e:dist` | 53 passed |
| `python3 tools/validation/validate_claims_language.py` | VALID |
| `python3 tools/practitioner_harness/harness.py self-check` | exit 0 |

- All 20 skips are the expected `D70_WRITER_*` skips.
- Both lanes ran once. The lock was released afterwards, and ports 5185 and 5186 were free. I never bound 5183 or 5184.
- **Edge-line cost, as guidance only.**
  - I ran 13 interleaved off/on series and 2 `resources` runs on the C1E build, before C1b. The record is `{LANE}/probes/C1E/MANAGER_SERIES.md`.
  - No on-minus-off difference falls outside T1's noise floor, and no refresh was missed.
  - Uncapped p95 is 3.4 to 3.5 ms at 10,000 pipes, against limits of 16.7 and 33.3 ms. The same holds on the 1000 × 828 stand-in canvas (window 1646 × 1168).
  - The ownership ledger is stable and the viewport settles.
- **What the series cannot show.** On this host the uncapped orbit is main-thread bound at about 2 ms a frame. The line adds only shader work, so its GPU cost under that bound is invisible to the probe. Only your qualification runs decide whether D-72 holds.

## Controls touched, contrast not yet checked
None. No HUD or canvas control changed.

## Appearance left for the closing pass
- **Derived silhouette shade.** The worst case is a node or rigid element in dark, `#3a3f45`, at 1.61:1 on `canvas.bg`. In light the same role is 4.66:1. A measured rim pixel agrees with the predicted value.
- **The line at device pixel ratio 1.** Its darkest pixel falls short of the token: `#575d63` to `#666b71` against `#42484f`. At ratio 2 a full pixel appears.
- **The line on a selected element.** `canvas.edge` over `canvas.selection` is 1.81:1 in light and 1.42:1 in dark, until C2's halo replaces the recolour. The rigid box's line on `canvas.pipeShade` is 2.55:1 and 4.35:1.
- **Roles the design does not name.**
  - A line on node spheres and on supports; none is drawn.
  - Creases on a box.
  - The open ends of a bend's arc, which get no line.
  - The selection cue's rim. I set it to `canvas.bg` on my own judgement.
- Still parked from before: the deformed shape's ink (C1-D1), `canvas.edgeAlt`, and the casing.

## Semantic changes, named
- **The figure gains an edge line.** Before, pipes and placeholders were a shaded fill only. Now a 1 CSS px band in `canvas.edge` lies inside the silhouette of pipes and of the four placeholder kinds, and a visible cap has a line at its rim. Outlines darken in light and lighten in dark, and tubes under two line widths wide draw as the line's colour alone.
- **Selected colour.** It moves from orange (`#a34400` light, `#f08c22` dark) to `canvas.selection` (`#106dce`, `#6dadff`), and it is still a recolour of the element. A routing draft and a selection now differ by form only.
- **Ground.** It moves from `#dfe5e8` / `#0c1114` to `canvas.bg`.
- **Canvas furniture.** The surface behind the canvas, the no-WebGL fallback and the length scale now follow the theme. Before, they were fixed light values.
- **The instrument's first profile.** From `58a4fefda` it no longer describes the lane's product, by design (addendum 2, ASK-11). Its files are untouched.
- C1's first-part changes stand as `C1A_RETURN.md` names them.

## What I bring back
1. **A stylesheet difference from P3's table.** The first `.viewport-shell` rule is shared with `.panel`, so editing it re-inked every panel. My new stylesheet test caught that, and I reverted the rule to byte-identical. The shell's `border-color` and `background` are now declared in its own later block. Every other granted row is as written. The comment line "Kept without a token…" moved down one sentence so that the comment stays true.
2. **Stage 3 of C1E, the side rings at free ends and at section changes, is not built.** Every pipe end in every model the product opens today lies inside a node sphere, so such a ring could not be seen. The design for it, the `pipeEndFacts` field it would add and the fixture counts are in `{LANE}/returns/C1E-EDGE_RETURN.md`. `modelIndex.ts` is untouched. One consequence: "diameter changes read as steps in the edge" is not yet visible anywhere, and it depends on C5's node geometry.
3. **What went wrong with the children.**
   - T1-ORBIT-PROBE-2 ran its series from a background driver and ended its turn to wait. The host killed the driver with the turn.
   - Because both foreground calls return together, C1E-EDGE waited its full ninety minutes and returned without measurements.
   - I resumed T1 by message with agent id `aba0e593bce0c1316`; it then finished cleanly.
   - I ran C1E's series myself, which was cheaper than resuming a 328k-token child.
   - The lesson is in the lane log: a child runs its lock holds as foreground commands and never ends its turn to wait.
4. **F3 and C1b were done by me, not by a child.** This rests on your launch brief's "small enough to do yourself" and departs from addendum 4 step 2, which named a new child for F3.
5. **Two things noticed in passing.**
   - A thin pale line under a pipe that lies on the ground plane. It is also in the base build and vanishes with the grid off, so it is not the lane's.
   - The lock has no queue, so a waiter can be passed over.

Returns are retained with their hashes in `{LANE}/briefs/_INDEX.md`. Screenshots are under `{LANE}/screenshots/C1E/` (40 pictures) and `{LANE}/screenshots/C1B/` (4 pictures, console empty).

## Next
On your go-ahead I start C2. It draws selection and hover as halos over the element's own colour, in new files, with `viewportSelectionPresentation.ts` left byte-identical. Hover goes behind a probe. It brings the pair-rule counts with no casing and the stencil options with their costs. No renderer-construction change comes back in a slice.

I claim no usability, conformance or performance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
