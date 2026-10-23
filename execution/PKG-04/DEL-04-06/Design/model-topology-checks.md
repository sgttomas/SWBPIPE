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

**Tolerance.** τ = 1e-6 and L is the diagonal of the bounding box of all nodes.
A relative tolerance makes the check independent of the length unit and of
the model's scale; an absolute one (say 1 mm) would flag every node of a
millimetre-scale test model and miss a 1 mm gap in a kilometre-long line.
1e-6 of the extent is well above binary64 coordinate noise (about 1e-16
relative) and well below any deliberate spacing in a piping model (for a 100 m
model, 0.1 mm). Two nodes an element joins are left out: the element's own
length checks (`AXIS_TOLERANCE`, DEL-04-01) govern them.

**Search.** Nodes are placed in cells of size τ·L; each node is compared with
the nodes of its own and the 26 neighbouring cells, so the cost is linear in
the node count for ordinary models.

## Known effect on bundled data

The bundled desktop demo (`fixtures/product_preview/invented_demo_model.json`)
and `invented_preview_model.json` declare a tee (`C-120` at `node:N-120`) on a
node that joins two pipes, so `BRANCH_TOPOLOGY_INVALID` fires on them. The
demo is not changed by this unit (Agent 0, 2026-10-23).

## Evidence

`core/product_physics/tests/topology_checks_runtime.rs` runs each check through
the desktop's value entry in both solver modes: a connected model and an
isolated anchored node give no finding; two anchored cantilevers warn and
their tip displacement equals the cantilever solved alone; an unsupported
part is named and still refused; coincident and near nodes warn identically in
metres and millimetres, and a gap of 1e-3·L does not; a tee on two pipes and a
tee whose header is its branch warn, and a valid tee does not.
