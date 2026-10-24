# Model topology checks (W-8)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Purpose

PRD FR-KNOW-002 (Must): "Software detects disconnected routes, invalid branch
topology, and unsupported model topology." PRD §12.8: the engine shall "detect
duplicate or near-duplicate nodes". Before this change the product detected
duplicate ids (`DUPLICATE_ID`), zero-length elements, unrestrained systems and
mechanisms, and that a tee's referenced pipes exist and end at its node
(`BRANCH_MAPPING_INPUT_INVALID`). A model made of two parts, each restrained,
solved silently as two structures; coincident or nearly coincident nodes that
no element joined were not reported; a tee on a node that does not join three
pipes was not reported.

## Ruling

Every finding here is a **warning**. Nothing new refuses:
- a model whose parts are each restrained is solvable, and is sometimes
  intended (two independent lines in one document);
- coincident nodes may be intended (for example, two lines crossing without a
  connection);
- an unrestrained part already refuses through the solve's mechanism and
  restraint checks, which are unchanged. The warning names it, so the user can
  see which part has no support.

The checks run on both routes (PF and EX) because both go through the same
entry (`run_linear_static_preview_observed_inner`). They change no number.

**Reading of PRD §14.4** (owner, 2026-10-23: "I accept your other
recommendations for the matters you needed from me"). §14.4 lists "schema or
topology invalid" under "Blocking for model validity". That clause is read as
topology that prevents the mechanics from being formed, which already blocks:
- a pipe endpoint that names an unknown node (`MODEL_INCOMPLETE`);
- a part with no support (the solve's mechanism and restraint checks);
- an invalid exact-route pressure-region chain: branching, cycles or
  disconnected traversal (`PRESSURE_REGION_TOPOLOGY_INVALID`,
  `PRESSURE_REGION_INTERNAL_BRANCH` in `pressure_runtime.rs`).

A model that can be formed but is unusual (disconnected parts that are each
supported, coincident or near-duplicate nodes, a malformed tee) is a §14.4
"Warning". REQ-04-06-012 stands as approved.

## Checks

| Code | Where | Fires when |
|---|---|---|
| `MODEL_TOPOLOGY_DISCONNECTED` | `core/product_physics/src/topology.rs` | The elements (straight members other than connector-replaced spans, curved spans and objective connectors) form two or more connected parts. A node no element touches is not a part. One diagnostic names up to six parts, each by its first node in model order, its node count, and `supported` or `no support` (any linear or nonlinear support at one of its nodes). |
| `NODE_COINCIDENT` | `topology.rs` | Two distinct nodes have exactly the same position and no element joins them. |
| `NODE_NEAR_DUPLICATE` | `topology.rs` | Two distinct nodes lie no farther apart than the node-proximity tolerance τ (and are not coincident), and no element joins them. |
| `BRANCH_TOPOLOGY_INVALID` | `core/product_physics/src/validation.rs` | A `branch`, `tee` or `branch_connection` component's node joins a number of pipes other than three, or its header and branch pipe references name the same pipe. One diagnostic per component. The finding warns and does not refuse (REQ-04-06-012). When the solved envelope publishes intensified rows for the component, the message also names those sides ("its branch stress factors are still applied (header SIF to pipe:…, branch SIF to pipe:…)"); see "The applied-sides statement" below. |

**The applied-sides statement.** The owner ruled (2026-10-23) that topology
findings warn, not refuse, and that this warning "must state plainly that the
component's branch stress factors are still applied to the named pipes".
- **What is applied.** The branch stress factors reach the results only as the
  intensified rows that `preview_physics::render` publishes through
  `associations`: a positive finite user-entered header or branch SIF, solver
  consumption `mechanics_geometry_only`, a header or branch reference equal to
  the pipe, and the component at that pipe's end node, on straight members only
  (an arc member publishes none). No flexibility factor is required, and the
  topology does not matter. The retired SIF × flexibility rows
  (`branch_stress_modifier_for_pipe`) are published on no route.
- **Where it is stated.** Validation emits the warning before the build with
  its topology text only, so it appears in an envelope refused for any reason.
  `validation::state_applied_branch_factors` adds the statement once, where
  `run_linear_static_preview_observed_inner` assembles the solved preview
  envelope, after the non-finite check, which is the last refusal on the
  preview route. It reads the sides from that envelope's own
  `intensified_measures` records, one beside each published row, so it names
  exactly the sides published. A refused envelope never carries the statement.
- **Why there.** The other options were to compute the statement in the
  topology stage, where the arcs are known, or to emit the whole warning after
  the build. Both still precede refusals that publish nothing, such as a model
  with no support, a singular solve or a non-finite result. A stage-time
  statement would then claim factors on a refused envelope. Re-deriving the
  publication predicate there would also be a second copy that could drift
  from it. Emitting the warning late would lose it from envelopes refused
  earlier. Reading the published records avoids both problems.
- **Limits.** The exact route publishes no intensified row (and refuses tees),
  so its warning never carries the statement. A certified intensified recovery
  failure inside the publication loop adds a blocking
  `ELEMENT_FORCE_RECOVERY_FAILED` and leaves that row out; the envelope is still
  published. The statement names a side when at least one load case publishes
  a row for it, so a side whose row failed in one case and was published in
  another is named. The composite source finalization that can still fail
  after assembly runs only on the exact route.

The disconnected and node checks run on the built model, after unit
normalization, so positions are in metres. Each of those codes yields at most
one diagnostic naming at most six parts or pairs (`formation_guard::NAMED`), so
the diagnostic count does not grow with model size.

**Tolerance.** τ is an absolute length, 5 mm unless the model sets another
value, and a pair at exactly τ is near. The owner chose it (2026-10-23): "I
choose C, but my value is "5mm" and give the user the means to change it. No
computed by any 1e-4, just 5mm". The model sets its value in
`model_settings.node_proximity_tolerance` (a length with a unit; schema 0.7.0),
whose reader validates it (finite, positive, a length unit). The check reads
it through `PreviewModel::node_proximity_tolerance_m`. The solve refuses a
malformed value as a blocking diagnostic with the reader's code
(`MODEL-SETTING-TOLERANCE-NOT-FINITE`, `-NOT-POSITIVE`, `-UNIT-NOT-LENGTH`),
as the document validator does, so the check never runs on an invalid
tolerance.

τ replaced a relative tolerance (1e-6 of the diagonal of the nodes' bounding
box). A relative tolerance made the finding depend on parts of the model
unrelated to the two nodes: adding a distant line changed it; in a model 1 m
across only gaps below about 1 µm were flagged, and in a line 2 km long gaps
up to 2 mm were. A modeller judges a gap in absolute terms, and the value they
accept depends on their work, so τ is a fixed default the model can override.
Two nodes an element joins are left out: the element's own length checks
(`AXIS_TOLERANCE`, DEL-04-01) govern them.

**User control of the node-proximity tolerance (owner ruling, 2026-10-23).**
The owner ruled that the near-duplicate node warning uses a fixed 5 mm
tolerance "and give the user the means to change it". The ruling replaced the
relative tolerance.
- **Storage.** The tolerance is an optional model-document member in 0.7.0: a
  length with a unit, default 5 mm (DEL-02-01 Design `model-document-0.7.0.md`).
- **Control.** The desktop shows it in project or analysis settings, where the
  user can see and edit it:
  - any length unit from the catalog may be entered;
  - non-positive values are refused;
  - when the member is unset, the control shows "5 mm (default)".
- **Editing.** The edit is an applier operation on the `Project` target, like
  `set_project_unit` (DEL-02-02 Design `project-unit-selection.md` §2.7).
- **Status.** The control is not implemented. It needs the 0.7.0 member and the
  `Project` target.

**Search.** Nodes are placed in cells 8τ wide, indexed by the coordinate
divided by the width (no offset by the lowest coordinate, whose rounding can
exceed a cell far from the origin), and each node is compared with the nodes
of its own and the 26 neighbouring cells. Two distinct coordinates within τ
on an axis have a spacing of at most 2τ there, so their quotients are below
2^51 and each rounds by at most a quarter cell: their indices differ by at
most one, and every pair within τ is examined. Indices saturate; a neighbour
past a saturated index is skipped, so no cell is visited twice. Distances use a norm scaled by the largest
component, so no square underflows or overflows; coincidence is exact
coordinate equality. Only the first six pairs in node order are kept, with
exact counts, so storage stays bounded however many nodes coincide; the time
is linear in the node count for ordinary models.

## Known effect on bundled data

The bundled desktop demo (`fixtures/product_preview/invented_demo_model.json`)
and `invented_preview_model.json` declare a tee (`C-120` at `node:N-120`) on a
node that joins two pipes, so `BRANCH_TOPOLOGY_INVALID` fires on them. The
solved demo's warning names the header SIF to `pipe:P-120` and the branch SIF
to `pipe:P-110`, which carry its intensified rows. The refused demo's warning
names none, because nothing is published there.

## Evidence

`core/product_physics/tests/topology_checks_runtime.rs` runs each check through
the desktop's value entry in both solver modes: a connected model and an
isolated anchored node give no finding; two anchored cantilevers warn and
their tip displacement equals the cantilever solved alone; an unsupported
part is named and still refused; coincident nodes, and nodes 4.9 mm and
exactly 5 mm apart, warn identically in metres and millimetres, and nodes just
beyond 5 mm do not; in a 2 km model 4 mm warns and 6 mm does not; a pair
across a cell boundary is found; a tee on two pipes and a tee whose header is
its branch warn, and a valid tee does not; a user-set tolerance (10 mm, in mm or m) replaces the default and is inclusive, and a zero, negative, underflowing or non-length value is refused by name with nothing solved; with entered factors the warning names the header and branch SIFs still applied, and those pipes carry the intensified rows, and without factors it names none. `the_branch_warning_names_exactly_the_published_sides`
compares the warning with the published intensified rows in both modes, 802
runs per mode. Header and branch SIF each take five values: absent, zero,
negative, positive, or positive in a non-dimensionless unit. The flexibility
factor takes four: absent, present, zero, or present in a wrong unit. These
combinations run on seven malformed tees and on a valid tee, which gets no
warning. The malformed tees are: a node joining two pipes; a missing header
pipe; a header pipe not ending at the node; a header that is the branch; a
missing branch pipe; another solver consumption; and a branch pipe realized
as an arc. Two more runs check a tee with no modifiers and a tee refused for
missing supports. The warning text must equal the topology text plus exactly
the published sides. A refused envelope names none, and the arc's side is
never named. The previous predicate fails this test. `src/topology_tests.rs` passes the
tolerance directly: another value is used inclusively, a node at -4e15 m does
not separate a near pair, and a saturated cell counts a pair once.
