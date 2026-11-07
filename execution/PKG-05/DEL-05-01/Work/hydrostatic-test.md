# Hydrostatic test condition: mechanics and user-workflow design

**Row:** RT-A6 (Must; OC-5 (a): "A flagged test condition with test pressure,
test contents density and spring supports held, marked as a test case in
results and reports"). **Checked source:** main `b06e1da32` (#156).
**Step 0:** this revision designs the mechanics and persistence contract; it
implements no production behavior and lifts no refusal. It replaces the
`8cab1b3e5` outline in this same file. Numerical meaning and changed commitment
meaning require independent scrutiny of the exact old/new text before merge.

The finish condition is one authored hydrostatic case with the user's test
pressure, its selected test contents and spring/constant-effort devices held,
saved and reopened without losing those choices, then solved with a clear test
mark in results, navigation and reports. A no-hold or plain-EX-only stage is an
internal implementation step, not this workflow's completion. No code-derived
test factor, test-fluid default, tolerance adoption, compliance, acceptance or
release follows from this design. The release-target criteria are unchanged.

## 1. Stored input and current behavior

The existing model-document 0.9.0 member is optional, closed and unchanged:

| Member of `load_cases[].test_condition` | Required value | Meaning |
|---|---|---|
| `kind` | `"hydrostatic"` | This is a hydrostatic test case, explicitly marked in its output. A case label alone has no mechanical effect. |
| `held_supports` | `"springs_and_constant_effort"` or `"none"` | The first replaces every eligible spring/variable-spring/constant-effort device on its acted DOFs; the second preserves active device behavior. |

Both fields must be authored. Missing, null, unknown and malformed inputs keep
the closed-record rules; `kind` does not default `held_supports`. The eventual
desktop hydrotest template explicitly writes `springs_and_constant_effort`.
Test pressure is the user's `pressure_regions[].pressure`. Pressure from
`condition_use.pressure` remains dependent on separate S4 work and is not
required or authorized by this contents/hydrotest design.

At the checked source, `operating_conditions::judge` blocks every test case
with `LOAD_CASE_TEST_CONDITION_UNSUPPORTED`; EX condition use is also blocked.
The applier's `create_load_case` refuses `test_condition`, and contents use is
refused in both condition authoring and resolution. (H1, §8, since admits an
explicitly selected contents use consumed by generated self-weight; the test
case, EX non-contents use and every other refusal here are unchanged.) Hanger sizing and MBF have
their own test-case refusals. None changes in Step 0. Readers must not silently
turn a stored test case into an ordinary case.

## 2. Reference geometry and what held means

`case_state/input.rs::ReferenceConfigurationInput` names the stress-free
installed state. Its sole geometry selection, `AuthoredModelGeometry`, is the
model's own normalized nodes, connectivity and pipe frames. The current motion
meaning is `absolute_reference_displacement`; `case_state/resolve.rs` converts
translations to m and rotations to rad and supplies prescribed global DOFs.
It does not replace a node's plant coordinates with that displacement.

Accordingly, the selected hydrotest hold is **zero global displacement or
rotation from the as-modelled authored geometry** on each device's acted DOF.
It is not an absolute plant-coordinate zero, an increment from a previously
solved state, a spring's entered cold load, or a device datum inferred from
metadata. The reference remains the authored geometry on both plain EX and
load/reference-state EX. A nonzero fit/free strain or an admitted motion at
another restraint does not move this held-device reference. No predecessor
ordering or `position_source: predecessor_value` is introduced.

For a held device, remove both its stiffness contribution and its active
force/preload action, including any active-spring base-motion force term. Add
its prescribed rigid replacement at the chosen zero reference value. No spring
or constant force remains in parallel. The active branch preserves the law
actually admitted by the engine; this design does not implement variable-spring
preload. On the checked source, installed/cold/hot variable-spring loads remain
review metadata with `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD` warnings.

Reaction is the support-on-pipe reaction from the **replacement** system's
complete unreduced equations, `r = K_replacement u - f_active`, at the constrained
DOFs. It retains the original support, node and acted-DOF identity. A held CE
support publishes a held reaction, never an additional active CE applied-load
row for its suppressed force. Existing reaction signs and qualification rules
remain authoritative. Inactive support semantics are not added.

R-4 §4.9 requires this complete replacement. Its older §4.4 statement retaining
spring stiffness is historical and cannot serve as this implementation's
warrant: at zero displacement its residual spring term can be numerically
invisible, but the spring law is still required to be absent from the plan.

## 3. One per-case lowerer and complete device plan

DEL-05-01 owns condition/test lowering and contents; RT-A3 owns the callable
EX device, assembly, reaction and ledger seam (R-4 §4.9). The planned
`case_devices::resolve_case_devices` consumes the model, immutable original
case/reference context and normalized participation requests and returns one
complete `ResolvedCaseDevices`, or named blocking diagnostics and no usable
partial plan. These are internal API requirements, not a new serialized format
or an implemented Rust signature.

The plan binds the case, authored/reference geometry and original support
records, and contains:

- active spring entries and active force/preload/base-motion actions;
- replacement rigid/prescribed constraints with global DOF and canonical
  m/rad value, retaining the authored support direction in source evidence;
- original support/node/component identity, active/held status, and the exact
  device contributions suppressed by each replacement.

Assembly, factor/cache identity, active load ledger, solve, recovery and
publication all consume this same plan. A change in participation cannot reuse
a stiffness/factor formed with the old active spring list. Prescribed values
and their reference identity remain bound to the case RHS and evidence.
`prepare_boundary` currently precedes the case loop, `form_basis_stiffness`
uses its spring list, and both `add_constant_effort_support_loads` and
`append_constant_effort_support_results` consume model-level devices. All these
connections must be corrected before a held case is admitted; a new prescribed
value alone is insufficient.

One load-case lowerer supplies this plan for plain EX and load/reference-state
EX. Plain EX may use an internal installed-geometry context; no new model field,
second lowerer, alternate lock format or 0.10.0 step is introduced. On the
load-state route it reconciles the existing `support_states[].participation`
records with the test request before either can act.

### 3.1 Identity, position and conflict rule

For `held_supports: springs_and_constant_effort`, derive the complete set of
eligible original device DOFs at zero. An explicit locked record can coalesce
with this derived request only when it names the **same unique original
support**, hence the same model node, and exactly its complete acted-DOF set,
with no repeated/missing/extra component. Every position must be an entered,
finite value of the correct displacement/rotation dimension that normalizes
exactly to zero. Positive and negative zero are the same mechanical target;
authored bytes/sign and direction remain in provenance. No numerical tolerance
or vector normalization is invented for this consistency test.

A different support identity at the same node is not interchangeable. Existing
unknown-support, duplicate-support and overlapping-DOF refusals remain; equal
values do not create authority to merge distinct support records. Wrong/nonzero
positions, predecessor positions, an explicit active-vs-held contradiction,
or extra `boundary_motion`, `base_motion` or `device_reference` on the held
entry refuse with the planned `HYDROTEST_DEVICE_STATE_CONFLICT` (or the existing
more specific input diagnostic). Even zero extra motion is not another source
of a held law. With test `held_supports: none`, an explicit lock conflicts;
active devices retain their ordinary semantics. Array order never decides.
This does not admit locks outside hydrotest by itself.

RT-A3 S6 retains its separate active-motion work: admitted rigid/nonlinear
prescribed motions, nonlinear ground offsets and base terms on **active**
springs may be nonzero. Held replacements must not retain those terms; refusing
extra motion on a held record does not refuse S6's valid active records.
Nonlinear support admission and its current confirmation rules remain RT-A3's.

### 3.2 Directions

Preserve the checked admission boundary: EX still refuses genuine skew with
`SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED`. Exactly axis-aligned authored
vectors use only the existing lowering, retaining their source identity. Do
not substitute a convenient global axis or normalize the stored direction.
A future genuine skew hold needs its own admitted constraint/reaction proof;
this design supplies none and does not label that open work an accepted limit.

## 4. One contents resolver with an explicit source context

The planned public crate callable `case_contents_density(model, source_context,
pipe)` supplies the same per-case contents selection to self-weight, regeneration,
seismic generation where admitted, and eventual named-state modal mass. Its
source context names an authoritative authored case and the selected
`condition_use.contents_density` key, with the pipe's assigned set and condition
value. It is an immutable validated selection, not a generated target name,
case label, guessed key or hash-only inference. An ordinary named-case wrapper
may obtain this context by exact case id; it must not duplicate the resolver.

The result carries the resolved optional density and its source. An explicit
selection resolves the assigned set/key for every consumed pipe and requires
an explicit finite nonnegative density in supported units, normalized by the
existing units authority. Zero is an authored value, not absence. Missing set,
key, density or invalid units refuse by name. The normalized density, selected
case/use/set/key and relevant authored/source quantities bind its provenance.
Explicit contents can apply to ordinary cases as well as test cases. This
alone admits no EX condition pressure, temperature, added weight or wind use.
PF test cases remain refused even when their contents selection is valid.

With **no authored contents use**, retain the current section-density behavior,
including absent and zero density distinctions, operation order, mass methods,
create-case behavior and exact legacy dependency/evidence bytes. Do not add an
empty/null context block to old records or stale every existing self-weight
load. Existing nominal metal area, reduced EX pressure bore, mill/corrosion
operands, insulation and component-substitution/eligibility rules remain the
mass basis; this design invents no fluid volume or extra component weight.
A source that the selected mass path cannot consume must be exposed/refused by
name rather than reported as applied. Set `added_weight` remains an additive
primitive, not a second contents/self-weight lineage.

### 4.1 Source case, created target and inspection scratch are distinct

The checked `self_weight::generate_operations` creates a **new** load case and
rejects an existing target id. Inspection clones the model then clears
`source.load_cases`; `inspect_generated_self_weight` also creates the scratch
`modal:weight-basis` case. Looking up that fresh target/scratch id cannot recover
the original selected contents. The following is part of the implementation
contract, not an added source hook in Step 0:

1. Capture and validate the immutable authoritative named-case/condition
   context **before** clearing cases, copying a subset or renaming to scratch.
   Context must bind the actual selected use and pipe assignments/values.
2. Pass that same context through the existing physical mass computation and
   generated evidence. A missing or inconsistent explicit context produces
   planned `CASE_CONTENTS_CONTEXT_MISSING` or `CASE_CONTENTS_CONTEXT_CONFLICT`;
   it never silently falls back to section density.
3. For a create-case plan using selected contents, persist that selection
   explicitly on the created target's existing `condition_use.contents_density`
   member. Bind generated evidence to the final adopted target identity while
   retaining the original source-case identity as origin provenance. The
   adopted case then supplies its own authoritative authored selection for
   later inspection; the origin is not an implicit live case dependency.
   Source selection and explicit target selection must agree before adoption.
   A test flag/held choice belongs to the explicitly authored target payload,
   not an automatic copy of an unrelated source case's other mechanics.
4. An existing-case refresh uses that case's captured context. A scratch name
   never replaces the authoritative source identity in provenance or modal's
   weight-basis comparison. Regeneration resolves current authored selection
   using the same callable and compares the bound source/value projection.
5. Absence of a selected contents use takes the unchanged legacy branch,
   including creation of a new case. No new lookup requirement is imposed on
   that legacy create-case path.

Changing the selected case use, pipe assignment, condition value or relevant
mass dependency makes the generated load stale; a renamed scratch case does
not. Changing from one condition source to another with the same numeric
density still changes source provenance. An edit to an unrelated case/key is
not a physical mass dependency. Source hashes keep their current integrity
roles; none substitutes for these explicit bindings. All self-weight inspection
and eventual modal identity checks must validate the context before trusting
the generated value. Any provenance/reader extension must be implemented and
independently scrutinized together; an older reader must fail closed where it
cannot validate the selected source.

### 4.2 Gravity, generated loads and the source ledger

Contents density is mass input, not a new automatic force. The user supplies
signed gravity/direction and invokes the maintained self-weight operation;
one mass calculation produces the existing weight primitives. There is no
parallel water-weight generator and no additional solve-time application of
those same contents. A complete hydrotest workflow demonstrates fresh,
actually included contents weight, not a stored density or preview alone.
Explicitly selected but unconsumed contents must have a named diagnostic.

On plain EX, those generated primitives enter the normal case ledger. On
load/reference-state EX they must enter `analysis_state.load_sources` with
source identity, factors and exactly-once inclusion. H-a is a prerequisite for
**one saved document containing both hydrotest and static seismic**: admit and
ledger generated equivalent-static sources on that same route, with collision,
factor and inclusion evidence. Retain H-a through all stages. A plain EX stage
may precede it, but cannot close this workflow or introduce H-b/a second format.

## 5. Admission, marking and saved/reopened workflow

- Hydrotest is EX only. Keep PF test cases refused by name (planned specific
  `LOAD_CASE_TEST_CONDITION_PF_UNSUPPORTED` when EX is implemented); do not
  silently solve a pressure-free surrogate.
- A test case used directly or transitively in a combination is withheld with
  a named test-condition rule (planned `LOAD_COMBINATION_TEST_CONDITION_UNSUPPORTED`)
  until combination meaning is separately defined. Existing case identity,
  withheld-dependency and certificate coverage rules still apply. Mechanical
  "held device" does not mean combination publication "withheld".
- Keep `HYDROTEST_PRESSURE_UNSUPPORTED` for the legacy pressure primitive,
  with an actionable direction to author `pressure_regions` under the EX
  contract and the explicit `test_condition`. No test-factor calculation or
  pressure inference is added.
- Unsupported families/routes/readers keep their named refusals until their
  actual connecting path and evidence are implemented. The retained admission,
  MBF and other consumer contracts do not expand by assumption.

The eventual connected user path is: enter test pressure regions, assign the
explicit test-contents condition, choose held support behavior, generate/review
weight with explicit gravity, save, close/reopen, solve, inspect the marked
case and held reactions, and generate a marked report. Saved/reopened input
retains condition selections/assignments, pressure regions, test/held fields,
generated provenance and explicit participation records verbatim. Save/reopen
must not silently migrate data, apply pending drafts or re-author a choice.

Use the existing closed model members and operation/transaction contracts.
Authoring must persist `test_condition` only when its consumers are implemented,
and every preview/apply/undo/save/reopen path must preserve the same choice.
The result case record, navigator and report load-case section must explicitly
identify the hydrostatic test, its selected held behavior and applicable
pressure/contents/device evidence. A blocked test remains visibly a test; a
success label is not input-completeness or code-compliance certification. Row
and report schemas/readers must bind any new marking before publishing it;
Step 0 changes none. No native/App launch is authorized for this design unit.

## 6. Independent mechanics and connected regressions

Reference identity is **DEL-04-07 Work/route-unification-r4.md §4.7 (HYDROTEST)**;
its H labels are distinct from the historical pressure-contract H rulings.
Use invented independent generators, not product-generated expected numbers.
Common header data are r_o = 0.1 m, t = 0.01 m, E = 200 GPa, steel density
7,850 kg/m³, test-contents density 1,000 kg/m³ and explicitly entered gravity
9.80665 m/s²; these are reference inputs, not product defaults.

- H-1: two 6 m spans with the middle spring held as a rigid prop. Weight is
  `w = (902800199/4000000) π N/m`; the end/middle/end reactions are
  `(3/8, 10/8, 3/8) wa`, giving middle reaction
  `(2708400597/1600000) π N`.
- H-1b: the same device active at k = 2×10⁶ N/m. The independent reference is
  `R_mid = δ0/(f + 1/k)`, `δ0 = 5wL⁴/(384EI)`, `f = L³/(48EI)`, L = 12 m.
  Active/held contrasts must distinguish force law and replacement behavior.
- H-2: explicitly entered 15 MPa gives nominal Lamé bore hoop
  `p(r_o²+r_i²)/(r_o²-r_i²) = 2715000000/19 Pa`; reduced sections use their
  own explicit reduced bore/wall, not this nominal number by relabelling.
- H-3: a held constant-effort device supplies no active force; recover the
  rigid replacement reaction under the original identity. An active twin and
  a deliberate unsuppressed-force mutation must distinguish the two.

At zero prescribed displacement, leaving a diagonal spring term can be
numerically invisible. Therefore assert the complete plan, suppressed-law
ledger and replacement matrix/source identities as well as the numerical
references; do not fabricate envelope sensitivity. Repeat the reference with
the authored geometry translated away from the coordinate origin, with the
same zero displacement target and corresponding mechanical results.

Required connected regressions for subsequent implementation:

1. Plain EX and load-state EX use the same lowerer/plan, with all-or-none
   failures, deterministic identity/order and no mutated authored input.
   Hold both stiffness and force/preload; active S6 motions remain active.
2. Permute explicit/derived lock input order: equivalent exact locks coalesce;
   wrong identity/DOF/position, duplicates, extra held motion and active/held
   conflicts refuse. Preserve skew admission and authored directions.
3. Resolve ordinary/test contents from the same source. Missing/conflicting
   captured context refuses, including equal-density/different-source controls.
   Generate into a different fresh target, adopt its persisted selection, save
   and reopen, then inspect: fresh weight stays fresh at the same density.
   Clearing cases and using `modal:weight-basis` must not lose or change that
   context. Mutate a selected dependency to get stale; mutate an unrelated
   case/key without fabricating a mass change.
4. No-use documents keep legacy generation/dependency bytes and frozen-corpus
   numerical results unchanged. Contents-selected twins equal explicitly
   authored section-density twins in mechanics, with source provenance
   intentionally distinct. Preserve component eligibility and EX bore basis.
5. H-a: hydrotest and static seismic in one saved load-state document; every
   generated source is included once at its authored factor. Missing,
   duplicated, altered and unlisted source controls refuse. No double water
   weight or CE load; published ledger and reactions match the consumed plan.
6. Stored pressure/contents/held choices survive preview, operation apply/undo,
   save/reopen, native/Wasm transport and report generation. Blocked, solved
   and withheld-combination outcomes keep correct test markings. Do not infer
   this connection from separate component passes.

No new tolerance is selected. Numerical/persistence/permission changes receive
the required independent scrutiny and appropriate native/Wasm/source-reader
checks. Any memory registration change is derived from then-current integrated
source; any later AC-004 run needs fresh technical and App/resource clearance,
READY/brief deferral/GO and exclusive controls. Prior windows grant nothing.

## 7. Delivery order and ownership

Step 0 is this docs-only contract and its independent review. Subsequent source
work is released and sequenced explicitly around RT-A3's PP/NI S6 ownership.
A plain-EX stage may establish contents, pressure and marking first, but the
unit is not complete until actual held devices, saved/reopened input, report
markings and the H-a shared-document path are demonstrated. Full unrelated S4
condition pressure/temperature/added-weight resolution is not a prerequisite
for explicitly authored test pressure plus the narrow contents consumer.

DEL-05-01 owns the single lowerer/contents resolver, provenance/staleness,
authoring and user/result/report workflow. RT-A3 owns the complete device plan,
EX assembly/recovery/source ledger, H-a and the HYDROTEST mechanical prerequisite
references. RT-A3's delivered docs-only R-4 reconciliation (`4ad92c2ab`) is
integrated here with its authorship preserved, so the two governing contracts
are reviewed together; no S6 production ancestor is included. Necessary guide/theory prose
must distinguish the current refusals from this planned behavior. Step 0
provides no production hook, schema change, native entry or modal edit, and
changes no release-target criterion or accepted limitation.

## 8. H1: explicit contents selection (implemented)

H1 delivers §4/§4.1's contents consumer for **self-weight only**. It admits no
`test_condition`, held device, H-a ledger, preload, EX pressure/temperature/added
weight/wind use, schema version or model member; hydrotest is not complete.

- **Resolver.** `operating_conditions::contents` holds the one resolver.
  `CaseContentsContext::capture(model, case_id)` takes an owned snapshot of one
  case's `condition_use.contents_density`, the pipes' `condition_set_ref` and the
  sets, by exact id (never trimmed; absent `CASE_CONTENTS_CONTEXT_MISSING`,
  duplicated `CASE_CONTENTS_CONTEXT_CONFLICT`). `case_contents_density(context,
  pipe)` returns the set, key, authored quantity and the density normalized by
  the generator's own units authority, or the named refusal
  (`OPERATING_CONDITION_SET_MISSING`, `_SET_UNRESOLVED`, `_KEY_MISSING`,
  `_KEY_DUPLICATE`, `_VALUE_MISSING`, `_QUANTITY_INVALID` for a non-density unit,
  a non-finite or a negative value). Zero is authored, not absence.
- **Request.** The self-weight JSON request takes one optional, request-only
  `contents_source_case_ref` (`ContentsAwareSelfWeightRequest`, beside the
  unchanged closed `SelfWeightRequest`). Present and not a nonblank string is
  `SELF_WEIGHT_CONTENTS_SOURCE_INVALID`; a source equal to the new case id is
  `CASE_CONTENTS_CONTEXT_CONFLICT`; a source with no contents use is
  `CASE_CONTENTS_CONTEXT_MISSING`. Malformed input never becomes the legacy
  request. The Wasm export and the native command forward the same JSON to
  `self_weight_wasm::generate_plan`; hash and unknown-field guards are unchanged.
- **Generation.** The selection replaces each consumed member's section contents
  density in the one existing mass computation (`stable_mass_per_length`; PF wall
  and EX D-5 bore unchanged). A member whose component weight replaces its
  distributed weight cannot consume contents and is refused
  (`SELF_WEIGHT_CONTENTS_NOT_CONSUMED`); component eligibility is unchanged. The
  record's `mass_inputs.contents_density` is the selected quantity, and
  `contents_source` records method `case_condition_contents_density/v1`, the
  selecting case, the origin case, set, key, authored and normalized density and
  any superseded section density. `normalized_dependencies.contents_source` is
  `{condition_set, condition_key}`. The created case carries
  `condition_use.contents_density` equal to the source's key, so the target owns
  its selection after adoption; the origin is provenance only.
- **Inspection and refresh.** Each generated record is regenerated with its own
  case's selection, captured from the authored model before the clone clears its
  cases. Changing the target's key, a pipe's set or the selected value, or adding
  or removing the target's use, is `SELF_WEIGHT_INPUTS_STALE` (an equal density
  from another set or key included). Editing or removing the origin case, an
  unrelated set or key, or the superseded section density is not a dependency.
  A refresh keeps identity and origin. A missing selected value is invalid by
  name, never a section-density fallback.
