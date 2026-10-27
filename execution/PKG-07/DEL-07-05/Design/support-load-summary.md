# Per-support load summary

This design covers survey item A2: for a chosen support, a table of its loads and movements for
each load case and combination that the result's finite emitted rows represent, followed by the
extreme values of each column with the case that governs it. Paths are relative to
`projects/chirality-piping/`. The standard claim fence applies (F-PIP-2). The table lists emitted
rows, and its extremes are arithmetic over the admitted rows that are one solved state (§3.4).
Neither is a qualification, an acceptance or a code check.

## 1. Basis and scope

HELP_HUMAN adopted A2 under the owner's 2026-10-27 delegation (#1662). The CAEPIPE survey
(#1657, `execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md` on
`claude/piping-caepipe-survey-20261026`) recommends it because support and equipment loads are the
main deliverable a stress run hands to the structural and mechanical disciplines, and the desktop
had no cross-case summary per support. It serves FR-SOL-007 and the support types of PRD §12.4,
and it touches DEL-05-02 (envelopes) and DEL-08-04 (result export). This deliverable owns the
viewer slice. W-17's key-results panel (A6) embeds the panel, so the derivation and the component
have a stated contract (§6).

The following are in scope: the derivation, the panel, its placement in the Results section and
keyboard stepping. The following are out of scope:

- **Export of the table** is item D2. The rows keep stored values, units and result-row ids
  (§3.4), so D2 can write them without a second derivation. D2 must apply the output policy
  (`apps/desktop/src/features/results/outputPolicy.ts`) as an output surface. This panel is a
  view, not an output surface.
- **Allowable loads.** The support record (`PreviewModel.supports[]` in
  `apps/desktop/src/types.ts`) has no field for user-supplied allowable loads, so the allowables
  row is omitted and the panel says so. Allowables from a rule pack raise a data-boundary question
  (`docs/IP_AND_DATA_BOUNDARY.md`), and the panel does not read them. Both are remaining items
  (§9). The product supplies no allowable.

## 2. Data sources

Every value is an emitted result row. The rows are read as the viewport's reaction arrows read
them (`apps/desktop/src/features/viewport/reactionOverlay.ts`; DEL-07-05 ScopeOfWork, OD-5
paragraph).

| Column | Row kind | Entity | Notes |
|---|---|---|---|
| Fx, Fy, Fz, Mx, My, Mz | `support_reaction_component_v2`, by `metadata.component` | the support | Sign convention as stated by the rows: support on pipe (`metadata.sign_convention`) |
| \|F\|, \|M\| | `support_reaction_force_magnitude_v2`, `support_reaction_moment_magnitude_v2` | the support | Emitted by the producer for load cases and combinations, and for a range envelope (the largest operand magnitude) |
| ux, uy, uz | `global_nodal_displacement_x/y/z` | the support's node | Global by kind |
| rx, ry, rz | `global_nodal_rotation_x/y/z` | the support's node | Global by kind |

- **Cases.** Each row's `basis_ref` names a `load_case` or a `combination`. The table lists the
  cases that at least one finite emitted row of the result names, of any kind, in the order they
  first appear in the result, load cases first and then combinations, which is the viewport's
  case order (`resultOverlayCases`). A case is labelled with its model
  label and id where the model has a label. A combination whose rows state
  `metadata.basis: "explicit_user_range_envelope"` is a range envelope.
- **Routes.** The preview (preview-physics-2) and exact (pressure-2, pressure-source-1) routes emit
  these kinds. The bundled invented reference and the pressure fixtures under
  `fixtures/product_preview/` carry them, and the producer adds combination rows
  (`fixtures/results/preview_physics_unicode_ids_sparse.json`,
  `preview_physics_range_envelope_sparse.json`). The panel admits a route's rows only by the
  standing rule in §3.2, as the overlays do. The retained routes have no numerical health view, so
  all of their rows are withheld.
- **Withheld attributions.** For a load case, `contract_evidence.preview_cases[].support_attribution.withheld`
  names the supports the solver withheld, with a reason code (for example
  `CONSTANT_EFFORT_NOT_CONSUMED`). The panel reads them through `withheldSupportAttributions`.
- **Equipment connections.** A support with a `boundary_association` (`equipment` or
  `equipment_nozzle`, with `equipment_reference` and an optional `nozzle_reference`) is labelled as
  an equipment connection. Its loads are that support's reaction rows. An `equipment_interfaces`
  record (W-13 model annotation, not evaluated) at the support's node is named beside it. An
  equipment interface at a node with no support has no reaction rows, so it gets no table.

The panel needs no value that the result does not already carry. It does not touch the solver,
the model-document schema or `product_physics`.

## 3. Derivation

`buildSupportLoadSummaries({ model, result, standing })` in
`apps/desktop/src/features/results/supportLoadSummary.ts` is pure: it has no React and no I/O.

### 3.1 Rows

