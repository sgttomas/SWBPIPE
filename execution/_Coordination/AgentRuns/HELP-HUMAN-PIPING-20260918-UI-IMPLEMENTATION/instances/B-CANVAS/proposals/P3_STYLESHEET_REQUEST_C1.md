# B-CANVAS change request P3: canvas furniture in `styles.css`, with slice C1

From B-CANVAS (WORKING_ITEMS, Type 1, canvas lane manager; Claude Fable 5.1) to ROOT, for the shell lane, 2026-09-19 (UTC). `src/styles.css` is the shell lane's file; the canvas lane has not edited it. Asked for by sealed addendum 1, "From slice B1", third bullet. Selectors and present values are as at the lane's head after the merge of `origin/main` at `8e4c5df6ec84928ca343224e5244ccdae5771cd4`; the shell lane's copy may have moved, so each row is given by selector and property, not by line.

## Requested now

| Selector | Property | Present value | Requested value | Why |
|---|---|---|---|---|
| `.viewport-shell` | `background` | `#fbfcfa` | `var(--ui-canvas)` | The surface behind the canvas shows while the canvas resizes or has not painted. It should be the scene's ground in both themes. `--ui-canvas` mirrors the scene background today and will be `var(--canvas-bg)` after C1b, so this one value is right before and after. Today it is a near-white flash in dark. |
| `.viewport-shell` | `border-color` (within `border: 1px solid #c8d0cb`) | `#c8d0cb` | `var(--ui-divider)` | Hard-coded light border. The workspace layout already removes this border (`.workspace-pane-viewport > .viewport-shell { border: 0 }`), so this matters only outside that layout. |
| `.viewport-scale-bar` | `color` | `#31403d` | `var(--canvas-label)` | The length scale ("1 m") is a plate over the canvas. The design pairs `canvas.label` on `canvas.labelBg` for plates over the canvas (design system §2.8). |
| `.viewport-scale-bar` | `background` | `rgb(255 255 255 / 80%)` | `var(--canvas-labelBg)` | In dark this is a white plate at 80 % on a near-black ground. |
| `.viewport-scale-bar` | `border-bottom-color` (within `border-bottom: 4px solid #31403d`) | `#31403d` | `var(--canvas-label)` | The bar itself, in the same ink as its figure. |
| `.viewport-fallback` | `color` | `#44504c` | `var(--ui-muted)` | Found beside the two the addendum names: the no-WebGL fallback is drawn in fixed light values. |
| `.viewport-fallback` | `background` | `#f4f6f4` | `var(--ui-canvas)` | As `.viewport-shell`. |

Contrast of the requested pairs, WCAG relative luminance, computed from `src/design/tokens.json` at `00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9`, translucent plates composited over the ground first:

| Pair | Light, on the held ground / on `canvas.bg` | Dark, on the held ground / on `canvas.bg` |
|---|---|---|
| `canvas.label` on `canvas.labelBg` (the scale bar's figure) | 14.41:1 / 14.65:1 | 13.88:1 / 13.58:1 |
| `canvas.label` on the ground (the bar) | 11.84:1 / 12.83:1 | 15.45:1 / 13.92:1 |
| `text.secondary` on the ground (the fallback's text) | 4.92:1 / 5.33:1 | 8.87:1 / 7.99:1 |

The design draws no length scale in the canvas; its "scale bar" is the result legend's. The product's length scale is kept as it is and only re-inked. Whether it stays is not this lane's to decide.

## Not requested yet, and when

- `--ui-canvas` and `--ui-viewport-selection-geometry` (the two mirrors): with C1b, as `var(--canvas-bg)` and `var(--canvas-selection)`, once ROOT has decided proposal P1. Neither variable has a consumer in `src/`; the benchmark instrument's camera preflight reads them back.
- Label plates (`.viewport-select-target`): their resting colours come today from the generic `.app-shell button` rule (`--ui-text` on `--ui-surface` with `--ui-border`), not from the plate's own rule, which that rule outranks. The per-kind border colours and the amber `.active` state (`#fff4df`, `#d57d16`, a light plate in both themes) are fixed values. The design's plate is `canvas.label` on `canvas.labelBg` with a `canvas.selection` edge when selected. That request comes with C2 (the selected edge) and C4 (the plates), not now.
- `.viewport-gizmo-host`: no colour of its own at this head; nothing to request.
- `.viewport-deformation-status.available`: with C5's HUD, as the addendum says.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
