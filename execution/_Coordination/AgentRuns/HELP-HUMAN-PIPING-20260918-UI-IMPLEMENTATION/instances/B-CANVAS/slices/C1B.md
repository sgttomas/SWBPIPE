# Slice C1b: the two held colour families move to tokens; the canvas furniture in the stylesheet

By the lane manager B-CANVAS (Claude Fable 5.1), 2026-09-19, done by the manager itself (small; the owner works under a usage limit). Authority: addendum 2 §1 ASK-11 and §3, addendum 5 item 2. Placeholders as in the lane brief.

## What changed

- `viewportResource.ts`: the block between `HELD-COLOURS:BEGIN` and `END` is gone. The scene's ground reads `canvas.bg`, the selected colour reads `canvas.selection`, and the selection cue's rim reads `canvas.bg` (the manager's judgement: the rim separates the cue from what it lies over; it was white in light and the old ground in dark). The gizmo's ground follows the scene's. No colour literal remains in any non-test file of the viewport folder.
- `styles.css`, exactly the granted exception: `.viewport-scale-bar` `color`, `background`, `border-bottom` colour; `.viewport-fallback` `color`, `background`; `.viewport-shell` `background` and border colour; the two mirrors `--ui-canvas: var(--canvas-bg)` and `--ui-viewport-selection-geometry: var(--canvas-selection)` with the dark block's two overrides removed; the comment's sentence about those two variables (and the line "Kept without a token…" moved below it so that the comment stays true). **One difference from P3's table:** the first `.viewport-shell` rule is shared with `.panel` (`.panel, .viewport-shell { … }`), so editing it would have re-inked every panel, which the grant does not cover. The two `.viewport-shell` values are therefore declared in the shell's own later block (`border-color`, `background`), and the shared rule is byte-identical.
- New test `viewportStylesheetFurniture.test.ts`: the three furniture selectors state no colour literal for ink, ground or border; each requested value is the token; each mirror is its token and is stated once. It was seen to fail when a mirror regained its literal.
- Tests moved, not weakened: `viewportResource.test.ts` pinned the old selected colour as display values in eight places (`0xa34400`, `0xf08c22`); they now read `canvas.selection` for the theme. `viewportColourLiterals.test.ts` pinned the held block's structure; it now asserts that no held block and no colour literal remain. No assertion of behaviour changed.

## Consequence, named (addendum 2, ASK-11)

From this commit the benchmark instrument's first profile no longer describes the lane's product, by design: its freeze step reads back `--ui-viewport-selection-geometry` and requires `#a34400` or `#f08c22`, and the product now answers `#106dce` or `#6dadff`. The first profile, its tokens, its oracle and its recorded results are untouched and remain the recorded demonstration. Nothing under `e2e/**` changed. `viewportSelectionPresentation.ts` and `viewportSelection.ts` are byte-identical.

## Semantic changes, named

- The selected colour changes from orange (`#a34400` light, `#f08c22` dark) to the design's selection blue (`#106dce`, `#6dadff`); the scene's ground from `#dfe5e8` and `#0c1114` to `#ebedef` and `#191c1f`. Selection is still a recolour of the element (G-31's halo is C2).
- A routing draft and a selection now share one colour and differ by form only, as design system §2.1 intends (addendum 2 §5).
- The surface behind the canvas, the no-WebGL fallback and the length scale follow the theme; before, they were fixed light values.

## Appearance left for the closing pass (numbers, not tuned)

- `canvas.selection` on `canvas.bg`: 4.36:1 light, 7.39:1 dark. On `canvas.pipe`: 2.21:1, 1.85:1. `canvas.edge` over a selected element: 1.81:1 light, 1.42:1 dark, so the edge line nearly disappears on a selected tube until C2's halo replaces the recolour.
- The cue rim in `canvas.bg` (the manager's judgement; the design names no rim).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
