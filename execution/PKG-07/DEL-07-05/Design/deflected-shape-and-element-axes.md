# Deflected-shape controls and element axes (A5, A7)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Both are
viewport displays of emitted result rows and of the model's own geometry. They
form no result, transform no result value and make no professional claim.
Every setting is session view state: it is not model data, not written to the
model document, not persisted in the UI preferences and not exported.

There are three separate controls:
- the deflected-shape controls, in the viewport's deformation popover;
- the element-axes toggle, in the viewport toolbar;
- the frame filter and the "Selected entity only" toggle, in the results
  table.

None of them sets another.

## Deflected shape (A5)

The controls are in `DeflectedShapeControls.tsx`. The builder is
`buildDeformationOverlay` in `deformationOverlay.ts`.

- **Case.** The default is each node's governing displacement row over every
  case, drawn normalized: the largest at 0.65 model length units. This is the
  earlier drawing, unchanged. It reads every displacement row, range-envelope
  rows included.

  Choosing one load case or combination draws only that case's rows, and then
  excludes range-envelope rows, because an envelope picks each component
  separately. The rotation glyphs read the same case. They exclude
  range-envelope components in every mode, as they did before.
- **Factor.** With one case, each node is drawn at its emitted displacement
  vector times the factor, converted to the model's length unit through the
  display conversion table (`services/unitConversion.ts`, `unitRatio`). A
  factor needs:
  - one case;
  - complete `ux`/`uy`/`uz` rows;
  - units that convert;
  - a nonzero displacement.

  Without these the drawing stays normalized, and the popover gives the
  reason.
- **Default factor.** With a positive, finite model extent, the default is
  the 1-2-5 value nearest, on a log scale, to drawing the largest displacement
  at one tenth of the model's largest bounding-box side (design system 6.8).
  Without a positive extent (for example a single node), or when the largest
  displacement is too small for the ratio to be finite, there is no default.
  The drawing then stays normalized and states that reason, and the field
  still takes a factor.
- **Entering a factor.** The steppers move through the 1-2-5 sequence. The
  field takes any positive number: Enter commits it and Escape reverts it.
- **A factor that cannot be drawn.** A finite factor can still make a node's
  offset or position non-finite. Such a factor is not drawn: the default is
  drawn instead, or the normalized scale when there is no default, and the
  popover states why.
- **Statement.** The canvas and the popover summary state the magnification
  drawn, which is the stated factor times the phase. At a partial phase both
  are named, for example "Deformed ×25, 25% of ×100 · case". The boundary
  string adds `phase=` and `drawn_factor=`. A normalized drawing names its
  basis: the chosen case, or "governing case per node". The readout keeps the
  emitted maximum with its unit at every factor and phase.
- **Undeformed ghost.** The model layer is drawn at the dimmed opacity
  through the dim batches that isolation uses
  (`ViewportResource.setUndeformedGhostPresentation`), so nothing is created or
  disposed. The deformed shape keeps its opacity. Hide still wins, and dimmed
  geometry stays selectable. Design system 6.8's dashed 1 px outline in
  `canvas.deformGhost` needs the edge-line mechanism and is not built.
- **Animation.** The animation runs for one case, with a factor, while the
  user has asked for it. It repeats 2.4 s cycles from 0 (undeformed) to 1 (the
  stated factor) and back, in 24 drawn steps, until stopped.
  - Stop returns the phase to 1.
  - Moving the phase slider stops the animation and leaves the phase where
    the slider puts it.
  - A reduced-motion preference disables the animation, and stops a running
    one at phase 1.
  - Each step rebuilds the result layer once (`deformationAtPhase` over the
    same offsets).

## Element axes (A7)

The viewport toolbar offers No axes, Local axes and Global axes
(`elementAxes.ts`, `viewportElementAxes.ts`). The triads are drawn in the
diagnostic layer, owned by their member. They hide and dim with the member,
but take no halo and no pick.

- **Local frame.** This is the solver's straight-member frame
  (`FrameElement::orientation` in `core/solver/frame_kernel`), formed on node
  coordinates converted to metres, as the product engine converts them before
  the kernel sees them:
  - x = (to − from)/|to − from|;
  - y = the `y_reference` less its x part, normalized;
  - z = x × y.
- **Kernel validation.** The kernel's validation now matches, with its
  arithmetic and its tolerance of 1e-12:
  - finite inputs;
  - each normalization refused at a magnitude of 1e-12 or less;
  - then `FrameOrientation::new`: each axis of unit length, the three mutually
    orthogonal, and x × y = z, each to 1e-12.
- **Withheld triads.** A member that would be refused gets no triad, and the
  reason is stated:
  - a zero-length member in metres;
  - no `y_reference` (no default orientation is applied);
  - a zero or parallel `y_reference`;
  - axes that are not orthonormal (a nearly parallel `y_reference`);
  - a model length unit that does not convert to metres here;
  - a span realized as a curved bend (a bend or elbow with
    `curved_bend_macro_element` and `bend_pipe_ref`), because its end rows are
    in end-tangent frames (DEL-04-01 `global-end-actions.md`).

  The status line counts the withheld members and gives the reason for the
  selected one.
- **Global frame.** The global frame is the model's X, Y and Z at each member.
- **Evidence of the convention.** `elementAxes.test.ts` rotates the bundled
  invented reference's published element-local end rows with the drawn axes.
  It recovers the published global end rows for every member, case and end. A
  component passes when |rotated − published| ≤ 1e-9 × max(1, the largest
  absolute published component of that vector). With z = y × x the test
  fails.
- **Results table.** The table states each row's frame from
  `metadata.coordinate_system` (Element local, Global, Pipe section and the
  others the rows publish; "Not stated" when absent). Its own frame filter
  shows one frame's rows. Values are shown as published and are never
  re-projected: the global end rows (W-7) are the solver's own rows.
- **Selected entity only.** This results-table toggle shows the rows stated
  for the primary selection, and follows it. With the toggle on and no model
  entity selected, the table shows no rows and says so. Turning it off shows
  every row.

## Evidence

- `deflectedShape.test.ts`
- `PipeViewport.deflection.test.tsx`
- `elementAxes.test.ts`
- `PipeViewport.elementAxes.test.tsx`
- `selectionReveal.test.tsx`
- `viewportPalette.test.ts` (the three element-axis roles take `canvas.axisX/Y/Z`)
