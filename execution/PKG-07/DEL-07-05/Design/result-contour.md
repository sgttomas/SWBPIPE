# Result contour along members, with legend and threshold (A4)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). The
contour is a viewport display of emitted result rows and of the session's own
rule-check outcomes. It forms no result and no engineering quantity, sets no
limit, forms no status and makes no professional claim. Every setting is
session view state. Settings are not model data. They are not written to the
model document, not persisted in the UI preferences and not exported.

Survey item A4 (`execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md`)
was adopted under the owner's 2026-10-27 delegation (`loop/LOOP_INIT.md`). It
extends the existing result colour (OD-5, FR-GUI-001, PRD §14.2 "stress/result
overlays"). It does not replace it. The sources are:

- `apps/desktop/src/features/viewport/resultColourOverlay.ts`: quantities, the
  station grid, the overlay builder and the member colour table.
- `resultContour.ts`: the interpolation, the contour rings, the numbers and
  the threshold.
- `resultContourMesh.ts`: the sleeves.
- `ResultOverlayLegend.tsx`: the controls and the legend.
- `ResultValueLabels.tsx`: the numbers display.
- `viewportResource.ts`: the theme repaint and the overlay label slot.
- `PipeViewport.tsx`: the wiring.

## 1. Controls

Result colour keeps its toolbar toggle and its case and quantity selectors.
The legend adds three controls. Each is a native, labelled control, so it can
be reached and operated from the keyboard.

- **Along** (`Result colour resolution`). The choices are "One value per
  member (largest)", the default, and "Along member (stated stations)".
- **Show** (`Result overlay display`). The choices are "Colours", the default,
  and "Numbers in place of colours".
- **Only above** (a checkbox) with a threshold field. The field is labelled
  "Threshold, in the legend's unit" and is enabled only while the checkbox is
  set.

One value per member stays the default because it adds no geometry (§9).
Along member is one selection away.

## 2. Quantities

Only existing result rows and outcomes are contoured. No code data ships, and
nothing is computed from other rows.

- **Emitted row kinds.** These are the `RESULT_COLOUR_QUANTITIES` that the
  result carries, unchanged:
  - displacement magnitude;
  - the normal-stress maximum and the section normal-stress extreme;
  - the intensified bending, transverse-shear and torsional-shear stresses;
  - the local axial and shear forces and the local moments.

  Each is shown as the magnitude of its rows, as before.
- **User-rule ratios.** One quantity is offered per stress category of the
  user's rule pack. It is offered only when the session has a rule-check run
  of this result and some of the category's outcomes are located on a single
  pipe of the model. App passes the run only while the shown result is the
  current solved result, the same rule the stress and ratio table uses.

  The ratio is the table's own (`buildStressRatioTable`, Design
  `stress-ratio-table.md` §3.1): computed ÷ limit, or limit ÷ computed under a
  lower bound. The viewport forms no ratio of its own. The category name and
  id come from the pack; the product supplies none. A ratio is a pure number,
  shown without a unit and never converted.

  An outcome is placed on a member when every one of its locations names that
  one pipe. Its station is placed when every location states the same station.
  An outcome located otherwise, for example on a node, a component or two
  pipes, is not drawn.

Range-envelope rows and range-envelope combinations are never drawn, as
before. Rows of one quantity and case in more than one unit are not compared,
as before.

## 3. Values along a member

**The station grid.** A member's stations come from the result itself: the
`metadata.location` token of each row. The product's fixed station grid gives
each token a fraction of the member, measured from its From node:

| Token | Fraction |
|---|---|
| `end_i` | 0 |
| `quarter_1` | 0.25 |
| `midspan` | 0.5 |
| `quarter_3` | 0.75 |
| `end_j` | 1 |

These are the fractions the solve routes emit at (`RESULT_STATION_FRACTIONS`).
Any other location states no position: a governing station, a node or a
summary. A nodal quantity (displacement magnitude) is placed at the member's
ends, its From node at 0 and its To node at 1. A component's intensified row
takes its own station token only when its source rows reach a single member.

**A station's value** is the largest magnitude stated at that fraction for the
case. A station has no value in two cases:

- the numerical standing withholds any row stated there (§6);
- any row stated there is not finite.

**A member with two or more stated stations** is drawn from those stations
only. Its legend value is the largest of its stations' values. Rows of that
member that state no station are not drawn, and the legend counts them. The
member's own tube beneath the sleeve takes the no-value colour along its whole
length. Where the first stated station is after the From node, or the last is
before the To node, the exposed end of the tube therefore shows no result
colour.

