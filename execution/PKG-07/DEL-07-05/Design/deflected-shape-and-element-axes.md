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
- **A factor that cannot be drawn.** The viewport keeps geometry in 32-bit
  floats and multiplies it on the GPU. A factor is drawn only if every drawn
  offset is within 1e15 model length units and every drawn position is finite
  at 32 bits (`MAX_DRAWN_DEFLECTION_OFFSET`). A larger factor is not drawn: the
  default is drawn instead, or the normalized scale when there is no default,
  and the popover states why. The instance buffers then hold only finite
  values.
- **Statement.** The canvas, the popover summary and the popover explanation
  state the magnification drawn, which is the stated factor times the phase.
  At a partial phase the canvas and explanation name both factors, for example
  "Deformed ×25, 25% of ×100 · case"; the popover summary shows the drawn
  factor. At a partial phase only, the boundary string adds `phase=` and
  `drawn_factor=`; at phase 1 it states the factor alone. A normalized drawing
  names its basis: the chosen case, or "governing case per node". The readout
  keeps the emitted maximum with its unit at every factor and phase.
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
- **Kernel validation.** The kernel's validation now matches, in its order,
  with its arithmetic and its tolerance of 1e-12:
  - finite inputs;
  - the member axis normalized by `FrameElement::local_x_axis`, then
    normalized again by `FrameOrientation::from_x_axis_and_y_reference`. The
    second pass changes the last bits, and with them the orthogonality outcome
    for some nearly parallel references;
  - each normalization refused at a magnitude of 1e-12 or less;
  - then `FrameOrientation::new`: each axis of unit length, the three mutually
    orthogonal, and x × y = z, each to 1e-12.

  The checked-in table `apps/desktop/src/test-support/frameKernelOrientationCases.json`
  was produced by compiling the kernel on 48 invented members: m, mm, in and
  ft; normal, nearly parallel, zero, parallel, huge and small references; and
  lengths about the 1e-12 cutoff. `elementAxes.test.ts` shows the view
  converting to the kernel's metres, accepting and refusing exactly where the
  kernel does, and giving its axes to the bit.
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
  entity selected, the table shows no rows and says so. Turning it off removes
  the selection restriction; the other table filters still apply.

## Selection following (B8)

The tree, the grid and the inspector share one ordered selection
(`workspaceSession.ts`, `handleSelectEntity`). A selection made on one surface
is revealed on the others:
- the tree opens the selected row's group and makes it the current row;
- the node, material and section tables, and the bulk grid families, scroll
  to the selected row;
- the results table can show only the selected entity's rows.

DOM focus and the selection do not move. A reveal asked for while focus is
in that surface's list, or in an edit, is dropped: that surface keeps its own
position.

A reveal is kept while its surface cannot show it, and is done once it can.
This covers:
- the other layout;
- another grid family;
- a filter that hides the row;
- a pane that is hidden, inert or hidden by the stylesheet (the shell's
  inactive panes are `display: none`);
- a table that has not been opened yet: it opens on the selection.

One helper decides "can this surface reveal now" for all three surfaces:
`canSurfaceRevealNow` in `features/workspace/surfaceReveal.ts`. It passes a
surface that is attached, not under `[hidden]` or `[inert]`, and laid out
(`checkVisibility`, or client rectangles where that is absent). jsdom does no
layout, so `selectionReveal.test.tsx` injects a modelled visibility probe; the
browser behaviour is covered by the review's Chromium probe (#1667).

## Evidence

- `deflectedShape.test.ts`
- `PipeViewport.deflection.test.tsx`
- `elementAxes.test.ts`
- `PipeViewport.elementAxes.test.tsx`
- `selectionReveal.test.tsx` (selection following and the reveal rule)
- `viewportPalette.test.ts` (the three element-axis roles take `canvas.axisX/Y/Z`)