- **Solve admission.** Contents alone is admitted on PF and EX; other EX uses
  stay `LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED`. A valid selected source with
  no generated contents weight gives `CASE_CONTENTS_WEIGHT_NOT_APPLIED`, a
  nonblocking warning explicitly saying no contents weight or gravity has been
  applied. It creates no force, and the source case remains in the solved model.
  Invalid sets/keys/quantities/context and stale generated inputs remain blocking.
  Contents plus unconnected seismic mass stays blocking as
  `LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`; authoring does not downgrade it.
  A nonempty warning vector never changes the PF plan into Err/no-plan.
  `lower` keeps only the contents selection (the case key, the sets' contents
  values and the pipes' references) when a case uses contents, so the
  post-lowering self-weight validation resolves the same selection; no solve
  body changed.
- **Unchanged and declined.** With no source the request, plan, evidence and
  create-case bytes are unchanged (pinned against the DEL-09-01 reference-system
  capture). The modal production entry is unchanged: selected contents leave
  context on the lowered model, so its existing explicit binder refuses the
  state case (`MODAL_STATE_CASE_UNAVAILABLE`); a modal
  without a state case keeps the section mass. The retained Direct/headless
  routes and their D1 admission still decline condition sets by name. The weight
  summary keeps the section-density plan. A contents-aware new EX target
  explicitly plans `pressure_regions: []`, which the real create operation
  preserves; it copies no source pressure or test flag. Legacy no-use case
  payloads remain unchanged. The positive static and transport proofs solve the
  complete applied/reopened source-plus-target document without deletion or a
  test-only pressure-field patch.
- **Repaired connecting proof (pre-review).** The complete source-plus-target
  model, unchanged after real batch apply and reopen, solves on PF and plain EX
  in both solver modes. Valid unused source selections remain authored with a
  warning and no inferred force. The positive proofs contain no origin deletion
  or post-generation target patch; intentional origin-removal independence is
  tested separately. The real panel/service/Wasm path covers PF and EX and
  retains the target's explicit empty regions. The modal production binder is
  unchanged; its deferral regression now asserts the existing
  `MODAL_STATE_CASE_UNAVAILABLE` gate exposed by the nonblocking static policy.
- **Evidence.** `core/product_physics/tests/h1_contents_self_weight.rs`
  (independent PF/EX closed forms `(902800199/4000000)π` and
  `(5686876335/25000000)π` N/m, a 6 m cantilever reaction, section-density twins,
  staleness, refusals, admission), `core/loads/self_weight_wasm/tests/contents_self_weight.rs`
  (transport, applier batch, save/reopen, refresh, solve, legacy capture bytes)
  and `apps/desktop/src/features/self-weight-authoring/SelfWeightContents.engine.test.tsx`
  (Conditions control and self-weight panel through the actual Wasm engines,
  atomic apply, project save/reopen and refresh inspection).