There is one row per support of the model (in model order) and per case (§2 order). A row holds at
most one value per column, taken from the first finite emitted row of that kind, entity and case.
Values and units are stored values, unchanged.

- **Resultants.** \|F\| and \|M\| come from the emitted magnitude rows or are absent. The panel
  never computes them from components.
- **Frame.** Only global rows are tabulated (`SUPPORT_LOAD_FRAME`, the reaction arrows'
  `REACTION_DRAWN_FRAME`). If any reaction row of the support and case states another frame, or
  states none, none of that row's reactions are tabulated, and the row says
  "Reaction rows in another frame; not tabulated (frame)". A node-movement row that states a frame
  other than global is skipped.

### 3.2 Numerical standing and withheld rows

A case's rows are admitted only by the viewport overlays' rule, `overlayMayShowRow(standing, row)`
in `apps/desktop/src/features/results/numericalHealth.ts`. That rule requires three things: the
reader's standing admits the result, a numerical health view exists for the route, and the results
table gives the case no badge other than "checks passed". A combination inherits its own state.
On the pressure-source route, which has no row badges, eligibility alone admits. The rule is
reused, not restated.

A withheld row carries no values at all. It states a reason: "Withheld: numerical health
{state}." (the case's state text), "Withheld: the result's numerical standing does not admit it.",
"Withheld: numerical health is not available for this result." or "Withheld: this case has no
numerical health state.". The panel also repeats the health header and the output sentence when
there is one.

### 3.3 Absent reactions

An admitted row with no reaction values says why:

- the solver withheld the support's attribution, "Reaction attribution withheld by the solver
  (reason code)";
- the support has "No reaction rows" for the case;
- the reaction rows are in another frame (§3.1).

The node's movements are still listed when they exist.

### 3.4 Extremes

For each of the fourteen columns, the maximum and the minimum are signed values taken over the
rows that are one solved state. Those rows are admitted, not a range envelope, and have at least
one value. A column's extremes are formed only when those rows' values in it share one stored
unit. Each extreme names its governing case (key and label) and its result-row id.

- **Ties.** The comparisons are strict, so a tie names the first row in row order. 0 and -0 tie.
- **Units.** If a column's values in those rows have more than one stored unit, no extreme is
  formed, and the cell says "Units differ (…); no extreme is formed", naming the stored units.
  Values are not converted to compare them.
- **Exclusions.** Rows withheld by standing, range-envelope rows and rows with no values are
  listed but stay out of the extremes. Each support's summary counts them (`exclusions`), and the
  panel states the count: "Maximum and minimum over n of m rows; not included: …".
- **Range envelopes.** A range-envelope row selects each value separately from its operands, so
  it is not one solved state. Its |F| is the largest operand magnitude, not the resultant of its
  components. It is listed and labelled "range envelope", with a note.

The extremes are arithmetic over the listed rows. They are not a governing load in any code sense,
and the panel says so.

## 4. Display rule

Values are shown with the report's display rule, `displayQuantityParts` and `displayQuantity` in
`apps/desktop/src/features/report/reportQuantityDisplay.ts` (#1655; DEL-08-01 Design
`report-package-current-results.md` §2.3 item 6). The rule keeps six significant digits, rounds
half away from zero, prints with the ECMAScript layout and shows Pa as MPa. It applies to forces,
moments, translations and rotations in their stored units (N, N*m, mm, rad on the current routes).
`displayQuantityParts` is an additive split of the same rule: `displayQuantity` prints its two
parts, and the shared parity vectors are unchanged. Where every listed value in a column has one
unit, the column header carries the unit and the cells carry the number. Otherwise each cell
carries both. Each value's tooltip (its `title`), in the cells and in the extreme rows alike,
gives the number and display unit under the same rule and the value's result-row id. No text or
attribute of the panel shows a value other than under the rule. The stored value and unit stay in
the derivation's rows for D2.

The rest of the results surface formats values differently. The results table and the overlay
legend use the session's display-unit preference (`QuantityReadout` and `useDisplayQuantity` in
`features/display-units`): the table shows the converted value at full precision, and the legend
shows four significant figures. This panel shows stored units under the report rule, as the brief
asks. Following the display-unit preference is a remaining item (§9).

## 5. Placement and interaction

The panel is `SupportLoadSummaryPanel` in
`apps/desktop/src/features/results/SupportLoadSummaryPanel.tsx`. It sits in the Results section
below the results panel (`App.tsx`). It is hidden while a reopened historical run is shown,
because that run's model need not be the session's model.

- **Choosing a support.** A select lists the supports by label and id. Previous and Next buttons
  step one support at a time without wrapping, and they are disabled at the ends. Inside the
  panel, Alt+ArrowLeft and Alt+ArrowRight step as well (`aria-keyshortcuts` on the buttons). A
  live "Support i of n" announces the position.
- **Selection.** When the session's selection is a support, the panel moves to it. Stepping in
  the panel does not change the session's selection.
