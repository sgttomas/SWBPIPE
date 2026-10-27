# Split into equal parts, and end points from a length, angle or slope (B5)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Survey item
B5 (`execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md` §Bundle B),
adopted under the owner's 2026-10-27 delegation (`loop/LOOP_INIT.md`). It serves
FR-GUI-004 (`docs/PRD.md`: add, modify, move and reconnect model entities while
preserving schema validation). Paths are relative to `projects/chirality-piping/`;
"src/" is `apps/desktop/src/`.

**What it is.** Two entry aids over existing operations:
- **Split into equal parts.** One pipe becomes N equal pipes with N − 1 labelled
  nodes, in one reviewed batch.
- **End point from a length and a direction.** A new end point is computed from a
  start node and one of: a length along a direction vector, a length at an angle
  from the horizontal, or a horizontal run at a slope of "1 in n".

Both add no operation, no model-document member (the document stays 0.8.0) and no
engineering default. The batches go through the existing review queue (DEL-16-06),
so validation, the per-step diff preview, the single all-or-nothing Apply and the
single Undo are the queue's own. B2 (`execution/PKG-07/DEL-07-02/Design/bulk-change.md`)
is the pattern.

**Risk class:** persistence and numerical meaning. Positions are computed in
binary64 as stated below, and the stored value is the computed value.

## 1. Operations used

| Aid | Operations, in batch order | Producer of the same operation |
|---|---|---|
| Split into equal parts | for k = 1 … N − 1: `split_pipe_run` (Element, `pipe_segments`), then `set_field` Node `position.x`/`.y`/`.z` for each component that needs an exact placement (§2.2) | Geometry tools' split (`geometryDraft.ts`); the inspector's node position fields |
| End point, new route end | `create_node`, `connect_pipe_run` (unchanged) | the viewport's route builder (`routeDraft.ts` `buildRouteSubmission`) |
| End point, reconnect to a new point | `create_node`, `reconnect_pipe_run` (unchanged) | Geometry tools' reconnect (`geometryDraft.ts` `buildReconnectBatch`) |

The end-point aid only fills the existing X, Y and Z fields and their unit. The
existing builders emit the operations.

## 2. Split into equal parts

Builder: `src/features/geometry-tools/splitIntoParts.ts`. Panel:
`SplitIntoPartsPanel.tsx`, under Editing and review tools → Geometry.

### 2.1 The batch

Split k acts on the piece that still ends at the original end B. For k = 1 that
is the source pipe. The fraction is g_k = 1 / (N − k + 1), measured from that
piece's start. The source pipe keeps its id on the first part, as the existing
split does. New pipe k runs from node k to B until split k + 1 shortens it.
Each new pipe is a copy of the piece it splits, as the applier makes it:
section, material, y_reference, tags and notes are carried. The applier also adds
each new pipe to the source's line after the piece it was split from. The
canonical `before` of split k is that piece's record as it stands at step k. The
planner simulates every step to produce it.