**A member with fewer than two stated stations** is shown as one value: the
largest magnitude over its rows, the same rule as one value per member, over
the member's whole length. This is the one-value display, not an extrapolation
of stations, and the legend says how many members are shown this way. The
rule-check outcomes follow the same two rules.

**A station's sign.** For a signed quantity (the table below), a station has a
sign when every row stated there has the same sign: 0 when all are zero, and
none when they differ. A station also records the sign convention its rows
state (`metadata.sign_convention`), when they all state the same one. The row
that gives a station's value is kept, and that row is the one each number's
tooltip names.

| Quantity | Rows | Rule along members |
|---|---|---|
| Axial force, shear forces y and z, torsional moment, bending moments y and z | One signed value per station | Signed |
| Torsional shear stress | One signed value per station | Signed |
| Displacement magnitude | Unsigned magnitude | Magnitude |
| Normal-stress maximum | Non-negative maximum | Magnitude |
| Section normal-stress extreme | σ+ and σ−; the larger magnitude governs, and the governing component may switch | Magnitude |
| Intensified bending stress | Non-negative | Magnitude |
| Transverse shear stress maximum | Non-negative | Magnitude |
| User-rule ratio | A ratio, never signed | Magnitude |

The emitted rows state the end stations (`end_i`, `end_j`) under the element's
end-force conventions. They state the interior stations under the section
convention (the j-side section action). Signs are compared only between
adjacent stations that state the same convention, so an end station's sign is
not compared with an interior station's. No sign is ever inferred.

**Interpolation.** `interpolateStations` is the one pure, tested function the
contour uses.

- Between two adjacent stated stations, the value is linear in the fraction
  between their two values.
- At a stated station, the value is that station's value exactly.
- There is no value before the first or after the last stated station. Nothing
  is extrapolated, and the sleeve starts and ends at those stations.
- There is no value at a station without a value, or between it and either
  neighbour. Nothing is interpolated across a gap.
- For a signed quantity, there is no value strictly between two adjacent
  stations of opposite non-zero sign stated under one convention
  (`oppositeSigns`). Blending their magnitudes would hide the zero crossing, so
  that open interval has the no-value colour. Each station keeps its own value
  at the station, as a ring joined to the gap by a hard edge, which is how the
  mesh draws a station beside any gap.

Otherwise the values interpolated are the magnitudes the overlay already
shows. That covers a magnitude quantity, a signed quantity between stations of
the same sign or with a zero, and stations whose conventions differ. For two
stations of one sign, the blend of their magnitudes is the magnitude of the
straight-line blend of their signed values. In every other case it is not, and
the legend says so in the product's words whenever Along member is selected:
"a straight-line graphical blend of adjacent stated magnitudes, which does not
represent a signed result or its zero crossing".

**One value per member** is the special case. Each member's value is the
largest magnitude over all its rows, exactly as before this item. Its colour
table, draw path and browser evidence are unchanged.

## 4. Legend and range

The legend states the following, as before:

- the quantity, its unit and the case;
- five ticks;
- the members' minimum and maximum;
- the count of members with no value.

Units go through the existing display-units path (`useDisplayQuantity`). The
ticks and every number use four significant figures.

**Range.** The scale runs from 0 to the largest value drawn in the active
resolution (design system 2.6). Along members, that is the largest stated
station value, or one value for members without stations. One value per member
uses the largest member value. No range is typed in, and no range is fitted to
a limit.

**Along member** adds these lines:

- how many members are drawn from stations, and how many are shown as one
  value;
- the count of stated stations without a value;
- the count of intervals between stations of opposite sign;
- the count of rows not drawn because they state no station;
- the boundary sentence "Along members, colour between two stated stations is
  a straight-line graphical blend of adjacent stated magnitudes, which does not
  represent a signed result or its zero crossing. Outside the first and last
  stated stations the tube has the no-value colour.";
- the quantity's sign rule. For a signed quantity: "This quantity is signed:
  between adjacent stations of opposite sign stated under one sign convention
  the tube has the no-value colour. Signs stated under different conventions,
  such as the end-force and section conventions, are not compared." Otherwise:
  "This quantity is shown as a magnitude whose sign is not stated or may switch
  between stations; no sign is inferred."

A rule ratio adds the count of outcomes that are incomplete or form no ratio.

## 5. Threshold and numbers

**Threshold.** The threshold is typed in the legend's display unit.
`useDisplayPerStored` converts 1 stored unit through the display-units path to
get the factor. While the legend shows the stored unit, the factor is 1: under
entered units, during a pending or unavailable conversion, and always for a
ratio. Every contoured dimension converts by a factor alone.

