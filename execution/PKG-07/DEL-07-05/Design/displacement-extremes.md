# Displacement extremes table

This design covers survey item A8. For each node, the table gives the largest and smallest value
of each translation and rotation component over the load cases and combinations the user chooses,
with the case that governs each value. A filter narrows the list to the nodes of supports of one
kind (for example guides, line stops or hangers). Paths are relative to
`projects/chirality-piping/`. The standard claim fence applies (F-PIP-2). The extremes are
arithmetic over emitted rows that are one solved state (§3.3). They are not a clearance check, a
travel check, an acceptance or a qualification, and the product supplies no limit to compare them
with.

## 1. Basis and scope

HELP_HUMAN adopted A8 under the owner's 2026-10-27 delegation (#1662). The CAEPIPE survey
(#1657, `execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md`) recommends it for
clearance checks and support travel reviews. The desktop showed node movements per row in the
results table and per support in A2's panel, but it had no cross-case envelope over every node.
A8 serves FR-SOL-007 (recovered results available for review) and FR-SOL-009 (combinations are
read and labelled; range envelopes are kept out, §3.3). It touches DEL-05-02, whose reader
supplies the rows and their standing. This deliverable owns the viewer slice. W-17's key-results
panel (A6) may embed it, so the derivation and the component have a stated contract (§7).

In scope: the derivation, the view rule (case choice, support-kind filter, sort), the panel, its
placement in the Results section, paging and selecting a node through the session's selection.
Out of scope:

- **Export of the table** is item D2. The extremes keep the stored value, unit, result-row id and
  case key (§3.2), so D2 can write them without a second derivation. D2 is an output surface under
  `apps/desktop/src/features/results/outputPolicy.ts`. This panel is a view, like the viewport
  overlays and A2's and A3's panels, and it is not in `OUTPUT_SURFACES`.
- **Clearance and travel limits.** No model record carries a clearance, and a hanger's
  `travel_range` and `movement_limit` are user annotations that the panel does not compare with
  anything. Comparing them is a remaining item (§10).

## 2. Data sources

| Source | Where the desktop holds it | Used for |
|---|---|---|
| Node movement rows | `MechanicsResult.results`, kinds `global_nodal_displacement_x/y/z` and `global_nodal_rotation_x/y/z` | The six components ux, uy, uz, rx, ry, rz. A2's column table (`SUPPORT_LOAD_COLUMNS`, the `node` columns) names them |
| Case labels and envelopes | `supportLoadCases(model, result)` (A2) | The case list, its order and the envelope flag |
| Numerical standing | `viewportResultStanding` in `App.tsx` (`{ eligible, health }`) | Withholding (§4) |
| Nodes | `PreviewModel.nodes` | The listed nodes, their order and labels |
| Support kinds | `PreviewModel.supports[]`: `family`, `hanger.hanger_type`, `nonlinear.behavior` and `nonlinear.dof` | The filter (§5.2) and the supports column |

The rows' `entity_ref` is the node id. Their `metadata.coordinate_system` is `global`. A row that
states another frame is not used and is counted (§3.3). The preview (preview-physics-2) and exact
(pressure-2, pressure-source-1) routes emit these kinds. The bundled invented reference and the
range-envelope fixture (`fixtures/results/preview_physics_range_envelope_sparse.json`, which emits
envelope rows for every node component) carry them.

### 2.1 Support-kind vocabulary

The kind is the support's stated `family`, the model document's `SupportFamily` enumeration
(`schemas/model.schema.yaml`): `anchor`, `guide`, `line_stop`, `vertical_support`, `spring`,
`variable_spring_hanger`, `spring_hanger`, `constant_effort_support` and `nonlinear`.
`SUPPORT_FAMILIES` in `displacementExtremes.ts` lists them with the support form's labels
(`SupportConfigurationForm.tsx`: Anchor, Guide, Line stop, Vertical support, Spring, Variable
spring hanger, Spring hanger, Constant-effort support, Nonlinear support). A test reads the schema
and fails if the two lists differ.

- `family` is optional, and an explicit `null` lets the solver infer one. The desktop does not
  restate the solver's inference, so such a support's kind is "Family not stated".
- A stated value outside the enumeration (a preserved source value) is shown as stated, for
  example `"strut" (as stated)`.
- **Hangers.** The filter also offers "Hangers" for nodes with a support the report treats as a
  spring hanger or constant-effort support. The rule is the report's, `isSpringHangerSupport` in
  `apps/desktop/src/features/report/springHangerSupport.ts`, which the report's spring hanger
  evidence (`ReportPanel.tsx`) and its review rows and provenance (`renderableReportInput.ts`)
  also call. The rule is the solver's (`support_hanger_type`, `is_variable_spring_hanger` and
  `is_constant_effort_support` in `core/product_physics/src/lib.rs`): the trimmed
  `hanger.hanger_type` when it is non-empty, otherwise the trimmed `family`, counts when it is
  `variable_spring_hanger`, `spring_hanger` or `constant_effort_support`. An explicit hanger type
  decides over the family. Trimming is Rust's `str::trim`, which removes the Unicode `White_Space`
  characters (`trimLikeRust`). JavaScript's `trim` differs: it also removes U+FEFF and keeps
  U+0085. The report's hanger summary splits variable and constant-effort hangers by the same
  reading (`springHangerKind`), so a padded or family-only hanger falls in its group. A
  `spring_hanger` support is therefore listed under "Hangers" as well as under its family, "Spring
  hanger" (report-content correction, HELP_HUMAN, 2026-10-27: the solver treats such supports as
  variable spring hangers and emits their review rows, so the report had omitted real evidence).