Each new node and new pipe takes its identity from a pattern. The patterns are
for the node id, node label, pipe id and pipe label, plus one provenance text.
`{k}` in a pattern becomes first + (k − 1) · step. First and step are whole
numbers that the user may change; they start at 1 and 1. B1's numbering is not
on main. This follows the existing split's rule: ids are explicit, and the user
writes them, here as a pattern. Id patterns must contain `{k}`. Each id must
pass the applier's `is_valid_schema_id`, must differ from every `id` anywhere
in the model (the applier's `global_ids`), and must differ from the batch's other
ids.

At most 100 parts are queued in one batch. This is a review-size bound, not an
engineering limit.

### 2.2 The arithmetic

Per component, in binary64 with round to nearest, from the stored end positions
A and B (project length unit):

```
P_k = A + (B − A) · (k / N)        k / N, B − A, the product and the sum each rounded once
```

−0 becomes +0, and the end positions are read with −0 as +0, because the
engine receives JSON, where −0 is written 0. P_k is exactly what one existing
split of the original pipe at fraction k / N would store
(`geometry_operations.rs:406`, `add(a, scale(sub(b, a), f))`). Node k is never
computed from node k − 1.

**The applier's chained value.** Split k places its node at
S_k = P_{k−1} + (B − P_{k−1}) · g_k. In general S_k is not bit-identical to
P_k. With A = 0.1, B = 7.3 and N = 7, nodes 5 and 6 differ by one ulp. Where they
differ, the batch follows split k with a `set_field` on that component of node k:
`before` = S_k as text and `after` = `{"value": P_k, "unit": <project unit>}`.
The applier compares `before` numerically (`lib.rs:8281`), and the project unit
makes the conversion the identity. So every node is stored as P_k, and the next
split starts from P_k. The planner predicts S_k with the applier's own operation
order. Rust does not contract floating-point operations, and JavaScript numbers
are binary64, so the prediction is exact. The engine test checks it.

**Error statement.** Let u = 2⁻⁵³ and x* = A + (B − A)k/N exactly. To first
order in u, with every intermediate in the normal range or zero:

```
|P_k − x*| ≤ u·|x*| + 3u·|B − A|·(k/N)
```

This is a first-order estimate, not an unconditional binary64 inequality. When the
product or the sum falls below the normal range, each such operation adds at
most 2⁻¹⁰⁷⁵ absolute in place of its relative term, so at most 2⁻¹⁰⁷⁴ in all.
A subnormal B − A is exact, and k/N ≥ 1/100 is normal. An overflow is refused as
non-finite. P_k is not claimed to be correctly rounded, even when B − A is exact,
because the product is rounded before the sum. With A = 6.9705855667666725,
B = −2.5521381070139633, N = 8 and k = 7, P = −1.3617976477913842, while the
correctly rounded interpolation is −1.3617976477913838. The engine test pins
the stored value to the formula's result.

This bound is the same for every k. A repeated addition of a fixed step
(A + step + step + …) carries up to k roundings to node k. The applier's chained
split S_k carries every earlier node's rounding into the next. The per-k formula
is not always the nearer of the two to x* for a given node. With A = 1.25,
B = −3.7 and N = 6, P_2 = −0.3999999999999999 and S_2 = −0.4. Its bound,
however, does not grow along the run.

No value is rounded to a display precision. The preview shows each coordinate in
ECMAScript's shortest round-trip form, which parses back to the same double. The
JSON the engine receives uses the same form, and serde_json `float_roundtrip`
parses it back to the same bits.

### 2.3 Refusals

The planner refuses whatever the applier's `split` would refuse, using the same
rules. Each split is checked on the model as it will stand at that step. Nothing
is queued while any refusal stands, and the first reason is shown:

- **The request.** N must be a whole number from 2 to 100. The patterns, labels
  and provenance must be present. First and step must be whole numbers.
- **`record_shape`** (`:128`). The pipe and its end nodes must have exactly the
  members the split understands, with finite numbers, and section quantities must
  each carry a value and a unit. With it comes `validate_corrosion_allowance`
  (`section_bindings.rs:89`). When the section has `corrosion_allowance`, the
  allowance, the wall and the mill tolerance (0 when absent or null) are each
  converted to metres by the engine. Then mill < 0 or mill ≥ wall refuses, and so
  does allowance < 0 or mill + allowance ≥ wall. The planner uses the applier's
  messages, and the engine test checks that the engine gives the same message
  and outcome.
- **`attachments` with split = true** (`:293`). Anything outside the model-data
  collections that names the piece blocks the split. That includes a support or a
  pipe load that names the pipe, wind exposure, pressure regions, and a realized
  bend's `bend_pipe_ref`. A component that names either end node also blocks it:
  a bend, valve or any other component at an end. Text metadata (label, name,
  provenance, tags, notes) does not count. A support at an end node does not
  block. The planner lists the references in the applier's order and path
  format. The engine test checks that its text contains the engine's own list.
- **`frame`** (`:366`). Each piece, before and after each split and at the final
  positions, must be longer than the frame kernel's 1e-12 m once its span is
  converted to metres by the engine. Its y_reference, when it has one, must not be
  parallel to it.
- **`metadata` and `global_ids`** (`:215`, `:197`): the id rules in §2.1.

Validate and Apply in Batch review still judge every operation.

### 2.4 Preview

The panel recomputes the plan whenever the model or the request changes. It
shows:
- a table of the new nodes, with k, id, label and X, Y and Z in the project unit;
- a table of the pipes after the split;
- the number of splits and of exact placements.

Batch review then shows each step's diff. "Queue split as one batch" builds the
batch from the plan. If the model or the request changed while the batch was
being built, it queues nothing and says so.

## 3. End point from a length and a direction

Arithmetic: `src/features/geometry-tools/directionArithmetic.ts`. Component:
`DirectionEntry.tsx`. It sits in the viewport's "New endpoint" (start: the
From node) and in Geometry tools → reconnect → "To a new point" (start: the
pipe's end that stays).

### 3.1 Methods

| Method | Entered | Needs the up direction |
|---|---|---|
| Length along a direction vector | length L and unit; direction (dx, dy, dz), any nonzero length | no |
| Length at an angle from the horizontal | length L along the element and unit; a horizontal direction h; angle θ and unit, positive rising | yes |
| Horizontal run at a slope of 1 in n | horizontal run R and unit; a horizontal direction h; n; falls or rises along h | yes |

**"1 in n"** means the rise or fall over the horizontal run is 1/n. The run is
horizontal, not measured along the pipe, which is the usual convention for drains.
The field label says so: "Slope 1 in n: rise or fall over the horizontal run, n".

**Which axis is vertical.** The model has no vertical or gravity convention. The
project records units only. Each self-weight load case records its own gravity
axis per case, and the aid does not read it. The user therefore states which
global direction is up (±X, ±Y or ±Z). There is no default. The angle and slope
methods are refused until it is stated. The horizontal direction must have an
exactly zero component on that axis.

### 3.2 The arithmetic

All in binary64, round to nearest, in this order:

```
Ls   = c(L)                       c = the engine's convert_for_dimension, entry unit → project length unit
s    = max(|dx|, |dy|, |dz|)      exact; refused unless finite and > 0
e_i  = d_i / s                    one rounding each; the largest |e_i| is exactly 1
n    = sqrt(ex·ex + ey·ey + ez·ez)   left to right; n in [1, √3]; sqrt correctly rounded
u_i  = e_i / n                    one rounding each
vector: Δ_i  = Ls · u_i
angle:  θr   = c(θ) to rad;  Δ_i = (Ls · cos θr) · h_i on the horizontal axes;  Δ_up = s_up · (Ls · sin θr)
slope:  Δ_i  = Ls · h_i on the horizontal axes;  Δ_up = s_up · s_sense · (Ls / n)
end_i = start_i + Δ_i,  then −0 becomes +0
```

- s_up and s_sense are ±1, so the multiplications by them are exact. For a
  slope, s_sense is applied first, to Ls / n, and s_up second. The
  vertical increment is never mixed with a horizontal term.
- **Conversion before addition**, as in B1 (DEL-07-02 `run-sheet-entry.md`
  §6.2). The length is converted once, through the engine's display-conversion
  facade (`display_units.rs` → `convert_for_dimension`, `core/units/src/lib.rs:917`).
  The aid never holds a factor. The start is the stored position in the project
  unit.
- **Trigonometry.** cos and sin are the runtime's `Math.cos` and `Math.sin`.
  ECMAScript leaves their last bit to the implementation, so a value may differ by
  an ulp between the test runtime and the desktop shell's web view. The computed
  value is the value shown and the value written, so the claim is
  displayed = emitted = stored. It is not a cross-runtime claim.
  - 90° converts to exactly fl(π/2), so the angle must satisfy |θr| < fl(π/2).
    A vertical element is entered as a direction vector, which gives exactly
    ±L on the axis.
  - 0 rad gives an exactly horizontal element.
- **Exact cases.** An axis direction gives ±Ls exactly. A slope along an axis
  gives (R, R/n), with one rounding in the division. The tests pin
  (12, −1/4) for 12 ft at 1 in 48.
- **Conversion c.** It is the identity, exactly, when the units are the same.
  Otherwise, for length and angle (offset 0), it is
  c(v) = ((v · f_e + 0) − 0) / f_s (`core/units/src/lib.rs:714-720`), with
  binary64 factors f_e and f_s.
  - There are up to two inexact roundings: the multiplication and the division.
  - Adding the source's zero offset is exact, except that it canonicalizes a
    signed zero (−0 + 0 = +0).
  - Subtracting the destination's zero offset before dividing is also exact.
- **Scaled normalization.** Dividing by the largest component first means no
  square overflows, and any square that underflows is below 2⁻¹⁰⁷⁴ against a
  sum of at least 1. Such underflowing small components do not materially
  affect the norm. For any finite nonzero d, |‖u‖ − 1| ≤ 3u to first order.
  This bound is on the norm, not on each component's relative accuracy. A
  direction with an axis component only gives u = ±1 exactly.
- **Error.** When nothing underflows in the normalization or the
  multiplication, each vector-method increment is within about 4u relative of
  Ls·d_i/|d|, plus one rounding in the sum.
  - A component can underflow when it is far smaller than the largest one. Its
    increment then carries an absolute error of up to about Ls · 2⁻¹⁰⁷⁴, and
    can be 0. For example, d = (1e-200, 1e200, 0) and Ls = 1e200 give an X
    increment of 0, where the ideal value is about 1e-200.
  - The same holds for h in the angle and slope methods. The angle method adds the runtime's trigonometric
  error and the angle conversion's rounding. The slope method has one rounding
  in R/n and, for a non-axis h, one in each product.
- **−0.** JSON cannot carry −0, so every computed end-point coordinate is
  normalized with `x === 0 ? 0 : x` before it is shown or used. −0 arises, for
  example, from a start coordinate of −0 plus a −0 increment. The returned
  increments Δ are not normalized and can keep −0; only end points are shown
  or emitted.

### 3.3 Displayed equals emitted

The readout shows each coordinate in shortest round-trip form. "Use these
coordinates" writes exactly that text into the X, Y and Z fields and sets the
coordinate unit to the project unit. The builders emit `Number(text)`, which is
the same double, and `create_node` with the project unit converts by the
identity. The UI says: "Entry aid only. The direction, angle or slope is not
saved: the model stores the node coordinates computed below."

## 4. Keyboard and accessibility

| Command | Button or menu | Keyboard |
|---|---|---|
| Open Split into equal parts on a pipe | "Split into N…" in the entity context menu of a pipe: tree rows, bulk-grid rows, viewport labels and canvas; the Geometry tab under Editing and review tools | Shift+F10 or the ContextMenu key, then the item and Enter; Tab to the Geometry tab, then Enter |
| Enter the split, queue it | the panel's fields and "Queue split as one batch" | native controls; Tab between them; Enter or Space on the button |
| Compute and use an end point | the aid's fields and "Use these coordinates for the route end / reconnect point" | native controls; Tab; Enter or Space |
| Validate, Apply, Undo | Batch review, the viewport's draft review, and the existing Undo | as today |

- **The B9 selection rule.** "Split into N…" acts on one pipe. It is disabled,
  with its reason, when the menu's pipe is not selected or more than one entity
  is selected. It is offered only for pipes.
- **Focus.** Opening the panel from the menu moves focus to "Number of equal
  parts".
- **Labels.** Every control has a label. The aid's labels name their surface
  ("Route end: …", "Reconnect point: …").
- **Disabled controls.** A disabled button is described by the reason it is
  disabled.
- **Status and tables.** The readouts are status text. The preview tables have
  captions and column headers.

## 5. Excluded, and why

- **Saving the slope, angle or direction.** These are entry aids. Saving them
  would need a model-document member and a version step, and the model stores
  coordinates only.
- **Splitting across a bend, component or attachment.** The existing split
  refuses it, and so does this one. Repartitioning attachments is a different
  operation.
- **Several pipes at once, unequal parts, or splitting at given distances.**
  These are outside the survey item.
- **A slope measured along the pipe.** The horizontal-run convention is the one
  used for drains. A length along the pipe at a slope can be entered with the
  angle method or a direction vector.
- **±90° by angle, and plan bearings.** Use a direction vector for either.
- **Absolute node coordinates in the inspector.** The inspector's node position
  fields stay absolute. The straight-element edit route is the reconnect to a
  new point.
- **The run sheet (B1).** It is not on main. Its increment columns could later
  call the same arithmetic.
- **The command palette.** As in B2, the menu and the tab give both routes.
- **Reserving ids against other queued batches.** The existing split does not
  do this. A collision between two queued batches is refused at Apply by the
  second batch.

## 6. Evidence

- `geometry-tools/splitIntoParts.test.ts`:
  - the generated batch equals a hand-built batch (two splits and one exact
    placement);
  - hand values of the stated positions;
  - every request and applier refusal: record shape, attachments, frame
    tolerance, parallel y_reference, ids.
- `geometry-tools/splitIntoParts.session.test.tsx`, through the wasm engine:
  - N = 7 in ft: every node is bit-identical to P_k (`Object.is` per component),
    with the chain, labels, tags, y_reference and line membership as above, and
    one Undo restores the model hash;
  - a −0 end coordinate;
  - the planner's refusal names the same references as the engine's
    `OP-GEOMETRY-ATTACHMENT-UNSUPPORTED` for the same single split;
  - node k equals what one separate engine split of the original pipe at k/N
    stores, for k = 1 … 6;
  - the correct-rounding counter-example stores the formula's value;
  - a piece that rounds onto an end node is refused, as the engine refuses that
    single split;
  - the corrosion and effective-wall rule: the planner and the engine agree on
    four refusals and one acceptance, with mixed mm and in.
- `geometry-tools/directionArithmetic.test.ts`:
  - the slope, angle and vector conversions against independent hand values,
    exact where the arithmetic is exact and within one ulp for √2 and the
    trigonometric cases;
  - units through the engine;
  - −0;
  - the refusals: no up direction, a non-horizontal h, no sense, ±90°;
  - all six up directions for the angle and slope methods;
  - unit length within the bound for tiny (1e-162), huge (1e160) and
    mixed-scale directions.
- `geometry-tools/directionEntry.session.test.tsx`, through the wasm engine:
  - a new route end and a reconnect point are stored bit-identical to the computed
    point, with lengths in inches and metres in a ft project;
  - a −0 case;
  - Undo.
- `geometry-tools/DirectionEntry.test.tsx` and `SplitIntoPartsPanel.test.tsx`:
  - displayed = emitted;
  - nothing is assumed;
  - nothing is queued on a refusal or a model change;
  - focus on open.
- `App.splitAndSlope.test.tsx`, through the shell:
  - the tree menu (mouse), a tree row (Shift+F10) and a viewport label
    (ContextMenu key) open the panel;
  - Batch review, Apply and Undo;
  - the B9 disabled reason;
  - the viewport's route end takes the computed point exactly.