`thresholdInOverlayUnit` reads the text strictly. Text that is not a number
applies no threshold, and the legend says "Not a number; no threshold is
applied."

Under a threshold, only values strictly above it are coloured. A member at or
below it draws its plain pipe colour (`pipe` role). That is distinct from the
no-value colour, so "below the threshold" is never confused with "no value".

Along members, a crossing is placed exactly, at the fraction where the linear
blend equals the threshold, as a hard edge of two coincident rings. That holds
inside an interval and at its end: a station whose value equals the threshold
puts the edge at that station. Every drawn value strictly above the threshold
is coloured, and no plain ring is ever blended into a coloured one. The
legend states "Coloured above *x* unit; at or below it, the plain pipe colour"
and "The threshold is the user's display filter. It is not a limit or an
allowable." The product supplies no default threshold.

**Numbers in place of colours.** The figure is left neutral, with no result
colour and no sleeves. The stated values are drawn on the canvas instead, in
the legend's display unit with four significant figures. A label is either a
station's own value at its station, or a member's one value at its midpoint.
An interpolated value is never labelled.

Under a threshold, only values above it are labelled. At most 60 numbers are
selected for labelling, the largest first (`CONTOUR_VALUE_LABEL_BUDGET`). A
selected number that would overlap one already placed, or that is outside the
view, is hidden, as with the entity labels. Placement changes with every
frame, so the legend states how many of how many stated values are selected
for labelling and says that some may be hidden; it does not claim a visible
count. Each number's tooltip names its member, its station and the row that
gives the station's value.

## 6. Refused, incomplete, withheld, non-finite and unhealthy results

Nothing is coloured or labelled as if valid unless the existing rules admit
it. The contour adds no rule of its own and relaxes none.

- **No result, or mechanics not solved.** The overlay is unavailable with that
  reason, as before.
- **Withheld by the numerical standing.** A row colours only where the
  reader's standing admits the result (`resultStanding.ts`) and the results
  table gives the row no badge other than "checks passed". This is
  `overlayMayShowRow` in `numericalHealth.ts`; the stress and ratio table
  applies the same rule to outcomes.

  A withheld row leaves its station without a value. The standing is per
  case, so within one case either every row of the quantity is withheld or
  none is. When nothing can be shown, the overlay is "held": the legend gives
  "No result colour on the
  current model" with the numerical health header, the case's state and the
  output sentence. Nothing is coloured, no sleeve is drawn and no number is
  labelled. The output policy (`outputPolicy.ts`) governs output surfaces.
  The overlay is not one (§7), so it follows the numerical standing alone, as
  before, and it admits nothing a surface refuses.
- **Non-finite rows.** One value per member ignores them, as before. Along
  members, a non-finite row leaves its station without a value.
- **Incomplete rule outcomes** (`RULE_INPUTS_INCOMPLETE`), and outcomes that
  form no ratio. These colour nothing, and the legend counts them.
- **A station without a value, and an interval between stations of opposite
  sign.** The sleeve there has the no-value colour (`canvas.unsolved` at 60 %
  over the ground). The tube beneath a member drawn from its stations has the
  no-value colour along its whole length, so it never shows the member's colour
  there or beyond the stated span.

## 7. Display only

The overlay changes no model, no result and no export. It queues no operation
and adds no undo entry. It writes no UI preference. It is not an output
surface: it adds no report or export content and activates no held output.
The viewport tests check the model, the result and the queue across every
control.

## 8. Colour and accessibility

The scale is the design system's `result.scale.1`–`.7`, interpolated in OKLCH
(`resultScaleHex`). It is single-hue and sequential: one teal hue, with order
carried by lightness. It therefore needs no red–green discrimination and reads
under the common colour-vision deficiencies.

Each theme has its own anchors, with the maximum at the end that contrasts
most with that theme's ground. The legend bar uses the same tokens. Legend text
uses the text tokens of the raised surface in both themes. The canvas numbers
use the `canvas.label` and `canvas.labelBg` tokens of both themes. The sleeves
hold both themes' vertex colours and repaint in place when the theme changes.
No colour literal is added to the viewport sources (`viewportColourLiterals.test.ts`).

## 9. Drawing, and the D-72 orbit qualification

**The sleeve.** A member drawn along its length gets a sleeve. The sleeve is a
tube 1.12 times the member's own tube radius, in both the schematic and the
Actual OD display. It has eight sides, so its inscribed circle clears the
member's ten-sided tube. It runs from the first to the last stated station.