- A nonlinear support's entry in the supports column adds its stated behavior and DOF (for
  example "Nonlinear support (gap, UY)"). The filter does not split nonlinear behaviors (§10).

Nothing else is needed. The panel does not touch the solver, the model-document schema or
`product_physics`.

## 3. Derivation

`buildDisplacementExtremes({ model, result, standing })` in
`apps/desktop/src/features/results/displacementExtremes.ts` is pure: it has no React and no I/O.
`displacementExtremesView(data, { caseKeys, filter, sort })` applies the user's choices to it and
is pure as well.

### 3.1 Cases

The cases are `supportLoadCases(model, result)` restricted to those with at least one finite
movement row, in the same order: load cases first, then combinations, each in order of first
appearance. Each case carries its standing (§4), its row count and whether it can enter the
extremes. Only an admitted case that is not a range envelope can enter. A withheld case and a
range envelope are listed but cannot be chosen, and each states why.

### 3.2 Values

For each model node in model order, each component and each case that can enter the extremes, the
derivation keeps the first finite emitted row of that kind, node and case. The producer emits one.
It keeps the stored value, the stored unit and the result-row id, unchanged. It keeps no value of
a withheld case or of a range envelope.

### 3.3 Extremes

For a node and a component, the maximum and the minimum are signed values taken over the chosen
cases' kept values. Each names its governing case (key and label) and its result-row id.

- **Ties.** The comparisons are strict, so a tie names the first case in case order. 0 and -0 tie.
  This is A2's rule.
- **Units.** An extreme is formed only when the values share one stored unit. Otherwise the cell
  says "Units differ (…); no extreme is formed", naming the stored units. Values are not converted
  to compare them.
- **Exclusions.** These stay out of every extreme, and the panel counts them: cases withheld by
  numerical standing (with their row counts), range envelopes, cases the user has not chosen, rows
  in another frame and rows whose node is not in the model.
- **Range envelopes.** A range-envelope row selects each value separately from its operands, so a
  set of envelope rows is not one solved state. Envelopes are listed in the case chooser, labelled
  and disabled, with A2's envelope note.
