# Sealed addendum 1 to brief B-CANVAS: what slice B1 found, and the instrument's visual cue

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19T00:04Z before the lane's launch. It is part of the lane's brief and binds as the brief does. Where it and the brief differ, this addendum governs. Path placeholders as in the brief.

## From slice B1 (merged)

Read `{RUN}/instances/B1-TOKENS/RETURN.md` and `CORRECTION_1_RETURN.md`.

- The product owns `apps/desktop/src/design/tokens.json` and generates `src/tokens.css` from it; both are the shell lane's paths and you never edit them. `tokens.json` can be imported from TypeScript (`resolveJsonModule` is on) for numeric colours; no consumer exists yet. If C1 needs a consumer module outside `features/viewport/**`, return the request to ROOT.
- Two workspace variables in `styles.css` were kept without a token because they mirror the viewport's constants: `--ui-canvas` (`viewportResource.ts` near 188 and 293: `0xdfe5e8` light, `0x0c1114` dark) and `--ui-viewport-selection-geometry` (`SELECTED_COLOR_LIGHT` `0xa34400`, `SELECTED_COLOR_DARK` `0xf08c22` near 218). `styles.css` is the shell lane's: when C1 or C2 changes what they mirror, return the two-line stylesheet change to ROOT with the slice.
- `.viewport-shell` (a hard-coded `#fbfcfa` behind the canvas) and `.viewport-scale-bar` (a white plate at 80 % opacity) live in `styles.css` and read wrong in dark. They are canvas furniture drawn by the stylesheet: propose their token values with C1; the shell lane applies them.
- `.viewport-deformation-status.available` is drawn with `--ui-accent`, which B1 mapped to the interaction blue. It is a results-availability cue and should not read as interaction. Propose its ink with C5's HUD.

## The benchmark instrument reads the selection cue

This is a constraint on C1 and C2 that the brief did not state.

- `e2e/ui-foundation/candidate-camera-preflight.benchmark.ts` near 103 reads back `--ui-viewport-selection-geometry` and `--ui-canvas` from the shell.
- `e2e/ui-foundation/freeze-candidate-point-oracle.mjs` near 72 asserts that the read-back selection colour equals the `viewportSelectionCue` of an externally frozen `VISUAL_TOKENS_V4.json` for the theme, and it binds the frozen sources `src/features/viewport/viewportSelectionPresentation.ts` (the cue) and `src/features/viewport/viewportSelection.ts` (the geometry). The README (near 92 and 138) describes the visual point oracle: a theme-specific token, a minimum interior-pixel count and a local check. Three benchmark files name "unchanged baseline PipeViewport selected material 0xf08c22" as their cue source.
- So the instrument detects "this element is selected" by pixels of the selected material's colour **inside** the element. C2 replaces that recolour with a halo over the element's own colour. Under C2 the present visual oracle cannot see a selection, and D-72's point-pick measurement cannot run on the redesigned canvas until the instrument has a cue definition for the halo.

What that means for the lane:

1. The existing frozen profile, its visual tokens, its oracle script and its recorded results are never edited. They remain the comparison basis (the recorded demonstration).
2. Before C1 changes either colour constant and before C2 changes `viewportSelectionPresentation.ts`, return to ROOT a proposal for the instrument's **second profile**: the visual tokens for the redesigned presentation (both themes), how a halo is detected (which pixels, which colour, what minimum count, what local check), what the oracle script binds, and how the second profile sits beside the first in `fixture-manifest.json` and the command files without touching the first. C4's label caps and D-72's window and canvas sizes belong to the same second profile; propose them together, once.
3. The proposal carries a score-independent rationale: the cue changes because the design changes selection's presentation by the owner's adopted design (G-31), not to obtain a result. It is frozen before any timed run uses it, and it is reviewed as instrument code by ROOT's independent reviewer.
4. Picking itself is untouched by any of this: `viewportSelection.ts`'s closest-point computation, its tolerances and its tests (PR #794) stay as they are. If the second profile needs the geometry source bound at a new hash only because presentation files around it moved, say so; do not change the geometry.
5. C1 can still make progress before the proposal is accepted: every canvas colour **other than** the scene background and the selected colour can move to tokens, with live repaint in both themes. Order your slices accordingly.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
