# Retained return — lane B-CANVAS, slice C1 part A with proposals P1 to P3 (manager: Claude Fable 5.1 requested; its implementer child C1A-PALETTE reports itself as claude-fable-5-1)

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

Slice C1, first part (C1a), is committed on the lane branch with every check green on the merged head, and the two proposals you asked for are done. Branch `codex/swbpipe-b-canvas-20260918`, head `beb69d603d74fea326f64ab663e076bc2cd8ba9d`. Nothing is pushed.

**Routing.** C1A-PALETTE's completion notice reached me directly, because I was mid-turn when it finished, so I did not stop and wait for your relay. I extracted its final message byte-for-byte from the host's task record by script (size and hash only; the transcript was not read) and retained it:
- `{RUN}/instances/B-CANVAS/returns/C1A-PALETTE_RETURN.md`, SHA-256 `01f6199ccb97a9e94f33fcf2324c775cbf7a0ccd0ddb47ac7f5d878e6c3108cc`, 34,864 bytes, model `claude-fable-5-1`.
- If you extract it too and the hashes differ, yours governs.
- INV1's return is retained at `c2279008…`.

**Slice.** Every canvas colour except the two held families (scene background; selected colour with its cue rim) now reads a design token, with live repaint in both themes. The durable record is `{RUN}/instances/B-CANVAS/slices/C1_PART_A.md` (`6c11123c930873ccb4af6d8164a16055f52564f4dbd7b86460408cfdc2785441`).

**Commits.**
- `cbc538777`: returns retained, P1 and P2.
- `2dcf97757d9f320502d1f140ab2d7a4d6521753c`: the slice's code.
- `9e2fe2826dfe9506d7adc558e8edad658944e9ed`: merge of `origin/main` at `8e4c5df6e`, a merge commit with no conflict, taken with no child running.
- `beb69d603`: screenshots and records.

**Diff range.** `git diff 8e4c5df6ec84928ca343224e5244ccdae5771cd4...HEAD -- projects/chirality-piping/apps/desktop` shows eight files, +1144 −336, all under `src/features/viewport/`.

**Checks, all run by me at `9e2fe2826`.** The last three were re-run at the final head.

| Command | Result |
|---|---|
| `npx vitest run src/features/viewport/viewportSelection.test.ts` (picking, first and last) | 69 passed both times |
| `npm run test:desktop` | 78 files, 1216 passed |
| `npm run build:desktop` | exit 0 |
| `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e` | 374 passed, 20 skipped, 0 failed |
| the same with `npm run test:e2e:dist` | 53 passed |
| `python3 tools/validation/validate_claims_language.py` | VALID |
| `python3 tools/practitioner_harness/harness.py self-check` | exit 0, no finding on this lane's files |
| `python3 -m pytest -q tools/practitioner_harness` | 379 passed |

- All 20 skips are the expected `D70_WRITER_*` skips in `full-cohort-controller.spec.ts`.
- The lock was released and the ports were free after each browser run.
- No per-frame work was added. Draw calls, triangles and ledger counts are identical in all 17 looked-at states. Frame time was not measured.

**What you asked me to check.**
- The scene background and the selected colour are unchanged in both themes. Pixel samples show them exact, and the six held values sit in one marked block in `viewportResource.ts`.
- `viewportSelectionPresentation.ts` (`00384d28…`) and `viewportSelection.ts` (`fdf3eaa4…`) are byte-identical to the lane base.
- Nothing under `e2e/` is changed.
- One side effect on what the first profile sees, stated for your reviewer. A selected element's body now draws exactly the selected colour, where it used to draw darker through a lit material. In dark, `cat.1` arrows now fall within ±48 per channel of the selected colour. Neither affects the paired witness, which reads the unchanged diamond.