- **No values.** A node with no kept value in the chosen cases shows "none" in each cell and is
  never ranked.

The extremes are arithmetic over the listed values. They are not a governing displacement in any
code sense, and the panel says so.

## 4. Numerical standing and withheld cases

A case is admitted only by the viewport overlays' rule, `overlayMayShowRow(standing, row)` in
`numericalHealth.ts`, applied to the case's basis as A2 applies it. The rule is reused, not
restated. A withheld case keeps no value. The case chooser shows it unchecked and disabled, with
A2's reason text (`caseWithheldReason`: "Withheld: numerical health {state}." and the others) and
its row count. The exclusion line counts withheld cases. When no chosen case is admitted, the
panel shows no table and says "No admitted case is chosen; no extreme is formed." The retained
routes have no numerical health view, so all of their rows are withheld. The panel also repeats the
numerical health header and the output sentence when there is one.

## 5. View rule

### 5.1 Case choice

Every case that can enter starts chosen. The user unchecks or rechecks each one, and "All cases"
chooses them all again. A new result keeps the user's unchecked keys, and any new case starts
chosen. `caseKeys: null` means every case that can enter. A key that is withheld, an envelope or
unknown is ignored.

### 5.2 Support-kind filter

| Filter | Nodes listed |
|---|---|
| All nodes | Every model node |
| Support nodes | Nodes with at least one support of any kind |
| Hangers | Nodes with a hanger (§2.1) |
| One family | Nodes with at least one support of that stated family |
| Family not stated | Nodes with a support whose family is absent or null |

A filter is offered only when it lists at least one node, except "All nodes", which is always
offered. Each option states its node count. Several supports may share a node. The node is listed
once, and its supports column names each support with its kind. An unknown filter id falls back to
"All nodes".

### 5.3 Sort

"Node order" keeps model order. "Largest |c|" for a component c sorts by the larger magnitude of
the node's maximum and minimum of c, which is the largest |value| over the chosen cases, largest
first. Equal keys keep model order, and 0 ties -0. Nodes with no extreme of c follow in model
order, unranked. If the ranked extremes of c have more than one stored unit, the view keeps model
order and says "Units differ (…); not sorted". The key column header has `aria-sort="descending"`
when sorted.

### 5.4 Paging and the large-model bound

The view lists every filtered node. The panel renders one page of `DISPLACEMENT_EXTREMES_PAGE` =
50 node rows, with Previous page and Next page buttons and a polite live line "Nodes i–j of n". A
page renders at most 50 rows of 14 cells (node, supports and 12 extremes), whatever the model's
size. A new filter, sort or case choice returns to the first page.

When the session's selection is a node, the panel moves to that node's page once each time the
selection changes. The move waits until the node's row can be shown: there is a result with
movement rows, at least one admitted case is chosen, and the current filter lists the node.
Until then the selection stays pending. A selection made before results exist, or while the
filter hides the node or no case is chosen, therefore shows its page when the row appears. While
the derivation is unavailable (no result, mechanics not solved, no nodes or no movement rows),
the session's selection is held pending again, so a result that returns shows its selection's
page. A panel that is shown again (a historical run hidden and shown) starts with its selection
pending. Once the move is made, paging is the user's: only a new selection, a result that
returns after none was available, or the panel being shown again moves the page to the
selection again.

The page, the pending selection and the inputs the page was set under are one state, adjusted
during render by a pure function of that state and the render's inputs, and set only when it
differs. No effect or ref carries them, so React's development effect replay (StrictMode's mount,
unmount and mount, or an Activity shown again) neither loses a pending selection nor resets the
user's page, and a repeated render settles at once.

The derivation is linear in the result-row count R: it scans `result.results` for movement rows,
`supportLoadCases` scans them again for the case list, and the kept movement rows are then
indexed by case and node. The view visits each filtered node's six components over the chosen
cases, which is O(N × C × 6), plus a sort of O(N log N). On the 10,000-node scale that is about
60,000 value visits per case per control change, with no rendering beyond the page.

