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
| `NODE_NEAR_DUPLICATE` | `topology.rs` | Two distinct nodes lie within τ·L of each other (and are not coincident), and no element joins them. |
| `BRANCH_TOPOLOGY_INVALID` | `core/product_physics/src/validation.rs` | A `branch`, `tee` or `branch_connection` component's node joins a number of pipes other than three, or its header and branch pipe references name the same pipe. One diagnostic per component. |

The disconnected and node checks run on the built model, after unit
normalization, so positions are in metres. Each of those codes yields at most
one diagnostic naming at most six parts or pairs (`formation_guard::NAMED`), so
the diagnostic count does not grow with model size.

**Tolerance.** The implemented tolerance is relative: τ = 1e-6 and L is the
diagonal of the bounding box of all nodes. Two nodes an element joins are left
out: the element's own length checks (`AXIS_TOLERANCE`, DEL-04-01) govern
them.

**Decided, not yet implemented:** an absolute node-proximity tolerance of 5 mm
that the user can change, replacing τ·L entirely. The owner ruled
(2026-10-23): "I choose C, but my value is "5mm" and give the user the means
to change it. No computed by any 1e-4, just 5mm". The check will warn when two
distinct nodes that no element joins lie no farther apart than the model's
node-proximity tolerance (a pair at exactly that distance warns). The model
sets the value in `model_settings.node_proximity_tolerance` (a length with a
unit, schema 0.7.0; absent means 5 mm), whose reader refuses a non-finite,
non-positive or non-length value. The relative tolerance made the finding
depend on parts of the model unrelated to the two nodes: adding a distant
line changed it; in a model 1 m across only gaps below about 1 µm were
flagged, and in a line 2 km long gaps up to 2 mm were. A modeller judges a gap
in absolute terms, and the value they accept depends on their work, so it is a
fixed default the model can override.

**User control of the node-proximity tolerance (owner ruling, 2026-10-23).**
The owner ruled that the near-duplicate node warning uses a fixed 5 mm
tolerance "and give the user the means to change it". The ruling replaces the
relative tolerance above.
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

**Search.** Nodes are placed in cells twice the tolerance wide, and each
node is compared with the nodes of its own and the 26 neighbouring cells.
Rounding in a cell index moves a point by far less than half a cell, so every
pair within the tolerance is examined (a cell exactly the tolerance wide could
put such a pair two cells apart). Distances use a norm scaled by the largest
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
part is named and still refused; coincident and near nodes warn identically in
metres and millimetres, and a gap of 1e-3·L does not; a tee on two pipes and a
tee whose header is its branch warn, and a valid tee does not.
