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

`collectSelfWeightEvidence(pipeRefs, run, { componentIds })` runs the generator. If every pipe is
ready, one call covers them all. Any one refused pipe makes the generator refuse the whole request.
A refusal is split only when every one of its diagnostics is known to belong to one requested pipe
(`isPipeLocalDiagnostic`, an allow-list that fails closed). Such a refusal is halved, and halved
again, until each is held by a single pipe, which is then not counted with the generator's
diagnostics.

**The pipe-local allow-list.** A diagnostic is pipe-local only when its code is one whose condition
`generate_operations` evaluates for one selected pipe, its section or its mapped component, so that
it recurs whenever that pipe is requested, alone or with others:
`SELF_WEIGHT_INPUT_INVALID`, `SELF_WEIGHT_RANGE_INVALID`, `UNIT_CONVERSION_UNAVAILABLE`,
`SECTION_REFERENCE_INVALID`, `SECTION_BINDING_UNSUPPORTED`, `SECTION_REFERENCE_CACHE_STALE`,
`PIPE_DIMENSION_INVALID` and `RIGID_COMPONENT_MAPPING_CONFLICT`. Its `affected_objects` must then
name exactly one requested pipe, or be exactly one per-pipe section field (`outside_diameter`,
`wall_thickness`, `mill_tolerance`, `material_density`, `contents_density`,
`insulation_thickness`, `insulation_density`, `corrosion_allowance`; the generator names the field,
not the pipe, when it does not normalize) with an input, range or unit code, or be exactly one
component with `SELF_WEIGHT_RANGE_INVALID` (the component's own centre-of-gravity, force and moment
range checks). Everything else is not pipe-local: any other code, including future ones; no
reference; a request field (`gravity`, `gravity.axis`, `case_id`, `label`, `provenance`,
`source_model_hash` or `pipe_refs`); two requested pipes; and `SELF_WEIGHT_INPUT_INVALID` naming a
component, which is the generator's "generated primitive identity collision" of two components that
share an identity, even when a pipe shares that identity too.

**Any other refusal is a refusal of the whole request.** The collector stops at once, never splits,
and the summary is unavailable: no total and no centre of gravity, and the panel shows the
generator's diagnostics. An engine error or a refusal with no diagnostic stops it too. Attributing a
cross-entity refusal to entities and excluding them was considered and not adopted: the generator
names the shared identity, not the pipes, and one of its pipes would have to be chosen.