- **Table semantics.** The table is labelled "Loads and movements at support {label}". Column
  headers and case row headers are `th` with scope. An empty cell reads "not emitted" (or "none"
  in the extreme rows) to a screen reader. A withheld row spans every column with its reason.
- **Context lines.** The panel states the node, "Frame: global", the emitted sign convention, any
  equipment connection, the exclusion count, the envelope note, the absence of allowables and the
  boundary sentence.

## 6. Composition contract (A6)

```ts
// supportLoadSummary.ts
export function buildSupportLoadSummaries(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined; // { eligible, health }: App's viewportResultStanding
}): SupportLoadSummaries;
// SupportLoadSummaries = { state: "available" | "unavailable"; reason; frame: "global"; cases; supports: SupportLoadSummary[] }
// SupportLoadSummary = { supportId, label, nodeId, equipment, equipmentInterfaces, signConvention,
//                        rows: SupportLoadRow[], extremes: SupportLoadExtreme[], extremeRowCount, exclusions }
// SupportLoadRow = { case, standing: "admitted" | "withheld", withheldReason, reactionAbsence,
//                    reactionAbsenceDetail, values: Partial<Record<SupportLoadColumnId, { value, unit, resultId }>>, inExtremes }
// SupportLoadExtreme = { column, max, min: { value, unit, resultId, caseKey, caseLabel } | null, note }
export const SUPPORT_LOAD_COLUMNS: readonly SupportLoadColumn[]; // display order and dimensions

// SupportLoadSummaryPanel.tsx
export function SupportLoadSummaryPanel(props: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  focusSupportId?: string | null;  // shows this support when it names one
  testIdPrefix?: string;           // default "support-load"
}): JSX.Element;
```

A6 passes the same three inputs it already has for the overlays. A key-results headline can call
`buildSupportLoadSummaries` and read each support's `extremes` without rendering the panel. An
embedding sets `testIdPrefix` so that two instances stay distinct. Every output is frozen and
deterministic for its inputs.

**Render cost.** The panel derives once per identity of (`model`, `result`, `standing`)
(`useMemo`). App passes the memoized `viewportResultStanding`, so the session's focus, stepping and
unrelated renders reuse the derivation. An embedding should pass stable identities too. The
derivation visits the result rows twice, once for the cases and once to index them by support and
node, and then visits each support's cases: O(R + S × C). Only the chosen support's table is
rendered, about 15 cells per case. In App the panel sits in the Results section. While that
section is dormant, the section's memo does not re-render it. On the native smoke's exact-route
model (1 support, 2 load cases, 297 rows in the committed producer bytes) a jsdom count gave 1
derivation and 2 panel renders through solve, Results, a result-filter edit and Report. The panel
had 213 elements, 75 of them cells, in a 2,465-element document. So the panel is rendered
eagerly, with no disclosure or deferral.

## 7. Claims fence

The panel lists emitted rows and their arithmetic extremes. It records and implies no acceptance,
issuance, compliance or adequacy of a support, an equipment nozzle or a load. It supplies no
allowable, and it compares nothing with a limit.

## 8. Tests

- `apps/desktop/src/features/results/supportLoadSummary.test.ts` covers the derivation: case order
  and labels; stored values and ids; resultants only when emitted; signed extremes with their
  governing case; ties naming the first row (and 0 against -0); mixed units; withheld rows under
  mixed standing (a sensitive case, a combination with and without its own state, ineligible
  standing, no health view, no standing); the pressure-source route; solver-withheld attribution
  with node movements kept; a support with no rows; another frame; range envelopes listed but kept
  out of the extremes; the unavailable states; and the bundled invented reference (each support,
  emitted ids, `CE-120` withheld).
- `apps/desktop/src/features/results/SupportLoadSummaryPanel.test.tsx` covers the panel: the
  display rule and header units; the rule in every cell, cell title and extreme title for a value
  that rounds (1234.5678 N), Pa values shown in MPa and a mixed-unit column, with no stored value
  at full precision anywhere in the panel's markup; a withheld row with no values and the
  exclusion note; the ineligible state; stepping by button, Alt+Arrow and select, with no wrap; following a focused
  support; and the unavailable state under a custom test-id prefix.
- `apps/desktop/src/features/results/SupportLoadSummaryPanel.derivation.test.tsx` counts
  derivations. There is one per input identity, none for a focus change, a test-id prefix or
  stepping, and one more for each new model, result or standing.

## 9. Remaining items

- **D2: export of the table.** It is an output surface under `outputPolicy.ts`.
- **User-supplied allowable loads on the support record.** This needs a model-document field,
  which is a schema change.
- **Allowables from a user's rule pack.** This is a data-boundary question.
- **Display units.** The panel could follow the session's display-unit preference (§4).
- **Sign presentation.** A "load on support" (pipe on support) view would negate the emitted
  support-on-pipe values, which is a presentation choice for the structural handoff. The emitted
  convention is shown today.
- **Supports beyond the model's list.** The panel lists the model's supports. A reaction row
  whose entity is not a model support (for example a stale result) is not shown.
