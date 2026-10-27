# Deflected-shape controls and element axes (A5, A7)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Both are
viewport displays of emitted result rows and of the model's own geometry. They
form no result, transform no result value and make no professional claim.
Every setting is session view state: it is not model data, not written to the
model document, not persisted in the UI preferences and not exported.

## Deflected shape (A5)

Controls sit in the viewport's deformation popover
(`DeflectedShapeControls.tsx`). The builder is `buildDeformationOverlay` in
`deformationOverlay.ts`.

- **Case.** The default is each node's governing displacement row over every
  case, drawn normalized: the largest at 0.65 model length units. This is the
  earlier drawing, unchanged. Choosing one load case or combination draws only
  that case's rows. Range-envelope rows are never drawn, because an envelope
  picks each component separately. The rotation glyphs read the same case.
- **Factor.** With one case, each node is drawn at its emitted displacement
  vector times the factor, converted to the model's length unit through the
  display conversion table (`services/unitConversion.ts`, `unitRatio`). The
  default factor draws the largest displacement at one tenth of the model's
  largest bounding-box side, rounded to the nearest 1, 2 or 5 (design system
  6.8). The steppers move through the 1-2-5 sequence. The field accepts any
  positive number: Enter commits it and Escape reverts it. A factor needs one
  case, complete `ux`/`uy`/`uz` rows and units that convert. Without them the
  drawing stays normalized, and the popover gives the reason.
- **Statement.** The canvas states the scale ("Deformed ×50 (default) ·
  case", or "normalized, not to scale"). The readout keeps the emitted
  maximum with its unit, whatever the factor.
- **Undeformed ghost.** The model layer is drawn at the dimmed opacity
  through the dim batches that isolation uses
  (`ViewportResource.setUndeformedGhostPresentation`), so nothing is created or
  disposed. The deformed shape keeps its opacity. Hide still wins, and dimmed
  geometry stays selectable. Design system 6.8's dashed 1 px outline in
  `canvas.deformGhost` needs the edge-line mechanism and is not built.
- **Animation.** The animation runs for one case only, and only while the user
  has asked for it. It oscillates the drawn phase from 0 (undeformed) to 1
  (the stated factor) in 24 steps over 2.4 s and stops at 1. The phase slider
  steps the phase by hand. A reduced-motion preference disables the
  animation. Each step rebuilds the result layer once, which is
  `deformationAtPhase` over the same offsets.

## Element axes (A7)

The viewport toolbar offers No axes, Local axes and Global axes
(`elementAxes.ts`, `viewportElementAxes.ts`). The triads are drawn in the
diagnostic layer, owned by their member. They hide and dim with the member,
but take no halo and no pick.

- **Local frame.** This is the solver's straight-member frame
  (`FrameOrientation::from_x_axis_and_y_reference`, used through
  `FrameElement::orientation` in `core/solver/frame_kernel`):
  - x = (to − from)/|to − from|;
  - y = the `y_reference` less its x part, normalized;
  - z = x × y.
  Each normalization is refused at 1e-12. A pipe without a `y_reference` has
  no triad, because no default orientation is applied. A span realized as a
  curved bend (a bend or elbow with `curved_bend_macro_element` and
  `bend_pipe_ref`) has no triad either, because its end rows are in
  end-tangent frames (DEL-04-01 `global-end-actions.md`). The status line
  counts such members and gives the reason for the selected one.
- **Global frame.** The global frame is the model's X, Y and Z at each member.
- **Evidence of the convention.** `elementAxes.test.ts` rotates the bundled
  invented reference's published element-local end rows with the drawn axes.
  It recovers the published global end rows of every member, case and end to
  1e-9 relative. With z = y × x the test fails.
- **Results table.** The table states each row's frame from
  `metadata.coordinate_system` (Element local, Global, Pipe section and the
  others the rows publish; "Not stated" when absent). A frame filter shows
  one frame's rows. Values are shown as published and are never re-projected:
  the global end rows (W-7) are the solver's own rows. The toggle therefore
  covers the display of the axes and the choice of published rows. It does
  not transform any value.

## Evidence

- `deflectedShape.test.ts`
- `PipeViewport.deflection.test.tsx`
- `elementAxes.test.ts`
- `PipeViewport.elementAxes.test.tsx`
- `viewportPalette.test.ts` (the three element-axis roles take `canvas.axisX/Y/Z`)