The sleeve has a ring of vertices at each contour ring (`contourRings`):

- every stated station;
- enough pieces per interval that a piece spans at most one twelfth of the
  scale, with at most 12 pieces per interval;
- an exact threshold crossing, drawn as a hard edge, including one at a
  station;
- the two sides of every gap, including a sign change, drawn as hard edges.

The GPU blends the vertex colours linearly between rings. Because the rings
are that close on the scale, the drawn blend follows the scale.

All sleeves of a spatial chunk are merged into one mesh with the figure
material. A sleeve is never picked, never haloed and never registered for
selection. The member's own tube stays underneath in the no-value colour, so
selection, hover, picking and labels behave as before, and any part of the
member outside its stated span shows no result colour. Hidden and dimmed members get no
sleeve and keep their own tube's presentation. The sleeves are rebuilt only
when one of these changes:

- the overlay;
- the threshold;
- the geometry mode;
- visibility.

Nothing is done per frame. The numbers display places at most 60 labels after
each main render, through the resource's overlay label slot.

**D-72.** The ruled criteria are in
`execution/_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md` and its
addendum. Their item 4 includes "orbit with result colour, edge lines and
labels at Budget" among the redesigned-only workloads, under the item 1 orbit
limits. Item 6 leaves "per-node interpolated result colour" without a frozen
number until the behaviour exists and can be observed. A supplementary
proposal is then ruled before it is used.

This item makes that behaviour exist, as along-member colour, and it affects
D-72 as follows:

- One value per member, the default, adds no draw call and no triangle. Its
  existing browser evidence (`e2e/result-overlays.spec.ts`) still holds, so
  the orbit workload with result colour is unchanged by default.
- Along member adds one draw call per spatial chunk that has a sleeve. It adds
  `(rings − 1) × 8 × 2` triangles per member. Five stations, with the piece cap
  reached in every interval, give at most 768 triangles per member, plus up to
  32 for each threshold crossing or gap edge.
- The D-72 orbit qualification with result colour has not run, with or without
  sleeves. Nothing here claims or implies it, or any performance acceptance.
- The number for along-member colour remains item 6's to propose, and the
  owner's to rule, after it is observed.

## 10. Evidence

- `resultContour.test.ts` covers:
  - interpolation at stations, ends, a single value and non-finite values,
    with no extrapolation;
  - opposite signs (a gap), the same sign (a blend), and signs under
    different conventions or with a zero (a blend, with no sign inferred);
  - rings, pieces, threshold edges inside an interval and at a station, gaps
    and sign-change gaps;
  - numbers, the threshold and the budget;
  - threshold parsing and conversion.
- `resultContourOverlay.test.ts` covers:
  - the station grid, which the bundled reference also states;
  - drawing along members and one value per member as the special case;
  - non-finite stations, nodal ends, and members with one stated value;
  - the no-value tube beneath a member drawn from stations at 0.25 and 0.75;
  - the governing row of a station with two rows;
  - station signs for a signed quantity, and none for the section extreme;
  - the standing per case;
  - the member table under a threshold, and the numbers display;
  - rule ratios, including incomplete outcomes, withheld outcomes and the
    no-run case.
- `resultContourMesh.test.ts` covers:
  - which members get a sleeve: hidden, dimmed, held and one-value members do
    not;
  - ring geometry and radius;
  - no picking;
  - ring colours in both themes;
  - the theme repaint through the resource's palette path.
- `PipeViewport.contour.test.tsx` covers:
  - labelled, focusable controls;
  - the along-member legend;
  - the threshold in the legend's unit, including invalid text;
  - the blend disclosure and each quantity's sign rule;
  - numbers in place of colours, with and without a threshold, selected for
    labelling;
  - a withheld result, which shows nothing;
  - no model, result or queue change.
- `PipeViewport.contourScene.test.tsx` runs the viewport resource with three's
  renderer replaced by a stand-in that draws nothing. It covers:
  - the sleeve meshes in the result layer along members only;
  - the no-value colour of every pipe instance beneath them;
  - disposal of their geometry and material when the layer is replaced and
    when the viewport unmounts.
- The existing result colour tests are unchanged and still pass.

What is drawn on the GPU still needs a browser: the sleeve's pixels, the hard
edges, the exposed tube ends and the canvas numbers' placement. It has no
browser evidence yet. That remains open and is not claimed. This note describes the design and
code only. It records no backcheck or acceptance.