## 6. Display rule, placement and interaction

Values are shown under the report's display rule, `displayQuantityParts` and `displayQuantity` in
`features/report/reportQuantityDisplay.ts` (#1655): six significant digits, half away from zero,
ECMAScript layout, Pa shown as MPa, and mm and rad as stored. Cells, titles and attributes follow
it, and no text or attribute of the panel shows a value otherwise. A value's title gives the
displayed number and unit, the governing case and the result-row id. Where every extreme in a
component's columns shares one unit, the column-group header carries it and the cells carry the
number. Otherwise each cell carries both.

The panel is `DisplacementExtremesPanel` in
`apps/desktop/src/features/results/DisplacementExtremesPanel.tsx`. It sits in the Results section
below A3's panel, and it is hidden while a reopened historical run is shown, as A2's and A3's
panels are.

- **Controls.** A "Nodes" select (the filters with counts), a "Sort" select (Node order, then
  "Largest |ux|" to "Largest |rz|"), and a "Cases" fieldset with one checkbox per case and an "All
  cases" button. A withheld or envelope checkbox is disabled, and its reason is tied to it with
  `aria-describedby`.
- **Table.** It is labelled "Displacement and rotation extremes: {filter}". The first header row
  has Node, Supports and one column group per component (`scope="colgroup"`), and the second has
  Max and Min (`scope="col"`). Each node's row header is a button "Select node {label}".
  Activating it (click, Enter or Space) calls `onSelectNode(nodeId)`. App maps it through
  `resolveEntitySelection` to `handleSelectEntity`, A3's entity path. The row of the session's
  selected node is marked (`aria-current`, `.selected-result-row`). Each extreme cell shows the
  value and, below it, the governing case label. An empty cell reads "none" to a screen reader.
- **Context lines.** The numerical health header and output sentence when present, the exclusion
  line ("Maximum and minimum over k of m cases; not included: …"), the envelope note when an
  envelope is listed, a count of rows not used (another frame, or a node not in the model) when
  there are any, and the boundary sentence.

## 7. Composition contract (A6)

```ts
// displacementExtremes.ts
export function buildDisplacementExtremes(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined; // App's viewportResultStanding
}): DisplacementExtremes;
// DisplacementExtremes = { state: "available" | "unavailable"; reason; cases: DisplacementCase[];
//   nodes: DisplacementNode[]; filters: DisplacementFilter[]; unusedRows: { otherFrame, notModelNode } }
export function displacementExtremesView(data: DisplacementExtremes, options: {
  caseKeys: readonly string[] | null;   // null: every case that can enter
  filter: string;                       // a DisplacementFilter id
  sort: "model" | DisplacementComponentId;
}): DisplacementExtremesView;
// DisplacementExtremesView = { filter, chosenCases, exclusions: { withheld, withheldRows, envelope, notChosen },
//   rows: DisplacementNodeExtremes[], sort: { component, sorted, note, rankedCount }, units }
// DisplacementNodeExtremes = { nodeId, label, supports, extremes: Record<component, { max, min, note }>, key }
// max/min = { value, unit, resultId, caseKey, caseLabel } | null

// DisplacementExtremesPanel.tsx
export function DisplacementExtremesPanel(props: {
  model: PreviewModel; result: MechanicsResult | null; standing: ResultOverlayStanding | null | undefined;
  selectedNodeId?: string | null;              // marked; its page is shown once per change (§5.4)
  onSelectNode?: (nodeId: string) => void;
  testIdPrefix?: string;                       // default "displacement-extremes"
}): JSX.Element;
```

A6 passes the inputs it already has. A key-results headline can call `buildDisplacementExtremes`
and `displacementExtremesView(data, { caseKeys: null, filter: "all", sort: "uy" })` and read
`rows[0]` without rendering the panel. It must state the exclusions beside any headline value.
Every output is frozen and deterministic for its inputs. The panel derives once per identity of
(`model`, `result`, `standing`) and applies the view per control change.

## 8. Claims fence

The panel lists the largest and smallest emitted movements over the chosen solved cases. It
records and implies no clearance, travel adequacy, acceptance, issuance or compliance. It supplies
no limit and compares nothing with one, and it writes nothing.

## 9. Tests

- `apps/desktop/src/features/results/displacementExtremes.test.ts` covers the derivation: the
  family list against the schema; case order, labels and the cases with movement rows; signed
  extremes with governing case and row id; ties in case order and 0 against -0; mixed units;
  withheld cases under mixed standing, ineligible standing and no health view, with counts and no
  values kept; a range envelope listed, not choosable and out of the extremes; case subsets
  (chosen, not chosen, ignored keys); each filter (all, supports, hangers by family and by hanger
  record, each family, family not stated, a preserved unrecognized family); nodes without supports;
  the sort with ties, unranked nodes and mixed units; rows in another frame and rows of nodes not
  in the model; the unavailable states; and the bundled invented reference.
- `apps/desktop/src/features/results/DisplacementExtremesPanel.test.tsx` covers the panel: the
  display rule in cells, titles and headers with no stored value at full precision in the markup;
  per-cell units when a column's units differ; the case chooser with withheld and envelope reasons;
  the no-admitted-case state; the filter and the sort with `aria-sort`; paging and its bound;
  selecting a node by click and keyboard; the selected-node mark and its page; a selection made
  before results exist (a 123-node model then shows nodes 101–123), changed while results are
  absent, hidden by the filter or made while no case is chosen, each shown when its row appears;
  manual paging kept after the move; under StrictMode, the selected node's page on mount, again
  after the panel is hidden and shown, and in the mount's one commit, and manual paging kept
  across an Activity hidden and shown; a custom test-id prefix; and one derivation per input
  identity.
- `apps/desktop/src/features/report/springHangerSupport.test.ts` pins the shared hanger rule over
  every schema `SupportFamily` value and an absent, null, empty and unknown family, crossed with
  every `SpringHanger.hanger_type` value, no hanger record, a record without a type and an empty
  and unknown type, with a family and type padded by spaces, U+0085, U+FEFF or U+001C, and a type of
  U+0085, U+FEFF or U+001C alone. The shared function equals the solver's classification, an
  explicit table read from the Rust whose source the test checks is unchanged; its trim equals
  Rust's `White_Space` set over every BMP code point and astral samples, a set hard-coded from a
  rustc probe; explicit, family and fallback readings each count a U+0085-padded value and not a
  U+FEFF- or U+001C-padded one; the summary's variable and constant-effort split follows the same
  reading; the cases the correction changed from the report's earlier rule are listed exactly;
  A8's support flag and "Hangers" filter give the same answer for every support; and the report's
  two files call the shared function and keep no inline copy.
- Native smoke (`apps/desktop/native-smoke/driver.js`, `run.mjs`): structure only, no values, and
  the committed `expected.json` is unchanged. After each solve the panel is present, and either it
  lists node rows with twelve extreme cells each and a page line, or no case is admitted and it
  states that with the withheld cases disabled.

## 10. Remaining items

- **D2: export of the table**, an output surface under `outputPolicy.ts`.
- **Comparing with user limits.** A hanger's `travel_range` or `movement_limit`, or a
  user-entered clearance, could be shown beside the extremes. That needs a statement of meaning,
  and clearances need a model-document field.
- **Nonlinear behaviors in the filter.** Gap and one-way supports act as stops. The filter could
  split the nonlinear family by stated behavior.
- **Display units.** The panel could follow the session's display-unit preference, as A2 and A3
  could.
- **Nodes beyond the model's list.** Rows of such nodes are counted, not shown.
