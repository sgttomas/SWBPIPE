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
| `BRANCH_TOPOLOGY_INVALID` | `core/product_physics/src/validation.rs` | A `branch`, `tee` or `branch_connection` component's node joins a number of pipes other than three, or its header and branch pipe references name the same pipe. One diagnostic per component. The finding warns and does not refuse (REQ-04-06-012), and the component's branch stress factors are still applied by pipe reference. So the message names each side still applied ("its branch stress factors are still applied (header SIF to pipe:…, branch SIF to pipe:…)"): a side whose factors are entered and valid, on a named pipe that ends at the component's node, the same conditions as the stress-multiplier rows. Without such a side the message claims none. |

The disconnected and node checks run on the built model, after unit
normalization, so positions are in metres. Each of those codes yields at most
one diagnostic naming at most six parts or pairs (`formation_guard::NAMED`), so
the diagnostic count does not grow with model size.

**Tolerance.** τ is an absolute length, 5 mm unless the model sets another
value, and a pair at exactly τ is near. The owner chose it (2026-10-23): "I
choose C, but my value is "5mm" and give the user the means to change it. No
computed by any 1e-4, just 5mm". The model sets its value in
`model_settings.node_proximity_tolerance` (a length with a unit; schema 0.7.0),
whose reader validates it (finite, positive, a length unit). Until the check
reads that field, every model uses 5 mm.

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
node that joins two pipes, so `BRANCH_TOPOLOGY_INVALID` fires on them.

## Evidence

`core/product_physics/tests/topology_checks_runtime.rs` runs each check through
the desktop's value entry in both solver modes: a connected model and an
isolated anchored node give no finding; two anchored cantilevers warn and
their tip displacement equals the cantilever solved alone; an unsupported
part is named and still refused; coincident nodes, and nodes 4.9 mm and
exactly 5 mm apart, warn identically in metres and millimetres, and nodes just
beyond 5 mm do not; in a 2 km model 4 mm warns and 6 mm does not; a pair
across a cell boundary is found; a tee on two pipes and a tee whose header is
its branch warn, and a valid tee does not; with entered factors the warning names the header and branch SIFs still applied, and those pipes carry the intensified rows, and without factors it names none. `src/topology_tests.rs` passes the
tolerance directly: another value is used inclusively, a node at -4e15 m does
not separate a near pair, and a saturated cell counts a pair once.