**Semantic changes, named.**
- Element kinds are no longer told apart by colour.
- The routing draft leaves the selection's colour for `canvas.draft`.
- Shading is matte and unlit, and the two scene lights are removed.
- The deformed overlay draws in neutral ink (`canvas.vector`) with no glow.
- Load arrows draw their token, and a selected arrow draws the selected colour.
- Grids draw the grid tokens at full opacity.
- A theme change repaints the model.
- The gizmo takes the axis tokens on `surface.raised`.

I changed one row of my own role table after the child returned: the deformed overlay moved from `canvas.deformGhost` to `canvas.vector`. The child measured `canvas.deformGhost` at the overlay's opacity as 2.29:1 light and 2.18:1 dark; `canvas.vector` is 4.70:1 and 8.10:1.

**Controls.** None touched, and no gap opened or closed.

**Screenshots.** `{RUN}/instances/B-CANVAS/screenshots/C1A/` holds 34 PNG files and a `README.md` index with hashes. It shows which six I re-took on the merged head.

**Brought back for your decision.**
- **P1, the second profile:** `{RUN}/instances/B-CANVAS/proposals/P1_SECOND_PROFILE.md` (`cd4650cb4a8c5d65b80bed8555b2349c1aa222938bb987aa09d1e696d7fd4626`), eleven asks.
  - It proposes a sibling directory, because `fixture-manifest.json` is bound by its own hash and cannot gain keys.
  - `full-cohort-controller.spec.ts:588` reads `viewportSelectionPresentation.ts` from disk in the ordinary source lane, so C2 must leave that file byte-identical.
  - The inventory misplaced the literal 34-file checks: they are in `characterization-observations.mjs:196` and `:278`, not in `resource-accounting.ts`.
- **P2, a slice for the edge line:** `{RUN}/instances/B-CANVAS/proposals/P2_EDGE_LINE_SLICE.md` (`abb5d7dbc681684bc27cc713ddf977ca05e2f041e1cbc20bc85811bc45ef0a61`), five asks. No slice of the brief draws the edge line, yet D-72 items 2 and 4 and the brief's C6 all assume it.
- **P3, stylesheet request for the shell lane:** `{RUN}/instances/B-CANVAS/proposals/P3_STYLESHEET_REQUEST_C1.md` (`0ecab4d2a6a04c9199ffd96fbbe76ba9e1b3991a67b90ebfe09fea5a19a14318`). It covers `.viewport-shell`, `.viewport-scale-bar` and `.viewport-fallback`. The two mirror variables follow with C1b.
- **A console error that predates the lane.** `replaceLayer` calls `layer.add(...objects)` with an empty list, twice per page load. The repair is one guard, `if (objects.length > 0)`. That code is retained foundation, so I propose the guard and have not made it.
- **`canvas.draft` equals `canvas.selection`.** Both tokens hold the same value in both themes (`#106dce`, `#6dadff`). After C1b the draft and a selection differ by form only, against specification §12 row 12. The token file is not this lane's.
- **Ground grid cadence.** The frames draw a two-weight ground grid, but the design states no interval for major lines.
- **Tube shading.** The frames draw tube shading as a flank band in non-token values. The slice follows design system §6.2 instead.

**What reads wrong after this slice.**
- In light, the gate theme, the figure is a 1.82:1 grey fill with no line, where it was a lit teal at about 6.5 to 7.5:1. It stays pale until the edge line exists. P2 ASK-5 asks whether the lane may reach `main` in that state.
- The light ground grid is nearly invisible on the held ground, as it was before.
- In dark, the supports are the loudest objects.
- The held selected colour and `cat.1` are close neighbours (1.61:1 light, 1.32:1 dark) until C1b moves selection to `canvas.selection`.

**Next.** C1b follows once you decide P1's ASK-1 to ASK-3 and ASK-11: the two held families move to tokens, and the two-line `styles.css` change comes back to you. After that, C1E or C2 in the order you choose under P2 ASK-2. If P1 waits, I can take C3 first; it changes neither held colour and leaves the selection presentation alone.

I claim no usability, conformance or performance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