**The joint request.** The generator returns its selection refusals (pipe provenance, end nodes,
span and a pipe's own load identity) before it reads any component, so a pipe-local refusal can hide
a conflict between two other pipes' components. When halving has accepted the pipes in more than
one part, one last call therefore requests every accepted pipe together, and the summary is built
from that one plan. If the generator refuses it, the refusal is of the whole request ("The
self-weight generator refused the pipes in scope taken together, though it accepted them in parts;
…"). So every weight summed comes from one plan the generator accepted for exactly the counted
pipes. A conflict between a refused pipe's component and a counted pipe's is not shown until the
refused pipe's own refusal is resolved, when it refuses the whole request.

For k refused pipes among n, the collection takes about 2k·log₂n calls, and at most 2n (2n − 1
for halving and one joint call).

### 3.3 Entity weights and positions

`buildWeightSummary` takes each sent pipe in scope order.

- **A component record for the pipe.** The weight is `weight_N` (N). It acts at the component's
  node plus `center_of_gravity_m` converted to the model's length unit.
- **Otherwise, a pipe record.** The weight is |intensity| × L, in the intensity's force unit
  (`N/m` gives `N`). L is the straight distance between the pipe's end nodes, converted to metres.
  The weight acts at the span's midpoint. The intensity carries the entered gravity's sign, so its
  magnitude is taken. |intensity| × L is the resultant of a uniform load on a straight member, and
  the solver's load preparation gives the same resultant: `add_uniform_element_loads`
  (`core/product_physics/src/lib.rs`) forms a straight member's equivalent nodal loads with
  `equivalent_global_nodal_loads_with_spans_formed` (`core/solver/straight_pipe/src/lib.rs`), w × L/2
  at each end with equal and opposite fixed-end moments, so the resultant is w × L at the midpoint.
  This step adds the length. It is not a new mass or weight rule.
- **Otherwise, the pipe is not counted.** The panel states why: the generator's diagnostics for a
  refused pipe, or "returned no weight".

An entity is also not counted, with its reason, in any of these cases: its weight is zero,
negative or not finite; an intensity unit is not per metre; an end node, a component node or the
centre-of-gravity offset is missing; or a component's position (node plus offset) is not finite. A
span's midpoint is (a + b)/2, or a/2 + b/2 where a + b would leave the binary64 range. A pipe or
component weight is never assumed.

**Components not included.** They are read from the model, in every scope, independently of what
the generator's plan states. A component with a stated `geometry.weight`, or one the generator's
warnings name, is listed with its reason unless its weight is counted, when it maps
(`geometry.rigid_pipe_ref`) or names as a bend span (`geometry.bend_pipe_ref`) a pipe in scope, and
every such component in the whole-model scope. The reasons, in order: the generator's warning; its
mapped pipe is a bend span; it names a bend span; its mapped pipe is not counted (refused, or
otherwise listed with the pipes not counted); its mapped pipe's own distributed weight is counted
instead (for example a `not_solver_consumed` component, which the generator honours with no
warning); otherwise its weight is applied to no counted member. The summary does not include a
weight the generator does not apply.

### 3.4 Sums and centre of gravity

The panel shows:

- **Category totals.** The pipes' weights and the components' weights, each summed per unit in
  scope order.
- **Total.** The sum over every counted entity, formed only when all of them share one unit.
  Otherwise it states "Units differ (…); no total or centre of gravity is formed."
- **Centre of gravity.** Σ(wᵢ·pᵢ)/Σwᵢ for each global axis, over the same counted entities, in
  the model's length unit. It is formed only when the total is formed. Otherwise the panel states
  why. With nothing counted, it reads "No weight is counted; no total or centre of gravity is
  formed."

**Range.** Each weight is finite and positive (§3.3), but sums and products can still leave the
binary64 range. The derivation refuses rather than shows a non-finite or underflowed value:

- If any category total or the total is not finite, the summary is unavailable: "The summed
  weight is outside the representable range; no total or centre of gravity is formed."
- The centre of gravity is formed range-safely. Every weight is first scaled by the one power of
  two 2^s that brings the largest weight into [1, 2), and Σ(2^s·wᵢ·pᵢ)/Σ(2^s·wᵢ) is formed. The
  scaled sum of weights cannot overflow, and a subnormal weight no longer underflows its products:
  a span from 0 to 10⁻²⁰⁰ m weighing about 5.969 × 10⁻³¹⁰ N has its centre of gravity at
  5 × 10⁻²⁰¹ m, not 0.
- Scaling a weight is exact only when its exact scaled value 2^s·wᵢ is at least 2⁻¹⁰²², the
  smallest normal binary64 value. Below that it is rounded, and it can round up to 2⁻¹⁰²² and pass
  a range test: 2 N beside 2·2⁻¹⁰²² − 2⁻¹⁰⁷⁴ N halves the smaller to 2⁻¹⁰²², and its altered moment
  can cancel to exactly 0. So a scaled weight is used only when it is normal and scales back to the
  stated weight; that holds exactly when 2^s·wᵢ ≥ 2⁻¹⁰²², that is, unless the weights are about
  2¹⁰²² apart. Otherwise no centre of gravity is formed: "The counted weights are about 2^1022
  apart or more, so the smaller cannot be scaled exactly within the normal binary64 range; no
  centre of gravity is formed."
- A product with a nonzero coordinate, or a nonzero moment's quotient, whose magnitude is at most
  2⁻¹⁰²² is refused ("A weighted position falls below the normal binary64 range, where it would
  lose precision; …"). Rounding is monotone, so a larger rounded value had a normal exact value.
  A value equal to 2⁻¹⁰²² may have been rounded up from below, so it is refused even when exact.
  This needs weights about 2¹⁰²² apart or coordinates near the subnormal range.
- Sums of the accepted products are rounded in the normal range, and a sum whose exact value is
  subnormal is exact, so no further step rounds below the normal range. A moment or result that is
  not finite is refused ("The centre of gravity is outside the representable range; none is
  formed."). In every refusal the finite total is still shown, because it does not depend on the
  moments.
- The weights and positions themselves are formed as §3.3 states (an engine weight, a span
  length times an intensity, a midpoint, a node plus an offset), each by ordinary binary64
  operations; the guards above cover the centre-of-gravity arithmetic on them.

**Agreement with the unscaled formula.** An accepted step is an ordinary binary64 operation on
exactly scaled values, and a power of two commutes with rounding in the normal range. So when every
product, sum and quotient of the unscaled formula Σ(wᵢ·pᵢ)/Σwᵢ is zero or normal and finite, the
centre of gravity is bit-for-bit the unscaled formula's result. Ordinary models are of this kind,
and their results do not change. Otherwise the result is the scaled formula's or a stated refusal:

- unscaled moments that overflowed with a finite total had no centre of gravity before and now have
  the correct one;
- products that underflowed gave a wrong one before and now give the correct one or a refusal;
- weights about 2¹⁰²² apart are refused.

A scaled moment can also leave the range where the unscaled one does not, when the largest weight
is below 1 and a coordinate is near the binary64 maximum; that is refused, never shown.

**Partial.** The summary is partial when a scope pipe is not counted, a component weight is not
included or a scope reference is unresolved. `countedPipeCount` is the scope pipes whose weight (or
their component's) is counted, and `partial` says whether anything is left out. The total and
centre of gravity are then those of the counted weights only.

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
and the joint request bound the calls at 2n when some pipes are refused (§3.2). The panel renders three category rows and one
centre-of-gravity row. Each list of pipes not counted or components not included shows at most
`WEIGHT_SUMMARY_LIST_LIMIT` = 50 entries, then "and n more". The panel computes when the user asks
("Compute summary"), not on every model change.

## 6. Display, placement and interaction

All values follow the report display rule, `displayQuantity` and `displayQuantityParts` in
`features/report/reportQuantityDisplay.ts` (#1655). That covers weights, centre-of-gravity cells
and their titles, and the echoed gravity. The centre-of-gravity header carries the length unit.
The panel never throws while rendering: a value the rule cannot print (only a non-finite one, which
§3.4 already refuses) reads "outside the representable range".

`WeightSummaryPanel` (`apps/desktop/src/features/results/WeightSummaryPanel.tsx`) sits in the
Results section below A8's panel.

- **Controls.** A Scope select, Gravity value and unit inputs, a Gravity direction select (Global
  X, Y or Z, with none chosen at first) and "Compute summary". Missing or invalid gravity gives an
  alert and no call.
- **Currency.** A summary is shown only while its model (by identity), scope, scope pipes,
  unresolved scope references and gravity inputs are unchanged. A selection that gains a reference
  naming no pipe therefore makes the summary stale. Otherwise the panel says "The model, scope or gravity has changed
  since the last summary; compute it again." A later computation supersedes an earlier one that is
  still running.
- **Summary.** A table labelled "Weight summary: {scope}", with Category, Counted and Weight
  columns and rows for pipes, components and total. Then the centre-of-gravity table (X, Y, Z) or
  the reason none is formed, and a basis line: the gravity and direction, how pipe and component
  weights are taken, the contents and insulation counts, and that the generator gives no
  metal/contents/insulation split. Then either "Every pipe in scope is counted." or a disclosure
  "k of n pipes not counted" with each pipe's reason, the unresolved scope references, and a
  disclosure "m component weight(s) not included" with each reason. Both disclosures are open. The
  boundary sentence closes the panel.
- **Partial labels.** The labels "Total" and "Centre of gravity" are used only when nothing is
  left out. A partial summary (§3.4) has a visible note above the table, "Partial: {qualifier}. The
  total and centre of gravity are those of the counted weights only; what is left out is listed
  below.", the total row "Partial total ({qualifier})" and the centre-of-gravity header "Centre of
  gravity of the counted weights (partial)". The qualifier is "c of n pipes counted", then "; m
  component weight(s) not included" and "; u unresolved scope reference(s)" when there are any. The
  bundled demo reads "Partial total (1 of 4 pipes counted)".

## 7. Composition contract (A6)

```ts
// weightSummary.ts
export function weightSummaryScopes(model: PreviewModel, selectedPipeRefs?: readonly string[]): WeightSummaryScope[];
export function parseWeightGravity(input: { value: string; unit: string; axis: string }): { gravity: WeightGravity } | { error: string };
export function planWeightSummary(model: PreviewModel, scope: WeightSummaryScope): WeightSummaryRequestPlan;
export function isPipeLocalDiagnostic(item: EngineDiagnostic, requested: ReadonlySet<string>, componentIds?: ReadonlySet<string>): boolean;
export function collectSelfWeightEvidence(pipeRefs: readonly string[], run: SelfWeightEngineRun,
  options?: { componentIds?: Iterable<string> }): Promise<SelfWeightEvidenceCollection>;
export function buildWeightSummary(input: { model; scope; gravity; request; collection }): WeightSummary;
export function computeWeightSummary(input: { model; scope; gravity; hash; generate }): Promise<WeightSummary>;
// WeightSummary = { state: "unavailable", scope, reason, diagnostics }
//   | { state: "available", scope, gravity, lengthUnit, entities, pipes, components, total, centreOfGravity,
//       note, notCounted, componentsNotIncluded, countedPipeCount, partial, contentsCount, insulationCount,
//       engineCalls }

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
counted and components not included, and when `partial` is true it must label them partial as §6
does. All outputs except the engine call's are frozen and
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
  - collection: one call when ready, halving to single refused pipes and one joint request with the
    call count, the pipe-local allow-list (accepted and refused diagnostics, fail closed), a refusal
    that is not pipe-local never split, a joint request refused as a whole, and a stop on
    request-level refusals and engine errors;
  - sums and centre of gravity on invented geometry with a known answer;
  - metre, millimetre and foot models, and a refused unknown length unit;
  - nothing counted (no centre of gravity, with the note);
  - zero, negative and non-finite generator weights, and a signed intensity;
  - mixed units (no total, with the note);
  - pipes not counted (bend span, refusal, no evidence) and components not included in the model,
    line and selection scopes (warning, honoured, refused pipe, bend span, mapped bend span);
  - partial and unqualified summaries;
  - range: an overflowing total (unavailable), a subnormal span's centre of gravity, mixed
    magnitudes, a coordinate whose product overflows, and bit-for-bit agreement with the unscaled
    formula on ordinary sets;
  - inexact scaling refused: the weight 2·2⁻¹⁰²² − 2⁻¹⁰⁷⁴ beside 2 N (refused, total shown),
    exactly scaled weights at 2·2⁻¹⁰²² and just above it (centre given, bit-for-bit the unscaled
    formula's), and a product rounded up to 2⁻¹⁰²² that would cancel a moment to 0 (refused);
  - a selection subset, and the unavailable states;
  - two tests through the actual candidate engines. The bundled demo: valve C-130's 420 N stands
    for `pipe:P-130` at (7.72, 2.42, 0) m, the other pipes state no density, 7 calls. An invented
    two-pipe millimetre model with contents and insulation, checked against hand areas. On an
    invented three-pipe model: two components sharing identity `V` on different pipes refuse the
    whole request (each pipe alone gives 30 N), the same behind another pipe's own refusal (joint
    request), a member mapped by two components split as pipe-local, and a `not_solver_consumed`
    valve listed in the model, line and selection scopes. Codex r2's case: components of 2 N and
    2·2⁻¹⁰²² − 2⁻¹⁰⁷⁴ N at X = −2.2250738585072014 m and 10³⁰⁸ m under gravity along global X,
    where the engine accepts the request and the centre of gravity is refused with its reason while
    the 2 N total is shown.
- `apps/desktop/src/features/results/WeightSummaryPanel.test.tsx`:
  - the gravity alert, with no call;
  - the display rule in the weight and centre-of-gravity cells, with no full-precision value in the
    markup;
  - the basis line, the not-counted disclosure, and staleness on gravity and model changes and on a
    selection gaining an unresolved reference;
  - partial labels for a pipe, a component weight and an unresolved reference left out, unqualified
    labels when nothing is, and an out-of-range total stated instead of thrown;
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
