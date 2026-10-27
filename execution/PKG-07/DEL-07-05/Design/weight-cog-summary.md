# Weight and centre-of-gravity summary

This design covers survey item A9. For the whole model, one line or the session's selected pipes,
the panel totals the weights of the pipes and components and gives their overall centre of
gravity. Paths are relative to `projects/chirality-piping/`. The standard claim fence applies
(F-PIP-2, §8). The summary restates the model's own weights as the self-weight generator gives
them. It is not a lifting, rigging or structural weight. It carries no allowance and it qualifies
nothing.

## 1. Basis, scope and governing deliverable

HELP_HUMAN adopted A9 under the owner's 2026-10-27 delegation (#1662). The CAEPIPE survey
(#1657, `execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md`) describes it as a fast
check on model mass that lifting and structural teams ask for. It touches FR-SOL-003 (weight on
straight pipe elements) and PRD §12.5 (load types).

**Governing deliverable: DEL-07-05.** The panel is a view in the Results section beside A2's, A3's
and A8's, and it computes no mass property. DEL-03-08 governs the section and mass-property
calculation, and DEL-05-01 owns the self-weight generator
(`core/product_physics/src/self_weight.rs`, `core/loads/self_weight_wasm`). Neither changes. The
generator's evidence is the input, recorded as a non-gating need on DEL-05-01 in
`deliverable.yaml`.

In scope: the scopes, the source (§2), the derivation (§3 and §4), the panel, its placement, and
the A6 contract (§7). Out of scope: export (item D2), and any weight the generator does not give
(§10).

## 2. Source of the weights

**Rule: the TypeScript computes no weight.** Each entity's weight comes from the self-weight
generator, the same code that writes a self-weight load case:

- `generateSelfWeightPlan` in `apps/desktop/src/services/selfWeightPlanService.ts`. It calls the
  native command `generate_self_weight_operation_plan` in the app, and the `self_weight_wasm`
  engine in the browser and in tests.
- The panel sends a full request: the scope's pipes, the entered gravity, the canonical model hash
  (`computeModelHash`) and a case id that names no existing case or primitive load
  (`weightSummaryCaseId`). It reads the returned plan's `source_evidence` and `warnings`. It never
  queues or applies the plan, so the model is unchanged.

The generator gives two kinds of record:

| Record (`method`) | Fields read | Meaning |
|---|---|---|
| `pipe_mass_per_length_times_explicit_axis_acceleration/v2` | `pipe_id`, `generated_payload.magnitude` (value, `N/m`), `contents_absent`, `insulation_absent` | The member's uniform self-weight intensity, mass per length times the entered gravity. The mass is pipe metal plus any stated contents and insulation, on the preview or exact (D-5) basis the document selects |
| `component_weight_at_center_of_gravity_global_offset/v1`, `wrench_part: "force"` | `component_id`, `component_node`, `omitted_pipe_id`, `weight_N`, `center_of_gravity_m` | A rigid-family component's normalized weight, which replaces the distributed self-weight of its mapped member (W-15 slice b). The offset is in global X/Y/Z from the node, in metres |

The generator's warning `SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED` names a component whose weight
it does not apply, with the reason. That member keeps its pipe weight.

**Breakdown available.** The generator gives one mass per length for each pipe: pipe metal, contents
and insulation together. It states whether contents and insulation entered. It gives each mapped
component's weight separately. The panel therefore shows two categories, pipes and components, and
counts how many counted pipes include contents and how many include insulation. The survey's
further split (empty pipe, insulation, contents, lining, added weights) is not available. A
TypeScript formula for those terms would restate the generator's mass rule, which this design
forbids, so they are remaining items (§10).

**No solve is needed.** The weights are model data through the generator. The panel works with no
result, does not read result rows, and neither result route's standing applies. It is hidden while
a reopened historical run is shown, like the other Results panels, because it describes the
current model.

## 3. Derivation

`apps/desktop/src/features/results/weightSummary.ts` holds the derivation. It has no React code.

### 3.1 Scopes

`weightSummaryScopes(model, selectedPipeRefs)` offers the following scopes:

| Scope | Pipes |
|---|---|
| Whole model | Every `pipe_segments` entry, in model order |
| Line (one per `model.lines` entry, W-13 §3.2) | The line's `pipe_refs`, in line order, each once |
| Selected pipes (offered only when there are any) | The session's selected pipes (`selectedPipeRefs`, from the ordered selection), each once |

A line or selection reference that names no model pipe is counted and listed. It is never sent to
the generator. The generator adds a component through its mapped member, so a line or selection
includes the components mapped onto its pipes.

### 3.2 Request and collection

`planWeightSummary(model, scope)` sets bend spans aside. A bend span is a pipe named by any
component's `geometry.bend_pipe_ref`. On a realized curved bend the solver integrates the
distributed weight over the arc (`distributed_load_treatment=arc_consistent_fixed_end_integration`
in `core/product_physics/src/lib.rs`). The desktop does not hold that arc length, and computing it
here would be a new geometry formula. Such a span is therefore not counted, and the panel names the
bend components. The rule is conservative: it also sets aside a geometry-only bend span, which the
solver keeps as a straight chord (§10).

`collectSelfWeightEvidence(pipeRefs, run)` runs the generator. If every pipe is ready, one call
covers them all. Any one refused pipe makes the generator refuse the whole request, so on a refusal
the set is halved, and halved again, until each refusal is held by a single pipe. For k refused
pipes among n, that takes about 2k·log₂n calls. The collector stops at once on a refusal of the
request or document as a whole: the `SELF-WEIGHT-*` model, request, hash and JSON codes, an engine
error, a refusal with no diagnostic, or a diagnostic whose `affected_objects` names a request field
(`gravity`, `gravity.axis`, `case_id`, `label`, `provenance`, `source_model_hash` or `pipe_refs`).
In that case the summary is unavailable and the panel shows the generator's diagnostics. Halving
relies on each pipe's record depending only on that pipe, its section and its mapped component, as
`generate_operations` computes them, so a set the generator accepts gives the same records as its
pipes one by one.

### 3.3 Entity weights and positions

`buildWeightSummary` takes each sent pipe in scope order.

- **A component record for the pipe.** The weight is `weight_N` (N). It acts at the component's
  node plus `center_of_gravity_m` converted to the model's length unit.
- **Otherwise, a pipe record.** The weight is |intensity| × L, in the intensity's force unit
  (`N/m` gives `N`). L is the straight distance between the pipe's end nodes, converted to metres.
  The weight acts at the span's midpoint. The intensity carries the entered gravity's sign, so its
  magnitude is taken. |intensity| × L is the resultant of a uniform load on a straight member, and
  the solver's load preparation gives the same resultant
  (`prepare_lumped_nodal_loads`, `core/loads/primitive_loads`: w × span split equally between the
  ends, so it acts at the midpoint). This step adds the length. It is not a new mass or weight rule.
- **Otherwise, the pipe is not counted.** The panel states why: the generator's diagnostics for a
  refused pipe, or "returned no weight".

An entity is also not counted, with its reason, in any of these cases: its weight is zero,
negative or not finite; an intensity unit is not per metre; or an end node, a component node or
the centre-of-gravity offset is missing. A pipe or component weight is never assumed.

**Components not included.** These are listed with their reasons: components that the generator's
warnings name, and, in the whole-model scope only, any component with a stated `geometry.weight`
that no evidence or warning names. The generator applies such a component's weight to no counted
member, so the summary does not either.

### 3.4 Sums and centre of gravity

The panel shows:

- **Category totals.** The pipes' weights and the components' weights, each summed per unit in
  scope order.
- **Total.** The sum over every counted entity, formed only when all of them share one unit.
  Otherwise it states "Units differ (…); no total or centre of gravity is formed."
- **Centre of gravity.** Σ(wᵢ·pᵢ)/Σwᵢ for each global axis, over the same counted entities, in
  the model's length unit. It is formed only when the total is formed, positive and finite.
  Otherwise the panel states why. With nothing counted, it reads "No weight is counted; no total
  or centre of gravity is formed."

Weights and positions come from the same generator call, so the summary and the self-weight case
the generator would write agree.

## 4. Units

- Weights are in the generator's units. Its pipe intensity is `N/m` and its component weight is
  `N`, so a total is normally in `N`. The derivation still handles mixed units (§3.4).
- Node positions are in `project.units.length`. A span's length is converted to metres, and a
  component's offset from metres to the model's unit, by the exact factors of
  `millimetersPerLengthUnit` (`apps/desktop/src/features/exportLengthConversion.ts`: m, mm, in,
  ft). This is the owner's 2026-10-26 allowance for routine engineering unit conversions. Any other
  length unit has no routine conversion, and the summary is unavailable with that reason.
- Gravity is a user entry: value, unit and global direction, exactly as the self-weight planner
  takes it. None is assumed (PRD §6.2; the generator refuses a zero or non-finite value). The
  generator normalizes the unit. The direction is required by the request. It changes only the sign
  and axis of the generated load, so it affects no reported weight or centre of gravity.
  Component weights are stated forces and do not scale with the entered gravity.

## 5. Large models

A clean model takes one generator call. Each call sends and hashes the whole model once. Halving
bounds the calls when some pipes are refused. The panel renders three category rows and one
centre-of-gravity row. Each list of pipes not counted or components not included shows at most
`WEIGHT_SUMMARY_LIST_LIMIT` = 50 entries, then "and n more". The panel computes when the user asks
("Compute summary"), not on every model change.

## 6. Display, placement and interaction

All values follow the report display rule, `displayQuantity` and `displayQuantityParts` in
`features/report/reportQuantityDisplay.ts` (#1655). That covers weights, centre-of-gravity cells
and their titles, and the echoed gravity. The centre-of-gravity header carries the length unit.

`WeightSummaryPanel` (`apps/desktop/src/features/results/WeightSummaryPanel.tsx`) sits in the
Results section below A8's panel.

- **Controls.** A Scope select, Gravity value and unit inputs, a Gravity direction select (Global
  X, Y or Z, with none chosen at first) and "Compute summary". Missing or invalid gravity gives an
  alert and no call.
- **Currency.** A summary is shown only while its model (by identity), scope, scope pipes and
  gravity inputs are unchanged. Otherwise the panel says "The model, scope or gravity has changed
  since the last summary; compute it again." A later computation supersedes an earlier one that is
  still running.
- **Summary.** A table labelled "Weight summary: {scope}", with Category, Counted and Weight
  columns and rows for pipes, components and total. Then the centre-of-gravity table (X, Y, Z) or
  the reason none is formed, and a basis line: the gravity and direction, how pipe and component
  weights are taken, the contents and insulation counts, and that the generator gives no
  metal/contents/insulation split. Then either "Every pipe in scope is counted." or a disclosure
  "k of n pipes not counted" with each pipe's reason, the unresolved scope references, and a
  disclosure of components with a stated weight not included. The boundary sentence closes the
  panel.

## 7. Composition contract (A6)

```ts
// weightSummary.ts
export function weightSummaryScopes(model: PreviewModel, selectedPipeRefs?: readonly string[]): WeightSummaryScope[];
export function parseWeightGravity(input: { value: string; unit: string; axis: string }): { gravity: WeightGravity } | { error: string };
export function planWeightSummary(model: PreviewModel, scope: WeightSummaryScope): WeightSummaryRequestPlan;
export function collectSelfWeightEvidence(pipeRefs: readonly string[], run: SelfWeightEngineRun): Promise<SelfWeightEvidenceCollection>;
export function buildWeightSummary(input: { model; scope; gravity; request; collection }): WeightSummary;
export function computeWeightSummary(input: { model; scope; gravity; hash; generate }): Promise<WeightSummary>;
// WeightSummary = { state: "unavailable", scope, reason, diagnostics }
//   | { state: "available", scope, gravity, lengthUnit, entities, pipes, components, total, centreOfGravity,
//       note, notCounted, componentsNotIncluded, contentsCount, insulationCount, engineCalls }

// WeightSummaryPanel.tsx
export function WeightSummaryPanel(props: {
  model: PreviewModel;
  selectedPipeRefs?: readonly string[];
  generate?: typeof generateSelfWeightPlan;   // default: the native command or Wasm engine
  hash?: typeof computeModelHash;             // default: the session's canonical hash
  testIdPrefix?: string;                      // default "weight-summary"
}): JSX.Element;
```

A key-results headline can call `computeWeightSummary` with the whole-model scope and read `total`
and `centreOfGravity`. Beside those values it must give the gravity and the counts of pipes not
counted and components not included. All outputs except the engine call's are frozen and
deterministic for their inputs.

## 8. Claims fence

The panel sums the weights the model states, as the self-weight generator gives them, and their
centre of gravity. It states no lifting, rigging, transport or structural weight. It adds no
allowance, contingency or dynamic factor, and it qualifies, accepts and issues nothing. It
changes no model, result or export and persists nothing.

## 9. Tests

- `apps/desktop/src/features/results/weightSummary.test.ts`:
  - scopes: whole model, lines with duplicate and unresolved references, an empty line, selection
    present and absent;
  - gravity parsing: missing, blank, zero, non-finite and unparsable values, and a bad axis;
  - bend spans set aside, and a case id that avoids existing case and load ids;
  - collection: one call when ready, halving to single refused pipes with the call count, and a
    stop on request-level refusals and engine errors;
  - sums and centre of gravity on invented geometry with a known answer;
  - metre, millimetre and foot models, and a refused unknown length unit;
  - nothing counted (no centre of gravity, with the note);
  - zero, negative and non-finite generator weights, and a signed intensity;
  - mixed units (no total, with the note);
  - pipes not counted (bend span, refusal, no evidence) and components not included (warning, and
    unnamed in the whole-model scope only);
  - a selection subset, and the unavailable states;
  - two tests through the actual candidate engines. The bundled demo: valve C-130's 420 N stands
    for `pipe:P-130` at (7.72, 2.42, 0) m, the other pipes state no density, 7 calls. An invented
    two-pipe millimetre model with contents and insulation, checked against hand areas.
- `apps/desktop/src/features/results/WeightSummaryPanel.test.tsx`:
  - the gravity alert, with no call;
  - the display rule in the weight and centre-of-gravity cells, with no full-precision value in the
    markup;
  - the basis line, the not-counted disclosure, and staleness on gravity and model changes;
  - line and selection scopes, unresolved references and a custom test-id prefix;
  - the generator's refusal;
  - the bundled demo through the actual candidate engines.
- Native smoke (`apps/desktop/native-smoke/driver.js`, `run.mjs`) checks structure only, and the
  committed `expected.json` is unchanged. After each solve the driver enters an invented gravity
  and computes. The summary must list three category rows, either a centre of gravity or the
  reason none is formed, and either every pipe counted or the pipes not counted. If not, it must
  state the generator's refusal.

## 10. Remaining items

- **Pipe metal, contents and insulation separately.** The generator's evidence would need to give
  each term's mass per length. That is a DEL-05-01 change to `self_weight.rs`, which holds the
  terms in `stable_mass_per_length`.
- **Lining.** No model field states a lining. It needs a model-document field and a generator term.
- **Added weights.** The model has no added-weight entity. A user-authored `weight` primitive load
  is a load-case load, not model mass, and the panel does not read load cases. An added-weight
  record (CAEPIPE's) would need a model field and a generator rule.
- **Realized bend spans.** The generator, or the solver's evidence, would need to give each span's
  integrated weight and centroid (the arc's centroid lies off the chord), so that curved spans can
  be counted. A narrower rule could also count geometry-only bend spans as straight chords.
- **Components with a stated weight that the generator does not apply** (not rigid-family, no
  mapping, or a weight condition unmet) are listed, not summed. Whether they should carry weight is
  the generator's rule, not the summary's.
- **D2: export** of the summary and its entity list, an output surface under `outputPolicy.ts`.
- **Display units.** The panel could follow the session's display-unit preference, as A2, A3 and A8
  could.
